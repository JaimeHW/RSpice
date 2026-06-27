#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1016] {
            s.store_offset_scaled(67, 316, ((p.p274) * (p.p273)), p.p273);
            s.store_scalar(68, p.p275);
            s.store_scalar(69, p.p276);
            s.store_scalar(70, p.p277);
            s.store_mul3_ad(71, A::scale_offset(A::powf(s.ad_value(314), p.p280), p.p279, p.p278), A::scale_offset(s.ad_value(316), p.p281, 1.0), A::scale_offset(s.ad_value(318), p.p282, 1.0));
            s.store_scalar(72, p.p283);
            s.store_scalar(73, p.p284);
            s.store_scalar(74, p.p285);
            s.store_mul3_ad_scaled_output(75, A::scale_offset(s.ad_value(314), p.p287, 1.0), A::scale_offset(s.ad_value(316), p.p288, 1.0), A::scale_offset(s.ad_value(318), p.p289, 1.0), p.p286);
            s.store_scalar(76, p.p290);
            s.store_scalar(77, p.p291);
            s.store_mul_scaled_ad_rhs(78, 316, p.p292, A::scale_offset(s.ad_value(316), p.p293, 1.0));
            s.store_scalar(79, p.p294);
            s.store_scalar(80, p.p295);
            s.store_scalar(81, p.p296);
            s.store_mul3_ad(82, A::offset(A::mul(A::div_scaled_inputs(s.ad_value(340), p.p298, s.ad_value(339), 1.0), A::powf(s.ad_value(314), p.p299)), p.p297), A::scale_offset(s.ad_value(316), p.p300, 1.0), A::scale_offset(s.ad_value(318), p.p301, 1.0));
            s.store_add_scaled_inputs3_offset(83, s.ad_value(314), p.p303, s.ad_value(316), p.p304, s.ad_value(318), p.p305, p.p302);
            s.store_scalar(84, p.p306);
            s.store_scalar(85, p.p307);
            s.store_scalar(86, p.p308);
            s.store_div_from_scalar_offset_scaled_input(87, p.p309, 314, p.p310, 1.0);
            s.store_scaled_mul_scale_offset_rhs_ad(88, A::powf(s.ad_value(314), p.p312), 316, p.p313, 1.0, p.p311);
            s.store_powf(341, 314, p.p315);
            s.store_div_scaled_product_offset_denominator(89, s.ad_value(341), A::scale_offset(s.ad_value(316), p.p317, 1.0), p.p314, A::mul_scaled_lhs(s.ad_value(314), p.p316, s.ad_value(341)), 1.0, 1.0);
            s.store_powf(341, 314, p.p319);
            s.store_div_scaled_product_offset_denominator(90, s.ad_value(341), A::scale_offset(s.ad_value(316), p.p321, 1.0), p.p318, A::mul_scaled_lhs(s.ad_value(314), p.p320, s.ad_value(341)), 1.0, 1.0);
            s.store_scalar(91, p.p322);
            s.store_scaled_mul_scale_offset_inputs(92, 314, p.p324, 1.0, 316, p.p325, 1.0, p.p323);
            s.store_scalar(93, p.p326);
            s.store_scalar(94, p.p327);
            s.store_scaled_mul_scale_offset_inputs(95, 314, p.p329, 1.0, 316, p.p330, 1.0, p.p328);
            s.store_scaled_mul_scale_offset_inputs(96, 314, p.p332, 1.0, 316, p.p333, 1.0, p.p331);
            s.store_scalar(97, p.p334);
            s.store_scalar(98, p.p335);
            s.store_div_from_scalar(99, p.p336, 318);
            s.store_div_from_scalar_scaled_input(100, (p.p337 * p.p237), 316, 1e-6);
            s.store_div_from_scalar_scaled_input(101, (p.p338 * p.p238), 316, 1e-6);
            s.store_scalar(102, p.p339);
            s.store_scalar(103, p.p340);
            s.store_scalar(104, p.p341);
            s.store_scalar(105, p.p340);
        }

        s.b[1019] = param_given[342];
        s.v[1019] = if s.b[1019] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1019]) {
            s.store_scalar(105, p.p342);
        }

        if s.b[1016] {
            s.store_scalar(106, p.p341);
        }

        s.b[1020] = param_given[343];
        s.v[1020] = if s.b[1020] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1020]) {
            s.store_scalar(106, p.p343);
        }

        if s.b[1016] {
            s.copy_ad(107, 105);
        }

        s.b[1021] = param_given[344];
        s.v[1021] = if s.b[1021] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1021]) {
            s.store_scalar(107, p.p344);
        }

        if s.b[1016] {
            s.copy_ad(108, 106);
        }

        s.b[1022] = param_given[345];
        s.v[1022] = if s.b[1022] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1022]) {
            s.store_scalar(108, p.p345);
        }

        if s.b[1016] {
            s.store_scalar(109, p.p346);
            s.store_div_from_scalar_scaled_input(110, (p.p347 * p.p237), 316, 1e-6);
            s.store_div_from_scalar_scaled_input(111, (p.p348 * p.p238), 316, 1e-6);
            s.store_scalar(112, p.p349);
            s.store_scalar(113, p.p350);
            s.store_scalar(114, p.p351);
            s.store_scalar(115, p.p352);
            s.store_scalar(116, p.p353);
            s.store_scalar(117, p.p354);
            s.store_scaled_mul(118, 321, 320, ((8.8541878176e-12 * p.p210) * 1.0 / (p.p209)));
            s.store_scale(125, 321, ((8.8541878176e-12 * p.p210) * (p.p237 * 1.0 / (p.p235))));
            s.store_scale(126, 321, ((8.8541878176e-12 * p.p210) * (p.p238 * 1.0 / (p.p236))));
            s.store_add_scaled_inputs3_offset(119, A::powf(s.ad_value(314), p.p357), p.p356, s.ad_value(316), p.p358, s.ad_value(318), p.p359, p.p355);
            s.store_add_scaled_inputs3_offset(120, s.ad_value(314), p.p361, s.ad_value(316), p.p362, s.ad_value(318), p.p363, p.p360);
            s.store_scalar(32, p.p297);
        }

        s.b[1023] = param_given[364];
        s.v[1023] = if s.b[1023] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1023]) {
            s.store_scalar(32, p.p364);
        }

        if s.b[1016] {
            s.store_scalar(33, p.p298);
        }

        s.b[1024] = param_given[365];
        s.v[1024] = if s.b[1024] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1024]) {
            s.store_scalar(33, p.p365);
        }

        if s.b[1016] {
            s.store_scalar(34, p.p299);
        }

        s.b[1025] = param_given[366];
        s.v[1025] = if s.b[1025] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1025]) {
            s.store_scalar(34, p.p366);
        }

        if s.b[1016] {
            s.store_scalar(35, p.p300);
        }

        s.b[1026] = param_given[367];
        s.v[1026] = if s.b[1026] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1026]) {
            s.store_scalar(35, p.p367);
        }

        if s.b[1016] {
            s.store_scalar(36, p.p301);
        }

        s.b[1027] = param_given[368];
        s.v[1027] = if s.b[1027] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1027]) {
            s.store_scalar(36, p.p368);
        }

        if s.b[1016] {
            s.store_mul3_ad(121, A::add_scaled_product(s.ad_value(32), 1.0, A::div_scaled_product(s.ad_value(33), s.ad_value(340), 1.0, s.ad_value(339), 1.0), A::pow(s.ad_value(314), s.ad_value(34)), 1.0), A::offset(A::mul(s.ad_value(35), s.ad_value(316)), 1.0), A::offset(A::mul(s.ad_value(36), s.ad_value(318)), 1.0));
            s.store_scalar(37, p.p309);
        }

        s.b[1028] = param_given[369];
        s.v[1028] = if s.b[1028] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1028]) {
            s.store_scalar(37, p.p369);
        }

        if s.b[1016] {
            s.store_scalar(38, p.p310);
        }

        s.b[1029] = param_given[370];
        s.v[1029] = if s.b[1029] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1029]) {
            s.store_scalar(38, p.p370);
        }

        if s.b[1016] {
            s.store_div_scaled_value_offset_denominator(122, s.ad_value(37), 1.0, A::mul(s.ad_value(38), s.ad_value(314)), 1.0, 1.0);
            s.store_scaled_mul_scale_offset_rhs_ad(123, A::powf(s.ad_value(314), p.p372), 316, p.p373, 1.0, p.p371);
            s.store_powf(341, 314, p.p375);
            s.store_div_scaled_product_offset_denominator(124, s.ad_value(341), A::scale_offset(s.ad_value(316), p.p377, 1.0), p.p374, A::mul_scaled_lhs(s.ad_value(314), p.p376, s.ad_value(341)), 1.0, 1.0);
            s.store_scalar(127, p.p378);
            s.store_scalar(128, p.p379);
            s.store_scalar(129, p.p380);
            s.store_scale(130, 325, p.p381);
            s.store_scale(131, 322, p.p382);
            s.store_scale(132, 322, p.p383);
            s.store_scalar(133, p.p384);
            s.store_scalar(134, p.p385);
            s.store_scalar(135, p.p386);
            s.store_scalar(136, p.p387);
            s.store_scale(137, 326, p.p388);
            s.store_scale(138, 326, p.p389);
            s.store_sub_from_scalar_ad(998, 1.0, A::div_from_scalar((2.0 * p.p396), s.ad_value(312)));
            s.store_scalar(139, p.p390);
            s.store_offset_scaled(344, 313, p.p399, (2.0 * p.p398));
            s.store_scalar(145, p.p400);
            s.store_add_scaled_inputs3_offset(146, s.ad_value(314), p.p402, s.ad_value(316), p.p403, s.ad_value(318), p.p404, p.p401);
            s.store_add_scaled_inputs3_offset(147, A::powf(s.ad_value(314), p.p407), p.p406, s.ad_value(316), p.p408, s.ad_value(318), p.p409, p.p405);
            s.store_mul3_ad_scaled_output(148, A::scale_offset(A::powf(s.ad_value(314), p.p412), p.p411, 1.0), A::scale_offset(s.ad_value(316), p.p413, 1.0), A::scale_offset(s.ad_value(318), p.p414, 1.0), p.p410);
            s.store_offset_scaled_ad(149, A::powf(s.ad_value(314), p.p417), p.p416, p.p415);
            s.store_offset_ad(347, A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p418 * p.p419), s.ad_value(312)), 1.0, A::exp_scaled_input(s.ad_value(312), (-1.0 / (p.p419)))), 1.0);
        }

        if s.b[1016] {
            if (s.v[347] > 1e-15) {
            } else {
                s.store_scalar(347, 1e-15);
            }
        }

        if s.b[1016] {
            s.store_mul_ad(150, A::div_scaled_inputs(s.ad_value(344), p.p259, A::mul(s.ad_value(347), s.ad_value(312)), 1.0), A::scale_offset(s.ad_value(316), p.p420, 1.0));
            s.store_add_scaled_inputs3_offset(151, s.ad_value(314), p.p422, s.ad_value(316), p.p423, s.ad_value(318), p.p424, p.p421);
            s.store_scaled_mul_scale_offset_rhs_ad(152, A::powf(s.ad_value(314), p.p426), 316, p.p427, 1.0, p.p425);
            s.store_scalar(153, p.p428);
            s.store_scalar(154, p.p429);
            s.store_scaled_mul_scale_offset_rhs_ad(155, A::powf(s.ad_value(314), p.p431), 316, p.p432, 1.0, p.p430);
            s.store_scalar(156, p.p434);
            s.store_scalar(157, p.p433);
            s.store_add_scaled_inputs3_offset(348, s.ad_value(314), p.p832, s.ad_value(316), p.p833, s.ad_value(318), p.p834, p.p831);
            s.store_add_scaled_inputs3_offset(349, s.ad_value(314), p.p836, s.ad_value(316), p.p837, s.ad_value(318), p.p838, p.p835);
            s.store_offset_div_scaled_offset_numerator(173, A::div_from_scalar(p.p458, s.ad_value(314)), p.p456, (((1.0) + (p.p457)) * p.p456), s.ad_value(316), 1.0, p.p455);
        }

        s.b[1031] = (((param_given[460] || param_given[461]) || param_given[462]) || param_given[463]);
        s.v[1031] = if s.b[1031] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1031]) {
            s.store_add_scaled_inputs3_offset(40, s.ad_value(314), p.p461, s.ad_value(316), p.p462, s.ad_value(318), p.p463, p.p460);
        }

        s.b[1032] = (((param_given[464] || param_given[465]) || param_given[466]) || param_given[467]);
        s.v[1032] = if s.b[1032] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1032]) {
            s.store_add_scaled_inputs3_offset(41, s.ad_value(314), p.p465, s.ad_value(316), p.p466, s.ad_value(318), p.p467, p.p464);
        }

        s.b[1033] = (((param_given[468] || param_given[469]) || param_given[470]) || param_given[471]);
        s.v[1033] = if s.b[1033] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1016] && s.b[1033]) {
            s.store_add_scaled_inputs3_offset(45, s.ad_value(314), p.p469, s.ad_value(316), p.p470, s.ad_value(318), p.p471, p.p468);
        }

        s.b[1034] = (((param_given[472] || param_given[473]) || param_given[474]) || param_given[475]);
        s.v[1034] = if s.b[1034] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1034]) {
            s.store_add_scaled_inputs3_offset(46, s.ad_value(314), p.p473, s.ad_value(316), p.p474, s.ad_value(318), p.p475, p.p472);
        }

        s.b[1035] = (((param_given[476] || param_given[477]) || param_given[478]) || param_given[479]);
        s.v[1035] = if s.b[1035] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1035]) {
            s.store_add_scaled_inputs3_offset(47, s.ad_value(314), p.p477, s.ad_value(316), p.p478, s.ad_value(318), p.p479, p.p476);
        }

        s.b[1036] = (((param_given[480] || param_given[481]) || param_given[482]) || param_given[483]);
        s.v[1036] = if s.b[1036] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1036]) {
            s.store_add_scaled_inputs3_offset(49, s.ad_value(314), p.p481, s.ad_value(316), p.p482, s.ad_value(318), p.p483, p.p480);
        }

        s.b[1037] = (((param_given[484] || param_given[485]) || param_given[486]) || param_given[487]);
        s.v[1037] = if s.b[1037] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1037]) {
            s.store_add_scaled_inputs3_offset(50, s.ad_value(314), p.p485, s.ad_value(316), p.p486, s.ad_value(318), p.p487, p.p484);
        }

        s.b[1038] = (((param_given[488] || param_given[489]) || param_given[490]) || param_given[491]);
        s.v[1038] = if s.b[1038] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1038]) {
            s.store_add_scaled_inputs3_offset(57, s.ad_value(314), p.p489, s.ad_value(316), p.p490, s.ad_value(318), p.p491, p.p488);
        }

        s.b[1039] = (((param_given[492] || param_given[493]) || param_given[494]) || param_given[495]);
        s.v[1039] = if s.b[1039] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1039]) {
            s.store_add_scaled_inputs3_offset(58, s.ad_value(314), p.p493, s.ad_value(316), p.p494, s.ad_value(318), p.p495, p.p492);
        }

        s.b[1040] = (((param_given[496] || param_given[497]) || param_given[498]) || param_given[499]);
        s.v[1040] = if s.b[1040] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1040]) {
            s.store_add_scaled_inputs3_offset(51, s.ad_value(314), p.p497, s.ad_value(316), p.p498, s.ad_value(318), p.p499, p.p496);
        }

        s.b[1041] = (((param_given[504] || param_given[505]) || param_given[506]) || param_given[507]);
        s.v[1041] = if s.b[1041] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1041]) {
            s.store_add_scaled_inputs3_offset(52, s.ad_value(314), p.p505, s.ad_value(316), p.p506, s.ad_value(318), p.p507, p.p504);
        }

        s.b[1042] = (((param_given[500] || param_given[501]) || param_given[502]) || param_given[503]);
        s.v[1042] = if s.b[1042] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1042]) {
            s.store_add_scaled_inputs3_offset(53, s.ad_value(314), p.p501, s.ad_value(316), p.p502, s.ad_value(318), p.p503, p.p500);
        }

        s.b[1043] = (((param_given[508] || param_given[509]) || param_given[510]) || param_given[511]);
        s.v[1043] = if s.b[1043] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1043]) {
            s.store_add_scaled_inputs3_offset(54, s.ad_value(314), p.p509, s.ad_value(316), p.p510, s.ad_value(318), p.p511, p.p508);
        }

        s.b[1044] = (((param_given[512] || param_given[513]) || param_given[514]) || param_given[515]);
        s.v[1044] = if s.b[1044] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1044]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(62, 315, s.ad_value(314), p.p513, s.ad_value(316), p.p514, s.ad_value(318), p.p515, p.p512);
        }

        s.b[1045] = (((param_given[520] || param_given[521]) || param_given[522]) || param_given[523]);
        s.v[1045] = if s.b[1045] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1045]) {
            s.store_add_scaled_inputs3_offset(63, s.ad_value(314), p.p521, s.ad_value(316), p.p522, s.ad_value(318), p.p523, p.p520);
        }

        s.b[1046] = (((param_given[516] || param_given[517]) || param_given[518]) || param_given[519]);
        s.v[1046] = if s.b[1046] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1046]) {
            s.store_add_scaled_inputs3_offset(64, s.ad_value(314), p.p517, s.ad_value(316), p.p518, s.ad_value(318), p.p519, p.p516);
        }

        s.b[1047] = (((param_given[524] || param_given[525]) || param_given[526]) || param_given[527]);
        s.v[1047] = if s.b[1047] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1047]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(59, 315, s.ad_value(314), p.p525, s.ad_value(316), p.p526, s.ad_value(318), p.p527, p.p524);
        }

        s.b[1048] = (((param_given[532] || param_given[533]) || param_given[534]) || param_given[535]);
        s.v[1048] = if s.b[1048] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1048]) {
            s.store_add_scaled_inputs3_offset(60, s.ad_value(314), p.p533, s.ad_value(316), p.p534, s.ad_value(318), p.p535, p.p532);
        }

        s.b[1049] = (((param_given[528] || param_given[529]) || param_given[530]) || param_given[531]);
        s.v[1049] = if s.b[1049] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1049]) {
            s.store_add_scaled_inputs3_offset(61, s.ad_value(314), p.p529, s.ad_value(316), p.p530, s.ad_value(318), p.p531, p.p528);
        }

        s.b[1050] = (((param_given[536] || param_given[537]) || param_given[538]) || param_given[539]);
        s.v[1050] = if s.b[1050] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1050]) {
            s.store_mul_ad(65, A::div(s.ad_value(313), s.ad_value(312)), A::add_scaled_inputs3_offset(s.ad_value(314), p.p537, s.ad_value(316), p.p538, s.ad_value(318), p.p539, p.p536));
        }

        s.b[1051] = (((param_given[540] || param_given[541]) || param_given[542]) || param_given[543]);
        s.v[1051] = if s.b[1051] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1051]) {
            s.store_add_scaled_inputs3_offset(66, s.ad_value(314), p.p541, s.ad_value(316), p.p542, s.ad_value(318), p.p543, p.p540);
        }

        s.b[1052] = (((param_given[544] || param_given[545]) || param_given[546]) || param_given[547]);
        s.v[1052] = if s.b[1052] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1052]) {
            s.store_add_scaled_inputs3_offset(67, s.ad_value(314), p.p545, s.ad_value(316), p.p546, s.ad_value(318), p.p547, p.p544);
        }

        s.b[1053] = (((param_given[548] || param_given[549]) || param_given[550]) || param_given[551]);
        s.v[1053] = if s.b[1053] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1053]) {
            s.store_add_scaled_inputs3_offset(69, s.ad_value(314), p.p549, s.ad_value(316), p.p550, s.ad_value(318), p.p551, p.p548);
        }

        s.b[1054] = (((param_given[552] || param_given[553]) || param_given[554]) || param_given[555]);
        s.v[1054] = if s.b[1054] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1054]) {
            s.store_add_scaled_inputs3_offset(71, s.ad_value(314), p.p553, s.ad_value(316), p.p554, s.ad_value(318), p.p555, p.p552);
        }

        s.b[1055] = (((param_given[556] || param_given[557]) || param_given[558]) || param_given[559]);
        s.v[1055] = if s.b[1055] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1055]) {
            s.store_add_scaled_inputs3_offset(73, s.ad_value(314), p.p557, s.ad_value(316), p.p558, s.ad_value(318), p.p559, p.p556);
        }

        s.b[1056] = (((param_given[560] || param_given[561]) || param_given[562]) || param_given[563]);
        s.v[1056] = if s.b[1056] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1056]) {
            s.store_add_scaled_inputs3_offset(75, s.ad_value(314), p.p561, s.ad_value(316), p.p562, s.ad_value(318), p.p563, p.p560);
        }

        s.b[1057] = (((param_given[564] || param_given[565]) || param_given[566]) || param_given[567]);
        s.v[1057] = if s.b[1057] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1057]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(78, 316, s.ad_value(314), p.p565, s.ad_value(316), p.p566, s.ad_value(318), p.p567, p.p564);
        }

        s.b[1058] = (((param_given[568] || param_given[569]) || param_given[570]) || param_given[571]);
        s.v[1058] = if s.b[1058] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1058]) {
            s.store_add_scaled_inputs3_offset(79, s.ad_value(314), p.p569, s.ad_value(316), p.p570, s.ad_value(318), p.p571, p.p568);
        }

        s.b[1059] = (((param_given[572] || param_given[573]) || param_given[574]) || param_given[575]);
        s.v[1059] = if s.b[1059] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1059]) {
            s.store_add_scaled_inputs3_offset(80, s.ad_value(314), p.p573, s.ad_value(316), p.p574, s.ad_value(318), p.p575, p.p572);
        }

        s.b[1060] = (((param_given[576] || param_given[577]) || param_given[578]) || param_given[579]);
        s.v[1060] = if s.b[1060] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1060]) {
            s.store_add_scaled_inputs3_offset(81, s.ad_value(314), p.p577, s.ad_value(316), p.p578, s.ad_value(318), p.p579, p.p576);
        }

        s.b[1061] = (((param_given[580] || param_given[581]) || param_given[582]) || param_given[583]);
        s.v[1061] = if s.b[1061] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1061]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(82, 314, s.ad_value(314), p.p581, s.ad_value(316), p.p582, s.ad_value(318), p.p583, p.p580);
        }

        s.b[1062] = (((param_given[584] || param_given[585]) || param_given[586]) || param_given[587]);
        s.v[1062] = if s.b[1062] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1062]) {
            s.store_add_scaled_inputs3_offset(83, s.ad_value(314), p.p585, s.ad_value(316), p.p586, s.ad_value(318), p.p587, p.p584);
        }

        s.b[1063] = (((param_given[588] || param_given[589]) || param_given[590]) || param_given[591]);
        s.v[1063] = if s.b[1063] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1063]) {
            s.store_add_scaled_inputs3_offset(84, s.ad_value(314), p.p589, s.ad_value(316), p.p590, s.ad_value(318), p.p591, p.p588);
        }

        s.b[1064] = (((param_given[592] || param_given[593]) || param_given[594]) || param_given[595]);
        s.v[1064] = if s.b[1064] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1064]) {
            s.store_add_scaled_inputs3_offset(85, s.ad_value(314), p.p593, s.ad_value(316), p.p594, s.ad_value(318), p.p595, p.p592);
        }

        s.b[1065] = (((param_given[596] || param_given[597]) || param_given[598]) || param_given[599]);
        s.v[1065] = if s.b[1065] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1065]) {
            s.store_add_scaled_inputs3_offset(87, s.ad_value(314), p.p597, s.ad_value(316), p.p598, s.ad_value(318), p.p599, p.p596);
        }

        s.b[1066] = (((param_given[600] || param_given[601]) || param_given[602]) || param_given[603]);
        s.v[1066] = if s.b[1066] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1066]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(88, 314, s.ad_value(314), p.p601, s.ad_value(316), p.p602, s.ad_value(318), p.p603, p.p600);
        }

        s.b[1067] = (((param_given[604] || param_given[605]) || param_given[606]) || param_given[607]);
        s.v[1067] = if s.b[1067] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1067]) {
            s.store_add_scaled_inputs3_offset(89, s.ad_value(314), p.p605, s.ad_value(316), p.p606, s.ad_value(318), p.p607, p.p604);
        }

        s.b[1068] = (((param_given[608] || param_given[609]) || param_given[610]) || param_given[611]);
        s.v[1068] = if s.b[1068] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1068]) {
            s.store_add_scaled_inputs3_offset(90, s.ad_value(314), p.p609, s.ad_value(316), p.p610, s.ad_value(318), p.p611, p.p608);
        }

        s.b[1069] = (((param_given[612] || param_given[613]) || param_given[614]) || param_given[615]);
        s.v[1069] = if s.b[1069] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1069]) {
            s.store_add_scaled_inputs3_offset(92, s.ad_value(314), p.p613, s.ad_value(316), p.p614, s.ad_value(318), p.p615, p.p612);
        }

        s.b[1070] = (((param_given[616] || param_given[617]) || param_given[618]) || param_given[619]);
        s.v[1070] = if s.b[1070] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1070]) {
            s.store_add_scaled_inputs3_offset(94, s.ad_value(314), p.p617, s.ad_value(316), p.p618, s.ad_value(318), p.p619, p.p616);
        }

        s.b[1071] = (((param_given[620] || param_given[621]) || param_given[622]) || param_given[623]);
        s.v[1071] = if s.b[1071] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1071]) {
            s.store_add_scaled_inputs3_offset(95, s.ad_value(314), p.p621, s.ad_value(316), p.p622, s.ad_value(318), p.p623, p.p620);
        }

        s.b[1072] = (((param_given[624] || param_given[625]) || param_given[626]) || param_given[627]);
        s.v[1072] = if s.b[1072] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1072]) {
            s.store_add_scaled_inputs3_offset(96, s.ad_value(314), p.p625, s.ad_value(316), p.p626, s.ad_value(318), p.p627, p.p624);
        }

        s.b[1073] = (((param_given[628] || param_given[629]) || param_given[630]) || param_given[631]);
        s.v[1073] = if s.b[1073] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1073]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(99, 319, s.ad_value(314), p.p629, s.ad_value(316), p.p630, s.ad_value(318), p.p631, p.p628);
        }

        s.b[1074] = (((param_given[632] || param_given[633]) || param_given[634]) || param_given[635]);
        s.v[1074] = if s.b[1074] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1074]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(100, 317, s.ad_value(314), p.p633, s.ad_value(316), p.p634, s.ad_value(318), p.p635, p.p632);
        }

        s.b[1075] = (((param_given[636] || param_given[637]) || param_given[638]) || param_given[639]);
        s.v[1075] = if s.b[1075] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1075]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(101, 317, s.ad_value(314), p.p637, s.ad_value(316), p.p638, s.ad_value(318), p.p639, p.p636);
        }

        s.b[1076] = (((param_given[640] || param_given[641]) || param_given[642]) || param_given[643]);
        s.v[1076] = if s.b[1076] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1076]) {
            s.store_add_scaled_inputs3_offset(102, s.ad_value(314), p.p641, s.ad_value(316), p.p642, s.ad_value(318), p.p643, p.p640);
        }

        s.b[1077] = (((param_given[644] || param_given[645]) || param_given[646]) || param_given[647]);
        s.v[1077] = if s.b[1077] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1077]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(110, 317, s.ad_value(314), p.p645, s.ad_value(316), p.p646, s.ad_value(318), p.p647, p.p644);
        }

        s.b[1078] = (((param_given[648] || param_given[649]) || param_given[650]) || param_given[651]);
        s.v[1078] = if s.b[1078] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1078]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(111, 317, s.ad_value(314), p.p649, s.ad_value(316), p.p650, s.ad_value(318), p.p651, p.p648);
        }

        s.b[1079] = (((param_given[652] || param_given[653]) || param_given[654]) || param_given[655]);
        s.v[1079] = if s.b[1079] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1079]) {
            s.store_add_scaled_inputs3_offset(114, s.ad_value(314), p.p653, s.ad_value(316), p.p654, s.ad_value(318), p.p655, p.p652);
        }

        s.b[1080] = (((param_given[656] || param_given[657]) || param_given[658]) || param_given[659]);
        s.v[1080] = if s.b[1080] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1080]) {
            s.store_add_scaled_inputs3_offset(115, s.ad_value(314), p.p657, s.ad_value(316), p.p658, s.ad_value(318), p.p659, p.p656);
        }

        s.b[1081] = (((param_given[660] || param_given[661]) || param_given[662]) || param_given[663]);
        s.v[1081] = if s.b[1081] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1081]) {
            s.store_mul_ad_affine_product_rhs(118, 322, s.ad_value(320), A::add_scaled_inputs3_offset(s.ad_value(314), p.p661, s.ad_value(316), p.p662, s.ad_value(318), p.p663, p.p660), 1.0 / (1e-6), 0.0);
        }

        s.b[1082] = (((param_given[664] || param_given[665]) || param_given[666]) || param_given[667]);
        s.v[1082] = if s.b[1082] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1082]) {
            s.store_add_scaled_inputs3_offset(119, s.ad_value(314), p.p665, s.ad_value(316), p.p666, s.ad_value(318), p.p667, p.p664);
        }

        s.b[1083] = (((param_given[668] || param_given[669]) || param_given[670]) || param_given[671]);
        s.v[1083] = if s.b[1083] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1083]) {
            s.store_add_scaled_inputs3_offset(120, s.ad_value(314), p.p669, s.ad_value(316), p.p670, s.ad_value(318), p.p671, p.p668);
        }

        s.b[1084] = (((((((param_given[672] || param_given[673]) || param_given[674]) || param_given[675]) || param_given[580]) || param_given[581]) || param_given[582]) || param_given[583]);
        s.v[1084] = if s.b[1084] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1084]) {
            s.store_scalar(28, p.p580);
        }

        s.b[1085] = param_given[672];
        s.v[1085] = if s.b[1085] { 1.0 } else { 0.0 };

        if ((s.b[1016] && s.b[1084]) && s.b[1085]) {
            s.store_scalar(28, p.p672);
        }

        if (s.b[1016] && s.b[1084]) {
            s.store_scalar(29, p.p581);
        }

        s.b[1086] = param_given[673];
        s.v[1086] = if s.b[1086] { 1.0 } else { 0.0 };

        if ((s.b[1016] && s.b[1084]) && s.b[1086]) {
            s.store_scalar(29, p.p673);
        }

        if (s.b[1016] && s.b[1084]) {
            s.store_scalar(30, p.p582);
        }

        s.b[1087] = param_given[674];
        s.v[1087] = if s.b[1087] { 1.0 } else { 0.0 };

        if ((s.b[1016] && s.b[1084]) && s.b[1087]) {
            s.store_scalar(30, p.p674);
        }

        if (s.b[1016] && s.b[1084]) {
            s.store_scalar(31, p.p583);
        }

        s.b[1088] = param_given[675];
        s.v[1088] = if s.b[1088] { 1.0 } else { 0.0 };

        if ((s.b[1016] && s.b[1084]) && s.b[1088]) {
            s.store_scalar(31, p.p675);
        }

        if (s.b[1016] && s.b[1084]) {
            s.store_mul_ad_rhs(121, 314, A::add_scaled_value_products3(s.ad_value(28), 1.0, s.ad_value(29), s.ad_value(314), 1.0, s.ad_value(30), s.ad_value(316), 1.0, s.ad_value(31), s.ad_value(318), 1.0));
        }

        s.b[1089] = (((((((param_given[676] || param_given[677]) || param_given[678]) || param_given[679]) || param_given[596]) || param_given[597]) || param_given[598]) || param_given[599]);
        s.v[1089] = if s.b[1089] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1089]) {
            s.store_scalar(28, p.p596);
        }

        s.b[1090] = param_given[676];
        s.v[1090] = if s.b[1090] { 1.0 } else { 0.0 };

        if ((s.b[1016] && s.b[1089]) && s.b[1090]) {
            s.store_scalar(28, p.p676);
        }

        if (s.b[1016] && s.b[1089]) {
            s.store_scalar(29, p.p597);
        }

        s.b[1091] = param_given[677];
        s.v[1091] = if s.b[1091] { 1.0 } else { 0.0 };

        if ((s.b[1016] && s.b[1089]) && s.b[1091]) {
            s.store_scalar(29, p.p677);
        }

        if (s.b[1016] && s.b[1089]) {
            s.store_scalar(30, p.p598);
        }

        s.b[1092] = param_given[678];
        s.v[1092] = if s.b[1092] { 1.0 } else { 0.0 };

        if ((s.b[1016] && s.b[1089]) && s.b[1092]) {
            s.store_scalar(30, p.p678);
        }

        if (s.b[1016] && s.b[1089]) {
            s.store_scalar(31, p.p599);
        }

        s.b[1093] = param_given[679];
        s.v[1093] = if s.b[1093] { 1.0 } else { 0.0 };

        if ((s.b[1016] && s.b[1089]) && s.b[1093]) {
            s.store_scalar(31, p.p679);
        }

        if (s.b[1016] && s.b[1089]) {
            s.store_add_scaled_value_products3_indices(122, 28, 1.0, 29, 314, 1.0, 30, 316, 1.0, 31, 318, 1.0);
        }

        s.b[1094] = (((param_given[680] || param_given[681]) || param_given[682]) || param_given[683]);
        s.v[1094] = if s.b[1094] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1094]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(123, 314, s.ad_value(314), p.p681, s.ad_value(316), p.p682, s.ad_value(318), p.p683, p.p680);
        }

        s.b[1095] = (((param_given[684] || param_given[685]) || param_given[686]) || param_given[687]);
        s.v[1095] = if s.b[1095] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1095]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(124, 314, s.ad_value(314), p.p685, s.ad_value(316), p.p686, s.ad_value(318), p.p687, p.p684);
        }

        s.b[1096] = (((param_given[688] || param_given[689]) || param_given[690]) || param_given[691]);
        s.v[1096] = if s.b[1096] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1096]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(125, 322, s.ad_value(314), p.p689, s.ad_value(316), p.p690, s.ad_value(318), p.p691, p.p688);
        }

        s.b[1097] = (((param_given[692] || param_given[693]) || param_given[694]) || param_given[695]);
        s.v[1097] = if s.b[1097] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1097]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(126, 322, s.ad_value(314), p.p693, s.ad_value(316), p.p694, s.ad_value(318), p.p695, p.p692);
        }

        s.b[1098] = (((param_given[696] || param_given[697]) || param_given[698]) || param_given[699]);
        s.v[1098] = if s.b[1098] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1098]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(130, 325, s.ad_value(314), p.p697, s.ad_value(316), p.p698, s.ad_value(318), p.p699, p.p696);
        }

        s.b[1099] = (((param_given[700] || param_given[701]) || param_given[702]) || param_given[703]);
        s.v[1099] = if s.b[1099] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1099]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(131, 322, s.ad_value(314), p.p701, s.ad_value(316), p.p702, s.ad_value(318), p.p703, p.p700);
        }

        s.b[1100] = (((param_given[704] || param_given[705]) || param_given[706]) || param_given[707]);
        s.v[1100] = if s.b[1100] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1100]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(132, 322, s.ad_value(314), p.p705, s.ad_value(316), p.p706, s.ad_value(318), p.p707, p.p704);
        }

        s.b[1101] = (((param_given[708] || param_given[709]) || param_given[710]) || param_given[711]);
        s.v[1101] = if s.b[1101] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1101]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(137, 326, s.ad_value(314), p.p709, s.ad_value(316), p.p710, s.ad_value(318), p.p711, p.p708);
        }

    }

    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1102] = (((param_given[712] || param_given[713]) || param_given[714]) || param_given[715]);
        s.v[1102] = if s.b[1102] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1102]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(138, 326, s.ad_value(314), p.p713, s.ad_value(316), p.p714, s.ad_value(318), p.p715, p.p712);
        }

        s.b[1107] = (((param_given[732] || param_given[733]) || param_given[734]) || param_given[735]);
        s.v[1107] = if s.b[1107] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1107]) {
            s.store_add_scaled_inputs3_offset(145, s.ad_value(314), p.p733, s.ad_value(316), p.p734, s.ad_value(318), p.p735, p.p732);
        }

        s.b[1108] = (((param_given[736] || param_given[737]) || param_given[738]) || param_given[739]);
        s.v[1108] = if s.b[1108] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1108]) {
            s.store_add_scaled_inputs3_offset(146, s.ad_value(314), p.p737, s.ad_value(316), p.p738, s.ad_value(318), p.p739, p.p736);
        }

        s.b[1109] = (((param_given[740] || param_given[741]) || param_given[742]) || param_given[743]);
        s.v[1109] = if s.b[1109] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1109]) {
            s.store_add_scaled_inputs3_offset(147, s.ad_value(314), p.p741, s.ad_value(316), p.p742, s.ad_value(318), p.p743, p.p740);
        }

        s.b[1110] = (((param_given[744] || param_given[745]) || param_given[746]) || param_given[747]);
        s.v[1110] = if s.b[1110] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1110]) {
            s.store_add_scaled_inputs3_offset(148, s.ad_value(314), p.p745, s.ad_value(316), p.p746, s.ad_value(318), p.p747, p.p744);
        }

        s.b[1111] = (((param_given[748] || param_given[749]) || param_given[750]) || param_given[751]);
        s.v[1111] = if s.b[1111] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1111]) {
            s.store_add_scaled_inputs3_offset(149, s.ad_value(314), p.p749, s.ad_value(316), p.p750, s.ad_value(318), p.p751, p.p748);
        }

        s.b[1112] = (((param_given[752] || param_given[753]) || param_given[754]) || param_given[755]);
        s.v[1112] = if s.b[1112] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1112]) {
            s.store_mul_ad(150, A::div(s.ad_value(344), s.ad_value(312)), A::add_scaled_inputs3_offset(s.ad_value(314), p.p753, s.ad_value(316), p.p754, s.ad_value(318), p.p755, p.p752));
        }

        s.b[1113] = (((param_given[756] || param_given[757]) || param_given[758]) || param_given[759]);
        s.v[1113] = if s.b[1113] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1113]) {
            s.store_add_scaled_inputs3_offset(151, s.ad_value(314), p.p757, s.ad_value(316), p.p758, s.ad_value(318), p.p759, p.p756);
        }

        s.b[1114] = (((param_given[760] || param_given[761]) || param_given[762]) || param_given[763]);
        s.v[1114] = if s.b[1114] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1114]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(152, 315, s.ad_value(314), p.p761, s.ad_value(316), p.p762, s.ad_value(318), p.p763, p.p760);
        }

        s.b[1115] = (((param_given[764] || param_given[765]) || param_given[766]) || param_given[767]);
        s.v[1115] = if s.b[1115] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1115]) {
            s.store_add_scaled_inputs3_offset(153, s.ad_value(314), p.p765, s.ad_value(316), p.p766, s.ad_value(318), p.p767, p.p764);
        }

        s.b[1116] = (((param_given[768] || param_given[769]) || param_given[770]) || param_given[771]);
        s.v[1116] = if s.b[1116] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1116]) {
            s.store_add_scaled_inputs3_offset(154, s.ad_value(314), p.p769, s.ad_value(316), p.p770, s.ad_value(318), p.p771, p.p768);
        }

        s.b[1117] = (((param_given[772] || param_given[773]) || param_given[774]) || param_given[775]);
        s.v[1117] = if s.b[1117] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1117]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(155, 315, s.ad_value(314), p.p773, s.ad_value(316), p.p774, s.ad_value(318), p.p775, p.p772);
        }

        s.b[1118] = (((param_given[780] || param_given[781]) || param_given[782]) || param_given[783]);
        s.v[1118] = if s.b[1118] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1118]) {
            s.store_add_scaled_inputs3_offset(156, s.ad_value(314), p.p781, s.ad_value(316), p.p782, s.ad_value(318), p.p783, p.p780);
        }

        s.b[1119] = (((param_given[776] || param_given[777]) || param_given[778]) || param_given[779]);
        s.v[1119] = if s.b[1119] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1119]) {
            s.store_add_scaled_inputs3_offset(157, s.ad_value(314), p.p777, s.ad_value(316), p.p778, s.ad_value(318), p.p779, p.p776);
        }

        s.b[1124] = (((param_given[800] || param_given[801]) || param_given[802]) || param_given[803]);
        s.v[1124] = if s.b[1124] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1124]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(173, 319, s.ad_value(314), p.p801, s.ad_value(316), p.p802, s.ad_value(318), p.p803, p.p800);
        }

        if s.b[1016] {
            s.store_scalar(1005, 0.0);
            s.store_scalar(1006, 0.0);
            s.store_scalar(1004, 0.0);
            s.store_scalar(39, p.p812);
        }

        s.b[1126] = param_given[813];
        s.v[1126] = if s.b[1126] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1126]) {
            s.store_scalar(39, p.p813);
        }

        s.b[1127] = (((s.v[5] > 0.0) && (s.v[6] > 0.0)) && ((s.v[1] == 1.0) || ((s.v[1] > 1.0) && (s.v[7] > 0.0))));
        s.v[1127] = if s.b[1127] { 1.0 } else { 0.0 };

        let mut assign9190_loop_guard: usize = 0;
        while {
            let assign9190_cond_e9116: f64 = (s.v[1] - 0.5);
            let assign9190_cond_e9118: f64 = if ((s.b[1016] && s.b[1127]) && (s.v[1004] < assign9190_cond_e9116)) { 1.0 } else { 0.0 };
            assign9190_cond_e9118 != 0.0
        } {
            assign9190_loop_guard += 1;
            assert!(assign9190_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1016] && s.b[1127]) {
                s.store_add_ad_rhs(1005, 1005, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1004), (s.v[7] + s.v[3]), (s.v[5] + (0.5 * s.v[3])))));
                s.store_add_ad_rhs(1006, 1006, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1004), (s.v[7] + s.v[3]), (s.v[6] + (0.5 * s.v[3])))));
                s.store_offset(1004, 1004, 1.0);
            }
        }

        if (s.b[1016] && s.b[1127]) {
            s.store_mul(989, 1005, 2);
            s.store_mul(990, 1006, 2);
            s.store_scalar(991, (1.0 / (p.p808 + (0.5 * s.v[3]))));
            s.store_scalar(992, (1.0 / (p.p809 + (0.5 * s.v[3]))));
        }

        if (s.b[1016] && s.b[1127]) {
            if ((s.v[3] + s.v[310]) > 1e-9) {
                s.store_offset(1002, 310, s.v[3]);
            } else {
                s.store_scalar(1002, 1e-9);
            }
        }

        if (s.b[1016] && s.b[1127]) {
            if (((s.v[4] + s.v[311]) + p.p810) > 1e-9) {
                s.store_offset_add(1003, 4, 311, p.p810);
            } else {
                s.store_scalar(1003, 1e-9);
            }
        }

        if (s.b[1016] && s.b[1127]) {
            s.store_div_from_scalar_powf_ad(1000, 1.0, s.ad_value(1002), p.p818);
            s.store_div_from_scalar_powf_ad(1001, 1.0, s.ad_value(1003), p.p819);
            s.store_add_scaled_inputs_product_first_ad(993, A::scale_offset(s.ad_value(1000), p.p815, 1.0), (1.0 + (p.p814 * (s.v[353] - 1.0))), 1001, (p.p816 * (1.0 + (p.p814 * (s.v[353] - 1.0)))), 1000, 1001, (p.p817 * (1.0 + (p.p814 * (s.v[353] - 1.0)))));
            s.store_div_scaled_inputs2(994, s.ad_value(989), p.p811, s.ad_value(990), p.p811, s.ad_value(993), 1.0);
            s.store_div_scaled_inputs2(995, s.ad_value(991), p.p811, s.ad_value(992), p.p811, s.ad_value(993), 1.0);
            s.store_div_from_scalar_powf_ad(1000, 1.0, s.ad_value(1002), p.p824);
            s.store_div_from_scalar_powf_ad(1001, 1.0, s.ad_value(1003), p.p825);
            s.store_add_scaled_inputs_product_first_ad(996, A::scale_offset(s.ad_value(1000), p.p821, 1.0), 1.0, 1001, p.p822, 1000, 1001, p.p823);
            s.store_add_scaled_inputs4(998, s.ad_value(989), 1.0, s.ad_value(990), 1.0, s.ad_value(991), -1.0, s.ad_value(992), -1.0);
            s.store_div_scaled_offset_numerator(999, s.ad_value(994), 1.0, 1.0, A::offset(s.ad_value(995), 1.0), 1.0);
            s.store_mul(65, 65, 999);
            s.store_div_scaled_product3_mixed_iiaa(82, 82, 999, A::scale_offset(s.ad_value(995), p.p812, 1.0), 1.0, A::scale_offset(s.ad_value(994), p.p812, 1.0), 1.0);
            s.store_div_scaled_product3_mixed_iiaa(121, 121, 999, A::offset(A::mul(s.ad_value(39), s.ad_value(995)), 1.0), 1.0, A::offset(A::mul(s.ad_value(39), s.ad_value(994)), 1.0), 1.0);
            s.store_mul(150, 150, 999);
            s.store_div_scaled_inputs(999, s.ad_value(998), p.p820, s.ad_value(996), 1.0);
            s.store_add(40, 40, 999);
            s.store_add(145, 145, 999);
            s.store_div_scaled_inputs(999, s.ad_value(998), p.p826, A::powf(s.ad_value(996), p.p827), 1.0);
            s.store_add(62, 62, 999);
            s.store_add(155, 155, 999);
        }

        s.b[1128] = ((((s.v[11] > 0.0) || (s.v[12] > 0.0)) || (s.v[13] > 0.0)) || (s.v[8] > 0.0));
        s.v[1128] = if s.b[1128] { 1.0 } else { 0.0 };

        s.b[1129] = (((s.v[11] == 0.0) && (s.v[12] == 0.0)) && (s.v[13] == 0.0));
        s.v[1129] = if s.b[1129] { 1.0 } else { 0.0 };

        if ((s.b[1016] && s.b[1128]) && s.b[1129]) {
            s.store_offset(998, 4, s.v[8]);
            s.store_scalar(999, (1.0 / p.p828));
            s.store_div_from_scalar_scaled_input(11, (p.p828 * p.p828), 998, s.v[8]);
            s.store_div_scaled_add_product(12, A::exp_scaled_input(s.ad_value(999), ((-10.0) * s.v[8])), ((0.1 * s.v[8]) + (0.01 * p.p828)), A::scale_offset(s.ad_value(998), 0.1, (0.01 * p.p828)), A::exp(A::mul_scaled_lhs(s.ad_value(998), (-10.0), s.ad_value(999))), (-1.0), s.ad_value(4), 1.0);
            s.store_div_scaled_add_product(13, A::exp_scaled_input(s.ad_value(999), ((-20.0) * s.v[8])), ((0.05 * s.v[8]) + (0.0025 * p.p828)), A::scale_offset(s.ad_value(998), 0.05, (0.0025 * p.p828)), A::exp(A::mul_scaled_lhs(s.ad_value(998), (-20.0), s.ad_value(999))), (-1.0), s.ad_value(4), 1.0);
        }

        if (s.b[1016] && s.b[1128]) {
            s.store_add_scaled_inputs3(998, s.ad_value(11), 1.0, s.ad_value(12), p.p829, s.ad_value(13), p.p830);
            s.store_add_scaled_product_indices(40, 40, 1.0, 348, 998, 1.0);
            s.store_mul_offset_ad_rhs(65, 65, A::mul(s.ad_value(349), s.ad_value(998)), 1.0);
            s.store_add_scaled_product_indices(145, 145, 1.0, 348, 998, 1.0);
            s.store_mul_offset_ad_rhs(150, 150, A::mul(s.ad_value(349), s.ad_value(998)), 1.0);
        }

        s.copy_ad(175, 40);

        s.copy_ad(176, 41);

        s.copy_ad(177, 42);

        s.copy_ad(179, 43);

        s.copy_ad(180, 44);

        if (s.v[45] > 1e20) {
            if (s.v[45] < 1e26) {
                s.copy_ad(181, 45);
            } else {
                s.store_scalar(181, 1e26);
            }
        } else {
            s.store_scalar(181, 1e20);
        }

        if (s.v[46] > 0.01) {
            s.copy_ad(182, 46);
        } else {
            s.store_scalar(182, 0.01);
        }

        if (s.v[47] > 0.0) {
            s.copy_ad(183, 47);
        } else {
            s.store_scalar(183, 0.0);
        }

        s.copy_ad(184, 48);

        s.copy_ad(185, 49);

        if (s.v[50] > 0.0) {
            s.copy_ad(186, 50);
        } else {
            s.store_scalar(186, 0.0);
        }

        s.copy_ad(190, 55);

        s.copy_ad(191, 56);

        if (s.v[57] > 1e23) {
            if (s.v[57] < 1e27) {
                s.copy_ad(192, 57);
            } else {
                s.store_scalar(192, 1e27);
            }
        } else {
            s.store_scalar(192, 1e23);
        }

        if (s.v[58] > 1e23) {
            if (s.v[58] < 1e27) {
                s.copy_ad(193, 58);
            } else {
                s.store_scalar(193, 1e27);
            }
        } else {
            s.store_scalar(193, 1e23);
        }

        if (s.v[51] > 0.0) {
            s.copy_ad(187, 51);
        } else {
            s.store_scalar(187, 0.0);
        }

        if (s.v[53] > 0.0) {
            if (s.v[53] < 0.5) {
                s.copy_ad(189, 53);
            } else {
                s.store_scalar(189, 0.5);
            }
        } else {
            s.store_scalar(189, 0.0);
        }

        if (s.v[52] > 0.0) {
            if (s.v[52] < 1.0) {
                s.copy_ad(188, 52);
            } else {
                s.store_scalar(188, 1.0);
            }
        } else {
            s.store_scalar(188, 0.0);
        }

        s.copy_ad(178, 54);

        if (s.v[62] > 0.0) {
            s.copy_ad(194, 62);
        } else {
            s.store_scalar(194, 0.0);
        }

        if (s.v[64] > 0.0) {
            if (s.v[64] < 1.0) {
                s.copy_ad(196, 64);
            } else {
                s.store_scalar(196, 1.0);
            }
        } else {
            s.store_scalar(196, 0.0);
        }

        if (s.v[63] > 0.0) {
            s.copy_ad(195, 63);
        } else {
            s.store_scalar(195, 0.0);
        }

        if (s.v[59] > 0.0) {
            s.copy_ad(197, 59);
        } else {
            s.store_scalar(197, 0.0);
        }

        if (s.v[61] > 0.0) {
            if (s.v[61] < 1.0) {
                s.copy_ad(198, 61);
            } else {
                s.store_scalar(198, 1.0);
            }
        } else {
            s.store_scalar(198, 0.0);
        }

        if (s.v[60] > 0.0) {
            s.copy_ad(199, 60);
        } else {
            s.store_scalar(199, 0.0);
        }

        if (s.v[65] > 0.0) {
            s.copy_ad(200, 65);
        } else {
            s.store_scalar(200, 0.0);
        }

        s.copy_ad(201, 66);

        if (s.v[67] > 0.0) {
            s.copy_ad(202, 67);
        } else {
            s.store_scalar(202, 0.0);
        }

        s.copy_ad(203, 68);

        if (s.v[69] > 0.0) {
            s.copy_ad(204, 69);
        } else {
            s.store_scalar(204, 0.0);
        }

        s.copy_ad(205, 70);

        if (s.v[71] > 0.0) {
            s.copy_ad(206, 71);
        } else {
            s.store_scalar(206, 0.0);
        }

        s.copy_ad(207, 72);

        if (s.v[73] > 0.0) {
            s.copy_ad(208, 73);
        } else {
            s.store_scalar(208, 0.0);
        }

        s.copy_ad(209, 74);

        if (s.v[75] > 0.0) {
            s.copy_ad(210, 75);
        } else {
            s.store_scalar(210, 0.0);
        }

        s.copy_ad(211, 76);

    }

    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.copy_ad(212, 77);

        if (s.v[78] > 0.0) {
            s.copy_ad(213, 78);
        } else {
            s.store_scalar(213, 0.0);
        }

        s.copy_ad(214, 79);

        if (s.v[80] > (-0.5)) {
            if (s.v[80] < 1.0) {
                s.copy_ad(215, 80);
            } else {
                s.store_scalar(215, 1.0);
            }
        } else {
            s.store_scalar(215, (-0.5));
        }

        if (s.v[81] > (-0.5)) {
            s.copy_ad(216, 81);
        } else {
            s.store_scalar(216, (-0.5));
        }

        if (s.v[82] > 0.0) {
            s.copy_ad(217, 82);
        } else {
            s.store_scalar(217, 0.0);
        }

        s.copy_ad(218, 83);

        if (s.v[84] > (-0.5)) {
            if (s.v[84] < 1.0) {
                s.copy_ad(219, 84);
            } else {
                s.store_scalar(219, 1.0);
            }
        } else {
            s.store_scalar(219, (-0.5));
        }

        if (s.v[85] > (-0.5)) {
            s.copy_ad(220, 85);
        } else {
            s.store_scalar(220, (-0.5));
        }

        if (s.v[86] > 0.01) {
            s.copy_ad(221, 86);
        } else {
            s.store_scalar(221, 0.01);
        }

        if (s.v[87] > 2.0) {
            s.copy_ad(222, 87);
        } else {
            s.store_scalar(222, 2.0);
        }

        if (s.v[88] > 0.0) {
            s.copy_ad(223, 88);
        } else {
            s.store_scalar(223, 0.0);
        }

        if (s.v[89] > 0.0) {
            s.copy_ad(224, 89);
        } else {
            s.store_scalar(224, 0.0);
        }

        if (s.v[90] > 0.0) {
            s.copy_ad(225, 90);
        } else {
            s.store_scalar(225, 0.0);
        }

        s.copy_ad(226, 91);

        if (s.v[92] > 0.0) {
            s.copy_ad(227, 92);
        } else {
            s.store_scalar(227, 0.0);
        }

        s.copy_ad(228, 93);

        s.copy_ad(229, 94);

        if (s.v[95] > 0.0) {
            s.copy_ad(230, 95);
        } else {
            s.store_scalar(230, 0.0);
        }

        if (s.v[96] > 0.0) {
            s.copy_ad(231, 96);
        } else {
            s.store_scalar(231, 0.0);
        }

        if (s.v[97] > 1e-12) {
            s.copy_ad(232, 97);
        } else {
            s.store_scalar(232, 1e-12);
        }

        s.copy_ad(233, 98);

        if (s.v[99] > 0.0) {
            s.copy_ad(234, 99);
        } else {
            s.store_scalar(234, 0.0);
        }

        if (s.v[100] > 0.0) {
            s.copy_ad(235, 100);
        } else {
            s.store_scalar(235, 0.0);
        }

        if (s.v[101] > 0.0) {
            s.copy_ad(236, 101);
        } else {
            s.store_scalar(236, 0.0);
        }

        s.copy_ad(237, 102);

        s.copy_ad(238, 103);

        s.copy_ad(239, 104);

        s.copy_ad(240, 105);

        s.copy_ad(241, 106);

        s.copy_ad(242, 107);

        s.copy_ad(243, 108);

        s.copy_ad(244, 109);

        if (s.v[110] > 0.0) {
            s.copy_ad(245, 110);
        } else {
            s.store_scalar(245, 0.0);
        }

        if (s.v[111] > 0.0) {
            s.copy_ad(246, 111);
        } else {
            s.store_scalar(246, 0.0);
        }

        s.copy_ad(247, 112);

        s.copy_ad(248, 113);

        s.copy_ad(249, 114);

        s.copy_ad(250, 115);

        s.copy_ad(251, 116);

        s.copy_ad(252, 117);

        if (s.v[118] > 0.0) {
            s.copy_ad(253, 118);
        } else {
            s.store_scalar(253, 0.0);
        }

        s.copy_ad(254, 119);

        if (s.v[120] > 0.0) {
            s.copy_ad(255, 120);
        } else {
            s.store_scalar(255, 0.0);
        }

        if (s.v[121] > 0.0) {
            s.copy_ad(256, 121);
        } else {
            s.store_scalar(256, 0.0);
        }

        if (s.v[122] > 2.0) {
            s.copy_ad(257, 122);
        } else {
            s.store_scalar(257, 2.0);
        }

        s.copy_ad(258, 123);

        if (s.v[124] > 0.0) {
            s.copy_ad(259, 124);
        } else {
            s.store_scalar(259, 0.0);
        }

        if (s.v[125] > 0.0) {
            s.copy_ad(260, 125);
        } else {
            s.store_scalar(260, 0.0);
        }

        if (s.v[126] > 0.0) {
            s.copy_ad(261, 126);
        } else {
            s.store_scalar(261, 0.0);
        }

        s.copy_ad(262, 127);

        s.copy_ad(263, 128);

        s.copy_ad(264, 129);

        if (s.v[130] > 0.0) {
            s.copy_ad(265, 130);
        } else {
            s.store_scalar(265, 0.0);
        }

        if (s.v[131] > 0.0) {
            s.copy_ad(266, 131);
        } else {
            s.store_scalar(266, 0.0);
        }

        if (s.v[132] > 0.0) {
            s.copy_ad(267, 132);
        } else {
            s.store_scalar(267, 0.0);
        }

        s.copy_ad(268, 133);

        s.copy_ad(269, 134);

        s.copy_ad(270, 135);

        s.copy_ad(271, 136);

        if (s.v[137] > 0.0) {
            s.copy_ad(272, 137);
        } else {
            s.store_scalar(272, 0.0);
        }

        if (s.v[138] > 0.0) {
            s.copy_ad(273, 138);
        } else {
            s.store_scalar(273, 0.0);
        }

        s.copy_ad(274, 139);

        s.copy_ad(280, 145);

        s.copy_ad(281, 146);

        s.copy_ad(282, 147);

        if (s.v[148] > 1e20) {
            if (s.v[148] < 1e26) {
                s.copy_ad(283, 148);
            } else {
                s.store_scalar(283, 1e26);
            }
        } else {
            s.store_scalar(283, 1e20);
        }

        if (s.v[149] > 0.0) {
            s.copy_ad(284, 149);
        } else {
            s.store_scalar(284, 0.0);
        }

        if (s.v[150] > 0.0) {
            s.copy_ad(285, 150);
        } else {
            s.store_scalar(285, 0.0);
        }

        s.copy_ad(286, 151);

        if (s.v[152] > 0.0) {
            s.copy_ad(287, 152);
        } else {
            s.store_scalar(287, 0.0);
        }

        if (s.v[153] > 0.0) {
            if (s.v[153] < 1.0) {
                s.copy_ad(288, 153);
            } else {
                s.store_scalar(288, 1.0);
            }
        } else {
            s.store_scalar(288, 0.0);
        }

        if (s.v[154] > 0.0) {
            s.copy_ad(289, 154);
        } else {
            s.store_scalar(289, 0.0);
        }

        if (s.v[155] > 0.0) {
            s.copy_ad(290, 155);
        } else {
            s.store_scalar(290, 0.0);
        }

        if (s.v[157] > 0.0) {
            if (s.v[157] < 1.0) {
                s.copy_ad(292, 157);
            } else {
                s.store_scalar(292, 1.0);
            }
        } else {
            s.store_scalar(292, 0.0);
        }

        if (s.v[156] > 0.0) {
            s.copy_ad(291, 156);
        } else {
            s.store_scalar(291, 0.0);
        }

        if (s.v[173] > 0.0) {
            s.copy_ad(306, 173);
        } else {
            s.store_scalar(306, 0.0);
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
            s.copy_ad(191, 190);
            s.copy_ad(193, 192);
            s.copy_ad(246, 245);
            s.copy_ad(248, 247);
            s.copy_ad(250, 249);
            s.copy_ad(252, 251);
            s.copy_ad(236, 235);
            s.copy_ad(242, 240);
            s.copy_ad(243, 241);
            s.copy_ad(261, 260);
            s.copy_ad(263, 262);
            s.copy_ad(267, 266);
            s.copy_ad(273, 272);
        }

        s.store_scale(757, 180, 8.8541878176e-12);

        s.store_div(758, 757, 179);

        s.store_square(759, 179);

        s.store_scale(760, 758, 6.241449993689894e18);

        s.store_mul(761, 255, 181);

        if (s.v[761] > 1e20) {
            if (s.v[761] < 1e26) {
            } else {
                s.store_scalar(761, 1e26);
            }
        } else {
            s.store_scalar(761, 1e20);
        }

        s.v[762] = 0.0;

        s.b[1131] = (p.p51 > 0.0);
        s.v[1131] = if s.b[1131] { 1.0 } else { 0.0 };

        if s.b[1131] {
            s.store_scale_ad(762, A::powf(s.ad_value(758), 0.6666666666666666), ((0.4 * 5.951993) * p.p51));
        }

        s.b[1132] = (s.v[0] == (-1.0));
        s.v[1132] = if s.b[1132] { 1.0 } else { 0.0 };

        if (s.b[1131] && s.b[1132]) {
            s.store_scale(762, 762, (7.448711 / 5.951993));
        }

        s.store_scale(763, 758, (1e-8 * 1.0 / (s.v[756])));

        s.store_scale(764, 212, 0.5);

        s.v[765] = 0.5;

        s.b[1133] = (s.v[0] == (-1.0));
        s.v[1133] = if s.b[1133] { 1.0 } else { 0.0 };

        if s.b[1133] {
            s.store_scale(764, 212, 0.3333333333333333);
            s.store_scalar(765, 0.3333333333333333);
        }

        s.store_offset_pow_from_scalar_ad(997, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(222)), 1.0), (-1.0));

        s.store_div_scaled_product_offset_lhs(766, s.ad_value(997), (-1.0), A::offset(s.ad_value(997), (-1.0)), 1.0, {
            if ((4.0 * s.v[997]) > 0.0001) {
                A::scale(s.ad_value(997), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);

        s.store_offset_pow_from_scalar_ad(997, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(257)), 1.0), (-1.0));

        s.store_div_scaled_product_offset_lhs(767, s.ad_value(997), (-1.0), A::offset(s.ad_value(997), (-1.0)), 1.0, {
            if ((4.0 * s.v[997]) > 0.0001) {
                A::scale(s.ad_value(997), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);

        s.store_div_from_scalar(768, 1.0, 226);

        s.store_div(769, 757, 190);

        s.store_div(770, 757, 191);

        s.store_div_ad_lhs(771, A::sqrt_scaled_input(s.ad_value(192), (((2.0 * 1.6021918e-19) * s.v[756]) * s.v[356])), 769);

        s.store_div_ad_lhs(772, A::sqrt_scaled_input(s.ad_value(193), (((2.0 * 1.6021918e-19) * s.v[756]) * s.v[356])), 770);

        s.store_square(773, 771);

        s.store_square(774, 772);

        s.store_offset_div_ad(775, A::ln(A::offset(A::exp_scaled_input(s.ad_value(264), (0.005 * s.v[356])), (-1.0))), s.ad_value(264), (-((((((0.005 * s.v[356])) as f64).exp() - 1.0)) as f64).ln()));

    }

    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_add_ad_lhs(776, A::ln_scaled_input(s.ad_value(771), 0.5), 775);

        s.store_add_ad_lhs(777, A::ln_scaled_input(s.ad_value(772), 0.5), 775);

        s.store_div_from_scalar(809, 1.0, 771);

        s.store_offset_scaled(810, 771, 3.1, 8.5);

        s.store_square(778, 810);

        s.store_scale(811, 810, 0.5);

        s.b[1134] = (s.v[809] < 0.06);
        s.v[1134] = if s.b[1134] { 1.0 } else { 0.0 };

        if s.b[1134] {
            s.store_scale(779, 809, 64.0);
        }

        s.b[1135] = (s.v[809] <= 0.45);
        s.v[1135] = if s.b[1135] { 1.0 } else { 0.0 };

        if ((!s.b[1134]) && s.b[1135]) {
            s.store_offset_scaled(779, 809, 22.0, 3.0);
        }

        s.b[1136] = (s.v[809] <= 1.6);
        s.v[1136] = if s.b[1136] { 1.0 } else { 0.0 };

        if (((!s.b[1134]) && (!s.b[1135])) && s.b[1136]) {
            s.store_offset_scaled(779, 809, (-7.2), 15.5);
        }

        if (((!s.b[1134]) && (!s.b[1135])) && (!s.b[1136])) {
            s.copy_ad(779, 771);
        }

        s.store_add_scaled_inputs_product_right_ad(780, 811, 1.0, 773, 0.5, 771, A::sqrt(A::add_scaled_inputs3(s.ad_value(811), 1.0, s.ad_value(773), 0.25, s.ad_value(779), 1.0)), (-1.0));

        s.store_div_from_scalar(809, 1.0, 772);

        s.store_offset_scaled(810, 772, 3.1, 8.5);

        s.store_square(781, 810);

        s.store_scale(811, 810, 0.5);

        s.b[1137] = (s.v[809] < 0.06);
        s.v[1137] = if s.b[1137] { 1.0 } else { 0.0 };

        if s.b[1137] {
            s.store_scale(782, 809, 64.0);
        }

        s.b[1138] = (s.v[809] <= 0.45);
        s.v[1138] = if s.b[1138] { 1.0 } else { 0.0 };

        if ((!s.b[1137]) && s.b[1138]) {
            s.store_offset_scaled(782, 809, 22.0, 3.0);
        }

        s.b[1139] = (s.v[809] <= 1.6);
        s.v[1139] = if s.b[1139] { 1.0 } else { 0.0 };

        if (((!s.b[1137]) && (!s.b[1138])) && s.b[1139]) {
            s.store_offset_scaled(782, 809, (-7.2), 15.5);
        }

        if (((!s.b[1137]) && (!s.b[1138])) && (!s.b[1139])) {
            s.copy_ad(782, 772);
        }

        s.store_add_scaled_inputs_product_right_ad(783, 811, 1.0, 774, 0.5, 772, A::sqrt(A::add_scaled_inputs3(s.ad_value(811), 1.0, s.ad_value(774), 0.25, s.ad_value(782), 1.0)), (-1.0));

        s.store_div_from_scalar(784, 1.0, 244);

        s.store_scaled_sqrt_scaled_input(785, 244, ((2.0 * 1.6021918e-19) * 9.1093826e-31), ((4.0 * 0.3333333333333333) * 9.482522800157122e33));

        s.store_mul(786, 785, 179);

        s.store_mul(787, 785, 190);

        s.store_mul(788, 785, 191);

        s.v[789] = 0.0;

        s.b[1140] = (s.v[239] < 0.0);
        s.v[1140] = if s.b[1140] { 1.0 } else { 0.0 };

        if s.b[1140] {
            s.store_div_scaled_inputs(789, s.ad_value(238), (-0.495), s.ad_value(239), 1.0);
        }

        s.v[790] = 0.0;

        s.b[1141] = (s.v[241] < 0.0);
        s.v[1141] = if s.b[1141] { 1.0 } else { 0.0 };

        if s.b[1141] {
            s.store_div_scaled_inputs(790, s.ad_value(240), (-0.495), s.ad_value(241), 1.0);
        }

        s.b[1142] = (s.v[243] < 0.0);
        s.v[1142] = if s.b[1142] { 1.0 } else { 0.0 };

        if s.b[1142] {
            s.store_div_scaled_inputs(791, s.ad_value(242), (-0.495), s.ad_value(243), 1.0);
        }

        s.store_pow_from_scalar_ad(792, s.v[353], s.ad_value(237));

        s.store_mul(234, 234, 792);

        s.store_mul(235, 235, 792);

        s.store_mul(236, 236, 792);

        if ((1.0 + (s.v[249] * s.v[354])) > 0.0) {
            s.store_offset_scaled(785, 249, s.v[354], 1.0);
        } else {
            s.store_scalar(785, 0.0);
        }

        s.store_mul(711, 247, 785);

        s.store_scaled_mul(795, 711, 190, 500000000.0);

        if ((1.0 + (s.v[250] * s.v[354])) > 0.0) {
            s.store_offset_scaled(785, 250, s.v[354], 1.0);
        } else {
            s.store_scalar(785, 0.0);
        }

        s.store_mul(712, 248, 785);

        s.store_scaled_mul(796, 712, 191, 500000000.0);

        s.v[797] = 0.0;

        s.b[1143] = (s.v[270] > 1e-10);
        s.v[1143] = if s.b[1143] { 1.0 } else { 0.0 };

        if s.b[1143] {
            s.store_div_from_scalar(797, 0.75, 270);
        }

        s.store_square(798, 271);

        s.store_scale(20, 2, s.v[647]);

        s.store_scale(21, 2, s.v[648]);

        s.store_scale(22, 2, s.v[649]);

        s.store_scale(23, 2, s.v[674]);

        s.store_scale(24, 2, s.v[675]);

        s.store_scale(25, 2, s.v[676]);

        s.v[26] = 0.0;

        s.b[1151] = (p.p43 == 3.0);
        s.v[1151] = if s.b[1151] { 1.0 } else { 0.0 };

        if s.b[1151] {
            s.store_scalar(26, 1.0);
        }

        s.copy_ad(27, 313);

        s.b[1152] = (p.p39 == 0.0);
        s.v[1152] = if s.b[1152] { 1.0 } else { 0.0 };

        if s.b[1152] {
            s.store_scalar(27, (if (s.v[10] > 0.0) { s.v[10] } else { 0.0 }));
        }

        s.b[1153] = ((p.p43 == 2.0) || (p.p43 == 3.0));
        s.v[1153] = if s.b[1153] { 1.0 } else { 0.0 };

        if s.b[1153] {
            s.store_scale(20, 2, s.v[650]);
            s.store_add_scaled_product_indices(21, 2, s.v[651], 26, 27, (-1.0));
            s.copy_ad(22, 27);
            s.store_scale(23, 2, s.v[677]);
            s.store_add_scaled_product_indices(24, 2, s.v[678], 26, 27, (-1.0));
            s.copy_ad(25, 27);
        }

        s.b[1154] = (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0));
        s.v[1154] = if s.b[1154] { 1.0 } else { 0.0 };

        if s.b[1154] {
            if (s.v[20] > 0.0) {
                s.copy_ad(647, 20);
            } else {
                s.store_scalar(647, 0.0);
            }
        }

        if s.b[1154] {
            if (s.v[21] > 0.0) {
                s.copy_ad(648, 21);
            } else {
                s.store_scalar(648, 0.0);
            }
        }

        if s.b[1154] {
            if (s.v[22] > 0.0) {
                s.copy_ad(649, 22);
            } else {
                s.store_scalar(649, 0.0);
            }
        }

        if s.b[1154] {
            if (s.v[23] > 0.0) {
                s.copy_ad(674, 23);
            } else {
                s.store_scalar(674, 0.0);
            }
        }

        if s.b[1154] {
            if (s.v[24] > 0.0) {
                s.copy_ad(675, 24);
            } else {
                s.store_scalar(675, 0.0);
            }
        }

        if s.b[1154] {
            if (s.v[25] > 0.0) {
                s.copy_ad(676, 25);
            } else {
                s.store_scalar(676, 0.0);
            }
        }

        if (!s.b[1154]) {
            s.store_scalar(647, 0.0);
            s.store_scalar(648, 0.0);
            s.store_scalar(649, 0.0);
            s.store_scalar(674, 0.0);
            s.store_scalar(675, 0.0);
            s.store_scalar(676, 0.0);
        }

        s.v[657] = 0.0;

        s.v[684] = 0.0;

        s.v[659] = 0.0;

        s.v[686] = 0.0;

        s.v[658] = 0.0;

        s.v[685] = 0.0;

        s.v[660] = 0.0;

        s.v[687] = 0.0;

        s.v[655] = 0.0;

        s.v[682] = 0.0;

        s.v[656] = 0.0;

        s.v[683] = 0.0;

        s.v[652] = 1.0;

        s.v[679] = 1.0;

        s.v[653] = 1.0;

        s.v[680] = 1.0;

        s.v[654] = 1.0;

        s.v[681] = 1.0;

        s.v[502] = 0.0;

        s.b[1155] = (p.p43 > 0.0);
        s.v[1155] = if s.b[1155] { 1.0 } else { 0.0 };

        s.b[1156] = ((s.v[388] * s.v[647]) > 0.0);
        s.v[1156] = if s.b[1156] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1156]) {
            s.store_scaled_ln_ad(455, A::offset(A::div_from_scalar(p.p839, A::scale(s.ad_value(647), s.v[388])), 1.0), s.v[371]);
        }

        if (s.b[1155] && (!s.b[1156])) {
            s.store_scalar(455, 100000000.0);
        }

        s.b[1157] = ((s.v[389] * s.v[648]) > 0.0);
        s.v[1157] = if s.b[1157] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1157]) {
            s.store_scaled_ln_ad(456, A::offset(A::div_from_scalar(p.p839, A::scale(s.ad_value(648), s.v[389])), 1.0), s.v[371]);
        }

        if (s.b[1155] && (!s.b[1157])) {
            s.store_scalar(456, 100000000.0);
        }

        s.b[1158] = ((s.v[390] * s.v[649]) > 0.0);
        s.v[1158] = if s.b[1158] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1158]) {
            s.store_scaled_ln_ad(457, A::offset(A::div_from_scalar(p.p839, A::scale(s.ad_value(649), s.v[390])), 1.0), s.v[371]);
        }

        if (s.b[1155] && (!s.b[1158])) {
            s.store_scalar(457, 100000000.0);
        }

        if s.b[1155] {
            s.store_min3(655, 455, 456, 457);
        }

        s.b[1159] = ((((s.v[655] * s.v[372])) as f64).abs() < 230.25850929940458);
        s.v[1159] = if s.b[1159] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1159]) {
            s.store_exp_scaled_input(656, 655, s.v[372]);
        }

        s.b[1160] = ((s.v[655] * s.v[372]) < 0.0);
        s.v[1160] = if s.b[1160] { 1.0 } else { 0.0 };

        if ((s.b[1155] && (!s.b[1159])) && s.b[1160]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(656, 1e-100, (-230.25850929940458), A::scale(s.ad_value(655), s.v[372]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(655), s.v[372]), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if ((s.b[1155] && (!s.b[1159])) && (!s.b[1160])) {
            s.store_scaled_offset_ad(656, A::mul_offset_rhs(A::scale_offset(s.ad_value(655), s.v[372], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(655), s.v[372], (-230.25850929940458)), A::scale_offset(s.ad_value(655), ((s.v[372]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if s.b[1155] {
            s.store_scalar(397, s.v[394]);
            s.store_scalar(398, s.v[395]);
            s.store_scalar(399, s.v[396]);
            s.store_scalar(400, p.p848);
            s.store_scalar(401, p.p849);
            s.store_scalar(402, p.p850);
            s.store_scalar(403, p.p845);
            s.store_scalar(404, p.p846);
            s.store_scalar(405, p.p847);
        }

        s.b[1161] = (s.v[647] == 0.0);
        s.v[1161] = if s.b[1161] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1161]) {
            s.store_scalar(397, (s.v[395] + s.v[396]));
            s.store_scalar(400, (0.9 * (p.p849).min(p.p850)));
            s.store_scalar(403, (p.p846 + p.p847));
        }

        s.b[1162] = (s.v[648] == 0.0);
        s.v[1162] = if s.b[1162] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1162]) {
            s.store_scalar(398, (s.v[394] + s.v[396]));
            s.store_scalar(401, (0.9 * (p.p848).min(p.p850)));
            s.store_scalar(404, (p.p845 + p.p847));
        }

        s.b[1163] = (s.v[649] == 0.0);
        s.v[1163] = if s.b[1163] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1163]) {
            s.store_scalar(399, (s.v[394] + s.v[395]));
            s.store_scalar(402, (0.9 * (p.p848).min(p.p849)));
            s.store_scalar(405, (p.p845 + p.p846));
        }

        if s.b[1155] {
            s.store_min3(657, 397, 398, 399);
            s.store_scale(658, 657, 0.1);
            s.store_max3(378, 400, 401, 402);
            s.store_mul_sub_from_scalar_ad_rhs(659, 657, 1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(378))));
            s.store_offset_min_ad(660, A::min(s.ad_value(403), s.ad_value(404)), s.ad_value(405), (-0.05));
        }

        s.b[1164] = ((s.v[564] * s.v[674]) > 0.0);
        s.v[1164] = if s.b[1164] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1164]) {
            s.store_scaled_ln_ad(455, A::offset(A::div_from_scalar(p.p839, A::mul(s.ad_value(564), s.ad_value(674))), 1.0), s.v[371]);
        }

        if (s.b[1155] && (!s.b[1164])) {
            s.store_scalar(455, 100000000.0);
        }

        s.b[1165] = ((s.v[565] * s.v[675]) > 0.0);
        s.v[1165] = if s.b[1165] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1165]) {
            s.store_scaled_ln_ad(456, A::offset(A::div_from_scalar(p.p839, A::mul(s.ad_value(565), s.ad_value(675))), 1.0), s.v[371]);
        }

        if (s.b[1155] && (!s.b[1165])) {
            s.store_scalar(456, 100000000.0);
        }

    }

    pub(super) fn stamp_reactive_block_8(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[1166] = ((s.v[566] * s.v[676]) > 0.0);
        s.v[1166] = if s.b[1166] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1166]) {
            s.store_scaled_ln_ad(457, A::offset(A::div_from_scalar(p.p839, A::mul(s.ad_value(566), s.ad_value(676))), 1.0), s.v[371]);
        }

        if (s.b[1155] && (!s.b[1166])) {
            s.store_scalar(457, 100000000.0);
        }

        if s.b[1155] {
            s.store_min3(682, 455, 456, 457);
        }

        s.b[1167] = ((((s.v[682] * s.v[372])) as f64).abs() < 230.25850929940458);
        s.v[1167] = if s.b[1167] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1167]) {
            s.store_exp_scaled_input(683, 682, s.v[372]);
        }

        s.b[1168] = ((s.v[682] * s.v[372]) < 0.0);
        s.v[1168] = if s.b[1168] { 1.0 } else { 0.0 };

        if ((s.b[1155] && (!s.b[1167])) && s.b[1168]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(683, 1e-100, (-230.25850929940458), A::scale(s.ad_value(682), s.v[372]), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(682), s.v[372]), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if ((s.b[1155] && (!s.b[1167])) && (!s.b[1168])) {
            s.store_scaled_offset_ad(683, A::mul_offset_rhs(A::scale_offset(s.ad_value(682), s.v[372], (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(682), s.v[372], (-230.25850929940458)), A::scale_offset(s.ad_value(682), ((s.v[372]) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if s.b[1155] {
            s.copy_ad(397, 570);
            s.copy_ad(398, 571);
            s.copy_ad(399, 572);
            s.copy_ad(400, 512);
            s.copy_ad(401, 513);
            s.copy_ad(402, 514);
            s.copy_ad(403, 509);
            s.copy_ad(404, 510);
            s.copy_ad(405, 511);
        }

        s.b[1169] = (s.v[674] == 0.0);
        s.v[1169] = if s.b[1169] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1169]) {
            s.store_add(397, 571, 572);
            s.store_scale_ad(400, A::min(s.ad_value(513), s.ad_value(514)), 0.9);
            s.store_add(403, 510, 511);
        }

        s.b[1170] = (s.v[675] == 0.0);
        s.v[1170] = if s.b[1170] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1170]) {
            s.store_add(398, 570, 572);
            s.store_scale_ad(401, A::min(s.ad_value(512), s.ad_value(514)), 0.9);
            s.store_add(404, 509, 511);
        }

        s.b[1171] = (s.v[676] == 0.0);
        s.v[1171] = if s.b[1171] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1171]) {
            s.store_add(399, 570, 571);
            s.store_scale_ad(402, A::min(s.ad_value(512), s.ad_value(513)), 0.9);
            s.store_add(405, 509, 510);
        }

        if s.b[1155] {
            s.store_min3(684, 397, 398, 399);
            s.store_scale(685, 684, 0.1);
            s.store_max3(378, 400, 401, 402);
            s.store_mul_sub_from_scalar_ad_rhs(686, 684, 1.0, A::pow_from_scalar(2.0, A::div_from_scalar((-1.0), s.ad_value(378))));
            s.store_offset_min_ad(687, A::min(s.ad_value(403), s.ad_value(404)), s.ad_value(405), (-0.05));
        }

        s.b[1172] = (s.v[475] == 1.0);
        s.v[1172] = if s.b[1172] { 1.0 } else { 0.0 };

        if (s.b[1155] && s.b[1172]) {
            s.store_add_scaled_inputs3(502, s.ad_value(647), (s.v[415] * p.p946), s.ad_value(648), (s.v[416] * p.p946), s.ad_value(649), (s.v[417] * p.p946));
        }

        s.b[1507] = ((s.v[647] * s.v[415]) <= s.v[502]);
        s.v[1507] = if s.b[1507] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1507]) {
            s.store_scalar(652, 0.0);
        }

        s.b[1508] = ((s.v[648] * s.v[416]) <= s.v[502]);
        s.v[1508] = if s.b[1508] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1508]) {
            s.store_scalar(653, 0.0);
        }

        s.b[1509] = ((s.v[649] * s.v[417]) <= s.v[502]);
        s.v[1509] = if s.b[1509] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1509]) {
            s.store_scalar(654, 0.0);
        }

        if (s.b[1155] && s.b[1172]) {
            s.store_mul_ad_rhs(502, 554, A::add_scaled_products3(s.ad_value(674), s.ad_value(582), 1.0, s.ad_value(675), s.ad_value(583), 1.0, s.ad_value(676), s.ad_value(584), 1.0));
        }

        s.b[1797] = ((s.v[674] * s.v[582]) <= s.v[502]);
        s.v[1797] = if s.b[1797] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1797]) {
            s.store_scalar(679, 0.0);
        }

        s.b[1798] = ((s.v[675] * s.v[583]) <= s.v[502]);
        s.v[1798] = if s.b[1798] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1798]) {
            s.store_scalar(680, 0.0);
        }

        s.b[1799] = ((s.v[676] * s.v[584]) <= s.v[502]);
        s.v[1799] = if s.b[1799] { 1.0 } else { 0.0 };

        if ((s.b[1155] && s.b[1172]) && s.b[1799]) {
            s.store_scalar(681, 0.0);
        }

        s.v[1929] = 0.0;

        s.v[1930] = 0.0;

        s.v[1931] = 0.0;

        s.store_offset_voltage(357, ctx, nodes, Some(4), None, s.v[352]);

        s.store_square(358, 357);

        s.store_offset(359, 357, (-s.v[351]));

        s.store_div_from_scalar(360, s.v[351], 357);

        s.store_ln(361, 360);

        s.store_scale(1916, 357, (1.3806505e-23 * 6.241449993689894e18));

        s.store_div_from_scalar(362, 1.0, 1916);

        s.store_sub_scaled_ad_lhs(363, A::sub_from_scalar(1.179, A::scale(s.ad_value(357), 9.025e-5)), 358, 3.05e-7);

        s.store_mul_ad_affine_product_lhs(364, A::scale_offset(s.ad_value(357), 0.00045, 1.045), A::sub_scaled_inputs(A::scale_offset(s.ad_value(357), 0.0014, 0.523), 1.0, s.ad_value(358), 1.48e-6), 1.1111111111111112e-5, 0.0, 358);

        if (!(s.v[364] > 0.001)) {
            s.store_scalar(364, 0.001);
        }

        s.store_add_scaled_inputs_product_right_ad(717, 363, 1.0, 185, 1.0, 1916, A::ln_scaled_input(A::mul(s.ad_value(181), A::powf(s.ad_value(364), (-0.75))), 4e-26), 2.0);

        if (!(s.v[717] > 0.05)) {
            s.store_scalar(717, 0.05);
        }

        s.store_div_ad_lhs(718, A::sqrt(A::mul_scaled_lhs(s.ad_value(181), ((2.0 * 1.6021918e-19) * s.v[756]), s.ad_value(362))), 758);

        s.v[719] = 0.0;

        s.v[720] = 0.0;

        s.b[2004] = (s.v[186] > 0.0);
        s.v[2004] = if s.b[2004] { 1.0 } else { 0.0 };

        if s.b[2004] {
            s.store_div_from_scalar(721, 80000000.0, 759);
        }

        if s.b[2004] {
            if (s.v[186] > s.v[721]) {
                s.copy_ad(720, 186);
            } else {
                s.copy_ad(720, 721);
            }
        }

        if s.b[2004] {
            if (5e24 > s.v[720]) {
                s.store_scalar(720, 5e24);
            } else {
            }
        }

        if s.b[2004] {
            s.store_div_scaled_product3_indices(719, 758, 758, 1916, 2.0, 720, (1.6021918e-19 * s.v[756]));
        }

        s.store_scaled_mul(722, 1916, 1916, 100.0);

        s.b[2005] = (p.p51 > 0.0);
        s.v[2005] = if s.b[2005] { 1.0 } else { 0.0 };

        if s.b[2005] {
            s.store_sqrt_mul_ad(723, A::mul3(s.ad_value(1916), s.ad_value(718), s.ad_value(718)), s.ad_value(717));
            s.store_mul_scaled_powf_rhs(724, 762, 0.75, 723, 0.6666666666666666);
            s.store_add(717, 717, 724);
            s.store_mul_offset_ad_rhs(718, 718, A::div_scaled_inputs(s.ad_value(724), (2.0 * 0.6666666666666666), s.ad_value(723), 1.0), 1.0);
        }

        s.store_sqrt(725, 717);

        s.store_scale(726, 717, 0.95);

        s.store_scaled_mul(727, 717, 717, 0.0025);

        s.copy_ad(728, 727);

        s.store_scaled_sqrt(729, 728, 0.5);

        s.store_add_scaled_inputs3_sqrt_third_mixed_iia(730, 726, 0.5, 729, ((-1.0) * 0.5), A::add_scaled_product(s.ad_value(727), 1.0, A::sub(s.ad_value(726), s.ad_value(729)), A::sub(s.ad_value(726), s.ad_value(729)), 1.0), (-0.5));

        s.store_scaled_add(731, 717, 363, 0.5);

        s.store_sub_ad_lhs(732, A::sqrt(A::add(s.ad_value(183), s.ad_value(717))), 725);

        s.store_add_scaled_inputs3_sqrt_first_mixed_aii(733, A::add_scaled_inputs3(s.ad_value(183), 1.0, s.ad_value(184), 1.0, s.ad_value(717), 1.0), 1.0, 725, (-1.0), 732, -1.0);

        s.store_add_scaled_product_mixed_aia(734, A::add_scaled_inputs3(s.ad_value(363), 1.0, s.ad_value(185), 1.0, s.ad_value(254), 1.0), 1.0, 1916, A::ln_scaled_input(A::mul(s.ad_value(761), A::powf(s.ad_value(364), (-0.75))), 4e-26), 2.0);

        if (!(s.v[734] > 0.05)) {
            s.store_scalar(734, 0.05);
        }

        s.store_div_ad_lhs(735, A::sqrt(A::mul_scaled_lhs(s.ad_value(761), ((2.0 * 1.6021918e-19) * s.v[756]), s.ad_value(362))), 758);

        s.b[2006] = (p.p51 > 0.0);
        s.v[2006] = if s.b[2006] { 1.0 } else { 0.0 };

        if s.b[2006] {
            s.store_sqrt_mul_ad(723, A::mul3(s.ad_value(1916), s.ad_value(735), s.ad_value(735)), s.ad_value(734));
            s.store_mul_scaled_powf_rhs(724, 762, 0.75, 723, 0.6666666666666666);
            s.store_add(734, 734, 724);
            s.store_mul_offset_ad_rhs(735, 735, A::div_scaled_inputs(s.ad_value(724), (2.0 * 0.6666666666666666), s.ad_value(723), 1.0), 1.0);
        }

        s.store_scale(736, 734, 0.95);

        s.store_scaled_mul(737, 734, 734, 0.0025);

        s.copy_ad(738, 737);

        s.store_scaled_sqrt(729, 738, 0.5);

        s.store_add_scaled_inputs3_sqrt_third_mixed_iia(739, 736, 0.5, 729, ((-1.0) * 0.5), A::add_scaled_product(s.ad_value(737), 1.0, A::sub(s.ad_value(736), s.ad_value(729)), A::sub(s.ad_value(736), s.ad_value(729)), 1.0), (-0.5));

        s.store_offset_add_ad(701, s.ad_value(175), A::mul3(s.ad_value(176), s.ad_value(359), A::offset(A::mul(s.ad_value(177), s.ad_value(359)), 1.0)), s.v[17]);

        s.store_exp_mul(740, 178, 361);

        s.store_mul(702, 187, 740);

        s.store_div(703, 188, 360);

        s.store_exp_mul(741, 201, 361);

        s.store_mul(704, 200, 741);

        s.store_scaled_mul(1917, 704, 758, s.v[16]);

        s.store_mul_exp_ad_rhs(706, 204, A::mul(s.ad_value(205), s.ad_value(361)));

        s.store_exp_mul(742, 203, 361);

        s.store_mul(705, 202, 742);

        s.store_mul_exp_ad_rhs(708, 208, A::mul(s.ad_value(209), s.ad_value(361)));

        s.store_exp_mul(743, 207, 361);

        s.store_mul(707, 206, 743);

        s.store_exp_mul(744, 211, 361);

        s.store_mul(709, 210, 744);

        s.store_exp_mul(745, 214, 361);

        s.store_mul(710, 213, 745);

        s.store_scaled_mul(746, 1917, 710, 2.0);

        s.store_exp_mul(747, 218, 361);

        s.store_mul(1921, 217, 747);

        s.store_mul(1922, 256, 747);

        s.store_mul_exp_ad_rhs(713, 228, A::mul_scaled_lhs(s.ad_value(229), -1.0, s.ad_value(361)));

        s.store_scaled_mul(1920, 274, 357, (4.0 * 1.3806505e-23));

        s.b[2007] = ((p.p46 != 0.0) && (s.v[285] > 0.0));
        s.v[2007] = if s.b[2007] { 1.0 } else { 0.0 };

        if s.b[2007] {
            s.store_offset_add_scaled_product(714, s.ad_value(280), 1.0, s.ad_value(281), s.ad_value(359), 1.0, s.v[19]);
            s.store_exp_mul(748, 286, 361);
            s.store_mul(715, 285, 748);
            s.store_scaled_mul(1918, 715, 758, s.v[18]);
            s.store_mul_offset_ad_rhs(1924, 1916, A::mul(s.ad_value(284), s.ad_value(360)), 1.0);
            s.store_add_scaled_inputs_product_right_ad(749, 363, 1.0, 282, 1.0, 1924, A::ln_scaled_input(A::mul(s.ad_value(283), A::powf(s.ad_value(364), (-0.75))), 4e-26), 2.0);
        }

        if s.b[2007] {
            if (s.v[749] > 0.05) {
            } else {
                s.store_scalar(749, 0.05);
            }
        }

        if s.b[2007] {
            s.store_div_ad_lhs(750, A::sqrt(A::mul_scaled_lhs(s.ad_value(283), ((2.0 * 1.6021918e-19) * s.v[756]), s.ad_value(362))), 758);
            s.store_square(1925, 750);
            s.store_ln(1926, 1925);
            s.store_scale(751, 749, 0.95);
            s.store_scaled_mul(752, 749, 749, 0.0025);
            s.copy_ad(753, 752);
            s.store_scaled_sqrt(754, 753, 0.5);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(755, 751, 0.5, 754, ((-1.0) * 0.5), A::add_scaled_product(s.ad_value(752), 1.0, A::sub(s.ad_value(751), s.ad_value(754)), A::sub(s.ad_value(751), s.ad_value(754)), 1.0), (-0.5));
        }

        if (!s.b[2007]) {
            s.store_scalar(714, 0.0);
            s.store_scalar(748, 1.0);
            s.store_scalar(715, 0.0);
            s.store_scalar(1918, 0.0);
            s.copy_ad(1924, 1916);
            s.store_scalar(749, 0.0);
            s.store_scalar(750, 1.0);
            s.store_scalar(1925, 1.0);
            s.store_scalar(1926, 0.0);
            s.store_scalar(751, 0.0);
            s.store_scalar(752, 0.0);
            s.store_scalar(753, 0.0);
            s.store_scalar(754, 0.0);
            s.store_scalar(755, 0.0);
        }

        s.b[2008] = (s.v[0] == 1.0);
        s.v[2008] = if s.b[2008] { 1.0 } else { 0.0 };

        if s.b[2008] {
            s.store_voltage(814, ctx, nodes, Some(6), Some(7));
        }

    }

    pub(super) fn stamp_reactive_block_9(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[2008] {
            s.store_voltage(815, ctx, nodes, Some(8), Some(7));
            s.store_voltage(816, ctx, nodes, Some(7), Some(9));
            s.store_scaled_voltage(821, ctx, nodes, Some(7), Some(11), -1.0);
            s.store_scaled_voltage(822, ctx, nodes, Some(8), Some(12), -1.0);
        }

        if (!s.b[2008]) {
            s.store_scaled_voltage(814, ctx, nodes, Some(6), Some(7), -1.0);
            s.store_scaled_voltage(815, ctx, nodes, Some(8), Some(7), -1.0);
            s.store_scaled_voltage(816, ctx, nodes, Some(7), Some(9), -1.0);
            s.store_voltage(821, ctx, nodes, Some(7), Some(11));
            s.store_voltage(822, ctx, nodes, Some(8), Some(12));
        }

        s.store_add(818, 814, 816);

        s.copy_ad(823, 814);

        s.copy_ad(824, 816);

        s.store_add(825, 815, 816);

        s.store_sub(826, 814, 815);

        s.store_scale(1801, 823, (-s.v[356]));

        s.store_scale(1802, 826, (-s.v[356]));

        s.store_scaled_sub(1803, 818, 701, (-s.v[356]));

        s.v[820] = 1.0;

        s.b[2009] = (s.v[815] < 0.0);
        s.v[2009] = if s.b[2009] { 1.0 } else { 0.0 };

        if s.b[2009] {
            s.store_scalar(820, (-1.0));
            s.store_sub(814, 814, 815);
            s.store_add(816, 816, 815);
            s.store_neg(815, 815);
        }

        s.store_add(817, 815, 816);

        s.store_div_scaled_product_offset_denominator(819, s.ad_value(815), s.ad_value(815), 1.0, A::sqrt(A::offset(A::square(s.ad_value(815)), 0.01)), 0.1, 1.0);

        s.store_add_scaled_inputs4(2013, s.ad_value(817), 0.5, s.ad_value(816), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(728), 1.0, A::sub(s.ad_value(817), s.ad_value(816)), A::sub(s.ad_value(817), s.ad_value(816)), 1.0)), (-0.5), s.ad_value(726), 1.0);

        s.copy_ad(1804, 2013);

        s.store_add_scaled_inputs4(1932, s.ad_value(816), 1.0, s.ad_value(2013), (-0.5), A::sqrt(A::add_scaled_product(s.ad_value(727), 1.0, s.ad_value(2013), s.ad_value(2013), 1.0)), (-(-0.5)), s.ad_value(730), 1.0);

        s.copy_ad(1805, 1932);

        s.v[1933] = 0.0;

        s.b[2169] = ((p.p45 != 0.0) && (s.v[182] != 1.0));
        s.v[2169] = if s.b[2169] { 1.0 } else { 0.0 };

        if s.b[2169] {
            s.store_add_scaled_inputs3(1934, s.ad_value(1932), 1.0, s.ad_value(815), 0.5, s.ad_value(819), (-0.5));
            s.store_sub_ad_lhs(1935, A::sqrt(A::add(s.ad_value(1934), s.ad_value(717))), 725);
            s.store_offset_div_scaled_inputs2(1929, s.ad_value(1935), 2.0, s.ad_value(732), (-2.0), s.ad_value(733), 1.0, (-1.0));
            s.store_add_scaled_product_mixed_iaa(1936, 1935, 1.0, A::mul_sub_from_scalar_lhs_scaled_output(1.0, s.ad_value(182), s.ad_value(733), 0.25), A::add(s.ad_value(1929), A::sqrt(A::offset(A::square(s.ad_value(1929)), 0.4804530139182))), (-1.0));
            s.store_add_scaled_square_product_indices(1937, 1936, 1.0, 725, 1936, 2.0);
            s.store_add_scaled_inputs3(1932, s.ad_value(1937), 1.0, s.ad_value(815), (-0.5), s.ad_value(819), (-(-0.5)));
            s.store_sub(1933, 1805, 1932);
        }

        s.copy_ad(2010, 717);

        s.copy_ad(2011, 727);

        s.copy_ad(2012, 718);

        s.copy_ad(2014, 1932);

        s.copy_ad(2018, 1933);

        s.copy_ad(2015, 1921);

        s.copy_ad(2016, 766);

        s.store_add_scaled_inputs3(2017, s.ad_value(818), 1.0, s.ad_value(2018), (-1.0), s.ad_value(701), -1.0);

        s.store_add_scaled_inputs3(2019, s.ad_value(2014), 1.0, s.ad_value(815), 0.5, s.ad_value(819), (-0.5));

        s.v[2031] = 1.0;

        s.b[2170] = (s.v[188] > 0.0);
        s.v[2170] = if s.b[2170] { 1.0 } else { 0.0 };

        if s.b[2170] {
            s.store_mul(2022, 2010, 362);
            s.store_mul(2023, 2019, 362);
            s.store_mul(2024, 2017, 362);
            s.store_offset_div_scaled_inputs(1930, s.ad_value(2012), 0.5, A::sqrt(s.ad_value(2022)), 1.0, 1.0);
            s.store_add_scaled_product_right_ad(1931, 2022, 1.0, 2012, A::sqrt(s.ad_value(2022)), 1.0);
            s.store_add_scaled_inputs_product_mixed_aiai(2025, A::div_scaled_inputs2(s.ad_value(2024), 1.0, s.ad_value(1931), (-1.0), s.ad_value(1930), 1.0), 1.0, 2022, 0.5, A::offset(s.ad_value(189), 1.0), 2023, (-1.0));
            s.store_offset_scaled(2026, 2022, 0.5, 2.0);
            s.store_add(2027, 2022, 2023);
            s.store_sub_scaled_inputs_ad(1930, A::add_scaled_inputs_product(s.ad_value(2024), 1.0, s.ad_value(2027), (-1.0), s.ad_value(2012), A::sqrt(s.ad_value(2027)), (-1.0)), 1.0, A::ln(A::add(A::div(s.ad_value(2022), s.ad_value(2012)), A::sqrt(s.ad_value(2022)))), 2.0);
            s.store_add_scaled_inputs(2028, 1930, 2.0, 2026, 1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1930, 2025, 0.5, 2028, 0.5, A::offset(A::mul(A::sub(s.ad_value(2025), s.ad_value(2028)), A::sub(s.ad_value(2025), s.ad_value(2028))), 20.0), 0.5);
            s.store_add_scaled_inputs3(1931, s.ad_value(2024), 2.0, s.ad_value(2023), (-2.0), s.ad_value(2026), -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2029, 1930, 0.5, 1931, 0.5, A::offset(A::mul(A::sub(s.ad_value(1930), s.ad_value(1931)), A::sub(s.ad_value(1930), s.ad_value(1931))), 20.0), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1930, 2029, 0.5, 2026, 0.5, A::offset(A::mul(A::sub(s.ad_value(2029), s.ad_value(2026)), A::sub(s.ad_value(2029), s.ad_value(2026))), 5.0), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2030, 1930, 0.5, 2026, ((-1.0) * 0.5), A::offset(A::mul(A::sub_scaled_inputs(s.ad_value(1930), 1.0, s.ad_value(2026), -1.0), A::sub_scaled_inputs(s.ad_value(1930), 1.0, s.ad_value(2026), -1.0)), 20.0), 0.5);
            s.store_mul_offset_ad_rhs(1931, 703, A::div(s.ad_value(2030), s.ad_value(2026)), 1.0);
        }

        s.b[2171] = (s.v[1931] > (-230.25850929940458));
        s.v[2171] = if s.b[2171] { 1.0 } else { 0.0 };

        if (s.b[2170] && s.b[2171]) {
            s.store_exp(2031, 1931);
        }

        if (s.b[2170] && (!s.b[2171])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2031, 1e-100, (-230.25850929940458), 1931, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.store_offset_mul(2032, 702, 2031, 1.0);

        s.store_mul(2033, 1916, 2032);

        s.store_mul_ad_product_rhs(2034, 197, A::offset(A::mul(s.ad_value(199), s.ad_value(819)), 1.0), A::offset(A::mul(s.ad_value(198), s.ad_value(2019)), 1.0));

        s.store_mul_offset_rhs(2035, 2033, 2034, 1.0);

        s.store_div_from_scalar(2036, 1.0, 2035);

        s.store_mul_sqrt_ad_rhs(2020, 2012, A::mul(s.ad_value(1916), s.ad_value(2036)));

        s.store_square(2021, 2020);

        s.store_div_from_scalar(2037, 1.0, 2021);

        s.store_mul(2038, 2014, 2036);

        s.store_mul(2039, 2017, 2036);

        s.store_div_scaled_value_offset_denominator(2040, s.ad_value(819), 2.0, A::sqrt(A::offset(A::mul(s.ad_value(195), s.ad_value(819)), 1.0)), 1.0, 1.0);

        s.store_mul_ad_product_rhs(2041, 194, s.ad_value(2040), A::offset(A::mul(s.ad_value(196), s.ad_value(2019)), 1.0));

        s.store_mul(2042, 2010, 2036);

        s.store_sqrt_square_add(1930, 2013, 2011);

        s.store_sqrt_ad(1931, A::add_scaled_product(s.ad_value(2011), 1.0, A::sub(s.ad_value(2013), s.ad_value(2041)), A::sub(s.ad_value(2013), s.ad_value(2041)), 1.0));

        s.store_mul_scaled_ad_rhs(2043, 2036, 0.5, A::add_scaled_inputs3(s.ad_value(2041), 1.0, s.ad_value(1930), 1.0, s.ad_value(1931), -1.0));

        s.store_add(2044, 2042, 2038);

        s.store_sub(2045, 2044, 2043);

        s.b[2172] = (p.p45 > 0.0);
        s.v[2172] = if s.b[2172] { 1.0 } else { 0.0 };

        s.b[2173] = (((s.v[2045]) as f64).abs() < 1e-5);
        s.v[2173] = if s.b[2173] { 1.0 } else { 0.0 };

        if (s.b[2172] && s.b[2173]) {
            s.store_offset_ad(2046, A::mul_sub_from_scalar_rhs(s.ad_value(2020), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2045), 1.0, A::scale(s.ad_value(2045), 0.3125), 0.5)), 1.0);
        }

        s.b[2174] = (s.v[2045] < 460.51701859880916);
        s.v[2174] = if s.b[2174] { 1.0 } else { 0.0 };

        if ((s.b[2172] && (!s.b[2173])) && s.b[2174]) {
            s.store_exp_neg_input(2060, 2045);
        }

        if ((s.b[2172] && (!s.b[2173])) && (!s.b[2174])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2060, 1e-200, 2045, (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2045), (-460.51701859880916), A::scale_offset(s.ad_value(2045), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if (s.b[2172] && (!s.b[2173])) {
            s.store_scalar(1929, (if (s.v[2045] > 0.0) { 1.0 } else { (-1.0) }));
        }

        if (s.b[2172] && (!s.b[2173])) {
            s.store_offset_ad(2046, A::div_scaled_product3(s.ad_value(1929), s.ad_value(2020), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(2060), 1.0, s.ad_value(2045))), 1.0, A::sqrt(A::mul_sub_from_scalar_rhs(s.ad_value(2045), 1.0, s.ad_value(2060))), 2.0), 1.0);
        }

        if (!s.b[2172]) {
            s.store_offset_div_scaled_inputs(2046, s.ad_value(2020), 0.5, A::sqrt(s.ad_value(2045)), 1.0, 1.0);
        }

        s.store_add_scaled_value_products(2047, s.ad_value(2045), 1.0, s.ad_value(2020), A::sqrt(s.ad_value(2045)), 1.0, s.ad_value(2046), A::ln(A::offset(s.ad_value(2046), (-1.0))), (-1.0));

        s.store_div_scaled_inputs2(2048, s.ad_value(2039), 1.0, s.ad_value(2047), (-1.0), s.ad_value(2046), 1.0);

        s.store_mul_scaled_offset_ad_rhs(2054, 2021, 0.5, A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2021)), 1.0)), (-1.0));

        s.v[2053] = 0.0;

        s.v[2055] = 1.0;

        s.b[2175] = (s.v[2048] > (-30.0));
        s.v[2175] = if s.b[2175] { 1.0 } else { 0.0 };

        if s.b[2175] {
            s.store_offset_mul(2049, 2046, 2048, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(1929, 2049, 2049, 10.0, 0.5);
            s.store_sub_ad_rhs(2050, 2048, A::ln(s.ad_value(1929)));
            s.store_scaled_add_sqrt_square_offset_rhs(2051, 2050, 2050, 2.0, 0.5);
        }

        s.b[2176] = ((s.v[2048] - s.v[2051]) < 230.25850929940458);
        s.v[2176] = if s.b[2176] { 1.0 } else { 0.0 };

        if (s.b[2175] && s.b[2176]) {
            s.store_exp_sub(1929, 2048, 2051);
        }

        if (s.b[2175] && (!s.b[2176])) {
            s.store_scaled_offset_mul_offset_lhs_ad(1929, A::sub(s.ad_value(2048), s.ad_value(2051)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2048), s.ad_value(2051)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2048), s.ad_value(2051)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if s.b[2175] {
            s.store_div(2052, 1929, 2046);
            s.store_sub_ad_lhs(1929, A::scaled_offset(s.ad_value(2051), 1.0, 2.0), 2052);
        }

        s.b[2177] = (s.v[2052] > 1e-6);
        s.v[2177] = if s.b[2177] { 1.0 } else { 0.0 };

        if (s.b[2175] && s.b[2177]) {
            s.store_mul_offset_ad_rhs(2053, 2046, A::sub(s.ad_value(2051), A::div_scaled_offset_numerator(A::sqrt(A::offset(A::mul(s.ad_value(2052), s.ad_value(1929)), 1.0)), 1.0, (-1.0), s.ad_value(2052), 1.0)), 1.0);
        }

        if (s.b[2175] && (!s.b[2177])) {
            s.store_mul_ad_affine_product_rhs(2053, 2046, s.ad_value(2052), A::offset(A::mul_scaled_lhs(s.ad_value(1929), 0.25, s.ad_value(1929)), 1.0), 0.5, 0.0);
        }

        if s.b[2175] {
            s.store_add_scaled_inputs3_offset(1929, s.ad_value(2039), 0.5, s.ad_value(2053), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul_offset_lhs(A::sub(s.ad_value(2039), s.ad_value(2053)), (-2.0), A::offset(A::sub(s.ad_value(2039), s.ad_value(2053)), (-2.0))), 1.0)), 0.5, (2.0 * 0.5));
            s.store_mul_scaled_offset_ad_rhs(2054, 2021, 0.5, A::sqrt(A::offset(A::mul(A::div_from_scalar(4.0, s.ad_value(2021)), s.ad_value(1929)), 1.0)), (-1.0));
            s.store_div_add_scaled_inputs_rhs_indices(2055, 2054, 2054, 1.0, 2053, 1.0);
            s.store_add_scaled_product_indices(2045, 2044, 1.0, 2055, 2043, (-1.0));
        }

        s.store_offset_scaled(2056, 2020, 0.7071067811865475, 1.0);

        s.store_scale(2057, 2056, 1e-5);

        s.store_div_from_scalar(2058, 1.0, 2056);

        s.v[2165] = 0.0;

        s.v[2059] = 0.0;

        s.b[2178] = (s.v[2045] < 460.51701859880916);
        s.v[2178] = if s.b[2178] { 1.0 } else { 0.0 };

        if s.b[2178] {
            s.store_exp_neg_input(2060, 2045);
        }

        if (!s.b[2178]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2060, 1e-200, 2045, (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2045), (-460.51701859880916), A::scale_offset(s.ad_value(2045), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        s.b[2179] = (((s.v[2039]) as f64).abs() <= s.v[2057]);
        s.v[2179] = if s.b[2179] { 1.0 } else { 0.0 };

        if s.b[2179] {
            s.store_scaled_square(2145, 2058, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(2059, 2039, s.ad_value(2058), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2039), 1.0, s.ad_value(2060)), s.ad_value(2020), s.ad_value(2145)), 1.0));
        }

        s.b[2180] = (s.v[2039] < (-s.v[2057]));
        s.v[2180] = if s.b[2180] { 1.0 } else { 0.0 };

        if ((!s.b[2179]) && s.b[2180]) {
            s.store_neg(2147, 2039);
            s.store_scaled_mul(2148, 2147, 2058, 1.25);
            s.store_scaled_sub_ad(2149, A::offset(s.ad_value(2148), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(2148), (-6.0), A::offset(s.ad_value(2148), (-6.0))), 64.0)), 0.5);
            s.store_sub(2144, 2147, 2149);
            s.store_add_scaled_square_product_mixed_iia(2150, 2144, 1.0, 2021, A::offset(s.ad_value(2149), 1.0), 1.0);
            s.store_sub_scaled_inputs(2151, 2144, 2.0, 2021, 1.0);
            s.store_sub_ad_lhs(2152, A::ln(A::mul(s.ad_value(2150), s.ad_value(2037))), 2149);
            s.store_add(813, 2150, 2151);
            s.store_add_scaled_square_product_mixed_iia(812, 813, 1.0, 2152, A::sub_scaled_inputs(A::square(s.ad_value(2151)), 0.5, s.ad_value(2150), 1.0), 1.0);
            s.store_add_ad_rhs(2153, 2149, A::div_scaled_product3(s.ad_value(2150), s.ad_value(813), s.ad_value(2152), 1.0, A::add(s.ad_value(812), A::mul3(A::mul3(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2152), s.ad_value(2152)), s.ad_value(2151), A::sub_scaled_inputs(A::square(s.ad_value(2151)), 0.3333333333333333, s.ad_value(2150), 1.0))), 1.0));
        }

        s.b[2181] = (s.v[2153] < 230.25850929940458);
        s.v[2181] = if s.b[2181] { 1.0 } else { 0.0 };

        if (((!s.b[2179]) && s.b[2180]) && s.b[2181]) {
            s.store_exp(2154, 2153);
        }

        if (((!s.b[2179]) && s.b[2180]) && (!s.b[2181])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(2154, 2153, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2153), (-230.25850929940458), A::scale_offset(s.ad_value(2153), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if ((!s.b[2179]) && s.b[2180]) {
            s.store_div_from_scalar(2155, 1.0, 2154);
            s.store_div_from_scalar_offset_ad(2144, 1.0, A::square(s.ad_value(2153)), 2.0);
            s.store_mul_square_lhs(2156, 2153, 2144);
            s.store_mul3_affine_lhs(2157, 2153, 2144, 4.0, 0.0, 2144);
            s.store_mul_ad_product_lhs(2158, A::sub_scaled_inputs(s.ad_value(2144), 8.0, s.ad_value(2156), 12.0), s.ad_value(2144), 2144);
            s.store_sub(2144, 2147, 2153);
            s.store_mul(2145, 2060, 2155);
            s.store_add_scaled_product_right_ad(2159, 2144, 2.0, 2021, A::add_scaled_inputs3_offset(s.ad_value(2154), 1.0, s.ad_value(2145), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(2060), 1.0, s.ad_value(2157)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2160, 2144, 1.0, 2021, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2154), 1.0, s.ad_value(2153), (-1.0), s.ad_value(2145), 1.0, (-1.0)), 1.0, s.ad_value(2060), A::sub(A::offset(s.ad_value(2153), (-1.0)), s.ad_value(2156)), 1.0), (-1.0));
            s.store_sub_from_scalar_ad(2144, 2.0, A::mul(s.ad_value(2021), A::add_scaled_inputs_product(s.ad_value(2154), 1.0, s.ad_value(2145), 1.0, s.ad_value(2060), s.ad_value(2158), (-1.0))));
            s.store_add_scaled_square_product_indices(2144, 2159, 1.0, 2160, 2144, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(2059, 2153, -1.0, A::div(s.ad_value(2160), A::add(s.ad_value(2159), A::sqrt(s.ad_value(2144)))), 2.0);
        }

        if ((!s.b[2179]) && (!s.b[2180])) {
            s.store_div_from_scalar_offset_scaled_input(2161, 1.0, 2020, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(2162, A::mul_scaled_lhs(s.ad_value(2056), 1.25, s.ad_value(2161)), (-1.0), 2161);
            s.store_mul_ad_product_rhs(2163, 2039, s.ad_value(2058), A::offset(A::mul(s.ad_value(2162), s.ad_value(2039)), 1.0));
        }

        s.b[2182] = ((-s.v[2163]) > (-230.25850929940458));
        s.v[2182] = if s.b[2182] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
    ) {
        if (((!s.b[2179]) && (!s.b[2180])) && s.b[2182]) {
            s.store_exp_neg_input(2144, 2163);
        }

        if (((!s.b[2179]) && (!s.b[2180])) && (!s.b[2182])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(2144, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2163)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(2163)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if ((!s.b[2179]) && (!s.b[2180])) {
            s.store_sub_from_scalar(2164, 1.0, 2144);
            s.store_add_scaled_inputs_product_right_ad(2165, 2039, 1.0, 2021, 0.5, 2020, A::sqrt(A::add_scaled_inputs3(s.ad_value(2039), 1.0, s.ad_value(2021), 0.25, s.ad_value(2164), -1.0)), (-1.0));
            s.store_offset(2166, 2045, 3.0);
            s.store_sub_ad(2149, A::add_scaled_inputs3(s.ad_value(2165), 0.5, s.ad_value(2166), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2165), s.ad_value(2166)), A::sub(s.ad_value(2165), s.ad_value(2166))), 5.0)), (-0.5)), A::sub_scaled_inputs(s.ad_value(2166), 0.5, A::sqrt(A::offset(A::square(s.ad_value(2166)), 5.0)), 0.5));
            s.store_sub(2144, 2039, 2149);
            s.store_exp_neg_input(2145, 2149);
            s.store_div_from_scalar_offset_ad(2146, 1.0, A::square(s.ad_value(2149)), 2.0);
            s.store_mul_square_lhs(2156, 2149, 2146);
            s.store_mul3_affine_lhs(2157, 2149, 2146, 4.0, 0.0, 2146);
            s.store_mul_ad_product_lhs(2158, A::sub_scaled_inputs(s.ad_value(2146), 8.0, s.ad_value(2156), 12.0), s.ad_value(2146), 2146);
        }

        if ((!s.b[2179]) && (!s.b[2180])) {
            if (1e-40 > ((s.v[2144] * s.v[2144]) - (s.v[2021] * (((s.v[2145] + s.v[2149]) - 1.0) - (s.v[2060] * ((s.v[2149] + 1.0) + s.v[2156])))))) {
                s.store_scalar(2150, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2150, 2144, 1.0, 2021, A::add_scaled_product(A::offset(A::add(s.ad_value(2145), s.ad_value(2149)), (-1.0)), 1.0, s.ad_value(2060), A::add(A::offset(s.ad_value(2149), 1.0), s.ad_value(2156)), (-1.0)), (-1.0));
            }
        }

        if ((!s.b[2179]) && (!s.b[2180])) {
            s.store_sub_from_scalar_ad(2167, 1.0, A::mul_scaled_output(s.ad_value(2021), A::add_scaled_product(s.ad_value(2145), 1.0, s.ad_value(2060), s.ad_value(2158), (-1.0)), 0.5));
            s.store_add_scaled_product_right_ad(2151, 2144, 2.0, 2021, A::add_scaled_sub_value_product(1.0, s.ad_value(2145), 1.0, s.ad_value(2060), A::offset(s.ad_value(2157), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3(2152, s.ad_value(2045), 1.0, s.ad_value(2149), (-1.0), A::ln(A::div(s.ad_value(2150), s.ad_value(2021))), 1.0);
            s.store_add(813, 2150, 2151);
            s.store_add_scaled_square_product_mixed_iia(812, 813, 1.0, 2152, A::add_scaled_square_product(s.ad_value(2151), 0.5, s.ad_value(2150), s.ad_value(2167), (-1.0)), 1.0);
            s.store_add_ad_rhs(2168, 2149, A::div_scaled_product3(s.ad_value(2150), s.ad_value(813), s.ad_value(2152), 1.0, A::add(s.ad_value(812), A::mul3(A::mul3(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2152), s.ad_value(2152)), s.ad_value(2151), A::add_scaled_square_product(s.ad_value(2151), 0.3333333333333333, s.ad_value(2150), s.ad_value(2167), (-1.0)))), 1.0));
        }

        s.b[2183] = (s.v[2168] < 230.25850929940458);
        s.v[2183] = if s.b[2183] { 1.0 } else { 0.0 };

        if (((!s.b[2179]) && (!s.b[2180])) && s.b[2183]) {
            s.store_exp(2154, 2168);
            s.store_div_from_scalar(2155, 1.0, 2154);
            s.store_mul(2154, 2060, 2154);
        }

        s.b[2184] = (s.v[2168] > (s.v[2045] - 230.25850929940458));
        s.v[2184] = if s.b[2184] { 1.0 } else { 0.0 };

        if ((((!s.b[2179]) && (!s.b[2180])) && (!s.b[2183])) && s.b[2184]) {
            s.store_exp_sub(2154, 2168, 2045);
            s.store_div(2155, 2060, 2154);
        }

        if ((((!s.b[2179]) && (!s.b[2180])) && (!s.b[2183])) && (!s.b[2184])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(2154, 1e-100, A::sub(s.ad_value(2045), s.ad_value(2168)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2045), s.ad_value(2168)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2045), s.ad_value(2168)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2155, 1e-100, 2168, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2168), (-230.25850929940458), A::scale_offset(s.ad_value(2168), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if ((!s.b[2179]) && (!s.b[2180])) {
            s.store_div_from_scalar_offset_ad(2144, 1.0, A::square(s.ad_value(2168)), 2.0);
            s.store_mul_square_lhs(2156, 2168, 2144);
            s.store_mul3_affine_lhs(2157, 2168, 2144, 4.0, 0.0, 2144);
            s.store_mul_ad_product_lhs(2158, A::sub_scaled_inputs(s.ad_value(2144), 8.0, s.ad_value(2156), 12.0), s.ad_value(2144), 2144);
            s.store_sub(2144, 2039, 2168);
            s.store_add_scaled_product_right_ad(2159, 2144, 2.0, 2021, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2155)), 1.0, s.ad_value(2154), 1.0, s.ad_value(2060), A::offset(s.ad_value(2157), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2160, 2144, 1.0, 2021, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2155), 1.0, s.ad_value(2168), 1.0, s.ad_value(2154), 1.0, (-1.0)), 1.0, s.ad_value(2060), A::add(A::offset(s.ad_value(2168), 1.0), s.ad_value(2156)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_ad(2144, 2.0, A::mul(s.ad_value(2021), A::add_scaled_inputs_product(s.ad_value(2155), 1.0, s.ad_value(2154), 1.0, s.ad_value(2060), s.ad_value(2158), (-1.0))));
            s.store_add_scaled_square_product_indices(2144, 2159, 1.0, 2160, 2144, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(2059, 2168, 1.0, A::div(s.ad_value(2160), A::add(s.ad_value(2159), A::sqrt(s.ad_value(2144)))), 2.0);
        }

        s.v[2062] = 0.0;

        s.v[2063] = 0.0;

        s.v[2064] = 0.0;

        s.v[2065] = 0.0;

        s.v[2066] = 0.0;

        s.v[2067] = 0.0;

        s.v[2068] = 0.0;

        s.v[2069] = 1.0;

        s.v[2070] = 1.0;

        s.store_sub(2071, 2039, 2059);

        s.v[2072] = 0.0;

        s.store_mul(2073, 2035, 2071);

        s.v[2074] = 1.0;

        s.v[2075] = 1.0;

        s.v[2079] = 1.0;

        s.v[2080] = 1.0;

        s.v[2082] = 1.0;

        s.b[2185] = (s.v[2039] > 0.0);
        s.v[2185] = if s.b[2185] { 1.0 } else { 0.0 };

        if s.b[2185] {
            s.store_div_from_scalar_offset_ad(1929, 1.0, A::square(s.ad_value(2059)), 2.0);
            s.store_mul_square_lhs(2061, 2059, 1929);
            s.store_mul3_affine_lhs(2062, 2059, 1929, 4.0, 0.0, 1929);
            s.store_mul_ad_product_lhs(2063, A::sub_scaled_inputs(s.ad_value(1929), 8.0, s.ad_value(2061), 12.0), s.ad_value(1929), 1929);
            s.store_scalar(2064, 0.0);
        }

        s.b[2186] = (s.v[2059] < 230.25850929940458);
        s.v[2186] = if s.b[2186] { 1.0 } else { 0.0 };

        if (s.b[2185] && s.b[2186]) {
            s.store_exp(2064, 2059);
            s.store_div_from_scalar(2065, 1.0, 2064);
            s.store_mul(2064, 2060, 2064);
        }

        s.b[2187] = (s.v[2059] > (s.v[2045] - 230.25850929940458));
        s.v[2187] = if s.b[2187] { 1.0 } else { 0.0 };

        if ((s.b[2185] && (!s.b[2186])) && s.b[2187]) {
            s.store_exp_sub(2064, 2059, 2045);
            s.store_div(2065, 2060, 2064);
        }

        if ((s.b[2185] && (!s.b[2186])) && (!s.b[2187])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(2064, 1e-100, A::sub(s.ad_value(2045), s.ad_value(2059)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2045), s.ad_value(2059)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2045), s.ad_value(2059)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2065, 1e-100, 2059, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2059), (-230.25850929940458), A::scale_offset(s.ad_value(2059), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if s.b[2185] {
            s.store_add_scaled_product_right_ad(2066, 2064, 1.0, 2060, A::add(A::offset(s.ad_value(2059), 1.0), s.ad_value(2061)), (-1.0));
        }

        s.b[2188] = (s.v[2059] < 1e-5);
        s.v[2188] = if s.b[2188] { 1.0 } else { 0.0 };

        if (s.b[2185] && s.b[2188]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2067, 2059, 1.0, 2059, 1.0, 2059, 0.25, 0.3333333333333333, 0.5);
            s.store_mul3_ad_middle_scaled_output(2066, A::mul3(s.ad_value(2060), s.ad_value(2059), s.ad_value(2059)), 2059, A::scale_offset(s.ad_value(2059), 1.75, 1.0), 0.16666666666666666);
            s.store_sqrt_sub_from_scalar_ad(1929, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2059), 1.0, A::scale(s.ad_value(2059), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2068, 2059, 1929, 0.7071067811865475);
            s.store_offset_div_scaled_product(2069, s.ad_value(2020), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2059), 0.5)), 1.0, A::square(s.ad_value(2059)), 0.16666666666666666), 0.7071067811865475, s.ad_value(1929), 1.0, 1.0);
        }

        if (s.b[2185] && (!s.b[2188])) {
            s.store_add_ad_lhs(2067, A::offset(s.ad_value(2059), (-1.0)), 2065);
            s.store_sqrt(2068, 2067);
            s.store_offset_scaled_ad(2069, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2020), 1.0, s.ad_value(2065)), s.ad_value(2068)), 0.5, 1.0);
        }

        if s.b[2185] {
            s.store_div_scaled_offset_numerator(2070, A::mul_scaled_lhs(s.ad_value(709), 0.2, s.ad_value(2019)), 1.0, 1.0, A::offset(A::mul(s.ad_value(709), s.ad_value(2019)), 1.0), 1.0);
        }

        s.b[2189] = (s.v[2066] > 1e-100);
        s.v[2189] = if s.b[2189] { 1.0 } else { 0.0 };

        if (s.b[2185] && s.b[2189]) {
            s.store_mul_sqrt_ad_rhs(2071, 2020, A::add(s.ad_value(2067), s.ad_value(2066)));
            s.store_div_scaled_product3_mixed_iiia(2072, 2021, 2066, 2035, 1.0, A::add_scaled_product(s.ad_value(2071), 1.0, s.ad_value(2020), s.ad_value(2068), 1.0), 1.0);
            s.store_mul3_lhs(2073, 2068, 2020, 2035);
        }

        s.b[2190] = (s.v[215] < 0.0);
        s.v[2190] = if s.b[2190] { 1.0 } else { 0.0 };

        if ((s.b[2185] && s.b[2189]) && s.b[2190]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2074, 1.0, 1.0, A::mul(s.ad_value(215), s.ad_value(2019)));
        }

        if ((s.b[2185] && s.b[2189]) && (!s.b[2190])) {
            s.store_offset_mul(2074, 215, 2019, 1.0);
        }

        s.b[2191] = (s.v[216] < 0.0);
        s.v[2191] = if s.b[2191] { 1.0 } else { 0.0 };

        if ((s.b[2185] && s.b[2189]) && s.b[2191]) {
            s.store_sub_from_scalar_ad(2075, 1.0, A::mul(s.ad_value(216), s.ad_value(2072)));
        }

        if ((s.b[2185] && s.b[2189]) && (!s.b[2191])) {
            s.store_div_from_scalar_offset_ad(2075, 1.0, A::mul(s.ad_value(216), s.ad_value(2072)), 1.0);
        }

        if (s.b[2185] && s.b[2189]) {
            s.store_mul_product3_rhs(2076, 2072, s.ad_value(746), s.ad_value(2074), s.ad_value(2075), 1.0);
            s.store_mul_add_scaled_product_rhs(2077, 763, s.ad_value(2073), 1.0, s.ad_value(764), s.ad_value(2072), 1.0);
            s.store_ln_ad(1930, A::div_scaled_value_offset_denominator(s.ad_value(2067), 1.0, A::add(s.ad_value(2067), s.ad_value(2066)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2078, A::pow(A::mul(s.ad_value(2077), s.ad_value(705)), s.ad_value(706)), 1.0, 707, A::exp(A::mul_scaled_lhs(s.ad_value(708), 0.5, s.ad_value(1930))), 1.0);
            s.store_mul_add_ad_lhs(2079, A::offset(s.ad_value(2078), 1.0), s.ad_value(2076), 2070);
        }

        s.b[2192] = (s.v[219] < 0.0);
        s.v[2192] = if s.b[2192] { 1.0 } else { 0.0 };

        if ((s.b[2185] && s.b[2189]) && s.b[2192]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2080, 1.0, 1.0, A::mul(s.ad_value(219), s.ad_value(2019)));
        }

        if ((s.b[2185] && s.b[2189]) && (!s.b[2192])) {
            s.store_offset_mul(2080, 219, 2019, 1.0);
        }

        if (s.b[2185] && s.b[2189]) {
            s.store_mul(1931, 2072, 2080);
            s.store_div_add_scaled_inputs_rhs_indices(2081, 1931, 221, 1.0, 1931, 1.0);
        }

        s.b[2193] = (s.v[220] < 0.0);
        s.v[2193] = if s.b[2193] { 1.0 } else { 0.0 };

        if ((s.b[2185] && s.b[2189]) && s.b[2193]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2082, 1.0, 1.0, A::mul(s.ad_value(220), s.ad_value(2081)));
        }

        if ((s.b[2185] && s.b[2189]) && (!s.b[2193])) {
            s.store_offset_mul(2082, 220, 2081, 1.0);
        }

        s.copy_ad(1806, 2017);

        s.copy_ad(1807, 2019);

        s.copy_ad(1808, 2035);

        s.copy_ad(1809, 2036);

        s.copy_ad(1810, 2020);

        s.copy_ad(1811, 2021);

        s.copy_ad(1812, 2037);

        s.copy_ad(1813, 2039);

        s.copy_ad(1814, 2044);

        s.copy_ad(1815, 2045);

        s.copy_ad(1816, 2056);

        s.copy_ad(1817, 2057);

        s.copy_ad(1818, 2058);

        s.copy_ad(1819, 2165);

        s.copy_ad(1820, 2060);

        s.copy_ad(1821, 2059);

        s.copy_ad(1822, 2062);

        s.copy_ad(1823, 2063);

        s.copy_ad(1824, 2064);

        s.copy_ad(1825, 2065);

        s.copy_ad(1826, 2067);

        s.copy_ad(1827, 2066);

        s.copy_ad(1828, 2068);

        s.copy_ad(1829, 2069);

        s.copy_ad(1830, 2070);

        s.copy_ad(1831, 2071);

        s.copy_ad(1832, 2072);

        s.copy_ad(1833, 2073);

        s.copy_ad(1834, 2074);

        s.copy_ad(1835, 2075);

        s.copy_ad(1836, 2079);

        s.copy_ad(1837, 2080);

        s.copy_ad(1838, 2082);

        s.v[2084] = 0.0;

        s.store_scale(2083, 2035, 4.60517018598809);

        s.copy_ad(2100, 2083);

        s.copy_ad(2101, 815);

        s.store_mul(2102, 815, 2036);

        s.copy_ad(2106, 2059);

        s.v[2107] = 0.0;

        s.v[2110] = 0.0;

        s.copy_ad(2112, 2065);

        s.copy_ad(2113, 2067);

        s.copy_ad(2115, 2066);

        s.copy_ad(2116, 2073);

        s.copy_ad(2117, 2059);

        s.copy_ad(2118, 2065);

        s.copy_ad(2120, 2066);

        s.copy_ad(2121, 2067);

        s.store_sub(2122, 2039, 2059);

        s.v[2123] = 1.0;

        s.v[2125] = 1.0;

        s.v[2124] = 0.0;

        s.copy_ad(2134, 2072);

        s.store_mul(2138, 2122, 2035);

        s.v[2135] = 0.0;

        s.copy_ad(2136, 2073);

        s.v[2141] = 0.0;

        s.v[2140] = 1.0;

        s.copy_ad(2143, 2015);

        s.copy_ad(2142, 2138);

        s.b[2194] = (s.v[2039] > 0.0);
        s.v[2194] = if s.b[2194] { 1.0 } else { 0.0 };

        s.b[2195] = (s.v[2066] > 1e-100);
        s.v[2195] = if s.b[2195] { 1.0 } else { 0.0 };

        if (s.b[2194] && s.b[2195]) {
            s.store_mul(2143, 2015, 2082);
        }

    }

    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[2194] && s.b[2195]) {
            s.store_div(2084, 2143, 2079);
            s.store_add_scaled_inputs(2085, 2071, 1.0, 2021, 0.5);
            s.store_div_scaled_product_by_product(1929, s.ad_value(2021), s.ad_value(2064), 1.0, s.ad_value(2085), s.ad_value(2085), 1.0);
        }

        s.b[2196] = (s.v[1929] > 0.0001);
        s.v[2196] = if s.b[2196] { 1.0 } else { 0.0 };

        if ((s.b[2194] && s.b[2195]) && s.b[2196]) {
            s.store_sub_from_scalar(1930, 1.0, 1929);
        }

        s.b[2197] = (s.v[1930] < 1e-10);
        s.v[2197] = if s.b[2197] { 1.0 } else { 0.0 };

        if (((s.b[2194] && s.b[2195]) && s.b[2196]) && s.b[2197]) {
            s.store_scalar(1931, 1.0);
        }

        if (((s.b[2194] && s.b[2195]) && s.b[2196]) && (!s.b[2197])) {
            s.store_sub_from_scalar_ad(1931, 1.0, A::sqrt(s.ad_value(1930)));
        }

        if ((s.b[2194] && s.b[2195]) && (!s.b[2196])) {
            s.store_scale(1931, 1929, 0.5);
        }

        if (s.b[2194] && s.b[2195]) {
            s.store_mul(2086, 1931, 2085);
        }

        s.b[2198] = ((s.v[707] > 0.0) && (s.v[708] > 0.0));
        s.v[2198] = if s.b[2198] { 1.0 } else { 0.0 };

        if ((s.b[2194] && s.b[2195]) && s.b[2198]) {
            s.store_scaled_mul(2087, 2035, 2086, 0.475);
            s.store_add_scaled_product_indices(1929, 2072, 1.0, 2069, 2087, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(2088, 1929, 1929, 1e-12, 0.5);
            s.store_add_scaled_value_products(2089, s.ad_value(2072), (-1.0), s.ad_value(2035), s.ad_value(2071), 1.0, A::offset(s.ad_value(2069), (-1.0)), s.ad_value(2087), 1.0);
            s.store_offset_div_scaled_product(2090, s.ad_value(2021), s.ad_value(2035), 0.5, s.ad_value(2089), 1.0, 1.0);
            s.store_add_scaled_product_indices(1929, 2089, 1.0, 764, 2088, 1.0);
            s.store_pow_ad(2091, A::mul3(s.ad_value(763), s.ad_value(1929), s.ad_value(705)), s.ad_value(706));
            s.store_mul_ad_lhs(1930, A::div_scaled_product_offset_rhs(s.ad_value(706), A::mul_sub_from_scalar_rhs(s.ad_value(2090), 1.0, s.ad_value(764)), (-1.0), 1.0, s.ad_value(1929), 1.0), 2091);
            s.store_div(1929, 2088, 2089);
            s.store_mul_pow_ad_rhs(2092, 707, A::offset(s.ad_value(1929), 1.0), A::neg(s.ad_value(708)));
            s.store_mul_div_scaled_product_rhs(1931, 2092, s.ad_value(708), A::add(A::offset(s.ad_value(2090), (-1.0)), A::div_scalar_offset_denominator(1.0, s.ad_value(1929), 1.0, 1.0)), 1.0, s.ad_value(2089), 1.0);
            s.store_mul_product3_rhs(2093, 2088, s.ad_value(746), s.ad_value(2074), s.ad_value(2075), 1.0);
            s.store_offset_ad(1929, A::div_scaled_add_product(s.ad_value(1930), 1.0, A::mul3(s.ad_value(746), s.ad_value(2074), s.ad_value(2075)), s.ad_value(2090), (-1.0), s.ad_value(1931), 1.0), 1.0);
        }

        s.b[2199] = (s.v[1929] < 230.25850929940458);
        s.v[2199] = if s.b[2199] { 1.0 } else { 0.0 };

        if (((s.b[2194] && s.b[2195]) && s.b[2198]) && s.b[2199]) {
            s.store_scaled_ln_one_plus_exp_scaled_input(1930, 1929, 2.0, 0.5);
        }

        if (((s.b[2194] && s.b[2195]) && s.b[2198]) && (!s.b[2199])) {
            s.copy_ad(1930, 1929);
        }

        if ((s.b[2194] && s.b[2195]) && s.b[2198]) {
            s.store_div_scaled_product3_mixed_iiia(2094, 2087, 1931, 1930, -1.0, A::add_scaled_inputs3_offset(s.ad_value(2091), 1.0, s.ad_value(2092), 1.0, s.ad_value(2093), 1.0, 1.0), 1.0);
            s.store_mul_offset_ad_rhs(2095, 2086, A::div_scaled_value_offset_denominator(s.ad_value(2094), 1.0, A::sqrt(A::offset(A::square(s.ad_value(2094)), 1.0)), 1.0, 1.0), 1.0);
        }

        if ((s.b[2194] && s.b[2195]) && (!s.b[2198])) {
            s.copy_ad(2095, 2086);
        }

        if (s.b[2194] && s.b[2195]) {
            s.store_mul3_affine_lhs(2096, 2035, 2084, 0.7071067811865475, 0.0, 2095);
        }

        s.b[2200] = (s.v[0] == (-1.0));
        s.v[2200] = if s.b[2200] { 1.0 } else { 0.0 };

        if ((s.b[2194] && s.b[2195]) && s.b[2200]) {
            s.store_div_ad_rhs(2096, 2096, A::sqrt(A::offset(s.ad_value(2096), 1.0)));
        }

        if (s.b[2194] && s.b[2195]) {
            s.store_div_from_scalar_offset_ad(2097, 2.0, A::sqrt(A::scale_offset(s.ad_value(2096), 4.0, 1.0)), 1.0);
            s.store_mul(1929, 2097, 2096);
            s.store_mul_ad_product_rhs(2098, 2095, s.ad_value(2097), A::offset(A::div(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1929), 1.0, A::mul(s.ad_value(1929), s.ad_value(2097)), 0.86), A::offset(A::mul3_scaled_output(s.ad_value(1929), s.ad_value(1929), s.ad_value(2097), 4.0), 1.0)), 1.0));
            s.store_scale(2099, 2098, 0.99);
            s.store_div_scaled_product3_mixed_iaii(1929, 2099, A::sub_scaled_inputs(s.ad_value(2099), 1.0, s.ad_value(2085), 2.0), 2037, 1.0, 2066, 1.0);
        }

        if (s.b[2194] && s.b[2195]) {
            s.store_mul_sub_ad_rhs(2100, 2035, s.ad_value(2099), A::ln(A::offset({
                if (s.v[1929] > (-0.99)) {
                    s.ad_value(1929)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0)));
        }

        if (s.b[2194] && (!s.b[2195])) {
            s.copy_ad(2100, 2083);
        }

        if s.b[2194] {
            s.store_offset(1929, 2016, 1.0);
            s.store_div_scaled_product_left_ad(1930, A::sqrt(s.ad_value(1929)), 815, 1.0, 2100, 1.0);
            s.store_add_ad_lhs(1931, A::square(s.ad_value(1930)), 1929);
            s.store_scale(1929, 1930, 2.0);
            s.store_div_scaled_product_add_scaled_denominator(2101, 2100, 1929, 1.0, A::sqrt(A::sub(s.ad_value(1931), s.ad_value(1929))), 1.0, A::sqrt(A::add(s.ad_value(1931), s.ad_value(1929))), 1.0, 1.0);
            s.store_mul(2102, 2101, 2036);
            s.store_add(2103, 2045, 2102);
        }

        s.b[2201] = (s.v[2102] < 460.51701859880916);
        s.v[2201] = if s.b[2201] { 1.0 } else { 0.0 };

        if (s.b[2194] && s.b[2201]) {
            s.store_exp_neg_input(2104, 2102);
        }

        if (s.b[2194] && (!s.b[2201])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2104, 1e-200, 2102, (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2102), (-460.51701859880916), A::scale_offset(s.ad_value(2102), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if s.b[2194] {
            s.store_mul(2105, 2060, 2104);
        }

        s.b[2202] = (((s.v[2039]) as f64).abs() <= s.v[2057]);
        s.v[2202] = if s.b[2202] { 1.0 } else { 0.0 };

        if (s.b[2194] && s.b[2202]) {
            s.store_scaled_square(2145, 2058, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(2106, 2039, s.ad_value(2058), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2039), 1.0, s.ad_value(2105)), s.ad_value(2020), s.ad_value(2145)), 1.0));
        }

        if (s.b[2194] && (!s.b[2202])) {
            s.store_offset(2166, 2103, 3.0);
            s.store_sub_ad(2149, A::add_scaled_inputs3(s.ad_value(2165), 0.5, s.ad_value(2166), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2165), s.ad_value(2166)), A::sub(s.ad_value(2165), s.ad_value(2166))), 5.0)), (-0.5)), A::sub_scaled_inputs(s.ad_value(2166), 0.5, A::sqrt(A::offset(A::square(s.ad_value(2166)), 5.0)), 0.5));
            s.store_sub(2144, 2039, 2149);
            s.store_exp_neg_input(2145, 2149);
            s.store_div_from_scalar_offset_ad(2146, 1.0, A::square(s.ad_value(2149)), 2.0);
            s.store_mul_square_lhs(2156, 2149, 2146);
            s.store_mul3_affine_lhs(2157, 2149, 2146, 4.0, 0.0, 2146);
            s.store_mul_ad_product_lhs(2158, A::sub_scaled_inputs(s.ad_value(2146), 8.0, s.ad_value(2156), 12.0), s.ad_value(2146), 2146);
        }

        if (s.b[2194] && (!s.b[2202])) {
            if (1e-40 > ((s.v[2144] * s.v[2144]) - (s.v[2021] * (((s.v[2145] + s.v[2149]) - 1.0) - (s.v[2105] * ((s.v[2149] + 1.0) + s.v[2156])))))) {
                s.store_scalar(2150, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2150, 2144, 1.0, 2021, A::add_scaled_product(A::offset(A::add(s.ad_value(2145), s.ad_value(2149)), (-1.0)), 1.0, s.ad_value(2105), A::add(A::offset(s.ad_value(2149), 1.0), s.ad_value(2156)), (-1.0)), (-1.0));
            }
        }

        if (s.b[2194] && (!s.b[2202])) {
            s.store_sub_from_scalar_ad(2167, 1.0, A::mul_scaled_output(s.ad_value(2021), A::add_scaled_product(s.ad_value(2145), 1.0, s.ad_value(2105), s.ad_value(2158), (-1.0)), 0.5));
            s.store_add_scaled_product_right_ad(2151, 2144, 2.0, 2021, A::add_scaled_sub_value_product(1.0, s.ad_value(2145), 1.0, s.ad_value(2105), A::offset(s.ad_value(2157), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3(2152, s.ad_value(2103), 1.0, s.ad_value(2149), (-1.0), A::ln(A::div(s.ad_value(2150), s.ad_value(2021))), 1.0);
            s.store_add(813, 2150, 2151);
            s.store_add_scaled_square_product_mixed_iia(812, 813, 1.0, 2152, A::add_scaled_square_product(s.ad_value(2151), 0.5, s.ad_value(2150), s.ad_value(2167), (-1.0)), 1.0);
            s.store_add_ad_rhs(2168, 2149, A::div_scaled_product3(s.ad_value(2150), s.ad_value(813), s.ad_value(2152), 1.0, A::add(s.ad_value(812), A::mul3(A::mul3(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2152), s.ad_value(2152)), s.ad_value(2151), A::add_scaled_square_product(s.ad_value(2151), 0.3333333333333333, s.ad_value(2150), s.ad_value(2167), (-1.0)))), 1.0));
        }

        s.b[2203] = (s.v[2168] < 230.25850929940458);
        s.v[2203] = if s.b[2203] { 1.0 } else { 0.0 };

        if ((s.b[2194] && (!s.b[2202])) && s.b[2203]) {
            s.store_exp(2154, 2168);
            s.store_div_from_scalar(2155, 1.0, 2154);
            s.store_mul(2154, 2105, 2154);
        }

        s.b[2204] = (s.v[2168] > (s.v[2103] - 230.25850929940458));
        s.v[2204] = if s.b[2204] { 1.0 } else { 0.0 };

        if (((s.b[2194] && (!s.b[2202])) && (!s.b[2203])) && s.b[2204]) {
            s.store_exp_sub(2154, 2168, 2103);
            s.store_div(2155, 2105, 2154);
        }

        if (((s.b[2194] && (!s.b[2202])) && (!s.b[2203])) && (!s.b[2204])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(2154, 1e-100, A::sub(s.ad_value(2103), s.ad_value(2168)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2103), s.ad_value(2168)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2103), s.ad_value(2168)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2155, 1e-100, 2168, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2168), (-230.25850929940458), A::scale_offset(s.ad_value(2168), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if (s.b[2194] && (!s.b[2202])) {
            s.store_div_from_scalar_offset_ad(2144, 1.0, A::square(s.ad_value(2168)), 2.0);
            s.store_mul_square_lhs(2156, 2168, 2144);
            s.store_mul3_affine_lhs(2157, 2168, 2144, 4.0, 0.0, 2144);
            s.store_mul_ad_product_lhs(2158, A::sub_scaled_inputs(s.ad_value(2144), 8.0, s.ad_value(2156), 12.0), s.ad_value(2144), 2144);
            s.store_sub(2144, 2039, 2168);
            s.store_add_scaled_product_right_ad(2159, 2144, 2.0, 2021, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2155)), 1.0, s.ad_value(2154), 1.0, s.ad_value(2105), A::offset(s.ad_value(2157), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2160, 2144, 1.0, 2021, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2155), 1.0, s.ad_value(2168), 1.0, s.ad_value(2154), 1.0, (-1.0)), 1.0, s.ad_value(2105), A::add(A::offset(s.ad_value(2168), 1.0), s.ad_value(2156)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_ad(2144, 2.0, A::mul(s.ad_value(2021), A::add_scaled_inputs_product(s.ad_value(2155), 1.0, s.ad_value(2154), 1.0, s.ad_value(2105), s.ad_value(2158), (-1.0))));
            s.store_add_scaled_square_product_indices(2144, 2159, 1.0, 2160, 2144, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(2106, 2168, 1.0, A::div(s.ad_value(2160), A::add(s.ad_value(2159), A::sqrt(s.ad_value(2144)))), 2.0);
        }

        if s.b[2194] {
            s.store_sub(2107, 2106, 2059);
        }

        s.b[2205] = (s.v[2107] < 1e-10);
        s.v[2205] = if s.b[2205] { 1.0 } else { 0.0 };

        if (s.b[2194] && s.b[2205]) {
            s.store_add_scaled_inputs_product_right_ad(2108, 2039, 2.0, 2059, (-2.0), 2021, A::add_scaled_offset_product_rhs(A::add_scaled_sub_value_product(1.0, s.ad_value(2065), 1.0, s.ad_value(2064), s.ad_value(2104), 1.0), 1.0, s.ad_value(2105), s.ad_value(2062), 1.0, (-1.0)), 1.0);
            s.store_mul_ad_lhs(2109, A::mul_sub_from_scalar_rhs(s.ad_value(2021), 1.0, s.ad_value(2104)), 2066);
            s.store_sub_from_scalar_ad(1929, 2.0, A::mul(s.ad_value(2021), A::add_scaled_value_products(s.ad_value(2065), 1.0, s.ad_value(2064), s.ad_value(2104), 1.0, s.ad_value(2105), s.ad_value(2063), (-1.0))));
            s.store_add_scaled_square_product_indices(1929, 2108, 1.0, 1929, 2109, (-2.0));
            s.store_scaled_div_ad_rhs(2107, 2109, A::add(s.ad_value(2108), A::sqrt(s.ad_value(1929))), 2.0);
            s.store_add(2106, 2059, 2107);
        }

        if s.b[2194] {
            s.store_mul(2110, 2107, 2035);
            s.store_div_scaled_product_offset_denominator(2111, s.ad_value(2106), s.ad_value(2106), 1.0, A::square(s.ad_value(2106)), 2.0, 1.0);
        }

        s.b[2206] = (s.v[2106] < 230.25850929940458);
        s.v[2206] = if s.b[2206] { 1.0 } else { 0.0 };

        if (s.b[2194] && s.b[2206]) {
            s.store_exp_neg_input(2112, 2106);
        }

        s.b[2207] = (s.v[2106] < 1e-5);
        s.v[2207] = if s.b[2207] { 1.0 } else { 0.0 };

        if ((s.b[2194] && s.b[2206]) && s.b[2207]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2113, 2106, 1.0, 2106, 1.0, 2106, 0.25, 0.3333333333333333, 0.5);
            s.store_sqrt_sub_from_scalar_ad(1929, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2106), 1.0, A::scale(s.ad_value(2106), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2114, 2106, 1929, 0.7071067811865475);
            s.store_mul3_ad_middle(2115, A::mul3_scaled_output(s.ad_value(2105), s.ad_value(2106), s.ad_value(2106), 0.16666666666666666), 2106, A::scale_offset(s.ad_value(2106), 1.75, 1.0));
        }

        if ((s.b[2194] && s.b[2206]) && (!s.b[2207])) {
            s.store_add_ad_lhs(2113, A::offset(s.ad_value(2106), (-1.0)), 2112);
            s.store_sqrt(2114, 2113);
            s.store_mul_add_scaled_inputs3_offset_rhs(2115, 2105, A::div_from_scalar(1.0, s.ad_value(2112)), 1.0, s.ad_value(2106), (-1.0), s.ad_value(2111), -1.0, (-1.0));
        }

        s.b[2208] = (s.v[2106] > (s.v[2103] - 230.25850929940458));
        s.v[2208] = if s.b[2208] { 1.0 } else { 0.0 };

        if ((s.b[2194] && (!s.b[2206])) && s.b[2208]) {
            s.store_exp_sub(1929, 2106, 2103);
            s.store_div(2112, 2105, 1929);
            s.store_add_scaled_product_right_ad(2115, 1929, 1.0, 2105, A::add(A::offset(s.ad_value(2106), 1.0), s.ad_value(2111)), (-1.0));
        }

        if ((s.b[2194] && (!s.b[2206])) && (!s.b[2208])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2112, 1e-100, 2106, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2106), (-230.25850929940458), A::scale_offset(s.ad_value(2106), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(1929, 1e-100, A::sub(s.ad_value(2103), s.ad_value(2106)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2103), s.ad_value(2106)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2103), s.ad_value(2106)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_add_scaled_product_right_ad(2115, 1929, 1.0, 2105, A::add(A::offset(s.ad_value(2106), 1.0), s.ad_value(2111)), (-1.0));
        }

        if (s.b[2194] && (!s.b[2206])) {
            s.store_add_ad_lhs(2113, A::offset(s.ad_value(2106), (-1.0)), 2112);
            s.store_sqrt(2114, 2113);
        }

        if s.b[2194] {
            s.store_mul3_lhs(2116, 2114, 2020, 2035);
            s.store_scaled_add(2117, 2059, 2106, 0.5);
            s.store_scalar(2118, 0.0);
            s.store_mul(1929, 2112, 2065);
        }

        s.b[2209] = (s.v[1929] > 0.0);
        s.v[2209] = if s.b[2209] { 1.0 } else { 0.0 };

        if (s.b[2194] && s.b[2209]) {
            s.store_sqrt(2118, 1929);
        }

        if s.b[2194] {
            s.store_scaled_add(2119, 2066, 2115, 0.5);
            s.store_add_scaled_product_mixed_iaa(2120, 2119, 1.0, A::square(s.ad_value(2107)), A::sub_scaled_inputs(s.ad_value(2118), 1.0, s.ad_value(2037), 2.0), 0.125);
        }

        s.b[2210] = (s.v[2117] < 1e-5);
        s.v[2210] = if s.b[2210] { 1.0 } else { 0.0 };

        if (s.b[2194] && s.b[2210]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2121, 2117, 1.0, 2117, 1.0, 2117, 0.25, 0.3333333333333333, 0.5);
            s.store_mul_sqrt_ad_rhs(2122, 2020, A::add(s.ad_value(2120), s.ad_value(2121)));
        }

    }

    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2211] = (s.v[719] > 0.0);
        s.v[2211] = if s.b[2211] { 1.0 } else { 0.0 };

        if ((s.b[2194] && s.b[2210]) && s.b[2211]) {
            s.store_div_from_scalar_sqrt_ad(2123, 1.0, A::offset(A::mul(s.ad_value(719), s.ad_value(2122)), 1.0));
        }

        if (s.b[2194] && s.b[2210]) {
            s.store_sqrt_sub_from_scalar_ad(1929, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2117), 1.0, A::scale(s.ad_value(2117), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2124, 2117, 1929, 0.7071067811865475);
            s.store_add_ad_rhs(2125, 2123, A::div_scaled_product(s.ad_value(2020), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2117), 0.5)), 1.0, A::square(s.ad_value(2117)), 0.16666666666666666), 0.7071067811865475, s.ad_value(1929), 1.0));
        }

        if (s.b[2194] && (!s.b[2210])) {
            s.store_add_ad_lhs(2121, A::offset(s.ad_value(2117), (-1.0)), 2118);
            s.store_mul_sqrt_ad_rhs(2122, 2020, A::add(s.ad_value(2120), s.ad_value(2121)));
        }

        s.b[2212] = (s.v[719] > 0.0);
        s.v[2212] = if s.b[2212] { 1.0 } else { 0.0 };

        if ((s.b[2194] && (!s.b[2210])) && s.b[2212]) {
            s.store_add_scaled_sub_value_product_indices(2126, 1.0, 2118, 1.0, 2122, 2037, 2.0);
            s.store_div_from_scalar_sqrt_ad(2123, 1.0, A::offset(A::mul(s.ad_value(719), s.ad_value(2122)), 1.0));
            s.store_div_scaled_value_offset_denominator(1929, s.ad_value(2123), 1.0, s.ad_value(2123), 1.0, 1.0);
            s.store_mul_product3_rhs(2127, 719, A::square(s.ad_value(1929)), s.ad_value(2021), s.ad_value(2120), 1.0);
            s.store_add_scaled_inputs_product_right_ad(2128, 2122, 2.0, 2127, (-2.0), 2021, A::add(A::sub_from_scalar(1.0, s.ad_value(2118)), s.ad_value(2120)), 1.0);
            s.store_mul_sub_scaled_inputs_rhs(2129, 2127, s.ad_value(2127), 1.0, s.ad_value(2122), 2.0);
            s.store_sub_from_scalar_ad(2130, 1.0, A::mul_scaled_output(s.ad_value(2021), A::add(s.ad_value(2118), s.ad_value(2120)), 0.5));
            s.store_div_scaled_product_denominator_ad(2131, 2129, 2128, 1.0, A::add_scaled_square_product(s.ad_value(2128), 1.0, s.ad_value(2130), s.ad_value(2129), (-1.0)), 1.0);
            s.store_add(2117, 2117, 2131);
            s.store_exp(2132, 2131);
            s.store_div(2118, 2118, 2132);
            s.store_mul(2120, 2120, 2132);
            s.store_add_ad_lhs(2121, A::offset(s.ad_value(2117), (-1.0)), 2118);
            s.store_mul_sqrt_ad_rhs(2122, 2020, A::add(s.ad_value(2120), s.ad_value(2121)));
            s.store_add_ad(2133, A::sub_from_scalar(1.0, s.ad_value(2118)), A::mul3_scaled_output(s.ad_value(2122), s.ad_value(2123), s.ad_value(2037), 2.0));
            s.store_div_scaled_product3_mixed_iiaa(2107, 2107, 2132, A::add(s.ad_value(2126), s.ad_value(2119)), 1.0, A::add_scaled_product(s.ad_value(2133), 1.0, s.ad_value(2132), s.ad_value(2119), 1.0), 1.0);
            s.store_mul(2110, 2107, 2035);
        }

        if (s.b[2194] && (!s.b[2210])) {
            s.store_sqrt(2124, 2121);
            s.store_add_scaled_inputs_ad_rhs(2125, 2123, 1.0, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2020), 1.0, s.ad_value(2118)), s.ad_value(2124)), 0.5);
        }

        if s.b[2194] {
            s.store_mul_div_scaled_product_rhs(2134, 2035, s.ad_value(2021), s.ad_value(2120), 1.0, A::add_scaled_product(s.ad_value(2122), 1.0, s.ad_value(2020), s.ad_value(2124), 1.0), 1.0);
            s.store_add_scaled_product_indices(2135, 2134, 1.0, 2035, 2125, 1.0);
            s.store_mul3_lhs(2136, 2124, 2020, 2035);
        }

        s.b[2213] = (s.v[216] < 0.0);
        s.v[2213] = if s.b[2213] { 1.0 } else { 0.0 };

        if (s.b[2194] && s.b[2213]) {
            s.store_sub_from_scalar_ad(2075, 1.0, A::mul(s.ad_value(216), s.ad_value(2134)));
        }

        if (s.b[2194] && (!s.b[2213])) {
            s.store_div_from_scalar_offset_ad(2075, 1.0, A::mul(s.ad_value(216), s.ad_value(2134)), 1.0);
        }

        if s.b[2194] {
            s.store_mul_product3_rhs(2076, 2134, s.ad_value(746), s.ad_value(2074), s.ad_value(2075), 1.0);
            s.store_add_scaled_product_indices(2137, 2136, 1.0, 764, 2134, 1.0);
            s.store_add_scaled_product_indices(2138, 2136, 1.0, 765, 2134, 1.0);
            s.store_mul(2139, 763, 2137);
            s.store_ln_ad(1930, A::div_scaled_value_offset_denominator(s.ad_value(2121), 1.0, A::add(s.ad_value(2121), s.ad_value(2120)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2078, A::pow(A::mul(s.ad_value(2139), s.ad_value(705)), s.ad_value(706)), 1.0, 707, A::exp(A::mul_scaled_lhs(s.ad_value(708), 0.5, s.ad_value(1930))), 1.0);
            s.store_mul_add_ad_lhs(2140, A::offset(s.ad_value(2078), 1.0), s.ad_value(2076), 2070);
            s.store_ln_ad(2141, A::div_scaled_offset_numerator(A::mul(A::sub(s.ad_value(815), s.ad_value(2110)), s.ad_value(768)), 1.0, 1.0, A::offset(A::mul(A::sub(s.ad_value(2101), s.ad_value(2110)), s.ad_value(768)), 1.0), 1.0));
            s.store_mul(1931, 2134, 2080);
            s.store_div_add_scaled_inputs_rhs_indices(2081, 1931, 221, 1.0, 1931, 1.0);
        }

        s.b[2214] = (s.v[220] < 0.0);
        s.v[2214] = if s.b[2214] { 1.0 } else { 0.0 };

        if (s.b[2194] && s.b[2214]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2082, 1.0, 1.0, A::mul(s.ad_value(220), s.ad_value(2081)));
        }

        if (s.b[2194] && (!s.b[2214])) {
            s.store_offset_mul(2082, 220, 2081, 1.0);
        }

        if s.b[2194] {
            s.store_mul(2143, 2015, 2082);
            s.store_mul(2142, 2122, 2035);
        }

        s.copy_ad(1839, 2083);

        s.copy_ad(1841, 2101);

        s.copy_ad(1842, 2102);

        s.copy_ad(1843, 2107);

        s.copy_ad(1844, 2110);

        s.copy_ad(1846, 2117);

        s.copy_ad(1845, 2116);

        s.copy_ad(1847, 2123);

        s.copy_ad(1848, 2125);

        s.copy_ad(1849, 2134);

        s.copy_ad(1850, 2135);

        s.copy_ad(1851, 2136);

        s.copy_ad(1852, 2138);

        s.copy_ad(1853, 2140);

        s.copy_ad(1855, 2141);

        s.copy_ad(1854, 2143);

        s.copy_ad(1856, 2142);

        s.v[1857] = 1.0;

        s.v[1858] = 1.0;

        s.v[1860] = 1.0;

        s.v[1861] = 1.0;

        s.v[827] = 0.0;

        s.b[2215] = (s.v[1813] > 0.0);
        s.v[2215] = if s.b[2215] { 1.0 } else { 0.0 };

        if s.b[2215] {
            s.store_ln_ad(1939, A::offset(A::mul(s.ad_value(819), s.ad_value(768)), 1.0));
            s.store_div_scaled_product_indices(1929, 1808, 1848, 1.0, 1850, 1.0);
            s.store_add_scaled_product_mixed_aai(1938, A::mul3(A::mul3(s.ad_value(225), s.ad_value(1851), s.ad_value(1929)), s.ad_value(1929), s.ad_value(1939)), 1.0, A::div_scaled_product(A::add(s.ad_value(223), A::div(s.ad_value(224), s.ad_value(1850))), s.ad_value(1849), 1.0, s.ad_value(1850), 1.0), 1855, 1.0);
            s.store_div_from_scalar_add_ad(1857, 1.0, A::offset(s.ad_value(1938), 1.0), A::square(s.ad_value(1938)));
            s.store_mul(1858, 1853, 1857);
            s.store_div(1859, 1854, 1858);
            s.store_mul_ad_product_lhs(1940, A::square(s.ad_value(1859)), s.ad_value(1844), 1844);
        }

        s.b[2216] = (s.v[0] == (-1.0));
        s.v[2216] = if s.b[2216] { 1.0 } else { 0.0 };

        if (s.b[2215] && s.b[2216]) {
            s.store_div_scaled_value_offset_denominator(1940, s.ad_value(1940), 1.0, A::mul(s.ad_value(1859), s.ad_value(1844)), 1.0, 1.0);
        }

        if s.b[2215] {
            s.store_mul_offset_rhs_scaled_ad_rhs(1941, 1858, A::sqrt(A::scale_offset(s.ad_value(1940), 2.0, 1.0)), 1.0, 0.5);
            s.store_div_from_scalar(1860, 1.0, 1941);
            s.store_mul(1929, 1858, 1860);
            s.store_mul_offset_ad_rhs(1942, 1848, A::mul3_scaled_output(s.ad_value(1940), s.ad_value(1929), s.ad_value(1929), 0.5), 1.0);
            s.store_div_scaled_product_indices(1861, 1929, 1850, 1.0, 1942, 1.0);
            s.store_mul_product3_rhs(827, 1860, s.ad_value(1917), s.ad_value(1850), s.ad_value(1844), 1.0);
        }

        s.v[1944] = 0.0;

        s.v[1945] = 0.0;

        s.v[1862] = 0.0;

        s.v[1863] = 0.0;

        s.b[2217] = (((((p.p40 != 0.0) && ((s.v[235] > 0.0) || (s.v[236] > 0.0))) || ((p.p42 != 0.0) && ((s.v[245] > 0.0) || (s.v[246] > 0.0)))) || (s.v[260] > 0.0)) || (s.v[261] > 0.0));
        s.v[2217] = if s.b[2217] { 1.0 } else { 0.0 };

        if s.b[2217] {
            s.store_scaled_add_ad_rhs(1943, 1801, A::sqrt(A::add(A::square(s.ad_value(1801)), s.ad_value(778))), 0.5);
            s.store_add_ad_lhs(1944, A::add_scaled_inputs_product(s.ad_value(1943), -1.0, s.ad_value(773), (-0.5), s.ad_value(771), A::sqrt(A::add_scaled_inputs3(s.ad_value(1943), 1.0, s.ad_value(773), 0.25, s.ad_value(779), 1.0)), 1.0), 780);
            s.store_scaled_add_ad_rhs(1943, 1802, A::sqrt(A::add(A::square(s.ad_value(1802)), s.ad_value(781))), 0.5);
            s.store_add_ad_lhs(1945, A::add_scaled_inputs_product(s.ad_value(1943), -1.0, s.ad_value(774), (-0.5), s.ad_value(772), A::sqrt(A::add_scaled_inputs3(s.ad_value(1943), 1.0, s.ad_value(774), 0.25, s.ad_value(782), 1.0)), 1.0), 783);
            s.store_scaled_add(1862, 1801, 1944, (-s.v[355]));
            s.store_scaled_add(1863, 1802, 1945, (-s.v[355]));
        }

        s.b[2218] = (p.p40 != 0.0);
        s.v[2218] = if s.b[2218] { 1.0 } else { 0.0 };

        s.b[2219] = (s.v[235] > 0.0);
        s.v[2219] = if s.b[2219] { 1.0 } else { 0.0 };

        if (s.b[2218] && s.b[2219]) {
            s.store_mul_sqrt_ad_lhs(1946, A::offset(A::square(s.ad_value(1862)), 1e-6), 784);
        }

        s.b[2220] = (s.v[241] < 0.0);
        s.v[2220] = if s.b[2220] { 1.0 } else { 0.0 };

        if ((s.b[2218] && s.b[2219]) && s.b[2220]) {
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1946, 1946, 0.5, 790, 0.5, A::offset(A::mul(A::sub(s.ad_value(1946), s.ad_value(790)), A::sub(s.ad_value(1946), s.ad_value(790))), 1e-6), (-0.5));
        }

        if (s.b[2218] && s.b[2219]) {
            s.store_mul_offset_ad_rhs(1929, 787, A::mul(s.ad_value(1946), A::add_scaled_product(s.ad_value(240), 1.0, s.ad_value(241), s.ad_value(1946), 1.0)), (-1.5));
            s.store_offset(1948, 1944, 3.0);
            s.store_sub_from_scalar(1949, (-3.0), 233);
            s.store_scale(1950, 823, 30.0);
            s.store_scalar(807, (4.0 - 0.9));
            s.store_add(808, 1948, 1950);
            s.store_mul_ad(1929, A::div_from_scalar(2.0, s.ad_value(807)), A::sub(s.ad_value(808), A::sqrt(A::sub(A::square(s.ad_value(808)), A::mul3(s.ad_value(807), s.ad_value(1948), s.ad_value(1950))))));
            s.store_scalar(807, (4.0 - 0.3));
            s.store_add(808, 1949, 1929);
        }

        s.b[2223] = (s.v[236] > 0.0);
        s.v[2223] = if s.b[2223] { 1.0 } else { 0.0 };

        if (s.b[2218] && s.b[2223]) {
            s.store_mul_sqrt_ad_lhs(1946, A::offset(A::square(s.ad_value(1863)), 1e-6), 784);
        }

        s.b[2224] = (s.v[243] < 0.0);
        s.v[2224] = if s.b[2224] { 1.0 } else { 0.0 };

        if ((s.b[2218] && s.b[2223]) && s.b[2224]) {
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1946, 1946, 0.5, 791, 0.5, A::offset(A::mul(A::sub(s.ad_value(1946), s.ad_value(791)), A::sub(s.ad_value(1946), s.ad_value(791))), 1e-6), (-0.5));
        }

        if (s.b[2218] && s.b[2223]) {
            s.store_mul_offset_ad_rhs(1929, 788, A::mul(s.ad_value(1946), A::add_scaled_product(s.ad_value(242), 1.0, s.ad_value(243), s.ad_value(1946), 1.0)), (-1.5));
            s.store_offset(1948, 1945, 3.0);
            s.store_sub_from_scalar(1949, (-3.0), 233);
            s.store_scale(1950, 826, 30.0);
            s.store_scalar(807, (4.0 - 0.9));
            s.store_add(808, 1948, 1950);
            s.store_mul_ad(1929, A::div_from_scalar(2.0, s.ad_value(807)), A::sub(s.ad_value(808), A::sqrt(A::sub(A::square(s.ad_value(808)), A::mul3(s.ad_value(807), s.ad_value(1948), s.ad_value(1950))))));
            s.store_scalar(807, (4.0 - 0.3));
            s.store_add(808, 1949, 1929);
        }

        s.b[2227] = (s.v[234] > 0.0);
        s.v[2227] = if s.b[2227] { 1.0 } else { 0.0 };

        s.b[2228] = (s.v[1813] <= 0.0);
        s.v[2228] = if s.b[2228] { 1.0 } else { 0.0 };

        if ((s.b[2218] && s.b[2227]) && s.b[2228]) {
            s.store_offset(1929, 766, 1.0);
            s.store_div_scaled_product_left_ad(1930, A::sqrt(s.ad_value(1929)), 815, 1.0, 1839, 1.0);
            s.store_add_ad_lhs(1931, A::square(s.ad_value(1930)), 1929);
            s.store_scale(1929, 1930, 2.0);
            s.store_div_scaled_product3_mixed_iiia(1842, 1839, 1809, 1929, 1.0, A::add(A::sqrt(A::sub(s.ad_value(1931), s.ad_value(1929))), A::sqrt(A::add(s.ad_value(1931), s.ad_value(1929)))), 1.0);
        }

        s.b[2229] = ((s.v[1843] - s.v[1842]) > (-230.25850929940458));
        s.v[2229] = if s.b[2229] { 1.0 } else { 0.0 };

        if ((s.b[2218] && s.b[2227]) && s.b[2229]) {
            s.store_exp_sub(1929, 1843, 1842);
        }

        if ((s.b[2218] && s.b[2227]) && (!s.b[2229])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1929, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1843), s.ad_value(1842)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1843), s.ad_value(1842)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (s.b[2218] && s.b[2227]) {
            s.store_add_scaled_product_right_ad(1952, 1932, 1.0, 1808, A::sub_scaled_inputs(s.ad_value(1843), 0.5, A::ln_scaled_input(A::offset(s.ad_value(1929), 1.0), 0.5), 1.0), 1.0);
            s.store_mul(1953, 233, 1808);
            s.store_add(1954, 1856, 1953);
            s.store_scaled_sub_ad_rhs(1955, 1954, A::sqrt(A::offset(A::mul_scaled_output(s.ad_value(1954), s.ad_value(1954), 1.0), 0.01)), 0.5);
            s.store_mul_sqrt_ad_lhs(1946, A::offset(A::square(s.ad_value(1856)), 1e-6), 784);
        }

        s.b[2230] = (s.v[239] < 0.0);
        s.v[2230] = if s.b[2230] { 1.0 } else { 0.0 };

        if ((s.b[2218] && s.b[2227]) && s.b[2230]) {
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1946, 1946, 0.5, 789, 0.5, A::offset(A::mul(A::sub(s.ad_value(1946), s.ad_value(789)), A::sub(s.ad_value(1946), s.ad_value(789))), 1e-6), (-0.5));
        }

        if (s.b[2218] && s.b[2227]) {
            s.store_add_scaled_product_left_ad(1956, 1846, 1.0, A::add_scaled_inputs3(s.ad_value(1955), 1.0, s.ad_value(731), (-1.0), s.ad_value(1952), -1.0), 1809, 1.0);
            s.store_mul_neg_ad_lhs(1956, A::add_scaled_inputs3(s.ad_value(814), 1.0, s.ad_value(1932), 1.0, s.ad_value(1952), -1.0), 1809);
        }

        s.b[2233] = (((s.v[1956]) as f64).abs() < 230.25850929940458);
        s.v[2233] = if s.b[2233] { 1.0 } else { 0.0 };

        if ((s.b[2218] && s.b[2227]) && s.b[2233]) {
            s.store_exp(1929, 1956);
        }

    }

    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2234] = (s.v[1956] < 0.0);
        s.v[2234] = if s.b[2234] { 1.0 } else { 0.0 };

        if (((s.b[2218] && s.b[2227]) && (!s.b[2233])) && s.b[2234]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1929, 1e-100, (-230.25850929940458), 1956, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[2218] && s.b[2227]) && (!s.b[2233])) && (!s.b[2234])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(1929, 1956, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(1956), (-230.25850929940458), A::scale_offset(s.ad_value(1956), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (s.b[2218] && s.b[2227]) {
            s.store_mul_offset_ad_rhs(1929, 786, A::mul(s.ad_value(1946), A::add_scaled_product(s.ad_value(238), 1.0, s.ad_value(239), s.ad_value(1946), 1.0)), (-1.5));
        }

        s.b[2237] = ((s.v[1813] <= 0.0) || ((s.v[238] == 0.0) && (s.v[239] == 0.0)));
        s.v[2237] = if s.b[2237] { 1.0 } else { 0.0 };

        if ((s.b[2218] && s.b[2227]) && (!s.b[2237])) {
            s.store_add_scaled_product_indices(1929, 238, 1.0, 239, 1946, 2.0);
            s.store_div_ad_rhs(1960, 244, A::mul(s.ad_value(1929), s.ad_value(786)));
            s.store_scaled_div(1961, 1844, 1960, 0.5);
        }

        s.b[2238] = (s.v[1961] < 0.001);
        s.v[2238] = if s.b[2238] { 1.0 } else { 0.0 };

        s.b[2239] = (((s.v[1961]) as f64).abs() < 230.25850929940458);
        s.v[2239] = if s.b[2239] { 1.0 } else { 0.0 };

        if ((((s.b[2218] && s.b[2227]) && (!s.b[2237])) && (!s.b[2238])) && s.b[2239]) {
            s.store_exp(1969, 1961);
        }

        s.b[2240] = (s.v[1961] < 0.0);
        s.v[2240] = if s.b[2240] { 1.0 } else { 0.0 };

        if (((((s.b[2218] && s.b[2227]) && (!s.b[2237])) && (!s.b[2238])) && (!s.b[2239])) && s.b[2240]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1969, 1e-100, (-230.25850929940458), 1961, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2218] && s.b[2227]) && (!s.b[2237])) && (!s.b[2238])) && (!s.b[2239])) && (!s.b[2240])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(1969, 1961, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(1961), (-230.25850929940458), A::scale_offset(s.ad_value(1961), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[2218] && s.b[2227]) && (!s.b[2237])) && (!s.b[2238])) {
            s.store_div_from_scalar(1970, 1.0, 1969);
            s.store_sub(1929, 1969, 1970);
            s.store_add(1931, 1969, 1970);
        }

        s.b[2241] = (p.p42 != 0.0);
        s.v[2241] = if s.b[2241] { 1.0 } else { 0.0 };

        s.b[2242] = ((s.v[246] > 0.0) && (s.v[1863] < 0.0));
        s.v[2242] = if s.b[2242] { 1.0 } else { 0.0 };

        if (s.b[2241] && s.b[2242]) {
            s.store_sqrt_offset_ad(1973, A::add_scaled_square_product(s.ad_value(1863), 1.0, A::square(s.ad_value(252)), A::square(s.ad_value(825)), 1.0), 1e-6);
            s.store_div_scaled_inputs(1929, s.ad_value(796), -1.0, s.ad_value(1973), 1.0);
        }

        s.b[2243] = (s.v[1929] > (-230.25850929940458));
        s.v[2243] = if s.b[2243] { 1.0 } else { 0.0 };

        if ((s.b[2241] && s.b[2242]) && s.b[2243]) {
            s.store_exp(1931, 1929);
        }

        if ((s.b[2241] && s.b[2242]) && (!s.b[2243])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1931, 1e-100, (-230.25850929940458), 1929, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2244] = ((s.v[245] > 0.0) && (s.v[1862] < 0.0));
        s.v[2244] = if s.b[2244] { 1.0 } else { 0.0 };

        if (s.b[2241] && s.b[2244]) {
            s.store_sqrt_offset_ad(1974, A::add_scaled_square_product(s.ad_value(1862), 1.0, A::square(s.ad_value(251)), A::square(s.ad_value(824)), 1.0), 1e-6);
            s.store_div_scaled_inputs(1929, s.ad_value(795), -1.0, s.ad_value(1974), 1.0);
        }

        s.b[2245] = (s.v[1929] > (-230.25850929940458));
        s.v[2245] = if s.b[2245] { 1.0 } else { 0.0 };

        if ((s.b[2241] && s.b[2244]) && s.b[2245]) {
            s.store_exp(1931, 1929);
        }

        if ((s.b[2241] && s.b[2244]) && (!s.b[2245])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1931, 1e-100, (-230.25850929940458), 1929, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.copy_ad(1978, 1916);

        s.v[1864] = 0.0;

        s.v[1865] = 0.0;

        s.v[1866] = 0.0;

        s.v[1867] = 1e-40;

        s.v[1868] = 1.0;

        s.v[835] = 0.0;

        s.b[2246] = ((p.p46 != 0.0) && (s.v[285] > 0.0));
        s.v[2246] = if s.b[2246] { 1.0 } else { 0.0 };

        if s.b[2246] {
            s.store_add_scaled_inputs4(1929, s.ad_value(817), 0.5, s.ad_value(816), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(753), 1.0, A::sub(s.ad_value(817), s.ad_value(816)), A::sub(s.ad_value(817), s.ad_value(816)), 1.0)), (-0.5), s.ad_value(751), 1.0);
            s.store_add_scaled_inputs4(1975, s.ad_value(816), 1.0, s.ad_value(1929), (-0.5), A::sqrt(A::add_scaled_product(s.ad_value(752), 1.0, s.ad_value(1929), s.ad_value(1929), 1.0)), (-(-0.5)), s.ad_value(755), 1.0);
            s.store_add_scaled_inputs3(1976, s.ad_value(1975), 1.0, s.ad_value(815), 0.5, s.ad_value(819), (-0.5));
            s.store_mul_ad_product_rhs(1977, 287, A::offset(A::mul(s.ad_value(289), s.ad_value(819)), 1.0), A::offset(A::mul(s.ad_value(288), s.ad_value(1976)), 1.0));
            s.store_mul_offset_rhs(1978, 1924, 1977, 1.0);
            s.store_div_from_scalar(1979, 1.0, 1978);
            s.store_div_scaled_value_offset_denominator(1980, s.ad_value(819), 2.0, A::sqrt(A::offset(A::mul(s.ad_value(291), s.ad_value(819)), 1.0)), 1.0, 1.0);
            s.store_mul_ad_product_rhs(1981, 290, s.ad_value(1980), A::offset(A::mul(s.ad_value(292), s.ad_value(1976)), 1.0));
            s.store_mul_add_scaled_inputs3_offset_rhs(1864, 1979, s.ad_value(818), 1.0, s.ad_value(1981), 1.0, s.ad_value(714), -1.0, 0.0);
            s.store_mul(1982, 1979, 749);
            s.store_scaled_ln_ad(1983, A::add(A::div(s.ad_value(1982), s.ad_value(750)), A::sqrt(s.ad_value(1982))), 2.0);
            s.store_mul(1984, 1979, 1975);
            s.store_add(1989, 1982, 1984);
            s.store_add_scaled_product_right_ad(1990, 1989, 1.0, 750, A::sqrt(s.ad_value(1989)), 1.0);
            s.store_add(1991, 1990, 1983);
            s.store_offset_div_scaled_inputs(1992, s.ad_value(750), 1.0, A::sqrt(s.ad_value(1989)), 2.0, 1.0);
            s.store_div_from_scalar(1993, 1.0, 1992);
            s.store_sub(1994, 1864, 1991);
        }

        s.b[2247] = (s.v[1994] > (-12.0));
        s.v[2247] = if s.b[2247] { 1.0 } else { 0.0 };

        if (s.b[2246] && s.b[2247]) {
            s.store_offset_add(1995, 1994, 1926, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(1996, 1995, 1995, 10.0, 0.5);
            s.store_add_ad_lhs(1997, A::add_scaled_product(s.ad_value(1994), 1.0, s.ad_value(1992), A::ln(s.ad_value(1996)), (-1.0)), 1926);
            s.store_scaled_add_sqrt_square_offset_rhs(1998, 1997, 1997, 2.0, 0.5);
        }

        s.b[2248] = ((s.v[1994] - s.v[1998]) < 230.25850929940458);
        s.v[2248] = if s.b[2248] { 1.0 } else { 0.0 };

        if ((s.b[2246] && s.b[2247]) && s.b[2248]) {
            s.store_exp_sub(1999, 1994, 1998);
        }

        if ((s.b[2246] && s.b[2247]) && (!s.b[2248])) {
            s.store_scaled_offset_mul_offset_lhs_ad(1999, A::sub(s.ad_value(1994), s.ad_value(1998)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(1994), s.ad_value(1998)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(1994), s.ad_value(1998)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (s.b[2246] && s.b[2247]) {
            s.store_mul(2000, 1925, 1999);
            s.store_pow_ad(2001, s.ad_value(2000), s.ad_value(1993));
            s.store_add_scaled_square_product_mixed_iai(2002, 1992, 1.0, A::add_scaled_inputs3(s.ad_value(1998), 2.0, s.ad_value(1992), 2.0, s.ad_value(2001), -1.0), 2001, 1.0);
            s.store_mul_offset_ad_rhs(2003, 1992, A::div_scaled_inputs2(A::sqrt(s.ad_value(2002)), 1.0, s.ad_value(1992), (-1.0), s.ad_value(2001), 1.0), (-1.0));
            s.store_sub(1985, 1998, 2003);
        }

        s.b[2249] = ((s.v[1993] * (s.v[1994] + s.v[1926])) > (-230.25850929940458));
        s.v[2249] = if s.b[2249] { 1.0 } else { 0.0 };

        if ((s.b[2246] && (!s.b[2247])) && s.b[2249]) {
            s.store_exp_ad(1985, A::mul(s.ad_value(1993), A::add(s.ad_value(1994), s.ad_value(1926))));
        }

        if ((s.b[2246] && (!s.b[2247])) && (!s.b[2249])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1985, 1e-100, (-230.25850929940458), A::mul(s.ad_value(1993), A::add(s.ad_value(1994), s.ad_value(1926))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::mul(s.ad_value(1993), A::add(s.ad_value(1994), s.ad_value(1926))), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if s.b[2246] {
            s.store_mul_add_rhs(1986, 1979, 1841, 1975);
        }

        s.b[2250] = ((s.v[1985] < 0.001) && (s.v[1841] < 1e-6));
        s.v[2250] = if s.b[2250] { 1.0 } else { 0.0 };

        s.b[2251] = (((-s.v[1986]) + s.v[1984]) > (-230.25850929940458));
        s.v[2251] = if s.b[2251] { 1.0 } else { 0.0 };

        if ((s.b[2246] && s.b[2250]) && s.b[2251]) {
            s.store_exp_sub(1929, 1984, 1986);
        }

        if ((s.b[2246] && s.b[2250]) && (!s.b[2251])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1929, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1984), s.ad_value(1986)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1984), s.ad_value(1986)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (s.b[2246] && s.b[2250]) {
            s.store_mul_offset_rhs(1865, 1985, 1929, (-1.0));
            s.store_add(1987, 1865, 1985);
        }

        if (s.b[2246] && (!s.b[2250])) {
            s.store_add(1989, 1982, 1986);
            s.store_add_scaled_product_right_ad(1990, 1989, 1.0, 750, A::sqrt(s.ad_value(1989)), 1.0);
            s.store_add(1991, 1990, 1983);
            s.store_offset_div_scaled_inputs(1992, s.ad_value(750), 1.0, A::sqrt(s.ad_value(1989)), 2.0, 1.0);
            s.store_div_from_scalar(1993, 1.0, 1992);
            s.store_sub(1994, 1864, 1991);
        }

        s.b[2252] = (s.v[1994] > (-12.0));
        s.v[2252] = if s.b[2252] { 1.0 } else { 0.0 };

        if ((s.b[2246] && (!s.b[2250])) && s.b[2252]) {
            s.store_offset_add(1995, 1994, 1926, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(1996, 1995, 1995, 10.0, 0.5);
            s.store_add_ad_lhs(1997, A::add_scaled_product(s.ad_value(1994), 1.0, s.ad_value(1992), A::ln(s.ad_value(1996)), (-1.0)), 1926);
            s.store_scaled_add_sqrt_square_offset_rhs(1998, 1997, 1997, 2.0, 0.5);
        }

        s.b[2253] = ((s.v[1994] - s.v[1998]) < 230.25850929940458);
        s.v[2253] = if s.b[2253] { 1.0 } else { 0.0 };

        if (((s.b[2246] && (!s.b[2250])) && s.b[2252]) && s.b[2253]) {
            s.store_exp_sub(1999, 1994, 1998);
        }

        if (((s.b[2246] && (!s.b[2250])) && s.b[2252]) && (!s.b[2253])) {
            s.store_scaled_offset_mul_offset_lhs_ad(1999, A::sub(s.ad_value(1994), s.ad_value(1998)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(1994), s.ad_value(1998)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(1994), s.ad_value(1998)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if ((s.b[2246] && (!s.b[2250])) && s.b[2252]) {
            s.store_mul(2000, 1925, 1999);
            s.store_pow_ad(2001, s.ad_value(2000), s.ad_value(1993));
            s.store_add_scaled_square_product_mixed_iai(2002, 1992, 1.0, A::add_scaled_inputs3(s.ad_value(1998), 2.0, s.ad_value(1992), 2.0, s.ad_value(2001), -1.0), 2001, 1.0);
            s.store_mul_offset_ad_rhs(2003, 1992, A::div_scaled_inputs2(A::sqrt(s.ad_value(2002)), 1.0, s.ad_value(1992), (-1.0), s.ad_value(2001), 1.0), (-1.0));
            s.store_sub(1987, 1998, 2003);
        }

        s.b[2254] = ((s.v[1993] * (s.v[1994] + s.v[1926])) > (-230.25850929940458));
        s.v[2254] = if s.b[2254] { 1.0 } else { 0.0 };

        if (((s.b[2246] && (!s.b[2250])) && (!s.b[2252])) && s.b[2254]) {
            s.store_exp_ad(1987, A::mul(s.ad_value(1993), A::add(s.ad_value(1994), s.ad_value(1926))));
        }

        if (((s.b[2246] && (!s.b[2250])) && (!s.b[2252])) && (!s.b[2254])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1987, 1e-100, (-230.25850929940458), A::mul(s.ad_value(1993), A::add(s.ad_value(1994), s.ad_value(1926))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::mul(s.ad_value(1993), A::add(s.ad_value(1994), s.ad_value(1926))), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (s.b[2246] && (!s.b[2250])) {
            s.store_sub(1865, 1987, 1985);
        }

        if s.b[2246] {
            s.store_scaled_add(1866, 1987, 1985, 0.5);
        }

        if s.b[2246] {
            if ((s.v[1864] - s.v[1866]) > 1e-40) {
                s.store_sub(1867, 1864, 1866);
            } else {
                s.store_scalar(1867, 1e-40);
            }
        }

        if s.b[2246] {
            s.store_sub_from_scalar_ad(1868, 1.0, A::div_scaled_inputs(s.ad_value(750), 0.5, A::sqrt(A::add_scaled_inputs(s.ad_value(1867), 1.0, s.ad_value(1925), 0.25)), 1.0));
            s.store_div_scaled_product3_mixed_aaii(835, A::mul3_scaled_output(s.ad_value(1918), s.ad_value(1978), s.ad_value(1978), -1.0), A::offset(A::mul(s.ad_value(1868), s.ad_value(1866)), 1.0), 1865, 1.0, 1853, 1.0);
        }

        s.v[1869] = 0.0;

        s.v[836] = 0.0;

        s.b[2255] = ((s.v[1813] > 0.0) && (p.p41 != 0.0));
        s.v[2255] = if s.b[2255] { 1.0 } else { 0.0 };

        if s.b[2255] {
            s.store_add_scaled_product_indices(1988, 815, 1.0, 230, 1844, (-1.0));
        }

        s.b[2256] = (s.v[1988] > 0.0);
        s.v[2256] = if s.b[2256] { 1.0 } else { 0.0 };

        if (s.b[2255] && s.b[2256]) {
            s.store_mul_div_scaled_offset_numerator_rhs(1931, 713, A::mul(s.ad_value(231), A::sub(A::sqrt(A::add(s.ad_value(717), s.ad_value(1932))), s.ad_value(725))), 1.0, 1.0, A::offset(s.ad_value(1988), 1e-30), 1.0);
        }

        s.b[2257] = ((((-s.v[1931])) as f64).abs() < 230.25850929940458);
        s.v[2257] = if s.b[2257] { 1.0 } else { 0.0 };

        if ((s.b[2255] && s.b[2256]) && s.b[2257]) {
            s.store_exp_neg_input(1929, 1931);
        }

        s.b[2258] = ((-s.v[1931]) < 0.0);
        s.v[2258] = if s.b[2258] { 1.0 } else { 0.0 };

        if (((s.b[2255] && s.b[2256]) && (!s.b[2257])) && s.b[2258]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1929, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1931)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(1931)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[2255] && s.b[2256]) && (!s.b[2257])) && (!s.b[2258])) {
            s.store_scaled_offset_mul_offset_lhs_ad(1929, A::neg(s.ad_value(1931)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(1931)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(1931)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (s.b[2255] && s.b[2256]) {
            s.store_mul3_lhs(1869, 227, 1988, 1929);
            s.store_mul_add_rhs(836, 1869, 827, 835);
        }

        s.b[2259] = (s.v[836] > (0.5 * s.v[232]));
        s.v[2259] = if s.b[2259] { 1.0 } else { 0.0 };

        if ((s.b[2255] && s.b[2256]) && s.b[2259]) {
            s.store_offset_div_scaled_inputs(1929, s.ad_value(836), 2.0, s.ad_value(232), 1.0, (-1.0));
            s.store_mul_scaled_offset_ad_rhs(836, 232, 0.5, A::div(s.ad_value(1929), A::sqrt(A::offset(A::square(s.ad_value(1929)), 1.0))), 1.0);
        }

        s.b[2453] = (((p.p45 == 1.0) || (p.p47 > 0.0)) || (p.p48 > 0.0));
        s.v[2453] = if s.b[2453] { 1.0 } else { 0.0 };

        s.b[2454] = ((p.p45 > 0.0) || (p.p47 > 0.0));
        s.v[2454] = if s.b[2454] { 1.0 } else { 0.0 };

        if (s.b[2453] && s.b[2454]) {
            s.copy_ad(2294, 717);
            s.copy_ad(2295, 727);
            s.copy_ad(2296, 718);
            s.copy_ad(2297, 1804);
            s.copy_ad(2298, 1805);
            s.store_scalar(2302, 0.0);
        }

        s.b[2455] = (p.p47 > 0.0);
        s.v[2455] = if s.b[2455] { 1.0 } else { 0.0 };

        if ((s.b[2453] && s.b[2454]) && s.b[2455]) {
            s.store_add_scaled_inputs4(2297, s.ad_value(817), 0.5, s.ad_value(816), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(738), 1.0, A::sub(s.ad_value(817), s.ad_value(816)), A::sub(s.ad_value(817), s.ad_value(816)), 1.0)), (-0.5), s.ad_value(736), 1.0);
            s.store_add_scaled_inputs4(1870, s.ad_value(816), 1.0, s.ad_value(2297), (-0.5), A::sqrt(A::add_scaled_product(s.ad_value(737), 1.0, s.ad_value(2297), s.ad_value(2297), 1.0)), (-(-0.5)), s.ad_value(739), 1.0);
            s.copy_ad(2298, 1870);
            s.copy_ad(2294, 734);
            s.copy_ad(2295, 737);
            s.copy_ad(2296, 735);
        }

    }

    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[2453] && s.b[2454]) {
            s.store_add_scaled_inputs3(2301, s.ad_value(818), 1.0, s.ad_value(2302), (-1.0), s.ad_value(701), -1.0);
            s.store_add_scaled_inputs3(2303, s.ad_value(2298), 1.0, s.ad_value(815), 0.5, s.ad_value(819), (-0.5));
            s.store_scalar(2315, 1.0);
        }

        s.b[2456] = (s.v[188] > 0.0);
        s.v[2456] = if s.b[2456] { 1.0 } else { 0.0 };

        if ((s.b[2453] && s.b[2454]) && s.b[2456]) {
            s.store_mul(2306, 2294, 362);
            s.store_mul(2307, 2303, 362);
            s.store_mul(2308, 2301, 362);
            s.store_offset_div_scaled_inputs(1930, s.ad_value(2296), 0.5, A::sqrt(s.ad_value(2306)), 1.0, 1.0);
            s.store_add_scaled_product_right_ad(1931, 2306, 1.0, 2296, A::sqrt(s.ad_value(2306)), 1.0);
            s.store_add_scaled_inputs_product_mixed_aiai(2309, A::div_scaled_inputs2(s.ad_value(2308), 1.0, s.ad_value(1931), (-1.0), s.ad_value(1930), 1.0), 1.0, 2306, 0.5, A::offset(s.ad_value(189), 1.0), 2307, (-1.0));
            s.store_offset_scaled(2310, 2306, 0.5, 2.0);
            s.store_add(2311, 2306, 2307);
            s.store_sub_scaled_inputs_ad(1930, A::add_scaled_inputs_product(s.ad_value(2308), 1.0, s.ad_value(2311), (-1.0), s.ad_value(2296), A::sqrt(s.ad_value(2311)), (-1.0)), 1.0, A::ln(A::add(A::div(s.ad_value(2306), s.ad_value(2296)), A::sqrt(s.ad_value(2306)))), 2.0);
            s.store_add_scaled_inputs(2312, 1930, 2.0, 2310, 1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1930, 2309, 0.5, 2312, 0.5, A::offset(A::mul(A::sub(s.ad_value(2309), s.ad_value(2312)), A::sub(s.ad_value(2309), s.ad_value(2312))), 20.0), 0.5);
            s.store_add_scaled_inputs3(1931, s.ad_value(2308), 2.0, s.ad_value(2307), (-2.0), s.ad_value(2310), -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2313, 1930, 0.5, 1931, 0.5, A::offset(A::mul(A::sub(s.ad_value(1930), s.ad_value(1931)), A::sub(s.ad_value(1930), s.ad_value(1931))), 20.0), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1930, 2313, 0.5, 2310, 0.5, A::offset(A::mul(A::sub(s.ad_value(2313), s.ad_value(2310)), A::sub(s.ad_value(2313), s.ad_value(2310))), 5.0), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2314, 1930, 0.5, 2310, ((-1.0) * 0.5), A::offset(A::mul(A::sub_scaled_inputs(s.ad_value(1930), 1.0, s.ad_value(2310), -1.0), A::sub_scaled_inputs(s.ad_value(1930), 1.0, s.ad_value(2310), -1.0)), 20.0), 0.5);
            s.store_mul_offset_ad_rhs(1931, 703, A::div(s.ad_value(2314), s.ad_value(2310)), 1.0);
        }

        s.b[2457] = (s.v[1931] > (-230.25850929940458));
        s.v[2457] = if s.b[2457] { 1.0 } else { 0.0 };

        if (((s.b[2453] && s.b[2454]) && s.b[2456]) && s.b[2457]) {
            s.store_exp(2315, 1931);
        }

        if (((s.b[2453] && s.b[2454]) && s.b[2456]) && (!s.b[2457])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2315, 1e-100, (-230.25850929940458), 1931, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (s.b[2453] && s.b[2454]) {
            s.store_offset_mul(2316, 702, 2315, 1.0);
            s.store_mul(2317, 1916, 2316);
            s.store_mul_ad_product_rhs(2318, 197, A::offset(A::mul(s.ad_value(199), s.ad_value(819)), 1.0), A::offset(A::mul(s.ad_value(198), s.ad_value(2303)), 1.0));
            s.store_mul_offset_rhs(2319, 2317, 2318, 1.0);
            s.store_div_from_scalar(2320, 1.0, 2319);
            s.store_mul_sqrt_ad_rhs(2304, 2296, A::mul(s.ad_value(1916), s.ad_value(2320)));
            s.store_square(2305, 2304);
            s.store_div_from_scalar(2321, 1.0, 2305);
            s.store_mul(2322, 2298, 2320);
            s.store_mul(2323, 2301, 2320);
            s.store_div_scaled_value_offset_denominator(2324, s.ad_value(819), 2.0, A::sqrt(A::offset(A::mul(s.ad_value(195), s.ad_value(819)), 1.0)), 1.0, 1.0);
            s.store_mul_ad_product_rhs(2325, 194, s.ad_value(2324), A::offset(A::mul(s.ad_value(196), s.ad_value(2303)), 1.0));
            s.store_mul(2326, 2294, 2320);
            s.store_sqrt_square_add(1930, 2297, 2295);
            s.store_sqrt_ad(1931, A::add_scaled_product(s.ad_value(2295), 1.0, A::sub(s.ad_value(2297), s.ad_value(2325)), A::sub(s.ad_value(2297), s.ad_value(2325)), 1.0));
            s.store_mul_scaled_ad_rhs(2327, 2320, 0.5, A::add_scaled_inputs3(s.ad_value(2325), 1.0, s.ad_value(1930), 1.0, s.ad_value(1931), -1.0));
            s.store_add(2328, 2326, 2322);
            s.store_sub(2329, 2328, 2327);
        }

        s.b[2458] = (p.p45 > 0.0);
        s.v[2458] = if s.b[2458] { 1.0 } else { 0.0 };

        s.b[2459] = (((s.v[2329]) as f64).abs() < 1e-5);
        s.v[2459] = if s.b[2459] { 1.0 } else { 0.0 };

        if (((s.b[2453] && s.b[2454]) && s.b[2458]) && s.b[2459]) {
            s.store_offset_ad(2330, A::mul_sub_from_scalar_rhs(s.ad_value(2304), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2329), 1.0, A::scale(s.ad_value(2329), 0.3125), 0.5)), 1.0);
        }

        s.b[2460] = (s.v[2329] < 460.51701859880916);
        s.v[2460] = if s.b[2460] { 1.0 } else { 0.0 };

        if ((((s.b[2453] && s.b[2454]) && s.b[2458]) && (!s.b[2459])) && s.b[2460]) {
            s.store_exp_neg_input(2344, 2329);
        }

        if ((((s.b[2453] && s.b[2454]) && s.b[2458]) && (!s.b[2459])) && (!s.b[2460])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2344, 1e-200, 2329, (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2329), (-460.51701859880916), A::scale_offset(s.ad_value(2329), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if (((s.b[2453] && s.b[2454]) && s.b[2458]) && (!s.b[2459])) {
            s.store_scalar(1929, (if (s.v[2329] > 0.0) { 1.0 } else { (-1.0) }));
        }

        if (((s.b[2453] && s.b[2454]) && s.b[2458]) && (!s.b[2459])) {
            s.store_offset_ad(2330, A::div_scaled_product3(s.ad_value(1929), s.ad_value(2304), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(2344), 1.0, s.ad_value(2329))), 1.0, A::sqrt(A::mul_sub_from_scalar_rhs(s.ad_value(2329), 1.0, s.ad_value(2344))), 2.0), 1.0);
        }

        if ((s.b[2453] && s.b[2454]) && (!s.b[2458])) {
            s.store_offset_div_scaled_inputs(2330, s.ad_value(2304), 0.5, A::sqrt(s.ad_value(2329)), 1.0, 1.0);
        }

        if (s.b[2453] && s.b[2454]) {
            s.store_add_scaled_value_products(2331, s.ad_value(2329), 1.0, s.ad_value(2304), A::sqrt(s.ad_value(2329)), 1.0, s.ad_value(2330), A::ln(A::offset(s.ad_value(2330), (-1.0))), (-1.0));
            s.store_div_scaled_inputs2(2332, s.ad_value(2323), 1.0, s.ad_value(2331), (-1.0), s.ad_value(2330), 1.0);
            s.store_mul_scaled_offset_ad_rhs(2338, 2305, 0.5, A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2305)), 1.0)), (-1.0));
            s.store_scalar(2337, 0.0);
            s.store_scalar(2339, 1.0);
        }

        s.b[2461] = (s.v[2332] > (-30.0));
        s.v[2461] = if s.b[2461] { 1.0 } else { 0.0 };

        if ((s.b[2453] && s.b[2454]) && s.b[2461]) {
            s.store_offset_mul(2333, 2330, 2332, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(1929, 2333, 2333, 10.0, 0.5);
            s.store_sub_ad_rhs(2334, 2332, A::ln(s.ad_value(1929)));
            s.store_scaled_add_sqrt_square_offset_rhs(2335, 2334, 2334, 2.0, 0.5);
        }

        s.b[2462] = ((s.v[2332] - s.v[2335]) < 230.25850929940458);
        s.v[2462] = if s.b[2462] { 1.0 } else { 0.0 };

        if (((s.b[2453] && s.b[2454]) && s.b[2461]) && s.b[2462]) {
            s.store_exp_sub(1929, 2332, 2335);
        }

        if (((s.b[2453] && s.b[2454]) && s.b[2461]) && (!s.b[2462])) {
            s.store_scaled_offset_mul_offset_lhs_ad(1929, A::sub(s.ad_value(2332), s.ad_value(2335)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2332), s.ad_value(2335)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2332), s.ad_value(2335)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if ((s.b[2453] && s.b[2454]) && s.b[2461]) {
            s.store_div(2336, 1929, 2330);
            s.store_sub_ad_lhs(1929, A::scaled_offset(s.ad_value(2335), 1.0, 2.0), 2336);
        }

        s.b[2463] = (s.v[2336] > 1e-6);
        s.v[2463] = if s.b[2463] { 1.0 } else { 0.0 };

        if (((s.b[2453] && s.b[2454]) && s.b[2461]) && s.b[2463]) {
            s.store_mul_offset_ad_rhs(2337, 2330, A::sub(s.ad_value(2335), A::div_scaled_offset_numerator(A::sqrt(A::offset(A::mul(s.ad_value(2336), s.ad_value(1929)), 1.0)), 1.0, (-1.0), s.ad_value(2336), 1.0)), 1.0);
        }

        if (((s.b[2453] && s.b[2454]) && s.b[2461]) && (!s.b[2463])) {
            s.store_mul_ad_affine_product_rhs(2337, 2330, s.ad_value(2336), A::offset(A::mul_scaled_lhs(s.ad_value(1929), 0.25, s.ad_value(1929)), 1.0), 0.5, 0.0);
        }

        if ((s.b[2453] && s.b[2454]) && s.b[2461]) {
            s.store_add_scaled_inputs3_offset(1929, s.ad_value(2323), 0.5, s.ad_value(2337), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul_offset_lhs(A::sub(s.ad_value(2323), s.ad_value(2337)), (-2.0), A::offset(A::sub(s.ad_value(2323), s.ad_value(2337)), (-2.0))), 1.0)), 0.5, (2.0 * 0.5));
            s.store_mul_scaled_offset_ad_rhs(2338, 2305, 0.5, A::sqrt(A::offset(A::mul(A::div_from_scalar(4.0, s.ad_value(2305)), s.ad_value(1929)), 1.0)), (-1.0));
            s.store_div_add_scaled_inputs_rhs_indices(2339, 2338, 2338, 1.0, 2337, 1.0);
            s.store_add_scaled_product_indices(2329, 2328, 1.0, 2339, 2327, (-1.0));
        }

        if (s.b[2453] && s.b[2454]) {
            s.store_offset_scaled(2340, 2304, 0.7071067811865475, 1.0);
            s.store_scale(2341, 2340, 1e-5);
            s.store_div_from_scalar(2342, 1.0, 2340);
            s.store_scalar(2449, 0.0);
            s.store_scalar(2343, 0.0);
        }

        s.b[2464] = (s.v[2329] < 460.51701859880916);
        s.v[2464] = if s.b[2464] { 1.0 } else { 0.0 };

        if ((s.b[2453] && s.b[2454]) && s.b[2464]) {
            s.store_exp_neg_input(2344, 2329);
        }

        if ((s.b[2453] && s.b[2454]) && (!s.b[2464])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2344, 1e-200, 2329, (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2329), (-460.51701859880916), A::scale_offset(s.ad_value(2329), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        s.b[2465] = (((s.v[2323]) as f64).abs() <= s.v[2341]);
        s.v[2465] = if s.b[2465] { 1.0 } else { 0.0 };

        if ((s.b[2453] && s.b[2454]) && s.b[2465]) {
            s.store_scaled_square(2429, 2342, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(2343, 2323, s.ad_value(2342), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2323), 1.0, s.ad_value(2344)), s.ad_value(2304), s.ad_value(2429)), 1.0));
        }

        s.b[2466] = (s.v[2323] < (-s.v[2341]));
        s.v[2466] = if s.b[2466] { 1.0 } else { 0.0 };

        if (((s.b[2453] && s.b[2454]) && (!s.b[2465])) && s.b[2466]) {
            s.store_neg(2431, 2323);
            s.store_scaled_mul(2432, 2431, 2342, 1.25);
            s.store_scaled_sub_ad(2433, A::offset(s.ad_value(2432), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(2432), (-6.0), A::offset(s.ad_value(2432), (-6.0))), 64.0)), 0.5);
            s.store_sub(2428, 2431, 2433);
            s.store_add_scaled_square_product_mixed_iia(2434, 2428, 1.0, 2305, A::offset(s.ad_value(2433), 1.0), 1.0);
            s.store_sub_scaled_inputs(2435, 2428, 2.0, 2305, 1.0);
            s.store_sub_ad_lhs(2436, A::ln(A::mul(s.ad_value(2434), s.ad_value(2321))), 2433);
            s.store_add(813, 2434, 2435);
            s.store_add_scaled_square_product_mixed_iia(812, 813, 1.0, 2436, A::sub_scaled_inputs(A::square(s.ad_value(2435)), 0.5, s.ad_value(2434), 1.0), 1.0);
            s.store_add_ad_rhs(2437, 2433, A::div_scaled_product3(s.ad_value(2434), s.ad_value(813), s.ad_value(2436), 1.0, A::add(s.ad_value(812), A::mul3(A::mul3(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2436), s.ad_value(2436)), s.ad_value(2435), A::sub_scaled_inputs(A::square(s.ad_value(2435)), 0.3333333333333333, s.ad_value(2434), 1.0))), 1.0));
        }

        s.b[2467] = (s.v[2437] < 230.25850929940458);
        s.v[2467] = if s.b[2467] { 1.0 } else { 0.0 };

        if ((((s.b[2453] && s.b[2454]) && (!s.b[2465])) && s.b[2466]) && s.b[2467]) {
            s.store_exp(2438, 2437);
        }

        if ((((s.b[2453] && s.b[2454]) && (!s.b[2465])) && s.b[2466]) && (!s.b[2467])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(2438, 2437, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2437), (-230.25850929940458), A::scale_offset(s.ad_value(2437), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[2453] && s.b[2454]) && (!s.b[2465])) && s.b[2466]) {
            s.store_div_from_scalar(2439, 1.0, 2438);
            s.store_div_from_scalar_offset_ad(2428, 1.0, A::square(s.ad_value(2437)), 2.0);
            s.store_mul_square_lhs(2440, 2437, 2428);
            s.store_mul3_affine_lhs(2441, 2437, 2428, 4.0, 0.0, 2428);
            s.store_mul_ad_product_lhs(2442, A::sub_scaled_inputs(s.ad_value(2428), 8.0, s.ad_value(2440), 12.0), s.ad_value(2428), 2428);
            s.store_sub(2428, 2431, 2437);
            s.store_mul(2429, 2344, 2439);
            s.store_add_scaled_product_right_ad(2443, 2428, 2.0, 2305, A::add_scaled_inputs3_offset(s.ad_value(2438), 1.0, s.ad_value(2429), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(2344), 1.0, s.ad_value(2441)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2444, 2428, 1.0, 2305, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2438), 1.0, s.ad_value(2437), (-1.0), s.ad_value(2429), 1.0, (-1.0)), 1.0, s.ad_value(2344), A::sub(A::offset(s.ad_value(2437), (-1.0)), s.ad_value(2440)), 1.0), (-1.0));
            s.store_sub_from_scalar_ad(2428, 2.0, A::mul(s.ad_value(2305), A::add_scaled_inputs_product(s.ad_value(2438), 1.0, s.ad_value(2429), 1.0, s.ad_value(2344), s.ad_value(2442), (-1.0))));
            s.store_add_scaled_square_product_indices(2428, 2443, 1.0, 2444, 2428, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(2343, 2437, -1.0, A::div(s.ad_value(2444), A::add(s.ad_value(2443), A::sqrt(s.ad_value(2428)))), 2.0);
        }

        if (((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) {
            s.store_div_from_scalar_offset_scaled_input(2445, 1.0, 2304, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(2446, A::mul_scaled_lhs(s.ad_value(2340), 1.25, s.ad_value(2445)), (-1.0), 2445);
            s.store_mul_ad_product_rhs(2447, 2323, s.ad_value(2342), A::offset(A::mul(s.ad_value(2446), s.ad_value(2323)), 1.0));
        }

        s.b[2468] = ((-s.v[2447]) > (-230.25850929940458));
        s.v[2468] = if s.b[2468] { 1.0 } else { 0.0 };

        if ((((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) && s.b[2468]) {
            s.store_exp_neg_input(2428, 2447);
        }

        if ((((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) && (!s.b[2468])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(2428, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2447)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(2447)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) {
            s.store_sub_from_scalar(2448, 1.0, 2428);
            s.store_add_scaled_inputs_product_right_ad(2449, 2323, 1.0, 2305, 0.5, 2304, A::sqrt(A::add_scaled_inputs3(s.ad_value(2323), 1.0, s.ad_value(2305), 0.25, s.ad_value(2448), -1.0)), (-1.0));
            s.store_offset(2450, 2329, 3.0);
            s.store_sub_ad(2433, A::add_scaled_inputs3(s.ad_value(2449), 0.5, s.ad_value(2450), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2449), s.ad_value(2450)), A::sub(s.ad_value(2449), s.ad_value(2450))), 5.0)), (-0.5)), A::sub_scaled_inputs(s.ad_value(2450), 0.5, A::sqrt(A::offset(A::square(s.ad_value(2450)), 5.0)), 0.5));
            s.store_sub(2428, 2323, 2433);
            s.store_exp_neg_input(2429, 2433);
            s.store_div_from_scalar_offset_ad(2430, 1.0, A::square(s.ad_value(2433)), 2.0);
            s.store_mul_square_lhs(2440, 2433, 2430);
            s.store_mul3_affine_lhs(2441, 2433, 2430, 4.0, 0.0, 2430);
            s.store_mul_ad_product_lhs(2442, A::sub_scaled_inputs(s.ad_value(2430), 8.0, s.ad_value(2440), 12.0), s.ad_value(2430), 2430);
        }

        if (((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) {
            if (1e-40 > ((s.v[2428] * s.v[2428]) - (s.v[2305] * (((s.v[2429] + s.v[2433]) - 1.0) - (s.v[2344] * ((s.v[2433] + 1.0) + s.v[2440])))))) {
                s.store_scalar(2434, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2434, 2428, 1.0, 2305, A::add_scaled_product(A::offset(A::add(s.ad_value(2429), s.ad_value(2433)), (-1.0)), 1.0, s.ad_value(2344), A::add(A::offset(s.ad_value(2433), 1.0), s.ad_value(2440)), (-1.0)), (-1.0));
            }
        }

        if (((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) {
            s.store_sub_from_scalar_ad(2451, 1.0, A::mul_scaled_output(s.ad_value(2305), A::add_scaled_product(s.ad_value(2429), 1.0, s.ad_value(2344), s.ad_value(2442), (-1.0)), 0.5));
            s.store_add_scaled_product_right_ad(2435, 2428, 2.0, 2305, A::add_scaled_sub_value_product(1.0, s.ad_value(2429), 1.0, s.ad_value(2344), A::offset(s.ad_value(2441), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3(2436, s.ad_value(2329), 1.0, s.ad_value(2433), (-1.0), A::ln(A::div(s.ad_value(2434), s.ad_value(2305))), 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) {
            s.store_add(813, 2434, 2435);
            s.store_add_scaled_square_product_mixed_iia(812, 813, 1.0, 2436, A::add_scaled_square_product(s.ad_value(2435), 0.5, s.ad_value(2434), s.ad_value(2451), (-1.0)), 1.0);
            s.store_add_ad_rhs(2452, 2433, A::div_scaled_product3(s.ad_value(2434), s.ad_value(813), s.ad_value(2436), 1.0, A::add(s.ad_value(812), A::mul3(A::mul3(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2436), s.ad_value(2436)), s.ad_value(2435), A::add_scaled_square_product(s.ad_value(2435), 0.3333333333333333, s.ad_value(2434), s.ad_value(2451), (-1.0)))), 1.0));
        }

        s.b[2469] = (s.v[2452] < 230.25850929940458);
        s.v[2469] = if s.b[2469] { 1.0 } else { 0.0 };

        if ((((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) && s.b[2469]) {
            s.store_exp(2438, 2452);
            s.store_div_from_scalar(2439, 1.0, 2438);
            s.store_mul(2438, 2344, 2438);
        }

        s.b[2470] = (s.v[2452] > (s.v[2329] - 230.25850929940458));
        s.v[2470] = if s.b[2470] { 1.0 } else { 0.0 };

        if (((((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) && (!s.b[2469])) && s.b[2470]) {
            s.store_exp_sub(2438, 2452, 2329);
            s.store_div(2439, 2344, 2438);
        }

        if (((((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) && (!s.b[2469])) && (!s.b[2470])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(2438, 1e-100, A::sub(s.ad_value(2329), s.ad_value(2452)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2329), s.ad_value(2452)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2329), s.ad_value(2452)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2439, 1e-100, 2452, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2452), (-230.25850929940458), A::scale_offset(s.ad_value(2452), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if (((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) {
            s.store_div_from_scalar_offset_ad(2428, 1.0, A::square(s.ad_value(2452)), 2.0);
            s.store_mul_square_lhs(2440, 2452, 2428);
            s.store_mul3_affine_lhs(2441, 2452, 2428, 4.0, 0.0, 2428);
            s.store_mul_ad_product_lhs(2442, A::sub_scaled_inputs(s.ad_value(2428), 8.0, s.ad_value(2440), 12.0), s.ad_value(2428), 2428);
            s.store_sub(2428, 2323, 2452);
            s.store_add_scaled_product_right_ad(2443, 2428, 2.0, 2305, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2439)), 1.0, s.ad_value(2438), 1.0, s.ad_value(2344), A::offset(s.ad_value(2441), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2444, 2428, 1.0, 2305, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2439), 1.0, s.ad_value(2452), 1.0, s.ad_value(2438), 1.0, (-1.0)), 1.0, s.ad_value(2344), A::add(A::offset(s.ad_value(2452), 1.0), s.ad_value(2440)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_ad(2428, 2.0, A::mul(s.ad_value(2305), A::add_scaled_inputs_product(s.ad_value(2439), 1.0, s.ad_value(2438), 1.0, s.ad_value(2344), s.ad_value(2442), (-1.0))));
            s.store_add_scaled_square_product_indices(2428, 2443, 1.0, 2444, 2428, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(2343, 2452, 1.0, A::div(s.ad_value(2444), A::add(s.ad_value(2443), A::sqrt(s.ad_value(2428)))), 2.0);
        }

        if (s.b[2453] && s.b[2454]) {
            s.store_scalar(2346, 0.0);
            s.store_scalar(2347, 0.0);
            s.store_scalar(2348, 0.0);
            s.store_scalar(2349, 0.0);
            s.store_scalar(2350, 0.0);
            s.store_scalar(2351, 0.0);
            s.store_scalar(2352, 0.0);
            s.store_scalar(2353, 1.0);
            s.store_scalar(2354, 1.0);
            s.store_sub(2355, 2323, 2343);
            s.store_scalar(2356, 0.0);
            s.store_mul(2357, 2319, 2355);
            s.store_scalar(2358, 1.0);
            s.store_scalar(2359, 1.0);
            s.store_scalar(2363, 1.0);
            s.store_scalar(2364, 1.0);
            s.store_scalar(2366, 1.0);
        }

        s.b[2471] = (s.v[2323] > 0.0);
        s.v[2471] = if s.b[2471] { 1.0 } else { 0.0 };

        if ((s.b[2453] && s.b[2454]) && s.b[2471]) {
            s.store_div_from_scalar_offset_ad(1929, 1.0, A::square(s.ad_value(2343)), 2.0);
            s.store_mul_square_lhs(2345, 2343, 1929);
            s.store_mul3_affine_lhs(2346, 2343, 1929, 4.0, 0.0, 1929);
            s.store_mul_ad_product_lhs(2347, A::sub_scaled_inputs(s.ad_value(1929), 8.0, s.ad_value(2345), 12.0), s.ad_value(1929), 1929);
            s.store_scalar(2348, 0.0);
        }

        s.b[2472] = (s.v[2343] < 230.25850929940458);
        s.v[2472] = if s.b[2472] { 1.0 } else { 0.0 };

        if (((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2472]) {
            s.store_exp(2348, 2343);
            s.store_div_from_scalar(2349, 1.0, 2348);
            s.store_mul(2348, 2344, 2348);
        }

        s.b[2473] = (s.v[2343] > (s.v[2329] - 230.25850929940458));
        s.v[2473] = if s.b[2473] { 1.0 } else { 0.0 };

        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && (!s.b[2472])) && s.b[2473]) {
            s.store_exp_sub(2348, 2343, 2329);
            s.store_div(2349, 2344, 2348);
        }

        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && (!s.b[2472])) && (!s.b[2473])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(2348, 1e-100, A::sub(s.ad_value(2329), s.ad_value(2343)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2329), s.ad_value(2343)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2329), s.ad_value(2343)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2349, 1e-100, 2343, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2343), (-230.25850929940458), A::scale_offset(s.ad_value(2343), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if ((s.b[2453] && s.b[2454]) && s.b[2471]) {
            s.store_add_scaled_product_right_ad(2350, 2348, 1.0, 2344, A::add(A::offset(s.ad_value(2343), 1.0), s.ad_value(2345)), (-1.0));
        }

        s.b[2474] = (s.v[2343] < 1e-5);
        s.v[2474] = if s.b[2474] { 1.0 } else { 0.0 };

        if (((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2474]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2351, 2343, 1.0, 2343, 1.0, 2343, 0.25, 0.3333333333333333, 0.5);
            s.store_mul3_ad_middle_scaled_output(2350, A::mul3(s.ad_value(2344), s.ad_value(2343), s.ad_value(2343)), 2343, A::scale_offset(s.ad_value(2343), 1.75, 1.0), 0.16666666666666666);
            s.store_sqrt_sub_from_scalar_ad(1929, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2343), 1.0, A::scale(s.ad_value(2343), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2352, 2343, 1929, 0.7071067811865475);
            s.store_offset_div_scaled_product(2353, s.ad_value(2304), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2343), 0.5)), 1.0, A::square(s.ad_value(2343)), 0.16666666666666666), 0.7071067811865475, s.ad_value(1929), 1.0, 1.0);
        }

        if (((s.b[2453] && s.b[2454]) && s.b[2471]) && (!s.b[2474])) {
            s.store_add_ad_lhs(2351, A::offset(s.ad_value(2343), (-1.0)), 2349);
            s.store_sqrt(2352, 2351);
            s.store_offset_scaled_ad(2353, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2304), 1.0, s.ad_value(2349)), s.ad_value(2352)), 0.5, 1.0);
        }

        if ((s.b[2453] && s.b[2454]) && s.b[2471]) {
            s.store_div_scaled_offset_numerator(2354, A::mul_scaled_lhs(s.ad_value(709), 0.2, s.ad_value(2303)), 1.0, 1.0, A::offset(A::mul(s.ad_value(709), s.ad_value(2303)), 1.0), 1.0);
        }

        s.b[2475] = (s.v[2350] > 1e-100);
        s.v[2475] = if s.b[2475] { 1.0 } else { 0.0 };

        if (((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) {
            s.store_mul_sqrt_ad_rhs(2355, 2304, A::add(s.ad_value(2351), s.ad_value(2350)));
            s.store_div_scaled_product3_mixed_iiia(2356, 2305, 2350, 2319, 1.0, A::add_scaled_product(s.ad_value(2355), 1.0, s.ad_value(2304), s.ad_value(2352), 1.0), 1.0);
            s.store_mul3_lhs(2357, 2352, 2304, 2319);
        }

        s.b[2476] = (s.v[215] < 0.0);
        s.v[2476] = if s.b[2476] { 1.0 } else { 0.0 };

        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) && s.b[2476]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2358, 1.0, 1.0, A::mul(s.ad_value(215), s.ad_value(2303)));
        }

        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) && (!s.b[2476])) {
            s.store_offset_mul(2358, 215, 2303, 1.0);
        }

        s.b[2477] = (s.v[216] < 0.0);
        s.v[2477] = if s.b[2477] { 1.0 } else { 0.0 };

        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) && s.b[2477]) {
            s.store_sub_from_scalar_ad(2359, 1.0, A::mul(s.ad_value(216), s.ad_value(2356)));
        }

        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) && (!s.b[2477])) {
            s.store_div_from_scalar_offset_ad(2359, 1.0, A::mul(s.ad_value(216), s.ad_value(2356)), 1.0);
        }

        if (((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) {
            s.store_mul_product3_rhs(2360, 2356, s.ad_value(746), s.ad_value(2358), s.ad_value(2359), 1.0);
            s.store_mul_add_scaled_product_rhs(2361, 763, s.ad_value(2357), 1.0, s.ad_value(764), s.ad_value(2356), 1.0);
            s.store_ln_ad(1930, A::div_scaled_value_offset_denominator(s.ad_value(2351), 1.0, A::add(s.ad_value(2351), s.ad_value(2350)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2362, A::pow(A::mul(s.ad_value(2361), s.ad_value(705)), s.ad_value(706)), 1.0, 707, A::exp(A::mul_scaled_lhs(s.ad_value(708), 0.5, s.ad_value(1930))), 1.0);
            s.store_mul_add_ad_lhs(2363, A::offset(s.ad_value(2362), 1.0), s.ad_value(2360), 2354);
        }

        s.b[2478] = (s.v[219] < 0.0);
        s.v[2478] = if s.b[2478] { 1.0 } else { 0.0 };

        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) && s.b[2478]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2364, 1.0, 1.0, A::mul(s.ad_value(219), s.ad_value(2303)));
        }

        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) && (!s.b[2478])) {
            s.store_offset_mul(2364, 219, 2303, 1.0);
        }

        if (((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) {
            s.store_mul(1931, 2356, 2364);
            s.store_div_add_scaled_inputs_rhs_indices(2365, 1931, 221, 1.0, 1931, 1.0);
        }

        s.b[2479] = (s.v[220] < 0.0);
        s.v[2479] = if s.b[2479] { 1.0 } else { 0.0 };

        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) && s.b[2479]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2366, 1.0, 1.0, A::mul(s.ad_value(220), s.ad_value(2365)));
        }

        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) && (!s.b[2479])) {
            s.store_offset_mul(2366, 220, 2365, 1.0);
        }

        if (s.b[2453] && (!s.b[2454])) {
            s.copy_ad(2301, 1806);
            s.copy_ad(2303, 1807);
            s.copy_ad(2319, 1808);
            s.copy_ad(2320, 1809);
            s.copy_ad(2304, 1810);
            s.copy_ad(2305, 1811);
            s.copy_ad(2321, 1812);
            s.copy_ad(2323, 1813);
            s.copy_ad(2328, 1814);
            s.copy_ad(2329, 1815);
            s.copy_ad(2340, 1816);
            s.copy_ad(2341, 1817);
            s.copy_ad(2342, 1818);
            s.copy_ad(2449, 1819);
            s.copy_ad(2344, 1820);
            s.copy_ad(2343, 1821);
            s.copy_ad(2346, 1822);
            s.copy_ad(2347, 1823);
            s.copy_ad(2348, 1824);
            s.copy_ad(2349, 1825);
            s.copy_ad(2351, 1826);
            s.copy_ad(2350, 1827);
            s.copy_ad(2352, 1828);
            s.copy_ad(2353, 1829);
            s.copy_ad(2354, 1830);
            s.copy_ad(2355, 1831);
            s.copy_ad(2356, 1832);
            s.copy_ad(2357, 1833);
            s.copy_ad(2358, 1834);
            s.copy_ad(2359, 1835);
            s.copy_ad(2363, 1836);
            s.copy_ad(2364, 1837);
            s.copy_ad(2366, 1838);
        }

        if s.b[2453] {
            s.copy_ad(2299, 1921);
            s.copy_ad(2300, 766);
        }

        s.b[2480] = (p.p48 != 0.0);
        s.v[2480] = if s.b[2480] { 1.0 } else { 0.0 };

        if (s.b[2453] && s.b[2480]) {
            s.copy_ad(2299, 1922);
            s.copy_ad(2300, 767);
        }

        if s.b[2453] {
            s.store_scalar(2368, 0.0);
            s.store_scale(2367, 2319, 4.60517018598809);
            s.copy_ad(2384, 2367);
            s.copy_ad(2385, 815);
            s.store_mul(2386, 815, 2320);
        }

    }

    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
    ) {
        if s.b[2453] {
            s.copy_ad(2390, 2343);
            s.store_scalar(2391, 0.0);
            s.store_scalar(2394, 0.0);
            s.copy_ad(2396, 2349);
            s.copy_ad(2397, 2351);
            s.copy_ad(2399, 2350);
            s.copy_ad(2400, 2357);
            s.copy_ad(2401, 2343);
            s.copy_ad(2402, 2349);
            s.copy_ad(2404, 2350);
            s.copy_ad(2405, 2351);
            s.store_sub(2406, 2323, 2343);
            s.store_scalar(2407, 1.0);
            s.store_scalar(2409, 1.0);
            s.store_scalar(2408, 0.0);
            s.copy_ad(2418, 2356);
            s.store_mul(2422, 2406, 2319);
            s.store_scalar(2419, 0.0);
            s.copy_ad(2420, 2357);
            s.store_scalar(2425, 0.0);
            s.store_scalar(2424, 1.0);
            s.copy_ad(2427, 2299);
            s.copy_ad(2426, 2422);
        }

        s.b[2481] = (s.v[2323] > 0.0);
        s.v[2481] = if s.b[2481] { 1.0 } else { 0.0 };

        s.b[2482] = (s.v[2350] > 1e-100);
        s.v[2482] = if s.b[2482] { 1.0 } else { 0.0 };

        if ((s.b[2453] && s.b[2481]) && s.b[2482]) {
            s.store_mul(2427, 2299, 2366);
            s.store_div(2368, 2427, 2363);
            s.store_add_scaled_inputs(2369, 2355, 1.0, 2305, 0.5);
            s.store_div_scaled_product_by_product(1929, s.ad_value(2305), s.ad_value(2348), 1.0, s.ad_value(2369), s.ad_value(2369), 1.0);
        }

        s.b[2483] = (s.v[1929] > 0.0001);
        s.v[2483] = if s.b[2483] { 1.0 } else { 0.0 };

        if (((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2483]) {
            s.store_sub_from_scalar(1930, 1.0, 1929);
        }

        s.b[2484] = (s.v[1930] < 1e-10);
        s.v[2484] = if s.b[2484] { 1.0 } else { 0.0 };

        if ((((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2483]) && s.b[2484]) {
            s.store_scalar(1931, 1.0);
        }

        if ((((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2483]) && (!s.b[2484])) {
            s.store_sub_from_scalar_ad(1931, 1.0, A::sqrt(s.ad_value(1930)));
        }

        if (((s.b[2453] && s.b[2481]) && s.b[2482]) && (!s.b[2483])) {
            s.store_scale(1931, 1929, 0.5);
        }

        if ((s.b[2453] && s.b[2481]) && s.b[2482]) {
            s.store_mul(2370, 1931, 2369);
        }

        s.b[2485] = ((s.v[707] > 0.0) && (s.v[708] > 0.0));
        s.v[2485] = if s.b[2485] { 1.0 } else { 0.0 };

        if (((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2485]) {
            s.store_scaled_mul(2371, 2319, 2370, 0.475);
            s.store_add_scaled_product_indices(1929, 2356, 1.0, 2353, 2371, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(2372, 1929, 1929, 1e-12, 0.5);
            s.store_add_scaled_value_products(2373, s.ad_value(2356), (-1.0), s.ad_value(2319), s.ad_value(2355), 1.0, A::offset(s.ad_value(2353), (-1.0)), s.ad_value(2371), 1.0);
            s.store_offset_div_scaled_product(2374, s.ad_value(2305), s.ad_value(2319), 0.5, s.ad_value(2373), 1.0, 1.0);
            s.store_add_scaled_product_indices(1929, 2373, 1.0, 764, 2372, 1.0);
            s.store_pow_ad(2375, A::mul3(s.ad_value(763), s.ad_value(1929), s.ad_value(705)), s.ad_value(706));
            s.store_mul_ad_lhs(1930, A::div_scaled_product_offset_rhs(s.ad_value(706), A::mul_sub_from_scalar_rhs(s.ad_value(2374), 1.0, s.ad_value(764)), (-1.0), 1.0, s.ad_value(1929), 1.0), 2375);
            s.store_div(1929, 2372, 2373);
            s.store_mul_pow_ad_rhs(2376, 707, A::offset(s.ad_value(1929), 1.0), A::neg(s.ad_value(708)));
            s.store_mul_div_scaled_product_rhs(1931, 2376, s.ad_value(708), A::add(A::offset(s.ad_value(2374), (-1.0)), A::div_scalar_offset_denominator(1.0, s.ad_value(1929), 1.0, 1.0)), 1.0, s.ad_value(2373), 1.0);
            s.store_mul_product3_rhs(2377, 2372, s.ad_value(746), s.ad_value(2358), s.ad_value(2359), 1.0);
            s.store_offset_ad(1929, A::div_scaled_add_product(s.ad_value(1930), 1.0, A::mul3(s.ad_value(746), s.ad_value(2358), s.ad_value(2359)), s.ad_value(2374), (-1.0), s.ad_value(1931), 1.0), 1.0);
        }

        s.b[2486] = (s.v[1929] < 230.25850929940458);
        s.v[2486] = if s.b[2486] { 1.0 } else { 0.0 };

        if ((((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2485]) && s.b[2486]) {
            s.store_scaled_ln_one_plus_exp_scaled_input(1930, 1929, 2.0, 0.5);
        }

        if ((((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2485]) && (!s.b[2486])) {
            s.copy_ad(1930, 1929);
        }

        if (((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2485]) {
            s.store_div_scaled_product3_mixed_iiia(2378, 2371, 1931, 1930, -1.0, A::add_scaled_inputs3_offset(s.ad_value(2375), 1.0, s.ad_value(2376), 1.0, s.ad_value(2377), 1.0, 1.0), 1.0);
            s.store_mul_offset_ad_rhs(2379, 2370, A::div_scaled_value_offset_denominator(s.ad_value(2378), 1.0, A::sqrt(A::offset(A::square(s.ad_value(2378)), 1.0)), 1.0, 1.0), 1.0);
        }

        if (((s.b[2453] && s.b[2481]) && s.b[2482]) && (!s.b[2485])) {
            s.copy_ad(2379, 2370);
        }

        if ((s.b[2453] && s.b[2481]) && s.b[2482]) {
            s.store_mul3_affine_lhs(2380, 2319, 2368, 0.7071067811865475, 0.0, 2379);
        }

        s.b[2487] = (s.v[0] == (-1.0));
        s.v[2487] = if s.b[2487] { 1.0 } else { 0.0 };

        if (((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2487]) {
            s.store_div_ad_rhs(2380, 2380, A::sqrt(A::offset(s.ad_value(2380), 1.0)));
        }

        if ((s.b[2453] && s.b[2481]) && s.b[2482]) {
            s.store_div_from_scalar_offset_ad(2381, 2.0, A::sqrt(A::scale_offset(s.ad_value(2380), 4.0, 1.0)), 1.0);
            s.store_mul(1929, 2381, 2380);
            s.store_mul_ad_product_rhs(2382, 2379, s.ad_value(2381), A::offset(A::div(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1929), 1.0, A::mul(s.ad_value(1929), s.ad_value(2381)), 0.86), A::offset(A::mul3_scaled_output(s.ad_value(1929), s.ad_value(1929), s.ad_value(2381), 4.0), 1.0)), 1.0));
            s.store_scale(2383, 2382, 0.99);
            s.store_div_scaled_product3_mixed_iaii(1929, 2383, A::sub_scaled_inputs(s.ad_value(2383), 1.0, s.ad_value(2369), 2.0), 2321, 1.0, 2350, 1.0);
        }

        if ((s.b[2453] && s.b[2481]) && s.b[2482]) {
            s.store_mul_sub_ad_rhs(2384, 2319, s.ad_value(2383), A::ln(A::offset({
                if (s.v[1929] > (-0.99)) {
                    s.ad_value(1929)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0)));
        }

        if ((s.b[2453] && s.b[2481]) && (!s.b[2482])) {
            s.copy_ad(2384, 2367);
        }

        if (s.b[2453] && s.b[2481]) {
            s.store_offset(1929, 2300, 1.0);
            s.store_div_scaled_product_left_ad(1930, A::sqrt(s.ad_value(1929)), 815, 1.0, 2384, 1.0);
            s.store_add_ad_lhs(1931, A::square(s.ad_value(1930)), 1929);
            s.store_scale(1929, 1930, 2.0);
            s.store_div_scaled_product_add_scaled_denominator(2385, 2384, 1929, 1.0, A::sqrt(A::sub(s.ad_value(1931), s.ad_value(1929))), 1.0, A::sqrt(A::add(s.ad_value(1931), s.ad_value(1929))), 1.0, 1.0);
            s.store_mul(2386, 2385, 2320);
            s.store_add(2387, 2329, 2386);
        }

        s.b[2488] = (s.v[2386] < 460.51701859880916);
        s.v[2488] = if s.b[2488] { 1.0 } else { 0.0 };

        if ((s.b[2453] && s.b[2481]) && s.b[2488]) {
            s.store_exp_neg_input(2388, 2386);
        }

        if ((s.b[2453] && s.b[2481]) && (!s.b[2488])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2388, 1e-200, 2386, (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2386), (-460.51701859880916), A::scale_offset(s.ad_value(2386), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if (s.b[2453] && s.b[2481]) {
            s.store_mul(2389, 2344, 2388);
        }

        s.b[2489] = (((s.v[2323]) as f64).abs() <= s.v[2341]);
        s.v[2489] = if s.b[2489] { 1.0 } else { 0.0 };

        if ((s.b[2453] && s.b[2481]) && s.b[2489]) {
            s.store_scaled_square(2429, 2342, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(2390, 2323, s.ad_value(2342), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2323), 1.0, s.ad_value(2389)), s.ad_value(2304), s.ad_value(2429)), 1.0));
        }

        if ((s.b[2453] && s.b[2481]) && (!s.b[2489])) {
            s.store_offset(2450, 2387, 3.0);
            s.store_sub_ad(2433, A::add_scaled_inputs3(s.ad_value(2449), 0.5, s.ad_value(2450), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2449), s.ad_value(2450)), A::sub(s.ad_value(2449), s.ad_value(2450))), 5.0)), (-0.5)), A::sub_scaled_inputs(s.ad_value(2450), 0.5, A::sqrt(A::offset(A::square(s.ad_value(2450)), 5.0)), 0.5));
            s.store_sub(2428, 2323, 2433);
            s.store_exp_neg_input(2429, 2433);
            s.store_div_from_scalar_offset_ad(2430, 1.0, A::square(s.ad_value(2433)), 2.0);
            s.store_mul_square_lhs(2440, 2433, 2430);
            s.store_mul3_affine_lhs(2441, 2433, 2430, 4.0, 0.0, 2430);
            s.store_mul_ad_product_lhs(2442, A::sub_scaled_inputs(s.ad_value(2430), 8.0, s.ad_value(2440), 12.0), s.ad_value(2430), 2430);
        }

        if ((s.b[2453] && s.b[2481]) && (!s.b[2489])) {
            if (1e-40 > ((s.v[2428] * s.v[2428]) - (s.v[2305] * (((s.v[2429] + s.v[2433]) - 1.0) - (s.v[2389] * ((s.v[2433] + 1.0) + s.v[2440])))))) {
                s.store_scalar(2434, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2434, 2428, 1.0, 2305, A::add_scaled_product(A::offset(A::add(s.ad_value(2429), s.ad_value(2433)), (-1.0)), 1.0, s.ad_value(2389), A::add(A::offset(s.ad_value(2433), 1.0), s.ad_value(2440)), (-1.0)), (-1.0));
            }
        }

        if ((s.b[2453] && s.b[2481]) && (!s.b[2489])) {
            s.store_sub_from_scalar_ad(2451, 1.0, A::mul_scaled_output(s.ad_value(2305), A::add_scaled_product(s.ad_value(2429), 1.0, s.ad_value(2389), s.ad_value(2442), (-1.0)), 0.5));
            s.store_add_scaled_product_right_ad(2435, 2428, 2.0, 2305, A::add_scaled_sub_value_product(1.0, s.ad_value(2429), 1.0, s.ad_value(2389), A::offset(s.ad_value(2441), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3(2436, s.ad_value(2387), 1.0, s.ad_value(2433), (-1.0), A::ln(A::div(s.ad_value(2434), s.ad_value(2305))), 1.0);
            s.store_add(813, 2434, 2435);
            s.store_add_scaled_square_product_mixed_iia(812, 813, 1.0, 2436, A::add_scaled_square_product(s.ad_value(2435), 0.5, s.ad_value(2434), s.ad_value(2451), (-1.0)), 1.0);
            s.store_add_ad_rhs(2452, 2433, A::div_scaled_product3(s.ad_value(2434), s.ad_value(813), s.ad_value(2436), 1.0, A::add(s.ad_value(812), A::mul3(A::mul3(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2436), s.ad_value(2436)), s.ad_value(2435), A::add_scaled_square_product(s.ad_value(2435), 0.3333333333333333, s.ad_value(2434), s.ad_value(2451), (-1.0)))), 1.0));
        }

        s.b[2490] = (s.v[2452] < 230.25850929940458);
        s.v[2490] = if s.b[2490] { 1.0 } else { 0.0 };

        if (((s.b[2453] && s.b[2481]) && (!s.b[2489])) && s.b[2490]) {
            s.store_exp(2438, 2452);
            s.store_div_from_scalar(2439, 1.0, 2438);
            s.store_mul(2438, 2389, 2438);
        }

        s.b[2491] = (s.v[2452] > (s.v[2387] - 230.25850929940458));
        s.v[2491] = if s.b[2491] { 1.0 } else { 0.0 };

        if ((((s.b[2453] && s.b[2481]) && (!s.b[2489])) && (!s.b[2490])) && s.b[2491]) {
            s.store_exp_sub(2438, 2452, 2387);
            s.store_div(2439, 2389, 2438);
        }

        if ((((s.b[2453] && s.b[2481]) && (!s.b[2489])) && (!s.b[2490])) && (!s.b[2491])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(2438, 1e-100, A::sub(s.ad_value(2387), s.ad_value(2452)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2387), s.ad_value(2452)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2387), s.ad_value(2452)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2439, 1e-100, 2452, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2452), (-230.25850929940458), A::scale_offset(s.ad_value(2452), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if ((s.b[2453] && s.b[2481]) && (!s.b[2489])) {
            s.store_div_from_scalar_offset_ad(2428, 1.0, A::square(s.ad_value(2452)), 2.0);
            s.store_mul_square_lhs(2440, 2452, 2428);
            s.store_mul3_affine_lhs(2441, 2452, 2428, 4.0, 0.0, 2428);
            s.store_mul_ad_product_lhs(2442, A::sub_scaled_inputs(s.ad_value(2428), 8.0, s.ad_value(2440), 12.0), s.ad_value(2428), 2428);
            s.store_sub(2428, 2323, 2452);
            s.store_add_scaled_product_right_ad(2443, 2428, 2.0, 2305, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2439)), 1.0, s.ad_value(2438), 1.0, s.ad_value(2389), A::offset(s.ad_value(2441), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2444, 2428, 1.0, 2305, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2439), 1.0, s.ad_value(2452), 1.0, s.ad_value(2438), 1.0, (-1.0)), 1.0, s.ad_value(2389), A::add(A::offset(s.ad_value(2452), 1.0), s.ad_value(2440)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_ad(2428, 2.0, A::mul(s.ad_value(2305), A::add_scaled_inputs_product(s.ad_value(2439), 1.0, s.ad_value(2438), 1.0, s.ad_value(2389), s.ad_value(2442), (-1.0))));
            s.store_add_scaled_square_product_indices(2428, 2443, 1.0, 2444, 2428, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(2390, 2452, 1.0, A::div(s.ad_value(2444), A::add(s.ad_value(2443), A::sqrt(s.ad_value(2428)))), 2.0);
        }

        if (s.b[2453] && s.b[2481]) {
            s.store_sub(2391, 2390, 2343);
        }

        s.b[2492] = (s.v[2391] < 1e-10);
        s.v[2492] = if s.b[2492] { 1.0 } else { 0.0 };

        if ((s.b[2453] && s.b[2481]) && s.b[2492]) {
            s.store_add_scaled_inputs_product_right_ad(2392, 2323, 2.0, 2343, (-2.0), 2305, A::add_scaled_offset_product_rhs(A::add_scaled_sub_value_product(1.0, s.ad_value(2349), 1.0, s.ad_value(2348), s.ad_value(2388), 1.0), 1.0, s.ad_value(2389), s.ad_value(2346), 1.0, (-1.0)), 1.0);
            s.store_mul_ad_lhs(2393, A::mul_sub_from_scalar_rhs(s.ad_value(2305), 1.0, s.ad_value(2388)), 2350);
            s.store_sub_from_scalar_ad(1929, 2.0, A::mul(s.ad_value(2305), A::add_scaled_value_products(s.ad_value(2349), 1.0, s.ad_value(2348), s.ad_value(2388), 1.0, s.ad_value(2389), s.ad_value(2347), (-1.0))));
            s.store_add_scaled_square_product_indices(1929, 2392, 1.0, 1929, 2393, (-2.0));
            s.store_scaled_div_ad_rhs(2391, 2393, A::add(s.ad_value(2392), A::sqrt(s.ad_value(1929))), 2.0);
            s.store_add(2390, 2343, 2391);
        }

        if (s.b[2453] && s.b[2481]) {
            s.store_mul(2394, 2391, 2319);
            s.store_div_scaled_product_offset_denominator(2395, s.ad_value(2390), s.ad_value(2390), 1.0, A::square(s.ad_value(2390)), 2.0, 1.0);
        }

        s.b[2493] = (s.v[2390] < 230.25850929940458);
        s.v[2493] = if s.b[2493] { 1.0 } else { 0.0 };

        if ((s.b[2453] && s.b[2481]) && s.b[2493]) {
            s.store_exp_neg_input(2396, 2390);
        }

        s.b[2494] = (s.v[2390] < 1e-5);
        s.v[2494] = if s.b[2494] { 1.0 } else { 0.0 };

        if (((s.b[2453] && s.b[2481]) && s.b[2493]) && s.b[2494]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2397, 2390, 1.0, 2390, 1.0, 2390, 0.25, 0.3333333333333333, 0.5);
        }

    }

    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[2453] && s.b[2481]) && s.b[2493]) && s.b[2494]) {
            s.store_sqrt_sub_from_scalar_ad(1929, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2390), 1.0, A::scale(s.ad_value(2390), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2398, 2390, 1929, 0.7071067811865475);
            s.store_mul3_ad_middle(2399, A::mul3_scaled_output(s.ad_value(2389), s.ad_value(2390), s.ad_value(2390), 0.16666666666666666), 2390, A::scale_offset(s.ad_value(2390), 1.75, 1.0));
        }

        if (((s.b[2453] && s.b[2481]) && s.b[2493]) && (!s.b[2494])) {
            s.store_add_ad_lhs(2397, A::offset(s.ad_value(2390), (-1.0)), 2396);
            s.store_sqrt(2398, 2397);
            s.store_mul_add_scaled_inputs3_offset_rhs(2399, 2389, A::div_from_scalar(1.0, s.ad_value(2396)), 1.0, s.ad_value(2390), (-1.0), s.ad_value(2395), -1.0, (-1.0));
        }

        s.b[2495] = (s.v[2390] > (s.v[2387] - 230.25850929940458));
        s.v[2495] = if s.b[2495] { 1.0 } else { 0.0 };

        if (((s.b[2453] && s.b[2481]) && (!s.b[2493])) && s.b[2495]) {
            s.store_exp_sub(1929, 2390, 2387);
            s.store_div(2396, 2389, 1929);
            s.store_add_scaled_product_right_ad(2399, 1929, 1.0, 2389, A::add(A::offset(s.ad_value(2390), 1.0), s.ad_value(2395)), (-1.0));
        }

        if (((s.b[2453] && s.b[2481]) && (!s.b[2493])) && (!s.b[2495])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2396, 1e-100, 2390, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2390), (-230.25850929940458), A::scale_offset(s.ad_value(2390), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(1929, 1e-100, A::sub(s.ad_value(2387), s.ad_value(2390)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2387), s.ad_value(2390)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2387), s.ad_value(2390)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_add_scaled_product_right_ad(2399, 1929, 1.0, 2389, A::add(A::offset(s.ad_value(2390), 1.0), s.ad_value(2395)), (-1.0));
        }

        if ((s.b[2453] && s.b[2481]) && (!s.b[2493])) {
            s.store_add_ad_lhs(2397, A::offset(s.ad_value(2390), (-1.0)), 2396);
            s.store_sqrt(2398, 2397);
        }

        if (s.b[2453] && s.b[2481]) {
            s.store_mul3_lhs(2400, 2398, 2304, 2319);
            s.store_scaled_add(2401, 2343, 2390, 0.5);
            s.store_scalar(2402, 0.0);
            s.store_mul(1929, 2396, 2349);
        }

        s.b[2496] = (s.v[1929] > 0.0);
        s.v[2496] = if s.b[2496] { 1.0 } else { 0.0 };

        if ((s.b[2453] && s.b[2481]) && s.b[2496]) {
            s.store_sqrt(2402, 1929);
        }

        if (s.b[2453] && s.b[2481]) {
            s.store_scaled_add(2403, 2350, 2399, 0.5);
            s.store_add_scaled_product_mixed_iaa(2404, 2403, 1.0, A::square(s.ad_value(2391)), A::sub_scaled_inputs(s.ad_value(2402), 1.0, s.ad_value(2321), 2.0), 0.125);
        }

        s.b[2497] = (s.v[2401] < 1e-5);
        s.v[2497] = if s.b[2497] { 1.0 } else { 0.0 };

        if ((s.b[2453] && s.b[2481]) && s.b[2497]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2405, 2401, 1.0, 2401, 1.0, 2401, 0.25, 0.3333333333333333, 0.5);
            s.store_mul_sqrt_ad_rhs(2406, 2304, A::add(s.ad_value(2404), s.ad_value(2405)));
        }

        s.b[2498] = (s.v[719] > 0.0);
        s.v[2498] = if s.b[2498] { 1.0 } else { 0.0 };

        if (((s.b[2453] && s.b[2481]) && s.b[2497]) && s.b[2498]) {
            s.store_div_from_scalar_sqrt_ad(2407, 1.0, A::offset(A::mul(s.ad_value(719), s.ad_value(2406)), 1.0));
        }

        if ((s.b[2453] && s.b[2481]) && s.b[2497]) {
            s.store_sqrt_sub_from_scalar_ad(1929, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2401), 1.0, A::scale(s.ad_value(2401), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2408, 2401, 1929, 0.7071067811865475);
            s.store_add_ad_rhs(2409, 2407, A::div_scaled_product(s.ad_value(2304), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2401), 0.5)), 1.0, A::square(s.ad_value(2401)), 0.16666666666666666), 0.7071067811865475, s.ad_value(1929), 1.0));
        }

        if ((s.b[2453] && s.b[2481]) && (!s.b[2497])) {
            s.store_add_ad_lhs(2405, A::offset(s.ad_value(2401), (-1.0)), 2402);
            s.store_mul_sqrt_ad_rhs(2406, 2304, A::add(s.ad_value(2404), s.ad_value(2405)));
        }

        s.b[2499] = (s.v[719] > 0.0);
        s.v[2499] = if s.b[2499] { 1.0 } else { 0.0 };

        if (((s.b[2453] && s.b[2481]) && (!s.b[2497])) && s.b[2499]) {
            s.store_add_scaled_sub_value_product_indices(2410, 1.0, 2402, 1.0, 2406, 2321, 2.0);
            s.store_div_from_scalar_sqrt_ad(2407, 1.0, A::offset(A::mul(s.ad_value(719), s.ad_value(2406)), 1.0));
            s.store_div_scaled_value_offset_denominator(1929, s.ad_value(2407), 1.0, s.ad_value(2407), 1.0, 1.0);
            s.store_mul_product3_rhs(2411, 719, A::square(s.ad_value(1929)), s.ad_value(2305), s.ad_value(2404), 1.0);
            s.store_add_scaled_inputs_product_right_ad(2412, 2406, 2.0, 2411, (-2.0), 2305, A::add(A::sub_from_scalar(1.0, s.ad_value(2402)), s.ad_value(2404)), 1.0);
            s.store_mul_sub_scaled_inputs_rhs(2413, 2411, s.ad_value(2411), 1.0, s.ad_value(2406), 2.0);
            s.store_sub_from_scalar_ad(2414, 1.0, A::mul_scaled_output(s.ad_value(2305), A::add(s.ad_value(2402), s.ad_value(2404)), 0.5));
            s.store_div_scaled_product_denominator_ad(2415, 2413, 2412, 1.0, A::add_scaled_square_product(s.ad_value(2412), 1.0, s.ad_value(2414), s.ad_value(2413), (-1.0)), 1.0);
            s.store_add(2401, 2401, 2415);
            s.store_exp(2416, 2415);
            s.store_div(2402, 2402, 2416);
            s.store_mul(2404, 2404, 2416);
            s.store_add_ad_lhs(2405, A::offset(s.ad_value(2401), (-1.0)), 2402);
            s.store_mul_sqrt_ad_rhs(2406, 2304, A::add(s.ad_value(2404), s.ad_value(2405)));
            s.store_add_ad(2417, A::sub_from_scalar(1.0, s.ad_value(2402)), A::mul3_scaled_output(s.ad_value(2406), s.ad_value(2407), s.ad_value(2321), 2.0));
            s.store_div_scaled_product3_mixed_iiaa(2391, 2391, 2416, A::add(s.ad_value(2410), s.ad_value(2403)), 1.0, A::add_scaled_product(s.ad_value(2417), 1.0, s.ad_value(2416), s.ad_value(2403), 1.0), 1.0);
            s.store_mul(2394, 2391, 2319);
        }

        if ((s.b[2453] && s.b[2481]) && (!s.b[2497])) {
            s.store_sqrt(2408, 2405);
            s.store_add_scaled_inputs_ad_rhs(2409, 2407, 1.0, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2304), 1.0, s.ad_value(2402)), s.ad_value(2408)), 0.5);
        }

        if (s.b[2453] && s.b[2481]) {
            s.store_mul_div_scaled_product_rhs(2418, 2319, s.ad_value(2305), s.ad_value(2404), 1.0, A::add_scaled_product(s.ad_value(2406), 1.0, s.ad_value(2304), s.ad_value(2408), 1.0), 1.0);
            s.store_add_scaled_product_indices(2419, 2418, 1.0, 2319, 2409, 1.0);
            s.store_mul3_lhs(2420, 2408, 2304, 2319);
        }

        s.b[2500] = (s.v[216] < 0.0);
        s.v[2500] = if s.b[2500] { 1.0 } else { 0.0 };

        if ((s.b[2453] && s.b[2481]) && s.b[2500]) {
            s.store_sub_from_scalar_ad(2359, 1.0, A::mul(s.ad_value(216), s.ad_value(2418)));
        }

        if ((s.b[2453] && s.b[2481]) && (!s.b[2500])) {
            s.store_div_from_scalar_offset_ad(2359, 1.0, A::mul(s.ad_value(216), s.ad_value(2418)), 1.0);
        }

        if (s.b[2453] && s.b[2481]) {
            s.store_mul_product3_rhs(2360, 2418, s.ad_value(746), s.ad_value(2358), s.ad_value(2359), 1.0);
            s.store_add_scaled_product_indices(2421, 2420, 1.0, 764, 2418, 1.0);
            s.store_add_scaled_product_indices(2422, 2420, 1.0, 765, 2418, 1.0);
            s.store_mul(2423, 763, 2421);
            s.store_ln_ad(1930, A::div_scaled_value_offset_denominator(s.ad_value(2405), 1.0, A::add(s.ad_value(2405), s.ad_value(2404)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2362, A::pow(A::mul(s.ad_value(2423), s.ad_value(705)), s.ad_value(706)), 1.0, 707, A::exp(A::mul_scaled_lhs(s.ad_value(708), 0.5, s.ad_value(1930))), 1.0);
            s.store_mul_add_ad_lhs(2424, A::offset(s.ad_value(2362), 1.0), s.ad_value(2360), 2354);
            s.store_ln_ad(2425, A::div_scaled_offset_numerator(A::mul(A::sub(s.ad_value(815), s.ad_value(2394)), s.ad_value(768)), 1.0, 1.0, A::offset(A::mul(A::sub(s.ad_value(2385), s.ad_value(2394)), s.ad_value(768)), 1.0), 1.0));
            s.store_mul(1931, 2418, 2364);
            s.store_div_add_scaled_inputs_rhs_indices(2365, 1931, 221, 1.0, 1931, 1.0);
        }

        s.b[2501] = (s.v[220] < 0.0);
        s.v[2501] = if s.b[2501] { 1.0 } else { 0.0 };

        if ((s.b[2453] && s.b[2481]) && s.b[2501]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2366, 1.0, 1.0, A::mul(s.ad_value(220), s.ad_value(2365)));
        }

        if ((s.b[2453] && s.b[2481]) && (!s.b[2501])) {
            s.store_offset_mul(2366, 220, 2365, 1.0);
        }

        if (s.b[2453] && s.b[2481]) {
            s.store_mul(2427, 2299, 2366);
            s.store_mul(2426, 2406, 2319);
        }

        if s.b[2453] {
            s.copy_ad(1871, 2301);
            s.copy_ad(1872, 2319);
            s.copy_ad(1873, 2304);
            s.copy_ad(1874, 2323);
            s.copy_ad(1875, 2328);
            s.copy_ad(1876, 2357);
            s.copy_ad(1877, 2394);
            s.copy_ad(1878, 2400);
            s.copy_ad(1879, 2407);
            s.copy_ad(1880, 2409);
            s.copy_ad(1881, 2418);
            s.copy_ad(1882, 2419);
            s.copy_ad(1883, 2422);
            s.copy_ad(1884, 2424);
            s.copy_ad(1885, 2425);
            s.copy_ad(1886, 2427);
            s.copy_ad(1887, 2426);
        }

        if (!s.b[2453]) {
            s.copy_ad(734, 717);
            s.copy_ad(1871, 1806);
            s.copy_ad(1872, 1808);
            s.copy_ad(1873, 1810);
            s.copy_ad(1874, 1813);
            s.copy_ad(1875, 1814);
            s.copy_ad(1876, 1833);
            s.copy_ad(1877, 1844);
            s.copy_ad(1878, 1845);
            s.copy_ad(1879, 1847);
            s.copy_ad(1880, 1848);
            s.copy_ad(1881, 1849);
            s.copy_ad(1882, 1850);
            s.copy_ad(1883, 1852);
            s.copy_ad(1884, 1853);
            s.copy_ad(1885, 1855);
            s.copy_ad(1886, 1854);
            s.copy_ad(1887, 1856);
        }

        s.copy_ad(1888, 253);

        s.b[2502] = (s.v[762] > 0.0);
        s.v[2502] = if s.b[2502] { 1.0 } else { 0.0 };

        if s.b[2502] {
            s.store_div_scaled_value_offset_denominator(1888, s.ad_value(253), 1.0, A::mul(s.ad_value(762), A::powf(A::add(A::square(s.ad_value(1883)), s.ad_value(722)), ((-1.0) * 0.16666666666666666))), 1.0, 1.0);
        }

        s.v[1889] = 1.0;

        s.v[1890] = 1.0;

        s.v[1891] = 0.0;

        s.v[1892] = 1.0;

        s.v[1893] = 1.0;

        s.copy_ad(2265, 1887);

        s.v[2268] = 0.0;

        s.v[2267] = 0.0;

        s.copy_ad(2269, 2265);

        s.b[2503] = (s.v[1874] > 0.0);
        s.v[2503] = if s.b[2503] { 1.0 } else { 0.0 };

        if s.b[2503] {
            s.store_mul_div_scaled_product_rhs(2260, 1885, A::add(s.ad_value(258), A::div(s.ad_value(259), s.ad_value(1882))), s.ad_value(1881), 1.0, s.ad_value(1882), 1.0);
        }

        s.b[2504] = (s.v[2260] > 0.0);
        s.v[2504] = if s.b[2504] { 1.0 } else { 0.0 };

        if (s.b[2503] && s.b[2504]) {
            s.store_div_from_scalar_add_ad(1889, 1.0, A::offset(s.ad_value(2260), 1.0), A::square(s.ad_value(2260)));
        }

        if (s.b[2503] && (!s.b[2504])) {
            s.store_sub_from_scalar(1889, 1.0, 2260);
        }

        if s.b[2503] {
            s.store_mul(1890, 1884, 1889);
            s.store_div(1891, 1886, 1890);
            s.store_mul_ad_product_lhs(2261, A::square(s.ad_value(1891)), s.ad_value(1877), 1877);
        }

        s.b[2505] = (s.v[0] == (-1.0));
        s.v[2505] = if s.b[2505] { 1.0 } else { 0.0 };

        if (s.b[2503] && s.b[2505]) {
            s.store_div_scaled_value_offset_denominator(2261, s.ad_value(2261), 1.0, A::mul(s.ad_value(1891), s.ad_value(1877)), 1.0, 1.0);
        }

        if s.b[2503] {
            s.store_mul_offset_rhs_scaled_ad_rhs(1892, 1890, A::sqrt(A::scale_offset(s.ad_value(2261), 2.0, 1.0)), 1.0, 0.5);
            s.store_div(1929, 1890, 1892);
            s.store_mul_offset_ad_rhs(2262, 1880, A::mul3_scaled_output(s.ad_value(2261), s.ad_value(1929), s.ad_value(1929), 0.5), 1.0);
            s.store_div_scaled_product_indices(1893, 1929, 1882, 1.0, 2262, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[2503] {
            s.store_scaled_div(2263, 1877, 1893, 0.5);
            s.store_square(2264, 2263);
            s.store_add_ad_rhs(2265, 1887, A::mul3_scaled_output(s.ad_value(1879), s.ad_value(1877), A::add(A::offset(A::mul_scaled_output(s.ad_value(2263), s.ad_value(1889), 0.3333333333333333), (-1.0)), s.ad_value(1889)), 0.5));
            s.store_scaled_mul(1929, 1880, 1877, 0.16666666666666666);
        }

        s.b[2506] = (p.p49 == 1.0);
        s.v[2506] = if s.b[2506] { 1.0 } else { 0.0 };

        if (s.b[2503] && s.b[2506]) {
            s.store_scalar(2266, 0.0);
            s.store_mul_ad_affine_product_rhs(2267, 1889, s.ad_value(1889), A::sub(s.ad_value(1881), A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1929), 2.0, s.ad_value(2263), 3.0)), 0.5, 0.0);
        }

        if (s.b[2503] && (!s.b[2506])) {
            s.store_mul_sub_from_scalar_lhs_ad_rhs(2266, 1.0, 1889, A::add_scaled_product(s.ad_value(1881), 1.0, s.ad_value(1880), s.ad_value(1877), (-0.5)));
            s.store_add_scaled_products_mixed_aaia(2267, A::square(s.ad_value(1889)), A::add_scaled_product(s.ad_value(1881), 1.0, s.ad_value(1929), A::sub_scaled_inputs(A::sub_from_scalar(1.0, s.ad_value(2263)), 1.0, s.ad_value(2264), 0.2), (-1.0)), 0.5, 2266, A::offset(s.ad_value(1889), 1.0), 0.5);
        }

        if s.b[2503] {
            s.store_add_scaled_product_right_ad(2268, 2266, 1.0, 1889, A::add_scaled_product(s.ad_value(1881), 1.0, s.ad_value(1929), s.ad_value(2263), 1.0), 1.0);
            s.store_sub(2269, 2265, 2268);
        }

        s.store_mul(840, 2265, 1888);

        s.store_mul_neg_lhs(842, 2267, 1888);

        s.store_mul_neg_lhs(841, 2269, 1888);

        s.v[2285] = 0.0;

        s.v[2286] = 0.0;

        s.v[2284] = 0.0;

        s.b[2507] = ((s.v[266] > 0.0) || (s.v[267] > 0.0));
        s.v[2507] = if s.b[2507] { 1.0 } else { 0.0 };

        if s.b[2507] {
            s.store_scalar(2274, 1.0);
            s.copy_ad(2273, 1871);
        }

        s.b[2508] = (s.v[270] > 1e-10);
        s.v[2508] = if s.b[2508] { 1.0 } else { 0.0 };

        if (s.b[2507] && s.b[2508]) {
            s.store_add_scaled_inputs3(2270, s.ad_value(1871), 1.0, s.ad_value(268), (-1.0), s.ad_value(797), 1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1929, 2270, 0.5, 797, 0.5, A::add_scaled_product(s.ad_value(798), 1.0, A::sub(s.ad_value(2270), s.ad_value(797)), A::sub(s.ad_value(2270), s.ad_value(797)), 1.0), 0.5);
            s.store_mul_add_scaled_inputs3_offset_rhs(1930, 1929, s.ad_value(1929), 2.0, s.ad_value(797), (-1.0), s.ad_value(2270), -1.0, 0.0);
            s.store_div(1931, 797, 1929);
            s.store_mul(2271, 2270, 1931);
            s.store_sqrt_sub_from_scalar_ad(2272, 1.0, A::mul(s.ad_value(2271), s.ad_value(270)));
            s.store_add_scaled_inputs3(2273, A::div(A::sub_from_scalar(1.0, s.ad_value(2272)), s.ad_value(270)), 1.0, s.ad_value(2270), 1.0, s.ad_value(2271), -1.0);
            s.store_offset_ad(2274, A::div_scaled_product3(A::offset(A::div_from_scalar(0.5, s.ad_value(2272)), (-1.0)), A::add_scaled_product(s.ad_value(1930), 1.0, s.ad_value(2270), A::sub(s.ad_value(797), s.ad_value(1929)), 1.0), s.ad_value(1931), 1.0, s.ad_value(1930), 1.0), 1.0);
        }

        if s.b[2507] {
            s.store_scalar(2276, 1.0);
            s.store_scalar(2277, 0.0);
        }

        s.b[2509] = (s.v[269] > 0.0);
        s.v[2509] = if s.b[2509] { 1.0 } else { 0.0 };

        if (s.b[2507] && s.b[2509]) {
            s.store_add_scaled_product_right_ad(1929, 734, 0.5, 1872, A::scale_offset(s.ad_value(1873), 0.7071067811865475, 1.0), 1.0);
            s.store_div(2275, 1871, 1929);
        }

        s.b[2510] = (((s.v[2275]) as f64).abs() < 230.25850929940458);
        s.v[2510] = if s.b[2510] { 1.0 } else { 0.0 };

        if ((s.b[2507] && s.b[2509]) && s.b[2510]) {
            s.store_div_from_scalar_offset_ad(2276, 1.0, A::exp_scaled_input(s.ad_value(2275), -1.0), 1.0);
        }

        s.b[2511] = (s.v[2275] < 0.0);
        s.v[2511] = if s.b[2511] { 1.0 } else { 0.0 };

        if (((s.b[2507] && s.b[2509]) && (!s.b[2510])) && s.b[2511]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2276, 1e-100, 2275, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2275), (-230.25850929940458), A::scale_offset(s.ad_value(2275), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        s.b[2512] = (s.v[2275] < 230.25850929940458);
        s.v[2512] = if s.b[2512] { 1.0 } else { 0.0 };

        if ((s.b[2507] && s.b[2509]) && s.b[2512]) {
            s.store_ln_one_plus_exp(1930, 2275);
        }

        if ((s.b[2507] && s.b[2509]) && (!s.b[2512])) {
            s.copy_ad(1930, 2275);
        }

        if (s.b[2507] && s.b[2509]) {
            s.store_mul(2277, 1929, 1930);
        }

        if s.b[2507] {
            s.store_add_scaled_product_right_ad(2278, 2274, 1.0, 269, A::sub(s.ad_value(2276), s.ad_value(2274)), 1.0);
            s.store_add_scaled_product_right_ad(2279, 2273, 1.0, 269, A::sub(s.ad_value(2277), s.ad_value(2273)), 1.0);
            s.store_add_scaled_inputs3(2280, A::add_scaled_product(s.ad_value(1871), 1.0, s.ad_value(1872), s.ad_value(1875), (-1.0)), 1.0, s.ad_value(1887), (-1.0), s.ad_value(1877), (-0.5));
            s.store_add_scaled_inputs3(2281, s.ad_value(1871), 1.0, s.ad_value(2280), (-1.0), s.ad_value(1876), -1.0);
            s.store_add_scaled_inputs3(2282, s.ad_value(1877), 1.0, s.ad_value(2280), 1.0, s.ad_value(815), -1.0);
            s.store_add_scaled_inputs3(2283, s.ad_value(1871), 1.0, s.ad_value(2282), (-1.0), s.ad_value(1878), -1.0);
        }

        s.b[2513] = (s.v[820] > 0.0);
        s.v[2513] = if s.b[2513] { 1.0 } else { 0.0 };

        if (s.b[2507] && s.b[2513]) {
            s.store_mul_ad_rhs(2284, 2278, A::add_scaled_products(s.ad_value(267), s.ad_value(2282), 1.0, s.ad_value(266), s.ad_value(2280), 1.0));
            s.store_mul_sub_rhs(2285, 266, 2281, 2279);
            s.store_mul_sub_rhs(2286, 267, 2283, 2279);
        }

        if (s.b[2507] && (!s.b[2513])) {
            s.store_mul_ad_rhs(2284, 2278, A::add_scaled_products(s.ad_value(266), s.ad_value(2282), 1.0, s.ad_value(267), s.ad_value(2280), 1.0));
            s.store_mul_sub_rhs(2285, 267, 2281, 2279);
            s.store_mul_sub_rhs(2286, 266, 2283, 2279);
        }

        if s.b[2507] {
            s.store_add(840, 840, 2284);
            s.store_add(842, 842, 2286);
            s.store_add_scaled_inputs4(841, s.ad_value(841), 1.0, s.ad_value(2284), (-1.0), s.ad_value(2286), -1.0, s.ad_value(2285), -1.0);
        }

        s.store_mul(1894, 260, 1862);

        s.store_mul(1895, 261, 1863);

        s.v[2289] = 0.0;

        s.v[2287] = 0.0;

        s.b[2514] = ((s.v[260] > 0.0) && (s.v[262] > 0.0));
        s.v[2514] = if s.b[2514] { 1.0 } else { 0.0 };

        if s.b[2514] {
            s.store_mul_add_scaled_inputs_rhs(1929, 264, s.ad_value(1803), 0.5, s.ad_value(776), 1.0);
        }

        s.b[2515] = (s.v[1929] < 230.25850929940458);
        s.v[2515] = if s.b[2515] { 1.0 } else { 0.0 };

        s.b[2516] = (s.v[1929] > (-230.25850929940458));
        s.v[2516] = if s.b[2516] { 1.0 } else { 0.0 };

        if ((s.b[2514] && s.b[2515]) && s.b[2516]) {
            s.store_exp(2287, 1929);
        }

        if ((s.b[2514] && s.b[2515]) && (!s.b[2516])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2287, 1e-100, (-230.25850929940458), 1929, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2517] = (s.v[2287] > 1e-10);
        s.v[2517] = if s.b[2517] { 1.0 } else { 0.0 };

        if ((s.b[2514] && s.b[2515]) && s.b[2517]) {
            s.store_ln_offset_input(2288, 2287, 1.0);
            s.store_mul_sub_from_scalar_ad_rhs(1930, 2288, 1.0, A::div(A::ln(A::offset(s.ad_value(2288), 1.0)), A::offset(s.ad_value(2288), 2.0)));
        }

        if ((s.b[2514] && s.b[2515]) && (!s.b[2517])) {
            s.copy_ad(2288, 2287);
            s.store_div_scaled_value_offset_denominator(1930, s.ad_value(2288), 2.0, s.ad_value(2288), 2.0, 1.0);
        }

        if (s.b[2514] && (!s.b[2515])) {
            s.copy_ad(2288, 1929);
            s.store_mul_sub_from_scalar_ad_rhs(1930, 2288, 1.0, A::div(A::ln(A::offset(s.ad_value(2288), 1.0)), A::offset(s.ad_value(2288), 2.0)));
        }

        if s.b[2514] {
            s.store_mul_ad_affine_product_lhs(2289, A::div_scaled_inputs(s.ad_value(262), (-2.0), s.ad_value(264), 1.0), s.ad_value(260), s.v[355], 0.0, 1930);
        }

        s.v[2292] = 0.0;

        s.v[2290] = 0.0;

        s.b[2518] = ((s.v[261] > 0.0) && (s.v[263] > 0.0));
        s.v[2518] = if s.b[2518] { 1.0 } else { 0.0 };

        if s.b[2518] {
            s.store_mul_add_scaled_inputs_rhs(1929, 264, s.ad_value(1803), 0.5, s.ad_value(777), 1.0);
        }

        s.b[2519] = (s.v[1929] < 230.25850929940458);
        s.v[2519] = if s.b[2519] { 1.0 } else { 0.0 };

        s.b[2520] = (s.v[1929] > (-230.25850929940458));
        s.v[2520] = if s.b[2520] { 1.0 } else { 0.0 };

        if ((s.b[2518] && s.b[2519]) && s.b[2520]) {
            s.store_exp(2290, 1929);
        }

        if ((s.b[2518] && s.b[2519]) && (!s.b[2520])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2290, 1e-100, (-230.25850929940458), 1929, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2521] = (s.v[2290] > 1e-10);
        s.v[2521] = if s.b[2521] { 1.0 } else { 0.0 };

        if ((s.b[2518] && s.b[2519]) && s.b[2521]) {
            s.store_ln_offset_input(2291, 2290, 1.0);
            s.store_mul_sub_from_scalar_ad_rhs(1930, 2291, 1.0, A::div(A::ln(A::offset(s.ad_value(2291), 1.0)), A::offset(s.ad_value(2291), 2.0)));
        }

        if ((s.b[2518] && s.b[2519]) && (!s.b[2521])) {
            s.copy_ad(2291, 2290);
            s.store_div_scaled_value_offset_denominator(1930, s.ad_value(2291), 2.0, s.ad_value(2291), 2.0, 1.0);
        }

        if (s.b[2518] && (!s.b[2519])) {
            s.copy_ad(2291, 1929);
            s.store_mul_sub_from_scalar_ad_rhs(1930, 2291, 1.0, A::div(A::ln(A::offset(s.ad_value(2291), 1.0)), A::offset(s.ad_value(2291), 2.0)));
        }

        if s.b[2518] {
            s.store_mul_ad_affine_product_lhs(2292, A::div_scaled_inputs(s.ad_value(263), (-2.0), s.ad_value(264), 1.0), s.ad_value(261), s.v[355], 0.0, 1930);
        }

        s.store_add(2293, 2289, 2292);

        s.store_add_scaled_product_indices(845, 2293, 1.0, 265, 818, 1.0);

        s.store_mul(843, 272, 823);

        s.store_mul(844, 273, 826);

        s.v[2522] = 0.0;

        s.v[2525] = 0.0;

        s.v[2526] = 0.0;

        s.v[2527] = 0.0;

        s.v[2528] = 0.0;

        s.v[2529] = 0.0;

        s.v[2530] = 0.0;

        s.v[2531] = 0.0;

        s.v[2532] = 0.0;

        s.v[2533] = 0.0;

        s.v[2534] = 0.0;

        s.v[2535] = 0.0;

        s.v[2536] = 0.0;

        s.v[2537] = 0.0;

        s.v[2538] = 0.0;

        s.v[2539] = 0.0;

        s.v[2540] = 0.0;

        s.v[2543] = 0.0;

        s.v[2547] = 0.0;

        s.v[2550] = 0.0;

        s.v[2551] = 0.0;

        s.v[2552] = 0.0;

        s.v[2553] = 0.0;

        s.v[2554] = 0.0;

        s.v[2555] = 0.0;

        s.v[2558] = 0.0;

        s.v[2559] = 0.0;

        s.v[2560] = 0.0;

        s.v[2561] = 0.0;

        s.v[2565] = 0.0;

        s.v[2567] = 0.0;

        s.v[2568] = 0.0;

        s.v[846] = 0.0;

        s.v[1902] = 0.0;

        s.v[1903] = 0.0;

        s.v[1904] = 0.0;

        s.v[847] = 0.0;

        s.v[1905] = 0.0;

        s.v[1906] = 0.0;

        s.v[1907] = 0.0;

        s.b[2569] = (p.p43 > 0.0);
        s.v[2569] = if s.b[2569] { 1.0 } else { 0.0 };

        s.b[2570] = (s.v[475] == 1.0);
        s.v[2570] = if s.b[2570] { 1.0 } else { 0.0 };

        if (s.b[2569] && s.b[2570]) {
            s.store_scalar(2573, 0.0);
            s.store_scalar(2574, 0.0);
            s.store_scaled_mul(2525, 658, 658, 4.0);
            s.store_div(2526, 658, 659);
            s.store_add_scaled_product_indices(2527, 821, 1.0, 658, 2526, 1.0);
            s.store_add(2528, 659, 2527);
            s.store_sub(2529, 659, 2527);
            s.store_sqrt_square_add(2530, 2529, 2525);
            s.store_div_scaled_product_add_scaled_denominator_indices(2574, 821, 659, 2.0, 2528, 1.0, 2530, 1.0, 1.0);
        }

        s.b[2575] = (s.v[652] > 0.5);
        s.v[2575] = if s.b[2575] { 1.0 } else { 0.0 };

        s.b[2576] = (s.v[409] == 0.5);
        s.v[2576] = if s.b[2576] { 1.0 } else { 0.0 };

        if (((s.b[2569] && s.b[2570]) && s.b[2575]) && s.b[2576]) {
            s.store_sqrt_sub_from_scalar_ad(2573, 1.0, A::scale(s.ad_value(2574), s.v[406]));
        }

        if (((s.b[2569] && s.b[2570]) && s.b[2575]) && (!s.b[2576])) {
            s.store_powf_ad(2573, A::sub_from_scalar(1.0, A::scale(s.ad_value(2574), s.v[406])), s.v[409]);
        }

        if ((s.b[2569] && s.b[2570]) && s.b[2575]) {
            s.store_add_scaled_inputs3_offset(1902, s.ad_value(2573), (-s.v[418]), s.ad_value(821), s.v[421], s.ad_value(2574), (-s.v[421]), s.v[418]);
        }

        s.b[2577] = (s.v[653] > 0.5);
        s.v[2577] = if s.b[2577] { 1.0 } else { 0.0 };

        s.b[2578] = (s.v[410] == 0.5);
        s.v[2578] = if s.b[2578] { 1.0 } else { 0.0 };

        if (((s.b[2569] && s.b[2570]) && s.b[2577]) && s.b[2578]) {
            s.store_sqrt_sub_from_scalar_ad(2573, 1.0, A::scale(s.ad_value(2574), s.v[407]));
        }

        if (((s.b[2569] && s.b[2570]) && s.b[2577]) && (!s.b[2578])) {
            s.store_powf_ad(2573, A::sub_from_scalar(1.0, A::scale(s.ad_value(2574), s.v[407])), s.v[410]);
        }

        if ((s.b[2569] && s.b[2570]) && s.b[2577]) {
            s.store_add_scaled_inputs3_offset(1903, s.ad_value(2573), (-s.v[419]), s.ad_value(821), s.v[422], s.ad_value(2574), (-s.v[422]), s.v[419]);
        }

        s.b[2579] = (s.v[654] > 0.5);
        s.v[2579] = if s.b[2579] { 1.0 } else { 0.0 };

        s.b[2580] = (s.v[411] == 0.5);
        s.v[2580] = if s.b[2580] { 1.0 } else { 0.0 };

        if (((s.b[2569] && s.b[2570]) && s.b[2579]) && s.b[2580]) {
            s.store_sqrt_sub_from_scalar_ad(2573, 1.0, A::scale(s.ad_value(2574), s.v[408]));
        }

        if (((s.b[2569] && s.b[2570]) && s.b[2579]) && (!s.b[2580])) {
            s.store_powf_ad(2573, A::sub_from_scalar(1.0, A::scale(s.ad_value(2574), s.v[408])), s.v[411]);
        }

        if ((s.b[2569] && s.b[2570]) && s.b[2579]) {
            s.store_add_scaled_inputs3_offset(1904, s.ad_value(2573), (-s.v[420]), s.ad_value(821), s.v[423], s.ad_value(2574), (-s.v[423]), s.v[420]);
        }

    }
}

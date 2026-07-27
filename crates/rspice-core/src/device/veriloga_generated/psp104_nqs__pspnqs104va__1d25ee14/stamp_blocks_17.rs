#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_scalar(44, p[57]);s.store_scalar(45, p[58]);s.store_scalar(46, p[59]);s.store_scalar(47, p[60]);s.store_scalar(48, p[61]);s.store_scalar(49, p[62]);s.store_scalar(50, p[63]);s.store_scalar(51, p[64]);s.store_scalar(52, p[65]);s.store_scalar(53, p[66]);s.store_scalar(54, p[67]);s.store_scalar(59, p[68]);s.store_scalar(60, p[69]);s.store_scalar(61, p[70]);s.store_scalar(62, p[71]);s.store_scalar(55, p[72]);s.store_scalar(56, p[74]);s.store_scalar(57, p[73]);s.store_scalar(58, p[75]);s.store_scalar(63, p[79]);s.store_scalar(64, p[81]);s.store_scalar(65, p[80]);s.store_scalar(66, p[76]);s.store_scalar(67, p[78]);s.store_scalar(68, p[77]);s.store_scalar(69, p[82]);s.store_scalar(70, p[83]);s.store_scalar(71, p[84]);s.store_scalar(72, p[85]);s.store_scalar(73, p[86]);s.store_scalar(74, p[87]);s.store_scalar(75, p[88]);s.store_scalar(76, p[89]);s.store_scalar(77, p[90]);s.store_scalar(78, p[91]);s.store_scalar(79, p[92]);s.store_scalar(80, p[93]);s.store_scalar(81, p[94]);s.store_scalar(82, p[95]);s.store_scalar(83, p[96]);s.store_scalar(84, p[97]);s.store_scalar(85, p[98]);s.store_scalar(86, p[99]);s.store_scalar(87, p[100]);s.store_scalar(88, p[101]);s.store_scalar(89, p[102]);s.store_scalar(90, p[103]);s.store_scalar(91, p[104]);s.store_scalar(92, p[105]);s.store_scalar(93, p[106]);s.store_scalar(94, p[107]);s.store_scalar(95, p[108]);s.store_scalar(96, p[109]);s.store_scalar(97, p[110]);s.store_scalar(98, p[111]);s.store_scalar(99, p[112]);s.store_scalar(100, p[113]);s.store_scalar(101, p[114]);s.store_scalar(102, p[115]);s.store_scalar(103, p[116]);s.store_scalar(104, p[117]);s.store_scalar(105, p[118]);s.store_scalar(106, p[119]);s.store_scalar(107, p[120]);s.store_scalar(108, p[121]);s.store_scalar(109, p[120]);s.b[1024] = param_given[122];s.store_scalar(1024, if s.b[1024] { 1.0 } else { 0.0 });
        if s.b[1024] {s.store_scalar(109, p[122]);}
        s.store_scalar(110, p[121]);s.b[1025] = param_given[123];s.store_scalar(1025, if s.b[1025] { 1.0 } else { 0.0 });
        if s.b[1025] {s.store_scalar(110, p[123]);}
        s.copy_ad(111, 109);s.b[1026] = param_given[124];s.store_scalar(1026, if s.b[1026] { 1.0 } else { 0.0 });
        if s.b[1026] {s.store_scalar(111, p[124]);}
        s.copy_ad(112, 110);s.b[1027] = param_given[125];s.store_scalar(1027, if s.b[1027] { 1.0 } else { 0.0 });
        if s.b[1027] {s.store_scalar(112, p[125]);}
        s.store_scalar(113, p[126]);s.store_scalar(114, p[127]);s.store_scalar(115, p[128]);s.store_scalar(116, p[129]);s.store_scalar(117, p[130]);s.store_scalar(118, p[131]);s.store_scalar(119, p[132]);s.store_scalar(120, p[133]);s.store_scalar(121, p[134]);s.store_scalar(122, p[135]);s.store_scalar(123, p[136]);s.store_scalar(124, p[137]);s.store_scalar(125, p[99]);s.b[1028] = param_given[138];s.store_scalar(1028, if s.b[1028] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1028] {s.store_scalar(125, p[138]);}
        s.store_scalar(126, p[104]);s.b[1029] = param_given[139];s.store_scalar(1029, if s.b[1029] { 1.0 } else { 0.0 });
        if s.b[1029] {s.store_scalar(126, p[139]);}
        s.store_scalar(127, p[140]);s.store_scalar(128, p[141]);s.store_scalar(129, p[142]);s.store_scalar(130, p[143]);s.store_scalar(131, p[144]);s.store_scalar(132, p[145]);s.store_scalar(133, p[146]);s.store_scalar(134, p[147]);s.store_scalar(135, p[148]);s.store_scalar(136, p[149]);s.store_scalar(137, p[150]);s.store_scalar(138, p[151]);s.store_scalar(139, p[152]);s.store_scalar(140, p[153]);s.store_scalar(141, p[154]);s.store_scalar(142, p[155]);s.store_scalar(143, p[156]);s.store_scalar(149, p[162]);s.store_scalar(150, p[163]);s.store_scalar(151, p[164]);s.store_scalar(152, p[165]);s.store_scalar(153, p[166]);s.store_scalar(154, p[167]);s.store_scalar(155, p[168]);s.store_scalar(156, p[169]);s.store_scalar(157, p[170]);s.store_scalar(158, p[171]);s.store_scalar(159, p[172]);s.store_scalar(160, p[174]);s.store_scalar(161, p[173]);s.store_scalar(176, p[187]);s.b[1030] = (p[39] > 0.0);s.store_scalar(1030, if s.b[1030] { 1.0 } else { 0.0 });
        if s.b[1030] {s.store_primal_add_scaled_inputs3_offset_mixed_aii(44, A::powf(s.ad_value(314), p[200]), p[199], 316, p[201], 318, p[202], p[198]);s.store_primal_add_scaled_inputs3_offset_indices(45, 314, p[204], 316, p[205], 318, p[206], p[203]);s.store_scalar(46, p[207]);s.store_scalar(47, p[208]);s.store_scalar(48, p[209]);}
        if s.b[1030] {
            s.store_primal_scale_ad(331, {
                if ((1.0 + ((p[211] * s.v[316]) * (((1.0 + (s.v[313] / p[212]))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p[211], A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p[212]), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p[210]);
        }
        if s.b[1030] {
            s.store_primal_scale_ad(332, {
                if ((1.0 + ((p[214] * s.v[316]) * (((1.0 + (s.v[313] / p[215]))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p[214], A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p[215]), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p[213]);
        }
        if s.b[1030] {
            s.store_primal_scale_ad(333, {
                if ((1.0 + ((p[217] * s.v[316]) * (((1.0 + (s.v[313] / p[215]))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p[217], A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p[215]), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p[216]);
        }
        s.b[1031] = (s.v[312] > (2.0 * s.v[333]));s.store_scalar(1031, if s.b[1031] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1031]) {s.store_scalar(334, 75000000000.0);s.store_primal_sub_ad(335, A::sqrt(A::add_scaled_inputs(s.ad_value(331), 1.0, s.ad_value(332), 0.5)), A::sqrt(s.ad_value(331)));s.store_primal_add_scaled_product_mixed_aia(336, A::sqrt(s.ad_value(331)), 1.0, 334, A::ln(A::offset(A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(333), 2.0, s.ad_value(312), 1.0), A::exp(A::div(s.ad_value(335), s.ad_value(334))), (-1.0)), 1.0)), 1.0);s.store_primal_square(336, 336);}
        s.b[1032] = (s.v[312] >= s.v[333]);s.store_scalar(1032, if s.b[1032] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1030] && (!s.b[1031])) && s.b[1032]) {s.store_primal_add_mixed_ia(336, 331, A::div_scaled_product(s.ad_value(332), s.ad_value(333), 1.0, s.ad_value(312), 1.0));}
        if ((s.b[1030] && (!s.b[1031])) && (!s.b[1032])) {s.store_primal_add_mixed_ia(336, 331, A::mul_sub_from_scalar_rhs(s.ad_value(332), 2.0, A::div(s.ad_value(312), s.ad_value(333))));}
        if s.b[1030] {s.store_primal_mul_sub_scaled_inputs_rhs_mixed_ai(49, 336, A::sub_from_scalar(1.0, A::scale(s.ad_value(314), p[218])), 1.0, 315, p[219]);s.store_primal_add_scaled_inputs3_offset_mixed_aii(50, A::powf(s.ad_value(314), p[222]), p[221], 316, p[223], 318, p[224], p[220]);s.store_scalar(51, p[225]);s.store_scalar(52, p[226]);s.store_primal_add_scaled_inputs3_offset_mixed_aii(53, A::powf(s.ad_value(314), p[229]), p[228], 316, p[230], 318, p[231], p[227]);}
        if s.b[1030] {
            s.store_primal_scale_ad(54, {
                if (1e-6 > (1.0 + (p[233] * s.v[314]))) {
                    A::constant(1e-6)
                } else {
                    A::scale_offset(s.ad_value(314), p[233], 1.0)
                }
            }, p[232]);
        }
        if s.b[1030] {s.store_scalar(59, p[234]);s.store_scalar(60, p[235]);s.store_scalar(61, p[238]);s.store_scalar(62, p[239]);s.store_primal_mul3_ad(55, A::scale_offset(A::powf(s.ad_value(314), p[242]), p[241], p[240]), A::scale_offset(s.ad_value(316), p[243], 1.0), A::scale_offset(s.ad_value(318), p[244], 1.0));s.store_scalar(56, p[246]);s.store_scalar(57, p[245]);s.store_scalar(58, p[247]);s.store_primal_mul_powf_scale_offset_lhs(66, 314, 316, p[249], (p[250]) * (p[248]), (1.0) * (p[248]));s.store_scalar(67, p[252]);s.store_scalar(68, p[251]);s.store_primal_mul_powf_scale_offset_lhs(63, 314, 316, p[254], (p[255]) * (p[253]), (1.0) * (p[253]));s.store_scalar(64, p[257]);s.store_scalar(65, p[256]);s.store_primal_offset_scaled(337, 316, ((p[260]) * (p[259])), p[259]);}
        if s.b[1030] {
            s.store_primal_scale_ad(338, {
                if ((1.0 + (p[262] * s.v[316])) > 0.001) {
                    A::scale_offset(s.ad_value(316), p[262], 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p[261]);
        }
        if s.b[1030] {s.store_primal_add_ad(339, A::offset(A::mul_sub_from_scalar_rhs(A::div_scaled_product(s.ad_value(337), s.ad_value(338), 1.0, s.ad_value(312), 1.0), 1.0, A::exp_div_scaled_inputs(s.ad_value(312), -1.0, s.ad_value(338), 1.0)), 1.0), A::mul_sub_from_scalar_rhs(A::div_from_scalar((p[263] * p[264]), s.ad_value(312)), 1.0, A::exp_scaled_input(s.ad_value(312), (-1.0 / (p[264])))));}
        if s.b[1030] {
            if (s.v[339] > 1e-15) {
            } else {
                s.store_scalar(339, 1e-15);
            }
        }
        if s.b[1030] {s.store_primal_add_scaled_product_mixed_aia(340, A::scale_offset(s.ad_value(316), p[265], 1.0), 1.0, 316, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p[267]), 1.0)), p[266]);s.store_primal_mul_div_scaled_inputs_mixed_iia(69, 340, 313, p[258], A::mul(s.ad_value(339), s.ad_value(312)), 1.0);s.store_primal_add_scaled_inputs3_offset_indices(70, 314, p[269], 316, p[270], 318, p[271], p[268]);s.store_primal_offset_scaled(71, 316, ((p[273]) * (p[272])), p[272]);s.store_scalar(72, p[274]);s.store_scalar(73, p[275]);s.store_scalar(74, p[276]);s.store_primal_mul3_ad(75, A::scale_offset(A::powf(s.ad_value(314), p[279]), p[278], p[277]), A::scale_offset(s.ad_value(316), p[280], 1.0), A::scale_offset(s.ad_value(318), p[281], 1.0));s.store_scalar(76, p[282]);s.store_scalar(77, p[283]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_8(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1030] {s.store_scalar(78, p[284]);s.store_primal_mul3_ad_scaled_output(79, A::scale_offset(s.ad_value(314), p[286], 1.0), A::scale_offset(s.ad_value(316), p[287], 1.0), A::scale_offset(s.ad_value(318), p[288], 1.0), p[285]);s.store_scalar(80, p[289]);s.store_scalar(81, p[290]);s.store_primal_mul_scale_offset_rhs(82, 316, 316, ((p[292]) * (p[291])), p[291]);s.store_scalar(83, p[293]);s.store_scalar(84, p[294]);s.store_scalar(85, p[295]);s.store_primal_mul3_ad(86, A::offset(A::mul(A::div_scaled_inputs(s.ad_value(340), p[297], s.ad_value(339), 1.0), A::powf(s.ad_value(314), p[298])), p[296]), A::scale_offset(s.ad_value(316), p[299], 1.0), A::scale_offset(s.ad_value(318), p[300], 1.0));s.store_primal_add_scaled_inputs3_offset_indices(87, 314, p[302], 316, p[303], 318, p[304], p[301]);s.store_scalar(88, p[305]);s.store_scalar(89, p[306]);s.store_scalar(90, p[307]);s.store_primal_div_from_scalar_offset_scaled_input(91, p[308], 314, p[309], 1.0);s.store_primal_mul_powf_scale_offset_lhs(92, 314, 316, p[311], (p[312]) * (p[310]), (1.0) * (p[310]));s.store_primal_powf(341, 314, p[314]);s.store_primal_div_scaled_product_offset_denominator_mixed_iaa(93, 341, A::scale_offset(s.ad_value(316), p[316], 1.0), p[313], A::mul_scaled_lhs(s.ad_value(314), p[315], s.ad_value(341)), 1.0, 1.0);s.store_primal_powf(341, 314, p[318]);s.store_primal_div_scaled_product_offset_denominator_mixed_iaa(94, 341, A::scale_offset(s.ad_value(316), p[320], 1.0), p[317], A::mul_scaled_lhs(s.ad_value(314), p[319], s.ad_value(341)), 1.0, 1.0);s.store_scalar(95, p[321]);s.store_primal_scaled_mul_scale_offset_inputs(96, 314, p[323], 1.0, 316, p[324], 1.0, p[322]);s.store_scalar(97, p[325]);s.store_scalar(98, p[326]);s.store_primal_scaled_mul_scale_offset_inputs(99, 314, p[328], 1.0, 316, p[329], 1.0, p[327]);s.store_primal_scaled_mul_scale_offset_inputs(100, 314, p[331], 1.0, 316, p[332], 1.0, p[330]);s.store_scalar(101, p[333]);s.store_scalar(102, p[334]);s.store_primal_div_from_scalar(103, p[335], 318);s.store_primal_div_from_scalar_scaled_input(104, (p[336] * p[236]), 316, 1e-6);s.store_primal_div_from_scalar_scaled_input(105, (p[337] * p[237]), 316, 1e-6);s.store_scalar(106, p[338]);s.store_scalar(107, p[339]);s.store_scalar(108, p[340]);s.store_scalar(109, p[339]);}
        s.b[1033] = param_given[341];s.store_scalar(1033, if s.b[1033] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1033]) {s.store_scalar(109, p[341]);}
        if s.b[1030] {s.store_scalar(110, p[340]);}
        s.b[1034] = param_given[342];s.store_scalar(1034, if s.b[1034] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1034]) {s.store_scalar(110, p[342]);}
        if s.b[1030] {s.copy_ad(111, 109);}
        s.b[1035] = param_given[343];s.store_scalar(1035, if s.b[1035] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1035]) {s.store_scalar(111, p[343]);}
        if s.b[1030] {s.copy_ad(112, 110);}
        s.b[1036] = param_given[344];s.store_scalar(1036, if s.b[1036] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1036]) {s.store_scalar(112, p[344]);}
        if s.b[1030] {s.store_scalar(113, p[345]);s.store_primal_div_from_scalar_scaled_input(114, (p[346] * p[236]), 316, 1e-6);s.store_primal_div_from_scalar_scaled_input(115, (p[347] * p[237]), 316, 1e-6);s.store_scalar(116, p[348]);s.store_scalar(117, p[349]);s.store_scalar(118, p[350]);s.store_scalar(119, p[351]);s.store_scalar(120, p[352]);s.store_scalar(121, p[353]);s.store_primal_scaled_mul(122, 321, 320, ((8.8541878176e-12 * p[209]) * 1.0 / (p[208])));s.store_primal_scale(129, 321, ((8.8541878176e-12 * p[209]) * (p[236] * 1.0 / (p[234]))));s.store_primal_scale(130, 321, ((8.8541878176e-12 * p[209]) * (p[237] * 1.0 / (p[235]))));s.store_primal_add_scaled_inputs3_offset_mixed_aii(123, A::powf(s.ad_value(314), p[356]), p[355], 316, p[357], 318, p[358], p[354]);s.store_primal_add_scaled_inputs3_offset_indices(124, 314, p[360], 316, p[361], 318, p[362], p[359]);s.store_scalar(36, p[296]);}
        s.b[1037] = param_given[363];s.store_scalar(1037, if s.b[1037] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1037]) {s.store_scalar(36, p[363]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_9(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1030] {s.store_scalar(37, p[297]);}
        s.b[1038] = param_given[364];s.store_scalar(1038, if s.b[1038] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1038]) {s.store_scalar(37, p[364]);}
        if s.b[1030] {s.store_scalar(38, p[298]);}
        s.b[1039] = param_given[365];s.store_scalar(1039, if s.b[1039] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1039]) {s.store_scalar(38, p[365]);}
        if s.b[1030] {s.store_scalar(39, p[299]);}
        s.b[1040] = param_given[366];s.store_scalar(1040, if s.b[1040] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1040]) {s.store_scalar(39, p[366]);}
        if s.b[1030] {s.store_scalar(40, p[300]);}
        s.b[1041] = param_given[367];s.store_scalar(1041, if s.b[1041] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1041]) {s.store_scalar(40, p[367]);}
        if s.b[1030] {s.store_primal_mul3_ad(125, A::add_scaled_product(s.ad_value(36), 1.0, A::div_scaled_product(s.ad_value(37), s.ad_value(340), 1.0, s.ad_value(339), 1.0), A::pow(s.ad_value(314), s.ad_value(38)), 1.0), A::offset(A::mul(s.ad_value(39), s.ad_value(316)), 1.0), A::offset(A::mul(s.ad_value(40), s.ad_value(318)), 1.0));s.store_scalar(41, p[308]);}
        s.b[1042] = param_given[368];s.store_scalar(1042, if s.b[1042] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1042]) {s.store_scalar(41, p[368]);}
        if s.b[1030] {s.store_scalar(42, p[309]);}
        s.b[1043] = param_given[369];s.store_scalar(1043, if s.b[1043] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1043]) {s.store_scalar(42, p[369]);}
        if s.b[1030] {s.store_primal_div_scaled_value_offset_denominator(126, s.ad_value(41), 1.0, A::mul(s.ad_value(42), s.ad_value(314)), 1.0, 1.0);s.store_primal_mul_powf_scale_offset_lhs(127, 314, 316, p[371], (p[372]) * (p[370]), (1.0) * (p[370]));s.store_primal_powf(341, 314, p[374]);s.store_primal_div_scaled_product_offset_denominator_mixed_iaa(128, 341, A::scale_offset(s.ad_value(316), p[376], 1.0), p[373], A::mul_scaled_lhs(s.ad_value(314), p[375], s.ad_value(341)), 1.0, 1.0);s.store_scalar(131, p[377]);s.store_scalar(132, p[378]);s.store_scalar(133, p[379]);s.store_primal_scale(134, 325, p[380]);s.store_primal_scale(135, 322, p[381]);s.store_primal_scale(136, 322, p[382]);s.store_scalar(137, p[383]);s.store_scalar(138, p[384]);s.store_scalar(139, p[385]);s.store_scalar(140, p[386]);s.store_primal_scale(141, 326, p[387]);s.store_primal_scale(142, 326, p[388]);s.store_primal_sub_from_scalar_ad(1012, 1.0, A::div_from_scalar((2.0 * p[395]), s.ad_value(312)));s.store_scalar(143, p[389]);s.store_primal_offset_scaled(344, 313, p[398], (2.0 * p[397]));s.store_scalar(149, p[399]);s.store_primal_add_scaled_inputs3_offset_indices(150, 314, p[401], 316, p[402], 318, p[403], p[400]);s.store_primal_add_scaled_inputs3_offset_mixed_aii(151, A::powf(s.ad_value(314), p[406]), p[405], 316, p[407], 318, p[408], p[404]);s.store_primal_mul3_ad_scaled_output(152, A::scale_offset(A::powf(s.ad_value(314), p[411]), p[410], 1.0), A::scale_offset(s.ad_value(316), p[412], 1.0), A::scale_offset(s.ad_value(318), p[413], 1.0), p[409]);s.store_primal_offset_scaled_ad(153, A::powf(s.ad_value(314), p[416]), p[415], p[414]);s.store_primal_offset_ad(347, A::mul_sub_from_scalar_rhs(A::div_from_scalar((p[417] * p[418]), s.ad_value(312)), 1.0, A::exp_scaled_input(s.ad_value(312), (-1.0 / (p[418])))), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1030] {
            if (s.v[347] > 1e-15) {
            } else {
                s.store_scalar(347, 1e-15);
            }
        }
        if s.b[1030] {s.store_primal_mul_div_scaled_inputs_mixed_aia(154, A::scale_offset(s.ad_value(316), p[419], 1.0), 344, p[258], A::mul(s.ad_value(347), s.ad_value(312)), 1.0);s.store_primal_add_scaled_inputs3_offset_indices(155, 314, p[421], 316, p[422], 318, p[423], p[420]);s.store_primal_mul_powf_scale_offset_lhs(156, 314, 316, p[425], (p[426]) * (p[424]), (1.0) * (p[424]));s.store_scalar(157, p[427]);s.store_scalar(158, p[428]);s.store_primal_mul_powf_scale_offset_lhs(159, 314, 316, p[430], (p[431]) * (p[429]), (1.0) * (p[429]));s.store_scalar(160, p[433]);s.store_scalar(161, p[432]);s.store_primal_add_scaled_inputs3_offset_indices(348, 314, p[815], 316, p[816], 318, p[817], p[814]);s.store_primal_add_scaled_inputs3_offset_indices(349, 314, p[819], 316, p[820], 318, p[821], p[818]);s.store_scalar(176, p[450]);}
        s.b[1045] = (((param_given[451] || param_given[452]) || param_given[453]) || param_given[454]);s.store_scalar(1045, if s.b[1045] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1045]) {s.store_primal_add_scaled_inputs3_offset_indices(44, 314, p[452], 316, p[453], 318, p[454], p[451]);}
        s.b[1046] = (((param_given[455] || param_given[456]) || param_given[457]) || param_given[458]);s.store_scalar(1046, if s.b[1046] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1046]) {s.store_primal_add_scaled_inputs3_offset_indices(45, 314, p[456], 316, p[457], 318, p[458], p[455]);}
        s.b[1047] = (((param_given[459] || param_given[460]) || param_given[461]) || param_given[462]);s.store_scalar(1047, if s.b[1047] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1047]) {s.store_primal_add_scaled_inputs3_offset_indices(49, 314, p[460], 316, p[461], 318, p[462], p[459]);}
        s.b[1048] = (((param_given[463] || param_given[464]) || param_given[465]) || param_given[466]);s.store_scalar(1048, if s.b[1048] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1048]) {s.store_primal_add_scaled_inputs3_offset_indices(50, 314, p[464], 316, p[465], 318, p[466], p[463]);}
        s.b[1049] = (((param_given[467] || param_given[468]) || param_given[469]) || param_given[470]);s.store_scalar(1049, if s.b[1049] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1049]) {s.store_primal_add_scaled_inputs3_offset_indices(51, 314, p[468], 316, p[469], 318, p[470], p[467]);}
        s.b[1050] = (((param_given[471] || param_given[472]) || param_given[473]) || param_given[474]);s.store_scalar(1050, if s.b[1050] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1050]) {s.store_primal_add_scaled_inputs3_offset_indices(53, 314, p[472], 316, p[473], 318, p[474], p[471]);}
        s.b[1051] = (((param_given[475] || param_given[476]) || param_given[477]) || param_given[478]);s.store_scalar(1051, if s.b[1051] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1051]) {s.store_primal_add_scaled_inputs3_offset_indices(54, 314, p[476], 316, p[477], 318, p[478], p[475]);}
        s.b[1052] = (((param_given[479] || param_given[480]) || param_given[481]) || param_given[482]);s.store_scalar(1052, if s.b[1052] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1052]) {s.store_primal_add_scaled_inputs3_offset_indices(61, 314, p[480], 316, p[481], 318, p[482], p[479]);}
        s.b[1053] = (((param_given[483] || param_given[484]) || param_given[485]) || param_given[486]);s.store_scalar(1053, if s.b[1053] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1053]) {s.store_primal_add_scaled_inputs3_offset_indices(62, 314, p[484], 316, p[485], 318, p[486], p[483]);}
        s.b[1054] = (((param_given[487] || param_given[488]) || param_given[489]) || param_given[490]);s.store_scalar(1054, if s.b[1054] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1054]) {s.store_primal_add_scaled_inputs3_offset_indices(55, 314, p[488], 316, p[489], 318, p[490], p[487]);}
        s.b[1055] = (((param_given[495] || param_given[496]) || param_given[497]) || param_given[498]);s.store_scalar(1055, if s.b[1055] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1055]) {s.store_primal_add_scaled_inputs3_offset_indices(56, 314, p[496], 316, p[497], 318, p[498], p[495]);}
        s.b[1056] = (((param_given[491] || param_given[492]) || param_given[493]) || param_given[494]);s.store_scalar(1056, if s.b[1056] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1056]) {s.store_primal_add_scaled_inputs3_offset_indices(57, 314, p[492], 316, p[493], 318, p[494], p[491]);}
        s.b[1057] = (((param_given[499] || param_given[500]) || param_given[501]) || param_given[502]);s.store_scalar(1057, if s.b[1057] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1057]) {s.store_primal_add_scaled_inputs3_offset_indices(58, 314, p[500], 316, p[501], 318, p[502], p[499]);}
        s.b[1058] = (((param_given[503] || param_given[504]) || param_given[505]) || param_given[506]);s.store_scalar(1058, if s.b[1058] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1058]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(66, 315, 314, p[504], 316, p[505], 318, p[506], p[503]);}
        s.b[1059] = (((param_given[511] || param_given[512]) || param_given[513]) || param_given[514]);s.store_scalar(1059, if s.b[1059] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1059]) {s.store_primal_add_scaled_inputs3_offset_indices(67, 314, p[512], 316, p[513], 318, p[514], p[511]);}
        s.b[1060] = (((param_given[507] || param_given[508]) || param_given[509]) || param_given[510]);s.store_scalar(1060, if s.b[1060] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1060]) {s.store_primal_add_scaled_inputs3_offset_indices(68, 314, p[508], 316, p[509], 318, p[510], p[507]);}
        s.b[1061] = (((param_given[515] || param_given[516]) || param_given[517]) || param_given[518]);s.store_scalar(1061, if s.b[1061] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1061]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(63, 315, 314, p[516], 316, p[517], 318, p[518], p[515]);}
        s.b[1062] = (((param_given[523] || param_given[524]) || param_given[525]) || param_given[526]);s.store_scalar(1062, if s.b[1062] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1062]) {s.store_primal_add_scaled_inputs3_offset_indices(64, 314, p[524], 316, p[525], 318, p[526], p[523]);}
        s.b[1063] = (((param_given[519] || param_given[520]) || param_given[521]) || param_given[522]);s.store_scalar(1063, if s.b[1063] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1063]) {s.store_primal_add_scaled_inputs3_offset_indices(65, 314, p[520], 316, p[521], 318, p[522], p[519]);}
        s.b[1064] = (((param_given[527] || param_given[528]) || param_given[529]) || param_given[530]);s.store_scalar(1064, if s.b[1064] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1064]) {s.store_primal_mul_div_scaled_inputs_mixed_aii(69, A::add_scaled_inputs3_offset(s.ad_value(314), p[528], s.ad_value(316), p[529], s.ad_value(318), p[530], p[527]), 313, 1.0, 312, 1.0);}
        s.b[1065] = (((param_given[531] || param_given[532]) || param_given[533]) || param_given[534]);s.store_scalar(1065, if s.b[1065] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1065]) {s.store_primal_add_scaled_inputs3_offset_indices(70, 314, p[532], 316, p[533], 318, p[534], p[531]);}
        s.b[1066] = (((param_given[535] || param_given[536]) || param_given[537]) || param_given[538]);s.store_scalar(1066, if s.b[1066] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1066]) {s.store_primal_add_scaled_inputs3_offset_indices(71, 314, p[536], 316, p[537], 318, p[538], p[535]);}
        s.b[1067] = (((param_given[539] || param_given[540]) || param_given[541]) || param_given[542]);s.store_scalar(1067, if s.b[1067] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1067]) {s.store_primal_add_scaled_inputs3_offset_indices(73, 314, p[540], 316, p[541], 318, p[542], p[539]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1068] = (((param_given[543] || param_given[544]) || param_given[545]) || param_given[546]);s.store_scalar(1068, if s.b[1068] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1068]) {s.store_primal_add_scaled_inputs3_offset_indices(75, 314, p[544], 316, p[545], 318, p[546], p[543]);}
        s.b[1069] = (((param_given[547] || param_given[548]) || param_given[549]) || param_given[550]);s.store_scalar(1069, if s.b[1069] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1069]) {s.store_primal_add_scaled_inputs3_offset_indices(77, 314, p[548], 316, p[549], 318, p[550], p[547]);}
        s.b[1070] = (((param_given[551] || param_given[552]) || param_given[553]) || param_given[554]);s.store_scalar(1070, if s.b[1070] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1070]) {s.store_primal_add_scaled_inputs3_offset_indices(79, 314, p[552], 316, p[553], 318, p[554], p[551]);}
        s.b[1071] = (((param_given[555] || param_given[556]) || param_given[557]) || param_given[558]);s.store_scalar(1071, if s.b[1071] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1071]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(82, 316, 314, p[556], 316, p[557], 318, p[558], p[555]);}
        s.b[1072] = (((param_given[559] || param_given[560]) || param_given[561]) || param_given[562]);s.store_scalar(1072, if s.b[1072] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1072]) {s.store_primal_add_scaled_inputs3_offset_indices(83, 314, p[560], 316, p[561], 318, p[562], p[559]);}
        s.b[1073] = (((param_given[563] || param_given[564]) || param_given[565]) || param_given[566]);s.store_scalar(1073, if s.b[1073] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1073]) {s.store_primal_add_scaled_inputs3_offset_indices(84, 314, p[564], 316, p[565], 318, p[566], p[563]);}
        s.b[1074] = (((param_given[567] || param_given[568]) || param_given[569]) || param_given[570]);s.store_scalar(1074, if s.b[1074] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1074]) {s.store_primal_add_scaled_inputs3_offset_indices(85, 314, p[568], 316, p[569], 318, p[570], p[567]);}
        s.b[1075] = (((param_given[571] || param_given[572]) || param_given[573]) || param_given[574]);s.store_scalar(1075, if s.b[1075] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1075]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(86, 314, 314, p[572], 316, p[573], 318, p[574], p[571]);}
        s.b[1076] = (((param_given[575] || param_given[576]) || param_given[577]) || param_given[578]);s.store_scalar(1076, if s.b[1076] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1076]) {s.store_primal_add_scaled_inputs3_offset_indices(87, 314, p[576], 316, p[577], 318, p[578], p[575]);}
        s.b[1077] = (((param_given[579] || param_given[580]) || param_given[581]) || param_given[582]);s.store_scalar(1077, if s.b[1077] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1077]) {s.store_primal_add_scaled_inputs3_offset_indices(88, 314, p[580], 316, p[581], 318, p[582], p[579]);}
        s.b[1078] = (((param_given[583] || param_given[584]) || param_given[585]) || param_given[586]);s.store_scalar(1078, if s.b[1078] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1078]) {s.store_primal_add_scaled_inputs3_offset_indices(89, 314, p[584], 316, p[585], 318, p[586], p[583]);}
        s.b[1079] = (((param_given[587] || param_given[588]) || param_given[589]) || param_given[590]);s.store_scalar(1079, if s.b[1079] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1079]) {s.store_primal_add_scaled_inputs3_offset_indices(91, 314, p[588], 316, p[589], 318, p[590], p[587]);}
        s.b[1080] = (((param_given[591] || param_given[592]) || param_given[593]) || param_given[594]);s.store_scalar(1080, if s.b[1080] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1080]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(92, 314, 314, p[592], 316, p[593], 318, p[594], p[591]);}
        s.b[1081] = (((param_given[595] || param_given[596]) || param_given[597]) || param_given[598]);s.store_scalar(1081, if s.b[1081] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1081]) {s.store_primal_add_scaled_inputs3_offset_indices(93, 314, p[596], 316, p[597], 318, p[598], p[595]);}
        s.b[1082] = (((param_given[599] || param_given[600]) || param_given[601]) || param_given[602]);s.store_scalar(1082, if s.b[1082] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1082]) {s.store_primal_add_scaled_inputs3_offset_indices(94, 314, p[600], 316, p[601], 318, p[602], p[599]);}
        s.b[1083] = (((param_given[603] || param_given[604]) || param_given[605]) || param_given[606]);s.store_scalar(1083, if s.b[1083] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1083]) {s.store_primal_add_scaled_inputs3_offset_indices(96, 314, p[604], 316, p[605], 318, p[606], p[603]);}
        s.b[1084] = (((param_given[607] || param_given[608]) || param_given[609]) || param_given[610]);s.store_scalar(1084, if s.b[1084] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1084]) {s.store_primal_add_scaled_inputs3_offset_indices(98, 314, p[608], 316, p[609], 318, p[610], p[607]);}
        s.b[1085] = (((param_given[611] || param_given[612]) || param_given[613]) || param_given[614]);s.store_scalar(1085, if s.b[1085] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1085]) {s.store_primal_add_scaled_inputs3_offset_indices(99, 314, p[612], 316, p[613], 318, p[614], p[611]);}
        s.b[1086] = (((param_given[615] || param_given[616]) || param_given[617]) || param_given[618]);s.store_scalar(1086, if s.b[1086] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1086]) {s.store_primal_add_scaled_inputs3_offset_indices(100, 314, p[616], 316, p[617], 318, p[618], p[615]);}
        s.b[1087] = (((param_given[619] || param_given[620]) || param_given[621]) || param_given[622]);s.store_scalar(1087, if s.b[1087] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1087]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(103, 319, 314, p[620], 316, p[621], 318, p[622], p[619]);}
        s.b[1088] = (((param_given[623] || param_given[624]) || param_given[625]) || param_given[626]);s.store_scalar(1088, if s.b[1088] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1088]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(104, 317, 314, p[624], 316, p[625], 318, p[626], p[623]);}
        s.b[1089] = (((param_given[627] || param_given[628]) || param_given[629]) || param_given[630]);s.store_scalar(1089, if s.b[1089] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1089]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(105, 317, 314, p[628], 316, p[629], 318, p[630], p[627]);}
        s.b[1090] = (((param_given[631] || param_given[632]) || param_given[633]) || param_given[634]);s.store_scalar(1090, if s.b[1090] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1090]) {s.store_primal_add_scaled_inputs3_offset_indices(106, 314, p[632], 316, p[633], 318, p[634], p[631]);}
        s.b[1091] = (((param_given[635] || param_given[636]) || param_given[637]) || param_given[638]);s.store_scalar(1091, if s.b[1091] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1091]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(114, 317, 314, p[636], 316, p[637], 318, p[638], p[635]);}
        s.b[1092] = (((param_given[639] || param_given[640]) || param_given[641]) || param_given[642]);s.store_scalar(1092, if s.b[1092] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1092]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(115, 317, 314, p[640], 316, p[641], 318, p[642], p[639]);}
        s.b[1093] = (((param_given[643] || param_given[644]) || param_given[645]) || param_given[646]);s.store_scalar(1093, if s.b[1093] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1093]) {s.store_primal_add_scaled_inputs3_offset_indices(118, 314, p[644], 316, p[645], 318, p[646], p[643]);}
        s.b[1094] = (((param_given[647] || param_given[648]) || param_given[649]) || param_given[650]);s.store_scalar(1094, if s.b[1094] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1094]) {s.store_primal_add_scaled_inputs3_offset_indices(119, 314, p[648], 316, p[649], 318, p[650], p[647]);}
        s.b[1095] = (((param_given[651] || param_given[652]) || param_given[653]) || param_given[654]);s.store_scalar(1095, if s.b[1095] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1095]) {s.store_primal_mul_ad_affine_product_rhs(122, 322, s.ad_value(320), A::add_scaled_inputs3_offset(s.ad_value(314), p[652], s.ad_value(316), p[653], s.ad_value(318), p[654], p[651]), 1.0 / (1e-6), 0.0);}
        s.b[1096] = (((param_given[655] || param_given[656]) || param_given[657]) || param_given[658]);s.store_scalar(1096, if s.b[1096] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1096]) {s.store_primal_add_scaled_inputs3_offset_indices(123, 314, p[656], 316, p[657], 318, p[658], p[655]);}
        s.b[1097] = (((param_given[659] || param_given[660]) || param_given[661]) || param_given[662]);s.store_scalar(1097, if s.b[1097] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1097]) {s.store_primal_add_scaled_inputs3_offset_indices(124, 314, p[660], 316, p[661], 318, p[662], p[659]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[1098] = (((((((param_given[663] || param_given[664]) || param_given[665]) || param_given[666]) || param_given[571]) || param_given[572]) || param_given[573]) || param_given[574]);s.store_scalar(1098, if s.b[1098] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1098]) {s.store_scalar(32, p[571]);}
        s.b[1099] = param_given[663];s.store_scalar(1099, if s.b[1099] { 1.0 } else { 0.0 });
        if ((s.b[1030] && s.b[1098]) && s.b[1099]) {s.store_scalar(32, p[663]);}
        if (s.b[1030] && s.b[1098]) {s.store_scalar(33, p[572]);}
        s.b[1100] = param_given[664];s.store_scalar(1100, if s.b[1100] { 1.0 } else { 0.0 });
        if ((s.b[1030] && s.b[1098]) && s.b[1100]) {s.store_scalar(33, p[664]);}
        if (s.b[1030] && s.b[1098]) {s.store_scalar(34, p[573]);}
        s.b[1101] = param_given[665];s.store_scalar(1101, if s.b[1101] { 1.0 } else { 0.0 });
        if ((s.b[1030] && s.b[1098]) && s.b[1101]) {s.store_scalar(34, p[665]);}
        if (s.b[1030] && s.b[1098]) {s.store_scalar(35, p[574]);}
        s.b[1102] = param_given[666];s.store_scalar(1102, if s.b[1102] { 1.0 } else { 0.0 });
        if ((s.b[1030] && s.b[1098]) && s.b[1102]) {s.store_scalar(35, p[666]);}
        if (s.b[1030] && s.b[1098]) {s.store_primal_mul_mixed_ia(125, 314, A::add_scaled_value_products3(s.ad_value(32), 1.0, s.ad_value(33), s.ad_value(314), 1.0, s.ad_value(34), s.ad_value(316), 1.0, s.ad_value(35), s.ad_value(318), 1.0));}
        s.b[1103] = (((((((param_given[667] || param_given[668]) || param_given[669]) || param_given[670]) || param_given[587]) || param_given[588]) || param_given[589]) || param_given[590]);s.store_scalar(1103, if s.b[1103] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1103]) {s.store_scalar(32, p[587]);}
        s.b[1104] = param_given[667];s.store_scalar(1104, if s.b[1104] { 1.0 } else { 0.0 });
        if ((s.b[1030] && s.b[1103]) && s.b[1104]) {s.store_scalar(32, p[667]);}
        if (s.b[1030] && s.b[1103]) {s.store_scalar(33, p[588]);}
        s.b[1105] = param_given[668];s.store_scalar(1105, if s.b[1105] { 1.0 } else { 0.0 });
        if ((s.b[1030] && s.b[1103]) && s.b[1105]) {s.store_scalar(33, p[668]);}
        if (s.b[1030] && s.b[1103]) {s.store_scalar(34, p[589]);}
        s.b[1106] = param_given[669];s.store_scalar(1106, if s.b[1106] { 1.0 } else { 0.0 });
        if ((s.b[1030] && s.b[1103]) && s.b[1106]) {s.store_scalar(34, p[669]);}
        if (s.b[1030] && s.b[1103]) {s.store_scalar(35, p[590]);}
        s.b[1107] = param_given[670];s.store_scalar(1107, if s.b[1107] { 1.0 } else { 0.0 });
        if ((s.b[1030] && s.b[1103]) && s.b[1107]) {s.store_scalar(35, p[670]);}
        if (s.b[1030] && s.b[1103]) {s.store_primal_add_scaled_value_products3_indices(126, 32, 1.0, 33, 314, 1.0, 34, 316, 1.0, 35, 318, 1.0);}
        s.b[1108] = (((param_given[671] || param_given[672]) || param_given[673]) || param_given[674]);s.store_scalar(1108, if s.b[1108] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1108]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(127, 314, 314, p[672], 316, p[673], 318, p[674], p[671]);}
        s.b[1109] = (((param_given[675] || param_given[676]) || param_given[677]) || param_given[678]);s.store_scalar(1109, if s.b[1109] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1109]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(128, 314, 314, p[676], 316, p[677], 318, p[678], p[675]);}
        s.b[1110] = (((param_given[679] || param_given[680]) || param_given[681]) || param_given[682]);s.store_scalar(1110, if s.b[1110] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1110]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(129, 322, 314, p[680], 316, p[681], 318, p[682], p[679]);}
        s.b[1111] = (((param_given[683] || param_given[684]) || param_given[685]) || param_given[686]);s.store_scalar(1111, if s.b[1111] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1111]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(130, 322, 314, p[684], 316, p[685], 318, p[686], p[683]);}
        s.b[1112] = (((param_given[687] || param_given[688]) || param_given[689]) || param_given[690]);s.store_scalar(1112, if s.b[1112] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1112]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(134, 325, 314, p[688], 316, p[689], 318, p[690], p[687]);}
        s.b[1113] = (((param_given[691] || param_given[692]) || param_given[693]) || param_given[694]);s.store_scalar(1113, if s.b[1113] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1113]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(135, 322, 314, p[692], 316, p[693], 318, p[694], p[691]);}
        s.b[1114] = (((param_given[695] || param_given[696]) || param_given[697]) || param_given[698]);s.store_scalar(1114, if s.b[1114] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1114]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(136, 322, 314, p[696], 316, p[697], 318, p[698], p[695]);}
        s.b[1115] = (((param_given[699] || param_given[700]) || param_given[701]) || param_given[702]);s.store_scalar(1115, if s.b[1115] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1115]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(141, 326, 314, p[700], 316, p[701], 318, p[702], p[699]);}
        s.b[1116] = (((param_given[703] || param_given[704]) || param_given[705]) || param_given[706]);s.store_scalar(1116, if s.b[1116] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1116]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(142, 326, 314, p[704], 316, p[705], 318, p[706], p[703]);}
        s.b[1121] = (((param_given[723] || param_given[724]) || param_given[725]) || param_given[726]);s.store_scalar(1121, if s.b[1121] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1121]) {s.store_primal_add_scaled_inputs3_offset_indices(149, 314, p[724], 316, p[725], 318, p[726], p[723]);}
        s.b[1122] = (((param_given[727] || param_given[728]) || param_given[729]) || param_given[730]);s.store_scalar(1122, if s.b[1122] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1122]) {s.store_primal_add_scaled_inputs3_offset_indices(150, 314, p[728], 316, p[729], 318, p[730], p[727]);}
        s.b[1123] = (((param_given[731] || param_given[732]) || param_given[733]) || param_given[734]);s.store_scalar(1123, if s.b[1123] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1123]) {s.store_primal_add_scaled_inputs3_offset_indices(151, 314, p[732], 316, p[733], 318, p[734], p[731]);}
        s.b[1124] = (((param_given[735] || param_given[736]) || param_given[737]) || param_given[738]);s.store_scalar(1124, if s.b[1124] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1124]) {s.store_primal_add_scaled_inputs3_offset_indices(152, 314, p[736], 316, p[737], 318, p[738], p[735]);}
        s.b[1125] = (((param_given[739] || param_given[740]) || param_given[741]) || param_given[742]);s.store_scalar(1125, if s.b[1125] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1125]) {s.store_primal_add_scaled_inputs3_offset_indices(153, 314, p[740], 316, p[741], 318, p[742], p[739]);}
        s.b[1126] = (((param_given[743] || param_given[744]) || param_given[745]) || param_given[746]);s.store_scalar(1126, if s.b[1126] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_13(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1030] && s.b[1126]) {s.store_primal_mul_div_scaled_inputs_mixed_aii(154, A::add_scaled_inputs3_offset(s.ad_value(314), p[744], s.ad_value(316), p[745], s.ad_value(318), p[746], p[743]), 344, 1.0, 312, 1.0);}
        s.b[1127] = (((param_given[747] || param_given[748]) || param_given[749]) || param_given[750]);s.store_scalar(1127, if s.b[1127] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1127]) {s.store_primal_add_scaled_inputs3_offset_indices(155, 314, p[748], 316, p[749], 318, p[750], p[747]);}
        s.b[1128] = (((param_given[751] || param_given[752]) || param_given[753]) || param_given[754]);s.store_scalar(1128, if s.b[1128] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1128]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(156, 315, 314, p[752], 316, p[753], 318, p[754], p[751]);}
        s.b[1129] = (((param_given[755] || param_given[756]) || param_given[757]) || param_given[758]);s.store_scalar(1129, if s.b[1129] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1129]) {s.store_primal_add_scaled_inputs3_offset_indices(157, 314, p[756], 316, p[757], 318, p[758], p[755]);}
        s.b[1130] = (((param_given[759] || param_given[760]) || param_given[761]) || param_given[762]);s.store_scalar(1130, if s.b[1130] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1130]) {s.store_primal_add_scaled_inputs3_offset_indices(158, 314, p[760], 316, p[761], 318, p[762], p[759]);}
        s.b[1131] = (((param_given[763] || param_given[764]) || param_given[765]) || param_given[766]);s.store_scalar(1131, if s.b[1131] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1131]) {s.store_primal_mul_add_scaled_inputs3_offset_rhs_indices(159, 315, 314, p[764], 316, p[765], 318, p[766], p[763]);}
        s.b[1132] = (((param_given[771] || param_given[772]) || param_given[773]) || param_given[774]);s.store_scalar(1132, if s.b[1132] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1132]) {s.store_primal_add_scaled_inputs3_offset_indices(160, 314, p[772], 316, p[773], 318, p[774], p[771]);}
        s.b[1133] = (((param_given[767] || param_given[768]) || param_given[769]) || param_given[770]);s.store_scalar(1133, if s.b[1133] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1133]) {s.store_primal_add_scaled_inputs3_offset_indices(161, 314, p[768], 316, p[769], 318, p[770], p[767]);}
        s.b[1137] = (((param_given[787] || param_given[788]) || param_given[789]) || param_given[790]);s.store_scalar(1137, if s.b[1137] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1137]) {s.store_primal_add_scaled_inputs3_offset_indices(176, 314, p[788], 316, p[789], 318, p[790], p[787]);}
        if s.b[1030] {s.store_scalar(1019, 0.0);s.store_scalar(1020, 0.0);s.store_scalar(1018, 0.0);s.store_scalar(43, p[795]);}
        s.b[1138] = param_given[796];s.store_scalar(1138, if s.b[1138] { 1.0 } else { 0.0 });
        if (s.b[1030] && s.b[1138]) {s.store_scalar(43, p[796]);}
        s.b[1139] = (((s.v[9] > 0.0) && (s.v[10] > 0.0)) && ((s.v[5] == 1.0) || ((s.v[5] > 1.0) && (s.v[11] > 0.0))));s.store_scalar(1139, if s.b[1139] { 1.0 } else { 0.0 });let mut t2: usize = 0;
        while {
            let t0: f64 = (s.v[5] - 0.5);let t1: f64 = if ((s.b[1030] && s.b[1139]) && (s.v[1018] < t0)) { 1.0 } else { 0.0 };
            t1 != 0.0
        } {
            t2 += 1;
            if t2 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t2, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (s.b[1030] && s.b[1139]) {s.store_primal_add_mixed_ia(1019, 1019, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1018), (s.v[11] + s.v[7]), (s.v[9] + (0.5 * s.v[7])))));s.store_primal_add_mixed_ia(1020, 1020, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1018), (s.v[11] + s.v[7]), (s.v[10] + (0.5 * s.v[7])))));s.store_primal_offset(1018, 1018, 1.0);}
        }
        if (s.b[1030] && s.b[1139]) {s.store_primal_mul(1003, 1019, 6);s.store_primal_mul(1004, 1020, 6);s.store_scalar(1005, (1.0 / (p[791] + (0.5 * s.v[7]))));s.store_scalar(1006, (1.0 / (p[792] + (0.5 * s.v[7]))));}
        if (s.b[1030] && s.b[1139]) {
            if ((s.v[7] + s.v[310]) > 1e-9) {
                s.store_primal_offset(1016, 310, s.v[7]);
            } else {
                s.store_scalar(1016, 1e-9);
            }
        }
        if (s.b[1030] && s.b[1139]) {
            if (((s.v[8] + s.v[311]) + p[793]) > 1e-9) {
                s.store_primal_offset_add(1017, 8, 311, p[793]);
            } else {
                s.store_scalar(1017, 1e-9);
            }
        }
        if (s.b[1030] && s.b[1139]) {s.store_primal_div_from_scalar_powf_ad(1014, 1.0, s.ad_value(1016), p[801]);s.store_primal_div_from_scalar_powf_ad(1015, 1.0, s.ad_value(1017), p[802]);s.store_primal_add_scaled_inputs_product_mixed_aiii(1007, A::scale_offset(s.ad_value(1014), p[798], 1.0), (1.0 + (p[797] * (s.v[352] - 1.0))), 1015, (p[799] * (1.0 + (p[797] * (s.v[352] - 1.0)))), 1014, 1015, (p[800] * (1.0 + (p[797] * (s.v[352] - 1.0)))));s.store_primal_div_scaled_inputs2_indices(1008, 1003, p[794], 1004, p[794], 1007, 1.0);s.store_primal_div_scaled_inputs2_indices(1009, 1005, p[794], 1006, p[794], 1007, 1.0);s.store_primal_div_from_scalar_powf_ad(1014, 1.0, s.ad_value(1016), p[807]);s.store_primal_div_from_scalar_powf_ad(1015, 1.0, s.ad_value(1017), p[808]);s.store_primal_add_scaled_inputs_product_mixed_aiii(1010, A::scale_offset(s.ad_value(1014), p[804], 1.0), 1.0, 1015, p[805], 1014, 1015, p[806]);s.store_primal_add_scaled_inputs4_indices(1012, 1003, 1.0, 1004, 1.0, 1005, -1.0, 1006, -1.0);s.store_primal_div_scaled_offset_numerator_mixed_ia(1013, 1008, 1.0, 1.0, A::offset(s.ad_value(1009), 1.0), 1.0);s.store_primal_mul(69, 69, 1013);s.store_primal_div_scaled_product3_mixed_iiaa(86, 86, 1013, A::scale_offset(s.ad_value(1009), p[795], 1.0), 1.0, A::scale_offset(s.ad_value(1008), p[795], 1.0), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1030] && s.b[1139]) {s.store_primal_div_scaled_product3_mixed_iiaa(125, 125, 1013, A::offset(A::mul(s.ad_value(43), s.ad_value(1009)), 1.0), 1.0, A::offset(A::mul(s.ad_value(43), s.ad_value(1008)), 1.0), 1.0);s.store_primal_mul(154, 154, 1013);s.store_primal_div_scaled_inputs_indices(1013, 1012, p[803], 1010, 1.0);s.store_primal_add(44, 44, 1013);s.store_primal_add(149, 149, 1013);s.store_primal_div_scaled_inputs_mixed_ia(1013, 1012, p[809], A::powf(s.ad_value(1010), p[810]), 1.0);s.store_primal_add(66, 66, 1013);s.store_primal_add(159, 159, 1013);}
        s.b[1140] = ((((s.v[15] > 0.0) || (s.v[16] > 0.0)) || (s.v[17] > 0.0)) || (s.v[12] > 0.0));s.store_scalar(1140, if s.b[1140] { 1.0 } else { 0.0 });s.b[1141] = (((s.v[15] == 0.0) && (s.v[16] == 0.0)) && (s.v[17] == 0.0));s.store_scalar(1141, if s.b[1141] { 1.0 } else { 0.0 });
        if ((s.b[1030] && s.b[1140]) && s.b[1141]) {s.store_primal_offset(1012, 8, s.v[12]);s.store_scalar(1013, (1.0 / p[811]));s.store_primal_div_from_scalar_scaled_input(15, (p[811] * p[811]), 1012, s.v[12]);s.store_primal_div_scaled_add_product_mixed_aaai(16, A::exp_scaled_input(s.ad_value(1013), ((-10.0) * s.v[12])), ((0.1 * s.v[12]) + (0.01 * p[811])), A::scale_offset(s.ad_value(1012), 0.1, (0.01 * p[811])), A::exp(A::mul_scaled_lhs(s.ad_value(1012), (-10.0), s.ad_value(1013))), (-1.0), 8, 1.0);s.store_primal_div_scaled_add_product_mixed_aaai(17, A::exp_scaled_input(s.ad_value(1013), ((-20.0) * s.v[12])), ((0.05 * s.v[12]) + (0.0025 * p[811])), A::scale_offset(s.ad_value(1012), 0.05, (0.0025 * p[811])), A::exp(A::mul_scaled_lhs(s.ad_value(1012), (-20.0), s.ad_value(1013))), (-1.0), 8, 1.0);}
        if (s.b[1030] && s.b[1140]) {s.store_primal_add_scaled_inputs3_indices(1012, 15, 1.0, 16, p[812], 17, p[813]);s.store_primal_add_scaled_product_indices(44, 44, 1.0, 348, 1012, 1.0);s.store_primal_mul_scale_offset_mixed_ia(69, 69, A::mul(s.ad_value(349), s.ad_value(1012)), 1.0, 1.0);s.store_primal_add_scaled_product_indices(149, 149, 1.0, 348, 1012, 1.0);s.store_primal_mul_scale_offset_mixed_ia(154, 154, A::mul(s.ad_value(349), s.ad_value(1012)), 1.0, 1.0);}
        s.copy_ad(177, 44);s.copy_ad(178, 45);s.copy_ad(179, 46);s.copy_ad(181, 47);s.copy_ad(182, 48);
        if (s.v[49] > 1e20) {
            if (s.v[49] < 1e26) {
                s.copy_ad(183, 49);
            } else {
                s.store_scalar(183, 1e26);
            }
        } else {
            s.store_scalar(183, 1e20);
        }
        if (s.v[50] > 0.01) {
            s.copy_ad(184, 50);
        } else {
            s.store_scalar(184, 0.01);
        }
        if (s.v[51] > 0.0) {
            s.copy_ad(185, 51);
        } else {
            s.store_scalar(185, 0.0);
        }
        s.copy_ad(186, 52);s.copy_ad(187, 53);
        if (s.v[54] > 0.0) {
            s.copy_ad(188, 54);
        } else {
            s.store_scalar(188, 0.0);
        }
        s.copy_ad(192, 59);s.copy_ad(193, 60);
        if (s.v[61] > 1e23) {
            if (s.v[61] < 1e27) {
                s.copy_ad(194, 61);
            } else {
                s.store_scalar(194, 1e27);
            }
        } else {
            s.store_scalar(194, 1e23);
        }
        if (s.v[62] > 1e23) {
            if (s.v[62] < 1e27) {
                s.copy_ad(195, 62);
            } else {
                s.store_scalar(195, 1e27);
            }
        } else {
            s.store_scalar(195, 1e23);
        }
        if (s.v[55] > 0.0) {
            s.copy_ad(189, 55);
        } else {
            s.store_scalar(189, 0.0);
        }
        if (s.v[57] > 0.0) {
            if (s.v[57] < 0.5) {
                s.copy_ad(191, 57);
            } else {
                s.store_scalar(191, 0.5);
            }
        } else {
            s.store_scalar(191, 0.0);
        }
        if (s.v[56] > 0.0) {
            if (s.v[56] < 1.0) {
                s.copy_ad(190, 56);
            } else {
                s.store_scalar(190, 1.0);
            }
        } else {
            s.store_scalar(190, 0.0);
        }
        s.copy_ad(180, 58);
        if (s.v[66] > 0.0) {
            s.copy_ad(196, 66);
        } else {
            s.store_scalar(196, 0.0);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
    ) {
        if (s.v[68] > 0.0) {
            if (s.v[68] < 1.0) {
                s.copy_ad(198, 68);
            } else {
                s.store_scalar(198, 1.0);
            }
        } else {
            s.store_scalar(198, 0.0);
        }
        if (s.v[67] > 0.0) {
            s.copy_ad(197, 67);
        } else {
            s.store_scalar(197, 0.0);
        }
        if (s.v[63] > 0.0) {
            s.copy_ad(199, 63);
        } else {
            s.store_scalar(199, 0.0);
        }
        if (s.v[65] > 0.0) {
            if (s.v[65] < 1.0) {
                s.copy_ad(200, 65);
            } else {
                s.store_scalar(200, 1.0);
            }
        } else {
            s.store_scalar(200, 0.0);
        }
        if (s.v[64] > 0.0) {
            s.copy_ad(201, 64);
        } else {
            s.store_scalar(201, 0.0);
        }
        if (s.v[69] > 0.0) {
            s.copy_ad(202, 69);
        } else {
            s.store_scalar(202, 0.0);
        }
        s.copy_ad(203, 70);
        if (s.v[71] > 0.0) {
            s.copy_ad(204, 71);
        } else {
            s.store_scalar(204, 0.0);
        }
        s.copy_ad(205, 72);
        if (s.v[73] > 0.0) {
            s.copy_ad(206, 73);
        } else {
            s.store_scalar(206, 0.0);
        }
        s.copy_ad(207, 74);
        if (s.v[75] > 0.0) {
            s.copy_ad(208, 75);
        } else {
            s.store_scalar(208, 0.0);
        }
        s.copy_ad(209, 76);
        if (s.v[77] > 0.0) {
            s.copy_ad(210, 77);
        } else {
            s.store_scalar(210, 0.0);
        }
        s.copy_ad(211, 78);
        if (s.v[79] > 0.0) {
            s.copy_ad(212, 79);
        } else {
            s.store_scalar(212, 0.0);
        }
        s.copy_ad(213, 80);s.copy_ad(214, 81);
        if (s.v[82] > 0.0) {
            s.copy_ad(215, 82);
        } else {
            s.store_scalar(215, 0.0);
        }
        s.copy_ad(216, 83);
        if (s.v[84] > (-0.5)) {
            if (s.v[84] < 1.0) {
                s.copy_ad(217, 84);
            } else {
                s.store_scalar(217, 1.0);
            }
        } else {
            s.store_scalar(217, (-0.5));
        }
        if (s.v[85] > (-0.5)) {
            s.copy_ad(218, 85);
        } else {
            s.store_scalar(218, (-0.5));
        }
        if (s.v[86] > 0.0) {
            s.copy_ad(219, 86);
        } else {
            s.store_scalar(219, 0.0);
        }
        s.copy_ad(220, 87);
        if (s.v[88] > (-0.5)) {
            if (s.v[88] < 1.0) {
                s.copy_ad(221, 88);
            } else {
                s.store_scalar(221, 1.0);
            }
        } else {
            s.store_scalar(221, (-0.5));
        }
        if (s.v[89] > (-0.5)) {
            s.copy_ad(222, 89);
        } else {
            s.store_scalar(222, (-0.5));
        }
        if (s.v[90] > 0.01) {
            s.copy_ad(223, 90);
        } else {
            s.store_scalar(223, 0.01);
        }
        if (s.v[91] > 2.0) {
            s.copy_ad(224, 91);
        } else {
            s.store_scalar(224, 2.0);
        }
        if (s.v[92] > 0.0) {
            s.copy_ad(225, 92);
        } else {
            s.store_scalar(225, 0.0);
        }
        if (s.v[93] > 0.0) {
            s.copy_ad(226, 93);
        } else {
            s.store_scalar(226, 0.0);
        }
        if (s.v[94] > 0.0) {
            s.copy_ad(227, 94);
        } else {
            s.store_scalar(227, 0.0);
        }
        s.copy_ad(228, 95);
        if (s.v[96] > 0.0) {
            s.copy_ad(229, 96);
        } else {
            s.store_scalar(229, 0.0);
        }
        s.copy_ad(230, 97);s.copy_ad(231, 98);
        if (s.v[99] > 0.0) {
            s.copy_ad(232, 99);
        } else {
            s.store_scalar(232, 0.0);
        }
        if (s.v[100] > 0.0) {
            s.copy_ad(233, 100);
        } else {
            s.store_scalar(233, 0.0);
        }
        if (s.v[101] > 1e-12) {
            s.copy_ad(234, 101);
        } else {
            s.store_scalar(234, 1e-12);
        }
        s.copy_ad(235, 102);
        if (s.v[103] > 0.0) {
            s.copy_ad(236, 103);
        } else {
            s.store_scalar(236, 0.0);
        }
        if (s.v[104] > 0.0) {
            s.copy_ad(237, 104);
        } else {
            s.store_scalar(237, 0.0);
        }
        if (s.v[105] > 0.0) {
            s.copy_ad(238, 105);
        } else {
            s.store_scalar(238, 0.0);
        }
        s.copy_ad(239, 106);s.copy_ad(240, 107);s.copy_ad(241, 108);s.copy_ad(242, 109);s.copy_ad(243, 110);s.copy_ad(244, 111);s.copy_ad(245, 112);s.copy_ad(246, 113);
        if (s.v[114] > 0.0) {
            s.copy_ad(247, 114);
        } else {
            s.store_scalar(247, 0.0);
        }
        if (s.v[115] > 0.0) {
            s.copy_ad(248, 115);
        } else {
            s.store_scalar(248, 0.0);
        }
        s.copy_ad(249, 116);s.copy_ad(250, 117);s.copy_ad(251, 118);s.copy_ad(252, 119);s.copy_ad(253, 120);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.copy_ad(254, 121);
        if (s.v[122] > 0.0) {
            s.copy_ad(255, 122);
        } else {
            s.store_scalar(255, 0.0);
        }
        s.copy_ad(256, 123);
        if (s.v[124] > 0.0) {
            s.copy_ad(257, 124);
        } else {
            s.store_scalar(257, 0.0);
        }
        if (s.v[125] > 0.0) {
            s.copy_ad(258, 125);
        } else {
            s.store_scalar(258, 0.0);
        }
        if (s.v[126] > 2.0) {
            s.copy_ad(259, 126);
        } else {
            s.store_scalar(259, 2.0);
        }
        s.copy_ad(260, 127);
        if (s.v[128] > 0.0) {
            s.copy_ad(261, 128);
        } else {
            s.store_scalar(261, 0.0);
        }
        if (s.v[129] > 0.0) {
            s.copy_ad(262, 129);
        } else {
            s.store_scalar(262, 0.0);
        }
        if (s.v[130] > 0.0) {
            s.copy_ad(263, 130);
        } else {
            s.store_scalar(263, 0.0);
        }
        s.copy_ad(264, 131);s.copy_ad(265, 132);s.copy_ad(266, 133);
        if (s.v[134] > 0.0) {
            s.copy_ad(267, 134);
        } else {
            s.store_scalar(267, 0.0);
        }
        if (s.v[135] > 0.0) {
            s.copy_ad(268, 135);
        } else {
            s.store_scalar(268, 0.0);
        }
        if (s.v[136] > 0.0) {
            s.copy_ad(269, 136);
        } else {
            s.store_scalar(269, 0.0);
        }
        s.copy_ad(270, 137);s.copy_ad(271, 138);s.copy_ad(272, 139);s.copy_ad(273, 140);
        if (s.v[141] > 0.0) {
            s.copy_ad(274, 141);
        } else {
            s.store_scalar(274, 0.0);
        }
        if (s.v[142] > 0.0) {
            s.copy_ad(275, 142);
        } else {
            s.store_scalar(275, 0.0);
        }
        s.copy_ad(276, 143);s.copy_ad(282, 149);s.copy_ad(283, 150);s.copy_ad(284, 151);
        if (s.v[152] > 1e20) {
            if (s.v[152] < 1e26) {
                s.copy_ad(285, 152);
            } else {
                s.store_scalar(285, 1e26);
            }
        } else {
            s.store_scalar(285, 1e20);
        }
        if (s.v[153] > 0.0) {
            s.copy_ad(286, 153);
        } else {
            s.store_scalar(286, 0.0);
        }
        if (s.v[154] > 0.0) {
            s.copy_ad(287, 154);
        } else {
            s.store_scalar(287, 0.0);
        }
        s.copy_ad(288, 155);
        if (s.v[156] > 0.0) {
            s.copy_ad(289, 156);
        } else {
            s.store_scalar(289, 0.0);
        }
        if (s.v[157] > 0.0) {
            if (s.v[157] < 1.0) {
                s.copy_ad(290, 157);
            } else {
                s.store_scalar(290, 1.0);
            }
        } else {
            s.store_scalar(290, 0.0);
        }
        if (s.v[158] > 0.0) {
            s.copy_ad(291, 158);
        } else {
            s.store_scalar(291, 0.0);
        }
        if (s.v[159] > 0.0) {
            s.copy_ad(292, 159);
        } else {
            s.store_scalar(292, 0.0);
        }
        if (s.v[161] > 0.0) {
            if (s.v[161] < 1.0) {
                s.copy_ad(294, 161);
            } else {
                s.store_scalar(294, 1.0);
            }
        } else {
            s.store_scalar(294, 0.0);
        }
        if (s.v[160] > 0.0) {
            s.copy_ad(293, 160);
        } else {
            s.store_scalar(293, 0.0);
        }
        if ((p[31] * s.v[5]) > 0.0) {
            s.store_primal_scale(19, 5, p[31]);
        } else {
            s.store_scalar(19, 0.0);
        }
        s.store_scalar(20, p[16]);s.store_scalar(21, p[15]);s.store_scalar(22, p[18]);s.store_scalar(23, p[17]);
        if (s.v[176] > 0.0) {
            s.copy_ad(307, 176);
        } else {
            s.store_scalar(307, 0.0);
        }
        s.b[1142] = (p[44] == 0.0);s.store_scalar(1142, if s.b[1142] { 1.0 } else { 0.0 });
        if s.b[1142] {s.copy_ad(193, 192);s.copy_ad(195, 194);s.copy_ad(248, 247);s.copy_ad(250, 249);s.copy_ad(252, 251);s.copy_ad(254, 253);s.copy_ad(238, 237);s.copy_ad(244, 242);s.copy_ad(245, 243);s.copy_ad(263, 262);s.copy_ad(265, 264);s.copy_ad(269, 268);s.copy_ad(275, 274);}
        s.store_primal_scale(768, 182, 8.8541878176e-12);s.store_primal_div(769, 768, 181);s.store_primal_square(770, 181);s.store_primal_scale(771, 769, 6.241449993689894e18);s.store_primal_mul(772, 257, 183);
        if (s.v[772] > 1e20) {
            if (s.v[772] < 1e26) {
            } else {
                s.store_scalar(772, 1e26);
            }
        } else {
            s.store_scalar(772, 1e20);
        }
        s.store_scalar(773, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1143] = (p[52] > 0.0);s.store_scalar(1143, if s.b[1143] { 1.0 } else { 0.0 });
        if s.b[1143] {s.store_primal_scale_ad(773, A::powf(s.ad_value(769), 0.6666666666666666), ((0.4 * 5.951993) * p[52]));}
        s.b[1144] = (s.v[0] == (-1.0));s.store_scalar(1144, if s.b[1144] { 1.0 } else { 0.0 });
        if (s.b[1143] && s.b[1144]) {s.store_primal_scale(773, 773, (7.448711 / 5.951993));}
        s.store_primal_scale(774, 769, (1e-8 * 1.0 / (s.v[767])));s.store_primal_scale(775, 214, 0.5);s.store_scalar(776, 0.5);s.b[1145] = (s.v[0] == (-1.0));s.store_scalar(1145, if s.b[1145] { 1.0 } else { 0.0 });
        if s.b[1145] {s.store_primal_scale(775, 214, 0.3333333333333333);s.store_scalar(776, 0.3333333333333333);}
        s.store_primal_offset_pow_from_scalar_ad(1011, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(224)), 1.0), (-1.0));
        s.store_primal_div_scaled_product_offset_lhs_mixed_iaa(777, 1011, (-1.0), A::offset(s.ad_value(1011), (-1.0)), 1.0, {
            if ((4.0 * s.v[1011]) > 0.0001) {
                A::scale(s.ad_value(1011), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);s.store_primal_offset_pow_from_scalar_ad(1011, 2.0, A::offset(A::div_from_scalar((-2.0), s.ad_value(259)), 1.0), (-1.0));
        s.store_primal_div_scaled_product_offset_lhs_mixed_iaa(778, 1011, (-1.0), A::offset(s.ad_value(1011), (-1.0)), 1.0, {
            if ((4.0 * s.v[1011]) > 0.0001) {
                A::scale(s.ad_value(1011), 4.0)
            } else {
                A::constant(0.0001)
            }
        }, 1.0);s.store_primal_div_from_scalar(779, 1.0, 228);s.store_primal_div(780, 768, 192);s.store_primal_div(781, 768, 193);s.store_primal_div_mixed_ai(782, A::sqrt_scaled_input(s.ad_value(194), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[355])), 780);s.store_primal_div_mixed_ai(783, A::sqrt_scaled_input(s.ad_value(195), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[355])), 781);s.store_primal_square(784, 782);s.store_primal_square(785, 783);s.store_primal_offset_div_ad(786, A::ln(A::offset(A::exp_scaled_input(s.ad_value(266), (0.005 * s.v[355])), (-1.0))), s.ad_value(266), (-((((((0.005 * s.v[355])) as f64).exp() - 1.0)) as f64).ln()));s.store_primal_add_mixed_ai(787, A::ln_scaled_input(s.ad_value(782), 0.5), 786);s.store_primal_add_mixed_ai(788, A::ln_scaled_input(s.ad_value(783), 0.5), 786);s.store_primal_div_from_scalar(820, 1.0, 782);s.store_primal_offset_scaled(821, 782, 3.1, 8.5);s.store_primal_square(789, 821);s.store_primal_scale(822, 821, 0.5);s.b[1146] = (s.v[820] < 0.06);s.store_scalar(1146, if s.b[1146] { 1.0 } else { 0.0 });
        if s.b[1146] {s.store_primal_scale(790, 820, 64.0);}
        s.b[1147] = (s.v[820] <= 0.45);s.store_scalar(1147, if s.b[1147] { 1.0 } else { 0.0 });
        if ((!s.b[1146]) && s.b[1147]) {s.store_primal_offset_scaled(790, 820, 22.0, 3.0);}
        s.b[1148] = (s.v[820] <= 1.6);s.store_scalar(1148, if s.b[1148] { 1.0 } else { 0.0 });
        if (((!s.b[1146]) && (!s.b[1147])) && s.b[1148]) {s.store_primal_offset_scaled(790, 820, (-7.2), 15.5);}
        if (((!s.b[1146]) && (!s.b[1147])) && (!s.b[1148])) {s.copy_ad(790, 782);}
        s.store_primal_add_scaled_inputs_product_mixed_iiia(791, 822, 1.0, 784, 0.5, 782, A::sqrt(A::add_scaled_inputs3(s.ad_value(822), 1.0, s.ad_value(784), 0.25, s.ad_value(790), 1.0)), (-1.0));s.store_primal_div_from_scalar(820, 1.0, 783);s.store_primal_offset_scaled(821, 783, 3.1, 8.5);s.store_primal_square(792, 821);s.store_primal_scale(822, 821, 0.5);s.b[1149] = (s.v[820] < 0.06);s.store_scalar(1149, if s.b[1149] { 1.0 } else { 0.0 });
        if s.b[1149] {s.store_primal_scale(793, 820, 64.0);}
        s.b[1150] = (s.v[820] <= 0.45);s.store_scalar(1150, if s.b[1150] { 1.0 } else { 0.0 });
        if ((!s.b[1149]) && s.b[1150]) {s.store_primal_offset_scaled(793, 820, 22.0, 3.0);}
        s.b[1151] = (s.v[820] <= 1.6);s.store_scalar(1151, if s.b[1151] { 1.0 } else { 0.0 });
        if (((!s.b[1149]) && (!s.b[1150])) && s.b[1151]) {s.store_primal_offset_scaled(793, 820, (-7.2), 15.5);}
        if (((!s.b[1149]) && (!s.b[1150])) && (!s.b[1151])) {s.copy_ad(793, 783);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_primal_add_scaled_inputs_product_mixed_iiia(794, 822, 1.0, 785, 0.5, 783, A::sqrt(A::add_scaled_inputs3(s.ad_value(822), 1.0, s.ad_value(785), 0.25, s.ad_value(793), 1.0)), (-1.0));s.store_primal_add_scaled_inputs_ad(728, A::offset(s.ad_value(187), s.v[362]), 1.0, A::ln_scaled_input(A::mul(s.ad_value(183), A::powf(s.ad_value(363), (-0.75))), 4e-26), (2.0 * s.v[715]));
        if (!(s.v[728] > 0.05)) {s.store_scalar(728, 0.05);}
        s.store_primal_div_mixed_ai(729, A::sqrt_scaled_input(s.ad_value(183), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[361])), 769);s.store_scalar(730, 0.0);s.store_scalar(731, 0.0);s.b[1152] = (s.v[188] > 0.0);s.store_scalar(1152, if s.b[1152] { 1.0 } else { 0.0 });
        if s.b[1152] {s.store_primal_div_from_scalar(732, 80000000.0, 770);}
        if s.b[1152] {
            if (s.v[188] > s.v[732]) {
                s.copy_ad(731, 188);
            } else {
                s.copy_ad(731, 732);
            }
        }
        if s.b[1152] {
            if (5e24 > s.v[731]) {
                s.store_scalar(731, 5e24);
            } else {
            }
        }
        if s.b[1152] {s.store_primal_div_scaled_product_indices(730, 769, 769, (2.0 * s.v[715]), 731, (1.6021918e-19 * s.v[767]));}
        s.store_scalar(733, ((100.0 * s.v[715]) * s.v[715]));s.b[1153] = (p[52] > 0.0);s.store_scalar(1153, if s.b[1153] { 1.0 } else { 0.0 });
        if s.b[1153] {s.store_primal_sqrt_ad(734, A::mul3_scaled_output(s.ad_value(729), s.ad_value(729), s.ad_value(728), s.v[715]));s.store_primal_mul_scaled_powf_rhs(735, 773, 0.75, 734, 0.6666666666666666);s.store_primal_add(728, 728, 735);s.store_primal_mul_scale_offset_mixed_ia(729, 729, A::div_scaled_inputs(s.ad_value(735), (2.0 * 0.6666666666666666), s.ad_value(734), 1.0), 1.0, 1.0);}
        s.store_primal_sqrt(736, 728);s.store_primal_scale(737, 728, 0.95);s.store_primal_scaled_mul(738, 728, 728, 0.0025);s.copy_ad(739, 738);s.store_primal_scaled_sqrt(740, 739, 0.5);s.store_primal_add_scaled_inputs3_sqrt_third_mixed_iia(741, 737, 0.5, 740, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(737), s.ad_value(740))), s.ad_value(738)), (-0.5));s.store_primal_scaled_offset(742, 728, s.v[362], 0.5);s.store_primal_sub_mixed_ai(743, A::sqrt(A::add(s.ad_value(185), s.ad_value(728))), 736);s.store_primal_add_scaled_inputs3_sqrt_first_mixed_aii(744, A::add_scaled_inputs3(s.ad_value(185), 1.0, s.ad_value(186), 1.0, s.ad_value(728), 1.0), 1.0, 736, (-1.0), 743, -1.0);s.store_primal_add_scaled_inputs3_offset_mixed_iia(745, 187, 1.0, 256, 1.0, A::ln_scaled_input(A::mul(s.ad_value(772), A::powf(s.ad_value(363), (-0.75))), 4e-26), (2.0 * s.v[715]), s.v[362]);
        if (!(s.v[745] > 0.05)) {s.store_scalar(745, 0.05);}
        s.store_primal_div_mixed_ai(746, A::sqrt_scaled_input(s.ad_value(772), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[361])), 769);s.b[1154] = (p[52] > 0.0);s.store_scalar(1154, if s.b[1154] { 1.0 } else { 0.0 });
        if s.b[1154] {s.store_primal_sqrt_ad(734, A::mul3_scaled_output(s.ad_value(746), s.ad_value(746), s.ad_value(745), s.v[715]));s.store_primal_mul_scaled_powf_rhs(735, 773, 0.75, 734, 0.6666666666666666);s.store_primal_add(745, 745, 735);s.store_primal_mul_scale_offset_mixed_ia(746, 746, A::div_scaled_inputs(s.ad_value(735), (2.0 * 0.6666666666666666), s.ad_value(734), 1.0), 1.0, 1.0);}
        s.store_primal_scale(747, 745, 0.95);s.store_primal_scaled_mul(748, 745, 745, 0.0025);s.copy_ad(749, 748);s.store_primal_scaled_sqrt(740, 749, 0.5);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_primal_add_scaled_inputs3_sqrt_third_mixed_iia(750, 747, 0.5, 740, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(747), s.ad_value(740))), s.ad_value(748)), (-0.5));s.store_primal_offset_add_scaled_product_mixed_iia(700, 177, 1.0, 178, A::scale_offset(s.ad_value(179), s.v[358], 1.0), s.v[358], s.v[21]);s.store_primal_exp_scaled_input(751, 180, s.v[360]);s.store_primal_mul(701, 189, 751);s.store_primal_scale(702, 190, 1.0 / (s.v[359]));s.store_primal_exp_scaled_input(752, 203, s.v[360]);s.store_primal_mul(703, 202, 752);s.store_primal_scaled_mul(716, 703, 769, s.v[20]);s.store_primal_mul_mixed_ia(705, 206, A::exp_scaled_input(s.ad_value(207), s.v[360]));s.store_primal_exp_scaled_input(753, 205, s.v[360]);s.store_primal_mul(704, 204, 753);s.store_primal_mul_mixed_ia(707, 210, A::exp_scaled_input(s.ad_value(211), s.v[360]));s.store_primal_exp_scaled_input(754, 209, s.v[360]);s.store_primal_mul(706, 208, 754);s.store_primal_exp_scaled_input(755, 213, s.v[360]);s.store_primal_mul(708, 212, 755);s.store_primal_exp_scaled_input(756, 216, s.v[360]);s.store_primal_mul(709, 215, 756);s.store_primal_scaled_mul(757, 716, 709, 2.0);s.store_primal_exp_scaled_input(758, 220, s.v[360]);s.store_primal_mul(720, 219, 758);s.store_primal_mul(721, 258, 758);s.store_primal_mul_mixed_ia(712, 230, A::exp_scaled_input(s.ad_value(231), (-s.v[360])));s.store_primal_scale(719, 276, (4.0 * (1.3806505e-23 * s.v[356])));s.b[1155] = ((p[46] != 0.0) && (s.v[287] > 0.0));s.store_scalar(1155, if s.b[1155] { 1.0 } else { 0.0 });
        if s.b[1155] {s.store_primal_offset_add_scaled_inputs_indices(713, 282, 1.0, 283, s.v[358], s.v[23]);s.store_primal_exp_scaled_input(759, 288, s.v[360]);s.store_primal_mul(714, 287, 759);s.store_primal_scaled_mul(717, 714, 769, s.v[22]);s.store_primal_offset_scaled(723, 286, ((s.v[359]) * (s.v[715])), s.v[715]);s.store_primal_add_scaled_product_mixed_aia(760, A::offset(s.ad_value(284), s.v[362]), 1.0, 723, A::ln_scaled_input(A::mul(s.ad_value(285), A::powf(s.ad_value(363), (-0.75))), 4e-26), 2.0);}
        if s.b[1155] {
            if (s.v[760] > 0.05) {
            } else {
                s.store_scalar(760, 0.05);
            }
        }
        if s.b[1155] {s.store_primal_div_mixed_ai(761, A::sqrt_scaled_input(s.ad_value(285), (((2.0 * 1.6021918e-19) * s.v[767]) * s.v[361])), 769);s.store_primal_square(724, 761);s.store_primal_ln(725, 724);s.store_primal_scale(762, 760, 0.95);s.store_primal_scaled_mul(763, 760, 760, 0.0025);s.copy_ad(764, 763);s.store_primal_scaled_sqrt(765, 764, 0.5);s.store_primal_add_scaled_inputs3_sqrt_third_mixed_iia(766, 762, 0.5, 765, ((-1.0) * 0.5), A::add(A::square(A::sub(s.ad_value(762), s.ad_value(765))), s.ad_value(763)), (-0.5));}
        if (!s.b[1155]) {s.store_scalar(713, 0.0);s.store_scalar(759, 1.0);s.store_scalar(714, 0.0);s.store_scalar(717, 0.0);s.store_scalar(723, s.v[715]);s.store_scalar(760, 0.0);s.store_scalar(761, 1.0);s.store_scalar(724, 1.0);s.store_scalar(725, 0.0);s.store_scalar(762, 0.0);s.store_scalar(763, 0.0);s.store_scalar(764, 0.0);s.store_scalar(765, 0.0);s.store_scalar(766, 0.0);}
        s.store_primal_div_from_scalar(795, 1.0, 246);s.store_primal_scaled_sqrt_scaled_input(796, 246, ((2.0 * 1.6021918e-19) * 9.1093826e-31), ((4.0 * 0.3333333333333333) * 9.482522800157122e33));s.store_primal_mul(797, 796, 181);s.store_primal_mul(798, 796, 192);s.store_primal_mul(799, 796, 193);s.store_scalar(800, 0.0);s.b[1156] = (s.v[241] < 0.0);s.store_scalar(1156, if s.b[1156] { 1.0 } else { 0.0 });
        if s.b[1156] {s.store_primal_div_scaled_inputs_indices(800, 240, (-0.495), 241, 1.0);}
        s.store_scalar(801, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1157] = (s.v[243] < 0.0);s.store_scalar(1157, if s.b[1157] { 1.0 } else { 0.0 });
        if s.b[1157] {s.store_primal_div_scaled_inputs_indices(801, 242, (-0.495), 243, 1.0);}
        s.b[1158] = (s.v[245] < 0.0);s.store_scalar(1158, if s.b[1158] { 1.0 } else { 0.0 });
        if s.b[1158] {s.store_primal_div_scaled_inputs_indices(802, 244, (-0.495), 245, 1.0);}
        s.store_primal_pow_from_scalar_ad(803, s.v[352], s.ad_value(239));s.store_primal_mul(236, 236, 803);s.store_primal_mul(237, 237, 803);s.store_primal_mul(238, 238, 803);
        if ((1.0 + (s.v[251] * s.v[353])) > 0.0) {
            s.store_primal_offset_scaled(796, 251, s.v[353], 1.0);
        } else {
            s.store_scalar(796, 0.0);
        }
        s.store_primal_mul(710, 249, 796);s.store_primal_scaled_mul(806, 710, 192, 500000000.0);
        if ((1.0 + (s.v[252] * s.v[353])) > 0.0) {
            s.store_primal_offset_scaled(796, 252, s.v[353], 1.0);
        } else {
            s.store_scalar(796, 0.0);
        }
        s.store_primal_mul(711, 250, 796);s.store_primal_scaled_mul(807, 711, 193, 500000000.0);s.store_scalar(808, 0.0);s.b[1159] = (s.v[272] > 1e-10);s.store_scalar(1159, if s.b[1159] { 1.0 } else { 0.0 });
        if s.b[1159] {s.store_primal_div_from_scalar(808, 0.75, 272);}
        s.store_primal_square(809, 273);s.store_primal_scale(24, 6, s.v[646]);s.store_primal_scale(25, 6, s.v[647]);s.store_primal_scale(26, 6, s.v[648]);s.store_primal_scale(27, 6, s.v[673]);s.store_primal_scale(28, 6, s.v[674]);s.store_primal_scale(29, 6, s.v[675]);s.store_scalar(30, 0.0);s.b[1167] = (p[43] == 3.0);s.store_scalar(1167, if s.b[1167] { 1.0 } else { 0.0 });
        if s.b[1167] {s.store_scalar(30, 1.0);}
        s.copy_ad(31, 313);s.b[1168] = (p[39] == 0.0);s.store_scalar(1168, if s.b[1168] { 1.0 } else { 0.0 });
        if s.b[1168] {s.store_scalar(31, (if (s.v[14] > 0.0) { s.v[14] } else { 0.0 }));}
        s.b[1169] = ((p[43] == 2.0) || (p[43] == 3.0));s.store_scalar(1169, if s.b[1169] { 1.0 } else { 0.0 });
        if s.b[1169] {s.store_primal_scale(24, 6, s.v[649]);s.store_primal_add_scaled_product_indices(25, 6, s.v[650], 30, 31, (-1.0));s.copy_ad(26, 31);s.store_primal_scale(27, 6, s.v[676]);s.store_primal_add_scaled_product_indices(28, 6, s.v[677], 30, 31, (-1.0));s.copy_ad(29, 31);}
        s.b[1170] = (((p[43] == 1.0) || (p[43] == 2.0)) || (p[43] == 3.0));s.store_scalar(1170, if s.b[1170] { 1.0 } else { 0.0 });
        if s.b[1170] {
            if (s.v[24] > 0.0) {
                s.copy_ad(646, 24);
            } else {
                s.store_scalar(646, 0.0);
            }
        }
        if s.b[1170] {
            if (s.v[25] > 0.0) {
                s.copy_ad(647, 25);
            } else {
                s.store_scalar(647, 0.0);
            }
        }
        if s.b[1170] {
            if (s.v[26] > 0.0) {
                s.copy_ad(648, 26);
            } else {
                s.store_scalar(648, 0.0);
            }
        }
        if s.b[1170] {
            if (s.v[27] > 0.0) {
                s.copy_ad(673, 27);
            } else {
                s.store_scalar(673, 0.0);
            }
        }
        if s.b[1170] {
            if (s.v[28] > 0.0) {
                s.copy_ad(674, 28);
            } else {
                s.store_scalar(674, 0.0);
            }
        }
        if s.b[1170] {
            if (s.v[29] > 0.0) {
                s.copy_ad(675, 29);
            } else {
                s.store_scalar(675, 0.0);
            }
        }
        if (!s.b[1170]) {s.store_scalar(646, 0.0);s.store_scalar(647, 0.0);s.store_scalar(648, 0.0);s.store_scalar(673, 0.0);s.store_scalar(674, 0.0);s.store_scalar(675, 0.0);}
        s.store_scalar(656, 0.0);s.store_scalar(683, 0.0);s.store_scalar(658, 0.0);s.store_scalar(685, 0.0);s.store_scalar(657, 0.0);s.store_scalar(684, 0.0);s.store_scalar(659, 0.0);s.store_scalar(686, 0.0);s.store_scalar(654, 0.0);s.store_scalar(681, 0.0);s.store_scalar(655, 0.0);s.store_scalar(682, 0.0);s.store_scalar(651, 1.0);s.store_scalar(678, 1.0);s.store_scalar(652, 1.0);s.store_scalar(679, 1.0);s.store_scalar(653, 1.0);s.store_scalar(680, 1.0);s.store_scalar(501, 0.0);s.b[1171] = (p[43] > 0.0);s.store_scalar(1171, if s.b[1171] { 1.0 } else { 0.0 });s.b[1172] = ((s.v[387] * s.v[646]) > 0.0);s.store_scalar(1172, if s.b[1172] { 1.0 } else { 0.0 });
    }
}

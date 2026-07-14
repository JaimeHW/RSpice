#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_mul_sqrt_mixed_ia(701, 700, A::div_scaled_inputs(s.ad_value(778), 2.0, s.ad_value(478), (1.60219e-19 * 1000000.0)));s.store_sqrt(702, 701);s.b[937] = (s.v[68] == 0.0);s.store_scalar(937, if s.b[937] { 1.0 } else { 0.0 });
        if s.b[937] {s.store_sqrt_scaled_input_ad(489, A::mul(A::div_from_scalar((3.0 * 3.9), s.ad_value(777)), s.ad_value(608)), s.v[91]);}
        if (!s.b[937]) {s.store_sqrt_ad(489, A::div_scaled_product3(s.ad_value(778), s.ad_value(608), s.ad_value(776), 1.0, s.ad_value(777), 8.85418e-12));}
        s.store_mul_mixed_ia(485, 409, {
                    if (((1e20 * s.v[478]) / (s.v[817] * s.v[817])) > 1e-38) {
                        A::ln(A::div_scaled_inputs(s.ad_value(478), 1e20, A::square(s.ad_value(817)), 1.0))
                    } else {
                        A::neg(A::constant(87.49823353377374))
                    }
                });
        s.store_sqrt_ad(728, A::div_scaled_product(s.ad_value(778), s.ad_value(478), (1.60219e-19 * (1000000.0 * 0.5)), s.ad_value(488), 1.0));s.b[938] = (s.v[68] == 0.0);s.store_scalar(938, if s.b[938] { 1.0 } else { 0.0 });s.b[939] = (s.v[480] > 0.0);s.store_scalar(939, if s.b[939] { 1.0 } else { 0.0 });
        if (s.b[938] && s.b[939]) {
            s.store_mul_mixed_ia(736, 831, {
                            if ((s.v[480] / 1e20) > 1e-38) {
                                A::ln_scaled_input(s.ad_value(480), 1.0 / (1e20))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if (s.b[938] && (!s.b[939])) {s.store_scalar(736, 0.0);}
        if (!s.b[938]) {
            s.store_mul_mixed_ia(818, 831, {
                            if ((s.v[481] / s.v[817]) > 1e-38) {
                                A::ln(A::div(s.ad_value(481), s.ad_value(817)))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if (!s.b[938]) {s.store_scale(819, 816, 0.5);}
        s.b[940] = (s.v[818] > s.v[819]);s.store_scalar(940, if s.b[940] { 1.0 } else { 0.0 });
        if ((!s.b[938]) && s.b[940]) {s.copy_ad(818, 819);}
        if (!s.b[938]) {s.store_sub_scaled_inputs_mixed_ai(820, A::offset(s.ad_value(819), s.v[80]), 1.0, 818, s.v[36]);s.store_sub_from_scalar(736, s.v[79], 820);}
        s.store_scalar(729, (((((s.v[360] * (if ((s.v[361] / s.v[357]) > 1e-38) { (((s.v[361] / s.v[357])) as f64).ln() } else { (-87.49823353377374) }))) as f64).exp() / s.v[357]) / s.v[357]));
        s.store_div_scaled_value_by_product_mixed_aii(732, A::exp_scaled_input({
            if ((s.v[361] / (s.v[357] * s.v[580])) > 1e-38) {
                A::ln(A::div_from_scalar(s.v[361], A::scale(s.ad_value(580), s.v[357])))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, s.v[360]), (1.0 / (s.v[357]) * 1.0 / (s.v[357])), 580, 580, 1.0);
        if (s.v[36] == 1.0) {
            s.copy_ad(730, 789);
        } else {
            s.copy_ad(730, 788);
        }
        if (s.v[36] == 1.0) {
            s.copy_ad(731, 791);
        } else {
            s.copy_ad(731, 790);
        }
        s.store_mul3_affine_lhs(733, 730, 581, ((s.v[689] / s.v[59]) + s.v[61]), 0.0, 732);s.store_mul3_affine_lhs(734, 730, 581, ((s.v[689] / s.v[59]) + s.v[60]), 0.0, 732);s.store_scaled_mul(735, 731, 580, (-s.v[357]));s.store_scale(730, 730, (s.v[729] * (((s.v[689] / s.v[59]) * s.v[688]) + (s.v[64] / s.v[39]))));s.store_primal_scale(731, 731, (-s.v[357]));s.b[941] = (param_given[89] || param_given[93]);s.store_scalar(941, if s.b[941] { 1.0 } else { 0.0 });s.b[942] = (!param_given[89]);s.store_scalar(942, if s.b[942] { 1.0 } else { 0.0 });
        if (s.b[941] && s.b[942]) {s.store_scalar(490, 0.53);}
        s.b[943] = (!param_given[93]);s.store_scalar(943, if s.b[943] { 1.0 } else { 0.0 });
        if (s.b[941] && s.b[943]) {s.store_scalar(494, (-0.0186));}
        s.b[949] = (!param_given[86]);s.store_scalar(949, if s.b[949] { 1.0 } else { 0.0 });
        if (((!s.b[941]) && s.b[949]) && (s.v[68] != 0.0)) {s.store_scaled_div_from_scalar_ad(818, 1.60219e-19, A::scale(s.ad_value(778), 2.0), 1000000.0);}
        if (((!s.b[941]) && s.b[949]) && (s.v[68] == 0.0)) {s.store_scalar(818, 0.00077348);}
        if ((!s.b[941]) && s.b[949]) {s.store_add_scaled_product_indices(484, 488, 1.0, 818, 478, (-(s.v[487] * s.v[487])));}
        s.b[950] = (s.v[484] > 0.0);s.store_scalar(950, if s.b[950] { 1.0 } else { 0.0 });
        if ((!s.b[941]) && s.b[950]) {s.store_neg(484, 484);}
        s.b[951] = (s.v[486] > 0.0);s.store_scalar(951, if s.b[951] { 1.0 } else { 0.0 });
        if ((!s.b[941]) && s.b[951]) {s.store_scalar(486, (-s.v[486]));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[952] = (!param_given[84]);s.store_scalar(952, if s.b[952] { 1.0 } else { 0.0 });
        if ((!s.b[941]) && s.b[952]) {s.store_div_scaled_product_mixed_iai(482, 780, A::sqrt(s.ad_value(478)), 1.0, 757, 1.0);}
        s.b[953] = (!param_given[85]);s.store_scalar(953, if s.b[953] { 1.0 } else { 0.0 });
        if ((!s.b[941]) && s.b[953]) {s.store_div_scaled_product_mixed_iai(483, 780, A::sqrt(s.ad_value(479)), 1.0, 757, 1.0);}
        if (!s.b[941]) {s.store_sub(818, 482, 483);s.store_sub_mixed_ai(819, A::sqrt(A::sub(s.ad_value(488), s.ad_value(484))), 700);s.store_mul_sub_mixed_iai(820, 700, A::sqrt(A::sub(s.ad_value(488), s.ad_value(486))), 700);s.store_div_scaled_product_add_scaled_denominator_indices(494, 818, 819, 1.0, 820, 2.0, 486, 1.0, 1.0);s.store_add_scaled_product_mixed_iia(490, 483, 1.0, 494, A::sqrt(A::sub(s.ad_value(488), s.ad_value(486))), (-2.0));}
        s.store_offset(818, 628, s.v[689]);s.b[954] = (s.v[818] < 1e-8);s.store_scalar(954, if s.b[954] { 1.0 } else { 0.0 });
        if s.b[954] {s.store_scalar(818, 1e-8);}
        s.store_mul_scale_offset_mixed_ia(707, 490, A::div(s.ad_value(627), s.ad_value(818)), 1.0, 1.0);s.b[955] = (!param_given[108]);s.store_scalar(955, if s.b[955] { 1.0 } else { 0.0 });s.b[956] = (param_given[107] || param_given[106]);s.store_scalar(956, if s.b[956] { 1.0 } else { 0.0 });
        if (s.b[955] && s.b[956]) {s.store_add_scaled_inputs_product_indices(522, 507, s.v[36], 488, (-1.0), 707, 700, (-1.0));}
        if (s.b[955] && (!s.b[956])) {s.store_scalar(522, (-1.0));}
        s.b[957] = (!param_given[107]);s.store_scalar(957, if s.b[957] { 1.0 } else { 0.0 });
        if s.b[957] {s.store_add_scaled_inputs_product_indices(507, 522, s.v[36], 488, s.v[36], 707, 700, s.v[36]);}
        s.store_scale(737, 707, (s.v[91] * 1.0 / (s.v[93])));s.store_mul(819, 758, 702);s.store_ad_value(818, A::exp_div_scaled_inputs(s.ad_value(506), ((-0.5) * s.v[688]), s.ad_value(819), 1.0));s.store_add_scaled_product_indices(703, 818, 1.0, 818, 818, 2.0);s.store_ad_value(818, A::exp_div_scaled_inputs(s.ad_value(505), ((-0.5) * s.v[688]), s.ad_value(819), 1.0));s.store_add_scaled_product_indices(820, 818, 1.0, 818, 818, 2.0);s.store_add_scaled_product_indices(704, 562, 1.0, 561, 820, 1.0);s.store_div_mixed_ia(752, 741, A::exp_scaled_input(s.ad_value(742), (if (s.v[688] > 1e-38) { ((s.v[688]) as f64).ln() } else { (-87.49823353377374) })));s.b[958] = (s.v[248] < 0.0);s.store_scalar(958, if s.b[958] { 1.0 } else { 0.0 });
        if s.b[958] {s.store_scalar(248, 0.0);}
        s.store_scalar(818, ((s.v[825]) as f64).powf(s.v[253]));s.store_primal_offset(841, 248, s.v[826]);s.store_powf(819, 841, s.v[254]);s.store_add_ad(813, A::offset(A::div_from_scalar(p.p231, s.ad_value(819)), (p.p230 / s.v[818])), A::div_from_scalar(p.p232, A::scale(s.ad_value(819), s.v[818])));s.store_offset(597, 813, 1.0);s.store_scalar(818, ((s.v[825]) as f64).powf(s.v[255]));s.store_powf(819, 841, s.v[256]);s.store_add_ad(813, A::offset(A::div_from_scalar(p.p234, s.ad_value(819)), (p.p233 / s.v[818])), A::div_from_scalar(p.p235, A::scale(s.ad_value(819), s.v[818])));s.store_offset(598, 813, 1.0);s.store_sqrt_square_offset(598, 598, 1e-9);s.store_scalar(818, (s.v[827] - 1.0));s.store_offset_scaled(599, 597, (1.0 + (s.v[252] * s.v[818])), 1e-9);s.store_scalar(835, (1.0 / (s.v[246] + (0.5 * s.v[825]))));s.store_scalar(836, (1.0 / (s.v[247] + (0.5 * s.v[825]))));s.store_scalar(601, (s.v[835] + s.v[836]));s.store_scale_ad(600, A::div_from_scalar(s.v[249], s.ad_value(599)), s.v[601]);s.b[959] = (((s.v[40] > 0.0) && (s.v[41] > 0.0)) && ((s.v[39] == 1.0) || ((s.v[39] > 1.0) && (s.v[42] > 0.0))));s.store_scalar(959, if s.b[959] { 1.0 } else { 0.0 });
        if s.b[959] {s.store_scalar(837, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
    ) {
        if s.b[959] {s.store_scalar(838, 0.0);}
        s.b[960] = (s.v[250] < (-1.0));s.store_scalar(960, if s.b[960] { 1.0 } else { 0.0 });
        if (s.b[959] && s.b[960]) {s.store_scalar(250, (-1.0));}
        s.b[961] = (s.v[250] > 1.0);s.store_scalar(961, if s.b[961] { 1.0 } else { 0.0 });
        if ((s.b[959] && (!s.b[960])) && s.b[961]) {s.store_scalar(250, 1.0);}
        if ((s.b[959] && (!s.b[960])) && (!s.b[961])) {
        }
        if s.b[959] {s.store_scalar(847, 0.0);}
        let mut t31: usize = 0;
        while {
            let t30: f64 = if (s.b[959] && (s.v[847] < s.v[39])) { 1.0 } else { 0.0 };
            t30 != 0.0
        } {
            t31 += 1;assert!(t31 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if s.b[959] {s.store_primal_div_from_scalar_offset_scaled_input(962, (1.0 / s.v[39]), 847, (s.v[42] + s.v[825]), (s.v[40] + (0.5 * s.v[825])));s.store_primal_div_from_scalar_offset_scaled_input(963, (1.0 / s.v[39]), 847, (s.v[42] + s.v[825]), (s.v[41] + (0.5 * s.v[825])));s.store_primal_add(837, 837, 962);s.store_primal_add(838, 838, 963);s.store_primal_offset(847, 847, 1.0);}
        }
        if s.b[959] {s.store_primal_add(842, 837, 838);s.copy_ad(414, 842);s.store_mul_div_from_scalar_lhs_ad_indices(839, s.v[249], 599, 842);s.store_div_scaled_offset_numerator_mixed_ia(818, 839, 1.0, 1.0, A::offset(s.ad_value(600), 1.0), 1.0);s.store_mul(765, 698, 818);s.store_div_scaled_offset_numerator(819, A::mul(s.ad_value(250), s.ad_value(839)), 1.0, 1.0, A::offset(A::mul(s.ad_value(250), s.ad_value(600)), 1.0), 1.0);s.store_mul(767, 699, 819);s.store_primal_offset(843, 842, (-s.v[601]));s.store_mul_div_from_scalar_lhs_ad_indices(840, s.v[251], 598, 843);s.store_mul_div_from_scalar_lhs_ad_mixed_ai(844, s.v[257], A::powf(s.ad_value(598), s.v[258]), 843);s.store_mul_div_from_scalar_lhs_ad_mixed_ai(845, s.v[259], A::powf(s.ad_value(598), s.v[260]), 843);s.store_mul_div_from_scalar_lhs_ad_mixed_ai(846, s.v[261], A::powf(s.ad_value(598), s.v[262]), 843);s.store_add(768, 507, 840);s.store_add(763, 494, 844);s.store_add(761, 556, 845);s.store_add(762, 558, 846);}
        if (!s.b[959]) {s.copy_ad(765, 698);s.copy_ad(768, 507);s.copy_ad(767, 699);s.copy_ad(763, 494);s.copy_ad(761, 556);s.copy_ad(762, 558);s.store_scalar(414, 0.0);s.store_scalar(601, 0.0);s.store_scalar(250, 0.0);}
        s.store_scale(764, 763, (s.v[91] * 1.0 / (s.v[93])));s.store_offset(768, 768, s.v[56]);s.store_offset(766, 522, (s.v[36] * s.v[56]));s.store_scalar(430, (s.v[753] * s.v[44]));s.store_scale(432, 336, s.v[44]);s.store_scalar(431, (s.v[753] * s.v[43]));s.store_scale(433, 336, s.v[43]);s.b[964] = (s.v[336] > 0.0);s.store_scalar(964, if s.b[964] { 1.0 } else { 0.0 });s.b[965] = (((s.v[479] > 0.0) && (s.v[36] > 0.0)) || ((s.v[479] < 0.0) && (s.v[36] < 0.0)));s.store_scalar(965, if s.b[965] { 1.0 } else { 0.0 });
        if (s.b[964] && s.b[965]) {s.store_sub(818, 684, 683);s.store_add_scaled_inputs(545, 683, 1.0, 818, s.v[337]);s.store_sub_from_scalar(819, s.v[430], 432);s.store_div_scaled_value_by_product_indices(820, 819, 1.0, 818, 818, 1.0);s.store_scale(546, 820, 1.0 / (s.v[337]));s.store_scale(547, 820, 1.0 / ((1.0 - s.v[337])));s.store_add_scaled_products_indices(434, 818, 819, ((1.0 + s.v[337]) * 0.3333333333333333), 432, 683, (-1.0));s.store_sub_from_scalar(819, s.v[431], 433);s.store_div_scaled_value_by_product_indices(820, 819, 1.0, 818, 818, 1.0);s.store_scale(548, 820, 1.0 / (s.v[337]));s.store_scale(549, 820, 1.0 / ((1.0 - s.v[337])));s.store_add_scaled_products_indices(435, 818, 819, ((1.0 + s.v[337]) * 0.3333333333333333), 433, 683, (-1.0));}
        if (s.b[964] && (!s.b[965])) {s.store_sub(818, 683, 684);s.store_add_scaled_inputs(545, 684, 1.0, 818, s.v[337]);s.store_offset(819, 432, (-s.v[430]));s.store_div_scaled_value_by_product_indices(820, 819, 1.0, 818, 818, 1.0);s.store_scale(546, 820, 1.0 / (s.v[337]));s.store_scale(547, 820, 1.0 / ((1.0 - s.v[337])));s.store_add_scaled_product_indices(434, 684, (-s.v[430]), 818, 819, ((1.0 + s.v[337]) * 0.3333333333333333));s.store_offset(819, 433, (-s.v[431]));s.store_div_scaled_value_by_product_indices(820, 819, 1.0, 818, 818, 1.0);s.store_scale(548, 820, 1.0 / (s.v[337]));s.store_scale(549, 820, 1.0 / ((1.0 - s.v[337])));s.store_add_scaled_product_indices(435, 684, (-s.v[431]), 818, 819, ((1.0 + s.v[337]) * 0.3333333333333333));}
        if (!s.b[964]) {s.store_scalar(545, 0.0);s.store_scalar(546, 0.0);s.store_scalar(547, 0.0);s.store_scalar(434, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
    ) {
        if (!s.b[964]) {s.store_scalar(548, 0.0);s.store_scalar(549, 0.0);s.store_scalar(435, 0.0);}
        s.b[966] = ((s.v[354] < 1.0) || (s.v[354] > 2.0));s.store_scalar(966, if s.b[966] { 1.0 } else { 0.0 });
        if s.b[966] {s.store_scalar(354, 1.0);}
        s.store_scale_ad(818, {
            if ((s.v[354] * (1.0 + (s.v[174] / s.v[173]))) > 1e-38) {
                A::ln_scaled_input(s.ad_value(354), (1.0 + (s.v[174] / s.v[173])))
            } else {
                A::neg(A::constant(87.49823353377374))
            }
        }, s.v[338]);s.store_scalar(819, (s.v[46] - s.v[38]));s.b[967] = (s.v[819] > 0.0);s.store_scalar(967, if s.b[967] { 1.0 } else { 0.0 });
        if s.b[967] {s.store_scale(428, 818, s.v[819]);}
        if (!s.b[967]) {s.store_scalar(428, 0.0);}
        s.store_scalar(819, (s.v[45] - s.v[38]));s.b[968] = (s.v[819] > 0.0);s.store_scalar(968, if s.b[968] { 1.0 } else { 0.0 });
        if s.b[968] {s.store_scale(429, 818, s.v[819]);}
        if (!s.b[968]) {s.store_scalar(429, 0.0);}
        s.store_scalar(423, (s.v[155] * s.v[47]));s.b[969] = (s.v[423] <= 0.001);s.store_scalar(969, if s.b[969] { 1.0 } else { 0.0 });
        if s.b[969] {s.store_scalar(423, 0.001);}
        s.store_scalar(422, (s.v[155] * s.v[48]));s.b[970] = (s.v[422] <= 0.001);s.store_scalar(970, if s.b[970] { 1.0 } else { 0.0 });
        if s.b[970] {s.store_scalar(422, 0.001);}
        s.b[971] = (s.v[317] < 1e-15);s.store_scalar(971, if s.b[971] { 1.0 } else { 0.0 });
        if s.b[971] {s.store_scalar(317, 1e-15);}
        s.store_div_scalar_by_product_indices(818, (((-0.5) * s.v[688]) * s.v[688]), 317, 317, 1.0);s.b[972] = (s.v[818] > 100.0);s.store_scalar(972, if s.b[972] { 1.0 } else { 0.0 });
        if s.b[972] {s.store_scaled_offset(819, 818, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[973] = (s.v[818] < (-100.0));s.store_scalar(973, if s.b[973] { 1.0 } else { 0.0 });
        if ((!s.b[972]) && s.b[973]) {s.store_scalar(819, 3.720075976e-44);}
        if ((!s.b[972]) && (!s.b[973])) {s.store_exp(819, 818);}
        s.copy_ad(712, 819);s.store_mul_scale_offset_mixed_ia(818, 680, A::div_from_scalar(1.0, s.ad_value(317)), 1.0, (1.0 / s.v[688]));s.store_pow_indices(713, 818, 679);s.store_offset_scaled_ad(714, A::pow(s.ad_value(818), s.ad_value(616)), s.v[324], 1.0);s.store_add_scaled_inputs(715, 681, 1.0, 682, s.v[688]);s.b[974] = (s.v[715] < 1.0);s.store_scalar(974, if s.b[974] { 1.0 } else { 0.0 });
        if s.b[974] {s.store_scalar(715, 1.0);}
        s.b[975] = (s.v[68] == 0.0);s.store_scalar(975, if s.b[975] { 1.0 } else { 0.0 });
        if s.b[975] {s.store_scalar(92, (s.v[91] - s.v[94]));}
        if (!s.b[975]) {s.store_scalar(850, (8.617087e-5 * s.v[84]));s.copy_ad(851, 850);}
        if (!s.b[975]) {
            s.store_mul_mixed_ia(852, 850, {
                            if (((1e20 * s.v[478]) / (s.v[817] * s.v[817])) > 1e-38) {
                                A::ln(A::div_scaled_inputs(s.ad_value(478), 1e20, A::square(s.ad_value(817)), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if (!s.b[975]) {
            s.store_mul_scale_offset_mixed_ia(853, 850, {
                if ((s.v[478] / s.v[817]) > 1e-38) {
                    A::ln(A::div(s.ad_value(478), s.ad_value(817)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 2.0, 0.0);
        }
        if (!s.b[975]) {s.store_sqrt(854, 853);s.store_add(814, 766, 853);s.store_scalar(855, (s.v[36] * s.v[83]));s.store_scalar(818, (s.v[87] * 8.85418e-12));}
        s.b[976] = ((((s.v[480] > 1e18) && (s.v[480] < 1e25)) && (s.v[855] > s.v[814])) && (s.v[818] != 0.0));s.store_scalar(976, if s.b[976] { 1.0 } else { 0.0 });
        if ((!s.b[975]) && s.b[976]) {s.store_div_scaled_product_mixed_iia(819, 778, 480, (1000000.0 * 1.60219e-19), A::square(s.ad_value(757)), 1.0);s.store_sqrt_offset_ad(822, A::div_scaled_inputs2(s.ad_value(855), 2.0, s.ad_value(818), (-2.0), s.ad_value(819), 1.0), 1.0);s.store_mul_scale_offset_indices(820, 819, 822, 1.0, (-1.0));s.store_div_scaled_product_indices(821, 820, 820, 0.5, 819, 1.0);s.store_offset_sub(884, 782, 821, (-0.05));s.store_sqrt_square_offset(824, 884, 0.224);s.store_add_scaled_inputs3_indices(823, 782, 1.0, 884, (-0.5), 824, (-0.5));s.store_sub(856, 855, 823);}
        if ((!s.b[975]) && (!s.b[976])) {s.copy_ad(856, 855);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
    ) {
        if (!s.b[975]) {s.store_sub(858, 852, 853);s.copy_ad(821, 702);s.store_mul(861, 758, 821);s.store_mul(862, 758, 821);s.store_div_scaled_inputs_indices(818, 500, ((-0.5) * s.v[81]), 861, 1.0);}
        s.b[977] = (s.v[818] > (-100.0));s.store_scalar(977, if s.b[977] { 1.0 } else { 0.0 });
        if ((!s.b[975]) && s.b[977]) {s.store_exp(819, 818);s.store_mul_scale_offset_rhs(875, 819, 819, 2.0, 1.0);}
        if ((!s.b[975]) && (!s.b[977])) {s.store_scalar(819, 3.720075976e-44);s.store_mul_scale_offset_rhs(875, 819, 819, 2.0, 1.0);}
        if (!s.b[975]) {s.store_div_scaled_product_indices(820, 470, 778, 1.0, 701, 1.0);s.copy_ad(821, 466);s.store_div_scaled_inputs2_mixed_aii(822, A::add_scaled_product(s.ad_value(820), 1.0, s.ad_value(821), s.ad_value(875), 1.0), 1.0, 469, 1.0, 757, 1.0);}
        s.b[978] = (s.v[822] >= (-0.5));s.store_scalar(978, if s.b[978] { 1.0 } else { 0.0 });
        if ((!s.b[975]) && s.b[978]) {s.store_offset(864, 822, 1.0);}
        if ((!s.b[975]) && (!s.b[978])) {s.store_div_from_scalar_offset_scaled_input(818, 1.0, 822, 8.0, 3.0);s.store_mul_scale_offset_rhs(864, 818, 822, 3.0, 1.0);}
        s.b[979] = (s.v[739] > 0.0);s.store_scalar(979, if s.b[979] { 1.0 } else { 0.0 });
        if ((!s.b[975]) && s.b[979]) {s.store_offset_scaled(821, 739, 2.0, s.v[81]);}
        if ((!s.b[975]) && s.b[979]) {
            s.store_mul_mixed_ia(822, 851, {
                            if ((s.v[81] / s.v[821]) > 1e-38) {
                                A::ln(A::div_from_scalar(s.v[81], s.ad_value(821)))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if ((!s.b[975]) && s.b[979]) {s.store_mul(872, 864, 822);}
        if ((!s.b[975]) && (!s.b[979])) {s.store_scalar(872, 0.0);}
        if (!s.b[975]) {s.store_mul(411, 499, 875);s.store_mul(876, 411, 858);s.store_div_scaled_inputs_indices(818, 503, ((-0.5) * (s.v[82] * s.v[81])), 862, 1.0);}
        s.b[980] = (s.v[818] > (-100.0));s.store_scalar(980, if s.b[980] { 1.0 } else { 0.0 });
        if ((!s.b[975]) && s.b[980]) {s.store_exp(819, 818);s.store_mul_scale_offset_rhs(820, 819, 819, 2.0, 1.0);}
        if ((!s.b[975]) && (!s.b[980])) {s.store_scalar(819, 3.720075976e-44);s.store_mul_scale_offset_rhs(820, 819, 819, 2.0, 1.0);}
        if (!s.b[975]) {s.store_mul(818, 502, 820);s.store_mul(877, 818, 858);s.store_scalar(863, ((s.v[84] / s.v[150]) - 1.0));s.store_sqrt_offset_scaled_input(818, 498, 1.0 / (s.v[81]), 1.0);s.store_add_scaled_inputs(819, 491, 1.0, 492, 1.0 / (s.v[81]));s.store_add_scaled_product_mixed_aii(873, A::mul3(s.ad_value(737), A::offset(s.ad_value(818), (-1.0)), s.ad_value(854)), 1.0, 819, 863, 1.0);s.store_div_scaled_product_offset_denominator_indices(814, 776, 853, 1.0, 497, s.v[82], 1.0);s.store_scalar(870, 0.0);s.store_scalar(874, 0.0);s.store_sqrt_offset_scaled_input(871, 738, 1.0 / (s.v[81]), 1.0);s.copy_ad(867, 854);s.store_sub_add_scaled_inputs4_lhs_mixed_aiii(859, A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(768), s.v[36], A::add_scaled_products(s.ad_value(737), s.ad_value(867), 1.0, s.ad_value(707), s.ad_value(854), (-1.0)), s.ad_value(871), 1.0), 1.0, s.ad_value(876), (-1.0), s.ad_value(877), -1.0), 1.0, s.ad_value(495), s.ad_value(814), 1.0), 1.0, 873, 1.0, 870, -1.0, 872, -1.0, 874);s.store_sub(860, 856, 859);s.store_mul(849, 864, 851);s.store_div_scaled_product_indices(865, 745, 860, 1.0, 849, 1.0);s.store_div_scaled_inputs2_mixed_iai(866, 521, 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(745), s.ad_value(860)), (-1.0), 849, 1.0);}
        s.b[981] = (s.v[865] > 100.0);s.store_scalar(981, if s.b[981] { 1.0 } else { 0.0 });
        if ((!s.b[975]) && s.b[981]) {s.copy_ad(857, 860);}
        s.b[982] = (s.v[866] > 100.0);s.store_scalar(982, if s.b[982] { 1.0 } else { 0.0 });
        if (((!s.b[975]) && (!s.b[981])) && s.b[982]) {s.store_div_scaled_inputs2_by_product_indices(818, 860, 1.0, 521, (-1.0), 864, 851, 1.0);s.store_exp(868, 818);s.store_mul_div_scaled_product_indices(857, 868, 851, 728, 1.0, 757, 1.0);}
        if (((!s.b[975]) && (!s.b[981])) && (!s.b[982])) {s.store_exp(868, 865);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
    ) {
        if (((!s.b[975]) && (!s.b[981])) && (!s.b[982])) {
            s.store_mul_mixed_ia(819, 849, {
                            if ((1.0 + s.v[868]) > 1e-38) {
                                A::ln(A::offset(s.ad_value(868), 1.0))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if (((!s.b[975]) && (!s.b[981])) && (!s.b[982])) {s.store_mul3_ad(822, A::div_scaled_inputs(s.ad_value(757), -1.0, A::mul(s.ad_value(850), s.ad_value(728)), 1.0), A::exp(s.ad_value(866)), A::sub_from_scalar(1.0, s.ad_value(745)));s.store_sub_mixed_ia(820, 745, A::div_scaled_product(s.ad_value(849), s.ad_value(822), 1.0, A::sub_from_scalar(1.0, s.ad_value(745)), 1.0));s.store_div(857, 819, 820);}
        if (!s.b[975]) {s.store_add_scaled_inputs3_indices(821, 768, s.v[36], 766, (-1.0), 853, -1.0);s.store_scale(869, 821, 4.0);}
        s.b[983] = (s.v[869] < 0.0);s.store_scalar(983, if s.b[983] { 1.0 } else { 0.0 });
        if ((!s.b[975]) && s.b[983]) {s.store_scalar(869, 0.0);}
        if (!s.b[975]) {s.store_scalar(878, 0.0);s.copy_ad(879, 776);s.store_scalar(880, 1000000.0);}
        let mut t2f: usize = 0;
        while {
            let t0: f64 = (s.v[879] - s.v[880]);let ta: f64 = (s.dn[879][0] - s.dn[880][0]);let tb: f64 = (s.dn[879][1] - s.dn[880][1]);let tf: f64 = (s.dn[879][2] - s.dn[880][2]);let t10: f64 = (s.dn[879][3] - s.dn[880][3]);let t11: f64 = (s.dn[879][4] - s.dn[880][4]);let t12: f64 = (s.dn[879][5] - s.dn[880][5]);let t13: f64 = (s.dn[879][6] - s.dn[880][6]);let t14: f64 = (s.dn[879][7] - s.dn[880][7]);let t15: f64 = (s.dn[879][8] - s.dn[880][8]);let t16: f64 = (s.dn[879][9] - s.dn[880][9]);let tc: f64 = (s.dn[879][10] - s.dn[880][10]);let td: f64 = (s.dn[879][11] - s.dn[880][11]);let te: f64 = (s.dn[879][12] - s.dn[880][12]);let t1: f64 = (s.db[879][0] - s.db[880][0]);let t2: f64 = (s.db[879][1] - s.db[880][1]);let t3: f64 = (s.db[879][2] - s.db[880][2]);let t4: f64 = (s.db[879][3] - s.db[880][3]);let t5: f64 = (s.db[879][4] - s.db[880][4]);let t6: f64 = (s.db[879][5] - s.db[880][5]);let t7: f64 = (s.db[879][6] - s.db[880][6]);let t8: f64 = (s.db[879][7] - s.db[880][7]);let t9: f64 = (s.db[879][8] - s.db[880][8]);let t17: f64 = (t0).abs();let t21: f64 = if t0 >= 0.0 { ta } else { (-ta) };let t22: f64 = if t0 >= 0.0 { tb } else { (-tb) };let t26: f64 = if t0 >= 0.0 { tf } else { (-tf) };let t27: f64 = if t0 >= 0.0 { t10 } else { (-t10) };let t28: f64 = if t0 >= 0.0 { t11 } else { (-t11) };let t29: f64 = if t0 >= 0.0 { t12 } else { (-t12) };let t2a: f64 = if t0 >= 0.0 { t13 } else { (-t13) };let t2b: f64 = if t0 >= 0.0 { t14 } else { (-t14) };let t2c: f64 = if t0 >= 0.0 { t15 } else { (-t15) };let t2d: f64 = if t0 >= 0.0 { t16 } else { (-t16) };let t23: f64 = if t0 >= 0.0 { tc } else { (-tc) };let t24: f64 = if t0 >= 0.0 { td } else { (-td) };let t25: f64 = if t0 >= 0.0 { te } else { (-te) };let t18: f64 = if t0 >= 0.0 { t1 } else { (-t1) };let t19: f64 = if t0 >= 0.0 { t2 } else { (-t2) };let t1a: f64 = if t0 >= 0.0 { t3 } else { (-t3) };
            let t1b: f64 = if t0 >= 0.0 { t4 } else { (-t4) };let t1c: f64 = if t0 >= 0.0 { t5 } else { (-t5) };let t1d: f64 = if t0 >= 0.0 { t6 } else { (-t6) };let t1e: f64 = if t0 >= 0.0 { t7 } else { (-t7) };let t1f: f64 = if t0 >= 0.0 { t8 } else { (-t8) };let t20: f64 = if t0 >= 0.0 { t9 } else { (-t9) };let t2e: f64 = if ((!s.b[975]) && ((s.v[878] <= 4.0) && (t17 > 1e-12))) { 1.0 } else { 0.0 };
            t2e != 0.0
        } {
            t2f += 1;assert!(t2f <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (!s.b[975]) {s.copy_ad(880, 879);s.store_scale(814, 879, 200000000.0);s.store_div_scaled_inputs2_indices(984, 857, 1.0, 869, 1.0, 814, 1.0);}
            if (!s.b[975]) {
                s.store_offset_ad(985, A::exp_scaled_input({
                    if (s.v[984] > 1e-38) {
                        A::ln(s.ad_value(984))
                    } else {
                        A::neg(A::constant(87.49823353377374))
                    }
                }, (s.v[86] * 0.7)), 1.0);
            }
            if (!s.b[975]) {s.store_div_from_scalar(881, (s.v[85] * 1.9e-9), 985);s.store_add_scaled_product_indices(879, 776, 1.0, 777, 881, (-1.0 / (s.v[74])));s.store_primal_offset(878, 878, 1.0);}
        }
        if (!s.b[975]) {s.copy_ad(92, 879);}
        s.copy_ad(812, 702);s.store_sub(813, 485, 488);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
    ) {
        s.store_mul(814, 758, 812);s.store_div_scaled_inputs_indices(818, 503, ((-0.5) * (s.v[689] * s.v[688])), 814, 1.0);s.b[986] = (s.v[818] > (-100.0));s.store_scalar(986, if s.b[986] { 1.0 } else { 0.0 });
        if s.b[986] {s.store_exp(819, 818);s.store_mul_scale_offset_rhs(820, 819, 819, 2.0, 1.0);}
        if (!s.b[986]) {s.store_scalar(819, 3.720075976e-44);s.store_mul_scale_offset_rhs(820, 819, 819, 2.0, 1.0);}
        s.store_mul(818, 502, 820);s.store_mul(820, 818, 813);s.store_div_scaled_inputs_indices(818, 500, ((-0.5) * s.v[688]), 814, 1.0);s.b[987] = (s.v[818] > (-100.0));s.store_scalar(987, if s.b[987] { 1.0 } else { 0.0 });
        if s.b[987] {s.store_exp(819, 818);s.store_mul_scale_offset_rhs(821, 819, 819, 2.0, 1.0);}
        if (!s.b[987]) {s.store_scalar(819, 3.720075976e-44);s.store_mul_scale_offset_rhs(821, 819, 819, 2.0, 1.0);}
        s.store_mul3_lhs(821, 499, 821, 813);s.store_div_scaled_product_offset_denominator_indices(822, 92, 488, 1.0, 497, s.v[689], 1.0);s.store_sqrt_offset_scaled_input(818, 498, 1.0 / (s.v[688]), 1.0);s.store_add_scaled_inputs3_mixed_aii(823, A::mul3(s.ad_value(737), A::offset(s.ad_value(818), (-1.0)), s.ad_value(700)), 1.0, 491, (s.v[827] - 1.0), 492, (1.0 / (s.v[688]) * (s.v[827] - 1.0)));s.store_add_mixed_ai(883, A::add_scaled_product(A::add_scaled_inputs3(s.ad_value(507), s.v[36], s.ad_value(820), (-1.0), s.ad_value(821), -1.0), 1.0, s.ad_value(495), s.ad_value(822), 1.0), 823);s.store_add_scaled_inputs_product_indices(720, 883, 1.0, 488, (-1.0), 490, 700, (-1.0));s.store_mul_scale_offset_rhs(705, 478, 498, ((1.0 / (s.v[688])) * ((1.60219e-19 * (1000000.0 * s.v[174])))), (1.60219e-19 * (1000000.0 * s.v[174])));s.store_scalar(421, ((s.v[399] * (s.v[401] + (((s.v[689] / s.v[59]) / 3.0) / s.v[400]))) / ((s.v[400] * s.v[39]) * (s.v[37] - s.v[402]))));s.b[988] = (s.v[421] > 0.0);s.store_scalar(988, if s.b[988] { 1.0 } else { 0.0 });
        if s.b[988] {s.store_scalar(421, (1.0 / s.v[421]));}
        if (!s.b[988]) {s.store_scalar(421, 1000.0);}
        s.store_offset(424, 720, (s.v[36] * s.v[56]));s.store_scaled_sqrt_ad(721, A::div_scaled_product(s.ad_value(778), s.ad_value(831), 1.0, s.ad_value(478), (1.60219e-19 * 1000000.0)), 0.3333333333333333);s.store_add_scaled_inputs3_indices(819, 768, s.v[36], 766, (-1.0), 488, -1.0);s.store_scale(820, 819, 2.0);s.store_scale(821, 819, 2.5);
        if (s.v[36] == 1.0) {
            s.copy_ad(425, 820);
        } else {
            s.copy_ad(425, 821);
        }
        s.b[992] = (s.v[425] < 0.0);s.store_scalar(992, if s.b[992] { 1.0 } else { 0.0 });
        if s.b[992] {s.store_scalar(425, 0.0);}
        s.b[993] = (s.v[89] == 4.0);s.store_scalar(993, if s.b[993] { 1.0 } else { 0.0 });
        if s.b[993] {s.store_mul(861, 758, 702);s.store_div_scaled_inputs_indices(818, 500, s.v[688], 861, 1.0);}
        s.b[994] = (s.v[818] < 100.0);s.store_scalar(994, if s.b[994] { 1.0 } else { 0.0 });
        if (s.b[993] && s.b[994]) {s.store_exp(819, 818);s.store_offset(820, 819, (-1.0));s.store_square(821, 820);s.store_add_scaled_inputs(822, 821, 1.0, 819, (2.0 * 3.720075976e-44));s.store_div(875, 819, 822);}
        if (s.b[993] && (!s.b[994])) {s.store_scalar(875, (1.0 / (2.688117142e43 - 2.0)));}
        if s.b[993] {s.store_div(813, 778, 701);s.store_mul(814, 470, 813);s.store_div_scaled_inputs2_mixed_aii(883, A::add_scaled_product(s.ad_value(814), 1.0, s.ad_value(466), s.ad_value(875), 1.0), 1.0, 469, 1.0, 757, 1.0);}
        s.b[995] = (s.v[883] >= (-0.5));s.store_scalar(995, if s.b[995] { 1.0 } else { 0.0 });
        if (s.b[993] && s.b[995]) {s.store_offset(882, 883, 1.0);}
        if (s.b[993] && (!s.b[995])) {s.store_div_from_scalar_offset_scaled_input(818, 1.0, 883, 8.0, 3.0);s.store_mul_scale_offset_rhs(882, 818, 883, 3.0, 1.0);}
        if s.b[993] {s.store_mul(818, 882, 831);s.copy_ad(819, 521);s.store_div(820, 819, 818);}
        s.b[996] = (s.v[820] < (-100.0));s.store_scalar(996, if s.b[996] { 1.0 } else { 0.0 });
        if (s.b[993] && s.b[996]) {s.store_div_scaled_inputs_indices(821, 757, 3.720075976e-44, 728, 1.0);s.store_add_scaled_product_indices(822, 745, 1.0, 821, 882, 1.0);}
        s.b[997] = (s.v[820] > 100.0);s.store_scalar(997, if s.b[997] { 1.0 } else { 0.0 });
        if ((s.b[993] && (!s.b[996])) && s.b[997]) {s.store_div_scaled_inputs_indices(821, 757, 2.688117142e43, 728, 1.0);s.store_add_scaled_product_indices(822, 745, 1.0, 821, 882, 1.0);}
        if ((s.b[993] && (!s.b[996])) && (!s.b[997])) {s.store_div_scaled_product_mixed_aii(821, A::exp(s.ad_value(820)), 757, 1.0, 728, 1.0);s.store_add_scaled_product_indices(822, 745, 1.0, 821, 882, 1.0);}
        if s.b[993] {s.store_div_scaled_inputs_indices(426, 818, 0.6931471805599453, 822, 1.0);}
        if (!s.b[993]) {s.store_scalar(426, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_17(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[1050] = ((p.p35 >= 4.4) || (p.p61 != 0.0));s.store_scalar(1050, if s.b[1050] { 1.0 } else { 0.0 });s.b[1051] = (s.v[476] < 0.01);s.store_scalar(1051, if s.b[1051] { 1.0 } else { 0.0 });
        if (s.b[1050] && s.b[1051]) {s.store_scalar(476, 0.01);}
        s.b[1052] = (s.v[476] > 1.0);s.store_scalar(1052, if s.b[1052] { 1.0 } else { 0.0 });
        if ((s.b[1050] && (!s.b[1051])) && s.b[1052]) {s.store_scalar(476, 1.0);s.store_scalar(475, 0.0);}
        s.b[1053] = (s.v[551] < 0.0);s.store_scalar(1053, if s.b[1053] { 1.0 } else { 0.0 });
        if s.b[1053] {s.store_scalar(551, 0.0);s.store_scalar(552, 0.0);}
        s.b[1054] = ((s.v[552] < 0.001) && (s.v[552] != 0.0));s.store_scalar(1054, if s.b[1054] { 1.0 } else { 0.0 });
        if ((!s.b[1053]) && s.b[1054]) {s.store_scalar(552, 0.0);}
        s.store_scalar(770, 0.0);s.b[1144] = ((p.p33 == 1.0) && (p.p16 != 0.0));s.store_scalar(1144, if s.b[1144] { 1.0 } else { 0.0 });
        if s.b[1144] {s.store_voltage(770, ctx, nodes, Some(6), None);}
        if (!s.b[1144]) {s.store_scalar(770, 0.0);}
        s.store_offset(769, 770, s.v[769]);s.store_scale(771, 769, 1.0 / (s.v[150]));s.store_offset_scaled(772, 769, 1.0 / (s.v[150]), (-1.0));s.store_scalar(1466, 0.0);s.store_scalar(1467, 0.0);s.store_scalar(1468, 0.0);s.store_scalar(1469, 0.0);s.store_scalar(1464, 0.0);s.store_scalar(1454, 0.0);s.store_scalar(1191, 0.0);s.store_scalar(1455, 0.0);s.store_scalar(1463, 0.0);s.store_scalar(1460, 0.0);s.store_scalar(1461, 0.0);s.store_scalar(1459, 0.0);s.store_scalar(1451, 0.0);s.copy_ad(1290, 552);s.copy_ad(1429, 543);s.copy_ad(1430, 544);s.b[1492] = ((p.p33 == 1.0) && (p.p16 != 0.0));s.store_scalar(1492, if s.b[1492] { 1.0 } else { 0.0 });s.b[1493] = (s.v[68] == 0.0);s.store_scalar(1493, if s.b[1493] { 1.0 } else { 0.0 });
        if (s.b[1492] && s.b[1493]) {s.store_scale(1168, 769, 8.617087e-5);s.store_offset(1179, 769, 1108.0);s.store_square(1184, 769);s.store_sub_from_scalar_ad(1247, 1.16, A::div_scaled_inputs(s.ad_value(1184), 0.000702, s.ad_value(1179), 1.0));s.store_scalar(1181, 0.00019230584);s.store_sqrt(1184, 769);s.store_mul3_affine_lhs(1182, 769, 1184, 14500000000.0, 0.0, 1181);s.store_sub_from_scalar_ad(1185, 21.5565981, A::div_scaled_inputs(s.ad_value(1247), 1.0, s.ad_value(1168), 2.0));}
        s.b[1494] = (s.v[1185] > (-100.0));s.store_scalar(1494, if s.b[1494] { 1.0 } else { 0.0 });
        if ((s.b[1492] && s.b[1493]) && s.b[1494]) {s.store_exp(1183, 1185);}
        if ((s.b[1492] && s.b[1493]) && (!s.b[1494])) {s.store_scalar(1183, (((-100.0)) as f64).exp());}
        if (s.b[1492] && s.b[1493]) {s.store_mul(1246, 1182, 1183);}
        if (s.b[1492] && s.b[1493]) {
            if (((1e20 * s.v[478]) / (s.v[1246] * s.v[1246])) > 1e-38) {
                s.store_ln_div_scaled_input_square_denominator(1179, 478, 1e20, 1246, 1.0);
            } else {
                s.store_scalar(1179, -(87.49823353377374));
            }
        }
        if (s.b[1492] && s.b[1493]) {s.store_mul(1275, 1168, 1179);}
        if (s.b[1492] && (!s.b[1493])) {s.store_scalar(1435, s.v[150]);s.store_scale(1168, 769, 8.617087e-5);s.store_primal_scale(1437, 1435, 8.617087e-5);s.copy_ad(1436, 755);s.store_sub_from_scalar_ad(1247, s.v[76], A::div_scaled_product_offset_denominator(s.ad_value(769), s.ad_value(769), s.v[77], s.ad_value(769), s.v[78], 1.0));s.store_div_from_scalar_sqrt_ad(1181, 1.0, A::mul(A::square(s.ad_value(1435)), s.ad_value(1435)));s.store_sqrt(1184, 769);s.store_mul3_affine_lhs(1182, 769, 1184, s.v[75], 0.0, 1181);s.store_exp_ad(1183, A::sub(A::div_scaled_inputs(s.ad_value(1436), 1.0, s.ad_value(1437), 2.0), A::div_scaled_inputs(s.ad_value(1247), 1.0, s.ad_value(1168), 2.0)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1492] && (!s.b[1493])) {s.store_mul(1246, 1182, 1183);}
        if (s.b[1492] && (!s.b[1493])) {
            if (((1e20 * s.v[478]) / (s.v[1246] * s.v[1246])) > 1e-38) {
                s.store_ln_div_scaled_input_square_denominator(1179, 478, 1e20, 1246, 1.0);
            } else {
                s.store_scalar(1179, -(87.49823353377374));
            }
        }
        if (s.b[1492] && (!s.b[1493])) {s.store_mul(1275, 1168, 1179);}
        s.b[1495] = (s.v[479] > 0.0);s.store_scalar(1495, if s.b[1495] { 1.0 } else { 0.0 });
        if (s.b[1492] && s.b[1495]) {
            if ((s.v[478] / s.v[479]) > 1e-38) {
                s.store_ln_div(1179, 478, 479);
            } else {
                s.store_scalar(1179, -(87.49823353377374));
            }
        }
        if (s.b[1492] && s.b[1495]) {s.store_scaled_mul(1276, 1168, 1179, (-s.v[36]));}
        if (s.b[1492] && (!s.b[1495])) {
            if (((((-s.v[478]) * s.v[479]) / s.v[1246]) / s.v[1246]) > 1e-38) {
                s.store_ln_ad(1179, A::div_scaled_product_by_product(s.ad_value(478), s.ad_value(479), -1.0, s.ad_value(1246), s.ad_value(1246), 1.0));
            } else {
                s.store_scalar(1179, -(87.49823353377374));
            }
        }
        if (s.b[1492] && (!s.b[1495])) {s.store_scaled_mul(1276, 1168, 1179, (-s.v[36]));}
        if s.b[1492] {
            s.store_mul_scale_offset_mixed_ia(1277, 1168, {
                if ((s.v[478] / s.v[1246]) > 1e-38) {
                    A::ln(A::div(s.ad_value(478), s.ad_value(1246)))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 2.0, 0.0);
        }
        if s.b[1492] {s.store_sqrt(1278, 1277);s.store_mul_sqrt_mixed_ia(1279, 1278, A::div_scaled_inputs(s.ad_value(778), 2.0, s.ad_value(478), (1.60219e-19 * 1000000.0)));s.store_div_mixed_ai(1473, A::sqrt_scaled_input(A::mul_scaled_lhs(s.ad_value(778), 1.60219e-19, s.ad_value(478)), (1000000.0 * 1.0 / (2.0))), 1278);s.store_sqrt_ad(1180, A::mul3(A::div_scaled_inputs(s.ad_value(778), 1.0, s.ad_value(777), 8.85418e-12), s.ad_value(776), s.ad_value(1279)));s.store_ad_value(1179, A::exp_div_scaled_inputs(s.ad_value(506), ((-0.5) * s.v[688]), s.ad_value(1180), 1.0));s.store_add_scaled_product_indices(1474, 1179, 1.0, 1179, 1179, 2.0);s.store_ad_value(1179, A::exp_div_scaled_inputs(s.ad_value(505), ((-0.5) * s.v[688]), s.ad_value(1180), 1.0));s.store_add_scaled_product_indices(1181, 1179, 1.0, 1179, 1179, 2.0);s.store_add_scaled_product_indices(1475, 562, 1.0, 561, 1181, 1.0);s.copy_ad(409, 1168);s.store_offset(1182, 771, (-1.0));s.store_mul_div_from_scalar_lhs_ad_indices(1183, 1.115, 1168, 1182);s.store_div_scaled_product_indices(1186, 619, 1183, 1.0, 661, 1.0);}
        s.b[1496] = (s.v[1186] > 100.0);s.store_scalar(1496, if s.b[1496] { 1.0 } else { 0.0 });
        if (s.b[1492] && s.b[1496]) {s.store_scaled_offset(1179, 1186, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1497] = (s.v[1186] < (-100.0));s.store_scalar(1497, if s.b[1497] { 1.0 } else { 0.0 });
        if ((s.b[1492] && (!s.b[1496])) && s.b[1497]) {s.store_scalar(1179, 3.720075976e-44);}
        if ((s.b[1492] && (!s.b[1496])) && (!s.b[1497])) {s.store_exp(1179, 1186);}
        s.b[1498] = (s.v[619] == s.v[620]);s.store_scalar(1498, if s.b[1498] { 1.0 } else { 0.0 });
        if (s.b[1492] && s.b[1498]) {s.copy_ad(1180, 1179);}
        if (s.b[1492] && (!s.b[1498])) {s.store_div_scaled_product_indices(1186, 620, 1183, 1.0, 661, 1.0);}
        s.b[1499] = (s.v[1186] > 100.0);s.store_scalar(1499, if s.b[1499] { 1.0 } else { 0.0 });
        if ((s.b[1492] && (!s.b[1498])) && s.b[1499]) {s.store_scaled_offset(1180, 1186, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1500] = (s.v[1186] < (-100.0));s.store_scalar(1500, if s.b[1500] { 1.0 } else { 0.0 });
        if (((s.b[1492] && (!s.b[1498])) && (!s.b[1499])) && s.b[1500]) {s.store_scalar(1180, 3.720075976e-44);}
        if (((s.b[1492] && (!s.b[1498])) && (!s.b[1499])) && (!s.b[1500])) {s.store_exp(1180, 1186);}
        if s.b[1492] {s.store_div_scaled_product_indices(1186, 621, 1183, 1.0, 663, 1.0);}
        s.b[1501] = (s.v[1186] > 100.0);s.store_scalar(1501, if s.b[1501] { 1.0 } else { 0.0 });
        if (s.b[1492] && s.b[1501]) {s.store_scaled_offset(1181, 1186, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1502] = (s.v[1186] < (-100.0));s.store_scalar(1502, if s.b[1502] { 1.0 } else { 0.0 });
        if ((s.b[1492] && (!s.b[1501])) && s.b[1502]) {s.store_scalar(1181, 3.720075976e-44);}
        if ((s.b[1492] && (!s.b[1501])) && (!s.b[1502])) {s.store_exp(1181, 1186);}
        if s.b[1492] {s.store_mul(1307, 716, 1179);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1492] {s.store_mul(1284, 667, 1179);s.store_mul(1282, 669, 1180);s.store_mul(1286, 671, 1181);s.store_mul(1186, 622, 1182);}
        s.b[1503] = (s.v[1186] > 100.0);s.store_scalar(1503, if s.b[1503] { 1.0 } else { 0.0 });
        if (s.b[1492] && s.b[1503]) {s.store_scaled_offset(1179, 1186, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1504] = (s.v[1186] < (-100.0));s.store_scalar(1504, if s.b[1504] { 1.0 } else { 0.0 });
        if ((s.b[1492] && (!s.b[1503])) && s.b[1504]) {s.store_scalar(1179, 3.720075976e-44);}
        if ((s.b[1492] && (!s.b[1503])) && (!s.b[1504])) {s.store_exp(1179, 1186);}
        if s.b[1492] {s.store_mul(1288, 673, 1179);s.store_div_scaled_product_indices(1186, 619, 1183, 1.0, 662, 1.0);}
        s.b[1505] = (s.v[1186] > 100.0);s.store_scalar(1505, if s.b[1505] { 1.0 } else { 0.0 });
        if (s.b[1492] && s.b[1505]) {s.store_scaled_offset(1179, 1186, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1506] = (s.v[1186] < (-100.0));s.store_scalar(1506, if s.b[1506] { 1.0 } else { 0.0 });
        if ((s.b[1492] && (!s.b[1505])) && s.b[1506]) {s.store_scalar(1179, 3.720075976e-44);}
        if ((s.b[1492] && (!s.b[1505])) && (!s.b[1506])) {s.store_exp(1179, 1186);}
        s.b[1507] = (s.v[619] == s.v[623]);s.store_scalar(1507, if s.b[1507] { 1.0 } else { 0.0 });
        if (s.b[1492] && s.b[1507]) {s.copy_ad(1180, 1179);}
        if (s.b[1492] && (!s.b[1507])) {s.store_div_scaled_product_indices(1186, 623, 1183, 1.0, 662, 1.0);}
        s.b[1508] = (s.v[1186] > 100.0);s.store_scalar(1508, if s.b[1508] { 1.0 } else { 0.0 });
        if ((s.b[1492] && (!s.b[1507])) && s.b[1508]) {s.store_scaled_offset(1180, 1186, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1509] = (s.v[1186] < (-100.0));s.store_scalar(1509, if s.b[1509] { 1.0 } else { 0.0 });
        if (((s.b[1492] && (!s.b[1507])) && (!s.b[1508])) && s.b[1509]) {s.store_scalar(1180, 3.720075976e-44);}
        if (((s.b[1492] && (!s.b[1507])) && (!s.b[1508])) && (!s.b[1509])) {s.store_exp(1180, 1186);}
        if s.b[1492] {s.store_div_scaled_product_indices(1186, 624, 1183, 1.0, 664, 1.0);}
        s.b[1510] = (s.v[1186] > 100.0);s.store_scalar(1510, if s.b[1510] { 1.0 } else { 0.0 });
        if (s.b[1492] && s.b[1510]) {s.store_scaled_offset(1181, 1186, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1511] = (s.v[1186] < (-100.0));s.store_scalar(1511, if s.b[1511] { 1.0 } else { 0.0 });
        if ((s.b[1492] && (!s.b[1510])) && s.b[1511]) {s.store_scalar(1181, 3.720075976e-44);}
        if ((s.b[1492] && (!s.b[1510])) && (!s.b[1511])) {s.store_exp(1181, 1186);}
        if s.b[1492] {s.store_mul(1308, 717, 1179);s.store_mul(1285, 668, 1179);s.store_mul(1283, 670, 1180);s.store_mul(1287, 672, 1181);s.store_mul(1186, 625, 1182);}
        s.b[1512] = (s.v[1186] > 100.0);s.store_scalar(1512, if s.b[1512] { 1.0 } else { 0.0 });
        if (s.b[1492] && s.b[1512]) {s.store_scaled_offset(1179, 1186, ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1513] = (s.v[1186] < (-100.0));s.store_scalar(1513, if s.b[1513] { 1.0 } else { 0.0 });
        if ((s.b[1492] && (!s.b[1512])) && s.b[1513]) {s.store_scalar(1179, 3.720075976e-44);}
        if ((s.b[1492] && (!s.b[1512])) && (!s.b[1513])) {s.store_exp(1179, 1186);}
        if s.b[1492] {s.store_mul(1289, 674, 1179);s.store_mul_pow_indices(1280, 514, 771, 515);}
        s.b[1514] = (p.p35 < 4.2);s.store_scalar(1514, if s.b[1514] { 1.0 } else { 0.0 });
        if (s.b[1492] && s.b[1514]) {s.store_offset_mul_ad(1296, s.ad_value(597), A::scale_offset(s.ad_value(771), s.v[252], 1.0), 1e-9);}
        if (s.b[1492] && (!s.b[1514])) {s.store_offset_mul_ad(1296, s.ad_value(597), A::scale_offset(s.ad_value(1182), s.v[252], 1.0), 1e-9);}
        if s.b[1492] {s.store_scale(1186, 601, s.v[249]);s.store_div(1295, 1186, 1296);s.store_scale(1183, 414, s.v[249]);s.store_div(1294, 1183, 1296);s.store_offset(1181, 1294, 1.0);s.store_offset(1186, 1295, 1.0);s.store_div(1179, 1181, 1186);s.store_mul(1280, 1280, 1179);s.store_add_scaled_product_indices(1281, 471, 1.0, 472, 1182, (-1.0));s.store_offset_mul(1181, 250, 1294, 1.0);s.store_offset_mul(1186, 250, 1295, 1.0);s.store_div(1179, 1181, 1186);s.store_mul(1281, 1281, 1179);}
        s.b[1515] = (s.v[403] != 1.0);s.store_scalar(1515, if s.b[1515] { 1.0 } else { 0.0 });
        if (s.b[1492] && s.b[1515]) {s.store_div_scaled_add_product_indices(1290, 551, 1.0, 555, 1182, 1.0, 529, 1.0);s.store_scalar(1429, 0.0);s.store_scalar(1430, 0.0);}
        if (s.b[1492] && (!s.b[1515])) {s.store_scalar(1290, 0.0);s.store_scale(1428, 529, s.v[39]);s.store_mul(1189, 555, 1182);s.store_add(1180, 539, 1189);s.store_offset(1181, 1189, s.v[160]);s.store_div(1429, 1180, 1428);s.store_add(1186, 540, 1189);s.store_offset(1183, 1189, s.v[159]);s.store_div(1430, 1186, 1428);}
        if s.b[1492] {s.store_add_scaled_product_indices(1291, 523, 1.0, 509, 1182, 1.0);s.store_add_scaled_product_indices(1292, 524, 1.0, 511, 1182, 1.0);s.store_add_scaled_product_indices(1293, 525, 1.0, 513, 1182, 1.0);}
        if (!s.b[1492]) {s.copy_ad(1275, 485);s.copy_ad(1276, 530);s.copy_ad(1277, 488);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (!s.b[1492]) {s.copy_ad(1278, 700);s.copy_ad(1279, 701);s.copy_ad(1247, 756);s.copy_ad(1473, 728);s.copy_ad(1474, 703);s.copy_ad(1475, 704);s.copy_ad(1284, 531);s.copy_ad(1285, 532);s.copy_ad(1282, 533);s.copy_ad(1283, 534);s.copy_ad(1286, 535);s.copy_ad(1287, 536);s.copy_ad(1288, 537);s.copy_ad(1289, 538);s.copy_ad(1307, 718);s.copy_ad(1308, 719);s.copy_ad(1280, 765);s.copy_ad(1281, 767);s.copy_ad(1291, 508);s.copy_ad(1292, 510);s.copy_ad(1293, 512);}
        s.b[1516] = (param_given[89] || param_given[93]);s.store_scalar(1516, if s.b[1516] { 1.0 } else { 0.0 });s.b[1517] = (!param_given[89]);s.store_scalar(1517, if s.b[1517] { 1.0 } else { 0.0 });
        if (s.b[1516] && s.b[1517]) {s.store_scalar(490, 0.53);}
        s.b[1518] = (!param_given[93]);s.store_scalar(1518, if s.b[1518] { 1.0 } else { 0.0 });
        if (s.b[1516] && s.b[1518]) {s.store_scalar(494, (-0.0186));}
        s.b[1524] = (!param_given[86]);s.store_scalar(1524, if s.b[1524] { 1.0 } else { 0.0 });
        if (((!s.b[1516]) && s.b[1524]) && (s.v[68] != 0.0)) {s.store_scaled_div_from_scalar_ad(1179, 1.60219e-19, A::scale(s.ad_value(778), 2.0), 1000000.0);}
        if (((!s.b[1516]) && s.b[1524]) && (s.v[68] == 0.0)) {s.store_scalar(1179, 0.00077348);}
        if ((!s.b[1516]) && s.b[1524]) {s.store_add_scaled_product_indices(484, 1277, 1.0, 1179, 478, (-(s.v[487] * s.v[487])));}
        s.b[1525] = (s.v[484] > 0.0);s.store_scalar(1525, if s.b[1525] { 1.0 } else { 0.0 });
        if ((!s.b[1516]) && s.b[1525]) {s.store_neg(484, 484);}
        s.b[1526] = (s.v[486] > 0.0);s.store_scalar(1526, if s.b[1526] { 1.0 } else { 0.0 });
        if ((!s.b[1516]) && s.b[1526]) {s.store_primal_neg(486, 486);}
        s.b[1527] = (!param_given[84]);s.store_scalar(1527, if s.b[1527] { 1.0 } else { 0.0 });
        if ((!s.b[1516]) && s.b[1527]) {s.store_div_scaled_product_mixed_iai(482, 780, A::sqrt(s.ad_value(478)), 1.0, 757, 1.0);}
        s.b[1528] = (!param_given[85]);s.store_scalar(1528, if s.b[1528] { 1.0 } else { 0.0 });
        if ((!s.b[1516]) && s.b[1528]) {s.store_div_scaled_product_mixed_iai(483, 780, A::sqrt(s.ad_value(479)), 1.0, 757, 1.0);}
        if (!s.b[1516]) {s.store_sub(1179, 482, 483);s.store_sub_mixed_ai(1180, A::sqrt(A::sub(s.ad_value(1277), s.ad_value(484))), 1278);s.store_mul_sub_mixed_iai(1181, 1278, A::sqrt(A::sub(s.ad_value(1277), s.ad_value(486))), 1278);s.store_div_scaled_product_add_scaled_denominator_indices(1182, 1179, 1180, 1.0, 1181, 2.0, 486, 1.0, 1.0);s.store_add_scaled_inputs3_indices(763, 763, 1.0, 494, (-1.0), 1182, 1.0);s.store_add_scaled_product_mixed_iia(490, 483, 1.0, 763, A::sqrt(A::sub(s.ad_value(1277), s.ad_value(486))), (-2.0));}
        s.store_offset(1179, 628, s.v[689]);s.b[1529] = (s.v[1179] < 1e-8);s.store_scalar(1529, if s.b[1529] { 1.0 } else { 0.0 });
        if s.b[1529] {s.store_scalar(1179, 1e-8);}
        s.store_mul_scale_offset_mixed_ia(707, 490, A::div(s.ad_value(627), s.ad_value(1179)), 1.0, 1.0);s.b[1530] = (!param_given[108]);s.store_scalar(1530, if s.b[1530] { 1.0 } else { 0.0 });s.b[1531] = (param_given[107] || param_given[106]);s.store_scalar(1531, if s.b[1531] { 1.0 } else { 0.0 });
        if (s.b[1530] && s.b[1531]) {s.store_add_scaled_product_mixed_aii(766, A::add_scaled_inputs4(s.ad_value(766), 1.0, s.ad_value(522), (-1.0), s.ad_value(768), s.v[36], s.ad_value(1277), -1.0), 1.0, 707, 1278, (-1.0));}
        if (s.b[1530] && (!s.b[1531])) {
        }
        s.b[1532] = (!param_given[107]);s.store_scalar(1532, if s.b[1532] { 1.0 } else { 0.0 });
        if s.b[1532] {s.store_add_scaled_inputs_product_indices(768, 766, s.v[36], 1277, s.v[36], 707, 1278, s.v[36]);}
        s.b[1533] = (p.p35 < 4.2);s.store_scalar(1533, if s.b[1533] { 1.0 } else { 0.0 });
        if s.b[1533] {s.copy_ad(1429, 543);s.copy_ad(1473, 728);s.copy_ad(1474, 703);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_21(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1533] {s.copy_ad(1475, 704);}
        s.b[1534] = (s.v[89] == 4.0);s.store_scalar(1534, if s.b[1534] { 1.0 } else { 0.0 });
        if (s.b[1533] && s.b[1534]) {s.copy_ad(1291, 508);s.copy_ad(1293, 512);}
        s.store_scaled_voltage(1155, ctx, nodes, Some(7), Some(8), s.v[36]);s.store_scaled_voltage(1154, ctx, nodes, Some(5), Some(8), s.v[36]);s.store_scaled_voltage(1157, ctx, nodes, Some(9), Some(8), s.v[36]);s.store_scaled_voltage(1232, ctx, nodes, Some(3), Some(8), s.v[36]);s.store_scaled_voltage(1447, ctx, nodes, Some(9), Some(4), s.v[36]);s.store_scaled_voltage(1421, ctx, nodes, Some(11), Some(8), s.v[36]);s.store_scaled_voltage(1422, ctx, nodes, Some(12), Some(7), s.v[36]);s.store_scaled_voltage(1353, ctx, nodes, Some(10), Some(8), s.v[36]);s.store_sub(1153, 1154, 1155);s.store_sub(1156, 1157, 1155);s.store_sub(1233, 1232, 1155);s.store_sub(1354, 1353, 1155);s.b[1535] = (s.v[1155] >= 0.0);s.store_scalar(1535, if s.b[1535] { 1.0 } else { 0.0 });
        if s.b[1535] {s.store_scalar(759, 1.0);s.copy_ad(1158, 1155);s.copy_ad(1159, 1157);s.copy_ad(1160, 1154);s.copy_ad(1235, 1153);s.copy_ad(1236, 1232);s.copy_ad(1443, 1156);s.copy_ad(1476, 645);s.copy_ad(1477, 646);s.copy_ad(1478, 647);s.copy_ad(1479, 648);s.copy_ad(1480, 649);s.copy_ad(1481, 650);s.copy_ad(1482, 651);s.copy_ad(1483, 652);s.copy_ad(1484, 653);s.copy_ad(1485, 654);s.copy_ad(1486, 655);s.copy_ad(1487, 656);s.copy_ad(1488, 657);s.copy_ad(1489, 658);}
        if (!s.b[1535]) {s.store_scalar(759, (-1.0));s.store_neg(1158, 1155);s.copy_ad(1159, 1156);s.copy_ad(1160, 1153);s.copy_ad(1235, 1154);s.copy_ad(1236, 1233);s.copy_ad(1443, 1157);s.copy_ad(1476, 652);s.copy_ad(1477, 653);s.copy_ad(1478, 654);s.copy_ad(1479, 655);s.copy_ad(1480, 656);s.copy_ad(1481, 657);s.copy_ad(1482, 658);s.copy_ad(1483, 645);s.copy_ad(1484, 646);s.copy_ad(1485, 647);s.copy_ad(1486, 648);s.copy_ad(1487, 649);s.copy_ad(1488, 650);s.copy_ad(1489, 651);}
        s.store_sub(1237, 1236, 1276);s.store_scalar(1248, s.v[753]);s.store_add(1179, 766, 1277);s.b[1536] = (s.v[68] == 0.0);s.store_scalar(1536, if s.b[1536] { 1.0 } else { 0.0 });
        if s.b[1536] {s.copy_ad(779, 778);}
        if (!s.b[1536]) {s.store_scalar(779, (s.v[87] * 8.85418e-12));}
        s.b[1537] = ((((s.v[480] > 1e18) && (s.v[480] < 1e25)) && (s.v[1159] > s.v[1179])) && (s.v[779] != 0.0));s.store_scalar(1537, if s.b[1537] { 1.0 } else { 0.0 });
        if s.b[1537] {s.store_div_scaled_product_mixed_iia(1180, 779, 480, (1000000.0 * 1.60219e-19), A::square(s.ad_value(757)), 1.0);s.store_sqrt_offset_ad(1183, A::div_scaled_inputs2(s.ad_value(1159), 2.0, s.ad_value(1179), (-2.0), s.ad_value(1180), 1.0), 1.0);s.store_mul_scale_offset_indices(1181, 1180, 1183, 1.0, (-1.0));s.store_div_scaled_product_indices(1182, 1181, 1181, 0.5, 1180, 1.0);s.store_offset_sub(1186, 782, 1182, (-0.05));s.store_sqrt_square_offset(1185, 1186, 0.224);s.store_add_scaled_inputs3_indices(1184, 782, 1.0, 1186, (-0.5), 1185, (-0.5));s.store_sub(1161, 1159, 1184);}
        if (!s.b[1537]) {s.copy_ad(1161, 1159);}
        s.b[1538] = ((((s.v[480] > 1e18) && (s.v[480] < 1e25)) && (s.v[1443] > s.v[1179])) && (s.v[779] != 0.0));s.store_scalar(1538, if s.b[1538] { 1.0 } else { 0.0 });
        if s.b[1538] {s.store_div_scaled_product_mixed_iia(1180, 779, 480, (1000000.0 * 1.60219e-19), A::square(s.ad_value(757)), 1.0);s.store_sqrt_offset_ad(1183, A::div_scaled_inputs2(s.ad_value(1443), 2.0, s.ad_value(1179), (-2.0), s.ad_value(1180), 1.0), 1.0);s.store_mul_scale_offset_indices(1181, 1180, 1183, 1.0, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_22(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1538] {s.store_div_scaled_product_indices(1182, 1181, 1181, 0.5, 1180, 1.0);s.store_offset_sub(1186, 782, 1182, (-0.05));s.store_sqrt_square_offset(1185, 1186, 0.224);s.store_add_scaled_inputs3_indices(1184, 782, 1.0, 1186, (-0.5), 1185, (-0.5));s.store_sub(1444, 1443, 1184);}
        if (!s.b[1538]) {s.copy_ad(1444, 1443);}
        s.copy_ad(1458, 1159);s.store_scalar(1227, s.v[688]);s.b[1539] = ((p.p33 == 1.0) && (p.p16 != 0.0));s.store_scalar(1539, if s.b[1539] { 1.0 } else { 0.0 });
        if s.b[1539] {s.store_scale(1168, 769, 8.617087e-5);}
        if (!s.b[1539]) {s.copy_ad(1168, 409);}
        s.store_sub(1170, 1275, 1277);s.b[1540] = (s.v[57] == 0.0);s.store_scalar(1540, if s.b[1540] { 1.0 } else { 0.0 });
        if s.b[1540] {s.copy_ad(1367, 1160);s.copy_ad(1382, 1160);}
        s.b[1541] = (s.v[404] == 0.0);s.store_scalar(1541, if s.b[1541] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1541]) {s.store_div_scaled_inputs_indices(1179, 591, (-s.v[688]), 489, 1.0);s.store_mul_add_scaled_inputs_rhs(1180, 590, A::exp_scaled_input(s.ad_value(1179), 0.5), 1.0, A::exp(s.ad_value(1179)), 2.0);s.store_mul_sub_rhs(1181, 1180, 1275, 1277);s.store_div_scaled_inputs_indices(1182, 705, 0.5, 754, 1.0);s.store_add_scaled_inputs4_indices(1370, 1277, 1.0, 1182, (-1.0), 582, 1.0, 1181, 1.0);s.store_offset_scaled(1179, 754, 1.0 / (s.v[1248]), 1.0);s.store_div_scaled_inputs_indices(1182, 589, (-s.v[688]), 489, 1.0);s.store_mul_add_scaled_inputs_rhs(1184, 588, A::exp_scaled_input(s.ad_value(1182), 0.5), 1.0, A::exp(s.ad_value(1182)), 2.0);s.store_div_scaled_inputs2_indices(1180, 587, 1.0, 1184, (-1.0), 1179, 1.0);s.store_mul(1181, 1180, 1237);s.store_div_from_scalar_offset_ad(1183, 1.0, A::div_from_scalar(s.v[1248], s.ad_value(754)), 1.0);s.store_add_scaled_product_indices(1365, 1181, 1.0, 1183, 1370, 1.0);}
        if ((!s.b[1540]) && (!s.b[1541])) {s.store_div_from_scalar_add_ad(1179, 1.0, A::offset(s.ad_value(754), s.v[1248]), s.ad_value(584));s.store_div_scaled_inputs_indices(1180, 591, (-s.v[688]), 489, 1.0);s.store_mul_add_scaled_inputs_rhs(1181, 590, A::exp_scaled_input(s.ad_value(1180), 0.5), 1.0, A::exp(s.ad_value(1180)), 2.0);s.store_mul_add_rhs(1182, 1181, 1158, 583);s.store_div_scaled_inputs_indices(1183, 705, 0.5, 754, 1.0);s.store_mul_ad_product_rhs_mixed_ia(1184, 754, 1179, A::add_scaled_inputs3(s.ad_value(1277), 1.0, s.ad_value(1183), (-1.0), s.ad_value(582), 1.0));s.store_mul3_lhs(1185, 584, 1179, 1182);s.store_add(1370, 1184, 1185);s.store_scaled_mul(1186, 1179, 1237, s.v[1248]);s.store_add(1365, 1370, 1186);}
        if (!s.b[1540]) {s.store_offset_sub(1180, 1370, 1365, (-0.005));s.store_sqrt_square_offset(1181, 1180, 2.5e-5);s.store_scaled_add(1182, 1180, 1181, 0.5);s.store_div_scaled_product_indices(1183, 1182, 754, 1.0, 705, 1.0);s.store_add_scaled_product_indices(1366, 1365, 1.0, 1182, 1183, (-0.5));s.store_offset(1180, 1277, (-0.02));s.store_offset_sub(1181, 1180, 1366, (-0.005));s.store_sqrt_square_offset(1182, 1181, (4.0 * 0.005));s.store_add_scaled_inputs3_indices(1366, 1180, 1.0, 1181, (-0.5), 1182, (-0.5));s.store_sub(1163, 1277, 1366);s.store_sqrt(1164, 1163);s.store_div_scaled_product_indices(1199, 1279, 1164, 1.0, 1278, 1.0);s.store_sqrt(1182, 1199);s.store_mul(1179, 501, 1366);}
        s.b[1542] = (s.v[1179] >= (-0.5));s.store_scalar(1542, if s.b[1542] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1542]) {s.store_offset(1180, 1179, 1.0);}
        if ((!s.b[1540]) && (!s.b[1542])) {s.store_div_from_scalar_offset_scaled_input(1183, 1.0, 1179, 8.0, 3.0);s.store_mul_scale_offset_rhs(1180, 1183, 1179, 3.0, 1.0);}
        if (!s.b[1540]) {s.store_mul3_lhs(1200, 758, 1182, 1180);s.store_mul(1179, 504, 1366);}
        s.b[1543] = (s.v[1179] >= (-0.5));s.store_scalar(1543, if s.b[1543] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1543]) {s.store_offset(1180, 1179, 1.0);}
        if ((!s.b[1540]) && (!s.b[1543])) {s.store_div_from_scalar_offset_scaled_input(1183, 1.0, 1179, 8.0, 3.0);s.store_mul_scale_offset_rhs(1180, 1183, 1179, 3.0, 1.0);}
        if (!s.b[1540]) {s.store_mul3_lhs(1201, 758, 1182, 1180);s.store_div_scaled_inputs_indices(1179, 500, ((-0.5) * s.v[1227]), 1200, 1.0);}
        s.b[1544] = (s.v[1179] > (-100.0));s.store_scalar(1544, if s.b[1544] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1544]) {s.store_exp(1180, 1179);s.store_mul_scale_offset_rhs(1203, 1180, 1180, 2.0, 1.0);}
        if ((!s.b[1540]) && (!s.b[1544])) {s.store_scalar(1180, 3.720075976e-44);s.store_mul_scale_offset_rhs(1203, 1180, 1180, 2.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_23(
        s: &mut ReactiveScratch,
    ) {
        if (!s.b[1540]) {s.store_div_scaled_product_indices(1181, 470, 778, 1.0, 1199, 1.0);s.store_add_scaled_value_products_indices(1182, 466, 1.0, 467, 1366, 1.0, 468, 1158, 1.0);s.store_div_scaled_inputs2_mixed_aii(1183, A::add_scaled_product(s.ad_value(1181), 1.0, s.ad_value(1182), s.ad_value(1203), 1.0), 1.0, 469, 1.0, 757, 1.0);}
        s.b[1545] = (s.v[1183] >= (-0.5));s.store_scalar(1545, if s.b[1545] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1545]) {s.store_offset(1167, 1183, 1.0);}
        if ((!s.b[1540]) && (!s.b[1545])) {s.store_div_from_scalar_offset_scaled_input(1179, 1.0, 1183, 8.0, 3.0);s.store_mul_scale_offset_rhs(1167, 1179, 1183, 3.0, 1.0);}
        s.b[1546] = (s.v[739] > 0.0);s.store_scalar(1546, if s.b[1546] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1546]) {s.store_mul_scale_offset_indices(1179, 1158, 740, -1.0, 0.0);}
        s.b[1547] = (s.v[1179] < (-100.0));s.store_scalar(1547, if s.b[1547] { 1.0 } else { 0.0 });
        if (((!s.b[1540]) && s.b[1546]) && s.b[1547]) {s.store_scalar(1181, 3.720075976e-44);}
        if (((!s.b[1540]) && s.b[1546]) && (!s.b[1547])) {s.store_exp(1181, 1179);}
        if ((!s.b[1540]) && s.b[1546]) {s.store_offset_mul_offset_rhs(1182, 739, 1181, 1.0, s.v[1227]);}
        if ((!s.b[1540]) && s.b[1546]) {
            s.store_mul_mixed_ia(1183, 1168, {
                            if ((s.v[1227] / s.v[1182]) > 1e-38) {
                                A::ln(A::div_from_scalar(s.v[1227], s.ad_value(1182)))
                            } else {
                                A::neg(A::constant(87.49823353377374))
                            }
                        });
        }
        if ((!s.b[1540]) && s.b[1546]) {s.store_mul(1424, 1167, 1183);}
        if ((!s.b[1540]) && (!s.b[1546])) {s.store_scalar(1424, 0.0);}
        if (!s.b[1540]) {s.store_mul(411, 499, 1203);s.store_mul(1202, 411, 1170);s.store_div_scaled_inputs_indices(1179, 503, ((-0.5) * (s.v[689] * s.v[1227])), 1201, 1.0);}
        s.b[1548] = (s.v[1179] > (-100.0));s.store_scalar(1548, if s.b[1548] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1548]) {s.store_exp(1180, 1179);s.store_mul_scale_offset_rhs(1181, 1180, 1180, 2.0, 1.0);}
        if ((!s.b[1540]) && (!s.b[1548])) {s.store_scalar(1180, 3.720075976e-44);s.store_mul_scale_offset_rhs(1181, 1180, 1180, 2.0, 1.0);}
        if (!s.b[1540]) {s.store_mul(1179, 502, 1181);s.store_mul(1239, 1179, 1170);s.store_sqrt_offset_scaled_input(1179, 498, 1.0 / (s.v[1227]), 1.0);s.store_add_scaled_inputs_product_indices(1180, 491, 1.0, 492, 1.0 / (s.v[1227]), 493, 1366, 1.0);s.store_add_scaled_product_mixed_aii(1238, A::mul3(s.ad_value(737), A::offset(s.ad_value(1179), (-1.0)), s.ad_value(1278)), 1.0, 1180, 772, 1.0);s.store_div_scaled_product_offset_denominator_indices(1205, 776, 1277, 1.0, 497, s.v[689], 1.0);s.store_add_scaled_product_indices(1182, 761, 1.0, 557, 1366, 1.0);}
        s.b[1549] = (s.v[1182] < 0.0001);s.store_scalar(1549, if s.b[1549] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1549]) {s.store_div_from_scalar_sub_from_scalar_ad(1188, 1.0, 3.0, A::scale(s.ad_value(1182), 20000.0));s.store_mul_scale_offset_indices(1182, 1188, 1182, -1.0, 0.0002);}
        if (!s.b[1540]) {s.store_mul3_lhs(1208, 1182, 1474, 1158);s.store_add_scaled_product_indices(1182, 762, 1.0, 559, 1366, 1.0);}
        s.b[1550] = (s.v[1182] < 0.0001);s.store_scalar(1550, if s.b[1550] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1550]) {s.store_div_from_scalar_sub_from_scalar_ad(1188, 1.0, 3.0, A::scale(s.ad_value(1182), 20000.0));s.store_mul_scale_offset_indices(1182, 1188, 1182, -1.0, 0.0002);}
        if (!s.b[1540]) {s.store_mul3_lhs(1404, 1182, 1474, 1158);s.store_sqrt_offset_scaled_input(1423, 738, 1.0 / (s.v[1227]), 1.0);s.store_exp_mul_scaled_lhs_indices(1179, 743, 2.0, 1158);s.store_div_scaled_product_offset_denominator_mixed_iai(1425, 752, A::offset(s.ad_value(1179), (-1.0)), 1.0, 1179, 1.0, 1.0);s.store_sub_add_scaled_inputs4_lhs_mixed_aiii(1371, A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_value_products(s.ad_value(768), s.v[36], A::add_scaled_products(s.ad_value(737), s.ad_value(1164), 1.0, s.ad_value(707), s.ad_value(1278), (-1.0)), s.ad_value(1423), 1.0, s.ad_value(764), s.ad_value(1366), (-1.0)), 1.0, s.ad_value(1202), (-1.0), s.ad_value(1239), -1.0), 1.0, A::add_scaled_product(s.ad_value(495), 1.0, s.ad_value(496), s.ad_value(1366), 1.0), s.ad_value(1205), 1.0), 1.0, 1238, 1.0, 1208, -1.0, 1424, -1.0, 1425);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_24(
        s: &mut ReactiveScratch,
    ) {
        if (!s.b[1540]) {s.store_sub_add_scaled_inputs4_lhs_mixed_aiii(1386, A::add_scaled_product(A::add_scaled_inputs3(A::add_scaled_value_products(s.ad_value(768), s.v[36], A::add_scaled_products(s.ad_value(737), s.ad_value(1164), 1.0, s.ad_value(707), s.ad_value(1278), (-1.0)), s.ad_value(1423), 1.0, s.ad_value(764), s.ad_value(1366), (-1.0)), 1.0, s.ad_value(1202), (-1.0), s.ad_value(1239), -1.0), 1.0, A::add_scaled_product(s.ad_value(495), 1.0, s.ad_value(496), s.ad_value(1366), 1.0), s.ad_value(1205), 1.0), 1.0, 1238, 1.0, 1404, -1.0, 1424, -1.0, 1425);s.store_sub(1372, 1371, 1161);s.store_mul(1189, 585, 1168);}
        s.b[1551] = (((s.v[1372] - s.v[586]) / s.v[1189]) > 100.0);s.store_scalar(1551, if s.b[1551] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1551]) {s.store_scaled_offset_ad(1373, A::div_scaled_inputs2(s.ad_value(1372), 1.0, s.ad_value(586), (-1.0), s.ad_value(1189), 1.0), ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1552] = (((s.v[1372] - s.v[586]) / s.v[1189]) < (-100.0));s.store_scalar(1552, if s.b[1552] { 1.0 } else { 0.0 });
        if (((!s.b[1540]) && (!s.b[1551])) && s.b[1552]) {s.store_scalar(1373, 3.720075976e-44);}
        if (((!s.b[1540]) && (!s.b[1551])) && (!s.b[1552])) {s.store_exp_ad(1373, A::div_scaled_inputs2(s.ad_value(1372), 1.0, s.ad_value(586), (-1.0), s.ad_value(1189), 1.0));}
        if (!s.b[1540]) {s.store_mul_ln_mixed_ia(1376, 1189, A::offset(s.ad_value(1373), 1.0));s.store_sub(1374, 1161, 1371);}
        s.b[1553] = (((s.v[1374] - s.v[586]) / s.v[1189]) > 100.0);s.store_scalar(1553, if s.b[1553] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1553]) {s.store_scaled_offset_ad(1375, A::div_scaled_inputs2(s.ad_value(1374), 1.0, s.ad_value(586), (-1.0), s.ad_value(1189), 1.0), ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1554] = (((s.v[1374] - s.v[586]) / s.v[1189]) < (-100.0));s.store_scalar(1554, if s.b[1554] { 1.0 } else { 0.0 });
        if (((!s.b[1540]) && (!s.b[1553])) && s.b[1554]) {s.store_scalar(1375, 3.720075976e-44);}
        if (((!s.b[1540]) && (!s.b[1553])) && (!s.b[1554])) {s.store_exp_ad(1375, A::div_scaled_inputs2(s.ad_value(1374), 1.0, s.ad_value(586), (-1.0), s.ad_value(1189), 1.0));}
        if (!s.b[1540]) {s.store_mul_ln_mixed_ia(1377, 1189, A::offset(s.ad_value(1375), 1.0));s.store_mul_product3_indices(1180, 1168, 592, 737, 1168, 1.0);s.store_add_scaled_product_mixed_iia(1181, 1377, 1.0, 707, A::sqrt(s.ad_value(1277)), 2.0);s.store_offset_div_scaled_product_indices(1179, 1377, 1181, 1.0, 1180, 1.0, 1.0);}
        if (!s.b[1540]) {
            s.store_add_scaled_product_mixed_iia(1368, 1277, 1.0, 1168, {
                if (s.v[1179] > 1e-38) {
                    A::ln(s.ad_value(1179))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 1.0);
        }
        if (!s.b[1540]) {s.store_div_add_scaled_inputs_rhs_mixed_ia(1179, 757, 757, 1.0, A::div_scalar_offset_denominator(1.0, A::div_from_scalar(1.0, s.ad_value(754)), (1.0 / s.v[1248]), 1.0), 1.0);s.store_add_scaled_product_indices(1369, 1368, 1.0, 1179, 1376, (-1.0));}
        s.b[1555] = (s.v[404] == 0.0);s.store_scalar(1555, if s.b[1555] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1555]) {s.store_div_scaled_inputs_indices(1179, 591, (-s.v[688]), 489, 1.0);s.store_mul_add_scaled_inputs_rhs(1180, 590, A::exp_scaled_input(s.ad_value(1179), 0.5), 1.0, A::exp(s.ad_value(1179)), 2.0);s.store_mul_sub_rhs(1181, 1180, 1275, 1277);s.store_div_scaled_inputs_indices(1182, 705, 0.5, 754, 1.0);s.store_add_scaled_inputs4_indices(1370, 1369, 1.0, 1182, (-1.0), 582, 1.0, 1181, 1.0);s.store_offset_scaled(1179, 754, 1.0 / (s.v[1248]), 1.0);s.store_div_scaled_inputs_indices(1182, 589, (-s.v[688]), 489, 1.0);s.store_mul_add_scaled_inputs_rhs(1184, 588, A::exp_scaled_input(s.ad_value(1182), 0.5), 1.0, A::exp(s.ad_value(1182)), 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_25(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[1540]) && s.b[1555]) {s.store_div_scaled_inputs2_indices(1180, 587, 1.0, 1184, (-1.0), 1179, 1.0);s.store_mul(1181, 1180, 1237);s.store_div_from_scalar_offset_ad(1179, 1.0, A::div_from_scalar(s.v[1248], s.ad_value(754)), 1.0);s.store_add_scaled_product_indices(1365, 1181, 1.0, 1179, 1370, 1.0);}
        if ((!s.b[1540]) && (!s.b[1555])) {s.store_div_from_scalar_add_ad(1179, 1.0, A::offset(s.ad_value(754), s.v[1248]), s.ad_value(584));s.store_div_scaled_inputs_indices(1180, 591, (-s.v[688]), 489, 1.0);s.store_mul_add_scaled_inputs_rhs(1181, 590, A::exp_scaled_input(s.ad_value(1180), 0.5), 1.0, A::exp(s.ad_value(1180)), 2.0);s.store_mul_add_rhs(1182, 1181, 1158, 583);s.store_div_scaled_inputs_indices(1183, 705, 0.5, 754, 1.0);s.store_mul_ad_product_rhs_mixed_ia(1184, 754, 1179, A::add_scaled_inputs3(s.ad_value(1369), 1.0, s.ad_value(1183), (-1.0), s.ad_value(582), 1.0));s.store_mul3_lhs(1185, 584, 1179, 1182);s.store_add(1370, 1184, 1185);s.store_scaled_mul(1186, 1179, 1237, s.v[1248]);s.store_add(1365, 1370, 1186);}
        s.b[1556] = (s.v[57] == 2.0);s.store_scalar(1556, if s.b[1556] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1556]) {s.store_offset(1364, 1365, 0.02);s.store_offset(1160, 1365, 0.02);}
        if ((!s.b[1540]) && (!s.b[1556])) {s.store_offset_sub_ad(1180, s.ad_value(1160), A::offset(s.ad_value(1365), 0.02), (-0.01));s.store_sqrt_square_offset(1181, 1180, 0.0001);s.store_add_scaled_inputs3_offset_indices(1364, 1365, 1.0, 1180, 0.5, 1181, 0.5, 0.02);}
        if (!s.b[1540]) {s.store_offset_sub(1180, 1370, 1364, (-0.005));s.store_sqrt_square_offset(1181, 1180, 2.5e-5);s.store_scaled_add(1182, 1180, 1181, 0.5);s.store_div_scaled_product_indices(1183, 1182, 754, 1.0, 705, 1.0);s.store_add_scaled_product_indices(1367, 1364, 1.0, 1182, 1183, (-0.5));s.store_sub(1394, 1386, 1161);s.store_mul(1189, 585, 1168);}
        s.b[1557] = (((s.v[1394] - s.v[586]) / s.v[1189]) > 100.0);s.store_scalar(1557, if s.b[1557] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1557]) {s.store_scaled_offset_ad(1395, A::div_scaled_inputs2(s.ad_value(1394), 1.0, s.ad_value(586), (-1.0), s.ad_value(1189), 1.0), ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1558] = (((s.v[1394] - s.v[586]) / s.v[1189]) < (-100.0));s.store_scalar(1558, if s.b[1558] { 1.0 } else { 0.0 });
        if (((!s.b[1540]) && (!s.b[1557])) && s.b[1558]) {s.store_scalar(1395, 3.720075976e-44);}
        if (((!s.b[1540]) && (!s.b[1557])) && (!s.b[1558])) {s.store_exp_ad(1395, A::div_scaled_inputs2(s.ad_value(1394), 1.0, s.ad_value(586), (-1.0), s.ad_value(1189), 1.0));}
        if (!s.b[1540]) {s.store_mul_ln_mixed_ia(1398, 1189, A::offset(s.ad_value(1395), 1.0));s.store_sub(1396, 1161, 1386);}
        s.b[1559] = (((s.v[1396] - s.v[586]) / s.v[1189]) > 100.0);s.store_scalar(1559, if s.b[1559] { 1.0 } else { 0.0 });
        if ((!s.b[1540]) && s.b[1559]) {s.store_scaled_offset_ad(1397, A::div_scaled_inputs2(s.ad_value(1396), 1.0, s.ad_value(586), (-1.0), s.ad_value(1189), 1.0), ((1.0) + ((-100.0))), 2.688117142e43);}
        s.b[1560] = (((s.v[1396] - s.v[586]) / s.v[1189]) < (-100.0));s.store_scalar(1560, if s.b[1560] { 1.0 } else { 0.0 });
        if (((!s.b[1540]) && (!s.b[1559])) && s.b[1560]) {s.store_scalar(1397, 3.720075976e-44);}
        if (((!s.b[1540]) && (!s.b[1559])) && (!s.b[1560])) {s.store_exp_ad(1397, A::div_scaled_inputs2(s.ad_value(1396), 1.0, s.ad_value(586), (-1.0), s.ad_value(1189), 1.0));}
        if (!s.b[1540]) {s.store_mul_ln_mixed_ia(1399, 1189, A::offset(s.ad_value(1397), 1.0));s.store_mul_product3_indices(1180, 1168, 592, 737, 1168, 1.0);s.store_add_scaled_product_mixed_iia(1181, 1399, 1.0, 707, A::sqrt(s.ad_value(1277)), 2.0);s.store_offset_div_scaled_product_indices(1179, 1399, 1181, 1.0, 1180, 1.0, 1.0);}
        if (!s.b[1540]) {
            s.store_add_scaled_product_mixed_iia(1383, 1277, 1.0, 1168, {
                if (s.v[1179] > 1e-38) {
                    A::ln(s.ad_value(1179))
                } else {
                    A::neg(A::constant(87.49823353377374))
                }
            }, 1.0);
        }
        if (!s.b[1540]) {s.store_div_add_scaled_inputs_rhs_mixed_ia(1179, 757, 757, 1.0, A::div_scalar_offset_denominator(1.0, A::div_from_scalar(1.0, s.ad_value(754)), (1.0 / s.v[1248]), 1.0), 1.0);s.store_add_scaled_product_indices(1384, 1383, 1.0, 1179, 1398, (-1.0));}
    }
}

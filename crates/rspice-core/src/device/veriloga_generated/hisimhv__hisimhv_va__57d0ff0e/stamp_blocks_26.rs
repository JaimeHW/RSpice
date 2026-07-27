#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_195(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[3277] && s.b[3278]) && (!s.b[3279])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p[504]));
            }
        }
        if (s.b[3277] && s.b[3278]) {s.store_mul_ad_affine_product_rhs(893, 842, s.ad_value(833), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p[504])), 0.0);}
        if (s.b[3277] && (!s.b[3278])) {s.copy_ad(335, 833);s.store_div_scaled_inputs_indices(336, 833, p[504], 842, 1.0);s.store_mul_add_scaled_product_rhs_indices(893, 860, 335, 1.0, 860, 336, 0.5);}
        if (!s.b[3277]) {s.store_scalar(893, 0.0);}
        s.b[3280] = (p[48] > 0.0);s.store_scalar(3280, if s.b[3280] { 1.0 } else { 0.0 });s.b[3281] = (s.v[834] > 0.0);s.store_scalar(3281, if s.b[3281] { 1.0 } else { 0.0 });s.b[3282] = (s.v[868] < 0.0);s.store_scalar(3282, if s.b[3282] { 1.0 } else { 0.0 });
        if ((s.b[3280] && s.b[3281]) && s.b[3282]) {s.store_sub_from_scalar_div_indices(770, 1.0, 868, 843);}
        s.b[3283] = (p[505] == 0.5);s.store_scalar(3283, if s.b[3283] { 1.0 } else { 0.0 });
        if (((s.b[3280] && s.b[3281]) && s.b[3282]) && s.b[3283]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
        if (((s.b[3280] && s.b[3281]) && s.b[3282]) && (!s.b[3283])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p[505]));
            }
        }
        if ((s.b[3280] && s.b[3281]) && s.b[3282]) {s.store_mul_ad_affine_product_rhs(895, 843, s.ad_value(834), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p[505])), 0.0);}
        if ((s.b[3280] && s.b[3281]) && (!s.b[3282])) {s.copy_ad(335, 834);s.store_div_scaled_inputs_indices(336, 834, p[505], 843, 1.0);s.store_mul_add_scaled_product_rhs_indices(895, 868, 335, 1.0, 868, 336, 0.5);}
        if (s.b[3280] && (!s.b[3281])) {s.store_scalar(895, 0.0);}
        s.b[3284] = (s.v[834] > 0.0);s.store_scalar(3284, if s.b[3284] { 1.0 } else { 0.0 });s.b[3285] = (s.v[860] < 0.0);s.store_scalar(3285, if s.b[3285] { 1.0 } else { 0.0 });
        if (((!s.b[3280]) && s.b[3284]) && s.b[3285]) {s.store_sub_from_scalar_div_indices(770, 1.0, 860, 843);}
        s.b[3286] = (p[505] == 0.5);s.store_scalar(3286, if s.b[3286] { 1.0 } else { 0.0 });
        if ((((!s.b[3280]) && s.b[3284]) && s.b[3285]) && s.b[3286]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
        if ((((!s.b[3280]) && s.b[3284]) && s.b[3285]) && (!s.b[3286])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p[505]));
            }
        }
        if (((!s.b[3280]) && s.b[3284]) && s.b[3285]) {s.store_mul_ad_affine_product_rhs(895, 843, s.ad_value(834), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p[505])), 0.0);}
        if (((!s.b[3280]) && s.b[3284]) && (!s.b[3285])) {s.copy_ad(335, 834);s.store_div_scaled_inputs_indices(336, 834, p[505], 843, 1.0);s.store_mul_add_scaled_product_rhs_indices(895, 860, 335, 1.0, 860, 336, 0.5);}
        if ((!s.b[3280]) && (!s.b[3284])) {s.store_scalar(895, 0.0);}
        s.b[3287] = (s.v[835] > 0.0);s.store_scalar(3287, if s.b[3287] { 1.0 } else { 0.0 });s.b[3288] = (s.v[859] < 0.0);s.store_scalar(3288, if s.b[3288] { 1.0 } else { 0.0 });
        if (s.b[3287] && s.b[3288]) {s.store_sub_from_scalar_div_indices(770, 1.0, 859, 844);}
        s.b[3289] = (p[526] == 0.5);s.store_scalar(3289, if s.b[3289] { 1.0 } else { 0.0 });
        if ((s.b[3287] && s.b[3288]) && s.b[3289]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
        if ((s.b[3287] && s.b[3288]) && (!s.b[3289])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p[526]));
            }
        }
        if (s.b[3287] && s.b[3288]) {s.store_mul_ad_affine_product_rhs(892, 844, s.ad_value(835), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p[526])), 0.0);}
        if (s.b[3287] && (!s.b[3288])) {s.copy_ad(335, 835);s.store_div_scaled_inputs_indices(336, 835, p[526], 844, 1.0);s.store_mul_add_scaled_product_rhs_indices(892, 859, 335, 1.0, 859, 336, 0.5);}
        if (!s.b[3287]) {s.store_scalar(892, 0.0);}
        s.b[3290] = (s.v[838] > 0.0);s.store_scalar(3290, if s.b[3290] { 1.0 } else { 0.0 });s.b[3291] = (s.v[859] < 0.0);s.store_scalar(3291, if s.b[3291] { 1.0 } else { 0.0 });
        if (s.b[3290] && s.b[3291]) {s.store_sub_from_scalar_div_indices(770, 1.0, 859, 845);}
        s.b[3292] = (p[527] == 0.5);s.store_scalar(3292, if s.b[3292] { 1.0 } else { 0.0 });
        if ((s.b[3290] && s.b[3291]) && s.b[3292]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_196(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[3290] && s.b[3291]) && (!s.b[3292])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p[527]));
            }
        }
        if (s.b[3290] && s.b[3291]) {s.store_mul_ad_affine_product_rhs(894, 845, s.ad_value(838), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p[527])), 0.0);}
        if (s.b[3290] && (!s.b[3291])) {s.copy_ad(335, 838);s.store_div_scaled_inputs_indices(336, 838, p[527], 845, 1.0);s.store_mul_add_scaled_product_rhs_indices(894, 859, 335, 1.0, 859, 336, 0.5);}
        if (!s.b[3290]) {s.store_scalar(894, 0.0);}
        s.b[3293] = (p[48] > 0.0);s.store_scalar(3293, if s.b[3293] { 1.0 } else { 0.0 });s.b[3294] = (s.v[839] > 0.0);s.store_scalar(3294, if s.b[3294] { 1.0 } else { 0.0 });s.b[3295] = (s.v[867] < 0.0);s.store_scalar(3295, if s.b[3295] { 1.0 } else { 0.0 });
        if ((s.b[3293] && s.b[3294]) && s.b[3295]) {s.store_sub_from_scalar_div_indices(770, 1.0, 867, 846);}
        s.b[3296] = (p[528] == 0.5);s.store_scalar(3296, if s.b[3296] { 1.0 } else { 0.0 });
        if (((s.b[3293] && s.b[3294]) && s.b[3295]) && s.b[3296]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
        if (((s.b[3293] && s.b[3294]) && s.b[3295]) && (!s.b[3296])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p[528]));
            }
        }
        if ((s.b[3293] && s.b[3294]) && s.b[3295]) {s.store_mul_ad_affine_product_rhs(896, 846, s.ad_value(839), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p[528])), 0.0);}
        if ((s.b[3293] && s.b[3294]) && (!s.b[3295])) {s.copy_ad(335, 839);s.store_div_scaled_inputs_indices(336, 839, p[528], 846, 1.0);s.store_mul_add_scaled_product_rhs_indices(896, 867, 335, 1.0, 867, 336, 0.5);}
        if (s.b[3293] && (!s.b[3294])) {s.store_scalar(896, 0.0);}
        s.b[3297] = (s.v[839] > 0.0);s.store_scalar(3297, if s.b[3297] { 1.0 } else { 0.0 });s.b[3298] = (s.v[859] < 0.0);s.store_scalar(3298, if s.b[3298] { 1.0 } else { 0.0 });
        if (((!s.b[3293]) && s.b[3297]) && s.b[3298]) {s.store_sub_from_scalar_div_indices(770, 1.0, 859, 846);}
        s.b[3299] = (p[528] == 0.5);s.store_scalar(3299, if s.b[3299] { 1.0 } else { 0.0 });
        if ((((!s.b[3293]) && s.b[3297]) && s.b[3298]) && s.b[3299]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
        if ((((!s.b[3293]) && s.b[3297]) && s.b[3298]) && (!s.b[3299])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p[528]));
            }
        }
        if (((!s.b[3293]) && s.b[3297]) && s.b[3298]) {s.store_mul_ad_affine_product_rhs(896, 846, s.ad_value(839), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p[528])), 0.0);}
        if (((!s.b[3293]) && s.b[3297]) && (!s.b[3298])) {s.copy_ad(335, 839);s.store_div_scaled_inputs_indices(336, 839, p[528], 846, 1.0);s.store_mul_add_scaled_product_rhs_indices(896, 859, 335, 1.0, 859, 336, 0.5);}
        if ((!s.b[3293]) && (!s.b[3297])) {s.store_scalar(896, 0.0);}
        s.b[3300] = (p[48] > 0.0);s.store_scalar(3300, if s.b[3300] { 1.0 } else { 0.0 });
        if s.b[3300] {s.store_scaled_add(66, 892, 894, s.v[365]);s.store_scaled_add(65, 891, 893, s.v[365]);s.store_scale(68, 896, s.v[365]);s.store_scale(67, 895, s.v[365]);}
        if (!s.b[3300]) {s.store_add_scaled_inputs3_indices(66, 892, s.v[365], 894, s.v[365], 896, s.v[365]);s.store_add_scaled_inputs3_indices(65, 891, s.v[365], 893, s.v[365], 895, s.v[365]);s.store_scalar(68, 0.0);s.store_scalar(67, 0.0);}
        s.store_scalar(903, (p[540] / 1e-6));s.store_scalar(906, s.v[820]);s.store_scalar(904, (1450.0 / 10000.0));s.store_scalar(905, (500.0 / 10000.0));s.store_scalar(943, 0.001);s.store_scale_ad(908, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[499]), 1.0 / (s.v[820])), 1.45e16);s.store_scaled_square(907, 908, 1.0 / (s.v[903]));s.store_powf(335, 676, (-1.5));s.store_scaled_mul(909, 335, 155, s.v[904]);s.store_scaled_mul(910, 335, 155, s.v[905]);s.store_div_scaled_product_add_scaled_denominator_indices(911, 909, 910, 2.0, 909, 1.0, 910, 1.0, 1.0);s.store_powf(336, 676, p[547]);s.store_scale(913, 336, p[544]);s.store_sqrt_mul(912, 913, 911);s.store_mul_scaled_ln_ad_rhs(934, 155, s.v[906], A::div_from_scalar(s.v[903], s.ad_value(907)));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_197(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.store_mul_add_scaled_inputs_rhs(935, 155, A::ln(A::div_from_scalar(s.v[903], s.ad_value(907))), s.v[906], A::div_from_scalar(p[545], s.ad_value(912)), s.v[906]);s.b[3301] = (p[539] > 0.0);s.store_scalar(3301, if s.b[3301] { 1.0 } else { 0.0 });
        if s.b[3301] {s.store_scalar(936, s.v[820]);s.store_exp_mul(937, 860, 850);}
        s.b[3302] = ((s.v[860] - (s.v[935] - s.v[934])) > 0.0);s.store_scalar(3302, if s.b[3302] { 1.0 } else { 0.0 });
        if (s.b[3301] && s.b[3302]) {s.store_exp_ad(938, A::mul(s.ad_value(154), A::sub(A::div(s.ad_value(860), s.ad_value(936)), A::div_scaled_inputs2(s.ad_value(935), 1.0, s.ad_value(934), (-1.0), s.ad_value(936), 1.0))));}
        if (s.b[3301] && (!s.b[3302])) {s.store_scalar(938, 1.0);}
        s.b[3303] = ((p[542] == 0.0) || (s.v[860] < s.v[934]));s.store_scalar(3303, if s.b[3303] { 1.0 } else { 0.0 });
        if (s.b[3301] && s.b[3303]) {s.store_scale(941, 937, p[541]);}
        if (s.b[3301] && (!s.b[3303])) {s.store_mul_scaled_exp_ad_rhs(941, 937, p[541], A::mul3_scaled_output(A::sub(s.ad_value(860), s.ad_value(934)), A::sub(s.ad_value(860), s.ad_value(934)), A::exp_scaled_input(A::ln(A::div_from_scalar(1.0, s.ad_value(676))), p[548]), (-p[542])));}
        if s.b[3301] {
            if (s.v[941] > 1e20) {
                s.store_scalar(941, 1e20);
            } else {
            }
        }
        if s.b[3301] {s.store_mul(939, 907, 941);s.store_scaled_sub(920, 939, 907, (1.6021918e-19 * p[13]));}
        s.b[3304] = (p[543] > 0.0);s.store_scalar(3304, if s.b[3304] { 1.0 } else { 0.0 });
        if (s.b[3301] && s.b[3304]) {s.store_scale(922, 920, p[543]);s.store_scaled_voltage(924, ctx, nodes, Some(16), None, p[543]);s.store_scaled_sub(926, 924, 922, 1.0 / (p[543]));s.store_scale(928, 924, 1.0 / (p[543]));}
        if (s.b[3301] && (!s.b[3304])) {s.copy_ad(922, 920);s.copy_ad(928, 922);}
        s.b[3305] = ((p[542] == 0.0) || (s.v[860] < s.v[935]));s.store_scalar(3305, if s.b[3305] { 1.0 } else { 0.0 });
        if (s.b[3301] && s.b[3305]) {s.store_scale(942, 938, p[541]);}
        if (s.b[3301] && (!s.b[3305])) {s.store_mul_scaled_exp_ad_rhs(942, 938, p[541], A::mul3_scaled_output(A::sub(s.ad_value(860), s.ad_value(935)), A::sub(s.ad_value(860), s.ad_value(935)), A::exp_scaled_input(A::ln(A::div_from_scalar(1.0, s.ad_value(676))), p[548]), (-p[542])));}
        if s.b[3301] {
            if (s.v[942] > 1e20) {
                s.store_scalar(942, 1e20);
            } else {
            }
        }
        if s.b[3301] {s.store_mul(940, 907, 942);s.store_scaled_sub(921, 940, 907, (1.6021918e-19 * p[13]));}
        s.b[3306] = (p[543] > 0.0);s.store_scalar(3306, if s.b[3306] { 1.0 } else { 0.0 });
        if (s.b[3301] && s.b[3306]) {s.store_scale(923, 921, p[543]);s.store_scaled_voltage(925, ctx, nodes, Some(17), None, p[543]);s.store_scaled_sub(927, 925, 923, 1.0 / (p[543]));s.store_scale(929, 925, 1.0 / (p[543]));}
        if (s.b[3301] && (!s.b[3306])) {s.copy_ad(923, 921);s.copy_ad(929, 923);}
        if s.b[3301] {s.store_sub_from_scalar(914, p[506], 860);s.store_sqrt_square_offset(782, 914, ((4.0 * s.v[943]) * s.v[943]));s.store_offset_scaled_div(334, 914, 782, 0.5, 0.5);s.store_scaled_add(914, 914, 782, 0.5);}
        s.b[3307] = (s.v[914] < 0.0);s.store_scalar(3307, if s.b[3307] { 1.0 } else { 0.0 });
        if (s.b[3301] && s.b[3307]) {s.store_scalar(914, 0.0);s.store_scalar(334, 0.0);}
        if s.b[3301] {s.store_sqrt_scaled_input(915, 914, ((2.0 * 1.034943e-10) * 1.0 / ((1.6021918e-19 * s.v[903]))));s.store_offset_sub_from_scalar_ad(781, p[545], s.ad_value(915), (-1e-7));s.store_scalar(782, ((4.0 * p[545]) * 1e-7));}
        if s.b[3301] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_198(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[3301] {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(915, 781, (-0.5), 782, (-0.5), p[545]);}
        s.b[3308] = (p[546] > 0.0);s.store_scalar(3308, if s.b[3308] { 1.0 } else { 0.0 });
        if (s.b[3301] && s.b[3308]) {s.store_scale(930, 915, p[546]);s.store_scaled_voltage(931, ctx, nodes, Some(18), None, p[546]);s.store_scaled_sub(932, 931, 930, 1.0 / (p[546]));s.store_scale(933, 931, 1.0 / (p[546]));}
        if (s.b[3301] && (!s.b[3308])) {s.copy_ad(930, 915);s.copy_ad(933, 930);}
        if s.b[3301] {s.store_scalar(916, ((-((s.v[903] * p[13]) * 1.6021918e-19)) * p[545]));s.store_mul_ad_product_rhs_mixed_ia(917, 912, 928, A::sub(A::exp(A::div_from_scalar((-p[545]), s.ad_value(912))), A::exp_div_scaled_inputs(s.ad_value(933), -1.0, s.ad_value(912), 1.0)));s.store_mul_ad_product_rhs_mixed_ia(918, 912, 929, A::offset(A::exp_div_scaled_inputs(A::sub_from_scalar(p[545], s.ad_value(933)), -1.0, s.ad_value(912), 1.0), (-1.0)));s.store_add_scaled_inputs3_indices(919, 916, (-1.0), 917, (-1.0), 918, (-1.0));s.store_add_scaled_inputs(65, 65, 1.0, 919, s.v[365]);}
        s.b[3309] = ((p[539] > 0.0) && (p[543] > 0.0));s.store_scalar(3309, if s.b[3309] { 1.0 } else { 0.0 });s.b[3310] = ((p[539] > 0.0) && (p[546] > 0.0));s.store_scalar(3310, if s.b[3310] { 1.0 } else { 0.0 });s.b[3311] = (p[46] == 1.0);s.store_scalar(3311, if s.b[3311] { 1.0 } else { 0.0 });s.b[3312] = ((s.v[486] > 0.0) && (s.v[454] > 0.0));s.store_scalar(3312, if s.b[3312] { 1.0 } else { 0.0 });
        if (s.b[3311] && s.b[3312]) {s.store_mul(335, 665, 85);s.store_scale(337, 636, 1.0 / ((s.v[188] * s.v[188])));s.store_scale_ad(338, A::div_from_scalar(2.0, s.ad_value(636)), (s.v[188] * s.v[188]));s.store_add_scaled_inputs_product_indices(339, 335, 1.0, 155, (-1.0), 666, 1438, (-1.0));s.store_offset_mul(340, 338, 339, 1.0);s.store_scaled_offset(341, 338, 1.0, 2.0);}
        s.b[3313] = ((s.v[340] < s.v[341]) && (s.v[341] >= 0.0));s.store_scalar(3313, if s.b[3313] { 1.0 } else { 0.0 });
        if ((s.b[3311] && s.b[3312]) && s.b[3313]) {s.store_sub(781, 341, 340);s.store_square(722, 781);s.store_square(723, 341);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3314] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(3314, if s.b[3314] { 1.0 } else { 0.0 });s.b[3315] = (4.0 == 1.0);s.store_scalar(3315, if s.b[3315] { 1.0 } else { 0.0 });
        if ((((s.b[3311] && s.b[3312]) && s.b[3313]) && s.b[3314]) && s.b[3315]) {s.store_scalar(720, 1.0);}
        s.b[3316] = (4.0 == 2.0);s.store_scalar(3316, if s.b[3316] { 1.0 } else { 0.0 });
        if (((((s.b[3311] && s.b[3312]) && s.b[3313]) && s.b[3314]) && (!s.b[3315])) && s.b[3316]) {s.store_scalar(720, 2.0);}
        s.b[3317] = (4.0 == 4.0);s.store_scalar(3317, if s.b[3317] { 1.0 } else { 0.0 });
        if ((((((s.b[3311] && s.b[3312]) && s.b[3313]) && s.b[3314]) && (!s.b[3315])) && (!s.b[3316])) && s.b[3317]) {s.store_scalar(720, 3.0);}
        s.b[3318] = (4.0 == 8.0);s.store_scalar(3318, if s.b[3318] { 1.0 } else { 0.0 });
        if (((((((s.b[3311] && s.b[3312]) && s.b[3313]) && s.b[3314]) && (!s.b[3315])) && (!s.b[3316])) && (!s.b[3317])) && s.b[3318]) {s.store_scalar(720, 4.0);}
        if (((s.b[3311] && s.b[3312]) && s.b[3313]) && s.b[3314]) {s.store_scalar(719, 0.0);}
        let mut t1: usize = 0;
        while {
            let t0: f64 = if ((((s.b[3311] && s.b[3312]) && s.b[3313]) && s.b[3314]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t0 != 0.0
        } {
            t1 += 1;
            if t1 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[3311] && s.b[3312]) && s.b[3313]) && s.b[3314]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[3311] && s.b[3312]) && s.b[3313]) && (!s.b[3314])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if ((s.b[3311] && s.b[3312]) && s.b[3313]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 341, 726);s.store_div_scaled_product3_indices(334, 341, 725, 726, 1.0, 770, 1.0);s.store_sub(340, 341, 780);}
        if ((s.b[3311] && s.b[3312]) && s.b[3313]) {
        }
        if ((s.b[3311] && s.b[3312]) && (!s.b[3313])) {
        }
        if ((s.b[3311] && s.b[3312]) && (!s.b[3313])) {s.store_scalar(334, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_199(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[3311] && s.b[3312]) {s.store_sqrt(340, 340);s.store_add_mul_sub_from_scalar_rhs_indices(282, 335, 337, 1.0, 340);s.store_div_from_scalar_offset_input(336, s.v[582], 667, s.v[582]);s.store_add_scaled_inputs_product_indices(283, 1439, s.v[488], 109, 1.0, 336, 282, (-1.0));s.store_sqrt_square_offset(782, 283, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(343, 283, 782, 0.5, 0.5);s.store_scaled_add(283, 283, 782, 0.5);}
        s.b[3319] = (s.v[283] < 0.0);s.store_scalar(3319, if s.b[3319] { 1.0 } else { 0.0 });
        if ((s.b[3311] && s.b[3312]) && s.b[3319]) {s.store_scalar(283, 0.0);s.store_scalar(343, 0.0);}
        if (s.b[3311] && s.b[3312]) {s.store_offset(283, 283, 1e-25);s.store_offset_mul_offset_rhs(958, 957, 387, (-s.v[764]), 1.0);}
        if (s.b[3311] && s.b[3312]) {
            if (s.v[958] <= 0.001) {
                s.store_scalar(958, 0.001);
            } else {
            }
        }
        if (s.b[3311] && s.b[3312]) {s.store_div(339, 668, 958);s.store_mul(340, 669, 958);s.store_ad_value(336, A::exp_div_scaled_inputs(s.ad_value(340), -1.0, s.ad_value(283), 1.0));}
        s.b[3321] = (s.v[78] == 0.0);s.store_scalar(3321, if s.b[3321] { 1.0 } else { 0.0 });
        if ((s.v[81] != 0.0) && s.b[3321]) {s.store_scalar(346, p[270]);s.store_scalar(344, p[271]);s.copy_ad(337, 170);s.store_mul_product3_indices(335, 337, 346, 344, 337, 1.0);s.store_offset_add_ad(336, A::mul3(s.ad_value(253), s.ad_value(127), s.ad_value(346)), A::mul3(s.ad_value(344), s.ad_value(337), s.ad_value(337)), 1e-25);}
        if (s.v[81] != 0.0) {s.store_scalar(336, s.v[565]);}
        s.b[3322] = ((p[26] != 0.0) && (s.v[78] == 0.0));s.store_scalar(3322, if s.b[3322] { 1.0 } else { 0.0 });
        if s.b[3322] {s.store_scalar(309, s.v[522]);s.store_scalar(311, s.v[563]);s.store_scale(335, 238, 6.241449993689894e18);s.store_sqrt_offset_ad(782, A::square(A::sub(s.ad_value(87), s.ad_value(1435))), ((4.0 * 0.001) * 0.001));s.store_scaled_offset_ad(334, A::div_scaled_inputs2(s.ad_value(87), 1.0, s.ad_value(1435), (-1.0), s.ad_value(782), 1.0), 1.0, 0.5);s.store_add_scaled_inputs3_indices(339, 87, 0.5, 1435, ((-1.0) * 0.5), 782, 0.5);}
        s.b[3323] = (s.v[339] < 0.0);s.store_scalar(3323, if s.b[3323] { 1.0 } else { 0.0 });
        if (s.b[3322] && s.b[3323]) {s.store_scalar(339, 0.0);s.store_scalar(334, 0.0);}
        if s.b[3322] {s.store_mul_scale_offset_mixed_ai(336, A::add_scaled_inputs3(s.ad_value(185), 1.0, A::div(s.ad_value(238), s.ad_value(339)), 1.0, s.ad_value(311), 1.0), 155, 6.241449993689894e18, 0.0);s.store_sub_mixed_ai(337, A::div_scaled_inputs(s.ad_value(979), (((-2.0) * 6.241449993689894e18) * 1.0 / (s.v[635])), s.ad_value(170), 1.0), 335);}
        s.b[3324] = ((((s.v[337] - s.v[335])) as f64).abs() > (10.0 * 2.220446049250313e-16));s.store_scalar(3324, if s.b[3324] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_200(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[3322] && s.b[3324]) {s.store_add_scaled_value_products_mixed_aaaai(338, A::div_scalar_by_product(1.0, A::add(s.ad_value(335), s.ad_value(336)), A::add(s.ad_value(337), s.ad_value(336)), 1.0), 1.0, A::div_scaled_product3(s.ad_value(309), s.ad_value(255), s.ad_value(253), 2.0, A::sub(s.ad_value(337), s.ad_value(335)), 1.0), A::ln(A::div_scaled_inputs2(s.ad_value(337), 1.0, s.ad_value(336), 1.0, A::add(s.ad_value(335), s.ad_value(336)), 1.0)), 1.0, A::mul3(A::mul3(s.ad_value(309), s.ad_value(255), s.ad_value(253)), s.ad_value(309), s.ad_value(255)), 253, 1.0);}
        if (s.b[3322] && (!s.b[3324])) {s.store_add_scaled_inputs_product_mixed_aaai(338, A::div_scalar_by_product(1.0, A::add(s.ad_value(335), s.ad_value(336)), A::add(s.ad_value(337), s.ad_value(336)), 1.0), 1.0, A::div_scaled_product3(s.ad_value(309), s.ad_value(255), s.ad_value(253), 2.0, A::add(s.ad_value(335), s.ad_value(336)), 1.0), 1.0, A::mul3(A::mul3(s.ad_value(309), s.ad_value(255), s.ad_value(253)), s.ad_value(309), s.ad_value(255)), 253, 1.0);}
        s.b[3325] = (((p[30] != 0.0) && (s.v[78] == 0.0)) && (s.v[963] == 0.0));s.store_scalar(3325, if s.b[3325] { 1.0 } else { 0.0 });
        if s.b[3325] {s.store_div_scaled_offset_numerator_mixed_ai(313, A::sub(s.ad_value(168), s.ad_value(87)), 1.0, (10.0 * 2.220446049250313e-16), 170, 1.0);}
        if s.b[3325] {
            if (s.v[313] >= 0.0) {
            } else {
                s.store_scalar(313, 0.0);
            }
        }
        if s.b[3325] {s.store_scaled_mul(346, 254, 313, 1e-7);}
        s.b[3326] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3326, if s.b[3326] { 1.0 } else { 0.0 });
        if (s.b[3325] && s.b[3326]) {s.store_scalar(341, 1.0);}
        s.b[3327] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3327, if s.b[3327] { 1.0 } else { 0.0 });
        if ((s.b[3325] && (!s.b[3326])) && s.b[3327]) {s.copy_ad(341, 346);}
        if ((s.b[3325] && (!s.b[3326])) && (!s.b[3327])) {
            if (s.v[313] == 0.0) {
                s.store_scalar(341, 0.0);
            } else {
                s.store_powf(341, 313, (p[178] - 1.0));
            }
        }
        if s.b[3325] {s.store_mul(342, 346, 341);s.store_offset(343, 342, 1.0);}
        if s.b[3325] {
            if (s.v[343] == 0.0) {
                s.store_scalar(344, 0.0);
            } else {
                s.store_powf(344, 343, (((-1.0) / p[178]) - 1.0));
            }
        }
        if s.b[3325] {s.store_mul(345, 343, 344);s.store_mul(316, 254, 345);s.store_scaled_add(314, 253, 316, 0.5);s.store_square(334, 125);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_201(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[3325] {s.store_div_scaled_product_by_product_mixed_aaai(315, A::mul3_scaled_output(s.ad_value(185), s.ad_value(127), s.ad_value(253), s.v[632]), A::add_scaled_inputs3(A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(125), 3.0, 1.0), 1.0, s.ad_value(334), 6.0), s.ad_value(316), s.ad_value(316)), 1.0, A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(125), 4.0, 3.0), 1.0, s.ad_value(334), 3.0), s.ad_value(316), s.ad_value(253)), 1.0, A::mul3(A::add(A::scale_offset(s.ad_value(125), 3.0, 6.0), s.ad_value(334)), s.ad_value(253), s.ad_value(253)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(170), A::offset(s.ad_value(125), 1.0), s.ad_value(314), 15.0), 314, 1.0);}
        if (!s.b[3325]) {s.store_scalar(315, 0.0);}
        s.b[3328] = (((((p[31] != 0.0) && (p[30] != 0.0)) && (s.v[321] == 1.0)) && (s.v[78] == 0.0)) && (s.v[963] == 0.0));s.store_scalar(3328, if s.b[3328] { 1.0 } else { 0.0 });
        if s.b[3328] {s.store_sqrt(322, 319);s.store_add(336, 127, 322);s.store_square(337, 317);s.store_square(338, 319);s.store_scaled_mul(339, 317, 319, 42.0);s.store_add_scaled_inputs3_indices(339, 339, 1.0, 337, 4.0, 338, 4.0);s.store_add_product3_rhs_mixed_iia(339, 339, 322, 127, A::add(s.ad_value(317), s.ad_value(319)), 20.0);s.store_square(344, 336);s.store_square(344, 344);s.store_div_scaled_value_by_product_indices(323, 339, 1.0, 344, 336, 1.0);s.store_mul_ad_product_lhs_mixed_ai(324, A::div_from_scalar(s.v[632], s.ad_value(170)), 253, 185);s.store_add_mixed_ai(341, A::add_scaled_product(s.ad_value(317), 1.0, s.ad_value(127), s.ad_value(322), 4.0), 319);}
        s.store_scale(0, 134, s.v[365]);s.store_scale(699, 400, s.v[365]);s.store_scalar(705, 0.0);s.store_scalar(706, 0.0);s.store_scalar(707, 0.0);s.store_scalar(811, 0.0);s.store_scalar(810, 0.0);s.store_scalar(812, 0.0);s.store_scalar(703, 0.0);s.store_scalar(704, 0.0);s.b[3329] = ((s.v[81] != 0.0) || (p[22] == 2.0));s.store_scalar(3329, if s.b[3329] { 1.0 } else { 0.0 });
        if s.b[3329] {s.store_scalar(700, 0.0);s.store_scalar(701, 0.0);s.store_scalar(702, 0.0);s.copy_ad(708, 247);s.store_scale(132, 132, s.v[365]);}
        if (!s.b[3329]) {s.store_scaled_add(700, 20, 132, (-s.v[365]));s.store_scale(701, 19, s.v[365]);s.store_scaled_sub(702, 132, 19, s.v[365]);}
        if (p[29] != 0.0) {s.store_scale(572, 91, s.v[572]);s.store_sqrt_square_offset(782, 572, ((4.0 * 1e-12) * 1e-12));s.store_offset_scaled_div(334, 572, 782, 0.5, 0.5);s.store_scaled_add(572, 572, 782, 0.5);}
        s.b[3330] = (s.v[572] < 0.0);s.store_scalar(3330, if s.b[3330] { 1.0 } else { 0.0 });
        if ((p[29] != 0.0) && s.b[3330]) {s.store_scalar(572, 0.0);s.store_scalar(334, 0.0);}
        if (p[29] != 0.0) {s.store_voltage(817, ctx, nodes, Some(14), None);s.store_add_scaled_inputs3_indices(352, 352, 1.0, 816, -1.0, 817, 1.0);s.copy_ad(355, 817);}
        if (p[29] == 0.0) {s.copy_ad(817, 816);}
        s.b[3331] = (p[22] > 0.0);s.store_scalar(3331, if s.b[3331] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_202(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if s.b[3331] {s.store_scaled_add_mixed_ai(811, A::add_scaled_inputs4(s.ad_value(293), 1.0, s.ad_value(352), (-1.0), s.ad_value(353), -1.0, s.ad_value(291), 1.0), 292, s.v[365]);s.store_scaled_sub(810, 355, 292, s.v[365]);s.store_scaled_sub(812, 356, 291, s.v[365]);s.store_add_scaled_inputs4_indices(700, 700, 1.0, 305, s.v[365], 360, ((-1.0) * s.v[365]), 362, (-s.v[365]));s.store_add_scaled_inputs3_indices(701, 701, 1.0, 361, s.v[365], 305, (-s.v[365]));s.store_add_scaled_inputs(702, 702, 1.0, 363, s.v[365]);s.store_sub_scaled_inputs(705, 350, (-s.v[365]), 351, s.v[365]);s.store_scale(706, 358, s.v[365]);s.store_scale(707, 359, s.v[365]);s.store_offset_sub_scaled_inputs_indices(703, 299, (-s.v[365]), 298, s.v[365], s.v[703]);s.store_offset_sub_scaled_inputs_indices(704, 301, (-s.v[365]), 297, s.v[365], s.v[704]);}
        s.store_scaled_add(709, 280, 287, s.v[365]);s.store_scale(710, 281, s.v[365]);s.store_scale(807, 387, (4.0 * 1.3806226e-23));s.store_scale(712, 315, s.v[365]);s.store_scalar(22, A::ddx_projection(&s.ad_value(700), Some(6), None));s.store_scale(22, 22, p[87]);s.store_scalar(23, A::ddx_projection(&s.ad_value(700), Some(8), None));s.store_scale(23, 23, p[87]);
        if (s.v[949] > 0.0) {
            s.copy_ad(757, 23);
        } else {
            s.copy_ad(757, 22);
        }
        s.store_scalar(713, 0.0);s.b[3334] = (((((p[31] != 0.0) && (p[30] != 0.0)) && (s.v[321] == 1.0)) && (s.v[78] == 0.0)) && (s.v[963] == 0.0));s.store_scalar(3334, if s.b[3334] { 1.0 } else { 0.0 });
        if s.b[3334] {s.store_scaled_mul(334, 185, 162, (1e-6 * s.v[635]));s.store_scale(344, 757, 1.0 / (s.v[365]));s.store_div_scaled_product3_indices(328, 155, 344, 344, (0.1185185185185185 * 1.6021918e-19), 324, 1.0);}
        s.b[3335] = ((s.v[320] > (10.0 * 2.220446049250313e-16)) && (s.v[790] > (10.0 * 2.220446049250313e-16)));s.store_scalar(3335, if s.b[3335] { 1.0 } else { 0.0 });
        if (s.b[3334] && s.b[3335]) {s.store_div(329, 254, 253);s.store_div_scaled_inputs2_mixed_aii(330, A::div(s.ad_value(254), s.ad_value(316)), 1.0, 329, (-1.0), 790, 1.0);s.store_add_mixed_ia(331, 329, A::div_scaled_product(s.ad_value(330), A::add(A::add_scaled_product(s.ad_value(317), 1.0, s.ad_value(127), s.ad_value(322), 1.0), s.ad_value(319)), 0.6666666666666667, A::add(s.ad_value(127), s.ad_value(322)), 1.0));}
        if (s.b[3334] && (!s.b[3335])) {s.store_div(331, 254, 316);}
        if s.b[3334] {s.store_mul3_affine_lhs(713, 328, 323, s.v[365], 0.0, 331);}
        if s.b[3334] {
            if (s.v[713] < 0.0) {
                s.store_scalar(713, 0.0);
            } else {
            }
        }
        if s.b[3334] {
            if ((-s.v[344]) > s.v[334]) {
            } else {
                s.store_scalar(713, 0.0);
            }
        }
        s.store_mul(952, 807, 712);
        if ((s.v[952] > 0.0) && (s.v[713] > 0.0)) {
            s.store_sqrt_div(953, 713, 952);
        } else {
            s.store_scalar(953, 0.0);
        }
        if (s.v[949] > 0.0) {
            s.store_mul_scale_offset_indices(954, 953, 247, -1.0, 1.0);
        } else {
            s.store_mul(954, 953, 247);
        }
        if (s.v[949] > 0.0) {
            s.store_mul(955, 953, 247);
        } else {
            s.store_mul_scale_offset_indices(955, 953, 247, -1.0, 1.0);
        }
        s.store_scalar(716, 0.0);s.store_scalar(715, 0.0);s.b[3336] = (s.v[449] == 1.0);s.store_scalar(3336, if s.b[3336] { 1.0 } else { 0.0 });s.b[3337] = (s.v[76] == 0.0);s.store_scalar(3337, if s.b[3337] { 1.0 } else { 0.0 });s.b[3338] = ((p[53] > 0.0) && (s.v[541] != 0.0));s.store_scalar(3338, if s.b[3338] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3337])) && s.b[3338]) {
            if (s.v[676] == 0.0) {
                s.store_scalar(335, 0.0);
            } else {
                s.store_powf(335, 676, p[416]);
            }
        }
        if ((s.b[3336] && (!s.b[3337])) && s.b[3338]) {s.store_div_from_scalar(794, s.v[569], 335);s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p[418]), p[418]));s.store_div_from_scalar(795, s.v[570], 334);s.store_add_mixed_ia(959, 959, A::scaled_offset(s.ad_value(387), (-s.v[764]), p[439]));}
        if ((s.b[3336] && (!s.b[3337])) && (!s.b[3338])) {s.store_scalar(387, (ctx_temp + p[11]));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_203(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[3336] && (!s.b[3337])) {s.store_scalar(164, (s.v[630] * p[7]));s.store_scalar(604, p[71]);s.store_scalar(605, s.v[460]);s.store_mul(606, 794, 653);s.store_offset_product3(607, s.ad_value(795), s.ad_value(786), s.ad_value(652), 1.0, 1e-25);s.store_div(608, 804, 604);s.store_mul(609, 606, 608);}
        s.b[3339] = (s.v[804] >= 0.0);s.store_scalar(3339, if s.b[3339] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3337])) && s.b[3339]) {s.store_div(335, 609, 607);}
        if ((s.b[3336] && (!s.b[3337])) && (!s.b[3339])) {s.store_div_scaled_inputs_indices(335, 609, -1.0, 607, 1.0);}
        s.b[3340] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[959]) && (s.v[959] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3340, if s.b[3340] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3337])) && s.b[3340]) {s.store_scalar(337, 1.0);}
        s.b[3341] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[959]) && (s.v[959] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3341, if s.b[3341] { 1.0 } else { 0.0 });
        if (((s.b[3336] && (!s.b[3337])) && (!s.b[3340])) && s.b[3341]) {s.copy_ad(337, 335);}
        if (((s.b[3336] && (!s.b[3337])) && (!s.b[3340])) && (!s.b[3341])) {s.store_pow_offset_rhs(337, 335, 959, (-1.0));}
        if (s.b[3336] && (!s.b[3337])) {s.store_mul(336, 335, 337);s.store_offset(338, 336, 1.0);}
        s.b[3342] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[959]) && (s.v[959] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3342, if s.b[3342] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3337])) && s.b[3342]) {s.store_div_from_scalar(339, 1.0, 338);}
        s.b[3343] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[959]) && (s.v[959] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3343, if s.b[3343] { 1.0 } else { 0.0 });
        if (((s.b[3336] && (!s.b[3337])) && (!s.b[3342])) && s.b[3343]) {s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));}
        if (((s.b[3336] && (!s.b[3337])) && (!s.b[3342])) && (!s.b[3343])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_pow_ad(340, s.ad_value(338), A::offset(A::div_from_scalar((-1.0), s.ad_value(959)), (-1.0)));
            }
        }
        if (((s.b[3336] && (!s.b[3337])) && (!s.b[3342])) && (!s.b[3343])) {s.store_mul(339, 338, 340);}
        if (s.b[3336] && (!s.b[3337])) {s.store_mul(610, 606, 339);s.copy_ad(611, 605);s.copy_ad(612, 614);s.store_div_from_scalar(335, 1.6021918e-19, 604);s.store_mul_product3_indices(613, 611, 335, 612, 610, 1.0);}
        s.b[3344] = ((s.v[613] < 1e-25) && (1e-25 >= 0.0));s.store_scalar(3344, if s.b[3344] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3337])) && s.b[3344]) {s.store_sub_from_scalar(781, 1e-25, 613);s.store_square(722, 781);s.store_scalar(723, (1e-25 * 1e-25));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3345] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(3345, if s.b[3345] { 1.0 } else { 0.0 });s.b[3346] = (2.0 == 1.0);s.store_scalar(3346, if s.b[3346] { 1.0 } else { 0.0 });
        if ((((s.b[3336] && (!s.b[3337])) && s.b[3344]) && s.b[3345]) && s.b[3346]) {s.store_scalar(720, 1.0);}
        s.b[3347] = (2.0 == 2.0);s.store_scalar(3347, if s.b[3347] { 1.0 } else { 0.0 });
        if (((((s.b[3336] && (!s.b[3337])) && s.b[3344]) && s.b[3345]) && (!s.b[3346])) && s.b[3347]) {s.store_scalar(720, 2.0);}
        s.b[3348] = (2.0 == 4.0);s.store_scalar(3348, if s.b[3348] { 1.0 } else { 0.0 });
        if ((((((s.b[3336] && (!s.b[3337])) && s.b[3344]) && s.b[3345]) && (!s.b[3346])) && (!s.b[3347])) && s.b[3348]) {s.store_scalar(720, 3.0);}
        s.b[3349] = (2.0 == 8.0);s.store_scalar(3349, if s.b[3349] { 1.0 } else { 0.0 });
        if (((((((s.b[3336] && (!s.b[3337])) && s.b[3344]) && s.b[3345]) && (!s.b[3346])) && (!s.b[3347])) && (!s.b[3348])) && s.b[3349]) {s.store_scalar(720, 4.0);}
        if (((s.b[3336] && (!s.b[3337])) && s.b[3344]) && s.b[3345]) {s.store_scalar(719, 0.0);}
        let mut t3: usize = 0;
        while {
            let t2: f64 = if ((((s.b[3336] && (!s.b[3337])) && s.b[3344]) && s.b[3345]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t2 != 0.0
        } {
            t3 += 1;
            if t3 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t3, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[3336] && (!s.b[3337])) && s.b[3344]) && s.b[3345]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[3336] && (!s.b[3337])) && s.b[3344]) && (!s.b[3345])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[3336] && (!s.b[3337])) && s.b[3344]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-25);s.store_div_scaled_product_indices(334, 725, 726, 1e-25, 770, 1.0);s.store_sub_from_scalar(613, 1e-25, 780);}
        if ((s.b[3336] && (!s.b[3337])) && s.b[3344]) {
        }
        if ((s.b[3336] && (!s.b[3337])) && (!s.b[3344])) {
        }
        if ((s.b[3336] && (!s.b[3337])) && (!s.b[3344])) {s.store_scalar(334, 1.0);}
        if (s.b[3336] && (!s.b[3337])) {s.store_div_from_scalar(5, 1.0, 613);s.store_div(5, 5, 164);s.store_add(5, 5, 648);}
        s.b[3351] = (s.v[5] < p[444]);s.store_scalar(3351, if s.b[3351] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_204(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if ((s.b[3336] && (!s.b[3337])) && s.b[3351]) {s.store_scalar(5, p[444]);}
        if (s.b[3336] && (!s.b[3337])) {s.store_scale(716, 5, 1.0 / (s.v[365]));}
        s.b[3356] = (s.v[75] == 0.0);s.store_scalar(3356, if s.b[3356] { 1.0 } else { 0.0 });
        if (s.b[3336] && (!s.b[3356])) {s.copy_ad(3352, 729);s.copy_ad(3353, 728);}
        s.b[3357] = ((p[53] > 0.0) && (s.v[541] != 0.0));s.store_scalar(3357, if s.b[3357] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3356])) && s.b[3357]) {
            if (s.v[676] == 0.0) {
                s.store_scalar(335, 0.0);
            } else {
                s.store_powf(335, 676, p[415]);
            }
        }
        if ((s.b[3336] && (!s.b[3356])) && s.b[3357]) {s.store_div_from_scalar(787, s.v[567], 335);s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p[417]), p[417]));s.store_div_from_scalar(788, s.v[568], 334);s.store_add_mixed_ia(956, 956, A::scaled_offset(s.ad_value(387), (-s.v[764]), p[438]));}
        s.b[3359] = (s.v[956] < 0.1);s.store_scalar(3359, if s.b[3359] { 1.0 } else { 0.0 });
        if (((s.b[3336] && (!s.b[3356])) && s.b[3357]) && s.b[3359]) {s.store_scalar(956, 0.1);}
        if ((s.b[3336] && (!s.b[3356])) && (!s.b[3357])) {s.store_scalar(387, (ctx_temp + p[11]));}
        if (s.b[3336] && (!s.b[3356])) {s.store_scalar(164, (s.v[630] * p[7]));s.store_scalar(785, (p[67] + p[68]));s.store_primal_offset(789, 451, 1e-12);s.store_scalar(408, s.v[459]);s.store_offset_ad(335, A::mul_sub_from_scalar_rhs(s.ad_value(3353), p[410], A::scale(s.ad_value(3353), p[411])), 1.0);s.store_sqrt_square_offset(782, 335, ((4.0 * 0.1) * 0.1));s.store_offset_scaled_div(336, 335, 782, 0.5, 0.5);s.store_scaled_add(654, 335, 782, 0.5);}
        s.b[3360] = (s.v[654] < 0.0);s.store_scalar(3360, if s.b[3360] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3356])) && s.b[3360]) {s.store_scalar(654, 0.0);s.store_scalar(336, 0.0);}
        if (s.b[3336] && (!s.b[3356])) {s.store_mul3_lhs(593, 787, 653, 654);s.store_offset_product3(3355, s.ad_value(788), s.ad_value(786), s.ad_value(652), 1.0, 1e-25);s.copy_ad(594, 453);s.store_scalar(595, p[421]);s.store_scale(335, 593, 10000.0);s.store_scale(336, 3355, 100.0);}
        s.b[3363] = (s.v[799] < 0.0);s.store_scalar(3363, if s.b[3363] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3356])) && s.b[3363]) {s.store_scale(781, 799, ((-0.5) * (2.0 * 1.0 / (p[262]))));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(108, p[262], 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);}
        s.b[3364] = (s.v[108] < 1e-12);s.store_scalar(3364, if s.b[3364] { 1.0 } else { 0.0 });
        if (((s.b[3336] && (!s.b[3356])) && s.b[3363]) && s.b[3364]) {s.store_scalar(108, 1e-12);}
        if ((s.b[3336] && (!s.b[3356])) && s.b[3363]) {s.store_sub_scaled_inputs(598, 799, 1.0, 108, 2.0);}
        if ((s.b[3336] && (!s.b[3356])) && (!s.b[3363])) {s.store_scale(781, 799, (0.5 * (2.0 * 1.0 / (p[262]))));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_205(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[3336] && (!s.b[3356])) && (!s.b[3363])) {s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(108, p[262], 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);}
        s.b[3365] = (s.v[108] < 1e-12);s.store_scalar(3365, if s.b[3365] { 1.0 } else { 0.0 });
        if (((s.b[3336] && (!s.b[3356])) && (!s.b[3363])) && s.b[3365]) {s.store_scalar(108, 1e-12);}
        if ((s.b[3336] && (!s.b[3356])) && (!s.b[3363])) {s.store_add_scaled_inputs(598, 799, 1.0, 108, 2.0);}
        if (s.b[3336] && (!s.b[3356])) {s.store_div(591, 598, 785);s.store_mul(592, 593, 591);}
        s.b[3366] = (s.v[799] >= 0.0);s.store_scalar(3366, if s.b[3366] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3356])) && s.b[3366]) {s.store_div(335, 592, 3355);}
        if ((s.b[3336] && (!s.b[3356])) && (!s.b[3366])) {s.store_div_scaled_inputs_indices(335, 592, -1.0, 3355, 1.0);}
        s.b[3367] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3367, if s.b[3367] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3356])) && s.b[3367]) {s.store_scalar(337, 1.0);}
        s.b[3368] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3368, if s.b[3368] { 1.0 } else { 0.0 });
        if (((s.b[3336] && (!s.b[3356])) && (!s.b[3367])) && s.b[3368]) {s.copy_ad(337, 335);}
        if (((s.b[3336] && (!s.b[3356])) && (!s.b[3367])) && (!s.b[3368])) {s.store_pow_offset_rhs(337, 335, 956, (-1.0));}
        if (s.b[3336] && (!s.b[3356])) {s.store_mul(336, 335, 337);s.store_offset(338, 336, 1.0);}
        s.b[3369] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3369, if s.b[3369] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3356])) && s.b[3369]) {s.store_div_from_scalar(339, 1.0, 338);}
        s.b[3370] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3370, if s.b[3370] { 1.0 } else { 0.0 });
        if (((s.b[3336] && (!s.b[3356])) && (!s.b[3369])) && s.b[3370]) {s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));}
        if (((s.b[3336] && (!s.b[3356])) && (!s.b[3369])) && (!s.b[3370])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_pow_ad(340, s.ad_value(338), A::offset(A::div_from_scalar((-1.0), s.ad_value(956)), (-1.0)));
            }
        }
        if (((s.b[3336] && (!s.b[3356])) && (!s.b[3369])) && (!s.b[3370])) {s.store_mul(339, 338, 340);}
        if (s.b[3336] && (!s.b[3356])) {s.store_mul(3354, 593, 339);s.store_offset(338, 335, 1.0);s.store_div_from_scalar(339, 1.0, 338);s.store_offset_ad(338, A::div_scaled_product_offset_denominator(A::mul_sub_from_scalar_rhs(s.ad_value(595), 1.0, s.ad_value(339)), s.ad_value(598), 1.0, s.ad_value(785), (-p[423]), 1.0), 1.0);s.store_offset(781, 338, (-0.001));s.store_scalar(782, 0.0);}
        if (s.b[3336] && (!s.b[3356])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[3336] && (!s.b[3356])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_scaled_add(339, 781, 782, 0.5);s.store_mul(717, 408, 339);s.store_scale(718, 698, (6.241449993689894e18 * p[430]));s.store_add_scaled_inputs3_indices(781, 717, 1.0, 718, (-1.0), 717, (-0.001));s.store_scaled_mul(782, 717, 717, (4.0 * 0.001));}
        if (s.b[3336] && (!s.b[3356])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[3336] && (!s.b[3356])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(718, 717, 1.0, 781, (-0.5), 782, (-0.5));s.store_sub(597, 717, 718);}
        s.b[3371] = ((p[441] > 0.0) && (p[440] > 1.0));s.store_scalar(3371, if s.b[3371] { 1.0 } else { 0.0 });s.b[3372] = ((s.v[597] > ((s.v[408] * p[440]) - (s.v[408] * p[441]))) && ((s.v[408] * p[441]) >= 0.0));s.store_scalar(3372, if s.b[3372] { 1.0 } else { 0.0 });
        if (((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) {s.store_add_scaled_inputs3_indices(781, 597, 1.0, 408, (-p[440]), 408, p[441]);s.store_square(722, 781);s.store_scaled_mul(723, 408, 408, (p[441] * p[441]));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_scalar(719, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_206(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t5: usize = 0;
        while {
            let t4: f64 = if ((((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) && (s.v[719] < p[442])) { 1.0 } else { 0.0 };
            t4 != 0.0
        } {
            t5 += 1;
            if t5 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t5, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) {s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3373] = ((((p[442] == 1.0) || (p[442] == 2.0)) || (p[442] == 4.0)) || (p[442] == 8.0));s.store_scalar(3373, if s.b[3373] { 1.0 } else { 0.0 });s.b[3374] = (p[442] == 1.0);s.store_scalar(3374, if s.b[3374] { 1.0 } else { 0.0 });
        if (((((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) && s.b[3373]) && s.b[3374]) {s.store_scalar(720, 1.0);}
        s.b[3375] = (p[442] == 2.0);s.store_scalar(3375, if s.b[3375] { 1.0 } else { 0.0 });
        if ((((((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) && s.b[3373]) && (!s.b[3374])) && s.b[3375]) {s.store_scalar(720, 2.0);}
        s.b[3376] = (p[442] == 4.0);s.store_scalar(3376, if s.b[3376] { 1.0 } else { 0.0 });
        if (((((((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) && s.b[3373]) && (!s.b[3374])) && (!s.b[3375])) && s.b[3376]) {s.store_scalar(720, 3.0);}
        s.b[3377] = (p[442] == 8.0);s.store_scalar(3377, if s.b[3377] { 1.0 } else { 0.0 });
        if ((((((((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) && s.b[3373]) && (!s.b[3374])) && (!s.b[3375])) && (!s.b[3376])) && s.b[3377]) {s.store_scalar(720, 4.0);}
        if ((((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) && s.b[3373]) {s.store_scalar(719, 0.0);}
        let mut t7: usize = 0;
        while {
            let t6: f64 = if (((((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) && s.b[3373]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t6 != 0.0
        } {
            t7 += 1;
            if t7 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t7, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) && s.b[3373]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) && (!s.b[3373])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * p[442])));
            }
        }
        if (((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_affine_lhs(780, 781, 408, p[441], 0.0, 726);s.store_div_scaled_product3_indices(334, 408, 725, 726, p[441], 770, 1.0);s.store_add_scaled_inputs3_indices(336, 408, p[440], 408, (-p[441]), 780, 1.0);}
        if (((s.b[3336] && (!s.b[3356])) && s.b[3371]) && s.b[3372]) {
        }
        if (((s.b[3336] && (!s.b[3356])) && s.b[3371]) && (!s.b[3372])) {s.copy_ad(336, 597);s.store_scalar(334, 1.0);}
        if ((s.b[3336] && (!s.b[3356])) && s.b[3371]) {s.copy_ad(597, 336);}
        if (s.b[3336] && (!s.b[3356])) {s.store_neg(334, 697);s.store_sqrt_square_offset(782, 334, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(343, 334, 782, 0.5, 0.5);s.store_scaled_add(334, 334, 782, 0.5);}
        s.b[3378] = (s.v[334] < 0.0);s.store_scalar(3378, if s.b[3378] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3356])) && s.b[3378]) {s.store_scalar(334, 0.0);s.store_scalar(343, 0.0);}
        if (s.b[3336] && (!s.b[3356])) {s.store_offset(334, 334, (10.0 * 2.220446049250313e-16));s.store_sqrt_mul(599, 650, 334);s.store_offset_sub(336, 3352, 3353, p[137]);s.store_sqrt_square_offset(782, 336, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[3379] = (s.v[336] < 0.0);s.store_scalar(3379, if s.b[3379] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3356])) && s.b[3379]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (s.b[3336] && (!s.b[3356])) {s.store_offset(336, 336, (10.0 * 2.220446049250313e-16));s.store_sqrt_mul(600, 651, 336);s.store_add_scaled_inputs3_indices(781, 789, 1.0, 600, (-1.0), 789, (-0.01));s.store_scaled_mul(782, 789, 789, (4.0 * 0.01));}
        if (s.b[3336] && (!s.b[3356])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[3336] && (!s.b[3356])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(602, 789, 1.0, 781, (-0.5), 782, (-0.5));s.store_scalar(601, (p[419] + 1e-25));s.store_mul_scale_offset_mixed_ia(596, 649, A::mul(s.ad_value(594), A::add(A::div(s.ad_value(599), s.ad_value(601)), A::div(s.ad_value(602), s.ad_value(789)))), -1.0, 1.0);s.store_sqrt_ad(782, A::add_scaled_square_product(s.ad_value(596), 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(453), s.ad_value(649)), A::mul_sub_from_scalar_lhs(1.0, s.ad_value(453), s.ad_value(649)), ((1.0 / (100.0) * 4.0) * 1.0 / (100.0))));s.store_offset_scaled_div(343, 596, 782, 0.5, 0.5);s.store_scaled_add(596, 596, 782, 0.5);}
        s.b[3380] = (s.v[596] < 0.0);s.store_scalar(3380, if s.b[3380] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_207(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[3336] && (!s.b[3356])) && s.b[3380]) {s.store_scalar(596, 0.0);s.store_scalar(343, 0.0);}
        if (s.b[3336] && (!s.b[3356])) {s.store_div_from_scalar_offset_input(335, 1.6021918e-19, 785, p[422]);s.store_mul_product3_indices(739, 597, 335, 596, 3354, 1.0);}
        s.b[3381] = ((s.v[739] < 1e-25) && (1e-25 >= 0.0));s.store_scalar(3381, if s.b[3381] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3356])) && s.b[3381]) {s.store_sub_from_scalar(781, 1e-25, 739);s.store_square(722, 781);s.store_scalar(723, (1e-25 * 1e-25));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3382] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(3382, if s.b[3382] { 1.0 } else { 0.0 });s.b[3383] = (2.0 == 1.0);s.store_scalar(3383, if s.b[3383] { 1.0 } else { 0.0 });
        if ((((s.b[3336] && (!s.b[3356])) && s.b[3381]) && s.b[3382]) && s.b[3383]) {s.store_scalar(720, 1.0);}
        s.b[3384] = (2.0 == 2.0);s.store_scalar(3384, if s.b[3384] { 1.0 } else { 0.0 });
        if (((((s.b[3336] && (!s.b[3356])) && s.b[3381]) && s.b[3382]) && (!s.b[3383])) && s.b[3384]) {s.store_scalar(720, 2.0);}
        s.b[3385] = (2.0 == 4.0);s.store_scalar(3385, if s.b[3385] { 1.0 } else { 0.0 });
        if ((((((s.b[3336] && (!s.b[3356])) && s.b[3381]) && s.b[3382]) && (!s.b[3383])) && (!s.b[3384])) && s.b[3385]) {s.store_scalar(720, 3.0);}
        s.b[3386] = (2.0 == 8.0);s.store_scalar(3386, if s.b[3386] { 1.0 } else { 0.0 });
        if (((((((s.b[3336] && (!s.b[3356])) && s.b[3381]) && s.b[3382]) && (!s.b[3383])) && (!s.b[3384])) && (!s.b[3385])) && s.b[3386]) {s.store_scalar(720, 4.0);}
        if (((s.b[3336] && (!s.b[3356])) && s.b[3381]) && s.b[3382]) {s.store_scalar(719, 0.0);}
        let mut t9: usize = 0;
        while {
            let t8: f64 = if ((((s.b[3336] && (!s.b[3356])) && s.b[3381]) && s.b[3382]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t8 != 0.0
        } {
            t9 += 1;
            if t9 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t9, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[3336] && (!s.b[3356])) && s.b[3381]) && s.b[3382]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((s.b[3336] && (!s.b[3356])) && s.b[3381]) && (!s.b[3382])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[3336] && (!s.b[3356])) && s.b[3381]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-25);s.store_div_scaled_product_indices(334, 725, 726, 1e-25, 770, 1.0);s.store_sub_from_scalar(739, 1e-25, 780);}
        if ((s.b[3336] && (!s.b[3356])) && s.b[3381]) {
        }
        if ((s.b[3336] && (!s.b[3356])) && (!s.b[3381])) {
        }
        if ((s.b[3336] && (!s.b[3356])) && (!s.b[3381])) {s.store_scalar(334, 1.0);}
        if (s.b[3336] && (!s.b[3356])) {s.store_div_from_scalar(4, 1.0, 739);s.store_div(4, 4, 164);}
        s.b[3387] = ((s.v[4] > (1000000.0 - 1000.0)) && (1000.0 >= 0.0));s.store_scalar(3387, if s.b[3387] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3356])) && s.b[3387]) {s.store_offset(781, 4, (((-1000000.0)) + (1000.0)));s.store_square(722, 781);s.store_scalar(723, (1000.0 * 1000.0));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3388] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(3388, if s.b[3388] { 1.0 } else { 0.0 });s.b[3389] = (2.0 == 1.0);s.store_scalar(3389, if s.b[3389] { 1.0 } else { 0.0 });
        if ((((s.b[3336] && (!s.b[3356])) && s.b[3387]) && s.b[3388]) && s.b[3389]) {s.store_scalar(720, 1.0);}
        s.b[3390] = (2.0 == 2.0);s.store_scalar(3390, if s.b[3390] { 1.0 } else { 0.0 });
        if (((((s.b[3336] && (!s.b[3356])) && s.b[3387]) && s.b[3388]) && (!s.b[3389])) && s.b[3390]) {s.store_scalar(720, 2.0);}
        s.b[3391] = (2.0 == 4.0);s.store_scalar(3391, if s.b[3391] { 1.0 } else { 0.0 });
        if ((((((s.b[3336] && (!s.b[3356])) && s.b[3387]) && s.b[3388]) && (!s.b[3389])) && (!s.b[3390])) && s.b[3391]) {s.store_scalar(720, 3.0);}
        s.b[3392] = (2.0 == 8.0);s.store_scalar(3392, if s.b[3392] { 1.0 } else { 0.0 });
        if (((((((s.b[3336] && (!s.b[3356])) && s.b[3387]) && s.b[3388]) && (!s.b[3389])) && (!s.b[3390])) && (!s.b[3391])) && s.b[3392]) {s.store_scalar(720, 4.0);}
        if (((s.b[3336] && (!s.b[3356])) && s.b[3387]) && s.b[3388]) {s.store_scalar(719, 0.0);}
        let mut tb: usize = 0;
        while {
            let ta: f64 = if ((((s.b[3336] && (!s.b[3356])) && s.b[3387]) && s.b[3388]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            ta != 0.0
        } {
            tb += 1;
            if tb > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", tb, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[3336] && (!s.b[3356])) && s.b[3387]) && s.b[3388]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_208(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[3336] && (!s.b[3356])) && s.b[3387]) && (!s.b[3388])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[3336] && (!s.b[3356])) && s.b[3387]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1000.0);s.store_div_scaled_product_indices(334, 725, 726, 1000.0, 770, 1.0);s.store_offset(4, 780, (1000000.0 - 1000.0));}
        if ((s.b[3336] && (!s.b[3356])) && s.b[3387]) {
        }
        if ((s.b[3336] && (!s.b[3356])) && (!s.b[3387])) {
        }
        if ((s.b[3336] && (!s.b[3356])) && (!s.b[3387])) {s.store_scalar(334, 1.0);}
        s.b[3393] = ((p[54] == 1.0) && ((s.v[459] * (s.v[544] + s.v[459])) > 0.0));s.store_scalar(3393, if s.b[3393] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3356])) && s.b[3393]) {s.store_sub_from_scalar(385, p[334], 384);s.store_div_scaled_inputs_indices(4, 4, s.v[165], 385, 1.0);}
        if (s.b[3336] && (!s.b[3356])) {s.store_add(4, 4, 644);}
        s.b[3395] = (s.v[4] < p[444]);s.store_scalar(3395, if s.b[3395] { 1.0 } else { 0.0 });
        if ((s.b[3336] && (!s.b[3356])) && s.b[3395]) {s.store_scalar(4, p[444]);}
        if (s.b[3336] && (!s.b[3356])) {s.store_scale(715, 4, 1.0 / (s.v[365]));}
        s.b[3396] = (s.v[4] < p[444]);s.store_scalar(3396, if s.b[3396] { 1.0 } else { 0.0 });
        if ((!s.b[3336]) && s.b[3396]) {s.store_scalar(4, p[444]);}
        s.b[3397] = (s.v[5] < p[444]);s.store_scalar(3397, if s.b[3397] { 1.0 } else { 0.0 });
        if ((!s.b[3336]) && s.b[3397]) {s.store_scalar(5, p[444]);}
        s.b[3398] = (s.v[370] > 0.0);s.store_scalar(3398, if s.b[3398] { 1.0 } else { 0.0 });
        if ((!s.b[3336]) && s.b[3398]) {s.store_scale(715, 4, 1.0 / (s.v[365]));s.store_scale(716, 5, 1.0 / (s.v[365]));}
        if ((!s.b[3336]) && (!s.b[3398])) {s.store_scale(715, 5, 1.0 / (s.v[365]));s.store_scale(716, 4, 1.0 / (s.v[365]));}
        s.copy_ad(4, 715);s.copy_ad(5, 716);s.b[3399] = (s.v[949] > 0.0);s.store_scalar(3399, if s.b[3399] { 1.0 } else { 0.0 });
        if s.b[3399] {s.copy_ad(134, 0);s.copy_ad(19, 701);s.copy_ad(18, 700);s.copy_ad(741, 702);s.store_add_scaled_inputs3_indices(20, 700, (-1.0), 701, (-1.0), 702, (-1.0));s.copy_ad(280, 709);s.copy_ad(281, 710);s.copy_ad(400, 699);}
        if (s.b[3399] && (s.v[81] != 0.0)) {s.copy_ad(247, 708);}
        if (!s.b[3399]) {s.store_neg(134, 0);s.copy_ad(19, 702);s.copy_ad(18, 700);s.copy_ad(741, 701);s.store_add_scaled_inputs3_indices(20, 700, (-1.0), 701, (-1.0), 702, (-1.0));s.store_scalar(280, 0.0);s.store_scalar(281, 0.0);s.store_scalar(400, 0.0);}
        if ((!s.b[3399]) && (s.v[81] != 0.0)) {s.store_sub_from_scalar(247, 1.0, 708);}
        s.store_add(18, 18, 811);s.store_add(19, 19, 810);s.store_add(741, 741, 812);s.store_add_scaled_inputs3_indices(20, 18, (-1.0), 19, (-1.0), 741, (-1.0));s.copy_ad(299, 703);s.copy_ad(301, 704);s.copy_ad(742, 706);s.copy_ad(743, 705);s.store_add_scaled_inputs3_indices(744, 705, (-1.0), 706, (-1.0), 707, (-1.0));s.b[3400] = (p[53] > 0.0);s.store_scalar(3400, if s.b[3400] { 1.0 } else { 0.0 });s.b[3401] = (s.v[766] > 0.0001);s.store_scalar(3401, if s.b[3401] { 1.0 } else { 0.0 });
        if (s.b[3400] && s.b[3401]) {s.store_div_from_scalar(740, 1.0, 766);}
        if (s.b[3400] && (!s.b[3401])) {s.store_scalar(740, (1.0 / 0.0001));}
        s.b[3402] = ((s.v[729] * (s.v[733] - s.v[729])) >= 0.0);s.store_scalar(3402, if s.b[3402] { 1.0 } else { 0.0 });s.b[3403] = (s.v[529] == 1.0);s.store_scalar(3403, if s.b[3403] { 1.0 } else { 0.0 });
        if ((s.b[3400] && s.b[3402]) && s.b[3403]) {s.copy_ad(745, 733);}
        if ((s.b[3400] && s.b[3402]) && (!s.b[3403])) {s.store_add_scaled_product_right_sub(745, 729, 1.0, 683, 733, 729, 1.0);}
        if (s.b[3400] && (!s.b[3402])) {s.copy_ad(745, 729);}
        if s.b[3400] {s.store_mul(746, 134, 745);}
        s.b[3404] = (p[53] == 1.0);s.store_scalar(3404, if s.b[3404] { 1.0 } else { 0.0 });
        if (s.b[3400] && s.b[3404]) {s.store_scale(335, 740, p[433]);s.store_add_scaled_inputs3_indices(781, 335, 1.0, 746, (-1.0), 740, (-p[337]));s.store_scaled_mul(782, 335, 740, (4.0 * p[337]));}
        if (s.b[3400] && s.b[3404]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[3400] && s.b[3404]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(336, 335, 1.0, 781, (-0.5), 782, (-0.5));s.copy_ad(746, 336);}
        if (!s.b[3400]) {s.store_scalar(740, 0.0);s.store_scalar(746, 0.0);}
        if (s.v[81] != 0.0) {s.store_mul(751, 747, 247);s.store_sub_scaled_inputs(753, 747, -1.0, 748, 1.0);s.store_mul_scale_offset_indices(752, 747, 247, -1.0, 1.0);}
        if (s.v[81] == 0.0) {s.store_scalar(751, 0.0);s.store_scalar(753, 0.0);s.store_scalar(752, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_209(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scaled_mul(0, 949, 134, p[87]);s.store_scalar(22, A::ddx_projection(&s.ad_value(18), Some(6), None));s.store_scale(22, 22, p[87]);s.store_scalar(23, A::ddx_projection(&s.ad_value(18), Some(8), None));s.store_scale(23, 23, p[87]);s.b[3407] = (s.v[949] == 1.0);s.store_scalar(3407, if s.b[3407] { 1.0 } else { 0.0 });
        if s.b[3407] {s.copy_ad(757, 23);}
        if (!s.b[3407]) {s.copy_ad(757, 22);}
        s.b[3409] = (p[48] > 0.0);s.store_scalar(3409, if s.b[3409] { 1.0 } else { 0.0 });s.b[3413] = (p[53] > 0.0);s.store_scalar(3413, if s.b[3413] { 1.0 } else { 0.0 });
        if (!s.b[3413]) {s.store_scalar(767, 0.0);}
        if (p[28] != 0.0) {s.store_scalar(800, 1.0);s.store_scalar(801, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
    ) {
        let (eq0_e1025,) = {
    if s.b[1001] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq0_value: f64 = eq0_e1025;
        stamper.stamp_potential_const_local(
            0,
            eq0_value,
        );
        let (eq1_e1032, eq1_e1032_d_n0, eq1_e1032_d_n1, eq1_e1032_d_n2, eq1_e1032_d_n3, eq1_e1032_d_n4, eq1_e1032_d_n5, eq1_e1032_d_n6, eq1_e1032_d_n7, eq1_e1032_d_n8, eq1_e1032_d_n9, eq1_e1032_d_n10, eq1_e1032_d_n11, eq1_e1032_d_n12, eq1_e1032_d_n13, eq1_e1032_d_n14, eq1_e1032_d_n15, eq1_e1032_d_n16, eq1_e1032_d_n17, eq1_e1032_d_n18, eq1_e1032_d_b0, eq1_e1032_d_b1, eq1_e1032_d_b2, eq1_e1032_d_b3, eq1_e1032_d_b4, eq1_e1032_d_b5, eq1_e1032_d_b6, eq1_e1032_d_b7, eq1_e1032_d_b8, eq1_e1032_d_b9, eq1_e1032_d_b10, eq1_e1032_d_b11, eq1_e1032_d_b12,) = {
    if s.b[3309] {
        let eq1_e1029: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, s.v[924]);let eq1_e1030: f64 = (s.v[926] + eq1_e1029);let eq1_e1030_d_n0: f64 = (s.dn[926][0] + (s.dn[924][0] * ddt_scale));let eq1_e1030_d_n1: f64 = (s.dn[926][1] + (s.dn[924][1] * ddt_scale));let eq1_e1030_d_n2: f64 = (s.dn[926][2] + (s.dn[924][2] * ddt_scale));let eq1_e1030_d_n3: f64 = (s.dn[926][3] + (s.dn[924][3] * ddt_scale));let eq1_e1030_d_n4: f64 = (s.dn[926][4] + (s.dn[924][4] * ddt_scale));let eq1_e1030_d_n5: f64 = (s.dn[926][5] + (s.dn[924][5] * ddt_scale));let eq1_e1030_d_n6: f64 = (s.dn[926][6] + (s.dn[924][6] * ddt_scale));let eq1_e1030_d_n7: f64 = (s.dn[926][7] + (s.dn[924][7] * ddt_scale));let eq1_e1030_d_n8: f64 = (s.dn[926][8] + (s.dn[924][8] * ddt_scale));let eq1_e1030_d_n9: f64 = (s.dn[926][9] + (s.dn[924][9] * ddt_scale));let eq1_e1030_d_n10: f64 = (s.dn[926][10] + (s.dn[924][10] * ddt_scale));let eq1_e1030_d_n11: f64 = (s.dn[926][11] + (s.dn[924][11] * ddt_scale));let eq1_e1030_d_n12: f64 = (s.dn[926][12] + (s.dn[924][12] * ddt_scale));let eq1_e1030_d_n13: f64 = (s.dn[926][13] + (s.dn[924][13] * ddt_scale));let eq1_e1030_d_n14: f64 = (s.dn[926][14] + (s.dn[924][14] * ddt_scale));let eq1_e1030_d_n15: f64 = (s.dn[926][15] + (s.dn[924][15] * ddt_scale));let eq1_e1030_d_n16: f64 = (s.dn[926][16] + (s.dn[924][16] * ddt_scale));let eq1_e1030_d_n17: f64 = (s.dn[926][17] + (s.dn[924][17] * ddt_scale));let eq1_e1030_d_n18: f64 = (s.dn[926][18] + (s.dn[924][18] * ddt_scale));let eq1_e1030_d_b0: f64 = (s.db[926][0] + (s.db[924][0] * ddt_scale));let eq1_e1030_d_b1: f64 = (s.db[926][1] + (s.db[924][1] * ddt_scale));let eq1_e1030_d_b2: f64 = (s.db[926][2] + (s.db[924][2] * ddt_scale));let eq1_e1030_d_b3: f64 = (s.db[926][3] + (s.db[924][3] * ddt_scale));let eq1_e1030_d_b4: f64 = (s.db[926][4] + (s.db[924][4] * ddt_scale));let eq1_e1030_d_b5: f64 = (s.db[926][5] + (s.db[924][5] * ddt_scale));let eq1_e1030_d_b6: f64 = (s.db[926][6] + (s.db[924][6] * ddt_scale));let eq1_e1030_d_b7: f64 = (s.db[926][7] + (s.db[924][7] * ddt_scale));let eq1_e1030_d_b8: f64 = (s.db[926][8] + (s.db[924][8] * ddt_scale));let eq1_e1030_d_b9: f64 = (s.db[926][9] + (s.db[924][9] * ddt_scale));let eq1_e1030_d_b10: f64 = (s.db[926][10] + (s.db[924][10] * ddt_scale));let eq1_e1030_d_b11: f64 = (s.db[926][11] + (s.db[924][11] * ddt_scale));let eq1_e1030_d_b12: f64 = (s.db[926][12] + (s.db[924][12] * ddt_scale));
        (eq1_e1030, eq1_e1030_d_n0, eq1_e1030_d_n1, eq1_e1030_d_n2, eq1_e1030_d_n3, eq1_e1030_d_n4, eq1_e1030_d_n5, eq1_e1030_d_n6, eq1_e1030_d_n7, eq1_e1030_d_n8, eq1_e1030_d_n9, eq1_e1030_d_n10, eq1_e1030_d_n11, eq1_e1030_d_n12, eq1_e1030_d_n13, eq1_e1030_d_n14, eq1_e1030_d_n15, eq1_e1030_d_n16, eq1_e1030_d_n17, eq1_e1030_d_n18, eq1_e1030_d_b0, eq1_e1030_d_b1, eq1_e1030_d_b2, eq1_e1030_d_b3, eq1_e1030_d_b4, eq1_e1030_d_b5, eq1_e1030_d_b6, eq1_e1030_d_b7, eq1_e1030_d_b8, eq1_e1030_d_b9, eq1_e1030_d_b10, eq1_e1030_d_b11, eq1_e1030_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e1032;let eq1_node_derivatives: [f64; 19] = [eq1_e1032_d_n0, eq1_e1032_d_n1, eq1_e1032_d_n2, eq1_e1032_d_n3, eq1_e1032_d_n4, eq1_e1032_d_n5, eq1_e1032_d_n6, eq1_e1032_d_n7, eq1_e1032_d_n8, eq1_e1032_d_n9, eq1_e1032_d_n10, eq1_e1032_d_n11, eq1_e1032_d_n12, eq1_e1032_d_n13, eq1_e1032_d_n14, eq1_e1032_d_n15, eq1_e1032_d_n16, eq1_e1032_d_n17, eq1_e1032_d_n18];let eq1_branch_derivatives: [f64; 13] = [eq1_e1032_d_b0, eq1_e1032_d_b1, eq1_e1032_d_b2, eq1_e1032_d_b3, eq1_e1032_d_b4, eq1_e1032_d_b5, eq1_e1032_d_b6, eq1_e1032_d_b7, eq1_e1032_d_b8, eq1_e1032_d_b9, eq1_e1032_d_b10, eq1_e1032_d_b11, eq1_e1032_d_b12];
        stamper.stamp_current_dense_local(
            Some(16),
            None,
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let (eq2_e1039, eq2_e1039_d_n0, eq2_e1039_d_n1, eq2_e1039_d_n2, eq2_e1039_d_n3, eq2_e1039_d_n4, eq2_e1039_d_n5, eq2_e1039_d_n6, eq2_e1039_d_n7, eq2_e1039_d_n8, eq2_e1039_d_n9, eq2_e1039_d_n10, eq2_e1039_d_n11, eq2_e1039_d_n12, eq2_e1039_d_n13, eq2_e1039_d_n14, eq2_e1039_d_n15, eq2_e1039_d_n16, eq2_e1039_d_n17, eq2_e1039_d_n18, eq2_e1039_d_b0, eq2_e1039_d_b1, eq2_e1039_d_b2, eq2_e1039_d_b3, eq2_e1039_d_b4, eq2_e1039_d_b5, eq2_e1039_d_b6, eq2_e1039_d_b7, eq2_e1039_d_b8, eq2_e1039_d_b9, eq2_e1039_d_b10, eq2_e1039_d_b11, eq2_e1039_d_b12,) = {
    if s.b[3309] {
        let eq2_e1036: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, s.v[925]);let eq2_e1037: f64 = (s.v[927] + eq2_e1036);let eq2_e1037_d_n0: f64 = (s.dn[927][0] + (s.dn[925][0] * ddt_scale));let eq2_e1037_d_n1: f64 = (s.dn[927][1] + (s.dn[925][1] * ddt_scale));let eq2_e1037_d_n2: f64 = (s.dn[927][2] + (s.dn[925][2] * ddt_scale));let eq2_e1037_d_n3: f64 = (s.dn[927][3] + (s.dn[925][3] * ddt_scale));let eq2_e1037_d_n4: f64 = (s.dn[927][4] + (s.dn[925][4] * ddt_scale));let eq2_e1037_d_n5: f64 = (s.dn[927][5] + (s.dn[925][5] * ddt_scale));let eq2_e1037_d_n6: f64 = (s.dn[927][6] + (s.dn[925][6] * ddt_scale));let eq2_e1037_d_n7: f64 = (s.dn[927][7] + (s.dn[925][7] * ddt_scale));let eq2_e1037_d_n8: f64 = (s.dn[927][8] + (s.dn[925][8] * ddt_scale));let eq2_e1037_d_n9: f64 = (s.dn[927][9] + (s.dn[925][9] * ddt_scale));let eq2_e1037_d_n10: f64 = (s.dn[927][10] + (s.dn[925][10] * ddt_scale));let eq2_e1037_d_n11: f64 = (s.dn[927][11] + (s.dn[925][11] * ddt_scale));let eq2_e1037_d_n12: f64 = (s.dn[927][12] + (s.dn[925][12] * ddt_scale));let eq2_e1037_d_n13: f64 = (s.dn[927][13] + (s.dn[925][13] * ddt_scale));let eq2_e1037_d_n14: f64 = (s.dn[927][14] + (s.dn[925][14] * ddt_scale));let eq2_e1037_d_n15: f64 = (s.dn[927][15] + (s.dn[925][15] * ddt_scale));let eq2_e1037_d_n16: f64 = (s.dn[927][16] + (s.dn[925][16] * ddt_scale));let eq2_e1037_d_n17: f64 = (s.dn[927][17] + (s.dn[925][17] * ddt_scale));let eq2_e1037_d_n18: f64 = (s.dn[927][18] + (s.dn[925][18] * ddt_scale));let eq2_e1037_d_b0: f64 = (s.db[927][0] + (s.db[925][0] * ddt_scale));let eq2_e1037_d_b1: f64 = (s.db[927][1] + (s.db[925][1] * ddt_scale));let eq2_e1037_d_b2: f64 = (s.db[927][2] + (s.db[925][2] * ddt_scale));let eq2_e1037_d_b3: f64 = (s.db[927][3] + (s.db[925][3] * ddt_scale));let eq2_e1037_d_b4: f64 = (s.db[927][4] + (s.db[925][4] * ddt_scale));let eq2_e1037_d_b5: f64 = (s.db[927][5] + (s.db[925][5] * ddt_scale));let eq2_e1037_d_b6: f64 = (s.db[927][6] + (s.db[925][6] * ddt_scale));let eq2_e1037_d_b7: f64 = (s.db[927][7] + (s.db[925][7] * ddt_scale));let eq2_e1037_d_b8: f64 = (s.db[927][8] + (s.db[925][8] * ddt_scale));let eq2_e1037_d_b9: f64 = (s.db[927][9] + (s.db[925][9] * ddt_scale));let eq2_e1037_d_b10: f64 = (s.db[927][10] + (s.db[925][10] * ddt_scale));let eq2_e1037_d_b11: f64 = (s.db[927][11] + (s.db[925][11] * ddt_scale));let eq2_e1037_d_b12: f64 = (s.db[927][12] + (s.db[925][12] * ddt_scale));
        (eq2_e1037, eq2_e1037_d_n0, eq2_e1037_d_n1, eq2_e1037_d_n2, eq2_e1037_d_n3, eq2_e1037_d_n4, eq2_e1037_d_n5, eq2_e1037_d_n6, eq2_e1037_d_n7, eq2_e1037_d_n8, eq2_e1037_d_n9, eq2_e1037_d_n10, eq2_e1037_d_n11, eq2_e1037_d_n12, eq2_e1037_d_n13, eq2_e1037_d_n14, eq2_e1037_d_n15, eq2_e1037_d_n16, eq2_e1037_d_n17, eq2_e1037_d_n18, eq2_e1037_d_b0, eq2_e1037_d_b1, eq2_e1037_d_b2, eq2_e1037_d_b3, eq2_e1037_d_b4, eq2_e1037_d_b5, eq2_e1037_d_b6, eq2_e1037_d_b7, eq2_e1037_d_b8, eq2_e1037_d_b9, eq2_e1037_d_b10, eq2_e1037_d_b11, eq2_e1037_d_b12,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e1039;let eq2_node_derivatives: [f64; 19] = [eq2_e1039_d_n0, eq2_e1039_d_n1, eq2_e1039_d_n2, eq2_e1039_d_n3, eq2_e1039_d_n4, eq2_e1039_d_n5, eq2_e1039_d_n6, eq2_e1039_d_n7, eq2_e1039_d_n8, eq2_e1039_d_n9, eq2_e1039_d_n10, eq2_e1039_d_n11, eq2_e1039_d_n12, eq2_e1039_d_n13, eq2_e1039_d_n14, eq2_e1039_d_n15, eq2_e1039_d_n16, eq2_e1039_d_n17, eq2_e1039_d_n18];let eq2_branch_derivatives: [f64; 13] = [eq2_e1039_d_b0, eq2_e1039_d_b1, eq2_e1039_d_b2, eq2_e1039_d_b3, eq2_e1039_d_b4, eq2_e1039_d_b5, eq2_e1039_d_b6, eq2_e1039_d_b7, eq2_e1039_d_b8, eq2_e1039_d_b9, eq2_e1039_d_b10, eq2_e1039_d_b11, eq2_e1039_d_b12];
        stamper.stamp_current_dense_local(
            Some(17),
            None,
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let (eq3_e1044,) = {
    if (!s.b[3309]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq3_value: f64 = eq3_e1044;
        stamper.stamp_potential_const_local(
            1,
            eq3_value,
        );
        let (eq4_e1049,) = {
    if (!s.b[3309]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq4_value: f64 = eq4_e1049;
        stamper.stamp_potential_const_local(
            2,
            eq4_value,
        );
    }
}

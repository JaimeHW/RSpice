#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_224(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[3256] && s.b[3257]) && (!s.b[3258])) {s.store_add_ad(889, A::add_scaled_products3(s.ad_value(877), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(338), A::sub(s.ad_value(868), s.ad_value(848)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[821]));}
        if (s.b[3256] && (!s.b[3257])) {s.store_scalar(889, 0.0);}
        if s.b[3256] {s.store_scale(346, 878, p[514]);s.store_add_scaled_product_indices(889, 889, 1.0, 346, 868, 1.0);}
        if (!s.b[3256]) {s.store_scalar(889, 0.0);}
        s.store_scale(344, 855, p[534]);s.store_scale(343, 854, p[533]);s.b[3260] = (s.v[879] > 0.0);s.store_scalar(3260, if s.b[3260] { 1.0 } else { 0.0 });
        if s.b[3260] {s.store_mul(334, 880, 343);s.store_mul_scale_offset_indices(332, 344, 859, -1.0, 0.0);s.store_exp(336, 332);s.copy_ad(337, 336);}
        s.b[3261] = (s.v[859] < s.v[853]);s.store_scalar(3261, if s.b[3261] { 1.0 } else { 0.0 });
        if (s.b[3260] && s.b[3261]) {s.store_mul(332, 859, 855);}
        s.b[3262] = (s.v[332] < ((-3.0) * 34.0));s.store_scalar(3262, if s.b[3262] { 1.0 } else { 0.0 });
        if ((s.b[3260] && s.b[3261]) && s.b[3262]) {s.store_scalar(335, 0.0);}
        if ((s.b[3260] && s.b[3261]) && (!s.b[3262])) {s.store_exp(335, 332);}
        if (s.b[3260] && s.b[3261]) {s.store_add_ad(886, A::add_scaled_products(s.ad_value(879), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[826]));}
        if (s.b[3260] && (!s.b[3261])) {s.copy_ad(335, 856);s.store_mul3_lhs(338, 879, 855, 335);s.store_add_ad(886, A::add_scaled_products3(s.ad_value(879), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(338), A::sub(s.ad_value(859), s.ad_value(853)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[826]));}
        if (!s.b[3260]) {s.store_scalar(886, 0.0);}
        s.store_scale(346, 880, p[537]);s.store_add_scaled_product_indices(886, 886, 1.0, 346, 859, 1.0);s.b[3263] = (s.v[881] > 0.0);s.store_scalar(3263, if s.b[3263] { 1.0 } else { 0.0 });
        if s.b[3263] {s.store_mul(334, 882, 343);s.store_mul_scale_offset_indices(332, 344, 859, -1.0, 0.0);s.store_exp(336, 332);s.copy_ad(337, 336);}
        s.b[3264] = (s.v[859] < s.v[853]);s.store_scalar(3264, if s.b[3264] { 1.0 } else { 0.0 });
        if (s.b[3263] && s.b[3264]) {s.store_mul(332, 859, 855);}
        s.b[3265] = (s.v[332] < ((-3.0) * 34.0));s.store_scalar(3265, if s.b[3265] { 1.0 } else { 0.0 });
        if ((s.b[3263] && s.b[3264]) && s.b[3265]) {s.store_scalar(335, 0.0);}
        if ((s.b[3263] && s.b[3264]) && (!s.b[3265])) {s.store_exp(335, 332);}
        if (s.b[3263] && s.b[3264]) {s.store_add_ad(888, A::add_scaled_products(s.ad_value(881), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[826]));}
        if (s.b[3263] && (!s.b[3264])) {s.copy_ad(335, 856);s.store_mul3_lhs(338, 881, 855, 335);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_225(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[3263] && (!s.b[3264])) {s.store_add_ad(888, A::add_scaled_products3(s.ad_value(881), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(338), A::sub(s.ad_value(859), s.ad_value(853)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[826]));}
        if (!s.b[3263]) {s.store_scalar(888, 0.0);}
        s.store_scale(346, 882, p[537]);s.store_add_scaled_product_indices(888, 888, 1.0, 346, 859, 1.0);s.b[3266] = (p[48] > 0.0);s.store_scalar(3266, if s.b[3266] { 1.0 } else { 0.0 });s.b[3267] = (s.v[883] > 0.0);s.store_scalar(3267, if s.b[3267] { 1.0 } else { 0.0 });
        if (s.b[3266] && s.b[3267]) {s.store_mul(334, 884, 343);s.store_mul_scale_offset_indices(332, 344, 867, -1.0, 0.0);s.store_exp(336, 332);s.copy_ad(337, 336);}
        s.b[3268] = (s.v[867] < s.v[853]);s.store_scalar(3268, if s.b[3268] { 1.0 } else { 0.0 });
        if ((s.b[3266] && s.b[3267]) && s.b[3268]) {s.store_mul(332, 867, 855);}
        s.b[3269] = (s.v[332] < ((-3.0) * 34.0));s.store_scalar(3269, if s.b[3269] { 1.0 } else { 0.0 });
        if (((s.b[3266] && s.b[3267]) && s.b[3268]) && s.b[3269]) {s.store_scalar(335, 0.0);}
        if (((s.b[3266] && s.b[3267]) && s.b[3268]) && (!s.b[3269])) {s.store_exp(335, 332);}
        if ((s.b[3266] && s.b[3267]) && s.b[3268]) {s.store_add_ad(890, A::add_scaled_products(s.ad_value(883), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[826]));}
        if ((s.b[3266] && s.b[3267]) && (!s.b[3268])) {s.copy_ad(335, 856);s.store_mul3_lhs(338, 883, 855, 335);s.store_add_ad(890, A::add_scaled_products3(s.ad_value(883), A::offset(s.ad_value(335), (-1.0)), 1.0, s.ad_value(338), A::sub(s.ad_value(867), s.ad_value(853)), 1.0, s.ad_value(334), A::offset(s.ad_value(336), (-1.0)), 1.0), A::scaled_offset(s.ad_value(337), (-1.0), s.v[826]));}
        if (s.b[3266] && (!s.b[3267])) {s.store_scalar(890, 0.0);}
        if s.b[3266] {s.store_scale(346, 884, p[537]);s.store_add_scaled_product_indices(890, 890, 1.0, 346, 867, 1.0);}
        if (!s.b[3266]) {s.store_scalar(890, 0.0);}
        s.b[3270] = (s.v[832] > 0.0);s.store_scalar(3270, if s.b[3270] { 1.0 } else { 0.0 });s.b[3271] = (s.v[860] < 0.0);s.store_scalar(3271, if s.b[3271] { 1.0 } else { 0.0 });
        if (s.b[3270] && s.b[3271]) {s.store_sub_from_scalar_div_indices(770, 1.0, 860, 841);}
        s.b[3272] = (p[503] == 0.5);s.store_scalar(3272, if s.b[3272] { 1.0 } else { 0.0 });
        if ((s.b[3270] && s.b[3271]) && s.b[3272]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
        if ((s.b[3270] && s.b[3271]) && (!s.b[3272])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p[503]));
            }
        }
        if (s.b[3270] && s.b[3271]) {s.store_mul_ad_affine_product_rhs(891, 841, s.ad_value(832), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p[503])), 0.0);}
        if (s.b[3270] && (!s.b[3271])) {s.copy_ad(335, 832);s.store_div_scaled_inputs_indices(336, 832, p[503], 841, 1.0);s.store_mul_add_scaled_product_rhs_indices(891, 860, 335, 1.0, 860, 336, 0.5);}
        if (!s.b[3270]) {s.store_scalar(891, 0.0);}
        s.b[3273] = (s.v[833] > 0.0);s.store_scalar(3273, if s.b[3273] { 1.0 } else { 0.0 });s.b[3274] = (s.v[860] < 0.0);s.store_scalar(3274, if s.b[3274] { 1.0 } else { 0.0 });
        if (s.b[3273] && s.b[3274]) {s.store_sub_from_scalar_div_indices(770, 1.0, 860, 842);}
        s.b[3275] = (p[504] == 0.5);s.store_scalar(3275, if s.b[3275] { 1.0 } else { 0.0 });
        if ((s.b[3273] && s.b[3274]) && s.b[3275]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_226(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[3273] && s.b[3274]) && (!s.b[3275])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p[504]));
            }
        }
        if (s.b[3273] && s.b[3274]) {s.store_mul_ad_affine_product_rhs(893, 842, s.ad_value(833), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p[504])), 0.0);}
        if (s.b[3273] && (!s.b[3274])) {s.copy_ad(335, 833);s.store_div_scaled_inputs_indices(336, 833, p[504], 842, 1.0);s.store_mul_add_scaled_product_rhs_indices(893, 860, 335, 1.0, 860, 336, 0.5);}
        if (!s.b[3273]) {s.store_scalar(893, 0.0);}
        s.b[3276] = (p[48] > 0.0);s.store_scalar(3276, if s.b[3276] { 1.0 } else { 0.0 });s.b[3277] = (s.v[834] > 0.0);s.store_scalar(3277, if s.b[3277] { 1.0 } else { 0.0 });s.b[3278] = (s.v[868] < 0.0);s.store_scalar(3278, if s.b[3278] { 1.0 } else { 0.0 });
        if ((s.b[3276] && s.b[3277]) && s.b[3278]) {s.store_sub_from_scalar_div_indices(770, 1.0, 868, 843);}
        s.b[3279] = (p[505] == 0.5);s.store_scalar(3279, if s.b[3279] { 1.0 } else { 0.0 });
        if (((s.b[3276] && s.b[3277]) && s.b[3278]) && s.b[3279]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
        if (((s.b[3276] && s.b[3277]) && s.b[3278]) && (!s.b[3279])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p[505]));
            }
        }
        if ((s.b[3276] && s.b[3277]) && s.b[3278]) {s.store_mul_ad_affine_product_rhs(895, 843, s.ad_value(834), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p[505])), 0.0);}
        if ((s.b[3276] && s.b[3277]) && (!s.b[3278])) {s.copy_ad(335, 834);s.store_div_scaled_inputs_indices(336, 834, p[505], 843, 1.0);s.store_mul_add_scaled_product_rhs_indices(895, 868, 335, 1.0, 868, 336, 0.5);}
        if (s.b[3276] && (!s.b[3277])) {s.store_scalar(895, 0.0);}
        s.b[3280] = (s.v[834] > 0.0);s.store_scalar(3280, if s.b[3280] { 1.0 } else { 0.0 });s.b[3281] = (s.v[860] < 0.0);s.store_scalar(3281, if s.b[3281] { 1.0 } else { 0.0 });
        if (((!s.b[3276]) && s.b[3280]) && s.b[3281]) {s.store_sub_from_scalar_div_indices(770, 1.0, 860, 843);}
        s.b[3282] = (p[505] == 0.5);s.store_scalar(3282, if s.b[3282] { 1.0 } else { 0.0 });
        if ((((!s.b[3276]) && s.b[3280]) && s.b[3281]) && s.b[3282]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
        if ((((!s.b[3276]) && s.b[3280]) && s.b[3281]) && (!s.b[3282])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p[505]));
            }
        }
        if (((!s.b[3276]) && s.b[3280]) && s.b[3281]) {s.store_mul_ad_affine_product_rhs(895, 843, s.ad_value(834), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p[505])), 0.0);}
        if (((!s.b[3276]) && s.b[3280]) && (!s.b[3281])) {s.copy_ad(335, 834);s.store_div_scaled_inputs_indices(336, 834, p[505], 843, 1.0);s.store_mul_add_scaled_product_rhs_indices(895, 860, 335, 1.0, 860, 336, 0.5);}
        if ((!s.b[3276]) && (!s.b[3280])) {s.store_scalar(895, 0.0);}
        s.b[3283] = (s.v[835] > 0.0);s.store_scalar(3283, if s.b[3283] { 1.0 } else { 0.0 });s.b[3284] = (s.v[859] < 0.0);s.store_scalar(3284, if s.b[3284] { 1.0 } else { 0.0 });
        if (s.b[3283] && s.b[3284]) {s.store_sub_from_scalar_div_indices(770, 1.0, 859, 844);}
        s.b[3285] = (p[526] == 0.5);s.store_scalar(3285, if s.b[3285] { 1.0 } else { 0.0 });
        if ((s.b[3283] && s.b[3284]) && s.b[3285]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
        if ((s.b[3283] && s.b[3284]) && (!s.b[3285])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p[526]));
            }
        }
        if (s.b[3283] && s.b[3284]) {s.store_mul_ad_affine_product_rhs(892, 844, s.ad_value(835), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p[526])), 0.0);}
        if (s.b[3283] && (!s.b[3284])) {s.copy_ad(335, 835);s.store_div_scaled_inputs_indices(336, 835, p[526], 844, 1.0);s.store_mul_add_scaled_product_rhs_indices(892, 859, 335, 1.0, 859, 336, 0.5);}
        if (!s.b[3283]) {s.store_scalar(892, 0.0);}
        s.b[3286] = (s.v[838] > 0.0);s.store_scalar(3286, if s.b[3286] { 1.0 } else { 0.0 });s.b[3287] = (s.v[859] < 0.0);s.store_scalar(3287, if s.b[3287] { 1.0 } else { 0.0 });
        if (s.b[3286] && s.b[3287]) {s.store_sub_from_scalar_div_indices(770, 1.0, 859, 845);}
        s.b[3288] = (p[527] == 0.5);s.store_scalar(3288, if s.b[3288] { 1.0 } else { 0.0 });
        if ((s.b[3286] && s.b[3287]) && s.b[3288]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_227(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[3286] && s.b[3287]) && (!s.b[3288])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p[527]));
            }
        }
        if (s.b[3286] && s.b[3287]) {s.store_mul_ad_affine_product_rhs(894, 845, s.ad_value(838), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p[527])), 0.0);}
        if (s.b[3286] && (!s.b[3287])) {s.copy_ad(335, 838);s.store_div_scaled_inputs_indices(336, 838, p[527], 845, 1.0);s.store_mul_add_scaled_product_rhs_indices(894, 859, 335, 1.0, 859, 336, 0.5);}
        if (!s.b[3286]) {s.store_scalar(894, 0.0);}
        s.b[3289] = (p[48] > 0.0);s.store_scalar(3289, if s.b[3289] { 1.0 } else { 0.0 });s.b[3290] = (s.v[839] > 0.0);s.store_scalar(3290, if s.b[3290] { 1.0 } else { 0.0 });s.b[3291] = (s.v[867] < 0.0);s.store_scalar(3291, if s.b[3291] { 1.0 } else { 0.0 });
        if ((s.b[3289] && s.b[3290]) && s.b[3291]) {s.store_sub_from_scalar_div_indices(770, 1.0, 867, 846);}
        s.b[3292] = (p[528] == 0.5);s.store_scalar(3292, if s.b[3292] { 1.0 } else { 0.0 });
        if (((s.b[3289] && s.b[3290]) && s.b[3291]) && s.b[3292]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
        if (((s.b[3289] && s.b[3290]) && s.b[3291]) && (!s.b[3292])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p[528]));
            }
        }
        if ((s.b[3289] && s.b[3290]) && s.b[3291]) {s.store_mul_ad_affine_product_rhs(896, 846, s.ad_value(839), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p[528])), 0.0);}
        if ((s.b[3289] && s.b[3290]) && (!s.b[3291])) {s.copy_ad(335, 839);s.store_div_scaled_inputs_indices(336, 839, p[528], 846, 1.0);s.store_mul_add_scaled_product_rhs_indices(896, 867, 335, 1.0, 867, 336, 0.5);}
        if (s.b[3289] && (!s.b[3290])) {s.store_scalar(896, 0.0);}
        s.b[3293] = (s.v[839] > 0.0);s.store_scalar(3293, if s.b[3293] { 1.0 } else { 0.0 });s.b[3294] = (s.v[859] < 0.0);s.store_scalar(3294, if s.b[3294] { 1.0 } else { 0.0 });
        if (((!s.b[3289]) && s.b[3293]) && s.b[3294]) {s.store_sub_from_scalar_div_indices(770, 1.0, 859, 846);}
        s.b[3295] = (p[528] == 0.5);s.store_scalar(3295, if s.b[3295] { 1.0 } else { 0.0 });
        if ((((!s.b[3289]) && s.b[3293]) && s.b[3294]) && s.b[3295]) {s.store_div_from_scalar_sqrt_ad(840, 1.0, s.ad_value(770));}
        if ((((!s.b[3289]) && s.b[3293]) && s.b[3294]) && (!s.b[3295])) {
            if (s.v[770] == 0.0) {
                s.store_scalar(840, 0.0);
            } else {
                s.store_powf(840, 770, (-p[528]));
            }
        }
        if (((!s.b[3289]) && s.b[3293]) && s.b[3294]) {s.store_mul_ad_affine_product_rhs(896, 846, s.ad_value(839), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840))), 1.0 / ((1.0 - p[528])), 0.0);}
        if (((!s.b[3289]) && s.b[3293]) && (!s.b[3294])) {s.copy_ad(335, 839);s.store_div_scaled_inputs_indices(336, 839, p[528], 846, 1.0);s.store_mul_add_scaled_product_rhs_indices(896, 859, 335, 1.0, 859, 336, 0.5);}
        if ((!s.b[3289]) && (!s.b[3293])) {s.store_scalar(896, 0.0);}
        s.store_scaled_add(862, 886, 888, s.v[365]);s.store_scaled_add(861, 885, 887, s.v[365]);s.b[3296] = (p[48] > 0.0);s.store_scalar(3296, if s.b[3296] { 1.0 } else { 0.0 });
        if s.b[3296] {s.store_scale(870, 890, s.v[365]);s.store_scale(869, 889, s.v[365]);s.store_scaled_add(66, 892, 894, s.v[365]);s.store_scaled_add(65, 891, 893, s.v[365]);s.store_scale(68, 896, s.v[365]);s.store_scale(67, 895, s.v[365]);}
        if (!s.b[3296]) {s.store_scalar(870, 0.0);s.store_scalar(869, 0.0);s.store_add_scaled_inputs3_indices(66, 892, s.v[365], 894, s.v[365], 896, s.v[365]);s.store_add_scaled_inputs3_indices(65, 891, s.v[365], 893, s.v[365], 895, s.v[365]);s.store_scalar(68, 0.0);s.store_scalar(67, 0.0);}
        s.store_scalar(903, (p[540] / 1e-6));s.store_scalar(906, s.v[820]);s.store_scalar(904, (1450.0 / 10000.0));s.store_scalar(905, (500.0 / 10000.0));s.store_scalar(943, 0.001);s.store_scale_ad(908, A::exp_scaled_input(A::add_scaled_inputs(A::add_scaled_product(s.ad_value(678), s.v[616], s.ad_value(393), s.ad_value(154), (-1.0)), 1.0, s.ad_value(590), p[499]), 1.0 / (s.v[820])), 1.45e16);s.store_scaled_square(907, 908, 1.0 / (s.v[903]));s.store_powf(335, 676, (-1.5));s.store_scaled_mul(909, 335, 155, s.v[904]);s.store_scaled_mul(910, 335, 155, s.v[905]);s.store_div_scaled_product_add_scaled_denominator_indices(911, 909, 910, 2.0, 909, 1.0, 910, 1.0, 1.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_228(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.store_powf(336, 676, p[547]);s.store_scale(913, 336, p[544]);s.store_sqrt_mul(912, 913, 911);s.store_mul_scaled_ln_ad_rhs(934, 155, s.v[906], A::div_from_scalar(s.v[903], s.ad_value(907)));s.store_mul_add_scaled_inputs_rhs(935, 155, A::ln(A::div_from_scalar(s.v[903], s.ad_value(907))), s.v[906], A::div_from_scalar(p[545], s.ad_value(912)), s.v[906]);s.b[3297] = (p[539] > 0.0);s.store_scalar(3297, if s.b[3297] { 1.0 } else { 0.0 });
        if s.b[3297] {s.store_scalar(936, s.v[820]);s.store_exp_mul(937, 860, 850);}
        s.b[3298] = ((s.v[860] - (s.v[935] - s.v[934])) > 0.0);s.store_scalar(3298, if s.b[3298] { 1.0 } else { 0.0 });
        if (s.b[3297] && s.b[3298]) {s.store_exp_ad(938, A::mul(s.ad_value(154), A::sub(A::div(s.ad_value(860), s.ad_value(936)), A::div_scaled_inputs2(s.ad_value(935), 1.0, s.ad_value(934), (-1.0), s.ad_value(936), 1.0))));}
        if (s.b[3297] && (!s.b[3298])) {s.store_scalar(938, 1.0);}
        s.b[3299] = ((p[542] == 0.0) || (s.v[860] < s.v[934]));s.store_scalar(3299, if s.b[3299] { 1.0 } else { 0.0 });
        if (s.b[3297] && s.b[3299]) {s.store_scale(941, 937, p[541]);}
        if (s.b[3297] && (!s.b[3299])) {s.store_mul_scaled_exp_ad_rhs(941, 937, p[541], A::mul3_scaled_output(A::sub(s.ad_value(860), s.ad_value(934)), A::sub(s.ad_value(860), s.ad_value(934)), A::exp_scaled_input(A::ln(A::div_from_scalar(1.0, s.ad_value(676))), p[548]), (-p[542])));}
        if s.b[3297] {
            if (s.v[941] > 1e20) {
                s.store_scalar(941, 1e20);
            } else {
            }
        }
        if s.b[3297] {s.store_mul(939, 907, 941);s.store_scaled_sub(920, 939, 907, (1.6021918e-19 * p[13]));}
        s.b[3300] = (p[543] > 0.0);s.store_scalar(3300, if s.b[3300] { 1.0 } else { 0.0 });
        if (s.b[3297] && s.b[3300]) {s.store_scale(922, 920, p[543]);s.store_scaled_voltage(924, ctx, nodes, Some(15), None, p[543]);s.store_scaled_sub(926, 924, 922, 1.0 / (p[543]));s.store_scale(928, 924, 1.0 / (p[543]));}
        if (s.b[3297] && (!s.b[3300])) {s.copy_ad(922, 920);s.copy_ad(928, 922);}
        s.b[3301] = ((p[542] == 0.0) || (s.v[860] < s.v[935]));s.store_scalar(3301, if s.b[3301] { 1.0 } else { 0.0 });
        if (s.b[3297] && s.b[3301]) {s.store_scale(942, 938, p[541]);}
        if (s.b[3297] && (!s.b[3301])) {s.store_mul_scaled_exp_ad_rhs(942, 938, p[541], A::mul3_scaled_output(A::sub(s.ad_value(860), s.ad_value(935)), A::sub(s.ad_value(860), s.ad_value(935)), A::exp_scaled_input(A::ln(A::div_from_scalar(1.0, s.ad_value(676))), p[548]), (-p[542])));}
        if s.b[3297] {
            if (s.v[942] > 1e20) {
                s.store_scalar(942, 1e20);
            } else {
            }
        }
        if s.b[3297] {s.store_mul(940, 907, 942);s.store_scaled_sub(921, 940, 907, (1.6021918e-19 * p[13]));}
        s.b[3302] = (p[543] > 0.0);s.store_scalar(3302, if s.b[3302] { 1.0 } else { 0.0 });
        if (s.b[3297] && s.b[3302]) {s.store_scale(923, 921, p[543]);s.store_scaled_voltage(925, ctx, nodes, Some(16), None, p[543]);s.store_scaled_sub(927, 925, 923, 1.0 / (p[543]));s.store_scale(929, 925, 1.0 / (p[543]));}
        if (s.b[3297] && (!s.b[3302])) {s.copy_ad(923, 921);s.copy_ad(929, 923);}
        if s.b[3297] {s.store_sub_from_scalar(914, p[506], 860);s.store_sqrt_square_offset(782, 914, ((4.0 * s.v[943]) * s.v[943]));s.store_offset_scaled_div(334, 914, 782, 0.5, 0.5);s.store_scaled_add(914, 914, 782, 0.5);}
        s.b[3303] = (s.v[914] < 0.0);s.store_scalar(3303, if s.b[3303] { 1.0 } else { 0.0 });
        if (s.b[3297] && s.b[3303]) {s.store_scalar(914, 0.0);s.store_scalar(334, 0.0);}
        if s.b[3297] {s.store_sqrt_scaled_input(915, 914, ((2.0 * 1.034943e-10) * 1.0 / ((1.6021918e-19 * s.v[903]))));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_229(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[3297] {s.store_offset_sub_from_scalar_ad(781, p[545], s.ad_value(915), (-1e-7));s.store_scalar(782, ((4.0 * p[545]) * 1e-7));}
        if s.b[3297] {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if s.b[3297] {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(915, 781, (-0.5), 782, (-0.5), p[545]);}
        s.b[3304] = (p[546] > 0.0);s.store_scalar(3304, if s.b[3304] { 1.0 } else { 0.0 });
        if (s.b[3297] && s.b[3304]) {s.store_scale(930, 915, p[546]);s.store_scaled_voltage(931, ctx, nodes, Some(17), None, p[546]);s.store_scaled_sub(932, 931, 930, 1.0 / (p[546]));s.store_scale(933, 931, 1.0 / (p[546]));}
        if (s.b[3297] && (!s.b[3304])) {s.copy_ad(930, 915);s.copy_ad(933, 930);}
        if s.b[3297] {s.store_scalar(916, ((-((s.v[903] * p[13]) * 1.6021918e-19)) * p[545]));s.store_mul_ad_product_rhs_mixed_ia(917, 912, 928, A::sub(A::exp(A::div_from_scalar((-p[545]), s.ad_value(912))), A::exp_div_scaled_inputs(s.ad_value(933), -1.0, s.ad_value(912), 1.0)));s.store_mul_ad_product_rhs_mixed_ia(918, 912, 929, A::offset(A::exp_div_scaled_inputs(A::sub_from_scalar(p[545], s.ad_value(933)), -1.0, s.ad_value(912), 1.0), (-1.0)));s.store_add_scaled_inputs3_indices(919, 916, (-1.0), 917, (-1.0), 918, (-1.0));s.store_add_scaled_inputs(65, 65, 1.0, 919, s.v[365]);}
        s.b[3305] = ((p[539] > 0.0) && (p[543] > 0.0));s.store_scalar(3305, if s.b[3305] { 1.0 } else { 0.0 });s.b[3306] = ((p[539] > 0.0) && (p[546] > 0.0));s.store_scalar(3306, if s.b[3306] { 1.0 } else { 0.0 });s.b[3307] = (p[46] == 1.0);s.store_scalar(3307, if s.b[3307] { 1.0 } else { 0.0 });s.b[3308] = ((s.v[486] > 0.0) && (s.v[454] > 0.0));s.store_scalar(3308, if s.b[3308] { 1.0 } else { 0.0 });
        if (s.b[3307] && s.b[3308]) {s.store_mul(335, 665, 85);s.store_scale(337, 636, 1.0 / ((s.v[188] * s.v[188])));s.store_scale_ad(338, A::div_from_scalar(2.0, s.ad_value(636)), (s.v[188] * s.v[188]));s.store_add_scaled_inputs_product_indices(339, 335, 1.0, 155, (-1.0), 666, 1434, (-1.0));s.store_offset_mul(340, 338, 339, 1.0);s.store_scaled_offset(341, 338, 1.0, 2.0);}
        s.b[3309] = ((s.v[340] < s.v[341]) && (s.v[341] >= 0.0));s.store_scalar(3309, if s.b[3309] { 1.0 } else { 0.0 });
        if ((s.b[3307] && s.b[3308]) && s.b[3309]) {s.store_sub(781, 341, 340);s.store_square(722, 781);s.store_square(723, 341);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t0,) = {
    if ((s.b[3307] && s.b[3308]) && s.b[3309]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t0);
        let (t1,) = {
    if ((s.b[3307] && s.b[3308]) && s.b[3309]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t1);
        if ((s.b[3307] && s.b[3308]) && s.b[3309]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3310] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(3310, if s.b[3310] { 1.0 } else { 0.0 });s.b[3311] = (4.0 == 1.0);s.store_scalar(3311, if s.b[3311] { 1.0 } else { 0.0 });
        let (t2,) = {
    if ((((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) && s.b[3311]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t2);s.b[3312] = (4.0 == 2.0);s.store_scalar(3312, if s.b[3312] { 1.0 } else { 0.0 });
        let (t3,) = {
    if (((((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) && (!s.b[3311])) && s.b[3312]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t3);s.b[3313] = (4.0 == 4.0);s.store_scalar(3313, if s.b[3313] { 1.0 } else { 0.0 });
        let (t4,) = {
    if ((((((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) && (!s.b[3311])) && (!s.b[3312])) && s.b[3313]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t4);s.b[3314] = (4.0 == 8.0);s.store_scalar(3314, if s.b[3314] { 1.0 } else { 0.0 });
        let (t5,) = {
    if (((((((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) && (!s.b[3311])) && (!s.b[3312])) && (!s.b[3313])) && s.b[3314]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t5);
        let (t6,) = {
    if (((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t6);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_230(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut ta: usize = 0;
        while {
            let t9: f64 = if ((((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t9 != 0.0
        } {
            ta += 1;
            if ta > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", ta, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) {s.store_sqrt(726, 726);}
            let (t8,) = {
    if (((s.b[3307] && s.b[3308]) && s.b[3309]) && s.b[3310]) {
        let t7: f64 = (s.v[719] + 1.0);
        (t7,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t8);
        }
        if (((s.b[3307] && s.b[3308]) && s.b[3309]) && (!s.b[3310])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if ((s.b[3307] && s.b[3308]) && s.b[3309]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 341, 726);s.store_div_scaled_product3_indices(334, 341, 725, 726, 1.0, 770, 1.0);s.store_sub(340, 341, 780);}
        if ((s.b[3307] && s.b[3308]) && s.b[3309]) {
        }
        if ((s.b[3307] && s.b[3308]) && (!s.b[3309])) {
        }
        if ((s.b[3307] && s.b[3308]) && (!s.b[3309])) {s.store_scalar(334, 1.0);}
        if (s.b[3307] && s.b[3308]) {s.store_sqrt(340, 340);s.store_add_mul_sub_from_scalar_rhs_indices(282, 335, 337, 1.0, 340);s.store_div_from_scalar_offset_input(336, s.v[582], 667, s.v[582]);s.store_add_scaled_inputs_product_indices(283, 1435, s.v[488], 109, 1.0, 336, 282, (-1.0));s.store_sqrt_square_offset(782, 283, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(343, 283, 782, 0.5, 0.5);s.store_scaled_add(283, 283, 782, 0.5);}
        s.b[3315] = (s.v[283] < 0.0);s.store_scalar(3315, if s.b[3315] { 1.0 } else { 0.0 });
        if ((s.b[3307] && s.b[3308]) && s.b[3315]) {s.store_scalar(283, 0.0);s.store_scalar(343, 0.0);}
        if (s.b[3307] && s.b[3308]) {s.store_offset(283, 283, 1e-25);s.store_offset_mul_offset_rhs(958, 957, 387, (-s.v[764]), 1.0);}
        if (s.b[3307] && s.b[3308]) {
            if (s.v[958] <= 0.001) {
                s.store_scalar(958, 0.001);
            } else {
            }
        }
        if (s.b[3307] && s.b[3308]) {s.store_div(339, 668, 958);s.store_mul(340, 669, 958);s.store_ad_value(336, A::exp_div_scaled_inputs(s.ad_value(340), -1.0, s.ad_value(283), 1.0));s.store_mul3_lhs(428, 339, 283, 336);}
        s.b[3316] = (p[48] > 0.0);s.store_scalar(3316, if s.b[3316] { 1.0 } else { 0.0 });
        if ((s.b[3307] && s.b[3308]) && s.b[3316]) {s.store_mul_scale_offset_indices(429, 870, 428, 1.0, 1.0);}
        if ((s.b[3307] && s.b[3308]) && (!s.b[3316])) {s.store_mul_scale_offset_indices(429, 862, 428, 1.0, 1.0);}
        s.b[3317] = (s.v[78] == 0.0);s.store_scalar(3317, if s.b[3317] { 1.0 } else { 0.0 });
        if ((s.v[81] != 0.0) && s.b[3317]) {s.store_scalar(346, p[270]);s.store_scalar(344, p[271]);s.copy_ad(337, 170);s.store_mul_product3_indices(335, 337, 346, 344, 337, 1.0);s.store_offset_add_ad(336, A::mul3(s.ad_value(253), s.ad_value(127), s.ad_value(346)), A::mul3(s.ad_value(344), s.ad_value(337), s.ad_value(337)), 1e-25);s.store_div(306, 335, 336);}
        if ((s.v[81] != 0.0) && (!s.b[3317])) {s.store_scalar(306, p[270]);}
        if (s.v[81] != 0.0) {s.store_scalar(336, s.v[565]);s.store_mul(307, 336, 185);}
        s.b[3318] = ((p[26] != 0.0) && (s.v[78] == 0.0));s.store_scalar(3318, if s.b[3318] { 1.0 } else { 0.0 });
        if s.b[3318] {s.store_scalar(309, s.v[522]);s.store_scalar(311, s.v[563]);s.store_scale(335, 238, 6.241449993689894e18);s.store_sqrt_offset_ad(782, A::square(A::sub(s.ad_value(87), s.ad_value(1431))), ((4.0 * 0.001) * 0.001));s.store_scaled_offset_ad(334, A::div_scaled_inputs2(s.ad_value(87), 1.0, s.ad_value(1431), (-1.0), s.ad_value(782), 1.0), 1.0, 0.5);s.store_add_scaled_inputs3_indices(339, 87, 0.5, 1431, ((-1.0) * 0.5), 782, 0.5);}
        s.b[3319] = (s.v[339] < 0.0);s.store_scalar(3319, if s.b[3319] { 1.0 } else { 0.0 });
        if (s.b[3318] && s.b[3319]) {s.store_scalar(339, 0.0);s.store_scalar(334, 0.0);}
        if s.b[3318] {s.store_mul_scale_offset_mixed_ai(336, A::add_scaled_inputs3(s.ad_value(185), 1.0, A::div(s.ad_value(238), s.ad_value(339)), 1.0, s.ad_value(311), 1.0), 155, 6.241449993689894e18, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_231(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[3318] {s.store_sub_mixed_ai(337, A::div_scaled_inputs(s.ad_value(979), (((-2.0) * 6.241449993689894e18) * 1.0 / (s.v[635])), s.ad_value(170), 1.0), 335);}
        s.b[3320] = ((((s.v[337] - s.v[335])) as f64).abs() > (10.0 * 2.220446049250313e-16));s.store_scalar(3320, if s.b[3320] { 1.0 } else { 0.0 });
        if (s.b[3318] && s.b[3320]) {s.store_add_scaled_value_products_mixed_aaaai(338, A::div_scalar_by_product(1.0, A::add(s.ad_value(335), s.ad_value(336)), A::add(s.ad_value(337), s.ad_value(336)), 1.0), 1.0, A::div_scaled_product3(s.ad_value(309), s.ad_value(255), s.ad_value(253), 2.0, A::sub(s.ad_value(337), s.ad_value(335)), 1.0), A::ln(A::div_scaled_inputs2(s.ad_value(337), 1.0, s.ad_value(336), 1.0, A::add(s.ad_value(335), s.ad_value(336)), 1.0)), 1.0, A::mul3(A::mul3(s.ad_value(309), s.ad_value(255), s.ad_value(253)), s.ad_value(309), s.ad_value(255)), 253, 1.0);}
        if (s.b[3318] && (!s.b[3320])) {s.store_add_scaled_inputs_product_mixed_aaai(338, A::div_scalar_by_product(1.0, A::add(s.ad_value(335), s.ad_value(336)), A::add(s.ad_value(337), s.ad_value(336)), 1.0), 1.0, A::div_scaled_product3(s.ad_value(309), s.ad_value(255), s.ad_value(253), 2.0, A::add(s.ad_value(335), s.ad_value(336)), 1.0), 1.0, A::mul3(A::mul3(s.ad_value(309), s.ad_value(255), s.ad_value(253)), s.ad_value(309), s.ad_value(255)), 253, 1.0);}
        s.b[3321] = (((p[30] != 0.0) && (s.v[78] == 0.0)) && (s.v[963] == 0.0));s.store_scalar(3321, if s.b[3321] { 1.0 } else { 0.0 });
        if s.b[3321] {s.store_div_scaled_offset_numerator_mixed_ai(313, A::sub(s.ad_value(168), s.ad_value(87)), 1.0, (10.0 * 2.220446049250313e-16), 170, 1.0);}
        if s.b[3321] {
            if (s.v[313] >= 0.0) {
            } else {
                s.store_scalar(313, 0.0);
            }
        }
        if s.b[3321] {s.store_scaled_mul(346, 254, 313, 1e-7);}
        s.b[3322] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3322, if s.b[3322] { 1.0 } else { 0.0 });
        if (s.b[3321] && s.b[3322]) {s.store_scalar(341, 1.0);}
        s.b[3323] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p[178]) && (p[178] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3323, if s.b[3323] { 1.0 } else { 0.0 });
        if ((s.b[3321] && (!s.b[3322])) && s.b[3323]) {s.copy_ad(341, 346);}
        if ((s.b[3321] && (!s.b[3322])) && (!s.b[3323])) {
            if (s.v[313] == 0.0) {
                s.store_scalar(341, 0.0);
            } else {
                s.store_powf(341, 313, (p[178] - 1.0));
            }
        }
        if s.b[3321] {s.store_mul(342, 346, 341);s.store_offset(343, 342, 1.0);}
        if s.b[3321] {
            if (s.v[343] == 0.0) {
                s.store_scalar(344, 0.0);
            } else {
                s.store_powf(344, 343, (((-1.0) / p[178]) - 1.0));
            }
        }
        if s.b[3321] {s.store_mul(345, 343, 344);s.store_mul(316, 254, 345);s.store_scaled_add(314, 253, 316, 0.5);s.store_square(334, 125);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_232(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[3321] {s.store_div_scaled_product_by_product_mixed_aaai(315, A::mul3_scaled_output(s.ad_value(185), s.ad_value(127), s.ad_value(253), s.v[632]), A::add_scaled_inputs3(A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(125), 3.0, 1.0), 1.0, s.ad_value(334), 6.0), s.ad_value(316), s.ad_value(316)), 1.0, A::mul3(A::add_scaled_inputs(A::scale_offset(s.ad_value(125), 4.0, 3.0), 1.0, s.ad_value(334), 3.0), s.ad_value(316), s.ad_value(253)), 1.0, A::mul3(A::add(A::scale_offset(s.ad_value(125), 3.0, 6.0), s.ad_value(334)), s.ad_value(253), s.ad_value(253)), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(170), A::offset(s.ad_value(125), 1.0), s.ad_value(314), 15.0), 314, 1.0);}
        if (!s.b[3321]) {s.store_scalar(315, 0.0);}
        s.b[3324] = (((((p[31] != 0.0) && (p[30] != 0.0)) && (s.v[321] == 1.0)) && (s.v[78] == 0.0)) && (s.v[963] == 0.0));s.store_scalar(3324, if s.b[3324] { 1.0 } else { 0.0 });
        if s.b[3324] {s.store_sqrt(322, 319);s.store_add(336, 127, 322);s.store_square(337, 317);s.store_square(338, 319);s.store_scaled_mul(339, 317, 319, 42.0);s.store_add_scaled_inputs3_indices(339, 339, 1.0, 337, 4.0, 338, 4.0);s.store_add_product3_rhs_mixed_iia(339, 339, 322, 127, A::add(s.ad_value(317), s.ad_value(319)), 20.0);s.store_square(344, 336);s.store_square(344, 344);s.store_div_scaled_value_by_product_indices(323, 339, 1.0, 344, 336, 1.0);s.store_mul_ad_product_lhs_mixed_ai(324, A::div_from_scalar(s.v[632], s.ad_value(170)), 253, 185);s.store_mul(325, 324, 127);s.store_div(326, 315, 325);s.store_add_mixed_ai(341, A::add_scaled_product(s.ad_value(317), 1.0, s.ad_value(127), s.ad_value(322), 4.0), 319);s.store_div_scaled_product_by_product_mixed_iiia(327, 320, 341, 3.872983346207417, 336, A::sqrt(A::mul(A::mul3(s.ad_value(326), s.ad_value(336), s.ad_value(127)), s.ad_value(339))), 6.0);}
        s.store_scale(0, 134, s.v[365]);s.store_scale(699, 400, s.v[365]);s.copy_ad(430, 429);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_233(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(705, 0.0);s.store_scalar(706, 0.0);s.store_scalar(707, 0.0);s.store_scalar(811, 0.0);s.store_scalar(810, 0.0);s.store_scalar(812, 0.0);s.store_scalar(703, 0.0);s.store_scalar(704, 0.0);s.b[3325] = ((s.v[81] != 0.0) || (p[22] == 2.0));s.store_scalar(3325, if s.b[3325] { 1.0 } else { 0.0 });
        if s.b[3325] {s.store_scalar(700, 0.0);s.store_scalar(701, 0.0);s.store_scalar(702, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_234(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[3325] {s.copy_ad(708, 247);s.store_scale(754, 20, s.v[365]);s.store_scale(132, 132, s.v[365]);}
        if (!s.b[3325]) {s.store_scaled_add(700, 20, 132, (-s.v[365]));s.store_scale(701, 19, s.v[365]);s.store_scaled_sub(702, 132, 19, s.v[365]);}
        if (p[29] != 0.0) {s.store_scale(572, 91, s.v[572]);s.store_sqrt_square_offset(782, 572, ((4.0 * 1e-12) * 1e-12));s.store_offset_scaled_div(334, 572, 782, 0.5, 0.5);s.store_scaled_add(572, 572, 782, 0.5);}
        s.b[3326] = (s.v[572] < 0.0);s.store_scalar(3326, if s.b[3326] { 1.0 } else { 0.0 });
        if ((p[29] != 0.0) && s.b[3326]) {s.store_scalar(572, 0.0);s.store_scalar(334, 0.0);}
        if (p[29] != 0.0) {s.store_scale(308, 572, s.v[188]);s.store_voltage(817, ctx, nodes, Some(13), None);s.store_div_scaled_inputs2_indices(815, 817, 1.0, 816, (-1.0), 308, 1.0);s.store_add_scaled_inputs3_indices(352, 352, 1.0, 816, -1.0, 817, 1.0);s.copy_ad(355, 817);}
        if (p[29] == 0.0) {s.copy_ad(817, 816);}
        s.b[3327] = (p[22] > 0.0);s.store_scalar(3327, if s.b[3327] { 1.0 } else { 0.0 });
        if s.b[3327] {s.store_scaled_add_mixed_ai(811, A::add_scaled_inputs4(s.ad_value(293), 1.0, s.ad_value(352), (-1.0), s.ad_value(353), -1.0, s.ad_value(291), 1.0), 292, s.v[365]);s.store_scaled_sub(810, 355, 292, s.v[365]);s.store_scaled_sub(812, 356, 291, s.v[365]);s.store_add_scaled_inputs4_indices(700, 700, 1.0, 305, s.v[365], 360, ((-1.0) * s.v[365]), 362, (-s.v[365]));s.store_add_scaled_inputs3_indices(701, 701, 1.0, 361, s.v[365], 305, (-s.v[365]));s.store_add_scaled_inputs(702, 702, 1.0, 363, s.v[365]);s.store_sub_scaled_inputs(705, 350, (-s.v[365]), 351, s.v[365]);s.store_scale(706, 358, s.v[365]);s.store_scale(707, 359, s.v[365]);s.store_offset_sub_scaled_inputs_indices(703, 299, (-s.v[365]), 298, s.v[365], s.v[703]);s.store_offset_sub_scaled_inputs_indices(704, 301, (-s.v[365]), 297, s.v[365], s.v[704]);}
        s.store_scaled_add(709, 280, 287, s.v[365]);s.store_scale(710, 281, s.v[365]);s.store_scale(11, 202, (-s.v[365]));s.b[3328] = (s.v[949] == 1.0);s.store_scalar(3328, if s.b[3328] { 1.0 } else { 0.0 });
        if s.b[3328] {s.store_sub_scaled_inputs(9, 199, (p[252] * s.v[365]), 201, s.v[365]);}
        if (!s.b[3328]) {s.store_sub_scaled_inputs(9, 199, ((1.0 - p[252]) * s.v[365]), 200, s.v[365]);}
        s.b[3329] = (s.v[949] == 1.0);s.store_scalar(3329, if s.b[3329] { 1.0 } else { 0.0 });
        if s.b[3329] {s.store_sub_scaled_inputs(10, 199, ((1.0 - p[252]) * s.v[365]), 200, s.v[365]);}
        if (!s.b[3329]) {s.store_sub_scaled_inputs(10, 199, (p[252] * s.v[365]), 201, s.v[365]);}
        s.store_scale(7, 203, s.v[365]);s.store_scale(8, 204, s.v[365]);s.store_scale(807, 387, (4.0 * 1.3806226e-23));s.store_scale(712, 315, s.v[365]);s.store_scalar(22, A::ddx_projection(&s.ad_value(700), Some(5), None));s.store_scale(22, 22, p[87]);s.store_scalar(23, A::ddx_projection(&s.ad_value(700), Some(7), None));s.store_scale(23, 23, p[87]);
        if (s.v[949] > 0.0) {
            s.copy_ad(757, 23);
        } else {
            s.copy_ad(757, 22);
        }
        s.store_scalar(713, 0.0);s.store_scalar(714, 0.0);s.b[3330] = (((((p[31] != 0.0) && (p[30] != 0.0)) && (s.v[321] == 1.0)) && (s.v[78] == 0.0)) && (s.v[963] == 0.0));s.store_scalar(3330, if s.b[3330] { 1.0 } else { 0.0 });
        if s.b[3330] {s.store_scaled_mul(334, 185, 162, (1e-6 * s.v[635]));s.store_scale(344, 757, 1.0 / (s.v[365]));s.store_div_scaled_product3_indices(328, 155, 344, 344, (0.1185185185185185 * 1.6021918e-19), 324, 1.0);}
        s.b[3331] = ((s.v[320] > (10.0 * 2.220446049250313e-16)) && (s.v[790] > (10.0 * 2.220446049250313e-16)));s.store_scalar(3331, if s.b[3331] { 1.0 } else { 0.0 });
        if (s.b[3330] && s.b[3331]) {s.store_div(329, 254, 253);s.store_div_scaled_inputs2_mixed_aii(330, A::div(s.ad_value(254), s.ad_value(316)), 1.0, 329, (-1.0), 790, 1.0);s.store_add_mixed_ia(331, 329, A::div_scaled_product(s.ad_value(330), A::add(A::add_scaled_product(s.ad_value(317), 1.0, s.ad_value(127), s.ad_value(322), 1.0), s.ad_value(319)), 0.6666666666666667, A::add(s.ad_value(127), s.ad_value(322)), 1.0));}
        if (s.b[3330] && (!s.b[3331])) {s.store_div(331, 254, 316);}
        if s.b[3330] {s.store_mul3_affine_lhs(713, 328, 323, s.v[365], 0.0, 331);s.copy_ad(714, 327);}
        if s.b[3330] {
            if (s.v[713] < 0.0) {
                s.store_scalar(713, 0.0);
            } else {
            }
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_235(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if s.b[3330] {
            if ((-s.v[344]) > s.v[334]) {
            } else {
                s.store_scalar(713, 0.0);
            }
        }
        if s.b[3330] {
            if ((-s.v[344]) > s.v[334]) {
            } else {
                s.store_scalar(714, 0.0);
            }
        }
        s.store_mul(952, 807, 712);s.copy_ad(951, 714);
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
        s.store_scalar(716, 0.0);s.store_scalar(715, 0.0);s.b[3332] = (s.v[449] == 1.0);s.store_scalar(3332, if s.b[3332] { 1.0 } else { 0.0 });s.b[3333] = (s.v[76] == 0.0);s.store_scalar(3333, if s.b[3333] { 1.0 } else { 0.0 });s.b[3334] = ((p[53] > 0.0) && (s.v[541] != 0.0));s.store_scalar(3334, if s.b[3334] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3333])) && s.b[3334]) {
            if (s.v[676] == 0.0) {
                s.store_scalar(335, 0.0);
            } else {
                s.store_powf(335, 676, p[416]);
            }
        }
        if ((s.b[3332] && (!s.b[3333])) && s.b[3334]) {s.store_div_from_scalar(794, s.v[569], 335);s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p[418]), p[418]));s.store_div_from_scalar(795, s.v[570], 334);s.store_add_mixed_ia(959, 959, A::scaled_offset(s.ad_value(387), (-s.v[764]), p[439]));}
        if ((s.b[3332] && (!s.b[3333])) && (!s.b[3334])) {s.store_scalar(387, (ctx_temp + p[11]));}
        if (s.b[3332] && (!s.b[3333])) {s.store_scalar(164, (s.v[630] * p[7]));s.store_scalar(604, p[71]);s.store_scalar(605, s.v[460]);s.store_mul(606, 794, 653);s.store_offset_product3(607, s.ad_value(795), s.ad_value(786), s.ad_value(652), 1.0, 1e-25);s.store_div(608, 804, 604);s.store_mul(609, 606, 608);}
        s.b[3335] = (s.v[804] >= 0.0);s.store_scalar(3335, if s.b[3335] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3333])) && s.b[3335]) {s.store_div(335, 609, 607);}
        if ((s.b[3332] && (!s.b[3333])) && (!s.b[3335])) {s.store_div_scaled_inputs_indices(335, 609, -1.0, 607, 1.0);}
        s.b[3336] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[959]) && (s.v[959] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3336, if s.b[3336] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3333])) && s.b[3336]) {s.store_scalar(337, 1.0);}
        s.b[3337] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[959]) && (s.v[959] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3337, if s.b[3337] { 1.0 } else { 0.0 });
        if (((s.b[3332] && (!s.b[3333])) && (!s.b[3336])) && s.b[3337]) {s.copy_ad(337, 335);}
        if (((s.b[3332] && (!s.b[3333])) && (!s.b[3336])) && (!s.b[3337])) {s.store_pow_offset_rhs(337, 335, 959, (-1.0));}
        if (s.b[3332] && (!s.b[3333])) {s.store_mul(336, 335, 337);s.store_offset(338, 336, 1.0);}
        s.b[3338] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[959]) && (s.v[959] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3338, if s.b[3338] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3333])) && s.b[3338]) {s.store_div_from_scalar(339, 1.0, 338);}
        s.b[3339] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[959]) && (s.v[959] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3339, if s.b[3339] { 1.0 } else { 0.0 });
        if (((s.b[3332] && (!s.b[3333])) && (!s.b[3338])) && s.b[3339]) {s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));}
        if (((s.b[3332] && (!s.b[3333])) && (!s.b[3338])) && (!s.b[3339])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_pow_ad(340, s.ad_value(338), A::offset(A::div_from_scalar((-1.0), s.ad_value(959)), (-1.0)));
            }
        }
        if (((s.b[3332] && (!s.b[3333])) && (!s.b[3338])) && (!s.b[3339])) {s.store_mul(339, 338, 340);}
        if (s.b[3332] && (!s.b[3333])) {s.store_mul(610, 606, 339);s.copy_ad(611, 605);s.copy_ad(612, 614);s.store_div_from_scalar(335, 1.6021918e-19, 604);s.store_mul_product3_indices(613, 611, 335, 612, 610, 1.0);}
        s.b[3340] = ((s.v[613] < 1e-25) && (1e-25 >= 0.0));s.store_scalar(3340, if s.b[3340] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3333])) && s.b[3340]) {s.store_sub_from_scalar(781, 1e-25, 613);s.store_square(722, 781);s.store_scalar(723, (1e-25 * 1e-25));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tb,) = {
    if ((s.b[3332] && (!s.b[3333])) && s.b[3340]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tb);
        let (tc,) = {
    if ((s.b[3332] && (!s.b[3333])) && s.b[3340]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc);
        if ((s.b[3332] && (!s.b[3333])) && s.b[3340]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_236(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if ((s.b[3332] && (!s.b[3333])) && s.b[3340]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3341] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(3341, if s.b[3341] { 1.0 } else { 0.0 });s.b[3342] = (2.0 == 1.0);s.store_scalar(3342, if s.b[3342] { 1.0 } else { 0.0 });
        let (td,) = {
    if ((((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) && s.b[3342]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, td);s.b[3343] = (2.0 == 2.0);s.store_scalar(3343, if s.b[3343] { 1.0 } else { 0.0 });
        let (te,) = {
    if (((((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) && (!s.b[3342])) && s.b[3343]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, te);s.b[3344] = (2.0 == 4.0);s.store_scalar(3344, if s.b[3344] { 1.0 } else { 0.0 });
        let (tf,) = {
    if ((((((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) && (!s.b[3342])) && (!s.b[3343])) && s.b[3344]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tf);s.b[3345] = (2.0 == 8.0);s.store_scalar(3345, if s.b[3345] { 1.0 } else { 0.0 });
        let (t10,) = {
    if (((((((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) && (!s.b[3342])) && (!s.b[3343])) && (!s.b[3344])) && s.b[3345]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t10);
        let (t11,) = {
    if (((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t11);let mut t15: usize = 0;
        while {
            let t14: f64 = if ((((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t14 != 0.0
        } {
            t15 += 1;
            if t15 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t15, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) {s.store_sqrt(726, 726);}
            let (t13,) = {
    if (((s.b[3332] && (!s.b[3333])) && s.b[3340]) && s.b[3341]) {
        let t12: f64 = (s.v[719] + 1.0);
        (t12,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t13);
        }
        if (((s.b[3332] && (!s.b[3333])) && s.b[3340]) && (!s.b[3341])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[3332] && (!s.b[3333])) && s.b[3340]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-25);s.store_div_scaled_product_indices(334, 725, 726, 1e-25, 770, 1.0);s.store_sub_from_scalar(613, 1e-25, 780);}
        if ((s.b[3332] && (!s.b[3333])) && s.b[3340]) {
        }
        if ((s.b[3332] && (!s.b[3333])) && (!s.b[3340])) {
        }
        if ((s.b[3332] && (!s.b[3333])) && (!s.b[3340])) {s.store_scalar(334, 1.0);}
        if (s.b[3332] && (!s.b[3333])) {s.store_div_from_scalar(5, 1.0, 613);s.store_div(5, 5, 164);s.store_add(5, 5, 648);}
        s.b[3347] = (s.v[5] < p[444]);s.store_scalar(3347, if s.b[3347] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3333])) && s.b[3347]) {s.store_scalar(5, p[444]);}
        if (s.b[3332] && (!s.b[3333])) {s.store_scale(716, 5, 1.0 / (s.v[365]));}
        s.b[3352] = (s.v[75] == 0.0);s.store_scalar(3352, if s.b[3352] { 1.0 } else { 0.0 });
        if (s.b[3332] && (!s.b[3352])) {s.copy_ad(3348, 729);s.copy_ad(3349, 728);}
        s.b[3353] = ((p[53] > 0.0) && (s.v[541] != 0.0));s.store_scalar(3353, if s.b[3353] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3352])) && s.b[3353]) {
            if (s.v[676] == 0.0) {
                s.store_scalar(335, 0.0);
            } else {
                s.store_powf(335, 676, p[415]);
            }
        }
        if ((s.b[3332] && (!s.b[3352])) && s.b[3353]) {s.store_div_from_scalar(787, s.v[567], 335);s.store_sub_ad(334, A::add_scaled_product(A::scale_offset(s.ad_value(676), 0.4, 1.8), 1.0, s.ad_value(676), s.ad_value(676), 0.1), A::scale_offset(s.ad_value(676), (-p[417]), p[417]));s.store_div_from_scalar(788, s.v[568], 334);s.store_add_mixed_ia(956, 956, A::scaled_offset(s.ad_value(387), (-s.v[764]), p[438]));}
        s.b[3355] = (s.v[956] < 0.1);s.store_scalar(3355, if s.b[3355] { 1.0 } else { 0.0 });
        if (((s.b[3332] && (!s.b[3352])) && s.b[3353]) && s.b[3355]) {s.store_scalar(956, 0.1);}
        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3353])) {s.store_scalar(387, (ctx_temp + p[11]));}
        if (s.b[3332] && (!s.b[3352])) {s.store_scalar(164, (s.v[630] * p[7]));s.store_scalar(785, (p[67] + p[68]));s.store_primal_offset(789, 451, 1e-12);s.store_scalar(408, s.v[459]);s.store_offset_ad(335, A::mul_sub_from_scalar_rhs(s.ad_value(3349), p[410], A::scale(s.ad_value(3349), p[411])), 1.0);s.store_sqrt_square_offset(782, 335, ((4.0 * 0.1) * 0.1));s.store_offset_scaled_div(336, 335, 782, 0.5, 0.5);s.store_scaled_add(654, 335, 782, 0.5);}
        s.b[3356] = (s.v[654] < 0.0);s.store_scalar(3356, if s.b[3356] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3352])) && s.b[3356]) {s.store_scalar(654, 0.0);s.store_scalar(336, 0.0);}
        if (s.b[3332] && (!s.b[3352])) {s.store_mul3_lhs(593, 787, 653, 654);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_237(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[3332] && (!s.b[3352])) {s.store_offset_product3(3351, s.ad_value(788), s.ad_value(786), s.ad_value(652), 1.0, 1e-25);s.copy_ad(594, 453);s.store_scalar(595, p[421]);s.store_scale(335, 593, 10000.0);s.store_scale(336, 3351, 100.0);}
        s.b[3359] = (s.v[799] < 0.0);s.store_scalar(3359, if s.b[3359] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3352])) && s.b[3359]) {s.store_scale(781, 799, ((-0.5) * (2.0 * 1.0 / (p[262]))));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(108, p[262], 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);}
        s.b[3360] = (s.v[108] < 1e-12);s.store_scalar(3360, if s.b[3360] { 1.0 } else { 0.0 });
        if (((s.b[3332] && (!s.b[3352])) && s.b[3359]) && s.b[3360]) {s.store_scalar(108, 1e-12);}
        if ((s.b[3332] && (!s.b[3352])) && s.b[3359]) {s.store_sub_scaled_inputs(598, 799, 1.0, 108, 2.0);}
        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3359])) {s.store_scale(781, 799, (0.5 * (2.0 * 1.0 / (p[262]))));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(108, p[262], 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);}
        s.b[3361] = (s.v[108] < 1e-12);s.store_scalar(3361, if s.b[3361] { 1.0 } else { 0.0 });
        if (((s.b[3332] && (!s.b[3352])) && (!s.b[3359])) && s.b[3361]) {s.store_scalar(108, 1e-12);}
        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3359])) {s.store_add_scaled_inputs(598, 799, 1.0, 108, 2.0);}
        if (s.b[3332] && (!s.b[3352])) {s.store_div(591, 598, 785);s.store_mul(592, 593, 591);}
        s.b[3362] = (s.v[799] >= 0.0);s.store_scalar(3362, if s.b[3362] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3352])) && s.b[3362]) {s.store_div(335, 592, 3351);}
        if ((s.b[3332] && (!s.b[3352])) && (!s.b[3362])) {s.store_div_scaled_inputs_indices(335, 592, -1.0, 3351, 1.0);}
        s.b[3363] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3363, if s.b[3363] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3352])) && s.b[3363]) {s.store_scalar(337, 1.0);}
        s.b[3364] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3364, if s.b[3364] { 1.0 } else { 0.0 });
        if (((s.b[3332] && (!s.b[3352])) && (!s.b[3363])) && s.b[3364]) {s.copy_ad(337, 335);}
        if (((s.b[3332] && (!s.b[3352])) && (!s.b[3363])) && (!s.b[3364])) {s.store_pow_offset_rhs(337, 335, 956, (-1.0));}
        if (s.b[3332] && (!s.b[3352])) {s.store_mul(336, 335, 337);s.store_offset(338, 336, 1.0);}
        s.b[3365] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3365, if s.b[3365] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3352])) && s.b[3365]) {s.store_div_from_scalar(339, 1.0, 338);}
        s.b[3366] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(3366, if s.b[3366] { 1.0 } else { 0.0 });
        if (((s.b[3332] && (!s.b[3352])) && (!s.b[3365])) && s.b[3366]) {s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));}
        if (((s.b[3332] && (!s.b[3352])) && (!s.b[3365])) && (!s.b[3366])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_pow_ad(340, s.ad_value(338), A::offset(A::div_from_scalar((-1.0), s.ad_value(956)), (-1.0)));
            }
        }
        if (((s.b[3332] && (!s.b[3352])) && (!s.b[3365])) && (!s.b[3366])) {s.store_mul(339, 338, 340);}
        if (s.b[3332] && (!s.b[3352])) {s.store_mul(3350, 593, 339);s.store_offset(338, 335, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_238(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[3332] && (!s.b[3352])) {s.store_div_from_scalar(339, 1.0, 338);s.store_offset_ad(338, A::div_scaled_product_offset_denominator(A::mul_sub_from_scalar_rhs(s.ad_value(595), 1.0, s.ad_value(339)), s.ad_value(598), 1.0, s.ad_value(785), (-p[423]), 1.0), 1.0);s.store_offset(781, 338, (-0.001));s.store_scalar(782, 0.0);}
        if (s.b[3332] && (!s.b[3352])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[3332] && (!s.b[3352])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_scaled_add(339, 781, 782, 0.5);s.store_mul(717, 408, 339);s.store_scale(718, 698, (6.241449993689894e18 * p[430]));s.store_add_scaled_inputs3_indices(781, 717, 1.0, 718, (-1.0), 717, (-0.001));s.store_scaled_mul(782, 717, 717, (4.0 * 0.001));}
        if (s.b[3332] && (!s.b[3352])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[3332] && (!s.b[3352])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(718, 717, 1.0, 781, (-0.5), 782, (-0.5));s.store_sub(597, 717, 718);}
        s.b[3367] = ((p[441] > 0.0) && (p[440] > 1.0));s.store_scalar(3367, if s.b[3367] { 1.0 } else { 0.0 });s.b[3368] = ((s.v[597] > ((s.v[408] * p[440]) - (s.v[408] * p[441]))) && ((s.v[408] * p[441]) >= 0.0));s.store_scalar(3368, if s.b[3368] { 1.0 } else { 0.0 });
        if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {s.store_add_scaled_inputs3_indices(781, 597, 1.0, 408, (-p[440]), 408, p[441]);s.store_square(722, 781);s.store_scaled_mul(723, 408, 408, (p[441] * p[441]));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t16,) = {
    if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t16);
        let (t17,) = {
    if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t17);
        if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);}
        let (t18,) = {
    if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t18);let mut t1c: usize = 0;
        while {
            let t1b: f64 = if ((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && (s.v[719] < p[442])) { 1.0 } else { 0.0 };
            t1b != 0.0
        } {
            t1c += 1;
            if t1c > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t1c, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);}
            let (t1a,) = {
    if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {
        let t19: f64 = (s.v[719] + 1.0);
        (t19,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t1a);
        }
        if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3369] = ((((p[442] == 1.0) || (p[442] == 2.0)) || (p[442] == 4.0)) || (p[442] == 8.0));s.store_scalar(3369, if s.b[3369] { 1.0 } else { 0.0 });s.b[3370] = (p[442] == 1.0);s.store_scalar(3370, if s.b[3370] { 1.0 } else { 0.0 });
        let (t1d,) = {
    if (((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) && s.b[3370]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t1d);s.b[3371] = (p[442] == 2.0);s.store_scalar(3371, if s.b[3371] { 1.0 } else { 0.0 });
        let (t1e,) = {
    if ((((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) && (!s.b[3370])) && s.b[3371]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t1e);s.b[3372] = (p[442] == 4.0);s.store_scalar(3372, if s.b[3372] { 1.0 } else { 0.0 });
        let (t1f,) = {
    if (((((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) && (!s.b[3370])) && (!s.b[3371])) && s.b[3372]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t1f);s.b[3373] = (p[442] == 8.0);s.store_scalar(3373, if s.b[3373] { 1.0 } else { 0.0 });
        let (t20,) = {
    if ((((((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) && (!s.b[3370])) && (!s.b[3371])) && (!s.b[3372])) && s.b[3373]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t20);
        let (t21,) = {
    if ((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t21);let mut t25: usize = 0;
        while {
            let t24: f64 = if (((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t24 != 0.0
        } {
            t25 += 1;
            if t25 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("transient stamp", t25, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) {s.store_sqrt(726, 726);}
            let (t23,) = {
    if ((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && s.b[3369]) {
        let t22: f64 = (s.v[719] + 1.0);
        (t22,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t23);
        }
        if ((((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) && (!s.b[3369])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * p[442])));
            }
        }
        if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_affine_lhs(780, 781, 408, p[441], 0.0, 726);s.store_div_scaled_product3_indices(334, 408, 725, 726, p[441], 770, 1.0);s.store_add_scaled_inputs3_indices(336, 408, p[440], 408, (-p[441]), 780, 1.0);}
        if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && s.b[3368]) {
        }
        if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && (!s.b[3368])) {s.copy_ad(336, 597);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_239(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[3332] && (!s.b[3352])) && s.b[3367]) && (!s.b[3368])) {s.store_scalar(334, 1.0);}
        if ((s.b[3332] && (!s.b[3352])) && s.b[3367]) {s.copy_ad(597, 336);}
        if (s.b[3332] && (!s.b[3352])) {s.store_neg(334, 697);s.store_sqrt_square_offset(782, 334, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(343, 334, 782, 0.5, 0.5);s.store_scaled_add(334, 334, 782, 0.5);}
        s.b[3374] = (s.v[334] < 0.0);s.store_scalar(3374, if s.b[3374] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3352])) && s.b[3374]) {s.store_scalar(334, 0.0);s.store_scalar(343, 0.0);}
        if (s.b[3332] && (!s.b[3352])) {s.store_offset(334, 334, (10.0 * 2.220446049250313e-16));s.store_sqrt_mul(599, 650, 334);s.store_offset_sub(336, 3348, 3349, p[137]);s.store_sqrt_square_offset(782, 336, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[3375] = (s.v[336] < 0.0);s.store_scalar(3375, if s.b[3375] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3352])) && s.b[3375]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (s.b[3332] && (!s.b[3352])) {s.store_offset(336, 336, (10.0 * 2.220446049250313e-16));s.store_sqrt_mul(600, 651, 336);s.store_add_scaled_inputs3_indices(781, 789, 1.0, 600, (-1.0), 789, (-0.01));s.store_scaled_mul(782, 789, 789, (4.0 * 0.01));}
        if (s.b[3332] && (!s.b[3352])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.b[3332] && (!s.b[3352])) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(602, 789, 1.0, 781, (-0.5), 782, (-0.5));s.store_scalar(601, (p[419] + 1e-25));s.store_mul_scale_offset_mixed_ia(596, 649, A::mul(s.ad_value(594), A::add(A::div(s.ad_value(599), s.ad_value(601)), A::div(s.ad_value(602), s.ad_value(789)))), -1.0, 1.0);s.store_sqrt_ad(782, A::add_scaled_square_product(s.ad_value(596), 1.0, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(453), s.ad_value(649)), A::mul_sub_from_scalar_lhs(1.0, s.ad_value(453), s.ad_value(649)), ((1.0 / (100.0) * 4.0) * 1.0 / (100.0))));s.store_offset_scaled_div(343, 596, 782, 0.5, 0.5);s.store_scaled_add(596, 596, 782, 0.5);}
        s.b[3376] = (s.v[596] < 0.0);s.store_scalar(3376, if s.b[3376] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3352])) && s.b[3376]) {s.store_scalar(596, 0.0);s.store_scalar(343, 0.0);}
        if (s.b[3332] && (!s.b[3352])) {s.store_div_from_scalar_offset_input(335, 1.6021918e-19, 785, p[422]);s.store_mul_product3_indices(739, 597, 335, 596, 3350, 1.0);}
        s.b[3377] = ((s.v[739] < 1e-25) && (1e-25 >= 0.0));s.store_scalar(3377, if s.b[3377] { 1.0 } else { 0.0 });
        if ((s.b[3332] && (!s.b[3352])) && s.b[3377]) {s.store_sub_from_scalar(781, 1e-25, 739);s.store_square(722, 781);s.store_scalar(723, (1e-25 * 1e-25));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t26,) = {
    if ((s.b[3332] && (!s.b[3352])) && s.b[3377]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t26);
        let (t27,) = {
    if ((s.b[3332] && (!s.b[3352])) && s.b[3377]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t27);
        if ((s.b[3332] && (!s.b[3352])) && s.b[3377]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[3378] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(3378, if s.b[3378] { 1.0 } else { 0.0 });s.b[3379] = (2.0 == 1.0);s.store_scalar(3379, if s.b[3379] { 1.0 } else { 0.0 });
        let (t28,) = {
    if ((((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) && s.b[3379]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t28);s.b[3380] = (2.0 == 2.0);s.store_scalar(3380, if s.b[3380] { 1.0 } else { 0.0 });
        let (t29,) = {
    if (((((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) && (!s.b[3379])) && s.b[3380]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t29);s.b[3381] = (2.0 == 4.0);s.store_scalar(3381, if s.b[3381] { 1.0 } else { 0.0 });
        let (t2a,) = {
    if ((((((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) && (!s.b[3379])) && (!s.b[3380])) && s.b[3381]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t2a);s.b[3382] = (2.0 == 8.0);s.store_scalar(3382, if s.b[3382] { 1.0 } else { 0.0 });
        let (t2b,) = {
    if (((((((s.b[3332] && (!s.b[3352])) && s.b[3377]) && s.b[3378]) && (!s.b[3379])) && (!s.b[3380])) && (!s.b[3381])) && s.b[3382]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t2b);
    }
}

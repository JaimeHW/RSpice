#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_25(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1267] {
            s.store_mul_div_scaled_inputs_mixed_aii(946, {
                if (!((s.v[933] / s.v[945]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[933] / s.v[945]) > 1e-38) {
                            A::ln(A::div(s.ad_value(933), s.ad_value(945)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 943, ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.85), 944, 1.0);
        }
        if s.b[1267] {s.store_scalar(627, 1.2e-12);s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p[42], 933, ((-0.5) * p[42]), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p[42]));s.store_add(1038, 938, 947);}
        s.b[1276] = (p[1090] > 0.0);s.store_scalar(1276, if s.b[1276] { 1.0 } else { 0.0 });
        if (s.b[1267] && s.b[1276]) {s.store_scalar(1032, 0.0);}
        s.b[1277] = (p[1080] > 0.0);s.store_scalar(1277, if s.b[1277] { 1.0 } else { 0.0 });
        if ((s.b[1267] && (!s.b[1276])) && s.b[1277]) {s.store_scalar(1032, ((p[4] - p[43]) * ((p[1080] * p[1084]) + p[1081])));}
        if ((s.b[1267] && (!s.b[1276])) && (!s.b[1277])) {s.store_primal_scale(1032, 450, (p[4] - p[43]));}
        if s.b[1267] {s.store_primal_scale(1033, 1031, (p[4] - p[43]));s.store_primal_scaled_offset_ad(455, A::add_scaled_inputs(s.ad_value(1032), p[5], s.ad_value(1033), ((2.0 * p[56]) * p[5])), ((p[1092]) + (p[1091])), (s.v[144] * 1.0 / (p[1087])));s.store_scaled_add_ad(453, A::add_scaled_inputs3(s.ad_value(455), 1.0, s.ad_value(1034), p[5], s.ad_value(1035), ((2.0 * p[56]) * p[5])), A::add_scaled_inputs3(s.ad_value(1036), (p[1103] * (p[5] * 2.0)), s.ad_value(1037), ((p[56] - 1.0) * (p[1103] * (p[5] * 2.0))), s.ad_value(1038), (p[1103] * (p[5] * 2.0))), p[59]);s.store_scale(453, 453, (0.0_f64).max((((p[1099] + (p[1100] * p[43])) + (p[1101] * p[4])) + (p[1102] * p[20]))));}
        s.store_scalar(168, (p[1583] * (if (!((1.0 + (p[92] / p[91])) > 1e-38)) { (-87.498233534) } else { (if ((1.0 + (p[92] / p[91])) > 1e-38) { (((1.0 + (p[92] / p[91]))) as f64).ln() } else { 0.0 }) })));s.store_scalar(515, ((s.v[165] * p[7]) + (s.v[168] * (0.0_f64).max((p[9] - (p[4] * s.v[115]))))));s.store_scalar(516, ((s.v[165] * p[8]) + (s.v[168] * (0.0_f64).max((p[10] - (p[4] * s.v[115]))))));s.b[1278] = (p[62] != 5.0);s.store_scalar(1278, if s.b[1278] { 1.0 } else { 0.0 });
        if s.b[1278] {s.store_primal_scale(517, 149, (((p[1544] * p[59]) * p[6]) + (p[1545] * s.v[115])));}
        if (!s.b[1278]) {s.store_primal_mul_scale_offset_rhs(517, 149, 161, ((p[1546]) * (s.v[115])), ((((p[1545]) * (s.v[115]))) + (((p[1544] * p[59]) * p[6]))));}
        s.store_scalar(420, (1e-8 / (s.v[145] * p[89])));s.store_primal_div_from_scalar_scaled_ad(189, 1.0, A::pow(A::scale(s.ad_value(158), 1000000.0), s.ad_value(713)), s.v[115]);s.store_scalar(578, (((((s.v[145] * p[89]) * 0.5) * p[3])) as f64).sqrt());s.store_primal_sqrt_ad(351, A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(894), s.v[143], s.ad_value(893), 1.0), A::div_scaled_product_by_product(s.ad_value(894), s.ad_value(893), 1.0, s.ad_value(895), s.ad_value(895), (2.0 * s.v[143])), 1.0));s.b[1279] = (!param_given[172]);s.store_scalar(1279, if s.b[1279] { 1.0 } else { 0.0 });
        if s.b[1279] {s.store_offset_div_scaled_product_indices(360, 670, 153, 1.0, 351, 1.0, 1e-6);}
        s.b[1280] = (s.v[360] < 40.0);s.store_scalar(1280, if s.b[1280] { 1.0 } else { 0.0 });
        if (s.b[1279] && s.b[1280]) {s.store_div_from_scalar_offset_ad(361, 0.5, A::cosh(s.ad_value(360)), (-1.0));}
        if (s.b[1279] && (!s.b[1280])) {s.store_limited_exp_neg_input(361, 360);}
        if (!s.b[1279]) {s.store_scalar(361, p[172]);}
        s.b[1281] = (!param_given[174]);s.store_scalar(1281, if s.b[1281] { 1.0 } else { 0.0 });
        if s.b[1281] {s.store_offset_div_scaled_product_indices(360, 671, 153, 1.0, 351, 1.0, 1e-6);}
        s.b[1282] = (s.v[360] < 40.0);s.store_scalar(1282, if s.b[1282] { 1.0 } else { 0.0 });
        if (s.b[1281] && s.b[1282]) {s.store_div_from_scalar_offset_ad(362, 0.5, A::cosh(s.ad_value(360)), (-1.0));}
        if (s.b[1281] && (!s.b[1282])) {s.store_limited_exp_neg_input(362, 360);}
        if (!s.b[1281]) {s.store_scalar(362, p[174]);}
        s.b[1283] = (!param_given[173]);s.store_scalar(1283, if s.b[1283] { 1.0 } else { 0.0 });
        if s.b[1283] {s.store_offset_div_scaled_product_indices(360, 678, 153, 1.0, 351, 1.0, 1e-6);}
        s.b[1284] = (s.v[360] < 40.0);s.store_scalar(1284, if s.b[1284] { 1.0 } else { 0.0 });
        if (s.b[1283] && s.b[1284]) {s.store_div_from_scalar_offset_ad(363, 0.5, A::cosh(s.ad_value(360)), (-1.0));}
        if (s.b[1283] && (!s.b[1284])) {s.store_limited_exp_neg_input(363, 360);}
        if (!s.b[1283]) {s.store_scalar(363, p[173]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_26(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_offset_sqrt_ad(364, A::offset(A::div(s.ad_value(803), s.ad_value(153)), 1.0), (-1.0));s.store_offset_div_scaled_product_indices(360, 678, 153, 1.0, 351, 1.0, 1e-6);s.b[1285] = (s.v[360] < 40.0);s.store_scalar(1285, if s.b[1285] { 1.0 } else { 0.0 });
        if s.b[1285] {s.store_div_from_scalar_ad(365, 1.0, A::max_with_scalar(A::scale_offset(A::cosh(s.ad_value(360)), p[171], (((((-2.0)) * (p[171]))) + (1.0))), 1e-6));}
        if (!s.b[1285]) {let t0: A = A::limited_exp_scaled_input(s.ad_value(360), -1.0);s.store_div_ad(365, t0, A::max_with_scalar(A::offset(t0, p[171]), 1e-6));}
        s.store_primal_div_scaled_product_indices(396, 640, 894, 1.60219e-19, 893, 1.0);s.b[1286] = (p[60] == 1.0);s.store_scalar(1286, if s.b[1286] { 1.0 } else { 0.0 });
        if s.b[1286] {s.store_scalar(485, 745669000000.0);}
        if (!s.b[1286]) {s.store_scalar(485, 1166450000000.0);}
        s.store_scalar(168, (p[1109] * p[1109]));s.store_scale(169, 742, p[1109]);s.store_square(170, 169);s.b[1287] = (p[1717] < (-273.15));s.store_scalar(1287, if s.b[1287] { 1.0 } else { 0.0 });
        if s.b[1287] {s.store_scalar(228, 300.15);}
        if (!s.b[1287]) {s.store_scalar(228, (p[1717] + 273.15));}
        s.b[1288] = (p[57] == 1.0);s.store_scalar(1288, if s.b[1288] { 1.0 } else { 0.0 });
        if s.b[1288] {s.store_primal_add_mixed_ai(960, A::scale_offset(s.ad_value(882), (-1.0 / ((1.0 + { let limited_exp_arg = (((p[1827] * 1000000000.0) - (p[43] * 1000000000.0)) / p[1828]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), ((p[1806]) * (1.0 / ((1.0 + { let limited_exp_arg = (((p[1827] * 1000000000.0) - (p[43] * 1000000000.0)) / p[1828]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))))), 882);}
        if s.b[1288] {s.store_primal_add_mixed_ai(961, A::scale_offset(s.ad_value(883), (-1.0 / ((1.0 + { let limited_exp_arg = (((p[1827] * 1000000000.0) - (p[43] * 1000000000.0)) / p[1828]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), ((p[1813]) * (1.0 / ((1.0 + { let limited_exp_arg = (((p[1827] * 1000000000.0) - (p[43] * 1000000000.0)) / p[1828]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))))), 883);}
        if s.b[1288] {s.store_primal_add_mixed_ai(962, A::scale_offset(s.ad_value(884), (-1.0 / ((1.0 + { let limited_exp_arg = (((p[1827] * 1000000000.0) - (p[43] * 1000000000.0)) / p[1828]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), ((p[1820]) * (1.0 / ((1.0 + { let limited_exp_arg = (((p[1827] * 1000000000.0) - (p[43] * 1000000000.0)) / p[1828]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))))), 884);}
        if s.b[1288] {s.store_primal_scaled_add_sqrt_square_offset_ad(963, A::offset(s.ad_value(885), ((-p[1847]) / (1.0 + { let limited_exp_arg = (((p[1850] * 1000000000.0) - (p[43] * 1000000000.0)) / p[1851]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), ((0.25 * 0.001) * 0.001), 0.5);}
        if s.b[1288] {s.store_primal_scaled_add_sqrt_square_offset_ad(964, A::offset(s.ad_value(886), ((-p[1848]) / (1.0 + { let limited_exp_arg = (((p[1850] * 1000000000.0) - (p[43] * 1000000000.0)) / p[1851]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), ((0.25 * 0.001) * 0.001), 0.5);}
        if s.b[1288] {s.store_primal_scaled_add_sqrt_square_offset_ad(965, A::offset(s.ad_value(887), ((-p[1849]) / (1.0 + { let limited_exp_arg = (((p[1850] * 1000000000.0) - (p[43] * 1000000000.0)) / p[1851]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), ((0.25 * 0.001) * 0.001), 0.5);}
        if s.b[1288] {let t1: A = A::sqrt_square_offset(A::scaled_offset(s.ad_value(960), (-1.001), 1.001), ((0.25 * 0.001) * 0.001));s.store_primal_offset_add_scaled_inputs3_offset_mixed_iaa(966, 960, ((0.5 * 1.001) * 0.5), t1, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(960), (-1.001), 1.001), 0.5, t1, 0.5), (-1.0)), ((0.25 * 0.001) * 0.001)), (-0.5), ((1.0 + (0.5 * ((-1.001) * 1.001))) * 0.5), (0.25 * 0.001));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_27(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1288] {let t2: A = A::sqrt_square_offset(A::scaled_offset(s.ad_value(960), (-2.001), 1.001), ((0.25 * 0.001) * 0.001));s.store_primal_offset_add_scaled_inputs3_offset_mixed_iaa(969, 960, ((0.5 * 1.001) * 0.5), t2, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(960), (-2.001), 1.001), 0.5, t2, 0.5), (-1.0)), ((0.25 * 0.001) * 0.001)), (-0.5), ((1.0 + (0.5 * ((-2.001) * 1.001))) * 0.5), (0.25 * 0.001));let t3: A = A::sqrt_square_offset(A::scaled_offset(s.ad_value(961), (-1.001), 1.001), ((0.25 * 0.001) * 0.001));s.store_primal_offset_add_scaled_inputs3_offset_mixed_iaa(967, 961, ((0.5 * 1.001) * 0.5), t3, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(961), (-1.001), 1.001), 0.5, t3, 0.5), (-1.0)), ((0.25 * 0.001) * 0.001)), (-0.5), ((1.0 + (0.5 * ((-1.001) * 1.001))) * 0.5), (0.25 * 0.001));let t4: A = A::sqrt_square_offset(A::scaled_offset(s.ad_value(961), (-2.001), 1.001), ((0.25 * 0.001) * 0.001));s.store_primal_offset_add_scaled_inputs3_offset_mixed_iaa(970, 961, ((0.5 * 1.001) * 0.5), t4, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(961), (-2.001), 1.001), 0.5, t4, 0.5), (-1.0)), ((0.25 * 0.001) * 0.001)), (-0.5), ((1.0 + (0.5 * ((-2.001) * 1.001))) * 0.5), (0.25 * 0.001));let t5: A = A::sqrt_square_offset(A::scaled_offset(s.ad_value(962), (-1.001), 1.001), ((0.25 * 0.001) * 0.001));s.store_primal_offset_add_scaled_inputs3_offset_mixed_iaa(968, 962, ((0.5 * 1.001) * 0.5), t5, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(962), (-1.001), 1.001), 0.5, t5, 0.5), (-1.0)), ((0.25 * 0.001) * 0.001)), (-0.5), ((1.0 + (0.5 * ((-1.001) * 1.001))) * 0.5), (0.25 * 0.001));let t6: A = A::sqrt_square_offset(A::scaled_offset(s.ad_value(962), (-2.001), 1.001), ((0.25 * 0.001) * 0.001));s.store_primal_offset_add_scaled_inputs3_offset_mixed_iaa(971, 962, ((0.5 * 1.001) * 0.5), t6, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(962), (-2.001), 1.001), 0.5, t6, 0.5), (-1.0)), ((0.25 * 0.001) * 0.001)), (-0.5), ((1.0 + (0.5 * ((-2.001) * 1.001))) * 0.5), (0.25 * 0.001));s.store_primal_mul_pow_mixed_aii(976, A::pow(A::div(s.ad_value(894), s.ad_value(158)), s.ad_value(969)), 158, 966);s.store_primal_div(979, 976, 893);s.store_primal_mul_pow_mixed_aii(977, A::pow(A::div(s.ad_value(894), s.ad_value(158)), s.ad_value(970)), 158, 967);s.store_primal_div(980, 977, 893);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_28(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1288] {s.store_primal_mul_pow_mixed_aii(978, A::pow(A::div(s.ad_value(894), s.ad_value(158)), s.ad_value(971)), 158, 968);s.store_primal_div(981, 978, 893);}
        if s.b[1288] {s.store_scalar(982, (0.5 * (((1.0 / (1.0 + { let limited_exp_arg = ((2.75 - (p[40] * 1000000000.0)) / 0.78); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })) + 0.5) + ((((((1.0 / (1.0 + { let limited_exp_arg = ((2.75 - (p[40] * 1000000000.0)) / 0.78); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })) - 0.5) * ((1.0 / (1.0 + { let limited_exp_arg = ((2.75 - (p[40] * 1000000000.0)) / 0.78); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })) - 0.5)) + ((0.25 * 0.003) * 0.003))) as f64).sqrt())));}
        if s.b[1288] {s.store_primal_add_div_lhs(983, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(982), A::sub(s.ad_value(960), s.ad_value(882))), A::sub_from_scalar(p[1806], s.ad_value(882)), 982);s.store_primal_div_from_scalar_offset_ad(984, 1.0, A::limited_exp_scaled_input(A::offset(s.ad_value(983), (-0.999)), 1.0 / (0.0001)), 1.0);s.store_scalar(1013, (((((0.5 * p[40]) * p[40]) * 1e18) - ((1.5 * p[40]) * 1000000000.0)) + 2.0));s.store_primal_offset_sub_scaled_inputs(1014, A::offset(s.ad_value(1013), 4.0), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(1013), (-4.0)), ((0.25 * 0.01) * 0.01)), 0.5, (0.25 * 0.01));let t7: A = A::powf(A::sub_from_scalar((p[40] * 1000000000.0), s.ad_value(1014)), p[1893]);let t8: A = A::sqrt_square_offset(A::scale_offset(t7, ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p[1893]))), ((s.v[168]) + ((-18100.0)))), ((0.25 * 0.01) * 0.01));s.store_offset_add_scaled_inputs3_offset(974, t7, ((0.5 * ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p[1893])))) * 0.5), t8, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scale_offset(t7, ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p[1893]))), ((s.v[168]) + (18100.0))), 0.5, t8, 0.5), (-924000.0)), ((0.25 * 9240.0) * 9240.0)), (-0.5), ((924000.0 + (0.5 * ((s.v[168]) + (18100.0)))) * 0.5), (0.25 * 9240.0));let t9: A = A::powf(A::sub_from_scalar((p[40] * 1000000000.0), s.ad_value(1014)), p[1894]);let ta: A = A::sqrt_square_offset(A::scale_offset(t9, ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p[1894]))), 5.5), ((0.25 * 0.01) * 0.01));s.store_primal_offset_add_scaled_inputs3_offset(975, t9, ((0.5 * ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p[1894])))) * 0.5), ta, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scale_offset(t9, ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p[1894]))), 5.5), 0.5, ta, 0.5), (-8.0)), ((0.25 * 0.01) * 0.01)), (-0.5), ((8.0 + (0.5 * 5.5)) * 0.5), (0.25 * 0.01));s.store_scalar(972, ((120.66 * ((4.0) as f64).powf(p[1895])) / (((p[40] * 1000000000.0)) as f64).powf(p[1895])));s.store_scalar(973, ((2.0 * ((4.0) as f64).powf(p[1896])) / (((p[40] * 1000000000.0)) as f64).powf(p[1896])));s.store_scalar(989, ((107.0 * ((4.0) as f64).powf(p[1897])) / (((p[40] * 1000000000.0)) as f64).powf(p[1897])));let tb: A = A::powf(A::sub_from_scalar((p[40] * 1000000000.0), s.ad_value(1014)), p[1898]);let tc: A = A::sqrt_square_offset(A::scale_offset(tb, 0.1, ((0.7) + ((-0.5)))), ((0.25 * 0.01) * 0.01));s.store_primal_offset_add_scaled_inputs3_offset(990, tb, ((0.5 * 0.1) * 0.5), tc, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scale_offset(tb, 0.1, ((0.7) + (0.5))), 0.5, tc, 0.5), (-1.0)), ((0.25 * 0.01) * 0.01)), (-0.5), ((1.0 + (0.5 * ((0.7) + (0.5)))) * 0.5), (0.25 * 0.01));s.store_scalar(991, ((103.0 * ((4.0) as f64).powf(p[1899])) / (((p[40] * 1000000000.0)) as f64).powf(p[1899])));s.store_scalar(992, ((1.5 * ((4.0) as f64).powf(p[1900])) / (((p[40] * 1000000000.0)) as f64).powf(p[1900])));s.store_scalar(993, ((833.0 * ((4.0) as f64).powf(p[1901])) / (((p[40] * 1000000000.0)) as f64).powf(p[1901])));s.store_scalar(994, ((3.4 * ((4.0) as f64).powf(p[1902])) / (((p[40] * 1000000000.0)) as f64).powf(p[1902])));s.store_div_mixed_ia(987, 974, A::pow_from_scalar((p[1852] * 1000000000.0), A::scale(s.ad_value(975), p[1867])));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_29(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1288] {s.store_primal_div_mixed_ia(988, 972, A::pow_from_scalar((p[1852] * 1000000000.0), A::scale(s.ad_value(973), p[1868])));let td: A = A::pow_from_scalar((p[43] * 1000000000.0), A::scale(s.ad_value(975), p[1867]));s.store_add_scaled_inputs4_mixed_iaia(985, 888, 0.5, A::div(s.ad_value(974), td), (p[1865] * 0.5), 987, ((-p[1865]) * 0.5), A::sqrt_square_offset(A::add_scaled_inputs3(s.ad_value(888), 1.0, A::div(s.ad_value(974), td), p[1865], s.ad_value(987), (-p[1865])), ((0.25 * 0.01) * 0.01)), 0.5);let te: A = A::pow_from_scalar((p[43] * 1000000000.0), A::scale(s.ad_value(973), p[1868]));s.store_primal_add_scaled_inputs4_mixed_iaia(986, 889, 0.5, A::div(s.ad_value(972), te), (p[1866] * 0.5), 988, ((-p[1866]) * 0.5), A::sqrt_square_offset(A::add_scaled_inputs3(s.ad_value(889), 1.0, A::div(s.ad_value(972), te), p[1866], s.ad_value(988), (-p[1866])), ((0.25 * 0.01) * 0.01)), 0.5);let tf: A = A::pow_from_scalar((p[43] * 1000000000.0), A::scale(s.ad_value(990), p[1890]));let t10: A = A::powf(A::scale_offset(tf, 5.0, 1.0), 0.5);s.store_primal_scaled_add_sqrt_square_offset_ad(995, A::div(s.ad_value(989), t10), ((0.25 * 0.1) * 0.1), 0.5);let t11: A = A::pow_from_scalar((p[1852] * 1000000000.0), A::scale(s.ad_value(990), p[1890]));let t12: A = A::powf(A::scale_offset(t11, 5.0, 1.0), 0.5);s.store_primal_scaled_add_sqrt_square_offset_ad(996, A::div(s.ad_value(989), t12), ((0.25 * 0.1) * 0.1), 0.5);s.store_primal_add_scaled_inputs3_indices(997, 890, 1.0, 995, p[1887], 996, (-p[1887]));let t13: A = A::pow_from_scalar((p[43] * 1000000000.0), A::scale(s.ad_value(992), p[1891]));let t14: A = A::powf(A::scale_offset(t13, 5.0, 1.0), 0.5);s.store_primal_scaled_add_sqrt_square_offset_ad(998, A::div(s.ad_value(991), t14), ((0.25 * 0.1) * 0.1), 0.5);let t15: A = A::pow_from_scalar((p[1852] * 1000000000.0), A::scale(s.ad_value(992), p[1891]));let t16: A = A::powf(A::scale_offset(t15, 5.0, 1.0), 0.5);s.store_primal_scaled_add_sqrt_square_offset_ad(999, A::div(s.ad_value(991), t16), ((0.25 * 0.1) * 0.1), 0.5);s.store_primal_add_scaled_inputs3_indices(1000, 891, 1.0, 998, p[1888], 999, (-p[1888]));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_30(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1288] {let t17: A = A::pow_from_scalar((p[43] * 1000000000.0), A::scale(s.ad_value(994), p[1892]));let t18: A = A::powf(A::scale_offset(t17, 5.0, 1.0), 0.5);s.store_primal_scaled_add_sqrt_square_offset_ad(1001, A::div(s.ad_value(993), t18), ((0.25 * 0.1) * 0.1), 0.5);let t19: A = A::pow_from_scalar((p[1852] * 1000000000.0), A::scale(s.ad_value(994), p[1892]));let t1a: A = A::powf(A::scale_offset(t19, 5.0, 1.0), 0.5);s.store_primal_scaled_add_sqrt_square_offset_ad(1002, A::div(s.ad_value(993), t1a), ((0.25 * 0.1) * 0.1), 0.5);s.store_primal_add_scaled_inputs3_indices(1003, 892, 1.0, 1001, p[1889], 1002, (-p[1889]));let t1b: A = A::scale_offset(s.ad_value(960), ((0.5) * (2.0)), ((((1.0) + ((-1.0)))) * (2.0)));s.store_primal_mul_product3_mixed_iiaa(1010, 979, 960, A::div(A::pow_from_scalar(3.14, A::scale(s.ad_value(960), 0.5)), A::offset(A::sub(A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(A::exp_scaled_input(A::scale_offset(s.ad_value(960), 0.5, ((1.0) + ((-1.0)))), (-4.6)), 0.0385, A::powi(A::scale_offset(s.ad_value(960), ((0.5) * (2.0)), ((2.0) + ((-3.0)))), 8), 7.5893e-7, A::powi(t1b, 6), 6.9583e-5, A::powi(t1b, 5), (-0.0006583)), 1.0, A::pow4(t1b), 0.0065), 1.0, A::cube(t1b), 0.026), 1.0, A::square(t1b), 0.1371), A::scale_offset(s.ad_value(960), ((0.5) * ((0.194 * 2.0))), ((((1.0) + ((-1.0)))) * ((0.194 * 2.0))))), 0.959)), A::pow(A::scale(s.ad_value(997), 1000000.0), s.ad_value(960)), (1.0 / (2.0) * 1.60219e-19));let t1c: A = A::scale_offset(s.ad_value(961), ((0.5) * (2.0)), ((((1.0) + ((-1.0)))) * (2.0)));s.store_primal_mul_product3_mixed_iiaa(1011, 980, 961, A::div(A::pow_from_scalar(3.14, A::scale(s.ad_value(961), 0.5)), A::offset(A::sub(A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(A::exp_scaled_input(A::scale_offset(s.ad_value(961), 0.5, ((1.0) + ((-1.0)))), (-4.6)), 0.0385, A::powi(A::scale_offset(s.ad_value(961), ((0.5) * (2.0)), ((2.0) + ((-3.0)))), 8), 7.5893e-7, A::powi(t1c, 6), 6.9583e-5, A::powi(t1c, 5), (-0.0006583)), 1.0, A::pow4(t1c), 0.0065), 1.0, A::cube(t1c), 0.026), 1.0, A::square(t1c), 0.1371), A::scale_offset(s.ad_value(961), ((0.5) * ((0.194 * 2.0))), ((((1.0) + ((-1.0)))) * ((0.194 * 2.0))))), 0.959)), A::pow(A::scale(s.ad_value(1000), 1000000.0), s.ad_value(961)), (1.0 / (2.0) * 1.60219e-19));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_31(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        if s.b[1288] {let t1d: A = A::scale_offset(s.ad_value(962), ((0.5) * (2.0)), ((((1.0) + ((-1.0)))) * (2.0)));s.store_primal_mul_product3_mixed_iiaa(1012, 981, 962, A::div(A::pow_from_scalar(3.14, A::scale(s.ad_value(962), 0.5)), A::offset(A::sub(A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(A::exp_scaled_input(A::scale_offset(s.ad_value(962), 0.5, ((1.0) + ((-1.0)))), (-4.6)), 0.0385, A::powi(A::scale_offset(s.ad_value(962), ((0.5) * (2.0)), ((2.0) + ((-3.0)))), 8), 7.5893e-7, A::powi(t1d, 6), 6.9583e-5, A::powi(t1d, 5), (-0.0006583)), 1.0, A::pow4(t1d), 0.0065), 1.0, A::cube(t1d), 0.026), 1.0, A::square(t1d), 0.1371), A::scale_offset(s.ad_value(962), ((0.5) * ((0.194 * 2.0))), ((((1.0) + ((-1.0)))) * ((0.194 * 2.0))))), 0.959)), A::pow(A::scale(s.ad_value(1003), 1000000.0), s.ad_value(962)), (1.0 / (2.0) * 1.60219e-19));}
        s.b[1289] = (p[58] == 1.0);s.store_scalar(1289, if s.b[1289] { 1.0 } else { 0.0 });
        if s.b[1289] {s.store_primal_offset_scaled(707, 707, 1.0 / (({ let limited_exp_arg = (((p[890] * 1000000000.0) - (p[40] * 1000000000.0)) / p[891]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } } + 1.0)), (((((-p[889])) * (1.0 / (({ let limited_exp_arg = (((p[890] * 1000000000.0) - (p[40] * 1000000000.0)) / p[891]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } } + 1.0))))) + (p[889])));}
        if s.b[1289] {s.store_offset(1024, 807, (((-p[892])) + ((-((p[893] * 1000000000.0) * p[894])))));}
        if s.b[1289] {s.store_scaled_offset(1025, 1024, ((p[40] * 1000000000.0) * p[894]), 1.0 / ((1.0 + { let limited_exp_arg = (((p[895] * 1000000000.0) - (p[40] * 1000000000.0)) / p[896]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })));}
        if s.b[1289] {s.store_add_scaled_inputs3_offset_mixed_iia(807, 1025, 0.5, 807, 0.5, A::sqrt_square_offset(A::sub(A::offset(s.ad_value(1025), p[892]), A::offset(s.ad_value(807), 0.2)), ((0.25 * 0.6) * 0.6)), (-0.5), ((p[892] + 0.2) * 0.5));}
        if s.b[1289] {s.store_add_scaled_inputs3_offset_indices(1026, 811, (-(370.0 * 1.0 / ((((p[40] * 1000000000.0)) as f64).powf(p[898])))), 811, (-1.0 / ((1.0 + { let limited_exp_arg = (((p[40] * 1000000000.0) - (p[899] * 1000000000.0)) / p[900]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), 811, 1.0, (((p[897]) * ((370.0 * 1.0 / ((((p[40] * 1000000000.0)) as f64).powf(p[898]))))) + ((p[897]) * (1.0 / ((1.0 + { let limited_exp_arg = (((p[40] * 1000000000.0) - (p[899] * 1000000000.0)) / p[900]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))))));}
        if s.b[1289] {s.store_scaled_sub_offset_sqrt_square_offset(811, 1026, p[897], (-p[897]), ((0.25 * 0.2) * 0.2), 0.5);s.store_scalar(1027, (p[43] / (p[43] + p[40])));s.store_scalar(1028, ((((p[905] * p[40]) * p[40]) * 1e18) - (p[906] * 0.001)));s.store_primal_scaled_add_mixed_ia(1029, 1028, A::powf(A::offset(A::square(s.ad_value(1028)), ((((((4.0 * p[906]) * 0.001) * (p[905] + 0.24)) * p[40]) * p[40]) * 1e18)), 0.5), 1.0 / (((((2.0 * (p[905] + 0.24)) * p[40]) * p[40]) * 1e18)));s.store_primal_scaled_sub_offset_sqrt_square_offset_ad(1030, A::div_scalar_offset_denominator(0.0001, s.ad_value(1029), (((-0.8208)) + ((-(p[907] * 1e-5)))), 1.0), 1.0, (-1.0), ((0.25 * 0.06) * 0.06), 0.5);s.store_mul_ad_product_lhs_mixed_ia(704, 704, A::add(s.ad_value(1027), A::scale_offset(s.ad_value(1027), (-p[904]), p[904])), 1030);s.store_add_mixed_ai(812, A::scale_offset(s.ad_value(812), (-(((0.5 * (((p[902] * 1000000000.0) - (p[40] * 1000000000.0)) + ((((((p[902] * 1000000000.0) - (p[40] * 1000000000.0)) * ((p[902] * 1000000000.0) - (p[40] * 1000000000.0))) + 0.25)) as f64).sqrt()))) as f64).powf(p[903])), ((p[901]) * ((((0.5 * (((p[902] * 1000000000.0) - (p[40] * 1000000000.0)) + ((((((p[902] * 1000000000.0) - (p[40] * 1000000000.0)) * ((p[902] * 1000000000.0) - (p[40] * 1000000000.0))) + 0.25)) as f64).sqrt()))) as f64).powf(p[903])))), 812);}
        s.b[1290] = ((p[74] != 0.0) && (p[1791] > 0.0));s.store_scalar(1290, if s.b[1290] { 1.0 } else { 0.0 });
        if s.b[1290] {s.store_offset_voltage(116, ctx, nodes, Some(4), None, ((ctx_temp) + (p[22])));}
        if (!s.b[1290]) {s.store_scalar(116, (ctx_temp + p[22]));}
        s.store_div(229, 116, 228);s.store_offset(230, 229, (-1.0));s.store_sub(232, 116, 228);s.store_scale(179, 116, 8.617087e-5);s.store_primal_scale(180, 228, 8.617087e-5);s.store_scalar(121, p[1786]);s.b[1291] = (p[80] != 0.0);s.store_scalar(1291, if s.b[1291] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_32(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1291] {s.store_scaled_add_offset_sqrt_square_offset(119, 116, s.v[121], (-s.v[121]), ((0.25 * p[1788]) * p[1788]), 0.5);s.store_scaled_add_sqrt_square_offset_ad(120, A::scaled_offset(s.ad_value(116), (-p[1787]), (-p[1790])), ((0.25 * p[1789]) * p[1789]), 0.5);}
        s.b[1292] = (p[80] == 1.0);s.store_scalar(1292, if s.b[1292] { 1.0 } else { 0.0 });
        if (s.b[1291] && s.b[1292]) {s.store_scaled_add_offset_sqrt_square_offset(169, 228, s.v[121], (-s.v[121]), ((0.25 * p[1788]) * p[1788]), 0.5);s.store_scaled_add_sqrt_square_offset_ad(170, A::scaled_offset(s.ad_value(228), (-p[1787]), (-p[1790])), ((0.25 * p[1789]) * p[1789]), 0.5);}
        s.b[1293] = (s.v[228] > s.v[121]);s.store_scalar(1293, if s.b[1293] { 1.0 } else { 0.0 });
        if ((s.b[1291] && s.b[1292]) && s.b[1293]) {s.store_add_mixed_ai(171, A::add_scaled_inputs4(s.ad_value(119), 1.0, s.ad_value(120), 1.0, s.ad_value(169), -1.0, s.ad_value(170), -1.0), 228);}
        if ((s.b[1291] && s.b[1292]) && (!s.b[1293])) {s.store_add_scaled_inputs4_offset_indices(171, 119, 1.0, 120, 1.0, 169, -1.0, 170, -1.0, s.v[121]);}
        if (s.b[1291] && s.b[1292]) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(118, 116, 0.5, 171, 0.5, 116, 171, ((0.25 * 0.2) * 0.2), 0.5);}
        s.b[1294] = (s.v[121] > 210.0);s.store_scalar(1294, if s.b[1294] { 1.0 } else { 0.0 });
        if ((s.b[1291] && (!s.b[1292])) && s.b[1294]) {s.store_scalar(121, 210.0);}
        if (s.b[1291] && (!s.b[1292])) {s.store_offset_scaled_ad(312, A::tanh_scaled_input(A::offset(s.ad_value(116), (-210.0)), 0.5), 0.5, 0.5);s.store_sub_from_scalar(313, 1.0, 312);}
        s.b[1295] = (s.v[228] > 210.0);s.store_scalar(1295, if s.b[1295] { 1.0 } else { 0.0 });
        if ((s.b[1291] && (!s.b[1292])) && s.b[1295]) {s.store_scaled_add_ad(169, A::offset(s.ad_value(121), 210.0), A::sqrt_square_offset(A::sub_from_scalar(210.0, s.ad_value(121)), ((0.25 * p[1788]) * p[1788])), 0.5);s.store_scalar(170, (0.5 * (((-p[1790]) * (210.0 - p[1787])) + ((((((-p[1790]) * (210.0 - p[1787])) * ((-p[1790]) * (210.0 - p[1787]))) + ((0.25 * p[1789]) * p[1789]))) as f64).sqrt())));s.store_add_scaled_inputs4_offset_indices(171, 119, 1.0, 120, 1.0, 169, -1.0, 170, -1.0, 210.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(118, 116, 0.5, 171, 0.5, 116, 171, ((0.25 * 0.2) * 0.2), 0.5);}
        if ((s.b[1291] && (!s.b[1292])) && (!s.b[1295])) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(169, 228, 0.5, 121, 0.5, 228, 121, ((0.25 * p[1788]) * p[1788]), 0.5);s.store_scaled_add_sqrt_square_offset_ad(170, A::scaled_offset(s.ad_value(228), (-p[1787]), (-p[1790])), ((0.25 * p[1789]) * p[1789]), 0.5);}
        s.b[1296] = (s.v[228] > s.v[121]);s.store_scalar(1296, if s.b[1296] { 1.0 } else { 0.0 });
        if (((s.b[1291] && (!s.b[1292])) && (!s.b[1295])) && s.b[1296]) {s.store_add_mixed_ai(171, A::add_scaled_inputs4(s.ad_value(119), 1.0, s.ad_value(120), 1.0, s.ad_value(169), -1.0, s.ad_value(170), -1.0), 228);}
        if (((s.b[1291] && (!s.b[1292])) && (!s.b[1295])) && (!s.b[1296])) {s.store_add_mixed_ai(171, A::add_scaled_inputs4(s.ad_value(119), 1.0, s.ad_value(120), 1.0, s.ad_value(169), -1.0, s.ad_value(170), -1.0), 121);}
        if ((s.b[1291] && (!s.b[1292])) && (!s.b[1295])) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(172, 116, 0.5, 171, 0.5, 116, 171, ((0.25 * 0.2) * 0.2), 0.5);s.store_add_scaled_products_indices(118, 313, 172, 1.0, 312, 116, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_33(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1291] && (!s.b[1292])) {s.store_scaled_sub_offset_sqrt_square_offset(117, 116, 210.0, (-210.0), ((0.25 * 0.2) * 0.2), 0.5);s.store_add_scaled_inputs3_offset_mixed_iia(233, 117, 1.0, 228, (-0.5), A::sqrt_square_offset(A::offset(s.ad_value(228), (-210.0)), ((0.25 * 0.2) * 0.2)), (-(-0.5)), ((-0.5) * 210.0));s.store_div_scaled_offset_numerator_indices(234, 117, 1.0, (-210.0), 228, 1.0);}
        if s.b[1291] {s.store_scale(182, 118, 8.617087e-5);}
        s.store_sub_from_scalar_ad(146, p[106], A::div_scaled_product_offset_denominator(s.ad_value(116), s.ad_value(116), p[1718], s.ad_value(116), p[1719], 1.0));s.store_primal_sub_from_scalar_ad(147, p[106], A::div_scaled_product_offset_denominator(s.ad_value(228), s.ad_value(228), p[1718], s.ad_value(228), p[1719], 1.0));s.store_mul_scaled_sqrt_scaled_input_rhs(169, 116, 1.0 / (300.15), 116, 1.0 / (300.15));s.store_mul_scaled_limited_exp_ad_rhs(141, 169, p[105], A::sub_from_scalar((p[106] / ((2.0 * 8.617087e-5) * 300.15)), A::div_scaled_inputs(s.ad_value(146), 1.0, s.ad_value(179), 2.0)));s.b[1297] = (p[80] == 0.0);s.store_scalar(1297, if s.b[1297] { 1.0 } else { 0.0 });
        if s.b[1297] {s.store_scale(148, 169, p[107]);}
        if (!s.b[1297]) {s.store_mul_scaled_sqrt_scaled_input_rhs(148, 118, (1.0 / (300.15) * p[107]), 118, 1.0 / (300.15));}
        if (!s.b[1297]) {
            s.store_sub_ad(142, A::offset({
                if (!((p[105] * s.v[169]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((p[105] * s.v[169]) > 1e-38) {
                            A::ln_scaled_input(s.ad_value(169), p[105])
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, (p[106] / ((2.0 * 8.617087e-5) * 300.15))), A::div_scaled_inputs(s.ad_value(146), 1.0, s.ad_value(179), 2.0));
        }
        if (!(((1.0 + (s.v[859] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
            s.store_scaled_add_sqrt_square_offset_ad(235, A::offset(A::mul(s.ad_value(859), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001), 0.5);
        } else {
            if (((1.0 + (s.v[859] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                s.store_div_from_scalar_offset_product(235, ((-0.001) * 0.001), 859, 232, ((1.0) + ((-1e-6))));
            } else {
                s.store_scalar(235, 0.0);
            }
        }
        s.store_scale(389, 179, 1.60219e-19);s.store_div_from_scalar_ad(168, (1.05457e-34 * 3.141592653589793), A::div_scaled_inputs(s.ad_value(894), 2.0, s.ad_value(895), 1.0));s.store_scaled_square(377, 168, 1.0 / ((2.0 * s.v[381])));s.store_scaled_square(378, 168, 1.0 / ((2.0 * s.v[382])));s.store_scale(379, 377, 4.0);s.store_scale(380, 378, 4.0);s.store_scalar(169, ((s.v[385] * s.v[384]) / (s.v[386] * s.v[383])));s.store_offset_scaled_ad(387, A::limited_exp(A::div_scaled_inputs2(s.ad_value(377), 1.0, s.ad_value(378), (-1.0), s.ad_value(389), 1.0)), s.v[169], 1.0);s.store_add_scaled_inputs3_mixed_iaa(388, 387, 1.0, A::limited_exp(A::div_scaled_inputs2(s.ad_value(377), 1.0, s.ad_value(379), (-1.0), s.ad_value(389), 1.0)), 1.0, A::limited_exp(A::div_scaled_inputs2(s.ad_value(377), 1.0, s.ad_value(380), (-1.0), s.ad_value(389), 1.0)), s.v[169]);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_34(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_mul_scale_offset_mixed_ia(170, 179, {
            if (!((((((s.v[386] * s.v[383]) / (((3.141592653589793 * 1.05457e-34) * 1.05457e-34) * s.v[148])) * s.v[389]) / ((2.0 * s.v[894]) / s.v[895])) * s.v[388]) > 1e-38)) {
                A::neg(A::constant(87.498233534))
            } else {
                {
                    if ((((((s.v[386] * s.v[383]) / (((3.141592653589793 * 1.05457e-34) * 1.05457e-34) * s.v[148])) * s.v[389]) / ((2.0 * s.v[894]) / s.v[895])) * s.v[388]) > 1e-38) {
                        A::ln(A::mul(A::div_scaled_value_by_product(s.ad_value(389), (s.v[386] * s.v[383]), A::scale(s.ad_value(148), ((3.141592653589793 * 1.05457e-34) * 1.05457e-34)), A::div_scaled_inputs(s.ad_value(894), 2.0, s.ad_value(895), 1.0), 1.0), s.ad_value(388)))
                    } else {
                        A::constant(0.0)
                    }
                }
            }
        }, -1.0, 0.0);s.store_mul_add_scaled_inputs_rhs_indices(375, 654, 377, 6.241457005723417e18, 170, 1.0);s.store_ln(418, 229);s.b[1298] = (p[80] == 0.0);s.store_scalar(1298, if s.b[1298] { 1.0 } else { 0.0 });
        if s.b[1298] {s.store_mul_exp_mixed_ia(169, 704, A::mul(s.ad_value(836), s.ad_value(418)));s.store_add_scaled_inputs4_offset_mixed_iiaa(413, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));}
        s.b[1299] = (p[66] == 1.0);s.store_scalar(1299, if s.b[1299] { 1.0 } else { 0.0 });
        if (s.b[1298] && s.b[1299]) {s.store_mul_exp_mixed_ia(169, 706, A::mul(s.ad_value(845), s.ad_value(418)));s.store_add_scaled_inputs4_offset_mixed_iiaa(321, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));s.copy_ad(417, 321);}
        if s.b[1298] {s.store_add_scaled_inputs4_offset_mixed_iaai(303, 807, 1.0, A::add_scaled_product(s.ad_value(807), 1.0, s.ad_value(823), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(807), 1.0, s.ad_value(823), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(807), (-(4.0 * 1e-6)))), 0.5, 807, (-1.0), (0.5 * (-1e-6)));s.copy_ad(323, 811);}
        s.b[1300] = (p[66] != 0.0);s.store_scalar(1300, if s.b[1300] { 1.0 } else { 0.0 });
        if (s.b[1298] && s.b[1300]) {s.store_add_scaled_inputs4_offset_mixed_iaai(305, 815, 1.0, A::add_scaled_product(s.ad_value(815), 1.0, s.ad_value(825), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(815), 1.0, s.ad_value(825), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(815), (-(4.0 * 1e-6)))), 0.5, 815, (-1.0), (0.5 * (-1e-6)));}
        if s.b[1298] {s.store_mul_exp_mixed_ia(318, 812, A::mul(s.ad_value(830), s.ad_value(418)));}
        s.b[1301] = (p[66] != 0.0);s.store_scalar(1301, if s.b[1301] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_35(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1298] && s.b[1301]) {s.store_mul_exp_mixed_ia(320, 818, A::mul(s.ad_value(844), s.ad_value(418)));}
        if s.b[1298] {s.store_mul_exp_mixed_ia(317, 814, A::mul(s.ad_value(834), s.ad_value(418)));}
        if s.b[1298] {
            if (!(((1.0 + (s.v[854] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                s.store_scaled_add_sqrt_square_offset_ad(194, A::offset(A::mul(s.ad_value(854), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001), 0.5);
            } else {
                if (((1.0 + (s.v[854] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                    s.store_div_from_scalar_offset_product(194, ((-0.001) * 0.001), 854, 232, ((1.0) + ((-1e-6))));
                } else {
                    s.store_scalar(194, 0.0);
                }
            }
        }
        s.b[1302] = (p[75] != 0.0);s.store_scalar(1302, if s.b[1302] { 1.0 } else { 0.0 });
        if (s.b[1298] && s.b[1302]) {s.store_add_scaled_inputs4_offset_mixed_iaai(332, 679, 1.0, A::add_scaled_product(s.ad_value(679), 1.0, s.ad_value(849), s.ad_value(232), -1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(679), 1.0, s.ad_value(849), s.ad_value(232), -1.0), (-1e-6))), 1.0, s.ad_value(679), (-(4.0 * 1e-6)))), 0.5, 679, (-1.0), (0.5 * (-1e-6)));}
        if (s.b[1298] && (!s.b[1302])) {
            s.store_mul_mixed_ia(332, 679, {
                            if (!(((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.b[1303] = (p[66] != 0.0);s.store_scalar(1303, if s.b[1303] { 1.0 } else { 0.0 });s.b[1304] = (p[75] != 0.0);s.store_scalar(1304, if s.b[1304] { 1.0 } else { 0.0 });
        if ((s.b[1298] && s.b[1303]) && s.b[1304]) {s.store_add_scaled_inputs4_offset_mixed_iaai(333, 680, 1.0, A::add_scaled_product(s.ad_value(680), 1.0, s.ad_value(851), s.ad_value(232), -1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(680), 1.0, s.ad_value(851), s.ad_value(232), -1.0), (-1e-6))), 1.0, s.ad_value(680), (-(4.0 * 1e-6)))), 0.5, 680, (-1.0), (0.5 * (-1e-6)));}
        if ((s.b[1298] && s.b[1303]) && (!s.b[1304])) {
            s.store_mul_mixed_ia(333, 680, {
                            if (!(((1.0 + ((-s.v[851]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(851), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul_scaled_lhs(s.ad_value(851), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + ((-s.v[851]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_scaled_lhs(s.ad_value(851), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_36(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1305] = (s.v[333] < 1000.0);s.store_scalar(1305, if s.b[1305] { 1.0 } else { 0.0 });
        if ((s.b[1298] && s.b[1303]) && s.b[1305]) {s.store_scalar(333, 1000.0);}
        s.b[1306] = (p[67] == 1.0);s.store_scalar(1306, if s.b[1306] { 1.0 } else { 0.0 });
        if (s.b[1298] && s.b[1306]) {s.store_mul_exp_mixed_ia(169, 705, A::mul(s.ad_value(839), s.ad_value(418)));s.store_add_scaled_inputs4_offset_mixed_iiaa(414, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));s.store_add_scaled_inputs4_offset_mixed_iaai(304, 808, 1.0, A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(826), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(808), 1.0, s.ad_value(826), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(808), (-(4.0 * 1e-6)))), 0.5, 808, (-1.0), (0.5 * (-1e-6)));s.store_mul_exp_mixed_ia(319, 813, A::mul(s.ad_value(832), s.ad_value(418)));}
        s.b[1307] = (p[75] != 0.0);s.store_scalar(1307, if s.b[1307] { 1.0 } else { 0.0 });
        if (s.b[1298] && s.b[1307]) {s.store_add_scaled_inputs4_offset_mixed_iaai(334, 698, 1.0, A::add_scaled_product(s.ad_value(698), 1.0, s.ad_value(849), s.ad_value(232), -1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(698), 1.0, s.ad_value(849), s.ad_value(232), -1.0), (-1e-6))), 1.0, s.ad_value(698), (-(4.0 * 1e-6)))), 0.5, 698, (-1.0), (0.5 * (-1e-6)));}
        if (s.b[1298] && (!s.b[1307])) {
            s.store_mul_mixed_ia(334, 698, {
                            if (!(((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.b[1308] = (p[66] != 0.0);s.store_scalar(1308, if s.b[1308] { 1.0 } else { 0.0 });s.b[1309] = (p[75] != 0.0);s.store_scalar(1309, if s.b[1309] { 1.0 } else { 0.0 });
        if ((s.b[1298] && s.b[1308]) && s.b[1309]) {s.store_add_scaled_inputs4_offset_mixed_iaai(335, 699, 1.0, A::add_scaled_product(s.ad_value(699), 1.0, s.ad_value(849), s.ad_value(232), -1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(699), 1.0, s.ad_value(849), s.ad_value(232), -1.0), (-1e-6))), 1.0, s.ad_value(699), (-(4.0 * 1e-6)))), 0.5, 699, (-1.0), (0.5 * (-1e-6)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_37(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1298] && s.b[1308]) && (!s.b[1309])) {
            s.store_mul_mixed_ia(335, 699, {
                            if (!(((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + ((-s.v[849]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_scaled_lhs(s.ad_value(849), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.b[1310] = (s.v[335] < 1000.0);s.store_scalar(1310, if s.b[1310] { 1.0 } else { 0.0 });
        if ((s.b[1298] && s.b[1308]) && s.b[1310]) {s.store_scalar(335, 1000.0);}
        s.b[1311] = (p[75] != 0.0);s.store_scalar(1311, if s.b[1311] { 1.0 } else { 0.0 });
        if (s.b[1298] && s.b[1311]) {s.store_add_scaled_inputs4_offset_mixed_iaai(336, 702, 1.0, A::add_scaled_product(s.ad_value(702), 1.0, s.ad_value(850), s.ad_value(232), -1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(702), 1.0, s.ad_value(850), s.ad_value(232), -1.0), (-1e-6))), 1.0, s.ad_value(702), (-(4.0 * 1e-6)))), 0.5, 702, (-1.0), (0.5 * (-1e-6)));}
        if (s.b[1298] && (!s.b[1311])) {
            s.store_mul_mixed_ia(336, 702, {
                            if (!(((1.0 + ((-s.v[850]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul_scaled_lhs(s.ad_value(850), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul_scaled_lhs(s.ad_value(850), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + ((-s.v[850]) * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul_scaled_lhs(s.ad_value(850), -1.0, s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        if s.b[1298] {
            s.store_offset_ad(337, {
                if (!(((s.v[790] * (1.0 + (p[450] * s.v[232]))) - 2.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(790), A::scale_offset(s.ad_value(232), p[450], 1.0)), (-2.0)), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(790), A::scale_offset(s.ad_value(232), p[450], 1.0)), (-2.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((s.v[790] * (1.0 + (p[450] * s.v[232]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(790), A::scale_offset(s.ad_value(232), p[450], 1.0)), (-2.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 2.0);
        }
        s.b[1312] = (p[66] != 0.0);s.store_scalar(1312, if s.b[1312] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_38(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1298] && s.b[1312]) {
            s.store_offset_ad(338, {
                if (!(((s.v[791] * (1.0 + (p[452] * s.v[232]))) - 2.0) < ((-10000.0) * 0.001))) {
                    A::add_scaled_inputs(A::offset(A::mul(s.ad_value(791), A::scale_offset(s.ad_value(232), p[452], 1.0)), (-2.0)), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(791), A::scale_offset(s.ad_value(232), p[452], 1.0)), (-2.0)), ((4.0 * 0.001) * 0.001)), 0.5)
                } else {
                    {
                        if (((s.v[791] * (1.0 + (p[452] * s.v[232]))) - 2.0) < ((-10000.0) * 0.001)) {
                            A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(791), A::scale_offset(s.ad_value(232), p[452], 1.0)), (-2.0), 1.0)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, 2.0);
        }
        if s.b[1298] {s.copy_ad(660, 657);s.copy_ad(797, 792);s.store_mul_add_mixed_iia(231, 230, 858, A::div_from_scalar(p[1720], s.ad_value(153)));}
        s.b[1313] = (p[80] == 1.0);s.store_scalar(1313, if s.b[1313] { 1.0 } else { 0.0 });
        if ((!s.b[1298]) && s.b[1313]) {s.store_mul_exp_mixed_ia(169, 704, A::mul(A::add_scaled_product(s.ad_value(836), 1.0, s.ad_value(837), s.ad_value(229), 1.0), s.ad_value(418)));s.store_add_scaled_inputs4_offset_mixed_iiaa(413, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(838), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));}
        s.b[1314] = (p[66] == 1.0);s.store_scalar(1314, if s.b[1314] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && s.b[1313]) && s.b[1314]) {s.store_mul_exp_mixed_ia(169, 706, A::mul(A::add_scaled_product(s.ad_value(845), 1.0, s.ad_value(837), s.ad_value(229), 1.0), s.ad_value(418)));s.store_add_scaled_inputs4_offset_mixed_iiaa(321, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(846), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));s.copy_ad(417, 321);}
        if ((!s.b[1298]) && s.b[1313]) {s.store_mul_exp_mixed_ia(303, 807, A::mul(A::add_scaled_product(s.ad_value(823), 1.0, s.ad_value(824), s.ad_value(229), 1.0), s.ad_value(418)));}
        s.b[1315] = (p[66] != 0.0);s.store_scalar(1315, if s.b[1315] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && s.b[1313]) && s.b[1315]) {s.store_mul_exp_mixed_ia(305, 815, A::mul(A::add_scaled_product(s.ad_value(825), 1.0, s.ad_value(824), s.ad_value(229), 1.0), s.ad_value(418)));}
        if ((!s.b[1298]) && s.b[1313]) {s.store_mul_exp_mixed_ia(318, 812, A::mul(A::add_scaled_product(s.ad_value(830), 1.0, s.ad_value(831), s.ad_value(229), 1.0), s.ad_value(418)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_39(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[1316] = (p[66] != 0.0);s.store_scalar(1316, if s.b[1316] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && s.b[1313]) && s.b[1316]) {s.store_mul_exp_mixed_ia(320, 818, A::mul(A::add_scaled_product(s.ad_value(844), 1.0, s.ad_value(831), s.ad_value(229), 1.0), s.ad_value(418)));}
        if ((!s.b[1298]) && s.b[1313]) {s.store_mul_exp_mixed_ia(317, 814, A::mul(A::add_scaled_inputs(s.ad_value(834), 1.0, s.ad_value(229), p[881]), s.ad_value(418)));s.store_mul_scale_offset_mixed_ia(324, 325, A::limited_exp(A::mul(s.ad_value(326), s.ad_value(230))), 1.0, (-1.0));s.store_mul_scale_offset_mixed_ia(327, 328, A::limited_exp(A::mul(s.ad_value(329), s.ad_value(230))), 1.0, (-1.0));s.store_offset(330, 324, 0.5);s.store_offset(331, 327, 0.5);}
        s.b[1317] = (p[75] != 0.0);s.store_scalar(1317, if s.b[1317] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && s.b[1313]) && s.b[1317]) {s.store_add_scaled_inputs4_offset_mixed_iaai(323, 811, 1.0, A::add_scaled_product(s.ad_value(811), 1.0, s.ad_value(847), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(811), 1.0, s.ad_value(847), s.ad_value(232), 1.0), (-1e-6))), 1.0, s.ad_value(811), (-(4.0 * 1e-6)))), 0.5, 811, (-1.0), (0.5 * (-1e-6)));}
        if (((!s.b[1298]) && s.b[1313]) && (!s.b[1317])) {
            s.store_mul_mixed_ia(323, 811, {
                            if (!(((1.0 + (s.v[847] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001))) {
                                A::add_scaled_inputs(A::offset(A::mul(s.ad_value(847), s.ad_value(232)), ((1.0) + ((-1e-6)))), 0.5, A::sqrt_square_offset(A::offset(A::mul(s.ad_value(847), s.ad_value(232)), ((1.0) + ((-1e-6)))), ((4.0 * 0.001) * 0.001)), 0.5)
                            } else {
                                {
                                    if (((1.0 + (s.v[847] * s.v[232])) - 1e-6) < ((-10000.0) * 0.001)) {
                                        A::div_scalar_offset_denominator(((-0.001) * 0.001), A::mul(s.ad_value(847), s.ad_value(232)), ((1.0) + ((-1e-6))), 1.0)
                                    } else {
                                        A::constant(0.0)
                                    }
                                }
                            }
                        });
        }
        s.b[1318] = (p[67] == 1.0);s.store_scalar(1318, if s.b[1318] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && s.b[1313]) && s.b[1318]) {s.store_mul_exp_mixed_ia(169, 705, A::mul(A::add_scaled_product(s.ad_value(839), 1.0, s.ad_value(840), s.ad_value(229), 1.0), s.ad_value(418)));s.store_add_scaled_inputs4_offset_mixed_iiaa(414, 169, 1.0, 169, (-0.9), A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), 0.5, A::sqrt(A::sub_scaled_inputs(A::square(A::offset(A::add_scaled_product(s.ad_value(169), (-(-0.9)), s.ad_value(841), s.ad_value(232), 1.0), (-0.0001))), 1.0, s.ad_value(169), ((-0.9) * (4.0 * 0.0001)))), 0.5, (0.5 * (-0.0001)));s.store_mul_exp_mixed_ia(304, 808, A::mul(A::add_scaled_product(s.ad_value(826), 1.0, s.ad_value(827), s.ad_value(229), 1.0), s.ad_value(418)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_40(
        s: &mut ReactiveScratch,
    ) {
        if (((!s.b[1298]) && s.b[1313]) && s.b[1318]) {s.store_mul_exp_mixed_ia(319, 813, A::mul(A::add_scaled_product(s.ad_value(832), 1.0, s.ad_value(833), s.ad_value(229), 1.0), s.ad_value(418)));}
        s.b[1319] = (s.v[854] == s.v[855]);s.store_scalar(1319, if s.b[1319] { 1.0 } else { 0.0 });
        if (((!s.b[1298]) && s.b[1313]) && s.b[1319]) {s.store_offset_mul(170, 854, 232, 1.0);}
        s.b[1320] = (s.v[856] < s.v[228]);s.store_scalar(1320, if s.b[1320] { 1.0 } else { 0.0 });
        if ((((!s.b[1298]) && s.b[1313]) && (!s.b[1319])) && s.b[1320]) {s.store_offset_mul(195, 854, 232, 1.0);s.store_add_scaled_product_mixed_aia(196, A::offset(A::mul(s.ad_value(855), A::sub(s.ad_value(116), s.ad_value(856))), 1.0), 1.0, 854, A::sub(s.ad_value(856), s.ad_value(228)), 1.0);s.store_mul_sub_by_sub(171, 854, 855, 856, 228);}
        s.b[1321] = (s.v[855] < s.v[854]);s.store_scalar(1321, if s.b[1321] { 1.0 } else { 0.0 });
        if (((((!s.b[1298]) && s.b[1313]) && (!s.b[1319])) && s.b[1320]) && s.b[1321]) {s.store_sub_ad(170, A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5), A::add_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));}
        if (((((!s.b[1298]) && s.b[1313]) && (!s.b[1319])) && s.b[1320]) && (!s.b[1321])) {s.store_sub_ad(170, A::add_scaled_inputs3(s.ad_value(195), 0.5, s.ad_value(196), 0.5, A::sqrt(A::add_scaled_square_product(A::sub(s.ad_value(195), s.ad_value(196)), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), (-0.5)), A::sub_scaled_inputs(s.ad_value(171), 0.5, A::sqrt(A::add_scaled_square_product(s.ad_value(171), 1.0, s.ad_value(857), s.ad_value(857), 0.25)), 0.5));}
        if ((((!s.b[1298]) && s.b[1313]) && (!s.b[1319])) && (!s.b[1320])) {s.store_offset_mul_ad(196, s.ad_value(855), A::sub(s.ad_value(116), s.ad_value(228)), 1.0);s.store_add_scaled_product_mixed_aia(195, A::offset(A::mul(s.ad_value(854), A::sub(s.ad_value(116), s.ad_value(856))), 1.0), 1.0, 855, A::sub(s.ad_value(856), s.ad_value(228)), 1.0);s.store_mul_sub_by_sub(171, 855, 854, 856, 228);}
        s.b[1322] = (s.v[855] < s.v[854]);s.store_scalar(1322, if s.b[1322] { 1.0 } else { 0.0 });
    }
}

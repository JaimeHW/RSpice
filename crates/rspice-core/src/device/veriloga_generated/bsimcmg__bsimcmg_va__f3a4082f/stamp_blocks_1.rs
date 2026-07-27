#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_16(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1259] && (!s.b[1260])) {s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p[3], 933, ((-0.5) * p[3]), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p[3]));s.store_add(451, 938, 947);}
        s.b[1262] = (p[1090] > 0.0);s.store_scalar(1262, if s.b[1262] { 1.0 } else { 0.0 });
        if (s.b[1259] && s.b[1262]) {s.store_primal_offset_div_scaled_offset_numerator_indices(925, 448, 0.2, (p[90] * 0.2), 449, 1.0, 2.3);s.store_scalar(926, 1.05);s.store_primal_abs_ad(927, A::sub(A::offset(s.ad_value(448), p[90]), s.ad_value(449)));s.store_primal_scale(928, 926, p[1087]);s.store_primal_min_offset_rhs(929, 449, 448, p[90]);s.store_primal_div_from_scalar_offset_input(930, p[1087], 925, 1.0);s.store_scalar(931, 1700000000000.0);s.store_primal_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p[1087])));s.store_mul(933, 931, 932);}
        s.b[1263] = (s.v[933] > 80.0);s.store_scalar(1263, if s.b[1263] { 1.0 } else { 0.0 });
        if ((s.b[1259] && s.b[1262]) && s.b[1263]) {s.copy_ad(934, 932);}
        if ((s.b[1259] && s.b[1262]) && (!s.b[1263])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }
        if (s.b[1259] && s.b[1262]) {s.store_primal_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(449), 1.0, s.ad_value(448), p[90], 1.0), A::div_scaled_offset_numerator(s.ad_value(448), 1.0, p[90], s.ad_value(449), 1.0)), 0.5);s.store_primal_mul(936, 927, 935);}
        if (s.b[1259] && s.b[1262]) {
            s.store_primal_scale_ad(937, {
                if (!(((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p[1087]), 1.0 / (p[1087]))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }
        if (s.b[1259] && s.b[1262]) {s.store_scaled_add(938, 934, 937, p[92]);s.store_primal_div(930, 928, 448);s.store_primal_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);s.store_primal_add_mixed_ai(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(448), (2.0 * p[90]), (p[90] * p[90])), 1.0, A::square(s.ad_value(448)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p[90]), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);s.store_primal_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p[90], A::scaled_offset(s.ad_value(930), 2.0, p[90]), 1.0);}
        if (s.b[1259] && s.b[1262]) {
            s.store_primal_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }
        if (s.b[1259] && s.b[1262]) {s.store_primal_mul(943, 925, 926);s.store_primal_sqrt_square_offset(944, 943, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_17(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1259] && s.b[1262]) {s.store_add_mixed_ai(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p[90] * p[90]), s.ad_value(943), s.ad_value(928), (2.0 * p[90])), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p[90], A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);s.store_primal_mul_scale_offset_mixed_ai(945, A::offset(s.ad_value(944), 1.0), 943, p[90], 0.0);}
        if (s.b[1259] && s.b[1262]) {
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
            }, 943, ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.7), 944, 1.0);
        }
        if (s.b[1259] && s.b[1262]) {s.store_scalar(627, 1.2e-12);s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p[92], 933, ((-0.5) * p[92]), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p[92]));s.store_add(452, 938, 947);}
        if (s.b[1259] && (!s.b[1262])) {s.store_primal_offset_div_scaled_offset_numerator_indices(925, 448, 0.2, (p[90] * 0.2), 449, 1.0, 2.3);s.store_scalar(926, 1.05);s.store_primal_abs_ad(927, A::sub(A::offset(s.ad_value(448), p[90]), s.ad_value(449)));s.store_primal_scale(928, 926, p[1087]);s.store_primal_min_offset_rhs(929, 449, 448, p[90]);s.store_primal_div_from_scalar_offset_input(930, p[1087], 925, 1.0);s.store_scalar(931, 1700000000000.0);s.store_primal_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p[1087])));s.store_mul(933, 931, 932);}
        s.b[1264] = (s.v[933] > 80.0);s.store_scalar(1264, if s.b[1264] { 1.0 } else { 0.0 });
        if ((s.b[1259] && (!s.b[1262])) && s.b[1264]) {s.copy_ad(934, 932);}
        if ((s.b[1259] && (!s.b[1262])) && (!s.b[1264])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }
        if (s.b[1259] && (!s.b[1262])) {s.store_primal_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(449), 1.0, s.ad_value(448), p[90], 1.0), A::div_scaled_offset_numerator(s.ad_value(448), 1.0, p[90], s.ad_value(449), 1.0)), 0.5);s.store_primal_mul(936, 927, 935);}
        if (s.b[1259] && (!s.b[1262])) {
            s.store_primal_scale_ad(937, {
                if (!(((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p[1087]), 1.0 / (p[1087]))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }
        if (s.b[1259] && (!s.b[1262])) {s.store_scaled_add(938, 934, 937, p[92]);s.store_primal_div(930, 928, 448);s.store_primal_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_18(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1259] && (!s.b[1262])) {s.store_primal_add_mixed_ai(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(448), (2.0 * p[90]), (p[90] * p[90])), 1.0, A::square(s.ad_value(448)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p[90]), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);s.store_primal_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p[90], A::scaled_offset(s.ad_value(930), 2.0, p[90]), 1.0);}
        if (s.b[1259] && (!s.b[1262])) {
            s.store_primal_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }
        if (s.b[1259] && (!s.b[1262])) {s.store_primal_mul(943, 925, 926);s.store_primal_sqrt_square_offset(944, 943, 1.0);s.store_add_mixed_ai(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p[90] * p[90]), s.ad_value(943), s.ad_value(928), (2.0 * p[90])), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p[90], A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);s.store_primal_mul_scale_offset_mixed_ai(945, A::offset(s.ad_value(944), 1.0), 943, p[90], 0.0);}
        if (s.b[1259] && (!s.b[1262])) {
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
        if (s.b[1259] && (!s.b[1262])) {s.store_scalar(627, 1.2e-12);s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p[92], 933, ((-0.5) * p[92]), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p[92]));s.store_add(452, 938, 947);}
        s.b[1265] = (p[1090] > 0.0);s.store_scalar(1265, if s.b[1265] { 1.0 } else { 0.0 });
        if (s.b[1259] && s.b[1265]) {s.store_scalar(454, 0.0);}
        s.b[1266] = (p[1080] > 0.0);s.store_scalar(1266, if s.b[1266] { 1.0 } else { 0.0 });
        if ((s.b[1259] && (!s.b[1265])) && s.b[1266]) {s.store_scalar(454, ((p[4] - p[3]) * ((p[1080] * p[1084]) + p[1081])));}
        if ((s.b[1259] && (!s.b[1265])) && (!s.b[1266])) {s.store_primal_scale(454, 450, (p[4] - p[3]));}
        if s.b[1259] {s.store_primal_offset_scaled(455, 454, ((p[5]) * ((s.v[144] * 1.0 / (p[1087])))), ((((p[1092]) + (p[1091]))) * ((s.v[144] * 1.0 / (p[1087])))));s.store_add_scaled_inputs3_indices(453, 455, p[59], 451, (p[5] * p[59]), 452, ((p[1103] * (p[5] * 2.0)) * p[59]));s.store_scale(453, 453, (0.0_f64).max((((p[1099] + (p[1100] * p[3])) + (p[1101] * p[4])) + (p[1102] * p[20]))));}
        s.b[1267] = (p[78] == 3.0);s.store_scalar(1267, if s.b[1267] { 1.0 } else { 0.0 });
        if s.b[1267] {s.store_scalar(447, (p[1089] + p[1090]));s.store_scalar(449, (0.5 * (p[4] - p[43])));s.store_primal_max_from_scalar_ad(448, 0.0, A::offset(s.ad_value(449), (-p[90])));s.store_scalar(450, (0.0_f64).max((p[1080] + p[1081])));s.store_scalar(1031, (0.5 * p[41]));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_19(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[1268] = (p[1090] > 0.0);s.store_scalar(1268, if s.b[1268] { 1.0 } else { 0.0 });
        if (s.b[1267] && s.b[1268]) {s.store_scalar(168, (3.467e-11 * (if (!(((1e-7 * p[1088]) / (3.9 * p[1087])) > 1e-38)) { (-87.498233534) } else { (if (((1e-7 * p[1088]) / (3.9 * p[1087])) > 1e-38) { ((((1e-7 * p[1088]) / (3.9 * p[1087]))) as f64).ln() } else { 0.0 }) })));}
        if (s.b[1267] && s.b[1268]) {s.store_scale(169, 450, (0.942 * (s.v[144] * 1.0 / (p[1087]))));s.store_scaled_add(1034, 168, 169, (p[43] + ((p[4] - p[43]) * p[1084])));}
        if (s.b[1267] && (!s.b[1268])) {s.store_primal_offset_div_scaled_offset_numerator_indices(925, 447, 0.2, (p[90] * 0.2), 450, 1.0, 2.3);s.store_scalar(926, 1.05);s.store_primal_abs_ad(927, A::sub(A::offset(s.ad_value(447), p[90]), s.ad_value(450)));s.store_primal_scale(928, 926, p[1087]);s.store_primal_min_offset_rhs(929, 450, 447, p[90]);s.store_primal_div_from_scalar_offset_input(930, p[1087], 925, 1.0);s.store_scalar(931, 1700000000000.0);s.store_primal_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p[1087])));s.store_mul(933, 931, 932);}
        s.b[1269] = (s.v[933] > 80.0);s.store_scalar(1269, if s.b[1269] { 1.0 } else { 0.0 });
        if ((s.b[1267] && (!s.b[1268])) && s.b[1269]) {s.copy_ad(934, 932);}
        if ((s.b[1267] && (!s.b[1268])) && (!s.b[1269])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }
        if (s.b[1267] && (!s.b[1268])) {s.store_primal_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(450), 1.0, s.ad_value(447), p[90], 1.0), A::div_scaled_offset_numerator(s.ad_value(447), 1.0, p[90], s.ad_value(450), 1.0)), 0.5);s.store_primal_mul(936, 927, 935);}
        if (s.b[1267] && (!s.b[1268])) {
            s.store_primal_scale_ad(937, {
                if (!(((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p[1087]), 1.0 / (p[1087]))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }
        if (s.b[1267] && (!s.b[1268])) {s.store_scaled_add(938, 934, 937, p[43]);s.store_primal_div(930, 928, 447);s.store_primal_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);s.store_primal_add_mixed_ai(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(447), (2.0 * p[90]), (p[90] * p[90])), 1.0, A::square(s.ad_value(447)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p[90]), 1.0, s.ad_value(447), s.ad_value(930), 1.0), 447);s.store_primal_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p[90], A::scaled_offset(s.ad_value(930), 2.0, p[90]), 1.0);}
        if (s.b[1267] && (!s.b[1268])) {
            s.store_primal_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }
        if (s.b[1267] && (!s.b[1268])) {s.store_primal_mul(943, 925, 926);s.store_primal_sqrt_square_offset(944, 943, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_20(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1267] && (!s.b[1268])) {s.store_add_mixed_ai(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p[90] * p[90]), s.ad_value(943), s.ad_value(928), (2.0 * p[90])), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p[90], A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);s.store_primal_mul_scale_offset_mixed_ai(945, A::offset(s.ad_value(944), 1.0), 943, p[90], 0.0);}
        if (s.b[1267] && (!s.b[1268])) {
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
        if (s.b[1267] && (!s.b[1268])) {s.store_scalar(627, 1.2e-12);s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p[43], 933, ((-0.5) * p[43]), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p[43]));s.store_add(1034, 938, 947);}
        if s.b[1267] {s.store_primal_offset_div_from_scalar_ad(925, (0.2 * (p[1089] + p[90])), s.ad_value(1031), 2.3);s.store_scalar(926, 1.05);s.store_primal_abs_ad(927, A::sub_from_scalar((p[1089] + p[90]), s.ad_value(1031)));s.store_primal_scale(928, 926, p[1087]);s.store_primal_min_with_scalar(929, 1031, (p[1089] + p[90]));s.store_primal_div_from_scalar_offset_input(930, p[1087], 925, 1.0);s.store_scalar(931, 1700000000000.0);s.store_primal_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p[1087])));s.store_mul(933, 931, 932);}
        s.b[1270] = (s.v[933] > 80.0);s.store_scalar(1270, if s.b[1270] { 1.0 } else { 0.0 });
        if (s.b[1267] && s.b[1270]) {s.copy_ad(934, 932);}
        if (s.b[1267] && (!s.b[1270])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }
        if s.b[1267] {s.store_primal_scale_ad(935, A::min(A::scale(s.ad_value(1031), 1.0 / ((p[1089] + p[90]))), A::div_from_scalar((p[1089] + p[90]), s.ad_value(1031))), 0.5);s.store_primal_mul(936, 927, 935);}
        if s.b[1267] {
            s.store_primal_scale_ad(937, {
                if (!(((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p[1087]), 1.0 / (p[1087]))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }
        if s.b[1267] {s.store_scaled_add(938, 934, 937, p[43]);s.store_primal_scale(930, 928, 1.0 / (p[1089]));s.store_primal_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);s.store_primal_offset_add_scaled_inputs_mixed_ai(940, A::offset(A::mul(A::sqrt(A::scale_offset(s.ad_value(930), (p[1089] * p[1089]), (((p[1089] * p[1089])) + (((p[90] * p[90]) + ((2.0 * p[1089]) * p[90])))))), A::sqrt(A::offset(s.ad_value(930), 1.0))), p[90]), 1.0, 930, p[1089], p[1089]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_21(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1267] {s.store_primal_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p[90], A::scaled_offset(s.ad_value(930), 2.0, p[90]), 1.0);}
        if s.b[1267] {
            s.store_primal_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }
        if s.b[1267] {s.store_primal_mul(943, 925, 926);s.store_primal_sqrt_square_offset(944, 943, 1.0);s.store_add_mixed_ai(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p[90] * p[90]), s.ad_value(943), s.ad_value(928), (2.0 * p[90])), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p[90], A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);s.store_primal_mul_scale_offset_mixed_ai(945, A::offset(s.ad_value(944), 1.0), 943, p[90], 0.0);}
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
        if s.b[1267] {s.store_scalar(627, 1.2e-12);s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p[43], 933, ((-0.5) * p[43]), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p[43]));s.store_add(1035, 938, 947);}
        s.b[1271] = (p[1090] > 0.0);s.store_scalar(1271, if s.b[1271] { 1.0 } else { 0.0 });
        if (s.b[1267] && s.b[1271]) {s.store_primal_offset_div_scaled_offset_numerator_indices(925, 448, 0.2, (p[90] * 0.2), 449, 1.0, 2.3);s.store_scalar(926, 1.05);s.store_primal_abs_ad(927, A::sub(A::offset(s.ad_value(448), p[90]), s.ad_value(449)));s.store_primal_scale(928, 926, p[1087]);s.store_primal_min_offset_rhs(929, 449, 448, p[90]);s.store_primal_div_from_scalar_offset_input(930, p[1087], 925, 1.0);s.store_scalar(931, 1700000000000.0);s.store_primal_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p[1087])));s.store_mul(933, 931, 932);}
        s.b[1272] = (s.v[933] > 80.0);s.store_scalar(1272, if s.b[1272] { 1.0 } else { 0.0 });
        if ((s.b[1267] && s.b[1271]) && s.b[1272]) {s.copy_ad(934, 932);}
        if ((s.b[1267] && s.b[1271]) && (!s.b[1272])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }
        if (s.b[1267] && s.b[1271]) {s.store_primal_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(449), 1.0, s.ad_value(448), p[90], 1.0), A::div_scaled_offset_numerator(s.ad_value(448), 1.0, p[90], s.ad_value(449), 1.0)), 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_22(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1267] && s.b[1271]) {s.store_primal_mul(936, 927, 935);}
        if (s.b[1267] && s.b[1271]) {
            s.store_primal_scale_ad(937, {
                if (!(((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p[1087]), 1.0 / (p[1087]))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }
        if (s.b[1267] && s.b[1271]) {s.store_scaled_add(938, 934, 937, p[40]);s.store_primal_div(930, 928, 448);s.store_primal_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);s.store_primal_add_mixed_ai(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(448), (2.0 * p[90]), (p[90] * p[90])), 1.0, A::square(s.ad_value(448)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p[90]), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);s.store_primal_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p[90], A::scaled_offset(s.ad_value(930), 2.0, p[90]), 1.0);}
        if (s.b[1267] && s.b[1271]) {
            s.store_primal_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }
        if (s.b[1267] && s.b[1271]) {s.store_primal_mul(943, 925, 926);s.store_primal_sqrt_square_offset(944, 943, 1.0);s.store_add_mixed_ai(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p[90] * p[90]), s.ad_value(943), s.ad_value(928), (2.0 * p[90])), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p[90], A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);s.store_primal_mul_scale_offset_mixed_ai(945, A::offset(s.ad_value(944), 1.0), 943, p[90], 0.0);}
        if (s.b[1267] && s.b[1271]) {
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
            }, 943, ((((2.0 * s.v[144]) * ((2.0) as f64).sqrt()) / 3.141592653589793) * 0.7), 944, 1.0);
        }
        if (s.b[1267] && s.b[1271]) {s.store_scalar(627, 1.2e-12);s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p[40], 933, ((-0.5) * p[40]), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p[40]));s.store_add(1036, 938, 947);}
        if (s.b[1267] && (!s.b[1271])) {s.store_primal_offset_div_scaled_offset_numerator_indices(925, 448, 0.2, (p[90] * 0.2), 449, 1.0, 2.3);s.store_scalar(926, 1.05);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_23(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1267] && (!s.b[1271])) {s.store_primal_abs_ad(927, A::sub(A::offset(s.ad_value(448), p[90]), s.ad_value(449)));s.store_primal_scale(928, 926, p[1087]);s.store_primal_min_offset_rhs(929, 449, 448, p[90]);s.store_primal_div_from_scalar_offset_input(930, p[1087], 925, 1.0);s.store_scalar(931, 1700000000000.0);s.store_primal_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p[1087])));s.store_mul(933, 931, 932);}
        s.b[1273] = (s.v[933] > 80.0);s.store_scalar(1273, if s.b[1273] { 1.0 } else { 0.0 });
        if ((s.b[1267] && (!s.b[1271])) && s.b[1273]) {s.copy_ad(934, 932);}
        if ((s.b[1267] && (!s.b[1271])) && (!s.b[1273])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }
        if (s.b[1267] && (!s.b[1271])) {s.store_primal_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(449), 1.0, s.ad_value(448), p[90], 1.0), A::div_scaled_offset_numerator(s.ad_value(448), 1.0, p[90], s.ad_value(449), 1.0)), 0.5);s.store_primal_mul(936, 927, 935);}
        if (s.b[1267] && (!s.b[1271])) {
            s.store_primal_scale_ad(937, {
                if (!(((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p[1087]), 1.0 / (p[1087]))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }
        if (s.b[1267] && (!s.b[1271])) {s.store_scaled_add(938, 934, 937, p[40]);s.store_primal_div(930, 928, 448);s.store_primal_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);s.store_primal_add_mixed_ai(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(448), (2.0 * p[90]), (p[90] * p[90])), 1.0, A::square(s.ad_value(448)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p[90]), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);s.store_primal_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p[90], A::scaled_offset(s.ad_value(930), 2.0, p[90]), 1.0);}
        if (s.b[1267] && (!s.b[1271])) {
            s.store_primal_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }
        if (s.b[1267] && (!s.b[1271])) {s.store_primal_mul(943, 925, 926);s.store_primal_sqrt_square_offset(944, 943, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_24(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1267] && (!s.b[1271])) {s.store_add_mixed_ai(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p[90] * p[90]), s.ad_value(943), s.ad_value(928), (2.0 * p[90])), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p[90], A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);s.store_primal_mul_scale_offset_mixed_ai(945, A::offset(s.ad_value(944), 1.0), 943, p[90], 0.0);}
        if (s.b[1267] && (!s.b[1271])) {
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
        if (s.b[1267] && (!s.b[1271])) {s.store_scalar(627, 1.2e-12);s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p[40], 933, ((-0.5) * p[40]), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p[40]));s.store_add(1036, 938, 947);}
        if s.b[1267] {s.store_primal_offset_div_scaled_offset_numerator_indices(925, 448, 0.2, (p[90] * 0.2), 449, 1.0, 2.3);s.store_scalar(926, 1.05);s.store_primal_abs_ad(927, A::sub(A::offset(s.ad_value(448), p[90]), s.ad_value(449)));s.store_primal_scale(928, 926, p[1087]);s.store_primal_min_offset_rhs(929, 449, 448, p[90]);s.store_primal_div_from_scalar_offset_input(930, p[1087], 925, 1.0);s.store_scalar(931, 1700000000000.0);s.store_primal_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p[1087])));s.store_mul(933, 931, 932);}
        s.b[1274] = (s.v[933] > 80.0);s.store_scalar(1274, if s.b[1274] { 1.0 } else { 0.0 });
        if (s.b[1267] && s.b[1274]) {s.copy_ad(934, 932);}
        if (s.b[1267] && (!s.b[1274])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }
        if s.b[1267] {s.store_primal_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(449), 1.0, s.ad_value(448), p[90], 1.0), A::div_scaled_offset_numerator(s.ad_value(448), 1.0, p[90], s.ad_value(449), 1.0)), 0.5);s.store_primal_mul(936, 927, 935);}
        if s.b[1267] {
            s.store_primal_scale_ad(937, {
                if (!(((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p[1087]), 1.0 / (p[1087]))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }
        if s.b[1267] {s.store_scaled_add(938, 934, 937, p[40]);s.store_primal_div(930, 928, 448);s.store_primal_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_25(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1267] {s.store_primal_add_mixed_ai(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(448), (2.0 * p[90]), (p[90] * p[90])), 1.0, A::square(s.ad_value(448)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p[90]), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);s.store_primal_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p[90], A::scaled_offset(s.ad_value(930), 2.0, p[90]), 1.0);}
        if s.b[1267] {
            s.store_primal_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }
        if s.b[1267] {s.store_primal_mul(943, 925, 926);s.store_primal_sqrt_square_offset(944, 943, 1.0);s.store_add_mixed_ai(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p[90] * p[90]), s.ad_value(943), s.ad_value(928), (2.0 * p[90])), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p[90], A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);s.store_primal_mul_scale_offset_mixed_ai(945, A::offset(s.ad_value(944), 1.0), 943, p[90], 0.0);}
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
        if s.b[1267] {s.store_scalar(627, 1.2e-12);s.store_add_scaled_inputs3_indices(933, 946, 1.0, 942, (-1.0), 627, -1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(947, 946, p[40], 933, ((-0.5) * p[40]), A::add_scaled_square_product(s.ad_value(933), 1.0, s.ad_value(627), s.ad_value(946), 4.0), ((-0.5) * p[40]));s.store_add(1037, 938, 947);s.store_primal_offset_div_scaled_offset_numerator_indices(925, 448, 0.2, (p[90] * 0.2), 449, 1.0, 2.3);s.store_scalar(926, 1.05);s.store_primal_abs_ad(927, A::sub(A::offset(s.ad_value(448), p[90]), s.ad_value(449)));s.store_primal_scale(928, 926, p[1087]);s.store_primal_min_offset_rhs(929, 449, 448, p[90]);s.store_primal_div_from_scalar_offset_input(930, p[1087], 925, 1.0);s.store_scalar(931, 1700000000000.0);s.store_primal_scaled_sub(932, 929, 930, (s.v[144] * 1.0 / (p[1087])));s.store_mul(933, 931, 932);}
        s.b[1275] = (s.v[933] > 80.0);s.store_scalar(1275, if s.b[1275] { 1.0 } else { 0.0 });
        if (s.b[1267] && s.b[1275]) {s.copy_ad(934, 932);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_26(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1267] && (!s.b[1275])) {
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(934, 1.0, 931, {
                            if ((!(s.v[933] > 37.0)) && (!(s.v[933] < (-37.0)))) {
                                A::ln_one_plus_exp(s.ad_value(933))
                            } else {
                                {
                                    if ((!(s.v[933] > 37.0)) && (s.v[933] < (-37.0))) {
                                        A::exp(s.ad_value(933))
                                    } else {
                                        {
                                            if (s.v[933] > 37.0) {
                                                s.ad_value(933)
                                            } else {
                                                A::constant(0.0)
                                            }
                                        }
                                    }
                                }
                            }
                        });
        }
        if s.b[1267] {s.store_primal_scale_ad(935, A::min(A::div_scaled_value_offset_denominator(s.ad_value(449), 1.0, s.ad_value(448), p[90], 1.0), A::div_scaled_offset_numerator(s.ad_value(448), 1.0, p[90], s.ad_value(449), 1.0)), 0.5);s.store_primal_mul(936, 927, 935);}
        if s.b[1267] {
            s.store_primal_scale_ad(937, {
                if (!(((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if (((p[1087] + ((0.5 * 3.141592653589793) * s.v[936])) / p[1087]) > 1e-38) {
                            A::ln_scaled_input(A::scale_offset(s.ad_value(936), (0.5 * 3.141592653589793), p[1087]), 1.0 / (p[1087]))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }, ((s.v[144] * 2.0) / 3.141592653589793));
        }
        if s.b[1267] {s.store_scaled_add(938, 934, 937, p[42]);s.store_primal_div(930, 928, 448);s.store_primal_div_from_scalar_scaled_ad(939, 4.0, A::sqrt_scaled_input(A::offset(s.ad_value(930), 1.0), 2.0), 3.141592653589793);s.store_primal_add_mixed_ai(940, A::add_scaled_product(A::offset(A::mul(A::sqrt(A::add_scaled_offset_product_rhs(A::scale_offset(s.ad_value(448), (2.0 * p[90]), (p[90] * p[90])), 1.0, A::square(s.ad_value(448)), s.ad_value(930), 1.0, 1.0)), A::sqrt(A::offset(s.ad_value(930), 1.0))), p[90]), 1.0, s.ad_value(448), s.ad_value(930), 1.0), 448);s.store_primal_add_scaled_inputs_ad(941, A::sqrt(A::mul_offset_lhs(s.ad_value(930), 1.0, A::offset(s.ad_value(930), 4.0))), p[90], A::scaled_offset(s.ad_value(930), 2.0, p[90]), 1.0);}
        if s.b[1267] {
            s.store_primal_scaled_offset_ad(942, A::mul(s.ad_value(939), {
                if (!((s.v[940] / s.v[941]) > 1e-38)) {
                    A::neg(A::constant(87.498233534))
                } else {
                    {
                        if ((s.v[940] / s.v[941]) > 1e-38) {
                            A::ln(A::div(s.ad_value(940), s.ad_value(941)))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            }), 12.27, s.v[144]);
        }
        if s.b[1267] {s.store_primal_mul(943, 925, 926);s.store_primal_sqrt_square_offset(944, 943, 1.0);s.store_add_mixed_ai(933, A::add_scaled_inputs_product(A::sqrt(A::mul_offset_lhs(A::square(s.ad_value(943)), 1.0, A::add(A::add_scaled_products(s.ad_value(943), s.ad_value(943), (p[90] * p[90]), s.ad_value(943), s.ad_value(928), (2.0 * p[90])), A::mul3(A::offset(A::square(s.ad_value(943)), 1.0), s.ad_value(928), s.ad_value(928))))), 1.0, s.ad_value(943), p[90], A::square(s.ad_value(943)), s.ad_value(928), 1.0), 928);s.store_primal_mul_scale_offset_mixed_ai(945, A::offset(s.ad_value(944), 1.0), 943, p[90], 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_27(
        s: &mut Scratch,
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
    pub(super) fn stamp_transient_block_28(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_offset_sqrt_ad(364, A::offset(A::div(s.ad_value(803), s.ad_value(153)), 1.0), (-1.0));s.store_offset_div_scaled_product_indices(360, 678, 153, 1.0, 351, 1.0, 1e-6);s.b[1285] = (s.v[360] < 40.0);s.store_scalar(1285, if s.b[1285] { 1.0 } else { 0.0 });
        if s.b[1285] {s.store_div_from_scalar_ad(365, 1.0, A::max_with_scalar(A::scale_offset(A::cosh(s.ad_value(360)), p[171], (((((-2.0)) * (p[171]))) + (1.0))), 1e-6));}
        if (!s.b[1285]) {let t0: A = A::limited_exp_scaled_input(s.ad_value(360), -1.0);s.store_div_ad(365, t0, A::max_with_scalar(A::offset(t0, p[171]), 1e-6));}
        s.store_primal_div_scaled_product_indices(396, 640, 894, 1.60219e-19, 893, 1.0);s.b[1286] = (p[60] == 1.0);s.store_scalar(1286, if s.b[1286] { 1.0 } else { 0.0 });
        if s.b[1286] {s.store_scalar(484, 4.97232e-7);s.store_scalar(485, 745669000000.0);}
        if (!s.b[1286]) {s.store_scalar(484, 3.42537e-7);s.store_scalar(485, 1166450000000.0);}
        s.store_scalar(168, (p[1109] * p[1109]));s.store_scale(169, 742, p[1109]);s.store_square(170, 169);s.store_scale_ad(486, A::pow_from_scalar((p[1108] / p[1109]), s.ad_value(741)), 1.0 / (s.v[168]));s.store_div_mixed_ai(487, A::pow(A::div_from_scalar(p[1108], s.ad_value(169)), s.ad_value(741)), 170);s.store_mul3_lhs(463, 158, 484, 487);s.b[1287] = (p[1717] < (-273.15));s.store_scalar(1287, if s.b[1287] { 1.0 } else { 0.0 });
        if s.b[1287] {s.store_scalar(228, 300.15);}
        if (!s.b[1287]) {s.store_scalar(228, (p[1717] + 273.15));}
        s.b[1288] = (p[57] == 1.0);s.store_scalar(1288, if s.b[1288] { 1.0 } else { 0.0 });
        if s.b[1288] {s.store_primal_add_mixed_ai(960, A::scale_offset(s.ad_value(882), (-1.0 / ((1.0 + { let limited_exp_arg = (((p[1827] * 1000000000.0) - (p[43] * 1000000000.0)) / p[1828]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), ((p[1806]) * (1.0 / ((1.0 + { let limited_exp_arg = (((p[1827] * 1000000000.0) - (p[43] * 1000000000.0)) / p[1828]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))))), 882);}
        if s.b[1288] {s.store_primal_add_mixed_ai(961, A::scale_offset(s.ad_value(883), (-1.0 / ((1.0 + { let limited_exp_arg = (((p[1827] * 1000000000.0) - (p[43] * 1000000000.0)) / p[1828]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), ((p[1813]) * (1.0 / ((1.0 + { let limited_exp_arg = (((p[1827] * 1000000000.0) - (p[43] * 1000000000.0)) / p[1828]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))))), 883);}
        if s.b[1288] {s.store_primal_add_mixed_ai(962, A::scale_offset(s.ad_value(884), (-1.0 / ((1.0 + { let limited_exp_arg = (((p[1827] * 1000000000.0) - (p[43] * 1000000000.0)) / p[1828]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), ((p[1820]) * (1.0 / ((1.0 + { let limited_exp_arg = (((p[1827] * 1000000000.0) - (p[43] * 1000000000.0)) / p[1828]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))))), 884);}
        if s.b[1288] {s.store_primal_scaled_add_sqrt_square_offset_ad(963, A::offset(s.ad_value(885), ((-p[1847]) / (1.0 + { let limited_exp_arg = (((p[1850] * 1000000000.0) - (p[43] * 1000000000.0)) / p[1851]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), ((0.25 * 0.001) * 0.001), 0.5);}
        if s.b[1288] {s.store_primal_scaled_add_sqrt_square_offset_ad(964, A::offset(s.ad_value(886), ((-p[1848]) / (1.0 + { let limited_exp_arg = (((p[1850] * 1000000000.0) - (p[43] * 1000000000.0)) / p[1851]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), ((0.25 * 0.001) * 0.001), 0.5);}
        if s.b[1288] {s.store_primal_scaled_add_sqrt_square_offset_ad(965, A::offset(s.ad_value(887), ((-p[1849]) / (1.0 + { let limited_exp_arg = (((p[1850] * 1000000000.0) - (p[43] * 1000000000.0)) / p[1851]); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } }))), ((0.25 * 0.001) * 0.001), 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_29(
        s: &mut Scratch,
    ) {
        if s.b[1288] {let t1: A = A::sqrt_square_offset(A::scaled_offset(s.ad_value(960), (-1.001), 1.001), ((0.25 * 0.001) * 0.001));s.store_primal_offset_add_scaled_inputs3_offset_mixed_iaa(966, 960, ((0.5 * 1.001) * 0.5), t1, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(960), (-1.001), 1.001), 0.5, t1, 0.5), (-1.0)), ((0.25 * 0.001) * 0.001)), (-0.5), ((1.0 + (0.5 * ((-1.001) * 1.001))) * 0.5), (0.25 * 0.001));let t2: A = A::sqrt_square_offset(A::scaled_offset(s.ad_value(960), (-2.001), 1.001), ((0.25 * 0.001) * 0.001));s.store_primal_offset_add_scaled_inputs3_offset_mixed_iaa(969, 960, ((0.5 * 1.001) * 0.5), t2, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(960), (-2.001), 1.001), 0.5, t2, 0.5), (-1.0)), ((0.25 * 0.001) * 0.001)), (-0.5), ((1.0 + (0.5 * ((-2.001) * 1.001))) * 0.5), (0.25 * 0.001));let t3: A = A::sqrt_square_offset(A::scaled_offset(s.ad_value(961), (-1.001), 1.001), ((0.25 * 0.001) * 0.001));s.store_primal_offset_add_scaled_inputs3_offset_mixed_iaa(967, 961, ((0.5 * 1.001) * 0.5), t3, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(961), (-1.001), 1.001), 0.5, t3, 0.5), (-1.0)), ((0.25 * 0.001) * 0.001)), (-0.5), ((1.0 + (0.5 * ((-1.001) * 1.001))) * 0.5), (0.25 * 0.001));let t4: A = A::sqrt_square_offset(A::scaled_offset(s.ad_value(961), (-2.001), 1.001), ((0.25 * 0.001) * 0.001));s.store_primal_offset_add_scaled_inputs3_offset_mixed_iaa(970, 961, ((0.5 * 1.001) * 0.5), t4, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(961), (-2.001), 1.001), 0.5, t4, 0.5), (-1.0)), ((0.25 * 0.001) * 0.001)), (-0.5), ((1.0 + (0.5 * ((-2.001) * 1.001))) * 0.5), (0.25 * 0.001));let t5: A = A::sqrt_square_offset(A::scaled_offset(s.ad_value(962), (-1.001), 1.001), ((0.25 * 0.001) * 0.001));s.store_primal_offset_add_scaled_inputs3_offset_mixed_iaa(968, 962, ((0.5 * 1.001) * 0.5), t5, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(962), (-1.001), 1.001), 0.5, t5, 0.5), (-1.0)), ((0.25 * 0.001) * 0.001)), (-0.5), ((1.0 + (0.5 * ((-1.001) * 1.001))) * 0.5), (0.25 * 0.001));let t6: A = A::sqrt_square_offset(A::scaled_offset(s.ad_value(962), (-2.001), 1.001), ((0.25 * 0.001) * 0.001));s.store_primal_offset_add_scaled_inputs3_offset_mixed_iaa(971, 962, ((0.5 * 1.001) * 0.5), t6, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scaled_offset(s.ad_value(962), (-2.001), 1.001), 0.5, t6, 0.5), (-1.0)), ((0.25 * 0.001) * 0.001)), (-0.5), ((1.0 + (0.5 * ((-2.001) * 1.001))) * 0.5), (0.25 * 0.001));s.store_primal_mul_pow_mixed_aii(976, A::pow(A::div(s.ad_value(894), s.ad_value(158)), s.ad_value(969)), 158, 966);s.store_primal_div(979, 976, 893);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_30(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1288] {s.store_primal_mul_pow_mixed_aii(977, A::pow(A::div(s.ad_value(894), s.ad_value(158)), s.ad_value(970)), 158, 967);s.store_primal_div(980, 977, 893);s.store_primal_mul_pow_mixed_aii(978, A::pow(A::div(s.ad_value(894), s.ad_value(158)), s.ad_value(971)), 158, 968);s.store_primal_div(981, 978, 893);}
        if s.b[1288] {s.store_scalar(982, (0.5 * (((1.0 / (1.0 + { let limited_exp_arg = ((2.75 - (p[40] * 1000000000.0)) / 0.78); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })) + 0.5) + ((((((1.0 / (1.0 + { let limited_exp_arg = ((2.75 - (p[40] * 1000000000.0)) / 0.78); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })) - 0.5) * ((1.0 / (1.0 + { let limited_exp_arg = ((2.75 - (p[40] * 1000000000.0)) / 0.78); if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } })) - 0.5)) + ((0.25 * 0.003) * 0.003))) as f64).sqrt())));}
        if s.b[1288] {s.store_primal_add_div_lhs(983, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(982), A::sub(s.ad_value(960), s.ad_value(882))), A::sub_from_scalar(p[1806], s.ad_value(882)), 982);s.store_primal_div_from_scalar_offset_ad(984, 1.0, A::limited_exp_scaled_input(A::offset(s.ad_value(983), (-0.999)), 1.0 / (0.0001)), 1.0);s.store_scalar(1013, (((((0.5 * p[40]) * p[40]) * 1e18) - ((1.5 * p[40]) * 1000000000.0)) + 2.0));s.store_primal_offset_sub_scaled_inputs(1014, A::offset(s.ad_value(1013), 4.0), 0.5, A::sqrt_square_offset(A::offset(s.ad_value(1013), (-4.0)), ((0.25 * 0.01) * 0.01)), 0.5, (0.25 * 0.01));let t7: A = A::powf(A::sub_from_scalar((p[40] * 1000000000.0), s.ad_value(1014)), p[1893]);let t8: A = A::sqrt_square_offset(A::scale_offset(t7, ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p[1893]))), ((s.v[168]) + ((-18100.0)))), ((0.25 * 0.01) * 0.01));s.store_offset_add_scaled_inputs3_offset(974, t7, ((0.5 * ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p[1893])))) * 0.5), t8, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scale_offset(t7, ((924000.0 - 18100.0) * 1.0 / (((2.0) as f64).powf(p[1893]))), ((s.v[168]) + (18100.0))), 0.5, t8, 0.5), (-924000.0)), ((0.25 * 9240.0) * 9240.0)), (-0.5), ((924000.0 + (0.5 * ((s.v[168]) + (18100.0)))) * 0.5), (0.25 * 9240.0));let t9: A = A::powf(A::sub_from_scalar((p[40] * 1000000000.0), s.ad_value(1014)), p[1894]);let ta: A = A::sqrt_square_offset(A::scale_offset(t9, ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p[1894]))), 5.5), ((0.25 * 0.01) * 0.01));s.store_primal_offset_add_scaled_inputs3_offset(975, t9, ((0.5 * ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p[1894])))) * 0.5), ta, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scale_offset(t9, ((8.0 - 5.5) * 1.0 / (((2.0) as f64).powf(p[1894]))), 5.5), 0.5, ta, 0.5), (-8.0)), ((0.25 * 0.01) * 0.01)), (-0.5), ((8.0 + (0.5 * 5.5)) * 0.5), (0.25 * 0.01));s.store_scalar(972, ((120.66 * ((4.0) as f64).powf(p[1895])) / (((p[40] * 1000000000.0)) as f64).powf(p[1895])));s.store_scalar(973, ((2.0 * ((4.0) as f64).powf(p[1896])) / (((p[40] * 1000000000.0)) as f64).powf(p[1896])));s.store_scalar(989, ((107.0 * ((4.0) as f64).powf(p[1897])) / (((p[40] * 1000000000.0)) as f64).powf(p[1897])));let tb: A = A::powf(A::sub_from_scalar((p[40] * 1000000000.0), s.ad_value(1014)), p[1898]);let tc: A = A::sqrt_square_offset(A::scale_offset(tb, 0.1, ((0.7) + ((-0.5)))), ((0.25 * 0.01) * 0.01));s.store_primal_offset_add_scaled_inputs3_offset(990, tb, ((0.5 * 0.1) * 0.5), tc, (0.5 * 0.5), A::sqrt_square_offset(A::offset(A::add_scaled_inputs(A::scale_offset(tb, 0.1, ((0.7) + (0.5))), 0.5, tc, 0.5), (-1.0)), ((0.25 * 0.01) * 0.01)), (-0.5), ((1.0 + (0.5 * ((0.7) + (0.5)))) * 0.5), (0.25 * 0.01));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_31(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[1288] {s.store_scalar(991, ((103.0 * ((4.0) as f64).powf(p[1899])) / (((p[40] * 1000000000.0)) as f64).powf(p[1899])));s.store_scalar(992, ((1.5 * ((4.0) as f64).powf(p[1900])) / (((p[40] * 1000000000.0)) as f64).powf(p[1900])));s.store_scalar(993, ((833.0 * ((4.0) as f64).powf(p[1901])) / (((p[40] * 1000000000.0)) as f64).powf(p[1901])));s.store_scalar(994, ((3.4 * ((4.0) as f64).powf(p[1902])) / (((p[40] * 1000000000.0)) as f64).powf(p[1902])));s.store_div_mixed_ia(987, 974, A::pow_from_scalar((p[1852] * 1000000000.0), A::scale(s.ad_value(975), p[1867])));s.store_primal_div_mixed_ia(988, 972, A::pow_from_scalar((p[1852] * 1000000000.0), A::scale(s.ad_value(973), p[1868])));let td: A = A::pow_from_scalar((p[43] * 1000000000.0), A::scale(s.ad_value(975), p[1867]));s.store_add_scaled_inputs4_mixed_iaia(985, 888, 0.5, A::div(s.ad_value(974), td), (p[1865] * 0.5), 987, ((-p[1865]) * 0.5), A::sqrt_square_offset(A::add_scaled_inputs3(s.ad_value(888), 1.0, A::div(s.ad_value(974), td), p[1865], s.ad_value(987), (-p[1865])), ((0.25 * 0.01) * 0.01)), 0.5);let te: A = A::pow_from_scalar((p[43] * 1000000000.0), A::scale(s.ad_value(973), p[1868]));s.store_primal_add_scaled_inputs4_mixed_iaia(986, 889, 0.5, A::div(s.ad_value(972), te), (p[1866] * 0.5), 988, ((-p[1866]) * 0.5), A::sqrt_square_offset(A::add_scaled_inputs3(s.ad_value(889), 1.0, A::div(s.ad_value(972), te), p[1866], s.ad_value(988), (-p[1866])), ((0.25 * 0.01) * 0.01)), 0.5);let tf: A = A::pow_from_scalar((p[43] * 1000000000.0), A::scale(s.ad_value(990), p[1890]));let t10: A = A::powf(A::scale_offset(tf, 5.0, 1.0), 0.5);s.store_primal_scaled_add_sqrt_square_offset_ad(995, A::div(s.ad_value(989), t10), ((0.25 * 0.1) * 0.1), 0.5);let t11: A = A::pow_from_scalar((p[1852] * 1000000000.0), A::scale(s.ad_value(990), p[1890]));let t12: A = A::powf(A::scale_offset(t11, 5.0, 1.0), 0.5);s.store_primal_scaled_add_sqrt_square_offset_ad(996, A::div(s.ad_value(989), t12), ((0.25 * 0.1) * 0.1), 0.5);s.store_primal_add_scaled_inputs3_indices(997, 890, 1.0, 995, p[1887], 996, (-p[1887]));let t13: A = A::pow_from_scalar((p[43] * 1000000000.0), A::scale(s.ad_value(992), p[1891]));let t14: A = A::powf(A::scale_offset(t13, 5.0, 1.0), 0.5);s.store_primal_scaled_add_sqrt_square_offset_ad(998, A::div(s.ad_value(991), t14), ((0.25 * 0.1) * 0.1), 0.5);}
    }
}

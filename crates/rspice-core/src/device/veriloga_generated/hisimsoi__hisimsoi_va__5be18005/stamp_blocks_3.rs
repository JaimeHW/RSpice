#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
    ) {
        if ((!s.b[737]) && s.b[1089]) {
            s.store_add_ad_rhs(1096, 159, A::mul(s.ad_value(1091), A::sub_from_scalar(1.0, s.ad_value(1092))));
            s.store_sqrt_square_offset(44, 1096, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_add(1096, 1096, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1103] = (s.v[1096] < 0.0);
        s.v[1103] = if s.b[1103] { 1.0 } else { 0.0 };

        if (((!s.b[737]) && s.b[1089]) && s.b[1103]) {
            s.store_scalar(1096, 0.0);
        }

        if ((!s.b[737]) && s.b[1089]) {
            s.store_div(1090, 157, 1096);
            s.store_pow_ad(1091, s.ad_value(1090), A::offset(s.ad_value(138), (-1.0)));
            s.store_mul(1095, 1091, 1090);
            s.store_offset(1092, 1095, 1.0);
            s.store_pow_ad(1093, s.ad_value(1092), A::offset(A::div_from_scalar(1.0, s.ad_value(138)), (-1.0)));
            s.store_mul(1094, 1093, 1092);
            s.store_div(452, 157, 1094);
            s.copy_ad(157, 452);
            s.store_exp_ad(484, A::mul(s.ad_value(225), A::sub(s.ad_value(515), s.ad_value(157))));
        }

        s.b[1104] = (s.v[157] <= 0.0);
        s.v[1104] = if s.b[1104] { 1.0 } else { 0.0 };

        if (((!s.b[737]) && s.b[1089]) && s.b[1104]) {
            s.store_scalar(164, 0.0);
            s.copy_ad(162, 161);
            s.store_scalar(430, 0.0);
        }

        s.b[1105] = (s.v[144] >= 1.0);
        s.v[1105] = if s.b[1105] { 1.0 } else { 0.0 };

        if ((((!s.b[737]) && s.b[1089]) && (!s.b[1104])) && s.b[1105]) {
            s.store_scalar(352, s.v[622]);
            s.store_sub_from_scalar(165, s.v[622], 161);
        }

        s.b[1106] = (s.v[144] == 0.0);
        s.v[1106] = if s.b[1106] { 1.0 } else { 0.0 };

        if ((((!s.b[737]) && s.b[1089]) && (!s.b[1104])) && s.b[1106]) {
            s.store_ad_value(166, {
                if ((s.v[163] - s.v[161]) >= 0.0) {
                    A::sub(s.ad_value(163), s.ad_value(161))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if ((((!s.b[737]) && s.b[1089]) && (!s.b[1104])) && s.b[1106]) {
            s.store_offset_sub_ad(44, A::scale(s.ad_value(166), (1.0 + 0.3)), s.ad_value(157), (-0.03));
            s.store_scale(45, 166, ((1.0 + 0.3) * (4.0 * 0.03)));
        }

        if ((((!s.b[737]) && s.b[1089]) && (!s.b[1104])) && s.b[1106]) {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if ((((!s.b[737]) && s.b[1089]) && (!s.b[1104])) && s.b[1106]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_sub_scaled_ad_rhs(165, 166, (1.0 + 0.3), A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((((!s.b[737]) && s.b[1089]) && (!s.b[1104])) && s.b[1106]) {
            s.store_ad_value(165, {
                if (s.v[165] <= s.v[166]) {
                    s.ad_value(165)
                } else {
                    s.ad_value(166)
                }
            });
        }

        s.b[1107] = (s.v[165] < 0.0);
        s.v[1107] = if s.b[1107] { 1.0 } else { 0.0 };

        if ((((!s.b[737]) && s.b[1089]) && (!s.b[1104])) && s.b[1107]) {
            s.store_scalar(165, 0.0);
        }

        s.b[1108] = (s.v[165] > s.v[157]);
        s.v[1108] = if s.b[1108] { 1.0 } else { 0.0 };

        if (((((!s.b[737]) && s.b[1089]) && (!s.b[1104])) && (!s.b[1107])) && s.b[1108]) {
            s.copy_ad(165, 157);
        }

        if (((!s.b[737]) && s.b[1089]) && (!s.b[1104])) {
            s.copy_ad(164, 165);
            s.store_add(162, 161, 164);
            s.store_scalar(430, 0.0);
        }

        if ((!s.b[737]) && s.b[1089]) {
            s.copy_ad(352, 162);
            s.store_scalar(168, 1.0);
        }

        let mut assign16250_loop_guard: usize = 0;
        while {
            let assign16250_cond_e23013: f64 = (s.v[58] + 1.0);
            let assign16250_cond_e23015: f64 = if (((!s.b[737]) && s.b[1089]) && (s.v[168] <= assign16250_cond_e23013)) { 1.0 } else { 0.0 };
            assign16250_cond_e23015 != 0.0
        } {
            assign16250_loop_guard += 1;
            assert!(assign16250_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((!s.b[737]) && s.b[1089]) {
                s.store_sub(418, 352, 515);
                s.store_mul(181, 225, 418);
                s.store_mul_sub_rhs(337, 420, 418, 419);
            }
            s.b[1109] = (s.v[337] < 80.0);
            s.v[1109] = if s.b[1109] { 1.0 } else { 0.0 };
            if (((!s.b[737]) && s.b[1089]) && s.b[1109]) {
                s.store_exp(328, 337);
                s.store_exp_ad(327, A::mul(A::neg(s.ad_value(420)), s.ad_value(419)));
                s.store_sub(329, 328, 327);
                s.store_div_ad_lhs(422, A::ln(A::offset(s.ad_value(329), 1.0)), 420);
                s.store_div_ad_rhs(423, 328, A::offset(s.ad_value(329), 1.0));
            }
            if (((!s.b[737]) && s.b[1089]) && (!s.b[1109])) {
                s.store_sub(422, 418, 419);
                s.store_scalar(423, 1.0);
            }
            if ((!s.b[737]) && s.b[1089]) {
                s.store_mul(421, 225, 422);
            }
            s.b[1110] = (((s.v[181]) as f64).abs() < 1e-16);
            s.v[1110] = if s.b[1110] { 1.0 } else { 0.0 };
            if (((!s.b[737]) && s.b[1089]) && s.b[1110]) {
                s.store_sqrt_scaled_ad(327, A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 0.5);
                s.store_mul(242, 181, 327);
                s.store_mul(443, 225, 327);
            }
            s.b[1111] = (s.v[181] < 0.0);
            s.v[1111] = if s.b[1111] { 1.0 } else { 0.0 };
            if ((((!s.b[737]) && s.b[1089]) && s.b[1110]) && s.b[1111]) {
                s.store_neg(242, 242);
                s.store_neg(443, 443);
            }
            s.b[1112] = (((s.v[181]) as f64).abs() < 0.005);
            s.v[1112] = if s.b[1112] { 1.0 } else { 0.0 };
            if ((((!s.b[737]) && s.b[1089]) && (!s.b[1110])) && s.b[1112]) {
                s.store_mul_ad(327, A::scale(A::square(s.ad_value(181)), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(181), 0.3333333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(181), 0.25), A::sub_from_scalar(1.0, A::scale(s.ad_value(181), 0.2)))))));
                s.store_mul_sub_from_scalar_ad_rhs(328, 181, 1.0, A::mul(A::scale(s.ad_value(181), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(181), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(181), 0.25))))));
                s.store_mul_ad(329, A::scale(A::square(s.ad_value(421)), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(421), 0.3333333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(421), 0.25), A::sub_from_scalar(1.0, A::scale(s.ad_value(421), 0.2)))))));
                s.store_mul_sub_from_scalar_ad_rhs(330, 421, 1.0, A::mul(A::scale(s.ad_value(421), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(421), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(421), 0.25))))));
                s.store_sqrt_sub(242, 327, 329);
                s.store_div_ad_lhs(443, A::mul(A::scale(s.ad_value(225), 0.5), A::sub(s.ad_value(328), A::mul(s.ad_value(423), s.ad_value(330)))), 242);
            }
            if ((((!s.b[737]) && s.b[1089]) && (!s.b[1110])) && (!s.b[1112])) {
                s.store_exp_neg_input(327, 181);
                s.store_exp_neg_input(328, 421);
                s.store_sqrt_add_ad(242, A::sub(s.ad_value(181), s.ad_value(421)), A::sub(s.ad_value(327), s.ad_value(328)));
                s.store_div_ad_lhs(443, A::mul(A::scale(s.ad_value(225), 0.5), A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul(s.ad_value(423), A::sub_from_scalar(1.0, s.ad_value(328))))), 242);
            }
            s.b[1113] = (s.v[338] == (-1.0));
            s.v[1113] = if s.b[1113] { 1.0 } else { 0.0 };
            if (((!s.b[737]) && s.b[1089]) && s.b[1113]) {
                s.store_scalar(401, 0.0);
            }
            if (((!s.b[737]) && s.b[1089]) && (!s.b[1113])) {
                s.store_mul(401, 444, 242);
            }
            if ((!s.b[737]) && s.b[1089]) {
                s.store_mul(370, 229, 401);
            }
            s.b[1114] = (s.v[181] < 0.0);
            s.v[1114] = if s.b[1114] { 1.0 } else { 0.0 };
            if (((!s.b[737]) && s.b[1089]) && s.b[1114]) {
                s.store_neg(499, 242);
                s.store_neg(500, 443);
            }
            s.b[1115] = (s.v[181] < 1e-7);
            s.v[1115] = if s.b[1115] { 1.0 } else { 0.0 };
            if ((((!s.b[737]) && s.b[1089]) && (!s.b[1114])) && s.b[1115]) {
                s.copy_ad(499, 242);
                s.copy_ad(500, 443);
            }
            if ((((!s.b[737]) && s.b[1089]) && (!s.b[1114])) && (!s.b[1115])) {
                s.store_mul_sub_rhs(501, 225, 352, 157);
                s.store_exp(502, 501);
                s.store_mul_sub_ad_rhs(497, 379, s.ad_value(502), A::mul(s.ad_value(484), A::offset(s.ad_value(181), 1.0)));
                s.store_mul_ad(498, A::mul(s.ad_value(379), s.ad_value(225)), A::sub(s.ad_value(502), s.ad_value(484)));
                s.store_sqrt_square_add(499, 242, 497);
                s.store_div_ad_lhs(500, A::scale(A::add(A::mul(A::scale(s.ad_value(443), 2.0), s.ad_value(242)), s.ad_value(498)), 0.5), 499);
            }
            if ((!s.b[737]) && s.b[1089]) {
                s.store_sub_ad(503, A::add(A::sub(s.ad_value(352), s.ad_value(159)), A::mul(s.ad_value(240), s.ad_value(499))), A::mul(s.ad_value(324), s.ad_value(393)));
                s.store_offset_mul(504, 240, 500, 1.0);
            }
            s.b[1116] = ((s.v[430] == 1.0) && (s.v[168] > 3.0));
            s.v[1116] = if s.b[1116] { 1.0 } else { 0.0 };
            if (((!s.b[737]) && s.b[1089]) && s.b[1116]) {
                s.store_scalar(168, (s.v[58] + 1.0));
            }
            if (((!s.b[737]) && s.b[1089]) && (!s.b[1116])) {
                s.store_div_ad_lhs(495, A::neg(s.ad_value(503)), 504);
            }
            if (((!s.b[737]) && s.b[1089]) && (!s.b[1116])) {
                s.store_scaled_offset_ad(496, {
                    if (1.0 >= ((s.v[352]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(352))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1117] = (((s.v[495]) as f64).abs() > s.v[496]);
            s.v[1117] = if s.b[1117] { 1.0 } else { 0.0 };
            if ((((!s.b[737]) && s.b[1089]) && (!s.b[1116])) && s.b[1117]) {
                s.store_scale(495, 496, (if (s.v[495] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((!s.b[737]) && s.b[1089]) && (!s.b[1116])) {
                s.store_add(352, 352, 495);
            }
            s.b[1118] = ((((s.v[495]) as f64).abs() <= 5e-12) && (((s.v[503]) as f64).abs() <= 1e-8));
            s.v[1118] = if s.b[1118] { 1.0 } else { 0.0 };
            if ((((!s.b[737]) && s.b[1089]) && (!s.b[1116])) && s.b[1118]) {
                s.store_scalar(430, 1.0);
            }
            if ((!s.b[737]) && s.b[1089]) {
                s.store_offset(168, 168, 1.0);
            }
        }

        if ((!s.b[737]) && s.b[1089]) {
            s.store_offset(168, 168, (-1.0));
            s.copy_ad(372, 370);
            s.copy_ad(359, 372);
            s.copy_ad(162, 352);
            s.store_div(569, 372, 238);
            s.store_offset(171, 569, (10.0 * 2.220446049250313e-16));
            s.store_div_from_scalar_add_ad(328, 1.0, s.ad_value(499), s.ad_value(171));
            s.store_mul3_lhs(358, 238, 497, 328);
            s.store_neg(358, 358);
            s.store_sub(164, 162, 161);
            s.copy_ad(157, 453);
            s.store_div(328, 225, 169);
            s.store_mul(505, 328, 164);
            s.store_offset(506, 505, 1.0);
            s.store_sqrt(507, 506);
            s.store_div_from_scalar_offset_input(508, 1.0, 507, 1.0);
            s.store_div(509, 508, 170);
            s.store_scaled_add(510, 568, 569, 0.5);
            s.store_sub_ad(328, A::add(s.ad_value(159), s.ad_value(227)), A::scale(A::add(A::scale(s.ad_value(161), 2.0), s.ad_value(164)), 0.5));
            s.store_sub(329, 509, 510);
            s.store_mul(330, 225, 323);
            s.store_mul(331, 225, 238);
            s.store_add_ad(511, A::mul(s.ad_value(330), s.ad_value(328)), A::mul(s.ad_value(331), s.ad_value(329)));
            s.store_scaled_add(424, 359, 356, 0.5);
            s.store_scale_ad(425, A::neg(A::add(s.ad_value(358), s.ad_value(355))), 0.5);
            s.store_sub(426, 359, 356);
            s.store_neg_ad(427, A::sub(s.ad_value(358), s.ad_value(355)));
            s.store_square(428, 238);
        }

        s.b[1119] = (s.v[339] <= 1.0);
        s.v[1119] = if s.b[1119] { 1.0 } else { 0.0 };

        if (((!s.b[737]) && s.b[1089]) && s.b[1119]) {
            s.store_sub_ad(246, A::sub(A::mul(A::mul(s.ad_value(425), s.ad_value(225)), s.ad_value(164)), s.ad_value(427)), A::scale(A::div(A::mul(A::square(s.ad_value(426)), s.ad_value(426)), s.ad_value(428)), 0.16666666666666666));
        }

        if (((!s.b[737]) && s.b[1089]) && (!s.b[1119])) {
            s.store_mul(246, 164, 511);
        }

        s.b[1120] = ((s.v[84] >= 1.0) && (s.v[246] < 0.0));
        s.v[1120] = if s.b[1120] { 1.0 } else { 0.0 };

        if (((!s.b[737]) && s.b[1089]) && s.b[1120]) {
            s.store_scalar(246, 0.0);
        }

        s.b[1121] = (s.v[339] <= 1.0);
        s.v[1121] = if s.b[1121] { 1.0 } else { 0.0 };

        s.b[1122] = (((s.v[164]) as f64).abs() > 1e-6);
        s.v[1122] = if s.b[1122] { 1.0 } else { 0.0 };

        if ((((!s.b[737]) && s.b[1089]) && s.b[1121]) && s.b[1122]) {
            let assign16620_ad_e24190: A = A::mul(A::mul(A::mul(A::add(A::sub(s.ad_value(425), A::scale(s.ad_value(424), 2.0)), A::mul(A::div(s.ad_value(323), s.ad_value(225)), A::add(A::sub_from_scalar(1.0, A::div(A::mul(A::scale(s.ad_value(424), 2.0), s.ad_value(424)), s.ad_value(428))), A::scale(A::div(A::square(s.ad_value(426)), s.ad_value(428)), 0.1)))), s.ad_value(426)), s.ad_value(426)), s.ad_value(426));
            s.store_add_ad(437, A::mul(s.ad_value(424), A::sub(A::mul(A::mul(s.ad_value(425), s.ad_value(225)), s.ad_value(164)), s.ad_value(427))), A::scale(A::div(assign16620_ad_e24190, s.ad_value(428)), 0.16666666666666666));
        }

        if ((((!s.b[737]) && s.b[1089]) && s.b[1121]) && s.b[1122]) {
            s.store_div(437, 437, 246);
        }

        if ((((!s.b[737]) && s.b[1089]) && s.b[1121]) && (!s.b[1122])) {
            s.copy_ad(437, 424);
        }

        if (((!s.b[737]) && s.b[1089]) && (!s.b[1121])) {
            s.store_scaled_add(437, 359, 356, 0.5);
        }

        if ((!s.b[737]) && s.b[1089]) {
            s.store_scale(328, 240, 2.0);
            s.store_mul_sub_rhs(512, 328, 510, 170);
            s.store_add(191, 164, 512);
        }

    }

    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[737]) && s.b[1089]) {
            s.store_div_from_scalar(328, 1.0, 192);
            s.store_mul(329, 191, 328);
            s.store_sub_from_scalar(330, 1.0, 329);
            s.store_sub_from_scalar(336, 1.0, 330);
            s.store_square(49, 336);
            s.store_scalar(50, 1.0);
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1123] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1123] = if s.b[1123] { 1.0 } else { 0.0 };

        s.b[1124] = (4.0 == 1.0);
        s.v[1124] = if s.b[1124] { 1.0 } else { 0.0 };

        if ((((!s.b[737]) && s.b[1089]) && s.b[1123]) && s.b[1124]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1125] = (4.0 == 2.0);
        s.v[1125] = if s.b[1125] { 1.0 } else { 0.0 };

        if (((((!s.b[737]) && s.b[1089]) && s.b[1123]) && (!s.b[1124])) && s.b[1125]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1126] = (4.0 == 4.0);
        s.v[1126] = if s.b[1126] { 1.0 } else { 0.0 };

        if ((((((!s.b[737]) && s.b[1089]) && s.b[1123]) && (!s.b[1124])) && (!s.b[1125])) && s.b[1126]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1127] = (4.0 == 8.0);
        s.v[1127] = if s.b[1127] { 1.0 } else { 0.0 };

        if (((((((!s.b[737]) && s.b[1089]) && s.b[1123]) && (!s.b[1124])) && (!s.b[1125])) && (!s.b[1126])) && s.b[1127]) {
            s.store_scalar(55, 4.0);
        }

        if (((!s.b[737]) && s.b[1089]) && s.b[1123]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign17010_loop_guard: usize = 0;
        while {
            let assign17010_cond_e24557: f64 = if ((((!s.b[737]) && s.b[1089]) && s.b[1123]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign17010_cond_e24557 != 0.0
        } {
            assign17010_loop_guard += 1;
            assert!(assign17010_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((!s.b[737]) && s.b[1089]) && s.b[1123]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((!s.b[737]) && s.b[1089]) && (!s.b[1123])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if ((!s.b[737]) && s.b[1089]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul(337, 336, 53);
            s.store_sub_from_scalar(190, 1.0, 337);
            s.store_offset_mul_ad(478, s.ad_value(190), A::offset(s.ad_value(190), 1.0), 1.0);
        }

        if ((!s.b[737]) && s.b[1089]) {
            s.store_ad_value(479, {
                if ((1.0 + s.v[190]) >= (10.0 * 2.220446049250313e-16)) {
                    A::offset(s.ad_value(190), 1.0)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if ((!s.b[737]) && s.b[1089]) {
            s.store_div_ad_lhs(328, A::mul(A::scale(s.ad_value(192), 0.6666666666666667), s.ad_value(478)), 479);
        }

        s.b[1128] = (s.v[339] <= 1.0);
        s.v[1128] = if s.b[1128] { 1.0 } else { 0.0 };

        s.b[1129] = (((s.v[164]) as f64).abs() > 1e-6);
        s.v[1129] = if s.b[1129] { 1.0 } else { 0.0 };

        if ((((!s.b[737]) && s.b[1089]) && s.b[1128]) && s.b[1129]) {
            let assign17110_ad_e24727: A = A::sub(A::sub(A::mul(A::mul(A::add(A::square(s.ad_value(425)), A::scale(A::square(s.ad_value(427)), 0.08333333333333333)), s.ad_value(225)), s.ad_value(164)), A::mul(s.ad_value(425), s.ad_value(427))), A::scale(A::div(A::mul(A::mul(A::mul(A::add(A::scale(s.ad_value(425), 2.0), A::scale(A::div(A::mul(A::mul(A::div(s.ad_value(323), s.ad_value(225)), s.ad_value(426)), s.ad_value(426)), s.ad_value(428)), 0.2)), s.ad_value(426)), s.ad_value(426)), s.ad_value(426)), s.ad_value(428)), 0.16666666666666666));
            s.store_ad_value(436, assign17110_ad_e24727);
        }

        if ((((!s.b[737]) && s.b[1089]) && s.b[1128]) && s.b[1129]) {
            s.store_div(436, 436, 246);
        }

        if ((((!s.b[737]) && s.b[1089]) && s.b[1128]) && (!s.b[1129])) {
            s.copy_ad(436, 425);
        }

        if (((!s.b[737]) && s.b[1089]) && (!s.b[1128])) {
            s.store_scaled_add(436, 355, 358, (-0.5));
        }

        s.b[1133] = (s.v[612] == 0.0);
        s.v[1133] = if s.b[1133] { 1.0 } else { 0.0 };

        if s.b[1133] {
            s.store_offset(480, 190, 0.5);
            s.store_mul(481, 479, 478);
            s.store_scaled_div(482, 480, 481, 0.4);
            s.store_sub_from_scalar(438, 0.6, 482);
        }

        s.b[1134] = (s.v[438] > (0.5 + 1e-8));
        s.v[1134] = if s.b[1134] { 1.0 } else { 0.0 };

        if (s.b[1133] && s.b[1134]) {
            s.store_scalar(438, 0.5);
        }

        if s.b[1133] {
            s.copy_ad(439, 438);
            s.store_scalar(438, 0.5);
        }

        s.b[1136] = (s.v[145] == 0.0);
        s.v[1136] = if s.b[1136] { 1.0 } else { 0.0 };

        s.b[1152] = ((p.p190 < (10.0 * 2.220446049250313e-16)) && (p.p191 < (10.0 * 2.220446049250313e-16)));
        s.v[1152] = if s.b[1152] { 1.0 } else { 0.0 };

        if ((s.b[1133] && s.b[1136]) && s.b[1152]) {
            s.store_scalar(316, 0.0);
            s.copy_ad(314, 162);
        }

        s.b[1153] = (s.v[314] > ((s.v[161] + s.v[173]) - (10.0 * 2.220446049250313e-16)));
        s.v[1153] = if s.b[1153] { 1.0 } else { 0.0 };

        if (((s.b[1133] && s.b[1136]) && s.b[1152]) && s.b[1153]) {
            s.store_offset_add(314, 161, 173, (-(10.0 * 2.220446049250313e-16)));
        }

        if ((s.b[1133] && s.b[1136]) && (!s.b[1152])) {
            s.store_scalar(1151, (if (p.p43 == 1.0) { p.p237 } else { s.v[402] }));
        }

        if ((s.b[1133] && s.b[1136]) && (!s.b[1152])) {
            s.store_div_from_scalar(1137, 1.0, 1151);
            s.store_mul(1138, 244, 1137);
            s.store_scale(1139, 1138, p.p191);
            s.store_add_ad_lhs(1142, A::mul(s.ad_value(80), s.ad_value(229)), 1139);
            s.store_div_from_scalar(1138, 1.0, 1142);
            s.store_scale(1141, 1138, 1.034943e-10);
            s.store_scalar(1138, (1.0 - p.p189));
            s.store_add_ad(314, A::scale(A::add(s.ad_value(157), s.ad_value(161)), p.p189), A::mul(s.ad_value(1138), s.ad_value(162)));
        }

        s.b[1154] = (s.v[314] > ((s.v[161] + s.v[173]) - (10.0 * 2.220446049250313e-16)));
        s.v[1154] = if s.b[1154] { 1.0 } else { 0.0 };

        if (((s.b[1133] && s.b[1136]) && (!s.b[1152])) && s.b[1154]) {
            s.store_offset_add(314, 161, 173, (-(10.0 * 2.220446049250313e-16)));
        }

        if ((s.b[1133] && s.b[1136]) && (!s.b[1152])) {
            s.store_sub(1144, 314, 162);
            s.store_sqrt_square_offset(44, 1144, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_add(1143, 1144, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1155] = (s.v[1143] < 0.0);
        s.v[1155] = if s.b[1155] { 1.0 } else { 0.0 };

        if (((s.b[1133] && s.b[1136]) && (!s.b[1152])) && s.b[1155]) {
            s.store_scalar(1143, 0.0);
        }

        if ((s.b[1133] && s.b[1136]) && (!s.b[1152])) {
            s.store_mul(1140, 225, 244);
            s.store_div_from_scalar(1138, 1.0, 1140);
            s.store_mul(1142, 246, 1138);
        }

        s.b[1156] = (s.v[1142] < s.v[227]);
        s.v[1156] = if s.b[1156] { 1.0 } else { 0.0 };

        if (((s.b[1133] && s.b[1136]) && (!s.b[1152])) && s.b[1156]) {
            s.copy_ad(1142, 227);
        }

        if ((s.b[1133] && s.b[1136]) && (!s.b[1152])) {
            s.store_scale(1148, 229, 9662367879.197212);
            s.store_scalar(1138, (100000.0 * 10000.0));
            s.store_scalar(1139, (1.0 / s.v[97]));
            s.store_mul_add_ad_lhs(1150, A::add(A::scale(s.ad_value(1142), 2.0), A::mul(A::mul(A::scale(s.ad_value(1148), 2.0), s.ad_value(1143)), s.ad_value(1141))), A::mul(s.ad_value(1138), s.ad_value(1141)), 1139);
            s.store_mul(1145, 1150, 1141);
            s.store_scaled_add_ad_lhs(1149, A::mul(A::scale(s.ad_value(1148), 2.0), s.ad_value(1143)), 1138, 4.0);
            s.store_mul3_lhs(1146, 1149, 1141, 1141);
            s.store_sqrt_square_add(1147, 1145, 1146);
            s.store_mul_scale_ad_rhs(316, 326, A::sub(s.ad_value(1147), s.ad_value(1145)), 0.5);
        }

        if (s.b[1133] && s.b[1136]) {
            s.store_scale(316, 316, s.v[127]);
        }

        if s.b[1133] {
            s.store_sub_from_scalar(441, s.v[97], 316);
        }

        s.b[1157] = (s.v[441] < 1e-9);
        s.v[1157] = if s.b[1157] { 1.0 } else { 0.0 };

        if (s.b[1133] && s.b[1157]) {
            s.store_scalar(441, 1e-9);
        }

        if s.b[1133] {
            s.store_scale(328, 108, (-s.v[98]));
            s.store_mul(196, 328, 437);
            s.store_mul(197, 328, 436);
            s.store_mul(198, 197, 438);
        }

        s.b[1158] = (p.p43 == 0.0);
        s.v[1158] = if s.b[1158] { 1.0 } else { 0.0 };

        if (s.b[1133] && s.b[1158]) {
            s.store_scale(477, 196, 0.5);
            s.store_scale(476, 196, (1.0 - 0.5));
            s.store_mul_scale_ad_lhs(392, A::add(s.ad_value(357), s.ad_value(360)), (0.5 * s.v[98]), 108);
        }

        if s.b[1133] {
            s.store_scaled_sub(1159, 157, 164, 0.5);
            s.store_scale(44, 1159, (2.0 * 1.0 / (p.p227)));
            s.store_offset_mul_ad(45, s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::scale(s.ad_value(44), (1.0 / 5040.0)), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0)), 1.0);
            s.store_div_from_scalar(177, p.p227, 45);
        }

        s.b[1160] = (s.v[177] < (10.0 * 2.220446049250313e-16));
        s.v[1160] = if s.b[1160] { 1.0 } else { 0.0 };

        if (s.b[1133] && s.b[1160]) {
            s.store_scalar(177, (10.0 * 2.220446049250313e-16));
        }

        if s.b[1133] {
            s.store_add(176, 161, 177);
            s.store_scalar(1170, (1.034943e-10 / 100.0));
            s.store_scale(1171, 437, 0.0001);
            s.store_scale(1172, 436, 0.0001);
            s.store_div_from_scalar(1161, p.p92, 1170);
            s.store_div_from_scalar(1162, p.p93, 1170);
            s.store_scalar(1163, p.p94);
            s.store_offset_mul_ad(1164, A::sub(s.ad_value(162), s.ad_value(161)), s.ad_value(1163), 1.0);
            s.store_add_ad(1165, A::mul(s.ad_value(1161), s.ad_value(1171)), A::mul(s.ad_value(1162), s.ad_value(1172)));
            s.store_div(1166, 1165, 1164);
            s.copy_ad(248, 1166);
            s.store_sqrt_square_offset(44, 248, ((4.0 * 3000.0) * 3000.0));
            s.store_offset_scaled_add(1163, 248, 44, 0.5, (1e-10 * 3000.0));
        }

        s.b[1173] = (s.v[1163] < 0.0);
        s.v[1173] = if s.b[1173] { 1.0 } else { 0.0 };

        if (s.b[1133] && s.b[1173]) {
            s.store_scalar(1163, 0.0);
        }

        if s.b[1133] {
            s.store_powf(1165, 1163, (p.p97 - 1.0));
            s.store_mul(1167, 1165, 1163);
            s.store_powf(1168, 1163, (s.v[111] - 1.0));
            s.store_mul(1169, 1168, 1163);
        }

    }

    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[1133] {
            s.store_scale(249, 1172, 6.241449993689894e18);
            s.store_add_scaled_ad_lhs(1161, A::add(A::div_from_scalar(1.0, A::offset(A::scale(s.ad_value(249), (p.p96 * 1e-11)), p.p95)), A::mul(s.ad_value(543), s.ad_value(1167))), 1169, 1.0 / (p.p106));
            s.store_div_from_scalar(251, 1.0, 1161);
            s.store_scale(251, 251, 0.0001);
            s.store_mul3_lhs(1174, 225, 244, 441);
            s.store_sqrt_square_offset(44, 1174, ((4.0 * 1e-50) * 1e-50));
            s.store_offset_scaled_add(1174, 1174, 44, 0.5, (1e-10 * 1e-50));
        }

        s.b[1182] = (s.v[1174] < 0.0);
        s.v[1182] = if s.b[1182] { 1.0 } else { 0.0 };

        if (s.b[1133] && s.b[1182]) {
            s.store_scalar(1174, 0.0);
        }

        if s.b[1133] {
            s.store_div_from_scalar(1175, 1.0, 1174);
            s.store_mul(1176, 246, 1175);
            s.store_scaled_div(1174, 253, 251, 0.2);
            s.store_sqrt_square_sum(252, 1176, 1174);
            s.store_mul(1177, 251, 252);
            s.store_div(1175, 1177, 253);
        }

        s.b[1183] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1183] = if s.b[1183] { 1.0 } else { 0.0 };

        if (s.b[1133] && s.b[1183]) {
            s.store_scalar(1178, 1.0);
        }

        s.b[1184] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1184] = if s.b[1184] { 1.0 } else { 0.0 };

        if ((s.b[1133] && (!s.b[1183])) && s.b[1184]) {
            s.copy_ad(1178, 1175);
        }

        if ((s.b[1133] && (!s.b[1183])) && (!s.b[1184])) {
            s.store_powf(1178, 1175, (p.p113 - 1.0));
        }

        if s.b[1133] {
            s.store_mul(1174, 1175, 1178);
            s.store_offset(1179, 1174, 1.0);
        }

        s.b[1185] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1185] = if s.b[1185] { 1.0 } else { 0.0 };

        if (s.b[1133] && s.b[1185]) {
            s.store_div_from_scalar(1180, 1.0, 1179);
        }

        s.b[1186] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1186] = if s.b[1186] { 1.0 } else { 0.0 };

        if ((s.b[1133] && (!s.b[1185])) && s.b[1186]) {
            s.store_div_from_scalar_sqrt_ad(1180, 1.0, s.ad_value(1179));
        }

        if ((s.b[1133] && (!s.b[1185])) && (!s.b[1186])) {
            s.store_powf(1181, 1179, (((-1.0) / p.p113) - 1.0));
            s.store_mul(1180, 1179, 1181);
        }

        if s.b[1133] {
            s.store_mul(250, 251, 1180);
            s.store_div_ad(264, A::mul(s.ad_value(107), s.ad_value(227)), A::sub_from_scalar(s.v[97], s.ad_value(316)));
            s.store_mul3_lhs(200, 264, 246, 250);
            s.store_scalar(201, 0.0);
        }

        s.b[1196] = ((p.p281 > 0.0) && (p.p244 != 0.0));
        s.v[1196] = if s.b[1196] { 1.0 } else { 0.0 };

        if (s.b[1133] && s.b[1196]) {
            s.store_scaled_sub(1187, 157, 164, 0.5);
            s.store_scale(44, 1187, (2.0 * 100.0));
            s.store_offset_mul_ad(45, s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::mul(s.ad_value(44), A::offset(A::scale(s.ad_value(44), (1.0 / 5040.0)), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0)), 1.0);
            s.store_div_from_scalar(1193, 0.01, 45);
            s.store_sub_from_scalar_ad(1187, 1.1, A::add(s.ad_value(161), s.ad_value(1193)));
            s.store_sqrt_square_offset(44, 1187, ((4.0 * 0.05) * 0.05));
            s.store_offset_scaled_add(1195, 1187, 44, 0.5, (1e-10 * 0.05));
        }

        s.b[1197] = (s.v[1195] < 0.0);
        s.v[1197] = if s.b[1197] { 1.0 } else { 0.0 };

        if ((s.b[1133] && s.b[1196]) && s.b[1197]) {
            s.store_scalar(1195, 0.0);
        }

        if (s.b[1133] && s.b[1196]) {
            s.store_scale(1188, 225, s.v[116]);
            s.store_mul(1189, 323, 1188);
            s.store_powf(1188, 1195, p.p245);
            s.store_mul(1190, 1189, 1188);
            s.store_offset_scaled(1191, 173, p.p246, 1.0);
            s.store_scalar(1188, s.v[117]);
        }

        s.b[1198] = ((s.v[56] < 3.0) || (p.p43 == 1.0));
        s.v[1198] = if s.b[1198] { 1.0 } else { 0.0 };

        if ((s.b[1133] && s.b[1196]) && s.b[1198]) {
            s.store_sub_ad_lhs(1192, A::add(s.ad_value(161), s.ad_value(1193)), 172);
        }

        if ((s.b[1133] && s.b[1196]) && (!s.b[1198])) {
            s.store_sub_ad_lhs(1192, A::add(s.ad_value(161), s.ad_value(1193)), 350);
        }

        if (s.b[1133] && s.b[1196]) {
            s.store_add_ad_rhs(1191, 1191, A::mul(A::mul(s.ad_value(173), s.ad_value(1188)), s.ad_value(1192)));
            s.store_mul(1193, 1190, 1191);
            s.copy_ad(1190, 1193);
        }

        if (s.b[1133] && (!s.b[1196])) {
            s.store_scalar(1190, 0.0);
        }

        s.b[1199] = (p.p248 != 0.0);
        s.v[1199] = if s.b[1199] { 1.0 } else { 0.0 };

        if (s.b[1133] && s.b[1199]) {
            s.store_scale(1187, 225, s.v[118]);
            s.store_mul(1195, 323, 1187);
            s.store_mul(1194, 1195, 173);
        }

        if (s.b[1133] && (!s.b[1199])) {
            s.store_scalar(1194, 0.0);
        }

        s.b[1200] = ((s.v[1190] + s.v[1194]) > 0.0);
        s.v[1200] = if s.b[1200] { 1.0 } else { 0.0 };

        if (s.b[1133] && s.b[1200]) {
            s.store_mul_add_rhs(247, 164, 1190, 1194);
            s.store_mul3_lhs(201, 264, 247, 250);
        }

        if s.b[1133] {
            s.store_add(199, 200, 201);
            s.copy_ad(203, 201);
        }

        s.b[1210] = (p.p33 != 0.0);
        s.v[1210] = if s.b[1210] { 1.0 } else { 0.0 };

        if (s.b[1133] && s.b[1210]) {
            s.copy_ad(1203, 549);
            s.store_scalar(1204, (s.v[124] - p.p71));
            s.store_div_from_scalar_square_ad(1205, 1.0, s.ad_value(1204));
            s.store_mul_ad_product_lhs(1206, A::mul(A::scale(A::sub_from_scalar(p.p69, s.ad_value(233)), 2.0), A::scale(s.ad_value(324), 1.034943e-10)), s.ad_value(1203), 1205);
            s.store_mul(186, 1206, 235);
            s.store_offset_scaled(1202, 173, p.p155, p.p154);
            s.store_mul(206, 186, 1202);
            s.store_sub_from_scalar_ad(1201, p.p156, A::scale(s.ad_value(157), p.p157));
            s.store_add_ad_lhs(207, A::add(A::offset(s.ad_value(174), (-s.v[123])), s.ad_value(1201)), 206);
            s.store_mul3_lhs(210, 205, 324, 324);
            s.store_scaled_mul(211, 210, 225, 0.5);
            s.store_scaled_mul(212, 211, 225, 2.0);
            s.store_offset_sub_ad(1207, A::offset(A::offset(A::sub(s.ad_value(227), A::mul(s.ad_value(210), A::scale(s.ad_value(225), 0.25))), s.v[123]), (-p.p156)), s.ad_value(206), 1e-50);
            s.store_offset_sub(1201, 174, 1207, (-0.005));
        }

        if (s.b[1133] && s.b[1210]) {
            s.store_scalar(327, (if (s.v[1207] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if (s.b[1133] && s.b[1210]) {
            s.store_sqrt_add_ad(1203, A::square(s.ad_value(1201)), A::scale(A::mul(A::scale(s.ad_value(327), 4.0), s.ad_value(1207)), 0.005));
            s.store_sub_ad_lhs(1204, A::add(A::offset(A::offset(A::add(s.ad_value(1207), A::scale(A::add(s.ad_value(1201), s.ad_value(1203)), 0.5)), (-s.v[123])), p.p156), s.ad_value(206)), 514);
            s.store_offset_mul(1205, 225, 1204, (-1.0));
            s.store_div_from_scalar(1206, 4.0, 212);
            s.store_offset_mul(1202, 1205, 1206, 1.0);
            s.store_sqrt_square_offset(44, 1202, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_add(1201, 1202, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1211] = (s.v[1201] < 0.0);
        s.v[1211] = if s.b[1211] { 1.0 } else { 0.0 };

        if ((s.b[1133] && s.b[1210]) && s.b[1211]) {
            s.store_scalar(1201, 0.0);
        }

        if (s.b[1133] && s.b[1210]) {
            s.store_sqrt_offset_input(213, 1201, 1e-50);
            s.store_add_ad_rhs(215, 207, A::mul(s.ad_value(211), A::sub_from_scalar(1.0, s.ad_value(213))));
            s.store_div_from_scalar_add_ad(327, 1.0, s.ad_value(225), A::div_from_scalar(2.0, A::offset(s.ad_value(207), 1e-50)));
            s.store_mul_ln_ad_lhs(216, A::mul(A::div(A::div_from_scalar(1.0, s.ad_value(209)), s.ad_value(210)), A::square(s.ad_value(207))), 327);
            s.store_div_ad_rhs(1204, 216, A::offset(s.ad_value(207), 1e-50));
            s.store_offset_sub(217, 216, 215, (-0.002));
            s.store_sqrt_add_ad(327, A::square(s.ad_value(217)), A::scale(s.ad_value(216), (4.0 * 0.002)));
            s.store_sub_ad_rhs(218, 216, A::scale(A::add(s.ad_value(217), s.ad_value(327)), 0.5));
            s.store_div_from_scalar(1201, 1.0, 327);
            s.store_mul_exp_ad_rhs(327, 209, A::mul(s.ad_value(225), s.ad_value(218)));
            s.store_add_ad_lhs(1202, A::offset(A::mul(s.ad_value(225), A::sub(s.ad_value(218), s.ad_value(514))), (-1.0)), 327);
            s.store_sqrt_square_offset(44, 1202, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_add(1201, 1202, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1212] = (s.v[1201] < 0.0);
        s.v[1212] = if s.b[1212] { 1.0 } else { 0.0 };

        if ((s.b[1133] && s.b[1210]) && s.b[1212]) {
            s.store_scalar(1201, 0.0);
        }

        if (s.b[1133] && s.b[1210]) {
            s.store_sqrt_offset_input(219, 1201, (10.0 * 2.220446049250313e-16));
            s.store_offset_mul_ad(1202, s.ad_value(225), A::sub(s.ad_value(218), s.ad_value(514)), (-1.0));
            s.store_sqrt_square_offset(44, 1202, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_add(1201, 1202, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1213] = (s.v[1201] < 0.0);
        s.v[1213] = if s.b[1213] { 1.0 } else { 0.0 };

        if ((s.b[1133] && s.b[1210]) && s.b[1213]) {
            s.store_scalar(1201, 0.0);
        }

        if (s.b[1133] && s.b[1210]) {
            s.store_sqrt_offset_input(220, 1201, (10.0 * 2.220446049250313e-16));
            s.store_mul_sub_rhs(221, 208, 219, 220);
            s.store_sub(1202, 215, 218);
            s.store_sqrt_square_offset(44, 1202, ((4.0 * 0.1) * 0.1));
            s.store_offset_scaled_add(1201, 1202, 44, 0.5, (1e-10 * 0.1));
        }

        s.b[1214] = (s.v[1201] < 0.0);
        s.v[1214] = if s.b[1214] { 1.0 } else { 0.0 };

        if ((s.b[1133] && s.b[1210]) && s.b[1214]) {
            s.store_scalar(1201, 0.0);
        }

        if (s.b[1133] && s.b[1210]) {
            s.store_div_ad_rhs(1208, 157, A::offset(s.ad_value(1201), (10.0 * 2.220446049250313e-16)));
            s.store_square(49, 1208);
            s.store_scalar(50, 1.0);
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
        }

    }

    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1133] && s.b[1210]) {
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1215] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1215] = if s.b[1215] { 1.0 } else { 0.0 };

        s.b[1216] = (4.0 == 1.0);
        s.v[1216] = if s.b[1216] { 1.0 } else { 0.0 };

        if (((s.b[1133] && s.b[1210]) && s.b[1215]) && s.b[1216]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1217] = (4.0 == 2.0);
        s.v[1217] = if s.b[1217] { 1.0 } else { 0.0 };

        if ((((s.b[1133] && s.b[1210]) && s.b[1215]) && (!s.b[1216])) && s.b[1217]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1218] = (4.0 == 4.0);
        s.v[1218] = if s.b[1218] { 1.0 } else { 0.0 };

        if (((((s.b[1133] && s.b[1210]) && s.b[1215]) && (!s.b[1216])) && (!s.b[1217])) && s.b[1218]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1219] = (4.0 == 8.0);
        s.v[1219] = if s.b[1219] { 1.0 } else { 0.0 };

        if ((((((s.b[1133] && s.b[1210]) && s.b[1215]) && (!s.b[1216])) && (!s.b[1217])) && (!s.b[1218])) && s.b[1219]) {
            s.store_scalar(55, 4.0);
        }

        if ((s.b[1133] && s.b[1210]) && s.b[1215]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign19490_loop_guard: usize = 0;
        while {
            let assign19490_cond_e26982: f64 = if (((s.b[1133] && s.b[1210]) && s.b[1215]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign19490_cond_e26982 != 0.0
        } {
            assign19490_loop_guard += 1;
            assert!(assign19490_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1133] && s.b[1210]) && s.b[1215]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((s.b[1133] && s.b[1210]) && (!s.b[1215])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if (s.b[1133] && s.b[1210]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul(1209, 1208, 53);
            s.store_scale(214, 227, ((2.0 * s.v[126]) * p.p9));
            s.store_div_ad_lhs(222, A::mul(A::mul(A::mul(s.ad_value(214), s.ad_value(250)), s.ad_value(221)), s.ad_value(1209)), 441);
            s.store_add(199, 199, 222);
        }

        s.b[1220] = ((p.p30 != 0.0) && (p.p32 != 0.0));
        s.v[1220] = if s.b[1220] { 1.0 } else { 0.0 };

        if (s.b[1133] && s.b[1220]) {
            s.store_square(294, 192);
            s.store_mul_ad_product_lhs(295, A::scale(s.ad_value(227), 2.0), s.ad_value(324), 246);
            s.store_sub(296, 294, 295);
            s.store_sqrt_square_offset(44, 294, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_add(294, 294, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1221] = (s.v[294] < 0.0);
        s.v[1221] = if s.b[1221] { 1.0 } else { 0.0 };

        if ((s.b[1133] && s.b[1220]) && s.b[1221]) {
            s.store_scalar(294, 0.0);
        }

        if (s.b[1133] && s.b[1220]) {
            s.store_sqrt_square_offset(44, 296, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_add(296, 296, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1222] = (s.v[296] < 0.0);
        s.v[1222] = if s.b[1222] { 1.0 } else { 0.0 };

        if ((s.b[1133] && s.b[1220]) && s.b[1222]) {
            s.store_scalar(296, 0.0);
        }

        if (s.b[1133] && s.b[1220]) {
            s.store_sub(297, 294, 296);
        }

        s.b[1223] = ((s.v[244] < (10.0 * 2.220446049250313e-16)) || (s.v[297] < (10.0 * 2.220446049250313e-16)));
        s.v[1223] = if s.b[1223] { 1.0 } else { 0.0 };

        if ((s.b[1133] && s.b[1220]) && s.b[1223]) {
            s.store_scalar(146, 0.0);
        }

        if ((s.b[1133] && s.b[1220]) && (!s.b[1223])) {
            s.store_scalar(146, 1.0);
        }

        s.copy_ad(202, 199);

        s.v[204] = 0.0;

        s.b[1224] = ((p.p281 > 0.0) && (p.p285 > 0.0));
        s.v[1224] = if s.b[1224] { 1.0 } else { 0.0 };

        if s.b[1224] {
            s.store_scalar(1231, s.v[99]);
            s.store_scalar(1235, p.p237);
            s.store_offset_sub_ad(1236, A::add(A::offset(s.ad_value(158), (-s.v[123])), s.ad_value(185)), s.ad_value(320), (-p.p286));
            s.store_offset(1237, 182, p.p286);
            s.store_scalar(1239, p.p285);
            s.store_scalar(1238, p.p283);
            s.store_scalar(1229, s.v[70]);
            s.store_mul_ln_ad_rhs(1230, 227, A::div(A::mul(A::div(s.ad_value(1229), s.ad_value(230)), s.ad_value(536)), s.ad_value(230)));
        }

        if s.b[1224] {
            s.store_ad_value(1227, {
                if (p.p43 == 1.0) {
                    s.ad_value(435)
                } else {
                    s.ad_value(350)
                }
            });
        }

        if s.b[1224] {
            s.store_sqrt_div_ad(1232, A::mul(A::mul(A::scale(A::sub(s.ad_value(1230), s.ad_value(1227)), ((2.0 * 1.6021918e-19) * 9662367879.197212)), s.ad_value(536)), s.ad_value(1229)), A::add(s.ad_value(536), s.ad_value(1229)));
            s.store_mul(1226, 1232, 1231);
            s.store_div_ad(1225, A::mul(A::scale(s.ad_value(1226), (-0.25)), s.ad_value(1226)), A::add(s.ad_value(157), s.ad_value(1226)));
            s.copy_ad(1251, 1225);
            s.copy_ad(1252, 1237);
            s.store_offset_div_ad(336, A::scale(A::offset(A::mul(s.ad_value(225), A::sub(s.ad_value(1236), s.ad_value(1251))), (-1.0)), 4.0), A::mul(s.ad_value(241), s.ad_value(226)), 1.0);
        }

        if s.b[1224] {
            s.store_ad_value(336, {
                if (s.v[336] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(336)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if s.b[1224] {
            s.store_add_ad_rhs(376, 1236, A::mul(A::scale(A::mul(s.ad_value(241), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(336)))));
        }

        s.b[1253] = (s.v[158] < ((s.v[123] + s.v[1252]) * 0.5));
        s.v[1253] = if s.b[1253] { 1.0 } else { 0.0 };

        if (s.b[1224] && s.b[1253]) {
            s.store_scalar(144, 0.0);
        }

        s.b[1254] = ((s.v[144] == 0.0) || (1.0 != 0.0));
        s.v[1254] = if s.b[1254] { 1.0 } else { 0.0 };

        if (s.b[1224] && s.b[1254]) {
            s.store_mul_sub_rhs(181, 225, 376, 1251);
        }

        s.b[1255] = (s.v[181] < 3.0);
        s.v[1255] = if s.b[1255] { 1.0 } else { 0.0 };

        if ((s.b[1224] && s.b[1254]) && s.b[1255]) {
            s.store_mul_sub_rhs(337, 225, 1236, 1251);
            s.store_div_from_scalar_mul_ad(328, 1.0, A::scale(s.ad_value(225), (1.414213562373095 / 108.0)), s.ad_value(240));
            s.store_offset_scaled(329, 328, 3.0, 81.0);
            s.store_add_ad(330, A::sub_from_scalar((-2916.0), A::scale(s.ad_value(328), 81.0)), A::mul(A::scale(s.ad_value(328), 27.0), s.ad_value(337)));
            s.store_add_ad(331, A::sub_from_scalar(1458.0, A::scale(A::offset(s.ad_value(328), 54.0), 81.0)), A::mul(A::scale(s.ad_value(328), 27.0), s.ad_value(337)));
            s.store_square(331, 331);
            s.store_powf_ad(332, A::add(s.ad_value(330), A::sqrt(A::add(A::mul(A::mul(A::scale(s.ad_value(329), 4.0), s.ad_value(329)), s.ad_value(329)), s.ad_value(331)))), 0.3333333333333333);
            s.store_add_scaled_ad_lhs(336, A::sub_from_scalar(3.0, A::div(A::scale(s.ad_value(329), 1.259921049894873), A::scale(s.ad_value(332), 3.0))), 332, (1.0 / (3.0 * 1.259921049894873)));
            s.store_add_ad_lhs(376, A::mul(s.ad_value(336), s.ad_value(227)), 1251);
            s.copy_ad(378, 376);
        }

        s.b[1256] = ((s.v[158] - s.v[383]) <= s.v[1252]);
        s.v[1256] = if s.b[1256] { 1.0 } else { 0.0 };

        s.b[1257] = (p.p43 == 0.0);
        s.v[1257] = if s.b[1257] { 1.0 } else { 0.0 };

        if ((((s.b[1224] && s.b[1254]) && (!s.b[1255])) && s.b[1256]) && s.b[1257]) {
            s.store_div_from_scalar(327, 1.0, 323);
            s.store_scale(328, 1235, 9662367879.197212);
            s.store_scalar(329, (1.0 / s.v[93]));
            s.store_div_from_scalar_add_ad(330, 1.0, A::add(s.ad_value(327), s.ad_value(328)), s.ad_value(329));
            s.store_mul_add_ad_rhs(331, 330, A::sub(s.ad_value(1236), s.ad_value(475)), A::mul(A::add(s.ad_value(329), A::scale(s.ad_value(328), 0.5)), A::neg(s.ad_value(369))));
            s.store_sub_ad_rhs(376, 1236, A::div(s.ad_value(331), s.ad_value(323)));
        }

        if (((s.b[1224] && s.b[1254]) && (!s.b[1255])) && s.b[1256]) {
            s.copy_ad(378, 376);
        }

        if (((s.b[1224] && s.b[1254]) && (!s.b[1255])) && (!s.b[1256])) {
            s.store_div_ad_lhs(328, A::div_from_scalar(1.0, s.ad_value(379)), 434);
            s.store_mul_ad(329, A::mul(s.ad_value(328), A::sub(s.ad_value(1236), s.ad_value(383))), A::sub(s.ad_value(1236), s.ad_value(383)));
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(1236), s.ad_value(383))));
            s.store_offset_div_ad(377, A::ln(s.ad_value(329)), s.ad_value(330), p.p287);
            s.store_offset_sub(44, 377, 376, (-0.0008));
            s.store_scale(45, 377, (4.0 * 0.0008));
        }

        if (((s.b[1224] && s.b[1254]) && (!s.b[1255])) && (!s.b[1256])) {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((s.b[1224] && s.b[1254]) && (!s.b[1255])) && (!s.b[1256])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_sub_ad_rhs(378, 377, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        s.b[1258] = (p.p43 == 0.0);
        s.v[1258] = if s.b[1258] { 1.0 } else { 0.0 };

        s.b[1259] = ((s.v[158] - s.v[383]) <= s.v[1252]);
        s.v[1259] = if s.b[1259] { 1.0 } else { 0.0 };

        if (((s.b[1224] && s.b[1254]) && s.b[1258]) && s.b[1259]) {
            s.store_div_from_scalar(327, 1.0, 323);
            s.store_scale(328, 1235, 9662367879.197212);
            s.store_scalar(329, (1.0 / s.v[93]));
            s.store_div_from_scalar_add_ad(330, 1.0, A::add(s.ad_value(327), s.ad_value(328)), s.ad_value(329));
            s.store_mul_add_ad_rhs(331, 330, A::sub(s.ad_value(1236), s.ad_value(475)), A::mul(A::add(s.ad_value(329), A::scale(s.ad_value(328), 0.5)), A::neg(s.ad_value(369))));
            s.store_sub_ad_rhs(376, 1236, A::div(s.ad_value(331), s.ad_value(323)));
            s.copy_ad(378, 376);
        }

        if (((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) {
            s.store_div_from_scalar(327, 1.0, 323);
            s.store_scale(328, 1235, 9662367879.197212);
            s.store_scalar(329, (1.0 / s.v[93]));
            s.store_div_from_scalar_add_ad(330, 1.0, A::add(s.ad_value(327), s.ad_value(328)), s.ad_value(329));
            s.store_mul_add_ad_rhs(331, 330, A::sub(s.ad_value(1236), s.ad_value(475)), A::mul(A::add(s.ad_value(329), A::scale(s.ad_value(328), 0.5)), A::neg(s.ad_value(369))));
            s.store_sub_ad_rhs(376, 1236, A::div(s.ad_value(331), s.ad_value(323)));
            s.copy_ad(378, 376);
        }

        s.b[1260] = ((s.v[1236] - s.v[383]) > 0.0);
        s.v[1260] = if s.b[1260] { 1.0 } else { 0.0 };

        if ((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) {
            s.store_div_ad_lhs(328, A::div_from_scalar(1.0, s.ad_value(379)), 434);
            s.store_mul_ad(329, A::mul(s.ad_value(328), A::sub(s.ad_value(1236), s.ad_value(383))), A::sub(s.ad_value(1236), s.ad_value(383)));
            s.store_add_ad_rhs(330, 225, A::div_from_scalar(2.0, A::sub(s.ad_value(1236), s.ad_value(383))));
            s.store_offset_div_ad(377, A::ln(s.ad_value(329)), s.ad_value(330), p.p287);
        }

        s.b[1261] = ((s.v[376] > ((s.v[377] * 0.98) - 0.4)) && (0.4 >= 0.0));
        s.v[1261] = if s.b[1261] { 1.0 } else { 0.0 };

        if (((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) {
            s.store_offset_sub_ad(44, s.ad_value(376), A::scale(s.ad_value(377), 0.98), 0.4);
            s.store_square(49, 44);
            s.store_scalar(50, (0.4 * 0.4));
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
        }

    }

    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) {
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1262] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1262] = if s.b[1262] { 1.0 } else { 0.0 };

        s.b[1263] = (2.0 == 1.0);
        s.v[1263] = if s.b[1263] { 1.0 } else { 0.0 };

        if (((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) && s.b[1263]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1264] = (2.0 == 2.0);
        s.v[1264] = if s.b[1264] { 1.0 } else { 0.0 };

        if ((((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) && (!s.b[1263])) && s.b[1264]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1265] = (2.0 == 4.0);
        s.v[1265] = if s.b[1265] { 1.0 } else { 0.0 };

        if (((((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) && (!s.b[1263])) && (!s.b[1264])) && s.b[1265]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1266] = (2.0 == 8.0);
        s.v[1266] = if s.b[1266] { 1.0 } else { 0.0 };

        if ((((((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) && (!s.b[1263])) && (!s.b[1264])) && (!s.b[1265])) && s.b[1266]) {
            s.store_scalar(55, 4.0);
        }

        if ((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign20720_loop_guard: usize = 0;
        while {
            let assign20720_cond_e28554: f64 = if (((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign20720_cond_e28554 != 0.0
        } {
            assign20720_loop_guard += 1;
            assert!(assign20720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && s.b[1262]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) && (!s.b[1262])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 2.0)));
        }

        if (((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && s.b[1261]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_scaled_mul(43, 44, 53, 0.4);
            s.store_add_ad_lhs(378, A::offset(A::scale(s.ad_value(377), 0.98), (-0.4)), 43);
        }

        if (((((s.b[1224] && s.b[1254]) && s.b[1258]) && (!s.b[1259])) && s.b[1260]) && (!s.b[1261])) {
            s.copy_ad(378, 376);
        }

        if s.b[1224] {
            s.store_offset(336, 1251, (5e-12 / 2.0));
        }

        s.b[1267] = (s.v[378] < s.v[336]);
        s.v[1267] = if s.b[1267] { 1.0 } else { 0.0 };

        if (s.b[1224] && s.b[1267]) {
            s.copy_ad(378, 336);
        }

        if s.b[1224] {
            s.copy_ad(1234, 378);
            s.copy_ad(163, 376);
        }

        if (s.b[1224] && (0.0 != 0.0)) {
            s.store_ad_value(166, {
                if ((s.v[376] - s.v[1234]) >= 0.0) {
                    A::sub(s.ad_value(376), s.ad_value(1234))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.b[1224] && (0.0 != 0.0)) {
            s.store_offset_scaled_ad(44, A::scale(s.ad_value(166), (1.0 + 0.3)), 1.0, (((-p.p287)) + ((-0.03))));
            s.store_scale(45, 166, ((1.0 + 0.3) * (4.0 * 0.03)));
        }

        if (s.b[1224] && (0.0 != 0.0)) {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (s.b[1224] && (0.0 != 0.0)) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_sub_scaled_ad_rhs(165, 166, (1.0 + 0.3), A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if (s.b[1224] && (0.0 != 0.0)) {
            s.store_ad_value(165, {
                if (s.v[165] <= s.v[166]) {
                    s.ad_value(165)
                } else {
                    s.ad_value(166)
                }
            });
        }

        s.b[1268] = (s.v[165] < 0.0);
        s.v[1268] = if s.b[1268] { 1.0 } else { 0.0 };

        if ((s.b[1224] && (0.0 != 0.0)) && s.b[1268]) {
            s.store_scalar(165, 0.0);
        }

        s.b[1269] = (s.v[165] > s.v[157]);
        s.v[1269] = if s.b[1269] { 1.0 } else { 0.0 };

        if (((s.b[1224] && (0.0 != 0.0)) && (!s.b[1268])) && s.b[1269]) {
            s.copy_ad(165, 157);
        }

        if (s.b[1224] && (0.0 != 0.0)) {
            s.store_add(163, 1234, 165);
        }

        s.b[1270] = (p.p282 == 1.0);
        s.v[1270] = if s.b[1270] { 1.0 } else { 0.0 };

        if (s.b[1224] && s.b[1270]) {
            s.copy_ad(378, 1234);
            s.copy_ad(1271, 1225);
            s.store_offset_add_ad(160, A::add(A::sub_from_scalar(s.v[123], s.ad_value(185)), s.ad_value(320)), s.ad_value(1271), p.p286);
        }

        s.b[1273] = (s.v[158] < s.v[160]);
        s.v[1273] = if s.b[1273] { 1.0 } else { 0.0 };

        if ((s.b[1224] && s.b[1270]) && s.b[1273]) {
            s.store_scalar(338, (-1.0));
            s.store_mul_scaled_ad_rhs(254, 227, 2.0, A::ln(A::div_from_scalar((-s.v[139]), s.ad_value(240))));
            s.store_mul_sub_rhs(336, 225, 1236, 1271);
            s.store_div_from_scalar_mul_ad(328, 1.0, s.ad_value(225), s.ad_value(238));
            s.store_mul(337, 328, 323);
            s.store_offset_scaled(262, 337, (3.0 * 1.414213562373095), 2.0);
            s.store_mul_ad_product_lhs(260, A::scale(s.ad_value(262), 8.0), s.ad_value(262), 262);
            s.store_offset(331, 336, (-2.0));
            s.store_scaled_mul(332, 337, 331, 9.0);
            s.store_sub_from_scalar(261, (7.0 * 1.414213562373095), 332);
            s.store_square(259, 261);
        }

        s.b[1274] = (s.v[260] < (s.v[259] * 1e-8));
        s.v[1274] = if s.b[1274] { 1.0 } else { 0.0 };

        if (((s.b[1224] && s.b[1270]) && s.b[1273]) && s.b[1274]) {
            s.store_add_ad_lhs(257, A::add(A::offset(s.ad_value(261), ((-7.0) * 1.414213562373095)), A::div(A::scale(s.ad_value(260), 0.5), s.ad_value(261))), 332);
        }

        if (((s.b[1224] && s.b[1270]) && s.b[1273]) && (!s.b[1274])) {
            s.store_sqrt_add(258, 260, 259);
            s.store_add_ad_lhs(257, A::offset(s.ad_value(258), ((-7.0) * 1.414213562373095)), 332);
        }

        if ((s.b[1224] && s.b[1270]) && s.b[1273]) {
            s.store_powf(256, 257, 0.3333333333333333);
            s.store_add_ad(255, A::add(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(337), 12.0)), A::scale(s.ad_value(256), 2.0)), A::mul(A::scale(s.ad_value(256), 1.414213562373095), s.ad_value(256)));
            s.store_div_from_scalar(328, 1.0, 256);
            s.store_mul(181, 255, 328);
            s.store_add_ad_lhs(313, A::mul(s.ad_value(181), s.ad_value(227)), 1271);
            s.store_sub(328, 313, 1271);
            s.store_div(329, 328, 254);
            s.store_sqrt_square_offset(330, 329, 1.0);
            s.store_add_ad_lhs(1234, A::div(s.ad_value(328), s.ad_value(330)), 1271);
        }

        if ((s.b[1224] && s.b[1270]) && (!s.b[1273])) {
            s.store_exp_ad(484, A::mul(s.ad_value(225), A::offset(s.ad_value(1271), (-p.p287))));
            s.store_scalar(430, 0.0);
            s.copy_ad(1272, 378);
            s.store_scale_ad(419, A::scale(s.ad_value(229), (p.p237 * (p.p237 * 0.5))), 9662367879.197212);
            s.store_sqrt_mul_ad(327, A::scale(s.ad_value(225), 2.0), s.ad_value(419));
            s.store_scaled_add_ad(328, A::exp(s.ad_value(327)), A::exp(A::neg(s.ad_value(327))), 0.5);
            s.store_div_ad_lhs(420, A::ln(s.ad_value(328)), 419);
            s.store_scalar(167, 1.0);
        }

        let mut assign21320_loop_guard: usize = 0;
        while {
            let assign21320_cond_e29284: f64 = (s.v[57] + 1.0);
            let assign21320_cond_e29286: f64 = if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (s.v[167] <= assign21320_cond_e29284)) { 1.0 } else { 0.0 };
            assign21320_cond_e29286 != 0.0
        } {
            assign21320_loop_guard += 1;
            assert!(assign21320_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1224] && s.b[1270]) && (!s.b[1273])) {
                s.store_sub(417, 1272, 1271);
                s.store_mul(181, 225, 417);
                s.store_mul_sub_rhs(337, 420, 417, 419);
            }
            s.b[1275] = (s.v[337] < 80.0);
            s.v[1275] = if s.b[1275] { 1.0 } else { 0.0 };
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && s.b[1275]) {
                s.store_exp(328, 337);
                s.store_exp_ad(327, A::mul(A::neg(s.ad_value(420)), s.ad_value(419)));
                s.store_sub(329, 328, 327);
                s.store_div_ad_lhs(422, A::ln(A::offset(s.ad_value(329), 1.0)), 420);
                s.store_div_ad_rhs(423, 328, A::offset(s.ad_value(329), 1.0));
            }
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1275])) {
                s.store_sub(422, 417, 419);
                s.store_scalar(423, 1.0);
            }
            if ((s.b[1224] && s.b[1270]) && (!s.b[1273])) {
                s.store_mul(421, 225, 422);
            }
            s.b[1276] = (((s.v[181]) as f64).abs() < 1e-16);
            s.v[1276] = if s.b[1276] { 1.0 } else { 0.0 };
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && s.b[1276]) {
                s.store_sqrt_scaled_ad(327, A::sub_from_scalar(1.0, A::square(s.ad_value(423))), 0.5);
                s.store_mul(242, 181, 327);
                s.store_mul(443, 225, 327);
            }
            s.b[1277] = (s.v[181] < 0.0);
            s.v[1277] = if s.b[1277] { 1.0 } else { 0.0 };
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && s.b[1276]) && s.b[1277]) {
                s.store_neg(242, 242);
                s.store_neg(443, 443);
            }
            s.b[1278] = (((s.v[181]) as f64).abs() < 0.005);
            s.v[1278] = if s.b[1278] { 1.0 } else { 0.0 };
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1276])) && s.b[1278]) {
                s.store_mul_ad(327, A::scale(A::square(s.ad_value(181)), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(181), 0.3333333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(181), 0.25), A::sub_from_scalar(1.0, A::scale(s.ad_value(181), 0.2)))))));
                s.store_mul_sub_from_scalar_ad_rhs(328, 181, 1.0, A::mul(A::scale(s.ad_value(181), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(181), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(181), 0.25))))));
                s.store_mul_ad(329, A::scale(A::square(s.ad_value(421)), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(421), 0.3333333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(421), 0.25), A::sub_from_scalar(1.0, A::scale(s.ad_value(421), 0.2)))))));
                s.store_mul_sub_from_scalar_ad_rhs(330, 421, 1.0, A::mul(A::scale(s.ad_value(421), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(421), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(421), 0.25))))));
                s.store_sqrt_sub(242, 327, 329);
                s.store_div_ad_lhs(443, A::mul(A::scale(s.ad_value(225), 0.5), A::sub(s.ad_value(328), A::mul(s.ad_value(423), s.ad_value(330)))), 242);
            }
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1276])) && (!s.b[1278])) {
                s.store_exp_neg_input(327, 181);
                s.store_exp_neg_input(328, 421);
                s.store_sqrt_add_ad(242, A::sub(s.ad_value(181), s.ad_value(421)), A::sub(s.ad_value(327), s.ad_value(328)));
                s.store_div_ad_lhs(443, A::mul(A::scale(s.ad_value(225), 0.5), A::sub(A::sub_from_scalar(1.0, s.ad_value(327)), A::mul(s.ad_value(423), A::sub_from_scalar(1.0, s.ad_value(328))))), 242);
            }
            s.b[1279] = ((s.v[430] == 1.0) && (s.v[181] < 0.0));
            s.v[1279] = if s.b[1279] { 1.0 } else { 0.0 };
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && s.b[1279]) {
                s.store_scalar(338, (-1.0));
            }
            s.b[1280] = (s.v[181] < 0.0);
            s.v[1280] = if s.b[1280] { 1.0 } else { 0.0 };
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && s.b[1280]) {
                s.store_neg(490, 242);
                s.store_neg(491, 443);
            }
            s.b[1281] = (s.v[181] < 1e-7);
            s.v[1281] = if s.b[1281] { 1.0 } else { 0.0 };
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1280])) && s.b[1281]) {
                s.copy_ad(490, 242);
                s.copy_ad(491, 443);
            }
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1280])) && (!s.b[1281])) {
                s.store_mul_offset_rhs(501, 225, 1272, (-p.p287));
                s.store_exp(502, 501);
                s.store_mul_sub_ad_rhs(488, 379, s.ad_value(502), A::mul(s.ad_value(484), A::offset(s.ad_value(181), 1.0)));
                s.store_mul_ad(489, A::mul(s.ad_value(379), s.ad_value(225)), A::sub(s.ad_value(502), s.ad_value(484)));
                s.store_sqrt_square_add(490, 242, 488);
                s.store_div_ad_lhs(491, A::scale(A::add(A::mul(A::scale(s.ad_value(443), 2.0), s.ad_value(242)), s.ad_value(489)), 0.5), 490);
            }
            if ((s.b[1224] && s.b[1270]) && (!s.b[1273])) {
                s.store_add_ad(492, A::sub(s.ad_value(1272), s.ad_value(1236)), A::mul(s.ad_value(240), s.ad_value(490)));
                s.store_offset_mul(493, 240, 491, 1.0);
            }
            s.b[1282] = (s.v[430] == 1.0);
            s.v[1282] = if s.b[1282] { 1.0 } else { 0.0 };
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && s.b[1282]) {
                s.store_scalar(167, (s.v[57] + 1.0));
            }
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1282])) {
                s.store_div_ad_lhs(494, A::neg(s.ad_value(492)), 493);
            }
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1282])) {
                s.store_scaled_offset_ad(496, {
                    if (1.0 >= ((s.v[1272]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1272))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1283] = (((s.v[494]) as f64).abs() > s.v[496]);
            s.v[1283] = if s.b[1283] { 1.0 } else { 0.0 };
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1282])) && s.b[1283]) {
                s.store_scale(494, 496, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1282])) {
                s.store_add(1272, 1272, 494);
            }
            s.b[1284] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[492]) as f64).abs() <= 1e-8));
            s.v[1284] = if s.b[1284] { 1.0 } else { 0.0 };
            if ((((s.b[1224] && s.b[1270]) && (!s.b[1273])) && (!s.b[1282])) && s.b[1284]) {
                s.store_scalar(430, 1.0);
            }
            if ((s.b[1224] && s.b[1270]) && (!s.b[1273])) {
                s.store_offset(167, 167, 1.0);
            }
        }

        if ((s.b[1224] && s.b[1270]) && (!s.b[1273])) {
            s.copy_ad(1234, 1272);
        }

        if s.b[1224] {
            s.store_mul_scaled_ad_rhs(332, 225, -1.0, A::sub(s.ad_value(1234), s.ad_value(1225)));
        }

        if s.b[1224] {
            s.store_scalar(1249, (if (s.v[332] >= 0.0) { 1.0 } else { (-1.0) }));
        }

        if s.b[1224] {
            s.store_mul(1250, 1249, 332);
            s.store_exp(333, 332);
            s.store_sub_ad_lhs(334, A::offset(s.ad_value(333), (-1.0)), 332);
        }

        s.b[1285] = (s.v[332] > 1e-7);
        s.v[1285] = if s.b[1285] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1224] && s.b[1285]) {
            s.store_mul_scaled_ad_rhs(437, 238, -1.0, A::sqrt(s.ad_value(334)));
        }

        s.b[1286] = (s.v[1250] > 1e-7);
        s.v[1286] = if s.b[1286] { 1.0 } else { 0.0 };

        if ((s.b[1224] && (!s.b[1285])) && s.b[1286]) {
            s.store_mul_sqrt_rhs(437, 238, 334);
        }

        if ((s.b[1224] && (!s.b[1285])) && (!s.b[1286])) {
            s.store_mul_ad(437, A::scale(A::mul(A::neg(s.ad_value(1249)), s.ad_value(1250)), 0.7071067811865475), A::sqrt(A::offset(A::mul(A::scale(s.ad_value(1250), 0.3333333333333333), A::offset(A::scale(s.ad_value(1250), 0.25), 1.0)), 1.0)));
        }

        if s.b[1224] {
            s.store_sqrt_square_offset(44, 437, ((4.0 * 1e-6) * 1e-6));
            s.store_offset_scaled_add(1246, 437, 44, 0.5, (1e-10 * 1e-6));
        }

        s.b[1287] = (s.v[1246] < 0.0);
        s.v[1287] = if s.b[1287] { 1.0 } else { 0.0 };

        if (s.b[1224] && s.b[1287]) {
            s.store_scalar(1246, 0.0);
        }

        if s.b[1224] {
            s.store_scaled_div(1247, 1246, 536, (1.0 / (1.6021918e-19)));
            s.store_sub(328, 1247, 1238);
            s.store_scale(1248, 1247, 0.01);
            s.store_sqrt_add_ad(44, A::square(s.ad_value(328)), A::mul(A::scale(s.ad_value(1248), 4.0), s.ad_value(1248)));
            s.store_add_scaled_ad_lhs(329, A::scale(A::add(s.ad_value(328), s.ad_value(44)), 0.5), 1248, 1e-10);
        }

        s.b[1288] = (s.v[329] < 0.0);
        s.v[1288] = if s.b[1288] { 1.0 } else { 0.0 };

        if (s.b[1224] && s.b[1288]) {
            s.store_scalar(329, 0.0);
        }

        if s.b[1224] {
            s.store_div_ad_lhs(1245, A::mul(A::div(s.ad_value(329), s.ad_value(1247)), s.ad_value(329)), 1247);
            s.store_add_ad_lhs(1228, A::mul(A::sub(s.ad_value(1234), s.ad_value(1225)), s.ad_value(1245)), 1225);
            s.store_sub_ad(337, A::exp(A::mul(s.ad_value(225), s.ad_value(1228))), A::exp(A::mul(s.ad_value(225), A::sub(s.ad_value(1228), s.ad_value(157)))));
            s.store_sqrt_scaled_input(1241, 1229, ((2.0 * 1.6021918e-19) * 1.034943e-10));
            s.store_mul_sqrt_rhs(1242, 1241, 227);
            s.store_mul_sub_rhs(1233, 225, 1228, 1225);
        }

        s.b[1289] = ((s.v[1233] < (0.2 * s.v[225])) && ((0.2 * s.v[225]) >= 0.0));
        s.v[1289] = if s.b[1289] { 1.0 } else { 0.0 };

        if (s.b[1224] && s.b[1289]) {
            s.store_sub_ad_lhs(44, A::scale(s.ad_value(225), 0.2), 1233);
            s.store_square(49, 44);
            s.store_scaled_square(50, 225, ((0.2) * (0.2)));
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1290] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[1290] = if s.b[1290] { 1.0 } else { 0.0 };

        s.b[1291] = (1.0 == 1.0);
        s.v[1291] = if s.b[1291] { 1.0 } else { 0.0 };

        if (((s.b[1224] && s.b[1289]) && s.b[1290]) && s.b[1291]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1292] = (1.0 == 2.0);
        s.v[1292] = if s.b[1292] { 1.0 } else { 0.0 };

        if ((((s.b[1224] && s.b[1289]) && s.b[1290]) && (!s.b[1291])) && s.b[1292]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1293] = (1.0 == 4.0);
        s.v[1293] = if s.b[1293] { 1.0 } else { 0.0 };

        if (((((s.b[1224] && s.b[1289]) && s.b[1290]) && (!s.b[1291])) && (!s.b[1292])) && s.b[1293]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1294] = (1.0 == 8.0);
        s.v[1294] = if s.b[1294] { 1.0 } else { 0.0 };

        if ((((((s.b[1224] && s.b[1289]) && s.b[1290]) && (!s.b[1291])) && (!s.b[1292])) && (!s.b[1293])) && s.b[1294]) {
            s.store_scalar(55, 4.0);
        }

        if ((s.b[1224] && s.b[1289]) && s.b[1290]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign21850_loop_guard: usize = 0;
        while {
            let assign21850_cond_e30601: f64 = if (((s.b[1224] && s.b[1289]) && s.b[1290]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign21850_cond_e30601 != 0.0
        } {
            assign21850_loop_guard += 1;
            assert!(assign21850_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1224] && s.b[1289]) && s.b[1290]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if ((s.b[1224] && s.b[1289]) && (!s.b[1290])) {
            s.store_powf(53, 53, (1.0 / 2.0));
        }

        if (s.b[1224] && s.b[1289]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul_ad_product_lhs(43, s.ad_value(44), A::scale(s.ad_value(225), 0.2), 53);
            s.store_sub_ad_lhs(328, A::scale(s.ad_value(225), 0.2), 43);
        }

        if (s.b[1224] && (!s.b[1289])) {
            s.copy_ad(328, 1233);
        }

        if s.b[1224] {
            s.store_sqrt_offset_input(1243, 328, (10.0 * 2.220446049250313e-16));
            s.store_mul(1244, 1242, 1243);
            s.store_mul_div_ad_lhs(1240, A::scale(s.ad_value(227), 2.0), s.ad_value(1231), 1244);
            s.store_mul_ad_product_lhs(204, A::mul(s.ad_value(1240), s.ad_value(1239)), s.ad_value(107), 337);
            s.store_add(199, 202, 204);
        }

        s.store_add(201, 203, 204);

        s.b[1295] = ((p.p43 == 1.0) || (p.p45 == 1.0));
        s.v[1295] = if s.b[1295] { 1.0 } else { 0.0 };

        s.b[1308] = ((s.v[145] == 1.0) || (p.p25 == 0.0));
        s.v[1308] = if s.b[1308] { 1.0 } else { 0.0 };

        if (s.b[1295] && s.b[1308]) {
            s.store_scalar(263, 0.0);
        }

        s.b[1309] = ((p.p117 <= 0.0) || (s.v[73] <= 0.0));
        s.v[1309] = if s.b[1309] { 1.0 } else { 0.0 };

        if ((s.b[1295] && (!s.b[1308])) && s.b[1309]) {
            s.store_scalar(263, 0.0);
        }

        if ((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) {
            s.store_offset_sub_ad(445, A::add(A::offset(s.ad_value(174), (-s.v[136])), s.ad_value(185)), s.ad_value(320), p.p48);
        }

        s.b[1310] = (p.p44 <= 0.0);
        s.v[1310] = if s.b[1310] { 1.0 } else { 0.0 };

        if (((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && s.b[1310]) {
            s.copy_ad(1296, 445);
            s.store_square(1303, 323);
            s.copy_ad(1304, 545);
            s.store_div(1298, 1304, 1303);
            s.store_div_from_scalar(1305, 2.0, 1304);
            s.store_mul(1299, 1305, 1303);
            s.store_sub_ad(1300, A::sub(s.ad_value(1296), s.ad_value(227)), A::mul(s.ad_value(130), s.ad_value(514)));
            s.store_scale(483, 393, (p.p49 * 1.0 / (s.v[89])));
            s.store_sub_ad_rhs(1300, 1300, A::mul(s.ad_value(130), s.ad_value(483)));
            s.store_offset_mul(1302, 1299, 1300, 1.0);
            s.store_sqrt_square_offset(44, 1302, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_add(1301, 1302, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1311] = (s.v[1301] < 0.0);
        s.v[1311] = if s.b[1311] { 1.0 } else { 0.0 };

        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && s.b[1310]) && s.b[1311]) {
            s.store_scalar(1301, 0.0);
        }

        if (((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && s.b[1310]) {
            s.store_offset(1301, 1301, 1e-50);
            s.store_sqrt(1301, 1301);
            s.store_add_ad(1306, A::mul(s.ad_value(1296), s.ad_value(137)), A::mul(s.ad_value(1298), A::sub_from_scalar(1.0, s.ad_value(1301))));
            s.store_sub_ad(1307, A::add(A::scale(s.ad_value(173), p.p122), s.ad_value(176)), A::mul(A::mul(s.ad_value(131), s.ad_value(129)), s.ad_value(1306)));
            s.store_sqrt_square_offset(44, 1307, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_add(1307, 1307, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1312] = (s.v[1307] < 0.0);
        s.v[1312] = if s.b[1312] { 1.0 } else { 0.0 };

        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && s.b[1310]) && s.b[1312]) {
            s.store_scalar(1307, 0.0);
        }

        if (((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) {
            s.store_mul(1296, 134, 445);
            s.store_div_ad_rhs(1298, 545, A::square(s.ad_value(323)));
            s.store_mul_ad(1299, A::div_from_scalar(2.0, s.ad_value(545)), A::square(s.ad_value(323)));
            s.store_sub_ad(1300, A::sub(s.ad_value(1296), s.ad_value(227)), A::mul(s.ad_value(130), s.ad_value(514)));
            s.store_scale(483, 393, (p.p49 * 1.0 / (s.v[89])));
            s.store_sub_ad_rhs(1300, 1300, A::mul(s.ad_value(130), s.ad_value(483)));
            s.store_offset_mul(1301, 1299, 1300, 1.0);
            s.store_scaled_offset(1303, 1299, 1.0, 2.0);
        }

        s.b[1313] = ((s.v[1301] < (1e-50 + s.v[1303])) && (s.v[1303] >= 0.0));
        s.v[1313] = if s.b[1313] { 1.0 } else { 0.0 };

        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) {
            s.store_sub_ad_lhs(44, A::offset(s.ad_value(1303), 1e-50), 1301);
            s.store_square(49, 44);
            s.store_square(50, 1303);
            s.store_scalar(51, 1.0);
            s.store_scalar(52, 1.0);
            s.store_scalar(54, 0.0);
            s.store_scalar(55, 0.0);
            s.store_scalar(48, 0.0);
            s.store_scalar(53, 0.0);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_mul(51, 51, 49);
            s.store_mul(52, 52, 50);
            s.store_add(48, 51, 52);
            s.copy_ad(53, 48);
        }

        s.b[1314] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1314] = if s.b[1314] { 1.0 } else { 0.0 };

        s.b[1315] = (4.0 == 1.0);
        s.v[1315] = if s.b[1315] { 1.0 } else { 0.0 };

        if ((((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) && s.b[1315]) {
            s.store_scalar(55, 1.0);
        }

        s.b[1316] = (4.0 == 2.0);
        s.v[1316] = if s.b[1316] { 1.0 } else { 0.0 };

        if (((((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) && (!s.b[1315])) && s.b[1316]) {
            s.store_scalar(55, 2.0);
        }

        s.b[1317] = (4.0 == 4.0);
        s.v[1317] = if s.b[1317] { 1.0 } else { 0.0 };

        if ((((((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) && (!s.b[1315])) && (!s.b[1316])) && s.b[1317]) {
            s.store_scalar(55, 3.0);
        }

        s.b[1318] = (4.0 == 8.0);
        s.v[1318] = if s.b[1318] { 1.0 } else { 0.0 };

        if (((((((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) && (!s.b[1315])) && (!s.b[1316])) && (!s.b[1317])) && s.b[1318]) {
            s.store_scalar(55, 4.0);
        }

        if (((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) {
            s.store_scalar(54, 0.0);
        }

        let mut assign22640_loop_guard: usize = 0;
        while {
            let assign22640_cond_e31720: f64 = if ((((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) && (s.v[54] < s.v[55])) { 1.0 } else { 0.0 };
            assign22640_cond_e31720 != 0.0
        } {
            assign22640_loop_guard += 1;
            assert!(assign22640_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && s.b[1314]) {
                s.store_sqrt(53, 53);
                s.store_offset(54, 54, 1.0);
            }
        }

        if (((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) && (!s.b[1314])) {
            s.store_powf(53, 53, (1.0 / (2.0 * 4.0)));
        }

        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) {
            s.store_div_from_scalar(53, 1.0, 53);
            s.store_mul3_lhs(43, 44, 1303, 53);
        }

    }

    pub(super) fn stamp_reactive_block_21(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1313]) {
            s.store_sub_ad_lhs(1301, A::offset(s.ad_value(1303), 1e-50), 43);
        }

        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && (!s.b[1313])) {
        }

        if (((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) {
            s.store_ad_value(1301, {
                if (s.v[1301] <= 0.0) {
                    A::constant(0.0)
                } else {
                    A::sqrt(s.ad_value(1301))
                }
            });
        }

        if (((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) {
            s.store_add_ad_rhs(1306, 1296, A::mul(s.ad_value(1298), A::sub_from_scalar(1.0, s.ad_value(1301))));
            s.store_div_from_scalar_offset_input(1297, s.v[100], 131, s.v[100]);
            s.store_sub_ad(1307, A::add(A::scale(s.ad_value(173), p.p122), s.ad_value(176)), A::mul(s.ad_value(1297), s.ad_value(1306)));
            s.store_sqrt_square_offset(44, 1307, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_add(1307, 1307, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1319] = (s.v[1307] < 0.0);
        s.v[1319] = if s.b[1319] { 1.0 } else { 0.0 };

        if ((((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) && (!s.b[1310])) && s.b[1319]) {
            s.store_scalar(1307, 0.0);
        }

        if ((s.b[1295] && (!s.b[1308])) && (!s.b[1309])) {
            s.store_offset(1307, 1307, 1e-50);
            s.store_exp_ad(1297, A::div(A::neg(s.ad_value(133)), s.ad_value(1307)));
            s.store_mul_ad_product_lhs(263, A::mul(s.ad_value(132), s.ad_value(1307)), s.ad_value(199), 1297);
        }

        s.b[1320] = (((p.p25 == 1.0) && (p.p26 == 2.0)) && (p.p43 == 1.0));
        s.v[1320] = if s.b[1320] { 1.0 } else { 0.0 };

        if s.b[1320] {
            s.store_scale(1324, 227, 0.0);
            s.store_sub_scaled_ad_lhs(44, A::sub(s.ad_value(231), s.ad_value(1324)), 231, 0.01);
            s.store_scaled_square(45, 231, ((4.0) * (0.01)));
        }

        if s.b[1320] {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if s.b[1320] {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_sub_ad_rhs(1324, 231, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
            s.store_sqrt_mul_ad(1325, A::scale(s.ad_value(544), ((2.0 * 1.034943e-10) * 1.6021918e-19)), s.ad_value(227));
            s.store_mul_sub_rhs(1326, 225, 176, 1324);
        }

        if s.b[1320] {
            s.store_ad_value(1326, {
                if (s.v[1326] > 0.0) {
                    A::sqrt(s.ad_value(1326))
                } else {
                    A::neg(A::sqrt(A::neg(s.ad_value(1326))))
                }
            });
        }

        if s.b[1320] {
            s.store_sqrt_mul(1327, 225, 176);
            s.store_mul_scaled_ad_rhs(1328, 1325, -1.0, A::sub(s.ad_value(1326), s.ad_value(1327)));
            s.store_offset_sub_from_scalar_ad(44, p.p47, s.ad_value(1328), (-(p.p47 * 0.01)));
            s.store_scalar(45, ((4.0 * p.p47) * (p.p47 * 0.01)));
        }

        if s.b[1320] {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if s.b[1320] {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_sub_from_scalar_ad(393, p.p47, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
            s.store_scaled_voltage(596, ctx, nodes, Some(17), None, (1e-9 / 0.0001));
            s.copy_ad(393, 596);
        }

        s.b[1342] = (((s.v[145] == 0.0) && (s.v[263] > 0.0)) && (p.p146 != 0.0));
        s.v[1342] = if s.b[1342] { 1.0 } else { 0.0 };

        s.b[1343] = (s.v[56] < 3.0);
        s.v[1343] = if s.b[1343] { 1.0 } else { 0.0 };

        if (s.b[1342] && s.b[1343]) {
            s.store_scalar(516, 0.0);
            s.store_scalar(517, 0.0);
        }

        if (s.b[1342] && (!s.b[1343])) {
            s.store_ad_value(516, {
                if (p.p43 == 1.0) {
                    s.ad_value(156)
                } else {
                    s.ad_value(350)
                }
            });
        }

        if (s.b[1342] && (!s.b[1343])) {
            s.store_ad_value(517, {
                if (p.p43 == 1.0) {
                    s.ad_value(156)
                } else {
                    s.ad_value(353)
                }
            });
        }

        if s.b[1342] {
            s.store_offset_scaled(1329, 185, p.p147, 1.0);
            s.store_scaled_mul(1330, 1329, 263, p.p146);
            s.store_offset_mul_ad(1331, s.ad_value(225), A::sub(s.ad_value(161), s.ad_value(516)), (-1.0));
            s.store_sqrt_square_offset(44, 1331, ((4.0 * 0.1) * 0.1));
            s.store_offset_scaled_add(1331, 1331, 44, 0.5, (1e-10 * 0.1));
        }

        s.b[1344] = (s.v[1331] < 0.0);
        s.v[1344] = if s.b[1344] { 1.0 } else { 0.0 };

        if (s.b[1342] && s.b[1344]) {
            s.store_scalar(1331, 0.0);
        }

        if s.b[1342] {
            s.store_sqrt(1332, 1331);
            s.store_mul(1333, 1331, 1332);
            s.store_offset_mul_ad(1334, s.ad_value(225), A::sub(s.ad_value(162), s.ad_value(517)), (-1.0));
            s.store_sqrt_square_offset(44, 1334, ((4.0 * 0.1) * 0.1));
            s.store_offset_scaled_add(1334, 1334, 44, 0.5, (1e-10 * 0.1));
        }

        s.b[1345] = (s.v[1334] < 0.0);
        s.v[1345] = if s.b[1345] { 1.0 } else { 0.0 };

        if (s.b[1342] && s.b[1345]) {
            s.store_scalar(1334, 0.0);
        }

        if s.b[1342] {
            s.store_sqrt(1335, 1334);
            s.store_mul(1336, 1334, 1335);
            s.store_div_from_scalar(1337, 1.0, 1331);
            s.store_mul3_lhs(328, 225, 1330, 1337);
            s.store_div_from_scalar(1337, 1.0, 1334);
            s.store_mul3_lhs(1338, 225, 1330, 1337);
            s.store_mul_sub_ad_rhs(1339, 238, A::mul(s.ad_value(1336), s.ad_value(1338)), A::mul(s.ad_value(1333), s.ad_value(328)));
            s.store_mul_scaled_ad_rhs(1340, 238, 0.5, A::add(A::mul(A::neg(s.ad_value(1335)), s.ad_value(1338)), A::mul(s.ad_value(1332), s.ad_value(328))));
            s.store_add(1341, 1339, 1340);
            s.store_mul3_lhs(265, 264, 1341, 250);
        }

        s.v[1359] = (s.v[88] * 100.0);

        s.store_scale(1360, 323, 0.0001);

        s.v[1361] = (s.v[97] * 100.0);

        s.store_scale(1362, 107, 100.0);

        s.store_scale(1363, 252, 0.01);

        s.store_scale(1364, 436, 0.0001);

        s.store_scale(1365, 238, 0.0001);

        s.b[1366] = (p.p27 == 0.0);
        s.v[1366] = if s.b[1366] { 1.0 } else { 0.0 };

        s.b[1367] = (s.v[145] == 0.0);
        s.v[1367] = if s.b[1367] { 1.0 } else { 0.0 };

        if ((!s.b[1366]) && s.b[1367]) {
            s.store_offset_add(1358, 176, 173, (-(10.0 * 2.220446049250313e-16)));
            s.store_sub_scaled_ad_lhs(1348, A::add(A::offset(s.ad_value(174), (-s.v[123])), A::scale(A::sub(s.ad_value(185), s.ad_value(320)), (p.p216 * s.v[1361]))), 1358, p.p215);
            s.store_scalar(1350, (1.0 / s.v[1359]));
            s.store_mul(1349, 1348, 1350);
            s.store_scalar(1350, (1.0 / p.p217));
            s.store_offset_mul(1354, 1363, 1350, 1.0);
            s.store_mul(1357, 1349, 1354);
            s.store_sqrt_square_offset(44, 1357, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_add(1357, 1357, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1368] = (s.v[1357] < 0.0);
        s.v[1368] = if s.b[1368] { 1.0 } else { 0.0 };

        if (((!s.b[1366]) && s.b[1367]) && s.b[1368]) {
            s.store_scalar(1357, 0.0);
        }

        if ((!s.b[1366]) && s.b[1367]) {
            s.store_sqrt_square_offset(44, 174, ((4.0 * 0.001) * 0.001));
            s.store_offset_scaled_add(1350, 174, 44, 0.5, (1e-10 * 0.001));
        }

        s.b[1369] = (s.v[1350] < 0.0);
        s.v[1369] = if s.b[1369] { 1.0 } else { 0.0 };

        if (((!s.b[1366]) && s.b[1367]) && s.b[1369]) {
            s.store_scalar(1350, 0.0);
        }

        if ((!s.b[1366]) && s.b[1367]) {
            s.store_offset(1350, 1350, (-p.p226));
            s.store_scale(1346, 1350, 10.0);
            s.store_offset_square(1349, 1346, 1.0);
            s.store_sub_from_scalar_ad(1348, 1.0, A::div_from_scalar(1.0, s.ad_value(1349)));
            s.store_mul(1357, 1357, 1348);
            s.store_scale(1347, 1362, s.v[1361]);
            s.store_div_from_scalar_offset_input(1354, p.p219, 1347, p.p219);
            s.store_scalar(1353, p.p218);
            s.store_div_from_scalar_offset_input(1351, 1.0, 1357, 1e-50);
            s.store_scaled_mul(1348, 303, 1351, (-p.p214));
        }

        s.b[1370] = (s.v[1348] < (-34.0));
        s.v[1370] = if s.b[1370] { 1.0 } else { 0.0 };

        if (((!s.b[1366]) && s.b[1367]) && (!s.b[1370])) {
            s.store_exp(1349, 1348);
            s.store_mul_scale_ad_lhs(1350, A::div_from_scalar(p.p213, s.ad_value(302)), 1.6021918e-19, 1347);
            s.store_div_from_scalar(1352, 1.0, 1365);
            s.store_sqrt_mul_ad(1353, A::add(s.ad_value(1364), A::scale(s.ad_value(1360), 1e-12)), s.ad_value(1352));
            s.store_mul3_lhs(1351, 1349, 1350, 1353);
        }

        if (!s.b[1366]) {
            s.store_offset_scaled(1347, 158, (-p.p221), p.p222);
            s.store_exp_scaled_input(1349, 1347, s.v[1359]);
            s.store_scale_ad(1347, A::scale(s.ad_value(158), 1.0 / (s.v[1359])), 1.0 / (s.v[1359]));
            s.store_mul(1350, 158, 1347);
            s.store_scale(1351, 1362, (p.p220 / 1000000.0));
            s.store_sub(1348, 158, 157);
            s.store_offset_scaled(1347, 1348, (-p.p221), p.p222);
            s.store_exp_scaled_input(1349, 1347, s.v[1359]);
            s.store_scale_ad(1347, A::scale(s.ad_value(1348), 1.0 / (s.v[1359])), 1.0 / (s.v[1359]));
            s.store_mul(1350, 1348, 1347);
            s.store_scale(1351, 1362, (p.p220 / 1000000.0));
            s.store_scaled_offset_ad(1357, A::offset(A::sub(s.ad_value(513), s.ad_value(158)), s.v[123]), p.p225, 1.0 / (s.v[1359]));
            s.store_sqrt_square_offset(44, 1357, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_add(1357, 1357, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1373] = (s.v[1357] < 0.0);
        s.v[1373] = if s.b[1373] { 1.0 } else { 0.0 };

        if ((!s.b[1366]) && s.b[1373]) {
            s.store_scalar(1357, 0.0);
        }

        if (!s.b[1366]) {
            s.store_offset(1357, 1357, 1e-50);
            s.store_div_from_scalar(1348, (-p.p224), 1357);
        }

        s.b[1374] = (s.v[1348] < (-34.0));
        s.v[1374] = if s.b[1374] { 1.0 } else { 0.0 };

        if ((!s.b[1366]) && (!s.b[1374])) {
            s.store_exp(1349, 1348);
            s.store_scale(1350, 1362, (p.p223 * s.v[1361]));
        }

        s.b[1382] = (p.p28 == 0.0);
        s.v[1382] = if s.b[1382] { 1.0 } else { 0.0 };

        if (!s.b[1382]) {
            s.store_add_ad(1375, A::sub(A::scale(A::offset(s.ad_value(157), p.p210), p.p209), s.ad_value(158)), A::scale(A::add(s.ad_value(187), s.ad_value(319)), p.p211));
            s.store_scalar(1376, (1.0 / s.v[88]));
        }

    }

    pub(super) fn stamp_reactive_block_22(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (!s.b[1382]) {
            s.store_mul(1377, 1375, 1376);
            s.store_sqrt_square_offset(44, 1377, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_add(304, 1377, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1383] = (s.v[304] < 0.0);
        s.v[1383] = if s.b[1383] { 1.0 } else { 0.0 };

        if ((!s.b[1382]) && s.b[1383]) {
            s.store_scalar(304, 0.0);
        }

        if (!s.b[1382]) {
            s.store_div_from_scalar_offset_input(1378, 1.0, 304, 1e-50);
            s.store_scaled_mul(1379, 303, 1378, (-p.p208));
        }

        s.b[1384] = (s.v[1379] < (-34.0));
        s.v[1384] = if s.b[1384] { 1.0 } else { 0.0 };

        if ((!s.b[1382]) && (!s.b[1384])) {
            s.store_exp(1375, 1379);
            s.store_mul_scale_ad_lhs(1376, A::div_from_scalar(p.p207, s.ad_value(302)), 1.6021918e-19, 107);
        }

        if (!s.b[1382]) {
            s.store_sub(1381, 157, 513);
        }

        s.b[1385] = (s.v[1381] > 0.0);
        s.v[1385] = if s.b[1385] { 1.0 } else { 0.0 };

        if ((!s.b[1382]) && s.b[1385]) {
            s.store_square(1376, 1381);
            s.store_mul(331, 1376, 1381);
            s.store_offset(1379, 331, p.p212);
        }

        s.b[1393] = (p.p28 == 0.0);
        s.v[1393] = if s.b[1393] { 1.0 } else { 0.0 };

        if (!s.b[1393]) {
            s.store_add_ad(1386, A::sub(A::scale(A::sub_from_scalar(p.p210, s.ad_value(157)), p.p209), A::sub(s.ad_value(158), s.ad_value(157))), A::scale(A::add(s.ad_value(187), s.ad_value(319)), p.p211));
            s.store_scalar(1387, (1.0 / s.v[88]));
            s.store_mul(1388, 1386, 1387);
            s.store_sqrt_square_offset(44, 1388, ((4.0 * 0.01) * 0.01));
            s.store_offset_scaled_add(305, 1388, 44, 0.5, (1e-10 * 0.01));
        }

        s.b[1394] = (s.v[305] < 0.0);
        s.v[1394] = if s.b[1394] { 1.0 } else { 0.0 };

        if ((!s.b[1393]) && s.b[1394]) {
            s.store_scalar(305, 0.0);
        }

        if (!s.b[1393]) {
            s.store_div_from_scalar_offset_input(1389, 1.0, 305, 1e-50);
            s.store_scaled_mul(1390, 303, 1389, (-p.p208));
        }

        s.b[1395] = (s.v[1390] < (-34.0));
        s.v[1395] = if s.b[1395] { 1.0 } else { 0.0 };

        if ((!s.b[1393]) && (!s.b[1395])) {
            s.store_exp(1386, 1390);
            s.store_div_from_scalar(1389, 1.0, 302);
            s.store_scaled_mul(1387, 1389, 107, (p.p207 * 1.6021918e-19));
        }

        if (!s.b[1393]) {
            s.store_neg(1392, 513);
        }

        s.b[1396] = (s.v[1392] > 0.0);
        s.v[1396] = if s.b[1396] { 1.0 } else { 0.0 };

        if ((!s.b[1393]) && s.b[1396]) {
            s.store_square(1387, 1392);
            s.store_mul(331, 1387, 1392);
            s.store_offset(1390, 331, p.p212);
        }

        s.b[1397] = (p.p43 == 1.0);
        s.v[1397] = if s.b[1397] { 1.0 } else { 0.0 };

        if s.b[1397] {
            s.store_scalar(1407, s.v[91]);
            s.store_div_from_scalar(1408, 1.0, 1407);
            s.store_scalar(1464, 0.0);
            s.store_scalar(1466, 0.0);
            s.store_scalar(1468, 0.0);
            s.store_neg(1400, 534);
            s.store_mul(1401, 1400, 436);
            s.store_add_ad_rhs(331, 1401, A::mul(s.ad_value(1400), s.ad_value(437)));
            s.store_mul(470, 1401, 438);
            s.store_sub(469, 1401, 470);
            s.store_mul(468, 331, 438);
            s.store_sub(467, 331, 468);
        }

        if (s.b[1397] && (p.p24 != 0.0)) {
            s.copy_ad(521, 536);
            s.store_scalar(528, 0.0);
        }

        s.b[1477] = (1.0 == 1.0);
        s.v[1477] = if s.b[1477] { 1.0 } else { 0.0 };

        s.b[1478] = (1.0 == 2.0);
        s.v[1478] = if s.b[1478] { 1.0 } else { 0.0 };

        if ((s.b[1397] && (p.p24 != 0.0)) && s.b[1477]) {
            s.store_scale(522, 533, 0.5);
            s.store_scalar(523, p.p292);
            s.store_scalar(528, s.v[525]);
        }

        if ((s.b[1397] && (p.p24 != 0.0)) && (s.b[1478] && (!s.b[1477]))) {
            s.store_scale(522, 534, 0.5);
            s.store_scalar(523, p.p68);
            s.store_scalar(528, s.v[524]);
            s.store_scalar(528, 1.0);
        }

        s.b[1479] = (s.v[528] == 0.0);
        s.v[1479] = if s.b[1479] { 1.0 } else { 0.0 };

        if ((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) {
            s.store_mul_sqrt_ad_rhs(1427, 238, A::div(s.ad_value(521), s.ad_value(536)));
            s.store_scalar(1409, ((1.0 - -1.0) / 2.0));
            s.store_scalar(1410, ((1.0 + -1.0) / 2.0));
            s.store_add_ad(1420, A::mul(s.ad_value(461), s.ad_value(156)), A::mul(s.ad_value(462), A::sub(s.ad_value(156), s.ad_value(157))));
            s.store_add_ad(1421, A::mul(s.ad_value(461), s.ad_value(157)), A::mul(s.ad_value(462), A::neg(s.ad_value(157))));
            s.store_add_ad(1422, A::mul(s.ad_value(461), s.ad_value(158)), A::mul(s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157))));
            s.store_add_ad(1423, A::mul(s.ad_value(462), s.ad_value(158)), A::mul(s.ad_value(461), A::sub(s.ad_value(158), s.ad_value(157))));
            s.store_sub(1424, 1421, 1420);
            s.store_neg(1425, 1420);
            s.store_add_ad(1411, A::mul(s.ad_value(1409), s.ad_value(461)), A::mul(s.ad_value(1410), s.ad_value(462)));
            s.store_add_ad(1412, A::mul(s.ad_value(1409), s.ad_value(462)), A::mul(s.ad_value(1410), s.ad_value(461)));
            s.store_add_ad(1426, A::mul(s.ad_value(1411), s.ad_value(1422)), A::mul(s.ad_value(1412), s.ad_value(1423)));
            s.store_offset_add_ad(1418, A::mul(s.ad_value(1411), s.ad_value(1425)), A::mul(s.ad_value(1412), s.ad_value(1424)), (10.0 * 2.220446049250313e-16));
            s.store_neg(1398, 1418);
        }

        s.b[1480] = (s.v[1398] > s.v[141]);
        s.v[1480] = if s.b[1480] { 1.0 } else { 0.0 };

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1480]) {
            s.store_sub(1399, 1398, 141);
            s.store_sub(1400, 140, 141);
            s.store_div(44, 1399, 1400);
            s.store_square(45, 44);
            s.store_mul(46, 45, 44);
            s.store_square(47, 45);
            s.store_div_from_scalar_add_ad(1406, 1.0, A::add(A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(45)), s.ad_value(46)), s.ad_value(47));
            s.store_mul_sub_from_scalar_rhs(1406, 1400, 1.0, 1406);
            s.store_add(1403, 141, 1406);
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1480])) {
            s.copy_ad(1403, 1398);
        }

        if ((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) {
            s.store_offset_scaled(1419, 1403, -1.0, (-1e-12));
            s.store_mul(1428, 1427, 1408);
            s.store_square(1429, 1428);
            s.store_sub(1430, 1426, 523);
            s.store_div(1398, 521, 230);
            s.store_mul_ad(1431, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(s.ad_value(1398)));
            s.store_neg(1432, 1419);
        }

        s.b[1481] = (s.v[1430] < s.v[1432]);
        s.v[1481] = if s.b[1481] { 1.0 } else { 0.0 };

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1481]) {
            s.store_div_from_scalar_mul_ad(1399, 1.0, s.ad_value(225), s.ad_value(1427));
            s.store_mul(1406, 1399, 1407);
            s.store_offset_scaled(1433, 1406, (3.0 * 1.414213562373095), 2.0);
            s.store_mul_ad_product_lhs(1434, A::scale(s.ad_value(1433), 8.0), s.ad_value(1433), 1433);
            s.store_sub(1435, 237, 1431);
            s.store_mul_add_rhs(1405, 225, 1430, 1419);
            s.store_sub_from_scalar_ad(1436, (7.0 * 1.414213562373095), A::mul(A::scale(s.ad_value(1406), 9.0), A::offset(s.ad_value(1405), (-2.0))));
            s.store_square(1437, 1436);
        }

        s.b[1482] = (s.v[1434] < (s.v[1437] * 1e-8));
        s.v[1482] = if s.b[1482] { 1.0 } else { 0.0 };

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1481]) && s.b[1482]) {
            s.store_add_ad(1439, A::add(A::offset(s.ad_value(1436), ((-7.0) * 1.414213562373095)), A::div(A::scale(s.ad_value(1434), 0.5), s.ad_value(1436))), A::mul(A::scale(s.ad_value(1406), 9.0), A::offset(s.ad_value(1405), (-2.0))));
        }

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1481]) && (!s.b[1482])) {
            s.store_sqrt_add(1438, 1434, 1437);
            s.store_add_ad(1439, A::offset(s.ad_value(1438), ((-7.0) * 1.414213562373095)), A::mul(A::scale(s.ad_value(1406), 9.0), A::offset(s.ad_value(1405), (-2.0))));
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1481]) {
            s.store_powf(1440, 1439, 0.3333333333333333);
            s.store_add_ad(1441, A::add(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1406), 12.0)), A::scale(s.ad_value(1440), 2.0)), A::mul(A::scale(s.ad_value(1440), 1.414213562373095), s.ad_value(1440)));
            s.store_div(1442, 1441, 1440);
            s.store_sub_ad_lhs(1443, A::mul(s.ad_value(1442), s.ad_value(227)), 1419);
            s.store_add(1399, 1443, 1419);
            s.store_div(1400, 1399, 1435);
            s.store_sqrt_square_offset(1401, 1400, 1.0);
            s.store_sub_ad_lhs(1444, A::div(s.ad_value(1399), s.ad_value(1401)), 1419);
            s.store_sub(1400, 1430, 1444);
            s.store_mul(459, 1407, 1400);
            s.copy_ad(458, 459);
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) {
            s.store_scalar(1442, 3.0);
            s.store_sub_ad_lhs(1445, A::div(s.ad_value(1442), s.ad_value(225)), 1419);
            s.store_exp_neg_input(1406, 1442);
            s.store_offset_div_ad(1405, A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1430), s.ad_value(1419))), (-1.0)), s.ad_value(1406)), 4.0), A::mul(s.ad_value(1429), s.ad_value(226)), 1.0);
        }

        s.b[1483] = (s.v[1405] < (10.0 * 2.220446049250313e-16));
        s.v[1483] = if s.b[1483] { 1.0 } else { 0.0 };

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1483]) {
            s.store_scalar(1405, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) {
            s.store_add_ad_rhs(1445, 1430, A::mul(A::scale(A::mul(s.ad_value(1429), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1405)))));
            s.store_mul_add_rhs(1442, 225, 1445, 1419);
            s.store_exp_neg_input(1406, 1442);
            s.store_offset_div_ad(1405, A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1430), s.ad_value(1419))), (-1.0)), s.ad_value(1406)), 4.0), A::mul(s.ad_value(1429), s.ad_value(226)), 1.0);
        }

        s.b[1484] = (s.v[1405] < (10.0 * 2.220446049250313e-16));
        s.v[1484] = if s.b[1484] { 1.0 } else { 0.0 };

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1484]) {
            s.store_scalar(1405, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) {
            s.store_add_ad_rhs(1445, 1430, A::mul(A::scale(A::mul(s.ad_value(1429), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1405)))));
            s.store_mul_add_rhs(1442, 225, 1445, 1419);
        }

        s.b[1485] = (s.v[1442] < 3.0);
        s.v[1485] = if s.b[1485] { 1.0 } else { 0.0 };

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1485]) {
            s.store_scalar(1446, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(1447, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
        }

    }

    pub(super) fn stamp_reactive_block_23(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1485]) {
            s.store_offset_div_from_scalar_ad(1448, 1.0, A::mul(s.ad_value(225), s.ad_value(1428)), (1.0 / 1.414213562373095));
            s.store_div_ad_lhs(1449, A::neg(A::add(s.ad_value(1430), s.ad_value(1419))), 1428);
            s.store_add_ad(1450, A::sub(A::div(A::mul(A::square(s.ad_value(1447)), s.ad_value(1447)), A::mul(A::mul(A::scale(s.ad_value(1446), 27.0), s.ad_value(1446)), s.ad_value(1446))), A::div(A::mul(s.ad_value(1447), s.ad_value(1448)), A::mul(A::scale(s.ad_value(1446), 6.0), s.ad_value(1446)))), A::div(s.ad_value(1449), A::scale(s.ad_value(1446), 2.0)));
            s.store_div_ad(1451, A::sub(A::mul(A::scale(s.ad_value(1446), 3.0), s.ad_value(1448)), A::square(s.ad_value(1447))), A::mul(A::scale(s.ad_value(1446), 9.0), s.ad_value(1446)));
            s.store_sqrt_add_ad(1402, A::square(s.ad_value(1450)), A::mul(A::square(s.ad_value(1451)), s.ad_value(1451)));
            s.store_powf_ad(1452, A::sub(s.ad_value(1402), s.ad_value(1450)), 0.3333333333333333);
            s.store_neg_ad(1453, A::powf(A::add(s.ad_value(1450), s.ad_value(1402)), 0.3333333333333333));
            s.store_sub_ad(1405, A::add(s.ad_value(1452), s.ad_value(1453)), A::div(s.ad_value(1447), A::scale(s.ad_value(1446), 3.0)));
            s.store_sub_ad_lhs(1445, A::mul(s.ad_value(1405), s.ad_value(227)), 1419);
            s.store_mul_add_rhs(1442, 225, 1445, 1419);
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) {
            s.store_offset_add(1454, 1430, 1419, 0.1);
            s.store_offset_exp_ad(1461, A::mul(s.ad_value(225), A::neg(s.ad_value(1419))), 1e-50);
            s.store_div(1398, 230, 521);
            s.store_square(1455, 1398);
            s.store_mul(1456, 1455, 1461);
            s.store_mul(1398, 226, 1429);
            s.store_mul(1457, 225, 1454);
            s.store_add_ad(1458, A::sub(A::ln(A::add(A::mul(s.ad_value(1456), s.ad_value(1398)), A::square(s.ad_value(1457)))), A::ln(A::mul(s.ad_value(1455), s.ad_value(1398)))), A::mul(s.ad_value(225), s.ad_value(1419)));
            s.store_offset_sub(44, 1457, 1458, (-1.0));
            s.store_scale(45, 1457, 4.0);
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1399, 44, 45, 0.5, 0.5);
            s.store_scaled_sub_from_scalar_ad(1400, 1.0, A::div(A::offset(s.ad_value(44), 2.0), s.ad_value(45)), 0.5);
            s.store_sub_ad_rhs(1458, 1457, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
            s.store_sub(1457, 1457, 1458);
            s.store_add_ad_rhs(1457, 1457, A::scale(s.ad_value(225), 0.1));
            s.store_add_ad(1459, A::sub(A::ln(A::add(A::mul(s.ad_value(1456), s.ad_value(1398)), A::square(s.ad_value(1457)))), A::ln(A::mul(s.ad_value(1455), s.ad_value(1398)))), A::mul(s.ad_value(225), s.ad_value(1419)));
            s.copy_ad(1460, 1442);
            s.store_offset_sub(44, 1459, 1460, (-(0.0008 * 75.0)));
            s.store_scale(45, 1459, (4.0 * (0.0008 * 75.0)));
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1399, 44, 45, 0.5, 0.5);
            s.store_scaled_sub_from_scalar_ad(1400, 1.0, A::div(A::offset(s.ad_value(44), ((2.0 * 0.0008) * 75.0)), s.ad_value(45)), 0.5);
            s.store_sub_ad_rhs(1442, 1459, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
            s.store_sub_ad_lhs(1444, A::div(s.ad_value(1442), s.ad_value(225)), 1419);
            s.store_add_ad(1399, A::offset(s.ad_value(1442), (-1.0)), A::exp(A::neg(s.ad_value(1442))));
        }

        s.b[1486] = (s.v[1399] < (10.0 * 2.220446049250313e-16));
        s.v[1486] = if s.b[1486] { 1.0 } else { 0.0 };

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1486]) {
            s.store_scalar(1399, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) {
            s.store_sqrt(1400, 1399);
            s.store_mul(458, 1427, 1400);
            s.store_mul_sub_rhs(459, 1407, 1430, 1444);
        }

        s.b[1487] = (p.p42 == 1.0);
        s.v[1487] = if s.b[1487] { 1.0 } else { 0.0 };

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) {
            s.store_exp_ad(1461, A::mul(s.ad_value(225), A::neg(s.ad_value(1419))));
            s.store_div(1398, 230, 521);
            s.store_square(1455, 1398);
            s.store_mul(1470, 1455, 1461);
            s.store_scalar(1415, 0.0);
            s.store_scalar(167, 1.0);
        }

        let mut assign26240_loop_guard: usize = 0;
        while {
            let assign26240_cond_e35811: f64 = (2.0 * 20.0);
            let assign26240_cond_e35813: f64 = (assign26240_cond_e35811 + 1.0);
            let assign26240_cond_e35815: f64 = if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (s.v[167] <= assign26240_cond_e35813)) { 1.0 } else { 0.0 };
            assign26240_cond_e35815 != 0.0
        } {
            assign26240_loop_guard += 1;
            assert!(assign26240_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) {
                s.store_scalar(1466, 0.0);
                s.store_mul_add_rhs(1442, 225, 1444, 1419);
            }
            s.b[1488] = (s.v[1442] < 5.0);
            s.v[1488] = if s.b[1488] { 1.0 } else { 0.0 };
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && s.b[1488]) {
                s.store_mul_ad(1462, A::mul(A::square(s.ad_value(1442)), s.ad_value(1442)), A::offset(A::mul(s.ad_value(1442), A::offset(A::scale(s.ad_value(1442), 0.006115288895133179), (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_ad(1463, A::square(s.ad_value(1442)), A::offset(A::mul(s.ad_value(1442), A::offset(A::scale(s.ad_value(1442), (5.0 * 0.006115288895133179)), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
                s.store_mul3_lhs(1464, 1470, 1462, 1462);
                s.store_mul_ad_product_lhs(1465, A::scale(A::mul(s.ad_value(1470), s.ad_value(225)), 2.0), s.ad_value(1462), 1463);
                s.store_mul_ad_affine_product_rhs(1466, 1442, s.ad_value(1442), A::offset(A::mul(s.ad_value(1442), A::offset(A::mul(s.ad_value(1442), A::offset(A::scale(s.ad_value(1442), 6.36964918866352e-5), (-0.00163730162779191))), 0.0178800506338833)), (-0.117851130197758)), 1.0, 0.707106781186548);
                s.store_offset_mul_ad(1467, s.ad_value(1442), A::offset(A::mul(s.ad_value(1442), A::offset(A::mul(s.ad_value(1442), A::offset(A::scale(s.ad_value(1442), (5.0 * 6.36964918866352e-5)), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833))), (2.0 * (-0.117851130197758))), 0.707106781186548);
                s.store_sqrt_offset_ad(1468, A::add(A::square(s.ad_value(1466)), s.ad_value(1464)), 1e-50);
                s.store_div_ad(1469, A::add(A::mul(A::scale(A::mul(s.ad_value(225), s.ad_value(1467)), 2.0), s.ad_value(1466)), s.ad_value(1465)), A::scale(s.ad_value(1468), 2.0));
            }
            s.b[1489] = (s.v[1442] < 80.0);
            s.v[1489] = if s.b[1489] { 1.0 } else { 0.0 };
            if ((((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1488])) && s.b[1489]) {
                s.store_exp(243, 1442);
                s.store_mul_offset_rhs(1464, 1470, 243, (-1.0));
                s.store_mul3_lhs(1465, 1470, 225, 243);
            }
            if ((((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1488])) && (!s.b[1489])) {
                s.store_exp_mul(1471, 225, 1444);
                s.store_mul_sub_rhs(1464, 1455, 1471, 1461);
                s.store_mul3_lhs(1465, 1455, 225, 1471);
            }
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1488])) {
                s.store_sqrt_add_ad(1468, A::offset(s.ad_value(1442), (-1.0)), s.ad_value(1464));
                s.store_scaled_div_ad_lhs(1469, A::add(s.ad_value(225), s.ad_value(1465)), 1468, 0.5);
            }
            if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) {
                s.store_sub_ad(1472, A::sub(s.ad_value(1430), s.ad_value(1444)), A::mul(s.ad_value(1428), s.ad_value(1468)));
                s.store_sub_from_scalar_ad(1473, (-1.0), A::mul(s.ad_value(1428), s.ad_value(1469)));
            }
            s.b[1490] = (s.v[1415] == 1.0);
            s.v[1490] = if s.b[1490] { 1.0 } else { 0.0 };
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && s.b[1490]) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1490])) {
                s.store_div_ad_lhs(494, A::neg(s.ad_value(1472)), 1473);
            }
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1490])) {
                s.store_scaled_offset_ad(1474, {
                    if (1.0 >= ((s.v[1444]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1444))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1491] = (((s.v[494]) as f64).abs() > s.v[1474]);
            s.v[1491] = if s.b[1491] { 1.0 } else { 0.0 };
            if ((((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1490])) && s.b[1491]) {
                s.store_scale(494, 1474, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1490])) {
                s.store_add(1444, 1444, 494);
            }
            s.b[1492] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1472]) as f64).abs() <= 1e-8));
            s.v[1492] = if s.b[1492] { 1.0 } else { 0.0 };
            if ((((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1490])) && s.b[1492]) {
                s.store_scalar(1415, 1.0);
            }
            if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.b[1494] = (s.v[1442] < 5.0);
        s.v[1494] = if s.b[1494] { 1.0 } else { 0.0 };

        if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && s.b[1494]) {
            s.store_offset_square(1475, 1466, (10.0 * 2.220446049250313e-16));
            s.store_offset(1476, 1466, (10.0 * 2.220446049250313e-16));
        }

        if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) && (!s.b[1494])) {
            s.store_offset(1475, 1442, (-1.0));
            s.store_sqrt(1476, 1475);
        }

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1481])) && s.b[1487]) {
            s.store_mul(458, 1427, 1476);
            s.store_div_from_scalar_add_ad(1399, 1.0, s.ad_value(1468), s.ad_value(1476));
            s.store_mul3_lhs(460, 1427, 1464, 1399);
            s.store_add(459, 458, 460);
        }

        if ((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) {
            s.store_sub(460, 459, 458);
        }

        s.b[1496] = (1.0 == 1.0);
        s.v[1496] = if s.b[1496] { 1.0 } else { 0.0 };

        s.b[1497] = (1.0 == 2.0);
        s.v[1497] = if s.b[1497] { 1.0 } else { 0.0 };

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1496]) && (s.v[1409] != 0.0)) {
            s.store_mul_neg_lhs(463, 522, 459);
            s.store_mul_neg_lhs(465, 522, 460);
        }

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1496]) && (s.v[1410] != 0.0)) {
            s.store_mul_neg_lhs(464, 522, 459);
            s.store_mul_neg_lhs(466, 522, 460);
        }

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (s.b[1497] && (!s.b[1496]))) && (s.v[1409] != 0.0)) {
            s.store_mul_neg_lhs(467, 522, 459);
            s.store_mul_neg_lhs(469, 522, 460);
        }

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (s.b[1497] && (!s.b[1496]))) && (s.v[1410] != 0.0)) {
            s.store_mul_neg_lhs(468, 522, 459);
            s.store_mul_neg_lhs(470, 522, 460);
        }

        if ((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) {
            s.store_scalar(1409, ((1.0 - 1.0) / 2.0));
            s.store_scalar(1410, ((1.0 + 1.0) / 2.0));
            s.store_add_ad(1420, A::mul(s.ad_value(461), s.ad_value(156)), A::mul(s.ad_value(462), A::sub(s.ad_value(156), s.ad_value(157))));
            s.store_add_ad(1421, A::mul(s.ad_value(461), s.ad_value(157)), A::mul(s.ad_value(462), A::neg(s.ad_value(157))));
            s.store_add_ad(1422, A::mul(s.ad_value(461), s.ad_value(158)), A::mul(s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157))));
            s.store_add_ad(1423, A::mul(s.ad_value(462), s.ad_value(158)), A::mul(s.ad_value(461), A::sub(s.ad_value(158), s.ad_value(157))));
            s.store_sub(1424, 1421, 1420);
            s.store_neg(1425, 1420);
            s.store_add_ad(1411, A::mul(s.ad_value(1409), s.ad_value(461)), A::mul(s.ad_value(1410), s.ad_value(462)));
            s.store_add_ad(1412, A::mul(s.ad_value(1409), s.ad_value(462)), A::mul(s.ad_value(1410), s.ad_value(461)));
            s.store_add_ad(1426, A::mul(s.ad_value(1411), s.ad_value(1422)), A::mul(s.ad_value(1412), s.ad_value(1423)));
            s.store_offset_add_ad(1418, A::mul(s.ad_value(1411), s.ad_value(1425)), A::mul(s.ad_value(1412), s.ad_value(1424)), (10.0 * 2.220446049250313e-16));
            s.store_neg(1398, 1418);
        }

        s.b[1498] = (s.v[1398] > s.v[141]);
        s.v[1498] = if s.b[1498] { 1.0 } else { 0.0 };

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1498]) {
            s.store_sub(1399, 1398, 141);
            s.store_sub(1400, 140, 141);
            s.store_div(44, 1399, 1400);
            s.store_square(45, 44);
            s.store_mul(46, 45, 44);
            s.store_square(47, 45);
            s.store_div_from_scalar_add_ad(1406, 1.0, A::add(A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(45)), s.ad_value(46)), s.ad_value(47));
            s.store_mul_sub_from_scalar_rhs(1406, 1400, 1.0, 1406);
            s.store_add(1403, 141, 1406);
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1498])) {
            s.copy_ad(1403, 1398);
        }

        if ((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) {
            s.store_offset_scaled(1419, 1403, -1.0, (-1e-12));
            s.store_mul(1428, 1427, 1408);
            s.store_square(1429, 1428);
            s.store_sub(1430, 1426, 523);
            s.store_div(1398, 521, 230);
        }

    }

    pub(super) fn stamp_reactive_block_24(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) {
            s.store_mul_ad(1431, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(s.ad_value(1398)));
            s.store_neg(1432, 1419);
        }

        s.b[1499] = (s.v[1430] < s.v[1432]);
        s.v[1499] = if s.b[1499] { 1.0 } else { 0.0 };

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1499]) {
            s.store_div_from_scalar_mul_ad(1399, 1.0, s.ad_value(225), s.ad_value(1427));
            s.store_mul(1406, 1399, 1407);
            s.store_offset_scaled(1433, 1406, (3.0 * 1.414213562373095), 2.0);
            s.store_mul_ad_product_lhs(1434, A::scale(s.ad_value(1433), 8.0), s.ad_value(1433), 1433);
            s.store_sub(1435, 237, 1431);
            s.store_mul_add_rhs(1405, 225, 1430, 1419);
            s.store_sub_from_scalar_ad(1436, (7.0 * 1.414213562373095), A::mul(A::scale(s.ad_value(1406), 9.0), A::offset(s.ad_value(1405), (-2.0))));
            s.store_square(1437, 1436);
        }

        s.b[1500] = (s.v[1434] < (s.v[1437] * 1e-8));
        s.v[1500] = if s.b[1500] { 1.0 } else { 0.0 };

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1499]) && s.b[1500]) {
            s.store_add_ad(1439, A::add(A::offset(s.ad_value(1436), ((-7.0) * 1.414213562373095)), A::div(A::scale(s.ad_value(1434), 0.5), s.ad_value(1436))), A::mul(A::scale(s.ad_value(1406), 9.0), A::offset(s.ad_value(1405), (-2.0))));
        }

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1499]) && (!s.b[1500])) {
            s.store_sqrt_add(1438, 1434, 1437);
            s.store_add_ad(1439, A::offset(s.ad_value(1438), ((-7.0) * 1.414213562373095)), A::mul(A::scale(s.ad_value(1406), 9.0), A::offset(s.ad_value(1405), (-2.0))));
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1499]) {
            s.store_powf(1440, 1439, 0.3333333333333333);
            s.store_add_ad(1441, A::add(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1406), 12.0)), A::scale(s.ad_value(1440), 2.0)), A::mul(A::scale(s.ad_value(1440), 1.414213562373095), s.ad_value(1440)));
            s.store_div(1442, 1441, 1440);
            s.store_sub_ad_lhs(1443, A::mul(s.ad_value(1442), s.ad_value(227)), 1419);
            s.store_add(1399, 1443, 1419);
            s.store_div(1400, 1399, 1435);
            s.store_sqrt_square_offset(1401, 1400, 1.0);
            s.store_sub_ad_lhs(1444, A::div(s.ad_value(1399), s.ad_value(1401)), 1419);
            s.store_sub(1400, 1430, 1444);
            s.store_mul(459, 1407, 1400);
            s.copy_ad(458, 459);
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) {
            s.store_scalar(1442, 3.0);
            s.store_sub_ad_lhs(1445, A::div(s.ad_value(1442), s.ad_value(225)), 1419);
            s.store_exp_neg_input(1406, 1442);
            s.store_offset_div_ad(1405, A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1430), s.ad_value(1419))), (-1.0)), s.ad_value(1406)), 4.0), A::mul(s.ad_value(1429), s.ad_value(226)), 1.0);
        }

        s.b[1501] = (s.v[1405] < (10.0 * 2.220446049250313e-16));
        s.v[1501] = if s.b[1501] { 1.0 } else { 0.0 };

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1501]) {
            s.store_scalar(1405, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) {
            s.store_add_ad_rhs(1445, 1430, A::mul(A::scale(A::mul(s.ad_value(1429), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1405)))));
            s.store_mul_add_rhs(1442, 225, 1445, 1419);
            s.store_exp_neg_input(1406, 1442);
            s.store_offset_div_ad(1405, A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1430), s.ad_value(1419))), (-1.0)), s.ad_value(1406)), 4.0), A::mul(s.ad_value(1429), s.ad_value(226)), 1.0);
        }

        s.b[1502] = (s.v[1405] < (10.0 * 2.220446049250313e-16));
        s.v[1502] = if s.b[1502] { 1.0 } else { 0.0 };

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1502]) {
            s.store_scalar(1405, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) {
            s.store_add_ad_rhs(1445, 1430, A::mul(A::scale(A::mul(s.ad_value(1429), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1405)))));
            s.store_mul_add_rhs(1442, 225, 1445, 1419);
        }

        s.b[1503] = (s.v[1442] < 3.0);
        s.v[1503] = if s.b[1503] { 1.0 } else { 0.0 };

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1503]) {
            s.store_scalar(1446, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(1447, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
            s.store_offset_div_from_scalar_ad(1448, 1.0, A::mul(s.ad_value(225), s.ad_value(1428)), (1.0 / 1.414213562373095));
            s.store_div_ad_lhs(1449, A::neg(A::add(s.ad_value(1430), s.ad_value(1419))), 1428);
            s.store_add_ad(1450, A::sub(A::div(A::mul(A::square(s.ad_value(1447)), s.ad_value(1447)), A::mul(A::mul(A::scale(s.ad_value(1446), 27.0), s.ad_value(1446)), s.ad_value(1446))), A::div(A::mul(s.ad_value(1447), s.ad_value(1448)), A::mul(A::scale(s.ad_value(1446), 6.0), s.ad_value(1446)))), A::div(s.ad_value(1449), A::scale(s.ad_value(1446), 2.0)));
            s.store_div_ad(1451, A::sub(A::mul(A::scale(s.ad_value(1446), 3.0), s.ad_value(1448)), A::square(s.ad_value(1447))), A::mul(A::scale(s.ad_value(1446), 9.0), s.ad_value(1446)));
            s.store_sqrt_add_ad(1402, A::square(s.ad_value(1450)), A::mul(A::square(s.ad_value(1451)), s.ad_value(1451)));
            s.store_powf_ad(1452, A::sub(s.ad_value(1402), s.ad_value(1450)), 0.3333333333333333);
            s.store_neg_ad(1453, A::powf(A::add(s.ad_value(1450), s.ad_value(1402)), 0.3333333333333333));
            s.store_sub_ad(1405, A::add(s.ad_value(1452), s.ad_value(1453)), A::div(s.ad_value(1447), A::scale(s.ad_value(1446), 3.0)));
            s.store_sub_ad_lhs(1445, A::mul(s.ad_value(1405), s.ad_value(227)), 1419);
            s.store_mul_add_rhs(1442, 225, 1445, 1419);
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) {
            s.store_offset_add(1454, 1430, 1419, 0.1);
            s.store_offset_exp_ad(1461, A::mul(s.ad_value(225), A::neg(s.ad_value(1419))), 1e-50);
            s.store_div(1398, 230, 521);
            s.store_square(1455, 1398);
            s.store_mul(1456, 1455, 1461);
            s.store_mul(1398, 226, 1429);
            s.store_mul(1457, 225, 1454);
            s.store_add_ad(1458, A::sub(A::ln(A::add(A::mul(s.ad_value(1456), s.ad_value(1398)), A::square(s.ad_value(1457)))), A::ln(A::mul(s.ad_value(1455), s.ad_value(1398)))), A::mul(s.ad_value(225), s.ad_value(1419)));
            s.store_offset_sub(44, 1457, 1458, (-1.0));
            s.store_scale(45, 1457, 4.0);
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1399, 44, 45, 0.5, 0.5);
            s.store_scaled_sub_from_scalar_ad(1400, 1.0, A::div(A::offset(s.ad_value(44), 2.0), s.ad_value(45)), 0.5);
            s.store_sub_ad_rhs(1458, 1457, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
            s.store_sub(1457, 1457, 1458);
            s.store_add_ad_rhs(1457, 1457, A::scale(s.ad_value(225), 0.1));
            s.store_add_ad(1459, A::sub(A::ln(A::add(A::mul(s.ad_value(1456), s.ad_value(1398)), A::square(s.ad_value(1457)))), A::ln(A::mul(s.ad_value(1455), s.ad_value(1398)))), A::mul(s.ad_value(225), s.ad_value(1419)));
            s.copy_ad(1460, 1442);
            s.store_offset_sub(44, 1459, 1460, (-(0.0008 * 75.0)));
            s.store_scale(45, 1459, (4.0 * (0.0008 * 75.0)));
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1399, 44, 45, 0.5, 0.5);
            s.store_scaled_sub_from_scalar_ad(1400, 1.0, A::div(A::offset(s.ad_value(44), ((2.0 * 0.0008) * 75.0)), s.ad_value(45)), 0.5);
            s.store_sub_ad_rhs(1442, 1459, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
            s.store_sub_ad_lhs(1444, A::div(s.ad_value(1442), s.ad_value(225)), 1419);
            s.store_add_ad(1399, A::offset(s.ad_value(1442), (-1.0)), A::exp(A::neg(s.ad_value(1442))));
        }

        s.b[1504] = (s.v[1399] < (10.0 * 2.220446049250313e-16));
        s.v[1504] = if s.b[1504] { 1.0 } else { 0.0 };

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1504]) {
            s.store_scalar(1399, (10.0 * 2.220446049250313e-16));
        }

        if (((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) {
            s.store_sqrt(1400, 1399);
            s.store_mul(458, 1427, 1400);
            s.store_mul_sub_rhs(459, 1407, 1430, 1444);
        }

        s.b[1505] = (p.p42 == 1.0);
        s.v[1505] = if s.b[1505] { 1.0 } else { 0.0 };

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) {
            s.store_exp_ad(1461, A::mul(s.ad_value(225), A::neg(s.ad_value(1419))));
            s.store_div(1398, 230, 521);
            s.store_square(1455, 1398);
            s.store_mul(1470, 1455, 1461);
            s.store_scalar(1415, 0.0);
            s.store_scalar(167, 1.0);
        }

        let mut assign27790_loop_guard: usize = 0;
        while {
            let assign27790_cond_e38754: f64 = (2.0 * 20.0);
            let assign27790_cond_e38756: f64 = (assign27790_cond_e38754 + 1.0);
            let assign27790_cond_e38758: f64 = if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (s.v[167] <= assign27790_cond_e38756)) { 1.0 } else { 0.0 };
            assign27790_cond_e38758 != 0.0
        } {
            assign27790_loop_guard += 1;
            assert!(assign27790_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) {
                s.store_scalar(1466, 0.0);
                s.store_mul_add_rhs(1442, 225, 1444, 1419);
            }
            s.b[1506] = (s.v[1442] < 5.0);
            s.v[1506] = if s.b[1506] { 1.0 } else { 0.0 };
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && s.b[1506]) {
                s.store_mul_ad(1462, A::mul(A::square(s.ad_value(1442)), s.ad_value(1442)), A::offset(A::mul(s.ad_value(1442), A::offset(A::scale(s.ad_value(1442), 0.006115288895133179), (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_ad(1463, A::square(s.ad_value(1442)), A::offset(A::mul(s.ad_value(1442), A::offset(A::scale(s.ad_value(1442), (5.0 * 0.006115288895133179)), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
                s.store_mul3_lhs(1464, 1470, 1462, 1462);
                s.store_mul_ad_product_lhs(1465, A::scale(A::mul(s.ad_value(1470), s.ad_value(225)), 2.0), s.ad_value(1462), 1463);
                s.store_mul_ad_affine_product_rhs(1466, 1442, s.ad_value(1442), A::offset(A::mul(s.ad_value(1442), A::offset(A::mul(s.ad_value(1442), A::offset(A::scale(s.ad_value(1442), 6.36964918866352e-5), (-0.00163730162779191))), 0.0178800506338833)), (-0.117851130197758)), 1.0, 0.707106781186548);
                s.store_offset_mul_ad(1467, s.ad_value(1442), A::offset(A::mul(s.ad_value(1442), A::offset(A::mul(s.ad_value(1442), A::offset(A::scale(s.ad_value(1442), (5.0 * 6.36964918866352e-5)), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833))), (2.0 * (-0.117851130197758))), 0.707106781186548);
                s.store_sqrt_offset_ad(1468, A::add(A::square(s.ad_value(1466)), s.ad_value(1464)), 1e-50);
                s.store_div_ad(1469, A::add(A::mul(A::scale(A::mul(s.ad_value(225), s.ad_value(1467)), 2.0), s.ad_value(1466)), s.ad_value(1465)), A::scale(s.ad_value(1468), 2.0));
            }
            s.b[1507] = (s.v[1442] < 80.0);
            s.v[1507] = if s.b[1507] { 1.0 } else { 0.0 };
            if ((((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1506])) && s.b[1507]) {
                s.store_exp(243, 1442);
                s.store_mul_offset_rhs(1464, 1470, 243, (-1.0));
                s.store_mul3_lhs(1465, 1470, 225, 243);
            }
            if ((((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1506])) && (!s.b[1507])) {
                s.store_exp_mul(1471, 225, 1444);
                s.store_mul_sub_rhs(1464, 1455, 1471, 1461);
                s.store_mul3_lhs(1465, 1455, 225, 1471);
            }
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1506])) {
                s.store_sqrt_add_ad(1468, A::offset(s.ad_value(1442), (-1.0)), s.ad_value(1464));
                s.store_scaled_div_ad_lhs(1469, A::add(s.ad_value(225), s.ad_value(1465)), 1468, 0.5);
            }
            if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) {
                s.store_sub_ad(1472, A::sub(s.ad_value(1430), s.ad_value(1444)), A::mul(s.ad_value(1428), s.ad_value(1468)));
                s.store_sub_from_scalar_ad(1473, (-1.0), A::mul(s.ad_value(1428), s.ad_value(1469)));
            }
            s.b[1508] = (s.v[1415] == 1.0);
            s.v[1508] = if s.b[1508] { 1.0 } else { 0.0 };
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && s.b[1508]) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1508])) {
                s.store_div_ad_lhs(494, A::neg(s.ad_value(1472)), 1473);
            }
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1508])) {
                s.store_scaled_offset_ad(1474, {
                    if (1.0 >= ((s.v[1444]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1444))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1509] = (((s.v[494]) as f64).abs() > s.v[1474]);
            s.v[1509] = if s.b[1509] { 1.0 } else { 0.0 };
            if ((((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1508])) && s.b[1509]) {
                s.store_scale(494, 1474, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1508])) {
                s.store_add(1444, 1444, 494);
            }
            s.b[1510] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1472]) as f64).abs() <= 1e-8));
            s.v[1510] = if s.b[1510] { 1.0 } else { 0.0 };
            if ((((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1508])) && s.b[1510]) {
                s.store_scalar(1415, 1.0);
            }
            if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.b[1512] = (s.v[1442] < 5.0);
        s.v[1512] = if s.b[1512] { 1.0 } else { 0.0 };

        if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && s.b[1512]) {
            s.store_offset_square(1475, 1466, (10.0 * 2.220446049250313e-16));
            s.store_offset(1476, 1466, (10.0 * 2.220446049250313e-16));
        }

        if (((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) && (!s.b[1512])) {
            s.store_offset(1475, 1442, (-1.0));
            s.store_sqrt(1476, 1475);
        }

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) {
            s.store_mul(458, 1427, 1476);
            s.store_div_from_scalar_add_ad(1399, 1.0, s.ad_value(1468), s.ad_value(1476));
        }

    }

    pub(super) fn stamp_reactive_block_25(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (!s.b[1499])) && s.b[1505]) {
            s.store_mul3_lhs(460, 1427, 1464, 1399);
            s.store_add(459, 458, 460);
        }

        if ((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) {
            s.store_sub(460, 459, 458);
        }

        s.b[1514] = (1.0 == 1.0);
        s.v[1514] = if s.b[1514] { 1.0 } else { 0.0 };

        s.b[1515] = (1.0 == 2.0);
        s.v[1515] = if s.b[1515] { 1.0 } else { 0.0 };

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1514]) && (s.v[1409] != 0.0)) {
            s.store_mul_neg_lhs(463, 522, 459);
            s.store_mul_neg_lhs(465, 522, 460);
        }

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && s.b[1514]) && (s.v[1410] != 0.0)) {
            s.store_mul_neg_lhs(464, 522, 459);
            s.store_mul_neg_lhs(466, 522, 460);
        }

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (s.b[1515] && (!s.b[1514]))) && (s.v[1409] != 0.0)) {
            s.store_mul_neg_lhs(467, 522, 459);
            s.store_mul_neg_lhs(469, 522, 460);
        }

        if ((((s.b[1397] && (p.p24 != 0.0)) && s.b[1479]) && (s.b[1515] && (!s.b[1514]))) && (s.v[1410] != 0.0)) {
            s.store_mul_neg_lhs(468, 522, 459);
            s.store_mul_neg_lhs(470, 522, 460);
        }

        s.v[317] = p.p189;

        s.b[1518] = (s.v[145] != 0.0);
        s.v[1518] = if s.b[1518] { 1.0 } else { 0.0 };

        if s.b[1518] {
            s.store_add(1517, 157, 161);
            s.store_add_scaled_inputs(314, 1517, s.v[317], 162, (1.0 - s.v[317]));
        }

        s.b[1519] = (p.p64 != 0.0);
        s.v[1519] = if s.b[1519] { 1.0 } else { 0.0 };

        if (s.b[1518] && s.b[1519]) {
            s.store_scalar(315, 0.0);
        }

        s.b[1520] = (s.v[314] > ((s.v[161] + s.v[157]) - (10.0 * 2.220446049250313e-16)));
        s.v[1520] = if s.b[1520] { 1.0 } else { 0.0 };

        if (s.b[1518] && s.b[1520]) {
            s.store_offset_add(314, 161, 157, (-(10.0 * 2.220446049250313e-16)));
        }

        s.b[1521] = (p.p64 != 0.0);
        s.v[1521] = if s.b[1521] { 1.0 } else { 0.0 };

        s.b[1522] = (s.v[246] < 1e-15);
        s.v[1522] = if s.b[1522] { 1.0 } else { 0.0 };

        if (((!s.b[1518]) && s.b[1521]) && s.b[1522]) {
            s.store_scalar(315, 0.0);
        }

        if (((!s.b[1518]) && s.b[1521]) && (!s.b[1522])) {
            s.store_scale(1516, 227, 1.0 / (s.v[97]));
            s.store_div_from_scalar(1517, 1.0, 244);
            s.store_mul3_lhs(315, 246, 1516, 1517);
        }

        s.v[1534] = s.v[91];

        s.v[1535] = (1.0 / s.v[1534]);

        s.v[1555] = 0.0;

        s.v[1595] = 0.0;

        s.v[1593] = 0.0;

        s.v[1597] = 0.0;

        s.b[1606] = ((p.p29 >= 1.0) && (p.p188 > 0.0));
        s.v[1606] = if s.b[1606] { 1.0 } else { 0.0 };

        if ((p.p24 != 0.0) && s.b[1606]) {
            s.store_scalar(1537, p.p171);
            s.store_scalar(1538, p.p172);
            s.copy_ad(1539, 158);
            s.store_scalar(1536, p.p188);
        }

        s.b[1607] = ((s.v[69] == 0.0) && (p.p188 > 0.0));
        s.v[1607] = if s.b[1607] { 1.0 } else { 0.0 };

        if (((p.p24 != 0.0) && s.b[1606]) && s.b[1607]) {
            s.store_ad_value(1524, {
                if (p.p43 == 1.0) {
                    A::scale(s.ad_value(287), s.v[1534])
                } else {
                    A::scale(s.ad_value(108), s.v[1534])
                }
            });
        }

        if (((p.p24 != 0.0) && s.b[1606]) && s.b[1607]) {
            s.store_mul_ad(1527, A::mul(s.ad_value(1537), s.ad_value(1524)), A::add(s.ad_value(1538), s.ad_value(1539)));
            s.store_mul(1528, 1536, 1524);
            s.copy_ad(1532, 161);
            s.store_sub_from_scalar(1529, 1.2, 1532);
            s.store_sub_ad(267, A::mul(s.ad_value(158), s.ad_value(1528)), A::mul(s.ad_value(1529), s.ad_value(1527)));
            s.store_mul_ad(1527, A::mul(s.ad_value(1537), s.ad_value(1524)), A::sub(A::add(s.ad_value(1538), s.ad_value(1539)), s.ad_value(157)));
            s.store_sub(1532, 162, 157);
            s.store_sub_from_scalar(1529, 1.2, 1532);
            s.store_sub_ad(268, A::mul(A::sub(s.ad_value(158), s.ad_value(157)), s.ad_value(1528)), A::mul(s.ad_value(1527), s.ad_value(1529)));
        }

        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {
            s.store_mul_sqrt_ad_rhs(1556, 238, A::div_from_scalar(s.v[69], s.ad_value(536)));
            s.store_scalar(1540, ((1.0 - -1.0) / 2.0));
            s.store_scalar(1541, ((1.0 + -1.0) / 2.0));
        }

        s.b[1608] = (p.p43 == 1.0);
        s.v[1608] = if s.b[1608] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1608]) {
            s.store_add_ad(1550, A::mul(s.ad_value(461), s.ad_value(156)), A::mul(s.ad_value(462), A::sub(s.ad_value(156), s.ad_value(157))));
            s.store_add_ad(1551, A::mul(s.ad_value(461), s.ad_value(157)), A::mul(s.ad_value(462), A::neg(s.ad_value(157))));
            s.store_add_ad(1552, A::mul(s.ad_value(461), s.ad_value(158)), A::mul(s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157))));
            s.store_sub(1553, 1551, 1550);
            s.store_sub(1555, 1552, 1550);
            s.store_neg(1554, 1550);
            s.store_add_ad(1542, A::mul(s.ad_value(1540), s.ad_value(461)), A::mul(s.ad_value(1541), s.ad_value(462)));
            s.store_add_ad(1543, A::mul(s.ad_value(1540), s.ad_value(462)), A::mul(s.ad_value(1541), s.ad_value(461)));
            s.store_offset_add_ad(1548, A::mul(s.ad_value(1542), s.ad_value(1554)), A::mul(s.ad_value(1543), s.ad_value(1553)), (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1608])) {
            s.store_add_ad(1542, A::mul(s.ad_value(1540), s.ad_value(461)), A::mul(s.ad_value(1541), s.ad_value(462)));
            s.store_add_ad(1543, A::mul(s.ad_value(1540), s.ad_value(462)), A::mul(s.ad_value(1541), s.ad_value(461)));
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1608])) && (s.v[1540] != 0.0)) {
            s.store_add_ad(1555, A::mul(s.ad_value(461), s.ad_value(158)), A::mul(s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1608])) && (s.v[1541] != 0.0)) {
            s.store_add_ad(1555, A::mul(s.ad_value(462), s.ad_value(158)), A::mul(s.ad_value(461), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1608])) {
            s.store_scalar(1548, 0.0);
        }

        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {
            s.store_neg(1523, 1548);
        }

        s.b[1609] = (s.v[1523] > s.v[141]);
        s.v[1609] = if s.b[1609] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1609]) {
            s.store_sub(1524, 1523, 141);
            s.store_sub(1525, 140, 141);
            s.store_div(44, 1524, 1525);
            s.store_square(45, 44);
            s.store_mul(46, 45, 44);
            s.store_square(47, 45);
            s.store_div_from_scalar_add_ad(1533, 1.0, A::add(A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(45)), s.ad_value(46)), s.ad_value(47));
            s.store_mul_sub_from_scalar_rhs(1533, 1525, 1.0, 1533);
            s.store_add(1530, 141, 1533);
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1609])) {
            s.copy_ad(1530, 1523);
        }

        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {
            s.store_offset_scaled(1549, 1530, -1.0, (-1e-12));
            s.store_scale(1557, 1556, s.v[1535]);
            s.store_square(1558, 1557);
            s.store_sub_from_scalar(1559, s.v[82], 1555);
            s.store_div_from_scalar(1523, s.v[69], 230);
            s.store_mul_ad(1560, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(s.ad_value(1523)));
            s.store_neg(1561, 1549);
        }

        s.b[1610] = (s.v[1559] < s.v[1561]);
        s.v[1610] = if s.b[1610] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1610]) {
            s.store_div_from_scalar_mul_ad(1524, 1.0, s.ad_value(225), s.ad_value(1556));
            s.store_scale(1533, 1524, s.v[1534]);
            s.store_offset_scaled(1562, 1533, (3.0 * 1.414213562373095), 2.0);
            s.store_mul_ad_product_lhs(1563, A::scale(s.ad_value(1562), 8.0), s.ad_value(1562), 1562);
            s.store_sub(1564, 237, 1560);
            s.store_mul_add_rhs(1532, 225, 1559, 1549);
            s.store_sub_from_scalar_ad(1565, (7.0 * 1.414213562373095), A::mul(A::scale(s.ad_value(1533), 9.0), A::offset(s.ad_value(1532), (-2.0))));
            s.store_square(1566, 1565);
        }

        s.b[1611] = (s.v[1563] < (s.v[1566] * 1e-8));
        s.v[1611] = if s.b[1611] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1610]) && s.b[1611]) {
            s.store_add_ad(1568, A::add(A::offset(s.ad_value(1565), ((-7.0) * 1.414213562373095)), A::div(A::scale(s.ad_value(1563), 0.5), s.ad_value(1565))), A::mul(A::scale(s.ad_value(1533), 9.0), A::offset(s.ad_value(1532), (-2.0))));
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1610]) && (!s.b[1611])) {
            s.store_sqrt_add(1567, 1563, 1566);
            s.store_add_ad(1568, A::offset(s.ad_value(1567), ((-7.0) * 1.414213562373095)), A::mul(A::scale(s.ad_value(1533), 9.0), A::offset(s.ad_value(1532), (-2.0))));
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1610]) {
            s.store_powf(1569, 1568, 0.3333333333333333);
            s.store_add_ad(1570, A::add(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1533), 12.0)), A::scale(s.ad_value(1569), 2.0)), A::mul(A::scale(s.ad_value(1569), 1.414213562373095), s.ad_value(1569)));
            s.store_div(1571, 1570, 1569);
            s.store_sub_ad_lhs(1572, A::mul(s.ad_value(1571), s.ad_value(227)), 1549);
            s.store_add(1524, 1572, 1549);
            s.store_div(1525, 1524, 1564);
            s.store_sqrt_square_offset(1526, 1525, 1.0);
            s.store_sub_ad_lhs(1573, A::div(s.ad_value(1524), s.ad_value(1526)), 1549);
            s.store_sub(1525, 1559, 1573);
            s.store_scale(459, 1525, s.v[1534]);
            s.copy_ad(458, 459);
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) {
            s.store_scalar(1571, 3.0);
            s.store_sub_ad_lhs(1574, A::div(s.ad_value(1571), s.ad_value(225)), 1549);
            s.store_exp_neg_input(1533, 1571);
            s.store_offset_div_ad(1532, A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1559), s.ad_value(1549))), (-1.0)), s.ad_value(1533)), 4.0), A::mul(s.ad_value(1558), s.ad_value(226)), 1.0);
        }

        s.b[1612] = (s.v[1532] < (10.0 * 2.220446049250313e-16));
        s.v[1612] = if s.b[1612] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1612]) {
            s.store_scalar(1532, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) {
            s.store_add_ad_rhs(1574, 1559, A::mul(A::scale(A::mul(s.ad_value(1558), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1532)))));
            s.store_mul_add_rhs(1571, 225, 1574, 1549);
            s.store_exp_neg_input(1533, 1571);
            s.store_offset_div_ad(1532, A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1559), s.ad_value(1549))), (-1.0)), s.ad_value(1533)), 4.0), A::mul(s.ad_value(1558), s.ad_value(226)), 1.0);
        }

        s.b[1613] = (s.v[1532] < (10.0 * 2.220446049250313e-16));
        s.v[1613] = if s.b[1613] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1613]) {
            s.store_scalar(1532, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) {
            s.store_add_ad_rhs(1574, 1559, A::mul(A::scale(A::mul(s.ad_value(1558), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1532)))));
            s.store_mul_add_rhs(1571, 225, 1574, 1549);
        }

        s.b[1614] = (s.v[1571] < 3.0);
        s.v[1614] = if s.b[1614] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1614]) {
            s.store_scalar(1575, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(1576, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
            s.store_offset_div_from_scalar_ad(1577, 1.0, A::mul(s.ad_value(225), s.ad_value(1557)), (1.0 / 1.414213562373095));
            s.store_div_ad_lhs(1578, A::neg(A::add(s.ad_value(1559), s.ad_value(1549))), 1557);
            s.store_add_ad(1579, A::sub(A::div(A::mul(A::square(s.ad_value(1576)), s.ad_value(1576)), A::mul(A::mul(A::scale(s.ad_value(1575), 27.0), s.ad_value(1575)), s.ad_value(1575))), A::div(A::mul(s.ad_value(1576), s.ad_value(1577)), A::mul(A::scale(s.ad_value(1575), 6.0), s.ad_value(1575)))), A::div(s.ad_value(1578), A::scale(s.ad_value(1575), 2.0)));
            s.store_div_ad(1580, A::sub(A::mul(A::scale(s.ad_value(1575), 3.0), s.ad_value(1577)), A::square(s.ad_value(1576))), A::mul(A::scale(s.ad_value(1575), 9.0), s.ad_value(1575)));
            s.store_sqrt_add_ad(1528, A::square(s.ad_value(1579)), A::mul(A::square(s.ad_value(1580)), s.ad_value(1580)));
            s.store_powf_ad(1581, A::sub(s.ad_value(1528), s.ad_value(1579)), 0.3333333333333333);
            s.store_neg_ad(1582, A::powf(A::add(s.ad_value(1579), s.ad_value(1528)), 0.3333333333333333));
        }

    }

    pub(super) fn stamp_reactive_block_26(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1614]) {
            s.store_sub_ad(1532, A::add(s.ad_value(1581), s.ad_value(1582)), A::div(s.ad_value(1576), A::scale(s.ad_value(1575), 3.0)));
            s.store_sub_ad_lhs(1574, A::mul(s.ad_value(1532), s.ad_value(227)), 1549);
            s.store_mul_add_rhs(1571, 225, 1574, 1549);
        }

        s.b[1615] = (p.p41 > 0.0);
        s.v[1615] = if s.b[1615] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1615]) {
            s.store_offset_add(1583, 1559, 1549, 0.1);
            s.store_offset_exp_ad(1590, A::mul(s.ad_value(225), A::neg(s.ad_value(1549))), 1e-50);
            s.store_scale(1523, 230, 1.0 / (s.v[69]));
            s.store_square(1584, 1523);
            s.store_mul(1585, 1584, 1590);
            s.store_mul(1523, 226, 1558);
            s.store_mul(1586, 225, 1583);
            s.store_add_ad(1587, A::sub(A::ln(A::add(A::mul(s.ad_value(1585), s.ad_value(1523)), A::square(s.ad_value(1586)))), A::ln(A::mul(s.ad_value(1584), s.ad_value(1523)))), A::mul(s.ad_value(225), s.ad_value(1549)));
            s.store_offset_sub(44, 1586, 1587, (-1.0));
            s.store_scale(45, 1586, 4.0);
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1615]) {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1615]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1524, 44, 45, 0.5, 0.5);
            s.store_scaled_sub_from_scalar_ad(1525, 1.0, A::div(A::offset(s.ad_value(44), 2.0), s.ad_value(45)), 0.5);
            s.store_sub_ad_rhs(1587, 1586, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
            s.store_sub(1586, 1586, 1587);
            s.store_add_ad_rhs(1586, 1586, A::scale(s.ad_value(225), 0.1));
            s.store_add_ad(1588, A::sub(A::ln(A::add(A::mul(s.ad_value(1585), s.ad_value(1523)), A::square(s.ad_value(1586)))), A::ln(A::mul(s.ad_value(1584), s.ad_value(1523)))), A::mul(s.ad_value(225), s.ad_value(1549)));
            s.copy_ad(1589, 1571);
            s.store_offset_sub(44, 1588, 1589, (-(0.0008 * 75.0)));
            s.store_scale(45, 1588, (4.0 * (0.0008 * 75.0)));
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1615]) {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1615]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1524, 44, 45, 0.5, 0.5);
            s.store_scaled_sub_from_scalar_ad(1525, 1.0, A::div(A::offset(s.ad_value(44), ((2.0 * 0.0008) * 75.0)), s.ad_value(45)), 0.5);
            s.store_sub_ad_rhs(1571, 1588, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) {
            s.store_sub_ad_lhs(1573, A::div(s.ad_value(1571), s.ad_value(225)), 1549);
            s.store_add_ad(1524, A::offset(s.ad_value(1571), (-1.0)), A::exp(A::neg(s.ad_value(1571))));
        }

        s.b[1616] = (s.v[1524] < (10.0 * 2.220446049250313e-16));
        s.v[1616] = if s.b[1616] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1616]) {
            s.store_scalar(1524, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) {
            s.store_sqrt(1525, 1524);
            s.store_mul(458, 1556, 1525);
            s.store_scaled_sub(459, 1559, 1573, s.v[1534]);
        }

        s.b[1617] = (p.p41 == 1.0);
        s.v[1617] = if s.b[1617] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) {
            s.store_exp_ad(1590, A::mul(s.ad_value(225), A::neg(s.ad_value(1549))));
            s.store_scale(1523, 230, 1.0 / (s.v[69]));
            s.store_square(1584, 1523);
            s.store_mul(1599, 1584, 1590);
            s.store_scalar(1546, 0.0);
            s.store_scalar(1593, 0.0);
            s.store_scalar(1597, 0.0);
            s.store_scalar(167, 1.0);
        }

        let mut assign29790_loop_guard: usize = 0;
        while {
            let assign29790_cond_e42287: f64 = (2.0 * 20.0);
            let assign29790_cond_e42289: f64 = (assign29790_cond_e42287 + 1.0);
            let assign29790_cond_e42291: f64 = if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (s.v[167] <= assign29790_cond_e42289)) { 1.0 } else { 0.0 };
            assign29790_cond_e42291 != 0.0
        } {
            assign29790_loop_guard += 1;
            assert!(assign29790_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) {
                s.store_scalar(1595, 0.0);
                s.store_mul_add_rhs(1571, 225, 1573, 1549);
            }
            s.b[1618] = (s.v[1571] < 5.0);
            s.v[1618] = if s.b[1618] { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && s.b[1618]) {
                s.store_mul_ad(1591, A::mul(A::square(s.ad_value(1571)), s.ad_value(1571)), A::offset(A::mul(s.ad_value(1571), A::offset(A::scale(s.ad_value(1571), 0.006115288895133179), (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_ad(1592, A::square(s.ad_value(1571)), A::offset(A::mul(s.ad_value(1571), A::offset(A::scale(s.ad_value(1571), (5.0 * 0.006115288895133179)), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
                s.store_mul3_lhs(1593, 1599, 1591, 1591);
                s.store_mul_ad_product_lhs(1594, A::scale(A::mul(s.ad_value(1599), s.ad_value(225)), 2.0), s.ad_value(1591), 1592);
                s.store_mul_ad_affine_product_rhs(1595, 1571, s.ad_value(1571), A::offset(A::mul(s.ad_value(1571), A::offset(A::mul(s.ad_value(1571), A::offset(A::scale(s.ad_value(1571), 6.36964918866352e-5), (-0.00163730162779191))), 0.0178800506338833)), (-0.117851130197758)), 1.0, 0.707106781186548);
                s.store_offset_mul_ad(1596, s.ad_value(1571), A::offset(A::mul(s.ad_value(1571), A::offset(A::mul(s.ad_value(1571), A::offset(A::scale(s.ad_value(1571), (5.0 * 6.36964918866352e-5)), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833))), (2.0 * (-0.117851130197758))), 0.707106781186548);
                s.store_sqrt_offset_ad(1597, A::add(A::square(s.ad_value(1595)), s.ad_value(1593)), 1e-50);
                s.store_div_ad(1598, A::add(A::mul(A::scale(A::mul(s.ad_value(225), s.ad_value(1596)), 2.0), s.ad_value(1595)), s.ad_value(1594)), A::scale(s.ad_value(1597), 2.0));
            }
            s.b[1619] = (s.v[1571] < 80.0);
            s.v[1619] = if s.b[1619] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (!s.b[1618])) && s.b[1619]) {
                s.store_exp(243, 1571);
                s.store_mul_offset_rhs(1593, 1599, 243, (-1.0));
                s.store_mul3_lhs(1594, 1599, 225, 243);
            }
            if (((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (!s.b[1618])) && (!s.b[1619])) {
                s.store_exp_mul(1600, 225, 1573);
                s.store_mul_sub_rhs(1593, 1584, 1600, 1590);
                s.store_mul3_lhs(1594, 1584, 225, 1600);
            }
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (!s.b[1618])) {
                s.store_sqrt_add_ad(1597, A::offset(s.ad_value(1571), (-1.0)), s.ad_value(1593));
                s.store_scaled_div_ad_lhs(1598, A::add(s.ad_value(225), s.ad_value(1594)), 1597, 0.5);
            }
            if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) {
                s.store_sub_ad(1601, A::sub(s.ad_value(1559), s.ad_value(1573)), A::mul(s.ad_value(1557), s.ad_value(1597)));
                s.store_sub_from_scalar_ad(1602, (-1.0), A::mul(s.ad_value(1557), s.ad_value(1598)));
            }
            s.b[1620] = (s.v[1546] == 1.0);
            s.v[1620] = if s.b[1620] { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && s.b[1620]) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (!s.b[1620])) {
                s.store_div_ad_lhs(494, A::neg(s.ad_value(1601)), 1602);
            }
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (!s.b[1620])) {
                s.store_scaled_offset_ad(1603, {
                    if (1.0 >= ((s.v[1573]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1573))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1621] = (((s.v[494]) as f64).abs() > s.v[1603]);
            s.v[1621] = if s.b[1621] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (!s.b[1620])) && s.b[1621]) {
                s.store_scale(494, 1603, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (!s.b[1620])) {
                s.store_add(1573, 1573, 494);
            }
            s.b[1622] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1601]) as f64).abs() <= 1e-8));
            s.v[1622] = if s.b[1622] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (!s.b[1620])) && s.b[1622]) {
                s.store_scalar(1546, 1.0);
            }
            if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.b[1624] = (s.v[1571] < 5.0);
        s.v[1624] = if s.b[1624] { 1.0 } else { 0.0 };

        if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && s.b[1624]) {
            s.store_offset_square(1604, 1595, (10.0 * 2.220446049250313e-16));
            s.store_offset(1605, 1595, (10.0 * 2.220446049250313e-16));
        }

        if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) && (!s.b[1624])) {
            s.store_offset(1604, 1571, (-1.0));
            s.store_sqrt(1605, 1604);
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1610])) && s.b[1617]) {
            s.store_mul(458, 1556, 1605);
            s.store_div_from_scalar_add_ad(1524, 1.0, s.ad_value(1597), s.ad_value(1605));
            s.store_mul3_lhs(460, 1556, 1593, 1524);
            s.store_add(459, 458, 460);
        }

        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {
            s.store_sub(460, 459, 458);
        }

        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {
            s.store_ad_value(1527, {
                if (p.p43 == 1.0) {
                    A::mul(s.ad_value(287), s.ad_value(1536))
                } else {
                    A::mul(s.ad_value(108), s.ad_value(1536))
                }
            });
        }

        s.b[1626] = (((s.v[1542] != 0.0) && (p.p43 == 0.0)) || ((s.v[1540] != 0.0) && (p.p43 == 1.0)));
        s.v[1626] = if s.b[1626] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1626]) {
            s.store_mul(455, 1527, 459);
            s.store_mul(457, 1527, 458);
        }

        s.b[1627] = (((s.v[1543] != 0.0) && (p.p43 == 0.0)) || ((s.v[1541] != 0.0) && (p.p43 == 1.0)));
        s.v[1627] = if s.b[1627] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1627]) {
            s.store_mul(454, 1527, 459);
            s.store_mul(456, 1527, 458);
        }

        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {
            s.store_scalar(1540, ((1.0 - 1.0) / 2.0));
            s.store_scalar(1541, ((1.0 + 1.0) / 2.0));
        }

        s.b[1628] = (p.p43 == 1.0);
        s.v[1628] = if s.b[1628] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1628]) {
            s.store_add_ad(1550, A::mul(s.ad_value(461), s.ad_value(156)), A::mul(s.ad_value(462), A::sub(s.ad_value(156), s.ad_value(157))));
            s.store_add_ad(1551, A::mul(s.ad_value(461), s.ad_value(157)), A::mul(s.ad_value(462), A::neg(s.ad_value(157))));
            s.store_add_ad(1552, A::mul(s.ad_value(461), s.ad_value(158)), A::mul(s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157))));
            s.store_sub(1553, 1551, 1550);
            s.store_sub(1555, 1552, 1550);
            s.store_neg(1554, 1550);
            s.store_add_ad(1542, A::mul(s.ad_value(1540), s.ad_value(461)), A::mul(s.ad_value(1541), s.ad_value(462)));
            s.store_add_ad(1543, A::mul(s.ad_value(1540), s.ad_value(462)), A::mul(s.ad_value(1541), s.ad_value(461)));
            s.store_offset_add_ad(1548, A::mul(s.ad_value(1542), s.ad_value(1554)), A::mul(s.ad_value(1543), s.ad_value(1553)), (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1628])) {
            s.store_add_ad(1542, A::mul(s.ad_value(1540), s.ad_value(461)), A::mul(s.ad_value(1541), s.ad_value(462)));
            s.store_add_ad(1543, A::mul(s.ad_value(1540), s.ad_value(462)), A::mul(s.ad_value(1541), s.ad_value(461)));
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1628])) && (s.v[1540] != 0.0)) {
            s.store_add_ad(1555, A::mul(s.ad_value(461), s.ad_value(158)), A::mul(s.ad_value(462), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1628])) && (s.v[1541] != 0.0)) {
            s.store_add_ad(1555, A::mul(s.ad_value(462), s.ad_value(158)), A::mul(s.ad_value(461), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1628])) {
            s.store_scalar(1548, 0.0);
        }

        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {
            s.store_neg(1523, 1548);
        }

        s.b[1629] = (s.v[1523] > s.v[141]);
        s.v[1629] = if s.b[1629] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1629]) {
            s.store_sub(1524, 1523, 141);
            s.store_sub(1525, 140, 141);
            s.store_div(44, 1524, 1525);
            s.store_square(45, 44);
            s.store_mul(46, 45, 44);
            s.store_square(47, 45);
            s.store_div_from_scalar_add_ad(1533, 1.0, A::add(A::add(A::offset(s.ad_value(44), 1.0), s.ad_value(45)), s.ad_value(46)), s.ad_value(47));
            s.store_mul_sub_from_scalar_rhs(1533, 1525, 1.0, 1533);
            s.store_add(1530, 141, 1533);
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1629])) {
            s.copy_ad(1530, 1523);
        }

        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {
            s.store_offset_scaled(1549, 1530, -1.0, (-1e-12));
            s.store_scale(1557, 1556, s.v[1535]);
            s.store_square(1558, 1557);
            s.store_sub_from_scalar(1559, s.v[82], 1555);
            s.store_div_from_scalar(1523, s.v[69], 230);
            s.store_mul_ad(1560, A::div_from_scalar(2.0, s.ad_value(225)), A::ln(s.ad_value(1523)));
        }

    }

    pub(super) fn stamp_reactive_block_27(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {
            s.store_neg(1561, 1549);
        }

        s.b[1630] = (s.v[1559] < s.v[1561]);
        s.v[1630] = if s.b[1630] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1630]) {
            s.store_div_from_scalar_mul_ad(1524, 1.0, s.ad_value(225), s.ad_value(1556));
            s.store_scale(1533, 1524, s.v[1534]);
            s.store_offset_scaled(1562, 1533, (3.0 * 1.414213562373095), 2.0);
            s.store_mul_ad_product_lhs(1563, A::scale(s.ad_value(1562), 8.0), s.ad_value(1562), 1562);
            s.store_sub(1564, 237, 1560);
            s.store_mul_add_rhs(1532, 225, 1559, 1549);
            s.store_sub_from_scalar_ad(1565, (7.0 * 1.414213562373095), A::mul(A::scale(s.ad_value(1533), 9.0), A::offset(s.ad_value(1532), (-2.0))));
            s.store_square(1566, 1565);
        }

        s.b[1631] = (s.v[1563] < (s.v[1566] * 1e-8));
        s.v[1631] = if s.b[1631] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1630]) && s.b[1631]) {
            s.store_add_ad(1568, A::add(A::offset(s.ad_value(1565), ((-7.0) * 1.414213562373095)), A::div(A::scale(s.ad_value(1563), 0.5), s.ad_value(1565))), A::mul(A::scale(s.ad_value(1533), 9.0), A::offset(s.ad_value(1532), (-2.0))));
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1630]) && (!s.b[1631])) {
            s.store_sqrt_add(1567, 1563, 1566);
            s.store_add_ad(1568, A::offset(s.ad_value(1567), ((-7.0) * 1.414213562373095)), A::mul(A::scale(s.ad_value(1533), 9.0), A::offset(s.ad_value(1532), (-2.0))));
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1630]) {
            s.store_powf(1569, 1568, 0.3333333333333333);
            s.store_add_ad(1570, A::add(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(1533), 12.0)), A::scale(s.ad_value(1569), 2.0)), A::mul(A::scale(s.ad_value(1569), 1.414213562373095), s.ad_value(1569)));
            s.store_div(1571, 1570, 1569);
            s.store_sub_ad_lhs(1572, A::mul(s.ad_value(1571), s.ad_value(227)), 1549);
            s.store_add(1524, 1572, 1549);
            s.store_div(1525, 1524, 1564);
            s.store_sqrt_square_offset(1526, 1525, 1.0);
            s.store_sub_ad_lhs(1573, A::div(s.ad_value(1524), s.ad_value(1526)), 1549);
            s.store_sub(1525, 1559, 1573);
            s.store_scale(459, 1525, s.v[1534]);
            s.copy_ad(458, 459);
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) {
            s.store_scalar(1571, 3.0);
            s.store_sub_ad_lhs(1574, A::div(s.ad_value(1571), s.ad_value(225)), 1549);
            s.store_exp_neg_input(1533, 1571);
            s.store_offset_div_ad(1532, A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1559), s.ad_value(1549))), (-1.0)), s.ad_value(1533)), 4.0), A::mul(s.ad_value(1558), s.ad_value(226)), 1.0);
        }

        s.b[1632] = (s.v[1532] < (10.0 * 2.220446049250313e-16));
        s.v[1632] = if s.b[1632] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1632]) {
            s.store_scalar(1532, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) {
            s.store_add_ad_rhs(1574, 1559, A::mul(A::scale(A::mul(s.ad_value(1558), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1532)))));
            s.store_mul_add_rhs(1571, 225, 1574, 1549);
            s.store_exp_neg_input(1533, 1571);
            s.store_offset_div_ad(1532, A::scale(A::add(A::offset(A::mul(s.ad_value(225), A::add(s.ad_value(1559), s.ad_value(1549))), (-1.0)), s.ad_value(1533)), 4.0), A::mul(s.ad_value(1558), s.ad_value(226)), 1.0);
        }

        s.b[1633] = (s.v[1532] < (10.0 * 2.220446049250313e-16));
        s.v[1633] = if s.b[1633] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1633]) {
            s.store_scalar(1532, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) {
            s.store_add_ad_rhs(1574, 1559, A::mul(A::scale(A::mul(s.ad_value(1558), s.ad_value(225)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(1532)))));
            s.store_mul_add_rhs(1571, 225, 1574, 1549);
        }

        s.b[1634] = (s.v[1571] < 3.0);
        s.v[1634] = if s.b[1634] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1634]) {
            s.store_scalar(1575, ((1.0 / (9.0 * 1.414213562373095)) - ((5.0 + (7.0 * 0.049787068367863944)) / (54.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt()))));
            s.store_scalar(1576, (((1.0 + 0.049787068367863944) / (2.0 * (((2.0 + 0.049787068367863944)) as f64).sqrt())) - (1.414213562373095 / 3.0)));
            s.store_offset_div_from_scalar_ad(1577, 1.0, A::mul(s.ad_value(225), s.ad_value(1557)), (1.0 / 1.414213562373095));
            s.store_div_ad_lhs(1578, A::neg(A::add(s.ad_value(1559), s.ad_value(1549))), 1557);
            s.store_add_ad(1579, A::sub(A::div(A::mul(A::square(s.ad_value(1576)), s.ad_value(1576)), A::mul(A::mul(A::scale(s.ad_value(1575), 27.0), s.ad_value(1575)), s.ad_value(1575))), A::div(A::mul(s.ad_value(1576), s.ad_value(1577)), A::mul(A::scale(s.ad_value(1575), 6.0), s.ad_value(1575)))), A::div(s.ad_value(1578), A::scale(s.ad_value(1575), 2.0)));
            s.store_div_ad(1580, A::sub(A::mul(A::scale(s.ad_value(1575), 3.0), s.ad_value(1577)), A::square(s.ad_value(1576))), A::mul(A::scale(s.ad_value(1575), 9.0), s.ad_value(1575)));
            s.store_sqrt_add_ad(1528, A::square(s.ad_value(1579)), A::mul(A::square(s.ad_value(1580)), s.ad_value(1580)));
            s.store_powf_ad(1581, A::sub(s.ad_value(1528), s.ad_value(1579)), 0.3333333333333333);
            s.store_neg_ad(1582, A::powf(A::add(s.ad_value(1579), s.ad_value(1528)), 0.3333333333333333));
            s.store_sub_ad(1532, A::add(s.ad_value(1581), s.ad_value(1582)), A::div(s.ad_value(1576), A::scale(s.ad_value(1575), 3.0)));
            s.store_sub_ad_lhs(1574, A::mul(s.ad_value(1532), s.ad_value(227)), 1549);
            s.store_mul_add_rhs(1571, 225, 1574, 1549);
        }

        s.b[1635] = (p.p41 > 0.0);
        s.v[1635] = if s.b[1635] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1635]) {
            s.store_offset_add(1583, 1559, 1549, 0.1);
            s.store_offset_exp_ad(1590, A::mul(s.ad_value(225), A::neg(s.ad_value(1549))), 1e-50);
            s.store_scale(1523, 230, 1.0 / (s.v[69]));
            s.store_square(1584, 1523);
            s.store_mul(1585, 1584, 1590);
            s.store_mul(1523, 226, 1558);
            s.store_mul(1586, 225, 1583);
            s.store_add_ad(1587, A::sub(A::ln(A::add(A::mul(s.ad_value(1585), s.ad_value(1523)), A::square(s.ad_value(1586)))), A::ln(A::mul(s.ad_value(1584), s.ad_value(1523)))), A::mul(s.ad_value(225), s.ad_value(1549)));
            s.store_offset_sub(44, 1586, 1587, (-1.0));
            s.store_scale(45, 1586, 4.0);
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1635]) {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1635]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1524, 44, 45, 0.5, 0.5);
            s.store_scaled_sub_from_scalar_ad(1525, 1.0, A::div(A::offset(s.ad_value(44), 2.0), s.ad_value(45)), 0.5);
            s.store_sub_ad_rhs(1587, 1586, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
            s.store_sub(1586, 1586, 1587);
            s.store_add_ad_rhs(1586, 1586, A::scale(s.ad_value(225), 0.1));
            s.store_add_ad(1588, A::sub(A::ln(A::add(A::mul(s.ad_value(1585), s.ad_value(1523)), A::square(s.ad_value(1586)))), A::ln(A::mul(s.ad_value(1584), s.ad_value(1523)))), A::mul(s.ad_value(225), s.ad_value(1549)));
            s.copy_ad(1589, 1571);
            s.store_offset_sub(44, 1588, 1589, (-(0.0008 * 75.0)));
            s.store_scale(45, 1588, (4.0 * (0.0008 * 75.0)));
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1635]) {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1635]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_offset_scaled_div(1524, 44, 45, 0.5, 0.5);
            s.store_scaled_sub_from_scalar_ad(1525, 1.0, A::div(A::offset(s.ad_value(44), ((2.0 * 0.0008) * 75.0)), s.ad_value(45)), 0.5);
            s.store_sub_ad_rhs(1571, 1588, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) {
            s.store_sub_ad_lhs(1573, A::div(s.ad_value(1571), s.ad_value(225)), 1549);
            s.store_add_ad(1524, A::offset(s.ad_value(1571), (-1.0)), A::exp(A::neg(s.ad_value(1571))));
        }

        s.b[1636] = (s.v[1524] < (10.0 * 2.220446049250313e-16));
        s.v[1636] = if s.b[1636] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1636]) {
            s.store_scalar(1524, (10.0 * 2.220446049250313e-16));
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) {
            s.store_sqrt(1525, 1524);
            s.store_mul(458, 1556, 1525);
            s.store_scaled_sub(459, 1559, 1573, s.v[1534]);
        }

        s.b[1637] = (p.p41 == 1.0);
        s.v[1637] = if s.b[1637] { 1.0 } else { 0.0 };

        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) {
            s.store_exp_ad(1590, A::mul(s.ad_value(225), A::neg(s.ad_value(1549))));
            s.store_scale(1523, 230, 1.0 / (s.v[69]));
            s.store_square(1584, 1523);
            s.store_mul(1599, 1584, 1590);
            s.store_scalar(1546, 0.0);
            s.store_scalar(1593, 0.0);
            s.store_scalar(1597, 0.0);
            s.store_scalar(167, 1.0);
        }

        let mut assign31390_loop_guard: usize = 0;
        while {
            let assign31390_cond_e45523: f64 = (2.0 * 20.0);
            let assign31390_cond_e45525: f64 = (assign31390_cond_e45523 + 1.0);
            let assign31390_cond_e45527: f64 = if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (s.v[167] <= assign31390_cond_e45525)) { 1.0 } else { 0.0 };
            assign31390_cond_e45527 != 0.0
        } {
            assign31390_loop_guard += 1;
            assert!(assign31390_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) {
                s.store_scalar(1595, 0.0);
                s.store_mul_add_rhs(1571, 225, 1573, 1549);
            }
            s.b[1638] = (s.v[1571] < 5.0);
            s.v[1638] = if s.b[1638] { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && s.b[1638]) {
                s.store_mul_ad(1591, A::mul(A::square(s.ad_value(1571)), s.ad_value(1571)), A::offset(A::mul(s.ad_value(1571), A::offset(A::scale(s.ad_value(1571), 0.006115288895133179), (-0.07053654284009761))), 0.29693154855771));
                s.store_mul_ad(1592, A::square(s.ad_value(1571)), A::offset(A::mul(s.ad_value(1571), A::offset(A::scale(s.ad_value(1571), (5.0 * 0.006115288895133179)), (4.0 * (-0.07053654284009761)))), (3.0 * 0.29693154855771)));
                s.store_mul3_lhs(1593, 1599, 1591, 1591);
                s.store_mul_ad_product_lhs(1594, A::scale(A::mul(s.ad_value(1599), s.ad_value(225)), 2.0), s.ad_value(1591), 1592);
                s.store_mul_ad_affine_product_rhs(1595, 1571, s.ad_value(1571), A::offset(A::mul(s.ad_value(1571), A::offset(A::mul(s.ad_value(1571), A::offset(A::scale(s.ad_value(1571), 6.36964918866352e-5), (-0.00163730162779191))), 0.0178800506338833)), (-0.117851130197758)), 1.0, 0.707106781186548);
                s.store_offset_mul_ad(1596, s.ad_value(1571), A::offset(A::mul(s.ad_value(1571), A::offset(A::mul(s.ad_value(1571), A::offset(A::scale(s.ad_value(1571), (5.0 * 6.36964918866352e-5)), (4.0 * (-0.00163730162779191)))), (3.0 * 0.0178800506338833))), (2.0 * (-0.117851130197758))), 0.707106781186548);
                s.store_sqrt_offset_ad(1597, A::add(A::square(s.ad_value(1595)), s.ad_value(1593)), 1e-50);
                s.store_div_ad(1598, A::add(A::mul(A::scale(A::mul(s.ad_value(225), s.ad_value(1596)), 2.0), s.ad_value(1595)), s.ad_value(1594)), A::scale(s.ad_value(1597), 2.0));
            }
            s.b[1639] = (s.v[1571] < 80.0);
            s.v[1639] = if s.b[1639] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (!s.b[1638])) && s.b[1639]) {
                s.store_exp(243, 1571);
                s.store_mul_offset_rhs(1593, 1599, 243, (-1.0));
                s.store_mul3_lhs(1594, 1599, 225, 243);
            }
            if (((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (!s.b[1638])) && (!s.b[1639])) {
                s.store_exp_mul(1600, 225, 1573);
                s.store_mul_sub_rhs(1593, 1584, 1600, 1590);
                s.store_mul3_lhs(1594, 1584, 225, 1600);
            }
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (!s.b[1638])) {
                s.store_sqrt_add_ad(1597, A::offset(s.ad_value(1571), (-1.0)), s.ad_value(1593));
                s.store_scaled_div_ad_lhs(1598, A::add(s.ad_value(225), s.ad_value(1594)), 1597, 0.5);
            }
            if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) {
                s.store_sub_ad(1601, A::sub(s.ad_value(1559), s.ad_value(1573)), A::mul(s.ad_value(1557), s.ad_value(1597)));
                s.store_sub_from_scalar_ad(1602, (-1.0), A::mul(s.ad_value(1557), s.ad_value(1598)));
            }
            s.b[1640] = (s.v[1546] == 1.0);
            s.v[1640] = if s.b[1640] { 1.0 } else { 0.0 };
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && s.b[1640]) {
                s.store_scalar(167, ((2.0 * 20.0) + 1.0));
            }
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (!s.b[1640])) {
                s.store_div_ad_lhs(494, A::neg(s.ad_value(1601)), 1602);
            }
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (!s.b[1640])) {
                s.store_scaled_offset_ad(1603, {
                    if (1.0 >= ((s.v[1573]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(1573))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[1641] = (((s.v[494]) as f64).abs() > s.v[1603]);
            s.v[1641] = if s.b[1641] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (!s.b[1640])) && s.b[1641]) {
                s.store_scale(494, 1603, (if (s.v[494] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (!s.b[1640])) {
                s.store_add(1573, 1573, 494);
            }
            s.b[1642] = ((((s.v[494]) as f64).abs() <= 5e-12) && (((s.v[1601]) as f64).abs() <= 1e-8));
            s.v[1642] = if s.b[1642] { 1.0 } else { 0.0 };
            if (((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (!s.b[1640])) && s.b[1642]) {
                s.store_scalar(1546, 1.0);
            }
            if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) {
                s.store_offset(167, 167, 1.0);
            }
        }

        s.b[1644] = (s.v[1571] < 5.0);
        s.v[1644] = if s.b[1644] { 1.0 } else { 0.0 };

        if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && s.b[1644]) {
            s.store_offset_square(1604, 1595, (10.0 * 2.220446049250313e-16));
            s.store_offset(1605, 1595, (10.0 * 2.220446049250313e-16));
        }

        if ((((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) && (!s.b[1644])) {
            s.store_offset(1604, 1571, (-1.0));
            s.store_sqrt(1605, 1604);
        }

    }

    pub(super) fn stamp_reactive_block_28(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && (!s.b[1630])) && s.b[1637]) {
            s.store_mul(458, 1556, 1605);
            s.store_div_from_scalar_add_ad(1524, 1.0, s.ad_value(1597), s.ad_value(1605));
            s.store_mul3_lhs(460, 1556, 1593, 1524);
            s.store_add(459, 458, 460);
        }

        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {
            s.store_sub(460, 459, 458);
        }

        if (((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) {
            s.store_ad_value(1527, {
                if (p.p43 == 1.0) {
                    A::mul(s.ad_value(287), s.ad_value(1536))
                } else {
                    A::mul(s.ad_value(108), s.ad_value(1536))
                }
            });
        }

        s.b[1646] = (((s.v[1542] != 0.0) && (p.p43 == 0.0)) || ((s.v[1540] != 0.0) && (p.p43 == 1.0)));
        s.v[1646] = if s.b[1646] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1646]) {
            s.store_mul(455, 1527, 459);
            s.store_mul(457, 1527, 458);
        }

        s.b[1647] = (((s.v[1543] != 0.0) && (p.p43 == 0.0)) || ((s.v[1541] != 0.0) && (p.p43 == 1.0)));
        s.v[1647] = if s.b[1647] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1606]) && (!s.b[1607])) && s.b[1647]) {
            s.store_mul(454, 1527, 459);
            s.store_mul(456, 1527, 458);
        }

        if ((p.p24 != 0.0) && s.b[1606]) {
            s.store_add_scaled_inputs(266, 462, s.v[566], 461, s.v[565]);
        }

        if (((p.p24 != 0.0) && s.b[1606]) && (s.v[266] != 0.0)) {
            s.store_add_scaled_inputs(269, 462, p.p170, 461, p.p169);
        }

        s.b[1648] = (p.p43 == 1.0);
        s.v[1648] = if s.b[1648] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1606]) && (s.v[266] != 0.0)) && s.b[1648]) {
            s.store_add_ad(1524, A::mul(s.ad_value(462), s.ad_value(287)), A::mul(s.ad_value(461), s.ad_value(288)));
            s.store_mul_neg_rhs(269, 269, 1524);
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (s.v[266] != 0.0)) && (!s.b[1648])) {
            s.store_mul_neg_rhs(269, 269, 108);
        }

        if (((p.p24 != 0.0) && s.b[1606]) && (s.v[266] != 0.0)) {
            s.store_add_ad_rhs(268, 268, A::mul(A::neg(s.ad_value(269)), A::sub(s.ad_value(158), s.ad_value(157))));
        }

        if ((p.p24 != 0.0) && s.b[1606]) {
            s.store_add_scaled_inputs(266, 461, s.v[566], 462, s.v[565]);
        }

        if (((p.p24 != 0.0) && s.b[1606]) && (s.v[266] != 0.0)) {
            s.store_add_scaled_inputs(270, 461, p.p170, 462, p.p169);
        }

        s.b[1649] = (p.p43 == 1.0);
        s.v[1649] = if s.b[1649] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && s.b[1606]) && (s.v[266] != 0.0)) && s.b[1649]) {
            s.store_add_ad(1524, A::mul(s.ad_value(461), s.ad_value(287)), A::mul(s.ad_value(462), s.ad_value(288)));
            s.store_mul_neg_rhs(270, 270, 1524);
        }

        if ((((p.p24 != 0.0) && s.b[1606]) && (s.v[266] != 0.0)) && (!s.b[1649])) {
            s.store_mul_neg_rhs(270, 270, 108);
        }

        if (((p.p24 != 0.0) && s.b[1606]) && (s.v[266] != 0.0)) {
            s.store_add_ad_rhs(267, 267, A::mul(A::neg(s.ad_value(270)), s.ad_value(158)));
        }

        s.b[1650] = (((s.v[613] == 1.0) && (!s.b[565])) || ((s.v[613] != 1.0) && (!s.b[566])));
        s.v[1650] = if s.b[1650] { 1.0 } else { 0.0 };

        s.b[1651] = (p.p43 == 1.0);
        s.v[1651] = if s.b[1651] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (!s.b[1606])) && s.b[1650]) && s.b[1651]) {
            s.store_scale(269, 288, ((-s.v[1534]) * p.p188));
        }

        if ((((p.p24 != 0.0) && (!s.b[1606])) && s.b[1650]) && (!s.b[1651])) {
            s.store_scale(269, 108, ((-s.v[1534]) * p.p188));
        }

        if (((p.p24 != 0.0) && (!s.b[1606])) && (!s.b[1650])) {
            s.store_add_scaled_inputs(269, 462, p.p170, 461, p.p169);
        }

        s.b[1652] = (p.p43 == 1.0);
        s.v[1652] = if s.b[1652] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (!s.b[1606])) && (!s.b[1650])) && s.b[1652]) {
            s.store_add_ad(1524, A::mul(s.ad_value(462), s.ad_value(287)), A::mul(s.ad_value(461), s.ad_value(288)));
            s.store_mul_neg_rhs(269, 269, 1524);
        }

        if ((((p.p24 != 0.0) && (!s.b[1606])) && (!s.b[1650])) && (!s.b[1652])) {
            s.store_mul_neg_rhs(269, 269, 108);
        }

        if ((p.p24 != 0.0) && (!s.b[1606])) {
            s.store_mul_scaled_ad_rhs(268, 269, -1.0, A::sub(s.ad_value(158), s.ad_value(157)));
        }

        s.b[1653] = (((s.v[613] == 1.0) && (!s.b[566])) || ((s.v[613] != 1.0) && (!s.b[565])));
        s.v[1653] = if s.b[1653] { 1.0 } else { 0.0 };

        s.b[1654] = (p.p43 == 1.0);
        s.v[1654] = if s.b[1654] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (!s.b[1606])) && s.b[1653]) && s.b[1654]) {
            s.store_scale(270, 287, ((-s.v[1534]) * p.p188));
        }

        if ((((p.p24 != 0.0) && (!s.b[1606])) && s.b[1653]) && (!s.b[1654])) {
            s.store_scale(270, 108, ((-s.v[1534]) * p.p188));
        }

        if (((p.p24 != 0.0) && (!s.b[1606])) && (!s.b[1653])) {
            s.store_add_scaled_inputs(270, 461, p.p170, 462, p.p169);
        }

        s.b[1655] = (p.p43 == 1.0);
        s.v[1655] = if s.b[1655] { 1.0 } else { 0.0 };

        if ((((p.p24 != 0.0) && (!s.b[1606])) && (!s.b[1653])) && s.b[1655]) {
            s.store_add_ad(1524, A::mul(s.ad_value(461), s.ad_value(287)), A::mul(s.ad_value(462), s.ad_value(288)));
            s.store_mul_neg_rhs(270, 270, 1524);
        }

        if ((((p.p24 != 0.0) && (!s.b[1606])) && (!s.b[1653])) && (!s.b[1655])) {
            s.store_mul_neg_rhs(270, 270, 108);
        }

        if ((p.p24 != 0.0) && (!s.b[1606])) {
            s.store_mul_neg_lhs(267, 270, 158);
        }

        s.b[1656] = (p.p43 == 1.0);
        s.v[1656] = if s.b[1656] { 1.0 } else { 0.0 };

        if s.b[1656] {
            s.copy_ad(1672, 590);
            s.copy_ad(1673, 591);
            s.store_scale_ad(1674, A::exp(A::scale(A::add(A::sub_from_scalar((s.v[87] * s.v[114]), A::mul(s.ad_value(237), s.ad_value(225))), A::scale(A::ln(A::scale(s.ad_value(429), 1.0 / (s.v[81]))), p.p175)), 1.0 / (p.p174))), p.p173);
            s.store_scale_ad(1675, A::exp(A::scale(A::add(A::sub_from_scalar((s.v[87] * s.v[114]), A::mul(s.ad_value(237), s.ad_value(225))), A::scale(A::ln(A::scale(s.ad_value(429), 1.0 / (s.v[81]))), p.p176)), 1.0 / (p.p174))), p.p173);
            s.store_scaled_mul(1679, 286, 1674, p.p237);
            s.store_scaled_mul(1681, 286, 1675, p.p237);
            s.store_scaled_mul(1680, 285, 1674, p.p237);
            s.store_scaled_mul(1682, 285, 1675, p.p237);
            s.store_scale(1658, 429, 1.0 / (s.v[81]));
            s.store_offset(1659, 1679, 1e-50);
            s.store_scale_ad(1677, A::div_from_scalar(p.p174, s.ad_value(225)), 0.0);
            s.store_scale_ad(1678, A::div_from_scalar(p.p174, s.ad_value(225)), 0.0);
            s.store_scale(1676, 227, p.p174);
        }

        s.b[1685] = (s.v[1672] < s.v[1677]);
        s.v[1685] = if s.b[1685] { 1.0 } else { 0.0 };

        if (s.b[1656] && s.b[1685]) {
            s.store_exp_div(1658, 1672, 1676);
            s.store_mul_offset_rhs(282, 1679, 1658, (-1.0));
        }

        if (s.b[1656] && (!s.b[1685])) {
            s.store_exp_div(1658, 1677, 1676);
            s.store_add_ad(282, A::mul(s.ad_value(1679), A::offset(s.ad_value(1658), (-1.0))), A::mul(A::mul(A::div(s.ad_value(1679), s.ad_value(1676)), s.ad_value(1658)), A::sub(s.ad_value(1672), s.ad_value(1677))));
        }

        if s.b[1656] {
            s.store_add_ad_rhs(282, 282, A::mul(A::scale(s.ad_value(1672), p.p178), s.ad_value(1681)));
        }

        s.b[1686] = (s.v[1673] < s.v[1678]);
        s.v[1686] = if s.b[1686] { 1.0 } else { 0.0 };

        if (s.b[1656] && s.b[1686]) {
            s.store_exp_div(1658, 1673, 1676);
            s.store_mul_offset_rhs(281, 1680, 1658, (-1.0));
        }

        if (s.b[1656] && (!s.b[1686])) {
            s.store_exp_div(1658, 1678, 1676);
            s.store_add_ad(281, A::mul(s.ad_value(1680), A::offset(s.ad_value(1658), (-1.0))), A::mul(A::mul(A::div(s.ad_value(1680), s.ad_value(1676)), s.ad_value(1658)), A::sub(s.ad_value(1673), s.ad_value(1678))));
        }

        if s.b[1656] {
            s.store_add_ad_rhs(281, 281, A::mul(A::scale(s.ad_value(1673), p.p178), s.ad_value(1682)));
            s.store_add_ad_rhs(282, 282, A::scale(s.ad_value(1672), s.v[142]));
            s.store_add_ad_rhs(281, 281, A::scale(s.ad_value(1673), s.v[142]));
            s.store_scalar(1666, (p.p179 * p.p2));
            s.store_scalar(1667, (p.p179 * p.p3));
            s.store_scalar(1665, (p.p237 - p.p238));
        }

        s.b[1687] = (s.v[1665] <= 0.0);
        s.v[1687] = if s.b[1687] { 1.0 } else { 0.0 };

        if (s.b[1656] && s.b[1687]) {
            s.store_scalar(1666, 0.0);
            s.store_scalar(1667, 0.0);
        }

        s.b[1688] = (p.p5 > s.v[287]);
        s.v[1688] = if s.b[1688] { 1.0 } else { 0.0 };

        if (s.b[1656] && s.b[1688]) {
            s.store_scale_ad(1669, A::sub_from_scalar(p.p5, s.ad_value(287)), p.p180);
            s.store_scale(1671, 287, p.p181);
        }

        s.b[1689] = (s.v[1673] < 0.0);
        s.v[1689] = if s.b[1689] { 1.0 } else { 0.0 };

        s.b[1690] = (s.v[1667] > 0.0);
        s.v[1690] = if s.b[1690] { 1.0 } else { 0.0 };

        if (((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1690]) {
            s.store_sub_from_scalar_ad(1683, 1.0, A::scale(s.ad_value(1673), 1.0 / (p.p185)));
        }

        s.b[1691] = (p.p182 == 0.5);
        s.v[1691] = if s.b[1691] { 1.0 } else { 0.0 };

        if ((((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1690]) && s.b[1691]) {
            s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));
        }

        if ((((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1690]) && (!s.b[1691])) {
            s.store_powf(1684, 1683, (-p.p182));
        }

        if (((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1690]) {
            s.store_mul_scaled_ad_rhs(283, 1667, ((p.p185) * (1.0 / ((1.0 - p.p182)))), A::sub_from_scalar(1.0, A::mul(s.ad_value(1683), s.ad_value(1684))));
        }

        if (((s.b[1656] && s.b[1688]) && s.b[1689]) && (!s.b[1690])) {
            s.store_scalar(283, 0.0);
        }

        s.b[1692] = (s.v[1669] > 0.0);
        s.v[1692] = if s.b[1692] { 1.0 } else { 0.0 };

        if (((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1692]) {
            s.store_sub_from_scalar_ad(1683, 1.0, A::scale(s.ad_value(1673), 1.0 / (p.p186)));
        }

        s.b[1693] = (p.p183 == 0.5);
        s.v[1693] = if s.b[1693] { 1.0 } else { 0.0 };

        if ((((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1692]) && s.b[1693]) {
            s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));
        }

        if ((((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1692]) && (!s.b[1693])) {
            s.store_powf(1684, 1683, (-p.p183));
        }

        if (((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1692]) {
            s.store_add_ad_rhs(283, 283, A::scale(A::mul(A::scale(s.ad_value(1669), p.p186), A::sub_from_scalar(1.0, A::mul(s.ad_value(1683), s.ad_value(1684)))), 1.0 / ((1.0 - p.p183))));
        }

        s.b[1694] = (s.v[1671] > 0.0);
        s.v[1694] = if s.b[1694] { 1.0 } else { 0.0 };

        if (((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1694]) {
            s.store_sub_from_scalar_ad(1683, 1.0, A::scale(s.ad_value(1673), 1.0 / (p.p187)));
        }

        s.b[1695] = (p.p184 == 0.5);
        s.v[1695] = if s.b[1695] { 1.0 } else { 0.0 };

        if ((((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1694]) && s.b[1695]) {
            s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));
        }

        if ((((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1694]) && (!s.b[1695])) {
            s.store_powf(1684, 1683, (-p.p184));
        }

        if (((s.b[1656] && s.b[1688]) && s.b[1689]) && s.b[1694]) {
            s.store_add_ad_rhs(283, 283, A::scale(A::mul(A::scale(s.ad_value(1671), p.p187), A::sub_from_scalar(1.0, A::mul(s.ad_value(1683), s.ad_value(1684)))), 1.0 / ((1.0 - p.p184))));
        }

        if ((s.b[1656] && s.b[1688]) && (!s.b[1689])) {
            s.store_add_ad_lhs(1658, A::add(s.ad_value(1667), s.ad_value(1669)), 1671);
            s.store_add_scaled_ad_lhs(1659, A::add(A::scale(s.ad_value(1667), (p.p182 * 1.0 / (p.p185))), A::scale(s.ad_value(1669), (p.p183 * 1.0 / (p.p186)))), 1671, (p.p184 * 1.0 / (p.p187)));
            s.store_mul_add_ad_rhs(283, 1673, s.ad_value(1658), A::mul(A::scale(s.ad_value(1673), 0.5), s.ad_value(1659)));
        }

        if (s.b[1656] && (!s.b[1688])) {
            s.store_scalar(1671, (p.p181 * p.p5));
        }

        s.b[1696] = (s.v[1673] < 0.0);
        s.v[1696] = if s.b[1696] { 1.0 } else { 0.0 };

        s.b[1697] = (s.v[1667] > 0.0);
        s.v[1697] = if s.b[1697] { 1.0 } else { 0.0 };

        if (((s.b[1656] && (!s.b[1688])) && s.b[1696]) && s.b[1697]) {
            s.store_sub_from_scalar_ad(1683, 1.0, A::scale(s.ad_value(1673), 1.0 / (p.p185)));
        }

        s.b[1698] = (p.p182 == 0.5);
        s.v[1698] = if s.b[1698] { 1.0 } else { 0.0 };

        if ((((s.b[1656] && (!s.b[1688])) && s.b[1696]) && s.b[1697]) && s.b[1698]) {
            s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));
        }

        if ((((s.b[1656] && (!s.b[1688])) && s.b[1696]) && s.b[1697]) && (!s.b[1698])) {
            s.store_powf(1684, 1683, (-p.p182));
        }

        if (((s.b[1656] && (!s.b[1688])) && s.b[1696]) && s.b[1697]) {
            s.store_mul_scaled_ad_rhs(283, 1667, ((p.p185) * (1.0 / ((1.0 - p.p182)))), A::sub_from_scalar(1.0, A::mul(s.ad_value(1683), s.ad_value(1684))));
        }

        if (((s.b[1656] && (!s.b[1688])) && s.b[1696]) && (!s.b[1697])) {
            s.store_scalar(283, 0.0);
        }

        s.b[1699] = (s.v[1671] > 0.0);
        s.v[1699] = if s.b[1699] { 1.0 } else { 0.0 };

        if (((s.b[1656] && (!s.b[1688])) && s.b[1696]) && s.b[1699]) {
            s.store_sub_from_scalar_ad(1683, 1.0, A::scale(s.ad_value(1673), 1.0 / (p.p187)));
        }

        s.b[1700] = (p.p184 == 0.5);
        s.v[1700] = if s.b[1700] { 1.0 } else { 0.0 };

        if ((((s.b[1656] && (!s.b[1688])) && s.b[1696]) && s.b[1699]) && s.b[1700]) {
            s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));
        }

        if ((((s.b[1656] && (!s.b[1688])) && s.b[1696]) && s.b[1699]) && (!s.b[1700])) {
            s.store_powf(1684, 1683, (-p.p184));
        }

        if (((s.b[1656] && (!s.b[1688])) && s.b[1696]) && s.b[1699]) {
            s.store_add_ad_rhs(283, 283, A::scale(A::mul(A::scale(s.ad_value(1671), p.p187), A::sub_from_scalar(1.0, A::mul(s.ad_value(1683), s.ad_value(1684)))), 1.0 / ((1.0 - p.p184))));
        }

        if ((s.b[1656] && (!s.b[1688])) && (!s.b[1696])) {
            s.store_add(1658, 1667, 1671);
            s.store_add_scaled_inputs(1659, 1667, (p.p182 * 1.0 / (p.p185)), 1671, (p.p184 * 1.0 / (p.p187)));
            s.store_mul_add_ad_rhs(283, 1673, s.ad_value(1658), A::mul(A::scale(s.ad_value(1673), 0.5), s.ad_value(1659)));
        }

        s.b[1701] = (p.p4 > s.v[288]);
        s.v[1701] = if s.b[1701] { 1.0 } else { 0.0 };

        if (s.b[1656] && s.b[1701]) {
            s.store_scale_ad(1668, A::sub_from_scalar(p.p4, s.ad_value(288)), p.p180);
            s.store_scale(1670, 288, p.p181);
        }

        s.b[1702] = (s.v[1672] < 0.0);
        s.v[1702] = if s.b[1702] { 1.0 } else { 0.0 };

        s.b[1703] = (s.v[1666] > 0.0);
        s.v[1703] = if s.b[1703] { 1.0 } else { 0.0 };

        if (((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1703]) {
            s.store_sub_from_scalar_ad(1683, 1.0, A::scale(s.ad_value(1672), 1.0 / (p.p185)));
        }

        s.b[1704] = (p.p182 == 0.5);
        s.v[1704] = if s.b[1704] { 1.0 } else { 0.0 };

        if ((((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1703]) && s.b[1704]) {
            s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));
        }

        if ((((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1703]) && (!s.b[1704])) {
            s.store_powf(1684, 1683, (-p.p182));
        }

        if (((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1703]) {
            s.store_mul_scaled_ad_rhs(284, 1666, ((p.p185) * (1.0 / ((1.0 - p.p182)))), A::sub_from_scalar(1.0, A::mul(s.ad_value(1683), s.ad_value(1684))));
        }

    }

    pub(super) fn stamp_reactive_block_29(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1656] && s.b[1701]) && s.b[1702]) && (!s.b[1703])) {
            s.store_scalar(284, 0.0);
        }

        s.b[1705] = (s.v[1668] > 0.0);
        s.v[1705] = if s.b[1705] { 1.0 } else { 0.0 };

        if (((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1705]) {
            s.store_sub_from_scalar_ad(1683, 1.0, A::scale(s.ad_value(1672), 1.0 / (p.p186)));
        }

        s.b[1706] = (p.p183 == 0.5);
        s.v[1706] = if s.b[1706] { 1.0 } else { 0.0 };

        if ((((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1705]) && s.b[1706]) {
            s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));
        }

        if ((((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1705]) && (!s.b[1706])) {
            s.store_powf(1684, 1683, (-p.p183));
        }

        if (((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1705]) {
            s.store_add_ad_rhs(284, 284, A::scale(A::mul(A::scale(s.ad_value(1668), p.p186), A::sub_from_scalar(1.0, A::mul(s.ad_value(1683), s.ad_value(1684)))), 1.0 / ((1.0 - p.p183))));
        }

        s.b[1707] = (s.v[1670] > 0.0);
        s.v[1707] = if s.b[1707] { 1.0 } else { 0.0 };

        if (((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1707]) {
            s.store_sub_from_scalar_ad(1683, 1.0, A::scale(s.ad_value(1672), 1.0 / (p.p187)));
        }

        s.b[1708] = (p.p184 == 0.5);
        s.v[1708] = if s.b[1708] { 1.0 } else { 0.0 };

        if ((((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1707]) && s.b[1708]) {
            s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));
        }

        if ((((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1707]) && (!s.b[1708])) {
            s.store_powf(1684, 1683, (-p.p184));
        }

        if (((s.b[1656] && s.b[1701]) && s.b[1702]) && s.b[1707]) {
            s.store_add_ad_rhs(284, 284, A::scale(A::mul(A::scale(s.ad_value(1670), p.p187), A::sub_from_scalar(1.0, A::mul(s.ad_value(1683), s.ad_value(1684)))), 1.0 / ((1.0 - p.p184))));
        }

        if ((s.b[1656] && s.b[1701]) && (!s.b[1702])) {
            s.store_add_ad_lhs(1658, A::add(s.ad_value(1666), s.ad_value(1668)), 1670);
            s.store_add_scaled_ad_lhs(1659, A::add(A::scale(s.ad_value(1666), (p.p182 * 1.0 / (p.p185))), A::scale(s.ad_value(1668), (p.p183 * 1.0 / (p.p186)))), 1670, (p.p184 * 1.0 / (p.p187)));
            s.store_mul_add_ad_rhs(284, 1672, s.ad_value(1658), A::mul(A::scale(s.ad_value(1672), 0.5), s.ad_value(1659)));
        }

        if (s.b[1656] && (!s.b[1701])) {
            s.store_scalar(1670, (p.p181 * p.p4));
        }

        s.b[1709] = (s.v[1672] < 0.0);
        s.v[1709] = if s.b[1709] { 1.0 } else { 0.0 };

        s.b[1710] = (s.v[1666] > 0.0);
        s.v[1710] = if s.b[1710] { 1.0 } else { 0.0 };

        if (((s.b[1656] && (!s.b[1701])) && s.b[1709]) && s.b[1710]) {
            s.store_sub_from_scalar_ad(1683, 1.0, A::scale(s.ad_value(1672), 1.0 / (p.p185)));
        }

        s.b[1711] = (p.p182 == 0.5);
        s.v[1711] = if s.b[1711] { 1.0 } else { 0.0 };

        if ((((s.b[1656] && (!s.b[1701])) && s.b[1709]) && s.b[1710]) && s.b[1711]) {
            s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));
        }

        if ((((s.b[1656] && (!s.b[1701])) && s.b[1709]) && s.b[1710]) && (!s.b[1711])) {
            s.store_powf(1684, 1683, (-p.p182));
        }

        if (((s.b[1656] && (!s.b[1701])) && s.b[1709]) && s.b[1710]) {
            s.store_mul_scaled_ad_rhs(284, 1666, ((p.p185) * (1.0 / ((1.0 - p.p182)))), A::sub_from_scalar(1.0, A::mul(s.ad_value(1683), s.ad_value(1684))));
        }

        if (((s.b[1656] && (!s.b[1701])) && s.b[1709]) && (!s.b[1710])) {
            s.store_scalar(284, 0.0);
        }

        s.b[1712] = (s.v[1670] > 0.0);
        s.v[1712] = if s.b[1712] { 1.0 } else { 0.0 };

        if (((s.b[1656] && (!s.b[1701])) && s.b[1709]) && s.b[1712]) {
            s.store_sub_from_scalar_ad(1683, 1.0, A::scale(s.ad_value(1672), 1.0 / (p.p187)));
        }

        s.b[1713] = (p.p184 == 0.5);
        s.v[1713] = if s.b[1713] { 1.0 } else { 0.0 };

        if ((((s.b[1656] && (!s.b[1701])) && s.b[1709]) && s.b[1712]) && s.b[1713]) {
            s.store_div_from_scalar_sqrt_ad(1684, 1.0, s.ad_value(1683));
        }

        if ((((s.b[1656] && (!s.b[1701])) && s.b[1709]) && s.b[1712]) && (!s.b[1713])) {
            s.store_powf(1684, 1683, (-p.p184));
        }

        if (((s.b[1656] && (!s.b[1701])) && s.b[1709]) && s.b[1712]) {
            s.store_add_ad_rhs(284, 284, A::scale(A::mul(A::scale(s.ad_value(1670), p.p187), A::sub_from_scalar(1.0, A::mul(s.ad_value(1683), s.ad_value(1684)))), 1.0 / ((1.0 - p.p184))));
        }

        if ((s.b[1656] && (!s.b[1701])) && (!s.b[1709])) {
            s.store_add(1658, 1666, 1670);
            s.store_add_scaled_inputs(1659, 1666, (p.p182 * 1.0 / (p.p185)), 1670, (p.p184 * 1.0 / (p.p187)));
            s.store_mul_add_ad_rhs(284, 1672, s.ad_value(1658), A::mul(A::scale(s.ad_value(1672), 0.5), s.ad_value(1659)));
        }

        s.b[1714] = (s.v[1667] > 0.0);
        s.v[1714] = if s.b[1714] { 1.0 } else { 0.0 };

        if (s.b[1656] && s.b[1714]) {
            s.store_scaled_mul(1661, 544, 1665, (((-1.6021918e-19)) * (p.p3)));
            s.store_scale(1663, 1661, (-0.001));
            s.store_sub_ad_lhs(44, A::sub(A::neg(s.ad_value(1661)), A::neg(s.ad_value(283))), 1663);
            s.store_mul_scale_ad_lhs(45, A::neg(s.ad_value(1661)), 4.0, 1663);
        }

        if (s.b[1656] && s.b[1714]) {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (s.b[1656] && s.b[1714]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_sub_scaled_ad_rhs(283, 1661, -1.0, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
            s.store_scale(283, 283, (-1.0));
        }

        s.b[1715] = (s.v[1666] > 0.0);
        s.v[1715] = if s.b[1715] { 1.0 } else { 0.0 };

        if (s.b[1656] && s.b[1715]) {
            s.store_scaled_mul(1662, 544, 1665, (((-1.6021918e-19)) * (p.p2)));
            s.store_scale(1664, 1662, (-0.001));
            s.store_sub_ad_lhs(44, A::sub(A::neg(s.ad_value(1662)), A::neg(s.ad_value(284))), 1664);
            s.store_mul_scale_ad_lhs(45, A::neg(s.ad_value(1662)), 4.0, 1664);
        }

        if (s.b[1656] && s.b[1715]) {
            s.store_ad_value(45, {
                if (s.v[45] > 0.0) {
                    s.ad_value(45)
                } else {
                    A::neg(s.ad_value(45))
                }
            });
        }

        if (s.b[1656] && s.b[1715]) {
            s.store_sqrt_square_add(45, 44, 45);
            s.store_sub_scaled_ad_rhs(284, 1662, -1.0, A::scale(A::add(s.ad_value(44), s.ad_value(45)), 0.5));
            s.store_scale(284, 284, (-1.0));
        }

        s.b[1748] = ((p.p32 != 0.0) && (s.v[145] == 0.0));
        s.v[1748] = if s.b[1748] { 1.0 } else { 0.0 };

        if s.b[1748] {
            s.store_div_ad_lhs(1731, A::sub(s.ad_value(314), s.ad_value(161)), 441);
            s.store_scaled_mul(1732, 251, 1731, 1e-5);
        }

        s.b[1749] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1749] = if s.b[1749] { 1.0 } else { 0.0 };

        if (s.b[1748] && s.b[1749]) {
            s.store_scalar(1733, 1.0);
        }

        s.b[1750] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p113) && (p.p113 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1750] = if s.b[1750] { 1.0 } else { 0.0 };

        if ((s.b[1748] && (!s.b[1749])) && s.b[1750]) {
            s.copy_ad(1733, 1732);
        }

        if ((s.b[1748] && (!s.b[1749])) && (!s.b[1750])) {
            s.store_powf(1733, 1732, (p.p113 - 1.0));
        }

        if s.b[1748] {
            s.store_mul(1734, 1732, 1733);
            s.store_offset(1735, 1734, 1.0);
            s.store_powf(1736, 1735, (((-1.0) / p.p113) - 1.0));
            s.store_mul(1737, 1735, 1736);
            s.store_mul(293, 251, 1737);
            s.store_scaled_add(1739, 250, 293, 0.5);
            s.store_square(1738, 190);
        }

        if s.b[1748] {
            let assign33750_ad_e48938: A = A::add(A::add(A::mul(A::mul(A::add(A::offset(A::scale(s.ad_value(190), 3.0), 1.0), A::scale(s.ad_value(1738), 6.0)), s.ad_value(293)), s.ad_value(293)), A::mul(A::mul(A::add(A::offset(A::scale(s.ad_value(190), 4.0), 3.0), A::scale(s.ad_value(1738), 3.0)), s.ad_value(293)), s.ad_value(250))), A::mul(A::mul(A::add(A::offset(A::scale(s.ad_value(190), 3.0), 6.0), s.ad_value(1738)), s.ad_value(250)), s.ad_value(250)));
            s.store_div_ad(292, A::mul(A::mul(A::mul(A::mul(s.ad_value(107), s.ad_value(323)), s.ad_value(192)), s.ad_value(250)), assign33750_ad_e48938), A::mul(A::mul(A::mul(A::scale(s.ad_value(441), 15.0), A::offset(s.ad_value(190), 1.0)), s.ad_value(1739)), s.ad_value(1739)));
        }

        if (!s.b[1748]) {
            s.store_scalar(292, 0.0);
        }

        s.b[1751] = ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (s.v[146] == 1.0)) && (s.v[145] == 0.0));
        s.v[1751] = if s.b[1751] { 1.0 } else { 0.0 };

        if s.b[1751] {
            s.store_sqrt(298, 296);
            s.store_add(1740, 192, 298);
            s.store_square(1741, 294);
            s.store_square(1742, 296);
            s.store_scaled_mul(1743, 294, 296, 42.0);
            s.store_add_ad_rhs(1743, 1743, A::scale(A::add(s.ad_value(1741), s.ad_value(1742)), 4.0));
            s.store_add_ad_rhs(1743, 1743, A::mul(A::mul(A::scale(s.ad_value(298), 20.0), s.ad_value(192)), A::add(s.ad_value(294), s.ad_value(296))));
            s.store_square(1744, 1740);
            s.store_square(1736, 1744);
            s.store_div_ad_rhs(299, 1743, A::mul(s.ad_value(1736), s.ad_value(1740)));
            s.store_mul_ad_product_lhs(300, A::div(s.ad_value(107), s.ad_value(441)), s.ad_value(250), 323);
        }

        s.store_add(199, 199, 265);

        s.b[1752] = (p.p43 == 1.0);
        s.v[1752] = if s.b[1752] { 1.0 } else { 0.0 };

        if s.b[1752] {
            s.store_add(271, 531, 532);
        }

        if (s.b[1752] && s.b[564]) {
            s.store_offset(271, 271, (-(p.p168 * s.v[99])));
        }

        if s.b[1752] {
            s.store_mul_scaled_ad_rhs(272, 271, -1.0, A::sub(s.ad_value(158), s.ad_value(513)));
            s.store_scalar(276, 0.0);
            s.store_mul_scaled_ad_rhs(274, 276, p.p9, A::offset(s.ad_value(518), s.v[101]));
            s.store_mul_scaled_ad_rhs(275, 276, p.p9, A::offset(s.ad_value(519), s.v[101]));
            s.store_mul_sub_rhs(277, 274, 158, 157);
            s.store_mul(278, 275, 158);
            s.store_mul_scaled_ad_rhs(279, 276, (p.p19 * p.p9), A::sub(s.ad_value(158), s.ad_value(513)));
            s.store_add(268, 268, 277);
            s.store_add(267, 267, 278);
            s.store_add(272, 272, 279);
        }

        if ((!s.b[1752]) && s.b[564]) {
            s.store_scalar(271, ((-p.p168) * s.v[99]));
            s.store_mul_scaled_ad_rhs(272, 271, -1.0, A::sub(s.ad_value(158), s.ad_value(513)));
        }

        if ((!s.b[1752]) && (!s.b[564])) {
            s.store_scalar(271, 0.0);
            s.store_scalar(272, 0.0);
        }

        if (!s.b[1752]) {
            s.store_scalar(273, 0.0);
            s.copy_ad(274, 273);
            s.copy_ad(275, 273);
            s.store_mul_sub_rhs(277, 274, 158, 157);
            s.store_mul(278, 275, 158);
            s.store_add(268, 268, 277);
            s.store_add(267, 267, 278);
        }

        s.store_scale(9, 199, s.v[451]);

        if (s.v[85] != 0.0) {
            s.store_scalar(24, 0.0);
            s.store_scalar(23, 0.0);
        }

        s.b[1753] = (p.p43 == 1.0);
        s.v[1753] = if s.b[1753] { 1.0 } else { 0.0 };

        if ((s.v[85] != 0.0) && s.b[1753]) {
            s.store_scalar(25, 0.0);
            s.copy_ad(556, 438);
        }

        if ((s.v[85] != 0.0) && (!s.b[1753])) {
            s.store_scalar(554, 0.0);
        }

        s.b[1754] = (p.p43 == 1.0);
        s.v[1754] = if s.b[1754] { 1.0 } else { 0.0 };

        if ((s.v[85] == 0.0) && s.b[1754]) {
            s.store_scaled_sub_ad_lhs(23, A::neg(s.ad_value(196)), 197, s.v[451]);
            s.store_scale(24, 198, s.v[451]);
            s.store_scaled_sub(25, 197, 198, s.v[451]);
        }

        if ((s.v[85] == 0.0) && (!s.b[1754])) {
            s.store_scaled_sub_ad_lhs(23, A::sub(A::sub(A::neg(s.ad_value(392)), s.ad_value(197)), s.ad_value(476)), 477, s.v[451]);
            s.store_scaled_add(24, 198, 477, s.v[451]);
            s.store_scaled_add_ad_lhs(25, A::sub(s.ad_value(197), s.ad_value(198)), 476, s.v[451]);
        }

        s.b[1760] = (p.p64 == 0.0);
        s.v[1760] = if s.b[1760] { 1.0 } else { 0.0 };

        if s.b[1760] {
            s.store_scalar(280, 0.0);
        }

        if (!s.b[1760]) {
            s.store_add_ad_lhs(1755, A::scale(s.ad_value(315), s.v[97]), 161);
        }

        s.b[1761] = (s.v[1755] > s.v[314]);
        s.v[1761] = if s.b[1761] { 1.0 } else { 0.0 };

        if ((!s.b[1760]) && s.b[1761]) {
            s.copy_ad(1755, 314);
        }

        if (!s.b[1760]) {
            s.store_add_scaled_ad_lhs(1756, A::scale(A::add(s.ad_value(157), s.ad_value(161)), s.v[317]), 1755, (1.0 - s.v[317]));
            s.store_sqrt_div_from_scalar_ad(1757, (2.0 * 1.034943e-10), s.ad_value(229));
            s.store_scale(1758, 1757, 1.3);
            s.store_scaled_mul(1759, 108, 1758, 1.034943e-10);
            s.store_mul_sub_ad_lhs(280, A::scale(A::sub(A::add(s.ad_value(161), s.ad_value(157)), s.ad_value(1756)), 1.0 / (p.p64)), s.ad_value(315), 1759);
        }

        s.b[1762] = (p.p65 != 0.0);
        s.v[1762] = if s.b[1762] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_30(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if s.b[1762] {
            s.store_add_ad_rhs(280, 280, A::mul(s.ad_value(135), s.ad_value(513)));
        }

        s.b[1763] = (p.p24 == 1.0);
        s.v[1763] = if s.b[1763] { 1.0 } else { 0.0 };

        s.b[1764] = (p.p43 == 1.0);
        s.v[1764] = if s.b[1764] { 1.0 } else { 0.0 };

        if (s.b[1763] && s.b[1764]) {
            s.store_sub_ad_lhs(471, A::sub(A::sub(A::neg(s.ad_value(463)), s.ad_value(464)), s.ad_value(467)), 468);
            s.store_add(472, 466, 470);
            s.store_add(473, 465, 469);
            s.store_add_ad_rhs(23, 23, A::scale(A::add(A::sub(A::sub(A::sub(A::add(A::add(s.ad_value(268), s.ad_value(267)), s.ad_value(272)), s.ad_value(280)), s.ad_value(455)), s.ad_value(454)), s.ad_value(471)), s.v[451]));
            s.store_add_ad_rhs(24, 24, A::scale(A::add(A::add(A::sub(s.ad_value(280), s.ad_value(268)), s.ad_value(456)), s.ad_value(472)), s.v[451]));
            s.store_add_ad_rhs(25, 25, A::scale(A::add(A::sub(s.ad_value(457), s.ad_value(267)), s.ad_value(473)), s.v[451]));
        }

        if (s.b[1763] && (!s.b[1764])) {
            s.store_add_ad_rhs(23, 23, A::scale(A::sub(A::sub(A::sub(A::add(A::add(s.ad_value(268), s.ad_value(267)), s.ad_value(272)), s.ad_value(280)), s.ad_value(455)), s.ad_value(454)), s.v[451]));
            s.store_add_ad_rhs(24, 24, A::scale(A::add(A::sub(s.ad_value(280), s.ad_value(268)), s.ad_value(456)), s.v[451]));
            s.store_add_ad_rhs(25, 25, A::scale(A::sub(s.ad_value(457), s.ad_value(267)), s.v[451]));
        }

        s.b[1765] = (p.p43 == 1.0);
        s.v[1765] = if s.b[1765] { 1.0 } else { 0.0 };

        if s.b[1765] {
            s.store_scale(36, 281, s.v[451]);
            s.store_scale(35, 282, s.v[451]);
            s.store_scale(560, 284, s.v[451]);
            s.store_scale(561, 283, s.v[451]);
        }

        if (!s.b[1765]) {
            s.store_scalar(36, 0.0);
            s.store_scalar(35, 0.0);
            s.store_scalar(560, 0.0);
            s.store_scalar(561, 0.0);
        }

        s.b[1766] = (p.p25 != 1.0);
        s.v[1766] = if s.b[1766] { 1.0 } else { 0.0 };

        if s.b[1766] {
            s.store_scalar(557, 0.0);
        }

        if (!s.b[1766]) {
            s.store_scale(557, 263, s.v[451]);
        }

        s.store_scale(598, 292, s.v[451]);

        s.store_scalar(27, A::ddx_projection(&s.ad_value(23), Some(6), None));

        s.store_scale(27, 27, p.p50);

        s.store_scalar(28, A::ddx_projection(&s.ad_value(23), Some(7), None));

        s.store_scale(28, 28, p.p50);

        if (s.v[613] > 0.0) {
            s.copy_ad(555, 28);
        } else {
            s.copy_ad(555, 27);
        }

        s.b[1775] = ((((p.p30 != 0.0) && (p.p32 != 0.0)) && (s.v[146] == 1.0)) && (s.v[145] == 0.0));
        s.v[1775] = if s.b[1775] { 1.0 } else { 0.0 };

        if s.b[1775] {
            s.store_scaled_mul(1769, 323, 108, ((1e-6) * (s.v[98])));
            s.store_scale(1770, 555, 1.0 / (s.v[451]));
            s.store_div_ad_lhs(1771, A::mul(A::mul(A::scale(s.ad_value(227), (0.1185185185185185 * 1.6021918e-19)), s.ad_value(1770)), s.ad_value(1770)), 300);
        }

        s.b[1776] = ((s.v[297] > (10.0 * 2.220446049250313e-16)) && (s.v[157] > (10.0 * 2.220446049250313e-16)));
        s.v[1776] = if s.b[1776] { 1.0 } else { 0.0 };

        if (s.b[1775] && s.b[1776]) {
            s.store_div(1772, 251, 250);
            s.store_div_ad_lhs(1773, A::sub(A::div(s.ad_value(251), s.ad_value(293)), s.ad_value(1772)), 157);
            s.store_add_ad_rhs(1774, 1772, A::div(A::mul(A::scale(s.ad_value(1773), 0.6666666666666667), A::add(A::add(s.ad_value(294), A::mul(s.ad_value(192), s.ad_value(298))), s.ad_value(296))), A::add(s.ad_value(192), s.ad_value(298))));
        }

        if (s.b[1775] && (!s.b[1776])) {
            s.store_div(1774, 251, 293);
        }

        if s.b[1775] {
            s.store_mul_ad_product_lhs(558, A::scale(s.ad_value(1771), s.v[451]), s.ad_value(299), 1774);
        }

        if s.b[1775] {
            s.store_ad_value(558, {
                if (((-s.v[1770]) > s.v[1769]) && (s.v[558] > 0.0)) {
                    s.ad_value(558)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (!s.b[1775]) {
            s.store_scalar(558, 0.0);
        }

        s.b[1777] = (p.p259 == 1.0);
        s.v[1777] = if s.b[1777] { 1.0 } else { 0.0 };

        if s.b[1777] {
            s.store_scalar(3, 1.0);
        }

        s.b[1797] = (s.v[3] == 1.0);
        s.v[1797] = if s.b[1797] { 1.0 } else { 0.0 };

        if (s.b[1777] && s.b[1797]) {
            s.store_scalar(1781, p.p266);
            s.store_scalar(1782, p.p268);
            s.store_scalar(1783, p.p273);
            s.store_scalar(1787, p.p258);
            s.store_scaled_voltage(1785, ctx, nodes, Some(7), Some(2), p.p50);
        }

        if (s.b[1777] && (!s.b[1797])) {
            s.store_scalar(1781, p.p265);
            s.store_scalar(1782, p.p267);
            s.store_scalar(1783, p.p272);
            s.store_scalar(1787, p.p257);
            s.store_scaled_voltage(1785, ctx, nodes, Some(0), Some(6), p.p50);
        }

        if s.b[1777] {
            s.store_scale(1781, 1781, 0.0001);
            s.store_scale(1782, 1782, 0.01);
            s.store_scale(1786, 429, 1.0 / (s.v[81]));
            s.store_powf(328, 1786, p.p269);
            s.store_div(1789, 1781, 328);
            s.store_sub_ad(327, A::add(A::offset(A::scale(s.ad_value(1786), 0.4), 1.8), A::mul(A::scale(s.ad_value(1786), 0.1), s.ad_value(1786))), A::scale(A::sub_from_scalar(1.0, s.ad_value(1786)), p.p270));
            s.store_div(1790, 1782, 327);
            s.store_add_ad_rhs(1783, 1783, A::scale(A::offset(s.ad_value(429), (-s.v[81])), p.p274));
            s.store_scalar(1778, (1.0 + (p.p279 / ((s.v[100]) as f64).powf(p.p280))));
            s.store_scalar(1780, (1.0 + (p.p277 / ((s.v[100]) as f64).powf(p.p278))));
            s.store_scalar(1779, (1.0 + (p.p275 / ((s.v[109]) as f64).powf(p.p276))));
            s.store_mul(1789, 1789, 1778);
            s.store_offset_mul_ad(1790, A::mul(s.ad_value(1790), s.ad_value(1779)), s.ad_value(1780), 1e-50);
            s.store_div(1791, 1785, 1787);
            s.store_mul(1792, 1789, 1791);
        }

        s.b[1798] = (s.v[1785] >= 0.0);
        s.v[1798] = if s.b[1798] { 1.0 } else { 0.0 };

        if (s.b[1777] && s.b[1798]) {
            s.store_div(328, 1792, 1790);
        }

        if (s.b[1777] && (!s.b[1798])) {
            s.store_div_ad_lhs(328, A::neg(s.ad_value(1792)), 1790);
        }

        s.b[1799] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1783]) && (s.v[1783] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1799] = if s.b[1799] { 1.0 } else { 0.0 };

        if (s.b[1777] && s.b[1799]) {
            s.store_scalar(330, 1.0);
        }

        s.b[1800] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1783]) && (s.v[1783] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1800] = if s.b[1800] { 1.0 } else { 0.0 };

        if ((s.b[1777] && (!s.b[1799])) && s.b[1800]) {
            s.copy_ad(330, 328);
        }

        if ((s.b[1777] && (!s.b[1799])) && (!s.b[1800])) {
            s.store_pow_ad(330, s.ad_value(328), A::offset(s.ad_value(1783), (-1.0)));
        }

        if s.b[1777] {
            s.store_mul(329, 328, 330);
            s.store_offset(331, 329, 1.0);
        }

        s.b[1801] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1783]) && (s.v[1783] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1801] = if s.b[1801] { 1.0 } else { 0.0 };

        if (s.b[1777] && s.b[1801]) {
            s.store_div_from_scalar(332, 1.0, 331);
        }

        s.b[1802] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1783]) && (s.v[1783] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1802] = if s.b[1802] { 1.0 } else { 0.0 };

        if ((s.b[1777] && (!s.b[1801])) && s.b[1802]) {
            s.store_div_from_scalar_sqrt_ad(332, 1.0, s.ad_value(331));
        }

        if ((s.b[1777] && (!s.b[1801])) && (!s.b[1802])) {
            s.store_pow_ad(333, s.ad_value(331), A::offset(A::div_from_scalar((-1.0), s.ad_value(1783)), (-1.0)));
            s.store_mul(332, 331, 333);
        }

        if s.b[1777] {
            s.store_div_from_scalar(328, 1.6021918e-19, 1787);
        }

        s.b[1805] = (p.p260 == 1.0);
        s.v[1805] = if s.b[1805] { 1.0 } else { 0.0 };

        if s.b[1805] {
            s.store_scalar(3, 2.0);
        }

        s.b[1825] = (s.v[3] == 1.0);
        s.v[1825] = if s.b[1825] { 1.0 } else { 0.0 };

        if (s.b[1805] && s.b[1825]) {
            s.store_scalar(1809, p.p266);
            s.store_scalar(1810, p.p268);
            s.store_scalar(1811, p.p273);
            s.store_scalar(1815, p.p258);
            s.store_scaled_voltage(1813, ctx, nodes, Some(7), Some(2), p.p50);
        }

        if (s.b[1805] && (!s.b[1825])) {
            s.store_scalar(1809, p.p265);
            s.store_scalar(1810, p.p267);
            s.store_scalar(1811, p.p272);
            s.store_scalar(1815, p.p257);
            s.store_scaled_voltage(1813, ctx, nodes, Some(0), Some(6), p.p50);
        }

        if s.b[1805] {
            s.store_scale(1809, 1809, 0.0001);
            s.store_scale(1810, 1810, 0.01);
            s.store_scale(1814, 429, 1.0 / (s.v[81]));
            s.store_powf(328, 1814, p.p269);
            s.store_div(1817, 1809, 328);
            s.store_sub_ad(327, A::add(A::offset(A::scale(s.ad_value(1814), 0.4), 1.8), A::mul(A::scale(s.ad_value(1814), 0.1), s.ad_value(1814))), A::scale(A::sub_from_scalar(1.0, s.ad_value(1814)), p.p270));
            s.store_div(1818, 1810, 327);
            s.store_add_ad_rhs(1811, 1811, A::scale(A::offset(s.ad_value(429), (-s.v[81])), p.p274));
            s.store_scalar(1806, (1.0 + (p.p279 / ((s.v[100]) as f64).powf(p.p280))));
            s.store_scalar(1808, (1.0 + (p.p277 / ((s.v[100]) as f64).powf(p.p278))));
            s.store_scalar(1807, (1.0 + (p.p275 / ((s.v[109]) as f64).powf(p.p276))));
            s.store_mul(1817, 1817, 1806);
            s.store_offset_mul_ad(1818, A::mul(s.ad_value(1818), s.ad_value(1807)), s.ad_value(1808), 1e-50);
            s.store_div(1819, 1813, 1815);
            s.store_mul(1820, 1817, 1819);
        }

        s.b[1826] = (s.v[1813] >= 0.0);
        s.v[1826] = if s.b[1826] { 1.0 } else { 0.0 };

        if (s.b[1805] && s.b[1826]) {
            s.store_div(328, 1820, 1818);
        }

        if (s.b[1805] && (!s.b[1826])) {
            s.store_div_ad_lhs(328, A::neg(s.ad_value(1820)), 1818);
        }

        s.b[1827] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1811]) && (s.v[1811] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1827] = if s.b[1827] { 1.0 } else { 0.0 };

        if (s.b[1805] && s.b[1827]) {
            s.store_scalar(330, 1.0);
        }

        s.b[1828] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1811]) && (s.v[1811] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1828] = if s.b[1828] { 1.0 } else { 0.0 };

        if ((s.b[1805] && (!s.b[1827])) && s.b[1828]) {
            s.copy_ad(330, 328);
        }

        if ((s.b[1805] && (!s.b[1827])) && (!s.b[1828])) {
            s.store_pow_ad(330, s.ad_value(328), A::offset(s.ad_value(1811), (-1.0)));
        }

        if s.b[1805] {
            s.store_mul(329, 328, 330);
            s.store_offset(331, 329, 1.0);
        }

        s.b[1829] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1811]) && (s.v[1811] <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1829] = if s.b[1829] { 1.0 } else { 0.0 };

        if (s.b[1805] && s.b[1829]) {
            s.store_div_from_scalar(332, 1.0, 331);
        }

        s.b[1830] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[1811]) && (s.v[1811] <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[1830] = if s.b[1830] { 1.0 } else { 0.0 };

        if ((s.b[1805] && (!s.b[1829])) && s.b[1830]) {
            s.store_div_from_scalar_sqrt_ad(332, 1.0, s.ad_value(331));
        }

        if ((s.b[1805] && (!s.b[1829])) && (!s.b[1830])) {
            s.store_pow_ad(333, s.ad_value(331), A::offset(A::div_from_scalar((-1.0), s.ad_value(1811)), (-1.0)));
            s.store_mul(332, 331, 333);
        }

        if s.b[1805] {
            s.store_div_from_scalar(328, 1.6021918e-19, 1815);
        }

        s.b[1833] = (p.p43 == 1.0);
        s.v[1833] = if s.b[1833] { 1.0 } else { 0.0 };

    }
}

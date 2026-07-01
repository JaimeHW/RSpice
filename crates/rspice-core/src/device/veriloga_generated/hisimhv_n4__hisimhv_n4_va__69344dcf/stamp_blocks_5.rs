#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_80(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign79050_loop_guard: usize = 0;
        while {
            let assign79050_cond_e119567: f64 = (s.v[421] + 1.0);
            let assign79050_cond_e119569: f64 = if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (s.v[97] <= assign79050_cond_e119567)) { 1.0 } else { 0.0 };
            assign79050_cond_e119569 != 0.0
        } {
            assign79050_loop_guard += 1;
            assert!(assign79050_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2826] = (s.v[333] < 60.0);
            s.store_scalar(2826, if s.b[2826] { 1.0 } else { 0.0 });
            if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && s.b[2826]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ln_offset_lhs(416, 336, 1.0, 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2826])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2827] = (s.v[116] < 0.0);
            s.store_scalar(2827, if s.b[2827] { 1.0 } else { 0.0 });
            if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && s.b[2827]) {
                s.store_scalar(334, (-0.7071067811865475));
                s.store_mul(223, 116, 334);
                s.store_mul(420, 154, 334);
            }
            s.b[2828] = (s.v[116] < 1e-6);
            s.store_scalar(2828, if s.b[2828] { 1.0 } else { 0.0 });
            if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2827])) && s.b[2828]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(338, 334, 336);
            }
            s.b[2829] = (s.v[338] > 0.0);
            s.store_scalar(2829, if s.b[2829] { 1.0 } else { 0.0 });
            if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2827])) && s.b[2828]) && s.b[2829]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);
            }
            if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2827])) && s.b[2828]) && (!s.b[2829])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2827])) && (!s.b[2828])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(338, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
            }
            s.b[2830] = (s.v[338] > 0.0);
            s.store_scalar(2830, if s.b[2830] { 1.0 } else { 0.0 });
            if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2827])) && (!s.b[2828])) && s.b[2830]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335))), 0.5, 223, 1.0);
            }
            if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2827])) && (!s.b[2828])) && (!s.b[2830])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            s.b[2831] = (s.v[116] < 0.0);
            s.store_scalar(2831, if s.b[2831] { 1.0 } else { 0.0 });
            if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && s.b[2831]) {
                s.store_scalar(214, 0.0);
                s.store_scalar(215, 0.0);
                s.store_neg(216, 223);
                s.store_neg(217, 420);
            }
            s.b[2832] = (s.v[116] < 60.0);
            s.store_scalar(2832, if s.b[2832] { 1.0 } else { 0.0 });
            s.b[2833] = (s.v[116] < 5e-5);
            s.store_scalar(2833, if s.b[2833] { 1.0 } else { 0.0 });
            if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2831])) && s.b[2832]) && s.b[2833]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            if ((((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2831])) && s.b[2832]) && (!s.b[2833])) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2831])) && (!s.b[2832])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2834] = (s.v[214] > 0.0);
            s.store_scalar(2834, if s.b[2834] { 1.0 } else { 0.0 });
            if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2831])) && s.b[2834]) {
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_scaled_add_product(217, s.ad_value(215), 0.5, s.ad_value(420), s.ad_value(223), (2.0 * 0.5), s.ad_value(216), 1.0);
            }
            if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2831])) && (!s.b[2834])) {
                s.copy_ad(216, 223);
                s.copy_ad(217, 420);
            }
            if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2835] = (s.v[79] == 1.0);
            s.store_scalar(2835, if s.b[2835] { 1.0 } else { 0.0 });
            let (assign79050_body72_e120715,) = {
    if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && s.b[2835]) {
        let assign79050_body72_e120713: f64 = (s.v[421] + 1.0);
        (assign79050_body72_e120713,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign79050_body72_e120715);
            if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2835])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2835])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2836] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.store_scalar(2836, if s.b[2836] { 1.0 } else { 0.0 });
            if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2835])) && s.b[2836]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2835])) {
                s.store_add(404, 404, 236);
            }
            s.b[2837] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.store_scalar(2837, if s.b[2837] { 1.0 } else { 0.0 });
            let (assign79050_body79_e120818,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) && (!s.b[2835])) && s.b[2837]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, assign79050_body79_e120818);
            let (assign79050_body80_e120829,) = {
    if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) {
        let assign79050_body80_e120827: f64 = (s.v[97] + 1.0);
        (assign79050_body80_e120827,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign79050_body80_e120829);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2782])) && s.b[2821]) {
            s.store_mul(2756, 982, 223);
            s.store_mul(2757, 2758, 2756);
            s.store_offset_div(100, 2757, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        s.b[2839] = (p.p33 == 4.0);
        s.store_scalar(2839, if s.b[2839] { 1.0 } else { 0.0 });

        if ((s.v[2621] != 0.0) && s.b[2839]) {
            s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
            s.store_mul(222, 405, 229);
            s.copy_ad(404, 2765);
        }

        let (assign79200_e120966,) = {
    if ((s.v[2621] != 0.0) && s.b[2839]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign79200_e120966);

        if ((s.v[2621] != 0.0) && s.b[2839]) {
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2758)), s.ad_value(155)), 2.0);
        }

        s.b[2840] = (s.v[411] > 0.0);
        s.store_scalar(2840, if s.b[2840] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && s.b[2839]) && s.b[2840]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if (((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2840])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2841] = (s.v[336] < 0.0);
        s.store_scalar(2841, if s.b[2841] { 1.0 } else { 0.0 });

        if ((((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2840])) && s.b[2841]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2840])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if ((s.v[2621] != 0.0) && s.b[2839]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2842] = (s.v[336] < 0.0);
        s.store_scalar(2842, if s.b[2842] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && s.b[2839]) && s.b[2842]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((s.v[2621] != 0.0) && s.b[2839]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2758, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(419, 335, 418);
        }

        let (assign79430_e121215,) = {
    if ((s.v[2621] != 0.0) && s.b[2839]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, assign79430_e121215);

    }

    pub(super) fn stamp_transient_block_81(
        s: &mut Scratch,
        p: &Parameters,
        var_cox0: f64,
        var_uc_nover: f64,
        var_uc_novers: f64,
        var_vgsei: f64,
        var_vgsei_db0: f64,
        var_vgsei_db1: f64,
        var_vgsei_db10: f64,
        var_vgsei_db11: f64,
        var_vgsei_db2: f64,
        var_vgsei_db3: f64,
        var_vgsei_db4: f64,
        var_vgsei_db5: f64,
        var_vgsei_db6: f64,
        var_vgsei_db7: f64,
        var_vgsei_db8: f64,
        var_vgsei_db9: f64,
        var_vgsei_dn0: f64,
        var_vgsei_dn1: f64,
        var_vgsei_dn10: f64,
        var_vgsei_dn11: f64,
        var_vgsei_dn12: f64,
        var_vgsei_dn13: f64,
        var_vgsei_dn14: f64,
        var_vgsei_dn15: f64,
        var_vgsei_dn16: f64,
        var_vgsei_dn17: f64,
        var_vgsei_dn2: f64,
        var_vgsei_dn3: f64,
        var_vgsei_dn4: f64,
        var_vgsei_dn5: f64,
        var_vgsei_dn6: f64,
        var_vgsei_dn7: f64,
        var_vgsei_dn8: f64,
        var_vgsei_dn9: f64,
        var_weffcv_nf: f64,
        var_flg_coovlps_slot: &mut f64,
        var_guard1869_slot: &mut f64,
        var_guard1870_slot: &mut f64,
        var_guard1871_slot: &mut f64,
        var_guard1873_slot: &mut f64,
        var_guard1875_slot: &mut f64,
    ) {
        let mut var_flg_coovlps: f64 = *var_flg_coovlps_slot;
        let mut var_guard1869: f64 = *var_guard1869_slot;
        let mut var_guard1870: f64 = *var_guard1870_slot;
        let mut var_guard1871: f64 = *var_guard1871_slot;
        let mut var_guard1873: f64 = *var_guard1873_slot;
        let mut var_guard1875: f64 = *var_guard1875_slot;

        let mut assign79440_loop_guard: usize = 0;
        while {
            let assign79440_cond_e121222: f64 = (s.v[421] + 1.0);
            let assign79440_cond_e121224: f64 = if (((s.v[2621] != 0.0) && s.b[2839]) && (s.v[97] <= assign79440_cond_e121222)) { 1.0 } else { 0.0 };
            assign79440_cond_e121224 != 0.0
        } {
            assign79440_loop_guard += 1;
            assert!(assign79440_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2621] != 0.0) && s.b[2839]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2844] = (s.v[333] < 60.0);
            s.store_scalar(2844, if s.b[2844] { 1.0 } else { 0.0 });
            if (((s.v[2621] != 0.0) && s.b[2839]) && s.b[2844]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ln_offset_lhs(416, 336, 1.0, 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if (((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2844])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if ((s.v[2621] != 0.0) && s.b[2839]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2845] = (((s.v[116]) as f64).abs() < 1e-6);
            s.store_scalar(2845, if s.b[2845] { 1.0 } else { 0.0 });
            if (((s.v[2621] != 0.0) && s.b[2839]) && s.b[2845]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(2766, 334, 336);
                s.store_mul_add_scaled_product_rhs(2767, 154, s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0));
            }
            if (((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2845])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(2766, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
                s.store_mul_sub_ad_rhs(2767, 154, A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335)));
            }
            s.b[2846] = (((s.v[116]) as f64).abs() < 5e-5);
            s.store_scalar(2846, if s.b[2846] { 1.0 } else { 0.0 });
            if (((s.v[2621] != 0.0) && s.b[2839]) && s.b[2846]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            s.b[2847] = (((s.v[116]) as f64).abs() < 60.0);
            s.store_scalar(2847, if s.b[2847] { 1.0 } else { 0.0 });
            if ((((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2846])) && s.b[2847]) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if ((((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2846])) && (!s.b[2847])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2848] = (s.v[214] > 0.0);
            s.store_scalar(2848, if s.b[2848] { 1.0 } else { 0.0 });
            if (((s.v[2621] != 0.0) && s.b[2839]) && s.b[2848]) {
                s.store_sqrt_add(216, 2766, 214);
                s.store_div_scaled_inputs2_indices(217, 2767, 0.5, 215, 0.5, 216, 1.0);
            }
            s.b[2849] = (s.v[2766] > 0.0);
            s.store_scalar(2849, if s.b[2849] { 1.0 } else { 0.0 });
            if ((((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2848])) && s.b[2849]) {
                s.store_sqrt(216, 2766);
                s.store_div_scaled_inputs_indices(217, 2767, 0.5, 216, 1.0);
            }
            if ((((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2848])) && (!s.b[2849])) {
                s.store_scalar(216, 0.0);
                s.store_scalar(217, 0.0);
            }
            if ((s.v[2621] != 0.0) && s.b[2839]) {
                s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2621] != 0.0) && s.b[2839]) {
                s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2621] != 0.0) && s.b[2839]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2850] = (s.v[79] > 0.0);
            s.store_scalar(2850, if s.b[2850] { 1.0 } else { 0.0 });
            let (assign79440_body56_e121964,) = {
    if (((s.v[2621] != 0.0) && s.b[2839]) && s.b[2850]) {
        let assign79440_body56_e121962: f64 = (s.v[421] + 1.0);
        (assign79440_body56_e121962,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign79440_body56_e121964);
            if (((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2850])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if (((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2850])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2851] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.store_scalar(2851, if s.b[2851] { 1.0 } else { 0.0 });
            if ((((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2850])) && s.b[2851]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2850])) {
                s.store_add(404, 404, 236);
            }
            s.b[2852] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.store_scalar(2852, if s.b[2852] { 1.0 } else { 0.0 });
            let (assign79440_body63_e122054,) = {
    if ((((s.v[2621] != 0.0) && s.b[2839]) && (!s.b[2850])) && s.b[2852]) {
        let assign79440_body63_e122052: f64 = (s.v[79] + 2.0);
        (assign79440_body63_e122052,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, assign79440_body63_e122054);
            let (assign79440_body64_e122062,) = {
    if ((s.v[2621] != 0.0) && s.b[2839]) {
        let assign79440_body64_e122060: f64 = (s.v[97] + 1.0);
        (assign79440_body64_e122060,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign79440_body64_e122062);
        }

        if ((s.v[2621] != 0.0) && s.b[2839]) {
            if (s.v[2766] >= 0.0) {
                s.store_scaled_sqrt(223, 2766, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            } else {
                s.store_scalar(223, 0.0);
            }
        }

        if ((s.v[2621] != 0.0) && s.b[2839]) {
            s.store_mul(2756, 982, 223);
            s.store_mul(2757, 2758, 2756);
            s.store_offset_div(100, 2757, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        if (s.v[2621] != 0.0) {
            s.store_sub(399, 398, 354);
        }

        s.b[2854] = (s.v[407] < 0.0);
        s.store_scalar(2854, if s.b[2854] { 1.0 } else { 0.0 });

        if ((s.v[2621] != 0.0) && s.b[2854]) {
            s.store_neg(407, 407);
        }

        s.b[2855] = (p.p55 == 0.0);
        s.store_scalar(2855, if s.b[2855] { 1.0 } else { 0.0 });

        s.b[2856] = (p.p50 == 0.0);
        s.store_scalar(2856, if s.b[2856] { 1.0 } else { 0.0 });

        if ((((s.v[2621] != 0.0) && s.b[2854]) && s.b[2855]) && s.b[2856]) {
            s.store_neg(2759, 404);
        }

        if ((((s.v[2621] != 0.0) && s.b[2854]) && s.b[2855]) && (!s.b[2856])) {
            s.copy_ad(2759, 396);
        }

        if (((s.v[2621] != 0.0) && s.b[2854]) && s.b[2855]) {
            s.store_sqrt_offset_square_offset(782, 2759, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(2759), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(2759), p.p137), 782, 0.5);
        }

        s.b[2857] = (s.v[336] < 0.0);
        s.store_scalar(2857, if s.b[2857] { 1.0 } else { 0.0 });

        if ((((s.v[2621] != 0.0) && s.b[2854]) && s.b[2855]) && s.b[2857]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && s.b[2854]) && s.b[2855]) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_add_scaled_inputs3_indices(781, 407, 1.0, 600, (-1.0), 407, (-0.1));
            s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));
        }

        if (((s.v[2621] != 0.0) && s.b[2854]) && s.b[2855]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.v[2621] != 0.0) && s.b[2854]) && s.b[2855]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(603, 407, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_sub(407, 407, 603);
        }

        s.b[2858] = (2.0 == 1.0);
        s.store_scalar(2858, if s.b[2858] { 1.0 } else { 0.0 });

        s.b[2859] = (2.0 == 2.0);
        s.store_scalar(2859, if s.b[2859] { 1.0 } else { 0.0 });

        s.b[2860] = (2.0 == 3.0);
        s.store_scalar(2860, if s.b[2860] { 1.0 } else { 0.0 });

        s.b[2861] = (2.0 == 4.0);
        s.store_scalar(2861, if s.b[2861] { 1.0 } else { 0.0 });

        s.b[2862] = (p.p55 == 1.0);
        s.store_scalar(2862, if s.b[2862] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && s.b[2858]) && s.b[2862]) {
            s.store_scale(338, 407, var_weffcv_nf);
        }

        if (((s.v[2621] != 0.0) && s.b[2858]) && (!s.b[2862])) {
            s.store_scale(338, 407, (var_weffcv_nf * (1.0 - s.v[526])));
        }

        if ((s.v[2621] != 0.0) && s.b[2858]) {
            s.store_mul(353, 338, 398);
            s.store_mul(356, 338, 354);
        }

        if ((s.v[2621] != 0.0) && (s.b[2859] && (!s.b[2858]))) {
            s.store_scale(338, 407, (var_weffcv_nf * s.v[526]));
            s.store_mul(351, 338, 398);
            s.store_mul(359, 338, 354);
        }

        s.b[2863] = (p.p55 == 1.0);
        s.store_scalar(2863, if s.b[2863] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && (s.b[2860] && (!(s.b[2858] || s.b[2859])))) && s.b[2863]) {
            s.store_scale(338, 407, var_weffcv_nf);
        }

        if (((s.v[2621] != 0.0) && (s.b[2860] && (!(s.b[2858] || s.b[2859])))) && (!s.b[2863])) {
            s.store_scale(338, 407, (var_weffcv_nf * (1.0 - s.v[526])));
        }

        if ((s.v[2621] != 0.0) && (s.b[2860] && (!(s.b[2858] || s.b[2859])))) {
            s.copy_ad(697, 404);
        }

        s.b[2864] = (p.p430 == 0.0);
        s.store_scalar(2864, if s.b[2864] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && (s.b[2860] && (!(s.b[2858] || s.b[2859])))) && s.b[2864]) {
            s.copy_ad(698, 354);
        }

        if ((s.v[2621] != 0.0) && (s.b[2860] && (!(s.b[2858] || s.b[2859])))) {
            s.store_mul(352, 338, 398);
            s.store_mul(355, 338, 354);
            s.copy_ad(816, 355);
        }

        if ((s.v[2621] != 0.0) && (s.b[2861] && (!((s.b[2858] || s.b[2859]) || s.b[2860])))) {
            s.store_scale(338, 407, (var_weffcv_nf * s.v[526]));
            s.store_mul(350, 338, 398);
            s.store_mul(358, 338, 354);
        }

        s.store_scalar(2621, 0.0);

        let assign80020_e122629: f64 = if 3.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard1869 = assign80020_e122629;

        let assign80030_e122632: f64 = if 3.0 == 2.0 { 1.0 } else { 0.0 };
        var_guard1870 = assign80030_e122632;

        let assign80040_e122635: f64 = if 3.0 == 3.0 { 1.0 } else { 0.0 };
        var_guard1871 = assign80040_e122635;

        s.b[2868] = (3.0 == 4.0);
        s.store_scalar(2868, if s.b[2868] { 1.0 } else { 0.0 });

        let assign80060_e122649: f64 = if (((p.p36 == 1.0) && (p.p66 > 0.0)) && (var_uc_novers > 0.0)) { 1.0 } else { 0.0 };
        var_guard1873 = assign80060_e122649;

        let (assign80070_e122655,) = {
    if ((var_guard1869 != 0.0) && (var_guard1873 != 0.0)) {
        (1.0,)
    } else {
        (s.v[2621],)
    }
};
        s.store_scalar(2621, assign80070_e122655);

        let (assign80080_e122661,) = {
    if ((var_guard1869 != 0.0) && (var_guard1873 != 0.0)) {
        (1.0,)
    } else {
        (var_flg_coovlps,)
    }
};
        var_flg_coovlps = assign80080_e122661;

        if ((s.v[2865] != 0.0) && (s.v[2869] != 0.0)) {
            s.store_sub(395, 731, 728);
            s.store_neg(396, 728);
            s.store_scalar(409, var_uc_novers);
            s.store_scalar(407, p.p66);
            s.store_scalar(411, 0.0);
            s.copy_ad(410, 687);
            s.store_scalar(413, var_cox0);
        }

        s.b[2870] = (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.store_scalar(2870, if s.b[2870] { 1.0 } else { 0.0 });

        let (assign80170_e122734,) = {
    if (((var_guard1870 != 0.0) && (var_guard1869 == 0.0)) && s.b[2870]) {
        (1.0,)
    } else {
        (s.v[2621],)
    }
};
        s.store_scalar(2621, assign80170_e122734);

        if (((s.v[2866] != 0.0) && (s.v[2865] == 0.0)) && s.b[2870]) {
            s.store_sub_ad_lhs(395, A::from_derivatives(var_vgsei, [var_vgsei_dn0, var_vgsei_dn1, var_vgsei_dn2, var_vgsei_dn3, var_vgsei_dn4, var_vgsei_dn5, var_vgsei_dn6, var_vgsei_dn7, var_vgsei_dn8, var_vgsei_dn9, var_vgsei_dn10, var_vgsei_dn11, var_vgsei_dn12, var_vgsei_dn13, var_vgsei_dn14, var_vgsei_dn15, var_vgsei_dn16, var_vgsei_dn17], [var_vgsei_db0, var_vgsei_db1, var_vgsei_db2, var_vgsei_db3, var_vgsei_db4, var_vgsei_db5, var_vgsei_db6, var_vgsei_db7, var_vgsei_db8, var_vgsei_db9, var_vgsei_db10, var_vgsei_db11]), 735);
            s.store_neg(396, 735);
        }

        let assign80200_e122766: f64 = if (((p.p35 == 1.0) && (p.p63 > 0.0)) && (var_uc_nover > 0.0)) { 1.0 } else { 0.0 };
        var_guard1875 = assign80200_e122766;

        let (assign80210_e122777,) = {
    if (((var_guard1871 != 0.0) && (!((var_guard1869 != 0.0) || (var_guard1870 != 0.0)))) && (var_guard1875 != 0.0)) {
        (1.0,)
    } else {
        (s.v[2621],)
    }
};
        s.store_scalar(2621, assign80210_e122777);

        *var_flg_coovlps_slot = var_flg_coovlps;
        *var_guard1869_slot = var_guard1869;
        *var_guard1870_slot = var_guard1870;
        *var_guard1871_slot = var_guard1871;
        *var_guard1873_slot = var_guard1873;
        *var_guard1875_slot = var_guard1875;
    }

    pub(super) fn stamp_transient_block_82(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_coxb0: f64,
        var_guard1869: f64,
        var_guard1870: f64,
        var_guard1871: f64,
        var_guard1875: f64,
        var_uc_nover: f64,
        var_vdsei: f64,
        var_vdsei_db0: f64,
        var_vdsei_db1: f64,
        var_vdsei_db10: f64,
        var_vdsei_db11: f64,
        var_vdsei_db2: f64,
        var_vdsei_db3: f64,
        var_vdsei_db4: f64,
        var_vdsei_db5: f64,
        var_vdsei_db6: f64,
        var_vdsei_db7: f64,
        var_vdsei_db8: f64,
        var_vdsei_db9: f64,
        var_vdsei_dn0: f64,
        var_vdsei_dn1: f64,
        var_vdsei_dn10: f64,
        var_vdsei_dn11: f64,
        var_vdsei_dn12: f64,
        var_vdsei_dn13: f64,
        var_vdsei_dn14: f64,
        var_vdsei_dn15: f64,
        var_vdsei_dn16: f64,
        var_vdsei_dn17: f64,
        var_vdsei_dn2: f64,
        var_vdsei_dn3: f64,
        var_vdsei_dn4: f64,
        var_vdsei_dn5: f64,
        var_vdsei_dn6: f64,
        var_vdsei_dn7: f64,
        var_vdsei_dn8: f64,
        var_vdsei_dn9: f64,
        var_vgsei: f64,
        var_vgsei_db0: f64,
        var_vgsei_db1: f64,
        var_vgsei_db10: f64,
        var_vgsei_db11: f64,
        var_vgsei_db2: f64,
        var_vgsei_db3: f64,
        var_vgsei_db4: f64,
        var_vgsei_db5: f64,
        var_vgsei_db6: f64,
        var_vgsei_db7: f64,
        var_vgsei_db8: f64,
        var_vgsei_db9: f64,
        var_vgsei_dn0: f64,
        var_vgsei_dn1: f64,
        var_vgsei_dn10: f64,
        var_vgsei_dn11: f64,
        var_vgsei_dn12: f64,
        var_vgsei_dn13: f64,
        var_vgsei_dn14: f64,
        var_vgsei_dn15: f64,
        var_vgsei_dn16: f64,
        var_vgsei_dn17: f64,
        var_vgsei_dn2: f64,
        var_vgsei_dn3: f64,
        var_vgsei_dn4: f64,
        var_vgsei_dn5: f64,
        var_vgsei_dn6: f64,
        var_vgsei_dn7: f64,
        var_vgsei_dn8: f64,
        var_vgsei_dn9: f64,
        var_flg_coovlp_slot: &mut f64,
    ) {
        let mut var_flg_coovlp: f64 = *var_flg_coovlp_slot;

        let (assign80220_e122788,) = {
    if (((var_guard1871 != 0.0) && (!((var_guard1869 != 0.0) || (var_guard1870 != 0.0)))) && (var_guard1875 != 0.0)) {
        (1.0,)
    } else {
        (var_flg_coovlp,)
    }
};
        var_flg_coovlp = assign80220_e122788;

        if (((s.v[2867] != 0.0) && (!((s.v[2865] != 0.0) || (s.v[2866] != 0.0)))) && (s.v[2871] != 0.0)) {
            s.store_sub(395, 731, 728);
            s.store_sub(396, 729, 728);
            s.store_scalar(409, var_uc_nover);
            s.store_scalar(407, (p.p63 + (p.p64 * p.p55)));
            s.copy_ad(411, 384);
            s.copy_ad(410, 686);
            s.store_scalar(413, var_coxb0);
            s.store_neg(407, 407);
        }

        s.b[2872] = (((s.v[407] < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0));
        s.store_scalar(2872, if s.b[2872] { 1.0 } else { 0.0 });

        if ((((s.v[2867] != 0.0) && (!((s.v[2865] != 0.0) || (s.v[2866] != 0.0)))) && (s.v[2871] != 0.0)) && s.b[2872]) {
            s.store_neg(407, 407);
            s.store_scalar(335, p.p63);
            s.store_offset_div_scaled_product(996, s.ad_value(335), s.ad_value(335), 1.0, s.ad_value(651), 1.0, (-p.p137));
        }

        s.b[2873] = (p.p113 > 0.0);
        s.store_scalar(2873, if s.b[2873] { 1.0 } else { 0.0 });

        s.b[2874] = ((s.v[396] == 0.0) || (p.p113 <= 0.0));
        s.store_scalar(2874, if s.b[2874] { 1.0 } else { 0.0 });

        if ((((((s.v[2867] != 0.0) && (!((s.v[2865] != 0.0) || (s.v[2866] != 0.0)))) && (s.v[2871] != 0.0)) && s.b[2872]) && s.b[2873]) && s.b[2874]) {
        }

        if ((((((s.v[2867] != 0.0) && (!((s.v[2865] != 0.0) || (s.v[2866] != 0.0)))) && (s.v[2871] != 0.0)) && s.b[2872]) && s.b[2873]) && (!s.b[2874])) {
            s.store_scalar(783, (if (s.v[396] < 0.0) { (-1.0) } else { 1.0 }));
        }

        if ((((((s.v[2867] != 0.0) && (!((s.v[2865] != 0.0) || (s.v[2866] != 0.0)))) && (s.v[2871] != 0.0)) && s.b[2872]) && s.b[2873]) && (!s.b[2874])) {
            s.store_mul(784, 783, 396);
            s.store_offset_powf_ad(781, A::div(s.ad_value(784), s.ad_value(996)), p.p113, 1.0);
            s.store_powf(782, 781, (1.0 / p.p113));
            s.store_div_scaled_product_indices(396, 783, 784, 1.0, 782, 1.0);
        }

        if (((((s.v[2867] != 0.0) && (!((s.v[2865] != 0.0) || (s.v[2866] != 0.0)))) && (s.v[2871] != 0.0)) && s.b[2872]) && s.b[2873]) {
            s.store_sqrt_offset_square_offset(782, 396, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(396), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(396), p.p137), 782, 0.5);
        }

        s.b[2875] = (s.v[336] < 0.0);
        s.store_scalar(2875, if s.b[2875] { 1.0 } else { 0.0 });

        if ((((((s.v[2867] != 0.0) && (!((s.v[2865] != 0.0) || (s.v[2866] != 0.0)))) && (s.v[2871] != 0.0)) && s.b[2872]) && s.b[2873]) && s.b[2875]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((((s.v[2867] != 0.0) && (!((s.v[2865] != 0.0) || (s.v[2866] != 0.0)))) && (s.v[2871] != 0.0)) && s.b[2872]) && s.b[2873]) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub(407, 407, 600);
        }

        s.b[2876] = (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.store_scalar(2876, if s.b[2876] { 1.0 } else { 0.0 });

        let (assign80520_e123259,) = {
    if ((s.b[2868] && (!(((var_guard1869 != 0.0) || (var_guard1870 != 0.0)) || (var_guard1871 != 0.0)))) && s.b[2876]) {
        (1.0,)
    } else {
        (s.v[2621],)
    }
};
        s.store_scalar(2621, assign80520_e123259);

        if ((s.b[2868] && (!(((s.v[2865] != 0.0) || (s.v[2866] != 0.0)) || (s.v[2867] != 0.0)))) && s.b[2876]) {
            s.store_sub_ad_lhs(395, A::from_derivatives(var_vgsei, [var_vgsei_dn0, var_vgsei_dn1, var_vgsei_dn2, var_vgsei_dn3, var_vgsei_dn4, var_vgsei_dn5, var_vgsei_dn6, var_vgsei_dn7, var_vgsei_dn8, var_vgsei_dn9, var_vgsei_dn10, var_vgsei_dn11, var_vgsei_dn12, var_vgsei_dn13, var_vgsei_dn14, var_vgsei_dn15, var_vgsei_dn16, var_vgsei_dn17], [var_vgsei_db0, var_vgsei_db1, var_vgsei_db2, var_vgsei_db3, var_vgsei_db4, var_vgsei_db5, var_vgsei_db6, var_vgsei_db7, var_vgsei_db8, var_vgsei_db9, var_vgsei_db10, var_vgsei_db11]), 735);
            s.store_sub_ad_lhs(396, A::from_derivatives(var_vdsei, [var_vdsei_dn0, var_vdsei_dn1, var_vdsei_dn2, var_vdsei_dn3, var_vdsei_dn4, var_vdsei_dn5, var_vdsei_dn6, var_vdsei_dn7, var_vdsei_dn8, var_vdsei_dn9, var_vdsei_dn10, var_vdsei_dn11, var_vdsei_dn12, var_vdsei_dn13, var_vdsei_dn14, var_vdsei_dn15, var_vdsei_dn16, var_vdsei_dn17], [var_vdsei_db0, var_vdsei_db1, var_vdsei_db2, var_vdsei_db3, var_vdsei_db4, var_vdsei_db5, var_vdsei_db6, var_vdsei_db7, var_vdsei_db8, var_vdsei_db9, var_vdsei_db10, var_vdsei_db11]), 735);
        }

        if (s.v[2621] != 0.0) {
            s.store_scalar(2884, 0.4);
        }

        let (assign80570_e123301,) = {
    if (s.v[2621] != 0.0) {
        (0.0,)
    } else {
        (s.v[2885],)
    }
};
        s.store_scalar(2885, assign80570_e123301);

        if (s.v[2621] != 0.0) {
            s.store_scalar(223, 0.0);
            s.store_scalar(214, 0.0);
            s.store_scalar(216, 0.0);
            s.store_scalar(232, 0.0);
            s.store_scalar(236, 0.0);
            s.store_scalar(233, 0.0);
            s.store_scalar(217, 0.0);
            s.store_scalar(420, 0.0);
            s.store_scalar(215, 0.0);
            s.store_scalar(447, 0.0);
            s.store_scalar(445, 0.0);
            s.store_scalar(446, 0.0);
        }

        let (assign80700_e123354,) = {
    if (s.v[2621] != 0.0) {
        let assign80700_e123352: f64 = (-1.0);
        (assign80700_e123352,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign80700_e123354);

        if (s.v[2621] != 0.0) {
            s.store_scalar(2886, 0.0);
            s.store_scalar(2887, 0.0);
            s.store_mul_scaled_ln_ad_rhs(2882, 155, 2.0, A::div(s.ad_value(409), s.ad_value(394)));
            s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(2882), (-0.1));
            s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        }

        if (s.v[2621] != 0.0) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.v[2621] != 0.0) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(2883, 781, (-0.5), 782, (-0.5), 0.8);
        }

        s.b[2889] = (s.v[2884] > (s.v[2883] * 0.5));
        s.store_scalar(2889, if s.b[2889] { 1.0 } else { 0.0 });

        if ((s.v[2621] != 0.0) && s.b[2889]) {
            s.store_scale(2884, 2883, 0.5);
        }

        s.b[2890] = param_given[338];
        s.store_scalar(2890, if s.b[2890] { 1.0 } else { 0.0 });

        if ((s.v[2621] != 0.0) && s.b[2890]) {
            s.store_scalar(2883, p.p338);
        }

        s.b[2891] = param_given[339];
        s.store_scalar(2891, if s.b[2891] { 1.0 } else { 0.0 });

        if ((s.v[2621] != 0.0) && s.b[2891]) {
            s.store_scalar(2884, p.p339);
        }

        s.b[2892] = param_given[338];
        s.store_scalar(2892, if s.b[2892] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && (!s.b[2891])) && s.b[2892]) {
            s.store_scale(2884, 2883, 0.5);
        }

        s.b[2893] = (s.v[2884] > (s.v[2883] * 0.5));
        s.store_scalar(2893, if s.b[2893] { 1.0 } else { 0.0 });

        if ((s.v[2621] != 0.0) && s.b[2893]) {
            s.store_scale(2884, 2883, 0.5);
        }

        s.b[2894] = (p.p38 == 1.0);
        s.store_scalar(2894, if s.b[2894] { 1.0 } else { 0.0 });

        if ((s.v[2621] != 0.0) && s.b[2894]) {
            s.store_neg(334, 396);
        }

        s.b[2895] = (s.v[334] > s.v[2884]);
        s.store_scalar(2895, if s.b[2895] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && s.b[2894]) && s.b[2895]) {
            s.store_sub(335, 334, 2884);
            s.store_sub(336, 2883, 2884);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 345, 1.0);
            s.store_neg(345, 345);
            s.store_add(344, 2884, 333);
        }

        if (((s.v[2621] != 0.0) && s.b[2894]) && (!s.b[2895])) {
            s.copy_ad(344, 334);
        }

        if ((s.v[2621] != 0.0) && s.b[2894]) {
            s.store_neg(397, 344);
        }

        if ((s.v[2621] != 0.0) && (!s.b[2894])) {
            s.copy_ad(397, 396);
        }

        if (s.v[2621] != 0.0) {
            s.store_div(212, 410, 413);
            s.store_square(213, 212);
            s.store_sub_from_scalar(402, s.v[458], 395);
        }

        let (assign81110_e123695,) = {
    if (s.v[2621] != 0.0) {
        let assign81110_e123689: f64 = (-s.v[397]);
        let assign81110_e123692: f64 = (10.0 * 2.220446049250313e-16);
        let assign81110_e123693: f64 = (assign81110_e123689 + assign81110_e123692);
        (assign81110_e123693,)
    } else {
        (s.v[403],)
    }
};
        s.store_scalar(403, assign81110_e123695);

        if (s.v[2621] != 0.0) {
            s.store_scalar(2878, 0.0);
            s.store_scale(2879, 409, 1.6021918e-19);
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
        }

        s.b[2896] = ((s.v[154] * (-s.v[397])) >= 500.0);
        s.store_scalar(2896, if s.b[2896] { 1.0 } else { 0.0 });

        if ((s.v[2621] != 0.0) && s.b[2896]) {
            s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(334, 1.403592217853e217);
        }

        if ((s.v[2621] != 0.0) && (!s.b[2896])) {
            s.store_mul_neg_rhs(781, 154, 397);
            s.store_scalar(229, 1.0);
        }

        let mut assign81210_loop_guard: usize = 0;
        while {
            let assign81210_cond_e123769: f64 = if (((s.v[2621] != 0.0) && (!s.b[2896])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign81210_cond_e123769 != 0.0
        } {
            assign81210_loop_guard += 1;
            assert!(assign81210_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2621] != 0.0) && (!s.b[2896])) {
                s.store_scale(229, 229, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if ((s.v[2621] != 0.0) && (!s.b[2896])) {
            s.store_mul_exp_rhs(229, 229, 781);
            s.copy_ad(334, 229);
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));
            s.store_scalar(782, (4.0 * 0.5));
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(335, 781, 0.5, 782, 0.5, 0.5);
        }

        s.b[2897] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));
        s.store_scalar(2897, if s.b[2897] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2897]) {
            s.store_add_scaled_inputs3_indices(781, 402, 1.0, 397, 1.0, 335, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 335);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign81360_e123943,) = {
    if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2897]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign81360_e123943);

        let (assign81370_e123951,) = {
    if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2897]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign81370_e123951);

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2897]) {
            s.store_scalar(770, 0.0);
        }

        *var_flg_coovlp_slot = var_flg_coovlp;
    }

    pub(super) fn stamp_transient_block_83(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2897]) {
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2898] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.store_scalar(2898, if s.b[2898] { 1.0 } else { 0.0 });

        s.b[2899] = (1.0 == 1.0);
        s.store_scalar(2899, if s.b[2899] { 1.0 } else { 0.0 });

        let (assign81460_e124035,) = {
    if (((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2897]) && s.b[2898]) && s.b[2899]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign81460_e124035);

        s.b[2900] = (1.0 == 2.0);
        s.store_scalar(2900, if s.b[2900] { 1.0 } else { 0.0 });

        let (assign81480_e124053,) = {
    if ((((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2897]) && s.b[2898]) && (!s.b[2899])) && s.b[2900]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign81480_e124053);

        s.b[2901] = (1.0 == 4.0);
        s.store_scalar(2901, if s.b[2901] { 1.0 } else { 0.0 });

        let (assign81500_e124074,) = {
    if (((((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2897]) && s.b[2898]) && (!s.b[2899])) && (!s.b[2900])) && s.b[2901]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign81500_e124074);

        s.b[2902] = (1.0 == 8.0);
        s.store_scalar(2902, if s.b[2902] { 1.0 } else { 0.0 });

        let (assign81520_e124098,) = {
    if ((((((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2897]) && s.b[2898]) && (!s.b[2899])) && (!s.b[2900])) && (!s.b[2901])) && s.b[2902]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign81520_e124098);

        let (assign81530_e124108,) = {
    if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2897]) && s.b[2898]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign81530_e124108);

        let mut assign81540_loop_guard: usize = 0;
        while {
            let assign81540_cond_e124119: f64 = if (((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2897]) && s.b[2898]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign81540_cond_e124119 != 0.0
        } {
            assign81540_loop_guard += 1;
            assert!(assign81540_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2897]) && s.b[2898]) {
                s.store_sqrt(726, 726);
            }
            let (assign81540_body1_e124142,) = {
    if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2897]) && s.b[2898]) {
        let assign81540_body1_e124140: f64 = (s.v[719] + 1.0);
        (assign81540_body1_e124140,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign81540_body1_e124142);
        }

        if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2897]) && (!s.b[2898])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2897]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);
        }

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2897]) {
        }

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && (!s.b[2897])) {
            s.store_add(335, 402, 397);
            s.store_scalar(334, 1.0);
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sub(397, 335, 402);
        }

        let (assign81640_e124259,) = {
    if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
        let assign81640_e124253: f64 = (-s.v[397]);
        let assign81640_e124256: f64 = (10.0 * 2.220446049250313e-16);
        let assign81640_e124257: f64 = (assign81640_e124253 + assign81640_e124256);
        (assign81640_e124257,)
    } else {
        (s.v[403],)
    }
};
        s.store_scalar(403, assign81640_e124259);

        s.b[2903] = (s.v[402] < s.v[403]);
        s.store_scalar(2903, if s.b[2903] { 1.0 } else { 0.0 });

        if ((s.v[2621] != 0.0) && s.b[2903]) {
            s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));
            s.store_mul_add_rhs(332, 154, 402, 397);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(410));
            s.store_mul(333, 335, 413);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(278, (7.0 * 1.414213562373095), 333, A::offset(s.ad_value(332), (-2.0)), 9.0);
            s.store_square(276, 278);
        }

        s.b[2904] = (s.v[277] < (s.v[276] * 1e-8));
        s.store_scalar(2904, if s.b[2904] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && s.b[2903]) && s.b[2904]) {
            s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);
        }

        if (((s.v[2621] != 0.0) && s.b[2903]) && (!s.b[2904])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((s.v[2621] != 0.0) && s.b[2903]) {
            s.store_powf(273, 274, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);
            s.store_div(116, 272, 273);
            s.store_mul(335, 116, 155);
            s.store_div(336, 335, 271);
            s.store_sqrt_square_offset(337, 336, 1.0);
            s.store_sub_div_lhs_indices(404, 335, 337, 397);
            s.store_sub(336, 402, 404);
            s.store_mul(398, 413, 336);
            s.copy_ad(354, 398);
            s.copy_ad(2886, 404);
        }

        s.b[2905] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));
        s.store_scalar(2905, if s.b[2905] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2905]) {
            s.store_add_scaled_product_indices(89, 402, 1.0, 213, 154, 0.5);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && (!s.b[2905])) {
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
        }

        if ((s.v[2621] != 0.0) && (!s.b[2903])) {
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        s.b[2906] = (s.v[116] >= 3.0);
        s.store_scalar(2906, if s.b[2906] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2906]) {
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
            s.store_mul_add_rhs(116, 154, 89, 397);
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && (!s.b[2906])) {
            s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));
            s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));
            s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));
            s.store_div_scaled_inputs2_indices(437, 402, -1.0, 397, -1.0, 212, 1.0);
            s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(441, A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(435), s.ad_value(436), 1.0, s.ad_value(434), s.ad_value(434), 6.0), (-1.0), 437, 1.0, 434, 2.0, 1.0);
            s.store_div_ad(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), A::mul_scaled_lhs(s.ad_value(434), 9.0, s.ad_value(434)));
            s.store_sqrt_add_scaled_square_cube_product(339, 441, 1.0, 440, 1.0);
            s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);
            s.store_neg_powf_add_input(438, 441, 339, 0.3333333333333333);
            s.store_add_scaled_inputs3_div_scaled_third_indices(116, 439, 1.0, 438, 1.0, 435, 1.0, 434, 3.0, -1.0);
            s.store_add_scaled_product_indices(89, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2907] = (p.p33 > 0.0);
        s.store_scalar(2907, if s.b[2907] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) {
            s.store_offset_add(442, 402, 397, 0.1);
            s.store_mul(222, 405, 229);
            s.store_mul(443, 405, 229);
            s.store_mul(334, 156, 213);
            s.store_mul(444, 154, 442);
            s.store_add_scaled_inputs_product_mixed_aaii(447, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), 154, 397, 1.0);
        }

        s.b[2908] = (p.p33 == 2.0);
        s.store_scalar(2908, if s.b[2908] { 1.0 } else { 0.0 });

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2908]) {
            s.store_offset_sub(781, 444, 447, (-1.0));
            s.store_scale(782, 444, 4.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2908]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2908]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(447, 444, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && (!s.b[2908])) {
            if (s.v[447] <= s.v[444]) {
            } else {
                s.copy_ad(447, 444);
            }
        }

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) {
            if (s.v[447] >= 0.0) {
            } else {
                s.store_scalar(447, 0.0);
            }
        }

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) {
            s.store_sub(444, 444, 447);
            s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);
            s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));
            s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) {
            s.copy_ad(445, 116);
        }

        s.b[2909] = (p.p33 == 2.0);
        s.store_scalar(2909, if s.b[2909] { 1.0 } else { 0.0 });

        s.b[2910] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));
        s.store_scalar(2910, if s.b[2910] { 1.0 } else { 0.0 });

        if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && s.b[2910]) {
            s.store_add_scaled_inputs3_indices(781, 445, 1.0, 446, (-1.0), 446, 0.2);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 446, 446, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign82470_e125405,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && s.b[2910]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign82470_e125405);

        let (assign82480_e125418,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && s.b[2910]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign82480_e125418);

        if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && s.b[2910]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
        }

    }

    pub(super) fn stamp_transient_block_84(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && s.b[2910]) {
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2911] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(2911, if s.b[2911] { 1.0 } else { 0.0 });

        s.b[2912] = (2.0 == 1.0);
        s.store_scalar(2912, if s.b[2912] { 1.0 } else { 0.0 });

        let (assign82590_e125567,) = {
    if (((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && s.b[2910]) && s.b[2911]) && s.b[2912]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign82590_e125567);

        s.b[2913] = (2.0 == 2.0);
        s.store_scalar(2913, if s.b[2913] { 1.0 } else { 0.0 });

        let (assign82610_e125590,) = {
    if ((((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && s.b[2910]) && s.b[2911]) && (!s.b[2912])) && s.b[2913]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign82610_e125590);

        s.b[2914] = (2.0 == 4.0);
        s.store_scalar(2914, if s.b[2914] { 1.0 } else { 0.0 });

        let (assign82630_e125616,) = {
    if (((((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && s.b[2910]) && s.b[2911]) && (!s.b[2912])) && (!s.b[2913])) && s.b[2914]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign82630_e125616);

        s.b[2915] = (2.0 == 8.0);
        s.store_scalar(2915, if s.b[2915] { 1.0 } else { 0.0 });

        let (assign82650_e125645,) = {
    if ((((((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && s.b[2910]) && s.b[2911]) && (!s.b[2912])) && (!s.b[2913])) && (!s.b[2914])) && s.b[2915]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign82650_e125645);

        let (assign82660_e125660,) = {
    if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && s.b[2910]) && s.b[2911]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign82660_e125660);

        let mut assign82670_loop_guard: usize = 0;
        while {
            let assign82670_cond_e125676: f64 = if (((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && s.b[2910]) && s.b[2911]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign82670_cond_e125676 != 0.0
        } {
            assign82670_loop_guard += 1;
            assert!(assign82670_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && s.b[2910]) && s.b[2911]) {
                s.store_sqrt(726, 726);
            }
            let (assign82670_body1_e125709,) = {
    if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && s.b[2910]) && s.b[2911]) {
        let assign82670_body1_e125707: f64 = (s.v[719] + 1.0);
        (assign82670_body1_e125707,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign82670_body1_e125709);
        }

        if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && s.b[2910]) && (!s.b[2911])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && s.b[2910]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);
            s.store_div_scaled_product3_indices(335, 446, 725, 726, 0.2, 770, 1.0);
            s.store_add_scaled_inputs3_indices(116, 446, 1.0, 446, (-0.2), 780, 1.0);
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && s.b[2910]) {
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && s.b[2909]) && (!s.b[2910])) {
            s.copy_ad(116, 445);
            s.store_scalar(335, 1.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2907]) && (!s.b[2909])) {
            if (s.v[445] <= s.v[446]) {
                s.copy_ad(116, 445);
            } else {
                s.copy_ad(116, 446);
            }
        }

        s.b[2916] = (p.p33 == 1.0);
        s.store_scalar(2916, if s.b[2916] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2916]) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2917] = (s.v[411] > 0.0);
        s.store_scalar(2917, if s.b[2917] { 1.0 } else { 0.0 });

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2916]) && s.b[2917]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2916]) && (!s.b[2917])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2918] = (s.v[336] < 0.0);
        s.store_scalar(2918, if s.b[2918] { 1.0 } else { 0.0 });

        if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2916]) && (!s.b[2917])) && s.b[2918]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2916]) && (!s.b[2917])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2916]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2919] = (s.v[336] < 0.0);
        s.store_scalar(2919, if s.b[2919] { 1.0 } else { 0.0 });

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2916]) && s.b[2919]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2916]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2879, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(419, 335, 418);
            s.store_add(414, 404, 397);
            s.store_mul_sub_rhs(333, 419, 414, 418);
        }

        s.b[2920] = (s.v[333] < 60.0);
        s.store_scalar(2920, if s.b[2920] { 1.0 } else { 0.0 });

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2916]) && s.b[2920]) {
            s.store_exp(335, 333);
            s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
            s.store_sub(336, 335, 334);
            s.store_div_ln_offset_lhs(416, 336, 1.0, 419);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2916]) && (!s.b[2920])) {
            s.store_sub(416, 414, 418);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2916]) {
            s.store_mul(415, 154, 416);
        }

        s.b[2921] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));
        s.store_scalar(2921, if s.b[2921] { 1.0 } else { 0.0 });

        let (assign83100_e126298,) = {
    if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2916]) && s.b[2921]) {
        let assign83100_e126296: f64 = (s.v[2885] + 1.0);
        (assign83100_e126296,)
    } else {
        (s.v[2885],)
    }
};
        s.store_scalar(2885, assign83100_e126298);

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2916]) && s.b[2921]) {
            s.copy_ad(116, 447);
        }

        if ((s.v[2621] != 0.0) && (!s.b[2903])) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2922] = (((s.v[116]) as f64).abs() > 1e-6);
        s.store_scalar(2922, if s.b[2922] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2922]) {
            s.store_add_offset_lhs_ad_rhs(335, 116, (-1.0), A::exp_scaled_input(s.ad_value(116), -1.0));
            s.store_sqrt(336, 335);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && (!s.b[2922])) {
            s.store_mul_scaled_sqrt_ad_rhs(336, 116, 0.7071067811865475, A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333)));
        }

        if ((s.v[2621] != 0.0) && (!s.b[2903])) {
            s.store_mul(354, 410, 336);
            s.store_mul_sub_rhs(398, 413, 402, 404);
            s.store_div(2923, 354, 2879);
        }

        s.b[2925] = (p.p33 == 2.0);
        s.store_scalar(2925, if s.b[2925] { 1.0 } else { 0.0 });

        s.b[2926] = ((s.v[2923] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));
        s.store_scalar(2926, if s.b[2926] { 1.0 } else { 0.0 });

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && s.b[2926]) {
            s.store_add_scaled_inputs3_indices(781, 2923, 1.0, 386, (-1.0), 386, 0.1);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 386, 386, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign83280_e126505,) = {
    if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && s.b[2926]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign83280_e126505);

        let (assign83290_e126516,) = {
    if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && s.b[2926]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign83290_e126516);

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && s.b[2926]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2927] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(2927, if s.b[2927] { 1.0 } else { 0.0 });

        s.b[2928] = (2.0 == 1.0);
        s.store_scalar(2928, if s.b[2928] { 1.0 } else { 0.0 });

        let (assign83400_e126647,) = {
    if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && s.b[2926]) && s.b[2927]) && s.b[2928]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign83400_e126647);

        s.b[2929] = (2.0 == 2.0);
        s.store_scalar(2929, if s.b[2929] { 1.0 } else { 0.0 });

        let (assign83420_e126668,) = {
    if (((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && s.b[2926]) && s.b[2927]) && (!s.b[2928])) && s.b[2929]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign83420_e126668);

        s.b[2930] = (2.0 == 4.0);
        s.store_scalar(2930, if s.b[2930] { 1.0 } else { 0.0 });

        let (assign83440_e126692,) = {
    if ((((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && s.b[2926]) && s.b[2927]) && (!s.b[2928])) && (!s.b[2929])) && s.b[2930]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign83440_e126692);

        s.b[2931] = (2.0 == 8.0);
        s.store_scalar(2931, if s.b[2931] { 1.0 } else { 0.0 });

        let (assign83460_e126719,) = {
    if (((((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && s.b[2926]) && s.b[2927]) && (!s.b[2928])) && (!s.b[2929])) && (!s.b[2930])) && s.b[2931]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign83460_e126719);

        let (assign83470_e126732,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && s.b[2926]) && s.b[2927]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign83470_e126732);

        let mut assign83480_loop_guard: usize = 0;
        while {
            let assign83480_cond_e126746: f64 = if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && s.b[2926]) && s.b[2927]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign83480_cond_e126746 != 0.0
        } {
            assign83480_loop_guard += 1;
            assert!(assign83480_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && s.b[2926]) && s.b[2927]) {
                s.store_sqrt(726, 726);
            }
            let (assign83480_body1_e126775,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && s.b[2926]) && s.b[2927]) {
        let assign83480_body1_e126773: f64 = (s.v[719] + 1.0);
        (assign83480_body1_e126773,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign83480_body1_e126775);
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && s.b[2926]) && (!s.b[2927])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && s.b[2926]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 386, 725, 726, 0.1, 770, 1.0);
            s.store_add_scaled_inputs3_indices(335, 386, 1.0, 386, (-0.1), 780, 1.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && s.b[2926]) {
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && (!s.b[2926])) {
            s.copy_ad(335, 2923);
        }

    }

    pub(super) fn stamp_transient_block_85(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && (!s.b[2926])) {
            s.store_scalar(334, 1.0);
        }

        s.b[2932] = (s.v[334] < 1.0);
        s.store_scalar(2932, if s.b[2932] { 1.0 } else { 0.0 });

        let (assign83580_e126917,) = {
    if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2925]) && s.b[2932]) {
        let assign83580_e126915: f64 = (s.v[2885] + 2.0);
        (assign83580_e126915,)
    } else {
        (s.v[2885],)
    }
};
        s.store_scalar(2885, assign83580_e126917);

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && (!s.b[2925])) {
            if (s.v[2923] <= s.v[386]) {
                s.copy_ad(335, 2923);
            } else {
                s.copy_ad(335, 386);
            }
        }

        s.b[2933] = (s.v[2923] >= s.v[386]);
        s.store_scalar(2933, if s.b[2933] { 1.0 } else { 0.0 });

        let (assign83610_e126949,) = {
    if ((((s.v[2621] != 0.0) && (!s.b[2903])) && (!s.b[2925])) && s.b[2933]) {
        let assign83610_e126947: f64 = (s.v[2885] + 2.0);
        (assign83610_e126947,)
    } else {
        (s.v[2885],)
    }
};
        s.store_scalar(2885, assign83610_e126949);

        s.b[2934] = (s.v[2885] >= 2.0);
        s.store_scalar(2934, if s.b[2934] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) {
            s.copy_ad(2924, 404);
            s.store_mul(354, 335, 2879);
            s.store_sub_div_rhs_indices(404, 402, 354, 413);
        }

        s.b[2935] = (p.p33 == 2.0);
        s.store_scalar(2935, if s.b[2935] { 1.0 } else { 0.0 });

        s.b[2936] = ((s.v[404] > (s.v[2924] - 0.1)) && (0.1 >= 0.0));
        s.store_scalar(2936, if s.b[2936] { 1.0 } else { 0.0 });

        if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && s.b[2936]) {
            s.store_offset_sub(781, 404, 2924, 0.1);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign83730_e127083,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && s.b[2936]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign83730_e127083);

        let (assign83740_e127096,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && s.b[2936]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign83740_e127096);

        if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && s.b[2936]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2937] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(2937, if s.b[2937] { 1.0 } else { 0.0 });

        s.b[2938] = (2.0 == 1.0);
        s.store_scalar(2938, if s.b[2938] { 1.0 } else { 0.0 });

        let (assign83850_e127245,) = {
    if (((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && s.b[2936]) && s.b[2937]) && s.b[2938]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign83850_e127245);

        s.b[2939] = (2.0 == 2.0);
        s.store_scalar(2939, if s.b[2939] { 1.0 } else { 0.0 });

        let (assign83870_e127268,) = {
    if ((((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && s.b[2936]) && s.b[2937]) && (!s.b[2938])) && s.b[2939]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign83870_e127268);

        s.b[2940] = (2.0 == 4.0);
        s.store_scalar(2940, if s.b[2940] { 1.0 } else { 0.0 });

        let (assign83890_e127294,) = {
    if (((((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && s.b[2936]) && s.b[2937]) && (!s.b[2938])) && (!s.b[2939])) && s.b[2940]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign83890_e127294);

        s.b[2941] = (2.0 == 8.0);
        s.store_scalar(2941, if s.b[2941] { 1.0 } else { 0.0 });

        let (assign83910_e127323,) = {
    if ((((((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && s.b[2936]) && s.b[2937]) && (!s.b[2938])) && (!s.b[2939])) && (!s.b[2940])) && s.b[2941]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign83910_e127323);

        let (assign83920_e127338,) = {
    if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && s.b[2936]) && s.b[2937]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign83920_e127338);

        let mut assign83930_loop_guard: usize = 0;
        while {
            let assign83930_cond_e127354: f64 = if (((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && s.b[2936]) && s.b[2937]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign83930_cond_e127354 != 0.0
        } {
            assign83930_loop_guard += 1;
            assert!(assign83930_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && s.b[2936]) && s.b[2937]) {
                s.store_sqrt(726, 726);
            }
            let (assign83930_body1_e127387,) = {
    if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && s.b[2936]) && s.b[2937]) {
        let assign83930_body1_e127385: f64 = (s.v[719] + 1.0);
        (assign83930_body1_e127385,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign83930_body1_e127387);
        }

        if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && s.b[2936]) && (!s.b[2937])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && s.b[2936]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_add_offset_lhs(404, 2924, (-0.1), 780);
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && s.b[2936]) {
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && (!s.b[2936])) {
        }

        if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && s.b[2935]) && (!s.b[2936])) {
            s.store_scalar(334, 1.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2934]) && (!s.b[2935])) {
            if (s.v[404] <= s.v[2924]) {
            } else {
                s.copy_ad(404, 2924);
            }
        }

        if ((s.v[2621] != 0.0) && (!s.b[2903])) {
            s.copy_ad(2886, 404);
        }

        s.b[2942] = (p.p33 == 1.0);
        s.store_scalar(2942, if s.b[2942] { 1.0 } else { 0.0 });

        let (assign84050_e127559,) = {
    if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign84050_e127559);

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) {
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2879)), s.ad_value(155)), 2.0);
        }

        s.b[2943] = (s.v[411] > 0.0);
        s.store_scalar(2943, if s.b[2943] { 1.0 } else { 0.0 });

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && s.b[2943]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2943])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2944] = (s.v[336] < 0.0);
        s.store_scalar(2944, if s.b[2944] { 1.0 } else { 0.0 });

        if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2943])) && s.b[2944]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2943])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2945] = (s.v[336] < 0.0);
        s.store_scalar(2945, if s.b[2945] { 1.0 } else { 0.0 });

        if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && s.b[2945]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2879, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(419, 335, 418);
        }

        let (assign84280_e127868,) = {
    if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, assign84280_e127868);

    }

    pub(super) fn stamp_transient_block_86(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign84290_loop_guard: usize = 0;
        while {
            let assign84290_cond_e127878: f64 = (s.v[421] + 1.0);
            let assign84290_cond_e127880: f64 = if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (s.v[97] <= assign84290_cond_e127878)) { 1.0 } else { 0.0 };
            assign84290_cond_e127880 != 0.0
        } {
            assign84290_loop_guard += 1;
            assert!(assign84290_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2947] = (s.v[333] < 60.0);
            s.store_scalar(2947, if s.b[2947] { 1.0 } else { 0.0 });
            if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && s.b[2947]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ln_offset_lhs(416, 336, 1.0, 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2947])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2948] = (s.v[116] < 0.0);
            s.store_scalar(2948, if s.b[2948] { 1.0 } else { 0.0 });
            if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && s.b[2948]) {
                s.store_scalar(334, (-0.7071067811865475));
                s.store_mul(223, 116, 334);
                s.store_mul(420, 154, 334);
            }
            s.b[2949] = (s.v[116] < 1e-6);
            s.store_scalar(2949, if s.b[2949] { 1.0 } else { 0.0 });
            if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2948])) && s.b[2949]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(338, 334, 336);
            }
            s.b[2950] = (s.v[338] > 0.0);
            s.store_scalar(2950, if s.b[2950] { 1.0 } else { 0.0 });
            if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2948])) && s.b[2949]) && s.b[2950]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);
            }
            if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2948])) && s.b[2949]) && (!s.b[2950])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2948])) && (!s.b[2949])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(338, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
            }
            s.b[2951] = (s.v[338] > 0.0);
            s.store_scalar(2951, if s.b[2951] { 1.0 } else { 0.0 });
            if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2948])) && (!s.b[2949])) && s.b[2951]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335))), 0.5, 223, 1.0);
            }
            if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2948])) && (!s.b[2949])) && (!s.b[2951])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            s.b[2952] = (s.v[116] < 0.0);
            s.store_scalar(2952, if s.b[2952] { 1.0 } else { 0.0 });
            if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && s.b[2952]) {
                s.store_scalar(214, 0.0);
                s.store_scalar(215, 0.0);
                s.store_neg(216, 223);
                s.store_neg(217, 420);
            }
            s.b[2953] = (s.v[116] < 60.0);
            s.store_scalar(2953, if s.b[2953] { 1.0 } else { 0.0 });
            s.b[2954] = (s.v[116] < 5e-5);
            s.store_scalar(2954, if s.b[2954] { 1.0 } else { 0.0 });
            if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2952])) && s.b[2953]) && s.b[2954]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            if ((((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2952])) && s.b[2953]) && (!s.b[2954])) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2952])) && (!s.b[2953])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2955] = (s.v[214] > 0.0);
            s.store_scalar(2955, if s.b[2955] { 1.0 } else { 0.0 });
            if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2952])) && s.b[2955]) {
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_scaled_add_product(217, s.ad_value(215), 0.5, s.ad_value(420), s.ad_value(223), (2.0 * 0.5), s.ad_value(216), 1.0);
            }
            if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2952])) && (!s.b[2955])) {
                s.copy_ad(216, 223);
                s.copy_ad(217, 420);
            }
            if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2956] = (s.v[79] == 1.0);
            s.store_scalar(2956, if s.b[2956] { 1.0 } else { 0.0 });
            let (assign84290_body72_e129026,) = {
    if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && s.b[2956]) {
        let assign84290_body72_e129024: f64 = (s.v[421] + 1.0);
        (assign84290_body72_e129024,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign84290_body72_e129026);
            if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2956])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2956])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2957] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.store_scalar(2957, if s.b[2957] { 1.0 } else { 0.0 });
            if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2956])) && s.b[2957]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2956])) {
                s.store_add(404, 404, 236);
            }
            s.b[2958] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.store_scalar(2958, if s.b[2958] { 1.0 } else { 0.0 });
            let (assign84290_body79_e129129,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) && (!s.b[2956])) && s.b[2958]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, assign84290_body79_e129129);
            let (assign84290_body80_e129140,) = {
    if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) {
        let assign84290_body80_e129138: f64 = (s.v[97] + 1.0);
        (assign84290_body80_e129138,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign84290_body80_e129140);
        }

        if (((s.v[2621] != 0.0) && (!s.b[2903])) && s.b[2942]) {
            s.store_mul(2877, 982, 223);
            s.store_mul(2878, 2879, 2877);
            s.store_offset_div(100, 2878, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        s.b[2960] = (p.p33 == 4.0);
        s.store_scalar(2960, if s.b[2960] { 1.0 } else { 0.0 });

        if ((s.v[2621] != 0.0) && s.b[2960]) {
            s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
            s.store_mul(222, 405, 229);
            s.copy_ad(404, 2886);
        }

        let (assign84440_e129277,) = {
    if ((s.v[2621] != 0.0) && s.b[2960]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign84440_e129277);

        if ((s.v[2621] != 0.0) && s.b[2960]) {
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2879)), s.ad_value(155)), 2.0);
        }

        s.b[2961] = (s.v[411] > 0.0);
        s.store_scalar(2961, if s.b[2961] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && s.b[2960]) && s.b[2961]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if (((s.v[2621] != 0.0) && s.b[2960]) && (!s.b[2961])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2962] = (s.v[336] < 0.0);
        s.store_scalar(2962, if s.b[2962] { 1.0 } else { 0.0 });

        if ((((s.v[2621] != 0.0) && s.b[2960]) && (!s.b[2961])) && s.b[2962]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && s.b[2960]) && (!s.b[2961])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if ((s.v[2621] != 0.0) && s.b[2960]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2963] = (s.v[336] < 0.0);
        s.store_scalar(2963, if s.b[2963] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && s.b[2960]) && s.b[2963]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((s.v[2621] != 0.0) && s.b[2960]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2879, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(419, 335, 418);
        }

        let (assign84670_e129526,) = {
    if ((s.v[2621] != 0.0) && s.b[2960]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, assign84670_e129526);

    }

    pub(super) fn stamp_transient_block_87(
        s: &mut Scratch,
        p: &Parameters,
        var_cox0: f64,
        var_uc_nover: f64,
        var_uc_novers: f64,
        var_vgsei: f64,
        var_vgsei_db0: f64,
        var_vgsei_db1: f64,
        var_vgsei_db10: f64,
        var_vgsei_db11: f64,
        var_vgsei_db2: f64,
        var_vgsei_db3: f64,
        var_vgsei_db4: f64,
        var_vgsei_db5: f64,
        var_vgsei_db6: f64,
        var_vgsei_db7: f64,
        var_vgsei_db8: f64,
        var_vgsei_db9: f64,
        var_vgsei_dn0: f64,
        var_vgsei_dn1: f64,
        var_vgsei_dn10: f64,
        var_vgsei_dn11: f64,
        var_vgsei_dn12: f64,
        var_vgsei_dn13: f64,
        var_vgsei_dn14: f64,
        var_vgsei_dn15: f64,
        var_vgsei_dn16: f64,
        var_vgsei_dn17: f64,
        var_vgsei_dn2: f64,
        var_vgsei_dn3: f64,
        var_vgsei_dn4: f64,
        var_vgsei_dn5: f64,
        var_vgsei_dn6: f64,
        var_vgsei_dn7: f64,
        var_vgsei_dn8: f64,
        var_vgsei_dn9: f64,
        var_weffcv_nf: f64,
        var_flg_coovlps_slot: &mut f64,
        var_guard1990_slot: &mut f64,
        var_guard1991_slot: &mut f64,
        var_guard1992_slot: &mut f64,
        var_guard1994_slot: &mut f64,
        var_guard1996_slot: &mut f64,
    ) {
        let mut var_flg_coovlps: f64 = *var_flg_coovlps_slot;
        let mut var_guard1990: f64 = *var_guard1990_slot;
        let mut var_guard1991: f64 = *var_guard1991_slot;
        let mut var_guard1992: f64 = *var_guard1992_slot;
        let mut var_guard1994: f64 = *var_guard1994_slot;
        let mut var_guard1996: f64 = *var_guard1996_slot;

        let mut assign84680_loop_guard: usize = 0;
        while {
            let assign84680_cond_e129533: f64 = (s.v[421] + 1.0);
            let assign84680_cond_e129535: f64 = if (((s.v[2621] != 0.0) && s.b[2960]) && (s.v[97] <= assign84680_cond_e129533)) { 1.0 } else { 0.0 };
            assign84680_cond_e129535 != 0.0
        } {
            assign84680_loop_guard += 1;
            assert!(assign84680_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2621] != 0.0) && s.b[2960]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2965] = (s.v[333] < 60.0);
            s.store_scalar(2965, if s.b[2965] { 1.0 } else { 0.0 });
            if (((s.v[2621] != 0.0) && s.b[2960]) && s.b[2965]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ln_offset_lhs(416, 336, 1.0, 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if (((s.v[2621] != 0.0) && s.b[2960]) && (!s.b[2965])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if ((s.v[2621] != 0.0) && s.b[2960]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2966] = (((s.v[116]) as f64).abs() < 1e-6);
            s.store_scalar(2966, if s.b[2966] { 1.0 } else { 0.0 });
            if (((s.v[2621] != 0.0) && s.b[2960]) && s.b[2966]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(2887, 334, 336);
                s.store_mul_add_scaled_product_rhs(2888, 154, s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0));
            }
            if (((s.v[2621] != 0.0) && s.b[2960]) && (!s.b[2966])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(2887, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
                s.store_mul_sub_ad_rhs(2888, 154, A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335)));
            }
            s.b[2967] = (((s.v[116]) as f64).abs() < 5e-5);
            s.store_scalar(2967, if s.b[2967] { 1.0 } else { 0.0 });
            if (((s.v[2621] != 0.0) && s.b[2960]) && s.b[2967]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            s.b[2968] = (((s.v[116]) as f64).abs() < 60.0);
            s.store_scalar(2968, if s.b[2968] { 1.0 } else { 0.0 });
            if ((((s.v[2621] != 0.0) && s.b[2960]) && (!s.b[2967])) && s.b[2968]) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if ((((s.v[2621] != 0.0) && s.b[2960]) && (!s.b[2967])) && (!s.b[2968])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2969] = (s.v[214] > 0.0);
            s.store_scalar(2969, if s.b[2969] { 1.0 } else { 0.0 });
            if (((s.v[2621] != 0.0) && s.b[2960]) && s.b[2969]) {
                s.store_sqrt_add(216, 2887, 214);
                s.store_div_scaled_inputs2_indices(217, 2888, 0.5, 215, 0.5, 216, 1.0);
            }
            s.b[2970] = (s.v[2887] > 0.0);
            s.store_scalar(2970, if s.b[2970] { 1.0 } else { 0.0 });
            if ((((s.v[2621] != 0.0) && s.b[2960]) && (!s.b[2969])) && s.b[2970]) {
                s.store_sqrt(216, 2887);
                s.store_div_scaled_inputs_indices(217, 2888, 0.5, 216, 1.0);
            }
            if ((((s.v[2621] != 0.0) && s.b[2960]) && (!s.b[2969])) && (!s.b[2970])) {
                s.store_scalar(216, 0.0);
                s.store_scalar(217, 0.0);
            }
            if ((s.v[2621] != 0.0) && s.b[2960]) {
                s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2621] != 0.0) && s.b[2960]) {
                s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2621] != 0.0) && s.b[2960]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2971] = (s.v[79] > 0.0);
            s.store_scalar(2971, if s.b[2971] { 1.0 } else { 0.0 });
            let (assign84680_body56_e130275,) = {
    if (((s.v[2621] != 0.0) && s.b[2960]) && s.b[2971]) {
        let assign84680_body56_e130273: f64 = (s.v[421] + 1.0);
        (assign84680_body56_e130273,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign84680_body56_e130275);
            if (((s.v[2621] != 0.0) && s.b[2960]) && (!s.b[2971])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if (((s.v[2621] != 0.0) && s.b[2960]) && (!s.b[2971])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2972] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.store_scalar(2972, if s.b[2972] { 1.0 } else { 0.0 });
            if ((((s.v[2621] != 0.0) && s.b[2960]) && (!s.b[2971])) && s.b[2972]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.v[2621] != 0.0) && s.b[2960]) && (!s.b[2971])) {
                s.store_add(404, 404, 236);
            }
            s.b[2973] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.store_scalar(2973, if s.b[2973] { 1.0 } else { 0.0 });
            let (assign84680_body63_e130365,) = {
    if ((((s.v[2621] != 0.0) && s.b[2960]) && (!s.b[2971])) && s.b[2973]) {
        let assign84680_body63_e130363: f64 = (s.v[79] + 2.0);
        (assign84680_body63_e130363,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, assign84680_body63_e130365);
            let (assign84680_body64_e130373,) = {
    if ((s.v[2621] != 0.0) && s.b[2960]) {
        let assign84680_body64_e130371: f64 = (s.v[97] + 1.0);
        (assign84680_body64_e130371,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign84680_body64_e130373);
        }

        if ((s.v[2621] != 0.0) && s.b[2960]) {
            if (s.v[2887] >= 0.0) {
                s.store_scaled_sqrt(223, 2887, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            } else {
                s.store_scalar(223, 0.0);
            }
        }

        if ((s.v[2621] != 0.0) && s.b[2960]) {
            s.store_mul(2877, 982, 223);
            s.store_mul(2878, 2879, 2877);
            s.store_offset_div(100, 2878, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        if (s.v[2621] != 0.0) {
            s.store_sub(399, 398, 354);
        }

        s.b[2975] = (s.v[407] < 0.0);
        s.store_scalar(2975, if s.b[2975] { 1.0 } else { 0.0 });

        if ((s.v[2621] != 0.0) && s.b[2975]) {
            s.store_neg(407, 407);
        }

        s.b[2976] = (p.p55 == 0.0);
        s.store_scalar(2976, if s.b[2976] { 1.0 } else { 0.0 });

        s.b[2977] = (p.p50 == 0.0);
        s.store_scalar(2977, if s.b[2977] { 1.0 } else { 0.0 });

        if ((((s.v[2621] != 0.0) && s.b[2975]) && s.b[2976]) && s.b[2977]) {
            s.store_neg(2880, 404);
        }

        if ((((s.v[2621] != 0.0) && s.b[2975]) && s.b[2976]) && (!s.b[2977])) {
            s.copy_ad(2880, 396);
        }

        if (((s.v[2621] != 0.0) && s.b[2975]) && s.b[2976]) {
            s.store_sqrt_offset_square_offset(782, 2880, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(2880), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(2880), p.p137), 782, 0.5);
        }

        s.b[2978] = (s.v[336] < 0.0);
        s.store_scalar(2978, if s.b[2978] { 1.0 } else { 0.0 });

        if ((((s.v[2621] != 0.0) && s.b[2975]) && s.b[2976]) && s.b[2978]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && s.b[2975]) && s.b[2976]) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_add_scaled_inputs3_indices(781, 407, 1.0, 600, (-1.0), 407, (-0.1));
            s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));
        }

        if (((s.v[2621] != 0.0) && s.b[2975]) && s.b[2976]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.v[2621] != 0.0) && s.b[2975]) && s.b[2976]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(603, 407, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_sub(407, 407, 603);
        }

        s.b[2979] = (3.0 == 1.0);
        s.store_scalar(2979, if s.b[2979] { 1.0 } else { 0.0 });

        s.b[2980] = (3.0 == 2.0);
        s.store_scalar(2980, if s.b[2980] { 1.0 } else { 0.0 });

        s.b[2981] = (3.0 == 3.0);
        s.store_scalar(2981, if s.b[2981] { 1.0 } else { 0.0 });

        s.b[2982] = (3.0 == 4.0);
        s.store_scalar(2982, if s.b[2982] { 1.0 } else { 0.0 });

        s.b[2983] = (p.p55 == 1.0);
        s.store_scalar(2983, if s.b[2983] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && s.b[2979]) && s.b[2983]) {
            s.store_scale(338, 407, var_weffcv_nf);
        }

        if (((s.v[2621] != 0.0) && s.b[2979]) && (!s.b[2983])) {
            s.store_scale(338, 407, (var_weffcv_nf * (1.0 - s.v[526])));
        }

        if ((s.v[2621] != 0.0) && s.b[2979]) {
            s.store_mul(353, 338, 398);
            s.store_mul(356, 338, 354);
        }

        if ((s.v[2621] != 0.0) && (s.b[2980] && (!s.b[2979]))) {
            s.store_scale(338, 407, (var_weffcv_nf * s.v[526]));
            s.store_mul(351, 338, 398);
            s.store_mul(359, 338, 354);
        }

        s.b[2984] = (p.p55 == 1.0);
        s.store_scalar(2984, if s.b[2984] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && (s.b[2981] && (!(s.b[2979] || s.b[2980])))) && s.b[2984]) {
            s.store_scale(338, 407, var_weffcv_nf);
        }

        if (((s.v[2621] != 0.0) && (s.b[2981] && (!(s.b[2979] || s.b[2980])))) && (!s.b[2984])) {
            s.store_scale(338, 407, (var_weffcv_nf * (1.0 - s.v[526])));
        }

        if ((s.v[2621] != 0.0) && (s.b[2981] && (!(s.b[2979] || s.b[2980])))) {
            s.copy_ad(697, 404);
        }

        s.b[2985] = (p.p430 == 0.0);
        s.store_scalar(2985, if s.b[2985] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && (s.b[2981] && (!(s.b[2979] || s.b[2980])))) && s.b[2985]) {
            s.copy_ad(698, 354);
        }

        if ((s.v[2621] != 0.0) && (s.b[2981] && (!(s.b[2979] || s.b[2980])))) {
            s.store_mul(352, 338, 398);
            s.store_mul(355, 338, 354);
            s.copy_ad(816, 355);
        }

        if ((s.v[2621] != 0.0) && (s.b[2982] && (!((s.b[2979] || s.b[2980]) || s.b[2981])))) {
            s.store_scale(338, 407, (var_weffcv_nf * s.v[526]));
            s.store_mul(350, 338, 398);
            s.store_mul(358, 338, 354);
        }

        s.store_scalar(2621, 0.0);

        let assign85260_e130940: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard1990 = assign85260_e130940;

        let assign85270_e130943: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        var_guard1991 = assign85270_e130943;

        let assign85280_e130946: f64 = if 4.0 == 3.0 { 1.0 } else { 0.0 };
        var_guard1992 = assign85280_e130946;

        s.b[2989] = (4.0 == 4.0);
        s.store_scalar(2989, if s.b[2989] { 1.0 } else { 0.0 });

        let assign85300_e130960: f64 = if (((p.p36 == 1.0) && (p.p66 > 0.0)) && (var_uc_novers > 0.0)) { 1.0 } else { 0.0 };
        var_guard1994 = assign85300_e130960;

        let (assign85310_e130966,) = {
    if ((var_guard1990 != 0.0) && (var_guard1994 != 0.0)) {
        (1.0,)
    } else {
        (s.v[2621],)
    }
};
        s.store_scalar(2621, assign85310_e130966);

        let (assign85320_e130972,) = {
    if ((var_guard1990 != 0.0) && (var_guard1994 != 0.0)) {
        (1.0,)
    } else {
        (var_flg_coovlps,)
    }
};
        var_flg_coovlps = assign85320_e130972;

        if ((s.v[2986] != 0.0) && (s.v[2990] != 0.0)) {
            s.store_sub(395, 731, 728);
            s.store_neg(396, 728);
            s.store_scalar(409, var_uc_novers);
            s.store_scalar(407, p.p66);
            s.store_scalar(411, 0.0);
            s.copy_ad(410, 687);
            s.store_scalar(413, var_cox0);
        }

        s.b[2991] = (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.store_scalar(2991, if s.b[2991] { 1.0 } else { 0.0 });

        let (assign85410_e131045,) = {
    if (((var_guard1991 != 0.0) && (var_guard1990 == 0.0)) && s.b[2991]) {
        (1.0,)
    } else {
        (s.v[2621],)
    }
};
        s.store_scalar(2621, assign85410_e131045);

        if (((s.v[2987] != 0.0) && (s.v[2986] == 0.0)) && s.b[2991]) {
            s.store_sub_ad_lhs(395, A::from_derivatives(var_vgsei, [var_vgsei_dn0, var_vgsei_dn1, var_vgsei_dn2, var_vgsei_dn3, var_vgsei_dn4, var_vgsei_dn5, var_vgsei_dn6, var_vgsei_dn7, var_vgsei_dn8, var_vgsei_dn9, var_vgsei_dn10, var_vgsei_dn11, var_vgsei_dn12, var_vgsei_dn13, var_vgsei_dn14, var_vgsei_dn15, var_vgsei_dn16, var_vgsei_dn17], [var_vgsei_db0, var_vgsei_db1, var_vgsei_db2, var_vgsei_db3, var_vgsei_db4, var_vgsei_db5, var_vgsei_db6, var_vgsei_db7, var_vgsei_db8, var_vgsei_db9, var_vgsei_db10, var_vgsei_db11]), 735);
            s.store_neg(396, 735);
        }

        let assign85440_e131077: f64 = if (((p.p35 == 1.0) && (p.p63 > 0.0)) && (var_uc_nover > 0.0)) { 1.0 } else { 0.0 };
        var_guard1996 = assign85440_e131077;

        let (assign85450_e131088,) = {
    if (((var_guard1992 != 0.0) && (!((var_guard1990 != 0.0) || (var_guard1991 != 0.0)))) && (var_guard1996 != 0.0)) {
        (1.0,)
    } else {
        (s.v[2621],)
    }
};
        s.store_scalar(2621, assign85450_e131088);

        *var_flg_coovlps_slot = var_flg_coovlps;
        *var_guard1990_slot = var_guard1990;
        *var_guard1991_slot = var_guard1991;
        *var_guard1992_slot = var_guard1992;
        *var_guard1994_slot = var_guard1994;
        *var_guard1996_slot = var_guard1996;
    }

    pub(super) fn stamp_transient_block_88(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_coxb0: f64,
        var_guard1990: f64,
        var_guard1991: f64,
        var_guard1992: f64,
        var_guard1996: f64,
        var_uc_nover: f64,
        var_vdsei: f64,
        var_vdsei_db0: f64,
        var_vdsei_db1: f64,
        var_vdsei_db10: f64,
        var_vdsei_db11: f64,
        var_vdsei_db2: f64,
        var_vdsei_db3: f64,
        var_vdsei_db4: f64,
        var_vdsei_db5: f64,
        var_vdsei_db6: f64,
        var_vdsei_db7: f64,
        var_vdsei_db8: f64,
        var_vdsei_db9: f64,
        var_vdsei_dn0: f64,
        var_vdsei_dn1: f64,
        var_vdsei_dn10: f64,
        var_vdsei_dn11: f64,
        var_vdsei_dn12: f64,
        var_vdsei_dn13: f64,
        var_vdsei_dn14: f64,
        var_vdsei_dn15: f64,
        var_vdsei_dn16: f64,
        var_vdsei_dn17: f64,
        var_vdsei_dn2: f64,
        var_vdsei_dn3: f64,
        var_vdsei_dn4: f64,
        var_vdsei_dn5: f64,
        var_vdsei_dn6: f64,
        var_vdsei_dn7: f64,
        var_vdsei_dn8: f64,
        var_vdsei_dn9: f64,
        var_vgsei: f64,
        var_vgsei_db0: f64,
        var_vgsei_db1: f64,
        var_vgsei_db10: f64,
        var_vgsei_db11: f64,
        var_vgsei_db2: f64,
        var_vgsei_db3: f64,
        var_vgsei_db4: f64,
        var_vgsei_db5: f64,
        var_vgsei_db6: f64,
        var_vgsei_db7: f64,
        var_vgsei_db8: f64,
        var_vgsei_db9: f64,
        var_vgsei_dn0: f64,
        var_vgsei_dn1: f64,
        var_vgsei_dn10: f64,
        var_vgsei_dn11: f64,
        var_vgsei_dn12: f64,
        var_vgsei_dn13: f64,
        var_vgsei_dn14: f64,
        var_vgsei_dn15: f64,
        var_vgsei_dn16: f64,
        var_vgsei_dn17: f64,
        var_vgsei_dn2: f64,
        var_vgsei_dn3: f64,
        var_vgsei_dn4: f64,
        var_vgsei_dn5: f64,
        var_vgsei_dn6: f64,
        var_vgsei_dn7: f64,
        var_vgsei_dn8: f64,
        var_vgsei_dn9: f64,
        var_flg_coovlp_slot: &mut f64,
    ) {
        let mut var_flg_coovlp: f64 = *var_flg_coovlp_slot;

        let (assign85460_e131099,) = {
    if (((var_guard1992 != 0.0) && (!((var_guard1990 != 0.0) || (var_guard1991 != 0.0)))) && (var_guard1996 != 0.0)) {
        (1.0,)
    } else {
        (var_flg_coovlp,)
    }
};
        var_flg_coovlp = assign85460_e131099;

        if (((s.v[2988] != 0.0) && (!((s.v[2986] != 0.0) || (s.v[2987] != 0.0)))) && (s.v[2992] != 0.0)) {
            s.store_sub(395, 731, 728);
            s.store_sub(396, 729, 728);
            s.store_scalar(409, var_uc_nover);
            s.store_scalar(407, (p.p63 + (p.p64 * p.p55)));
            s.copy_ad(411, 384);
            s.copy_ad(410, 686);
            s.store_scalar(413, var_coxb0);
            s.store_neg(407, 407);
        }

        s.b[2993] = (((s.v[407] < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0));
        s.store_scalar(2993, if s.b[2993] { 1.0 } else { 0.0 });

        if ((((s.v[2988] != 0.0) && (!((s.v[2986] != 0.0) || (s.v[2987] != 0.0)))) && (s.v[2992] != 0.0)) && s.b[2993]) {
            s.store_neg(407, 407);
            s.store_scalar(335, p.p63);
            s.store_offset_div_scaled_product(996, s.ad_value(335), s.ad_value(335), 1.0, s.ad_value(651), 1.0, (-p.p137));
        }

        s.b[2994] = (p.p113 > 0.0);
        s.store_scalar(2994, if s.b[2994] { 1.0 } else { 0.0 });

        s.b[2995] = ((s.v[396] == 0.0) || (p.p113 <= 0.0));
        s.store_scalar(2995, if s.b[2995] { 1.0 } else { 0.0 });

        if ((((((s.v[2988] != 0.0) && (!((s.v[2986] != 0.0) || (s.v[2987] != 0.0)))) && (s.v[2992] != 0.0)) && s.b[2993]) && s.b[2994]) && s.b[2995]) {
        }

        if ((((((s.v[2988] != 0.0) && (!((s.v[2986] != 0.0) || (s.v[2987] != 0.0)))) && (s.v[2992] != 0.0)) && s.b[2993]) && s.b[2994]) && (!s.b[2995])) {
            s.store_scalar(783, (if (s.v[396] < 0.0) { (-1.0) } else { 1.0 }));
        }

        if ((((((s.v[2988] != 0.0) && (!((s.v[2986] != 0.0) || (s.v[2987] != 0.0)))) && (s.v[2992] != 0.0)) && s.b[2993]) && s.b[2994]) && (!s.b[2995])) {
            s.store_mul(784, 783, 396);
            s.store_offset_powf_ad(781, A::div(s.ad_value(784), s.ad_value(996)), p.p113, 1.0);
            s.store_powf(782, 781, (1.0 / p.p113));
            s.store_div_scaled_product_indices(396, 783, 784, 1.0, 782, 1.0);
        }

        if (((((s.v[2988] != 0.0) && (!((s.v[2986] != 0.0) || (s.v[2987] != 0.0)))) && (s.v[2992] != 0.0)) && s.b[2993]) && s.b[2994]) {
            s.store_sqrt_offset_square_offset(782, 396, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(396), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(396), p.p137), 782, 0.5);
        }

        s.b[2996] = (s.v[336] < 0.0);
        s.store_scalar(2996, if s.b[2996] { 1.0 } else { 0.0 });

        if ((((((s.v[2988] != 0.0) && (!((s.v[2986] != 0.0) || (s.v[2987] != 0.0)))) && (s.v[2992] != 0.0)) && s.b[2993]) && s.b[2994]) && s.b[2996]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((((s.v[2988] != 0.0) && (!((s.v[2986] != 0.0) || (s.v[2987] != 0.0)))) && (s.v[2992] != 0.0)) && s.b[2993]) && s.b[2994]) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub(407, 407, 600);
        }

        s.b[2997] = (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.store_scalar(2997, if s.b[2997] { 1.0 } else { 0.0 });

        let (assign85760_e131570,) = {
    if ((s.b[2989] && (!(((var_guard1990 != 0.0) || (var_guard1991 != 0.0)) || (var_guard1992 != 0.0)))) && s.b[2997]) {
        (1.0,)
    } else {
        (s.v[2621],)
    }
};
        s.store_scalar(2621, assign85760_e131570);

        if ((s.b[2989] && (!(((s.v[2986] != 0.0) || (s.v[2987] != 0.0)) || (s.v[2988] != 0.0)))) && s.b[2997]) {
            s.store_sub_ad_lhs(395, A::from_derivatives(var_vgsei, [var_vgsei_dn0, var_vgsei_dn1, var_vgsei_dn2, var_vgsei_dn3, var_vgsei_dn4, var_vgsei_dn5, var_vgsei_dn6, var_vgsei_dn7, var_vgsei_dn8, var_vgsei_dn9, var_vgsei_dn10, var_vgsei_dn11, var_vgsei_dn12, var_vgsei_dn13, var_vgsei_dn14, var_vgsei_dn15, var_vgsei_dn16, var_vgsei_dn17], [var_vgsei_db0, var_vgsei_db1, var_vgsei_db2, var_vgsei_db3, var_vgsei_db4, var_vgsei_db5, var_vgsei_db6, var_vgsei_db7, var_vgsei_db8, var_vgsei_db9, var_vgsei_db10, var_vgsei_db11]), 735);
            s.store_sub_ad_lhs(396, A::from_derivatives(var_vdsei, [var_vdsei_dn0, var_vdsei_dn1, var_vdsei_dn2, var_vdsei_dn3, var_vdsei_dn4, var_vdsei_dn5, var_vdsei_dn6, var_vdsei_dn7, var_vdsei_dn8, var_vdsei_dn9, var_vdsei_dn10, var_vdsei_dn11, var_vdsei_dn12, var_vdsei_dn13, var_vdsei_dn14, var_vdsei_dn15, var_vdsei_dn16, var_vdsei_dn17], [var_vdsei_db0, var_vdsei_db1, var_vdsei_db2, var_vdsei_db3, var_vdsei_db4, var_vdsei_db5, var_vdsei_db6, var_vdsei_db7, var_vdsei_db8, var_vdsei_db9, var_vdsei_db10, var_vdsei_db11]), 735);
        }

        if (s.v[2621] != 0.0) {
            s.store_scalar(3005, 0.4);
        }

        let (assign85810_e131612,) = {
    if (s.v[2621] != 0.0) {
        (0.0,)
    } else {
        (s.v[3006],)
    }
};
        s.store_scalar(3006, assign85810_e131612);

        if (s.v[2621] != 0.0) {
            s.store_scalar(223, 0.0);
            s.store_scalar(214, 0.0);
            s.store_scalar(216, 0.0);
            s.store_scalar(232, 0.0);
            s.store_scalar(236, 0.0);
            s.store_scalar(233, 0.0);
            s.store_scalar(217, 0.0);
            s.store_scalar(420, 0.0);
            s.store_scalar(215, 0.0);
            s.store_scalar(447, 0.0);
            s.store_scalar(445, 0.0);
            s.store_scalar(446, 0.0);
        }

        let (assign85940_e131665,) = {
    if (s.v[2621] != 0.0) {
        let assign85940_e131663: f64 = (-1.0);
        (assign85940_e131663,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign85940_e131665);

        if (s.v[2621] != 0.0) {
            s.store_scalar(3007, 0.0);
            s.store_scalar(3008, 0.0);
            s.store_mul_scaled_ln_ad_rhs(3003, 155, 2.0, A::div(s.ad_value(409), s.ad_value(394)));
            s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(3003), (-0.1));
            s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        }

        if (s.v[2621] != 0.0) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.v[2621] != 0.0) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(3004, 781, (-0.5), 782, (-0.5), 0.8);
        }

        s.b[3010] = (s.v[3005] > (s.v[3004] * 0.5));
        s.store_scalar(3010, if s.b[3010] { 1.0 } else { 0.0 });

        if ((s.v[2621] != 0.0) && s.b[3010]) {
            s.store_scale(3005, 3004, 0.5);
        }

        s.b[3011] = param_given[338];
        s.store_scalar(3011, if s.b[3011] { 1.0 } else { 0.0 });

        if ((s.v[2621] != 0.0) && s.b[3011]) {
            s.store_scalar(3004, p.p338);
        }

        s.b[3012] = param_given[339];
        s.store_scalar(3012, if s.b[3012] { 1.0 } else { 0.0 });

        if ((s.v[2621] != 0.0) && s.b[3012]) {
            s.store_scalar(3005, p.p339);
        }

        s.b[3013] = param_given[338];
        s.store_scalar(3013, if s.b[3013] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && (!s.b[3012])) && s.b[3013]) {
            s.store_scale(3005, 3004, 0.5);
        }

        s.b[3014] = (s.v[3005] > (s.v[3004] * 0.5));
        s.store_scalar(3014, if s.b[3014] { 1.0 } else { 0.0 });

        if ((s.v[2621] != 0.0) && s.b[3014]) {
            s.store_scale(3005, 3004, 0.5);
        }

        s.b[3015] = (p.p38 == 1.0);
        s.store_scalar(3015, if s.b[3015] { 1.0 } else { 0.0 });

        if ((s.v[2621] != 0.0) && s.b[3015]) {
            s.store_neg(334, 396);
        }

        s.b[3016] = (s.v[334] > s.v[3005]);
        s.store_scalar(3016, if s.b[3016] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && s.b[3015]) && s.b[3016]) {
            s.store_sub(335, 334, 3005);
            s.store_sub(336, 3004, 3005);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 345, 1.0);
            s.store_neg(345, 345);
            s.store_add(344, 3005, 333);
        }

        if (((s.v[2621] != 0.0) && s.b[3015]) && (!s.b[3016])) {
            s.copy_ad(344, 334);
        }

        if ((s.v[2621] != 0.0) && s.b[3015]) {
            s.store_neg(397, 344);
        }

        if ((s.v[2621] != 0.0) && (!s.b[3015])) {
            s.copy_ad(397, 396);
        }

        if (s.v[2621] != 0.0) {
            s.store_div(212, 410, 413);
            s.store_square(213, 212);
            s.store_sub_from_scalar(402, s.v[458], 395);
        }

        let (assign86350_e132006,) = {
    if (s.v[2621] != 0.0) {
        let assign86350_e132000: f64 = (-s.v[397]);
        let assign86350_e132003: f64 = (10.0 * 2.220446049250313e-16);
        let assign86350_e132004: f64 = (assign86350_e132000 + assign86350_e132003);
        (assign86350_e132004,)
    } else {
        (s.v[403],)
    }
};
        s.store_scalar(403, assign86350_e132006);

        if (s.v[2621] != 0.0) {
            s.store_scalar(2999, 0.0);
            s.store_scale(3000, 409, 1.6021918e-19);
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
        }

        s.b[3017] = ((s.v[154] * (-s.v[397])) >= 500.0);
        s.store_scalar(3017, if s.b[3017] { 1.0 } else { 0.0 });

        if ((s.v[2621] != 0.0) && s.b[3017]) {
            s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(334, 1.403592217853e217);
        }

        if ((s.v[2621] != 0.0) && (!s.b[3017])) {
            s.store_mul_neg_rhs(781, 154, 397);
            s.store_scalar(229, 1.0);
        }

        let mut assign86450_loop_guard: usize = 0;
        while {
            let assign86450_cond_e132080: f64 = if (((s.v[2621] != 0.0) && (!s.b[3017])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign86450_cond_e132080 != 0.0
        } {
            assign86450_loop_guard += 1;
            assert!(assign86450_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2621] != 0.0) && (!s.b[3017])) {
                s.store_scale(229, 229, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if ((s.v[2621] != 0.0) && (!s.b[3017])) {
            s.store_mul_exp_rhs(229, 229, 781);
            s.copy_ad(334, 229);
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));
            s.store_scalar(782, (4.0 * 0.5));
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(335, 781, 0.5, 782, 0.5, 0.5);
        }

        s.b[3018] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));
        s.store_scalar(3018, if s.b[3018] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[3018]) {
            s.store_add_scaled_inputs3_indices(781, 402, 1.0, 397, 1.0, 335, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 335);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign86600_e132254,) = {
    if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[3018]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign86600_e132254);

        let (assign86610_e132262,) = {
    if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[3018]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign86610_e132262);

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[3018]) {
            s.store_scalar(770, 0.0);
        }

        *var_flg_coovlp_slot = var_flg_coovlp;
    }

    pub(super) fn stamp_transient_block_89(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[3018]) {
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3019] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.store_scalar(3019, if s.b[3019] { 1.0 } else { 0.0 });

        s.b[3020] = (1.0 == 1.0);
        s.store_scalar(3020, if s.b[3020] { 1.0 } else { 0.0 });

        let (assign86700_e132346,) = {
    if (((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[3018]) && s.b[3019]) && s.b[3020]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign86700_e132346);

        s.b[3021] = (1.0 == 2.0);
        s.store_scalar(3021, if s.b[3021] { 1.0 } else { 0.0 });

        let (assign86720_e132364,) = {
    if ((((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[3018]) && s.b[3019]) && (!s.b[3020])) && s.b[3021]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign86720_e132364);

        s.b[3022] = (1.0 == 4.0);
        s.store_scalar(3022, if s.b[3022] { 1.0 } else { 0.0 });

        let (assign86740_e132385,) = {
    if (((((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[3018]) && s.b[3019]) && (!s.b[3020])) && (!s.b[3021])) && s.b[3022]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign86740_e132385);

        s.b[3023] = (1.0 == 8.0);
        s.store_scalar(3023, if s.b[3023] { 1.0 } else { 0.0 });

        let (assign86760_e132409,) = {
    if ((((((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[3018]) && s.b[3019]) && (!s.b[3020])) && (!s.b[3021])) && (!s.b[3022])) && s.b[3023]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign86760_e132409);

        let (assign86770_e132419,) = {
    if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[3018]) && s.b[3019]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign86770_e132419);

        let mut assign86780_loop_guard: usize = 0;
        while {
            let assign86780_cond_e132430: f64 = if (((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[3018]) && s.b[3019]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign86780_cond_e132430 != 0.0
        } {
            assign86780_loop_guard += 1;
            assert!(assign86780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[3018]) && s.b[3019]) {
                s.store_sqrt(726, 726);
            }
            let (assign86780_body1_e132453,) = {
    if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[3018]) && s.b[3019]) {
        let assign86780_body1_e132451: f64 = (s.v[719] + 1.0);
        (assign86780_body1_e132451,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign86780_body1_e132453);
        }

        if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[3018]) && (!s.b[3019])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[3018]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);
        }

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[3018]) {
        }

        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && (!s.b[3018])) {
            s.store_add(335, 402, 397);
            s.store_scalar(334, 1.0);
        }

        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sub(397, 335, 402);
        }

        let (assign86880_e132570,) = {
    if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
        let assign86880_e132564: f64 = (-s.v[397]);
        let assign86880_e132567: f64 = (10.0 * 2.220446049250313e-16);
        let assign86880_e132568: f64 = (assign86880_e132564 + assign86880_e132567);
        (assign86880_e132568,)
    } else {
        (s.v[403],)
    }
};
        s.store_scalar(403, assign86880_e132570);

        s.b[3024] = (s.v[402] < s.v[403]);
        s.store_scalar(3024, if s.b[3024] { 1.0 } else { 0.0 });

        if ((s.v[2621] != 0.0) && s.b[3024]) {
            s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));
            s.store_mul_add_rhs(332, 154, 402, 397);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(410));
            s.store_mul(333, 335, 413);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(278, (7.0 * 1.414213562373095), 333, A::offset(s.ad_value(332), (-2.0)), 9.0);
            s.store_square(276, 278);
        }

        s.b[3025] = (s.v[277] < (s.v[276] * 1e-8));
        s.store_scalar(3025, if s.b[3025] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && s.b[3024]) && s.b[3025]) {
            s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);
        }

        if (((s.v[2621] != 0.0) && s.b[3024]) && (!s.b[3025])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((s.v[2621] != 0.0) && s.b[3024]) {
            s.store_powf(273, 274, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);
            s.store_div(116, 272, 273);
            s.store_mul(335, 116, 155);
            s.store_div(336, 335, 271);
            s.store_sqrt_square_offset(337, 336, 1.0);
            s.store_sub_div_lhs_indices(404, 335, 337, 397);
            s.store_sub(336, 402, 404);
            s.store_mul(398, 413, 336);
            s.copy_ad(354, 398);
            s.copy_ad(3007, 404);
        }

        s.b[3026] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));
        s.store_scalar(3026, if s.b[3026] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3026]) {
            s.store_add_scaled_product_indices(89, 402, 1.0, 213, 154, 0.5);
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && (!s.b[3026])) {
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
        }

        if ((s.v[2621] != 0.0) && (!s.b[3024])) {
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        s.b[3027] = (s.v[116] >= 3.0);
        s.store_scalar(3027, if s.b[3027] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3027]) {
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
            s.store_mul_add_rhs(116, 154, 89, 397);
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && (!s.b[3027])) {
            s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));
            s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));
            s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));
            s.store_div_scaled_inputs2_indices(437, 402, -1.0, 397, -1.0, 212, 1.0);
            s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(441, A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(435), s.ad_value(436), 1.0, s.ad_value(434), s.ad_value(434), 6.0), (-1.0), 437, 1.0, 434, 2.0, 1.0);
            s.store_div_ad(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), A::mul_scaled_lhs(s.ad_value(434), 9.0, s.ad_value(434)));
            s.store_sqrt_add_scaled_square_cube_product(339, 441, 1.0, 440, 1.0);
            s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);
            s.store_neg_powf_add_input(438, 441, 339, 0.3333333333333333);
            s.store_add_scaled_inputs3_div_scaled_third_indices(116, 439, 1.0, 438, 1.0, 435, 1.0, 434, 3.0, -1.0);
            s.store_add_scaled_product_indices(89, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[3028] = (p.p33 > 0.0);
        s.store_scalar(3028, if s.b[3028] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) {
            s.store_offset_add(442, 402, 397, 0.1);
            s.store_mul(222, 405, 229);
            s.store_mul(443, 405, 229);
            s.store_mul(334, 156, 213);
            s.store_mul(444, 154, 442);
            s.store_add_scaled_inputs_product_mixed_aaii(447, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), 154, 397, 1.0);
        }

        s.b[3029] = (p.p33 == 2.0);
        s.store_scalar(3029, if s.b[3029] { 1.0 } else { 0.0 });

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3029]) {
            s.store_offset_sub(781, 444, 447, (-1.0));
            s.store_scale(782, 444, 4.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3029]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3029]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(447, 444, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && (!s.b[3029])) {
            if (s.v[447] <= s.v[444]) {
            } else {
                s.copy_ad(447, 444);
            }
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) {
            if (s.v[447] >= 0.0) {
            } else {
                s.store_scalar(447, 0.0);
            }
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) {
            s.store_sub(444, 444, 447);
            s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);
            s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));
            s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) {
            s.copy_ad(445, 116);
        }

        s.b[3030] = (p.p33 == 2.0);
        s.store_scalar(3030, if s.b[3030] { 1.0 } else { 0.0 });

        s.b[3031] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));
        s.store_scalar(3031, if s.b[3031] { 1.0 } else { 0.0 });

        if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && s.b[3031]) {
            s.store_add_scaled_inputs3_indices(781, 445, 1.0, 446, (-1.0), 446, 0.2);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 446, 446, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign87710_e133716,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && s.b[3031]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign87710_e133716);

        let (assign87720_e133729,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && s.b[3031]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign87720_e133729);

        if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && s.b[3031]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
        }

    }

    pub(super) fn stamp_transient_block_90(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && s.b[3031]) {
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3032] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(3032, if s.b[3032] { 1.0 } else { 0.0 });

        s.b[3033] = (2.0 == 1.0);
        s.store_scalar(3033, if s.b[3033] { 1.0 } else { 0.0 });

        let (assign87830_e133878,) = {
    if (((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && s.b[3031]) && s.b[3032]) && s.b[3033]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign87830_e133878);

        s.b[3034] = (2.0 == 2.0);
        s.store_scalar(3034, if s.b[3034] { 1.0 } else { 0.0 });

        let (assign87850_e133901,) = {
    if ((((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && s.b[3031]) && s.b[3032]) && (!s.b[3033])) && s.b[3034]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign87850_e133901);

        s.b[3035] = (2.0 == 4.0);
        s.store_scalar(3035, if s.b[3035] { 1.0 } else { 0.0 });

        let (assign87870_e133927,) = {
    if (((((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && s.b[3031]) && s.b[3032]) && (!s.b[3033])) && (!s.b[3034])) && s.b[3035]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign87870_e133927);

        s.b[3036] = (2.0 == 8.0);
        s.store_scalar(3036, if s.b[3036] { 1.0 } else { 0.0 });

        let (assign87890_e133956,) = {
    if ((((((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && s.b[3031]) && s.b[3032]) && (!s.b[3033])) && (!s.b[3034])) && (!s.b[3035])) && s.b[3036]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign87890_e133956);

        let (assign87900_e133971,) = {
    if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && s.b[3031]) && s.b[3032]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign87900_e133971);

        let mut assign87910_loop_guard: usize = 0;
        while {
            let assign87910_cond_e133987: f64 = if (((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && s.b[3031]) && s.b[3032]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign87910_cond_e133987 != 0.0
        } {
            assign87910_loop_guard += 1;
            assert!(assign87910_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && s.b[3031]) && s.b[3032]) {
                s.store_sqrt(726, 726);
            }
            let (assign87910_body1_e134020,) = {
    if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && s.b[3031]) && s.b[3032]) {
        let assign87910_body1_e134018: f64 = (s.v[719] + 1.0);
        (assign87910_body1_e134018,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign87910_body1_e134020);
        }

        if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && s.b[3031]) && (!s.b[3032])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && s.b[3031]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);
            s.store_div_scaled_product3_indices(335, 446, 725, 726, 0.2, 770, 1.0);
            s.store_add_scaled_inputs3_indices(116, 446, 1.0, 446, (-0.2), 780, 1.0);
        }

        if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && s.b[3031]) {
        }

        if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && s.b[3030]) && (!s.b[3031])) {
            s.copy_ad(116, 445);
            s.store_scalar(335, 1.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3028]) && (!s.b[3030])) {
            if (s.v[445] <= s.v[446]) {
                s.copy_ad(116, 445);
            } else {
                s.copy_ad(116, 446);
            }
        }

        s.b[3037] = (p.p33 == 1.0);
        s.store_scalar(3037, if s.b[3037] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3037]) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[3038] = (s.v[411] > 0.0);
        s.store_scalar(3038, if s.b[3038] { 1.0 } else { 0.0 });

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3037]) && s.b[3038]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3037]) && (!s.b[3038])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[3039] = (s.v[336] < 0.0);
        s.store_scalar(3039, if s.b[3039] { 1.0 } else { 0.0 });

        if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3037]) && (!s.b[3038])) && s.b[3039]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3037]) && (!s.b[3038])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3037]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[3040] = (s.v[336] < 0.0);
        s.store_scalar(3040, if s.b[3040] { 1.0 } else { 0.0 });

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3037]) && s.b[3040]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3037]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 3000, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(419, 335, 418);
            s.store_add(414, 404, 397);
            s.store_mul_sub_rhs(333, 419, 414, 418);
        }

        s.b[3041] = (s.v[333] < 60.0);
        s.store_scalar(3041, if s.b[3041] { 1.0 } else { 0.0 });

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3037]) && s.b[3041]) {
            s.store_exp(335, 333);
            s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
            s.store_sub(336, 335, 334);
            s.store_div_ln_offset_lhs(416, 336, 1.0, 419);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3037]) && (!s.b[3041])) {
            s.store_sub(416, 414, 418);
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3037]) {
            s.store_mul(415, 154, 416);
        }

        s.b[3042] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));
        s.store_scalar(3042, if s.b[3042] { 1.0 } else { 0.0 });

        let (assign88340_e134609,) = {
    if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3037]) && s.b[3042]) {
        let assign88340_e134607: f64 = (s.v[3006] + 1.0);
        (assign88340_e134607,)
    } else {
        (s.v[3006],)
    }
};
        s.store_scalar(3006, assign88340_e134609);

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3037]) && s.b[3042]) {
            s.copy_ad(116, 447);
        }

        if ((s.v[2621] != 0.0) && (!s.b[3024])) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[3043] = (((s.v[116]) as f64).abs() > 1e-6);
        s.store_scalar(3043, if s.b[3043] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3043]) {
            s.store_add_offset_lhs_ad_rhs(335, 116, (-1.0), A::exp_scaled_input(s.ad_value(116), -1.0));
            s.store_sqrt(336, 335);
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && (!s.b[3043])) {
            s.store_mul_scaled_sqrt_ad_rhs(336, 116, 0.7071067811865475, A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333)));
        }

        if ((s.v[2621] != 0.0) && (!s.b[3024])) {
            s.store_mul(354, 410, 336);
            s.store_mul_sub_rhs(398, 413, 402, 404);
            s.store_div(3044, 354, 3000);
        }

        s.b[3046] = (p.p33 == 2.0);
        s.store_scalar(3046, if s.b[3046] { 1.0 } else { 0.0 });

        s.b[3047] = ((s.v[3044] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));
        s.store_scalar(3047, if s.b[3047] { 1.0 } else { 0.0 });

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && s.b[3047]) {
            s.store_add_scaled_inputs3_indices(781, 3044, 1.0, 386, (-1.0), 386, 0.1);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 386, 386, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign88520_e134816,) = {
    if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && s.b[3047]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign88520_e134816);

        let (assign88530_e134827,) = {
    if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && s.b[3047]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign88530_e134827);

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && s.b[3047]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3048] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(3048, if s.b[3048] { 1.0 } else { 0.0 });

        s.b[3049] = (2.0 == 1.0);
        s.store_scalar(3049, if s.b[3049] { 1.0 } else { 0.0 });

        let (assign88640_e134958,) = {
    if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && s.b[3047]) && s.b[3048]) && s.b[3049]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign88640_e134958);

        s.b[3050] = (2.0 == 2.0);
        s.store_scalar(3050, if s.b[3050] { 1.0 } else { 0.0 });

        let (assign88660_e134979,) = {
    if (((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && s.b[3047]) && s.b[3048]) && (!s.b[3049])) && s.b[3050]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign88660_e134979);

        s.b[3051] = (2.0 == 4.0);
        s.store_scalar(3051, if s.b[3051] { 1.0 } else { 0.0 });

        let (assign88680_e135003,) = {
    if ((((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && s.b[3047]) && s.b[3048]) && (!s.b[3049])) && (!s.b[3050])) && s.b[3051]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign88680_e135003);

        s.b[3052] = (2.0 == 8.0);
        s.store_scalar(3052, if s.b[3052] { 1.0 } else { 0.0 });

        let (assign88700_e135030,) = {
    if (((((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && s.b[3047]) && s.b[3048]) && (!s.b[3049])) && (!s.b[3050])) && (!s.b[3051])) && s.b[3052]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign88700_e135030);

        let (assign88710_e135043,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && s.b[3047]) && s.b[3048]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign88710_e135043);

        let mut assign88720_loop_guard: usize = 0;
        while {
            let assign88720_cond_e135057: f64 = if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && s.b[3047]) && s.b[3048]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign88720_cond_e135057 != 0.0
        } {
            assign88720_loop_guard += 1;
            assert!(assign88720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && s.b[3047]) && s.b[3048]) {
                s.store_sqrt(726, 726);
            }
            let (assign88720_body1_e135086,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && s.b[3047]) && s.b[3048]) {
        let assign88720_body1_e135084: f64 = (s.v[719] + 1.0);
        (assign88720_body1_e135084,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign88720_body1_e135086);
        }

        if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && s.b[3047]) && (!s.b[3048])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && s.b[3047]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 386, 725, 726, 0.1, 770, 1.0);
            s.store_add_scaled_inputs3_indices(335, 386, 1.0, 386, (-0.1), 780, 1.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && s.b[3047]) {
        }

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && (!s.b[3047])) {
            s.copy_ad(335, 3044);
        }

    }

    pub(super) fn stamp_transient_block_91(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && (!s.b[3047])) {
            s.store_scalar(334, 1.0);
        }

        s.b[3053] = (s.v[334] < 1.0);
        s.store_scalar(3053, if s.b[3053] { 1.0 } else { 0.0 });

        let (assign88820_e135228,) = {
    if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3046]) && s.b[3053]) {
        let assign88820_e135226: f64 = (s.v[3006] + 2.0);
        (assign88820_e135226,)
    } else {
        (s.v[3006],)
    }
};
        s.store_scalar(3006, assign88820_e135228);

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && (!s.b[3046])) {
            if (s.v[3044] <= s.v[386]) {
                s.copy_ad(335, 3044);
            } else {
                s.copy_ad(335, 386);
            }
        }

        s.b[3054] = (s.v[3044] >= s.v[386]);
        s.store_scalar(3054, if s.b[3054] { 1.0 } else { 0.0 });

        let (assign88850_e135260,) = {
    if ((((s.v[2621] != 0.0) && (!s.b[3024])) && (!s.b[3046])) && s.b[3054]) {
        let assign88850_e135258: f64 = (s.v[3006] + 2.0);
        (assign88850_e135258,)
    } else {
        (s.v[3006],)
    }
};
        s.store_scalar(3006, assign88850_e135260);

        s.b[3055] = (s.v[3006] >= 2.0);
        s.store_scalar(3055, if s.b[3055] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) {
            s.copy_ad(3045, 404);
            s.store_mul(354, 335, 3000);
            s.store_sub_div_rhs_indices(404, 402, 354, 413);
        }

        s.b[3056] = (p.p33 == 2.0);
        s.store_scalar(3056, if s.b[3056] { 1.0 } else { 0.0 });

        s.b[3057] = ((s.v[404] > (s.v[3045] - 0.1)) && (0.1 >= 0.0));
        s.store_scalar(3057, if s.b[3057] { 1.0 } else { 0.0 });

        if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && s.b[3057]) {
            s.store_offset_sub(781, 404, 3045, 0.1);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign88970_e135394,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && s.b[3057]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign88970_e135394);

        let (assign88980_e135407,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && s.b[3057]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign88980_e135407);

        if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && s.b[3057]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3058] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(3058, if s.b[3058] { 1.0 } else { 0.0 });

        s.b[3059] = (2.0 == 1.0);
        s.store_scalar(3059, if s.b[3059] { 1.0 } else { 0.0 });

        let (assign89090_e135556,) = {
    if (((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && s.b[3057]) && s.b[3058]) && s.b[3059]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign89090_e135556);

        s.b[3060] = (2.0 == 2.0);
        s.store_scalar(3060, if s.b[3060] { 1.0 } else { 0.0 });

        let (assign89110_e135579,) = {
    if ((((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && s.b[3057]) && s.b[3058]) && (!s.b[3059])) && s.b[3060]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign89110_e135579);

        s.b[3061] = (2.0 == 4.0);
        s.store_scalar(3061, if s.b[3061] { 1.0 } else { 0.0 });

        let (assign89130_e135605,) = {
    if (((((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && s.b[3057]) && s.b[3058]) && (!s.b[3059])) && (!s.b[3060])) && s.b[3061]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign89130_e135605);

        s.b[3062] = (2.0 == 8.0);
        s.store_scalar(3062, if s.b[3062] { 1.0 } else { 0.0 });

        let (assign89150_e135634,) = {
    if ((((((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && s.b[3057]) && s.b[3058]) && (!s.b[3059])) && (!s.b[3060])) && (!s.b[3061])) && s.b[3062]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign89150_e135634);

        let (assign89160_e135649,) = {
    if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && s.b[3057]) && s.b[3058]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign89160_e135649);

        let mut assign89170_loop_guard: usize = 0;
        while {
            let assign89170_cond_e135665: f64 = if (((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && s.b[3057]) && s.b[3058]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign89170_cond_e135665 != 0.0
        } {
            assign89170_loop_guard += 1;
            assert!(assign89170_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && s.b[3057]) && s.b[3058]) {
                s.store_sqrt(726, 726);
            }
            let (assign89170_body1_e135698,) = {
    if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && s.b[3057]) && s.b[3058]) {
        let assign89170_body1_e135696: f64 = (s.v[719] + 1.0);
        (assign89170_body1_e135696,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign89170_body1_e135698);
        }

        if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && s.b[3057]) && (!s.b[3058])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && s.b[3057]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_add_offset_lhs(404, 3045, (-0.1), 780);
        }

        if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && s.b[3057]) {
        }

        if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && (!s.b[3057])) {
        }

        if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && s.b[3056]) && (!s.b[3057])) {
            s.store_scalar(334, 1.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3055]) && (!s.b[3056])) {
            if (s.v[404] <= s.v[3045]) {
            } else {
                s.copy_ad(404, 3045);
            }
        }

        if ((s.v[2621] != 0.0) && (!s.b[3024])) {
            s.copy_ad(3007, 404);
        }

        s.b[3063] = (p.p33 == 1.0);
        s.store_scalar(3063, if s.b[3063] { 1.0 } else { 0.0 });

        let (assign89290_e135870,) = {
    if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign89290_e135870);

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) {
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(3000)), s.ad_value(155)), 2.0);
        }

        s.b[3064] = (s.v[411] > 0.0);
        s.store_scalar(3064, if s.b[3064] { 1.0 } else { 0.0 });

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && s.b[3064]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3064])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[3065] = (s.v[336] < 0.0);
        s.store_scalar(3065, if s.b[3065] { 1.0 } else { 0.0 });

        if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3064])) && s.b[3065]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3064])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[3066] = (s.v[336] < 0.0);
        s.store_scalar(3066, if s.b[3066] { 1.0 } else { 0.0 });

        if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && s.b[3066]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 3000, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(419, 335, 418);
        }

        let (assign89520_e136179,) = {
    if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, assign89520_e136179);

    }

    pub(super) fn stamp_transient_block_92(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign89530_loop_guard: usize = 0;
        while {
            let assign89530_cond_e136189: f64 = (s.v[421] + 1.0);
            let assign89530_cond_e136191: f64 = if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (s.v[97] <= assign89530_cond_e136189)) { 1.0 } else { 0.0 };
            assign89530_cond_e136191 != 0.0
        } {
            assign89530_loop_guard += 1;
            assert!(assign89530_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[3068] = (s.v[333] < 60.0);
            s.store_scalar(3068, if s.b[3068] { 1.0 } else { 0.0 });
            if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && s.b[3068]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ln_offset_lhs(416, 336, 1.0, 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3068])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) {
                s.store_mul(415, 154, 416);
            }
            s.b[3069] = (s.v[116] < 0.0);
            s.store_scalar(3069, if s.b[3069] { 1.0 } else { 0.0 });
            if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && s.b[3069]) {
                s.store_scalar(334, (-0.7071067811865475));
                s.store_mul(223, 116, 334);
                s.store_mul(420, 154, 334);
            }
            s.b[3070] = (s.v[116] < 1e-6);
            s.store_scalar(3070, if s.b[3070] { 1.0 } else { 0.0 });
            if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3069])) && s.b[3070]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(338, 334, 336);
            }
            s.b[3071] = (s.v[338] > 0.0);
            s.store_scalar(3071, if s.b[3071] { 1.0 } else { 0.0 });
            if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3069])) && s.b[3070]) && s.b[3071]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);
            }
            if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3069])) && s.b[3070]) && (!s.b[3071])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3069])) && (!s.b[3070])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(338, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
            }
            s.b[3072] = (s.v[338] > 0.0);
            s.store_scalar(3072, if s.b[3072] { 1.0 } else { 0.0 });
            if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3069])) && (!s.b[3070])) && s.b[3072]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335))), 0.5, 223, 1.0);
            }
            if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3069])) && (!s.b[3070])) && (!s.b[3072])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            s.b[3073] = (s.v[116] < 0.0);
            s.store_scalar(3073, if s.b[3073] { 1.0 } else { 0.0 });
            if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && s.b[3073]) {
                s.store_scalar(214, 0.0);
                s.store_scalar(215, 0.0);
                s.store_neg(216, 223);
                s.store_neg(217, 420);
            }
            s.b[3074] = (s.v[116] < 60.0);
            s.store_scalar(3074, if s.b[3074] { 1.0 } else { 0.0 });
            s.b[3075] = (s.v[116] < 5e-5);
            s.store_scalar(3075, if s.b[3075] { 1.0 } else { 0.0 });
            if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3073])) && s.b[3074]) && s.b[3075]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            if ((((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3073])) && s.b[3074]) && (!s.b[3075])) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3073])) && (!s.b[3074])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[3076] = (s.v[214] > 0.0);
            s.store_scalar(3076, if s.b[3076] { 1.0 } else { 0.0 });
            if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3073])) && s.b[3076]) {
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_scaled_add_product(217, s.ad_value(215), 0.5, s.ad_value(420), s.ad_value(223), (2.0 * 0.5), s.ad_value(216), 1.0);
            }
            if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3073])) && (!s.b[3076])) {
                s.copy_ad(216, 223);
                s.copy_ad(217, 420);
            }
            if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[3077] = (s.v[79] == 1.0);
            s.store_scalar(3077, if s.b[3077] { 1.0 } else { 0.0 });
            let (assign89530_body72_e137337,) = {
    if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && s.b[3077]) {
        let assign89530_body72_e137335: f64 = (s.v[421] + 1.0);
        (assign89530_body72_e137335,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign89530_body72_e137337);
            if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3077])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3077])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[3078] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.store_scalar(3078, if s.b[3078] { 1.0 } else { 0.0 });
            if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3077])) && s.b[3078]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3077])) {
                s.store_add(404, 404, 236);
            }
            s.b[3079] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.store_scalar(3079, if s.b[3079] { 1.0 } else { 0.0 });
            let (assign89530_body79_e137440,) = {
    if (((((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) && (!s.b[3077])) && s.b[3079]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, assign89530_body79_e137440);
            let (assign89530_body80_e137451,) = {
    if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) {
        let assign89530_body80_e137449: f64 = (s.v[97] + 1.0);
        (assign89530_body80_e137449,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign89530_body80_e137451);
        }

        if (((s.v[2621] != 0.0) && (!s.b[3024])) && s.b[3063]) {
            s.store_mul(2998, 982, 223);
            s.store_mul(2999, 3000, 2998);
            s.store_offset_div(100, 2999, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        s.b[3081] = (p.p33 == 4.0);
        s.store_scalar(3081, if s.b[3081] { 1.0 } else { 0.0 });

        if ((s.v[2621] != 0.0) && s.b[3081]) {
            s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
            s.store_mul(222, 405, 229);
            s.copy_ad(404, 3007);
        }

        let (assign89680_e137588,) = {
    if ((s.v[2621] != 0.0) && s.b[3081]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign89680_e137588);

        if ((s.v[2621] != 0.0) && s.b[3081]) {
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(3000)), s.ad_value(155)), 2.0);
        }

        s.b[3082] = (s.v[411] > 0.0);
        s.store_scalar(3082, if s.b[3082] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && s.b[3081]) && s.b[3082]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if (((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3082])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[3083] = (s.v[336] < 0.0);
        s.store_scalar(3083, if s.b[3083] { 1.0 } else { 0.0 });

        if ((((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3082])) && s.b[3083]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3082])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if ((s.v[2621] != 0.0) && s.b[3081]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[3084] = (s.v[336] < 0.0);
        s.store_scalar(3084, if s.b[3084] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && s.b[3081]) && s.b[3084]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((s.v[2621] != 0.0) && s.b[3081]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 3000, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(419, 335, 418);
        }

        let (assign89910_e137837,) = {
    if ((s.v[2621] != 0.0) && s.b[3081]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, assign89910_e137837);

    }

    pub(super) fn stamp_transient_block_93(
        s: &mut Scratch,
        p: &Parameters,
        var_coxb0: f64,
        var_uc_nover: f64,
        var_weffcv_nf: f64,
    ) {
        let mut assign89920_loop_guard: usize = 0;
        while {
            let assign89920_cond_e137844: f64 = (s.v[421] + 1.0);
            let assign89920_cond_e137846: f64 = if (((s.v[2621] != 0.0) && s.b[3081]) && (s.v[97] <= assign89920_cond_e137844)) { 1.0 } else { 0.0 };
            assign89920_cond_e137846 != 0.0
        } {
            assign89920_loop_guard += 1;
            assert!(assign89920_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2621] != 0.0) && s.b[3081]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[3086] = (s.v[333] < 60.0);
            s.store_scalar(3086, if s.b[3086] { 1.0 } else { 0.0 });
            if (((s.v[2621] != 0.0) && s.b[3081]) && s.b[3086]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ln_offset_lhs(416, 336, 1.0, 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if (((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3086])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if ((s.v[2621] != 0.0) && s.b[3081]) {
                s.store_mul(415, 154, 416);
            }
            s.b[3087] = (((s.v[116]) as f64).abs() < 1e-6);
            s.store_scalar(3087, if s.b[3087] { 1.0 } else { 0.0 });
            if (((s.v[2621] != 0.0) && s.b[3081]) && s.b[3087]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(3008, 334, 336);
                s.store_mul_add_scaled_product_rhs(3009, 154, s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0));
            }
            if (((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3087])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(3008, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
                s.store_mul_sub_ad_rhs(3009, 154, A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335)));
            }
            s.b[3088] = (((s.v[116]) as f64).abs() < 5e-5);
            s.store_scalar(3088, if s.b[3088] { 1.0 } else { 0.0 });
            if (((s.v[2621] != 0.0) && s.b[3081]) && s.b[3088]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            s.b[3089] = (((s.v[116]) as f64).abs() < 60.0);
            s.store_scalar(3089, if s.b[3089] { 1.0 } else { 0.0 });
            if ((((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3088])) && s.b[3089]) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if ((((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3088])) && (!s.b[3089])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[3090] = (s.v[214] > 0.0);
            s.store_scalar(3090, if s.b[3090] { 1.0 } else { 0.0 });
            if (((s.v[2621] != 0.0) && s.b[3081]) && s.b[3090]) {
                s.store_sqrt_add(216, 3008, 214);
                s.store_div_scaled_inputs2_indices(217, 3009, 0.5, 215, 0.5, 216, 1.0);
            }
            s.b[3091] = (s.v[3008] > 0.0);
            s.store_scalar(3091, if s.b[3091] { 1.0 } else { 0.0 });
            if ((((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3090])) && s.b[3091]) {
                s.store_sqrt(216, 3008);
                s.store_div_scaled_inputs_indices(217, 3009, 0.5, 216, 1.0);
            }
            if ((((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3090])) && (!s.b[3091])) {
                s.store_scalar(216, 0.0);
                s.store_scalar(217, 0.0);
            }
            if ((s.v[2621] != 0.0) && s.b[3081]) {
                s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2621] != 0.0) && s.b[3081]) {
                s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2621] != 0.0) && s.b[3081]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[3092] = (s.v[79] > 0.0);
            s.store_scalar(3092, if s.b[3092] { 1.0 } else { 0.0 });
            let (assign89920_body56_e138586,) = {
    if (((s.v[2621] != 0.0) && s.b[3081]) && s.b[3092]) {
        let assign89920_body56_e138584: f64 = (s.v[421] + 1.0);
        (assign89920_body56_e138584,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign89920_body56_e138586);
            if (((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3092])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if (((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3092])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[3093] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.store_scalar(3093, if s.b[3093] { 1.0 } else { 0.0 });
            if ((((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3092])) && s.b[3093]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3092])) {
                s.store_add(404, 404, 236);
            }
            s.b[3094] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.store_scalar(3094, if s.b[3094] { 1.0 } else { 0.0 });
            let (assign89920_body63_e138676,) = {
    if ((((s.v[2621] != 0.0) && s.b[3081]) && (!s.b[3092])) && s.b[3094]) {
        let assign89920_body63_e138674: f64 = (s.v[79] + 2.0);
        (assign89920_body63_e138674,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, assign89920_body63_e138676);
            let (assign89920_body64_e138684,) = {
    if ((s.v[2621] != 0.0) && s.b[3081]) {
        let assign89920_body64_e138682: f64 = (s.v[97] + 1.0);
        (assign89920_body64_e138682,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign89920_body64_e138684);
        }

        if ((s.v[2621] != 0.0) && s.b[3081]) {
            if (s.v[3008] >= 0.0) {
                s.store_scaled_sqrt(223, 3008, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            } else {
                s.store_scalar(223, 0.0);
            }
        }

        if ((s.v[2621] != 0.0) && s.b[3081]) {
            s.store_mul(2998, 982, 223);
            s.store_mul(2999, 3000, 2998);
            s.store_offset_div(100, 2999, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        if (s.v[2621] != 0.0) {
            s.store_sub(399, 398, 354);
        }

        s.b[3096] = (s.v[407] < 0.0);
        s.store_scalar(3096, if s.b[3096] { 1.0 } else { 0.0 });

        if ((s.v[2621] != 0.0) && s.b[3096]) {
            s.store_neg(407, 407);
        }

        s.b[3097] = (p.p55 == 0.0);
        s.store_scalar(3097, if s.b[3097] { 1.0 } else { 0.0 });

        s.b[3098] = (p.p50 == 0.0);
        s.store_scalar(3098, if s.b[3098] { 1.0 } else { 0.0 });

        if ((((s.v[2621] != 0.0) && s.b[3096]) && s.b[3097]) && s.b[3098]) {
            s.store_neg(3001, 404);
        }

        if ((((s.v[2621] != 0.0) && s.b[3096]) && s.b[3097]) && (!s.b[3098])) {
            s.copy_ad(3001, 396);
        }

        if (((s.v[2621] != 0.0) && s.b[3096]) && s.b[3097]) {
            s.store_sqrt_offset_square_offset(782, 3001, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(3001), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(3001), p.p137), 782, 0.5);
        }

        s.b[3099] = (s.v[336] < 0.0);
        s.store_scalar(3099, if s.b[3099] { 1.0 } else { 0.0 });

        if ((((s.v[2621] != 0.0) && s.b[3096]) && s.b[3097]) && s.b[3099]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && s.b[3096]) && s.b[3097]) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_add_scaled_inputs3_indices(781, 407, 1.0, 600, (-1.0), 407, (-0.1));
            s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));
        }

        if (((s.v[2621] != 0.0) && s.b[3096]) && s.b[3097]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.v[2621] != 0.0) && s.b[3096]) && s.b[3097]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(603, 407, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_sub(407, 407, 603);
        }

        s.b[3100] = (4.0 == 1.0);
        s.store_scalar(3100, if s.b[3100] { 1.0 } else { 0.0 });

        s.b[3101] = (4.0 == 2.0);
        s.store_scalar(3101, if s.b[3101] { 1.0 } else { 0.0 });

        s.b[3102] = (4.0 == 3.0);
        s.store_scalar(3102, if s.b[3102] { 1.0 } else { 0.0 });

        s.b[3103] = (4.0 == 4.0);
        s.store_scalar(3103, if s.b[3103] { 1.0 } else { 0.0 });

        s.b[3104] = (p.p55 == 1.0);
        s.store_scalar(3104, if s.b[3104] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && s.b[3100]) && s.b[3104]) {
            s.store_scale(338, 407, var_weffcv_nf);
        }

        if (((s.v[2621] != 0.0) && s.b[3100]) && (!s.b[3104])) {
            s.store_scale(338, 407, (var_weffcv_nf * (1.0 - s.v[526])));
        }

        if ((s.v[2621] != 0.0) && s.b[3100]) {
            s.store_mul(353, 338, 398);
            s.store_mul(356, 338, 354);
        }

        if ((s.v[2621] != 0.0) && (s.b[3101] && (!s.b[3100]))) {
            s.store_scale(338, 407, (var_weffcv_nf * s.v[526]));
            s.store_mul(351, 338, 398);
            s.store_mul(359, 338, 354);
        }

        s.b[3105] = (p.p55 == 1.0);
        s.store_scalar(3105, if s.b[3105] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && (s.b[3102] && (!(s.b[3100] || s.b[3101])))) && s.b[3105]) {
            s.store_scale(338, 407, var_weffcv_nf);
        }

        if (((s.v[2621] != 0.0) && (s.b[3102] && (!(s.b[3100] || s.b[3101])))) && (!s.b[3105])) {
            s.store_scale(338, 407, (var_weffcv_nf * (1.0 - s.v[526])));
        }

        if ((s.v[2621] != 0.0) && (s.b[3102] && (!(s.b[3100] || s.b[3101])))) {
            s.copy_ad(697, 404);
        }

        s.b[3106] = (p.p430 == 0.0);
        s.store_scalar(3106, if s.b[3106] { 1.0 } else { 0.0 });

        if (((s.v[2621] != 0.0) && (s.b[3102] && (!(s.b[3100] || s.b[3101])))) && s.b[3106]) {
            s.copy_ad(698, 354);
        }

        if ((s.v[2621] != 0.0) && (s.b[3102] && (!(s.b[3100] || s.b[3101])))) {
            s.store_mul(352, 338, 398);
            s.store_mul(355, 338, 354);
            s.copy_ad(816, 355);
        }

        if ((s.v[2621] != 0.0) && (s.b[3103] && (!((s.b[3100] || s.b[3101]) || s.b[3102])))) {
            s.store_scale(338, 407, (var_weffcv_nf * s.v[526]));
            s.store_mul(350, 338, 398);
            s.store_mul(358, 338, 354);
        }

        s.b[3107] = (p.p430 > 0.0);
        s.store_scalar(3107, if s.b[3107] { 1.0 } else { 0.0 });

        let (assign90500_e139254,) = {
    if s.b[3107] {
        (1.0,)
    } else {
        (s.v[406],)
    }
};
        s.store_scalar(406, assign90500_e139254);

        s.b[3108] = (((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0));
        s.store_scalar(3108, if s.b[3108] { 1.0 } else { 0.0 });

        if (s.b[3107] && s.b[3108]) {
            s.store_sub(395, 731, 728);
            s.store_sub(396, 729, 728);
            s.store_scalar(409, var_uc_nover);
            s.store_scalar(407, 0.0);
            s.copy_ad(411, 384);
            s.copy_ad(410, 686);
            s.store_scalar(413, var_coxb0);
            s.store_scalar(3116, 0.4);
        }

        let (assign90610_e139329,) = {
    if (s.b[3107] && s.b[3108]) {
        (0.0,)
    } else {
        (s.v[3117],)
    }
};
        s.store_scalar(3117, assign90610_e139329);

        if (s.b[3107] && s.b[3108]) {
            s.store_scalar(223, 0.0);
            s.store_scalar(214, 0.0);
            s.store_scalar(216, 0.0);
            s.store_scalar(232, 0.0);
            s.store_scalar(236, 0.0);
            s.store_scalar(233, 0.0);
            s.store_scalar(217, 0.0);
            s.store_scalar(420, 0.0);
            s.store_scalar(215, 0.0);
            s.store_scalar(447, 0.0);
            s.store_scalar(445, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_94(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[3107] && s.b[3108]) {
            s.store_scalar(446, 0.0);
        }

        let (assign90740_e139408,) = {
    if (s.b[3107] && s.b[3108]) {
        let assign90740_e139406: f64 = (-1.0);
        (assign90740_e139406,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign90740_e139408);

        if (s.b[3107] && s.b[3108]) {
            s.store_scalar(3118, 0.0);
            s.store_scalar(3119, 0.0);
            s.store_mul_scaled_ln_ad_rhs(3114, 155, 2.0, A::div(s.ad_value(409), s.ad_value(394)));
            s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(3114), (-0.1));
            s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        }

        if (s.b[3107] && s.b[3108]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.b[3107] && s.b[3108]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(3115, 781, (-0.5), 782, (-0.5), 0.8);
        }

        s.b[3121] = (s.v[3116] > (s.v[3115] * 0.5));
        s.store_scalar(3121, if s.b[3121] { 1.0 } else { 0.0 });

        if ((s.b[3107] && s.b[3108]) && s.b[3121]) {
            s.store_scale(3116, 3115, 0.5);
        }

        s.b[3122] = param_given[338];
        s.store_scalar(3122, if s.b[3122] { 1.0 } else { 0.0 });

        if ((s.b[3107] && s.b[3108]) && s.b[3122]) {
            s.store_scalar(3115, p.p338);
        }

        s.b[3123] = param_given[339];
        s.store_scalar(3123, if s.b[3123] { 1.0 } else { 0.0 });

        if ((s.b[3107] && s.b[3108]) && s.b[3123]) {
            s.store_scalar(3116, p.p339);
        }

        s.b[3124] = param_given[338];
        s.store_scalar(3124, if s.b[3124] { 1.0 } else { 0.0 });

        if (((s.b[3107] && s.b[3108]) && (!s.b[3123])) && s.b[3124]) {
            s.store_scale(3116, 3115, 0.5);
        }

        s.b[3125] = (s.v[3116] > (s.v[3115] * 0.5));
        s.store_scalar(3125, if s.b[3125] { 1.0 } else { 0.0 });

        if ((s.b[3107] && s.b[3108]) && s.b[3125]) {
            s.store_scale(3116, 3115, 0.5);
        }

        s.b[3126] = (p.p38 == 1.0);
        s.store_scalar(3126, if s.b[3126] { 1.0 } else { 0.0 });

        if ((s.b[3107] && s.b[3108]) && s.b[3126]) {
            s.store_neg(334, 396);
        }

        s.b[3127] = (s.v[334] > s.v[3116]);
        s.store_scalar(3127, if s.b[3127] { 1.0 } else { 0.0 });

        if (((s.b[3107] && s.b[3108]) && s.b[3126]) && s.b[3127]) {
            s.store_sub(335, 334, 3116);
            s.store_sub(336, 3115, 3116);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 345, 1.0);
            s.store_neg(345, 345);
            s.store_add(344, 3116, 333);
        }

        if (((s.b[3107] && s.b[3108]) && s.b[3126]) && (!s.b[3127])) {
            s.copy_ad(344, 334);
        }

        if ((s.b[3107] && s.b[3108]) && s.b[3126]) {
            s.store_neg(397, 344);
        }

        if ((s.b[3107] && s.b[3108]) && (!s.b[3126])) {
            s.copy_ad(397, 396);
        }

        if (s.b[3107] && s.b[3108]) {
            s.store_div(212, 410, 413);
            s.store_square(213, 212);
            s.store_sub_from_scalar(402, s.v[458], 395);
        }

        let (assign91150_e139817,) = {
    if (s.b[3107] && s.b[3108]) {
        let assign91150_e139811: f64 = (-s.v[397]);
        let assign91150_e139814: f64 = (10.0 * 2.220446049250313e-16);
        let assign91150_e139815: f64 = (assign91150_e139811 + assign91150_e139814);
        (assign91150_e139815,)
    } else {
        (s.v[403],)
    }
};
        s.store_scalar(403, assign91150_e139817);

        if (s.b[3107] && s.b[3108]) {
            s.store_scalar(3110, 0.0);
            s.store_scale(3111, 409, 1.6021918e-19);
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
        }

        s.b[3128] = ((s.v[154] * (-s.v[397])) >= 500.0);
        s.store_scalar(3128, if s.b[3128] { 1.0 } else { 0.0 });

        if ((s.b[3107] && s.b[3108]) && s.b[3128]) {
            s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(334, 1.403592217853e217);
        }

        if ((s.b[3107] && s.b[3108]) && (!s.b[3128])) {
            s.store_mul_neg_rhs(781, 154, 397);
            s.store_scalar(229, 1.0);
        }

        let mut assign91250_loop_guard: usize = 0;
        while {
            let assign91250_cond_e139909: f64 = if (((s.b[3107] && s.b[3108]) && (!s.b[3128])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign91250_cond_e139909 != 0.0
        } {
            assign91250_loop_guard += 1;
            assert!(assign91250_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[3107] && s.b[3108]) && (!s.b[3128])) {
                s.store_scale(229, 229, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if ((s.b[3107] && s.b[3108]) && (!s.b[3128])) {
            s.store_mul_exp_rhs(229, 229, 781);
            s.copy_ad(334, 229);
        }

        if ((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) {
            s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));
            s.store_scalar(782, (4.0 * 0.5));
        }

        if ((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(335, 781, 0.5, 782, 0.5, 0.5);
        }

        s.b[3129] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));
        s.store_scalar(3129, if s.b[3129] { 1.0 } else { 0.0 });

        if (((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) {
            s.store_add_scaled_inputs3_indices(781, 402, 1.0, 397, 1.0, 335, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 335);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign91400_e140115,) = {
    if (((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign91400_e140115);

        let (assign91410_e140125,) = {
    if (((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign91410_e140125);

        if (((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3130] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.store_scalar(3130, if s.b[3130] { 1.0 } else { 0.0 });

        s.b[3131] = (1.0 == 1.0);
        s.store_scalar(3131, if s.b[3131] { 1.0 } else { 0.0 });

        let (assign91500_e140223,) = {
    if (((((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) && s.b[3130]) && s.b[3131]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign91500_e140223);

        s.b[3132] = (1.0 == 2.0);
        s.store_scalar(3132, if s.b[3132] { 1.0 } else { 0.0 });

        let (assign91520_e140243,) = {
    if ((((((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) && s.b[3130]) && (!s.b[3131])) && s.b[3132]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign91520_e140243);

        s.b[3133] = (1.0 == 4.0);
        s.store_scalar(3133, if s.b[3133] { 1.0 } else { 0.0 });

        let (assign91540_e140266,) = {
    if (((((((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) && s.b[3130]) && (!s.b[3131])) && (!s.b[3132])) && s.b[3133]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign91540_e140266);

        s.b[3134] = (1.0 == 8.0);
        s.store_scalar(3134, if s.b[3134] { 1.0 } else { 0.0 });

        let (assign91560_e140292,) = {
    if ((((((((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) && s.b[3130]) && (!s.b[3131])) && (!s.b[3132])) && (!s.b[3133])) && s.b[3134]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign91560_e140292);

        let (assign91570_e140304,) = {
    if ((((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) && s.b[3130]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign91570_e140304);

        let mut assign91580_loop_guard: usize = 0;
        while {
            let assign91580_cond_e140317: f64 = if (((((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) && s.b[3130]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign91580_cond_e140317 != 0.0
        } {
            assign91580_loop_guard += 1;
            assert!(assign91580_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) && s.b[3130]) {
                s.store_sqrt(726, 726);
            }
            let (assign91580_body1_e140344,) = {
    if ((((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) && s.b[3130]) {
        let assign91580_body1_e140342: f64 = (s.v[719] + 1.0);
        (assign91580_body1_e140342,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign91580_body1_e140344);
        }

        if ((((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) && (!s.b[3130])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);
        }

        if (((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && s.b[3129]) {
        }

        if (((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) && (!s.b[3129])) {
            s.store_add(335, 402, 397);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) {
            s.store_sub(397, 335, 402);
        }

        let (assign91680_e140481,) = {
    if ((s.b[3107] && s.b[3108]) && (s.v[406] != 0.0)) {
        let assign91680_e140475: f64 = (-s.v[397]);
        let assign91680_e140478: f64 = (10.0 * 2.220446049250313e-16);
        let assign91680_e140479: f64 = (assign91680_e140475 + assign91680_e140478);
        (assign91680_e140479,)
    } else {
        (s.v[403],)
    }
};
        s.store_scalar(403, assign91680_e140481);

        s.b[3135] = (s.v[402] < s.v[403]);
        s.store_scalar(3135, if s.b[3135] { 1.0 } else { 0.0 });

        if ((s.b[3107] && s.b[3108]) && s.b[3135]) {
            s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));
            s.store_mul_add_rhs(332, 154, 402, 397);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(410));
            s.store_mul(333, 335, 413);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(278, (7.0 * 1.414213562373095), 333, A::offset(s.ad_value(332), (-2.0)), 9.0);
            s.store_square(276, 278);
        }

        s.b[3136] = (s.v[277] < (s.v[276] * 1e-8));
        s.store_scalar(3136, if s.b[3136] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_transient_block_95(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[3107] && s.b[3108]) && s.b[3135]) && s.b[3136]) {
            s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);
        }

        if (((s.b[3107] && s.b[3108]) && s.b[3135]) && (!s.b[3136])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((s.b[3107] && s.b[3108]) && s.b[3135]) {
            s.store_powf(273, 274, 0.3333333333333333);
            s.store_add_scaled_inputs_product_first_ad(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);
            s.store_div(116, 272, 273);
            s.store_mul(335, 116, 155);
            s.store_div(336, 335, 271);
            s.store_sqrt_square_offset(337, 336, 1.0);
            s.store_sub_div_lhs_indices(404, 335, 337, 397);
            s.store_sub(336, 402, 404);
            s.store_mul(398, 413, 336);
            s.copy_ad(354, 398);
            s.copy_ad(3118, 404);
        }

        s.b[3137] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));
        s.store_scalar(3137, if s.b[3137] { 1.0 } else { 0.0 });

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3137]) {
            s.store_add_scaled_product_indices(89, 402, 1.0, 213, 154, 0.5);
        }

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && (!s.b[3137])) {
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
        }

        if ((s.b[3107] && s.b[3108]) && (!s.b[3135])) {
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        s.b[3138] = (s.v[116] >= 3.0);
        s.store_scalar(3138, if s.b[3138] { 1.0 } else { 0.0 });

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3138]) {
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
            s.store_mul_add_rhs(116, 154, 89, 397);
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && (!s.b[3138])) {
            s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));
            s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));
            s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));
            s.store_div_scaled_inputs2_indices(437, 402, -1.0, 397, -1.0, 212, 1.0);
            s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(441, A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(435), s.ad_value(436), 1.0, s.ad_value(434), s.ad_value(434), 6.0), (-1.0), 437, 1.0, 434, 2.0, 1.0);
            s.store_div_ad(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), A::mul_scaled_lhs(s.ad_value(434), 9.0, s.ad_value(434)));
            s.store_sqrt_add_scaled_square_cube_product(339, 441, 1.0, 440, 1.0);
            s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);
            s.store_neg_powf_add_input(438, 441, 339, 0.3333333333333333);
            s.store_add_scaled_inputs3_div_scaled_third_indices(116, 439, 1.0, 438, 1.0, 435, 1.0, 434, 3.0, -1.0);
            s.store_add_scaled_product_indices(89, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[3139] = (p.p33 > 0.0);
        s.store_scalar(3139, if s.b[3139] { 1.0 } else { 0.0 });

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) {
            s.store_offset_add(442, 402, 397, 0.1);
            s.store_mul(222, 405, 229);
            s.store_mul(443, 405, 229);
            s.store_mul(334, 156, 213);
            s.store_mul(444, 154, 442);
            s.store_add_scaled_inputs_product_mixed_aaii(447, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), 154, 397, 1.0);
        }

        s.b[3140] = (p.p33 == 2.0);
        s.store_scalar(3140, if s.b[3140] { 1.0 } else { 0.0 });

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3140]) {
            s.store_offset_sub(781, 444, 447, (-1.0));
            s.store_scale(782, 444, 4.0);
        }

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3140]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3140]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(447, 444, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && (!s.b[3140])) {
            if (s.v[447] <= s.v[444]) {
            } else {
                s.copy_ad(447, 444);
            }
        }

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) {
            if (s.v[447] >= 0.0) {
            } else {
                s.store_scalar(447, 0.0);
            }
        }

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) {
            s.store_sub(444, 444, 447);
            s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);
            s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));
            s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);
        }

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) {
            s.copy_ad(445, 116);
        }

        s.b[3141] = (p.p33 == 2.0);
        s.store_scalar(3141, if s.b[3141] { 1.0 } else { 0.0 });

        s.b[3142] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));
        s.store_scalar(3142, if s.b[3142] { 1.0 } else { 0.0 });

        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) {
            s.store_add_scaled_inputs3_indices(781, 445, 1.0, 446, (-1.0), 446, 0.2);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 446, 446, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign92510_e141777,) = {
    if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign92510_e141777);

        let (assign92520_e141792,) = {
    if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign92520_e141792);

        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3143] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(3143, if s.b[3143] { 1.0 } else { 0.0 });

        s.b[3144] = (2.0 == 1.0);
        s.store_scalar(3144, if s.b[3144] { 1.0 } else { 0.0 });

        let (assign92630_e141959,) = {
    if (((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) && s.b[3143]) && s.b[3144]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign92630_e141959);

        s.b[3145] = (2.0 == 2.0);
        s.store_scalar(3145, if s.b[3145] { 1.0 } else { 0.0 });

        let (assign92650_e141984,) = {
    if ((((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) && s.b[3143]) && (!s.b[3144])) && s.b[3145]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign92650_e141984);

        s.b[3146] = (2.0 == 4.0);
        s.store_scalar(3146, if s.b[3146] { 1.0 } else { 0.0 });

        let (assign92670_e142012,) = {
    if (((((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) && s.b[3143]) && (!s.b[3144])) && (!s.b[3145])) && s.b[3146]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign92670_e142012);

        s.b[3147] = (2.0 == 8.0);
        s.store_scalar(3147, if s.b[3147] { 1.0 } else { 0.0 });

        let (assign92690_e142043,) = {
    if ((((((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) && s.b[3143]) && (!s.b[3144])) && (!s.b[3145])) && (!s.b[3146])) && s.b[3147]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign92690_e142043);

        let (assign92700_e142060,) = {
    if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) && s.b[3143]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign92700_e142060);

        let mut assign92710_loop_guard: usize = 0;
        while {
            let assign92710_cond_e142078: f64 = if (((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) && s.b[3143]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign92710_cond_e142078 != 0.0
        } {
            assign92710_loop_guard += 1;
            assert!(assign92710_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) && s.b[3143]) {
                s.store_sqrt(726, 726);
            }
            let (assign92710_body1_e142115,) = {
    if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) && s.b[3143]) {
        let assign92710_body1_e142113: f64 = (s.v[719] + 1.0);
        (assign92710_body1_e142113,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign92710_body1_e142115);
        }

        if ((((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) && (!s.b[3143])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);
            s.store_div_scaled_product3_indices(335, 446, 725, 726, 0.2, 770, 1.0);
            s.store_add_scaled_inputs3_indices(116, 446, 1.0, 446, (-0.2), 780, 1.0);
        }

        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && s.b[3142]) {
        }

        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && s.b[3141]) && (!s.b[3142])) {
            s.copy_ad(116, 445);
            s.store_scalar(335, 1.0);
        }

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3139]) && (!s.b[3141])) {
            if (s.v[445] <= s.v[446]) {
                s.copy_ad(116, 445);
            } else {
                s.copy_ad(116, 446);
            }
        }

        s.b[3148] = (p.p33 == 1.0);
        s.store_scalar(3148, if s.b[3148] { 1.0 } else { 0.0 });

        if (((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[3149] = (s.v[411] > 0.0);
        s.store_scalar(3149, if s.b[3149] { 1.0 } else { 0.0 });

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) && s.b[3149]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) && (!s.b[3149])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[3150] = (s.v[336] < 0.0);
        s.store_scalar(3150, if s.b[3150] { 1.0 } else { 0.0 });

        if (((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) && (!s.b[3149])) && s.b[3150]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.b[3107] && s.b[3108]) && (!s.b[3135])) && s.b[3148]) && (!s.b[3149])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
        }

    }
}

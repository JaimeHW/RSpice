#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_80(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign79090_loop_guard: usize = 0;
        while {
            let assign79090_cond_e119587: f64 = (s.v[421] + 1.0);
            let assign79090_cond_e119589: f64 = if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (s.v[97] <= assign79090_cond_e119587)) { 1.0 } else { 0.0 };
            assign79090_cond_e119589 != 0.0
        } {
            assign79090_loop_guard += 1;
            assert!(assign79090_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2830] = (s.v[333] < 60.0);
            s.store_scalar(2830, if s.b[2830] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && s.b[2830]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ln_offset_lhs(416, 336, 1.0, 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2830])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2831] = (s.v[116] < 0.0);
            s.store_scalar(2831, if s.b[2831] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && s.b[2831]) {
                s.store_scalar(334, (-0.7071067811865475));
                s.store_mul(223, 116, 334);
                s.store_mul(420, 154, 334);
            }
            s.b[2832] = (s.v[116] < 1e-6);
            s.store_scalar(2832, if s.b[2832] { 1.0 } else { 0.0 });
            if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2831])) && s.b[2832]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(338, 334, 336);
            }
            s.b[2833] = (s.v[338] > 0.0);
            s.store_scalar(2833, if s.b[2833] { 1.0 } else { 0.0 });
            if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2831])) && s.b[2832]) && s.b[2833]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);
            }
            if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2831])) && s.b[2832]) && (!s.b[2833])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2831])) && (!s.b[2832])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(338, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
            }
            s.b[2834] = (s.v[338] > 0.0);
            s.store_scalar(2834, if s.b[2834] { 1.0 } else { 0.0 });
            if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2831])) && (!s.b[2832])) && s.b[2834]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335))), 0.5, 223, 1.0);
            }
            if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2831])) && (!s.b[2832])) && (!s.b[2834])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            s.b[2835] = (s.v[116] < 0.0);
            s.store_scalar(2835, if s.b[2835] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && s.b[2835]) {
                s.store_scalar(214, 0.0);
                s.store_scalar(215, 0.0);
                s.store_neg(216, 223);
                s.store_neg(217, 420);
            }
            s.b[2836] = (s.v[116] < 60.0);
            s.store_scalar(2836, if s.b[2836] { 1.0 } else { 0.0 });
            s.b[2837] = (s.v[116] < 5e-5);
            s.store_scalar(2837, if s.b[2837] { 1.0 } else { 0.0 });
            if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2835])) && s.b[2836]) && s.b[2837]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2835])) && s.b[2836]) && (!s.b[2837])) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2835])) && (!s.b[2836])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2838] = (s.v[214] > 0.0);
            s.store_scalar(2838, if s.b[2838] { 1.0 } else { 0.0 });
            if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2835])) && s.b[2838]) {
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_scaled_add_product(217, s.ad_value(215), 0.5, s.ad_value(420), s.ad_value(223), (2.0 * 0.5), s.ad_value(216), 1.0);
            }
            if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2835])) && (!s.b[2838])) {
                s.copy_ad(216, 223);
                s.copy_ad(217, 420);
            }
            if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2839] = (s.v[79] == 1.0);
            s.store_scalar(2839, if s.b[2839] { 1.0 } else { 0.0 });
            let (assign79090_body72_e120735,) = {
    if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && s.b[2839]) {
        let assign79090_body72_e120733: f64 = (s.v[421] + 1.0);
        (assign79090_body72_e120733,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign79090_body72_e120735);
            if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2839])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2839])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2840] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.store_scalar(2840, if s.b[2840] { 1.0 } else { 0.0 });
            if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2839])) && s.b[2840]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2839])) {
                s.store_add(404, 404, 236);
            }
            s.b[2841] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.store_scalar(2841, if s.b[2841] { 1.0 } else { 0.0 });
            let (assign79090_body79_e120838,) = {
    if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2839])) && s.b[2841]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, assign79090_body79_e120838);
            let (assign79090_body80_e120849,) = {
    if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) {
        let assign79090_body80_e120847: f64 = (s.v[97] + 1.0);
        (assign79090_body80_e120847,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign79090_body80_e120849);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) {
            s.store_mul(2760, 982, 223);
            s.store_mul(2761, 2762, 2760);
            s.store_offset_div(100, 2761, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        s.b[2843] = (p.p33 == 4.0);
        s.store_scalar(2843, if s.b[2843] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[2843]) {
            s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
            s.store_mul(222, 405, 229);
            s.copy_ad(404, 2769);
        }

        let (assign79240_e120986,) = {
    if ((s.v[2625] != 0.0) && s.b[2843]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign79240_e120986);

        if ((s.v[2625] != 0.0) && s.b[2843]) {
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2762)), s.ad_value(155)), 2.0);
        }

        s.b[2844] = (s.v[411] > 0.0);
        s.store_scalar(2844, if s.b[2844] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && s.b[2843]) && s.b[2844]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if (((s.v[2625] != 0.0) && s.b[2843]) && (!s.b[2844])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2845] = (s.v[336] < 0.0);
        s.store_scalar(2845, if s.b[2845] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && s.b[2843]) && (!s.b[2844])) && s.b[2845]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2625] != 0.0) && s.b[2843]) && (!s.b[2844])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if ((s.v[2625] != 0.0) && s.b[2843]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2846] = (s.v[336] < 0.0);
        s.store_scalar(2846, if s.b[2846] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && s.b[2843]) && s.b[2846]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((s.v[2625] != 0.0) && s.b[2843]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2762, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(419, 335, 418);
        }

        let (assign79470_e121235,) = {
    if ((s.v[2625] != 0.0) && s.b[2843]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, assign79470_e121235);

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
        var_vgsei_db12: f64,
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
        var_vgsei_dn18: f64,
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
        var_guard1873_slot: &mut f64,
        var_guard1874_slot: &mut f64,
        var_guard1875_slot: &mut f64,
        var_guard1877_slot: &mut f64,
        var_guard1879_slot: &mut f64,
    ) {
        let mut var_flg_coovlps: f64 = *var_flg_coovlps_slot;
        let mut var_guard1873: f64 = *var_guard1873_slot;
        let mut var_guard1874: f64 = *var_guard1874_slot;
        let mut var_guard1875: f64 = *var_guard1875_slot;
        let mut var_guard1877: f64 = *var_guard1877_slot;
        let mut var_guard1879: f64 = *var_guard1879_slot;

        let mut assign79480_loop_guard: usize = 0;
        while {
            let assign79480_cond_e121242: f64 = (s.v[421] + 1.0);
            let assign79480_cond_e121244: f64 = if (((s.v[2625] != 0.0) && s.b[2843]) && (s.v[97] <= assign79480_cond_e121242)) { 1.0 } else { 0.0 };
            assign79480_cond_e121244 != 0.0
        } {
            assign79480_loop_guard += 1;
            assert!(assign79480_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2625] != 0.0) && s.b[2843]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2848] = (s.v[333] < 60.0);
            s.store_scalar(2848, if s.b[2848] { 1.0 } else { 0.0 });
            if (((s.v[2625] != 0.0) && s.b[2843]) && s.b[2848]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ln_offset_lhs(416, 336, 1.0, 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if (((s.v[2625] != 0.0) && s.b[2843]) && (!s.b[2848])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if ((s.v[2625] != 0.0) && s.b[2843]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2849] = (((s.v[116]) as f64).abs() < 1e-6);
            s.store_scalar(2849, if s.b[2849] { 1.0 } else { 0.0 });
            if (((s.v[2625] != 0.0) && s.b[2843]) && s.b[2849]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(2770, 334, 336);
                s.store_mul_add_scaled_product_rhs(2771, 154, s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0));
            }
            if (((s.v[2625] != 0.0) && s.b[2843]) && (!s.b[2849])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(2770, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
                s.store_mul_sub_ad_rhs(2771, 154, A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335)));
            }
            s.b[2850] = (((s.v[116]) as f64).abs() < 5e-5);
            s.store_scalar(2850, if s.b[2850] { 1.0 } else { 0.0 });
            if (((s.v[2625] != 0.0) && s.b[2843]) && s.b[2850]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            s.b[2851] = (((s.v[116]) as f64).abs() < 60.0);
            s.store_scalar(2851, if s.b[2851] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && s.b[2843]) && (!s.b[2850])) && s.b[2851]) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if ((((s.v[2625] != 0.0) && s.b[2843]) && (!s.b[2850])) && (!s.b[2851])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2852] = (s.v[214] > 0.0);
            s.store_scalar(2852, if s.b[2852] { 1.0 } else { 0.0 });
            if (((s.v[2625] != 0.0) && s.b[2843]) && s.b[2852]) {
                s.store_sqrt_add(216, 2770, 214);
                s.store_div_scaled_inputs2_indices(217, 2771, 0.5, 215, 0.5, 216, 1.0);
            }
            s.b[2853] = (s.v[2770] > 0.0);
            s.store_scalar(2853, if s.b[2853] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && s.b[2843]) && (!s.b[2852])) && s.b[2853]) {
                s.store_sqrt(216, 2770);
                s.store_div_scaled_inputs_indices(217, 2771, 0.5, 216, 1.0);
            }
            if ((((s.v[2625] != 0.0) && s.b[2843]) && (!s.b[2852])) && (!s.b[2853])) {
                s.store_scalar(216, 0.0);
                s.store_scalar(217, 0.0);
            }
            if ((s.v[2625] != 0.0) && s.b[2843]) {
                s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2625] != 0.0) && s.b[2843]) {
                s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2625] != 0.0) && s.b[2843]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2854] = (s.v[79] > 0.0);
            s.store_scalar(2854, if s.b[2854] { 1.0 } else { 0.0 });
            let (assign79480_body56_e121984,) = {
    if (((s.v[2625] != 0.0) && s.b[2843]) && s.b[2854]) {
        let assign79480_body56_e121982: f64 = (s.v[421] + 1.0);
        (assign79480_body56_e121982,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign79480_body56_e121984);
            if (((s.v[2625] != 0.0) && s.b[2843]) && (!s.b[2854])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if (((s.v[2625] != 0.0) && s.b[2843]) && (!s.b[2854])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2855] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.store_scalar(2855, if s.b[2855] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && s.b[2843]) && (!s.b[2854])) && s.b[2855]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.v[2625] != 0.0) && s.b[2843]) && (!s.b[2854])) {
                s.store_add(404, 404, 236);
            }
            s.b[2856] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.store_scalar(2856, if s.b[2856] { 1.0 } else { 0.0 });
            let (assign79480_body63_e122074,) = {
    if ((((s.v[2625] != 0.0) && s.b[2843]) && (!s.b[2854])) && s.b[2856]) {
        let assign79480_body63_e122072: f64 = (s.v[79] + 2.0);
        (assign79480_body63_e122072,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, assign79480_body63_e122074);
            let (assign79480_body64_e122082,) = {
    if ((s.v[2625] != 0.0) && s.b[2843]) {
        let assign79480_body64_e122080: f64 = (s.v[97] + 1.0);
        (assign79480_body64_e122080,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign79480_body64_e122082);
        }

        if ((s.v[2625] != 0.0) && s.b[2843]) {
            if (s.v[2770] >= 0.0) {
                s.store_scaled_sqrt(223, 2770, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            } else {
                s.store_scalar(223, 0.0);
            }
        }

        if ((s.v[2625] != 0.0) && s.b[2843]) {
            s.store_mul(2760, 982, 223);
            s.store_mul(2761, 2762, 2760);
            s.store_offset_div(100, 2761, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        if (s.v[2625] != 0.0) {
            s.store_sub(399, 398, 354);
        }

        s.b[2858] = (s.v[407] < 0.0);
        s.store_scalar(2858, if s.b[2858] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[2858]) {
            s.store_neg(407, 407);
        }

        s.b[2859] = (p.p55 == 0.0);
        s.store_scalar(2859, if s.b[2859] { 1.0 } else { 0.0 });

        s.b[2860] = (p.p50 == 0.0);
        s.store_scalar(2860, if s.b[2860] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && s.b[2858]) && s.b[2859]) && s.b[2860]) {
            s.store_neg(2763, 404);
        }

        if ((((s.v[2625] != 0.0) && s.b[2858]) && s.b[2859]) && (!s.b[2860])) {
            s.copy_ad(2763, 396);
        }

        if (((s.v[2625] != 0.0) && s.b[2858]) && s.b[2859]) {
            s.store_sqrt_offset_square_offset(782, 2763, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(2763), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(2763), p.p137), 782, 0.5);
        }

        s.b[2861] = (s.v[336] < 0.0);
        s.store_scalar(2861, if s.b[2861] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && s.b[2858]) && s.b[2859]) && s.b[2861]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2625] != 0.0) && s.b[2858]) && s.b[2859]) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_add_scaled_inputs3_indices(781, 407, 1.0, 600, (-1.0), 407, (-0.1));
            s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));
        }

        if (((s.v[2625] != 0.0) && s.b[2858]) && s.b[2859]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.v[2625] != 0.0) && s.b[2858]) && s.b[2859]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(603, 407, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_sub(407, 407, 603);
        }

        s.b[2862] = (2.0 == 1.0);
        s.store_scalar(2862, if s.b[2862] { 1.0 } else { 0.0 });

        s.b[2863] = (2.0 == 2.0);
        s.store_scalar(2863, if s.b[2863] { 1.0 } else { 0.0 });

        s.b[2864] = (2.0 == 3.0);
        s.store_scalar(2864, if s.b[2864] { 1.0 } else { 0.0 });

        s.b[2865] = (2.0 == 4.0);
        s.store_scalar(2865, if s.b[2865] { 1.0 } else { 0.0 });

        s.b[2866] = (p.p55 == 1.0);
        s.store_scalar(2866, if s.b[2866] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && s.b[2862]) && s.b[2866]) {
            s.store_scale(338, 407, var_weffcv_nf);
        }

        if (((s.v[2625] != 0.0) && s.b[2862]) && (!s.b[2866])) {
            s.store_scale(338, 407, (var_weffcv_nf * (1.0 - s.v[526])));
        }

        if ((s.v[2625] != 0.0) && s.b[2862]) {
            s.store_mul(353, 338, 398);
            s.store_mul(356, 338, 354);
        }

        if ((s.v[2625] != 0.0) && (s.b[2863] && (!s.b[2862]))) {
            s.store_scale(338, 407, (var_weffcv_nf * s.v[526]));
            s.store_mul(351, 338, 398);
            s.store_mul(359, 338, 354);
        }

        s.b[2867] = (p.p55 == 1.0);
        s.store_scalar(2867, if s.b[2867] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (s.b[2864] && (!(s.b[2862] || s.b[2863])))) && s.b[2867]) {
            s.store_scale(338, 407, var_weffcv_nf);
        }

        if (((s.v[2625] != 0.0) && (s.b[2864] && (!(s.b[2862] || s.b[2863])))) && (!s.b[2867])) {
            s.store_scale(338, 407, (var_weffcv_nf * (1.0 - s.v[526])));
        }

        if ((s.v[2625] != 0.0) && (s.b[2864] && (!(s.b[2862] || s.b[2863])))) {
            s.copy_ad(697, 404);
        }

        s.b[2868] = (p.p430 == 0.0);
        s.store_scalar(2868, if s.b[2868] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (s.b[2864] && (!(s.b[2862] || s.b[2863])))) && s.b[2868]) {
            s.copy_ad(698, 354);
        }

        if ((s.v[2625] != 0.0) && (s.b[2864] && (!(s.b[2862] || s.b[2863])))) {
            s.store_mul(352, 338, 398);
            s.store_mul(355, 338, 354);
            s.copy_ad(816, 355);
        }

        if ((s.v[2625] != 0.0) && (s.b[2865] && (!((s.b[2862] || s.b[2863]) || s.b[2864])))) {
            s.store_scale(338, 407, (var_weffcv_nf * s.v[526]));
            s.store_mul(350, 338, 398);
            s.store_mul(358, 338, 354);
        }

        s.store_scalar(2625, 0.0);

        let assign80060_e122649: f64 = if 3.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard1873 = assign80060_e122649;

        let assign80070_e122652: f64 = if 3.0 == 2.0 { 1.0 } else { 0.0 };
        var_guard1874 = assign80070_e122652;

        let assign80080_e122655: f64 = if 3.0 == 3.0 { 1.0 } else { 0.0 };
        var_guard1875 = assign80080_e122655;

        s.b[2872] = (3.0 == 4.0);
        s.store_scalar(2872, if s.b[2872] { 1.0 } else { 0.0 });

        let assign80100_e122669: f64 = if (((p.p36 == 1.0) && (p.p66 > 0.0)) && (var_uc_novers > 0.0)) { 1.0 } else { 0.0 };
        var_guard1877 = assign80100_e122669;

        let (assign80110_e122675,) = {
    if ((var_guard1873 != 0.0) && (var_guard1877 != 0.0)) {
        (1.0,)
    } else {
        (s.v[2625],)
    }
};
        s.store_scalar(2625, assign80110_e122675);

        let (assign80120_e122681,) = {
    if ((var_guard1873 != 0.0) && (var_guard1877 != 0.0)) {
        (1.0,)
    } else {
        (var_flg_coovlps,)
    }
};
        var_flg_coovlps = assign80120_e122681;

        if ((s.v[2869] != 0.0) && (s.v[2873] != 0.0)) {
            s.store_sub(395, 731, 728);
            s.store_neg(396, 728);
            s.store_scalar(409, var_uc_novers);
            s.store_scalar(407, p.p66);
            s.store_scalar(411, 0.0);
            s.copy_ad(410, 687);
            s.store_scalar(413, var_cox0);
        }

        s.b[2874] = (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.store_scalar(2874, if s.b[2874] { 1.0 } else { 0.0 });

        let (assign80210_e122754,) = {
    if (((var_guard1874 != 0.0) && (var_guard1873 == 0.0)) && s.b[2874]) {
        (1.0,)
    } else {
        (s.v[2625],)
    }
};
        s.store_scalar(2625, assign80210_e122754);

        if (((s.v[2870] != 0.0) && (s.v[2869] == 0.0)) && s.b[2874]) {
            s.store_sub_ad_lhs(395, A::from_derivatives(var_vgsei, [var_vgsei_dn0, var_vgsei_dn1, var_vgsei_dn2, var_vgsei_dn3, var_vgsei_dn4, var_vgsei_dn5, var_vgsei_dn6, var_vgsei_dn7, var_vgsei_dn8, var_vgsei_dn9, var_vgsei_dn10, var_vgsei_dn11, var_vgsei_dn12, var_vgsei_dn13, var_vgsei_dn14, var_vgsei_dn15, var_vgsei_dn16, var_vgsei_dn17, var_vgsei_dn18], [var_vgsei_db0, var_vgsei_db1, var_vgsei_db2, var_vgsei_db3, var_vgsei_db4, var_vgsei_db5, var_vgsei_db6, var_vgsei_db7, var_vgsei_db8, var_vgsei_db9, var_vgsei_db10, var_vgsei_db11, var_vgsei_db12]), 735);
            s.store_neg(396, 735);
        }

        let assign80240_e122786: f64 = if (((p.p35 == 1.0) && (p.p63 > 0.0)) && (var_uc_nover > 0.0)) { 1.0 } else { 0.0 };
        var_guard1879 = assign80240_e122786;

        let (assign80250_e122797,) = {
    if (((var_guard1875 != 0.0) && (!((var_guard1873 != 0.0) || (var_guard1874 != 0.0)))) && (var_guard1879 != 0.0)) {
        (1.0,)
    } else {
        (s.v[2625],)
    }
};
        s.store_scalar(2625, assign80250_e122797);

        *var_flg_coovlps_slot = var_flg_coovlps;
        *var_guard1873_slot = var_guard1873;
        *var_guard1874_slot = var_guard1874;
        *var_guard1875_slot = var_guard1875;
        *var_guard1877_slot = var_guard1877;
        *var_guard1879_slot = var_guard1879;
    }

    pub(super) fn stamp_transient_block_82(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_coxb0: f64,
        var_guard1873: f64,
        var_guard1874: f64,
        var_guard1875: f64,
        var_guard1879: f64,
        var_uc_nover: f64,
        var_vdsei: f64,
        var_vdsei_db0: f64,
        var_vdsei_db1: f64,
        var_vdsei_db10: f64,
        var_vdsei_db11: f64,
        var_vdsei_db12: f64,
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
        var_vdsei_dn18: f64,
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
        var_vgsei_db12: f64,
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
        var_vgsei_dn18: f64,
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

        let (assign80260_e122808,) = {
    if (((var_guard1875 != 0.0) && (!((var_guard1873 != 0.0) || (var_guard1874 != 0.0)))) && (var_guard1879 != 0.0)) {
        (1.0,)
    } else {
        (var_flg_coovlp,)
    }
};
        var_flg_coovlp = assign80260_e122808;

        if (((s.v[2871] != 0.0) && (!((s.v[2869] != 0.0) || (s.v[2870] != 0.0)))) && (s.v[2875] != 0.0)) {
            s.store_sub(395, 731, 728);
            s.store_sub(396, 729, 728);
            s.store_scalar(409, var_uc_nover);
            s.store_scalar(407, (p.p63 + (p.p64 * p.p55)));
            s.copy_ad(411, 384);
            s.copy_ad(410, 686);
            s.store_scalar(413, var_coxb0);
            s.store_neg(407, 407);
        }

        s.b[2876] = (((s.v[407] < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0));
        s.store_scalar(2876, if s.b[2876] { 1.0 } else { 0.0 });

        if ((((s.v[2871] != 0.0) && (!((s.v[2869] != 0.0) || (s.v[2870] != 0.0)))) && (s.v[2875] != 0.0)) && s.b[2876]) {
            s.store_neg(407, 407);
            s.store_scalar(335, p.p63);
            s.store_offset_div_scaled_product(996, s.ad_value(335), s.ad_value(335), 1.0, s.ad_value(651), 1.0, (-p.p137));
        }

        s.b[2877] = (p.p113 > 0.0);
        s.store_scalar(2877, if s.b[2877] { 1.0 } else { 0.0 });

        s.b[2878] = ((s.v[396] == 0.0) || (p.p113 <= 0.0));
        s.store_scalar(2878, if s.b[2878] { 1.0 } else { 0.0 });

        if ((((((s.v[2871] != 0.0) && (!((s.v[2869] != 0.0) || (s.v[2870] != 0.0)))) && (s.v[2875] != 0.0)) && s.b[2876]) && s.b[2877]) && s.b[2878]) {
        }

        if ((((((s.v[2871] != 0.0) && (!((s.v[2869] != 0.0) || (s.v[2870] != 0.0)))) && (s.v[2875] != 0.0)) && s.b[2876]) && s.b[2877]) && (!s.b[2878])) {
            s.store_scalar(783, (if (s.v[396] < 0.0) { (-1.0) } else { 1.0 }));
        }

        if ((((((s.v[2871] != 0.0) && (!((s.v[2869] != 0.0) || (s.v[2870] != 0.0)))) && (s.v[2875] != 0.0)) && s.b[2876]) && s.b[2877]) && (!s.b[2878])) {
            s.store_mul(784, 783, 396);
            s.store_offset_powf_ad(781, A::div(s.ad_value(784), s.ad_value(996)), p.p113, 1.0);
            s.store_powf(782, 781, (1.0 / p.p113));
            s.store_div_scaled_product_indices(396, 783, 784, 1.0, 782, 1.0);
        }

        if (((((s.v[2871] != 0.0) && (!((s.v[2869] != 0.0) || (s.v[2870] != 0.0)))) && (s.v[2875] != 0.0)) && s.b[2876]) && s.b[2877]) {
            s.store_sqrt_offset_square_offset(782, 396, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(396), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(396), p.p137), 782, 0.5);
        }

        s.b[2879] = (s.v[336] < 0.0);
        s.store_scalar(2879, if s.b[2879] { 1.0 } else { 0.0 });

        if ((((((s.v[2871] != 0.0) && (!((s.v[2869] != 0.0) || (s.v[2870] != 0.0)))) && (s.v[2875] != 0.0)) && s.b[2876]) && s.b[2877]) && s.b[2879]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((((s.v[2871] != 0.0) && (!((s.v[2869] != 0.0) || (s.v[2870] != 0.0)))) && (s.v[2875] != 0.0)) && s.b[2876]) && s.b[2877]) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub(407, 407, 600);
        }

        s.b[2880] = (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.store_scalar(2880, if s.b[2880] { 1.0 } else { 0.0 });

        let (assign80560_e123279,) = {
    if ((s.b[2872] && (!(((var_guard1873 != 0.0) || (var_guard1874 != 0.0)) || (var_guard1875 != 0.0)))) && s.b[2880]) {
        (1.0,)
    } else {
        (s.v[2625],)
    }
};
        s.store_scalar(2625, assign80560_e123279);

        if ((s.b[2872] && (!(((s.v[2869] != 0.0) || (s.v[2870] != 0.0)) || (s.v[2871] != 0.0)))) && s.b[2880]) {
            s.store_sub_ad_lhs(395, A::from_derivatives(var_vgsei, [var_vgsei_dn0, var_vgsei_dn1, var_vgsei_dn2, var_vgsei_dn3, var_vgsei_dn4, var_vgsei_dn5, var_vgsei_dn6, var_vgsei_dn7, var_vgsei_dn8, var_vgsei_dn9, var_vgsei_dn10, var_vgsei_dn11, var_vgsei_dn12, var_vgsei_dn13, var_vgsei_dn14, var_vgsei_dn15, var_vgsei_dn16, var_vgsei_dn17, var_vgsei_dn18], [var_vgsei_db0, var_vgsei_db1, var_vgsei_db2, var_vgsei_db3, var_vgsei_db4, var_vgsei_db5, var_vgsei_db6, var_vgsei_db7, var_vgsei_db8, var_vgsei_db9, var_vgsei_db10, var_vgsei_db11, var_vgsei_db12]), 735);
            s.store_sub_ad_lhs(396, A::from_derivatives(var_vdsei, [var_vdsei_dn0, var_vdsei_dn1, var_vdsei_dn2, var_vdsei_dn3, var_vdsei_dn4, var_vdsei_dn5, var_vdsei_dn6, var_vdsei_dn7, var_vdsei_dn8, var_vdsei_dn9, var_vdsei_dn10, var_vdsei_dn11, var_vdsei_dn12, var_vdsei_dn13, var_vdsei_dn14, var_vdsei_dn15, var_vdsei_dn16, var_vdsei_dn17, var_vdsei_dn18], [var_vdsei_db0, var_vdsei_db1, var_vdsei_db2, var_vdsei_db3, var_vdsei_db4, var_vdsei_db5, var_vdsei_db6, var_vdsei_db7, var_vdsei_db8, var_vdsei_db9, var_vdsei_db10, var_vdsei_db11, var_vdsei_db12]), 735);
        }

        if (s.v[2625] != 0.0) {
            s.store_scalar(2888, 0.4);
        }

        let (assign80610_e123321,) = {
    if (s.v[2625] != 0.0) {
        (0.0,)
    } else {
        (s.v[2889],)
    }
};
        s.store_scalar(2889, assign80610_e123321);

        if (s.v[2625] != 0.0) {
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

        let (assign80740_e123374,) = {
    if (s.v[2625] != 0.0) {
        let assign80740_e123372: f64 = (-1.0);
        (assign80740_e123372,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign80740_e123374);

        if (s.v[2625] != 0.0) {
            s.store_scalar(2890, 0.0);
            s.store_scalar(2891, 0.0);
            s.store_mul_scaled_ln_ad_rhs(2886, 155, 2.0, A::div(s.ad_value(409), s.ad_value(394)));
            s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(2886), (-0.1));
            s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        }

        if (s.v[2625] != 0.0) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.v[2625] != 0.0) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(2887, 781, (-0.5), 782, (-0.5), 0.8);
        }

        s.b[2893] = (s.v[2888] > (s.v[2887] * 0.5));
        s.store_scalar(2893, if s.b[2893] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[2893]) {
            s.store_scale(2888, 2887, 0.5);
        }

        s.b[2894] = param_given[338];
        s.store_scalar(2894, if s.b[2894] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[2894]) {
            s.store_scalar(2887, p.p338);
        }

        s.b[2895] = param_given[339];
        s.store_scalar(2895, if s.b[2895] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[2895]) {
            s.store_scalar(2888, p.p339);
        }

        s.b[2896] = param_given[338];
        s.store_scalar(2896, if s.b[2896] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (!s.b[2895])) && s.b[2896]) {
            s.store_scale(2888, 2887, 0.5);
        }

        s.b[2897] = (s.v[2888] > (s.v[2887] * 0.5));
        s.store_scalar(2897, if s.b[2897] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[2897]) {
            s.store_scale(2888, 2887, 0.5);
        }

        s.b[2898] = (p.p38 == 1.0);
        s.store_scalar(2898, if s.b[2898] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[2898]) {
            s.store_neg(334, 396);
        }

        s.b[2899] = (s.v[334] > s.v[2888]);
        s.store_scalar(2899, if s.b[2899] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && s.b[2898]) && s.b[2899]) {
            s.store_sub(335, 334, 2888);
            s.store_sub(336, 2887, 2888);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 345, 1.0);
            s.store_neg(345, 345);
            s.store_add(344, 2888, 333);
        }

        if (((s.v[2625] != 0.0) && s.b[2898]) && (!s.b[2899])) {
            s.copy_ad(344, 334);
        }

        if ((s.v[2625] != 0.0) && s.b[2898]) {
            s.store_neg(397, 344);
        }

        if ((s.v[2625] != 0.0) && (!s.b[2898])) {
            s.copy_ad(397, 396);
        }

        if (s.v[2625] != 0.0) {
            s.store_div(212, 410, 413);
            s.store_square(213, 212);
            s.store_sub_from_scalar(402, s.v[458], 395);
        }

        let (assign81150_e123715,) = {
    if (s.v[2625] != 0.0) {
        let assign81150_e123709: f64 = (-s.v[397]);
        let assign81150_e123712: f64 = (10.0 * 2.220446049250313e-16);
        let assign81150_e123713: f64 = (assign81150_e123709 + assign81150_e123712);
        (assign81150_e123713,)
    } else {
        (s.v[403],)
    }
};
        s.store_scalar(403, assign81150_e123715);

        if (s.v[2625] != 0.0) {
            s.store_scalar(2882, 0.0);
            s.store_scale(2883, 409, 1.6021918e-19);
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
        }

        s.b[2900] = ((s.v[154] * (-s.v[397])) >= 500.0);
        s.store_scalar(2900, if s.b[2900] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[2900]) {
            s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(334, 1.403592217853e217);
        }

        if ((s.v[2625] != 0.0) && (!s.b[2900])) {
            s.store_mul_neg_rhs(781, 154, 397);
            s.store_scalar(229, 1.0);
        }

        let mut assign81250_loop_guard: usize = 0;
        while {
            let assign81250_cond_e123789: f64 = if (((s.v[2625] != 0.0) && (!s.b[2900])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign81250_cond_e123789 != 0.0
        } {
            assign81250_loop_guard += 1;
            assert!(assign81250_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2625] != 0.0) && (!s.b[2900])) {
                s.store_scale(229, 229, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if ((s.v[2625] != 0.0) && (!s.b[2900])) {
            s.store_mul_exp_rhs(229, 229, 781);
            s.copy_ad(334, 229);
        }

        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {
            s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));
            s.store_scalar(782, (4.0 * 0.5));
        }

        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(335, 781, 0.5, 782, 0.5, 0.5);
        }

        s.b[2901] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));
        s.store_scalar(2901, if s.b[2901] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) {
            s.store_add_scaled_inputs3_indices(781, 402, 1.0, 397, 1.0, 335, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 335);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign81400_e123963,) = {
    if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign81400_e123963);

        let (assign81410_e123971,) = {
    if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign81410_e123971);

        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) {
            s.store_scalar(770, 0.0);
        }

        *var_flg_coovlp_slot = var_flg_coovlp;
    }

    pub(super) fn stamp_transient_block_83(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) {
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2902] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.store_scalar(2902, if s.b[2902] { 1.0 } else { 0.0 });

        s.b[2903] = (1.0 == 1.0);
        s.store_scalar(2903, if s.b[2903] { 1.0 } else { 0.0 });

        let (assign81500_e124055,) = {
    if (((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) && s.b[2902]) && s.b[2903]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign81500_e124055);

        s.b[2904] = (1.0 == 2.0);
        s.store_scalar(2904, if s.b[2904] { 1.0 } else { 0.0 });

        let (assign81520_e124073,) = {
    if ((((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) && s.b[2902]) && (!s.b[2903])) && s.b[2904]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign81520_e124073);

        s.b[2905] = (1.0 == 4.0);
        s.store_scalar(2905, if s.b[2905] { 1.0 } else { 0.0 });

        let (assign81540_e124094,) = {
    if (((((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) && s.b[2902]) && (!s.b[2903])) && (!s.b[2904])) && s.b[2905]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign81540_e124094);

        s.b[2906] = (1.0 == 8.0);
        s.store_scalar(2906, if s.b[2906] { 1.0 } else { 0.0 });

        let (assign81560_e124118,) = {
    if ((((((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) && s.b[2902]) && (!s.b[2903])) && (!s.b[2904])) && (!s.b[2905])) && s.b[2906]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign81560_e124118);

        let (assign81570_e124128,) = {
    if ((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) && s.b[2902]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign81570_e124128);

        let mut assign81580_loop_guard: usize = 0;
        while {
            let assign81580_cond_e124139: f64 = if (((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) && s.b[2902]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign81580_cond_e124139 != 0.0
        } {
            assign81580_loop_guard += 1;
            assert!(assign81580_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) && s.b[2902]) {
                s.store_sqrt(726, 726);
            }
            let (assign81580_body1_e124162,) = {
    if ((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) && s.b[2902]) {
        let assign81580_body1_e124160: f64 = (s.v[719] + 1.0);
        (assign81580_body1_e124160,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign81580_body1_e124162);
        }

        if ((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) && (!s.b[2902])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);
        }

        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) {
        }

        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && (!s.b[2901])) {
            s.store_add(335, 402, 397);
            s.store_scalar(334, 1.0);
        }

        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sub(397, 335, 402);
        }

        let (assign81680_e124279,) = {
    if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {
        let assign81680_e124273: f64 = (-s.v[397]);
        let assign81680_e124276: f64 = (10.0 * 2.220446049250313e-16);
        let assign81680_e124277: f64 = (assign81680_e124273 + assign81680_e124276);
        (assign81680_e124277,)
    } else {
        (s.v[403],)
    }
};
        s.store_scalar(403, assign81680_e124279);

        s.b[2907] = (s.v[402] < s.v[403]);
        s.store_scalar(2907, if s.b[2907] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[2907]) {
            s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));
            s.store_mul_add_rhs(332, 154, 402, 397);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(410));
            s.store_mul(333, 335, 413);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(278, (7.0 * 1.414213562373095), 333, A::offset(s.ad_value(332), (-2.0)), 9.0);
            s.store_square(276, 278);
        }

        s.b[2908] = (s.v[277] < (s.v[276] * 1e-8));
        s.store_scalar(2908, if s.b[2908] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && s.b[2907]) && s.b[2908]) {
            s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);
        }

        if (((s.v[2625] != 0.0) && s.b[2907]) && (!s.b[2908])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((s.v[2625] != 0.0) && s.b[2907]) {
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
            s.copy_ad(2890, 404);
        }

        s.b[2909] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));
        s.store_scalar(2909, if s.b[2909] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2909]) {
            s.store_add_scaled_product_indices(89, 402, 1.0, 213, 154, 0.5);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2907])) && (!s.b[2909])) {
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
        }

        if ((s.v[2625] != 0.0) && (!s.b[2907])) {
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        s.b[2910] = (s.v[116] >= 3.0);
        s.store_scalar(2910, if s.b[2910] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2910]) {
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
            s.store_mul_add_rhs(116, 154, 89, 397);
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2907])) && (!s.b[2910])) {
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

        s.b[2911] = (p.p33 > 0.0);
        s.store_scalar(2911, if s.b[2911] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) {
            s.store_offset_add(442, 402, 397, 0.1);
            s.store_mul(222, 405, 229);
            s.store_mul(443, 405, 229);
            s.store_mul(334, 156, 213);
            s.store_mul(444, 154, 442);
            s.store_add_scaled_inputs_product_mixed_aaii(447, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), 154, 397, 1.0);
        }

        s.b[2912] = (p.p33 == 2.0);
        s.store_scalar(2912, if s.b[2912] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2912]) {
            s.store_offset_sub(781, 444, 447, (-1.0));
            s.store_scale(782, 444, 4.0);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2912]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2912]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(447, 444, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && (!s.b[2912])) {
            if (s.v[447] <= s.v[444]) {
            } else {
                s.copy_ad(447, 444);
            }
        }

        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) {
            if (s.v[447] >= 0.0) {
            } else {
                s.store_scalar(447, 0.0);
            }
        }

        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) {
            s.store_sub(444, 444, 447);
            s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);
            s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));
            s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }

        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) {
            s.copy_ad(445, 116);
        }

        s.b[2913] = (p.p33 == 2.0);
        s.store_scalar(2913, if s.b[2913] { 1.0 } else { 0.0 });

        s.b[2914] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));
        s.store_scalar(2914, if s.b[2914] { 1.0 } else { 0.0 });

        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) {
            s.store_add_scaled_inputs3_indices(781, 445, 1.0, 446, (-1.0), 446, 0.2);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 446, 446, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign82510_e125425,) = {
    if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign82510_e125425);

        let (assign82520_e125438,) = {
    if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign82520_e125438);

        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
        }

    }

    pub(super) fn stamp_transient_block_84(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) {
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2915] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(2915, if s.b[2915] { 1.0 } else { 0.0 });

        s.b[2916] = (2.0 == 1.0);
        s.store_scalar(2916, if s.b[2916] { 1.0 } else { 0.0 });

        let (assign82630_e125587,) = {
    if (((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) && s.b[2915]) && s.b[2916]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign82630_e125587);

        s.b[2917] = (2.0 == 2.0);
        s.store_scalar(2917, if s.b[2917] { 1.0 } else { 0.0 });

        let (assign82650_e125610,) = {
    if ((((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) && s.b[2915]) && (!s.b[2916])) && s.b[2917]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign82650_e125610);

        s.b[2918] = (2.0 == 4.0);
        s.store_scalar(2918, if s.b[2918] { 1.0 } else { 0.0 });

        let (assign82670_e125636,) = {
    if (((((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) && s.b[2915]) && (!s.b[2916])) && (!s.b[2917])) && s.b[2918]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign82670_e125636);

        s.b[2919] = (2.0 == 8.0);
        s.store_scalar(2919, if s.b[2919] { 1.0 } else { 0.0 });

        let (assign82690_e125665,) = {
    if ((((((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) && s.b[2915]) && (!s.b[2916])) && (!s.b[2917])) && (!s.b[2918])) && s.b[2919]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign82690_e125665);

        let (assign82700_e125680,) = {
    if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) && s.b[2915]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign82700_e125680);

        let mut assign82710_loop_guard: usize = 0;
        while {
            let assign82710_cond_e125696: f64 = if (((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) && s.b[2915]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign82710_cond_e125696 != 0.0
        } {
            assign82710_loop_guard += 1;
            assert!(assign82710_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) && s.b[2915]) {
                s.store_sqrt(726, 726);
            }
            let (assign82710_body1_e125729,) = {
    if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) && s.b[2915]) {
        let assign82710_body1_e125727: f64 = (s.v[719] + 1.0);
        (assign82710_body1_e125727,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign82710_body1_e125729);
        }

        if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) && (!s.b[2915])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);
            s.store_div_scaled_product3_indices(335, 446, 725, 726, 0.2, 770, 1.0);
            s.store_add_scaled_inputs3_indices(116, 446, 1.0, 446, (-0.2), 780, 1.0);
        }

        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) {
        }

        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && (!s.b[2914])) {
            s.copy_ad(116, 445);
            s.store_scalar(335, 1.0);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && (!s.b[2913])) {
            if (s.v[445] <= s.v[446]) {
                s.copy_ad(116, 445);
            } else {
                s.copy_ad(116, 446);
            }
        }

        s.b[2920] = (p.p33 == 1.0);
        s.store_scalar(2920, if s.b[2920] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2921] = (s.v[411] > 0.0);
        s.store_scalar(2921, if s.b[2921] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) && s.b[2921]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) && (!s.b[2921])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2922] = (s.v[336] < 0.0);
        s.store_scalar(2922, if s.b[2922] { 1.0 } else { 0.0 });

        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) && (!s.b[2921])) && s.b[2922]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) && (!s.b[2921])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2923] = (s.v[336] < 0.0);
        s.store_scalar(2923, if s.b[2923] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) && s.b[2923]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2883, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(419, 335, 418);
            s.store_add(414, 404, 397);
            s.store_mul_sub_rhs(333, 419, 414, 418);
        }

        s.b[2924] = (s.v[333] < 60.0);
        s.store_scalar(2924, if s.b[2924] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) && s.b[2924]) {
            s.store_exp(335, 333);
            s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
            s.store_sub(336, 335, 334);
            s.store_div_ln_offset_lhs(416, 336, 1.0, 419);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) && (!s.b[2924])) {
            s.store_sub(416, 414, 418);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) {
            s.store_mul(415, 154, 416);
        }

        s.b[2925] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));
        s.store_scalar(2925, if s.b[2925] { 1.0 } else { 0.0 });

        let (assign83140_e126318,) = {
    if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) && s.b[2925]) {
        let assign83140_e126316: f64 = (s.v[2889] + 1.0);
        (assign83140_e126316,)
    } else {
        (s.v[2889],)
    }
};
        s.store_scalar(2889, assign83140_e126318);

        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) && s.b[2925]) {
            s.copy_ad(116, 447);
        }

        if ((s.v[2625] != 0.0) && (!s.b[2907])) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[2926] = (((s.v[116]) as f64).abs() > 1e-6);
        s.store_scalar(2926, if s.b[2926] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2926]) {
            s.store_add_offset_lhs_ad_rhs(335, 116, (-1.0), A::exp_scaled_input(s.ad_value(116), -1.0));
            s.store_sqrt(336, 335);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2907])) && (!s.b[2926])) {
            s.store_mul_scaled_sqrt_ad_rhs(336, 116, 0.7071067811865475, A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333)));
        }

        if ((s.v[2625] != 0.0) && (!s.b[2907])) {
            s.store_mul(354, 410, 336);
            s.store_mul_sub_rhs(398, 413, 402, 404);
            s.store_div(2927, 354, 2883);
        }

        s.b[2929] = (p.p33 == 2.0);
        s.store_scalar(2929, if s.b[2929] { 1.0 } else { 0.0 });

        s.b[2930] = ((s.v[2927] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));
        s.store_scalar(2930, if s.b[2930] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) {
            s.store_add_scaled_inputs3_indices(781, 2927, 1.0, 386, (-1.0), 386, 0.1);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 386, 386, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign83320_e126525,) = {
    if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign83320_e126525);

        let (assign83330_e126536,) = {
    if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign83330_e126536);

        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2931] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(2931, if s.b[2931] { 1.0 } else { 0.0 });

        s.b[2932] = (2.0 == 1.0);
        s.store_scalar(2932, if s.b[2932] { 1.0 } else { 0.0 });

        let (assign83440_e126667,) = {
    if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) && s.b[2931]) && s.b[2932]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign83440_e126667);

        s.b[2933] = (2.0 == 2.0);
        s.store_scalar(2933, if s.b[2933] { 1.0 } else { 0.0 });

        let (assign83460_e126688,) = {
    if (((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) && s.b[2931]) && (!s.b[2932])) && s.b[2933]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign83460_e126688);

        s.b[2934] = (2.0 == 4.0);
        s.store_scalar(2934, if s.b[2934] { 1.0 } else { 0.0 });

        let (assign83480_e126712,) = {
    if ((((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) && s.b[2931]) && (!s.b[2932])) && (!s.b[2933])) && s.b[2934]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign83480_e126712);

        s.b[2935] = (2.0 == 8.0);
        s.store_scalar(2935, if s.b[2935] { 1.0 } else { 0.0 });

        let (assign83500_e126739,) = {
    if (((((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) && s.b[2931]) && (!s.b[2932])) && (!s.b[2933])) && (!s.b[2934])) && s.b[2935]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign83500_e126739);

        let (assign83510_e126752,) = {
    if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) && s.b[2931]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign83510_e126752);

        let mut assign83520_loop_guard: usize = 0;
        while {
            let assign83520_cond_e126766: f64 = if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) && s.b[2931]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign83520_cond_e126766 != 0.0
        } {
            assign83520_loop_guard += 1;
            assert!(assign83520_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) && s.b[2931]) {
                s.store_sqrt(726, 726);
            }
            let (assign83520_body1_e126795,) = {
    if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) && s.b[2931]) {
        let assign83520_body1_e126793: f64 = (s.v[719] + 1.0);
        (assign83520_body1_e126793,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign83520_body1_e126795);
        }

        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) && (!s.b[2931])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 386, 725, 726, 0.1, 770, 1.0);
            s.store_add_scaled_inputs3_indices(335, 386, 1.0, 386, (-0.1), 780, 1.0);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) {
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && (!s.b[2930])) {
            s.copy_ad(335, 2927);
        }

    }

    pub(super) fn stamp_transient_block_85(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && (!s.b[2930])) {
            s.store_scalar(334, 1.0);
        }

        s.b[2936] = (s.v[334] < 1.0);
        s.store_scalar(2936, if s.b[2936] { 1.0 } else { 0.0 });

        let (assign83620_e126937,) = {
    if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2936]) {
        let assign83620_e126935: f64 = (s.v[2889] + 2.0);
        (assign83620_e126935,)
    } else {
        (s.v[2889],)
    }
};
        s.store_scalar(2889, assign83620_e126937);

        if (((s.v[2625] != 0.0) && (!s.b[2907])) && (!s.b[2929])) {
            if (s.v[2927] <= s.v[386]) {
                s.copy_ad(335, 2927);
            } else {
                s.copy_ad(335, 386);
            }
        }

        s.b[2937] = (s.v[2927] >= s.v[386]);
        s.store_scalar(2937, if s.b[2937] { 1.0 } else { 0.0 });

        let (assign83650_e126969,) = {
    if ((((s.v[2625] != 0.0) && (!s.b[2907])) && (!s.b[2929])) && s.b[2937]) {
        let assign83650_e126967: f64 = (s.v[2889] + 2.0);
        (assign83650_e126967,)
    } else {
        (s.v[2889],)
    }
};
        s.store_scalar(2889, assign83650_e126969);

        s.b[2938] = (s.v[2889] >= 2.0);
        s.store_scalar(2938, if s.b[2938] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) {
            s.copy_ad(2928, 404);
            s.store_mul(354, 335, 2883);
            s.store_sub_div_rhs_indices(404, 402, 354, 413);
        }

        s.b[2939] = (p.p33 == 2.0);
        s.store_scalar(2939, if s.b[2939] { 1.0 } else { 0.0 });

        s.b[2940] = ((s.v[404] > (s.v[2928] - 0.1)) && (0.1 >= 0.0));
        s.store_scalar(2940, if s.b[2940] { 1.0 } else { 0.0 });

        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) {
            s.store_offset_sub(781, 404, 2928, 0.1);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign83770_e127103,) = {
    if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign83770_e127103);

        let (assign83780_e127116,) = {
    if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign83780_e127116);

        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2941] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(2941, if s.b[2941] { 1.0 } else { 0.0 });

        s.b[2942] = (2.0 == 1.0);
        s.store_scalar(2942, if s.b[2942] { 1.0 } else { 0.0 });

        let (assign83890_e127265,) = {
    if (((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) && s.b[2941]) && s.b[2942]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign83890_e127265);

        s.b[2943] = (2.0 == 2.0);
        s.store_scalar(2943, if s.b[2943] { 1.0 } else { 0.0 });

        let (assign83910_e127288,) = {
    if ((((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) && s.b[2941]) && (!s.b[2942])) && s.b[2943]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign83910_e127288);

        s.b[2944] = (2.0 == 4.0);
        s.store_scalar(2944, if s.b[2944] { 1.0 } else { 0.0 });

        let (assign83930_e127314,) = {
    if (((((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) && s.b[2941]) && (!s.b[2942])) && (!s.b[2943])) && s.b[2944]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign83930_e127314);

        s.b[2945] = (2.0 == 8.0);
        s.store_scalar(2945, if s.b[2945] { 1.0 } else { 0.0 });

        let (assign83950_e127343,) = {
    if ((((((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) && s.b[2941]) && (!s.b[2942])) && (!s.b[2943])) && (!s.b[2944])) && s.b[2945]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign83950_e127343);

        let (assign83960_e127358,) = {
    if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) && s.b[2941]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign83960_e127358);

        let mut assign83970_loop_guard: usize = 0;
        while {
            let assign83970_cond_e127374: f64 = if (((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) && s.b[2941]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign83970_cond_e127374 != 0.0
        } {
            assign83970_loop_guard += 1;
            assert!(assign83970_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) && s.b[2941]) {
                s.store_sqrt(726, 726);
            }
            let (assign83970_body1_e127407,) = {
    if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) && s.b[2941]) {
        let assign83970_body1_e127405: f64 = (s.v[719] + 1.0);
        (assign83970_body1_e127405,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign83970_body1_e127407);
        }

        if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) && (!s.b[2941])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_add_offset_lhs(404, 2928, (-0.1), 780);
        }

        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) {
        }

        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && (!s.b[2940])) {
        }

        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && (!s.b[2940])) {
            s.store_scalar(334, 1.0);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && (!s.b[2939])) {
            if (s.v[404] <= s.v[2928]) {
            } else {
                s.copy_ad(404, 2928);
            }
        }

        if ((s.v[2625] != 0.0) && (!s.b[2907])) {
            s.copy_ad(2890, 404);
        }

        s.b[2946] = (p.p33 == 1.0);
        s.store_scalar(2946, if s.b[2946] { 1.0 } else { 0.0 });

        let (assign84090_e127579,) = {
    if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign84090_e127579);

        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) {
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2883)), s.ad_value(155)), 2.0);
        }

        s.b[2947] = (s.v[411] > 0.0);
        s.store_scalar(2947, if s.b[2947] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && s.b[2947]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2947])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2948] = (s.v[336] < 0.0);
        s.store_scalar(2948, if s.b[2948] { 1.0 } else { 0.0 });

        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2947])) && s.b[2948]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2947])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2949] = (s.v[336] < 0.0);
        s.store_scalar(2949, if s.b[2949] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && s.b[2949]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2883, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(419, 335, 418);
        }

        let (assign84320_e127888,) = {
    if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, assign84320_e127888);

    }

    pub(super) fn stamp_transient_block_86(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign84330_loop_guard: usize = 0;
        while {
            let assign84330_cond_e127898: f64 = (s.v[421] + 1.0);
            let assign84330_cond_e127900: f64 = if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (s.v[97] <= assign84330_cond_e127898)) { 1.0 } else { 0.0 };
            assign84330_cond_e127900 != 0.0
        } {
            assign84330_loop_guard += 1;
            assert!(assign84330_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2951] = (s.v[333] < 60.0);
            s.store_scalar(2951, if s.b[2951] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && s.b[2951]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ln_offset_lhs(416, 336, 1.0, 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2951])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2952] = (s.v[116] < 0.0);
            s.store_scalar(2952, if s.b[2952] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && s.b[2952]) {
                s.store_scalar(334, (-0.7071067811865475));
                s.store_mul(223, 116, 334);
                s.store_mul(420, 154, 334);
            }
            s.b[2953] = (s.v[116] < 1e-6);
            s.store_scalar(2953, if s.b[2953] { 1.0 } else { 0.0 });
            if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2952])) && s.b[2953]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(338, 334, 336);
            }
            s.b[2954] = (s.v[338] > 0.0);
            s.store_scalar(2954, if s.b[2954] { 1.0 } else { 0.0 });
            if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2952])) && s.b[2953]) && s.b[2954]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);
            }
            if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2952])) && s.b[2953]) && (!s.b[2954])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2952])) && (!s.b[2953])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(338, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
            }
            s.b[2955] = (s.v[338] > 0.0);
            s.store_scalar(2955, if s.b[2955] { 1.0 } else { 0.0 });
            if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2952])) && (!s.b[2953])) && s.b[2955]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335))), 0.5, 223, 1.0);
            }
            if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2952])) && (!s.b[2953])) && (!s.b[2955])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            s.b[2956] = (s.v[116] < 0.0);
            s.store_scalar(2956, if s.b[2956] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && s.b[2956]) {
                s.store_scalar(214, 0.0);
                s.store_scalar(215, 0.0);
                s.store_neg(216, 223);
                s.store_neg(217, 420);
            }
            s.b[2957] = (s.v[116] < 60.0);
            s.store_scalar(2957, if s.b[2957] { 1.0 } else { 0.0 });
            s.b[2958] = (s.v[116] < 5e-5);
            s.store_scalar(2958, if s.b[2958] { 1.0 } else { 0.0 });
            if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2956])) && s.b[2957]) && s.b[2958]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2956])) && s.b[2957]) && (!s.b[2958])) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2956])) && (!s.b[2957])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2959] = (s.v[214] > 0.0);
            s.store_scalar(2959, if s.b[2959] { 1.0 } else { 0.0 });
            if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2956])) && s.b[2959]) {
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_scaled_add_product(217, s.ad_value(215), 0.5, s.ad_value(420), s.ad_value(223), (2.0 * 0.5), s.ad_value(216), 1.0);
            }
            if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2956])) && (!s.b[2959])) {
                s.copy_ad(216, 223);
                s.copy_ad(217, 420);
            }
            if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2960] = (s.v[79] == 1.0);
            s.store_scalar(2960, if s.b[2960] { 1.0 } else { 0.0 });
            let (assign84330_body72_e129046,) = {
    if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && s.b[2960]) {
        let assign84330_body72_e129044: f64 = (s.v[421] + 1.0);
        (assign84330_body72_e129044,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign84330_body72_e129046);
            if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2960])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2960])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2961] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.store_scalar(2961, if s.b[2961] { 1.0 } else { 0.0 });
            if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2960])) && s.b[2961]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2960])) {
                s.store_add(404, 404, 236);
            }
            s.b[2962] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.store_scalar(2962, if s.b[2962] { 1.0 } else { 0.0 });
            let (assign84330_body79_e129149,) = {
    if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2960])) && s.b[2962]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, assign84330_body79_e129149);
            let (assign84330_body80_e129160,) = {
    if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) {
        let assign84330_body80_e129158: f64 = (s.v[97] + 1.0);
        (assign84330_body80_e129158,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign84330_body80_e129160);
        }

        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) {
            s.store_mul(2881, 982, 223);
            s.store_mul(2882, 2883, 2881);
            s.store_offset_div(100, 2882, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        s.b[2964] = (p.p33 == 4.0);
        s.store_scalar(2964, if s.b[2964] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[2964]) {
            s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
            s.store_mul(222, 405, 229);
            s.copy_ad(404, 2890);
        }

        let (assign84480_e129297,) = {
    if ((s.v[2625] != 0.0) && s.b[2964]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign84480_e129297);

        if ((s.v[2625] != 0.0) && s.b[2964]) {
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2883)), s.ad_value(155)), 2.0);
        }

        s.b[2965] = (s.v[411] > 0.0);
        s.store_scalar(2965, if s.b[2965] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && s.b[2964]) && s.b[2965]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if (((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2965])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[2966] = (s.v[336] < 0.0);
        s.store_scalar(2966, if s.b[2966] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2965])) && s.b[2966]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2965])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if ((s.v[2625] != 0.0) && s.b[2964]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[2967] = (s.v[336] < 0.0);
        s.store_scalar(2967, if s.b[2967] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && s.b[2964]) && s.b[2967]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((s.v[2625] != 0.0) && s.b[2964]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 2883, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(419, 335, 418);
        }

        let (assign84710_e129546,) = {
    if ((s.v[2625] != 0.0) && s.b[2964]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, assign84710_e129546);

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
        var_vgsei_db12: f64,
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
        var_vgsei_dn18: f64,
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
        var_guard1994_slot: &mut f64,
        var_guard1995_slot: &mut f64,
        var_guard1996_slot: &mut f64,
        var_guard1998_slot: &mut f64,
        var_guard2000_slot: &mut f64,
    ) {
        let mut var_flg_coovlps: f64 = *var_flg_coovlps_slot;
        let mut var_guard1994: f64 = *var_guard1994_slot;
        let mut var_guard1995: f64 = *var_guard1995_slot;
        let mut var_guard1996: f64 = *var_guard1996_slot;
        let mut var_guard1998: f64 = *var_guard1998_slot;
        let mut var_guard2000: f64 = *var_guard2000_slot;

        let mut assign84720_loop_guard: usize = 0;
        while {
            let assign84720_cond_e129553: f64 = (s.v[421] + 1.0);
            let assign84720_cond_e129555: f64 = if (((s.v[2625] != 0.0) && s.b[2964]) && (s.v[97] <= assign84720_cond_e129553)) { 1.0 } else { 0.0 };
            assign84720_cond_e129555 != 0.0
        } {
            assign84720_loop_guard += 1;
            assert!(assign84720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2625] != 0.0) && s.b[2964]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[2969] = (s.v[333] < 60.0);
            s.store_scalar(2969, if s.b[2969] { 1.0 } else { 0.0 });
            if (((s.v[2625] != 0.0) && s.b[2964]) && s.b[2969]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ln_offset_lhs(416, 336, 1.0, 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if (((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2969])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if ((s.v[2625] != 0.0) && s.b[2964]) {
                s.store_mul(415, 154, 416);
            }
            s.b[2970] = (((s.v[116]) as f64).abs() < 1e-6);
            s.store_scalar(2970, if s.b[2970] { 1.0 } else { 0.0 });
            if (((s.v[2625] != 0.0) && s.b[2964]) && s.b[2970]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(2891, 334, 336);
                s.store_mul_add_scaled_product_rhs(2892, 154, s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0));
            }
            if (((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2970])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(2891, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
                s.store_mul_sub_ad_rhs(2892, 154, A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335)));
            }
            s.b[2971] = (((s.v[116]) as f64).abs() < 5e-5);
            s.store_scalar(2971, if s.b[2971] { 1.0 } else { 0.0 });
            if (((s.v[2625] != 0.0) && s.b[2964]) && s.b[2971]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            s.b[2972] = (((s.v[116]) as f64).abs() < 60.0);
            s.store_scalar(2972, if s.b[2972] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2971])) && s.b[2972]) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if ((((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2971])) && (!s.b[2972])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[2973] = (s.v[214] > 0.0);
            s.store_scalar(2973, if s.b[2973] { 1.0 } else { 0.0 });
            if (((s.v[2625] != 0.0) && s.b[2964]) && s.b[2973]) {
                s.store_sqrt_add(216, 2891, 214);
                s.store_div_scaled_inputs2_indices(217, 2892, 0.5, 215, 0.5, 216, 1.0);
            }
            s.b[2974] = (s.v[2891] > 0.0);
            s.store_scalar(2974, if s.b[2974] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2973])) && s.b[2974]) {
                s.store_sqrt(216, 2891);
                s.store_div_scaled_inputs_indices(217, 2892, 0.5, 216, 1.0);
            }
            if ((((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2973])) && (!s.b[2974])) {
                s.store_scalar(216, 0.0);
                s.store_scalar(217, 0.0);
            }
            if ((s.v[2625] != 0.0) && s.b[2964]) {
                s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2625] != 0.0) && s.b[2964]) {
                s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2625] != 0.0) && s.b[2964]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[2975] = (s.v[79] > 0.0);
            s.store_scalar(2975, if s.b[2975] { 1.0 } else { 0.0 });
            let (assign84720_body56_e130295,) = {
    if (((s.v[2625] != 0.0) && s.b[2964]) && s.b[2975]) {
        let assign84720_body56_e130293: f64 = (s.v[421] + 1.0);
        (assign84720_body56_e130293,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign84720_body56_e130295);
            if (((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2975])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if (((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2975])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2976] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.store_scalar(2976, if s.b[2976] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2975])) && s.b[2976]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2975])) {
                s.store_add(404, 404, 236);
            }
            s.b[2977] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.store_scalar(2977, if s.b[2977] { 1.0 } else { 0.0 });
            let (assign84720_body63_e130385,) = {
    if ((((s.v[2625] != 0.0) && s.b[2964]) && (!s.b[2975])) && s.b[2977]) {
        let assign84720_body63_e130383: f64 = (s.v[79] + 2.0);
        (assign84720_body63_e130383,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, assign84720_body63_e130385);
            let (assign84720_body64_e130393,) = {
    if ((s.v[2625] != 0.0) && s.b[2964]) {
        let assign84720_body64_e130391: f64 = (s.v[97] + 1.0);
        (assign84720_body64_e130391,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign84720_body64_e130393);
        }

        if ((s.v[2625] != 0.0) && s.b[2964]) {
            if (s.v[2891] >= 0.0) {
                s.store_scaled_sqrt(223, 2891, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            } else {
                s.store_scalar(223, 0.0);
            }
        }

        if ((s.v[2625] != 0.0) && s.b[2964]) {
            s.store_mul(2881, 982, 223);
            s.store_mul(2882, 2883, 2881);
            s.store_offset_div(100, 2882, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        if (s.v[2625] != 0.0) {
            s.store_sub(399, 398, 354);
        }

        s.b[2979] = (s.v[407] < 0.0);
        s.store_scalar(2979, if s.b[2979] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[2979]) {
            s.store_neg(407, 407);
        }

        s.b[2980] = (p.p55 == 0.0);
        s.store_scalar(2980, if s.b[2980] { 1.0 } else { 0.0 });

        s.b[2981] = (p.p50 == 0.0);
        s.store_scalar(2981, if s.b[2981] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && s.b[2979]) && s.b[2980]) && s.b[2981]) {
            s.store_neg(2884, 404);
        }

        if ((((s.v[2625] != 0.0) && s.b[2979]) && s.b[2980]) && (!s.b[2981])) {
            s.copy_ad(2884, 396);
        }

        if (((s.v[2625] != 0.0) && s.b[2979]) && s.b[2980]) {
            s.store_sqrt_offset_square_offset(782, 2884, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(2884), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(2884), p.p137), 782, 0.5);
        }

        s.b[2982] = (s.v[336] < 0.0);
        s.store_scalar(2982, if s.b[2982] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && s.b[2979]) && s.b[2980]) && s.b[2982]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2625] != 0.0) && s.b[2979]) && s.b[2980]) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_add_scaled_inputs3_indices(781, 407, 1.0, 600, (-1.0), 407, (-0.1));
            s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));
        }

        if (((s.v[2625] != 0.0) && s.b[2979]) && s.b[2980]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.v[2625] != 0.0) && s.b[2979]) && s.b[2980]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(603, 407, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_sub(407, 407, 603);
        }

        s.b[2983] = (3.0 == 1.0);
        s.store_scalar(2983, if s.b[2983] { 1.0 } else { 0.0 });

        s.b[2984] = (3.0 == 2.0);
        s.store_scalar(2984, if s.b[2984] { 1.0 } else { 0.0 });

        s.b[2985] = (3.0 == 3.0);
        s.store_scalar(2985, if s.b[2985] { 1.0 } else { 0.0 });

        s.b[2986] = (3.0 == 4.0);
        s.store_scalar(2986, if s.b[2986] { 1.0 } else { 0.0 });

        s.b[2987] = (p.p55 == 1.0);
        s.store_scalar(2987, if s.b[2987] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && s.b[2983]) && s.b[2987]) {
            s.store_scale(338, 407, var_weffcv_nf);
        }

        if (((s.v[2625] != 0.0) && s.b[2983]) && (!s.b[2987])) {
            s.store_scale(338, 407, (var_weffcv_nf * (1.0 - s.v[526])));
        }

        if ((s.v[2625] != 0.0) && s.b[2983]) {
            s.store_mul(353, 338, 398);
            s.store_mul(356, 338, 354);
        }

        if ((s.v[2625] != 0.0) && (s.b[2984] && (!s.b[2983]))) {
            s.store_scale(338, 407, (var_weffcv_nf * s.v[526]));
            s.store_mul(351, 338, 398);
            s.store_mul(359, 338, 354);
        }

        s.b[2988] = (p.p55 == 1.0);
        s.store_scalar(2988, if s.b[2988] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (s.b[2985] && (!(s.b[2983] || s.b[2984])))) && s.b[2988]) {
            s.store_scale(338, 407, var_weffcv_nf);
        }

        if (((s.v[2625] != 0.0) && (s.b[2985] && (!(s.b[2983] || s.b[2984])))) && (!s.b[2988])) {
            s.store_scale(338, 407, (var_weffcv_nf * (1.0 - s.v[526])));
        }

        if ((s.v[2625] != 0.0) && (s.b[2985] && (!(s.b[2983] || s.b[2984])))) {
            s.copy_ad(697, 404);
        }

        s.b[2989] = (p.p430 == 0.0);
        s.store_scalar(2989, if s.b[2989] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (s.b[2985] && (!(s.b[2983] || s.b[2984])))) && s.b[2989]) {
            s.copy_ad(698, 354);
        }

        if ((s.v[2625] != 0.0) && (s.b[2985] && (!(s.b[2983] || s.b[2984])))) {
            s.store_mul(352, 338, 398);
            s.store_mul(355, 338, 354);
            s.copy_ad(816, 355);
        }

        if ((s.v[2625] != 0.0) && (s.b[2986] && (!((s.b[2983] || s.b[2984]) || s.b[2985])))) {
            s.store_scale(338, 407, (var_weffcv_nf * s.v[526]));
            s.store_mul(350, 338, 398);
            s.store_mul(358, 338, 354);
        }

        s.store_scalar(2625, 0.0);

        let assign85300_e130960: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        var_guard1994 = assign85300_e130960;

        let assign85310_e130963: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        var_guard1995 = assign85310_e130963;

        let assign85320_e130966: f64 = if 4.0 == 3.0 { 1.0 } else { 0.0 };
        var_guard1996 = assign85320_e130966;

        s.b[2993] = (4.0 == 4.0);
        s.store_scalar(2993, if s.b[2993] { 1.0 } else { 0.0 });

        let assign85340_e130980: f64 = if (((p.p36 == 1.0) && (p.p66 > 0.0)) && (var_uc_novers > 0.0)) { 1.0 } else { 0.0 };
        var_guard1998 = assign85340_e130980;

        let (assign85350_e130986,) = {
    if ((var_guard1994 != 0.0) && (var_guard1998 != 0.0)) {
        (1.0,)
    } else {
        (s.v[2625],)
    }
};
        s.store_scalar(2625, assign85350_e130986);

        let (assign85360_e130992,) = {
    if ((var_guard1994 != 0.0) && (var_guard1998 != 0.0)) {
        (1.0,)
    } else {
        (var_flg_coovlps,)
    }
};
        var_flg_coovlps = assign85360_e130992;

        if ((s.v[2990] != 0.0) && (s.v[2994] != 0.0)) {
            s.store_sub(395, 731, 728);
            s.store_neg(396, 728);
            s.store_scalar(409, var_uc_novers);
            s.store_scalar(407, p.p66);
            s.store_scalar(411, 0.0);
            s.copy_ad(410, 687);
            s.store_scalar(413, var_cox0);
        }

        s.b[2995] = (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.store_scalar(2995, if s.b[2995] { 1.0 } else { 0.0 });

        let (assign85450_e131065,) = {
    if (((var_guard1995 != 0.0) && (var_guard1994 == 0.0)) && s.b[2995]) {
        (1.0,)
    } else {
        (s.v[2625],)
    }
};
        s.store_scalar(2625, assign85450_e131065);

        if (((s.v[2991] != 0.0) && (s.v[2990] == 0.0)) && s.b[2995]) {
            s.store_sub_ad_lhs(395, A::from_derivatives(var_vgsei, [var_vgsei_dn0, var_vgsei_dn1, var_vgsei_dn2, var_vgsei_dn3, var_vgsei_dn4, var_vgsei_dn5, var_vgsei_dn6, var_vgsei_dn7, var_vgsei_dn8, var_vgsei_dn9, var_vgsei_dn10, var_vgsei_dn11, var_vgsei_dn12, var_vgsei_dn13, var_vgsei_dn14, var_vgsei_dn15, var_vgsei_dn16, var_vgsei_dn17, var_vgsei_dn18], [var_vgsei_db0, var_vgsei_db1, var_vgsei_db2, var_vgsei_db3, var_vgsei_db4, var_vgsei_db5, var_vgsei_db6, var_vgsei_db7, var_vgsei_db8, var_vgsei_db9, var_vgsei_db10, var_vgsei_db11, var_vgsei_db12]), 735);
            s.store_neg(396, 735);
        }

        let assign85480_e131097: f64 = if (((p.p35 == 1.0) && (p.p63 > 0.0)) && (var_uc_nover > 0.0)) { 1.0 } else { 0.0 };
        var_guard2000 = assign85480_e131097;

        let (assign85490_e131108,) = {
    if (((var_guard1996 != 0.0) && (!((var_guard1994 != 0.0) || (var_guard1995 != 0.0)))) && (var_guard2000 != 0.0)) {
        (1.0,)
    } else {
        (s.v[2625],)
    }
};
        s.store_scalar(2625, assign85490_e131108);

        *var_flg_coovlps_slot = var_flg_coovlps;
        *var_guard1994_slot = var_guard1994;
        *var_guard1995_slot = var_guard1995;
        *var_guard1996_slot = var_guard1996;
        *var_guard1998_slot = var_guard1998;
        *var_guard2000_slot = var_guard2000;
    }

    pub(super) fn stamp_transient_block_88(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_coxb0: f64,
        var_guard1994: f64,
        var_guard1995: f64,
        var_guard1996: f64,
        var_guard2000: f64,
        var_uc_nover: f64,
        var_vdsei: f64,
        var_vdsei_db0: f64,
        var_vdsei_db1: f64,
        var_vdsei_db10: f64,
        var_vdsei_db11: f64,
        var_vdsei_db12: f64,
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
        var_vdsei_dn18: f64,
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
        var_vgsei_db12: f64,
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
        var_vgsei_dn18: f64,
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

        let (assign85500_e131119,) = {
    if (((var_guard1996 != 0.0) && (!((var_guard1994 != 0.0) || (var_guard1995 != 0.0)))) && (var_guard2000 != 0.0)) {
        (1.0,)
    } else {
        (var_flg_coovlp,)
    }
};
        var_flg_coovlp = assign85500_e131119;

        if (((s.v[2992] != 0.0) && (!((s.v[2990] != 0.0) || (s.v[2991] != 0.0)))) && (s.v[2996] != 0.0)) {
            s.store_sub(395, 731, 728);
            s.store_sub(396, 729, 728);
            s.store_scalar(409, var_uc_nover);
            s.store_scalar(407, (p.p63 + (p.p64 * p.p55)));
            s.copy_ad(411, 384);
            s.copy_ad(410, 686);
            s.store_scalar(413, var_coxb0);
            s.store_neg(407, 407);
        }

        s.b[2997] = (((s.v[407] < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0));
        s.store_scalar(2997, if s.b[2997] { 1.0 } else { 0.0 });

        if ((((s.v[2992] != 0.0) && (!((s.v[2990] != 0.0) || (s.v[2991] != 0.0)))) && (s.v[2996] != 0.0)) && s.b[2997]) {
            s.store_neg(407, 407);
            s.store_scalar(335, p.p63);
            s.store_offset_div_scaled_product(996, s.ad_value(335), s.ad_value(335), 1.0, s.ad_value(651), 1.0, (-p.p137));
        }

        s.b[2998] = (p.p113 > 0.0);
        s.store_scalar(2998, if s.b[2998] { 1.0 } else { 0.0 });

        s.b[2999] = ((s.v[396] == 0.0) || (p.p113 <= 0.0));
        s.store_scalar(2999, if s.b[2999] { 1.0 } else { 0.0 });

        if ((((((s.v[2992] != 0.0) && (!((s.v[2990] != 0.0) || (s.v[2991] != 0.0)))) && (s.v[2996] != 0.0)) && s.b[2997]) && s.b[2998]) && s.b[2999]) {
        }

        if ((((((s.v[2992] != 0.0) && (!((s.v[2990] != 0.0) || (s.v[2991] != 0.0)))) && (s.v[2996] != 0.0)) && s.b[2997]) && s.b[2998]) && (!s.b[2999])) {
            s.store_scalar(783, (if (s.v[396] < 0.0) { (-1.0) } else { 1.0 }));
        }

        if ((((((s.v[2992] != 0.0) && (!((s.v[2990] != 0.0) || (s.v[2991] != 0.0)))) && (s.v[2996] != 0.0)) && s.b[2997]) && s.b[2998]) && (!s.b[2999])) {
            s.store_mul(784, 783, 396);
            s.store_offset_powf_ad(781, A::div(s.ad_value(784), s.ad_value(996)), p.p113, 1.0);
            s.store_powf(782, 781, (1.0 / p.p113));
            s.store_div_scaled_product_indices(396, 783, 784, 1.0, 782, 1.0);
        }

        if (((((s.v[2992] != 0.0) && (!((s.v[2990] != 0.0) || (s.v[2991] != 0.0)))) && (s.v[2996] != 0.0)) && s.b[2997]) && s.b[2998]) {
            s.store_sqrt_offset_square_offset(782, 396, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(396), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(396), p.p137), 782, 0.5);
        }

        s.b[3000] = (s.v[336] < 0.0);
        s.store_scalar(3000, if s.b[3000] { 1.0 } else { 0.0 });

        if ((((((s.v[2992] != 0.0) && (!((s.v[2990] != 0.0) || (s.v[2991] != 0.0)))) && (s.v[2996] != 0.0)) && s.b[2997]) && s.b[2998]) && s.b[3000]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((((s.v[2992] != 0.0) && (!((s.v[2990] != 0.0) || (s.v[2991] != 0.0)))) && (s.v[2996] != 0.0)) && s.b[2997]) && s.b[2998]) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub(407, 407, 600);
        }

        s.b[3001] = (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));
        s.store_scalar(3001, if s.b[3001] { 1.0 } else { 0.0 });

        let (assign85800_e131590,) = {
    if ((s.b[2993] && (!(((var_guard1994 != 0.0) || (var_guard1995 != 0.0)) || (var_guard1996 != 0.0)))) && s.b[3001]) {
        (1.0,)
    } else {
        (s.v[2625],)
    }
};
        s.store_scalar(2625, assign85800_e131590);

        if ((s.b[2993] && (!(((s.v[2990] != 0.0) || (s.v[2991] != 0.0)) || (s.v[2992] != 0.0)))) && s.b[3001]) {
            s.store_sub_ad_lhs(395, A::from_derivatives(var_vgsei, [var_vgsei_dn0, var_vgsei_dn1, var_vgsei_dn2, var_vgsei_dn3, var_vgsei_dn4, var_vgsei_dn5, var_vgsei_dn6, var_vgsei_dn7, var_vgsei_dn8, var_vgsei_dn9, var_vgsei_dn10, var_vgsei_dn11, var_vgsei_dn12, var_vgsei_dn13, var_vgsei_dn14, var_vgsei_dn15, var_vgsei_dn16, var_vgsei_dn17, var_vgsei_dn18], [var_vgsei_db0, var_vgsei_db1, var_vgsei_db2, var_vgsei_db3, var_vgsei_db4, var_vgsei_db5, var_vgsei_db6, var_vgsei_db7, var_vgsei_db8, var_vgsei_db9, var_vgsei_db10, var_vgsei_db11, var_vgsei_db12]), 735);
            s.store_sub_ad_lhs(396, A::from_derivatives(var_vdsei, [var_vdsei_dn0, var_vdsei_dn1, var_vdsei_dn2, var_vdsei_dn3, var_vdsei_dn4, var_vdsei_dn5, var_vdsei_dn6, var_vdsei_dn7, var_vdsei_dn8, var_vdsei_dn9, var_vdsei_dn10, var_vdsei_dn11, var_vdsei_dn12, var_vdsei_dn13, var_vdsei_dn14, var_vdsei_dn15, var_vdsei_dn16, var_vdsei_dn17, var_vdsei_dn18], [var_vdsei_db0, var_vdsei_db1, var_vdsei_db2, var_vdsei_db3, var_vdsei_db4, var_vdsei_db5, var_vdsei_db6, var_vdsei_db7, var_vdsei_db8, var_vdsei_db9, var_vdsei_db10, var_vdsei_db11, var_vdsei_db12]), 735);
        }

        if (s.v[2625] != 0.0) {
            s.store_scalar(3009, 0.4);
        }

        let (assign85850_e131632,) = {
    if (s.v[2625] != 0.0) {
        (0.0,)
    } else {
        (s.v[3010],)
    }
};
        s.store_scalar(3010, assign85850_e131632);

        if (s.v[2625] != 0.0) {
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

        let (assign85980_e131685,) = {
    if (s.v[2625] != 0.0) {
        let assign85980_e131683: f64 = (-1.0);
        (assign85980_e131683,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign85980_e131685);

        if (s.v[2625] != 0.0) {
            s.store_scalar(3011, 0.0);
            s.store_scalar(3012, 0.0);
            s.store_mul_scaled_ln_ad_rhs(3007, 155, 2.0, A::div(s.ad_value(409), s.ad_value(394)));
            s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(3007), (-0.1));
            s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        }

        if (s.v[2625] != 0.0) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.v[2625] != 0.0) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(3008, 781, (-0.5), 782, (-0.5), 0.8);
        }

        s.b[3014] = (s.v[3009] > (s.v[3008] * 0.5));
        s.store_scalar(3014, if s.b[3014] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[3014]) {
            s.store_scale(3009, 3008, 0.5);
        }

        s.b[3015] = param_given[338];
        s.store_scalar(3015, if s.b[3015] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[3015]) {
            s.store_scalar(3008, p.p338);
        }

        s.b[3016] = param_given[339];
        s.store_scalar(3016, if s.b[3016] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[3016]) {
            s.store_scalar(3009, p.p339);
        }

        s.b[3017] = param_given[338];
        s.store_scalar(3017, if s.b[3017] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (!s.b[3016])) && s.b[3017]) {
            s.store_scale(3009, 3008, 0.5);
        }

        s.b[3018] = (s.v[3009] > (s.v[3008] * 0.5));
        s.store_scalar(3018, if s.b[3018] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[3018]) {
            s.store_scale(3009, 3008, 0.5);
        }

        s.b[3019] = (p.p38 == 1.0);
        s.store_scalar(3019, if s.b[3019] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[3019]) {
            s.store_neg(334, 396);
        }

        s.b[3020] = (s.v[334] > s.v[3009]);
        s.store_scalar(3020, if s.b[3020] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && s.b[3019]) && s.b[3020]) {
            s.store_sub(335, 334, 3009);
            s.store_sub(336, 3008, 3009);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 345, 1.0);
            s.store_neg(345, 345);
            s.store_add(344, 3009, 333);
        }

        if (((s.v[2625] != 0.0) && s.b[3019]) && (!s.b[3020])) {
            s.copy_ad(344, 334);
        }

        if ((s.v[2625] != 0.0) && s.b[3019]) {
            s.store_neg(397, 344);
        }

        if ((s.v[2625] != 0.0) && (!s.b[3019])) {
            s.copy_ad(397, 396);
        }

        if (s.v[2625] != 0.0) {
            s.store_div(212, 410, 413);
            s.store_square(213, 212);
            s.store_sub_from_scalar(402, s.v[458], 395);
        }

        let (assign86390_e132026,) = {
    if (s.v[2625] != 0.0) {
        let assign86390_e132020: f64 = (-s.v[397]);
        let assign86390_e132023: f64 = (10.0 * 2.220446049250313e-16);
        let assign86390_e132024: f64 = (assign86390_e132020 + assign86390_e132023);
        (assign86390_e132024,)
    } else {
        (s.v[403],)
    }
};
        s.store_scalar(403, assign86390_e132026);

        if (s.v[2625] != 0.0) {
            s.store_scalar(3003, 0.0);
            s.store_scale(3004, 409, 1.6021918e-19);
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
        }

        s.b[3021] = ((s.v[154] * (-s.v[397])) >= 500.0);
        s.store_scalar(3021, if s.b[3021] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[3021]) {
            s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(334, 1.403592217853e217);
        }

        if ((s.v[2625] != 0.0) && (!s.b[3021])) {
            s.store_mul_neg_rhs(781, 154, 397);
            s.store_scalar(229, 1.0);
        }

        let mut assign86490_loop_guard: usize = 0;
        while {
            let assign86490_cond_e132100: f64 = if (((s.v[2625] != 0.0) && (!s.b[3021])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign86490_cond_e132100 != 0.0
        } {
            assign86490_loop_guard += 1;
            assert!(assign86490_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2625] != 0.0) && (!s.b[3021])) {
                s.store_scale(229, 229, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if ((s.v[2625] != 0.0) && (!s.b[3021])) {
            s.store_mul_exp_rhs(229, 229, 781);
            s.copy_ad(334, 229);
        }

        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {
            s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));
            s.store_scalar(782, (4.0 * 0.5));
        }

        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(335, 781, 0.5, 782, 0.5, 0.5);
        }

        s.b[3022] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));
        s.store_scalar(3022, if s.b[3022] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) {
            s.store_add_scaled_inputs3_indices(781, 402, 1.0, 397, 1.0, 335, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 335);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign86640_e132274,) = {
    if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign86640_e132274);

        let (assign86650_e132282,) = {
    if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign86650_e132282);

        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) {
            s.store_scalar(770, 0.0);
        }

        *var_flg_coovlp_slot = var_flg_coovlp;
    }

    pub(super) fn stamp_transient_block_89(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) {
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3023] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.store_scalar(3023, if s.b[3023] { 1.0 } else { 0.0 });

        s.b[3024] = (1.0 == 1.0);
        s.store_scalar(3024, if s.b[3024] { 1.0 } else { 0.0 });

        let (assign86740_e132366,) = {
    if (((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) && s.b[3023]) && s.b[3024]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign86740_e132366);

        s.b[3025] = (1.0 == 2.0);
        s.store_scalar(3025, if s.b[3025] { 1.0 } else { 0.0 });

        let (assign86760_e132384,) = {
    if ((((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) && s.b[3023]) && (!s.b[3024])) && s.b[3025]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign86760_e132384);

        s.b[3026] = (1.0 == 4.0);
        s.store_scalar(3026, if s.b[3026] { 1.0 } else { 0.0 });

        let (assign86780_e132405,) = {
    if (((((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) && s.b[3023]) && (!s.b[3024])) && (!s.b[3025])) && s.b[3026]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign86780_e132405);

        s.b[3027] = (1.0 == 8.0);
        s.store_scalar(3027, if s.b[3027] { 1.0 } else { 0.0 });

        let (assign86800_e132429,) = {
    if ((((((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) && s.b[3023]) && (!s.b[3024])) && (!s.b[3025])) && (!s.b[3026])) && s.b[3027]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign86800_e132429);

        let (assign86810_e132439,) = {
    if ((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) && s.b[3023]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign86810_e132439);

        let mut assign86820_loop_guard: usize = 0;
        while {
            let assign86820_cond_e132450: f64 = if (((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) && s.b[3023]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign86820_cond_e132450 != 0.0
        } {
            assign86820_loop_guard += 1;
            assert!(assign86820_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) && s.b[3023]) {
                s.store_sqrt(726, 726);
            }
            let (assign86820_body1_e132473,) = {
    if ((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) && s.b[3023]) {
        let assign86820_body1_e132471: f64 = (s.v[719] + 1.0);
        (assign86820_body1_e132471,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign86820_body1_e132473);
        }

        if ((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) && (!s.b[3023])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);
        }

        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[3022]) {
        }

        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && (!s.b[3022])) {
            s.store_add(335, 402, 397);
            s.store_scalar(334, 1.0);
        }

        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {
            s.store_sub(397, 335, 402);
        }

        let (assign86920_e132590,) = {
    if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {
        let assign86920_e132584: f64 = (-s.v[397]);
        let assign86920_e132587: f64 = (10.0 * 2.220446049250313e-16);
        let assign86920_e132588: f64 = (assign86920_e132584 + assign86920_e132587);
        (assign86920_e132588,)
    } else {
        (s.v[403],)
    }
};
        s.store_scalar(403, assign86920_e132590);

        s.b[3028] = (s.v[402] < s.v[403]);
        s.store_scalar(3028, if s.b[3028] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[3028]) {
            s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));
            s.store_mul_add_rhs(332, 154, 402, 397);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(410));
            s.store_mul(333, 335, 413);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(278, (7.0 * 1.414213562373095), 333, A::offset(s.ad_value(332), (-2.0)), 9.0);
            s.store_square(276, 278);
        }

        s.b[3029] = (s.v[277] < (s.v[276] * 1e-8));
        s.store_scalar(3029, if s.b[3029] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && s.b[3028]) && s.b[3029]) {
            s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);
        }

        if (((s.v[2625] != 0.0) && s.b[3028]) && (!s.b[3029])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((s.v[2625] != 0.0) && s.b[3028]) {
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
            s.copy_ad(3011, 404);
        }

        s.b[3030] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));
        s.store_scalar(3030, if s.b[3030] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3030]) {
            s.store_add_scaled_product_indices(89, 402, 1.0, 213, 154, 0.5);
        }

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && (!s.b[3030])) {
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
        }

        if ((s.v[2625] != 0.0) && (!s.b[3028])) {
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        s.b[3031] = (s.v[116] >= 3.0);
        s.store_scalar(3031, if s.b[3031] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3031]) {
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
            s.store_mul_add_rhs(116, 154, 89, 397);
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && (!s.b[3031])) {
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

        s.b[3032] = (p.p33 > 0.0);
        s.store_scalar(3032, if s.b[3032] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) {
            s.store_offset_add(442, 402, 397, 0.1);
            s.store_mul(222, 405, 229);
            s.store_mul(443, 405, 229);
            s.store_mul(334, 156, 213);
            s.store_mul(444, 154, 442);
            s.store_add_scaled_inputs_product_mixed_aaii(447, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), 154, 397, 1.0);
        }

        s.b[3033] = (p.p33 == 2.0);
        s.store_scalar(3033, if s.b[3033] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3033]) {
            s.store_offset_sub(781, 444, 447, (-1.0));
            s.store_scale(782, 444, 4.0);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3033]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3033]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(447, 444, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && (!s.b[3033])) {
            if (s.v[447] <= s.v[444]) {
            } else {
                s.copy_ad(447, 444);
            }
        }

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) {
            if (s.v[447] >= 0.0) {
            } else {
                s.store_scalar(447, 0.0);
            }
        }

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) {
            s.store_sub(444, 444, 447);
            s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);
            s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));
            s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);
        }

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) {
            s.copy_ad(445, 116);
        }

        s.b[3034] = (p.p33 == 2.0);
        s.store_scalar(3034, if s.b[3034] { 1.0 } else { 0.0 });

        s.b[3035] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));
        s.store_scalar(3035, if s.b[3035] { 1.0 } else { 0.0 });

        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) {
            s.store_add_scaled_inputs3_indices(781, 445, 1.0, 446, (-1.0), 446, 0.2);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 446, 446, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign87750_e133736,) = {
    if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign87750_e133736);

        let (assign87760_e133749,) = {
    if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign87760_e133749);

        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
        }

    }

    pub(super) fn stamp_transient_block_90(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) {
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3036] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(3036, if s.b[3036] { 1.0 } else { 0.0 });

        s.b[3037] = (2.0 == 1.0);
        s.store_scalar(3037, if s.b[3037] { 1.0 } else { 0.0 });

        let (assign87870_e133898,) = {
    if (((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) && s.b[3036]) && s.b[3037]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign87870_e133898);

        s.b[3038] = (2.0 == 2.0);
        s.store_scalar(3038, if s.b[3038] { 1.0 } else { 0.0 });

        let (assign87890_e133921,) = {
    if ((((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) && s.b[3036]) && (!s.b[3037])) && s.b[3038]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign87890_e133921);

        s.b[3039] = (2.0 == 4.0);
        s.store_scalar(3039, if s.b[3039] { 1.0 } else { 0.0 });

        let (assign87910_e133947,) = {
    if (((((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) && s.b[3036]) && (!s.b[3037])) && (!s.b[3038])) && s.b[3039]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign87910_e133947);

        s.b[3040] = (2.0 == 8.0);
        s.store_scalar(3040, if s.b[3040] { 1.0 } else { 0.0 });

        let (assign87930_e133976,) = {
    if ((((((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) && s.b[3036]) && (!s.b[3037])) && (!s.b[3038])) && (!s.b[3039])) && s.b[3040]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign87930_e133976);

        let (assign87940_e133991,) = {
    if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) && s.b[3036]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign87940_e133991);

        let mut assign87950_loop_guard: usize = 0;
        while {
            let assign87950_cond_e134007: f64 = if (((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) && s.b[3036]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign87950_cond_e134007 != 0.0
        } {
            assign87950_loop_guard += 1;
            assert!(assign87950_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) && s.b[3036]) {
                s.store_sqrt(726, 726);
            }
            let (assign87950_body1_e134040,) = {
    if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) && s.b[3036]) {
        let assign87950_body1_e134038: f64 = (s.v[719] + 1.0);
        (assign87950_body1_e134038,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign87950_body1_e134040);
        }

        if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) && (!s.b[3036])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);
            s.store_div_scaled_product3_indices(335, 446, 725, 726, 0.2, 770, 1.0);
            s.store_add_scaled_inputs3_indices(116, 446, 1.0, 446, (-0.2), 780, 1.0);
        }

        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && s.b[3035]) {
        }

        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && s.b[3034]) && (!s.b[3035])) {
            s.copy_ad(116, 445);
            s.store_scalar(335, 1.0);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3032]) && (!s.b[3034])) {
            if (s.v[445] <= s.v[446]) {
                s.copy_ad(116, 445);
            } else {
                s.copy_ad(116, 446);
            }
        }

        s.b[3041] = (p.p33 == 1.0);
        s.store_scalar(3041, if s.b[3041] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[3042] = (s.v[411] > 0.0);
        s.store_scalar(3042, if s.b[3042] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) && s.b[3042]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) && (!s.b[3042])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[3043] = (s.v[336] < 0.0);
        s.store_scalar(3043, if s.b[3043] { 1.0 } else { 0.0 });

        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) && (!s.b[3042])) && s.b[3043]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) && (!s.b[3042])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[3044] = (s.v[336] < 0.0);
        s.store_scalar(3044, if s.b[3044] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) && s.b[3044]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 3004, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(419, 335, 418);
            s.store_add(414, 404, 397);
            s.store_mul_sub_rhs(333, 419, 414, 418);
        }

        s.b[3045] = (s.v[333] < 60.0);
        s.store_scalar(3045, if s.b[3045] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) && s.b[3045]) {
            s.store_exp(335, 333);
            s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
            s.store_sub(336, 335, 334);
            s.store_div_ln_offset_lhs(416, 336, 1.0, 419);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) && (!s.b[3045])) {
            s.store_sub(416, 414, 418);
        }

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) {
            s.store_mul(415, 154, 416);
        }

        s.b[3046] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));
        s.store_scalar(3046, if s.b[3046] { 1.0 } else { 0.0 });

        let (assign88380_e134629,) = {
    if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) && s.b[3046]) {
        let assign88380_e134627: f64 = (s.v[3010] + 1.0);
        (assign88380_e134627,)
    } else {
        (s.v[3010],)
    }
};
        s.store_scalar(3010, assign88380_e134629);

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3041]) && s.b[3046]) {
            s.copy_ad(116, 447);
        }

        if ((s.v[2625] != 0.0) && (!s.b[3028])) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[3047] = (((s.v[116]) as f64).abs() > 1e-6);
        s.store_scalar(3047, if s.b[3047] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3047]) {
            s.store_add_offset_lhs_ad_rhs(335, 116, (-1.0), A::exp_scaled_input(s.ad_value(116), -1.0));
            s.store_sqrt(336, 335);
        }

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && (!s.b[3047])) {
            s.store_mul_scaled_sqrt_ad_rhs(336, 116, 0.7071067811865475, A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333)));
        }

        if ((s.v[2625] != 0.0) && (!s.b[3028])) {
            s.store_mul(354, 410, 336);
            s.store_mul_sub_rhs(398, 413, 402, 404);
            s.store_div(3048, 354, 3004);
        }

        s.b[3050] = (p.p33 == 2.0);
        s.store_scalar(3050, if s.b[3050] { 1.0 } else { 0.0 });

        s.b[3051] = ((s.v[3048] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));
        s.store_scalar(3051, if s.b[3051] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) {
            s.store_add_scaled_inputs3_indices(781, 3048, 1.0, 386, (-1.0), 386, 0.1);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 386, 386, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign88560_e134836,) = {
    if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign88560_e134836);

        let (assign88570_e134847,) = {
    if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign88570_e134847);

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3052] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(3052, if s.b[3052] { 1.0 } else { 0.0 });

        s.b[3053] = (2.0 == 1.0);
        s.store_scalar(3053, if s.b[3053] { 1.0 } else { 0.0 });

        let (assign88680_e134978,) = {
    if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) && s.b[3052]) && s.b[3053]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign88680_e134978);

        s.b[3054] = (2.0 == 2.0);
        s.store_scalar(3054, if s.b[3054] { 1.0 } else { 0.0 });

        let (assign88700_e134999,) = {
    if (((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) && s.b[3052]) && (!s.b[3053])) && s.b[3054]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign88700_e134999);

        s.b[3055] = (2.0 == 4.0);
        s.store_scalar(3055, if s.b[3055] { 1.0 } else { 0.0 });

        let (assign88720_e135023,) = {
    if ((((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) && s.b[3052]) && (!s.b[3053])) && (!s.b[3054])) && s.b[3055]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign88720_e135023);

        s.b[3056] = (2.0 == 8.0);
        s.store_scalar(3056, if s.b[3056] { 1.0 } else { 0.0 });

        let (assign88740_e135050,) = {
    if (((((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) && s.b[3052]) && (!s.b[3053])) && (!s.b[3054])) && (!s.b[3055])) && s.b[3056]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign88740_e135050);

        let (assign88750_e135063,) = {
    if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) && s.b[3052]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign88750_e135063);

        let mut assign88760_loop_guard: usize = 0;
        while {
            let assign88760_cond_e135077: f64 = if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) && s.b[3052]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign88760_cond_e135077 != 0.0
        } {
            assign88760_loop_guard += 1;
            assert!(assign88760_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) && s.b[3052]) {
                s.store_sqrt(726, 726);
            }
            let (assign88760_body1_e135106,) = {
    if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) && s.b[3052]) {
        let assign88760_body1_e135104: f64 = (s.v[719] + 1.0);
        (assign88760_body1_e135104,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign88760_body1_e135106);
        }

        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) && (!s.b[3052])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);
            s.store_div_scaled_product3_indices(334, 386, 725, 726, 0.1, 770, 1.0);
            s.store_add_scaled_inputs3_indices(335, 386, 1.0, 386, (-0.1), 780, 1.0);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3051]) {
        }

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && (!s.b[3051])) {
            s.copy_ad(335, 3048);
        }

    }

    pub(super) fn stamp_transient_block_91(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && (!s.b[3051])) {
            s.store_scalar(334, 1.0);
        }

        s.b[3057] = (s.v[334] < 1.0);
        s.store_scalar(3057, if s.b[3057] { 1.0 } else { 0.0 });

        let (assign88860_e135248,) = {
    if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3050]) && s.b[3057]) {
        let assign88860_e135246: f64 = (s.v[3010] + 2.0);
        (assign88860_e135246,)
    } else {
        (s.v[3010],)
    }
};
        s.store_scalar(3010, assign88860_e135248);

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && (!s.b[3050])) {
            if (s.v[3048] <= s.v[386]) {
                s.copy_ad(335, 3048);
            } else {
                s.copy_ad(335, 386);
            }
        }

        s.b[3058] = (s.v[3048] >= s.v[386]);
        s.store_scalar(3058, if s.b[3058] { 1.0 } else { 0.0 });

        let (assign88890_e135280,) = {
    if ((((s.v[2625] != 0.0) && (!s.b[3028])) && (!s.b[3050])) && s.b[3058]) {
        let assign88890_e135278: f64 = (s.v[3010] + 2.0);
        (assign88890_e135278,)
    } else {
        (s.v[3010],)
    }
};
        s.store_scalar(3010, assign88890_e135280);

        s.b[3059] = (s.v[3010] >= 2.0);
        s.store_scalar(3059, if s.b[3059] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) {
            s.copy_ad(3049, 404);
            s.store_mul(354, 335, 3004);
            s.store_sub_div_rhs_indices(404, 402, 354, 413);
        }

        s.b[3060] = (p.p33 == 2.0);
        s.store_scalar(3060, if s.b[3060] { 1.0 } else { 0.0 });

        s.b[3061] = ((s.v[404] > (s.v[3049] - 0.1)) && (0.1 >= 0.0));
        s.store_scalar(3061, if s.b[3061] { 1.0 } else { 0.0 });

        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && s.b[3061]) {
            s.store_offset_sub(781, 404, 3049, 0.1);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign89010_e135414,) = {
    if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && s.b[3061]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign89010_e135414);

        let (assign89020_e135427,) = {
    if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && s.b[3061]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign89020_e135427);

        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && s.b[3061]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3062] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(3062, if s.b[3062] { 1.0 } else { 0.0 });

        s.b[3063] = (2.0 == 1.0);
        s.store_scalar(3063, if s.b[3063] { 1.0 } else { 0.0 });

        let (assign89130_e135576,) = {
    if (((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && s.b[3061]) && s.b[3062]) && s.b[3063]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign89130_e135576);

        s.b[3064] = (2.0 == 2.0);
        s.store_scalar(3064, if s.b[3064] { 1.0 } else { 0.0 });

        let (assign89150_e135599,) = {
    if ((((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && s.b[3061]) && s.b[3062]) && (!s.b[3063])) && s.b[3064]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign89150_e135599);

        s.b[3065] = (2.0 == 4.0);
        s.store_scalar(3065, if s.b[3065] { 1.0 } else { 0.0 });

        let (assign89170_e135625,) = {
    if (((((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && s.b[3061]) && s.b[3062]) && (!s.b[3063])) && (!s.b[3064])) && s.b[3065]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign89170_e135625);

        s.b[3066] = (2.0 == 8.0);
        s.store_scalar(3066, if s.b[3066] { 1.0 } else { 0.0 });

        let (assign89190_e135654,) = {
    if ((((((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && s.b[3061]) && s.b[3062]) && (!s.b[3063])) && (!s.b[3064])) && (!s.b[3065])) && s.b[3066]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign89190_e135654);

        let (assign89200_e135669,) = {
    if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && s.b[3061]) && s.b[3062]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign89200_e135669);

        let mut assign89210_loop_guard: usize = 0;
        while {
            let assign89210_cond_e135685: f64 = if (((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && s.b[3061]) && s.b[3062]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign89210_cond_e135685 != 0.0
        } {
            assign89210_loop_guard += 1;
            assert!(assign89210_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && s.b[3061]) && s.b[3062]) {
                s.store_sqrt(726, 726);
            }
            let (assign89210_body1_e135718,) = {
    if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && s.b[3061]) && s.b[3062]) {
        let assign89210_body1_e135716: f64 = (s.v[719] + 1.0);
        (assign89210_body1_e135716,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign89210_body1_e135718);
        }

        if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && s.b[3061]) && (!s.b[3062])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && s.b[3061]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_add_offset_lhs(404, 3049, (-0.1), 780);
        }

        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && s.b[3061]) {
        }

        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && (!s.b[3061])) {
        }

        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && s.b[3060]) && (!s.b[3061])) {
            s.store_scalar(334, 1.0);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3059]) && (!s.b[3060])) {
            if (s.v[404] <= s.v[3049]) {
            } else {
                s.copy_ad(404, 3049);
            }
        }

        if ((s.v[2625] != 0.0) && (!s.b[3028])) {
            s.copy_ad(3011, 404);
        }

        s.b[3067] = (p.p33 == 1.0);
        s.store_scalar(3067, if s.b[3067] { 1.0 } else { 0.0 });

        let (assign89330_e135890,) = {
    if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign89330_e135890);

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) {
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(3004)), s.ad_value(155)), 2.0);
        }

        s.b[3068] = (s.v[411] > 0.0);
        s.store_scalar(3068, if s.b[3068] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && s.b[3068]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3068])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[3069] = (s.v[336] < 0.0);
        s.store_scalar(3069, if s.b[3069] { 1.0 } else { 0.0 });

        if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3068])) && s.b[3069]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3068])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[3070] = (s.v[336] < 0.0);
        s.store_scalar(3070, if s.b[3070] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && s.b[3070]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 3004, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(419, 335, 418);
        }

        let (assign89560_e136199,) = {
    if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, assign89560_e136199);

    }

    pub(super) fn stamp_transient_block_92(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign89570_loop_guard: usize = 0;
        while {
            let assign89570_cond_e136209: f64 = (s.v[421] + 1.0);
            let assign89570_cond_e136211: f64 = if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (s.v[97] <= assign89570_cond_e136209)) { 1.0 } else { 0.0 };
            assign89570_cond_e136211 != 0.0
        } {
            assign89570_loop_guard += 1;
            assert!(assign89570_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[3072] = (s.v[333] < 60.0);
            s.store_scalar(3072, if s.b[3072] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && s.b[3072]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ln_offset_lhs(416, 336, 1.0, 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3072])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) {
                s.store_mul(415, 154, 416);
            }
            s.b[3073] = (s.v[116] < 0.0);
            s.store_scalar(3073, if s.b[3073] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && s.b[3073]) {
                s.store_scalar(334, (-0.7071067811865475));
                s.store_mul(223, 116, 334);
                s.store_mul(420, 154, 334);
            }
            s.b[3074] = (s.v[116] < 1e-6);
            s.store_scalar(3074, if s.b[3074] { 1.0 } else { 0.0 });
            if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3073])) && s.b[3074]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(338, 334, 336);
            }
            s.b[3075] = (s.v[338] > 0.0);
            s.store_scalar(3075, if s.b[3075] { 1.0 } else { 0.0 });
            if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3073])) && s.b[3074]) && s.b[3075]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);
            }
            if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3073])) && s.b[3074]) && (!s.b[3075])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3073])) && (!s.b[3074])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(338, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
            }
            s.b[3076] = (s.v[338] > 0.0);
            s.store_scalar(3076, if s.b[3076] { 1.0 } else { 0.0 });
            if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3073])) && (!s.b[3074])) && s.b[3076]) {
                s.store_sqrt(223, 338);
                s.store_div_scaled_product_right_ad(420, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335))), 0.5, 223, 1.0);
            }
            if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3073])) && (!s.b[3074])) && (!s.b[3076])) {
                s.store_scalar(223, 0.0);
                s.store_scalar(420, 0.0);
            }
            s.b[3077] = (s.v[116] < 0.0);
            s.store_scalar(3077, if s.b[3077] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && s.b[3077]) {
                s.store_scalar(214, 0.0);
                s.store_scalar(215, 0.0);
                s.store_neg(216, 223);
                s.store_neg(217, 420);
            }
            s.b[3078] = (s.v[116] < 60.0);
            s.store_scalar(3078, if s.b[3078] { 1.0 } else { 0.0 });
            s.b[3079] = (s.v[116] < 5e-5);
            s.store_scalar(3079, if s.b[3079] { 1.0 } else { 0.0 });
            if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3077])) && s.b[3078]) && s.b[3079]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            if ((((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3077])) && s.b[3078]) && (!s.b[3079])) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3077])) && (!s.b[3078])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[3080] = (s.v[214] > 0.0);
            s.store_scalar(3080, if s.b[3080] { 1.0 } else { 0.0 });
            if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3077])) && s.b[3080]) {
                s.store_sqrt_square_add(216, 223, 214);
                s.store_div_scaled_add_product(217, s.ad_value(215), 0.5, s.ad_value(420), s.ad_value(223), (2.0 * 0.5), s.ad_value(216), 1.0);
            }
            if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3077])) && (!s.b[3080])) {
                s.copy_ad(216, 223);
                s.copy_ad(217, 420);
            }
            if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[3081] = (s.v[79] == 1.0);
            s.store_scalar(3081, if s.b[3081] { 1.0 } else { 0.0 });
            let (assign89570_body72_e137357,) = {
    if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && s.b[3081]) {
        let assign89570_body72_e137355: f64 = (s.v[421] + 1.0);
        (assign89570_body72_e137355,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign89570_body72_e137357);
            if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3081])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3081])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[3082] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.store_scalar(3082, if s.b[3082] { 1.0 } else { 0.0 });
            if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3081])) && s.b[3082]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3081])) {
                s.store_add(404, 404, 236);
            }
            s.b[3083] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.store_scalar(3083, if s.b[3083] { 1.0 } else { 0.0 });
            let (assign89570_body79_e137460,) = {
    if (((((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) && (!s.b[3081])) && s.b[3083]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, assign89570_body79_e137460);
            let (assign89570_body80_e137471,) = {
    if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) {
        let assign89570_body80_e137469: f64 = (s.v[97] + 1.0);
        (assign89570_body80_e137469,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign89570_body80_e137471);
        }

        if (((s.v[2625] != 0.0) && (!s.b[3028])) && s.b[3067]) {
            s.store_mul(3002, 982, 223);
            s.store_mul(3003, 3004, 3002);
            s.store_offset_div(100, 3003, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        s.b[3085] = (p.p33 == 4.0);
        s.store_scalar(3085, if s.b[3085] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[3085]) {
            s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
            s.store_mul(222, 405, 229);
            s.copy_ad(404, 3011);
        }

        let (assign89720_e137608,) = {
    if ((s.v[2625] != 0.0) && s.b[3085]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign89720_e137608);

        if ((s.v[2625] != 0.0) && s.b[3085]) {
            s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(3004)), s.ad_value(155)), 2.0);
        }

        s.b[3086] = (s.v[411] > 0.0);
        s.store_scalar(3086, if s.b[3086] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && s.b[3085]) && s.b[3086]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if (((s.v[2625] != 0.0) && s.b[3085]) && (!s.b[3086])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[3087] = (s.v[336] < 0.0);
        s.store_scalar(3087, if s.b[3087] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && s.b[3085]) && (!s.b[3086])) && s.b[3087]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2625] != 0.0) && s.b[3085]) && (!s.b[3086])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if ((s.v[2625] != 0.0) && s.b[3085]) {
            s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));
            s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.b[3088] = (s.v[336] < 0.0);
        s.store_scalar(3088, if s.b[3088] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && s.b[3085]) && s.b[3088]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((s.v[2625] != 0.0) && s.b[3085]) {
            s.copy_ad(386, 336);
            s.store_mul3_affine_lhs(418, 3004, 386, (0.5 * 9662367879.197212), 0.0, 386);
            s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);
            s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(419, 335, 418);
        }

        let (assign89950_e137857,) = {
    if ((s.v[2625] != 0.0) && s.b[3085]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, assign89950_e137857);

    }

    pub(super) fn stamp_transient_block_93(
        s: &mut Scratch,
        p: &Parameters,
        var_coxb0: f64,
        var_uc_nover: f64,
        var_weffcv_nf: f64,
    ) {
        let mut assign89960_loop_guard: usize = 0;
        while {
            let assign89960_cond_e137864: f64 = (s.v[421] + 1.0);
            let assign89960_cond_e137866: f64 = if (((s.v[2625] != 0.0) && s.b[3085]) && (s.v[97] <= assign89960_cond_e137864)) { 1.0 } else { 0.0 };
            assign89960_cond_e137866 != 0.0
        } {
            assign89960_loop_guard += 1;
            assert!(assign89960_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2625] != 0.0) && s.b[3085]) {
                s.store_add(414, 404, 397);
                s.store_mul(116, 154, 414);
                s.store_mul_sub_rhs(333, 419, 414, 418);
            }
            s.b[3090] = (s.v[333] < 60.0);
            s.store_scalar(3090, if s.b[3090] { 1.0 } else { 0.0 });
            if (((s.v[2625] != 0.0) && s.b[3085]) && s.b[3090]) {
                s.store_exp(335, 333);
                s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);
                s.store_sub(336, 335, 334);
                s.store_div_ln_offset_lhs(416, 336, 1.0, 419);
                s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);
            }
            if (((s.v[2625] != 0.0) && s.b[3085]) && (!s.b[3090])) {
                s.store_sub(416, 414, 418);
                s.store_scalar(417, 1.0);
            }
            if ((s.v[2625] != 0.0) && s.b[3085]) {
                s.store_mul(415, 154, 416);
            }
            s.b[3091] = (((s.v[116]) as f64).abs() < 1e-6);
            s.store_scalar(3091, if s.b[3091] { 1.0 } else { 0.0 });
            if (((s.v[2625] != 0.0) && s.b[3085]) && s.b[3091]) {
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(335, 116, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));
                s.store_mul_sub_from_scalar_ad_rhs(337, 415, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)));
                s.store_sub(3012, 334, 336);
                s.store_mul_add_scaled_product_rhs(3013, 154, s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0));
            }
            if (((s.v[2625] != 0.0) && s.b[3085]) && (!s.b[3091])) {
                s.store_exp_neg_input(334, 116);
                s.store_exp_neg_input(335, 415);
                s.store_add_scaled_inputs4_indices(3012, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));
                s.store_mul_sub_ad_rhs(3013, 154, A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335)));
            }
            s.b[3092] = (((s.v[116]) as f64).abs() < 5e-5);
            s.store_scalar(3092, if s.b[3092] { 1.0 } else { 0.0 });
            if (((s.v[2625] != 0.0) && s.b[3085]) && s.b[3092]) {
                s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));
                s.store_mul_offset_ad_rhs(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0);
                s.store_mul(214, 222, 334);
                s.store_mul3_lhs(215, 222, 335, 154);
            }
            s.b[3093] = (((s.v[116]) as f64).abs() < 60.0);
            s.store_scalar(3093, if s.b[3093] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && s.b[3085]) && (!s.b[3092])) && s.b[3093]) {
                s.store_exp(227, 116);
                s.store_offset(335, 227, (-1.0));
                s.store_mul_sub_rhs(214, 222, 335, 116);
                s.store_mul3_lhs(215, 222, 154, 335);
            }
            if ((((s.v[2625] != 0.0) && s.b[3085]) && (!s.b[3092])) && (!s.b[3093])) {
                s.store_exp_mul(231, 154, 404);
                s.store_mul_ad_rhs(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));
                s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.b[3094] = (s.v[214] > 0.0);
            s.store_scalar(3094, if s.b[3094] { 1.0 } else { 0.0 });
            if (((s.v[2625] != 0.0) && s.b[3085]) && s.b[3094]) {
                s.store_sqrt_add(216, 3012, 214);
                s.store_div_scaled_inputs2_indices(217, 3013, 0.5, 215, 0.5, 216, 1.0);
            }
            s.b[3095] = (s.v[3012] > 0.0);
            s.store_scalar(3095, if s.b[3095] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && s.b[3085]) && (!s.b[3094])) && s.b[3095]) {
                s.store_sqrt(216, 3012);
                s.store_div_scaled_inputs_indices(217, 3013, 0.5, 216, 1.0);
            }
            if ((((s.v[2625] != 0.0) && s.b[3085]) && (!s.b[3094])) && (!s.b[3095])) {
                s.store_scalar(216, 0.0);
                s.store_scalar(217, 0.0);
            }
            if ((s.v[2625] != 0.0) && s.b[3085]) {
                s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2625] != 0.0) && s.b[3085]) {
                s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2625] != 0.0) && s.b[3085]) {
                s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);
                s.store_offset_mul(233, 212, 217, 1.0);
            }
            s.b[3096] = (s.v[79] > 0.0);
            s.store_scalar(3096, if s.b[3096] { 1.0 } else { 0.0 });
            let (assign89960_body56_e138606,) = {
    if (((s.v[2625] != 0.0) && s.b[3085]) && s.b[3096]) {
        let assign89960_body56_e138604: f64 = (s.v[421] + 1.0);
        (assign89960_body56_e138604,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign89960_body56_e138606);
            if (((s.v[2625] != 0.0) && s.b[3085]) && (!s.b[3096])) {
                s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);
            }
            if (((s.v[2625] != 0.0) && s.b[3085]) && (!s.b[3096])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[3097] = (((s.v[236]) as f64).abs() > s.v[93]);
            s.store_scalar(3097, if s.b[3097] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && s.b[3085]) && (!s.b[3096])) && s.b[3097]) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.v[2625] != 0.0) && s.b[3085]) && (!s.b[3096])) {
                s.store_add(404, 404, 236);
            }
            s.b[3098] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));
            s.store_scalar(3098, if s.b[3098] { 1.0 } else { 0.0 });
            let (assign89960_body63_e138696,) = {
    if ((((s.v[2625] != 0.0) && s.b[3085]) && (!s.b[3096])) && s.b[3098]) {
        let assign89960_body63_e138694: f64 = (s.v[79] + 2.0);
        (assign89960_body63_e138694,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, assign89960_body63_e138696);
            let (assign89960_body64_e138704,) = {
    if ((s.v[2625] != 0.0) && s.b[3085]) {
        let assign89960_body64_e138702: f64 = (s.v[97] + 1.0);
        (assign89960_body64_e138702,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, assign89960_body64_e138704);
        }

        if ((s.v[2625] != 0.0) && s.b[3085]) {
            if (s.v[3012] >= 0.0) {
                s.store_scaled_sqrt(223, 3012, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            } else {
                s.store_scalar(223, 0.0);
            }
        }

        if ((s.v[2625] != 0.0) && s.b[3085]) {
            s.store_mul(3002, 982, 223);
            s.store_mul(3003, 3004, 3002);
            s.store_offset_div(100, 3003, 410, (10.0 * 2.220446049250313e-16));
            s.store_mul(354, 410, 100);
            s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));
            s.store_mul3_lhs(399, 410, 214, 335);
            s.store_add(398, 354, 399);
        }

        if (s.v[2625] != 0.0) {
            s.store_sub(399, 398, 354);
        }

        s.b[3100] = (s.v[407] < 0.0);
        s.store_scalar(3100, if s.b[3100] { 1.0 } else { 0.0 });

        if ((s.v[2625] != 0.0) && s.b[3100]) {
            s.store_neg(407, 407);
        }

        s.b[3101] = (p.p55 == 0.0);
        s.store_scalar(3101, if s.b[3101] { 1.0 } else { 0.0 });

        s.b[3102] = (p.p50 == 0.0);
        s.store_scalar(3102, if s.b[3102] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && s.b[3100]) && s.b[3101]) && s.b[3102]) {
            s.store_neg(3005, 404);
        }

        if ((((s.v[2625] != 0.0) && s.b[3100]) && s.b[3101]) && (!s.b[3102])) {
            s.copy_ad(3005, 396);
        }

        if (((s.v[2625] != 0.0) && s.b[3100]) && s.b[3101]) {
            s.store_sqrt_offset_square_offset(782, 3005, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(3005), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(3005), p.p137), 782, 0.5);
        }

        s.b[3103] = (s.v[336] < 0.0);
        s.store_scalar(3103, if s.b[3103] { 1.0 } else { 0.0 });

        if ((((s.v[2625] != 0.0) && s.b[3100]) && s.b[3101]) && s.b[3103]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2625] != 0.0) && s.b[3100]) && s.b[3101]) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
            s.store_add_scaled_inputs3_indices(781, 407, 1.0, 600, (-1.0), 407, (-0.1));
            s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));
        }

        if (((s.v[2625] != 0.0) && s.b[3100]) && s.b[3101]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.v[2625] != 0.0) && s.b[3100]) && s.b[3101]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(603, 407, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_sub(407, 407, 603);
        }

        s.b[3104] = (4.0 == 1.0);
        s.store_scalar(3104, if s.b[3104] { 1.0 } else { 0.0 });

        s.b[3105] = (4.0 == 2.0);
        s.store_scalar(3105, if s.b[3105] { 1.0 } else { 0.0 });

        s.b[3106] = (4.0 == 3.0);
        s.store_scalar(3106, if s.b[3106] { 1.0 } else { 0.0 });

        s.b[3107] = (4.0 == 4.0);
        s.store_scalar(3107, if s.b[3107] { 1.0 } else { 0.0 });

        s.b[3108] = (p.p55 == 1.0);
        s.store_scalar(3108, if s.b[3108] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && s.b[3104]) && s.b[3108]) {
            s.store_scale(338, 407, var_weffcv_nf);
        }

        if (((s.v[2625] != 0.0) && s.b[3104]) && (!s.b[3108])) {
            s.store_scale(338, 407, (var_weffcv_nf * (1.0 - s.v[526])));
        }

        if ((s.v[2625] != 0.0) && s.b[3104]) {
            s.store_mul(353, 338, 398);
            s.store_mul(356, 338, 354);
        }

        if ((s.v[2625] != 0.0) && (s.b[3105] && (!s.b[3104]))) {
            s.store_scale(338, 407, (var_weffcv_nf * s.v[526]));
            s.store_mul(351, 338, 398);
            s.store_mul(359, 338, 354);
        }

        s.b[3109] = (p.p55 == 1.0);
        s.store_scalar(3109, if s.b[3109] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (s.b[3106] && (!(s.b[3104] || s.b[3105])))) && s.b[3109]) {
            s.store_scale(338, 407, var_weffcv_nf);
        }

        if (((s.v[2625] != 0.0) && (s.b[3106] && (!(s.b[3104] || s.b[3105])))) && (!s.b[3109])) {
            s.store_scale(338, 407, (var_weffcv_nf * (1.0 - s.v[526])));
        }

        if ((s.v[2625] != 0.0) && (s.b[3106] && (!(s.b[3104] || s.b[3105])))) {
            s.copy_ad(697, 404);
        }

        s.b[3110] = (p.p430 == 0.0);
        s.store_scalar(3110, if s.b[3110] { 1.0 } else { 0.0 });

        if (((s.v[2625] != 0.0) && (s.b[3106] && (!(s.b[3104] || s.b[3105])))) && s.b[3110]) {
            s.copy_ad(698, 354);
        }

        if ((s.v[2625] != 0.0) && (s.b[3106] && (!(s.b[3104] || s.b[3105])))) {
            s.store_mul(352, 338, 398);
            s.store_mul(355, 338, 354);
            s.copy_ad(816, 355);
        }

        if ((s.v[2625] != 0.0) && (s.b[3107] && (!((s.b[3104] || s.b[3105]) || s.b[3106])))) {
            s.store_scale(338, 407, (var_weffcv_nf * s.v[526]));
            s.store_mul(350, 338, 398);
            s.store_mul(358, 338, 354);
        }

        s.b[3111] = (p.p430 > 0.0);
        s.store_scalar(3111, if s.b[3111] { 1.0 } else { 0.0 });

        let (assign90540_e139274,) = {
    if s.b[3111] {
        (1.0,)
    } else {
        (s.v[406],)
    }
};
        s.store_scalar(406, assign90540_e139274);

        s.b[3112] = (((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0));
        s.store_scalar(3112, if s.b[3112] { 1.0 } else { 0.0 });

        if (s.b[3111] && s.b[3112]) {
            s.store_sub(395, 731, 728);
            s.store_sub(396, 729, 728);
            s.store_scalar(409, var_uc_nover);
            s.store_scalar(407, 0.0);
            s.copy_ad(411, 384);
            s.copy_ad(410, 686);
            s.store_scalar(413, var_coxb0);
            s.store_scalar(3120, 0.4);
        }

        let (assign90650_e139349,) = {
    if (s.b[3111] && s.b[3112]) {
        (0.0,)
    } else {
        (s.v[3121],)
    }
};
        s.store_scalar(3121, assign90650_e139349);

        if (s.b[3111] && s.b[3112]) {
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
        if (s.b[3111] && s.b[3112]) {
            s.store_scalar(446, 0.0);
        }

        let (assign90780_e139428,) = {
    if (s.b[3111] && s.b[3112]) {
        let assign90780_e139426: f64 = (-1.0);
        (assign90780_e139426,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, assign90780_e139428);

        if (s.b[3111] && s.b[3112]) {
            s.store_scalar(3122, 0.0);
            s.store_scalar(3123, 0.0);
            s.store_mul_scaled_ln_ad_rhs(3118, 155, 2.0, A::div(s.ad_value(409), s.ad_value(394)));
            s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(3118), (-0.1));
            s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        }

        if (s.b[3111] && s.b[3112]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.b[3111] && s.b[3112]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(3119, 781, (-0.5), 782, (-0.5), 0.8);
        }

        s.b[3125] = (s.v[3120] > (s.v[3119] * 0.5));
        s.store_scalar(3125, if s.b[3125] { 1.0 } else { 0.0 });

        if ((s.b[3111] && s.b[3112]) && s.b[3125]) {
            s.store_scale(3120, 3119, 0.5);
        }

        s.b[3126] = param_given[338];
        s.store_scalar(3126, if s.b[3126] { 1.0 } else { 0.0 });

        if ((s.b[3111] && s.b[3112]) && s.b[3126]) {
            s.store_scalar(3119, p.p338);
        }

        s.b[3127] = param_given[339];
        s.store_scalar(3127, if s.b[3127] { 1.0 } else { 0.0 });

        if ((s.b[3111] && s.b[3112]) && s.b[3127]) {
            s.store_scalar(3120, p.p339);
        }

        s.b[3128] = param_given[338];
        s.store_scalar(3128, if s.b[3128] { 1.0 } else { 0.0 });

        if (((s.b[3111] && s.b[3112]) && (!s.b[3127])) && s.b[3128]) {
            s.store_scale(3120, 3119, 0.5);
        }

        s.b[3129] = (s.v[3120] > (s.v[3119] * 0.5));
        s.store_scalar(3129, if s.b[3129] { 1.0 } else { 0.0 });

        if ((s.b[3111] && s.b[3112]) && s.b[3129]) {
            s.store_scale(3120, 3119, 0.5);
        }

        s.b[3130] = (p.p38 == 1.0);
        s.store_scalar(3130, if s.b[3130] { 1.0 } else { 0.0 });

        if ((s.b[3111] && s.b[3112]) && s.b[3130]) {
            s.store_neg(334, 396);
        }

        s.b[3131] = (s.v[334] > s.v[3120]);
        s.store_scalar(3131, if s.b[3131] { 1.0 } else { 0.0 });

        if (((s.b[3111] && s.b[3112]) && s.b[3130]) && s.b[3131]) {
            s.store_sub(335, 334, 3120);
            s.store_sub(336, 3119, 3120);
            s.store_div(781, 335, 336);
            s.store_square(782, 781);
            s.store_mul(783, 782, 781);
            s.store_square(784, 782);
            s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));
            s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);
            s.store_mul_sub_from_scalar_rhs(333, 336, 1.0, 780);
            s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 345, 1.0);
            s.store_neg(345, 345);
            s.store_add(344, 3120, 333);
        }

        if (((s.b[3111] && s.b[3112]) && s.b[3130]) && (!s.b[3131])) {
            s.copy_ad(344, 334);
        }

        if ((s.b[3111] && s.b[3112]) && s.b[3130]) {
            s.store_neg(397, 344);
        }

        if ((s.b[3111] && s.b[3112]) && (!s.b[3130])) {
            s.copy_ad(397, 396);
        }

        if (s.b[3111] && s.b[3112]) {
            s.store_div(212, 410, 413);
            s.store_square(213, 212);
            s.store_sub_from_scalar(402, s.v[458], 395);
        }

        let (assign91190_e139837,) = {
    if (s.b[3111] && s.b[3112]) {
        let assign91190_e139831: f64 = (-s.v[397]);
        let assign91190_e139834: f64 = (10.0 * 2.220446049250313e-16);
        let assign91190_e139835: f64 = (assign91190_e139831 + assign91190_e139834);
        (assign91190_e139835,)
    } else {
        (s.v[403],)
    }
};
        s.store_scalar(403, assign91190_e139837);

        if (s.b[3111] && s.b[3112]) {
            s.store_scalar(3114, 0.0);
            s.store_scale(3115, 409, 1.6021918e-19);
            s.store_div(334, 394, 409);
            s.store_square(405, 334);
        }

        s.b[3132] = ((s.v[154] * (-s.v[397])) >= 500.0);
        s.store_scalar(3132, if s.b[3132] { 1.0 } else { 0.0 });

        if ((s.b[3111] && s.b[3112]) && s.b[3132]) {
            s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);
            s.store_scalar(334, 1.403592217853e217);
        }

        if ((s.b[3111] && s.b[3112]) && (!s.b[3132])) {
            s.store_mul_neg_rhs(781, 154, 397);
            s.store_scalar(229, 1.0);
        }

        let mut assign91290_loop_guard: usize = 0;
        while {
            let assign91290_cond_e139929: f64 = if (((s.b[3111] && s.b[3112]) && (!s.b[3132])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign91290_cond_e139929 != 0.0
        } {
            assign91290_loop_guard += 1;
            assert!(assign91290_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[3111] && s.b[3112]) && (!s.b[3132])) {
                s.store_scale(229, 229, 1.14200738981568e26);
                s.store_offset(781, 781, (-60.0));
            }
        }

        if ((s.b[3111] && s.b[3112]) && (!s.b[3132])) {
            s.store_mul_exp_rhs(229, 229, 781);
            s.copy_ad(334, 229);
        }

        if ((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) {
            s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));
            s.store_scalar(782, (4.0 * 0.5));
        }

        if ((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(335, 781, 0.5, 782, 0.5, 0.5);
        }

        s.b[3133] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));
        s.store_scalar(3133, if s.b[3133] { 1.0 } else { 0.0 });

        if (((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) && s.b[3133]) {
            s.store_add_scaled_inputs3_indices(781, 402, 1.0, 397, 1.0, 335, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 335);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign91440_e140135,) = {
    if (((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) && s.b[3133]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign91440_e140135);

        let (assign91450_e140145,) = {
    if (((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) && s.b[3133]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign91450_e140145);

        if (((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) && s.b[3133]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3134] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.store_scalar(3134, if s.b[3134] { 1.0 } else { 0.0 });

        s.b[3135] = (1.0 == 1.0);
        s.store_scalar(3135, if s.b[3135] { 1.0 } else { 0.0 });

        let (assign91540_e140243,) = {
    if (((((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) && s.b[3133]) && s.b[3134]) && s.b[3135]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign91540_e140243);

        s.b[3136] = (1.0 == 2.0);
        s.store_scalar(3136, if s.b[3136] { 1.0 } else { 0.0 });

        let (assign91560_e140263,) = {
    if ((((((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) && s.b[3133]) && s.b[3134]) && (!s.b[3135])) && s.b[3136]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign91560_e140263);

        s.b[3137] = (1.0 == 4.0);
        s.store_scalar(3137, if s.b[3137] { 1.0 } else { 0.0 });

        let (assign91580_e140286,) = {
    if (((((((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) && s.b[3133]) && s.b[3134]) && (!s.b[3135])) && (!s.b[3136])) && s.b[3137]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign91580_e140286);

        s.b[3138] = (1.0 == 8.0);
        s.store_scalar(3138, if s.b[3138] { 1.0 } else { 0.0 });

        let (assign91600_e140312,) = {
    if ((((((((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) && s.b[3133]) && s.b[3134]) && (!s.b[3135])) && (!s.b[3136])) && (!s.b[3137])) && s.b[3138]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign91600_e140312);

        let (assign91610_e140324,) = {
    if ((((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) && s.b[3133]) && s.b[3134]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign91610_e140324);

        let mut assign91620_loop_guard: usize = 0;
        while {
            let assign91620_cond_e140337: f64 = if (((((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) && s.b[3133]) && s.b[3134]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign91620_cond_e140337 != 0.0
        } {
            assign91620_loop_guard += 1;
            assert!(assign91620_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) && s.b[3133]) && s.b[3134]) {
                s.store_sqrt(726, 726);
            }
            let (assign91620_body1_e140364,) = {
    if ((((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) && s.b[3133]) && s.b[3134]) {
        let assign91620_body1_e140362: f64 = (s.v[719] + 1.0);
        (assign91620_body1_e140362,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign91620_body1_e140364);
        }

        if ((((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) && s.b[3133]) && (!s.b[3134])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if (((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) && s.b[3133]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 335, 726);
            s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);
        }

        if (((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) && s.b[3133]) {
        }

        if (((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) && (!s.b[3133])) {
            s.store_add(335, 402, 397);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) {
            s.store_sub(397, 335, 402);
        }

        let (assign91720_e140501,) = {
    if ((s.b[3111] && s.b[3112]) && (s.v[406] != 0.0)) {
        let assign91720_e140495: f64 = (-s.v[397]);
        let assign91720_e140498: f64 = (10.0 * 2.220446049250313e-16);
        let assign91720_e140499: f64 = (assign91720_e140495 + assign91720_e140498);
        (assign91720_e140499,)
    } else {
        (s.v[403],)
    }
};
        s.store_scalar(403, assign91720_e140501);

        s.b[3139] = (s.v[402] < s.v[403]);
        s.store_scalar(3139, if s.b[3139] { 1.0 } else { 0.0 });

        if ((s.b[3111] && s.b[3112]) && s.b[3139]) {
            s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));
            s.store_mul_add_rhs(332, 154, 402, 397);
            s.store_div_from_scalar_mul_ad(335, 1.0, s.ad_value(154), s.ad_value(410));
            s.store_mul(333, 335, 413);
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
            s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(278, (7.0 * 1.414213562373095), 333, A::offset(s.ad_value(332), (-2.0)), 9.0);
            s.store_square(276, 278);
        }

        s.b[3140] = (s.v[277] < (s.v[276] * 1e-8));
        s.store_scalar(3140, if s.b[3140] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_transient_block_95(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[3111] && s.b[3112]) && s.b[3139]) && s.b[3140]) {
            s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);
        }

        if (((s.b[3111] && s.b[3112]) && s.b[3139]) && (!s.b[3140])) {
            s.store_sqrt_add(275, 277, 276);
            s.store_sub(274, 275, 278);
        }

        if ((s.b[3111] && s.b[3112]) && s.b[3139]) {
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
            s.copy_ad(3122, 404);
        }

        s.b[3141] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));
        s.store_scalar(3141, if s.b[3141] { 1.0 } else { 0.0 });

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3141]) {
            s.store_add_scaled_product_indices(89, 402, 1.0, 213, 154, 0.5);
        }

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && (!s.b[3141])) {
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
        }

        if ((s.b[3111] && s.b[3112]) && (!s.b[3139])) {
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        s.b[3142] = (s.v[116] >= 3.0);
        s.store_scalar(3142, if s.b[3142] { 1.0 } else { 0.0 });

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3142]) {
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
            s.store_mul_add_rhs(116, 154, 89, 397);
            s.store_exp_neg_input(333, 116);
            s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
            s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));
            s.store_mul_add_rhs(116, 154, 89, 397);
        }

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && (!s.b[3142])) {
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

        s.b[3143] = (p.p33 > 0.0);
        s.store_scalar(3143, if s.b[3143] { 1.0 } else { 0.0 });

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) {
            s.store_offset_add(442, 402, 397, 0.1);
            s.store_mul(222, 405, 229);
            s.store_mul(443, 405, 229);
            s.store_mul(334, 156, 213);
            s.store_mul(444, 154, 442);
            s.store_add_scaled_inputs_product_mixed_aaii(447, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), 154, 397, 1.0);
        }

        s.b[3144] = (p.p33 == 2.0);
        s.store_scalar(3144, if s.b[3144] { 1.0 } else { 0.0 });

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3144]) {
            s.store_offset_sub(781, 444, 447, (-1.0));
            s.store_scale(782, 444, 4.0);
        }

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3144]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3144]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(447, 444, 1.0, 781, (-0.5), 782, (-0.5));
        }

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && (!s.b[3144])) {
            if (s.v[447] <= s.v[444]) {
            } else {
                s.copy_ad(447, 444);
            }
        }

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) {
            if (s.v[447] >= 0.0) {
            } else {
                s.store_scalar(447, 0.0);
            }
        }

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) {
            s.store_sub(444, 444, 447);
            s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);
            s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));
            s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);
        }

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) {
            s.copy_ad(445, 116);
        }

        s.b[3145] = (p.p33 == 2.0);
        s.store_scalar(3145, if s.b[3145] { 1.0 } else { 0.0 });

        s.b[3146] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));
        s.store_scalar(3146, if s.b[3146] { 1.0 } else { 0.0 });

        if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3145]) && s.b[3146]) {
            s.store_add_scaled_inputs3_indices(781, 445, 1.0, 446, (-1.0), 446, 0.2);
            s.store_square(722, 781);
            s.store_scaled_mul(723, 446, 446, (0.2 * 0.2));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign92550_e141797,) = {
    if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3145]) && s.b[3146]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign92550_e141797);

        let (assign92560_e141812,) = {
    if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3145]) && s.b[3146]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign92560_e141812);

        if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3145]) && s.b[3146]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[3147] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.store_scalar(3147, if s.b[3147] { 1.0 } else { 0.0 });

        s.b[3148] = (2.0 == 1.0);
        s.store_scalar(3148, if s.b[3148] { 1.0 } else { 0.0 });

        let (assign92670_e141979,) = {
    if (((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3145]) && s.b[3146]) && s.b[3147]) && s.b[3148]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign92670_e141979);

        s.b[3149] = (2.0 == 2.0);
        s.store_scalar(3149, if s.b[3149] { 1.0 } else { 0.0 });

        let (assign92690_e142004,) = {
    if ((((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3145]) && s.b[3146]) && s.b[3147]) && (!s.b[3148])) && s.b[3149]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign92690_e142004);

        s.b[3150] = (2.0 == 4.0);
        s.store_scalar(3150, if s.b[3150] { 1.0 } else { 0.0 });

        let (assign92710_e142032,) = {
    if (((((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3145]) && s.b[3146]) && s.b[3147]) && (!s.b[3148])) && (!s.b[3149])) && s.b[3150]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign92710_e142032);

        s.b[3151] = (2.0 == 8.0);
        s.store_scalar(3151, if s.b[3151] { 1.0 } else { 0.0 });

        let (assign92730_e142063,) = {
    if ((((((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3145]) && s.b[3146]) && s.b[3147]) && (!s.b[3148])) && (!s.b[3149])) && (!s.b[3150])) && s.b[3151]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, assign92730_e142063);

        let (assign92740_e142080,) = {
    if ((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3145]) && s.b[3146]) && s.b[3147]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, assign92740_e142080);

        let mut assign92750_loop_guard: usize = 0;
        while {
            let assign92750_cond_e142098: f64 = if (((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3145]) && s.b[3146]) && s.b[3147]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign92750_cond_e142098 != 0.0
        } {
            assign92750_loop_guard += 1;
            assert!(assign92750_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3145]) && s.b[3146]) && s.b[3147]) {
                s.store_sqrt(726, 726);
            }
            let (assign92750_body1_e142135,) = {
    if ((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3145]) && s.b[3146]) && s.b[3147]) {
        let assign92750_body1_e142133: f64 = (s.v[719] + 1.0);
        (assign92750_body1_e142133,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, assign92750_body1_e142135);
        }

        if ((((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3145]) && s.b[3146]) && (!s.b[3147])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3145]) && s.b[3146]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);
            s.store_div_scaled_product3_indices(335, 446, 725, 726, 0.2, 770, 1.0);
            s.store_add_scaled_inputs3_indices(116, 446, 1.0, 446, (-0.2), 780, 1.0);
        }

        if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3145]) && s.b[3146]) {
        }

        if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && s.b[3145]) && (!s.b[3146])) {
            s.copy_ad(116, 445);
            s.store_scalar(335, 1.0);
        }

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3143]) && (!s.b[3145])) {
            if (s.v[445] <= s.v[446]) {
                s.copy_ad(116, 445);
            } else {
                s.copy_ad(116, 446);
            }
        }

        s.b[3152] = (p.p33 == 1.0);
        s.store_scalar(3152, if s.b[3152] { 1.0 } else { 0.0 });

        if (((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3152]) {
            s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);
        }

        s.b[3153] = (s.v[411] > 0.0);
        s.store_scalar(3153, if s.b[3153] { 1.0 } else { 0.0 });

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3152]) && s.b[3153]) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3152]) && (!s.b[3153])) {
            s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));
            s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);
            s.store_scaled_add_ad_lhs(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);
        }

        s.b[3154] = (s.v[336] < 0.0);
        s.store_scalar(3154, if s.b[3154] { 1.0 } else { 0.0 });

        if (((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3152]) && (!s.b[3153])) && s.b[3154]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(343, 0.0);
        }

        if ((((s.b[3111] && s.b[3112]) && (!s.b[3139])) && s.b[3152]) && (!s.b[3153])) {
            s.store_scaled_sqrt_mul(600, 651, 336, p.p432);
        }

    }
}

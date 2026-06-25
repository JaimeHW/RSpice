#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_block_80(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) {
            s.store_scale_ad(335, A::add(A::limited_exp(s.ad_value(334)), A::limited_exp(A::neg(s.ad_value(334)))), 0.5);
        }

        if (((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) {
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
        }

        if (((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) {
            s.store_scalar(97, 1.0);
        }

        let mut assign89530_loop_guard: usize = 0;
        while {
            let assign89530_cond_e136189: f64 = (s.v[421] + 1.0);
            let assign89530_cond_e136191: f64 = if ((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (s.v[97] <= assign89530_cond_e136189)) { 1.0 } else { 0.0 };
            assign89530_cond_e136191 != 0.0
        } {
            assign89530_loop_guard += 1;
            assert!(assign89530_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) {
                s.store_add(414, 404, 397);
            }
            if (((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) {
                s.store_mul(116, 154, 414);
            }
            if (((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) {
                s.store_mul_ad_rhs(333, 419, A::sub(s.ad_value(414), s.ad_value(418)));
            }
            s.v[3068] = if (s.v[333] < 60.0) { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (s.v[3068] != 0.0)) {
                s.store_exp(335, 333);
            }
            if ((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (s.v[3068] != 0.0)) {
                s.store_exp_ad(334, A::mul(A::neg(s.ad_value(419)), s.ad_value(418)));
            }
            if ((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (s.v[3068] != 0.0)) {
                s.store_sub(336, 335, 334);
            }
            if ((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (s.v[3068] != 0.0)) {
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
            }
            if ((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (s.v[3068] != 0.0)) {
                s.store_div_ad_rhs(417, 335, A::offset(s.ad_value(336), 1.0));
            }
            if ((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3068] != 0.0))) {
                s.store_sub(416, 414, 418);
            }
            if ((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3068] != 0.0))) {
                s.store_scalar(417, 1.0);
            }
            if (((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) {
                s.store_mul(415, 154, 416);
            }
            s.v[3069] = if (s.v[116] < 0.0) { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (s.v[3069] != 0.0)) {
                s.store_scalar(334, (-0.7071067811865475));
            }
            if ((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (s.v[3069] != 0.0)) {
                s.store_mul(223, 116, 334);
            }
            if ((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (s.v[3069] != 0.0)) {
                s.store_mul(420, 154, 334);
            }
            s.v[3070] = if (s.v[116] < 1e-6) { 1.0 } else { 0.0 };
            if (((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3069] != 0.0))) && (s.v[3070] != 0.0)) {
                s.store_mul_ad(334, A::scale(A::square(s.ad_value(116)), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(116), 0.3333333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(116), 0.25), A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.2)))))));
            }
            if (((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3069] != 0.0))) && (s.v[3070] != 0.0)) {
                s.store_mul_ad_rhs(335, 116, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(116), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(116), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.25)))))));
            }
            if (((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3069] != 0.0))) && (s.v[3070] != 0.0)) {
                s.store_mul_ad(336, A::scale(A::square(s.ad_value(415)), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(415), 0.3333333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(415), 0.25), A::sub_from_scalar(1.0, A::scale(s.ad_value(415), 0.2)))))));
            }
            if (((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3069] != 0.0))) && (s.v[3070] != 0.0)) {
                s.store_mul_ad_rhs(337, 415, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(415), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(415), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(415), 0.25)))))));
            }
            if (((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3069] != 0.0))) && (s.v[3070] != 0.0)) {
                s.store_sub(338, 334, 336);
            }
            s.v[3071] = if (s.v[338] > 0.0) { 1.0 } else { 0.0 };
            if ((((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3069] != 0.0))) && (s.v[3070] != 0.0)) && (s.v[3071] != 0.0)) {
                s.store_sqrt(223, 338);
            }
            if ((((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3069] != 0.0))) && (s.v[3070] != 0.0)) && (s.v[3071] != 0.0)) {
                s.store_div_ad_lhs(420, A::mul(A::scale(s.ad_value(154), 0.5), A::sub(s.ad_value(335), A::mul(s.ad_value(417), s.ad_value(337)))), 223);
            }
            if ((((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3069] != 0.0))) && (s.v[3070] != 0.0)) && (!(s.v[3071] != 0.0))) {
                s.store_scalar(223, 0.0);
            }
            if ((((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3069] != 0.0))) && (s.v[3070] != 0.0)) && (!(s.v[3071] != 0.0))) {
                s.store_scalar(420, 0.0);
            }
            if (((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3069] != 0.0))) && (!(s.v[3070] != 0.0))) {
                s.store_exp_ad(334, A::neg(s.ad_value(116)));
            }
            if (((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3069] != 0.0))) && (!(s.v[3070] != 0.0))) {
                s.store_exp_ad(335, A::neg(s.ad_value(415)));
            }
            if (((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3069] != 0.0))) && (!(s.v[3070] != 0.0))) {
                s.store_add_ad(338, A::sub(s.ad_value(116), s.ad_value(415)), A::sub(s.ad_value(334), s.ad_value(335)));
            }
            s.v[3072] = if (s.v[338] > 0.0) { 1.0 } else { 0.0 };
            if ((((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3069] != 0.0))) && (!(s.v[3070] != 0.0))) && (s.v[3072] != 0.0)) {
                s.store_sqrt(223, 338);
            }
            if ((((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3069] != 0.0))) && (!(s.v[3070] != 0.0))) && (s.v[3072] != 0.0)) {
                s.store_div_ad_lhs(420, A::mul(A::scale(s.ad_value(154), 0.5), A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul(s.ad_value(417), A::sub_from_scalar(1.0, s.ad_value(335))))), 223);
            }
            if ((((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3069] != 0.0))) && (!(s.v[3070] != 0.0))) && (!(s.v[3072] != 0.0))) {
                s.store_scalar(223, 0.0);
            }
            if ((((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3069] != 0.0))) && (!(s.v[3070] != 0.0))) && (!(s.v[3072] != 0.0))) {
                s.store_scalar(420, 0.0);
            }
            s.v[3073] = if (s.v[116] < 0.0) { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (s.v[3073] != 0.0)) {
                s.store_scalar(214, 0.0);
            }
            if ((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (s.v[3073] != 0.0)) {
                s.store_scalar(215, 0.0);
            }
            if ((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (s.v[3073] != 0.0)) {
                s.store_neg(216, 223);
            }
            if ((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (s.v[3073] != 0.0)) {
                s.store_neg(217, 420);
            }
            s.v[3074] = if (s.v[116] < 60.0) { 1.0 } else { 0.0 };
            s.v[3075] = if (s.v[116] < 5e-5) { 1.0 } else { 0.0 };
            if ((((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3073] != 0.0))) && (s.v[3074] != 0.0)) && (s.v[3075] != 0.0)) {
                s.store_mul_ad(334, A::scale(A::square(s.ad_value(116)), 0.5), A::offset(A::mul(A::scale(s.ad_value(116), 0.3333333333333333), A::offset(A::mul(A::scale(s.ad_value(116), 0.25), A::offset(A::scale(s.ad_value(116), 0.2), 1.0)), 1.0)), 1.0));
            }
            if ((((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3073] != 0.0))) && (s.v[3074] != 0.0)) && (s.v[3075] != 0.0)) {
                s.store_mul_ad_rhs(335, 116, A::offset(A::mul(A::scale(s.ad_value(116), 0.5), A::offset(A::mul(A::scale(s.ad_value(116), 0.3333333333333333), A::offset(A::scale(s.ad_value(116), 0.25), 1.0)), 1.0)), 1.0));
            }
            if ((((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3073] != 0.0))) && (s.v[3074] != 0.0)) && (s.v[3075] != 0.0)) {
                s.store_mul(214, 222, 334);
            }
            if ((((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3073] != 0.0))) && (s.v[3074] != 0.0)) && (s.v[3075] != 0.0)) {
                s.store_mul_ad_lhs(215, A::mul(s.ad_value(222), s.ad_value(335)), 154);
            }
            if ((((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3073] != 0.0))) && (s.v[3074] != 0.0)) && (!(s.v[3075] != 0.0))) {
                s.store_exp(227, 116);
            }
            if ((((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3073] != 0.0))) && (s.v[3074] != 0.0)) && (!(s.v[3075] != 0.0))) {
                s.store_offset(335, 227, (-1.0));
            }
            if ((((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3073] != 0.0))) && (s.v[3074] != 0.0)) && (!(s.v[3075] != 0.0))) {
                s.store_mul_ad_rhs(214, 222, A::sub(s.ad_value(335), s.ad_value(116)));
            }
            if ((((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3073] != 0.0))) && (s.v[3074] != 0.0)) && (!(s.v[3075] != 0.0))) {
                s.store_mul_ad_lhs(215, A::mul(s.ad_value(222), s.ad_value(154)), 335);
            }
            if (((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3073] != 0.0))) && (!(s.v[3074] != 0.0))) {
                s.store_exp_ad(231, A::mul(s.ad_value(154), s.ad_value(404)));
            }
            if (((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3073] != 0.0))) && (!(s.v[3074] != 0.0))) {
                s.store_mul_ad_rhs(214, 405, A::sub(s.ad_value(231), A::mul(s.ad_value(229), A::offset(s.ad_value(116), 1.0))));
            }
            if (((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3073] != 0.0))) && (!(s.v[3074] != 0.0))) {
                s.store_mul_ad(215, A::mul(s.ad_value(405), s.ad_value(154)), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.v[3076] = if (s.v[214] > 0.0) { 1.0 } else { 0.0 };
            if (((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3073] != 0.0))) && (s.v[3076] != 0.0)) {
                s.store_sqrt_ad(216, A::add(A::square(s.ad_value(223)), s.ad_value(214)));
            }
            if (((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3073] != 0.0))) && (s.v[3076] != 0.0)) {
                s.store_div_ad_lhs(217, A::scale(A::add(A::mul(A::scale(s.ad_value(420), 2.0), s.ad_value(223)), s.ad_value(215)), 0.5), 216);
            }
            if (((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3073] != 0.0))) && (!(s.v[3076] != 0.0))) {
                s.copy_ad(216, 223);
            }
            if (((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3073] != 0.0))) && (!(s.v[3076] != 0.0))) {
                s.copy_ad(217, 420);
            }
            if (((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) {
                s.store_add_ad(232, A::sub(s.ad_value(404), s.ad_value(402)), A::mul(s.ad_value(212), s.ad_value(216)));
            }
            if (((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) {
                s.store_offset_ad(233, A::mul(s.ad_value(212), s.ad_value(217)), 1.0);
            }
            s.v[3077] = if (s.v[79] == 1.0) { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (s.v[3077] != 0.0)) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if ((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3077] != 0.0))) {
                s.store_div_ad_lhs(236, A::neg(s.ad_value(232)), 233);
            }
            if ((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3077] != 0.0))) {
                s.store_scale_ad(93, A::offset({
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0), (0.5 * 0.1));
            }
            s.v[3078] = if (((s.v[236]) as f64).abs() > s.v[93]) { 1.0 } else { 0.0 };
            if (((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3077] != 0.0))) && (s.v[3078] != 0.0)) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3077] != 0.0))) {
                s.store_add(404, 404, 236);
            }
            s.v[3079] = if ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8)) { 1.0 } else { 0.0 };
            if (((((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) && (!(s.v[3077] != 0.0))) && (s.v[3079] != 0.0)) {
                s.store_scalar(79, 1.0);
            }
            if (((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if (((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) {
            s.store_mul(2998, 982, 223);
        }

        if (((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) {
            s.store_mul(2999, 3000, 2998);
        }

        if (((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) {
            s.store_offset_ad(100, A::div(s.ad_value(2999), s.ad_value(410)), (10.0 * 2.220446049250313e-16));
        }

        if (((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) {
            s.store_mul(354, 410, 100);
        }

        if (((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) {
            s.store_div_from_scalar_ad(335, 1.0, A::add(s.ad_value(216), s.ad_value(100)));
        }

        if (((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) {
            s.store_mul_ad_lhs(399, A::mul(s.ad_value(410), s.ad_value(214)), 335);
        }

        if (((s.v[2621] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3063] != 0.0)) {
            s.store_add(398, 354, 399);
        }

        s.v[3081] = if (p.p33 == 4.0) { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
            s.store_exp_ad(229, A::mul(s.ad_value(154), A::neg(s.ad_value(397))));
        }

        if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
            s.store_div(334, 394, 409);
        }

        if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
            s.store_square(405, 334);
        }

        if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
            s.store_mul(222, 405, 229);
        }

        if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
            s.copy_ad(404, 3007);
        }

        if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
            s.store_scalar(79, 0.0);
        }

        if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
            s.store_sqrt_ad(982, A::scale(A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(3000)), s.ad_value(155)), 2.0));
        }

        s.v[3082] = if (s.v[411] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (s.v[3082] != 0.0)) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3082] != 0.0))) {
            s.store_sqrt_ad(782, A::offset(A::mul(A::offset(s.ad_value(729), p.p137), A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1)));
        }

        if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3082] != 0.0))) {
            s.store_scale_ad(343, A::offset(A::div(A::offset(s.ad_value(729), p.p137), s.ad_value(782)), 1.0), 0.5);
        }

        if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3082] != 0.0))) {
            s.store_scale_ad(336, A::add(A::offset(s.ad_value(729), p.p137), s.ad_value(782)), 0.5);
        }

        s.v[3083] = if (s.v[336] < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3082] != 0.0))) && (s.v[3083] != 0.0)) {
            s.store_scalar(336, 0.0);
        }

        if ((((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3082] != 0.0))) && (s.v[3083] != 0.0)) {
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3082] != 0.0))) {
            s.store_scale_ad(600, A::sqrt(A::mul(s.ad_value(651), s.ad_value(336))), p.p432);
        }

        if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3082] != 0.0))) {
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
            s.store_sqrt_ad(782, A::offset(A::square(s.ad_value(336)), ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01))));
        }

        if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
            s.store_scale_ad(343, A::offset(A::div(s.ad_value(336), s.ad_value(782)), 1.0), 0.5);
        }

        if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.v[3084] = if (s.v[336] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (s.v[3084] != 0.0)) {
            s.store_scalar(336, 0.0);
        }

        if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (s.v[3084] != 0.0)) {
            s.store_scalar(343, 0.0);
        }

        if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
            s.copy_ad(386, 336);
        }

        if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
            s.store_scale_ad(418, A::scale(A::mul(A::mul(s.ad_value(3000), s.ad_value(386)), s.ad_value(386)), 0.5), 9662367879.197212);
        }

        if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
            s.store_sqrt_ad(334, A::mul(A::scale(s.ad_value(154), 2.0), s.ad_value(418)));
        }

        if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
            s.store_scale_ad(335, A::add(A::limited_exp(s.ad_value(334)), A::limited_exp(A::neg(s.ad_value(334)))), 0.5);
        }

        if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
        }

        if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_81(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let mut assign89920_loop_guard: usize = 0;
        while {
            let assign89920_cond_e137844: f64 = (s.v[421] + 1.0);
            let assign89920_cond_e137846: f64 = if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (s.v[97] <= assign89920_cond_e137844)) { 1.0 } else { 0.0 };
            assign89920_cond_e137846 != 0.0
        } {
            assign89920_loop_guard += 1;
            assert!(assign89920_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
                s.store_add(414, 404, 397);
            }
            if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
                s.store_mul(116, 154, 414);
            }
            if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
                s.store_mul_ad_rhs(333, 419, A::sub(s.ad_value(414), s.ad_value(418)));
            }
            s.v[3086] = if (s.v[333] < 60.0) { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (s.v[3086] != 0.0)) {
                s.store_exp(335, 333);
            }
            if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (s.v[3086] != 0.0)) {
                s.store_exp_ad(334, A::mul(A::neg(s.ad_value(419)), s.ad_value(418)));
            }
            if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (s.v[3086] != 0.0)) {
                s.store_sub(336, 335, 334);
            }
            if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (s.v[3086] != 0.0)) {
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
            }
            if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (s.v[3086] != 0.0)) {
                s.store_div_ad_rhs(417, 335, A::offset(s.ad_value(336), 1.0));
            }
            if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3086] != 0.0))) {
                s.store_sub(416, 414, 418);
            }
            if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3086] != 0.0))) {
                s.store_scalar(417, 1.0);
            }
            if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
                s.store_mul(415, 154, 416);
            }
            s.v[3087] = if (((s.v[116]) as f64).abs() < 1e-6) { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (s.v[3087] != 0.0)) {
                s.store_mul_ad(334, A::scale(A::square(s.ad_value(116)), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(116), 0.3333333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(116), 0.25), A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.2)))))));
            }
            if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (s.v[3087] != 0.0)) {
                s.store_mul_ad_rhs(335, 116, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(116), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(116), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.25)))))));
            }
            if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (s.v[3087] != 0.0)) {
                s.store_mul_ad(336, A::scale(A::square(s.ad_value(415)), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(415), 0.3333333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(415), 0.25), A::sub_from_scalar(1.0, A::scale(s.ad_value(415), 0.2)))))));
            }
            if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (s.v[3087] != 0.0)) {
                s.store_mul_ad_rhs(337, 415, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(415), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(415), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(415), 0.25)))))));
            }
            if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (s.v[3087] != 0.0)) {
                s.store_sub(3008, 334, 336);
            }
            if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (s.v[3087] != 0.0)) {
                s.store_mul_ad_rhs(3009, 154, A::sub(s.ad_value(335), A::mul(s.ad_value(417), s.ad_value(337))));
            }
            if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3087] != 0.0))) {
                s.store_exp_ad(334, A::neg(s.ad_value(116)));
            }
            if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3087] != 0.0))) {
                s.store_exp_ad(335, A::neg(s.ad_value(415)));
            }
            if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3087] != 0.0))) {
                s.store_add_ad(3008, A::sub(s.ad_value(116), s.ad_value(415)), A::sub(s.ad_value(334), s.ad_value(335)));
            }
            if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3087] != 0.0))) {
                s.store_mul_ad_rhs(3009, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul(s.ad_value(417), A::sub_from_scalar(1.0, s.ad_value(335)))));
            }
            s.v[3088] = if (((s.v[116]) as f64).abs() < 5e-5) { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (s.v[3088] != 0.0)) {
                s.store_mul_ad(334, A::scale(A::square(s.ad_value(116)), 0.5), A::offset(A::mul(A::scale(s.ad_value(116), 0.3333333333333333), A::offset(A::mul(A::scale(s.ad_value(116), 0.25), A::offset(A::scale(s.ad_value(116), 0.2), 1.0)), 1.0)), 1.0));
            }
            if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (s.v[3088] != 0.0)) {
                s.store_mul_ad_rhs(335, 116, A::offset(A::mul(A::scale(s.ad_value(116), 0.5), A::offset(A::mul(A::scale(s.ad_value(116), 0.3333333333333333), A::offset(A::scale(s.ad_value(116), 0.25), 1.0)), 1.0)), 1.0));
            }
            if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (s.v[3088] != 0.0)) {
                s.store_mul(214, 222, 334);
            }
            if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (s.v[3088] != 0.0)) {
                s.store_mul_ad_lhs(215, A::mul(s.ad_value(222), s.ad_value(335)), 154);
            }
            s.v[3089] = if (((s.v[116]) as f64).abs() < 60.0) { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3088] != 0.0))) && (s.v[3089] != 0.0)) {
                s.store_exp(227, 116);
            }
            if ((((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3088] != 0.0))) && (s.v[3089] != 0.0)) {
                s.store_offset(335, 227, (-1.0));
            }
            if ((((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3088] != 0.0))) && (s.v[3089] != 0.0)) {
                s.store_mul_ad_rhs(214, 222, A::sub(s.ad_value(335), s.ad_value(116)));
            }
            if ((((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3088] != 0.0))) && (s.v[3089] != 0.0)) {
                s.store_mul_ad_lhs(215, A::mul(s.ad_value(222), s.ad_value(154)), 335);
            }
            if ((((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3088] != 0.0))) && (!(s.v[3089] != 0.0))) {
                s.store_exp_ad(231, A::mul(s.ad_value(154), s.ad_value(404)));
            }
            if ((((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3088] != 0.0))) && (!(s.v[3089] != 0.0))) {
                s.store_mul_ad_rhs(214, 405, A::sub(s.ad_value(231), A::mul(s.ad_value(229), A::offset(s.ad_value(116), 1.0))));
            }
            if ((((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3088] != 0.0))) && (!(s.v[3089] != 0.0))) {
                s.store_mul_ad(215, A::mul(s.ad_value(405), s.ad_value(154)), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.v[3090] = if (s.v[214] > 0.0) { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (s.v[3090] != 0.0)) {
                s.store_sqrt_ad(216, A::add(s.ad_value(3008), s.ad_value(214)));
            }
            if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (s.v[3090] != 0.0)) {
                s.store_div_ad_lhs(217, A::scale(A::add(s.ad_value(3009), s.ad_value(215)), 0.5), 216);
            }
            s.v[3091] = if (s.v[3008] > 0.0) { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3090] != 0.0))) && (s.v[3091] != 0.0)) {
                s.store_sqrt(216, 3008);
            }
            if ((((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3090] != 0.0))) && (s.v[3091] != 0.0)) {
                s.store_div_ad_lhs(217, A::scale(s.ad_value(3009), 0.5), 216);
            }
            if ((((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3090] != 0.0))) && (!(s.v[3091] != 0.0))) {
                s.store_scalar(216, 0.0);
            }
            if ((((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3090] != 0.0))) && (!(s.v[3091] != 0.0))) {
                s.store_scalar(217, 0.0);
            }
            if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
                s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
                s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
                s.store_add_ad(232, A::sub(s.ad_value(404), s.ad_value(402)), A::mul(s.ad_value(212), s.ad_value(216)));
            }
            if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
                s.store_offset_ad(233, A::mul(s.ad_value(212), s.ad_value(217)), 1.0);
            }
            s.v[3092] = if (s.v[79] > 0.0) { 1.0 } else { 0.0 };
            if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (s.v[3092] != 0.0)) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3092] != 0.0))) {
                s.store_div_ad_lhs(236, A::neg(s.ad_value(232)), 233);
            }
            if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3092] != 0.0))) {
                s.store_scale_ad(93, A::offset({
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0), (0.5 * 0.1));
            }
            s.v[3093] = if (((s.v[236]) as f64).abs() > s.v[93]) { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3092] != 0.0))) && (s.v[3093] != 0.0)) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3092] != 0.0))) {
                s.store_add(404, 404, 236);
            }
            s.v[3094] = if ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8)) { 1.0 } else { 0.0 };
            if ((((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) && (!(s.v[3092] != 0.0))) && (s.v[3094] != 0.0)) {
                s.store_offset(79, 79, 2.0);
            }
            if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
            s.store_ad(223, &{
                if (s.v[3008] >= 0.0) {
                    A::scale(A::sqrt(s.ad_value(3008)), (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
            s.store_mul(2998, 982, 223);
        }

        if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
            s.store_mul(2999, 3000, 2998);
        }

        if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
            s.store_offset_ad(100, A::div(s.ad_value(2999), s.ad_value(410)), (10.0 * 2.220446049250313e-16));
        }

        if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
            s.store_mul(354, 410, 100);
        }

        if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
            s.store_div_from_scalar_ad(335, 1.0, A::add(s.ad_value(216), s.ad_value(100)));
        }

        if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
            s.store_mul_ad_lhs(399, A::mul(s.ad_value(410), s.ad_value(214)), 335);
        }

        if ((s.v[2621] != 0.0) && (s.v[3081] != 0.0)) {
            s.store_add(398, 354, 399);
        }

        if (s.v[2621] != 0.0) {
            s.store_sub(399, 398, 354);
        }

        s.v[3096] = if (s.v[407] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2621] != 0.0) && (s.v[3096] != 0.0)) {
            s.store_neg(407, 407);
        }

        s.v[3097] = if (p.p55 == 0.0) { 1.0 } else { 0.0 };

        s.v[3098] = if (p.p50 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (s.v[3096] != 0.0)) && (s.v[3097] != 0.0)) && (s.v[3098] != 0.0)) {
            s.store_neg(3001, 404);
        }

        if ((((s.v[2621] != 0.0) && (s.v[3096] != 0.0)) && (s.v[3097] != 0.0)) && (!(s.v[3098] != 0.0))) {
            s.copy_ad(3001, 396);
        }

        if (((s.v[2621] != 0.0) && (s.v[3096] != 0.0)) && (s.v[3097] != 0.0)) {
            s.store_sqrt_ad(782, A::offset(A::mul(A::offset(s.ad_value(3001), p.p137), A::offset(s.ad_value(3001), p.p137)), ((4.0 * 0.1) * 0.1)));
        }

        if (((s.v[2621] != 0.0) && (s.v[3096] != 0.0)) && (s.v[3097] != 0.0)) {
            s.store_scale_ad(343, A::offset(A::div(A::offset(s.ad_value(3001), p.p137), s.ad_value(782)), 1.0), 0.5);
        }

        if (((s.v[2621] != 0.0) && (s.v[3096] != 0.0)) && (s.v[3097] != 0.0)) {
            s.store_scale_ad(336, A::add(A::offset(s.ad_value(3001), p.p137), s.ad_value(782)), 0.5);
        }

        s.v[3099] = if (s.v[336] < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2621] != 0.0) && (s.v[3096] != 0.0)) && (s.v[3097] != 0.0)) && (s.v[3099] != 0.0)) {
            s.store_scalar(336, 0.0);
        }

        if ((((s.v[2621] != 0.0) && (s.v[3096] != 0.0)) && (s.v[3097] != 0.0)) && (s.v[3099] != 0.0)) {
            s.store_scalar(343, 0.0);
        }

        if (((s.v[2621] != 0.0) && (s.v[3096] != 0.0)) && (s.v[3097] != 0.0)) {
            s.store_scale_ad(600, A::sqrt(A::mul(s.ad_value(651), s.ad_value(336))), p.p432);
        }

        if (((s.v[2621] != 0.0) && (s.v[3096] != 0.0)) && (s.v[3097] != 0.0)) {
            s.store_sub_ad(781, A::sub(s.ad_value(407), s.ad_value(600)), A::scale(s.ad_value(407), 0.1));
        }

        if (((s.v[2621] != 0.0) && (s.v[3096] != 0.0)) && (s.v[3097] != 0.0)) {
            s.store_mul_ad(782, A::scale(s.ad_value(407), 4.0), A::scale(s.ad_value(407), 0.1));
        }

        if (((s.v[2621] != 0.0) && (s.v[3096] != 0.0)) && (s.v[3097] != 0.0)) {
            s.store_ad(782, &{
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (((s.v[2621] != 0.0) && (s.v[3096] != 0.0)) && (s.v[3097] != 0.0)) {
            s.store_sqrt_ad(782, A::add(A::square(s.ad_value(781)), s.ad_value(782)));
        }

        if (((s.v[2621] != 0.0) && (s.v[3096] != 0.0)) && (s.v[3097] != 0.0)) {
            s.store_scale_ad(334, A::offset(A::div(s.ad_value(781), s.ad_value(782)), 1.0), 0.5);
        }

        if (((s.v[2621] != 0.0) && (s.v[3096] != 0.0)) && (s.v[3097] != 0.0)) {
            s.store_sub_ad_rhs(603, 407, A::scale(A::add(s.ad_value(781), s.ad_value(782)), 0.5));
        }

        if (((s.v[2621] != 0.0) && (s.v[3096] != 0.0)) && (s.v[3097] != 0.0)) {
            s.store_sub(407, 407, 603);
        }

        s.v[3100] = if (4.0 == 1.0) { 1.0 } else { 0.0 };

        s.v[3101] = if (4.0 == 2.0) { 1.0 } else { 0.0 };

        s.v[3102] = if (4.0 == 3.0) { 1.0 } else { 0.0 };

        s.v[3103] = if (4.0 == 4.0) { 1.0 } else { 0.0 };

        s.v[3104] = if (p.p55 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && (s.v[3100] != 0.0)) && (s.v[3104] != 0.0)) {
            s.store_scale(338, 407, s.v[635]);
        }

        if (((s.v[2621] != 0.0) && (s.v[3100] != 0.0)) && (!(s.v[3104] != 0.0))) {
            s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));
        }

        if ((s.v[2621] != 0.0) && (s.v[3100] != 0.0)) {
            s.store_mul(353, 338, 398);
        }

        if ((s.v[2621] != 0.0) && (s.v[3100] != 0.0)) {
            s.store_mul(356, 338, 354);
        }

        if ((s.v[2621] != 0.0) && ((s.v[3101] != 0.0) && (!(s.v[3100] != 0.0)))) {
            s.store_scale(338, 407, (s.v[635] * s.v[526]));
        }

        if ((s.v[2621] != 0.0) && ((s.v[3101] != 0.0) && (!(s.v[3100] != 0.0)))) {
            s.store_mul(351, 338, 398);
        }

        if ((s.v[2621] != 0.0) && ((s.v[3101] != 0.0) && (!(s.v[3100] != 0.0)))) {
            s.store_mul(359, 338, 354);
        }

        s.v[3105] = if (p.p55 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && ((s.v[3102] != 0.0) && (!((s.v[3100] != 0.0) || (s.v[3101] != 0.0))))) && (s.v[3105] != 0.0)) {
            s.store_scale(338, 407, s.v[635]);
        }

        if (((s.v[2621] != 0.0) && ((s.v[3102] != 0.0) && (!((s.v[3100] != 0.0) || (s.v[3101] != 0.0))))) && (!(s.v[3105] != 0.0))) {
            s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));
        }

        if ((s.v[2621] != 0.0) && ((s.v[3102] != 0.0) && (!((s.v[3100] != 0.0) || (s.v[3101] != 0.0))))) {
            s.copy_ad(697, 404);
        }

        s.v[3106] = if (p.p430 == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2621] != 0.0) && ((s.v[3102] != 0.0) && (!((s.v[3100] != 0.0) || (s.v[3101] != 0.0))))) && (s.v[3106] != 0.0)) {
            s.copy_ad(698, 354);
        }

        if ((s.v[2621] != 0.0) && ((s.v[3102] != 0.0) && (!((s.v[3100] != 0.0) || (s.v[3101] != 0.0))))) {
            s.store_mul(352, 338, 398);
        }

        if ((s.v[2621] != 0.0) && ((s.v[3102] != 0.0) && (!((s.v[3100] != 0.0) || (s.v[3101] != 0.0))))) {
            s.store_mul(355, 338, 354);
        }

        if ((s.v[2621] != 0.0) && ((s.v[3102] != 0.0) && (!((s.v[3100] != 0.0) || (s.v[3101] != 0.0))))) {
            s.copy_ad(816, 355);
        }

        if ((s.v[2621] != 0.0) && ((s.v[3103] != 0.0) && (!(((s.v[3100] != 0.0) || (s.v[3101] != 0.0)) || (s.v[3102] != 0.0))))) {
            s.store_scale(338, 407, (s.v[635] * s.v[526]));
        }

        if ((s.v[2621] != 0.0) && ((s.v[3103] != 0.0) && (!(((s.v[3100] != 0.0) || (s.v[3101] != 0.0)) || (s.v[3102] != 0.0))))) {
            s.store_mul(350, 338, 398);
        }

        if ((s.v[2621] != 0.0) && ((s.v[3103] != 0.0) && (!(((s.v[3100] != 0.0) || (s.v[3101] != 0.0)) || (s.v[3102] != 0.0))))) {
            s.store_mul(358, 338, 354);
        }

        s.v[3107] = if (p.p430 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[3107] != 0.0) {
            s.store_scalar(406, 1.0);
        }

        s.v[3108] = if (((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_sub(395, 731, 728);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_sub(396, 729, 728);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_scalar(409, s.v[459]);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_scalar(407, 0.0);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.copy_ad(411, 384);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.copy_ad(410, 686);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.copy_ad(413, 412);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_scalar(3116, 0.4);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_scalar(3117, 0.0);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_scalar(223, 0.0);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_scalar(214, 0.0);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_scalar(216, 0.0);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_scalar(232, 0.0);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_scalar(236, 0.0);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_scalar(233, 0.0);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_scalar(217, 0.0);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_scalar(420, 0.0);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_scalar(215, 0.0);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_scalar(447, 0.0);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_scalar(445, 0.0);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_scalar(446, 0.0);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_scalar(79, (-1.0));
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_scalar(3118, 0.0);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_scalar(3119, 0.0);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_mul_ad(3114, A::scale(s.ad_value(155), 2.0), A::ln(A::div(s.ad_value(409), s.ad_value(394))));
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_offset_ad(781, A::sub_from_scalar(0.8, s.ad_value(3114)), (-0.1));
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_scalar(782, ((4.0 * 0.8) * 0.1));
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_ad(782, &{
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_sqrt_ad(782, A::add(A::square(s.ad_value(781)), s.ad_value(782)));
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_scale_ad(334, A::offset(A::div(s.ad_value(781), s.ad_value(782)), 1.0), 0.5);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_sub_from_scalar_ad(3115, 0.8, A::scale(A::add(s.ad_value(781), s.ad_value(782)), 0.5));
        }

    }

    pub(super) fn stamp_transient_block_82(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[3121] = if (s.v[3116] > (s.v[3115] * 0.5)) { 1.0 } else { 0.0 };

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3121] != 0.0)) {
            s.store_scale(3116, 3115, 0.5);
        }

        s.v[3122] = if self.param_given[338] { 1.0 } else { 0.0 };

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3122] != 0.0)) {
            s.store_scalar(3115, p.p338);
        }

        s.v[3123] = if self.param_given[339] { 1.0 } else { 0.0 };

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3123] != 0.0)) {
            s.store_scalar(3116, p.p339);
        }

        s.v[3124] = if self.param_given[338] { 1.0 } else { 0.0 };

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3123] != 0.0))) && (s.v[3124] != 0.0)) {
            s.store_scale(3116, 3115, 0.5);
        }

        s.v[3125] = if (s.v[3116] > (s.v[3115] * 0.5)) { 1.0 } else { 0.0 };

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3125] != 0.0)) {
            s.store_scale(3116, 3115, 0.5);
        }

        s.v[3126] = if (p.p38 == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3126] != 0.0)) {
            s.store_neg(334, 396);
        }

        s.v[3127] = if (s.v[334] > s.v[3116]) { 1.0 } else { 0.0 };

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3126] != 0.0)) && (s.v[3127] != 0.0)) {
            s.store_sub(335, 334, 3116);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3126] != 0.0)) && (s.v[3127] != 0.0)) {
            s.store_sub(336, 3115, 3116);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3126] != 0.0)) && (s.v[3127] != 0.0)) {
            s.store_div(781, 335, 336);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3126] != 0.0)) && (s.v[3127] != 0.0)) {
            s.store_square(782, 781);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3126] != 0.0)) && (s.v[3127] != 0.0)) {
            s.store_mul(783, 782, 781);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3126] != 0.0)) && (s.v[3127] != 0.0)) {
            s.store_square(784, 782);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3126] != 0.0)) && (s.v[3127] != 0.0)) {
            s.store_div_from_scalar_ad(780, 1.0, A::add(A::add(A::add(A::offset(s.ad_value(781), 1.0), s.ad_value(782)), s.ad_value(783)), s.ad_value(784)));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3126] != 0.0)) && (s.v[3127] != 0.0)) {
            s.store_mul_ad_lhs(345, A::mul(A::neg(A::add(A::add(A::offset(A::scale(s.ad_value(781), 2.0), 1.0), A::scale(s.ad_value(782), 3.0)), A::scale(s.ad_value(783), 4.0))), s.ad_value(780)), 780);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3126] != 0.0)) && (s.v[3127] != 0.0)) {
            s.store_mul_ad_rhs(333, 336, A::sub_from_scalar(1.0, s.ad_value(780)));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3126] != 0.0)) && (s.v[3127] != 0.0)) {
            s.store_add_ad(334, A::sub_from_scalar(1.0, s.ad_value(780)), A::mul(s.ad_value(781), s.ad_value(345)));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3126] != 0.0)) && (s.v[3127] != 0.0)) {
            s.store_neg(345, 345);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3126] != 0.0)) && (s.v[3127] != 0.0)) {
            s.store_add(344, 3116, 333);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3126] != 0.0)) && (!(s.v[3127] != 0.0))) {
            s.copy_ad(344, 334);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3126] != 0.0)) {
            s.store_neg(397, 344);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3126] != 0.0))) {
            s.copy_ad(397, 396);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_div(212, 410, 413);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_square(213, 212);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_sub_from_scalar(402, s.v[458], 395);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_scalar(3110, 0.0);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_scale(3111, 409, 1.6021918e-19);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_div(334, 394, 409);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_square(405, 334);
        }

        s.v[3128] = if ((s.v[154] * (-s.v[397])) >= 500.0) { 1.0 } else { 0.0 };

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3128] != 0.0)) {
            s.store_scale_ad(229, A::offset(A::offset(A::mul(s.ad_value(154), A::neg(s.ad_value(397))), 1.0), (-500.0)), 1.403592217853e217);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3128] != 0.0)) {
            s.store_scalar(334, 1.403592217853e217);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3128] != 0.0))) {
            s.store_mul_ad_rhs(781, 154, A::neg(s.ad_value(397)));
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3128] != 0.0))) {
            s.store_scalar(229, 1.0);
        }

        let mut assign91250_loop_guard: usize = 0;
        while {
            let assign91250_cond_e139909: f64 = if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3128] != 0.0))) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            assign91250_cond_e139909 != 0.0
        } {
            assign91250_loop_guard += 1;
            assert!(assign91250_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3128] != 0.0))) {
                s.store_scale(229, 229, 1.14200738981568e26);
            }
            if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3128] != 0.0))) {
                s.store_offset(781, 781, (-60.0));
            }
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3128] != 0.0))) {
            s.store_mul_ad_rhs(229, 229, A::exp(s.ad_value(781)));
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3128] != 0.0))) {
            s.copy_ad(334, 229);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) {
            s.store_offset_ad(781, A::offset(A::scale(A::neg(s.ad_value(402)), 0.5), (-0.5)), (-1.0));
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) {
            s.store_scalar(782, (4.0 * 0.5));
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) {
            s.store_ad(782, &{
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) {
            s.store_sqrt_ad(782, A::add(A::square(s.ad_value(781)), s.ad_value(782)));
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) {
            s.store_scale_ad(334, A::offset(A::div(s.ad_value(781), s.ad_value(782)), 1.0), 0.5);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) {
            s.store_offset_ad(335, A::scale(A::add(s.ad_value(781), s.ad_value(782)), 0.5), 0.5);
        }

        s.v[3129] = if (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (s.v[3129] != 0.0)) {
            s.store_add_ad_lhs(781, A::add(s.ad_value(402), s.ad_value(397)), 335);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (s.v[3129] != 0.0)) {
            s.store_square(722, 781);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (s.v[3129] != 0.0)) {
            s.store_square(723, 335);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (s.v[3129] != 0.0)) {
            s.store_scalar(724, 1.0);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (s.v[3129] != 0.0)) {
            s.store_scalar(725, 1.0);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (s.v[3129] != 0.0)) {
            s.store_scalar(719, 0.0);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (s.v[3129] != 0.0)) {
            s.store_scalar(720, 0.0);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (s.v[3129] != 0.0)) {
            s.store_scalar(770, 0.0);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (s.v[3129] != 0.0)) {
            s.store_scalar(726, 0.0);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (s.v[3129] != 0.0)) {
            s.store_mul(724, 724, 722);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (s.v[3129] != 0.0)) {
            s.store_mul(725, 725, 723);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (s.v[3129] != 0.0)) {
            s.store_add(770, 724, 725);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (s.v[3129] != 0.0)) {
            s.copy_ad(726, 770);
        }

        s.v[3130] = if ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[3131] = if (1.0 == 1.0) { 1.0 } else { 0.0 };

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (s.v[3129] != 0.0)) && (s.v[3130] != 0.0)) && (s.v[3131] != 0.0)) {
            s.store_scalar(720, 1.0);
        }

        s.v[3132] = if (1.0 == 2.0) { 1.0 } else { 0.0 };

        if (((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (s.v[3129] != 0.0)) && (s.v[3130] != 0.0)) && (!(s.v[3131] != 0.0))) && (s.v[3132] != 0.0)) {
            s.store_scalar(720, 2.0);
        }

        s.v[3133] = if (1.0 == 4.0) { 1.0 } else { 0.0 };

        if ((((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (s.v[3129] != 0.0)) && (s.v[3130] != 0.0)) && (!(s.v[3131] != 0.0))) && (!(s.v[3132] != 0.0))) && (s.v[3133] != 0.0)) {
            s.store_scalar(720, 3.0);
        }

        s.v[3134] = if (1.0 == 8.0) { 1.0 } else { 0.0 };

        if (((((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (s.v[3129] != 0.0)) && (s.v[3130] != 0.0)) && (!(s.v[3131] != 0.0))) && (!(s.v[3132] != 0.0))) && (!(s.v[3133] != 0.0))) && (s.v[3134] != 0.0)) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (s.v[3129] != 0.0)) && (s.v[3130] != 0.0)) {
            s.store_scalar(719, 0.0);
        }

        let mut assign91580_loop_guard: usize = 0;
        while {
            let assign91580_cond_e140317: f64 = if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (s.v[3129] != 0.0)) && (s.v[3130] != 0.0)) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign91580_cond_e140317 != 0.0
        } {
            assign91580_loop_guard += 1;
            assert!(assign91580_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (s.v[3129] != 0.0)) && (s.v[3130] != 0.0)) {
                s.store_sqrt(726, 726);
            }
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (s.v[3129] != 0.0)) && (s.v[3130] != 0.0)) {
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (s.v[3129] != 0.0)) && (!(s.v[3130] != 0.0))) {
            s.store_ad(726, &{
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / 2.0))
                }
            });
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (s.v[3129] != 0.0)) {
            s.store_div_from_scalar(726, 1.0, 726);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (s.v[3129] != 0.0)) {
            s.store_mul_ad_lhs(780, A::mul(s.ad_value(781), s.ad_value(335)), 726);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (s.v[3129] != 0.0)) {
            s.store_div_ad_lhs(334, A::mul(A::mul(s.ad_value(335), s.ad_value(725)), s.ad_value(726)), 770);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (s.v[3129] != 0.0)) {
            s.store_add_ad_lhs(335, A::neg(s.ad_value(335)), 780);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (s.v[3129] != 0.0)) {
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (!(s.v[3129] != 0.0))) {
            s.store_add(335, 402, 397);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) && (!(s.v[3129] != 0.0))) {
            s.store_scalar(334, 1.0);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) {
            s.store_sub(397, 335, 402);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[406] != 0.0)) {
            s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);
        }

        s.v[3135] = if (s.v[402] < s.v[403]) { 1.0 } else { 0.0 };

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3135] != 0.0)) {
            s.store_mul_ad(271, A::scale(s.ad_value(155), 2.0), A::ln(A::div_from_scalar((-s.v[270]), s.ad_value(212))));
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3135] != 0.0)) {
            s.store_mul_ad_rhs(332, 154, A::add(s.ad_value(402), s.ad_value(397)));
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3135] != 0.0)) {
            s.store_div_from_scalar_ad(335, 1.0, A::mul(s.ad_value(154), s.ad_value(410)));
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3135] != 0.0)) {
            s.store_mul(333, 335, 413);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3135] != 0.0)) {
            s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3135] != 0.0)) {
            s.store_mul_ad_lhs(277, A::mul(A::scale(s.ad_value(279), 8.0), s.ad_value(279)), 279);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3135] != 0.0)) {
            s.store_sub_from_scalar_ad(278, (7.0 * 1.414213562373095), A::mul(A::scale(s.ad_value(333), 9.0), A::offset(s.ad_value(332), (-2.0))));
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3135] != 0.0)) {
            s.store_square(276, 278);
        }

        s.v[3136] = if (s.v[277] < (s.v[276] * 1e-8)) { 1.0 } else { 0.0 };

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3135] != 0.0)) && (s.v[3136] != 0.0)) {
            s.store_div_ad_lhs(274, A::scale(s.ad_value(277), 0.5), 278);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3135] != 0.0)) && (!(s.v[3136] != 0.0))) {
            s.store_sqrt_ad(275, A::add(s.ad_value(277), s.ad_value(276)));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3135] != 0.0)) && (!(s.v[3136] != 0.0))) {
            s.store_sub(274, 275, 278);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3135] != 0.0)) {
            s.store_powf(273, 274, 0.3333333333333333);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3135] != 0.0)) {
            s.store_add_ad(272, A::add(A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), A::scale(s.ad_value(273), 2.0)), A::mul(A::scale(s.ad_value(273), 1.414213562373095), s.ad_value(273)));
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3135] != 0.0)) {
            s.store_div(116, 272, 273);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3135] != 0.0)) {
            s.store_mul(335, 116, 155);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3135] != 0.0)) {
            s.store_div(336, 335, 271);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3135] != 0.0)) {
            s.store_sqrt_ad(337, A::offset(A::square(s.ad_value(336)), 1.0));
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3135] != 0.0)) {
            s.store_sub_ad_lhs(404, A::div(s.ad_value(335), s.ad_value(337)), 397);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3135] != 0.0)) {
            s.store_sub(336, 402, 404);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3135] != 0.0)) {
            s.store_mul(398, 413, 336);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3135] != 0.0)) {
            s.copy_ad(354, 398);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3135] != 0.0)) {
            s.copy_ad(3118, 404);
        }

        s.v[3137] = if ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0))) { 1.0 } else { 0.0 };

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3137] != 0.0)) {
            s.store_add_ad_rhs(89, 402, A::scale(A::mul(s.ad_value(213), s.ad_value(154)), 0.5));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (!(s.v[3137] != 0.0))) {
            s.store_offset_ad(332, A::div(A::scale(A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0), A::mul(s.ad_value(213), s.ad_value(156))), 1.0);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (!(s.v[3137] != 0.0))) {
            s.store_add_ad_rhs(89, 402, A::mul(A::scale(A::mul(s.ad_value(213), s.ad_value(154)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332)))));
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) {
            s.store_mul_ad_rhs(116, 154, A::add(s.ad_value(89), s.ad_value(397)));
        }

        s.v[3138] = if (s.v[116] >= 3.0) { 1.0 } else { 0.0 };

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3138] != 0.0)) {
            s.store_exp_ad(333, A::neg(s.ad_value(116)));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3138] != 0.0)) {
            s.store_offset_ad(332, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), s.ad_value(333)), 4.0), A::mul(s.ad_value(213), s.ad_value(156))), 1.0);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3138] != 0.0)) {
            s.store_add_ad_rhs(89, 402, A::mul(A::scale(A::mul(s.ad_value(213), s.ad_value(154)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332)))));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3138] != 0.0)) {
            s.store_mul_ad_rhs(116, 154, A::add(s.ad_value(89), s.ad_value(397)));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3138] != 0.0)) {
            s.store_exp_ad(333, A::neg(s.ad_value(116)));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3138] != 0.0)) {
            s.store_offset_ad(332, A::div(A::scale(A::add(A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), s.ad_value(333)), 4.0), A::mul(s.ad_value(213), s.ad_value(156))), 1.0);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3138] != 0.0)) {
            s.store_add_ad_rhs(89, 402, A::mul(A::scale(A::mul(s.ad_value(213), s.ad_value(154)), 0.5), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332)))));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3138] != 0.0)) {
            s.store_mul_ad_rhs(116, 154, A::add(s.ad_value(89), s.ad_value(397)));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (!(s.v[3138] != 0.0))) {
            s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (!(s.v[3138] != 0.0))) {
            s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (!(s.v[3138] != 0.0))) {
            s.store_offset_ad(436, A::div_from_scalar(1.0, A::mul(s.ad_value(154), s.ad_value(212))), (1.0 / ((2.0) as f64).sqrt()));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (!(s.v[3138] != 0.0))) {
            s.store_div_ad_lhs(437, A::neg(A::add(s.ad_value(402), s.ad_value(397))), 212);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (!(s.v[3138] != 0.0))) {
            s.store_add_ad(441, A::sub(A::div(A::mul(A::square(s.ad_value(435)), s.ad_value(435)), A::mul(A::mul(A::scale(s.ad_value(434), 27.0), s.ad_value(434)), s.ad_value(434))), A::div(A::mul(s.ad_value(435), s.ad_value(436)), A::mul(A::scale(s.ad_value(434), 6.0), s.ad_value(434)))), A::div(s.ad_value(437), A::scale(s.ad_value(434), 2.0)));
        }

    }

    pub(super) fn stamp_transient_block_83(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (!(s.v[3138] != 0.0))) {
            s.store_div_ad(440, A::sub(A::mul(A::scale(s.ad_value(434), 3.0), s.ad_value(436)), A::square(s.ad_value(435))), A::mul(A::scale(s.ad_value(434), 9.0), s.ad_value(434)));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (!(s.v[3138] != 0.0))) {
            s.store_sqrt_ad(339, A::add(A::square(s.ad_value(441)), A::mul(A::square(s.ad_value(440)), s.ad_value(440))));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (!(s.v[3138] != 0.0))) {
            s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (!(s.v[3138] != 0.0))) {
            s.store_neg_ad(438, A::powf(A::add(s.ad_value(441), s.ad_value(339)), 0.3333333333333333));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (!(s.v[3138] != 0.0))) {
            s.store_sub_ad(116, A::add(s.ad_value(439), s.ad_value(438)), A::div(s.ad_value(435), A::scale(s.ad_value(434), 3.0)));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (!(s.v[3138] != 0.0))) {
            s.store_sub_ad_lhs(89, A::mul(s.ad_value(116), s.ad_value(155)), 397);
        }

        s.v[3139] = if (p.p33 > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) {
            s.store_offset_ad(442, A::add(s.ad_value(402), s.ad_value(397)), 0.1);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) {
            s.store_mul(222, 405, 229);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) {
            s.store_mul(443, 405, 229);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) {
            s.store_mul(334, 156, 213);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) {
            s.store_mul(444, 154, 442);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) {
            s.store_add_ad(447, A::sub(A::ln(A::add(A::mul(s.ad_value(443), s.ad_value(334)), A::square(s.ad_value(444)))), A::ln(A::mul(s.ad_value(405), s.ad_value(334)))), A::mul(s.ad_value(154), s.ad_value(397)));
        }

        s.v[3140] = if (p.p33 == 2.0) { 1.0 } else { 0.0 };

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3140] != 0.0)) {
            s.store_offset_ad(781, A::sub(s.ad_value(444), s.ad_value(447)), (-1.0));
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3140] != 0.0)) {
            s.store_scale(782, 444, 4.0);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3140] != 0.0)) {
            s.store_ad(782, &{
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3140] != 0.0)) {
            s.store_sqrt_ad(782, A::add(A::square(s.ad_value(781)), s.ad_value(782)));
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3140] != 0.0)) {
            s.store_scale_ad(335, A::offset(A::div(s.ad_value(781), s.ad_value(782)), 1.0), 0.5);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3140] != 0.0)) {
            s.store_sub_ad_rhs(447, 444, A::scale(A::add(s.ad_value(781), s.ad_value(782)), 0.5));
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (!(s.v[3140] != 0.0))) {
            s.store_ad(447, &{
                if (s.v[447] <= s.v[444]) {
                    s.ad_value(447)
                } else {
                    s.ad_value(444)
                }
            });
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) {
            s.store_ad(447, &{
                if (s.v[447] >= 0.0) {
                    s.ad_value(447)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) {
            s.store_sub(444, 444, 447);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) {
            s.store_add_ad_rhs(444, 444, A::scale(s.ad_value(154), 0.1));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) {
            s.store_sub_ad(335, A::ln(A::add(A::mul(s.ad_value(443), s.ad_value(334)), A::square(s.ad_value(444)))), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) {
            s.store_add_ad_rhs(446, 335, A::mul(s.ad_value(154), s.ad_value(397)));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) {
            s.store_ad(446, &{
                if (s.v[446] >= 0.0) {
                    s.ad_value(446)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) {
            s.copy_ad(445, 116);
        }

        s.v[3141] = if (p.p33 == 2.0) { 1.0 } else { 0.0 };

        s.v[3142] = if ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0)) { 1.0 } else { 0.0 };

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) {
            s.store_add_ad(781, A::sub(s.ad_value(445), s.ad_value(446)), A::scale(s.ad_value(446), 0.2));
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) {
            s.store_square(722, 781);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) {
            s.store_mul_ad(723, A::scale(s.ad_value(446), 0.2), A::scale(s.ad_value(446), 0.2));
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) {
            s.store_scalar(724, 1.0);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) {
            s.store_scalar(725, 1.0);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) {
            s.store_scalar(719, 0.0);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) {
            s.store_scalar(720, 0.0);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) {
            s.store_scalar(770, 0.0);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) {
            s.store_scalar(726, 0.0);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) {
            s.store_mul(724, 724, 722);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) {
            s.store_mul(725, 725, 723);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) {
            s.store_mul(724, 724, 722);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) {
            s.store_mul(725, 725, 723);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) {
            s.store_add(770, 724, 725);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) {
            s.copy_ad(726, 770);
        }

        s.v[3143] = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[3144] = if (2.0 == 1.0) { 1.0 } else { 0.0 };

        if ((((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) && (s.v[3143] != 0.0)) && (s.v[3144] != 0.0)) {
            s.store_scalar(720, 1.0);
        }

        s.v[3145] = if (2.0 == 2.0) { 1.0 } else { 0.0 };

        if (((((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) && (s.v[3143] != 0.0)) && (!(s.v[3144] != 0.0))) && (s.v[3145] != 0.0)) {
            s.store_scalar(720, 2.0);
        }

        s.v[3146] = if (2.0 == 4.0) { 1.0 } else { 0.0 };

        if ((((((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) && (s.v[3143] != 0.0)) && (!(s.v[3144] != 0.0))) && (!(s.v[3145] != 0.0))) && (s.v[3146] != 0.0)) {
            s.store_scalar(720, 3.0);
        }

        s.v[3147] = if (2.0 == 8.0) { 1.0 } else { 0.0 };

        if (((((((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) && (s.v[3143] != 0.0)) && (!(s.v[3144] != 0.0))) && (!(s.v[3145] != 0.0))) && (!(s.v[3146] != 0.0))) && (s.v[3147] != 0.0)) {
            s.store_scalar(720, 4.0);
        }

        if (((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) && (s.v[3143] != 0.0)) {
            s.store_scalar(719, 0.0);
        }

        let mut assign92710_loop_guard: usize = 0;
        while {
            let assign92710_cond_e142078: f64 = if ((((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) && (s.v[3143] != 0.0)) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign92710_cond_e142078 != 0.0
        } {
            assign92710_loop_guard += 1;
            assert!(assign92710_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) && (s.v[3143] != 0.0)) {
                s.store_sqrt(726, 726);
            }
            if (((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) && (s.v[3143] != 0.0)) {
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) && (!(s.v[3143] != 0.0))) {
            s.store_ad(726, &{
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) {
            s.store_div_from_scalar(726, 1.0, 726);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) {
            s.store_mul_ad_lhs(780, A::mul(s.ad_value(781), A::scale(s.ad_value(446), 0.2)), 726);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) {
            s.store_div_ad_lhs(335, A::mul(A::mul(A::scale(s.ad_value(446), 0.2), s.ad_value(725)), s.ad_value(726)), 770);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) {
            s.store_add_ad_lhs(116, A::sub(s.ad_value(446), A::scale(s.ad_value(446), 0.2)), 780);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (s.v[3142] != 0.0)) {
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (!(s.v[3142] != 0.0))) {
            s.copy_ad(116, 445);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (s.v[3141] != 0.0)) && (!(s.v[3142] != 0.0))) {
            s.store_scalar(335, 1.0);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3139] != 0.0)) && (!(s.v[3141] != 0.0))) {
            s.store_ad(116, &{
                if (s.v[445] <= s.v[446]) {
                    s.ad_value(445)
                } else {
                    s.ad_value(446)
                }
            });
        }

        s.v[3148] = if (p.p33 == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) {
            s.store_sub_ad_lhs(404, A::mul(s.ad_value(116), s.ad_value(155)), 397);
        }

        s.v[3149] = if (s.v[411] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) && (s.v[3149] != 0.0)) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) && (!(s.v[3149] != 0.0))) {
            s.store_sqrt_ad(782, A::offset(A::mul(A::offset(s.ad_value(729), p.p137), A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1)));
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) && (!(s.v[3149] != 0.0))) {
            s.store_scale_ad(343, A::offset(A::div(A::offset(s.ad_value(729), p.p137), s.ad_value(782)), 1.0), 0.5);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) && (!(s.v[3149] != 0.0))) {
            s.store_scale_ad(336, A::add(A::offset(s.ad_value(729), p.p137), s.ad_value(782)), 0.5);
        }

        s.v[3150] = if (s.v[336] < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) && (!(s.v[3149] != 0.0))) && (s.v[3150] != 0.0)) {
            s.store_scalar(336, 0.0);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) && (!(s.v[3149] != 0.0))) && (s.v[3150] != 0.0)) {
            s.store_scalar(343, 0.0);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) && (!(s.v[3149] != 0.0))) {
            s.store_scale_ad(600, A::sqrt(A::mul(s.ad_value(651), s.ad_value(336))), p.p432);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) && (!(s.v[3149] != 0.0))) {
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) {
            s.store_sqrt_ad(782, A::offset(A::square(s.ad_value(336)), ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01))));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) {
            s.store_scale_ad(343, A::offset(A::div(s.ad_value(336), s.ad_value(782)), 1.0), 0.5);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) {
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.v[3151] = if (s.v[336] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) && (s.v[3151] != 0.0)) {
            s.store_scalar(336, 0.0);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) && (s.v[3151] != 0.0)) {
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) {
            s.copy_ad(386, 336);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) {
            s.store_scale_ad(418, A::scale(A::mul(A::mul(s.ad_value(3111), s.ad_value(386)), s.ad_value(386)), 0.5), 9662367879.197212);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) {
            s.store_sqrt_ad(334, A::mul(A::scale(s.ad_value(154), 2.0), s.ad_value(418)));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) {
            s.store_scale_ad(335, A::add(A::limited_exp(s.ad_value(334)), A::limited_exp(A::neg(s.ad_value(334)))), 0.5);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) {
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) {
            s.store_add(414, 404, 397);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) {
            s.store_mul_ad_rhs(333, 419, A::sub(s.ad_value(414), s.ad_value(418)));
        }

        s.v[3152] = if (s.v[333] < 60.0) { 1.0 } else { 0.0 };

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) && (s.v[3152] != 0.0)) {
            s.store_exp(335, 333);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) && (s.v[3152] != 0.0)) {
            s.store_exp_ad(334, A::mul(A::neg(s.ad_value(419)), s.ad_value(418)));
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) && (s.v[3152] != 0.0)) {
            s.store_sub(336, 335, 334);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) && (s.v[3152] != 0.0)) {
            s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) && (!(s.v[3152] != 0.0))) {
            s.store_sub(416, 414, 418);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) {
            s.store_mul(415, 154, 416);
        }

        s.v[3153] = if ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0)) { 1.0 } else { 0.0 };

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) && (s.v[3153] != 0.0)) {
            s.store_offset(3117, 3117, 1.0);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3148] != 0.0)) && (s.v[3153] != 0.0)) {
            s.copy_ad(116, 447);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) {
            s.store_sub_ad_lhs(404, A::mul(s.ad_value(116), s.ad_value(155)), 397);
        }

        s.v[3154] = if (((s.v[116]) as f64).abs() > 1e-6) { 1.0 } else { 0.0 };

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3154] != 0.0)) {
            s.store_add_ad(335, A::offset(s.ad_value(116), (-1.0)), A::exp(A::neg(s.ad_value(116))));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3154] != 0.0)) {
            s.store_sqrt(336, 335);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (!(s.v[3154] != 0.0))) {
            s.store_mul_ad(336, A::scale(s.ad_value(116), 0.7071067811865475), A::sqrt(A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333))));
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) {
            s.store_mul(354, 410, 336);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) {
            s.store_mul_ad_rhs(398, 413, A::sub(s.ad_value(402), s.ad_value(404)));
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) {
            s.store_div(3155, 354, 3111);
        }

        s.v[3157] = if (p.p33 == 2.0) { 1.0 } else { 0.0 };

        s.v[3158] = if ((s.v[3155] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0)) { 1.0 } else { 0.0 };

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) {
            s.store_add_ad(781, A::sub(s.ad_value(3155), s.ad_value(386)), A::scale(s.ad_value(386), 0.1));
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) {
            s.store_square(722, 781);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) {
            s.store_mul_ad(723, A::scale(s.ad_value(386), 0.1), A::scale(s.ad_value(386), 0.1));
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) {
            s.store_scalar(724, 1.0);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) {
            s.store_scalar(725, 1.0);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) {
            s.store_scalar(719, 0.0);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) {
            s.store_scalar(720, 0.0);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) {
            s.store_scalar(770, 0.0);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) {
            s.store_scalar(726, 0.0);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) {
            s.store_mul(724, 724, 722);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) {
            s.store_mul(725, 725, 723);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) {
            s.store_mul(724, 724, 722);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) {
            s.store_mul(725, 725, 723);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) {
            s.store_add(770, 724, 725);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) {
            s.copy_ad(726, 770);
        }

        s.v[3159] = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_84(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[3160] = if (2.0 == 1.0) { 1.0 } else { 0.0 };

        if (((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) && (s.v[3159] != 0.0)) && (s.v[3160] != 0.0)) {
            s.store_scalar(720, 1.0);
        }

        s.v[3161] = if (2.0 == 2.0) { 1.0 } else { 0.0 };

        if ((((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) && (s.v[3159] != 0.0)) && (!(s.v[3160] != 0.0))) && (s.v[3161] != 0.0)) {
            s.store_scalar(720, 2.0);
        }

        s.v[3162] = if (2.0 == 4.0) { 1.0 } else { 0.0 };

        if (((((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) && (s.v[3159] != 0.0)) && (!(s.v[3160] != 0.0))) && (!(s.v[3161] != 0.0))) && (s.v[3162] != 0.0)) {
            s.store_scalar(720, 3.0);
        }

        s.v[3163] = if (2.0 == 8.0) { 1.0 } else { 0.0 };

        if ((((((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) && (s.v[3159] != 0.0)) && (!(s.v[3160] != 0.0))) && (!(s.v[3161] != 0.0))) && (!(s.v[3162] != 0.0))) && (s.v[3163] != 0.0)) {
            s.store_scalar(720, 4.0);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) && (s.v[3159] != 0.0)) {
            s.store_scalar(719, 0.0);
        }

        let mut assign93520_loop_guard: usize = 0;
        while {
            let assign93520_cond_e143286: f64 = if (((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) && (s.v[3159] != 0.0)) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign93520_cond_e143286 != 0.0
        } {
            assign93520_loop_guard += 1;
            assert!(assign93520_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) && (s.v[3159] != 0.0)) {
                s.store_sqrt(726, 726);
            }
            if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) && (s.v[3159] != 0.0)) {
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) && (!(s.v[3159] != 0.0))) {
            s.store_ad(726, &{
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) {
            s.store_div_from_scalar(726, 1.0, 726);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) {
            s.store_mul_ad_lhs(780, A::mul(s.ad_value(781), A::scale(s.ad_value(386), 0.1)), 726);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) {
            s.store_div_ad_lhs(334, A::mul(A::mul(A::scale(s.ad_value(386), 0.1), s.ad_value(725)), s.ad_value(726)), 770);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) {
            s.store_add_ad_lhs(335, A::sub(s.ad_value(386), A::scale(s.ad_value(386), 0.1)), 780);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3158] != 0.0)) {
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (!(s.v[3158] != 0.0))) {
            s.copy_ad(335, 3155);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (!(s.v[3158] != 0.0))) {
            s.store_scalar(334, 1.0);
        }

        s.v[3164] = if (s.v[334] < 1.0) { 1.0 } else { 0.0 };

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3157] != 0.0)) && (s.v[3164] != 0.0)) {
            s.store_offset(3117, 3117, 2.0);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (!(s.v[3157] != 0.0))) {
            s.store_ad(335, &{
                if (s.v[3155] <= s.v[386]) {
                    s.ad_value(3155)
                } else {
                    s.ad_value(386)
                }
            });
        }

        s.v[3165] = if (s.v[3155] >= s.v[386]) { 1.0 } else { 0.0 };

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (!(s.v[3157] != 0.0))) && (s.v[3165] != 0.0)) {
            s.store_offset(3117, 3117, 2.0);
        }

        s.v[3166] = if (s.v[3117] >= 2.0) { 1.0 } else { 0.0 };

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) {
            s.copy_ad(3156, 404);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) {
            s.store_mul(354, 335, 3111);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) {
            s.store_sub_ad_rhs(404, 402, A::div(s.ad_value(354), s.ad_value(413)));
        }

        s.v[3167] = if (p.p33 == 2.0) { 1.0 } else { 0.0 };

        s.v[3168] = if ((s.v[404] > (s.v[3156] - 0.1)) && (0.1 >= 0.0)) { 1.0 } else { 0.0 };

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) {
            s.store_offset_ad(781, A::sub(s.ad_value(404), s.ad_value(3156)), 0.1);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) {
            s.store_square(722, 781);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) {
            s.store_scalar(723, (0.1 * 0.1));
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) {
            s.store_scalar(724, 1.0);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) {
            s.store_scalar(725, 1.0);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) {
            s.store_scalar(719, 0.0);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) {
            s.store_scalar(720, 0.0);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) {
            s.store_scalar(770, 0.0);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) {
            s.store_scalar(726, 0.0);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) {
            s.store_mul(724, 724, 722);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) {
            s.store_mul(725, 725, 723);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) {
            s.store_mul(724, 724, 722);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) {
            s.store_mul(725, 725, 723);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) {
            s.store_add(770, 724, 725);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) {
            s.copy_ad(726, 770);
        }

        s.v[3169] = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[3170] = if (2.0 == 1.0) { 1.0 } else { 0.0 };

        if ((((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) && (s.v[3169] != 0.0)) && (s.v[3170] != 0.0)) {
            s.store_scalar(720, 1.0);
        }

        s.v[3171] = if (2.0 == 2.0) { 1.0 } else { 0.0 };

        if (((((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) && (s.v[3169] != 0.0)) && (!(s.v[3170] != 0.0))) && (s.v[3171] != 0.0)) {
            s.store_scalar(720, 2.0);
        }

        s.v[3172] = if (2.0 == 4.0) { 1.0 } else { 0.0 };

        if ((((((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) && (s.v[3169] != 0.0)) && (!(s.v[3170] != 0.0))) && (!(s.v[3171] != 0.0))) && (s.v[3172] != 0.0)) {
            s.store_scalar(720, 3.0);
        }

        s.v[3173] = if (2.0 == 8.0) { 1.0 } else { 0.0 };

        if (((((((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) && (s.v[3169] != 0.0)) && (!(s.v[3170] != 0.0))) && (!(s.v[3171] != 0.0))) && (!(s.v[3172] != 0.0))) && (s.v[3173] != 0.0)) {
            s.store_scalar(720, 4.0);
        }

        if (((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) && (s.v[3169] != 0.0)) {
            s.store_scalar(719, 0.0);
        }

        let mut assign93970_loop_guard: usize = 0;
        while {
            let assign93970_cond_e143968: f64 = if ((((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) && (s.v[3169] != 0.0)) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign93970_cond_e143968 != 0.0
        } {
            assign93970_loop_guard += 1;
            assert!(assign93970_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) && (s.v[3169] != 0.0)) {
                s.store_sqrt(726, 726);
            }
            if (((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) && (s.v[3169] != 0.0)) {
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) && (!(s.v[3169] != 0.0))) {
            s.store_ad(726, &{
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) {
            s.store_div_from_scalar(726, 1.0, 726);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) {
            s.store_mul_ad_lhs(780, A::scale(s.ad_value(781), 0.1), 726);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) {
            s.store_div_ad_lhs(334, A::mul(A::scale(s.ad_value(725), 0.1), s.ad_value(726)), 770);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) {
            s.store_add_ad_lhs(404, A::offset(s.ad_value(3156), (-0.1)), 780);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (s.v[3168] != 0.0)) {
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (!(s.v[3168] != 0.0))) {
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (s.v[3167] != 0.0)) && (!(s.v[3168] != 0.0))) {
            s.store_scalar(334, 1.0);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3166] != 0.0)) && (!(s.v[3167] != 0.0))) {
            s.store_ad(404, &{
                if (s.v[404] <= s.v[3156]) {
                    s.ad_value(404)
                } else {
                    s.ad_value(3156)
                }
            });
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) {
            s.copy_ad(3118, 404);
        }

        s.v[3174] = if (p.p33 == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) {
            s.store_scalar(79, 0.0);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) {
            s.store_sqrt_ad(982, A::scale(A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(3111)), s.ad_value(155)), 2.0));
        }

        s.v[3175] = if (s.v[411] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (s.v[3175] != 0.0)) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3175] != 0.0))) {
            s.store_sqrt_ad(782, A::offset(A::mul(A::offset(s.ad_value(729), p.p137), A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1)));
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3175] != 0.0))) {
            s.store_scale_ad(343, A::offset(A::div(A::offset(s.ad_value(729), p.p137), s.ad_value(782)), 1.0), 0.5);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3175] != 0.0))) {
            s.store_scale_ad(336, A::add(A::offset(s.ad_value(729), p.p137), s.ad_value(782)), 0.5);
        }

        s.v[3176] = if (s.v[336] < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3175] != 0.0))) && (s.v[3176] != 0.0)) {
            s.store_scalar(336, 0.0);
        }

        if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3175] != 0.0))) && (s.v[3176] != 0.0)) {
            s.store_scalar(343, 0.0);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3175] != 0.0))) {
            s.store_scale_ad(600, A::sqrt(A::mul(s.ad_value(651), s.ad_value(336))), p.p432);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3175] != 0.0))) {
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) {
            s.store_sqrt_ad(782, A::offset(A::square(s.ad_value(336)), ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01))));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) {
            s.store_scale_ad(343, A::offset(A::div(s.ad_value(336), s.ad_value(782)), 1.0), 0.5);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) {
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.v[3177] = if (s.v[336] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (s.v[3177] != 0.0)) {
            s.store_scalar(336, 0.0);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (s.v[3177] != 0.0)) {
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) {
            s.copy_ad(386, 336);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) {
            s.store_scale_ad(418, A::scale(A::mul(A::mul(s.ad_value(3111), s.ad_value(386)), s.ad_value(386)), 0.5), 9662367879.197212);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) {
            s.store_sqrt_ad(334, A::mul(A::scale(s.ad_value(154), 2.0), s.ad_value(418)));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) {
            s.store_scale_ad(335, A::add(A::limited_exp(s.ad_value(334)), A::limited_exp(A::neg(s.ad_value(334)))), 0.5);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) {
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) {
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_85(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let mut assign94330_loop_guard: usize = 0;
        while {
            let assign94330_cond_e144560: f64 = (s.v[421] + 1.0);
            let assign94330_cond_e144562: f64 = if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (s.v[97] <= assign94330_cond_e144560)) { 1.0 } else { 0.0 };
            assign94330_cond_e144562 != 0.0
        } {
            assign94330_loop_guard += 1;
            assert!(assign94330_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) {
                s.store_add(414, 404, 397);
            }
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) {
                s.store_mul(116, 154, 414);
            }
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) {
                s.store_mul_ad_rhs(333, 419, A::sub(s.ad_value(414), s.ad_value(418)));
            }
            s.v[3179] = if (s.v[333] < 60.0) { 1.0 } else { 0.0 };
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (s.v[3179] != 0.0)) {
                s.store_exp(335, 333);
            }
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (s.v[3179] != 0.0)) {
                s.store_exp_ad(334, A::mul(A::neg(s.ad_value(419)), s.ad_value(418)));
            }
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (s.v[3179] != 0.0)) {
                s.store_sub(336, 335, 334);
            }
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (s.v[3179] != 0.0)) {
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
            }
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (s.v[3179] != 0.0)) {
                s.store_div_ad_rhs(417, 335, A::offset(s.ad_value(336), 1.0));
            }
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3179] != 0.0))) {
                s.store_sub(416, 414, 418);
            }
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3179] != 0.0))) {
                s.store_scalar(417, 1.0);
            }
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) {
                s.store_mul(415, 154, 416);
            }
            s.v[3180] = if (s.v[116] < 0.0) { 1.0 } else { 0.0 };
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (s.v[3180] != 0.0)) {
                s.store_scalar(334, (-0.7071067811865475));
            }
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (s.v[3180] != 0.0)) {
                s.store_mul(223, 116, 334);
            }
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (s.v[3180] != 0.0)) {
                s.store_mul(420, 154, 334);
            }
            s.v[3181] = if (s.v[116] < 1e-6) { 1.0 } else { 0.0 };
            if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3180] != 0.0))) && (s.v[3181] != 0.0)) {
                s.store_mul_ad(334, A::scale(A::square(s.ad_value(116)), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(116), 0.3333333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(116), 0.25), A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.2)))))));
            }
            if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3180] != 0.0))) && (s.v[3181] != 0.0)) {
                s.store_mul_ad_rhs(335, 116, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(116), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(116), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.25)))))));
            }
            if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3180] != 0.0))) && (s.v[3181] != 0.0)) {
                s.store_mul_ad(336, A::scale(A::square(s.ad_value(415)), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(415), 0.3333333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(415), 0.25), A::sub_from_scalar(1.0, A::scale(s.ad_value(415), 0.2)))))));
            }
            if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3180] != 0.0))) && (s.v[3181] != 0.0)) {
                s.store_mul_ad_rhs(337, 415, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(415), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(415), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(415), 0.25)))))));
            }
            if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3180] != 0.0))) && (s.v[3181] != 0.0)) {
                s.store_sub(338, 334, 336);
            }
            s.v[3182] = if (s.v[338] > 0.0) { 1.0 } else { 0.0 };
            if (((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3180] != 0.0))) && (s.v[3181] != 0.0)) && (s.v[3182] != 0.0)) {
                s.store_sqrt(223, 338);
            }
            if (((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3180] != 0.0))) && (s.v[3181] != 0.0)) && (s.v[3182] != 0.0)) {
                s.store_div_ad_lhs(420, A::mul(A::scale(s.ad_value(154), 0.5), A::sub(s.ad_value(335), A::mul(s.ad_value(417), s.ad_value(337)))), 223);
            }
            if (((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3180] != 0.0))) && (s.v[3181] != 0.0)) && (!(s.v[3182] != 0.0))) {
                s.store_scalar(223, 0.0);
            }
            if (((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3180] != 0.0))) && (s.v[3181] != 0.0)) && (!(s.v[3182] != 0.0))) {
                s.store_scalar(420, 0.0);
            }
            if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3180] != 0.0))) && (!(s.v[3181] != 0.0))) {
                s.store_exp_ad(334, A::neg(s.ad_value(116)));
            }
            if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3180] != 0.0))) && (!(s.v[3181] != 0.0))) {
                s.store_exp_ad(335, A::neg(s.ad_value(415)));
            }
            if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3180] != 0.0))) && (!(s.v[3181] != 0.0))) {
                s.store_add_ad(338, A::sub(s.ad_value(116), s.ad_value(415)), A::sub(s.ad_value(334), s.ad_value(335)));
            }
            s.v[3183] = if (s.v[338] > 0.0) { 1.0 } else { 0.0 };
            if (((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3180] != 0.0))) && (!(s.v[3181] != 0.0))) && (s.v[3183] != 0.0)) {
                s.store_sqrt(223, 338);
            }
            if (((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3180] != 0.0))) && (!(s.v[3181] != 0.0))) && (s.v[3183] != 0.0)) {
                s.store_div_ad_lhs(420, A::mul(A::scale(s.ad_value(154), 0.5), A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul(s.ad_value(417), A::sub_from_scalar(1.0, s.ad_value(335))))), 223);
            }
            if (((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3180] != 0.0))) && (!(s.v[3181] != 0.0))) && (!(s.v[3183] != 0.0))) {
                s.store_scalar(223, 0.0);
            }
            if (((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3180] != 0.0))) && (!(s.v[3181] != 0.0))) && (!(s.v[3183] != 0.0))) {
                s.store_scalar(420, 0.0);
            }
            s.v[3184] = if (s.v[116] < 0.0) { 1.0 } else { 0.0 };
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (s.v[3184] != 0.0)) {
                s.store_scalar(214, 0.0);
            }
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (s.v[3184] != 0.0)) {
                s.store_scalar(215, 0.0);
            }
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (s.v[3184] != 0.0)) {
                s.store_neg(216, 223);
            }
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (s.v[3184] != 0.0)) {
                s.store_neg(217, 420);
            }
            s.v[3185] = if (s.v[116] < 60.0) { 1.0 } else { 0.0 };
            s.v[3186] = if (s.v[116] < 5e-5) { 1.0 } else { 0.0 };
            if (((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3184] != 0.0))) && (s.v[3185] != 0.0)) && (s.v[3186] != 0.0)) {
                s.store_mul_ad(334, A::scale(A::square(s.ad_value(116)), 0.5), A::offset(A::mul(A::scale(s.ad_value(116), 0.3333333333333333), A::offset(A::mul(A::scale(s.ad_value(116), 0.25), A::offset(A::scale(s.ad_value(116), 0.2), 1.0)), 1.0)), 1.0));
            }
            if (((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3184] != 0.0))) && (s.v[3185] != 0.0)) && (s.v[3186] != 0.0)) {
                s.store_mul_ad_rhs(335, 116, A::offset(A::mul(A::scale(s.ad_value(116), 0.5), A::offset(A::mul(A::scale(s.ad_value(116), 0.3333333333333333), A::offset(A::scale(s.ad_value(116), 0.25), 1.0)), 1.0)), 1.0));
            }
            if (((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3184] != 0.0))) && (s.v[3185] != 0.0)) && (s.v[3186] != 0.0)) {
                s.store_mul(214, 222, 334);
            }
            if (((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3184] != 0.0))) && (s.v[3185] != 0.0)) && (s.v[3186] != 0.0)) {
                s.store_mul_ad_lhs(215, A::mul(s.ad_value(222), s.ad_value(335)), 154);
            }
            if (((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3184] != 0.0))) && (s.v[3185] != 0.0)) && (!(s.v[3186] != 0.0))) {
                s.store_exp(227, 116);
            }
            if (((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3184] != 0.0))) && (s.v[3185] != 0.0)) && (!(s.v[3186] != 0.0))) {
                s.store_offset(335, 227, (-1.0));
            }
            if (((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3184] != 0.0))) && (s.v[3185] != 0.0)) && (!(s.v[3186] != 0.0))) {
                s.store_mul_ad_rhs(214, 222, A::sub(s.ad_value(335), s.ad_value(116)));
            }
            if (((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3184] != 0.0))) && (s.v[3185] != 0.0)) && (!(s.v[3186] != 0.0))) {
                s.store_mul_ad_lhs(215, A::mul(s.ad_value(222), s.ad_value(154)), 335);
            }
            if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3184] != 0.0))) && (!(s.v[3185] != 0.0))) {
                s.store_exp_ad(231, A::mul(s.ad_value(154), s.ad_value(404)));
            }
            if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3184] != 0.0))) && (!(s.v[3185] != 0.0))) {
                s.store_mul_ad_rhs(214, 405, A::sub(s.ad_value(231), A::mul(s.ad_value(229), A::offset(s.ad_value(116), 1.0))));
            }
            if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3184] != 0.0))) && (!(s.v[3185] != 0.0))) {
                s.store_mul_ad(215, A::mul(s.ad_value(405), s.ad_value(154)), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.v[3187] = if (s.v[214] > 0.0) { 1.0 } else { 0.0 };
            if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3184] != 0.0))) && (s.v[3187] != 0.0)) {
                s.store_sqrt_ad(216, A::add(A::square(s.ad_value(223)), s.ad_value(214)));
            }
            if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3184] != 0.0))) && (s.v[3187] != 0.0)) {
                s.store_div_ad_lhs(217, A::scale(A::add(A::mul(A::scale(s.ad_value(420), 2.0), s.ad_value(223)), s.ad_value(215)), 0.5), 216);
            }
            if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3184] != 0.0))) && (!(s.v[3187] != 0.0))) {
                s.copy_ad(216, 223);
            }
            if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3184] != 0.0))) && (!(s.v[3187] != 0.0))) {
                s.copy_ad(217, 420);
            }
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) {
                s.store_add_ad(232, A::sub(s.ad_value(404), s.ad_value(402)), A::mul(s.ad_value(212), s.ad_value(216)));
            }
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) {
                s.store_offset_ad(233, A::mul(s.ad_value(212), s.ad_value(217)), 1.0);
            }
            s.v[3188] = if (s.v[79] == 1.0) { 1.0 } else { 0.0 };
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (s.v[3188] != 0.0)) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3188] != 0.0))) {
                s.store_div_ad_lhs(236, A::neg(s.ad_value(232)), 233);
            }
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3188] != 0.0))) {
                s.store_scale_ad(93, A::offset({
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0), (0.5 * 0.1));
            }
            s.v[3189] = if (((s.v[236]) as f64).abs() > s.v[93]) { 1.0 } else { 0.0 };
            if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3188] != 0.0))) && (s.v[3189] != 0.0)) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3188] != 0.0))) {
                s.store_add(404, 404, 236);
            }
            s.v[3190] = if ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8)) { 1.0 } else { 0.0 };
            if ((((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) && (!(s.v[3188] != 0.0))) && (s.v[3190] != 0.0)) {
                s.store_scalar(79, 1.0);
            }
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) {
            s.store_mul(3109, 982, 223);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) {
            s.store_mul(3110, 3111, 3109);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) {
            s.store_offset_ad(100, A::div(s.ad_value(3110), s.ad_value(410)), (10.0 * 2.220446049250313e-16));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) {
            s.store_mul(354, 410, 100);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) {
            s.store_div_from_scalar_ad(335, 1.0, A::add(s.ad_value(216), s.ad_value(100)));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) {
            s.store_mul_ad_lhs(399, A::mul(s.ad_value(410), s.ad_value(214)), 335);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (!(s.v[3135] != 0.0))) && (s.v[3174] != 0.0)) {
            s.store_add(398, 354, 399);
        }

        s.v[3192] = if (p.p33 == 4.0) { 1.0 } else { 0.0 };

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
            s.store_exp_ad(229, A::mul(s.ad_value(154), A::neg(s.ad_value(397))));
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
            s.store_div(334, 394, 409);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
            s.store_square(405, 334);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
            s.store_mul(222, 405, 229);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
            s.copy_ad(404, 3118);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
            s.store_scalar(79, 0.0);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
            s.store_sqrt_ad(982, A::scale(A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(3111)), s.ad_value(155)), 2.0));
        }

        s.v[3193] = if (s.v[411] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (s.v[3193] != 0.0)) {
            s.store_sub_from_scalar(336, p.p334, 411);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3193] != 0.0))) {
            s.store_sqrt_ad(782, A::offset(A::mul(A::offset(s.ad_value(729), p.p137), A::offset(s.ad_value(729), p.p137)), ((4.0 * 0.1) * 0.1)));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3193] != 0.0))) {
            s.store_scale_ad(343, A::offset(A::div(A::offset(s.ad_value(729), p.p137), s.ad_value(782)), 1.0), 0.5);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3193] != 0.0))) {
            s.store_scale_ad(336, A::add(A::offset(s.ad_value(729), p.p137), s.ad_value(782)), 0.5);
        }

        s.v[3194] = if (s.v[336] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3193] != 0.0))) && (s.v[3194] != 0.0)) {
            s.store_scalar(336, 0.0);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3193] != 0.0))) && (s.v[3194] != 0.0)) {
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3193] != 0.0))) {
            s.store_scale_ad(600, A::sqrt(A::mul(s.ad_value(651), s.ad_value(336))), p.p432);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3193] != 0.0))) {
            s.store_sub_from_scalar(336, p.p334, 600);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
            s.store_sqrt_ad(782, A::offset(A::square(s.ad_value(336)), ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01))));
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
            s.store_scale_ad(343, A::offset(A::div(s.ad_value(336), s.ad_value(782)), 1.0), 0.5);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.v[3195] = if (s.v[336] < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (s.v[3195] != 0.0)) {
            s.store_scalar(336, 0.0);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (s.v[3195] != 0.0)) {
            s.store_scalar(343, 0.0);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
            s.copy_ad(386, 336);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
            s.store_scale_ad(418, A::scale(A::mul(A::mul(s.ad_value(3111), s.ad_value(386)), s.ad_value(386)), 0.5), 9662367879.197212);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
            s.store_sqrt_ad(334, A::mul(A::scale(s.ad_value(154), 2.0), s.ad_value(418)));
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
            s.store_scale_ad(335, A::add(A::limited_exp(s.ad_value(334)), A::limited_exp(A::neg(s.ad_value(334)))), 0.5);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
            s.store_div_ad_lhs(419, A::ln(s.ad_value(335)), 418);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_86(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        let mut assign94720_loop_guard: usize = 0;
        while {
            let assign94720_cond_e146419: f64 = (s.v[421] + 1.0);
            let assign94720_cond_e146421: f64 = if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (s.v[97] <= assign94720_cond_e146419)) { 1.0 } else { 0.0 };
            assign94720_cond_e146421 != 0.0
        } {
            assign94720_loop_guard += 1;
            assert!(assign94720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
                s.store_add(414, 404, 397);
            }
            if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
                s.store_mul(116, 154, 414);
            }
            if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
                s.store_mul_ad_rhs(333, 419, A::sub(s.ad_value(414), s.ad_value(418)));
            }
            s.v[3197] = if (s.v[333] < 60.0) { 1.0 } else { 0.0 };
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (s.v[3197] != 0.0)) {
                s.store_exp(335, 333);
            }
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (s.v[3197] != 0.0)) {
                s.store_exp_ad(334, A::mul(A::neg(s.ad_value(419)), s.ad_value(418)));
            }
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (s.v[3197] != 0.0)) {
                s.store_sub(336, 335, 334);
            }
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (s.v[3197] != 0.0)) {
                s.store_div_ad_lhs(416, A::ln(A::offset(s.ad_value(336), 1.0)), 419);
            }
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (s.v[3197] != 0.0)) {
                s.store_div_ad_rhs(417, 335, A::offset(s.ad_value(336), 1.0));
            }
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3197] != 0.0))) {
                s.store_sub(416, 414, 418);
            }
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3197] != 0.0))) {
                s.store_scalar(417, 1.0);
            }
            if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
                s.store_mul(415, 154, 416);
            }
            s.v[3198] = if (((s.v[116]) as f64).abs() < 1e-6) { 1.0 } else { 0.0 };
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (s.v[3198] != 0.0)) {
                s.store_mul_ad(334, A::scale(A::square(s.ad_value(116)), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(116), 0.3333333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(116), 0.25), A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.2)))))));
            }
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (s.v[3198] != 0.0)) {
                s.store_mul_ad_rhs(335, 116, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(116), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(116), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.25)))))));
            }
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (s.v[3198] != 0.0)) {
                s.store_mul_ad(336, A::scale(A::square(s.ad_value(415)), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(415), 0.3333333333333333), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(415), 0.25), A::sub_from_scalar(1.0, A::scale(s.ad_value(415), 0.2)))))));
            }
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (s.v[3198] != 0.0)) {
                s.store_mul_ad_rhs(337, 415, A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(415), 0.5), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(415), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(415), 0.25)))))));
            }
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (s.v[3198] != 0.0)) {
                s.store_sub(3119, 334, 336);
            }
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (s.v[3198] != 0.0)) {
                s.store_mul_ad_rhs(3120, 154, A::sub(s.ad_value(335), A::mul(s.ad_value(417), s.ad_value(337))));
            }
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3198] != 0.0))) {
                s.store_exp_ad(334, A::neg(s.ad_value(116)));
            }
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3198] != 0.0))) {
                s.store_exp_ad(335, A::neg(s.ad_value(415)));
            }
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3198] != 0.0))) {
                s.store_add_ad(3119, A::sub(s.ad_value(116), s.ad_value(415)), A::sub(s.ad_value(334), s.ad_value(335)));
            }
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3198] != 0.0))) {
                s.store_mul_ad_rhs(3120, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul(s.ad_value(417), A::sub_from_scalar(1.0, s.ad_value(335)))));
            }
            s.v[3199] = if (((s.v[116]) as f64).abs() < 5e-5) { 1.0 } else { 0.0 };
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (s.v[3199] != 0.0)) {
                s.store_mul_ad(334, A::scale(A::square(s.ad_value(116)), 0.5), A::offset(A::mul(A::scale(s.ad_value(116), 0.3333333333333333), A::offset(A::mul(A::scale(s.ad_value(116), 0.25), A::offset(A::scale(s.ad_value(116), 0.2), 1.0)), 1.0)), 1.0));
            }
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (s.v[3199] != 0.0)) {
                s.store_mul_ad_rhs(335, 116, A::offset(A::mul(A::scale(s.ad_value(116), 0.5), A::offset(A::mul(A::scale(s.ad_value(116), 0.3333333333333333), A::offset(A::scale(s.ad_value(116), 0.25), 1.0)), 1.0)), 1.0));
            }
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (s.v[3199] != 0.0)) {
                s.store_mul(214, 222, 334);
            }
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (s.v[3199] != 0.0)) {
                s.store_mul_ad_lhs(215, A::mul(s.ad_value(222), s.ad_value(335)), 154);
            }
            s.v[3200] = if (((s.v[116]) as f64).abs() < 60.0) { 1.0 } else { 0.0 };
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3199] != 0.0))) && (s.v[3200] != 0.0)) {
                s.store_exp(227, 116);
            }
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3199] != 0.0))) && (s.v[3200] != 0.0)) {
                s.store_offset(335, 227, (-1.0));
            }
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3199] != 0.0))) && (s.v[3200] != 0.0)) {
                s.store_mul_ad_rhs(214, 222, A::sub(s.ad_value(335), s.ad_value(116)));
            }
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3199] != 0.0))) && (s.v[3200] != 0.0)) {
                s.store_mul_ad_lhs(215, A::mul(s.ad_value(222), s.ad_value(154)), 335);
            }
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3199] != 0.0))) && (!(s.v[3200] != 0.0))) {
                s.store_exp_ad(231, A::mul(s.ad_value(154), s.ad_value(404)));
            }
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3199] != 0.0))) && (!(s.v[3200] != 0.0))) {
                s.store_mul_ad_rhs(214, 405, A::sub(s.ad_value(231), A::mul(s.ad_value(229), A::offset(s.ad_value(116), 1.0))));
            }
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3199] != 0.0))) && (!(s.v[3200] != 0.0))) {
                s.store_mul_ad(215, A::mul(s.ad_value(405), s.ad_value(154)), A::sub(s.ad_value(231), s.ad_value(229)));
            }
            s.v[3201] = if (s.v[214] > 0.0) { 1.0 } else { 0.0 };
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (s.v[3201] != 0.0)) {
                s.store_sqrt_ad(216, A::add(s.ad_value(3119), s.ad_value(214)));
            }
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (s.v[3201] != 0.0)) {
                s.store_div_ad_lhs(217, A::scale(A::add(s.ad_value(3120), s.ad_value(215)), 0.5), 216);
            }
            s.v[3202] = if (s.v[3119] > 0.0) { 1.0 } else { 0.0 };
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3201] != 0.0))) && (s.v[3202] != 0.0)) {
                s.store_sqrt(216, 3119);
            }
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3201] != 0.0))) && (s.v[3202] != 0.0)) {
                s.store_div_ad_lhs(217, A::scale(s.ad_value(3120), 0.5), 216);
            }
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3201] != 0.0))) && (!(s.v[3202] != 0.0))) {
                s.store_scalar(216, 0.0);
            }
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3201] != 0.0))) && (!(s.v[3202] != 0.0))) {
                s.store_scalar(217, 0.0);
            }
            if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
                s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
                s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
                s.store_add_ad(232, A::sub(s.ad_value(404), s.ad_value(402)), A::mul(s.ad_value(212), s.ad_value(216)));
            }
            if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
                s.store_offset_ad(233, A::mul(s.ad_value(212), s.ad_value(217)), 1.0);
            }
            s.v[3203] = if (s.v[79] > 0.0) { 1.0 } else { 0.0 };
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (s.v[3203] != 0.0)) {
                s.store_scalar(97, (s.v[421] + 1.0));
            }
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3203] != 0.0))) {
                s.store_div_ad_lhs(236, A::neg(s.ad_value(232)), 233);
            }
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3203] != 0.0))) {
                s.store_scale_ad(93, A::offset({
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0), (0.5 * 0.1));
            }
            s.v[3204] = if (((s.v[236]) as f64).abs() > s.v[93]) { 1.0 } else { 0.0 };
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3203] != 0.0))) && (s.v[3204] != 0.0)) {
                s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));
            }
            if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3203] != 0.0))) {
                s.store_add(404, 404, 236);
            }
            s.v[3205] = if ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8)) { 1.0 } else { 0.0 };
            if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) && (!(s.v[3203] != 0.0))) && (s.v[3205] != 0.0)) {
                s.store_offset(79, 79, 2.0);
            }
            if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
            s.store_ad(223, &{
                if (s.v[3119] >= 0.0) {
                    A::scale(A::sqrt(s.ad_value(3119)), (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }))
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
            s.store_mul(3109, 982, 223);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
            s.store_mul(3110, 3111, 3109);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
            s.store_offset_ad(100, A::div(s.ad_value(3110), s.ad_value(410)), (10.0 * 2.220446049250313e-16));
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
            s.store_mul(354, 410, 100);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
            s.store_div_from_scalar_ad(335, 1.0, A::add(s.ad_value(216), s.ad_value(100)));
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
            s.store_mul_ad_lhs(399, A::mul(s.ad_value(410), s.ad_value(214)), 335);
        }

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3192] != 0.0)) {
            s.store_add(398, 354, 399);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.store_sub(399, 398, 354);
        }

        s.v[3207] = if (s.v[407] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3207] != 0.0)) {
            s.store_neg(407, 407);
        }

        s.v[3208] = if (p.p55 == 0.0) { 1.0 } else { 0.0 };

        s.v[3209] = if (p.p50 == 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3207] != 0.0)) && (s.v[3208] != 0.0)) && (s.v[3209] != 0.0)) {
            s.store_neg(3112, 404);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3207] != 0.0)) && (s.v[3208] != 0.0)) && (!(s.v[3209] != 0.0))) {
            s.copy_ad(3112, 396);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3207] != 0.0)) && (s.v[3208] != 0.0)) {
            s.store_sqrt_ad(782, A::offset(A::mul(A::offset(s.ad_value(3112), p.p137), A::offset(s.ad_value(3112), p.p137)), ((4.0 * 0.1) * 0.1)));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3207] != 0.0)) && (s.v[3208] != 0.0)) {
            s.store_scale_ad(343, A::offset(A::div(A::offset(s.ad_value(3112), p.p137), s.ad_value(782)), 1.0), 0.5);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3207] != 0.0)) && (s.v[3208] != 0.0)) {
            s.store_scale_ad(336, A::add(A::offset(s.ad_value(3112), p.p137), s.ad_value(782)), 0.5);
        }

        s.v[3210] = if (s.v[336] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3207] != 0.0)) && (s.v[3208] != 0.0)) && (s.v[3210] != 0.0)) {
            s.store_scalar(336, 0.0);
        }

        if (((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3207] != 0.0)) && (s.v[3208] != 0.0)) && (s.v[3210] != 0.0)) {
            s.store_scalar(343, 0.0);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3207] != 0.0)) && (s.v[3208] != 0.0)) {
            s.store_scale_ad(600, A::sqrt(A::mul(s.ad_value(651), s.ad_value(336))), p.p432);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3207] != 0.0)) && (s.v[3208] != 0.0)) {
            s.store_sub_ad(781, A::sub(s.ad_value(407), s.ad_value(600)), A::scale(s.ad_value(407), 0.1));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3207] != 0.0)) && (s.v[3208] != 0.0)) {
            s.store_mul_ad(782, A::scale(s.ad_value(407), 4.0), A::scale(s.ad_value(407), 0.1));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3207] != 0.0)) && (s.v[3208] != 0.0)) {
            s.store_ad(782, &{
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3207] != 0.0)) && (s.v[3208] != 0.0)) {
            s.store_sqrt_ad(782, A::add(A::square(s.ad_value(781)), s.ad_value(782)));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3207] != 0.0)) && (s.v[3208] != 0.0)) {
            s.store_scale_ad(334, A::offset(A::div(s.ad_value(781), s.ad_value(782)), 1.0), 0.5);
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3207] != 0.0)) && (s.v[3208] != 0.0)) {
            s.store_sub_ad_rhs(603, 407, A::scale(A::add(s.ad_value(781), s.ad_value(782)), 0.5));
        }

        if ((((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) && (s.v[3207] != 0.0)) && (s.v[3208] != 0.0)) {
            s.store_sub(407, 407, 603);
        }

        if ((s.v[3107] != 0.0) && (s.v[3108] != 0.0)) {
            s.copy_ad(698, 354);
        }

        s.v[3211] = if (((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] == 0.0)) { 1.0 } else { 0.0 };

        if (s.v[3211] != 0.0) {
            s.store_scalar(2619, 1.0);
        }

        if (s.v[3211] != 0.0) {
            s.store_scalar(289, s.v[564]);
        }

        if (s.v[3211] != 0.0) {
            s.store_scalar(290, p.p276);
        }

        if (s.v[3211] != 0.0) {
            s.store_scalar(335, (s.v[188] * s.v[635]));
        }

        s.v[3212] = if (s.v[949] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[3211] != 0.0) && (s.v[3212] != 0.0)) {
            s.store_mul_ad(338, A::mul(s.ad_value(289), s.ad_value(335)), A::add(s.ad_value(290), s.ad_value(791)));
        }

        if ((s.v[3211] != 0.0) && (s.v[3212] != 0.0)) {
            s.store_scale(339, 335, p.p66);
        }

        if ((s.v[3211] != 0.0) && (s.v[3212] != 0.0)) {
            s.store_sub_from_scalar(343, 1.2, 87);
        }

        if ((s.v[3211] != 0.0) && (s.v[3212] != 0.0)) {
            s.store_sub_ad(291, A::mul(s.ad_value(791), s.ad_value(339)), A::mul(s.ad_value(338), s.ad_value(343)));
        }

        if ((s.v[3211] != 0.0) && (!(s.v[3212] != 0.0))) {
            s.store_mul_ad(338, A::mul(s.ad_value(289), s.ad_value(335)), A::sub(A::add(s.ad_value(290), s.ad_value(791)), s.ad_value(790)));
        }

        if ((s.v[3211] != 0.0) && (!(s.v[3212] != 0.0))) {
            s.store_scale(339, 335, p.p66);
        }

        if ((s.v[3211] != 0.0) && (!(s.v[3212] != 0.0))) {
            s.store_sub_ad_lhs(343, A::offset(s.ad_value(790), 1.2), 91);
        }

        if ((s.v[3211] != 0.0) && (!(s.v[3212] != 0.0))) {
            s.store_sub_ad(291, A::mul(A::sub(s.ad_value(791), s.ad_value(790)), s.ad_value(339)), A::mul(s.ad_value(338), s.ad_value(343)));
        }

        s.v[3213] = if (((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] == 0.0)) { 1.0 } else { 0.0 };

        if (s.v[3213] != 0.0) {
            s.store_scalar(2622, 1.0);
        }

        if (s.v[3213] != 0.0) {
            s.store_scalar(289, s.v[564]);
        }

        if (s.v[3213] != 0.0) {
            s.store_scalar(290, p.p276);
        }

        if (s.v[3213] != 0.0) {
            s.store_scale(335, 412, s.v[635]);
        }

        s.v[3214] = if (s.v[949] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[3213] != 0.0) && (s.v[3214] != 0.0)) {
            s.store_mul_ad(338, A::mul(s.ad_value(289), s.ad_value(335)), A::sub(A::add(s.ad_value(290), s.ad_value(791)), s.ad_value(790)));
        }

        if ((s.v[3213] != 0.0) && (s.v[3214] != 0.0)) {
            s.store_scale(339, 335, p.p63);
        }

        if ((s.v[3213] != 0.0) && (s.v[3214] != 0.0)) {
            s.store_sub_ad_lhs(343, A::offset(s.ad_value(790), 1.2), 91);
        }

        if ((s.v[3213] != 0.0) && (s.v[3214] != 0.0)) {
            s.store_sub_ad(292, A::mul(A::sub(s.ad_value(791), s.ad_value(790)), s.ad_value(339)), A::mul(s.ad_value(338), s.ad_value(343)));
        }

        if ((s.v[3213] != 0.0) && (!(s.v[3214] != 0.0))) {
            s.store_mul_ad(338, A::mul(s.ad_value(289), s.ad_value(335)), A::add(s.ad_value(290), s.ad_value(791)));
        }

        if ((s.v[3213] != 0.0) && (!(s.v[3214] != 0.0))) {
            s.store_scale(339, 335, p.p63);
        }

        if ((s.v[3213] != 0.0) && (!(s.v[3214] != 0.0))) {
            s.store_sub_from_scalar(343, 1.2, 87);
        }

        if ((s.v[3213] != 0.0) && (!(s.v[3214] != 0.0))) {
            s.store_sub_ad(292, A::mul(s.ad_value(791), s.ad_value(339)), A::mul(s.ad_value(338), s.ad_value(343)));
        }

        if (s.v[768] != 0.0) {
            s.store_scalar(295, (s.v[505] * (-s.v[635])));
        }

        s.v[3215] = if (s.v[2619] == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[768] != 0.0)) && (s.v[3215] != 0.0)) {
            s.store_scalar(295, (((-s.v[188]) * p.p66) * s.v[635]));
        }

        s.store_mul_ad_lhs(297, A::neg(s.ad_value(295)), 734);

        if (s.v[769] != 0.0) {
            s.store_scalar(294, (s.v[506] * (-s.v[635])));
        }

        s.v[3216] = if (s.v[2622] == 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[769] != 0.0)) && (s.v[3216] != 0.0)) {
            s.store_scale_ad(294, A::neg(s.ad_value(412)), (p.p63 * s.v[635]));
        }

        s.store_mul_ad(298, A::neg(s.ad_value(294)), A::sub(s.ad_value(734), s.ad_value(733)));

        s.v[3217] = if (s.v[949] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[3217] != 0.0) {
            s.store_scaled_sub(357, 790, 94, p.p431);
        }

        if (s.v[3217] != 0.0) {
            s.store_mul(360, 338, 357);
        }

        if (s.v[3217] != 0.0) {
            s.store_mul(361, 338, 357);
        }

        if (!(s.v[3217] != 0.0)) {
            s.store_scaled_sub(357, 790, 94, (-p.p431));
        }

        if (!(s.v[3217] != 0.0)) {
            s.store_mul(362, 338, 357);
        }

        if (!(s.v[3217] != 0.0)) {
            s.store_mul(363, 338, 357);
        }

        s.v[296] = ((-s.v[525]) * s.v[582]);

        s.store_scaled_sub(293, 731, 728, (-s.v[296]));

        s.v[172] = s.v[507];

        s.v[3218] = if (s.v[78] != 0.0) { 1.0 } else { 0.0 };

        if (s.v[3218] != 0.0) {
            s.store_add_ad(168, A::scale(A::add(s.ad_value(790), s.ad_value(87)), s.v[172]), A::scale(s.ad_value(91), (1.0 - s.v[172])));
        }

        s.v[3219] = if ((s.v[168] > (((s.v[87] + s.v[790]) - (10.0 * 2.220446049250313e-16)) - (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) {
            s.store_offset_ad(781, A::sub(s.ad_value(168), A::offset(A::add(s.ad_value(87), s.ad_value(790)), (-(10.0 * 2.220446049250313e-16)))), (10.0 * 2.220446049250313e-16));
        }

        if ((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) {
            s.store_square(722, 781);
        }

        if ((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) {
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
        }

        if ((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) {
            s.store_scalar(724, 1.0);
        }

        if ((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) {
            s.store_scalar(725, 1.0);
        }

        if ((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) {
            s.store_scalar(719, 0.0);
        }

        if ((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) {
            s.store_scalar(720, 0.0);
        }

        if ((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) {
            s.store_scalar(770, 0.0);
        }

        if ((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) {
            s.store_scalar(726, 0.0);
        }

        if ((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) {
            s.store_mul(724, 724, 722);
        }

        if ((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) {
            s.store_mul(725, 725, 723);
        }

        if ((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) {
            s.store_mul(724, 724, 722);
        }

    }

    pub(super) fn stamp_transient_block_87(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) {
            s.store_mul(725, 725, 723);
        }

        if ((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) {
            s.store_add(770, 724, 725);
        }

        if ((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) {
            s.copy_ad(726, 770);
        }

        s.v[3220] = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[3221] = if (2.0 == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) && (s.v[3220] != 0.0)) && (s.v[3221] != 0.0)) {
            s.store_scalar(720, 1.0);
        }

        s.v[3222] = if (2.0 == 2.0) { 1.0 } else { 0.0 };

        if (((((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) && (s.v[3220] != 0.0)) && (!(s.v[3221] != 0.0))) && (s.v[3222] != 0.0)) {
            s.store_scalar(720, 2.0);
        }

        s.v[3223] = if (2.0 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) && (s.v[3220] != 0.0)) && (!(s.v[3221] != 0.0))) && (!(s.v[3222] != 0.0))) && (s.v[3223] != 0.0)) {
            s.store_scalar(720, 3.0);
        }

        s.v[3224] = if (2.0 == 8.0) { 1.0 } else { 0.0 };

        if (((((((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) && (s.v[3220] != 0.0)) && (!(s.v[3221] != 0.0))) && (!(s.v[3222] != 0.0))) && (!(s.v[3223] != 0.0))) && (s.v[3224] != 0.0)) {
            s.store_scalar(720, 4.0);
        }

        if (((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) && (s.v[3220] != 0.0)) {
            s.store_scalar(719, 0.0);
        }

        let mut assign95780_loop_guard: usize = 0;
        while {
            let assign95780_cond_e148350: f64 = if ((((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) && (s.v[3220] != 0.0)) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign95780_cond_e148350 != 0.0
        } {
            assign95780_loop_guard += 1;
            assert!(assign95780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) && (s.v[3220] != 0.0)) {
                s.store_sqrt(726, 726);
            }
            if (((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) && (s.v[3220] != 0.0)) {
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) && (!(s.v[3220] != 0.0))) {
            s.store_ad(726, &{
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) {
            s.store_div_from_scalar(726, 1.0, 726);
        }

        if ((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) {
            s.store_mul_ad_lhs(780, A::scale(s.ad_value(781), (10.0 * 2.220446049250313e-16)), 726);
        }

        if ((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) {
            s.store_div_ad_lhs(334, A::mul(A::scale(s.ad_value(725), (10.0 * 2.220446049250313e-16)), s.ad_value(726)), 770);
        }

        if ((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) {
            s.store_add_ad_lhs(168, A::offset(A::offset(A::add(s.ad_value(87), s.ad_value(790)), (-(10.0 * 2.220446049250313e-16))), (-(10.0 * 2.220446049250313e-16))), 780);
        }

        if ((s.v[3218] != 0.0) && (s.v[3219] != 0.0)) {
        }

        if ((s.v[3218] != 0.0) && (!(s.v[3219] != 0.0))) {
        }

        if ((s.v[3218] != 0.0) && (!(s.v[3219] != 0.0))) {
            s.store_scalar(334, 1.0);
        }

        if ((s.v[3218] != 0.0) && (s.v[82] != 0.0)) {
            s.store_scalar(303, 0.0);
        }

        s.v[3225] = if ((s.v[248] < 1e-15) || (s.v[348] < 1e-6)) { 1.0 } else { 0.0 };

        if (((!(s.v[3218] != 0.0)) && (s.v[82] != 0.0)) && (s.v[3225] != 0.0)) {
            s.store_scalar(303, 0.0);
        }

        if (((!(s.v[3218] != 0.0)) && (s.v[82] != 0.0)) && (!(s.v[3225] != 0.0))) {
            s.store_div_ad_lhs(303, A::mul(A::div(s.ad_value(248), s.ad_value(238)), s.ad_value(155)), 162);
        }

        s.v[3226] = if (s.v[82] == 0.0) { 1.0 } else { 0.0 };

        if (s.v[3226] != 0.0) {
            s.store_scalar(305, 0.0);
        }

        if (!(s.v[3226] != 0.0)) {
            s.store_scale(336, 684, ((1.034943e-10 * s.v[635]) * 1.3));
        }

        s.v[3227] = if (p.p133 != 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[3226] != 0.0)) && (s.v[3227] != 0.0)) {
            s.store_add_ad_lhs(304, A::mul(s.ad_value(303), s.ad_value(162)), 87);
        }

        if ((!(s.v[3226] != 0.0)) && (s.v[3227] != 0.0)) {
            s.store_add_ad(335, A::scale(A::add(s.ad_value(1435), s.ad_value(87)), s.v[172]), A::scale(s.ad_value(304), (1.0 - s.v[172])));
        }

        if ((!(s.v[3226] != 0.0)) && (s.v[3227] != 0.0)) {
            s.store_mul_ad_lhs(305, A::neg(A::scale(A::sub(A::add(s.ad_value(87), s.ad_value(1435)), s.ad_value(335)), 1.0 / (p.p133))), 336);
        }

        s.v[3228] = if (p.p134 != 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[3226] != 0.0)) && (s.v[3228] != 0.0)) {
            s.store_add_ad_rhs(305, 305, A::scale(s.ad_value(792), s.v[671]));
        }

        s.v[300] = s.v[670];

        s.v[302] = s.v[670];

        s.store_scaled_sub(299, 734, 733, s.v[300]);

        s.store_scale(301, 734, s.v[302]);

        s.v[3229] = if ((p.p53 > 0.0) && (s.v[541] != 0.0)) { 1.0 } else { 0.0 };

        if (s.v[3229] != 0.0) {
            s.store_square(334, 676);
        }

        if (s.v[3229] != 0.0) {
            s.store_scale_ad(828, A::exp(A::scale(A::add(A::sub(A::scale(s.ad_value(678), s.v[616]), A::mul(s.ad_value(393), s.ad_value(154))), A::scale(s.ad_value(590), p.p499)), 1.0 / (s.v[820]))), s.v[818]);
        }

        if (s.v[3229] != 0.0) {
            s.store_scale_ad(829, A::exp(A::scale(A::add(A::sub(A::scale(s.ad_value(678), s.v[616]), A::mul(s.ad_value(393), s.ad_value(154))), A::scale(s.ad_value(590), p.p499)), 1.0 / (p.p497))), s.v[819]);
        }

        if (s.v[3229] != 0.0) {
            s.store_scale_ad(836, A::exp(A::scale(A::add(A::sub(A::scale(s.ad_value(678), s.v[616]), A::mul(s.ad_value(393), s.ad_value(154))), A::scale(s.ad_value(590), p.p499)), 1.0 / (p.p498))), p.p495);
        }

        if (s.v[3229] != 0.0) {
            s.store_scale_ad(830, A::exp(A::scale(A::add(A::sub(A::scale(s.ad_value(678), s.v[616]), A::mul(s.ad_value(393), s.ad_value(154))), A::scale(s.ad_value(590), p.p509)), 1.0 / (s.v[820]))), s.v[818]);
        }

        if (s.v[3229] != 0.0) {
            s.store_scale_ad(831, A::exp(A::scale(A::add(A::sub(A::scale(s.ad_value(678), s.v[616]), A::mul(s.ad_value(393), s.ad_value(154))), A::scale(s.ad_value(590), p.p509)), 1.0 / (p.p497))), s.v[819]);
        }

        if (s.v[3229] != 0.0) {
            s.store_scale_ad(837, A::exp(A::scale(A::add(A::sub(A::scale(s.ad_value(678), s.v[616]), A::mul(s.ad_value(393), s.ad_value(154))), A::scale(s.ad_value(590), p.p509)), 1.0 / (p.p498))), p.p495);
        }

        s.v[3230] = if (p.p48 > 0.0) { 1.0 } else { 0.0 };

        s.v[3231] = if (p.p15 > s.v[632]) { 1.0 } else { 0.0 };

        if (((s.v[3229] != 0.0) && (s.v[3230] != 0.0)) && (s.v[3231] != 0.0)) {
            s.store_scale(873, 828, p.p13);
        }

        if (((s.v[3229] != 0.0) && (s.v[3230] != 0.0)) && (s.v[3231] != 0.0)) {
            s.store_scale(874, 830, p.p13);
        }

        if (((s.v[3229] != 0.0) && (s.v[3230] != 0.0)) && (s.v[3231] != 0.0)) {
            s.store_scale(875, 829, (p.p15 - s.v[632]));
        }

        if (((s.v[3229] != 0.0) && (s.v[3230] != 0.0)) && (s.v[3231] != 0.0)) {
            s.store_scale(876, 831, (p.p15 - s.v[632]));
        }

        if (((s.v[3229] != 0.0) && (s.v[3230] != 0.0)) && (s.v[3231] != 0.0)) {
            s.store_scale(877, 836, s.v[632]);
        }

        if (((s.v[3229] != 0.0) && (s.v[3230] != 0.0)) && (s.v[3231] != 0.0)) {
            s.store_scale(878, 837, s.v[632]);
        }

        if (((s.v[3229] != 0.0) && (s.v[3230] != 0.0)) && (!(s.v[3231] != 0.0))) {
            s.store_scale(873, 828, p.p13);
        }

        if (((s.v[3229] != 0.0) && (s.v[3230] != 0.0)) && (!(s.v[3231] != 0.0))) {
            s.store_scale(874, 830, p.p13);
        }

        if (((s.v[3229] != 0.0) && (s.v[3230] != 0.0)) && (!(s.v[3231] != 0.0))) {
            s.store_scalar(875, 0.0);
        }

        if (((s.v[3229] != 0.0) && (s.v[3230] != 0.0)) && (!(s.v[3231] != 0.0))) {
            s.store_scalar(876, 0.0);
        }

        if (((s.v[3229] != 0.0) && (s.v[3230] != 0.0)) && (!(s.v[3231] != 0.0))) {
            s.store_scale(877, 836, p.p15);
        }

        if (((s.v[3229] != 0.0) && (s.v[3230] != 0.0)) && (!(s.v[3231] != 0.0))) {
            s.store_scale(878, 837, p.p15);
        }

        if ((s.v[3229] != 0.0) && (!(s.v[3230] != 0.0))) {
            s.store_scale(873, 828, p.p13);
        }

        if ((s.v[3229] != 0.0) && (!(s.v[3230] != 0.0))) {
            s.store_scale(874, 830, p.p13);
        }

        if ((s.v[3229] != 0.0) && (!(s.v[3230] != 0.0))) {
            s.store_scale(875, 829, p.p15);
        }

        if ((s.v[3229] != 0.0) && (!(s.v[3230] != 0.0))) {
            s.store_scale(876, 831, p.p15);
        }

        if ((s.v[3229] != 0.0) && (!(s.v[3230] != 0.0))) {
            s.store_scalar(877, 0.0);
        }

        if ((s.v[3229] != 0.0) && (!(s.v[3230] != 0.0))) {
            s.store_scalar(878, 0.0);
        }

        if (s.v[3229] != 0.0) {
            s.store_add_ad_lhs(847, A::add(s.ad_value(873), s.ad_value(875)), 877);
        }

        s.v[3232] = if (s.v[847] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[3229] != 0.0) && (s.v[3232] != 0.0)) {
            s.store_offset(336, 847, 1e-25);
        }

        if ((s.v[3229] != 0.0) && (s.v[3232] != 0.0)) {
            s.store_mul_ad(848, A::div_from_scalar(s.v[820], s.ad_value(154)), A::ln(A::offset(A::div(A::scale(s.ad_value(334), s.v[822]), s.ad_value(336)), 1.0)));
        }

        if ((s.v[3229] != 0.0) && (s.v[3232] != 0.0)) {
            s.store_exp_ad(849, A::scale(A::offset(s.ad_value(676), (-1.0)), p.p512));
        }

        if ((s.v[3229] != 0.0) && (s.v[3232] != 0.0)) {
            s.store_div_from_scalar_ad(850, 1.0, A::div_from_scalar(s.v[820], s.ad_value(154)));
        }

        if ((s.v[3229] != 0.0) && (s.v[3232] != 0.0)) {
            s.store_exp_ad(851, A::mul(s.ad_value(848), s.ad_value(850)));
        }

        if (s.v[3229] != 0.0) {
            s.store_scale_ad(828, A::exp(A::scale(A::add(A::sub(A::scale(s.ad_value(678), s.v[616]), A::mul(s.ad_value(393), s.ad_value(154))), A::scale(s.ad_value(590), p.p522)), 1.0 / (s.v[825]))), s.v[823]);
        }

        if (s.v[3229] != 0.0) {
            s.store_scale_ad(829, A::exp(A::scale(A::add(A::sub(A::scale(s.ad_value(678), s.v[616]), A::mul(s.ad_value(393), s.ad_value(154))), A::scale(s.ad_value(590), p.p522)), 1.0 / (p.p520))), s.v[824]);
        }

        if (s.v[3229] != 0.0) {
            s.store_scale_ad(836, A::exp(A::scale(A::add(A::sub(A::scale(s.ad_value(678), s.v[616]), A::mul(s.ad_value(393), s.ad_value(154))), A::scale(s.ad_value(590), p.p522)), 1.0 / (p.p521))), p.p518);
        }

        if (s.v[3229] != 0.0) {
            s.store_scale_ad(830, A::exp(A::scale(A::add(A::sub(A::scale(s.ad_value(678), s.v[616]), A::mul(s.ad_value(393), s.ad_value(154))), A::scale(s.ad_value(590), p.p532)), 1.0 / (s.v[825]))), s.v[823]);
        }

        if (s.v[3229] != 0.0) {
            s.store_scale_ad(831, A::exp(A::scale(A::add(A::sub(A::scale(s.ad_value(678), s.v[616]), A::mul(s.ad_value(393), s.ad_value(154))), A::scale(s.ad_value(590), p.p532)), 1.0 / (p.p520))), s.v[824]);
        }

        if (s.v[3229] != 0.0) {
            s.store_scale_ad(837, A::exp(A::scale(A::add(A::sub(A::scale(s.ad_value(678), s.v[616]), A::mul(s.ad_value(393), s.ad_value(154))), A::scale(s.ad_value(590), p.p532)), 1.0 / (p.p521))), p.p518);
        }

        s.v[3233] = if (p.p48 > 0.0) { 1.0 } else { 0.0 };

        s.v[3234] = if (p.p16 > s.v[632]) { 1.0 } else { 0.0 };

        if (((s.v[3229] != 0.0) && (s.v[3233] != 0.0)) && (s.v[3234] != 0.0)) {
            s.store_scale(879, 828, p.p14);
        }

        if (((s.v[3229] != 0.0) && (s.v[3233] != 0.0)) && (s.v[3234] != 0.0)) {
            s.store_scale(880, 830, p.p14);
        }

        if (((s.v[3229] != 0.0) && (s.v[3233] != 0.0)) && (s.v[3234] != 0.0)) {
            s.store_scale(881, 829, (p.p16 - s.v[632]));
        }

        if (((s.v[3229] != 0.0) && (s.v[3233] != 0.0)) && (s.v[3234] != 0.0)) {
            s.store_scale(882, 831, (p.p16 - s.v[632]));
        }

        if (((s.v[3229] != 0.0) && (s.v[3233] != 0.0)) && (s.v[3234] != 0.0)) {
            s.store_scale(883, 836, s.v[632]);
        }

        if (((s.v[3229] != 0.0) && (s.v[3233] != 0.0)) && (s.v[3234] != 0.0)) {
            s.store_scale(884, 837, s.v[632]);
        }

        if (((s.v[3229] != 0.0) && (s.v[3233] != 0.0)) && (!(s.v[3234] != 0.0))) {
            s.store_scale(879, 828, p.p14);
        }

        if (((s.v[3229] != 0.0) && (s.v[3233] != 0.0)) && (!(s.v[3234] != 0.0))) {
            s.store_scale(880, 830, p.p14);
        }

        if (((s.v[3229] != 0.0) && (s.v[3233] != 0.0)) && (!(s.v[3234] != 0.0))) {
            s.store_scalar(881, 0.0);
        }

        if (((s.v[3229] != 0.0) && (s.v[3233] != 0.0)) && (!(s.v[3234] != 0.0))) {
            s.store_scalar(882, 0.0);
        }

        if (((s.v[3229] != 0.0) && (s.v[3233] != 0.0)) && (!(s.v[3234] != 0.0))) {
            s.store_scale(883, 836, p.p16);
        }

        if (((s.v[3229] != 0.0) && (s.v[3233] != 0.0)) && (!(s.v[3234] != 0.0))) {
            s.store_scale(884, 837, p.p16);
        }

        if ((s.v[3229] != 0.0) && (!(s.v[3233] != 0.0))) {
            s.store_scale(879, 828, p.p14);
        }

        if ((s.v[3229] != 0.0) && (!(s.v[3233] != 0.0))) {
            s.store_scale(880, 830, p.p14);
        }

        if ((s.v[3229] != 0.0) && (!(s.v[3233] != 0.0))) {
            s.store_scale(881, 829, p.p16);
        }

        if ((s.v[3229] != 0.0) && (!(s.v[3233] != 0.0))) {
            s.store_scale(882, 831, p.p16);
        }

        if ((s.v[3229] != 0.0) && (!(s.v[3233] != 0.0))) {
            s.store_scalar(883, 0.0);
        }

        if ((s.v[3229] != 0.0) && (!(s.v[3233] != 0.0))) {
            s.store_scalar(884, 0.0);
        }

        if (s.v[3229] != 0.0) {
            s.store_add_ad_lhs(852, A::add(s.ad_value(879), s.ad_value(881)), 883);
        }

        s.v[3235] = if (s.v[852] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[3229] != 0.0) && (s.v[3235] != 0.0)) {
            s.store_offset(337, 852, 1e-25);
        }

        if ((s.v[3229] != 0.0) && (s.v[3235] != 0.0)) {
            s.store_mul_ad(853, A::div_from_scalar(s.v[825], s.ad_value(154)), A::ln(A::offset(A::div(A::scale(s.ad_value(334), s.v[827]), s.ad_value(337)), 1.0)));
        }

        if ((s.v[3229] != 0.0) && (s.v[3235] != 0.0)) {
            s.store_exp_ad(854, A::scale(A::offset(s.ad_value(676), (-1.0)), p.p535));
        }

        if ((s.v[3229] != 0.0) && (s.v[3235] != 0.0)) {
            s.store_div_from_scalar_ad(855, 1.0, A::div_from_scalar(s.v[825], s.ad_value(154)));
        }

        if ((s.v[3229] != 0.0) && (s.v[3235] != 0.0)) {
            s.store_exp_ad(856, A::mul(s.ad_value(853), s.ad_value(855)));
        }

        if (s.v[3229] != 0.0) {
            s.store_scale_ad(832, A::offset(A::scale(s.ad_value(391), p.p481), 1.0), (p.p500 * p.p13));
        }

        s.v[3236] = if (p.p15 > s.v[632]) { 1.0 } else { 0.0 };

        if ((s.v[3229] != 0.0) && (s.v[3236] != 0.0)) {
            s.store_scale_ad(833, A::offset(A::scale(s.ad_value(391), p.p483), 1.0), (p.p501 * (p.p15 - s.v[632])));
        }

        if ((s.v[3229] != 0.0) && (s.v[3236] != 0.0)) {
            s.store_scale_ad(834, A::offset(A::scale(s.ad_value(391), p.p485), 1.0), (p.p502 * s.v[632]));
        }

        if ((s.v[3229] != 0.0) && (!(s.v[3236] != 0.0))) {
            s.store_scalar(833, 0.0);
        }

        if ((s.v[3229] != 0.0) && (!(s.v[3236] != 0.0))) {
            s.store_scale_ad(834, A::offset(A::scale(s.ad_value(391), p.p485), 1.0), (p.p502 * p.p15));
        }

        s.v[3237] = if (s.v[832] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[3229] != 0.0) && (s.v[3237] != 0.0)) {
            s.store_scalar(832, 0.0);
        }

        s.v[3238] = if (s.v[833] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[3229] != 0.0) && (s.v[3238] != 0.0)) {
            s.store_scalar(833, 0.0);
        }

        s.v[3239] = if (s.v[834] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[3229] != 0.0) && (s.v[3239] != 0.0)) {
            s.store_scalar(834, 0.0);
        }

        if (s.v[3229] != 0.0) {
            s.store_sub_from_scalar_ad(841, p.p506, A::scale(s.ad_value(391), p.p487));
        }

        if (s.v[3229] != 0.0) {
            s.store_sub_from_scalar_ad(842, p.p507, A::scale(s.ad_value(391), p.p489));
        }

        if (s.v[3229] != 0.0) {
            s.store_sub_from_scalar_ad(843, p.p508, A::scale(s.ad_value(391), p.p491));
        }

        s.v[3240] = if ((s.v[841] < 0.01) && (p.p13 > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[3229] != 0.0) && (s.v[3240] != 0.0)) {
            s.store_scalar(841, 0.01);
        }

        s.v[3241] = if ((s.v[842] < 0.01) && (p.p15 > s.v[632])) { 1.0 } else { 0.0 };

        if ((s.v[3229] != 0.0) && (s.v[3241] != 0.0)) {
            s.store_scalar(842, 0.01);
        }

        s.v[3242] = if ((s.v[843] < 0.01) && (p.p15 > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[3229] != 0.0) && (s.v[3242] != 0.0)) {
            s.store_scalar(843, 0.01);
        }

        if (s.v[3229] != 0.0) {
            s.store_scale_ad(835, A::offset(A::scale(s.ad_value(391), p.p482), 1.0), (p.p523 * p.p14));
        }

        s.v[3243] = if (p.p16 > s.v[632]) { 1.0 } else { 0.0 };

        if ((s.v[3229] != 0.0) && (s.v[3243] != 0.0)) {
            s.store_scale_ad(838, A::offset(A::scale(s.ad_value(391), p.p484), 1.0), (p.p524 * (p.p16 - s.v[632])));
        }

        if ((s.v[3229] != 0.0) && (s.v[3243] != 0.0)) {
            s.store_scale_ad(839, A::offset(A::scale(s.ad_value(391), p.p486), 1.0), (p.p525 * s.v[632]));
        }

        if ((s.v[3229] != 0.0) && (!(s.v[3243] != 0.0))) {
            s.store_scalar(838, 0.0);
        }

        if ((s.v[3229] != 0.0) && (!(s.v[3243] != 0.0))) {
            s.store_scale_ad(839, A::offset(A::scale(s.ad_value(391), p.p486), 1.0), (p.p525 * p.p16));
        }

        s.v[3244] = if (s.v[835] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[3229] != 0.0) && (s.v[3244] != 0.0)) {
            s.store_scalar(835, 0.0);
        }

        s.v[3245] = if (s.v[838] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[3229] != 0.0) && (s.v[3245] != 0.0)) {
            s.store_scalar(838, 0.0);
        }

        s.v[3246] = if (s.v[839] < 0.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_88(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[3229] != 0.0) && (s.v[3246] != 0.0)) {
            s.store_scalar(839, 0.0);
        }

        if (s.v[3229] != 0.0) {
            s.store_sub_from_scalar_ad(844, p.p529, A::scale(s.ad_value(391), p.p488));
        }

        if (s.v[3229] != 0.0) {
            s.store_sub_from_scalar_ad(845, p.p530, A::scale(s.ad_value(391), p.p490));
        }

        if (s.v[3229] != 0.0) {
            s.store_sub_from_scalar_ad(846, p.p531, A::scale(s.ad_value(391), p.p492));
        }

        s.v[3247] = if ((s.v[844] < 0.01) && (p.p14 > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[3229] != 0.0) && (s.v[3247] != 0.0)) {
            s.store_scalar(844, 0.01);
        }

        s.v[3248] = if ((s.v[845] < 0.01) && (p.p16 > s.v[632])) { 1.0 } else { 0.0 };

        if ((s.v[3229] != 0.0) && (s.v[3248] != 0.0)) {
            s.store_scalar(845, 0.01);
        }

        s.v[3249] = if ((s.v[846] < 0.01) && (p.p16 > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[3229] != 0.0) && (s.v[3249] != 0.0)) {
            s.store_scalar(846, 0.01);
        }

        if (!(s.v[3229] != 0.0)) {
            s.store_scalar(387, (ctx.temperature() + p.p11));
        }

        s.store_scale(344, 850, p.p511);

        s.store_scale(343, 849, p.p510);

        s.v[3250] = if (s.v[873] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[3250] != 0.0) {
            s.store_mul(334, 874, 343);
        }

        if (s.v[3250] != 0.0) {
            s.store_mul_ad_lhs(332, A::neg(s.ad_value(860)), 344);
        }

        if (s.v[3250] != 0.0) {
            s.store_exp(336, 332);
        }

        if (s.v[3250] != 0.0) {
            s.copy_ad(337, 336);
        }

        s.v[3251] = if (s.v[860] < s.v[848]) { 1.0 } else { 0.0 };

        if ((s.v[3250] != 0.0) && (s.v[3251] != 0.0)) {
            s.store_mul(332, 860, 850);
        }

        s.v[3252] = if (s.v[332] < ((-3.0) * 34.0)) { 1.0 } else { 0.0 };

        if (((s.v[3250] != 0.0) && (s.v[3251] != 0.0)) && (s.v[3252] != 0.0)) {
            s.store_scalar(335, 0.0);
        }

        if (((s.v[3250] != 0.0) && (s.v[3251] != 0.0)) && (!(s.v[3252] != 0.0))) {
            s.store_exp(335, 332);
        }

        if ((s.v[3250] != 0.0) && (s.v[3251] != 0.0)) {
            s.store_add_ad(885, A::add(A::mul(s.ad_value(873), A::offset(s.ad_value(335), (-1.0))), A::mul(s.ad_value(334), A::offset(s.ad_value(336), (-1.0)))), A::scale(A::offset(s.ad_value(337), (-1.0)), s.v[821]));
        }

        if ((s.v[3250] != 0.0) && (!(s.v[3251] != 0.0))) {
            s.copy_ad(335, 851);
        }

        if ((s.v[3250] != 0.0) && (!(s.v[3251] != 0.0))) {
            s.store_mul_ad_lhs(338, A::mul(s.ad_value(873), s.ad_value(850)), 335);
        }

        if ((s.v[3250] != 0.0) && (!(s.v[3251] != 0.0))) {
            s.store_add_ad(885, A::add(A::add(A::mul(s.ad_value(873), A::offset(s.ad_value(335), (-1.0))), A::mul(s.ad_value(338), A::sub(s.ad_value(860), s.ad_value(848)))), A::mul(s.ad_value(334), A::offset(s.ad_value(336), (-1.0)))), A::scale(A::offset(s.ad_value(337), (-1.0)), s.v[821]));
        }

        if (!(s.v[3250] != 0.0)) {
            s.store_scalar(885, 0.0);
        }

        s.store_scale(346, 874, p.p514);

        s.store_add_ad_rhs(885, 885, A::mul(s.ad_value(346), s.ad_value(860)));

        s.v[3253] = if (s.v[875] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[3253] != 0.0) {
            s.store_mul(334, 876, 343);
        }

        if (s.v[3253] != 0.0) {
            s.store_mul_ad_lhs(332, A::neg(s.ad_value(860)), 344);
        }

        if (s.v[3253] != 0.0) {
            s.store_exp(336, 332);
        }

        if (s.v[3253] != 0.0) {
            s.copy_ad(337, 336);
        }

        s.v[3254] = if (s.v[860] < s.v[848]) { 1.0 } else { 0.0 };

        if ((s.v[3253] != 0.0) && (s.v[3254] != 0.0)) {
            s.store_mul(332, 860, 850);
        }

        s.v[3255] = if (s.v[332] < ((-3.0) * 34.0)) { 1.0 } else { 0.0 };

        if (((s.v[3253] != 0.0) && (s.v[3254] != 0.0)) && (s.v[3255] != 0.0)) {
            s.store_scalar(335, 0.0);
        }

        if (((s.v[3253] != 0.0) && (s.v[3254] != 0.0)) && (!(s.v[3255] != 0.0))) {
            s.store_exp(335, 332);
        }

        if ((s.v[3253] != 0.0) && (s.v[3254] != 0.0)) {
            s.store_add_ad(887, A::add(A::mul(s.ad_value(875), A::offset(s.ad_value(335), (-1.0))), A::mul(s.ad_value(334), A::offset(s.ad_value(336), (-1.0)))), A::scale(A::offset(s.ad_value(337), (-1.0)), s.v[821]));
        }

        if ((s.v[3253] != 0.0) && (!(s.v[3254] != 0.0))) {
            s.copy_ad(335, 851);
        }

        if ((s.v[3253] != 0.0) && (!(s.v[3254] != 0.0))) {
            s.store_mul_ad_lhs(338, A::mul(s.ad_value(875), s.ad_value(850)), 335);
        }

        if ((s.v[3253] != 0.0) && (!(s.v[3254] != 0.0))) {
            s.store_add_ad(887, A::add(A::add(A::mul(s.ad_value(875), A::offset(s.ad_value(335), (-1.0))), A::mul(s.ad_value(338), A::sub(s.ad_value(860), s.ad_value(848)))), A::mul(s.ad_value(334), A::offset(s.ad_value(336), (-1.0)))), A::scale(A::offset(s.ad_value(337), (-1.0)), s.v[821]));
        }

        if (!(s.v[3253] != 0.0)) {
            s.store_scalar(887, 0.0);
        }

        s.store_scale(346, 876, p.p514);

        s.store_add_ad_rhs(887, 887, A::mul(s.ad_value(346), s.ad_value(860)));

        s.v[3256] = if (p.p48 > 0.0) { 1.0 } else { 0.0 };

        s.v[3257] = if (s.v[877] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[3256] != 0.0) && (s.v[3257] != 0.0)) {
            s.store_mul(334, 878, 343);
        }

        if ((s.v[3256] != 0.0) && (s.v[3257] != 0.0)) {
            s.store_mul_ad_lhs(332, A::neg(s.ad_value(868)), 344);
        }

        if ((s.v[3256] != 0.0) && (s.v[3257] != 0.0)) {
            s.store_exp(336, 332);
        }

        if ((s.v[3256] != 0.0) && (s.v[3257] != 0.0)) {
            s.copy_ad(337, 336);
        }

        s.v[3258] = if (s.v[868] < s.v[848]) { 1.0 } else { 0.0 };

        if (((s.v[3256] != 0.0) && (s.v[3257] != 0.0)) && (s.v[3258] != 0.0)) {
            s.store_mul(332, 868, 850);
        }

        s.v[3259] = if (s.v[332] < ((-3.0) * 34.0)) { 1.0 } else { 0.0 };

        if ((((s.v[3256] != 0.0) && (s.v[3257] != 0.0)) && (s.v[3258] != 0.0)) && (s.v[3259] != 0.0)) {
            s.store_scalar(335, 0.0);
        }

        if ((((s.v[3256] != 0.0) && (s.v[3257] != 0.0)) && (s.v[3258] != 0.0)) && (!(s.v[3259] != 0.0))) {
            s.store_exp(335, 332);
        }

        if (((s.v[3256] != 0.0) && (s.v[3257] != 0.0)) && (s.v[3258] != 0.0)) {
            s.store_add_ad(889, A::add(A::mul(s.ad_value(877), A::offset(s.ad_value(335), (-1.0))), A::mul(s.ad_value(334), A::offset(s.ad_value(336), (-1.0)))), A::scale(A::offset(s.ad_value(337), (-1.0)), s.v[821]));
        }

        if (((s.v[3256] != 0.0) && (s.v[3257] != 0.0)) && (!(s.v[3258] != 0.0))) {
            s.copy_ad(335, 851);
        }

        if (((s.v[3256] != 0.0) && (s.v[3257] != 0.0)) && (!(s.v[3258] != 0.0))) {
            s.store_mul_ad_lhs(338, A::mul(s.ad_value(877), s.ad_value(850)), 335);
        }

        if (((s.v[3256] != 0.0) && (s.v[3257] != 0.0)) && (!(s.v[3258] != 0.0))) {
            s.store_add_ad(889, A::add(A::add(A::mul(s.ad_value(877), A::offset(s.ad_value(335), (-1.0))), A::mul(s.ad_value(338), A::sub(s.ad_value(868), s.ad_value(848)))), A::mul(s.ad_value(334), A::offset(s.ad_value(336), (-1.0)))), A::scale(A::offset(s.ad_value(337), (-1.0)), s.v[821]));
        }

        if ((s.v[3256] != 0.0) && (!(s.v[3257] != 0.0))) {
            s.store_scalar(889, 0.0);
        }

        if (s.v[3256] != 0.0) {
            s.store_scale(346, 878, p.p514);
        }

        if (s.v[3256] != 0.0) {
            s.store_add_ad_rhs(889, 889, A::mul(s.ad_value(346), s.ad_value(868)));
        }

        if (!(s.v[3256] != 0.0)) {
            s.store_scalar(889, 0.0);
        }

        s.store_scale(344, 855, p.p534);

        s.store_scale(343, 854, p.p533);

        s.v[3260] = if (s.v[879] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[3260] != 0.0) {
            s.store_mul(334, 880, 343);
        }

        if (s.v[3260] != 0.0) {
            s.store_mul_ad_lhs(332, A::neg(s.ad_value(859)), 344);
        }

        if (s.v[3260] != 0.0) {
            s.store_exp(336, 332);
        }

        if (s.v[3260] != 0.0) {
            s.copy_ad(337, 336);
        }

        s.v[3261] = if (s.v[859] < s.v[853]) { 1.0 } else { 0.0 };

        if ((s.v[3260] != 0.0) && (s.v[3261] != 0.0)) {
            s.store_mul(332, 859, 855);
        }

        s.v[3262] = if (s.v[332] < ((-3.0) * 34.0)) { 1.0 } else { 0.0 };

        if (((s.v[3260] != 0.0) && (s.v[3261] != 0.0)) && (s.v[3262] != 0.0)) {
            s.store_scalar(335, 0.0);
        }

        if (((s.v[3260] != 0.0) && (s.v[3261] != 0.0)) && (!(s.v[3262] != 0.0))) {
            s.store_exp(335, 332);
        }

        if ((s.v[3260] != 0.0) && (s.v[3261] != 0.0)) {
            s.store_add_ad(886, A::add(A::mul(s.ad_value(879), A::offset(s.ad_value(335), (-1.0))), A::mul(s.ad_value(334), A::offset(s.ad_value(336), (-1.0)))), A::scale(A::offset(s.ad_value(337), (-1.0)), s.v[826]));
        }

        if ((s.v[3260] != 0.0) && (!(s.v[3261] != 0.0))) {
            s.copy_ad(335, 856);
        }

        if ((s.v[3260] != 0.0) && (!(s.v[3261] != 0.0))) {
            s.store_mul_ad_lhs(338, A::mul(s.ad_value(879), s.ad_value(855)), 335);
        }

        if ((s.v[3260] != 0.0) && (!(s.v[3261] != 0.0))) {
            s.store_add_ad(886, A::add(A::add(A::mul(s.ad_value(879), A::offset(s.ad_value(335), (-1.0))), A::mul(s.ad_value(338), A::sub(s.ad_value(859), s.ad_value(853)))), A::mul(s.ad_value(334), A::offset(s.ad_value(336), (-1.0)))), A::scale(A::offset(s.ad_value(337), (-1.0)), s.v[826]));
        }

        if (!(s.v[3260] != 0.0)) {
            s.store_scalar(886, 0.0);
        }

        s.store_scale(346, 880, p.p537);

        s.store_add_ad_rhs(886, 886, A::mul(s.ad_value(346), s.ad_value(859)));

        s.v[3263] = if (s.v[881] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[3263] != 0.0) {
            s.store_mul(334, 882, 343);
        }

        if (s.v[3263] != 0.0) {
            s.store_mul_ad_lhs(332, A::neg(s.ad_value(859)), 344);
        }

        if (s.v[3263] != 0.0) {
            s.store_exp(336, 332);
        }

        if (s.v[3263] != 0.0) {
            s.copy_ad(337, 336);
        }

        s.v[3264] = if (s.v[859] < s.v[853]) { 1.0 } else { 0.0 };

        if ((s.v[3263] != 0.0) && (s.v[3264] != 0.0)) {
            s.store_mul(332, 859, 855);
        }

        s.v[3265] = if (s.v[332] < ((-3.0) * 34.0)) { 1.0 } else { 0.0 };

        if (((s.v[3263] != 0.0) && (s.v[3264] != 0.0)) && (s.v[3265] != 0.0)) {
            s.store_scalar(335, 0.0);
        }

        if (((s.v[3263] != 0.0) && (s.v[3264] != 0.0)) && (!(s.v[3265] != 0.0))) {
            s.store_exp(335, 332);
        }

        if ((s.v[3263] != 0.0) && (s.v[3264] != 0.0)) {
            s.store_add_ad(888, A::add(A::mul(s.ad_value(881), A::offset(s.ad_value(335), (-1.0))), A::mul(s.ad_value(334), A::offset(s.ad_value(336), (-1.0)))), A::scale(A::offset(s.ad_value(337), (-1.0)), s.v[826]));
        }

        if ((s.v[3263] != 0.0) && (!(s.v[3264] != 0.0))) {
            s.copy_ad(335, 856);
        }

        if ((s.v[3263] != 0.0) && (!(s.v[3264] != 0.0))) {
            s.store_mul_ad_lhs(338, A::mul(s.ad_value(881), s.ad_value(855)), 335);
        }

        if ((s.v[3263] != 0.0) && (!(s.v[3264] != 0.0))) {
            s.store_add_ad(888, A::add(A::add(A::mul(s.ad_value(881), A::offset(s.ad_value(335), (-1.0))), A::mul(s.ad_value(338), A::sub(s.ad_value(859), s.ad_value(853)))), A::mul(s.ad_value(334), A::offset(s.ad_value(336), (-1.0)))), A::scale(A::offset(s.ad_value(337), (-1.0)), s.v[826]));
        }

        if (!(s.v[3263] != 0.0)) {
            s.store_scalar(888, 0.0);
        }

        s.store_scale(346, 882, p.p537);

        s.store_add_ad_rhs(888, 888, A::mul(s.ad_value(346), s.ad_value(859)));

        s.v[3266] = if (p.p48 > 0.0) { 1.0 } else { 0.0 };

        s.v[3267] = if (s.v[883] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[3266] != 0.0) && (s.v[3267] != 0.0)) {
            s.store_mul(334, 884, 343);
        }

        if ((s.v[3266] != 0.0) && (s.v[3267] != 0.0)) {
            s.store_mul_ad_lhs(332, A::neg(s.ad_value(867)), 344);
        }

        if ((s.v[3266] != 0.0) && (s.v[3267] != 0.0)) {
            s.store_exp(336, 332);
        }

        if ((s.v[3266] != 0.0) && (s.v[3267] != 0.0)) {
            s.copy_ad(337, 336);
        }

        s.v[3268] = if (s.v[867] < s.v[853]) { 1.0 } else { 0.0 };

        if (((s.v[3266] != 0.0) && (s.v[3267] != 0.0)) && (s.v[3268] != 0.0)) {
            s.store_mul(332, 867, 855);
        }

        s.v[3269] = if (s.v[332] < ((-3.0) * 34.0)) { 1.0 } else { 0.0 };

        if ((((s.v[3266] != 0.0) && (s.v[3267] != 0.0)) && (s.v[3268] != 0.0)) && (s.v[3269] != 0.0)) {
            s.store_scalar(335, 0.0);
        }

        if ((((s.v[3266] != 0.0) && (s.v[3267] != 0.0)) && (s.v[3268] != 0.0)) && (!(s.v[3269] != 0.0))) {
            s.store_exp(335, 332);
        }

        if (((s.v[3266] != 0.0) && (s.v[3267] != 0.0)) && (s.v[3268] != 0.0)) {
            s.store_add_ad(890, A::add(A::mul(s.ad_value(883), A::offset(s.ad_value(335), (-1.0))), A::mul(s.ad_value(334), A::offset(s.ad_value(336), (-1.0)))), A::scale(A::offset(s.ad_value(337), (-1.0)), s.v[826]));
        }

        if (((s.v[3266] != 0.0) && (s.v[3267] != 0.0)) && (!(s.v[3268] != 0.0))) {
            s.copy_ad(335, 856);
        }

        if (((s.v[3266] != 0.0) && (s.v[3267] != 0.0)) && (!(s.v[3268] != 0.0))) {
            s.store_mul_ad_lhs(338, A::mul(s.ad_value(883), s.ad_value(855)), 335);
        }

        if (((s.v[3266] != 0.0) && (s.v[3267] != 0.0)) && (!(s.v[3268] != 0.0))) {
            s.store_add_ad(890, A::add(A::add(A::mul(s.ad_value(883), A::offset(s.ad_value(335), (-1.0))), A::mul(s.ad_value(338), A::sub(s.ad_value(867), s.ad_value(853)))), A::mul(s.ad_value(334), A::offset(s.ad_value(336), (-1.0)))), A::scale(A::offset(s.ad_value(337), (-1.0)), s.v[826]));
        }

        if ((s.v[3266] != 0.0) && (!(s.v[3267] != 0.0))) {
            s.store_scalar(890, 0.0);
        }

        if (s.v[3266] != 0.0) {
            s.store_scale(346, 884, p.p537);
        }

        if (s.v[3266] != 0.0) {
            s.store_add_ad_rhs(890, 890, A::mul(s.ad_value(346), s.ad_value(867)));
        }

        if (!(s.v[3266] != 0.0)) {
            s.store_scalar(890, 0.0);
        }

        s.v[3270] = if (s.v[832] > 0.0) { 1.0 } else { 0.0 };

        s.v[3271] = if (s.v[860] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[3270] != 0.0) && (s.v[3271] != 0.0)) {
            s.store_sub_from_scalar_ad(770, 1.0, A::div(s.ad_value(860), s.ad_value(841)));
        }

        s.v[3272] = if (p.p503 == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[3270] != 0.0) && (s.v[3271] != 0.0)) && (s.v[3272] != 0.0)) {
            s.store_div_from_scalar_ad(840, 1.0, A::sqrt(s.ad_value(770)));
        }

        if (((s.v[3270] != 0.0) && (s.v[3271] != 0.0)) && (!(s.v[3272] != 0.0))) {
            s.store_ad(840, &{
                if (s.v[770] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(770), (-p.p503))
                }
            });
        }

        if ((s.v[3270] != 0.0) && (s.v[3271] != 0.0)) {
            s.store_scale_ad(891, A::mul(A::mul(s.ad_value(841), s.ad_value(832)), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840)))), 1.0 / ((1.0 - p.p503)));
        }

        if ((s.v[3270] != 0.0) && (!(s.v[3271] != 0.0))) {
            s.copy_ad(335, 832);
        }

        if ((s.v[3270] != 0.0) && (!(s.v[3271] != 0.0))) {
            s.store_div_ad_lhs(336, A::scale(s.ad_value(832), p.p503), 841);
        }

        if ((s.v[3270] != 0.0) && (!(s.v[3271] != 0.0))) {
            s.store_mul_ad_rhs(891, 860, A::add(s.ad_value(335), A::mul(A::scale(s.ad_value(860), 0.5), s.ad_value(336))));
        }

        if (!(s.v[3270] != 0.0)) {
            s.store_scalar(891, 0.0);
        }

        s.v[3273] = if (s.v[833] > 0.0) { 1.0 } else { 0.0 };

        s.v[3274] = if (s.v[860] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[3273] != 0.0) && (s.v[3274] != 0.0)) {
            s.store_sub_from_scalar_ad(770, 1.0, A::div(s.ad_value(860), s.ad_value(842)));
        }

        s.v[3275] = if (p.p504 == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[3273] != 0.0) && (s.v[3274] != 0.0)) && (s.v[3275] != 0.0)) {
            s.store_div_from_scalar_ad(840, 1.0, A::sqrt(s.ad_value(770)));
        }

        if (((s.v[3273] != 0.0) && (s.v[3274] != 0.0)) && (!(s.v[3275] != 0.0))) {
            s.store_ad(840, &{
                if (s.v[770] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(770), (-p.p504))
                }
            });
        }

        if ((s.v[3273] != 0.0) && (s.v[3274] != 0.0)) {
            s.store_scale_ad(893, A::mul(A::mul(s.ad_value(842), s.ad_value(833)), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840)))), 1.0 / ((1.0 - p.p504)));
        }

        if ((s.v[3273] != 0.0) && (!(s.v[3274] != 0.0))) {
            s.copy_ad(335, 833);
        }

        if ((s.v[3273] != 0.0) && (!(s.v[3274] != 0.0))) {
            s.store_div_ad_lhs(336, A::scale(s.ad_value(833), p.p504), 842);
        }

        if ((s.v[3273] != 0.0) && (!(s.v[3274] != 0.0))) {
            s.store_mul_ad_rhs(893, 860, A::add(s.ad_value(335), A::mul(A::scale(s.ad_value(860), 0.5), s.ad_value(336))));
        }

        if (!(s.v[3273] != 0.0)) {
            s.store_scalar(893, 0.0);
        }

        s.v[3276] = if (p.p48 > 0.0) { 1.0 } else { 0.0 };

        s.v[3277] = if (s.v[834] > 0.0) { 1.0 } else { 0.0 };

        s.v[3278] = if (s.v[868] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[3276] != 0.0) && (s.v[3277] != 0.0)) && (s.v[3278] != 0.0)) {
            s.store_sub_from_scalar_ad(770, 1.0, A::div(s.ad_value(868), s.ad_value(843)));
        }

    }

    pub(super) fn stamp_transient_block_89(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[3279] = if (p.p505 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[3276] != 0.0) && (s.v[3277] != 0.0)) && (s.v[3278] != 0.0)) && (s.v[3279] != 0.0)) {
            s.store_div_from_scalar_ad(840, 1.0, A::sqrt(s.ad_value(770)));
        }

        if ((((s.v[3276] != 0.0) && (s.v[3277] != 0.0)) && (s.v[3278] != 0.0)) && (!(s.v[3279] != 0.0))) {
            s.store_ad(840, &{
                if (s.v[770] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(770), (-p.p505))
                }
            });
        }

        if (((s.v[3276] != 0.0) && (s.v[3277] != 0.0)) && (s.v[3278] != 0.0)) {
            s.store_scale_ad(895, A::mul(A::mul(s.ad_value(843), s.ad_value(834)), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840)))), 1.0 / ((1.0 - p.p505)));
        }

        if (((s.v[3276] != 0.0) && (s.v[3277] != 0.0)) && (!(s.v[3278] != 0.0))) {
            s.copy_ad(335, 834);
        }

        if (((s.v[3276] != 0.0) && (s.v[3277] != 0.0)) && (!(s.v[3278] != 0.0))) {
            s.store_div_ad_lhs(336, A::scale(s.ad_value(834), p.p505), 843);
        }

        if (((s.v[3276] != 0.0) && (s.v[3277] != 0.0)) && (!(s.v[3278] != 0.0))) {
            s.store_mul_ad_rhs(895, 868, A::add(s.ad_value(335), A::mul(A::scale(s.ad_value(868), 0.5), s.ad_value(336))));
        }

        if ((s.v[3276] != 0.0) && (!(s.v[3277] != 0.0))) {
            s.store_scalar(895, 0.0);
        }

        s.v[3280] = if (s.v[834] > 0.0) { 1.0 } else { 0.0 };

        s.v[3281] = if (s.v[860] < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[3276] != 0.0)) && (s.v[3280] != 0.0)) && (s.v[3281] != 0.0)) {
            s.store_sub_from_scalar_ad(770, 1.0, A::div(s.ad_value(860), s.ad_value(843)));
        }

        s.v[3282] = if (p.p505 == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[3276] != 0.0)) && (s.v[3280] != 0.0)) && (s.v[3281] != 0.0)) && (s.v[3282] != 0.0)) {
            s.store_div_from_scalar_ad(840, 1.0, A::sqrt(s.ad_value(770)));
        }

        if ((((!(s.v[3276] != 0.0)) && (s.v[3280] != 0.0)) && (s.v[3281] != 0.0)) && (!(s.v[3282] != 0.0))) {
            s.store_ad(840, &{
                if (s.v[770] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(770), (-p.p505))
                }
            });
        }

        if (((!(s.v[3276] != 0.0)) && (s.v[3280] != 0.0)) && (s.v[3281] != 0.0)) {
            s.store_scale_ad(895, A::mul(A::mul(s.ad_value(843), s.ad_value(834)), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840)))), 1.0 / ((1.0 - p.p505)));
        }

        if (((!(s.v[3276] != 0.0)) && (s.v[3280] != 0.0)) && (!(s.v[3281] != 0.0))) {
            s.copy_ad(335, 834);
        }

        if (((!(s.v[3276] != 0.0)) && (s.v[3280] != 0.0)) && (!(s.v[3281] != 0.0))) {
            s.store_div_ad_lhs(336, A::scale(s.ad_value(834), p.p505), 843);
        }

        if (((!(s.v[3276] != 0.0)) && (s.v[3280] != 0.0)) && (!(s.v[3281] != 0.0))) {
            s.store_mul_ad_rhs(895, 860, A::add(s.ad_value(335), A::mul(A::scale(s.ad_value(860), 0.5), s.ad_value(336))));
        }

        if ((!(s.v[3276] != 0.0)) && (!(s.v[3280] != 0.0))) {
            s.store_scalar(895, 0.0);
        }

        s.v[3283] = if (s.v[835] > 0.0) { 1.0 } else { 0.0 };

        s.v[3284] = if (s.v[859] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[3283] != 0.0) && (s.v[3284] != 0.0)) {
            s.store_sub_from_scalar_ad(770, 1.0, A::div(s.ad_value(859), s.ad_value(844)));
        }

        s.v[3285] = if (p.p526 == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[3283] != 0.0) && (s.v[3284] != 0.0)) && (s.v[3285] != 0.0)) {
            s.store_div_from_scalar_ad(840, 1.0, A::sqrt(s.ad_value(770)));
        }

        if (((s.v[3283] != 0.0) && (s.v[3284] != 0.0)) && (!(s.v[3285] != 0.0))) {
            s.store_ad(840, &{
                if (s.v[770] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(770), (-p.p526))
                }
            });
        }

        if ((s.v[3283] != 0.0) && (s.v[3284] != 0.0)) {
            s.store_scale_ad(892, A::mul(A::mul(s.ad_value(844), s.ad_value(835)), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840)))), 1.0 / ((1.0 - p.p526)));
        }

        if ((s.v[3283] != 0.0) && (!(s.v[3284] != 0.0))) {
            s.copy_ad(335, 835);
        }

        if ((s.v[3283] != 0.0) && (!(s.v[3284] != 0.0))) {
            s.store_div_ad_lhs(336, A::scale(s.ad_value(835), p.p526), 844);
        }

        if ((s.v[3283] != 0.0) && (!(s.v[3284] != 0.0))) {
            s.store_mul_ad_rhs(892, 859, A::add(s.ad_value(335), A::mul(A::scale(s.ad_value(859), 0.5), s.ad_value(336))));
        }

        if (!(s.v[3283] != 0.0)) {
            s.store_scalar(892, 0.0);
        }

        s.v[3286] = if (s.v[838] > 0.0) { 1.0 } else { 0.0 };

        s.v[3287] = if (s.v[859] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[3286] != 0.0) && (s.v[3287] != 0.0)) {
            s.store_sub_from_scalar_ad(770, 1.0, A::div(s.ad_value(859), s.ad_value(845)));
        }

        s.v[3288] = if (p.p527 == 0.5) { 1.0 } else { 0.0 };

        if (((s.v[3286] != 0.0) && (s.v[3287] != 0.0)) && (s.v[3288] != 0.0)) {
            s.store_div_from_scalar_ad(840, 1.0, A::sqrt(s.ad_value(770)));
        }

        if (((s.v[3286] != 0.0) && (s.v[3287] != 0.0)) && (!(s.v[3288] != 0.0))) {
            s.store_ad(840, &{
                if (s.v[770] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(770), (-p.p527))
                }
            });
        }

        if ((s.v[3286] != 0.0) && (s.v[3287] != 0.0)) {
            s.store_scale_ad(894, A::mul(A::mul(s.ad_value(845), s.ad_value(838)), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840)))), 1.0 / ((1.0 - p.p527)));
        }

        if ((s.v[3286] != 0.0) && (!(s.v[3287] != 0.0))) {
            s.copy_ad(335, 838);
        }

        if ((s.v[3286] != 0.0) && (!(s.v[3287] != 0.0))) {
            s.store_div_ad_lhs(336, A::scale(s.ad_value(838), p.p527), 845);
        }

        if ((s.v[3286] != 0.0) && (!(s.v[3287] != 0.0))) {
            s.store_mul_ad_rhs(894, 859, A::add(s.ad_value(335), A::mul(A::scale(s.ad_value(859), 0.5), s.ad_value(336))));
        }

        if (!(s.v[3286] != 0.0)) {
            s.store_scalar(894, 0.0);
        }

        s.v[3289] = if (p.p48 > 0.0) { 1.0 } else { 0.0 };

        s.v[3290] = if (s.v[839] > 0.0) { 1.0 } else { 0.0 };

        s.v[3291] = if (s.v[867] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[3289] != 0.0) && (s.v[3290] != 0.0)) && (s.v[3291] != 0.0)) {
            s.store_sub_from_scalar_ad(770, 1.0, A::div(s.ad_value(867), s.ad_value(846)));
        }

        s.v[3292] = if (p.p528 == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[3289] != 0.0) && (s.v[3290] != 0.0)) && (s.v[3291] != 0.0)) && (s.v[3292] != 0.0)) {
            s.store_div_from_scalar_ad(840, 1.0, A::sqrt(s.ad_value(770)));
        }

        if ((((s.v[3289] != 0.0) && (s.v[3290] != 0.0)) && (s.v[3291] != 0.0)) && (!(s.v[3292] != 0.0))) {
            s.store_ad(840, &{
                if (s.v[770] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(770), (-p.p528))
                }
            });
        }

        if (((s.v[3289] != 0.0) && (s.v[3290] != 0.0)) && (s.v[3291] != 0.0)) {
            s.store_scale_ad(896, A::mul(A::mul(s.ad_value(846), s.ad_value(839)), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840)))), 1.0 / ((1.0 - p.p528)));
        }

        if (((s.v[3289] != 0.0) && (s.v[3290] != 0.0)) && (!(s.v[3291] != 0.0))) {
            s.copy_ad(335, 839);
        }

        if (((s.v[3289] != 0.0) && (s.v[3290] != 0.0)) && (!(s.v[3291] != 0.0))) {
            s.store_div_ad_lhs(336, A::scale(s.ad_value(839), p.p528), 846);
        }

        if (((s.v[3289] != 0.0) && (s.v[3290] != 0.0)) && (!(s.v[3291] != 0.0))) {
            s.store_mul_ad_rhs(896, 867, A::add(s.ad_value(335), A::mul(A::scale(s.ad_value(867), 0.5), s.ad_value(336))));
        }

        if ((s.v[3289] != 0.0) && (!(s.v[3290] != 0.0))) {
            s.store_scalar(896, 0.0);
        }

        s.v[3293] = if (s.v[839] > 0.0) { 1.0 } else { 0.0 };

        s.v[3294] = if (s.v[859] < 0.0) { 1.0 } else { 0.0 };

        if (((!(s.v[3289] != 0.0)) && (s.v[3293] != 0.0)) && (s.v[3294] != 0.0)) {
            s.store_sub_from_scalar_ad(770, 1.0, A::div(s.ad_value(859), s.ad_value(846)));
        }

        s.v[3295] = if (p.p528 == 0.5) { 1.0 } else { 0.0 };

        if ((((!(s.v[3289] != 0.0)) && (s.v[3293] != 0.0)) && (s.v[3294] != 0.0)) && (s.v[3295] != 0.0)) {
            s.store_div_from_scalar_ad(840, 1.0, A::sqrt(s.ad_value(770)));
        }

        if ((((!(s.v[3289] != 0.0)) && (s.v[3293] != 0.0)) && (s.v[3294] != 0.0)) && (!(s.v[3295] != 0.0))) {
            s.store_ad(840, &{
                if (s.v[770] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(770), (-p.p528))
                }
            });
        }

        if (((!(s.v[3289] != 0.0)) && (s.v[3293] != 0.0)) && (s.v[3294] != 0.0)) {
            s.store_scale_ad(896, A::mul(A::mul(s.ad_value(846), s.ad_value(839)), A::sub_from_scalar(1.0, A::mul(s.ad_value(770), s.ad_value(840)))), 1.0 / ((1.0 - p.p528)));
        }

        if (((!(s.v[3289] != 0.0)) && (s.v[3293] != 0.0)) && (!(s.v[3294] != 0.0))) {
            s.copy_ad(335, 839);
        }

        if (((!(s.v[3289] != 0.0)) && (s.v[3293] != 0.0)) && (!(s.v[3294] != 0.0))) {
            s.store_div_ad_lhs(336, A::scale(s.ad_value(839), p.p528), 846);
        }

        if (((!(s.v[3289] != 0.0)) && (s.v[3293] != 0.0)) && (!(s.v[3294] != 0.0))) {
            s.store_mul_ad_rhs(896, 859, A::add(s.ad_value(335), A::mul(A::scale(s.ad_value(859), 0.5), s.ad_value(336))));
        }

        if ((!(s.v[3289] != 0.0)) && (!(s.v[3293] != 0.0))) {
            s.store_scalar(896, 0.0);
        }

        s.store_scaled_add(862, 886, 888, s.v[365]);

        s.store_scaled_add(861, 885, 887, s.v[365]);

        s.v[3296] = if (p.p48 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[3296] != 0.0) {
            s.store_scale(870, 890, s.v[365]);
        }

        if (s.v[3296] != 0.0) {
            s.store_scale(869, 889, s.v[365]);
        }

        if (s.v[3296] != 0.0) {
            s.store_scaled_add(66, 892, 894, s.v[365]);
        }

        if (s.v[3296] != 0.0) {
            s.store_scaled_add(65, 891, 893, s.v[365]);
        }

        if (s.v[3296] != 0.0) {
            s.store_scale(68, 896, s.v[365]);
        }

        if (s.v[3296] != 0.0) {
            s.store_scale(67, 895, s.v[365]);
        }

        if (!(s.v[3296] != 0.0)) {
            s.store_scalar(870, 0.0);
        }

        if (!(s.v[3296] != 0.0)) {
            s.store_scalar(869, 0.0);
        }

        if (!(s.v[3296] != 0.0)) {
            s.store_scale_ad(66, A::add(A::add(s.ad_value(892), s.ad_value(894)), s.ad_value(896)), s.v[365]);
        }

        if (!(s.v[3296] != 0.0)) {
            s.store_scale_ad(65, A::add(A::add(s.ad_value(891), s.ad_value(893)), s.ad_value(895)), s.v[365]);
        }

        if (!(s.v[3296] != 0.0)) {
            s.store_scalar(68, 0.0);
        }

        if (!(s.v[3296] != 0.0)) {
            s.store_scalar(67, 0.0);
        }

        s.v[903] = (p.p540 / 1e-6);

        s.v[906] = s.v[820];

        s.v[904] = (1450.0 / 10000.0);

        s.v[905] = (500.0 / 10000.0);

        s.v[943] = 0.001;

        s.store_scale_ad(908, A::exp(A::scale(A::add(A::sub(A::scale(s.ad_value(678), s.v[616]), A::mul(s.ad_value(393), s.ad_value(154))), A::scale(s.ad_value(590), p.p499)), 1.0 / (s.v[820]))), 1.45e16);

        s.store_scale_ad(907, A::square(s.ad_value(908)), 1.0 / (s.v[903]));

        s.store_powf(335, 676, (-1.5));

        s.store_mul_ad_lhs(909, A::scale(s.ad_value(335), s.v[904]), 155);

        s.store_mul_ad_lhs(910, A::scale(s.ad_value(335), s.v[905]), 155);

        s.store_div_ad(911, A::mul(A::scale(s.ad_value(909), 2.0), s.ad_value(910)), A::add(s.ad_value(909), s.ad_value(910)));

        s.store_powf(336, 676, p.p547);

        s.store_scale(913, 336, p.p544);

        s.store_sqrt_ad(912, A::mul(s.ad_value(913), s.ad_value(911)));

        s.store_mul_ad(934, A::scale(s.ad_value(155), s.v[906]), A::ln(A::div_from_scalar(s.v[903], s.ad_value(907))));

        s.store_mul_ad(935, A::scale(s.ad_value(155), s.v[906]), A::add(A::ln(A::div_from_scalar(s.v[903], s.ad_value(907))), A::div_from_scalar(p.p545, s.ad_value(912))));

        s.v[3297] = if (p.p539 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[3297] != 0.0) {
            s.store_scalar(936, s.v[820]);
        }

        if (s.v[3297] != 0.0) {
            s.store_exp_ad(937, A::mul(s.ad_value(860), s.ad_value(850)));
        }

        s.v[3298] = if ((s.v[860] - (s.v[935] - s.v[934])) > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[3297] != 0.0) && (s.v[3298] != 0.0)) {
            s.store_exp_ad(938, A::mul(s.ad_value(154), A::sub(A::div(s.ad_value(860), s.ad_value(936)), A::div(A::sub(s.ad_value(935), s.ad_value(934)), s.ad_value(936)))));
        }

        if ((s.v[3297] != 0.0) && (!(s.v[3298] != 0.0))) {
            s.store_scalar(938, 1.0);
        }

        s.v[3299] = if ((p.p542 == 0.0) || (s.v[860] < s.v[934])) { 1.0 } else { 0.0 };

        if ((s.v[3297] != 0.0) && (s.v[3299] != 0.0)) {
            s.store_scale(941, 937, p.p541);
        }

        if ((s.v[3297] != 0.0) && (!(s.v[3299] != 0.0))) {
            s.store_mul_ad(941, A::scale(s.ad_value(937), p.p541), A::exp(A::mul(A::mul(A::scale(A::sub(s.ad_value(860), s.ad_value(934)), (-p.p542)), A::sub(s.ad_value(860), s.ad_value(934))), A::exp(A::scale(A::ln(A::div_from_scalar(1.0, s.ad_value(676))), p.p548)))));
        }

        if (s.v[3297] != 0.0) {
            s.store_ad(941, &{
                if (s.v[941] > 1e20) {
                    A::constant(1e20)
                } else {
                    s.ad_value(941)
                }
            });
        }

        if (s.v[3297] != 0.0) {
            s.store_mul(939, 907, 941);
        }

        if (s.v[3297] != 0.0) {
            s.store_scaled_sub(920, 939, 907, (1.6021918e-19 * p.p13));
        }

        s.v[3300] = if (p.p543 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[3297] != 0.0) && (s.v[3300] != 0.0)) {
            s.store_scale(922, 920, p.p543);
        }

        if ((s.v[3297] != 0.0) && (s.v[3300] != 0.0)) {
            s.store_ad(924, &A::scale(A::voltage(ctx, &nodes, Some(15), None), p.p543));
        }

        if ((s.v[3297] != 0.0) && (s.v[3300] != 0.0)) {
            s.store_scaled_sub(926, 924, 922, 1.0 / (p.p543));
        }

        if ((s.v[3297] != 0.0) && (s.v[3300] != 0.0)) {
            s.store_scale(928, 924, 1.0 / (p.p543));
        }

        if ((s.v[3297] != 0.0) && (!(s.v[3300] != 0.0))) {
            s.copy_ad(922, 920);
        }

        if ((s.v[3297] != 0.0) && (!(s.v[3300] != 0.0))) {
            s.copy_ad(928, 922);
        }

        s.v[3301] = if ((p.p542 == 0.0) || (s.v[860] < s.v[935])) { 1.0 } else { 0.0 };

        if ((s.v[3297] != 0.0) && (s.v[3301] != 0.0)) {
            s.store_scale(942, 938, p.p541);
        }

        if ((s.v[3297] != 0.0) && (!(s.v[3301] != 0.0))) {
            s.store_mul_ad(942, A::scale(s.ad_value(938), p.p541), A::exp(A::mul(A::mul(A::scale(A::sub(s.ad_value(860), s.ad_value(935)), (-p.p542)), A::sub(s.ad_value(860), s.ad_value(935))), A::exp(A::scale(A::ln(A::div_from_scalar(1.0, s.ad_value(676))), p.p548)))));
        }

        if (s.v[3297] != 0.0) {
            s.store_ad(942, &{
                if (s.v[942] > 1e20) {
                    A::constant(1e20)
                } else {
                    s.ad_value(942)
                }
            });
        }

        if (s.v[3297] != 0.0) {
            s.store_mul(940, 907, 942);
        }

        if (s.v[3297] != 0.0) {
            s.store_scaled_sub(921, 940, 907, (1.6021918e-19 * p.p13));
        }

        s.v[3302] = if (p.p543 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[3297] != 0.0) && (s.v[3302] != 0.0)) {
            s.store_scale(923, 921, p.p543);
        }

        if ((s.v[3297] != 0.0) && (s.v[3302] != 0.0)) {
            s.store_ad(925, &A::scale(A::voltage(ctx, &nodes, Some(16), None), p.p543));
        }

        if ((s.v[3297] != 0.0) && (s.v[3302] != 0.0)) {
            s.store_scaled_sub(927, 925, 923, 1.0 / (p.p543));
        }

        if ((s.v[3297] != 0.0) && (s.v[3302] != 0.0)) {
            s.store_scale(929, 925, 1.0 / (p.p543));
        }

        if ((s.v[3297] != 0.0) && (!(s.v[3302] != 0.0))) {
            s.copy_ad(923, 921);
        }

        if ((s.v[3297] != 0.0) && (!(s.v[3302] != 0.0))) {
            s.copy_ad(929, 923);
        }

        if (s.v[3297] != 0.0) {
            s.store_sub_from_scalar(914, p.p506, 860);
        }

        if (s.v[3297] != 0.0) {
            s.store_sqrt_ad(782, A::offset(A::square(s.ad_value(914)), ((4.0 * s.v[943]) * s.v[943])));
        }

        if (s.v[3297] != 0.0) {
            s.store_scale_ad(334, A::offset(A::div(s.ad_value(914), s.ad_value(782)), 1.0), 0.5);
        }

        if (s.v[3297] != 0.0) {
            s.store_scaled_add(914, 914, 782, 0.5);
        }

        s.v[3303] = if (s.v[914] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[3297] != 0.0) && (s.v[3303] != 0.0)) {
            s.store_scalar(914, 0.0);
        }

        if ((s.v[3297] != 0.0) && (s.v[3303] != 0.0)) {
            s.store_scalar(334, 0.0);
        }

        if (s.v[3297] != 0.0) {
            s.store_sqrt_ad(915, A::scale(s.ad_value(914), ((2.0 * 1.034943e-10) * 1.0 / ((1.6021918e-19 * s.v[903])))));
        }

        if (s.v[3297] != 0.0) {
            s.store_offset_ad(781, A::sub_from_scalar(p.p545, s.ad_value(915)), (-1e-7));
        }

        if (s.v[3297] != 0.0) {
            s.store_scalar(782, ((4.0 * p.p545) * 1e-7));
        }

    }

    pub(super) fn stamp_transient_block_90(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[3297] != 0.0) {
            s.store_ad(782, &{
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (s.v[3297] != 0.0) {
            s.store_sqrt_ad(782, A::add(A::square(s.ad_value(781)), s.ad_value(782)));
        }

        if (s.v[3297] != 0.0) {
            s.store_scale_ad(334, A::offset(A::div(s.ad_value(781), s.ad_value(782)), 1.0), 0.5);
        }

        if (s.v[3297] != 0.0) {
            s.store_sub_from_scalar_ad(915, p.p545, A::scale(A::add(s.ad_value(781), s.ad_value(782)), 0.5));
        }

        s.v[3304] = if (p.p546 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[3297] != 0.0) && (s.v[3304] != 0.0)) {
            s.store_scale(930, 915, p.p546);
        }

        if ((s.v[3297] != 0.0) && (s.v[3304] != 0.0)) {
            s.store_ad(931, &A::scale(A::voltage(ctx, &nodes, Some(17), None), p.p546));
        }

        if ((s.v[3297] != 0.0) && (s.v[3304] != 0.0)) {
            s.store_scaled_sub(932, 931, 930, 1.0 / (p.p546));
        }

        if ((s.v[3297] != 0.0) && (s.v[3304] != 0.0)) {
            s.store_scale(933, 931, 1.0 / (p.p546));
        }

        if ((s.v[3297] != 0.0) && (!(s.v[3304] != 0.0))) {
            s.copy_ad(930, 915);
        }

        if ((s.v[3297] != 0.0) && (!(s.v[3304] != 0.0))) {
            s.copy_ad(933, 930);
        }

        if (s.v[3297] != 0.0) {
            s.store_scalar(916, ((-((s.v[903] * p.p13) * 1.6021918e-19)) * p.p545));
        }

        if (s.v[3297] != 0.0) {
            s.store_mul_ad(917, A::mul(s.ad_value(912), s.ad_value(928)), A::sub(A::exp(A::div_from_scalar((-p.p545), s.ad_value(912))), A::exp(A::div(A::neg(s.ad_value(933)), s.ad_value(912)))));
        }

        if (s.v[3297] != 0.0) {
            s.store_mul_ad(918, A::mul(s.ad_value(912), s.ad_value(929)), A::offset(A::exp(A::div(A::neg(A::sub_from_scalar(p.p545, s.ad_value(933))), s.ad_value(912))), (-1.0)));
        }

        if (s.v[3297] != 0.0) {
            s.store_neg_ad(919, A::add(A::add(s.ad_value(916), s.ad_value(917)), s.ad_value(918)));
        }

        if (s.v[3297] != 0.0) {
            s.store_add_ad_rhs(65, 65, A::scale(s.ad_value(919), s.v[365]));
        }

        s.v[3305] = if ((p.p539 > 0.0) && (p.p543 > 0.0)) { 1.0 } else { 0.0 };

        s.v[3306] = if ((p.p539 > 0.0) && (p.p546 > 0.0)) { 1.0 } else { 0.0 };

        s.v[3307] = if (p.p46 == 1.0) { 1.0 } else { 0.0 };

        s.v[3308] = if ((s.v[486] > 0.0) && (s.v[454] > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) {
            s.store_mul(335, 665, 85);
        }

        if ((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) {
            s.store_scale(337, 636, 1.0 / ((s.v[188] * s.v[188])));
        }

        if ((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) {
            s.store_scale_ad(338, A::div_from_scalar(2.0, s.ad_value(636)), (s.v[188] * s.v[188]));
        }

        if ((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) {
            s.store_sub_ad(339, A::sub(s.ad_value(335), s.ad_value(155)), A::mul(s.ad_value(666), s.ad_value(1434)));
        }

        if ((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) {
            s.store_offset_ad(340, A::mul(s.ad_value(338), s.ad_value(339)), 1.0);
        }

        if ((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) {
            s.store_scaled_offset(341, 338, 1.0, 2.0);
        }

        s.v[3309] = if ((s.v[340] < s.v[341]) && (s.v[341] >= 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) {
            s.store_sub(781, 341, 340);
        }

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) {
            s.store_square(722, 781);
        }

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) {
            s.store_square(723, 341);
        }

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) {
            s.store_scalar(724, 1.0);
        }

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) {
            s.store_scalar(725, 1.0);
        }

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) {
            s.store_scalar(719, 0.0);
        }

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) {
            s.store_scalar(720, 0.0);
        }

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) {
            s.store_scalar(770, 0.0);
        }

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) {
            s.store_scalar(726, 0.0);
        }

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) {
            s.store_mul(724, 724, 722);
        }

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) {
            s.store_mul(725, 725, 723);
        }

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) {
            s.store_mul(724, 724, 722);
        }

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) {
            s.store_mul(725, 725, 723);
        }

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) {
            s.store_mul(724, 724, 722);
        }

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) {
            s.store_mul(725, 725, 723);
        }

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) {
            s.store_mul(724, 724, 722);
        }

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) {
            s.store_mul(725, 725, 723);
        }

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) {
            s.store_add(770, 724, 725);
        }

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) {
            s.copy_ad(726, 770);
        }

        s.v[3310] = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[3311] = if (4.0 == 1.0) { 1.0 } else { 0.0 };

        if (((((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) && (s.v[3310] != 0.0)) && (s.v[3311] != 0.0)) {
            s.store_scalar(720, 1.0);
        }

        s.v[3312] = if (4.0 == 2.0) { 1.0 } else { 0.0 };

        if ((((((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) && (s.v[3310] != 0.0)) && (!(s.v[3311] != 0.0))) && (s.v[3312] != 0.0)) {
            s.store_scalar(720, 2.0);
        }

        s.v[3313] = if (4.0 == 4.0) { 1.0 } else { 0.0 };

        if (((((((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) && (s.v[3310] != 0.0)) && (!(s.v[3311] != 0.0))) && (!(s.v[3312] != 0.0))) && (s.v[3313] != 0.0)) {
            s.store_scalar(720, 3.0);
        }

        s.v[3314] = if (4.0 == 8.0) { 1.0 } else { 0.0 };

        if ((((((((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) && (s.v[3310] != 0.0)) && (!(s.v[3311] != 0.0))) && (!(s.v[3312] != 0.0))) && (!(s.v[3313] != 0.0))) && (s.v[3314] != 0.0)) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) && (s.v[3310] != 0.0)) {
            s.store_scalar(719, 0.0);
        }

        let mut assign100760_loop_guard: usize = 0;
        while {
            let assign100760_cond_e152863: f64 = if (((((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) && (s.v[3310] != 0.0)) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign100760_cond_e152863 != 0.0
        } {
            assign100760_loop_guard += 1;
            assert!(assign100760_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) && (s.v[3310] != 0.0)) {
                s.store_sqrt(726, 726);
            }
            if ((((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) && (s.v[3310] != 0.0)) {
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) && (!(s.v[3310] != 0.0))) {
            s.store_ad(726, &{
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                }
            });
        }

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) {
            s.store_div_from_scalar(726, 1.0, 726);
        }

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) {
            s.store_mul_ad_lhs(780, A::mul(s.ad_value(781), s.ad_value(341)), 726);
        }

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) {
            s.store_div_ad_lhs(334, A::mul(A::mul(s.ad_value(341), s.ad_value(725)), s.ad_value(726)), 770);
        }

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) {
            s.store_sub(340, 341, 780);
        }

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3309] != 0.0)) {
        }

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (!(s.v[3309] != 0.0))) {
        }

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (!(s.v[3309] != 0.0))) {
            s.store_scalar(334, 1.0);
        }

        if ((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) {
            s.store_sqrt(340, 340);
        }

        if ((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) {
            s.store_add_ad_rhs(282, 335, A::mul(s.ad_value(337), A::sub_from_scalar(1.0, s.ad_value(340))));
        }

        if ((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) {
            s.store_div_from_scalar_ad(336, s.v[582], A::offset(s.ad_value(667), s.v[582]));
        }

        if ((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) {
            s.store_sub_ad(283, A::add(A::scale(s.ad_value(1435), s.v[488]), s.ad_value(109)), A::mul(s.ad_value(336), s.ad_value(282)));
        }

        if ((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) {
            s.store_sqrt_ad(782, A::offset(A::square(s.ad_value(283)), ((4.0 * 0.001) * 0.001)));
        }

        if ((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) {
            s.store_scale_ad(343, A::offset(A::div(s.ad_value(283), s.ad_value(782)), 1.0), 0.5);
        }

        if ((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) {
            s.store_scaled_add(283, 283, 782, 0.5);
        }

        s.v[3315] = if (s.v[283] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3315] != 0.0)) {
            s.store_scalar(283, 0.0);
        }

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3315] != 0.0)) {
            s.store_scalar(343, 0.0);
        }

        if ((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) {
            s.store_offset(283, 283, 1e-25);
        }

        if ((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) {
            s.store_offset_ad(958, A::mul(s.ad_value(957), A::offset(s.ad_value(387), (-s.v[764]))), 1.0);
        }

        if ((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) {
            s.store_ad(958, &{
                if (s.v[958] <= 0.001) {
                    A::constant(0.001)
                } else {
                    s.ad_value(958)
                }
            });
        }

        if ((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) {
            s.store_div(339, 668, 958);
        }

        if ((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) {
            s.store_mul(340, 669, 958);
        }

        if ((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) {
            s.store_exp_ad(336, A::div(A::neg(s.ad_value(340)), s.ad_value(283)));
        }

        if ((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) {
            s.store_mul_ad_lhs(428, A::mul(s.ad_value(339), s.ad_value(283)), 336);
        }

        s.v[3316] = if (p.p48 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (s.v[3316] != 0.0)) {
            s.store_mul_ad_lhs(429, A::offset(s.ad_value(428), 1.0), 870);
        }

        if (((s.v[3307] != 0.0) && (s.v[3308] != 0.0)) && (!(s.v[3316] != 0.0))) {
            s.store_mul_ad_lhs(429, A::offset(s.ad_value(428), 1.0), 862);
        }

        s.v[3317] = if (s.v[78] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[81] != 0.0) && (s.v[3317] != 0.0)) {
            s.store_scalar(346, p.p270);
        }

        if ((s.v[81] != 0.0) && (s.v[3317] != 0.0)) {
            s.store_scalar(344, p.p271);
        }

        if ((s.v[81] != 0.0) && (s.v[3317] != 0.0)) {
            s.copy_ad(337, 170);
        }

        if ((s.v[81] != 0.0) && (s.v[3317] != 0.0)) {
            s.store_mul_ad_lhs(335, A::mul(A::mul(s.ad_value(346), s.ad_value(344)), s.ad_value(337)), 337);
        }

        if ((s.v[81] != 0.0) && (s.v[3317] != 0.0)) {
            s.store_offset_ad(336, A::add(A::mul(A::mul(s.ad_value(253), s.ad_value(127)), s.ad_value(346)), A::mul(A::mul(s.ad_value(344), s.ad_value(337)), s.ad_value(337))), 1e-25);
        }

        if ((s.v[81] != 0.0) && (s.v[3317] != 0.0)) {
            s.store_div(306, 335, 336);
        }

        if ((s.v[81] != 0.0) && (!(s.v[3317] != 0.0))) {
            s.store_scalar(306, p.p270);
        }

        if (s.v[81] != 0.0) {
            s.store_scalar(336, s.v[565]);
        }

        if (s.v[81] != 0.0) {
            s.store_mul(307, 336, 185);
        }

        s.v[3318] = if ((p.p26 != 0.0) && (!(s.v[78] != 0.0))) { 1.0 } else { 0.0 };

        if (s.v[3318] != 0.0) {
            s.store_scalar(309, s.v[522]);
        }

        if (s.v[3318] != 0.0) {
            s.store_scalar(310, s.v[521]);
        }

        if (s.v[3318] != 0.0) {
            s.store_scalar(311, s.v[563]);
        }

        if (s.v[3318] != 0.0) {
            s.store_scale(335, 238, 6.241449993689894e18);
        }

        if (s.v[3318] != 0.0) {
            s.store_sqrt_ad(782, A::offset(A::mul(A::sub(s.ad_value(87), s.ad_value(1431)), A::sub(s.ad_value(87), s.ad_value(1431))), ((4.0 * 0.001) * 0.001)));
        }

        if (s.v[3318] != 0.0) {
            s.store_scale_ad(334, A::offset(A::div(A::sub(s.ad_value(87), s.ad_value(1431)), s.ad_value(782)), 1.0), 0.5);
        }

        if (s.v[3318] != 0.0) {
            s.store_scale_ad(339, A::add(A::sub(s.ad_value(87), s.ad_value(1431)), s.ad_value(782)), 0.5);
        }

        s.v[3319] = if (s.v[339] < 0.0) { 1.0 } else { 0.0 };

        if ((s.v[3318] != 0.0) && (s.v[3319] != 0.0)) {
            s.store_scalar(339, 0.0);
        }

        if ((s.v[3318] != 0.0) && (s.v[3319] != 0.0)) {
            s.store_scalar(334, 0.0);
        }

        if (s.v[3318] != 0.0) {
            s.store_scale_ad(336, A::mul(A::add(A::add(s.ad_value(185), A::div(s.ad_value(238), s.ad_value(339))), s.ad_value(311)), s.ad_value(155)), 6.241449993689894e18);
        }

        if (s.v[3318] != 0.0) {
            s.store_sub_ad_lhs(337, A::scale(A::div(A::scale(s.ad_value(979), ((-2.0) * 6.241449993689894e18)), s.ad_value(170)), 1.0 / (s.v[635])), 335);
        }

        s.v[3320] = if ((((s.v[337] - s.v[335])) as f64).abs() > (10.0 * 2.220446049250313e-16)) { 1.0 } else { 0.0 };

        if ((s.v[3318] != 0.0) && (s.v[3320] != 0.0)) {
            let assign101290_ad_e153400: A = A::add(A::div(A::div_from_scalar(1.0, A::add(s.ad_value(335), s.ad_value(336))), A::add(s.ad_value(337), s.ad_value(336))), A::mul(A::div(A::mul(A::mul(A::scale(s.ad_value(309), 2.0), s.ad_value(255)), s.ad_value(253)), A::sub(s.ad_value(337), s.ad_value(335))), A::ln(A::div(A::add(s.ad_value(337), s.ad_value(336)), A::add(s.ad_value(335), s.ad_value(336))))));
            s.store_add_ad(338, assign101290_ad_e153400, A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(309), s.ad_value(255)), s.ad_value(253)), s.ad_value(309)), s.ad_value(255)), s.ad_value(253)));
        }

        if ((s.v[3318] != 0.0) && (!(s.v[3320] != 0.0))) {
            let assign101300_ad_e153451: A = A::add(A::add(A::div(A::div_from_scalar(1.0, A::add(s.ad_value(335), s.ad_value(336))), A::add(s.ad_value(337), s.ad_value(336))), A::div(A::mul(A::mul(A::scale(s.ad_value(309), 2.0), s.ad_value(255)), s.ad_value(253)), A::add(s.ad_value(335), s.ad_value(336)))), A::mul(A::mul(A::mul(A::mul(A::mul(s.ad_value(309), s.ad_value(255)), s.ad_value(253)), s.ad_value(309)), s.ad_value(255)), s.ad_value(253)));
            s.store_ad(338, &assign101300_ad_e153451);
        }

        if (s.v[3318] != 0.0) {
            s.store_mul_ad_lhs(312, A::div(A::mul(A::square(s.ad_value(134)), s.ad_value(310)), A::scale(A::mul(s.ad_value(170), s.ad_value(154)), s.v[632])), 338);
        }

        if (!(s.v[3318] != 0.0)) {
            s.store_scalar(312, 0.0);
        }

        s.v[3321] = if (((p.p30 != 0.0) && (!(s.v[78] != 0.0))) && (s.v[963] == 0.0)) { 1.0 } else { 0.0 };

        if (s.v[3321] != 0.0) {
            s.store_div_ad_lhs(313, A::offset(A::sub(s.ad_value(168), s.ad_value(87)), (10.0 * 2.220446049250313e-16)), 170);
        }

        if (s.v[3321] != 0.0) {
            s.store_ad(313, &{
                if (s.v[313] >= 0.0) {
                    s.ad_value(313)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[3321] != 0.0) {
            s.store_scaled_mul(346, 254, 313, 1e-7);
        }

        s.v[3322] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((s.v[3321] != 0.0) && (s.v[3322] != 0.0)) {
            s.store_scalar(341, 1.0);
        }

        s.v[3323] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[3321] != 0.0) && (!(s.v[3322] != 0.0))) && (s.v[3323] != 0.0)) {
            s.copy_ad(341, 346);
        }

        if (((s.v[3321] != 0.0) && (!(s.v[3322] != 0.0))) && (!(s.v[3323] != 0.0))) {
            s.store_ad(341, &{
                if (s.v[313] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(313), (p.p178 - 1.0))
                }
            });
        }

        if (s.v[3321] != 0.0) {
            s.store_mul(342, 346, 341);
        }

        if (s.v[3321] != 0.0) {
            s.store_offset(343, 342, 1.0);
        }

        if (s.v[3321] != 0.0) {
            s.store_ad(344, &{
                if (s.v[343] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(343), (((-1.0) / p.p178) - 1.0))
                }
            });
        }

        if (s.v[3321] != 0.0) {
            s.store_mul(345, 343, 344);
        }

    }

    pub(super) fn stamp_transient_block_91(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[3321] != 0.0) {
            s.store_mul(316, 254, 345);
        }

        if (s.v[3321] != 0.0) {
            s.store_scaled_add(314, 253, 316, 0.5);
        }

        if (s.v[3321] != 0.0) {
            s.store_square(334, 125);
        }

        if (s.v[3321] != 0.0) {
            let assign101490_ad_e153678: A = A::add(A::add(A::mul(A::mul(A::add(A::offset(A::scale(s.ad_value(125), 3.0), 1.0), A::scale(s.ad_value(334), 6.0)), s.ad_value(316)), s.ad_value(316)), A::mul(A::mul(A::add(A::offset(A::scale(s.ad_value(125), 4.0), 3.0), A::scale(s.ad_value(334), 3.0)), s.ad_value(316)), s.ad_value(253))), A::mul(A::mul(A::add(A::offset(A::scale(s.ad_value(125), 3.0), 6.0), s.ad_value(334)), s.ad_value(253)), s.ad_value(253)));
            s.store_div_ad(315, A::mul(A::mul(A::mul(A::scale(s.ad_value(185), s.v[632]), s.ad_value(127)), s.ad_value(253)), assign101490_ad_e153678), A::mul(A::mul(A::mul(A::scale(s.ad_value(170), 15.0), A::offset(s.ad_value(125), 1.0)), s.ad_value(314)), s.ad_value(314)));
        }

        if (!(s.v[3321] != 0.0)) {
            s.store_scalar(315, 0.0);
        }

        s.v[3324] = if (((((p.p31 != 0.0) && (p.p30 != 0.0)) && (s.v[321] == 1.0)) && (!(s.v[78] != 0.0))) && (s.v[963] == 0.0)) { 1.0 } else { 0.0 };

        if (s.v[3324] != 0.0) {
            s.store_sqrt(322, 319);
        }

        if (s.v[3324] != 0.0) {
            s.store_add(336, 127, 322);
        }

        if (s.v[3324] != 0.0) {
            s.store_square(337, 317);
        }

        if (s.v[3324] != 0.0) {
            s.store_square(338, 319);
        }

        if (s.v[3324] != 0.0) {
            s.store_mul_ad_lhs(339, A::scale(s.ad_value(317), 42.0), 319);
        }

        if (s.v[3324] != 0.0) {
            s.store_add_ad_rhs(339, 339, A::scale(A::add(s.ad_value(337), s.ad_value(338)), 4.0));
        }

        if (s.v[3324] != 0.0) {
            s.store_add_ad_rhs(339, 339, A::mul(A::mul(A::scale(s.ad_value(322), 20.0), s.ad_value(127)), A::add(s.ad_value(317), s.ad_value(319))));
        }

        if (s.v[3324] != 0.0) {
            s.store_square(344, 336);
        }

        if (s.v[3324] != 0.0) {
            s.store_square(344, 344);
        }

        if (s.v[3324] != 0.0) {
            s.store_div_ad_rhs(323, 339, A::mul(s.ad_value(344), s.ad_value(336)));
        }

        if (s.v[3324] != 0.0) {
            s.store_mul_ad_lhs(324, A::mul(A::div_from_scalar(s.v[632], s.ad_value(170)), s.ad_value(253)), 185);
        }

        if (s.v[3324] != 0.0) {
            s.store_mul(325, 324, 127);
        }

        if (s.v[3324] != 0.0) {
            s.store_div(326, 315, 325);
        }

        if (s.v[3324] != 0.0) {
            s.store_add_ad_lhs(341, A::add(s.ad_value(317), A::mul(A::scale(s.ad_value(127), 4.0), s.ad_value(322))), 319);
        }

        if (s.v[3324] != 0.0) {
            s.store_div_ad(327, A::mul(A::scale(s.ad_value(320), 3.872983346207417), s.ad_value(341)), A::mul(A::scale(s.ad_value(336), 6.0), A::sqrt(A::mul(A::mul(A::mul(s.ad_value(326), s.ad_value(336)), s.ad_value(127)), s.ad_value(339)))));
        }

        s.store_scale(0, 134, s.v[365]);

        s.store_scale(699, 400, s.v[365]);

        s.copy_ad(430, 429);

        s.v[705] = 0.0;

        s.v[706] = 0.0;

        s.v[707] = 0.0;

        s.v[811] = 0.0;

        s.v[810] = 0.0;

        s.v[812] = 0.0;

        s.v[703] = 0.0;

        s.v[704] = 0.0;

        s.v[3325] = if ((s.v[81] != 0.0) || (p.p22 == 2.0)) { 1.0 } else { 0.0 };

        if (s.v[3325] != 0.0) {
            s.store_scalar(700, 0.0);
        }

        if (s.v[3325] != 0.0) {
            s.store_scalar(701, 0.0);
        }

        if (s.v[3325] != 0.0) {
            s.store_scalar(702, 0.0);
        }

        if (s.v[3325] != 0.0) {
            s.copy_ad(708, 247);
        }

        if (s.v[3325] != 0.0) {
            s.store_scale(754, 20, s.v[365]);
        }

        if (s.v[3325] != 0.0) {
            s.store_scale(132, 132, s.v[365]);
        }

        if (!(s.v[3325] != 0.0)) {
            s.store_scale_ad(700, A::neg(A::add(s.ad_value(20), s.ad_value(132))), s.v[365]);
        }

        if (!(s.v[3325] != 0.0)) {
            s.store_scale(701, 19, s.v[365]);
        }

        if (!(s.v[3325] != 0.0)) {
            s.store_scaled_sub(702, 132, 19, s.v[365]);
        }

        if (p.p29 != 0.0) {
            s.store_scale(572, 91, s.v[572]);
        }

        if (p.p29 != 0.0) {
            s.store_sqrt_ad(782, A::offset(A::square(s.ad_value(572)), ((4.0 * 1e-12) * 1e-12)));
        }

        if (p.p29 != 0.0) {
            s.store_scale_ad(334, A::offset(A::div(s.ad_value(572), s.ad_value(782)), 1.0), 0.5);
        }

        if (p.p29 != 0.0) {
            s.store_scaled_add(572, 572, 782, 0.5);
        }

        s.v[3326] = if (s.v[572] < 0.0) { 1.0 } else { 0.0 };

        if ((p.p29 != 0.0) && (s.v[3326] != 0.0)) {
            s.store_scalar(572, 0.0);
        }

        if ((p.p29 != 0.0) && (s.v[3326] != 0.0)) {
            s.store_scalar(334, 0.0);
        }

        if (p.p29 != 0.0) {
            s.store_scale(308, 572, s.v[188]);
        }

        if (p.p29 != 0.0) {
            s.store_ad(817, &A::voltage(ctx, &nodes, Some(13), None));
        }

        if (p.p29 != 0.0) {
            s.store_div_ad_lhs(815, A::sub(s.ad_value(817), s.ad_value(816)), 308);
        }

        if (p.p29 != 0.0) {
            s.store_sub_ad_rhs(352, 352, A::sub(s.ad_value(816), s.ad_value(817)));
        }

        if (p.p29 != 0.0) {
            s.copy_ad(355, 817);
        }

        if (!(p.p29 != 0.0)) {
            s.copy_ad(817, 816);
        }

        s.v[3327] = if (p.p22 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[3327] != 0.0) {
            s.store_scale_ad(811, A::add(A::add(A::sub(A::sub(s.ad_value(293), s.ad_value(352)), s.ad_value(353)), s.ad_value(291)), s.ad_value(292)), s.v[365]);
        }

        if (s.v[3327] != 0.0) {
            s.store_scaled_sub(810, 355, 292, s.v[365]);
        }

        if (s.v[3327] != 0.0) {
            s.store_scaled_sub(812, 356, 291, s.v[365]);
        }

        if (s.v[3327] != 0.0) {
            s.store_add_ad_rhs(700, 700, A::scale(A::sub(A::sub(s.ad_value(305), s.ad_value(360)), s.ad_value(362)), s.v[365]));
        }

        if (s.v[3327] != 0.0) {
            s.store_add_ad_rhs(701, 701, A::scale(A::sub(s.ad_value(361), s.ad_value(305)), s.v[365]));
        }

        if (s.v[3327] != 0.0) {
            s.store_add_ad_rhs(702, 702, A::scale(s.ad_value(363), s.v[365]));
        }

        if (s.v[3327] != 0.0) {
            s.store_scale_ad(705, A::sub(A::neg(s.ad_value(350)), s.ad_value(351)), s.v[365]);
        }

        if (s.v[3327] != 0.0) {
            s.store_scale(706, 358, s.v[365]);
        }

        if (s.v[3327] != 0.0) {
            s.store_scale(707, 359, s.v[365]);
        }

        if (s.v[3327] != 0.0) {
            s.store_offset_ad(703, A::scale(A::sub(A::neg(s.ad_value(299)), s.ad_value(298)), s.v[365]), s.v[703]);
        }

        if (s.v[3327] != 0.0) {
            s.store_offset_ad(704, A::scale(A::sub(A::neg(s.ad_value(301)), s.ad_value(297)), s.v[365]), s.v[704]);
        }

        s.store_scaled_add(709, 280, 287, s.v[365]);

        s.store_scale(710, 281, s.v[365]);

        s.store_scale_ad(11, A::neg(s.ad_value(202)), s.v[365]);

        s.v[3328] = if (s.v[949] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[3328] != 0.0) {
            s.store_scale_ad(9, A::sub(A::scale(s.ad_value(199), p.p252), s.ad_value(201)), s.v[365]);
        }

        if (!(s.v[3328] != 0.0)) {
            s.store_scale_ad(9, A::sub(A::scale(s.ad_value(199), (1.0 - p.p252)), s.ad_value(200)), s.v[365]);
        }

        s.v[3329] = if (s.v[949] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[3329] != 0.0) {
            s.store_scale_ad(10, A::sub(A::scale(s.ad_value(199), (1.0 - p.p252)), s.ad_value(200)), s.v[365]);
        }

        if (!(s.v[3329] != 0.0)) {
            s.store_scale_ad(10, A::sub(A::scale(s.ad_value(199), p.p252), s.ad_value(201)), s.v[365]);
        }

        s.store_scale(7, 203, s.v[365]);

        s.store_scale(8, 204, s.v[365]);

        s.store_scale(807, 387, (4.0 * 1.3806226e-23));

        s.store_scale(711, 312, s.v[365]);

        s.store_scale(712, 315, s.v[365]);

        s.store_scalar(22, A::ddx_projection(&s.ad_value(700), Some(5), None));

        s.store_scale(22, 22, p.p87);

        s.store_scalar(23, A::ddx_projection(&s.ad_value(700), Some(7), None));

        s.store_scale(23, 23, p.p87);

        if (s.v[949] > 0.0) {
            s.copy_ad(757, 23);
        } else {
            s.copy_ad(757, 22);
        }

        s.v[713] = 0.0;

        s.v[714] = 0.0;

        s.v[3330] = if (((((p.p31 != 0.0) && (p.p30 != 0.0)) && (s.v[321] == 1.0)) && (!(s.v[78] != 0.0))) && (s.v[963] == 0.0)) { 1.0 } else { 0.0 };

        if (s.v[3330] != 0.0) {
            s.store_mul_ad_lhs(334, A::scale(s.ad_value(185), (1e-6 * s.v[635])), 162);
        }

        if (s.v[3330] != 0.0) {
            s.store_scale(344, 757, 1.0 / (s.v[365]));
        }

        if (s.v[3330] != 0.0) {
            s.store_div_ad_lhs(328, A::mul(A::mul(A::scale(s.ad_value(155), (0.1185185185185185 * 1.6021918e-19)), s.ad_value(344)), s.ad_value(344)), 324);
        }

        s.v[3331] = if ((s.v[320] > (10.0 * 2.220446049250313e-16)) && (s.v[790] > (10.0 * 2.220446049250313e-16))) { 1.0 } else { 0.0 };

        if ((s.v[3330] != 0.0) && (s.v[3331] != 0.0)) {
            s.store_div(329, 254, 253);
        }

        if ((s.v[3330] != 0.0) && (s.v[3331] != 0.0)) {
            s.store_div_ad_lhs(330, A::sub(A::div(s.ad_value(254), s.ad_value(316)), s.ad_value(329)), 790);
        }

        if ((s.v[3330] != 0.0) && (s.v[3331] != 0.0)) {
            s.store_add_ad_rhs(331, 329, A::div(A::mul(A::scale(s.ad_value(330), 0.6666666666666667), A::add(A::add(s.ad_value(317), A::mul(s.ad_value(127), s.ad_value(322))), s.ad_value(319))), A::add(s.ad_value(127), s.ad_value(322))));
        }

        if ((s.v[3330] != 0.0) && (!(s.v[3331] != 0.0))) {
            s.store_div(331, 254, 316);
        }

        if (s.v[3330] != 0.0) {
            s.store_mul_ad_lhs(713, A::mul(A::scale(s.ad_value(328), s.v[365]), s.ad_value(323)), 331);
        }

        if (s.v[3330] != 0.0) {
            s.copy_ad(714, 327);
        }

        if (s.v[3330] != 0.0) {
            s.store_ad(713, &{
                if (s.v[713] < 0.0) {
                    A::constant(0.0)
                } else {
                    s.ad_value(713)
                }
            });
        }

        if (s.v[3330] != 0.0) {
            s.store_ad(713, &{
                if ((-s.v[344]) > s.v[334]) {
                    s.ad_value(713)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[3330] != 0.0) {
            s.store_ad(714, &{
                if ((-s.v[344]) > s.v[334]) {
                    s.ad_value(714)
                } else {
                    A::constant(0.0)
                }
            });
        }

        s.store_mul(952, 807, 712);

        s.copy_ad(951, 714);

        if ((s.v[952] > 0.0) && (s.v[713] > 0.0)) {
            s.store_sqrt_ad(953, A::div(s.ad_value(713), s.ad_value(952)));
        } else {
            s.store_scalar(953, 0.0);
        }

        if (s.v[949] > 0.0) {
            s.store_mul_ad_rhs(954, 953, A::sub_from_scalar(1.0, s.ad_value(247)));
        } else {
            s.store_mul(954, 953, 247);
        }

        if (s.v[949] > 0.0) {
            s.store_mul(955, 953, 247);
        } else {
            s.store_mul_ad_rhs(955, 953, A::sub_from_scalar(1.0, s.ad_value(247)));
        }

        s.v[716] = 0.0;

        s.v[715] = 0.0;

        s.v[3332] = if (s.v[449] == 1.0) { 1.0 } else { 0.0 };

        s.v[3333] = if (s.v[76] == 0.0) { 1.0 } else { 0.0 };

        s.v[3334] = if ((p.p53 > 0.0) && (s.v[541] != 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3334] != 0.0)) {
            s.store_ad(335, &{
                if (s.v[676] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(676), p.p416)
                }
            });
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3334] != 0.0)) {
            s.store_div_from_scalar(794, s.v[569], 335);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3334] != 0.0)) {
            s.store_sub_ad(334, A::add(A::offset(A::scale(s.ad_value(676), 0.4), 1.8), A::mul(A::scale(s.ad_value(676), 0.1), s.ad_value(676))), A::scale(A::sub_from_scalar(1.0, s.ad_value(676)), p.p418));
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3334] != 0.0)) {
            s.store_div_from_scalar(795, s.v[570], 334);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3334] != 0.0)) {
            s.store_add_ad_rhs(959, 959, A::scale(A::offset(s.ad_value(387), (-s.v[764])), p.p439));
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (!(s.v[3334] != 0.0))) {
            s.store_scalar(387, (ctx.temperature() + p.p11));
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) {
            s.store_scalar(164, (s.v[630] * p.p7));
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) {
            s.store_scalar(604, p.p71);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) {
            s.store_scalar(605, s.v[460]);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) {
            s.store_mul(606, 794, 653);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) {
            s.store_offset_ad(607, A::mul(A::mul(s.ad_value(795), s.ad_value(786)), s.ad_value(652)), 1e-25);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) {
            s.store_div(608, 804, 604);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) {
            s.store_mul(609, 606, 608);
        }

        s.v[3335] = if (s.v[804] >= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3335] != 0.0)) {
            s.store_div(335, 609, 607);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (!(s.v[3335] != 0.0))) {
            s.store_div_ad_lhs(335, A::neg(s.ad_value(609)), 607);
        }

        s.v[3336] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[959]) && (s.v[959] <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3336] != 0.0)) {
            s.store_scalar(337, 1.0);
        }

        s.v[3337] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[959]) && (s.v[959] <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (!(s.v[3336] != 0.0))) && (s.v[3337] != 0.0)) {
            s.copy_ad(337, 335);
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (!(s.v[3336] != 0.0))) && (!(s.v[3337] != 0.0))) {
            s.store_ad(337, &A::pow(s.ad_value(335), A::offset(s.ad_value(959), (-1.0))));
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) {
            s.store_mul(336, 335, 337);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) {
            s.store_offset(338, 336, 1.0);
        }

        s.v[3338] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[959]) && (s.v[959] <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3338] != 0.0)) {
            s.store_div_from_scalar(339, 1.0, 338);
        }

        s.v[3339] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[959]) && (s.v[959] <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (!(s.v[3338] != 0.0))) && (s.v[3339] != 0.0)) {
            s.store_div_from_scalar_ad(339, 1.0, A::sqrt(s.ad_value(338)));
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (!(s.v[3338] != 0.0))) && (!(s.v[3339] != 0.0))) {
            s.store_ad(340, &{
                if (s.v[338] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(338), A::offset(A::div_from_scalar((-1.0), s.ad_value(959)), (-1.0)))
                }
            });
        }

    }

    pub(super) fn stamp_transient_block_92(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (!(s.v[3338] != 0.0))) && (!(s.v[3339] != 0.0))) {
            s.store_mul(339, 338, 340);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) {
            s.store_mul(610, 606, 339);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) {
            s.copy_ad(611, 605);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) {
            s.copy_ad(612, 614);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) {
            s.store_div_from_scalar(335, 1.6021918e-19, 604);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) {
            s.store_mul_ad_lhs(613, A::mul(A::mul(s.ad_value(335), s.ad_value(612)), s.ad_value(610)), 611);
        }

        s.v[3340] = if ((s.v[613] < 1e-25) && (1e-25 >= 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) {
            s.store_sub_from_scalar(781, 1e-25, 613);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) {
            s.store_square(722, 781);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) {
            s.store_scalar(723, (1e-25 * 1e-25));
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) {
            s.store_scalar(724, 1.0);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) {
            s.store_scalar(725, 1.0);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) {
            s.store_scalar(719, 0.0);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) {
            s.store_scalar(720, 0.0);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) {
            s.store_scalar(770, 0.0);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) {
            s.store_scalar(726, 0.0);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) {
            s.store_mul(724, 724, 722);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) {
            s.store_mul(725, 725, 723);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) {
            s.store_mul(724, 724, 722);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) {
            s.store_mul(725, 725, 723);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) {
            s.store_add(770, 724, 725);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) {
            s.copy_ad(726, 770);
        }

        s.v[3341] = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[3342] = if (2.0 == 1.0) { 1.0 } else { 0.0 };

        if (((((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) && (s.v[3341] != 0.0)) && (s.v[3342] != 0.0)) {
            s.store_scalar(720, 1.0);
        }

        s.v[3343] = if (2.0 == 2.0) { 1.0 } else { 0.0 };

        if ((((((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) && (s.v[3341] != 0.0)) && (!(s.v[3342] != 0.0))) && (s.v[3343] != 0.0)) {
            s.store_scalar(720, 2.0);
        }

        s.v[3344] = if (2.0 == 4.0) { 1.0 } else { 0.0 };

        if (((((((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) && (s.v[3341] != 0.0)) && (!(s.v[3342] != 0.0))) && (!(s.v[3343] != 0.0))) && (s.v[3344] != 0.0)) {
            s.store_scalar(720, 3.0);
        }

        s.v[3345] = if (2.0 == 8.0) { 1.0 } else { 0.0 };

        if ((((((((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) && (s.v[3341] != 0.0)) && (!(s.v[3342] != 0.0))) && (!(s.v[3343] != 0.0))) && (!(s.v[3344] != 0.0))) && (s.v[3345] != 0.0)) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) && (s.v[3341] != 0.0)) {
            s.store_scalar(719, 0.0);
        }

        let mut assign103230_loop_guard: usize = 0;
        while {
            let assign103230_cond_e155130: f64 = if (((((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) && (s.v[3341] != 0.0)) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign103230_cond_e155130 != 0.0
        } {
            assign103230_loop_guard += 1;
            assert!(assign103230_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) && (s.v[3341] != 0.0)) {
                s.store_sqrt(726, 726);
            }
            if ((((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) && (s.v[3341] != 0.0)) {
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) && (!(s.v[3341] != 0.0))) {
            s.store_ad(726, &{
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) {
            s.store_div_from_scalar(726, 1.0, 726);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) {
            s.store_mul_ad_lhs(780, A::scale(s.ad_value(781), 1e-25), 726);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) {
            s.store_div_ad_lhs(334, A::mul(A::scale(s.ad_value(725), 1e-25), s.ad_value(726)), 770);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) {
            s.store_sub_from_scalar(613, 1e-25, 780);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3340] != 0.0)) {
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (!(s.v[3340] != 0.0))) {
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (!(s.v[3340] != 0.0))) {
            s.store_scalar(334, 1.0);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) {
            s.store_div_from_scalar(5, 1.0, 613);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) {
            s.store_div(5, 5, 164);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) {
            s.store_add(5, 5, 648);
        }

        s.v[3346] = if ((s.v[5] > p.p444) && (p.p30 != 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3346] != 0.0)) {
            s.store_div_from_scalar(696, s.v[365], 5);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (!(s.v[3346] != 0.0))) {
            s.store_scalar(696, 0.0);
        }

        s.v[3347] = if (s.v[5] < p.p444) { 1.0 } else { 0.0 };

        if (((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) && (s.v[3347] != 0.0)) {
            s.store_scalar(5, p.p444);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3333] != 0.0))) {
            s.store_scale(716, 5, 1.0 / (s.v[365]));
        }

        s.v[3352] = if (s.v[75] == 0.0) { 1.0 } else { 0.0 };

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.copy_ad(3348, 729);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.copy_ad(3349, 728);
        }

        s.v[3353] = if ((p.p53 > 0.0) && (s.v[541] != 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3353] != 0.0)) {
            s.store_ad(335, &{
                if (s.v[676] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(676), p.p415)
                }
            });
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3353] != 0.0)) {
            s.store_div_from_scalar(787, s.v[567], 335);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3353] != 0.0)) {
            s.store_sub_ad(334, A::add(A::offset(A::scale(s.ad_value(676), 0.4), 1.8), A::mul(A::scale(s.ad_value(676), 0.1), s.ad_value(676))), A::scale(A::sub_from_scalar(1.0, s.ad_value(676)), p.p417));
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3353] != 0.0)) {
            s.store_div_from_scalar(788, s.v[568], 334);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3353] != 0.0)) {
            s.store_add_ad_rhs(956, 956, A::scale(A::offset(s.ad_value(387), (-s.v[764])), p.p438));
        }

        s.v[3355] = if (s.v[956] < 0.1) { 1.0 } else { 0.0 };

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3353] != 0.0)) && (s.v[3355] != 0.0)) {
            s.store_scalar(956, 0.1);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (!(s.v[3353] != 0.0))) {
            s.store_scalar(387, (ctx.temperature() + p.p11));
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_scalar(164, (s.v[630] * p.p7));
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_scalar(785, (p.p67 + p.p68));
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_offset(789, 451, 1e-12);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_scalar(408, s.v[459]);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_offset_ad(335, A::mul(s.ad_value(3349), A::sub_from_scalar(p.p410, A::scale(s.ad_value(3349), p.p411))), 1.0);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_sqrt_ad(782, A::offset(A::square(s.ad_value(335)), ((4.0 * 0.1) * 0.1)));
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_scale_ad(336, A::offset(A::div(s.ad_value(335), s.ad_value(782)), 1.0), 0.5);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_scaled_add(654, 335, 782, 0.5);
        }

        s.v[3356] = if (s.v[654] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3356] != 0.0)) {
            s.store_scalar(654, 0.0);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3356] != 0.0)) {
            s.store_scalar(336, 0.0);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_mul_ad_lhs(593, A::mul(s.ad_value(787), s.ad_value(653)), 654);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_offset_ad(3351, A::mul(A::mul(s.ad_value(788), s.ad_value(786)), s.ad_value(652)), 1e-25);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.copy_ad(594, 453);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_scalar(595, p.p421);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_scale(335, 593, 10000.0);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_scale(336, 3351, 100.0);
        }

        s.v[3359] = if (s.v[799] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3359] != 0.0)) {
            s.store_scale_ad(781, A::scale(A::neg(s.ad_value(799)), 0.5), (2.0 * 1.0 / (p.p262)));
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3359] != 0.0)) {
            s.store_offset_ad(782, A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::scale(s.ad_value(781), (1.0 / 5040.0)), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0))), 1.0);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3359] != 0.0)) {
            s.store_offset_ad(783, A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::scale(s.ad_value(781), (1.0 / 840.0)), (1.0 / 144.0))), (1.0 / 30.0))), (1.0 / 8.0))), (1.0 / 3.0))), (1.0 / 2.0));
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3359] != 0.0)) {
            s.store_div_from_scalar(108, p.p262, 782);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3359] != 0.0)) {
            s.store_div_ad(336, A::scale(s.ad_value(783), (-2.0)), A::square(s.ad_value(782)));
        }

        s.v[3360] = if (s.v[108] < 1e-12) { 1.0 } else { 0.0 };

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3359] != 0.0)) && (s.v[3360] != 0.0)) {
            s.store_scalar(108, 1e-12);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3359] != 0.0)) {
            s.store_sub_ad_rhs(598, 799, A::scale(s.ad_value(108), 2.0));
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (!(s.v[3359] != 0.0))) {
            s.store_scale_ad(781, A::scale(s.ad_value(799), 0.5), (2.0 * 1.0 / (p.p262)));
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (!(s.v[3359] != 0.0))) {
            s.store_offset_ad(782, A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::scale(s.ad_value(781), (1.0 / 5040.0)), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0))), 1.0);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (!(s.v[3359] != 0.0))) {
            s.store_offset_ad(783, A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::scale(s.ad_value(781), (1.0 / 840.0)), (1.0 / 144.0))), (1.0 / 30.0))), (1.0 / 8.0))), (1.0 / 3.0))), (1.0 / 2.0));
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (!(s.v[3359] != 0.0))) {
            s.store_div_from_scalar(108, p.p262, 782);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (!(s.v[3359] != 0.0))) {
            s.store_div_ad(336, A::scale(s.ad_value(783), (-2.0)), A::square(s.ad_value(782)));
        }

        s.v[3361] = if (s.v[108] < 1e-12) { 1.0 } else { 0.0 };

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (!(s.v[3359] != 0.0))) && (s.v[3361] != 0.0)) {
            s.store_scalar(108, 1e-12);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (!(s.v[3359] != 0.0))) {
            s.store_add_ad_rhs(598, 799, A::scale(s.ad_value(108), 2.0));
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_div(591, 598, 785);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_mul(592, 593, 591);
        }

        s.v[3362] = if (s.v[799] >= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3362] != 0.0)) {
            s.store_div(335, 592, 3351);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (!(s.v[3362] != 0.0))) {
            s.store_div_ad_lhs(335, A::neg(s.ad_value(592)), 3351);
        }

        s.v[3363] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3363] != 0.0)) {
            s.store_scalar(337, 1.0);
        }

        s.v[3364] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (!(s.v[3363] != 0.0))) && (s.v[3364] != 0.0)) {
            s.copy_ad(337, 335);
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (!(s.v[3363] != 0.0))) && (!(s.v[3364] != 0.0))) {
            s.store_ad(337, &A::pow(s.ad_value(335), A::offset(s.ad_value(956), (-1.0))));
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_mul(336, 335, 337);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_offset(338, 336, 1.0);
        }

        s.v[3365] = if (((1.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (1.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3365] != 0.0)) {
            s.store_div_from_scalar(339, 1.0, 338);
        }

        s.v[3366] = if (((2.0 - (10.0 * 2.220446049250313e-16)) <= s.v[956]) && (s.v[956] <= (2.0 + (10.0 * 2.220446049250313e-16)))) { 1.0 } else { 0.0 };

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (!(s.v[3365] != 0.0))) && (s.v[3366] != 0.0)) {
            s.store_div_from_scalar_ad(339, 1.0, A::sqrt(s.ad_value(338)));
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (!(s.v[3365] != 0.0))) && (!(s.v[3366] != 0.0))) {
            s.store_ad(340, &{
                if (s.v[338] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(338), A::offset(A::div_from_scalar((-1.0), s.ad_value(956)), (-1.0)))
                }
            });
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (!(s.v[3365] != 0.0))) && (!(s.v[3366] != 0.0))) {
            s.store_mul(339, 338, 340);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_mul(3350, 593, 339);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_offset(338, 335, 1.0);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_div_from_scalar(339, 1.0, 338);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_offset_ad(338, A::div(A::mul(A::mul(s.ad_value(595), A::sub_from_scalar(1.0, s.ad_value(339))), s.ad_value(598)), A::offset(s.ad_value(785), (-p.p423))), 1.0);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_offset(781, 338, (-0.001));
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_scalar(782, 0.0);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_ad(782, &{
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_sqrt_ad(782, A::add(A::square(s.ad_value(781)), s.ad_value(782)));
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_scale_ad(334, A::offset(A::div(s.ad_value(781), s.ad_value(782)), 1.0), 0.5);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_scaled_add(339, 781, 782, 0.5);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_mul(717, 408, 339);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_scale_ad(718, A::scale(s.ad_value(698), 6.241449993689894e18), p.p430);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_sub_ad(781, A::sub(s.ad_value(717), s.ad_value(718)), A::scale(s.ad_value(717), 0.001));
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_mul_ad(782, A::scale(s.ad_value(717), 4.0), A::scale(s.ad_value(717), 0.001));
        }

    }

    pub(super) fn stamp_transient_block_93(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_ad(782, &{
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_sqrt_ad(782, A::add(A::square(s.ad_value(781)), s.ad_value(782)));
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_scale_ad(334, A::offset(A::div(s.ad_value(781), s.ad_value(782)), 1.0), 0.5);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_sub_ad_rhs(718, 717, A::scale(A::add(s.ad_value(781), s.ad_value(782)), 0.5));
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_sub(597, 717, 718);
        }

        s.v[3367] = if ((p.p441 > 0.0) && (p.p440 > 1.0)) { 1.0 } else { 0.0 };

        s.v[3368] = if ((s.v[597] > ((s.v[408] * p.p440) - (s.v[408] * p.p441))) && ((s.v[408] * p.p441) >= 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) {
            s.store_add_ad(781, A::sub(s.ad_value(597), A::scale(s.ad_value(408), p.p440)), A::scale(s.ad_value(408), p.p441));
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) {
            s.store_square(722, 781);
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) {
            s.store_mul_ad(723, A::scale(s.ad_value(408), p.p441), A::scale(s.ad_value(408), p.p441));
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) {
            s.store_scalar(724, 1.0);
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) {
            s.store_scalar(725, 1.0);
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) {
            s.store_scalar(719, 0.0);
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) {
            s.store_scalar(720, 0.0);
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) {
            s.store_scalar(770, 0.0);
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) {
            s.store_scalar(726, 0.0);
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) {
            s.store_scalar(719, 0.0);
        }

        let mut assign104390_loop_guard: usize = 0;
        while {
            let assign104390_cond_e156585: f64 = if (((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) && (s.v[719] < p.p442)) { 1.0 } else { 0.0 };
            assign104390_cond_e156585 != 0.0
        } {
            assign104390_loop_guard += 1;
            assert!(assign104390_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) {
                s.store_mul(724, 724, 722);
            }
            if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) {
                s.store_mul(725, 725, 723);
            }
            if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) {
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) {
            s.store_add(770, 724, 725);
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) {
            s.copy_ad(726, 770);
        }

        s.v[3369] = if ((((p.p442 == 1.0) || (p.p442 == 2.0)) || (p.p442 == 4.0)) || (p.p442 == 8.0)) { 1.0 } else { 0.0 };

        s.v[3370] = if (p.p442 == 1.0) { 1.0 } else { 0.0 };

        if ((((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) && (s.v[3369] != 0.0)) && (s.v[3370] != 0.0)) {
            s.store_scalar(720, 1.0);
        }

        s.v[3371] = if (p.p442 == 2.0) { 1.0 } else { 0.0 };

        if (((((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) && (s.v[3369] != 0.0)) && (!(s.v[3370] != 0.0))) && (s.v[3371] != 0.0)) {
            s.store_scalar(720, 2.0);
        }

        s.v[3372] = if (p.p442 == 4.0) { 1.0 } else { 0.0 };

        if ((((((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) && (s.v[3369] != 0.0)) && (!(s.v[3370] != 0.0))) && (!(s.v[3371] != 0.0))) && (s.v[3372] != 0.0)) {
            s.store_scalar(720, 3.0);
        }

        s.v[3373] = if (p.p442 == 8.0) { 1.0 } else { 0.0 };

        if (((((((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) && (s.v[3369] != 0.0)) && (!(s.v[3370] != 0.0))) && (!(s.v[3371] != 0.0))) && (!(s.v[3372] != 0.0))) && (s.v[3373] != 0.0)) {
            s.store_scalar(720, 4.0);
        }

        if (((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) && (s.v[3369] != 0.0)) {
            s.store_scalar(719, 0.0);
        }

        let mut assign104520_loop_guard: usize = 0;
        while {
            let assign104520_cond_e156780: f64 = if ((((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) && (s.v[3369] != 0.0)) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign104520_cond_e156780 != 0.0
        } {
            assign104520_loop_guard += 1;
            assert!(assign104520_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) && (s.v[3369] != 0.0)) {
                s.store_sqrt(726, 726);
            }
            if (((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) && (s.v[3369] != 0.0)) {
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) && (!(s.v[3369] != 0.0))) {
            s.store_ad(726, &{
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * p.p442)))
                }
            });
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) {
            s.store_div_from_scalar(726, 1.0, 726);
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) {
            s.store_mul_ad_lhs(780, A::mul(s.ad_value(781), A::scale(s.ad_value(408), p.p441)), 726);
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) {
            s.store_div_ad_lhs(334, A::mul(A::mul(A::scale(s.ad_value(408), p.p441), s.ad_value(725)), s.ad_value(726)), 770);
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) {
            s.store_add_ad_lhs(336, A::sub(A::scale(s.ad_value(408), p.p440), A::scale(s.ad_value(408), p.p441)), 780);
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (s.v[3368] != 0.0)) {
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (!(s.v[3368] != 0.0))) {
            s.copy_ad(336, 597);
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) && (!(s.v[3368] != 0.0))) {
            s.store_scalar(334, 1.0);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3367] != 0.0)) {
            s.copy_ad(597, 336);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_neg(334, 697);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_sqrt_ad(782, A::offset(A::square(s.ad_value(334)), ((4.0 * 0.01) * 0.01)));
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_scale_ad(343, A::offset(A::div(s.ad_value(334), s.ad_value(782)), 1.0), 0.5);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_scaled_add(334, 334, 782, 0.5);
        }

        s.v[3374] = if (s.v[334] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3374] != 0.0)) {
            s.store_scalar(334, 0.0);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3374] != 0.0)) {
            s.store_scalar(343, 0.0);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_offset(334, 334, (10.0 * 2.220446049250313e-16));
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_sqrt_ad(599, A::mul(s.ad_value(650), s.ad_value(334)));
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_offset_ad(336, A::sub(s.ad_value(3348), s.ad_value(3349)), p.p137);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_sqrt_ad(782, A::offset(A::square(s.ad_value(336)), ((4.0 * 0.01) * 0.01)));
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_scale_ad(343, A::offset(A::div(s.ad_value(336), s.ad_value(782)), 1.0), 0.5);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_scaled_add(336, 336, 782, 0.5);
        }

        s.v[3375] = if (s.v[336] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3375] != 0.0)) {
            s.store_scalar(336, 0.0);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3375] != 0.0)) {
            s.store_scalar(343, 0.0);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_offset(336, 336, (10.0 * 2.220446049250313e-16));
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_sqrt_ad(600, A::mul(s.ad_value(651), s.ad_value(336)));
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_sub_ad(781, A::sub(s.ad_value(789), s.ad_value(600)), A::scale(s.ad_value(789), 0.01));
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_mul_ad(782, A::scale(s.ad_value(789), 4.0), A::scale(s.ad_value(789), 0.01));
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_ad(782, &{
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_sqrt_ad(782, A::add(A::square(s.ad_value(781)), s.ad_value(782)));
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_scale_ad(334, A::offset(A::div(s.ad_value(781), s.ad_value(782)), 1.0), 0.5);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_sub_ad_rhs(602, 789, A::scale(A::add(s.ad_value(781), s.ad_value(782)), 0.5));
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_scalar(601, (p.p419 + 1e-25));
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_mul_ad_rhs(596, 649, A::sub_from_scalar(1.0, A::mul(s.ad_value(594), A::add(A::div(s.ad_value(599), s.ad_value(601)), A::div(s.ad_value(602), s.ad_value(789))))));
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_sqrt_ad(782, A::add(A::square(s.ad_value(596)), A::mul(A::scale(A::scale(A::mul(A::sub_from_scalar(1.0, s.ad_value(453)), s.ad_value(649)), 0.01), 4.0), A::scale(A::mul(A::sub_from_scalar(1.0, s.ad_value(453)), s.ad_value(649)), 0.01))));
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_scale_ad(343, A::offset(A::div(s.ad_value(596), s.ad_value(782)), 1.0), 0.5);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_scaled_add(596, 596, 782, 0.5);
        }

        s.v[3376] = if (s.v[596] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3376] != 0.0)) {
            s.store_scalar(596, 0.0);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3376] != 0.0)) {
            s.store_scalar(343, 0.0);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_div_from_scalar_ad(335, 1.6021918e-19, A::offset(s.ad_value(785), p.p422));
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_mul_ad_lhs(739, A::mul(A::mul(s.ad_value(335), s.ad_value(596)), s.ad_value(3350)), 597);
        }

        s.v[3377] = if ((s.v[739] < 1e-25) && (1e-25 >= 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) {
            s.store_sub_from_scalar(781, 1e-25, 739);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) {
            s.store_square(722, 781);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) {
            s.store_scalar(723, (1e-25 * 1e-25));
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) {
            s.store_scalar(724, 1.0);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) {
            s.store_scalar(725, 1.0);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) {
            s.store_scalar(719, 0.0);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) {
            s.store_scalar(720, 0.0);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) {
            s.store_scalar(770, 0.0);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) {
            s.store_scalar(726, 0.0);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) {
            s.store_mul(724, 724, 722);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) {
            s.store_mul(725, 725, 723);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) {
            s.store_mul(724, 724, 722);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) {
            s.store_mul(725, 725, 723);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) {
            s.store_add(770, 724, 725);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) {
            s.copy_ad(726, 770);
        }

        s.v[3378] = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[3379] = if (2.0 == 1.0) { 1.0 } else { 0.0 };

        if (((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) && (s.v[3378] != 0.0)) && (s.v[3379] != 0.0)) {
            s.store_scalar(720, 1.0);
        }

        s.v[3380] = if (2.0 == 2.0) { 1.0 } else { 0.0 };

        if ((((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) && (s.v[3378] != 0.0)) && (!(s.v[3379] != 0.0))) && (s.v[3380] != 0.0)) {
            s.store_scalar(720, 2.0);
        }

        s.v[3381] = if (2.0 == 4.0) { 1.0 } else { 0.0 };

        if (((((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) && (s.v[3378] != 0.0)) && (!(s.v[3379] != 0.0))) && (!(s.v[3380] != 0.0))) && (s.v[3381] != 0.0)) {
            s.store_scalar(720, 3.0);
        }

        s.v[3382] = if (2.0 == 8.0) { 1.0 } else { 0.0 };

        if ((((((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) && (s.v[3378] != 0.0)) && (!(s.v[3379] != 0.0))) && (!(s.v[3380] != 0.0))) && (!(s.v[3381] != 0.0))) && (s.v[3382] != 0.0)) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) && (s.v[3378] != 0.0)) {
            s.store_scalar(719, 0.0);
        }

        let mut assign105220_loop_guard: usize = 0;
        while {
            let assign105220_cond_e157613: f64 = if (((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) && (s.v[3378] != 0.0)) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign105220_cond_e157613 != 0.0
        } {
            assign105220_loop_guard += 1;
            assert!(assign105220_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) && (s.v[3378] != 0.0)) {
                s.store_sqrt(726, 726);
            }
            if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) && (s.v[3378] != 0.0)) {
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) && (!(s.v[3378] != 0.0))) {
            s.store_ad(726, &{
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) {
            s.store_div_from_scalar(726, 1.0, 726);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) {
            s.store_mul_ad_lhs(780, A::scale(s.ad_value(781), 1e-25), 726);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) {
            s.store_div_ad_lhs(334, A::mul(A::scale(s.ad_value(725), 1e-25), s.ad_value(726)), 770);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) {
            s.store_sub_from_scalar(739, 1e-25, 780);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3377] != 0.0)) {
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (!(s.v[3377] != 0.0))) {
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (!(s.v[3377] != 0.0))) {
            s.store_scalar(334, 1.0);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_div_from_scalar(4, 1.0, 739);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_div(4, 4, 164);
        }

        s.v[3383] = if ((s.v[4] > (1000000.0 - 1000.0)) && (1000.0 >= 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) {
            s.store_offset(781, 4, (((-1000000.0)) + (1000.0)));
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) {
            s.store_square(722, 781);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) {
            s.store_scalar(723, (1000.0 * 1000.0));
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) {
            s.store_scalar(724, 1.0);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) {
            s.store_scalar(725, 1.0);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) {
            s.store_scalar(719, 0.0);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) {
            s.store_scalar(720, 0.0);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) {
            s.store_scalar(770, 0.0);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) {
            s.store_scalar(726, 0.0);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) {
            s.store_mul(724, 724, 722);
        }

    }

    pub(super) fn stamp_transient_block_94(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) {
            s.store_mul(725, 725, 723);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) {
            s.store_mul(724, 724, 722);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) {
            s.store_mul(725, 725, 723);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) {
            s.store_add(770, 724, 725);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) {
            s.copy_ad(726, 770);
        }

        s.v[3384] = if ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0)) { 1.0 } else { 0.0 };

        s.v[3385] = if (2.0 == 1.0) { 1.0 } else { 0.0 };

        if (((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) && (s.v[3384] != 0.0)) && (s.v[3385] != 0.0)) {
            s.store_scalar(720, 1.0);
        }

        s.v[3386] = if (2.0 == 2.0) { 1.0 } else { 0.0 };

        if ((((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) && (s.v[3384] != 0.0)) && (!(s.v[3385] != 0.0))) && (s.v[3386] != 0.0)) {
            s.store_scalar(720, 2.0);
        }

        s.v[3387] = if (2.0 == 4.0) { 1.0 } else { 0.0 };

        if (((((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) && (s.v[3384] != 0.0)) && (!(s.v[3385] != 0.0))) && (!(s.v[3386] != 0.0))) && (s.v[3387] != 0.0)) {
            s.store_scalar(720, 3.0);
        }

        s.v[3388] = if (2.0 == 8.0) { 1.0 } else { 0.0 };

        if ((((((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) && (s.v[3384] != 0.0)) && (!(s.v[3385] != 0.0))) && (!(s.v[3386] != 0.0))) && (!(s.v[3387] != 0.0))) && (s.v[3388] != 0.0)) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) && (s.v[3384] != 0.0)) {
            s.store_scalar(719, 0.0);
        }

        let mut assign105590_loop_guard: usize = 0;
        while {
            let assign105590_cond_e158042: f64 = if (((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) && (s.v[3384] != 0.0)) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign105590_cond_e158042 != 0.0
        } {
            assign105590_loop_guard += 1;
            assert!(assign105590_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) && (s.v[3384] != 0.0)) {
                s.store_sqrt(726, 726);
            }
            if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) && (s.v[3384] != 0.0)) {
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) && (!(s.v[3384] != 0.0))) {
            s.store_ad(726, &{
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) {
            s.store_div_from_scalar(726, 1.0, 726);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) {
            s.store_mul_ad_lhs(780, A::scale(s.ad_value(781), 1000.0), 726);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) {
            s.store_div_ad_lhs(334, A::mul(A::scale(s.ad_value(725), 1000.0), s.ad_value(726)), 770);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) {
            s.store_offset(4, 780, (1000000.0 - 1000.0));
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3383] != 0.0)) {
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (!(s.v[3383] != 0.0))) {
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (!(s.v[3383] != 0.0))) {
            s.store_scalar(334, 1.0);
        }

        s.v[3389] = if ((p.p54 == 1.0) && ((s.v[459] * (s.v[544] + s.v[459])) > 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3389] != 0.0)) {
            s.store_sub_from_scalar(385, p.p334, 384);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3389] != 0.0)) {
            s.store_div_ad_lhs(4, A::scale(s.ad_value(4), s.v[165]), 385);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_add(4, 4, 644);
        }

        s.v[3390] = if ((s.v[4] > p.p444) && (p.p30 != 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3390] != 0.0)) {
            s.store_div_from_scalar(695, s.v[365], 4);
        }

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (!(s.v[3390] != 0.0))) {
            s.store_scalar(695, 0.0);
        }

        s.v[3391] = if (s.v[4] < p.p444) { 1.0 } else { 0.0 };

        if (((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) && (s.v[3391] != 0.0)) {
            s.store_scalar(4, p.p444);
        }

        if ((s.v[3332] != 0.0) && (!(s.v[3352] != 0.0))) {
            s.store_scale(715, 4, 1.0 / (s.v[365]));
        }

        s.v[3392] = if (s.v[4] < p.p444) { 1.0 } else { 0.0 };

        if ((!(s.v[3332] != 0.0)) && (s.v[3392] != 0.0)) {
            s.store_scalar(4, p.p444);
        }

        s.v[3393] = if (s.v[5] < p.p444) { 1.0 } else { 0.0 };

        if ((!(s.v[3332] != 0.0)) && (s.v[3393] != 0.0)) {
            s.store_scalar(5, p.p444);
        }

        s.v[3394] = if (s.v[370] > 0.0) { 1.0 } else { 0.0 };

        if ((!(s.v[3332] != 0.0)) && (s.v[3394] != 0.0)) {
            s.store_scale(715, 4, 1.0 / (s.v[365]));
        }

        if ((!(s.v[3332] != 0.0)) && (s.v[3394] != 0.0)) {
            s.store_scale(716, 5, 1.0 / (s.v[365]));
        }

        if ((!(s.v[3332] != 0.0)) && (!(s.v[3394] != 0.0))) {
            s.store_scale(715, 5, 1.0 / (s.v[365]));
        }

        if ((!(s.v[3332] != 0.0)) && (!(s.v[3394] != 0.0))) {
            s.store_scale(716, 4, 1.0 / (s.v[365]));
        }

        s.copy_ad(4, 715);

        s.copy_ad(5, 716);

        s.copy_ad(201, 9);

        s.copy_ad(200, 10);

        s.copy_ad(202, 11);

        s.v[3395] = if (s.v[949] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[3395] != 0.0) {
            s.copy_ad(134, 0);
        }

        if (s.v[3395] != 0.0) {
            s.copy_ad(19, 701);
        }

        if (s.v[3395] != 0.0) {
            s.copy_ad(18, 700);
        }

        if (s.v[3395] != 0.0) {
            s.copy_ad(741, 702);
        }

        if (s.v[3395] != 0.0) {
            s.store_neg_ad(20, A::add(A::add(s.ad_value(700), s.ad_value(701)), s.ad_value(702)));
        }

        if (s.v[3395] != 0.0) {
            s.copy_ad(280, 709);
        }

        if (s.v[3395] != 0.0) {
            s.store_scalar(736, 0.0);
        }

        if (s.v[3395] != 0.0) {
            s.copy_ad(281, 710);
        }

        if (s.v[3395] != 0.0) {
            s.store_scalar(737, 0.0);
        }

        if (s.v[3395] != 0.0) {
            s.copy_ad(400, 699);
        }

        if (s.v[3395] != 0.0) {
            s.store_scalar(738, 0.0);
        }

        if (s.v[3395] != 0.0) {
            s.copy_ad(431, 430);
        }

        if (s.v[3395] != 0.0) {
            s.store_scalar(432, 0.0);
        }

        if (s.v[3395] != 0.0) {
            s.copy_ad(424, 422);
        }

        if (s.v[3395] != 0.0) {
            s.store_scalar(425, 0.0);
        }

        if (s.v[3395] != 0.0) {
            s.copy_ad(203, 7);
        }

        if (s.v[3395] != 0.0) {
            s.copy_ad(204, 8);
        }

        if ((s.v[3395] != 0.0) && (s.v[81] != 0.0)) {
            s.copy_ad(247, 708);
        }

        if (!(s.v[3395] != 0.0)) {
            s.store_neg(134, 0);
        }

        if (!(s.v[3395] != 0.0)) {
            s.copy_ad(19, 702);
        }

        if (!(s.v[3395] != 0.0)) {
            s.copy_ad(18, 700);
        }

        if (!(s.v[3395] != 0.0)) {
            s.copy_ad(741, 701);
        }

        if (!(s.v[3395] != 0.0)) {
            s.store_neg_ad(20, A::add(A::add(s.ad_value(700), s.ad_value(701)), s.ad_value(702)));
        }

        if (!(s.v[3395] != 0.0)) {
            s.store_scalar(280, 0.0);
        }

        if (!(s.v[3395] != 0.0)) {
            s.copy_ad(736, 709);
        }

        if (!(s.v[3395] != 0.0)) {
            s.store_scalar(281, 0.0);
        }

        if (!(s.v[3395] != 0.0)) {
            s.copy_ad(737, 710);
        }

        if (!(s.v[3395] != 0.0)) {
            s.store_scalar(400, 0.0);
        }

        if (!(s.v[3395] != 0.0)) {
            s.copy_ad(738, 699);
        }

        if (!(s.v[3395] != 0.0)) {
            s.store_scalar(431, 0.0);
        }

        if (!(s.v[3395] != 0.0)) {
            s.copy_ad(432, 430);
        }

        if (!(s.v[3395] != 0.0)) {
            s.store_scalar(424, 0.0);
        }

        if (!(s.v[3395] != 0.0)) {
            s.copy_ad(425, 422);
        }

        if (!(s.v[3395] != 0.0)) {
            s.copy_ad(203, 8);
        }

        if (!(s.v[3395] != 0.0)) {
            s.copy_ad(204, 7);
        }

        if ((!(s.v[3395] != 0.0)) && (s.v[81] != 0.0)) {
            s.store_sub_from_scalar(247, 1.0, 708);
        }

        s.store_add(18, 18, 811);

        s.store_add(19, 19, 810);

        s.store_add(741, 741, 812);

        s.store_neg_ad(20, A::add(A::add(s.ad_value(18), s.ad_value(19)), s.ad_value(741)));

        s.copy_ad(299, 703);

        s.copy_ad(301, 704);

        s.copy_ad(742, 706);

        s.copy_ad(743, 705);

        s.store_neg_ad(744, A::add(A::add(s.ad_value(705), s.ad_value(706)), s.ad_value(707)));

        s.v[3396] = if (p.p53 > 0.0) { 1.0 } else { 0.0 };

        s.v[3397] = if (s.v[766] > 0.0001) { 1.0 } else { 0.0 };

        if ((s.v[3396] != 0.0) && (s.v[3397] != 0.0)) {
            s.store_div_from_scalar(740, 1.0, 766);
        }

        if ((s.v[3396] != 0.0) && (!(s.v[3397] != 0.0))) {
            s.store_scalar(740, (1.0 / 0.0001));
        }

        s.v[3398] = if ((s.v[729] * (s.v[733] - s.v[729])) >= 0.0) { 1.0 } else { 0.0 };

        s.v[3399] = if (s.v[529] == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[3396] != 0.0) && (s.v[3398] != 0.0)) && (s.v[3399] != 0.0)) {
            s.copy_ad(745, 733);
        }

        if (((s.v[3396] != 0.0) && (s.v[3398] != 0.0)) && (!(s.v[3399] != 0.0))) {
            s.store_add_ad_rhs(745, 729, A::mul(s.ad_value(683), A::sub(s.ad_value(733), s.ad_value(729))));
        }

        if ((s.v[3396] != 0.0) && (!(s.v[3398] != 0.0))) {
            s.copy_ad(745, 729);
        }

        if (s.v[3396] != 0.0) {
            s.store_mul(746, 134, 745);
        }

        s.v[3400] = if (p.p53 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[3396] != 0.0) && (s.v[3400] != 0.0)) {
            s.store_scale(335, 740, p.p433);
        }

        if ((s.v[3396] != 0.0) && (s.v[3400] != 0.0)) {
            s.store_sub_ad(781, A::sub(s.ad_value(335), s.ad_value(746)), A::scale(s.ad_value(740), p.p337));
        }

        if ((s.v[3396] != 0.0) && (s.v[3400] != 0.0)) {
            s.store_mul_ad(782, A::scale(s.ad_value(335), 4.0), A::scale(s.ad_value(740), p.p337));
        }

        if ((s.v[3396] != 0.0) && (s.v[3400] != 0.0)) {
            s.store_ad(782, &{
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.v[3396] != 0.0) && (s.v[3400] != 0.0)) {
            s.store_sqrt_ad(782, A::add(A::square(s.ad_value(781)), s.ad_value(782)));
        }

        if ((s.v[3396] != 0.0) && (s.v[3400] != 0.0)) {
            s.store_scale_ad(334, A::offset(A::div(s.ad_value(781), s.ad_value(782)), 1.0), 0.5);
        }

        if ((s.v[3396] != 0.0) && (s.v[3400] != 0.0)) {
            s.store_sub_ad_rhs(336, 335, A::scale(A::add(s.ad_value(781), s.ad_value(782)), 0.5));
        }

        if ((s.v[3396] != 0.0) && (s.v[3400] != 0.0)) {
            s.copy_ad(746, 336);
        }

        if (!(s.v[3396] != 0.0)) {
            s.store_scalar(740, 0.0);
        }

        if (!(s.v[3396] != 0.0)) {
            s.store_scalar(746, 0.0);
        }

        s.v[3401] = if (s.v[306] < 1e-15) { 1.0 } else { 0.0 };

        if ((s.v[81] != 0.0) && (s.v[3401] != 0.0)) {
            s.store_scalar(306, 1e-15);
        }

        s.v[3402] = if (s.v[307] < 1e-15) { 1.0 } else { 0.0 };

        if ((s.v[81] != 0.0) && (s.v[3402] != 0.0)) {
            s.store_scalar(307, 1e-15);
        }

        if (s.v[81] != 0.0) {
            s.store_div_ad_lhs(749, A::sub(s.ad_value(747), s.ad_value(132)), 306);
        }

        if (s.v[81] != 0.0) {
            s.store_div_ad_lhs(750, A::sub(s.ad_value(748), s.ad_value(754)), 307);
        }

        if (s.v[81] != 0.0) {
            s.store_mul(751, 747, 247);
        }

        if (s.v[81] != 0.0) {
            s.store_sub_ad_lhs(753, A::neg(s.ad_value(747)), 748);
        }

        if (s.v[81] != 0.0) {
            s.store_mul_ad_rhs(752, 747, A::sub_from_scalar(1.0, s.ad_value(247)));
        }

        if (!(s.v[81] != 0.0)) {
            s.store_scalar(749, 0.0);
        }

        if (!(s.v[81] != 0.0)) {
            s.store_scalar(750, 0.0);
        }

        if (!(s.v[81] != 0.0)) {
            s.store_scalar(751, 0.0);
        }

        if (!(s.v[81] != 0.0)) {
            s.store_scalar(753, 0.0);
        }

        if (!(s.v[81] != 0.0)) {
            s.store_scalar(752, 0.0);
        }

        s.store_mul_ad_lhs(0, A::scale(s.ad_value(949), p.p87), 134);

        s.store_scalar(22, A::ddx_projection(&s.ad_value(18), Some(5), None));

        s.store_scale(22, 22, p.p87);

        s.store_scalar(23, A::ddx_projection(&s.ad_value(18), Some(7), None));

        s.store_scale(23, 23, p.p87);

        s.v[3403] = if (s.v[949] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[3403] != 0.0) {
            s.copy_ad(757, 23);
        }

        if (!(s.v[3403] != 0.0)) {
            s.copy_ad(757, 22);
        }

        s.v[3405] = if (p.p48 > 0.0) { 1.0 } else { 0.0 };

        s.v[3406] = if (p.p24 == 1.0) { 1.0 } else { 0.0 };

        s.v[3407] = if (((s.v[74] != 1.0) && (s.v[74] != 3.0)) && (s.v[449] != 1.0)) { 1.0 } else { 0.0 };

        if (s.v[3407] != 0.0) {
            s.store_scalar(695, 0.0);
        }

        if (s.v[3407] != 0.0) {
            s.store_scalar(696, 0.0);
        }

        s.v[3408] = if ((p.p51 == 1.0) && (p.p132 > 0.0)) { 1.0 } else { 0.0 };

        s.v[3409] = if (p.p53 > 0.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_95(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[3409] != 0.0) {
            s.copy_ad(802, 746);
        }

        if (!(s.v[3409] != 0.0)) {
            s.store_scalar(767, 0.0);
        }

        if (p.p28 != 0.0) {
            s.store_scalar(800, 1.0);
        }

        if (p.p28 != 0.0) {
            s.store_scalar(801, 1.0);
        }

    }
}

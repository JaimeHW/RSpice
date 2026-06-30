#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_23(
        s: &mut ReactiveScratch,
    ) {
        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && s.b[2827]) {
            s.copy_ad(2027, 1942);
            s.copy_ad(1942, 1943);
            s.copy_ad(1943, 2027);
        }

        s.b[2828] = (s.v[1] == 3.0);
        s.store_scalar(2828, if s.b[2828] { 1.0 } else { 0.0 });

        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) {
            s.store_add_scaled_product_left_ad(1960, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.25, s.ad_value(1936))))), 1932, 1.0);
        }

        s.b[2829] = (((s.v[1960]) as f64).abs() <= s.v[1933]);
        s.store_scalar(2829, if s.b[2829] { 1.0 } else { 0.0 });

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && s.b[2829]) {
            s.store_mul_ad_affine_product_rhs(1996, 1960, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1960), 1.0, A::scale(s.ad_value(1960), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2830] = ((((-s.v[1960])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2830, if s.b[2830] { 1.0 } else { 0.0 });

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2829])) && s.b[2830]) {
            s.store_exp_neg_input(2027, 1960);
        }

        s.b[2831] = ((-s.v[1960]) < 0.0);
        s.store_scalar(2831, if s.b[2831] { 1.0 } else { 0.0 });

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2829])) && (!s.b[2830])) && s.b[2831]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1960)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2829])) && (!s.b[2830])) && (!s.b[2831])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1960)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2829])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0)));
        }

        s.b[2832] = (s.v[1960] > s.v[1933]);
        s.store_scalar(2832, if s.b[2832] { 1.0 } else { 0.0 });

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2829])) && s.b[2832]) {
            s.store_neg(1996, 1996);
        }

        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) {
            s.store_add_scaled_product_right_sub(1942, 1996, (-1.0), 1937, 1890, 1960, -1.0);
            s.store_add_scaled_product_left_ad(1961, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.5, s.ad_value(1936))))), 1932, 1.0);
        }

        s.b[2833] = (((s.v[1961]) as f64).abs() <= s.v[1933]);
        s.store_scalar(2833, if s.b[2833] { 1.0 } else { 0.0 });

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && s.b[2833]) {
            s.store_mul_ad_affine_product_rhs(1996, 1961, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1961), 1.0, A::scale(s.ad_value(1961), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2834] = ((((-s.v[1961])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2834, if s.b[2834] { 1.0 } else { 0.0 });

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2833])) && s.b[2834]) {
            s.store_exp_neg_input(2027, 1961);
        }

        s.b[2835] = ((-s.v[1961]) < 0.0);
        s.store_scalar(2835, if s.b[2835] { 1.0 } else { 0.0 });

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2833])) && (!s.b[2834])) && s.b[2835]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1961)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2833])) && (!s.b[2834])) && (!s.b[2835])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1961)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2833])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1961)), (-1.0)));
        }

        s.b[2836] = (s.v[1961] > s.v[1933]);
        s.store_scalar(2836, if s.b[2836] { 1.0 } else { 0.0 });

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2833])) && s.b[2836]) {
            s.store_neg(1996, 1996);
        }

        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) {
            s.store_add_scaled_product_right_sub(1943, 1996, (-1.0), 1937, 1890, 1961, -1.0);
            s.store_add_scaled_product_left_ad(1962, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.75, s.ad_value(1936))))), 1932, 1.0);
        }

        s.b[2837] = (((s.v[1962]) as f64).abs() <= s.v[1933]);
        s.store_scalar(2837, if s.b[2837] { 1.0 } else { 0.0 });

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && s.b[2837]) {
            s.store_mul_ad_affine_product_rhs(1996, 1962, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1962), 1.0, A::scale(s.ad_value(1962), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2838] = ((((-s.v[1962])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2838, if s.b[2838] { 1.0 } else { 0.0 });

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2837])) && s.b[2838]) {
            s.store_exp_neg_input(2027, 1962);
        }

        s.b[2839] = ((-s.v[1962]) < 0.0);
        s.store_scalar(2839, if s.b[2839] { 1.0 } else { 0.0 });

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2837])) && (!s.b[2838])) && s.b[2839]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1962)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2837])) && (!s.b[2838])) && (!s.b[2839])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1962)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2837])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1962)), (-1.0)));
        }

        s.b[2840] = (s.v[1962] > s.v[1933]);
        s.store_scalar(2840, if s.b[2840] { 1.0 } else { 0.0 });

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2837])) && s.b[2840]) {
            s.store_neg(1996, 1996);
        }

        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) {
            s.store_add_scaled_product_right_sub(1944, 1996, (-1.0), 1937, 1890, 1962, -1.0);
        }

        s.b[2841] = (s.v[831] < 0.0);
        s.store_scalar(2841, if s.b[2841] { 1.0 } else { 0.0 });

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && s.b[2841]) {
            s.copy_ad(2027, 1942);
            s.copy_ad(1942, 1944);
            s.copy_ad(1944, 2027);
        }

        s.b[2842] = (s.v[1] == 5.0);
        s.store_scalar(2842, if s.b[2842] { 1.0 } else { 0.0 });

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) {
            s.store_add_scaled_product_left_ad(1960, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.16666666666666666, s.ad_value(1936))))), 1932, 1.0);
        }

        s.b[2843] = (((s.v[1960]) as f64).abs() <= s.v[1933]);
        s.store_scalar(2843, if s.b[2843] { 1.0 } else { 0.0 });

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && s.b[2843]) {
            s.store_mul_ad_affine_product_rhs(1996, 1960, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1960), 1.0, A::scale(s.ad_value(1960), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2844] = ((((-s.v[1960])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2844, if s.b[2844] { 1.0 } else { 0.0 });

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2843])) && s.b[2844]) {
            s.store_exp_neg_input(2027, 1960);
        }

        s.b[2845] = ((-s.v[1960]) < 0.0);
        s.store_scalar(2845, if s.b[2845] { 1.0 } else { 0.0 });

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2843])) && (!s.b[2844])) && s.b[2845]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1960)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2843])) && (!s.b[2844])) && (!s.b[2845])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1960)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2843])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0)));
        }

        s.b[2846] = (s.v[1960] > s.v[1933]);
        s.store_scalar(2846, if s.b[2846] { 1.0 } else { 0.0 });

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2843])) && s.b[2846]) {
            s.store_neg(1996, 1996);
        }

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) {
            s.store_add_scaled_product_right_sub(1942, 1996, (-1.0), 1937, 1890, 1960, -1.0);
            s.store_add_scaled_product_left_ad(1961, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.3333333333333333, s.ad_value(1936))))), 1932, 1.0);
        }

        s.b[2847] = (((s.v[1961]) as f64).abs() <= s.v[1933]);
        s.store_scalar(2847, if s.b[2847] { 1.0 } else { 0.0 });

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && s.b[2847]) {
            s.store_mul_ad_affine_product_rhs(1996, 1961, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1961), 1.0, A::scale(s.ad_value(1961), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2848] = ((((-s.v[1961])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2848, if s.b[2848] { 1.0 } else { 0.0 });

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2847])) && s.b[2848]) {
            s.store_exp_neg_input(2027, 1961);
        }

        s.b[2849] = ((-s.v[1961]) < 0.0);
        s.store_scalar(2849, if s.b[2849] { 1.0 } else { 0.0 });

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2847])) && (!s.b[2848])) && s.b[2849]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1961)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2847])) && (!s.b[2848])) && (!s.b[2849])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1961)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2847])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1961)), (-1.0)));
        }

        s.b[2850] = (s.v[1961] > s.v[1933]);
        s.store_scalar(2850, if s.b[2850] { 1.0 } else { 0.0 });

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2847])) && s.b[2850]) {
            s.store_neg(1996, 1996);
        }

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) {
            s.store_add_scaled_product_right_sub(1943, 1996, (-1.0), 1937, 1890, 1961, -1.0);
            s.store_add_scaled_product_left_ad(1962, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.5, s.ad_value(1936))))), 1932, 1.0);
        }

        s.b[2851] = (((s.v[1962]) as f64).abs() <= s.v[1933]);
        s.store_scalar(2851, if s.b[2851] { 1.0 } else { 0.0 });

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && s.b[2851]) {
            s.store_mul_ad_affine_product_rhs(1996, 1962, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1962), 1.0, A::scale(s.ad_value(1962), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2852] = ((((-s.v[1962])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2852, if s.b[2852] { 1.0 } else { 0.0 });

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2851])) && s.b[2852]) {
            s.store_exp_neg_input(2027, 1962);
        }

        s.b[2853] = ((-s.v[1962]) < 0.0);
        s.store_scalar(2853, if s.b[2853] { 1.0 } else { 0.0 });

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2851])) && (!s.b[2852])) && s.b[2853]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1962)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2851])) && (!s.b[2852])) && (!s.b[2853])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1962)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2851])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1962)), (-1.0)));
        }

        s.b[2854] = (s.v[1962] > s.v[1933]);
        s.store_scalar(2854, if s.b[2854] { 1.0 } else { 0.0 });

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2851])) && s.b[2854]) {
            s.store_neg(1996, 1996);
        }

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) {
            s.store_add_scaled_product_right_sub(1944, 1996, (-1.0), 1937, 1890, 1962, -1.0);
            s.store_add_scaled_product_left_ad(1963, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.6666666666666666, s.ad_value(1936))))), 1932, 1.0);
        }

        s.b[2855] = (((s.v[1963]) as f64).abs() <= s.v[1933]);
        s.store_scalar(2855, if s.b[2855] { 1.0 } else { 0.0 });

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && s.b[2855]) {
            s.store_mul_ad_affine_product_rhs(1996, 1963, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1963), 1.0, A::scale(s.ad_value(1963), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2856] = ((((-s.v[1963])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2856, if s.b[2856] { 1.0 } else { 0.0 });

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2855])) && s.b[2856]) {
            s.store_exp_neg_input(2027, 1963);
        }

        s.b[2857] = ((-s.v[1963]) < 0.0);
        s.store_scalar(2857, if s.b[2857] { 1.0 } else { 0.0 });

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2855])) && (!s.b[2856])) && s.b[2857]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1963)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2855])) && (!s.b[2856])) && (!s.b[2857])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1963)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2855])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1963)), (-1.0)));
        }

        s.b[2858] = (s.v[1963] > s.v[1933]);
        s.store_scalar(2858, if s.b[2858] { 1.0 } else { 0.0 });

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2855])) && s.b[2858]) {
            s.store_neg(1996, 1996);
        }

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) {
            s.store_add_scaled_product_right_sub(1945, 1996, (-1.0), 1937, 1890, 1963, -1.0);
            s.store_add_scaled_product_left_ad(1964, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.8333333333333333, s.ad_value(1936))))), 1932, 1.0);
        }

        s.b[2859] = (((s.v[1964]) as f64).abs() <= s.v[1933]);
        s.store_scalar(2859, if s.b[2859] { 1.0 } else { 0.0 });

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && s.b[2859]) {
            s.store_mul_ad_affine_product_rhs(1996, 1964, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1964), 1.0, A::scale(s.ad_value(1964), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2860] = ((((-s.v[1964])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2860, if s.b[2860] { 1.0 } else { 0.0 });

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2859])) && s.b[2860]) {
            s.store_exp_neg_input(2027, 1964);
        }

        s.b[2861] = ((-s.v[1964]) < 0.0);
        s.store_scalar(2861, if s.b[2861] { 1.0 } else { 0.0 });

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2859])) && (!s.b[2860])) && s.b[2861]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1964)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2859])) && (!s.b[2860])) && (!s.b[2861])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1964)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2859])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1964)), (-1.0)));
        }

        s.b[2862] = (s.v[1964] > s.v[1933]);
        s.store_scalar(2862, if s.b[2862] { 1.0 } else { 0.0 });

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2859])) && s.b[2862]) {
            s.store_neg(1996, 1996);
        }

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) {
            s.store_add_scaled_product_right_sub(1946, 1996, (-1.0), 1937, 1890, 1964, -1.0);
        }

        s.b[2863] = (s.v[831] < 0.0);
        s.store_scalar(2863, if s.b[2863] { 1.0 } else { 0.0 });

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && s.b[2863]) {
            s.copy_ad(2027, 1942);
            s.copy_ad(1942, 1946);
            s.copy_ad(1946, 2027);
            s.copy_ad(2027, 1943);
            s.copy_ad(1943, 1945);
            s.copy_ad(1945, 2027);
        }

        s.b[2864] = (s.v[1] == 9.0);
        s.store_scalar(2864, if s.b[2864] { 1.0 } else { 0.0 });

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_add_scaled_product_left_ad(1960, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.1, s.ad_value(1936))))), 1932, 1.0);
        }

        s.b[2865] = (((s.v[1960]) as f64).abs() <= s.v[1933]);
        s.store_scalar(2865, if s.b[2865] { 1.0 } else { 0.0 });

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2865]) {
            s.store_mul_ad_affine_product_rhs(1996, 1960, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1960), 1.0, A::scale(s.ad_value(1960), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2866] = ((((-s.v[1960])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2866, if s.b[2866] { 1.0 } else { 0.0 });

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2865])) && s.b[2866]) {
            s.store_exp_neg_input(2027, 1960);
        }

        s.b[2867] = ((-s.v[1960]) < 0.0);
        s.store_scalar(2867, if s.b[2867] { 1.0 } else { 0.0 });

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2865])) && (!s.b[2866])) && s.b[2867]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1960)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2865])) && (!s.b[2866])) && (!s.b[2867])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1960)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2865])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0)));
        }

        s.b[2868] = (s.v[1960] > s.v[1933]);
        s.store_scalar(2868, if s.b[2868] { 1.0 } else { 0.0 });

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2865])) && s.b[2868]) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_add_scaled_product_right_sub(1942, 1996, (-1.0), 1937, 1890, 1960, -1.0);
            s.store_add_scaled_product_left_ad(1961, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.2, s.ad_value(1936))))), 1932, 1.0);
        }

        s.b[2869] = (((s.v[1961]) as f64).abs() <= s.v[1933]);
        s.store_scalar(2869, if s.b[2869] { 1.0 } else { 0.0 });

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2869]) {
            s.store_mul_ad_affine_product_rhs(1996, 1961, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1961), 1.0, A::scale(s.ad_value(1961), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2870] = ((((-s.v[1961])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2870, if s.b[2870] { 1.0 } else { 0.0 });

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2869])) && s.b[2870]) {
            s.store_exp_neg_input(2027, 1961);
        }

        s.b[2871] = ((-s.v[1961]) < 0.0);
        s.store_scalar(2871, if s.b[2871] { 1.0 } else { 0.0 });

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2869])) && (!s.b[2870])) && s.b[2871]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1961)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2869])) && (!s.b[2870])) && (!s.b[2871])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1961)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2869])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1961)), (-1.0)));
        }

        s.b[2872] = (s.v[1961] > s.v[1933]);
        s.store_scalar(2872, if s.b[2872] { 1.0 } else { 0.0 });

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2869])) && s.b[2872]) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_add_scaled_product_right_sub(1943, 1996, (-1.0), 1937, 1890, 1961, -1.0);
            s.store_add_scaled_product_left_ad(1962, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.3, s.ad_value(1936))))), 1932, 1.0);
        }

        s.b[2873] = (((s.v[1962]) as f64).abs() <= s.v[1933]);
        s.store_scalar(2873, if s.b[2873] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_24(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2873]) {
            s.store_mul_ad_affine_product_rhs(1996, 1962, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1962), 1.0, A::scale(s.ad_value(1962), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2874] = ((((-s.v[1962])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2874, if s.b[2874] { 1.0 } else { 0.0 });

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2873])) && s.b[2874]) {
            s.store_exp_neg_input(2027, 1962);
        }

        s.b[2875] = ((-s.v[1962]) < 0.0);
        s.store_scalar(2875, if s.b[2875] { 1.0 } else { 0.0 });

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2873])) && (!s.b[2874])) && s.b[2875]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1962)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2873])) && (!s.b[2874])) && (!s.b[2875])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1962)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2873])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1962)), (-1.0)));
        }

        s.b[2876] = (s.v[1962] > s.v[1933]);
        s.store_scalar(2876, if s.b[2876] { 1.0 } else { 0.0 });

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2873])) && s.b[2876]) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_add_scaled_product_right_sub(1944, 1996, (-1.0), 1937, 1890, 1962, -1.0);
            s.store_add_scaled_product_left_ad(1963, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.4, s.ad_value(1936))))), 1932, 1.0);
        }

        s.b[2877] = (((s.v[1963]) as f64).abs() <= s.v[1933]);
        s.store_scalar(2877, if s.b[2877] { 1.0 } else { 0.0 });

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2877]) {
            s.store_mul_ad_affine_product_rhs(1996, 1963, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1963), 1.0, A::scale(s.ad_value(1963), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2878] = ((((-s.v[1963])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2878, if s.b[2878] { 1.0 } else { 0.0 });

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2877])) && s.b[2878]) {
            s.store_exp_neg_input(2027, 1963);
        }

        s.b[2879] = ((-s.v[1963]) < 0.0);
        s.store_scalar(2879, if s.b[2879] { 1.0 } else { 0.0 });

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2877])) && (!s.b[2878])) && s.b[2879]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1963)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2877])) && (!s.b[2878])) && (!s.b[2879])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1963)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2877])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1963)), (-1.0)));
        }

        s.b[2880] = (s.v[1963] > s.v[1933]);
        s.store_scalar(2880, if s.b[2880] { 1.0 } else { 0.0 });

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2877])) && s.b[2880]) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_add_scaled_product_right_sub(1945, 1996, (-1.0), 1937, 1890, 1963, -1.0);
            s.store_add_scaled_product_left_ad(1964, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.5, s.ad_value(1936))))), 1932, 1.0);
        }

        s.b[2881] = (((s.v[1964]) as f64).abs() <= s.v[1933]);
        s.store_scalar(2881, if s.b[2881] { 1.0 } else { 0.0 });

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2881]) {
            s.store_mul_ad_affine_product_rhs(1996, 1964, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1964), 1.0, A::scale(s.ad_value(1964), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2882] = ((((-s.v[1964])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2882, if s.b[2882] { 1.0 } else { 0.0 });

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2881])) && s.b[2882]) {
            s.store_exp_neg_input(2027, 1964);
        }

        s.b[2883] = ((-s.v[1964]) < 0.0);
        s.store_scalar(2883, if s.b[2883] { 1.0 } else { 0.0 });

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2881])) && (!s.b[2882])) && s.b[2883]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1964)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2881])) && (!s.b[2882])) && (!s.b[2883])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1964)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2881])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1964)), (-1.0)));
        }

        s.b[2884] = (s.v[1964] > s.v[1933]);
        s.store_scalar(2884, if s.b[2884] { 1.0 } else { 0.0 });

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2881])) && s.b[2884]) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_add_scaled_product_right_sub(1946, 1996, (-1.0), 1937, 1890, 1964, -1.0);
            s.store_add_scaled_product_left_ad(1965, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.6, s.ad_value(1936))))), 1932, 1.0);
        }

        s.b[2885] = (((s.v[1965]) as f64).abs() <= s.v[1933]);
        s.store_scalar(2885, if s.b[2885] { 1.0 } else { 0.0 });

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2885]) {
            s.store_mul_ad_affine_product_rhs(1996, 1965, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1965), 1.0, A::scale(s.ad_value(1965), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2886] = ((((-s.v[1965])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2886, if s.b[2886] { 1.0 } else { 0.0 });

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2885])) && s.b[2886]) {
            s.store_exp_neg_input(2027, 1965);
        }

        s.b[2887] = ((-s.v[1965]) < 0.0);
        s.store_scalar(2887, if s.b[2887] { 1.0 } else { 0.0 });

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2885])) && (!s.b[2886])) && s.b[2887]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1965)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2885])) && (!s.b[2886])) && (!s.b[2887])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1965)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2885])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1965)), (-1.0)));
        }

        s.b[2888] = (s.v[1965] > s.v[1933]);
        s.store_scalar(2888, if s.b[2888] { 1.0 } else { 0.0 });

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2885])) && s.b[2888]) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_add_scaled_product_right_sub(1947, 1996, (-1.0), 1937, 1890, 1965, -1.0);
            s.store_add_scaled_product_left_ad(1966, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.7, s.ad_value(1936))))), 1932, 1.0);
        }

        s.b[2889] = (((s.v[1966]) as f64).abs() <= s.v[1933]);
        s.store_scalar(2889, if s.b[2889] { 1.0 } else { 0.0 });

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2889]) {
            s.store_mul_ad_affine_product_rhs(1996, 1966, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1966), 1.0, A::scale(s.ad_value(1966), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2890] = ((((-s.v[1966])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2890, if s.b[2890] { 1.0 } else { 0.0 });

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2889])) && s.b[2890]) {
            s.store_exp_neg_input(2027, 1966);
        }

        s.b[2891] = ((-s.v[1966]) < 0.0);
        s.store_scalar(2891, if s.b[2891] { 1.0 } else { 0.0 });

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2889])) && (!s.b[2890])) && s.b[2891]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1966)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2889])) && (!s.b[2890])) && (!s.b[2891])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1966)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2889])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1966)), (-1.0)));
        }

        s.b[2892] = (s.v[1966] > s.v[1933]);
        s.store_scalar(2892, if s.b[2892] { 1.0 } else { 0.0 });

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2889])) && s.b[2892]) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_add_scaled_product_right_sub(1948, 1996, (-1.0), 1937, 1890, 1966, -1.0);
            s.store_add_scaled_product_left_ad(1967, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.8, s.ad_value(1936))))), 1932, 1.0);
        }

        s.b[2893] = (((s.v[1967]) as f64).abs() <= s.v[1933]);
        s.store_scalar(2893, if s.b[2893] { 1.0 } else { 0.0 });

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2893]) {
            s.store_mul_ad_affine_product_rhs(1996, 1967, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1967), 1.0, A::scale(s.ad_value(1967), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2894] = ((((-s.v[1967])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2894, if s.b[2894] { 1.0 } else { 0.0 });

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2893])) && s.b[2894]) {
            s.store_exp_neg_input(2027, 1967);
        }

        s.b[2895] = ((-s.v[1967]) < 0.0);
        s.store_scalar(2895, if s.b[2895] { 1.0 } else { 0.0 });

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2893])) && (!s.b[2894])) && s.b[2895]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1967)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2893])) && (!s.b[2894])) && (!s.b[2895])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1967)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2893])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1967)), (-1.0)));
        }

        s.b[2896] = (s.v[1967] > s.v[1933]);
        s.store_scalar(2896, if s.b[2896] { 1.0 } else { 0.0 });

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2893])) && s.b[2896]) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_add_scaled_product_right_sub(1949, 1996, (-1.0), 1937, 1890, 1967, -1.0);
            s.store_add_scaled_product_left_ad(1968, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.9, s.ad_value(1936))))), 1932, 1.0);
        }

        s.b[2897] = (((s.v[1968]) as f64).abs() <= s.v[1933]);
        s.store_scalar(2897, if s.b[2897] { 1.0 } else { 0.0 });

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2897]) {
            s.store_mul_ad_affine_product_rhs(1996, 1968, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1968), 1.0, A::scale(s.ad_value(1968), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2898] = ((((-s.v[1968])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2898, if s.b[2898] { 1.0 } else { 0.0 });

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2897])) && s.b[2898]) {
            s.store_exp_neg_input(2027, 1968);
        }

        s.b[2899] = ((-s.v[1968]) < 0.0);
        s.store_scalar(2899, if s.b[2899] { 1.0 } else { 0.0 });

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2897])) && (!s.b[2898])) && s.b[2899]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1968)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2897])) && (!s.b[2898])) && (!s.b[2899])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1968)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2897])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1968)), (-1.0)));
        }

        s.b[2900] = (s.v[1968] > s.v[1933]);
        s.store_scalar(2900, if s.b[2900] { 1.0 } else { 0.0 });

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2897])) && s.b[2900]) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_add_scaled_product_right_sub(1950, 1996, (-1.0), 1937, 1890, 1968, -1.0);
        }

        s.b[2901] = (s.v[831] < 0.0);
        s.store_scalar(2901, if s.b[2901] { 1.0 } else { 0.0 });

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2901]) {
            s.copy_ad(2027, 1942);
            s.copy_ad(1942, 1950);
            s.copy_ad(1950, 2027);
            s.copy_ad(2027, 1943);
            s.copy_ad(1943, 1949);
            s.copy_ad(1949, 2027);
            s.copy_ad(2027, 1944);
            s.copy_ad(1944, 1948);
            s.copy_ad(1948, 2027);
            s.copy_ad(2027, 1945);
            s.copy_ad(1945, 1947);
            s.copy_ad(1947, 2027);
        }

        s.store_scalar(1983, 0.0);

        s.store_scalar(1984, 0.0);

        s.store_scalar(1978, 0.0);

        s.store_scalar(1979, 0.0);

        s.b[2902] = (s.v[1] != 0.0);
        s.store_scalar(2902, if s.b[2902] { 1.0 } else { 0.0 });

        if s.b[2902] {
            s.store_sub_ad_rhs(1983, 1934, A::mul3_scaled_output(s.ad_value(831), s.ad_value(1893), s.ad_value(1932), 0.5));
            s.store_add_product3_rhs_indices(1984, 1934, 831, 1893, 1932, 0.5);
            s.store_scalar(1978, 0.0);
            s.store_scalar(1979, 0.0);
        }

        s.b[2903] = (s.v[1983] > 0.0);
        s.store_scalar(2903, if s.b[2903] { 1.0 } else { 0.0 });

        s.b[2904] = (((s.v[1983]) as f64).abs() <= s.v[1933]);
        s.store_scalar(2904, if s.b[2904] { 1.0 } else { 0.0 });

        if ((s.b[2902] && s.b[2903]) && s.b[2904]) {
            s.store_mul_ad_affine_product_rhs(1997, 1983, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1983), 1.0, A::scale(s.ad_value(1983), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2905] = ((((-s.v[1983])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2905, if s.b[2905] { 1.0 } else { 0.0 });

        if (((s.b[2902] && s.b[2903]) && (!s.b[2904])) && s.b[2905]) {
            s.store_exp_neg_input(2027, 1983);
        }

        s.b[2906] = ((-s.v[1983]) < 0.0);
        s.store_scalar(2906, if s.b[2906] { 1.0 } else { 0.0 });

        if ((((s.b[2902] && s.b[2903]) && (!s.b[2904])) && (!s.b[2905])) && s.b[2906]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1983)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[2902] && s.b[2903]) && (!s.b[2904])) && (!s.b[2905])) && (!s.b[2906])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1983)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[2902] && s.b[2903]) && (!s.b[2904])) {
            s.store_mul_sqrt_ad_rhs(1997, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1983)), (-1.0)));
        }

        s.b[2907] = (s.v[1983] > s.v[1933]);
        s.store_scalar(2907, if s.b[2907] { 1.0 } else { 0.0 });

        if (((s.b[2902] && s.b[2903]) && (!s.b[2904])) && s.b[2907]) {
            s.store_neg(1997, 1997);
        }

        if (s.b[2902] && s.b[2903]) {
            s.store_add_scaled_product_right_sub(1978, 1997, (-1.0), 1937, 1890, 1983, -1.0);
        }

        s.b[2908] = (s.v[1984] > 0.0);
        s.store_scalar(2908, if s.b[2908] { 1.0 } else { 0.0 });

        s.b[2909] = (((s.v[1984]) as f64).abs() <= s.v[1933]);
        s.store_scalar(2909, if s.b[2909] { 1.0 } else { 0.0 });

        if ((s.b[2902] && s.b[2908]) && s.b[2909]) {
            s.store_mul_ad_affine_product_rhs(1997, 1984, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1984), 1.0, A::scale(s.ad_value(1984), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2910] = ((((-s.v[1984])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2910, if s.b[2910] { 1.0 } else { 0.0 });

        if (((s.b[2902] && s.b[2908]) && (!s.b[2909])) && s.b[2910]) {
            s.store_exp_neg_input(2027, 1984);
        }

        s.b[2911] = ((-s.v[1984]) < 0.0);
        s.store_scalar(2911, if s.b[2911] { 1.0 } else { 0.0 });

        if ((((s.b[2902] && s.b[2908]) && (!s.b[2909])) && (!s.b[2910])) && s.b[2911]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1984)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[2902] && s.b[2908]) && (!s.b[2909])) && (!s.b[2910])) && (!s.b[2911])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1984)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[2902] && s.b[2908]) && (!s.b[2909])) {
            s.store_mul_sqrt_ad_rhs(1997, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1984)), (-1.0)));
        }

        s.b[2912] = (s.v[1984] > s.v[1933]);
        s.store_scalar(2912, if s.b[2912] { 1.0 } else { 0.0 });

        if (((s.b[2902] && s.b[2908]) && (!s.b[2909])) && s.b[2912]) {
            s.store_neg(1997, 1997);
        }

        if (s.b[2902] && s.b[2908]) {
            s.store_add_scaled_product_right_sub(1979, 1997, (-1.0), 1937, 1890, 1984, -1.0);
        }

        s.store_scaled_voltage(1969, ctx, nodes, Some(12), None, s.v[3]);

        s.store_scaled_voltage(1970, ctx, nodes, Some(13), None, s.v[3]);

        s.store_scaled_voltage(1971, ctx, nodes, Some(14), None, s.v[3]);

        s.store_scaled_voltage(1972, ctx, nodes, Some(15), None, s.v[3]);

        s.store_scaled_voltage(1973, ctx, nodes, Some(16), None, s.v[3]);

        s.store_scaled_voltage(1974, ctx, nodes, Some(17), None, s.v[3]);

        s.store_scaled_voltage(1975, ctx, nodes, Some(18), None, s.v[3]);

        s.store_scaled_voltage(1976, ctx, nodes, Some(19), None, s.v[3]);

        s.store_scaled_voltage(1977, ctx, nodes, Some(20), None, s.v[3]);

        s.store_scalar(1995, 0.0);

        s.b[2921] = (s.v[1] != 0.0);
        s.store_scalar(2921, if s.b[2921] { 1.0 } else { 0.0 });

        if s.b[2921] {
            s.store_div_scaled_product3_by_product(1995, s.ad_value(307), s.ad_value(1888), s.ad_value(716), 1.0, s.ad_value(1904), s.ad_value(1906), 1.0);
            s.store_mul_ad_product_lhs_mixed_ai(2018, A::square(s.ad_value(1907)), 1888, 1888);
        }

        s.b[2922] = (s.v[1] == 1.0);
        s.store_scalar(2922, if s.b[2922] { 1.0 } else { 0.0 });

        if (s.b[2921] && s.b[2922]) {
            s.store_sub(1992, 1979, 1978);
            s.store_add_scaled_inputs3_indices(1993, 1978, 6.0, 1979, 6.0, 1969, (-12.0));
        }

        s.b[2923] = (s.v[1] == 2.0);
        s.store_scalar(2923, if s.b[2923] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_25(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2921] && (!s.b[2922])) && s.b[2923]) {
            s.store_add_scaled_inputs4_indices(1992, 1978, ((-7.0) * 0.2), 1969, ((-3.0) * 0.2), 1970, (12.0 * 0.2), 1979, ((-2.0) * 0.2));
            s.store_add_scaled_inputs4_indices(1993, 1978, ((-4.0) * ((-18.0) / 5.0)), 1969, (9.0 * ((-18.0) / 5.0)), 1970, ((-6.0) * ((-18.0) / 5.0)), 1979, ((-18.0) / 5.0));
        }

        s.b[2924] = (s.v[1] == 3.0);
        s.store_scalar(2924, if s.b[2924] { 1.0 } else { 0.0 });

        if (((s.b[2921] && (!s.b[2922])) && (!s.b[2923])) && s.b[2924]) {
            s.store_scaled_add_ad_lhs(1992, A::add_scaled_inputs4(s.ad_value(1978), (-13.0), s.ad_value(1969), (-6.0), s.ad_value(1970), 24.0, s.ad_value(1971), (-6.0)), 1979, 0.14285714285714285);
            s.store_add_scaled_inputs_ad_lhs(1993, A::add_scaled_inputs4(s.ad_value(1978), 180.0, s.ad_value(1969), (-408.0), s.ad_value(1970), 288.0, s.ad_value(1971), (-72.0)), 0.14285714285714285, 1979, (12.0 * 0.14285714285714285));
        }

        s.b[2925] = (s.v[1] == 5.0);
        s.store_scalar(2925, if s.b[2925] { 1.0 } else { 0.0 });

        if ((((s.b[2921] && (!s.b[2922])) && (!s.b[2923])) && (!s.b[2924])) && s.b[2925]) {
            s.store_add_scaled_inputs_ad_lhs(1992, A::add(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1978), (-181.0), s.ad_value(1969), (-84.0), s.ad_value(1972), 24.0, s.ad_value(1973), (-6.0)), 1.0, s.ad_value(1971), 90.0), s.ad_value(1979)), 0.015384615384615385, 1970, (336.0 * 0.015384615384615385));
            s.store_add_scaled_inputs_ad_lhs(1993, A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1972), 432.0, s.ad_value(1973), (-108.0), s.ad_value(1971), (-1620.0), s.ad_value(1979), 18.0), 1.0, s.ad_value(1978), 3762.0), 1.0, s.ad_value(1969), 8532.0), 0.015384615384615385, 1970, (6048.0 * 0.015384615384615385));
        }

        s.b[2926] = (s.v[1] == 9.0);
        s.store_scalar(2926, if s.b[2926] { 1.0 } else { 0.0 });

        if (((((s.b[2921] && (!s.b[2922])) && (!s.b[2923])) && (!s.b[2924])) && (!s.b[2925])) && s.b[2926]) {
            s.store_sub_scaled_ad_lhs(1992, A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1974), 1680.0, s.ad_value(1972), 23400.0, s.ad_value(1979), 5.0, s.ad_value(1971), (-87330.0)), 1.0, s.ad_value(1976), 120.0), 1.0, s.ad_value(1975), 450.0), 1.0, s.ad_value(1969), 81480.0), 1.0, s.ad_value(1970), 325920.0), 1.0, s.ad_value(1978), 175565.0), 2.6434745829918846e-5, s.ad_value(1977), (30.0 * 2.6434745829918846e-5)), 1973, (30.0 / 181.0));
            s.store_sub_scaled_ad_lhs(1993, A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1975), (-13500.0), s.ad_value(1972), 702000.0, s.ad_value(1971), (-2619900.0), s.ad_value(1969), (-13793100.0)), 1.0, s.ad_value(1970), 9777600.0), 1.0, s.ad_value(1978), 6081750.0), 1.0, s.ad_value(1979), 150.0), 1.0, s.ad_value(1976), 3600.0), 1.0, s.ad_value(1977), 900.0), 2.6434745829918846e-5, s.ad_value(1974), (50400.0 * 2.6434745829918846e-5)), 1973, (900.0 / 181.0));
        }

        if (((((s.b[2921] && (!s.b[2922])) && (!s.b[2923])) && (!s.b[2924])) && (!s.b[2925])) && (!s.b[2926])) {
            s.store_scalar(1992, 0.0);
            s.store_scalar(1993, 0.0);
        }

        if s.b[2921] {
            s.store_add_div_lhs_indices(2027, 1969, 1937, 1890);
        }

        s.b[2927] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(2927, if s.b[2927] { 1.0 } else { 0.0 });

        if (s.b[2921] && s.b[2927]) {
            s.store_div(2016, 2027, 1940);
        }

        s.b[2928] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(2928, if s.b[2928] { 1.0 } else { 0.0 });

        if ((s.b[2921] && (!s.b[2927])) && s.b[2928]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[2929] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(2929, if s.b[2929] { 1.0 } else { 0.0 });

        if (((s.b[2921] && (!s.b[2927])) && s.b[2928]) && s.b[2929]) {
            s.store_exp(2005, 2015);
        }

        s.b[2930] = (s.v[2015] < 0.0);
        s.store_scalar(2930, if s.b[2930] { 1.0 } else { 0.0 });

        if ((((s.b[2921] && (!s.b[2927])) && s.b[2928]) && (!s.b[2929])) && s.b[2930]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[2921] && (!s.b[2927])) && s.b[2928]) && (!s.b[2929])) && (!s.b[2930])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[2921] && (!s.b[2927])) && s.b[2928]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2016, 2015, 2012);
        }

        if ((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[2931] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2931, if s.b[2931] { 1.0 } else { 0.0 });

        if (((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) && s.b[2931]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[2932] = ((-s.v[2011]) < 0.0);
        s.store_scalar(2932, if s.b[2932] { 1.0 } else { 0.0 });

        if ((((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) && (!s.b[2931])) && s.b[2932]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) && (!s.b[2931])) && (!s.b[2932])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[2933] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2933, if s.b[2933] { 1.0 } else { 0.0 });

        if (((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) && s.b[2933]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[2934] = ((-s.v[2013]) < 0.0);
        s.store_scalar(2934, if s.b[2934] { 1.0 } else { 0.0 });

        if ((((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) && (!s.b[2933])) && s.b[2934]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) && (!s.b[2933])) && (!s.b[2934])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2016, 2013, 2014);
        }

        s.b[2935] = (((s.v[2016]) as f64).abs() <= s.v[1933]);
        s.store_scalar(2935, if s.b[2935] { 1.0 } else { 0.0 });

        if (s.b[2921] && s.b[2935]) {
            s.store_mul_ad_affine_product_rhs(1996, 2016, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(1991, 1889, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.25), 0.3333333333333333), (-0.70710678));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1990, 1889, 1.0, 2016, 0.5, (-0.235702));
        }

        s.b[2936] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2936, if s.b[2936] { 1.0 } else { 0.0 });

        if ((s.b[2921] && (!s.b[2935])) && s.b[2936]) {
            s.store_exp_neg_input(2027, 2016);
        }

        s.b[2937] = ((-s.v[2016]) < 0.0);
        s.store_scalar(2937, if s.b[2937] { 1.0 } else { 0.0 });

        if (((s.b[2921] && (!s.b[2935])) && (!s.b[2936])) && s.b[2937]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2016)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[2921] && (!s.b[2935])) && (!s.b[2936])) && (!s.b[2937])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(2016)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (s.b[2921] && (!s.b[2935])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));
        }

        s.b[2938] = (s.v[2016] > s.v[1933]);
        s.store_scalar(2938, if s.b[2938] { 1.0 } else { 0.0 });

        if ((s.b[2921] && (!s.b[2935])) && s.b[2938]) {
            s.store_neg(1996, 1996);
        }

        if (s.b[2921] && (!s.b[2935])) {
            s.store_div_scaled_product3_mixed_iiai(1991, 1889, 1889, A::sub_from_scalar(1.0, s.ad_value(2027)), 0.5, 1996, 1.0);
            s.store_add_div_lhs_mixed_ai(1990, A::add_scaled_square_product(s.ad_value(1991), 1.0, s.ad_value(1889), s.ad_value(1889), (-0.5)), 1996, 1991);
        }

        if s.b[2921] {
            s.store_sub(1988, 1937, 1991);
            s.store_div_from_scalar(1989, 1.0, 1988);
            s.store_offset_mul(1987, 1969, 1989, (-1.0));
            s.store_mul_sub_from_scalar_ad_lhs(1986, 1.0, A::mul(A::mul3(s.ad_value(1969), s.ad_value(1990), s.ad_value(1989)), s.ad_value(1989)), 1989);
            s.store_add_scaled_product_value_ad(2017, A::mul3(s.ad_value(1986), s.ad_value(1992), s.ad_value(1992)), 1.0, 1987, 1993, 1.0);
            s.store_mul_ad_product_lhs_mixed_ai(1994, A::square(s.ad_value(1992)), 1989, 1989);
            s.store_mul(1985, 2018, 1994);
        }

        s.b[2939] = (s.v[0] == (-1.0));
        s.store_scalar(2939, if s.b[2939] { 1.0 } else { 0.0 });

        if (s.b[2921] && s.b[2939]) {
            s.store_div_scaled_value_offset_denominator(1985, s.ad_value(1985), 1.0, A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0, 1.0);
        }

        if s.b[2921] {
            s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);
            s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);
            s.store_add_scaled_product_indices(2028, 1993, 1.0, 1994, 1990, (-1.0));
            s.store_mul_sub_ad_rhs(1951, 2019, s.ad_value(2017), A::div_scaled_product(A::mul3(s.ad_value(1985), s.ad_value(1987), s.ad_value(2028)), s.ad_value(2019), 1.0, s.ad_value(2027), 1.0));
        }

        if (!s.b[2921]) {
            s.store_scalar(2018, 0.0);
        }

        s.b[2940] = (s.v[1] >= 2.0);
        s.store_scalar(2940, if s.b[2940] { 1.0 } else { 0.0 });

        s.b[2941] = (s.v[1] == 2.0);
        s.store_scalar(2941, if s.b[2941] { 1.0 } else { 0.0 });

        if (s.b[2940] && s.b[2941]) {
            s.store_add_scaled_inputs4_indices(1992, 1978, (2.0 * 0.2), 1969, ((-12.0) * 0.2), 1970, (3.0 * 0.2), 1979, (7.0 * 0.2));
            s.store_add_scaled_inputs4_indices(1993, 1979, ((-4.0) * ((-18.0) / 5.0)), 1970, (9.0 * ((-18.0) / 5.0)), 1969, ((-6.0) * ((-18.0) / 5.0)), 1978, ((-18.0) / 5.0));
        }

        s.b[2942] = (s.v[1] == 3.0);
        s.store_scalar(2942, if s.b[2942] { 1.0 } else { 0.0 });

        if ((s.b[2940] && (!s.b[2941])) && s.b[2942]) {
            s.store_add_scaled_inputs4_indices(1992, 1978, 0.5, 1969, (-3.0), 1971, 3.0, 1979, (-0.5));
            s.store_sub_scaled_inputs_ad_lhs(1993, A::add_scaled_inputs4(s.ad_value(1978), (-48.0), s.ad_value(1969), 288.0, s.ad_value(1970), (-480.0), s.ad_value(1971), 288.0), 0.14285714285714285, 1979, (48.0 * 0.14285714285714285));
        }

        s.b[2943] = (s.v[1] == 5.0);
        s.store_scalar(2943, if s.b[2943] { 1.0 } else { 0.0 });

        if (((s.b[2940] && (!s.b[2941])) && (!s.b[2942])) && s.b[2943]) {
            s.store_add_ad(1992, A::add_scaled_inputs4(s.ad_value(1969), ((-291.0) * 0.015384615384615385), s.ad_value(1970), ((-6.0) * 0.015384615384615385), s.ad_value(1972), ((-84.0) * 0.015384615384615385), s.ad_value(1973), (21.0 * 0.015384615384615385)), A::add_scaled_inputs3(s.ad_value(1971), (630.0 * 0.007692307692307693), s.ad_value(1979), ((-7.0) * 0.007692307692307693), s.ad_value(1978), (97.0 * 0.007692307692307693)));
            s.store_sub_scaled_inputs_ad_lhs(1993, A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1972), (-1728.0), s.ad_value(1973), 432.0, s.ad_value(1971), 6480.0, s.ad_value(1979), (-72.0)), 1.0, s.ad_value(1978), 1008.0), 1.0, s.ad_value(1969), 6048.0), 0.015384615384615385, 1970, (10152.0 * 0.015384615384615385));
        }

        s.b[2944] = (s.v[1] == 9.0);
        s.store_scalar(2944, if s.b[2944] { 1.0 } else { 0.0 });

        if ((((s.b[2940] && (!s.b[2941])) && (!s.b[2942])) && (!s.b[2943])) && s.b[2944]) {
            s.store_add_ad(1992, A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1974), (-5880.0), s.ad_value(1972), (-81900.0), s.ad_value(1971), 305655.0, s.ad_value(1976), (-420.0)), 1.0, s.ad_value(1977), 105.0), 1.0, s.ad_value(1969), 282255.0), 1.0, s.ad_value(1975), 1575.0), 2.6434745829918846e-5, s.ad_value(1970), (5850.0 * 2.6434745829918846e-5)), 1.0, s.ad_value(1973), (105.0 / 181.0)), A::sub_scaled_inputs(s.ad_value(1978), (94085.0 * 1.3217372914959423e-5), s.ad_value(1979), (35.0 * 1.3217372914959423e-5)));
            s.store_add_scaled_ad_lhs(1993, A::sub_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1969), 9777600.0, s.ad_value(1975), 54000.0, s.ad_value(1972), (-2808000.0), s.ad_value(1971), 10479600.0), 1.0, s.ad_value(1970), 16413000.0), 1.0, s.ad_value(1978), 1629600.0), 1.0, s.ad_value(1979), 600.0), 1.0, s.ad_value(1976), 14400.0), 1.0, s.ad_value(1977), 3600.0), 2.6434745829918846e-5, s.ad_value(1974), (201600.0 * 2.6434745829918846e-5)), 1973, (3600.0 * 0.0055248618784530384));
        }

        if ((((s.b[2940] && (!s.b[2941])) && (!s.b[2942])) && (!s.b[2943])) && (!s.b[2944])) {
            s.store_scalar(1992, 0.0);
            s.store_scalar(1993, 0.0);
        }

        if s.b[2940] {
            s.store_add_div_lhs_indices(2027, 1970, 1937, 1890);
        }

        s.b[2945] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(2945, if s.b[2945] { 1.0 } else { 0.0 });

        if (s.b[2940] && s.b[2945]) {
            s.store_div(2016, 2027, 1940);
        }

        s.b[2946] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(2946, if s.b[2946] { 1.0 } else { 0.0 });

        if ((s.b[2940] && (!s.b[2945])) && s.b[2946]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[2947] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(2947, if s.b[2947] { 1.0 } else { 0.0 });

        if (((s.b[2940] && (!s.b[2945])) && s.b[2946]) && s.b[2947]) {
            s.store_exp(2005, 2015);
        }

        s.b[2948] = (s.v[2015] < 0.0);
        s.store_scalar(2948, if s.b[2948] { 1.0 } else { 0.0 });

        if ((((s.b[2940] && (!s.b[2945])) && s.b[2946]) && (!s.b[2947])) && s.b[2948]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[2940] && (!s.b[2945])) && s.b[2946]) && (!s.b[2947])) && (!s.b[2948])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[2940] && (!s.b[2945])) && s.b[2946]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2016, 2015, 2012);
        }

        if ((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[2949] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2949, if s.b[2949] { 1.0 } else { 0.0 });

        if (((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && s.b[2949]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[2950] = ((-s.v[2011]) < 0.0);
        s.store_scalar(2950, if s.b[2950] { 1.0 } else { 0.0 });

        if ((((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && (!s.b[2949])) && s.b[2950]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && (!s.b[2949])) && (!s.b[2950])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
        }

    }

    pub(super) fn stamp_reactive_block_26(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) {
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[2951] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2951, if s.b[2951] { 1.0 } else { 0.0 });

        if (((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && s.b[2951]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[2952] = ((-s.v[2013]) < 0.0);
        s.store_scalar(2952, if s.b[2952] { 1.0 } else { 0.0 });

        if ((((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && (!s.b[2951])) && s.b[2952]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && (!s.b[2951])) && (!s.b[2952])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2016, 2013, 2014);
        }

        s.b[2953] = (((s.v[2016]) as f64).abs() <= s.v[1933]);
        s.store_scalar(2953, if s.b[2953] { 1.0 } else { 0.0 });

        if (s.b[2940] && s.b[2953]) {
            s.store_mul_ad_affine_product_rhs(1996, 2016, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(1991, 1889, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.25), 0.3333333333333333), (-0.70710678));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1990, 1889, 1.0, 2016, 0.5, (-0.235702));
        }

        s.b[2954] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2954, if s.b[2954] { 1.0 } else { 0.0 });

        if ((s.b[2940] && (!s.b[2953])) && s.b[2954]) {
            s.store_exp_neg_input(2027, 2016);
        }

        s.b[2955] = ((-s.v[2016]) < 0.0);
        s.store_scalar(2955, if s.b[2955] { 1.0 } else { 0.0 });

        if (((s.b[2940] && (!s.b[2953])) && (!s.b[2954])) && s.b[2955]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2016)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[2940] && (!s.b[2953])) && (!s.b[2954])) && (!s.b[2955])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(2016)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (s.b[2940] && (!s.b[2953])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));
        }

        s.b[2956] = (s.v[2016] > s.v[1933]);
        s.store_scalar(2956, if s.b[2956] { 1.0 } else { 0.0 });

        if ((s.b[2940] && (!s.b[2953])) && s.b[2956]) {
            s.store_neg(1996, 1996);
        }

        if (s.b[2940] && (!s.b[2953])) {
            s.store_div_scaled_product3_mixed_iiai(1991, 1889, 1889, A::sub_from_scalar(1.0, s.ad_value(2027)), 0.5, 1996, 1.0);
            s.store_add_div_lhs_mixed_ai(1990, A::add_scaled_square_product(s.ad_value(1991), 1.0, s.ad_value(1889), s.ad_value(1889), (-0.5)), 1996, 1991);
        }

        if s.b[2940] {
            s.store_sub(1988, 1937, 1991);
            s.store_div_from_scalar(1989, 1.0, 1988);
            s.store_offset_mul(1987, 1970, 1989, (-1.0));
            s.store_mul_sub_from_scalar_ad_lhs(1986, 1.0, A::mul(A::mul3(s.ad_value(1970), s.ad_value(1990), s.ad_value(1989)), s.ad_value(1989)), 1989);
            s.store_add_scaled_product_value_ad(2017, A::mul3(s.ad_value(1986), s.ad_value(1992), s.ad_value(1992)), 1.0, 1987, 1993, 1.0);
            s.store_mul_ad_product_lhs_mixed_ai(1994, A::square(s.ad_value(1992)), 1989, 1989);
            s.store_mul(1985, 2018, 1994);
        }

        s.b[2957] = (s.v[0] == (-1.0));
        s.store_scalar(2957, if s.b[2957] { 1.0 } else { 0.0 });

        if (s.b[2940] && s.b[2957]) {
            s.store_div_scaled_value_offset_denominator(1985, s.ad_value(1985), 1.0, A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0, 1.0);
        }

        if s.b[2940] {
            s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);
            s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);
            s.store_add_scaled_product_indices(2028, 1993, 1.0, 1994, 1990, (-1.0));
            s.store_mul_sub_ad_rhs(1952, 2019, s.ad_value(2017), A::div_scaled_product(A::mul3(s.ad_value(1985), s.ad_value(1987), s.ad_value(2028)), s.ad_value(2019), 1.0, s.ad_value(2027), 1.0));
        }

        s.b[2958] = (s.v[1] >= 3.0);
        s.store_scalar(2958, if s.b[2958] { 1.0 } else { 0.0 });

        s.b[2959] = (s.v[1] == 3.0);
        s.store_scalar(2959, if s.b[2959] { 1.0 } else { 0.0 });

        if (s.b[2958] && s.b[2959]) {
            s.store_scaled_sub_ad_lhs(1992, A::add_scaled_inputs4(s.ad_value(1979), 13.0, s.ad_value(1971), 6.0, s.ad_value(1970), (-24.0), s.ad_value(1969), 6.0), 1978, 0.14285714285714285);
            s.store_add_scaled_inputs_ad_lhs(1993, A::add_scaled_inputs4(s.ad_value(1979), 180.0, s.ad_value(1971), (-408.0), s.ad_value(1970), 288.0, s.ad_value(1969), (-72.0)), 0.14285714285714285, 1978, (12.0 * 0.14285714285714285));
        }

        s.b[2960] = (s.v[1] == 5.0);
        s.store_scalar(2960, if s.b[2960] { 1.0 } else { 0.0 });

        if ((s.b[2958] && (!s.b[2959])) && s.b[2960]) {
            s.store_scaled_sub_ad_lhs(1992, A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1979), 1.0, s.ad_value(1973), (-6.0), s.ad_value(1972), 24.0, s.ad_value(1970), (-24.0)), 1.0, s.ad_value(1969), 6.0), 1978, 0.2);
            s.store_scaled_add_ad(1993, A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1972), 1296.0, s.ad_value(1970), 1296.0, s.ad_value(1973), (-324.0), s.ad_value(1969), (-324.0)), 1.0, s.ad_value(1971), 2052.0), A::add_scaled_inputs(s.ad_value(1979), 54.0, s.ad_value(1978), 54.0), 0.07692307692307693);
        }

        s.b[2961] = (s.v[1] == 9.0);
        s.store_scalar(2961, if s.b[2961] { 1.0 } else { 0.0 });

        if (((s.b[2958] && (!s.b[2959])) && (!s.b[2960])) && s.b[2961]) {
            s.store_sub_scaled_ad_lhs(1992, A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1974), 21840.0, s.ad_value(1972), 304200.0, s.ad_value(1979), 65.0, s.ad_value(1971), (-420.0)), 1.0, s.ad_value(1976), 1560.0), 1.0, s.ad_value(1978), 12605.0), 1.0, s.ad_value(1977), 390.0), 1.0, s.ad_value(1969), 75630.0), 1.0, s.ad_value(1975), 5850.0), 2.6434745829918846e-5, s.ad_value(1970), (302520.0 * 2.6434745829918846e-5)), 1973, (390.0 / 181.0));
            s.store_sub_scaled_ad_lhs(1993, A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1969), (-2619900.0), s.ad_value(1975), (-202500.0), s.ad_value(1972), 10530000.0, s.ad_value(1971), (-16601100.0)), 1.0, s.ad_value(1970), 10479600.0), 1.0, s.ad_value(1978), 436650.0), 1.0, s.ad_value(1979), 2250.0), 1.0, s.ad_value(1976), 54000.0), 1.0, s.ad_value(1977), 13500.0), 2.6434745829918846e-5, s.ad_value(1974), (756000.0 * 2.6434745829918846e-5)), 1973, (13500.0 * 0.0055248618784530384));
        }

        if (((s.b[2958] && (!s.b[2959])) && (!s.b[2960])) && (!s.b[2961])) {
            s.store_scalar(1992, 0.0);
            s.store_scalar(1993, 0.0);
        }

        if s.b[2958] {
            s.store_add_div_lhs_indices(2027, 1971, 1937, 1890);
        }

        s.b[2962] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(2962, if s.b[2962] { 1.0 } else { 0.0 });

        if (s.b[2958] && s.b[2962]) {
            s.store_div(2016, 2027, 1940);
        }

        s.b[2963] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(2963, if s.b[2963] { 1.0 } else { 0.0 });

        if ((s.b[2958] && (!s.b[2962])) && s.b[2963]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[2964] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(2964, if s.b[2964] { 1.0 } else { 0.0 });

        if (((s.b[2958] && (!s.b[2962])) && s.b[2963]) && s.b[2964]) {
            s.store_exp(2005, 2015);
        }

        s.b[2965] = (s.v[2015] < 0.0);
        s.store_scalar(2965, if s.b[2965] { 1.0 } else { 0.0 });

        if ((((s.b[2958] && (!s.b[2962])) && s.b[2963]) && (!s.b[2964])) && s.b[2965]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[2958] && (!s.b[2962])) && s.b[2963]) && (!s.b[2964])) && (!s.b[2965])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[2958] && (!s.b[2962])) && s.b[2963]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2016, 2015, 2012);
        }

        if ((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[2966] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2966, if s.b[2966] { 1.0 } else { 0.0 });

        if (((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) && s.b[2966]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[2967] = ((-s.v[2011]) < 0.0);
        s.store_scalar(2967, if s.b[2967] { 1.0 } else { 0.0 });

        if ((((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) && (!s.b[2966])) && s.b[2967]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) && (!s.b[2966])) && (!s.b[2967])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[2968] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2968, if s.b[2968] { 1.0 } else { 0.0 });

        if (((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) && s.b[2968]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[2969] = ((-s.v[2013]) < 0.0);
        s.store_scalar(2969, if s.b[2969] { 1.0 } else { 0.0 });

        if ((((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) && (!s.b[2968])) && s.b[2969]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) && (!s.b[2968])) && (!s.b[2969])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2016, 2013, 2014);
        }

        s.b[2970] = (((s.v[2016]) as f64).abs() <= s.v[1933]);
        s.store_scalar(2970, if s.b[2970] { 1.0 } else { 0.0 });

        if (s.b[2958] && s.b[2970]) {
            s.store_mul_ad_affine_product_rhs(1996, 2016, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(1991, 1889, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.25), 0.3333333333333333), (-0.70710678));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1990, 1889, 1.0, 2016, 0.5, (-0.235702));
        }

        s.b[2971] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2971, if s.b[2971] { 1.0 } else { 0.0 });

        if ((s.b[2958] && (!s.b[2970])) && s.b[2971]) {
            s.store_exp_neg_input(2027, 2016);
        }

        s.b[2972] = ((-s.v[2016]) < 0.0);
        s.store_scalar(2972, if s.b[2972] { 1.0 } else { 0.0 });

        if (((s.b[2958] && (!s.b[2970])) && (!s.b[2971])) && s.b[2972]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2016)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[2958] && (!s.b[2970])) && (!s.b[2971])) && (!s.b[2972])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(2016)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (s.b[2958] && (!s.b[2970])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));
        }

        s.b[2973] = (s.v[2016] > s.v[1933]);
        s.store_scalar(2973, if s.b[2973] { 1.0 } else { 0.0 });

        if ((s.b[2958] && (!s.b[2970])) && s.b[2973]) {
            s.store_neg(1996, 1996);
        }

        if (s.b[2958] && (!s.b[2970])) {
            s.store_div_scaled_product3_mixed_iiai(1991, 1889, 1889, A::sub_from_scalar(1.0, s.ad_value(2027)), 0.5, 1996, 1.0);
            s.store_add_div_lhs_mixed_ai(1990, A::add_scaled_square_product(s.ad_value(1991), 1.0, s.ad_value(1889), s.ad_value(1889), (-0.5)), 1996, 1991);
        }

        if s.b[2958] {
            s.store_sub(1988, 1937, 1991);
            s.store_div_from_scalar(1989, 1.0, 1988);
            s.store_offset_mul(1987, 1971, 1989, (-1.0));
            s.store_mul_sub_from_scalar_ad_lhs(1986, 1.0, A::mul(A::mul3(s.ad_value(1971), s.ad_value(1990), s.ad_value(1989)), s.ad_value(1989)), 1989);
            s.store_add_scaled_product_value_ad(2017, A::mul3(s.ad_value(1986), s.ad_value(1992), s.ad_value(1992)), 1.0, 1987, 1993, 1.0);
            s.store_mul_ad_product_lhs_mixed_ai(1994, A::square(s.ad_value(1992)), 1989, 1989);
            s.store_mul(1985, 2018, 1994);
        }

        s.b[2974] = (s.v[0] == (-1.0));
        s.store_scalar(2974, if s.b[2974] { 1.0 } else { 0.0 });

        if (s.b[2958] && s.b[2974]) {
            s.store_div_scaled_value_offset_denominator(1985, s.ad_value(1985), 1.0, A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0, 1.0);
        }

        if s.b[2958] {
            s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);
            s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);
            s.store_add_scaled_product_indices(2028, 1993, 1.0, 1994, 1990, (-1.0));
            s.store_mul_sub_ad_rhs(1953, 2019, s.ad_value(2017), A::div_scaled_product(A::mul3(s.ad_value(1985), s.ad_value(1987), s.ad_value(2028)), s.ad_value(2019), 1.0, s.ad_value(2027), 1.0));
        }

        s.b[2975] = (s.v[1] >= 4.0);
        s.store_scalar(2975, if s.b[2975] { 1.0 } else { 0.0 });

        s.b[2976] = (s.v[1] == 5.0);
        s.store_scalar(2976, if s.b[2976] { 1.0 } else { 0.0 });

        if (s.b[2975] && s.b[2976]) {
            s.store_add_scaled_inputs_ad_lhs(1992, A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1971), (-630.0), s.ad_value(1972), 12.0, s.ad_value(1973), 582.0, s.ad_value(1979), (-97.0)), 1.0, s.ad_value(1978), 7.0), 1.0, s.ad_value(1969), 42.0), 0.007692307692307693, 1970, (168.0 * 0.007692307692307693));
            s.store_sub_scaled_inputs_ad_lhs(1993, A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1972), (-10152.0), s.ad_value(1973), 6048.0, s.ad_value(1971), 6480.0, s.ad_value(1979), (-1008.0)), 1.0, s.ad_value(1978), 72.0), 1.0, s.ad_value(1969), 432.0), 0.015384615384615385, 1970, (1728.0 * 0.015384615384615385));
        }

        s.b[2977] = (s.v[1] == 9.0);
        s.store_scalar(2977, if s.b[2977] { 1.0 } else { 0.0 });

        if ((s.b[2975] && (!s.b[2976])) && s.b[2977]) {
            s.store_add_scaled_ad_lhs(1992, A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1974), (-81480.0), s.ad_value(1972), (-30.0), s.ad_value(1971), (-303975.0), s.ad_value(1976), (-5820.0)), 1.0, s.ad_value(1977), 1455.0), 1.0, s.ad_value(1969), 20265.0), 1.0, s.ad_value(1975), 21825.0), 2.6434745829918846e-5, s.ad_value(1970), (81060.0 * 2.6434745829918846e-5)), 1.0, s.ad_value(1979), (485.0 / 75658.0)), 1.0, s.ad_value(1973), (1455.0 * 0.0055248618784530384)), 1978, (6755.0 * 1.3217372914959423e-5));
            s.store_add_scaled_ad_lhs(1993, A::sub_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1969), 702000.0, s.ad_value(1975), 756000.0, s.ad_value(1972), (-16614600.0), s.ad_value(1971), 10530000.0), 1.0, s.ad_value(1970), 2808000.0), 1.0, s.ad_value(1978), 117000.0), 1.0, s.ad_value(1979), 8400.0), 1.0, s.ad_value(1976), 201600.0), 1.0, s.ad_value(1977), 50400.0), 2.6434745829918846e-5, s.ad_value(1974), (2822400.0 * 2.6434745829918846e-5)), 1973, (50400.0 * 0.0055248618784530384));
        }

        if ((s.b[2975] && (!s.b[2976])) && (!s.b[2977])) {
            s.store_scalar(1992, 0.0);
            s.store_scalar(1993, 0.0);
        }

        if s.b[2975] {
            s.store_add_div_lhs_indices(2027, 1972, 1937, 1890);
        }

        s.b[2978] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(2978, if s.b[2978] { 1.0 } else { 0.0 });

        if (s.b[2975] && s.b[2978]) {
            s.store_div(2016, 2027, 1940);
        }

    }

    pub(super) fn stamp_reactive_block_27(
        s: &mut ReactiveScratch,
    ) {
        s.b[2979] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(2979, if s.b[2979] { 1.0 } else { 0.0 });

        if ((s.b[2975] && (!s.b[2978])) && s.b[2979]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[2980] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(2980, if s.b[2980] { 1.0 } else { 0.0 });

        if (((s.b[2975] && (!s.b[2978])) && s.b[2979]) && s.b[2980]) {
            s.store_exp(2005, 2015);
        }

        s.b[2981] = (s.v[2015] < 0.0);
        s.store_scalar(2981, if s.b[2981] { 1.0 } else { 0.0 });

        if ((((s.b[2975] && (!s.b[2978])) && s.b[2979]) && (!s.b[2980])) && s.b[2981]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[2975] && (!s.b[2978])) && s.b[2979]) && (!s.b[2980])) && (!s.b[2981])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[2975] && (!s.b[2978])) && s.b[2979]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2016, 2015, 2012);
        }

        if ((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[2982] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2982, if s.b[2982] { 1.0 } else { 0.0 });

        if (((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) && s.b[2982]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[2983] = ((-s.v[2011]) < 0.0);
        s.store_scalar(2983, if s.b[2983] { 1.0 } else { 0.0 });

        if ((((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) && (!s.b[2982])) && s.b[2983]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) && (!s.b[2982])) && (!s.b[2983])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[2984] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2984, if s.b[2984] { 1.0 } else { 0.0 });

        if (((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) && s.b[2984]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[2985] = ((-s.v[2013]) < 0.0);
        s.store_scalar(2985, if s.b[2985] { 1.0 } else { 0.0 });

        if ((((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) && (!s.b[2984])) && s.b[2985]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) && (!s.b[2984])) && (!s.b[2985])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2016, 2013, 2014);
        }

        s.b[2986] = (((s.v[2016]) as f64).abs() <= s.v[1933]);
        s.store_scalar(2986, if s.b[2986] { 1.0 } else { 0.0 });

        if (s.b[2975] && s.b[2986]) {
            s.store_mul_ad_affine_product_rhs(1996, 2016, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(1991, 1889, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.25), 0.3333333333333333), (-0.70710678));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1990, 1889, 1.0, 2016, 0.5, (-0.235702));
        }

        s.b[2987] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2987, if s.b[2987] { 1.0 } else { 0.0 });

        if ((s.b[2975] && (!s.b[2986])) && s.b[2987]) {
            s.store_exp_neg_input(2027, 2016);
        }

        s.b[2988] = ((-s.v[2016]) < 0.0);
        s.store_scalar(2988, if s.b[2988] { 1.0 } else { 0.0 });

        if (((s.b[2975] && (!s.b[2986])) && (!s.b[2987])) && s.b[2988]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2016)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[2975] && (!s.b[2986])) && (!s.b[2987])) && (!s.b[2988])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(2016)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (s.b[2975] && (!s.b[2986])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));
        }

        s.b[2989] = (s.v[2016] > s.v[1933]);
        s.store_scalar(2989, if s.b[2989] { 1.0 } else { 0.0 });

        if ((s.b[2975] && (!s.b[2986])) && s.b[2989]) {
            s.store_neg(1996, 1996);
        }

        if (s.b[2975] && (!s.b[2986])) {
            s.store_div_scaled_product3_mixed_iiai(1991, 1889, 1889, A::sub_from_scalar(1.0, s.ad_value(2027)), 0.5, 1996, 1.0);
            s.store_add_div_lhs_mixed_ai(1990, A::add_scaled_square_product(s.ad_value(1991), 1.0, s.ad_value(1889), s.ad_value(1889), (-0.5)), 1996, 1991);
        }

        if s.b[2975] {
            s.store_sub(1988, 1937, 1991);
            s.store_div_from_scalar(1989, 1.0, 1988);
            s.store_offset_mul(1987, 1972, 1989, (-1.0));
            s.store_mul_sub_from_scalar_ad_lhs(1986, 1.0, A::mul(A::mul3(s.ad_value(1972), s.ad_value(1990), s.ad_value(1989)), s.ad_value(1989)), 1989);
            s.store_add_scaled_product_value_ad(2017, A::mul3(s.ad_value(1986), s.ad_value(1992), s.ad_value(1992)), 1.0, 1987, 1993, 1.0);
            s.store_mul_ad_product_lhs_mixed_ai(1994, A::square(s.ad_value(1992)), 1989, 1989);
            s.store_mul(1985, 2018, 1994);
        }

        s.b[2990] = (s.v[0] == (-1.0));
        s.store_scalar(2990, if s.b[2990] { 1.0 } else { 0.0 });

        if (s.b[2975] && s.b[2990]) {
            s.store_div_scaled_value_offset_denominator(1985, s.ad_value(1985), 1.0, A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0, 1.0);
        }

        if s.b[2975] {
            s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);
            s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);
            s.store_add_scaled_product_indices(2028, 1993, 1.0, 1994, 1990, (-1.0));
            s.store_mul_sub_ad_rhs(1954, 2019, s.ad_value(2017), A::div_scaled_product(A::mul3(s.ad_value(1985), s.ad_value(1987), s.ad_value(2028)), s.ad_value(2019), 1.0, s.ad_value(2027), 1.0));
        }

        s.b[2991] = (s.v[1] >= 5.0);
        s.store_scalar(2991, if s.b[2991] { 1.0 } else { 0.0 });

        s.b[2992] = (s.v[1] == 5.0);
        s.store_scalar(2992, if s.b[2992] { 1.0 } else { 0.0 });

        if (s.b[2991] && s.b[2992]) {
            s.store_sub_scaled_inputs_ad_lhs(1992, A::add_scaled_inputs(A::sub(A::add_scaled_inputs4(s.ad_value(1972), (-336.0), s.ad_value(1973), 84.0, s.ad_value(1971), 90.0, s.ad_value(1979), 181.0), s.ad_value(1978)), 1.0, s.ad_value(1969), 6.0), 0.015384615384615385, 1970, (24.0 * 0.015384615384615385));
            s.store_sub_scaled_inputs_ad_lhs(1993, A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1978), 18.0, s.ad_value(1979), 3762.0, s.ad_value(1972), 6048.0, s.ad_value(1970), 432.0), 1.0, s.ad_value(1971), 1620.0), 1.0, s.ad_value(1969), 108.0), 0.015384615384615385, 1973, (8532.0 * 0.015384615384615385));
        }

        s.b[2993] = (s.v[1] == 9.0);
        s.store_scalar(2993, if s.b[2993] { 1.0 } else { 0.0 });

        if ((s.b[2991] && (!s.b[2992])) && s.b[2993]) {
            s.store_scaled_sub_ad(1992, A::add(A::add(A::add_scaled_inputs4(s.ad_value(1974), 1680.0, s.ad_value(1972), (-1680.0), s.ad_value(1979), 5.0, s.ad_value(1978), (-5.0)), A::sub_scaled_inputs(s.ad_value(1971), 450.0, s.ad_value(1975), 450.0)), A::sub_scaled_inputs(s.ad_value(1976), 120.0, s.ad_value(1970), 120.0)), A::sub_scaled_inputs(s.ad_value(1977), 30.0, s.ad_value(1969), 30.0), 0.004784688995215311);
            s.store_scaled_add_ad(1993, A::add(A::add(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1969), (-900.0), s.ad_value(1977), (-900.0), s.ad_value(1975), (-13500.0), s.ad_value(1971), (-13500.0)), 1.0, s.ad_value(1973), 79500.0), A::add_scaled_inputs(s.ad_value(1972), 50400.0, s.ad_value(1974), 50400.0)), A::add_scaled_inputs(s.ad_value(1970), 3600.0, s.ad_value(1976), 3600.0)), A::add_scaled_inputs(s.ad_value(1978), 150.0, s.ad_value(1979), 150.0), 0.0055248618784530384);
        }

        if ((s.b[2991] && (!s.b[2992])) && (!s.b[2993])) {
            s.store_scalar(1992, 0.0);
            s.store_scalar(1993, 0.0);
        }

        if s.b[2991] {
            s.store_add_div_lhs_indices(2027, 1973, 1937, 1890);
        }

        s.b[2994] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(2994, if s.b[2994] { 1.0 } else { 0.0 });

        if (s.b[2991] && s.b[2994]) {
            s.store_div(2016, 2027, 1940);
        }

        s.b[2995] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(2995, if s.b[2995] { 1.0 } else { 0.0 });

        if ((s.b[2991] && (!s.b[2994])) && s.b[2995]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[2996] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(2996, if s.b[2996] { 1.0 } else { 0.0 });

        if (((s.b[2991] && (!s.b[2994])) && s.b[2995]) && s.b[2996]) {
            s.store_exp(2005, 2015);
        }

        s.b[2997] = (s.v[2015] < 0.0);
        s.store_scalar(2997, if s.b[2997] { 1.0 } else { 0.0 });

        if ((((s.b[2991] && (!s.b[2994])) && s.b[2995]) && (!s.b[2996])) && s.b[2997]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[2991] && (!s.b[2994])) && s.b[2995]) && (!s.b[2996])) && (!s.b[2997])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[2991] && (!s.b[2994])) && s.b[2995]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2016, 2015, 2012);
        }

        if ((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[2998] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2998, if s.b[2998] { 1.0 } else { 0.0 });

        if (((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) && s.b[2998]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[2999] = ((-s.v[2011]) < 0.0);
        s.store_scalar(2999, if s.b[2999] { 1.0 } else { 0.0 });

        if ((((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) && (!s.b[2998])) && s.b[2999]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) && (!s.b[2998])) && (!s.b[2999])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[3000] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3000, if s.b[3000] { 1.0 } else { 0.0 });

        if (((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) && s.b[3000]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3001] = ((-s.v[2013]) < 0.0);
        s.store_scalar(3001, if s.b[3001] { 1.0 } else { 0.0 });

        if ((((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) && (!s.b[3000])) && s.b[3001]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) && (!s.b[3000])) && (!s.b[3001])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2016, 2013, 2014);
        }

        s.b[3002] = (((s.v[2016]) as f64).abs() <= s.v[1933]);
        s.store_scalar(3002, if s.b[3002] { 1.0 } else { 0.0 });

        if (s.b[2991] && s.b[3002]) {
            s.store_mul_ad_affine_product_rhs(1996, 2016, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(1991, 1889, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.25), 0.3333333333333333), (-0.70710678));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1990, 1889, 1.0, 2016, 0.5, (-0.235702));
        }

        s.b[3003] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3003, if s.b[3003] { 1.0 } else { 0.0 });

        if ((s.b[2991] && (!s.b[3002])) && s.b[3003]) {
            s.store_exp_neg_input(2027, 2016);
        }

        s.b[3004] = ((-s.v[2016]) < 0.0);
        s.store_scalar(3004, if s.b[3004] { 1.0 } else { 0.0 });

        if (((s.b[2991] && (!s.b[3002])) && (!s.b[3003])) && s.b[3004]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2016)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[2991] && (!s.b[3002])) && (!s.b[3003])) && (!s.b[3004])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(2016)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (s.b[2991] && (!s.b[3002])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));
        }

        s.b[3005] = (s.v[2016] > s.v[1933]);
        s.store_scalar(3005, if s.b[3005] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_28(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2991] && (!s.b[3002])) && s.b[3005]) {
            s.store_neg(1996, 1996);
        }

        if (s.b[2991] && (!s.b[3002])) {
            s.store_div_scaled_product3_mixed_iiai(1991, 1889, 1889, A::sub_from_scalar(1.0, s.ad_value(2027)), 0.5, 1996, 1.0);
            s.store_add_div_lhs_mixed_ai(1990, A::add_scaled_square_product(s.ad_value(1991), 1.0, s.ad_value(1889), s.ad_value(1889), (-0.5)), 1996, 1991);
        }

        if s.b[2991] {
            s.store_sub(1988, 1937, 1991);
            s.store_div_from_scalar(1989, 1.0, 1988);
            s.store_offset_mul(1987, 1973, 1989, (-1.0));
            s.store_mul_sub_from_scalar_ad_lhs(1986, 1.0, A::mul(A::mul3(s.ad_value(1973), s.ad_value(1990), s.ad_value(1989)), s.ad_value(1989)), 1989);
            s.store_add_scaled_product_value_ad(2017, A::mul3(s.ad_value(1986), s.ad_value(1992), s.ad_value(1992)), 1.0, 1987, 1993, 1.0);
            s.store_mul_ad_product_lhs_mixed_ai(1994, A::square(s.ad_value(1992)), 1989, 1989);
            s.store_mul(1985, 2018, 1994);
        }

        s.b[3006] = (s.v[0] == (-1.0));
        s.store_scalar(3006, if s.b[3006] { 1.0 } else { 0.0 });

        if (s.b[2991] && s.b[3006]) {
            s.store_div_scaled_value_offset_denominator(1985, s.ad_value(1985), 1.0, A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0, 1.0);
        }

        if s.b[2991] {
            s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);
            s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);
            s.store_add_scaled_product_indices(2028, 1993, 1.0, 1994, 1990, (-1.0));
            s.store_mul_sub_ad_rhs(1955, 2019, s.ad_value(2017), A::div_scaled_product(A::mul3(s.ad_value(1985), s.ad_value(1987), s.ad_value(2028)), s.ad_value(2019), 1.0, s.ad_value(2027), 1.0));
        }

        s.b[3007] = (s.v[1] >= 6.0);
        s.store_scalar(3007, if s.b[3007] { 1.0 } else { 0.0 });

        s.b[3008] = (s.v[1] == 9.0);
        s.store_scalar(3008, if s.b[3008] { 1.0 } else { 0.0 });

        if (s.b[3007] && s.b[3008]) {
            s.store_sub_scaled_ad_lhs(1992, A::sub(A::add_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1974), 30.0, s.ad_value(1972), 81480.0, s.ad_value(1971), (-21825.0), s.ad_value(1976), (-81060.0)), 1.0, s.ad_value(1977), 20265.0), 1.0, s.ad_value(1969), 1455.0), 1.0, s.ad_value(1975), 303975.0), 2.6434745829918846e-5, s.ad_value(1970), (5820.0 * 2.6434745829918846e-5)), A::sub_scaled_inputs(s.ad_value(1979), (6755.0 * 1.3217372914959423e-5), s.ad_value(1978), (485.0 * 1.3217372914959423e-5))), 1973, (1455.0 / 181.0));
            s.store_add_scaled_ad_lhs(1993, A::sub_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1969), 50400.0, s.ad_value(1975), 10530000.0, s.ad_value(1972), (-2822400.0), s.ad_value(1971), 756000.0), 1.0, s.ad_value(1970), 201600.0), 1.0, s.ad_value(1978), 8400.0), 1.0, s.ad_value(1979), 117000.0), 1.0, s.ad_value(1976), 2808000.0), 1.0, s.ad_value(1977), 702000.0), 2.6434745829918846e-5, s.ad_value(1974), (16614600.0 * 2.6434745829918846e-5)), 1973, (50400.0 * 0.0055248618784530384));
        }

        if (s.b[3007] && (!s.b[3008])) {
            s.store_scalar(1992, 0.0);
            s.store_scalar(1993, 0.0);
        }

        if s.b[3007] {
            s.store_add_div_lhs_indices(2027, 1974, 1937, 1890);
        }

        s.b[3009] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(3009, if s.b[3009] { 1.0 } else { 0.0 });

        if (s.b[3007] && s.b[3009]) {
            s.store_div(2016, 2027, 1940);
        }

        s.b[3010] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(3010, if s.b[3010] { 1.0 } else { 0.0 });

        if ((s.b[3007] && (!s.b[3009])) && s.b[3010]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3011] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(3011, if s.b[3011] { 1.0 } else { 0.0 });

        if (((s.b[3007] && (!s.b[3009])) && s.b[3010]) && s.b[3011]) {
            s.store_exp(2005, 2015);
        }

        s.b[3012] = (s.v[2015] < 0.0);
        s.store_scalar(3012, if s.b[3012] { 1.0 } else { 0.0 });

        if ((((s.b[3007] && (!s.b[3009])) && s.b[3010]) && (!s.b[3011])) && s.b[3012]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[3007] && (!s.b[3009])) && s.b[3010]) && (!s.b[3011])) && (!s.b[3012])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[3007] && (!s.b[3009])) && s.b[3010]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2016, 2015, 2012);
        }

        if ((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[3013] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3013, if s.b[3013] { 1.0 } else { 0.0 });

        if (((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) && s.b[3013]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3014] = ((-s.v[2011]) < 0.0);
        s.store_scalar(3014, if s.b[3014] { 1.0 } else { 0.0 });

        if ((((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) && (!s.b[3013])) && s.b[3014]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) && (!s.b[3013])) && (!s.b[3014])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[3015] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3015, if s.b[3015] { 1.0 } else { 0.0 });

        if (((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) && s.b[3015]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3016] = ((-s.v[2013]) < 0.0);
        s.store_scalar(3016, if s.b[3016] { 1.0 } else { 0.0 });

        if ((((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) && (!s.b[3015])) && s.b[3016]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) && (!s.b[3015])) && (!s.b[3016])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2016, 2013, 2014);
        }

        s.b[3017] = (((s.v[2016]) as f64).abs() <= s.v[1933]);
        s.store_scalar(3017, if s.b[3017] { 1.0 } else { 0.0 });

        if (s.b[3007] && s.b[3017]) {
            s.store_mul_ad_affine_product_rhs(1996, 2016, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(1991, 1889, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.25), 0.3333333333333333), (-0.70710678));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1990, 1889, 1.0, 2016, 0.5, (-0.235702));
        }

        s.b[3018] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3018, if s.b[3018] { 1.0 } else { 0.0 });

        if ((s.b[3007] && (!s.b[3017])) && s.b[3018]) {
            s.store_exp_neg_input(2027, 2016);
        }

        s.b[3019] = ((-s.v[2016]) < 0.0);
        s.store_scalar(3019, if s.b[3019] { 1.0 } else { 0.0 });

        if (((s.b[3007] && (!s.b[3017])) && (!s.b[3018])) && s.b[3019]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2016)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[3007] && (!s.b[3017])) && (!s.b[3018])) && (!s.b[3019])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(2016)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (s.b[3007] && (!s.b[3017])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));
        }

        s.b[3020] = (s.v[2016] > s.v[1933]);
        s.store_scalar(3020, if s.b[3020] { 1.0 } else { 0.0 });

        if ((s.b[3007] && (!s.b[3017])) && s.b[3020]) {
            s.store_neg(1996, 1996);
        }

        if (s.b[3007] && (!s.b[3017])) {
            s.store_div_scaled_product3_mixed_iiai(1991, 1889, 1889, A::sub_from_scalar(1.0, s.ad_value(2027)), 0.5, 1996, 1.0);
            s.store_add_div_lhs_mixed_ai(1990, A::add_scaled_square_product(s.ad_value(1991), 1.0, s.ad_value(1889), s.ad_value(1889), (-0.5)), 1996, 1991);
        }

        if s.b[3007] {
            s.store_sub(1988, 1937, 1991);
            s.store_div_from_scalar(1989, 1.0, 1988);
            s.store_offset_mul(1987, 1974, 1989, (-1.0));
            s.store_mul_sub_from_scalar_ad_lhs(1986, 1.0, A::mul(A::mul3(s.ad_value(1974), s.ad_value(1990), s.ad_value(1989)), s.ad_value(1989)), 1989);
            s.store_add_scaled_product_value_ad(2017, A::mul3(s.ad_value(1986), s.ad_value(1992), s.ad_value(1992)), 1.0, 1987, 1993, 1.0);
            s.store_mul_ad_product_lhs_mixed_ai(1994, A::square(s.ad_value(1992)), 1989, 1989);
            s.store_mul(1985, 2018, 1994);
        }

        s.b[3021] = (s.v[0] == (-1.0));
        s.store_scalar(3021, if s.b[3021] { 1.0 } else { 0.0 });

        if (s.b[3007] && s.b[3021]) {
            s.store_div_scaled_value_offset_denominator(1985, s.ad_value(1985), 1.0, A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0, 1.0);
        }

        if s.b[3007] {
            s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);
            s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);
            s.store_add_scaled_product_indices(2028, 1993, 1.0, 1994, 1990, (-1.0));
            s.store_mul_sub_ad_rhs(1956, 2019, s.ad_value(2017), A::div_scaled_product(A::mul3(s.ad_value(1985), s.ad_value(1987), s.ad_value(2028)), s.ad_value(2019), 1.0, s.ad_value(2027), 1.0));
        }

        s.b[3022] = (s.v[1] >= 7.0);
        s.store_scalar(3022, if s.b[3022] { 1.0 } else { 0.0 });

        s.b[3023] = (s.v[1] == 9.0);
        s.store_scalar(3023, if s.b[3023] { 1.0 } else { 0.0 });

        if (s.b[3022] && s.b[3023]) {
            s.store_add_scaled_ad_lhs(1992, A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1974), (-304200.0), s.ad_value(1972), (-21840.0), s.ad_value(1979), 12605.0, s.ad_value(1971), 5850.0), 1.0, s.ad_value(1976), 302520.0), 1.0, s.ad_value(1978), 65.0), 1.0, s.ad_value(1977), 75630.0), 1.0, s.ad_value(1969), 390.0), 1.0, s.ad_value(1975), 420.0), 2.6434745829918846e-5, s.ad_value(1970), (1560.0 * 2.6434745829918846e-5)), 1973, (390.0 / 181.0));
            s.store_sub_scaled_ad_lhs(1993, A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1969), (-13500.0), s.ad_value(1975), (-16601100.0), s.ad_value(1972), 756000.0, s.ad_value(1971), (-202500.0)), 1.0, s.ad_value(1970), 54000.0), 1.0, s.ad_value(1978), 2250.0), 1.0, s.ad_value(1979), 436650.0), 1.0, s.ad_value(1976), 10479600.0), 1.0, s.ad_value(1977), 2619900.0), 2.6434745829918846e-5, s.ad_value(1974), (10530000.0 * 2.6434745829918846e-5)), 1973, (13500.0 * 0.0055248618784530384));
        }

        if (s.b[3022] && (!s.b[3023])) {
            s.store_scalar(1992, 0.0);
            s.store_scalar(1993, 0.0);
        }

        if s.b[3022] {
            s.store_add_div_lhs_indices(2027, 1975, 1937, 1890);
        }

        s.b[3024] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(3024, if s.b[3024] { 1.0 } else { 0.0 });

        if (s.b[3022] && s.b[3024]) {
            s.store_div(2016, 2027, 1940);
        }

        s.b[3025] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(3025, if s.b[3025] { 1.0 } else { 0.0 });

        if ((s.b[3022] && (!s.b[3024])) && s.b[3025]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3026] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(3026, if s.b[3026] { 1.0 } else { 0.0 });

        if (((s.b[3022] && (!s.b[3024])) && s.b[3025]) && s.b[3026]) {
            s.store_exp(2005, 2015);
        }

        s.b[3027] = (s.v[2015] < 0.0);
        s.store_scalar(3027, if s.b[3027] { 1.0 } else { 0.0 });

        if ((((s.b[3022] && (!s.b[3024])) && s.b[3025]) && (!s.b[3026])) && s.b[3027]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[3022] && (!s.b[3024])) && s.b[3025]) && (!s.b[3026])) && (!s.b[3027])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[3022] && (!s.b[3024])) && s.b[3025]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2016, 2015, 2012);
        }

        if ((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[3028] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3028, if s.b[3028] { 1.0 } else { 0.0 });

        if (((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) && s.b[3028]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3029] = ((-s.v[2011]) < 0.0);
        s.store_scalar(3029, if s.b[3029] { 1.0 } else { 0.0 });

        if ((((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) && (!s.b[3028])) && s.b[3029]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) && (!s.b[3028])) && (!s.b[3029])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

    }

    pub(super) fn stamp_reactive_block_29(
        s: &mut ReactiveScratch,
    ) {
        s.b[3030] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3030, if s.b[3030] { 1.0 } else { 0.0 });

        if (((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) && s.b[3030]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3031] = ((-s.v[2013]) < 0.0);
        s.store_scalar(3031, if s.b[3031] { 1.0 } else { 0.0 });

        if ((((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) && (!s.b[3030])) && s.b[3031]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) && (!s.b[3030])) && (!s.b[3031])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2016, 2013, 2014);
        }

        s.b[3032] = (((s.v[2016]) as f64).abs() <= s.v[1933]);
        s.store_scalar(3032, if s.b[3032] { 1.0 } else { 0.0 });

        if (s.b[3022] && s.b[3032]) {
            s.store_mul_ad_affine_product_rhs(1996, 2016, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(1991, 1889, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.25), 0.3333333333333333), (-0.70710678));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1990, 1889, 1.0, 2016, 0.5, (-0.235702));
        }

        s.b[3033] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3033, if s.b[3033] { 1.0 } else { 0.0 });

        if ((s.b[3022] && (!s.b[3032])) && s.b[3033]) {
            s.store_exp_neg_input(2027, 2016);
        }

        s.b[3034] = ((-s.v[2016]) < 0.0);
        s.store_scalar(3034, if s.b[3034] { 1.0 } else { 0.0 });

        if (((s.b[3022] && (!s.b[3032])) && (!s.b[3033])) && s.b[3034]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2016)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[3022] && (!s.b[3032])) && (!s.b[3033])) && (!s.b[3034])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(2016)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (s.b[3022] && (!s.b[3032])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));
        }

        s.b[3035] = (s.v[2016] > s.v[1933]);
        s.store_scalar(3035, if s.b[3035] { 1.0 } else { 0.0 });

        if ((s.b[3022] && (!s.b[3032])) && s.b[3035]) {
            s.store_neg(1996, 1996);
        }

        if (s.b[3022] && (!s.b[3032])) {
            s.store_div_scaled_product3_mixed_iiai(1991, 1889, 1889, A::sub_from_scalar(1.0, s.ad_value(2027)), 0.5, 1996, 1.0);
            s.store_add_div_lhs_mixed_ai(1990, A::add_scaled_square_product(s.ad_value(1991), 1.0, s.ad_value(1889), s.ad_value(1889), (-0.5)), 1996, 1991);
        }

        if s.b[3022] {
            s.store_sub(1988, 1937, 1991);
            s.store_div_from_scalar(1989, 1.0, 1988);
            s.store_offset_mul(1987, 1975, 1989, (-1.0));
            s.store_mul_sub_from_scalar_ad_lhs(1986, 1.0, A::mul(A::mul3(s.ad_value(1975), s.ad_value(1990), s.ad_value(1989)), s.ad_value(1989)), 1989);
            s.store_add_scaled_product_value_ad(2017, A::mul3(s.ad_value(1986), s.ad_value(1992), s.ad_value(1992)), 1.0, 1987, 1993, 1.0);
            s.store_mul_ad_product_lhs_mixed_ai(1994, A::square(s.ad_value(1992)), 1989, 1989);
            s.store_mul(1985, 2018, 1994);
        }

        s.b[3036] = (s.v[0] == (-1.0));
        s.store_scalar(3036, if s.b[3036] { 1.0 } else { 0.0 });

        if (s.b[3022] && s.b[3036]) {
            s.store_div_scaled_value_offset_denominator(1985, s.ad_value(1985), 1.0, A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0, 1.0);
        }

        if s.b[3022] {
            s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);
            s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);
            s.store_add_scaled_product_indices(2028, 1993, 1.0, 1994, 1990, (-1.0));
            s.store_mul_sub_ad_rhs(1957, 2019, s.ad_value(2017), A::div_scaled_product(A::mul3(s.ad_value(1985), s.ad_value(1987), s.ad_value(2028)), s.ad_value(2019), 1.0, s.ad_value(2027), 1.0));
        }

        s.b[3037] = (s.v[1] >= 8.0);
        s.store_scalar(3037, if s.b[3037] { 1.0 } else { 0.0 });

        s.b[3038] = (s.v[1] == 9.0);
        s.store_scalar(3038, if s.b[3038] { 1.0 } else { 0.0 });

        if (s.b[3037] && s.b[3038]) {
            s.store_sub_scaled_ad_lhs(1992, A::add(A::add_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1974), 81900.0, s.ad_value(1972), 5880.0, s.ad_value(1971), (-1575.0), s.ad_value(1976), 5850.0), 1.0, s.ad_value(1977), 282255.0), 1.0, s.ad_value(1969), 105.0), 1.0, s.ad_value(1975), 305655.0), 2.6434745829918846e-5, s.ad_value(1970), (420.0 * 2.6434745829918846e-5)), A::sub_scaled_inputs(s.ad_value(1978), (35.0 * 1.3217372914959423e-5), s.ad_value(1979), (94085.0 * 1.3217372914959423e-5))), 1973, (105.0 / 181.0));
            s.store_add_scaled_ad_lhs(1993, A::sub_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1969), 3600.0, s.ad_value(1975), 10479600.0, s.ad_value(1972), (-201600.0), s.ad_value(1971), 54000.0), 1.0, s.ad_value(1970), 14400.0), 1.0, s.ad_value(1978), 600.0), 1.0, s.ad_value(1979), 1629600.0), 1.0, s.ad_value(1976), 16413000.0), 1.0, s.ad_value(1977), 9777600.0), 2.6434745829918846e-5, s.ad_value(1974), (2808000.0 * 2.6434745829918846e-5)), 1973, (3600.0 * 0.0055248618784530384));
        }

        if (s.b[3037] && (!s.b[3038])) {
            s.store_scalar(1992, 0.0);
            s.store_scalar(1993, 0.0);
        }

        if s.b[3037] {
            s.store_add_div_lhs_indices(2027, 1976, 1937, 1890);
        }

        s.b[3039] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(3039, if s.b[3039] { 1.0 } else { 0.0 });

        if (s.b[3037] && s.b[3039]) {
            s.store_div(2016, 2027, 1940);
        }

        s.b[3040] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(3040, if s.b[3040] { 1.0 } else { 0.0 });

        if ((s.b[3037] && (!s.b[3039])) && s.b[3040]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3041] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(3041, if s.b[3041] { 1.0 } else { 0.0 });

        if (((s.b[3037] && (!s.b[3039])) && s.b[3040]) && s.b[3041]) {
            s.store_exp(2005, 2015);
        }

        s.b[3042] = (s.v[2015] < 0.0);
        s.store_scalar(3042, if s.b[3042] { 1.0 } else { 0.0 });

        if ((((s.b[3037] && (!s.b[3039])) && s.b[3040]) && (!s.b[3041])) && s.b[3042]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[3037] && (!s.b[3039])) && s.b[3040]) && (!s.b[3041])) && (!s.b[3042])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[3037] && (!s.b[3039])) && s.b[3040]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2016, 2015, 2012);
        }

        if ((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[3043] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3043, if s.b[3043] { 1.0 } else { 0.0 });

        if (((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) && s.b[3043]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3044] = ((-s.v[2011]) < 0.0);
        s.store_scalar(3044, if s.b[3044] { 1.0 } else { 0.0 });

        if ((((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) && (!s.b[3043])) && s.b[3044]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) && (!s.b[3043])) && (!s.b[3044])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[3045] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3045, if s.b[3045] { 1.0 } else { 0.0 });

        if (((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) && s.b[3045]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3046] = ((-s.v[2013]) < 0.0);
        s.store_scalar(3046, if s.b[3046] { 1.0 } else { 0.0 });

        if ((((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) && (!s.b[3045])) && s.b[3046]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) && (!s.b[3045])) && (!s.b[3046])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2016, 2013, 2014);
        }

        s.b[3047] = (((s.v[2016]) as f64).abs() <= s.v[1933]);
        s.store_scalar(3047, if s.b[3047] { 1.0 } else { 0.0 });

        if (s.b[3037] && s.b[3047]) {
            s.store_mul_ad_affine_product_rhs(1996, 2016, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(1991, 1889, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.25), 0.3333333333333333), (-0.70710678));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1990, 1889, 1.0, 2016, 0.5, (-0.235702));
        }

        s.b[3048] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3048, if s.b[3048] { 1.0 } else { 0.0 });

        if ((s.b[3037] && (!s.b[3047])) && s.b[3048]) {
            s.store_exp_neg_input(2027, 2016);
        }

        s.b[3049] = ((-s.v[2016]) < 0.0);
        s.store_scalar(3049, if s.b[3049] { 1.0 } else { 0.0 });

        if (((s.b[3037] && (!s.b[3047])) && (!s.b[3048])) && s.b[3049]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2016)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[3037] && (!s.b[3047])) && (!s.b[3048])) && (!s.b[3049])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(2016)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (s.b[3037] && (!s.b[3047])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));
        }

        s.b[3050] = (s.v[2016] > s.v[1933]);
        s.store_scalar(3050, if s.b[3050] { 1.0 } else { 0.0 });

        if ((s.b[3037] && (!s.b[3047])) && s.b[3050]) {
            s.store_neg(1996, 1996);
        }

        if (s.b[3037] && (!s.b[3047])) {
            s.store_div_scaled_product3_mixed_iiai(1991, 1889, 1889, A::sub_from_scalar(1.0, s.ad_value(2027)), 0.5, 1996, 1.0);
            s.store_add_div_lhs_mixed_ai(1990, A::add_scaled_square_product(s.ad_value(1991), 1.0, s.ad_value(1889), s.ad_value(1889), (-0.5)), 1996, 1991);
        }

        if s.b[3037] {
            s.store_sub(1988, 1937, 1991);
            s.store_div_from_scalar(1989, 1.0, 1988);
            s.store_offset_mul(1987, 1976, 1989, (-1.0));
            s.store_mul_sub_from_scalar_ad_lhs(1986, 1.0, A::mul(A::mul3(s.ad_value(1976), s.ad_value(1990), s.ad_value(1989)), s.ad_value(1989)), 1989);
            s.store_add_scaled_product_value_ad(2017, A::mul3(s.ad_value(1986), s.ad_value(1992), s.ad_value(1992)), 1.0, 1987, 1993, 1.0);
            s.store_mul_ad_product_lhs_mixed_ai(1994, A::square(s.ad_value(1992)), 1989, 1989);
            s.store_mul(1985, 2018, 1994);
        }

        s.b[3051] = (s.v[0] == (-1.0));
        s.store_scalar(3051, if s.b[3051] { 1.0 } else { 0.0 });

        if (s.b[3037] && s.b[3051]) {
            s.store_div_scaled_value_offset_denominator(1985, s.ad_value(1985), 1.0, A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0, 1.0);
        }

        if s.b[3037] {
            s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);
            s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);
            s.store_add_scaled_product_indices(2028, 1993, 1.0, 1994, 1990, (-1.0));
            s.store_mul_sub_ad_rhs(1958, 2019, s.ad_value(2017), A::div_scaled_product(A::mul3(s.ad_value(1985), s.ad_value(1987), s.ad_value(2028)), s.ad_value(2019), 1.0, s.ad_value(2027), 1.0));
        }

        s.b[3052] = (s.v[1] >= 9.0);
        s.store_scalar(3052, if s.b[3052] { 1.0 } else { 0.0 });

        s.b[3053] = (s.v[1] == 9.0);
        s.store_scalar(3053, if s.b[3053] { 1.0 } else { 0.0 });

        if (s.b[3052] && s.b[3053]) {
            s.store_add_scaled_ad_lhs(1992, A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1974), (-23400.0), s.ad_value(1972), (-1680.0), s.ad_value(1979), 175565.0, s.ad_value(1971), 450.0), 1.0, s.ad_value(1976), 325920.0), 1.0, s.ad_value(1978), 5.0), 1.0, s.ad_value(1977), 81480.0), 1.0, s.ad_value(1969), 30.0), 1.0, s.ad_value(1975), 87330.0), 2.6434745829918846e-5, s.ad_value(1970), (120.0 * 2.6434745829918846e-5)), 1973, (30.0 * 0.0055248618784530384));
            s.store_sub_scaled_ad_lhs(1993, A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1969), (-900.0), s.ad_value(1975), (-2619900.0), s.ad_value(1972), 50400.0, s.ad_value(1971), (-13500.0)), 1.0, s.ad_value(1970), 3600.0), 1.0, s.ad_value(1978), 150.0), 1.0, s.ad_value(1979), 6081750.0), 1.0, s.ad_value(1976), 9777600.0), 1.0, s.ad_value(1977), 13793100.0), 2.6434745829918846e-5, s.ad_value(1974), (702000.0 * 2.6434745829918846e-5)), 1973, (900.0 * 0.0055248618784530384));
        }

        if (s.b[3052] && (!s.b[3053])) {
            s.store_scalar(1992, 0.0);
            s.store_scalar(1993, 0.0);
        }

        if s.b[3052] {
            s.store_add_div_lhs_indices(2027, 1977, 1937, 1890);
        }

        s.b[3054] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(3054, if s.b[3054] { 1.0 } else { 0.0 });

        if (s.b[3052] && s.b[3054]) {
            s.store_div(2016, 2027, 1940);
        }

        s.b[3055] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(3055, if s.b[3055] { 1.0 } else { 0.0 });

        if ((s.b[3052] && (!s.b[3054])) && s.b[3055]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_30(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[3052] && (!s.b[3054])) && s.b[3055]) {
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3056] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(3056, if s.b[3056] { 1.0 } else { 0.0 });

        if (((s.b[3052] && (!s.b[3054])) && s.b[3055]) && s.b[3056]) {
            s.store_exp(2005, 2015);
        }

        s.b[3057] = (s.v[2015] < 0.0);
        s.store_scalar(3057, if s.b[3057] { 1.0 } else { 0.0 });

        if ((((s.b[3052] && (!s.b[3054])) && s.b[3055]) && (!s.b[3056])) && s.b[3057]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[3052] && (!s.b[3054])) && s.b[3055]) && (!s.b[3056])) && (!s.b[3057])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[3052] && (!s.b[3054])) && s.b[3055]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2016, 2015, 2012);
        }

        if ((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[3058] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3058, if s.b[3058] { 1.0 } else { 0.0 });

        if (((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) && s.b[3058]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3059] = ((-s.v[2011]) < 0.0);
        s.store_scalar(3059, if s.b[3059] { 1.0 } else { 0.0 });

        if ((((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) && (!s.b[3058])) && s.b[3059]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) && (!s.b[3058])) && (!s.b[3059])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[3060] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3060, if s.b[3060] { 1.0 } else { 0.0 });

        if (((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) && s.b[3060]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3061] = ((-s.v[2013]) < 0.0);
        s.store_scalar(3061, if s.b[3061] { 1.0 } else { 0.0 });

        if ((((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) && (!s.b[3060])) && s.b[3061]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) && (!s.b[3060])) && (!s.b[3061])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2016, 2013, 2014);
        }

        s.b[3062] = (((s.v[2016]) as f64).abs() <= s.v[1933]);
        s.store_scalar(3062, if s.b[3062] { 1.0 } else { 0.0 });

        if (s.b[3052] && s.b[3062]) {
            s.store_mul_ad_affine_product_rhs(1996, 2016, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(1991, 1889, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.25), 0.3333333333333333), (-0.70710678));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1990, 1889, 1.0, 2016, 0.5, (-0.235702));
        }

        s.b[3063] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3063, if s.b[3063] { 1.0 } else { 0.0 });

        if ((s.b[3052] && (!s.b[3062])) && s.b[3063]) {
            s.store_exp_neg_input(2027, 2016);
        }

        s.b[3064] = ((-s.v[2016]) < 0.0);
        s.store_scalar(3064, if s.b[3064] { 1.0 } else { 0.0 });

        if (((s.b[3052] && (!s.b[3062])) && (!s.b[3063])) && s.b[3064]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2016)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[3052] && (!s.b[3062])) && (!s.b[3063])) && (!s.b[3064])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(2016)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (s.b[3052] && (!s.b[3062])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));
        }

        s.b[3065] = (s.v[2016] > s.v[1933]);
        s.store_scalar(3065, if s.b[3065] { 1.0 } else { 0.0 });

        if ((s.b[3052] && (!s.b[3062])) && s.b[3065]) {
            s.store_neg(1996, 1996);
        }

        if (s.b[3052] && (!s.b[3062])) {
            s.store_div_scaled_product3_mixed_iiai(1991, 1889, 1889, A::sub_from_scalar(1.0, s.ad_value(2027)), 0.5, 1996, 1.0);
            s.store_add_div_lhs_mixed_ai(1990, A::add_scaled_square_product(s.ad_value(1991), 1.0, s.ad_value(1889), s.ad_value(1889), (-0.5)), 1996, 1991);
        }

        if s.b[3052] {
            s.store_sub(1988, 1937, 1991);
            s.store_div_from_scalar(1989, 1.0, 1988);
            s.store_offset_mul(1987, 1977, 1989, (-1.0));
            s.store_mul_sub_from_scalar_ad_lhs(1986, 1.0, A::mul(A::mul3(s.ad_value(1977), s.ad_value(1990), s.ad_value(1989)), s.ad_value(1989)), 1989);
            s.store_add_scaled_product_value_ad(2017, A::mul3(s.ad_value(1986), s.ad_value(1992), s.ad_value(1992)), 1.0, 1987, 1993, 1.0);
            s.store_mul_ad_product_lhs_mixed_ai(1994, A::square(s.ad_value(1992)), 1989, 1989);
            s.store_mul(1985, 2018, 1994);
        }

        s.b[3066] = (s.v[0] == (-1.0));
        s.store_scalar(3066, if s.b[3066] { 1.0 } else { 0.0 });

        if (s.b[3052] && s.b[3066]) {
            s.store_div_scaled_value_offset_denominator(1985, s.ad_value(1985), 1.0, A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0, 1.0);
        }

        if s.b[3052] {
            s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);
            s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);
            s.store_add_scaled_product_indices(2028, 1993, 1.0, 1994, 1990, (-1.0));
            s.store_mul_sub_ad_rhs(1959, 2019, s.ad_value(2017), A::div_scaled_product(A::mul3(s.ad_value(1985), s.ad_value(1987), s.ad_value(2028)), s.ad_value(2019), 1.0, s.ad_value(2027), 1.0));
        }

        s.store_scalar(1981, 0.0);

        s.store_scalar(1982, 0.0);

        s.store_scalar(1980, 0.0);

        s.b[3067] = (s.v[1] != 0.0);
        s.store_scalar(3067, if s.b[3067] { 1.0 } else { 0.0 });

        s.b[3068] = (s.v[1] == 1.0);
        s.store_scalar(3068, if s.b[3068] { 1.0 } else { 0.0 });

        if (s.b[3067] && s.b[3068]) {
            s.store_add_scaled_inputs3_indices(1981, 1978, (17.0 * 0.010416666666666666), 1969, (30.0 * 0.010416666666666666), 1979, 0.010416666666666666);
            s.store_add_scaled_inputs3_indices(1982, 1978, 0.010416666666666666, 1969, (30.0 * 0.010416666666666666), 1979, (17.0 * 0.010416666666666666));
            s.store_add_div_lhs_indices(2027, 1969, 1937, 1890);
        }

        s.b[3069] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(3069, if s.b[3069] { 1.0 } else { 0.0 });

        if ((s.b[3067] && s.b[3068]) && s.b[3069]) {
            s.store_div(2028, 2027, 1940);
        }

        s.b[3070] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(3070, if s.b[3070] { 1.0 } else { 0.0 });

        if (((s.b[3067] && s.b[3068]) && (!s.b[3069])) && s.b[3070]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3071] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(3071, if s.b[3071] { 1.0 } else { 0.0 });

        if ((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && s.b[3070]) && s.b[3071]) {
            s.store_exp(2005, 2015);
        }

        s.b[3072] = (s.v[2015] < 0.0);
        s.store_scalar(3072, if s.b[3072] { 1.0 } else { 0.0 });

        if (((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && s.b[3070]) && (!s.b[3071])) && s.b[3072]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && s.b[3070]) && (!s.b[3071])) && (!s.b[3072])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[3067] && s.b[3068]) && (!s.b[3069])) && s.b[3070]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2028, 2015, 2012);
        }

        if (((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[3073] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3073, if s.b[3073] { 1.0 } else { 0.0 });

        if ((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) && s.b[3073]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3074] = ((-s.v[2011]) < 0.0);
        s.store_scalar(3074, if s.b[3074] { 1.0 } else { 0.0 });

        if (((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) && (!s.b[3073])) && s.b[3074]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) && (!s.b[3073])) && (!s.b[3074])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[3075] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3075, if s.b[3075] { 1.0 } else { 0.0 });

        if ((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) && s.b[3075]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3076] = ((-s.v[2013]) < 0.0);
        s.store_scalar(3076, if s.b[3076] { 1.0 } else { 0.0 });

        if (((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) && (!s.b[3075])) && s.b[3076]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) && (!s.b[3075])) && (!s.b[3076])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2028, 2013, 2014);
        }

        if (s.b[3067] && s.b[3068]) {
            s.store_add_scaled_inputs4_indices(1980, 1890, 1.0, 1983, (-0.16666666666666666), 2028, (-(4.0 * 0.16666666666666666)), 1984, (-0.16666666666666666));
        }

        s.b[3077] = (s.v[1] == 2.0);
        s.store_scalar(3077, if s.b[3077] { 1.0 } else { 0.0 });

        if ((s.b[3067] && (!s.b[3068])) && s.b[3077]) {
            s.store_add_scaled_inputs4_indices(1981, 1978, (11.0 * 0.011111111111111112), 1969, (24.0 * 0.011111111111111112), 1970, (9.0 * 0.011111111111111112), 1979, 0.011111111111111112);
            s.store_add_scaled_inputs4_indices(1982, 1979, (11.0 * 0.011111111111111112), 1970, (24.0 * 0.011111111111111112), 1969, (9.0 * 0.011111111111111112), 1978, 0.011111111111111112);
            s.store_add_div_lhs_indices(2027, 1969, 1937, 1890);
        }

        s.b[3078] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(3078, if s.b[3078] { 1.0 } else { 0.0 });

        if (((s.b[3067] && (!s.b[3068])) && s.b[3077]) && s.b[3078]) {
            s.store_div(2028, 2027, 1940);
        }

        s.b[3079] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(3079, if s.b[3079] { 1.0 } else { 0.0 });

        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && s.b[3079]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3080] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(3080, if s.b[3080] { 1.0 } else { 0.0 });

        if (((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && s.b[3079]) && s.b[3080]) {
            s.store_exp(2005, 2015);
        }

        s.b[3081] = (s.v[2015] < 0.0);
        s.store_scalar(3081, if s.b[3081] { 1.0 } else { 0.0 });

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && s.b[3079]) && (!s.b[3080])) && s.b[3081]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && s.b[3079]) && (!s.b[3080])) && (!s.b[3081])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && s.b[3079]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_31(
        s: &mut ReactiveScratch,
    ) {
        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && s.b[3079]) {
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2028, 2015, 2012);
        }

        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && (!s.b[3079])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[3082] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3082, if s.b[3082] { 1.0 } else { 0.0 });

        if (((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && (!s.b[3079])) && s.b[3082]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3083] = ((-s.v[2011]) < 0.0);
        s.store_scalar(3083, if s.b[3083] { 1.0 } else { 0.0 });

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && (!s.b[3079])) && (!s.b[3082])) && s.b[3083]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && (!s.b[3079])) && (!s.b[3082])) && (!s.b[3083])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && (!s.b[3079])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[3084] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3084, if s.b[3084] { 1.0 } else { 0.0 });

        if (((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && (!s.b[3079])) && s.b[3084]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3085] = ((-s.v[2013]) < 0.0);
        s.store_scalar(3085, if s.b[3085] { 1.0 } else { 0.0 });

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && (!s.b[3079])) && (!s.b[3084])) && s.b[3085]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && (!s.b[3079])) && (!s.b[3084])) && (!s.b[3085])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && (!s.b[3079])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2028, 2013, 2014);
        }

        if ((s.b[3067] && (!s.b[3068])) && s.b[3077]) {
            s.store_add_div_lhs_indices(2027, 1970, 1937, 1890);
        }

        s.b[3086] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(3086, if s.b[3086] { 1.0 } else { 0.0 });

        if (((s.b[3067] && (!s.b[3068])) && s.b[3077]) && s.b[3086]) {
            s.store_div(2029, 2027, 1940);
        }

        s.b[3087] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(3087, if s.b[3087] { 1.0 } else { 0.0 });

        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && s.b[3087]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3088] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(3088, if s.b[3088] { 1.0 } else { 0.0 });

        if (((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && s.b[3087]) && s.b[3088]) {
            s.store_exp(2005, 2015);
        }

        s.b[3089] = (s.v[2015] < 0.0);
        s.store_scalar(3089, if s.b[3089] { 1.0 } else { 0.0 });

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && s.b[3087]) && (!s.b[3088])) && s.b[3089]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && s.b[3087]) && (!s.b[3088])) && (!s.b[3089])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && s.b[3087]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2029, 2015, 2012);
        }

        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && (!s.b[3087])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[3090] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3090, if s.b[3090] { 1.0 } else { 0.0 });

        if (((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && (!s.b[3087])) && s.b[3090]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3091] = ((-s.v[2011]) < 0.0);
        s.store_scalar(3091, if s.b[3091] { 1.0 } else { 0.0 });

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && (!s.b[3087])) && (!s.b[3090])) && s.b[3091]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && (!s.b[3087])) && (!s.b[3090])) && (!s.b[3091])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && (!s.b[3087])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[3092] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3092, if s.b[3092] { 1.0 } else { 0.0 });

        if (((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && (!s.b[3087])) && s.b[3092]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3093] = ((-s.v[2013]) < 0.0);
        s.store_scalar(3093, if s.b[3093] { 1.0 } else { 0.0 });

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && (!s.b[3087])) && (!s.b[3092])) && s.b[3093]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && (!s.b[3087])) && (!s.b[3092])) && (!s.b[3093])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && (!s.b[3087])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2029, 2013, 2014);
        }

        if ((s.b[3067] && (!s.b[3068])) && s.b[3077]) {
            s.store_sub_ad_rhs(1980, 1890, A::add_scaled_inputs4(s.ad_value(1983), 0.125, s.ad_value(2028), (3.0 * 0.125), s.ad_value(2029), (3.0 * 0.125), s.ad_value(1984), 0.125));
        }

        s.b[3094] = (s.v[1] == 3.0);
        s.store_scalar(3094, if s.b[3094] { 1.0 } else { 0.0 });

        if (((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) {
            s.store_add_scaled_inputs_ad_lhs(1981, A::add_scaled_inputs4(s.ad_value(1978), 251.0, s.ad_value(1969), 594.0, s.ad_value(1970), 312.0, s.ad_value(1971), 174.0), 0.0003720238095238095, 1979, (13.0 * 0.0003720238095238095));
            s.store_add_scaled_inputs_ad_lhs(1982, A::add_scaled_inputs4(s.ad_value(1979), 251.0, s.ad_value(1971), 594.0, s.ad_value(1970), 312.0, s.ad_value(1969), 174.0), 0.0003720238095238095, 1978, (13.0 * 0.0003720238095238095));
            s.store_add_div_lhs_indices(2027, 1969, 1937, 1890);
        }

        s.b[3095] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(3095, if s.b[3095] { 1.0 } else { 0.0 });

        if ((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && s.b[3095]) {
            s.store_div(2028, 2027, 1940);
        }

        s.b[3096] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(3096, if s.b[3096] { 1.0 } else { 0.0 });

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && s.b[3096]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3097] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(3097, if s.b[3097] { 1.0 } else { 0.0 });

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && s.b[3096]) && s.b[3097]) {
            s.store_exp(2005, 2015);
        }

        s.b[3098] = (s.v[2015] < 0.0);
        s.store_scalar(3098, if s.b[3098] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && s.b[3096]) && (!s.b[3097])) && s.b[3098]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && s.b[3096]) && (!s.b[3097])) && (!s.b[3098])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && s.b[3096]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2028, 2015, 2012);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && (!s.b[3096])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[3099] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3099, if s.b[3099] { 1.0 } else { 0.0 });

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && (!s.b[3096])) && s.b[3099]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3100] = ((-s.v[2011]) < 0.0);
        s.store_scalar(3100, if s.b[3100] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && (!s.b[3096])) && (!s.b[3099])) && s.b[3100]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && (!s.b[3096])) && (!s.b[3099])) && (!s.b[3100])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && (!s.b[3096])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[3101] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3101, if s.b[3101] { 1.0 } else { 0.0 });

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && (!s.b[3096])) && s.b[3101]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3102] = ((-s.v[2013]) < 0.0);
        s.store_scalar(3102, if s.b[3102] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && (!s.b[3096])) && (!s.b[3101])) && s.b[3102]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && (!s.b[3096])) && (!s.b[3101])) && (!s.b[3102])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && (!s.b[3096])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2028, 2013, 2014);
        }

        if (((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) {
            s.store_add_div_lhs_indices(2027, 1970, 1937, 1890);
        }

        s.b[3103] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(3103, if s.b[3103] { 1.0 } else { 0.0 });

        if ((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && s.b[3103]) {
            s.store_div(2029, 2027, 1940);
        }

        s.b[3104] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(3104, if s.b[3104] { 1.0 } else { 0.0 });

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && s.b[3104]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3105] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(3105, if s.b[3105] { 1.0 } else { 0.0 });

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && s.b[3104]) && s.b[3105]) {
            s.store_exp(2005, 2015);
        }

    }

    pub(super) fn stamp_reactive_block_32(
        s: &mut ReactiveScratch,
    ) {
        s.b[3106] = (s.v[2015] < 0.0);
        s.store_scalar(3106, if s.b[3106] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && s.b[3104]) && (!s.b[3105])) && s.b[3106]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && s.b[3104]) && (!s.b[3105])) && (!s.b[3106])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && s.b[3104]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2029, 2015, 2012);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && (!s.b[3104])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[3107] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3107, if s.b[3107] { 1.0 } else { 0.0 });

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && (!s.b[3104])) && s.b[3107]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3108] = ((-s.v[2011]) < 0.0);
        s.store_scalar(3108, if s.b[3108] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && (!s.b[3104])) && (!s.b[3107])) && s.b[3108]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && (!s.b[3104])) && (!s.b[3107])) && (!s.b[3108])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && (!s.b[3104])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[3109] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3109, if s.b[3109] { 1.0 } else { 0.0 });

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && (!s.b[3104])) && s.b[3109]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3110] = ((-s.v[2013]) < 0.0);
        s.store_scalar(3110, if s.b[3110] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && (!s.b[3104])) && (!s.b[3109])) && s.b[3110]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && (!s.b[3104])) && (!s.b[3109])) && (!s.b[3110])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && (!s.b[3104])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2029, 2013, 2014);
        }

        if (((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) {
            s.store_add_div_lhs_indices(2027, 1971, 1937, 1890);
        }

        s.b[3111] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(3111, if s.b[3111] { 1.0 } else { 0.0 });

        if ((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && s.b[3111]) {
            s.store_div(2020, 2027, 1940);
        }

        s.b[3112] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(3112, if s.b[3112] { 1.0 } else { 0.0 });

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && s.b[3112]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3113] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(3113, if s.b[3113] { 1.0 } else { 0.0 });

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && s.b[3112]) && s.b[3113]) {
            s.store_exp(2005, 2015);
        }

        s.b[3114] = (s.v[2015] < 0.0);
        s.store_scalar(3114, if s.b[3114] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && s.b[3112]) && (!s.b[3113])) && s.b[3114]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && s.b[3112]) && (!s.b[3113])) && (!s.b[3114])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && s.b[3112]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2020, 2015, 2012);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && (!s.b[3112])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[3115] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3115, if s.b[3115] { 1.0 } else { 0.0 });

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && (!s.b[3112])) && s.b[3115]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3116] = ((-s.v[2011]) < 0.0);
        s.store_scalar(3116, if s.b[3116] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && (!s.b[3112])) && (!s.b[3115])) && s.b[3116]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && (!s.b[3112])) && (!s.b[3115])) && (!s.b[3116])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && (!s.b[3112])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[3117] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3117, if s.b[3117] { 1.0 } else { 0.0 });

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && (!s.b[3112])) && s.b[3117]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3118] = ((-s.v[2013]) < 0.0);
        s.store_scalar(3118, if s.b[3118] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && (!s.b[3112])) && (!s.b[3117])) && s.b[3118]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && (!s.b[3112])) && (!s.b[3117])) && (!s.b[3118])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && (!s.b[3112])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2020, 2013, 2014);
        }

        if (((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) {
            s.store_sub_ad_rhs(1980, 1890, A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1983), 1.0, s.ad_value(2028), 4.0, s.ad_value(2029), 2.0, s.ad_value(2020), 4.0), 0.08333333333333333, s.ad_value(1984), 0.08333333333333333));
        }

        s.b[3119] = (s.v[1] == 5.0);
        s.store_scalar(3119, if s.b[3119] { 1.0 } else { 0.0 });

        if ((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) {
            s.store_add_ad(1981, A::add_scaled_inputs(s.ad_value(1978), (1187.0 * 5.341880341880342e-5), s.ad_value(1979), (43.0 * 5.341880341880342e-5)), A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1969), 503.0, s.ad_value(1972), 172.0, s.ad_value(1973), 87.0, s.ad_value(1971), 265.0), 0.0003205128205128205, s.ad_value(1970), (328.0 * 0.0003205128205128205)));
            s.store_add_ad(1982, A::add_scaled_inputs(s.ad_value(1979), (1187.0 * 5.341880341880342e-5), s.ad_value(1978), (43.0 * 5.341880341880342e-5)), A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1973), 503.0, s.ad_value(1970), 172.0, s.ad_value(1969), 87.0, s.ad_value(1971), 265.0), 0.0003205128205128205, s.ad_value(1972), (328.0 * 0.0003205128205128205)));
            s.store_add_div_lhs_indices(2027, 1969, 1937, 1890);
        }

        s.b[3120] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(3120, if s.b[3120] { 1.0 } else { 0.0 });

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && s.b[3120]) {
            s.store_div(2028, 2027, 1940);
        }

        s.b[3121] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(3121, if s.b[3121] { 1.0 } else { 0.0 });

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && s.b[3121]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3122] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(3122, if s.b[3122] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && s.b[3121]) && s.b[3122]) {
            s.store_exp(2005, 2015);
        }

        s.b[3123] = (s.v[2015] < 0.0);
        s.store_scalar(3123, if s.b[3123] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && s.b[3121]) && (!s.b[3122])) && s.b[3123]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && s.b[3121]) && (!s.b[3122])) && (!s.b[3123])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && s.b[3121]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2028, 2015, 2012);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && (!s.b[3121])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[3124] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3124, if s.b[3124] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && (!s.b[3121])) && s.b[3124]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3125] = ((-s.v[2011]) < 0.0);
        s.store_scalar(3125, if s.b[3125] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && (!s.b[3121])) && (!s.b[3124])) && s.b[3125]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && (!s.b[3121])) && (!s.b[3124])) && (!s.b[3125])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && (!s.b[3121])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[3126] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3126, if s.b[3126] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && (!s.b[3121])) && s.b[3126]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3127] = ((-s.v[2013]) < 0.0);
        s.store_scalar(3127, if s.b[3127] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && (!s.b[3121])) && (!s.b[3126])) && s.b[3127]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && (!s.b[3121])) && (!s.b[3126])) && (!s.b[3127])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && (!s.b[3121])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2028, 2013, 2014);
        }

        if ((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) {
            s.store_add_div_lhs_indices(2027, 1970, 1937, 1890);
        }

        s.b[3128] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(3128, if s.b[3128] { 1.0 } else { 0.0 });

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && s.b[3128]) {
            s.store_div(2029, 2027, 1940);
        }

        s.b[3129] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(3129, if s.b[3129] { 1.0 } else { 0.0 });

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && s.b[3129]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
        }

    }

    pub(super) fn stamp_reactive_block_33(
        s: &mut ReactiveScratch,
    ) {
        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && s.b[3129]) {
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3130] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(3130, if s.b[3130] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && s.b[3129]) && s.b[3130]) {
            s.store_exp(2005, 2015);
        }

        s.b[3131] = (s.v[2015] < 0.0);
        s.store_scalar(3131, if s.b[3131] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && s.b[3129]) && (!s.b[3130])) && s.b[3131]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && s.b[3129]) && (!s.b[3130])) && (!s.b[3131])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && s.b[3129]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2029, 2015, 2012);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && (!s.b[3129])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[3132] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3132, if s.b[3132] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && (!s.b[3129])) && s.b[3132]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3133] = ((-s.v[2011]) < 0.0);
        s.store_scalar(3133, if s.b[3133] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && (!s.b[3129])) && (!s.b[3132])) && s.b[3133]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && (!s.b[3129])) && (!s.b[3132])) && (!s.b[3133])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && (!s.b[3129])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[3134] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3134, if s.b[3134] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && (!s.b[3129])) && s.b[3134]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3135] = ((-s.v[2013]) < 0.0);
        s.store_scalar(3135, if s.b[3135] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && (!s.b[3129])) && (!s.b[3134])) && s.b[3135]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && (!s.b[3129])) && (!s.b[3134])) && (!s.b[3135])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && (!s.b[3129])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2029, 2013, 2014);
        }

        if ((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) {
            s.store_add_div_lhs_indices(2027, 1971, 1937, 1890);
        }

        s.b[3136] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(3136, if s.b[3136] { 1.0 } else { 0.0 });

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && s.b[3136]) {
            s.store_div(2020, 2027, 1940);
        }

        s.b[3137] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(3137, if s.b[3137] { 1.0 } else { 0.0 });

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && s.b[3137]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3138] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(3138, if s.b[3138] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && s.b[3137]) && s.b[3138]) {
            s.store_exp(2005, 2015);
        }

        s.b[3139] = (s.v[2015] < 0.0);
        s.store_scalar(3139, if s.b[3139] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && s.b[3137]) && (!s.b[3138])) && s.b[3139]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && s.b[3137]) && (!s.b[3138])) && (!s.b[3139])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && s.b[3137]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2020, 2015, 2012);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && (!s.b[3137])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[3140] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3140, if s.b[3140] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && (!s.b[3137])) && s.b[3140]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3141] = ((-s.v[2011]) < 0.0);
        s.store_scalar(3141, if s.b[3141] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && (!s.b[3137])) && (!s.b[3140])) && s.b[3141]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && (!s.b[3137])) && (!s.b[3140])) && (!s.b[3141])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && (!s.b[3137])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[3142] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3142, if s.b[3142] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && (!s.b[3137])) && s.b[3142]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3143] = ((-s.v[2013]) < 0.0);
        s.store_scalar(3143, if s.b[3143] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && (!s.b[3137])) && (!s.b[3142])) && s.b[3143]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && (!s.b[3137])) && (!s.b[3142])) && (!s.b[3143])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && (!s.b[3137])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2020, 2013, 2014);
        }

        if ((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) {
            s.store_add_div_lhs_indices(2027, 1972, 1937, 1890);
        }

        s.b[3144] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(3144, if s.b[3144] { 1.0 } else { 0.0 });

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && s.b[3144]) {
            s.store_div(2021, 2027, 1940);
        }

        s.b[3145] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(3145, if s.b[3145] { 1.0 } else { 0.0 });

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && s.b[3145]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3146] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(3146, if s.b[3146] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && s.b[3145]) && s.b[3146]) {
            s.store_exp(2005, 2015);
        }

        s.b[3147] = (s.v[2015] < 0.0);
        s.store_scalar(3147, if s.b[3147] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && s.b[3145]) && (!s.b[3146])) && s.b[3147]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && s.b[3145]) && (!s.b[3146])) && (!s.b[3147])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && s.b[3145]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2021, 2015, 2012);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && (!s.b[3145])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[3148] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3148, if s.b[3148] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && (!s.b[3145])) && s.b[3148]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3149] = ((-s.v[2011]) < 0.0);
        s.store_scalar(3149, if s.b[3149] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && (!s.b[3145])) && (!s.b[3148])) && s.b[3149]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && (!s.b[3145])) && (!s.b[3148])) && (!s.b[3149])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && (!s.b[3145])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[3150] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3150, if s.b[3150] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && (!s.b[3145])) && s.b[3150]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3151] = ((-s.v[2013]) < 0.0);
        s.store_scalar(3151, if s.b[3151] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && (!s.b[3145])) && (!s.b[3150])) && s.b[3151]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && (!s.b[3145])) && (!s.b[3150])) && (!s.b[3151])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && (!s.b[3145])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2021, 2013, 2014);
        }

        if ((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) {
            s.store_add_div_lhs_indices(2027, 1973, 1937, 1890);
        }

        s.b[3152] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(3152, if s.b[3152] { 1.0 } else { 0.0 });

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && s.b[3152]) {
            s.store_div(2022, 2027, 1940);
        }

        s.b[3153] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(3153, if s.b[3153] { 1.0 } else { 0.0 });

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && s.b[3153]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
        }

    }

    pub(super) fn stamp_reactive_block_34(
        s: &mut ReactiveScratch,
    ) {
        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && s.b[3153]) {
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3154] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(3154, if s.b[3154] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && s.b[3153]) && s.b[3154]) {
            s.store_exp(2005, 2015);
        }

        s.b[3155] = (s.v[2015] < 0.0);
        s.store_scalar(3155, if s.b[3155] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && s.b[3153]) && (!s.b[3154])) && s.b[3155]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && s.b[3153]) && (!s.b[3154])) && (!s.b[3155])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && s.b[3153]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2022, 2015, 2012);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && (!s.b[3153])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[3156] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3156, if s.b[3156] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && (!s.b[3153])) && s.b[3156]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3157] = ((-s.v[2011]) < 0.0);
        s.store_scalar(3157, if s.b[3157] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && (!s.b[3153])) && (!s.b[3156])) && s.b[3157]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && (!s.b[3153])) && (!s.b[3156])) && (!s.b[3157])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && (!s.b[3153])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[3158] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3158, if s.b[3158] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && (!s.b[3153])) && s.b[3158]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3159] = ((-s.v[2013]) < 0.0);
        s.store_scalar(3159, if s.b[3159] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && (!s.b[3153])) && (!s.b[3158])) && s.b[3159]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && (!s.b[3153])) && (!s.b[3158])) && (!s.b[3159])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && (!s.b[3153])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2022, 2013, 2014);
        }

        if ((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) {
            s.store_sub_ad_rhs(1980, 1890, A::add_scaled_inputs(A::add(A::add_scaled_inputs4(s.ad_value(1983), 1.0, s.ad_value(2028), 4.0, s.ad_value(2020), 4.0, s.ad_value(2022), 4.0), A::add_scaled_inputs(s.ad_value(2029), 2.0, s.ad_value(2021), 2.0)), 0.05555555555555555, s.ad_value(1984), 0.05555555555555555));
        }

        s.b[3160] = (s.v[1] == 9.0);
        s.store_scalar(3160, if s.b[3160] { 1.0 } else { 0.0 });

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {
            s.store_add_ad(1981, A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add(A::add_scaled_inputs(s.ad_value(1976), (75653.0 * 2.6434745829918845e-7), s.ad_value(1972), (225999.0 * 2.6434745829918845e-7)), A::add_scaled_inputs4(s.ad_value(1977), (151321.0 * 6.608686457479711e-8), s.ad_value(1975), (454023.0 * 6.608686457479711e-8), s.ad_value(1971), (1073767.0 * 6.608686457479711e-8), s.ad_value(1969), (1564569.0 * 6.608686457479711e-8))), 1.0, s.ad_value(1974), (75623.0 * 5.286949165983769e-7)), 1.0, s.ad_value(1973), (145.0 * 0.0003453038674033149)), 1.0, s.ad_value(1970), (72263.0 * 1.0573898331967538e-6)), A::add_scaled_inputs(s.ad_value(1978), (3504517.0 * 1.1014477429132853e-8), s.ad_value(1979), (75653.0 * 1.1014477429132853e-8)));
            s.store_add_ad(1982, A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add(A::add_scaled_inputs(s.ad_value(1970), (75653.0 * 2.6434745829918845e-7), s.ad_value(1974), (225999.0 * 2.6434745829918845e-7)), A::add_scaled_inputs4(s.ad_value(1969), (151321.0 * 6.608686457479711e-8), s.ad_value(1971), (454023.0 * 6.608686457479711e-8), s.ad_value(1975), (1073767.0 * 6.608686457479711e-8), s.ad_value(1977), (1564569.0 * 6.608686457479711e-8))), 1.0, s.ad_value(1972), (75623.0 * 5.286949165983769e-7)), 1.0, s.ad_value(1973), (145.0 * 0.0003453038674033149)), 1.0, s.ad_value(1976), (72263.0 * 1.0573898331967538e-6)), A::add_scaled_inputs(s.ad_value(1979), (3504517.0 * 1.1014477429132853e-8), s.ad_value(1978), (75653.0 * 1.1014477429132853e-8)));
            s.store_add_div_lhs_indices(2027, 1969, 1937, 1890);
        }

        s.b[3161] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(3161, if s.b[3161] { 1.0 } else { 0.0 });

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && s.b[3161]) {
            s.store_div(2028, 2027, 1940);
        }

        s.b[3162] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(3162, if s.b[3162] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && s.b[3162]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3163] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(3163, if s.b[3163] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && s.b[3162]) && s.b[3163]) {
            s.store_exp(2005, 2015);
        }

        s.b[3164] = (s.v[2015] < 0.0);
        s.store_scalar(3164, if s.b[3164] { 1.0 } else { 0.0 });

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && s.b[3162]) && (!s.b[3163])) && s.b[3164]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && s.b[3162]) && (!s.b[3163])) && (!s.b[3164])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && s.b[3162]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2028, 2015, 2012);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && (!s.b[3162])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[3165] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3165, if s.b[3165] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && (!s.b[3162])) && s.b[3165]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3166] = ((-s.v[2011]) < 0.0);
        s.store_scalar(3166, if s.b[3166] { 1.0 } else { 0.0 });

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && (!s.b[3162])) && (!s.b[3165])) && s.b[3166]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && (!s.b[3162])) && (!s.b[3165])) && (!s.b[3166])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && (!s.b[3162])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[3167] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3167, if s.b[3167] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && (!s.b[3162])) && s.b[3167]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3168] = ((-s.v[2013]) < 0.0);
        s.store_scalar(3168, if s.b[3168] { 1.0 } else { 0.0 });

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && (!s.b[3162])) && (!s.b[3167])) && s.b[3168]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && (!s.b[3162])) && (!s.b[3167])) && (!s.b[3168])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && (!s.b[3162])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2028, 2013, 2014);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {
            s.store_add_div_lhs_indices(2027, 1970, 1937, 1890);
        }

        s.b[3169] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(3169, if s.b[3169] { 1.0 } else { 0.0 });

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && s.b[3169]) {
            s.store_div(2029, 2027, 1940);
        }

        s.b[3170] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(3170, if s.b[3170] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && s.b[3170]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3171] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(3171, if s.b[3171] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && s.b[3170]) && s.b[3171]) {
            s.store_exp(2005, 2015);
        }

        s.b[3172] = (s.v[2015] < 0.0);
        s.store_scalar(3172, if s.b[3172] { 1.0 } else { 0.0 });

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && s.b[3170]) && (!s.b[3171])) && s.b[3172]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && s.b[3170]) && (!s.b[3171])) && (!s.b[3172])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && s.b[3170]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2029, 2015, 2012);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && (!s.b[3170])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[3173] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3173, if s.b[3173] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && (!s.b[3170])) && s.b[3173]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3174] = ((-s.v[2011]) < 0.0);
        s.store_scalar(3174, if s.b[3174] { 1.0 } else { 0.0 });

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && (!s.b[3170])) && (!s.b[3173])) && s.b[3174]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && (!s.b[3170])) && (!s.b[3173])) && (!s.b[3174])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && (!s.b[3170])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[3175] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3175, if s.b[3175] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && (!s.b[3170])) && s.b[3175]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3176] = ((-s.v[2013]) < 0.0);
        s.store_scalar(3176, if s.b[3176] { 1.0 } else { 0.0 });

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && (!s.b[3170])) && (!s.b[3175])) && s.b[3176]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && (!s.b[3170])) && (!s.b[3175])) && (!s.b[3176])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && (!s.b[3170])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2029, 2013, 2014);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {
            s.store_add_div_lhs_indices(2027, 1971, 1937, 1890);
        }

        s.b[3177] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(3177, if s.b[3177] { 1.0 } else { 0.0 });

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && s.b[3177]) {
            s.store_div(2020, 2027, 1940);
        }

        s.b[3178] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(3178, if s.b[3178] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_35(
        s: &mut ReactiveScratch,
    ) {
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && s.b[3178]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3179] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(3179, if s.b[3179] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && s.b[3178]) && s.b[3179]) {
            s.store_exp(2005, 2015);
        }

        s.b[3180] = (s.v[2015] < 0.0);
        s.store_scalar(3180, if s.b[3180] { 1.0 } else { 0.0 });

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && s.b[3178]) && (!s.b[3179])) && s.b[3180]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && s.b[3178]) && (!s.b[3179])) && (!s.b[3180])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && s.b[3178]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2020, 2015, 2012);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && (!s.b[3178])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[3181] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3181, if s.b[3181] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && (!s.b[3178])) && s.b[3181]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3182] = ((-s.v[2011]) < 0.0);
        s.store_scalar(3182, if s.b[3182] { 1.0 } else { 0.0 });

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && (!s.b[3178])) && (!s.b[3181])) && s.b[3182]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && (!s.b[3178])) && (!s.b[3181])) && (!s.b[3182])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && (!s.b[3178])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[3183] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3183, if s.b[3183] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && (!s.b[3178])) && s.b[3183]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3184] = ((-s.v[2013]) < 0.0);
        s.store_scalar(3184, if s.b[3184] { 1.0 } else { 0.0 });

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && (!s.b[3178])) && (!s.b[3183])) && s.b[3184]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && (!s.b[3178])) && (!s.b[3183])) && (!s.b[3184])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && (!s.b[3178])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2020, 2013, 2014);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {
            s.store_add_div_lhs_indices(2027, 1972, 1937, 1890);
        }

        s.b[3185] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(3185, if s.b[3185] { 1.0 } else { 0.0 });

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && s.b[3185]) {
            s.store_div(2021, 2027, 1940);
        }

        s.b[3186] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(3186, if s.b[3186] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && s.b[3186]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3187] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(3187, if s.b[3187] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && s.b[3186]) && s.b[3187]) {
            s.store_exp(2005, 2015);
        }

        s.b[3188] = (s.v[2015] < 0.0);
        s.store_scalar(3188, if s.b[3188] { 1.0 } else { 0.0 });

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && s.b[3186]) && (!s.b[3187])) && s.b[3188]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && s.b[3186]) && (!s.b[3187])) && (!s.b[3188])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && s.b[3186]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2021, 2015, 2012);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[3189] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3189, if s.b[3189] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) && s.b[3189]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3190] = ((-s.v[2011]) < 0.0);
        s.store_scalar(3190, if s.b[3190] { 1.0 } else { 0.0 });

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) && (!s.b[3189])) && s.b[3190]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) && (!s.b[3189])) && (!s.b[3190])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[3191] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3191, if s.b[3191] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) && s.b[3191]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3192] = ((-s.v[2013]) < 0.0);
        s.store_scalar(3192, if s.b[3192] { 1.0 } else { 0.0 });

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) && (!s.b[3191])) && s.b[3192]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) && (!s.b[3191])) && (!s.b[3192])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2021, 2013, 2014);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {
            s.store_add_div_lhs_indices(2027, 1973, 1937, 1890);
        }

        s.b[3193] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(3193, if s.b[3193] { 1.0 } else { 0.0 });

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && s.b[3193]) {
            s.store_div(2022, 2027, 1940);
        }

        s.b[3194] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(3194, if s.b[3194] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && s.b[3194]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3195] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(3195, if s.b[3195] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && s.b[3194]) && s.b[3195]) {
            s.store_exp(2005, 2015);
        }

        s.b[3196] = (s.v[2015] < 0.0);
        s.store_scalar(3196, if s.b[3196] { 1.0 } else { 0.0 });

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && s.b[3194]) && (!s.b[3195])) && s.b[3196]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && s.b[3194]) && (!s.b[3195])) && (!s.b[3196])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && s.b[3194]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2022, 2015, 2012);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && (!s.b[3194])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[3197] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3197, if s.b[3197] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && (!s.b[3194])) && s.b[3197]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3198] = ((-s.v[2011]) < 0.0);
        s.store_scalar(3198, if s.b[3198] { 1.0 } else { 0.0 });

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && (!s.b[3194])) && (!s.b[3197])) && s.b[3198]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && (!s.b[3194])) && (!s.b[3197])) && (!s.b[3198])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && (!s.b[3194])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[3199] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3199, if s.b[3199] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && (!s.b[3194])) && s.b[3199]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3200] = ((-s.v[2013]) < 0.0);
        s.store_scalar(3200, if s.b[3200] { 1.0 } else { 0.0 });

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && (!s.b[3194])) && (!s.b[3199])) && s.b[3200]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && (!s.b[3194])) && (!s.b[3199])) && (!s.b[3200])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && (!s.b[3194])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2022, 2013, 2014);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {
            s.store_add_div_lhs_indices(2027, 1974, 1937, 1890);
        }

        s.b[3201] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(3201, if s.b[3201] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_36(
        s: &mut ReactiveScratch,
    ) {
        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && s.b[3201]) {
            s.store_div(2023, 2027, 1940);
        }

        s.b[3202] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(3202, if s.b[3202] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && s.b[3202]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3203] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(3203, if s.b[3203] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && s.b[3202]) && s.b[3203]) {
            s.store_exp(2005, 2015);
        }

        s.b[3204] = (s.v[2015] < 0.0);
        s.store_scalar(3204, if s.b[3204] { 1.0 } else { 0.0 });

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && s.b[3202]) && (!s.b[3203])) && s.b[3204]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && s.b[3202]) && (!s.b[3203])) && (!s.b[3204])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && s.b[3202]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2023, 2015, 2012);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && (!s.b[3202])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[3205] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3205, if s.b[3205] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && (!s.b[3202])) && s.b[3205]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3206] = ((-s.v[2011]) < 0.0);
        s.store_scalar(3206, if s.b[3206] { 1.0 } else { 0.0 });

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && (!s.b[3202])) && (!s.b[3205])) && s.b[3206]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && (!s.b[3202])) && (!s.b[3205])) && (!s.b[3206])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && (!s.b[3202])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[3207] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3207, if s.b[3207] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && (!s.b[3202])) && s.b[3207]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3208] = ((-s.v[2013]) < 0.0);
        s.store_scalar(3208, if s.b[3208] { 1.0 } else { 0.0 });

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && (!s.b[3202])) && (!s.b[3207])) && s.b[3208]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && (!s.b[3202])) && (!s.b[3207])) && (!s.b[3208])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && (!s.b[3202])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2023, 2013, 2014);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {
            s.store_add_div_lhs_indices(2027, 1975, 1937, 1890);
        }

        s.b[3209] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(3209, if s.b[3209] { 1.0 } else { 0.0 });

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && s.b[3209]) {
            s.store_div(2024, 2027, 1940);
        }

        s.b[3210] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(3210, if s.b[3210] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && s.b[3210]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3211] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(3211, if s.b[3211] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && s.b[3210]) && s.b[3211]) {
            s.store_exp(2005, 2015);
        }

        s.b[3212] = (s.v[2015] < 0.0);
        s.store_scalar(3212, if s.b[3212] { 1.0 } else { 0.0 });

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && s.b[3210]) && (!s.b[3211])) && s.b[3212]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && s.b[3210]) && (!s.b[3211])) && (!s.b[3212])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && s.b[3210]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2024, 2015, 2012);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && (!s.b[3210])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[3213] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3213, if s.b[3213] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && (!s.b[3210])) && s.b[3213]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3214] = ((-s.v[2011]) < 0.0);
        s.store_scalar(3214, if s.b[3214] { 1.0 } else { 0.0 });

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && (!s.b[3210])) && (!s.b[3213])) && s.b[3214]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && (!s.b[3210])) && (!s.b[3213])) && (!s.b[3214])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && (!s.b[3210])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[3215] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3215, if s.b[3215] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && (!s.b[3210])) && s.b[3215]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3216] = ((-s.v[2013]) < 0.0);
        s.store_scalar(3216, if s.b[3216] { 1.0 } else { 0.0 });

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && (!s.b[3210])) && (!s.b[3215])) && s.b[3216]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && (!s.b[3210])) && (!s.b[3215])) && (!s.b[3216])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && (!s.b[3210])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2024, 2013, 2014);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {
            s.store_add_div_lhs_indices(2027, 1976, 1937, 1890);
        }

        s.b[3217] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(3217, if s.b[3217] { 1.0 } else { 0.0 });

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && s.b[3217]) {
            s.store_div(2025, 2027, 1940);
        }

        s.b[3218] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(3218, if s.b[3218] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && s.b[3218]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3219] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(3219, if s.b[3219] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && s.b[3218]) && s.b[3219]) {
            s.store_exp(2005, 2015);
        }

        s.b[3220] = (s.v[2015] < 0.0);
        s.store_scalar(3220, if s.b[3220] { 1.0 } else { 0.0 });

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && s.b[3218]) && (!s.b[3219])) && s.b[3220]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && s.b[3218]) && (!s.b[3219])) && (!s.b[3220])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && s.b[3218]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2025, 2015, 2012);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && (!s.b[3218])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[3221] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3221, if s.b[3221] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && (!s.b[3218])) && s.b[3221]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3222] = ((-s.v[2011]) < 0.0);
        s.store_scalar(3222, if s.b[3222] { 1.0 } else { 0.0 });

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && (!s.b[3218])) && (!s.b[3221])) && s.b[3222]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && (!s.b[3218])) && (!s.b[3221])) && (!s.b[3222])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && (!s.b[3218])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[3223] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3223, if s.b[3223] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && (!s.b[3218])) && s.b[3223]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3224] = ((-s.v[2013]) < 0.0);
        s.store_scalar(3224, if s.b[3224] { 1.0 } else { 0.0 });

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && (!s.b[3218])) && (!s.b[3223])) && s.b[3224]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && (!s.b[3218])) && (!s.b[3223])) && (!s.b[3224])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && (!s.b[3218])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2025, 2013, 2014);
        }

    }

    pub(super) fn stamp_reactive_block_37(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {
            s.store_add_div_lhs_indices(2027, 1977, 1937, 1890);
        }

        s.b[3225] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.store_scalar(3225, if s.b[3225] { 1.0 } else { 0.0 });

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && s.b[3225]) {
            s.store_div(2026, 2027, 1940);
        }

        s.b[3226] = (s.v[2027] < (-s.v[1941]));
        s.store_scalar(3226, if s.b[3226] { 1.0 } else { 0.0 });

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && s.b[3226]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3227] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.store_scalar(3227, if s.b[3227] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && s.b[3226]) && s.b[3227]) {
            s.store_exp(2005, 2015);
        }

        s.b[3228] = (s.v[2015] < 0.0);
        s.store_scalar(3228, if s.b[3228] { 1.0 } else { 0.0 });

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && s.b[3226]) && (!s.b[3227])) && s.b[3228]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && s.b[3226]) && (!s.b[3227])) && (!s.b[3228])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && s.b[3226]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_neg_add(2026, 2015, 2012);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && (!s.b[3226])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_mul_offset_rhs_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0);
        }

        s.b[3229] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3229, if s.b[3229] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && (!s.b[3226])) && s.b[3229]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3230] = ((-s.v[2011]) < 0.0);
        s.store_scalar(3230, if s.b[3230] { 1.0 } else { 0.0 });

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && (!s.b[3226])) && (!s.b[3229])) && s.b[3230]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && (!s.b[3226])) && (!s.b[3229])) && (!s.b[3230])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && (!s.b[3226])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[3231] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.store_scalar(3231, if s.b[3231] { 1.0 } else { 0.0 });

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && (!s.b[3226])) && s.b[3231]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3232] = ((-s.v[2013]) < 0.0);
        s.store_scalar(3232, if s.b[3232] { 1.0 } else { 0.0 });

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && (!s.b[3226])) && (!s.b[3231])) && s.b[3232]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && (!s.b[3226])) && (!s.b[3231])) && (!s.b[3232])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && (!s.b[3226])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2026, 2013, 2014);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {
            s.store_sub_ad_rhs(1980, 1890, A::add_scaled_inputs(A::add(A::add(s.ad_value(1983), A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(2028), 1.0, s.ad_value(2020), 1.0, s.ad_value(2022), 1.0, s.ad_value(2024), 1.0), 4.0, s.ad_value(2026), 4.0)), A::add_scaled_inputs4(s.ad_value(2029), 2.0, s.ad_value(2021), 2.0, s.ad_value(2023), 2.0, s.ad_value(2025), 2.0)), 0.03333333333333333, s.ad_value(1984), 0.03333333333333333));
        }

        if s.b[3067] {
            s.store_mul(1980, 1937, 1980);
        }

        s.b[3233] = (s.v[831] > 0.0);
        s.store_scalar(3233, if s.b[3233] { 1.0 } else { 0.0 });

        if (s.b[3067] && s.b[3233]) {
            s.store_mul3_lhs(850, 1904, 1888, 1981);
            s.store_mul3_lhs(853, 1904, 1888, 1982);
        }

        if (s.b[3067] && (!s.b[3233])) {
            s.store_mul3_lhs(850, 1904, 1888, 1982);
            s.store_mul3_lhs(853, 1904, 1888, 1981);
        }

        if s.b[3067] {
            s.store_mul3_lhs(851, 1904, 1888, 1980);
            s.store_add_scaled_inputs3_indices(852, 851, -1.0, 850, (-1.0), 853, -1.0);
        }

        s.store_add_scaled_inputs3_indices(850, 851, (-1.0), 852, (-1.0), 853, (-1.0));

        s.store_add(854, 854, 1910);

        s.store_add(855, 855, 1911);

        s.store_add_scaled_products3(857, s.ad_value(646), s.ad_value(1918), 1.0, s.ad_value(647), s.ad_value(1919), 1.0, s.ad_value(648), s.ad_value(1920), 1.0);

        s.store_add_scaled_products3(858, s.ad_value(673), s.ad_value(1921), 1.0, s.ad_value(674), s.ad_value(1922), 1.0, s.ad_value(675), s.ad_value(1923), 1.0);

        s.b[3235] = (s.v[831] < 0.0);
        s.store_scalar(3235, if s.b[3235] { 1.0 } else { 0.0 });

        if s.b[3235] {
            s.copy_ad(3234, 853);
            s.copy_ad(853, 850);
            s.copy_ad(850, 3234);
        }

        s.store_mul(860, 1904, 1895);

        s.b[3268] = ((s.v[1829] > 0.0) && (s.v[716] > 0.0));
        s.store_scalar(3268, if s.b[3268] { 1.0 } else { 0.0 });

        s.b[3273] = ((((p.p50 == 1.0) && (s.v[719] > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0));
        s.store_scalar(3273, if s.b[3273] { 1.0 } else { 0.0 });

        if (s.b[3268] && s.b[3273]) {
            s.store_div_scaled_product3_mixed_aiia(860, A::square(s.ad_value(1908)), 1904, 1895, 1.0, A::square(s.ad_value(1906)), 1.0);
        }

        s.b[3277] = (((p.p46 != 0.0) && (s.v[287] > 0.0)) && (s.v[1880] > 0.0));
        s.store_scalar(3277, if s.b[3277] { 1.0 } else { 0.0 });

        if s.b[3277] {
            s.store_div_scaled_inputs_indices(2028, 1883, 4.0, 724, 1.0);
            s.store_scale(2028, 771, s.v[715]);
            s.store_mul(2028, 1864, 1877);
        }

    }

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        var_chnl_type: f64,
        var_ijun_s: f64,
        var_ijun_s_db0: f64,
        var_ijun_s_db1: f64,
        var_ijun_s_db10: f64,
        var_ijun_s_db11: f64,
        var_ijun_s_db12: f64,
        var_ijun_s_db13: f64,
        var_ijun_s_db14: f64,
        var_ijun_s_db15: f64,
        var_ijun_s_db16: f64,
        var_ijun_s_db17: f64,
        var_ijun_s_db18: f64,
        var_ijun_s_db19: f64,
        var_ijun_s_db2: f64,
        var_ijun_s_db20: f64,
        var_ijun_s_db21: f64,
        var_ijun_s_db22: f64,
        var_ijun_s_db23: f64,
        var_ijun_s_db24: f64,
        var_ijun_s_db3: f64,
        var_ijun_s_db4: f64,
        var_ijun_s_db5: f64,
        var_ijun_s_db6: f64,
        var_ijun_s_db7: f64,
        var_ijun_s_db8: f64,
        var_ijun_s_db9: f64,
        var_ijun_s_dn0: f64,
        var_ijun_s_dn1: f64,
        var_ijun_s_dn10: f64,
        var_ijun_s_dn11: f64,
        var_ijun_s_dn12: f64,
        var_ijun_s_dn13: f64,
        var_ijun_s_dn14: f64,
        var_ijun_s_dn15: f64,
        var_ijun_s_dn16: f64,
        var_ijun_s_dn17: f64,
        var_ijun_s_dn18: f64,
        var_ijun_s_dn19: f64,
        var_ijun_s_dn2: f64,
        var_ijun_s_dn20: f64,
        var_ijun_s_dn3: f64,
        var_ijun_s_dn4: f64,
        var_ijun_s_dn5: f64,
        var_ijun_s_dn6: f64,
        var_ijun_s_dn7: f64,
        var_ijun_s_dn8: f64,
        var_ijun_s_dn9: f64,
        var_mult_inst: f64,
    ) {
        let __rspice_deriv_cse_0: f64 = (s.dn[838][0] + s.dn[846][0]);
        let __rspice_deriv_cse_1: f64 = (s.dn[838][1] + s.dn[846][1]);
        let __rspice_deriv_cse_2: f64 = (s.dn[838][2] + s.dn[846][2]);
        let __rspice_deriv_cse_3: f64 = (s.dn[838][3] + s.dn[846][3]);
        let __rspice_deriv_cse_4: f64 = (s.dn[838][4] + s.dn[846][4]);
        let __rspice_deriv_cse_5: f64 = (s.dn[838][5] + s.dn[846][5]);
        let __rspice_deriv_cse_6: f64 = (s.dn[838][6] + s.dn[846][6]);
        let __rspice_deriv_cse_7: f64 = (s.dn[838][7] + s.dn[846][7]);
        let __rspice_deriv_cse_8: f64 = (s.dn[838][8] + s.dn[846][8]);
        let __rspice_deriv_cse_9: f64 = (s.dn[838][9] + s.dn[846][9]);
        let __rspice_deriv_cse_10: f64 = (s.dn[838][10] + s.dn[846][10]);
        let __rspice_deriv_cse_11: f64 = (s.dn[838][11] + s.dn[846][11]);
        let __rspice_deriv_cse_12: f64 = (s.dn[838][12] + s.dn[846][12]);
        let __rspice_deriv_cse_13: f64 = (s.dn[838][13] + s.dn[846][13]);
        let __rspice_deriv_cse_14: f64 = (s.dn[838][14] + s.dn[846][14]);
        let __rspice_deriv_cse_15: f64 = (s.dn[838][15] + s.dn[846][15]);
        let __rspice_deriv_cse_16: f64 = (s.dn[838][16] + s.dn[846][16]);
        let __rspice_deriv_cse_17: f64 = (s.dn[838][17] + s.dn[846][17]);
        let __rspice_deriv_cse_18: f64 = (s.dn[838][18] + s.dn[846][18]);
        let __rspice_deriv_cse_19: f64 = (s.dn[838][19] + s.dn[846][19]);
        let __rspice_deriv_cse_20: f64 = (s.dn[838][20] + s.dn[846][20]);
        let __rspice_deriv_cse_21: f64 = (s.db[838][0] + s.db[846][0]);
        let __rspice_deriv_cse_22: f64 = (s.db[838][1] + s.db[846][1]);
        let __rspice_deriv_cse_23: f64 = (s.db[838][2] + s.db[846][2]);
        let __rspice_deriv_cse_24: f64 = (s.db[838][3] + s.db[846][3]);
        let __rspice_deriv_cse_25: f64 = (s.db[838][4] + s.db[846][4]);
        let __rspice_deriv_cse_26: f64 = (s.db[838][5] + s.db[846][5]);
        let __rspice_deriv_cse_27: f64 = (s.db[838][6] + s.db[846][6]);
        let __rspice_deriv_cse_28: f64 = (s.db[838][7] + s.db[846][7]);
        let __rspice_deriv_cse_29: f64 = (s.db[838][8] + s.db[846][8]);
        let __rspice_deriv_cse_30: f64 = (s.db[838][9] + s.db[846][9]);
        let __rspice_deriv_cse_31: f64 = (s.db[838][10] + s.db[846][10]);
        let __rspice_deriv_cse_32: f64 = (s.db[838][11] + s.db[846][11]);
        let __rspice_deriv_cse_33: f64 = (s.db[838][12] + s.db[846][12]);
        let __rspice_deriv_cse_34: f64 = (s.db[838][13] + s.db[846][13]);
        let __rspice_deriv_cse_35: f64 = (s.db[838][14] + s.db[846][14]);
        let __rspice_deriv_cse_36: f64 = (s.db[838][15] + s.db[846][15]);
        let __rspice_deriv_cse_37: f64 = (s.db[838][16] + s.db[846][16]);
        let __rspice_deriv_cse_38: f64 = (s.db[838][17] + s.db[846][17]);
        let __rspice_deriv_cse_39: f64 = (s.db[838][18] + s.db[846][18]);
        let __rspice_deriv_cse_40: f64 = (s.db[838][19] + s.db[846][19]);
        let __rspice_deriv_cse_41: f64 = (s.db[838][20] + s.db[846][20]);
        let __rspice_deriv_cse_42: f64 = (s.db[838][21] + s.db[846][21]);
        let __rspice_deriv_cse_43: f64 = (s.db[838][22] + s.db[846][22]);
        let __rspice_deriv_cse_44: f64 = (s.db[838][23] + s.db[846][23]);
        let __rspice_deriv_cse_45: f64 = (s.db[838][24] + s.db[846][24]);
        let (eq0_e955, eq0_e955_d_n0, eq0_e955_d_n1, eq0_e955_d_n2, eq0_e955_d_n3, eq0_e955_d_n4, eq0_e955_d_n5, eq0_e955_d_n6, eq0_e955_d_n7, eq0_e955_d_n8, eq0_e955_d_n9, eq0_e955_d_n10, eq0_e955_d_n11, eq0_e955_d_n12, eq0_e955_d_n13, eq0_e955_d_n14, eq0_e955_d_n15, eq0_e955_d_n16, eq0_e955_d_n17, eq0_e955_d_n18, eq0_e955_d_n19, eq0_e955_d_n20, eq0_e955_d_b0, eq0_e955_d_b1, eq0_e955_d_b2, eq0_e955_d_b3, eq0_e955_d_b4, eq0_e955_d_b5, eq0_e955_d_b6, eq0_e955_d_b7, eq0_e955_d_b8, eq0_e955_d_b9, eq0_e955_d_b10, eq0_e955_d_b11, eq0_e955_d_b12, eq0_e955_d_b13, eq0_e955_d_b14, eq0_e955_d_b15, eq0_e955_d_b16, eq0_e955_d_b17, eq0_e955_d_b18, eq0_e955_d_b19, eq0_e955_d_b20, eq0_e955_d_b21, eq0_e955_d_b22, eq0_e955_d_b23, eq0_e955_d_b24,) = {
    if s.b[2913] {
        let eq0_e949: f64 = (var_chnl_type * var_mult_inst);
        let eq0_e951: f64 = (eq0_e949 * p.p32);
        let eq0_e953: f64 = (eq0_e951 * s.v[847]);
        (eq0_e953, (eq0_e951 * s.dn[847][0]), (eq0_e951 * s.dn[847][1]), (eq0_e951 * s.dn[847][2]), (eq0_e951 * s.dn[847][3]), (eq0_e951 * s.dn[847][4]), (eq0_e951 * s.dn[847][5]), (eq0_e951 * s.dn[847][6]), (eq0_e951 * s.dn[847][7]), (eq0_e951 * s.dn[847][8]), (eq0_e951 * s.dn[847][9]), (eq0_e951 * s.dn[847][10]), (eq0_e951 * s.dn[847][11]), (eq0_e951 * s.dn[847][12]), (eq0_e951 * s.dn[847][13]), (eq0_e951 * s.dn[847][14]), (eq0_e951 * s.dn[847][15]), (eq0_e951 * s.dn[847][16]), (eq0_e951 * s.dn[847][17]), (eq0_e951 * s.dn[847][18]), (eq0_e951 * s.dn[847][19]), (eq0_e951 * s.dn[847][20]), (eq0_e951 * s.db[847][0]), (eq0_e951 * s.db[847][1]), (eq0_e951 * s.db[847][2]), (eq0_e951 * s.db[847][3]), (eq0_e951 * s.db[847][4]), (eq0_e951 * s.db[847][5]), (eq0_e951 * s.db[847][6]), (eq0_e951 * s.db[847][7]), (eq0_e951 * s.db[847][8]), (eq0_e951 * s.db[847][9]), (eq0_e951 * s.db[847][10]), (eq0_e951 * s.db[847][11]), (eq0_e951 * s.db[847][12]), (eq0_e951 * s.db[847][13]), (eq0_e951 * s.db[847][14]), (eq0_e951 * s.db[847][15]), (eq0_e951 * s.db[847][16]), (eq0_e951 * s.db[847][17]), (eq0_e951 * s.db[847][18]), (eq0_e951 * s.db[847][19]), (eq0_e951 * s.db[847][20]), (eq0_e951 * s.db[847][21]), (eq0_e951 * s.db[847][22]), (eq0_e951 * s.db[847][23]), (eq0_e951 * s.db[847][24]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e955;
        let eq0_node_derivatives: [f64; 21] = [eq0_e955_d_n0, eq0_e955_d_n1, eq0_e955_d_n2, eq0_e955_d_n3, eq0_e955_d_n4, eq0_e955_d_n5, eq0_e955_d_n6, eq0_e955_d_n7, eq0_e955_d_n8, eq0_e955_d_n9, eq0_e955_d_n10, eq0_e955_d_n11, eq0_e955_d_n12, eq0_e955_d_n13, eq0_e955_d_n14, eq0_e955_d_n15, eq0_e955_d_n16, eq0_e955_d_n17, eq0_e955_d_n18, eq0_e955_d_n19, eq0_e955_d_n20];
        let eq0_branch_derivatives: [f64; 25] = [eq0_e955_d_b0, eq0_e955_d_b1, eq0_e955_d_b2, eq0_e955_d_b3, eq0_e955_d_b4, eq0_e955_d_b5, eq0_e955_d_b6, eq0_e955_d_b7, eq0_e955_d_b8, eq0_e955_d_b9, eq0_e955_d_b10, eq0_e955_d_b11, eq0_e955_d_b12, eq0_e955_d_b13, eq0_e955_d_b14, eq0_e955_d_b15, eq0_e955_d_b16, eq0_e955_d_b17, eq0_e955_d_b18, eq0_e955_d_b19, eq0_e955_d_b20, eq0_e955_d_b21, eq0_e955_d_b22, eq0_e955_d_b23, eq0_e955_d_b24];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let (eq1_e967, eq1_e967_d_n0, eq1_e967_d_n1, eq1_e967_d_n2, eq1_e967_d_n3, eq1_e967_d_n4, eq1_e967_d_n5, eq1_e967_d_n6, eq1_e967_d_n7, eq1_e967_d_n8, eq1_e967_d_n9, eq1_e967_d_n10, eq1_e967_d_n11, eq1_e967_d_n12, eq1_e967_d_n13, eq1_e967_d_n14, eq1_e967_d_n15, eq1_e967_d_n16, eq1_e967_d_n17, eq1_e967_d_n18, eq1_e967_d_n19, eq1_e967_d_n20, eq1_e967_d_b0, eq1_e967_d_b1, eq1_e967_d_b2, eq1_e967_d_b3, eq1_e967_d_b4, eq1_e967_d_b5, eq1_e967_d_b6, eq1_e967_d_b7, eq1_e967_d_b8, eq1_e967_d_b9, eq1_e967_d_b10, eq1_e967_d_b11, eq1_e967_d_b12, eq1_e967_d_b13, eq1_e967_d_b14, eq1_e967_d_b15, eq1_e967_d_b16, eq1_e967_d_b17, eq1_e967_d_b18, eq1_e967_d_b19, eq1_e967_d_b20, eq1_e967_d_b21, eq1_e967_d_b22, eq1_e967_d_b23, eq1_e967_d_b24,) = {
    if s.b[2913] {
        let eq1_e959: f64 = (var_chnl_type * var_mult_inst);
        let eq1_e961: f64 = (eq1_e959 * p.p32);
        let eq1_e964: f64 = (s.v[838] + s.v[846]);
        let eq1_e965: f64 = (eq1_e961 * eq1_e964);
        let eq1_e965_d_n0: f64 = (eq1_e961 * __rspice_deriv_cse_0);
        let eq1_e965_d_n1: f64 = (eq1_e961 * __rspice_deriv_cse_1);
        let eq1_e965_d_n2: f64 = (eq1_e961 * __rspice_deriv_cse_2);
        let eq1_e965_d_n3: f64 = (eq1_e961 * __rspice_deriv_cse_3);
        let eq1_e965_d_n4: f64 = (eq1_e961 * __rspice_deriv_cse_4);
        let eq1_e965_d_n5: f64 = (eq1_e961 * __rspice_deriv_cse_5);
        let eq1_e965_d_n6: f64 = (eq1_e961 * __rspice_deriv_cse_6);
        let eq1_e965_d_n7: f64 = (eq1_e961 * __rspice_deriv_cse_7);
        let eq1_e965_d_n8: f64 = (eq1_e961 * __rspice_deriv_cse_8);
        let eq1_e965_d_n9: f64 = (eq1_e961 * __rspice_deriv_cse_9);
        let eq1_e965_d_n10: f64 = (eq1_e961 * __rspice_deriv_cse_10);
        let eq1_e965_d_n11: f64 = (eq1_e961 * __rspice_deriv_cse_11);
        let eq1_e965_d_n12: f64 = (eq1_e961 * __rspice_deriv_cse_12);
        let eq1_e965_d_n13: f64 = (eq1_e961 * __rspice_deriv_cse_13);
        let eq1_e965_d_n14: f64 = (eq1_e961 * __rspice_deriv_cse_14);
        let eq1_e965_d_n15: f64 = (eq1_e961 * __rspice_deriv_cse_15);
        let eq1_e965_d_n16: f64 = (eq1_e961 * __rspice_deriv_cse_16);
        let eq1_e965_d_n17: f64 = (eq1_e961 * __rspice_deriv_cse_17);
        let eq1_e965_d_n18: f64 = (eq1_e961 * __rspice_deriv_cse_18);
        let eq1_e965_d_n19: f64 = (eq1_e961 * __rspice_deriv_cse_19);
        let eq1_e965_d_n20: f64 = (eq1_e961 * __rspice_deriv_cse_20);
        let eq1_e965_d_b0: f64 = (eq1_e961 * __rspice_deriv_cse_21);
        let eq1_e965_d_b1: f64 = (eq1_e961 * __rspice_deriv_cse_22);
        let eq1_e965_d_b2: f64 = (eq1_e961 * __rspice_deriv_cse_23);
        let eq1_e965_d_b3: f64 = (eq1_e961 * __rspice_deriv_cse_24);
        let eq1_e965_d_b4: f64 = (eq1_e961 * __rspice_deriv_cse_25);
        let eq1_e965_d_b5: f64 = (eq1_e961 * __rspice_deriv_cse_26);
        let eq1_e965_d_b6: f64 = (eq1_e961 * __rspice_deriv_cse_27);
        let eq1_e965_d_b7: f64 = (eq1_e961 * __rspice_deriv_cse_28);
        let eq1_e965_d_b8: f64 = (eq1_e961 * __rspice_deriv_cse_29);
        let eq1_e965_d_b9: f64 = (eq1_e961 * __rspice_deriv_cse_30);
        let eq1_e965_d_b10: f64 = (eq1_e961 * __rspice_deriv_cse_31);
        let eq1_e965_d_b11: f64 = (eq1_e961 * __rspice_deriv_cse_32);
        let eq1_e965_d_b12: f64 = (eq1_e961 * __rspice_deriv_cse_33);
        let eq1_e965_d_b13: f64 = (eq1_e961 * __rspice_deriv_cse_34);
        let eq1_e965_d_b14: f64 = (eq1_e961 * __rspice_deriv_cse_35);
        let eq1_e965_d_b15: f64 = (eq1_e961 * __rspice_deriv_cse_36);
        let eq1_e965_d_b16: f64 = (eq1_e961 * __rspice_deriv_cse_37);
        let eq1_e965_d_b17: f64 = (eq1_e961 * __rspice_deriv_cse_38);
        let eq1_e965_d_b18: f64 = (eq1_e961 * __rspice_deriv_cse_39);
        let eq1_e965_d_b19: f64 = (eq1_e961 * __rspice_deriv_cse_40);
        let eq1_e965_d_b20: f64 = (eq1_e961 * __rspice_deriv_cse_41);
        let eq1_e965_d_b21: f64 = (eq1_e961 * __rspice_deriv_cse_42);
        let eq1_e965_d_b22: f64 = (eq1_e961 * __rspice_deriv_cse_43);
        let eq1_e965_d_b23: f64 = (eq1_e961 * __rspice_deriv_cse_44);
        let eq1_e965_d_b24: f64 = (eq1_e961 * __rspice_deriv_cse_45);
        (eq1_e965, eq1_e965_d_n0, eq1_e965_d_n1, eq1_e965_d_n2, eq1_e965_d_n3, eq1_e965_d_n4, eq1_e965_d_n5, eq1_e965_d_n6, eq1_e965_d_n7, eq1_e965_d_n8, eq1_e965_d_n9, eq1_e965_d_n10, eq1_e965_d_n11, eq1_e965_d_n12, eq1_e965_d_n13, eq1_e965_d_n14, eq1_e965_d_n15, eq1_e965_d_n16, eq1_e965_d_n17, eq1_e965_d_n18, eq1_e965_d_n19, eq1_e965_d_n20, eq1_e965_d_b0, eq1_e965_d_b1, eq1_e965_d_b2, eq1_e965_d_b3, eq1_e965_d_b4, eq1_e965_d_b5, eq1_e965_d_b6, eq1_e965_d_b7, eq1_e965_d_b8, eq1_e965_d_b9, eq1_e965_d_b10, eq1_e965_d_b11, eq1_e965_d_b12, eq1_e965_d_b13, eq1_e965_d_b14, eq1_e965_d_b15, eq1_e965_d_b16, eq1_e965_d_b17, eq1_e965_d_b18, eq1_e965_d_b19, eq1_e965_d_b20, eq1_e965_d_b21, eq1_e965_d_b22, eq1_e965_d_b23, eq1_e965_d_b24,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e967;
        let eq1_node_derivatives: [f64; 21] = [eq1_e967_d_n0, eq1_e967_d_n1, eq1_e967_d_n2, eq1_e967_d_n3, eq1_e967_d_n4, eq1_e967_d_n5, eq1_e967_d_n6, eq1_e967_d_n7, eq1_e967_d_n8, eq1_e967_d_n9, eq1_e967_d_n10, eq1_e967_d_n11, eq1_e967_d_n12, eq1_e967_d_n13, eq1_e967_d_n14, eq1_e967_d_n15, eq1_e967_d_n16, eq1_e967_d_n17, eq1_e967_d_n18, eq1_e967_d_n19, eq1_e967_d_n20];
        let eq1_branch_derivatives: [f64; 25] = [eq1_e967_d_b0, eq1_e967_d_b1, eq1_e967_d_b2, eq1_e967_d_b3, eq1_e967_d_b4, eq1_e967_d_b5, eq1_e967_d_b6, eq1_e967_d_b7, eq1_e967_d_b8, eq1_e967_d_b9, eq1_e967_d_b10, eq1_e967_d_b11, eq1_e967_d_b12, eq1_e967_d_b13, eq1_e967_d_b14, eq1_e967_d_b15, eq1_e967_d_b16, eq1_e967_d_b17, eq1_e967_d_b18, eq1_e967_d_b19, eq1_e967_d_b20, eq1_e967_d_b21, eq1_e967_d_b22, eq1_e967_d_b23, eq1_e967_d_b24];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let (eq2_e977, eq2_e977_d_n0, eq2_e977_d_n1, eq2_e977_d_n2, eq2_e977_d_n3, eq2_e977_d_n4, eq2_e977_d_n5, eq2_e977_d_n6, eq2_e977_d_n7, eq2_e977_d_n8, eq2_e977_d_n9, eq2_e977_d_n10, eq2_e977_d_n11, eq2_e977_d_n12, eq2_e977_d_n13, eq2_e977_d_n14, eq2_e977_d_n15, eq2_e977_d_n16, eq2_e977_d_n17, eq2_e977_d_n18, eq2_e977_d_n19, eq2_e977_d_n20, eq2_e977_d_b0, eq2_e977_d_b1, eq2_e977_d_b2, eq2_e977_d_b3, eq2_e977_d_b4, eq2_e977_d_b5, eq2_e977_d_b6, eq2_e977_d_b7, eq2_e977_d_b8, eq2_e977_d_b9, eq2_e977_d_b10, eq2_e977_d_b11, eq2_e977_d_b12, eq2_e977_d_b13, eq2_e977_d_b14, eq2_e977_d_b15, eq2_e977_d_b16, eq2_e977_d_b17, eq2_e977_d_b18, eq2_e977_d_b19, eq2_e977_d_b20, eq2_e977_d_b21, eq2_e977_d_b22, eq2_e977_d_b23, eq2_e977_d_b24,) = {
    if s.b[2913] {
        let eq2_e971: f64 = (var_chnl_type * var_mult_inst);
        let eq2_e973: f64 = (eq2_e971 * p.p32);
        let eq2_e975: f64 = (eq2_e973 * s.v[841]);
        (eq2_e975, (eq2_e973 * s.dn[841][0]), (eq2_e973 * s.dn[841][1]), (eq2_e973 * s.dn[841][2]), (eq2_e973 * s.dn[841][3]), (eq2_e973 * s.dn[841][4]), (eq2_e973 * s.dn[841][5]), (eq2_e973 * s.dn[841][6]), (eq2_e973 * s.dn[841][7]), (eq2_e973 * s.dn[841][8]), (eq2_e973 * s.dn[841][9]), (eq2_e973 * s.dn[841][10]), (eq2_e973 * s.dn[841][11]), (eq2_e973 * s.dn[841][12]), (eq2_e973 * s.dn[841][13]), (eq2_e973 * s.dn[841][14]), (eq2_e973 * s.dn[841][15]), (eq2_e973 * s.dn[841][16]), (eq2_e973 * s.dn[841][17]), (eq2_e973 * s.dn[841][18]), (eq2_e973 * s.dn[841][19]), (eq2_e973 * s.dn[841][20]), (eq2_e973 * s.db[841][0]), (eq2_e973 * s.db[841][1]), (eq2_e973 * s.db[841][2]), (eq2_e973 * s.db[841][3]), (eq2_e973 * s.db[841][4]), (eq2_e973 * s.db[841][5]), (eq2_e973 * s.db[841][6]), (eq2_e973 * s.db[841][7]), (eq2_e973 * s.db[841][8]), (eq2_e973 * s.db[841][9]), (eq2_e973 * s.db[841][10]), (eq2_e973 * s.db[841][11]), (eq2_e973 * s.db[841][12]), (eq2_e973 * s.db[841][13]), (eq2_e973 * s.db[841][14]), (eq2_e973 * s.db[841][15]), (eq2_e973 * s.db[841][16]), (eq2_e973 * s.db[841][17]), (eq2_e973 * s.db[841][18]), (eq2_e973 * s.db[841][19]), (eq2_e973 * s.db[841][20]), (eq2_e973 * s.db[841][21]), (eq2_e973 * s.db[841][22]), (eq2_e973 * s.db[841][23]), (eq2_e973 * s.db[841][24]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e977;
        let eq2_node_derivatives: [f64; 21] = [eq2_e977_d_n0, eq2_e977_d_n1, eq2_e977_d_n2, eq2_e977_d_n3, eq2_e977_d_n4, eq2_e977_d_n5, eq2_e977_d_n6, eq2_e977_d_n7, eq2_e977_d_n8, eq2_e977_d_n9, eq2_e977_d_n10, eq2_e977_d_n11, eq2_e977_d_n12, eq2_e977_d_n13, eq2_e977_d_n14, eq2_e977_d_n15, eq2_e977_d_n16, eq2_e977_d_n17, eq2_e977_d_n18, eq2_e977_d_n19, eq2_e977_d_n20];
        let eq2_branch_derivatives: [f64; 25] = [eq2_e977_d_b0, eq2_e977_d_b1, eq2_e977_d_b2, eq2_e977_d_b3, eq2_e977_d_b4, eq2_e977_d_b5, eq2_e977_d_b6, eq2_e977_d_b7, eq2_e977_d_b8, eq2_e977_d_b9, eq2_e977_d_b10, eq2_e977_d_b11, eq2_e977_d_b12, eq2_e977_d_b13, eq2_e977_d_b14, eq2_e977_d_b15, eq2_e977_d_b16, eq2_e977_d_b17, eq2_e977_d_b18, eq2_e977_d_b19, eq2_e977_d_b20, eq2_e977_d_b21, eq2_e977_d_b22, eq2_e977_d_b23, eq2_e977_d_b24];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let (eq3_e987, eq3_e987_d_n0, eq3_e987_d_n1, eq3_e987_d_n2, eq3_e987_d_n3, eq3_e987_d_n4, eq3_e987_d_n5, eq3_e987_d_n6, eq3_e987_d_n7, eq3_e987_d_n8, eq3_e987_d_n9, eq3_e987_d_n10, eq3_e987_d_n11, eq3_e987_d_n12, eq3_e987_d_n13, eq3_e987_d_n14, eq3_e987_d_n15, eq3_e987_d_n16, eq3_e987_d_n17, eq3_e987_d_n18, eq3_e987_d_n19, eq3_e987_d_n20, eq3_e987_d_b0, eq3_e987_d_b1, eq3_e987_d_b2, eq3_e987_d_b3, eq3_e987_d_b4, eq3_e987_d_b5, eq3_e987_d_b6, eq3_e987_d_b7, eq3_e987_d_b8, eq3_e987_d_b9, eq3_e987_d_b10, eq3_e987_d_b11, eq3_e987_d_b12, eq3_e987_d_b13, eq3_e987_d_b14, eq3_e987_d_b15, eq3_e987_d_b16, eq3_e987_d_b17, eq3_e987_d_b18, eq3_e987_d_b19, eq3_e987_d_b20, eq3_e987_d_b21, eq3_e987_d_b22, eq3_e987_d_b23, eq3_e987_d_b24,) = {
    if s.b[2913] {
        let eq3_e981: f64 = (var_chnl_type * var_mult_inst);
        let eq3_e983: f64 = (eq3_e981 * p.p32);
        let eq3_e985: f64 = (eq3_e983 * s.v[842]);
        (eq3_e985, (eq3_e983 * s.dn[842][0]), (eq3_e983 * s.dn[842][1]), (eq3_e983 * s.dn[842][2]), (eq3_e983 * s.dn[842][3]), (eq3_e983 * s.dn[842][4]), (eq3_e983 * s.dn[842][5]), (eq3_e983 * s.dn[842][6]), (eq3_e983 * s.dn[842][7]), (eq3_e983 * s.dn[842][8]), (eq3_e983 * s.dn[842][9]), (eq3_e983 * s.dn[842][10]), (eq3_e983 * s.dn[842][11]), (eq3_e983 * s.dn[842][12]), (eq3_e983 * s.dn[842][13]), (eq3_e983 * s.dn[842][14]), (eq3_e983 * s.dn[842][15]), (eq3_e983 * s.dn[842][16]), (eq3_e983 * s.dn[842][17]), (eq3_e983 * s.dn[842][18]), (eq3_e983 * s.dn[842][19]), (eq3_e983 * s.dn[842][20]), (eq3_e983 * s.db[842][0]), (eq3_e983 * s.db[842][1]), (eq3_e983 * s.db[842][2]), (eq3_e983 * s.db[842][3]), (eq3_e983 * s.db[842][4]), (eq3_e983 * s.db[842][5]), (eq3_e983 * s.db[842][6]), (eq3_e983 * s.db[842][7]), (eq3_e983 * s.db[842][8]), (eq3_e983 * s.db[842][9]), (eq3_e983 * s.db[842][10]), (eq3_e983 * s.db[842][11]), (eq3_e983 * s.db[842][12]), (eq3_e983 * s.db[842][13]), (eq3_e983 * s.db[842][14]), (eq3_e983 * s.db[842][15]), (eq3_e983 * s.db[842][16]), (eq3_e983 * s.db[842][17]), (eq3_e983 * s.db[842][18]), (eq3_e983 * s.db[842][19]), (eq3_e983 * s.db[842][20]), (eq3_e983 * s.db[842][21]), (eq3_e983 * s.db[842][22]), (eq3_e983 * s.db[842][23]), (eq3_e983 * s.db[842][24]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e987;
        let eq3_node_derivatives: [f64; 21] = [eq3_e987_d_n0, eq3_e987_d_n1, eq3_e987_d_n2, eq3_e987_d_n3, eq3_e987_d_n4, eq3_e987_d_n5, eq3_e987_d_n6, eq3_e987_d_n7, eq3_e987_d_n8, eq3_e987_d_n9, eq3_e987_d_n10, eq3_e987_d_n11, eq3_e987_d_n12, eq3_e987_d_n13, eq3_e987_d_n14, eq3_e987_d_n15, eq3_e987_d_n16, eq3_e987_d_n17, eq3_e987_d_n18, eq3_e987_d_n19, eq3_e987_d_n20];
        let eq3_branch_derivatives: [f64; 25] = [eq3_e987_d_b0, eq3_e987_d_b1, eq3_e987_d_b2, eq3_e987_d_b3, eq3_e987_d_b4, eq3_e987_d_b5, eq3_e987_d_b6, eq3_e987_d_b7, eq3_e987_d_b8, eq3_e987_d_b9, eq3_e987_d_b10, eq3_e987_d_b11, eq3_e987_d_b12, eq3_e987_d_b13, eq3_e987_d_b14, eq3_e987_d_b15, eq3_e987_d_b16, eq3_e987_d_b17, eq3_e987_d_b18, eq3_e987_d_b19, eq3_e987_d_b20, eq3_e987_d_b21, eq3_e987_d_b22, eq3_e987_d_b23, eq3_e987_d_b24];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let (eq4_e998, eq4_e998_d_n0, eq4_e998_d_n1, eq4_e998_d_n2, eq4_e998_d_n3, eq4_e998_d_n4, eq4_e998_d_n5, eq4_e998_d_n6, eq4_e998_d_n7, eq4_e998_d_n8, eq4_e998_d_n9, eq4_e998_d_n10, eq4_e998_d_n11, eq4_e998_d_n12, eq4_e998_d_n13, eq4_e998_d_n14, eq4_e998_d_n15, eq4_e998_d_n16, eq4_e998_d_n17, eq4_e998_d_n18, eq4_e998_d_n19, eq4_e998_d_n20, eq4_e998_d_b0, eq4_e998_d_b1, eq4_e998_d_b2, eq4_e998_d_b3, eq4_e998_d_b4, eq4_e998_d_b5, eq4_e998_d_b6, eq4_e998_d_b7, eq4_e998_d_b8, eq4_e998_d_b9, eq4_e998_d_b10, eq4_e998_d_b11, eq4_e998_d_b12, eq4_e998_d_b13, eq4_e998_d_b14, eq4_e998_d_b15, eq4_e998_d_b16, eq4_e998_d_b17, eq4_e998_d_b18, eq4_e998_d_b19, eq4_e998_d_b20, eq4_e998_d_b21, eq4_e998_d_b22, eq4_e998_d_b23, eq4_e998_d_b24,) = {
    if (!s.b[2913]) {
        let eq4_e992: f64 = (var_chnl_type * var_mult_inst);
        let eq4_e994: f64 = (eq4_e992 * p.p32);
        let eq4_e996: f64 = (eq4_e994 * s.v[847]);
        (eq4_e996, (eq4_e994 * s.dn[847][0]), (eq4_e994 * s.dn[847][1]), (eq4_e994 * s.dn[847][2]), (eq4_e994 * s.dn[847][3]), (eq4_e994 * s.dn[847][4]), (eq4_e994 * s.dn[847][5]), (eq4_e994 * s.dn[847][6]), (eq4_e994 * s.dn[847][7]), (eq4_e994 * s.dn[847][8]), (eq4_e994 * s.dn[847][9]), (eq4_e994 * s.dn[847][10]), (eq4_e994 * s.dn[847][11]), (eq4_e994 * s.dn[847][12]), (eq4_e994 * s.dn[847][13]), (eq4_e994 * s.dn[847][14]), (eq4_e994 * s.dn[847][15]), (eq4_e994 * s.dn[847][16]), (eq4_e994 * s.dn[847][17]), (eq4_e994 * s.dn[847][18]), (eq4_e994 * s.dn[847][19]), (eq4_e994 * s.dn[847][20]), (eq4_e994 * s.db[847][0]), (eq4_e994 * s.db[847][1]), (eq4_e994 * s.db[847][2]), (eq4_e994 * s.db[847][3]), (eq4_e994 * s.db[847][4]), (eq4_e994 * s.db[847][5]), (eq4_e994 * s.db[847][6]), (eq4_e994 * s.db[847][7]), (eq4_e994 * s.db[847][8]), (eq4_e994 * s.db[847][9]), (eq4_e994 * s.db[847][10]), (eq4_e994 * s.db[847][11]), (eq4_e994 * s.db[847][12]), (eq4_e994 * s.db[847][13]), (eq4_e994 * s.db[847][14]), (eq4_e994 * s.db[847][15]), (eq4_e994 * s.db[847][16]), (eq4_e994 * s.db[847][17]), (eq4_e994 * s.db[847][18]), (eq4_e994 * s.db[847][19]), (eq4_e994 * s.db[847][20]), (eq4_e994 * s.db[847][21]), (eq4_e994 * s.db[847][22]), (eq4_e994 * s.db[847][23]), (eq4_e994 * s.db[847][24]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e998;
        let eq4_node_derivatives: [f64; 21] = [eq4_e998_d_n0, eq4_e998_d_n1, eq4_e998_d_n2, eq4_e998_d_n3, eq4_e998_d_n4, eq4_e998_d_n5, eq4_e998_d_n6, eq4_e998_d_n7, eq4_e998_d_n8, eq4_e998_d_n9, eq4_e998_d_n10, eq4_e998_d_n11, eq4_e998_d_n12, eq4_e998_d_n13, eq4_e998_d_n14, eq4_e998_d_n15, eq4_e998_d_n16, eq4_e998_d_n17, eq4_e998_d_n18, eq4_e998_d_n19, eq4_e998_d_n20];
        let eq4_branch_derivatives: [f64; 25] = [eq4_e998_d_b0, eq4_e998_d_b1, eq4_e998_d_b2, eq4_e998_d_b3, eq4_e998_d_b4, eq4_e998_d_b5, eq4_e998_d_b6, eq4_e998_d_b7, eq4_e998_d_b8, eq4_e998_d_b9, eq4_e998_d_b10, eq4_e998_d_b11, eq4_e998_d_b12, eq4_e998_d_b13, eq4_e998_d_b14, eq4_e998_d_b15, eq4_e998_d_b16, eq4_e998_d_b17, eq4_e998_d_b18, eq4_e998_d_b19, eq4_e998_d_b20, eq4_e998_d_b21, eq4_e998_d_b22, eq4_e998_d_b23, eq4_e998_d_b24];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e1011, eq5_e1011_d_n0, eq5_e1011_d_n1, eq5_e1011_d_n2, eq5_e1011_d_n3, eq5_e1011_d_n4, eq5_e1011_d_n5, eq5_e1011_d_n6, eq5_e1011_d_n7, eq5_e1011_d_n8, eq5_e1011_d_n9, eq5_e1011_d_n10, eq5_e1011_d_n11, eq5_e1011_d_n12, eq5_e1011_d_n13, eq5_e1011_d_n14, eq5_e1011_d_n15, eq5_e1011_d_n16, eq5_e1011_d_n17, eq5_e1011_d_n18, eq5_e1011_d_n19, eq5_e1011_d_n20, eq5_e1011_d_b0, eq5_e1011_d_b1, eq5_e1011_d_b2, eq5_e1011_d_b3, eq5_e1011_d_b4, eq5_e1011_d_b5, eq5_e1011_d_b6, eq5_e1011_d_b7, eq5_e1011_d_b8, eq5_e1011_d_b9, eq5_e1011_d_b10, eq5_e1011_d_b11, eq5_e1011_d_b12, eq5_e1011_d_b13, eq5_e1011_d_b14, eq5_e1011_d_b15, eq5_e1011_d_b16, eq5_e1011_d_b17, eq5_e1011_d_b18, eq5_e1011_d_b19, eq5_e1011_d_b20, eq5_e1011_d_b21, eq5_e1011_d_b22, eq5_e1011_d_b23, eq5_e1011_d_b24,) = {
    if (!s.b[2913]) {
        let eq5_e1003: f64 = (var_chnl_type * var_mult_inst);
        let eq5_e1005: f64 = (eq5_e1003 * p.p32);
        let eq5_e1008: f64 = (s.v[838] + s.v[846]);
        let eq5_e1009: f64 = (eq5_e1005 * eq5_e1008);
        let eq5_e1009_d_n0: f64 = (eq5_e1005 * __rspice_deriv_cse_0);
        let eq5_e1009_d_n1: f64 = (eq5_e1005 * __rspice_deriv_cse_1);
        let eq5_e1009_d_n2: f64 = (eq5_e1005 * __rspice_deriv_cse_2);
        let eq5_e1009_d_n3: f64 = (eq5_e1005 * __rspice_deriv_cse_3);
        let eq5_e1009_d_n4: f64 = (eq5_e1005 * __rspice_deriv_cse_4);
        let eq5_e1009_d_n5: f64 = (eq5_e1005 * __rspice_deriv_cse_5);
        let eq5_e1009_d_n6: f64 = (eq5_e1005 * __rspice_deriv_cse_6);
        let eq5_e1009_d_n7: f64 = (eq5_e1005 * __rspice_deriv_cse_7);
        let eq5_e1009_d_n8: f64 = (eq5_e1005 * __rspice_deriv_cse_8);
        let eq5_e1009_d_n9: f64 = (eq5_e1005 * __rspice_deriv_cse_9);
        let eq5_e1009_d_n10: f64 = (eq5_e1005 * __rspice_deriv_cse_10);
        let eq5_e1009_d_n11: f64 = (eq5_e1005 * __rspice_deriv_cse_11);
        let eq5_e1009_d_n12: f64 = (eq5_e1005 * __rspice_deriv_cse_12);
        let eq5_e1009_d_n13: f64 = (eq5_e1005 * __rspice_deriv_cse_13);
        let eq5_e1009_d_n14: f64 = (eq5_e1005 * __rspice_deriv_cse_14);
        let eq5_e1009_d_n15: f64 = (eq5_e1005 * __rspice_deriv_cse_15);
        let eq5_e1009_d_n16: f64 = (eq5_e1005 * __rspice_deriv_cse_16);
        let eq5_e1009_d_n17: f64 = (eq5_e1005 * __rspice_deriv_cse_17);
        let eq5_e1009_d_n18: f64 = (eq5_e1005 * __rspice_deriv_cse_18);
        let eq5_e1009_d_n19: f64 = (eq5_e1005 * __rspice_deriv_cse_19);
        let eq5_e1009_d_n20: f64 = (eq5_e1005 * __rspice_deriv_cse_20);
        let eq5_e1009_d_b0: f64 = (eq5_e1005 * __rspice_deriv_cse_21);
        let eq5_e1009_d_b1: f64 = (eq5_e1005 * __rspice_deriv_cse_22);
        let eq5_e1009_d_b2: f64 = (eq5_e1005 * __rspice_deriv_cse_23);
        let eq5_e1009_d_b3: f64 = (eq5_e1005 * __rspice_deriv_cse_24);
        let eq5_e1009_d_b4: f64 = (eq5_e1005 * __rspice_deriv_cse_25);
        let eq5_e1009_d_b5: f64 = (eq5_e1005 * __rspice_deriv_cse_26);
        let eq5_e1009_d_b6: f64 = (eq5_e1005 * __rspice_deriv_cse_27);
        let eq5_e1009_d_b7: f64 = (eq5_e1005 * __rspice_deriv_cse_28);
        let eq5_e1009_d_b8: f64 = (eq5_e1005 * __rspice_deriv_cse_29);
        let eq5_e1009_d_b9: f64 = (eq5_e1005 * __rspice_deriv_cse_30);
        let eq5_e1009_d_b10: f64 = (eq5_e1005 * __rspice_deriv_cse_31);
        let eq5_e1009_d_b11: f64 = (eq5_e1005 * __rspice_deriv_cse_32);
        let eq5_e1009_d_b12: f64 = (eq5_e1005 * __rspice_deriv_cse_33);
        let eq5_e1009_d_b13: f64 = (eq5_e1005 * __rspice_deriv_cse_34);
        let eq5_e1009_d_b14: f64 = (eq5_e1005 * __rspice_deriv_cse_35);
        let eq5_e1009_d_b15: f64 = (eq5_e1005 * __rspice_deriv_cse_36);
        let eq5_e1009_d_b16: f64 = (eq5_e1005 * __rspice_deriv_cse_37);
        let eq5_e1009_d_b17: f64 = (eq5_e1005 * __rspice_deriv_cse_38);
        let eq5_e1009_d_b18: f64 = (eq5_e1005 * __rspice_deriv_cse_39);
        let eq5_e1009_d_b19: f64 = (eq5_e1005 * __rspice_deriv_cse_40);
        let eq5_e1009_d_b20: f64 = (eq5_e1005 * __rspice_deriv_cse_41);
        let eq5_e1009_d_b21: f64 = (eq5_e1005 * __rspice_deriv_cse_42);
        let eq5_e1009_d_b22: f64 = (eq5_e1005 * __rspice_deriv_cse_43);
        let eq5_e1009_d_b23: f64 = (eq5_e1005 * __rspice_deriv_cse_44);
        let eq5_e1009_d_b24: f64 = (eq5_e1005 * __rspice_deriv_cse_45);
        (eq5_e1009, eq5_e1009_d_n0, eq5_e1009_d_n1, eq5_e1009_d_n2, eq5_e1009_d_n3, eq5_e1009_d_n4, eq5_e1009_d_n5, eq5_e1009_d_n6, eq5_e1009_d_n7, eq5_e1009_d_n8, eq5_e1009_d_n9, eq5_e1009_d_n10, eq5_e1009_d_n11, eq5_e1009_d_n12, eq5_e1009_d_n13, eq5_e1009_d_n14, eq5_e1009_d_n15, eq5_e1009_d_n16, eq5_e1009_d_n17, eq5_e1009_d_n18, eq5_e1009_d_n19, eq5_e1009_d_n20, eq5_e1009_d_b0, eq5_e1009_d_b1, eq5_e1009_d_b2, eq5_e1009_d_b3, eq5_e1009_d_b4, eq5_e1009_d_b5, eq5_e1009_d_b6, eq5_e1009_d_b7, eq5_e1009_d_b8, eq5_e1009_d_b9, eq5_e1009_d_b10, eq5_e1009_d_b11, eq5_e1009_d_b12, eq5_e1009_d_b13, eq5_e1009_d_b14, eq5_e1009_d_b15, eq5_e1009_d_b16, eq5_e1009_d_b17, eq5_e1009_d_b18, eq5_e1009_d_b19, eq5_e1009_d_b20, eq5_e1009_d_b21, eq5_e1009_d_b22, eq5_e1009_d_b23, eq5_e1009_d_b24,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e1011;
        let eq5_node_derivatives: [f64; 21] = [eq5_e1011_d_n0, eq5_e1011_d_n1, eq5_e1011_d_n2, eq5_e1011_d_n3, eq5_e1011_d_n4, eq5_e1011_d_n5, eq5_e1011_d_n6, eq5_e1011_d_n7, eq5_e1011_d_n8, eq5_e1011_d_n9, eq5_e1011_d_n10, eq5_e1011_d_n11, eq5_e1011_d_n12, eq5_e1011_d_n13, eq5_e1011_d_n14, eq5_e1011_d_n15, eq5_e1011_d_n16, eq5_e1011_d_n17, eq5_e1011_d_n18, eq5_e1011_d_n19, eq5_e1011_d_n20];
        let eq5_branch_derivatives: [f64; 25] = [eq5_e1011_d_b0, eq5_e1011_d_b1, eq5_e1011_d_b2, eq5_e1011_d_b3, eq5_e1011_d_b4, eq5_e1011_d_b5, eq5_e1011_d_b6, eq5_e1011_d_b7, eq5_e1011_d_b8, eq5_e1011_d_b9, eq5_e1011_d_b10, eq5_e1011_d_b11, eq5_e1011_d_b12, eq5_e1011_d_b13, eq5_e1011_d_b14, eq5_e1011_d_b15, eq5_e1011_d_b16, eq5_e1011_d_b17, eq5_e1011_d_b18, eq5_e1011_d_b19, eq5_e1011_d_b20, eq5_e1011_d_b21, eq5_e1011_d_b22, eq5_e1011_d_b23, eq5_e1011_d_b24];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let (eq6_e1022, eq6_e1022_d_n0, eq6_e1022_d_n1, eq6_e1022_d_n2, eq6_e1022_d_n3, eq6_e1022_d_n4, eq6_e1022_d_n5, eq6_e1022_d_n6, eq6_e1022_d_n7, eq6_e1022_d_n8, eq6_e1022_d_n9, eq6_e1022_d_n10, eq6_e1022_d_n11, eq6_e1022_d_n12, eq6_e1022_d_n13, eq6_e1022_d_n14, eq6_e1022_d_n15, eq6_e1022_d_n16, eq6_e1022_d_n17, eq6_e1022_d_n18, eq6_e1022_d_n19, eq6_e1022_d_n20, eq6_e1022_d_b0, eq6_e1022_d_b1, eq6_e1022_d_b2, eq6_e1022_d_b3, eq6_e1022_d_b4, eq6_e1022_d_b5, eq6_e1022_d_b6, eq6_e1022_d_b7, eq6_e1022_d_b8, eq6_e1022_d_b9, eq6_e1022_d_b10, eq6_e1022_d_b11, eq6_e1022_d_b12, eq6_e1022_d_b13, eq6_e1022_d_b14, eq6_e1022_d_b15, eq6_e1022_d_b16, eq6_e1022_d_b17, eq6_e1022_d_b18, eq6_e1022_d_b19, eq6_e1022_d_b20, eq6_e1022_d_b21, eq6_e1022_d_b22, eq6_e1022_d_b23, eq6_e1022_d_b24,) = {
    if (!s.b[2913]) {
        let eq6_e1016: f64 = (var_chnl_type * var_mult_inst);
        let eq6_e1018: f64 = (eq6_e1016 * p.p32);
        let eq6_e1020: f64 = (eq6_e1018 * s.v[841]);
        (eq6_e1020, (eq6_e1018 * s.dn[841][0]), (eq6_e1018 * s.dn[841][1]), (eq6_e1018 * s.dn[841][2]), (eq6_e1018 * s.dn[841][3]), (eq6_e1018 * s.dn[841][4]), (eq6_e1018 * s.dn[841][5]), (eq6_e1018 * s.dn[841][6]), (eq6_e1018 * s.dn[841][7]), (eq6_e1018 * s.dn[841][8]), (eq6_e1018 * s.dn[841][9]), (eq6_e1018 * s.dn[841][10]), (eq6_e1018 * s.dn[841][11]), (eq6_e1018 * s.dn[841][12]), (eq6_e1018 * s.dn[841][13]), (eq6_e1018 * s.dn[841][14]), (eq6_e1018 * s.dn[841][15]), (eq6_e1018 * s.dn[841][16]), (eq6_e1018 * s.dn[841][17]), (eq6_e1018 * s.dn[841][18]), (eq6_e1018 * s.dn[841][19]), (eq6_e1018 * s.dn[841][20]), (eq6_e1018 * s.db[841][0]), (eq6_e1018 * s.db[841][1]), (eq6_e1018 * s.db[841][2]), (eq6_e1018 * s.db[841][3]), (eq6_e1018 * s.db[841][4]), (eq6_e1018 * s.db[841][5]), (eq6_e1018 * s.db[841][6]), (eq6_e1018 * s.db[841][7]), (eq6_e1018 * s.db[841][8]), (eq6_e1018 * s.db[841][9]), (eq6_e1018 * s.db[841][10]), (eq6_e1018 * s.db[841][11]), (eq6_e1018 * s.db[841][12]), (eq6_e1018 * s.db[841][13]), (eq6_e1018 * s.db[841][14]), (eq6_e1018 * s.db[841][15]), (eq6_e1018 * s.db[841][16]), (eq6_e1018 * s.db[841][17]), (eq6_e1018 * s.db[841][18]), (eq6_e1018 * s.db[841][19]), (eq6_e1018 * s.db[841][20]), (eq6_e1018 * s.db[841][21]), (eq6_e1018 * s.db[841][22]), (eq6_e1018 * s.db[841][23]), (eq6_e1018 * s.db[841][24]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e1022;
        let eq6_node_derivatives: [f64; 21] = [eq6_e1022_d_n0, eq6_e1022_d_n1, eq6_e1022_d_n2, eq6_e1022_d_n3, eq6_e1022_d_n4, eq6_e1022_d_n5, eq6_e1022_d_n6, eq6_e1022_d_n7, eq6_e1022_d_n8, eq6_e1022_d_n9, eq6_e1022_d_n10, eq6_e1022_d_n11, eq6_e1022_d_n12, eq6_e1022_d_n13, eq6_e1022_d_n14, eq6_e1022_d_n15, eq6_e1022_d_n16, eq6_e1022_d_n17, eq6_e1022_d_n18, eq6_e1022_d_n19, eq6_e1022_d_n20];
        let eq6_branch_derivatives: [f64; 25] = [eq6_e1022_d_b0, eq6_e1022_d_b1, eq6_e1022_d_b2, eq6_e1022_d_b3, eq6_e1022_d_b4, eq6_e1022_d_b5, eq6_e1022_d_b6, eq6_e1022_d_b7, eq6_e1022_d_b8, eq6_e1022_d_b9, eq6_e1022_d_b10, eq6_e1022_d_b11, eq6_e1022_d_b12, eq6_e1022_d_b13, eq6_e1022_d_b14, eq6_e1022_d_b15, eq6_e1022_d_b16, eq6_e1022_d_b17, eq6_e1022_d_b18, eq6_e1022_d_b19, eq6_e1022_d_b20, eq6_e1022_d_b21, eq6_e1022_d_b22, eq6_e1022_d_b23, eq6_e1022_d_b24];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let (eq7_e1033, eq7_e1033_d_n0, eq7_e1033_d_n1, eq7_e1033_d_n2, eq7_e1033_d_n3, eq7_e1033_d_n4, eq7_e1033_d_n5, eq7_e1033_d_n6, eq7_e1033_d_n7, eq7_e1033_d_n8, eq7_e1033_d_n9, eq7_e1033_d_n10, eq7_e1033_d_n11, eq7_e1033_d_n12, eq7_e1033_d_n13, eq7_e1033_d_n14, eq7_e1033_d_n15, eq7_e1033_d_n16, eq7_e1033_d_n17, eq7_e1033_d_n18, eq7_e1033_d_n19, eq7_e1033_d_n20, eq7_e1033_d_b0, eq7_e1033_d_b1, eq7_e1033_d_b2, eq7_e1033_d_b3, eq7_e1033_d_b4, eq7_e1033_d_b5, eq7_e1033_d_b6, eq7_e1033_d_b7, eq7_e1033_d_b8, eq7_e1033_d_b9, eq7_e1033_d_b10, eq7_e1033_d_b11, eq7_e1033_d_b12, eq7_e1033_d_b13, eq7_e1033_d_b14, eq7_e1033_d_b15, eq7_e1033_d_b16, eq7_e1033_d_b17, eq7_e1033_d_b18, eq7_e1033_d_b19, eq7_e1033_d_b20, eq7_e1033_d_b21, eq7_e1033_d_b22, eq7_e1033_d_b23, eq7_e1033_d_b24,) = {
    if (!s.b[2913]) {
        let eq7_e1027: f64 = (var_chnl_type * var_mult_inst);
        let eq7_e1029: f64 = (eq7_e1027 * p.p32);
        let eq7_e1031: f64 = (eq7_e1029 * s.v[842]);
        (eq7_e1031, (eq7_e1029 * s.dn[842][0]), (eq7_e1029 * s.dn[842][1]), (eq7_e1029 * s.dn[842][2]), (eq7_e1029 * s.dn[842][3]), (eq7_e1029 * s.dn[842][4]), (eq7_e1029 * s.dn[842][5]), (eq7_e1029 * s.dn[842][6]), (eq7_e1029 * s.dn[842][7]), (eq7_e1029 * s.dn[842][8]), (eq7_e1029 * s.dn[842][9]), (eq7_e1029 * s.dn[842][10]), (eq7_e1029 * s.dn[842][11]), (eq7_e1029 * s.dn[842][12]), (eq7_e1029 * s.dn[842][13]), (eq7_e1029 * s.dn[842][14]), (eq7_e1029 * s.dn[842][15]), (eq7_e1029 * s.dn[842][16]), (eq7_e1029 * s.dn[842][17]), (eq7_e1029 * s.dn[842][18]), (eq7_e1029 * s.dn[842][19]), (eq7_e1029 * s.dn[842][20]), (eq7_e1029 * s.db[842][0]), (eq7_e1029 * s.db[842][1]), (eq7_e1029 * s.db[842][2]), (eq7_e1029 * s.db[842][3]), (eq7_e1029 * s.db[842][4]), (eq7_e1029 * s.db[842][5]), (eq7_e1029 * s.db[842][6]), (eq7_e1029 * s.db[842][7]), (eq7_e1029 * s.db[842][8]), (eq7_e1029 * s.db[842][9]), (eq7_e1029 * s.db[842][10]), (eq7_e1029 * s.db[842][11]), (eq7_e1029 * s.db[842][12]), (eq7_e1029 * s.db[842][13]), (eq7_e1029 * s.db[842][14]), (eq7_e1029 * s.db[842][15]), (eq7_e1029 * s.db[842][16]), (eq7_e1029 * s.db[842][17]), (eq7_e1029 * s.db[842][18]), (eq7_e1029 * s.db[842][19]), (eq7_e1029 * s.db[842][20]), (eq7_e1029 * s.db[842][21]), (eq7_e1029 * s.db[842][22]), (eq7_e1029 * s.db[842][23]), (eq7_e1029 * s.db[842][24]),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e1033;
        let eq7_node_derivatives: [f64; 21] = [eq7_e1033_d_n0, eq7_e1033_d_n1, eq7_e1033_d_n2, eq7_e1033_d_n3, eq7_e1033_d_n4, eq7_e1033_d_n5, eq7_e1033_d_n6, eq7_e1033_d_n7, eq7_e1033_d_n8, eq7_e1033_d_n9, eq7_e1033_d_n10, eq7_e1033_d_n11, eq7_e1033_d_n12, eq7_e1033_d_n13, eq7_e1033_d_n14, eq7_e1033_d_n15, eq7_e1033_d_n16, eq7_e1033_d_n17, eq7_e1033_d_n18, eq7_e1033_d_n19, eq7_e1033_d_n20];
        let eq7_branch_derivatives: [f64; 25] = [eq7_e1033_d_b0, eq7_e1033_d_b1, eq7_e1033_d_b2, eq7_e1033_d_b3, eq7_e1033_d_b4, eq7_e1033_d_b5, eq7_e1033_d_b6, eq7_e1033_d_b7, eq7_e1033_d_b8, eq7_e1033_d_b9, eq7_e1033_d_b10, eq7_e1033_d_b11, eq7_e1033_d_b12, eq7_e1033_d_b13, eq7_e1033_d_b14, eq7_e1033_d_b15, eq7_e1033_d_b16, eq7_e1033_d_b17, eq7_e1033_d_b18, eq7_e1033_d_b19, eq7_e1033_d_b20, eq7_e1033_d_b21, eq7_e1033_d_b22, eq7_e1033_d_b23, eq7_e1033_d_b24];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let eq8_e1036: f64 = (var_chnl_type * var_mult_inst);
        let eq8_e1038: f64 = (eq8_e1036 * p.p32);
        let eq8_e1040: f64 = (eq8_e1038 * s.v[843]);
        let eq8_value: f64 = eq8_e1040;
        stamper.stamp_current_dense_local(
            Some(5),
            Some(8),
            multiplicity * (eq8_value),
            &s.dn[843],
            &s.db[843],
            (multiplicity) * (eq8_e1038),
        );
        let eq9_e1043: f64 = (var_chnl_type * var_mult_inst);
        let eq9_e1045: f64 = (eq9_e1043 * p.p32);
        let eq9_e1047: f64 = (eq9_e1045 * s.v[839]);
        let eq9_value: f64 = eq9_e1047;
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq9_value),
            &s.dn[839],
            &s.db[839],
            (multiplicity) * (eq9_e1045),
        );
        let eq10_e1050: f64 = (var_chnl_type * var_mult_inst);
        let eq10_e1052: f64 = (eq10_e1050 * p.p32);
        let eq10_e1054: f64 = (eq10_e1052 * s.v[840]);
        let eq10_value: f64 = eq10_e1054;
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq10_value),
            &s.dn[840],
            &s.db[840],
            (multiplicity) * (eq10_e1052),
        );
        let eq11_e1057: f64 = (var_chnl_type * var_mult_inst);
        let eq11_e1059: f64 = (eq11_e1057 * p.p32);
        let eq11_e1061: f64 = (eq11_e1059 * s.v[844]);
        let eq11_value: f64 = eq11_e1061;
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq11_value),
            &s.dn[844],
            &s.db[844],
            (multiplicity) * (eq11_e1059),
        );
        let eq12_e1064: f64 = (var_chnl_type * var_mult_inst);
        let eq12_e1066: f64 = (eq12_e1064 * p.p32);
        let eq12_e1068: f64 = (eq12_e1066 * s.v[845]);
        let eq12_value: f64 = eq12_e1068;
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq12_value),
            &s.dn[845],
            &s.db[845],
            (multiplicity) * (eq12_e1066),
        );
        let eq13_e1071: f64 = (var_chnl_type * var_mult_inst);
        let eq13_e1073: f64 = (eq13_e1071 * p.p32);
        let eq13_e1075: f64 = (eq13_e1073 * var_ijun_s);
        let eq13_e1075_d_n0: f64 = (eq13_e1073 * var_ijun_s_dn0);
        let eq13_e1075_d_n1: f64 = (eq13_e1073 * var_ijun_s_dn1);
        let eq13_e1075_d_n2: f64 = (eq13_e1073 * var_ijun_s_dn2);
        let eq13_e1075_d_n3: f64 = (eq13_e1073 * var_ijun_s_dn3);
        let eq13_e1075_d_n4: f64 = (eq13_e1073 * var_ijun_s_dn4);
        let eq13_e1075_d_n5: f64 = (eq13_e1073 * var_ijun_s_dn5);
        let eq13_e1075_d_n6: f64 = (eq13_e1073 * var_ijun_s_dn6);
        let eq13_e1075_d_n7: f64 = (eq13_e1073 * var_ijun_s_dn7);
        let eq13_e1075_d_n8: f64 = (eq13_e1073 * var_ijun_s_dn8);
        let eq13_e1075_d_n9: f64 = (eq13_e1073 * var_ijun_s_dn9);
        let eq13_e1075_d_n10: f64 = (eq13_e1073 * var_ijun_s_dn10);
        let eq13_e1075_d_n11: f64 = (eq13_e1073 * var_ijun_s_dn11);
        let eq13_e1075_d_n12: f64 = (eq13_e1073 * var_ijun_s_dn12);
        let eq13_e1075_d_n13: f64 = (eq13_e1073 * var_ijun_s_dn13);
        let eq13_e1075_d_n14: f64 = (eq13_e1073 * var_ijun_s_dn14);
        let eq13_e1075_d_n15: f64 = (eq13_e1073 * var_ijun_s_dn15);
        let eq13_e1075_d_n16: f64 = (eq13_e1073 * var_ijun_s_dn16);
        let eq13_e1075_d_n17: f64 = (eq13_e1073 * var_ijun_s_dn17);
        let eq13_e1075_d_n18: f64 = (eq13_e1073 * var_ijun_s_dn18);
        let eq13_e1075_d_n19: f64 = (eq13_e1073 * var_ijun_s_dn19);
        let eq13_e1075_d_n20: f64 = (eq13_e1073 * var_ijun_s_dn20);
        let eq13_e1075_d_b0: f64 = (eq13_e1073 * var_ijun_s_db0);
        let eq13_e1075_d_b1: f64 = (eq13_e1073 * var_ijun_s_db1);
        let eq13_e1075_d_b2: f64 = (eq13_e1073 * var_ijun_s_db2);
        let eq13_e1075_d_b3: f64 = (eq13_e1073 * var_ijun_s_db3);
        let eq13_e1075_d_b4: f64 = (eq13_e1073 * var_ijun_s_db4);
        let eq13_e1075_d_b5: f64 = (eq13_e1073 * var_ijun_s_db5);
        let eq13_e1075_d_b6: f64 = (eq13_e1073 * var_ijun_s_db6);
        let eq13_e1075_d_b7: f64 = (eq13_e1073 * var_ijun_s_db7);
        let eq13_e1075_d_b8: f64 = (eq13_e1073 * var_ijun_s_db8);
        let eq13_e1075_d_b9: f64 = (eq13_e1073 * var_ijun_s_db9);
        let eq13_e1075_d_b10: f64 = (eq13_e1073 * var_ijun_s_db10);
        let eq13_e1075_d_b11: f64 = (eq13_e1073 * var_ijun_s_db11);
        let eq13_e1075_d_b12: f64 = (eq13_e1073 * var_ijun_s_db12);
        let eq13_e1075_d_b13: f64 = (eq13_e1073 * var_ijun_s_db13);
        let eq13_e1075_d_b14: f64 = (eq13_e1073 * var_ijun_s_db14);
        let eq13_e1075_d_b15: f64 = (eq13_e1073 * var_ijun_s_db15);
        let eq13_e1075_d_b16: f64 = (eq13_e1073 * var_ijun_s_db16);
        let eq13_e1075_d_b17: f64 = (eq13_e1073 * var_ijun_s_db17);
        let eq13_e1075_d_b18: f64 = (eq13_e1073 * var_ijun_s_db18);
        let eq13_e1075_d_b19: f64 = (eq13_e1073 * var_ijun_s_db19);
        let eq13_e1075_d_b20: f64 = (eq13_e1073 * var_ijun_s_db20);
        let eq13_e1075_d_b21: f64 = (eq13_e1073 * var_ijun_s_db21);
        let eq13_e1075_d_b22: f64 = (eq13_e1073 * var_ijun_s_db22);
        let eq13_e1075_d_b23: f64 = (eq13_e1073 * var_ijun_s_db23);
        let eq13_e1075_d_b24: f64 = (eq13_e1073 * var_ijun_s_db24);
        let eq13_value: f64 = eq13_e1075;
        let eq13_node_derivatives: [f64; 21] = [eq13_e1075_d_n0, eq13_e1075_d_n1, eq13_e1075_d_n2, eq13_e1075_d_n3, eq13_e1075_d_n4, eq13_e1075_d_n5, eq13_e1075_d_n6, eq13_e1075_d_n7, eq13_e1075_d_n8, eq13_e1075_d_n9, eq13_e1075_d_n10, eq13_e1075_d_n11, eq13_e1075_d_n12, eq13_e1075_d_n13, eq13_e1075_d_n14, eq13_e1075_d_n15, eq13_e1075_d_n16, eq13_e1075_d_n17, eq13_e1075_d_n18, eq13_e1075_d_n19, eq13_e1075_d_n20];
        let eq13_branch_derivatives: [f64; 25] = [eq13_e1075_d_b0, eq13_e1075_d_b1, eq13_e1075_d_b2, eq13_e1075_d_b3, eq13_e1075_d_b4, eq13_e1075_d_b5, eq13_e1075_d_b6, eq13_e1075_d_b7, eq13_e1075_d_b8, eq13_e1075_d_b9, eq13_e1075_d_b10, eq13_e1075_d_b11, eq13_e1075_d_b12, eq13_e1075_d_b13, eq13_e1075_d_b14, eq13_e1075_d_b15, eq13_e1075_d_b16, eq13_e1075_d_b17, eq13_e1075_d_b18, eq13_e1075_d_b19, eq13_e1075_d_b20, eq13_e1075_d_b21, eq13_e1075_d_b22, eq13_e1075_d_b23, eq13_e1075_d_b24];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(6),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
    }
}

#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_144(
        s: &mut Scratch,
    ) {
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2847])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1961)), (-1.0)));}
        s.b[2850] = (s.v[1961] > s.v[1933]);s.store_scalar(2850, if s.b[2850] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2847])) && s.b[2850]) {s.store_neg(1996, 1996);}
        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) {s.store_add_scaled_product_right_sub(1943, 1996, (-1.0), 1937, 1890, 1961, -1.0);s.store_add_scaled_product_mixed_iai(1962, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.5, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2851] = (((s.v[1962]) as f64).abs() <= s.v[1933]);s.store_scalar(2851, if s.b[2851] { 1.0 } else { 0.0 });
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && s.b[2851]) {s.store_mul_ad_affine_product_rhs(1996, 1962, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1962), 1.0, A::scale(s.ad_value(1962), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2852] = ((((-s.v[1962])) as f64).abs() < 230.25850929940458);s.store_scalar(2852, if s.b[2852] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2851])) && s.b[2852]) {s.store_exp_neg_input(2027, 1962);}
        s.b[2853] = ((-s.v[1962]) < 0.0);s.store_scalar(2853, if s.b[2853] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2851])) && (!s.b[2852])) && s.b[2853]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1962)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2851])) && (!s.b[2852])) && (!s.b[2853])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1962)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2851])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1962)), (-1.0)));}
        s.b[2854] = (s.v[1962] > s.v[1933]);s.store_scalar(2854, if s.b[2854] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2851])) && s.b[2854]) {s.store_neg(1996, 1996);}
        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) {s.store_add_scaled_product_right_sub(1944, 1996, (-1.0), 1937, 1890, 1962, -1.0);s.store_add_scaled_product_mixed_iai(1963, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.6666666666666666, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2855] = (((s.v[1963]) as f64).abs() <= s.v[1933]);s.store_scalar(2855, if s.b[2855] { 1.0 } else { 0.0 });
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && s.b[2855]) {s.store_mul_ad_affine_product_rhs(1996, 1963, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1963), 1.0, A::scale(s.ad_value(1963), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2856] = ((((-s.v[1963])) as f64).abs() < 230.25850929940458);s.store_scalar(2856, if s.b[2856] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2855])) && s.b[2856]) {s.store_exp_neg_input(2027, 1963);}
        s.b[2857] = ((-s.v[1963]) < 0.0);s.store_scalar(2857, if s.b[2857] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2855])) && (!s.b[2856])) && s.b[2857]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1963)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2855])) && (!s.b[2856])) && (!s.b[2857])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1963)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2855])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1963)), (-1.0)));}
        s.b[2858] = (s.v[1963] > s.v[1933]);s.store_scalar(2858, if s.b[2858] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2855])) && s.b[2858]) {s.store_neg(1996, 1996);}
        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) {s.store_add_scaled_product_right_sub(1945, 1996, (-1.0), 1937, 1890, 1963, -1.0);s.store_add_scaled_product_mixed_iai(1964, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.8333333333333333, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2859] = (((s.v[1964]) as f64).abs() <= s.v[1933]);s.store_scalar(2859, if s.b[2859] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_145(
        s: &mut Scratch,
    ) {
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && s.b[2859]) {s.store_mul_ad_affine_product_rhs(1996, 1964, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1964), 1.0, A::scale(s.ad_value(1964), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2860] = ((((-s.v[1964])) as f64).abs() < 230.25850929940458);s.store_scalar(2860, if s.b[2860] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2859])) && s.b[2860]) {s.store_exp_neg_input(2027, 1964);}
        s.b[2861] = ((-s.v[1964]) < 0.0);s.store_scalar(2861, if s.b[2861] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2859])) && (!s.b[2860])) && s.b[2861]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1964)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2859])) && (!s.b[2860])) && (!s.b[2861])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1964)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2859])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1964)), (-1.0)));}
        s.b[2862] = (s.v[1964] > s.v[1933]);s.store_scalar(2862, if s.b[2862] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2859])) && s.b[2862]) {s.store_neg(1996, 1996);}
        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) {s.store_add_scaled_product_right_sub(1946, 1996, (-1.0), 1937, 1890, 1964, -1.0);}
        s.b[2863] = (s.v[831] < 0.0);s.store_scalar(2863, if s.b[2863] { 1.0 } else { 0.0 });
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && s.b[2863]) {s.copy_ad(2027, 1942);s.copy_ad(1942, 1946);s.copy_ad(1946, 2027);s.copy_ad(2027, 1943);s.copy_ad(1943, 1945);s.copy_ad(1945, 2027);}
        s.b[2864] = (s.v[1] == 9.0);s.store_scalar(2864, if s.b[2864] { 1.0 } else { 0.0 });
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {s.store_add_scaled_product_mixed_iai(1960, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.1, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2865] = (((s.v[1960]) as f64).abs() <= s.v[1933]);s.store_scalar(2865, if s.b[2865] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2865]) {s.store_mul_ad_affine_product_rhs(1996, 1960, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1960), 1.0, A::scale(s.ad_value(1960), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2866] = ((((-s.v[1960])) as f64).abs() < 230.25850929940458);s.store_scalar(2866, if s.b[2866] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2865])) && s.b[2866]) {s.store_exp_neg_input(2027, 1960);}
        s.b[2867] = ((-s.v[1960]) < 0.0);s.store_scalar(2867, if s.b[2867] { 1.0 } else { 0.0 });
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2865])) && (!s.b[2866])) && s.b[2867]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1960)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2865])) && (!s.b[2866])) && (!s.b[2867])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1960)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2865])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0)));}
        s.b[2868] = (s.v[1960] > s.v[1933]);s.store_scalar(2868, if s.b[2868] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2865])) && s.b[2868]) {s.store_neg(1996, 1996);}
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {s.store_add_scaled_product_right_sub(1942, 1996, (-1.0), 1937, 1890, 1960, -1.0);s.store_add_scaled_product_mixed_iai(1961, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.2, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2869] = (((s.v[1961]) as f64).abs() <= s.v[1933]);s.store_scalar(2869, if s.b[2869] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2869]) {s.store_mul_ad_affine_product_rhs(1996, 1961, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1961), 1.0, A::scale(s.ad_value(1961), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2870] = ((((-s.v[1961])) as f64).abs() < 230.25850929940458);s.store_scalar(2870, if s.b[2870] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2869])) && s.b[2870]) {s.store_exp_neg_input(2027, 1961);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_146(
        s: &mut Scratch,
    ) {
        s.b[2871] = ((-s.v[1961]) < 0.0);s.store_scalar(2871, if s.b[2871] { 1.0 } else { 0.0 });
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2869])) && (!s.b[2870])) && s.b[2871]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1961)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2869])) && (!s.b[2870])) && (!s.b[2871])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1961)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2869])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1961)), (-1.0)));}
        s.b[2872] = (s.v[1961] > s.v[1933]);s.store_scalar(2872, if s.b[2872] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2869])) && s.b[2872]) {s.store_neg(1996, 1996);}
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {s.store_add_scaled_product_right_sub(1943, 1996, (-1.0), 1937, 1890, 1961, -1.0);s.store_add_scaled_product_mixed_iai(1962, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.3, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2873] = (((s.v[1962]) as f64).abs() <= s.v[1933]);s.store_scalar(2873, if s.b[2873] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2873]) {s.store_mul_ad_affine_product_rhs(1996, 1962, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1962), 1.0, A::scale(s.ad_value(1962), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2874] = ((((-s.v[1962])) as f64).abs() < 230.25850929940458);s.store_scalar(2874, if s.b[2874] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2873])) && s.b[2874]) {s.store_exp_neg_input(2027, 1962);}
        s.b[2875] = ((-s.v[1962]) < 0.0);s.store_scalar(2875, if s.b[2875] { 1.0 } else { 0.0 });
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2873])) && (!s.b[2874])) && s.b[2875]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1962)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2873])) && (!s.b[2874])) && (!s.b[2875])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1962)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2873])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1962)), (-1.0)));}
        s.b[2876] = (s.v[1962] > s.v[1933]);s.store_scalar(2876, if s.b[2876] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2873])) && s.b[2876]) {s.store_neg(1996, 1996);}
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {s.store_add_scaled_product_right_sub(1944, 1996, (-1.0), 1937, 1890, 1962, -1.0);s.store_add_scaled_product_mixed_iai(1963, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.4, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2877] = (((s.v[1963]) as f64).abs() <= s.v[1933]);s.store_scalar(2877, if s.b[2877] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2877]) {s.store_mul_ad_affine_product_rhs(1996, 1963, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1963), 1.0, A::scale(s.ad_value(1963), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2878] = ((((-s.v[1963])) as f64).abs() < 230.25850929940458);s.store_scalar(2878, if s.b[2878] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2877])) && s.b[2878]) {s.store_exp_neg_input(2027, 1963);}
        s.b[2879] = ((-s.v[1963]) < 0.0);s.store_scalar(2879, if s.b[2879] { 1.0 } else { 0.0 });
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2877])) && (!s.b[2878])) && s.b[2879]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1963)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2877])) && (!s.b[2878])) && (!s.b[2879])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1963)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2877])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1963)), (-1.0)));}
        s.b[2880] = (s.v[1963] > s.v[1933]);s.store_scalar(2880, if s.b[2880] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2877])) && s.b[2880]) {s.store_neg(1996, 1996);}
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {s.store_add_scaled_product_right_sub(1945, 1996, (-1.0), 1937, 1890, 1963, -1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_147(
        s: &mut Scratch,
    ) {
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {s.store_add_scaled_product_mixed_iai(1964, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.5, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2881] = (((s.v[1964]) as f64).abs() <= s.v[1933]);s.store_scalar(2881, if s.b[2881] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2881]) {s.store_mul_ad_affine_product_rhs(1996, 1964, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1964), 1.0, A::scale(s.ad_value(1964), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2882] = ((((-s.v[1964])) as f64).abs() < 230.25850929940458);s.store_scalar(2882, if s.b[2882] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2881])) && s.b[2882]) {s.store_exp_neg_input(2027, 1964);}
        s.b[2883] = ((-s.v[1964]) < 0.0);s.store_scalar(2883, if s.b[2883] { 1.0 } else { 0.0 });
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2881])) && (!s.b[2882])) && s.b[2883]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1964)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2881])) && (!s.b[2882])) && (!s.b[2883])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1964)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2881])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1964)), (-1.0)));}
        s.b[2884] = (s.v[1964] > s.v[1933]);s.store_scalar(2884, if s.b[2884] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2881])) && s.b[2884]) {s.store_neg(1996, 1996);}
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {s.store_add_scaled_product_right_sub(1946, 1996, (-1.0), 1937, 1890, 1964, -1.0);s.store_add_scaled_product_mixed_iai(1965, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.6, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2885] = (((s.v[1965]) as f64).abs() <= s.v[1933]);s.store_scalar(2885, if s.b[2885] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2885]) {s.store_mul_ad_affine_product_rhs(1996, 1965, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1965), 1.0, A::scale(s.ad_value(1965), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2886] = ((((-s.v[1965])) as f64).abs() < 230.25850929940458);s.store_scalar(2886, if s.b[2886] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2885])) && s.b[2886]) {s.store_exp_neg_input(2027, 1965);}
        s.b[2887] = ((-s.v[1965]) < 0.0);s.store_scalar(2887, if s.b[2887] { 1.0 } else { 0.0 });
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2885])) && (!s.b[2886])) && s.b[2887]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1965)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2885])) && (!s.b[2886])) && (!s.b[2887])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1965)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2885])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1965)), (-1.0)));}
        s.b[2888] = (s.v[1965] > s.v[1933]);s.store_scalar(2888, if s.b[2888] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2885])) && s.b[2888]) {s.store_neg(1996, 1996);}
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {s.store_add_scaled_product_right_sub(1947, 1996, (-1.0), 1937, 1890, 1965, -1.0);s.store_add_scaled_product_mixed_iai(1966, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.7, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2889] = (((s.v[1966]) as f64).abs() <= s.v[1933]);s.store_scalar(2889, if s.b[2889] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2889]) {s.store_mul_ad_affine_product_rhs(1996, 1966, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1966), 1.0, A::scale(s.ad_value(1966), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2890] = ((((-s.v[1966])) as f64).abs() < 230.25850929940458);s.store_scalar(2890, if s.b[2890] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_148(
        s: &mut Scratch,
    ) {
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2889])) && s.b[2890]) {s.store_exp_neg_input(2027, 1966);}
        s.b[2891] = ((-s.v[1966]) < 0.0);s.store_scalar(2891, if s.b[2891] { 1.0 } else { 0.0 });
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2889])) && (!s.b[2890])) && s.b[2891]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1966)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2889])) && (!s.b[2890])) && (!s.b[2891])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1966)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2889])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1966)), (-1.0)));}
        s.b[2892] = (s.v[1966] > s.v[1933]);s.store_scalar(2892, if s.b[2892] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2889])) && s.b[2892]) {s.store_neg(1996, 1996);}
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {s.store_add_scaled_product_right_sub(1948, 1996, (-1.0), 1937, 1890, 1966, -1.0);s.store_add_scaled_product_mixed_iai(1967, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.8, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2893] = (((s.v[1967]) as f64).abs() <= s.v[1933]);s.store_scalar(2893, if s.b[2893] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2893]) {s.store_mul_ad_affine_product_rhs(1996, 1967, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1967), 1.0, A::scale(s.ad_value(1967), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2894] = ((((-s.v[1967])) as f64).abs() < 230.25850929940458);s.store_scalar(2894, if s.b[2894] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2893])) && s.b[2894]) {s.store_exp_neg_input(2027, 1967);}
        s.b[2895] = ((-s.v[1967]) < 0.0);s.store_scalar(2895, if s.b[2895] { 1.0 } else { 0.0 });
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2893])) && (!s.b[2894])) && s.b[2895]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1967)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2893])) && (!s.b[2894])) && (!s.b[2895])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1967)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2893])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1967)), (-1.0)));}
        s.b[2896] = (s.v[1967] > s.v[1933]);s.store_scalar(2896, if s.b[2896] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2893])) && s.b[2896]) {s.store_neg(1996, 1996);}
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {s.store_add_scaled_product_right_sub(1949, 1996, (-1.0), 1937, 1890, 1967, -1.0);s.store_add_scaled_product_mixed_iai(1968, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.9, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2897] = (((s.v[1968]) as f64).abs() <= s.v[1933]);s.store_scalar(2897, if s.b[2897] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2897]) {s.store_mul_ad_affine_product_rhs(1996, 1968, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1968), 1.0, A::scale(s.ad_value(1968), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2898] = ((((-s.v[1968])) as f64).abs() < 230.25850929940458);s.store_scalar(2898, if s.b[2898] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2897])) && s.b[2898]) {s.store_exp_neg_input(2027, 1968);}
        s.b[2899] = ((-s.v[1968]) < 0.0);s.store_scalar(2899, if s.b[2899] { 1.0 } else { 0.0 });
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2897])) && (!s.b[2898])) && s.b[2899]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1968)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2897])) && (!s.b[2898])) && (!s.b[2899])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1968)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2897])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1968)), (-1.0)));}
        s.b[2900] = (s.v[1968] > s.v[1933]);s.store_scalar(2900, if s.b[2900] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2897])) && s.b[2900]) {s.store_neg(1996, 1996);}
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {s.store_add_scaled_product_right_sub(1950, 1996, (-1.0), 1937, 1890, 1968, -1.0);}
        s.b[2901] = (s.v[831] < 0.0);s.store_scalar(2901, if s.b[2901] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_149(
        s: &mut Scratch,
    ) {
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2901]) {s.copy_ad(2027, 1942);s.copy_ad(1942, 1950);s.copy_ad(1950, 2027);s.copy_ad(2027, 1943);s.copy_ad(1943, 1949);s.copy_ad(1949, 2027);s.copy_ad(2027, 1944);s.copy_ad(1944, 1948);s.copy_ad(1948, 2027);s.copy_ad(2027, 1945);s.copy_ad(1945, 1947);s.copy_ad(1947, 2027);}
        s.store_scalar(1983, 0.0);s.store_scalar(1984, 0.0);s.store_scalar(1978, 0.0);s.store_scalar(1979, 0.0);s.b[2902] = (s.v[1] != 0.0);s.store_scalar(2902, if s.b[2902] { 1.0 } else { 0.0 });
        if s.b[2902] {s.store_sub_mixed_ia(1983, 1934, A::mul3_scaled_output(s.ad_value(831), s.ad_value(1893), s.ad_value(1932), 0.5));s.store_add_product3_rhs_indices(1984, 1934, 831, 1893, 1932, 0.5);s.store_scalar(1978, 0.0);s.store_scalar(1979, 0.0);}
        s.b[2903] = (s.v[1983] > 0.0);s.store_scalar(2903, if s.b[2903] { 1.0 } else { 0.0 });s.b[2904] = (((s.v[1983]) as f64).abs() <= s.v[1933]);s.store_scalar(2904, if s.b[2904] { 1.0 } else { 0.0 });
        if ((s.b[2902] && s.b[2903]) && s.b[2904]) {s.store_mul_ad_affine_product_rhs(1997, 1983, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1983), 1.0, A::scale(s.ad_value(1983), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2905] = ((((-s.v[1983])) as f64).abs() < 230.25850929940458);s.store_scalar(2905, if s.b[2905] { 1.0 } else { 0.0 });
        if (((s.b[2902] && s.b[2903]) && (!s.b[2904])) && s.b[2905]) {s.store_exp_neg_input(2027, 1983);}
        s.b[2906] = ((-s.v[1983]) < 0.0);s.store_scalar(2906, if s.b[2906] { 1.0 } else { 0.0 });
        if ((((s.b[2902] && s.b[2903]) && (!s.b[2904])) && (!s.b[2905])) && s.b[2906]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1983)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2902] && s.b[2903]) && (!s.b[2904])) && (!s.b[2905])) && (!s.b[2906])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1983)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2902] && s.b[2903]) && (!s.b[2904])) {s.store_mul_sqrt_mixed_ia(1997, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1983)), (-1.0)));}
        s.b[2907] = (s.v[1983] > s.v[1933]);s.store_scalar(2907, if s.b[2907] { 1.0 } else { 0.0 });
        if (((s.b[2902] && s.b[2903]) && (!s.b[2904])) && s.b[2907]) {s.store_neg(1997, 1997);}
        if (s.b[2902] && s.b[2903]) {s.store_add_scaled_product_right_sub(1978, 1997, (-1.0), 1937, 1890, 1983, -1.0);}
        s.b[2908] = (s.v[1984] > 0.0);s.store_scalar(2908, if s.b[2908] { 1.0 } else { 0.0 });s.b[2909] = (((s.v[1984]) as f64).abs() <= s.v[1933]);s.store_scalar(2909, if s.b[2909] { 1.0 } else { 0.0 });
        if ((s.b[2902] && s.b[2908]) && s.b[2909]) {s.store_mul_ad_affine_product_rhs(1997, 1984, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1984), 1.0, A::scale(s.ad_value(1984), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2910] = ((((-s.v[1984])) as f64).abs() < 230.25850929940458);s.store_scalar(2910, if s.b[2910] { 1.0 } else { 0.0 });
        if (((s.b[2902] && s.b[2908]) && (!s.b[2909])) && s.b[2910]) {s.store_exp_neg_input(2027, 1984);}
        s.b[2911] = ((-s.v[1984]) < 0.0);s.store_scalar(2911, if s.b[2911] { 1.0 } else { 0.0 });
        if ((((s.b[2902] && s.b[2908]) && (!s.b[2909])) && (!s.b[2910])) && s.b[2911]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1984)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2902] && s.b[2908]) && (!s.b[2909])) && (!s.b[2910])) && (!s.b[2911])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1984)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2902] && s.b[2908]) && (!s.b[2909])) {s.store_mul_sqrt_mixed_ia(1997, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1984)), (-1.0)));}
        s.b[2912] = (s.v[1984] > s.v[1933]);s.store_scalar(2912, if s.b[2912] { 1.0 } else { 0.0 });
        if (((s.b[2902] && s.b[2908]) && (!s.b[2909])) && s.b[2912]) {s.store_neg(1997, 1997);}
        if (s.b[2902] && s.b[2908]) {s.store_add_scaled_product_right_sub(1979, 1997, (-1.0), 1937, 1890, 1984, -1.0);}
        s.b[2913] = (s.v[831] > 0.0);s.store_scalar(2913, if s.b[2913] { 1.0 } else { 0.0 });s.b[2914] = (s.v[300] > 0.0);s.store_scalar(2914, if s.b[2914] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_150(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[2915] = (s.v[301] > 0.0);s.store_scalar(2915, if s.b[2915] { 1.0 } else { 0.0 });s.b[2916] = (s.v[302] > 0.0);s.store_scalar(2916, if s.b[2916] { 1.0 } else { 0.0 });s.b[2917] = (s.v[303] > 0.0);s.store_scalar(2917, if s.b[2917] { 1.0 } else { 0.0 });s.b[2918] = (s.v[304] > 0.0);s.store_scalar(2918, if s.b[2918] { 1.0 } else { 0.0 });s.b[2919] = (s.v[305] > 0.0);s.store_scalar(2919, if s.b[2919] { 1.0 } else { 0.0 });s.b[2920] = (s.v[306] > 0.0);s.store_scalar(2920, if s.b[2920] { 1.0 } else { 0.0 });s.store_scaled_voltage(1969, ctx, nodes, Some(12), None, s.v[3]);s.store_scaled_voltage(1970, ctx, nodes, Some(13), None, s.v[3]);s.store_scaled_voltage(1971, ctx, nodes, Some(14), None, s.v[3]);s.store_scaled_voltage(1972, ctx, nodes, Some(15), None, s.v[3]);s.store_scaled_voltage(1973, ctx, nodes, Some(16), None, s.v[3]);s.store_scaled_voltage(1974, ctx, nodes, Some(17), None, s.v[3]);s.store_scaled_voltage(1975, ctx, nodes, Some(18), None, s.v[3]);s.store_scaled_voltage(1976, ctx, nodes, Some(19), None, s.v[3]);s.store_scaled_voltage(1977, ctx, nodes, Some(20), None, s.v[3]);s.store_scalar(1995, 0.0);s.b[2921] = (s.v[1] != 0.0);s.store_scalar(2921, if s.b[2921] { 1.0 } else { 0.0 });
        if s.b[2921] {s.store_div_scaled_product3_by_product_indices(1995, 307, 1888, 716, 1.0, 1904, 1906, 1.0);s.store_mul_ad_product_lhs_mixed_ai(2018, A::square(s.ad_value(1907)), 1888, 1888);}
        s.b[2922] = (s.v[1] == 1.0);s.store_scalar(2922, if s.b[2922] { 1.0 } else { 0.0 });
        if (s.b[2921] && s.b[2922]) {s.store_sub(1992, 1979, 1978);s.store_add_scaled_inputs3_indices(1993, 1978, 6.0, 1979, 6.0, 1969, (-12.0));}
        s.b[2923] = (s.v[1] == 2.0);s.store_scalar(2923, if s.b[2923] { 1.0 } else { 0.0 });
        if ((s.b[2921] && (!s.b[2922])) && s.b[2923]) {s.store_add_scaled_inputs4_indices(1992, 1978, ((-7.0) * 0.2), 1969, ((-3.0) * 0.2), 1970, (12.0 * 0.2), 1979, ((-2.0) * 0.2));s.store_add_scaled_inputs4_indices(1993, 1978, ((-4.0) * ((-18.0) / 5.0)), 1969, (9.0 * ((-18.0) / 5.0)), 1970, ((-6.0) * ((-18.0) / 5.0)), 1979, ((-18.0) / 5.0));}
        s.b[2924] = (s.v[1] == 3.0);s.store_scalar(2924, if s.b[2924] { 1.0 } else { 0.0 });
        if (((s.b[2921] && (!s.b[2922])) && (!s.b[2923])) && s.b[2924]) {s.store_scaled_add_mixed_ai(1992, A::add_scaled_inputs4(s.ad_value(1978), (-13.0), s.ad_value(1969), (-6.0), s.ad_value(1970), 24.0, s.ad_value(1971), (-6.0)), 1979, 0.14285714285714285);s.store_add_scaled_inputs_mixed_ai(1993, A::add_scaled_inputs4(s.ad_value(1978), 180.0, s.ad_value(1969), (-408.0), s.ad_value(1970), 288.0, s.ad_value(1971), (-72.0)), 0.14285714285714285, 1979, (12.0 * 0.14285714285714285));}
        s.b[2925] = (s.v[1] == 5.0);s.store_scalar(2925, if s.b[2925] { 1.0 } else { 0.0 });
        if ((((s.b[2921] && (!s.b[2922])) && (!s.b[2923])) && (!s.b[2924])) && s.b[2925]) {s.store_add_scaled_inputs_mixed_ai(1992, A::add(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1978), (-181.0), s.ad_value(1969), (-84.0), s.ad_value(1972), 24.0, s.ad_value(1973), (-6.0)), 1.0, s.ad_value(1971), 90.0), s.ad_value(1979)), 0.015384615384615385, 1970, (336.0 * 0.015384615384615385));s.store_add_scaled_inputs_mixed_ai(1993, A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1972), 432.0, s.ad_value(1973), (-108.0), s.ad_value(1971), (-1620.0), s.ad_value(1979), 18.0), 1.0, s.ad_value(1978), 3762.0), 1.0, s.ad_value(1969), 8532.0), 0.015384615384615385, 1970, (6048.0 * 0.015384615384615385));}
        s.b[2926] = (s.v[1] == 9.0);s.store_scalar(2926, if s.b[2926] { 1.0 } else { 0.0 });
        if (((((s.b[2921] && (!s.b[2922])) && (!s.b[2923])) && (!s.b[2924])) && (!s.b[2925])) && s.b[2926]) {s.store_sub_scaled_inputs_mixed_ai(1992, A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1974), 1680.0, s.ad_value(1972), 23400.0, s.ad_value(1979), 5.0, s.ad_value(1971), (-87330.0)), 1.0, s.ad_value(1976), 120.0), 1.0, s.ad_value(1975), 450.0), 1.0, s.ad_value(1969), 81480.0), 1.0, s.ad_value(1970), 325920.0), 1.0, s.ad_value(1978), 175565.0), 2.6434745829918846e-5, s.ad_value(1977), (30.0 * 2.6434745829918846e-5)), 1.0, 1973, (30.0 / 181.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_151(
        s: &mut Scratch,
    ) {
        if (((((s.b[2921] && (!s.b[2922])) && (!s.b[2923])) && (!s.b[2924])) && (!s.b[2925])) && s.b[2926]) {s.store_sub_scaled_inputs_mixed_ai(1993, A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1975), (-13500.0), s.ad_value(1972), 702000.0, s.ad_value(1971), (-2619900.0), s.ad_value(1969), (-13793100.0)), 1.0, s.ad_value(1970), 9777600.0), 1.0, s.ad_value(1978), 6081750.0), 1.0, s.ad_value(1979), 150.0), 1.0, s.ad_value(1976), 3600.0), 1.0, s.ad_value(1977), 900.0), 2.6434745829918846e-5, s.ad_value(1974), (50400.0 * 2.6434745829918846e-5)), 1.0, 1973, (900.0 / 181.0));}
        if (((((s.b[2921] && (!s.b[2922])) && (!s.b[2923])) && (!s.b[2924])) && (!s.b[2925])) && (!s.b[2926])) {s.store_scalar(1992, 0.0);s.store_scalar(1993, 0.0);}
        if s.b[2921] {s.store_add_div_lhs_indices(2027, 1969, 1937, 1890);}
        s.b[2927] = (((s.v[2027]) as f64).abs() <= s.v[1941]);s.store_scalar(2927, if s.b[2927] { 1.0 } else { 0.0 });
        if (s.b[2921] && s.b[2927]) {s.store_div(2016, 2027, 1940);}
        s.b[2928] = (s.v[2027] < (-s.v[1941]));s.store_scalar(2928, if s.b[2928] { 1.0 } else { 0.0 });
        if ((s.b[2921] && (!s.b[2927])) && s.b[2928]) {s.store_neg(1999, 2027);s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);s.store_add(824, 2002, 2003);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);s.store_add_mixed_ia(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));}
        s.b[2929] = (((s.v[2015]) as f64).abs() < 230.25850929940458);s.store_scalar(2929, if s.b[2929] { 1.0 } else { 0.0 });
        if (((s.b[2921] && (!s.b[2927])) && s.b[2928]) && s.b[2929]) {s.store_exp(2005, 2015);}
        s.b[2930] = (s.v[2015] < 0.0);s.store_scalar(2930, if s.b[2930] { 1.0 } else { 0.0 });
        if ((((s.b[2921] && (!s.b[2927])) && s.b[2928]) && (!s.b[2929])) && s.b[2930]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2921] && (!s.b[2927])) && s.b[2928]) && (!s.b[2929])) && (!s.b[2930])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2921] && (!s.b[2927])) && s.b[2928]) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs_product_mixed_iiia(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_152(
        s: &mut Scratch,
    ) {
        if ((s.b[2921] && (!s.b[2927])) && s.b[2928]) {s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_neg_add(2016, 2015, 2012);}
        if ((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) {s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);s.store_mul_scale_offset_mixed_ia(2010, 1998, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), 1.0, (-1.0));s.store_mul_scale_offset(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0, 1.0);}
        s.b[2931] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);s.store_scalar(2931, if s.b[2931] { 1.0 } else { 0.0 });
        if (((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) && s.b[2931]) {s.store_exp_neg_input(2009, 2011);}
        s.b[2932] = ((-s.v[2011]) < 0.0);s.store_scalar(2932, if s.b[2932] { 1.0 } else { 0.0 });
        if ((((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) && (!s.b[2931])) && s.b[2932]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) && (!s.b[2931])) && (!s.b[2932])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) {s.store_sub_from_scalar(2012, 1.0, 2009);s.store_add_scaled_inputs_product_mixed_iiia(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));}
        s.b[2933] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);s.store_scalar(2933, if s.b[2933] { 1.0 } else { 0.0 });
        if (((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) && s.b[2933]) {s.store_exp_neg_input(2005, 2013);}
        s.b[2934] = ((-s.v[2013]) < 0.0);s.store_scalar(2934, if s.b[2934] { 1.0 } else { 0.0 });
        if ((((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) && (!s.b[2933])) && s.b[2934]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) && (!s.b[2933])) && (!s.b[2934])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_add(2016, 2013, 2014);}
        s.b[2935] = (((s.v[2016]) as f64).abs() <= s.v[1933]);s.store_scalar(2935, if s.b[2935] { 1.0 } else { 0.0 });
        if (s.b[2921] && s.b[2935]) {s.store_mul_ad_affine_product_rhs(1996, 2016, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(1991, 1889, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.25), 0.3333333333333333), 1.0, (-0.70710678));s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1990, 1889, 1.0, 2016, 0.5, (-0.235702));}
        s.b[2936] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);s.store_scalar(2936, if s.b[2936] { 1.0 } else { 0.0 });
        if ((s.b[2921] && (!s.b[2935])) && s.b[2936]) {s.store_exp_neg_input(2027, 2016);}
        s.b[2937] = ((-s.v[2016]) < 0.0);s.store_scalar(2937, if s.b[2937] { 1.0 } else { 0.0 });
        if (((s.b[2921] && (!s.b[2935])) && (!s.b[2936])) && s.b[2937]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2016)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[2921] && (!s.b[2935])) && (!s.b[2936])) && (!s.b[2937])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(2016)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_153(
        s: &mut Scratch,
    ) {
        if (s.b[2921] && (!s.b[2935])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));}
        s.b[2938] = (s.v[2016] > s.v[1933]);s.store_scalar(2938, if s.b[2938] { 1.0 } else { 0.0 });
        if ((s.b[2921] && (!s.b[2935])) && s.b[2938]) {s.store_neg(1996, 1996);}
        if (s.b[2921] && (!s.b[2935])) {s.store_div_scaled_product3_mixed_iiai(1991, 1889, 1889, A::sub_from_scalar(1.0, s.ad_value(2027)), 0.5, 1996, 1.0);s.store_add_div_lhs_mixed_ai(1990, A::add_scaled_square_product(s.ad_value(1991), 1.0, s.ad_value(1889), s.ad_value(1889), (-0.5)), 1996, 1991);}
        if s.b[2921] {s.store_sub(1988, 1937, 1991);s.store_div_from_scalar(1989, 1.0, 1988);s.store_offset_mul(1987, 1969, 1989, (-1.0));s.store_mul_scale_offset_mixed_ia(1986, 1989, A::mul(A::mul3(s.ad_value(1969), s.ad_value(1990), s.ad_value(1989)), s.ad_value(1989)), -1.0, 1.0);s.store_add_scaled_product_mixed_aii(2017, A::mul3(s.ad_value(1986), s.ad_value(1992), s.ad_value(1992)), 1.0, 1987, 1993, 1.0);s.store_mul_ad_product_lhs_mixed_ai(1994, A::square(s.ad_value(1992)), 1989, 1989);s.store_mul(1985, 2018, 1994);}
        s.b[2939] = (s.v[0] == (-1.0));s.store_scalar(2939, if s.b[2939] { 1.0 } else { 0.0 });
        if (s.b[2921] && s.b[2939]) {s.store_div_scaled_value_offset_denominator(1985, s.ad_value(1985), 1.0, A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0, 1.0);}
        if s.b[2921] {s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);s.store_add_scaled_product_indices(2028, 1993, 1.0, 1994, 1990, (-1.0));s.store_mul_sub_mixed_iia(1951, 2019, 2017, A::div_scaled_product(A::mul3(s.ad_value(1985), s.ad_value(1987), s.ad_value(2028)), s.ad_value(2019), 1.0, s.ad_value(2027), 1.0));}
        if (!s.b[2921]) {s.store_scalar(2018, 0.0);}
        s.b[2940] = (s.v[1] >= 2.0);s.store_scalar(2940, if s.b[2940] { 1.0 } else { 0.0 });s.b[2941] = (s.v[1] == 2.0);s.store_scalar(2941, if s.b[2941] { 1.0 } else { 0.0 });
        if (s.b[2940] && s.b[2941]) {s.store_add_scaled_inputs4_indices(1992, 1978, (2.0 * 0.2), 1969, ((-12.0) * 0.2), 1970, (3.0 * 0.2), 1979, (7.0 * 0.2));s.store_add_scaled_inputs4_indices(1993, 1979, ((-4.0) * ((-18.0) / 5.0)), 1970, (9.0 * ((-18.0) / 5.0)), 1969, ((-6.0) * ((-18.0) / 5.0)), 1978, ((-18.0) / 5.0));}
        s.b[2942] = (s.v[1] == 3.0);s.store_scalar(2942, if s.b[2942] { 1.0 } else { 0.0 });
        if ((s.b[2940] && (!s.b[2941])) && s.b[2942]) {s.store_add_scaled_inputs4_indices(1992, 1978, 0.5, 1969, (-3.0), 1971, 3.0, 1979, (-0.5));s.store_sub_scaled_inputs_mixed_ai(1993, A::add_scaled_inputs4(s.ad_value(1978), (-48.0), s.ad_value(1969), 288.0, s.ad_value(1970), (-480.0), s.ad_value(1971), 288.0), 0.14285714285714285, 1979, (48.0 * 0.14285714285714285));}
        s.b[2943] = (s.v[1] == 5.0);s.store_scalar(2943, if s.b[2943] { 1.0 } else { 0.0 });
        if (((s.b[2940] && (!s.b[2941])) && (!s.b[2942])) && s.b[2943]) {s.store_add_ad(1992, A::add_scaled_inputs4(s.ad_value(1969), ((-291.0) * 0.015384615384615385), s.ad_value(1970), ((-6.0) * 0.015384615384615385), s.ad_value(1972), ((-84.0) * 0.015384615384615385), s.ad_value(1973), (21.0 * 0.015384615384615385)), A::add_scaled_inputs3(s.ad_value(1971), (630.0 * 0.007692307692307693), s.ad_value(1979), ((-7.0) * 0.007692307692307693), s.ad_value(1978), (97.0 * 0.007692307692307693)));s.store_sub_scaled_inputs_mixed_ai(1993, A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1972), (-1728.0), s.ad_value(1973), 432.0, s.ad_value(1971), 6480.0, s.ad_value(1979), (-72.0)), 1.0, s.ad_value(1978), 1008.0), 1.0, s.ad_value(1969), 6048.0), 0.015384615384615385, 1970, (10152.0 * 0.015384615384615385));}
        s.b[2944] = (s.v[1] == 9.0);s.store_scalar(2944, if s.b[2944] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_154(
        s: &mut Scratch,
    ) {
        if ((((s.b[2940] && (!s.b[2941])) && (!s.b[2942])) && (!s.b[2943])) && s.b[2944]) {s.store_add_ad(1992, A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1974), (-5880.0), s.ad_value(1972), (-81900.0), s.ad_value(1971), 305655.0, s.ad_value(1976), (-420.0)), 1.0, s.ad_value(1977), 105.0), 1.0, s.ad_value(1969), 282255.0), 1.0, s.ad_value(1975), 1575.0), 2.6434745829918846e-5, s.ad_value(1970), (5850.0 * 2.6434745829918846e-5)), 1.0, s.ad_value(1973), (105.0 / 181.0)), A::sub_scaled_inputs(s.ad_value(1978), (94085.0 * 1.3217372914959423e-5), s.ad_value(1979), (35.0 * 1.3217372914959423e-5)));s.store_add_scaled_inputs_mixed_ai(1993, A::sub_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1969), 9777600.0, s.ad_value(1975), 54000.0, s.ad_value(1972), (-2808000.0), s.ad_value(1971), 10479600.0), 1.0, s.ad_value(1970), 16413000.0), 1.0, s.ad_value(1978), 1629600.0), 1.0, s.ad_value(1979), 600.0), 1.0, s.ad_value(1976), 14400.0), 1.0, s.ad_value(1977), 3600.0), 2.6434745829918846e-5, s.ad_value(1974), (201600.0 * 2.6434745829918846e-5)), 1.0, 1973, (3600.0 * 0.0055248618784530384));}
        if ((((s.b[2940] && (!s.b[2941])) && (!s.b[2942])) && (!s.b[2943])) && (!s.b[2944])) {s.store_scalar(1992, 0.0);s.store_scalar(1993, 0.0);}
        if s.b[2940] {s.store_add_div_lhs_indices(2027, 1970, 1937, 1890);}
        s.b[2945] = (((s.v[2027]) as f64).abs() <= s.v[1941]);s.store_scalar(2945, if s.b[2945] { 1.0 } else { 0.0 });
        if (s.b[2940] && s.b[2945]) {s.store_div(2016, 2027, 1940);}
        s.b[2946] = (s.v[2027] < (-s.v[1941]));s.store_scalar(2946, if s.b[2946] { 1.0 } else { 0.0 });
        if ((s.b[2940] && (!s.b[2945])) && s.b[2946]) {s.store_neg(1999, 2027);s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);s.store_add(824, 2002, 2003);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);s.store_add_mixed_ia(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));}
        s.b[2947] = (((s.v[2015]) as f64).abs() < 230.25850929940458);s.store_scalar(2947, if s.b[2947] { 1.0 } else { 0.0 });
        if (((s.b[2940] && (!s.b[2945])) && s.b[2946]) && s.b[2947]) {s.store_exp(2005, 2015);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_155(
        s: &mut Scratch,
    ) {
        s.b[2948] = (s.v[2015] < 0.0);s.store_scalar(2948, if s.b[2948] { 1.0 } else { 0.0 });
        if ((((s.b[2940] && (!s.b[2945])) && s.b[2946]) && (!s.b[2947])) && s.b[2948]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2940] && (!s.b[2945])) && s.b[2946]) && (!s.b[2947])) && (!s.b[2948])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2940] && (!s.b[2945])) && s.b[2946]) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs_product_mixed_iiia(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_neg_add(2016, 2015, 2012);}
        if ((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) {s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);s.store_mul_scale_offset_mixed_ia(2010, 1998, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), 1.0, (-1.0));s.store_mul_scale_offset(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0, 1.0);}
        s.b[2949] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);s.store_scalar(2949, if s.b[2949] { 1.0 } else { 0.0 });
        if (((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && s.b[2949]) {s.store_exp_neg_input(2009, 2011);}
        s.b[2950] = ((-s.v[2011]) < 0.0);s.store_scalar(2950, if s.b[2950] { 1.0 } else { 0.0 });
        if ((((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && (!s.b[2949])) && s.b[2950]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && (!s.b[2949])) && (!s.b[2950])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) {s.store_sub_from_scalar(2012, 1.0, 2009);s.store_add_scaled_inputs_product_mixed_iiia(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));}
        s.b[2951] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);s.store_scalar(2951, if s.b[2951] { 1.0 } else { 0.0 });
        if (((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && s.b[2951]) {s.store_exp_neg_input(2005, 2013);}
        s.b[2952] = ((-s.v[2013]) < 0.0);s.store_scalar(2952, if s.b[2952] { 1.0 } else { 0.0 });
        if ((((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && (!s.b[2951])) && s.b[2952]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && (!s.b[2951])) && (!s.b[2952])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_add(2016, 2013, 2014);}
        s.b[2953] = (((s.v[2016]) as f64).abs() <= s.v[1933]);s.store_scalar(2953, if s.b[2953] { 1.0 } else { 0.0 });
        if (s.b[2940] && s.b[2953]) {s.store_mul_ad_affine_product_rhs(1996, 2016, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_156(
        s: &mut Scratch,
    ) {
        if (s.b[2940] && s.b[2953]) {s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(1991, 1889, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.25), 0.3333333333333333), 1.0, (-0.70710678));s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1990, 1889, 1.0, 2016, 0.5, (-0.235702));}
        s.b[2954] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);s.store_scalar(2954, if s.b[2954] { 1.0 } else { 0.0 });
        if ((s.b[2940] && (!s.b[2953])) && s.b[2954]) {s.store_exp_neg_input(2027, 2016);}
        s.b[2955] = ((-s.v[2016]) < 0.0);s.store_scalar(2955, if s.b[2955] { 1.0 } else { 0.0 });
        if (((s.b[2940] && (!s.b[2953])) && (!s.b[2954])) && s.b[2955]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2016)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[2940] && (!s.b[2953])) && (!s.b[2954])) && (!s.b[2955])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(2016)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (s.b[2940] && (!s.b[2953])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));}
        s.b[2956] = (s.v[2016] > s.v[1933]);s.store_scalar(2956, if s.b[2956] { 1.0 } else { 0.0 });
        if ((s.b[2940] && (!s.b[2953])) && s.b[2956]) {s.store_neg(1996, 1996);}
        if (s.b[2940] && (!s.b[2953])) {s.store_div_scaled_product3_mixed_iiai(1991, 1889, 1889, A::sub_from_scalar(1.0, s.ad_value(2027)), 0.5, 1996, 1.0);s.store_add_div_lhs_mixed_ai(1990, A::add_scaled_square_product(s.ad_value(1991), 1.0, s.ad_value(1889), s.ad_value(1889), (-0.5)), 1996, 1991);}
        if s.b[2940] {s.store_sub(1988, 1937, 1991);s.store_div_from_scalar(1989, 1.0, 1988);s.store_offset_mul(1987, 1970, 1989, (-1.0));s.store_mul_scale_offset_mixed_ia(1986, 1989, A::mul(A::mul3(s.ad_value(1970), s.ad_value(1990), s.ad_value(1989)), s.ad_value(1989)), -1.0, 1.0);s.store_add_scaled_product_mixed_aii(2017, A::mul3(s.ad_value(1986), s.ad_value(1992), s.ad_value(1992)), 1.0, 1987, 1993, 1.0);s.store_mul_ad_product_lhs_mixed_ai(1994, A::square(s.ad_value(1992)), 1989, 1989);s.store_mul(1985, 2018, 1994);}
        s.b[2957] = (s.v[0] == (-1.0));s.store_scalar(2957, if s.b[2957] { 1.0 } else { 0.0 });
        if (s.b[2940] && s.b[2957]) {s.store_div_scaled_value_offset_denominator(1985, s.ad_value(1985), 1.0, A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0, 1.0);}
        if s.b[2940] {s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);s.store_add_scaled_product_indices(2028, 1993, 1.0, 1994, 1990, (-1.0));s.store_mul_sub_mixed_iia(1952, 2019, 2017, A::div_scaled_product(A::mul3(s.ad_value(1985), s.ad_value(1987), s.ad_value(2028)), s.ad_value(2019), 1.0, s.ad_value(2027), 1.0));}
        s.b[2958] = (s.v[1] >= 3.0);s.store_scalar(2958, if s.b[2958] { 1.0 } else { 0.0 });s.b[2959] = (s.v[1] == 3.0);s.store_scalar(2959, if s.b[2959] { 1.0 } else { 0.0 });
        if (s.b[2958] && s.b[2959]) {s.store_scaled_sub_mixed_ai(1992, A::add_scaled_inputs4(s.ad_value(1979), 13.0, s.ad_value(1971), 6.0, s.ad_value(1970), (-24.0), s.ad_value(1969), 6.0), 1978, 0.14285714285714285);s.store_add_scaled_inputs_mixed_ai(1993, A::add_scaled_inputs4(s.ad_value(1979), 180.0, s.ad_value(1971), (-408.0), s.ad_value(1970), 288.0, s.ad_value(1969), (-72.0)), 0.14285714285714285, 1978, (12.0 * 0.14285714285714285));}
        s.b[2960] = (s.v[1] == 5.0);s.store_scalar(2960, if s.b[2960] { 1.0 } else { 0.0 });
        if ((s.b[2958] && (!s.b[2959])) && s.b[2960]) {s.store_scaled_sub_mixed_ai(1992, A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1979), 1.0, s.ad_value(1973), (-6.0), s.ad_value(1972), 24.0, s.ad_value(1970), (-24.0)), 1.0, s.ad_value(1969), 6.0), 1978, 0.2);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_157(
        s: &mut Scratch,
    ) {
        if ((s.b[2958] && (!s.b[2959])) && s.b[2960]) {s.store_scaled_add_ad(1993, A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1972), 1296.0, s.ad_value(1970), 1296.0, s.ad_value(1973), (-324.0), s.ad_value(1969), (-324.0)), 1.0, s.ad_value(1971), 2052.0), A::add_scaled_inputs(s.ad_value(1979), 54.0, s.ad_value(1978), 54.0), 0.07692307692307693);}
        s.b[2961] = (s.v[1] == 9.0);s.store_scalar(2961, if s.b[2961] { 1.0 } else { 0.0 });
        if (((s.b[2958] && (!s.b[2959])) && (!s.b[2960])) && s.b[2961]) {s.store_sub_scaled_inputs_mixed_ai(1992, A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1974), 21840.0, s.ad_value(1972), 304200.0, s.ad_value(1979), 65.0, s.ad_value(1971), (-420.0)), 1.0, s.ad_value(1976), 1560.0), 1.0, s.ad_value(1978), 12605.0), 1.0, s.ad_value(1977), 390.0), 1.0, s.ad_value(1969), 75630.0), 1.0, s.ad_value(1975), 5850.0), 2.6434745829918846e-5, s.ad_value(1970), (302520.0 * 2.6434745829918846e-5)), 1.0, 1973, (390.0 / 181.0));s.store_sub_scaled_inputs_mixed_ai(1993, A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1969), (-2619900.0), s.ad_value(1975), (-202500.0), s.ad_value(1972), 10530000.0, s.ad_value(1971), (-16601100.0)), 1.0, s.ad_value(1970), 10479600.0), 1.0, s.ad_value(1978), 436650.0), 1.0, s.ad_value(1979), 2250.0), 1.0, s.ad_value(1976), 54000.0), 1.0, s.ad_value(1977), 13500.0), 2.6434745829918846e-5, s.ad_value(1974), (756000.0 * 2.6434745829918846e-5)), 1.0, 1973, (13500.0 * 0.0055248618784530384));}
        if (((s.b[2958] && (!s.b[2959])) && (!s.b[2960])) && (!s.b[2961])) {s.store_scalar(1992, 0.0);s.store_scalar(1993, 0.0);}
        if s.b[2958] {s.store_add_div_lhs_indices(2027, 1971, 1937, 1890);}
        s.b[2962] = (((s.v[2027]) as f64).abs() <= s.v[1941]);s.store_scalar(2962, if s.b[2962] { 1.0 } else { 0.0 });
        if (s.b[2958] && s.b[2962]) {s.store_div(2016, 2027, 1940);}
        s.b[2963] = (s.v[2027] < (-s.v[1941]));s.store_scalar(2963, if s.b[2963] { 1.0 } else { 0.0 });
        if ((s.b[2958] && (!s.b[2962])) && s.b[2963]) {s.store_neg(1999, 2027);s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);s.store_add(824, 2002, 2003);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_158(
        s: &mut Scratch,
    ) {
        if ((s.b[2958] && (!s.b[2962])) && s.b[2963]) {s.store_add_mixed_ia(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));}
        s.b[2964] = (((s.v[2015]) as f64).abs() < 230.25850929940458);s.store_scalar(2964, if s.b[2964] { 1.0 } else { 0.0 });
        if (((s.b[2958] && (!s.b[2962])) && s.b[2963]) && s.b[2964]) {s.store_exp(2005, 2015);}
        s.b[2965] = (s.v[2015] < 0.0);s.store_scalar(2965, if s.b[2965] { 1.0 } else { 0.0 });
        if ((((s.b[2958] && (!s.b[2962])) && s.b[2963]) && (!s.b[2964])) && s.b[2965]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2958] && (!s.b[2962])) && s.b[2963]) && (!s.b[2964])) && (!s.b[2965])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2958] && (!s.b[2962])) && s.b[2963]) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs_product_mixed_iiia(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_neg_add(2016, 2015, 2012);}
        if ((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) {s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);s.store_mul_scale_offset_mixed_ia(2010, 1998, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), 1.0, (-1.0));s.store_mul_scale_offset(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0, 1.0);}
        s.b[2966] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);s.store_scalar(2966, if s.b[2966] { 1.0 } else { 0.0 });
        if (((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) && s.b[2966]) {s.store_exp_neg_input(2009, 2011);}
        s.b[2967] = ((-s.v[2011]) < 0.0);s.store_scalar(2967, if s.b[2967] { 1.0 } else { 0.0 });
        if ((((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) && (!s.b[2966])) && s.b[2967]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) && (!s.b[2966])) && (!s.b[2967])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) {s.store_sub_from_scalar(2012, 1.0, 2009);s.store_add_scaled_inputs_product_mixed_iiia(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));}
        s.b[2968] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);s.store_scalar(2968, if s.b[2968] { 1.0 } else { 0.0 });
        if (((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) && s.b[2968]) {s.store_exp_neg_input(2005, 2013);}
        s.b[2969] = ((-s.v[2013]) < 0.0);s.store_scalar(2969, if s.b[2969] { 1.0 } else { 0.0 });
        if ((((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) && (!s.b[2968])) && s.b[2969]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) && (!s.b[2968])) && (!s.b[2969])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_159(
        s: &mut Scratch,
    ) {
        if ((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) {s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_add(2016, 2013, 2014);}
        s.b[2970] = (((s.v[2016]) as f64).abs() <= s.v[1933]);s.store_scalar(2970, if s.b[2970] { 1.0 } else { 0.0 });
        if (s.b[2958] && s.b[2970]) {s.store_mul_ad_affine_product_rhs(1996, 2016, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(1991, 1889, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.25), 0.3333333333333333), 1.0, (-0.70710678));s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1990, 1889, 1.0, 2016, 0.5, (-0.235702));}
        s.b[2971] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);s.store_scalar(2971, if s.b[2971] { 1.0 } else { 0.0 });
        if ((s.b[2958] && (!s.b[2970])) && s.b[2971]) {s.store_exp_neg_input(2027, 2016);}
        s.b[2972] = ((-s.v[2016]) < 0.0);s.store_scalar(2972, if s.b[2972] { 1.0 } else { 0.0 });
        if (((s.b[2958] && (!s.b[2970])) && (!s.b[2971])) && s.b[2972]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2016)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[2958] && (!s.b[2970])) && (!s.b[2971])) && (!s.b[2972])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(2016)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (s.b[2958] && (!s.b[2970])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));}
        s.b[2973] = (s.v[2016] > s.v[1933]);s.store_scalar(2973, if s.b[2973] { 1.0 } else { 0.0 });
        if ((s.b[2958] && (!s.b[2970])) && s.b[2973]) {s.store_neg(1996, 1996);}
        if (s.b[2958] && (!s.b[2970])) {s.store_div_scaled_product3_mixed_iiai(1991, 1889, 1889, A::sub_from_scalar(1.0, s.ad_value(2027)), 0.5, 1996, 1.0);s.store_add_div_lhs_mixed_ai(1990, A::add_scaled_square_product(s.ad_value(1991), 1.0, s.ad_value(1889), s.ad_value(1889), (-0.5)), 1996, 1991);}
        if s.b[2958] {s.store_sub(1988, 1937, 1991);s.store_div_from_scalar(1989, 1.0, 1988);s.store_offset_mul(1987, 1971, 1989, (-1.0));s.store_mul_scale_offset_mixed_ia(1986, 1989, A::mul(A::mul3(s.ad_value(1971), s.ad_value(1990), s.ad_value(1989)), s.ad_value(1989)), -1.0, 1.0);s.store_add_scaled_product_mixed_aii(2017, A::mul3(s.ad_value(1986), s.ad_value(1992), s.ad_value(1992)), 1.0, 1987, 1993, 1.0);s.store_mul_ad_product_lhs_mixed_ai(1994, A::square(s.ad_value(1992)), 1989, 1989);s.store_mul(1985, 2018, 1994);}
        s.b[2974] = (s.v[0] == (-1.0));s.store_scalar(2974, if s.b[2974] { 1.0 } else { 0.0 });
        if (s.b[2958] && s.b[2974]) {s.store_div_scaled_value_offset_denominator(1985, s.ad_value(1985), 1.0, A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0, 1.0);}
        if s.b[2958] {s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);s.store_add_scaled_product_indices(2028, 1993, 1.0, 1994, 1990, (-1.0));s.store_mul_sub_mixed_iia(1953, 2019, 2017, A::div_scaled_product(A::mul3(s.ad_value(1985), s.ad_value(1987), s.ad_value(2028)), s.ad_value(2019), 1.0, s.ad_value(2027), 1.0));}
        s.b[2975] = (s.v[1] >= 4.0);s.store_scalar(2975, if s.b[2975] { 1.0 } else { 0.0 });
    }
}

#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_85(
        s: &mut ReactiveScratch,
    ) {
        s.b[2940] = (s.v[1] >= 2.0);s.store_scalar(2940, if s.b[2940] { 1.0 } else { 0.0 });s.b[2941] = (s.v[1] == 2.0);s.store_scalar(2941, if s.b[2941] { 1.0 } else { 0.0 });
        if (s.b[2940] && s.b[2941]) {s.store_add_scaled_inputs4_indices(1992, 1978, (2.0 * 0.2), 1969, ((-12.0) * 0.2), 1970, (3.0 * 0.2), 1979, (7.0 * 0.2));s.store_add_scaled_inputs4_indices(1993, 1979, ((-4.0) * ((-18.0) / 5.0)), 1970, (9.0 * ((-18.0) / 5.0)), 1969, ((-6.0) * ((-18.0) / 5.0)), 1978, ((-18.0) / 5.0));}
        s.b[2942] = (s.v[1] == 3.0);s.store_scalar(2942, if s.b[2942] { 1.0 } else { 0.0 });
        if ((s.b[2940] && (!s.b[2941])) && s.b[2942]) {s.store_add_scaled_inputs4_indices(1992, 1978, 0.5, 1969, (-3.0), 1971, 3.0, 1979, (-0.5));s.store_sub_scaled_inputs_mixed_ai(1993, A::add_scaled_inputs4(s.ad_value(1978), (-48.0), s.ad_value(1969), 288.0, s.ad_value(1970), (-480.0), s.ad_value(1971), 288.0), 0.14285714285714285, 1979, (48.0 * 0.14285714285714285));}
        s.b[2943] = (s.v[1] == 5.0);s.store_scalar(2943, if s.b[2943] { 1.0 } else { 0.0 });
        if (((s.b[2940] && (!s.b[2941])) && (!s.b[2942])) && s.b[2943]) {s.store_add_ad(1992, A::add_scaled_inputs4(s.ad_value(1969), ((-291.0) * 0.015384615384615385), s.ad_value(1970), ((-6.0) * 0.015384615384615385), s.ad_value(1972), ((-84.0) * 0.015384615384615385), s.ad_value(1973), (21.0 * 0.015384615384615385)), A::add_scaled_inputs3(s.ad_value(1971), (630.0 * 0.007692307692307693), s.ad_value(1979), ((-7.0) * 0.007692307692307693), s.ad_value(1978), (97.0 * 0.007692307692307693)));s.store_sub_scaled_inputs_mixed_ai(1993, A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1972), (-1728.0), s.ad_value(1973), 432.0, s.ad_value(1971), 6480.0, s.ad_value(1979), (-72.0)), 1.0, s.ad_value(1978), 1008.0), 1.0, s.ad_value(1969), 6048.0), 0.015384615384615385, 1970, (10152.0 * 0.015384615384615385));}
        s.b[2944] = (s.v[1] == 9.0);s.store_scalar(2944, if s.b[2944] { 1.0 } else { 0.0 });
        if ((((s.b[2940] && (!s.b[2941])) && (!s.b[2942])) && (!s.b[2943])) && s.b[2944]) {s.store_add_ad(1992, A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1974), (-5880.0), s.ad_value(1972), (-81900.0), s.ad_value(1971), 305655.0, s.ad_value(1976), (-420.0)), 1.0, s.ad_value(1977), 105.0), 1.0, s.ad_value(1969), 282255.0), 1.0, s.ad_value(1975), 1575.0), 2.6434745829918846e-5, s.ad_value(1970), (5850.0 * 2.6434745829918846e-5)), 1.0, s.ad_value(1973), (105.0 / 181.0)), A::sub_scaled_inputs(s.ad_value(1978), (94085.0 * 1.3217372914959423e-5), s.ad_value(1979), (35.0 * 1.3217372914959423e-5)));s.store_add_scaled_inputs_mixed_ai(1993, A::sub_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1969), 9777600.0, s.ad_value(1975), 54000.0, s.ad_value(1972), (-2808000.0), s.ad_value(1971), 10479600.0), 1.0, s.ad_value(1970), 16413000.0), 1.0, s.ad_value(1978), 1629600.0), 1.0, s.ad_value(1979), 600.0), 1.0, s.ad_value(1976), 14400.0), 1.0, s.ad_value(1977), 3600.0), 2.6434745829918846e-5, s.ad_value(1974), (201600.0 * 2.6434745829918846e-5)), 1.0, 1973, (3600.0 * 0.0055248618784530384));}
        if ((((s.b[2940] && (!s.b[2941])) && (!s.b[2942])) && (!s.b[2943])) && (!s.b[2944])) {s.store_scalar(1992, 0.0);s.store_scalar(1993, 0.0);}
        if s.b[2940] {s.store_add_div_lhs_indices(2027, 1970, 1937, 1890);}
        s.b[2945] = (((s.v[2027]) as f64).abs() <= s.v[1941]);s.store_scalar(2945, if s.b[2945] { 1.0 } else { 0.0 });
        if (s.b[2940] && s.b[2945]) {s.store_div(2016, 2027, 1940);}
        s.b[2946] = (s.v[2027] < (-s.v[1941]));s.store_scalar(2946, if s.b[2946] { 1.0 } else { 0.0 });
        if ((s.b[2940] && (!s.b[2945])) && s.b[2946]) {s.store_neg(1999, 2027);s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_86(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2940] && (!s.b[2945])) && s.b[2946]) {s.store_add_scaled_sub_square_product_mixed_ia(2002, 1999, 2001, 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);s.store_add(824, 2002, 2003);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);s.store_add_mixed_ia(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));}
        s.b[2947] = (((s.v[2015]) as f64).abs() < 230.25850929940458);s.store_scalar(2947, if s.b[2947] { 1.0 } else { 0.0 });
        if (((s.b[2940] && (!s.b[2945])) && s.b[2946]) && s.b[2947]) {s.store_exp(2005, 2015);}
        s.b[2948] = (s.v[2015] < 0.0);s.store_scalar(2948, if s.b[2948] { 1.0 } else { 0.0 });
        if ((((s.b[2940] && (!s.b[2945])) && s.b[2946]) && (!s.b[2947])) && s.b[2948]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2940] && (!s.b[2945])) && s.b[2946]) && (!s.b[2947])) && (!s.b[2948])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2940] && (!s.b[2945])) && s.b[2946]) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs_product_mixed_iiia(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);s.store_add_scaled_sub_square_product_mixed_ia(2008, 1999, 2015, 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_neg_add(2016, 2015, 2012);}
        if ((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) {s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);s.store_mul_scale_offset_mixed_ia(2010, 1998, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), 1.0, (-1.0));s.store_mul_scale_offset(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0, 1.0);}
        s.b[2949] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);s.store_scalar(2949, if s.b[2949] { 1.0 } else { 0.0 });
        if (((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && s.b[2949]) {s.store_exp_neg_input(2009, 2011);}
        s.b[2950] = ((-s.v[2011]) < 0.0);s.store_scalar(2950, if s.b[2950] { 1.0 } else { 0.0 });
        if ((((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && (!s.b[2949])) && s.b[2950]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && (!s.b[2949])) && (!s.b[2950])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) {s.store_sub_from_scalar(2012, 1.0, 2009);s.store_add_scaled_inputs_product_mixed_iiia(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));}
        s.b[2951] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);s.store_scalar(2951, if s.b[2951] { 1.0 } else { 0.0 });
        if (((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && s.b[2951]) {s.store_exp_neg_input(2005, 2013);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_87(
        s: &mut ReactiveScratch,
    ) {
        s.b[2952] = ((-s.v[2013]) < 0.0);s.store_scalar(2952, if s.b[2952] { 1.0 } else { 0.0 });
        if ((((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && (!s.b[2951])) && s.b[2952]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && (!s.b[2951])) && (!s.b[2952])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);s.store_add_scaled_sub_square_product_mixed_ia(2008, 2027, 2013, 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_add(2016, 2013, 2014);}
        s.b[2953] = (((s.v[2016]) as f64).abs() <= s.v[1933]);s.store_scalar(2953, if s.b[2953] { 1.0 } else { 0.0 });
        if (s.b[2940] && s.b[2953]) {s.store_mul_ad_affine_product_rhs(1996, 2016, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(1991, 1889, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.25), 0.3333333333333333), 1.0, (-0.70710678));s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1990, 1889, 1.0, 2016, 0.5, (-0.235702));}
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
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_88(
        s: &mut ReactiveScratch,
    ) {
        if s.b[2940] {s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);s.store_add_scaled_product_indices(2028, 1993, 1.0, 1994, 1990, (-1.0));s.store_mul_sub_mixed_iia(1952, 2019, 2017, A::div_scaled_product(A::mul3(s.ad_value(1985), s.ad_value(1987), s.ad_value(2028)), s.ad_value(2019), 1.0, s.ad_value(2027), 1.0));}
        s.b[2958] = (s.v[1] >= 3.0);s.store_scalar(2958, if s.b[2958] { 1.0 } else { 0.0 });s.b[2959] = (s.v[1] == 3.0);s.store_scalar(2959, if s.b[2959] { 1.0 } else { 0.0 });
        if (s.b[2958] && s.b[2959]) {s.store_scaled_sub_mixed_ai(1992, A::add_scaled_inputs4(s.ad_value(1979), 13.0, s.ad_value(1971), 6.0, s.ad_value(1970), (-24.0), s.ad_value(1969), 6.0), 1978, 0.14285714285714285);s.store_add_scaled_inputs_mixed_ai(1993, A::add_scaled_inputs4(s.ad_value(1979), 180.0, s.ad_value(1971), (-408.0), s.ad_value(1970), 288.0, s.ad_value(1969), (-72.0)), 0.14285714285714285, 1978, (12.0 * 0.14285714285714285));}
        s.b[2960] = (s.v[1] == 5.0);s.store_scalar(2960, if s.b[2960] { 1.0 } else { 0.0 });
        if ((s.b[2958] && (!s.b[2959])) && s.b[2960]) {s.store_scaled_sub_mixed_ai(1992, A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1979), 1.0, s.ad_value(1973), (-6.0), s.ad_value(1972), 24.0, s.ad_value(1970), (-24.0)), 1.0, s.ad_value(1969), 6.0), 1978, 0.2);s.store_scaled_add_ad(1993, A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1972), 1296.0, s.ad_value(1970), 1296.0, s.ad_value(1973), (-324.0), s.ad_value(1969), (-324.0)), 1.0, s.ad_value(1971), 2052.0), A::add_scaled_inputs(s.ad_value(1979), 54.0, s.ad_value(1978), 54.0), 0.07692307692307693);}
        s.b[2961] = (s.v[1] == 9.0);s.store_scalar(2961, if s.b[2961] { 1.0 } else { 0.0 });
        if (((s.b[2958] && (!s.b[2959])) && (!s.b[2960])) && s.b[2961]) {s.store_sub_scaled_inputs_mixed_ai(1992, A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1974), 21840.0, s.ad_value(1972), 304200.0, s.ad_value(1979), 65.0, s.ad_value(1971), (-420.0)), 1.0, s.ad_value(1976), 1560.0), 1.0, s.ad_value(1978), 12605.0), 1.0, s.ad_value(1977), 390.0), 1.0, s.ad_value(1969), 75630.0), 1.0, s.ad_value(1975), 5850.0), 2.6434745829918846e-5, s.ad_value(1970), (302520.0 * 2.6434745829918846e-5)), 1.0, 1973, (390.0 / 181.0));s.store_sub_scaled_inputs_mixed_ai(1993, A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1969), (-2619900.0), s.ad_value(1975), (-202500.0), s.ad_value(1972), 10530000.0, s.ad_value(1971), (-16601100.0)), 1.0, s.ad_value(1970), 10479600.0), 1.0, s.ad_value(1978), 436650.0), 1.0, s.ad_value(1979), 2250.0), 1.0, s.ad_value(1976), 54000.0), 1.0, s.ad_value(1977), 13500.0), 2.6434745829918846e-5, s.ad_value(1974), (756000.0 * 2.6434745829918846e-5)), 1.0, 1973, (13500.0 * 0.0055248618784530384));}
        if (((s.b[2958] && (!s.b[2959])) && (!s.b[2960])) && (!s.b[2961])) {s.store_scalar(1992, 0.0);s.store_scalar(1993, 0.0);}
        if s.b[2958] {s.store_add_div_lhs_indices(2027, 1971, 1937, 1890);}
        s.b[2962] = (((s.v[2027]) as f64).abs() <= s.v[1941]);s.store_scalar(2962, if s.b[2962] { 1.0 } else { 0.0 });
        if (s.b[2958] && s.b[2962]) {s.store_div(2016, 2027, 1940);}
        s.b[2963] = (s.v[2027] < (-s.v[1941]));s.store_scalar(2963, if s.b[2963] { 1.0 } else { 0.0 });
        if ((s.b[2958] && (!s.b[2962])) && s.b[2963]) {s.store_neg(1999, 2027);s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_89(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2958] && (!s.b[2962])) && s.b[2963]) {s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_sub_square_product_mixed_ia(2002, 1999, 2001, 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);s.store_add(824, 2002, 2003);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);s.store_add_mixed_ia(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));}
        s.b[2964] = (((s.v[2015]) as f64).abs() < 230.25850929940458);s.store_scalar(2964, if s.b[2964] { 1.0 } else { 0.0 });
        if (((s.b[2958] && (!s.b[2962])) && s.b[2963]) && s.b[2964]) {s.store_exp(2005, 2015);}
        s.b[2965] = (s.v[2015] < 0.0);s.store_scalar(2965, if s.b[2965] { 1.0 } else { 0.0 });
        if ((((s.b[2958] && (!s.b[2962])) && s.b[2963]) && (!s.b[2964])) && s.b[2965]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2958] && (!s.b[2962])) && s.b[2963]) && (!s.b[2964])) && (!s.b[2965])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2958] && (!s.b[2962])) && s.b[2963]) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs_product_mixed_iiia(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);s.store_add_scaled_sub_square_product_mixed_ia(2008, 1999, 2015, 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_neg_add(2016, 2015, 2012);}
        if ((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) {s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);s.store_mul_scale_offset_mixed_ia(2010, 1998, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), 1.0, (-1.0));s.store_mul_scale_offset(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0, 1.0);}
        s.b[2966] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);s.store_scalar(2966, if s.b[2966] { 1.0 } else { 0.0 });
        if (((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) && s.b[2966]) {s.store_exp_neg_input(2009, 2011);}
        s.b[2967] = ((-s.v[2011]) < 0.0);s.store_scalar(2967, if s.b[2967] { 1.0 } else { 0.0 });
        if ((((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) && (!s.b[2966])) && s.b[2967]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) && (!s.b[2966])) && (!s.b[2967])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) {s.store_sub_from_scalar(2012, 1.0, 2009);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_90(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) {s.store_add_scaled_inputs_product_mixed_iiia(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));}
        s.b[2968] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);s.store_scalar(2968, if s.b[2968] { 1.0 } else { 0.0 });
        if (((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) && s.b[2968]) {s.store_exp_neg_input(2005, 2013);}
        s.b[2969] = ((-s.v[2013]) < 0.0);s.store_scalar(2969, if s.b[2969] { 1.0 } else { 0.0 });
        if ((((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) && (!s.b[2968])) && s.b[2969]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) && (!s.b[2968])) && (!s.b[2969])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);s.store_add_scaled_sub_square_product_mixed_ia(2008, 2027, 2013, 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_add(2016, 2013, 2014);}
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
        if s.b[2958] {s.store_sub(1988, 1937, 1991);s.store_div_from_scalar(1989, 1.0, 1988);s.store_offset_mul(1987, 1971, 1989, (-1.0));s.store_mul_scale_offset_mixed_ia(1986, 1989, A::mul(A::mul3(s.ad_value(1971), s.ad_value(1990), s.ad_value(1989)), s.ad_value(1989)), -1.0, 1.0);s.store_add_scaled_product_mixed_aii(2017, A::mul3(s.ad_value(1986), s.ad_value(1992), s.ad_value(1992)), 1.0, 1987, 1993, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_91(
        s: &mut ReactiveScratch,
    ) {
        if s.b[2958] {s.store_mul_ad_product_lhs_mixed_ai(1994, A::square(s.ad_value(1992)), 1989, 1989);s.store_mul(1985, 2018, 1994);}
        s.b[2974] = (s.v[0] == (-1.0));s.store_scalar(2974, if s.b[2974] { 1.0 } else { 0.0 });
        if (s.b[2958] && s.b[2974]) {s.store_div_scaled_value_offset_denominator(1985, s.ad_value(1985), 1.0, A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0, 1.0);}
        if s.b[2958] {s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);s.store_add_scaled_product_indices(2028, 1993, 1.0, 1994, 1990, (-1.0));s.store_mul_sub_mixed_iia(1953, 2019, 2017, A::div_scaled_product(A::mul3(s.ad_value(1985), s.ad_value(1987), s.ad_value(2028)), s.ad_value(2019), 1.0, s.ad_value(2027), 1.0));}
        s.b[2975] = (s.v[1] >= 4.0);s.store_scalar(2975, if s.b[2975] { 1.0 } else { 0.0 });s.b[2976] = (s.v[1] == 5.0);s.store_scalar(2976, if s.b[2976] { 1.0 } else { 0.0 });
        if (s.b[2975] && s.b[2976]) {s.store_add_scaled_inputs_mixed_ai(1992, A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1971), (-630.0), s.ad_value(1972), 12.0, s.ad_value(1973), 582.0, s.ad_value(1979), (-97.0)), 1.0, s.ad_value(1978), 7.0), 1.0, s.ad_value(1969), 42.0), 0.007692307692307693, 1970, (168.0 * 0.007692307692307693));s.store_sub_scaled_inputs_mixed_ai(1993, A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1972), (-10152.0), s.ad_value(1973), 6048.0, s.ad_value(1971), 6480.0, s.ad_value(1979), (-1008.0)), 1.0, s.ad_value(1978), 72.0), 1.0, s.ad_value(1969), 432.0), 0.015384615384615385, 1970, (1728.0 * 0.015384615384615385));}
        s.b[2977] = (s.v[1] == 9.0);s.store_scalar(2977, if s.b[2977] { 1.0 } else { 0.0 });
        if ((s.b[2975] && (!s.b[2976])) && s.b[2977]) {s.store_add_scaled_inputs_mixed_ai(1992, A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1974), (-81480.0), s.ad_value(1972), (-30.0), s.ad_value(1971), (-303975.0), s.ad_value(1976), (-5820.0)), 1.0, s.ad_value(1977), 1455.0), 1.0, s.ad_value(1969), 20265.0), 1.0, s.ad_value(1975), 21825.0), 2.6434745829918846e-5, s.ad_value(1970), (81060.0 * 2.6434745829918846e-5)), 1.0, s.ad_value(1979), (485.0 / 75658.0)), 1.0, s.ad_value(1973), (1455.0 * 0.0055248618784530384)), 1.0, 1978, (6755.0 * 1.3217372914959423e-5));s.store_add_scaled_inputs_mixed_ai(1993, A::sub_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1969), 702000.0, s.ad_value(1975), 756000.0, s.ad_value(1972), (-16614600.0), s.ad_value(1971), 10530000.0), 1.0, s.ad_value(1970), 2808000.0), 1.0, s.ad_value(1978), 117000.0), 1.0, s.ad_value(1979), 8400.0), 1.0, s.ad_value(1976), 201600.0), 1.0, s.ad_value(1977), 50400.0), 2.6434745829918846e-5, s.ad_value(1974), (2822400.0 * 2.6434745829918846e-5)), 1.0, 1973, (50400.0 * 0.0055248618784530384));}
        if ((s.b[2975] && (!s.b[2976])) && (!s.b[2977])) {s.store_scalar(1992, 0.0);s.store_scalar(1993, 0.0);}
        if s.b[2975] {s.store_add_div_lhs_indices(2027, 1972, 1937, 1890);}
        s.b[2978] = (((s.v[2027]) as f64).abs() <= s.v[1941]);s.store_scalar(2978, if s.b[2978] { 1.0 } else { 0.0 });
        if (s.b[2975] && s.b[2978]) {s.store_div(2016, 2027, 1940);}
        s.b[2979] = (s.v[2027] < (-s.v[1941]));s.store_scalar(2979, if s.b[2979] { 1.0 } else { 0.0 });
        if ((s.b[2975] && (!s.b[2978])) && s.b[2979]) {s.store_neg(1999, 2027);s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_92(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2975] && (!s.b[2978])) && s.b[2979]) {s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_sub_square_product_mixed_ia(2002, 1999, 2001, 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);s.store_add(824, 2002, 2003);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);s.store_add_mixed_ia(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));}
        s.b[2980] = (((s.v[2015]) as f64).abs() < 230.25850929940458);s.store_scalar(2980, if s.b[2980] { 1.0 } else { 0.0 });
        if (((s.b[2975] && (!s.b[2978])) && s.b[2979]) && s.b[2980]) {s.store_exp(2005, 2015);}
        s.b[2981] = (s.v[2015] < 0.0);s.store_scalar(2981, if s.b[2981] { 1.0 } else { 0.0 });
        if ((((s.b[2975] && (!s.b[2978])) && s.b[2979]) && (!s.b[2980])) && s.b[2981]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2975] && (!s.b[2978])) && s.b[2979]) && (!s.b[2980])) && (!s.b[2981])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2975] && (!s.b[2978])) && s.b[2979]) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs_product_mixed_iiia(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);s.store_add_scaled_sub_square_product_mixed_ia(2008, 1999, 2015, 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_neg_add(2016, 2015, 2012);}
        if ((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) {s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);s.store_mul_scale_offset_mixed_ia(2010, 1998, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), 1.0, (-1.0));s.store_mul_scale_offset(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0, 1.0);}
        s.b[2982] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);s.store_scalar(2982, if s.b[2982] { 1.0 } else { 0.0 });
        if (((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) && s.b[2982]) {s.store_exp_neg_input(2009, 2011);}
        s.b[2983] = ((-s.v[2011]) < 0.0);s.store_scalar(2983, if s.b[2983] { 1.0 } else { 0.0 });
        if ((((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) && (!s.b[2982])) && s.b[2983]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) && (!s.b[2982])) && (!s.b[2983])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) {s.store_sub_from_scalar(2012, 1.0, 2009);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_93(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) {s.store_add_scaled_inputs_product_mixed_iiia(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));}
        s.b[2984] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);s.store_scalar(2984, if s.b[2984] { 1.0 } else { 0.0 });
        if (((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) && s.b[2984]) {s.store_exp_neg_input(2005, 2013);}
        s.b[2985] = ((-s.v[2013]) < 0.0);s.store_scalar(2985, if s.b[2985] { 1.0 } else { 0.0 });
        if ((((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) && (!s.b[2984])) && s.b[2985]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) && (!s.b[2984])) && (!s.b[2985])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);s.store_add_scaled_sub_square_product_mixed_ia(2008, 2027, 2013, 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_add(2016, 2013, 2014);}
        s.b[2986] = (((s.v[2016]) as f64).abs() <= s.v[1933]);s.store_scalar(2986, if s.b[2986] { 1.0 } else { 0.0 });
        if (s.b[2975] && s.b[2986]) {s.store_mul_ad_affine_product_rhs(1996, 2016, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(1991, 1889, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.25), 0.3333333333333333), 1.0, (-0.70710678));s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1990, 1889, 1.0, 2016, 0.5, (-0.235702));}
        s.b[2987] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);s.store_scalar(2987, if s.b[2987] { 1.0 } else { 0.0 });
        if ((s.b[2975] && (!s.b[2986])) && s.b[2987]) {s.store_exp_neg_input(2027, 2016);}
        s.b[2988] = ((-s.v[2016]) < 0.0);s.store_scalar(2988, if s.b[2988] { 1.0 } else { 0.0 });
        if (((s.b[2975] && (!s.b[2986])) && (!s.b[2987])) && s.b[2988]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2016)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[2975] && (!s.b[2986])) && (!s.b[2987])) && (!s.b[2988])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(2016)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (s.b[2975] && (!s.b[2986])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));}
        s.b[2989] = (s.v[2016] > s.v[1933]);s.store_scalar(2989, if s.b[2989] { 1.0 } else { 0.0 });
        if ((s.b[2975] && (!s.b[2986])) && s.b[2989]) {s.store_neg(1996, 1996);}
        if (s.b[2975] && (!s.b[2986])) {s.store_div_scaled_product3_mixed_iiai(1991, 1889, 1889, A::sub_from_scalar(1.0, s.ad_value(2027)), 0.5, 1996, 1.0);s.store_add_div_lhs_mixed_ai(1990, A::add_scaled_square_product(s.ad_value(1991), 1.0, s.ad_value(1889), s.ad_value(1889), (-0.5)), 1996, 1991);}
        if s.b[2975] {s.store_sub(1988, 1937, 1991);s.store_div_from_scalar(1989, 1.0, 1988);s.store_offset_mul(1987, 1972, 1989, (-1.0));s.store_mul_scale_offset_mixed_ia(1986, 1989, A::mul(A::mul3(s.ad_value(1972), s.ad_value(1990), s.ad_value(1989)), s.ad_value(1989)), -1.0, 1.0);s.store_add_scaled_product_mixed_aii(2017, A::mul3(s.ad_value(1986), s.ad_value(1992), s.ad_value(1992)), 1.0, 1987, 1993, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_94(
        s: &mut ReactiveScratch,
    ) {
        if s.b[2975] {s.store_mul_ad_product_lhs_mixed_ai(1994, A::square(s.ad_value(1992)), 1989, 1989);s.store_mul(1985, 2018, 1994);}
        s.b[2990] = (s.v[0] == (-1.0));s.store_scalar(2990, if s.b[2990] { 1.0 } else { 0.0 });
        if (s.b[2975] && s.b[2990]) {s.store_div_scaled_value_offset_denominator(1985, s.ad_value(1985), 1.0, A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0, 1.0);}
        if s.b[2975] {s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);s.store_add_scaled_product_indices(2028, 1993, 1.0, 1994, 1990, (-1.0));s.store_mul_sub_mixed_iia(1954, 2019, 2017, A::div_scaled_product(A::mul3(s.ad_value(1985), s.ad_value(1987), s.ad_value(2028)), s.ad_value(2019), 1.0, s.ad_value(2027), 1.0));}
        s.b[2991] = (s.v[1] >= 5.0);s.store_scalar(2991, if s.b[2991] { 1.0 } else { 0.0 });s.b[2992] = (s.v[1] == 5.0);s.store_scalar(2992, if s.b[2992] { 1.0 } else { 0.0 });
        if (s.b[2991] && s.b[2992]) {s.store_sub_scaled_inputs_mixed_ai(1992, A::add_scaled_inputs(A::sub(A::add_scaled_inputs4(s.ad_value(1972), (-336.0), s.ad_value(1973), 84.0, s.ad_value(1971), 90.0, s.ad_value(1979), 181.0), s.ad_value(1978)), 1.0, s.ad_value(1969), 6.0), 0.015384615384615385, 1970, (24.0 * 0.015384615384615385));s.store_sub_scaled_inputs_mixed_ai(1993, A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1978), 18.0, s.ad_value(1979), 3762.0, s.ad_value(1972), 6048.0, s.ad_value(1970), 432.0), 1.0, s.ad_value(1971), 1620.0), 1.0, s.ad_value(1969), 108.0), 0.015384615384615385, 1973, (8532.0 * 0.015384615384615385));}
        s.b[2993] = (s.v[1] == 9.0);s.store_scalar(2993, if s.b[2993] { 1.0 } else { 0.0 });
        if ((s.b[2991] && (!s.b[2992])) && s.b[2993]) {s.store_scaled_sub_ad(1992, A::add(A::add(A::add_scaled_inputs4(s.ad_value(1974), 1680.0, s.ad_value(1972), (-1680.0), s.ad_value(1979), 5.0, s.ad_value(1978), (-5.0)), A::sub_scaled_inputs(s.ad_value(1971), 450.0, s.ad_value(1975), 450.0)), A::sub_scaled_inputs(s.ad_value(1976), 120.0, s.ad_value(1970), 120.0)), A::sub_scaled_inputs(s.ad_value(1977), 30.0, s.ad_value(1969), 30.0), 0.004784688995215311);s.store_scaled_add_ad(1993, A::add(A::add(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1969), (-900.0), s.ad_value(1977), (-900.0), s.ad_value(1975), (-13500.0), s.ad_value(1971), (-13500.0)), 1.0, s.ad_value(1973), 79500.0), A::add_scaled_inputs(s.ad_value(1972), 50400.0, s.ad_value(1974), 50400.0)), A::add_scaled_inputs(s.ad_value(1970), 3600.0, s.ad_value(1976), 3600.0)), A::add_scaled_inputs(s.ad_value(1978), 150.0, s.ad_value(1979), 150.0), 0.0055248618784530384);}
        if ((s.b[2991] && (!s.b[2992])) && (!s.b[2993])) {s.store_scalar(1992, 0.0);s.store_scalar(1993, 0.0);}
        if s.b[2991] {s.store_add_div_lhs_indices(2027, 1973, 1937, 1890);}
        s.b[2994] = (((s.v[2027]) as f64).abs() <= s.v[1941]);s.store_scalar(2994, if s.b[2994] { 1.0 } else { 0.0 });
        if (s.b[2991] && s.b[2994]) {s.store_div(2016, 2027, 1940);}
        s.b[2995] = (s.v[2027] < (-s.v[1941]));s.store_scalar(2995, if s.b[2995] { 1.0 } else { 0.0 });
        if ((s.b[2991] && (!s.b[2994])) && s.b[2995]) {s.store_neg(1999, 2027);s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_95(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2991] && (!s.b[2994])) && s.b[2995]) {s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_sub_square_product_mixed_ia(2002, 1999, 2001, 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);s.store_add(824, 2002, 2003);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);s.store_add_mixed_ia(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));}
        s.b[2996] = (((s.v[2015]) as f64).abs() < 230.25850929940458);s.store_scalar(2996, if s.b[2996] { 1.0 } else { 0.0 });
        if (((s.b[2991] && (!s.b[2994])) && s.b[2995]) && s.b[2996]) {s.store_exp(2005, 2015);}
        s.b[2997] = (s.v[2015] < 0.0);s.store_scalar(2997, if s.b[2997] { 1.0 } else { 0.0 });
        if ((((s.b[2991] && (!s.b[2994])) && s.b[2995]) && (!s.b[2996])) && s.b[2997]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2991] && (!s.b[2994])) && s.b[2995]) && (!s.b[2996])) && (!s.b[2997])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2991] && (!s.b[2994])) && s.b[2995]) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs_product_mixed_iiia(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);s.store_add_scaled_sub_square_product_mixed_ia(2008, 1999, 2015, 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_neg_add(2016, 2015, 2012);}
        if ((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) {s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);s.store_mul_scale_offset_mixed_ia(2010, 1998, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), 1.0, (-1.0));s.store_mul_scale_offset(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0, 1.0);}
        s.b[2998] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);s.store_scalar(2998, if s.b[2998] { 1.0 } else { 0.0 });
        if (((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) && s.b[2998]) {s.store_exp_neg_input(2009, 2011);}
        s.b[2999] = ((-s.v[2011]) < 0.0);s.store_scalar(2999, if s.b[2999] { 1.0 } else { 0.0 });
        if ((((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) && (!s.b[2998])) && s.b[2999]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) && (!s.b[2998])) && (!s.b[2999])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) {s.store_sub_from_scalar(2012, 1.0, 2009);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_96(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) {s.store_add_scaled_inputs_product_mixed_iiia(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));}
        s.b[3000] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);s.store_scalar(3000, if s.b[3000] { 1.0 } else { 0.0 });
        if (((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) && s.b[3000]) {s.store_exp_neg_input(2005, 2013);}
        s.b[3001] = ((-s.v[2013]) < 0.0);s.store_scalar(3001, if s.b[3001] { 1.0 } else { 0.0 });
        if ((((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) && (!s.b[3000])) && s.b[3001]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) && (!s.b[3000])) && (!s.b[3001])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);s.store_add_scaled_sub_square_product_mixed_ia(2008, 2027, 2013, 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_add(2016, 2013, 2014);}
        s.b[3002] = (((s.v[2016]) as f64).abs() <= s.v[1933]);s.store_scalar(3002, if s.b[3002] { 1.0 } else { 0.0 });
        if (s.b[2991] && s.b[3002]) {s.store_mul_ad_affine_product_rhs(1996, 2016, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(1991, 1889, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.25), 0.3333333333333333), 1.0, (-0.70710678));s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1990, 1889, 1.0, 2016, 0.5, (-0.235702));}
        s.b[3003] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);s.store_scalar(3003, if s.b[3003] { 1.0 } else { 0.0 });
        if ((s.b[2991] && (!s.b[3002])) && s.b[3003]) {s.store_exp_neg_input(2027, 2016);}
        s.b[3004] = ((-s.v[2016]) < 0.0);s.store_scalar(3004, if s.b[3004] { 1.0 } else { 0.0 });
        if (((s.b[2991] && (!s.b[3002])) && (!s.b[3003])) && s.b[3004]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2016)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[2991] && (!s.b[3002])) && (!s.b[3003])) && (!s.b[3004])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(2016)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (s.b[2991] && (!s.b[3002])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));}
        s.b[3005] = (s.v[2016] > s.v[1933]);s.store_scalar(3005, if s.b[3005] { 1.0 } else { 0.0 });
        if ((s.b[2991] && (!s.b[3002])) && s.b[3005]) {s.store_neg(1996, 1996);}
        if (s.b[2991] && (!s.b[3002])) {s.store_div_scaled_product3_mixed_iiai(1991, 1889, 1889, A::sub_from_scalar(1.0, s.ad_value(2027)), 0.5, 1996, 1.0);s.store_add_div_lhs_mixed_ai(1990, A::add_scaled_square_product(s.ad_value(1991), 1.0, s.ad_value(1889), s.ad_value(1889), (-0.5)), 1996, 1991);}
        if s.b[2991] {s.store_sub(1988, 1937, 1991);s.store_div_from_scalar(1989, 1.0, 1988);s.store_offset_mul(1987, 1973, 1989, (-1.0));s.store_mul_scale_offset_mixed_ia(1986, 1989, A::mul(A::mul3(s.ad_value(1973), s.ad_value(1990), s.ad_value(1989)), s.ad_value(1989)), -1.0, 1.0);s.store_add_scaled_product_mixed_aii(2017, A::mul3(s.ad_value(1986), s.ad_value(1992), s.ad_value(1992)), 1.0, 1987, 1993, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_97(
        s: &mut ReactiveScratch,
    ) {
        if s.b[2991] {s.store_mul_ad_product_lhs_mixed_ai(1994, A::square(s.ad_value(1992)), 1989, 1989);s.store_mul(1985, 2018, 1994);}
        s.b[3006] = (s.v[0] == (-1.0));s.store_scalar(3006, if s.b[3006] { 1.0 } else { 0.0 });
        if (s.b[2991] && s.b[3006]) {s.store_div_scaled_value_offset_denominator(1985, s.ad_value(1985), 1.0, A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0, 1.0);}
        if s.b[2991] {s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);s.store_add_scaled_product_indices(2028, 1993, 1.0, 1994, 1990, (-1.0));s.store_mul_sub_mixed_iia(1955, 2019, 2017, A::div_scaled_product(A::mul3(s.ad_value(1985), s.ad_value(1987), s.ad_value(2028)), s.ad_value(2019), 1.0, s.ad_value(2027), 1.0));}
        s.b[3007] = (s.v[1] >= 6.0);s.store_scalar(3007, if s.b[3007] { 1.0 } else { 0.0 });s.b[3008] = (s.v[1] == 9.0);s.store_scalar(3008, if s.b[3008] { 1.0 } else { 0.0 });
        if (s.b[3007] && s.b[3008]) {s.store_sub_scaled_inputs_mixed_ai(1992, A::sub(A::add_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1974), 30.0, s.ad_value(1972), 81480.0, s.ad_value(1971), (-21825.0), s.ad_value(1976), (-81060.0)), 1.0, s.ad_value(1977), 20265.0), 1.0, s.ad_value(1969), 1455.0), 1.0, s.ad_value(1975), 303975.0), 2.6434745829918846e-5, s.ad_value(1970), (5820.0 * 2.6434745829918846e-5)), A::sub_scaled_inputs(s.ad_value(1979), (6755.0 * 1.3217372914959423e-5), s.ad_value(1978), (485.0 * 1.3217372914959423e-5))), 1.0, 1973, (1455.0 / 181.0));s.store_add_scaled_inputs_mixed_ai(1993, A::sub_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1969), 50400.0, s.ad_value(1975), 10530000.0, s.ad_value(1972), (-2822400.0), s.ad_value(1971), 756000.0), 1.0, s.ad_value(1970), 201600.0), 1.0, s.ad_value(1978), 8400.0), 1.0, s.ad_value(1979), 117000.0), 1.0, s.ad_value(1976), 2808000.0), 1.0, s.ad_value(1977), 702000.0), 2.6434745829918846e-5, s.ad_value(1974), (16614600.0 * 2.6434745829918846e-5)), 1.0, 1973, (50400.0 * 0.0055248618784530384));}
        if (s.b[3007] && (!s.b[3008])) {s.store_scalar(1992, 0.0);s.store_scalar(1993, 0.0);}
        if s.b[3007] {s.store_add_div_lhs_indices(2027, 1974, 1937, 1890);}
        s.b[3009] = (((s.v[2027]) as f64).abs() <= s.v[1941]);s.store_scalar(3009, if s.b[3009] { 1.0 } else { 0.0 });
        if (s.b[3007] && s.b[3009]) {s.store_div(2016, 2027, 1940);}
        s.b[3010] = (s.v[2027] < (-s.v[1941]));s.store_scalar(3010, if s.b[3010] { 1.0 } else { 0.0 });
        if ((s.b[3007] && (!s.b[3009])) && s.b[3010]) {s.store_neg(1999, 2027);s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_sub_square_product_mixed_ia(2002, 1999, 2001, 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);s.store_add(824, 2002, 2003);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_98(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[3007] && (!s.b[3009])) && s.b[3010]) {s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);s.store_add_mixed_ia(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));}
        s.b[3011] = (((s.v[2015]) as f64).abs() < 230.25850929940458);s.store_scalar(3011, if s.b[3011] { 1.0 } else { 0.0 });
        if (((s.b[3007] && (!s.b[3009])) && s.b[3010]) && s.b[3011]) {s.store_exp(2005, 2015);}
        s.b[3012] = (s.v[2015] < 0.0);s.store_scalar(3012, if s.b[3012] { 1.0 } else { 0.0 });
        if ((((s.b[3007] && (!s.b[3009])) && s.b[3010]) && (!s.b[3011])) && s.b[3012]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[3007] && (!s.b[3009])) && s.b[3010]) && (!s.b[3011])) && (!s.b[3012])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[3007] && (!s.b[3009])) && s.b[3010]) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs_product_mixed_iiia(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);s.store_add_scaled_sub_square_product_mixed_ia(2008, 1999, 2015, 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_neg_add(2016, 2015, 2012);}
        if ((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) {s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);s.store_mul_scale_offset_mixed_ia(2010, 1998, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), 1.0, (-1.0));s.store_mul_scale_offset(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0, 1.0);}
        s.b[3013] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);s.store_scalar(3013, if s.b[3013] { 1.0 } else { 0.0 });
        if (((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) && s.b[3013]) {s.store_exp_neg_input(2009, 2011);}
        s.b[3014] = ((-s.v[2011]) < 0.0);s.store_scalar(3014, if s.b[3014] { 1.0 } else { 0.0 });
        if ((((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) && (!s.b[3013])) && s.b[3014]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) && (!s.b[3013])) && (!s.b[3014])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) {s.store_sub_from_scalar(2012, 1.0, 2009);s.store_add_scaled_inputs_product_mixed_iiia(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));}
        s.b[3015] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);s.store_scalar(3015, if s.b[3015] { 1.0 } else { 0.0 });
        if (((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) && s.b[3015]) {s.store_exp_neg_input(2005, 2013);}
        s.b[3016] = ((-s.v[2013]) < 0.0);s.store_scalar(3016, if s.b[3016] { 1.0 } else { 0.0 });
        if ((((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) && (!s.b[3015])) && s.b[3016]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) && (!s.b[3015])) && (!s.b[3016])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_99(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) {s.store_add_scaled_sub_square_product_mixed_ia(2008, 2027, 2013, 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_add(2016, 2013, 2014);}
        s.b[3017] = (((s.v[2016]) as f64).abs() <= s.v[1933]);s.store_scalar(3017, if s.b[3017] { 1.0 } else { 0.0 });
        if (s.b[3007] && s.b[3017]) {s.store_mul_ad_affine_product_rhs(1996, 2016, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(1991, 1889, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.25), 0.3333333333333333), 1.0, (-0.70710678));s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1990, 1889, 1.0, 2016, 0.5, (-0.235702));}
        s.b[3018] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);s.store_scalar(3018, if s.b[3018] { 1.0 } else { 0.0 });
        if ((s.b[3007] && (!s.b[3017])) && s.b[3018]) {s.store_exp_neg_input(2027, 2016);}
        s.b[3019] = ((-s.v[2016]) < 0.0);s.store_scalar(3019, if s.b[3019] { 1.0 } else { 0.0 });
        if (((s.b[3007] && (!s.b[3017])) && (!s.b[3018])) && s.b[3019]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2016)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[3007] && (!s.b[3017])) && (!s.b[3018])) && (!s.b[3019])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(2016)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (s.b[3007] && (!s.b[3017])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));}
        s.b[3020] = (s.v[2016] > s.v[1933]);s.store_scalar(3020, if s.b[3020] { 1.0 } else { 0.0 });
        if ((s.b[3007] && (!s.b[3017])) && s.b[3020]) {s.store_neg(1996, 1996);}
        if (s.b[3007] && (!s.b[3017])) {s.store_div_scaled_product3_mixed_iiai(1991, 1889, 1889, A::sub_from_scalar(1.0, s.ad_value(2027)), 0.5, 1996, 1.0);s.store_add_div_lhs_mixed_ai(1990, A::add_scaled_square_product(s.ad_value(1991), 1.0, s.ad_value(1889), s.ad_value(1889), (-0.5)), 1996, 1991);}
        if s.b[3007] {s.store_sub(1988, 1937, 1991);s.store_div_from_scalar(1989, 1.0, 1988);s.store_offset_mul(1987, 1974, 1989, (-1.0));s.store_mul_scale_offset_mixed_ia(1986, 1989, A::mul(A::mul3(s.ad_value(1974), s.ad_value(1990), s.ad_value(1989)), s.ad_value(1989)), -1.0, 1.0);s.store_add_scaled_product_mixed_aii(2017, A::mul3(s.ad_value(1986), s.ad_value(1992), s.ad_value(1992)), 1.0, 1987, 1993, 1.0);s.store_mul_ad_product_lhs_mixed_ai(1994, A::square(s.ad_value(1992)), 1989, 1989);s.store_mul(1985, 2018, 1994);}
        s.b[3021] = (s.v[0] == (-1.0));s.store_scalar(3021, if s.b[3021] { 1.0 } else { 0.0 });
        if (s.b[3007] && s.b[3021]) {s.store_div_scaled_value_offset_denominator(1985, s.ad_value(1985), 1.0, A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0, 1.0);}
        if s.b[3007] {s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);s.store_add_scaled_product_indices(2028, 1993, 1.0, 1994, 1990, (-1.0));s.store_mul_sub_mixed_iia(1956, 2019, 2017, A::div_scaled_product(A::mul3(s.ad_value(1985), s.ad_value(1987), s.ad_value(2028)), s.ad_value(2019), 1.0, s.ad_value(2027), 1.0));}
        s.b[3022] = (s.v[1] >= 7.0);s.store_scalar(3022, if s.b[3022] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_100(
        s: &mut ReactiveScratch,
    ) {
        s.b[3023] = (s.v[1] == 9.0);s.store_scalar(3023, if s.b[3023] { 1.0 } else { 0.0 });
        if (s.b[3022] && s.b[3023]) {s.store_add_scaled_inputs_mixed_ai(1992, A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1974), (-304200.0), s.ad_value(1972), (-21840.0), s.ad_value(1979), 12605.0, s.ad_value(1971), 5850.0), 1.0, s.ad_value(1976), 302520.0), 1.0, s.ad_value(1978), 65.0), 1.0, s.ad_value(1977), 75630.0), 1.0, s.ad_value(1969), 390.0), 1.0, s.ad_value(1975), 420.0), 2.6434745829918846e-5, s.ad_value(1970), (1560.0 * 2.6434745829918846e-5)), 1.0, 1973, (390.0 / 181.0));s.store_sub_scaled_inputs_mixed_ai(1993, A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1969), (-13500.0), s.ad_value(1975), (-16601100.0), s.ad_value(1972), 756000.0, s.ad_value(1971), (-202500.0)), 1.0, s.ad_value(1970), 54000.0), 1.0, s.ad_value(1978), 2250.0), 1.0, s.ad_value(1979), 436650.0), 1.0, s.ad_value(1976), 10479600.0), 1.0, s.ad_value(1977), 2619900.0), 2.6434745829918846e-5, s.ad_value(1974), (10530000.0 * 2.6434745829918846e-5)), 1.0, 1973, (13500.0 * 0.0055248618784530384));}
        if (s.b[3022] && (!s.b[3023])) {s.store_scalar(1992, 0.0);s.store_scalar(1993, 0.0);}
        if s.b[3022] {s.store_add_div_lhs_indices(2027, 1975, 1937, 1890);}
        s.b[3024] = (((s.v[2027]) as f64).abs() <= s.v[1941]);s.store_scalar(3024, if s.b[3024] { 1.0 } else { 0.0 });
        if (s.b[3022] && s.b[3024]) {s.store_div(2016, 2027, 1940);}
        s.b[3025] = (s.v[2027] < (-s.v[1941]));s.store_scalar(3025, if s.b[3025] { 1.0 } else { 0.0 });
        if ((s.b[3022] && (!s.b[3024])) && s.b[3025]) {s.store_neg(1999, 2027);s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_sub_square_product_mixed_ia(2002, 1999, 2001, 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);s.store_add(824, 2002, 2003);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);s.store_add_mixed_ia(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));}
        s.b[3026] = (((s.v[2015]) as f64).abs() < 230.25850929940458);s.store_scalar(3026, if s.b[3026] { 1.0 } else { 0.0 });
        if (((s.b[3022] && (!s.b[3024])) && s.b[3025]) && s.b[3026]) {s.store_exp(2005, 2015);}
    }
}

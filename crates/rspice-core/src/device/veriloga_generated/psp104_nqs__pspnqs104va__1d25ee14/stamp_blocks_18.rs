#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

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
        p: &Parameters,
        multiplicity: f64,
        var_chnl_type: f64,
        var_guard1924: f64,
        var_i_ds: f64,
        var_i_ds_dn12: f64,
        var_i_ds_dn13: f64,
        var_i_ds_dn14: f64,
        var_i_ds_dn15: f64,
        var_i_ds_dn16: f64,
        var_i_ds_dn17: f64,
        var_i_ds_dn18: f64,
        var_i_ds_dn19: f64,
        var_i_ds_dn20: f64,
        var_i_ds_dn5: f64,
        var_i_ds_dn6: f64,
        var_i_ds_dn7: f64,
        var_i_ds_dn8: f64,
        var_i_dsedge: f64,
        var_i_dsedge_dn12: f64,
        var_i_dsedge_dn13: f64,
        var_i_dsedge_dn14: f64,
        var_i_dsedge_dn15: f64,
        var_i_dsedge_dn16: f64,
        var_i_dsedge_dn17: f64,
        var_i_dsedge_dn18: f64,
        var_i_dsedge_dn19: f64,
        var_i_dsedge_dn20: f64,
        var_i_dsedge_dn5: f64,
        var_i_dsedge_dn6: f64,
        var_i_dsedge_dn7: f64,
        var_i_dsedge_dn8: f64,
        var_i_gb: f64,
        var_i_gb_dn12: f64,
        var_i_gb_dn13: f64,
        var_i_gb_dn14: f64,
        var_i_gb_dn15: f64,
        var_i_gb_dn16: f64,
        var_i_gb_dn17: f64,
        var_i_gb_dn18: f64,
        var_i_gb_dn19: f64,
        var_i_gb_dn20: f64,
        var_i_gb_dn5: f64,
        var_i_gb_dn6: f64,
        var_i_gb_dn7: f64,
        var_i_gb_dn8: f64,
        var_i_gcd: f64,
        var_i_gcd_dn12: f64,
        var_i_gcd_dn13: f64,
        var_i_gcd_dn14: f64,
        var_i_gcd_dn15: f64,
        var_i_gcd_dn16: f64,
        var_i_gcd_dn17: f64,
        var_i_gcd_dn18: f64,
        var_i_gcd_dn19: f64,
        var_i_gcd_dn20: f64,
        var_i_gcd_dn5: f64,
        var_i_gcd_dn6: f64,
        var_i_gcd_dn7: f64,
        var_i_gcd_dn8: f64,
        var_i_gcs: f64,
        var_i_gcs_dn12: f64,
        var_i_gcs_dn13: f64,
        var_i_gcs_dn14: f64,
        var_i_gcs_dn15: f64,
        var_i_gcs_dn16: f64,
        var_i_gcs_dn17: f64,
        var_i_gcs_dn18: f64,
        var_i_gcs_dn19: f64,
        var_i_gcs_dn20: f64,
        var_i_gcs_dn5: f64,
        var_i_gcs_dn6: f64,
        var_i_gcs_dn7: f64,
        var_i_gcs_dn8: f64,
        var_i_gidl: f64,
        var_i_gidl_dn12: f64,
        var_i_gidl_dn13: f64,
        var_i_gidl_dn14: f64,
        var_i_gidl_dn15: f64,
        var_i_gidl_dn16: f64,
        var_i_gidl_dn17: f64,
        var_i_gidl_dn18: f64,
        var_i_gidl_dn19: f64,
        var_i_gidl_dn20: f64,
        var_i_gidl_dn5: f64,
        var_i_gidl_dn6: f64,
        var_i_gidl_dn7: f64,
        var_i_gidl_dn8: f64,
        var_i_gisl: f64,
        var_i_gisl_dn12: f64,
        var_i_gisl_dn13: f64,
        var_i_gisl_dn14: f64,
        var_i_gisl_dn15: f64,
        var_i_gisl_dn16: f64,
        var_i_gisl_dn17: f64,
        var_i_gisl_dn18: f64,
        var_i_gisl_dn19: f64,
        var_i_gisl_dn20: f64,
        var_i_gisl_dn5: f64,
        var_i_gisl_dn6: f64,
        var_i_gisl_dn7: f64,
        var_i_gisl_dn8: f64,
        var_igdov: f64,
        var_igdov_dn12: f64,
        var_igdov_dn13: f64,
        var_igdov_dn14: f64,
        var_igdov_dn15: f64,
        var_igdov_dn16: f64,
        var_igdov_dn17: f64,
        var_igdov_dn18: f64,
        var_igdov_dn19: f64,
        var_igdov_dn20: f64,
        var_igdov_dn5: f64,
        var_igdov_dn6: f64,
        var_igdov_dn7: f64,
        var_igdov_dn8: f64,
        var_igsov: f64,
        var_igsov_dn12: f64,
        var_igsov_dn13: f64,
        var_igsov_dn14: f64,
        var_igsov_dn15: f64,
        var_igsov_dn16: f64,
        var_igsov_dn17: f64,
        var_igsov_dn18: f64,
        var_igsov_dn19: f64,
        var_igsov_dn20: f64,
        var_igsov_dn5: f64,
        var_igsov_dn6: f64,
        var_igsov_dn7: f64,
        var_igsov_dn8: f64,
        var_iimpact: f64,
        var_iimpact_dn12: f64,
        var_iimpact_dn13: f64,
        var_iimpact_dn14: f64,
        var_iimpact_dn15: f64,
        var_iimpact_dn16: f64,
        var_iimpact_dn17: f64,
        var_iimpact_dn18: f64,
        var_iimpact_dn19: f64,
        var_iimpact_dn20: f64,
        var_iimpact_dn5: f64,
        var_iimpact_dn6: f64,
        var_iimpact_dn7: f64,
        var_iimpact_dn8: f64,
        var_ijun_s: f64,
        var_ijun_s_dn10: f64,
        var_ijun_s_dn11: f64,
        var_ijun_s_dn5: f64,
        var_ijun_s_dn6: f64,
        var_ijun_s_dn7: f64,
        var_ijun_s_dn8: f64,
        var_mult_inst: f64,
    ) {
        let (eq0_e955, eq0_e955_d_n5, eq0_e955_d_n6, eq0_e955_d_n7, eq0_e955_d_n8, eq0_e955_d_n12, eq0_e955_d_n13, eq0_e955_d_n14, eq0_e955_d_n15, eq0_e955_d_n16, eq0_e955_d_n17, eq0_e955_d_n18, eq0_e955_d_n19, eq0_e955_d_n20,) = {
    if (var_guard1924 != 0.0) {
        let eq0_e949: f64 = (var_chnl_type * var_mult_inst);
        let eq0_e951: f64 = (eq0_e949 * p.p32);
        let eq0_e953: f64 = (eq0_e951 * var_iimpact);
        let eq0_e953_d_n5: f64 = (eq0_e951 * var_iimpact_dn5);
        let eq0_e953_d_n6: f64 = (eq0_e951 * var_iimpact_dn6);
        let eq0_e953_d_n7: f64 = (eq0_e951 * var_iimpact_dn7);
        let eq0_e953_d_n8: f64 = (eq0_e951 * var_iimpact_dn8);
        let eq0_e953_d_n12: f64 = (eq0_e951 * var_iimpact_dn12);
        let eq0_e953_d_n13: f64 = (eq0_e951 * var_iimpact_dn13);
        let eq0_e953_d_n14: f64 = (eq0_e951 * var_iimpact_dn14);
        let eq0_e953_d_n15: f64 = (eq0_e951 * var_iimpact_dn15);
        let eq0_e953_d_n16: f64 = (eq0_e951 * var_iimpact_dn16);
        let eq0_e953_d_n17: f64 = (eq0_e951 * var_iimpact_dn17);
        let eq0_e953_d_n18: f64 = (eq0_e951 * var_iimpact_dn18);
        let eq0_e953_d_n19: f64 = (eq0_e951 * var_iimpact_dn19);
        let eq0_e953_d_n20: f64 = (eq0_e951 * var_iimpact_dn20);
        (eq0_e953, eq0_e953_d_n5, eq0_e953_d_n6, eq0_e953_d_n7, eq0_e953_d_n8, eq0_e953_d_n12, eq0_e953_d_n13, eq0_e953_d_n14, eq0_e953_d_n15, eq0_e953_d_n16, eq0_e953_d_n17, eq0_e953_d_n18, eq0_e953_d_n19, eq0_e953_d_n20,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e955;
        let eq0_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq0_node_derivatives: [f64; 13] = [eq0_e955_d_n5, eq0_e955_d_n6, eq0_e955_d_n7, eq0_e955_d_n8, eq0_e955_d_n12, eq0_e955_d_n13, eq0_e955_d_n14, eq0_e955_d_n15, eq0_e955_d_n16, eq0_e955_d_n17, eq0_e955_d_n18, eq0_e955_d_n19, eq0_e955_d_n20];
        let eq0_branch_derivative_indices: [usize; 0] = [];
        let eq0_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq0_value),
            &eq0_node_derivative_indices,
            &eq0_node_derivatives,
            &eq0_branch_derivative_indices,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let (eq1_e967, eq1_e967_d_n5, eq1_e967_d_n6, eq1_e967_d_n7, eq1_e967_d_n8, eq1_e967_d_n12, eq1_e967_d_n13, eq1_e967_d_n14, eq1_e967_d_n15, eq1_e967_d_n16, eq1_e967_d_n17, eq1_e967_d_n18, eq1_e967_d_n19, eq1_e967_d_n20,) = {
    if (var_guard1924 != 0.0) {
        let eq1_e959: f64 = (var_chnl_type * var_mult_inst);
        let eq1_e961: f64 = (eq1_e959 * p.p32);
        let eq1_e964: f64 = (var_i_ds + var_i_dsedge);
        let eq1_e964_d_n5: f64 = (var_i_ds_dn5 + var_i_dsedge_dn5);
        let eq1_e964_d_n6: f64 = (var_i_ds_dn6 + var_i_dsedge_dn6);
        let eq1_e964_d_n7: f64 = (var_i_ds_dn7 + var_i_dsedge_dn7);
        let eq1_e964_d_n8: f64 = (var_i_ds_dn8 + var_i_dsedge_dn8);
        let eq1_e964_d_n12: f64 = (var_i_ds_dn12 + var_i_dsedge_dn12);
        let eq1_e964_d_n13: f64 = (var_i_ds_dn13 + var_i_dsedge_dn13);
        let eq1_e964_d_n14: f64 = (var_i_ds_dn14 + var_i_dsedge_dn14);
        let eq1_e964_d_n15: f64 = (var_i_ds_dn15 + var_i_dsedge_dn15);
        let eq1_e964_d_n16: f64 = (var_i_ds_dn16 + var_i_dsedge_dn16);
        let eq1_e964_d_n17: f64 = (var_i_ds_dn17 + var_i_dsedge_dn17);
        let eq1_e964_d_n18: f64 = (var_i_ds_dn18 + var_i_dsedge_dn18);
        let eq1_e964_d_n19: f64 = (var_i_ds_dn19 + var_i_dsedge_dn19);
        let eq1_e964_d_n20: f64 = (var_i_ds_dn20 + var_i_dsedge_dn20);
        let eq1_e965: f64 = (eq1_e961 * eq1_e964);
        let eq1_e965_d_n5: f64 = (eq1_e961 * eq1_e964_d_n5);
        let eq1_e965_d_n6: f64 = (eq1_e961 * eq1_e964_d_n6);
        let eq1_e965_d_n7: f64 = (eq1_e961 * eq1_e964_d_n7);
        let eq1_e965_d_n8: f64 = (eq1_e961 * eq1_e964_d_n8);
        let eq1_e965_d_n12: f64 = (eq1_e961 * eq1_e964_d_n12);
        let eq1_e965_d_n13: f64 = (eq1_e961 * eq1_e964_d_n13);
        let eq1_e965_d_n14: f64 = (eq1_e961 * eq1_e964_d_n14);
        let eq1_e965_d_n15: f64 = (eq1_e961 * eq1_e964_d_n15);
        let eq1_e965_d_n16: f64 = (eq1_e961 * eq1_e964_d_n16);
        let eq1_e965_d_n17: f64 = (eq1_e961 * eq1_e964_d_n17);
        let eq1_e965_d_n18: f64 = (eq1_e961 * eq1_e964_d_n18);
        let eq1_e965_d_n19: f64 = (eq1_e961 * eq1_e964_d_n19);
        let eq1_e965_d_n20: f64 = (eq1_e961 * eq1_e964_d_n20);
        (eq1_e965, eq1_e965_d_n5, eq1_e965_d_n6, eq1_e965_d_n7, eq1_e965_d_n8, eq1_e965_d_n12, eq1_e965_d_n13, eq1_e965_d_n14, eq1_e965_d_n15, eq1_e965_d_n16, eq1_e965_d_n17, eq1_e965_d_n18, eq1_e965_d_n19, eq1_e965_d_n20,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e967;
        let eq1_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq1_node_derivatives: [f64; 13] = [eq1_e967_d_n5, eq1_e967_d_n6, eq1_e967_d_n7, eq1_e967_d_n8, eq1_e967_d_n12, eq1_e967_d_n13, eq1_e967_d_n14, eq1_e967_d_n15, eq1_e967_d_n16, eq1_e967_d_n17, eq1_e967_d_n18, eq1_e967_d_n19, eq1_e967_d_n20];
        let eq1_branch_derivative_indices: [usize; 0] = [];
        let eq1_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq1_value),
            &eq1_node_derivative_indices,
            &eq1_node_derivatives,
            &eq1_branch_derivative_indices,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let (eq2_e977, eq2_e977_d_n5, eq2_e977_d_n6, eq2_e977_d_n7, eq2_e977_d_n8, eq2_e977_d_n12, eq2_e977_d_n13, eq2_e977_d_n14, eq2_e977_d_n15, eq2_e977_d_n16, eq2_e977_d_n17, eq2_e977_d_n18, eq2_e977_d_n19, eq2_e977_d_n20,) = {
    if (var_guard1924 != 0.0) {
        let eq2_e971: f64 = (var_chnl_type * var_mult_inst);
        let eq2_e973: f64 = (eq2_e971 * p.p32);
        let eq2_e975: f64 = (eq2_e973 * var_i_gcs);
        let eq2_e975_d_n5: f64 = (eq2_e973 * var_i_gcs_dn5);
        let eq2_e975_d_n6: f64 = (eq2_e973 * var_i_gcs_dn6);
        let eq2_e975_d_n7: f64 = (eq2_e973 * var_i_gcs_dn7);
        let eq2_e975_d_n8: f64 = (eq2_e973 * var_i_gcs_dn8);
        let eq2_e975_d_n12: f64 = (eq2_e973 * var_i_gcs_dn12);
        let eq2_e975_d_n13: f64 = (eq2_e973 * var_i_gcs_dn13);
        let eq2_e975_d_n14: f64 = (eq2_e973 * var_i_gcs_dn14);
        let eq2_e975_d_n15: f64 = (eq2_e973 * var_i_gcs_dn15);
        let eq2_e975_d_n16: f64 = (eq2_e973 * var_i_gcs_dn16);
        let eq2_e975_d_n17: f64 = (eq2_e973 * var_i_gcs_dn17);
        let eq2_e975_d_n18: f64 = (eq2_e973 * var_i_gcs_dn18);
        let eq2_e975_d_n19: f64 = (eq2_e973 * var_i_gcs_dn19);
        let eq2_e975_d_n20: f64 = (eq2_e973 * var_i_gcs_dn20);
        (eq2_e975, eq2_e975_d_n5, eq2_e975_d_n6, eq2_e975_d_n7, eq2_e975_d_n8, eq2_e975_d_n12, eq2_e975_d_n13, eq2_e975_d_n14, eq2_e975_d_n15, eq2_e975_d_n16, eq2_e975_d_n17, eq2_e975_d_n18, eq2_e975_d_n19, eq2_e975_d_n20,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e977;
        let eq2_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq2_node_derivatives: [f64; 13] = [eq2_e977_d_n5, eq2_e977_d_n6, eq2_e977_d_n7, eq2_e977_d_n8, eq2_e977_d_n12, eq2_e977_d_n13, eq2_e977_d_n14, eq2_e977_d_n15, eq2_e977_d_n16, eq2_e977_d_n17, eq2_e977_d_n18, eq2_e977_d_n19, eq2_e977_d_n20];
        let eq2_branch_derivative_indices: [usize; 0] = [];
        let eq2_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq2_value),
            &eq2_node_derivative_indices,
            &eq2_node_derivatives,
            &eq2_branch_derivative_indices,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let (eq3_e987, eq3_e987_d_n5, eq3_e987_d_n6, eq3_e987_d_n7, eq3_e987_d_n8, eq3_e987_d_n12, eq3_e987_d_n13, eq3_e987_d_n14, eq3_e987_d_n15, eq3_e987_d_n16, eq3_e987_d_n17, eq3_e987_d_n18, eq3_e987_d_n19, eq3_e987_d_n20,) = {
    if (var_guard1924 != 0.0) {
        let eq3_e981: f64 = (var_chnl_type * var_mult_inst);
        let eq3_e983: f64 = (eq3_e981 * p.p32);
        let eq3_e985: f64 = (eq3_e983 * var_i_gcd);
        let eq3_e985_d_n5: f64 = (eq3_e983 * var_i_gcd_dn5);
        let eq3_e985_d_n6: f64 = (eq3_e983 * var_i_gcd_dn6);
        let eq3_e985_d_n7: f64 = (eq3_e983 * var_i_gcd_dn7);
        let eq3_e985_d_n8: f64 = (eq3_e983 * var_i_gcd_dn8);
        let eq3_e985_d_n12: f64 = (eq3_e983 * var_i_gcd_dn12);
        let eq3_e985_d_n13: f64 = (eq3_e983 * var_i_gcd_dn13);
        let eq3_e985_d_n14: f64 = (eq3_e983 * var_i_gcd_dn14);
        let eq3_e985_d_n15: f64 = (eq3_e983 * var_i_gcd_dn15);
        let eq3_e985_d_n16: f64 = (eq3_e983 * var_i_gcd_dn16);
        let eq3_e985_d_n17: f64 = (eq3_e983 * var_i_gcd_dn17);
        let eq3_e985_d_n18: f64 = (eq3_e983 * var_i_gcd_dn18);
        let eq3_e985_d_n19: f64 = (eq3_e983 * var_i_gcd_dn19);
        let eq3_e985_d_n20: f64 = (eq3_e983 * var_i_gcd_dn20);
        (eq3_e985, eq3_e985_d_n5, eq3_e985_d_n6, eq3_e985_d_n7, eq3_e985_d_n8, eq3_e985_d_n12, eq3_e985_d_n13, eq3_e985_d_n14, eq3_e985_d_n15, eq3_e985_d_n16, eq3_e985_d_n17, eq3_e985_d_n18, eq3_e985_d_n19, eq3_e985_d_n20,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e987;
        let eq3_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq3_node_derivatives: [f64; 13] = [eq3_e987_d_n5, eq3_e987_d_n6, eq3_e987_d_n7, eq3_e987_d_n8, eq3_e987_d_n12, eq3_e987_d_n13, eq3_e987_d_n14, eq3_e987_d_n15, eq3_e987_d_n16, eq3_e987_d_n17, eq3_e987_d_n18, eq3_e987_d_n19, eq3_e987_d_n20];
        let eq3_branch_derivative_indices: [usize; 0] = [];
        let eq3_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq3_value),
            &eq3_node_derivative_indices,
            &eq3_node_derivatives,
            &eq3_branch_derivative_indices,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let (eq4_e998, eq4_e998_d_n5, eq4_e998_d_n6, eq4_e998_d_n7, eq4_e998_d_n8, eq4_e998_d_n12, eq4_e998_d_n13, eq4_e998_d_n14, eq4_e998_d_n15, eq4_e998_d_n16, eq4_e998_d_n17, eq4_e998_d_n18, eq4_e998_d_n19, eq4_e998_d_n20,) = {
    if (var_guard1924 == 0.0) {
        let eq4_e992: f64 = (var_chnl_type * var_mult_inst);
        let eq4_e994: f64 = (eq4_e992 * p.p32);
        let eq4_e996: f64 = (eq4_e994 * var_iimpact);
        let eq4_e996_d_n5: f64 = (eq4_e994 * var_iimpact_dn5);
        let eq4_e996_d_n6: f64 = (eq4_e994 * var_iimpact_dn6);
        let eq4_e996_d_n7: f64 = (eq4_e994 * var_iimpact_dn7);
        let eq4_e996_d_n8: f64 = (eq4_e994 * var_iimpact_dn8);
        let eq4_e996_d_n12: f64 = (eq4_e994 * var_iimpact_dn12);
        let eq4_e996_d_n13: f64 = (eq4_e994 * var_iimpact_dn13);
        let eq4_e996_d_n14: f64 = (eq4_e994 * var_iimpact_dn14);
        let eq4_e996_d_n15: f64 = (eq4_e994 * var_iimpact_dn15);
        let eq4_e996_d_n16: f64 = (eq4_e994 * var_iimpact_dn16);
        let eq4_e996_d_n17: f64 = (eq4_e994 * var_iimpact_dn17);
        let eq4_e996_d_n18: f64 = (eq4_e994 * var_iimpact_dn18);
        let eq4_e996_d_n19: f64 = (eq4_e994 * var_iimpact_dn19);
        let eq4_e996_d_n20: f64 = (eq4_e994 * var_iimpact_dn20);
        (eq4_e996, eq4_e996_d_n5, eq4_e996_d_n6, eq4_e996_d_n7, eq4_e996_d_n8, eq4_e996_d_n12, eq4_e996_d_n13, eq4_e996_d_n14, eq4_e996_d_n15, eq4_e996_d_n16, eq4_e996_d_n17, eq4_e996_d_n18, eq4_e996_d_n19, eq4_e996_d_n20,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e998;
        let eq4_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq4_node_derivatives: [f64; 13] = [eq4_e998_d_n5, eq4_e998_d_n6, eq4_e998_d_n7, eq4_e998_d_n8, eq4_e998_d_n12, eq4_e998_d_n13, eq4_e998_d_n14, eq4_e998_d_n15, eq4_e998_d_n16, eq4_e998_d_n17, eq4_e998_d_n18, eq4_e998_d_n19, eq4_e998_d_n20];
        let eq4_branch_derivative_indices: [usize; 0] = [];
        let eq4_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq4_value),
            &eq4_node_derivative_indices,
            &eq4_node_derivatives,
            &eq4_branch_derivative_indices,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e1011, eq5_e1011_d_n5, eq5_e1011_d_n6, eq5_e1011_d_n7, eq5_e1011_d_n8, eq5_e1011_d_n12, eq5_e1011_d_n13, eq5_e1011_d_n14, eq5_e1011_d_n15, eq5_e1011_d_n16, eq5_e1011_d_n17, eq5_e1011_d_n18, eq5_e1011_d_n19, eq5_e1011_d_n20,) = {
    if (var_guard1924 == 0.0) {
        let eq5_e1003: f64 = (var_chnl_type * var_mult_inst);
        let eq5_e1005: f64 = (eq5_e1003 * p.p32);
        let eq5_e1008: f64 = (var_i_ds + var_i_dsedge);
        let eq5_e1008_d_n5: f64 = (var_i_ds_dn5 + var_i_dsedge_dn5);
        let eq5_e1008_d_n6: f64 = (var_i_ds_dn6 + var_i_dsedge_dn6);
        let eq5_e1008_d_n7: f64 = (var_i_ds_dn7 + var_i_dsedge_dn7);
        let eq5_e1008_d_n8: f64 = (var_i_ds_dn8 + var_i_dsedge_dn8);
        let eq5_e1008_d_n12: f64 = (var_i_ds_dn12 + var_i_dsedge_dn12);
        let eq5_e1008_d_n13: f64 = (var_i_ds_dn13 + var_i_dsedge_dn13);
        let eq5_e1008_d_n14: f64 = (var_i_ds_dn14 + var_i_dsedge_dn14);
        let eq5_e1008_d_n15: f64 = (var_i_ds_dn15 + var_i_dsedge_dn15);
        let eq5_e1008_d_n16: f64 = (var_i_ds_dn16 + var_i_dsedge_dn16);
        let eq5_e1008_d_n17: f64 = (var_i_ds_dn17 + var_i_dsedge_dn17);
        let eq5_e1008_d_n18: f64 = (var_i_ds_dn18 + var_i_dsedge_dn18);
        let eq5_e1008_d_n19: f64 = (var_i_ds_dn19 + var_i_dsedge_dn19);
        let eq5_e1008_d_n20: f64 = (var_i_ds_dn20 + var_i_dsedge_dn20);
        let eq5_e1009: f64 = (eq5_e1005 * eq5_e1008);
        let eq5_e1009_d_n5: f64 = (eq5_e1005 * eq5_e1008_d_n5);
        let eq5_e1009_d_n6: f64 = (eq5_e1005 * eq5_e1008_d_n6);
        let eq5_e1009_d_n7: f64 = (eq5_e1005 * eq5_e1008_d_n7);
        let eq5_e1009_d_n8: f64 = (eq5_e1005 * eq5_e1008_d_n8);
        let eq5_e1009_d_n12: f64 = (eq5_e1005 * eq5_e1008_d_n12);
        let eq5_e1009_d_n13: f64 = (eq5_e1005 * eq5_e1008_d_n13);
        let eq5_e1009_d_n14: f64 = (eq5_e1005 * eq5_e1008_d_n14);
        let eq5_e1009_d_n15: f64 = (eq5_e1005 * eq5_e1008_d_n15);
        let eq5_e1009_d_n16: f64 = (eq5_e1005 * eq5_e1008_d_n16);
        let eq5_e1009_d_n17: f64 = (eq5_e1005 * eq5_e1008_d_n17);
        let eq5_e1009_d_n18: f64 = (eq5_e1005 * eq5_e1008_d_n18);
        let eq5_e1009_d_n19: f64 = (eq5_e1005 * eq5_e1008_d_n19);
        let eq5_e1009_d_n20: f64 = (eq5_e1005 * eq5_e1008_d_n20);
        (eq5_e1009, eq5_e1009_d_n5, eq5_e1009_d_n6, eq5_e1009_d_n7, eq5_e1009_d_n8, eq5_e1009_d_n12, eq5_e1009_d_n13, eq5_e1009_d_n14, eq5_e1009_d_n15, eq5_e1009_d_n16, eq5_e1009_d_n17, eq5_e1009_d_n18, eq5_e1009_d_n19, eq5_e1009_d_n20,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e1011;
        let eq5_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq5_node_derivatives: [f64; 13] = [eq5_e1011_d_n5, eq5_e1011_d_n6, eq5_e1011_d_n7, eq5_e1011_d_n8, eq5_e1011_d_n12, eq5_e1011_d_n13, eq5_e1011_d_n14, eq5_e1011_d_n15, eq5_e1011_d_n16, eq5_e1011_d_n17, eq5_e1011_d_n18, eq5_e1011_d_n19, eq5_e1011_d_n20];
        let eq5_branch_derivative_indices: [usize; 0] = [];
        let eq5_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq5_value),
            &eq5_node_derivative_indices,
            &eq5_node_derivatives,
            &eq5_branch_derivative_indices,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let (eq6_e1022, eq6_e1022_d_n5, eq6_e1022_d_n6, eq6_e1022_d_n7, eq6_e1022_d_n8, eq6_e1022_d_n12, eq6_e1022_d_n13, eq6_e1022_d_n14, eq6_e1022_d_n15, eq6_e1022_d_n16, eq6_e1022_d_n17, eq6_e1022_d_n18, eq6_e1022_d_n19, eq6_e1022_d_n20,) = {
    if (var_guard1924 == 0.0) {
        let eq6_e1016: f64 = (var_chnl_type * var_mult_inst);
        let eq6_e1018: f64 = (eq6_e1016 * p.p32);
        let eq6_e1020: f64 = (eq6_e1018 * var_i_gcs);
        let eq6_e1020_d_n5: f64 = (eq6_e1018 * var_i_gcs_dn5);
        let eq6_e1020_d_n6: f64 = (eq6_e1018 * var_i_gcs_dn6);
        let eq6_e1020_d_n7: f64 = (eq6_e1018 * var_i_gcs_dn7);
        let eq6_e1020_d_n8: f64 = (eq6_e1018 * var_i_gcs_dn8);
        let eq6_e1020_d_n12: f64 = (eq6_e1018 * var_i_gcs_dn12);
        let eq6_e1020_d_n13: f64 = (eq6_e1018 * var_i_gcs_dn13);
        let eq6_e1020_d_n14: f64 = (eq6_e1018 * var_i_gcs_dn14);
        let eq6_e1020_d_n15: f64 = (eq6_e1018 * var_i_gcs_dn15);
        let eq6_e1020_d_n16: f64 = (eq6_e1018 * var_i_gcs_dn16);
        let eq6_e1020_d_n17: f64 = (eq6_e1018 * var_i_gcs_dn17);
        let eq6_e1020_d_n18: f64 = (eq6_e1018 * var_i_gcs_dn18);
        let eq6_e1020_d_n19: f64 = (eq6_e1018 * var_i_gcs_dn19);
        let eq6_e1020_d_n20: f64 = (eq6_e1018 * var_i_gcs_dn20);
        (eq6_e1020, eq6_e1020_d_n5, eq6_e1020_d_n6, eq6_e1020_d_n7, eq6_e1020_d_n8, eq6_e1020_d_n12, eq6_e1020_d_n13, eq6_e1020_d_n14, eq6_e1020_d_n15, eq6_e1020_d_n16, eq6_e1020_d_n17, eq6_e1020_d_n18, eq6_e1020_d_n19, eq6_e1020_d_n20,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e1022;
        let eq6_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq6_node_derivatives: [f64; 13] = [eq6_e1022_d_n5, eq6_e1022_d_n6, eq6_e1022_d_n7, eq6_e1022_d_n8, eq6_e1022_d_n12, eq6_e1022_d_n13, eq6_e1022_d_n14, eq6_e1022_d_n15, eq6_e1022_d_n16, eq6_e1022_d_n17, eq6_e1022_d_n18, eq6_e1022_d_n19, eq6_e1022_d_n20];
        let eq6_branch_derivative_indices: [usize; 0] = [];
        let eq6_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq6_value),
            &eq6_node_derivative_indices,
            &eq6_node_derivatives,
            &eq6_branch_derivative_indices,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let (eq7_e1033, eq7_e1033_d_n5, eq7_e1033_d_n6, eq7_e1033_d_n7, eq7_e1033_d_n8, eq7_e1033_d_n12, eq7_e1033_d_n13, eq7_e1033_d_n14, eq7_e1033_d_n15, eq7_e1033_d_n16, eq7_e1033_d_n17, eq7_e1033_d_n18, eq7_e1033_d_n19, eq7_e1033_d_n20,) = {
    if (var_guard1924 == 0.0) {
        let eq7_e1027: f64 = (var_chnl_type * var_mult_inst);
        let eq7_e1029: f64 = (eq7_e1027 * p.p32);
        let eq7_e1031: f64 = (eq7_e1029 * var_i_gcd);
        let eq7_e1031_d_n5: f64 = (eq7_e1029 * var_i_gcd_dn5);
        let eq7_e1031_d_n6: f64 = (eq7_e1029 * var_i_gcd_dn6);
        let eq7_e1031_d_n7: f64 = (eq7_e1029 * var_i_gcd_dn7);
        let eq7_e1031_d_n8: f64 = (eq7_e1029 * var_i_gcd_dn8);
        let eq7_e1031_d_n12: f64 = (eq7_e1029 * var_i_gcd_dn12);
        let eq7_e1031_d_n13: f64 = (eq7_e1029 * var_i_gcd_dn13);
        let eq7_e1031_d_n14: f64 = (eq7_e1029 * var_i_gcd_dn14);
        let eq7_e1031_d_n15: f64 = (eq7_e1029 * var_i_gcd_dn15);
        let eq7_e1031_d_n16: f64 = (eq7_e1029 * var_i_gcd_dn16);
        let eq7_e1031_d_n17: f64 = (eq7_e1029 * var_i_gcd_dn17);
        let eq7_e1031_d_n18: f64 = (eq7_e1029 * var_i_gcd_dn18);
        let eq7_e1031_d_n19: f64 = (eq7_e1029 * var_i_gcd_dn19);
        let eq7_e1031_d_n20: f64 = (eq7_e1029 * var_i_gcd_dn20);
        (eq7_e1031, eq7_e1031_d_n5, eq7_e1031_d_n6, eq7_e1031_d_n7, eq7_e1031_d_n8, eq7_e1031_d_n12, eq7_e1031_d_n13, eq7_e1031_d_n14, eq7_e1031_d_n15, eq7_e1031_d_n16, eq7_e1031_d_n17, eq7_e1031_d_n18, eq7_e1031_d_n19, eq7_e1031_d_n20,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e1033;
        let eq7_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq7_node_derivatives: [f64; 13] = [eq7_e1033_d_n5, eq7_e1033_d_n6, eq7_e1033_d_n7, eq7_e1033_d_n8, eq7_e1033_d_n12, eq7_e1033_d_n13, eq7_e1033_d_n14, eq7_e1033_d_n15, eq7_e1033_d_n16, eq7_e1033_d_n17, eq7_e1033_d_n18, eq7_e1033_d_n19, eq7_e1033_d_n20];
        let eq7_branch_derivative_indices: [usize; 0] = [];
        let eq7_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq7_value),
            &eq7_node_derivative_indices,
            &eq7_node_derivatives,
            &eq7_branch_derivative_indices,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let eq8_e1036: f64 = (var_chnl_type * var_mult_inst);
        let eq8_e1038: f64 = (eq8_e1036 * p.p32);
        let eq8_e1040: f64 = (eq8_e1038 * var_i_gb);
        let eq8_e1040_d_n5: f64 = (eq8_e1038 * var_i_gb_dn5);
        let eq8_e1040_d_n6: f64 = (eq8_e1038 * var_i_gb_dn6);
        let eq8_e1040_d_n7: f64 = (eq8_e1038 * var_i_gb_dn7);
        let eq8_e1040_d_n8: f64 = (eq8_e1038 * var_i_gb_dn8);
        let eq8_e1040_d_n12: f64 = (eq8_e1038 * var_i_gb_dn12);
        let eq8_e1040_d_n13: f64 = (eq8_e1038 * var_i_gb_dn13);
        let eq8_e1040_d_n14: f64 = (eq8_e1038 * var_i_gb_dn14);
        let eq8_e1040_d_n15: f64 = (eq8_e1038 * var_i_gb_dn15);
        let eq8_e1040_d_n16: f64 = (eq8_e1038 * var_i_gb_dn16);
        let eq8_e1040_d_n17: f64 = (eq8_e1038 * var_i_gb_dn17);
        let eq8_e1040_d_n18: f64 = (eq8_e1038 * var_i_gb_dn18);
        let eq8_e1040_d_n19: f64 = (eq8_e1038 * var_i_gb_dn19);
        let eq8_e1040_d_n20: f64 = (eq8_e1038 * var_i_gb_dn20);
        let eq8_value: f64 = eq8_e1040;
        let eq8_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq8_node_derivatives: [f64; 13] = [eq8_e1040_d_n5, eq8_e1040_d_n6, eq8_e1040_d_n7, eq8_e1040_d_n8, eq8_e1040_d_n12, eq8_e1040_d_n13, eq8_e1040_d_n14, eq8_e1040_d_n15, eq8_e1040_d_n16, eq8_e1040_d_n17, eq8_e1040_d_n18, eq8_e1040_d_n19, eq8_e1040_d_n20];
        let eq8_branch_derivative_indices: [usize; 0] = [];
        let eq8_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(8),
            multiplicity * (eq8_value),
            &eq8_node_derivative_indices,
            &eq8_node_derivatives,
            &eq8_branch_derivative_indices,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let eq9_e1043: f64 = (var_chnl_type * var_mult_inst);
        let eq9_e1045: f64 = (eq9_e1043 * p.p32);
        let eq9_e1047: f64 = (eq9_e1045 * var_igsov);
        let eq9_e1047_d_n5: f64 = (eq9_e1045 * var_igsov_dn5);
        let eq9_e1047_d_n6: f64 = (eq9_e1045 * var_igsov_dn6);
        let eq9_e1047_d_n7: f64 = (eq9_e1045 * var_igsov_dn7);
        let eq9_e1047_d_n8: f64 = (eq9_e1045 * var_igsov_dn8);
        let eq9_e1047_d_n12: f64 = (eq9_e1045 * var_igsov_dn12);
        let eq9_e1047_d_n13: f64 = (eq9_e1045 * var_igsov_dn13);
        let eq9_e1047_d_n14: f64 = (eq9_e1045 * var_igsov_dn14);
        let eq9_e1047_d_n15: f64 = (eq9_e1045 * var_igsov_dn15);
        let eq9_e1047_d_n16: f64 = (eq9_e1045 * var_igsov_dn16);
        let eq9_e1047_d_n17: f64 = (eq9_e1045 * var_igsov_dn17);
        let eq9_e1047_d_n18: f64 = (eq9_e1045 * var_igsov_dn18);
        let eq9_e1047_d_n19: f64 = (eq9_e1045 * var_igsov_dn19);
        let eq9_e1047_d_n20: f64 = (eq9_e1045 * var_igsov_dn20);
        let eq9_value: f64 = eq9_e1047;
        let eq9_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq9_node_derivatives: [f64; 13] = [eq9_e1047_d_n5, eq9_e1047_d_n6, eq9_e1047_d_n7, eq9_e1047_d_n8, eq9_e1047_d_n12, eq9_e1047_d_n13, eq9_e1047_d_n14, eq9_e1047_d_n15, eq9_e1047_d_n16, eq9_e1047_d_n17, eq9_e1047_d_n18, eq9_e1047_d_n19, eq9_e1047_d_n20];
        let eq9_branch_derivative_indices: [usize; 0] = [];
        let eq9_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq9_value),
            &eq9_node_derivative_indices,
            &eq9_node_derivatives,
            &eq9_branch_derivative_indices,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let eq10_e1050: f64 = (var_chnl_type * var_mult_inst);
        let eq10_e1052: f64 = (eq10_e1050 * p.p32);
        let eq10_e1054: f64 = (eq10_e1052 * var_igdov);
        let eq10_e1054_d_n5: f64 = (eq10_e1052 * var_igdov_dn5);
        let eq10_e1054_d_n6: f64 = (eq10_e1052 * var_igdov_dn6);
        let eq10_e1054_d_n7: f64 = (eq10_e1052 * var_igdov_dn7);
        let eq10_e1054_d_n8: f64 = (eq10_e1052 * var_igdov_dn8);
        let eq10_e1054_d_n12: f64 = (eq10_e1052 * var_igdov_dn12);
        let eq10_e1054_d_n13: f64 = (eq10_e1052 * var_igdov_dn13);
        let eq10_e1054_d_n14: f64 = (eq10_e1052 * var_igdov_dn14);
        let eq10_e1054_d_n15: f64 = (eq10_e1052 * var_igdov_dn15);
        let eq10_e1054_d_n16: f64 = (eq10_e1052 * var_igdov_dn16);
        let eq10_e1054_d_n17: f64 = (eq10_e1052 * var_igdov_dn17);
        let eq10_e1054_d_n18: f64 = (eq10_e1052 * var_igdov_dn18);
        let eq10_e1054_d_n19: f64 = (eq10_e1052 * var_igdov_dn19);
        let eq10_e1054_d_n20: f64 = (eq10_e1052 * var_igdov_dn20);
        let eq10_value: f64 = eq10_e1054;
        let eq10_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq10_node_derivatives: [f64; 13] = [eq10_e1054_d_n5, eq10_e1054_d_n6, eq10_e1054_d_n7, eq10_e1054_d_n8, eq10_e1054_d_n12, eq10_e1054_d_n13, eq10_e1054_d_n14, eq10_e1054_d_n15, eq10_e1054_d_n16, eq10_e1054_d_n17, eq10_e1054_d_n18, eq10_e1054_d_n19, eq10_e1054_d_n20];
        let eq10_branch_derivative_indices: [usize; 0] = [];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq10_value),
            &eq10_node_derivative_indices,
            &eq10_node_derivatives,
            &eq10_branch_derivative_indices,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e1057: f64 = (var_chnl_type * var_mult_inst);
        let eq11_e1059: f64 = (eq11_e1057 * p.p32);
        let eq11_e1061: f64 = (eq11_e1059 * var_i_gisl);
        let eq11_e1061_d_n5: f64 = (eq11_e1059 * var_i_gisl_dn5);
        let eq11_e1061_d_n6: f64 = (eq11_e1059 * var_i_gisl_dn6);
        let eq11_e1061_d_n7: f64 = (eq11_e1059 * var_i_gisl_dn7);
        let eq11_e1061_d_n8: f64 = (eq11_e1059 * var_i_gisl_dn8);
        let eq11_e1061_d_n12: f64 = (eq11_e1059 * var_i_gisl_dn12);
        let eq11_e1061_d_n13: f64 = (eq11_e1059 * var_i_gisl_dn13);
        let eq11_e1061_d_n14: f64 = (eq11_e1059 * var_i_gisl_dn14);
        let eq11_e1061_d_n15: f64 = (eq11_e1059 * var_i_gisl_dn15);
        let eq11_e1061_d_n16: f64 = (eq11_e1059 * var_i_gisl_dn16);
        let eq11_e1061_d_n17: f64 = (eq11_e1059 * var_i_gisl_dn17);
        let eq11_e1061_d_n18: f64 = (eq11_e1059 * var_i_gisl_dn18);
        let eq11_e1061_d_n19: f64 = (eq11_e1059 * var_i_gisl_dn19);
        let eq11_e1061_d_n20: f64 = (eq11_e1059 * var_i_gisl_dn20);
        let eq11_value: f64 = eq11_e1061;
        let eq11_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq11_node_derivatives: [f64; 13] = [eq11_e1061_d_n5, eq11_e1061_d_n6, eq11_e1061_d_n7, eq11_e1061_d_n8, eq11_e1061_d_n12, eq11_e1061_d_n13, eq11_e1061_d_n14, eq11_e1061_d_n15, eq11_e1061_d_n16, eq11_e1061_d_n17, eq11_e1061_d_n18, eq11_e1061_d_n19, eq11_e1061_d_n20];
        let eq11_branch_derivative_indices: [usize; 0] = [];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq11_value),
            &eq11_node_derivative_indices,
            &eq11_node_derivatives,
            &eq11_branch_derivative_indices,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e1064: f64 = (var_chnl_type * var_mult_inst);
        let eq12_e1066: f64 = (eq12_e1064 * p.p32);
        let eq12_e1068: f64 = (eq12_e1066 * var_i_gidl);
        let eq12_e1068_d_n5: f64 = (eq12_e1066 * var_i_gidl_dn5);
        let eq12_e1068_d_n6: f64 = (eq12_e1066 * var_i_gidl_dn6);
        let eq12_e1068_d_n7: f64 = (eq12_e1066 * var_i_gidl_dn7);
        let eq12_e1068_d_n8: f64 = (eq12_e1066 * var_i_gidl_dn8);
        let eq12_e1068_d_n12: f64 = (eq12_e1066 * var_i_gidl_dn12);
        let eq12_e1068_d_n13: f64 = (eq12_e1066 * var_i_gidl_dn13);
        let eq12_e1068_d_n14: f64 = (eq12_e1066 * var_i_gidl_dn14);
        let eq12_e1068_d_n15: f64 = (eq12_e1066 * var_i_gidl_dn15);
        let eq12_e1068_d_n16: f64 = (eq12_e1066 * var_i_gidl_dn16);
        let eq12_e1068_d_n17: f64 = (eq12_e1066 * var_i_gidl_dn17);
        let eq12_e1068_d_n18: f64 = (eq12_e1066 * var_i_gidl_dn18);
        let eq12_e1068_d_n19: f64 = (eq12_e1066 * var_i_gidl_dn19);
        let eq12_e1068_d_n20: f64 = (eq12_e1066 * var_i_gidl_dn20);
        let eq12_value: f64 = eq12_e1068;
        let eq12_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq12_node_derivatives: [f64; 13] = [eq12_e1068_d_n5, eq12_e1068_d_n6, eq12_e1068_d_n7, eq12_e1068_d_n8, eq12_e1068_d_n12, eq12_e1068_d_n13, eq12_e1068_d_n14, eq12_e1068_d_n15, eq12_e1068_d_n16, eq12_e1068_d_n17, eq12_e1068_d_n18, eq12_e1068_d_n19, eq12_e1068_d_n20];
        let eq12_branch_derivative_indices: [usize; 0] = [];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq12_value),
            &eq12_node_derivative_indices,
            &eq12_node_derivatives,
            &eq12_branch_derivative_indices,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq13_e1071: f64 = (var_chnl_type * var_mult_inst);
        let eq13_e1073: f64 = (eq13_e1071 * p.p32);
        let eq13_e1075: f64 = (eq13_e1073 * var_ijun_s);
        let eq13_e1075_d_n5: f64 = (eq13_e1073 * var_ijun_s_dn5);
        let eq13_e1075_d_n6: f64 = (eq13_e1073 * var_ijun_s_dn6);
        let eq13_e1075_d_n7: f64 = (eq13_e1073 * var_ijun_s_dn7);
        let eq13_e1075_d_n8: f64 = (eq13_e1073 * var_ijun_s_dn8);
        let eq13_e1075_d_n10: f64 = (eq13_e1073 * var_ijun_s_dn10);
        let eq13_e1075_d_n11: f64 = (eq13_e1073 * var_ijun_s_dn11);
        let eq13_value: f64 = eq13_e1075;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(10),
            Some(6),
            multiplicity * (eq13_value),
            [5, 6, 7, 8, 10, 11],
            [multiplicity * (eq13_e1075_d_n5), multiplicity * (eq13_e1075_d_n6), multiplicity * (eq13_e1075_d_n7), multiplicity * (eq13_e1075_d_n8), multiplicity * (eq13_e1075_d_n10), multiplicity * (eq13_e1075_d_n11)],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        idt_scale: f64,
        idt_state_current: &mut [f64; Instance::IDT_STATE_COUNT],
        idt_state_previous: &mut [f64; Instance::IDT_STATE_COUNT],
        idt_state_initialized: &mut [bool; Instance::IDT_STATE_COUNT],
        var_chnl_type: f64,
        var_fk1: f64,
        var_fk1_dn12: f64,
        var_fk1_dn13: f64,
        var_fk1_dn14: f64,
        var_fk1_dn15: f64,
        var_fk1_dn16: f64,
        var_fk1_dn17: f64,
        var_fk1_dn18: f64,
        var_fk1_dn19: f64,
        var_fk1_dn20: f64,
        var_fk1_dn5: f64,
        var_fk1_dn6: f64,
        var_fk1_dn7: f64,
        var_fk1_dn8: f64,
        var_fk2: f64,
        var_fk2_dn12: f64,
        var_fk2_dn13: f64,
        var_fk2_dn14: f64,
        var_fk2_dn15: f64,
        var_fk2_dn16: f64,
        var_fk2_dn17: f64,
        var_fk2_dn18: f64,
        var_fk2_dn19: f64,
        var_fk2_dn20: f64,
        var_fk2_dn5: f64,
        var_fk2_dn6: f64,
        var_fk2_dn7: f64,
        var_fk2_dn8: f64,
        var_fk3: f64,
        var_fk3_dn12: f64,
        var_fk3_dn13: f64,
        var_fk3_dn14: f64,
        var_fk3_dn15: f64,
        var_fk3_dn16: f64,
        var_fk3_dn17: f64,
        var_fk3_dn18: f64,
        var_fk3_dn19: f64,
        var_fk3_dn20: f64,
        var_fk3_dn5: f64,
        var_fk3_dn6: f64,
        var_fk3_dn7: f64,
        var_fk3_dn8: f64,
        var_fk4: f64,
        var_fk4_dn12: f64,
        var_fk4_dn13: f64,
        var_fk4_dn14: f64,
        var_fk4_dn15: f64,
        var_fk4_dn16: f64,
        var_fk4_dn17: f64,
        var_fk4_dn18: f64,
        var_fk4_dn19: f64,
        var_fk4_dn20: f64,
        var_fk4_dn5: f64,
        var_fk4_dn6: f64,
        var_fk4_dn7: f64,
        var_fk4_dn8: f64,
        var_fk5: f64,
        var_fk5_dn12: f64,
        var_fk5_dn13: f64,
        var_fk5_dn14: f64,
        var_fk5_dn15: f64,
        var_fk5_dn16: f64,
        var_fk5_dn17: f64,
        var_fk5_dn18: f64,
        var_fk5_dn19: f64,
        var_fk5_dn20: f64,
        var_fk5_dn5: f64,
        var_fk5_dn6: f64,
        var_fk5_dn7: f64,
        var_fk5_dn8: f64,
        var_fk6: f64,
        var_fk6_dn12: f64,
        var_fk6_dn13: f64,
        var_fk6_dn14: f64,
        var_fk6_dn15: f64,
        var_fk6_dn16: f64,
        var_fk6_dn17: f64,
        var_fk6_dn18: f64,
        var_fk6_dn19: f64,
        var_fk6_dn20: f64,
        var_fk6_dn5: f64,
        var_fk6_dn6: f64,
        var_fk6_dn7: f64,
        var_fk6_dn8: f64,
        var_gbulk: f64,
        var_gdrain: f64,
        var_ggate: f64,
        var_gjund: f64,
        var_gjuns: f64,
        var_gsource: f64,
        var_guard1925: f64,
        var_guard1926: f64,
        var_guard1927: f64,
        var_guard1928: f64,
        var_guard1929: f64,
        var_guard1930: f64,
        var_guard1931: f64,
        var_gwell: f64,
        var_ijun_d: f64,
        var_ijun_d_dn10: f64,
        var_ijun_d_dn11: f64,
        var_ijun_d_dn5: f64,
        var_ijun_d_dn6: f64,
        var_ijun_d_dn7: f64,
        var_ijun_d_dn8: f64,
        var_mult_inst: f64,
        var_qp1_0: f64,
        var_qp2_0: f64,
        var_qp3_0: f64,
        var_qp4_0: f64,
        var_qp5_0: f64,
        var_qp6_0: f64,
        var_tnorm: f64,
        var_tnorm_dn12: f64,
        var_tnorm_dn13: f64,
        var_tnorm_dn14: f64,
        var_tnorm_dn15: f64,
        var_tnorm_dn16: f64,
        var_tnorm_dn17: f64,
        var_tnorm_dn18: f64,
        var_tnorm_dn19: f64,
        var_tnorm_dn20: f64,
        var_tnorm_dn5: f64,
        var_tnorm_dn6: f64,
        var_tnorm_dn7: f64,
        var_tnorm_dn8: f64,
        var_vnorm_inv: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let eq14_e1078: f64 = (var_chnl_type * var_mult_inst);
        let eq14_e1080: f64 = (eq14_e1078 * p.p32);
        let eq14_e1082: f64 = (eq14_e1080 * var_ijun_d);
        let eq14_e1082_d_n5: f64 = (eq14_e1080 * var_ijun_d_dn5);
        let eq14_e1082_d_n6: f64 = (eq14_e1080 * var_ijun_d_dn6);
        let eq14_e1082_d_n7: f64 = (eq14_e1080 * var_ijun_d_dn7);
        let eq14_e1082_d_n8: f64 = (eq14_e1080 * var_ijun_d_dn8);
        let eq14_e1082_d_n10: f64 = (eq14_e1080 * var_ijun_d_dn10);
        let eq14_e1082_d_n11: f64 = (eq14_e1080 * var_ijun_d_dn11);
        let eq14_value: f64 = eq14_e1082;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(7),
            multiplicity * (eq14_value),
            [5, 6, 7, 8, 10, 11],
            [multiplicity * (eq14_e1082_d_n5), multiplicity * (eq14_e1082_d_n6), multiplicity * (eq14_e1082_d_n7), multiplicity * (eq14_e1082_d_n8), multiplicity * (eq14_e1082_d_n10), multiplicity * (eq14_e1082_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq15_e1092, eq15_e1092_d_n1, eq15_e1092_d_n5,) = {
    if (var_guard1925 != 0.0) {
        let eq15_e1086: f64 = (var_mult_inst * p.p32);
        let eq15_e1088: f64 = (eq15_e1086 * var_ggate);
        let eq15_e1090: f64 = (eq15_e1088 * (nv1 - nv5));
        (eq15_e1090, eq15_e1088, (-eq15_e1088),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e1092;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(5),
            multiplicity * (eq15_value),
            1,
            multiplicity * (eq15_e1092_d_n1),
            5,
            multiplicity * (eq15_e1092_d_n5),
        );
        let (eq17_e1107,) = {
    if (var_guard1925 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq17_value: f64 = eq17_e1107;
        stamper.stamp_potential_const_local(
            0,
            eq17_value,
        );
        let (eq18_e1117, eq18_e1117_d_n2, eq18_e1117_d_n6,) = {
    if (var_guard1926 != 0.0) {
        let eq18_e1111: f64 = (var_mult_inst * p.p32);
        let eq18_e1113: f64 = (eq18_e1111 * var_gsource);
        let eq18_e1115: f64 = (eq18_e1113 * (nv2 - nv6));
        (eq18_e1115, eq18_e1113, (-eq18_e1113),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1117;
        stamper.stamp_current_node2_local(
            Some(2),
            Some(6),
            multiplicity * (eq18_value),
            2,
            multiplicity * (eq18_e1117_d_n2),
            6,
            multiplicity * (eq18_e1117_d_n6),
        );
        let (eq20_e1132,) = {
    if (var_guard1926 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e1132;
        stamper.stamp_potential_const_local(
            1,
            eq20_value,
        );
        let (eq21_e1142, eq21_e1142_d_n0, eq21_e1142_d_n7,) = {
    if (var_guard1927 != 0.0) {
        let eq21_e1136: f64 = (var_mult_inst * p.p32);
        let eq21_e1138: f64 = (eq21_e1136 * var_gdrain);
        let eq21_e1140: f64 = (eq21_e1138 * (nv0 - nv7));
        (eq21_e1140, eq21_e1138, (-eq21_e1138),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1142;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(7),
            multiplicity * (eq21_value),
            0,
            multiplicity * (eq21_e1142_d_n0),
            7,
            multiplicity * (eq21_e1142_d_n7),
        );
        let (eq23_e1157,) = {
    if (var_guard1927 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e1157;
        stamper.stamp_potential_const_local(
            2,
            eq23_value,
        );
        let (eq24_e1167, eq24_e1167_d_n8, eq24_e1167_d_n9,) = {
    if (var_guard1928 != 0.0) {
        let eq24_e1161: f64 = (var_mult_inst * p.p32);
        let eq24_e1163: f64 = (eq24_e1161 * var_gbulk);
        let eq24_e1165: f64 = (eq24_e1163 * (nv8 - nv9));
        (eq24_e1165, eq24_e1163, (-eq24_e1163),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1167;
        stamper.stamp_current_node2_local(
            Some(8),
            Some(9),
            multiplicity * (eq24_value),
            8,
            multiplicity * (eq24_e1167_d_n8),
            9,
            multiplicity * (eq24_e1167_d_n9),
        );
        let (eq26_e1182,) = {
    if (var_guard1928 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e1182;
        stamper.stamp_potential_const_local(
            3,
            eq26_value,
        );
        let (eq27_e1192, eq27_e1192_d_n9, eq27_e1192_d_n10,) = {
    if (var_guard1929 != 0.0) {
        let eq27_e1186: f64 = (var_mult_inst * p.p32);
        let eq27_e1188: f64 = (eq27_e1186 * var_gjuns);
        let eq27_e1190: f64 = (eq27_e1188 * (nv10 - nv9));
        (eq27_e1190, (-eq27_e1188), eq27_e1188,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1192;
        stamper.stamp_current_node2_local(
            Some(10),
            Some(9),
            multiplicity * (eq27_value),
            9,
            multiplicity * (eq27_e1192_d_n9),
            10,
            multiplicity * (eq27_e1192_d_n10),
        );
        let (eq29_e1207,) = {
    if (var_guard1929 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e1207;
        stamper.stamp_potential_const_local(
            4,
            eq29_value,
        );
        let (eq30_e1217, eq30_e1217_d_n9, eq30_e1217_d_n11,) = {
    if (var_guard1930 != 0.0) {
        let eq30_e1211: f64 = (var_mult_inst * p.p32);
        let eq30_e1213: f64 = (eq30_e1211 * var_gjund);
        let eq30_e1215: f64 = (eq30_e1213 * (nv11 - nv9));
        (eq30_e1215, (-eq30_e1213), eq30_e1213,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e1217;
        stamper.stamp_current_node2_local(
            Some(11),
            Some(9),
            multiplicity * (eq30_value),
            9,
            multiplicity * (eq30_e1217_d_n9),
            11,
            multiplicity * (eq30_e1217_d_n11),
        );
        let (eq32_e1232,) = {
    if (var_guard1930 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e1232;
        stamper.stamp_potential_const_local(
            5,
            eq32_value,
        );
        let (eq33_e1242, eq33_e1242_d_n3, eq33_e1242_d_n9,) = {
    if (var_guard1931 != 0.0) {
        let eq33_e1236: f64 = (var_mult_inst * p.p32);
        let eq33_e1238: f64 = (eq33_e1236 * var_gwell);
        let eq33_e1240: f64 = (eq33_e1238 * (nv3 - nv9));
        (eq33_e1240, eq33_e1238, (-eq33_e1238),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e1242;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(9),
            multiplicity * (eq33_value),
            3,
            multiplicity * (eq33_e1242_d_n3),
            9,
            multiplicity * (eq33_e1242_d_n9),
        );
        let (eq35_e1257,) = {
    if (var_guard1931 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e1257;
        stamper.stamp_potential_const_local(
            6,
            eq35_value,
        );
        let eq39_e1275: f64 = (-var_tnorm);
        let eq39_e1277: f64 = (eq39_e1275 * var_fk1);
        let eq39_e1277_d_n5: f64 = (((-var_tnorm_dn5) * var_fk1) + (eq39_e1275 * var_fk1_dn5));
        let eq39_e1277_d_n6: f64 = (((-var_tnorm_dn6) * var_fk1) + (eq39_e1275 * var_fk1_dn6));
        let eq39_e1277_d_n7: f64 = (((-var_tnorm_dn7) * var_fk1) + (eq39_e1275 * var_fk1_dn7));
        let eq39_e1277_d_n8: f64 = (((-var_tnorm_dn8) * var_fk1) + (eq39_e1275 * var_fk1_dn8));
        let eq39_e1277_d_n12: f64 = (((-var_tnorm_dn12) * var_fk1) + (eq39_e1275 * var_fk1_dn12));
        let eq39_e1277_d_n13: f64 = (((-var_tnorm_dn13) * var_fk1) + (eq39_e1275 * var_fk1_dn13));
        let eq39_e1277_d_n14: f64 = (((-var_tnorm_dn14) * var_fk1) + (eq39_e1275 * var_fk1_dn14));
        let eq39_e1277_d_n15: f64 = (((-var_tnorm_dn15) * var_fk1) + (eq39_e1275 * var_fk1_dn15));
        let eq39_e1277_d_n16: f64 = (((-var_tnorm_dn16) * var_fk1) + (eq39_e1275 * var_fk1_dn16));
        let eq39_e1277_d_n17: f64 = (((-var_tnorm_dn17) * var_fk1) + (eq39_e1275 * var_fk1_dn17));
        let eq39_e1277_d_n18: f64 = (((-var_tnorm_dn18) * var_fk1) + (eq39_e1275 * var_fk1_dn18));
        let eq39_e1277_d_n19: f64 = (((-var_tnorm_dn19) * var_fk1) + (eq39_e1275 * var_fk1_dn19));
        let eq39_e1277_d_n20: f64 = (((-var_tnorm_dn20) * var_fk1) + (eq39_e1275 * var_fk1_dn20));
        let eq39_e1279: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 0, eq39_e1277, var_qp1_0);
        let eq39_e1280: f64 = (var_vnorm_inv * eq39_e1279);
        let eq39_e1280_d_n5: f64 = (var_vnorm_inv * (eq39_e1277_d_n5 * idt_scale));
        let eq39_e1280_d_n6: f64 = (var_vnorm_inv * (eq39_e1277_d_n6 * idt_scale));
        let eq39_e1280_d_n7: f64 = (var_vnorm_inv * (eq39_e1277_d_n7 * idt_scale));
        let eq39_e1280_d_n8: f64 = (var_vnorm_inv * (eq39_e1277_d_n8 * idt_scale));
        let eq39_e1280_d_n12: f64 = (var_vnorm_inv * (eq39_e1277_d_n12 * idt_scale));
        let eq39_e1280_d_n13: f64 = (var_vnorm_inv * (eq39_e1277_d_n13 * idt_scale));
        let eq39_e1280_d_n14: f64 = (var_vnorm_inv * (eq39_e1277_d_n14 * idt_scale));
        let eq39_e1280_d_n15: f64 = (var_vnorm_inv * (eq39_e1277_d_n15 * idt_scale));
        let eq39_e1280_d_n16: f64 = (var_vnorm_inv * (eq39_e1277_d_n16 * idt_scale));
        let eq39_e1280_d_n17: f64 = (var_vnorm_inv * (eq39_e1277_d_n17 * idt_scale));
        let eq39_e1280_d_n18: f64 = (var_vnorm_inv * (eq39_e1277_d_n18 * idt_scale));
        let eq39_e1280_d_n19: f64 = (var_vnorm_inv * (eq39_e1277_d_n19 * idt_scale));
        let eq39_e1280_d_n20: f64 = (var_vnorm_inv * (eq39_e1277_d_n20 * idt_scale));
        let eq39_value: f64 = eq39_e1280;
        let eq39_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq39_node_derivatives: [f64; 13] = [eq39_e1280_d_n5, eq39_e1280_d_n6, eq39_e1280_d_n7, eq39_e1280_d_n8, eq39_e1280_d_n12, eq39_e1280_d_n13, eq39_e1280_d_n14, eq39_e1280_d_n15, eq39_e1280_d_n16, eq39_e1280_d_n17, eq39_e1280_d_n18, eq39_e1280_d_n19, eq39_e1280_d_n20];
        let eq39_branch_derivative_indices: [usize; 0] = [];
        let eq39_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            8,
            eq39_value,
            &eq39_node_derivative_indices,
            &eq39_node_derivatives,
            &eq39_branch_derivative_indices,
            &eq39_branch_derivatives,
        );
        let eq41_e1288: f64 = (-var_tnorm);
        let eq41_e1290: f64 = (eq41_e1288 * var_fk2);
        let eq41_e1290_d_n5: f64 = (((-var_tnorm_dn5) * var_fk2) + (eq41_e1288 * var_fk2_dn5));
        let eq41_e1290_d_n6: f64 = (((-var_tnorm_dn6) * var_fk2) + (eq41_e1288 * var_fk2_dn6));
        let eq41_e1290_d_n7: f64 = (((-var_tnorm_dn7) * var_fk2) + (eq41_e1288 * var_fk2_dn7));
        let eq41_e1290_d_n8: f64 = (((-var_tnorm_dn8) * var_fk2) + (eq41_e1288 * var_fk2_dn8));
        let eq41_e1290_d_n12: f64 = (((-var_tnorm_dn12) * var_fk2) + (eq41_e1288 * var_fk2_dn12));
        let eq41_e1290_d_n13: f64 = (((-var_tnorm_dn13) * var_fk2) + (eq41_e1288 * var_fk2_dn13));
        let eq41_e1290_d_n14: f64 = (((-var_tnorm_dn14) * var_fk2) + (eq41_e1288 * var_fk2_dn14));
        let eq41_e1290_d_n15: f64 = (((-var_tnorm_dn15) * var_fk2) + (eq41_e1288 * var_fk2_dn15));
        let eq41_e1290_d_n16: f64 = (((-var_tnorm_dn16) * var_fk2) + (eq41_e1288 * var_fk2_dn16));
        let eq41_e1290_d_n17: f64 = (((-var_tnorm_dn17) * var_fk2) + (eq41_e1288 * var_fk2_dn17));
        let eq41_e1290_d_n18: f64 = (((-var_tnorm_dn18) * var_fk2) + (eq41_e1288 * var_fk2_dn18));
        let eq41_e1290_d_n19: f64 = (((-var_tnorm_dn19) * var_fk2) + (eq41_e1288 * var_fk2_dn19));
        let eq41_e1290_d_n20: f64 = (((-var_tnorm_dn20) * var_fk2) + (eq41_e1288 * var_fk2_dn20));
        let eq41_e1292: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 1, eq41_e1290, var_qp2_0);
        let eq41_e1293: f64 = (var_vnorm_inv * eq41_e1292);
        let eq41_e1293_d_n5: f64 = (var_vnorm_inv * (eq41_e1290_d_n5 * idt_scale));
        let eq41_e1293_d_n6: f64 = (var_vnorm_inv * (eq41_e1290_d_n6 * idt_scale));
        let eq41_e1293_d_n7: f64 = (var_vnorm_inv * (eq41_e1290_d_n7 * idt_scale));
        let eq41_e1293_d_n8: f64 = (var_vnorm_inv * (eq41_e1290_d_n8 * idt_scale));
        let eq41_e1293_d_n12: f64 = (var_vnorm_inv * (eq41_e1290_d_n12 * idt_scale));
        let eq41_e1293_d_n13: f64 = (var_vnorm_inv * (eq41_e1290_d_n13 * idt_scale));
        let eq41_e1293_d_n14: f64 = (var_vnorm_inv * (eq41_e1290_d_n14 * idt_scale));
        let eq41_e1293_d_n15: f64 = (var_vnorm_inv * (eq41_e1290_d_n15 * idt_scale));
        let eq41_e1293_d_n16: f64 = (var_vnorm_inv * (eq41_e1290_d_n16 * idt_scale));
        let eq41_e1293_d_n17: f64 = (var_vnorm_inv * (eq41_e1290_d_n17 * idt_scale));
        let eq41_e1293_d_n18: f64 = (var_vnorm_inv * (eq41_e1290_d_n18 * idt_scale));
        let eq41_e1293_d_n19: f64 = (var_vnorm_inv * (eq41_e1290_d_n19 * idt_scale));
        let eq41_e1293_d_n20: f64 = (var_vnorm_inv * (eq41_e1290_d_n20 * idt_scale));
        let eq41_value: f64 = eq41_e1293;
        let eq41_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq41_node_derivatives: [f64; 13] = [eq41_e1293_d_n5, eq41_e1293_d_n6, eq41_e1293_d_n7, eq41_e1293_d_n8, eq41_e1293_d_n12, eq41_e1293_d_n13, eq41_e1293_d_n14, eq41_e1293_d_n15, eq41_e1293_d_n16, eq41_e1293_d_n17, eq41_e1293_d_n18, eq41_e1293_d_n19, eq41_e1293_d_n20];
        let eq41_branch_derivative_indices: [usize; 0] = [];
        let eq41_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            10,
            eq41_value,
            &eq41_node_derivative_indices,
            &eq41_node_derivatives,
            &eq41_branch_derivative_indices,
            &eq41_branch_derivatives,
        );
        let eq43_e1301: f64 = (-var_tnorm);
        let eq43_e1303: f64 = (eq43_e1301 * var_fk3);
        let eq43_e1303_d_n5: f64 = (((-var_tnorm_dn5) * var_fk3) + (eq43_e1301 * var_fk3_dn5));
        let eq43_e1303_d_n6: f64 = (((-var_tnorm_dn6) * var_fk3) + (eq43_e1301 * var_fk3_dn6));
        let eq43_e1303_d_n7: f64 = (((-var_tnorm_dn7) * var_fk3) + (eq43_e1301 * var_fk3_dn7));
        let eq43_e1303_d_n8: f64 = (((-var_tnorm_dn8) * var_fk3) + (eq43_e1301 * var_fk3_dn8));
        let eq43_e1303_d_n12: f64 = (((-var_tnorm_dn12) * var_fk3) + (eq43_e1301 * var_fk3_dn12));
        let eq43_e1303_d_n13: f64 = (((-var_tnorm_dn13) * var_fk3) + (eq43_e1301 * var_fk3_dn13));
        let eq43_e1303_d_n14: f64 = (((-var_tnorm_dn14) * var_fk3) + (eq43_e1301 * var_fk3_dn14));
        let eq43_e1303_d_n15: f64 = (((-var_tnorm_dn15) * var_fk3) + (eq43_e1301 * var_fk3_dn15));
        let eq43_e1303_d_n16: f64 = (((-var_tnorm_dn16) * var_fk3) + (eq43_e1301 * var_fk3_dn16));
        let eq43_e1303_d_n17: f64 = (((-var_tnorm_dn17) * var_fk3) + (eq43_e1301 * var_fk3_dn17));
        let eq43_e1303_d_n18: f64 = (((-var_tnorm_dn18) * var_fk3) + (eq43_e1301 * var_fk3_dn18));
        let eq43_e1303_d_n19: f64 = (((-var_tnorm_dn19) * var_fk3) + (eq43_e1301 * var_fk3_dn19));
        let eq43_e1303_d_n20: f64 = (((-var_tnorm_dn20) * var_fk3) + (eq43_e1301 * var_fk3_dn20));
        let eq43_e1305: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 2, eq43_e1303, var_qp3_0);
        let eq43_e1306: f64 = (var_vnorm_inv * eq43_e1305);
        let eq43_e1306_d_n5: f64 = (var_vnorm_inv * (eq43_e1303_d_n5 * idt_scale));
        let eq43_e1306_d_n6: f64 = (var_vnorm_inv * (eq43_e1303_d_n6 * idt_scale));
        let eq43_e1306_d_n7: f64 = (var_vnorm_inv * (eq43_e1303_d_n7 * idt_scale));
        let eq43_e1306_d_n8: f64 = (var_vnorm_inv * (eq43_e1303_d_n8 * idt_scale));
        let eq43_e1306_d_n12: f64 = (var_vnorm_inv * (eq43_e1303_d_n12 * idt_scale));
        let eq43_e1306_d_n13: f64 = (var_vnorm_inv * (eq43_e1303_d_n13 * idt_scale));
        let eq43_e1306_d_n14: f64 = (var_vnorm_inv * (eq43_e1303_d_n14 * idt_scale));
        let eq43_e1306_d_n15: f64 = (var_vnorm_inv * (eq43_e1303_d_n15 * idt_scale));
        let eq43_e1306_d_n16: f64 = (var_vnorm_inv * (eq43_e1303_d_n16 * idt_scale));
        let eq43_e1306_d_n17: f64 = (var_vnorm_inv * (eq43_e1303_d_n17 * idt_scale));
        let eq43_e1306_d_n18: f64 = (var_vnorm_inv * (eq43_e1303_d_n18 * idt_scale));
        let eq43_e1306_d_n19: f64 = (var_vnorm_inv * (eq43_e1303_d_n19 * idt_scale));
        let eq43_e1306_d_n20: f64 = (var_vnorm_inv * (eq43_e1303_d_n20 * idt_scale));
        let eq43_value: f64 = eq43_e1306;
        let eq43_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq43_node_derivatives: [f64; 13] = [eq43_e1306_d_n5, eq43_e1306_d_n6, eq43_e1306_d_n7, eq43_e1306_d_n8, eq43_e1306_d_n12, eq43_e1306_d_n13, eq43_e1306_d_n14, eq43_e1306_d_n15, eq43_e1306_d_n16, eq43_e1306_d_n17, eq43_e1306_d_n18, eq43_e1306_d_n19, eq43_e1306_d_n20];
        let eq43_branch_derivative_indices: [usize; 0] = [];
        let eq43_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            12,
            eq43_value,
            &eq43_node_derivative_indices,
            &eq43_node_derivatives,
            &eq43_branch_derivative_indices,
            &eq43_branch_derivatives,
        );
        let eq45_e1314: f64 = (-var_tnorm);
        let eq45_e1316: f64 = (eq45_e1314 * var_fk4);
        let eq45_e1316_d_n5: f64 = (((-var_tnorm_dn5) * var_fk4) + (eq45_e1314 * var_fk4_dn5));
        let eq45_e1316_d_n6: f64 = (((-var_tnorm_dn6) * var_fk4) + (eq45_e1314 * var_fk4_dn6));
        let eq45_e1316_d_n7: f64 = (((-var_tnorm_dn7) * var_fk4) + (eq45_e1314 * var_fk4_dn7));
        let eq45_e1316_d_n8: f64 = (((-var_tnorm_dn8) * var_fk4) + (eq45_e1314 * var_fk4_dn8));
        let eq45_e1316_d_n12: f64 = (((-var_tnorm_dn12) * var_fk4) + (eq45_e1314 * var_fk4_dn12));
        let eq45_e1316_d_n13: f64 = (((-var_tnorm_dn13) * var_fk4) + (eq45_e1314 * var_fk4_dn13));
        let eq45_e1316_d_n14: f64 = (((-var_tnorm_dn14) * var_fk4) + (eq45_e1314 * var_fk4_dn14));
        let eq45_e1316_d_n15: f64 = (((-var_tnorm_dn15) * var_fk4) + (eq45_e1314 * var_fk4_dn15));
        let eq45_e1316_d_n16: f64 = (((-var_tnorm_dn16) * var_fk4) + (eq45_e1314 * var_fk4_dn16));
        let eq45_e1316_d_n17: f64 = (((-var_tnorm_dn17) * var_fk4) + (eq45_e1314 * var_fk4_dn17));
        let eq45_e1316_d_n18: f64 = (((-var_tnorm_dn18) * var_fk4) + (eq45_e1314 * var_fk4_dn18));
        let eq45_e1316_d_n19: f64 = (((-var_tnorm_dn19) * var_fk4) + (eq45_e1314 * var_fk4_dn19));
        let eq45_e1316_d_n20: f64 = (((-var_tnorm_dn20) * var_fk4) + (eq45_e1314 * var_fk4_dn20));
        let eq45_e1318: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 3, eq45_e1316, var_qp4_0);
        let eq45_e1319: f64 = (var_vnorm_inv * eq45_e1318);
        let eq45_e1319_d_n5: f64 = (var_vnorm_inv * (eq45_e1316_d_n5 * idt_scale));
        let eq45_e1319_d_n6: f64 = (var_vnorm_inv * (eq45_e1316_d_n6 * idt_scale));
        let eq45_e1319_d_n7: f64 = (var_vnorm_inv * (eq45_e1316_d_n7 * idt_scale));
        let eq45_e1319_d_n8: f64 = (var_vnorm_inv * (eq45_e1316_d_n8 * idt_scale));
        let eq45_e1319_d_n12: f64 = (var_vnorm_inv * (eq45_e1316_d_n12 * idt_scale));
        let eq45_e1319_d_n13: f64 = (var_vnorm_inv * (eq45_e1316_d_n13 * idt_scale));
        let eq45_e1319_d_n14: f64 = (var_vnorm_inv * (eq45_e1316_d_n14 * idt_scale));
        let eq45_e1319_d_n15: f64 = (var_vnorm_inv * (eq45_e1316_d_n15 * idt_scale));
        let eq45_e1319_d_n16: f64 = (var_vnorm_inv * (eq45_e1316_d_n16 * idt_scale));
        let eq45_e1319_d_n17: f64 = (var_vnorm_inv * (eq45_e1316_d_n17 * idt_scale));
        let eq45_e1319_d_n18: f64 = (var_vnorm_inv * (eq45_e1316_d_n18 * idt_scale));
        let eq45_e1319_d_n19: f64 = (var_vnorm_inv * (eq45_e1316_d_n19 * idt_scale));
        let eq45_e1319_d_n20: f64 = (var_vnorm_inv * (eq45_e1316_d_n20 * idt_scale));
        let eq45_value: f64 = eq45_e1319;
        let eq45_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq45_node_derivatives: [f64; 13] = [eq45_e1319_d_n5, eq45_e1319_d_n6, eq45_e1319_d_n7, eq45_e1319_d_n8, eq45_e1319_d_n12, eq45_e1319_d_n13, eq45_e1319_d_n14, eq45_e1319_d_n15, eq45_e1319_d_n16, eq45_e1319_d_n17, eq45_e1319_d_n18, eq45_e1319_d_n19, eq45_e1319_d_n20];
        let eq45_branch_derivative_indices: [usize; 0] = [];
        let eq45_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            14,
            eq45_value,
            &eq45_node_derivative_indices,
            &eq45_node_derivatives,
            &eq45_branch_derivative_indices,
            &eq45_branch_derivatives,
        );
        let eq47_e1327: f64 = (-var_tnorm);
        let eq47_e1329: f64 = (eq47_e1327 * var_fk5);
        let eq47_e1329_d_n5: f64 = (((-var_tnorm_dn5) * var_fk5) + (eq47_e1327 * var_fk5_dn5));
        let eq47_e1329_d_n6: f64 = (((-var_tnorm_dn6) * var_fk5) + (eq47_e1327 * var_fk5_dn6));
        let eq47_e1329_d_n7: f64 = (((-var_tnorm_dn7) * var_fk5) + (eq47_e1327 * var_fk5_dn7));
        let eq47_e1329_d_n8: f64 = (((-var_tnorm_dn8) * var_fk5) + (eq47_e1327 * var_fk5_dn8));
        let eq47_e1329_d_n12: f64 = (((-var_tnorm_dn12) * var_fk5) + (eq47_e1327 * var_fk5_dn12));
        let eq47_e1329_d_n13: f64 = (((-var_tnorm_dn13) * var_fk5) + (eq47_e1327 * var_fk5_dn13));
        let eq47_e1329_d_n14: f64 = (((-var_tnorm_dn14) * var_fk5) + (eq47_e1327 * var_fk5_dn14));
        let eq47_e1329_d_n15: f64 = (((-var_tnorm_dn15) * var_fk5) + (eq47_e1327 * var_fk5_dn15));
        let eq47_e1329_d_n16: f64 = (((-var_tnorm_dn16) * var_fk5) + (eq47_e1327 * var_fk5_dn16));
        let eq47_e1329_d_n17: f64 = (((-var_tnorm_dn17) * var_fk5) + (eq47_e1327 * var_fk5_dn17));
        let eq47_e1329_d_n18: f64 = (((-var_tnorm_dn18) * var_fk5) + (eq47_e1327 * var_fk5_dn18));
        let eq47_e1329_d_n19: f64 = (((-var_tnorm_dn19) * var_fk5) + (eq47_e1327 * var_fk5_dn19));
        let eq47_e1329_d_n20: f64 = (((-var_tnorm_dn20) * var_fk5) + (eq47_e1327 * var_fk5_dn20));
        let eq47_e1331: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 4, eq47_e1329, var_qp5_0);
        let eq47_e1332: f64 = (var_vnorm_inv * eq47_e1331);
        let eq47_e1332_d_n5: f64 = (var_vnorm_inv * (eq47_e1329_d_n5 * idt_scale));
        let eq47_e1332_d_n6: f64 = (var_vnorm_inv * (eq47_e1329_d_n6 * idt_scale));
        let eq47_e1332_d_n7: f64 = (var_vnorm_inv * (eq47_e1329_d_n7 * idt_scale));
        let eq47_e1332_d_n8: f64 = (var_vnorm_inv * (eq47_e1329_d_n8 * idt_scale));
        let eq47_e1332_d_n12: f64 = (var_vnorm_inv * (eq47_e1329_d_n12 * idt_scale));
        let eq47_e1332_d_n13: f64 = (var_vnorm_inv * (eq47_e1329_d_n13 * idt_scale));
        let eq47_e1332_d_n14: f64 = (var_vnorm_inv * (eq47_e1329_d_n14 * idt_scale));
        let eq47_e1332_d_n15: f64 = (var_vnorm_inv * (eq47_e1329_d_n15 * idt_scale));
        let eq47_e1332_d_n16: f64 = (var_vnorm_inv * (eq47_e1329_d_n16 * idt_scale));
        let eq47_e1332_d_n17: f64 = (var_vnorm_inv * (eq47_e1329_d_n17 * idt_scale));
        let eq47_e1332_d_n18: f64 = (var_vnorm_inv * (eq47_e1329_d_n18 * idt_scale));
        let eq47_e1332_d_n19: f64 = (var_vnorm_inv * (eq47_e1329_d_n19 * idt_scale));
        let eq47_e1332_d_n20: f64 = (var_vnorm_inv * (eq47_e1329_d_n20 * idt_scale));
        let eq47_value: f64 = eq47_e1332;
        let eq47_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq47_node_derivatives: [f64; 13] = [eq47_e1332_d_n5, eq47_e1332_d_n6, eq47_e1332_d_n7, eq47_e1332_d_n8, eq47_e1332_d_n12, eq47_e1332_d_n13, eq47_e1332_d_n14, eq47_e1332_d_n15, eq47_e1332_d_n16, eq47_e1332_d_n17, eq47_e1332_d_n18, eq47_e1332_d_n19, eq47_e1332_d_n20];
        let eq47_branch_derivative_indices: [usize; 0] = [];
        let eq47_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            16,
            eq47_value,
            &eq47_node_derivative_indices,
            &eq47_node_derivatives,
            &eq47_branch_derivative_indices,
            &eq47_branch_derivatives,
        );
        let eq49_e1340: f64 = (-var_tnorm);
        let eq49_e1342: f64 = (eq49_e1340 * var_fk6);
        let eq49_e1342_d_n5: f64 = (((-var_tnorm_dn5) * var_fk6) + (eq49_e1340 * var_fk6_dn5));
        let eq49_e1342_d_n6: f64 = (((-var_tnorm_dn6) * var_fk6) + (eq49_e1340 * var_fk6_dn6));
        let eq49_e1342_d_n7: f64 = (((-var_tnorm_dn7) * var_fk6) + (eq49_e1340 * var_fk6_dn7));
        let eq49_e1342_d_n8: f64 = (((-var_tnorm_dn8) * var_fk6) + (eq49_e1340 * var_fk6_dn8));
        let eq49_e1342_d_n12: f64 = (((-var_tnorm_dn12) * var_fk6) + (eq49_e1340 * var_fk6_dn12));
        let eq49_e1342_d_n13: f64 = (((-var_tnorm_dn13) * var_fk6) + (eq49_e1340 * var_fk6_dn13));
        let eq49_e1342_d_n14: f64 = (((-var_tnorm_dn14) * var_fk6) + (eq49_e1340 * var_fk6_dn14));
        let eq49_e1342_d_n15: f64 = (((-var_tnorm_dn15) * var_fk6) + (eq49_e1340 * var_fk6_dn15));
        let eq49_e1342_d_n16: f64 = (((-var_tnorm_dn16) * var_fk6) + (eq49_e1340 * var_fk6_dn16));
        let eq49_e1342_d_n17: f64 = (((-var_tnorm_dn17) * var_fk6) + (eq49_e1340 * var_fk6_dn17));
        let eq49_e1342_d_n18: f64 = (((-var_tnorm_dn18) * var_fk6) + (eq49_e1340 * var_fk6_dn18));
        let eq49_e1342_d_n19: f64 = (((-var_tnorm_dn19) * var_fk6) + (eq49_e1340 * var_fk6_dn19));
        let eq49_e1342_d_n20: f64 = (((-var_tnorm_dn20) * var_fk6) + (eq49_e1340 * var_fk6_dn20));
        let eq49_e1344: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 5, eq49_e1342, var_qp6_0);
        let eq49_e1345: f64 = (var_vnorm_inv * eq49_e1344);
        let eq49_e1345_d_n5: f64 = (var_vnorm_inv * (eq49_e1342_d_n5 * idt_scale));
        let eq49_e1345_d_n6: f64 = (var_vnorm_inv * (eq49_e1342_d_n6 * idt_scale));
        let eq49_e1345_d_n7: f64 = (var_vnorm_inv * (eq49_e1342_d_n7 * idt_scale));
        let eq49_e1345_d_n8: f64 = (var_vnorm_inv * (eq49_e1342_d_n8 * idt_scale));
        let eq49_e1345_d_n12: f64 = (var_vnorm_inv * (eq49_e1342_d_n12 * idt_scale));
        let eq49_e1345_d_n13: f64 = (var_vnorm_inv * (eq49_e1342_d_n13 * idt_scale));
        let eq49_e1345_d_n14: f64 = (var_vnorm_inv * (eq49_e1342_d_n14 * idt_scale));
        let eq49_e1345_d_n15: f64 = (var_vnorm_inv * (eq49_e1342_d_n15 * idt_scale));
        let eq49_e1345_d_n16: f64 = (var_vnorm_inv * (eq49_e1342_d_n16 * idt_scale));
        let eq49_e1345_d_n17: f64 = (var_vnorm_inv * (eq49_e1342_d_n17 * idt_scale));
        let eq49_e1345_d_n18: f64 = (var_vnorm_inv * (eq49_e1342_d_n18 * idt_scale));
        let eq49_e1345_d_n19: f64 = (var_vnorm_inv * (eq49_e1342_d_n19 * idt_scale));
        let eq49_e1345_d_n20: f64 = (var_vnorm_inv * (eq49_e1342_d_n20 * idt_scale));
        let eq49_value: f64 = eq49_e1345;
        let eq49_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq49_node_derivatives: [f64; 13] = [eq49_e1345_d_n5, eq49_e1345_d_n6, eq49_e1345_d_n7, eq49_e1345_d_n8, eq49_e1345_d_n12, eq49_e1345_d_n13, eq49_e1345_d_n14, eq49_e1345_d_n15, eq49_e1345_d_n16, eq49_e1345_d_n17, eq49_e1345_d_n18, eq49_e1345_d_n19, eq49_e1345_d_n20];
        let eq49_branch_derivative_indices: [usize; 0] = [];
        let eq49_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            18,
            eq49_value,
            &eq49_node_derivative_indices,
            &eq49_node_derivatives,
            &eq49_branch_derivative_indices,
            &eq49_branch_derivatives,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        idt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        idt_state_current: &mut [f64; Instance::IDT_STATE_COUNT],
        idt_state_previous: &mut [f64; Instance::IDT_STATE_COUNT],
        idt_state_initialized: &mut [bool; Instance::IDT_STATE_COUNT],
        var_chnl_type: f64,
        var_fk7: f64,
        var_fk7_dn12: f64,
        var_fk7_dn13: f64,
        var_fk7_dn14: f64,
        var_fk7_dn15: f64,
        var_fk7_dn16: f64,
        var_fk7_dn17: f64,
        var_fk7_dn18: f64,
        var_fk7_dn19: f64,
        var_fk7_dn20: f64,
        var_fk7_dn5: f64,
        var_fk7_dn6: f64,
        var_fk7_dn7: f64,
        var_fk7_dn8: f64,
        var_fk8: f64,
        var_fk8_dn12: f64,
        var_fk8_dn13: f64,
        var_fk8_dn14: f64,
        var_fk8_dn15: f64,
        var_fk8_dn16: f64,
        var_fk8_dn17: f64,
        var_fk8_dn18: f64,
        var_fk8_dn19: f64,
        var_fk8_dn20: f64,
        var_fk8_dn5: f64,
        var_fk8_dn6: f64,
        var_fk8_dn7: f64,
        var_fk8_dn8: f64,
        var_fk9: f64,
        var_fk9_dn12: f64,
        var_fk9_dn13: f64,
        var_fk9_dn14: f64,
        var_fk9_dn15: f64,
        var_fk9_dn16: f64,
        var_fk9_dn17: f64,
        var_fk9_dn18: f64,
        var_fk9_dn19: f64,
        var_fk9_dn20: f64,
        var_fk9_dn5: f64,
        var_fk9_dn6: f64,
        var_fk9_dn7: f64,
        var_fk9_dn8: f64,
        var_mult_inst: f64,
        var_qb: f64,
        var_qb_dn12: f64,
        var_qb_dn13: f64,
        var_qb_dn14: f64,
        var_qb_dn15: f64,
        var_qb_dn16: f64,
        var_qb_dn17: f64,
        var_qb_dn18: f64,
        var_qb_dn19: f64,
        var_qb_dn20: f64,
        var_qb_dn5: f64,
        var_qb_dn6: f64,
        var_qb_dn7: f64,
        var_qb_dn8: f64,
        var_qd: f64,
        var_qd_dn12: f64,
        var_qd_dn13: f64,
        var_qd_dn14: f64,
        var_qd_dn15: f64,
        var_qd_dn16: f64,
        var_qd_dn17: f64,
        var_qd_dn18: f64,
        var_qd_dn19: f64,
        var_qd_dn20: f64,
        var_qd_dn5: f64,
        var_qd_dn6: f64,
        var_qd_dn7: f64,
        var_qd_dn8: f64,
        var_qfgd: f64,
        var_qfgd_dn5: f64,
        var_qfgd_dn6: f64,
        var_qfgd_dn7: f64,
        var_qfgs: f64,
        var_qfgs_dn5: f64,
        var_qfgs_dn6: f64,
        var_qfgs_dn7: f64,
        var_qg: f64,
        var_qg_dn12: f64,
        var_qg_dn13: f64,
        var_qg_dn14: f64,
        var_qg_dn15: f64,
        var_qg_dn16: f64,
        var_qg_dn17: f64,
        var_qg_dn18: f64,
        var_qg_dn19: f64,
        var_qg_dn20: f64,
        var_qg_dn5: f64,
        var_qg_dn6: f64,
        var_qg_dn7: f64,
        var_qg_dn8: f64,
        var_qgb_ov: f64,
        var_qgb_ov_dn12: f64,
        var_qgb_ov_dn13: f64,
        var_qgb_ov_dn14: f64,
        var_qgb_ov_dn15: f64,
        var_qgb_ov_dn16: f64,
        var_qgb_ov_dn17: f64,
        var_qgb_ov_dn18: f64,
        var_qgb_ov_dn19: f64,
        var_qgb_ov_dn20: f64,
        var_qgb_ov_dn5: f64,
        var_qgb_ov_dn6: f64,
        var_qgb_ov_dn7: f64,
        var_qgb_ov_dn8: f64,
        var_qjun_d: f64,
        var_qjun_d_dn10: f64,
        var_qjun_d_dn11: f64,
        var_qjun_d_dn5: f64,
        var_qjun_d_dn6: f64,
        var_qjun_d_dn7: f64,
        var_qjun_d_dn8: f64,
        var_qjun_s: f64,
        var_qjun_s_dn10: f64,
        var_qjun_s_dn11: f64,
        var_qjun_s_dn5: f64,
        var_qjun_s_dn6: f64,
        var_qjun_s_dn7: f64,
        var_qjun_s_dn8: f64,
        var_qp7_0: f64,
        var_qp8_0: f64,
        var_qp9_0: f64,
        var_tnorm: f64,
        var_tnorm_dn12: f64,
        var_tnorm_dn13: f64,
        var_tnorm_dn14: f64,
        var_tnorm_dn15: f64,
        var_tnorm_dn16: f64,
        var_tnorm_dn17: f64,
        var_tnorm_dn18: f64,
        var_tnorm_dn19: f64,
        var_tnorm_dn20: f64,
        var_tnorm_dn5: f64,
        var_tnorm_dn6: f64,
        var_tnorm_dn7: f64,
        var_tnorm_dn8: f64,
        var_vnorm_inv: f64,
    ) {
        let eq51_e1353: f64 = (-var_tnorm);
        let eq51_e1355: f64 = (eq51_e1353 * var_fk7);
        let eq51_e1355_d_n5: f64 = (((-var_tnorm_dn5) * var_fk7) + (eq51_e1353 * var_fk7_dn5));
        let eq51_e1355_d_n6: f64 = (((-var_tnorm_dn6) * var_fk7) + (eq51_e1353 * var_fk7_dn6));
        let eq51_e1355_d_n7: f64 = (((-var_tnorm_dn7) * var_fk7) + (eq51_e1353 * var_fk7_dn7));
        let eq51_e1355_d_n8: f64 = (((-var_tnorm_dn8) * var_fk7) + (eq51_e1353 * var_fk7_dn8));
        let eq51_e1355_d_n12: f64 = (((-var_tnorm_dn12) * var_fk7) + (eq51_e1353 * var_fk7_dn12));
        let eq51_e1355_d_n13: f64 = (((-var_tnorm_dn13) * var_fk7) + (eq51_e1353 * var_fk7_dn13));
        let eq51_e1355_d_n14: f64 = (((-var_tnorm_dn14) * var_fk7) + (eq51_e1353 * var_fk7_dn14));
        let eq51_e1355_d_n15: f64 = (((-var_tnorm_dn15) * var_fk7) + (eq51_e1353 * var_fk7_dn15));
        let eq51_e1355_d_n16: f64 = (((-var_tnorm_dn16) * var_fk7) + (eq51_e1353 * var_fk7_dn16));
        let eq51_e1355_d_n17: f64 = (((-var_tnorm_dn17) * var_fk7) + (eq51_e1353 * var_fk7_dn17));
        let eq51_e1355_d_n18: f64 = (((-var_tnorm_dn18) * var_fk7) + (eq51_e1353 * var_fk7_dn18));
        let eq51_e1355_d_n19: f64 = (((-var_tnorm_dn19) * var_fk7) + (eq51_e1353 * var_fk7_dn19));
        let eq51_e1355_d_n20: f64 = (((-var_tnorm_dn20) * var_fk7) + (eq51_e1353 * var_fk7_dn20));
        let eq51_e1357: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 6, eq51_e1355, var_qp7_0);
        let eq51_e1358: f64 = (var_vnorm_inv * eq51_e1357);
        let eq51_e1358_d_n5: f64 = (var_vnorm_inv * (eq51_e1355_d_n5 * idt_scale));
        let eq51_e1358_d_n6: f64 = (var_vnorm_inv * (eq51_e1355_d_n6 * idt_scale));
        let eq51_e1358_d_n7: f64 = (var_vnorm_inv * (eq51_e1355_d_n7 * idt_scale));
        let eq51_e1358_d_n8: f64 = (var_vnorm_inv * (eq51_e1355_d_n8 * idt_scale));
        let eq51_e1358_d_n12: f64 = (var_vnorm_inv * (eq51_e1355_d_n12 * idt_scale));
        let eq51_e1358_d_n13: f64 = (var_vnorm_inv * (eq51_e1355_d_n13 * idt_scale));
        let eq51_e1358_d_n14: f64 = (var_vnorm_inv * (eq51_e1355_d_n14 * idt_scale));
        let eq51_e1358_d_n15: f64 = (var_vnorm_inv * (eq51_e1355_d_n15 * idt_scale));
        let eq51_e1358_d_n16: f64 = (var_vnorm_inv * (eq51_e1355_d_n16 * idt_scale));
        let eq51_e1358_d_n17: f64 = (var_vnorm_inv * (eq51_e1355_d_n17 * idt_scale));
        let eq51_e1358_d_n18: f64 = (var_vnorm_inv * (eq51_e1355_d_n18 * idt_scale));
        let eq51_e1358_d_n19: f64 = (var_vnorm_inv * (eq51_e1355_d_n19 * idt_scale));
        let eq51_e1358_d_n20: f64 = (var_vnorm_inv * (eq51_e1355_d_n20 * idt_scale));
        let eq51_value: f64 = eq51_e1358;
        let eq51_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq51_node_derivatives: [f64; 13] = [eq51_e1358_d_n5, eq51_e1358_d_n6, eq51_e1358_d_n7, eq51_e1358_d_n8, eq51_e1358_d_n12, eq51_e1358_d_n13, eq51_e1358_d_n14, eq51_e1358_d_n15, eq51_e1358_d_n16, eq51_e1358_d_n17, eq51_e1358_d_n18, eq51_e1358_d_n19, eq51_e1358_d_n20];
        let eq51_branch_derivative_indices: [usize; 0] = [];
        let eq51_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            20,
            eq51_value,
            &eq51_node_derivative_indices,
            &eq51_node_derivatives,
            &eq51_branch_derivative_indices,
            &eq51_branch_derivatives,
        );
        let eq53_e1366: f64 = (-var_tnorm);
        let eq53_e1368: f64 = (eq53_e1366 * var_fk8);
        let eq53_e1368_d_n5: f64 = (((-var_tnorm_dn5) * var_fk8) + (eq53_e1366 * var_fk8_dn5));
        let eq53_e1368_d_n6: f64 = (((-var_tnorm_dn6) * var_fk8) + (eq53_e1366 * var_fk8_dn6));
        let eq53_e1368_d_n7: f64 = (((-var_tnorm_dn7) * var_fk8) + (eq53_e1366 * var_fk8_dn7));
        let eq53_e1368_d_n8: f64 = (((-var_tnorm_dn8) * var_fk8) + (eq53_e1366 * var_fk8_dn8));
        let eq53_e1368_d_n12: f64 = (((-var_tnorm_dn12) * var_fk8) + (eq53_e1366 * var_fk8_dn12));
        let eq53_e1368_d_n13: f64 = (((-var_tnorm_dn13) * var_fk8) + (eq53_e1366 * var_fk8_dn13));
        let eq53_e1368_d_n14: f64 = (((-var_tnorm_dn14) * var_fk8) + (eq53_e1366 * var_fk8_dn14));
        let eq53_e1368_d_n15: f64 = (((-var_tnorm_dn15) * var_fk8) + (eq53_e1366 * var_fk8_dn15));
        let eq53_e1368_d_n16: f64 = (((-var_tnorm_dn16) * var_fk8) + (eq53_e1366 * var_fk8_dn16));
        let eq53_e1368_d_n17: f64 = (((-var_tnorm_dn17) * var_fk8) + (eq53_e1366 * var_fk8_dn17));
        let eq53_e1368_d_n18: f64 = (((-var_tnorm_dn18) * var_fk8) + (eq53_e1366 * var_fk8_dn18));
        let eq53_e1368_d_n19: f64 = (((-var_tnorm_dn19) * var_fk8) + (eq53_e1366 * var_fk8_dn19));
        let eq53_e1368_d_n20: f64 = (((-var_tnorm_dn20) * var_fk8) + (eq53_e1366 * var_fk8_dn20));
        let eq53_e1370: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 7, eq53_e1368, var_qp8_0);
        let eq53_e1371: f64 = (var_vnorm_inv * eq53_e1370);
        let eq53_e1371_d_n5: f64 = (var_vnorm_inv * (eq53_e1368_d_n5 * idt_scale));
        let eq53_e1371_d_n6: f64 = (var_vnorm_inv * (eq53_e1368_d_n6 * idt_scale));
        let eq53_e1371_d_n7: f64 = (var_vnorm_inv * (eq53_e1368_d_n7 * idt_scale));
        let eq53_e1371_d_n8: f64 = (var_vnorm_inv * (eq53_e1368_d_n8 * idt_scale));
        let eq53_e1371_d_n12: f64 = (var_vnorm_inv * (eq53_e1368_d_n12 * idt_scale));
        let eq53_e1371_d_n13: f64 = (var_vnorm_inv * (eq53_e1368_d_n13 * idt_scale));
        let eq53_e1371_d_n14: f64 = (var_vnorm_inv * (eq53_e1368_d_n14 * idt_scale));
        let eq53_e1371_d_n15: f64 = (var_vnorm_inv * (eq53_e1368_d_n15 * idt_scale));
        let eq53_e1371_d_n16: f64 = (var_vnorm_inv * (eq53_e1368_d_n16 * idt_scale));
        let eq53_e1371_d_n17: f64 = (var_vnorm_inv * (eq53_e1368_d_n17 * idt_scale));
        let eq53_e1371_d_n18: f64 = (var_vnorm_inv * (eq53_e1368_d_n18 * idt_scale));
        let eq53_e1371_d_n19: f64 = (var_vnorm_inv * (eq53_e1368_d_n19 * idt_scale));
        let eq53_e1371_d_n20: f64 = (var_vnorm_inv * (eq53_e1368_d_n20 * idt_scale));
        let eq53_value: f64 = eq53_e1371;
        let eq53_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq53_node_derivatives: [f64; 13] = [eq53_e1371_d_n5, eq53_e1371_d_n6, eq53_e1371_d_n7, eq53_e1371_d_n8, eq53_e1371_d_n12, eq53_e1371_d_n13, eq53_e1371_d_n14, eq53_e1371_d_n15, eq53_e1371_d_n16, eq53_e1371_d_n17, eq53_e1371_d_n18, eq53_e1371_d_n19, eq53_e1371_d_n20];
        let eq53_branch_derivative_indices: [usize; 0] = [];
        let eq53_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            22,
            eq53_value,
            &eq53_node_derivative_indices,
            &eq53_node_derivatives,
            &eq53_branch_derivative_indices,
            &eq53_branch_derivatives,
        );
        let eq55_e1379: f64 = (-var_tnorm);
        let eq55_e1381: f64 = (eq55_e1379 * var_fk9);
        let eq55_e1381_d_n5: f64 = (((-var_tnorm_dn5) * var_fk9) + (eq55_e1379 * var_fk9_dn5));
        let eq55_e1381_d_n6: f64 = (((-var_tnorm_dn6) * var_fk9) + (eq55_e1379 * var_fk9_dn6));
        let eq55_e1381_d_n7: f64 = (((-var_tnorm_dn7) * var_fk9) + (eq55_e1379 * var_fk9_dn7));
        let eq55_e1381_d_n8: f64 = (((-var_tnorm_dn8) * var_fk9) + (eq55_e1379 * var_fk9_dn8));
        let eq55_e1381_d_n12: f64 = (((-var_tnorm_dn12) * var_fk9) + (eq55_e1379 * var_fk9_dn12));
        let eq55_e1381_d_n13: f64 = (((-var_tnorm_dn13) * var_fk9) + (eq55_e1379 * var_fk9_dn13));
        let eq55_e1381_d_n14: f64 = (((-var_tnorm_dn14) * var_fk9) + (eq55_e1379 * var_fk9_dn14));
        let eq55_e1381_d_n15: f64 = (((-var_tnorm_dn15) * var_fk9) + (eq55_e1379 * var_fk9_dn15));
        let eq55_e1381_d_n16: f64 = (((-var_tnorm_dn16) * var_fk9) + (eq55_e1379 * var_fk9_dn16));
        let eq55_e1381_d_n17: f64 = (((-var_tnorm_dn17) * var_fk9) + (eq55_e1379 * var_fk9_dn17));
        let eq55_e1381_d_n18: f64 = (((-var_tnorm_dn18) * var_fk9) + (eq55_e1379 * var_fk9_dn18));
        let eq55_e1381_d_n19: f64 = (((-var_tnorm_dn19) * var_fk9) + (eq55_e1379 * var_fk9_dn19));
        let eq55_e1381_d_n20: f64 = (((-var_tnorm_dn20) * var_fk9) + (eq55_e1379 * var_fk9_dn20));
        let eq55_e1383: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 8, eq55_e1381, var_qp9_0);
        let eq55_e1384: f64 = (var_vnorm_inv * eq55_e1383);
        let eq55_e1384_d_n5: f64 = (var_vnorm_inv * (eq55_e1381_d_n5 * idt_scale));
        let eq55_e1384_d_n6: f64 = (var_vnorm_inv * (eq55_e1381_d_n6 * idt_scale));
        let eq55_e1384_d_n7: f64 = (var_vnorm_inv * (eq55_e1381_d_n7 * idt_scale));
        let eq55_e1384_d_n8: f64 = (var_vnorm_inv * (eq55_e1381_d_n8 * idt_scale));
        let eq55_e1384_d_n12: f64 = (var_vnorm_inv * (eq55_e1381_d_n12 * idt_scale));
        let eq55_e1384_d_n13: f64 = (var_vnorm_inv * (eq55_e1381_d_n13 * idt_scale));
        let eq55_e1384_d_n14: f64 = (var_vnorm_inv * (eq55_e1381_d_n14 * idt_scale));
        let eq55_e1384_d_n15: f64 = (var_vnorm_inv * (eq55_e1381_d_n15 * idt_scale));
        let eq55_e1384_d_n16: f64 = (var_vnorm_inv * (eq55_e1381_d_n16 * idt_scale));
        let eq55_e1384_d_n17: f64 = (var_vnorm_inv * (eq55_e1381_d_n17 * idt_scale));
        let eq55_e1384_d_n18: f64 = (var_vnorm_inv * (eq55_e1381_d_n18 * idt_scale));
        let eq55_e1384_d_n19: f64 = (var_vnorm_inv * (eq55_e1381_d_n19 * idt_scale));
        let eq55_e1384_d_n20: f64 = (var_vnorm_inv * (eq55_e1381_d_n20 * idt_scale));
        let eq55_value: f64 = eq55_e1384;
        let eq55_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq55_node_derivatives: [f64; 13] = [eq55_e1384_d_n5, eq55_e1384_d_n6, eq55_e1384_d_n7, eq55_e1384_d_n8, eq55_e1384_d_n12, eq55_e1384_d_n13, eq55_e1384_d_n14, eq55_e1384_d_n15, eq55_e1384_d_n16, eq55_e1384_d_n17, eq55_e1384_d_n18, eq55_e1384_d_n19, eq55_e1384_d_n20];
        let eq55_branch_derivative_indices: [usize; 0] = [];
        let eq55_branch_derivatives: [f64; 0] = [];
        stamper.stamp_potential_indexed_dense_local(
            24,
            eq55_value,
            &eq55_node_derivative_indices,
            &eq55_node_derivatives,
            &eq55_branch_derivative_indices,
            &eq55_branch_derivatives,
        );
        let eq56_e1387: f64 = (var_chnl_type * var_mult_inst);
        let eq56_e1389: f64 = (eq56_e1387 * p.p33);
        let eq56_e1391: f64 = (eq56_e1389 * var_qg);
        let eq56_e1391_d_n5: f64 = (eq56_e1389 * var_qg_dn5);
        let eq56_e1391_d_n6: f64 = (eq56_e1389 * var_qg_dn6);
        let eq56_e1391_d_n7: f64 = (eq56_e1389 * var_qg_dn7);
        let eq56_e1391_d_n8: f64 = (eq56_e1389 * var_qg_dn8);
        let eq56_e1391_d_n12: f64 = (eq56_e1389 * var_qg_dn12);
        let eq56_e1391_d_n13: f64 = (eq56_e1389 * var_qg_dn13);
        let eq56_e1391_d_n14: f64 = (eq56_e1389 * var_qg_dn14);
        let eq56_e1391_d_n15: f64 = (eq56_e1389 * var_qg_dn15);
        let eq56_e1391_d_n16: f64 = (eq56_e1389 * var_qg_dn16);
        let eq56_e1391_d_n17: f64 = (eq56_e1389 * var_qg_dn17);
        let eq56_e1391_d_n18: f64 = (eq56_e1389 * var_qg_dn18);
        let eq56_e1391_d_n19: f64 = (eq56_e1389 * var_qg_dn19);
        let eq56_e1391_d_n20: f64 = (eq56_e1389 * var_qg_dn20);
        let eq56_e1392: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq56_e1391);
        let eq56_value: f64 = eq56_e1392;
        let eq56_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq56_node_derivatives: [f64; 13] = [(eq56_e1391_d_n5 * ddt_scale), (eq56_e1391_d_n6 * ddt_scale), (eq56_e1391_d_n7 * ddt_scale), (eq56_e1391_d_n8 * ddt_scale), (eq56_e1391_d_n12 * ddt_scale), (eq56_e1391_d_n13 * ddt_scale), (eq56_e1391_d_n14 * ddt_scale), (eq56_e1391_d_n15 * ddt_scale), (eq56_e1391_d_n16 * ddt_scale), (eq56_e1391_d_n17 * ddt_scale), (eq56_e1391_d_n18 * ddt_scale), (eq56_e1391_d_n19 * ddt_scale), (eq56_e1391_d_n20 * ddt_scale)];
        let eq56_branch_derivative_indices: [usize; 0] = [];
        let eq56_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq56_value),
            &eq56_node_derivative_indices,
            &eq56_node_derivatives,
            &eq56_branch_derivative_indices,
            &eq56_branch_derivatives,
            multiplicity,
        );
        let eq57_e1395: f64 = (var_chnl_type * var_mult_inst);
        let eq57_e1397: f64 = (eq57_e1395 * p.p33);
        let eq57_e1399: f64 = (eq57_e1397 * var_qb);
        let eq57_e1399_d_n5: f64 = (eq57_e1397 * var_qb_dn5);
        let eq57_e1399_d_n6: f64 = (eq57_e1397 * var_qb_dn6);
        let eq57_e1399_d_n7: f64 = (eq57_e1397 * var_qb_dn7);
        let eq57_e1399_d_n8: f64 = (eq57_e1397 * var_qb_dn8);
        let eq57_e1399_d_n12: f64 = (eq57_e1397 * var_qb_dn12);
        let eq57_e1399_d_n13: f64 = (eq57_e1397 * var_qb_dn13);
        let eq57_e1399_d_n14: f64 = (eq57_e1397 * var_qb_dn14);
        let eq57_e1399_d_n15: f64 = (eq57_e1397 * var_qb_dn15);
        let eq57_e1399_d_n16: f64 = (eq57_e1397 * var_qb_dn16);
        let eq57_e1399_d_n17: f64 = (eq57_e1397 * var_qb_dn17);
        let eq57_e1399_d_n18: f64 = (eq57_e1397 * var_qb_dn18);
        let eq57_e1399_d_n19: f64 = (eq57_e1397 * var_qb_dn19);
        let eq57_e1399_d_n20: f64 = (eq57_e1397 * var_qb_dn20);
        let eq57_e1400: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq57_e1399);
        let eq57_value: f64 = eq57_e1400;
        let eq57_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq57_node_derivatives: [f64; 13] = [(eq57_e1399_d_n5 * ddt_scale), (eq57_e1399_d_n6 * ddt_scale), (eq57_e1399_d_n7 * ddt_scale), (eq57_e1399_d_n8 * ddt_scale), (eq57_e1399_d_n12 * ddt_scale), (eq57_e1399_d_n13 * ddt_scale), (eq57_e1399_d_n14 * ddt_scale), (eq57_e1399_d_n15 * ddt_scale), (eq57_e1399_d_n16 * ddt_scale), (eq57_e1399_d_n17 * ddt_scale), (eq57_e1399_d_n18 * ddt_scale), (eq57_e1399_d_n19 * ddt_scale), (eq57_e1399_d_n20 * ddt_scale)];
        let eq57_branch_derivative_indices: [usize; 0] = [];
        let eq57_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq57_value),
            &eq57_node_derivative_indices,
            &eq57_node_derivatives,
            &eq57_branch_derivative_indices,
            &eq57_branch_derivatives,
            multiplicity,
        );
        let eq58_e1403: f64 = (var_chnl_type * var_mult_inst);
        let eq58_e1405: f64 = (eq58_e1403 * p.p33);
        let eq58_e1407: f64 = (eq58_e1405 * var_qd);
        let eq58_e1407_d_n5: f64 = (eq58_e1405 * var_qd_dn5);
        let eq58_e1407_d_n6: f64 = (eq58_e1405 * var_qd_dn6);
        let eq58_e1407_d_n7: f64 = (eq58_e1405 * var_qd_dn7);
        let eq58_e1407_d_n8: f64 = (eq58_e1405 * var_qd_dn8);
        let eq58_e1407_d_n12: f64 = (eq58_e1405 * var_qd_dn12);
        let eq58_e1407_d_n13: f64 = (eq58_e1405 * var_qd_dn13);
        let eq58_e1407_d_n14: f64 = (eq58_e1405 * var_qd_dn14);
        let eq58_e1407_d_n15: f64 = (eq58_e1405 * var_qd_dn15);
        let eq58_e1407_d_n16: f64 = (eq58_e1405 * var_qd_dn16);
        let eq58_e1407_d_n17: f64 = (eq58_e1405 * var_qd_dn17);
        let eq58_e1407_d_n18: f64 = (eq58_e1405 * var_qd_dn18);
        let eq58_e1407_d_n19: f64 = (eq58_e1405 * var_qd_dn19);
        let eq58_e1407_d_n20: f64 = (eq58_e1405 * var_qd_dn20);
        let eq58_e1408: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq58_e1407);
        let eq58_value: f64 = eq58_e1408;
        let eq58_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq58_node_derivatives: [f64; 13] = [(eq58_e1407_d_n5 * ddt_scale), (eq58_e1407_d_n6 * ddt_scale), (eq58_e1407_d_n7 * ddt_scale), (eq58_e1407_d_n8 * ddt_scale), (eq58_e1407_d_n12 * ddt_scale), (eq58_e1407_d_n13 * ddt_scale), (eq58_e1407_d_n14 * ddt_scale), (eq58_e1407_d_n15 * ddt_scale), (eq58_e1407_d_n16 * ddt_scale), (eq58_e1407_d_n17 * ddt_scale), (eq58_e1407_d_n18 * ddt_scale), (eq58_e1407_d_n19 * ddt_scale), (eq58_e1407_d_n20 * ddt_scale)];
        let eq58_branch_derivative_indices: [usize; 0] = [];
        let eq58_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq58_value),
            &eq58_node_derivative_indices,
            &eq58_node_derivatives,
            &eq58_branch_derivative_indices,
            &eq58_branch_derivatives,
            multiplicity,
        );
        let eq59_e1411: f64 = (var_chnl_type * var_mult_inst);
        let eq59_e1413: f64 = (eq59_e1411 * p.p33);
        let eq59_e1415: f64 = (eq59_e1413 * var_qfgs);
        let eq59_e1415_d_n5: f64 = (eq59_e1413 * var_qfgs_dn5);
        let eq59_e1415_d_n6: f64 = (eq59_e1413 * var_qfgs_dn6);
        let eq59_e1415_d_n7: f64 = (eq59_e1413 * var_qfgs_dn7);
        let eq59_e1416: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq59_e1415);
        let eq59_value: f64 = eq59_e1416;
        stamper.stamp_current_node3_local(
            Some(5),
            Some(6),
            multiplicity * (eq59_value),
            5,
            multiplicity * ((eq59_e1415_d_n5 * ddt_scale)),
            6,
            multiplicity * ((eq59_e1415_d_n6 * ddt_scale)),
            7,
            multiplicity * ((eq59_e1415_d_n7 * ddt_scale)),
        );
        let eq60_e1419: f64 = (var_chnl_type * var_mult_inst);
        let eq60_e1421: f64 = (eq60_e1419 * p.p33);
        let eq60_e1423: f64 = (eq60_e1421 * var_qfgd);
        let eq60_e1423_d_n5: f64 = (eq60_e1421 * var_qfgd_dn5);
        let eq60_e1423_d_n6: f64 = (eq60_e1421 * var_qfgd_dn6);
        let eq60_e1423_d_n7: f64 = (eq60_e1421 * var_qfgd_dn7);
        let eq60_e1424: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq60_e1423);
        let eq60_value: f64 = eq60_e1424;
        stamper.stamp_current_node3_local(
            Some(5),
            Some(7),
            multiplicity * (eq60_value),
            5,
            multiplicity * ((eq60_e1423_d_n5 * ddt_scale)),
            6,
            multiplicity * ((eq60_e1423_d_n6 * ddt_scale)),
            7,
            multiplicity * ((eq60_e1423_d_n7 * ddt_scale)),
        );
        let eq61_e1427: f64 = (var_chnl_type * var_mult_inst);
        let eq61_e1429: f64 = (eq61_e1427 * p.p33);
        let eq61_e1431: f64 = (eq61_e1429 * var_qgb_ov);
        let eq61_e1431_d_n5: f64 = (eq61_e1429 * var_qgb_ov_dn5);
        let eq61_e1431_d_n6: f64 = (eq61_e1429 * var_qgb_ov_dn6);
        let eq61_e1431_d_n7: f64 = (eq61_e1429 * var_qgb_ov_dn7);
        let eq61_e1431_d_n8: f64 = (eq61_e1429 * var_qgb_ov_dn8);
        let eq61_e1431_d_n12: f64 = (eq61_e1429 * var_qgb_ov_dn12);
        let eq61_e1431_d_n13: f64 = (eq61_e1429 * var_qgb_ov_dn13);
        let eq61_e1431_d_n14: f64 = (eq61_e1429 * var_qgb_ov_dn14);
        let eq61_e1431_d_n15: f64 = (eq61_e1429 * var_qgb_ov_dn15);
        let eq61_e1431_d_n16: f64 = (eq61_e1429 * var_qgb_ov_dn16);
        let eq61_e1431_d_n17: f64 = (eq61_e1429 * var_qgb_ov_dn17);
        let eq61_e1431_d_n18: f64 = (eq61_e1429 * var_qgb_ov_dn18);
        let eq61_e1431_d_n19: f64 = (eq61_e1429 * var_qgb_ov_dn19);
        let eq61_e1431_d_n20: f64 = (eq61_e1429 * var_qgb_ov_dn20);
        let eq61_e1432: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq61_e1431);
        let eq61_value: f64 = eq61_e1432;
        let eq61_node_derivative_indices: [usize; 13] = [5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq61_node_derivatives: [f64; 13] = [(eq61_e1431_d_n5 * ddt_scale), (eq61_e1431_d_n6 * ddt_scale), (eq61_e1431_d_n7 * ddt_scale), (eq61_e1431_d_n8 * ddt_scale), (eq61_e1431_d_n12 * ddt_scale), (eq61_e1431_d_n13 * ddt_scale), (eq61_e1431_d_n14 * ddt_scale), (eq61_e1431_d_n15 * ddt_scale), (eq61_e1431_d_n16 * ddt_scale), (eq61_e1431_d_n17 * ddt_scale), (eq61_e1431_d_n18 * ddt_scale), (eq61_e1431_d_n19 * ddt_scale), (eq61_e1431_d_n20 * ddt_scale)];
        let eq61_branch_derivative_indices: [usize; 0] = [];
        let eq61_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(8),
            multiplicity * (eq61_value),
            &eq61_node_derivative_indices,
            &eq61_node_derivatives,
            &eq61_branch_derivative_indices,
            &eq61_branch_derivatives,
            multiplicity,
        );
        let eq62_e1435: f64 = (var_chnl_type * var_mult_inst);
        let eq62_e1437: f64 = (eq62_e1435 * p.p33);
        let eq62_e1439: f64 = (eq62_e1437 * var_qjun_s);
        let eq62_e1439_d_n5: f64 = (eq62_e1437 * var_qjun_s_dn5);
        let eq62_e1439_d_n6: f64 = (eq62_e1437 * var_qjun_s_dn6);
        let eq62_e1439_d_n7: f64 = (eq62_e1437 * var_qjun_s_dn7);
        let eq62_e1439_d_n8: f64 = (eq62_e1437 * var_qjun_s_dn8);
        let eq62_e1439_d_n10: f64 = (eq62_e1437 * var_qjun_s_dn10);
        let eq62_e1439_d_n11: f64 = (eq62_e1437 * var_qjun_s_dn11);
        let eq62_e1440: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq62_e1439);
        let eq62_value: f64 = eq62_e1440;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(10),
            Some(6),
            multiplicity * (eq62_value),
            [5, 6, 7, 8, 10, 11],
            [multiplicity * ((eq62_e1439_d_n5 * ddt_scale)), multiplicity * ((eq62_e1439_d_n6 * ddt_scale)), multiplicity * ((eq62_e1439_d_n7 * ddt_scale)), multiplicity * ((eq62_e1439_d_n8 * ddt_scale)), multiplicity * ((eq62_e1439_d_n10 * ddt_scale)), multiplicity * ((eq62_e1439_d_n11 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq63_e1443: f64 = (var_chnl_type * var_mult_inst);
        let eq63_e1445: f64 = (eq63_e1443 * p.p33);
        let eq63_e1447: f64 = (eq63_e1445 * var_qjun_d);
        let eq63_e1447_d_n5: f64 = (eq63_e1445 * var_qjun_d_dn5);
        let eq63_e1447_d_n6: f64 = (eq63_e1445 * var_qjun_d_dn6);
        let eq63_e1447_d_n7: f64 = (eq63_e1445 * var_qjun_d_dn7);
        let eq63_e1447_d_n8: f64 = (eq63_e1445 * var_qjun_d_dn8);
        let eq63_e1447_d_n10: f64 = (eq63_e1445 * var_qjun_d_dn10);
        let eq63_e1447_d_n11: f64 = (eq63_e1445 * var_qjun_d_dn11);
        let eq63_e1448: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq63_e1447);
        let eq63_value: f64 = eq63_e1448;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(7),
            multiplicity * (eq63_value),
            [5, 6, 7, 8, 10, 11],
            [multiplicity * ((eq63_e1447_d_n5 * ddt_scale)), multiplicity * ((eq63_e1447_d_n6 * ddt_scale)), multiplicity * ((eq63_e1447_d_n7 * ddt_scale)), multiplicity * ((eq63_e1447_d_n8 * ddt_scale)), multiplicity * ((eq63_e1447_d_n10 * ddt_scale)), multiplicity * ((eq63_e1447_d_n11 * ddt_scale))],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_previous_value_scale: f64,
        ddt_older_value_scale: f64,
        ddt_previous_derivative_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_older: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
        ddt_derivative_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_derivative_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        var_cgeff: f64,
        var_cgeff_dn12: f64,
        var_cgeff_dn13: f64,
        var_cgeff_dn14: f64,
        var_cgeff_dn15: f64,
        var_cgeff_dn16: f64,
        var_cgeff_dn17: f64,
        var_cgeff_dn18: f64,
        var_cgeff_dn19: f64,
        var_cgeff_dn20: f64,
        var_cgeff_dn5: f64,
        var_cgeff_dn6: f64,
        var_cgeff_dn7: f64,
        var_cgeff_dn8: f64,
        var_mig: f64,
        var_mig_dn12: f64,
        var_mig_dn13: f64,
        var_mig_dn14: f64,
        var_mig_dn15: f64,
        var_mig_dn16: f64,
        var_mig_dn17: f64,
        var_mig_dn18: f64,
        var_mig_dn19: f64,
        var_mig_dn20: f64,
        var_mig_dn5: f64,
        var_mig_dn6: f64,
        var_mig_dn7: f64,
        var_mig_dn8: f64,
        var_mult_inst: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let __rspice_inv_cse_0: f64 = 1.0 / var_mig;
        let eq65_e1456: f64 = ((nv4 - 0.0) * __rspice_inv_cse_0);
        let eq65_e1456_d_n4: f64 = (1.0 * __rspice_inv_cse_0);
        let eq65_e1456_d_n5: f64 = (-(((nv4 - 0.0) * var_mig_dn5) / (var_mig * var_mig)));
        let eq65_e1456_d_n6: f64 = (-(((nv4 - 0.0) * var_mig_dn6) / (var_mig * var_mig)));
        let eq65_e1456_d_n7: f64 = (-(((nv4 - 0.0) * var_mig_dn7) / (var_mig * var_mig)));
        let eq65_e1456_d_n8: f64 = (-(((nv4 - 0.0) * var_mig_dn8) / (var_mig * var_mig)));
        let eq65_e1456_d_n12: f64 = (-(((nv4 - 0.0) * var_mig_dn12) / (var_mig * var_mig)));
        let eq65_e1456_d_n13: f64 = (-(((nv4 - 0.0) * var_mig_dn13) / (var_mig * var_mig)));
        let eq65_e1456_d_n14: f64 = (-(((nv4 - 0.0) * var_mig_dn14) / (var_mig * var_mig)));
        let eq65_e1456_d_n15: f64 = (-(((nv4 - 0.0) * var_mig_dn15) / (var_mig * var_mig)));
        let eq65_e1456_d_n16: f64 = (-(((nv4 - 0.0) * var_mig_dn16) / (var_mig * var_mig)));
        let eq65_e1456_d_n17: f64 = (-(((nv4 - 0.0) * var_mig_dn17) / (var_mig * var_mig)));
        let eq65_e1456_d_n18: f64 = (-(((nv4 - 0.0) * var_mig_dn18) / (var_mig * var_mig)));
        let eq65_e1456_d_n19: f64 = (-(((nv4 - 0.0) * var_mig_dn19) / (var_mig * var_mig)));
        let eq65_e1456_d_n20: f64 = (-(((nv4 - 0.0) * var_mig_dn20) / (var_mig * var_mig)));
        let eq65_value: f64 = eq65_e1456;
        let eq65_node_derivative_indices: [usize; 14] = [4, 5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq65_node_derivatives: [f64; 14] = [eq65_e1456_d_n4, eq65_e1456_d_n5, eq65_e1456_d_n6, eq65_e1456_d_n7, eq65_e1456_d_n8, eq65_e1456_d_n12, eq65_e1456_d_n13, eq65_e1456_d_n14, eq65_e1456_d_n15, eq65_e1456_d_n16, eq65_e1456_d_n17, eq65_e1456_d_n18, eq65_e1456_d_n19, eq65_e1456_d_n20];
        let eq65_branch_derivative_indices: [usize; 0] = [];
        let eq65_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq65_value),
            &eq65_node_derivative_indices,
            &eq65_node_derivatives,
            &eq65_branch_derivative_indices,
            &eq65_branch_derivatives,
            multiplicity,
        );
        let eq66_e1459: f64 = (var_cgeff * (nv4 - 0.0));
        let eq66_e1459_d_n5: f64 = (var_cgeff_dn5 * (nv4 - 0.0));
        let eq66_e1459_d_n6: f64 = (var_cgeff_dn6 * (nv4 - 0.0));
        let eq66_e1459_d_n7: f64 = (var_cgeff_dn7 * (nv4 - 0.0));
        let eq66_e1459_d_n8: f64 = (var_cgeff_dn8 * (nv4 - 0.0));
        let eq66_e1459_d_n12: f64 = (var_cgeff_dn12 * (nv4 - 0.0));
        let eq66_e1459_d_n13: f64 = (var_cgeff_dn13 * (nv4 - 0.0));
        let eq66_e1459_d_n14: f64 = (var_cgeff_dn14 * (nv4 - 0.0));
        let eq66_e1459_d_n15: f64 = (var_cgeff_dn15 * (nv4 - 0.0));
        let eq66_e1459_d_n16: f64 = (var_cgeff_dn16 * (nv4 - 0.0));
        let eq66_e1459_d_n17: f64 = (var_cgeff_dn17 * (nv4 - 0.0));
        let eq66_e1459_d_n18: f64 = (var_cgeff_dn18 * (nv4 - 0.0));
        let eq66_e1459_d_n19: f64 = (var_cgeff_dn19 * (nv4 - 0.0));
        let eq66_e1459_d_n20: f64 = (var_cgeff_dn20 * (nv4 - 0.0));
        let eq66_e1460: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq66_e1459);
        let eq66_value: f64 = eq66_e1460;
        let eq66_node_derivative_indices: [usize; 14] = [4, 5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq66_node_derivatives: [f64; 14] = [(var_cgeff * ddt_scale), (eq66_e1459_d_n5 * ddt_scale), (eq66_e1459_d_n6 * ddt_scale), (eq66_e1459_d_n7 * ddt_scale), (eq66_e1459_d_n8 * ddt_scale), (eq66_e1459_d_n12 * ddt_scale), (eq66_e1459_d_n13 * ddt_scale), (eq66_e1459_d_n14 * ddt_scale), (eq66_e1459_d_n15 * ddt_scale), (eq66_e1459_d_n16 * ddt_scale), (eq66_e1459_d_n17 * ddt_scale), (eq66_e1459_d_n18 * ddt_scale), (eq66_e1459_d_n19 * ddt_scale), (eq66_e1459_d_n20 * ddt_scale)];
        let eq66_branch_derivative_indices: [usize; 0] = [];
        let eq66_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq66_value),
            &eq66_node_derivative_indices,
            &eq66_node_derivatives,
            &eq66_branch_derivative_indices,
            &eq66_branch_derivatives,
            multiplicity,
        );
        let eq67_e1463: f64 = (var_mult_inst * p.p32);
        let eq67_e1464: f64 = (eq67_e1463).sqrt();
        let eq67_e1466: f64 = (eq67_e1464 * 0.5);
        let eq67_e1468: f64 = (eq67_e1466 * var_cgeff);
        let eq67_e1468_d_n5: f64 = (eq67_e1466 * var_cgeff_dn5);
        let eq67_e1468_d_n6: f64 = (eq67_e1466 * var_cgeff_dn6);
        let eq67_e1468_d_n7: f64 = (eq67_e1466 * var_cgeff_dn7);
        let eq67_e1468_d_n8: f64 = (eq67_e1466 * var_cgeff_dn8);
        let eq67_e1468_d_n12: f64 = (eq67_e1466 * var_cgeff_dn12);
        let eq67_e1468_d_n13: f64 = (eq67_e1466 * var_cgeff_dn13);
        let eq67_e1468_d_n14: f64 = (eq67_e1466 * var_cgeff_dn14);
        let eq67_e1468_d_n15: f64 = (eq67_e1466 * var_cgeff_dn15);
        let eq67_e1468_d_n16: f64 = (eq67_e1466 * var_cgeff_dn16);
        let eq67_e1468_d_n17: f64 = (eq67_e1466 * var_cgeff_dn17);
        let eq67_e1468_d_n18: f64 = (eq67_e1466 * var_cgeff_dn18);
        let eq67_e1468_d_n19: f64 = (eq67_e1466 * var_cgeff_dn19);
        let eq67_e1468_d_n20: f64 = (eq67_e1466 * var_cgeff_dn20);
        let eq67_e1470: f64 = (eq67_e1468 * (nv4 - 0.0));
        let eq67_e1470_d_n5: f64 = (eq67_e1468_d_n5 * (nv4 - 0.0));
        let eq67_e1470_d_n6: f64 = (eq67_e1468_d_n6 * (nv4 - 0.0));
        let eq67_e1470_d_n7: f64 = (eq67_e1468_d_n7 * (nv4 - 0.0));
        let eq67_e1470_d_n8: f64 = (eq67_e1468_d_n8 * (nv4 - 0.0));
        let eq67_e1470_d_n12: f64 = (eq67_e1468_d_n12 * (nv4 - 0.0));
        let eq67_e1470_d_n13: f64 = (eq67_e1468_d_n13 * (nv4 - 0.0));
        let eq67_e1470_d_n14: f64 = (eq67_e1468_d_n14 * (nv4 - 0.0));
        let eq67_e1470_d_n15: f64 = (eq67_e1468_d_n15 * (nv4 - 0.0));
        let eq67_e1470_d_n16: f64 = (eq67_e1468_d_n16 * (nv4 - 0.0));
        let eq67_e1470_d_n17: f64 = (eq67_e1468_d_n17 * (nv4 - 0.0));
        let eq67_e1470_d_n18: f64 = (eq67_e1468_d_n18 * (nv4 - 0.0));
        let eq67_e1470_d_n19: f64 = (eq67_e1468_d_n19 * (nv4 - 0.0));
        let eq67_e1470_d_n20: f64 = (eq67_e1468_d_n20 * (nv4 - 0.0));
        let eq67_e1471: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq67_e1470);
        let eq67_e1472: f64 = (-eq67_e1471);
        let eq67_e1472_d_n4: f64 = (-(eq67_e1468 * ddt_scale));
        let eq67_e1472_d_n5: f64 = (-(eq67_e1470_d_n5 * ddt_scale));
        let eq67_e1472_d_n6: f64 = (-(eq67_e1470_d_n6 * ddt_scale));
        let eq67_e1472_d_n7: f64 = (-(eq67_e1470_d_n7 * ddt_scale));
        let eq67_e1472_d_n8: f64 = (-(eq67_e1470_d_n8 * ddt_scale));
        let eq67_e1472_d_n12: f64 = (-(eq67_e1470_d_n12 * ddt_scale));
        let eq67_e1472_d_n13: f64 = (-(eq67_e1470_d_n13 * ddt_scale));
        let eq67_e1472_d_n14: f64 = (-(eq67_e1470_d_n14 * ddt_scale));
        let eq67_e1472_d_n15: f64 = (-(eq67_e1470_d_n15 * ddt_scale));
        let eq67_e1472_d_n16: f64 = (-(eq67_e1470_d_n16 * ddt_scale));
        let eq67_e1472_d_n17: f64 = (-(eq67_e1470_d_n17 * ddt_scale));
        let eq67_e1472_d_n18: f64 = (-(eq67_e1470_d_n18 * ddt_scale));
        let eq67_e1472_d_n19: f64 = (-(eq67_e1470_d_n19 * ddt_scale));
        let eq67_e1472_d_n20: f64 = (-(eq67_e1470_d_n20 * ddt_scale));
        let eq67_value: f64 = eq67_e1472;
        let eq67_node_derivative_indices: [usize; 14] = [4, 5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq67_node_derivatives: [f64; 14] = [eq67_e1472_d_n4, eq67_e1472_d_n5, eq67_e1472_d_n6, eq67_e1472_d_n7, eq67_e1472_d_n8, eq67_e1472_d_n12, eq67_e1472_d_n13, eq67_e1472_d_n14, eq67_e1472_d_n15, eq67_e1472_d_n16, eq67_e1472_d_n17, eq67_e1472_d_n18, eq67_e1472_d_n19, eq67_e1472_d_n20];
        let eq67_branch_derivative_indices: [usize; 0] = [];
        let eq67_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq67_value),
            &eq67_node_derivative_indices,
            &eq67_node_derivatives,
            &eq67_branch_derivative_indices,
            &eq67_branch_derivatives,
            multiplicity,
        );
        let eq68_e1475: f64 = (var_mult_inst * p.p32);
        let eq68_e1476: f64 = (eq68_e1475).sqrt();
        let eq68_e1478: f64 = (eq68_e1476 * 0.5);
        let eq68_e1480: f64 = (eq68_e1478 * var_cgeff);
        let eq68_e1480_d_n5: f64 = (eq68_e1478 * var_cgeff_dn5);
        let eq68_e1480_d_n6: f64 = (eq68_e1478 * var_cgeff_dn6);
        let eq68_e1480_d_n7: f64 = (eq68_e1478 * var_cgeff_dn7);
        let eq68_e1480_d_n8: f64 = (eq68_e1478 * var_cgeff_dn8);
        let eq68_e1480_d_n12: f64 = (eq68_e1478 * var_cgeff_dn12);
        let eq68_e1480_d_n13: f64 = (eq68_e1478 * var_cgeff_dn13);
        let eq68_e1480_d_n14: f64 = (eq68_e1478 * var_cgeff_dn14);
        let eq68_e1480_d_n15: f64 = (eq68_e1478 * var_cgeff_dn15);
        let eq68_e1480_d_n16: f64 = (eq68_e1478 * var_cgeff_dn16);
        let eq68_e1480_d_n17: f64 = (eq68_e1478 * var_cgeff_dn17);
        let eq68_e1480_d_n18: f64 = (eq68_e1478 * var_cgeff_dn18);
        let eq68_e1480_d_n19: f64 = (eq68_e1478 * var_cgeff_dn19);
        let eq68_e1480_d_n20: f64 = (eq68_e1478 * var_cgeff_dn20);
        let eq68_e1482: f64 = (eq68_e1480 * (nv4 - 0.0));
        let eq68_e1482_d_n5: f64 = (eq68_e1480_d_n5 * (nv4 - 0.0));
        let eq68_e1482_d_n6: f64 = (eq68_e1480_d_n6 * (nv4 - 0.0));
        let eq68_e1482_d_n7: f64 = (eq68_e1480_d_n7 * (nv4 - 0.0));
        let eq68_e1482_d_n8: f64 = (eq68_e1480_d_n8 * (nv4 - 0.0));
        let eq68_e1482_d_n12: f64 = (eq68_e1480_d_n12 * (nv4 - 0.0));
        let eq68_e1482_d_n13: f64 = (eq68_e1480_d_n13 * (nv4 - 0.0));
        let eq68_e1482_d_n14: f64 = (eq68_e1480_d_n14 * (nv4 - 0.0));
        let eq68_e1482_d_n15: f64 = (eq68_e1480_d_n15 * (nv4 - 0.0));
        let eq68_e1482_d_n16: f64 = (eq68_e1480_d_n16 * (nv4 - 0.0));
        let eq68_e1482_d_n17: f64 = (eq68_e1480_d_n17 * (nv4 - 0.0));
        let eq68_e1482_d_n18: f64 = (eq68_e1480_d_n18 * (nv4 - 0.0));
        let eq68_e1482_d_n19: f64 = (eq68_e1480_d_n19 * (nv4 - 0.0));
        let eq68_e1482_d_n20: f64 = (eq68_e1480_d_n20 * (nv4 - 0.0));
        let eq68_e1483: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq68_e1482);
        let eq68_e1484: f64 = (-eq68_e1483);
        let eq68_e1484_d_n4: f64 = (-(eq68_e1480 * ddt_scale));
        let eq68_e1484_d_n5: f64 = (-(eq68_e1482_d_n5 * ddt_scale));
        let eq68_e1484_d_n6: f64 = (-(eq68_e1482_d_n6 * ddt_scale));
        let eq68_e1484_d_n7: f64 = (-(eq68_e1482_d_n7 * ddt_scale));
        let eq68_e1484_d_n8: f64 = (-(eq68_e1482_d_n8 * ddt_scale));
        let eq68_e1484_d_n12: f64 = (-(eq68_e1482_d_n12 * ddt_scale));
        let eq68_e1484_d_n13: f64 = (-(eq68_e1482_d_n13 * ddt_scale));
        let eq68_e1484_d_n14: f64 = (-(eq68_e1482_d_n14 * ddt_scale));
        let eq68_e1484_d_n15: f64 = (-(eq68_e1482_d_n15 * ddt_scale));
        let eq68_e1484_d_n16: f64 = (-(eq68_e1482_d_n16 * ddt_scale));
        let eq68_e1484_d_n17: f64 = (-(eq68_e1482_d_n17 * ddt_scale));
        let eq68_e1484_d_n18: f64 = (-(eq68_e1482_d_n18 * ddt_scale));
        let eq68_e1484_d_n19: f64 = (-(eq68_e1482_d_n19 * ddt_scale));
        let eq68_e1484_d_n20: f64 = (-(eq68_e1482_d_n20 * ddt_scale));
        let eq68_value: f64 = eq68_e1484;
        let eq68_node_derivative_indices: [usize; 14] = [4, 5, 6, 7, 8, 12, 13, 14, 15, 16, 17, 18, 19, 20];
        let eq68_node_derivatives: [f64; 14] = [eq68_e1484_d_n4, eq68_e1484_d_n5, eq68_e1484_d_n6, eq68_e1484_d_n7, eq68_e1484_d_n8, eq68_e1484_d_n12, eq68_e1484_d_n13, eq68_e1484_d_n14, eq68_e1484_d_n15, eq68_e1484_d_n16, eq68_e1484_d_n17, eq68_e1484_d_n18, eq68_e1484_d_n19, eq68_e1484_d_n20];
        let eq68_branch_derivative_indices: [usize; 0] = [];
        let eq68_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq68_value),
            &eq68_node_derivative_indices,
            &eq68_node_derivatives,
            &eq68_branch_derivative_indices,
            &eq68_branch_derivatives,
            multiplicity,
        );
    }
}

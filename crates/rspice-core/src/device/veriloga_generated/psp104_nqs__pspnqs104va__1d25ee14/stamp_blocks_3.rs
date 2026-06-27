#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_48(
        s: &mut Scratch,
    ) {
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
        s.v[2968] = if s.b[2968] { 1.0 } else { 0.0 };

        if (((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) && s.b[2968]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[2969] = ((-s.v[2013]) < 0.0);
        s.v[2969] = if s.b[2969] { 1.0 } else { 0.0 };

        if ((((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) && (!s.b[2968])) && s.b[2969]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) && (!s.b[2968])) && (!s.b[2969])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2016, 2013, 2014);
        }

        s.b[2970] = (((s.v[2016]) as f64).abs() <= s.v[1933]);
        s.v[2970] = if s.b[2970] { 1.0 } else { 0.0 };

        if (s.b[2958] && s.b[2970]) {
            s.store_mul_ad_affine_product_rhs(1996, 2016, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(1991, 1889, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.25), 0.3333333333333333), (-0.70710678));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1990, 1889, 1.0, 2016, 0.5, (-0.235702));
        }

        s.b[2971] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);
        s.v[2971] = if s.b[2971] { 1.0 } else { 0.0 };

        if ((s.b[2958] && (!s.b[2970])) && s.b[2971]) {
            s.store_exp_neg_input(2027, 2016);
        }

        s.b[2972] = ((-s.v[2016]) < 0.0);
        s.v[2972] = if s.b[2972] { 1.0 } else { 0.0 };

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
        s.v[2973] = if s.b[2973] { 1.0 } else { 0.0 };

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
            s.store_mul_ad_product_lhs(1994, A::square(s.ad_value(1992)), s.ad_value(1989), 1989);
            s.store_mul(1985, 2018, 1994);
        }

        s.b[2974] = (s.v[0] == (-1.0));
        s.v[2974] = if s.b[2974] { 1.0 } else { 0.0 };

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
        s.v[2975] = if s.b[2975] { 1.0 } else { 0.0 };

        s.b[2976] = (s.v[1] == 5.0);
        s.v[2976] = if s.b[2976] { 1.0 } else { 0.0 };

        if (s.b[2975] && s.b[2976]) {
            s.store_add_scaled_inputs_ad_lhs(1992, A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1971), (-630.0), s.ad_value(1972), 12.0, s.ad_value(1973), 582.0, s.ad_value(1979), (-97.0)), 1.0, s.ad_value(1978), 7.0), 1.0, s.ad_value(1969), 42.0), 0.007692307692307693, 1970, (168.0 * 0.007692307692307693));
            s.store_sub_scaled_inputs_ad_lhs(1993, A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1972), (-10152.0), s.ad_value(1973), 6048.0, s.ad_value(1971), 6480.0, s.ad_value(1979), (-1008.0)), 1.0, s.ad_value(1978), 72.0), 1.0, s.ad_value(1969), 432.0), 0.015384615384615385, 1970, (1728.0 * 0.015384615384615385));
        }

        s.b[2977] = (s.v[1] == 9.0);
        s.v[2977] = if s.b[2977] { 1.0 } else { 0.0 };

        if ((s.b[2975] && (!s.b[2976])) && s.b[2977]) {
            let assign68700_ad_e92316: A = A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1974), (-81480.0), s.ad_value(1972), (-30.0), s.ad_value(1971), (-303975.0), s.ad_value(1976), (-5820.0)), 1.0, s.ad_value(1977), 1455.0), 1.0, s.ad_value(1969), 20265.0), 1.0, s.ad_value(1975), 21825.0), 2.6434745829918846e-5, s.ad_value(1970), (81060.0 * 2.6434745829918846e-5)), 1.0, s.ad_value(1979), (485.0 / 75658.0));
            s.store_add_scaled_inputs3_mixed_aii(1992, assign68700_ad_e92316, 1.0, 1973, (1455.0 * 0.0055248618784530384), 1978, (6755.0 * 1.3217372914959423e-5));
        }

        if ((s.b[2975] && (!s.b[2976])) && s.b[2977]) {
            let assign68710_ad_e92375: A = A::sub_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1969), 702000.0, s.ad_value(1975), 756000.0, s.ad_value(1972), (-16614600.0), s.ad_value(1971), 10530000.0), 1.0, s.ad_value(1970), 2808000.0), 1.0, s.ad_value(1978), 117000.0), 1.0, s.ad_value(1979), 8400.0), 1.0, s.ad_value(1976), 201600.0), 1.0, s.ad_value(1977), 50400.0), 1.0, s.ad_value(1974), 2822400.0);
            s.store_add_scaled_inputs_ad_lhs(1993, assign68710_ad_e92375, 2.6434745829918846e-5, 1973, (50400.0 * 0.0055248618784530384));
        }

        if ((s.b[2975] && (!s.b[2976])) && (!s.b[2977])) {
            s.store_scalar(1992, 0.0);
            s.store_scalar(1993, 0.0);
        }

        if s.b[2975] {
            s.store_add_div_lhs_indices(2027, 1972, 1937, 1890);
        }

        s.b[2978] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[2978] = if s.b[2978] { 1.0 } else { 0.0 };

        if (s.b[2975] && s.b[2978]) {
            s.store_div(2016, 2027, 1940);
        }

        s.b[2979] = (s.v[2027] < (-s.v[1941]));
        s.v[2979] = if s.b[2979] { 1.0 } else { 0.0 };

        if ((s.b[2975] && (!s.b[2978])) && s.b[2979]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_products_mixed_aaia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[2980] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[2980] = if s.b[2980] { 1.0 } else { 0.0 };

        if (((s.b[2975] && (!s.b[2978])) && s.b[2979]) && s.b[2980]) {
            s.store_exp(2005, 2015);
        }

        s.b[2981] = (s.v[2015] < 0.0);
        s.v[2981] = if s.b[2981] { 1.0 } else { 0.0 };

        if ((((s.b[2975] && (!s.b[2978])) && s.b[2979]) && (!s.b[2980])) && s.b[2981]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[2975] && (!s.b[2978])) && s.b[2979]) && (!s.b[2980])) && (!s.b[2981])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[2975] && (!s.b[2978])) && s.b[2979]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
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
        s.v[2982] = if s.b[2982] { 1.0 } else { 0.0 };

        if (((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) && s.b[2982]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[2983] = ((-s.v[2011]) < 0.0);
        s.v[2983] = if s.b[2983] { 1.0 } else { 0.0 };

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
        s.v[2984] = if s.b[2984] { 1.0 } else { 0.0 };

        if (((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) && s.b[2984]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[2985] = ((-s.v[2013]) < 0.0);
        s.v[2985] = if s.b[2985] { 1.0 } else { 0.0 };

        if ((((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) && (!s.b[2984])) && s.b[2985]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) && (!s.b[2984])) && (!s.b[2985])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2016, 2013, 2014);
        }

        s.b[2986] = (((s.v[2016]) as f64).abs() <= s.v[1933]);
        s.v[2986] = if s.b[2986] { 1.0 } else { 0.0 };

        if (s.b[2975] && s.b[2986]) {
            s.store_mul_ad_affine_product_rhs(1996, 2016, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(1991, 1889, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.25), 0.3333333333333333), (-0.70710678));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1990, 1889, 1.0, 2016, 0.5, (-0.235702));
        }

        s.b[2987] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);
        s.v[2987] = if s.b[2987] { 1.0 } else { 0.0 };

        if ((s.b[2975] && (!s.b[2986])) && s.b[2987]) {
            s.store_exp_neg_input(2027, 2016);
        }

        s.b[2988] = ((-s.v[2016]) < 0.0);
        s.v[2988] = if s.b[2988] { 1.0 } else { 0.0 };

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
        s.v[2989] = if s.b[2989] { 1.0 } else { 0.0 };

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
            s.store_mul_ad_product_lhs(1994, A::square(s.ad_value(1992)), s.ad_value(1989), 1989);
            s.store_mul(1985, 2018, 1994);
        }

        s.b[2990] = (s.v[0] == (-1.0));
        s.v[2990] = if s.b[2990] { 1.0 } else { 0.0 };

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
        s.v[2991] = if s.b[2991] { 1.0 } else { 0.0 };

        s.b[2992] = (s.v[1] == 5.0);
        s.v[2992] = if s.b[2992] { 1.0 } else { 0.0 };

        if (s.b[2991] && s.b[2992]) {
            s.store_sub_scaled_inputs_ad_lhs(1992, A::add_scaled_inputs(A::sub(A::add_scaled_inputs4(s.ad_value(1972), (-336.0), s.ad_value(1973), 84.0, s.ad_value(1971), 90.0, s.ad_value(1979), 181.0), s.ad_value(1978)), 1.0, s.ad_value(1969), 6.0), 0.015384615384615385, 1970, (24.0 * 0.015384615384615385));
            s.store_sub_scaled_inputs_ad_lhs(1993, A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1978), 18.0, s.ad_value(1979), 3762.0, s.ad_value(1972), 6048.0, s.ad_value(1970), 432.0), 1.0, s.ad_value(1971), 1620.0), 1.0, s.ad_value(1969), 108.0), 0.015384615384615385, 1973, (8532.0 * 0.015384615384615385));
        }

        s.b[2993] = (s.v[1] == 9.0);
        s.v[2993] = if s.b[2993] { 1.0 } else { 0.0 };

        if ((s.b[2991] && (!s.b[2992])) && s.b[2993]) {
            s.store_scaled_sub_ad(1992, A::add(A::add(A::add_scaled_inputs4(s.ad_value(1974), 1680.0, s.ad_value(1972), (-1680.0), s.ad_value(1979), 5.0, s.ad_value(1978), (-5.0)), A::sub_scaled_inputs(s.ad_value(1971), 450.0, s.ad_value(1975), 450.0)), A::sub_scaled_inputs(s.ad_value(1976), 120.0, s.ad_value(1970), 120.0)), A::sub_scaled_inputs(s.ad_value(1977), 30.0, s.ad_value(1969), 30.0), 0.004784688995215311);
        }

        if ((s.b[2991] && (!s.b[2992])) && s.b[2993]) {
            let assign69520_ad_e93698: A = A::add(A::add(A::add(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1969), (-900.0), s.ad_value(1977), (-900.0), s.ad_value(1975), (-13500.0), s.ad_value(1971), (-13500.0)), 1.0, s.ad_value(1973), 79500.0), A::add_scaled_inputs(s.ad_value(1972), 50400.0, s.ad_value(1974), 50400.0)), A::add_scaled_inputs(s.ad_value(1970), 3600.0, s.ad_value(1976), 3600.0)), A::add_scaled_inputs(s.ad_value(1978), 150.0, s.ad_value(1979), 150.0));
            s.store_scale_ad(1993, assign69520_ad_e93698, 0.0055248618784530384);
        }

        if ((s.b[2991] && (!s.b[2992])) && (!s.b[2993])) {
            s.store_scalar(1992, 0.0);
            s.store_scalar(1993, 0.0);
        }

        if s.b[2991] {
            s.store_add_div_lhs_indices(2027, 1973, 1937, 1890);
        }

    }

    pub(super) fn stamp_transient_block_49(
        s: &mut Scratch,
    ) {
        s.b[2994] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[2994] = if s.b[2994] { 1.0 } else { 0.0 };

        if (s.b[2991] && s.b[2994]) {
            s.store_div(2016, 2027, 1940);
        }

        s.b[2995] = (s.v[2027] < (-s.v[1941]));
        s.v[2995] = if s.b[2995] { 1.0 } else { 0.0 };

        if ((s.b[2991] && (!s.b[2994])) && s.b[2995]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_products_mixed_aaia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[2996] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[2996] = if s.b[2996] { 1.0 } else { 0.0 };

        if (((s.b[2991] && (!s.b[2994])) && s.b[2995]) && s.b[2996]) {
            s.store_exp(2005, 2015);
        }

        s.b[2997] = (s.v[2015] < 0.0);
        s.v[2997] = if s.b[2997] { 1.0 } else { 0.0 };

        if ((((s.b[2991] && (!s.b[2994])) && s.b[2995]) && (!s.b[2996])) && s.b[2997]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[2991] && (!s.b[2994])) && s.b[2995]) && (!s.b[2996])) && (!s.b[2997])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[2991] && (!s.b[2994])) && s.b[2995]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
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
        s.v[2998] = if s.b[2998] { 1.0 } else { 0.0 };

        if (((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) && s.b[2998]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[2999] = ((-s.v[2011]) < 0.0);
        s.v[2999] = if s.b[2999] { 1.0 } else { 0.0 };

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
        s.v[3000] = if s.b[3000] { 1.0 } else { 0.0 };

        if (((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) && s.b[3000]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3001] = ((-s.v[2013]) < 0.0);
        s.v[3001] = if s.b[3001] { 1.0 } else { 0.0 };

        if ((((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) && (!s.b[3000])) && s.b[3001]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) && (!s.b[3000])) && (!s.b[3001])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2016, 2013, 2014);
        }

        s.b[3002] = (((s.v[2016]) as f64).abs() <= s.v[1933]);
        s.v[3002] = if s.b[3002] { 1.0 } else { 0.0 };

        if (s.b[2991] && s.b[3002]) {
            s.store_mul_ad_affine_product_rhs(1996, 2016, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(1991, 1889, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.25), 0.3333333333333333), (-0.70710678));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1990, 1889, 1.0, 2016, 0.5, (-0.235702));
        }

        s.b[3003] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);
        s.v[3003] = if s.b[3003] { 1.0 } else { 0.0 };

        if ((s.b[2991] && (!s.b[3002])) && s.b[3003]) {
            s.store_exp_neg_input(2027, 2016);
        }

        s.b[3004] = ((-s.v[2016]) < 0.0);
        s.v[3004] = if s.b[3004] { 1.0 } else { 0.0 };

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
        s.v[3005] = if s.b[3005] { 1.0 } else { 0.0 };

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
            s.store_mul_ad_product_lhs(1994, A::square(s.ad_value(1992)), s.ad_value(1989), 1989);
            s.store_mul(1985, 2018, 1994);
        }

        s.b[3006] = (s.v[0] == (-1.0));
        s.v[3006] = if s.b[3006] { 1.0 } else { 0.0 };

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
        s.v[3007] = if s.b[3007] { 1.0 } else { 0.0 };

        s.b[3008] = (s.v[1] == 9.0);
        s.v[3008] = if s.b[3008] { 1.0 } else { 0.0 };

        if (s.b[3007] && s.b[3008]) {
            let assign70290_ad_e94912: A = A::sub(A::add_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1974), 30.0, s.ad_value(1972), 81480.0, s.ad_value(1971), (-21825.0), s.ad_value(1976), (-81060.0)), 1.0, s.ad_value(1977), 20265.0), 1.0, s.ad_value(1969), 1455.0), 1.0, s.ad_value(1975), 303975.0), 2.6434745829918846e-5, s.ad_value(1970), (5820.0 * 2.6434745829918846e-5)), A::sub_scaled_inputs(s.ad_value(1979), (6755.0 * 1.3217372914959423e-5), s.ad_value(1978), (485.0 * 1.3217372914959423e-5)));
            s.store_sub_scaled_ad_lhs(1992, assign70290_ad_e94912, 1973, (1455.0 / 181.0));
        }

        if (s.b[3007] && s.b[3008]) {
            let assign70300_ad_e94962: A = A::sub_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1969), 50400.0, s.ad_value(1975), 10530000.0, s.ad_value(1972), (-2822400.0), s.ad_value(1971), 756000.0), 1.0, s.ad_value(1970), 201600.0), 1.0, s.ad_value(1978), 8400.0), 1.0, s.ad_value(1979), 117000.0), 1.0, s.ad_value(1976), 2808000.0), 1.0, s.ad_value(1977), 702000.0), 1.0, s.ad_value(1974), 16614600.0);
            s.store_add_scaled_inputs_ad_lhs(1993, assign70300_ad_e94962, 2.6434745829918846e-5, 1973, (50400.0 * 0.0055248618784530384));
        }

        if (s.b[3007] && (!s.b[3008])) {
            s.store_scalar(1992, 0.0);
            s.store_scalar(1993, 0.0);
        }

        if s.b[3007] {
            s.store_add_div_lhs_indices(2027, 1974, 1937, 1890);
        }

        s.b[3009] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3009] = if s.b[3009] { 1.0 } else { 0.0 };

        if (s.b[3007] && s.b[3009]) {
            s.store_div(2016, 2027, 1940);
        }

        s.b[3010] = (s.v[2027] < (-s.v[1941]));
        s.v[3010] = if s.b[3010] { 1.0 } else { 0.0 };

        if ((s.b[3007] && (!s.b[3009])) && s.b[3010]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_products_mixed_aaia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3011] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3011] = if s.b[3011] { 1.0 } else { 0.0 };

        if (((s.b[3007] && (!s.b[3009])) && s.b[3010]) && s.b[3011]) {
            s.store_exp(2005, 2015);
        }

        s.b[3012] = (s.v[2015] < 0.0);
        s.v[3012] = if s.b[3012] { 1.0 } else { 0.0 };

        if ((((s.b[3007] && (!s.b[3009])) && s.b[3010]) && (!s.b[3011])) && s.b[3012]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[3007] && (!s.b[3009])) && s.b[3010]) && (!s.b[3011])) && (!s.b[3012])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[3007] && (!s.b[3009])) && s.b[3010]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
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
        s.v[3013] = if s.b[3013] { 1.0 } else { 0.0 };

        if (((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) && s.b[3013]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3014] = ((-s.v[2011]) < 0.0);
        s.v[3014] = if s.b[3014] { 1.0 } else { 0.0 };

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
        s.v[3015] = if s.b[3015] { 1.0 } else { 0.0 };

        if (((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) && s.b[3015]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3016] = ((-s.v[2013]) < 0.0);
        s.v[3016] = if s.b[3016] { 1.0 } else { 0.0 };

        if ((((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) && (!s.b[3015])) && s.b[3016]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) && (!s.b[3015])) && (!s.b[3016])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2016, 2013, 2014);
        }

        s.b[3017] = (((s.v[2016]) as f64).abs() <= s.v[1933]);
        s.v[3017] = if s.b[3017] { 1.0 } else { 0.0 };

        if (s.b[3007] && s.b[3017]) {
            s.store_mul_ad_affine_product_rhs(1996, 2016, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(1991, 1889, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.25), 0.3333333333333333), (-0.70710678));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1990, 1889, 1.0, 2016, 0.5, (-0.235702));
        }

        s.b[3018] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);
        s.v[3018] = if s.b[3018] { 1.0 } else { 0.0 };

        if ((s.b[3007] && (!s.b[3017])) && s.b[3018]) {
            s.store_exp_neg_input(2027, 2016);
        }

        s.b[3019] = ((-s.v[2016]) < 0.0);
        s.v[3019] = if s.b[3019] { 1.0 } else { 0.0 };

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
        s.v[3020] = if s.b[3020] { 1.0 } else { 0.0 };

        if ((s.b[3007] && (!s.b[3017])) && s.b[3020]) {
            s.store_neg(1996, 1996);
        }

    }

    pub(super) fn stamp_transient_block_50(
        s: &mut Scratch,
    ) {
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
            s.store_mul_ad_product_lhs(1994, A::square(s.ad_value(1992)), s.ad_value(1989), 1989);
            s.store_mul(1985, 2018, 1994);
        }

        s.b[3021] = (s.v[0] == (-1.0));
        s.v[3021] = if s.b[3021] { 1.0 } else { 0.0 };

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
        s.v[3022] = if s.b[3022] { 1.0 } else { 0.0 };

        s.b[3023] = (s.v[1] == 9.0);
        s.v[3023] = if s.b[3023] { 1.0 } else { 0.0 };

        if (s.b[3022] && s.b[3023]) {
            let assign71070_ad_e96173: A = A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1974), (-304200.0), s.ad_value(1972), (-21840.0), s.ad_value(1979), 12605.0, s.ad_value(1971), 5850.0), 1.0, s.ad_value(1976), 302520.0), 1.0, s.ad_value(1978), 65.0), 1.0, s.ad_value(1977), 75630.0), 1.0, s.ad_value(1969), 390.0), 1.0, s.ad_value(1975), 420.0), 1.0, s.ad_value(1970), 1560.0);
            s.store_add_scaled_inputs_ad_lhs(1992, assign71070_ad_e96173, 2.6434745829918846e-5, 1973, (390.0 / 181.0));
        }

        if (s.b[3022] && s.b[3023]) {
            let assign71080_ad_e96226: A = A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1969), (-13500.0), s.ad_value(1975), (-16601100.0), s.ad_value(1972), 756000.0, s.ad_value(1971), (-202500.0)), 1.0, s.ad_value(1970), 54000.0), 1.0, s.ad_value(1978), 2250.0), 1.0, s.ad_value(1979), 436650.0), 1.0, s.ad_value(1976), 10479600.0), 1.0, s.ad_value(1977), 2619900.0), 1.0, s.ad_value(1974), 10530000.0);
            s.store_sub_scaled_inputs_ad_lhs(1993, assign71080_ad_e96226, 2.6434745829918846e-5, 1973, (13500.0 * 0.0055248618784530384));
        }

        if (s.b[3022] && (!s.b[3023])) {
            s.store_scalar(1992, 0.0);
            s.store_scalar(1993, 0.0);
        }

        if s.b[3022] {
            s.store_add_div_lhs_indices(2027, 1975, 1937, 1890);
        }

        s.b[3024] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3024] = if s.b[3024] { 1.0 } else { 0.0 };

        if (s.b[3022] && s.b[3024]) {
            s.store_div(2016, 2027, 1940);
        }

        s.b[3025] = (s.v[2027] < (-s.v[1941]));
        s.v[3025] = if s.b[3025] { 1.0 } else { 0.0 };

        if ((s.b[3022] && (!s.b[3024])) && s.b[3025]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_products_mixed_aaia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3026] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3026] = if s.b[3026] { 1.0 } else { 0.0 };

        if (((s.b[3022] && (!s.b[3024])) && s.b[3025]) && s.b[3026]) {
            s.store_exp(2005, 2015);
        }

        s.b[3027] = (s.v[2015] < 0.0);
        s.v[3027] = if s.b[3027] { 1.0 } else { 0.0 };

        if ((((s.b[3022] && (!s.b[3024])) && s.b[3025]) && (!s.b[3026])) && s.b[3027]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[3022] && (!s.b[3024])) && s.b[3025]) && (!s.b[3026])) && (!s.b[3027])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[3022] && (!s.b[3024])) && s.b[3025]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
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
        s.v[3028] = if s.b[3028] { 1.0 } else { 0.0 };

        if (((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) && s.b[3028]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3029] = ((-s.v[2011]) < 0.0);
        s.v[3029] = if s.b[3029] { 1.0 } else { 0.0 };

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

        s.b[3030] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[3030] = if s.b[3030] { 1.0 } else { 0.0 };

        if (((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) && s.b[3030]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3031] = ((-s.v[2013]) < 0.0);
        s.v[3031] = if s.b[3031] { 1.0 } else { 0.0 };

        if ((((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) && (!s.b[3030])) && s.b[3031]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) && (!s.b[3030])) && (!s.b[3031])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2016, 2013, 2014);
        }

        s.b[3032] = (((s.v[2016]) as f64).abs() <= s.v[1933]);
        s.v[3032] = if s.b[3032] { 1.0 } else { 0.0 };

        if (s.b[3022] && s.b[3032]) {
            s.store_mul_ad_affine_product_rhs(1996, 2016, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(1991, 1889, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.25), 0.3333333333333333), (-0.70710678));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1990, 1889, 1.0, 2016, 0.5, (-0.235702));
        }

        s.b[3033] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);
        s.v[3033] = if s.b[3033] { 1.0 } else { 0.0 };

        if ((s.b[3022] && (!s.b[3032])) && s.b[3033]) {
            s.store_exp_neg_input(2027, 2016);
        }

        s.b[3034] = ((-s.v[2016]) < 0.0);
        s.v[3034] = if s.b[3034] { 1.0 } else { 0.0 };

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
        s.v[3035] = if s.b[3035] { 1.0 } else { 0.0 };

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
            s.store_mul_ad_product_lhs(1994, A::square(s.ad_value(1992)), s.ad_value(1989), 1989);
            s.store_mul(1985, 2018, 1994);
        }

        s.b[3036] = (s.v[0] == (-1.0));
        s.v[3036] = if s.b[3036] { 1.0 } else { 0.0 };

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
        s.v[3037] = if s.b[3037] { 1.0 } else { 0.0 };

        s.b[3038] = (s.v[1] == 9.0);
        s.v[3038] = if s.b[3038] { 1.0 } else { 0.0 };

        if (s.b[3037] && s.b[3038]) {
            let assign71850_ad_e97440: A = A::add(A::add_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1974), 81900.0, s.ad_value(1972), 5880.0, s.ad_value(1971), (-1575.0), s.ad_value(1976), 5850.0), 1.0, s.ad_value(1977), 282255.0), 1.0, s.ad_value(1969), 105.0), 1.0, s.ad_value(1975), 305655.0), 2.6434745829918846e-5, s.ad_value(1970), (420.0 * 2.6434745829918846e-5)), A::sub_scaled_inputs(s.ad_value(1978), (35.0 * 1.3217372914959423e-5), s.ad_value(1979), (94085.0 * 1.3217372914959423e-5)));
            s.store_sub_scaled_ad_lhs(1992, assign71850_ad_e97440, 1973, (105.0 / 181.0));
        }

        if (s.b[3037] && s.b[3038]) {
            let assign71860_ad_e97490: A = A::sub_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1969), 3600.0, s.ad_value(1975), 10479600.0, s.ad_value(1972), (-201600.0), s.ad_value(1971), 54000.0), 1.0, s.ad_value(1970), 14400.0), 1.0, s.ad_value(1978), 600.0), 1.0, s.ad_value(1979), 1629600.0), 1.0, s.ad_value(1976), 16413000.0), 1.0, s.ad_value(1977), 9777600.0), 1.0, s.ad_value(1974), 2808000.0);
            s.store_add_scaled_inputs_ad_lhs(1993, assign71860_ad_e97490, 2.6434745829918846e-5, 1973, (3600.0 * 0.0055248618784530384));
        }

        if (s.b[3037] && (!s.b[3038])) {
            s.store_scalar(1992, 0.0);
            s.store_scalar(1993, 0.0);
        }

        if s.b[3037] {
            s.store_add_div_lhs_indices(2027, 1976, 1937, 1890);
        }

        s.b[3039] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3039] = if s.b[3039] { 1.0 } else { 0.0 };

        if (s.b[3037] && s.b[3039]) {
            s.store_div(2016, 2027, 1940);
        }

        s.b[3040] = (s.v[2027] < (-s.v[1941]));
        s.v[3040] = if s.b[3040] { 1.0 } else { 0.0 };

        if ((s.b[3037] && (!s.b[3039])) && s.b[3040]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_products_mixed_aaia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3041] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3041] = if s.b[3041] { 1.0 } else { 0.0 };

        if (((s.b[3037] && (!s.b[3039])) && s.b[3040]) && s.b[3041]) {
            s.store_exp(2005, 2015);
        }

        s.b[3042] = (s.v[2015] < 0.0);
        s.v[3042] = if s.b[3042] { 1.0 } else { 0.0 };

        if ((((s.b[3037] && (!s.b[3039])) && s.b[3040]) && (!s.b[3041])) && s.b[3042]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[3037] && (!s.b[3039])) && s.b[3040]) && (!s.b[3041])) && (!s.b[3042])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[3037] && (!s.b[3039])) && s.b[3040]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
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
        s.v[3043] = if s.b[3043] { 1.0 } else { 0.0 };

        if (((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) && s.b[3043]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3044] = ((-s.v[2011]) < 0.0);
        s.v[3044] = if s.b[3044] { 1.0 } else { 0.0 };

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

    }

    pub(super) fn stamp_transient_block_51(
        s: &mut Scratch,
    ) {
        s.b[3045] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[3045] = if s.b[3045] { 1.0 } else { 0.0 };

        if (((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) && s.b[3045]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3046] = ((-s.v[2013]) < 0.0);
        s.v[3046] = if s.b[3046] { 1.0 } else { 0.0 };

        if ((((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) && (!s.b[3045])) && s.b[3046]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) && (!s.b[3045])) && (!s.b[3046])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2016, 2013, 2014);
        }

        s.b[3047] = (((s.v[2016]) as f64).abs() <= s.v[1933]);
        s.v[3047] = if s.b[3047] { 1.0 } else { 0.0 };

        if (s.b[3037] && s.b[3047]) {
            s.store_mul_ad_affine_product_rhs(1996, 2016, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(1991, 1889, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.25), 0.3333333333333333), (-0.70710678));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1990, 1889, 1.0, 2016, 0.5, (-0.235702));
        }

        s.b[3048] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);
        s.v[3048] = if s.b[3048] { 1.0 } else { 0.0 };

        if ((s.b[3037] && (!s.b[3047])) && s.b[3048]) {
            s.store_exp_neg_input(2027, 2016);
        }

        s.b[3049] = ((-s.v[2016]) < 0.0);
        s.v[3049] = if s.b[3049] { 1.0 } else { 0.0 };

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
        s.v[3050] = if s.b[3050] { 1.0 } else { 0.0 };

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
            s.store_mul_ad_product_lhs(1994, A::square(s.ad_value(1992)), s.ad_value(1989), 1989);
            s.store_mul(1985, 2018, 1994);
        }

        s.b[3051] = (s.v[0] == (-1.0));
        s.v[3051] = if s.b[3051] { 1.0 } else { 0.0 };

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
        s.v[3052] = if s.b[3052] { 1.0 } else { 0.0 };

        s.b[3053] = (s.v[1] == 9.0);
        s.v[3053] = if s.b[3053] { 1.0 } else { 0.0 };

        if (s.b[3052] && s.b[3053]) {
            let assign72630_ad_e98701: A = A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1974), (-23400.0), s.ad_value(1972), (-1680.0), s.ad_value(1979), 175565.0, s.ad_value(1971), 450.0), 1.0, s.ad_value(1976), 325920.0), 1.0, s.ad_value(1978), 5.0), 1.0, s.ad_value(1977), 81480.0), 1.0, s.ad_value(1969), 30.0), 1.0, s.ad_value(1975), 87330.0), 1.0, s.ad_value(1970), 120.0);
            s.store_add_scaled_inputs_ad_lhs(1992, assign72630_ad_e98701, 2.6434745829918846e-5, 1973, (30.0 * 0.0055248618784530384));
        }

        if (s.b[3052] && s.b[3053]) {
            let assign72640_ad_e98754: A = A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1969), (-900.0), s.ad_value(1975), (-2619900.0), s.ad_value(1972), 50400.0, s.ad_value(1971), (-13500.0)), 1.0, s.ad_value(1970), 3600.0), 1.0, s.ad_value(1978), 150.0), 1.0, s.ad_value(1979), 6081750.0), 1.0, s.ad_value(1976), 9777600.0), 1.0, s.ad_value(1977), 13793100.0), 1.0, s.ad_value(1974), 702000.0);
            s.store_sub_scaled_inputs_ad_lhs(1993, assign72640_ad_e98754, 2.6434745829918846e-5, 1973, (900.0 * 0.0055248618784530384));
        }

        if (s.b[3052] && (!s.b[3053])) {
            s.store_scalar(1992, 0.0);
            s.store_scalar(1993, 0.0);
        }

        if s.b[3052] {
            s.store_add_div_lhs_indices(2027, 1977, 1937, 1890);
        }

        s.b[3054] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3054] = if s.b[3054] { 1.0 } else { 0.0 };

        if (s.b[3052] && s.b[3054]) {
            s.store_div(2016, 2027, 1940);
        }

        s.b[3055] = (s.v[2027] < (-s.v[1941]));
        s.v[3055] = if s.b[3055] { 1.0 } else { 0.0 };

        if ((s.b[3052] && (!s.b[3054])) && s.b[3055]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_products_mixed_aaia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3056] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3056] = if s.b[3056] { 1.0 } else { 0.0 };

        if (((s.b[3052] && (!s.b[3054])) && s.b[3055]) && s.b[3056]) {
            s.store_exp(2005, 2015);
        }

        s.b[3057] = (s.v[2015] < 0.0);
        s.v[3057] = if s.b[3057] { 1.0 } else { 0.0 };

        if ((((s.b[3052] && (!s.b[3054])) && s.b[3055]) && (!s.b[3056])) && s.b[3057]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[3052] && (!s.b[3054])) && s.b[3055]) && (!s.b[3056])) && (!s.b[3057])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[3052] && (!s.b[3054])) && s.b[3055]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
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
        s.v[3058] = if s.b[3058] { 1.0 } else { 0.0 };

        if (((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) && s.b[3058]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3059] = ((-s.v[2011]) < 0.0);
        s.v[3059] = if s.b[3059] { 1.0 } else { 0.0 };

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
        s.v[3060] = if s.b[3060] { 1.0 } else { 0.0 };

        if (((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) && s.b[3060]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3061] = ((-s.v[2013]) < 0.0);
        s.v[3061] = if s.b[3061] { 1.0 } else { 0.0 };

        if ((((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) && (!s.b[3060])) && s.b[3061]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) && (!s.b[3060])) && (!s.b[3061])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2016, 2013, 2014);
        }

        s.b[3062] = (((s.v[2016]) as f64).abs() <= s.v[1933]);
        s.v[3062] = if s.b[3062] { 1.0 } else { 0.0 };

        if (s.b[3052] && s.b[3062]) {
            s.store_mul_ad_affine_product_rhs(1996, 2016, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
            s.store_mul_sub_from_scalar_ad_rhs_scaled_output(1991, 1889, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.25), 0.3333333333333333), (-0.70710678));
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1990, 1889, 1.0, 2016, 0.5, (-0.235702));
        }

        s.b[3063] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);
        s.v[3063] = if s.b[3063] { 1.0 } else { 0.0 };

        if ((s.b[3052] && (!s.b[3062])) && s.b[3063]) {
            s.store_exp_neg_input(2027, 2016);
        }

        s.b[3064] = ((-s.v[2016]) < 0.0);
        s.v[3064] = if s.b[3064] { 1.0 } else { 0.0 };

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
        s.v[3065] = if s.b[3065] { 1.0 } else { 0.0 };

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
            s.store_mul_ad_product_lhs(1994, A::square(s.ad_value(1992)), s.ad_value(1989), 1989);
            s.store_mul(1985, 2018, 1994);
        }

        s.b[3066] = (s.v[0] == (-1.0));
        s.v[3066] = if s.b[3066] { 1.0 } else { 0.0 };

        if (s.b[3052] && s.b[3066]) {
            s.store_div_scaled_value_offset_denominator(1985, s.ad_value(1985), 1.0, A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0, 1.0);
        }

        if s.b[3052] {
            s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);
            s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);
            s.store_add_scaled_product_indices(2028, 1993, 1.0, 1994, 1990, (-1.0));
            s.store_mul_sub_ad_rhs(1959, 2019, s.ad_value(2017), A::div_scaled_product(A::mul3(s.ad_value(1985), s.ad_value(1987), s.ad_value(2028)), s.ad_value(2019), 1.0, s.ad_value(2027), 1.0));
        }

        s.v[1981] = 0.0;

        s.v[1982] = 0.0;

        s.v[1980] = 0.0;

        s.b[3067] = (s.v[1] != 0.0);
        s.v[3067] = if s.b[3067] { 1.0 } else { 0.0 };

        s.b[3068] = (s.v[1] == 1.0);
        s.v[3068] = if s.b[3068] { 1.0 } else { 0.0 };

        if (s.b[3067] && s.b[3068]) {
            s.store_add_scaled_inputs3_indices(1981, 1978, (17.0 * 0.010416666666666666), 1969, (30.0 * 0.010416666666666666), 1979, 0.010416666666666666);
            s.store_add_scaled_inputs3_indices(1982, 1978, 0.010416666666666666, 1969, (30.0 * 0.010416666666666666), 1979, (17.0 * 0.010416666666666666));
            s.store_add_div_lhs_indices(2027, 1969, 1937, 1890);
        }

        s.b[3069] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3069] = if s.b[3069] { 1.0 } else { 0.0 };

        if ((s.b[3067] && s.b[3068]) && s.b[3069]) {
            s.store_div(2028, 2027, 1940);
        }

        s.b[3070] = (s.v[2027] < (-s.v[1941]));
        s.v[3070] = if s.b[3070] { 1.0 } else { 0.0 };

        if (((s.b[3067] && s.b[3068]) && (!s.b[3069])) && s.b[3070]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_products_mixed_aaia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
        }

    }

    pub(super) fn stamp_transient_block_52(
        s: &mut Scratch,
    ) {
        if (((s.b[3067] && s.b[3068]) && (!s.b[3069])) && s.b[3070]) {
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3071] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3071] = if s.b[3071] { 1.0 } else { 0.0 };

        if ((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && s.b[3070]) && s.b[3071]) {
            s.store_exp(2005, 2015);
        }

        s.b[3072] = (s.v[2015] < 0.0);
        s.v[3072] = if s.b[3072] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && s.b[3070]) && (!s.b[3071])) && s.b[3072]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && s.b[3070]) && (!s.b[3071])) && (!s.b[3072])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[3067] && s.b[3068]) && (!s.b[3069])) && s.b[3070]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
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
        s.v[3073] = if s.b[3073] { 1.0 } else { 0.0 };

        if ((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) && s.b[3073]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3074] = ((-s.v[2011]) < 0.0);
        s.v[3074] = if s.b[3074] { 1.0 } else { 0.0 };

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
        s.v[3075] = if s.b[3075] { 1.0 } else { 0.0 };

        if ((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) && s.b[3075]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3076] = ((-s.v[2013]) < 0.0);
        s.v[3076] = if s.b[3076] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) && (!s.b[3075])) && s.b[3076]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) && (!s.b[3075])) && (!s.b[3076])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2028, 2013, 2014);
        }

        if (s.b[3067] && s.b[3068]) {
            s.store_add_scaled_inputs4_indices(1980, 1890, 1.0, 1983, (-0.16666666666666666), 2028, (-(4.0 * 0.16666666666666666)), 1984, (-0.16666666666666666));
        }

        s.b[3077] = (s.v[1] == 2.0);
        s.v[3077] = if s.b[3077] { 1.0 } else { 0.0 };

        if ((s.b[3067] && (!s.b[3068])) && s.b[3077]) {
            s.store_add_scaled_inputs4_indices(1981, 1978, (11.0 * 0.011111111111111112), 1969, (24.0 * 0.011111111111111112), 1970, (9.0 * 0.011111111111111112), 1979, 0.011111111111111112);
            s.store_add_scaled_inputs4_indices(1982, 1979, (11.0 * 0.011111111111111112), 1970, (24.0 * 0.011111111111111112), 1969, (9.0 * 0.011111111111111112), 1978, 0.011111111111111112);
            s.store_add_div_lhs_indices(2027, 1969, 1937, 1890);
        }

        s.b[3078] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3078] = if s.b[3078] { 1.0 } else { 0.0 };

        if (((s.b[3067] && (!s.b[3068])) && s.b[3077]) && s.b[3078]) {
            s.store_div(2028, 2027, 1940);
        }

        s.b[3079] = (s.v[2027] < (-s.v[1941]));
        s.v[3079] = if s.b[3079] { 1.0 } else { 0.0 };

        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && s.b[3079]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_products_mixed_aaia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3080] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3080] = if s.b[3080] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && s.b[3079]) && s.b[3080]) {
            s.store_exp(2005, 2015);
        }

        s.b[3081] = (s.v[2015] < 0.0);
        s.v[3081] = if s.b[3081] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && s.b[3079]) && (!s.b[3080])) && s.b[3081]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && s.b[3079]) && (!s.b[3080])) && (!s.b[3081])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && s.b[3079]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
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
        s.v[3082] = if s.b[3082] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && (!s.b[3079])) && s.b[3082]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3083] = ((-s.v[2011]) < 0.0);
        s.v[3083] = if s.b[3083] { 1.0 } else { 0.0 };

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
        s.v[3084] = if s.b[3084] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && (!s.b[3079])) && s.b[3084]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3085] = ((-s.v[2013]) < 0.0);
        s.v[3085] = if s.b[3085] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && (!s.b[3079])) && (!s.b[3084])) && s.b[3085]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && (!s.b[3079])) && (!s.b[3084])) && (!s.b[3085])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && (!s.b[3079])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2028, 2013, 2014);
        }

        if ((s.b[3067] && (!s.b[3068])) && s.b[3077]) {
            s.store_add_div_lhs_indices(2027, 1970, 1937, 1890);
        }

        s.b[3086] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3086] = if s.b[3086] { 1.0 } else { 0.0 };

        if (((s.b[3067] && (!s.b[3068])) && s.b[3077]) && s.b[3086]) {
            s.store_div(2029, 2027, 1940);
        }

        s.b[3087] = (s.v[2027] < (-s.v[1941]));
        s.v[3087] = if s.b[3087] { 1.0 } else { 0.0 };

        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && s.b[3087]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_products_mixed_aaia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3088] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3088] = if s.b[3088] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && s.b[3087]) && s.b[3088]) {
            s.store_exp(2005, 2015);
        }

        s.b[3089] = (s.v[2015] < 0.0);
        s.v[3089] = if s.b[3089] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && s.b[3087]) && (!s.b[3088])) && s.b[3089]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && s.b[3087]) && (!s.b[3088])) && (!s.b[3089])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && s.b[3087]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
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
        s.v[3090] = if s.b[3090] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && (!s.b[3087])) && s.b[3090]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3091] = ((-s.v[2011]) < 0.0);
        s.v[3091] = if s.b[3091] { 1.0 } else { 0.0 };

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
        s.v[3092] = if s.b[3092] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && (!s.b[3087])) && s.b[3092]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3093] = ((-s.v[2013]) < 0.0);
        s.v[3093] = if s.b[3093] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && (!s.b[3087])) && (!s.b[3092])) && s.b[3093]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && (!s.b[3087])) && (!s.b[3092])) && (!s.b[3093])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && (!s.b[3087])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2029, 2013, 2014);
        }

        if ((s.b[3067] && (!s.b[3068])) && s.b[3077]) {
            s.store_sub_ad_rhs(1980, 1890, A::add_scaled_inputs4(s.ad_value(1983), 0.125, s.ad_value(2028), (3.0 * 0.125), s.ad_value(2029), (3.0 * 0.125), s.ad_value(1984), 0.125));
        }

        s.b[3094] = (s.v[1] == 3.0);
        s.v[3094] = if s.b[3094] { 1.0 } else { 0.0 };

        if (((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) {
            s.store_add_scaled_inputs_ad_lhs(1981, A::add_scaled_inputs4(s.ad_value(1978), 251.0, s.ad_value(1969), 594.0, s.ad_value(1970), 312.0, s.ad_value(1971), 174.0), 0.0003720238095238095, 1979, (13.0 * 0.0003720238095238095));
            s.store_add_scaled_inputs_ad_lhs(1982, A::add_scaled_inputs4(s.ad_value(1979), 251.0, s.ad_value(1971), 594.0, s.ad_value(1970), 312.0, s.ad_value(1969), 174.0), 0.0003720238095238095, 1978, (13.0 * 0.0003720238095238095));
            s.store_add_div_lhs_indices(2027, 1969, 1937, 1890);
        }

        s.b[3095] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3095] = if s.b[3095] { 1.0 } else { 0.0 };

        if ((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && s.b[3095]) {
            s.store_div(2028, 2027, 1940);
        }

    }

    pub(super) fn stamp_transient_block_53(
        s: &mut Scratch,
    ) {
        s.b[3096] = (s.v[2027] < (-s.v[1941]));
        s.v[3096] = if s.b[3096] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && s.b[3096]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_products_mixed_aaia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3097] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3097] = if s.b[3097] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && s.b[3096]) && s.b[3097]) {
            s.store_exp(2005, 2015);
        }

        s.b[3098] = (s.v[2015] < 0.0);
        s.v[3098] = if s.b[3098] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && s.b[3096]) && (!s.b[3097])) && s.b[3098]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && s.b[3096]) && (!s.b[3097])) && (!s.b[3098])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && s.b[3096]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
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
        s.v[3099] = if s.b[3099] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && (!s.b[3096])) && s.b[3099]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3100] = ((-s.v[2011]) < 0.0);
        s.v[3100] = if s.b[3100] { 1.0 } else { 0.0 };

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
        s.v[3101] = if s.b[3101] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && (!s.b[3096])) && s.b[3101]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3102] = ((-s.v[2013]) < 0.0);
        s.v[3102] = if s.b[3102] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && (!s.b[3096])) && (!s.b[3101])) && s.b[3102]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && (!s.b[3096])) && (!s.b[3101])) && (!s.b[3102])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && (!s.b[3096])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2028, 2013, 2014);
        }

        if (((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) {
            s.store_add_div_lhs_indices(2027, 1970, 1937, 1890);
        }

        s.b[3103] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3103] = if s.b[3103] { 1.0 } else { 0.0 };

        if ((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && s.b[3103]) {
            s.store_div(2029, 2027, 1940);
        }

        s.b[3104] = (s.v[2027] < (-s.v[1941]));
        s.v[3104] = if s.b[3104] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && s.b[3104]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_products_mixed_aaia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3105] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3105] = if s.b[3105] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && s.b[3104]) && s.b[3105]) {
            s.store_exp(2005, 2015);
        }

        s.b[3106] = (s.v[2015] < 0.0);
        s.v[3106] = if s.b[3106] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && s.b[3104]) && (!s.b[3105])) && s.b[3106]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && s.b[3104]) && (!s.b[3105])) && (!s.b[3106])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && s.b[3104]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
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
        s.v[3107] = if s.b[3107] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && (!s.b[3104])) && s.b[3107]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3108] = ((-s.v[2011]) < 0.0);
        s.v[3108] = if s.b[3108] { 1.0 } else { 0.0 };

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
        s.v[3109] = if s.b[3109] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && (!s.b[3104])) && s.b[3109]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3110] = ((-s.v[2013]) < 0.0);
        s.v[3110] = if s.b[3110] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && (!s.b[3104])) && (!s.b[3109])) && s.b[3110]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && (!s.b[3104])) && (!s.b[3109])) && (!s.b[3110])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && (!s.b[3104])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2029, 2013, 2014);
        }

        if (((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) {
            s.store_add_div_lhs_indices(2027, 1971, 1937, 1890);
        }

        s.b[3111] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3111] = if s.b[3111] { 1.0 } else { 0.0 };

        if ((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && s.b[3111]) {
            s.store_div(2020, 2027, 1940);
        }

        s.b[3112] = (s.v[2027] < (-s.v[1941]));
        s.v[3112] = if s.b[3112] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && s.b[3112]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_products_mixed_aaia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3113] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3113] = if s.b[3113] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && s.b[3112]) && s.b[3113]) {
            s.store_exp(2005, 2015);
        }

        s.b[3114] = (s.v[2015] < 0.0);
        s.v[3114] = if s.b[3114] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && s.b[3112]) && (!s.b[3113])) && s.b[3114]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && s.b[3112]) && (!s.b[3113])) && (!s.b[3114])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && s.b[3112]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
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
        s.v[3115] = if s.b[3115] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && (!s.b[3112])) && s.b[3115]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3116] = ((-s.v[2011]) < 0.0);
        s.v[3116] = if s.b[3116] { 1.0 } else { 0.0 };

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
        s.v[3117] = if s.b[3117] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && (!s.b[3112])) && s.b[3117]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3118] = ((-s.v[2013]) < 0.0);
        s.v[3118] = if s.b[3118] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && (!s.b[3112])) && (!s.b[3117])) && s.b[3118]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && (!s.b[3112])) && (!s.b[3117])) && (!s.b[3118])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && (!s.b[3112])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2020, 2013, 2014);
        }

        if (((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) {
            s.store_sub_ad_rhs(1980, 1890, A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1983), 1.0, s.ad_value(2028), 4.0, s.ad_value(2029), 2.0, s.ad_value(2020), 4.0), 0.08333333333333333, s.ad_value(1984), 0.08333333333333333));
        }

        s.b[3119] = (s.v[1] == 5.0);
        s.v[3119] = if s.b[3119] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_54(
        s: &mut Scratch,
    ) {
        if ((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) {
            s.store_add_ad(1981, A::add_scaled_inputs(s.ad_value(1978), (1187.0 * 5.341880341880342e-5), s.ad_value(1979), (43.0 * 5.341880341880342e-5)), A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1969), 503.0, s.ad_value(1972), 172.0, s.ad_value(1973), 87.0, s.ad_value(1971), 265.0), 0.0003205128205128205, s.ad_value(1970), (328.0 * 0.0003205128205128205)));
            s.store_add_ad(1982, A::add_scaled_inputs(s.ad_value(1979), (1187.0 * 5.341880341880342e-5), s.ad_value(1978), (43.0 * 5.341880341880342e-5)), A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1973), 503.0, s.ad_value(1970), 172.0, s.ad_value(1969), 87.0, s.ad_value(1971), 265.0), 0.0003205128205128205, s.ad_value(1972), (328.0 * 0.0003205128205128205)));
            s.store_add_div_lhs_indices(2027, 1969, 1937, 1890);
        }

        s.b[3120] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3120] = if s.b[3120] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && s.b[3120]) {
            s.store_div(2028, 2027, 1940);
        }

        s.b[3121] = (s.v[2027] < (-s.v[1941]));
        s.v[3121] = if s.b[3121] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && s.b[3121]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_products_mixed_aaia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3122] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3122] = if s.b[3122] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && s.b[3121]) && s.b[3122]) {
            s.store_exp(2005, 2015);
        }

        s.b[3123] = (s.v[2015] < 0.0);
        s.v[3123] = if s.b[3123] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && s.b[3121]) && (!s.b[3122])) && s.b[3123]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && s.b[3121]) && (!s.b[3122])) && (!s.b[3123])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && s.b[3121]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
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
        s.v[3124] = if s.b[3124] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && (!s.b[3121])) && s.b[3124]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3125] = ((-s.v[2011]) < 0.0);
        s.v[3125] = if s.b[3125] { 1.0 } else { 0.0 };

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
        s.v[3126] = if s.b[3126] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && (!s.b[3121])) && s.b[3126]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3127] = ((-s.v[2013]) < 0.0);
        s.v[3127] = if s.b[3127] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && (!s.b[3121])) && (!s.b[3126])) && s.b[3127]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && (!s.b[3121])) && (!s.b[3126])) && (!s.b[3127])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && (!s.b[3121])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2028, 2013, 2014);
        }

        if ((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) {
            s.store_add_div_lhs_indices(2027, 1970, 1937, 1890);
        }

        s.b[3128] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3128] = if s.b[3128] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && s.b[3128]) {
            s.store_div(2029, 2027, 1940);
        }

        s.b[3129] = (s.v[2027] < (-s.v[1941]));
        s.v[3129] = if s.b[3129] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && s.b[3129]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_products_mixed_aaia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3130] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3130] = if s.b[3130] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && s.b[3129]) && s.b[3130]) {
            s.store_exp(2005, 2015);
        }

        s.b[3131] = (s.v[2015] < 0.0);
        s.v[3131] = if s.b[3131] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && s.b[3129]) && (!s.b[3130])) && s.b[3131]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && s.b[3129]) && (!s.b[3130])) && (!s.b[3131])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && s.b[3129]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
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
        s.v[3132] = if s.b[3132] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && (!s.b[3129])) && s.b[3132]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3133] = ((-s.v[2011]) < 0.0);
        s.v[3133] = if s.b[3133] { 1.0 } else { 0.0 };

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
        s.v[3134] = if s.b[3134] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && (!s.b[3129])) && s.b[3134]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3135] = ((-s.v[2013]) < 0.0);
        s.v[3135] = if s.b[3135] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && (!s.b[3129])) && (!s.b[3134])) && s.b[3135]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && (!s.b[3129])) && (!s.b[3134])) && (!s.b[3135])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && (!s.b[3129])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2029, 2013, 2014);
        }

        if ((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) {
            s.store_add_div_lhs_indices(2027, 1971, 1937, 1890);
        }

        s.b[3136] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3136] = if s.b[3136] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && s.b[3136]) {
            s.store_div(2020, 2027, 1940);
        }

        s.b[3137] = (s.v[2027] < (-s.v[1941]));
        s.v[3137] = if s.b[3137] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && s.b[3137]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_products_mixed_aaia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3138] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3138] = if s.b[3138] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && s.b[3137]) && s.b[3138]) {
            s.store_exp(2005, 2015);
        }

        s.b[3139] = (s.v[2015] < 0.0);
        s.v[3139] = if s.b[3139] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && s.b[3137]) && (!s.b[3138])) && s.b[3139]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && s.b[3137]) && (!s.b[3138])) && (!s.b[3139])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && s.b[3137]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
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
        s.v[3140] = if s.b[3140] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && (!s.b[3137])) && s.b[3140]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3141] = ((-s.v[2011]) < 0.0);
        s.v[3141] = if s.b[3141] { 1.0 } else { 0.0 };

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
        s.v[3142] = if s.b[3142] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && (!s.b[3137])) && s.b[3142]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3143] = ((-s.v[2013]) < 0.0);
        s.v[3143] = if s.b[3143] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && (!s.b[3137])) && (!s.b[3142])) && s.b[3143]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && (!s.b[3137])) && (!s.b[3142])) && (!s.b[3143])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && (!s.b[3137])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
        }

    }

    pub(super) fn stamp_transient_block_55(
        s: &mut Scratch,
    ) {
        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && (!s.b[3137])) {
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2020, 2013, 2014);
        }

        if ((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) {
            s.store_add_div_lhs_indices(2027, 1972, 1937, 1890);
        }

        s.b[3144] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3144] = if s.b[3144] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && s.b[3144]) {
            s.store_div(2021, 2027, 1940);
        }

        s.b[3145] = (s.v[2027] < (-s.v[1941]));
        s.v[3145] = if s.b[3145] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && s.b[3145]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_products_mixed_aaia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3146] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3146] = if s.b[3146] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && s.b[3145]) && s.b[3146]) {
            s.store_exp(2005, 2015);
        }

        s.b[3147] = (s.v[2015] < 0.0);
        s.v[3147] = if s.b[3147] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && s.b[3145]) && (!s.b[3146])) && s.b[3147]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && s.b[3145]) && (!s.b[3146])) && (!s.b[3147])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && s.b[3145]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
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
        s.v[3148] = if s.b[3148] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && (!s.b[3145])) && s.b[3148]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3149] = ((-s.v[2011]) < 0.0);
        s.v[3149] = if s.b[3149] { 1.0 } else { 0.0 };

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
        s.v[3150] = if s.b[3150] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && (!s.b[3145])) && s.b[3150]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3151] = ((-s.v[2013]) < 0.0);
        s.v[3151] = if s.b[3151] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && (!s.b[3145])) && (!s.b[3150])) && s.b[3151]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && (!s.b[3145])) && (!s.b[3150])) && (!s.b[3151])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && (!s.b[3145])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2021, 2013, 2014);
        }

        if ((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) {
            s.store_add_div_lhs_indices(2027, 1973, 1937, 1890);
        }

        s.b[3152] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3152] = if s.b[3152] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && s.b[3152]) {
            s.store_div(2022, 2027, 1940);
        }

        s.b[3153] = (s.v[2027] < (-s.v[1941]));
        s.v[3153] = if s.b[3153] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && s.b[3153]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_products_mixed_aaia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3154] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3154] = if s.b[3154] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && s.b[3153]) && s.b[3154]) {
            s.store_exp(2005, 2015);
        }

        s.b[3155] = (s.v[2015] < 0.0);
        s.v[3155] = if s.b[3155] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && s.b[3153]) && (!s.b[3154])) && s.b[3155]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && s.b[3153]) && (!s.b[3154])) && (!s.b[3155])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && s.b[3153]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
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
        s.v[3156] = if s.b[3156] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && (!s.b[3153])) && s.b[3156]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3157] = ((-s.v[2011]) < 0.0);
        s.v[3157] = if s.b[3157] { 1.0 } else { 0.0 };

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
        s.v[3158] = if s.b[3158] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && (!s.b[3153])) && s.b[3158]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3159] = ((-s.v[2013]) < 0.0);
        s.v[3159] = if s.b[3159] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && (!s.b[3153])) && (!s.b[3158])) && s.b[3159]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && (!s.b[3153])) && (!s.b[3158])) && (!s.b[3159])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && (!s.b[3153])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2022, 2013, 2014);
        }

        if ((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) {
            s.store_sub_ad_rhs(1980, 1890, A::add_scaled_inputs(A::add(A::add_scaled_inputs4(s.ad_value(1983), 1.0, s.ad_value(2028), 4.0, s.ad_value(2020), 4.0, s.ad_value(2022), 4.0), A::add_scaled_inputs(s.ad_value(2029), 2.0, s.ad_value(2021), 2.0)), 0.05555555555555555, s.ad_value(1984), 0.05555555555555555));
        }

        s.b[3160] = (s.v[1] == 9.0);
        s.v[3160] = if s.b[3160] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {
            let assign78550_ad_e112431: A = A::add_scaled_inputs(A::add(A::add_scaled_inputs(s.ad_value(1976), (75653.0 * 2.6434745829918845e-7), s.ad_value(1972), (225999.0 * 2.6434745829918845e-7)), A::add_scaled_inputs4(s.ad_value(1977), (151321.0 * 6.608686457479711e-8), s.ad_value(1975), (454023.0 * 6.608686457479711e-8), s.ad_value(1971), (1073767.0 * 6.608686457479711e-8), s.ad_value(1969), (1564569.0 * 6.608686457479711e-8))), 1.0, s.ad_value(1974), (75623.0 * 5.286949165983769e-7));
            s.store_add_ad(1981, A::add_scaled_inputs3(assign78550_ad_e112431, 1.0, s.ad_value(1973), (145.0 * 0.0003453038674033149), s.ad_value(1970), (72263.0 * 1.0573898331967538e-6)), A::add_scaled_inputs(s.ad_value(1978), (3504517.0 * 1.1014477429132853e-8), s.ad_value(1979), (75653.0 * 1.1014477429132853e-8)));
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {
            let assign78560_ad_e112503: A = A::add_scaled_inputs(A::add(A::add_scaled_inputs(s.ad_value(1970), (75653.0 * 2.6434745829918845e-7), s.ad_value(1974), (225999.0 * 2.6434745829918845e-7)), A::add_scaled_inputs4(s.ad_value(1969), (151321.0 * 6.608686457479711e-8), s.ad_value(1971), (454023.0 * 6.608686457479711e-8), s.ad_value(1975), (1073767.0 * 6.608686457479711e-8), s.ad_value(1977), (1564569.0 * 6.608686457479711e-8))), 1.0, s.ad_value(1972), (75623.0 * 5.286949165983769e-7));
            s.store_add_ad(1982, A::add_scaled_inputs3(assign78560_ad_e112503, 1.0, s.ad_value(1973), (145.0 * 0.0003453038674033149), s.ad_value(1976), (72263.0 * 1.0573898331967538e-6)), A::add_scaled_inputs(s.ad_value(1979), (3504517.0 * 1.1014477429132853e-8), s.ad_value(1978), (75653.0 * 1.1014477429132853e-8)));
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {
            s.store_add_div_lhs_indices(2027, 1969, 1937, 1890);
        }

        s.b[3161] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3161] = if s.b[3161] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && s.b[3161]) {
            s.store_div(2028, 2027, 1940);
        }

        s.b[3162] = (s.v[2027] < (-s.v[1941]));
        s.v[3162] = if s.b[3162] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && s.b[3162]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_products_mixed_aaia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3163] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3163] = if s.b[3163] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && s.b[3162]) && s.b[3163]) {
            s.store_exp(2005, 2015);
        }

        s.b[3164] = (s.v[2015] < 0.0);
        s.v[3164] = if s.b[3164] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && s.b[3162]) && (!s.b[3163])) && s.b[3164]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && s.b[3162]) && (!s.b[3163])) && (!s.b[3164])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && s.b[3162]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
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
        s.v[3165] = if s.b[3165] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && (!s.b[3162])) && s.b[3165]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3166] = ((-s.v[2011]) < 0.0);
        s.v[3166] = if s.b[3166] { 1.0 } else { 0.0 };

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
        s.v[3167] = if s.b[3167] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && (!s.b[3162])) && s.b[3167]) {
            s.store_exp_neg_input(2005, 2013);
        }

    }

    pub(super) fn stamp_transient_block_56(
        s: &mut Scratch,
    ) {
        s.b[3168] = ((-s.v[2013]) < 0.0);
        s.v[3168] = if s.b[3168] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && (!s.b[3162])) && (!s.b[3167])) && s.b[3168]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && (!s.b[3162])) && (!s.b[3167])) && (!s.b[3168])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && (!s.b[3162])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2028, 2013, 2014);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {
            s.store_add_div_lhs_indices(2027, 1970, 1937, 1890);
        }

        s.b[3169] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3169] = if s.b[3169] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && s.b[3169]) {
            s.store_div(2029, 2027, 1940);
        }

        s.b[3170] = (s.v[2027] < (-s.v[1941]));
        s.v[3170] = if s.b[3170] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && s.b[3170]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_products_mixed_aaia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3171] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3171] = if s.b[3171] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && s.b[3170]) && s.b[3171]) {
            s.store_exp(2005, 2015);
        }

        s.b[3172] = (s.v[2015] < 0.0);
        s.v[3172] = if s.b[3172] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && s.b[3170]) && (!s.b[3171])) && s.b[3172]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && s.b[3170]) && (!s.b[3171])) && (!s.b[3172])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && s.b[3170]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
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
        s.v[3173] = if s.b[3173] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && (!s.b[3170])) && s.b[3173]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3174] = ((-s.v[2011]) < 0.0);
        s.v[3174] = if s.b[3174] { 1.0 } else { 0.0 };

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
        s.v[3175] = if s.b[3175] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && (!s.b[3170])) && s.b[3175]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3176] = ((-s.v[2013]) < 0.0);
        s.v[3176] = if s.b[3176] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && (!s.b[3170])) && (!s.b[3175])) && s.b[3176]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && (!s.b[3170])) && (!s.b[3175])) && (!s.b[3176])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && (!s.b[3170])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2029, 2013, 2014);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {
            s.store_add_div_lhs_indices(2027, 1971, 1937, 1890);
        }

        s.b[3177] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3177] = if s.b[3177] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && s.b[3177]) {
            s.store_div(2020, 2027, 1940);
        }

        s.b[3178] = (s.v[2027] < (-s.v[1941]));
        s.v[3178] = if s.b[3178] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && s.b[3178]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_products_mixed_aaia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3179] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3179] = if s.b[3179] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && s.b[3178]) && s.b[3179]) {
            s.store_exp(2005, 2015);
        }

        s.b[3180] = (s.v[2015] < 0.0);
        s.v[3180] = if s.b[3180] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && s.b[3178]) && (!s.b[3179])) && s.b[3180]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && s.b[3178]) && (!s.b[3179])) && (!s.b[3180])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && s.b[3178]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
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
        s.v[3181] = if s.b[3181] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && (!s.b[3178])) && s.b[3181]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3182] = ((-s.v[2011]) < 0.0);
        s.v[3182] = if s.b[3182] { 1.0 } else { 0.0 };

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
        s.v[3183] = if s.b[3183] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && (!s.b[3178])) && s.b[3183]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3184] = ((-s.v[2013]) < 0.0);
        s.v[3184] = if s.b[3184] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && (!s.b[3178])) && (!s.b[3183])) && s.b[3184]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && (!s.b[3178])) && (!s.b[3183])) && (!s.b[3184])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && (!s.b[3178])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2020, 2013, 2014);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {
            s.store_add_div_lhs_indices(2027, 1972, 1937, 1890);
        }

        s.b[3185] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3185] = if s.b[3185] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && s.b[3185]) {
            s.store_div(2021, 2027, 1940);
        }

        s.b[3186] = (s.v[2027] < (-s.v[1941]));
        s.v[3186] = if s.b[3186] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && s.b[3186]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_products_mixed_aaia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3187] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3187] = if s.b[3187] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && s.b[3186]) && s.b[3187]) {
            s.store_exp(2005, 2015);
        }

        s.b[3188] = (s.v[2015] < 0.0);
        s.v[3188] = if s.b[3188] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && s.b[3186]) && (!s.b[3187])) && s.b[3188]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && s.b[3186]) && (!s.b[3187])) && (!s.b[3188])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && s.b[3186]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
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
        s.v[3189] = if s.b[3189] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) && s.b[3189]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3190] = ((-s.v[2011]) < 0.0);
        s.v[3190] = if s.b[3190] { 1.0 } else { 0.0 };

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
        s.v[3191] = if s.b[3191] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_57(
        s: &mut Scratch,
    ) {
        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) && s.b[3191]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3192] = ((-s.v[2013]) < 0.0);
        s.v[3192] = if s.b[3192] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) && (!s.b[3191])) && s.b[3192]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) && (!s.b[3191])) && (!s.b[3192])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2021, 2013, 2014);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {
            s.store_add_div_lhs_indices(2027, 1973, 1937, 1890);
        }

        s.b[3193] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3193] = if s.b[3193] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && s.b[3193]) {
            s.store_div(2022, 2027, 1940);
        }

        s.b[3194] = (s.v[2027] < (-s.v[1941]));
        s.v[3194] = if s.b[3194] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && s.b[3194]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_products_mixed_aaia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3195] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3195] = if s.b[3195] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && s.b[3194]) && s.b[3195]) {
            s.store_exp(2005, 2015);
        }

        s.b[3196] = (s.v[2015] < 0.0);
        s.v[3196] = if s.b[3196] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && s.b[3194]) && (!s.b[3195])) && s.b[3196]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && s.b[3194]) && (!s.b[3195])) && (!s.b[3196])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && s.b[3194]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
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
        s.v[3197] = if s.b[3197] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && (!s.b[3194])) && s.b[3197]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3198] = ((-s.v[2011]) < 0.0);
        s.v[3198] = if s.b[3198] { 1.0 } else { 0.0 };

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
        s.v[3199] = if s.b[3199] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && (!s.b[3194])) && s.b[3199]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3200] = ((-s.v[2013]) < 0.0);
        s.v[3200] = if s.b[3200] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && (!s.b[3194])) && (!s.b[3199])) && s.b[3200]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && (!s.b[3194])) && (!s.b[3199])) && (!s.b[3200])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && (!s.b[3194])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2022, 2013, 2014);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {
            s.store_add_div_lhs_indices(2027, 1974, 1937, 1890);
        }

        s.b[3201] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3201] = if s.b[3201] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && s.b[3201]) {
            s.store_div(2023, 2027, 1940);
        }

        s.b[3202] = (s.v[2027] < (-s.v[1941]));
        s.v[3202] = if s.b[3202] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && s.b[3202]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_products_mixed_aaia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3203] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3203] = if s.b[3203] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && s.b[3202]) && s.b[3203]) {
            s.store_exp(2005, 2015);
        }

        s.b[3204] = (s.v[2015] < 0.0);
        s.v[3204] = if s.b[3204] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && s.b[3202]) && (!s.b[3203])) && s.b[3204]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && s.b[3202]) && (!s.b[3203])) && (!s.b[3204])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && s.b[3202]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
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
        s.v[3205] = if s.b[3205] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && (!s.b[3202])) && s.b[3205]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3206] = ((-s.v[2011]) < 0.0);
        s.v[3206] = if s.b[3206] { 1.0 } else { 0.0 };

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
        s.v[3207] = if s.b[3207] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && (!s.b[3202])) && s.b[3207]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3208] = ((-s.v[2013]) < 0.0);
        s.v[3208] = if s.b[3208] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && (!s.b[3202])) && (!s.b[3207])) && s.b[3208]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && (!s.b[3202])) && (!s.b[3207])) && (!s.b[3208])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && (!s.b[3202])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2023, 2013, 2014);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {
            s.store_add_div_lhs_indices(2027, 1975, 1937, 1890);
        }

        s.b[3209] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3209] = if s.b[3209] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && s.b[3209]) {
            s.store_div(2024, 2027, 1940);
        }

        s.b[3210] = (s.v[2027] < (-s.v[1941]));
        s.v[3210] = if s.b[3210] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && s.b[3210]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_products_mixed_aaia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3211] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3211] = if s.b[3211] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && s.b[3210]) && s.b[3211]) {
            s.store_exp(2005, 2015);
        }

        s.b[3212] = (s.v[2015] < 0.0);
        s.v[3212] = if s.b[3212] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && s.b[3210]) && (!s.b[3211])) && s.b[3212]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && s.b[3210]) && (!s.b[3211])) && (!s.b[3212])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && s.b[3210]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
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
        s.v[3213] = if s.b[3213] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && (!s.b[3210])) && s.b[3213]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3214] = ((-s.v[2011]) < 0.0);
        s.v[3214] = if s.b[3214] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && (!s.b[3210])) && (!s.b[3213])) && s.b[3214]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && (!s.b[3210])) && (!s.b[3213])) && (!s.b[3214])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && (!s.b[3210])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
        }

    }

    pub(super) fn stamp_transient_block_58(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && (!s.b[3210])) {
            s.store_add_scaled_inputs_product_right_ad(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));
        }

        s.b[3215] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[3215] = if s.b[3215] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && (!s.b[3210])) && s.b[3215]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3216] = ((-s.v[2013]) < 0.0);
        s.v[3216] = if s.b[3216] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && (!s.b[3210])) && (!s.b[3215])) && s.b[3216]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && (!s.b[3210])) && (!s.b[3215])) && (!s.b[3216])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && (!s.b[3210])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2024, 2013, 2014);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {
            s.store_add_div_lhs_indices(2027, 1976, 1937, 1890);
        }

        s.b[3217] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3217] = if s.b[3217] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && s.b[3217]) {
            s.store_div(2025, 2027, 1940);
        }

        s.b[3218] = (s.v[2027] < (-s.v[1941]));
        s.v[3218] = if s.b[3218] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && s.b[3218]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_products_mixed_aaia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3219] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3219] = if s.b[3219] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && s.b[3218]) && s.b[3219]) {
            s.store_exp(2005, 2015);
        }

        s.b[3220] = (s.v[2015] < 0.0);
        s.v[3220] = if s.b[3220] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && s.b[3218]) && (!s.b[3219])) && s.b[3220]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && s.b[3218]) && (!s.b[3219])) && (!s.b[3220])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && s.b[3218]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
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
        s.v[3221] = if s.b[3221] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && (!s.b[3218])) && s.b[3221]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3222] = ((-s.v[2011]) < 0.0);
        s.v[3222] = if s.b[3222] { 1.0 } else { 0.0 };

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
        s.v[3223] = if s.b[3223] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && (!s.b[3218])) && s.b[3223]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3224] = ((-s.v[2013]) < 0.0);
        s.v[3224] = if s.b[3224] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && (!s.b[3218])) && (!s.b[3223])) && s.b[3224]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && (!s.b[3218])) && (!s.b[3223])) && (!s.b[3224])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && (!s.b[3218])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
            s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));
            s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);
            s.store_add(2025, 2013, 2014);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {
            s.store_add_div_lhs_indices(2027, 1977, 1937, 1890);
        }

        s.b[3225] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3225] = if s.b[3225] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && s.b[3225]) {
            s.store_div(2026, 2027, 1940);
        }

        s.b[3226] = (s.v[2027] < (-s.v[1941]));
        s.v[3226] = if s.b[3226] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && s.b[3226]) {
            s.store_neg(1999, 2027);
            s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);
            s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);
            s.store_add_scaled_products_mixed_aaia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);
            s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);
            s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[3227] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3227] = if s.b[3227] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && s.b[3226]) && s.b[3227]) {
            s.store_exp(2005, 2015);
        }

        s.b[3228] = (s.v[2015] < 0.0);
        s.v[3228] = if s.b[3228] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && s.b[3226]) && (!s.b[3227])) && s.b[3228]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && s.b[3226]) && (!s.b[3227])) && (!s.b[3228])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && s.b[3226]) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs_product_right_ad(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);
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
        s.v[3229] = if s.b[3229] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && (!s.b[3226])) && s.b[3229]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3230] = ((-s.v[2011]) < 0.0);
        s.v[3230] = if s.b[3230] { 1.0 } else { 0.0 };

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
        s.v[3231] = if s.b[3231] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && (!s.b[3226])) && s.b[3231]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3232] = ((-s.v[2013]) < 0.0);
        s.v[3232] = if s.b[3232] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && (!s.b[3226])) && (!s.b[3231])) && s.b[3232]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && (!s.b[3226])) && (!s.b[3231])) && (!s.b[3232])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && (!s.b[3226])) {
            s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);
            s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);
            s.store_add_scaled_products_mixed_aaia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));
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
        s.v[3233] = if s.b[3233] { 1.0 } else { 0.0 };

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

        s.store_neg_ad(850, A::add_scaled_inputs3(s.ad_value(851), 1.0, s.ad_value(852), 1.0, s.ad_value(853), 1.0));

        s.store_add(854, 854, 1910);

        s.store_add(855, 855, 1911);

        s.store_add_scaled_products3(857, s.ad_value(646), s.ad_value(1918), 1.0, s.ad_value(647), s.ad_value(1919), 1.0, s.ad_value(648), s.ad_value(1920), 1.0);

        s.store_add_scaled_products3(858, s.ad_value(673), s.ad_value(1921), 1.0, s.ad_value(674), s.ad_value(1922), 1.0, s.ad_value(675), s.ad_value(1923), 1.0);

        s.b[3235] = (s.v[831] < 0.0);
        s.v[3235] = if s.b[3235] { 1.0 } else { 0.0 };

        if s.b[3235] {
            s.copy_ad(3234, 853);
            s.copy_ad(853, 850);
            s.copy_ad(850, 3234);
        }

        s.v[3252] = 0.0;

        s.v[3247] = 0.0;

        s.v[859] = 1e-40;

        s.v[861] = 0.0;

        s.v[863] = 0.0;

        s.store_mul(860, 1904, 1895);

        s.v[862] = 0.0;

        s.v[3254] = 0.0;

        s.b[3268] = ((s.v[1829] > 0.0) && (s.v[716] > 0.0));
        s.v[3268] = if s.b[3268] { 1.0 } else { 0.0 };

        s.b[3270] = (p.p32 > 0.0);
        s.v[3270] = if s.b[3270] { 1.0 } else { 0.0 };

        if (s.b[3268] && s.b[3270]) {
            s.store_div(3239, 1866, 1864);
            s.store_div(3240, 1865, 1866);
            s.store_scaled_div(3241, 1860, 3239, (0.5 * 0.16666666666666666));
            s.store_square(3242, 3241);
            s.store_offset_div(3243, 3239, 1877, (-1.0));
        }

        if (s.b[3268] && s.b[3270]) {
            if ((1.0 - (12.0 * (s.v[3243] * s.v[3242]))) > 1e-20) {
                s.store_sub_from_scalar_scaled_mul(3244, 1.0, 3243, 3242, 12.0);
            } else {
                s.store_scalar(3244, 1e-20);
            }
        }

        if (s.b[3268] && s.b[3270]) {
            s.store_div_from_scalar_square_ad(3245, 1.0, s.ad_value(3244));
            s.store_mul3_lhs(3246, 716, 1866, 1876);
        }

    }

    pub(super) fn stamp_transient_block_59(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[3268] && s.b[3270]) {
            s.store_add_scaled_inputs3_mixed_iia(3247, 3240, 1.0, 3242, 12.0, A::mul3_scaled_output(A::offset(s.ad_value(3240), 1.0), s.ad_value(3242), s.ad_value(3243), 24.0), -1.0);
        }

        if (s.b[3268] && s.b[3270]) {
            if (s.v[3247] > 1e-40) {
            } else {
                s.store_scalar(3247, 1e-40);
            }
        }

        if (s.b[3268] && s.b[3270]) {
            s.store_mul3_lhs(3247, 3246, 3245, 3247);
        }

        s.b[3271] = (s.v[277] > 0.0);
        s.v[3271] = if s.b[3271] { 1.0 } else { 0.0 };

        if ((s.b[3268] && s.b[3270]) && s.b[3271]) {
            s.store_div(3248, 1870, 1869);
            s.store_mul_ad_product_lhs(3249, A::square(s.ad_value(3248)), s.ad_value(1860), 1860);
        }

        s.b[3272] = (s.v[0] == (-1.0));
        s.v[3272] = if s.b[3272] { 1.0 } else { 0.0 };

        if (((s.b[3268] && s.b[3270]) && s.b[3271]) && s.b[3272]) {
            s.store_div_scaled_value_offset_denominator(3249, s.ad_value(3249), 1.0, A::mul(s.ad_value(3248), s.ad_value(1860)), 1.0, 1.0);
        }

        if ((s.b[3268] && s.b[3270]) && s.b[3271]) {
            s.store_mul_offset_rhs_scaled_ad_rhs(3250, 1869, A::sqrt(A::scale_offset(s.ad_value(3249), 2.0, 1.0)), 1.0, 0.5);
            s.store_div_ad_rhs(3251, 1869, A::mul(s.ad_value(3250), s.ad_value(3244)));
            s.store_mul_ad_product_lhs(3252, A::mul3(s.ad_value(810), s.ad_value(838), s.ad_value(1857)), s.ad_value(3251), 3251);
            s.store_add_scaled_inputs(3247, 3247, 1.0, 3252, 1.0 / (s.v[718]));
        }

        if (s.b[3268] && s.b[3270]) {
            s.store_sqrt_mul(862, 719, 3247);
        }

        s.b[3273] = ((((p.p50 == 1.0) && (s.v[719] > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0));
        s.v[3273] = if s.b[3273] { 1.0 } else { 0.0 };

        if (s.b[3268] && s.b[3273]) {
            s.store_sub_ad(859, A::add_scaled_product(s.ad_value(3240), 0.08333333333333333, s.ad_value(3242), A::sub_scaled_inputs(A::offset(s.ad_value(3240), 0.2), 1.0, s.ad_value(3242), 12.0), (-1.0)), A::mul3_scaled_output(s.ad_value(3242), A::sub_scaled_inputs(A::offset(s.ad_value(3240), 1.0), 1.0, s.ad_value(3242), 12.0), s.ad_value(3243), 1.6));
        }

        if (s.b[3268] && s.b[3273]) {
            if (s.v[859] > 1e-40) {
            } else {
                s.store_scalar(859, 1e-40);
            }
        }

        if (s.b[3268] && s.b[3273]) {
            s.store_mul_div_lhs(859, 3245, 3246, 859);
            s.store_mul_ad_product_rhs(3253, 3245, s.ad_value(3241), A::add_scaled_sub_value_product(1.0, A::scale(s.ad_value(3242), 12.0), 1.0, A::add_scaled_inputs_product(s.ad_value(3240), 1.0, s.ad_value(3242), 19.2, s.ad_value(3240), s.ad_value(3242), (-12.0)), s.ad_value(3243), (-1.0)));
            s.store_div_scaled_product3_mixed_aiia(860, A::square(s.ad_value(1908)), 1904, 1895, 1.0, A::square(s.ad_value(1906)), 1.0);
        }

        s.b[3274] = (s.v[277] > 0.0);
        s.v[3274] = if s.b[3274] { 1.0 } else { 0.0 };

        if ((s.b[3268] && s.b[3273]) && s.b[3274]) {
            s.store_add_ad_rhs(859, 859, A::div_scaled_product_by_product(s.ad_value(3252), A::scale_offset(s.ad_value(3242), 12.0, 1.0), 1.0, s.ad_value(3246), s.ad_value(3246), (12.0 * s.v[718])));
            s.store_sub_ad_rhs(3253, 3253, A::div_scaled_product3(s.ad_value(3252), s.ad_value(3241), A::offset(s.ad_value(3243), 1.0), 1.0, s.ad_value(3246), s.v[718]));
        }

        if (s.b[3268] && s.b[3273]) {
            s.store_sqrt_div(3254, 719, 859);
        }

        s.b[3275] = (s.v[862] <= 0.0);
        s.v[3275] = if s.b[3275] { 1.0 } else { 0.0 };

        if ((s.b[3268] && s.b[3273]) && s.b[3275]) {
            s.store_scalar(863, 0.0);
        }

        if ((s.b[3268] && s.b[3273]) && (!s.b[3275])) {
            s.store_div_scaled_product_indices(863, 3253, 3254, 1.0, 862, 1.0);
        }

        if (s.b[3268] && s.b[3273]) {
            if (s.v[863] > 0.0) {
                if (s.v[863] < 1.0) {
                } else {
                    s.store_scalar(863, 1.0);
                }
            } else {
                s.store_scalar(863, 0.0);
            }
        }

        if (s.b[3268] && s.b[3273]) {
            s.store_div_scaled_product_indices(861, 863, 862, 1.0, 3254, 1.0);
        }

        s.b[3277] = (((p.p46 != 0.0) && (s.v[287] > 0.0)) && (s.v[1880] > 0.0));
        s.v[3277] = if s.b[3277] { 1.0 } else { 0.0 };

        if s.b[3277] {
            s.store_div_scaled_inputs_indices(2028, 1883, 4.0, 724, 1.0);
            s.store_scale(2028, 771, s.v[715]);
            s.store_mul(2028, 1864, 1877);
        }

    }

    pub(super) fn stamp_reactive_block_0(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[990] = (p.p37 >= 0.0);
        s.v[990] = if s.b[990] { 1.0 } else { 0.0 };

        if s.b[990] {
            s.store_scalar(0, 1.0);
        }

        if (!s.b[990]) {
            s.store_scalar(0, (-1.0));
        }

        s.v[767] = (8.8541878176e-12 * 11.8);

        s.b[991] = (p.p51 < 0.5);
        s.v[991] = if s.b[991] { 1.0 } else { 0.0 };

        if s.b[991] {
            s.store_scalar(1, 0.0);
        }

        s.b[992] = (p.p51 < 1.5);
        s.v[992] = if s.b[992] { 1.0 } else { 0.0 };

        if ((!s.b[991]) && s.b[992]) {
            s.store_scalar(1, 1.0);
        }

        s.b[993] = (p.p51 < 2.5);
        s.v[993] = if s.b[993] { 1.0 } else { 0.0 };

        if (((!s.b[991]) && (!s.b[992])) && s.b[993]) {
            s.store_scalar(1, 2.0);
        }

        s.b[994] = (p.p51 < 4.0);
        s.v[994] = if s.b[994] { 1.0 } else { 0.0 };

        if ((((!s.b[991]) && (!s.b[992])) && (!s.b[993])) && s.b[994]) {
            s.store_scalar(1, 3.0);
        }

        s.b[995] = (p.p51 < 7.0);
        s.v[995] = if s.b[995] { 1.0 } else { 0.0 };

        if (((((!s.b[991]) && (!s.b[992])) && (!s.b[993])) && (!s.b[994])) && s.b[995]) {
            s.store_scalar(1, 5.0);
        }

        if (((((!s.b[991]) && (!s.b[992])) && (!s.b[993])) && (!s.b[994])) && (!s.b[995])) {
            s.store_scalar(1, 9.0);
        }

        s.v[3] = 10.0;

        s.v[4] = (1.0 / s.v[3]);

        s.v[350] = (273.15 + p.p38);

        s.v[474] = 0.0;

        s.b[996] = (p.p927 > 0.5);
        s.v[996] = if s.b[996] { 1.0 } else { 0.0 };

        if s.b[996] {
            s.store_scalar(474, 1.0);
        }

        if (!s.b[996]) {
            s.store_scalar(474, 0.0);
        }

        s.v[364] = (273.15 + p.p823);

        s.v[367] = (1.3806505e-23 / 1.6021918e-19);

        s.v[368] = (s.v[367] * s.v[364]);

        s.v[369] = (1.0 / s.v[368]);

        s.v[375] = ((-((0.000702 * s.v[364]) * s.v[364])) / (1108.0 + s.v[364]));

        s.v[378] = (p.p834 + s.v[375]);

        s.v[379] = (p.p835 + s.v[375]);

        s.v[380] = (p.p836 + s.v[375]);

        s.v[408] = (1.0 - p.p831);

        s.v[409] = (1.0 - p.p832);

        s.v[410] = (1.0 - p.p833);

        s.v[411] = (1.0 / s.v[408]);

        s.v[412] = (1.0 / s.v[409]);

        s.v[413] = (1.0 / s.v[410]);

        s.v[423] = (s.v[767] / p.p825);

        s.v[424] = ((p.p843 * s.v[767]) / p.p826);

        s.v[425] = ((p.p844 * s.v[767]) / p.p827);

        s.v[426] = (1.0 / s.v[423]);

        s.v[427] = (1.0 / s.v[424]);

        s.v[428] = (1.0 / s.v[425]);

        s.v[429] = (1.0 / p.p828);

        s.v[430] = (1.0 / p.p829);

        s.v[431] = (1.0 / p.p830);

        s.v[444] = (1.0 - (1.0 / p.p824));

        s.v[448] = (1.0 / p.p860);

        s.v[449] = (1.0 / p.p861);

        s.v[450] = (1.0 / p.p862);

        s.b[997] = ((((p.p866 != 1.0) || (p.p867 != 1.0)) || (p.p868 != 1.0)) || (p.p869 != 1.0));
        s.v[997] = if s.b[997] { 1.0 } else { 0.0 };

        if s.b[997] {
            s.store_scalar(473, 1.0);
        }

        if (!s.b[997]) {
            s.store_scalar(473, 0.0);
        }

        s.b[998] = (s.v[473] == 1.0);
        s.v[998] = if s.b[998] { 1.0 } else { 0.0 };

        if s.b[998] {
            s.store_scalar(457, (if ((p.p827 * p.p866) > 1e-18) { (p.p827 * p.p866) } else { 1e-18 }));
        }

        if s.b[998] {
            s.store_scalar(458, (if ((p.p830 * p.p867) > 0.05) { (p.p830 * p.p867) } else { 0.05 }));
        }

        if s.b[998] {
            s.store_scalar(459, (if ((if ((p.p833 * p.p868) > 0.05) { (p.p833 * p.p868) } else { 0.05 }) < 0.95) { (if ((p.p833 * p.p868) > 0.05) { (p.p833 * p.p868) } else { 0.05 }) } else { 0.95 }));
        }

        if s.b[998] {
            s.store_scalar(460, (p.p836 * p.p869));
            s.store_offset(462, 460, s.v[375]);
            s.store_sub_from_scalar(467, 1.0, 459);
            s.store_div_from_scalar(468, 1.0, 467);
        }

        s.b[999] = (p.p44 == 0.0);
        s.v[999] = if s.b[999] { 1.0 } else { 0.0 };

        if s.b[999] {
            s.store_scalar(505, p.p825);
            s.store_scalar(506, p.p826);
            s.store_scalar(507, p.p827);
            s.store_scalar(508, p.p828);
            s.store_scalar(509, p.p829);
            s.store_scalar(510, p.p830);
            s.store_scalar(511, p.p831);
            s.store_scalar(512, p.p832);
            s.store_scalar(513, p.p833);
            s.store_scalar(514, p.p834);
            s.store_scalar(515, p.p835);
            s.store_scalar(516, p.p836);
            s.store_scalar(517, p.p837);
            s.store_scalar(518, p.p838);
            s.store_scalar(519, p.p839);
            s.store_scalar(522, p.p840);
            s.store_scalar(523, p.p841);
            s.store_scalar(524, p.p842);
            s.store_scalar(520, p.p843);
            s.store_scalar(521, p.p844);
            s.store_scalar(525, p.p845);
            s.store_scalar(526, p.p846);
            s.store_scalar(527, p.p847);
            s.store_scalar(528, p.p848);
            s.store_scalar(529, p.p849);
            s.store_scalar(530, p.p850);
            s.store_scalar(531, p.p851);
            s.store_scalar(532, p.p852);
            s.store_scalar(533, p.p853);
            s.store_scalar(534, p.p854);
            s.store_scalar(535, p.p855);
            s.store_scalar(536, p.p856);
            s.store_scalar(537, p.p857);
            s.store_scalar(538, p.p858);
            s.store_scalar(539, p.p859);
            s.store_scalar(540, p.p860);
            s.store_scalar(541, p.p861);
            s.store_scalar(542, p.p862);
            s.store_scalar(543, p.p863);
            s.store_scalar(544, p.p864);
            s.store_scalar(545, p.p865);
            s.store_scalar(553, p.p929);
            s.store_scalar(636, p.p872);
            s.store_scalar(637, p.p873);
            s.store_scalar(638, p.p874);
            s.store_scalar(639, p.p875);
            s.store_scalar(546, p.p866);
            s.store_scalar(547, p.p867);
            s.store_scalar(548, p.p868);
            s.store_scalar(549, p.p869);
            s.store_scalar(550, p.p870);
            s.store_scalar(551, p.p871);
        }

        if (!s.b[999]) {
            s.store_scalar(505, p.p876);
            s.store_scalar(506, p.p877);
            s.store_scalar(507, p.p878);
            s.store_scalar(508, p.p879);
            s.store_scalar(509, p.p880);
            s.store_scalar(510, p.p881);
            s.store_scalar(511, p.p882);
            s.store_scalar(512, p.p883);
            s.store_scalar(513, p.p884);
            s.store_scalar(514, p.p885);
            s.store_scalar(515, p.p886);
            s.store_scalar(516, p.p887);
            s.store_scalar(517, p.p888);
            s.store_scalar(518, p.p889);
            s.store_scalar(519, p.p890);
            s.store_scalar(522, p.p891);
            s.store_scalar(523, p.p892);
            s.store_scalar(524, p.p893);
            s.store_scalar(520, p.p894);
            s.store_scalar(521, p.p895);
            s.store_scalar(525, p.p896);
            s.store_scalar(526, p.p897);
            s.store_scalar(527, p.p898);
            s.store_scalar(528, p.p899);
            s.store_scalar(529, p.p900);
            s.store_scalar(530, p.p901);
            s.store_scalar(531, p.p902);
            s.store_scalar(532, p.p903);
            s.store_scalar(533, p.p904);
            s.store_scalar(534, p.p905);
            s.store_scalar(535, p.p906);
            s.store_scalar(536, p.p907);
            s.store_scalar(537, p.p908);
        }

    }

    pub(super) fn stamp_reactive_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if (!s.b[999]) {
            s.store_scalar(538, p.p909);
            s.store_scalar(539, p.p910);
            s.store_scalar(540, p.p911);
            s.store_scalar(541, p.p912);
            s.store_scalar(542, p.p913);
            s.store_scalar(543, p.p914);
            s.store_scalar(544, p.p915);
            s.store_scalar(545, p.p916);
            s.store_scalar(553, p.p931);
            s.store_scalar(636, p.p923);
            s.store_scalar(637, p.p924);
            s.store_scalar(638, p.p925);
            s.store_scalar(639, p.p926);
            s.store_scalar(546, p.p917);
            s.store_scalar(547, p.p918);
            s.store_scalar(548, p.p919);
            s.store_scalar(549, p.p920);
            s.store_scalar(550, p.p921);
            s.store_scalar(551, p.p922);
        }

        s.store_offset(554, 514, s.v[375]);

        s.store_offset(555, 515, s.v[375]);

        s.store_offset(556, 516, s.v[375]);

        s.store_sub_from_scalar(575, 1.0, 511);

        s.store_sub_from_scalar(576, 1.0, 512);

        s.store_sub_from_scalar(577, 1.0, 513);

        s.store_div_from_scalar(578, 1.0, 575);

        s.store_div_from_scalar(579, 1.0, 576);

        s.store_div_from_scalar(580, 1.0, 577);

        s.store_div_from_scalar(590, s.v[767], 505);

        s.store_div_scaled_inputs_indices(591, 520, s.v[767], 506, 1.0);

        s.store_div_scaled_inputs_indices(592, 521, s.v[767], 507, 1.0);

        s.store_div_from_scalar(593, 1.0, 590);

        s.store_div_from_scalar(594, 1.0, 591);

        s.store_div_from_scalar(595, 1.0, 592);

        s.store_div_from_scalar(596, 1.0, 508);

        s.store_div_from_scalar(597, 1.0, 509);

        s.store_div_from_scalar(598, 1.0, 510);

        s.store_div_from_scalar(614, 1.0, 540);

        s.store_div_from_scalar(615, 1.0, 541);

        s.store_div_from_scalar(616, 1.0, 542);

        s.b[1000] = ((((s.v[546] != 1.0) || (s.v[547] != 1.0)) || (s.v[548] != 1.0)) || (s.v[549] != 1.0));
        s.v[1000] = if s.b[1000] { 1.0 } else { 0.0 };

        if s.b[1000] {
            s.store_scalar(635, 1.0);
        }

        if (!s.b[1000]) {
            s.store_scalar(635, 0.0);
        }

        s.b[1001] = (s.v[635] == 1.0);
        s.v[1001] = if s.b[1001] { 1.0 } else { 0.0 };

        if s.b[1001] {
            if ((s.v[507] * s.v[546]) > 1e-18) {
                s.store_mul(620, 507, 546);
            } else {
                s.store_scalar(620, 1e-18);
            }
        }

        if s.b[1001] {
            if ((s.v[510] * s.v[547]) > 0.05) {
                s.store_mul(621, 510, 547);
            } else {
                s.store_scalar(621, 0.05);
            }
        }

        if s.b[1001] {
            if ((if ((s.v[513] * s.v[548]) > 0.05) { (s.v[513] * s.v[548]) } else { 0.05 }) < 0.95) {
                if ((s.v[513] * s.v[548]) > 0.05) {
                    s.store_mul(622, 513, 548);
                } else {
                    s.store_scalar(622, 0.05);
                }
            } else {
                s.store_scalar(622, 0.95);
            }
        }

        if s.b[1001] {
            s.store_mul(623, 516, 549);
            s.store_offset(625, 623, s.v[375]);
            s.store_sub_from_scalar(630, 1.0, 622);
            s.store_div_from_scalar(631, 1.0, 630);
        }

        s.v[351] = ((ctx_temp + p.p56) + p.p35);

        s.v[352] = (s.v[351] / s.v[350]);

        s.v[353] = (s.v[351] - s.v[350]);

        s.v[354] = ((s.v[351] * 1.3806505e-23) / 1.6021918e-19);

        s.v[355] = (1.0 / s.v[354]);

        s.v[356] = s.v[351];

        s.v[357] = (s.v[356] * s.v[356]);

        s.v[358] = (s.v[356] - s.v[350]);

        s.v[359] = (s.v[350] / s.v[356]);

        s.v[360] = ((s.v[359]) as f64).ln();

        s.v[715] = ((s.v[356] * 1.3806505e-23) / 1.6021918e-19);

        s.v[361] = (1.0 / s.v[715]);

        s.v[362] = ((1.179 - (9.025e-5 * s.v[356])) - (3.05e-7 * s.v[357]));

        s.v[363] = ((((1.045 + (0.00045 * s.v[356])) * ((0.523 + (0.0014 * s.v[356])) - (1.48e-6 * s.v[357]))) * s.v[357]) / 90000.0);

        if (!(s.v[363] > 0.001)) {
            s.store_scalar(363, 0.001);
        }

        s.v[365] = (((ctx_temp + p.p56) + p.p35)).max((273.15 + (-250.0)));

        s.v[366] = (s.v[365] / s.v[364]);

        s.v[370] = (s.v[367] * s.v[365]);

        s.v[371] = (1.0 / s.v[370]);

        s.v[376] = ((-((0.000702 * s.v[365]) * s.v[365])) / (1108.0 + s.v[365]));

        s.v[381] = (p.p834 + s.v[376]);

        s.v[382] = (p.p835 + s.v[376]);

        s.v[383] = (p.p836 + s.v[376]);

        s.v[384] = (((s.v[366]) as f64).powf(1.5) * (((0.5 * ((s.v[378] * s.v[369]) - (s.v[381] * s.v[371])))) as f64).exp());

        s.v[385] = (((s.v[366]) as f64).powf(1.5) * (((0.5 * ((s.v[379] * s.v[369]) - (s.v[382] * s.v[371])))) as f64).exp());

        s.v[386] = (((s.v[366]) as f64).powf(1.5) * (((0.5 * ((s.v[380] * s.v[369]) - (s.v[383] * s.v[371])))) as f64).exp());

        s.v[387] = ((p.p837 * s.v[384]) * s.v[384]);

        s.v[388] = ((p.p838 * s.v[385]) * s.v[385]);

        s.v[389] = ((p.p839 * s.v[386]) * s.v[386]);

        s.v[390] = ((p.p828 * s.v[366]) - ((2.0 * s.v[370]) * ((s.v[384]) as f64).ln()));

        s.v[391] = ((p.p829 * s.v[366]) - ((2.0 * s.v[370]) * ((s.v[385]) as f64).ln()));

        s.v[392] = ((p.p830 * s.v[366]) - ((2.0 * s.v[370]) * ((s.v[386]) as f64).ln()));

        s.v[393] = (s.v[390] + (s.v[370] * (((1.0 + ((((0.05 - s.v[390]) * s.v[371])) as f64).exp())) as f64).ln()));

        s.v[394] = (s.v[391] + (s.v[370] * (((1.0 + ((((0.05 - s.v[391]) * s.v[371])) as f64).exp())) as f64).ln()));

        s.v[395] = (s.v[392] + (s.v[370] * (((1.0 + ((((0.05 - s.v[392]) * s.v[371])) as f64).exp())) as f64).ln()));

        s.v[405] = (1.0 / s.v[393]);

        s.v[406] = (1.0 / s.v[394]);

        s.v[407] = (1.0 / s.v[395]);

        s.v[414] = (p.p825 * (((p.p828 * s.v[405])) as f64).powf(p.p831));

        s.v[415] = (p.p826 * (((p.p829 * s.v[406])) as f64).powf(p.p832));

        s.v[416] = (p.p827 * (((p.p830 * s.v[407])) as f64).powf(p.p833));

        s.v[417] = ((s.v[414] * s.v[393]) * s.v[411]);

        s.v[418] = ((s.v[415] * s.v[394]) * s.v[412]);

        s.v[419] = ((s.v[416] * s.v[395]) * s.v[413]);

        s.v[420] = (2.0 * s.v[414]);

        s.v[421] = (2.0 * s.v[415]);

        s.v[422] = (2.0 * s.v[416]);

        s.v[432] = ((0.5 * s.v[381])).max(s.v[370]);

        s.v[433] = ((0.5 * s.v[382])).max(s.v[370]);

        s.v[434] = ((0.5 * s.v[383])).max(s.v[370]);

        s.v[435] = (s.v[432] * s.v[371]);

        s.v[436] = (s.v[433] * s.v[371]);

        s.v[437] = (s.v[434] * s.v[371]);

        s.v[438] = (((((((32.0 * p.p848) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[432] * s.v[432]) * s.v[432]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[439] = (((((((32.0 * p.p849) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[433] * s.v[433]) * s.v[433]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[440] = (((((((32.0 * p.p850) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[434] * s.v[434]) * s.v[434]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[441] = (p.p854 * (1.0 + (p.p857 * (s.v[365] - s.v[364]))));

        s.v[442] = (p.p855 * (1.0 + (p.p858 * (s.v[365] - s.v[364]))));

        s.v[443] = (p.p856 * (1.0 + (p.p859 * (s.v[365] - s.v[364]))));

        if (!(s.v[441] > 0.0)) {
            s.store_scalar(441, 0.0);
        }

        if (!(s.v[442] > 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (!(s.v[443] > 0.0)) {
            s.store_scalar(443, 0.0);
        }

        s.b[1021] = (s.v[473] == 1.0);
        s.v[1021] = if s.b[1021] { 1.0 } else { 0.0 };

        if s.b[1021] {
            s.store_offset(461, 460, s.v[376]);
            s.store_scale_ad(463, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(462), s.v[369], s.ad_value(461), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));
            s.store_sub_scaled_inputs_ad_rhs(464, 458, s.v[366], A::ln(s.ad_value(463)), (2.0 * s.v[370]));
            s.store_add_scaled_inputs_ad_rhs(465, 464, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(464), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);
            s.store_div_from_scalar(466, 1.0, 465);
            s.store_mul_pow_ad_rhs(469, 457, A::mul(s.ad_value(458), s.ad_value(466)), s.ad_value(459));
            s.store_mul3_lhs(470, 469, 465, 468);
            s.store_scale(471, 469, 2.0);
        }

        s.store_offset(557, 514, s.v[376]);

        s.store_offset(558, 515, s.v[376]);

        s.store_offset(559, 516, s.v[376]);

        s.store_scale_ad(560, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(554), s.v[369], s.ad_value(557), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));

        s.store_scale_ad(561, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(555), s.v[369], s.ad_value(558), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));

        s.store_scale_ad(562, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(556), s.v[369], s.ad_value(559), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));

        s.store_mul3_lhs(563, 517, 560, 560);

        s.store_mul3_lhs(564, 518, 561, 561);

        s.store_mul3_lhs(565, 519, 562, 562);

        s.store_sub_scaled_inputs_ad_rhs(566, 508, s.v[366], A::ln(s.ad_value(560)), (2.0 * s.v[370]));

        s.store_sub_scaled_inputs_ad_rhs(567, 509, s.v[366], A::ln(s.ad_value(561)), (2.0 * s.v[370]));

        s.store_sub_scaled_inputs_ad_rhs(568, 510, s.v[366], A::ln(s.ad_value(562)), (2.0 * s.v[370]));

        s.store_add_scaled_inputs_ad_rhs(569, 566, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(566), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);

        s.store_add_scaled_inputs_ad_rhs(570, 567, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(567), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);

        s.store_add_scaled_inputs_ad_rhs(571, 568, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(568), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);

        s.store_div_from_scalar(572, 1.0, 569);

        s.store_div_from_scalar(573, 1.0, 570);

        s.store_div_from_scalar(574, 1.0, 571);

        s.store_mul_pow_ad_rhs(581, 505, A::mul(s.ad_value(508), s.ad_value(572)), s.ad_value(511));

        s.store_mul_pow_ad_rhs(582, 506, A::mul(s.ad_value(509), s.ad_value(573)), s.ad_value(512));

        s.store_mul_pow_ad_rhs(583, 507, A::mul(s.ad_value(510), s.ad_value(574)), s.ad_value(513));

        s.store_mul3_lhs(584, 581, 569, 578);

        s.store_mul3_lhs(585, 582, 570, 579);

        s.store_mul3_lhs(586, 583, 571, 580);

        s.store_scale(587, 581, 2.0);

        s.store_scale(588, 582, 2.0);

        s.store_scale(589, 583, 2.0);

        s.store_max_with_scalar_ad(599, A::scale(s.ad_value(557), 0.5), s.v[370]);

        s.store_max_with_scalar_ad(600, A::scale(s.ad_value(558), 0.5), s.v[370]);

        s.store_max_with_scalar_ad(601, A::scale(s.ad_value(559), 0.5), s.v[370]);

        s.store_scale(602, 599, s.v[371]);

        s.store_scale(603, 600, s.v[371]);

        s.store_scale(604, 601, s.v[371]);

        s.store_scaled_sqrt_ad(605, A::mul3_scaled_output(s.ad_value(528), A::square(s.ad_value(599)), s.ad_value(599), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_scaled_sqrt_ad(606, A::mul3_scaled_output(s.ad_value(529), A::square(s.ad_value(600)), s.ad_value(600), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_scaled_sqrt_ad(607, A::mul3_scaled_output(s.ad_value(530), A::square(s.ad_value(601)), s.ad_value(601), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_mul_scale_offset_rhs(608, 534, 537, (s.v[365] - s.v[364]), 1.0);

        s.store_mul_scale_offset_rhs(609, 535, 538, (s.v[365] - s.v[364]), 1.0);

        s.store_mul_scale_offset_rhs(610, 536, 539, (s.v[365] - s.v[364]), 1.0);

        if (!(s.v[608] > 0.0)) {
            s.store_scalar(608, 0.0);
        }

        if (!(s.v[609] > 0.0)) {
            s.store_scalar(609, 0.0);
        }

        if (!(s.v[610] > 0.0)) {
            s.store_scalar(610, 0.0);
        }

        s.b[1022] = (s.v[635] == 1.0);
        s.v[1022] = if s.b[1022] { 1.0 } else { 0.0 };

        if s.b[1022] {
            s.store_offset(624, 623, s.v[376]);
            s.store_scale_ad(626, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(625), s.v[369], s.ad_value(624), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));
            s.store_sub_scaled_inputs_ad_rhs(627, 621, s.v[366], A::ln(s.ad_value(626)), (2.0 * s.v[370]));
            s.store_add_scaled_inputs_ad_rhs(628, 627, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(627), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);
            s.store_div_from_scalar(629, 1.0, 628);
            s.store_mul_pow_ad_rhs(632, 620, A::mul(s.ad_value(621), s.ad_value(629)), s.ad_value(622));
            s.store_mul3_lhs(633, 632, 628, 631);
            s.store_scale(634, 632, 2.0);
        }

        s.v[5] = 1.0;

        s.v[6] = 1.0;

        s.v[312] = 0.0;

        s.v[313] = 0.0;

        s.v[7] = p.p0;

        s.v[8] = p.p1;

        s.v[9] = p.p2;

        s.v[10] = p.p3;

        s.v[11] = p.p4;

        s.v[12] = p.p8;

        s.v[646] = p.p19;

        s.v[647] = p.p20;

        s.v[648] = p.p21;

        s.v[673] = p.p22;

        s.v[674] = p.p23;

        s.v[675] = p.p24;

        s.v[649] = p.p25;

        s.v[650] = p.p26;

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.v[676] = p.p27;

        s.v[677] = p.p28;

        s.v[14] = p.p14;

        s.b[1023] = (p.p39 > 0.0);
        s.v[1023] = if s.b[1023] { 1.0 } else { 0.0 };

        if s.b[1023] {
            s.store_scalar(5, (if (p.p9 > 1.0) { p.p9 } else { 1.0 }));
        }

        if s.b[1023] {
            s.store_floor_ad(5, A::offset(s.ad_value(5), 0.5));
            s.store_div_from_scalar(6, 1.0, 5);
        }

        if ((s.v[8] * s.v[6]) > 1e-9) {
            s.store_scale(8, 6, s.v[8]);
        } else {
            s.store_scalar(8, 1e-9);
        }

        s.v[15] = p.p5;

        s.v[16] = p.p6;

        s.v[17] = p.p7;

        s.v[308] = (1e-6 / s.v[7]);

        s.store_div_from_scalar(309, 1e-6, 8);

        s.store_offset_scaled(310, 309, ((p.p190) * ((p.p188 * (1.0 + (p.p189 * s.v[308]))))), (p.p188 * (1.0 + (p.p189 * s.v[308]))));

        s.store_offset_scaled(311, 309, ((p.p194) * ((p.p192 * (1.0 + (p.p193 * s.v[308]))))), (p.p192 * (1.0 + (p.p193 * s.v[308]))));

        if (((s.v[7] + s.v[310]) - (2.0 * p.p191)) > 1e-9) {
            s.store_offset(312, 310, ((s.v[7]) + ((-(2.0 * p.p191)))));
        } else {
            s.store_scalar(312, 1e-9);
        }

        if (((s.v[8] + s.v[311]) - (2.0 * p.p195)) > 1e-9) {
            s.store_offset_add(313, 8, 311, (-(2.0 * p.p195)));
        } else {
            s.store_scalar(313, 1e-9);
        }

        s.store_div_from_scalar(314, 1e-6, 312);

        s.store_square(315, 314);

        s.store_div_from_scalar(316, 1e-6, 313);

        s.store_div_from_scalar(317, 1.0, 316);

        s.store_mul(318, 314, 316);

        s.store_div_from_scalar(319, 1.0, 318);

        if ((((s.v[7] + s.v[310]) - (2.0 * p.p191)) + p.p196) > 1e-9) {
            s.store_offset(320, 310, ((((s.v[7]) + ((-(2.0 * p.p191))))) + (p.p196)));
        } else {
            s.store_scalar(320, 1e-9);
        }

        if ((((s.v[8] + s.v[311]) - (2.0 * p.p195)) + p.p197) > 1e-9) {
            s.store_offset_add(321, 8, 311, (((-(2.0 * p.p195))) + (p.p197)));
        } else {
            s.store_scalar(321, 1e-9);
        }

        s.store_scale(322, 321, 1000000.0);

        if (((s.v[7] + s.v[310]) + p.p196) > 1e-9) {
            s.store_offset(323, 310, ((s.v[7]) + (p.p196)));
        } else {
            s.store_scalar(323, 1e-9);
        }

        if (((s.v[8] + s.v[311]) + p.p197) > 1e-9) {
            s.store_offset_add(324, 8, 311, p.p197);
        } else {
            s.store_scalar(324, 1e-9);
        }

        s.store_scale(325, 323, 1000000.0);

        s.store_scale(326, 324, 1000000.0);

        s.v[44] = p.p57;

        s.v[45] = p.p58;

        s.v[46] = p.p59;

        s.v[47] = p.p60;

        s.v[48] = p.p61;

        s.v[49] = p.p62;

        s.v[50] = p.p63;

        s.v[51] = p.p64;

        s.v[52] = p.p65;

        s.v[53] = p.p66;

        s.v[54] = p.p67;

        s.v[59] = p.p68;

        s.v[60] = p.p69;

        s.v[61] = p.p70;

        s.v[62] = p.p71;

        s.v[55] = p.p72;

        s.v[56] = p.p74;

        s.v[57] = p.p73;

        s.v[58] = p.p75;

        s.v[63] = p.p79;

        s.v[64] = p.p81;

        s.v[65] = p.p80;

        s.v[66] = p.p76;

        s.v[67] = p.p78;

        s.v[68] = p.p77;

        s.v[69] = p.p82;

        s.v[70] = p.p83;

        s.v[71] = p.p84;

        s.v[72] = p.p85;

        s.v[73] = p.p86;

        s.v[74] = p.p87;

        s.v[75] = p.p88;

        s.v[76] = p.p89;

        s.v[77] = p.p90;

        s.v[78] = p.p91;

        s.v[79] = p.p92;

        s.v[80] = p.p93;

        s.v[81] = p.p94;

        s.v[82] = p.p95;

        s.v[83] = p.p96;

        s.v[84] = p.p97;

        s.v[85] = p.p98;

        s.v[86] = p.p99;

        s.v[87] = p.p100;

        s.v[88] = p.p101;

        s.v[89] = p.p102;

        s.v[90] = p.p103;

        s.v[91] = p.p104;

        s.v[92] = p.p105;

        s.v[93] = p.p106;

        s.v[94] = p.p107;

        s.v[95] = p.p108;

        s.v[96] = p.p109;

        s.v[97] = p.p110;

        s.v[98] = p.p111;

        s.v[99] = p.p112;

        s.v[100] = p.p113;

        s.v[101] = p.p114;

        s.v[102] = p.p115;

        s.v[103] = p.p116;

        s.v[104] = p.p117;

        s.v[105] = p.p118;

        s.v[106] = p.p119;

        s.v[107] = p.p120;

        s.v[108] = p.p121;

        s.v[109] = p.p120;

        s.b[1024] = param_given[122];
        s.v[1024] = if s.b[1024] { 1.0 } else { 0.0 };

        if s.b[1024] {
            s.store_scalar(109, p.p122);
        }

        s.v[110] = p.p121;

        s.b[1025] = param_given[123];
        s.v[1025] = if s.b[1025] { 1.0 } else { 0.0 };

        if s.b[1025] {
            s.store_scalar(110, p.p123);
        }

        s.copy_ad(111, 109);

        s.b[1026] = param_given[124];
        s.v[1026] = if s.b[1026] { 1.0 } else { 0.0 };

        if s.b[1026] {
            s.store_scalar(111, p.p124);
        }

        s.copy_ad(112, 110);

        s.b[1027] = param_given[125];
        s.v[1027] = if s.b[1027] { 1.0 } else { 0.0 };

        if s.b[1027] {
            s.store_scalar(112, p.p125);
        }

        s.v[113] = p.p126;

        s.v[114] = p.p127;

        s.v[115] = p.p128;

        s.v[116] = p.p129;

        s.v[117] = p.p130;

        s.v[118] = p.p131;

        s.v[119] = p.p132;

        s.v[120] = p.p133;

        s.v[121] = p.p134;

        s.v[122] = p.p135;

        s.v[123] = p.p136;

        s.v[124] = p.p137;

        s.v[125] = p.p99;

        s.b[1028] = param_given[138];
        s.v[1028] = if s.b[1028] { 1.0 } else { 0.0 };

        if s.b[1028] {
            s.store_scalar(125, p.p138);
        }

        s.v[126] = p.p104;

        s.b[1029] = param_given[139];
        s.v[1029] = if s.b[1029] { 1.0 } else { 0.0 };

        if s.b[1029] {
            s.store_scalar(126, p.p139);
        }

        s.v[127] = p.p140;

        s.v[128] = p.p141;

        s.v[129] = p.p142;

        s.v[130] = p.p143;

        s.v[131] = p.p144;

        s.v[132] = p.p145;

        s.v[133] = p.p146;

        s.v[134] = p.p147;

        s.v[135] = p.p148;

        s.v[136] = p.p149;

        s.v[137] = p.p150;

        s.v[138] = p.p151;

        s.v[139] = p.p152;

        s.v[140] = p.p153;

        s.v[141] = p.p154;

        s.v[142] = p.p155;

        s.v[143] = p.p156;

        s.v[149] = p.p162;

        s.v[150] = p.p163;

        s.v[151] = p.p164;

        s.v[152] = p.p165;

        s.v[153] = p.p166;

        s.v[154] = p.p167;

        s.v[155] = p.p168;

        s.v[156] = p.p169;

        s.v[157] = p.p170;

        s.v[158] = p.p171;

        s.v[159] = p.p172;

        s.v[160] = p.p174;

        s.v[161] = p.p173;

        s.v[176] = p.p187;

        s.b[1030] = (p.p39 > 0.0);
        s.v[1030] = if s.b[1030] { 1.0 } else { 0.0 };

        if s.b[1030] {
            s.store_add_scaled_inputs3_offset_mixed_aii(44, A::powf(s.ad_value(314), p.p200), p.p199, 316, p.p201, 318, p.p202, p.p198);
            s.store_add_scaled_inputs3_offset_indices(45, 314, p.p204, 316, p.p205, 318, p.p206, p.p203);
            s.store_scalar(46, p.p207);
            s.store_scalar(47, p.p208);
            s.store_scalar(48, p.p209);
        }

        if s.b[1030] {
            s.store_scale_ad(331, {
                if ((1.0 + ((p.p211 * s.v[316]) * (((1.0 + (s.v[313] / p.p212))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p.p211, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p212), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p210);
        }

        if s.b[1030] {
            s.store_scale_ad(332, {
                if ((1.0 + ((p.p214 * s.v[316]) * (((1.0 + (s.v[313] / p.p215))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p.p214, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p215), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p213);
        }

        if s.b[1030] {
            s.store_scale_ad(333, {
                if ((1.0 + ((p.p217 * s.v[316]) * (((1.0 + (s.v[313] / p.p215))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p.p217, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p215), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p216);
        }

        s.b[1031] = (s.v[312] > (2.0 * s.v[333]));
        s.v[1031] = if s.b[1031] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1031]) {
            s.store_scalar(334, 75000000000.0);
            s.store_sub_ad(335, A::sqrt(A::add_scaled_inputs(s.ad_value(331), 1.0, s.ad_value(332), 0.5)), A::sqrt(s.ad_value(331)));
            s.store_add_scaled_product_mixed_aia(336, A::sqrt(s.ad_value(331)), 1.0, 334, A::ln(A::offset(A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(333), 2.0, s.ad_value(312), 1.0), A::exp(A::div(s.ad_value(335), s.ad_value(334))), (-1.0)), 1.0)), 1.0);
            s.store_square(336, 336);
        }

        s.b[1032] = (s.v[312] >= s.v[333]);
        s.v[1032] = if s.b[1032] { 1.0 } else { 0.0 };

        if ((s.b[1030] && (!s.b[1031])) && s.b[1032]) {
            s.store_add_ad_rhs(336, 331, A::div_scaled_product(s.ad_value(332), s.ad_value(333), 1.0, s.ad_value(312), 1.0));
        }

        if ((s.b[1030] && (!s.b[1031])) && (!s.b[1032])) {
            s.store_add_ad_rhs(336, 331, A::mul_sub_from_scalar_rhs(s.ad_value(332), 2.0, A::div(s.ad_value(312), s.ad_value(333))));
        }

        if s.b[1030] {
            s.store_mul_sub_scaled_inputs_rhs(49, 336, A::sub_from_scalar(1.0, A::scale(s.ad_value(314), p.p218)), 1.0, s.ad_value(315), p.p219);
            s.store_add_scaled_inputs3_offset_mixed_aii(50, A::powf(s.ad_value(314), p.p222), p.p221, 316, p.p223, 318, p.p224, p.p220);
            s.store_scalar(51, p.p225);
            s.store_scalar(52, p.p226);
            s.store_add_scaled_inputs3_offset_mixed_aii(53, A::powf(s.ad_value(314), p.p229), p.p228, 316, p.p230, 318, p.p231, p.p227);
        }

        if s.b[1030] {
            s.store_scale_ad(54, {
                if (1e-6 > (1.0 + (p.p233 * s.v[314]))) {
                    A::constant(1e-6)
                } else {
                    A::scale_offset(s.ad_value(314), p.p233, 1.0)
                }
            }, p.p232);
        }

        if s.b[1030] {
            s.store_scalar(59, p.p234);
            s.store_scalar(60, p.p235);
            s.store_scalar(61, p.p238);
            s.store_scalar(62, p.p239);
            s.store_mul3_ad(55, A::scale_offset(A::powf(s.ad_value(314), p.p242), p.p241, p.p240), A::scale_offset(s.ad_value(316), p.p243, 1.0), A::scale_offset(s.ad_value(318), p.p244, 1.0));
            s.store_scalar(56, p.p246);
            s.store_scalar(57, p.p245);
            s.store_scalar(58, p.p247);
        }

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1030] {
            s.store_scaled_mul_scale_offset_rhs_ad(66, A::powf(s.ad_value(314), p.p249), 316, p.p250, 1.0, p.p248);
            s.store_scalar(67, p.p252);
            s.store_scalar(68, p.p251);
            s.store_scaled_mul_scale_offset_rhs_ad(63, A::powf(s.ad_value(314), p.p254), 316, p.p255, 1.0, p.p253);
            s.store_scalar(64, p.p257);
            s.store_scalar(65, p.p256);
            s.store_offset_scaled(337, 316, ((p.p260) * (p.p259)), p.p259);
        }

        if s.b[1030] {
            s.store_scale_ad(338, {
                if ((1.0 + (p.p262 * s.v[316])) > 0.001) {
                    A::scale_offset(s.ad_value(316), p.p262, 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p261);
        }

        if s.b[1030] {
            s.store_add_ad(339, A::offset(A::mul_sub_from_scalar_rhs(A::div_scaled_product(s.ad_value(337), s.ad_value(338), 1.0, s.ad_value(312), 1.0), 1.0, A::exp(A::div_scaled_inputs(s.ad_value(312), -1.0, s.ad_value(338), 1.0))), 1.0), A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p263 * p.p264), s.ad_value(312)), 1.0, A::exp_scaled_input(s.ad_value(312), (-1.0 / (p.p264)))));
        }

        if s.b[1030] {
            if (s.v[339] > 1e-15) {
            } else {
                s.store_scalar(339, 1e-15);
            }
        }

        if s.b[1030] {
            s.store_add_scaled_product_mixed_aia(340, A::scale_offset(s.ad_value(316), p.p265, 1.0), 1.0, 316, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p267), 1.0)), p.p266);
            s.store_mul_div_scaled_inputs_rhs(69, 340, s.ad_value(313), p.p258, A::mul(s.ad_value(339), s.ad_value(312)), 1.0);
            s.store_add_scaled_inputs3_offset_indices(70, 314, p.p269, 316, p.p270, 318, p.p271, p.p268);
            s.store_offset_scaled(71, 316, ((p.p273) * (p.p272)), p.p272);
            s.store_scalar(72, p.p274);
            s.store_scalar(73, p.p275);
            s.store_scalar(74, p.p276);
            s.store_mul3_ad(75, A::scale_offset(A::powf(s.ad_value(314), p.p279), p.p278, p.p277), A::scale_offset(s.ad_value(316), p.p280, 1.0), A::scale_offset(s.ad_value(318), p.p281, 1.0));
            s.store_scalar(76, p.p282);
            s.store_scalar(77, p.p283);
            s.store_scalar(78, p.p284);
            s.store_mul3_ad_scaled_output(79, A::scale_offset(s.ad_value(314), p.p286, 1.0), A::scale_offset(s.ad_value(316), p.p287, 1.0), A::scale_offset(s.ad_value(318), p.p288, 1.0), p.p285);
            s.store_scalar(80, p.p289);
            s.store_scalar(81, p.p290);
            s.store_mul_scale_offset_rhs(82, 316, 316, ((p.p292) * (p.p291)), p.p291);
            s.store_scalar(83, p.p293);
            s.store_scalar(84, p.p294);
            s.store_scalar(85, p.p295);
            s.store_mul3_ad(86, A::offset(A::mul(A::div_scaled_inputs(s.ad_value(340), p.p297, s.ad_value(339), 1.0), A::powf(s.ad_value(314), p.p298)), p.p296), A::scale_offset(s.ad_value(316), p.p299, 1.0), A::scale_offset(s.ad_value(318), p.p300, 1.0));
            s.store_add_scaled_inputs3_offset_indices(87, 314, p.p302, 316, p.p303, 318, p.p304, p.p301);
            s.store_scalar(88, p.p305);
            s.store_scalar(89, p.p306);
            s.store_scalar(90, p.p307);
            s.store_div_from_scalar_offset_scaled_input(91, p.p308, 314, p.p309, 1.0);
            s.store_scaled_mul_scale_offset_rhs_ad(92, A::powf(s.ad_value(314), p.p311), 316, p.p312, 1.0, p.p310);
            s.store_powf(341, 314, p.p314);
            s.store_div_scaled_product_offset_denominator(93, s.ad_value(341), A::scale_offset(s.ad_value(316), p.p316, 1.0), p.p313, A::mul_scaled_lhs(s.ad_value(314), p.p315, s.ad_value(341)), 1.0, 1.0);
            s.store_powf(341, 314, p.p318);
            s.store_div_scaled_product_offset_denominator(94, s.ad_value(341), A::scale_offset(s.ad_value(316), p.p320, 1.0), p.p317, A::mul_scaled_lhs(s.ad_value(314), p.p319, s.ad_value(341)), 1.0, 1.0);
            s.store_scalar(95, p.p321);
            s.store_scaled_mul_scale_offset_inputs(96, 314, p.p323, 1.0, 316, p.p324, 1.0, p.p322);
            s.store_scalar(97, p.p325);
            s.store_scalar(98, p.p326);
            s.store_scaled_mul_scale_offset_inputs(99, 314, p.p328, 1.0, 316, p.p329, 1.0, p.p327);
            s.store_scaled_mul_scale_offset_inputs(100, 314, p.p331, 1.0, 316, p.p332, 1.0, p.p330);
            s.store_scalar(101, p.p333);
            s.store_scalar(102, p.p334);
            s.store_div_from_scalar(103, p.p335, 318);
            s.store_div_from_scalar_scaled_input(104, (p.p336 * p.p236), 316, 1e-6);
            s.store_div_from_scalar_scaled_input(105, (p.p337 * p.p237), 316, 1e-6);
            s.store_scalar(106, p.p338);
            s.store_scalar(107, p.p339);
            s.store_scalar(108, p.p340);
            s.store_scalar(109, p.p339);
        }

        s.b[1033] = param_given[341];
        s.v[1033] = if s.b[1033] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1033]) {
            s.store_scalar(109, p.p341);
        }

        if s.b[1030] {
            s.store_scalar(110, p.p340);
        }

        s.b[1034] = param_given[342];
        s.v[1034] = if s.b[1034] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1034]) {
            s.store_scalar(110, p.p342);
        }

        if s.b[1030] {
            s.copy_ad(111, 109);
        }

        s.b[1035] = param_given[343];
        s.v[1035] = if s.b[1035] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1035]) {
            s.store_scalar(111, p.p343);
        }

        if s.b[1030] {
            s.copy_ad(112, 110);
        }

        s.b[1036] = param_given[344];
        s.v[1036] = if s.b[1036] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1036]) {
            s.store_scalar(112, p.p344);
        }

        if s.b[1030] {
            s.store_scalar(113, p.p345);
            s.store_div_from_scalar_scaled_input(114, (p.p346 * p.p236), 316, 1e-6);
            s.store_div_from_scalar_scaled_input(115, (p.p347 * p.p237), 316, 1e-6);
            s.store_scalar(116, p.p348);
            s.store_scalar(117, p.p349);
            s.store_scalar(118, p.p350);
            s.store_scalar(119, p.p351);
            s.store_scalar(120, p.p352);
            s.store_scalar(121, p.p353);
            s.store_scaled_mul(122, 321, 320, ((8.8541878176e-12 * p.p209) * 1.0 / (p.p208)));
            s.store_scale(129, 321, ((8.8541878176e-12 * p.p209) * (p.p236 * 1.0 / (p.p234))));
            s.store_scale(130, 321, ((8.8541878176e-12 * p.p209) * (p.p237 * 1.0 / (p.p235))));
            s.store_add_scaled_inputs3_offset_mixed_aii(123, A::powf(s.ad_value(314), p.p356), p.p355, 316, p.p357, 318, p.p358, p.p354);
            s.store_add_scaled_inputs3_offset_indices(124, 314, p.p360, 316, p.p361, 318, p.p362, p.p359);
            s.store_scalar(36, p.p296);
        }

        s.b[1037] = param_given[363];
        s.v[1037] = if s.b[1037] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1037]) {
            s.store_scalar(36, p.p363);
        }

        if s.b[1030] {
            s.store_scalar(37, p.p297);
        }

        s.b[1038] = param_given[364];
        s.v[1038] = if s.b[1038] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1038]) {
            s.store_scalar(37, p.p364);
        }

        if s.b[1030] {
            s.store_scalar(38, p.p298);
        }

        s.b[1039] = param_given[365];
        s.v[1039] = if s.b[1039] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1039]) {
            s.store_scalar(38, p.p365);
        }

        if s.b[1030] {
            s.store_scalar(39, p.p299);
        }

        s.b[1040] = param_given[366];
        s.v[1040] = if s.b[1040] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1040]) {
            s.store_scalar(39, p.p366);
        }

        if s.b[1030] {
            s.store_scalar(40, p.p300);
        }

        s.b[1041] = param_given[367];
        s.v[1041] = if s.b[1041] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1041]) {
            s.store_scalar(40, p.p367);
        }

        if s.b[1030] {
            s.store_mul3_ad(125, A::add_scaled_product(s.ad_value(36), 1.0, A::div_scaled_product(s.ad_value(37), s.ad_value(340), 1.0, s.ad_value(339), 1.0), A::pow(s.ad_value(314), s.ad_value(38)), 1.0), A::offset(A::mul(s.ad_value(39), s.ad_value(316)), 1.0), A::offset(A::mul(s.ad_value(40), s.ad_value(318)), 1.0));
            s.store_scalar(41, p.p308);
        }

        s.b[1042] = param_given[368];
        s.v[1042] = if s.b[1042] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1042]) {
            s.store_scalar(41, p.p368);
        }

        if s.b[1030] {
            s.store_scalar(42, p.p309);
        }

        s.b[1043] = param_given[369];
        s.v[1043] = if s.b[1043] { 1.0 } else { 0.0 };

        if (s.b[1030] && s.b[1043]) {
            s.store_scalar(42, p.p369);
        }

        if s.b[1030] {
            s.store_div_scaled_value_offset_denominator(126, s.ad_value(41), 1.0, A::mul(s.ad_value(42), s.ad_value(314)), 1.0, 1.0);
            s.store_scaled_mul_scale_offset_rhs_ad(127, A::powf(s.ad_value(314), p.p371), 316, p.p372, 1.0, p.p370);
            s.store_powf(341, 314, p.p374);
            s.store_div_scaled_product_offset_denominator(128, s.ad_value(341), A::scale_offset(s.ad_value(316), p.p376, 1.0), p.p373, A::mul_scaled_lhs(s.ad_value(314), p.p375, s.ad_value(341)), 1.0, 1.0);
            s.store_scalar(131, p.p377);
            s.store_scalar(132, p.p378);
            s.store_scalar(133, p.p379);
            s.store_scale(134, 325, p.p380);
            s.store_scale(135, 322, p.p381);
            s.store_scale(136, 322, p.p382);
            s.store_scalar(137, p.p383);
            s.store_scalar(138, p.p384);
            s.store_scalar(139, p.p385);
            s.store_scalar(140, p.p386);
            s.store_scale(141, 326, p.p387);
            s.store_scale(142, 326, p.p388);
            s.store_sub_from_scalar_ad(1012, 1.0, A::div_from_scalar((2.0 * p.p395), s.ad_value(312)));
            s.store_scalar(143, p.p389);
            s.store_offset_scaled(344, 313, p.p398, (2.0 * p.p397));
            s.store_scalar(149, p.p399);
            s.store_add_scaled_inputs3_offset_indices(150, 314, p.p401, 316, p.p402, 318, p.p403, p.p400);
            s.store_add_scaled_inputs3_offset_mixed_aii(151, A::powf(s.ad_value(314), p.p406), p.p405, 316, p.p407, 318, p.p408, p.p404);
            s.store_mul3_ad_scaled_output(152, A::scale_offset(A::powf(s.ad_value(314), p.p411), p.p410, 1.0), A::scale_offset(s.ad_value(316), p.p412, 1.0), A::scale_offset(s.ad_value(318), p.p413, 1.0), p.p409);
            s.store_offset_scaled_ad(153, A::powf(s.ad_value(314), p.p416), p.p415, p.p414);
            s.store_offset_ad(347, A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p417 * p.p418), s.ad_value(312)), 1.0, A::exp_scaled_input(s.ad_value(312), (-1.0 / (p.p418)))), 1.0);
        }

    }
}

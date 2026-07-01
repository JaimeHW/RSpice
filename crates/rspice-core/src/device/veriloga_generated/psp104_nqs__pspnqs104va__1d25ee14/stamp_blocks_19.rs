#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_304(
        s: &mut Scratch,
    ) {
        if ((s.b[3037] && (!s.b[3039])) && s.b[3040]) {
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

    }

    pub(super) fn stamp_transient_block_305(
        s: &mut Scratch,
    ) {
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
        }

    }

    pub(super) fn stamp_transient_block_306(
        s: &mut Scratch,
    ) {
        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && (!s.b[3087])) {
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

    }

    pub(super) fn stamp_transient_block_307(
        s: &mut Scratch,
    ) {
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

    }

    pub(super) fn stamp_transient_block_308(
        s: &mut Scratch,
    ) {
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
        }

    }

    pub(super) fn stamp_transient_block_309(
        s: &mut Scratch,
    ) {
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && (!s.b[3162])) {
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

    }

    pub(super) fn stamp_transient_block_310(
        s: &mut Scratch,
    ) {
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
        }

    }

    pub(super) fn stamp_transient_block_311(
        s: &mut Scratch,
    ) {
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && s.b[3210]) {
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

    }

    pub(super) fn stamp_transient_block_312(
        s: &mut Scratch,
        p: &Parameters,
        var_abdrain_i: f64,
        var_absource_i: f64,
        var_lgdrain_i: f64,
        var_lgsource_i: f64,
        var_lsdrain_i: f64,
        var_lssource_i: f64,
        var_qgd_ov: f64,
        var_qgd_ov_db0: f64,
        var_qgd_ov_db1: f64,
        var_qgd_ov_db10: f64,
        var_qgd_ov_db11: f64,
        var_qgd_ov_db12: f64,
        var_qgd_ov_db13: f64,
        var_qgd_ov_db14: f64,
        var_qgd_ov_db15: f64,
        var_qgd_ov_db16: f64,
        var_qgd_ov_db17: f64,
        var_qgd_ov_db18: f64,
        var_qgd_ov_db19: f64,
        var_qgd_ov_db2: f64,
        var_qgd_ov_db20: f64,
        var_qgd_ov_db21: f64,
        var_qgd_ov_db22: f64,
        var_qgd_ov_db23: f64,
        var_qgd_ov_db24: f64,
        var_qgd_ov_db3: f64,
        var_qgd_ov_db4: f64,
        var_qgd_ov_db5: f64,
        var_qgd_ov_db6: f64,
        var_qgd_ov_db7: f64,
        var_qgd_ov_db8: f64,
        var_qgd_ov_db9: f64,
        var_qgd_ov_dn0: f64,
        var_qgd_ov_dn1: f64,
        var_qgd_ov_dn10: f64,
        var_qgd_ov_dn11: f64,
        var_qgd_ov_dn12: f64,
        var_qgd_ov_dn13: f64,
        var_qgd_ov_dn14: f64,
        var_qgd_ov_dn15: f64,
        var_qgd_ov_dn16: f64,
        var_qgd_ov_dn17: f64,
        var_qgd_ov_dn18: f64,
        var_qgd_ov_dn19: f64,
        var_qgd_ov_dn2: f64,
        var_qgd_ov_dn20: f64,
        var_qgd_ov_dn3: f64,
        var_qgd_ov_dn4: f64,
        var_qgd_ov_dn5: f64,
        var_qgd_ov_dn6: f64,
        var_qgd_ov_dn7: f64,
        var_qgd_ov_dn8: f64,
        var_qgd_ov_dn9: f64,
        var_qgs_ov: f64,
        var_qgs_ov_db0: f64,
        var_qgs_ov_db1: f64,
        var_qgs_ov_db10: f64,
        var_qgs_ov_db11: f64,
        var_qgs_ov_db12: f64,
        var_qgs_ov_db13: f64,
        var_qgs_ov_db14: f64,
        var_qgs_ov_db15: f64,
        var_qgs_ov_db16: f64,
        var_qgs_ov_db17: f64,
        var_qgs_ov_db18: f64,
        var_qgs_ov_db19: f64,
        var_qgs_ov_db2: f64,
        var_qgs_ov_db20: f64,
        var_qgs_ov_db21: f64,
        var_qgs_ov_db22: f64,
        var_qgs_ov_db23: f64,
        var_qgs_ov_db24: f64,
        var_qgs_ov_db3: f64,
        var_qgs_ov_db4: f64,
        var_qgs_ov_db5: f64,
        var_qgs_ov_db6: f64,
        var_qgs_ov_db7: f64,
        var_qgs_ov_db8: f64,
        var_qgs_ov_db9: f64,
        var_qgs_ov_dn0: f64,
        var_qgs_ov_dn1: f64,
        var_qgs_ov_dn10: f64,
        var_qgs_ov_dn11: f64,
        var_qgs_ov_dn12: f64,
        var_qgs_ov_dn13: f64,
        var_qgs_ov_dn14: f64,
        var_qgs_ov_dn15: f64,
        var_qgs_ov_dn16: f64,
        var_qgs_ov_dn17: f64,
        var_qgs_ov_dn18: f64,
        var_qgs_ov_dn19: f64,
        var_qgs_ov_dn2: f64,
        var_qgs_ov_dn20: f64,
        var_qgs_ov_dn3: f64,
        var_qgs_ov_dn4: f64,
        var_qgs_ov_dn5: f64,
        var_qgs_ov_dn6: f64,
        var_qgs_ov_dn7: f64,
        var_qgs_ov_dn8: f64,
        var_qgs_ov_dn9: f64,
        var_qjunbot_d: f64,
        var_qjunbot_d_db0: f64,
        var_qjunbot_d_db1: f64,
        var_qjunbot_d_db10: f64,
        var_qjunbot_d_db11: f64,
        var_qjunbot_d_db12: f64,
        var_qjunbot_d_db13: f64,
        var_qjunbot_d_db14: f64,
        var_qjunbot_d_db15: f64,
        var_qjunbot_d_db16: f64,
        var_qjunbot_d_db17: f64,
        var_qjunbot_d_db18: f64,
        var_qjunbot_d_db19: f64,
        var_qjunbot_d_db2: f64,
        var_qjunbot_d_db20: f64,
        var_qjunbot_d_db21: f64,
        var_qjunbot_d_db22: f64,
        var_qjunbot_d_db23: f64,
        var_qjunbot_d_db24: f64,
        var_qjunbot_d_db3: f64,
        var_qjunbot_d_db4: f64,
        var_qjunbot_d_db5: f64,
        var_qjunbot_d_db6: f64,
        var_qjunbot_d_db7: f64,
        var_qjunbot_d_db8: f64,
        var_qjunbot_d_db9: f64,
        var_qjunbot_d_dn0: f64,
        var_qjunbot_d_dn1: f64,
        var_qjunbot_d_dn10: f64,
        var_qjunbot_d_dn11: f64,
        var_qjunbot_d_dn12: f64,
        var_qjunbot_d_dn13: f64,
        var_qjunbot_d_dn14: f64,
        var_qjunbot_d_dn15: f64,
        var_qjunbot_d_dn16: f64,
        var_qjunbot_d_dn17: f64,
        var_qjunbot_d_dn18: f64,
        var_qjunbot_d_dn19: f64,
        var_qjunbot_d_dn2: f64,
        var_qjunbot_d_dn20: f64,
        var_qjunbot_d_dn3: f64,
        var_qjunbot_d_dn4: f64,
        var_qjunbot_d_dn5: f64,
        var_qjunbot_d_dn6: f64,
        var_qjunbot_d_dn7: f64,
        var_qjunbot_d_dn8: f64,
        var_qjunbot_d_dn9: f64,
        var_qjunbot_s: f64,
        var_qjunbot_s_db0: f64,
        var_qjunbot_s_db1: f64,
        var_qjunbot_s_db10: f64,
        var_qjunbot_s_db11: f64,
        var_qjunbot_s_db12: f64,
        var_qjunbot_s_db13: f64,
        var_qjunbot_s_db14: f64,
        var_qjunbot_s_db15: f64,
        var_qjunbot_s_db16: f64,
        var_qjunbot_s_db17: f64,
        var_qjunbot_s_db18: f64,
        var_qjunbot_s_db19: f64,
        var_qjunbot_s_db2: f64,
        var_qjunbot_s_db20: f64,
        var_qjunbot_s_db21: f64,
        var_qjunbot_s_db22: f64,
        var_qjunbot_s_db23: f64,
        var_qjunbot_s_db24: f64,
        var_qjunbot_s_db3: f64,
        var_qjunbot_s_db4: f64,
        var_qjunbot_s_db5: f64,
        var_qjunbot_s_db6: f64,
        var_qjunbot_s_db7: f64,
        var_qjunbot_s_db8: f64,
        var_qjunbot_s_db9: f64,
        var_qjunbot_s_dn0: f64,
        var_qjunbot_s_dn1: f64,
        var_qjunbot_s_dn10: f64,
        var_qjunbot_s_dn11: f64,
        var_qjunbot_s_dn12: f64,
        var_qjunbot_s_dn13: f64,
        var_qjunbot_s_dn14: f64,
        var_qjunbot_s_dn15: f64,
        var_qjunbot_s_dn16: f64,
        var_qjunbot_s_dn17: f64,
        var_qjunbot_s_dn18: f64,
        var_qjunbot_s_dn19: f64,
        var_qjunbot_s_dn2: f64,
        var_qjunbot_s_dn20: f64,
        var_qjunbot_s_dn3: f64,
        var_qjunbot_s_dn4: f64,
        var_qjunbot_s_dn5: f64,
        var_qjunbot_s_dn6: f64,
        var_qjunbot_s_dn7: f64,
        var_qjunbot_s_dn8: f64,
        var_qjunbot_s_dn9: f64,
        var_qjungat_d: f64,
        var_qjungat_d_db0: f64,
        var_qjungat_d_db1: f64,
        var_qjungat_d_db10: f64,
        var_qjungat_d_db11: f64,
        var_qjungat_d_db12: f64,
        var_qjungat_d_db13: f64,
        var_qjungat_d_db14: f64,
        var_qjungat_d_db15: f64,
        var_qjungat_d_db16: f64,
        var_qjungat_d_db17: f64,
        var_qjungat_d_db18: f64,
        var_qjungat_d_db19: f64,
        var_qjungat_d_db2: f64,
        var_qjungat_d_db20: f64,
        var_qjungat_d_db21: f64,
        var_qjungat_d_db22: f64,
        var_qjungat_d_db23: f64,
        var_qjungat_d_db24: f64,
        var_qjungat_d_db3: f64,
        var_qjungat_d_db4: f64,
        var_qjungat_d_db5: f64,
        var_qjungat_d_db6: f64,
        var_qjungat_d_db7: f64,
        var_qjungat_d_db8: f64,
        var_qjungat_d_db9: f64,
        var_qjungat_d_dn0: f64,
        var_qjungat_d_dn1: f64,
        var_qjungat_d_dn10: f64,
        var_qjungat_d_dn11: f64,
        var_qjungat_d_dn12: f64,
        var_qjungat_d_dn13: f64,
        var_qjungat_d_dn14: f64,
        var_qjungat_d_dn15: f64,
        var_qjungat_d_dn16: f64,
        var_qjungat_d_dn17: f64,
        var_qjungat_d_dn18: f64,
        var_qjungat_d_dn19: f64,
        var_qjungat_d_dn2: f64,
        var_qjungat_d_dn20: f64,
        var_qjungat_d_dn3: f64,
        var_qjungat_d_dn4: f64,
        var_qjungat_d_dn5: f64,
        var_qjungat_d_dn6: f64,
        var_qjungat_d_dn7: f64,
        var_qjungat_d_dn8: f64,
        var_qjungat_d_dn9: f64,
        var_qjungat_s: f64,
        var_qjungat_s_db0: f64,
        var_qjungat_s_db1: f64,
        var_qjungat_s_db10: f64,
        var_qjungat_s_db11: f64,
        var_qjungat_s_db12: f64,
        var_qjungat_s_db13: f64,
        var_qjungat_s_db14: f64,
        var_qjungat_s_db15: f64,
        var_qjungat_s_db16: f64,
        var_qjungat_s_db17: f64,
        var_qjungat_s_db18: f64,
        var_qjungat_s_db19: f64,
        var_qjungat_s_db2: f64,
        var_qjungat_s_db20: f64,
        var_qjungat_s_db21: f64,
        var_qjungat_s_db22: f64,
        var_qjungat_s_db23: f64,
        var_qjungat_s_db24: f64,
        var_qjungat_s_db3: f64,
        var_qjungat_s_db4: f64,
        var_qjungat_s_db5: f64,
        var_qjungat_s_db6: f64,
        var_qjungat_s_db7: f64,
        var_qjungat_s_db8: f64,
        var_qjungat_s_db9: f64,
        var_qjungat_s_dn0: f64,
        var_qjungat_s_dn1: f64,
        var_qjungat_s_dn10: f64,
        var_qjungat_s_dn11: f64,
        var_qjungat_s_dn12: f64,
        var_qjungat_s_dn13: f64,
        var_qjungat_s_dn14: f64,
        var_qjungat_s_dn15: f64,
        var_qjungat_s_dn16: f64,
        var_qjungat_s_dn17: f64,
        var_qjungat_s_dn18: f64,
        var_qjungat_s_dn19: f64,
        var_qjungat_s_dn2: f64,
        var_qjungat_s_dn20: f64,
        var_qjungat_s_dn3: f64,
        var_qjungat_s_dn4: f64,
        var_qjungat_s_dn5: f64,
        var_qjungat_s_dn6: f64,
        var_qjungat_s_dn7: f64,
        var_qjungat_s_dn8: f64,
        var_qjungat_s_dn9: f64,
        var_qjunsti_d: f64,
        var_qjunsti_d_db0: f64,
        var_qjunsti_d_db1: f64,
        var_qjunsti_d_db10: f64,
        var_qjunsti_d_db11: f64,
        var_qjunsti_d_db12: f64,
        var_qjunsti_d_db13: f64,
        var_qjunsti_d_db14: f64,
        var_qjunsti_d_db15: f64,
        var_qjunsti_d_db16: f64,
        var_qjunsti_d_db17: f64,
        var_qjunsti_d_db18: f64,
        var_qjunsti_d_db19: f64,
        var_qjunsti_d_db2: f64,
        var_qjunsti_d_db20: f64,
        var_qjunsti_d_db21: f64,
        var_qjunsti_d_db22: f64,
        var_qjunsti_d_db23: f64,
        var_qjunsti_d_db24: f64,
        var_qjunsti_d_db3: f64,
        var_qjunsti_d_db4: f64,
        var_qjunsti_d_db5: f64,
        var_qjunsti_d_db6: f64,
        var_qjunsti_d_db7: f64,
        var_qjunsti_d_db8: f64,
        var_qjunsti_d_db9: f64,
        var_qjunsti_d_dn0: f64,
        var_qjunsti_d_dn1: f64,
        var_qjunsti_d_dn10: f64,
        var_qjunsti_d_dn11: f64,
        var_qjunsti_d_dn12: f64,
        var_qjunsti_d_dn13: f64,
        var_qjunsti_d_dn14: f64,
        var_qjunsti_d_dn15: f64,
        var_qjunsti_d_dn16: f64,
        var_qjunsti_d_dn17: f64,
        var_qjunsti_d_dn18: f64,
        var_qjunsti_d_dn19: f64,
        var_qjunsti_d_dn2: f64,
        var_qjunsti_d_dn20: f64,
        var_qjunsti_d_dn3: f64,
        var_qjunsti_d_dn4: f64,
        var_qjunsti_d_dn5: f64,
        var_qjunsti_d_dn6: f64,
        var_qjunsti_d_dn7: f64,
        var_qjunsti_d_dn8: f64,
        var_qjunsti_d_dn9: f64,
        var_qjunsti_s: f64,
        var_qjunsti_s_db0: f64,
        var_qjunsti_s_db1: f64,
        var_qjunsti_s_db10: f64,
        var_qjunsti_s_db11: f64,
        var_qjunsti_s_db12: f64,
        var_qjunsti_s_db13: f64,
        var_qjunsti_s_db14: f64,
        var_qjunsti_s_db15: f64,
        var_qjunsti_s_db16: f64,
        var_qjunsti_s_db17: f64,
        var_qjunsti_s_db18: f64,
        var_qjunsti_s_db19: f64,
        var_qjunsti_s_db2: f64,
        var_qjunsti_s_db20: f64,
        var_qjunsti_s_db21: f64,
        var_qjunsti_s_db22: f64,
        var_qjunsti_s_db23: f64,
        var_qjunsti_s_db24: f64,
        var_qjunsti_s_db3: f64,
        var_qjunsti_s_db4: f64,
        var_qjunsti_s_db5: f64,
        var_qjunsti_s_db6: f64,
        var_qjunsti_s_db7: f64,
        var_qjunsti_s_db8: f64,
        var_qjunsti_s_db9: f64,
        var_qjunsti_s_dn0: f64,
        var_qjunsti_s_dn1: f64,
        var_qjunsti_s_dn10: f64,
        var_qjunsti_s_dn11: f64,
        var_qjunsti_s_dn12: f64,
        var_qjunsti_s_dn13: f64,
        var_qjunsti_s_dn14: f64,
        var_qjunsti_s_dn15: f64,
        var_qjunsti_s_dn16: f64,
        var_qjunsti_s_dn17: f64,
        var_qjunsti_s_dn18: f64,
        var_qjunsti_s_dn19: f64,
        var_qjunsti_s_dn2: f64,
        var_qjunsti_s_dn20: f64,
        var_qjunsti_s_dn3: f64,
        var_qjunsti_s_dn4: f64,
        var_qjunsti_s_dn5: f64,
        var_qjunsti_s_dn6: f64,
        var_qjunsti_s_dn7: f64,
        var_qjunsti_s_dn8: f64,
        var_qjunsti_s_dn9: f64,
        var_qfgd_slot: &mut f64,
        var_qfgd_db0_slot: &mut f64,
        var_qfgd_db1_slot: &mut f64,
        var_qfgd_db10_slot: &mut f64,
        var_qfgd_db11_slot: &mut f64,
        var_qfgd_db12_slot: &mut f64,
        var_qfgd_db13_slot: &mut f64,
        var_qfgd_db14_slot: &mut f64,
        var_qfgd_db15_slot: &mut f64,
        var_qfgd_db16_slot: &mut f64,
        var_qfgd_db17_slot: &mut f64,
        var_qfgd_db18_slot: &mut f64,
        var_qfgd_db19_slot: &mut f64,
        var_qfgd_db2_slot: &mut f64,
        var_qfgd_db20_slot: &mut f64,
        var_qfgd_db21_slot: &mut f64,
        var_qfgd_db22_slot: &mut f64,
        var_qfgd_db23_slot: &mut f64,
        var_qfgd_db24_slot: &mut f64,
        var_qfgd_db3_slot: &mut f64,
        var_qfgd_db4_slot: &mut f64,
        var_qfgd_db5_slot: &mut f64,
        var_qfgd_db6_slot: &mut f64,
        var_qfgd_db7_slot: &mut f64,
        var_qfgd_db8_slot: &mut f64,
        var_qfgd_db9_slot: &mut f64,
        var_qfgd_dn0_slot: &mut f64,
        var_qfgd_dn1_slot: &mut f64,
        var_qfgd_dn10_slot: &mut f64,
        var_qfgd_dn11_slot: &mut f64,
        var_qfgd_dn12_slot: &mut f64,
        var_qfgd_dn13_slot: &mut f64,
        var_qfgd_dn14_slot: &mut f64,
        var_qfgd_dn15_slot: &mut f64,
        var_qfgd_dn16_slot: &mut f64,
        var_qfgd_dn17_slot: &mut f64,
        var_qfgd_dn18_slot: &mut f64,
        var_qfgd_dn19_slot: &mut f64,
        var_qfgd_dn2_slot: &mut f64,
        var_qfgd_dn20_slot: &mut f64,
        var_qfgd_dn3_slot: &mut f64,
        var_qfgd_dn4_slot: &mut f64,
        var_qfgd_dn5_slot: &mut f64,
        var_qfgd_dn6_slot: &mut f64,
        var_qfgd_dn7_slot: &mut f64,
        var_qfgd_dn8_slot: &mut f64,
        var_qfgd_dn9_slot: &mut f64,
        var_qfgs_slot: &mut f64,
        var_qfgs_db0_slot: &mut f64,
        var_qfgs_db1_slot: &mut f64,
        var_qfgs_db10_slot: &mut f64,
        var_qfgs_db11_slot: &mut f64,
        var_qfgs_db12_slot: &mut f64,
        var_qfgs_db13_slot: &mut f64,
        var_qfgs_db14_slot: &mut f64,
        var_qfgs_db15_slot: &mut f64,
        var_qfgs_db16_slot: &mut f64,
        var_qfgs_db17_slot: &mut f64,
        var_qfgs_db18_slot: &mut f64,
        var_qfgs_db19_slot: &mut f64,
        var_qfgs_db2_slot: &mut f64,
        var_qfgs_db20_slot: &mut f64,
        var_qfgs_db21_slot: &mut f64,
        var_qfgs_db22_slot: &mut f64,
        var_qfgs_db23_slot: &mut f64,
        var_qfgs_db24_slot: &mut f64,
        var_qfgs_db3_slot: &mut f64,
        var_qfgs_db4_slot: &mut f64,
        var_qfgs_db5_slot: &mut f64,
        var_qfgs_db6_slot: &mut f64,
        var_qfgs_db7_slot: &mut f64,
        var_qfgs_db8_slot: &mut f64,
        var_qfgs_db9_slot: &mut f64,
        var_qfgs_dn0_slot: &mut f64,
        var_qfgs_dn1_slot: &mut f64,
        var_qfgs_dn10_slot: &mut f64,
        var_qfgs_dn11_slot: &mut f64,
        var_qfgs_dn12_slot: &mut f64,
        var_qfgs_dn13_slot: &mut f64,
        var_qfgs_dn14_slot: &mut f64,
        var_qfgs_dn15_slot: &mut f64,
        var_qfgs_dn16_slot: &mut f64,
        var_qfgs_dn17_slot: &mut f64,
        var_qfgs_dn18_slot: &mut f64,
        var_qfgs_dn19_slot: &mut f64,
        var_qfgs_dn2_slot: &mut f64,
        var_qfgs_dn20_slot: &mut f64,
        var_qfgs_dn3_slot: &mut f64,
        var_qfgs_dn4_slot: &mut f64,
        var_qfgs_dn5_slot: &mut f64,
        var_qfgs_dn6_slot: &mut f64,
        var_qfgs_dn7_slot: &mut f64,
        var_qfgs_dn8_slot: &mut f64,
        var_qfgs_dn9_slot: &mut f64,
        var_qjun_d_slot: &mut f64,
        var_qjun_d_db0_slot: &mut f64,
        var_qjun_d_db1_slot: &mut f64,
        var_qjun_d_db10_slot: &mut f64,
        var_qjun_d_db11_slot: &mut f64,
        var_qjun_d_db12_slot: &mut f64,
        var_qjun_d_db13_slot: &mut f64,
        var_qjun_d_db14_slot: &mut f64,
        var_qjun_d_db15_slot: &mut f64,
        var_qjun_d_db16_slot: &mut f64,
        var_qjun_d_db17_slot: &mut f64,
        var_qjun_d_db18_slot: &mut f64,
        var_qjun_d_db19_slot: &mut f64,
        var_qjun_d_db2_slot: &mut f64,
        var_qjun_d_db20_slot: &mut f64,
        var_qjun_d_db21_slot: &mut f64,
        var_qjun_d_db22_slot: &mut f64,
        var_qjun_d_db23_slot: &mut f64,
        var_qjun_d_db24_slot: &mut f64,
        var_qjun_d_db3_slot: &mut f64,
        var_qjun_d_db4_slot: &mut f64,
        var_qjun_d_db5_slot: &mut f64,
        var_qjun_d_db6_slot: &mut f64,
        var_qjun_d_db7_slot: &mut f64,
        var_qjun_d_db8_slot: &mut f64,
        var_qjun_d_db9_slot: &mut f64,
        var_qjun_d_dn0_slot: &mut f64,
        var_qjun_d_dn1_slot: &mut f64,
        var_qjun_d_dn10_slot: &mut f64,
        var_qjun_d_dn11_slot: &mut f64,
        var_qjun_d_dn12_slot: &mut f64,
        var_qjun_d_dn13_slot: &mut f64,
        var_qjun_d_dn14_slot: &mut f64,
        var_qjun_d_dn15_slot: &mut f64,
        var_qjun_d_dn16_slot: &mut f64,
        var_qjun_d_dn17_slot: &mut f64,
        var_qjun_d_dn18_slot: &mut f64,
        var_qjun_d_dn19_slot: &mut f64,
        var_qjun_d_dn2_slot: &mut f64,
        var_qjun_d_dn20_slot: &mut f64,
        var_qjun_d_dn3_slot: &mut f64,
        var_qjun_d_dn4_slot: &mut f64,
        var_qjun_d_dn5_slot: &mut f64,
        var_qjun_d_dn6_slot: &mut f64,
        var_qjun_d_dn7_slot: &mut f64,
        var_qjun_d_dn8_slot: &mut f64,
        var_qjun_d_dn9_slot: &mut f64,
        var_qjun_s_slot: &mut f64,
        var_qjun_s_db0_slot: &mut f64,
        var_qjun_s_db1_slot: &mut f64,
        var_qjun_s_db10_slot: &mut f64,
        var_qjun_s_db11_slot: &mut f64,
        var_qjun_s_db12_slot: &mut f64,
        var_qjun_s_db13_slot: &mut f64,
        var_qjun_s_db14_slot: &mut f64,
        var_qjun_s_db15_slot: &mut f64,
        var_qjun_s_db16_slot: &mut f64,
        var_qjun_s_db17_slot: &mut f64,
        var_qjun_s_db18_slot: &mut f64,
        var_qjun_s_db19_slot: &mut f64,
        var_qjun_s_db2_slot: &mut f64,
        var_qjun_s_db20_slot: &mut f64,
        var_qjun_s_db21_slot: &mut f64,
        var_qjun_s_db22_slot: &mut f64,
        var_qjun_s_db23_slot: &mut f64,
        var_qjun_s_db24_slot: &mut f64,
        var_qjun_s_db3_slot: &mut f64,
        var_qjun_s_db4_slot: &mut f64,
        var_qjun_s_db5_slot: &mut f64,
        var_qjun_s_db6_slot: &mut f64,
        var_qjun_s_db7_slot: &mut f64,
        var_qjun_s_db8_slot: &mut f64,
        var_qjun_s_db9_slot: &mut f64,
        var_qjun_s_dn0_slot: &mut f64,
        var_qjun_s_dn1_slot: &mut f64,
        var_qjun_s_dn10_slot: &mut f64,
        var_qjun_s_dn11_slot: &mut f64,
        var_qjun_s_dn12_slot: &mut f64,
        var_qjun_s_dn13_slot: &mut f64,
        var_qjun_s_dn14_slot: &mut f64,
        var_qjun_s_dn15_slot: &mut f64,
        var_qjun_s_dn16_slot: &mut f64,
        var_qjun_s_dn17_slot: &mut f64,
        var_qjun_s_dn18_slot: &mut f64,
        var_qjun_s_dn19_slot: &mut f64,
        var_qjun_s_dn2_slot: &mut f64,
        var_qjun_s_dn20_slot: &mut f64,
        var_qjun_s_dn3_slot: &mut f64,
        var_qjun_s_dn4_slot: &mut f64,
        var_qjun_s_dn5_slot: &mut f64,
        var_qjun_s_dn6_slot: &mut f64,
        var_qjun_s_dn7_slot: &mut f64,
        var_qjun_s_dn8_slot: &mut f64,
        var_qjun_s_dn9_slot: &mut f64,
    ) {
        let mut var_qfgd: f64 = *var_qfgd_slot;
        let mut var_qfgd_db0: f64 = *var_qfgd_db0_slot;
        let mut var_qfgd_db1: f64 = *var_qfgd_db1_slot;
        let mut var_qfgd_db10: f64 = *var_qfgd_db10_slot;
        let mut var_qfgd_db11: f64 = *var_qfgd_db11_slot;
        let mut var_qfgd_db12: f64 = *var_qfgd_db12_slot;
        let mut var_qfgd_db13: f64 = *var_qfgd_db13_slot;
        let mut var_qfgd_db14: f64 = *var_qfgd_db14_slot;
        let mut var_qfgd_db15: f64 = *var_qfgd_db15_slot;
        let mut var_qfgd_db16: f64 = *var_qfgd_db16_slot;
        let mut var_qfgd_db17: f64 = *var_qfgd_db17_slot;
        let mut var_qfgd_db18: f64 = *var_qfgd_db18_slot;
        let mut var_qfgd_db19: f64 = *var_qfgd_db19_slot;
        let mut var_qfgd_db2: f64 = *var_qfgd_db2_slot;
        let mut var_qfgd_db20: f64 = *var_qfgd_db20_slot;
        let mut var_qfgd_db21: f64 = *var_qfgd_db21_slot;
        let mut var_qfgd_db22: f64 = *var_qfgd_db22_slot;
        let mut var_qfgd_db23: f64 = *var_qfgd_db23_slot;
        let mut var_qfgd_db24: f64 = *var_qfgd_db24_slot;
        let mut var_qfgd_db3: f64 = *var_qfgd_db3_slot;
        let mut var_qfgd_db4: f64 = *var_qfgd_db4_slot;
        let mut var_qfgd_db5: f64 = *var_qfgd_db5_slot;
        let mut var_qfgd_db6: f64 = *var_qfgd_db6_slot;
        let mut var_qfgd_db7: f64 = *var_qfgd_db7_slot;
        let mut var_qfgd_db8: f64 = *var_qfgd_db8_slot;
        let mut var_qfgd_db9: f64 = *var_qfgd_db9_slot;
        let mut var_qfgd_dn0: f64 = *var_qfgd_dn0_slot;
        let mut var_qfgd_dn1: f64 = *var_qfgd_dn1_slot;
        let mut var_qfgd_dn10: f64 = *var_qfgd_dn10_slot;
        let mut var_qfgd_dn11: f64 = *var_qfgd_dn11_slot;
        let mut var_qfgd_dn12: f64 = *var_qfgd_dn12_slot;
        let mut var_qfgd_dn13: f64 = *var_qfgd_dn13_slot;
        let mut var_qfgd_dn14: f64 = *var_qfgd_dn14_slot;
        let mut var_qfgd_dn15: f64 = *var_qfgd_dn15_slot;
        let mut var_qfgd_dn16: f64 = *var_qfgd_dn16_slot;
        let mut var_qfgd_dn17: f64 = *var_qfgd_dn17_slot;
        let mut var_qfgd_dn18: f64 = *var_qfgd_dn18_slot;
        let mut var_qfgd_dn19: f64 = *var_qfgd_dn19_slot;
        let mut var_qfgd_dn2: f64 = *var_qfgd_dn2_slot;
        let mut var_qfgd_dn20: f64 = *var_qfgd_dn20_slot;
        let mut var_qfgd_dn3: f64 = *var_qfgd_dn3_slot;
        let mut var_qfgd_dn4: f64 = *var_qfgd_dn4_slot;
        let mut var_qfgd_dn5: f64 = *var_qfgd_dn5_slot;
        let mut var_qfgd_dn6: f64 = *var_qfgd_dn6_slot;
        let mut var_qfgd_dn7: f64 = *var_qfgd_dn7_slot;
        let mut var_qfgd_dn8: f64 = *var_qfgd_dn8_slot;
        let mut var_qfgd_dn9: f64 = *var_qfgd_dn9_slot;
        let mut var_qfgs: f64 = *var_qfgs_slot;
        let mut var_qfgs_db0: f64 = *var_qfgs_db0_slot;
        let mut var_qfgs_db1: f64 = *var_qfgs_db1_slot;
        let mut var_qfgs_db10: f64 = *var_qfgs_db10_slot;
        let mut var_qfgs_db11: f64 = *var_qfgs_db11_slot;
        let mut var_qfgs_db12: f64 = *var_qfgs_db12_slot;
        let mut var_qfgs_db13: f64 = *var_qfgs_db13_slot;
        let mut var_qfgs_db14: f64 = *var_qfgs_db14_slot;
        let mut var_qfgs_db15: f64 = *var_qfgs_db15_slot;
        let mut var_qfgs_db16: f64 = *var_qfgs_db16_slot;
        let mut var_qfgs_db17: f64 = *var_qfgs_db17_slot;
        let mut var_qfgs_db18: f64 = *var_qfgs_db18_slot;
        let mut var_qfgs_db19: f64 = *var_qfgs_db19_slot;
        let mut var_qfgs_db2: f64 = *var_qfgs_db2_slot;
        let mut var_qfgs_db20: f64 = *var_qfgs_db20_slot;
        let mut var_qfgs_db21: f64 = *var_qfgs_db21_slot;
        let mut var_qfgs_db22: f64 = *var_qfgs_db22_slot;
        let mut var_qfgs_db23: f64 = *var_qfgs_db23_slot;
        let mut var_qfgs_db24: f64 = *var_qfgs_db24_slot;
        let mut var_qfgs_db3: f64 = *var_qfgs_db3_slot;
        let mut var_qfgs_db4: f64 = *var_qfgs_db4_slot;
        let mut var_qfgs_db5: f64 = *var_qfgs_db5_slot;
        let mut var_qfgs_db6: f64 = *var_qfgs_db6_slot;
        let mut var_qfgs_db7: f64 = *var_qfgs_db7_slot;
        let mut var_qfgs_db8: f64 = *var_qfgs_db8_slot;
        let mut var_qfgs_db9: f64 = *var_qfgs_db9_slot;
        let mut var_qfgs_dn0: f64 = *var_qfgs_dn0_slot;
        let mut var_qfgs_dn1: f64 = *var_qfgs_dn1_slot;
        let mut var_qfgs_dn10: f64 = *var_qfgs_dn10_slot;
        let mut var_qfgs_dn11: f64 = *var_qfgs_dn11_slot;
        let mut var_qfgs_dn12: f64 = *var_qfgs_dn12_slot;
        let mut var_qfgs_dn13: f64 = *var_qfgs_dn13_slot;
        let mut var_qfgs_dn14: f64 = *var_qfgs_dn14_slot;
        let mut var_qfgs_dn15: f64 = *var_qfgs_dn15_slot;
        let mut var_qfgs_dn16: f64 = *var_qfgs_dn16_slot;
        let mut var_qfgs_dn17: f64 = *var_qfgs_dn17_slot;
        let mut var_qfgs_dn18: f64 = *var_qfgs_dn18_slot;
        let mut var_qfgs_dn19: f64 = *var_qfgs_dn19_slot;
        let mut var_qfgs_dn2: f64 = *var_qfgs_dn2_slot;
        let mut var_qfgs_dn20: f64 = *var_qfgs_dn20_slot;
        let mut var_qfgs_dn3: f64 = *var_qfgs_dn3_slot;
        let mut var_qfgs_dn4: f64 = *var_qfgs_dn4_slot;
        let mut var_qfgs_dn5: f64 = *var_qfgs_dn5_slot;
        let mut var_qfgs_dn6: f64 = *var_qfgs_dn6_slot;
        let mut var_qfgs_dn7: f64 = *var_qfgs_dn7_slot;
        let mut var_qfgs_dn8: f64 = *var_qfgs_dn8_slot;
        let mut var_qfgs_dn9: f64 = *var_qfgs_dn9_slot;
        let mut var_qjun_d: f64 = *var_qjun_d_slot;
        let mut var_qjun_d_db0: f64 = *var_qjun_d_db0_slot;
        let mut var_qjun_d_db1: f64 = *var_qjun_d_db1_slot;
        let mut var_qjun_d_db10: f64 = *var_qjun_d_db10_slot;
        let mut var_qjun_d_db11: f64 = *var_qjun_d_db11_slot;
        let mut var_qjun_d_db12: f64 = *var_qjun_d_db12_slot;
        let mut var_qjun_d_db13: f64 = *var_qjun_d_db13_slot;
        let mut var_qjun_d_db14: f64 = *var_qjun_d_db14_slot;
        let mut var_qjun_d_db15: f64 = *var_qjun_d_db15_slot;
        let mut var_qjun_d_db16: f64 = *var_qjun_d_db16_slot;
        let mut var_qjun_d_db17: f64 = *var_qjun_d_db17_slot;
        let mut var_qjun_d_db18: f64 = *var_qjun_d_db18_slot;
        let mut var_qjun_d_db19: f64 = *var_qjun_d_db19_slot;
        let mut var_qjun_d_db2: f64 = *var_qjun_d_db2_slot;
        let mut var_qjun_d_db20: f64 = *var_qjun_d_db20_slot;
        let mut var_qjun_d_db21: f64 = *var_qjun_d_db21_slot;
        let mut var_qjun_d_db22: f64 = *var_qjun_d_db22_slot;
        let mut var_qjun_d_db23: f64 = *var_qjun_d_db23_slot;
        let mut var_qjun_d_db24: f64 = *var_qjun_d_db24_slot;
        let mut var_qjun_d_db3: f64 = *var_qjun_d_db3_slot;
        let mut var_qjun_d_db4: f64 = *var_qjun_d_db4_slot;
        let mut var_qjun_d_db5: f64 = *var_qjun_d_db5_slot;
        let mut var_qjun_d_db6: f64 = *var_qjun_d_db6_slot;
        let mut var_qjun_d_db7: f64 = *var_qjun_d_db7_slot;
        let mut var_qjun_d_db8: f64 = *var_qjun_d_db8_slot;
        let mut var_qjun_d_db9: f64 = *var_qjun_d_db9_slot;
        let mut var_qjun_d_dn0: f64 = *var_qjun_d_dn0_slot;
        let mut var_qjun_d_dn1: f64 = *var_qjun_d_dn1_slot;
        let mut var_qjun_d_dn10: f64 = *var_qjun_d_dn10_slot;
        let mut var_qjun_d_dn11: f64 = *var_qjun_d_dn11_slot;
        let mut var_qjun_d_dn12: f64 = *var_qjun_d_dn12_slot;
        let mut var_qjun_d_dn13: f64 = *var_qjun_d_dn13_slot;
        let mut var_qjun_d_dn14: f64 = *var_qjun_d_dn14_slot;
        let mut var_qjun_d_dn15: f64 = *var_qjun_d_dn15_slot;
        let mut var_qjun_d_dn16: f64 = *var_qjun_d_dn16_slot;
        let mut var_qjun_d_dn17: f64 = *var_qjun_d_dn17_slot;
        let mut var_qjun_d_dn18: f64 = *var_qjun_d_dn18_slot;
        let mut var_qjun_d_dn19: f64 = *var_qjun_d_dn19_slot;
        let mut var_qjun_d_dn2: f64 = *var_qjun_d_dn2_slot;
        let mut var_qjun_d_dn20: f64 = *var_qjun_d_dn20_slot;
        let mut var_qjun_d_dn3: f64 = *var_qjun_d_dn3_slot;
        let mut var_qjun_d_dn4: f64 = *var_qjun_d_dn4_slot;
        let mut var_qjun_d_dn5: f64 = *var_qjun_d_dn5_slot;
        let mut var_qjun_d_dn6: f64 = *var_qjun_d_dn6_slot;
        let mut var_qjun_d_dn7: f64 = *var_qjun_d_dn7_slot;
        let mut var_qjun_d_dn8: f64 = *var_qjun_d_dn8_slot;
        let mut var_qjun_d_dn9: f64 = *var_qjun_d_dn9_slot;
        let mut var_qjun_s: f64 = *var_qjun_s_slot;
        let mut var_qjun_s_db0: f64 = *var_qjun_s_db0_slot;
        let mut var_qjun_s_db1: f64 = *var_qjun_s_db1_slot;
        let mut var_qjun_s_db10: f64 = *var_qjun_s_db10_slot;
        let mut var_qjun_s_db11: f64 = *var_qjun_s_db11_slot;
        let mut var_qjun_s_db12: f64 = *var_qjun_s_db12_slot;
        let mut var_qjun_s_db13: f64 = *var_qjun_s_db13_slot;
        let mut var_qjun_s_db14: f64 = *var_qjun_s_db14_slot;
        let mut var_qjun_s_db15: f64 = *var_qjun_s_db15_slot;
        let mut var_qjun_s_db16: f64 = *var_qjun_s_db16_slot;
        let mut var_qjun_s_db17: f64 = *var_qjun_s_db17_slot;
        let mut var_qjun_s_db18: f64 = *var_qjun_s_db18_slot;
        let mut var_qjun_s_db19: f64 = *var_qjun_s_db19_slot;
        let mut var_qjun_s_db2: f64 = *var_qjun_s_db2_slot;
        let mut var_qjun_s_db20: f64 = *var_qjun_s_db20_slot;
        let mut var_qjun_s_db21: f64 = *var_qjun_s_db21_slot;
        let mut var_qjun_s_db22: f64 = *var_qjun_s_db22_slot;
        let mut var_qjun_s_db23: f64 = *var_qjun_s_db23_slot;
        let mut var_qjun_s_db24: f64 = *var_qjun_s_db24_slot;
        let mut var_qjun_s_db3: f64 = *var_qjun_s_db3_slot;
        let mut var_qjun_s_db4: f64 = *var_qjun_s_db4_slot;
        let mut var_qjun_s_db5: f64 = *var_qjun_s_db5_slot;
        let mut var_qjun_s_db6: f64 = *var_qjun_s_db6_slot;
        let mut var_qjun_s_db7: f64 = *var_qjun_s_db7_slot;
        let mut var_qjun_s_db8: f64 = *var_qjun_s_db8_slot;
        let mut var_qjun_s_db9: f64 = *var_qjun_s_db9_slot;
        let mut var_qjun_s_dn0: f64 = *var_qjun_s_dn0_slot;
        let mut var_qjun_s_dn1: f64 = *var_qjun_s_dn1_slot;
        let mut var_qjun_s_dn10: f64 = *var_qjun_s_dn10_slot;
        let mut var_qjun_s_dn11: f64 = *var_qjun_s_dn11_slot;
        let mut var_qjun_s_dn12: f64 = *var_qjun_s_dn12_slot;
        let mut var_qjun_s_dn13: f64 = *var_qjun_s_dn13_slot;
        let mut var_qjun_s_dn14: f64 = *var_qjun_s_dn14_slot;
        let mut var_qjun_s_dn15: f64 = *var_qjun_s_dn15_slot;
        let mut var_qjun_s_dn16: f64 = *var_qjun_s_dn16_slot;
        let mut var_qjun_s_dn17: f64 = *var_qjun_s_dn17_slot;
        let mut var_qjun_s_dn18: f64 = *var_qjun_s_dn18_slot;
        let mut var_qjun_s_dn19: f64 = *var_qjun_s_dn19_slot;
        let mut var_qjun_s_dn2: f64 = *var_qjun_s_dn2_slot;
        let mut var_qjun_s_dn20: f64 = *var_qjun_s_dn20_slot;
        let mut var_qjun_s_dn3: f64 = *var_qjun_s_dn3_slot;
        let mut var_qjun_s_dn4: f64 = *var_qjun_s_dn4_slot;
        let mut var_qjun_s_dn5: f64 = *var_qjun_s_dn5_slot;
        let mut var_qjun_s_dn6: f64 = *var_qjun_s_dn6_slot;
        let mut var_qjun_s_dn7: f64 = *var_qjun_s_dn7_slot;
        let mut var_qjun_s_dn8: f64 = *var_qjun_s_dn8_slot;
        let mut var_qjun_s_dn9: f64 = *var_qjun_s_dn9_slot;

        let assign82720_e124467: f64 = (var_qfgs + var_qgs_ov);
        var_qfgs = assign82720_e124467;
        var_qfgs_dn0 = (var_qfgs_dn0 + var_qgs_ov_dn0);
        var_qfgs_dn1 = (var_qfgs_dn1 + var_qgs_ov_dn1);
        var_qfgs_dn2 = (var_qfgs_dn2 + var_qgs_ov_dn2);
        var_qfgs_dn3 = (var_qfgs_dn3 + var_qgs_ov_dn3);
        var_qfgs_dn4 = (var_qfgs_dn4 + var_qgs_ov_dn4);
        var_qfgs_dn5 = (var_qfgs_dn5 + var_qgs_ov_dn5);
        var_qfgs_dn6 = (var_qfgs_dn6 + var_qgs_ov_dn6);
        var_qfgs_dn7 = (var_qfgs_dn7 + var_qgs_ov_dn7);
        var_qfgs_dn8 = (var_qfgs_dn8 + var_qgs_ov_dn8);
        var_qfgs_dn9 = (var_qfgs_dn9 + var_qgs_ov_dn9);
        var_qfgs_dn10 = (var_qfgs_dn10 + var_qgs_ov_dn10);
        var_qfgs_dn11 = (var_qfgs_dn11 + var_qgs_ov_dn11);
        var_qfgs_dn12 = (var_qfgs_dn12 + var_qgs_ov_dn12);
        var_qfgs_dn13 = (var_qfgs_dn13 + var_qgs_ov_dn13);
        var_qfgs_dn14 = (var_qfgs_dn14 + var_qgs_ov_dn14);
        var_qfgs_dn15 = (var_qfgs_dn15 + var_qgs_ov_dn15);
        var_qfgs_dn16 = (var_qfgs_dn16 + var_qgs_ov_dn16);
        var_qfgs_dn17 = (var_qfgs_dn17 + var_qgs_ov_dn17);
        var_qfgs_dn18 = (var_qfgs_dn18 + var_qgs_ov_dn18);
        var_qfgs_dn19 = (var_qfgs_dn19 + var_qgs_ov_dn19);
        var_qfgs_dn20 = (var_qfgs_dn20 + var_qgs_ov_dn20);
        var_qfgs_db0 = (var_qfgs_db0 + var_qgs_ov_db0);
        var_qfgs_db1 = (var_qfgs_db1 + var_qgs_ov_db1);
        var_qfgs_db2 = (var_qfgs_db2 + var_qgs_ov_db2);
        var_qfgs_db3 = (var_qfgs_db3 + var_qgs_ov_db3);
        var_qfgs_db4 = (var_qfgs_db4 + var_qgs_ov_db4);
        var_qfgs_db5 = (var_qfgs_db5 + var_qgs_ov_db5);
        var_qfgs_db6 = (var_qfgs_db6 + var_qgs_ov_db6);
        var_qfgs_db7 = (var_qfgs_db7 + var_qgs_ov_db7);
        var_qfgs_db8 = (var_qfgs_db8 + var_qgs_ov_db8);
        var_qfgs_db9 = (var_qfgs_db9 + var_qgs_ov_db9);
        var_qfgs_db10 = (var_qfgs_db10 + var_qgs_ov_db10);
        var_qfgs_db11 = (var_qfgs_db11 + var_qgs_ov_db11);
        var_qfgs_db12 = (var_qfgs_db12 + var_qgs_ov_db12);
        var_qfgs_db13 = (var_qfgs_db13 + var_qgs_ov_db13);
        var_qfgs_db14 = (var_qfgs_db14 + var_qgs_ov_db14);
        var_qfgs_db15 = (var_qfgs_db15 + var_qgs_ov_db15);
        var_qfgs_db16 = (var_qfgs_db16 + var_qgs_ov_db16);
        var_qfgs_db17 = (var_qfgs_db17 + var_qgs_ov_db17);
        var_qfgs_db18 = (var_qfgs_db18 + var_qgs_ov_db18);
        var_qfgs_db19 = (var_qfgs_db19 + var_qgs_ov_db19);
        var_qfgs_db20 = (var_qfgs_db20 + var_qgs_ov_db20);
        var_qfgs_db21 = (var_qfgs_db21 + var_qgs_ov_db21);
        var_qfgs_db22 = (var_qfgs_db22 + var_qgs_ov_db22);
        var_qfgs_db23 = (var_qfgs_db23 + var_qgs_ov_db23);
        var_qfgs_db24 = (var_qfgs_db24 + var_qgs_ov_db24);

        let assign82730_e124470: f64 = (var_qfgd + var_qgd_ov);
        var_qfgd = assign82730_e124470;
        var_qfgd_dn0 = (var_qfgd_dn0 + var_qgd_ov_dn0);
        var_qfgd_dn1 = (var_qfgd_dn1 + var_qgd_ov_dn1);
        var_qfgd_dn2 = (var_qfgd_dn2 + var_qgd_ov_dn2);
        var_qfgd_dn3 = (var_qfgd_dn3 + var_qgd_ov_dn3);
        var_qfgd_dn4 = (var_qfgd_dn4 + var_qgd_ov_dn4);
        var_qfgd_dn5 = (var_qfgd_dn5 + var_qgd_ov_dn5);
        var_qfgd_dn6 = (var_qfgd_dn6 + var_qgd_ov_dn6);
        var_qfgd_dn7 = (var_qfgd_dn7 + var_qgd_ov_dn7);
        var_qfgd_dn8 = (var_qfgd_dn8 + var_qgd_ov_dn8);
        var_qfgd_dn9 = (var_qfgd_dn9 + var_qgd_ov_dn9);
        var_qfgd_dn10 = (var_qfgd_dn10 + var_qgd_ov_dn10);
        var_qfgd_dn11 = (var_qfgd_dn11 + var_qgd_ov_dn11);
        var_qfgd_dn12 = (var_qfgd_dn12 + var_qgd_ov_dn12);
        var_qfgd_dn13 = (var_qfgd_dn13 + var_qgd_ov_dn13);
        var_qfgd_dn14 = (var_qfgd_dn14 + var_qgd_ov_dn14);
        var_qfgd_dn15 = (var_qfgd_dn15 + var_qgd_ov_dn15);
        var_qfgd_dn16 = (var_qfgd_dn16 + var_qgd_ov_dn16);
        var_qfgd_dn17 = (var_qfgd_dn17 + var_qgd_ov_dn17);
        var_qfgd_dn18 = (var_qfgd_dn18 + var_qgd_ov_dn18);
        var_qfgd_dn19 = (var_qfgd_dn19 + var_qgd_ov_dn19);
        var_qfgd_dn20 = (var_qfgd_dn20 + var_qgd_ov_dn20);
        var_qfgd_db0 = (var_qfgd_db0 + var_qgd_ov_db0);
        var_qfgd_db1 = (var_qfgd_db1 + var_qgd_ov_db1);
        var_qfgd_db2 = (var_qfgd_db2 + var_qgd_ov_db2);
        var_qfgd_db3 = (var_qfgd_db3 + var_qgd_ov_db3);
        var_qfgd_db4 = (var_qfgd_db4 + var_qgd_ov_db4);
        var_qfgd_db5 = (var_qfgd_db5 + var_qgd_ov_db5);
        var_qfgd_db6 = (var_qfgd_db6 + var_qgd_ov_db6);
        var_qfgd_db7 = (var_qfgd_db7 + var_qgd_ov_db7);
        var_qfgd_db8 = (var_qfgd_db8 + var_qgd_ov_db8);
        var_qfgd_db9 = (var_qfgd_db9 + var_qgd_ov_db9);
        var_qfgd_db10 = (var_qfgd_db10 + var_qgd_ov_db10);
        var_qfgd_db11 = (var_qfgd_db11 + var_qgd_ov_db11);
        var_qfgd_db12 = (var_qfgd_db12 + var_qgd_ov_db12);
        var_qfgd_db13 = (var_qfgd_db13 + var_qgd_ov_db13);
        var_qfgd_db14 = (var_qfgd_db14 + var_qgd_ov_db14);
        var_qfgd_db15 = (var_qfgd_db15 + var_qgd_ov_db15);
        var_qfgd_db16 = (var_qfgd_db16 + var_qgd_ov_db16);
        var_qfgd_db17 = (var_qfgd_db17 + var_qgd_ov_db17);
        var_qfgd_db18 = (var_qfgd_db18 + var_qgd_ov_db18);
        var_qfgd_db19 = (var_qfgd_db19 + var_qgd_ov_db19);
        var_qfgd_db20 = (var_qfgd_db20 + var_qgd_ov_db20);
        var_qfgd_db21 = (var_qfgd_db21 + var_qgd_ov_db21);
        var_qfgd_db22 = (var_qfgd_db22 + var_qgd_ov_db22);
        var_qfgd_db23 = (var_qfgd_db23 + var_qgd_ov_db23);
        var_qfgd_db24 = (var_qfgd_db24 + var_qgd_ov_db24);

        let assign82740_e124473: f64 = (var_absource_i * var_qjunbot_s);
        let assign82740_e124476: f64 = (var_lssource_i * var_qjunsti_s);
        let assign82740_e124477: f64 = (assign82740_e124473 + assign82740_e124476);
        let assign82740_e124480: f64 = (var_lgsource_i * var_qjungat_s);
        let assign82740_e124481: f64 = (assign82740_e124477 + assign82740_e124480);
        var_qjun_s = assign82740_e124481;
        var_qjun_s_dn0 = (((var_absource_i * var_qjunbot_s_dn0) + (var_lssource_i * var_qjunsti_s_dn0)) + (var_lgsource_i * var_qjungat_s_dn0));
        var_qjun_s_dn1 = (((var_absource_i * var_qjunbot_s_dn1) + (var_lssource_i * var_qjunsti_s_dn1)) + (var_lgsource_i * var_qjungat_s_dn1));
        var_qjun_s_dn2 = (((var_absource_i * var_qjunbot_s_dn2) + (var_lssource_i * var_qjunsti_s_dn2)) + (var_lgsource_i * var_qjungat_s_dn2));
        var_qjun_s_dn3 = (((var_absource_i * var_qjunbot_s_dn3) + (var_lssource_i * var_qjunsti_s_dn3)) + (var_lgsource_i * var_qjungat_s_dn3));
        var_qjun_s_dn4 = (((var_absource_i * var_qjunbot_s_dn4) + (var_lssource_i * var_qjunsti_s_dn4)) + (var_lgsource_i * var_qjungat_s_dn4));
        var_qjun_s_dn5 = (((var_absource_i * var_qjunbot_s_dn5) + (var_lssource_i * var_qjunsti_s_dn5)) + (var_lgsource_i * var_qjungat_s_dn5));
        var_qjun_s_dn6 = (((var_absource_i * var_qjunbot_s_dn6) + (var_lssource_i * var_qjunsti_s_dn6)) + (var_lgsource_i * var_qjungat_s_dn6));
        var_qjun_s_dn7 = (((var_absource_i * var_qjunbot_s_dn7) + (var_lssource_i * var_qjunsti_s_dn7)) + (var_lgsource_i * var_qjungat_s_dn7));
        var_qjun_s_dn8 = (((var_absource_i * var_qjunbot_s_dn8) + (var_lssource_i * var_qjunsti_s_dn8)) + (var_lgsource_i * var_qjungat_s_dn8));
        var_qjun_s_dn9 = (((var_absource_i * var_qjunbot_s_dn9) + (var_lssource_i * var_qjunsti_s_dn9)) + (var_lgsource_i * var_qjungat_s_dn9));
        var_qjun_s_dn10 = (((var_absource_i * var_qjunbot_s_dn10) + (var_lssource_i * var_qjunsti_s_dn10)) + (var_lgsource_i * var_qjungat_s_dn10));
        var_qjun_s_dn11 = (((var_absource_i * var_qjunbot_s_dn11) + (var_lssource_i * var_qjunsti_s_dn11)) + (var_lgsource_i * var_qjungat_s_dn11));
        var_qjun_s_dn12 = (((var_absource_i * var_qjunbot_s_dn12) + (var_lssource_i * var_qjunsti_s_dn12)) + (var_lgsource_i * var_qjungat_s_dn12));
        var_qjun_s_dn13 = (((var_absource_i * var_qjunbot_s_dn13) + (var_lssource_i * var_qjunsti_s_dn13)) + (var_lgsource_i * var_qjungat_s_dn13));
        var_qjun_s_dn14 = (((var_absource_i * var_qjunbot_s_dn14) + (var_lssource_i * var_qjunsti_s_dn14)) + (var_lgsource_i * var_qjungat_s_dn14));
        var_qjun_s_dn15 = (((var_absource_i * var_qjunbot_s_dn15) + (var_lssource_i * var_qjunsti_s_dn15)) + (var_lgsource_i * var_qjungat_s_dn15));
        var_qjun_s_dn16 = (((var_absource_i * var_qjunbot_s_dn16) + (var_lssource_i * var_qjunsti_s_dn16)) + (var_lgsource_i * var_qjungat_s_dn16));
        var_qjun_s_dn17 = (((var_absource_i * var_qjunbot_s_dn17) + (var_lssource_i * var_qjunsti_s_dn17)) + (var_lgsource_i * var_qjungat_s_dn17));
        var_qjun_s_dn18 = (((var_absource_i * var_qjunbot_s_dn18) + (var_lssource_i * var_qjunsti_s_dn18)) + (var_lgsource_i * var_qjungat_s_dn18));
        var_qjun_s_dn19 = (((var_absource_i * var_qjunbot_s_dn19) + (var_lssource_i * var_qjunsti_s_dn19)) + (var_lgsource_i * var_qjungat_s_dn19));
        var_qjun_s_dn20 = (((var_absource_i * var_qjunbot_s_dn20) + (var_lssource_i * var_qjunsti_s_dn20)) + (var_lgsource_i * var_qjungat_s_dn20));
        var_qjun_s_db0 = (((var_absource_i * var_qjunbot_s_db0) + (var_lssource_i * var_qjunsti_s_db0)) + (var_lgsource_i * var_qjungat_s_db0));
        var_qjun_s_db1 = (((var_absource_i * var_qjunbot_s_db1) + (var_lssource_i * var_qjunsti_s_db1)) + (var_lgsource_i * var_qjungat_s_db1));
        var_qjun_s_db2 = (((var_absource_i * var_qjunbot_s_db2) + (var_lssource_i * var_qjunsti_s_db2)) + (var_lgsource_i * var_qjungat_s_db2));
        var_qjun_s_db3 = (((var_absource_i * var_qjunbot_s_db3) + (var_lssource_i * var_qjunsti_s_db3)) + (var_lgsource_i * var_qjungat_s_db3));
        var_qjun_s_db4 = (((var_absource_i * var_qjunbot_s_db4) + (var_lssource_i * var_qjunsti_s_db4)) + (var_lgsource_i * var_qjungat_s_db4));
        var_qjun_s_db5 = (((var_absource_i * var_qjunbot_s_db5) + (var_lssource_i * var_qjunsti_s_db5)) + (var_lgsource_i * var_qjungat_s_db5));
        var_qjun_s_db6 = (((var_absource_i * var_qjunbot_s_db6) + (var_lssource_i * var_qjunsti_s_db6)) + (var_lgsource_i * var_qjungat_s_db6));
        var_qjun_s_db7 = (((var_absource_i * var_qjunbot_s_db7) + (var_lssource_i * var_qjunsti_s_db7)) + (var_lgsource_i * var_qjungat_s_db7));
        var_qjun_s_db8 = (((var_absource_i * var_qjunbot_s_db8) + (var_lssource_i * var_qjunsti_s_db8)) + (var_lgsource_i * var_qjungat_s_db8));
        var_qjun_s_db9 = (((var_absource_i * var_qjunbot_s_db9) + (var_lssource_i * var_qjunsti_s_db9)) + (var_lgsource_i * var_qjungat_s_db9));
        var_qjun_s_db10 = (((var_absource_i * var_qjunbot_s_db10) + (var_lssource_i * var_qjunsti_s_db10)) + (var_lgsource_i * var_qjungat_s_db10));
        var_qjun_s_db11 = (((var_absource_i * var_qjunbot_s_db11) + (var_lssource_i * var_qjunsti_s_db11)) + (var_lgsource_i * var_qjungat_s_db11));
        var_qjun_s_db12 = (((var_absource_i * var_qjunbot_s_db12) + (var_lssource_i * var_qjunsti_s_db12)) + (var_lgsource_i * var_qjungat_s_db12));
        var_qjun_s_db13 = (((var_absource_i * var_qjunbot_s_db13) + (var_lssource_i * var_qjunsti_s_db13)) + (var_lgsource_i * var_qjungat_s_db13));
        var_qjun_s_db14 = (((var_absource_i * var_qjunbot_s_db14) + (var_lssource_i * var_qjunsti_s_db14)) + (var_lgsource_i * var_qjungat_s_db14));
        var_qjun_s_db15 = (((var_absource_i * var_qjunbot_s_db15) + (var_lssource_i * var_qjunsti_s_db15)) + (var_lgsource_i * var_qjungat_s_db15));
        var_qjun_s_db16 = (((var_absource_i * var_qjunbot_s_db16) + (var_lssource_i * var_qjunsti_s_db16)) + (var_lgsource_i * var_qjungat_s_db16));
        var_qjun_s_db17 = (((var_absource_i * var_qjunbot_s_db17) + (var_lssource_i * var_qjunsti_s_db17)) + (var_lgsource_i * var_qjungat_s_db17));
        var_qjun_s_db18 = (((var_absource_i * var_qjunbot_s_db18) + (var_lssource_i * var_qjunsti_s_db18)) + (var_lgsource_i * var_qjungat_s_db18));
        var_qjun_s_db19 = (((var_absource_i * var_qjunbot_s_db19) + (var_lssource_i * var_qjunsti_s_db19)) + (var_lgsource_i * var_qjungat_s_db19));
        var_qjun_s_db20 = (((var_absource_i * var_qjunbot_s_db20) + (var_lssource_i * var_qjunsti_s_db20)) + (var_lgsource_i * var_qjungat_s_db20));
        var_qjun_s_db21 = (((var_absource_i * var_qjunbot_s_db21) + (var_lssource_i * var_qjunsti_s_db21)) + (var_lgsource_i * var_qjungat_s_db21));
        var_qjun_s_db22 = (((var_absource_i * var_qjunbot_s_db22) + (var_lssource_i * var_qjunsti_s_db22)) + (var_lgsource_i * var_qjungat_s_db22));
        var_qjun_s_db23 = (((var_absource_i * var_qjunbot_s_db23) + (var_lssource_i * var_qjunsti_s_db23)) + (var_lgsource_i * var_qjungat_s_db23));
        var_qjun_s_db24 = (((var_absource_i * var_qjunbot_s_db24) + (var_lssource_i * var_qjunsti_s_db24)) + (var_lgsource_i * var_qjungat_s_db24));

        let assign82750_e124484: f64 = (var_abdrain_i * var_qjunbot_d);
        let assign82750_e124487: f64 = (var_lsdrain_i * var_qjunsti_d);
        let assign82750_e124488: f64 = (assign82750_e124484 + assign82750_e124487);
        let assign82750_e124491: f64 = (var_lgdrain_i * var_qjungat_d);
        let assign82750_e124492: f64 = (assign82750_e124488 + assign82750_e124491);
        var_qjun_d = assign82750_e124492;
        var_qjun_d_dn0 = (((var_abdrain_i * var_qjunbot_d_dn0) + (var_lsdrain_i * var_qjunsti_d_dn0)) + (var_lgdrain_i * var_qjungat_d_dn0));
        var_qjun_d_dn1 = (((var_abdrain_i * var_qjunbot_d_dn1) + (var_lsdrain_i * var_qjunsti_d_dn1)) + (var_lgdrain_i * var_qjungat_d_dn1));
        var_qjun_d_dn2 = (((var_abdrain_i * var_qjunbot_d_dn2) + (var_lsdrain_i * var_qjunsti_d_dn2)) + (var_lgdrain_i * var_qjungat_d_dn2));
        var_qjun_d_dn3 = (((var_abdrain_i * var_qjunbot_d_dn3) + (var_lsdrain_i * var_qjunsti_d_dn3)) + (var_lgdrain_i * var_qjungat_d_dn3));
        var_qjun_d_dn4 = (((var_abdrain_i * var_qjunbot_d_dn4) + (var_lsdrain_i * var_qjunsti_d_dn4)) + (var_lgdrain_i * var_qjungat_d_dn4));
        var_qjun_d_dn5 = (((var_abdrain_i * var_qjunbot_d_dn5) + (var_lsdrain_i * var_qjunsti_d_dn5)) + (var_lgdrain_i * var_qjungat_d_dn5));
        var_qjun_d_dn6 = (((var_abdrain_i * var_qjunbot_d_dn6) + (var_lsdrain_i * var_qjunsti_d_dn6)) + (var_lgdrain_i * var_qjungat_d_dn6));
        var_qjun_d_dn7 = (((var_abdrain_i * var_qjunbot_d_dn7) + (var_lsdrain_i * var_qjunsti_d_dn7)) + (var_lgdrain_i * var_qjungat_d_dn7));
        var_qjun_d_dn8 = (((var_abdrain_i * var_qjunbot_d_dn8) + (var_lsdrain_i * var_qjunsti_d_dn8)) + (var_lgdrain_i * var_qjungat_d_dn8));
        var_qjun_d_dn9 = (((var_abdrain_i * var_qjunbot_d_dn9) + (var_lsdrain_i * var_qjunsti_d_dn9)) + (var_lgdrain_i * var_qjungat_d_dn9));
        var_qjun_d_dn10 = (((var_abdrain_i * var_qjunbot_d_dn10) + (var_lsdrain_i * var_qjunsti_d_dn10)) + (var_lgdrain_i * var_qjungat_d_dn10));
        var_qjun_d_dn11 = (((var_abdrain_i * var_qjunbot_d_dn11) + (var_lsdrain_i * var_qjunsti_d_dn11)) + (var_lgdrain_i * var_qjungat_d_dn11));
        var_qjun_d_dn12 = (((var_abdrain_i * var_qjunbot_d_dn12) + (var_lsdrain_i * var_qjunsti_d_dn12)) + (var_lgdrain_i * var_qjungat_d_dn12));
        var_qjun_d_dn13 = (((var_abdrain_i * var_qjunbot_d_dn13) + (var_lsdrain_i * var_qjunsti_d_dn13)) + (var_lgdrain_i * var_qjungat_d_dn13));
        var_qjun_d_dn14 = (((var_abdrain_i * var_qjunbot_d_dn14) + (var_lsdrain_i * var_qjunsti_d_dn14)) + (var_lgdrain_i * var_qjungat_d_dn14));
        var_qjun_d_dn15 = (((var_abdrain_i * var_qjunbot_d_dn15) + (var_lsdrain_i * var_qjunsti_d_dn15)) + (var_lgdrain_i * var_qjungat_d_dn15));
        var_qjun_d_dn16 = (((var_abdrain_i * var_qjunbot_d_dn16) + (var_lsdrain_i * var_qjunsti_d_dn16)) + (var_lgdrain_i * var_qjungat_d_dn16));
        var_qjun_d_dn17 = (((var_abdrain_i * var_qjunbot_d_dn17) + (var_lsdrain_i * var_qjunsti_d_dn17)) + (var_lgdrain_i * var_qjungat_d_dn17));
        var_qjun_d_dn18 = (((var_abdrain_i * var_qjunbot_d_dn18) + (var_lsdrain_i * var_qjunsti_d_dn18)) + (var_lgdrain_i * var_qjungat_d_dn18));
        var_qjun_d_dn19 = (((var_abdrain_i * var_qjunbot_d_dn19) + (var_lsdrain_i * var_qjunsti_d_dn19)) + (var_lgdrain_i * var_qjungat_d_dn19));
        var_qjun_d_dn20 = (((var_abdrain_i * var_qjunbot_d_dn20) + (var_lsdrain_i * var_qjunsti_d_dn20)) + (var_lgdrain_i * var_qjungat_d_dn20));
        var_qjun_d_db0 = (((var_abdrain_i * var_qjunbot_d_db0) + (var_lsdrain_i * var_qjunsti_d_db0)) + (var_lgdrain_i * var_qjungat_d_db0));
        var_qjun_d_db1 = (((var_abdrain_i * var_qjunbot_d_db1) + (var_lsdrain_i * var_qjunsti_d_db1)) + (var_lgdrain_i * var_qjungat_d_db1));
        var_qjun_d_db2 = (((var_abdrain_i * var_qjunbot_d_db2) + (var_lsdrain_i * var_qjunsti_d_db2)) + (var_lgdrain_i * var_qjungat_d_db2));
        var_qjun_d_db3 = (((var_abdrain_i * var_qjunbot_d_db3) + (var_lsdrain_i * var_qjunsti_d_db3)) + (var_lgdrain_i * var_qjungat_d_db3));
        var_qjun_d_db4 = (((var_abdrain_i * var_qjunbot_d_db4) + (var_lsdrain_i * var_qjunsti_d_db4)) + (var_lgdrain_i * var_qjungat_d_db4));
        var_qjun_d_db5 = (((var_abdrain_i * var_qjunbot_d_db5) + (var_lsdrain_i * var_qjunsti_d_db5)) + (var_lgdrain_i * var_qjungat_d_db5));
        var_qjun_d_db6 = (((var_abdrain_i * var_qjunbot_d_db6) + (var_lsdrain_i * var_qjunsti_d_db6)) + (var_lgdrain_i * var_qjungat_d_db6));
        var_qjun_d_db7 = (((var_abdrain_i * var_qjunbot_d_db7) + (var_lsdrain_i * var_qjunsti_d_db7)) + (var_lgdrain_i * var_qjungat_d_db7));
        var_qjun_d_db8 = (((var_abdrain_i * var_qjunbot_d_db8) + (var_lsdrain_i * var_qjunsti_d_db8)) + (var_lgdrain_i * var_qjungat_d_db8));
        var_qjun_d_db9 = (((var_abdrain_i * var_qjunbot_d_db9) + (var_lsdrain_i * var_qjunsti_d_db9)) + (var_lgdrain_i * var_qjungat_d_db9));
        var_qjun_d_db10 = (((var_abdrain_i * var_qjunbot_d_db10) + (var_lsdrain_i * var_qjunsti_d_db10)) + (var_lgdrain_i * var_qjungat_d_db10));
        var_qjun_d_db11 = (((var_abdrain_i * var_qjunbot_d_db11) + (var_lsdrain_i * var_qjunsti_d_db11)) + (var_lgdrain_i * var_qjungat_d_db11));
        var_qjun_d_db12 = (((var_abdrain_i * var_qjunbot_d_db12) + (var_lsdrain_i * var_qjunsti_d_db12)) + (var_lgdrain_i * var_qjungat_d_db12));
        var_qjun_d_db13 = (((var_abdrain_i * var_qjunbot_d_db13) + (var_lsdrain_i * var_qjunsti_d_db13)) + (var_lgdrain_i * var_qjungat_d_db13));
        var_qjun_d_db14 = (((var_abdrain_i * var_qjunbot_d_db14) + (var_lsdrain_i * var_qjunsti_d_db14)) + (var_lgdrain_i * var_qjungat_d_db14));
        var_qjun_d_db15 = (((var_abdrain_i * var_qjunbot_d_db15) + (var_lsdrain_i * var_qjunsti_d_db15)) + (var_lgdrain_i * var_qjungat_d_db15));
        var_qjun_d_db16 = (((var_abdrain_i * var_qjunbot_d_db16) + (var_lsdrain_i * var_qjunsti_d_db16)) + (var_lgdrain_i * var_qjungat_d_db16));
        var_qjun_d_db17 = (((var_abdrain_i * var_qjunbot_d_db17) + (var_lsdrain_i * var_qjunsti_d_db17)) + (var_lgdrain_i * var_qjungat_d_db17));
        var_qjun_d_db18 = (((var_abdrain_i * var_qjunbot_d_db18) + (var_lsdrain_i * var_qjunsti_d_db18)) + (var_lgdrain_i * var_qjungat_d_db18));
        var_qjun_d_db19 = (((var_abdrain_i * var_qjunbot_d_db19) + (var_lsdrain_i * var_qjunsti_d_db19)) + (var_lgdrain_i * var_qjungat_d_db19));
        var_qjun_d_db20 = (((var_abdrain_i * var_qjunbot_d_db20) + (var_lsdrain_i * var_qjunsti_d_db20)) + (var_lgdrain_i * var_qjungat_d_db20));
        var_qjun_d_db21 = (((var_abdrain_i * var_qjunbot_d_db21) + (var_lsdrain_i * var_qjunsti_d_db21)) + (var_lgdrain_i * var_qjungat_d_db21));
        var_qjun_d_db22 = (((var_abdrain_i * var_qjunbot_d_db22) + (var_lsdrain_i * var_qjunsti_d_db22)) + (var_lgdrain_i * var_qjungat_d_db22));
        var_qjun_d_db23 = (((var_abdrain_i * var_qjunbot_d_db23) + (var_lsdrain_i * var_qjunsti_d_db23)) + (var_lgdrain_i * var_qjungat_d_db23));
        var_qjun_d_db24 = (((var_abdrain_i * var_qjunbot_d_db24) + (var_lsdrain_i * var_qjunsti_d_db24)) + (var_lgdrain_i * var_qjungat_d_db24));

        s.b[3235] = (s.v[831] < 0.0);
        s.store_scalar(3235, if s.b[3235] { 1.0 } else { 0.0 });

        if s.b[3235] {
            s.copy_ad(3234, 853);
            s.copy_ad(853, 850);
            s.copy_ad(850, 3234);
        }

        s.store_scalar(3252, 0.0);

        s.store_scalar(3247, 0.0);

        s.store_scalar(859, 1e-40);

        s.store_scalar(861, 0.0);

        s.store_scalar(863, 0.0);

        s.store_mul(860, 1904, 1895);

        s.store_scalar(862, 0.0);

        s.store_scalar(3254, 0.0);

        s.b[3268] = ((s.v[1829] > 0.0) && (s.v[716] > 0.0));
        s.store_scalar(3268, if s.b[3268] { 1.0 } else { 0.0 });

        s.b[3270] = (p.p32 > 0.0);
        s.store_scalar(3270, if s.b[3270] { 1.0 } else { 0.0 });

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
        s.store_scalar(3271, if s.b[3271] { 1.0 } else { 0.0 });

        if ((s.b[3268] && s.b[3270]) && s.b[3271]) {
            s.store_div(3248, 1870, 1869);
            s.store_mul_ad_product_lhs_mixed_ai(3249, A::square(s.ad_value(3248)), 1860, 1860);
        }

        s.b[3272] = (s.v[0] == (-1.0));
        s.store_scalar(3272, if s.b[3272] { 1.0 } else { 0.0 });

        if (((s.b[3268] && s.b[3270]) && s.b[3271]) && s.b[3272]) {
            s.store_div_scaled_value_offset_denominator(3249, s.ad_value(3249), 1.0, A::mul(s.ad_value(3248), s.ad_value(1860)), 1.0, 1.0);
        }

        if ((s.b[3268] && s.b[3270]) && s.b[3271]) {
            s.store_mul_offset_rhs_scaled_ad_rhs(3250, 1869, A::sqrt(A::scale_offset(s.ad_value(3249), 2.0, 1.0)), 1.0, 0.5);
            s.store_div_ad_rhs(3251, 1869, A::mul(s.ad_value(3250), s.ad_value(3244)));
            s.store_mul_ad_product_lhs_mixed_ai(3252, A::mul3(s.ad_value(810), s.ad_value(838), s.ad_value(1857)), 3251, 3251);
            s.store_add_scaled_inputs(3247, 3247, 1.0, 3252, 1.0 / (s.v[718]));
        }

        if (s.b[3268] && s.b[3270]) {
            s.store_sqrt_mul(862, 719, 3247);
        }

        s.b[3273] = ((((p.p50 == 1.0) && (s.v[719] > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0));
        s.store_scalar(3273, if s.b[3273] { 1.0 } else { 0.0 });

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
            s.store_mul_ad_product_rhs_mixed_ia(3253, 3245, 3241, A::add_scaled_sub_value_product(1.0, A::scale(s.ad_value(3242), 12.0), 1.0, A::add_scaled_inputs_product(s.ad_value(3240), 1.0, s.ad_value(3242), 19.2, s.ad_value(3240), s.ad_value(3242), (-12.0)), s.ad_value(3243), (-1.0)));
            s.store_div_scaled_product3_mixed_aiia(860, A::square(s.ad_value(1908)), 1904, 1895, 1.0, A::square(s.ad_value(1906)), 1.0);
        }

        s.b[3274] = (s.v[277] > 0.0);
        s.store_scalar(3274, if s.b[3274] { 1.0 } else { 0.0 });

        if ((s.b[3268] && s.b[3273]) && s.b[3274]) {
            s.store_add_ad_rhs(859, 859, A::div_scaled_product_by_product(s.ad_value(3252), A::scale_offset(s.ad_value(3242), 12.0, 1.0), 1.0, s.ad_value(3246), s.ad_value(3246), (12.0 * s.v[718])));
            s.store_sub_ad_rhs(3253, 3253, A::div_scaled_product3(s.ad_value(3252), s.ad_value(3241), A::offset(s.ad_value(3243), 1.0), 1.0, s.ad_value(3246), s.v[718]));
        }

        if (s.b[3268] && s.b[3273]) {
            s.store_sqrt_div(3254, 719, 859);
        }

        s.b[3275] = (s.v[862] <= 0.0);
        s.store_scalar(3275, if s.b[3275] { 1.0 } else { 0.0 });

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
        s.store_scalar(3277, if s.b[3277] { 1.0 } else { 0.0 });

        if s.b[3277] {
            s.store_div_scaled_inputs_indices(2028, 1883, 4.0, 724, 1.0);
            s.store_scale(2028, 771, s.v[715]);
            s.store_mul(2028, 1864, 1877);
        }

        *var_qfgd_slot = var_qfgd;
        *var_qfgd_db0_slot = var_qfgd_db0;
        *var_qfgd_db1_slot = var_qfgd_db1;
        *var_qfgd_db10_slot = var_qfgd_db10;
        *var_qfgd_db11_slot = var_qfgd_db11;
        *var_qfgd_db12_slot = var_qfgd_db12;
        *var_qfgd_db13_slot = var_qfgd_db13;
        *var_qfgd_db14_slot = var_qfgd_db14;
        *var_qfgd_db15_slot = var_qfgd_db15;
        *var_qfgd_db16_slot = var_qfgd_db16;
        *var_qfgd_db17_slot = var_qfgd_db17;
        *var_qfgd_db18_slot = var_qfgd_db18;
        *var_qfgd_db19_slot = var_qfgd_db19;
        *var_qfgd_db2_slot = var_qfgd_db2;
        *var_qfgd_db20_slot = var_qfgd_db20;
        *var_qfgd_db21_slot = var_qfgd_db21;
        *var_qfgd_db22_slot = var_qfgd_db22;
        *var_qfgd_db23_slot = var_qfgd_db23;
        *var_qfgd_db24_slot = var_qfgd_db24;
        *var_qfgd_db3_slot = var_qfgd_db3;
        *var_qfgd_db4_slot = var_qfgd_db4;
        *var_qfgd_db5_slot = var_qfgd_db5;
        *var_qfgd_db6_slot = var_qfgd_db6;
        *var_qfgd_db7_slot = var_qfgd_db7;
        *var_qfgd_db8_slot = var_qfgd_db8;
        *var_qfgd_db9_slot = var_qfgd_db9;
        *var_qfgd_dn0_slot = var_qfgd_dn0;
        *var_qfgd_dn1_slot = var_qfgd_dn1;
        *var_qfgd_dn10_slot = var_qfgd_dn10;
        *var_qfgd_dn11_slot = var_qfgd_dn11;
        *var_qfgd_dn12_slot = var_qfgd_dn12;
        *var_qfgd_dn13_slot = var_qfgd_dn13;
        *var_qfgd_dn14_slot = var_qfgd_dn14;
        *var_qfgd_dn15_slot = var_qfgd_dn15;
        *var_qfgd_dn16_slot = var_qfgd_dn16;
        *var_qfgd_dn17_slot = var_qfgd_dn17;
        *var_qfgd_dn18_slot = var_qfgd_dn18;
        *var_qfgd_dn19_slot = var_qfgd_dn19;
        *var_qfgd_dn2_slot = var_qfgd_dn2;
        *var_qfgd_dn20_slot = var_qfgd_dn20;
        *var_qfgd_dn3_slot = var_qfgd_dn3;
        *var_qfgd_dn4_slot = var_qfgd_dn4;
        *var_qfgd_dn5_slot = var_qfgd_dn5;
        *var_qfgd_dn6_slot = var_qfgd_dn6;
        *var_qfgd_dn7_slot = var_qfgd_dn7;
        *var_qfgd_dn8_slot = var_qfgd_dn8;
        *var_qfgd_dn9_slot = var_qfgd_dn9;
        *var_qfgs_slot = var_qfgs;
        *var_qfgs_db0_slot = var_qfgs_db0;
        *var_qfgs_db1_slot = var_qfgs_db1;
        *var_qfgs_db10_slot = var_qfgs_db10;
        *var_qfgs_db11_slot = var_qfgs_db11;
        *var_qfgs_db12_slot = var_qfgs_db12;
        *var_qfgs_db13_slot = var_qfgs_db13;
        *var_qfgs_db14_slot = var_qfgs_db14;
        *var_qfgs_db15_slot = var_qfgs_db15;
        *var_qfgs_db16_slot = var_qfgs_db16;
        *var_qfgs_db17_slot = var_qfgs_db17;
        *var_qfgs_db18_slot = var_qfgs_db18;
        *var_qfgs_db19_slot = var_qfgs_db19;
        *var_qfgs_db2_slot = var_qfgs_db2;
        *var_qfgs_db20_slot = var_qfgs_db20;
        *var_qfgs_db21_slot = var_qfgs_db21;
        *var_qfgs_db22_slot = var_qfgs_db22;
        *var_qfgs_db23_slot = var_qfgs_db23;
        *var_qfgs_db24_slot = var_qfgs_db24;
        *var_qfgs_db3_slot = var_qfgs_db3;
        *var_qfgs_db4_slot = var_qfgs_db4;
        *var_qfgs_db5_slot = var_qfgs_db5;
        *var_qfgs_db6_slot = var_qfgs_db6;
        *var_qfgs_db7_slot = var_qfgs_db7;
        *var_qfgs_db8_slot = var_qfgs_db8;
        *var_qfgs_db9_slot = var_qfgs_db9;
        *var_qfgs_dn0_slot = var_qfgs_dn0;
        *var_qfgs_dn1_slot = var_qfgs_dn1;
        *var_qfgs_dn10_slot = var_qfgs_dn10;
        *var_qfgs_dn11_slot = var_qfgs_dn11;
        *var_qfgs_dn12_slot = var_qfgs_dn12;
        *var_qfgs_dn13_slot = var_qfgs_dn13;
        *var_qfgs_dn14_slot = var_qfgs_dn14;
        *var_qfgs_dn15_slot = var_qfgs_dn15;
        *var_qfgs_dn16_slot = var_qfgs_dn16;
        *var_qfgs_dn17_slot = var_qfgs_dn17;
        *var_qfgs_dn18_slot = var_qfgs_dn18;
        *var_qfgs_dn19_slot = var_qfgs_dn19;
        *var_qfgs_dn2_slot = var_qfgs_dn2;
        *var_qfgs_dn20_slot = var_qfgs_dn20;
        *var_qfgs_dn3_slot = var_qfgs_dn3;
        *var_qfgs_dn4_slot = var_qfgs_dn4;
        *var_qfgs_dn5_slot = var_qfgs_dn5;
        *var_qfgs_dn6_slot = var_qfgs_dn6;
        *var_qfgs_dn7_slot = var_qfgs_dn7;
        *var_qfgs_dn8_slot = var_qfgs_dn8;
        *var_qfgs_dn9_slot = var_qfgs_dn9;
        *var_qjun_d_slot = var_qjun_d;
        *var_qjun_d_db0_slot = var_qjun_d_db0;
        *var_qjun_d_db1_slot = var_qjun_d_db1;
        *var_qjun_d_db10_slot = var_qjun_d_db10;
        *var_qjun_d_db11_slot = var_qjun_d_db11;
        *var_qjun_d_db12_slot = var_qjun_d_db12;
        *var_qjun_d_db13_slot = var_qjun_d_db13;
        *var_qjun_d_db14_slot = var_qjun_d_db14;
        *var_qjun_d_db15_slot = var_qjun_d_db15;
        *var_qjun_d_db16_slot = var_qjun_d_db16;
        *var_qjun_d_db17_slot = var_qjun_d_db17;
        *var_qjun_d_db18_slot = var_qjun_d_db18;
        *var_qjun_d_db19_slot = var_qjun_d_db19;
        *var_qjun_d_db2_slot = var_qjun_d_db2;
        *var_qjun_d_db20_slot = var_qjun_d_db20;
        *var_qjun_d_db21_slot = var_qjun_d_db21;
        *var_qjun_d_db22_slot = var_qjun_d_db22;
        *var_qjun_d_db23_slot = var_qjun_d_db23;
        *var_qjun_d_db24_slot = var_qjun_d_db24;
        *var_qjun_d_db3_slot = var_qjun_d_db3;
        *var_qjun_d_db4_slot = var_qjun_d_db4;
        *var_qjun_d_db5_slot = var_qjun_d_db5;
        *var_qjun_d_db6_slot = var_qjun_d_db6;
        *var_qjun_d_db7_slot = var_qjun_d_db7;
        *var_qjun_d_db8_slot = var_qjun_d_db8;
        *var_qjun_d_db9_slot = var_qjun_d_db9;
        *var_qjun_d_dn0_slot = var_qjun_d_dn0;
        *var_qjun_d_dn1_slot = var_qjun_d_dn1;
        *var_qjun_d_dn10_slot = var_qjun_d_dn10;
        *var_qjun_d_dn11_slot = var_qjun_d_dn11;
        *var_qjun_d_dn12_slot = var_qjun_d_dn12;
        *var_qjun_d_dn13_slot = var_qjun_d_dn13;
        *var_qjun_d_dn14_slot = var_qjun_d_dn14;
        *var_qjun_d_dn15_slot = var_qjun_d_dn15;
        *var_qjun_d_dn16_slot = var_qjun_d_dn16;
        *var_qjun_d_dn17_slot = var_qjun_d_dn17;
        *var_qjun_d_dn18_slot = var_qjun_d_dn18;
        *var_qjun_d_dn19_slot = var_qjun_d_dn19;
        *var_qjun_d_dn2_slot = var_qjun_d_dn2;
        *var_qjun_d_dn20_slot = var_qjun_d_dn20;
        *var_qjun_d_dn3_slot = var_qjun_d_dn3;
        *var_qjun_d_dn4_slot = var_qjun_d_dn4;
        *var_qjun_d_dn5_slot = var_qjun_d_dn5;
        *var_qjun_d_dn6_slot = var_qjun_d_dn6;
        *var_qjun_d_dn7_slot = var_qjun_d_dn7;
        *var_qjun_d_dn8_slot = var_qjun_d_dn8;
        *var_qjun_d_dn9_slot = var_qjun_d_dn9;
        *var_qjun_s_slot = var_qjun_s;
        *var_qjun_s_db0_slot = var_qjun_s_db0;
        *var_qjun_s_db1_slot = var_qjun_s_db1;
        *var_qjun_s_db10_slot = var_qjun_s_db10;
        *var_qjun_s_db11_slot = var_qjun_s_db11;
        *var_qjun_s_db12_slot = var_qjun_s_db12;
        *var_qjun_s_db13_slot = var_qjun_s_db13;
        *var_qjun_s_db14_slot = var_qjun_s_db14;
        *var_qjun_s_db15_slot = var_qjun_s_db15;
        *var_qjun_s_db16_slot = var_qjun_s_db16;
        *var_qjun_s_db17_slot = var_qjun_s_db17;
        *var_qjun_s_db18_slot = var_qjun_s_db18;
        *var_qjun_s_db19_slot = var_qjun_s_db19;
        *var_qjun_s_db2_slot = var_qjun_s_db2;
        *var_qjun_s_db20_slot = var_qjun_s_db20;
        *var_qjun_s_db21_slot = var_qjun_s_db21;
        *var_qjun_s_db22_slot = var_qjun_s_db22;
        *var_qjun_s_db23_slot = var_qjun_s_db23;
        *var_qjun_s_db24_slot = var_qjun_s_db24;
        *var_qjun_s_db3_slot = var_qjun_s_db3;
        *var_qjun_s_db4_slot = var_qjun_s_db4;
        *var_qjun_s_db5_slot = var_qjun_s_db5;
        *var_qjun_s_db6_slot = var_qjun_s_db6;
        *var_qjun_s_db7_slot = var_qjun_s_db7;
        *var_qjun_s_db8_slot = var_qjun_s_db8;
        *var_qjun_s_db9_slot = var_qjun_s_db9;
        *var_qjun_s_dn0_slot = var_qjun_s_dn0;
        *var_qjun_s_dn1_slot = var_qjun_s_dn1;
        *var_qjun_s_dn10_slot = var_qjun_s_dn10;
        *var_qjun_s_dn11_slot = var_qjun_s_dn11;
        *var_qjun_s_dn12_slot = var_qjun_s_dn12;
        *var_qjun_s_dn13_slot = var_qjun_s_dn13;
        *var_qjun_s_dn14_slot = var_qjun_s_dn14;
        *var_qjun_s_dn15_slot = var_qjun_s_dn15;
        *var_qjun_s_dn16_slot = var_qjun_s_dn16;
        *var_qjun_s_dn17_slot = var_qjun_s_dn17;
        *var_qjun_s_dn18_slot = var_qjun_s_dn18;
        *var_qjun_s_dn19_slot = var_qjun_s_dn19;
        *var_qjun_s_dn2_slot = var_qjun_s_dn2;
        *var_qjun_s_dn20_slot = var_qjun_s_dn20;
        *var_qjun_s_dn3_slot = var_qjun_s_dn3;
        *var_qjun_s_dn4_slot = var_qjun_s_dn4;
        *var_qjun_s_dn5_slot = var_qjun_s_dn5;
        *var_qjun_s_dn6_slot = var_qjun_s_dn6;
        *var_qjun_s_dn7_slot = var_qjun_s_dn7;
        *var_qjun_s_dn8_slot = var_qjun_s_dn8;
        *var_qjun_s_dn9_slot = var_qjun_s_dn9;
    }

    pub(super) fn stamp_reactive_block_0(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[990] = (p.p37 >= 0.0);
        s.store_scalar(990, if s.b[990] { 1.0 } else { 0.0 });

        if s.b[990] {
            s.store_scalar(0, 1.0);
        }

        if (!s.b[990]) {
            s.store_scalar(0, (-1.0));
        }

        s.store_scalar(767, (8.8541878176e-12 * 11.8));

        s.b[991] = (p.p51 < 0.5);
        s.store_scalar(991, if s.b[991] { 1.0 } else { 0.0 });

        if s.b[991] {
            s.store_scalar(1, 0.0);
        }

        s.b[992] = (p.p51 < 1.5);
        s.store_scalar(992, if s.b[992] { 1.0 } else { 0.0 });

        if ((!s.b[991]) && s.b[992]) {
            s.store_scalar(1, 1.0);
        }

        s.b[993] = (p.p51 < 2.5);
        s.store_scalar(993, if s.b[993] { 1.0 } else { 0.0 });

        if (((!s.b[991]) && (!s.b[992])) && s.b[993]) {
            s.store_scalar(1, 2.0);
        }

        s.b[994] = (p.p51 < 4.0);
        s.store_scalar(994, if s.b[994] { 1.0 } else { 0.0 });

        if ((((!s.b[991]) && (!s.b[992])) && (!s.b[993])) && s.b[994]) {
            s.store_scalar(1, 3.0);
        }

        s.b[995] = (p.p51 < 7.0);
        s.store_scalar(995, if s.b[995] { 1.0 } else { 0.0 });

        if (((((!s.b[991]) && (!s.b[992])) && (!s.b[993])) && (!s.b[994])) && s.b[995]) {
            s.store_scalar(1, 5.0);
        }

        if (((((!s.b[991]) && (!s.b[992])) && (!s.b[993])) && (!s.b[994])) && (!s.b[995])) {
            s.store_scalar(1, 9.0);
        }

        s.store_scalar(3, 10.0);

        s.store_scalar(4, (1.0 / s.v[3]));

        s.store_scalar(350, (273.15 + p.p38));

        s.store_scalar(474, 0.0);

        s.b[996] = (p.p927 > 0.5);
        s.store_scalar(996, if s.b[996] { 1.0 } else { 0.0 });

        if s.b[996] {
            s.store_scalar(474, 1.0);
        }

        if (!s.b[996]) {
            s.store_scalar(474, 0.0);
        }

        s.store_scalar(364, (273.15 + p.p823));

        s.store_scalar(367, (1.3806505e-23 / 1.6021918e-19));

        s.store_scalar(368, (s.v[367] * s.v[364]));

        s.store_scalar(369, (1.0 / s.v[368]));

        s.store_scalar(375, ((-((0.000702 * s.v[364]) * s.v[364])) / (1108.0 + s.v[364])));

        s.store_scalar(378, (p.p834 + s.v[375]));

        s.store_scalar(379, (p.p835 + s.v[375]));

        s.store_scalar(380, (p.p836 + s.v[375]));

        s.store_scalar(408, (1.0 - p.p831));

        s.store_scalar(409, (1.0 - p.p832));

        s.store_scalar(410, (1.0 - p.p833));

        s.store_scalar(411, (1.0 / s.v[408]));

        s.store_scalar(412, (1.0 / s.v[409]));

        s.store_scalar(413, (1.0 / s.v[410]));

        s.store_scalar(423, (s.v[767] / p.p825));

        s.store_scalar(424, ((p.p843 * s.v[767]) / p.p826));

        s.store_scalar(425, ((p.p844 * s.v[767]) / p.p827));

        s.store_scalar(426, (1.0 / s.v[423]));

        s.store_scalar(427, (1.0 / s.v[424]));

        s.store_scalar(428, (1.0 / s.v[425]));

        s.store_scalar(429, (1.0 / p.p828));

        s.store_scalar(430, (1.0 / p.p829));

        s.store_scalar(431, (1.0 / p.p830));

        s.store_scalar(444, (1.0 - (1.0 / p.p824)));

        s.store_scalar(448, (1.0 / p.p860));

        s.store_scalar(449, (1.0 / p.p861));

        s.store_scalar(450, (1.0 / p.p862));

        s.b[997] = ((((p.p866 != 1.0) || (p.p867 != 1.0)) || (p.p868 != 1.0)) || (p.p869 != 1.0));
        s.store_scalar(997, if s.b[997] { 1.0 } else { 0.0 });

        if s.b[997] {
            s.store_scalar(473, 1.0);
        }

        if (!s.b[997]) {
            s.store_scalar(473, 0.0);
        }

        s.b[998] = (s.v[473] == 1.0);
        s.store_scalar(998, if s.b[998] { 1.0 } else { 0.0 });

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
        s.store_scalar(999, if s.b[999] { 1.0 } else { 0.0 });

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
        s.store_scalar(1000, if s.b[1000] { 1.0 } else { 0.0 });

        if s.b[1000] {
            s.store_scalar(635, 1.0);
        }

        if (!s.b[1000]) {
            s.store_scalar(635, 0.0);
        }

        s.b[1001] = (s.v[635] == 1.0);
        s.store_scalar(1001, if s.b[1001] { 1.0 } else { 0.0 });

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

        s.store_scalar(351, ((ctx_temp + p.p56) + p.p35));

        s.store_scalar(352, (s.v[351] / s.v[350]));

        s.store_scalar(353, (s.v[351] - s.v[350]));

        s.store_scalar(354, ((s.v[351] * 1.3806505e-23) / 1.6021918e-19));

        s.store_scalar(355, (1.0 / s.v[354]));

        s.store_scalar(356, s.v[351]);

        s.store_scalar(357, (s.v[356] * s.v[356]));

        s.store_scalar(358, (s.v[356] - s.v[350]));

        s.store_scalar(359, (s.v[350] / s.v[356]));

        s.store_scalar(360, ((s.v[359]) as f64).ln());

        s.store_scalar(715, ((s.v[356] * 1.3806505e-23) / 1.6021918e-19));

        s.store_scalar(361, (1.0 / s.v[715]));

        s.store_scalar(362, ((1.179 - (9.025e-5 * s.v[356])) - (3.05e-7 * s.v[357])));

        s.store_scalar(363, ((((1.045 + (0.00045 * s.v[356])) * ((0.523 + (0.0014 * s.v[356])) - (1.48e-6 * s.v[357]))) * s.v[357]) / 90000.0));

        if (!(s.v[363] > 0.001)) {
            s.store_scalar(363, 0.001);
        }

        s.store_scalar(365, (((ctx_temp + p.p56) + p.p35)).max((273.15 + (-250.0))));

        s.store_scalar(366, (s.v[365] / s.v[364]));

        s.store_scalar(370, (s.v[367] * s.v[365]));

        s.store_scalar(371, (1.0 / s.v[370]));

        s.store_scalar(376, ((-((0.000702 * s.v[365]) * s.v[365])) / (1108.0 + s.v[365])));

        s.store_scalar(381, (p.p834 + s.v[376]));

        s.store_scalar(382, (p.p835 + s.v[376]));

        s.store_scalar(383, (p.p836 + s.v[376]));

        s.store_scalar(384, (((s.v[366]) as f64).powf(1.5) * (((0.5 * ((s.v[378] * s.v[369]) - (s.v[381] * s.v[371])))) as f64).exp()));

        s.store_scalar(385, (((s.v[366]) as f64).powf(1.5) * (((0.5 * ((s.v[379] * s.v[369]) - (s.v[382] * s.v[371])))) as f64).exp()));

        s.store_scalar(386, (((s.v[366]) as f64).powf(1.5) * (((0.5 * ((s.v[380] * s.v[369]) - (s.v[383] * s.v[371])))) as f64).exp()));

        s.store_scalar(387, ((p.p837 * s.v[384]) * s.v[384]));

        s.store_scalar(388, ((p.p838 * s.v[385]) * s.v[385]));

        s.store_scalar(389, ((p.p839 * s.v[386]) * s.v[386]));

        s.store_scalar(390, ((p.p828 * s.v[366]) - ((2.0 * s.v[370]) * ((s.v[384]) as f64).ln())));

        s.store_scalar(391, ((p.p829 * s.v[366]) - ((2.0 * s.v[370]) * ((s.v[385]) as f64).ln())));

        s.store_scalar(392, ((p.p830 * s.v[366]) - ((2.0 * s.v[370]) * ((s.v[386]) as f64).ln())));

        s.store_scalar(393, (s.v[390] + (s.v[370] * (((1.0 + ((((0.05 - s.v[390]) * s.v[371])) as f64).exp())) as f64).ln())));

        s.store_scalar(394, (s.v[391] + (s.v[370] * (((1.0 + ((((0.05 - s.v[391]) * s.v[371])) as f64).exp())) as f64).ln())));

        s.store_scalar(395, (s.v[392] + (s.v[370] * (((1.0 + ((((0.05 - s.v[392]) * s.v[371])) as f64).exp())) as f64).ln())));

        s.store_scalar(405, (1.0 / s.v[393]));

        s.store_scalar(406, (1.0 / s.v[394]));

        s.store_scalar(407, (1.0 / s.v[395]));

        s.store_scalar(414, (p.p825 * (((p.p828 * s.v[405])) as f64).powf(p.p831)));

        s.store_scalar(415, (p.p826 * (((p.p829 * s.v[406])) as f64).powf(p.p832)));

        s.store_scalar(416, (p.p827 * (((p.p830 * s.v[407])) as f64).powf(p.p833)));

        s.store_scalar(417, ((s.v[414] * s.v[393]) * s.v[411]));

        s.store_scalar(418, ((s.v[415] * s.v[394]) * s.v[412]));

        s.store_scalar(419, ((s.v[416] * s.v[395]) * s.v[413]));

        s.store_scalar(420, (2.0 * s.v[414]));

        s.store_scalar(421, (2.0 * s.v[415]));

        s.store_scalar(422, (2.0 * s.v[416]));

        s.store_scalar(432, ((0.5 * s.v[381])).max(s.v[370]));

        s.store_scalar(433, ((0.5 * s.v[382])).max(s.v[370]));

        s.store_scalar(434, ((0.5 * s.v[383])).max(s.v[370]));

        s.store_scalar(435, (s.v[432] * s.v[371]));

        s.store_scalar(436, (s.v[433] * s.v[371]));

        s.store_scalar(437, (s.v[434] * s.v[371]));

        s.store_scalar(438, (((((((32.0 * p.p848) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[432] * s.v[432]) * s.v[432]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));

        s.store_scalar(439, (((((((32.0 * p.p849) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[433] * s.v[433]) * s.v[433]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));

        s.store_scalar(440, (((((((32.0 * p.p850) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[434] * s.v[434]) * s.v[434]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));

        s.store_scalar(441, (p.p854 * (1.0 + (p.p857 * (s.v[365] - s.v[364])))));

        s.store_scalar(442, (p.p855 * (1.0 + (p.p858 * (s.v[365] - s.v[364])))));

        s.store_scalar(443, (p.p856 * (1.0 + (p.p859 * (s.v[365] - s.v[364])))));

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
        s.store_scalar(1021, if s.b[1021] { 1.0 } else { 0.0 });

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
        s.store_scalar(1022, if s.b[1022] { 1.0 } else { 0.0 });

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

        s.store_scalar(5, 1.0);

        s.store_scalar(6, 1.0);

        s.store_scalar(312, 0.0);

        s.store_scalar(313, 0.0);

        s.store_scalar(7, p.p0);

        s.store_scalar(8, p.p1);

        s.store_scalar(9, p.p2);

        s.store_scalar(10, p.p3);

        s.store_scalar(11, p.p4);

        s.store_scalar(12, p.p8);

        s.store_scalar(646, p.p19);

        s.store_scalar(647, p.p20);

        s.store_scalar(648, p.p21);

        s.store_scalar(673, p.p22);

        s.store_scalar(674, p.p23);

        s.store_scalar(675, p.p24);

        s.store_scalar(649, p.p25);

        s.store_scalar(650, p.p26);

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.store_scalar(676, p.p27);

        s.store_scalar(677, p.p28);

        s.store_scalar(14, p.p14);

        s.b[1023] = (p.p39 > 0.0);
        s.store_scalar(1023, if s.b[1023] { 1.0 } else { 0.0 });

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

        s.store_scalar(15, p.p5);

        s.store_scalar(16, p.p6);

        s.store_scalar(17, p.p7);

        s.store_scalar(308, (1e-6 / s.v[7]));

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

        s.store_scalar(44, p.p57);

        s.store_scalar(45, p.p58);

        s.store_scalar(46, p.p59);

        s.store_scalar(47, p.p60);

        s.store_scalar(48, p.p61);

        s.store_scalar(49, p.p62);

        s.store_scalar(50, p.p63);

        s.store_scalar(51, p.p64);

        s.store_scalar(52, p.p65);

        s.store_scalar(53, p.p66);

        s.store_scalar(54, p.p67);

        s.store_scalar(59, p.p68);

        s.store_scalar(60, p.p69);

        s.store_scalar(61, p.p70);

        s.store_scalar(62, p.p71);

        s.store_scalar(55, p.p72);

        s.store_scalar(56, p.p74);

        s.store_scalar(57, p.p73);

        s.store_scalar(58, p.p75);

        s.store_scalar(63, p.p79);

        s.store_scalar(64, p.p81);

        s.store_scalar(65, p.p80);

        s.store_scalar(66, p.p76);

        s.store_scalar(67, p.p78);

        s.store_scalar(68, p.p77);

        s.store_scalar(69, p.p82);

        s.store_scalar(70, p.p83);

        s.store_scalar(71, p.p84);

        s.store_scalar(72, p.p85);

        s.store_scalar(73, p.p86);

        s.store_scalar(74, p.p87);

        s.store_scalar(75, p.p88);

        s.store_scalar(76, p.p89);

        s.store_scalar(77, p.p90);

        s.store_scalar(78, p.p91);

        s.store_scalar(79, p.p92);

        s.store_scalar(80, p.p93);

        s.store_scalar(81, p.p94);

        s.store_scalar(82, p.p95);

        s.store_scalar(83, p.p96);

        s.store_scalar(84, p.p97);

        s.store_scalar(85, p.p98);

        s.store_scalar(86, p.p99);

        s.store_scalar(87, p.p100);

        s.store_scalar(88, p.p101);

        s.store_scalar(89, p.p102);

        s.store_scalar(90, p.p103);

        s.store_scalar(91, p.p104);

        s.store_scalar(92, p.p105);

        s.store_scalar(93, p.p106);

        s.store_scalar(94, p.p107);

        s.store_scalar(95, p.p108);

        s.store_scalar(96, p.p109);

        s.store_scalar(97, p.p110);

        s.store_scalar(98, p.p111);

        s.store_scalar(99, p.p112);

        s.store_scalar(100, p.p113);

        s.store_scalar(101, p.p114);

        s.store_scalar(102, p.p115);

        s.store_scalar(103, p.p116);

        s.store_scalar(104, p.p117);

        s.store_scalar(105, p.p118);

        s.store_scalar(106, p.p119);

        s.store_scalar(107, p.p120);

        s.store_scalar(108, p.p121);

        s.store_scalar(109, p.p120);

        s.b[1024] = param_given[122];
        s.store_scalar(1024, if s.b[1024] { 1.0 } else { 0.0 });

        if s.b[1024] {
            s.store_scalar(109, p.p122);
        }

        s.store_scalar(110, p.p121);

        s.b[1025] = param_given[123];
        s.store_scalar(1025, if s.b[1025] { 1.0 } else { 0.0 });

        if s.b[1025] {
            s.store_scalar(110, p.p123);
        }

        s.copy_ad(111, 109);

        s.b[1026] = param_given[124];
        s.store_scalar(1026, if s.b[1026] { 1.0 } else { 0.0 });

        if s.b[1026] {
            s.store_scalar(111, p.p124);
        }

        s.copy_ad(112, 110);

        s.b[1027] = param_given[125];
        s.store_scalar(1027, if s.b[1027] { 1.0 } else { 0.0 });

        if s.b[1027] {
            s.store_scalar(112, p.p125);
        }

        s.store_scalar(113, p.p126);

        s.store_scalar(114, p.p127);

        s.store_scalar(115, p.p128);

        s.store_scalar(116, p.p129);

        s.store_scalar(117, p.p130);

        s.store_scalar(118, p.p131);

        s.store_scalar(119, p.p132);

        s.store_scalar(120, p.p133);

        s.store_scalar(121, p.p134);

        s.store_scalar(122, p.p135);

        s.store_scalar(123, p.p136);

        s.store_scalar(124, p.p137);

        s.store_scalar(125, p.p99);

        s.b[1028] = param_given[138];
        s.store_scalar(1028, if s.b[1028] { 1.0 } else { 0.0 });

        if s.b[1028] {
            s.store_scalar(125, p.p138);
        }

        s.store_scalar(126, p.p104);

        s.b[1029] = param_given[139];
        s.store_scalar(1029, if s.b[1029] { 1.0 } else { 0.0 });

        if s.b[1029] {
            s.store_scalar(126, p.p139);
        }

        s.store_scalar(127, p.p140);

        s.store_scalar(128, p.p141);

        s.store_scalar(129, p.p142);

        s.store_scalar(130, p.p143);

        s.store_scalar(131, p.p144);

        s.store_scalar(132, p.p145);

        s.store_scalar(133, p.p146);

        s.store_scalar(134, p.p147);

        s.store_scalar(135, p.p148);

        s.store_scalar(136, p.p149);

        s.store_scalar(137, p.p150);

        s.store_scalar(138, p.p151);

        s.store_scalar(139, p.p152);

        s.store_scalar(140, p.p153);

        s.store_scalar(141, p.p154);

        s.store_scalar(142, p.p155);

        s.store_scalar(143, p.p156);

        s.store_scalar(149, p.p162);

        s.store_scalar(150, p.p163);

        s.store_scalar(151, p.p164);

        s.store_scalar(152, p.p165);

        s.store_scalar(153, p.p166);

        s.store_scalar(154, p.p167);

        s.store_scalar(155, p.p168);

        s.store_scalar(156, p.p169);

        s.store_scalar(157, p.p170);

        s.store_scalar(158, p.p171);

        s.store_scalar(159, p.p172);

        s.store_scalar(160, p.p174);

        s.store_scalar(161, p.p173);

        s.store_scalar(176, p.p187);

        s.b[1030] = (p.p39 > 0.0);
        s.store_scalar(1030, if s.b[1030] { 1.0 } else { 0.0 });

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
        s.store_scalar(1031, if s.b[1031] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1031]) {
            s.store_scalar(334, 75000000000.0);
            s.store_sub_ad(335, A::sqrt(A::add_scaled_inputs(s.ad_value(331), 1.0, s.ad_value(332), 0.5)), A::sqrt(s.ad_value(331)));
            s.store_add_scaled_product_mixed_aia(336, A::sqrt(s.ad_value(331)), 1.0, 334, A::ln(A::offset(A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(333), 2.0, s.ad_value(312), 1.0), A::exp(A::div(s.ad_value(335), s.ad_value(334))), (-1.0)), 1.0)), 1.0);
            s.store_square(336, 336);
        }

        s.b[1032] = (s.v[312] >= s.v[333]);
        s.store_scalar(1032, if s.b[1032] { 1.0 } else { 0.0 });

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
            s.store_add_ad(339, A::offset(A::mul_sub_from_scalar_rhs(A::div_scaled_product(s.ad_value(337), s.ad_value(338), 1.0, s.ad_value(312), 1.0), 1.0, A::exp_div_scaled_inputs(s.ad_value(312), -1.0, s.ad_value(338), 1.0)), 1.0), A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p263 * p.p264), s.ad_value(312)), 1.0, A::exp_scaled_input(s.ad_value(312), (-1.0 / (p.p264)))));
        }

        if s.b[1030] {
            if (s.v[339] > 1e-15) {
            } else {
                s.store_scalar(339, 1e-15);
            }
        }

        if s.b[1030] {
            s.store_add_scaled_product_mixed_aia(340, A::scale_offset(s.ad_value(316), p.p265, 1.0), 1.0, 316, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p267), 1.0)), p.p266);
            s.store_mul_div_scaled_inputs_mixed_iia(69, 340, 313, p.p258, A::mul(s.ad_value(339), s.ad_value(312)), 1.0);
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
        s.store_scalar(1033, if s.b[1033] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1033]) {
            s.store_scalar(109, p.p341);
        }

        if s.b[1030] {
            s.store_scalar(110, p.p340);
        }

        s.b[1034] = param_given[342];
        s.store_scalar(1034, if s.b[1034] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1034]) {
            s.store_scalar(110, p.p342);
        }

        if s.b[1030] {
            s.copy_ad(111, 109);
        }

        s.b[1035] = param_given[343];
        s.store_scalar(1035, if s.b[1035] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1035]) {
            s.store_scalar(111, p.p343);
        }

        if s.b[1030] {
            s.copy_ad(112, 110);
        }

        s.b[1036] = param_given[344];
        s.store_scalar(1036, if s.b[1036] { 1.0 } else { 0.0 });

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
        s.store_scalar(1037, if s.b[1037] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1037]) {
            s.store_scalar(36, p.p363);
        }

        if s.b[1030] {
            s.store_scalar(37, p.p297);
        }

        s.b[1038] = param_given[364];
        s.store_scalar(1038, if s.b[1038] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1038]) {
            s.store_scalar(37, p.p364);
        }

        if s.b[1030] {
            s.store_scalar(38, p.p298);
        }

        s.b[1039] = param_given[365];
        s.store_scalar(1039, if s.b[1039] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1039]) {
            s.store_scalar(38, p.p365);
        }

        if s.b[1030] {
            s.store_scalar(39, p.p299);
        }

        s.b[1040] = param_given[366];
        s.store_scalar(1040, if s.b[1040] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1040]) {
            s.store_scalar(39, p.p366);
        }

        if s.b[1030] {
            s.store_scalar(40, p.p300);
        }

        s.b[1041] = param_given[367];
        s.store_scalar(1041, if s.b[1041] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1041]) {
            s.store_scalar(40, p.p367);
        }

        if s.b[1030] {
            s.store_mul3_ad(125, A::add_scaled_product(s.ad_value(36), 1.0, A::div_scaled_product(s.ad_value(37), s.ad_value(340), 1.0, s.ad_value(339), 1.0), A::pow(s.ad_value(314), s.ad_value(38)), 1.0), A::offset(A::mul(s.ad_value(39), s.ad_value(316)), 1.0), A::offset(A::mul(s.ad_value(40), s.ad_value(318)), 1.0));
            s.store_scalar(41, p.p308);
        }

        s.b[1042] = param_given[368];
        s.store_scalar(1042, if s.b[1042] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1042]) {
            s.store_scalar(41, p.p368);
        }

        if s.b[1030] {
            s.store_scalar(42, p.p309);
        }

        s.b[1043] = param_given[369];
        s.store_scalar(1043, if s.b[1043] { 1.0 } else { 0.0 });

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

    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if s.b[1030] {
            if (s.v[347] > 1e-15) {
            } else {
                s.store_scalar(347, 1e-15);
            }
        }

        if s.b[1030] {
            s.store_mul_div_scaled_inputs_mixed_aia(154, A::scale_offset(s.ad_value(316), p.p419, 1.0), 344, p.p258, A::mul(s.ad_value(347), s.ad_value(312)), 1.0);
            s.store_add_scaled_inputs3_offset_indices(155, 314, p.p421, 316, p.p422, 318, p.p423, p.p420);
            s.store_scaled_mul_scale_offset_rhs_ad(156, A::powf(s.ad_value(314), p.p425), 316, p.p426, 1.0, p.p424);
            s.store_scalar(157, p.p427);
            s.store_scalar(158, p.p428);
            s.store_scaled_mul_scale_offset_rhs_ad(159, A::powf(s.ad_value(314), p.p430), 316, p.p431, 1.0, p.p429);
            s.store_scalar(160, p.p433);
            s.store_scalar(161, p.p432);
            s.store_add_scaled_inputs3_offset_indices(348, 314, p.p815, 316, p.p816, 318, p.p817, p.p814);
            s.store_add_scaled_inputs3_offset_indices(349, 314, p.p819, 316, p.p820, 318, p.p821, p.p818);
            s.store_scalar(176, p.p450);
        }

        s.b[1045] = (((param_given[451] || param_given[452]) || param_given[453]) || param_given[454]);
        s.store_scalar(1045, if s.b[1045] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1045]) {
            s.store_add_scaled_inputs3_offset_indices(44, 314, p.p452, 316, p.p453, 318, p.p454, p.p451);
        }

        s.b[1046] = (((param_given[455] || param_given[456]) || param_given[457]) || param_given[458]);
        s.store_scalar(1046, if s.b[1046] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1046]) {
            s.store_add_scaled_inputs3_offset_indices(45, 314, p.p456, 316, p.p457, 318, p.p458, p.p455);
        }

        s.b[1047] = (((param_given[459] || param_given[460]) || param_given[461]) || param_given[462]);
        s.store_scalar(1047, if s.b[1047] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1047]) {
            s.store_add_scaled_inputs3_offset_indices(49, 314, p.p460, 316, p.p461, 318, p.p462, p.p459);
        }

        s.b[1048] = (((param_given[463] || param_given[464]) || param_given[465]) || param_given[466]);
        s.store_scalar(1048, if s.b[1048] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1048]) {
            s.store_add_scaled_inputs3_offset_indices(50, 314, p.p464, 316, p.p465, 318, p.p466, p.p463);
        }

        s.b[1049] = (((param_given[467] || param_given[468]) || param_given[469]) || param_given[470]);
        s.store_scalar(1049, if s.b[1049] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1049]) {
            s.store_add_scaled_inputs3_offset_indices(51, 314, p.p468, 316, p.p469, 318, p.p470, p.p467);
        }

        s.b[1050] = (((param_given[471] || param_given[472]) || param_given[473]) || param_given[474]);
        s.store_scalar(1050, if s.b[1050] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1050]) {
            s.store_add_scaled_inputs3_offset_indices(53, 314, p.p472, 316, p.p473, 318, p.p474, p.p471);
        }

        s.b[1051] = (((param_given[475] || param_given[476]) || param_given[477]) || param_given[478]);
        s.store_scalar(1051, if s.b[1051] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1051]) {
            s.store_add_scaled_inputs3_offset_indices(54, 314, p.p476, 316, p.p477, 318, p.p478, p.p475);
        }

        s.b[1052] = (((param_given[479] || param_given[480]) || param_given[481]) || param_given[482]);
        s.store_scalar(1052, if s.b[1052] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1052]) {
            s.store_add_scaled_inputs3_offset_indices(61, 314, p.p480, 316, p.p481, 318, p.p482, p.p479);
        }

        s.b[1053] = (((param_given[483] || param_given[484]) || param_given[485]) || param_given[486]);
        s.store_scalar(1053, if s.b[1053] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1053]) {
            s.store_add_scaled_inputs3_offset_indices(62, 314, p.p484, 316, p.p485, 318, p.p486, p.p483);
        }

        s.b[1054] = (((param_given[487] || param_given[488]) || param_given[489]) || param_given[490]);
        s.store_scalar(1054, if s.b[1054] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1054]) {
            s.store_add_scaled_inputs3_offset_indices(55, 314, p.p488, 316, p.p489, 318, p.p490, p.p487);
        }

        s.b[1055] = (((param_given[495] || param_given[496]) || param_given[497]) || param_given[498]);
        s.store_scalar(1055, if s.b[1055] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1055]) {
            s.store_add_scaled_inputs3_offset_indices(56, 314, p.p496, 316, p.p497, 318, p.p498, p.p495);
        }

        s.b[1056] = (((param_given[491] || param_given[492]) || param_given[493]) || param_given[494]);
        s.store_scalar(1056, if s.b[1056] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1056]) {
            s.store_add_scaled_inputs3_offset_indices(57, 314, p.p492, 316, p.p493, 318, p.p494, p.p491);
        }

        s.b[1057] = (((param_given[499] || param_given[500]) || param_given[501]) || param_given[502]);
        s.store_scalar(1057, if s.b[1057] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1057]) {
            s.store_add_scaled_inputs3_offset_indices(58, 314, p.p500, 316, p.p501, 318, p.p502, p.p499);
        }

        s.b[1058] = (((param_given[503] || param_given[504]) || param_given[505]) || param_given[506]);
        s.store_scalar(1058, if s.b[1058] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1058]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(66, 315, s.ad_value(314), p.p504, s.ad_value(316), p.p505, s.ad_value(318), p.p506, p.p503);
        }

        s.b[1059] = (((param_given[511] || param_given[512]) || param_given[513]) || param_given[514]);
        s.store_scalar(1059, if s.b[1059] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1059]) {
            s.store_add_scaled_inputs3_offset_indices(67, 314, p.p512, 316, p.p513, 318, p.p514, p.p511);
        }

        s.b[1060] = (((param_given[507] || param_given[508]) || param_given[509]) || param_given[510]);
        s.store_scalar(1060, if s.b[1060] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1060]) {
            s.store_add_scaled_inputs3_offset_indices(68, 314, p.p508, 316, p.p509, 318, p.p510, p.p507);
        }

        s.b[1061] = (((param_given[515] || param_given[516]) || param_given[517]) || param_given[518]);
        s.store_scalar(1061, if s.b[1061] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1061]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(63, 315, s.ad_value(314), p.p516, s.ad_value(316), p.p517, s.ad_value(318), p.p518, p.p515);
        }

        s.b[1062] = (((param_given[523] || param_given[524]) || param_given[525]) || param_given[526]);
        s.store_scalar(1062, if s.b[1062] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1062]) {
            s.store_add_scaled_inputs3_offset_indices(64, 314, p.p524, 316, p.p525, 318, p.p526, p.p523);
        }

        s.b[1063] = (((param_given[519] || param_given[520]) || param_given[521]) || param_given[522]);
        s.store_scalar(1063, if s.b[1063] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1063]) {
            s.store_add_scaled_inputs3_offset_indices(65, 314, p.p520, 316, p.p521, 318, p.p522, p.p519);
        }

        s.b[1064] = (((param_given[527] || param_given[528]) || param_given[529]) || param_given[530]);
        s.store_scalar(1064, if s.b[1064] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1064]) {
            s.store_mul_div_scaled_inputs_mixed_aii(69, A::add_scaled_inputs3_offset(s.ad_value(314), p.p528, s.ad_value(316), p.p529, s.ad_value(318), p.p530, p.p527), 313, 1.0, 312, 1.0);
        }

        s.b[1065] = (((param_given[531] || param_given[532]) || param_given[533]) || param_given[534]);
        s.store_scalar(1065, if s.b[1065] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1065]) {
            s.store_add_scaled_inputs3_offset_indices(70, 314, p.p532, 316, p.p533, 318, p.p534, p.p531);
        }

        s.b[1066] = (((param_given[535] || param_given[536]) || param_given[537]) || param_given[538]);
        s.store_scalar(1066, if s.b[1066] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1066]) {
            s.store_add_scaled_inputs3_offset_indices(71, 314, p.p536, 316, p.p537, 318, p.p538, p.p535);
        }

        s.b[1067] = (((param_given[539] || param_given[540]) || param_given[541]) || param_given[542]);
        s.store_scalar(1067, if s.b[1067] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1067]) {
            s.store_add_scaled_inputs3_offset_indices(73, 314, p.p540, 316, p.p541, 318, p.p542, p.p539);
        }

        s.b[1068] = (((param_given[543] || param_given[544]) || param_given[545]) || param_given[546]);
        s.store_scalar(1068, if s.b[1068] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1068]) {
            s.store_add_scaled_inputs3_offset_indices(75, 314, p.p544, 316, p.p545, 318, p.p546, p.p543);
        }

        s.b[1069] = (((param_given[547] || param_given[548]) || param_given[549]) || param_given[550]);
        s.store_scalar(1069, if s.b[1069] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1069]) {
            s.store_add_scaled_inputs3_offset_indices(77, 314, p.p548, 316, p.p549, 318, p.p550, p.p547);
        }

        s.b[1070] = (((param_given[551] || param_given[552]) || param_given[553]) || param_given[554]);
        s.store_scalar(1070, if s.b[1070] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1070]) {
            s.store_add_scaled_inputs3_offset_indices(79, 314, p.p552, 316, p.p553, 318, p.p554, p.p551);
        }

        s.b[1071] = (((param_given[555] || param_given[556]) || param_given[557]) || param_given[558]);
        s.store_scalar(1071, if s.b[1071] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1071]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(82, 316, s.ad_value(314), p.p556, s.ad_value(316), p.p557, s.ad_value(318), p.p558, p.p555);
        }

        s.b[1072] = (((param_given[559] || param_given[560]) || param_given[561]) || param_given[562]);
        s.store_scalar(1072, if s.b[1072] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1072]) {
            s.store_add_scaled_inputs3_offset_indices(83, 314, p.p560, 316, p.p561, 318, p.p562, p.p559);
        }

        s.b[1073] = (((param_given[563] || param_given[564]) || param_given[565]) || param_given[566]);
        s.store_scalar(1073, if s.b[1073] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1073]) {
            s.store_add_scaled_inputs3_offset_indices(84, 314, p.p564, 316, p.p565, 318, p.p566, p.p563);
        }

        s.b[1074] = (((param_given[567] || param_given[568]) || param_given[569]) || param_given[570]);
        s.store_scalar(1074, if s.b[1074] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1074]) {
            s.store_add_scaled_inputs3_offset_indices(85, 314, p.p568, 316, p.p569, 318, p.p570, p.p567);
        }

        s.b[1075] = (((param_given[571] || param_given[572]) || param_given[573]) || param_given[574]);
        s.store_scalar(1075, if s.b[1075] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1075]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(86, 314, s.ad_value(314), p.p572, s.ad_value(316), p.p573, s.ad_value(318), p.p574, p.p571);
        }

        s.b[1076] = (((param_given[575] || param_given[576]) || param_given[577]) || param_given[578]);
        s.store_scalar(1076, if s.b[1076] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1076]) {
            s.store_add_scaled_inputs3_offset_indices(87, 314, p.p576, 316, p.p577, 318, p.p578, p.p575);
        }

        s.b[1077] = (((param_given[579] || param_given[580]) || param_given[581]) || param_given[582]);
        s.store_scalar(1077, if s.b[1077] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1077]) {
            s.store_add_scaled_inputs3_offset_indices(88, 314, p.p580, 316, p.p581, 318, p.p582, p.p579);
        }

        s.b[1078] = (((param_given[583] || param_given[584]) || param_given[585]) || param_given[586]);
        s.store_scalar(1078, if s.b[1078] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1078]) {
            s.store_add_scaled_inputs3_offset_indices(89, 314, p.p584, 316, p.p585, 318, p.p586, p.p583);
        }

        s.b[1079] = (((param_given[587] || param_given[588]) || param_given[589]) || param_given[590]);
        s.store_scalar(1079, if s.b[1079] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1079]) {
            s.store_add_scaled_inputs3_offset_indices(91, 314, p.p588, 316, p.p589, 318, p.p590, p.p587);
        }

        s.b[1080] = (((param_given[591] || param_given[592]) || param_given[593]) || param_given[594]);
        s.store_scalar(1080, if s.b[1080] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1080]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(92, 314, s.ad_value(314), p.p592, s.ad_value(316), p.p593, s.ad_value(318), p.p594, p.p591);
        }

        s.b[1081] = (((param_given[595] || param_given[596]) || param_given[597]) || param_given[598]);
        s.store_scalar(1081, if s.b[1081] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1081]) {
            s.store_add_scaled_inputs3_offset_indices(93, 314, p.p596, 316, p.p597, 318, p.p598, p.p595);
        }

        s.b[1082] = (((param_given[599] || param_given[600]) || param_given[601]) || param_given[602]);
        s.store_scalar(1082, if s.b[1082] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1082]) {
            s.store_add_scaled_inputs3_offset_indices(94, 314, p.p600, 316, p.p601, 318, p.p602, p.p599);
        }

        s.b[1083] = (((param_given[603] || param_given[604]) || param_given[605]) || param_given[606]);
        s.store_scalar(1083, if s.b[1083] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1083]) {
            s.store_add_scaled_inputs3_offset_indices(96, 314, p.p604, 316, p.p605, 318, p.p606, p.p603);
        }

        s.b[1084] = (((param_given[607] || param_given[608]) || param_given[609]) || param_given[610]);
        s.store_scalar(1084, if s.b[1084] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1084]) {
            s.store_add_scaled_inputs3_offset_indices(98, 314, p.p608, 316, p.p609, 318, p.p610, p.p607);
        }

        s.b[1085] = (((param_given[611] || param_given[612]) || param_given[613]) || param_given[614]);
        s.store_scalar(1085, if s.b[1085] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1085]) {
            s.store_add_scaled_inputs3_offset_indices(99, 314, p.p612, 316, p.p613, 318, p.p614, p.p611);
        }

        s.b[1086] = (((param_given[615] || param_given[616]) || param_given[617]) || param_given[618]);
        s.store_scalar(1086, if s.b[1086] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1086]) {
            s.store_add_scaled_inputs3_offset_indices(100, 314, p.p616, 316, p.p617, 318, p.p618, p.p615);
        }

        s.b[1087] = (((param_given[619] || param_given[620]) || param_given[621]) || param_given[622]);
        s.store_scalar(1087, if s.b[1087] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1087]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(103, 319, s.ad_value(314), p.p620, s.ad_value(316), p.p621, s.ad_value(318), p.p622, p.p619);
        }

        s.b[1088] = (((param_given[623] || param_given[624]) || param_given[625]) || param_given[626]);
        s.store_scalar(1088, if s.b[1088] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1088]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(104, 317, s.ad_value(314), p.p624, s.ad_value(316), p.p625, s.ad_value(318), p.p626, p.p623);
        }

        s.b[1089] = (((param_given[627] || param_given[628]) || param_given[629]) || param_given[630]);
        s.store_scalar(1089, if s.b[1089] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1089]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(105, 317, s.ad_value(314), p.p628, s.ad_value(316), p.p629, s.ad_value(318), p.p630, p.p627);
        }

        s.b[1090] = (((param_given[631] || param_given[632]) || param_given[633]) || param_given[634]);
        s.store_scalar(1090, if s.b[1090] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1090]) {
            s.store_add_scaled_inputs3_offset_indices(106, 314, p.p632, 316, p.p633, 318, p.p634, p.p631);
        }

        s.b[1091] = (((param_given[635] || param_given[636]) || param_given[637]) || param_given[638]);
        s.store_scalar(1091, if s.b[1091] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1091]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(114, 317, s.ad_value(314), p.p636, s.ad_value(316), p.p637, s.ad_value(318), p.p638, p.p635);
        }

        s.b[1092] = (((param_given[639] || param_given[640]) || param_given[641]) || param_given[642]);
        s.store_scalar(1092, if s.b[1092] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1092]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(115, 317, s.ad_value(314), p.p640, s.ad_value(316), p.p641, s.ad_value(318), p.p642, p.p639);
        }

        s.b[1093] = (((param_given[643] || param_given[644]) || param_given[645]) || param_given[646]);
        s.store_scalar(1093, if s.b[1093] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1093]) {
            s.store_add_scaled_inputs3_offset_indices(118, 314, p.p644, 316, p.p645, 318, p.p646, p.p643);
        }

        s.b[1094] = (((param_given[647] || param_given[648]) || param_given[649]) || param_given[650]);
        s.store_scalar(1094, if s.b[1094] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1094]) {
            s.store_add_scaled_inputs3_offset_indices(119, 314, p.p648, 316, p.p649, 318, p.p650, p.p647);
        }

        s.b[1095] = (((param_given[651] || param_given[652]) || param_given[653]) || param_given[654]);
        s.store_scalar(1095, if s.b[1095] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1095]) {
            s.store_mul_ad_affine_product_rhs(122, 322, s.ad_value(320), A::add_scaled_inputs3_offset(s.ad_value(314), p.p652, s.ad_value(316), p.p653, s.ad_value(318), p.p654, p.p651), 1.0 / (1e-6), 0.0);
        }

        s.b[1096] = (((param_given[655] || param_given[656]) || param_given[657]) || param_given[658]);
        s.store_scalar(1096, if s.b[1096] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1096]) {
            s.store_add_scaled_inputs3_offset_indices(123, 314, p.p656, 316, p.p657, 318, p.p658, p.p655);
        }

        s.b[1097] = (((param_given[659] || param_given[660]) || param_given[661]) || param_given[662]);
        s.store_scalar(1097, if s.b[1097] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1097]) {
            s.store_add_scaled_inputs3_offset_indices(124, 314, p.p660, 316, p.p661, 318, p.p662, p.p659);
        }

        s.b[1098] = (((((((param_given[663] || param_given[664]) || param_given[665]) || param_given[666]) || param_given[571]) || param_given[572]) || param_given[573]) || param_given[574]);
        s.store_scalar(1098, if s.b[1098] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1098]) {
            s.store_scalar(32, p.p571);
        }

        s.b[1099] = param_given[663];
        s.store_scalar(1099, if s.b[1099] { 1.0 } else { 0.0 });

        if ((s.b[1030] && s.b[1098]) && s.b[1099]) {
            s.store_scalar(32, p.p663);
        }

        if (s.b[1030] && s.b[1098]) {
            s.store_scalar(33, p.p572);
        }

        s.b[1100] = param_given[664];
        s.store_scalar(1100, if s.b[1100] { 1.0 } else { 0.0 });

        if ((s.b[1030] && s.b[1098]) && s.b[1100]) {
            s.store_scalar(33, p.p664);
        }

        if (s.b[1030] && s.b[1098]) {
            s.store_scalar(34, p.p573);
        }

        s.b[1101] = param_given[665];
        s.store_scalar(1101, if s.b[1101] { 1.0 } else { 0.0 });

        if ((s.b[1030] && s.b[1098]) && s.b[1101]) {
            s.store_scalar(34, p.p665);
        }

        if (s.b[1030] && s.b[1098]) {
            s.store_scalar(35, p.p574);
        }

        s.b[1102] = param_given[666];
        s.store_scalar(1102, if s.b[1102] { 1.0 } else { 0.0 });

        if ((s.b[1030] && s.b[1098]) && s.b[1102]) {
            s.store_scalar(35, p.p666);
        }

        if (s.b[1030] && s.b[1098]) {
            s.store_mul_ad_rhs(125, 314, A::add_scaled_value_products3(s.ad_value(32), 1.0, s.ad_value(33), s.ad_value(314), 1.0, s.ad_value(34), s.ad_value(316), 1.0, s.ad_value(35), s.ad_value(318), 1.0));
        }

        s.b[1103] = (((((((param_given[667] || param_given[668]) || param_given[669]) || param_given[670]) || param_given[587]) || param_given[588]) || param_given[589]) || param_given[590]);
        s.store_scalar(1103, if s.b[1103] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1103]) {
            s.store_scalar(32, p.p587);
        }

        s.b[1104] = param_given[667];
        s.store_scalar(1104, if s.b[1104] { 1.0 } else { 0.0 });

        if ((s.b[1030] && s.b[1103]) && s.b[1104]) {
            s.store_scalar(32, p.p667);
        }

        if (s.b[1030] && s.b[1103]) {
            s.store_scalar(33, p.p588);
        }

        s.b[1105] = param_given[668];
        s.store_scalar(1105, if s.b[1105] { 1.0 } else { 0.0 });

        if ((s.b[1030] && s.b[1103]) && s.b[1105]) {
            s.store_scalar(33, p.p668);
        }

        if (s.b[1030] && s.b[1103]) {
            s.store_scalar(34, p.p589);
        }

        s.b[1106] = param_given[669];
        s.store_scalar(1106, if s.b[1106] { 1.0 } else { 0.0 });

        if ((s.b[1030] && s.b[1103]) && s.b[1106]) {
            s.store_scalar(34, p.p669);
        }

    }

    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[1030] && s.b[1103]) {
            s.store_scalar(35, p.p590);
        }

        s.b[1107] = param_given[670];
        s.store_scalar(1107, if s.b[1107] { 1.0 } else { 0.0 });

        if ((s.b[1030] && s.b[1103]) && s.b[1107]) {
            s.store_scalar(35, p.p670);
        }

        if (s.b[1030] && s.b[1103]) {
            s.store_add_scaled_value_products3_indices(126, 32, 1.0, 33, 314, 1.0, 34, 316, 1.0, 35, 318, 1.0);
        }

        s.b[1108] = (((param_given[671] || param_given[672]) || param_given[673]) || param_given[674]);
        s.store_scalar(1108, if s.b[1108] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1108]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(127, 314, s.ad_value(314), p.p672, s.ad_value(316), p.p673, s.ad_value(318), p.p674, p.p671);
        }

        s.b[1109] = (((param_given[675] || param_given[676]) || param_given[677]) || param_given[678]);
        s.store_scalar(1109, if s.b[1109] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1109]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(128, 314, s.ad_value(314), p.p676, s.ad_value(316), p.p677, s.ad_value(318), p.p678, p.p675);
        }

        s.b[1110] = (((param_given[679] || param_given[680]) || param_given[681]) || param_given[682]);
        s.store_scalar(1110, if s.b[1110] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1110]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(129, 322, s.ad_value(314), p.p680, s.ad_value(316), p.p681, s.ad_value(318), p.p682, p.p679);
        }

        s.b[1111] = (((param_given[683] || param_given[684]) || param_given[685]) || param_given[686]);
        s.store_scalar(1111, if s.b[1111] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1111]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(130, 322, s.ad_value(314), p.p684, s.ad_value(316), p.p685, s.ad_value(318), p.p686, p.p683);
        }

        s.b[1112] = (((param_given[687] || param_given[688]) || param_given[689]) || param_given[690]);
        s.store_scalar(1112, if s.b[1112] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1112]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(134, 325, s.ad_value(314), p.p688, s.ad_value(316), p.p689, s.ad_value(318), p.p690, p.p687);
        }

        s.b[1113] = (((param_given[691] || param_given[692]) || param_given[693]) || param_given[694]);
        s.store_scalar(1113, if s.b[1113] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1113]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(135, 322, s.ad_value(314), p.p692, s.ad_value(316), p.p693, s.ad_value(318), p.p694, p.p691);
        }

        s.b[1114] = (((param_given[695] || param_given[696]) || param_given[697]) || param_given[698]);
        s.store_scalar(1114, if s.b[1114] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1114]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(136, 322, s.ad_value(314), p.p696, s.ad_value(316), p.p697, s.ad_value(318), p.p698, p.p695);
        }

        s.b[1115] = (((param_given[699] || param_given[700]) || param_given[701]) || param_given[702]);
        s.store_scalar(1115, if s.b[1115] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1115]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(141, 326, s.ad_value(314), p.p700, s.ad_value(316), p.p701, s.ad_value(318), p.p702, p.p699);
        }

        s.b[1116] = (((param_given[703] || param_given[704]) || param_given[705]) || param_given[706]);
        s.store_scalar(1116, if s.b[1116] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1116]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(142, 326, s.ad_value(314), p.p704, s.ad_value(316), p.p705, s.ad_value(318), p.p706, p.p703);
        }

        s.b[1121] = (((param_given[723] || param_given[724]) || param_given[725]) || param_given[726]);
        s.store_scalar(1121, if s.b[1121] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1121]) {
            s.store_add_scaled_inputs3_offset_indices(149, 314, p.p724, 316, p.p725, 318, p.p726, p.p723);
        }

        s.b[1122] = (((param_given[727] || param_given[728]) || param_given[729]) || param_given[730]);
        s.store_scalar(1122, if s.b[1122] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1122]) {
            s.store_add_scaled_inputs3_offset_indices(150, 314, p.p728, 316, p.p729, 318, p.p730, p.p727);
        }

        s.b[1123] = (((param_given[731] || param_given[732]) || param_given[733]) || param_given[734]);
        s.store_scalar(1123, if s.b[1123] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1123]) {
            s.store_add_scaled_inputs3_offset_indices(151, 314, p.p732, 316, p.p733, 318, p.p734, p.p731);
        }

        s.b[1124] = (((param_given[735] || param_given[736]) || param_given[737]) || param_given[738]);
        s.store_scalar(1124, if s.b[1124] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1124]) {
            s.store_add_scaled_inputs3_offset_indices(152, 314, p.p736, 316, p.p737, 318, p.p738, p.p735);
        }

        s.b[1125] = (((param_given[739] || param_given[740]) || param_given[741]) || param_given[742]);
        s.store_scalar(1125, if s.b[1125] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1125]) {
            s.store_add_scaled_inputs3_offset_indices(153, 314, p.p740, 316, p.p741, 318, p.p742, p.p739);
        }

        s.b[1126] = (((param_given[743] || param_given[744]) || param_given[745]) || param_given[746]);
        s.store_scalar(1126, if s.b[1126] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1126]) {
            s.store_mul_div_scaled_inputs_mixed_aii(154, A::add_scaled_inputs3_offset(s.ad_value(314), p.p744, s.ad_value(316), p.p745, s.ad_value(318), p.p746, p.p743), 344, 1.0, 312, 1.0);
        }

        s.b[1127] = (((param_given[747] || param_given[748]) || param_given[749]) || param_given[750]);
        s.store_scalar(1127, if s.b[1127] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1127]) {
            s.store_add_scaled_inputs3_offset_indices(155, 314, p.p748, 316, p.p749, 318, p.p750, p.p747);
        }

        s.b[1128] = (((param_given[751] || param_given[752]) || param_given[753]) || param_given[754]);
        s.store_scalar(1128, if s.b[1128] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1128]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(156, 315, s.ad_value(314), p.p752, s.ad_value(316), p.p753, s.ad_value(318), p.p754, p.p751);
        }

        s.b[1129] = (((param_given[755] || param_given[756]) || param_given[757]) || param_given[758]);
        s.store_scalar(1129, if s.b[1129] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1129]) {
            s.store_add_scaled_inputs3_offset_indices(157, 314, p.p756, 316, p.p757, 318, p.p758, p.p755);
        }

        s.b[1130] = (((param_given[759] || param_given[760]) || param_given[761]) || param_given[762]);
        s.store_scalar(1130, if s.b[1130] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1130]) {
            s.store_add_scaled_inputs3_offset_indices(158, 314, p.p760, 316, p.p761, 318, p.p762, p.p759);
        }

        s.b[1131] = (((param_given[763] || param_given[764]) || param_given[765]) || param_given[766]);
        s.store_scalar(1131, if s.b[1131] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1131]) {
            s.store_mul_add_scaled_inputs3_offset_rhs(159, 315, s.ad_value(314), p.p764, s.ad_value(316), p.p765, s.ad_value(318), p.p766, p.p763);
        }

        s.b[1132] = (((param_given[771] || param_given[772]) || param_given[773]) || param_given[774]);
        s.store_scalar(1132, if s.b[1132] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1132]) {
            s.store_add_scaled_inputs3_offset_indices(160, 314, p.p772, 316, p.p773, 318, p.p774, p.p771);
        }

        s.b[1133] = (((param_given[767] || param_given[768]) || param_given[769]) || param_given[770]);
        s.store_scalar(1133, if s.b[1133] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1133]) {
            s.store_add_scaled_inputs3_offset_indices(161, 314, p.p768, 316, p.p769, 318, p.p770, p.p767);
        }

        s.b[1137] = (((param_given[787] || param_given[788]) || param_given[789]) || param_given[790]);
        s.store_scalar(1137, if s.b[1137] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1137]) {
            s.store_add_scaled_inputs3_offset_indices(176, 314, p.p788, 316, p.p789, 318, p.p790, p.p787);
        }

        if s.b[1030] {
            s.store_scalar(1019, 0.0);
            s.store_scalar(1020, 0.0);
            s.store_scalar(1018, 0.0);
            s.store_scalar(43, p.p795);
        }

        s.b[1138] = param_given[796];
        s.store_scalar(1138, if s.b[1138] { 1.0 } else { 0.0 });

        if (s.b[1030] && s.b[1138]) {
            s.store_scalar(43, p.p796);
        }

        s.b[1139] = (((s.v[9] > 0.0) && (s.v[10] > 0.0)) && ((s.v[5] == 1.0) || ((s.v[5] > 1.0) && (s.v[11] > 0.0))));
        s.store_scalar(1139, if s.b[1139] { 1.0 } else { 0.0 });

        let mut assign9340_loop_guard: usize = 0;
        while {
            let assign9340_cond_e9222: f64 = (s.v[5] - 0.5);
            let assign9340_cond_e9224: f64 = if ((s.b[1030] && s.b[1139]) && (s.v[1018] < assign9340_cond_e9222)) { 1.0 } else { 0.0 };
            assign9340_cond_e9224 != 0.0
        } {
            assign9340_loop_guard += 1;
            assert!(assign9340_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1030] && s.b[1139]) {
                s.store_add_ad_rhs(1019, 1019, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1018), (s.v[11] + s.v[7]), (s.v[9] + (0.5 * s.v[7])))));
                s.store_add_ad_rhs(1020, 1020, A::div_from_scalar(1.0, A::scale_offset(s.ad_value(1018), (s.v[11] + s.v[7]), (s.v[10] + (0.5 * s.v[7])))));
                s.store_offset(1018, 1018, 1.0);
            }
        }

        if (s.b[1030] && s.b[1139]) {
            s.store_mul(1003, 1019, 6);
            s.store_mul(1004, 1020, 6);
            s.store_scalar(1005, (1.0 / (p.p791 + (0.5 * s.v[7]))));
            s.store_scalar(1006, (1.0 / (p.p792 + (0.5 * s.v[7]))));
        }

        if (s.b[1030] && s.b[1139]) {
            if ((s.v[7] + s.v[310]) > 1e-9) {
                s.store_offset(1016, 310, s.v[7]);
            } else {
                s.store_scalar(1016, 1e-9);
            }
        }

        if (s.b[1030] && s.b[1139]) {
            if (((s.v[8] + s.v[311]) + p.p793) > 1e-9) {
                s.store_offset_add(1017, 8, 311, p.p793);
            } else {
                s.store_scalar(1017, 1e-9);
            }
        }

        if (s.b[1030] && s.b[1139]) {
            s.store_div_from_scalar_powf_ad(1014, 1.0, s.ad_value(1016), p.p801);
            s.store_div_from_scalar_powf_ad(1015, 1.0, s.ad_value(1017), p.p802);
            s.store_add_scaled_inputs_product_first_ad(1007, A::scale_offset(s.ad_value(1014), p.p798, 1.0), (1.0 + (p.p797 * (s.v[352] - 1.0))), 1015, (p.p799 * (1.0 + (p.p797 * (s.v[352] - 1.0)))), 1014, 1015, (p.p800 * (1.0 + (p.p797 * (s.v[352] - 1.0)))));
            s.store_div_scaled_inputs2_indices(1008, 1003, p.p794, 1004, p.p794, 1007, 1.0);
            s.store_div_scaled_inputs2_indices(1009, 1005, p.p794, 1006, p.p794, 1007, 1.0);
            s.store_div_from_scalar_powf_ad(1014, 1.0, s.ad_value(1016), p.p807);
            s.store_div_from_scalar_powf_ad(1015, 1.0, s.ad_value(1017), p.p808);
            s.store_add_scaled_inputs_product_first_ad(1010, A::scale_offset(s.ad_value(1014), p.p804, 1.0), 1.0, 1015, p.p805, 1014, 1015, p.p806);
            s.store_add_scaled_inputs4_indices(1012, 1003, 1.0, 1004, 1.0, 1005, -1.0, 1006, -1.0);
            s.store_div_scaled_offset_numerator(1013, s.ad_value(1008), 1.0, 1.0, A::offset(s.ad_value(1009), 1.0), 1.0);
            s.store_mul(69, 69, 1013);
            s.store_div_scaled_product3_mixed_iiaa(86, 86, 1013, A::scale_offset(s.ad_value(1009), p.p795, 1.0), 1.0, A::scale_offset(s.ad_value(1008), p.p795, 1.0), 1.0);
            s.store_div_scaled_product3_mixed_iiaa(125, 125, 1013, A::offset(A::mul(s.ad_value(43), s.ad_value(1009)), 1.0), 1.0, A::offset(A::mul(s.ad_value(43), s.ad_value(1008)), 1.0), 1.0);
            s.store_mul(154, 154, 1013);
            s.store_div_scaled_inputs_indices(1013, 1012, p.p803, 1010, 1.0);
            s.store_add(44, 44, 1013);
            s.store_add(149, 149, 1013);
            s.store_div_scaled_inputs_mixed_ia(1013, 1012, p.p809, A::powf(s.ad_value(1010), p.p810), 1.0);
            s.store_add(66, 66, 1013);
            s.store_add(159, 159, 1013);
        }

        s.b[1140] = ((((s.v[15] > 0.0) || (s.v[16] > 0.0)) || (s.v[17] > 0.0)) || (s.v[12] > 0.0));
        s.store_scalar(1140, if s.b[1140] { 1.0 } else { 0.0 });

        s.b[1141] = (((s.v[15] == 0.0) && (s.v[16] == 0.0)) && (s.v[17] == 0.0));
        s.store_scalar(1141, if s.b[1141] { 1.0 } else { 0.0 });

        if ((s.b[1030] && s.b[1140]) && s.b[1141]) {
            s.store_offset(1012, 8, s.v[12]);
            s.store_scalar(1013, (1.0 / p.p811));
            s.store_div_from_scalar_scaled_input(15, (p.p811 * p.p811), 1012, s.v[12]);
            s.store_div_scaled_add_product(16, A::exp_scaled_input(s.ad_value(1013), ((-10.0) * s.v[12])), ((0.1 * s.v[12]) + (0.01 * p.p811)), A::scale_offset(s.ad_value(1012), 0.1, (0.01 * p.p811)), A::exp(A::mul_scaled_lhs(s.ad_value(1012), (-10.0), s.ad_value(1013))), (-1.0), s.ad_value(8), 1.0);
            s.store_div_scaled_add_product(17, A::exp_scaled_input(s.ad_value(1013), ((-20.0) * s.v[12])), ((0.05 * s.v[12]) + (0.0025 * p.p811)), A::scale_offset(s.ad_value(1012), 0.05, (0.0025 * p.p811)), A::exp(A::mul_scaled_lhs(s.ad_value(1012), (-20.0), s.ad_value(1013))), (-1.0), s.ad_value(8), 1.0);
        }

        if (s.b[1030] && s.b[1140]) {
            s.store_add_scaled_inputs3_indices(1012, 15, 1.0, 16, p.p812, 17, p.p813);
            s.store_add_scaled_product_indices(44, 44, 1.0, 348, 1012, 1.0);
            s.store_mul_offset_ad_rhs(69, 69, A::mul(s.ad_value(349), s.ad_value(1012)), 1.0);
            s.store_add_scaled_product_indices(149, 149, 1.0, 348, 1012, 1.0);
            s.store_mul_offset_ad_rhs(154, 154, A::mul(s.ad_value(349), s.ad_value(1012)), 1.0);
        }

        s.copy_ad(177, 44);

        s.copy_ad(178, 45);

        s.copy_ad(179, 46);

        s.copy_ad(181, 47);

        s.copy_ad(182, 48);

        if (s.v[49] > 1e20) {
            if (s.v[49] < 1e26) {
                s.copy_ad(183, 49);
            } else {
                s.store_scalar(183, 1e26);
            }
        } else {
            s.store_scalar(183, 1e20);
        }

        if (s.v[50] > 0.01) {
            s.copy_ad(184, 50);
        } else {
            s.store_scalar(184, 0.01);
        }

        if (s.v[51] > 0.0) {
            s.copy_ad(185, 51);
        } else {
            s.store_scalar(185, 0.0);
        }

        s.copy_ad(186, 52);

        s.copy_ad(187, 53);

        if (s.v[54] > 0.0) {
            s.copy_ad(188, 54);
        } else {
            s.store_scalar(188, 0.0);
        }

        s.copy_ad(192, 59);

        s.copy_ad(193, 60);

        if (s.v[61] > 1e23) {
            if (s.v[61] < 1e27) {
                s.copy_ad(194, 61);
            } else {
                s.store_scalar(194, 1e27);
            }
        } else {
            s.store_scalar(194, 1e23);
        }

        if (s.v[62] > 1e23) {
            if (s.v[62] < 1e27) {
                s.copy_ad(195, 62);
            } else {
                s.store_scalar(195, 1e27);
            }
        } else {
            s.store_scalar(195, 1e23);
        }

        if (s.v[55] > 0.0) {
            s.copy_ad(189, 55);
        } else {
            s.store_scalar(189, 0.0);
        }

        if (s.v[57] > 0.0) {
            if (s.v[57] < 0.5) {
                s.copy_ad(191, 57);
            } else {
                s.store_scalar(191, 0.5);
            }
        } else {
            s.store_scalar(191, 0.0);
        }

        if (s.v[56] > 0.0) {
            if (s.v[56] < 1.0) {
                s.copy_ad(190, 56);
            } else {
                s.store_scalar(190, 1.0);
            }
        } else {
            s.store_scalar(190, 0.0);
        }

        s.copy_ad(180, 58);

        if (s.v[66] > 0.0) {
            s.copy_ad(196, 66);
        } else {
            s.store_scalar(196, 0.0);
        }

        if (s.v[68] > 0.0) {
            if (s.v[68] < 1.0) {
                s.copy_ad(198, 68);
            } else {
                s.store_scalar(198, 1.0);
            }
        } else {
            s.store_scalar(198, 0.0);
        }

        if (s.v[67] > 0.0) {
            s.copy_ad(197, 67);
        } else {
            s.store_scalar(197, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.v[63] > 0.0) {
            s.copy_ad(199, 63);
        } else {
            s.store_scalar(199, 0.0);
        }

        if (s.v[65] > 0.0) {
            if (s.v[65] < 1.0) {
                s.copy_ad(200, 65);
            } else {
                s.store_scalar(200, 1.0);
            }
        } else {
            s.store_scalar(200, 0.0);
        }

        if (s.v[64] > 0.0) {
            s.copy_ad(201, 64);
        } else {
            s.store_scalar(201, 0.0);
        }

        if (s.v[69] > 0.0) {
            s.copy_ad(202, 69);
        } else {
            s.store_scalar(202, 0.0);
        }

        s.copy_ad(203, 70);

        if (s.v[71] > 0.0) {
            s.copy_ad(204, 71);
        } else {
            s.store_scalar(204, 0.0);
        }

        s.copy_ad(205, 72);

        if (s.v[73] > 0.0) {
            s.copy_ad(206, 73);
        } else {
            s.store_scalar(206, 0.0);
        }

        s.copy_ad(207, 74);

        if (s.v[75] > 0.0) {
            s.copy_ad(208, 75);
        } else {
            s.store_scalar(208, 0.0);
        }

        s.copy_ad(209, 76);

        if (s.v[77] > 0.0) {
            s.copy_ad(210, 77);
        } else {
            s.store_scalar(210, 0.0);
        }

        s.copy_ad(211, 78);

        if (s.v[79] > 0.0) {
            s.copy_ad(212, 79);
        } else {
            s.store_scalar(212, 0.0);
        }

        s.copy_ad(213, 80);

        s.copy_ad(214, 81);

        if (s.v[82] > 0.0) {
            s.copy_ad(215, 82);
        } else {
            s.store_scalar(215, 0.0);
        }

        s.copy_ad(216, 83);

        if (s.v[84] > (-0.5)) {
            if (s.v[84] < 1.0) {
                s.copy_ad(217, 84);
            } else {
                s.store_scalar(217, 1.0);
            }
        } else {
            s.store_scalar(217, (-0.5));
        }

        if (s.v[85] > (-0.5)) {
            s.copy_ad(218, 85);
        } else {
            s.store_scalar(218, (-0.5));
        }

        if (s.v[86] > 0.0) {
            s.copy_ad(219, 86);
        } else {
            s.store_scalar(219, 0.0);
        }

        s.copy_ad(220, 87);

        if (s.v[88] > (-0.5)) {
            if (s.v[88] < 1.0) {
                s.copy_ad(221, 88);
            } else {
                s.store_scalar(221, 1.0);
            }
        } else {
            s.store_scalar(221, (-0.5));
        }

        if (s.v[89] > (-0.5)) {
            s.copy_ad(222, 89);
        } else {
            s.store_scalar(222, (-0.5));
        }

        if (s.v[90] > 0.01) {
            s.copy_ad(223, 90);
        } else {
            s.store_scalar(223, 0.01);
        }

        if (s.v[91] > 2.0) {
            s.copy_ad(224, 91);
        } else {
            s.store_scalar(224, 2.0);
        }

        if (s.v[92] > 0.0) {
            s.copy_ad(225, 92);
        } else {
            s.store_scalar(225, 0.0);
        }

        if (s.v[93] > 0.0) {
            s.copy_ad(226, 93);
        } else {
            s.store_scalar(226, 0.0);
        }

        if (s.v[94] > 0.0) {
            s.copy_ad(227, 94);
        } else {
            s.store_scalar(227, 0.0);
        }

        s.copy_ad(228, 95);

        if (s.v[96] > 0.0) {
            s.copy_ad(229, 96);
        } else {
            s.store_scalar(229, 0.0);
        }

        s.copy_ad(230, 97);

        s.copy_ad(231, 98);

        if (s.v[99] > 0.0) {
            s.copy_ad(232, 99);
        } else {
            s.store_scalar(232, 0.0);
        }

        if (s.v[100] > 0.0) {
            s.copy_ad(233, 100);
        } else {
            s.store_scalar(233, 0.0);
        }

        if (s.v[101] > 1e-12) {
            s.copy_ad(234, 101);
        } else {
            s.store_scalar(234, 1e-12);
        }

        s.copy_ad(235, 102);

        if (s.v[103] > 0.0) {
            s.copy_ad(236, 103);
        } else {
            s.store_scalar(236, 0.0);
        }

        if (s.v[104] > 0.0) {
            s.copy_ad(237, 104);
        } else {
            s.store_scalar(237, 0.0);
        }

        if (s.v[105] > 0.0) {
            s.copy_ad(238, 105);
        } else {
            s.store_scalar(238, 0.0);
        }

        s.copy_ad(239, 106);

        s.copy_ad(240, 107);

        s.copy_ad(241, 108);

        s.copy_ad(242, 109);

        s.copy_ad(243, 110);

        s.copy_ad(244, 111);

        s.copy_ad(245, 112);

        s.copy_ad(246, 113);

        if (s.v[114] > 0.0) {
            s.copy_ad(247, 114);
        } else {
            s.store_scalar(247, 0.0);
        }

        if (s.v[115] > 0.0) {
            s.copy_ad(248, 115);
        } else {
            s.store_scalar(248, 0.0);
        }

        s.copy_ad(249, 116);

        s.copy_ad(250, 117);

        s.copy_ad(251, 118);

        s.copy_ad(252, 119);

        s.copy_ad(253, 120);

        s.copy_ad(254, 121);

        if (s.v[122] > 0.0) {
            s.copy_ad(255, 122);
        } else {
            s.store_scalar(255, 0.0);
        }

        s.copy_ad(256, 123);

        if (s.v[124] > 0.0) {
            s.copy_ad(257, 124);
        } else {
            s.store_scalar(257, 0.0);
        }

        if (s.v[125] > 0.0) {
            s.copy_ad(258, 125);
        } else {
            s.store_scalar(258, 0.0);
        }

        if (s.v[126] > 2.0) {
            s.copy_ad(259, 126);
        } else {
            s.store_scalar(259, 2.0);
        }

        s.copy_ad(260, 127);

        if (s.v[128] > 0.0) {
            s.copy_ad(261, 128);
        } else {
            s.store_scalar(261, 0.0);
        }

        if (s.v[129] > 0.0) {
            s.copy_ad(262, 129);
        } else {
            s.store_scalar(262, 0.0);
        }

        if (s.v[130] > 0.0) {
            s.copy_ad(263, 130);
        } else {
            s.store_scalar(263, 0.0);
        }

        s.copy_ad(264, 131);

        s.copy_ad(265, 132);

        s.copy_ad(266, 133);

        if (s.v[134] > 0.0) {
            s.copy_ad(267, 134);
        } else {
            s.store_scalar(267, 0.0);
        }

        if (s.v[135] > 0.0) {
            s.copy_ad(268, 135);
        } else {
            s.store_scalar(268, 0.0);
        }

        if (s.v[136] > 0.0) {
            s.copy_ad(269, 136);
        } else {
            s.store_scalar(269, 0.0);
        }

        s.copy_ad(270, 137);

        s.copy_ad(271, 138);

        s.copy_ad(272, 139);

        s.copy_ad(273, 140);

        if (s.v[141] > 0.0) {
            s.copy_ad(274, 141);
        } else {
            s.store_scalar(274, 0.0);
        }

        if (s.v[142] > 0.0) {
            s.copy_ad(275, 142);
        } else {
            s.store_scalar(275, 0.0);
        }

        s.copy_ad(276, 143);

        s.copy_ad(282, 149);

        s.copy_ad(283, 150);

        s.copy_ad(284, 151);

        if (s.v[152] > 1e20) {
            if (s.v[152] < 1e26) {
                s.copy_ad(285, 152);
            } else {
                s.store_scalar(285, 1e26);
            }
        } else {
            s.store_scalar(285, 1e20);
        }

        if (s.v[153] > 0.0) {
            s.copy_ad(286, 153);
        } else {
            s.store_scalar(286, 0.0);
        }

        if (s.v[154] > 0.0) {
            s.copy_ad(287, 154);
        } else {
            s.store_scalar(287, 0.0);
        }

        s.copy_ad(288, 155);

        if (s.v[156] > 0.0) {
            s.copy_ad(289, 156);
        } else {
            s.store_scalar(289, 0.0);
        }

        if (s.v[157] > 0.0) {
            if (s.v[157] < 1.0) {
                s.copy_ad(290, 157);
            } else {
                s.store_scalar(290, 1.0);
            }
        } else {
            s.store_scalar(290, 0.0);
        }

        if (s.v[158] > 0.0) {
            s.copy_ad(291, 158);
        } else {
            s.store_scalar(291, 0.0);
        }

        if (s.v[159] > 0.0) {
            s.copy_ad(292, 159);
        } else {
            s.store_scalar(292, 0.0);
        }

        if (s.v[161] > 0.0) {
            if (s.v[161] < 1.0) {
                s.copy_ad(294, 161);
            } else {
                s.store_scalar(294, 1.0);
            }
        } else {
            s.store_scalar(294, 0.0);
        }

        if (s.v[160] > 0.0) {
            s.copy_ad(293, 160);
        } else {
            s.store_scalar(293, 0.0);
        }

        if ((p.p31 * s.v[5]) > 0.0) {
            s.store_scale(19, 5, p.p31);
        } else {
            s.store_scalar(19, 0.0);
        }

        s.store_scalar(20, p.p16);

        s.store_scalar(21, p.p15);

        s.store_scalar(22, p.p18);

        s.store_scalar(23, p.p17);

        if (s.v[176] > 0.0) {
            s.copy_ad(307, 176);
        } else {
            s.store_scalar(307, 0.0);
        }

        s.b[1142] = (p.p44 == 0.0);
        s.store_scalar(1142, if s.b[1142] { 1.0 } else { 0.0 });

        if s.b[1142] {
            s.copy_ad(193, 192);
            s.copy_ad(195, 194);
            s.copy_ad(248, 247);
            s.copy_ad(250, 249);
            s.copy_ad(252, 251);
            s.copy_ad(254, 253);
            s.copy_ad(238, 237);
            s.copy_ad(244, 242);
            s.copy_ad(245, 243);
            s.copy_ad(263, 262);
            s.copy_ad(265, 264);
            s.copy_ad(269, 268);
            s.copy_ad(275, 274);
        }

        s.store_scale(768, 182, 8.8541878176e-12);

        s.store_div(769, 768, 181);

        s.store_square(770, 181);

        s.store_scale(771, 769, 6.241449993689894e18);

        s.store_mul(772, 257, 183);

    }
}

#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_36(
        s: &mut ReactiveScratch,
    ) {
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
            s.store_neg_ad(2023, A::add(s.ad_value(2015), s.ad_value(2012)));
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
            s.store_neg_ad(2024, A::add(s.ad_value(2015), s.ad_value(2012)));
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
            s.store_neg_ad(2025, A::add(s.ad_value(2015), s.ad_value(2012)));
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

    }

    pub(super) fn stamp_reactive_block_37(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
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
            s.store_neg_ad(2026, A::add(s.ad_value(2015), s.ad_value(2012)));
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

        s.store_mul(860, 1904, 1895);

        s.b[3268] = ((s.v[1829] > 0.0) && (s.v[716] > 0.0));
        s.v[3268] = if s.b[3268] { 1.0 } else { 0.0 };

        s.b[3273] = ((((p.p50 == 1.0) && (s.v[719] > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0));
        s.v[3273] = if s.b[3273] { 1.0 } else { 0.0 };

        if (s.b[3268] && s.b[3273]) {
            s.store_div_scaled_product3_mixed_aiia(860, A::square(s.ad_value(1908)), 1904, 1895, 1.0, A::square(s.ad_value(1906)), 1.0);
        }

        s.b[3277] = (((p.p46 != 0.0) && (s.v[287] > 0.0)) && (s.v[1880] > 0.0));
        s.v[3277] = if s.b[3277] { 1.0 } else { 0.0 };

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
    ) {
        let (eq0_e955, eq0_e955_d_n0, eq0_e955_d_n1, eq0_e955_d_n2, eq0_e955_d_n3, eq0_e955_d_n4, eq0_e955_d_n5, eq0_e955_d_n6, eq0_e955_d_n7, eq0_e955_d_n8, eq0_e955_d_n9, eq0_e955_d_n10, eq0_e955_d_n11, eq0_e955_d_n12, eq0_e955_d_n13, eq0_e955_d_n14, eq0_e955_d_n15, eq0_e955_d_n16, eq0_e955_d_n17, eq0_e955_d_n18, eq0_e955_d_n19, eq0_e955_d_n20, eq0_e955_d_b0, eq0_e955_d_b1, eq0_e955_d_b2, eq0_e955_d_b3, eq0_e955_d_b4, eq0_e955_d_b5, eq0_e955_d_b6, eq0_e955_d_b7, eq0_e955_d_b8, eq0_e955_d_b9, eq0_e955_d_b10, eq0_e955_d_b11, eq0_e955_d_b12, eq0_e955_d_b13, eq0_e955_d_b14, eq0_e955_d_b15, eq0_e955_d_b16, eq0_e955_d_b17, eq0_e955_d_b18, eq0_e955_d_b19, eq0_e955_d_b20, eq0_e955_d_b21, eq0_e955_d_b22, eq0_e955_d_b23, eq0_e955_d_b24,) = {
    if s.b[2913] {
        let eq0_e949: f64 = (s.v[0] * s.v[19]);
        let eq0_e949_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq0_e949_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq0_e949_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq0_e949_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq0_e949_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq0_e949_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq0_e949_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq0_e949_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq0_e949_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq0_e949_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq0_e949_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq0_e949_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq0_e949_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq0_e949_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq0_e949_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq0_e949_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq0_e949_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq0_e949_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq0_e949_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq0_e949_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq0_e949_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq0_e949_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq0_e949_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq0_e949_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq0_e949_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq0_e949_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq0_e949_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq0_e949_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq0_e949_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq0_e949_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq0_e949_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq0_e949_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq0_e949_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq0_e949_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq0_e949_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq0_e949_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq0_e949_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq0_e949_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq0_e949_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq0_e949_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq0_e949_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq0_e949_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq0_e949_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq0_e949_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq0_e949_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq0_e949_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq0_e951: f64 = (eq0_e949 * p.p32);
        let eq0_e951_d_n0: f64 = (eq0_e949_d_n0 * p.p32);
        let eq0_e951_d_n1: f64 = (eq0_e949_d_n1 * p.p32);
        let eq0_e951_d_n2: f64 = (eq0_e949_d_n2 * p.p32);
        let eq0_e951_d_n3: f64 = (eq0_e949_d_n3 * p.p32);
        let eq0_e951_d_n4: f64 = (eq0_e949_d_n4 * p.p32);
        let eq0_e951_d_n5: f64 = (eq0_e949_d_n5 * p.p32);
        let eq0_e951_d_n6: f64 = (eq0_e949_d_n6 * p.p32);
        let eq0_e951_d_n7: f64 = (eq0_e949_d_n7 * p.p32);
        let eq0_e951_d_n8: f64 = (eq0_e949_d_n8 * p.p32);
        let eq0_e951_d_n9: f64 = (eq0_e949_d_n9 * p.p32);
        let eq0_e951_d_n10: f64 = (eq0_e949_d_n10 * p.p32);
        let eq0_e951_d_n11: f64 = (eq0_e949_d_n11 * p.p32);
        let eq0_e951_d_n12: f64 = (eq0_e949_d_n12 * p.p32);
        let eq0_e951_d_n13: f64 = (eq0_e949_d_n13 * p.p32);
        let eq0_e951_d_n14: f64 = (eq0_e949_d_n14 * p.p32);
        let eq0_e951_d_n15: f64 = (eq0_e949_d_n15 * p.p32);
        let eq0_e951_d_n16: f64 = (eq0_e949_d_n16 * p.p32);
        let eq0_e951_d_n17: f64 = (eq0_e949_d_n17 * p.p32);
        let eq0_e951_d_n18: f64 = (eq0_e949_d_n18 * p.p32);
        let eq0_e951_d_n19: f64 = (eq0_e949_d_n19 * p.p32);
        let eq0_e951_d_n20: f64 = (eq0_e949_d_n20 * p.p32);
        let eq0_e951_d_b0: f64 = (eq0_e949_d_b0 * p.p32);
        let eq0_e951_d_b1: f64 = (eq0_e949_d_b1 * p.p32);
        let eq0_e951_d_b2: f64 = (eq0_e949_d_b2 * p.p32);
        let eq0_e951_d_b3: f64 = (eq0_e949_d_b3 * p.p32);
        let eq0_e951_d_b4: f64 = (eq0_e949_d_b4 * p.p32);
        let eq0_e951_d_b5: f64 = (eq0_e949_d_b5 * p.p32);
        let eq0_e951_d_b6: f64 = (eq0_e949_d_b6 * p.p32);
        let eq0_e951_d_b7: f64 = (eq0_e949_d_b7 * p.p32);
        let eq0_e951_d_b8: f64 = (eq0_e949_d_b8 * p.p32);
        let eq0_e951_d_b9: f64 = (eq0_e949_d_b9 * p.p32);
        let eq0_e951_d_b10: f64 = (eq0_e949_d_b10 * p.p32);
        let eq0_e951_d_b11: f64 = (eq0_e949_d_b11 * p.p32);
        let eq0_e951_d_b12: f64 = (eq0_e949_d_b12 * p.p32);
        let eq0_e951_d_b13: f64 = (eq0_e949_d_b13 * p.p32);
        let eq0_e951_d_b14: f64 = (eq0_e949_d_b14 * p.p32);
        let eq0_e951_d_b15: f64 = (eq0_e949_d_b15 * p.p32);
        let eq0_e951_d_b16: f64 = (eq0_e949_d_b16 * p.p32);
        let eq0_e951_d_b17: f64 = (eq0_e949_d_b17 * p.p32);
        let eq0_e951_d_b18: f64 = (eq0_e949_d_b18 * p.p32);
        let eq0_e951_d_b19: f64 = (eq0_e949_d_b19 * p.p32);
        let eq0_e951_d_b20: f64 = (eq0_e949_d_b20 * p.p32);
        let eq0_e951_d_b21: f64 = (eq0_e949_d_b21 * p.p32);
        let eq0_e951_d_b22: f64 = (eq0_e949_d_b22 * p.p32);
        let eq0_e951_d_b23: f64 = (eq0_e949_d_b23 * p.p32);
        let eq0_e951_d_b24: f64 = (eq0_e949_d_b24 * p.p32);
        let eq0_e953: f64 = (eq0_e951 * s.v[847]);
        let eq0_e953_d_n0: f64 = ((eq0_e951_d_n0 * s.v[847]) + (eq0_e951 * s.dn[847][0]));
        let eq0_e953_d_n1: f64 = ((eq0_e951_d_n1 * s.v[847]) + (eq0_e951 * s.dn[847][1]));
        let eq0_e953_d_n2: f64 = ((eq0_e951_d_n2 * s.v[847]) + (eq0_e951 * s.dn[847][2]));
        let eq0_e953_d_n3: f64 = ((eq0_e951_d_n3 * s.v[847]) + (eq0_e951 * s.dn[847][3]));
        let eq0_e953_d_n4: f64 = ((eq0_e951_d_n4 * s.v[847]) + (eq0_e951 * s.dn[847][4]));
        let eq0_e953_d_n5: f64 = ((eq0_e951_d_n5 * s.v[847]) + (eq0_e951 * s.dn[847][5]));
        let eq0_e953_d_n6: f64 = ((eq0_e951_d_n6 * s.v[847]) + (eq0_e951 * s.dn[847][6]));
        let eq0_e953_d_n7: f64 = ((eq0_e951_d_n7 * s.v[847]) + (eq0_e951 * s.dn[847][7]));
        let eq0_e953_d_n8: f64 = ((eq0_e951_d_n8 * s.v[847]) + (eq0_e951 * s.dn[847][8]));
        let eq0_e953_d_n9: f64 = ((eq0_e951_d_n9 * s.v[847]) + (eq0_e951 * s.dn[847][9]));
        let eq0_e953_d_n10: f64 = ((eq0_e951_d_n10 * s.v[847]) + (eq0_e951 * s.dn[847][10]));
        let eq0_e953_d_n11: f64 = ((eq0_e951_d_n11 * s.v[847]) + (eq0_e951 * s.dn[847][11]));
        let eq0_e953_d_n12: f64 = ((eq0_e951_d_n12 * s.v[847]) + (eq0_e951 * s.dn[847][12]));
        let eq0_e953_d_n13: f64 = ((eq0_e951_d_n13 * s.v[847]) + (eq0_e951 * s.dn[847][13]));
        let eq0_e953_d_n14: f64 = ((eq0_e951_d_n14 * s.v[847]) + (eq0_e951 * s.dn[847][14]));
        let eq0_e953_d_n15: f64 = ((eq0_e951_d_n15 * s.v[847]) + (eq0_e951 * s.dn[847][15]));
        let eq0_e953_d_n16: f64 = ((eq0_e951_d_n16 * s.v[847]) + (eq0_e951 * s.dn[847][16]));
        let eq0_e953_d_n17: f64 = ((eq0_e951_d_n17 * s.v[847]) + (eq0_e951 * s.dn[847][17]));
        let eq0_e953_d_n18: f64 = ((eq0_e951_d_n18 * s.v[847]) + (eq0_e951 * s.dn[847][18]));
        let eq0_e953_d_n19: f64 = ((eq0_e951_d_n19 * s.v[847]) + (eq0_e951 * s.dn[847][19]));
        let eq0_e953_d_n20: f64 = ((eq0_e951_d_n20 * s.v[847]) + (eq0_e951 * s.dn[847][20]));
        let eq0_e953_d_b0: f64 = ((eq0_e951_d_b0 * s.v[847]) + (eq0_e951 * s.db[847][0]));
        let eq0_e953_d_b1: f64 = ((eq0_e951_d_b1 * s.v[847]) + (eq0_e951 * s.db[847][1]));
        let eq0_e953_d_b2: f64 = ((eq0_e951_d_b2 * s.v[847]) + (eq0_e951 * s.db[847][2]));
        let eq0_e953_d_b3: f64 = ((eq0_e951_d_b3 * s.v[847]) + (eq0_e951 * s.db[847][3]));
        let eq0_e953_d_b4: f64 = ((eq0_e951_d_b4 * s.v[847]) + (eq0_e951 * s.db[847][4]));
        let eq0_e953_d_b5: f64 = ((eq0_e951_d_b5 * s.v[847]) + (eq0_e951 * s.db[847][5]));
        let eq0_e953_d_b6: f64 = ((eq0_e951_d_b6 * s.v[847]) + (eq0_e951 * s.db[847][6]));
        let eq0_e953_d_b7: f64 = ((eq0_e951_d_b7 * s.v[847]) + (eq0_e951 * s.db[847][7]));
        let eq0_e953_d_b8: f64 = ((eq0_e951_d_b8 * s.v[847]) + (eq0_e951 * s.db[847][8]));
        let eq0_e953_d_b9: f64 = ((eq0_e951_d_b9 * s.v[847]) + (eq0_e951 * s.db[847][9]));
        let eq0_e953_d_b10: f64 = ((eq0_e951_d_b10 * s.v[847]) + (eq0_e951 * s.db[847][10]));
        let eq0_e953_d_b11: f64 = ((eq0_e951_d_b11 * s.v[847]) + (eq0_e951 * s.db[847][11]));
        let eq0_e953_d_b12: f64 = ((eq0_e951_d_b12 * s.v[847]) + (eq0_e951 * s.db[847][12]));
        let eq0_e953_d_b13: f64 = ((eq0_e951_d_b13 * s.v[847]) + (eq0_e951 * s.db[847][13]));
        let eq0_e953_d_b14: f64 = ((eq0_e951_d_b14 * s.v[847]) + (eq0_e951 * s.db[847][14]));
        let eq0_e953_d_b15: f64 = ((eq0_e951_d_b15 * s.v[847]) + (eq0_e951 * s.db[847][15]));
        let eq0_e953_d_b16: f64 = ((eq0_e951_d_b16 * s.v[847]) + (eq0_e951 * s.db[847][16]));
        let eq0_e953_d_b17: f64 = ((eq0_e951_d_b17 * s.v[847]) + (eq0_e951 * s.db[847][17]));
        let eq0_e953_d_b18: f64 = ((eq0_e951_d_b18 * s.v[847]) + (eq0_e951 * s.db[847][18]));
        let eq0_e953_d_b19: f64 = ((eq0_e951_d_b19 * s.v[847]) + (eq0_e951 * s.db[847][19]));
        let eq0_e953_d_b20: f64 = ((eq0_e951_d_b20 * s.v[847]) + (eq0_e951 * s.db[847][20]));
        let eq0_e953_d_b21: f64 = ((eq0_e951_d_b21 * s.v[847]) + (eq0_e951 * s.db[847][21]));
        let eq0_e953_d_b22: f64 = ((eq0_e951_d_b22 * s.v[847]) + (eq0_e951 * s.db[847][22]));
        let eq0_e953_d_b23: f64 = ((eq0_e951_d_b23 * s.v[847]) + (eq0_e951 * s.db[847][23]));
        let eq0_e953_d_b24: f64 = ((eq0_e951_d_b24 * s.v[847]) + (eq0_e951 * s.db[847][24]));
        (eq0_e953, eq0_e953_d_n0, eq0_e953_d_n1, eq0_e953_d_n2, eq0_e953_d_n3, eq0_e953_d_n4, eq0_e953_d_n5, eq0_e953_d_n6, eq0_e953_d_n7, eq0_e953_d_n8, eq0_e953_d_n9, eq0_e953_d_n10, eq0_e953_d_n11, eq0_e953_d_n12, eq0_e953_d_n13, eq0_e953_d_n14, eq0_e953_d_n15, eq0_e953_d_n16, eq0_e953_d_n17, eq0_e953_d_n18, eq0_e953_d_n19, eq0_e953_d_n20, eq0_e953_d_b0, eq0_e953_d_b1, eq0_e953_d_b2, eq0_e953_d_b3, eq0_e953_d_b4, eq0_e953_d_b5, eq0_e953_d_b6, eq0_e953_d_b7, eq0_e953_d_b8, eq0_e953_d_b9, eq0_e953_d_b10, eq0_e953_d_b11, eq0_e953_d_b12, eq0_e953_d_b13, eq0_e953_d_b14, eq0_e953_d_b15, eq0_e953_d_b16, eq0_e953_d_b17, eq0_e953_d_b18, eq0_e953_d_b19, eq0_e953_d_b20, eq0_e953_d_b21, eq0_e953_d_b22, eq0_e953_d_b23, eq0_e953_d_b24,)
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
        let eq1_e959: f64 = (s.v[0] * s.v[19]);
        let eq1_e959_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq1_e959_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq1_e959_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq1_e959_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq1_e959_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq1_e959_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq1_e959_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq1_e959_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq1_e959_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq1_e959_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq1_e959_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq1_e959_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq1_e959_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq1_e959_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq1_e959_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq1_e959_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq1_e959_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq1_e959_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq1_e959_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq1_e959_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq1_e959_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq1_e959_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq1_e959_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq1_e959_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq1_e959_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq1_e959_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq1_e959_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq1_e959_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq1_e959_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq1_e959_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq1_e959_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq1_e959_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq1_e959_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq1_e959_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq1_e959_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq1_e959_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq1_e959_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq1_e959_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq1_e959_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq1_e959_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq1_e959_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq1_e959_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq1_e959_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq1_e959_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq1_e959_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq1_e959_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq1_e961: f64 = (eq1_e959 * p.p32);
        let eq1_e961_d_n0: f64 = (eq1_e959_d_n0 * p.p32);
        let eq1_e961_d_n1: f64 = (eq1_e959_d_n1 * p.p32);
        let eq1_e961_d_n2: f64 = (eq1_e959_d_n2 * p.p32);
        let eq1_e961_d_n3: f64 = (eq1_e959_d_n3 * p.p32);
        let eq1_e961_d_n4: f64 = (eq1_e959_d_n4 * p.p32);
        let eq1_e961_d_n5: f64 = (eq1_e959_d_n5 * p.p32);
        let eq1_e961_d_n6: f64 = (eq1_e959_d_n6 * p.p32);
        let eq1_e961_d_n7: f64 = (eq1_e959_d_n7 * p.p32);
        let eq1_e961_d_n8: f64 = (eq1_e959_d_n8 * p.p32);
        let eq1_e961_d_n9: f64 = (eq1_e959_d_n9 * p.p32);
        let eq1_e961_d_n10: f64 = (eq1_e959_d_n10 * p.p32);
        let eq1_e961_d_n11: f64 = (eq1_e959_d_n11 * p.p32);
        let eq1_e961_d_n12: f64 = (eq1_e959_d_n12 * p.p32);
        let eq1_e961_d_n13: f64 = (eq1_e959_d_n13 * p.p32);
        let eq1_e961_d_n14: f64 = (eq1_e959_d_n14 * p.p32);
        let eq1_e961_d_n15: f64 = (eq1_e959_d_n15 * p.p32);
        let eq1_e961_d_n16: f64 = (eq1_e959_d_n16 * p.p32);
        let eq1_e961_d_n17: f64 = (eq1_e959_d_n17 * p.p32);
        let eq1_e961_d_n18: f64 = (eq1_e959_d_n18 * p.p32);
        let eq1_e961_d_n19: f64 = (eq1_e959_d_n19 * p.p32);
        let eq1_e961_d_n20: f64 = (eq1_e959_d_n20 * p.p32);
        let eq1_e961_d_b0: f64 = (eq1_e959_d_b0 * p.p32);
        let eq1_e961_d_b1: f64 = (eq1_e959_d_b1 * p.p32);
        let eq1_e961_d_b2: f64 = (eq1_e959_d_b2 * p.p32);
        let eq1_e961_d_b3: f64 = (eq1_e959_d_b3 * p.p32);
        let eq1_e961_d_b4: f64 = (eq1_e959_d_b4 * p.p32);
        let eq1_e961_d_b5: f64 = (eq1_e959_d_b5 * p.p32);
        let eq1_e961_d_b6: f64 = (eq1_e959_d_b6 * p.p32);
        let eq1_e961_d_b7: f64 = (eq1_e959_d_b7 * p.p32);
        let eq1_e961_d_b8: f64 = (eq1_e959_d_b8 * p.p32);
        let eq1_e961_d_b9: f64 = (eq1_e959_d_b9 * p.p32);
        let eq1_e961_d_b10: f64 = (eq1_e959_d_b10 * p.p32);
        let eq1_e961_d_b11: f64 = (eq1_e959_d_b11 * p.p32);
        let eq1_e961_d_b12: f64 = (eq1_e959_d_b12 * p.p32);
        let eq1_e961_d_b13: f64 = (eq1_e959_d_b13 * p.p32);
        let eq1_e961_d_b14: f64 = (eq1_e959_d_b14 * p.p32);
        let eq1_e961_d_b15: f64 = (eq1_e959_d_b15 * p.p32);
        let eq1_e961_d_b16: f64 = (eq1_e959_d_b16 * p.p32);
        let eq1_e961_d_b17: f64 = (eq1_e959_d_b17 * p.p32);
        let eq1_e961_d_b18: f64 = (eq1_e959_d_b18 * p.p32);
        let eq1_e961_d_b19: f64 = (eq1_e959_d_b19 * p.p32);
        let eq1_e961_d_b20: f64 = (eq1_e959_d_b20 * p.p32);
        let eq1_e961_d_b21: f64 = (eq1_e959_d_b21 * p.p32);
        let eq1_e961_d_b22: f64 = (eq1_e959_d_b22 * p.p32);
        let eq1_e961_d_b23: f64 = (eq1_e959_d_b23 * p.p32);
        let eq1_e961_d_b24: f64 = (eq1_e959_d_b24 * p.p32);
        let eq1_e964: f64 = (s.v[838] + s.v[846]);
        let eq1_e964_d_n0: f64 = (s.dn[838][0] + s.dn[846][0]);
        let eq1_e964_d_n1: f64 = (s.dn[838][1] + s.dn[846][1]);
        let eq1_e964_d_n2: f64 = (s.dn[838][2] + s.dn[846][2]);
        let eq1_e964_d_n3: f64 = (s.dn[838][3] + s.dn[846][3]);
        let eq1_e964_d_n4: f64 = (s.dn[838][4] + s.dn[846][4]);
        let eq1_e964_d_n5: f64 = (s.dn[838][5] + s.dn[846][5]);
        let eq1_e964_d_n6: f64 = (s.dn[838][6] + s.dn[846][6]);
        let eq1_e964_d_n7: f64 = (s.dn[838][7] + s.dn[846][7]);
        let eq1_e964_d_n8: f64 = (s.dn[838][8] + s.dn[846][8]);
        let eq1_e964_d_n9: f64 = (s.dn[838][9] + s.dn[846][9]);
        let eq1_e964_d_n10: f64 = (s.dn[838][10] + s.dn[846][10]);
        let eq1_e964_d_n11: f64 = (s.dn[838][11] + s.dn[846][11]);
        let eq1_e964_d_n12: f64 = (s.dn[838][12] + s.dn[846][12]);
        let eq1_e964_d_n13: f64 = (s.dn[838][13] + s.dn[846][13]);
        let eq1_e964_d_n14: f64 = (s.dn[838][14] + s.dn[846][14]);
        let eq1_e964_d_n15: f64 = (s.dn[838][15] + s.dn[846][15]);
        let eq1_e964_d_n16: f64 = (s.dn[838][16] + s.dn[846][16]);
        let eq1_e964_d_n17: f64 = (s.dn[838][17] + s.dn[846][17]);
        let eq1_e964_d_n18: f64 = (s.dn[838][18] + s.dn[846][18]);
        let eq1_e964_d_n19: f64 = (s.dn[838][19] + s.dn[846][19]);
        let eq1_e964_d_n20: f64 = (s.dn[838][20] + s.dn[846][20]);
        let eq1_e964_d_b0: f64 = (s.db[838][0] + s.db[846][0]);
        let eq1_e964_d_b1: f64 = (s.db[838][1] + s.db[846][1]);
        let eq1_e964_d_b2: f64 = (s.db[838][2] + s.db[846][2]);
        let eq1_e964_d_b3: f64 = (s.db[838][3] + s.db[846][3]);
        let eq1_e964_d_b4: f64 = (s.db[838][4] + s.db[846][4]);
        let eq1_e964_d_b5: f64 = (s.db[838][5] + s.db[846][5]);
        let eq1_e964_d_b6: f64 = (s.db[838][6] + s.db[846][6]);
        let eq1_e964_d_b7: f64 = (s.db[838][7] + s.db[846][7]);
        let eq1_e964_d_b8: f64 = (s.db[838][8] + s.db[846][8]);
        let eq1_e964_d_b9: f64 = (s.db[838][9] + s.db[846][9]);
        let eq1_e964_d_b10: f64 = (s.db[838][10] + s.db[846][10]);
        let eq1_e964_d_b11: f64 = (s.db[838][11] + s.db[846][11]);
        let eq1_e964_d_b12: f64 = (s.db[838][12] + s.db[846][12]);
        let eq1_e964_d_b13: f64 = (s.db[838][13] + s.db[846][13]);
        let eq1_e964_d_b14: f64 = (s.db[838][14] + s.db[846][14]);
        let eq1_e964_d_b15: f64 = (s.db[838][15] + s.db[846][15]);
        let eq1_e964_d_b16: f64 = (s.db[838][16] + s.db[846][16]);
        let eq1_e964_d_b17: f64 = (s.db[838][17] + s.db[846][17]);
        let eq1_e964_d_b18: f64 = (s.db[838][18] + s.db[846][18]);
        let eq1_e964_d_b19: f64 = (s.db[838][19] + s.db[846][19]);
        let eq1_e964_d_b20: f64 = (s.db[838][20] + s.db[846][20]);
        let eq1_e964_d_b21: f64 = (s.db[838][21] + s.db[846][21]);
        let eq1_e964_d_b22: f64 = (s.db[838][22] + s.db[846][22]);
        let eq1_e964_d_b23: f64 = (s.db[838][23] + s.db[846][23]);
        let eq1_e964_d_b24: f64 = (s.db[838][24] + s.db[846][24]);
        let eq1_e965: f64 = (eq1_e961 * eq1_e964);
        let eq1_e965_d_n0: f64 = ((eq1_e961_d_n0 * eq1_e964) + (eq1_e961 * eq1_e964_d_n0));
        let eq1_e965_d_n1: f64 = ((eq1_e961_d_n1 * eq1_e964) + (eq1_e961 * eq1_e964_d_n1));
        let eq1_e965_d_n2: f64 = ((eq1_e961_d_n2 * eq1_e964) + (eq1_e961 * eq1_e964_d_n2));
        let eq1_e965_d_n3: f64 = ((eq1_e961_d_n3 * eq1_e964) + (eq1_e961 * eq1_e964_d_n3));
        let eq1_e965_d_n4: f64 = ((eq1_e961_d_n4 * eq1_e964) + (eq1_e961 * eq1_e964_d_n4));
        let eq1_e965_d_n5: f64 = ((eq1_e961_d_n5 * eq1_e964) + (eq1_e961 * eq1_e964_d_n5));
        let eq1_e965_d_n6: f64 = ((eq1_e961_d_n6 * eq1_e964) + (eq1_e961 * eq1_e964_d_n6));
        let eq1_e965_d_n7: f64 = ((eq1_e961_d_n7 * eq1_e964) + (eq1_e961 * eq1_e964_d_n7));
        let eq1_e965_d_n8: f64 = ((eq1_e961_d_n8 * eq1_e964) + (eq1_e961 * eq1_e964_d_n8));
        let eq1_e965_d_n9: f64 = ((eq1_e961_d_n9 * eq1_e964) + (eq1_e961 * eq1_e964_d_n9));
        let eq1_e965_d_n10: f64 = ((eq1_e961_d_n10 * eq1_e964) + (eq1_e961 * eq1_e964_d_n10));
        let eq1_e965_d_n11: f64 = ((eq1_e961_d_n11 * eq1_e964) + (eq1_e961 * eq1_e964_d_n11));
        let eq1_e965_d_n12: f64 = ((eq1_e961_d_n12 * eq1_e964) + (eq1_e961 * eq1_e964_d_n12));
        let eq1_e965_d_n13: f64 = ((eq1_e961_d_n13 * eq1_e964) + (eq1_e961 * eq1_e964_d_n13));
        let eq1_e965_d_n14: f64 = ((eq1_e961_d_n14 * eq1_e964) + (eq1_e961 * eq1_e964_d_n14));
        let eq1_e965_d_n15: f64 = ((eq1_e961_d_n15 * eq1_e964) + (eq1_e961 * eq1_e964_d_n15));
        let eq1_e965_d_n16: f64 = ((eq1_e961_d_n16 * eq1_e964) + (eq1_e961 * eq1_e964_d_n16));
        let eq1_e965_d_n17: f64 = ((eq1_e961_d_n17 * eq1_e964) + (eq1_e961 * eq1_e964_d_n17));
        let eq1_e965_d_n18: f64 = ((eq1_e961_d_n18 * eq1_e964) + (eq1_e961 * eq1_e964_d_n18));
        let eq1_e965_d_n19: f64 = ((eq1_e961_d_n19 * eq1_e964) + (eq1_e961 * eq1_e964_d_n19));
        let eq1_e965_d_n20: f64 = ((eq1_e961_d_n20 * eq1_e964) + (eq1_e961 * eq1_e964_d_n20));
        let eq1_e965_d_b0: f64 = ((eq1_e961_d_b0 * eq1_e964) + (eq1_e961 * eq1_e964_d_b0));
        let eq1_e965_d_b1: f64 = ((eq1_e961_d_b1 * eq1_e964) + (eq1_e961 * eq1_e964_d_b1));
        let eq1_e965_d_b2: f64 = ((eq1_e961_d_b2 * eq1_e964) + (eq1_e961 * eq1_e964_d_b2));
        let eq1_e965_d_b3: f64 = ((eq1_e961_d_b3 * eq1_e964) + (eq1_e961 * eq1_e964_d_b3));
        let eq1_e965_d_b4: f64 = ((eq1_e961_d_b4 * eq1_e964) + (eq1_e961 * eq1_e964_d_b4));
        let eq1_e965_d_b5: f64 = ((eq1_e961_d_b5 * eq1_e964) + (eq1_e961 * eq1_e964_d_b5));
        let eq1_e965_d_b6: f64 = ((eq1_e961_d_b6 * eq1_e964) + (eq1_e961 * eq1_e964_d_b6));
        let eq1_e965_d_b7: f64 = ((eq1_e961_d_b7 * eq1_e964) + (eq1_e961 * eq1_e964_d_b7));
        let eq1_e965_d_b8: f64 = ((eq1_e961_d_b8 * eq1_e964) + (eq1_e961 * eq1_e964_d_b8));
        let eq1_e965_d_b9: f64 = ((eq1_e961_d_b9 * eq1_e964) + (eq1_e961 * eq1_e964_d_b9));
        let eq1_e965_d_b10: f64 = ((eq1_e961_d_b10 * eq1_e964) + (eq1_e961 * eq1_e964_d_b10));
        let eq1_e965_d_b11: f64 = ((eq1_e961_d_b11 * eq1_e964) + (eq1_e961 * eq1_e964_d_b11));
        let eq1_e965_d_b12: f64 = ((eq1_e961_d_b12 * eq1_e964) + (eq1_e961 * eq1_e964_d_b12));
        let eq1_e965_d_b13: f64 = ((eq1_e961_d_b13 * eq1_e964) + (eq1_e961 * eq1_e964_d_b13));
        let eq1_e965_d_b14: f64 = ((eq1_e961_d_b14 * eq1_e964) + (eq1_e961 * eq1_e964_d_b14));
        let eq1_e965_d_b15: f64 = ((eq1_e961_d_b15 * eq1_e964) + (eq1_e961 * eq1_e964_d_b15));
        let eq1_e965_d_b16: f64 = ((eq1_e961_d_b16 * eq1_e964) + (eq1_e961 * eq1_e964_d_b16));
        let eq1_e965_d_b17: f64 = ((eq1_e961_d_b17 * eq1_e964) + (eq1_e961 * eq1_e964_d_b17));
        let eq1_e965_d_b18: f64 = ((eq1_e961_d_b18 * eq1_e964) + (eq1_e961 * eq1_e964_d_b18));
        let eq1_e965_d_b19: f64 = ((eq1_e961_d_b19 * eq1_e964) + (eq1_e961 * eq1_e964_d_b19));
        let eq1_e965_d_b20: f64 = ((eq1_e961_d_b20 * eq1_e964) + (eq1_e961 * eq1_e964_d_b20));
        let eq1_e965_d_b21: f64 = ((eq1_e961_d_b21 * eq1_e964) + (eq1_e961 * eq1_e964_d_b21));
        let eq1_e965_d_b22: f64 = ((eq1_e961_d_b22 * eq1_e964) + (eq1_e961 * eq1_e964_d_b22));
        let eq1_e965_d_b23: f64 = ((eq1_e961_d_b23 * eq1_e964) + (eq1_e961 * eq1_e964_d_b23));
        let eq1_e965_d_b24: f64 = ((eq1_e961_d_b24 * eq1_e964) + (eq1_e961 * eq1_e964_d_b24));
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
    }

    pub(super) fn stamp_transient_equations_block_1(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let (eq2_e977, eq2_e977_d_n0, eq2_e977_d_n1, eq2_e977_d_n2, eq2_e977_d_n3, eq2_e977_d_n4, eq2_e977_d_n5, eq2_e977_d_n6, eq2_e977_d_n7, eq2_e977_d_n8, eq2_e977_d_n9, eq2_e977_d_n10, eq2_e977_d_n11, eq2_e977_d_n12, eq2_e977_d_n13, eq2_e977_d_n14, eq2_e977_d_n15, eq2_e977_d_n16, eq2_e977_d_n17, eq2_e977_d_n18, eq2_e977_d_n19, eq2_e977_d_n20, eq2_e977_d_b0, eq2_e977_d_b1, eq2_e977_d_b2, eq2_e977_d_b3, eq2_e977_d_b4, eq2_e977_d_b5, eq2_e977_d_b6, eq2_e977_d_b7, eq2_e977_d_b8, eq2_e977_d_b9, eq2_e977_d_b10, eq2_e977_d_b11, eq2_e977_d_b12, eq2_e977_d_b13, eq2_e977_d_b14, eq2_e977_d_b15, eq2_e977_d_b16, eq2_e977_d_b17, eq2_e977_d_b18, eq2_e977_d_b19, eq2_e977_d_b20, eq2_e977_d_b21, eq2_e977_d_b22, eq2_e977_d_b23, eq2_e977_d_b24,) = {
    if s.b[2913] {
        let eq2_e971: f64 = (s.v[0] * s.v[19]);
        let eq2_e971_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq2_e971_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq2_e971_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq2_e971_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq2_e971_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq2_e971_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq2_e971_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq2_e971_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq2_e971_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq2_e971_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq2_e971_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq2_e971_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq2_e971_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq2_e971_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq2_e971_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq2_e971_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq2_e971_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq2_e971_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq2_e971_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq2_e971_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq2_e971_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq2_e971_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq2_e971_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq2_e971_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq2_e971_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq2_e971_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq2_e971_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq2_e971_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq2_e971_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq2_e971_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq2_e971_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq2_e971_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq2_e971_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq2_e971_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq2_e971_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq2_e971_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq2_e971_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq2_e971_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq2_e971_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq2_e971_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq2_e971_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq2_e971_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq2_e971_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq2_e971_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq2_e971_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq2_e971_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq2_e973: f64 = (eq2_e971 * p.p32);
        let eq2_e973_d_n0: f64 = (eq2_e971_d_n0 * p.p32);
        let eq2_e973_d_n1: f64 = (eq2_e971_d_n1 * p.p32);
        let eq2_e973_d_n2: f64 = (eq2_e971_d_n2 * p.p32);
        let eq2_e973_d_n3: f64 = (eq2_e971_d_n3 * p.p32);
        let eq2_e973_d_n4: f64 = (eq2_e971_d_n4 * p.p32);
        let eq2_e973_d_n5: f64 = (eq2_e971_d_n5 * p.p32);
        let eq2_e973_d_n6: f64 = (eq2_e971_d_n6 * p.p32);
        let eq2_e973_d_n7: f64 = (eq2_e971_d_n7 * p.p32);
        let eq2_e973_d_n8: f64 = (eq2_e971_d_n8 * p.p32);
        let eq2_e973_d_n9: f64 = (eq2_e971_d_n9 * p.p32);
        let eq2_e973_d_n10: f64 = (eq2_e971_d_n10 * p.p32);
        let eq2_e973_d_n11: f64 = (eq2_e971_d_n11 * p.p32);
        let eq2_e973_d_n12: f64 = (eq2_e971_d_n12 * p.p32);
        let eq2_e973_d_n13: f64 = (eq2_e971_d_n13 * p.p32);
        let eq2_e973_d_n14: f64 = (eq2_e971_d_n14 * p.p32);
        let eq2_e973_d_n15: f64 = (eq2_e971_d_n15 * p.p32);
        let eq2_e973_d_n16: f64 = (eq2_e971_d_n16 * p.p32);
        let eq2_e973_d_n17: f64 = (eq2_e971_d_n17 * p.p32);
        let eq2_e973_d_n18: f64 = (eq2_e971_d_n18 * p.p32);
        let eq2_e973_d_n19: f64 = (eq2_e971_d_n19 * p.p32);
        let eq2_e973_d_n20: f64 = (eq2_e971_d_n20 * p.p32);
        let eq2_e973_d_b0: f64 = (eq2_e971_d_b0 * p.p32);
        let eq2_e973_d_b1: f64 = (eq2_e971_d_b1 * p.p32);
        let eq2_e973_d_b2: f64 = (eq2_e971_d_b2 * p.p32);
        let eq2_e973_d_b3: f64 = (eq2_e971_d_b3 * p.p32);
        let eq2_e973_d_b4: f64 = (eq2_e971_d_b4 * p.p32);
        let eq2_e973_d_b5: f64 = (eq2_e971_d_b5 * p.p32);
        let eq2_e973_d_b6: f64 = (eq2_e971_d_b6 * p.p32);
        let eq2_e973_d_b7: f64 = (eq2_e971_d_b7 * p.p32);
        let eq2_e973_d_b8: f64 = (eq2_e971_d_b8 * p.p32);
        let eq2_e973_d_b9: f64 = (eq2_e971_d_b9 * p.p32);
        let eq2_e973_d_b10: f64 = (eq2_e971_d_b10 * p.p32);
        let eq2_e973_d_b11: f64 = (eq2_e971_d_b11 * p.p32);
        let eq2_e973_d_b12: f64 = (eq2_e971_d_b12 * p.p32);
        let eq2_e973_d_b13: f64 = (eq2_e971_d_b13 * p.p32);
        let eq2_e973_d_b14: f64 = (eq2_e971_d_b14 * p.p32);
        let eq2_e973_d_b15: f64 = (eq2_e971_d_b15 * p.p32);
        let eq2_e973_d_b16: f64 = (eq2_e971_d_b16 * p.p32);
        let eq2_e973_d_b17: f64 = (eq2_e971_d_b17 * p.p32);
        let eq2_e973_d_b18: f64 = (eq2_e971_d_b18 * p.p32);
        let eq2_e973_d_b19: f64 = (eq2_e971_d_b19 * p.p32);
        let eq2_e973_d_b20: f64 = (eq2_e971_d_b20 * p.p32);
        let eq2_e973_d_b21: f64 = (eq2_e971_d_b21 * p.p32);
        let eq2_e973_d_b22: f64 = (eq2_e971_d_b22 * p.p32);
        let eq2_e973_d_b23: f64 = (eq2_e971_d_b23 * p.p32);
        let eq2_e973_d_b24: f64 = (eq2_e971_d_b24 * p.p32);
        let eq2_e975: f64 = (eq2_e973 * s.v[841]);
        let eq2_e975_d_n0: f64 = ((eq2_e973_d_n0 * s.v[841]) + (eq2_e973 * s.dn[841][0]));
        let eq2_e975_d_n1: f64 = ((eq2_e973_d_n1 * s.v[841]) + (eq2_e973 * s.dn[841][1]));
        let eq2_e975_d_n2: f64 = ((eq2_e973_d_n2 * s.v[841]) + (eq2_e973 * s.dn[841][2]));
        let eq2_e975_d_n3: f64 = ((eq2_e973_d_n3 * s.v[841]) + (eq2_e973 * s.dn[841][3]));
        let eq2_e975_d_n4: f64 = ((eq2_e973_d_n4 * s.v[841]) + (eq2_e973 * s.dn[841][4]));
        let eq2_e975_d_n5: f64 = ((eq2_e973_d_n5 * s.v[841]) + (eq2_e973 * s.dn[841][5]));
        let eq2_e975_d_n6: f64 = ((eq2_e973_d_n6 * s.v[841]) + (eq2_e973 * s.dn[841][6]));
        let eq2_e975_d_n7: f64 = ((eq2_e973_d_n7 * s.v[841]) + (eq2_e973 * s.dn[841][7]));
        let eq2_e975_d_n8: f64 = ((eq2_e973_d_n8 * s.v[841]) + (eq2_e973 * s.dn[841][8]));
        let eq2_e975_d_n9: f64 = ((eq2_e973_d_n9 * s.v[841]) + (eq2_e973 * s.dn[841][9]));
        let eq2_e975_d_n10: f64 = ((eq2_e973_d_n10 * s.v[841]) + (eq2_e973 * s.dn[841][10]));
        let eq2_e975_d_n11: f64 = ((eq2_e973_d_n11 * s.v[841]) + (eq2_e973 * s.dn[841][11]));
        let eq2_e975_d_n12: f64 = ((eq2_e973_d_n12 * s.v[841]) + (eq2_e973 * s.dn[841][12]));
        let eq2_e975_d_n13: f64 = ((eq2_e973_d_n13 * s.v[841]) + (eq2_e973 * s.dn[841][13]));
        let eq2_e975_d_n14: f64 = ((eq2_e973_d_n14 * s.v[841]) + (eq2_e973 * s.dn[841][14]));
        let eq2_e975_d_n15: f64 = ((eq2_e973_d_n15 * s.v[841]) + (eq2_e973 * s.dn[841][15]));
        let eq2_e975_d_n16: f64 = ((eq2_e973_d_n16 * s.v[841]) + (eq2_e973 * s.dn[841][16]));
        let eq2_e975_d_n17: f64 = ((eq2_e973_d_n17 * s.v[841]) + (eq2_e973 * s.dn[841][17]));
        let eq2_e975_d_n18: f64 = ((eq2_e973_d_n18 * s.v[841]) + (eq2_e973 * s.dn[841][18]));
        let eq2_e975_d_n19: f64 = ((eq2_e973_d_n19 * s.v[841]) + (eq2_e973 * s.dn[841][19]));
        let eq2_e975_d_n20: f64 = ((eq2_e973_d_n20 * s.v[841]) + (eq2_e973 * s.dn[841][20]));
        let eq2_e975_d_b0: f64 = ((eq2_e973_d_b0 * s.v[841]) + (eq2_e973 * s.db[841][0]));
        let eq2_e975_d_b1: f64 = ((eq2_e973_d_b1 * s.v[841]) + (eq2_e973 * s.db[841][1]));
        let eq2_e975_d_b2: f64 = ((eq2_e973_d_b2 * s.v[841]) + (eq2_e973 * s.db[841][2]));
        let eq2_e975_d_b3: f64 = ((eq2_e973_d_b3 * s.v[841]) + (eq2_e973 * s.db[841][3]));
        let eq2_e975_d_b4: f64 = ((eq2_e973_d_b4 * s.v[841]) + (eq2_e973 * s.db[841][4]));
        let eq2_e975_d_b5: f64 = ((eq2_e973_d_b5 * s.v[841]) + (eq2_e973 * s.db[841][5]));
        let eq2_e975_d_b6: f64 = ((eq2_e973_d_b6 * s.v[841]) + (eq2_e973 * s.db[841][6]));
        let eq2_e975_d_b7: f64 = ((eq2_e973_d_b7 * s.v[841]) + (eq2_e973 * s.db[841][7]));
        let eq2_e975_d_b8: f64 = ((eq2_e973_d_b8 * s.v[841]) + (eq2_e973 * s.db[841][8]));
        let eq2_e975_d_b9: f64 = ((eq2_e973_d_b9 * s.v[841]) + (eq2_e973 * s.db[841][9]));
        let eq2_e975_d_b10: f64 = ((eq2_e973_d_b10 * s.v[841]) + (eq2_e973 * s.db[841][10]));
        let eq2_e975_d_b11: f64 = ((eq2_e973_d_b11 * s.v[841]) + (eq2_e973 * s.db[841][11]));
        let eq2_e975_d_b12: f64 = ((eq2_e973_d_b12 * s.v[841]) + (eq2_e973 * s.db[841][12]));
        let eq2_e975_d_b13: f64 = ((eq2_e973_d_b13 * s.v[841]) + (eq2_e973 * s.db[841][13]));
        let eq2_e975_d_b14: f64 = ((eq2_e973_d_b14 * s.v[841]) + (eq2_e973 * s.db[841][14]));
        let eq2_e975_d_b15: f64 = ((eq2_e973_d_b15 * s.v[841]) + (eq2_e973 * s.db[841][15]));
        let eq2_e975_d_b16: f64 = ((eq2_e973_d_b16 * s.v[841]) + (eq2_e973 * s.db[841][16]));
        let eq2_e975_d_b17: f64 = ((eq2_e973_d_b17 * s.v[841]) + (eq2_e973 * s.db[841][17]));
        let eq2_e975_d_b18: f64 = ((eq2_e973_d_b18 * s.v[841]) + (eq2_e973 * s.db[841][18]));
        let eq2_e975_d_b19: f64 = ((eq2_e973_d_b19 * s.v[841]) + (eq2_e973 * s.db[841][19]));
        let eq2_e975_d_b20: f64 = ((eq2_e973_d_b20 * s.v[841]) + (eq2_e973 * s.db[841][20]));
        let eq2_e975_d_b21: f64 = ((eq2_e973_d_b21 * s.v[841]) + (eq2_e973 * s.db[841][21]));
        let eq2_e975_d_b22: f64 = ((eq2_e973_d_b22 * s.v[841]) + (eq2_e973 * s.db[841][22]));
        let eq2_e975_d_b23: f64 = ((eq2_e973_d_b23 * s.v[841]) + (eq2_e973 * s.db[841][23]));
        let eq2_e975_d_b24: f64 = ((eq2_e973_d_b24 * s.v[841]) + (eq2_e973 * s.db[841][24]));
        (eq2_e975, eq2_e975_d_n0, eq2_e975_d_n1, eq2_e975_d_n2, eq2_e975_d_n3, eq2_e975_d_n4, eq2_e975_d_n5, eq2_e975_d_n6, eq2_e975_d_n7, eq2_e975_d_n8, eq2_e975_d_n9, eq2_e975_d_n10, eq2_e975_d_n11, eq2_e975_d_n12, eq2_e975_d_n13, eq2_e975_d_n14, eq2_e975_d_n15, eq2_e975_d_n16, eq2_e975_d_n17, eq2_e975_d_n18, eq2_e975_d_n19, eq2_e975_d_n20, eq2_e975_d_b0, eq2_e975_d_b1, eq2_e975_d_b2, eq2_e975_d_b3, eq2_e975_d_b4, eq2_e975_d_b5, eq2_e975_d_b6, eq2_e975_d_b7, eq2_e975_d_b8, eq2_e975_d_b9, eq2_e975_d_b10, eq2_e975_d_b11, eq2_e975_d_b12, eq2_e975_d_b13, eq2_e975_d_b14, eq2_e975_d_b15, eq2_e975_d_b16, eq2_e975_d_b17, eq2_e975_d_b18, eq2_e975_d_b19, eq2_e975_d_b20, eq2_e975_d_b21, eq2_e975_d_b22, eq2_e975_d_b23, eq2_e975_d_b24,)
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
        let eq3_e981: f64 = (s.v[0] * s.v[19]);
        let eq3_e981_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq3_e981_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq3_e981_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq3_e981_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq3_e981_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq3_e981_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq3_e981_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq3_e981_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq3_e981_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq3_e981_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq3_e981_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq3_e981_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq3_e981_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq3_e981_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq3_e981_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq3_e981_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq3_e981_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq3_e981_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq3_e981_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq3_e981_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq3_e981_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq3_e981_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq3_e981_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq3_e981_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq3_e981_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq3_e981_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq3_e981_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq3_e981_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq3_e981_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq3_e981_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq3_e981_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq3_e981_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq3_e981_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq3_e981_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq3_e981_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq3_e981_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq3_e981_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq3_e981_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq3_e981_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq3_e981_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq3_e981_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq3_e981_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq3_e981_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq3_e981_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq3_e981_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq3_e981_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq3_e983: f64 = (eq3_e981 * p.p32);
        let eq3_e983_d_n0: f64 = (eq3_e981_d_n0 * p.p32);
        let eq3_e983_d_n1: f64 = (eq3_e981_d_n1 * p.p32);
        let eq3_e983_d_n2: f64 = (eq3_e981_d_n2 * p.p32);
        let eq3_e983_d_n3: f64 = (eq3_e981_d_n3 * p.p32);
        let eq3_e983_d_n4: f64 = (eq3_e981_d_n4 * p.p32);
        let eq3_e983_d_n5: f64 = (eq3_e981_d_n5 * p.p32);
        let eq3_e983_d_n6: f64 = (eq3_e981_d_n6 * p.p32);
        let eq3_e983_d_n7: f64 = (eq3_e981_d_n7 * p.p32);
        let eq3_e983_d_n8: f64 = (eq3_e981_d_n8 * p.p32);
        let eq3_e983_d_n9: f64 = (eq3_e981_d_n9 * p.p32);
        let eq3_e983_d_n10: f64 = (eq3_e981_d_n10 * p.p32);
        let eq3_e983_d_n11: f64 = (eq3_e981_d_n11 * p.p32);
        let eq3_e983_d_n12: f64 = (eq3_e981_d_n12 * p.p32);
        let eq3_e983_d_n13: f64 = (eq3_e981_d_n13 * p.p32);
        let eq3_e983_d_n14: f64 = (eq3_e981_d_n14 * p.p32);
        let eq3_e983_d_n15: f64 = (eq3_e981_d_n15 * p.p32);
        let eq3_e983_d_n16: f64 = (eq3_e981_d_n16 * p.p32);
        let eq3_e983_d_n17: f64 = (eq3_e981_d_n17 * p.p32);
        let eq3_e983_d_n18: f64 = (eq3_e981_d_n18 * p.p32);
        let eq3_e983_d_n19: f64 = (eq3_e981_d_n19 * p.p32);
        let eq3_e983_d_n20: f64 = (eq3_e981_d_n20 * p.p32);
        let eq3_e983_d_b0: f64 = (eq3_e981_d_b0 * p.p32);
        let eq3_e983_d_b1: f64 = (eq3_e981_d_b1 * p.p32);
        let eq3_e983_d_b2: f64 = (eq3_e981_d_b2 * p.p32);
        let eq3_e983_d_b3: f64 = (eq3_e981_d_b3 * p.p32);
        let eq3_e983_d_b4: f64 = (eq3_e981_d_b4 * p.p32);
        let eq3_e983_d_b5: f64 = (eq3_e981_d_b5 * p.p32);
        let eq3_e983_d_b6: f64 = (eq3_e981_d_b6 * p.p32);
        let eq3_e983_d_b7: f64 = (eq3_e981_d_b7 * p.p32);
        let eq3_e983_d_b8: f64 = (eq3_e981_d_b8 * p.p32);
        let eq3_e983_d_b9: f64 = (eq3_e981_d_b9 * p.p32);
        let eq3_e983_d_b10: f64 = (eq3_e981_d_b10 * p.p32);
        let eq3_e983_d_b11: f64 = (eq3_e981_d_b11 * p.p32);
        let eq3_e983_d_b12: f64 = (eq3_e981_d_b12 * p.p32);
        let eq3_e983_d_b13: f64 = (eq3_e981_d_b13 * p.p32);
        let eq3_e983_d_b14: f64 = (eq3_e981_d_b14 * p.p32);
        let eq3_e983_d_b15: f64 = (eq3_e981_d_b15 * p.p32);
        let eq3_e983_d_b16: f64 = (eq3_e981_d_b16 * p.p32);
        let eq3_e983_d_b17: f64 = (eq3_e981_d_b17 * p.p32);
        let eq3_e983_d_b18: f64 = (eq3_e981_d_b18 * p.p32);
        let eq3_e983_d_b19: f64 = (eq3_e981_d_b19 * p.p32);
        let eq3_e983_d_b20: f64 = (eq3_e981_d_b20 * p.p32);
        let eq3_e983_d_b21: f64 = (eq3_e981_d_b21 * p.p32);
        let eq3_e983_d_b22: f64 = (eq3_e981_d_b22 * p.p32);
        let eq3_e983_d_b23: f64 = (eq3_e981_d_b23 * p.p32);
        let eq3_e983_d_b24: f64 = (eq3_e981_d_b24 * p.p32);
        let eq3_e985: f64 = (eq3_e983 * s.v[842]);
        let eq3_e985_d_n0: f64 = ((eq3_e983_d_n0 * s.v[842]) + (eq3_e983 * s.dn[842][0]));
        let eq3_e985_d_n1: f64 = ((eq3_e983_d_n1 * s.v[842]) + (eq3_e983 * s.dn[842][1]));
        let eq3_e985_d_n2: f64 = ((eq3_e983_d_n2 * s.v[842]) + (eq3_e983 * s.dn[842][2]));
        let eq3_e985_d_n3: f64 = ((eq3_e983_d_n3 * s.v[842]) + (eq3_e983 * s.dn[842][3]));
        let eq3_e985_d_n4: f64 = ((eq3_e983_d_n4 * s.v[842]) + (eq3_e983 * s.dn[842][4]));
        let eq3_e985_d_n5: f64 = ((eq3_e983_d_n5 * s.v[842]) + (eq3_e983 * s.dn[842][5]));
        let eq3_e985_d_n6: f64 = ((eq3_e983_d_n6 * s.v[842]) + (eq3_e983 * s.dn[842][6]));
        let eq3_e985_d_n7: f64 = ((eq3_e983_d_n7 * s.v[842]) + (eq3_e983 * s.dn[842][7]));
        let eq3_e985_d_n8: f64 = ((eq3_e983_d_n8 * s.v[842]) + (eq3_e983 * s.dn[842][8]));
        let eq3_e985_d_n9: f64 = ((eq3_e983_d_n9 * s.v[842]) + (eq3_e983 * s.dn[842][9]));
        let eq3_e985_d_n10: f64 = ((eq3_e983_d_n10 * s.v[842]) + (eq3_e983 * s.dn[842][10]));
        let eq3_e985_d_n11: f64 = ((eq3_e983_d_n11 * s.v[842]) + (eq3_e983 * s.dn[842][11]));
        let eq3_e985_d_n12: f64 = ((eq3_e983_d_n12 * s.v[842]) + (eq3_e983 * s.dn[842][12]));
        let eq3_e985_d_n13: f64 = ((eq3_e983_d_n13 * s.v[842]) + (eq3_e983 * s.dn[842][13]));
        let eq3_e985_d_n14: f64 = ((eq3_e983_d_n14 * s.v[842]) + (eq3_e983 * s.dn[842][14]));
        let eq3_e985_d_n15: f64 = ((eq3_e983_d_n15 * s.v[842]) + (eq3_e983 * s.dn[842][15]));
        let eq3_e985_d_n16: f64 = ((eq3_e983_d_n16 * s.v[842]) + (eq3_e983 * s.dn[842][16]));
        let eq3_e985_d_n17: f64 = ((eq3_e983_d_n17 * s.v[842]) + (eq3_e983 * s.dn[842][17]));
        let eq3_e985_d_n18: f64 = ((eq3_e983_d_n18 * s.v[842]) + (eq3_e983 * s.dn[842][18]));
        let eq3_e985_d_n19: f64 = ((eq3_e983_d_n19 * s.v[842]) + (eq3_e983 * s.dn[842][19]));
        let eq3_e985_d_n20: f64 = ((eq3_e983_d_n20 * s.v[842]) + (eq3_e983 * s.dn[842][20]));
        let eq3_e985_d_b0: f64 = ((eq3_e983_d_b0 * s.v[842]) + (eq3_e983 * s.db[842][0]));
        let eq3_e985_d_b1: f64 = ((eq3_e983_d_b1 * s.v[842]) + (eq3_e983 * s.db[842][1]));
        let eq3_e985_d_b2: f64 = ((eq3_e983_d_b2 * s.v[842]) + (eq3_e983 * s.db[842][2]));
        let eq3_e985_d_b3: f64 = ((eq3_e983_d_b3 * s.v[842]) + (eq3_e983 * s.db[842][3]));
        let eq3_e985_d_b4: f64 = ((eq3_e983_d_b4 * s.v[842]) + (eq3_e983 * s.db[842][4]));
        let eq3_e985_d_b5: f64 = ((eq3_e983_d_b5 * s.v[842]) + (eq3_e983 * s.db[842][5]));
        let eq3_e985_d_b6: f64 = ((eq3_e983_d_b6 * s.v[842]) + (eq3_e983 * s.db[842][6]));
        let eq3_e985_d_b7: f64 = ((eq3_e983_d_b7 * s.v[842]) + (eq3_e983 * s.db[842][7]));
        let eq3_e985_d_b8: f64 = ((eq3_e983_d_b8 * s.v[842]) + (eq3_e983 * s.db[842][8]));
        let eq3_e985_d_b9: f64 = ((eq3_e983_d_b9 * s.v[842]) + (eq3_e983 * s.db[842][9]));
        let eq3_e985_d_b10: f64 = ((eq3_e983_d_b10 * s.v[842]) + (eq3_e983 * s.db[842][10]));
        let eq3_e985_d_b11: f64 = ((eq3_e983_d_b11 * s.v[842]) + (eq3_e983 * s.db[842][11]));
        let eq3_e985_d_b12: f64 = ((eq3_e983_d_b12 * s.v[842]) + (eq3_e983 * s.db[842][12]));
        let eq3_e985_d_b13: f64 = ((eq3_e983_d_b13 * s.v[842]) + (eq3_e983 * s.db[842][13]));
        let eq3_e985_d_b14: f64 = ((eq3_e983_d_b14 * s.v[842]) + (eq3_e983 * s.db[842][14]));
        let eq3_e985_d_b15: f64 = ((eq3_e983_d_b15 * s.v[842]) + (eq3_e983 * s.db[842][15]));
        let eq3_e985_d_b16: f64 = ((eq3_e983_d_b16 * s.v[842]) + (eq3_e983 * s.db[842][16]));
        let eq3_e985_d_b17: f64 = ((eq3_e983_d_b17 * s.v[842]) + (eq3_e983 * s.db[842][17]));
        let eq3_e985_d_b18: f64 = ((eq3_e983_d_b18 * s.v[842]) + (eq3_e983 * s.db[842][18]));
        let eq3_e985_d_b19: f64 = ((eq3_e983_d_b19 * s.v[842]) + (eq3_e983 * s.db[842][19]));
        let eq3_e985_d_b20: f64 = ((eq3_e983_d_b20 * s.v[842]) + (eq3_e983 * s.db[842][20]));
        let eq3_e985_d_b21: f64 = ((eq3_e983_d_b21 * s.v[842]) + (eq3_e983 * s.db[842][21]));
        let eq3_e985_d_b22: f64 = ((eq3_e983_d_b22 * s.v[842]) + (eq3_e983 * s.db[842][22]));
        let eq3_e985_d_b23: f64 = ((eq3_e983_d_b23 * s.v[842]) + (eq3_e983 * s.db[842][23]));
        let eq3_e985_d_b24: f64 = ((eq3_e983_d_b24 * s.v[842]) + (eq3_e983 * s.db[842][24]));
        (eq3_e985, eq3_e985_d_n0, eq3_e985_d_n1, eq3_e985_d_n2, eq3_e985_d_n3, eq3_e985_d_n4, eq3_e985_d_n5, eq3_e985_d_n6, eq3_e985_d_n7, eq3_e985_d_n8, eq3_e985_d_n9, eq3_e985_d_n10, eq3_e985_d_n11, eq3_e985_d_n12, eq3_e985_d_n13, eq3_e985_d_n14, eq3_e985_d_n15, eq3_e985_d_n16, eq3_e985_d_n17, eq3_e985_d_n18, eq3_e985_d_n19, eq3_e985_d_n20, eq3_e985_d_b0, eq3_e985_d_b1, eq3_e985_d_b2, eq3_e985_d_b3, eq3_e985_d_b4, eq3_e985_d_b5, eq3_e985_d_b6, eq3_e985_d_b7, eq3_e985_d_b8, eq3_e985_d_b9, eq3_e985_d_b10, eq3_e985_d_b11, eq3_e985_d_b12, eq3_e985_d_b13, eq3_e985_d_b14, eq3_e985_d_b15, eq3_e985_d_b16, eq3_e985_d_b17, eq3_e985_d_b18, eq3_e985_d_b19, eq3_e985_d_b20, eq3_e985_d_b21, eq3_e985_d_b22, eq3_e985_d_b23, eq3_e985_d_b24,)
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
        let eq4_e992: f64 = (s.v[0] * s.v[19]);
        let eq4_e992_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq4_e992_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq4_e992_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq4_e992_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq4_e992_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq4_e992_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq4_e992_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq4_e992_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq4_e992_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq4_e992_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq4_e992_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq4_e992_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq4_e992_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq4_e992_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq4_e992_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq4_e992_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq4_e992_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq4_e992_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq4_e992_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq4_e992_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq4_e992_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq4_e992_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq4_e992_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq4_e992_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq4_e992_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq4_e992_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq4_e992_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq4_e992_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq4_e992_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq4_e992_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq4_e992_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq4_e992_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq4_e992_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq4_e992_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq4_e992_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq4_e992_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq4_e992_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq4_e992_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq4_e992_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq4_e992_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq4_e992_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq4_e992_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq4_e992_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq4_e992_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq4_e992_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq4_e992_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq4_e994: f64 = (eq4_e992 * p.p32);
        let eq4_e994_d_n0: f64 = (eq4_e992_d_n0 * p.p32);
        let eq4_e994_d_n1: f64 = (eq4_e992_d_n1 * p.p32);
        let eq4_e994_d_n2: f64 = (eq4_e992_d_n2 * p.p32);
        let eq4_e994_d_n3: f64 = (eq4_e992_d_n3 * p.p32);
        let eq4_e994_d_n4: f64 = (eq4_e992_d_n4 * p.p32);
        let eq4_e994_d_n5: f64 = (eq4_e992_d_n5 * p.p32);
        let eq4_e994_d_n6: f64 = (eq4_e992_d_n6 * p.p32);
        let eq4_e994_d_n7: f64 = (eq4_e992_d_n7 * p.p32);
        let eq4_e994_d_n8: f64 = (eq4_e992_d_n8 * p.p32);
        let eq4_e994_d_n9: f64 = (eq4_e992_d_n9 * p.p32);
        let eq4_e994_d_n10: f64 = (eq4_e992_d_n10 * p.p32);
        let eq4_e994_d_n11: f64 = (eq4_e992_d_n11 * p.p32);
        let eq4_e994_d_n12: f64 = (eq4_e992_d_n12 * p.p32);
        let eq4_e994_d_n13: f64 = (eq4_e992_d_n13 * p.p32);
        let eq4_e994_d_n14: f64 = (eq4_e992_d_n14 * p.p32);
        let eq4_e994_d_n15: f64 = (eq4_e992_d_n15 * p.p32);
        let eq4_e994_d_n16: f64 = (eq4_e992_d_n16 * p.p32);
        let eq4_e994_d_n17: f64 = (eq4_e992_d_n17 * p.p32);
        let eq4_e994_d_n18: f64 = (eq4_e992_d_n18 * p.p32);
        let eq4_e994_d_n19: f64 = (eq4_e992_d_n19 * p.p32);
        let eq4_e994_d_n20: f64 = (eq4_e992_d_n20 * p.p32);
        let eq4_e994_d_b0: f64 = (eq4_e992_d_b0 * p.p32);
        let eq4_e994_d_b1: f64 = (eq4_e992_d_b1 * p.p32);
        let eq4_e994_d_b2: f64 = (eq4_e992_d_b2 * p.p32);
        let eq4_e994_d_b3: f64 = (eq4_e992_d_b3 * p.p32);
        let eq4_e994_d_b4: f64 = (eq4_e992_d_b4 * p.p32);
        let eq4_e994_d_b5: f64 = (eq4_e992_d_b5 * p.p32);
        let eq4_e994_d_b6: f64 = (eq4_e992_d_b6 * p.p32);
        let eq4_e994_d_b7: f64 = (eq4_e992_d_b7 * p.p32);
        let eq4_e994_d_b8: f64 = (eq4_e992_d_b8 * p.p32);
        let eq4_e994_d_b9: f64 = (eq4_e992_d_b9 * p.p32);
        let eq4_e994_d_b10: f64 = (eq4_e992_d_b10 * p.p32);
        let eq4_e994_d_b11: f64 = (eq4_e992_d_b11 * p.p32);
        let eq4_e994_d_b12: f64 = (eq4_e992_d_b12 * p.p32);
        let eq4_e994_d_b13: f64 = (eq4_e992_d_b13 * p.p32);
        let eq4_e994_d_b14: f64 = (eq4_e992_d_b14 * p.p32);
        let eq4_e994_d_b15: f64 = (eq4_e992_d_b15 * p.p32);
        let eq4_e994_d_b16: f64 = (eq4_e992_d_b16 * p.p32);
        let eq4_e994_d_b17: f64 = (eq4_e992_d_b17 * p.p32);
        let eq4_e994_d_b18: f64 = (eq4_e992_d_b18 * p.p32);
        let eq4_e994_d_b19: f64 = (eq4_e992_d_b19 * p.p32);
        let eq4_e994_d_b20: f64 = (eq4_e992_d_b20 * p.p32);
        let eq4_e994_d_b21: f64 = (eq4_e992_d_b21 * p.p32);
        let eq4_e994_d_b22: f64 = (eq4_e992_d_b22 * p.p32);
        let eq4_e994_d_b23: f64 = (eq4_e992_d_b23 * p.p32);
        let eq4_e994_d_b24: f64 = (eq4_e992_d_b24 * p.p32);
        let eq4_e996: f64 = (eq4_e994 * s.v[847]);
        let eq4_e996_d_n0: f64 = ((eq4_e994_d_n0 * s.v[847]) + (eq4_e994 * s.dn[847][0]));
        let eq4_e996_d_n1: f64 = ((eq4_e994_d_n1 * s.v[847]) + (eq4_e994 * s.dn[847][1]));
        let eq4_e996_d_n2: f64 = ((eq4_e994_d_n2 * s.v[847]) + (eq4_e994 * s.dn[847][2]));
        let eq4_e996_d_n3: f64 = ((eq4_e994_d_n3 * s.v[847]) + (eq4_e994 * s.dn[847][3]));
        let eq4_e996_d_n4: f64 = ((eq4_e994_d_n4 * s.v[847]) + (eq4_e994 * s.dn[847][4]));
        let eq4_e996_d_n5: f64 = ((eq4_e994_d_n5 * s.v[847]) + (eq4_e994 * s.dn[847][5]));
        let eq4_e996_d_n6: f64 = ((eq4_e994_d_n6 * s.v[847]) + (eq4_e994 * s.dn[847][6]));
        let eq4_e996_d_n7: f64 = ((eq4_e994_d_n7 * s.v[847]) + (eq4_e994 * s.dn[847][7]));
        let eq4_e996_d_n8: f64 = ((eq4_e994_d_n8 * s.v[847]) + (eq4_e994 * s.dn[847][8]));
        let eq4_e996_d_n9: f64 = ((eq4_e994_d_n9 * s.v[847]) + (eq4_e994 * s.dn[847][9]));
        let eq4_e996_d_n10: f64 = ((eq4_e994_d_n10 * s.v[847]) + (eq4_e994 * s.dn[847][10]));
        let eq4_e996_d_n11: f64 = ((eq4_e994_d_n11 * s.v[847]) + (eq4_e994 * s.dn[847][11]));
        let eq4_e996_d_n12: f64 = ((eq4_e994_d_n12 * s.v[847]) + (eq4_e994 * s.dn[847][12]));
        let eq4_e996_d_n13: f64 = ((eq4_e994_d_n13 * s.v[847]) + (eq4_e994 * s.dn[847][13]));
        let eq4_e996_d_n14: f64 = ((eq4_e994_d_n14 * s.v[847]) + (eq4_e994 * s.dn[847][14]));
        let eq4_e996_d_n15: f64 = ((eq4_e994_d_n15 * s.v[847]) + (eq4_e994 * s.dn[847][15]));
        let eq4_e996_d_n16: f64 = ((eq4_e994_d_n16 * s.v[847]) + (eq4_e994 * s.dn[847][16]));
        let eq4_e996_d_n17: f64 = ((eq4_e994_d_n17 * s.v[847]) + (eq4_e994 * s.dn[847][17]));
        let eq4_e996_d_n18: f64 = ((eq4_e994_d_n18 * s.v[847]) + (eq4_e994 * s.dn[847][18]));
        let eq4_e996_d_n19: f64 = ((eq4_e994_d_n19 * s.v[847]) + (eq4_e994 * s.dn[847][19]));
        let eq4_e996_d_n20: f64 = ((eq4_e994_d_n20 * s.v[847]) + (eq4_e994 * s.dn[847][20]));
        let eq4_e996_d_b0: f64 = ((eq4_e994_d_b0 * s.v[847]) + (eq4_e994 * s.db[847][0]));
        let eq4_e996_d_b1: f64 = ((eq4_e994_d_b1 * s.v[847]) + (eq4_e994 * s.db[847][1]));
        let eq4_e996_d_b2: f64 = ((eq4_e994_d_b2 * s.v[847]) + (eq4_e994 * s.db[847][2]));
        let eq4_e996_d_b3: f64 = ((eq4_e994_d_b3 * s.v[847]) + (eq4_e994 * s.db[847][3]));
        let eq4_e996_d_b4: f64 = ((eq4_e994_d_b4 * s.v[847]) + (eq4_e994 * s.db[847][4]));
        let eq4_e996_d_b5: f64 = ((eq4_e994_d_b5 * s.v[847]) + (eq4_e994 * s.db[847][5]));
        let eq4_e996_d_b6: f64 = ((eq4_e994_d_b6 * s.v[847]) + (eq4_e994 * s.db[847][6]));
        let eq4_e996_d_b7: f64 = ((eq4_e994_d_b7 * s.v[847]) + (eq4_e994 * s.db[847][7]));
        let eq4_e996_d_b8: f64 = ((eq4_e994_d_b8 * s.v[847]) + (eq4_e994 * s.db[847][8]));
        let eq4_e996_d_b9: f64 = ((eq4_e994_d_b9 * s.v[847]) + (eq4_e994 * s.db[847][9]));
        let eq4_e996_d_b10: f64 = ((eq4_e994_d_b10 * s.v[847]) + (eq4_e994 * s.db[847][10]));
        let eq4_e996_d_b11: f64 = ((eq4_e994_d_b11 * s.v[847]) + (eq4_e994 * s.db[847][11]));
        let eq4_e996_d_b12: f64 = ((eq4_e994_d_b12 * s.v[847]) + (eq4_e994 * s.db[847][12]));
        let eq4_e996_d_b13: f64 = ((eq4_e994_d_b13 * s.v[847]) + (eq4_e994 * s.db[847][13]));
        let eq4_e996_d_b14: f64 = ((eq4_e994_d_b14 * s.v[847]) + (eq4_e994 * s.db[847][14]));
        let eq4_e996_d_b15: f64 = ((eq4_e994_d_b15 * s.v[847]) + (eq4_e994 * s.db[847][15]));
        let eq4_e996_d_b16: f64 = ((eq4_e994_d_b16 * s.v[847]) + (eq4_e994 * s.db[847][16]));
        let eq4_e996_d_b17: f64 = ((eq4_e994_d_b17 * s.v[847]) + (eq4_e994 * s.db[847][17]));
        let eq4_e996_d_b18: f64 = ((eq4_e994_d_b18 * s.v[847]) + (eq4_e994 * s.db[847][18]));
        let eq4_e996_d_b19: f64 = ((eq4_e994_d_b19 * s.v[847]) + (eq4_e994 * s.db[847][19]));
        let eq4_e996_d_b20: f64 = ((eq4_e994_d_b20 * s.v[847]) + (eq4_e994 * s.db[847][20]));
        let eq4_e996_d_b21: f64 = ((eq4_e994_d_b21 * s.v[847]) + (eq4_e994 * s.db[847][21]));
        let eq4_e996_d_b22: f64 = ((eq4_e994_d_b22 * s.v[847]) + (eq4_e994 * s.db[847][22]));
        let eq4_e996_d_b23: f64 = ((eq4_e994_d_b23 * s.v[847]) + (eq4_e994 * s.db[847][23]));
        let eq4_e996_d_b24: f64 = ((eq4_e994_d_b24 * s.v[847]) + (eq4_e994 * s.db[847][24]));
        (eq4_e996, eq4_e996_d_n0, eq4_e996_d_n1, eq4_e996_d_n2, eq4_e996_d_n3, eq4_e996_d_n4, eq4_e996_d_n5, eq4_e996_d_n6, eq4_e996_d_n7, eq4_e996_d_n8, eq4_e996_d_n9, eq4_e996_d_n10, eq4_e996_d_n11, eq4_e996_d_n12, eq4_e996_d_n13, eq4_e996_d_n14, eq4_e996_d_n15, eq4_e996_d_n16, eq4_e996_d_n17, eq4_e996_d_n18, eq4_e996_d_n19, eq4_e996_d_n20, eq4_e996_d_b0, eq4_e996_d_b1, eq4_e996_d_b2, eq4_e996_d_b3, eq4_e996_d_b4, eq4_e996_d_b5, eq4_e996_d_b6, eq4_e996_d_b7, eq4_e996_d_b8, eq4_e996_d_b9, eq4_e996_d_b10, eq4_e996_d_b11, eq4_e996_d_b12, eq4_e996_d_b13, eq4_e996_d_b14, eq4_e996_d_b15, eq4_e996_d_b16, eq4_e996_d_b17, eq4_e996_d_b18, eq4_e996_d_b19, eq4_e996_d_b20, eq4_e996_d_b21, eq4_e996_d_b22, eq4_e996_d_b23, eq4_e996_d_b24,)
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
    }

    pub(super) fn stamp_transient_equations_block_2(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let (eq5_e1011, eq5_e1011_d_n0, eq5_e1011_d_n1, eq5_e1011_d_n2, eq5_e1011_d_n3, eq5_e1011_d_n4, eq5_e1011_d_n5, eq5_e1011_d_n6, eq5_e1011_d_n7, eq5_e1011_d_n8, eq5_e1011_d_n9, eq5_e1011_d_n10, eq5_e1011_d_n11, eq5_e1011_d_n12, eq5_e1011_d_n13, eq5_e1011_d_n14, eq5_e1011_d_n15, eq5_e1011_d_n16, eq5_e1011_d_n17, eq5_e1011_d_n18, eq5_e1011_d_n19, eq5_e1011_d_n20, eq5_e1011_d_b0, eq5_e1011_d_b1, eq5_e1011_d_b2, eq5_e1011_d_b3, eq5_e1011_d_b4, eq5_e1011_d_b5, eq5_e1011_d_b6, eq5_e1011_d_b7, eq5_e1011_d_b8, eq5_e1011_d_b9, eq5_e1011_d_b10, eq5_e1011_d_b11, eq5_e1011_d_b12, eq5_e1011_d_b13, eq5_e1011_d_b14, eq5_e1011_d_b15, eq5_e1011_d_b16, eq5_e1011_d_b17, eq5_e1011_d_b18, eq5_e1011_d_b19, eq5_e1011_d_b20, eq5_e1011_d_b21, eq5_e1011_d_b22, eq5_e1011_d_b23, eq5_e1011_d_b24,) = {
    if (!s.b[2913]) {
        let eq5_e1003: f64 = (s.v[0] * s.v[19]);
        let eq5_e1003_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq5_e1003_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq5_e1003_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq5_e1003_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq5_e1003_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq5_e1003_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq5_e1003_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq5_e1003_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq5_e1003_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq5_e1003_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq5_e1003_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq5_e1003_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq5_e1003_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq5_e1003_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq5_e1003_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq5_e1003_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq5_e1003_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq5_e1003_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq5_e1003_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq5_e1003_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq5_e1003_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq5_e1003_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq5_e1003_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq5_e1003_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq5_e1003_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq5_e1003_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq5_e1003_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq5_e1003_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq5_e1003_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq5_e1003_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq5_e1003_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq5_e1003_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq5_e1003_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq5_e1003_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq5_e1003_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq5_e1003_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq5_e1003_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq5_e1003_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq5_e1003_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq5_e1003_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq5_e1003_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq5_e1003_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq5_e1003_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq5_e1003_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq5_e1003_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq5_e1003_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq5_e1005: f64 = (eq5_e1003 * p.p32);
        let eq5_e1005_d_n0: f64 = (eq5_e1003_d_n0 * p.p32);
        let eq5_e1005_d_n1: f64 = (eq5_e1003_d_n1 * p.p32);
        let eq5_e1005_d_n2: f64 = (eq5_e1003_d_n2 * p.p32);
        let eq5_e1005_d_n3: f64 = (eq5_e1003_d_n3 * p.p32);
        let eq5_e1005_d_n4: f64 = (eq5_e1003_d_n4 * p.p32);
        let eq5_e1005_d_n5: f64 = (eq5_e1003_d_n5 * p.p32);
        let eq5_e1005_d_n6: f64 = (eq5_e1003_d_n6 * p.p32);
        let eq5_e1005_d_n7: f64 = (eq5_e1003_d_n7 * p.p32);
        let eq5_e1005_d_n8: f64 = (eq5_e1003_d_n8 * p.p32);
        let eq5_e1005_d_n9: f64 = (eq5_e1003_d_n9 * p.p32);
        let eq5_e1005_d_n10: f64 = (eq5_e1003_d_n10 * p.p32);
        let eq5_e1005_d_n11: f64 = (eq5_e1003_d_n11 * p.p32);
        let eq5_e1005_d_n12: f64 = (eq5_e1003_d_n12 * p.p32);
        let eq5_e1005_d_n13: f64 = (eq5_e1003_d_n13 * p.p32);
        let eq5_e1005_d_n14: f64 = (eq5_e1003_d_n14 * p.p32);
        let eq5_e1005_d_n15: f64 = (eq5_e1003_d_n15 * p.p32);
        let eq5_e1005_d_n16: f64 = (eq5_e1003_d_n16 * p.p32);
        let eq5_e1005_d_n17: f64 = (eq5_e1003_d_n17 * p.p32);
        let eq5_e1005_d_n18: f64 = (eq5_e1003_d_n18 * p.p32);
        let eq5_e1005_d_n19: f64 = (eq5_e1003_d_n19 * p.p32);
        let eq5_e1005_d_n20: f64 = (eq5_e1003_d_n20 * p.p32);
        let eq5_e1005_d_b0: f64 = (eq5_e1003_d_b0 * p.p32);
        let eq5_e1005_d_b1: f64 = (eq5_e1003_d_b1 * p.p32);
        let eq5_e1005_d_b2: f64 = (eq5_e1003_d_b2 * p.p32);
        let eq5_e1005_d_b3: f64 = (eq5_e1003_d_b3 * p.p32);
        let eq5_e1005_d_b4: f64 = (eq5_e1003_d_b4 * p.p32);
        let eq5_e1005_d_b5: f64 = (eq5_e1003_d_b5 * p.p32);
        let eq5_e1005_d_b6: f64 = (eq5_e1003_d_b6 * p.p32);
        let eq5_e1005_d_b7: f64 = (eq5_e1003_d_b7 * p.p32);
        let eq5_e1005_d_b8: f64 = (eq5_e1003_d_b8 * p.p32);
        let eq5_e1005_d_b9: f64 = (eq5_e1003_d_b9 * p.p32);
        let eq5_e1005_d_b10: f64 = (eq5_e1003_d_b10 * p.p32);
        let eq5_e1005_d_b11: f64 = (eq5_e1003_d_b11 * p.p32);
        let eq5_e1005_d_b12: f64 = (eq5_e1003_d_b12 * p.p32);
        let eq5_e1005_d_b13: f64 = (eq5_e1003_d_b13 * p.p32);
        let eq5_e1005_d_b14: f64 = (eq5_e1003_d_b14 * p.p32);
        let eq5_e1005_d_b15: f64 = (eq5_e1003_d_b15 * p.p32);
        let eq5_e1005_d_b16: f64 = (eq5_e1003_d_b16 * p.p32);
        let eq5_e1005_d_b17: f64 = (eq5_e1003_d_b17 * p.p32);
        let eq5_e1005_d_b18: f64 = (eq5_e1003_d_b18 * p.p32);
        let eq5_e1005_d_b19: f64 = (eq5_e1003_d_b19 * p.p32);
        let eq5_e1005_d_b20: f64 = (eq5_e1003_d_b20 * p.p32);
        let eq5_e1005_d_b21: f64 = (eq5_e1003_d_b21 * p.p32);
        let eq5_e1005_d_b22: f64 = (eq5_e1003_d_b22 * p.p32);
        let eq5_e1005_d_b23: f64 = (eq5_e1003_d_b23 * p.p32);
        let eq5_e1005_d_b24: f64 = (eq5_e1003_d_b24 * p.p32);
        let eq5_e1008: f64 = (s.v[838] + s.v[846]);
        let eq5_e1008_d_n0: f64 = (s.dn[838][0] + s.dn[846][0]);
        let eq5_e1008_d_n1: f64 = (s.dn[838][1] + s.dn[846][1]);
        let eq5_e1008_d_n2: f64 = (s.dn[838][2] + s.dn[846][2]);
        let eq5_e1008_d_n3: f64 = (s.dn[838][3] + s.dn[846][3]);
        let eq5_e1008_d_n4: f64 = (s.dn[838][4] + s.dn[846][4]);
        let eq5_e1008_d_n5: f64 = (s.dn[838][5] + s.dn[846][5]);
        let eq5_e1008_d_n6: f64 = (s.dn[838][6] + s.dn[846][6]);
        let eq5_e1008_d_n7: f64 = (s.dn[838][7] + s.dn[846][7]);
        let eq5_e1008_d_n8: f64 = (s.dn[838][8] + s.dn[846][8]);
        let eq5_e1008_d_n9: f64 = (s.dn[838][9] + s.dn[846][9]);
        let eq5_e1008_d_n10: f64 = (s.dn[838][10] + s.dn[846][10]);
        let eq5_e1008_d_n11: f64 = (s.dn[838][11] + s.dn[846][11]);
        let eq5_e1008_d_n12: f64 = (s.dn[838][12] + s.dn[846][12]);
        let eq5_e1008_d_n13: f64 = (s.dn[838][13] + s.dn[846][13]);
        let eq5_e1008_d_n14: f64 = (s.dn[838][14] + s.dn[846][14]);
        let eq5_e1008_d_n15: f64 = (s.dn[838][15] + s.dn[846][15]);
        let eq5_e1008_d_n16: f64 = (s.dn[838][16] + s.dn[846][16]);
        let eq5_e1008_d_n17: f64 = (s.dn[838][17] + s.dn[846][17]);
        let eq5_e1008_d_n18: f64 = (s.dn[838][18] + s.dn[846][18]);
        let eq5_e1008_d_n19: f64 = (s.dn[838][19] + s.dn[846][19]);
        let eq5_e1008_d_n20: f64 = (s.dn[838][20] + s.dn[846][20]);
        let eq5_e1008_d_b0: f64 = (s.db[838][0] + s.db[846][0]);
        let eq5_e1008_d_b1: f64 = (s.db[838][1] + s.db[846][1]);
        let eq5_e1008_d_b2: f64 = (s.db[838][2] + s.db[846][2]);
        let eq5_e1008_d_b3: f64 = (s.db[838][3] + s.db[846][3]);
        let eq5_e1008_d_b4: f64 = (s.db[838][4] + s.db[846][4]);
        let eq5_e1008_d_b5: f64 = (s.db[838][5] + s.db[846][5]);
        let eq5_e1008_d_b6: f64 = (s.db[838][6] + s.db[846][6]);
        let eq5_e1008_d_b7: f64 = (s.db[838][7] + s.db[846][7]);
        let eq5_e1008_d_b8: f64 = (s.db[838][8] + s.db[846][8]);
        let eq5_e1008_d_b9: f64 = (s.db[838][9] + s.db[846][9]);
        let eq5_e1008_d_b10: f64 = (s.db[838][10] + s.db[846][10]);
        let eq5_e1008_d_b11: f64 = (s.db[838][11] + s.db[846][11]);
        let eq5_e1008_d_b12: f64 = (s.db[838][12] + s.db[846][12]);
        let eq5_e1008_d_b13: f64 = (s.db[838][13] + s.db[846][13]);
        let eq5_e1008_d_b14: f64 = (s.db[838][14] + s.db[846][14]);
        let eq5_e1008_d_b15: f64 = (s.db[838][15] + s.db[846][15]);
        let eq5_e1008_d_b16: f64 = (s.db[838][16] + s.db[846][16]);
        let eq5_e1008_d_b17: f64 = (s.db[838][17] + s.db[846][17]);
        let eq5_e1008_d_b18: f64 = (s.db[838][18] + s.db[846][18]);
        let eq5_e1008_d_b19: f64 = (s.db[838][19] + s.db[846][19]);
        let eq5_e1008_d_b20: f64 = (s.db[838][20] + s.db[846][20]);
        let eq5_e1008_d_b21: f64 = (s.db[838][21] + s.db[846][21]);
        let eq5_e1008_d_b22: f64 = (s.db[838][22] + s.db[846][22]);
        let eq5_e1008_d_b23: f64 = (s.db[838][23] + s.db[846][23]);
        let eq5_e1008_d_b24: f64 = (s.db[838][24] + s.db[846][24]);
        let eq5_e1009: f64 = (eq5_e1005 * eq5_e1008);
        let eq5_e1009_d_n0: f64 = ((eq5_e1005_d_n0 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n0));
        let eq5_e1009_d_n1: f64 = ((eq5_e1005_d_n1 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n1));
        let eq5_e1009_d_n2: f64 = ((eq5_e1005_d_n2 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n2));
        let eq5_e1009_d_n3: f64 = ((eq5_e1005_d_n3 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n3));
        let eq5_e1009_d_n4: f64 = ((eq5_e1005_d_n4 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n4));
        let eq5_e1009_d_n5: f64 = ((eq5_e1005_d_n5 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n5));
        let eq5_e1009_d_n6: f64 = ((eq5_e1005_d_n6 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n6));
        let eq5_e1009_d_n7: f64 = ((eq5_e1005_d_n7 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n7));
        let eq5_e1009_d_n8: f64 = ((eq5_e1005_d_n8 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n8));
        let eq5_e1009_d_n9: f64 = ((eq5_e1005_d_n9 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n9));
        let eq5_e1009_d_n10: f64 = ((eq5_e1005_d_n10 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n10));
        let eq5_e1009_d_n11: f64 = ((eq5_e1005_d_n11 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n11));
        let eq5_e1009_d_n12: f64 = ((eq5_e1005_d_n12 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n12));
        let eq5_e1009_d_n13: f64 = ((eq5_e1005_d_n13 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n13));
        let eq5_e1009_d_n14: f64 = ((eq5_e1005_d_n14 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n14));
        let eq5_e1009_d_n15: f64 = ((eq5_e1005_d_n15 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n15));
        let eq5_e1009_d_n16: f64 = ((eq5_e1005_d_n16 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n16));
        let eq5_e1009_d_n17: f64 = ((eq5_e1005_d_n17 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n17));
        let eq5_e1009_d_n18: f64 = ((eq5_e1005_d_n18 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n18));
        let eq5_e1009_d_n19: f64 = ((eq5_e1005_d_n19 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n19));
        let eq5_e1009_d_n20: f64 = ((eq5_e1005_d_n20 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_n20));
        let eq5_e1009_d_b0: f64 = ((eq5_e1005_d_b0 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b0));
        let eq5_e1009_d_b1: f64 = ((eq5_e1005_d_b1 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b1));
        let eq5_e1009_d_b2: f64 = ((eq5_e1005_d_b2 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b2));
        let eq5_e1009_d_b3: f64 = ((eq5_e1005_d_b3 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b3));
        let eq5_e1009_d_b4: f64 = ((eq5_e1005_d_b4 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b4));
        let eq5_e1009_d_b5: f64 = ((eq5_e1005_d_b5 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b5));
        let eq5_e1009_d_b6: f64 = ((eq5_e1005_d_b6 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b6));
        let eq5_e1009_d_b7: f64 = ((eq5_e1005_d_b7 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b7));
        let eq5_e1009_d_b8: f64 = ((eq5_e1005_d_b8 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b8));
        let eq5_e1009_d_b9: f64 = ((eq5_e1005_d_b9 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b9));
        let eq5_e1009_d_b10: f64 = ((eq5_e1005_d_b10 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b10));
        let eq5_e1009_d_b11: f64 = ((eq5_e1005_d_b11 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b11));
        let eq5_e1009_d_b12: f64 = ((eq5_e1005_d_b12 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b12));
        let eq5_e1009_d_b13: f64 = ((eq5_e1005_d_b13 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b13));
        let eq5_e1009_d_b14: f64 = ((eq5_e1005_d_b14 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b14));
        let eq5_e1009_d_b15: f64 = ((eq5_e1005_d_b15 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b15));
        let eq5_e1009_d_b16: f64 = ((eq5_e1005_d_b16 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b16));
        let eq5_e1009_d_b17: f64 = ((eq5_e1005_d_b17 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b17));
        let eq5_e1009_d_b18: f64 = ((eq5_e1005_d_b18 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b18));
        let eq5_e1009_d_b19: f64 = ((eq5_e1005_d_b19 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b19));
        let eq5_e1009_d_b20: f64 = ((eq5_e1005_d_b20 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b20));
        let eq5_e1009_d_b21: f64 = ((eq5_e1005_d_b21 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b21));
        let eq5_e1009_d_b22: f64 = ((eq5_e1005_d_b22 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b22));
        let eq5_e1009_d_b23: f64 = ((eq5_e1005_d_b23 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b23));
        let eq5_e1009_d_b24: f64 = ((eq5_e1005_d_b24 * eq5_e1008) + (eq5_e1005 * eq5_e1008_d_b24));
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
        let eq6_e1016: f64 = (s.v[0] * s.v[19]);
        let eq6_e1016_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq6_e1016_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq6_e1016_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq6_e1016_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq6_e1016_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq6_e1016_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq6_e1016_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq6_e1016_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq6_e1016_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq6_e1016_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq6_e1016_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq6_e1016_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq6_e1016_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq6_e1016_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq6_e1016_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq6_e1016_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq6_e1016_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq6_e1016_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq6_e1016_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq6_e1016_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq6_e1016_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq6_e1016_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq6_e1016_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq6_e1016_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq6_e1016_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq6_e1016_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq6_e1016_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq6_e1016_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq6_e1016_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq6_e1016_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq6_e1016_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq6_e1016_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq6_e1016_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq6_e1016_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq6_e1016_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq6_e1016_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq6_e1016_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq6_e1016_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq6_e1016_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq6_e1016_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq6_e1016_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq6_e1016_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq6_e1016_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq6_e1016_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq6_e1016_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq6_e1016_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq6_e1018: f64 = (eq6_e1016 * p.p32);
        let eq6_e1018_d_n0: f64 = (eq6_e1016_d_n0 * p.p32);
        let eq6_e1018_d_n1: f64 = (eq6_e1016_d_n1 * p.p32);
        let eq6_e1018_d_n2: f64 = (eq6_e1016_d_n2 * p.p32);
        let eq6_e1018_d_n3: f64 = (eq6_e1016_d_n3 * p.p32);
        let eq6_e1018_d_n4: f64 = (eq6_e1016_d_n4 * p.p32);
        let eq6_e1018_d_n5: f64 = (eq6_e1016_d_n5 * p.p32);
        let eq6_e1018_d_n6: f64 = (eq6_e1016_d_n6 * p.p32);
        let eq6_e1018_d_n7: f64 = (eq6_e1016_d_n7 * p.p32);
        let eq6_e1018_d_n8: f64 = (eq6_e1016_d_n8 * p.p32);
        let eq6_e1018_d_n9: f64 = (eq6_e1016_d_n9 * p.p32);
        let eq6_e1018_d_n10: f64 = (eq6_e1016_d_n10 * p.p32);
        let eq6_e1018_d_n11: f64 = (eq6_e1016_d_n11 * p.p32);
        let eq6_e1018_d_n12: f64 = (eq6_e1016_d_n12 * p.p32);
        let eq6_e1018_d_n13: f64 = (eq6_e1016_d_n13 * p.p32);
        let eq6_e1018_d_n14: f64 = (eq6_e1016_d_n14 * p.p32);
        let eq6_e1018_d_n15: f64 = (eq6_e1016_d_n15 * p.p32);
        let eq6_e1018_d_n16: f64 = (eq6_e1016_d_n16 * p.p32);
        let eq6_e1018_d_n17: f64 = (eq6_e1016_d_n17 * p.p32);
        let eq6_e1018_d_n18: f64 = (eq6_e1016_d_n18 * p.p32);
        let eq6_e1018_d_n19: f64 = (eq6_e1016_d_n19 * p.p32);
        let eq6_e1018_d_n20: f64 = (eq6_e1016_d_n20 * p.p32);
        let eq6_e1018_d_b0: f64 = (eq6_e1016_d_b0 * p.p32);
        let eq6_e1018_d_b1: f64 = (eq6_e1016_d_b1 * p.p32);
        let eq6_e1018_d_b2: f64 = (eq6_e1016_d_b2 * p.p32);
        let eq6_e1018_d_b3: f64 = (eq6_e1016_d_b3 * p.p32);
        let eq6_e1018_d_b4: f64 = (eq6_e1016_d_b4 * p.p32);
        let eq6_e1018_d_b5: f64 = (eq6_e1016_d_b5 * p.p32);
        let eq6_e1018_d_b6: f64 = (eq6_e1016_d_b6 * p.p32);
        let eq6_e1018_d_b7: f64 = (eq6_e1016_d_b7 * p.p32);
        let eq6_e1018_d_b8: f64 = (eq6_e1016_d_b8 * p.p32);
        let eq6_e1018_d_b9: f64 = (eq6_e1016_d_b9 * p.p32);
        let eq6_e1018_d_b10: f64 = (eq6_e1016_d_b10 * p.p32);
        let eq6_e1018_d_b11: f64 = (eq6_e1016_d_b11 * p.p32);
        let eq6_e1018_d_b12: f64 = (eq6_e1016_d_b12 * p.p32);
        let eq6_e1018_d_b13: f64 = (eq6_e1016_d_b13 * p.p32);
        let eq6_e1018_d_b14: f64 = (eq6_e1016_d_b14 * p.p32);
        let eq6_e1018_d_b15: f64 = (eq6_e1016_d_b15 * p.p32);
        let eq6_e1018_d_b16: f64 = (eq6_e1016_d_b16 * p.p32);
        let eq6_e1018_d_b17: f64 = (eq6_e1016_d_b17 * p.p32);
        let eq6_e1018_d_b18: f64 = (eq6_e1016_d_b18 * p.p32);
        let eq6_e1018_d_b19: f64 = (eq6_e1016_d_b19 * p.p32);
        let eq6_e1018_d_b20: f64 = (eq6_e1016_d_b20 * p.p32);
        let eq6_e1018_d_b21: f64 = (eq6_e1016_d_b21 * p.p32);
        let eq6_e1018_d_b22: f64 = (eq6_e1016_d_b22 * p.p32);
        let eq6_e1018_d_b23: f64 = (eq6_e1016_d_b23 * p.p32);
        let eq6_e1018_d_b24: f64 = (eq6_e1016_d_b24 * p.p32);
        let eq6_e1020: f64 = (eq6_e1018 * s.v[841]);
        let eq6_e1020_d_n0: f64 = ((eq6_e1018_d_n0 * s.v[841]) + (eq6_e1018 * s.dn[841][0]));
        let eq6_e1020_d_n1: f64 = ((eq6_e1018_d_n1 * s.v[841]) + (eq6_e1018 * s.dn[841][1]));
        let eq6_e1020_d_n2: f64 = ((eq6_e1018_d_n2 * s.v[841]) + (eq6_e1018 * s.dn[841][2]));
        let eq6_e1020_d_n3: f64 = ((eq6_e1018_d_n3 * s.v[841]) + (eq6_e1018 * s.dn[841][3]));
        let eq6_e1020_d_n4: f64 = ((eq6_e1018_d_n4 * s.v[841]) + (eq6_e1018 * s.dn[841][4]));
        let eq6_e1020_d_n5: f64 = ((eq6_e1018_d_n5 * s.v[841]) + (eq6_e1018 * s.dn[841][5]));
        let eq6_e1020_d_n6: f64 = ((eq6_e1018_d_n6 * s.v[841]) + (eq6_e1018 * s.dn[841][6]));
        let eq6_e1020_d_n7: f64 = ((eq6_e1018_d_n7 * s.v[841]) + (eq6_e1018 * s.dn[841][7]));
        let eq6_e1020_d_n8: f64 = ((eq6_e1018_d_n8 * s.v[841]) + (eq6_e1018 * s.dn[841][8]));
        let eq6_e1020_d_n9: f64 = ((eq6_e1018_d_n9 * s.v[841]) + (eq6_e1018 * s.dn[841][9]));
        let eq6_e1020_d_n10: f64 = ((eq6_e1018_d_n10 * s.v[841]) + (eq6_e1018 * s.dn[841][10]));
        let eq6_e1020_d_n11: f64 = ((eq6_e1018_d_n11 * s.v[841]) + (eq6_e1018 * s.dn[841][11]));
        let eq6_e1020_d_n12: f64 = ((eq6_e1018_d_n12 * s.v[841]) + (eq6_e1018 * s.dn[841][12]));
        let eq6_e1020_d_n13: f64 = ((eq6_e1018_d_n13 * s.v[841]) + (eq6_e1018 * s.dn[841][13]));
        let eq6_e1020_d_n14: f64 = ((eq6_e1018_d_n14 * s.v[841]) + (eq6_e1018 * s.dn[841][14]));
        let eq6_e1020_d_n15: f64 = ((eq6_e1018_d_n15 * s.v[841]) + (eq6_e1018 * s.dn[841][15]));
        let eq6_e1020_d_n16: f64 = ((eq6_e1018_d_n16 * s.v[841]) + (eq6_e1018 * s.dn[841][16]));
        let eq6_e1020_d_n17: f64 = ((eq6_e1018_d_n17 * s.v[841]) + (eq6_e1018 * s.dn[841][17]));
        let eq6_e1020_d_n18: f64 = ((eq6_e1018_d_n18 * s.v[841]) + (eq6_e1018 * s.dn[841][18]));
        let eq6_e1020_d_n19: f64 = ((eq6_e1018_d_n19 * s.v[841]) + (eq6_e1018 * s.dn[841][19]));
        let eq6_e1020_d_n20: f64 = ((eq6_e1018_d_n20 * s.v[841]) + (eq6_e1018 * s.dn[841][20]));
        let eq6_e1020_d_b0: f64 = ((eq6_e1018_d_b0 * s.v[841]) + (eq6_e1018 * s.db[841][0]));
        let eq6_e1020_d_b1: f64 = ((eq6_e1018_d_b1 * s.v[841]) + (eq6_e1018 * s.db[841][1]));
        let eq6_e1020_d_b2: f64 = ((eq6_e1018_d_b2 * s.v[841]) + (eq6_e1018 * s.db[841][2]));
        let eq6_e1020_d_b3: f64 = ((eq6_e1018_d_b3 * s.v[841]) + (eq6_e1018 * s.db[841][3]));
        let eq6_e1020_d_b4: f64 = ((eq6_e1018_d_b4 * s.v[841]) + (eq6_e1018 * s.db[841][4]));
        let eq6_e1020_d_b5: f64 = ((eq6_e1018_d_b5 * s.v[841]) + (eq6_e1018 * s.db[841][5]));
        let eq6_e1020_d_b6: f64 = ((eq6_e1018_d_b6 * s.v[841]) + (eq6_e1018 * s.db[841][6]));
        let eq6_e1020_d_b7: f64 = ((eq6_e1018_d_b7 * s.v[841]) + (eq6_e1018 * s.db[841][7]));
        let eq6_e1020_d_b8: f64 = ((eq6_e1018_d_b8 * s.v[841]) + (eq6_e1018 * s.db[841][8]));
        let eq6_e1020_d_b9: f64 = ((eq6_e1018_d_b9 * s.v[841]) + (eq6_e1018 * s.db[841][9]));
        let eq6_e1020_d_b10: f64 = ((eq6_e1018_d_b10 * s.v[841]) + (eq6_e1018 * s.db[841][10]));
        let eq6_e1020_d_b11: f64 = ((eq6_e1018_d_b11 * s.v[841]) + (eq6_e1018 * s.db[841][11]));
        let eq6_e1020_d_b12: f64 = ((eq6_e1018_d_b12 * s.v[841]) + (eq6_e1018 * s.db[841][12]));
        let eq6_e1020_d_b13: f64 = ((eq6_e1018_d_b13 * s.v[841]) + (eq6_e1018 * s.db[841][13]));
        let eq6_e1020_d_b14: f64 = ((eq6_e1018_d_b14 * s.v[841]) + (eq6_e1018 * s.db[841][14]));
        let eq6_e1020_d_b15: f64 = ((eq6_e1018_d_b15 * s.v[841]) + (eq6_e1018 * s.db[841][15]));
        let eq6_e1020_d_b16: f64 = ((eq6_e1018_d_b16 * s.v[841]) + (eq6_e1018 * s.db[841][16]));
        let eq6_e1020_d_b17: f64 = ((eq6_e1018_d_b17 * s.v[841]) + (eq6_e1018 * s.db[841][17]));
        let eq6_e1020_d_b18: f64 = ((eq6_e1018_d_b18 * s.v[841]) + (eq6_e1018 * s.db[841][18]));
        let eq6_e1020_d_b19: f64 = ((eq6_e1018_d_b19 * s.v[841]) + (eq6_e1018 * s.db[841][19]));
        let eq6_e1020_d_b20: f64 = ((eq6_e1018_d_b20 * s.v[841]) + (eq6_e1018 * s.db[841][20]));
        let eq6_e1020_d_b21: f64 = ((eq6_e1018_d_b21 * s.v[841]) + (eq6_e1018 * s.db[841][21]));
        let eq6_e1020_d_b22: f64 = ((eq6_e1018_d_b22 * s.v[841]) + (eq6_e1018 * s.db[841][22]));
        let eq6_e1020_d_b23: f64 = ((eq6_e1018_d_b23 * s.v[841]) + (eq6_e1018 * s.db[841][23]));
        let eq6_e1020_d_b24: f64 = ((eq6_e1018_d_b24 * s.v[841]) + (eq6_e1018 * s.db[841][24]));
        (eq6_e1020, eq6_e1020_d_n0, eq6_e1020_d_n1, eq6_e1020_d_n2, eq6_e1020_d_n3, eq6_e1020_d_n4, eq6_e1020_d_n5, eq6_e1020_d_n6, eq6_e1020_d_n7, eq6_e1020_d_n8, eq6_e1020_d_n9, eq6_e1020_d_n10, eq6_e1020_d_n11, eq6_e1020_d_n12, eq6_e1020_d_n13, eq6_e1020_d_n14, eq6_e1020_d_n15, eq6_e1020_d_n16, eq6_e1020_d_n17, eq6_e1020_d_n18, eq6_e1020_d_n19, eq6_e1020_d_n20, eq6_e1020_d_b0, eq6_e1020_d_b1, eq6_e1020_d_b2, eq6_e1020_d_b3, eq6_e1020_d_b4, eq6_e1020_d_b5, eq6_e1020_d_b6, eq6_e1020_d_b7, eq6_e1020_d_b8, eq6_e1020_d_b9, eq6_e1020_d_b10, eq6_e1020_d_b11, eq6_e1020_d_b12, eq6_e1020_d_b13, eq6_e1020_d_b14, eq6_e1020_d_b15, eq6_e1020_d_b16, eq6_e1020_d_b17, eq6_e1020_d_b18, eq6_e1020_d_b19, eq6_e1020_d_b20, eq6_e1020_d_b21, eq6_e1020_d_b22, eq6_e1020_d_b23, eq6_e1020_d_b24,)
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
    }

    pub(super) fn stamp_transient_equations_block_3(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let (eq7_e1033, eq7_e1033_d_n0, eq7_e1033_d_n1, eq7_e1033_d_n2, eq7_e1033_d_n3, eq7_e1033_d_n4, eq7_e1033_d_n5, eq7_e1033_d_n6, eq7_e1033_d_n7, eq7_e1033_d_n8, eq7_e1033_d_n9, eq7_e1033_d_n10, eq7_e1033_d_n11, eq7_e1033_d_n12, eq7_e1033_d_n13, eq7_e1033_d_n14, eq7_e1033_d_n15, eq7_e1033_d_n16, eq7_e1033_d_n17, eq7_e1033_d_n18, eq7_e1033_d_n19, eq7_e1033_d_n20, eq7_e1033_d_b0, eq7_e1033_d_b1, eq7_e1033_d_b2, eq7_e1033_d_b3, eq7_e1033_d_b4, eq7_e1033_d_b5, eq7_e1033_d_b6, eq7_e1033_d_b7, eq7_e1033_d_b8, eq7_e1033_d_b9, eq7_e1033_d_b10, eq7_e1033_d_b11, eq7_e1033_d_b12, eq7_e1033_d_b13, eq7_e1033_d_b14, eq7_e1033_d_b15, eq7_e1033_d_b16, eq7_e1033_d_b17, eq7_e1033_d_b18, eq7_e1033_d_b19, eq7_e1033_d_b20, eq7_e1033_d_b21, eq7_e1033_d_b22, eq7_e1033_d_b23, eq7_e1033_d_b24,) = {
    if (!s.b[2913]) {
        let eq7_e1027: f64 = (s.v[0] * s.v[19]);
        let eq7_e1027_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq7_e1027_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq7_e1027_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq7_e1027_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq7_e1027_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq7_e1027_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq7_e1027_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq7_e1027_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq7_e1027_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq7_e1027_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq7_e1027_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq7_e1027_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq7_e1027_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq7_e1027_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq7_e1027_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq7_e1027_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq7_e1027_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq7_e1027_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq7_e1027_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq7_e1027_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq7_e1027_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq7_e1027_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq7_e1027_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq7_e1027_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq7_e1027_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq7_e1027_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq7_e1027_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq7_e1027_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq7_e1027_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq7_e1027_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq7_e1027_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq7_e1027_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq7_e1027_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq7_e1027_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq7_e1027_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq7_e1027_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq7_e1027_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq7_e1027_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq7_e1027_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq7_e1027_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq7_e1027_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq7_e1027_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq7_e1027_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq7_e1027_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq7_e1027_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq7_e1027_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq7_e1029: f64 = (eq7_e1027 * p.p32);
        let eq7_e1029_d_n0: f64 = (eq7_e1027_d_n0 * p.p32);
        let eq7_e1029_d_n1: f64 = (eq7_e1027_d_n1 * p.p32);
        let eq7_e1029_d_n2: f64 = (eq7_e1027_d_n2 * p.p32);
        let eq7_e1029_d_n3: f64 = (eq7_e1027_d_n3 * p.p32);
        let eq7_e1029_d_n4: f64 = (eq7_e1027_d_n4 * p.p32);
        let eq7_e1029_d_n5: f64 = (eq7_e1027_d_n5 * p.p32);
        let eq7_e1029_d_n6: f64 = (eq7_e1027_d_n6 * p.p32);
        let eq7_e1029_d_n7: f64 = (eq7_e1027_d_n7 * p.p32);
        let eq7_e1029_d_n8: f64 = (eq7_e1027_d_n8 * p.p32);
        let eq7_e1029_d_n9: f64 = (eq7_e1027_d_n9 * p.p32);
        let eq7_e1029_d_n10: f64 = (eq7_e1027_d_n10 * p.p32);
        let eq7_e1029_d_n11: f64 = (eq7_e1027_d_n11 * p.p32);
        let eq7_e1029_d_n12: f64 = (eq7_e1027_d_n12 * p.p32);
        let eq7_e1029_d_n13: f64 = (eq7_e1027_d_n13 * p.p32);
        let eq7_e1029_d_n14: f64 = (eq7_e1027_d_n14 * p.p32);
        let eq7_e1029_d_n15: f64 = (eq7_e1027_d_n15 * p.p32);
        let eq7_e1029_d_n16: f64 = (eq7_e1027_d_n16 * p.p32);
        let eq7_e1029_d_n17: f64 = (eq7_e1027_d_n17 * p.p32);
        let eq7_e1029_d_n18: f64 = (eq7_e1027_d_n18 * p.p32);
        let eq7_e1029_d_n19: f64 = (eq7_e1027_d_n19 * p.p32);
        let eq7_e1029_d_n20: f64 = (eq7_e1027_d_n20 * p.p32);
        let eq7_e1029_d_b0: f64 = (eq7_e1027_d_b0 * p.p32);
        let eq7_e1029_d_b1: f64 = (eq7_e1027_d_b1 * p.p32);
        let eq7_e1029_d_b2: f64 = (eq7_e1027_d_b2 * p.p32);
        let eq7_e1029_d_b3: f64 = (eq7_e1027_d_b3 * p.p32);
        let eq7_e1029_d_b4: f64 = (eq7_e1027_d_b4 * p.p32);
        let eq7_e1029_d_b5: f64 = (eq7_e1027_d_b5 * p.p32);
        let eq7_e1029_d_b6: f64 = (eq7_e1027_d_b6 * p.p32);
        let eq7_e1029_d_b7: f64 = (eq7_e1027_d_b7 * p.p32);
        let eq7_e1029_d_b8: f64 = (eq7_e1027_d_b8 * p.p32);
        let eq7_e1029_d_b9: f64 = (eq7_e1027_d_b9 * p.p32);
        let eq7_e1029_d_b10: f64 = (eq7_e1027_d_b10 * p.p32);
        let eq7_e1029_d_b11: f64 = (eq7_e1027_d_b11 * p.p32);
        let eq7_e1029_d_b12: f64 = (eq7_e1027_d_b12 * p.p32);
        let eq7_e1029_d_b13: f64 = (eq7_e1027_d_b13 * p.p32);
        let eq7_e1029_d_b14: f64 = (eq7_e1027_d_b14 * p.p32);
        let eq7_e1029_d_b15: f64 = (eq7_e1027_d_b15 * p.p32);
        let eq7_e1029_d_b16: f64 = (eq7_e1027_d_b16 * p.p32);
        let eq7_e1029_d_b17: f64 = (eq7_e1027_d_b17 * p.p32);
        let eq7_e1029_d_b18: f64 = (eq7_e1027_d_b18 * p.p32);
        let eq7_e1029_d_b19: f64 = (eq7_e1027_d_b19 * p.p32);
        let eq7_e1029_d_b20: f64 = (eq7_e1027_d_b20 * p.p32);
        let eq7_e1029_d_b21: f64 = (eq7_e1027_d_b21 * p.p32);
        let eq7_e1029_d_b22: f64 = (eq7_e1027_d_b22 * p.p32);
        let eq7_e1029_d_b23: f64 = (eq7_e1027_d_b23 * p.p32);
        let eq7_e1029_d_b24: f64 = (eq7_e1027_d_b24 * p.p32);
        let eq7_e1031: f64 = (eq7_e1029 * s.v[842]);
        let eq7_e1031_d_n0: f64 = ((eq7_e1029_d_n0 * s.v[842]) + (eq7_e1029 * s.dn[842][0]));
        let eq7_e1031_d_n1: f64 = ((eq7_e1029_d_n1 * s.v[842]) + (eq7_e1029 * s.dn[842][1]));
        let eq7_e1031_d_n2: f64 = ((eq7_e1029_d_n2 * s.v[842]) + (eq7_e1029 * s.dn[842][2]));
        let eq7_e1031_d_n3: f64 = ((eq7_e1029_d_n3 * s.v[842]) + (eq7_e1029 * s.dn[842][3]));
        let eq7_e1031_d_n4: f64 = ((eq7_e1029_d_n4 * s.v[842]) + (eq7_e1029 * s.dn[842][4]));
        let eq7_e1031_d_n5: f64 = ((eq7_e1029_d_n5 * s.v[842]) + (eq7_e1029 * s.dn[842][5]));
        let eq7_e1031_d_n6: f64 = ((eq7_e1029_d_n6 * s.v[842]) + (eq7_e1029 * s.dn[842][6]));
        let eq7_e1031_d_n7: f64 = ((eq7_e1029_d_n7 * s.v[842]) + (eq7_e1029 * s.dn[842][7]));
        let eq7_e1031_d_n8: f64 = ((eq7_e1029_d_n8 * s.v[842]) + (eq7_e1029 * s.dn[842][8]));
        let eq7_e1031_d_n9: f64 = ((eq7_e1029_d_n9 * s.v[842]) + (eq7_e1029 * s.dn[842][9]));
        let eq7_e1031_d_n10: f64 = ((eq7_e1029_d_n10 * s.v[842]) + (eq7_e1029 * s.dn[842][10]));
        let eq7_e1031_d_n11: f64 = ((eq7_e1029_d_n11 * s.v[842]) + (eq7_e1029 * s.dn[842][11]));
        let eq7_e1031_d_n12: f64 = ((eq7_e1029_d_n12 * s.v[842]) + (eq7_e1029 * s.dn[842][12]));
        let eq7_e1031_d_n13: f64 = ((eq7_e1029_d_n13 * s.v[842]) + (eq7_e1029 * s.dn[842][13]));
        let eq7_e1031_d_n14: f64 = ((eq7_e1029_d_n14 * s.v[842]) + (eq7_e1029 * s.dn[842][14]));
        let eq7_e1031_d_n15: f64 = ((eq7_e1029_d_n15 * s.v[842]) + (eq7_e1029 * s.dn[842][15]));
        let eq7_e1031_d_n16: f64 = ((eq7_e1029_d_n16 * s.v[842]) + (eq7_e1029 * s.dn[842][16]));
        let eq7_e1031_d_n17: f64 = ((eq7_e1029_d_n17 * s.v[842]) + (eq7_e1029 * s.dn[842][17]));
        let eq7_e1031_d_n18: f64 = ((eq7_e1029_d_n18 * s.v[842]) + (eq7_e1029 * s.dn[842][18]));
        let eq7_e1031_d_n19: f64 = ((eq7_e1029_d_n19 * s.v[842]) + (eq7_e1029 * s.dn[842][19]));
        let eq7_e1031_d_n20: f64 = ((eq7_e1029_d_n20 * s.v[842]) + (eq7_e1029 * s.dn[842][20]));
        let eq7_e1031_d_b0: f64 = ((eq7_e1029_d_b0 * s.v[842]) + (eq7_e1029 * s.db[842][0]));
        let eq7_e1031_d_b1: f64 = ((eq7_e1029_d_b1 * s.v[842]) + (eq7_e1029 * s.db[842][1]));
        let eq7_e1031_d_b2: f64 = ((eq7_e1029_d_b2 * s.v[842]) + (eq7_e1029 * s.db[842][2]));
        let eq7_e1031_d_b3: f64 = ((eq7_e1029_d_b3 * s.v[842]) + (eq7_e1029 * s.db[842][3]));
        let eq7_e1031_d_b4: f64 = ((eq7_e1029_d_b4 * s.v[842]) + (eq7_e1029 * s.db[842][4]));
        let eq7_e1031_d_b5: f64 = ((eq7_e1029_d_b5 * s.v[842]) + (eq7_e1029 * s.db[842][5]));
        let eq7_e1031_d_b6: f64 = ((eq7_e1029_d_b6 * s.v[842]) + (eq7_e1029 * s.db[842][6]));
        let eq7_e1031_d_b7: f64 = ((eq7_e1029_d_b7 * s.v[842]) + (eq7_e1029 * s.db[842][7]));
        let eq7_e1031_d_b8: f64 = ((eq7_e1029_d_b8 * s.v[842]) + (eq7_e1029 * s.db[842][8]));
        let eq7_e1031_d_b9: f64 = ((eq7_e1029_d_b9 * s.v[842]) + (eq7_e1029 * s.db[842][9]));
        let eq7_e1031_d_b10: f64 = ((eq7_e1029_d_b10 * s.v[842]) + (eq7_e1029 * s.db[842][10]));
        let eq7_e1031_d_b11: f64 = ((eq7_e1029_d_b11 * s.v[842]) + (eq7_e1029 * s.db[842][11]));
        let eq7_e1031_d_b12: f64 = ((eq7_e1029_d_b12 * s.v[842]) + (eq7_e1029 * s.db[842][12]));
        let eq7_e1031_d_b13: f64 = ((eq7_e1029_d_b13 * s.v[842]) + (eq7_e1029 * s.db[842][13]));
        let eq7_e1031_d_b14: f64 = ((eq7_e1029_d_b14 * s.v[842]) + (eq7_e1029 * s.db[842][14]));
        let eq7_e1031_d_b15: f64 = ((eq7_e1029_d_b15 * s.v[842]) + (eq7_e1029 * s.db[842][15]));
        let eq7_e1031_d_b16: f64 = ((eq7_e1029_d_b16 * s.v[842]) + (eq7_e1029 * s.db[842][16]));
        let eq7_e1031_d_b17: f64 = ((eq7_e1029_d_b17 * s.v[842]) + (eq7_e1029 * s.db[842][17]));
        let eq7_e1031_d_b18: f64 = ((eq7_e1029_d_b18 * s.v[842]) + (eq7_e1029 * s.db[842][18]));
        let eq7_e1031_d_b19: f64 = ((eq7_e1029_d_b19 * s.v[842]) + (eq7_e1029 * s.db[842][19]));
        let eq7_e1031_d_b20: f64 = ((eq7_e1029_d_b20 * s.v[842]) + (eq7_e1029 * s.db[842][20]));
        let eq7_e1031_d_b21: f64 = ((eq7_e1029_d_b21 * s.v[842]) + (eq7_e1029 * s.db[842][21]));
        let eq7_e1031_d_b22: f64 = ((eq7_e1029_d_b22 * s.v[842]) + (eq7_e1029 * s.db[842][22]));
        let eq7_e1031_d_b23: f64 = ((eq7_e1029_d_b23 * s.v[842]) + (eq7_e1029 * s.db[842][23]));
        let eq7_e1031_d_b24: f64 = ((eq7_e1029_d_b24 * s.v[842]) + (eq7_e1029 * s.db[842][24]));
        (eq7_e1031, eq7_e1031_d_n0, eq7_e1031_d_n1, eq7_e1031_d_n2, eq7_e1031_d_n3, eq7_e1031_d_n4, eq7_e1031_d_n5, eq7_e1031_d_n6, eq7_e1031_d_n7, eq7_e1031_d_n8, eq7_e1031_d_n9, eq7_e1031_d_n10, eq7_e1031_d_n11, eq7_e1031_d_n12, eq7_e1031_d_n13, eq7_e1031_d_n14, eq7_e1031_d_n15, eq7_e1031_d_n16, eq7_e1031_d_n17, eq7_e1031_d_n18, eq7_e1031_d_n19, eq7_e1031_d_n20, eq7_e1031_d_b0, eq7_e1031_d_b1, eq7_e1031_d_b2, eq7_e1031_d_b3, eq7_e1031_d_b4, eq7_e1031_d_b5, eq7_e1031_d_b6, eq7_e1031_d_b7, eq7_e1031_d_b8, eq7_e1031_d_b9, eq7_e1031_d_b10, eq7_e1031_d_b11, eq7_e1031_d_b12, eq7_e1031_d_b13, eq7_e1031_d_b14, eq7_e1031_d_b15, eq7_e1031_d_b16, eq7_e1031_d_b17, eq7_e1031_d_b18, eq7_e1031_d_b19, eq7_e1031_d_b20, eq7_e1031_d_b21, eq7_e1031_d_b22, eq7_e1031_d_b23, eq7_e1031_d_b24,)
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
        let eq8_e1036: f64 = (s.v[0] * s.v[19]);
        let eq8_e1036_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq8_e1036_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq8_e1036_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq8_e1036_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq8_e1036_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq8_e1036_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq8_e1036_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq8_e1036_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq8_e1036_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq8_e1036_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq8_e1036_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq8_e1036_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq8_e1036_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq8_e1036_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq8_e1036_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq8_e1036_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq8_e1036_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq8_e1036_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq8_e1036_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq8_e1036_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq8_e1036_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq8_e1036_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq8_e1036_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq8_e1036_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq8_e1036_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq8_e1036_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq8_e1036_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq8_e1036_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq8_e1036_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq8_e1036_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq8_e1036_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq8_e1036_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq8_e1036_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq8_e1036_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq8_e1036_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq8_e1036_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq8_e1036_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq8_e1036_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq8_e1036_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq8_e1036_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq8_e1036_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq8_e1036_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq8_e1036_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq8_e1036_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq8_e1036_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq8_e1036_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq8_e1038: f64 = (eq8_e1036 * p.p32);
        let eq8_e1038_d_n0: f64 = (eq8_e1036_d_n0 * p.p32);
        let eq8_e1038_d_n1: f64 = (eq8_e1036_d_n1 * p.p32);
        let eq8_e1038_d_n2: f64 = (eq8_e1036_d_n2 * p.p32);
        let eq8_e1038_d_n3: f64 = (eq8_e1036_d_n3 * p.p32);
        let eq8_e1038_d_n4: f64 = (eq8_e1036_d_n4 * p.p32);
        let eq8_e1038_d_n5: f64 = (eq8_e1036_d_n5 * p.p32);
        let eq8_e1038_d_n6: f64 = (eq8_e1036_d_n6 * p.p32);
        let eq8_e1038_d_n7: f64 = (eq8_e1036_d_n7 * p.p32);
        let eq8_e1038_d_n8: f64 = (eq8_e1036_d_n8 * p.p32);
        let eq8_e1038_d_n9: f64 = (eq8_e1036_d_n9 * p.p32);
        let eq8_e1038_d_n10: f64 = (eq8_e1036_d_n10 * p.p32);
        let eq8_e1038_d_n11: f64 = (eq8_e1036_d_n11 * p.p32);
        let eq8_e1038_d_n12: f64 = (eq8_e1036_d_n12 * p.p32);
        let eq8_e1038_d_n13: f64 = (eq8_e1036_d_n13 * p.p32);
        let eq8_e1038_d_n14: f64 = (eq8_e1036_d_n14 * p.p32);
        let eq8_e1038_d_n15: f64 = (eq8_e1036_d_n15 * p.p32);
        let eq8_e1038_d_n16: f64 = (eq8_e1036_d_n16 * p.p32);
        let eq8_e1038_d_n17: f64 = (eq8_e1036_d_n17 * p.p32);
        let eq8_e1038_d_n18: f64 = (eq8_e1036_d_n18 * p.p32);
        let eq8_e1038_d_n19: f64 = (eq8_e1036_d_n19 * p.p32);
        let eq8_e1038_d_n20: f64 = (eq8_e1036_d_n20 * p.p32);
        let eq8_e1038_d_b0: f64 = (eq8_e1036_d_b0 * p.p32);
        let eq8_e1038_d_b1: f64 = (eq8_e1036_d_b1 * p.p32);
        let eq8_e1038_d_b2: f64 = (eq8_e1036_d_b2 * p.p32);
        let eq8_e1038_d_b3: f64 = (eq8_e1036_d_b3 * p.p32);
        let eq8_e1038_d_b4: f64 = (eq8_e1036_d_b4 * p.p32);
        let eq8_e1038_d_b5: f64 = (eq8_e1036_d_b5 * p.p32);
        let eq8_e1038_d_b6: f64 = (eq8_e1036_d_b6 * p.p32);
        let eq8_e1038_d_b7: f64 = (eq8_e1036_d_b7 * p.p32);
        let eq8_e1038_d_b8: f64 = (eq8_e1036_d_b8 * p.p32);
        let eq8_e1038_d_b9: f64 = (eq8_e1036_d_b9 * p.p32);
        let eq8_e1038_d_b10: f64 = (eq8_e1036_d_b10 * p.p32);
        let eq8_e1038_d_b11: f64 = (eq8_e1036_d_b11 * p.p32);
        let eq8_e1038_d_b12: f64 = (eq8_e1036_d_b12 * p.p32);
        let eq8_e1038_d_b13: f64 = (eq8_e1036_d_b13 * p.p32);
        let eq8_e1038_d_b14: f64 = (eq8_e1036_d_b14 * p.p32);
        let eq8_e1038_d_b15: f64 = (eq8_e1036_d_b15 * p.p32);
        let eq8_e1038_d_b16: f64 = (eq8_e1036_d_b16 * p.p32);
        let eq8_e1038_d_b17: f64 = (eq8_e1036_d_b17 * p.p32);
        let eq8_e1038_d_b18: f64 = (eq8_e1036_d_b18 * p.p32);
        let eq8_e1038_d_b19: f64 = (eq8_e1036_d_b19 * p.p32);
        let eq8_e1038_d_b20: f64 = (eq8_e1036_d_b20 * p.p32);
        let eq8_e1038_d_b21: f64 = (eq8_e1036_d_b21 * p.p32);
        let eq8_e1038_d_b22: f64 = (eq8_e1036_d_b22 * p.p32);
        let eq8_e1038_d_b23: f64 = (eq8_e1036_d_b23 * p.p32);
        let eq8_e1038_d_b24: f64 = (eq8_e1036_d_b24 * p.p32);
        let eq8_e1040: f64 = (eq8_e1038 * s.v[843]);
        let eq8_e1040_d_n0: f64 = ((eq8_e1038_d_n0 * s.v[843]) + (eq8_e1038 * s.dn[843][0]));
        let eq8_e1040_d_n1: f64 = ((eq8_e1038_d_n1 * s.v[843]) + (eq8_e1038 * s.dn[843][1]));
        let eq8_e1040_d_n2: f64 = ((eq8_e1038_d_n2 * s.v[843]) + (eq8_e1038 * s.dn[843][2]));
        let eq8_e1040_d_n3: f64 = ((eq8_e1038_d_n3 * s.v[843]) + (eq8_e1038 * s.dn[843][3]));
        let eq8_e1040_d_n4: f64 = ((eq8_e1038_d_n4 * s.v[843]) + (eq8_e1038 * s.dn[843][4]));
        let eq8_e1040_d_n5: f64 = ((eq8_e1038_d_n5 * s.v[843]) + (eq8_e1038 * s.dn[843][5]));
        let eq8_e1040_d_n6: f64 = ((eq8_e1038_d_n6 * s.v[843]) + (eq8_e1038 * s.dn[843][6]));
        let eq8_e1040_d_n7: f64 = ((eq8_e1038_d_n7 * s.v[843]) + (eq8_e1038 * s.dn[843][7]));
        let eq8_e1040_d_n8: f64 = ((eq8_e1038_d_n8 * s.v[843]) + (eq8_e1038 * s.dn[843][8]));
        let eq8_e1040_d_n9: f64 = ((eq8_e1038_d_n9 * s.v[843]) + (eq8_e1038 * s.dn[843][9]));
        let eq8_e1040_d_n10: f64 = ((eq8_e1038_d_n10 * s.v[843]) + (eq8_e1038 * s.dn[843][10]));
        let eq8_e1040_d_n11: f64 = ((eq8_e1038_d_n11 * s.v[843]) + (eq8_e1038 * s.dn[843][11]));
        let eq8_e1040_d_n12: f64 = ((eq8_e1038_d_n12 * s.v[843]) + (eq8_e1038 * s.dn[843][12]));
        let eq8_e1040_d_n13: f64 = ((eq8_e1038_d_n13 * s.v[843]) + (eq8_e1038 * s.dn[843][13]));
        let eq8_e1040_d_n14: f64 = ((eq8_e1038_d_n14 * s.v[843]) + (eq8_e1038 * s.dn[843][14]));
        let eq8_e1040_d_n15: f64 = ((eq8_e1038_d_n15 * s.v[843]) + (eq8_e1038 * s.dn[843][15]));
        let eq8_e1040_d_n16: f64 = ((eq8_e1038_d_n16 * s.v[843]) + (eq8_e1038 * s.dn[843][16]));
        let eq8_e1040_d_n17: f64 = ((eq8_e1038_d_n17 * s.v[843]) + (eq8_e1038 * s.dn[843][17]));
        let eq8_e1040_d_n18: f64 = ((eq8_e1038_d_n18 * s.v[843]) + (eq8_e1038 * s.dn[843][18]));
        let eq8_e1040_d_n19: f64 = ((eq8_e1038_d_n19 * s.v[843]) + (eq8_e1038 * s.dn[843][19]));
        let eq8_e1040_d_n20: f64 = ((eq8_e1038_d_n20 * s.v[843]) + (eq8_e1038 * s.dn[843][20]));
        let eq8_e1040_d_b0: f64 = ((eq8_e1038_d_b0 * s.v[843]) + (eq8_e1038 * s.db[843][0]));
        let eq8_e1040_d_b1: f64 = ((eq8_e1038_d_b1 * s.v[843]) + (eq8_e1038 * s.db[843][1]));
        let eq8_e1040_d_b2: f64 = ((eq8_e1038_d_b2 * s.v[843]) + (eq8_e1038 * s.db[843][2]));
        let eq8_e1040_d_b3: f64 = ((eq8_e1038_d_b3 * s.v[843]) + (eq8_e1038 * s.db[843][3]));
        let eq8_e1040_d_b4: f64 = ((eq8_e1038_d_b4 * s.v[843]) + (eq8_e1038 * s.db[843][4]));
        let eq8_e1040_d_b5: f64 = ((eq8_e1038_d_b5 * s.v[843]) + (eq8_e1038 * s.db[843][5]));
        let eq8_e1040_d_b6: f64 = ((eq8_e1038_d_b6 * s.v[843]) + (eq8_e1038 * s.db[843][6]));
        let eq8_e1040_d_b7: f64 = ((eq8_e1038_d_b7 * s.v[843]) + (eq8_e1038 * s.db[843][7]));
        let eq8_e1040_d_b8: f64 = ((eq8_e1038_d_b8 * s.v[843]) + (eq8_e1038 * s.db[843][8]));
        let eq8_e1040_d_b9: f64 = ((eq8_e1038_d_b9 * s.v[843]) + (eq8_e1038 * s.db[843][9]));
        let eq8_e1040_d_b10: f64 = ((eq8_e1038_d_b10 * s.v[843]) + (eq8_e1038 * s.db[843][10]));
        let eq8_e1040_d_b11: f64 = ((eq8_e1038_d_b11 * s.v[843]) + (eq8_e1038 * s.db[843][11]));
        let eq8_e1040_d_b12: f64 = ((eq8_e1038_d_b12 * s.v[843]) + (eq8_e1038 * s.db[843][12]));
        let eq8_e1040_d_b13: f64 = ((eq8_e1038_d_b13 * s.v[843]) + (eq8_e1038 * s.db[843][13]));
        let eq8_e1040_d_b14: f64 = ((eq8_e1038_d_b14 * s.v[843]) + (eq8_e1038 * s.db[843][14]));
        let eq8_e1040_d_b15: f64 = ((eq8_e1038_d_b15 * s.v[843]) + (eq8_e1038 * s.db[843][15]));
        let eq8_e1040_d_b16: f64 = ((eq8_e1038_d_b16 * s.v[843]) + (eq8_e1038 * s.db[843][16]));
        let eq8_e1040_d_b17: f64 = ((eq8_e1038_d_b17 * s.v[843]) + (eq8_e1038 * s.db[843][17]));
        let eq8_e1040_d_b18: f64 = ((eq8_e1038_d_b18 * s.v[843]) + (eq8_e1038 * s.db[843][18]));
        let eq8_e1040_d_b19: f64 = ((eq8_e1038_d_b19 * s.v[843]) + (eq8_e1038 * s.db[843][19]));
        let eq8_e1040_d_b20: f64 = ((eq8_e1038_d_b20 * s.v[843]) + (eq8_e1038 * s.db[843][20]));
        let eq8_e1040_d_b21: f64 = ((eq8_e1038_d_b21 * s.v[843]) + (eq8_e1038 * s.db[843][21]));
        let eq8_e1040_d_b22: f64 = ((eq8_e1038_d_b22 * s.v[843]) + (eq8_e1038 * s.db[843][22]));
        let eq8_e1040_d_b23: f64 = ((eq8_e1038_d_b23 * s.v[843]) + (eq8_e1038 * s.db[843][23]));
        let eq8_e1040_d_b24: f64 = ((eq8_e1038_d_b24 * s.v[843]) + (eq8_e1038 * s.db[843][24]));
        let eq8_value: f64 = eq8_e1040;
        let eq8_node_derivatives: [f64; 21] = [eq8_e1040_d_n0, eq8_e1040_d_n1, eq8_e1040_d_n2, eq8_e1040_d_n3, eq8_e1040_d_n4, eq8_e1040_d_n5, eq8_e1040_d_n6, eq8_e1040_d_n7, eq8_e1040_d_n8, eq8_e1040_d_n9, eq8_e1040_d_n10, eq8_e1040_d_n11, eq8_e1040_d_n12, eq8_e1040_d_n13, eq8_e1040_d_n14, eq8_e1040_d_n15, eq8_e1040_d_n16, eq8_e1040_d_n17, eq8_e1040_d_n18, eq8_e1040_d_n19, eq8_e1040_d_n20];
        let eq8_branch_derivatives: [f64; 25] = [eq8_e1040_d_b0, eq8_e1040_d_b1, eq8_e1040_d_b2, eq8_e1040_d_b3, eq8_e1040_d_b4, eq8_e1040_d_b5, eq8_e1040_d_b6, eq8_e1040_d_b7, eq8_e1040_d_b8, eq8_e1040_d_b9, eq8_e1040_d_b10, eq8_e1040_d_b11, eq8_e1040_d_b12, eq8_e1040_d_b13, eq8_e1040_d_b14, eq8_e1040_d_b15, eq8_e1040_d_b16, eq8_e1040_d_b17, eq8_e1040_d_b18, eq8_e1040_d_b19, eq8_e1040_d_b20, eq8_e1040_d_b21, eq8_e1040_d_b22, eq8_e1040_d_b23, eq8_e1040_d_b24];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(8),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let eq9_e1043: f64 = (s.v[0] * s.v[19]);
        let eq9_e1043_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq9_e1043_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq9_e1043_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq9_e1043_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq9_e1043_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq9_e1043_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq9_e1043_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq9_e1043_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq9_e1043_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq9_e1043_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq9_e1043_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq9_e1043_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq9_e1043_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq9_e1043_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq9_e1043_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq9_e1043_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq9_e1043_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq9_e1043_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq9_e1043_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq9_e1043_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq9_e1043_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq9_e1043_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq9_e1043_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq9_e1043_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq9_e1043_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq9_e1043_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq9_e1043_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq9_e1043_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq9_e1043_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq9_e1043_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq9_e1043_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq9_e1043_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq9_e1043_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq9_e1043_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq9_e1043_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq9_e1043_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq9_e1043_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq9_e1043_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq9_e1043_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq9_e1043_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq9_e1043_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq9_e1043_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq9_e1043_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq9_e1043_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq9_e1043_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq9_e1043_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq9_e1045: f64 = (eq9_e1043 * p.p32);
        let eq9_e1045_d_n0: f64 = (eq9_e1043_d_n0 * p.p32);
        let eq9_e1045_d_n1: f64 = (eq9_e1043_d_n1 * p.p32);
        let eq9_e1045_d_n2: f64 = (eq9_e1043_d_n2 * p.p32);
        let eq9_e1045_d_n3: f64 = (eq9_e1043_d_n3 * p.p32);
        let eq9_e1045_d_n4: f64 = (eq9_e1043_d_n4 * p.p32);
        let eq9_e1045_d_n5: f64 = (eq9_e1043_d_n5 * p.p32);
        let eq9_e1045_d_n6: f64 = (eq9_e1043_d_n6 * p.p32);
        let eq9_e1045_d_n7: f64 = (eq9_e1043_d_n7 * p.p32);
        let eq9_e1045_d_n8: f64 = (eq9_e1043_d_n8 * p.p32);
        let eq9_e1045_d_n9: f64 = (eq9_e1043_d_n9 * p.p32);
        let eq9_e1045_d_n10: f64 = (eq9_e1043_d_n10 * p.p32);
        let eq9_e1045_d_n11: f64 = (eq9_e1043_d_n11 * p.p32);
        let eq9_e1045_d_n12: f64 = (eq9_e1043_d_n12 * p.p32);
        let eq9_e1045_d_n13: f64 = (eq9_e1043_d_n13 * p.p32);
        let eq9_e1045_d_n14: f64 = (eq9_e1043_d_n14 * p.p32);
        let eq9_e1045_d_n15: f64 = (eq9_e1043_d_n15 * p.p32);
        let eq9_e1045_d_n16: f64 = (eq9_e1043_d_n16 * p.p32);
        let eq9_e1045_d_n17: f64 = (eq9_e1043_d_n17 * p.p32);
        let eq9_e1045_d_n18: f64 = (eq9_e1043_d_n18 * p.p32);
        let eq9_e1045_d_n19: f64 = (eq9_e1043_d_n19 * p.p32);
        let eq9_e1045_d_n20: f64 = (eq9_e1043_d_n20 * p.p32);
        let eq9_e1045_d_b0: f64 = (eq9_e1043_d_b0 * p.p32);
        let eq9_e1045_d_b1: f64 = (eq9_e1043_d_b1 * p.p32);
        let eq9_e1045_d_b2: f64 = (eq9_e1043_d_b2 * p.p32);
        let eq9_e1045_d_b3: f64 = (eq9_e1043_d_b3 * p.p32);
        let eq9_e1045_d_b4: f64 = (eq9_e1043_d_b4 * p.p32);
        let eq9_e1045_d_b5: f64 = (eq9_e1043_d_b5 * p.p32);
        let eq9_e1045_d_b6: f64 = (eq9_e1043_d_b6 * p.p32);
        let eq9_e1045_d_b7: f64 = (eq9_e1043_d_b7 * p.p32);
        let eq9_e1045_d_b8: f64 = (eq9_e1043_d_b8 * p.p32);
        let eq9_e1045_d_b9: f64 = (eq9_e1043_d_b9 * p.p32);
        let eq9_e1045_d_b10: f64 = (eq9_e1043_d_b10 * p.p32);
        let eq9_e1045_d_b11: f64 = (eq9_e1043_d_b11 * p.p32);
        let eq9_e1045_d_b12: f64 = (eq9_e1043_d_b12 * p.p32);
        let eq9_e1045_d_b13: f64 = (eq9_e1043_d_b13 * p.p32);
        let eq9_e1045_d_b14: f64 = (eq9_e1043_d_b14 * p.p32);
        let eq9_e1045_d_b15: f64 = (eq9_e1043_d_b15 * p.p32);
        let eq9_e1045_d_b16: f64 = (eq9_e1043_d_b16 * p.p32);
        let eq9_e1045_d_b17: f64 = (eq9_e1043_d_b17 * p.p32);
        let eq9_e1045_d_b18: f64 = (eq9_e1043_d_b18 * p.p32);
        let eq9_e1045_d_b19: f64 = (eq9_e1043_d_b19 * p.p32);
        let eq9_e1045_d_b20: f64 = (eq9_e1043_d_b20 * p.p32);
        let eq9_e1045_d_b21: f64 = (eq9_e1043_d_b21 * p.p32);
        let eq9_e1045_d_b22: f64 = (eq9_e1043_d_b22 * p.p32);
        let eq9_e1045_d_b23: f64 = (eq9_e1043_d_b23 * p.p32);
        let eq9_e1045_d_b24: f64 = (eq9_e1043_d_b24 * p.p32);
        let eq9_e1047: f64 = (eq9_e1045 * s.v[839]);
        let eq9_e1047_d_n0: f64 = ((eq9_e1045_d_n0 * s.v[839]) + (eq9_e1045 * s.dn[839][0]));
        let eq9_e1047_d_n1: f64 = ((eq9_e1045_d_n1 * s.v[839]) + (eq9_e1045 * s.dn[839][1]));
        let eq9_e1047_d_n2: f64 = ((eq9_e1045_d_n2 * s.v[839]) + (eq9_e1045 * s.dn[839][2]));
        let eq9_e1047_d_n3: f64 = ((eq9_e1045_d_n3 * s.v[839]) + (eq9_e1045 * s.dn[839][3]));
        let eq9_e1047_d_n4: f64 = ((eq9_e1045_d_n4 * s.v[839]) + (eq9_e1045 * s.dn[839][4]));
        let eq9_e1047_d_n5: f64 = ((eq9_e1045_d_n5 * s.v[839]) + (eq9_e1045 * s.dn[839][5]));
        let eq9_e1047_d_n6: f64 = ((eq9_e1045_d_n6 * s.v[839]) + (eq9_e1045 * s.dn[839][6]));
        let eq9_e1047_d_n7: f64 = ((eq9_e1045_d_n7 * s.v[839]) + (eq9_e1045 * s.dn[839][7]));
        let eq9_e1047_d_n8: f64 = ((eq9_e1045_d_n8 * s.v[839]) + (eq9_e1045 * s.dn[839][8]));
        let eq9_e1047_d_n9: f64 = ((eq9_e1045_d_n9 * s.v[839]) + (eq9_e1045 * s.dn[839][9]));
        let eq9_e1047_d_n10: f64 = ((eq9_e1045_d_n10 * s.v[839]) + (eq9_e1045 * s.dn[839][10]));
        let eq9_e1047_d_n11: f64 = ((eq9_e1045_d_n11 * s.v[839]) + (eq9_e1045 * s.dn[839][11]));
        let eq9_e1047_d_n12: f64 = ((eq9_e1045_d_n12 * s.v[839]) + (eq9_e1045 * s.dn[839][12]));
        let eq9_e1047_d_n13: f64 = ((eq9_e1045_d_n13 * s.v[839]) + (eq9_e1045 * s.dn[839][13]));
        let eq9_e1047_d_n14: f64 = ((eq9_e1045_d_n14 * s.v[839]) + (eq9_e1045 * s.dn[839][14]));
        let eq9_e1047_d_n15: f64 = ((eq9_e1045_d_n15 * s.v[839]) + (eq9_e1045 * s.dn[839][15]));
        let eq9_e1047_d_n16: f64 = ((eq9_e1045_d_n16 * s.v[839]) + (eq9_e1045 * s.dn[839][16]));
        let eq9_e1047_d_n17: f64 = ((eq9_e1045_d_n17 * s.v[839]) + (eq9_e1045 * s.dn[839][17]));
        let eq9_e1047_d_n18: f64 = ((eq9_e1045_d_n18 * s.v[839]) + (eq9_e1045 * s.dn[839][18]));
        let eq9_e1047_d_n19: f64 = ((eq9_e1045_d_n19 * s.v[839]) + (eq9_e1045 * s.dn[839][19]));
        let eq9_e1047_d_n20: f64 = ((eq9_e1045_d_n20 * s.v[839]) + (eq9_e1045 * s.dn[839][20]));
        let eq9_e1047_d_b0: f64 = ((eq9_e1045_d_b0 * s.v[839]) + (eq9_e1045 * s.db[839][0]));
        let eq9_e1047_d_b1: f64 = ((eq9_e1045_d_b1 * s.v[839]) + (eq9_e1045 * s.db[839][1]));
        let eq9_e1047_d_b2: f64 = ((eq9_e1045_d_b2 * s.v[839]) + (eq9_e1045 * s.db[839][2]));
        let eq9_e1047_d_b3: f64 = ((eq9_e1045_d_b3 * s.v[839]) + (eq9_e1045 * s.db[839][3]));
        let eq9_e1047_d_b4: f64 = ((eq9_e1045_d_b4 * s.v[839]) + (eq9_e1045 * s.db[839][4]));
        let eq9_e1047_d_b5: f64 = ((eq9_e1045_d_b5 * s.v[839]) + (eq9_e1045 * s.db[839][5]));
        let eq9_e1047_d_b6: f64 = ((eq9_e1045_d_b6 * s.v[839]) + (eq9_e1045 * s.db[839][6]));
        let eq9_e1047_d_b7: f64 = ((eq9_e1045_d_b7 * s.v[839]) + (eq9_e1045 * s.db[839][7]));
        let eq9_e1047_d_b8: f64 = ((eq9_e1045_d_b8 * s.v[839]) + (eq9_e1045 * s.db[839][8]));
        let eq9_e1047_d_b9: f64 = ((eq9_e1045_d_b9 * s.v[839]) + (eq9_e1045 * s.db[839][9]));
        let eq9_e1047_d_b10: f64 = ((eq9_e1045_d_b10 * s.v[839]) + (eq9_e1045 * s.db[839][10]));
        let eq9_e1047_d_b11: f64 = ((eq9_e1045_d_b11 * s.v[839]) + (eq9_e1045 * s.db[839][11]));
        let eq9_e1047_d_b12: f64 = ((eq9_e1045_d_b12 * s.v[839]) + (eq9_e1045 * s.db[839][12]));
        let eq9_e1047_d_b13: f64 = ((eq9_e1045_d_b13 * s.v[839]) + (eq9_e1045 * s.db[839][13]));
        let eq9_e1047_d_b14: f64 = ((eq9_e1045_d_b14 * s.v[839]) + (eq9_e1045 * s.db[839][14]));
        let eq9_e1047_d_b15: f64 = ((eq9_e1045_d_b15 * s.v[839]) + (eq9_e1045 * s.db[839][15]));
        let eq9_e1047_d_b16: f64 = ((eq9_e1045_d_b16 * s.v[839]) + (eq9_e1045 * s.db[839][16]));
        let eq9_e1047_d_b17: f64 = ((eq9_e1045_d_b17 * s.v[839]) + (eq9_e1045 * s.db[839][17]));
        let eq9_e1047_d_b18: f64 = ((eq9_e1045_d_b18 * s.v[839]) + (eq9_e1045 * s.db[839][18]));
        let eq9_e1047_d_b19: f64 = ((eq9_e1045_d_b19 * s.v[839]) + (eq9_e1045 * s.db[839][19]));
        let eq9_e1047_d_b20: f64 = ((eq9_e1045_d_b20 * s.v[839]) + (eq9_e1045 * s.db[839][20]));
        let eq9_e1047_d_b21: f64 = ((eq9_e1045_d_b21 * s.v[839]) + (eq9_e1045 * s.db[839][21]));
        let eq9_e1047_d_b22: f64 = ((eq9_e1045_d_b22 * s.v[839]) + (eq9_e1045 * s.db[839][22]));
        let eq9_e1047_d_b23: f64 = ((eq9_e1045_d_b23 * s.v[839]) + (eq9_e1045 * s.db[839][23]));
        let eq9_e1047_d_b24: f64 = ((eq9_e1045_d_b24 * s.v[839]) + (eq9_e1045 * s.db[839][24]));
        let eq9_value: f64 = eq9_e1047;
        let eq9_node_derivatives: [f64; 21] = [eq9_e1047_d_n0, eq9_e1047_d_n1, eq9_e1047_d_n2, eq9_e1047_d_n3, eq9_e1047_d_n4, eq9_e1047_d_n5, eq9_e1047_d_n6, eq9_e1047_d_n7, eq9_e1047_d_n8, eq9_e1047_d_n9, eq9_e1047_d_n10, eq9_e1047_d_n11, eq9_e1047_d_n12, eq9_e1047_d_n13, eq9_e1047_d_n14, eq9_e1047_d_n15, eq9_e1047_d_n16, eq9_e1047_d_n17, eq9_e1047_d_n18, eq9_e1047_d_n19, eq9_e1047_d_n20];
        let eq9_branch_derivatives: [f64; 25] = [eq9_e1047_d_b0, eq9_e1047_d_b1, eq9_e1047_d_b2, eq9_e1047_d_b3, eq9_e1047_d_b4, eq9_e1047_d_b5, eq9_e1047_d_b6, eq9_e1047_d_b7, eq9_e1047_d_b8, eq9_e1047_d_b9, eq9_e1047_d_b10, eq9_e1047_d_b11, eq9_e1047_d_b12, eq9_e1047_d_b13, eq9_e1047_d_b14, eq9_e1047_d_b15, eq9_e1047_d_b16, eq9_e1047_d_b17, eq9_e1047_d_b18, eq9_e1047_d_b19, eq9_e1047_d_b20, eq9_e1047_d_b21, eq9_e1047_d_b22, eq9_e1047_d_b23, eq9_e1047_d_b24];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_4(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq10_e1050: f64 = (s.v[0] * s.v[19]);
        let eq10_e1050_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq10_e1050_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq10_e1050_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq10_e1050_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq10_e1050_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq10_e1050_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq10_e1050_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq10_e1050_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq10_e1050_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq10_e1050_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq10_e1050_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq10_e1050_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq10_e1050_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq10_e1050_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq10_e1050_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq10_e1050_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq10_e1050_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq10_e1050_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq10_e1050_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq10_e1050_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq10_e1050_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq10_e1050_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq10_e1050_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq10_e1050_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq10_e1050_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq10_e1050_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq10_e1050_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq10_e1050_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq10_e1050_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq10_e1050_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq10_e1050_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq10_e1050_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq10_e1050_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq10_e1050_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq10_e1050_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq10_e1050_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq10_e1050_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq10_e1050_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq10_e1050_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq10_e1050_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq10_e1050_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq10_e1050_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq10_e1050_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq10_e1050_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq10_e1050_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq10_e1050_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq10_e1052: f64 = (eq10_e1050 * p.p32);
        let eq10_e1052_d_n0: f64 = (eq10_e1050_d_n0 * p.p32);
        let eq10_e1052_d_n1: f64 = (eq10_e1050_d_n1 * p.p32);
        let eq10_e1052_d_n2: f64 = (eq10_e1050_d_n2 * p.p32);
        let eq10_e1052_d_n3: f64 = (eq10_e1050_d_n3 * p.p32);
        let eq10_e1052_d_n4: f64 = (eq10_e1050_d_n4 * p.p32);
        let eq10_e1052_d_n5: f64 = (eq10_e1050_d_n5 * p.p32);
        let eq10_e1052_d_n6: f64 = (eq10_e1050_d_n6 * p.p32);
        let eq10_e1052_d_n7: f64 = (eq10_e1050_d_n7 * p.p32);
        let eq10_e1052_d_n8: f64 = (eq10_e1050_d_n8 * p.p32);
        let eq10_e1052_d_n9: f64 = (eq10_e1050_d_n9 * p.p32);
        let eq10_e1052_d_n10: f64 = (eq10_e1050_d_n10 * p.p32);
        let eq10_e1052_d_n11: f64 = (eq10_e1050_d_n11 * p.p32);
        let eq10_e1052_d_n12: f64 = (eq10_e1050_d_n12 * p.p32);
        let eq10_e1052_d_n13: f64 = (eq10_e1050_d_n13 * p.p32);
        let eq10_e1052_d_n14: f64 = (eq10_e1050_d_n14 * p.p32);
        let eq10_e1052_d_n15: f64 = (eq10_e1050_d_n15 * p.p32);
        let eq10_e1052_d_n16: f64 = (eq10_e1050_d_n16 * p.p32);
        let eq10_e1052_d_n17: f64 = (eq10_e1050_d_n17 * p.p32);
        let eq10_e1052_d_n18: f64 = (eq10_e1050_d_n18 * p.p32);
        let eq10_e1052_d_n19: f64 = (eq10_e1050_d_n19 * p.p32);
        let eq10_e1052_d_n20: f64 = (eq10_e1050_d_n20 * p.p32);
        let eq10_e1052_d_b0: f64 = (eq10_e1050_d_b0 * p.p32);
        let eq10_e1052_d_b1: f64 = (eq10_e1050_d_b1 * p.p32);
        let eq10_e1052_d_b2: f64 = (eq10_e1050_d_b2 * p.p32);
        let eq10_e1052_d_b3: f64 = (eq10_e1050_d_b3 * p.p32);
        let eq10_e1052_d_b4: f64 = (eq10_e1050_d_b4 * p.p32);
        let eq10_e1052_d_b5: f64 = (eq10_e1050_d_b5 * p.p32);
        let eq10_e1052_d_b6: f64 = (eq10_e1050_d_b6 * p.p32);
        let eq10_e1052_d_b7: f64 = (eq10_e1050_d_b7 * p.p32);
        let eq10_e1052_d_b8: f64 = (eq10_e1050_d_b8 * p.p32);
        let eq10_e1052_d_b9: f64 = (eq10_e1050_d_b9 * p.p32);
        let eq10_e1052_d_b10: f64 = (eq10_e1050_d_b10 * p.p32);
        let eq10_e1052_d_b11: f64 = (eq10_e1050_d_b11 * p.p32);
        let eq10_e1052_d_b12: f64 = (eq10_e1050_d_b12 * p.p32);
        let eq10_e1052_d_b13: f64 = (eq10_e1050_d_b13 * p.p32);
        let eq10_e1052_d_b14: f64 = (eq10_e1050_d_b14 * p.p32);
        let eq10_e1052_d_b15: f64 = (eq10_e1050_d_b15 * p.p32);
        let eq10_e1052_d_b16: f64 = (eq10_e1050_d_b16 * p.p32);
        let eq10_e1052_d_b17: f64 = (eq10_e1050_d_b17 * p.p32);
        let eq10_e1052_d_b18: f64 = (eq10_e1050_d_b18 * p.p32);
        let eq10_e1052_d_b19: f64 = (eq10_e1050_d_b19 * p.p32);
        let eq10_e1052_d_b20: f64 = (eq10_e1050_d_b20 * p.p32);
        let eq10_e1052_d_b21: f64 = (eq10_e1050_d_b21 * p.p32);
        let eq10_e1052_d_b22: f64 = (eq10_e1050_d_b22 * p.p32);
        let eq10_e1052_d_b23: f64 = (eq10_e1050_d_b23 * p.p32);
        let eq10_e1052_d_b24: f64 = (eq10_e1050_d_b24 * p.p32);
        let eq10_e1054: f64 = (eq10_e1052 * s.v[840]);
        let eq10_e1054_d_n0: f64 = ((eq10_e1052_d_n0 * s.v[840]) + (eq10_e1052 * s.dn[840][0]));
        let eq10_e1054_d_n1: f64 = ((eq10_e1052_d_n1 * s.v[840]) + (eq10_e1052 * s.dn[840][1]));
        let eq10_e1054_d_n2: f64 = ((eq10_e1052_d_n2 * s.v[840]) + (eq10_e1052 * s.dn[840][2]));
        let eq10_e1054_d_n3: f64 = ((eq10_e1052_d_n3 * s.v[840]) + (eq10_e1052 * s.dn[840][3]));
        let eq10_e1054_d_n4: f64 = ((eq10_e1052_d_n4 * s.v[840]) + (eq10_e1052 * s.dn[840][4]));
        let eq10_e1054_d_n5: f64 = ((eq10_e1052_d_n5 * s.v[840]) + (eq10_e1052 * s.dn[840][5]));
        let eq10_e1054_d_n6: f64 = ((eq10_e1052_d_n6 * s.v[840]) + (eq10_e1052 * s.dn[840][6]));
        let eq10_e1054_d_n7: f64 = ((eq10_e1052_d_n7 * s.v[840]) + (eq10_e1052 * s.dn[840][7]));
        let eq10_e1054_d_n8: f64 = ((eq10_e1052_d_n8 * s.v[840]) + (eq10_e1052 * s.dn[840][8]));
        let eq10_e1054_d_n9: f64 = ((eq10_e1052_d_n9 * s.v[840]) + (eq10_e1052 * s.dn[840][9]));
        let eq10_e1054_d_n10: f64 = ((eq10_e1052_d_n10 * s.v[840]) + (eq10_e1052 * s.dn[840][10]));
        let eq10_e1054_d_n11: f64 = ((eq10_e1052_d_n11 * s.v[840]) + (eq10_e1052 * s.dn[840][11]));
        let eq10_e1054_d_n12: f64 = ((eq10_e1052_d_n12 * s.v[840]) + (eq10_e1052 * s.dn[840][12]));
        let eq10_e1054_d_n13: f64 = ((eq10_e1052_d_n13 * s.v[840]) + (eq10_e1052 * s.dn[840][13]));
        let eq10_e1054_d_n14: f64 = ((eq10_e1052_d_n14 * s.v[840]) + (eq10_e1052 * s.dn[840][14]));
        let eq10_e1054_d_n15: f64 = ((eq10_e1052_d_n15 * s.v[840]) + (eq10_e1052 * s.dn[840][15]));
        let eq10_e1054_d_n16: f64 = ((eq10_e1052_d_n16 * s.v[840]) + (eq10_e1052 * s.dn[840][16]));
        let eq10_e1054_d_n17: f64 = ((eq10_e1052_d_n17 * s.v[840]) + (eq10_e1052 * s.dn[840][17]));
        let eq10_e1054_d_n18: f64 = ((eq10_e1052_d_n18 * s.v[840]) + (eq10_e1052 * s.dn[840][18]));
        let eq10_e1054_d_n19: f64 = ((eq10_e1052_d_n19 * s.v[840]) + (eq10_e1052 * s.dn[840][19]));
        let eq10_e1054_d_n20: f64 = ((eq10_e1052_d_n20 * s.v[840]) + (eq10_e1052 * s.dn[840][20]));
        let eq10_e1054_d_b0: f64 = ((eq10_e1052_d_b0 * s.v[840]) + (eq10_e1052 * s.db[840][0]));
        let eq10_e1054_d_b1: f64 = ((eq10_e1052_d_b1 * s.v[840]) + (eq10_e1052 * s.db[840][1]));
        let eq10_e1054_d_b2: f64 = ((eq10_e1052_d_b2 * s.v[840]) + (eq10_e1052 * s.db[840][2]));
        let eq10_e1054_d_b3: f64 = ((eq10_e1052_d_b3 * s.v[840]) + (eq10_e1052 * s.db[840][3]));
        let eq10_e1054_d_b4: f64 = ((eq10_e1052_d_b4 * s.v[840]) + (eq10_e1052 * s.db[840][4]));
        let eq10_e1054_d_b5: f64 = ((eq10_e1052_d_b5 * s.v[840]) + (eq10_e1052 * s.db[840][5]));
        let eq10_e1054_d_b6: f64 = ((eq10_e1052_d_b6 * s.v[840]) + (eq10_e1052 * s.db[840][6]));
        let eq10_e1054_d_b7: f64 = ((eq10_e1052_d_b7 * s.v[840]) + (eq10_e1052 * s.db[840][7]));
        let eq10_e1054_d_b8: f64 = ((eq10_e1052_d_b8 * s.v[840]) + (eq10_e1052 * s.db[840][8]));
        let eq10_e1054_d_b9: f64 = ((eq10_e1052_d_b9 * s.v[840]) + (eq10_e1052 * s.db[840][9]));
        let eq10_e1054_d_b10: f64 = ((eq10_e1052_d_b10 * s.v[840]) + (eq10_e1052 * s.db[840][10]));
        let eq10_e1054_d_b11: f64 = ((eq10_e1052_d_b11 * s.v[840]) + (eq10_e1052 * s.db[840][11]));
        let eq10_e1054_d_b12: f64 = ((eq10_e1052_d_b12 * s.v[840]) + (eq10_e1052 * s.db[840][12]));
        let eq10_e1054_d_b13: f64 = ((eq10_e1052_d_b13 * s.v[840]) + (eq10_e1052 * s.db[840][13]));
        let eq10_e1054_d_b14: f64 = ((eq10_e1052_d_b14 * s.v[840]) + (eq10_e1052 * s.db[840][14]));
        let eq10_e1054_d_b15: f64 = ((eq10_e1052_d_b15 * s.v[840]) + (eq10_e1052 * s.db[840][15]));
        let eq10_e1054_d_b16: f64 = ((eq10_e1052_d_b16 * s.v[840]) + (eq10_e1052 * s.db[840][16]));
        let eq10_e1054_d_b17: f64 = ((eq10_e1052_d_b17 * s.v[840]) + (eq10_e1052 * s.db[840][17]));
        let eq10_e1054_d_b18: f64 = ((eq10_e1052_d_b18 * s.v[840]) + (eq10_e1052 * s.db[840][18]));
        let eq10_e1054_d_b19: f64 = ((eq10_e1052_d_b19 * s.v[840]) + (eq10_e1052 * s.db[840][19]));
        let eq10_e1054_d_b20: f64 = ((eq10_e1052_d_b20 * s.v[840]) + (eq10_e1052 * s.db[840][20]));
        let eq10_e1054_d_b21: f64 = ((eq10_e1052_d_b21 * s.v[840]) + (eq10_e1052 * s.db[840][21]));
        let eq10_e1054_d_b22: f64 = ((eq10_e1052_d_b22 * s.v[840]) + (eq10_e1052 * s.db[840][22]));
        let eq10_e1054_d_b23: f64 = ((eq10_e1052_d_b23 * s.v[840]) + (eq10_e1052 * s.db[840][23]));
        let eq10_e1054_d_b24: f64 = ((eq10_e1052_d_b24 * s.v[840]) + (eq10_e1052 * s.db[840][24]));
        let eq10_value: f64 = eq10_e1054;
        let eq10_node_derivatives: [f64; 21] = [eq10_e1054_d_n0, eq10_e1054_d_n1, eq10_e1054_d_n2, eq10_e1054_d_n3, eq10_e1054_d_n4, eq10_e1054_d_n5, eq10_e1054_d_n6, eq10_e1054_d_n7, eq10_e1054_d_n8, eq10_e1054_d_n9, eq10_e1054_d_n10, eq10_e1054_d_n11, eq10_e1054_d_n12, eq10_e1054_d_n13, eq10_e1054_d_n14, eq10_e1054_d_n15, eq10_e1054_d_n16, eq10_e1054_d_n17, eq10_e1054_d_n18, eq10_e1054_d_n19, eq10_e1054_d_n20];
        let eq10_branch_derivatives: [f64; 25] = [eq10_e1054_d_b0, eq10_e1054_d_b1, eq10_e1054_d_b2, eq10_e1054_d_b3, eq10_e1054_d_b4, eq10_e1054_d_b5, eq10_e1054_d_b6, eq10_e1054_d_b7, eq10_e1054_d_b8, eq10_e1054_d_b9, eq10_e1054_d_b10, eq10_e1054_d_b11, eq10_e1054_d_b12, eq10_e1054_d_b13, eq10_e1054_d_b14, eq10_e1054_d_b15, eq10_e1054_d_b16, eq10_e1054_d_b17, eq10_e1054_d_b18, eq10_e1054_d_b19, eq10_e1054_d_b20, eq10_e1054_d_b21, eq10_e1054_d_b22, eq10_e1054_d_b23, eq10_e1054_d_b24];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e1057: f64 = (s.v[0] * s.v[19]);
        let eq11_e1057_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq11_e1057_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq11_e1057_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq11_e1057_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq11_e1057_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq11_e1057_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq11_e1057_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq11_e1057_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq11_e1057_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq11_e1057_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq11_e1057_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq11_e1057_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq11_e1057_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq11_e1057_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq11_e1057_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq11_e1057_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq11_e1057_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq11_e1057_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq11_e1057_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq11_e1057_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq11_e1057_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq11_e1057_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq11_e1057_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq11_e1057_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq11_e1057_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq11_e1057_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq11_e1057_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq11_e1057_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq11_e1057_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq11_e1057_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq11_e1057_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq11_e1057_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq11_e1057_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq11_e1057_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq11_e1057_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq11_e1057_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq11_e1057_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq11_e1057_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq11_e1057_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq11_e1057_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq11_e1057_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq11_e1057_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq11_e1057_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq11_e1057_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq11_e1057_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq11_e1057_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq11_e1059: f64 = (eq11_e1057 * p.p32);
        let eq11_e1059_d_n0: f64 = (eq11_e1057_d_n0 * p.p32);
        let eq11_e1059_d_n1: f64 = (eq11_e1057_d_n1 * p.p32);
        let eq11_e1059_d_n2: f64 = (eq11_e1057_d_n2 * p.p32);
        let eq11_e1059_d_n3: f64 = (eq11_e1057_d_n3 * p.p32);
        let eq11_e1059_d_n4: f64 = (eq11_e1057_d_n4 * p.p32);
        let eq11_e1059_d_n5: f64 = (eq11_e1057_d_n5 * p.p32);
        let eq11_e1059_d_n6: f64 = (eq11_e1057_d_n6 * p.p32);
        let eq11_e1059_d_n7: f64 = (eq11_e1057_d_n7 * p.p32);
        let eq11_e1059_d_n8: f64 = (eq11_e1057_d_n8 * p.p32);
        let eq11_e1059_d_n9: f64 = (eq11_e1057_d_n9 * p.p32);
        let eq11_e1059_d_n10: f64 = (eq11_e1057_d_n10 * p.p32);
        let eq11_e1059_d_n11: f64 = (eq11_e1057_d_n11 * p.p32);
        let eq11_e1059_d_n12: f64 = (eq11_e1057_d_n12 * p.p32);
        let eq11_e1059_d_n13: f64 = (eq11_e1057_d_n13 * p.p32);
        let eq11_e1059_d_n14: f64 = (eq11_e1057_d_n14 * p.p32);
        let eq11_e1059_d_n15: f64 = (eq11_e1057_d_n15 * p.p32);
        let eq11_e1059_d_n16: f64 = (eq11_e1057_d_n16 * p.p32);
        let eq11_e1059_d_n17: f64 = (eq11_e1057_d_n17 * p.p32);
        let eq11_e1059_d_n18: f64 = (eq11_e1057_d_n18 * p.p32);
        let eq11_e1059_d_n19: f64 = (eq11_e1057_d_n19 * p.p32);
        let eq11_e1059_d_n20: f64 = (eq11_e1057_d_n20 * p.p32);
        let eq11_e1059_d_b0: f64 = (eq11_e1057_d_b0 * p.p32);
        let eq11_e1059_d_b1: f64 = (eq11_e1057_d_b1 * p.p32);
        let eq11_e1059_d_b2: f64 = (eq11_e1057_d_b2 * p.p32);
        let eq11_e1059_d_b3: f64 = (eq11_e1057_d_b3 * p.p32);
        let eq11_e1059_d_b4: f64 = (eq11_e1057_d_b4 * p.p32);
        let eq11_e1059_d_b5: f64 = (eq11_e1057_d_b5 * p.p32);
        let eq11_e1059_d_b6: f64 = (eq11_e1057_d_b6 * p.p32);
        let eq11_e1059_d_b7: f64 = (eq11_e1057_d_b7 * p.p32);
        let eq11_e1059_d_b8: f64 = (eq11_e1057_d_b8 * p.p32);
        let eq11_e1059_d_b9: f64 = (eq11_e1057_d_b9 * p.p32);
        let eq11_e1059_d_b10: f64 = (eq11_e1057_d_b10 * p.p32);
        let eq11_e1059_d_b11: f64 = (eq11_e1057_d_b11 * p.p32);
        let eq11_e1059_d_b12: f64 = (eq11_e1057_d_b12 * p.p32);
        let eq11_e1059_d_b13: f64 = (eq11_e1057_d_b13 * p.p32);
        let eq11_e1059_d_b14: f64 = (eq11_e1057_d_b14 * p.p32);
        let eq11_e1059_d_b15: f64 = (eq11_e1057_d_b15 * p.p32);
        let eq11_e1059_d_b16: f64 = (eq11_e1057_d_b16 * p.p32);
        let eq11_e1059_d_b17: f64 = (eq11_e1057_d_b17 * p.p32);
        let eq11_e1059_d_b18: f64 = (eq11_e1057_d_b18 * p.p32);
        let eq11_e1059_d_b19: f64 = (eq11_e1057_d_b19 * p.p32);
        let eq11_e1059_d_b20: f64 = (eq11_e1057_d_b20 * p.p32);
        let eq11_e1059_d_b21: f64 = (eq11_e1057_d_b21 * p.p32);
        let eq11_e1059_d_b22: f64 = (eq11_e1057_d_b22 * p.p32);
        let eq11_e1059_d_b23: f64 = (eq11_e1057_d_b23 * p.p32);
        let eq11_e1059_d_b24: f64 = (eq11_e1057_d_b24 * p.p32);
        let eq11_e1061: f64 = (eq11_e1059 * s.v[844]);
        let eq11_e1061_d_n0: f64 = ((eq11_e1059_d_n0 * s.v[844]) + (eq11_e1059 * s.dn[844][0]));
        let eq11_e1061_d_n1: f64 = ((eq11_e1059_d_n1 * s.v[844]) + (eq11_e1059 * s.dn[844][1]));
        let eq11_e1061_d_n2: f64 = ((eq11_e1059_d_n2 * s.v[844]) + (eq11_e1059 * s.dn[844][2]));
        let eq11_e1061_d_n3: f64 = ((eq11_e1059_d_n3 * s.v[844]) + (eq11_e1059 * s.dn[844][3]));
        let eq11_e1061_d_n4: f64 = ((eq11_e1059_d_n4 * s.v[844]) + (eq11_e1059 * s.dn[844][4]));
        let eq11_e1061_d_n5: f64 = ((eq11_e1059_d_n5 * s.v[844]) + (eq11_e1059 * s.dn[844][5]));
        let eq11_e1061_d_n6: f64 = ((eq11_e1059_d_n6 * s.v[844]) + (eq11_e1059 * s.dn[844][6]));
        let eq11_e1061_d_n7: f64 = ((eq11_e1059_d_n7 * s.v[844]) + (eq11_e1059 * s.dn[844][7]));
        let eq11_e1061_d_n8: f64 = ((eq11_e1059_d_n8 * s.v[844]) + (eq11_e1059 * s.dn[844][8]));
        let eq11_e1061_d_n9: f64 = ((eq11_e1059_d_n9 * s.v[844]) + (eq11_e1059 * s.dn[844][9]));
        let eq11_e1061_d_n10: f64 = ((eq11_e1059_d_n10 * s.v[844]) + (eq11_e1059 * s.dn[844][10]));
        let eq11_e1061_d_n11: f64 = ((eq11_e1059_d_n11 * s.v[844]) + (eq11_e1059 * s.dn[844][11]));
        let eq11_e1061_d_n12: f64 = ((eq11_e1059_d_n12 * s.v[844]) + (eq11_e1059 * s.dn[844][12]));
        let eq11_e1061_d_n13: f64 = ((eq11_e1059_d_n13 * s.v[844]) + (eq11_e1059 * s.dn[844][13]));
        let eq11_e1061_d_n14: f64 = ((eq11_e1059_d_n14 * s.v[844]) + (eq11_e1059 * s.dn[844][14]));
        let eq11_e1061_d_n15: f64 = ((eq11_e1059_d_n15 * s.v[844]) + (eq11_e1059 * s.dn[844][15]));
        let eq11_e1061_d_n16: f64 = ((eq11_e1059_d_n16 * s.v[844]) + (eq11_e1059 * s.dn[844][16]));
        let eq11_e1061_d_n17: f64 = ((eq11_e1059_d_n17 * s.v[844]) + (eq11_e1059 * s.dn[844][17]));
        let eq11_e1061_d_n18: f64 = ((eq11_e1059_d_n18 * s.v[844]) + (eq11_e1059 * s.dn[844][18]));
        let eq11_e1061_d_n19: f64 = ((eq11_e1059_d_n19 * s.v[844]) + (eq11_e1059 * s.dn[844][19]));
        let eq11_e1061_d_n20: f64 = ((eq11_e1059_d_n20 * s.v[844]) + (eq11_e1059 * s.dn[844][20]));
        let eq11_e1061_d_b0: f64 = ((eq11_e1059_d_b0 * s.v[844]) + (eq11_e1059 * s.db[844][0]));
        let eq11_e1061_d_b1: f64 = ((eq11_e1059_d_b1 * s.v[844]) + (eq11_e1059 * s.db[844][1]));
        let eq11_e1061_d_b2: f64 = ((eq11_e1059_d_b2 * s.v[844]) + (eq11_e1059 * s.db[844][2]));
        let eq11_e1061_d_b3: f64 = ((eq11_e1059_d_b3 * s.v[844]) + (eq11_e1059 * s.db[844][3]));
        let eq11_e1061_d_b4: f64 = ((eq11_e1059_d_b4 * s.v[844]) + (eq11_e1059 * s.db[844][4]));
        let eq11_e1061_d_b5: f64 = ((eq11_e1059_d_b5 * s.v[844]) + (eq11_e1059 * s.db[844][5]));
        let eq11_e1061_d_b6: f64 = ((eq11_e1059_d_b6 * s.v[844]) + (eq11_e1059 * s.db[844][6]));
        let eq11_e1061_d_b7: f64 = ((eq11_e1059_d_b7 * s.v[844]) + (eq11_e1059 * s.db[844][7]));
        let eq11_e1061_d_b8: f64 = ((eq11_e1059_d_b8 * s.v[844]) + (eq11_e1059 * s.db[844][8]));
        let eq11_e1061_d_b9: f64 = ((eq11_e1059_d_b9 * s.v[844]) + (eq11_e1059 * s.db[844][9]));
        let eq11_e1061_d_b10: f64 = ((eq11_e1059_d_b10 * s.v[844]) + (eq11_e1059 * s.db[844][10]));
        let eq11_e1061_d_b11: f64 = ((eq11_e1059_d_b11 * s.v[844]) + (eq11_e1059 * s.db[844][11]));
        let eq11_e1061_d_b12: f64 = ((eq11_e1059_d_b12 * s.v[844]) + (eq11_e1059 * s.db[844][12]));
        let eq11_e1061_d_b13: f64 = ((eq11_e1059_d_b13 * s.v[844]) + (eq11_e1059 * s.db[844][13]));
        let eq11_e1061_d_b14: f64 = ((eq11_e1059_d_b14 * s.v[844]) + (eq11_e1059 * s.db[844][14]));
        let eq11_e1061_d_b15: f64 = ((eq11_e1059_d_b15 * s.v[844]) + (eq11_e1059 * s.db[844][15]));
        let eq11_e1061_d_b16: f64 = ((eq11_e1059_d_b16 * s.v[844]) + (eq11_e1059 * s.db[844][16]));
        let eq11_e1061_d_b17: f64 = ((eq11_e1059_d_b17 * s.v[844]) + (eq11_e1059 * s.db[844][17]));
        let eq11_e1061_d_b18: f64 = ((eq11_e1059_d_b18 * s.v[844]) + (eq11_e1059 * s.db[844][18]));
        let eq11_e1061_d_b19: f64 = ((eq11_e1059_d_b19 * s.v[844]) + (eq11_e1059 * s.db[844][19]));
        let eq11_e1061_d_b20: f64 = ((eq11_e1059_d_b20 * s.v[844]) + (eq11_e1059 * s.db[844][20]));
        let eq11_e1061_d_b21: f64 = ((eq11_e1059_d_b21 * s.v[844]) + (eq11_e1059 * s.db[844][21]));
        let eq11_e1061_d_b22: f64 = ((eq11_e1059_d_b22 * s.v[844]) + (eq11_e1059 * s.db[844][22]));
        let eq11_e1061_d_b23: f64 = ((eq11_e1059_d_b23 * s.v[844]) + (eq11_e1059 * s.db[844][23]));
        let eq11_e1061_d_b24: f64 = ((eq11_e1059_d_b24 * s.v[844]) + (eq11_e1059 * s.db[844][24]));
        let eq11_value: f64 = eq11_e1061;
        let eq11_node_derivatives: [f64; 21] = [eq11_e1061_d_n0, eq11_e1061_d_n1, eq11_e1061_d_n2, eq11_e1061_d_n3, eq11_e1061_d_n4, eq11_e1061_d_n5, eq11_e1061_d_n6, eq11_e1061_d_n7, eq11_e1061_d_n8, eq11_e1061_d_n9, eq11_e1061_d_n10, eq11_e1061_d_n11, eq11_e1061_d_n12, eq11_e1061_d_n13, eq11_e1061_d_n14, eq11_e1061_d_n15, eq11_e1061_d_n16, eq11_e1061_d_n17, eq11_e1061_d_n18, eq11_e1061_d_n19, eq11_e1061_d_n20];
        let eq11_branch_derivatives: [f64; 25] = [eq11_e1061_d_b0, eq11_e1061_d_b1, eq11_e1061_d_b2, eq11_e1061_d_b3, eq11_e1061_d_b4, eq11_e1061_d_b5, eq11_e1061_d_b6, eq11_e1061_d_b7, eq11_e1061_d_b8, eq11_e1061_d_b9, eq11_e1061_d_b10, eq11_e1061_d_b11, eq11_e1061_d_b12, eq11_e1061_d_b13, eq11_e1061_d_b14, eq11_e1061_d_b15, eq11_e1061_d_b16, eq11_e1061_d_b17, eq11_e1061_d_b18, eq11_e1061_d_b19, eq11_e1061_d_b20, eq11_e1061_d_b21, eq11_e1061_d_b22, eq11_e1061_d_b23, eq11_e1061_d_b24];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e1064: f64 = (s.v[0] * s.v[19]);
        let eq12_e1064_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq12_e1064_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq12_e1064_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq12_e1064_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq12_e1064_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq12_e1064_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq12_e1064_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq12_e1064_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq12_e1064_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq12_e1064_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq12_e1064_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq12_e1064_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq12_e1064_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq12_e1064_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq12_e1064_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq12_e1064_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq12_e1064_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq12_e1064_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq12_e1064_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq12_e1064_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq12_e1064_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq12_e1064_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq12_e1064_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq12_e1064_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq12_e1064_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq12_e1064_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq12_e1064_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq12_e1064_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq12_e1064_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq12_e1064_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq12_e1064_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq12_e1064_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq12_e1064_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq12_e1064_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq12_e1064_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq12_e1064_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq12_e1064_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq12_e1064_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq12_e1064_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq12_e1064_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq12_e1064_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq12_e1064_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq12_e1064_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq12_e1064_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq12_e1064_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq12_e1064_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq12_e1066: f64 = (eq12_e1064 * p.p32);
        let eq12_e1066_d_n0: f64 = (eq12_e1064_d_n0 * p.p32);
        let eq12_e1066_d_n1: f64 = (eq12_e1064_d_n1 * p.p32);
        let eq12_e1066_d_n2: f64 = (eq12_e1064_d_n2 * p.p32);
        let eq12_e1066_d_n3: f64 = (eq12_e1064_d_n3 * p.p32);
        let eq12_e1066_d_n4: f64 = (eq12_e1064_d_n4 * p.p32);
        let eq12_e1066_d_n5: f64 = (eq12_e1064_d_n5 * p.p32);
        let eq12_e1066_d_n6: f64 = (eq12_e1064_d_n6 * p.p32);
        let eq12_e1066_d_n7: f64 = (eq12_e1064_d_n7 * p.p32);
        let eq12_e1066_d_n8: f64 = (eq12_e1064_d_n8 * p.p32);
        let eq12_e1066_d_n9: f64 = (eq12_e1064_d_n9 * p.p32);
        let eq12_e1066_d_n10: f64 = (eq12_e1064_d_n10 * p.p32);
        let eq12_e1066_d_n11: f64 = (eq12_e1064_d_n11 * p.p32);
        let eq12_e1066_d_n12: f64 = (eq12_e1064_d_n12 * p.p32);
        let eq12_e1066_d_n13: f64 = (eq12_e1064_d_n13 * p.p32);
        let eq12_e1066_d_n14: f64 = (eq12_e1064_d_n14 * p.p32);
        let eq12_e1066_d_n15: f64 = (eq12_e1064_d_n15 * p.p32);
        let eq12_e1066_d_n16: f64 = (eq12_e1064_d_n16 * p.p32);
        let eq12_e1066_d_n17: f64 = (eq12_e1064_d_n17 * p.p32);
        let eq12_e1066_d_n18: f64 = (eq12_e1064_d_n18 * p.p32);
        let eq12_e1066_d_n19: f64 = (eq12_e1064_d_n19 * p.p32);
        let eq12_e1066_d_n20: f64 = (eq12_e1064_d_n20 * p.p32);
        let eq12_e1066_d_b0: f64 = (eq12_e1064_d_b0 * p.p32);
        let eq12_e1066_d_b1: f64 = (eq12_e1064_d_b1 * p.p32);
        let eq12_e1066_d_b2: f64 = (eq12_e1064_d_b2 * p.p32);
        let eq12_e1066_d_b3: f64 = (eq12_e1064_d_b3 * p.p32);
        let eq12_e1066_d_b4: f64 = (eq12_e1064_d_b4 * p.p32);
        let eq12_e1066_d_b5: f64 = (eq12_e1064_d_b5 * p.p32);
        let eq12_e1066_d_b6: f64 = (eq12_e1064_d_b6 * p.p32);
        let eq12_e1066_d_b7: f64 = (eq12_e1064_d_b7 * p.p32);
        let eq12_e1066_d_b8: f64 = (eq12_e1064_d_b8 * p.p32);
        let eq12_e1066_d_b9: f64 = (eq12_e1064_d_b9 * p.p32);
        let eq12_e1066_d_b10: f64 = (eq12_e1064_d_b10 * p.p32);
        let eq12_e1066_d_b11: f64 = (eq12_e1064_d_b11 * p.p32);
        let eq12_e1066_d_b12: f64 = (eq12_e1064_d_b12 * p.p32);
        let eq12_e1066_d_b13: f64 = (eq12_e1064_d_b13 * p.p32);
        let eq12_e1066_d_b14: f64 = (eq12_e1064_d_b14 * p.p32);
        let eq12_e1066_d_b15: f64 = (eq12_e1064_d_b15 * p.p32);
        let eq12_e1066_d_b16: f64 = (eq12_e1064_d_b16 * p.p32);
        let eq12_e1066_d_b17: f64 = (eq12_e1064_d_b17 * p.p32);
        let eq12_e1066_d_b18: f64 = (eq12_e1064_d_b18 * p.p32);
        let eq12_e1066_d_b19: f64 = (eq12_e1064_d_b19 * p.p32);
        let eq12_e1066_d_b20: f64 = (eq12_e1064_d_b20 * p.p32);
        let eq12_e1066_d_b21: f64 = (eq12_e1064_d_b21 * p.p32);
        let eq12_e1066_d_b22: f64 = (eq12_e1064_d_b22 * p.p32);
        let eq12_e1066_d_b23: f64 = (eq12_e1064_d_b23 * p.p32);
        let eq12_e1066_d_b24: f64 = (eq12_e1064_d_b24 * p.p32);
        let eq12_e1068: f64 = (eq12_e1066 * s.v[845]);
        let eq12_e1068_d_n0: f64 = ((eq12_e1066_d_n0 * s.v[845]) + (eq12_e1066 * s.dn[845][0]));
        let eq12_e1068_d_n1: f64 = ((eq12_e1066_d_n1 * s.v[845]) + (eq12_e1066 * s.dn[845][1]));
        let eq12_e1068_d_n2: f64 = ((eq12_e1066_d_n2 * s.v[845]) + (eq12_e1066 * s.dn[845][2]));
        let eq12_e1068_d_n3: f64 = ((eq12_e1066_d_n3 * s.v[845]) + (eq12_e1066 * s.dn[845][3]));
        let eq12_e1068_d_n4: f64 = ((eq12_e1066_d_n4 * s.v[845]) + (eq12_e1066 * s.dn[845][4]));
        let eq12_e1068_d_n5: f64 = ((eq12_e1066_d_n5 * s.v[845]) + (eq12_e1066 * s.dn[845][5]));
        let eq12_e1068_d_n6: f64 = ((eq12_e1066_d_n6 * s.v[845]) + (eq12_e1066 * s.dn[845][6]));
        let eq12_e1068_d_n7: f64 = ((eq12_e1066_d_n7 * s.v[845]) + (eq12_e1066 * s.dn[845][7]));
        let eq12_e1068_d_n8: f64 = ((eq12_e1066_d_n8 * s.v[845]) + (eq12_e1066 * s.dn[845][8]));
        let eq12_e1068_d_n9: f64 = ((eq12_e1066_d_n9 * s.v[845]) + (eq12_e1066 * s.dn[845][9]));
        let eq12_e1068_d_n10: f64 = ((eq12_e1066_d_n10 * s.v[845]) + (eq12_e1066 * s.dn[845][10]));
        let eq12_e1068_d_n11: f64 = ((eq12_e1066_d_n11 * s.v[845]) + (eq12_e1066 * s.dn[845][11]));
        let eq12_e1068_d_n12: f64 = ((eq12_e1066_d_n12 * s.v[845]) + (eq12_e1066 * s.dn[845][12]));
        let eq12_e1068_d_n13: f64 = ((eq12_e1066_d_n13 * s.v[845]) + (eq12_e1066 * s.dn[845][13]));
        let eq12_e1068_d_n14: f64 = ((eq12_e1066_d_n14 * s.v[845]) + (eq12_e1066 * s.dn[845][14]));
        let eq12_e1068_d_n15: f64 = ((eq12_e1066_d_n15 * s.v[845]) + (eq12_e1066 * s.dn[845][15]));
        let eq12_e1068_d_n16: f64 = ((eq12_e1066_d_n16 * s.v[845]) + (eq12_e1066 * s.dn[845][16]));
        let eq12_e1068_d_n17: f64 = ((eq12_e1066_d_n17 * s.v[845]) + (eq12_e1066 * s.dn[845][17]));
        let eq12_e1068_d_n18: f64 = ((eq12_e1066_d_n18 * s.v[845]) + (eq12_e1066 * s.dn[845][18]));
        let eq12_e1068_d_n19: f64 = ((eq12_e1066_d_n19 * s.v[845]) + (eq12_e1066 * s.dn[845][19]));
        let eq12_e1068_d_n20: f64 = ((eq12_e1066_d_n20 * s.v[845]) + (eq12_e1066 * s.dn[845][20]));
        let eq12_e1068_d_b0: f64 = ((eq12_e1066_d_b0 * s.v[845]) + (eq12_e1066 * s.db[845][0]));
        let eq12_e1068_d_b1: f64 = ((eq12_e1066_d_b1 * s.v[845]) + (eq12_e1066 * s.db[845][1]));
        let eq12_e1068_d_b2: f64 = ((eq12_e1066_d_b2 * s.v[845]) + (eq12_e1066 * s.db[845][2]));
        let eq12_e1068_d_b3: f64 = ((eq12_e1066_d_b3 * s.v[845]) + (eq12_e1066 * s.db[845][3]));
        let eq12_e1068_d_b4: f64 = ((eq12_e1066_d_b4 * s.v[845]) + (eq12_e1066 * s.db[845][4]));
        let eq12_e1068_d_b5: f64 = ((eq12_e1066_d_b5 * s.v[845]) + (eq12_e1066 * s.db[845][5]));
        let eq12_e1068_d_b6: f64 = ((eq12_e1066_d_b6 * s.v[845]) + (eq12_e1066 * s.db[845][6]));
        let eq12_e1068_d_b7: f64 = ((eq12_e1066_d_b7 * s.v[845]) + (eq12_e1066 * s.db[845][7]));
        let eq12_e1068_d_b8: f64 = ((eq12_e1066_d_b8 * s.v[845]) + (eq12_e1066 * s.db[845][8]));
        let eq12_e1068_d_b9: f64 = ((eq12_e1066_d_b9 * s.v[845]) + (eq12_e1066 * s.db[845][9]));
        let eq12_e1068_d_b10: f64 = ((eq12_e1066_d_b10 * s.v[845]) + (eq12_e1066 * s.db[845][10]));
        let eq12_e1068_d_b11: f64 = ((eq12_e1066_d_b11 * s.v[845]) + (eq12_e1066 * s.db[845][11]));
        let eq12_e1068_d_b12: f64 = ((eq12_e1066_d_b12 * s.v[845]) + (eq12_e1066 * s.db[845][12]));
        let eq12_e1068_d_b13: f64 = ((eq12_e1066_d_b13 * s.v[845]) + (eq12_e1066 * s.db[845][13]));
        let eq12_e1068_d_b14: f64 = ((eq12_e1066_d_b14 * s.v[845]) + (eq12_e1066 * s.db[845][14]));
        let eq12_e1068_d_b15: f64 = ((eq12_e1066_d_b15 * s.v[845]) + (eq12_e1066 * s.db[845][15]));
        let eq12_e1068_d_b16: f64 = ((eq12_e1066_d_b16 * s.v[845]) + (eq12_e1066 * s.db[845][16]));
        let eq12_e1068_d_b17: f64 = ((eq12_e1066_d_b17 * s.v[845]) + (eq12_e1066 * s.db[845][17]));
        let eq12_e1068_d_b18: f64 = ((eq12_e1066_d_b18 * s.v[845]) + (eq12_e1066 * s.db[845][18]));
        let eq12_e1068_d_b19: f64 = ((eq12_e1066_d_b19 * s.v[845]) + (eq12_e1066 * s.db[845][19]));
        let eq12_e1068_d_b20: f64 = ((eq12_e1066_d_b20 * s.v[845]) + (eq12_e1066 * s.db[845][20]));
        let eq12_e1068_d_b21: f64 = ((eq12_e1066_d_b21 * s.v[845]) + (eq12_e1066 * s.db[845][21]));
        let eq12_e1068_d_b22: f64 = ((eq12_e1066_d_b22 * s.v[845]) + (eq12_e1066 * s.db[845][22]));
        let eq12_e1068_d_b23: f64 = ((eq12_e1066_d_b23 * s.v[845]) + (eq12_e1066 * s.db[845][23]));
        let eq12_e1068_d_b24: f64 = ((eq12_e1066_d_b24 * s.v[845]) + (eq12_e1066 * s.db[845][24]));
        let eq12_value: f64 = eq12_e1068;
        let eq12_node_derivatives: [f64; 21] = [eq12_e1068_d_n0, eq12_e1068_d_n1, eq12_e1068_d_n2, eq12_e1068_d_n3, eq12_e1068_d_n4, eq12_e1068_d_n5, eq12_e1068_d_n6, eq12_e1068_d_n7, eq12_e1068_d_n8, eq12_e1068_d_n9, eq12_e1068_d_n10, eq12_e1068_d_n11, eq12_e1068_d_n12, eq12_e1068_d_n13, eq12_e1068_d_n14, eq12_e1068_d_n15, eq12_e1068_d_n16, eq12_e1068_d_n17, eq12_e1068_d_n18, eq12_e1068_d_n19, eq12_e1068_d_n20];
        let eq12_branch_derivatives: [f64; 25] = [eq12_e1068_d_b0, eq12_e1068_d_b1, eq12_e1068_d_b2, eq12_e1068_d_b3, eq12_e1068_d_b4, eq12_e1068_d_b5, eq12_e1068_d_b6, eq12_e1068_d_b7, eq12_e1068_d_b8, eq12_e1068_d_b9, eq12_e1068_d_b10, eq12_e1068_d_b11, eq12_e1068_d_b12, eq12_e1068_d_b13, eq12_e1068_d_b14, eq12_e1068_d_b15, eq12_e1068_d_b16, eq12_e1068_d_b17, eq12_e1068_d_b18, eq12_e1068_d_b19, eq12_e1068_d_b20, eq12_e1068_d_b21, eq12_e1068_d_b22, eq12_e1068_d_b23, eq12_e1068_d_b24];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_5(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq13_e1071: f64 = (s.v[0] * s.v[19]);
        let eq13_e1071_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq13_e1071_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq13_e1071_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq13_e1071_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq13_e1071_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq13_e1071_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq13_e1071_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq13_e1071_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq13_e1071_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq13_e1071_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq13_e1071_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq13_e1071_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq13_e1071_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq13_e1071_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq13_e1071_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq13_e1071_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq13_e1071_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq13_e1071_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq13_e1071_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq13_e1071_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq13_e1071_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq13_e1071_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq13_e1071_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq13_e1071_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq13_e1071_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq13_e1071_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq13_e1071_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq13_e1071_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq13_e1071_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq13_e1071_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq13_e1071_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq13_e1071_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq13_e1071_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq13_e1071_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq13_e1071_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq13_e1071_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq13_e1071_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq13_e1071_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq13_e1071_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq13_e1071_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq13_e1071_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq13_e1071_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq13_e1071_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq13_e1071_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq13_e1071_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq13_e1071_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq13_e1073: f64 = (eq13_e1071 * p.p32);
        let eq13_e1073_d_n0: f64 = (eq13_e1071_d_n0 * p.p32);
        let eq13_e1073_d_n1: f64 = (eq13_e1071_d_n1 * p.p32);
        let eq13_e1073_d_n2: f64 = (eq13_e1071_d_n2 * p.p32);
        let eq13_e1073_d_n3: f64 = (eq13_e1071_d_n3 * p.p32);
        let eq13_e1073_d_n4: f64 = (eq13_e1071_d_n4 * p.p32);
        let eq13_e1073_d_n5: f64 = (eq13_e1071_d_n5 * p.p32);
        let eq13_e1073_d_n6: f64 = (eq13_e1071_d_n6 * p.p32);
        let eq13_e1073_d_n7: f64 = (eq13_e1071_d_n7 * p.p32);
        let eq13_e1073_d_n8: f64 = (eq13_e1071_d_n8 * p.p32);
        let eq13_e1073_d_n9: f64 = (eq13_e1071_d_n9 * p.p32);
        let eq13_e1073_d_n10: f64 = (eq13_e1071_d_n10 * p.p32);
        let eq13_e1073_d_n11: f64 = (eq13_e1071_d_n11 * p.p32);
        let eq13_e1073_d_n12: f64 = (eq13_e1071_d_n12 * p.p32);
        let eq13_e1073_d_n13: f64 = (eq13_e1071_d_n13 * p.p32);
        let eq13_e1073_d_n14: f64 = (eq13_e1071_d_n14 * p.p32);
        let eq13_e1073_d_n15: f64 = (eq13_e1071_d_n15 * p.p32);
        let eq13_e1073_d_n16: f64 = (eq13_e1071_d_n16 * p.p32);
        let eq13_e1073_d_n17: f64 = (eq13_e1071_d_n17 * p.p32);
        let eq13_e1073_d_n18: f64 = (eq13_e1071_d_n18 * p.p32);
        let eq13_e1073_d_n19: f64 = (eq13_e1071_d_n19 * p.p32);
        let eq13_e1073_d_n20: f64 = (eq13_e1071_d_n20 * p.p32);
        let eq13_e1073_d_b0: f64 = (eq13_e1071_d_b0 * p.p32);
        let eq13_e1073_d_b1: f64 = (eq13_e1071_d_b1 * p.p32);
        let eq13_e1073_d_b2: f64 = (eq13_e1071_d_b2 * p.p32);
        let eq13_e1073_d_b3: f64 = (eq13_e1071_d_b3 * p.p32);
        let eq13_e1073_d_b4: f64 = (eq13_e1071_d_b4 * p.p32);
        let eq13_e1073_d_b5: f64 = (eq13_e1071_d_b5 * p.p32);
        let eq13_e1073_d_b6: f64 = (eq13_e1071_d_b6 * p.p32);
        let eq13_e1073_d_b7: f64 = (eq13_e1071_d_b7 * p.p32);
        let eq13_e1073_d_b8: f64 = (eq13_e1071_d_b8 * p.p32);
        let eq13_e1073_d_b9: f64 = (eq13_e1071_d_b9 * p.p32);
        let eq13_e1073_d_b10: f64 = (eq13_e1071_d_b10 * p.p32);
        let eq13_e1073_d_b11: f64 = (eq13_e1071_d_b11 * p.p32);
        let eq13_e1073_d_b12: f64 = (eq13_e1071_d_b12 * p.p32);
        let eq13_e1073_d_b13: f64 = (eq13_e1071_d_b13 * p.p32);
        let eq13_e1073_d_b14: f64 = (eq13_e1071_d_b14 * p.p32);
        let eq13_e1073_d_b15: f64 = (eq13_e1071_d_b15 * p.p32);
        let eq13_e1073_d_b16: f64 = (eq13_e1071_d_b16 * p.p32);
        let eq13_e1073_d_b17: f64 = (eq13_e1071_d_b17 * p.p32);
        let eq13_e1073_d_b18: f64 = (eq13_e1071_d_b18 * p.p32);
        let eq13_e1073_d_b19: f64 = (eq13_e1071_d_b19 * p.p32);
        let eq13_e1073_d_b20: f64 = (eq13_e1071_d_b20 * p.p32);
        let eq13_e1073_d_b21: f64 = (eq13_e1071_d_b21 * p.p32);
        let eq13_e1073_d_b22: f64 = (eq13_e1071_d_b22 * p.p32);
        let eq13_e1073_d_b23: f64 = (eq13_e1071_d_b23 * p.p32);
        let eq13_e1073_d_b24: f64 = (eq13_e1071_d_b24 * p.p32);
        let eq13_e1075: f64 = (eq13_e1073 * s.v[848]);
        let eq13_e1075_d_n0: f64 = ((eq13_e1073_d_n0 * s.v[848]) + (eq13_e1073 * s.dn[848][0]));
        let eq13_e1075_d_n1: f64 = ((eq13_e1073_d_n1 * s.v[848]) + (eq13_e1073 * s.dn[848][1]));
        let eq13_e1075_d_n2: f64 = ((eq13_e1073_d_n2 * s.v[848]) + (eq13_e1073 * s.dn[848][2]));
        let eq13_e1075_d_n3: f64 = ((eq13_e1073_d_n3 * s.v[848]) + (eq13_e1073 * s.dn[848][3]));
        let eq13_e1075_d_n4: f64 = ((eq13_e1073_d_n4 * s.v[848]) + (eq13_e1073 * s.dn[848][4]));
        let eq13_e1075_d_n5: f64 = ((eq13_e1073_d_n5 * s.v[848]) + (eq13_e1073 * s.dn[848][5]));
        let eq13_e1075_d_n6: f64 = ((eq13_e1073_d_n6 * s.v[848]) + (eq13_e1073 * s.dn[848][6]));
        let eq13_e1075_d_n7: f64 = ((eq13_e1073_d_n7 * s.v[848]) + (eq13_e1073 * s.dn[848][7]));
        let eq13_e1075_d_n8: f64 = ((eq13_e1073_d_n8 * s.v[848]) + (eq13_e1073 * s.dn[848][8]));
        let eq13_e1075_d_n9: f64 = ((eq13_e1073_d_n9 * s.v[848]) + (eq13_e1073 * s.dn[848][9]));
        let eq13_e1075_d_n10: f64 = ((eq13_e1073_d_n10 * s.v[848]) + (eq13_e1073 * s.dn[848][10]));
        let eq13_e1075_d_n11: f64 = ((eq13_e1073_d_n11 * s.v[848]) + (eq13_e1073 * s.dn[848][11]));
        let eq13_e1075_d_n12: f64 = ((eq13_e1073_d_n12 * s.v[848]) + (eq13_e1073 * s.dn[848][12]));
        let eq13_e1075_d_n13: f64 = ((eq13_e1073_d_n13 * s.v[848]) + (eq13_e1073 * s.dn[848][13]));
        let eq13_e1075_d_n14: f64 = ((eq13_e1073_d_n14 * s.v[848]) + (eq13_e1073 * s.dn[848][14]));
        let eq13_e1075_d_n15: f64 = ((eq13_e1073_d_n15 * s.v[848]) + (eq13_e1073 * s.dn[848][15]));
        let eq13_e1075_d_n16: f64 = ((eq13_e1073_d_n16 * s.v[848]) + (eq13_e1073 * s.dn[848][16]));
        let eq13_e1075_d_n17: f64 = ((eq13_e1073_d_n17 * s.v[848]) + (eq13_e1073 * s.dn[848][17]));
        let eq13_e1075_d_n18: f64 = ((eq13_e1073_d_n18 * s.v[848]) + (eq13_e1073 * s.dn[848][18]));
        let eq13_e1075_d_n19: f64 = ((eq13_e1073_d_n19 * s.v[848]) + (eq13_e1073 * s.dn[848][19]));
        let eq13_e1075_d_n20: f64 = ((eq13_e1073_d_n20 * s.v[848]) + (eq13_e1073 * s.dn[848][20]));
        let eq13_e1075_d_b0: f64 = ((eq13_e1073_d_b0 * s.v[848]) + (eq13_e1073 * s.db[848][0]));
        let eq13_e1075_d_b1: f64 = ((eq13_e1073_d_b1 * s.v[848]) + (eq13_e1073 * s.db[848][1]));
        let eq13_e1075_d_b2: f64 = ((eq13_e1073_d_b2 * s.v[848]) + (eq13_e1073 * s.db[848][2]));
        let eq13_e1075_d_b3: f64 = ((eq13_e1073_d_b3 * s.v[848]) + (eq13_e1073 * s.db[848][3]));
        let eq13_e1075_d_b4: f64 = ((eq13_e1073_d_b4 * s.v[848]) + (eq13_e1073 * s.db[848][4]));
        let eq13_e1075_d_b5: f64 = ((eq13_e1073_d_b5 * s.v[848]) + (eq13_e1073 * s.db[848][5]));
        let eq13_e1075_d_b6: f64 = ((eq13_e1073_d_b6 * s.v[848]) + (eq13_e1073 * s.db[848][6]));
        let eq13_e1075_d_b7: f64 = ((eq13_e1073_d_b7 * s.v[848]) + (eq13_e1073 * s.db[848][7]));
        let eq13_e1075_d_b8: f64 = ((eq13_e1073_d_b8 * s.v[848]) + (eq13_e1073 * s.db[848][8]));
        let eq13_e1075_d_b9: f64 = ((eq13_e1073_d_b9 * s.v[848]) + (eq13_e1073 * s.db[848][9]));
        let eq13_e1075_d_b10: f64 = ((eq13_e1073_d_b10 * s.v[848]) + (eq13_e1073 * s.db[848][10]));
        let eq13_e1075_d_b11: f64 = ((eq13_e1073_d_b11 * s.v[848]) + (eq13_e1073 * s.db[848][11]));
        let eq13_e1075_d_b12: f64 = ((eq13_e1073_d_b12 * s.v[848]) + (eq13_e1073 * s.db[848][12]));
        let eq13_e1075_d_b13: f64 = ((eq13_e1073_d_b13 * s.v[848]) + (eq13_e1073 * s.db[848][13]));
        let eq13_e1075_d_b14: f64 = ((eq13_e1073_d_b14 * s.v[848]) + (eq13_e1073 * s.db[848][14]));
        let eq13_e1075_d_b15: f64 = ((eq13_e1073_d_b15 * s.v[848]) + (eq13_e1073 * s.db[848][15]));
        let eq13_e1075_d_b16: f64 = ((eq13_e1073_d_b16 * s.v[848]) + (eq13_e1073 * s.db[848][16]));
        let eq13_e1075_d_b17: f64 = ((eq13_e1073_d_b17 * s.v[848]) + (eq13_e1073 * s.db[848][17]));
        let eq13_e1075_d_b18: f64 = ((eq13_e1073_d_b18 * s.v[848]) + (eq13_e1073 * s.db[848][18]));
        let eq13_e1075_d_b19: f64 = ((eq13_e1073_d_b19 * s.v[848]) + (eq13_e1073 * s.db[848][19]));
        let eq13_e1075_d_b20: f64 = ((eq13_e1073_d_b20 * s.v[848]) + (eq13_e1073 * s.db[848][20]));
        let eq13_e1075_d_b21: f64 = ((eq13_e1073_d_b21 * s.v[848]) + (eq13_e1073 * s.db[848][21]));
        let eq13_e1075_d_b22: f64 = ((eq13_e1073_d_b22 * s.v[848]) + (eq13_e1073 * s.db[848][22]));
        let eq13_e1075_d_b23: f64 = ((eq13_e1073_d_b23 * s.v[848]) + (eq13_e1073 * s.db[848][23]));
        let eq13_e1075_d_b24: f64 = ((eq13_e1073_d_b24 * s.v[848]) + (eq13_e1073 * s.db[848][24]));
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
        let eq14_e1078: f64 = (s.v[0] * s.v[19]);
        let eq14_e1078_d_n0: f64 = ((s.dn[0][0] * s.v[19]) + (s.v[0] * s.dn[19][0]));
        let eq14_e1078_d_n1: f64 = ((s.dn[0][1] * s.v[19]) + (s.v[0] * s.dn[19][1]));
        let eq14_e1078_d_n2: f64 = ((s.dn[0][2] * s.v[19]) + (s.v[0] * s.dn[19][2]));
        let eq14_e1078_d_n3: f64 = ((s.dn[0][3] * s.v[19]) + (s.v[0] * s.dn[19][3]));
        let eq14_e1078_d_n4: f64 = ((s.dn[0][4] * s.v[19]) + (s.v[0] * s.dn[19][4]));
        let eq14_e1078_d_n5: f64 = ((s.dn[0][5] * s.v[19]) + (s.v[0] * s.dn[19][5]));
        let eq14_e1078_d_n6: f64 = ((s.dn[0][6] * s.v[19]) + (s.v[0] * s.dn[19][6]));
        let eq14_e1078_d_n7: f64 = ((s.dn[0][7] * s.v[19]) + (s.v[0] * s.dn[19][7]));
        let eq14_e1078_d_n8: f64 = ((s.dn[0][8] * s.v[19]) + (s.v[0] * s.dn[19][8]));
        let eq14_e1078_d_n9: f64 = ((s.dn[0][9] * s.v[19]) + (s.v[0] * s.dn[19][9]));
        let eq14_e1078_d_n10: f64 = ((s.dn[0][10] * s.v[19]) + (s.v[0] * s.dn[19][10]));
        let eq14_e1078_d_n11: f64 = ((s.dn[0][11] * s.v[19]) + (s.v[0] * s.dn[19][11]));
        let eq14_e1078_d_n12: f64 = ((s.dn[0][12] * s.v[19]) + (s.v[0] * s.dn[19][12]));
        let eq14_e1078_d_n13: f64 = ((s.dn[0][13] * s.v[19]) + (s.v[0] * s.dn[19][13]));
        let eq14_e1078_d_n14: f64 = ((s.dn[0][14] * s.v[19]) + (s.v[0] * s.dn[19][14]));
        let eq14_e1078_d_n15: f64 = ((s.dn[0][15] * s.v[19]) + (s.v[0] * s.dn[19][15]));
        let eq14_e1078_d_n16: f64 = ((s.dn[0][16] * s.v[19]) + (s.v[0] * s.dn[19][16]));
        let eq14_e1078_d_n17: f64 = ((s.dn[0][17] * s.v[19]) + (s.v[0] * s.dn[19][17]));
        let eq14_e1078_d_n18: f64 = ((s.dn[0][18] * s.v[19]) + (s.v[0] * s.dn[19][18]));
        let eq14_e1078_d_n19: f64 = ((s.dn[0][19] * s.v[19]) + (s.v[0] * s.dn[19][19]));
        let eq14_e1078_d_n20: f64 = ((s.dn[0][20] * s.v[19]) + (s.v[0] * s.dn[19][20]));
        let eq14_e1078_d_b0: f64 = ((s.db[0][0] * s.v[19]) + (s.v[0] * s.db[19][0]));
        let eq14_e1078_d_b1: f64 = ((s.db[0][1] * s.v[19]) + (s.v[0] * s.db[19][1]));
        let eq14_e1078_d_b2: f64 = ((s.db[0][2] * s.v[19]) + (s.v[0] * s.db[19][2]));
        let eq14_e1078_d_b3: f64 = ((s.db[0][3] * s.v[19]) + (s.v[0] * s.db[19][3]));
        let eq14_e1078_d_b4: f64 = ((s.db[0][4] * s.v[19]) + (s.v[0] * s.db[19][4]));
        let eq14_e1078_d_b5: f64 = ((s.db[0][5] * s.v[19]) + (s.v[0] * s.db[19][5]));
        let eq14_e1078_d_b6: f64 = ((s.db[0][6] * s.v[19]) + (s.v[0] * s.db[19][6]));
        let eq14_e1078_d_b7: f64 = ((s.db[0][7] * s.v[19]) + (s.v[0] * s.db[19][7]));
        let eq14_e1078_d_b8: f64 = ((s.db[0][8] * s.v[19]) + (s.v[0] * s.db[19][8]));
        let eq14_e1078_d_b9: f64 = ((s.db[0][9] * s.v[19]) + (s.v[0] * s.db[19][9]));
        let eq14_e1078_d_b10: f64 = ((s.db[0][10] * s.v[19]) + (s.v[0] * s.db[19][10]));
        let eq14_e1078_d_b11: f64 = ((s.db[0][11] * s.v[19]) + (s.v[0] * s.db[19][11]));
        let eq14_e1078_d_b12: f64 = ((s.db[0][12] * s.v[19]) + (s.v[0] * s.db[19][12]));
        let eq14_e1078_d_b13: f64 = ((s.db[0][13] * s.v[19]) + (s.v[0] * s.db[19][13]));
        let eq14_e1078_d_b14: f64 = ((s.db[0][14] * s.v[19]) + (s.v[0] * s.db[19][14]));
        let eq14_e1078_d_b15: f64 = ((s.db[0][15] * s.v[19]) + (s.v[0] * s.db[19][15]));
        let eq14_e1078_d_b16: f64 = ((s.db[0][16] * s.v[19]) + (s.v[0] * s.db[19][16]));
        let eq14_e1078_d_b17: f64 = ((s.db[0][17] * s.v[19]) + (s.v[0] * s.db[19][17]));
        let eq14_e1078_d_b18: f64 = ((s.db[0][18] * s.v[19]) + (s.v[0] * s.db[19][18]));
        let eq14_e1078_d_b19: f64 = ((s.db[0][19] * s.v[19]) + (s.v[0] * s.db[19][19]));
        let eq14_e1078_d_b20: f64 = ((s.db[0][20] * s.v[19]) + (s.v[0] * s.db[19][20]));
        let eq14_e1078_d_b21: f64 = ((s.db[0][21] * s.v[19]) + (s.v[0] * s.db[19][21]));
        let eq14_e1078_d_b22: f64 = ((s.db[0][22] * s.v[19]) + (s.v[0] * s.db[19][22]));
        let eq14_e1078_d_b23: f64 = ((s.db[0][23] * s.v[19]) + (s.v[0] * s.db[19][23]));
        let eq14_e1078_d_b24: f64 = ((s.db[0][24] * s.v[19]) + (s.v[0] * s.db[19][24]));
        let eq14_e1080: f64 = (eq14_e1078 * p.p32);
        let eq14_e1080_d_n0: f64 = (eq14_e1078_d_n0 * p.p32);
        let eq14_e1080_d_n1: f64 = (eq14_e1078_d_n1 * p.p32);
        let eq14_e1080_d_n2: f64 = (eq14_e1078_d_n2 * p.p32);
        let eq14_e1080_d_n3: f64 = (eq14_e1078_d_n3 * p.p32);
        let eq14_e1080_d_n4: f64 = (eq14_e1078_d_n4 * p.p32);
        let eq14_e1080_d_n5: f64 = (eq14_e1078_d_n5 * p.p32);
        let eq14_e1080_d_n6: f64 = (eq14_e1078_d_n6 * p.p32);
        let eq14_e1080_d_n7: f64 = (eq14_e1078_d_n7 * p.p32);
        let eq14_e1080_d_n8: f64 = (eq14_e1078_d_n8 * p.p32);
        let eq14_e1080_d_n9: f64 = (eq14_e1078_d_n9 * p.p32);
        let eq14_e1080_d_n10: f64 = (eq14_e1078_d_n10 * p.p32);
        let eq14_e1080_d_n11: f64 = (eq14_e1078_d_n11 * p.p32);
        let eq14_e1080_d_n12: f64 = (eq14_e1078_d_n12 * p.p32);
        let eq14_e1080_d_n13: f64 = (eq14_e1078_d_n13 * p.p32);
        let eq14_e1080_d_n14: f64 = (eq14_e1078_d_n14 * p.p32);
        let eq14_e1080_d_n15: f64 = (eq14_e1078_d_n15 * p.p32);
        let eq14_e1080_d_n16: f64 = (eq14_e1078_d_n16 * p.p32);
        let eq14_e1080_d_n17: f64 = (eq14_e1078_d_n17 * p.p32);
        let eq14_e1080_d_n18: f64 = (eq14_e1078_d_n18 * p.p32);
        let eq14_e1080_d_n19: f64 = (eq14_e1078_d_n19 * p.p32);
        let eq14_e1080_d_n20: f64 = (eq14_e1078_d_n20 * p.p32);
        let eq14_e1080_d_b0: f64 = (eq14_e1078_d_b0 * p.p32);
        let eq14_e1080_d_b1: f64 = (eq14_e1078_d_b1 * p.p32);
        let eq14_e1080_d_b2: f64 = (eq14_e1078_d_b2 * p.p32);
        let eq14_e1080_d_b3: f64 = (eq14_e1078_d_b3 * p.p32);
        let eq14_e1080_d_b4: f64 = (eq14_e1078_d_b4 * p.p32);
        let eq14_e1080_d_b5: f64 = (eq14_e1078_d_b5 * p.p32);
        let eq14_e1080_d_b6: f64 = (eq14_e1078_d_b6 * p.p32);
        let eq14_e1080_d_b7: f64 = (eq14_e1078_d_b7 * p.p32);
        let eq14_e1080_d_b8: f64 = (eq14_e1078_d_b8 * p.p32);
        let eq14_e1080_d_b9: f64 = (eq14_e1078_d_b9 * p.p32);
        let eq14_e1080_d_b10: f64 = (eq14_e1078_d_b10 * p.p32);
        let eq14_e1080_d_b11: f64 = (eq14_e1078_d_b11 * p.p32);
        let eq14_e1080_d_b12: f64 = (eq14_e1078_d_b12 * p.p32);
        let eq14_e1080_d_b13: f64 = (eq14_e1078_d_b13 * p.p32);
        let eq14_e1080_d_b14: f64 = (eq14_e1078_d_b14 * p.p32);
        let eq14_e1080_d_b15: f64 = (eq14_e1078_d_b15 * p.p32);
        let eq14_e1080_d_b16: f64 = (eq14_e1078_d_b16 * p.p32);
        let eq14_e1080_d_b17: f64 = (eq14_e1078_d_b17 * p.p32);
        let eq14_e1080_d_b18: f64 = (eq14_e1078_d_b18 * p.p32);
        let eq14_e1080_d_b19: f64 = (eq14_e1078_d_b19 * p.p32);
        let eq14_e1080_d_b20: f64 = (eq14_e1078_d_b20 * p.p32);
        let eq14_e1080_d_b21: f64 = (eq14_e1078_d_b21 * p.p32);
        let eq14_e1080_d_b22: f64 = (eq14_e1078_d_b22 * p.p32);
        let eq14_e1080_d_b23: f64 = (eq14_e1078_d_b23 * p.p32);
        let eq14_e1080_d_b24: f64 = (eq14_e1078_d_b24 * p.p32);
        let eq14_e1082: f64 = (eq14_e1080 * s.v[849]);
        let eq14_e1082_d_n0: f64 = ((eq14_e1080_d_n0 * s.v[849]) + (eq14_e1080 * s.dn[849][0]));
        let eq14_e1082_d_n1: f64 = ((eq14_e1080_d_n1 * s.v[849]) + (eq14_e1080 * s.dn[849][1]));
        let eq14_e1082_d_n2: f64 = ((eq14_e1080_d_n2 * s.v[849]) + (eq14_e1080 * s.dn[849][2]));
        let eq14_e1082_d_n3: f64 = ((eq14_e1080_d_n3 * s.v[849]) + (eq14_e1080 * s.dn[849][3]));
        let eq14_e1082_d_n4: f64 = ((eq14_e1080_d_n4 * s.v[849]) + (eq14_e1080 * s.dn[849][4]));
        let eq14_e1082_d_n5: f64 = ((eq14_e1080_d_n5 * s.v[849]) + (eq14_e1080 * s.dn[849][5]));
        let eq14_e1082_d_n6: f64 = ((eq14_e1080_d_n6 * s.v[849]) + (eq14_e1080 * s.dn[849][6]));
        let eq14_e1082_d_n7: f64 = ((eq14_e1080_d_n7 * s.v[849]) + (eq14_e1080 * s.dn[849][7]));
        let eq14_e1082_d_n8: f64 = ((eq14_e1080_d_n8 * s.v[849]) + (eq14_e1080 * s.dn[849][8]));
        let eq14_e1082_d_n9: f64 = ((eq14_e1080_d_n9 * s.v[849]) + (eq14_e1080 * s.dn[849][9]));
        let eq14_e1082_d_n10: f64 = ((eq14_e1080_d_n10 * s.v[849]) + (eq14_e1080 * s.dn[849][10]));
        let eq14_e1082_d_n11: f64 = ((eq14_e1080_d_n11 * s.v[849]) + (eq14_e1080 * s.dn[849][11]));
        let eq14_e1082_d_n12: f64 = ((eq14_e1080_d_n12 * s.v[849]) + (eq14_e1080 * s.dn[849][12]));
        let eq14_e1082_d_n13: f64 = ((eq14_e1080_d_n13 * s.v[849]) + (eq14_e1080 * s.dn[849][13]));
        let eq14_e1082_d_n14: f64 = ((eq14_e1080_d_n14 * s.v[849]) + (eq14_e1080 * s.dn[849][14]));
        let eq14_e1082_d_n15: f64 = ((eq14_e1080_d_n15 * s.v[849]) + (eq14_e1080 * s.dn[849][15]));
        let eq14_e1082_d_n16: f64 = ((eq14_e1080_d_n16 * s.v[849]) + (eq14_e1080 * s.dn[849][16]));
        let eq14_e1082_d_n17: f64 = ((eq14_e1080_d_n17 * s.v[849]) + (eq14_e1080 * s.dn[849][17]));
        let eq14_e1082_d_n18: f64 = ((eq14_e1080_d_n18 * s.v[849]) + (eq14_e1080 * s.dn[849][18]));
        let eq14_e1082_d_n19: f64 = ((eq14_e1080_d_n19 * s.v[849]) + (eq14_e1080 * s.dn[849][19]));
        let eq14_e1082_d_n20: f64 = ((eq14_e1080_d_n20 * s.v[849]) + (eq14_e1080 * s.dn[849][20]));
        let eq14_e1082_d_b0: f64 = ((eq14_e1080_d_b0 * s.v[849]) + (eq14_e1080 * s.db[849][0]));
        let eq14_e1082_d_b1: f64 = ((eq14_e1080_d_b1 * s.v[849]) + (eq14_e1080 * s.db[849][1]));
        let eq14_e1082_d_b2: f64 = ((eq14_e1080_d_b2 * s.v[849]) + (eq14_e1080 * s.db[849][2]));
        let eq14_e1082_d_b3: f64 = ((eq14_e1080_d_b3 * s.v[849]) + (eq14_e1080 * s.db[849][3]));
        let eq14_e1082_d_b4: f64 = ((eq14_e1080_d_b4 * s.v[849]) + (eq14_e1080 * s.db[849][4]));
        let eq14_e1082_d_b5: f64 = ((eq14_e1080_d_b5 * s.v[849]) + (eq14_e1080 * s.db[849][5]));
        let eq14_e1082_d_b6: f64 = ((eq14_e1080_d_b6 * s.v[849]) + (eq14_e1080 * s.db[849][6]));
        let eq14_e1082_d_b7: f64 = ((eq14_e1080_d_b7 * s.v[849]) + (eq14_e1080 * s.db[849][7]));
        let eq14_e1082_d_b8: f64 = ((eq14_e1080_d_b8 * s.v[849]) + (eq14_e1080 * s.db[849][8]));
        let eq14_e1082_d_b9: f64 = ((eq14_e1080_d_b9 * s.v[849]) + (eq14_e1080 * s.db[849][9]));
        let eq14_e1082_d_b10: f64 = ((eq14_e1080_d_b10 * s.v[849]) + (eq14_e1080 * s.db[849][10]));
        let eq14_e1082_d_b11: f64 = ((eq14_e1080_d_b11 * s.v[849]) + (eq14_e1080 * s.db[849][11]));
        let eq14_e1082_d_b12: f64 = ((eq14_e1080_d_b12 * s.v[849]) + (eq14_e1080 * s.db[849][12]));
        let eq14_e1082_d_b13: f64 = ((eq14_e1080_d_b13 * s.v[849]) + (eq14_e1080 * s.db[849][13]));
        let eq14_e1082_d_b14: f64 = ((eq14_e1080_d_b14 * s.v[849]) + (eq14_e1080 * s.db[849][14]));
        let eq14_e1082_d_b15: f64 = ((eq14_e1080_d_b15 * s.v[849]) + (eq14_e1080 * s.db[849][15]));
        let eq14_e1082_d_b16: f64 = ((eq14_e1080_d_b16 * s.v[849]) + (eq14_e1080 * s.db[849][16]));
        let eq14_e1082_d_b17: f64 = ((eq14_e1080_d_b17 * s.v[849]) + (eq14_e1080 * s.db[849][17]));
        let eq14_e1082_d_b18: f64 = ((eq14_e1080_d_b18 * s.v[849]) + (eq14_e1080 * s.db[849][18]));
        let eq14_e1082_d_b19: f64 = ((eq14_e1080_d_b19 * s.v[849]) + (eq14_e1080 * s.db[849][19]));
        let eq14_e1082_d_b20: f64 = ((eq14_e1080_d_b20 * s.v[849]) + (eq14_e1080 * s.db[849][20]));
        let eq14_e1082_d_b21: f64 = ((eq14_e1080_d_b21 * s.v[849]) + (eq14_e1080 * s.db[849][21]));
        let eq14_e1082_d_b22: f64 = ((eq14_e1080_d_b22 * s.v[849]) + (eq14_e1080 * s.db[849][22]));
        let eq14_e1082_d_b23: f64 = ((eq14_e1080_d_b23 * s.v[849]) + (eq14_e1080 * s.db[849][23]));
        let eq14_e1082_d_b24: f64 = ((eq14_e1080_d_b24 * s.v[849]) + (eq14_e1080 * s.db[849][24]));
        let eq14_value: f64 = eq14_e1082;
        let eq14_node_derivatives: [f64; 21] = [eq14_e1082_d_n0, eq14_e1082_d_n1, eq14_e1082_d_n2, eq14_e1082_d_n3, eq14_e1082_d_n4, eq14_e1082_d_n5, eq14_e1082_d_n6, eq14_e1082_d_n7, eq14_e1082_d_n8, eq14_e1082_d_n9, eq14_e1082_d_n10, eq14_e1082_d_n11, eq14_e1082_d_n12, eq14_e1082_d_n13, eq14_e1082_d_n14, eq14_e1082_d_n15, eq14_e1082_d_n16, eq14_e1082_d_n17, eq14_e1082_d_n18, eq14_e1082_d_n19, eq14_e1082_d_n20];
        let eq14_branch_derivatives: [f64; 25] = [eq14_e1082_d_b0, eq14_e1082_d_b1, eq14_e1082_d_b2, eq14_e1082_d_b3, eq14_e1082_d_b4, eq14_e1082_d_b5, eq14_e1082_d_b6, eq14_e1082_d_b7, eq14_e1082_d_b8, eq14_e1082_d_b9, eq14_e1082_d_b10, eq14_e1082_d_b11, eq14_e1082_d_b12, eq14_e1082_d_b13, eq14_e1082_d_b14, eq14_e1082_d_b15, eq14_e1082_d_b16, eq14_e1082_d_b17, eq14_e1082_d_b18, eq14_e1082_d_b19, eq14_e1082_d_b20, eq14_e1082_d_b21, eq14_e1082_d_b22, eq14_e1082_d_b23, eq14_e1082_d_b24];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let (eq15_e1092, eq15_e1092_d_n0, eq15_e1092_d_n1, eq15_e1092_d_n2, eq15_e1092_d_n3, eq15_e1092_d_n4, eq15_e1092_d_n5, eq15_e1092_d_n6, eq15_e1092_d_n7, eq15_e1092_d_n8, eq15_e1092_d_n9, eq15_e1092_d_n10, eq15_e1092_d_n11, eq15_e1092_d_n12, eq15_e1092_d_n13, eq15_e1092_d_n14, eq15_e1092_d_n15, eq15_e1092_d_n16, eq15_e1092_d_n17, eq15_e1092_d_n18, eq15_e1092_d_n19, eq15_e1092_d_n20, eq15_e1092_d_b0, eq15_e1092_d_b1, eq15_e1092_d_b2, eq15_e1092_d_b3, eq15_e1092_d_b4, eq15_e1092_d_b5, eq15_e1092_d_b6, eq15_e1092_d_b7, eq15_e1092_d_b8, eq15_e1092_d_b9, eq15_e1092_d_b10, eq15_e1092_d_b11, eq15_e1092_d_b12, eq15_e1092_d_b13, eq15_e1092_d_b14, eq15_e1092_d_b15, eq15_e1092_d_b16, eq15_e1092_d_b17, eq15_e1092_d_b18, eq15_e1092_d_b19, eq15_e1092_d_b20, eq15_e1092_d_b21, eq15_e1092_d_b22, eq15_e1092_d_b23, eq15_e1092_d_b24,) = {
    if s.b[2914] {
        let eq15_e1086: f64 = (s.v[19] * p.p32);
        let eq15_e1086_d_n0: f64 = (s.dn[19][0] * p.p32);
        let eq15_e1086_d_n1: f64 = (s.dn[19][1] * p.p32);
        let eq15_e1086_d_n2: f64 = (s.dn[19][2] * p.p32);
        let eq15_e1086_d_n3: f64 = (s.dn[19][3] * p.p32);
        let eq15_e1086_d_n4: f64 = (s.dn[19][4] * p.p32);
        let eq15_e1086_d_n5: f64 = (s.dn[19][5] * p.p32);
        let eq15_e1086_d_n6: f64 = (s.dn[19][6] * p.p32);
        let eq15_e1086_d_n7: f64 = (s.dn[19][7] * p.p32);
        let eq15_e1086_d_n8: f64 = (s.dn[19][8] * p.p32);
        let eq15_e1086_d_n9: f64 = (s.dn[19][9] * p.p32);
        let eq15_e1086_d_n10: f64 = (s.dn[19][10] * p.p32);
        let eq15_e1086_d_n11: f64 = (s.dn[19][11] * p.p32);
        let eq15_e1086_d_n12: f64 = (s.dn[19][12] * p.p32);
        let eq15_e1086_d_n13: f64 = (s.dn[19][13] * p.p32);
        let eq15_e1086_d_n14: f64 = (s.dn[19][14] * p.p32);
        let eq15_e1086_d_n15: f64 = (s.dn[19][15] * p.p32);
        let eq15_e1086_d_n16: f64 = (s.dn[19][16] * p.p32);
        let eq15_e1086_d_n17: f64 = (s.dn[19][17] * p.p32);
        let eq15_e1086_d_n18: f64 = (s.dn[19][18] * p.p32);
        let eq15_e1086_d_n19: f64 = (s.dn[19][19] * p.p32);
        let eq15_e1086_d_n20: f64 = (s.dn[19][20] * p.p32);
        let eq15_e1086_d_b0: f64 = (s.db[19][0] * p.p32);
        let eq15_e1086_d_b1: f64 = (s.db[19][1] * p.p32);
        let eq15_e1086_d_b2: f64 = (s.db[19][2] * p.p32);
        let eq15_e1086_d_b3: f64 = (s.db[19][3] * p.p32);
        let eq15_e1086_d_b4: f64 = (s.db[19][4] * p.p32);
        let eq15_e1086_d_b5: f64 = (s.db[19][5] * p.p32);
        let eq15_e1086_d_b6: f64 = (s.db[19][6] * p.p32);
        let eq15_e1086_d_b7: f64 = (s.db[19][7] * p.p32);
        let eq15_e1086_d_b8: f64 = (s.db[19][8] * p.p32);
        let eq15_e1086_d_b9: f64 = (s.db[19][9] * p.p32);
        let eq15_e1086_d_b10: f64 = (s.db[19][10] * p.p32);
        let eq15_e1086_d_b11: f64 = (s.db[19][11] * p.p32);
        let eq15_e1086_d_b12: f64 = (s.db[19][12] * p.p32);
        let eq15_e1086_d_b13: f64 = (s.db[19][13] * p.p32);
        let eq15_e1086_d_b14: f64 = (s.db[19][14] * p.p32);
        let eq15_e1086_d_b15: f64 = (s.db[19][15] * p.p32);
        let eq15_e1086_d_b16: f64 = (s.db[19][16] * p.p32);
        let eq15_e1086_d_b17: f64 = (s.db[19][17] * p.p32);
        let eq15_e1086_d_b18: f64 = (s.db[19][18] * p.p32);
        let eq15_e1086_d_b19: f64 = (s.db[19][19] * p.p32);
        let eq15_e1086_d_b20: f64 = (s.db[19][20] * p.p32);
        let eq15_e1086_d_b21: f64 = (s.db[19][21] * p.p32);
        let eq15_e1086_d_b22: f64 = (s.db[19][22] * p.p32);
        let eq15_e1086_d_b23: f64 = (s.db[19][23] * p.p32);
        let eq15_e1086_d_b24: f64 = (s.db[19][24] * p.p32);
        let eq15_e1088: f64 = (eq15_e1086 * s.v[811]);
        let eq15_e1088_d_n0: f64 = ((eq15_e1086_d_n0 * s.v[811]) + (eq15_e1086 * s.dn[811][0]));
        let eq15_e1088_d_n1: f64 = ((eq15_e1086_d_n1 * s.v[811]) + (eq15_e1086 * s.dn[811][1]));
        let eq15_e1088_d_n2: f64 = ((eq15_e1086_d_n2 * s.v[811]) + (eq15_e1086 * s.dn[811][2]));
        let eq15_e1088_d_n3: f64 = ((eq15_e1086_d_n3 * s.v[811]) + (eq15_e1086 * s.dn[811][3]));
        let eq15_e1088_d_n4: f64 = ((eq15_e1086_d_n4 * s.v[811]) + (eq15_e1086 * s.dn[811][4]));
        let eq15_e1088_d_n5: f64 = ((eq15_e1086_d_n5 * s.v[811]) + (eq15_e1086 * s.dn[811][5]));
        let eq15_e1088_d_n6: f64 = ((eq15_e1086_d_n6 * s.v[811]) + (eq15_e1086 * s.dn[811][6]));
        let eq15_e1088_d_n7: f64 = ((eq15_e1086_d_n7 * s.v[811]) + (eq15_e1086 * s.dn[811][7]));
        let eq15_e1088_d_n8: f64 = ((eq15_e1086_d_n8 * s.v[811]) + (eq15_e1086 * s.dn[811][8]));
        let eq15_e1088_d_n9: f64 = ((eq15_e1086_d_n9 * s.v[811]) + (eq15_e1086 * s.dn[811][9]));
        let eq15_e1088_d_n10: f64 = ((eq15_e1086_d_n10 * s.v[811]) + (eq15_e1086 * s.dn[811][10]));
        let eq15_e1088_d_n11: f64 = ((eq15_e1086_d_n11 * s.v[811]) + (eq15_e1086 * s.dn[811][11]));
        let eq15_e1088_d_n12: f64 = ((eq15_e1086_d_n12 * s.v[811]) + (eq15_e1086 * s.dn[811][12]));
        let eq15_e1088_d_n13: f64 = ((eq15_e1086_d_n13 * s.v[811]) + (eq15_e1086 * s.dn[811][13]));
        let eq15_e1088_d_n14: f64 = ((eq15_e1086_d_n14 * s.v[811]) + (eq15_e1086 * s.dn[811][14]));
        let eq15_e1088_d_n15: f64 = ((eq15_e1086_d_n15 * s.v[811]) + (eq15_e1086 * s.dn[811][15]));
        let eq15_e1088_d_n16: f64 = ((eq15_e1086_d_n16 * s.v[811]) + (eq15_e1086 * s.dn[811][16]));
        let eq15_e1088_d_n17: f64 = ((eq15_e1086_d_n17 * s.v[811]) + (eq15_e1086 * s.dn[811][17]));
        let eq15_e1088_d_n18: f64 = ((eq15_e1086_d_n18 * s.v[811]) + (eq15_e1086 * s.dn[811][18]));
        let eq15_e1088_d_n19: f64 = ((eq15_e1086_d_n19 * s.v[811]) + (eq15_e1086 * s.dn[811][19]));
        let eq15_e1088_d_n20: f64 = ((eq15_e1086_d_n20 * s.v[811]) + (eq15_e1086 * s.dn[811][20]));
        let eq15_e1088_d_b0: f64 = ((eq15_e1086_d_b0 * s.v[811]) + (eq15_e1086 * s.db[811][0]));
        let eq15_e1088_d_b1: f64 = ((eq15_e1086_d_b1 * s.v[811]) + (eq15_e1086 * s.db[811][1]));
        let eq15_e1088_d_b2: f64 = ((eq15_e1086_d_b2 * s.v[811]) + (eq15_e1086 * s.db[811][2]));
        let eq15_e1088_d_b3: f64 = ((eq15_e1086_d_b3 * s.v[811]) + (eq15_e1086 * s.db[811][3]));
        let eq15_e1088_d_b4: f64 = ((eq15_e1086_d_b4 * s.v[811]) + (eq15_e1086 * s.db[811][4]));
        let eq15_e1088_d_b5: f64 = ((eq15_e1086_d_b5 * s.v[811]) + (eq15_e1086 * s.db[811][5]));
        let eq15_e1088_d_b6: f64 = ((eq15_e1086_d_b6 * s.v[811]) + (eq15_e1086 * s.db[811][6]));
        let eq15_e1088_d_b7: f64 = ((eq15_e1086_d_b7 * s.v[811]) + (eq15_e1086 * s.db[811][7]));
        let eq15_e1088_d_b8: f64 = ((eq15_e1086_d_b8 * s.v[811]) + (eq15_e1086 * s.db[811][8]));
        let eq15_e1088_d_b9: f64 = ((eq15_e1086_d_b9 * s.v[811]) + (eq15_e1086 * s.db[811][9]));
        let eq15_e1088_d_b10: f64 = ((eq15_e1086_d_b10 * s.v[811]) + (eq15_e1086 * s.db[811][10]));
        let eq15_e1088_d_b11: f64 = ((eq15_e1086_d_b11 * s.v[811]) + (eq15_e1086 * s.db[811][11]));
        let eq15_e1088_d_b12: f64 = ((eq15_e1086_d_b12 * s.v[811]) + (eq15_e1086 * s.db[811][12]));
        let eq15_e1088_d_b13: f64 = ((eq15_e1086_d_b13 * s.v[811]) + (eq15_e1086 * s.db[811][13]));
        let eq15_e1088_d_b14: f64 = ((eq15_e1086_d_b14 * s.v[811]) + (eq15_e1086 * s.db[811][14]));
        let eq15_e1088_d_b15: f64 = ((eq15_e1086_d_b15 * s.v[811]) + (eq15_e1086 * s.db[811][15]));
        let eq15_e1088_d_b16: f64 = ((eq15_e1086_d_b16 * s.v[811]) + (eq15_e1086 * s.db[811][16]));
        let eq15_e1088_d_b17: f64 = ((eq15_e1086_d_b17 * s.v[811]) + (eq15_e1086 * s.db[811][17]));
        let eq15_e1088_d_b18: f64 = ((eq15_e1086_d_b18 * s.v[811]) + (eq15_e1086 * s.db[811][18]));
        let eq15_e1088_d_b19: f64 = ((eq15_e1086_d_b19 * s.v[811]) + (eq15_e1086 * s.db[811][19]));
        let eq15_e1088_d_b20: f64 = ((eq15_e1086_d_b20 * s.v[811]) + (eq15_e1086 * s.db[811][20]));
        let eq15_e1088_d_b21: f64 = ((eq15_e1086_d_b21 * s.v[811]) + (eq15_e1086 * s.db[811][21]));
        let eq15_e1088_d_b22: f64 = ((eq15_e1086_d_b22 * s.v[811]) + (eq15_e1086 * s.db[811][22]));
        let eq15_e1088_d_b23: f64 = ((eq15_e1086_d_b23 * s.v[811]) + (eq15_e1086 * s.db[811][23]));
        let eq15_e1088_d_b24: f64 = ((eq15_e1086_d_b24 * s.v[811]) + (eq15_e1086 * s.db[811][24]));
        let eq15_e1090: f64 = (eq15_e1088 * (nv1 - nv5));
        let eq15_e1090_d_n0: f64 = (eq15_e1088_d_n0 * (nv1 - nv5));
        let eq15_e1090_d_n1: f64 = ((eq15_e1088_d_n1 * (nv1 - nv5)) + eq15_e1088);
        let eq15_e1090_d_n2: f64 = (eq15_e1088_d_n2 * (nv1 - nv5));
        let eq15_e1090_d_n3: f64 = (eq15_e1088_d_n3 * (nv1 - nv5));
        let eq15_e1090_d_n4: f64 = (eq15_e1088_d_n4 * (nv1 - nv5));
        let eq15_e1090_d_n5: f64 = ((eq15_e1088_d_n5 * (nv1 - nv5)) + (-eq15_e1088));
        let eq15_e1090_d_n6: f64 = (eq15_e1088_d_n6 * (nv1 - nv5));
        let eq15_e1090_d_n7: f64 = (eq15_e1088_d_n7 * (nv1 - nv5));
        let eq15_e1090_d_n8: f64 = (eq15_e1088_d_n8 * (nv1 - nv5));
        let eq15_e1090_d_n9: f64 = (eq15_e1088_d_n9 * (nv1 - nv5));
        let eq15_e1090_d_n10: f64 = (eq15_e1088_d_n10 * (nv1 - nv5));
        let eq15_e1090_d_n11: f64 = (eq15_e1088_d_n11 * (nv1 - nv5));
        let eq15_e1090_d_n12: f64 = (eq15_e1088_d_n12 * (nv1 - nv5));
        let eq15_e1090_d_n13: f64 = (eq15_e1088_d_n13 * (nv1 - nv5));
        let eq15_e1090_d_n14: f64 = (eq15_e1088_d_n14 * (nv1 - nv5));
        let eq15_e1090_d_n15: f64 = (eq15_e1088_d_n15 * (nv1 - nv5));
        let eq15_e1090_d_n16: f64 = (eq15_e1088_d_n16 * (nv1 - nv5));
        let eq15_e1090_d_n17: f64 = (eq15_e1088_d_n17 * (nv1 - nv5));
        let eq15_e1090_d_n18: f64 = (eq15_e1088_d_n18 * (nv1 - nv5));
        let eq15_e1090_d_n19: f64 = (eq15_e1088_d_n19 * (nv1 - nv5));
        let eq15_e1090_d_n20: f64 = (eq15_e1088_d_n20 * (nv1 - nv5));
        let eq15_e1090_d_b0: f64 = (eq15_e1088_d_b0 * (nv1 - nv5));
        let eq15_e1090_d_b1: f64 = (eq15_e1088_d_b1 * (nv1 - nv5));
        let eq15_e1090_d_b2: f64 = (eq15_e1088_d_b2 * (nv1 - nv5));
        let eq15_e1090_d_b3: f64 = (eq15_e1088_d_b3 * (nv1 - nv5));
        let eq15_e1090_d_b4: f64 = (eq15_e1088_d_b4 * (nv1 - nv5));
        let eq15_e1090_d_b5: f64 = (eq15_e1088_d_b5 * (nv1 - nv5));
        let eq15_e1090_d_b6: f64 = (eq15_e1088_d_b6 * (nv1 - nv5));
        let eq15_e1090_d_b7: f64 = (eq15_e1088_d_b7 * (nv1 - nv5));
        let eq15_e1090_d_b8: f64 = (eq15_e1088_d_b8 * (nv1 - nv5));
        let eq15_e1090_d_b9: f64 = (eq15_e1088_d_b9 * (nv1 - nv5));
        let eq15_e1090_d_b10: f64 = (eq15_e1088_d_b10 * (nv1 - nv5));
        let eq15_e1090_d_b11: f64 = (eq15_e1088_d_b11 * (nv1 - nv5));
        let eq15_e1090_d_b12: f64 = (eq15_e1088_d_b12 * (nv1 - nv5));
        let eq15_e1090_d_b13: f64 = (eq15_e1088_d_b13 * (nv1 - nv5));
        let eq15_e1090_d_b14: f64 = (eq15_e1088_d_b14 * (nv1 - nv5));
        let eq15_e1090_d_b15: f64 = (eq15_e1088_d_b15 * (nv1 - nv5));
        let eq15_e1090_d_b16: f64 = (eq15_e1088_d_b16 * (nv1 - nv5));
        let eq15_e1090_d_b17: f64 = (eq15_e1088_d_b17 * (nv1 - nv5));
        let eq15_e1090_d_b18: f64 = (eq15_e1088_d_b18 * (nv1 - nv5));
        let eq15_e1090_d_b19: f64 = (eq15_e1088_d_b19 * (nv1 - nv5));
        let eq15_e1090_d_b20: f64 = (eq15_e1088_d_b20 * (nv1 - nv5));
        let eq15_e1090_d_b21: f64 = (eq15_e1088_d_b21 * (nv1 - nv5));
        let eq15_e1090_d_b22: f64 = (eq15_e1088_d_b22 * (nv1 - nv5));
        let eq15_e1090_d_b23: f64 = (eq15_e1088_d_b23 * (nv1 - nv5));
        let eq15_e1090_d_b24: f64 = (eq15_e1088_d_b24 * (nv1 - nv5));
        (eq15_e1090, eq15_e1090_d_n0, eq15_e1090_d_n1, eq15_e1090_d_n2, eq15_e1090_d_n3, eq15_e1090_d_n4, eq15_e1090_d_n5, eq15_e1090_d_n6, eq15_e1090_d_n7, eq15_e1090_d_n8, eq15_e1090_d_n9, eq15_e1090_d_n10, eq15_e1090_d_n11, eq15_e1090_d_n12, eq15_e1090_d_n13, eq15_e1090_d_n14, eq15_e1090_d_n15, eq15_e1090_d_n16, eq15_e1090_d_n17, eq15_e1090_d_n18, eq15_e1090_d_n19, eq15_e1090_d_n20, eq15_e1090_d_b0, eq15_e1090_d_b1, eq15_e1090_d_b2, eq15_e1090_d_b3, eq15_e1090_d_b4, eq15_e1090_d_b5, eq15_e1090_d_b6, eq15_e1090_d_b7, eq15_e1090_d_b8, eq15_e1090_d_b9, eq15_e1090_d_b10, eq15_e1090_d_b11, eq15_e1090_d_b12, eq15_e1090_d_b13, eq15_e1090_d_b14, eq15_e1090_d_b15, eq15_e1090_d_b16, eq15_e1090_d_b17, eq15_e1090_d_b18, eq15_e1090_d_b19, eq15_e1090_d_b20, eq15_e1090_d_b21, eq15_e1090_d_b22, eq15_e1090_d_b23, eq15_e1090_d_b24,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e1092;
        let eq15_node_derivatives: [f64; 21] = [eq15_e1092_d_n0, eq15_e1092_d_n1, eq15_e1092_d_n2, eq15_e1092_d_n3, eq15_e1092_d_n4, eq15_e1092_d_n5, eq15_e1092_d_n6, eq15_e1092_d_n7, eq15_e1092_d_n8, eq15_e1092_d_n9, eq15_e1092_d_n10, eq15_e1092_d_n11, eq15_e1092_d_n12, eq15_e1092_d_n13, eq15_e1092_d_n14, eq15_e1092_d_n15, eq15_e1092_d_n16, eq15_e1092_d_n17, eq15_e1092_d_n18, eq15_e1092_d_n19, eq15_e1092_d_n20];
        let eq15_branch_derivatives: [f64; 25] = [eq15_e1092_d_b0, eq15_e1092_d_b1, eq15_e1092_d_b2, eq15_e1092_d_b3, eq15_e1092_d_b4, eq15_e1092_d_b5, eq15_e1092_d_b6, eq15_e1092_d_b7, eq15_e1092_d_b8, eq15_e1092_d_b9, eq15_e1092_d_b10, eq15_e1092_d_b11, eq15_e1092_d_b12, eq15_e1092_d_b13, eq15_e1092_d_b14, eq15_e1092_d_b15, eq15_e1092_d_b16, eq15_e1092_d_b17, eq15_e1092_d_b18, eq15_e1092_d_b19, eq15_e1092_d_b20, eq15_e1092_d_b21, eq15_e1092_d_b22, eq15_e1092_d_b23, eq15_e1092_d_b24];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(5),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let (eq17_e1107,) = {
    if (!s.b[2914]) {
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
    }

    pub(super) fn stamp_transient_equations_block_6(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq18_e1117, eq18_e1117_d_n0, eq18_e1117_d_n1, eq18_e1117_d_n2, eq18_e1117_d_n3, eq18_e1117_d_n4, eq18_e1117_d_n5, eq18_e1117_d_n6, eq18_e1117_d_n7, eq18_e1117_d_n8, eq18_e1117_d_n9, eq18_e1117_d_n10, eq18_e1117_d_n11, eq18_e1117_d_n12, eq18_e1117_d_n13, eq18_e1117_d_n14, eq18_e1117_d_n15, eq18_e1117_d_n16, eq18_e1117_d_n17, eq18_e1117_d_n18, eq18_e1117_d_n19, eq18_e1117_d_n20, eq18_e1117_d_b0, eq18_e1117_d_b1, eq18_e1117_d_b2, eq18_e1117_d_b3, eq18_e1117_d_b4, eq18_e1117_d_b5, eq18_e1117_d_b6, eq18_e1117_d_b7, eq18_e1117_d_b8, eq18_e1117_d_b9, eq18_e1117_d_b10, eq18_e1117_d_b11, eq18_e1117_d_b12, eq18_e1117_d_b13, eq18_e1117_d_b14, eq18_e1117_d_b15, eq18_e1117_d_b16, eq18_e1117_d_b17, eq18_e1117_d_b18, eq18_e1117_d_b19, eq18_e1117_d_b20, eq18_e1117_d_b21, eq18_e1117_d_b22, eq18_e1117_d_b23, eq18_e1117_d_b24,) = {
    if s.b[2915] {
        let eq18_e1111: f64 = (s.v[19] * p.p32);
        let eq18_e1111_d_n0: f64 = (s.dn[19][0] * p.p32);
        let eq18_e1111_d_n1: f64 = (s.dn[19][1] * p.p32);
        let eq18_e1111_d_n2: f64 = (s.dn[19][2] * p.p32);
        let eq18_e1111_d_n3: f64 = (s.dn[19][3] * p.p32);
        let eq18_e1111_d_n4: f64 = (s.dn[19][4] * p.p32);
        let eq18_e1111_d_n5: f64 = (s.dn[19][5] * p.p32);
        let eq18_e1111_d_n6: f64 = (s.dn[19][6] * p.p32);
        let eq18_e1111_d_n7: f64 = (s.dn[19][7] * p.p32);
        let eq18_e1111_d_n8: f64 = (s.dn[19][8] * p.p32);
        let eq18_e1111_d_n9: f64 = (s.dn[19][9] * p.p32);
        let eq18_e1111_d_n10: f64 = (s.dn[19][10] * p.p32);
        let eq18_e1111_d_n11: f64 = (s.dn[19][11] * p.p32);
        let eq18_e1111_d_n12: f64 = (s.dn[19][12] * p.p32);
        let eq18_e1111_d_n13: f64 = (s.dn[19][13] * p.p32);
        let eq18_e1111_d_n14: f64 = (s.dn[19][14] * p.p32);
        let eq18_e1111_d_n15: f64 = (s.dn[19][15] * p.p32);
        let eq18_e1111_d_n16: f64 = (s.dn[19][16] * p.p32);
        let eq18_e1111_d_n17: f64 = (s.dn[19][17] * p.p32);
        let eq18_e1111_d_n18: f64 = (s.dn[19][18] * p.p32);
        let eq18_e1111_d_n19: f64 = (s.dn[19][19] * p.p32);
        let eq18_e1111_d_n20: f64 = (s.dn[19][20] * p.p32);
        let eq18_e1111_d_b0: f64 = (s.db[19][0] * p.p32);
        let eq18_e1111_d_b1: f64 = (s.db[19][1] * p.p32);
        let eq18_e1111_d_b2: f64 = (s.db[19][2] * p.p32);
        let eq18_e1111_d_b3: f64 = (s.db[19][3] * p.p32);
        let eq18_e1111_d_b4: f64 = (s.db[19][4] * p.p32);
        let eq18_e1111_d_b5: f64 = (s.db[19][5] * p.p32);
        let eq18_e1111_d_b6: f64 = (s.db[19][6] * p.p32);
        let eq18_e1111_d_b7: f64 = (s.db[19][7] * p.p32);
        let eq18_e1111_d_b8: f64 = (s.db[19][8] * p.p32);
        let eq18_e1111_d_b9: f64 = (s.db[19][9] * p.p32);
        let eq18_e1111_d_b10: f64 = (s.db[19][10] * p.p32);
        let eq18_e1111_d_b11: f64 = (s.db[19][11] * p.p32);
        let eq18_e1111_d_b12: f64 = (s.db[19][12] * p.p32);
        let eq18_e1111_d_b13: f64 = (s.db[19][13] * p.p32);
        let eq18_e1111_d_b14: f64 = (s.db[19][14] * p.p32);
        let eq18_e1111_d_b15: f64 = (s.db[19][15] * p.p32);
        let eq18_e1111_d_b16: f64 = (s.db[19][16] * p.p32);
        let eq18_e1111_d_b17: f64 = (s.db[19][17] * p.p32);
        let eq18_e1111_d_b18: f64 = (s.db[19][18] * p.p32);
        let eq18_e1111_d_b19: f64 = (s.db[19][19] * p.p32);
        let eq18_e1111_d_b20: f64 = (s.db[19][20] * p.p32);
        let eq18_e1111_d_b21: f64 = (s.db[19][21] * p.p32);
        let eq18_e1111_d_b22: f64 = (s.db[19][22] * p.p32);
        let eq18_e1111_d_b23: f64 = (s.db[19][23] * p.p32);
        let eq18_e1111_d_b24: f64 = (s.db[19][24] * p.p32);
        let eq18_e1113: f64 = (eq18_e1111 * s.v[812]);
        let eq18_e1113_d_n0: f64 = ((eq18_e1111_d_n0 * s.v[812]) + (eq18_e1111 * s.dn[812][0]));
        let eq18_e1113_d_n1: f64 = ((eq18_e1111_d_n1 * s.v[812]) + (eq18_e1111 * s.dn[812][1]));
        let eq18_e1113_d_n2: f64 = ((eq18_e1111_d_n2 * s.v[812]) + (eq18_e1111 * s.dn[812][2]));
        let eq18_e1113_d_n3: f64 = ((eq18_e1111_d_n3 * s.v[812]) + (eq18_e1111 * s.dn[812][3]));
        let eq18_e1113_d_n4: f64 = ((eq18_e1111_d_n4 * s.v[812]) + (eq18_e1111 * s.dn[812][4]));
        let eq18_e1113_d_n5: f64 = ((eq18_e1111_d_n5 * s.v[812]) + (eq18_e1111 * s.dn[812][5]));
        let eq18_e1113_d_n6: f64 = ((eq18_e1111_d_n6 * s.v[812]) + (eq18_e1111 * s.dn[812][6]));
        let eq18_e1113_d_n7: f64 = ((eq18_e1111_d_n7 * s.v[812]) + (eq18_e1111 * s.dn[812][7]));
        let eq18_e1113_d_n8: f64 = ((eq18_e1111_d_n8 * s.v[812]) + (eq18_e1111 * s.dn[812][8]));
        let eq18_e1113_d_n9: f64 = ((eq18_e1111_d_n9 * s.v[812]) + (eq18_e1111 * s.dn[812][9]));
        let eq18_e1113_d_n10: f64 = ((eq18_e1111_d_n10 * s.v[812]) + (eq18_e1111 * s.dn[812][10]));
        let eq18_e1113_d_n11: f64 = ((eq18_e1111_d_n11 * s.v[812]) + (eq18_e1111 * s.dn[812][11]));
        let eq18_e1113_d_n12: f64 = ((eq18_e1111_d_n12 * s.v[812]) + (eq18_e1111 * s.dn[812][12]));
        let eq18_e1113_d_n13: f64 = ((eq18_e1111_d_n13 * s.v[812]) + (eq18_e1111 * s.dn[812][13]));
        let eq18_e1113_d_n14: f64 = ((eq18_e1111_d_n14 * s.v[812]) + (eq18_e1111 * s.dn[812][14]));
        let eq18_e1113_d_n15: f64 = ((eq18_e1111_d_n15 * s.v[812]) + (eq18_e1111 * s.dn[812][15]));
        let eq18_e1113_d_n16: f64 = ((eq18_e1111_d_n16 * s.v[812]) + (eq18_e1111 * s.dn[812][16]));
        let eq18_e1113_d_n17: f64 = ((eq18_e1111_d_n17 * s.v[812]) + (eq18_e1111 * s.dn[812][17]));
        let eq18_e1113_d_n18: f64 = ((eq18_e1111_d_n18 * s.v[812]) + (eq18_e1111 * s.dn[812][18]));
        let eq18_e1113_d_n19: f64 = ((eq18_e1111_d_n19 * s.v[812]) + (eq18_e1111 * s.dn[812][19]));
        let eq18_e1113_d_n20: f64 = ((eq18_e1111_d_n20 * s.v[812]) + (eq18_e1111 * s.dn[812][20]));
        let eq18_e1113_d_b0: f64 = ((eq18_e1111_d_b0 * s.v[812]) + (eq18_e1111 * s.db[812][0]));
        let eq18_e1113_d_b1: f64 = ((eq18_e1111_d_b1 * s.v[812]) + (eq18_e1111 * s.db[812][1]));
        let eq18_e1113_d_b2: f64 = ((eq18_e1111_d_b2 * s.v[812]) + (eq18_e1111 * s.db[812][2]));
        let eq18_e1113_d_b3: f64 = ((eq18_e1111_d_b3 * s.v[812]) + (eq18_e1111 * s.db[812][3]));
        let eq18_e1113_d_b4: f64 = ((eq18_e1111_d_b4 * s.v[812]) + (eq18_e1111 * s.db[812][4]));
        let eq18_e1113_d_b5: f64 = ((eq18_e1111_d_b5 * s.v[812]) + (eq18_e1111 * s.db[812][5]));
        let eq18_e1113_d_b6: f64 = ((eq18_e1111_d_b6 * s.v[812]) + (eq18_e1111 * s.db[812][6]));
        let eq18_e1113_d_b7: f64 = ((eq18_e1111_d_b7 * s.v[812]) + (eq18_e1111 * s.db[812][7]));
        let eq18_e1113_d_b8: f64 = ((eq18_e1111_d_b8 * s.v[812]) + (eq18_e1111 * s.db[812][8]));
        let eq18_e1113_d_b9: f64 = ((eq18_e1111_d_b9 * s.v[812]) + (eq18_e1111 * s.db[812][9]));
        let eq18_e1113_d_b10: f64 = ((eq18_e1111_d_b10 * s.v[812]) + (eq18_e1111 * s.db[812][10]));
        let eq18_e1113_d_b11: f64 = ((eq18_e1111_d_b11 * s.v[812]) + (eq18_e1111 * s.db[812][11]));
        let eq18_e1113_d_b12: f64 = ((eq18_e1111_d_b12 * s.v[812]) + (eq18_e1111 * s.db[812][12]));
        let eq18_e1113_d_b13: f64 = ((eq18_e1111_d_b13 * s.v[812]) + (eq18_e1111 * s.db[812][13]));
        let eq18_e1113_d_b14: f64 = ((eq18_e1111_d_b14 * s.v[812]) + (eq18_e1111 * s.db[812][14]));
        let eq18_e1113_d_b15: f64 = ((eq18_e1111_d_b15 * s.v[812]) + (eq18_e1111 * s.db[812][15]));
        let eq18_e1113_d_b16: f64 = ((eq18_e1111_d_b16 * s.v[812]) + (eq18_e1111 * s.db[812][16]));
        let eq18_e1113_d_b17: f64 = ((eq18_e1111_d_b17 * s.v[812]) + (eq18_e1111 * s.db[812][17]));
        let eq18_e1113_d_b18: f64 = ((eq18_e1111_d_b18 * s.v[812]) + (eq18_e1111 * s.db[812][18]));
        let eq18_e1113_d_b19: f64 = ((eq18_e1111_d_b19 * s.v[812]) + (eq18_e1111 * s.db[812][19]));
        let eq18_e1113_d_b20: f64 = ((eq18_e1111_d_b20 * s.v[812]) + (eq18_e1111 * s.db[812][20]));
        let eq18_e1113_d_b21: f64 = ((eq18_e1111_d_b21 * s.v[812]) + (eq18_e1111 * s.db[812][21]));
        let eq18_e1113_d_b22: f64 = ((eq18_e1111_d_b22 * s.v[812]) + (eq18_e1111 * s.db[812][22]));
        let eq18_e1113_d_b23: f64 = ((eq18_e1111_d_b23 * s.v[812]) + (eq18_e1111 * s.db[812][23]));
        let eq18_e1113_d_b24: f64 = ((eq18_e1111_d_b24 * s.v[812]) + (eq18_e1111 * s.db[812][24]));
        let eq18_e1115: f64 = (eq18_e1113 * (nv2 - nv6));
        let eq18_e1115_d_n0: f64 = (eq18_e1113_d_n0 * (nv2 - nv6));
        let eq18_e1115_d_n1: f64 = (eq18_e1113_d_n1 * (nv2 - nv6));
        let eq18_e1115_d_n2: f64 = ((eq18_e1113_d_n2 * (nv2 - nv6)) + eq18_e1113);
        let eq18_e1115_d_n3: f64 = (eq18_e1113_d_n3 * (nv2 - nv6));
        let eq18_e1115_d_n4: f64 = (eq18_e1113_d_n4 * (nv2 - nv6));
        let eq18_e1115_d_n5: f64 = (eq18_e1113_d_n5 * (nv2 - nv6));
        let eq18_e1115_d_n6: f64 = ((eq18_e1113_d_n6 * (nv2 - nv6)) + (-eq18_e1113));
        let eq18_e1115_d_n7: f64 = (eq18_e1113_d_n7 * (nv2 - nv6));
        let eq18_e1115_d_n8: f64 = (eq18_e1113_d_n8 * (nv2 - nv6));
        let eq18_e1115_d_n9: f64 = (eq18_e1113_d_n9 * (nv2 - nv6));
        let eq18_e1115_d_n10: f64 = (eq18_e1113_d_n10 * (nv2 - nv6));
        let eq18_e1115_d_n11: f64 = (eq18_e1113_d_n11 * (nv2 - nv6));
        let eq18_e1115_d_n12: f64 = (eq18_e1113_d_n12 * (nv2 - nv6));
        let eq18_e1115_d_n13: f64 = (eq18_e1113_d_n13 * (nv2 - nv6));
        let eq18_e1115_d_n14: f64 = (eq18_e1113_d_n14 * (nv2 - nv6));
        let eq18_e1115_d_n15: f64 = (eq18_e1113_d_n15 * (nv2 - nv6));
        let eq18_e1115_d_n16: f64 = (eq18_e1113_d_n16 * (nv2 - nv6));
        let eq18_e1115_d_n17: f64 = (eq18_e1113_d_n17 * (nv2 - nv6));
        let eq18_e1115_d_n18: f64 = (eq18_e1113_d_n18 * (nv2 - nv6));
        let eq18_e1115_d_n19: f64 = (eq18_e1113_d_n19 * (nv2 - nv6));
        let eq18_e1115_d_n20: f64 = (eq18_e1113_d_n20 * (nv2 - nv6));
        let eq18_e1115_d_b0: f64 = (eq18_e1113_d_b0 * (nv2 - nv6));
        let eq18_e1115_d_b1: f64 = (eq18_e1113_d_b1 * (nv2 - nv6));
        let eq18_e1115_d_b2: f64 = (eq18_e1113_d_b2 * (nv2 - nv6));
        let eq18_e1115_d_b3: f64 = (eq18_e1113_d_b3 * (nv2 - nv6));
        let eq18_e1115_d_b4: f64 = (eq18_e1113_d_b4 * (nv2 - nv6));
        let eq18_e1115_d_b5: f64 = (eq18_e1113_d_b5 * (nv2 - nv6));
        let eq18_e1115_d_b6: f64 = (eq18_e1113_d_b6 * (nv2 - nv6));
        let eq18_e1115_d_b7: f64 = (eq18_e1113_d_b7 * (nv2 - nv6));
        let eq18_e1115_d_b8: f64 = (eq18_e1113_d_b8 * (nv2 - nv6));
        let eq18_e1115_d_b9: f64 = (eq18_e1113_d_b9 * (nv2 - nv6));
        let eq18_e1115_d_b10: f64 = (eq18_e1113_d_b10 * (nv2 - nv6));
        let eq18_e1115_d_b11: f64 = (eq18_e1113_d_b11 * (nv2 - nv6));
        let eq18_e1115_d_b12: f64 = (eq18_e1113_d_b12 * (nv2 - nv6));
        let eq18_e1115_d_b13: f64 = (eq18_e1113_d_b13 * (nv2 - nv6));
        let eq18_e1115_d_b14: f64 = (eq18_e1113_d_b14 * (nv2 - nv6));
        let eq18_e1115_d_b15: f64 = (eq18_e1113_d_b15 * (nv2 - nv6));
        let eq18_e1115_d_b16: f64 = (eq18_e1113_d_b16 * (nv2 - nv6));
        let eq18_e1115_d_b17: f64 = (eq18_e1113_d_b17 * (nv2 - nv6));
        let eq18_e1115_d_b18: f64 = (eq18_e1113_d_b18 * (nv2 - nv6));
        let eq18_e1115_d_b19: f64 = (eq18_e1113_d_b19 * (nv2 - nv6));
        let eq18_e1115_d_b20: f64 = (eq18_e1113_d_b20 * (nv2 - nv6));
        let eq18_e1115_d_b21: f64 = (eq18_e1113_d_b21 * (nv2 - nv6));
        let eq18_e1115_d_b22: f64 = (eq18_e1113_d_b22 * (nv2 - nv6));
        let eq18_e1115_d_b23: f64 = (eq18_e1113_d_b23 * (nv2 - nv6));
        let eq18_e1115_d_b24: f64 = (eq18_e1113_d_b24 * (nv2 - nv6));
        (eq18_e1115, eq18_e1115_d_n0, eq18_e1115_d_n1, eq18_e1115_d_n2, eq18_e1115_d_n3, eq18_e1115_d_n4, eq18_e1115_d_n5, eq18_e1115_d_n6, eq18_e1115_d_n7, eq18_e1115_d_n8, eq18_e1115_d_n9, eq18_e1115_d_n10, eq18_e1115_d_n11, eq18_e1115_d_n12, eq18_e1115_d_n13, eq18_e1115_d_n14, eq18_e1115_d_n15, eq18_e1115_d_n16, eq18_e1115_d_n17, eq18_e1115_d_n18, eq18_e1115_d_n19, eq18_e1115_d_n20, eq18_e1115_d_b0, eq18_e1115_d_b1, eq18_e1115_d_b2, eq18_e1115_d_b3, eq18_e1115_d_b4, eq18_e1115_d_b5, eq18_e1115_d_b6, eq18_e1115_d_b7, eq18_e1115_d_b8, eq18_e1115_d_b9, eq18_e1115_d_b10, eq18_e1115_d_b11, eq18_e1115_d_b12, eq18_e1115_d_b13, eq18_e1115_d_b14, eq18_e1115_d_b15, eq18_e1115_d_b16, eq18_e1115_d_b17, eq18_e1115_d_b18, eq18_e1115_d_b19, eq18_e1115_d_b20, eq18_e1115_d_b21, eq18_e1115_d_b22, eq18_e1115_d_b23, eq18_e1115_d_b24,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1117;
        let eq18_node_derivatives: [f64; 21] = [eq18_e1117_d_n0, eq18_e1117_d_n1, eq18_e1117_d_n2, eq18_e1117_d_n3, eq18_e1117_d_n4, eq18_e1117_d_n5, eq18_e1117_d_n6, eq18_e1117_d_n7, eq18_e1117_d_n8, eq18_e1117_d_n9, eq18_e1117_d_n10, eq18_e1117_d_n11, eq18_e1117_d_n12, eq18_e1117_d_n13, eq18_e1117_d_n14, eq18_e1117_d_n15, eq18_e1117_d_n16, eq18_e1117_d_n17, eq18_e1117_d_n18, eq18_e1117_d_n19, eq18_e1117_d_n20];
        let eq18_branch_derivatives: [f64; 25] = [eq18_e1117_d_b0, eq18_e1117_d_b1, eq18_e1117_d_b2, eq18_e1117_d_b3, eq18_e1117_d_b4, eq18_e1117_d_b5, eq18_e1117_d_b6, eq18_e1117_d_b7, eq18_e1117_d_b8, eq18_e1117_d_b9, eq18_e1117_d_b10, eq18_e1117_d_b11, eq18_e1117_d_b12, eq18_e1117_d_b13, eq18_e1117_d_b14, eq18_e1117_d_b15, eq18_e1117_d_b16, eq18_e1117_d_b17, eq18_e1117_d_b18, eq18_e1117_d_b19, eq18_e1117_d_b20, eq18_e1117_d_b21, eq18_e1117_d_b22, eq18_e1117_d_b23, eq18_e1117_d_b24];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(6),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let (eq20_e1132,) = {
    if (!s.b[2915]) {
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
        let (eq21_e1142, eq21_e1142_d_n0, eq21_e1142_d_n1, eq21_e1142_d_n2, eq21_e1142_d_n3, eq21_e1142_d_n4, eq21_e1142_d_n5, eq21_e1142_d_n6, eq21_e1142_d_n7, eq21_e1142_d_n8, eq21_e1142_d_n9, eq21_e1142_d_n10, eq21_e1142_d_n11, eq21_e1142_d_n12, eq21_e1142_d_n13, eq21_e1142_d_n14, eq21_e1142_d_n15, eq21_e1142_d_n16, eq21_e1142_d_n17, eq21_e1142_d_n18, eq21_e1142_d_n19, eq21_e1142_d_n20, eq21_e1142_d_b0, eq21_e1142_d_b1, eq21_e1142_d_b2, eq21_e1142_d_b3, eq21_e1142_d_b4, eq21_e1142_d_b5, eq21_e1142_d_b6, eq21_e1142_d_b7, eq21_e1142_d_b8, eq21_e1142_d_b9, eq21_e1142_d_b10, eq21_e1142_d_b11, eq21_e1142_d_b12, eq21_e1142_d_b13, eq21_e1142_d_b14, eq21_e1142_d_b15, eq21_e1142_d_b16, eq21_e1142_d_b17, eq21_e1142_d_b18, eq21_e1142_d_b19, eq21_e1142_d_b20, eq21_e1142_d_b21, eq21_e1142_d_b22, eq21_e1142_d_b23, eq21_e1142_d_b24,) = {
    if s.b[2916] {
        let eq21_e1136: f64 = (s.v[19] * p.p32);
        let eq21_e1136_d_n0: f64 = (s.dn[19][0] * p.p32);
        let eq21_e1136_d_n1: f64 = (s.dn[19][1] * p.p32);
        let eq21_e1136_d_n2: f64 = (s.dn[19][2] * p.p32);
        let eq21_e1136_d_n3: f64 = (s.dn[19][3] * p.p32);
        let eq21_e1136_d_n4: f64 = (s.dn[19][4] * p.p32);
        let eq21_e1136_d_n5: f64 = (s.dn[19][5] * p.p32);
        let eq21_e1136_d_n6: f64 = (s.dn[19][6] * p.p32);
        let eq21_e1136_d_n7: f64 = (s.dn[19][7] * p.p32);
        let eq21_e1136_d_n8: f64 = (s.dn[19][8] * p.p32);
        let eq21_e1136_d_n9: f64 = (s.dn[19][9] * p.p32);
        let eq21_e1136_d_n10: f64 = (s.dn[19][10] * p.p32);
        let eq21_e1136_d_n11: f64 = (s.dn[19][11] * p.p32);
        let eq21_e1136_d_n12: f64 = (s.dn[19][12] * p.p32);
        let eq21_e1136_d_n13: f64 = (s.dn[19][13] * p.p32);
        let eq21_e1136_d_n14: f64 = (s.dn[19][14] * p.p32);
        let eq21_e1136_d_n15: f64 = (s.dn[19][15] * p.p32);
        let eq21_e1136_d_n16: f64 = (s.dn[19][16] * p.p32);
        let eq21_e1136_d_n17: f64 = (s.dn[19][17] * p.p32);
        let eq21_e1136_d_n18: f64 = (s.dn[19][18] * p.p32);
        let eq21_e1136_d_n19: f64 = (s.dn[19][19] * p.p32);
        let eq21_e1136_d_n20: f64 = (s.dn[19][20] * p.p32);
        let eq21_e1136_d_b0: f64 = (s.db[19][0] * p.p32);
        let eq21_e1136_d_b1: f64 = (s.db[19][1] * p.p32);
        let eq21_e1136_d_b2: f64 = (s.db[19][2] * p.p32);
        let eq21_e1136_d_b3: f64 = (s.db[19][3] * p.p32);
        let eq21_e1136_d_b4: f64 = (s.db[19][4] * p.p32);
        let eq21_e1136_d_b5: f64 = (s.db[19][5] * p.p32);
        let eq21_e1136_d_b6: f64 = (s.db[19][6] * p.p32);
        let eq21_e1136_d_b7: f64 = (s.db[19][7] * p.p32);
        let eq21_e1136_d_b8: f64 = (s.db[19][8] * p.p32);
        let eq21_e1136_d_b9: f64 = (s.db[19][9] * p.p32);
        let eq21_e1136_d_b10: f64 = (s.db[19][10] * p.p32);
        let eq21_e1136_d_b11: f64 = (s.db[19][11] * p.p32);
        let eq21_e1136_d_b12: f64 = (s.db[19][12] * p.p32);
        let eq21_e1136_d_b13: f64 = (s.db[19][13] * p.p32);
        let eq21_e1136_d_b14: f64 = (s.db[19][14] * p.p32);
        let eq21_e1136_d_b15: f64 = (s.db[19][15] * p.p32);
        let eq21_e1136_d_b16: f64 = (s.db[19][16] * p.p32);
        let eq21_e1136_d_b17: f64 = (s.db[19][17] * p.p32);
        let eq21_e1136_d_b18: f64 = (s.db[19][18] * p.p32);
        let eq21_e1136_d_b19: f64 = (s.db[19][19] * p.p32);
        let eq21_e1136_d_b20: f64 = (s.db[19][20] * p.p32);
        let eq21_e1136_d_b21: f64 = (s.db[19][21] * p.p32);
        let eq21_e1136_d_b22: f64 = (s.db[19][22] * p.p32);
        let eq21_e1136_d_b23: f64 = (s.db[19][23] * p.p32);
        let eq21_e1136_d_b24: f64 = (s.db[19][24] * p.p32);
        let eq21_e1138: f64 = (eq21_e1136 * s.v[813]);
        let eq21_e1138_d_n0: f64 = ((eq21_e1136_d_n0 * s.v[813]) + (eq21_e1136 * s.dn[813][0]));
        let eq21_e1138_d_n1: f64 = ((eq21_e1136_d_n1 * s.v[813]) + (eq21_e1136 * s.dn[813][1]));
        let eq21_e1138_d_n2: f64 = ((eq21_e1136_d_n2 * s.v[813]) + (eq21_e1136 * s.dn[813][2]));
        let eq21_e1138_d_n3: f64 = ((eq21_e1136_d_n3 * s.v[813]) + (eq21_e1136 * s.dn[813][3]));
        let eq21_e1138_d_n4: f64 = ((eq21_e1136_d_n4 * s.v[813]) + (eq21_e1136 * s.dn[813][4]));
        let eq21_e1138_d_n5: f64 = ((eq21_e1136_d_n5 * s.v[813]) + (eq21_e1136 * s.dn[813][5]));
        let eq21_e1138_d_n6: f64 = ((eq21_e1136_d_n6 * s.v[813]) + (eq21_e1136 * s.dn[813][6]));
        let eq21_e1138_d_n7: f64 = ((eq21_e1136_d_n7 * s.v[813]) + (eq21_e1136 * s.dn[813][7]));
        let eq21_e1138_d_n8: f64 = ((eq21_e1136_d_n8 * s.v[813]) + (eq21_e1136 * s.dn[813][8]));
        let eq21_e1138_d_n9: f64 = ((eq21_e1136_d_n9 * s.v[813]) + (eq21_e1136 * s.dn[813][9]));
        let eq21_e1138_d_n10: f64 = ((eq21_e1136_d_n10 * s.v[813]) + (eq21_e1136 * s.dn[813][10]));
        let eq21_e1138_d_n11: f64 = ((eq21_e1136_d_n11 * s.v[813]) + (eq21_e1136 * s.dn[813][11]));
        let eq21_e1138_d_n12: f64 = ((eq21_e1136_d_n12 * s.v[813]) + (eq21_e1136 * s.dn[813][12]));
        let eq21_e1138_d_n13: f64 = ((eq21_e1136_d_n13 * s.v[813]) + (eq21_e1136 * s.dn[813][13]));
        let eq21_e1138_d_n14: f64 = ((eq21_e1136_d_n14 * s.v[813]) + (eq21_e1136 * s.dn[813][14]));
        let eq21_e1138_d_n15: f64 = ((eq21_e1136_d_n15 * s.v[813]) + (eq21_e1136 * s.dn[813][15]));
        let eq21_e1138_d_n16: f64 = ((eq21_e1136_d_n16 * s.v[813]) + (eq21_e1136 * s.dn[813][16]));
        let eq21_e1138_d_n17: f64 = ((eq21_e1136_d_n17 * s.v[813]) + (eq21_e1136 * s.dn[813][17]));
        let eq21_e1138_d_n18: f64 = ((eq21_e1136_d_n18 * s.v[813]) + (eq21_e1136 * s.dn[813][18]));
        let eq21_e1138_d_n19: f64 = ((eq21_e1136_d_n19 * s.v[813]) + (eq21_e1136 * s.dn[813][19]));
        let eq21_e1138_d_n20: f64 = ((eq21_e1136_d_n20 * s.v[813]) + (eq21_e1136 * s.dn[813][20]));
        let eq21_e1138_d_b0: f64 = ((eq21_e1136_d_b0 * s.v[813]) + (eq21_e1136 * s.db[813][0]));
        let eq21_e1138_d_b1: f64 = ((eq21_e1136_d_b1 * s.v[813]) + (eq21_e1136 * s.db[813][1]));
        let eq21_e1138_d_b2: f64 = ((eq21_e1136_d_b2 * s.v[813]) + (eq21_e1136 * s.db[813][2]));
        let eq21_e1138_d_b3: f64 = ((eq21_e1136_d_b3 * s.v[813]) + (eq21_e1136 * s.db[813][3]));
        let eq21_e1138_d_b4: f64 = ((eq21_e1136_d_b4 * s.v[813]) + (eq21_e1136 * s.db[813][4]));
        let eq21_e1138_d_b5: f64 = ((eq21_e1136_d_b5 * s.v[813]) + (eq21_e1136 * s.db[813][5]));
        let eq21_e1138_d_b6: f64 = ((eq21_e1136_d_b6 * s.v[813]) + (eq21_e1136 * s.db[813][6]));
        let eq21_e1138_d_b7: f64 = ((eq21_e1136_d_b7 * s.v[813]) + (eq21_e1136 * s.db[813][7]));
        let eq21_e1138_d_b8: f64 = ((eq21_e1136_d_b8 * s.v[813]) + (eq21_e1136 * s.db[813][8]));
        let eq21_e1138_d_b9: f64 = ((eq21_e1136_d_b9 * s.v[813]) + (eq21_e1136 * s.db[813][9]));
        let eq21_e1138_d_b10: f64 = ((eq21_e1136_d_b10 * s.v[813]) + (eq21_e1136 * s.db[813][10]));
        let eq21_e1138_d_b11: f64 = ((eq21_e1136_d_b11 * s.v[813]) + (eq21_e1136 * s.db[813][11]));
        let eq21_e1138_d_b12: f64 = ((eq21_e1136_d_b12 * s.v[813]) + (eq21_e1136 * s.db[813][12]));
        let eq21_e1138_d_b13: f64 = ((eq21_e1136_d_b13 * s.v[813]) + (eq21_e1136 * s.db[813][13]));
        let eq21_e1138_d_b14: f64 = ((eq21_e1136_d_b14 * s.v[813]) + (eq21_e1136 * s.db[813][14]));
        let eq21_e1138_d_b15: f64 = ((eq21_e1136_d_b15 * s.v[813]) + (eq21_e1136 * s.db[813][15]));
        let eq21_e1138_d_b16: f64 = ((eq21_e1136_d_b16 * s.v[813]) + (eq21_e1136 * s.db[813][16]));
        let eq21_e1138_d_b17: f64 = ((eq21_e1136_d_b17 * s.v[813]) + (eq21_e1136 * s.db[813][17]));
        let eq21_e1138_d_b18: f64 = ((eq21_e1136_d_b18 * s.v[813]) + (eq21_e1136 * s.db[813][18]));
        let eq21_e1138_d_b19: f64 = ((eq21_e1136_d_b19 * s.v[813]) + (eq21_e1136 * s.db[813][19]));
        let eq21_e1138_d_b20: f64 = ((eq21_e1136_d_b20 * s.v[813]) + (eq21_e1136 * s.db[813][20]));
        let eq21_e1138_d_b21: f64 = ((eq21_e1136_d_b21 * s.v[813]) + (eq21_e1136 * s.db[813][21]));
        let eq21_e1138_d_b22: f64 = ((eq21_e1136_d_b22 * s.v[813]) + (eq21_e1136 * s.db[813][22]));
        let eq21_e1138_d_b23: f64 = ((eq21_e1136_d_b23 * s.v[813]) + (eq21_e1136 * s.db[813][23]));
        let eq21_e1138_d_b24: f64 = ((eq21_e1136_d_b24 * s.v[813]) + (eq21_e1136 * s.db[813][24]));
        let eq21_e1140: f64 = (eq21_e1138 * (nv0 - nv7));
        let eq21_e1140_d_n0: f64 = ((eq21_e1138_d_n0 * (nv0 - nv7)) + eq21_e1138);
        let eq21_e1140_d_n1: f64 = (eq21_e1138_d_n1 * (nv0 - nv7));
        let eq21_e1140_d_n2: f64 = (eq21_e1138_d_n2 * (nv0 - nv7));
        let eq21_e1140_d_n3: f64 = (eq21_e1138_d_n3 * (nv0 - nv7));
        let eq21_e1140_d_n4: f64 = (eq21_e1138_d_n4 * (nv0 - nv7));
        let eq21_e1140_d_n5: f64 = (eq21_e1138_d_n5 * (nv0 - nv7));
        let eq21_e1140_d_n6: f64 = (eq21_e1138_d_n6 * (nv0 - nv7));
        let eq21_e1140_d_n7: f64 = ((eq21_e1138_d_n7 * (nv0 - nv7)) + (-eq21_e1138));
        let eq21_e1140_d_n8: f64 = (eq21_e1138_d_n8 * (nv0 - nv7));
        let eq21_e1140_d_n9: f64 = (eq21_e1138_d_n9 * (nv0 - nv7));
        let eq21_e1140_d_n10: f64 = (eq21_e1138_d_n10 * (nv0 - nv7));
        let eq21_e1140_d_n11: f64 = (eq21_e1138_d_n11 * (nv0 - nv7));
        let eq21_e1140_d_n12: f64 = (eq21_e1138_d_n12 * (nv0 - nv7));
        let eq21_e1140_d_n13: f64 = (eq21_e1138_d_n13 * (nv0 - nv7));
        let eq21_e1140_d_n14: f64 = (eq21_e1138_d_n14 * (nv0 - nv7));
        let eq21_e1140_d_n15: f64 = (eq21_e1138_d_n15 * (nv0 - nv7));
        let eq21_e1140_d_n16: f64 = (eq21_e1138_d_n16 * (nv0 - nv7));
        let eq21_e1140_d_n17: f64 = (eq21_e1138_d_n17 * (nv0 - nv7));
        let eq21_e1140_d_n18: f64 = (eq21_e1138_d_n18 * (nv0 - nv7));
        let eq21_e1140_d_n19: f64 = (eq21_e1138_d_n19 * (nv0 - nv7));
        let eq21_e1140_d_n20: f64 = (eq21_e1138_d_n20 * (nv0 - nv7));
        let eq21_e1140_d_b0: f64 = (eq21_e1138_d_b0 * (nv0 - nv7));
        let eq21_e1140_d_b1: f64 = (eq21_e1138_d_b1 * (nv0 - nv7));
        let eq21_e1140_d_b2: f64 = (eq21_e1138_d_b2 * (nv0 - nv7));
        let eq21_e1140_d_b3: f64 = (eq21_e1138_d_b3 * (nv0 - nv7));
        let eq21_e1140_d_b4: f64 = (eq21_e1138_d_b4 * (nv0 - nv7));
        let eq21_e1140_d_b5: f64 = (eq21_e1138_d_b5 * (nv0 - nv7));
        let eq21_e1140_d_b6: f64 = (eq21_e1138_d_b6 * (nv0 - nv7));
        let eq21_e1140_d_b7: f64 = (eq21_e1138_d_b7 * (nv0 - nv7));
        let eq21_e1140_d_b8: f64 = (eq21_e1138_d_b8 * (nv0 - nv7));
        let eq21_e1140_d_b9: f64 = (eq21_e1138_d_b9 * (nv0 - nv7));
        let eq21_e1140_d_b10: f64 = (eq21_e1138_d_b10 * (nv0 - nv7));
        let eq21_e1140_d_b11: f64 = (eq21_e1138_d_b11 * (nv0 - nv7));
        let eq21_e1140_d_b12: f64 = (eq21_e1138_d_b12 * (nv0 - nv7));
        let eq21_e1140_d_b13: f64 = (eq21_e1138_d_b13 * (nv0 - nv7));
        let eq21_e1140_d_b14: f64 = (eq21_e1138_d_b14 * (nv0 - nv7));
        let eq21_e1140_d_b15: f64 = (eq21_e1138_d_b15 * (nv0 - nv7));
        let eq21_e1140_d_b16: f64 = (eq21_e1138_d_b16 * (nv0 - nv7));
        let eq21_e1140_d_b17: f64 = (eq21_e1138_d_b17 * (nv0 - nv7));
        let eq21_e1140_d_b18: f64 = (eq21_e1138_d_b18 * (nv0 - nv7));
        let eq21_e1140_d_b19: f64 = (eq21_e1138_d_b19 * (nv0 - nv7));
        let eq21_e1140_d_b20: f64 = (eq21_e1138_d_b20 * (nv0 - nv7));
        let eq21_e1140_d_b21: f64 = (eq21_e1138_d_b21 * (nv0 - nv7));
        let eq21_e1140_d_b22: f64 = (eq21_e1138_d_b22 * (nv0 - nv7));
        let eq21_e1140_d_b23: f64 = (eq21_e1138_d_b23 * (nv0 - nv7));
        let eq21_e1140_d_b24: f64 = (eq21_e1138_d_b24 * (nv0 - nv7));
        (eq21_e1140, eq21_e1140_d_n0, eq21_e1140_d_n1, eq21_e1140_d_n2, eq21_e1140_d_n3, eq21_e1140_d_n4, eq21_e1140_d_n5, eq21_e1140_d_n6, eq21_e1140_d_n7, eq21_e1140_d_n8, eq21_e1140_d_n9, eq21_e1140_d_n10, eq21_e1140_d_n11, eq21_e1140_d_n12, eq21_e1140_d_n13, eq21_e1140_d_n14, eq21_e1140_d_n15, eq21_e1140_d_n16, eq21_e1140_d_n17, eq21_e1140_d_n18, eq21_e1140_d_n19, eq21_e1140_d_n20, eq21_e1140_d_b0, eq21_e1140_d_b1, eq21_e1140_d_b2, eq21_e1140_d_b3, eq21_e1140_d_b4, eq21_e1140_d_b5, eq21_e1140_d_b6, eq21_e1140_d_b7, eq21_e1140_d_b8, eq21_e1140_d_b9, eq21_e1140_d_b10, eq21_e1140_d_b11, eq21_e1140_d_b12, eq21_e1140_d_b13, eq21_e1140_d_b14, eq21_e1140_d_b15, eq21_e1140_d_b16, eq21_e1140_d_b17, eq21_e1140_d_b18, eq21_e1140_d_b19, eq21_e1140_d_b20, eq21_e1140_d_b21, eq21_e1140_d_b22, eq21_e1140_d_b23, eq21_e1140_d_b24,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1142;
        let eq21_node_derivatives: [f64; 21] = [eq21_e1142_d_n0, eq21_e1142_d_n1, eq21_e1142_d_n2, eq21_e1142_d_n3, eq21_e1142_d_n4, eq21_e1142_d_n5, eq21_e1142_d_n6, eq21_e1142_d_n7, eq21_e1142_d_n8, eq21_e1142_d_n9, eq21_e1142_d_n10, eq21_e1142_d_n11, eq21_e1142_d_n12, eq21_e1142_d_n13, eq21_e1142_d_n14, eq21_e1142_d_n15, eq21_e1142_d_n16, eq21_e1142_d_n17, eq21_e1142_d_n18, eq21_e1142_d_n19, eq21_e1142_d_n20];
        let eq21_branch_derivatives: [f64; 25] = [eq21_e1142_d_b0, eq21_e1142_d_b1, eq21_e1142_d_b2, eq21_e1142_d_b3, eq21_e1142_d_b4, eq21_e1142_d_b5, eq21_e1142_d_b6, eq21_e1142_d_b7, eq21_e1142_d_b8, eq21_e1142_d_b9, eq21_e1142_d_b10, eq21_e1142_d_b11, eq21_e1142_d_b12, eq21_e1142_d_b13, eq21_e1142_d_b14, eq21_e1142_d_b15, eq21_e1142_d_b16, eq21_e1142_d_b17, eq21_e1142_d_b18, eq21_e1142_d_b19, eq21_e1142_d_b20, eq21_e1142_d_b21, eq21_e1142_d_b22, eq21_e1142_d_b23, eq21_e1142_d_b24];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(7),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let (eq23_e1157,) = {
    if (!s.b[2916]) {
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
        let (eq24_e1167, eq24_e1167_d_n0, eq24_e1167_d_n1, eq24_e1167_d_n2, eq24_e1167_d_n3, eq24_e1167_d_n4, eq24_e1167_d_n5, eq24_e1167_d_n6, eq24_e1167_d_n7, eq24_e1167_d_n8, eq24_e1167_d_n9, eq24_e1167_d_n10, eq24_e1167_d_n11, eq24_e1167_d_n12, eq24_e1167_d_n13, eq24_e1167_d_n14, eq24_e1167_d_n15, eq24_e1167_d_n16, eq24_e1167_d_n17, eq24_e1167_d_n18, eq24_e1167_d_n19, eq24_e1167_d_n20, eq24_e1167_d_b0, eq24_e1167_d_b1, eq24_e1167_d_b2, eq24_e1167_d_b3, eq24_e1167_d_b4, eq24_e1167_d_b5, eq24_e1167_d_b6, eq24_e1167_d_b7, eq24_e1167_d_b8, eq24_e1167_d_b9, eq24_e1167_d_b10, eq24_e1167_d_b11, eq24_e1167_d_b12, eq24_e1167_d_b13, eq24_e1167_d_b14, eq24_e1167_d_b15, eq24_e1167_d_b16, eq24_e1167_d_b17, eq24_e1167_d_b18, eq24_e1167_d_b19, eq24_e1167_d_b20, eq24_e1167_d_b21, eq24_e1167_d_b22, eq24_e1167_d_b23, eq24_e1167_d_b24,) = {
    if s.b[2917] {
        let eq24_e1161: f64 = (s.v[19] * p.p32);
        let eq24_e1161_d_n0: f64 = (s.dn[19][0] * p.p32);
        let eq24_e1161_d_n1: f64 = (s.dn[19][1] * p.p32);
        let eq24_e1161_d_n2: f64 = (s.dn[19][2] * p.p32);
        let eq24_e1161_d_n3: f64 = (s.dn[19][3] * p.p32);
        let eq24_e1161_d_n4: f64 = (s.dn[19][4] * p.p32);
        let eq24_e1161_d_n5: f64 = (s.dn[19][5] * p.p32);
        let eq24_e1161_d_n6: f64 = (s.dn[19][6] * p.p32);
        let eq24_e1161_d_n7: f64 = (s.dn[19][7] * p.p32);
        let eq24_e1161_d_n8: f64 = (s.dn[19][8] * p.p32);
        let eq24_e1161_d_n9: f64 = (s.dn[19][9] * p.p32);
        let eq24_e1161_d_n10: f64 = (s.dn[19][10] * p.p32);
        let eq24_e1161_d_n11: f64 = (s.dn[19][11] * p.p32);
        let eq24_e1161_d_n12: f64 = (s.dn[19][12] * p.p32);
        let eq24_e1161_d_n13: f64 = (s.dn[19][13] * p.p32);
        let eq24_e1161_d_n14: f64 = (s.dn[19][14] * p.p32);
        let eq24_e1161_d_n15: f64 = (s.dn[19][15] * p.p32);
        let eq24_e1161_d_n16: f64 = (s.dn[19][16] * p.p32);
        let eq24_e1161_d_n17: f64 = (s.dn[19][17] * p.p32);
        let eq24_e1161_d_n18: f64 = (s.dn[19][18] * p.p32);
        let eq24_e1161_d_n19: f64 = (s.dn[19][19] * p.p32);
        let eq24_e1161_d_n20: f64 = (s.dn[19][20] * p.p32);
        let eq24_e1161_d_b0: f64 = (s.db[19][0] * p.p32);
        let eq24_e1161_d_b1: f64 = (s.db[19][1] * p.p32);
        let eq24_e1161_d_b2: f64 = (s.db[19][2] * p.p32);
        let eq24_e1161_d_b3: f64 = (s.db[19][3] * p.p32);
        let eq24_e1161_d_b4: f64 = (s.db[19][4] * p.p32);
        let eq24_e1161_d_b5: f64 = (s.db[19][5] * p.p32);
        let eq24_e1161_d_b6: f64 = (s.db[19][6] * p.p32);
        let eq24_e1161_d_b7: f64 = (s.db[19][7] * p.p32);
        let eq24_e1161_d_b8: f64 = (s.db[19][8] * p.p32);
        let eq24_e1161_d_b9: f64 = (s.db[19][9] * p.p32);
        let eq24_e1161_d_b10: f64 = (s.db[19][10] * p.p32);
        let eq24_e1161_d_b11: f64 = (s.db[19][11] * p.p32);
        let eq24_e1161_d_b12: f64 = (s.db[19][12] * p.p32);
        let eq24_e1161_d_b13: f64 = (s.db[19][13] * p.p32);
        let eq24_e1161_d_b14: f64 = (s.db[19][14] * p.p32);
        let eq24_e1161_d_b15: f64 = (s.db[19][15] * p.p32);
        let eq24_e1161_d_b16: f64 = (s.db[19][16] * p.p32);
        let eq24_e1161_d_b17: f64 = (s.db[19][17] * p.p32);
        let eq24_e1161_d_b18: f64 = (s.db[19][18] * p.p32);
        let eq24_e1161_d_b19: f64 = (s.db[19][19] * p.p32);
        let eq24_e1161_d_b20: f64 = (s.db[19][20] * p.p32);
        let eq24_e1161_d_b21: f64 = (s.db[19][21] * p.p32);
        let eq24_e1161_d_b22: f64 = (s.db[19][22] * p.p32);
        let eq24_e1161_d_b23: f64 = (s.db[19][23] * p.p32);
        let eq24_e1161_d_b24: f64 = (s.db[19][24] * p.p32);
        let eq24_e1163: f64 = (eq24_e1161 * s.v[814]);
        let eq24_e1163_d_n0: f64 = ((eq24_e1161_d_n0 * s.v[814]) + (eq24_e1161 * s.dn[814][0]));
        let eq24_e1163_d_n1: f64 = ((eq24_e1161_d_n1 * s.v[814]) + (eq24_e1161 * s.dn[814][1]));
        let eq24_e1163_d_n2: f64 = ((eq24_e1161_d_n2 * s.v[814]) + (eq24_e1161 * s.dn[814][2]));
        let eq24_e1163_d_n3: f64 = ((eq24_e1161_d_n3 * s.v[814]) + (eq24_e1161 * s.dn[814][3]));
        let eq24_e1163_d_n4: f64 = ((eq24_e1161_d_n4 * s.v[814]) + (eq24_e1161 * s.dn[814][4]));
        let eq24_e1163_d_n5: f64 = ((eq24_e1161_d_n5 * s.v[814]) + (eq24_e1161 * s.dn[814][5]));
        let eq24_e1163_d_n6: f64 = ((eq24_e1161_d_n6 * s.v[814]) + (eq24_e1161 * s.dn[814][6]));
        let eq24_e1163_d_n7: f64 = ((eq24_e1161_d_n7 * s.v[814]) + (eq24_e1161 * s.dn[814][7]));
        let eq24_e1163_d_n8: f64 = ((eq24_e1161_d_n8 * s.v[814]) + (eq24_e1161 * s.dn[814][8]));
        let eq24_e1163_d_n9: f64 = ((eq24_e1161_d_n9 * s.v[814]) + (eq24_e1161 * s.dn[814][9]));
        let eq24_e1163_d_n10: f64 = ((eq24_e1161_d_n10 * s.v[814]) + (eq24_e1161 * s.dn[814][10]));
        let eq24_e1163_d_n11: f64 = ((eq24_e1161_d_n11 * s.v[814]) + (eq24_e1161 * s.dn[814][11]));
        let eq24_e1163_d_n12: f64 = ((eq24_e1161_d_n12 * s.v[814]) + (eq24_e1161 * s.dn[814][12]));
        let eq24_e1163_d_n13: f64 = ((eq24_e1161_d_n13 * s.v[814]) + (eq24_e1161 * s.dn[814][13]));
        let eq24_e1163_d_n14: f64 = ((eq24_e1161_d_n14 * s.v[814]) + (eq24_e1161 * s.dn[814][14]));
        let eq24_e1163_d_n15: f64 = ((eq24_e1161_d_n15 * s.v[814]) + (eq24_e1161 * s.dn[814][15]));
        let eq24_e1163_d_n16: f64 = ((eq24_e1161_d_n16 * s.v[814]) + (eq24_e1161 * s.dn[814][16]));
        let eq24_e1163_d_n17: f64 = ((eq24_e1161_d_n17 * s.v[814]) + (eq24_e1161 * s.dn[814][17]));
        let eq24_e1163_d_n18: f64 = ((eq24_e1161_d_n18 * s.v[814]) + (eq24_e1161 * s.dn[814][18]));
        let eq24_e1163_d_n19: f64 = ((eq24_e1161_d_n19 * s.v[814]) + (eq24_e1161 * s.dn[814][19]));
        let eq24_e1163_d_n20: f64 = ((eq24_e1161_d_n20 * s.v[814]) + (eq24_e1161 * s.dn[814][20]));
        let eq24_e1163_d_b0: f64 = ((eq24_e1161_d_b0 * s.v[814]) + (eq24_e1161 * s.db[814][0]));
        let eq24_e1163_d_b1: f64 = ((eq24_e1161_d_b1 * s.v[814]) + (eq24_e1161 * s.db[814][1]));
        let eq24_e1163_d_b2: f64 = ((eq24_e1161_d_b2 * s.v[814]) + (eq24_e1161 * s.db[814][2]));
        let eq24_e1163_d_b3: f64 = ((eq24_e1161_d_b3 * s.v[814]) + (eq24_e1161 * s.db[814][3]));
        let eq24_e1163_d_b4: f64 = ((eq24_e1161_d_b4 * s.v[814]) + (eq24_e1161 * s.db[814][4]));
        let eq24_e1163_d_b5: f64 = ((eq24_e1161_d_b5 * s.v[814]) + (eq24_e1161 * s.db[814][5]));
        let eq24_e1163_d_b6: f64 = ((eq24_e1161_d_b6 * s.v[814]) + (eq24_e1161 * s.db[814][6]));
        let eq24_e1163_d_b7: f64 = ((eq24_e1161_d_b7 * s.v[814]) + (eq24_e1161 * s.db[814][7]));
        let eq24_e1163_d_b8: f64 = ((eq24_e1161_d_b8 * s.v[814]) + (eq24_e1161 * s.db[814][8]));
        let eq24_e1163_d_b9: f64 = ((eq24_e1161_d_b9 * s.v[814]) + (eq24_e1161 * s.db[814][9]));
        let eq24_e1163_d_b10: f64 = ((eq24_e1161_d_b10 * s.v[814]) + (eq24_e1161 * s.db[814][10]));
        let eq24_e1163_d_b11: f64 = ((eq24_e1161_d_b11 * s.v[814]) + (eq24_e1161 * s.db[814][11]));
        let eq24_e1163_d_b12: f64 = ((eq24_e1161_d_b12 * s.v[814]) + (eq24_e1161 * s.db[814][12]));
        let eq24_e1163_d_b13: f64 = ((eq24_e1161_d_b13 * s.v[814]) + (eq24_e1161 * s.db[814][13]));
        let eq24_e1163_d_b14: f64 = ((eq24_e1161_d_b14 * s.v[814]) + (eq24_e1161 * s.db[814][14]));
        let eq24_e1163_d_b15: f64 = ((eq24_e1161_d_b15 * s.v[814]) + (eq24_e1161 * s.db[814][15]));
        let eq24_e1163_d_b16: f64 = ((eq24_e1161_d_b16 * s.v[814]) + (eq24_e1161 * s.db[814][16]));
        let eq24_e1163_d_b17: f64 = ((eq24_e1161_d_b17 * s.v[814]) + (eq24_e1161 * s.db[814][17]));
        let eq24_e1163_d_b18: f64 = ((eq24_e1161_d_b18 * s.v[814]) + (eq24_e1161 * s.db[814][18]));
        let eq24_e1163_d_b19: f64 = ((eq24_e1161_d_b19 * s.v[814]) + (eq24_e1161 * s.db[814][19]));
        let eq24_e1163_d_b20: f64 = ((eq24_e1161_d_b20 * s.v[814]) + (eq24_e1161 * s.db[814][20]));
        let eq24_e1163_d_b21: f64 = ((eq24_e1161_d_b21 * s.v[814]) + (eq24_e1161 * s.db[814][21]));
        let eq24_e1163_d_b22: f64 = ((eq24_e1161_d_b22 * s.v[814]) + (eq24_e1161 * s.db[814][22]));
        let eq24_e1163_d_b23: f64 = ((eq24_e1161_d_b23 * s.v[814]) + (eq24_e1161 * s.db[814][23]));
        let eq24_e1163_d_b24: f64 = ((eq24_e1161_d_b24 * s.v[814]) + (eq24_e1161 * s.db[814][24]));
        let eq24_e1165: f64 = (eq24_e1163 * (nv8 - nv9));
        let eq24_e1165_d_n0: f64 = (eq24_e1163_d_n0 * (nv8 - nv9));
        let eq24_e1165_d_n1: f64 = (eq24_e1163_d_n1 * (nv8 - nv9));
        let eq24_e1165_d_n2: f64 = (eq24_e1163_d_n2 * (nv8 - nv9));
        let eq24_e1165_d_n3: f64 = (eq24_e1163_d_n3 * (nv8 - nv9));
        let eq24_e1165_d_n4: f64 = (eq24_e1163_d_n4 * (nv8 - nv9));
        let eq24_e1165_d_n5: f64 = (eq24_e1163_d_n5 * (nv8 - nv9));
        let eq24_e1165_d_n6: f64 = (eq24_e1163_d_n6 * (nv8 - nv9));
        let eq24_e1165_d_n7: f64 = (eq24_e1163_d_n7 * (nv8 - nv9));
        let eq24_e1165_d_n8: f64 = ((eq24_e1163_d_n8 * (nv8 - nv9)) + eq24_e1163);
        let eq24_e1165_d_n9: f64 = ((eq24_e1163_d_n9 * (nv8 - nv9)) + (-eq24_e1163));
        let eq24_e1165_d_n10: f64 = (eq24_e1163_d_n10 * (nv8 - nv9));
        let eq24_e1165_d_n11: f64 = (eq24_e1163_d_n11 * (nv8 - nv9));
        let eq24_e1165_d_n12: f64 = (eq24_e1163_d_n12 * (nv8 - nv9));
        let eq24_e1165_d_n13: f64 = (eq24_e1163_d_n13 * (nv8 - nv9));
        let eq24_e1165_d_n14: f64 = (eq24_e1163_d_n14 * (nv8 - nv9));
        let eq24_e1165_d_n15: f64 = (eq24_e1163_d_n15 * (nv8 - nv9));
        let eq24_e1165_d_n16: f64 = (eq24_e1163_d_n16 * (nv8 - nv9));
        let eq24_e1165_d_n17: f64 = (eq24_e1163_d_n17 * (nv8 - nv9));
        let eq24_e1165_d_n18: f64 = (eq24_e1163_d_n18 * (nv8 - nv9));
        let eq24_e1165_d_n19: f64 = (eq24_e1163_d_n19 * (nv8 - nv9));
        let eq24_e1165_d_n20: f64 = (eq24_e1163_d_n20 * (nv8 - nv9));
        let eq24_e1165_d_b0: f64 = (eq24_e1163_d_b0 * (nv8 - nv9));
        let eq24_e1165_d_b1: f64 = (eq24_e1163_d_b1 * (nv8 - nv9));
        let eq24_e1165_d_b2: f64 = (eq24_e1163_d_b2 * (nv8 - nv9));
        let eq24_e1165_d_b3: f64 = (eq24_e1163_d_b3 * (nv8 - nv9));
        let eq24_e1165_d_b4: f64 = (eq24_e1163_d_b4 * (nv8 - nv9));
        let eq24_e1165_d_b5: f64 = (eq24_e1163_d_b5 * (nv8 - nv9));
        let eq24_e1165_d_b6: f64 = (eq24_e1163_d_b6 * (nv8 - nv9));
        let eq24_e1165_d_b7: f64 = (eq24_e1163_d_b7 * (nv8 - nv9));
        let eq24_e1165_d_b8: f64 = (eq24_e1163_d_b8 * (nv8 - nv9));
        let eq24_e1165_d_b9: f64 = (eq24_e1163_d_b9 * (nv8 - nv9));
        let eq24_e1165_d_b10: f64 = (eq24_e1163_d_b10 * (nv8 - nv9));
        let eq24_e1165_d_b11: f64 = (eq24_e1163_d_b11 * (nv8 - nv9));
        let eq24_e1165_d_b12: f64 = (eq24_e1163_d_b12 * (nv8 - nv9));
        let eq24_e1165_d_b13: f64 = (eq24_e1163_d_b13 * (nv8 - nv9));
        let eq24_e1165_d_b14: f64 = (eq24_e1163_d_b14 * (nv8 - nv9));
        let eq24_e1165_d_b15: f64 = (eq24_e1163_d_b15 * (nv8 - nv9));
        let eq24_e1165_d_b16: f64 = (eq24_e1163_d_b16 * (nv8 - nv9));
        let eq24_e1165_d_b17: f64 = (eq24_e1163_d_b17 * (nv8 - nv9));
        let eq24_e1165_d_b18: f64 = (eq24_e1163_d_b18 * (nv8 - nv9));
        let eq24_e1165_d_b19: f64 = (eq24_e1163_d_b19 * (nv8 - nv9));
        let eq24_e1165_d_b20: f64 = (eq24_e1163_d_b20 * (nv8 - nv9));
        let eq24_e1165_d_b21: f64 = (eq24_e1163_d_b21 * (nv8 - nv9));
        let eq24_e1165_d_b22: f64 = (eq24_e1163_d_b22 * (nv8 - nv9));
        let eq24_e1165_d_b23: f64 = (eq24_e1163_d_b23 * (nv8 - nv9));
        let eq24_e1165_d_b24: f64 = (eq24_e1163_d_b24 * (nv8 - nv9));
        (eq24_e1165, eq24_e1165_d_n0, eq24_e1165_d_n1, eq24_e1165_d_n2, eq24_e1165_d_n3, eq24_e1165_d_n4, eq24_e1165_d_n5, eq24_e1165_d_n6, eq24_e1165_d_n7, eq24_e1165_d_n8, eq24_e1165_d_n9, eq24_e1165_d_n10, eq24_e1165_d_n11, eq24_e1165_d_n12, eq24_e1165_d_n13, eq24_e1165_d_n14, eq24_e1165_d_n15, eq24_e1165_d_n16, eq24_e1165_d_n17, eq24_e1165_d_n18, eq24_e1165_d_n19, eq24_e1165_d_n20, eq24_e1165_d_b0, eq24_e1165_d_b1, eq24_e1165_d_b2, eq24_e1165_d_b3, eq24_e1165_d_b4, eq24_e1165_d_b5, eq24_e1165_d_b6, eq24_e1165_d_b7, eq24_e1165_d_b8, eq24_e1165_d_b9, eq24_e1165_d_b10, eq24_e1165_d_b11, eq24_e1165_d_b12, eq24_e1165_d_b13, eq24_e1165_d_b14, eq24_e1165_d_b15, eq24_e1165_d_b16, eq24_e1165_d_b17, eq24_e1165_d_b18, eq24_e1165_d_b19, eq24_e1165_d_b20, eq24_e1165_d_b21, eq24_e1165_d_b22, eq24_e1165_d_b23, eq24_e1165_d_b24,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1167;
        let eq24_node_derivatives: [f64; 21] = [eq24_e1167_d_n0, eq24_e1167_d_n1, eq24_e1167_d_n2, eq24_e1167_d_n3, eq24_e1167_d_n4, eq24_e1167_d_n5, eq24_e1167_d_n6, eq24_e1167_d_n7, eq24_e1167_d_n8, eq24_e1167_d_n9, eq24_e1167_d_n10, eq24_e1167_d_n11, eq24_e1167_d_n12, eq24_e1167_d_n13, eq24_e1167_d_n14, eq24_e1167_d_n15, eq24_e1167_d_n16, eq24_e1167_d_n17, eq24_e1167_d_n18, eq24_e1167_d_n19, eq24_e1167_d_n20];
        let eq24_branch_derivatives: [f64; 25] = [eq24_e1167_d_b0, eq24_e1167_d_b1, eq24_e1167_d_b2, eq24_e1167_d_b3, eq24_e1167_d_b4, eq24_e1167_d_b5, eq24_e1167_d_b6, eq24_e1167_d_b7, eq24_e1167_d_b8, eq24_e1167_d_b9, eq24_e1167_d_b10, eq24_e1167_d_b11, eq24_e1167_d_b12, eq24_e1167_d_b13, eq24_e1167_d_b14, eq24_e1167_d_b15, eq24_e1167_d_b16, eq24_e1167_d_b17, eq24_e1167_d_b18, eq24_e1167_d_b19, eq24_e1167_d_b20, eq24_e1167_d_b21, eq24_e1167_d_b22, eq24_e1167_d_b23, eq24_e1167_d_b24];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(9),
            multiplicity * (eq24_value),
            &eq24_node_derivatives,
            &eq24_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_7(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq26_e1182,) = {
    if (!s.b[2917]) {
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
        let (eq27_e1192, eq27_e1192_d_n0, eq27_e1192_d_n1, eq27_e1192_d_n2, eq27_e1192_d_n3, eq27_e1192_d_n4, eq27_e1192_d_n5, eq27_e1192_d_n6, eq27_e1192_d_n7, eq27_e1192_d_n8, eq27_e1192_d_n9, eq27_e1192_d_n10, eq27_e1192_d_n11, eq27_e1192_d_n12, eq27_e1192_d_n13, eq27_e1192_d_n14, eq27_e1192_d_n15, eq27_e1192_d_n16, eq27_e1192_d_n17, eq27_e1192_d_n18, eq27_e1192_d_n19, eq27_e1192_d_n20, eq27_e1192_d_b0, eq27_e1192_d_b1, eq27_e1192_d_b2, eq27_e1192_d_b3, eq27_e1192_d_b4, eq27_e1192_d_b5, eq27_e1192_d_b6, eq27_e1192_d_b7, eq27_e1192_d_b8, eq27_e1192_d_b9, eq27_e1192_d_b10, eq27_e1192_d_b11, eq27_e1192_d_b12, eq27_e1192_d_b13, eq27_e1192_d_b14, eq27_e1192_d_b15, eq27_e1192_d_b16, eq27_e1192_d_b17, eq27_e1192_d_b18, eq27_e1192_d_b19, eq27_e1192_d_b20, eq27_e1192_d_b21, eq27_e1192_d_b22, eq27_e1192_d_b23, eq27_e1192_d_b24,) = {
    if s.b[2918] {
        let eq27_e1186: f64 = (s.v[19] * p.p32);
        let eq27_e1186_d_n0: f64 = (s.dn[19][0] * p.p32);
        let eq27_e1186_d_n1: f64 = (s.dn[19][1] * p.p32);
        let eq27_e1186_d_n2: f64 = (s.dn[19][2] * p.p32);
        let eq27_e1186_d_n3: f64 = (s.dn[19][3] * p.p32);
        let eq27_e1186_d_n4: f64 = (s.dn[19][4] * p.p32);
        let eq27_e1186_d_n5: f64 = (s.dn[19][5] * p.p32);
        let eq27_e1186_d_n6: f64 = (s.dn[19][6] * p.p32);
        let eq27_e1186_d_n7: f64 = (s.dn[19][7] * p.p32);
        let eq27_e1186_d_n8: f64 = (s.dn[19][8] * p.p32);
        let eq27_e1186_d_n9: f64 = (s.dn[19][9] * p.p32);
        let eq27_e1186_d_n10: f64 = (s.dn[19][10] * p.p32);
        let eq27_e1186_d_n11: f64 = (s.dn[19][11] * p.p32);
        let eq27_e1186_d_n12: f64 = (s.dn[19][12] * p.p32);
        let eq27_e1186_d_n13: f64 = (s.dn[19][13] * p.p32);
        let eq27_e1186_d_n14: f64 = (s.dn[19][14] * p.p32);
        let eq27_e1186_d_n15: f64 = (s.dn[19][15] * p.p32);
        let eq27_e1186_d_n16: f64 = (s.dn[19][16] * p.p32);
        let eq27_e1186_d_n17: f64 = (s.dn[19][17] * p.p32);
        let eq27_e1186_d_n18: f64 = (s.dn[19][18] * p.p32);
        let eq27_e1186_d_n19: f64 = (s.dn[19][19] * p.p32);
        let eq27_e1186_d_n20: f64 = (s.dn[19][20] * p.p32);
        let eq27_e1186_d_b0: f64 = (s.db[19][0] * p.p32);
        let eq27_e1186_d_b1: f64 = (s.db[19][1] * p.p32);
        let eq27_e1186_d_b2: f64 = (s.db[19][2] * p.p32);
        let eq27_e1186_d_b3: f64 = (s.db[19][3] * p.p32);
        let eq27_e1186_d_b4: f64 = (s.db[19][4] * p.p32);
        let eq27_e1186_d_b5: f64 = (s.db[19][5] * p.p32);
        let eq27_e1186_d_b6: f64 = (s.db[19][6] * p.p32);
        let eq27_e1186_d_b7: f64 = (s.db[19][7] * p.p32);
        let eq27_e1186_d_b8: f64 = (s.db[19][8] * p.p32);
        let eq27_e1186_d_b9: f64 = (s.db[19][9] * p.p32);
        let eq27_e1186_d_b10: f64 = (s.db[19][10] * p.p32);
        let eq27_e1186_d_b11: f64 = (s.db[19][11] * p.p32);
        let eq27_e1186_d_b12: f64 = (s.db[19][12] * p.p32);
        let eq27_e1186_d_b13: f64 = (s.db[19][13] * p.p32);
        let eq27_e1186_d_b14: f64 = (s.db[19][14] * p.p32);
        let eq27_e1186_d_b15: f64 = (s.db[19][15] * p.p32);
        let eq27_e1186_d_b16: f64 = (s.db[19][16] * p.p32);
        let eq27_e1186_d_b17: f64 = (s.db[19][17] * p.p32);
        let eq27_e1186_d_b18: f64 = (s.db[19][18] * p.p32);
        let eq27_e1186_d_b19: f64 = (s.db[19][19] * p.p32);
        let eq27_e1186_d_b20: f64 = (s.db[19][20] * p.p32);
        let eq27_e1186_d_b21: f64 = (s.db[19][21] * p.p32);
        let eq27_e1186_d_b22: f64 = (s.db[19][22] * p.p32);
        let eq27_e1186_d_b23: f64 = (s.db[19][23] * p.p32);
        let eq27_e1186_d_b24: f64 = (s.db[19][24] * p.p32);
        let eq27_e1188: f64 = (eq27_e1186 * s.v[815]);
        let eq27_e1188_d_n0: f64 = ((eq27_e1186_d_n0 * s.v[815]) + (eq27_e1186 * s.dn[815][0]));
        let eq27_e1188_d_n1: f64 = ((eq27_e1186_d_n1 * s.v[815]) + (eq27_e1186 * s.dn[815][1]));
        let eq27_e1188_d_n2: f64 = ((eq27_e1186_d_n2 * s.v[815]) + (eq27_e1186 * s.dn[815][2]));
        let eq27_e1188_d_n3: f64 = ((eq27_e1186_d_n3 * s.v[815]) + (eq27_e1186 * s.dn[815][3]));
        let eq27_e1188_d_n4: f64 = ((eq27_e1186_d_n4 * s.v[815]) + (eq27_e1186 * s.dn[815][4]));
        let eq27_e1188_d_n5: f64 = ((eq27_e1186_d_n5 * s.v[815]) + (eq27_e1186 * s.dn[815][5]));
        let eq27_e1188_d_n6: f64 = ((eq27_e1186_d_n6 * s.v[815]) + (eq27_e1186 * s.dn[815][6]));
        let eq27_e1188_d_n7: f64 = ((eq27_e1186_d_n7 * s.v[815]) + (eq27_e1186 * s.dn[815][7]));
        let eq27_e1188_d_n8: f64 = ((eq27_e1186_d_n8 * s.v[815]) + (eq27_e1186 * s.dn[815][8]));
        let eq27_e1188_d_n9: f64 = ((eq27_e1186_d_n9 * s.v[815]) + (eq27_e1186 * s.dn[815][9]));
        let eq27_e1188_d_n10: f64 = ((eq27_e1186_d_n10 * s.v[815]) + (eq27_e1186 * s.dn[815][10]));
        let eq27_e1188_d_n11: f64 = ((eq27_e1186_d_n11 * s.v[815]) + (eq27_e1186 * s.dn[815][11]));
        let eq27_e1188_d_n12: f64 = ((eq27_e1186_d_n12 * s.v[815]) + (eq27_e1186 * s.dn[815][12]));
        let eq27_e1188_d_n13: f64 = ((eq27_e1186_d_n13 * s.v[815]) + (eq27_e1186 * s.dn[815][13]));
        let eq27_e1188_d_n14: f64 = ((eq27_e1186_d_n14 * s.v[815]) + (eq27_e1186 * s.dn[815][14]));
        let eq27_e1188_d_n15: f64 = ((eq27_e1186_d_n15 * s.v[815]) + (eq27_e1186 * s.dn[815][15]));
        let eq27_e1188_d_n16: f64 = ((eq27_e1186_d_n16 * s.v[815]) + (eq27_e1186 * s.dn[815][16]));
        let eq27_e1188_d_n17: f64 = ((eq27_e1186_d_n17 * s.v[815]) + (eq27_e1186 * s.dn[815][17]));
        let eq27_e1188_d_n18: f64 = ((eq27_e1186_d_n18 * s.v[815]) + (eq27_e1186 * s.dn[815][18]));
        let eq27_e1188_d_n19: f64 = ((eq27_e1186_d_n19 * s.v[815]) + (eq27_e1186 * s.dn[815][19]));
        let eq27_e1188_d_n20: f64 = ((eq27_e1186_d_n20 * s.v[815]) + (eq27_e1186 * s.dn[815][20]));
        let eq27_e1188_d_b0: f64 = ((eq27_e1186_d_b0 * s.v[815]) + (eq27_e1186 * s.db[815][0]));
        let eq27_e1188_d_b1: f64 = ((eq27_e1186_d_b1 * s.v[815]) + (eq27_e1186 * s.db[815][1]));
        let eq27_e1188_d_b2: f64 = ((eq27_e1186_d_b2 * s.v[815]) + (eq27_e1186 * s.db[815][2]));
        let eq27_e1188_d_b3: f64 = ((eq27_e1186_d_b3 * s.v[815]) + (eq27_e1186 * s.db[815][3]));
        let eq27_e1188_d_b4: f64 = ((eq27_e1186_d_b4 * s.v[815]) + (eq27_e1186 * s.db[815][4]));
        let eq27_e1188_d_b5: f64 = ((eq27_e1186_d_b5 * s.v[815]) + (eq27_e1186 * s.db[815][5]));
        let eq27_e1188_d_b6: f64 = ((eq27_e1186_d_b6 * s.v[815]) + (eq27_e1186 * s.db[815][6]));
        let eq27_e1188_d_b7: f64 = ((eq27_e1186_d_b7 * s.v[815]) + (eq27_e1186 * s.db[815][7]));
        let eq27_e1188_d_b8: f64 = ((eq27_e1186_d_b8 * s.v[815]) + (eq27_e1186 * s.db[815][8]));
        let eq27_e1188_d_b9: f64 = ((eq27_e1186_d_b9 * s.v[815]) + (eq27_e1186 * s.db[815][9]));
        let eq27_e1188_d_b10: f64 = ((eq27_e1186_d_b10 * s.v[815]) + (eq27_e1186 * s.db[815][10]));
        let eq27_e1188_d_b11: f64 = ((eq27_e1186_d_b11 * s.v[815]) + (eq27_e1186 * s.db[815][11]));
        let eq27_e1188_d_b12: f64 = ((eq27_e1186_d_b12 * s.v[815]) + (eq27_e1186 * s.db[815][12]));
        let eq27_e1188_d_b13: f64 = ((eq27_e1186_d_b13 * s.v[815]) + (eq27_e1186 * s.db[815][13]));
        let eq27_e1188_d_b14: f64 = ((eq27_e1186_d_b14 * s.v[815]) + (eq27_e1186 * s.db[815][14]));
        let eq27_e1188_d_b15: f64 = ((eq27_e1186_d_b15 * s.v[815]) + (eq27_e1186 * s.db[815][15]));
        let eq27_e1188_d_b16: f64 = ((eq27_e1186_d_b16 * s.v[815]) + (eq27_e1186 * s.db[815][16]));
        let eq27_e1188_d_b17: f64 = ((eq27_e1186_d_b17 * s.v[815]) + (eq27_e1186 * s.db[815][17]));
        let eq27_e1188_d_b18: f64 = ((eq27_e1186_d_b18 * s.v[815]) + (eq27_e1186 * s.db[815][18]));
        let eq27_e1188_d_b19: f64 = ((eq27_e1186_d_b19 * s.v[815]) + (eq27_e1186 * s.db[815][19]));
        let eq27_e1188_d_b20: f64 = ((eq27_e1186_d_b20 * s.v[815]) + (eq27_e1186 * s.db[815][20]));
        let eq27_e1188_d_b21: f64 = ((eq27_e1186_d_b21 * s.v[815]) + (eq27_e1186 * s.db[815][21]));
        let eq27_e1188_d_b22: f64 = ((eq27_e1186_d_b22 * s.v[815]) + (eq27_e1186 * s.db[815][22]));
        let eq27_e1188_d_b23: f64 = ((eq27_e1186_d_b23 * s.v[815]) + (eq27_e1186 * s.db[815][23]));
        let eq27_e1188_d_b24: f64 = ((eq27_e1186_d_b24 * s.v[815]) + (eq27_e1186 * s.db[815][24]));
        let eq27_e1190: f64 = (eq27_e1188 * (nv10 - nv9));
        let eq27_e1190_d_n0: f64 = (eq27_e1188_d_n0 * (nv10 - nv9));
        let eq27_e1190_d_n1: f64 = (eq27_e1188_d_n1 * (nv10 - nv9));
        let eq27_e1190_d_n2: f64 = (eq27_e1188_d_n2 * (nv10 - nv9));
        let eq27_e1190_d_n3: f64 = (eq27_e1188_d_n3 * (nv10 - nv9));
        let eq27_e1190_d_n4: f64 = (eq27_e1188_d_n4 * (nv10 - nv9));
        let eq27_e1190_d_n5: f64 = (eq27_e1188_d_n5 * (nv10 - nv9));
        let eq27_e1190_d_n6: f64 = (eq27_e1188_d_n6 * (nv10 - nv9));
        let eq27_e1190_d_n7: f64 = (eq27_e1188_d_n7 * (nv10 - nv9));
        let eq27_e1190_d_n8: f64 = (eq27_e1188_d_n8 * (nv10 - nv9));
        let eq27_e1190_d_n9: f64 = ((eq27_e1188_d_n9 * (nv10 - nv9)) + (-eq27_e1188));
        let eq27_e1190_d_n10: f64 = ((eq27_e1188_d_n10 * (nv10 - nv9)) + eq27_e1188);
        let eq27_e1190_d_n11: f64 = (eq27_e1188_d_n11 * (nv10 - nv9));
        let eq27_e1190_d_n12: f64 = (eq27_e1188_d_n12 * (nv10 - nv9));
        let eq27_e1190_d_n13: f64 = (eq27_e1188_d_n13 * (nv10 - nv9));
        let eq27_e1190_d_n14: f64 = (eq27_e1188_d_n14 * (nv10 - nv9));
        let eq27_e1190_d_n15: f64 = (eq27_e1188_d_n15 * (nv10 - nv9));
        let eq27_e1190_d_n16: f64 = (eq27_e1188_d_n16 * (nv10 - nv9));
        let eq27_e1190_d_n17: f64 = (eq27_e1188_d_n17 * (nv10 - nv9));
        let eq27_e1190_d_n18: f64 = (eq27_e1188_d_n18 * (nv10 - nv9));
        let eq27_e1190_d_n19: f64 = (eq27_e1188_d_n19 * (nv10 - nv9));
        let eq27_e1190_d_n20: f64 = (eq27_e1188_d_n20 * (nv10 - nv9));
        let eq27_e1190_d_b0: f64 = (eq27_e1188_d_b0 * (nv10 - nv9));
        let eq27_e1190_d_b1: f64 = (eq27_e1188_d_b1 * (nv10 - nv9));
        let eq27_e1190_d_b2: f64 = (eq27_e1188_d_b2 * (nv10 - nv9));
        let eq27_e1190_d_b3: f64 = (eq27_e1188_d_b3 * (nv10 - nv9));
        let eq27_e1190_d_b4: f64 = (eq27_e1188_d_b4 * (nv10 - nv9));
        let eq27_e1190_d_b5: f64 = (eq27_e1188_d_b5 * (nv10 - nv9));
        let eq27_e1190_d_b6: f64 = (eq27_e1188_d_b6 * (nv10 - nv9));
        let eq27_e1190_d_b7: f64 = (eq27_e1188_d_b7 * (nv10 - nv9));
        let eq27_e1190_d_b8: f64 = (eq27_e1188_d_b8 * (nv10 - nv9));
        let eq27_e1190_d_b9: f64 = (eq27_e1188_d_b9 * (nv10 - nv9));
        let eq27_e1190_d_b10: f64 = (eq27_e1188_d_b10 * (nv10 - nv9));
        let eq27_e1190_d_b11: f64 = (eq27_e1188_d_b11 * (nv10 - nv9));
        let eq27_e1190_d_b12: f64 = (eq27_e1188_d_b12 * (nv10 - nv9));
        let eq27_e1190_d_b13: f64 = (eq27_e1188_d_b13 * (nv10 - nv9));
        let eq27_e1190_d_b14: f64 = (eq27_e1188_d_b14 * (nv10 - nv9));
        let eq27_e1190_d_b15: f64 = (eq27_e1188_d_b15 * (nv10 - nv9));
        let eq27_e1190_d_b16: f64 = (eq27_e1188_d_b16 * (nv10 - nv9));
        let eq27_e1190_d_b17: f64 = (eq27_e1188_d_b17 * (nv10 - nv9));
        let eq27_e1190_d_b18: f64 = (eq27_e1188_d_b18 * (nv10 - nv9));
        let eq27_e1190_d_b19: f64 = (eq27_e1188_d_b19 * (nv10 - nv9));
        let eq27_e1190_d_b20: f64 = (eq27_e1188_d_b20 * (nv10 - nv9));
        let eq27_e1190_d_b21: f64 = (eq27_e1188_d_b21 * (nv10 - nv9));
        let eq27_e1190_d_b22: f64 = (eq27_e1188_d_b22 * (nv10 - nv9));
        let eq27_e1190_d_b23: f64 = (eq27_e1188_d_b23 * (nv10 - nv9));
        let eq27_e1190_d_b24: f64 = (eq27_e1188_d_b24 * (nv10 - nv9));
        (eq27_e1190, eq27_e1190_d_n0, eq27_e1190_d_n1, eq27_e1190_d_n2, eq27_e1190_d_n3, eq27_e1190_d_n4, eq27_e1190_d_n5, eq27_e1190_d_n6, eq27_e1190_d_n7, eq27_e1190_d_n8, eq27_e1190_d_n9, eq27_e1190_d_n10, eq27_e1190_d_n11, eq27_e1190_d_n12, eq27_e1190_d_n13, eq27_e1190_d_n14, eq27_e1190_d_n15, eq27_e1190_d_n16, eq27_e1190_d_n17, eq27_e1190_d_n18, eq27_e1190_d_n19, eq27_e1190_d_n20, eq27_e1190_d_b0, eq27_e1190_d_b1, eq27_e1190_d_b2, eq27_e1190_d_b3, eq27_e1190_d_b4, eq27_e1190_d_b5, eq27_e1190_d_b6, eq27_e1190_d_b7, eq27_e1190_d_b8, eq27_e1190_d_b9, eq27_e1190_d_b10, eq27_e1190_d_b11, eq27_e1190_d_b12, eq27_e1190_d_b13, eq27_e1190_d_b14, eq27_e1190_d_b15, eq27_e1190_d_b16, eq27_e1190_d_b17, eq27_e1190_d_b18, eq27_e1190_d_b19, eq27_e1190_d_b20, eq27_e1190_d_b21, eq27_e1190_d_b22, eq27_e1190_d_b23, eq27_e1190_d_b24,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1192;
        let eq27_node_derivatives: [f64; 21] = [eq27_e1192_d_n0, eq27_e1192_d_n1, eq27_e1192_d_n2, eq27_e1192_d_n3, eq27_e1192_d_n4, eq27_e1192_d_n5, eq27_e1192_d_n6, eq27_e1192_d_n7, eq27_e1192_d_n8, eq27_e1192_d_n9, eq27_e1192_d_n10, eq27_e1192_d_n11, eq27_e1192_d_n12, eq27_e1192_d_n13, eq27_e1192_d_n14, eq27_e1192_d_n15, eq27_e1192_d_n16, eq27_e1192_d_n17, eq27_e1192_d_n18, eq27_e1192_d_n19, eq27_e1192_d_n20];
        let eq27_branch_derivatives: [f64; 25] = [eq27_e1192_d_b0, eq27_e1192_d_b1, eq27_e1192_d_b2, eq27_e1192_d_b3, eq27_e1192_d_b4, eq27_e1192_d_b5, eq27_e1192_d_b6, eq27_e1192_d_b7, eq27_e1192_d_b8, eq27_e1192_d_b9, eq27_e1192_d_b10, eq27_e1192_d_b11, eq27_e1192_d_b12, eq27_e1192_d_b13, eq27_e1192_d_b14, eq27_e1192_d_b15, eq27_e1192_d_b16, eq27_e1192_d_b17, eq27_e1192_d_b18, eq27_e1192_d_b19, eq27_e1192_d_b20, eq27_e1192_d_b21, eq27_e1192_d_b22, eq27_e1192_d_b23, eq27_e1192_d_b24];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(9),
            multiplicity * (eq27_value),
            &eq27_node_derivatives,
            &eq27_branch_derivatives,
            multiplicity,
        );
        let (eq29_e1207,) = {
    if (!s.b[2918]) {
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
        let (eq30_e1217, eq30_e1217_d_n0, eq30_e1217_d_n1, eq30_e1217_d_n2, eq30_e1217_d_n3, eq30_e1217_d_n4, eq30_e1217_d_n5, eq30_e1217_d_n6, eq30_e1217_d_n7, eq30_e1217_d_n8, eq30_e1217_d_n9, eq30_e1217_d_n10, eq30_e1217_d_n11, eq30_e1217_d_n12, eq30_e1217_d_n13, eq30_e1217_d_n14, eq30_e1217_d_n15, eq30_e1217_d_n16, eq30_e1217_d_n17, eq30_e1217_d_n18, eq30_e1217_d_n19, eq30_e1217_d_n20, eq30_e1217_d_b0, eq30_e1217_d_b1, eq30_e1217_d_b2, eq30_e1217_d_b3, eq30_e1217_d_b4, eq30_e1217_d_b5, eq30_e1217_d_b6, eq30_e1217_d_b7, eq30_e1217_d_b8, eq30_e1217_d_b9, eq30_e1217_d_b10, eq30_e1217_d_b11, eq30_e1217_d_b12, eq30_e1217_d_b13, eq30_e1217_d_b14, eq30_e1217_d_b15, eq30_e1217_d_b16, eq30_e1217_d_b17, eq30_e1217_d_b18, eq30_e1217_d_b19, eq30_e1217_d_b20, eq30_e1217_d_b21, eq30_e1217_d_b22, eq30_e1217_d_b23, eq30_e1217_d_b24,) = {
    if s.b[2919] {
        let eq30_e1211: f64 = (s.v[19] * p.p32);
        let eq30_e1211_d_n0: f64 = (s.dn[19][0] * p.p32);
        let eq30_e1211_d_n1: f64 = (s.dn[19][1] * p.p32);
        let eq30_e1211_d_n2: f64 = (s.dn[19][2] * p.p32);
        let eq30_e1211_d_n3: f64 = (s.dn[19][3] * p.p32);
        let eq30_e1211_d_n4: f64 = (s.dn[19][4] * p.p32);
        let eq30_e1211_d_n5: f64 = (s.dn[19][5] * p.p32);
        let eq30_e1211_d_n6: f64 = (s.dn[19][6] * p.p32);
        let eq30_e1211_d_n7: f64 = (s.dn[19][7] * p.p32);
        let eq30_e1211_d_n8: f64 = (s.dn[19][8] * p.p32);
        let eq30_e1211_d_n9: f64 = (s.dn[19][9] * p.p32);
        let eq30_e1211_d_n10: f64 = (s.dn[19][10] * p.p32);
        let eq30_e1211_d_n11: f64 = (s.dn[19][11] * p.p32);
        let eq30_e1211_d_n12: f64 = (s.dn[19][12] * p.p32);
        let eq30_e1211_d_n13: f64 = (s.dn[19][13] * p.p32);
        let eq30_e1211_d_n14: f64 = (s.dn[19][14] * p.p32);
        let eq30_e1211_d_n15: f64 = (s.dn[19][15] * p.p32);
        let eq30_e1211_d_n16: f64 = (s.dn[19][16] * p.p32);
        let eq30_e1211_d_n17: f64 = (s.dn[19][17] * p.p32);
        let eq30_e1211_d_n18: f64 = (s.dn[19][18] * p.p32);
        let eq30_e1211_d_n19: f64 = (s.dn[19][19] * p.p32);
        let eq30_e1211_d_n20: f64 = (s.dn[19][20] * p.p32);
        let eq30_e1211_d_b0: f64 = (s.db[19][0] * p.p32);
        let eq30_e1211_d_b1: f64 = (s.db[19][1] * p.p32);
        let eq30_e1211_d_b2: f64 = (s.db[19][2] * p.p32);
        let eq30_e1211_d_b3: f64 = (s.db[19][3] * p.p32);
        let eq30_e1211_d_b4: f64 = (s.db[19][4] * p.p32);
        let eq30_e1211_d_b5: f64 = (s.db[19][5] * p.p32);
        let eq30_e1211_d_b6: f64 = (s.db[19][6] * p.p32);
        let eq30_e1211_d_b7: f64 = (s.db[19][7] * p.p32);
        let eq30_e1211_d_b8: f64 = (s.db[19][8] * p.p32);
        let eq30_e1211_d_b9: f64 = (s.db[19][9] * p.p32);
        let eq30_e1211_d_b10: f64 = (s.db[19][10] * p.p32);
        let eq30_e1211_d_b11: f64 = (s.db[19][11] * p.p32);
        let eq30_e1211_d_b12: f64 = (s.db[19][12] * p.p32);
        let eq30_e1211_d_b13: f64 = (s.db[19][13] * p.p32);
        let eq30_e1211_d_b14: f64 = (s.db[19][14] * p.p32);
        let eq30_e1211_d_b15: f64 = (s.db[19][15] * p.p32);
        let eq30_e1211_d_b16: f64 = (s.db[19][16] * p.p32);
        let eq30_e1211_d_b17: f64 = (s.db[19][17] * p.p32);
        let eq30_e1211_d_b18: f64 = (s.db[19][18] * p.p32);
        let eq30_e1211_d_b19: f64 = (s.db[19][19] * p.p32);
        let eq30_e1211_d_b20: f64 = (s.db[19][20] * p.p32);
        let eq30_e1211_d_b21: f64 = (s.db[19][21] * p.p32);
        let eq30_e1211_d_b22: f64 = (s.db[19][22] * p.p32);
        let eq30_e1211_d_b23: f64 = (s.db[19][23] * p.p32);
        let eq30_e1211_d_b24: f64 = (s.db[19][24] * p.p32);
        let eq30_e1213: f64 = (eq30_e1211 * s.v[816]);
        let eq30_e1213_d_n0: f64 = ((eq30_e1211_d_n0 * s.v[816]) + (eq30_e1211 * s.dn[816][0]));
        let eq30_e1213_d_n1: f64 = ((eq30_e1211_d_n1 * s.v[816]) + (eq30_e1211 * s.dn[816][1]));
        let eq30_e1213_d_n2: f64 = ((eq30_e1211_d_n2 * s.v[816]) + (eq30_e1211 * s.dn[816][2]));
        let eq30_e1213_d_n3: f64 = ((eq30_e1211_d_n3 * s.v[816]) + (eq30_e1211 * s.dn[816][3]));
        let eq30_e1213_d_n4: f64 = ((eq30_e1211_d_n4 * s.v[816]) + (eq30_e1211 * s.dn[816][4]));
        let eq30_e1213_d_n5: f64 = ((eq30_e1211_d_n5 * s.v[816]) + (eq30_e1211 * s.dn[816][5]));
        let eq30_e1213_d_n6: f64 = ((eq30_e1211_d_n6 * s.v[816]) + (eq30_e1211 * s.dn[816][6]));
        let eq30_e1213_d_n7: f64 = ((eq30_e1211_d_n7 * s.v[816]) + (eq30_e1211 * s.dn[816][7]));
        let eq30_e1213_d_n8: f64 = ((eq30_e1211_d_n8 * s.v[816]) + (eq30_e1211 * s.dn[816][8]));
        let eq30_e1213_d_n9: f64 = ((eq30_e1211_d_n9 * s.v[816]) + (eq30_e1211 * s.dn[816][9]));
        let eq30_e1213_d_n10: f64 = ((eq30_e1211_d_n10 * s.v[816]) + (eq30_e1211 * s.dn[816][10]));
        let eq30_e1213_d_n11: f64 = ((eq30_e1211_d_n11 * s.v[816]) + (eq30_e1211 * s.dn[816][11]));
        let eq30_e1213_d_n12: f64 = ((eq30_e1211_d_n12 * s.v[816]) + (eq30_e1211 * s.dn[816][12]));
        let eq30_e1213_d_n13: f64 = ((eq30_e1211_d_n13 * s.v[816]) + (eq30_e1211 * s.dn[816][13]));
        let eq30_e1213_d_n14: f64 = ((eq30_e1211_d_n14 * s.v[816]) + (eq30_e1211 * s.dn[816][14]));
        let eq30_e1213_d_n15: f64 = ((eq30_e1211_d_n15 * s.v[816]) + (eq30_e1211 * s.dn[816][15]));
        let eq30_e1213_d_n16: f64 = ((eq30_e1211_d_n16 * s.v[816]) + (eq30_e1211 * s.dn[816][16]));
        let eq30_e1213_d_n17: f64 = ((eq30_e1211_d_n17 * s.v[816]) + (eq30_e1211 * s.dn[816][17]));
        let eq30_e1213_d_n18: f64 = ((eq30_e1211_d_n18 * s.v[816]) + (eq30_e1211 * s.dn[816][18]));
        let eq30_e1213_d_n19: f64 = ((eq30_e1211_d_n19 * s.v[816]) + (eq30_e1211 * s.dn[816][19]));
        let eq30_e1213_d_n20: f64 = ((eq30_e1211_d_n20 * s.v[816]) + (eq30_e1211 * s.dn[816][20]));
        let eq30_e1213_d_b0: f64 = ((eq30_e1211_d_b0 * s.v[816]) + (eq30_e1211 * s.db[816][0]));
        let eq30_e1213_d_b1: f64 = ((eq30_e1211_d_b1 * s.v[816]) + (eq30_e1211 * s.db[816][1]));
        let eq30_e1213_d_b2: f64 = ((eq30_e1211_d_b2 * s.v[816]) + (eq30_e1211 * s.db[816][2]));
        let eq30_e1213_d_b3: f64 = ((eq30_e1211_d_b3 * s.v[816]) + (eq30_e1211 * s.db[816][3]));
        let eq30_e1213_d_b4: f64 = ((eq30_e1211_d_b4 * s.v[816]) + (eq30_e1211 * s.db[816][4]));
        let eq30_e1213_d_b5: f64 = ((eq30_e1211_d_b5 * s.v[816]) + (eq30_e1211 * s.db[816][5]));
        let eq30_e1213_d_b6: f64 = ((eq30_e1211_d_b6 * s.v[816]) + (eq30_e1211 * s.db[816][6]));
        let eq30_e1213_d_b7: f64 = ((eq30_e1211_d_b7 * s.v[816]) + (eq30_e1211 * s.db[816][7]));
        let eq30_e1213_d_b8: f64 = ((eq30_e1211_d_b8 * s.v[816]) + (eq30_e1211 * s.db[816][8]));
        let eq30_e1213_d_b9: f64 = ((eq30_e1211_d_b9 * s.v[816]) + (eq30_e1211 * s.db[816][9]));
        let eq30_e1213_d_b10: f64 = ((eq30_e1211_d_b10 * s.v[816]) + (eq30_e1211 * s.db[816][10]));
        let eq30_e1213_d_b11: f64 = ((eq30_e1211_d_b11 * s.v[816]) + (eq30_e1211 * s.db[816][11]));
        let eq30_e1213_d_b12: f64 = ((eq30_e1211_d_b12 * s.v[816]) + (eq30_e1211 * s.db[816][12]));
        let eq30_e1213_d_b13: f64 = ((eq30_e1211_d_b13 * s.v[816]) + (eq30_e1211 * s.db[816][13]));
        let eq30_e1213_d_b14: f64 = ((eq30_e1211_d_b14 * s.v[816]) + (eq30_e1211 * s.db[816][14]));
        let eq30_e1213_d_b15: f64 = ((eq30_e1211_d_b15 * s.v[816]) + (eq30_e1211 * s.db[816][15]));
        let eq30_e1213_d_b16: f64 = ((eq30_e1211_d_b16 * s.v[816]) + (eq30_e1211 * s.db[816][16]));
        let eq30_e1213_d_b17: f64 = ((eq30_e1211_d_b17 * s.v[816]) + (eq30_e1211 * s.db[816][17]));
        let eq30_e1213_d_b18: f64 = ((eq30_e1211_d_b18 * s.v[816]) + (eq30_e1211 * s.db[816][18]));
        let eq30_e1213_d_b19: f64 = ((eq30_e1211_d_b19 * s.v[816]) + (eq30_e1211 * s.db[816][19]));
        let eq30_e1213_d_b20: f64 = ((eq30_e1211_d_b20 * s.v[816]) + (eq30_e1211 * s.db[816][20]));
        let eq30_e1213_d_b21: f64 = ((eq30_e1211_d_b21 * s.v[816]) + (eq30_e1211 * s.db[816][21]));
        let eq30_e1213_d_b22: f64 = ((eq30_e1211_d_b22 * s.v[816]) + (eq30_e1211 * s.db[816][22]));
        let eq30_e1213_d_b23: f64 = ((eq30_e1211_d_b23 * s.v[816]) + (eq30_e1211 * s.db[816][23]));
        let eq30_e1213_d_b24: f64 = ((eq30_e1211_d_b24 * s.v[816]) + (eq30_e1211 * s.db[816][24]));
        let eq30_e1215: f64 = (eq30_e1213 * (nv11 - nv9));
        let eq30_e1215_d_n0: f64 = (eq30_e1213_d_n0 * (nv11 - nv9));
        let eq30_e1215_d_n1: f64 = (eq30_e1213_d_n1 * (nv11 - nv9));
        let eq30_e1215_d_n2: f64 = (eq30_e1213_d_n2 * (nv11 - nv9));
        let eq30_e1215_d_n3: f64 = (eq30_e1213_d_n3 * (nv11 - nv9));
        let eq30_e1215_d_n4: f64 = (eq30_e1213_d_n4 * (nv11 - nv9));
        let eq30_e1215_d_n5: f64 = (eq30_e1213_d_n5 * (nv11 - nv9));
        let eq30_e1215_d_n6: f64 = (eq30_e1213_d_n6 * (nv11 - nv9));
        let eq30_e1215_d_n7: f64 = (eq30_e1213_d_n7 * (nv11 - nv9));
        let eq30_e1215_d_n8: f64 = (eq30_e1213_d_n8 * (nv11 - nv9));
        let eq30_e1215_d_n9: f64 = ((eq30_e1213_d_n9 * (nv11 - nv9)) + (-eq30_e1213));
        let eq30_e1215_d_n10: f64 = (eq30_e1213_d_n10 * (nv11 - nv9));
        let eq30_e1215_d_n11: f64 = ((eq30_e1213_d_n11 * (nv11 - nv9)) + eq30_e1213);
        let eq30_e1215_d_n12: f64 = (eq30_e1213_d_n12 * (nv11 - nv9));
        let eq30_e1215_d_n13: f64 = (eq30_e1213_d_n13 * (nv11 - nv9));
        let eq30_e1215_d_n14: f64 = (eq30_e1213_d_n14 * (nv11 - nv9));
        let eq30_e1215_d_n15: f64 = (eq30_e1213_d_n15 * (nv11 - nv9));
        let eq30_e1215_d_n16: f64 = (eq30_e1213_d_n16 * (nv11 - nv9));
        let eq30_e1215_d_n17: f64 = (eq30_e1213_d_n17 * (nv11 - nv9));
        let eq30_e1215_d_n18: f64 = (eq30_e1213_d_n18 * (nv11 - nv9));
        let eq30_e1215_d_n19: f64 = (eq30_e1213_d_n19 * (nv11 - nv9));
        let eq30_e1215_d_n20: f64 = (eq30_e1213_d_n20 * (nv11 - nv9));
        let eq30_e1215_d_b0: f64 = (eq30_e1213_d_b0 * (nv11 - nv9));
        let eq30_e1215_d_b1: f64 = (eq30_e1213_d_b1 * (nv11 - nv9));
        let eq30_e1215_d_b2: f64 = (eq30_e1213_d_b2 * (nv11 - nv9));
        let eq30_e1215_d_b3: f64 = (eq30_e1213_d_b3 * (nv11 - nv9));
        let eq30_e1215_d_b4: f64 = (eq30_e1213_d_b4 * (nv11 - nv9));
        let eq30_e1215_d_b5: f64 = (eq30_e1213_d_b5 * (nv11 - nv9));
        let eq30_e1215_d_b6: f64 = (eq30_e1213_d_b6 * (nv11 - nv9));
        let eq30_e1215_d_b7: f64 = (eq30_e1213_d_b7 * (nv11 - nv9));
        let eq30_e1215_d_b8: f64 = (eq30_e1213_d_b8 * (nv11 - nv9));
        let eq30_e1215_d_b9: f64 = (eq30_e1213_d_b9 * (nv11 - nv9));
        let eq30_e1215_d_b10: f64 = (eq30_e1213_d_b10 * (nv11 - nv9));
        let eq30_e1215_d_b11: f64 = (eq30_e1213_d_b11 * (nv11 - nv9));
        let eq30_e1215_d_b12: f64 = (eq30_e1213_d_b12 * (nv11 - nv9));
        let eq30_e1215_d_b13: f64 = (eq30_e1213_d_b13 * (nv11 - nv9));
        let eq30_e1215_d_b14: f64 = (eq30_e1213_d_b14 * (nv11 - nv9));
        let eq30_e1215_d_b15: f64 = (eq30_e1213_d_b15 * (nv11 - nv9));
        let eq30_e1215_d_b16: f64 = (eq30_e1213_d_b16 * (nv11 - nv9));
        let eq30_e1215_d_b17: f64 = (eq30_e1213_d_b17 * (nv11 - nv9));
        let eq30_e1215_d_b18: f64 = (eq30_e1213_d_b18 * (nv11 - nv9));
        let eq30_e1215_d_b19: f64 = (eq30_e1213_d_b19 * (nv11 - nv9));
        let eq30_e1215_d_b20: f64 = (eq30_e1213_d_b20 * (nv11 - nv9));
        let eq30_e1215_d_b21: f64 = (eq30_e1213_d_b21 * (nv11 - nv9));
        let eq30_e1215_d_b22: f64 = (eq30_e1213_d_b22 * (nv11 - nv9));
        let eq30_e1215_d_b23: f64 = (eq30_e1213_d_b23 * (nv11 - nv9));
        let eq30_e1215_d_b24: f64 = (eq30_e1213_d_b24 * (nv11 - nv9));
        (eq30_e1215, eq30_e1215_d_n0, eq30_e1215_d_n1, eq30_e1215_d_n2, eq30_e1215_d_n3, eq30_e1215_d_n4, eq30_e1215_d_n5, eq30_e1215_d_n6, eq30_e1215_d_n7, eq30_e1215_d_n8, eq30_e1215_d_n9, eq30_e1215_d_n10, eq30_e1215_d_n11, eq30_e1215_d_n12, eq30_e1215_d_n13, eq30_e1215_d_n14, eq30_e1215_d_n15, eq30_e1215_d_n16, eq30_e1215_d_n17, eq30_e1215_d_n18, eq30_e1215_d_n19, eq30_e1215_d_n20, eq30_e1215_d_b0, eq30_e1215_d_b1, eq30_e1215_d_b2, eq30_e1215_d_b3, eq30_e1215_d_b4, eq30_e1215_d_b5, eq30_e1215_d_b6, eq30_e1215_d_b7, eq30_e1215_d_b8, eq30_e1215_d_b9, eq30_e1215_d_b10, eq30_e1215_d_b11, eq30_e1215_d_b12, eq30_e1215_d_b13, eq30_e1215_d_b14, eq30_e1215_d_b15, eq30_e1215_d_b16, eq30_e1215_d_b17, eq30_e1215_d_b18, eq30_e1215_d_b19, eq30_e1215_d_b20, eq30_e1215_d_b21, eq30_e1215_d_b22, eq30_e1215_d_b23, eq30_e1215_d_b24,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e1217;
        let eq30_node_derivatives: [f64; 21] = [eq30_e1217_d_n0, eq30_e1217_d_n1, eq30_e1217_d_n2, eq30_e1217_d_n3, eq30_e1217_d_n4, eq30_e1217_d_n5, eq30_e1217_d_n6, eq30_e1217_d_n7, eq30_e1217_d_n8, eq30_e1217_d_n9, eq30_e1217_d_n10, eq30_e1217_d_n11, eq30_e1217_d_n12, eq30_e1217_d_n13, eq30_e1217_d_n14, eq30_e1217_d_n15, eq30_e1217_d_n16, eq30_e1217_d_n17, eq30_e1217_d_n18, eq30_e1217_d_n19, eq30_e1217_d_n20];
        let eq30_branch_derivatives: [f64; 25] = [eq30_e1217_d_b0, eq30_e1217_d_b1, eq30_e1217_d_b2, eq30_e1217_d_b3, eq30_e1217_d_b4, eq30_e1217_d_b5, eq30_e1217_d_b6, eq30_e1217_d_b7, eq30_e1217_d_b8, eq30_e1217_d_b9, eq30_e1217_d_b10, eq30_e1217_d_b11, eq30_e1217_d_b12, eq30_e1217_d_b13, eq30_e1217_d_b14, eq30_e1217_d_b15, eq30_e1217_d_b16, eq30_e1217_d_b17, eq30_e1217_d_b18, eq30_e1217_d_b19, eq30_e1217_d_b20, eq30_e1217_d_b21, eq30_e1217_d_b22, eq30_e1217_d_b23, eq30_e1217_d_b24];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(9),
            multiplicity * (eq30_value),
            &eq30_node_derivatives,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let (eq32_e1232,) = {
    if (!s.b[2919]) {
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
    }

    pub(super) fn stamp_transient_equations_block_8(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let (eq33_e1242, eq33_e1242_d_n0, eq33_e1242_d_n1, eq33_e1242_d_n2, eq33_e1242_d_n3, eq33_e1242_d_n4, eq33_e1242_d_n5, eq33_e1242_d_n6, eq33_e1242_d_n7, eq33_e1242_d_n8, eq33_e1242_d_n9, eq33_e1242_d_n10, eq33_e1242_d_n11, eq33_e1242_d_n12, eq33_e1242_d_n13, eq33_e1242_d_n14, eq33_e1242_d_n15, eq33_e1242_d_n16, eq33_e1242_d_n17, eq33_e1242_d_n18, eq33_e1242_d_n19, eq33_e1242_d_n20, eq33_e1242_d_b0, eq33_e1242_d_b1, eq33_e1242_d_b2, eq33_e1242_d_b3, eq33_e1242_d_b4, eq33_e1242_d_b5, eq33_e1242_d_b6, eq33_e1242_d_b7, eq33_e1242_d_b8, eq33_e1242_d_b9, eq33_e1242_d_b10, eq33_e1242_d_b11, eq33_e1242_d_b12, eq33_e1242_d_b13, eq33_e1242_d_b14, eq33_e1242_d_b15, eq33_e1242_d_b16, eq33_e1242_d_b17, eq33_e1242_d_b18, eq33_e1242_d_b19, eq33_e1242_d_b20, eq33_e1242_d_b21, eq33_e1242_d_b22, eq33_e1242_d_b23, eq33_e1242_d_b24,) = {
    if s.b[2920] {
        let eq33_e1236: f64 = (s.v[19] * p.p32);
        let eq33_e1236_d_n0: f64 = (s.dn[19][0] * p.p32);
        let eq33_e1236_d_n1: f64 = (s.dn[19][1] * p.p32);
        let eq33_e1236_d_n2: f64 = (s.dn[19][2] * p.p32);
        let eq33_e1236_d_n3: f64 = (s.dn[19][3] * p.p32);
        let eq33_e1236_d_n4: f64 = (s.dn[19][4] * p.p32);
        let eq33_e1236_d_n5: f64 = (s.dn[19][5] * p.p32);
        let eq33_e1236_d_n6: f64 = (s.dn[19][6] * p.p32);
        let eq33_e1236_d_n7: f64 = (s.dn[19][7] * p.p32);
        let eq33_e1236_d_n8: f64 = (s.dn[19][8] * p.p32);
        let eq33_e1236_d_n9: f64 = (s.dn[19][9] * p.p32);
        let eq33_e1236_d_n10: f64 = (s.dn[19][10] * p.p32);
        let eq33_e1236_d_n11: f64 = (s.dn[19][11] * p.p32);
        let eq33_e1236_d_n12: f64 = (s.dn[19][12] * p.p32);
        let eq33_e1236_d_n13: f64 = (s.dn[19][13] * p.p32);
        let eq33_e1236_d_n14: f64 = (s.dn[19][14] * p.p32);
        let eq33_e1236_d_n15: f64 = (s.dn[19][15] * p.p32);
        let eq33_e1236_d_n16: f64 = (s.dn[19][16] * p.p32);
        let eq33_e1236_d_n17: f64 = (s.dn[19][17] * p.p32);
        let eq33_e1236_d_n18: f64 = (s.dn[19][18] * p.p32);
        let eq33_e1236_d_n19: f64 = (s.dn[19][19] * p.p32);
        let eq33_e1236_d_n20: f64 = (s.dn[19][20] * p.p32);
        let eq33_e1236_d_b0: f64 = (s.db[19][0] * p.p32);
        let eq33_e1236_d_b1: f64 = (s.db[19][1] * p.p32);
        let eq33_e1236_d_b2: f64 = (s.db[19][2] * p.p32);
        let eq33_e1236_d_b3: f64 = (s.db[19][3] * p.p32);
        let eq33_e1236_d_b4: f64 = (s.db[19][4] * p.p32);
        let eq33_e1236_d_b5: f64 = (s.db[19][5] * p.p32);
        let eq33_e1236_d_b6: f64 = (s.db[19][6] * p.p32);
        let eq33_e1236_d_b7: f64 = (s.db[19][7] * p.p32);
        let eq33_e1236_d_b8: f64 = (s.db[19][8] * p.p32);
        let eq33_e1236_d_b9: f64 = (s.db[19][9] * p.p32);
        let eq33_e1236_d_b10: f64 = (s.db[19][10] * p.p32);
        let eq33_e1236_d_b11: f64 = (s.db[19][11] * p.p32);
        let eq33_e1236_d_b12: f64 = (s.db[19][12] * p.p32);
        let eq33_e1236_d_b13: f64 = (s.db[19][13] * p.p32);
        let eq33_e1236_d_b14: f64 = (s.db[19][14] * p.p32);
        let eq33_e1236_d_b15: f64 = (s.db[19][15] * p.p32);
        let eq33_e1236_d_b16: f64 = (s.db[19][16] * p.p32);
        let eq33_e1236_d_b17: f64 = (s.db[19][17] * p.p32);
        let eq33_e1236_d_b18: f64 = (s.db[19][18] * p.p32);
        let eq33_e1236_d_b19: f64 = (s.db[19][19] * p.p32);
        let eq33_e1236_d_b20: f64 = (s.db[19][20] * p.p32);
        let eq33_e1236_d_b21: f64 = (s.db[19][21] * p.p32);
        let eq33_e1236_d_b22: f64 = (s.db[19][22] * p.p32);
        let eq33_e1236_d_b23: f64 = (s.db[19][23] * p.p32);
        let eq33_e1236_d_b24: f64 = (s.db[19][24] * p.p32);
        let eq33_e1238: f64 = (eq33_e1236 * s.v[817]);
        let eq33_e1238_d_n0: f64 = ((eq33_e1236_d_n0 * s.v[817]) + (eq33_e1236 * s.dn[817][0]));
        let eq33_e1238_d_n1: f64 = ((eq33_e1236_d_n1 * s.v[817]) + (eq33_e1236 * s.dn[817][1]));
        let eq33_e1238_d_n2: f64 = ((eq33_e1236_d_n2 * s.v[817]) + (eq33_e1236 * s.dn[817][2]));
        let eq33_e1238_d_n3: f64 = ((eq33_e1236_d_n3 * s.v[817]) + (eq33_e1236 * s.dn[817][3]));
        let eq33_e1238_d_n4: f64 = ((eq33_e1236_d_n4 * s.v[817]) + (eq33_e1236 * s.dn[817][4]));
        let eq33_e1238_d_n5: f64 = ((eq33_e1236_d_n5 * s.v[817]) + (eq33_e1236 * s.dn[817][5]));
        let eq33_e1238_d_n6: f64 = ((eq33_e1236_d_n6 * s.v[817]) + (eq33_e1236 * s.dn[817][6]));
        let eq33_e1238_d_n7: f64 = ((eq33_e1236_d_n7 * s.v[817]) + (eq33_e1236 * s.dn[817][7]));
        let eq33_e1238_d_n8: f64 = ((eq33_e1236_d_n8 * s.v[817]) + (eq33_e1236 * s.dn[817][8]));
        let eq33_e1238_d_n9: f64 = ((eq33_e1236_d_n9 * s.v[817]) + (eq33_e1236 * s.dn[817][9]));
        let eq33_e1238_d_n10: f64 = ((eq33_e1236_d_n10 * s.v[817]) + (eq33_e1236 * s.dn[817][10]));
        let eq33_e1238_d_n11: f64 = ((eq33_e1236_d_n11 * s.v[817]) + (eq33_e1236 * s.dn[817][11]));
        let eq33_e1238_d_n12: f64 = ((eq33_e1236_d_n12 * s.v[817]) + (eq33_e1236 * s.dn[817][12]));
        let eq33_e1238_d_n13: f64 = ((eq33_e1236_d_n13 * s.v[817]) + (eq33_e1236 * s.dn[817][13]));
        let eq33_e1238_d_n14: f64 = ((eq33_e1236_d_n14 * s.v[817]) + (eq33_e1236 * s.dn[817][14]));
        let eq33_e1238_d_n15: f64 = ((eq33_e1236_d_n15 * s.v[817]) + (eq33_e1236 * s.dn[817][15]));
        let eq33_e1238_d_n16: f64 = ((eq33_e1236_d_n16 * s.v[817]) + (eq33_e1236 * s.dn[817][16]));
        let eq33_e1238_d_n17: f64 = ((eq33_e1236_d_n17 * s.v[817]) + (eq33_e1236 * s.dn[817][17]));
        let eq33_e1238_d_n18: f64 = ((eq33_e1236_d_n18 * s.v[817]) + (eq33_e1236 * s.dn[817][18]));
        let eq33_e1238_d_n19: f64 = ((eq33_e1236_d_n19 * s.v[817]) + (eq33_e1236 * s.dn[817][19]));
        let eq33_e1238_d_n20: f64 = ((eq33_e1236_d_n20 * s.v[817]) + (eq33_e1236 * s.dn[817][20]));
        let eq33_e1238_d_b0: f64 = ((eq33_e1236_d_b0 * s.v[817]) + (eq33_e1236 * s.db[817][0]));
        let eq33_e1238_d_b1: f64 = ((eq33_e1236_d_b1 * s.v[817]) + (eq33_e1236 * s.db[817][1]));
        let eq33_e1238_d_b2: f64 = ((eq33_e1236_d_b2 * s.v[817]) + (eq33_e1236 * s.db[817][2]));
        let eq33_e1238_d_b3: f64 = ((eq33_e1236_d_b3 * s.v[817]) + (eq33_e1236 * s.db[817][3]));
        let eq33_e1238_d_b4: f64 = ((eq33_e1236_d_b4 * s.v[817]) + (eq33_e1236 * s.db[817][4]));
        let eq33_e1238_d_b5: f64 = ((eq33_e1236_d_b5 * s.v[817]) + (eq33_e1236 * s.db[817][5]));
        let eq33_e1238_d_b6: f64 = ((eq33_e1236_d_b6 * s.v[817]) + (eq33_e1236 * s.db[817][6]));
        let eq33_e1238_d_b7: f64 = ((eq33_e1236_d_b7 * s.v[817]) + (eq33_e1236 * s.db[817][7]));
        let eq33_e1238_d_b8: f64 = ((eq33_e1236_d_b8 * s.v[817]) + (eq33_e1236 * s.db[817][8]));
        let eq33_e1238_d_b9: f64 = ((eq33_e1236_d_b9 * s.v[817]) + (eq33_e1236 * s.db[817][9]));
        let eq33_e1238_d_b10: f64 = ((eq33_e1236_d_b10 * s.v[817]) + (eq33_e1236 * s.db[817][10]));
        let eq33_e1238_d_b11: f64 = ((eq33_e1236_d_b11 * s.v[817]) + (eq33_e1236 * s.db[817][11]));
        let eq33_e1238_d_b12: f64 = ((eq33_e1236_d_b12 * s.v[817]) + (eq33_e1236 * s.db[817][12]));
        let eq33_e1238_d_b13: f64 = ((eq33_e1236_d_b13 * s.v[817]) + (eq33_e1236 * s.db[817][13]));
        let eq33_e1238_d_b14: f64 = ((eq33_e1236_d_b14 * s.v[817]) + (eq33_e1236 * s.db[817][14]));
        let eq33_e1238_d_b15: f64 = ((eq33_e1236_d_b15 * s.v[817]) + (eq33_e1236 * s.db[817][15]));
        let eq33_e1238_d_b16: f64 = ((eq33_e1236_d_b16 * s.v[817]) + (eq33_e1236 * s.db[817][16]));
        let eq33_e1238_d_b17: f64 = ((eq33_e1236_d_b17 * s.v[817]) + (eq33_e1236 * s.db[817][17]));
        let eq33_e1238_d_b18: f64 = ((eq33_e1236_d_b18 * s.v[817]) + (eq33_e1236 * s.db[817][18]));
        let eq33_e1238_d_b19: f64 = ((eq33_e1236_d_b19 * s.v[817]) + (eq33_e1236 * s.db[817][19]));
        let eq33_e1238_d_b20: f64 = ((eq33_e1236_d_b20 * s.v[817]) + (eq33_e1236 * s.db[817][20]));
        let eq33_e1238_d_b21: f64 = ((eq33_e1236_d_b21 * s.v[817]) + (eq33_e1236 * s.db[817][21]));
        let eq33_e1238_d_b22: f64 = ((eq33_e1236_d_b22 * s.v[817]) + (eq33_e1236 * s.db[817][22]));
        let eq33_e1238_d_b23: f64 = ((eq33_e1236_d_b23 * s.v[817]) + (eq33_e1236 * s.db[817][23]));
        let eq33_e1238_d_b24: f64 = ((eq33_e1236_d_b24 * s.v[817]) + (eq33_e1236 * s.db[817][24]));
        let eq33_e1240: f64 = (eq33_e1238 * (nv3 - nv9));
        let eq33_e1240_d_n0: f64 = (eq33_e1238_d_n0 * (nv3 - nv9));
        let eq33_e1240_d_n1: f64 = (eq33_e1238_d_n1 * (nv3 - nv9));
        let eq33_e1240_d_n2: f64 = (eq33_e1238_d_n2 * (nv3 - nv9));
        let eq33_e1240_d_n3: f64 = ((eq33_e1238_d_n3 * (nv3 - nv9)) + eq33_e1238);
        let eq33_e1240_d_n4: f64 = (eq33_e1238_d_n4 * (nv3 - nv9));
        let eq33_e1240_d_n5: f64 = (eq33_e1238_d_n5 * (nv3 - nv9));
        let eq33_e1240_d_n6: f64 = (eq33_e1238_d_n6 * (nv3 - nv9));
        let eq33_e1240_d_n7: f64 = (eq33_e1238_d_n7 * (nv3 - nv9));
        let eq33_e1240_d_n8: f64 = (eq33_e1238_d_n8 * (nv3 - nv9));
        let eq33_e1240_d_n9: f64 = ((eq33_e1238_d_n9 * (nv3 - nv9)) + (-eq33_e1238));
        let eq33_e1240_d_n10: f64 = (eq33_e1238_d_n10 * (nv3 - nv9));
        let eq33_e1240_d_n11: f64 = (eq33_e1238_d_n11 * (nv3 - nv9));
        let eq33_e1240_d_n12: f64 = (eq33_e1238_d_n12 * (nv3 - nv9));
        let eq33_e1240_d_n13: f64 = (eq33_e1238_d_n13 * (nv3 - nv9));
        let eq33_e1240_d_n14: f64 = (eq33_e1238_d_n14 * (nv3 - nv9));
        let eq33_e1240_d_n15: f64 = (eq33_e1238_d_n15 * (nv3 - nv9));
        let eq33_e1240_d_n16: f64 = (eq33_e1238_d_n16 * (nv3 - nv9));
        let eq33_e1240_d_n17: f64 = (eq33_e1238_d_n17 * (nv3 - nv9));
        let eq33_e1240_d_n18: f64 = (eq33_e1238_d_n18 * (nv3 - nv9));
        let eq33_e1240_d_n19: f64 = (eq33_e1238_d_n19 * (nv3 - nv9));
        let eq33_e1240_d_n20: f64 = (eq33_e1238_d_n20 * (nv3 - nv9));
        let eq33_e1240_d_b0: f64 = (eq33_e1238_d_b0 * (nv3 - nv9));
        let eq33_e1240_d_b1: f64 = (eq33_e1238_d_b1 * (nv3 - nv9));
        let eq33_e1240_d_b2: f64 = (eq33_e1238_d_b2 * (nv3 - nv9));
        let eq33_e1240_d_b3: f64 = (eq33_e1238_d_b3 * (nv3 - nv9));
        let eq33_e1240_d_b4: f64 = (eq33_e1238_d_b4 * (nv3 - nv9));
        let eq33_e1240_d_b5: f64 = (eq33_e1238_d_b5 * (nv3 - nv9));
        let eq33_e1240_d_b6: f64 = (eq33_e1238_d_b6 * (nv3 - nv9));
        let eq33_e1240_d_b7: f64 = (eq33_e1238_d_b7 * (nv3 - nv9));
        let eq33_e1240_d_b8: f64 = (eq33_e1238_d_b8 * (nv3 - nv9));
        let eq33_e1240_d_b9: f64 = (eq33_e1238_d_b9 * (nv3 - nv9));
        let eq33_e1240_d_b10: f64 = (eq33_e1238_d_b10 * (nv3 - nv9));
        let eq33_e1240_d_b11: f64 = (eq33_e1238_d_b11 * (nv3 - nv9));
        let eq33_e1240_d_b12: f64 = (eq33_e1238_d_b12 * (nv3 - nv9));
        let eq33_e1240_d_b13: f64 = (eq33_e1238_d_b13 * (nv3 - nv9));
        let eq33_e1240_d_b14: f64 = (eq33_e1238_d_b14 * (nv3 - nv9));
        let eq33_e1240_d_b15: f64 = (eq33_e1238_d_b15 * (nv3 - nv9));
        let eq33_e1240_d_b16: f64 = (eq33_e1238_d_b16 * (nv3 - nv9));
        let eq33_e1240_d_b17: f64 = (eq33_e1238_d_b17 * (nv3 - nv9));
        let eq33_e1240_d_b18: f64 = (eq33_e1238_d_b18 * (nv3 - nv9));
        let eq33_e1240_d_b19: f64 = (eq33_e1238_d_b19 * (nv3 - nv9));
        let eq33_e1240_d_b20: f64 = (eq33_e1238_d_b20 * (nv3 - nv9));
        let eq33_e1240_d_b21: f64 = (eq33_e1238_d_b21 * (nv3 - nv9));
        let eq33_e1240_d_b22: f64 = (eq33_e1238_d_b22 * (nv3 - nv9));
        let eq33_e1240_d_b23: f64 = (eq33_e1238_d_b23 * (nv3 - nv9));
        let eq33_e1240_d_b24: f64 = (eq33_e1238_d_b24 * (nv3 - nv9));
        (eq33_e1240, eq33_e1240_d_n0, eq33_e1240_d_n1, eq33_e1240_d_n2, eq33_e1240_d_n3, eq33_e1240_d_n4, eq33_e1240_d_n5, eq33_e1240_d_n6, eq33_e1240_d_n7, eq33_e1240_d_n8, eq33_e1240_d_n9, eq33_e1240_d_n10, eq33_e1240_d_n11, eq33_e1240_d_n12, eq33_e1240_d_n13, eq33_e1240_d_n14, eq33_e1240_d_n15, eq33_e1240_d_n16, eq33_e1240_d_n17, eq33_e1240_d_n18, eq33_e1240_d_n19, eq33_e1240_d_n20, eq33_e1240_d_b0, eq33_e1240_d_b1, eq33_e1240_d_b2, eq33_e1240_d_b3, eq33_e1240_d_b4, eq33_e1240_d_b5, eq33_e1240_d_b6, eq33_e1240_d_b7, eq33_e1240_d_b8, eq33_e1240_d_b9, eq33_e1240_d_b10, eq33_e1240_d_b11, eq33_e1240_d_b12, eq33_e1240_d_b13, eq33_e1240_d_b14, eq33_e1240_d_b15, eq33_e1240_d_b16, eq33_e1240_d_b17, eq33_e1240_d_b18, eq33_e1240_d_b19, eq33_e1240_d_b20, eq33_e1240_d_b21, eq33_e1240_d_b22, eq33_e1240_d_b23, eq33_e1240_d_b24,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e1242;
        let eq33_node_derivatives: [f64; 21] = [eq33_e1242_d_n0, eq33_e1242_d_n1, eq33_e1242_d_n2, eq33_e1242_d_n3, eq33_e1242_d_n4, eq33_e1242_d_n5, eq33_e1242_d_n6, eq33_e1242_d_n7, eq33_e1242_d_n8, eq33_e1242_d_n9, eq33_e1242_d_n10, eq33_e1242_d_n11, eq33_e1242_d_n12, eq33_e1242_d_n13, eq33_e1242_d_n14, eq33_e1242_d_n15, eq33_e1242_d_n16, eq33_e1242_d_n17, eq33_e1242_d_n18, eq33_e1242_d_n19, eq33_e1242_d_n20];
        let eq33_branch_derivatives: [f64; 25] = [eq33_e1242_d_b0, eq33_e1242_d_b1, eq33_e1242_d_b2, eq33_e1242_d_b3, eq33_e1242_d_b4, eq33_e1242_d_b5, eq33_e1242_d_b6, eq33_e1242_d_b7, eq33_e1242_d_b8, eq33_e1242_d_b9, eq33_e1242_d_b10, eq33_e1242_d_b11, eq33_e1242_d_b12, eq33_e1242_d_b13, eq33_e1242_d_b14, eq33_e1242_d_b15, eq33_e1242_d_b16, eq33_e1242_d_b17, eq33_e1242_d_b18, eq33_e1242_d_b19, eq33_e1242_d_b20, eq33_e1242_d_b21, eq33_e1242_d_b22, eq33_e1242_d_b23, eq33_e1242_d_b24];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(9),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let (eq35_e1257,) = {
    if (!s.b[2920]) {
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
    }

    pub(super) fn stamp_transient_equations_block_9(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        ddt_active: bool,
        idt_scale: f64,
        idt_state_current: &mut [f64; Instance::IDT_STATE_COUNT],
        idt_state_previous: &mut [f64; Instance::IDT_STATE_COUNT],
        idt_state_initialized: &mut [bool; Instance::IDT_STATE_COUNT],
    ) {
        let eq39_e1275: f64 = (-s.v[1995]);
        let eq39_e1275_d_n0: f64 = (-s.dn[1995][0]);
        let eq39_e1275_d_n1: f64 = (-s.dn[1995][1]);
        let eq39_e1275_d_n2: f64 = (-s.dn[1995][2]);
        let eq39_e1275_d_n3: f64 = (-s.dn[1995][3]);
        let eq39_e1275_d_n4: f64 = (-s.dn[1995][4]);
        let eq39_e1275_d_n5: f64 = (-s.dn[1995][5]);
        let eq39_e1275_d_n6: f64 = (-s.dn[1995][6]);
        let eq39_e1275_d_n7: f64 = (-s.dn[1995][7]);
        let eq39_e1275_d_n8: f64 = (-s.dn[1995][8]);
        let eq39_e1275_d_n9: f64 = (-s.dn[1995][9]);
        let eq39_e1275_d_n10: f64 = (-s.dn[1995][10]);
        let eq39_e1275_d_n11: f64 = (-s.dn[1995][11]);
        let eq39_e1275_d_n12: f64 = (-s.dn[1995][12]);
        let eq39_e1275_d_n13: f64 = (-s.dn[1995][13]);
        let eq39_e1275_d_n14: f64 = (-s.dn[1995][14]);
        let eq39_e1275_d_n15: f64 = (-s.dn[1995][15]);
        let eq39_e1275_d_n16: f64 = (-s.dn[1995][16]);
        let eq39_e1275_d_n17: f64 = (-s.dn[1995][17]);
        let eq39_e1275_d_n18: f64 = (-s.dn[1995][18]);
        let eq39_e1275_d_n19: f64 = (-s.dn[1995][19]);
        let eq39_e1275_d_n20: f64 = (-s.dn[1995][20]);
        let eq39_e1275_d_b0: f64 = (-s.db[1995][0]);
        let eq39_e1275_d_b1: f64 = (-s.db[1995][1]);
        let eq39_e1275_d_b2: f64 = (-s.db[1995][2]);
        let eq39_e1275_d_b3: f64 = (-s.db[1995][3]);
        let eq39_e1275_d_b4: f64 = (-s.db[1995][4]);
        let eq39_e1275_d_b5: f64 = (-s.db[1995][5]);
        let eq39_e1275_d_b6: f64 = (-s.db[1995][6]);
        let eq39_e1275_d_b7: f64 = (-s.db[1995][7]);
        let eq39_e1275_d_b8: f64 = (-s.db[1995][8]);
        let eq39_e1275_d_b9: f64 = (-s.db[1995][9]);
        let eq39_e1275_d_b10: f64 = (-s.db[1995][10]);
        let eq39_e1275_d_b11: f64 = (-s.db[1995][11]);
        let eq39_e1275_d_b12: f64 = (-s.db[1995][12]);
        let eq39_e1275_d_b13: f64 = (-s.db[1995][13]);
        let eq39_e1275_d_b14: f64 = (-s.db[1995][14]);
        let eq39_e1275_d_b15: f64 = (-s.db[1995][15]);
        let eq39_e1275_d_b16: f64 = (-s.db[1995][16]);
        let eq39_e1275_d_b17: f64 = (-s.db[1995][17]);
        let eq39_e1275_d_b18: f64 = (-s.db[1995][18]);
        let eq39_e1275_d_b19: f64 = (-s.db[1995][19]);
        let eq39_e1275_d_b20: f64 = (-s.db[1995][20]);
        let eq39_e1275_d_b21: f64 = (-s.db[1995][21]);
        let eq39_e1275_d_b22: f64 = (-s.db[1995][22]);
        let eq39_e1275_d_b23: f64 = (-s.db[1995][23]);
        let eq39_e1275_d_b24: f64 = (-s.db[1995][24]);
        let eq39_e1277: f64 = (eq39_e1275 * s.v[1951]);
        let eq39_e1277_d_n0: f64 = ((eq39_e1275_d_n0 * s.v[1951]) + (eq39_e1275 * s.dn[1951][0]));
        let eq39_e1277_d_n1: f64 = ((eq39_e1275_d_n1 * s.v[1951]) + (eq39_e1275 * s.dn[1951][1]));
        let eq39_e1277_d_n2: f64 = ((eq39_e1275_d_n2 * s.v[1951]) + (eq39_e1275 * s.dn[1951][2]));
        let eq39_e1277_d_n3: f64 = ((eq39_e1275_d_n3 * s.v[1951]) + (eq39_e1275 * s.dn[1951][3]));
        let eq39_e1277_d_n4: f64 = ((eq39_e1275_d_n4 * s.v[1951]) + (eq39_e1275 * s.dn[1951][4]));
        let eq39_e1277_d_n5: f64 = ((eq39_e1275_d_n5 * s.v[1951]) + (eq39_e1275 * s.dn[1951][5]));
        let eq39_e1277_d_n6: f64 = ((eq39_e1275_d_n6 * s.v[1951]) + (eq39_e1275 * s.dn[1951][6]));
        let eq39_e1277_d_n7: f64 = ((eq39_e1275_d_n7 * s.v[1951]) + (eq39_e1275 * s.dn[1951][7]));
        let eq39_e1277_d_n8: f64 = ((eq39_e1275_d_n8 * s.v[1951]) + (eq39_e1275 * s.dn[1951][8]));
        let eq39_e1277_d_n9: f64 = ((eq39_e1275_d_n9 * s.v[1951]) + (eq39_e1275 * s.dn[1951][9]));
        let eq39_e1277_d_n10: f64 = ((eq39_e1275_d_n10 * s.v[1951]) + (eq39_e1275 * s.dn[1951][10]));
        let eq39_e1277_d_n11: f64 = ((eq39_e1275_d_n11 * s.v[1951]) + (eq39_e1275 * s.dn[1951][11]));
        let eq39_e1277_d_n12: f64 = ((eq39_e1275_d_n12 * s.v[1951]) + (eq39_e1275 * s.dn[1951][12]));
        let eq39_e1277_d_n13: f64 = ((eq39_e1275_d_n13 * s.v[1951]) + (eq39_e1275 * s.dn[1951][13]));
        let eq39_e1277_d_n14: f64 = ((eq39_e1275_d_n14 * s.v[1951]) + (eq39_e1275 * s.dn[1951][14]));
        let eq39_e1277_d_n15: f64 = ((eq39_e1275_d_n15 * s.v[1951]) + (eq39_e1275 * s.dn[1951][15]));
        let eq39_e1277_d_n16: f64 = ((eq39_e1275_d_n16 * s.v[1951]) + (eq39_e1275 * s.dn[1951][16]));
        let eq39_e1277_d_n17: f64 = ((eq39_e1275_d_n17 * s.v[1951]) + (eq39_e1275 * s.dn[1951][17]));
        let eq39_e1277_d_n18: f64 = ((eq39_e1275_d_n18 * s.v[1951]) + (eq39_e1275 * s.dn[1951][18]));
        let eq39_e1277_d_n19: f64 = ((eq39_e1275_d_n19 * s.v[1951]) + (eq39_e1275 * s.dn[1951][19]));
        let eq39_e1277_d_n20: f64 = ((eq39_e1275_d_n20 * s.v[1951]) + (eq39_e1275 * s.dn[1951][20]));
        let eq39_e1277_d_b0: f64 = ((eq39_e1275_d_b0 * s.v[1951]) + (eq39_e1275 * s.db[1951][0]));
        let eq39_e1277_d_b1: f64 = ((eq39_e1275_d_b1 * s.v[1951]) + (eq39_e1275 * s.db[1951][1]));
        let eq39_e1277_d_b2: f64 = ((eq39_e1275_d_b2 * s.v[1951]) + (eq39_e1275 * s.db[1951][2]));
        let eq39_e1277_d_b3: f64 = ((eq39_e1275_d_b3 * s.v[1951]) + (eq39_e1275 * s.db[1951][3]));
        let eq39_e1277_d_b4: f64 = ((eq39_e1275_d_b4 * s.v[1951]) + (eq39_e1275 * s.db[1951][4]));
        let eq39_e1277_d_b5: f64 = ((eq39_e1275_d_b5 * s.v[1951]) + (eq39_e1275 * s.db[1951][5]));
        let eq39_e1277_d_b6: f64 = ((eq39_e1275_d_b6 * s.v[1951]) + (eq39_e1275 * s.db[1951][6]));
        let eq39_e1277_d_b7: f64 = ((eq39_e1275_d_b7 * s.v[1951]) + (eq39_e1275 * s.db[1951][7]));
        let eq39_e1277_d_b8: f64 = ((eq39_e1275_d_b8 * s.v[1951]) + (eq39_e1275 * s.db[1951][8]));
        let eq39_e1277_d_b9: f64 = ((eq39_e1275_d_b9 * s.v[1951]) + (eq39_e1275 * s.db[1951][9]));
        let eq39_e1277_d_b10: f64 = ((eq39_e1275_d_b10 * s.v[1951]) + (eq39_e1275 * s.db[1951][10]));
        let eq39_e1277_d_b11: f64 = ((eq39_e1275_d_b11 * s.v[1951]) + (eq39_e1275 * s.db[1951][11]));
        let eq39_e1277_d_b12: f64 = ((eq39_e1275_d_b12 * s.v[1951]) + (eq39_e1275 * s.db[1951][12]));
        let eq39_e1277_d_b13: f64 = ((eq39_e1275_d_b13 * s.v[1951]) + (eq39_e1275 * s.db[1951][13]));
        let eq39_e1277_d_b14: f64 = ((eq39_e1275_d_b14 * s.v[1951]) + (eq39_e1275 * s.db[1951][14]));
        let eq39_e1277_d_b15: f64 = ((eq39_e1275_d_b15 * s.v[1951]) + (eq39_e1275 * s.db[1951][15]));
        let eq39_e1277_d_b16: f64 = ((eq39_e1275_d_b16 * s.v[1951]) + (eq39_e1275 * s.db[1951][16]));
        let eq39_e1277_d_b17: f64 = ((eq39_e1275_d_b17 * s.v[1951]) + (eq39_e1275 * s.db[1951][17]));
        let eq39_e1277_d_b18: f64 = ((eq39_e1275_d_b18 * s.v[1951]) + (eq39_e1275 * s.db[1951][18]));
        let eq39_e1277_d_b19: f64 = ((eq39_e1275_d_b19 * s.v[1951]) + (eq39_e1275 * s.db[1951][19]));
        let eq39_e1277_d_b20: f64 = ((eq39_e1275_d_b20 * s.v[1951]) + (eq39_e1275 * s.db[1951][20]));
        let eq39_e1277_d_b21: f64 = ((eq39_e1275_d_b21 * s.v[1951]) + (eq39_e1275 * s.db[1951][21]));
        let eq39_e1277_d_b22: f64 = ((eq39_e1275_d_b22 * s.v[1951]) + (eq39_e1275 * s.db[1951][22]));
        let eq39_e1277_d_b23: f64 = ((eq39_e1275_d_b23 * s.v[1951]) + (eq39_e1275 * s.db[1951][23]));
        let eq39_e1277_d_b24: f64 = ((eq39_e1275_d_b24 * s.v[1951]) + (eq39_e1275 * s.db[1951][24]));
        let eq39_e1279: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 0, eq39_e1277, s.v[1942]);
        let eq39_e1279_d_n0: f64 = (eq39_e1277_d_n0 * idt_scale);
        let eq39_e1279_d_n1: f64 = (eq39_e1277_d_n1 * idt_scale);
        let eq39_e1279_d_n2: f64 = (eq39_e1277_d_n2 * idt_scale);
        let eq39_e1279_d_n3: f64 = (eq39_e1277_d_n3 * idt_scale);
        let eq39_e1279_d_n4: f64 = (eq39_e1277_d_n4 * idt_scale);
        let eq39_e1279_d_n5: f64 = (eq39_e1277_d_n5 * idt_scale);
        let eq39_e1279_d_n6: f64 = (eq39_e1277_d_n6 * idt_scale);
        let eq39_e1279_d_n7: f64 = (eq39_e1277_d_n7 * idt_scale);
        let eq39_e1279_d_n8: f64 = (eq39_e1277_d_n8 * idt_scale);
        let eq39_e1279_d_n9: f64 = (eq39_e1277_d_n9 * idt_scale);
        let eq39_e1279_d_n10: f64 = (eq39_e1277_d_n10 * idt_scale);
        let eq39_e1279_d_n11: f64 = (eq39_e1277_d_n11 * idt_scale);
        let eq39_e1279_d_n12: f64 = (eq39_e1277_d_n12 * idt_scale);
        let eq39_e1279_d_n13: f64 = (eq39_e1277_d_n13 * idt_scale);
        let eq39_e1279_d_n14: f64 = (eq39_e1277_d_n14 * idt_scale);
        let eq39_e1279_d_n15: f64 = (eq39_e1277_d_n15 * idt_scale);
        let eq39_e1279_d_n16: f64 = (eq39_e1277_d_n16 * idt_scale);
        let eq39_e1279_d_n17: f64 = (eq39_e1277_d_n17 * idt_scale);
        let eq39_e1279_d_n18: f64 = (eq39_e1277_d_n18 * idt_scale);
        let eq39_e1279_d_n19: f64 = (eq39_e1277_d_n19 * idt_scale);
        let eq39_e1279_d_n20: f64 = (eq39_e1277_d_n20 * idt_scale);
        let eq39_e1279_d_b0: f64 = (eq39_e1277_d_b0 * idt_scale);
        let eq39_e1279_d_b1: f64 = (eq39_e1277_d_b1 * idt_scale);
        let eq39_e1279_d_b2: f64 = (eq39_e1277_d_b2 * idt_scale);
        let eq39_e1279_d_b3: f64 = (eq39_e1277_d_b3 * idt_scale);
        let eq39_e1279_d_b4: f64 = (eq39_e1277_d_b4 * idt_scale);
        let eq39_e1279_d_b5: f64 = (eq39_e1277_d_b5 * idt_scale);
        let eq39_e1279_d_b6: f64 = (eq39_e1277_d_b6 * idt_scale);
        let eq39_e1279_d_b7: f64 = (eq39_e1277_d_b7 * idt_scale);
        let eq39_e1279_d_b8: f64 = (eq39_e1277_d_b8 * idt_scale);
        let eq39_e1279_d_b9: f64 = (eq39_e1277_d_b9 * idt_scale);
        let eq39_e1279_d_b10: f64 = (eq39_e1277_d_b10 * idt_scale);
        let eq39_e1279_d_b11: f64 = (eq39_e1277_d_b11 * idt_scale);
        let eq39_e1279_d_b12: f64 = (eq39_e1277_d_b12 * idt_scale);
        let eq39_e1279_d_b13: f64 = (eq39_e1277_d_b13 * idt_scale);
        let eq39_e1279_d_b14: f64 = (eq39_e1277_d_b14 * idt_scale);
        let eq39_e1279_d_b15: f64 = (eq39_e1277_d_b15 * idt_scale);
        let eq39_e1279_d_b16: f64 = (eq39_e1277_d_b16 * idt_scale);
        let eq39_e1279_d_b17: f64 = (eq39_e1277_d_b17 * idt_scale);
        let eq39_e1279_d_b18: f64 = (eq39_e1277_d_b18 * idt_scale);
        let eq39_e1279_d_b19: f64 = (eq39_e1277_d_b19 * idt_scale);
        let eq39_e1279_d_b20: f64 = (eq39_e1277_d_b20 * idt_scale);
        let eq39_e1279_d_b21: f64 = (eq39_e1277_d_b21 * idt_scale);
        let eq39_e1279_d_b22: f64 = (eq39_e1277_d_b22 * idt_scale);
        let eq39_e1279_d_b23: f64 = (eq39_e1277_d_b23 * idt_scale);
        let eq39_e1279_d_b24: f64 = (eq39_e1277_d_b24 * idt_scale);
        let eq39_e1280: f64 = (s.v[4] * eq39_e1279);
        let eq39_e1280_d_n0: f64 = (s.v[4] * eq39_e1279_d_n0);
        let eq39_e1280_d_n1: f64 = (s.v[4] * eq39_e1279_d_n1);
        let eq39_e1280_d_n2: f64 = (s.v[4] * eq39_e1279_d_n2);
        let eq39_e1280_d_n3: f64 = (s.v[4] * eq39_e1279_d_n3);
        let eq39_e1280_d_n4: f64 = (s.v[4] * eq39_e1279_d_n4);
        let eq39_e1280_d_n5: f64 = (s.v[4] * eq39_e1279_d_n5);
        let eq39_e1280_d_n6: f64 = (s.v[4] * eq39_e1279_d_n6);
        let eq39_e1280_d_n7: f64 = (s.v[4] * eq39_e1279_d_n7);
        let eq39_e1280_d_n8: f64 = (s.v[4] * eq39_e1279_d_n8);
        let eq39_e1280_d_n9: f64 = (s.v[4] * eq39_e1279_d_n9);
        let eq39_e1280_d_n10: f64 = (s.v[4] * eq39_e1279_d_n10);
        let eq39_e1280_d_n11: f64 = (s.v[4] * eq39_e1279_d_n11);
        let eq39_e1280_d_n12: f64 = (s.v[4] * eq39_e1279_d_n12);
        let eq39_e1280_d_n13: f64 = (s.v[4] * eq39_e1279_d_n13);
        let eq39_e1280_d_n14: f64 = (s.v[4] * eq39_e1279_d_n14);
        let eq39_e1280_d_n15: f64 = (s.v[4] * eq39_e1279_d_n15);
        let eq39_e1280_d_n16: f64 = (s.v[4] * eq39_e1279_d_n16);
        let eq39_e1280_d_n17: f64 = (s.v[4] * eq39_e1279_d_n17);
        let eq39_e1280_d_n18: f64 = (s.v[4] * eq39_e1279_d_n18);
        let eq39_e1280_d_n19: f64 = (s.v[4] * eq39_e1279_d_n19);
        let eq39_e1280_d_n20: f64 = (s.v[4] * eq39_e1279_d_n20);
        let eq39_e1280_d_b0: f64 = (s.v[4] * eq39_e1279_d_b0);
        let eq39_e1280_d_b1: f64 = (s.v[4] * eq39_e1279_d_b1);
        let eq39_e1280_d_b2: f64 = (s.v[4] * eq39_e1279_d_b2);
        let eq39_e1280_d_b3: f64 = (s.v[4] * eq39_e1279_d_b3);
        let eq39_e1280_d_b4: f64 = (s.v[4] * eq39_e1279_d_b4);
        let eq39_e1280_d_b5: f64 = (s.v[4] * eq39_e1279_d_b5);
        let eq39_e1280_d_b6: f64 = (s.v[4] * eq39_e1279_d_b6);
        let eq39_e1280_d_b7: f64 = (s.v[4] * eq39_e1279_d_b7);
        let eq39_e1280_d_b8: f64 = (s.v[4] * eq39_e1279_d_b8);
        let eq39_e1280_d_b9: f64 = (s.v[4] * eq39_e1279_d_b9);
        let eq39_e1280_d_b10: f64 = (s.v[4] * eq39_e1279_d_b10);
        let eq39_e1280_d_b11: f64 = (s.v[4] * eq39_e1279_d_b11);
        let eq39_e1280_d_b12: f64 = (s.v[4] * eq39_e1279_d_b12);
        let eq39_e1280_d_b13: f64 = (s.v[4] * eq39_e1279_d_b13);
        let eq39_e1280_d_b14: f64 = (s.v[4] * eq39_e1279_d_b14);
        let eq39_e1280_d_b15: f64 = (s.v[4] * eq39_e1279_d_b15);
        let eq39_e1280_d_b16: f64 = (s.v[4] * eq39_e1279_d_b16);
        let eq39_e1280_d_b17: f64 = (s.v[4] * eq39_e1279_d_b17);
        let eq39_e1280_d_b18: f64 = (s.v[4] * eq39_e1279_d_b18);
        let eq39_e1280_d_b19: f64 = (s.v[4] * eq39_e1279_d_b19);
        let eq39_e1280_d_b20: f64 = (s.v[4] * eq39_e1279_d_b20);
        let eq39_e1280_d_b21: f64 = (s.v[4] * eq39_e1279_d_b21);
        let eq39_e1280_d_b22: f64 = (s.v[4] * eq39_e1279_d_b22);
        let eq39_e1280_d_b23: f64 = (s.v[4] * eq39_e1279_d_b23);
        let eq39_e1280_d_b24: f64 = (s.v[4] * eq39_e1279_d_b24);
        let eq39_value: f64 = eq39_e1280;
        let eq39_node_derivatives: [f64; 21] = [eq39_e1280_d_n0, eq39_e1280_d_n1, eq39_e1280_d_n2, eq39_e1280_d_n3, eq39_e1280_d_n4, eq39_e1280_d_n5, eq39_e1280_d_n6, eq39_e1280_d_n7, eq39_e1280_d_n8, eq39_e1280_d_n9, eq39_e1280_d_n10, eq39_e1280_d_n11, eq39_e1280_d_n12, eq39_e1280_d_n13, eq39_e1280_d_n14, eq39_e1280_d_n15, eq39_e1280_d_n16, eq39_e1280_d_n17, eq39_e1280_d_n18, eq39_e1280_d_n19, eq39_e1280_d_n20];
        let eq39_branch_derivatives: [f64; 25] = [eq39_e1280_d_b0, eq39_e1280_d_b1, eq39_e1280_d_b2, eq39_e1280_d_b3, eq39_e1280_d_b4, eq39_e1280_d_b5, eq39_e1280_d_b6, eq39_e1280_d_b7, eq39_e1280_d_b8, eq39_e1280_d_b9, eq39_e1280_d_b10, eq39_e1280_d_b11, eq39_e1280_d_b12, eq39_e1280_d_b13, eq39_e1280_d_b14, eq39_e1280_d_b15, eq39_e1280_d_b16, eq39_e1280_d_b17, eq39_e1280_d_b18, eq39_e1280_d_b19, eq39_e1280_d_b20, eq39_e1280_d_b21, eq39_e1280_d_b22, eq39_e1280_d_b23, eq39_e1280_d_b24];
        stamper.stamp_potential_dense_local(
            8,
            eq39_value,
            &eq39_node_derivatives,
            &eq39_branch_derivatives,
        );
    }

    pub(super) fn stamp_transient_equations_block_10(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        ddt_active: bool,
        idt_scale: f64,
        idt_state_current: &mut [f64; Instance::IDT_STATE_COUNT],
        idt_state_previous: &mut [f64; Instance::IDT_STATE_COUNT],
        idt_state_initialized: &mut [bool; Instance::IDT_STATE_COUNT],
    ) {
        let eq41_e1288: f64 = (-s.v[1995]);
        let eq41_e1288_d_n0: f64 = (-s.dn[1995][0]);
        let eq41_e1288_d_n1: f64 = (-s.dn[1995][1]);
        let eq41_e1288_d_n2: f64 = (-s.dn[1995][2]);
        let eq41_e1288_d_n3: f64 = (-s.dn[1995][3]);
        let eq41_e1288_d_n4: f64 = (-s.dn[1995][4]);
        let eq41_e1288_d_n5: f64 = (-s.dn[1995][5]);
        let eq41_e1288_d_n6: f64 = (-s.dn[1995][6]);
        let eq41_e1288_d_n7: f64 = (-s.dn[1995][7]);
        let eq41_e1288_d_n8: f64 = (-s.dn[1995][8]);
        let eq41_e1288_d_n9: f64 = (-s.dn[1995][9]);
        let eq41_e1288_d_n10: f64 = (-s.dn[1995][10]);
        let eq41_e1288_d_n11: f64 = (-s.dn[1995][11]);
        let eq41_e1288_d_n12: f64 = (-s.dn[1995][12]);
        let eq41_e1288_d_n13: f64 = (-s.dn[1995][13]);
        let eq41_e1288_d_n14: f64 = (-s.dn[1995][14]);
        let eq41_e1288_d_n15: f64 = (-s.dn[1995][15]);
        let eq41_e1288_d_n16: f64 = (-s.dn[1995][16]);
        let eq41_e1288_d_n17: f64 = (-s.dn[1995][17]);
        let eq41_e1288_d_n18: f64 = (-s.dn[1995][18]);
        let eq41_e1288_d_n19: f64 = (-s.dn[1995][19]);
        let eq41_e1288_d_n20: f64 = (-s.dn[1995][20]);
        let eq41_e1288_d_b0: f64 = (-s.db[1995][0]);
        let eq41_e1288_d_b1: f64 = (-s.db[1995][1]);
        let eq41_e1288_d_b2: f64 = (-s.db[1995][2]);
        let eq41_e1288_d_b3: f64 = (-s.db[1995][3]);
        let eq41_e1288_d_b4: f64 = (-s.db[1995][4]);
        let eq41_e1288_d_b5: f64 = (-s.db[1995][5]);
        let eq41_e1288_d_b6: f64 = (-s.db[1995][6]);
        let eq41_e1288_d_b7: f64 = (-s.db[1995][7]);
        let eq41_e1288_d_b8: f64 = (-s.db[1995][8]);
        let eq41_e1288_d_b9: f64 = (-s.db[1995][9]);
        let eq41_e1288_d_b10: f64 = (-s.db[1995][10]);
        let eq41_e1288_d_b11: f64 = (-s.db[1995][11]);
        let eq41_e1288_d_b12: f64 = (-s.db[1995][12]);
        let eq41_e1288_d_b13: f64 = (-s.db[1995][13]);
        let eq41_e1288_d_b14: f64 = (-s.db[1995][14]);
        let eq41_e1288_d_b15: f64 = (-s.db[1995][15]);
        let eq41_e1288_d_b16: f64 = (-s.db[1995][16]);
        let eq41_e1288_d_b17: f64 = (-s.db[1995][17]);
        let eq41_e1288_d_b18: f64 = (-s.db[1995][18]);
        let eq41_e1288_d_b19: f64 = (-s.db[1995][19]);
        let eq41_e1288_d_b20: f64 = (-s.db[1995][20]);
        let eq41_e1288_d_b21: f64 = (-s.db[1995][21]);
        let eq41_e1288_d_b22: f64 = (-s.db[1995][22]);
        let eq41_e1288_d_b23: f64 = (-s.db[1995][23]);
        let eq41_e1288_d_b24: f64 = (-s.db[1995][24]);
        let eq41_e1290: f64 = (eq41_e1288 * s.v[1952]);
        let eq41_e1290_d_n0: f64 = ((eq41_e1288_d_n0 * s.v[1952]) + (eq41_e1288 * s.dn[1952][0]));
        let eq41_e1290_d_n1: f64 = ((eq41_e1288_d_n1 * s.v[1952]) + (eq41_e1288 * s.dn[1952][1]));
        let eq41_e1290_d_n2: f64 = ((eq41_e1288_d_n2 * s.v[1952]) + (eq41_e1288 * s.dn[1952][2]));
        let eq41_e1290_d_n3: f64 = ((eq41_e1288_d_n3 * s.v[1952]) + (eq41_e1288 * s.dn[1952][3]));
        let eq41_e1290_d_n4: f64 = ((eq41_e1288_d_n4 * s.v[1952]) + (eq41_e1288 * s.dn[1952][4]));
        let eq41_e1290_d_n5: f64 = ((eq41_e1288_d_n5 * s.v[1952]) + (eq41_e1288 * s.dn[1952][5]));
        let eq41_e1290_d_n6: f64 = ((eq41_e1288_d_n6 * s.v[1952]) + (eq41_e1288 * s.dn[1952][6]));
        let eq41_e1290_d_n7: f64 = ((eq41_e1288_d_n7 * s.v[1952]) + (eq41_e1288 * s.dn[1952][7]));
        let eq41_e1290_d_n8: f64 = ((eq41_e1288_d_n8 * s.v[1952]) + (eq41_e1288 * s.dn[1952][8]));
        let eq41_e1290_d_n9: f64 = ((eq41_e1288_d_n9 * s.v[1952]) + (eq41_e1288 * s.dn[1952][9]));
        let eq41_e1290_d_n10: f64 = ((eq41_e1288_d_n10 * s.v[1952]) + (eq41_e1288 * s.dn[1952][10]));
        let eq41_e1290_d_n11: f64 = ((eq41_e1288_d_n11 * s.v[1952]) + (eq41_e1288 * s.dn[1952][11]));
        let eq41_e1290_d_n12: f64 = ((eq41_e1288_d_n12 * s.v[1952]) + (eq41_e1288 * s.dn[1952][12]));
        let eq41_e1290_d_n13: f64 = ((eq41_e1288_d_n13 * s.v[1952]) + (eq41_e1288 * s.dn[1952][13]));
        let eq41_e1290_d_n14: f64 = ((eq41_e1288_d_n14 * s.v[1952]) + (eq41_e1288 * s.dn[1952][14]));
        let eq41_e1290_d_n15: f64 = ((eq41_e1288_d_n15 * s.v[1952]) + (eq41_e1288 * s.dn[1952][15]));
        let eq41_e1290_d_n16: f64 = ((eq41_e1288_d_n16 * s.v[1952]) + (eq41_e1288 * s.dn[1952][16]));
        let eq41_e1290_d_n17: f64 = ((eq41_e1288_d_n17 * s.v[1952]) + (eq41_e1288 * s.dn[1952][17]));
        let eq41_e1290_d_n18: f64 = ((eq41_e1288_d_n18 * s.v[1952]) + (eq41_e1288 * s.dn[1952][18]));
        let eq41_e1290_d_n19: f64 = ((eq41_e1288_d_n19 * s.v[1952]) + (eq41_e1288 * s.dn[1952][19]));
        let eq41_e1290_d_n20: f64 = ((eq41_e1288_d_n20 * s.v[1952]) + (eq41_e1288 * s.dn[1952][20]));
        let eq41_e1290_d_b0: f64 = ((eq41_e1288_d_b0 * s.v[1952]) + (eq41_e1288 * s.db[1952][0]));
        let eq41_e1290_d_b1: f64 = ((eq41_e1288_d_b1 * s.v[1952]) + (eq41_e1288 * s.db[1952][1]));
        let eq41_e1290_d_b2: f64 = ((eq41_e1288_d_b2 * s.v[1952]) + (eq41_e1288 * s.db[1952][2]));
        let eq41_e1290_d_b3: f64 = ((eq41_e1288_d_b3 * s.v[1952]) + (eq41_e1288 * s.db[1952][3]));
        let eq41_e1290_d_b4: f64 = ((eq41_e1288_d_b4 * s.v[1952]) + (eq41_e1288 * s.db[1952][4]));
        let eq41_e1290_d_b5: f64 = ((eq41_e1288_d_b5 * s.v[1952]) + (eq41_e1288 * s.db[1952][5]));
        let eq41_e1290_d_b6: f64 = ((eq41_e1288_d_b6 * s.v[1952]) + (eq41_e1288 * s.db[1952][6]));
        let eq41_e1290_d_b7: f64 = ((eq41_e1288_d_b7 * s.v[1952]) + (eq41_e1288 * s.db[1952][7]));
        let eq41_e1290_d_b8: f64 = ((eq41_e1288_d_b8 * s.v[1952]) + (eq41_e1288 * s.db[1952][8]));
        let eq41_e1290_d_b9: f64 = ((eq41_e1288_d_b9 * s.v[1952]) + (eq41_e1288 * s.db[1952][9]));
        let eq41_e1290_d_b10: f64 = ((eq41_e1288_d_b10 * s.v[1952]) + (eq41_e1288 * s.db[1952][10]));
        let eq41_e1290_d_b11: f64 = ((eq41_e1288_d_b11 * s.v[1952]) + (eq41_e1288 * s.db[1952][11]));
        let eq41_e1290_d_b12: f64 = ((eq41_e1288_d_b12 * s.v[1952]) + (eq41_e1288 * s.db[1952][12]));
        let eq41_e1290_d_b13: f64 = ((eq41_e1288_d_b13 * s.v[1952]) + (eq41_e1288 * s.db[1952][13]));
        let eq41_e1290_d_b14: f64 = ((eq41_e1288_d_b14 * s.v[1952]) + (eq41_e1288 * s.db[1952][14]));
        let eq41_e1290_d_b15: f64 = ((eq41_e1288_d_b15 * s.v[1952]) + (eq41_e1288 * s.db[1952][15]));
        let eq41_e1290_d_b16: f64 = ((eq41_e1288_d_b16 * s.v[1952]) + (eq41_e1288 * s.db[1952][16]));
        let eq41_e1290_d_b17: f64 = ((eq41_e1288_d_b17 * s.v[1952]) + (eq41_e1288 * s.db[1952][17]));
        let eq41_e1290_d_b18: f64 = ((eq41_e1288_d_b18 * s.v[1952]) + (eq41_e1288 * s.db[1952][18]));
        let eq41_e1290_d_b19: f64 = ((eq41_e1288_d_b19 * s.v[1952]) + (eq41_e1288 * s.db[1952][19]));
        let eq41_e1290_d_b20: f64 = ((eq41_e1288_d_b20 * s.v[1952]) + (eq41_e1288 * s.db[1952][20]));
        let eq41_e1290_d_b21: f64 = ((eq41_e1288_d_b21 * s.v[1952]) + (eq41_e1288 * s.db[1952][21]));
        let eq41_e1290_d_b22: f64 = ((eq41_e1288_d_b22 * s.v[1952]) + (eq41_e1288 * s.db[1952][22]));
        let eq41_e1290_d_b23: f64 = ((eq41_e1288_d_b23 * s.v[1952]) + (eq41_e1288 * s.db[1952][23]));
        let eq41_e1290_d_b24: f64 = ((eq41_e1288_d_b24 * s.v[1952]) + (eq41_e1288 * s.db[1952][24]));
        let eq41_e1292: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 1, eq41_e1290, s.v[1943]);
        let eq41_e1292_d_n0: f64 = (eq41_e1290_d_n0 * idt_scale);
        let eq41_e1292_d_n1: f64 = (eq41_e1290_d_n1 * idt_scale);
        let eq41_e1292_d_n2: f64 = (eq41_e1290_d_n2 * idt_scale);
        let eq41_e1292_d_n3: f64 = (eq41_e1290_d_n3 * idt_scale);
        let eq41_e1292_d_n4: f64 = (eq41_e1290_d_n4 * idt_scale);
        let eq41_e1292_d_n5: f64 = (eq41_e1290_d_n5 * idt_scale);
        let eq41_e1292_d_n6: f64 = (eq41_e1290_d_n6 * idt_scale);
        let eq41_e1292_d_n7: f64 = (eq41_e1290_d_n7 * idt_scale);
        let eq41_e1292_d_n8: f64 = (eq41_e1290_d_n8 * idt_scale);
        let eq41_e1292_d_n9: f64 = (eq41_e1290_d_n9 * idt_scale);
        let eq41_e1292_d_n10: f64 = (eq41_e1290_d_n10 * idt_scale);
        let eq41_e1292_d_n11: f64 = (eq41_e1290_d_n11 * idt_scale);
        let eq41_e1292_d_n12: f64 = (eq41_e1290_d_n12 * idt_scale);
        let eq41_e1292_d_n13: f64 = (eq41_e1290_d_n13 * idt_scale);
        let eq41_e1292_d_n14: f64 = (eq41_e1290_d_n14 * idt_scale);
        let eq41_e1292_d_n15: f64 = (eq41_e1290_d_n15 * idt_scale);
        let eq41_e1292_d_n16: f64 = (eq41_e1290_d_n16 * idt_scale);
        let eq41_e1292_d_n17: f64 = (eq41_e1290_d_n17 * idt_scale);
        let eq41_e1292_d_n18: f64 = (eq41_e1290_d_n18 * idt_scale);
        let eq41_e1292_d_n19: f64 = (eq41_e1290_d_n19 * idt_scale);
        let eq41_e1292_d_n20: f64 = (eq41_e1290_d_n20 * idt_scale);
        let eq41_e1292_d_b0: f64 = (eq41_e1290_d_b0 * idt_scale);
        let eq41_e1292_d_b1: f64 = (eq41_e1290_d_b1 * idt_scale);
        let eq41_e1292_d_b2: f64 = (eq41_e1290_d_b2 * idt_scale);
        let eq41_e1292_d_b3: f64 = (eq41_e1290_d_b3 * idt_scale);
        let eq41_e1292_d_b4: f64 = (eq41_e1290_d_b4 * idt_scale);
        let eq41_e1292_d_b5: f64 = (eq41_e1290_d_b5 * idt_scale);
        let eq41_e1292_d_b6: f64 = (eq41_e1290_d_b6 * idt_scale);
        let eq41_e1292_d_b7: f64 = (eq41_e1290_d_b7 * idt_scale);
        let eq41_e1292_d_b8: f64 = (eq41_e1290_d_b8 * idt_scale);
        let eq41_e1292_d_b9: f64 = (eq41_e1290_d_b9 * idt_scale);
        let eq41_e1292_d_b10: f64 = (eq41_e1290_d_b10 * idt_scale);
        let eq41_e1292_d_b11: f64 = (eq41_e1290_d_b11 * idt_scale);
        let eq41_e1292_d_b12: f64 = (eq41_e1290_d_b12 * idt_scale);
        let eq41_e1292_d_b13: f64 = (eq41_e1290_d_b13 * idt_scale);
        let eq41_e1292_d_b14: f64 = (eq41_e1290_d_b14 * idt_scale);
        let eq41_e1292_d_b15: f64 = (eq41_e1290_d_b15 * idt_scale);
        let eq41_e1292_d_b16: f64 = (eq41_e1290_d_b16 * idt_scale);
        let eq41_e1292_d_b17: f64 = (eq41_e1290_d_b17 * idt_scale);
        let eq41_e1292_d_b18: f64 = (eq41_e1290_d_b18 * idt_scale);
        let eq41_e1292_d_b19: f64 = (eq41_e1290_d_b19 * idt_scale);
        let eq41_e1292_d_b20: f64 = (eq41_e1290_d_b20 * idt_scale);
        let eq41_e1292_d_b21: f64 = (eq41_e1290_d_b21 * idt_scale);
        let eq41_e1292_d_b22: f64 = (eq41_e1290_d_b22 * idt_scale);
        let eq41_e1292_d_b23: f64 = (eq41_e1290_d_b23 * idt_scale);
        let eq41_e1292_d_b24: f64 = (eq41_e1290_d_b24 * idt_scale);
        let eq41_e1293: f64 = (s.v[4] * eq41_e1292);
        let eq41_e1293_d_n0: f64 = (s.v[4] * eq41_e1292_d_n0);
        let eq41_e1293_d_n1: f64 = (s.v[4] * eq41_e1292_d_n1);
        let eq41_e1293_d_n2: f64 = (s.v[4] * eq41_e1292_d_n2);
        let eq41_e1293_d_n3: f64 = (s.v[4] * eq41_e1292_d_n3);
        let eq41_e1293_d_n4: f64 = (s.v[4] * eq41_e1292_d_n4);
        let eq41_e1293_d_n5: f64 = (s.v[4] * eq41_e1292_d_n5);
        let eq41_e1293_d_n6: f64 = (s.v[4] * eq41_e1292_d_n6);
        let eq41_e1293_d_n7: f64 = (s.v[4] * eq41_e1292_d_n7);
        let eq41_e1293_d_n8: f64 = (s.v[4] * eq41_e1292_d_n8);
        let eq41_e1293_d_n9: f64 = (s.v[4] * eq41_e1292_d_n9);
        let eq41_e1293_d_n10: f64 = (s.v[4] * eq41_e1292_d_n10);
        let eq41_e1293_d_n11: f64 = (s.v[4] * eq41_e1292_d_n11);
        let eq41_e1293_d_n12: f64 = (s.v[4] * eq41_e1292_d_n12);
        let eq41_e1293_d_n13: f64 = (s.v[4] * eq41_e1292_d_n13);
        let eq41_e1293_d_n14: f64 = (s.v[4] * eq41_e1292_d_n14);
        let eq41_e1293_d_n15: f64 = (s.v[4] * eq41_e1292_d_n15);
        let eq41_e1293_d_n16: f64 = (s.v[4] * eq41_e1292_d_n16);
        let eq41_e1293_d_n17: f64 = (s.v[4] * eq41_e1292_d_n17);
        let eq41_e1293_d_n18: f64 = (s.v[4] * eq41_e1292_d_n18);
        let eq41_e1293_d_n19: f64 = (s.v[4] * eq41_e1292_d_n19);
        let eq41_e1293_d_n20: f64 = (s.v[4] * eq41_e1292_d_n20);
        let eq41_e1293_d_b0: f64 = (s.v[4] * eq41_e1292_d_b0);
        let eq41_e1293_d_b1: f64 = (s.v[4] * eq41_e1292_d_b1);
        let eq41_e1293_d_b2: f64 = (s.v[4] * eq41_e1292_d_b2);
        let eq41_e1293_d_b3: f64 = (s.v[4] * eq41_e1292_d_b3);
        let eq41_e1293_d_b4: f64 = (s.v[4] * eq41_e1292_d_b4);
        let eq41_e1293_d_b5: f64 = (s.v[4] * eq41_e1292_d_b5);
        let eq41_e1293_d_b6: f64 = (s.v[4] * eq41_e1292_d_b6);
        let eq41_e1293_d_b7: f64 = (s.v[4] * eq41_e1292_d_b7);
        let eq41_e1293_d_b8: f64 = (s.v[4] * eq41_e1292_d_b8);
        let eq41_e1293_d_b9: f64 = (s.v[4] * eq41_e1292_d_b9);
        let eq41_e1293_d_b10: f64 = (s.v[4] * eq41_e1292_d_b10);
        let eq41_e1293_d_b11: f64 = (s.v[4] * eq41_e1292_d_b11);
        let eq41_e1293_d_b12: f64 = (s.v[4] * eq41_e1292_d_b12);
        let eq41_e1293_d_b13: f64 = (s.v[4] * eq41_e1292_d_b13);
        let eq41_e1293_d_b14: f64 = (s.v[4] * eq41_e1292_d_b14);
        let eq41_e1293_d_b15: f64 = (s.v[4] * eq41_e1292_d_b15);
        let eq41_e1293_d_b16: f64 = (s.v[4] * eq41_e1292_d_b16);
        let eq41_e1293_d_b17: f64 = (s.v[4] * eq41_e1292_d_b17);
        let eq41_e1293_d_b18: f64 = (s.v[4] * eq41_e1292_d_b18);
        let eq41_e1293_d_b19: f64 = (s.v[4] * eq41_e1292_d_b19);
        let eq41_e1293_d_b20: f64 = (s.v[4] * eq41_e1292_d_b20);
        let eq41_e1293_d_b21: f64 = (s.v[4] * eq41_e1292_d_b21);
        let eq41_e1293_d_b22: f64 = (s.v[4] * eq41_e1292_d_b22);
        let eq41_e1293_d_b23: f64 = (s.v[4] * eq41_e1292_d_b23);
        let eq41_e1293_d_b24: f64 = (s.v[4] * eq41_e1292_d_b24);
        let eq41_value: f64 = eq41_e1293;
        let eq41_node_derivatives: [f64; 21] = [eq41_e1293_d_n0, eq41_e1293_d_n1, eq41_e1293_d_n2, eq41_e1293_d_n3, eq41_e1293_d_n4, eq41_e1293_d_n5, eq41_e1293_d_n6, eq41_e1293_d_n7, eq41_e1293_d_n8, eq41_e1293_d_n9, eq41_e1293_d_n10, eq41_e1293_d_n11, eq41_e1293_d_n12, eq41_e1293_d_n13, eq41_e1293_d_n14, eq41_e1293_d_n15, eq41_e1293_d_n16, eq41_e1293_d_n17, eq41_e1293_d_n18, eq41_e1293_d_n19, eq41_e1293_d_n20];
        let eq41_branch_derivatives: [f64; 25] = [eq41_e1293_d_b0, eq41_e1293_d_b1, eq41_e1293_d_b2, eq41_e1293_d_b3, eq41_e1293_d_b4, eq41_e1293_d_b5, eq41_e1293_d_b6, eq41_e1293_d_b7, eq41_e1293_d_b8, eq41_e1293_d_b9, eq41_e1293_d_b10, eq41_e1293_d_b11, eq41_e1293_d_b12, eq41_e1293_d_b13, eq41_e1293_d_b14, eq41_e1293_d_b15, eq41_e1293_d_b16, eq41_e1293_d_b17, eq41_e1293_d_b18, eq41_e1293_d_b19, eq41_e1293_d_b20, eq41_e1293_d_b21, eq41_e1293_d_b22, eq41_e1293_d_b23, eq41_e1293_d_b24];
        stamper.stamp_potential_dense_local(
            10,
            eq41_value,
            &eq41_node_derivatives,
            &eq41_branch_derivatives,
        );
    }

    pub(super) fn stamp_transient_equations_block_11(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        ddt_active: bool,
        idt_scale: f64,
        idt_state_current: &mut [f64; Instance::IDT_STATE_COUNT],
        idt_state_previous: &mut [f64; Instance::IDT_STATE_COUNT],
        idt_state_initialized: &mut [bool; Instance::IDT_STATE_COUNT],
    ) {
        let eq43_e1301: f64 = (-s.v[1995]);
        let eq43_e1301_d_n0: f64 = (-s.dn[1995][0]);
        let eq43_e1301_d_n1: f64 = (-s.dn[1995][1]);
        let eq43_e1301_d_n2: f64 = (-s.dn[1995][2]);
        let eq43_e1301_d_n3: f64 = (-s.dn[1995][3]);
        let eq43_e1301_d_n4: f64 = (-s.dn[1995][4]);
        let eq43_e1301_d_n5: f64 = (-s.dn[1995][5]);
        let eq43_e1301_d_n6: f64 = (-s.dn[1995][6]);
        let eq43_e1301_d_n7: f64 = (-s.dn[1995][7]);
        let eq43_e1301_d_n8: f64 = (-s.dn[1995][8]);
        let eq43_e1301_d_n9: f64 = (-s.dn[1995][9]);
        let eq43_e1301_d_n10: f64 = (-s.dn[1995][10]);
        let eq43_e1301_d_n11: f64 = (-s.dn[1995][11]);
        let eq43_e1301_d_n12: f64 = (-s.dn[1995][12]);
        let eq43_e1301_d_n13: f64 = (-s.dn[1995][13]);
        let eq43_e1301_d_n14: f64 = (-s.dn[1995][14]);
        let eq43_e1301_d_n15: f64 = (-s.dn[1995][15]);
        let eq43_e1301_d_n16: f64 = (-s.dn[1995][16]);
        let eq43_e1301_d_n17: f64 = (-s.dn[1995][17]);
        let eq43_e1301_d_n18: f64 = (-s.dn[1995][18]);
        let eq43_e1301_d_n19: f64 = (-s.dn[1995][19]);
        let eq43_e1301_d_n20: f64 = (-s.dn[1995][20]);
        let eq43_e1301_d_b0: f64 = (-s.db[1995][0]);
        let eq43_e1301_d_b1: f64 = (-s.db[1995][1]);
        let eq43_e1301_d_b2: f64 = (-s.db[1995][2]);
        let eq43_e1301_d_b3: f64 = (-s.db[1995][3]);
        let eq43_e1301_d_b4: f64 = (-s.db[1995][4]);
        let eq43_e1301_d_b5: f64 = (-s.db[1995][5]);
        let eq43_e1301_d_b6: f64 = (-s.db[1995][6]);
        let eq43_e1301_d_b7: f64 = (-s.db[1995][7]);
        let eq43_e1301_d_b8: f64 = (-s.db[1995][8]);
        let eq43_e1301_d_b9: f64 = (-s.db[1995][9]);
        let eq43_e1301_d_b10: f64 = (-s.db[1995][10]);
        let eq43_e1301_d_b11: f64 = (-s.db[1995][11]);
        let eq43_e1301_d_b12: f64 = (-s.db[1995][12]);
        let eq43_e1301_d_b13: f64 = (-s.db[1995][13]);
        let eq43_e1301_d_b14: f64 = (-s.db[1995][14]);
        let eq43_e1301_d_b15: f64 = (-s.db[1995][15]);
        let eq43_e1301_d_b16: f64 = (-s.db[1995][16]);
        let eq43_e1301_d_b17: f64 = (-s.db[1995][17]);
        let eq43_e1301_d_b18: f64 = (-s.db[1995][18]);
        let eq43_e1301_d_b19: f64 = (-s.db[1995][19]);
        let eq43_e1301_d_b20: f64 = (-s.db[1995][20]);
        let eq43_e1301_d_b21: f64 = (-s.db[1995][21]);
        let eq43_e1301_d_b22: f64 = (-s.db[1995][22]);
        let eq43_e1301_d_b23: f64 = (-s.db[1995][23]);
        let eq43_e1301_d_b24: f64 = (-s.db[1995][24]);
        let eq43_e1303: f64 = (eq43_e1301 * s.v[1953]);
        let eq43_e1303_d_n0: f64 = ((eq43_e1301_d_n0 * s.v[1953]) + (eq43_e1301 * s.dn[1953][0]));
        let eq43_e1303_d_n1: f64 = ((eq43_e1301_d_n1 * s.v[1953]) + (eq43_e1301 * s.dn[1953][1]));
        let eq43_e1303_d_n2: f64 = ((eq43_e1301_d_n2 * s.v[1953]) + (eq43_e1301 * s.dn[1953][2]));
        let eq43_e1303_d_n3: f64 = ((eq43_e1301_d_n3 * s.v[1953]) + (eq43_e1301 * s.dn[1953][3]));
        let eq43_e1303_d_n4: f64 = ((eq43_e1301_d_n4 * s.v[1953]) + (eq43_e1301 * s.dn[1953][4]));
        let eq43_e1303_d_n5: f64 = ((eq43_e1301_d_n5 * s.v[1953]) + (eq43_e1301 * s.dn[1953][5]));
        let eq43_e1303_d_n6: f64 = ((eq43_e1301_d_n6 * s.v[1953]) + (eq43_e1301 * s.dn[1953][6]));
        let eq43_e1303_d_n7: f64 = ((eq43_e1301_d_n7 * s.v[1953]) + (eq43_e1301 * s.dn[1953][7]));
        let eq43_e1303_d_n8: f64 = ((eq43_e1301_d_n8 * s.v[1953]) + (eq43_e1301 * s.dn[1953][8]));
        let eq43_e1303_d_n9: f64 = ((eq43_e1301_d_n9 * s.v[1953]) + (eq43_e1301 * s.dn[1953][9]));
        let eq43_e1303_d_n10: f64 = ((eq43_e1301_d_n10 * s.v[1953]) + (eq43_e1301 * s.dn[1953][10]));
        let eq43_e1303_d_n11: f64 = ((eq43_e1301_d_n11 * s.v[1953]) + (eq43_e1301 * s.dn[1953][11]));
        let eq43_e1303_d_n12: f64 = ((eq43_e1301_d_n12 * s.v[1953]) + (eq43_e1301 * s.dn[1953][12]));
        let eq43_e1303_d_n13: f64 = ((eq43_e1301_d_n13 * s.v[1953]) + (eq43_e1301 * s.dn[1953][13]));
        let eq43_e1303_d_n14: f64 = ((eq43_e1301_d_n14 * s.v[1953]) + (eq43_e1301 * s.dn[1953][14]));
        let eq43_e1303_d_n15: f64 = ((eq43_e1301_d_n15 * s.v[1953]) + (eq43_e1301 * s.dn[1953][15]));
        let eq43_e1303_d_n16: f64 = ((eq43_e1301_d_n16 * s.v[1953]) + (eq43_e1301 * s.dn[1953][16]));
        let eq43_e1303_d_n17: f64 = ((eq43_e1301_d_n17 * s.v[1953]) + (eq43_e1301 * s.dn[1953][17]));
        let eq43_e1303_d_n18: f64 = ((eq43_e1301_d_n18 * s.v[1953]) + (eq43_e1301 * s.dn[1953][18]));
        let eq43_e1303_d_n19: f64 = ((eq43_e1301_d_n19 * s.v[1953]) + (eq43_e1301 * s.dn[1953][19]));
        let eq43_e1303_d_n20: f64 = ((eq43_e1301_d_n20 * s.v[1953]) + (eq43_e1301 * s.dn[1953][20]));
        let eq43_e1303_d_b0: f64 = ((eq43_e1301_d_b0 * s.v[1953]) + (eq43_e1301 * s.db[1953][0]));
        let eq43_e1303_d_b1: f64 = ((eq43_e1301_d_b1 * s.v[1953]) + (eq43_e1301 * s.db[1953][1]));
        let eq43_e1303_d_b2: f64 = ((eq43_e1301_d_b2 * s.v[1953]) + (eq43_e1301 * s.db[1953][2]));
        let eq43_e1303_d_b3: f64 = ((eq43_e1301_d_b3 * s.v[1953]) + (eq43_e1301 * s.db[1953][3]));
        let eq43_e1303_d_b4: f64 = ((eq43_e1301_d_b4 * s.v[1953]) + (eq43_e1301 * s.db[1953][4]));
        let eq43_e1303_d_b5: f64 = ((eq43_e1301_d_b5 * s.v[1953]) + (eq43_e1301 * s.db[1953][5]));
        let eq43_e1303_d_b6: f64 = ((eq43_e1301_d_b6 * s.v[1953]) + (eq43_e1301 * s.db[1953][6]));
        let eq43_e1303_d_b7: f64 = ((eq43_e1301_d_b7 * s.v[1953]) + (eq43_e1301 * s.db[1953][7]));
        let eq43_e1303_d_b8: f64 = ((eq43_e1301_d_b8 * s.v[1953]) + (eq43_e1301 * s.db[1953][8]));
        let eq43_e1303_d_b9: f64 = ((eq43_e1301_d_b9 * s.v[1953]) + (eq43_e1301 * s.db[1953][9]));
        let eq43_e1303_d_b10: f64 = ((eq43_e1301_d_b10 * s.v[1953]) + (eq43_e1301 * s.db[1953][10]));
        let eq43_e1303_d_b11: f64 = ((eq43_e1301_d_b11 * s.v[1953]) + (eq43_e1301 * s.db[1953][11]));
        let eq43_e1303_d_b12: f64 = ((eq43_e1301_d_b12 * s.v[1953]) + (eq43_e1301 * s.db[1953][12]));
        let eq43_e1303_d_b13: f64 = ((eq43_e1301_d_b13 * s.v[1953]) + (eq43_e1301 * s.db[1953][13]));
        let eq43_e1303_d_b14: f64 = ((eq43_e1301_d_b14 * s.v[1953]) + (eq43_e1301 * s.db[1953][14]));
        let eq43_e1303_d_b15: f64 = ((eq43_e1301_d_b15 * s.v[1953]) + (eq43_e1301 * s.db[1953][15]));
        let eq43_e1303_d_b16: f64 = ((eq43_e1301_d_b16 * s.v[1953]) + (eq43_e1301 * s.db[1953][16]));
        let eq43_e1303_d_b17: f64 = ((eq43_e1301_d_b17 * s.v[1953]) + (eq43_e1301 * s.db[1953][17]));
        let eq43_e1303_d_b18: f64 = ((eq43_e1301_d_b18 * s.v[1953]) + (eq43_e1301 * s.db[1953][18]));
        let eq43_e1303_d_b19: f64 = ((eq43_e1301_d_b19 * s.v[1953]) + (eq43_e1301 * s.db[1953][19]));
        let eq43_e1303_d_b20: f64 = ((eq43_e1301_d_b20 * s.v[1953]) + (eq43_e1301 * s.db[1953][20]));
        let eq43_e1303_d_b21: f64 = ((eq43_e1301_d_b21 * s.v[1953]) + (eq43_e1301 * s.db[1953][21]));
        let eq43_e1303_d_b22: f64 = ((eq43_e1301_d_b22 * s.v[1953]) + (eq43_e1301 * s.db[1953][22]));
        let eq43_e1303_d_b23: f64 = ((eq43_e1301_d_b23 * s.v[1953]) + (eq43_e1301 * s.db[1953][23]));
        let eq43_e1303_d_b24: f64 = ((eq43_e1301_d_b24 * s.v[1953]) + (eq43_e1301 * s.db[1953][24]));
        let eq43_e1305: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 2, eq43_e1303, s.v[1944]);
        let eq43_e1305_d_n0: f64 = (eq43_e1303_d_n0 * idt_scale);
        let eq43_e1305_d_n1: f64 = (eq43_e1303_d_n1 * idt_scale);
        let eq43_e1305_d_n2: f64 = (eq43_e1303_d_n2 * idt_scale);
        let eq43_e1305_d_n3: f64 = (eq43_e1303_d_n3 * idt_scale);
        let eq43_e1305_d_n4: f64 = (eq43_e1303_d_n4 * idt_scale);
        let eq43_e1305_d_n5: f64 = (eq43_e1303_d_n5 * idt_scale);
        let eq43_e1305_d_n6: f64 = (eq43_e1303_d_n6 * idt_scale);
        let eq43_e1305_d_n7: f64 = (eq43_e1303_d_n7 * idt_scale);
        let eq43_e1305_d_n8: f64 = (eq43_e1303_d_n8 * idt_scale);
        let eq43_e1305_d_n9: f64 = (eq43_e1303_d_n9 * idt_scale);
        let eq43_e1305_d_n10: f64 = (eq43_e1303_d_n10 * idt_scale);
        let eq43_e1305_d_n11: f64 = (eq43_e1303_d_n11 * idt_scale);
        let eq43_e1305_d_n12: f64 = (eq43_e1303_d_n12 * idt_scale);
        let eq43_e1305_d_n13: f64 = (eq43_e1303_d_n13 * idt_scale);
        let eq43_e1305_d_n14: f64 = (eq43_e1303_d_n14 * idt_scale);
        let eq43_e1305_d_n15: f64 = (eq43_e1303_d_n15 * idt_scale);
        let eq43_e1305_d_n16: f64 = (eq43_e1303_d_n16 * idt_scale);
        let eq43_e1305_d_n17: f64 = (eq43_e1303_d_n17 * idt_scale);
        let eq43_e1305_d_n18: f64 = (eq43_e1303_d_n18 * idt_scale);
        let eq43_e1305_d_n19: f64 = (eq43_e1303_d_n19 * idt_scale);
        let eq43_e1305_d_n20: f64 = (eq43_e1303_d_n20 * idt_scale);
        let eq43_e1305_d_b0: f64 = (eq43_e1303_d_b0 * idt_scale);
        let eq43_e1305_d_b1: f64 = (eq43_e1303_d_b1 * idt_scale);
        let eq43_e1305_d_b2: f64 = (eq43_e1303_d_b2 * idt_scale);
        let eq43_e1305_d_b3: f64 = (eq43_e1303_d_b3 * idt_scale);
        let eq43_e1305_d_b4: f64 = (eq43_e1303_d_b4 * idt_scale);
        let eq43_e1305_d_b5: f64 = (eq43_e1303_d_b5 * idt_scale);
        let eq43_e1305_d_b6: f64 = (eq43_e1303_d_b6 * idt_scale);
        let eq43_e1305_d_b7: f64 = (eq43_e1303_d_b7 * idt_scale);
        let eq43_e1305_d_b8: f64 = (eq43_e1303_d_b8 * idt_scale);
        let eq43_e1305_d_b9: f64 = (eq43_e1303_d_b9 * idt_scale);
        let eq43_e1305_d_b10: f64 = (eq43_e1303_d_b10 * idt_scale);
        let eq43_e1305_d_b11: f64 = (eq43_e1303_d_b11 * idt_scale);
        let eq43_e1305_d_b12: f64 = (eq43_e1303_d_b12 * idt_scale);
        let eq43_e1305_d_b13: f64 = (eq43_e1303_d_b13 * idt_scale);
        let eq43_e1305_d_b14: f64 = (eq43_e1303_d_b14 * idt_scale);
        let eq43_e1305_d_b15: f64 = (eq43_e1303_d_b15 * idt_scale);
        let eq43_e1305_d_b16: f64 = (eq43_e1303_d_b16 * idt_scale);
        let eq43_e1305_d_b17: f64 = (eq43_e1303_d_b17 * idt_scale);
        let eq43_e1305_d_b18: f64 = (eq43_e1303_d_b18 * idt_scale);
        let eq43_e1305_d_b19: f64 = (eq43_e1303_d_b19 * idt_scale);
        let eq43_e1305_d_b20: f64 = (eq43_e1303_d_b20 * idt_scale);
        let eq43_e1305_d_b21: f64 = (eq43_e1303_d_b21 * idt_scale);
        let eq43_e1305_d_b22: f64 = (eq43_e1303_d_b22 * idt_scale);
        let eq43_e1305_d_b23: f64 = (eq43_e1303_d_b23 * idt_scale);
        let eq43_e1305_d_b24: f64 = (eq43_e1303_d_b24 * idt_scale);
        let eq43_e1306: f64 = (s.v[4] * eq43_e1305);
        let eq43_e1306_d_n0: f64 = (s.v[4] * eq43_e1305_d_n0);
        let eq43_e1306_d_n1: f64 = (s.v[4] * eq43_e1305_d_n1);
        let eq43_e1306_d_n2: f64 = (s.v[4] * eq43_e1305_d_n2);
        let eq43_e1306_d_n3: f64 = (s.v[4] * eq43_e1305_d_n3);
        let eq43_e1306_d_n4: f64 = (s.v[4] * eq43_e1305_d_n4);
        let eq43_e1306_d_n5: f64 = (s.v[4] * eq43_e1305_d_n5);
        let eq43_e1306_d_n6: f64 = (s.v[4] * eq43_e1305_d_n6);
        let eq43_e1306_d_n7: f64 = (s.v[4] * eq43_e1305_d_n7);
        let eq43_e1306_d_n8: f64 = (s.v[4] * eq43_e1305_d_n8);
        let eq43_e1306_d_n9: f64 = (s.v[4] * eq43_e1305_d_n9);
        let eq43_e1306_d_n10: f64 = (s.v[4] * eq43_e1305_d_n10);
        let eq43_e1306_d_n11: f64 = (s.v[4] * eq43_e1305_d_n11);
        let eq43_e1306_d_n12: f64 = (s.v[4] * eq43_e1305_d_n12);
        let eq43_e1306_d_n13: f64 = (s.v[4] * eq43_e1305_d_n13);
        let eq43_e1306_d_n14: f64 = (s.v[4] * eq43_e1305_d_n14);
        let eq43_e1306_d_n15: f64 = (s.v[4] * eq43_e1305_d_n15);
        let eq43_e1306_d_n16: f64 = (s.v[4] * eq43_e1305_d_n16);
        let eq43_e1306_d_n17: f64 = (s.v[4] * eq43_e1305_d_n17);
        let eq43_e1306_d_n18: f64 = (s.v[4] * eq43_e1305_d_n18);
        let eq43_e1306_d_n19: f64 = (s.v[4] * eq43_e1305_d_n19);
        let eq43_e1306_d_n20: f64 = (s.v[4] * eq43_e1305_d_n20);
        let eq43_e1306_d_b0: f64 = (s.v[4] * eq43_e1305_d_b0);
        let eq43_e1306_d_b1: f64 = (s.v[4] * eq43_e1305_d_b1);
        let eq43_e1306_d_b2: f64 = (s.v[4] * eq43_e1305_d_b2);
        let eq43_e1306_d_b3: f64 = (s.v[4] * eq43_e1305_d_b3);
        let eq43_e1306_d_b4: f64 = (s.v[4] * eq43_e1305_d_b4);
        let eq43_e1306_d_b5: f64 = (s.v[4] * eq43_e1305_d_b5);
        let eq43_e1306_d_b6: f64 = (s.v[4] * eq43_e1305_d_b6);
        let eq43_e1306_d_b7: f64 = (s.v[4] * eq43_e1305_d_b7);
        let eq43_e1306_d_b8: f64 = (s.v[4] * eq43_e1305_d_b8);
        let eq43_e1306_d_b9: f64 = (s.v[4] * eq43_e1305_d_b9);
        let eq43_e1306_d_b10: f64 = (s.v[4] * eq43_e1305_d_b10);
        let eq43_e1306_d_b11: f64 = (s.v[4] * eq43_e1305_d_b11);
        let eq43_e1306_d_b12: f64 = (s.v[4] * eq43_e1305_d_b12);
        let eq43_e1306_d_b13: f64 = (s.v[4] * eq43_e1305_d_b13);
        let eq43_e1306_d_b14: f64 = (s.v[4] * eq43_e1305_d_b14);
        let eq43_e1306_d_b15: f64 = (s.v[4] * eq43_e1305_d_b15);
        let eq43_e1306_d_b16: f64 = (s.v[4] * eq43_e1305_d_b16);
        let eq43_e1306_d_b17: f64 = (s.v[4] * eq43_e1305_d_b17);
        let eq43_e1306_d_b18: f64 = (s.v[4] * eq43_e1305_d_b18);
        let eq43_e1306_d_b19: f64 = (s.v[4] * eq43_e1305_d_b19);
        let eq43_e1306_d_b20: f64 = (s.v[4] * eq43_e1305_d_b20);
        let eq43_e1306_d_b21: f64 = (s.v[4] * eq43_e1305_d_b21);
        let eq43_e1306_d_b22: f64 = (s.v[4] * eq43_e1305_d_b22);
        let eq43_e1306_d_b23: f64 = (s.v[4] * eq43_e1305_d_b23);
        let eq43_e1306_d_b24: f64 = (s.v[4] * eq43_e1305_d_b24);
        let eq43_value: f64 = eq43_e1306;
        let eq43_node_derivatives: [f64; 21] = [eq43_e1306_d_n0, eq43_e1306_d_n1, eq43_e1306_d_n2, eq43_e1306_d_n3, eq43_e1306_d_n4, eq43_e1306_d_n5, eq43_e1306_d_n6, eq43_e1306_d_n7, eq43_e1306_d_n8, eq43_e1306_d_n9, eq43_e1306_d_n10, eq43_e1306_d_n11, eq43_e1306_d_n12, eq43_e1306_d_n13, eq43_e1306_d_n14, eq43_e1306_d_n15, eq43_e1306_d_n16, eq43_e1306_d_n17, eq43_e1306_d_n18, eq43_e1306_d_n19, eq43_e1306_d_n20];
        let eq43_branch_derivatives: [f64; 25] = [eq43_e1306_d_b0, eq43_e1306_d_b1, eq43_e1306_d_b2, eq43_e1306_d_b3, eq43_e1306_d_b4, eq43_e1306_d_b5, eq43_e1306_d_b6, eq43_e1306_d_b7, eq43_e1306_d_b8, eq43_e1306_d_b9, eq43_e1306_d_b10, eq43_e1306_d_b11, eq43_e1306_d_b12, eq43_e1306_d_b13, eq43_e1306_d_b14, eq43_e1306_d_b15, eq43_e1306_d_b16, eq43_e1306_d_b17, eq43_e1306_d_b18, eq43_e1306_d_b19, eq43_e1306_d_b20, eq43_e1306_d_b21, eq43_e1306_d_b22, eq43_e1306_d_b23, eq43_e1306_d_b24];
        stamper.stamp_potential_dense_local(
            12,
            eq43_value,
            &eq43_node_derivatives,
            &eq43_branch_derivatives,
        );
    }

    pub(super) fn stamp_transient_equations_block_12(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        ddt_active: bool,
        idt_scale: f64,
        idt_state_current: &mut [f64; Instance::IDT_STATE_COUNT],
        idt_state_previous: &mut [f64; Instance::IDT_STATE_COUNT],
        idt_state_initialized: &mut [bool; Instance::IDT_STATE_COUNT],
    ) {
        let eq45_e1314: f64 = (-s.v[1995]);
        let eq45_e1314_d_n0: f64 = (-s.dn[1995][0]);
        let eq45_e1314_d_n1: f64 = (-s.dn[1995][1]);
        let eq45_e1314_d_n2: f64 = (-s.dn[1995][2]);
        let eq45_e1314_d_n3: f64 = (-s.dn[1995][3]);
        let eq45_e1314_d_n4: f64 = (-s.dn[1995][4]);
        let eq45_e1314_d_n5: f64 = (-s.dn[1995][5]);
        let eq45_e1314_d_n6: f64 = (-s.dn[1995][6]);
        let eq45_e1314_d_n7: f64 = (-s.dn[1995][7]);
        let eq45_e1314_d_n8: f64 = (-s.dn[1995][8]);
        let eq45_e1314_d_n9: f64 = (-s.dn[1995][9]);
        let eq45_e1314_d_n10: f64 = (-s.dn[1995][10]);
        let eq45_e1314_d_n11: f64 = (-s.dn[1995][11]);
        let eq45_e1314_d_n12: f64 = (-s.dn[1995][12]);
        let eq45_e1314_d_n13: f64 = (-s.dn[1995][13]);
        let eq45_e1314_d_n14: f64 = (-s.dn[1995][14]);
        let eq45_e1314_d_n15: f64 = (-s.dn[1995][15]);
        let eq45_e1314_d_n16: f64 = (-s.dn[1995][16]);
        let eq45_e1314_d_n17: f64 = (-s.dn[1995][17]);
        let eq45_e1314_d_n18: f64 = (-s.dn[1995][18]);
        let eq45_e1314_d_n19: f64 = (-s.dn[1995][19]);
        let eq45_e1314_d_n20: f64 = (-s.dn[1995][20]);
        let eq45_e1314_d_b0: f64 = (-s.db[1995][0]);
        let eq45_e1314_d_b1: f64 = (-s.db[1995][1]);
        let eq45_e1314_d_b2: f64 = (-s.db[1995][2]);
        let eq45_e1314_d_b3: f64 = (-s.db[1995][3]);
        let eq45_e1314_d_b4: f64 = (-s.db[1995][4]);
        let eq45_e1314_d_b5: f64 = (-s.db[1995][5]);
        let eq45_e1314_d_b6: f64 = (-s.db[1995][6]);
        let eq45_e1314_d_b7: f64 = (-s.db[1995][7]);
        let eq45_e1314_d_b8: f64 = (-s.db[1995][8]);
        let eq45_e1314_d_b9: f64 = (-s.db[1995][9]);
        let eq45_e1314_d_b10: f64 = (-s.db[1995][10]);
        let eq45_e1314_d_b11: f64 = (-s.db[1995][11]);
        let eq45_e1314_d_b12: f64 = (-s.db[1995][12]);
        let eq45_e1314_d_b13: f64 = (-s.db[1995][13]);
        let eq45_e1314_d_b14: f64 = (-s.db[1995][14]);
        let eq45_e1314_d_b15: f64 = (-s.db[1995][15]);
        let eq45_e1314_d_b16: f64 = (-s.db[1995][16]);
        let eq45_e1314_d_b17: f64 = (-s.db[1995][17]);
        let eq45_e1314_d_b18: f64 = (-s.db[1995][18]);
        let eq45_e1314_d_b19: f64 = (-s.db[1995][19]);
        let eq45_e1314_d_b20: f64 = (-s.db[1995][20]);
        let eq45_e1314_d_b21: f64 = (-s.db[1995][21]);
        let eq45_e1314_d_b22: f64 = (-s.db[1995][22]);
        let eq45_e1314_d_b23: f64 = (-s.db[1995][23]);
        let eq45_e1314_d_b24: f64 = (-s.db[1995][24]);
        let eq45_e1316: f64 = (eq45_e1314 * s.v[1954]);
        let eq45_e1316_d_n0: f64 = ((eq45_e1314_d_n0 * s.v[1954]) + (eq45_e1314 * s.dn[1954][0]));
        let eq45_e1316_d_n1: f64 = ((eq45_e1314_d_n1 * s.v[1954]) + (eq45_e1314 * s.dn[1954][1]));
        let eq45_e1316_d_n2: f64 = ((eq45_e1314_d_n2 * s.v[1954]) + (eq45_e1314 * s.dn[1954][2]));
        let eq45_e1316_d_n3: f64 = ((eq45_e1314_d_n3 * s.v[1954]) + (eq45_e1314 * s.dn[1954][3]));
        let eq45_e1316_d_n4: f64 = ((eq45_e1314_d_n4 * s.v[1954]) + (eq45_e1314 * s.dn[1954][4]));
        let eq45_e1316_d_n5: f64 = ((eq45_e1314_d_n5 * s.v[1954]) + (eq45_e1314 * s.dn[1954][5]));
        let eq45_e1316_d_n6: f64 = ((eq45_e1314_d_n6 * s.v[1954]) + (eq45_e1314 * s.dn[1954][6]));
        let eq45_e1316_d_n7: f64 = ((eq45_e1314_d_n7 * s.v[1954]) + (eq45_e1314 * s.dn[1954][7]));
        let eq45_e1316_d_n8: f64 = ((eq45_e1314_d_n8 * s.v[1954]) + (eq45_e1314 * s.dn[1954][8]));
        let eq45_e1316_d_n9: f64 = ((eq45_e1314_d_n9 * s.v[1954]) + (eq45_e1314 * s.dn[1954][9]));
        let eq45_e1316_d_n10: f64 = ((eq45_e1314_d_n10 * s.v[1954]) + (eq45_e1314 * s.dn[1954][10]));
        let eq45_e1316_d_n11: f64 = ((eq45_e1314_d_n11 * s.v[1954]) + (eq45_e1314 * s.dn[1954][11]));
        let eq45_e1316_d_n12: f64 = ((eq45_e1314_d_n12 * s.v[1954]) + (eq45_e1314 * s.dn[1954][12]));
        let eq45_e1316_d_n13: f64 = ((eq45_e1314_d_n13 * s.v[1954]) + (eq45_e1314 * s.dn[1954][13]));
        let eq45_e1316_d_n14: f64 = ((eq45_e1314_d_n14 * s.v[1954]) + (eq45_e1314 * s.dn[1954][14]));
        let eq45_e1316_d_n15: f64 = ((eq45_e1314_d_n15 * s.v[1954]) + (eq45_e1314 * s.dn[1954][15]));
        let eq45_e1316_d_n16: f64 = ((eq45_e1314_d_n16 * s.v[1954]) + (eq45_e1314 * s.dn[1954][16]));
        let eq45_e1316_d_n17: f64 = ((eq45_e1314_d_n17 * s.v[1954]) + (eq45_e1314 * s.dn[1954][17]));
        let eq45_e1316_d_n18: f64 = ((eq45_e1314_d_n18 * s.v[1954]) + (eq45_e1314 * s.dn[1954][18]));
        let eq45_e1316_d_n19: f64 = ((eq45_e1314_d_n19 * s.v[1954]) + (eq45_e1314 * s.dn[1954][19]));
        let eq45_e1316_d_n20: f64 = ((eq45_e1314_d_n20 * s.v[1954]) + (eq45_e1314 * s.dn[1954][20]));
        let eq45_e1316_d_b0: f64 = ((eq45_e1314_d_b0 * s.v[1954]) + (eq45_e1314 * s.db[1954][0]));
        let eq45_e1316_d_b1: f64 = ((eq45_e1314_d_b1 * s.v[1954]) + (eq45_e1314 * s.db[1954][1]));
        let eq45_e1316_d_b2: f64 = ((eq45_e1314_d_b2 * s.v[1954]) + (eq45_e1314 * s.db[1954][2]));
        let eq45_e1316_d_b3: f64 = ((eq45_e1314_d_b3 * s.v[1954]) + (eq45_e1314 * s.db[1954][3]));
        let eq45_e1316_d_b4: f64 = ((eq45_e1314_d_b4 * s.v[1954]) + (eq45_e1314 * s.db[1954][4]));
        let eq45_e1316_d_b5: f64 = ((eq45_e1314_d_b5 * s.v[1954]) + (eq45_e1314 * s.db[1954][5]));
        let eq45_e1316_d_b6: f64 = ((eq45_e1314_d_b6 * s.v[1954]) + (eq45_e1314 * s.db[1954][6]));
        let eq45_e1316_d_b7: f64 = ((eq45_e1314_d_b7 * s.v[1954]) + (eq45_e1314 * s.db[1954][7]));
        let eq45_e1316_d_b8: f64 = ((eq45_e1314_d_b8 * s.v[1954]) + (eq45_e1314 * s.db[1954][8]));
        let eq45_e1316_d_b9: f64 = ((eq45_e1314_d_b9 * s.v[1954]) + (eq45_e1314 * s.db[1954][9]));
        let eq45_e1316_d_b10: f64 = ((eq45_e1314_d_b10 * s.v[1954]) + (eq45_e1314 * s.db[1954][10]));
        let eq45_e1316_d_b11: f64 = ((eq45_e1314_d_b11 * s.v[1954]) + (eq45_e1314 * s.db[1954][11]));
        let eq45_e1316_d_b12: f64 = ((eq45_e1314_d_b12 * s.v[1954]) + (eq45_e1314 * s.db[1954][12]));
        let eq45_e1316_d_b13: f64 = ((eq45_e1314_d_b13 * s.v[1954]) + (eq45_e1314 * s.db[1954][13]));
        let eq45_e1316_d_b14: f64 = ((eq45_e1314_d_b14 * s.v[1954]) + (eq45_e1314 * s.db[1954][14]));
        let eq45_e1316_d_b15: f64 = ((eq45_e1314_d_b15 * s.v[1954]) + (eq45_e1314 * s.db[1954][15]));
        let eq45_e1316_d_b16: f64 = ((eq45_e1314_d_b16 * s.v[1954]) + (eq45_e1314 * s.db[1954][16]));
        let eq45_e1316_d_b17: f64 = ((eq45_e1314_d_b17 * s.v[1954]) + (eq45_e1314 * s.db[1954][17]));
        let eq45_e1316_d_b18: f64 = ((eq45_e1314_d_b18 * s.v[1954]) + (eq45_e1314 * s.db[1954][18]));
        let eq45_e1316_d_b19: f64 = ((eq45_e1314_d_b19 * s.v[1954]) + (eq45_e1314 * s.db[1954][19]));
        let eq45_e1316_d_b20: f64 = ((eq45_e1314_d_b20 * s.v[1954]) + (eq45_e1314 * s.db[1954][20]));
        let eq45_e1316_d_b21: f64 = ((eq45_e1314_d_b21 * s.v[1954]) + (eq45_e1314 * s.db[1954][21]));
        let eq45_e1316_d_b22: f64 = ((eq45_e1314_d_b22 * s.v[1954]) + (eq45_e1314 * s.db[1954][22]));
        let eq45_e1316_d_b23: f64 = ((eq45_e1314_d_b23 * s.v[1954]) + (eq45_e1314 * s.db[1954][23]));
        let eq45_e1316_d_b24: f64 = ((eq45_e1314_d_b24 * s.v[1954]) + (eq45_e1314 * s.db[1954][24]));
        let eq45_e1318: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 3, eq45_e1316, s.v[1945]);
        let eq45_e1318_d_n0: f64 = (eq45_e1316_d_n0 * idt_scale);
        let eq45_e1318_d_n1: f64 = (eq45_e1316_d_n1 * idt_scale);
        let eq45_e1318_d_n2: f64 = (eq45_e1316_d_n2 * idt_scale);
        let eq45_e1318_d_n3: f64 = (eq45_e1316_d_n3 * idt_scale);
        let eq45_e1318_d_n4: f64 = (eq45_e1316_d_n4 * idt_scale);
        let eq45_e1318_d_n5: f64 = (eq45_e1316_d_n5 * idt_scale);
        let eq45_e1318_d_n6: f64 = (eq45_e1316_d_n6 * idt_scale);
        let eq45_e1318_d_n7: f64 = (eq45_e1316_d_n7 * idt_scale);
        let eq45_e1318_d_n8: f64 = (eq45_e1316_d_n8 * idt_scale);
        let eq45_e1318_d_n9: f64 = (eq45_e1316_d_n9 * idt_scale);
        let eq45_e1318_d_n10: f64 = (eq45_e1316_d_n10 * idt_scale);
        let eq45_e1318_d_n11: f64 = (eq45_e1316_d_n11 * idt_scale);
        let eq45_e1318_d_n12: f64 = (eq45_e1316_d_n12 * idt_scale);
        let eq45_e1318_d_n13: f64 = (eq45_e1316_d_n13 * idt_scale);
        let eq45_e1318_d_n14: f64 = (eq45_e1316_d_n14 * idt_scale);
        let eq45_e1318_d_n15: f64 = (eq45_e1316_d_n15 * idt_scale);
        let eq45_e1318_d_n16: f64 = (eq45_e1316_d_n16 * idt_scale);
        let eq45_e1318_d_n17: f64 = (eq45_e1316_d_n17 * idt_scale);
        let eq45_e1318_d_n18: f64 = (eq45_e1316_d_n18 * idt_scale);
        let eq45_e1318_d_n19: f64 = (eq45_e1316_d_n19 * idt_scale);
        let eq45_e1318_d_n20: f64 = (eq45_e1316_d_n20 * idt_scale);
        let eq45_e1318_d_b0: f64 = (eq45_e1316_d_b0 * idt_scale);
        let eq45_e1318_d_b1: f64 = (eq45_e1316_d_b1 * idt_scale);
        let eq45_e1318_d_b2: f64 = (eq45_e1316_d_b2 * idt_scale);
        let eq45_e1318_d_b3: f64 = (eq45_e1316_d_b3 * idt_scale);
        let eq45_e1318_d_b4: f64 = (eq45_e1316_d_b4 * idt_scale);
        let eq45_e1318_d_b5: f64 = (eq45_e1316_d_b5 * idt_scale);
        let eq45_e1318_d_b6: f64 = (eq45_e1316_d_b6 * idt_scale);
        let eq45_e1318_d_b7: f64 = (eq45_e1316_d_b7 * idt_scale);
        let eq45_e1318_d_b8: f64 = (eq45_e1316_d_b8 * idt_scale);
        let eq45_e1318_d_b9: f64 = (eq45_e1316_d_b9 * idt_scale);
        let eq45_e1318_d_b10: f64 = (eq45_e1316_d_b10 * idt_scale);
        let eq45_e1318_d_b11: f64 = (eq45_e1316_d_b11 * idt_scale);
        let eq45_e1318_d_b12: f64 = (eq45_e1316_d_b12 * idt_scale);
        let eq45_e1318_d_b13: f64 = (eq45_e1316_d_b13 * idt_scale);
        let eq45_e1318_d_b14: f64 = (eq45_e1316_d_b14 * idt_scale);
        let eq45_e1318_d_b15: f64 = (eq45_e1316_d_b15 * idt_scale);
        let eq45_e1318_d_b16: f64 = (eq45_e1316_d_b16 * idt_scale);
        let eq45_e1318_d_b17: f64 = (eq45_e1316_d_b17 * idt_scale);
        let eq45_e1318_d_b18: f64 = (eq45_e1316_d_b18 * idt_scale);
        let eq45_e1318_d_b19: f64 = (eq45_e1316_d_b19 * idt_scale);
        let eq45_e1318_d_b20: f64 = (eq45_e1316_d_b20 * idt_scale);
        let eq45_e1318_d_b21: f64 = (eq45_e1316_d_b21 * idt_scale);
        let eq45_e1318_d_b22: f64 = (eq45_e1316_d_b22 * idt_scale);
        let eq45_e1318_d_b23: f64 = (eq45_e1316_d_b23 * idt_scale);
        let eq45_e1318_d_b24: f64 = (eq45_e1316_d_b24 * idt_scale);
        let eq45_e1319: f64 = (s.v[4] * eq45_e1318);
        let eq45_e1319_d_n0: f64 = (s.v[4] * eq45_e1318_d_n0);
        let eq45_e1319_d_n1: f64 = (s.v[4] * eq45_e1318_d_n1);
        let eq45_e1319_d_n2: f64 = (s.v[4] * eq45_e1318_d_n2);
        let eq45_e1319_d_n3: f64 = (s.v[4] * eq45_e1318_d_n3);
        let eq45_e1319_d_n4: f64 = (s.v[4] * eq45_e1318_d_n4);
        let eq45_e1319_d_n5: f64 = (s.v[4] * eq45_e1318_d_n5);
        let eq45_e1319_d_n6: f64 = (s.v[4] * eq45_e1318_d_n6);
        let eq45_e1319_d_n7: f64 = (s.v[4] * eq45_e1318_d_n7);
        let eq45_e1319_d_n8: f64 = (s.v[4] * eq45_e1318_d_n8);
        let eq45_e1319_d_n9: f64 = (s.v[4] * eq45_e1318_d_n9);
        let eq45_e1319_d_n10: f64 = (s.v[4] * eq45_e1318_d_n10);
        let eq45_e1319_d_n11: f64 = (s.v[4] * eq45_e1318_d_n11);
        let eq45_e1319_d_n12: f64 = (s.v[4] * eq45_e1318_d_n12);
        let eq45_e1319_d_n13: f64 = (s.v[4] * eq45_e1318_d_n13);
        let eq45_e1319_d_n14: f64 = (s.v[4] * eq45_e1318_d_n14);
        let eq45_e1319_d_n15: f64 = (s.v[4] * eq45_e1318_d_n15);
        let eq45_e1319_d_n16: f64 = (s.v[4] * eq45_e1318_d_n16);
        let eq45_e1319_d_n17: f64 = (s.v[4] * eq45_e1318_d_n17);
        let eq45_e1319_d_n18: f64 = (s.v[4] * eq45_e1318_d_n18);
        let eq45_e1319_d_n19: f64 = (s.v[4] * eq45_e1318_d_n19);
        let eq45_e1319_d_n20: f64 = (s.v[4] * eq45_e1318_d_n20);
        let eq45_e1319_d_b0: f64 = (s.v[4] * eq45_e1318_d_b0);
        let eq45_e1319_d_b1: f64 = (s.v[4] * eq45_e1318_d_b1);
        let eq45_e1319_d_b2: f64 = (s.v[4] * eq45_e1318_d_b2);
        let eq45_e1319_d_b3: f64 = (s.v[4] * eq45_e1318_d_b3);
        let eq45_e1319_d_b4: f64 = (s.v[4] * eq45_e1318_d_b4);
        let eq45_e1319_d_b5: f64 = (s.v[4] * eq45_e1318_d_b5);
        let eq45_e1319_d_b6: f64 = (s.v[4] * eq45_e1318_d_b6);
        let eq45_e1319_d_b7: f64 = (s.v[4] * eq45_e1318_d_b7);
        let eq45_e1319_d_b8: f64 = (s.v[4] * eq45_e1318_d_b8);
        let eq45_e1319_d_b9: f64 = (s.v[4] * eq45_e1318_d_b9);
        let eq45_e1319_d_b10: f64 = (s.v[4] * eq45_e1318_d_b10);
        let eq45_e1319_d_b11: f64 = (s.v[4] * eq45_e1318_d_b11);
        let eq45_e1319_d_b12: f64 = (s.v[4] * eq45_e1318_d_b12);
        let eq45_e1319_d_b13: f64 = (s.v[4] * eq45_e1318_d_b13);
        let eq45_e1319_d_b14: f64 = (s.v[4] * eq45_e1318_d_b14);
        let eq45_e1319_d_b15: f64 = (s.v[4] * eq45_e1318_d_b15);
        let eq45_e1319_d_b16: f64 = (s.v[4] * eq45_e1318_d_b16);
        let eq45_e1319_d_b17: f64 = (s.v[4] * eq45_e1318_d_b17);
        let eq45_e1319_d_b18: f64 = (s.v[4] * eq45_e1318_d_b18);
        let eq45_e1319_d_b19: f64 = (s.v[4] * eq45_e1318_d_b19);
        let eq45_e1319_d_b20: f64 = (s.v[4] * eq45_e1318_d_b20);
        let eq45_e1319_d_b21: f64 = (s.v[4] * eq45_e1318_d_b21);
        let eq45_e1319_d_b22: f64 = (s.v[4] * eq45_e1318_d_b22);
        let eq45_e1319_d_b23: f64 = (s.v[4] * eq45_e1318_d_b23);
        let eq45_e1319_d_b24: f64 = (s.v[4] * eq45_e1318_d_b24);
        let eq45_value: f64 = eq45_e1319;
        let eq45_node_derivatives: [f64; 21] = [eq45_e1319_d_n0, eq45_e1319_d_n1, eq45_e1319_d_n2, eq45_e1319_d_n3, eq45_e1319_d_n4, eq45_e1319_d_n5, eq45_e1319_d_n6, eq45_e1319_d_n7, eq45_e1319_d_n8, eq45_e1319_d_n9, eq45_e1319_d_n10, eq45_e1319_d_n11, eq45_e1319_d_n12, eq45_e1319_d_n13, eq45_e1319_d_n14, eq45_e1319_d_n15, eq45_e1319_d_n16, eq45_e1319_d_n17, eq45_e1319_d_n18, eq45_e1319_d_n19, eq45_e1319_d_n20];
        let eq45_branch_derivatives: [f64; 25] = [eq45_e1319_d_b0, eq45_e1319_d_b1, eq45_e1319_d_b2, eq45_e1319_d_b3, eq45_e1319_d_b4, eq45_e1319_d_b5, eq45_e1319_d_b6, eq45_e1319_d_b7, eq45_e1319_d_b8, eq45_e1319_d_b9, eq45_e1319_d_b10, eq45_e1319_d_b11, eq45_e1319_d_b12, eq45_e1319_d_b13, eq45_e1319_d_b14, eq45_e1319_d_b15, eq45_e1319_d_b16, eq45_e1319_d_b17, eq45_e1319_d_b18, eq45_e1319_d_b19, eq45_e1319_d_b20, eq45_e1319_d_b21, eq45_e1319_d_b22, eq45_e1319_d_b23, eq45_e1319_d_b24];
        stamper.stamp_potential_dense_local(
            14,
            eq45_value,
            &eq45_node_derivatives,
            &eq45_branch_derivatives,
        );
    }

    pub(super) fn stamp_transient_equations_block_13(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        ddt_active: bool,
        idt_scale: f64,
        idt_state_current: &mut [f64; Instance::IDT_STATE_COUNT],
        idt_state_previous: &mut [f64; Instance::IDT_STATE_COUNT],
        idt_state_initialized: &mut [bool; Instance::IDT_STATE_COUNT],
    ) {
        let eq47_e1327: f64 = (-s.v[1995]);
        let eq47_e1327_d_n0: f64 = (-s.dn[1995][0]);
        let eq47_e1327_d_n1: f64 = (-s.dn[1995][1]);
        let eq47_e1327_d_n2: f64 = (-s.dn[1995][2]);
        let eq47_e1327_d_n3: f64 = (-s.dn[1995][3]);
        let eq47_e1327_d_n4: f64 = (-s.dn[1995][4]);
        let eq47_e1327_d_n5: f64 = (-s.dn[1995][5]);
        let eq47_e1327_d_n6: f64 = (-s.dn[1995][6]);
        let eq47_e1327_d_n7: f64 = (-s.dn[1995][7]);
        let eq47_e1327_d_n8: f64 = (-s.dn[1995][8]);
        let eq47_e1327_d_n9: f64 = (-s.dn[1995][9]);
        let eq47_e1327_d_n10: f64 = (-s.dn[1995][10]);
        let eq47_e1327_d_n11: f64 = (-s.dn[1995][11]);
        let eq47_e1327_d_n12: f64 = (-s.dn[1995][12]);
        let eq47_e1327_d_n13: f64 = (-s.dn[1995][13]);
        let eq47_e1327_d_n14: f64 = (-s.dn[1995][14]);
        let eq47_e1327_d_n15: f64 = (-s.dn[1995][15]);
        let eq47_e1327_d_n16: f64 = (-s.dn[1995][16]);
        let eq47_e1327_d_n17: f64 = (-s.dn[1995][17]);
        let eq47_e1327_d_n18: f64 = (-s.dn[1995][18]);
        let eq47_e1327_d_n19: f64 = (-s.dn[1995][19]);
        let eq47_e1327_d_n20: f64 = (-s.dn[1995][20]);
        let eq47_e1327_d_b0: f64 = (-s.db[1995][0]);
        let eq47_e1327_d_b1: f64 = (-s.db[1995][1]);
        let eq47_e1327_d_b2: f64 = (-s.db[1995][2]);
        let eq47_e1327_d_b3: f64 = (-s.db[1995][3]);
        let eq47_e1327_d_b4: f64 = (-s.db[1995][4]);
        let eq47_e1327_d_b5: f64 = (-s.db[1995][5]);
        let eq47_e1327_d_b6: f64 = (-s.db[1995][6]);
        let eq47_e1327_d_b7: f64 = (-s.db[1995][7]);
        let eq47_e1327_d_b8: f64 = (-s.db[1995][8]);
        let eq47_e1327_d_b9: f64 = (-s.db[1995][9]);
        let eq47_e1327_d_b10: f64 = (-s.db[1995][10]);
        let eq47_e1327_d_b11: f64 = (-s.db[1995][11]);
        let eq47_e1327_d_b12: f64 = (-s.db[1995][12]);
        let eq47_e1327_d_b13: f64 = (-s.db[1995][13]);
        let eq47_e1327_d_b14: f64 = (-s.db[1995][14]);
        let eq47_e1327_d_b15: f64 = (-s.db[1995][15]);
        let eq47_e1327_d_b16: f64 = (-s.db[1995][16]);
        let eq47_e1327_d_b17: f64 = (-s.db[1995][17]);
        let eq47_e1327_d_b18: f64 = (-s.db[1995][18]);
        let eq47_e1327_d_b19: f64 = (-s.db[1995][19]);
        let eq47_e1327_d_b20: f64 = (-s.db[1995][20]);
        let eq47_e1327_d_b21: f64 = (-s.db[1995][21]);
        let eq47_e1327_d_b22: f64 = (-s.db[1995][22]);
        let eq47_e1327_d_b23: f64 = (-s.db[1995][23]);
        let eq47_e1327_d_b24: f64 = (-s.db[1995][24]);
        let eq47_e1329: f64 = (eq47_e1327 * s.v[1955]);
        let eq47_e1329_d_n0: f64 = ((eq47_e1327_d_n0 * s.v[1955]) + (eq47_e1327 * s.dn[1955][0]));
        let eq47_e1329_d_n1: f64 = ((eq47_e1327_d_n1 * s.v[1955]) + (eq47_e1327 * s.dn[1955][1]));
        let eq47_e1329_d_n2: f64 = ((eq47_e1327_d_n2 * s.v[1955]) + (eq47_e1327 * s.dn[1955][2]));
        let eq47_e1329_d_n3: f64 = ((eq47_e1327_d_n3 * s.v[1955]) + (eq47_e1327 * s.dn[1955][3]));
        let eq47_e1329_d_n4: f64 = ((eq47_e1327_d_n4 * s.v[1955]) + (eq47_e1327 * s.dn[1955][4]));
        let eq47_e1329_d_n5: f64 = ((eq47_e1327_d_n5 * s.v[1955]) + (eq47_e1327 * s.dn[1955][5]));
        let eq47_e1329_d_n6: f64 = ((eq47_e1327_d_n6 * s.v[1955]) + (eq47_e1327 * s.dn[1955][6]));
        let eq47_e1329_d_n7: f64 = ((eq47_e1327_d_n7 * s.v[1955]) + (eq47_e1327 * s.dn[1955][7]));
        let eq47_e1329_d_n8: f64 = ((eq47_e1327_d_n8 * s.v[1955]) + (eq47_e1327 * s.dn[1955][8]));
        let eq47_e1329_d_n9: f64 = ((eq47_e1327_d_n9 * s.v[1955]) + (eq47_e1327 * s.dn[1955][9]));
        let eq47_e1329_d_n10: f64 = ((eq47_e1327_d_n10 * s.v[1955]) + (eq47_e1327 * s.dn[1955][10]));
        let eq47_e1329_d_n11: f64 = ((eq47_e1327_d_n11 * s.v[1955]) + (eq47_e1327 * s.dn[1955][11]));
        let eq47_e1329_d_n12: f64 = ((eq47_e1327_d_n12 * s.v[1955]) + (eq47_e1327 * s.dn[1955][12]));
        let eq47_e1329_d_n13: f64 = ((eq47_e1327_d_n13 * s.v[1955]) + (eq47_e1327 * s.dn[1955][13]));
        let eq47_e1329_d_n14: f64 = ((eq47_e1327_d_n14 * s.v[1955]) + (eq47_e1327 * s.dn[1955][14]));
        let eq47_e1329_d_n15: f64 = ((eq47_e1327_d_n15 * s.v[1955]) + (eq47_e1327 * s.dn[1955][15]));
        let eq47_e1329_d_n16: f64 = ((eq47_e1327_d_n16 * s.v[1955]) + (eq47_e1327 * s.dn[1955][16]));
        let eq47_e1329_d_n17: f64 = ((eq47_e1327_d_n17 * s.v[1955]) + (eq47_e1327 * s.dn[1955][17]));
        let eq47_e1329_d_n18: f64 = ((eq47_e1327_d_n18 * s.v[1955]) + (eq47_e1327 * s.dn[1955][18]));
        let eq47_e1329_d_n19: f64 = ((eq47_e1327_d_n19 * s.v[1955]) + (eq47_e1327 * s.dn[1955][19]));
        let eq47_e1329_d_n20: f64 = ((eq47_e1327_d_n20 * s.v[1955]) + (eq47_e1327 * s.dn[1955][20]));
        let eq47_e1329_d_b0: f64 = ((eq47_e1327_d_b0 * s.v[1955]) + (eq47_e1327 * s.db[1955][0]));
        let eq47_e1329_d_b1: f64 = ((eq47_e1327_d_b1 * s.v[1955]) + (eq47_e1327 * s.db[1955][1]));
        let eq47_e1329_d_b2: f64 = ((eq47_e1327_d_b2 * s.v[1955]) + (eq47_e1327 * s.db[1955][2]));
        let eq47_e1329_d_b3: f64 = ((eq47_e1327_d_b3 * s.v[1955]) + (eq47_e1327 * s.db[1955][3]));
        let eq47_e1329_d_b4: f64 = ((eq47_e1327_d_b4 * s.v[1955]) + (eq47_e1327 * s.db[1955][4]));
        let eq47_e1329_d_b5: f64 = ((eq47_e1327_d_b5 * s.v[1955]) + (eq47_e1327 * s.db[1955][5]));
        let eq47_e1329_d_b6: f64 = ((eq47_e1327_d_b6 * s.v[1955]) + (eq47_e1327 * s.db[1955][6]));
        let eq47_e1329_d_b7: f64 = ((eq47_e1327_d_b7 * s.v[1955]) + (eq47_e1327 * s.db[1955][7]));
        let eq47_e1329_d_b8: f64 = ((eq47_e1327_d_b8 * s.v[1955]) + (eq47_e1327 * s.db[1955][8]));
        let eq47_e1329_d_b9: f64 = ((eq47_e1327_d_b9 * s.v[1955]) + (eq47_e1327 * s.db[1955][9]));
        let eq47_e1329_d_b10: f64 = ((eq47_e1327_d_b10 * s.v[1955]) + (eq47_e1327 * s.db[1955][10]));
        let eq47_e1329_d_b11: f64 = ((eq47_e1327_d_b11 * s.v[1955]) + (eq47_e1327 * s.db[1955][11]));
        let eq47_e1329_d_b12: f64 = ((eq47_e1327_d_b12 * s.v[1955]) + (eq47_e1327 * s.db[1955][12]));
        let eq47_e1329_d_b13: f64 = ((eq47_e1327_d_b13 * s.v[1955]) + (eq47_e1327 * s.db[1955][13]));
        let eq47_e1329_d_b14: f64 = ((eq47_e1327_d_b14 * s.v[1955]) + (eq47_e1327 * s.db[1955][14]));
        let eq47_e1329_d_b15: f64 = ((eq47_e1327_d_b15 * s.v[1955]) + (eq47_e1327 * s.db[1955][15]));
        let eq47_e1329_d_b16: f64 = ((eq47_e1327_d_b16 * s.v[1955]) + (eq47_e1327 * s.db[1955][16]));
        let eq47_e1329_d_b17: f64 = ((eq47_e1327_d_b17 * s.v[1955]) + (eq47_e1327 * s.db[1955][17]));
        let eq47_e1329_d_b18: f64 = ((eq47_e1327_d_b18 * s.v[1955]) + (eq47_e1327 * s.db[1955][18]));
        let eq47_e1329_d_b19: f64 = ((eq47_e1327_d_b19 * s.v[1955]) + (eq47_e1327 * s.db[1955][19]));
        let eq47_e1329_d_b20: f64 = ((eq47_e1327_d_b20 * s.v[1955]) + (eq47_e1327 * s.db[1955][20]));
        let eq47_e1329_d_b21: f64 = ((eq47_e1327_d_b21 * s.v[1955]) + (eq47_e1327 * s.db[1955][21]));
        let eq47_e1329_d_b22: f64 = ((eq47_e1327_d_b22 * s.v[1955]) + (eq47_e1327 * s.db[1955][22]));
        let eq47_e1329_d_b23: f64 = ((eq47_e1327_d_b23 * s.v[1955]) + (eq47_e1327 * s.db[1955][23]));
        let eq47_e1329_d_b24: f64 = ((eq47_e1327_d_b24 * s.v[1955]) + (eq47_e1327 * s.db[1955][24]));
        let eq47_e1331: f64 = eval_idt(idt_state_current, idt_state_previous, idt_state_initialized, ddt_active, idt_scale, 4, eq47_e1329, s.v[1946]);
        let eq47_e1331_d_n0: f64 = (eq47_e1329_d_n0 * idt_scale);
        let eq47_e1331_d_n1: f64 = (eq47_e1329_d_n1 * idt_scale);
        let eq47_e1331_d_n2: f64 = (eq47_e1329_d_n2 * idt_scale);
        let eq47_e1331_d_n3: f64 = (eq47_e1329_d_n3 * idt_scale);
        let eq47_e1331_d_n4: f64 = (eq47_e1329_d_n4 * idt_scale);
        let eq47_e1331_d_n5: f64 = (eq47_e1329_d_n5 * idt_scale);
        let eq47_e1331_d_n6: f64 = (eq47_e1329_d_n6 * idt_scale);
        let eq47_e1331_d_n7: f64 = (eq47_e1329_d_n7 * idt_scale);
        let eq47_e1331_d_n8: f64 = (eq47_e1329_d_n8 * idt_scale);
        let eq47_e1331_d_n9: f64 = (eq47_e1329_d_n9 * idt_scale);
        let eq47_e1331_d_n10: f64 = (eq47_e1329_d_n10 * idt_scale);
        let eq47_e1331_d_n11: f64 = (eq47_e1329_d_n11 * idt_scale);
        let eq47_e1331_d_n12: f64 = (eq47_e1329_d_n12 * idt_scale);
        let eq47_e1331_d_n13: f64 = (eq47_e1329_d_n13 * idt_scale);
        let eq47_e1331_d_n14: f64 = (eq47_e1329_d_n14 * idt_scale);
        let eq47_e1331_d_n15: f64 = (eq47_e1329_d_n15 * idt_scale);
        let eq47_e1331_d_n16: f64 = (eq47_e1329_d_n16 * idt_scale);
        let eq47_e1331_d_n17: f64 = (eq47_e1329_d_n17 * idt_scale);
        let eq47_e1331_d_n18: f64 = (eq47_e1329_d_n18 * idt_scale);
        let eq47_e1331_d_n19: f64 = (eq47_e1329_d_n19 * idt_scale);
        let eq47_e1331_d_n20: f64 = (eq47_e1329_d_n20 * idt_scale);
        let eq47_e1331_d_b0: f64 = (eq47_e1329_d_b0 * idt_scale);
        let eq47_e1331_d_b1: f64 = (eq47_e1329_d_b1 * idt_scale);
        let eq47_e1331_d_b2: f64 = (eq47_e1329_d_b2 * idt_scale);
        let eq47_e1331_d_b3: f64 = (eq47_e1329_d_b3 * idt_scale);
        let eq47_e1331_d_b4: f64 = (eq47_e1329_d_b4 * idt_scale);
        let eq47_e1331_d_b5: f64 = (eq47_e1329_d_b5 * idt_scale);
        let eq47_e1331_d_b6: f64 = (eq47_e1329_d_b6 * idt_scale);
        let eq47_e1331_d_b7: f64 = (eq47_e1329_d_b7 * idt_scale);
        let eq47_e1331_d_b8: f64 = (eq47_e1329_d_b8 * idt_scale);
        let eq47_e1331_d_b9: f64 = (eq47_e1329_d_b9 * idt_scale);
        let eq47_e1331_d_b10: f64 = (eq47_e1329_d_b10 * idt_scale);
        let eq47_e1331_d_b11: f64 = (eq47_e1329_d_b11 * idt_scale);
        let eq47_e1331_d_b12: f64 = (eq47_e1329_d_b12 * idt_scale);
        let eq47_e1331_d_b13: f64 = (eq47_e1329_d_b13 * idt_scale);
        let eq47_e1331_d_b14: f64 = (eq47_e1329_d_b14 * idt_scale);
        let eq47_e1331_d_b15: f64 = (eq47_e1329_d_b15 * idt_scale);
        let eq47_e1331_d_b16: f64 = (eq47_e1329_d_b16 * idt_scale);
        let eq47_e1331_d_b17: f64 = (eq47_e1329_d_b17 * idt_scale);
        let eq47_e1331_d_b18: f64 = (eq47_e1329_d_b18 * idt_scale);
        let eq47_e1331_d_b19: f64 = (eq47_e1329_d_b19 * idt_scale);
        let eq47_e1331_d_b20: f64 = (eq47_e1329_d_b20 * idt_scale);
        let eq47_e1331_d_b21: f64 = (eq47_e1329_d_b21 * idt_scale);
        let eq47_e1331_d_b22: f64 = (eq47_e1329_d_b22 * idt_scale);
        let eq47_e1331_d_b23: f64 = (eq47_e1329_d_b23 * idt_scale);
        let eq47_e1331_d_b24: f64 = (eq47_e1329_d_b24 * idt_scale);
        let eq47_e1332: f64 = (s.v[4] * eq47_e1331);
        let eq47_e1332_d_n0: f64 = (s.v[4] * eq47_e1331_d_n0);
        let eq47_e1332_d_n1: f64 = (s.v[4] * eq47_e1331_d_n1);
        let eq47_e1332_d_n2: f64 = (s.v[4] * eq47_e1331_d_n2);
        let eq47_e1332_d_n3: f64 = (s.v[4] * eq47_e1331_d_n3);
        let eq47_e1332_d_n4: f64 = (s.v[4] * eq47_e1331_d_n4);
        let eq47_e1332_d_n5: f64 = (s.v[4] * eq47_e1331_d_n5);
        let eq47_e1332_d_n6: f64 = (s.v[4] * eq47_e1331_d_n6);
        let eq47_e1332_d_n7: f64 = (s.v[4] * eq47_e1331_d_n7);
        let eq47_e1332_d_n8: f64 = (s.v[4] * eq47_e1331_d_n8);
        let eq47_e1332_d_n9: f64 = (s.v[4] * eq47_e1331_d_n9);
        let eq47_e1332_d_n10: f64 = (s.v[4] * eq47_e1331_d_n10);
        let eq47_e1332_d_n11: f64 = (s.v[4] * eq47_e1331_d_n11);
        let eq47_e1332_d_n12: f64 = (s.v[4] * eq47_e1331_d_n12);
        let eq47_e1332_d_n13: f64 = (s.v[4] * eq47_e1331_d_n13);
        let eq47_e1332_d_n14: f64 = (s.v[4] * eq47_e1331_d_n14);
        let eq47_e1332_d_n15: f64 = (s.v[4] * eq47_e1331_d_n15);
        let eq47_e1332_d_n16: f64 = (s.v[4] * eq47_e1331_d_n16);
        let eq47_e1332_d_n17: f64 = (s.v[4] * eq47_e1331_d_n17);
        let eq47_e1332_d_n18: f64 = (s.v[4] * eq47_e1331_d_n18);
        let eq47_e1332_d_n19: f64 = (s.v[4] * eq47_e1331_d_n19);
        let eq47_e1332_d_n20: f64 = (s.v[4] * eq47_e1331_d_n20);
        let eq47_e1332_d_b0: f64 = (s.v[4] * eq47_e1331_d_b0);
        let eq47_e1332_d_b1: f64 = (s.v[4] * eq47_e1331_d_b1);
        let eq47_e1332_d_b2: f64 = (s.v[4] * eq47_e1331_d_b2);
        let eq47_e1332_d_b3: f64 = (s.v[4] * eq47_e1331_d_b3);
        let eq47_e1332_d_b4: f64 = (s.v[4] * eq47_e1331_d_b4);
        let eq47_e1332_d_b5: f64 = (s.v[4] * eq47_e1331_d_b5);
        let eq47_e1332_d_b6: f64 = (s.v[4] * eq47_e1331_d_b6);
        let eq47_e1332_d_b7: f64 = (s.v[4] * eq47_e1331_d_b7);
        let eq47_e1332_d_b8: f64 = (s.v[4] * eq47_e1331_d_b8);
        let eq47_e1332_d_b9: f64 = (s.v[4] * eq47_e1331_d_b9);
        let eq47_e1332_d_b10: f64 = (s.v[4] * eq47_e1331_d_b10);
        let eq47_e1332_d_b11: f64 = (s.v[4] * eq47_e1331_d_b11);
        let eq47_e1332_d_b12: f64 = (s.v[4] * eq47_e1331_d_b12);
        let eq47_e1332_d_b13: f64 = (s.v[4] * eq47_e1331_d_b13);
        let eq47_e1332_d_b14: f64 = (s.v[4] * eq47_e1331_d_b14);
        let eq47_e1332_d_b15: f64 = (s.v[4] * eq47_e1331_d_b15);
        let eq47_e1332_d_b16: f64 = (s.v[4] * eq47_e1331_d_b16);
        let eq47_e1332_d_b17: f64 = (s.v[4] * eq47_e1331_d_b17);
        let eq47_e1332_d_b18: f64 = (s.v[4] * eq47_e1331_d_b18);
        let eq47_e1332_d_b19: f64 = (s.v[4] * eq47_e1331_d_b19);
        let eq47_e1332_d_b20: f64 = (s.v[4] * eq47_e1331_d_b20);
        let eq47_e1332_d_b21: f64 = (s.v[4] * eq47_e1331_d_b21);
        let eq47_e1332_d_b22: f64 = (s.v[4] * eq47_e1331_d_b22);
        let eq47_e1332_d_b23: f64 = (s.v[4] * eq47_e1331_d_b23);
        let eq47_e1332_d_b24: f64 = (s.v[4] * eq47_e1331_d_b24);
        let eq47_value: f64 = eq47_e1332;
        let eq47_node_derivatives: [f64; 21] = [eq47_e1332_d_n0, eq47_e1332_d_n1, eq47_e1332_d_n2, eq47_e1332_d_n3, eq47_e1332_d_n4, eq47_e1332_d_n5, eq47_e1332_d_n6, eq47_e1332_d_n7, eq47_e1332_d_n8, eq47_e1332_d_n9, eq47_e1332_d_n10, eq47_e1332_d_n11, eq47_e1332_d_n12, eq47_e1332_d_n13, eq47_e1332_d_n14, eq47_e1332_d_n15, eq47_e1332_d_n16, eq47_e1332_d_n17, eq47_e1332_d_n18, eq47_e1332_d_n19, eq47_e1332_d_n20];
        let eq47_branch_derivatives: [f64; 25] = [eq47_e1332_d_b0, eq47_e1332_d_b1, eq47_e1332_d_b2, eq47_e1332_d_b3, eq47_e1332_d_b4, eq47_e1332_d_b5, eq47_e1332_d_b6, eq47_e1332_d_b7, eq47_e1332_d_b8, eq47_e1332_d_b9, eq47_e1332_d_b10, eq47_e1332_d_b11, eq47_e1332_d_b12, eq47_e1332_d_b13, eq47_e1332_d_b14, eq47_e1332_d_b15, eq47_e1332_d_b16, eq47_e1332_d_b17, eq47_e1332_d_b18, eq47_e1332_d_b19, eq47_e1332_d_b20, eq47_e1332_d_b21, eq47_e1332_d_b22, eq47_e1332_d_b23, eq47_e1332_d_b24];
        stamper.stamp_potential_dense_local(
            16,
            eq47_value,
            &eq47_node_derivatives,
            &eq47_branch_derivatives,
        );
    }
}

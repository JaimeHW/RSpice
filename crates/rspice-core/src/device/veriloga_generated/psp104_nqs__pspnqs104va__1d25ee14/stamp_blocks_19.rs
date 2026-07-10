#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
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
        if ((s.b[2991] && (!s.b[2994])) && s.b[2995]) {s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);s.store_add(824, 2002, 2003);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);s.store_add_mixed_ia(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));}
        s.b[2996] = (((s.v[2015]) as f64).abs() < 230.25850929940458);s.store_scalar(2996, if s.b[2996] { 1.0 } else { 0.0 });
        if (((s.b[2991] && (!s.b[2994])) && s.b[2995]) && s.b[2996]) {s.store_exp(2005, 2015);}
        s.b[2997] = (s.v[2015] < 0.0);s.store_scalar(2997, if s.b[2997] { 1.0 } else { 0.0 });
        if ((((s.b[2991] && (!s.b[2994])) && s.b[2995]) && (!s.b[2996])) && s.b[2997]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2991] && (!s.b[2994])) && s.b[2995]) && (!s.b[2996])) && (!s.b[2997])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2991] && (!s.b[2994])) && s.b[2995]) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs_product_mixed_iiia(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_neg_add(2016, 2015, 2012);}
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
        if ((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_add(2016, 2013, 2014);}
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
        if ((s.b[3007] && (!s.b[3009])) && s.b[3010]) {s.store_neg(1999, 2027);s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);s.store_add(824, 2002, 2003);}
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
        if ((s.b[3007] && (!s.b[3009])) && s.b[3010]) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs_product_mixed_iiia(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_neg_add(2016, 2015, 2012);}
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
        if ((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) {s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_add(2016, 2013, 2014);}
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
        if ((s.b[3022] && (!s.b[3024])) && s.b[3025]) {s.store_neg(1999, 2027);s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);s.store_add(824, 2002, 2003);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);s.store_add_mixed_ia(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));}
        s.b[3026] = (((s.v[2015]) as f64).abs() < 230.25850929940458);s.store_scalar(3026, if s.b[3026] { 1.0 } else { 0.0 });
        if (((s.b[3022] && (!s.b[3024])) && s.b[3025]) && s.b[3026]) {s.store_exp(2005, 2015);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_101(
        s: &mut ReactiveScratch,
    ) {
        s.b[3027] = (s.v[2015] < 0.0);s.store_scalar(3027, if s.b[3027] { 1.0 } else { 0.0 });
        if ((((s.b[3022] && (!s.b[3024])) && s.b[3025]) && (!s.b[3026])) && s.b[3027]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[3022] && (!s.b[3024])) && s.b[3025]) && (!s.b[3026])) && (!s.b[3027])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[3022] && (!s.b[3024])) && s.b[3025]) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs_product_mixed_iiia(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_neg_add(2016, 2015, 2012);}
        if ((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) {s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);s.store_mul_scale_offset_mixed_ia(2010, 1998, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), 1.0, (-1.0));s.store_mul_scale_offset(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0, 1.0);}
        s.b[3028] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);s.store_scalar(3028, if s.b[3028] { 1.0 } else { 0.0 });
        if (((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) && s.b[3028]) {s.store_exp_neg_input(2009, 2011);}
        s.b[3029] = ((-s.v[2011]) < 0.0);s.store_scalar(3029, if s.b[3029] { 1.0 } else { 0.0 });
        if ((((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) && (!s.b[3028])) && s.b[3029]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) && (!s.b[3028])) && (!s.b[3029])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) {s.store_sub_from_scalar(2012, 1.0, 2009);s.store_add_scaled_inputs_product_mixed_iiia(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));}
        s.b[3030] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);s.store_scalar(3030, if s.b[3030] { 1.0 } else { 0.0 });
        if (((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) && s.b[3030]) {s.store_exp_neg_input(2005, 2013);}
        s.b[3031] = ((-s.v[2013]) < 0.0);s.store_scalar(3031, if s.b[3031] { 1.0 } else { 0.0 });
        if ((((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) && (!s.b[3030])) && s.b[3031]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) && (!s.b[3030])) && (!s.b[3031])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_add(2016, 2013, 2014);}
        s.b[3032] = (((s.v[2016]) as f64).abs() <= s.v[1933]);s.store_scalar(3032, if s.b[3032] { 1.0 } else { 0.0 });
        if (s.b[3022] && s.b[3032]) {s.store_mul_ad_affine_product_rhs(1996, 2016, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_102(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[3022] && s.b[3032]) {s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(1991, 1889, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.25), 0.3333333333333333), 1.0, (-0.70710678));s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1990, 1889, 1.0, 2016, 0.5, (-0.235702));}
        s.b[3033] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);s.store_scalar(3033, if s.b[3033] { 1.0 } else { 0.0 });
        if ((s.b[3022] && (!s.b[3032])) && s.b[3033]) {s.store_exp_neg_input(2027, 2016);}
        s.b[3034] = ((-s.v[2016]) < 0.0);s.store_scalar(3034, if s.b[3034] { 1.0 } else { 0.0 });
        if (((s.b[3022] && (!s.b[3032])) && (!s.b[3033])) && s.b[3034]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2016)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[3022] && (!s.b[3032])) && (!s.b[3033])) && (!s.b[3034])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(2016)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (s.b[3022] && (!s.b[3032])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));}
        s.b[3035] = (s.v[2016] > s.v[1933]);s.store_scalar(3035, if s.b[3035] { 1.0 } else { 0.0 });
        if ((s.b[3022] && (!s.b[3032])) && s.b[3035]) {s.store_neg(1996, 1996);}
        if (s.b[3022] && (!s.b[3032])) {s.store_div_scaled_product3_mixed_iiai(1991, 1889, 1889, A::sub_from_scalar(1.0, s.ad_value(2027)), 0.5, 1996, 1.0);s.store_add_div_lhs_mixed_ai(1990, A::add_scaled_square_product(s.ad_value(1991), 1.0, s.ad_value(1889), s.ad_value(1889), (-0.5)), 1996, 1991);}
        if s.b[3022] {s.store_sub(1988, 1937, 1991);s.store_div_from_scalar(1989, 1.0, 1988);s.store_offset_mul(1987, 1975, 1989, (-1.0));s.store_mul_scale_offset_mixed_ia(1986, 1989, A::mul(A::mul3(s.ad_value(1975), s.ad_value(1990), s.ad_value(1989)), s.ad_value(1989)), -1.0, 1.0);s.store_add_scaled_product_mixed_aii(2017, A::mul3(s.ad_value(1986), s.ad_value(1992), s.ad_value(1992)), 1.0, 1987, 1993, 1.0);s.store_mul_ad_product_lhs_mixed_ai(1994, A::square(s.ad_value(1992)), 1989, 1989);s.store_mul(1985, 2018, 1994);}
        s.b[3036] = (s.v[0] == (-1.0));s.store_scalar(3036, if s.b[3036] { 1.0 } else { 0.0 });
        if (s.b[3022] && s.b[3036]) {s.store_div_scaled_value_offset_denominator(1985, s.ad_value(1985), 1.0, A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0, 1.0);}
        if s.b[3022] {s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);s.store_add_scaled_product_indices(2028, 1993, 1.0, 1994, 1990, (-1.0));s.store_mul_sub_mixed_iia(1957, 2019, 2017, A::div_scaled_product(A::mul3(s.ad_value(1985), s.ad_value(1987), s.ad_value(2028)), s.ad_value(2019), 1.0, s.ad_value(2027), 1.0));}
        s.b[3037] = (s.v[1] >= 8.0);s.store_scalar(3037, if s.b[3037] { 1.0 } else { 0.0 });s.b[3038] = (s.v[1] == 9.0);s.store_scalar(3038, if s.b[3038] { 1.0 } else { 0.0 });
        if (s.b[3037] && s.b[3038]) {s.store_sub_scaled_inputs_mixed_ai(1992, A::add(A::add_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1974), 81900.0, s.ad_value(1972), 5880.0, s.ad_value(1971), (-1575.0), s.ad_value(1976), 5850.0), 1.0, s.ad_value(1977), 282255.0), 1.0, s.ad_value(1969), 105.0), 1.0, s.ad_value(1975), 305655.0), 2.6434745829918846e-5, s.ad_value(1970), (420.0 * 2.6434745829918846e-5)), A::sub_scaled_inputs(s.ad_value(1978), (35.0 * 1.3217372914959423e-5), s.ad_value(1979), (94085.0 * 1.3217372914959423e-5))), 1.0, 1973, (105.0 / 181.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_103(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[3037] && s.b[3038]) {s.store_add_scaled_inputs_mixed_ai(1993, A::sub_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1969), 3600.0, s.ad_value(1975), 10479600.0, s.ad_value(1972), (-201600.0), s.ad_value(1971), 54000.0), 1.0, s.ad_value(1970), 14400.0), 1.0, s.ad_value(1978), 600.0), 1.0, s.ad_value(1979), 1629600.0), 1.0, s.ad_value(1976), 16413000.0), 1.0, s.ad_value(1977), 9777600.0), 2.6434745829918846e-5, s.ad_value(1974), (2808000.0 * 2.6434745829918846e-5)), 1.0, 1973, (3600.0 * 0.0055248618784530384));}
        if (s.b[3037] && (!s.b[3038])) {s.store_scalar(1992, 0.0);s.store_scalar(1993, 0.0);}
        if s.b[3037] {s.store_add_div_lhs_indices(2027, 1976, 1937, 1890);}
        s.b[3039] = (((s.v[2027]) as f64).abs() <= s.v[1941]);s.store_scalar(3039, if s.b[3039] { 1.0 } else { 0.0 });
        if (s.b[3037] && s.b[3039]) {s.store_div(2016, 2027, 1940);}
        s.b[3040] = (s.v[2027] < (-s.v[1941]));s.store_scalar(3040, if s.b[3040] { 1.0 } else { 0.0 });
        if ((s.b[3037] && (!s.b[3039])) && s.b[3040]) {s.store_neg(1999, 2027);s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);s.store_add(824, 2002, 2003);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);s.store_add_mixed_ia(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));}
        s.b[3041] = (((s.v[2015]) as f64).abs() < 230.25850929940458);s.store_scalar(3041, if s.b[3041] { 1.0 } else { 0.0 });
        if (((s.b[3037] && (!s.b[3039])) && s.b[3040]) && s.b[3041]) {s.store_exp(2005, 2015);}
        s.b[3042] = (s.v[2015] < 0.0);s.store_scalar(3042, if s.b[3042] { 1.0 } else { 0.0 });
        if ((((s.b[3037] && (!s.b[3039])) && s.b[3040]) && (!s.b[3041])) && s.b[3042]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[3037] && (!s.b[3039])) && s.b[3040]) && (!s.b[3041])) && (!s.b[3042])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[3037] && (!s.b[3039])) && s.b[3040]) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs_product_mixed_iiia(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_104(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[3037] && (!s.b[3039])) && s.b[3040]) {s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_neg_add(2016, 2015, 2012);}
        if ((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) {s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);s.store_mul_scale_offset_mixed_ia(2010, 1998, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), 1.0, (-1.0));s.store_mul_scale_offset(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0, 1.0);}
        s.b[3043] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);s.store_scalar(3043, if s.b[3043] { 1.0 } else { 0.0 });
        if (((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) && s.b[3043]) {s.store_exp_neg_input(2009, 2011);}
        s.b[3044] = ((-s.v[2011]) < 0.0);s.store_scalar(3044, if s.b[3044] { 1.0 } else { 0.0 });
        if ((((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) && (!s.b[3043])) && s.b[3044]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) && (!s.b[3043])) && (!s.b[3044])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) {s.store_sub_from_scalar(2012, 1.0, 2009);s.store_add_scaled_inputs_product_mixed_iiia(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));}
        s.b[3045] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);s.store_scalar(3045, if s.b[3045] { 1.0 } else { 0.0 });
        if (((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) && s.b[3045]) {s.store_exp_neg_input(2005, 2013);}
        s.b[3046] = ((-s.v[2013]) < 0.0);s.store_scalar(3046, if s.b[3046] { 1.0 } else { 0.0 });
        if ((((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) && (!s.b[3045])) && s.b[3046]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) && (!s.b[3045])) && (!s.b[3046])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_add(2016, 2013, 2014);}
        s.b[3047] = (((s.v[2016]) as f64).abs() <= s.v[1933]);s.store_scalar(3047, if s.b[3047] { 1.0 } else { 0.0 });
        if (s.b[3037] && s.b[3047]) {s.store_mul_ad_affine_product_rhs(1996, 2016, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(1991, 1889, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.25), 0.3333333333333333), 1.0, (-0.70710678));s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1990, 1889, 1.0, 2016, 0.5, (-0.235702));}
        s.b[3048] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);s.store_scalar(3048, if s.b[3048] { 1.0 } else { 0.0 });
        if ((s.b[3037] && (!s.b[3047])) && s.b[3048]) {s.store_exp_neg_input(2027, 2016);}
        s.b[3049] = ((-s.v[2016]) < 0.0);s.store_scalar(3049, if s.b[3049] { 1.0 } else { 0.0 });
        if (((s.b[3037] && (!s.b[3047])) && (!s.b[3048])) && s.b[3049]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2016)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[3037] && (!s.b[3047])) && (!s.b[3048])) && (!s.b[3049])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(2016)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_105(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[3037] && (!s.b[3047])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));}
        s.b[3050] = (s.v[2016] > s.v[1933]);s.store_scalar(3050, if s.b[3050] { 1.0 } else { 0.0 });
        if ((s.b[3037] && (!s.b[3047])) && s.b[3050]) {s.store_neg(1996, 1996);}
        if (s.b[3037] && (!s.b[3047])) {s.store_div_scaled_product3_mixed_iiai(1991, 1889, 1889, A::sub_from_scalar(1.0, s.ad_value(2027)), 0.5, 1996, 1.0);s.store_add_div_lhs_mixed_ai(1990, A::add_scaled_square_product(s.ad_value(1991), 1.0, s.ad_value(1889), s.ad_value(1889), (-0.5)), 1996, 1991);}
        if s.b[3037] {s.store_sub(1988, 1937, 1991);s.store_div_from_scalar(1989, 1.0, 1988);s.store_offset_mul(1987, 1976, 1989, (-1.0));s.store_mul_scale_offset_mixed_ia(1986, 1989, A::mul(A::mul3(s.ad_value(1976), s.ad_value(1990), s.ad_value(1989)), s.ad_value(1989)), -1.0, 1.0);s.store_add_scaled_product_mixed_aii(2017, A::mul3(s.ad_value(1986), s.ad_value(1992), s.ad_value(1992)), 1.0, 1987, 1993, 1.0);s.store_mul_ad_product_lhs_mixed_ai(1994, A::square(s.ad_value(1992)), 1989, 1989);s.store_mul(1985, 2018, 1994);}
        s.b[3051] = (s.v[0] == (-1.0));s.store_scalar(3051, if s.b[3051] { 1.0 } else { 0.0 });
        if (s.b[3037] && s.b[3051]) {s.store_div_scaled_value_offset_denominator(1985, s.ad_value(1985), 1.0, A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0, 1.0);}
        if s.b[3037] {s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);s.store_add_scaled_product_indices(2028, 1993, 1.0, 1994, 1990, (-1.0));s.store_mul_sub_mixed_iia(1958, 2019, 2017, A::div_scaled_product(A::mul3(s.ad_value(1985), s.ad_value(1987), s.ad_value(2028)), s.ad_value(2019), 1.0, s.ad_value(2027), 1.0));}
        s.b[3052] = (s.v[1] >= 9.0);s.store_scalar(3052, if s.b[3052] { 1.0 } else { 0.0 });s.b[3053] = (s.v[1] == 9.0);s.store_scalar(3053, if s.b[3053] { 1.0 } else { 0.0 });
        if (s.b[3052] && s.b[3053]) {s.store_add_scaled_inputs_mixed_ai(1992, A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1974), (-23400.0), s.ad_value(1972), (-1680.0), s.ad_value(1979), 175565.0, s.ad_value(1971), 450.0), 1.0, s.ad_value(1976), 325920.0), 1.0, s.ad_value(1978), 5.0), 1.0, s.ad_value(1977), 81480.0), 1.0, s.ad_value(1969), 30.0), 1.0, s.ad_value(1975), 87330.0), 2.6434745829918846e-5, s.ad_value(1970), (120.0 * 2.6434745829918846e-5)), 1.0, 1973, (30.0 * 0.0055248618784530384));s.store_sub_scaled_inputs_mixed_ai(1993, A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1969), (-900.0), s.ad_value(1975), (-2619900.0), s.ad_value(1972), 50400.0, s.ad_value(1971), (-13500.0)), 1.0, s.ad_value(1970), 3600.0), 1.0, s.ad_value(1978), 150.0), 1.0, s.ad_value(1979), 6081750.0), 1.0, s.ad_value(1976), 9777600.0), 1.0, s.ad_value(1977), 13793100.0), 2.6434745829918846e-5, s.ad_value(1974), (702000.0 * 2.6434745829918846e-5)), 1.0, 1973, (900.0 * 0.0055248618784530384));}
        if (s.b[3052] && (!s.b[3053])) {s.store_scalar(1992, 0.0);s.store_scalar(1993, 0.0);}
        if s.b[3052] {s.store_add_div_lhs_indices(2027, 1977, 1937, 1890);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_106(
        s: &mut ReactiveScratch,
    ) {
        s.b[3054] = (((s.v[2027]) as f64).abs() <= s.v[1941]);s.store_scalar(3054, if s.b[3054] { 1.0 } else { 0.0 });
        if (s.b[3052] && s.b[3054]) {s.store_div(2016, 2027, 1940);}
        s.b[3055] = (s.v[2027] < (-s.v[1941]));s.store_scalar(3055, if s.b[3055] { 1.0 } else { 0.0 });
        if ((s.b[3052] && (!s.b[3054])) && s.b[3055]) {s.store_neg(1999, 2027);s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);s.store_add(824, 2002, 2003);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);s.store_add_mixed_ia(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));}
        s.b[3056] = (((s.v[2015]) as f64).abs() < 230.25850929940458);s.store_scalar(3056, if s.b[3056] { 1.0 } else { 0.0 });
        if (((s.b[3052] && (!s.b[3054])) && s.b[3055]) && s.b[3056]) {s.store_exp(2005, 2015);}
        s.b[3057] = (s.v[2015] < 0.0);s.store_scalar(3057, if s.b[3057] { 1.0 } else { 0.0 });
        if ((((s.b[3052] && (!s.b[3054])) && s.b[3055]) && (!s.b[3056])) && s.b[3057]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[3052] && (!s.b[3054])) && s.b[3055]) && (!s.b[3056])) && (!s.b[3057])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[3052] && (!s.b[3054])) && s.b[3055]) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs_product_mixed_iiia(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_neg_add(2016, 2015, 2012);}
        if ((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) {s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);s.store_mul_scale_offset_mixed_ia(2010, 1998, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), 1.0, (-1.0));s.store_mul_scale_offset(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0, 1.0);}
        s.b[3058] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);s.store_scalar(3058, if s.b[3058] { 1.0 } else { 0.0 });
        if (((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) && s.b[3058]) {s.store_exp_neg_input(2009, 2011);}
        s.b[3059] = ((-s.v[2011]) < 0.0);s.store_scalar(3059, if s.b[3059] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_107(
        s: &mut ReactiveScratch,
    ) {
        if ((((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) && (!s.b[3058])) && s.b[3059]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) && (!s.b[3058])) && (!s.b[3059])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) {s.store_sub_from_scalar(2012, 1.0, 2009);s.store_add_scaled_inputs_product_mixed_iiia(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));}
        s.b[3060] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);s.store_scalar(3060, if s.b[3060] { 1.0 } else { 0.0 });
        if (((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) && s.b[3060]) {s.store_exp_neg_input(2005, 2013);}
        s.b[3061] = ((-s.v[2013]) < 0.0);s.store_scalar(3061, if s.b[3061] { 1.0 } else { 0.0 });
        if ((((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) && (!s.b[3060])) && s.b[3061]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) && (!s.b[3060])) && (!s.b[3061])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_add(2016, 2013, 2014);}
        s.b[3062] = (((s.v[2016]) as f64).abs() <= s.v[1933]);s.store_scalar(3062, if s.b[3062] { 1.0 } else { 0.0 });
        if (s.b[3052] && s.b[3062]) {s.store_mul_ad_affine_product_rhs(1996, 2016, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);s.store_mul_sub_from_scalar_scaled_rhs_scaled_output_mixed_ia(1991, 1889, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.25), 0.3333333333333333), 1.0, (-0.70710678));s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1990, 1889, 1.0, 2016, 0.5, (-0.235702));}
        s.b[3063] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);s.store_scalar(3063, if s.b[3063] { 1.0 } else { 0.0 });
        if ((s.b[3052] && (!s.b[3062])) && s.b[3063]) {s.store_exp_neg_input(2027, 2016);}
        s.b[3064] = ((-s.v[2016]) < 0.0);s.store_scalar(3064, if s.b[3064] { 1.0 } else { 0.0 });
        if (((s.b[3052] && (!s.b[3062])) && (!s.b[3063])) && s.b[3064]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2016)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[3052] && (!s.b[3062])) && (!s.b[3063])) && (!s.b[3064])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(2016)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (s.b[3052] && (!s.b[3062])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));}
        s.b[3065] = (s.v[2016] > s.v[1933]);s.store_scalar(3065, if s.b[3065] { 1.0 } else { 0.0 });
        if ((s.b[3052] && (!s.b[3062])) && s.b[3065]) {s.store_neg(1996, 1996);}
        if (s.b[3052] && (!s.b[3062])) {s.store_div_scaled_product3_mixed_iiai(1991, 1889, 1889, A::sub_from_scalar(1.0, s.ad_value(2027)), 0.5, 1996, 1.0);s.store_add_div_lhs_mixed_ai(1990, A::add_scaled_square_product(s.ad_value(1991), 1.0, s.ad_value(1889), s.ad_value(1889), (-0.5)), 1996, 1991);}
        if s.b[3052] {s.store_sub(1988, 1937, 1991);s.store_div_from_scalar(1989, 1.0, 1988);s.store_offset_mul(1987, 1977, 1989, (-1.0));s.store_mul_scale_offset_mixed_ia(1986, 1989, A::mul(A::mul3(s.ad_value(1977), s.ad_value(1990), s.ad_value(1989)), s.ad_value(1989)), -1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_108(
        s: &mut ReactiveScratch,
    ) {
        if s.b[3052] {s.store_add_scaled_product_mixed_aii(2017, A::mul3(s.ad_value(1986), s.ad_value(1992), s.ad_value(1992)), 1.0, 1987, 1993, 1.0);s.store_mul_ad_product_lhs_mixed_ai(1994, A::square(s.ad_value(1992)), 1989, 1989);s.store_mul(1985, 2018, 1994);}
        s.b[3066] = (s.v[0] == (-1.0));s.store_scalar(3066, if s.b[3066] { 1.0 } else { 0.0 });
        if (s.b[3052] && s.b[3066]) {s.store_div_scaled_value_offset_denominator(1985, s.ad_value(1985), 1.0, A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0, 1.0);}
        if s.b[3052] {s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);s.store_add_scaled_product_indices(2028, 1993, 1.0, 1994, 1990, (-1.0));s.store_mul_sub_mixed_iia(1959, 2019, 2017, A::div_scaled_product(A::mul3(s.ad_value(1985), s.ad_value(1987), s.ad_value(2028)), s.ad_value(2019), 1.0, s.ad_value(2027), 1.0));}
        s.store_scalar(1981, 0.0);s.store_scalar(1982, 0.0);s.store_scalar(1980, 0.0);s.b[3067] = (s.v[1] != 0.0);s.store_scalar(3067, if s.b[3067] { 1.0 } else { 0.0 });s.b[3068] = (s.v[1] == 1.0);s.store_scalar(3068, if s.b[3068] { 1.0 } else { 0.0 });
        if (s.b[3067] && s.b[3068]) {s.store_add_scaled_inputs3_indices(1981, 1978, (17.0 * 0.010416666666666666), 1969, (30.0 * 0.010416666666666666), 1979, 0.010416666666666666);s.store_add_scaled_inputs3_indices(1982, 1978, 0.010416666666666666, 1969, (30.0 * 0.010416666666666666), 1979, (17.0 * 0.010416666666666666));s.store_add_div_lhs_indices(2027, 1969, 1937, 1890);}
        s.b[3069] = (((s.v[2027]) as f64).abs() <= s.v[1941]);s.store_scalar(3069, if s.b[3069] { 1.0 } else { 0.0 });
        if ((s.b[3067] && s.b[3068]) && s.b[3069]) {s.store_div(2028, 2027, 1940);}
        s.b[3070] = (s.v[2027] < (-s.v[1941]));s.store_scalar(3070, if s.b[3070] { 1.0 } else { 0.0 });
        if (((s.b[3067] && s.b[3068]) && (!s.b[3069])) && s.b[3070]) {s.store_neg(1999, 2027);s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);s.store_add(824, 2002, 2003);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);s.store_add_mixed_ia(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));}
        s.b[3071] = (((s.v[2015]) as f64).abs() < 230.25850929940458);s.store_scalar(3071, if s.b[3071] { 1.0 } else { 0.0 });
        if ((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && s.b[3070]) && s.b[3071]) {s.store_exp(2005, 2015);}
        s.b[3072] = (s.v[2015] < 0.0);s.store_scalar(3072, if s.b[3072] { 1.0 } else { 0.0 });
        if (((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && s.b[3070]) && (!s.b[3071])) && s.b[3072]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && s.b[3070]) && (!s.b[3071])) && (!s.b[3072])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_109(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[3067] && s.b[3068]) && (!s.b[3069])) && s.b[3070]) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs_product_mixed_iiia(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_neg_add(2028, 2015, 2012);}
        if (((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) {s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);s.store_mul_scale_offset_mixed_ia(2010, 1998, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), 1.0, (-1.0));s.store_mul_scale_offset(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0, 1.0);}
        s.b[3073] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);s.store_scalar(3073, if s.b[3073] { 1.0 } else { 0.0 });
        if ((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) && s.b[3073]) {s.store_exp_neg_input(2009, 2011);}
        s.b[3074] = ((-s.v[2011]) < 0.0);s.store_scalar(3074, if s.b[3074] { 1.0 } else { 0.0 });
        if (((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) && (!s.b[3073])) && s.b[3074]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) && (!s.b[3073])) && (!s.b[3074])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) {s.store_sub_from_scalar(2012, 1.0, 2009);s.store_add_scaled_inputs_product_mixed_iiia(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));}
        s.b[3075] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);s.store_scalar(3075, if s.b[3075] { 1.0 } else { 0.0 });
        if ((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) && s.b[3075]) {s.store_exp_neg_input(2005, 2013);}
        s.b[3076] = ((-s.v[2013]) < 0.0);s.store_scalar(3076, if s.b[3076] { 1.0 } else { 0.0 });
        if (((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) && (!s.b[3075])) && s.b[3076]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) && (!s.b[3075])) && (!s.b[3076])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_add(2028, 2013, 2014);}
        if (s.b[3067] && s.b[3068]) {s.store_add_scaled_inputs4_indices(1980, 1890, 1.0, 1983, (-0.16666666666666666), 2028, (-(4.0 * 0.16666666666666666)), 1984, (-0.16666666666666666));}
        s.b[3077] = (s.v[1] == 2.0);s.store_scalar(3077, if s.b[3077] { 1.0 } else { 0.0 });
        if ((s.b[3067] && (!s.b[3068])) && s.b[3077]) {s.store_add_scaled_inputs4_indices(1981, 1978, (11.0 * 0.011111111111111112), 1969, (24.0 * 0.011111111111111112), 1970, (9.0 * 0.011111111111111112), 1979, 0.011111111111111112);s.store_add_scaled_inputs4_indices(1982, 1979, (11.0 * 0.011111111111111112), 1970, (24.0 * 0.011111111111111112), 1969, (9.0 * 0.011111111111111112), 1978, 0.011111111111111112);s.store_add_div_lhs_indices(2027, 1969, 1937, 1890);}
        s.b[3078] = (((s.v[2027]) as f64).abs() <= s.v[1941]);s.store_scalar(3078, if s.b[3078] { 1.0 } else { 0.0 });
        if (((s.b[3067] && (!s.b[3068])) && s.b[3077]) && s.b[3078]) {s.store_div(2028, 2027, 1940);}
        s.b[3079] = (s.v[2027] < (-s.v[1941]));s.store_scalar(3079, if s.b[3079] { 1.0 } else { 0.0 });
        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && s.b[3079]) {s.store_neg(1999, 2027);s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);}
    }
}

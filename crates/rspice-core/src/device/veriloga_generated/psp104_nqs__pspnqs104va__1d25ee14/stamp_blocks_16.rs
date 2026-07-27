#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_256(
        s: &mut Scratch,
    ) {
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && s.b[3186]) && (!s.b[3187])) && (!s.b[3188])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && s.b[3186]) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs_product_mixed_iiia(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_neg_add(2021, 2015, 2012);}
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) {s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);s.store_mul_scale_offset_mixed_ia(2010, 1998, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), 1.0, (-1.0));s.store_mul_scale_offset(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0, 1.0);}
        s.b[3189] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);s.store_scalar(3189, if s.b[3189] { 1.0 } else { 0.0 });
        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) && s.b[3189]) {s.store_exp_neg_input(2009, 2011);}
        s.b[3190] = ((-s.v[2011]) < 0.0);s.store_scalar(3190, if s.b[3190] { 1.0 } else { 0.0 });
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) && (!s.b[3189])) && s.b[3190]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) && (!s.b[3189])) && (!s.b[3190])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) {s.store_sub_from_scalar(2012, 1.0, 2009);s.store_add_scaled_inputs_product_mixed_iiia(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));}
        s.b[3191] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);s.store_scalar(3191, if s.b[3191] { 1.0 } else { 0.0 });
        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) && s.b[3191]) {s.store_exp_neg_input(2005, 2013);}
        s.b[3192] = ((-s.v[2013]) < 0.0);s.store_scalar(3192, if s.b[3192] { 1.0 } else { 0.0 });
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) && (!s.b[3191])) && s.b[3192]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) && (!s.b[3191])) && (!s.b[3192])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_add(2021, 2013, 2014);}
        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {s.store_add_div_lhs_indices(2027, 1973, 1937, 1890);}
        s.b[3193] = (((s.v[2027]) as f64).abs() <= s.v[1941]);s.store_scalar(3193, if s.b[3193] { 1.0 } else { 0.0 });
        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && s.b[3193]) {s.store_div(2022, 2027, 1940);}
        s.b[3194] = (s.v[2027] < (-s.v[1941]));s.store_scalar(3194, if s.b[3194] { 1.0 } else { 0.0 });
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && s.b[3194]) {s.store_neg(1999, 2027);s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_257(
        s: &mut Scratch,
    ) {
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && s.b[3194]) {s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);s.store_add(824, 2002, 2003);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);s.store_add_mixed_ia(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));}
        s.b[3195] = (((s.v[2015]) as f64).abs() < 230.25850929940458);s.store_scalar(3195, if s.b[3195] { 1.0 } else { 0.0 });
        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && s.b[3194]) && s.b[3195]) {s.store_exp(2005, 2015);}
        s.b[3196] = (s.v[2015] < 0.0);s.store_scalar(3196, if s.b[3196] { 1.0 } else { 0.0 });
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && s.b[3194]) && (!s.b[3195])) && s.b[3196]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && s.b[3194]) && (!s.b[3195])) && (!s.b[3196])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && s.b[3194]) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs_product_mixed_iiia(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_neg_add(2022, 2015, 2012);}
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && (!s.b[3194])) {s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);s.store_mul_scale_offset_mixed_ia(2010, 1998, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), 1.0, (-1.0));s.store_mul_scale_offset(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0, 1.0);}
        s.b[3197] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);s.store_scalar(3197, if s.b[3197] { 1.0 } else { 0.0 });
        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && (!s.b[3194])) && s.b[3197]) {s.store_exp_neg_input(2009, 2011);}
        s.b[3198] = ((-s.v[2011]) < 0.0);s.store_scalar(3198, if s.b[3198] { 1.0 } else { 0.0 });
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && (!s.b[3194])) && (!s.b[3197])) && s.b[3198]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && (!s.b[3194])) && (!s.b[3197])) && (!s.b[3198])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && (!s.b[3194])) {s.store_sub_from_scalar(2012, 1.0, 2009);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_258(
        s: &mut Scratch,
    ) {
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && (!s.b[3194])) {s.store_add_scaled_inputs_product_mixed_iiia(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));}
        s.b[3199] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);s.store_scalar(3199, if s.b[3199] { 1.0 } else { 0.0 });
        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && (!s.b[3194])) && s.b[3199]) {s.store_exp_neg_input(2005, 2013);}
        s.b[3200] = ((-s.v[2013]) < 0.0);s.store_scalar(3200, if s.b[3200] { 1.0 } else { 0.0 });
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && (!s.b[3194])) && (!s.b[3199])) && s.b[3200]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && (!s.b[3194])) && (!s.b[3199])) && (!s.b[3200])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && (!s.b[3194])) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_add(2022, 2013, 2014);}
        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {s.store_add_div_lhs_indices(2027, 1974, 1937, 1890);}
        s.b[3201] = (((s.v[2027]) as f64).abs() <= s.v[1941]);s.store_scalar(3201, if s.b[3201] { 1.0 } else { 0.0 });
        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && s.b[3201]) {s.store_div(2023, 2027, 1940);}
        s.b[3202] = (s.v[2027] < (-s.v[1941]));s.store_scalar(3202, if s.b[3202] { 1.0 } else { 0.0 });
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && s.b[3202]) {s.store_neg(1999, 2027);s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);s.store_add(824, 2002, 2003);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);s.store_add_mixed_ia(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));}
        s.b[3203] = (((s.v[2015]) as f64).abs() < 230.25850929940458);s.store_scalar(3203, if s.b[3203] { 1.0 } else { 0.0 });
        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && s.b[3202]) && s.b[3203]) {s.store_exp(2005, 2015);}
        s.b[3204] = (s.v[2015] < 0.0);s.store_scalar(3204, if s.b[3204] { 1.0 } else { 0.0 });
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && s.b[3202]) && (!s.b[3203])) && s.b[3204]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_259(
        s: &mut Scratch,
    ) {
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && s.b[3202]) && (!s.b[3203])) && (!s.b[3204])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && s.b[3202]) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs_product_mixed_iiia(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_neg_add(2023, 2015, 2012);}
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && (!s.b[3202])) {s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);s.store_mul_scale_offset_mixed_ia(2010, 1998, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), 1.0, (-1.0));s.store_mul_scale_offset(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0, 1.0);}
        s.b[3205] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);s.store_scalar(3205, if s.b[3205] { 1.0 } else { 0.0 });
        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && (!s.b[3202])) && s.b[3205]) {s.store_exp_neg_input(2009, 2011);}
        s.b[3206] = ((-s.v[2011]) < 0.0);s.store_scalar(3206, if s.b[3206] { 1.0 } else { 0.0 });
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && (!s.b[3202])) && (!s.b[3205])) && s.b[3206]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && (!s.b[3202])) && (!s.b[3205])) && (!s.b[3206])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && (!s.b[3202])) {s.store_sub_from_scalar(2012, 1.0, 2009);s.store_add_scaled_inputs_product_mixed_iiia(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));}
        s.b[3207] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);s.store_scalar(3207, if s.b[3207] { 1.0 } else { 0.0 });
        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && (!s.b[3202])) && s.b[3207]) {s.store_exp_neg_input(2005, 2013);}
        s.b[3208] = ((-s.v[2013]) < 0.0);s.store_scalar(3208, if s.b[3208] { 1.0 } else { 0.0 });
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && (!s.b[3202])) && (!s.b[3207])) && s.b[3208]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && (!s.b[3202])) && (!s.b[3207])) && (!s.b[3208])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3201])) && (!s.b[3202])) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_add(2023, 2013, 2014);}
        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {s.store_add_div_lhs_indices(2027, 1975, 1937, 1890);}
        s.b[3209] = (((s.v[2027]) as f64).abs() <= s.v[1941]);s.store_scalar(3209, if s.b[3209] { 1.0 } else { 0.0 });
        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && s.b[3209]) {s.store_div(2024, 2027, 1940);}
        s.b[3210] = (s.v[2027] < (-s.v[1941]));s.store_scalar(3210, if s.b[3210] { 1.0 } else { 0.0 });
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && s.b[3210]) {s.store_neg(1999, 2027);s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_260(
        s: &mut Scratch,
    ) {
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && s.b[3210]) {s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);s.store_add(824, 2002, 2003);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);s.store_add_mixed_ia(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));}
        s.b[3211] = (((s.v[2015]) as f64).abs() < 230.25850929940458);s.store_scalar(3211, if s.b[3211] { 1.0 } else { 0.0 });
        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && s.b[3210]) && s.b[3211]) {s.store_exp(2005, 2015);}
        s.b[3212] = (s.v[2015] < 0.0);s.store_scalar(3212, if s.b[3212] { 1.0 } else { 0.0 });
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && s.b[3210]) && (!s.b[3211])) && s.b[3212]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && s.b[3210]) && (!s.b[3211])) && (!s.b[3212])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && s.b[3210]) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs_product_mixed_iiia(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_neg_add(2024, 2015, 2012);}
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && (!s.b[3210])) {s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);s.store_mul_scale_offset_mixed_ia(2010, 1998, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), 1.0, (-1.0));s.store_mul_scale_offset(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0, 1.0);}
        s.b[3213] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);s.store_scalar(3213, if s.b[3213] { 1.0 } else { 0.0 });
        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && (!s.b[3210])) && s.b[3213]) {s.store_exp_neg_input(2009, 2011);}
        s.b[3214] = ((-s.v[2011]) < 0.0);s.store_scalar(3214, if s.b[3214] { 1.0 } else { 0.0 });
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && (!s.b[3210])) && (!s.b[3213])) && s.b[3214]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && (!s.b[3210])) && (!s.b[3213])) && (!s.b[3214])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && (!s.b[3210])) {s.store_sub_from_scalar(2012, 1.0, 2009);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_261(
        s: &mut Scratch,
    ) {
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && (!s.b[3210])) {s.store_add_scaled_inputs_product_mixed_iiia(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));}
        s.b[3215] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);s.store_scalar(3215, if s.b[3215] { 1.0 } else { 0.0 });
        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && (!s.b[3210])) && s.b[3215]) {s.store_exp_neg_input(2005, 2013);}
        s.b[3216] = ((-s.v[2013]) < 0.0);s.store_scalar(3216, if s.b[3216] { 1.0 } else { 0.0 });
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && (!s.b[3210])) && (!s.b[3215])) && s.b[3216]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && (!s.b[3210])) && (!s.b[3215])) && (!s.b[3216])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3209])) && (!s.b[3210])) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_add(2024, 2013, 2014);}
        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {s.store_add_div_lhs_indices(2027, 1976, 1937, 1890);}
        s.b[3217] = (((s.v[2027]) as f64).abs() <= s.v[1941]);s.store_scalar(3217, if s.b[3217] { 1.0 } else { 0.0 });
        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && s.b[3217]) {s.store_div(2025, 2027, 1940);}
        s.b[3218] = (s.v[2027] < (-s.v[1941]));s.store_scalar(3218, if s.b[3218] { 1.0 } else { 0.0 });
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && s.b[3218]) {s.store_neg(1999, 2027);s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);s.store_add(824, 2002, 2003);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);s.store_add_mixed_ia(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));}
        s.b[3219] = (((s.v[2015]) as f64).abs() < 230.25850929940458);s.store_scalar(3219, if s.b[3219] { 1.0 } else { 0.0 });
        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && s.b[3218]) && s.b[3219]) {s.store_exp(2005, 2015);}
        s.b[3220] = (s.v[2015] < 0.0);s.store_scalar(3220, if s.b[3220] { 1.0 } else { 0.0 });
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && s.b[3218]) && (!s.b[3219])) && s.b[3220]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_262(
        s: &mut Scratch,
    ) {
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && s.b[3218]) && (!s.b[3219])) && (!s.b[3220])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && s.b[3218]) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs_product_mixed_iiia(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_neg_add(2025, 2015, 2012);}
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && (!s.b[3218])) {s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);s.store_mul_scale_offset_mixed_ia(2010, 1998, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), 1.0, (-1.0));s.store_mul_scale_offset(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0, 1.0);}
        s.b[3221] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);s.store_scalar(3221, if s.b[3221] { 1.0 } else { 0.0 });
        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && (!s.b[3218])) && s.b[3221]) {s.store_exp_neg_input(2009, 2011);}
        s.b[3222] = ((-s.v[2011]) < 0.0);s.store_scalar(3222, if s.b[3222] { 1.0 } else { 0.0 });
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && (!s.b[3218])) && (!s.b[3221])) && s.b[3222]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && (!s.b[3218])) && (!s.b[3221])) && (!s.b[3222])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && (!s.b[3218])) {s.store_sub_from_scalar(2012, 1.0, 2009);s.store_add_scaled_inputs_product_mixed_iiia(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));}
        s.b[3223] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);s.store_scalar(3223, if s.b[3223] { 1.0 } else { 0.0 });
        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && (!s.b[3218])) && s.b[3223]) {s.store_exp_neg_input(2005, 2013);}
        s.b[3224] = ((-s.v[2013]) < 0.0);s.store_scalar(3224, if s.b[3224] { 1.0 } else { 0.0 });
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && (!s.b[3218])) && (!s.b[3223])) && s.b[3224]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && (!s.b[3218])) && (!s.b[3223])) && (!s.b[3224])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3217])) && (!s.b[3218])) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_add(2025, 2013, 2014);}
        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {s.store_add_div_lhs_indices(2027, 1977, 1937, 1890);}
        s.b[3225] = (((s.v[2027]) as f64).abs() <= s.v[1941]);s.store_scalar(3225, if s.b[3225] { 1.0 } else { 0.0 });
        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && s.b[3225]) {s.store_div(2026, 2027, 1940);}
        s.b[3226] = (s.v[2027] < (-s.v[1941]));s.store_scalar(3226, if s.b[3226] { 1.0 } else { 0.0 });
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && s.b[3226]) {s.store_neg(1999, 2027);s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_263(
        s: &mut Scratch,
    ) {
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && s.b[3226]) {s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_square_product_mixed_aia(2002, A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);s.store_add(824, 2002, 2003);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);s.store_add_mixed_ia(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));}
        s.b[3227] = (((s.v[2015]) as f64).abs() < 230.25850929940458);s.store_scalar(3227, if s.b[3227] { 1.0 } else { 0.0 });
        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && s.b[3226]) && s.b[3227]) {s.store_exp(2005, 2015);}
        s.b[3228] = (s.v[2015] < 0.0);s.store_scalar(3228, if s.b[3228] { 1.0 } else { 0.0 });
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && s.b[3226]) && (!s.b[3227])) && s.b[3228]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2005, 1e-100, (-230.25850929940458), 2015, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && s.b[3226]) && (!s.b[3227])) && (!s.b[3228])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2005, 2015, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && s.b[3226]) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs_product_mixed_iiia(2007, 1999, 2.0, 2015, (-2.0), 1939, A::offset(s.ad_value(2005), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, 1939, A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2012, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_neg_add(2026, 2015, 2012);}
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && (!s.b[3226])) {s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);s.store_mul_scale_offset_mixed_ia(2010, 1998, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), 1.0, (-1.0));s.store_mul_scale_offset(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0, 1.0);}
        s.b[3229] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);s.store_scalar(3229, if s.b[3229] { 1.0 } else { 0.0 });
        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && (!s.b[3226])) && s.b[3229]) {s.store_exp_neg_input(2009, 2011);}
        s.b[3230] = ((-s.v[2011]) < 0.0);s.store_scalar(3230, if s.b[3230] { 1.0 } else { 0.0 });
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && (!s.b[3226])) && (!s.b[3229])) && s.b[3230]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2009, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && (!s.b[3226])) && (!s.b[3229])) && (!s.b[3230])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2009, A::neg(s.ad_value(2011)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && (!s.b[3226])) {s.store_sub_from_scalar(2012, 1.0, 2009);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_264(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && (!s.b[3226])) {s.store_add_scaled_inputs_product_mixed_iiia(2013, 2027, 1.0, 1939, 0.5, 1938, A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0));}
        s.b[3231] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);s.store_scalar(3231, if s.b[3231] { 1.0 } else { 0.0 });
        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && (!s.b[3226])) && s.b[3231]) {s.store_exp_neg_input(2005, 2013);}
        s.b[3232] = ((-s.v[2013]) < 0.0);s.store_scalar(3232, if s.b[3232] { 1.0 } else { 0.0 });
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && (!s.b[3226])) && (!s.b[3231])) && s.b[3232]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2005, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && (!s.b[3226])) && (!s.b[3231])) && (!s.b[3232])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2005, A::neg(s.ad_value(2013)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3225])) && (!s.b[3226])) {s.store_sub_from_scalar_scaled_mul(2006, 1.0, 1939, 2005, 0.5);s.store_add_scaled_inputs3_mixed_iia(2007, 2027, 2.0, 2013, (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0);s.store_add_scaled_square_product_mixed_aia(2008, A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, 1939, A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0));s.store_add_scaled_square_product_indices(2009, 2007, 1.0, 2006, 2008, (-4.0));s.store_div_scaled_inputs_mixed_ia(2014, 2008, 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0);s.store_add(2026, 2013, 2014);}
        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {s.store_sub_mixed_ia(1980, 1890, A::add_scaled_inputs(A::add(A::add(s.ad_value(1983), A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(2028), 1.0, s.ad_value(2020), 1.0, s.ad_value(2022), 1.0, s.ad_value(2024), 1.0), 4.0, s.ad_value(2026), 4.0)), A::add_scaled_inputs4(s.ad_value(2029), 2.0, s.ad_value(2021), 2.0, s.ad_value(2023), 2.0, s.ad_value(2025), 2.0)), 0.03333333333333333, s.ad_value(1984), 0.03333333333333333));}
        if s.b[3067] {s.store_mul(1980, 1937, 1980);}
        s.b[3233] = (s.v[831] > 0.0);s.store_scalar(3233, if s.b[3233] { 1.0 } else { 0.0 });
        if (s.b[3067] && s.b[3233]) {s.store_mul3_lhs(850, 1904, 1888, 1981);s.store_mul3_lhs(853, 1904, 1888, 1982);}
        if (s.b[3067] && (!s.b[3233])) {s.store_mul3_lhs(850, 1904, 1888, 1982);s.store_mul3_lhs(853, 1904, 1888, 1981);}
        if s.b[3067] {s.store_mul3_lhs(851, 1904, 1888, 1980);s.store_add_scaled_inputs3_indices(852, 851, -1.0, 850, (-1.0), 853, -1.0);}
        s.store_add_scaled_inputs3_indices(850, 851, (-1.0), 852, (-1.0), 853, (-1.0));s.store_add(854, 854, 1910);s.store_add(855, 855, 1911);s.store_add_scaled_products3_indices(857, 646, 1918, 1.0, 647, 1919, 1.0, 648, 1920, 1.0);s.store_add_scaled_products3_indices(858, 673, 1921, 1.0, 674, 1922, 1.0, 675, 1923, 1.0);s.b[3235] = (s.v[831] < 0.0);s.store_scalar(3235, if s.b[3235] { 1.0 } else { 0.0 });
        if s.b[3235] {s.copy_ad(3234, 853);s.copy_ad(853, 850);s.copy_ad(850, 3234);}
        s.store_scalar(3252, 0.0);s.store_scalar(3247, 0.0);s.store_scalar(859, 1e-40);s.store_scalar(861, 0.0);s.store_scalar(863, 0.0);s.store_mul(860, 1904, 1895);s.store_scalar(862, 0.0);s.store_scalar(3254, 0.0);s.b[3268] = ((s.v[1829] > 0.0) && (s.v[716] > 0.0));s.store_scalar(3268, if s.b[3268] { 1.0 } else { 0.0 });s.b[3270] = (p[32] > 0.0);s.store_scalar(3270, if s.b[3270] { 1.0 } else { 0.0 });
        if (s.b[3268] && s.b[3270]) {s.store_div(3239, 1866, 1864);s.store_div(3240, 1865, 1866);s.store_scaled_div(3241, 1860, 3239, (0.5 * 0.16666666666666666));s.store_square(3242, 3241);s.store_offset_div(3243, 3239, 1877, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_265(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[3268] && s.b[3270]) {
            if ((1.0 - (12.0 * (s.v[3243] * s.v[3242]))) > 1e-20) {
                s.store_sub_from_scalar_scaled_mul(3244, 1.0, 3243, 3242, 12.0);
            } else {
                s.store_scalar(3244, 1e-20);
            }
        }
        if (s.b[3268] && s.b[3270]) {s.store_div_from_scalar_square_ad(3245, 1.0, s.ad_value(3244));s.store_mul3_lhs(3246, 716, 1866, 1876);s.store_add_scaled_inputs3_mixed_iia(3247, 3240, 1.0, 3242, 12.0, A::mul3_scaled_output(A::offset(s.ad_value(3240), 1.0), s.ad_value(3242), s.ad_value(3243), 24.0), -1.0);}
        if (s.b[3268] && s.b[3270]) {
            if (s.v[3247] > 1e-40) {
            } else {
                s.store_scalar(3247, 1e-40);
            }
        }
        if (s.b[3268] && s.b[3270]) {s.store_mul3_lhs(3247, 3246, 3245, 3247);}
        s.b[3271] = (s.v[277] > 0.0);s.store_scalar(3271, if s.b[3271] { 1.0 } else { 0.0 });
        if ((s.b[3268] && s.b[3270]) && s.b[3271]) {s.store_div(3248, 1870, 1869);s.store_mul_ad_product_lhs_mixed_ai(3249, A::square(s.ad_value(3248)), 1860, 1860);}
        s.b[3272] = (s.v[0] == (-1.0));s.store_scalar(3272, if s.b[3272] { 1.0 } else { 0.0 });
        if (((s.b[3268] && s.b[3270]) && s.b[3271]) && s.b[3272]) {s.store_div_scaled_value_offset_denominator(3249, s.ad_value(3249), 1.0, A::mul(s.ad_value(3248), s.ad_value(1860)), 1.0, 1.0);}
        if ((s.b[3268] && s.b[3270]) && s.b[3271]) {s.store_mul_scale_offset_mixed_ia(3250, 1869, A::sqrt(A::scale_offset(s.ad_value(3249), 2.0, 1.0)), 0.5, (1.0) * (0.5));s.store_div_scaled_value_by_product_indices(3251, 1869, 1.0, 3250, 3244, 1.0);s.store_mul_ad_product_lhs_mixed_ai(3252, A::mul3(s.ad_value(810), s.ad_value(838), s.ad_value(1857)), 3251, 3251);s.store_add_scaled_inputs(3247, 3247, 1.0, 3252, 1.0 / (s.v[718]));}
        if (s.b[3268] && s.b[3270]) {s.store_sqrt_mul(862, 719, 3247);}
        s.b[3273] = ((((p[50] == 1.0) && (s.v[719] > 0.0)) && (p[32] > 0.0)) && (p[33] > 0.0));s.store_scalar(3273, if s.b[3273] { 1.0 } else { 0.0 });
        if (s.b[3268] && s.b[3273]) {s.store_sub_ad(859, A::add_scaled_product(s.ad_value(3240), 0.08333333333333333, s.ad_value(3242), A::sub_scaled_inputs(A::offset(s.ad_value(3240), 0.2), 1.0, s.ad_value(3242), 12.0), (-1.0)), A::mul3_scaled_output(s.ad_value(3242), A::sub_scaled_inputs(A::offset(s.ad_value(3240), 1.0), 1.0, s.ad_value(3242), 12.0), s.ad_value(3243), 1.6));}
        if (s.b[3268] && s.b[3273]) {
            if (s.v[859] > 1e-40) {
            } else {
                s.store_scalar(859, 1e-40);
            }
        }
        if (s.b[3268] && s.b[3273]) {s.store_mul_div_lhs(859, 3245, 3246, 859);s.store_mul_ad_product_rhs_mixed_ia(3253, 3245, 3241, A::add_scaled_sub_value_product(1.0, A::scale(s.ad_value(3242), 12.0), 1.0, A::add_scaled_inputs_product(s.ad_value(3240), 1.0, s.ad_value(3242), 19.2, s.ad_value(3240), s.ad_value(3242), (-12.0)), s.ad_value(3243), (-1.0)));s.store_div_scaled_product3_mixed_aiia(860, A::square(s.ad_value(1908)), 1904, 1895, 1.0, A::square(s.ad_value(1906)), 1.0);}
        s.b[3274] = (s.v[277] > 0.0);s.store_scalar(3274, if s.b[3274] { 1.0 } else { 0.0 });
        if ((s.b[3268] && s.b[3273]) && s.b[3274]) {s.store_add_mixed_ia(859, 859, A::div_scaled_product_by_product(s.ad_value(3252), A::scale_offset(s.ad_value(3242), 12.0, 1.0), 1.0, s.ad_value(3246), s.ad_value(3246), (12.0 * s.v[718])));s.store_sub_mixed_ia(3253, 3253, A::div_scaled_product3(s.ad_value(3252), s.ad_value(3241), A::offset(s.ad_value(3243), 1.0), 1.0, s.ad_value(3246), s.v[718]));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_266(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[3268] && s.b[3273]) {s.store_sqrt_div(3254, 719, 859);}
        s.b[3275] = (s.v[862] <= 0.0);s.store_scalar(3275, if s.b[3275] { 1.0 } else { 0.0 });
        if ((s.b[3268] && s.b[3273]) && s.b[3275]) {s.store_scalar(863, 0.0);}
        if ((s.b[3268] && s.b[3273]) && (!s.b[3275])) {s.store_div_scaled_product_indices(863, 3253, 3254, 1.0, 862, 1.0);}
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
        if (s.b[3268] && s.b[3273]) {s.store_div_scaled_product_indices(861, 863, 862, 1.0, 3254, 1.0);}
        s.b[3277] = (((p[46] != 0.0) && (s.v[287] > 0.0)) && (s.v[1880] > 0.0));s.store_scalar(3277, if s.b[3277] { 1.0 } else { 0.0 });
        if s.b[3277] {s.store_div_scaled_inputs_indices(2028, 1883, 4.0, 724, 1.0);s.store_scale(2028, 771, s.v[715]);s.store_mul(2028, 1864, 1877);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_0(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[990] = (p[37] >= 0.0);s.store_scalar(990, if s.b[990] { 1.0 } else { 0.0 });
        if s.b[990] {s.store_scalar(0, 1.0);}
        if (!s.b[990]) {s.store_scalar(0, (-1.0));}
        s.store_scalar(767, (8.8541878176e-12 * 11.8));s.b[991] = (p[51] < 0.5);s.store_scalar(991, if s.b[991] { 1.0 } else { 0.0 });
        if s.b[991] {s.store_scalar(1, 0.0);}
        s.b[992] = (p[51] < 1.5);s.store_scalar(992, if s.b[992] { 1.0 } else { 0.0 });
        if ((!s.b[991]) && s.b[992]) {s.store_scalar(1, 1.0);}
        s.b[993] = (p[51] < 2.5);s.store_scalar(993, if s.b[993] { 1.0 } else { 0.0 });
        if (((!s.b[991]) && (!s.b[992])) && s.b[993]) {s.store_scalar(1, 2.0);}
        s.b[994] = (p[51] < 4.0);s.store_scalar(994, if s.b[994] { 1.0 } else { 0.0 });
        if ((((!s.b[991]) && (!s.b[992])) && (!s.b[993])) && s.b[994]) {s.store_scalar(1, 3.0);}
        s.b[995] = (p[51] < 7.0);s.store_scalar(995, if s.b[995] { 1.0 } else { 0.0 });
        if (((((!s.b[991]) && (!s.b[992])) && (!s.b[993])) && (!s.b[994])) && s.b[995]) {s.store_scalar(1, 5.0);}
        if (((((!s.b[991]) && (!s.b[992])) && (!s.b[993])) && (!s.b[994])) && (!s.b[995])) {s.store_scalar(1, 9.0);}
        s.store_scalar(3, 10.0);s.store_scalar(4, (1.0 / s.v[3]));s.store_scalar(350, (273.15 + p[38]));s.store_scalar(474, 0.0);s.b[996] = (p[927] > 0.5);s.store_scalar(996, if s.b[996] { 1.0 } else { 0.0 });
        if s.b[996] {s.store_scalar(474, 1.0);}
        if (!s.b[996]) {s.store_scalar(474, 0.0);}
        s.store_scalar(364, (273.15 + p[823]));s.store_scalar(367, (1.3806505e-23 / 1.6021918e-19));s.store_scalar(368, (s.v[367] * s.v[364]));s.store_scalar(369, (1.0 / s.v[368]));s.store_scalar(375, ((-((0.000702 * s.v[364]) * s.v[364])) / (1108.0 + s.v[364])));s.store_scalar(378, (p[834] + s.v[375]));s.store_scalar(379, (p[835] + s.v[375]));s.store_scalar(380, (p[836] + s.v[375]));s.store_scalar(408, (1.0 - p[831]));s.store_scalar(409, (1.0 - p[832]));s.store_scalar(410, (1.0 - p[833]));s.store_scalar(411, (1.0 / s.v[408]));s.store_scalar(412, (1.0 / s.v[409]));s.store_scalar(413, (1.0 / s.v[410]));s.store_scalar(423, (s.v[767] / p[825]));s.store_scalar(424, ((p[843] * s.v[767]) / p[826]));s.store_scalar(425, ((p[844] * s.v[767]) / p[827]));s.store_scalar(426, (1.0 / s.v[423]));s.store_scalar(427, (1.0 / s.v[424]));s.store_scalar(428, (1.0 / s.v[425]));s.store_scalar(429, (1.0 / p[828]));s.store_scalar(430, (1.0 / p[829]));s.store_scalar(431, (1.0 / p[830]));s.store_scalar(444, (1.0 - (1.0 / p[824])));s.store_scalar(448, (1.0 / p[860]));s.store_scalar(449, (1.0 / p[861]));s.store_scalar(450, (1.0 / p[862]));s.b[997] = ((((p[866] != 1.0) || (p[867] != 1.0)) || (p[868] != 1.0)) || (p[869] != 1.0));s.store_scalar(997, if s.b[997] { 1.0 } else { 0.0 });
        if s.b[997] {s.store_scalar(473, 1.0);}
        if (!s.b[997]) {s.store_scalar(473, 0.0);}
        s.b[998] = (s.v[473] == 1.0);s.store_scalar(998, if s.b[998] { 1.0 } else { 0.0 });
        if s.b[998] {s.store_scalar(457, (if ((p[827] * p[866]) > 1e-18) { (p[827] * p[866]) } else { 1e-18 }));}
        if s.b[998] {s.store_scalar(458, (if ((p[830] * p[867]) > 0.05) { (p[830] * p[867]) } else { 0.05 }));}
        if s.b[998] {s.store_scalar(459, (if ((if ((p[833] * p[868]) > 0.05) { (p[833] * p[868]) } else { 0.05 }) < 0.95) { (if ((p[833] * p[868]) > 0.05) { (p[833] * p[868]) } else { 0.05 }) } else { 0.95 }));}
        if s.b[998] {s.store_scalar(460, (p[836] * p[869]));s.store_primal_offset(462, 460, s.v[375]);s.store_primal_sub_from_scalar(467, 1.0, 459);s.store_primal_div_from_scalar(468, 1.0, 467);}
        s.b[999] = (p[44] == 0.0);s.store_scalar(999, if s.b[999] { 1.0 } else { 0.0 });
        if s.b[999] {s.store_scalar(505, p[825]);s.store_scalar(506, p[826]);s.store_scalar(507, p[827]);s.store_scalar(508, p[828]);s.store_scalar(509, p[829]);s.store_scalar(510, p[830]);s.store_scalar(511, p[831]);s.store_scalar(512, p[832]);s.store_scalar(513, p[833]);s.store_scalar(514, p[834]);s.store_scalar(515, p[835]);s.store_scalar(516, p[836]);s.store_scalar(517, p[837]);s.store_scalar(518, p[838]);s.store_scalar(519, p[839]);s.store_scalar(522, p[840]);s.store_scalar(523, p[841]);s.store_scalar(524, p[842]);s.store_scalar(520, p[843]);s.store_scalar(521, p[844]);s.store_scalar(525, p[845]);s.store_scalar(526, p[846]);s.store_scalar(527, p[847]);s.store_scalar(528, p[848]);s.store_scalar(529, p[849]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[999] {s.store_scalar(530, p[850]);s.store_scalar(531, p[851]);s.store_scalar(532, p[852]);s.store_scalar(533, p[853]);s.store_scalar(534, p[854]);s.store_scalar(535, p[855]);s.store_scalar(536, p[856]);s.store_scalar(537, p[857]);s.store_scalar(538, p[858]);s.store_scalar(539, p[859]);s.store_scalar(540, p[860]);s.store_scalar(541, p[861]);s.store_scalar(542, p[862]);s.store_scalar(543, p[863]);s.store_scalar(544, p[864]);s.store_scalar(545, p[865]);s.store_scalar(553, p[929]);s.store_scalar(636, p[872]);s.store_scalar(637, p[873]);s.store_scalar(638, p[874]);s.store_scalar(639, p[875]);s.store_scalar(546, p[866]);s.store_scalar(547, p[867]);s.store_scalar(548, p[868]);s.store_scalar(549, p[869]);s.store_scalar(550, p[870]);s.store_scalar(551, p[871]);}
        if (!s.b[999]) {s.store_scalar(505, p[876]);s.store_scalar(506, p[877]);s.store_scalar(507, p[878]);s.store_scalar(508, p[879]);s.store_scalar(509, p[880]);s.store_scalar(510, p[881]);s.store_scalar(511, p[882]);s.store_scalar(512, p[883]);s.store_scalar(513, p[884]);s.store_scalar(514, p[885]);s.store_scalar(515, p[886]);s.store_scalar(516, p[887]);s.store_scalar(517, p[888]);s.store_scalar(518, p[889]);s.store_scalar(519, p[890]);s.store_scalar(522, p[891]);s.store_scalar(523, p[892]);s.store_scalar(524, p[893]);s.store_scalar(520, p[894]);s.store_scalar(521, p[895]);s.store_scalar(525, p[896]);s.store_scalar(526, p[897]);s.store_scalar(527, p[898]);s.store_scalar(528, p[899]);s.store_scalar(529, p[900]);s.store_scalar(530, p[901]);s.store_scalar(531, p[902]);s.store_scalar(532, p[903]);s.store_scalar(533, p[904]);s.store_scalar(534, p[905]);s.store_scalar(535, p[906]);s.store_scalar(536, p[907]);s.store_scalar(537, p[908]);s.store_scalar(538, p[909]);s.store_scalar(539, p[910]);s.store_scalar(540, p[911]);s.store_scalar(541, p[912]);s.store_scalar(542, p[913]);s.store_scalar(543, p[914]);s.store_scalar(544, p[915]);s.store_scalar(545, p[916]);s.store_scalar(553, p[931]);s.store_scalar(636, p[923]);s.store_scalar(637, p[924]);s.store_scalar(638, p[925]);s.store_scalar(639, p[926]);s.store_scalar(546, p[917]);s.store_scalar(547, p[918]);s.store_scalar(548, p[919]);s.store_scalar(549, p[920]);s.store_scalar(550, p[921]);s.store_scalar(551, p[922]);}
        s.store_primal_offset(554, 514, s.v[375]);s.store_primal_offset(555, 515, s.v[375]);s.store_primal_offset(556, 516, s.v[375]);s.store_primal_sub_from_scalar(575, 1.0, 511);s.store_primal_sub_from_scalar(576, 1.0, 512);s.store_primal_sub_from_scalar(577, 1.0, 513);s.store_primal_div_from_scalar(578, 1.0, 575);s.store_primal_div_from_scalar(579, 1.0, 576);s.store_primal_div_from_scalar(580, 1.0, 577);s.store_primal_div_from_scalar(590, s.v[767], 505);s.store_primal_div_scaled_inputs_indices(591, 520, s.v[767], 506, 1.0);s.store_primal_div_scaled_inputs_indices(592, 521, s.v[767], 507, 1.0);s.store_primal_div_from_scalar(593, 1.0, 590);s.store_primal_div_from_scalar(594, 1.0, 591);s.store_primal_div_from_scalar(595, 1.0, 592);s.store_primal_div_from_scalar(596, 1.0, 508);s.store_primal_div_from_scalar(597, 1.0, 509);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_2(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();s.store_primal_div_from_scalar(598, 1.0, 510);s.store_primal_div_from_scalar(614, 1.0, 540);s.store_primal_div_from_scalar(615, 1.0, 541);s.store_div_from_scalar(616, 1.0, 542);s.b[1000] = ((((s.v[546] != 1.0) || (s.v[547] != 1.0)) || (s.v[548] != 1.0)) || (s.v[549] != 1.0));s.store_scalar(1000, if s.b[1000] { 1.0 } else { 0.0 });
        if s.b[1000] {s.store_scalar(635, 1.0);}
        if (!s.b[1000]) {s.store_scalar(635, 0.0);}
        s.b[1001] = (s.v[635] == 1.0);s.store_scalar(1001, if s.b[1001] { 1.0 } else { 0.0 });
        if s.b[1001] {
            if ((s.v[507] * s.v[546]) > 1e-18) {
                s.store_primal_mul(620, 507, 546);
            } else {
                s.store_scalar(620, 1e-18);
            }
        }
        if s.b[1001] {
            if ((s.v[510] * s.v[547]) > 0.05) {
                s.store_primal_mul(621, 510, 547);
            } else {
                s.store_scalar(621, 0.05);
            }
        }
        if s.b[1001] {
            if ((if ((s.v[513] * s.v[548]) > 0.05) { (s.v[513] * s.v[548]) } else { 0.05 }) < 0.95) {
                if ((s.v[513] * s.v[548]) > 0.05) {
                    s.store_primal_mul(622, 513, 548);
                } else {
                    s.store_scalar(622, 0.05);
                }
            } else {
                s.store_scalar(622, 0.95);
            }
        }
        if s.b[1001] {s.store_primal_mul(623, 516, 549);s.store_primal_offset(625, 623, s.v[375]);s.store_primal_sub_from_scalar(630, 1.0, 622);s.store_primal_div_from_scalar(631, 1.0, 630);}
        s.store_scalar(351, ((ctx_temp + p[56]) + p[35]));s.store_scalar(352, (s.v[351] / s.v[350]));s.store_scalar(353, (s.v[351] - s.v[350]));s.store_scalar(354, ((s.v[351] * 1.3806505e-23) / 1.6021918e-19));s.store_scalar(355, (1.0 / s.v[354]));s.store_scalar(356, s.v[351]);s.store_scalar(357, (s.v[356] * s.v[356]));s.store_scalar(358, (s.v[356] - s.v[350]));s.store_scalar(359, (s.v[350] / s.v[356]));s.store_scalar(360, ((s.v[359]) as f64).ln());s.store_scalar(715, ((s.v[356] * 1.3806505e-23) / 1.6021918e-19));s.store_scalar(361, (1.0 / s.v[715]));s.store_scalar(362, ((1.179 - (9.025e-5 * s.v[356])) - (3.05e-7 * s.v[357])));s.store_scalar(363, ((((1.045 + (0.00045 * s.v[356])) * ((0.523 + (0.0014 * s.v[356])) - (1.48e-6 * s.v[357]))) * s.v[357]) / 90000.0));
        if (!(s.v[363] > 0.001)) {s.store_scalar(363, 0.001);}
        s.store_scalar(365, (((ctx_temp + p[56]) + p[35])).max((273.15 + (-250.0))));s.store_scalar(366, (s.v[365] / s.v[364]));s.store_scalar(370, (s.v[367] * s.v[365]));s.store_scalar(371, (1.0 / s.v[370]));s.store_scalar(376, ((-((0.000702 * s.v[365]) * s.v[365])) / (1108.0 + s.v[365])));s.store_scalar(381, (p[834] + s.v[376]));s.store_scalar(382, (p[835] + s.v[376]));s.store_scalar(383, (p[836] + s.v[376]));s.store_scalar(384, (((s.v[366]) as f64).powf(1.5) * (((0.5 * ((s.v[378] * s.v[369]) - (s.v[381] * s.v[371])))) as f64).exp()));s.store_scalar(385, (((s.v[366]) as f64).powf(1.5) * (((0.5 * ((s.v[379] * s.v[369]) - (s.v[382] * s.v[371])))) as f64).exp()));s.store_scalar(386, (((s.v[366]) as f64).powf(1.5) * (((0.5 * ((s.v[380] * s.v[369]) - (s.v[383] * s.v[371])))) as f64).exp()));s.store_scalar(387, ((p[837] * s.v[384]) * s.v[384]));s.store_scalar(388, ((p[838] * s.v[385]) * s.v[385]));s.store_scalar(389, ((p[839] * s.v[386]) * s.v[386]));s.store_scalar(390, ((p[828] * s.v[366]) - ((2.0 * s.v[370]) * ((s.v[384]) as f64).ln())));s.store_scalar(391, ((p[829] * s.v[366]) - ((2.0 * s.v[370]) * ((s.v[385]) as f64).ln())));s.store_scalar(392, ((p[830] * s.v[366]) - ((2.0 * s.v[370]) * ((s.v[386]) as f64).ln())));s.store_scalar(393, (s.v[390] + (s.v[370] * (((1.0 + ((((0.05 - s.v[390]) * s.v[371])) as f64).exp())) as f64).ln())));s.store_scalar(394, (s.v[391] + (s.v[370] * (((1.0 + ((((0.05 - s.v[391]) * s.v[371])) as f64).exp())) as f64).ln())));s.store_scalar(395, (s.v[392] + (s.v[370] * (((1.0 + ((((0.05 - s.v[392]) * s.v[371])) as f64).exp())) as f64).ln())));s.store_scalar(405, (1.0 / s.v[393]));s.store_scalar(406, (1.0 / s.v[394]));s.store_scalar(407, (1.0 / s.v[395]));s.store_scalar(414, (p[825] * (((p[828] * s.v[405])) as f64).powf(p[831])));s.store_scalar(415, (p[826] * (((p[829] * s.v[406])) as f64).powf(p[832])));s.store_scalar(416, (p[827] * (((p[830] * s.v[407])) as f64).powf(p[833])));s.store_scalar(417, ((s.v[414] * s.v[393]) * s.v[411]));s.store_scalar(418, ((s.v[415] * s.v[394]) * s.v[412]));s.store_scalar(419, ((s.v[416] * s.v[395]) * s.v[413]));s.store_scalar(420, (2.0 * s.v[414]));s.store_scalar(421, (2.0 * s.v[415]));s.store_scalar(422, (2.0 * s.v[416]));s.store_scalar(432, ((0.5 * s.v[381])).max(s.v[370]));s.store_scalar(433, ((0.5 * s.v[382])).max(s.v[370]));s.store_scalar(434, ((0.5 * s.v[383])).max(s.v[370]));s.store_scalar(435, (s.v[432] * s.v[371]));s.store_scalar(436, (s.v[433] * s.v[371]));s.store_scalar(437, (s.v[434] * s.v[371]));s.store_scalar(438, (((((((32.0 * p[848]) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[432] * s.v[432]) * s.v[432]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));s.store_scalar(439, (((((((32.0 * p[849]) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[433] * s.v[433]) * s.v[433]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));s.store_scalar(440, (((((((32.0 * p[850]) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[434] * s.v[434]) * s.v[434]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));s.store_scalar(441, (p[854] * (1.0 + (p[857] * (s.v[365] - s.v[364])))));s.store_scalar(442, (p[855] * (1.0 + (p[858] * (s.v[365] - s.v[364])))));s.store_scalar(443, (p[856] * (1.0 + (p[859] * (s.v[365] - s.v[364])))));
        if (!(s.v[441] > 0.0)) {s.store_scalar(441, 0.0);}
        if (!(s.v[442] > 0.0)) {s.store_scalar(442, 0.0);}
        if (!(s.v[443] > 0.0)) {s.store_scalar(443, 0.0);}
        s.b[1021] = (s.v[473] == 1.0);s.store_scalar(1021, if s.b[1021] { 1.0 } else { 0.0 });
        if s.b[1021] {s.store_primal_offset(461, 460, s.v[376]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
    ) {
        if s.b[1021] {s.store_primal_scale_ad(463, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(462), s.v[369], s.ad_value(461), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));s.store_primal_sub_scaled_inputs_ln_rhs(464, 458, s.v[366], 463, (2.0 * s.v[370]));s.store_primal_add_scaled_inputs_mixed_ia(465, 464, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(464), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);s.store_primal_div_from_scalar(466, 1.0, 465);s.store_primal_mul_pow_mixed_iai(469, 457, A::mul(s.ad_value(458), s.ad_value(466)), 459);s.store_primal_mul3_lhs(470, 469, 465, 468);s.store_primal_scale(471, 469, 2.0);}
        s.store_primal_offset(557, 514, s.v[376]);s.store_primal_offset(558, 515, s.v[376]);s.store_primal_offset(559, 516, s.v[376]);s.store_primal_scale_ad(560, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(554), s.v[369], s.ad_value(557), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));s.store_primal_scale_ad(561, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(555), s.v[369], s.ad_value(558), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));s.store_primal_scale_ad(562, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(556), s.v[369], s.ad_value(559), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));s.store_primal_mul3_lhs(563, 517, 560, 560);s.store_primal_mul3_lhs(564, 518, 561, 561);s.store_primal_mul3_lhs(565, 519, 562, 562);s.store_primal_sub_scaled_inputs_ln_rhs(566, 508, s.v[366], 560, (2.0 * s.v[370]));s.store_primal_sub_scaled_inputs_ln_rhs(567, 509, s.v[366], 561, (2.0 * s.v[370]));s.store_primal_sub_scaled_inputs_ln_rhs(568, 510, s.v[366], 562, (2.0 * s.v[370]));s.store_primal_add_scaled_inputs_mixed_ia(569, 566, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(566), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);s.store_primal_add_scaled_inputs_mixed_ia(570, 567, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(567), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);s.store_primal_add_scaled_inputs_mixed_ia(571, 568, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(568), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);s.store_primal_div_from_scalar(572, 1.0, 569);s.store_primal_div_from_scalar(573, 1.0, 570);s.store_primal_div_from_scalar(574, 1.0, 571);s.store_primal_mul_pow_mixed_iai(581, 505, A::mul(s.ad_value(508), s.ad_value(572)), 511);s.store_primal_mul_pow_mixed_iai(582, 506, A::mul(s.ad_value(509), s.ad_value(573)), 512);s.store_primal_mul_pow_mixed_iai(583, 507, A::mul(s.ad_value(510), s.ad_value(574)), 513);s.store_primal_mul3_lhs(584, 581, 569, 578);s.store_primal_mul3_lhs(585, 582, 570, 579);s.store_primal_mul3_lhs(586, 583, 571, 580);s.store_primal_scale(587, 581, 2.0);s.store_primal_scale(588, 582, 2.0);s.store_primal_scale(589, 583, 2.0);s.store_primal_max_with_scalar_ad(599, A::scale(s.ad_value(557), 0.5), s.v[370]);s.store_primal_max_with_scalar_ad(600, A::scale(s.ad_value(558), 0.5), s.v[370]);s.store_primal_max_with_scalar_ad(601, A::scale(s.ad_value(559), 0.5), s.v[370]);s.store_primal_scale(602, 599, s.v[371]);s.store_primal_scale(603, 600, s.v[371]);s.store_primal_scale(604, 601, s.v[371]);s.store_primal_scaled_sqrt_ad(605, A::mul3_scaled_output(s.ad_value(528), A::square(s.ad_value(599)), s.ad_value(599), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_primal_scaled_sqrt_ad(606, A::mul3_scaled_output(s.ad_value(529), A::square(s.ad_value(600)), s.ad_value(600), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));s.store_primal_scaled_sqrt_ad(607, A::mul3_scaled_output(s.ad_value(530), A::square(s.ad_value(601)), s.ad_value(601), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));s.store_primal_mul_scale_offset_rhs(608, 534, 537, (s.v[365] - s.v[364]), 1.0);s.store_primal_mul_scale_offset_rhs(609, 535, 538, (s.v[365] - s.v[364]), 1.0);s.store_mul_scale_offset_rhs(610, 536, 539, (s.v[365] - s.v[364]), 1.0);
        if (!(s.v[608] > 0.0)) {s.store_scalar(608, 0.0);}
        if (!(s.v[609] > 0.0)) {s.store_scalar(609, 0.0);}
        if (!(s.v[610] > 0.0)) {s.store_scalar(610, 0.0);}
        s.b[1022] = (s.v[635] == 1.0);s.store_scalar(1022, if s.b[1022] { 1.0 } else { 0.0 });
        if s.b[1022] {s.store_primal_offset(624, 623, s.v[376]);s.store_primal_scale_ad(626, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(625), s.v[369], s.ad_value(624), s.v[371]), 0.5), ((s.v[366]) as f64).powf(1.5));s.store_primal_sub_scaled_inputs_ln_rhs(627, 621, s.v[366], 626, (2.0 * s.v[370]));s.store_primal_add_scaled_inputs_mixed_ia(628, 627, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(627), (-s.v[371]), ((0.05) * (s.v[371])))), s.v[370]);s.store_primal_div_from_scalar(629, 1.0, 628);s.store_primal_mul_pow_mixed_iai(632, 620, A::mul(s.ad_value(621), s.ad_value(629)), 622);s.store_primal_mul3_lhs(633, 632, 628, 631);s.store_primal_scale(634, 632, 2.0);}
        s.store_scalar(5, 1.0);s.store_scalar(6, 1.0);s.store_scalar(312, 0.0);s.store_scalar(313, 0.0);s.store_scalar(7, p[0]);s.store_scalar(8, p[1]);s.store_scalar(9, p[2]);s.store_scalar(10, p[3]);s.store_scalar(11, p[4]);s.store_scalar(12, p[8]);s.store_scalar(646, p[19]);s.store_scalar(647, p[20]);s.store_scalar(648, p[21]);s.store_scalar(673, p[22]);s.store_scalar(674, p[23]);s.store_scalar(675, p[24]);s.store_scalar(649, p[25]);s.store_scalar(650, p[26]);s.store_scalar(676, p[27]);s.store_scalar(677, p[28]);s.store_scalar(14, p[14]);s.b[1023] = (p[39] > 0.0);s.store_scalar(1023, if s.b[1023] { 1.0 } else { 0.0 });
        if s.b[1023] {s.store_scalar(5, (if (p[9] > 1.0) { p[9] } else { 1.0 }));}
        if s.b[1023] {s.store_primal_floor_ad(5, A::offset(s.ad_value(5), 0.5));s.store_primal_div_from_scalar(6, 1.0, 5);}
        if ((s.v[8] * s.v[6]) > 1e-9) {
            s.store_primal_scale(8, 6, s.v[8]);
        } else {
            s.store_scalar(8, 1e-9);
        }
        s.store_scalar(15, p[5]);s.store_scalar(16, p[6]);s.store_scalar(17, p[7]);s.store_scalar(308, (1e-6 / s.v[7]));s.store_primal_div_from_scalar(309, 1e-6, 8);s.store_primal_offset_scaled(310, 309, ((p[190]) * ((p[188] * (1.0 + (p[189] * s.v[308]))))), (p[188] * (1.0 + (p[189] * s.v[308]))));s.store_primal_offset_scaled(311, 309, ((p[194]) * ((p[192] * (1.0 + (p[193] * s.v[308]))))), (p[192] * (1.0 + (p[193] * s.v[308]))));
        if (((s.v[7] + s.v[310]) - (2.0 * p[191])) > 1e-9) {
            s.store_primal_offset(312, 310, ((s.v[7]) + ((-(2.0 * p[191])))));
        } else {
            s.store_scalar(312, 1e-9);
        }
        if (((s.v[8] + s.v[311]) - (2.0 * p[195])) > 1e-9) {
            s.store_primal_offset_add(313, 8, 311, (-(2.0 * p[195])));
        } else {
            s.store_scalar(313, 1e-9);
        }
        s.store_primal_div_from_scalar(314, 1e-6, 312);s.store_primal_square(315, 314);s.store_primal_div_from_scalar(316, 1e-6, 313);s.store_primal_div_from_scalar(317, 1.0, 316);s.store_primal_mul(318, 314, 316);s.store_primal_div_from_scalar(319, 1.0, 318);
        if ((((s.v[7] + s.v[310]) - (2.0 * p[191])) + p[196]) > 1e-9) {
            s.store_primal_offset(320, 310, ((((s.v[7]) + ((-(2.0 * p[191]))))) + (p[196])));
        } else {
            s.store_scalar(320, 1e-9);
        }
        if ((((s.v[8] + s.v[311]) - (2.0 * p[195])) + p[197]) > 1e-9) {
            s.store_primal_offset_add(321, 8, 311, (((-(2.0 * p[195]))) + (p[197])));
        } else {
            s.store_scalar(321, 1e-9);
        }
        s.store_primal_scale(322, 321, 1000000.0);
        if (((s.v[7] + s.v[310]) + p[196]) > 1e-9) {
            s.store_primal_offset(323, 310, ((s.v[7]) + (p[196])));
        } else {
            s.store_scalar(323, 1e-9);
        }
        if (((s.v[8] + s.v[311]) + p[197]) > 1e-9) {
            s.store_primal_offset_add(324, 8, 311, p[197]);
        } else {
            s.store_scalar(324, 1e-9);
        }
        s.store_primal_scale(325, 323, 1000000.0);s.store_primal_scale(326, 324, 1000000.0);
    }
}

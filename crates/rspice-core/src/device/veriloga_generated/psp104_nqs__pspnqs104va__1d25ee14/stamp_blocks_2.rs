#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_32(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2313] = (s.v[237] > 0.0);
        s.v[2313] = if s.b[2313] { 1.0 } else { 0.0 };

        if (s.b[2312] && s.b[2313]) {
            s.store_mul_sqrt_ad_lhs(2044, A::offset(A::square(s.ad_value(1878)), 1e-6), 795);
        }

        s.b[2314] = (s.v[243] < 0.0);
        s.v[2314] = if s.b[2314] { 1.0 } else { 0.0 };

        if ((s.b[2312] && s.b[2313]) && s.b[2314]) {
            s.store_ad_value(2044, A::add_scaled_inputs3(s.ad_value(2044), 0.5, s.ad_value(801), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2044), s.ad_value(801)), A::sub(s.ad_value(2044), s.ad_value(801))), 1e-6)), (-0.5)));
        }

        if (s.b[2312] && s.b[2313]) {
            s.store_mul_offset_ad_rhs(2027, 798, A::mul(s.ad_value(2044), A::add_scaled_product(s.ad_value(242), 1.0, s.ad_value(243), s.ad_value(2044), 1.0)), (-1.5));
        }

        s.b[2315] = (s.v[2027] > 0.0);
        s.v[2315] = if s.b[2315] { 1.0 } else { 0.0 };

        if ((s.b[2312] && s.b[2313]) && s.b[2315]) {
            s.store_offset_ad(2045, A::mul_offset_rhs(s.ad_value(2027), A::mul_scaled_output(s.ad_value(2027), A::scale_offset(s.ad_value(2027), 0.3333333333333333, 1.0), 0.5), 1.0), 1.0);
        }

        s.b[2316] = (s.v[2027] > (-230.25850929940458));
        s.v[2316] = if s.b[2316] { 1.0 } else { 0.0 };

        if (((s.b[2312] && s.b[2313]) && (!s.b[2315])) && s.b[2316]) {
            s.store_exp(2045, 2027);
        }

        if (((s.b[2312] && s.b[2313]) && (!s.b[2315])) && (!s.b[2316])) {
            s.store_div_from_scalar_offset_ad(2045, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2027), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2027), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (s.b[2312] && s.b[2313]) {
            s.store_offset(2046, 2042, 3.0);
            s.store_sub_from_scalar(2047, (-3.0), 235);
            s.store_scale(2048, 834, 30.0);
            s.store_scalar(818, (4.0 - 0.9));
            s.store_add(819, 2046, 2048);
            s.store_mul_ad(2027, A::div_from_scalar(2.0, s.ad_value(818)), A::sub(s.ad_value(819), A::sqrt(A::sub(A::square(s.ad_value(819)), A::mul3(s.ad_value(818), s.ad_value(2046), s.ad_value(2048))))));
            s.store_scalar(818, (4.0 - 0.3));
            s.store_add(819, 2047, 2027);
            s.store_mul_ad(2049, A::div_from_scalar(2.0, s.ad_value(818)), A::add(s.ad_value(819), A::sqrt(A::sub(A::square(s.ad_value(819)), A::mul3(s.ad_value(818), s.ad_value(2047), s.ad_value(2027))))));
            s.store_mul3_lhs(839, 237, 2045, 2049);
        }

        s.b[2317] = (s.v[238] > 0.0);
        s.v[2317] = if s.b[2317] { 1.0 } else { 0.0 };

        if (s.b[2312] && s.b[2317]) {
            s.store_mul_sqrt_ad_lhs(2044, A::offset(A::square(s.ad_value(1879)), 1e-6), 795);
        }

        s.b[2318] = (s.v[245] < 0.0);
        s.v[2318] = if s.b[2318] { 1.0 } else { 0.0 };

        if ((s.b[2312] && s.b[2317]) && s.b[2318]) {
            s.store_ad_value(2044, A::add_scaled_inputs3(s.ad_value(2044), 0.5, s.ad_value(802), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2044), s.ad_value(802)), A::sub(s.ad_value(2044), s.ad_value(802))), 1e-6)), (-0.5)));
        }

        if (s.b[2312] && s.b[2317]) {
            s.store_mul_offset_ad_rhs(2027, 799, A::mul(s.ad_value(2044), A::add_scaled_product(s.ad_value(244), 1.0, s.ad_value(245), s.ad_value(2044), 1.0)), (-1.5));
        }

        s.b[2319] = (s.v[2027] > 0.0);
        s.v[2319] = if s.b[2319] { 1.0 } else { 0.0 };

        if ((s.b[2312] && s.b[2317]) && s.b[2319]) {
            s.store_offset_ad(2045, A::mul_offset_rhs(s.ad_value(2027), A::mul_scaled_output(s.ad_value(2027), A::scale_offset(s.ad_value(2027), 0.3333333333333333, 1.0), 0.5), 1.0), 1.0);
        }

        s.b[2320] = (s.v[2027] > (-230.25850929940458));
        s.v[2320] = if s.b[2320] { 1.0 } else { 0.0 };

        if (((s.b[2312] && s.b[2317]) && (!s.b[2319])) && s.b[2320]) {
            s.store_exp(2045, 2027);
        }

        if (((s.b[2312] && s.b[2317]) && (!s.b[2319])) && (!s.b[2320])) {
            s.store_div_from_scalar_offset_ad(2045, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2027), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2027), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (s.b[2312] && s.b[2317]) {
            s.store_offset(2046, 2043, 3.0);
            s.store_sub_from_scalar(2047, (-3.0), 235);
            s.store_scale(2048, 837, 30.0);
            s.store_scalar(818, (4.0 - 0.9));
            s.store_add(819, 2046, 2048);
            s.store_mul_ad(2027, A::div_from_scalar(2.0, s.ad_value(818)), A::sub(s.ad_value(819), A::sqrt(A::sub(A::square(s.ad_value(819)), A::mul3(s.ad_value(818), s.ad_value(2046), s.ad_value(2048))))));
            s.store_scalar(818, (4.0 - 0.3));
            s.store_add(819, 2047, 2027);
            s.store_mul_ad(2049, A::div_from_scalar(2.0, s.ad_value(818)), A::add(s.ad_value(819), A::sqrt(A::sub(A::square(s.ad_value(819)), A::mul3(s.ad_value(818), s.ad_value(2047), s.ad_value(2027))))));
            s.store_mul3_lhs(840, 238, 2045, 2049);
        }

        s.b[2321] = (s.v[236] > 0.0);
        s.v[2321] = if s.b[2321] { 1.0 } else { 0.0 };

        s.b[2322] = (s.v[1829] <= 0.0);
        s.v[2322] = if s.b[2322] { 1.0 } else { 0.0 };

        if ((s.b[2312] && s.b[2321]) && s.b[2322]) {
            s.store_offset(2027, 777, 1.0);
            s.store_ad_value(2028, A::div_scaled_product(A::sqrt(s.ad_value(2027)), s.ad_value(826), 1.0, s.ad_value(1855), 1.0));
            s.store_add_ad_lhs(2029, A::square(s.ad_value(2028)), 2027);
            s.store_scale(2027, 2028, 2.0);
            s.store_ad_value(1858, A::div_scaled_product3(s.ad_value(1855), s.ad_value(1825), s.ad_value(2027), 1.0, A::add(A::sqrt(A::sub(s.ad_value(2029), s.ad_value(2027))), A::sqrt(A::add(s.ad_value(2029), s.ad_value(2027)))), 1.0));
        }

        s.b[2323] = ((s.v[1859] - s.v[1858]) > (-230.25850929940458));
        s.v[2323] = if s.b[2323] { 1.0 } else { 0.0 };

        if ((s.b[2312] && s.b[2321]) && s.b[2323]) {
            s.store_exp_sub(2027, 1859, 1858);
        }

        if ((s.b[2312] && s.b[2321]) && (!s.b[2323])) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1859), s.ad_value(1858)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1859), s.ad_value(1858)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (s.b[2312] && s.b[2321]) {
            s.store_ad_value(2050, A::add_scaled_product(s.ad_value(2030), 1.0, s.ad_value(1824), A::sub_scaled_inputs(s.ad_value(1859), 0.5, A::ln_scaled_input(A::offset(s.ad_value(2027), 1.0), 0.5), 1.0), 1.0));
            s.store_mul(2051, 235, 1824);
            s.store_add(2052, 1872, 2051);
            s.store_scaled_sub_ad_rhs(2053, 2052, A::sqrt(A::offset(A::mul_scaled_output(s.ad_value(2052), s.ad_value(2052), 1.0), 0.01)), 0.5);
            s.store_mul_sqrt_ad_lhs(2044, A::offset(A::square(s.ad_value(1872)), 1e-6), 795);
        }

        s.b[2324] = (s.v[241] < 0.0);
        s.v[2324] = if s.b[2324] { 1.0 } else { 0.0 };

        if ((s.b[2312] && s.b[2321]) && s.b[2324]) {
            s.store_ad_value(2044, A::add_scaled_inputs3(s.ad_value(2044), 0.5, s.ad_value(800), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2044), s.ad_value(800)), A::sub(s.ad_value(2044), s.ad_value(800))), 1e-6)), (-0.5)));
        }

        if (s.b[2312] && s.b[2321]) {
            s.store_ad_value(2054, A::add_scaled_product(s.ad_value(1862), 1.0, A::add_scaled_inputs3(s.ad_value(2053), 1.0, s.ad_value(742), (-1.0), s.ad_value(2050), -1.0), s.ad_value(1825), 1.0));
        }

        s.b[2325] = (((s.v[2054]) as f64).abs() < 230.25850929940458);
        s.v[2325] = if s.b[2325] { 1.0 } else { 0.0 };

        if ((s.b[2312] && s.b[2321]) && s.b[2325]) {
            s.store_exp(2055, 2054);
        }

        s.b[2326] = (s.v[2054] < 0.0);
        s.v[2326] = if s.b[2326] { 1.0 } else { 0.0 };

        if (((s.b[2312] && s.b[2321]) && (!s.b[2325])) && s.b[2326]) {
            s.store_div_from_scalar_offset_ad(2055, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2054), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2054), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[2312] && s.b[2321]) && (!s.b[2325])) && (!s.b[2326])) {
            s.store_scaled_offset_ad(2055, A::mul_offset_lhs(s.ad_value(2054), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2054), (-230.25850929940458), A::scale_offset(s.ad_value(2054), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (s.b[2312] && s.b[2321]) {
            s.store_mul_neg_ad_lhs(2054, A::add_scaled_inputs3(s.ad_value(825), 1.0, s.ad_value(2030), 1.0, s.ad_value(2050), -1.0), 1825);
        }

        s.b[2327] = (((s.v[2054]) as f64).abs() < 230.25850929940458);
        s.v[2327] = if s.b[2327] { 1.0 } else { 0.0 };

        if ((s.b[2312] && s.b[2321]) && s.b[2327]) {
            s.store_exp(2027, 2054);
        }

        s.b[2328] = (s.v[2054] < 0.0);
        s.v[2328] = if s.b[2328] { 1.0 } else { 0.0 };

        if (((s.b[2312] && s.b[2321]) && (!s.b[2327])) && s.b[2328]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2054), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2054), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[2312] && s.b[2321]) && (!s.b[2327])) && (!s.b[2328])) {
            s.store_scaled_offset_ad(2027, A::mul_offset_lhs(s.ad_value(2054), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2054), (-230.25850929940458), A::scale_offset(s.ad_value(2054), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (s.b[2312] && s.b[2321]) {
            s.store_mul(2056, 2055, 2027);
            s.store_mul_offset_ad_rhs(2027, 797, A::mul(s.ad_value(2044), A::add_scaled_product(s.ad_value(240), 1.0, s.ad_value(241), s.ad_value(2044), 1.0)), (-1.5));
        }

        s.b[2329] = (s.v[2027] > 0.0);
        s.v[2329] = if s.b[2329] { 1.0 } else { 0.0 };

        if ((s.b[2312] && s.b[2321]) && s.b[2329]) {
            s.store_offset_ad(2045, A::mul_offset_rhs(s.ad_value(2027), A::mul_scaled_output(s.ad_value(2027), A::scale_offset(s.ad_value(2027), 0.3333333333333333, 1.0), 0.5), 1.0), 1.0);
        }

        s.b[2330] = (s.v[2027] > (-230.25850929940458));
        s.v[2330] = if s.b[2330] { 1.0 } else { 0.0 };

        if (((s.b[2312] && s.b[2321]) && (!s.b[2329])) && s.b[2330]) {
            s.store_exp(2045, 2027);
        }

        if (((s.b[2312] && s.b[2321]) && (!s.b[2329])) && (!s.b[2330])) {
            s.store_div_from_scalar_offset_ad(2045, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2027), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2027), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (s.b[2312] && s.b[2321]) {
            s.store_mul_ad_product_rhs(2057, 236, s.ad_value(2045), A::ln(A::div(A::offset(s.ad_value(2055), 1.0), A::offset(s.ad_value(2056), 1.0))));
        }

        s.b[2331] = ((s.v[1829] <= 0.0) || ((s.v[240] == 0.0) && (s.v[241] == 0.0)));
        s.v[2331] = if s.b[2331] { 1.0 } else { 0.0 };

        if ((s.b[2312] && s.b[2321]) && s.b[2331]) {
            s.store_scalar(2064, 1.0);
            s.store_scalar(2065, 0.5);
        }

        if ((s.b[2312] && s.b[2321]) && (!s.b[2331])) {
            s.store_ad_value(2027, A::add_scaled_product(s.ad_value(240), 1.0, s.ad_value(241), s.ad_value(2044), 2.0));
            s.store_div_ad_rhs(2058, 246, A::mul(s.ad_value(2027), s.ad_value(797)));
            s.store_scaled_div(2059, 1860, 2058, 0.5);
            s.store_div(2060, 2058, 1877);
            s.store_ad_value(2061, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2060), 1.0, s.ad_value(2060), 0.5));
            s.store_sub_from_scalar_ad(2062, 0.5, A::scale(s.ad_value(2061), 3.0));
        }

        s.b[2332] = (s.v[2059] < 0.001);
        s.v[2332] = if s.b[2332] { 1.0 } else { 0.0 };

        if (((s.b[2312] && s.b[2321]) && (!s.b[2331])) && s.b[2332]) {
            s.store_square(2063, 2059);
            s.store_offset_mul_ad(2064, s.ad_value(2063), A::add_scaled_product(A::scale_offset(s.ad_value(2060), 0.3333333333333333, 0.16666666666666666), 1.0, s.ad_value(2063), A::scale_offset(s.ad_value(2060), 0.2, 0.05), 0.16666666666666666), 1.0);
            s.store_ad_value(2065, A::add_scaled_product(s.ad_value(2064), 0.5, s.ad_value(2059), A::offset(A::mul(s.ad_value(2063), A::add_scaled_product(A::scaled_offset(s.ad_value(2061), 0.25, 0.4), 1.0, s.ad_value(2063), A::offset(s.ad_value(2061), 0.125), 0.0285714285714)), 1.0), (-0.16666666666666666)));
        }

        if (((s.b[2312] && s.b[2321]) && (!s.b[2331])) && (!s.b[2332])) {
            s.store_div_from_scalar(2066, 1.0, 2059);
        }

        s.b[2333] = (((s.v[2059]) as f64).abs() < 230.25850929940458);
        s.v[2333] = if s.b[2333] { 1.0 } else { 0.0 };

        if ((((s.b[2312] && s.b[2321]) && (!s.b[2331])) && (!s.b[2332])) && s.b[2333]) {
            s.store_exp(2067, 2059);
        }

        s.b[2334] = (s.v[2059] < 0.0);
        s.v[2334] = if s.b[2334] { 1.0 } else { 0.0 };

        if (((((s.b[2312] && s.b[2321]) && (!s.b[2331])) && (!s.b[2332])) && (!s.b[2333])) && s.b[2334]) {
            s.store_div_from_scalar_offset_ad(2067, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2059), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2059), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2312] && s.b[2321]) && (!s.b[2331])) && (!s.b[2332])) && (!s.b[2333])) && (!s.b[2334])) {
            s.store_scaled_offset_ad(2067, A::mul_offset_lhs(s.ad_value(2059), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2059), (-230.25850929940458), A::scale_offset(s.ad_value(2059), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((s.b[2312] && s.b[2321]) && (!s.b[2331])) && (!s.b[2332])) {
            s.store_div_from_scalar(2068, 1.0, 2067);
            s.store_sub(2027, 2067, 2068);
            s.store_add(2029, 2067, 2068);
            s.store_ad_value(2064, A::add_scaled_products(A::mul_sub_from_scalar_lhs(1.0, s.ad_value(2060), s.ad_value(2027)), s.ad_value(2066), 0.5, s.ad_value(2060), s.ad_value(2029), 0.5));
            s.store_scaled_sub_ad(2065, A::add_scaled_product(s.ad_value(2064), 1.0, s.ad_value(2027), A::sub(s.ad_value(2061), A::mul3(s.ad_value(2062), s.ad_value(2066), s.ad_value(2066))), (-1.0)), A::mul3(s.ad_value(2062), s.ad_value(2029), s.ad_value(2066)), 0.5);
        }

        if (s.b[2312] && s.b[2321]) {
            s.store_scaled_offset_ad(2069, A::div(s.ad_value(1829), A::sqrt(A::offset(A::square(s.ad_value(1829)), 1e-6))), 1.0, 0.5);
            s.store_mul3_lhs(2070, 2057, 2064, 2069);
            s.store_mul3_lhs(842, 2057, 2065, 2069);
            s.store_sub(841, 2070, 842);
            s.store_mul_ad_product_rhs(843, 2057, s.ad_value(2064), A::sub_from_scalar(1.0, s.ad_value(2069)));
        }

        s.v[845] = 0.0;

        s.v[844] = 0.0;

        s.b[2335] = (p.p42 != 0.0);
        s.v[2335] = if s.b[2335] { 1.0 } else { 0.0 };

        s.b[2336] = ((s.v[248] > 0.0) && (s.v[1879] < 0.0));
        s.v[2336] = if s.b[2336] { 1.0 } else { 0.0 };

        if (s.b[2335] && s.b[2336]) {
            s.store_sqrt_offset_ad(2071, A::add_scaled_square_product(s.ad_value(1879), 1.0, A::square(s.ad_value(254)), A::square(s.ad_value(836)), 1.0), 1e-6);
            s.store_scaled_div(2027, 807, 2071, -1.0);
        }

        s.b[2337] = (s.v[2027] > (-230.25850929940458));
        s.v[2337] = if s.b[2337] { 1.0 } else { 0.0 };

        if ((s.b[2335] && s.b[2336]) && s.b[2337]) {
            s.store_exp(2029, 2027);
        }

        if ((s.b[2335] && s.b[2336]) && (!s.b[2337])) {
            s.store_div_from_scalar_offset_ad(2029, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2027), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2027), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (s.b[2335] && s.b[2336]) {
            s.store_mul_ad_affine_product_lhs(845, s.ad_value(805), A::mul3(s.ad_value(836), s.ad_value(1879), s.ad_value(2071)), -1.0, 0.0, 2029);
        }

        s.b[2338] = ((s.v[247] > 0.0) && (s.v[1878] < 0.0));
        s.v[2338] = if s.b[2338] { 1.0 } else { 0.0 };

        if (s.b[2335] && s.b[2338]) {
            s.store_sqrt_offset_ad(2072, A::add_scaled_square_product(s.ad_value(1878), 1.0, A::square(s.ad_value(253)), A::square(s.ad_value(835)), 1.0), 1e-6);
            s.store_scaled_div(2027, 806, 2072, -1.0);
        }

        s.b[2339] = (s.v[2027] > (-230.25850929940458));
        s.v[2339] = if s.b[2339] { 1.0 } else { 0.0 };

        if ((s.b[2335] && s.b[2338]) && s.b[2339]) {
            s.store_exp(2029, 2027);
        }

        if ((s.b[2335] && s.b[2338]) && (!s.b[2339])) {
            s.store_div_from_scalar_offset_ad(2029, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2027), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2027), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (s.b[2335] && s.b[2338]) {
            s.store_mul_ad_affine_product_lhs(844, s.ad_value(804), A::mul3(s.ad_value(835), s.ad_value(1878), s.ad_value(2072)), -1.0, 0.0, 2029);
        }

        s.v[2076] = s.v[715];

        s.v[1880] = 0.0;

        s.v[1881] = 0.0;

        s.v[1882] = 0.0;

        s.v[1883] = 1e-40;

        s.v[1884] = 1.0;

        s.v[846] = 0.0;

        s.b[2340] = ((p.p46 != 0.0) && (s.v[287] > 0.0));
        s.v[2340] = if s.b[2340] { 1.0 } else { 0.0 };

        if s.b[2340] {
            s.store_add_ad_lhs(2027, A::add_scaled_inputs3(s.ad_value(828), 0.5, s.ad_value(827), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(764), 1.0, A::sub(s.ad_value(828), s.ad_value(827)), A::sub(s.ad_value(828), s.ad_value(827)), 1.0)), (-0.5)), 762);
            s.store_add_ad_lhs(2073, A::add_scaled_inputs3(s.ad_value(827), 1.0, s.ad_value(2027), (-0.5), A::sqrt(A::add_scaled_product(s.ad_value(763), 1.0, s.ad_value(2027), s.ad_value(2027), 1.0)), (-(-0.5))), 766);
            s.store_ad_value(2074, A::add_scaled_inputs3(s.ad_value(2073), 1.0, s.ad_value(826), 0.5, s.ad_value(830), (-0.5)));
            s.store_mul_ad_product_rhs(2075, 289, A::offset(A::mul(s.ad_value(291), s.ad_value(830)), 1.0), A::offset(A::mul(s.ad_value(290), s.ad_value(2074)), 1.0));
            s.store_mul_offset_rhs(2076, 723, 2075, 1.0);
            s.store_div_from_scalar(2077, 1.0, 2076);
            s.store_ad_value(2078, A::div_scaled_inputs(s.ad_value(830), 2.0, A::offset(A::sqrt(A::offset(A::mul(s.ad_value(293), s.ad_value(830)), 1.0)), 1.0), 1.0));
            s.store_mul_ad_product_rhs(2079, 292, s.ad_value(2078), A::offset(A::mul(s.ad_value(294), s.ad_value(2074)), 1.0));
        }

    }

    pub(super) fn stamp_transient_block_33(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[2340] {
            s.store_mul_ad_rhs(1880, 2077, A::add_scaled_inputs3(s.ad_value(829), 1.0, s.ad_value(2079), 1.0, s.ad_value(713), -1.0));
            s.store_mul(2080, 2077, 760);
            s.store_scaled_ln_ad(2081, A::add(A::div(s.ad_value(2080), s.ad_value(761)), A::sqrt(s.ad_value(2080))), 2.0);
            s.store_mul(2082, 2077, 2073);
            s.store_add(2087, 2080, 2082);
            s.store_ad_value(2088, A::add_scaled_product(s.ad_value(2087), 1.0, s.ad_value(761), A::sqrt(s.ad_value(2087)), 1.0));
            s.store_add(2089, 2088, 2081);
            s.store_offset_ad(2090, A::div_scaled_inputs(s.ad_value(761), 1.0, A::sqrt(s.ad_value(2087)), 2.0), 1.0);
            s.store_div_from_scalar(2091, 1.0, 2090);
            s.store_sub(2092, 1880, 2089);
        }

        s.b[2341] = (s.v[2092] > (-12.0));
        s.v[2341] = if s.b[2341] { 1.0 } else { 0.0 };

        if (s.b[2340] && s.b[2341]) {
            s.store_offset_add(2093, 2092, 725, (-1.0));
            s.store_scaled_add_ad_rhs(2094, 2093, A::sqrt(A::offset(A::square(s.ad_value(2093)), 10.0)), 0.5);
            s.store_add_ad_lhs(2095, A::add_scaled_product(s.ad_value(2092), 1.0, s.ad_value(2090), A::ln(s.ad_value(2094)), (-1.0)), 725);
            s.store_scaled_add_ad_rhs(2096, 2095, A::sqrt(A::offset(A::square(s.ad_value(2095)), 2.0)), 0.5);
        }

        s.b[2342] = ((s.v[2092] - s.v[2096]) < 230.25850929940458);
        s.v[2342] = if s.b[2342] { 1.0 } else { 0.0 };

        if ((s.b[2340] && s.b[2341]) && s.b[2342]) {
            s.store_exp_sub(2097, 2092, 2096);
        }

        if ((s.b[2340] && s.b[2341]) && (!s.b[2342])) {
            s.store_scaled_offset_ad(2097, A::mul_offset_lhs(A::sub(s.ad_value(2092), s.ad_value(2096)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2092), s.ad_value(2096)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2092), s.ad_value(2096)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (s.b[2340] && s.b[2341]) {
            s.store_mul(2098, 724, 2097);
            s.store_pow_ad(2099, s.ad_value(2098), s.ad_value(2091));
            s.store_ad_value(2100, A::add_scaled_square_product(s.ad_value(2090), 1.0, A::add_scaled_inputs3(s.ad_value(2096), 2.0, s.ad_value(2090), 2.0, s.ad_value(2099), -1.0), s.ad_value(2099), 1.0));
            s.store_mul_offset_ad_rhs(2101, 2090, A::div(A::sub(A::sqrt(s.ad_value(2100)), s.ad_value(2090)), s.ad_value(2099)), (-1.0));
            s.store_sub(2083, 2096, 2101);
        }

        s.b[2343] = ((s.v[2091] * (s.v[2092] + s.v[725])) > (-230.25850929940458));
        s.v[2343] = if s.b[2343] { 1.0 } else { 0.0 };

        if ((s.b[2340] && (!s.b[2341])) && s.b[2343]) {
            s.store_exp_ad(2083, A::mul(s.ad_value(2091), A::add(s.ad_value(2092), s.ad_value(725))));
        }

        if ((s.b[2340] && (!s.b[2341])) && (!s.b[2343])) {
            s.store_div_from_scalar_offset_ad(2083, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::mul(s.ad_value(2091), A::add(s.ad_value(2092), s.ad_value(725))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::mul(s.ad_value(2091), A::add(s.ad_value(2092), s.ad_value(725))), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if s.b[2340] {
            s.store_mul_add_rhs(2084, 2077, 1857, 2073);
        }

        s.b[2344] = ((s.v[2083] < 0.001) && (s.v[1857] < 1e-6));
        s.v[2344] = if s.b[2344] { 1.0 } else { 0.0 };

        s.b[2345] = (((-s.v[2084]) + s.v[2082]) > (-230.25850929940458));
        s.v[2345] = if s.b[2345] { 1.0 } else { 0.0 };

        if ((s.b[2340] && s.b[2344]) && s.b[2345]) {
            s.store_exp_sub(2027, 2082, 2084);
        }

        if ((s.b[2340] && s.b[2344]) && (!s.b[2345])) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(2082), s.ad_value(2084)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(2082), s.ad_value(2084)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (s.b[2340] && s.b[2344]) {
            s.store_mul_offset_rhs(1881, 2083, 2027, (-1.0));
            s.store_add(2085, 1881, 2083);
        }

        if (s.b[2340] && (!s.b[2344])) {
            s.store_add(2087, 2080, 2084);
            s.store_ad_value(2088, A::add_scaled_product(s.ad_value(2087), 1.0, s.ad_value(761), A::sqrt(s.ad_value(2087)), 1.0));
            s.store_add(2089, 2088, 2081);
            s.store_offset_ad(2090, A::div_scaled_inputs(s.ad_value(761), 1.0, A::sqrt(s.ad_value(2087)), 2.0), 1.0);
            s.store_div_from_scalar(2091, 1.0, 2090);
            s.store_sub(2092, 1880, 2089);
        }

        s.b[2346] = (s.v[2092] > (-12.0));
        s.v[2346] = if s.b[2346] { 1.0 } else { 0.0 };

        if ((s.b[2340] && (!s.b[2344])) && s.b[2346]) {
            s.store_offset_add(2093, 2092, 725, (-1.0));
            s.store_scaled_add_ad_rhs(2094, 2093, A::sqrt(A::offset(A::square(s.ad_value(2093)), 10.0)), 0.5);
            s.store_add_ad_lhs(2095, A::add_scaled_product(s.ad_value(2092), 1.0, s.ad_value(2090), A::ln(s.ad_value(2094)), (-1.0)), 725);
            s.store_scaled_add_ad_rhs(2096, 2095, A::sqrt(A::offset(A::square(s.ad_value(2095)), 2.0)), 0.5);
        }

        s.b[2347] = ((s.v[2092] - s.v[2096]) < 230.25850929940458);
        s.v[2347] = if s.b[2347] { 1.0 } else { 0.0 };

        if (((s.b[2340] && (!s.b[2344])) && s.b[2346]) && s.b[2347]) {
            s.store_exp_sub(2097, 2092, 2096);
        }

        if (((s.b[2340] && (!s.b[2344])) && s.b[2346]) && (!s.b[2347])) {
            s.store_scaled_offset_ad(2097, A::mul_offset_lhs(A::sub(s.ad_value(2092), s.ad_value(2096)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2092), s.ad_value(2096)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2092), s.ad_value(2096)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2340] && (!s.b[2344])) && s.b[2346]) {
            s.store_mul(2098, 724, 2097);
            s.store_pow_ad(2099, s.ad_value(2098), s.ad_value(2091));
            s.store_ad_value(2100, A::add_scaled_square_product(s.ad_value(2090), 1.0, A::add_scaled_inputs3(s.ad_value(2096), 2.0, s.ad_value(2090), 2.0, s.ad_value(2099), -1.0), s.ad_value(2099), 1.0));
            s.store_mul_offset_ad_rhs(2101, 2090, A::div(A::sub(A::sqrt(s.ad_value(2100)), s.ad_value(2090)), s.ad_value(2099)), (-1.0));
            s.store_sub(2085, 2096, 2101);
        }

        s.b[2348] = ((s.v[2091] * (s.v[2092] + s.v[725])) > (-230.25850929940458));
        s.v[2348] = if s.b[2348] { 1.0 } else { 0.0 };

        if (((s.b[2340] && (!s.b[2344])) && (!s.b[2346])) && s.b[2348]) {
            s.store_exp_ad(2085, A::mul(s.ad_value(2091), A::add(s.ad_value(2092), s.ad_value(725))));
        }

        if (((s.b[2340] && (!s.b[2344])) && (!s.b[2346])) && (!s.b[2348])) {
            s.store_div_from_scalar_offset_ad(2085, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::mul(s.ad_value(2091), A::add(s.ad_value(2092), s.ad_value(725))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::mul(s.ad_value(2091), A::add(s.ad_value(2092), s.ad_value(725))), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (s.b[2340] && (!s.b[2344])) {
            s.store_sub(1881, 2085, 2083);
        }

        if s.b[2340] {
            s.store_scaled_add(1882, 2085, 2083, 0.5);
        }

        if s.b[2340] {
            s.store_ad_value(1883, {
                if ((s.v[1880] - s.v[1882]) > 1e-40) {
                    A::sub(s.ad_value(1880), s.ad_value(1882))
                } else {
                    A::constant(1e-40)
                }
            });
        }

        if s.b[2340] {
            s.store_sub_from_scalar_ad(1884, 1.0, A::div_scaled_inputs(s.ad_value(761), 0.5, A::sqrt(A::add_scaled_inputs(s.ad_value(1883), 1.0, s.ad_value(724), 0.25)), 1.0));
            s.store_ad_value(846, A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(717), s.ad_value(2076), s.ad_value(2076), -1.0), A::offset(A::mul(s.ad_value(1884), s.ad_value(1882)), 1.0), s.ad_value(1881), 1.0, s.ad_value(1869), 1.0));
        }

        s.v[1885] = 0.0;

        s.v[847] = 0.0;

        s.b[2349] = ((s.v[1829] > 0.0) && (p.p41 != 0.0));
        s.v[2349] = if s.b[2349] { 1.0 } else { 0.0 };

        if s.b[2349] {
            s.store_ad_value(2086, A::add_scaled_product(s.ad_value(826), 1.0, s.ad_value(232), s.ad_value(1860), (-1.0)));
        }

        s.b[2350] = (s.v[2086] > 0.0);
        s.v[2350] = if s.b[2350] { 1.0 } else { 0.0 };

        if (s.b[2349] && s.b[2350]) {
            s.store_mul_div_ad_rhs(2029, 712, A::offset(A::mul(s.ad_value(233), A::sub(A::sqrt(A::add(s.ad_value(728), s.ad_value(2030))), s.ad_value(736))), 1.0), A::offset(s.ad_value(2086), 1e-30));
        }

        s.b[2351] = ((((-s.v[2029])) as f64).abs() < 230.25850929940458);
        s.v[2351] = if s.b[2351] { 1.0 } else { 0.0 };

        if ((s.b[2349] && s.b[2350]) && s.b[2351]) {
            s.store_exp_neg_input(2027, 2029);
        }

        s.b[2352] = ((-s.v[2029]) < 0.0);
        s.v[2352] = if s.b[2352] { 1.0 } else { 0.0 };

        if (((s.b[2349] && s.b[2350]) && (!s.b[2351])) && s.b[2352]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(2029)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(2029)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[2349] && s.b[2350]) && (!s.b[2351])) && (!s.b[2352])) {
            s.store_scaled_offset_ad(2027, A::mul_offset_lhs(A::neg(s.ad_value(2029)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(2029)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(2029)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (s.b[2349] && s.b[2350]) {
            s.store_mul3_lhs(1885, 229, 2086, 2027);
            s.store_mul_add_rhs(847, 1885, 838, 846);
        }

        s.b[2353] = (s.v[847] > (0.5 * s.v[234]));
        s.v[2353] = if s.b[2353] { 1.0 } else { 0.0 };

        if ((s.b[2349] && s.b[2350]) && s.b[2353]) {
            s.store_offset_ad(2027, A::div_scaled_inputs(s.ad_value(847), 2.0, s.ad_value(234), 1.0), (-1.0));
            s.store_mul_scaled_ad_rhs(847, 234, 0.5, A::offset(A::div(s.ad_value(2027), A::sqrt(A::offset(A::square(s.ad_value(2027)), 1.0))), 1.0));
        }

        s.b[2547] = (((p.p45 == 1.0) || (p.p47 > 0.0)) || (p.p48 > 0.0));
        s.v[2547] = if s.b[2547] { 1.0 } else { 0.0 };

        s.b[2548] = ((p.p45 > 0.0) || (p.p47 > 0.0));
        s.v[2548] = if s.b[2548] { 1.0 } else { 0.0 };

        if (s.b[2547] && s.b[2548]) {
            s.copy_ad(2388, 728);
            s.copy_ad(2389, 738);
            s.copy_ad(2390, 729);
            s.copy_ad(2391, 1820);
            s.copy_ad(2392, 1821);
            s.store_scalar(2396, 0.0);
        }

        s.b[2549] = (p.p47 > 0.0);
        s.v[2549] = if s.b[2549] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2548]) && s.b[2549]) {
            s.store_add_ad_lhs(2391, A::add_scaled_inputs3(s.ad_value(828), 0.5, s.ad_value(827), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(749), 1.0, A::sub(s.ad_value(828), s.ad_value(827)), A::sub(s.ad_value(828), s.ad_value(827)), 1.0)), (-0.5)), 747);
            s.store_add_ad_lhs(1886, A::add_scaled_inputs3(s.ad_value(827), 1.0, s.ad_value(2391), (-0.5), A::sqrt(A::add_scaled_product(s.ad_value(748), 1.0, s.ad_value(2391), s.ad_value(2391), 1.0)), (-(-0.5))), 750);
            s.copy_ad(2392, 1886);
            s.copy_ad(2388, 745);
            s.copy_ad(2389, 748);
            s.copy_ad(2390, 746);
        }

        if (s.b[2547] && s.b[2548]) {
            s.store_ad_value(2395, A::add_scaled_inputs3(s.ad_value(829), 1.0, s.ad_value(2396), (-1.0), s.ad_value(700), -1.0));
            s.store_ad_value(2397, A::add_scaled_inputs3(s.ad_value(2392), 1.0, s.ad_value(826), 0.5, s.ad_value(830), (-0.5)));
            s.store_scalar(2409, 1.0);
        }

        s.b[2550] = (s.v[190] > 0.0);
        s.v[2550] = if s.b[2550] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2548]) && s.b[2550]) {
            s.store_scale(2400, 2388, s.v[361]);
            s.store_scale(2401, 2397, s.v[361]);
            s.store_scale(2402, 2395, s.v[361]);
            s.store_offset_ad(2028, A::div_scaled_inputs(s.ad_value(2390), 0.5, A::sqrt(s.ad_value(2400)), 1.0), 1.0);
            s.store_ad_value(2029, A::add_scaled_product(s.ad_value(2400), 1.0, s.ad_value(2390), A::sqrt(s.ad_value(2400)), 1.0));
            s.store_ad_value(2403, A::add_scaled_inputs_product(A::div(A::sub(s.ad_value(2402), s.ad_value(2029)), s.ad_value(2028)), 1.0, s.ad_value(2400), 0.5, A::offset(s.ad_value(191), 1.0), s.ad_value(2401), (-1.0)));
            s.store_offset_scaled(2404, 2400, 0.5, 2.0);
            s.store_add(2405, 2400, 2401);
            s.store_ad_value(2028, A::sub_scaled_inputs(A::add_scaled_inputs_product(s.ad_value(2402), 1.0, s.ad_value(2405), (-1.0), s.ad_value(2390), A::sqrt(s.ad_value(2405)), (-1.0)), 1.0, A::ln(A::add(A::div(s.ad_value(2400), s.ad_value(2390)), A::sqrt(s.ad_value(2400)))), 2.0));
            s.store_add_scaled_inputs(2406, 2028, 2.0, 2404, 1.0);
            s.store_ad_value(2028, A::add_scaled_inputs3(s.ad_value(2403), 0.5, s.ad_value(2406), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2403), s.ad_value(2406)), A::sub(s.ad_value(2403), s.ad_value(2406))), 20.0)), 0.5));
            s.store_ad_value(2029, A::add_scaled_inputs3(s.ad_value(2402), 2.0, s.ad_value(2401), (-2.0), s.ad_value(2404), -1.0));
            s.store_ad_value(2407, A::add_scaled_inputs3(s.ad_value(2028), 0.5, s.ad_value(2029), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2028), s.ad_value(2029)), A::sub(s.ad_value(2028), s.ad_value(2029))), 20.0)), (-0.5)));
            s.store_ad_value(2028, A::add_scaled_inputs3(s.ad_value(2407), 0.5, s.ad_value(2404), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2407), s.ad_value(2404)), A::sub(s.ad_value(2407), s.ad_value(2404))), 5.0)), (-0.5)));
            s.store_ad_value(2408, A::add_scaled_inputs3(s.ad_value(2028), 0.5, s.ad_value(2404), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub_scaled_inputs(s.ad_value(2028), 1.0, s.ad_value(2404), -1.0), A::sub_scaled_inputs(s.ad_value(2028), 1.0, s.ad_value(2404), -1.0)), 20.0)), 0.5));
            s.store_mul_offset_ad_rhs(2029, 702, A::div(s.ad_value(2408), s.ad_value(2404)), 1.0);
        }

        s.b[2551] = (s.v[2029] > (-230.25850929940458));
        s.v[2551] = if s.b[2551] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2548]) && s.b[2550]) && s.b[2551]) {
            s.store_exp(2409, 2029);
        }

        if (((s.b[2547] && s.b[2548]) && s.b[2550]) && (!s.b[2551])) {
            s.store_div_from_scalar_offset_ad(2409, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2029), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2029), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (s.b[2547] && s.b[2548]) {
            s.store_offset_mul(2410, 701, 2409, 1.0);
            s.store_scale(2411, 2410, s.v[715]);
            s.store_mul_ad_product_rhs(2412, 199, A::offset(A::mul(s.ad_value(201), s.ad_value(830)), 1.0), A::offset(A::mul(s.ad_value(200), s.ad_value(2397)), 1.0));
            s.store_mul_offset_rhs(2413, 2411, 2412, 1.0);
            s.store_div_from_scalar(2414, 1.0, 2413);
            s.store_mul_ad_rhs(2398, 2390, A::sqrt_scaled_input(s.ad_value(2414), s.v[715]));
            s.store_square(2399, 2398);
            s.store_div_from_scalar(2415, 1.0, 2399);
            s.store_mul(2416, 2392, 2414);
            s.store_mul(2417, 2395, 2414);
            s.store_ad_value(2418, A::div_scaled_inputs(s.ad_value(830), 2.0, A::offset(A::sqrt(A::offset(A::mul(s.ad_value(197), s.ad_value(830)), 1.0)), 1.0), 1.0));
            s.store_mul_ad_product_rhs(2419, 196, s.ad_value(2418), A::offset(A::mul(s.ad_value(198), s.ad_value(2397)), 1.0));
            s.store_mul(2420, 2388, 2414);
            s.store_sqrt_square_add(2028, 2391, 2389);
            s.store_sqrt_ad(2029, A::add_scaled_product(s.ad_value(2389), 1.0, A::sub(s.ad_value(2391), s.ad_value(2419)), A::sub(s.ad_value(2391), s.ad_value(2419)), 1.0));
            s.store_mul_scaled_ad_rhs(2421, 2414, 0.5, A::add_scaled_inputs3(s.ad_value(2419), 1.0, s.ad_value(2028), 1.0, s.ad_value(2029), -1.0));
            s.store_add(2422, 2420, 2416);
            s.store_sub(2423, 2422, 2421);
        }

    }

    pub(super) fn stamp_transient_block_34(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2552] = (p.p45 > 0.0);
        s.v[2552] = if s.b[2552] { 1.0 } else { 0.0 };

        s.b[2553] = (((s.v[2423]) as f64).abs() < 1e-5);
        s.v[2553] = if s.b[2553] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2548]) && s.b[2552]) && s.b[2553]) {
            s.store_offset_ad(2424, A::mul_sub_from_scalar_rhs(s.ad_value(2398), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2423), 1.0, A::scale(s.ad_value(2423), 0.3125), 0.5)), 1.0);
        }

        s.b[2554] = (s.v[2423] < 460.51701859880916);
        s.v[2554] = if s.b[2554] { 1.0 } else { 0.0 };

        if ((((s.b[2547] && s.b[2548]) && s.b[2552]) && (!s.b[2553])) && s.b[2554]) {
            s.store_exp_neg_input(2438, 2423);
        }

        if ((((s.b[2547] && s.b[2548]) && s.b[2552]) && (!s.b[2553])) && (!s.b[2554])) {
            s.store_div_from_scalar_offset_ad(2438, 1e-200, A::mul_offset_lhs(s.ad_value(2423), (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2423), (-460.51701859880916), A::scale_offset(s.ad_value(2423), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        if (((s.b[2547] && s.b[2548]) && s.b[2552]) && (!s.b[2553])) {
            s.store_scalar(2027, (if (s.v[2423] > 0.0) { 1.0 } else { (-1.0) }));
        }

        if (((s.b[2547] && s.b[2548]) && s.b[2552]) && (!s.b[2553])) {
            s.store_offset_ad(2424, A::div_scaled_product3(s.ad_value(2027), s.ad_value(2398), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(2438), 1.0, s.ad_value(2423))), 1.0, A::sqrt(A::mul_sub_from_scalar_rhs(s.ad_value(2423), 1.0, s.ad_value(2438))), 2.0), 1.0);
        }

        if ((s.b[2547] && s.b[2548]) && (!s.b[2552])) {
            s.store_offset_ad(2424, A::div_scaled_inputs(s.ad_value(2398), 0.5, A::sqrt(s.ad_value(2423)), 1.0), 1.0);
        }

        if (s.b[2547] && s.b[2548]) {
            s.store_ad_value(2425, A::add_scaled_product(A::add_scaled_product(s.ad_value(2423), 1.0, s.ad_value(2398), A::sqrt(s.ad_value(2423)), 1.0), 1.0, s.ad_value(2424), A::ln(A::offset(s.ad_value(2424), (-1.0))), (-1.0)));
            s.store_div_ad_lhs(2426, A::sub(s.ad_value(2417), s.ad_value(2425)), 2424);
            s.store_mul_scaled_ad_rhs(2432, 2399, 0.5, A::offset(A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2399)), 1.0)), (-1.0)));
            s.store_scalar(2431, 0.0);
            s.store_scalar(2433, 1.0);
        }

        s.b[2555] = (s.v[2426] > (-30.0));
        s.v[2555] = if s.b[2555] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2548]) && s.b[2555]) {
            s.store_offset_mul(2427, 2424, 2426, (-1.0));
            s.store_scaled_add_ad_rhs(2027, 2427, A::sqrt(A::offset(A::square(s.ad_value(2427)), 10.0)), 0.5);
            s.store_sub_ad_rhs(2428, 2426, A::ln(s.ad_value(2027)));
            s.store_scaled_add_ad_rhs(2429, 2428, A::sqrt(A::offset(A::square(s.ad_value(2428)), 2.0)), 0.5);
        }

        s.b[2556] = ((s.v[2426] - s.v[2429]) < 230.25850929940458);
        s.v[2556] = if s.b[2556] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2548]) && s.b[2555]) && s.b[2556]) {
            s.store_exp_sub(2027, 2426, 2429);
        }

        if (((s.b[2547] && s.b[2548]) && s.b[2555]) && (!s.b[2556])) {
            s.store_scaled_offset_ad(2027, A::mul_offset_lhs(A::sub(s.ad_value(2426), s.ad_value(2429)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2426), s.ad_value(2429)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2426), s.ad_value(2429)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2547] && s.b[2548]) && s.b[2555]) {
            s.store_div(2430, 2027, 2424);
            s.store_sub_ad_lhs(2027, A::scaled_offset(s.ad_value(2429), 1.0, 2.0), 2430);
        }

        s.b[2557] = (s.v[2430] > 1e-6);
        s.v[2557] = if s.b[2557] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2548]) && s.b[2555]) && s.b[2557]) {
            s.store_mul_offset_ad_rhs(2431, 2424, A::sub(s.ad_value(2429), A::div(A::offset(A::sqrt(A::offset(A::mul(s.ad_value(2430), s.ad_value(2027)), 1.0)), (-1.0)), s.ad_value(2430))), 1.0);
        }

        if (((s.b[2547] && s.b[2548]) && s.b[2555]) && (!s.b[2557])) {
            s.store_mul_ad_affine_product_rhs(2431, 2424, s.ad_value(2430), A::offset(A::mul_scaled_lhs(s.ad_value(2027), 0.25, s.ad_value(2027)), 1.0), 0.5, 0.0);
        }

        if ((s.b[2547] && s.b[2548]) && s.b[2555]) {
            s.store_ad_value(2027, A::add_scaled_inputs3_offset(s.ad_value(2417), 0.5, s.ad_value(2431), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul_offset_lhs(A::sub(s.ad_value(2417), s.ad_value(2431)), (-2.0), A::offset(A::sub(s.ad_value(2417), s.ad_value(2431)), (-2.0))), 1.0)), 0.5, (2.0 * 0.5)));
            s.store_mul_scaled_ad_rhs(2432, 2399, 0.5, A::offset(A::sqrt(A::offset(A::mul(A::div_from_scalar(4.0, s.ad_value(2399)), s.ad_value(2027)), 1.0)), (-1.0)));
            s.store_div_ad_rhs(2433, 2432, A::add(s.ad_value(2432), s.ad_value(2431)));
            s.store_ad_value(2423, A::add_scaled_product(s.ad_value(2422), 1.0, s.ad_value(2433), s.ad_value(2421), (-1.0)));
        }

        if (s.b[2547] && s.b[2548]) {
            s.store_offset_scaled(2434, 2398, 0.7071067811865475, 1.0);
            s.store_scale(2435, 2434, 1e-5);
            s.store_div_from_scalar(2436, 1.0, 2434);
            s.store_scalar(2543, 0.0);
            s.store_scalar(2437, 0.0);
        }

        s.b[2558] = (s.v[2423] < 460.51701859880916);
        s.v[2558] = if s.b[2558] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2548]) && s.b[2558]) {
            s.store_exp_neg_input(2438, 2423);
        }

        if ((s.b[2547] && s.b[2548]) && (!s.b[2558])) {
            s.store_div_from_scalar_offset_ad(2438, 1e-200, A::mul_offset_lhs(s.ad_value(2423), (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2423), (-460.51701859880916), A::scale_offset(s.ad_value(2423), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        s.b[2559] = (((s.v[2417]) as f64).abs() <= s.v[2435]);
        s.v[2559] = if s.b[2559] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2548]) && s.b[2559]) {
            s.store_scaled_square(2523, 2436, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(2437, 2417, s.ad_value(2436), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2417), 1.0, s.ad_value(2438)), s.ad_value(2398), s.ad_value(2523)), 1.0));
        }

        s.b[2560] = (s.v[2417] < (-s.v[2435]));
        s.v[2560] = if s.b[2560] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && s.b[2560]) {
            s.store_neg(2525, 2417);
            s.store_scaled_mul(2526, 2525, 2436, 1.25);
            s.store_scaled_sub_ad(2527, A::offset(s.ad_value(2526), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(2526), (-6.0), A::offset(s.ad_value(2526), (-6.0))), 64.0)), 0.5);
            s.store_sub(2522, 2525, 2527);
            s.store_ad_value(2528, A::add_scaled_square_product(s.ad_value(2522), 1.0, s.ad_value(2399), A::offset(s.ad_value(2527), 1.0), 1.0));
            s.store_sub_scaled_inputs(2529, 2522, 2.0, 2399, 1.0);
            s.store_sub_ad_lhs(2530, A::ln(A::mul(s.ad_value(2528), s.ad_value(2415))), 2527);
            s.store_add(824, 2528, 2529);
            s.store_ad_value(823, A::add_scaled_square_product(s.ad_value(824), 1.0, s.ad_value(2530), A::sub_scaled_inputs(A::square(s.ad_value(2529)), 0.5, s.ad_value(2528), 1.0), 1.0));
            s.store_add_ad_rhs(2531, 2527, A::div_scaled_product3(s.ad_value(2528), s.ad_value(824), s.ad_value(2530), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2530), s.ad_value(2530)), s.ad_value(2529), A::sub_scaled_inputs(A::square(s.ad_value(2529)), 0.3333333333333333, s.ad_value(2528), 1.0))), 1.0));
        }

        s.b[2561] = (s.v[2531] < 230.25850929940458);
        s.v[2561] = if s.b[2561] { 1.0 } else { 0.0 };

        if ((((s.b[2547] && s.b[2548]) && (!s.b[2559])) && s.b[2560]) && s.b[2561]) {
            s.store_exp(2532, 2531);
        }

        if ((((s.b[2547] && s.b[2548]) && (!s.b[2559])) && s.b[2560]) && (!s.b[2561])) {
            s.store_scaled_offset_ad(2532, A::mul_offset_lhs(s.ad_value(2531), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2531), (-230.25850929940458), A::scale_offset(s.ad_value(2531), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && s.b[2560]) {
            s.store_div_from_scalar(2533, 1.0, 2532);
            s.store_div_from_scalar_offset_ad(2522, 1.0, A::square(s.ad_value(2531)), 2.0);
            s.store_mul_square_lhs(2534, 2531, 2522);
            s.store_mul3_affine_lhs(2535, 2531, 2522, 4.0, 0.0, 2522);
            s.store_mul_ad_product_lhs(2536, A::sub_scaled_inputs(s.ad_value(2522), 8.0, s.ad_value(2534), 12.0), s.ad_value(2522), 2522);
            s.store_sub(2522, 2525, 2531);
            s.store_mul(2523, 2438, 2533);
            s.store_ad_value(2537, A::add_scaled_product(s.ad_value(2522), 2.0, s.ad_value(2399), A::add_scaled_inputs3_offset(s.ad_value(2532), 1.0, s.ad_value(2523), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(2438), 1.0, s.ad_value(2535)), 1.0, (-1.0)), 1.0));
            s.store_ad_value(2538, A::add_scaled_square_product(s.ad_value(2522), 1.0, s.ad_value(2399), A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2532), 1.0, s.ad_value(2531), (-1.0), s.ad_value(2523), 1.0, (-1.0)), 1.0, s.ad_value(2438), A::sub(A::offset(s.ad_value(2531), (-1.0)), s.ad_value(2534)), 1.0), (-1.0)));
            s.store_sub_from_scalar_ad(2522, 2.0, A::mul(s.ad_value(2399), A::add_scaled_inputs_product(s.ad_value(2532), 1.0, s.ad_value(2523), 1.0, s.ad_value(2438), s.ad_value(2536), (-1.0))));
            s.store_ad_value(2522, A::add_scaled_square_product(s.ad_value(2537), 1.0, s.ad_value(2538), s.ad_value(2522), (-2.0)));
            s.store_ad_value(2437, A::sub_scaled_inputs(s.ad_value(2531), -1.0, A::div(s.ad_value(2538), A::add(s.ad_value(2537), A::sqrt(s.ad_value(2522)))), 2.0));
        }

        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) {
            s.store_div_from_scalar_offset_scaled_input(2539, 1.0, 2398, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(2540, A::mul_scaled_lhs(s.ad_value(2434), 1.25, s.ad_value(2539)), (-1.0), 2539);
            s.store_mul_ad_product_rhs(2541, 2417, s.ad_value(2436), A::offset(A::mul(s.ad_value(2540), s.ad_value(2417)), 1.0));
        }

        s.b[2562] = ((-s.v[2541]) > (-230.25850929940458));
        s.v[2562] = if s.b[2562] { 1.0 } else { 0.0 };

        if ((((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) && s.b[2562]) {
            s.store_exp_neg_input(2522, 2541);
        }

        if ((((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) && (!s.b[2562])) {
            s.store_div_from_scalar_offset_ad(2522, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(2541)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(2541)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) {
            s.store_sub_from_scalar(2542, 1.0, 2522);
            s.store_ad_value(2543, A::add_scaled_inputs_product(s.ad_value(2417), 1.0, s.ad_value(2399), 0.5, s.ad_value(2398), A::sqrt(A::add_scaled_inputs3(s.ad_value(2417), 1.0, s.ad_value(2399), 0.25, s.ad_value(2542), -1.0)), (-1.0)));
            s.store_offset(2544, 2423, 3.0);
            s.store_ad_value(2527, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(2543), 0.5, s.ad_value(2544), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2543), s.ad_value(2544)), A::sub(s.ad_value(2543), s.ad_value(2544))), 5.0)), (-0.5)), 1.0, s.ad_value(2544), (-0.5), A::sqrt(A::offset(A::square(s.ad_value(2544)), 5.0)), (-(-0.5))));
            s.store_sub(2522, 2417, 2527);
            s.store_exp_neg_input(2523, 2527);
            s.store_div_from_scalar_offset_ad(2524, 1.0, A::square(s.ad_value(2527)), 2.0);
            s.store_mul_square_lhs(2534, 2527, 2524);
            s.store_mul3_affine_lhs(2535, 2527, 2524, 4.0, 0.0, 2524);
            s.store_mul_ad_product_lhs(2536, A::sub_scaled_inputs(s.ad_value(2524), 8.0, s.ad_value(2534), 12.0), s.ad_value(2524), 2524);
        }

        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) {
            let assign49800_ad_e64198: A = {
                if (1e-40 > ((s.v[2522] * s.v[2522]) - (s.v[2399] * (((s.v[2523] + s.v[2527]) - 1.0) - (s.v[2438] * ((s.v[2527] + 1.0) + s.v[2534])))))) {
                    A::constant(1e-40)
                } else {
                    A::add_scaled_square_product(s.ad_value(2522), 1.0, s.ad_value(2399), A::add_scaled_product(A::offset(A::add(s.ad_value(2523), s.ad_value(2527)), (-1.0)), 1.0, s.ad_value(2438), A::add(A::offset(s.ad_value(2527), 1.0), s.ad_value(2534)), (-1.0)), (-1.0))
                }
            };
            s.store_ad_value(2528, assign49800_ad_e64198);
        }

        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) {
            s.store_sub_from_scalar_ad(2545, 1.0, A::mul_scaled_output(s.ad_value(2399), A::add_scaled_product(s.ad_value(2523), 1.0, s.ad_value(2438), s.ad_value(2536), (-1.0)), 0.5));
            s.store_ad_value(2529, A::add_scaled_product(s.ad_value(2522), 2.0, s.ad_value(2399), A::add_scaled_sub_value_product(1.0, s.ad_value(2523), 1.0, s.ad_value(2438), A::offset(s.ad_value(2535), 1.0), (-1.0)), 1.0));
            s.store_ad_value(2530, A::add_scaled_inputs3(s.ad_value(2423), 1.0, s.ad_value(2527), (-1.0), A::ln(A::div(s.ad_value(2528), s.ad_value(2399))), 1.0));
            s.store_add(824, 2528, 2529);
            s.store_ad_value(823, A::add_scaled_square_product(s.ad_value(824), 1.0, s.ad_value(2530), A::add_scaled_square_product(s.ad_value(2529), 0.5, s.ad_value(2528), s.ad_value(2545), (-1.0)), 1.0));
            s.store_add_ad_rhs(2546, 2527, A::div_scaled_product3(s.ad_value(2528), s.ad_value(824), s.ad_value(2530), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2530), s.ad_value(2530)), s.ad_value(2529), A::add_scaled_square_product(s.ad_value(2529), 0.3333333333333333, s.ad_value(2528), s.ad_value(2545), (-1.0)))), 1.0));
        }

        s.b[2563] = (s.v[2546] < 230.25850929940458);
        s.v[2563] = if s.b[2563] { 1.0 } else { 0.0 };

        if ((((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) && s.b[2563]) {
            s.store_exp(2532, 2546);
            s.store_div_from_scalar(2533, 1.0, 2532);
            s.store_mul(2532, 2438, 2532);
        }

        s.b[2564] = (s.v[2546] > (s.v[2423] - 230.25850929940458));
        s.v[2564] = if s.b[2564] { 1.0 } else { 0.0 };

        if (((((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) && (!s.b[2563])) && s.b[2564]) {
            s.store_exp_sub(2532, 2546, 2423);
            s.store_div(2533, 2438, 2532);
        }

        if (((((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) && (!s.b[2563])) && (!s.b[2564])) {
            s.store_div_from_scalar_offset_ad(2532, 1e-100, A::mul_offset_lhs(A::sub(s.ad_value(2423), s.ad_value(2546)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2423), s.ad_value(2546)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2423), s.ad_value(2546)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(2533, 1e-100, A::mul_offset_lhs(s.ad_value(2546), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2546), (-230.25850929940458), A::scale_offset(s.ad_value(2546), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && (!s.b[2560])) {
            s.store_div_from_scalar_offset_ad(2522, 1.0, A::square(s.ad_value(2546)), 2.0);
            s.store_mul_square_lhs(2534, 2546, 2522);
            s.store_mul3_affine_lhs(2535, 2546, 2522, 4.0, 0.0, 2522);
            s.store_mul_ad_product_lhs(2536, A::sub_scaled_inputs(s.ad_value(2522), 8.0, s.ad_value(2534), 12.0), s.ad_value(2522), 2522);
            s.store_sub(2522, 2417, 2546);
            s.store_ad_value(2537, A::add_scaled_product(s.ad_value(2522), 2.0, s.ad_value(2399), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2533)), 1.0, s.ad_value(2532), 1.0, s.ad_value(2438), A::offset(s.ad_value(2535), 1.0), (-1.0)), 1.0));
            s.store_ad_value(2538, A::add_scaled_square_product(s.ad_value(2522), 1.0, s.ad_value(2399), A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2533), 1.0, s.ad_value(2546), 1.0, s.ad_value(2532), 1.0, (-1.0)), 1.0, s.ad_value(2438), A::add(A::offset(s.ad_value(2546), 1.0), s.ad_value(2534)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_ad(2522, 2.0, A::mul(s.ad_value(2399), A::add_scaled_inputs_product(s.ad_value(2533), 1.0, s.ad_value(2532), 1.0, s.ad_value(2438), s.ad_value(2536), (-1.0))));
            s.store_ad_value(2522, A::add_scaled_square_product(s.ad_value(2537), 1.0, s.ad_value(2538), s.ad_value(2522), (-2.0)));
            s.store_ad_value(2437, A::add_scaled_inputs(s.ad_value(2546), 1.0, A::div(s.ad_value(2538), A::add(s.ad_value(2537), A::sqrt(s.ad_value(2522)))), 2.0));
        }

        if (s.b[2547] && s.b[2548]) {
            s.store_scalar(2440, 0.0);
            s.store_scalar(2441, 0.0);
            s.store_scalar(2442, 0.0);
            s.store_scalar(2443, 0.0);
            s.store_scalar(2444, 0.0);
            s.store_scalar(2445, 0.0);
            s.store_scalar(2446, 0.0);
            s.store_scalar(2447, 1.0);
            s.store_scalar(2448, 1.0);
            s.store_sub(2449, 2417, 2437);
            s.store_scalar(2450, 0.0);
            s.store_mul(2451, 2413, 2449);
            s.store_scalar(2452, 1.0);
            s.store_scalar(2453, 1.0);
            s.store_scalar(2457, 1.0);
            s.store_scalar(2458, 1.0);
            s.store_scalar(2460, 1.0);
        }

        s.b[2565] = (s.v[2417] > 0.0);
        s.v[2565] = if s.b[2565] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2548]) && s.b[2565]) {
            s.store_div_from_scalar_offset_ad(2027, 1.0, A::square(s.ad_value(2437)), 2.0);
        }

    }

    pub(super) fn stamp_transient_block_35(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[2547] && s.b[2548]) && s.b[2565]) {
            s.store_mul_square_lhs(2439, 2437, 2027);
            s.store_mul3_affine_lhs(2440, 2437, 2027, 4.0, 0.0, 2027);
            s.store_mul_ad_product_lhs(2441, A::sub_scaled_inputs(s.ad_value(2027), 8.0, s.ad_value(2439), 12.0), s.ad_value(2027), 2027);
            s.store_scalar(2442, 0.0);
        }

        s.b[2566] = (s.v[2437] < 230.25850929940458);
        s.v[2566] = if s.b[2566] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2566]) {
            s.store_exp(2442, 2437);
            s.store_div_from_scalar(2443, 1.0, 2442);
            s.store_mul(2442, 2438, 2442);
        }

        s.b[2567] = (s.v[2437] > (s.v[2423] - 230.25850929940458));
        s.v[2567] = if s.b[2567] { 1.0 } else { 0.0 };

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && (!s.b[2566])) && s.b[2567]) {
            s.store_exp_sub(2442, 2437, 2423);
            s.store_div(2443, 2438, 2442);
        }

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && (!s.b[2566])) && (!s.b[2567])) {
            s.store_div_from_scalar_offset_ad(2442, 1e-100, A::mul_offset_lhs(A::sub(s.ad_value(2423), s.ad_value(2437)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2423), s.ad_value(2437)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2423), s.ad_value(2437)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(2443, 1e-100, A::mul_offset_lhs(s.ad_value(2437), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2437), (-230.25850929940458), A::scale_offset(s.ad_value(2437), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        if ((s.b[2547] && s.b[2548]) && s.b[2565]) {
            s.store_ad_value(2444, A::add_scaled_product(s.ad_value(2442), 1.0, s.ad_value(2438), A::add(A::offset(s.ad_value(2437), 1.0), s.ad_value(2439)), (-1.0)));
        }

        s.b[2568] = (s.v[2437] < 1e-5);
        s.v[2568] = if s.b[2568] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2568]) {
            s.store_ad_value(2445, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(2437)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2437), 1.0, A::scale(s.ad_value(2437), 0.25), 0.3333333333333333), 0.5));
            s.store_ad_value(2444, A::mul3_scaled_output(A::mul3(s.ad_value(2438), s.ad_value(2437), s.ad_value(2437)), s.ad_value(2437), A::scale_offset(s.ad_value(2437), 1.75, 1.0), 0.16666666666666666));
            s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2437), 1.0, A::scale(s.ad_value(2437), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2446, 2437, 2027, 0.7071067811865475);
            s.store_offset_ad(2447, A::div_scaled_product(s.ad_value(2398), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2437), 0.5)), 1.0, A::square(s.ad_value(2437)), 0.16666666666666666), 0.7071067811865475, s.ad_value(2027), 1.0), 1.0);
        }

        if (((s.b[2547] && s.b[2548]) && s.b[2565]) && (!s.b[2568])) {
            s.store_add_ad_lhs(2445, A::offset(s.ad_value(2437), (-1.0)), 2443);
            s.store_sqrt(2446, 2445);
            s.store_offset_scaled_ad(2447, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2398), 1.0, s.ad_value(2443)), s.ad_value(2446)), 0.5, 1.0);
        }

        if ((s.b[2547] && s.b[2548]) && s.b[2565]) {
            s.store_div_ad(2448, A::offset(A::mul_scaled_lhs(s.ad_value(708), 0.2, s.ad_value(2397)), 1.0), A::offset(A::mul(s.ad_value(708), s.ad_value(2397)), 1.0));
        }

        s.b[2569] = (s.v[2444] > 1e-100);
        s.v[2569] = if s.b[2569] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) {
            s.store_mul_sqrt_ad_rhs(2449, 2398, A::add(s.ad_value(2445), s.ad_value(2444)));
            s.store_ad_value(2450, A::div_scaled_product3(s.ad_value(2399), s.ad_value(2444), s.ad_value(2413), 1.0, A::add_scaled_product(s.ad_value(2449), 1.0, s.ad_value(2398), s.ad_value(2446), 1.0), 1.0));
            s.store_mul3_lhs(2451, 2446, 2398, 2413);
        }

        s.b[2570] = (s.v[217] < 0.0);
        s.v[2570] = if s.b[2570] { 1.0 } else { 0.0 };

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && s.b[2570]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2452, 1.0, 1.0, A::mul(s.ad_value(217), s.ad_value(2397)));
        }

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && (!s.b[2570])) {
            s.store_offset_mul(2452, 217, 2397, 1.0);
        }

        s.b[2571] = (s.v[218] < 0.0);
        s.v[2571] = if s.b[2571] { 1.0 } else { 0.0 };

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && s.b[2571]) {
            s.store_sub_from_scalar_ad(2453, 1.0, A::mul(s.ad_value(218), s.ad_value(2450)));
        }

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && (!s.b[2571])) {
            s.store_div_from_scalar_offset_ad(2453, 1.0, A::mul(s.ad_value(218), s.ad_value(2450)), 1.0);
        }

        if (((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) {
            s.store_mul_ad_lhs(2454, A::mul3(s.ad_value(757), s.ad_value(2452), s.ad_value(2453)), 2450);
            s.store_mul_ad_rhs(2455, 774, A::add_scaled_product(s.ad_value(2451), 1.0, s.ad_value(775), s.ad_value(2450), 1.0));
            s.store_ln_ad(2028, A::div(s.ad_value(2445), A::offset(A::add(s.ad_value(2445), s.ad_value(2444)), 1e-14)));
            s.store_ad_value(2456, A::add_scaled_product(A::pow(A::mul(s.ad_value(2455), s.ad_value(704)), s.ad_value(705)), 1.0, s.ad_value(706), A::exp(A::mul_scaled_lhs(s.ad_value(707), 0.5, s.ad_value(2028))), 1.0));
            s.store_mul_add_ad_lhs(2457, A::offset(s.ad_value(2456), 1.0), s.ad_value(2454), 2448);
        }

        s.b[2572] = (s.v[221] < 0.0);
        s.v[2572] = if s.b[2572] { 1.0 } else { 0.0 };

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && s.b[2572]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2458, 1.0, 1.0, A::mul(s.ad_value(221), s.ad_value(2397)));
        }

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && (!s.b[2572])) {
            s.store_offset_mul(2458, 221, 2397, 1.0);
        }

        if (((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) {
            s.store_mul(2029, 2450, 2458);
            s.store_div_ad_rhs(2459, 2029, A::add(s.ad_value(223), s.ad_value(2029)));
        }

        s.b[2573] = (s.v[222] < 0.0);
        s.v[2573] = if s.b[2573] { 1.0 } else { 0.0 };

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && s.b[2573]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2460, 1.0, 1.0, A::mul(s.ad_value(222), s.ad_value(2459)));
        }

        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && (!s.b[2573])) {
            s.store_offset_mul(2460, 222, 2459, 1.0);
        }

        if (s.b[2547] && (!s.b[2548])) {
            s.copy_ad(2395, 1822);
            s.copy_ad(2397, 1823);
            s.copy_ad(2413, 1824);
            s.copy_ad(2414, 1825);
            s.copy_ad(2398, 1826);
            s.copy_ad(2399, 1827);
            s.copy_ad(2415, 1828);
            s.copy_ad(2417, 1829);
            s.copy_ad(2422, 1830);
            s.copy_ad(2423, 1831);
            s.copy_ad(2434, 1832);
            s.copy_ad(2435, 1833);
            s.copy_ad(2436, 1834);
            s.copy_ad(2543, 1835);
            s.copy_ad(2438, 1836);
            s.copy_ad(2437, 1837);
            s.copy_ad(2440, 1838);
            s.copy_ad(2441, 1839);
            s.copy_ad(2442, 1840);
            s.copy_ad(2443, 1841);
            s.copy_ad(2445, 1842);
            s.copy_ad(2444, 1843);
            s.copy_ad(2446, 1844);
            s.copy_ad(2447, 1845);
            s.copy_ad(2448, 1846);
            s.copy_ad(2449, 1847);
            s.copy_ad(2450, 1848);
            s.copy_ad(2451, 1849);
            s.copy_ad(2452, 1850);
            s.copy_ad(2453, 1851);
            s.copy_ad(2457, 1852);
            s.copy_ad(2458, 1853);
            s.copy_ad(2460, 1854);
        }

        if s.b[2547] {
            s.copy_ad(2393, 720);
            s.copy_ad(2394, 777);
        }

        s.b[2574] = (p.p48 != 0.0);
        s.v[2574] = if s.b[2574] { 1.0 } else { 0.0 };

        if (s.b[2547] && s.b[2574]) {
            s.copy_ad(2393, 721);
            s.copy_ad(2394, 778);
        }

        if s.b[2547] {
            s.store_scalar(2462, 0.0);
            s.store_scale(2461, 2413, 4.60517018598809);
            s.copy_ad(2478, 2461);
            s.copy_ad(2479, 826);
            s.store_mul(2480, 826, 2414);
            s.copy_ad(2484, 2437);
            s.store_scalar(2485, 0.0);
            s.store_scalar(2488, 0.0);
            s.copy_ad(2490, 2443);
            s.copy_ad(2491, 2445);
            s.copy_ad(2493, 2444);
            s.copy_ad(2494, 2451);
            s.copy_ad(2495, 2437);
            s.copy_ad(2496, 2443);
            s.copy_ad(2498, 2444);
            s.copy_ad(2499, 2445);
            s.store_sub(2500, 2417, 2437);
            s.store_scalar(2501, 1.0);
            s.store_scalar(2503, 1.0);
            s.store_scalar(2502, 0.0);
            s.copy_ad(2512, 2450);
            s.store_mul(2516, 2500, 2413);
            s.store_scalar(2513, 0.0);
            s.copy_ad(2514, 2451);
            s.store_scalar(2519, 0.0);
            s.store_scalar(2518, 1.0);
            s.copy_ad(2521, 2393);
            s.copy_ad(2520, 2516);
        }

        s.b[2575] = (s.v[2417] > 0.0);
        s.v[2575] = if s.b[2575] { 1.0 } else { 0.0 };

        s.b[2576] = (s.v[2444] > 1e-100);
        s.v[2576] = if s.b[2576] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2575]) && s.b[2576]) {
            s.store_mul(2521, 2393, 2460);
            s.store_div(2462, 2521, 2457);
            s.store_add_scaled_inputs(2463, 2449, 1.0, 2399, 0.5);
            s.store_div_ad_lhs(2027, A::div_scaled_product(s.ad_value(2399), s.ad_value(2442), 1.0, s.ad_value(2463), 1.0), 2463);
        }

        s.b[2577] = (s.v[2027] > 0.0001);
        s.v[2577] = if s.b[2577] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2577]) {
            s.store_sub_from_scalar(2028, 1.0, 2027);
        }

        s.b[2578] = (s.v[2028] < 1e-10);
        s.v[2578] = if s.b[2578] { 1.0 } else { 0.0 };

        if ((((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2577]) && s.b[2578]) {
            s.store_scalar(2029, 1.0);
        }

        if ((((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2577]) && (!s.b[2578])) {
            s.store_sub_from_scalar_ad(2029, 1.0, A::sqrt(s.ad_value(2028)));
        }

        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && (!s.b[2577])) {
            s.store_scale(2029, 2027, 0.5);
        }

        if ((s.b[2547] && s.b[2575]) && s.b[2576]) {
            s.store_mul(2464, 2029, 2463);
        }

        s.b[2579] = ((s.v[706] > 0.0) && (s.v[707] > 0.0));
        s.v[2579] = if s.b[2579] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2579]) {
            s.store_scaled_mul(2465, 2413, 2464, 0.475);
            s.store_ad_value(2027, A::add_scaled_product(s.ad_value(2450), 1.0, s.ad_value(2447), s.ad_value(2465), (-1.0)));
            s.store_scaled_add_ad_rhs(2466, 2027, A::sqrt(A::offset(A::square(s.ad_value(2027)), 1e-12)), 0.5);
            s.store_ad_value(2467, A::add_scaled_product(A::add_scaled_product(s.ad_value(2450), (-1.0), s.ad_value(2413), s.ad_value(2449), 1.0), 1.0, A::offset(s.ad_value(2447), (-1.0)), s.ad_value(2465), 1.0));
        }

    }

    pub(super) fn stamp_transient_block_36(
        s: &mut Scratch,
    ) {
        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2579]) {
            s.store_offset_ad(2468, A::div_scaled_product(s.ad_value(2399), s.ad_value(2413), 0.5, s.ad_value(2467), 1.0), 1.0);
            s.store_ad_value(2027, A::add_scaled_product(s.ad_value(2467), 1.0, s.ad_value(775), s.ad_value(2466), 1.0));
            s.store_pow_ad(2469, A::mul3(s.ad_value(774), s.ad_value(2027), s.ad_value(704)), s.ad_value(705));
            s.store_mul_ad_lhs(2028, A::div_scaled_product(s.ad_value(705), A::offset(A::mul_sub_from_scalar_rhs(s.ad_value(2468), 1.0, s.ad_value(775)), (-1.0)), 1.0, s.ad_value(2027), 1.0), 2469);
            s.store_div(2027, 2466, 2467);
            s.store_mul_pow_ad_rhs(2470, 706, A::offset(s.ad_value(2027), 1.0), A::neg(s.ad_value(707)));
            s.store_mul_ad_lhs(2029, A::div_scaled_product(s.ad_value(707), A::add(A::offset(s.ad_value(2468), (-1.0)), A::div_from_scalar(1.0, A::offset(s.ad_value(2027), 1.0))), 1.0, s.ad_value(2467), 1.0), 2470);
            s.store_mul_ad_lhs(2471, A::mul3(s.ad_value(757), s.ad_value(2452), s.ad_value(2453)), 2466);
            s.store_offset_div_ad(2027, A::add_scaled_product(s.ad_value(2028), 1.0, A::mul3(s.ad_value(757), s.ad_value(2452), s.ad_value(2453)), s.ad_value(2468), (-1.0)), s.ad_value(2029), 1.0);
        }

        s.b[2580] = (s.v[2027] < 230.25850929940458);
        s.v[2580] = if s.b[2580] { 1.0 } else { 0.0 };

        if ((((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2579]) && s.b[2580]) {
            s.store_scaled_ln_one_plus_exp_scaled_input(2028, 2027, 2.0, 0.5);
        }

        if ((((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2579]) && (!s.b[2580])) {
            s.copy_ad(2028, 2027);
        }

        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2579]) {
            s.store_ad_value(2472, A::div_scaled_product3(s.ad_value(2465), s.ad_value(2029), s.ad_value(2028), -1.0, A::add_scaled_inputs3_offset(s.ad_value(2469), 1.0, s.ad_value(2470), 1.0, s.ad_value(2471), 1.0, 1.0), 1.0));
            s.store_mul_offset_ad_rhs(2473, 2464, A::div(s.ad_value(2472), A::offset(A::sqrt(A::offset(A::square(s.ad_value(2472)), 1.0)), 1.0)), 1.0);
        }

        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && (!s.b[2579])) {
            s.copy_ad(2473, 2464);
        }

        if ((s.b[2547] && s.b[2575]) && s.b[2576]) {
            s.store_mul3_affine_lhs(2474, 2413, 2462, 0.7071067811865475, 0.0, 2473);
        }

        s.b[2581] = (s.v[0] == (-1.0));
        s.v[2581] = if s.b[2581] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2581]) {
            s.store_div_ad_rhs(2474, 2474, A::sqrt(A::offset(s.ad_value(2474), 1.0)));
        }

        if ((s.b[2547] && s.b[2575]) && s.b[2576]) {
            s.store_div_from_scalar_offset_ad(2475, 2.0, A::sqrt(A::scale_offset(s.ad_value(2474), 4.0, 1.0)), 1.0);
            s.store_mul(2027, 2475, 2474);
            s.store_mul_ad_product_rhs(2476, 2473, s.ad_value(2475), A::offset(A::div(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2027), 1.0, A::mul(s.ad_value(2027), s.ad_value(2475)), 0.86), A::offset(A::mul3_scaled_output(s.ad_value(2027), s.ad_value(2027), s.ad_value(2475), 4.0), 1.0)), 1.0));
            s.store_scale(2477, 2476, 0.99);
            s.store_ad_value(2027, A::div_scaled_product3(s.ad_value(2477), A::sub_scaled_inputs(s.ad_value(2477), 1.0, s.ad_value(2463), 2.0), s.ad_value(2415), 1.0, s.ad_value(2444), 1.0));
        }

        if ((s.b[2547] && s.b[2575]) && s.b[2576]) {
            s.store_mul_sub_ad_rhs(2478, 2413, s.ad_value(2477), A::ln(A::offset({
                if (s.v[2027] > (-0.99)) {
                    s.ad_value(2027)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0)));
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2576])) {
            s.copy_ad(2478, 2461);
        }

        if (s.b[2547] && s.b[2575]) {
            s.store_offset(2027, 2394, 1.0);
            s.store_ad_value(2028, A::div_scaled_product(A::sqrt(s.ad_value(2027)), s.ad_value(826), 1.0, s.ad_value(2478), 1.0));
            s.store_add_ad_lhs(2029, A::square(s.ad_value(2028)), 2027);
            s.store_scale(2027, 2028, 2.0);
            s.store_ad_value(2479, A::div_scaled_product(s.ad_value(2478), s.ad_value(2027), 1.0, A::add(A::sqrt(A::sub(s.ad_value(2029), s.ad_value(2027))), A::sqrt(A::add(s.ad_value(2029), s.ad_value(2027)))), 1.0));
            s.store_mul(2480, 2479, 2414);
            s.store_add(2481, 2423, 2480);
        }

        s.b[2582] = (s.v[2480] < 460.51701859880916);
        s.v[2582] = if s.b[2582] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2575]) && s.b[2582]) {
            s.store_exp_neg_input(2482, 2480);
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2582])) {
            s.store_div_from_scalar_offset_ad(2482, 1e-200, A::mul_offset_lhs(s.ad_value(2480), (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2480), (-460.51701859880916), A::scale_offset(s.ad_value(2480), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        if (s.b[2547] && s.b[2575]) {
            s.store_mul(2483, 2438, 2482);
        }

        s.b[2583] = (((s.v[2417]) as f64).abs() <= s.v[2435]);
        s.v[2583] = if s.b[2583] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2575]) && s.b[2583]) {
            s.store_scaled_square(2523, 2436, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(2484, 2417, s.ad_value(2436), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2417), 1.0, s.ad_value(2483)), s.ad_value(2398), s.ad_value(2523)), 1.0));
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2583])) {
            s.store_offset(2544, 2481, 3.0);
            s.store_ad_value(2527, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(2543), 0.5, s.ad_value(2544), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2543), s.ad_value(2544)), A::sub(s.ad_value(2543), s.ad_value(2544))), 5.0)), (-0.5)), 1.0, s.ad_value(2544), (-0.5), A::sqrt(A::offset(A::square(s.ad_value(2544)), 5.0)), (-(-0.5))));
            s.store_sub(2522, 2417, 2527);
            s.store_exp_neg_input(2523, 2527);
            s.store_div_from_scalar_offset_ad(2524, 1.0, A::square(s.ad_value(2527)), 2.0);
            s.store_mul_square_lhs(2534, 2527, 2524);
            s.store_mul3_affine_lhs(2535, 2527, 2524, 4.0, 0.0, 2524);
            s.store_mul_ad_product_lhs(2536, A::sub_scaled_inputs(s.ad_value(2524), 8.0, s.ad_value(2534), 12.0), s.ad_value(2524), 2524);
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2583])) {
            let assign52130_ad_e66997: A = {
                if (1e-40 > ((s.v[2522] * s.v[2522]) - (s.v[2399] * (((s.v[2523] + s.v[2527]) - 1.0) - (s.v[2483] * ((s.v[2527] + 1.0) + s.v[2534])))))) {
                    A::constant(1e-40)
                } else {
                    A::add_scaled_square_product(s.ad_value(2522), 1.0, s.ad_value(2399), A::add_scaled_product(A::offset(A::add(s.ad_value(2523), s.ad_value(2527)), (-1.0)), 1.0, s.ad_value(2483), A::add(A::offset(s.ad_value(2527), 1.0), s.ad_value(2534)), (-1.0)), (-1.0))
                }
            };
            s.store_ad_value(2528, assign52130_ad_e66997);
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2583])) {
            s.store_sub_from_scalar_ad(2545, 1.0, A::mul_scaled_output(s.ad_value(2399), A::add_scaled_product(s.ad_value(2523), 1.0, s.ad_value(2483), s.ad_value(2536), (-1.0)), 0.5));
            s.store_ad_value(2529, A::add_scaled_product(s.ad_value(2522), 2.0, s.ad_value(2399), A::add_scaled_sub_value_product(1.0, s.ad_value(2523), 1.0, s.ad_value(2483), A::offset(s.ad_value(2535), 1.0), (-1.0)), 1.0));
            s.store_ad_value(2530, A::add_scaled_inputs3(s.ad_value(2481), 1.0, s.ad_value(2527), (-1.0), A::ln(A::div(s.ad_value(2528), s.ad_value(2399))), 1.0));
            s.store_add(824, 2528, 2529);
            s.store_ad_value(823, A::add_scaled_square_product(s.ad_value(824), 1.0, s.ad_value(2530), A::add_scaled_square_product(s.ad_value(2529), 0.5, s.ad_value(2528), s.ad_value(2545), (-1.0)), 1.0));
            s.store_add_ad_rhs(2546, 2527, A::div_scaled_product3(s.ad_value(2528), s.ad_value(824), s.ad_value(2530), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2530), s.ad_value(2530)), s.ad_value(2529), A::add_scaled_square_product(s.ad_value(2529), 0.3333333333333333, s.ad_value(2528), s.ad_value(2545), (-1.0)))), 1.0));
        }

        s.b[2584] = (s.v[2546] < 230.25850929940458);
        s.v[2584] = if s.b[2584] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2575]) && (!s.b[2583])) && s.b[2584]) {
            s.store_exp(2532, 2546);
            s.store_div_from_scalar(2533, 1.0, 2532);
            s.store_mul(2532, 2483, 2532);
        }

        s.b[2585] = (s.v[2546] > (s.v[2481] - 230.25850929940458));
        s.v[2585] = if s.b[2585] { 1.0 } else { 0.0 };

        if ((((s.b[2547] && s.b[2575]) && (!s.b[2583])) && (!s.b[2584])) && s.b[2585]) {
            s.store_exp_sub(2532, 2546, 2481);
            s.store_div(2533, 2483, 2532);
        }

        if ((((s.b[2547] && s.b[2575]) && (!s.b[2583])) && (!s.b[2584])) && (!s.b[2585])) {
            s.store_div_from_scalar_offset_ad(2532, 1e-100, A::mul_offset_lhs(A::sub(s.ad_value(2481), s.ad_value(2546)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2481), s.ad_value(2546)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2481), s.ad_value(2546)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(2533, 1e-100, A::mul_offset_lhs(s.ad_value(2546), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2546), (-230.25850929940458), A::scale_offset(s.ad_value(2546), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2583])) {
            s.store_div_from_scalar_offset_ad(2522, 1.0, A::square(s.ad_value(2546)), 2.0);
            s.store_mul_square_lhs(2534, 2546, 2522);
            s.store_mul3_affine_lhs(2535, 2546, 2522, 4.0, 0.0, 2522);
            s.store_mul_ad_product_lhs(2536, A::sub_scaled_inputs(s.ad_value(2522), 8.0, s.ad_value(2534), 12.0), s.ad_value(2522), 2522);
            s.store_sub(2522, 2417, 2546);
            s.store_ad_value(2537, A::add_scaled_product(s.ad_value(2522), 2.0, s.ad_value(2399), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2533)), 1.0, s.ad_value(2532), 1.0, s.ad_value(2483), A::offset(s.ad_value(2535), 1.0), (-1.0)), 1.0));
            s.store_ad_value(2538, A::add_scaled_square_product(s.ad_value(2522), 1.0, s.ad_value(2399), A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2533), 1.0, s.ad_value(2546), 1.0, s.ad_value(2532), 1.0, (-1.0)), 1.0, s.ad_value(2483), A::add(A::offset(s.ad_value(2546), 1.0), s.ad_value(2534)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_ad(2522, 2.0, A::mul(s.ad_value(2399), A::add_scaled_inputs_product(s.ad_value(2533), 1.0, s.ad_value(2532), 1.0, s.ad_value(2483), s.ad_value(2536), (-1.0))));
            s.store_ad_value(2522, A::add_scaled_square_product(s.ad_value(2537), 1.0, s.ad_value(2538), s.ad_value(2522), (-2.0)));
            s.store_ad_value(2484, A::add_scaled_inputs(s.ad_value(2546), 1.0, A::div(s.ad_value(2538), A::add(s.ad_value(2537), A::sqrt(s.ad_value(2522)))), 2.0));
        }

        if (s.b[2547] && s.b[2575]) {
            s.store_sub(2485, 2484, 2437);
        }

        s.b[2586] = (s.v[2485] < 1e-10);
        s.v[2586] = if s.b[2586] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2575]) && s.b[2586]) {
            s.store_ad_value(2486, A::add_scaled_inputs_product(s.ad_value(2417), 2.0, s.ad_value(2437), (-2.0), s.ad_value(2399), A::add_scaled_product(A::add_scaled_sub_value_product(1.0, s.ad_value(2443), 1.0, s.ad_value(2442), s.ad_value(2482), 1.0), 1.0, s.ad_value(2483), A::offset(s.ad_value(2440), 1.0), (-1.0)), 1.0));
            s.store_mul_ad_lhs(2487, A::mul_sub_from_scalar_rhs(s.ad_value(2399), 1.0, s.ad_value(2482)), 2444);
            s.store_sub_from_scalar_ad(2027, 2.0, A::mul(s.ad_value(2399), A::add_scaled_product(A::add_scaled_product(s.ad_value(2443), 1.0, s.ad_value(2442), s.ad_value(2482), 1.0), 1.0, s.ad_value(2483), s.ad_value(2441), (-1.0))));
            s.store_ad_value(2027, A::add_scaled_square_product(s.ad_value(2486), 1.0, s.ad_value(2027), s.ad_value(2487), (-2.0)));
            s.store_scaled_div_ad_rhs(2485, 2487, A::add(s.ad_value(2486), A::sqrt(s.ad_value(2027))), 2.0);
            s.store_add(2484, 2437, 2485);
        }

        if (s.b[2547] && s.b[2575]) {
            s.store_mul(2488, 2485, 2413);
            s.store_ad_value(2489, A::div_scaled_product(s.ad_value(2484), s.ad_value(2484), 1.0, A::offset(A::square(s.ad_value(2484)), 2.0), 1.0));
        }

        s.b[2587] = (s.v[2484] < 230.25850929940458);
        s.v[2587] = if s.b[2587] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2575]) && s.b[2587]) {
            s.store_exp_neg_input(2490, 2484);
        }

        s.b[2588] = (s.v[2484] < 1e-5);
        s.v[2588] = if s.b[2588] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2575]) && s.b[2587]) && s.b[2588]) {
            s.store_ad_value(2491, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(2484)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2484), 1.0, A::scale(s.ad_value(2484), 0.25), 0.3333333333333333), 0.5));
            s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2484), 1.0, A::scale(s.ad_value(2484), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2492, 2484, 2027, 0.7071067811865475);
            s.store_ad_value(2493, A::mul3(A::mul3_scaled_output(s.ad_value(2483), s.ad_value(2484), s.ad_value(2484), 0.16666666666666666), s.ad_value(2484), A::scale_offset(s.ad_value(2484), 1.75, 1.0)));
        }

        if (((s.b[2547] && s.b[2575]) && s.b[2587]) && (!s.b[2588])) {
            s.store_add_ad_lhs(2491, A::offset(s.ad_value(2484), (-1.0)), 2490);
            s.store_sqrt(2492, 2491);
            s.store_mul_ad_rhs(2493, 2483, A::add_scaled_inputs3_offset(A::div_from_scalar(1.0, s.ad_value(2490)), 1.0, s.ad_value(2484), (-1.0), s.ad_value(2489), -1.0, (-1.0)));
        }

        s.b[2589] = (s.v[2484] > (s.v[2481] - 230.25850929940458));
        s.v[2589] = if s.b[2589] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2575]) && (!s.b[2587])) && s.b[2589]) {
            s.store_exp_sub(2027, 2484, 2481);
            s.store_div(2490, 2483, 2027);
            s.store_ad_value(2493, A::add_scaled_product(s.ad_value(2027), 1.0, s.ad_value(2483), A::add(A::offset(s.ad_value(2484), 1.0), s.ad_value(2489)), (-1.0)));
        }

        if (((s.b[2547] && s.b[2575]) && (!s.b[2587])) && (!s.b[2589])) {
            s.store_div_from_scalar_offset_ad(2490, 1e-100, A::mul_offset_lhs(s.ad_value(2484), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2484), (-230.25850929940458), A::scale_offset(s.ad_value(2484), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_offset_lhs(A::sub(s.ad_value(2481), s.ad_value(2484)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2481), s.ad_value(2484)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2481), s.ad_value(2484)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_ad_value(2493, A::add_scaled_product(s.ad_value(2027), 1.0, s.ad_value(2483), A::add(A::offset(s.ad_value(2484), 1.0), s.ad_value(2489)), (-1.0)));
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2587])) {
            s.store_add_ad_lhs(2491, A::offset(s.ad_value(2484), (-1.0)), 2490);
            s.store_sqrt(2492, 2491);
        }

        if (s.b[2547] && s.b[2575]) {
            s.store_mul3_lhs(2494, 2492, 2398, 2413);
            s.store_scaled_add(2495, 2437, 2484, 0.5);
            s.store_scalar(2496, 0.0);
            s.store_mul(2027, 2490, 2443);
        }

        s.b[2590] = (s.v[2027] > 0.0);
        s.v[2590] = if s.b[2590] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2575]) && s.b[2590]) {
            s.store_sqrt(2496, 2027);
        }

        if (s.b[2547] && s.b[2575]) {
            s.store_scaled_add(2497, 2444, 2493, 0.5);
            s.store_ad_value(2498, A::add_scaled_product(s.ad_value(2497), 1.0, A::square(s.ad_value(2485)), A::sub_scaled_inputs(s.ad_value(2496), 1.0, s.ad_value(2415), 2.0), 0.125));
        }

        s.b[2591] = (s.v[2495] < 1e-5);
        s.v[2591] = if s.b[2591] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2575]) && s.b[2591]) {
            s.store_ad_value(2499, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(2495)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2495), 1.0, A::scale(s.ad_value(2495), 0.25), 0.3333333333333333), 0.5));
            s.store_mul_sqrt_ad_rhs(2500, 2398, A::add(s.ad_value(2498), s.ad_value(2499)));
        }

        s.b[2592] = (s.v[730] > 0.0);
        s.v[2592] = if s.b[2592] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2575]) && s.b[2591]) && s.b[2592]) {
            s.store_div_from_scalar_sqrt_ad(2501, 1.0, A::offset(A::mul(s.ad_value(730), s.ad_value(2500)), 1.0));
        }

        if ((s.b[2547] && s.b[2575]) && s.b[2591]) {
            s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2495), 1.0, A::scale(s.ad_value(2495), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2502, 2495, 2027, 0.7071067811865475);
            s.store_add_ad_rhs(2503, 2501, A::div_scaled_product(s.ad_value(2398), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2495), 0.5)), 1.0, A::square(s.ad_value(2495)), 0.16666666666666666), 0.7071067811865475, s.ad_value(2027), 1.0));
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2591])) {
            s.store_add_ad_lhs(2499, A::offset(s.ad_value(2495), (-1.0)), 2496);
            s.store_mul_sqrt_ad_rhs(2500, 2398, A::add(s.ad_value(2498), s.ad_value(2499)));
        }

        s.b[2593] = (s.v[730] > 0.0);
        s.v[2593] = if s.b[2593] { 1.0 } else { 0.0 };

        if (((s.b[2547] && s.b[2575]) && (!s.b[2591])) && s.b[2593]) {
            s.store_ad_value(2504, A::add_scaled_sub_value_product(1.0, s.ad_value(2496), 1.0, s.ad_value(2500), s.ad_value(2415), 2.0));
            s.store_div_from_scalar_sqrt_ad(2501, 1.0, A::offset(A::mul(s.ad_value(730), s.ad_value(2500)), 1.0));
            s.store_div_ad_rhs(2027, 2501, A::offset(s.ad_value(2501), 1.0));
            s.store_mul_ad_rhs(2505, 730, A::mul3(A::square(s.ad_value(2027)), s.ad_value(2399), s.ad_value(2498)));
            s.store_ad_value(2506, A::add_scaled_inputs_product(s.ad_value(2500), 2.0, s.ad_value(2505), (-2.0), s.ad_value(2399), A::add(A::sub_from_scalar(1.0, s.ad_value(2496)), s.ad_value(2498)), 1.0));
            s.store_mul_ad_rhs(2507, 2505, A::sub_scaled_inputs(s.ad_value(2505), 1.0, s.ad_value(2500), 2.0));
            s.store_sub_from_scalar_ad(2508, 1.0, A::mul_scaled_output(s.ad_value(2399), A::add(s.ad_value(2496), s.ad_value(2498)), 0.5));
        }

    }

    pub(super) fn stamp_transient_block_37(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[2547] && s.b[2575]) && (!s.b[2591])) && s.b[2593]) {
            s.store_ad_value(2509, A::div_scaled_product(s.ad_value(2507), s.ad_value(2506), 1.0, A::add_scaled_square_product(s.ad_value(2506), 1.0, s.ad_value(2508), s.ad_value(2507), (-1.0)), 1.0));
            s.store_add(2495, 2495, 2509);
            s.store_exp(2510, 2509);
            s.store_div(2496, 2496, 2510);
            s.store_mul(2498, 2498, 2510);
            s.store_add_ad_lhs(2499, A::offset(s.ad_value(2495), (-1.0)), 2496);
            s.store_mul_sqrt_ad_rhs(2500, 2398, A::add(s.ad_value(2498), s.ad_value(2499)));
            s.store_add_ad(2511, A::sub_from_scalar(1.0, s.ad_value(2496)), A::mul3_scaled_output(s.ad_value(2500), s.ad_value(2501), s.ad_value(2415), 2.0));
            s.store_ad_value(2485, A::div_scaled_product3(s.ad_value(2485), s.ad_value(2510), A::add(s.ad_value(2504), s.ad_value(2497)), 1.0, A::add_scaled_product(s.ad_value(2511), 1.0, s.ad_value(2510), s.ad_value(2497), 1.0), 1.0));
            s.store_mul(2488, 2485, 2413);
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2591])) {
            s.store_sqrt(2502, 2499);
            s.store_ad_value(2503, A::add_scaled_inputs(s.ad_value(2501), 1.0, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2398), 1.0, s.ad_value(2496)), s.ad_value(2502)), 0.5));
        }

        if (s.b[2547] && s.b[2575]) {
            s.store_mul_ad_rhs(2512, 2413, A::div_scaled_product(s.ad_value(2399), s.ad_value(2498), 1.0, A::add_scaled_product(s.ad_value(2500), 1.0, s.ad_value(2398), s.ad_value(2502), 1.0), 1.0));
            s.store_ad_value(2513, A::add_scaled_product(s.ad_value(2512), 1.0, s.ad_value(2413), s.ad_value(2503), 1.0));
            s.store_mul3_lhs(2514, 2502, 2398, 2413);
        }

        s.b[2594] = (s.v[218] < 0.0);
        s.v[2594] = if s.b[2594] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2575]) && s.b[2594]) {
            s.store_sub_from_scalar_ad(2453, 1.0, A::mul(s.ad_value(218), s.ad_value(2512)));
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2594])) {
            s.store_div_from_scalar_offset_ad(2453, 1.0, A::mul(s.ad_value(218), s.ad_value(2512)), 1.0);
        }

        if (s.b[2547] && s.b[2575]) {
            s.store_mul_ad_lhs(2454, A::mul3(s.ad_value(757), s.ad_value(2452), s.ad_value(2453)), 2512);
            s.store_ad_value(2515, A::add_scaled_product(s.ad_value(2514), 1.0, s.ad_value(775), s.ad_value(2512), 1.0));
            s.store_ad_value(2516, A::add_scaled_product(s.ad_value(2514), 1.0, s.ad_value(776), s.ad_value(2512), 1.0));
            s.store_mul(2517, 774, 2515);
            s.store_ln_ad(2028, A::div(s.ad_value(2499), A::offset(A::add(s.ad_value(2499), s.ad_value(2498)), 1e-14)));
            s.store_ad_value(2456, A::add_scaled_product(A::pow(A::mul(s.ad_value(2517), s.ad_value(704)), s.ad_value(705)), 1.0, s.ad_value(706), A::exp(A::mul_scaled_lhs(s.ad_value(707), 0.5, s.ad_value(2028))), 1.0));
            s.store_mul_add_ad_lhs(2518, A::offset(s.ad_value(2456), 1.0), s.ad_value(2454), 2448);
            s.store_ln_ad(2519, A::div(A::offset(A::mul(A::sub(s.ad_value(826), s.ad_value(2488)), s.ad_value(779)), 1.0), A::offset(A::mul(A::sub(s.ad_value(2479), s.ad_value(2488)), s.ad_value(779)), 1.0)));
            s.store_mul(2029, 2512, 2458);
            s.store_div_ad_rhs(2459, 2029, A::add(s.ad_value(223), s.ad_value(2029)));
        }

        s.b[2595] = (s.v[222] < 0.0);
        s.v[2595] = if s.b[2595] { 1.0 } else { 0.0 };

        if ((s.b[2547] && s.b[2575]) && s.b[2595]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2460, 1.0, 1.0, A::mul(s.ad_value(222), s.ad_value(2459)));
        }

        if ((s.b[2547] && s.b[2575]) && (!s.b[2595])) {
            s.store_offset_mul(2460, 222, 2459, 1.0);
        }

        if (s.b[2547] && s.b[2575]) {
            s.store_mul(2521, 2393, 2460);
            s.store_mul(2520, 2500, 2413);
        }

        if s.b[2547] {
            s.copy_ad(1887, 2395);
            s.copy_ad(1888, 2413);
            s.copy_ad(1889, 2398);
            s.copy_ad(1890, 2417);
            s.copy_ad(1891, 2422);
            s.copy_ad(1892, 2451);
            s.copy_ad(1893, 2488);
            s.copy_ad(1894, 2494);
            s.copy_ad(1895, 2501);
            s.copy_ad(1896, 2503);
            s.copy_ad(1897, 2512);
            s.copy_ad(1898, 2513);
            s.copy_ad(1899, 2516);
            s.copy_ad(1900, 2518);
            s.copy_ad(1901, 2519);
            s.copy_ad(1902, 2521);
            s.copy_ad(1903, 2520);
            s.copy_ad(1932, 2414);
            s.copy_ad(1933, 2435);
            s.copy_ad(1934, 2495);
            s.copy_ad(1935, 2500);
        }

        if (!s.b[2547]) {
            s.copy_ad(745, 728);
            s.copy_ad(1887, 1822);
            s.copy_ad(1888, 1824);
            s.copy_ad(1889, 1826);
            s.copy_ad(1890, 1829);
            s.copy_ad(1891, 1830);
            s.copy_ad(1892, 1849);
            s.copy_ad(1893, 1860);
            s.copy_ad(1894, 1861);
            s.copy_ad(1895, 1863);
            s.copy_ad(1896, 1864);
            s.copy_ad(1897, 1865);
            s.copy_ad(1898, 1866);
            s.copy_ad(1899, 1868);
            s.copy_ad(1900, 1869);
            s.copy_ad(1901, 1871);
            s.copy_ad(1902, 1870);
            s.copy_ad(1903, 1872);
            s.copy_ad(1932, 1825);
            s.copy_ad(1933, 1833);
            s.copy_ad(1934, 1862);
            s.copy_ad(1935, 1931);
        }

        s.copy_ad(1904, 255);

        s.b[2596] = (s.v[773] > 0.0);
        s.v[2596] = if s.b[2596] { 1.0 } else { 0.0 };

        if s.b[2596] {
            s.store_div_ad_rhs(1904, 255, A::offset(A::mul(s.ad_value(773), A::powf(A::offset(A::square(s.ad_value(1899)), s.v[733]), ((-1.0) * 0.16666666666666666))), 1.0));
        }

        s.v[1905] = 1.0;

        s.v[1906] = 1.0;

        s.v[1907] = 0.0;

        s.v[1908] = 1.0;

        s.v[1909] = 1.0;

        s.copy_ad(2359, 1903);

        s.v[2362] = 0.0;

        s.v[2361] = 0.0;

        s.copy_ad(2363, 2359);

        s.b[2597] = (s.v[1890] > 0.0);
        s.v[2597] = if s.b[2597] { 1.0 } else { 0.0 };

        if s.b[2597] {
            s.store_mul_ad_lhs(2354, A::div_scaled_product(A::add(s.ad_value(260), A::div(s.ad_value(261), s.ad_value(1898))), s.ad_value(1897), 1.0, s.ad_value(1898), 1.0), 1901);
        }

        s.b[2598] = (s.v[2354] > 0.0);
        s.v[2598] = if s.b[2598] { 1.0 } else { 0.0 };

        if (s.b[2597] && s.b[2598]) {
            s.store_div_from_scalar_add_ad(1905, 1.0, A::offset(s.ad_value(2354), 1.0), A::square(s.ad_value(2354)));
        }

        if (s.b[2597] && (!s.b[2598])) {
            s.store_sub_from_scalar(1905, 1.0, 2354);
        }

        if s.b[2597] {
            s.store_mul(1906, 1900, 1905);
            s.store_div(1907, 1902, 1906);
            s.store_mul_ad_product_lhs(2355, A::square(s.ad_value(1907)), s.ad_value(1893), 1893);
        }

        s.b[2599] = (s.v[0] == (-1.0));
        s.v[2599] = if s.b[2599] { 1.0 } else { 0.0 };

        if (s.b[2597] && s.b[2599]) {
            s.store_div_ad_rhs(2355, 2355, A::offset(A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0));
        }

        if s.b[2597] {
            s.store_ad_value(1908, A::mul_offset_rhs_scaled_output(s.ad_value(1906), A::sqrt(A::scale_offset(s.ad_value(2355), 2.0, 1.0)), 1.0, 0.5));
            s.store_div(2027, 1906, 1908);
            s.store_mul_offset_ad_rhs(2356, 1896, A::mul3_scaled_output(s.ad_value(2355), s.ad_value(2027), s.ad_value(2027), 0.5), 1.0);
            s.store_ad_value(1909, A::div_scaled_product(s.ad_value(2027), s.ad_value(1898), 1.0, s.ad_value(2356), 1.0));
            s.store_scaled_div(2357, 1893, 1909, 0.5);
            s.store_square(2358, 2357);
            s.store_add_ad_rhs(2359, 1903, A::mul3_scaled_output(s.ad_value(1895), s.ad_value(1893), A::add(A::offset(A::mul_scaled_output(s.ad_value(2357), s.ad_value(1905), 0.3333333333333333), (-1.0)), s.ad_value(1905)), 0.5));
            s.store_scaled_mul(2027, 1896, 1893, 0.16666666666666666);
        }

        s.b[2600] = (p.p49 == 1.0);
        s.v[2600] = if s.b[2600] { 1.0 } else { 0.0 };

        if (s.b[2597] && s.b[2600]) {
            s.store_scalar(2360, 0.0);
            s.store_mul_ad_affine_product_rhs(2361, 1905, s.ad_value(1905), A::sub(s.ad_value(1897), A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2027), 2.0, s.ad_value(2357), 3.0)), 0.5, 0.0);
        }

        if (s.b[2597] && (!s.b[2600])) {
            s.store_ad_value(2360, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(1905), A::add_scaled_product(s.ad_value(1897), 1.0, s.ad_value(1896), s.ad_value(1893), (-0.5))));
            s.store_ad_value(2361, A::add_scaled_products(A::square(s.ad_value(1905)), A::add_scaled_product(s.ad_value(1897), 1.0, s.ad_value(2027), A::sub_scaled_inputs(A::sub_from_scalar(1.0, s.ad_value(2357)), 1.0, s.ad_value(2358), 0.2), (-1.0)), 0.5, s.ad_value(2360), A::offset(s.ad_value(1905), 1.0), 0.5));
        }

        if s.b[2597] {
            s.store_ad_value(2362, A::add_scaled_product(s.ad_value(2360), 1.0, s.ad_value(1905), A::add_scaled_product(s.ad_value(1897), 1.0, s.ad_value(2027), s.ad_value(2357), 1.0), 1.0));
            s.store_sub(2363, 2359, 2362);
        }

        s.store_mul(851, 2359, 1904);

        s.store_mul_neg_lhs(853, 2361, 1904);

        s.store_mul_neg_lhs(852, 2363, 1904);

        s.v[2379] = 0.0;

        s.v[2380] = 0.0;

        s.v[2378] = 0.0;

        s.b[2601] = ((s.v[268] > 0.0) || (s.v[269] > 0.0));
        s.v[2601] = if s.b[2601] { 1.0 } else { 0.0 };

        if s.b[2601] {
            s.store_scalar(2368, 1.0);
            s.copy_ad(2367, 1887);
        }

        s.b[2602] = (s.v[272] > 1e-10);
        s.v[2602] = if s.b[2602] { 1.0 } else { 0.0 };

        if (s.b[2601] && s.b[2602]) {
            s.store_ad_value(2364, A::add_scaled_inputs3(s.ad_value(1887), 1.0, s.ad_value(270), (-1.0), s.ad_value(808), 1.0));
            s.store_ad_value(2027, A::add_scaled_inputs3(s.ad_value(2364), 0.5, s.ad_value(808), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(809), 1.0, A::sub(s.ad_value(2364), s.ad_value(808)), A::sub(s.ad_value(2364), s.ad_value(808)), 1.0)), 0.5));
            s.store_mul_ad_rhs(2028, 2027, A::add_scaled_inputs3(s.ad_value(2027), 2.0, s.ad_value(808), (-1.0), s.ad_value(2364), -1.0));
            s.store_div(2029, 808, 2027);
            s.store_mul(2365, 2364, 2029);
            s.store_sqrt_sub_from_scalar_ad(2366, 1.0, A::mul(s.ad_value(2365), s.ad_value(272)));
            s.store_ad_value(2367, A::add_scaled_inputs3(A::div(A::sub_from_scalar(1.0, s.ad_value(2366)), s.ad_value(272)), 1.0, s.ad_value(2364), 1.0, s.ad_value(2365), -1.0));
            s.store_offset_ad(2368, A::div_scaled_product3(A::offset(A::div_from_scalar(0.5, s.ad_value(2366)), (-1.0)), A::add_scaled_product(s.ad_value(2028), 1.0, s.ad_value(2364), A::sub(s.ad_value(808), s.ad_value(2027)), 1.0), s.ad_value(2029), 1.0, s.ad_value(2028), 1.0), 1.0);
        }

        if s.b[2601] {
            s.store_scalar(2370, 1.0);
            s.store_scalar(2371, 0.0);
        }

        s.b[2603] = (s.v[271] > 0.0);
        s.v[2603] = if s.b[2603] { 1.0 } else { 0.0 };

        if (s.b[2601] && s.b[2603]) {
            s.store_ad_value(2027, A::add_scaled_product(s.ad_value(745), 0.5, s.ad_value(1888), A::scale_offset(s.ad_value(1889), 0.7071067811865475, 1.0), 1.0));
            s.store_div(2369, 1887, 2027);
        }

        s.b[2604] = (((s.v[2369]) as f64).abs() < 230.25850929940458);
        s.v[2604] = if s.b[2604] { 1.0 } else { 0.0 };

        if ((s.b[2601] && s.b[2603]) && s.b[2604]) {
            s.store_div_from_scalar_offset_ad(2370, 1.0, A::exp_scaled_input(s.ad_value(2369), -1.0), 1.0);
        }

        s.b[2605] = (s.v[2369] < 0.0);
        s.v[2605] = if s.b[2605] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_38(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[2601] && s.b[2603]) && (!s.b[2604])) && s.b[2605]) {
            s.store_div_from_scalar_offset_ad(2370, 1e-100, A::mul_offset_lhs(s.ad_value(2369), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2369), (-230.25850929940458), A::scale_offset(s.ad_value(2369), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        s.b[2606] = (s.v[2369] < 230.25850929940458);
        s.v[2606] = if s.b[2606] { 1.0 } else { 0.0 };

        if ((s.b[2601] && s.b[2603]) && s.b[2606]) {
            s.store_ln_one_plus_exp(2028, 2369);
        }

        if ((s.b[2601] && s.b[2603]) && (!s.b[2606])) {
            s.copy_ad(2028, 2369);
        }

        if (s.b[2601] && s.b[2603]) {
            s.store_mul(2371, 2027, 2028);
        }

        if s.b[2601] {
            s.store_ad_value(2372, A::add_scaled_product(s.ad_value(2368), 1.0, s.ad_value(271), A::sub(s.ad_value(2370), s.ad_value(2368)), 1.0));
            s.store_ad_value(2373, A::add_scaled_product(s.ad_value(2367), 1.0, s.ad_value(271), A::sub(s.ad_value(2371), s.ad_value(2367)), 1.0));
            s.store_ad_value(2374, A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(1887), 1.0, s.ad_value(1888), s.ad_value(1891), (-1.0)), 1.0, s.ad_value(1903), (-1.0), s.ad_value(1893), (-0.5)));
            s.store_ad_value(2375, A::add_scaled_inputs3(s.ad_value(1887), 1.0, s.ad_value(2374), (-1.0), s.ad_value(1892), -1.0));
            s.store_ad_value(2376, A::add_scaled_inputs3(s.ad_value(1893), 1.0, s.ad_value(2374), 1.0, s.ad_value(826), -1.0));
            s.store_ad_value(2377, A::add_scaled_inputs3(s.ad_value(1887), 1.0, s.ad_value(2376), (-1.0), s.ad_value(1894), -1.0));
        }

        s.b[2607] = (s.v[831] > 0.0);
        s.v[2607] = if s.b[2607] { 1.0 } else { 0.0 };

        if (s.b[2601] && s.b[2607]) {
            s.store_mul_ad_rhs(2378, 2372, A::add_scaled_products(s.ad_value(269), s.ad_value(2376), 1.0, s.ad_value(268), s.ad_value(2374), 1.0));
            s.store_mul_sub_rhs(2379, 268, 2375, 2373);
            s.store_mul_sub_rhs(2380, 269, 2377, 2373);
        }

        if (s.b[2601] && (!s.b[2607])) {
            s.store_mul_ad_rhs(2378, 2372, A::add_scaled_products(s.ad_value(268), s.ad_value(2376), 1.0, s.ad_value(269), s.ad_value(2374), 1.0));
            s.store_mul_sub_rhs(2379, 269, 2375, 2373);
            s.store_mul_sub_rhs(2380, 268, 2377, 2373);
        }

        if s.b[2601] {
            s.store_add(851, 851, 2378);
            s.store_add(853, 853, 2380);
            s.store_sub_ad_lhs(852, A::add_scaled_inputs3(s.ad_value(852), 1.0, s.ad_value(2378), (-1.0), s.ad_value(2380), -1.0), 2379);
        }

        s.store_mul(1910, 262, 1878);

        s.store_mul(1911, 263, 1879);

        s.v[2383] = 0.0;

        s.v[2381] = 0.0;

        s.b[2608] = ((s.v[262] > 0.0) && (s.v[264] > 0.0));
        s.v[2608] = if s.b[2608] { 1.0 } else { 0.0 };

        if s.b[2608] {
            s.store_mul_ad_rhs(2027, 266, A::add_scaled_inputs(s.ad_value(1819), 0.5, s.ad_value(787), 1.0));
        }

        s.b[2609] = (s.v[2027] < 230.25850929940458);
        s.v[2609] = if s.b[2609] { 1.0 } else { 0.0 };

        s.b[2610] = (s.v[2027] > (-230.25850929940458));
        s.v[2610] = if s.b[2610] { 1.0 } else { 0.0 };

        if ((s.b[2608] && s.b[2609]) && s.b[2610]) {
            s.store_exp(2381, 2027);
        }

        if ((s.b[2608] && s.b[2609]) && (!s.b[2610])) {
            s.store_div_from_scalar_offset_ad(2381, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2027), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2027), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        s.b[2611] = (s.v[2381] > 1e-10);
        s.v[2611] = if s.b[2611] { 1.0 } else { 0.0 };

        if ((s.b[2608] && s.b[2609]) && s.b[2611]) {
            s.store_ln_offset_input(2382, 2381, 1.0);
            s.store_mul_sub_from_scalar_ad_rhs(2028, 2382, 1.0, A::div(A::ln(A::offset(s.ad_value(2382), 1.0)), A::offset(s.ad_value(2382), 2.0)));
        }

        if ((s.b[2608] && s.b[2609]) && (!s.b[2611])) {
            s.copy_ad(2382, 2381);
            s.store_ad_value(2028, A::div_scaled_inputs(s.ad_value(2382), 2.0, A::offset(s.ad_value(2382), 2.0), 1.0));
        }

        if (s.b[2608] && (!s.b[2609])) {
            s.copy_ad(2382, 2027);
            s.store_mul_sub_from_scalar_ad_rhs(2028, 2382, 1.0, A::div(A::ln(A::offset(s.ad_value(2382), 1.0)), A::offset(s.ad_value(2382), 2.0)));
        }

        if s.b[2608] {
            s.store_mul_ad_affine_product_lhs(2383, A::div_scaled_inputs(s.ad_value(264), (-2.0), s.ad_value(266), 1.0), s.ad_value(262), s.v[354], 0.0, 2028);
        }

        s.v[2386] = 0.0;

        s.v[2384] = 0.0;

        s.b[2612] = ((s.v[263] > 0.0) && (s.v[265] > 0.0));
        s.v[2612] = if s.b[2612] { 1.0 } else { 0.0 };

        if s.b[2612] {
            s.store_mul_ad_rhs(2027, 266, A::add_scaled_inputs(s.ad_value(1819), 0.5, s.ad_value(788), 1.0));
        }

        s.b[2613] = (s.v[2027] < 230.25850929940458);
        s.v[2613] = if s.b[2613] { 1.0 } else { 0.0 };

        s.b[2614] = (s.v[2027] > (-230.25850929940458));
        s.v[2614] = if s.b[2614] { 1.0 } else { 0.0 };

        if ((s.b[2612] && s.b[2613]) && s.b[2614]) {
            s.store_exp(2384, 2027);
        }

        if ((s.b[2612] && s.b[2613]) && (!s.b[2614])) {
            s.store_div_from_scalar_offset_ad(2384, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2027), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2027), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        s.b[2615] = (s.v[2384] > 1e-10);
        s.v[2615] = if s.b[2615] { 1.0 } else { 0.0 };

        if ((s.b[2612] && s.b[2613]) && s.b[2615]) {
            s.store_ln_offset_input(2385, 2384, 1.0);
            s.store_mul_sub_from_scalar_ad_rhs(2028, 2385, 1.0, A::div(A::ln(A::offset(s.ad_value(2385), 1.0)), A::offset(s.ad_value(2385), 2.0)));
        }

        if ((s.b[2612] && s.b[2613]) && (!s.b[2615])) {
            s.copy_ad(2385, 2384);
            s.store_ad_value(2028, A::div_scaled_inputs(s.ad_value(2385), 2.0, A::offset(s.ad_value(2385), 2.0), 1.0));
        }

        if (s.b[2612] && (!s.b[2613])) {
            s.copy_ad(2385, 2027);
            s.store_mul_sub_from_scalar_ad_rhs(2028, 2385, 1.0, A::div(A::ln(A::offset(s.ad_value(2385), 1.0)), A::offset(s.ad_value(2385), 2.0)));
        }

        if s.b[2612] {
            s.store_mul_ad_affine_product_lhs(2386, A::div_scaled_inputs(s.ad_value(265), (-2.0), s.ad_value(266), 1.0), s.ad_value(263), s.v[354], 0.0, 2028);
        }

        s.store_add(2387, 2383, 2386);

        s.store_ad_value(856, A::add_scaled_product(s.ad_value(2387), 1.0, s.ad_value(267), s.ad_value(829), 1.0));

        s.store_mul(854, 274, 834);

        s.store_mul(855, 275, 837);

        s.v[1938] = 0.0;

        s.v[1939] = 0.0;

        s.v[1940] = 0.0;

        s.v[1941] = 0.0;

        s.b[2616] = (s.v[1] != 0.0);
        s.v[2616] = if s.b[2616] { 1.0 } else { 0.0 };

        s.b[2617] = (s.v[1890] <= 0.0);
        s.v[2617] = if s.b[2617] { 1.0 } else { 0.0 };

        if (s.b[2616] && s.b[2617]) {
            s.store_scalar(1936, 0.5);
            s.store_scalar(1937, 1.0);
            s.copy_ad(1938, 1889);
        }

        if (s.b[2616] && (!s.b[2617])) {
            s.store_offset_scaled_div(1936, 1893, 1909, ((0.25) * (0.5)), 0.5);
            s.store_div_ad_rhs(1937, 1935, A::sub(s.ad_value(1890), s.ad_value(1934)));
            s.store_div(1938, 1889, 1937);
        }

        if s.b[2616] {
            s.store_square(1939, 1938);
            s.store_offset_scaled(1940, 1938, 0.7071067811865475, 1.0);
            s.store_scale(1941, 1940, 1e-5);
        }

        s.v[2618] = 0.0;

        s.v[2619] = 0.0;

        s.v[2620] = 0.0;

        s.v[2621] = 0.0;

        s.v[2622] = 0.0;

        s.v[2623] = 0.0;

        s.v[2624] = 0.0;

        s.v[2625] = 0.0;

        s.v[2626] = 0.0;

        s.v[2627] = 0.0;

        s.v[2628] = 0.0;

        s.v[2629] = 0.0;

        s.v[2630] = 0.0;

        s.v[2631] = 0.0;

        s.v[2632] = 0.0;

        s.v[2633] = 0.0;

        s.v[2634] = 0.0;

        s.v[2635] = 0.0;

        s.v[2636] = 0.0;

        s.v[2637] = 0.0;

        s.v[2638] = 0.0;

        s.v[2639] = 0.0;

        s.v[2640] = 0.0;

        s.v[2641] = 0.0;

        s.v[2642] = 0.0;

        s.v[2643] = 0.0;

        s.v[2644] = 0.0;

        s.v[2645] = 0.0;

        s.v[2646] = 0.0;

        s.v[2647] = 0.0;

        s.v[2648] = 0.0;

        s.v[2649] = 0.0;

        s.v[2650] = 0.0;

        s.v[2651] = 0.0;

        s.v[2652] = 0.0;

        s.v[2653] = 0.0;

        s.v[2654] = 0.0;

        s.v[2655] = 0.0;

        s.v[2656] = 0.0;

        s.v[2657] = 0.0;

        s.v[2658] = 0.0;

        s.v[2659] = 0.0;

        s.v[2660] = 0.0;

        s.v[2661] = 0.0;

        s.v[2662] = 0.0;

        s.v[2663] = 0.0;

        s.v[2664] = 0.0;

        s.v[848] = 0.0;

        s.v[1912] = 0.0;

        s.v[1913] = 0.0;

        s.v[1914] = 0.0;

        s.v[849] = 0.0;

        s.v[1915] = 0.0;

        s.v[1916] = 0.0;

        s.v[1917] = 0.0;

        s.v[857] = 0.0;

        s.v[1918] = 0.0;

        s.v[1919] = 0.0;

        s.v[1920] = 0.0;

        s.v[858] = 0.0;

        s.v[1921] = 0.0;

        s.v[1922] = 0.0;

        s.v[1923] = 0.0;

        s.b[2665] = (p.p43 > 0.0);
        s.v[2665] = if s.b[2665] { 1.0 } else { 0.0 };

        s.b[2666] = (s.v[474] == 1.0);
        s.v[2666] = if s.b[2666] { 1.0 } else { 0.0 };

        if (s.b[2665] && s.b[2666]) {
            s.store_scale(496, 832, (s.v[371] * s.v[668]));
        }

        if (s.b[2665] && s.b[2666]) {
            s.store_ad_value(497, {
                if (s.v[496] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(496)), 1.0))
                } else {
                    {
                        if (s.v[496] > s.v[660]) {
                            A::mul_offset_rhs(s.ad_value(661), A::sub(s.ad_value(496), s.ad_value(660)), 1.0)
                        } else {
                            A::exp(s.ad_value(496))
                        }
                    }
                }
            });
        }

        if (s.b[2665] && s.b[2666]) {
            s.store_mul_offset_rhs(502, 667, 497, (-1.0));
            s.store_scaled_mul(496, 832, 670, s.v[371]);
        }

        if (s.b[2665] && s.b[2666]) {
            s.store_ad_value(497, {
                if (s.v[496] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(496)), 1.0))
                } else {
                    {
                        if (s.v[496] > s.v[662]) {
                            A::mul_offset_rhs(s.ad_value(663), A::sub(s.ad_value(496), s.ad_value(662)), 1.0)
                        } else {
                            A::exp(s.ad_value(496))
                        }
                    }
                }
            });
        }

        if (s.b[2665] && s.b[2666]) {
            s.store_mul_offset_rhs(503, 669, 497, (-1.0));
            s.store_scalar(504, 0.0);
        }

        s.b[2667] = (s.v[666] > 0.0);
        s.v[2667] = if s.b[2667] { 1.0 } else { 0.0 };

        if ((s.b[2665] && s.b[2666]) && s.b[2667]) {
            s.store_mul_ad_rhs(504, 832, A::add_scaled_product(s.ad_value(671), 1.0, s.ad_value(832), s.ad_value(672), 1.0));
        }

        if ((s.b[2665] && s.b[2666]) && (!s.b[2667])) {
            s.store_scaled_mul(496, 832, 672, (-s.v[371]));
        }

        if ((s.b[2665] && s.b[2666]) && (!s.b[2667])) {
            s.store_ad_value(497, {
                if (s.v[496] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(496)), 1.0))
                } else {
                    {
                        if (s.v[496] > s.v[664]) {
                            A::mul_offset_rhs(s.ad_value(665), A::sub(s.ad_value(496), s.ad_value(664)), 1.0)
                        } else {
                            A::exp(s.ad_value(496))
                        }
                    }
                }
            });
        }

        if ((s.b[2665] && s.b[2666]) && (!s.b[2667])) {
            s.store_mul_scaled_ad_rhs(504, 671, -1.0, A::offset(s.ad_value(497), (-1.0)));
        }

        if (s.b[2665] && s.b[2666]) {
            s.store_ad_value(848, A::add_scaled_inputs3(s.ad_value(502), 1.0, s.ad_value(503), 1.0, s.ad_value(504), 1.0));
            s.store_scale(496, 833, (s.v[371] * s.v[695]));
        }

        if (s.b[2665] && s.b[2666]) {
            s.store_ad_value(497, {
                if (s.v[496] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(496)), 1.0))
                } else {
                    {
                        if (s.v[496] > s.v[687]) {
                            A::mul_offset_rhs(s.ad_value(688), A::sub(s.ad_value(496), s.ad_value(687)), 1.0)
                        } else {
                            A::exp(s.ad_value(496))
                        }
                    }
                }
            });
        }

        if (s.b[2665] && s.b[2666]) {
            s.store_mul_offset_rhs(502, 694, 497, (-1.0));
            s.store_scaled_mul(496, 833, 697, s.v[371]);
        }

    }

    pub(super) fn stamp_transient_block_39(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[2665] && s.b[2666]) {
            s.store_ad_value(497, {
                if (s.v[496] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(496)), 1.0))
                } else {
                    {
                        if (s.v[496] > s.v[689]) {
                            A::mul_offset_rhs(s.ad_value(690), A::sub(s.ad_value(496), s.ad_value(689)), 1.0)
                        } else {
                            A::exp(s.ad_value(496))
                        }
                    }
                }
            });
        }

        if (s.b[2665] && s.b[2666]) {
            s.store_mul_offset_rhs(503, 696, 497, (-1.0));
            s.store_scalar(504, 0.0);
        }

        s.b[2668] = (s.v[693] > 0.0);
        s.v[2668] = if s.b[2668] { 1.0 } else { 0.0 };

        if ((s.b[2665] && s.b[2666]) && s.b[2668]) {
            s.store_mul_ad_rhs(504, 833, A::add_scaled_product(s.ad_value(698), 1.0, s.ad_value(833), s.ad_value(699), 1.0));
        }

        if ((s.b[2665] && s.b[2666]) && (!s.b[2668])) {
            s.store_scaled_mul(496, 833, 699, (-s.v[371]));
        }

        if ((s.b[2665] && s.b[2666]) && (!s.b[2668])) {
            s.store_ad_value(497, {
                if (s.v[496] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(496)), 1.0))
                } else {
                    {
                        if (s.v[496] > s.v[691]) {
                            A::mul_offset_rhs(s.ad_value(692), A::sub(s.ad_value(496), s.ad_value(691)), 1.0)
                        } else {
                            A::exp(s.ad_value(496))
                        }
                    }
                }
            });
        }

        if ((s.b[2665] && s.b[2666]) && (!s.b[2668])) {
            s.store_mul_scaled_ad_rhs(504, 698, -1.0, A::offset(s.ad_value(497), (-1.0)));
        }

        if (s.b[2665] && s.b[2666]) {
            s.store_ad_value(849, A::add_scaled_inputs3(s.ad_value(502), 1.0, s.ad_value(503), 1.0, s.ad_value(504), 1.0));
            s.store_scalar(2669, 0.0);
            s.store_scalar(2670, 0.0);
            s.store_scaled_mul(2621, 657, 657, 4.0);
            s.store_div(2622, 657, 658);
            s.store_ad_value(2623, A::add_scaled_product(s.ad_value(832), 1.0, s.ad_value(657), s.ad_value(2622), 1.0));
            s.store_add(2624, 658, 2623);
            s.store_sub(2625, 658, 2623);
            s.store_sqrt_square_add(2626, 2625, 2621);
            s.store_ad_value(2670, A::div_scaled_product(s.ad_value(832), s.ad_value(658), 2.0, A::add(s.ad_value(2624), s.ad_value(2626)), 1.0));
        }

        s.b[2671] = (s.v[651] > 0.5);
        s.v[2671] = if s.b[2671] { 1.0 } else { 0.0 };

        s.b[2672] = (s.v[408] == 0.5);
        s.v[2672] = if s.b[2672] { 1.0 } else { 0.0 };

        if (((s.b[2665] && s.b[2666]) && s.b[2671]) && s.b[2672]) {
            s.store_sqrt_sub_from_scalar_ad(2669, 1.0, A::scale(s.ad_value(2670), s.v[405]));
        }

        if (((s.b[2665] && s.b[2666]) && s.b[2671]) && (!s.b[2672])) {
            s.store_powf_ad(2669, A::sub_from_scalar(1.0, A::scale(s.ad_value(2670), s.v[405])), s.v[408]);
        }

        if ((s.b[2665] && s.b[2666]) && s.b[2671]) {
            s.store_ad_value(1918, A::add_scaled_inputs3_offset(s.ad_value(2669), (-s.v[417]), s.ad_value(832), s.v[420], s.ad_value(2670), (-s.v[420]), s.v[417]));
        }

        s.b[2673] = (s.v[652] > 0.5);
        s.v[2673] = if s.b[2673] { 1.0 } else { 0.0 };

        s.b[2674] = (s.v[409] == 0.5);
        s.v[2674] = if s.b[2674] { 1.0 } else { 0.0 };

        if (((s.b[2665] && s.b[2666]) && s.b[2673]) && s.b[2674]) {
            s.store_sqrt_sub_from_scalar_ad(2669, 1.0, A::scale(s.ad_value(2670), s.v[406]));
        }

        if (((s.b[2665] && s.b[2666]) && s.b[2673]) && (!s.b[2674])) {
            s.store_powf_ad(2669, A::sub_from_scalar(1.0, A::scale(s.ad_value(2670), s.v[406])), s.v[409]);
        }

        if ((s.b[2665] && s.b[2666]) && s.b[2673]) {
            s.store_ad_value(1919, A::add_scaled_inputs3_offset(s.ad_value(2669), (-s.v[418]), s.ad_value(832), s.v[421], s.ad_value(2670), (-s.v[421]), s.v[418]));
        }

        s.b[2675] = (s.v[653] > 0.5);
        s.v[2675] = if s.b[2675] { 1.0 } else { 0.0 };

        s.b[2676] = (s.v[410] == 0.5);
        s.v[2676] = if s.b[2676] { 1.0 } else { 0.0 };

        if (((s.b[2665] && s.b[2666]) && s.b[2675]) && s.b[2676]) {
            s.store_sqrt_sub_from_scalar_ad(2669, 1.0, A::scale(s.ad_value(2670), s.v[407]));
        }

        if (((s.b[2665] && s.b[2666]) && s.b[2675]) && (!s.b[2676])) {
            s.store_powf_ad(2669, A::sub_from_scalar(1.0, A::scale(s.ad_value(2670), s.v[407])), s.v[410]);
        }

        if ((s.b[2665] && s.b[2666]) && s.b[2675]) {
            s.store_ad_value(1920, A::add_scaled_inputs3_offset(s.ad_value(2669), (-s.v[419]), s.ad_value(832), s.v[422], s.ad_value(2670), (-s.v[422]), s.v[419]));
        }

        if (s.b[2665] && s.b[2666]) {
            s.store_scalar(2669, 0.0);
            s.store_scalar(2670, 0.0);
            s.store_scaled_mul(2621, 684, 684, 4.0);
            s.store_div(2622, 684, 685);
            s.store_ad_value(2623, A::add_scaled_product(s.ad_value(833), 1.0, s.ad_value(684), s.ad_value(2622), 1.0));
            s.store_add(2624, 685, 2623);
            s.store_sub(2625, 685, 2623);
            s.store_sqrt_square_add(2626, 2625, 2621);
            s.store_ad_value(2670, A::div_scaled_product(s.ad_value(833), s.ad_value(685), 2.0, A::add(s.ad_value(2624), s.ad_value(2626)), 1.0));
        }

        s.b[2677] = (s.v[678] > 0.5);
        s.v[2677] = if s.b[2677] { 1.0 } else { 0.0 };

        s.b[2678] = (s.v[575] == 0.5);
        s.v[2678] = if s.b[2678] { 1.0 } else { 0.0 };

        if (((s.b[2665] && s.b[2666]) && s.b[2677]) && s.b[2678]) {
            s.store_sqrt_sub_from_scalar_ad(2669, 1.0, A::mul(s.ad_value(2670), s.ad_value(572)));
        }

        if (((s.b[2665] && s.b[2666]) && s.b[2677]) && (!s.b[2678])) {
            s.store_pow_ad(2669, A::sub_from_scalar(1.0, A::mul(s.ad_value(2670), s.ad_value(572))), s.ad_value(575));
        }

        if ((s.b[2665] && s.b[2666]) && s.b[2677]) {
            s.store_ad_value(1921, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(584), 1.0, s.ad_value(2669)), 1.0, s.ad_value(587), A::sub(s.ad_value(833), s.ad_value(2670)), 1.0));
        }

        s.b[2679] = (s.v[679] > 0.5);
        s.v[2679] = if s.b[2679] { 1.0 } else { 0.0 };

        s.b[2680] = (s.v[576] == 0.5);
        s.v[2680] = if s.b[2680] { 1.0 } else { 0.0 };

        if (((s.b[2665] && s.b[2666]) && s.b[2679]) && s.b[2680]) {
            s.store_sqrt_sub_from_scalar_ad(2669, 1.0, A::mul(s.ad_value(2670), s.ad_value(573)));
        }

        if (((s.b[2665] && s.b[2666]) && s.b[2679]) && (!s.b[2680])) {
            s.store_pow_ad(2669, A::sub_from_scalar(1.0, A::mul(s.ad_value(2670), s.ad_value(573))), s.ad_value(576));
        }

        if ((s.b[2665] && s.b[2666]) && s.b[2679]) {
            s.store_ad_value(1922, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(585), 1.0, s.ad_value(2669)), 1.0, s.ad_value(588), A::sub(s.ad_value(833), s.ad_value(2670)), 1.0));
        }

        s.b[2681] = (s.v[680] > 0.5);
        s.v[2681] = if s.b[2681] { 1.0 } else { 0.0 };

        s.b[2682] = (s.v[577] == 0.5);
        s.v[2682] = if s.b[2682] { 1.0 } else { 0.0 };

        if (((s.b[2665] && s.b[2666]) && s.b[2681]) && s.b[2682]) {
            s.store_sqrt_sub_from_scalar_ad(2669, 1.0, A::mul(s.ad_value(2670), s.ad_value(574)));
        }

        if (((s.b[2665] && s.b[2666]) && s.b[2681]) && (!s.b[2682])) {
            s.store_pow_ad(2669, A::sub_from_scalar(1.0, A::mul(s.ad_value(2670), s.ad_value(574))), s.ad_value(577));
        }

        if ((s.b[2665] && s.b[2666]) && s.b[2681]) {
            s.store_ad_value(1923, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(586), 1.0, s.ad_value(2669)), 1.0, s.ad_value(589), A::sub(s.ad_value(833), s.ad_value(2670)), 1.0));
        }

        s.b[2683] = (p.p872 > 0.0);
        s.v[2683] = if s.b[2683] { 1.0 } else { 0.0 };

        if ((s.b[2665] && (!s.b[2666])) && s.b[2683]) {
            s.store_scaled_offset_ad(642, A::powf(A::add_scaled_inputs3(s.ad_value(825), 0.5, s.ad_value(827), 0.5, A::sqrt(A::offset(A::mul(A::add(s.ad_value(825), s.ad_value(827)), A::add(s.ad_value(825), s.ad_value(827))), (0.001 * 0.001))), 0.5), p.p873), (-(((0.5 * 0.001)) as f64).powf(p.p873)), p.p872);
            s.store_offset(640, 642, p.p862);
            s.store_div_from_scalar(450, 1.0, 640);
            s.store_div_from_scalar_offset_scaled_input(453, s.v[453], 642, 1.0 / (p.p862), 1.0);
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2683])) {
            s.store_scalar(640, p.p862);
        }

        s.b[2684] = (p.p874 > 0.0);
        s.v[2684] = if s.b[2684] { 1.0 } else { 0.0 };

        if ((s.b[2665] && (!s.b[2666])) && s.b[2684]) {
            s.store_scaled_offset_ad(644, A::powf(A::add_scaled_inputs3(s.ad_value(825), 0.5, s.ad_value(827), 0.5, A::sqrt(A::offset(A::mul(A::add(s.ad_value(825), s.ad_value(827)), A::add(s.ad_value(825), s.ad_value(827))), (0.001 * 0.001))), 0.5), p.p875), (-(((0.5 * 0.001)) as f64).powf(p.p875)), p.p874);
            s.store_mul_offset_rhs(443, 443, 644, 1.0);
        }

        if (s.b[2665] && (!s.b[2666])) {
            s.store_scalar(2634, 0.0);
            s.store_scalar(2631, 0.0);
        }

        s.b[2685] = (!(((s.v[646] == 0.0) && (s.v[647] == 0.0)) && (s.v[648] == 0.0)));
        s.v[2685] = if s.b[2685] { 1.0 } else { 0.0 };

        if ((s.b[2665] && (!s.b[2666])) && s.b[2685]) {
            s.store_scaled_mul(2621, 657, 657, 4.0);
            s.store_div(2622, 657, 658);
            s.store_ad_value(2623, A::add_scaled_product(s.ad_value(832), 1.0, s.ad_value(657), s.ad_value(2622), 1.0));
            s.store_add(2624, 658, 2623);
            s.store_sub(2625, 658, 2623);
            s.store_sqrt_square_add(2626, 2625, 2621);
            s.store_ad_value(2628, A::div_scaled_product(s.ad_value(832), s.ad_value(658), 2.0, A::add(s.ad_value(2624), s.ad_value(2626)), 1.0));
        }

        s.b[2686] = (s.v[832] < s.v[654]);
        s.v[2686] = if s.b[2686] { 1.0 } else { 0.0 };

        s.b[2687] = (((((-0.5) * (s.v[832] * s.v[371]))) as f64).abs() < 230.25850929940458);
        s.v[2687] = if s.b[2687] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && s.b[2685]) && s.b[2686]) && s.b[2687]) {
            s.store_exp_scaled_input(2629, 832, (s.v[371] * (-0.5)));
        }

        s.b[2688] = (((-0.5) * (s.v[832] * s.v[371])) < 0.0);
        s.v[2688] = if s.b[2688] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && s.b[2685]) && s.b[2686]) && (!s.b[2687])) && s.b[2688]) {
            s.store_div_from_scalar_offset_ad(2629, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::scale(s.ad_value(832), (s.v[371] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(832), (s.v[371] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2665] && (!s.b[2666])) && s.b[2685]) && s.b[2686]) && (!s.b[2687])) && (!s.b[2688])) {
            s.store_scaled_offset_ad(2629, A::mul_offset_rhs(A::scale_offset(s.ad_value(832), (s.v[371] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(832), (s.v[371] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(832), (((s.v[371] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[2665] && (!s.b[2666])) && s.b[2685]) && s.b[2686]) {
            s.store_div_from_scalar(2630, 1.0, 2629);
            s.store_square(2627, 2630);
        }

        if (((s.b[2665] && (!s.b[2666])) && s.b[2685]) && (!s.b[2686])) {
            s.store_mul_offset_ad_lhs(2627, A::sub_scaled_inputs(s.ad_value(832), s.v[371], s.ad_value(654), s.v[371]), 1.0, 655);
            s.store_sqrt(2630, 2627);
            s.store_div_from_scalar(2629, 1.0, 2630);
        }

        if ((s.b[2665] && (!s.b[2666])) && s.b[2685]) {
            s.store_offset(2627, 2627, (-1.0));
        }

        s.b[2689] = (s.v[832] > 0.0);
        s.v[2689] = if s.b[2689] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && s.b[2685]) && s.b[2689]) {
            s.store_scaled_ln_ad(2631, A::add(A::offset(s.ad_value(2629), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2629), 1.0, A::offset(s.ad_value(2629), 3.0)))), (s.v[370] * 2.0));
        }

        if (((s.b[2665] && (!s.b[2666])) && s.b[2685]) && (!s.b[2689])) {
            s.store_sub_ad_lhs(2631, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(2630), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2630), 1.0, A::scale_offset(s.ad_value(2630), 3.0, 1.0))))), (s.v[370] * 2.0)), 832);
        }

        if ((s.b[2665] && (!s.b[2666])) && s.b[2685]) {
            s.store_sub(2632, 656, 2631);
            s.store_ad_value(2633, A::add_scaled_inputs3(s.ad_value(832), 0.5, s.ad_value(2632), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(832), s.ad_value(2632)), A::sub(s.ad_value(832), s.ad_value(2632))), ((4.0 * s.v[370]) * s.v[370]))), (-0.5)));
            s.store_ad_value(2634, A::add_scaled_inputs3(s.ad_value(832), 0.5, s.ad_value(659), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(832), s.ad_value(659)), A::sub(s.ad_value(832), s.ad_value(659))), ((4.0 * s.v[368]) * s.v[368]))), (-0.5)));
            s.store_scaled_sub_ad_rhs(2635, 832, A::sqrt(A::offset(A::mul(s.ad_value(832), s.ad_value(832)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[2690] = (s.v[646] == 0.0);
        s.v[2690] = if s.b[2690] { 1.0 } else { 0.0 };

        if ((s.b[2665] && (!s.b[2666])) && s.b[2690]) {
            s.store_scalar(1912, 0.0);
            s.store_scalar(1918, 0.0);
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) {
            s.store_scale(2637, 2627, s.v[387]);
        }

        s.b[2691] = ((p.p840 == 0.0) && (p.p845 == 0.0));
        s.v[2691] = if s.b[2691] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && s.b[2691]) {
            s.store_scalar(2638, 0.0);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) {
            s.store_sub_from_scalar(2639, s.v[393], 2633);
            s.store_sub_from_scalar_ad(2640, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2631), s.ad_value(2639)))));
        }

        s.b[2692] = (p.p831 == 0.5);
        s.v[2692] = if s.b[2692] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) && s.b[2692]) {
            s.store_scalar(2641, 0.0);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) && (!s.b[2692])) {
            s.store_scaled_add_ad_lhs(2641, A::div_scaled_product(A::square(s.ad_value(2640)), A::ln(s.ad_value(2640)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2640)), 1.0), 2640, (1.0 - (2.0 * p.p831)));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) {
            s.store_add(2642, 2640, 2641);
        }

        s.b[2693] = (p.p831 == 0.5);
        s.v[2693] = if s.b[2693] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) && s.b[2693]) {
            s.store_sqrt_scaled_input(2636, 2639, s.v[429]);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) && (!s.b[2693])) {
            s.store_powf_ad(2636, A::scale(s.ad_value(2639), s.v[429]), p.p831);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) {
            s.store_scale(2643, 2636, s.v[423]);
            s.store_ad_value(2644, A::mul_offset_lhs_scaled_output(s.ad_value(2630), (-1.0), s.ad_value(2643), s.v[384]));
            s.store_scaled_mul(2638, 2644, 2642, p.p840);
        }

        s.b[2694] = (p.p845 == 0.0);
        s.v[2694] = if s.b[2694] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && s.b[2694]) {
            s.store_scalar(2645, 0.0);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) {
            s.store_scaled_div(2646, 2643, 2639, (s.v[408] * s.v[438]));
            s.store_div_from_scalar(2647, (0.666666666666667 * s.v[435]), 2646);
            s.store_square(2648, 2647);
            s.store_sqrt_ad(2649, A::div_scaled_product(s.ad_value(2648), s.ad_value(2648), 1.0, A::offset(A::square(s.ad_value(2648)), 1.0), 1.0));
            s.store_sqrt(2650, 2649);
            s.store_mul(2651, 2649, 2650);
        }

        s.b[2695] = (((-p.p831) * s.v[411]) == (-1.0));
        s.v[2695] = if s.b[2695] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && s.b[2695]) {
            s.store_div_from_scalar_offset_ad(2652, 1.0, A::mul(s.ad_value(2646), s.ad_value(2651)), 1.0);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && (!s.b[2695])) {
            s.store_powf_ad(2652, A::offset(A::mul(s.ad_value(2646), s.ad_value(2651)), 1.0), ((-p.p831) * s.v[411]));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) {
            s.store_ad_value(2653, A::div_scaled_product(s.ad_value(2642), s.ad_value(2652), 1.0, A::add(s.ad_value(2642), s.ad_value(2652)), 1.0));
            s.store_sqrt_scaled_ad(2654, A::div(s.ad_value(2646), s.ad_value(2650)), 0.375);
            s.store_ad_value(2655, A::add_scaled_product(s.ad_value(2649), (-1.0), s.ad_value(2647), s.ad_value(2650), 2.0));
            s.store_ad_value(2656, A::add_scaled_product(A::add_scaled_product(s.ad_value(2649), (-s.v[435]), s.ad_value(2647), s.ad_value(2650), s.v[435]), 1.0, s.ad_value(2646), s.ad_value(2651), 0.5));
        }

    }

    pub(super) fn stamp_transient_block_40(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) {
            s.store_mul_offset_lhs(2657, 2655, (-1.0), 2654);
            s.store_square(2618, 2657);
        }

        s.b[2696] = (s.v[2657] > 0.0);
        s.v[2696] = if s.b[2696] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && s.b[2696]) {
            s.store_div_from_scalar_offset_scaled_input(2619, 1.0, 2657, s.v[372], 1.0);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && (!s.b[2696])) {
            s.store_div_from_scalar_sub_from_scalar_ad(2619, 1.0, 1.0, A::scale(s.ad_value(2657), s.v[372]));
        }

        s.b[2697] = (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458));
        s.v[2697] = if s.b[2697] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && s.b[2697]) {
            s.store_exp_sub(2636, 2656, 2618);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && (!s.b[2697])) {
            s.store_div_from_scalar_offset_ad(2636, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) {
            s.store_mul_ad_lhs(2620, A::add_scaled_inputs_product(s.ad_value(2619), 0.29214664, A::square(s.ad_value(2619)), s.v[373], A::square(s.ad_value(2619)), s.ad_value(2619), s.v[374]), 2636);
        }

        s.b[2698] = (s.v[2657] > 0.0);
        s.v[2698] = if s.b[2698] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && s.b[2698]) {
            s.copy_ad(2658, 2620);
        }

        s.b[2699] = (s.v[2656] > (-230.25850929940458));
        s.v[2699] = if s.b[2699] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && (!s.b[2698])) && s.b[2699]) {
            s.store_exp(2636, 2656);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && (!s.b[2698])) && (!s.b[2699])) {
            s.store_div_from_scalar_offset_ad(2636, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2656), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2656), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && (!s.b[2698])) {
            s.store_sub_scaled_inputs(2658, 2636, 2.0, 2620, 1.0);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) {
            s.store_scaled_div(2659, 2658, 2654, (s.v[435] * (1.772453850905516 * 0.5)));
            s.store_mul3_affine_lhs(2645, 2644, 2659, p.p845, 0.0, 2653);
        }

        s.b[2700] = (p.p851 == 0.0);
        s.v[2700] = if s.b[2700] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && s.b[2700]) {
            s.store_scalar(2660, 0.0);
        }

        s.b[2701] = (p.p831 == 0.5);
        s.v[2701] = if s.b[2701] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && s.b[2701]) {
            s.store_sqrt_scaled_ad(2636, A::sub_from_scalar(p.p828, s.ad_value(2634)), s.v[429]);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && (!s.b[2701])) {
            s.store_powf_ad(2636, A::scale(A::sub_from_scalar(p.p828, s.ad_value(2634)), s.v[429]), p.p831);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) {
            s.store_ad_value(2661, A::div_scaled_inputs(A::sub_from_scalar(p.p828, s.ad_value(2634)), (s.v[426] * s.v[411]), s.ad_value(2636), 1.0));
        }

        s.b[2702] = (((((-s.v[441]) / s.v[2661])) as f64).abs() < 230.25850929940458);
        s.v[2702] = if s.b[2702] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && s.b[2702]) {
            s.store_exp_ad(2636, A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(2661), 1.0));
        }

        s.b[2703] = (((-s.v[441]) / s.v[2661]) < 0.0);
        s.v[2703] = if s.b[2703] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && (!s.b[2702])) && s.b[2703]) {
            s.store_div_from_scalar_offset_ad(2636, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(2661), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(2661), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && (!s.b[2702])) && (!s.b[2703])) {
            let assign57450_ad_e72701: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(2661), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(2661), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(2661), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2636, assign57450_ad_e72701, 1e100);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) {
            s.store_mul_scaled_ad_lhs(2660, A::mul3(s.ad_value(832), s.ad_value(2661), s.ad_value(2661)), 2636, p.p851);
        }

        s.b[2704] = (p.p860 > 1000.0);
        s.v[2704] = if s.b[2704] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && s.b[2704]) {
            s.store_scalar(2662, 1.0);
        }

        s.b[2705] = (s.v[2635] > ((-s.v[444]) * p.p860));
        s.v[2705] = if s.b[2705] { 1.0 } else { 0.0 };

        s.b[2706] = (p.p863 == 4.0);
        s.v[2706] = if s.b[2706] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2704])) && s.b[2705]) && s.b[2706]) {
            s.store_mul_scaled_ad_lhs(2636, A::mul3_scaled_output(s.ad_value(2635), s.ad_value(2635), s.ad_value(2635), ((s.v[448] * s.v[448]) * s.v[448])), 2635, s.v[448]);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2704])) && s.b[2705]) && (!s.b[2706])) {
            s.store_powf_ad(2636, A::abs_scaled_input(s.ad_value(2635), s.v[448]), p.p863);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2704])) && s.b[2705]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2662, 1.0, 1.0, s.ad_value(2636));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2704])) && (!s.b[2705])) {
            s.store_offset_scaled(2662, 2635, s.v[451], (((((s.v[444] * p.p860)) * (s.v[451]))) + (s.v[445])));
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) {
            s.store_mul_scale_ad_lhs(1912, A::add(A::add_scaled_inputs3(s.ad_value(2637), 1.0, s.ad_value(2638), 1.0, s.ad_value(2645), 1.0), s.ad_value(2660)), p.p29, 2662);
        }

        s.b[2707] = (s.v[408] == 0.5);
        s.v[2707] = if s.b[2707] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && s.b[2707]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::scale(s.ad_value(2628), s.v[405]));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2707])) {
            s.store_powf_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2628), s.v[405])), s.v[408]);
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) {
            s.store_ad_value(1918, A::add_scaled_inputs3_offset(s.ad_value(2636), ((-s.v[417]) * p.p30), s.ad_value(832), (s.v[420] * p.p30), s.ad_value(2628), ((-s.v[420]) * p.p30), (s.v[417] * p.p30)));
        }

        s.b[2708] = (s.v[647] == 0.0);
        s.v[2708] = if s.b[2708] { 1.0 } else { 0.0 };

        if ((s.b[2665] && (!s.b[2666])) && s.b[2708]) {
            s.store_scalar(1913, 0.0);
            s.store_scalar(1919, 0.0);
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) {
            s.store_scale(2637, 2627, s.v[388]);
        }

        s.b[2709] = ((p.p841 == 0.0) && (p.p846 == 0.0));
        s.v[2709] = if s.b[2709] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && s.b[2709]) {
            s.store_scalar(2638, 0.0);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2709])) {
            s.store_sub_from_scalar(2639, s.v[394], 2633);
            s.store_sub_from_scalar_ad(2640, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2631), s.ad_value(2639)))));
        }

        s.b[2710] = (p.p832 == 0.5);
        s.v[2710] = if s.b[2710] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2709])) && s.b[2710]) {
            s.store_scalar(2641, 0.0);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2709])) && (!s.b[2710])) {
            s.store_scaled_add_ad_lhs(2641, A::div_scaled_product(A::square(s.ad_value(2640)), A::ln(s.ad_value(2640)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2640)), 1.0), 2640, (1.0 - (2.0 * p.p832)));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2709])) {
            s.store_add(2642, 2640, 2641);
        }

        s.b[2711] = (p.p832 == 0.5);
        s.v[2711] = if s.b[2711] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2709])) && s.b[2711]) {
            s.store_sqrt_scaled_input(2636, 2639, s.v[430]);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2709])) && (!s.b[2711])) {
            s.store_powf_ad(2636, A::scale(s.ad_value(2639), s.v[430]), p.p832);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2709])) {
            s.store_scale(2643, 2636, s.v[424]);
            s.store_ad_value(2644, A::mul_offset_lhs_scaled_output(s.ad_value(2630), (-1.0), s.ad_value(2643), s.v[385]));
            s.store_scaled_mul(2638, 2644, 2642, p.p841);
        }

        s.b[2712] = (p.p846 == 0.0);
        s.v[2712] = if s.b[2712] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && s.b[2712]) {
            s.store_scalar(2645, 0.0);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) {
            s.store_scaled_div(2646, 2643, 2639, (s.v[409] * s.v[439]));
            s.store_div_from_scalar(2647, (0.666666666666667 * s.v[436]), 2646);
            s.store_square(2648, 2647);
            s.store_sqrt_ad(2649, A::div_scaled_product(s.ad_value(2648), s.ad_value(2648), 1.0, A::offset(A::square(s.ad_value(2648)), 1.0), 1.0));
            s.store_sqrt(2650, 2649);
            s.store_mul(2651, 2649, 2650);
        }

        s.b[2713] = (((-p.p832) * s.v[412]) == (-1.0));
        s.v[2713] = if s.b[2713] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) && s.b[2713]) {
            s.store_div_from_scalar_offset_ad(2652, 1.0, A::mul(s.ad_value(2646), s.ad_value(2651)), 1.0);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) && (!s.b[2713])) {
            s.store_powf_ad(2652, A::offset(A::mul(s.ad_value(2646), s.ad_value(2651)), 1.0), ((-p.p832) * s.v[412]));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) {
            s.store_ad_value(2653, A::div_scaled_product(s.ad_value(2642), s.ad_value(2652), 1.0, A::add(s.ad_value(2642), s.ad_value(2652)), 1.0));
            s.store_sqrt_scaled_ad(2654, A::div(s.ad_value(2646), s.ad_value(2650)), 0.375);
            s.store_ad_value(2655, A::add_scaled_product(s.ad_value(2649), (-1.0), s.ad_value(2647), s.ad_value(2650), 2.0));
            s.store_ad_value(2656, A::add_scaled_product(A::add_scaled_product(s.ad_value(2649), (-s.v[436]), s.ad_value(2647), s.ad_value(2650), s.v[436]), 1.0, s.ad_value(2646), s.ad_value(2651), 0.5));
            s.store_mul_offset_lhs(2657, 2655, (-1.0), 2654);
            s.store_square(2618, 2657);
        }

        s.b[2714] = (s.v[2657] > 0.0);
        s.v[2714] = if s.b[2714] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) && s.b[2714]) {
            s.store_div_from_scalar_offset_scaled_input(2619, 1.0, 2657, s.v[372], 1.0);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) && (!s.b[2714])) {
            s.store_div_from_scalar_sub_from_scalar_ad(2619, 1.0, 1.0, A::scale(s.ad_value(2657), s.v[372]));
        }

        s.b[2715] = (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458));
        s.v[2715] = if s.b[2715] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) && s.b[2715]) {
            s.store_exp_sub(2636, 2656, 2618);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) && (!s.b[2715])) {
            s.store_div_from_scalar_offset_ad(2636, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) {
            s.store_mul_ad_lhs(2620, A::add_scaled_inputs_product(s.ad_value(2619), 0.29214664, A::square(s.ad_value(2619)), s.v[373], A::square(s.ad_value(2619)), s.ad_value(2619), s.v[374]), 2636);
        }

        s.b[2716] = (s.v[2657] > 0.0);
        s.v[2716] = if s.b[2716] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) && s.b[2716]) {
            s.copy_ad(2658, 2620);
        }

        s.b[2717] = (s.v[2656] > (-230.25850929940458));
        s.v[2717] = if s.b[2717] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) && (!s.b[2716])) && s.b[2717]) {
            s.store_exp(2636, 2656);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) && (!s.b[2716])) && (!s.b[2717])) {
            s.store_div_from_scalar_offset_ad(2636, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2656), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2656), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) && (!s.b[2716])) {
            s.store_sub_scaled_inputs(2658, 2636, 2.0, 2620, 1.0);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) {
            s.store_scaled_div(2659, 2658, 2654, (s.v[436] * (1.772453850905516 * 0.5)));
            s.store_mul3_affine_lhs(2645, 2644, 2659, p.p846, 0.0, 2653);
        }

        s.b[2718] = (p.p852 == 0.0);
        s.v[2718] = if s.b[2718] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && s.b[2718]) {
            s.store_scalar(2660, 0.0);
        }

        s.b[2719] = (p.p832 == 0.5);
        s.v[2719] = if s.b[2719] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) && s.b[2719]) {
            s.store_sqrt_scaled_ad(2636, A::sub_from_scalar(p.p829, s.ad_value(2634)), s.v[430]);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) && (!s.b[2719])) {
            s.store_powf_ad(2636, A::scale(A::sub_from_scalar(p.p829, s.ad_value(2634)), s.v[430]), p.p832);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) {
            s.store_ad_value(2661, A::div_scaled_inputs(A::sub_from_scalar(p.p829, s.ad_value(2634)), (s.v[427] * s.v[412]), s.ad_value(2636), 1.0));
        }

        s.b[2720] = (((((-s.v[442]) / s.v[2661])) as f64).abs() < 230.25850929940458);
        s.v[2720] = if s.b[2720] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) && s.b[2720]) {
            s.store_exp_ad(2636, A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(2661), 1.0));
        }

        s.b[2721] = (((-s.v[442]) / s.v[2661]) < 0.0);
        s.v[2721] = if s.b[2721] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) && (!s.b[2720])) && s.b[2721]) {
            s.store_div_from_scalar_offset_ad(2636, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(2661), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(2661), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) && (!s.b[2720])) && (!s.b[2721])) {
            let assign58200_ad_e73967: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(2661), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(2661), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(2661), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2636, assign58200_ad_e73967, 1e100);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) {
            s.store_mul_scaled_ad_lhs(2660, A::mul3(s.ad_value(832), s.ad_value(2661), s.ad_value(2661)), 2636, p.p852);
        }

        s.b[2722] = (p.p861 > 1000.0);
        s.v[2722] = if s.b[2722] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && s.b[2722]) {
            s.store_scalar(2662, 1.0);
        }

        s.b[2723] = (s.v[2635] > ((-s.v[444]) * p.p861));
        s.v[2723] = if s.b[2723] { 1.0 } else { 0.0 };

        s.b[2724] = (p.p864 == 4.0);
        s.v[2724] = if s.b[2724] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2722])) && s.b[2723]) && s.b[2724]) {
            s.store_mul_scaled_ad_lhs(2636, A::mul3_scaled_output(s.ad_value(2635), s.ad_value(2635), s.ad_value(2635), ((s.v[449] * s.v[449]) * s.v[449])), 2635, s.v[449]);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2722])) && s.b[2723]) && (!s.b[2724])) {
            s.store_powf_ad(2636, A::abs_scaled_input(s.ad_value(2635), s.v[449]), p.p864);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2722])) && s.b[2723]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2662, 1.0, 1.0, s.ad_value(2636));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2722])) && (!s.b[2723])) {
            s.store_offset_scaled(2662, 2635, s.v[452], (((((s.v[444] * p.p861)) * (s.v[452]))) + (s.v[446])));
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) {
            s.store_mul_scale_ad_lhs(1913, A::add(A::add_scaled_inputs3(s.ad_value(2637), 1.0, s.ad_value(2638), 1.0, s.ad_value(2645), 1.0), s.ad_value(2660)), p.p29, 2662);
        }

        s.b[2725] = (s.v[409] == 0.5);
        s.v[2725] = if s.b[2725] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && s.b[2725]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::scale(s.ad_value(2628), s.v[406]));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2725])) {
            s.store_powf_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2628), s.v[406])), s.v[409]);
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) {
            s.store_ad_value(1919, A::add_scaled_inputs3_offset(s.ad_value(2636), ((-s.v[418]) * p.p30), s.ad_value(832), (s.v[421] * p.p30), s.ad_value(2628), ((-s.v[421]) * p.p30), (s.v[418] * p.p30)));
        }

        s.b[2726] = (s.v[648] == 0.0);
        s.v[2726] = if s.b[2726] { 1.0 } else { 0.0 };

        if ((s.b[2665] && (!s.b[2666])) && s.b[2726]) {
            s.store_scalar(1914, 0.0);
            s.store_scalar(1920, 0.0);
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) {
            s.store_scale(2637, 2627, s.v[389]);
        }

        s.b[2727] = ((p.p842 == 0.0) && (p.p847 == 0.0));
        s.v[2727] = if s.b[2727] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2727]) {
            s.store_scalar(2638, 0.0);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2727])) {
            s.store_sub_from_scalar(2639, s.v[395], 2633);
            s.store_sub_from_scalar_ad(2640, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2631), s.ad_value(2639)))));
        }

        s.b[2728] = (p.p833 == 0.5);
        s.v[2728] = if s.b[2728] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2727])) && s.b[2728]) {
            s.store_scalar(2641, 0.0);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2727])) && (!s.b[2728])) {
            s.store_scaled_add_ad_lhs(2641, A::div_scaled_product(A::square(s.ad_value(2640)), A::ln(s.ad_value(2640)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2640)), 1.0), 2640, (1.0 - (2.0 * p.p833)));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2727])) {
            s.store_add(2642, 2640, 2641);
        }

        s.b[2729] = (p.p833 == 0.5);
        s.v[2729] = if s.b[2729] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2727])) && s.b[2729]) {
            s.store_sqrt_scaled_input(2636, 2639, s.v[431]);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2727])) && (!s.b[2729])) {
            s.store_powf_ad(2636, A::scale(s.ad_value(2639), s.v[431]), p.p833);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2727])) {
            s.store_scale(2643, 2636, s.v[425]);
            s.store_ad_value(2644, A::mul_offset_lhs_scaled_output(s.ad_value(2630), (-1.0), s.ad_value(2643), s.v[386]));
            s.store_scaled_mul(2638, 2644, 2642, p.p842);
        }

        s.b[2730] = (p.p847 == 0.0);
        s.v[2730] = if s.b[2730] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_41(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2730]) {
            s.store_scalar(2645, 0.0);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) {
            s.store_scaled_div(2646, 2643, 2639, (s.v[410] * s.v[440]));
            s.store_div_from_scalar(2647, (0.666666666666667 * s.v[437]), 2646);
            s.store_square(2648, 2647);
            s.store_sqrt_ad(2649, A::div_scaled_product(s.ad_value(2648), s.ad_value(2648), 1.0, A::offset(A::square(s.ad_value(2648)), 1.0), 1.0));
            s.store_sqrt(2650, 2649);
            s.store_mul(2651, 2649, 2650);
        }

        s.b[2731] = (((-p.p833) * s.v[413]) == (-1.0));
        s.v[2731] = if s.b[2731] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) && s.b[2731]) {
            s.store_div_from_scalar_offset_ad(2652, 1.0, A::mul(s.ad_value(2646), s.ad_value(2651)), 1.0);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) && (!s.b[2731])) {
            s.store_powf_ad(2652, A::offset(A::mul(s.ad_value(2646), s.ad_value(2651)), 1.0), ((-p.p833) * s.v[413]));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) {
            s.store_ad_value(2653, A::div_scaled_product(s.ad_value(2642), s.ad_value(2652), 1.0, A::add(s.ad_value(2642), s.ad_value(2652)), 1.0));
            s.store_sqrt_scaled_ad(2654, A::div(s.ad_value(2646), s.ad_value(2650)), 0.375);
            s.store_ad_value(2655, A::add_scaled_product(s.ad_value(2649), (-1.0), s.ad_value(2647), s.ad_value(2650), 2.0));
            s.store_ad_value(2656, A::add_scaled_product(A::add_scaled_product(s.ad_value(2649), (-s.v[437]), s.ad_value(2647), s.ad_value(2650), s.v[437]), 1.0, s.ad_value(2646), s.ad_value(2651), 0.5));
            s.store_mul_offset_lhs(2657, 2655, (-1.0), 2654);
            s.store_square(2618, 2657);
        }

        s.b[2732] = (s.v[2657] > 0.0);
        s.v[2732] = if s.b[2732] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) && s.b[2732]) {
            s.store_div_from_scalar_offset_scaled_input(2619, 1.0, 2657, s.v[372], 1.0);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) && (!s.b[2732])) {
            s.store_div_from_scalar_sub_from_scalar_ad(2619, 1.0, 1.0, A::scale(s.ad_value(2657), s.v[372]));
        }

        s.b[2733] = (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458));
        s.v[2733] = if s.b[2733] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) && s.b[2733]) {
            s.store_exp_sub(2636, 2656, 2618);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) && (!s.b[2733])) {
            s.store_div_from_scalar_offset_ad(2636, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) {
            s.store_mul_ad_lhs(2620, A::add_scaled_inputs_product(s.ad_value(2619), 0.29214664, A::square(s.ad_value(2619)), s.v[373], A::square(s.ad_value(2619)), s.ad_value(2619), s.v[374]), 2636);
        }

        s.b[2734] = (s.v[2657] > 0.0);
        s.v[2734] = if s.b[2734] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) && s.b[2734]) {
            s.copy_ad(2658, 2620);
        }

        s.b[2735] = (s.v[2656] > (-230.25850929940458));
        s.v[2735] = if s.b[2735] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) && (!s.b[2734])) && s.b[2735]) {
            s.store_exp(2636, 2656);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) && (!s.b[2734])) && (!s.b[2735])) {
            s.store_div_from_scalar_offset_ad(2636, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2656), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2656), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) && (!s.b[2734])) {
            s.store_sub_scaled_inputs(2658, 2636, 2.0, 2620, 1.0);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) {
            s.store_scaled_div(2659, 2658, 2654, (s.v[437] * (1.772453850905516 * 0.5)));
            s.store_mul3_affine_lhs(2645, 2644, 2659, p.p847, 0.0, 2653);
        }

        s.b[2736] = (p.p853 == 0.0);
        s.v[2736] = if s.b[2736] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2736]) {
            s.store_scalar(2660, 0.0);
        }

        s.b[2737] = (p.p833 == 0.5);
        s.v[2737] = if s.b[2737] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) && s.b[2737]) {
            s.store_sqrt_scaled_ad(2636, A::sub_from_scalar(p.p830, s.ad_value(2634)), s.v[431]);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) && (!s.b[2737])) {
            s.store_powf_ad(2636, A::scale(A::sub_from_scalar(p.p830, s.ad_value(2634)), s.v[431]), p.p833);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) {
            s.store_ad_value(2661, A::div_scaled_inputs(A::sub_from_scalar(p.p830, s.ad_value(2634)), (s.v[428] * s.v[413]), s.ad_value(2636), 1.0));
        }

        s.b[2738] = (((((-s.v[443]) / s.v[2661])) as f64).abs() < 230.25850929940458);
        s.v[2738] = if s.b[2738] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) && s.b[2738]) {
            s.store_exp_ad(2636, A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(2661), 1.0));
        }

        s.b[2739] = (((-s.v[443]) / s.v[2661]) < 0.0);
        s.v[2739] = if s.b[2739] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) && (!s.b[2738])) && s.b[2739]) {
            s.store_div_from_scalar_offset_ad(2636, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(2661), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(2661), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) && (!s.b[2738])) && (!s.b[2739])) {
            let assign58950_ad_e75233: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(2661), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(2661), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(2661), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2636, assign58950_ad_e75233, 1e100);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) {
            s.store_mul_scaled_ad_lhs(2660, A::mul3(s.ad_value(832), s.ad_value(2661), s.ad_value(2661)), 2636, p.p853);
        }

        s.b[2740] = (s.v[640] > 1000.0);
        s.v[2740] = if s.b[2740] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2740]) {
            s.store_scalar(2662, 1.0);
        }

        s.b[2741] = (s.v[2635] > ((-s.v[444]) * s.v[640]));
        s.v[2741] = if s.b[2741] { 1.0 } else { 0.0 };

        s.b[2742] = (p.p865 == 4.0);
        s.v[2742] = if s.b[2742] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2740])) && s.b[2741]) && s.b[2742]) {
            s.store_mul_ad_product_lhs(2636, A::mul3(A::mul3(s.ad_value(2635), s.ad_value(450), A::mul(s.ad_value(2635), s.ad_value(450))), s.ad_value(2635), s.ad_value(450)), s.ad_value(2635), 450);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2740])) && s.b[2741]) && (!s.b[2742])) {
            s.store_powf_ad(2636, A::abs(A::mul(s.ad_value(2635), s.ad_value(450))), p.p865);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2740])) && s.b[2741]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2662, 1.0, 1.0, s.ad_value(2636));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2740])) && (!s.b[2741])) {
            s.store_offset_mul_ad(2662, A::add_scaled_inputs(s.ad_value(2635), 1.0, s.ad_value(640), s.v[444]), s.ad_value(453), s.v[447]);
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) {
            s.store_mul_scale_ad_lhs(1914, A::add(A::add_scaled_inputs3(s.ad_value(2637), 1.0, s.ad_value(2638), 1.0, s.ad_value(2645), 1.0), s.ad_value(2660)), p.p29, 2662);
        }

        s.b[2743] = (s.v[473] == 1.0);
        s.v[2743] = if s.b[2743] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) {
            let assign59070_ad_e75459: A = {
                if (s.v[832] < p.p870) {
                    {
                        if (((s.v[832] - p.p870) / p.p871) < (-37.0)) {
                            A::constant(p.p870)
                        } else {
                            A::scale_offset(A::ln_one_plus_exp(A::scaled_offset(s.ad_value(832), (-p.p870), 1.0 / (p.p871))), p.p871, p.p870)
                        }
                    }
                } else {
                    {
                        if (((s.v[832] - p.p870) / p.p871) > 37.0) {
                            s.ad_value(832)
                        } else {
                            A::add_scaled_inputs(s.ad_value(832), 1.0, A::ln_one_plus_exp(A::scale(A::sub_from_scalar(p.p870, s.ad_value(832)), 1.0 / (p.p871))), p.p871)
                        }
                    }
                }
            };
            s.store_ad_value(2663, assign59070_ad_e75459);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) {
            s.store_scaled_mul(2621, 657, 657, 4.0);
            s.store_div(2622, 657, 658);
            s.store_ad_value(2623, A::add_scaled_product(s.ad_value(2663), 1.0, s.ad_value(657), s.ad_value(2622), 1.0));
            s.store_add(2624, 658, 2623);
            s.store_sub(2625, 658, 2623);
            s.store_sqrt_square_add(2626, 2625, 2621);
            s.store_ad_value(2664, A::div_scaled_product(s.ad_value(2663), s.ad_value(658), 2.0, A::add(s.ad_value(2624), s.ad_value(2626)), 1.0));
        }

        s.b[2744] = (s.v[410] == 0.5);
        s.v[2744] = if s.b[2744] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) && s.b[2744]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::scale(s.ad_value(2664), s.v[407]));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) && (!s.b[2744])) {
            s.store_powf_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2664), s.v[407])), s.v[410]);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) {
            s.store_ad_value(1920, A::add_scaled_inputs3_offset(s.ad_value(2636), ((-s.v[419]) * p.p30), s.ad_value(2663), (s.v[422] * p.p30), s.ad_value(2664), ((-s.v[422]) * p.p30), (s.v[419] * p.p30)));
            s.store_sub_ad_lhs(2663, A::offset(s.ad_value(832), p.p870), 2663);
            s.store_scaled_mul(2621, 657, 657, 4.0);
            s.store_div(2622, 657, 658);
            s.store_ad_value(2623, A::add_scaled_product(s.ad_value(2663), 1.0, s.ad_value(657), s.ad_value(2622), 1.0));
            s.store_add(2624, 658, 2623);
            s.store_sub(2625, 658, 2623);
            s.store_sqrt_square_add(2626, 2625, 2621);
            s.store_ad_value(2664, A::div_scaled_product(s.ad_value(2663), s.ad_value(658), 2.0, A::add(s.ad_value(2624), s.ad_value(2626)), 1.0));
        }

        s.b[2745] = (s.v[467] == 0.5);
        s.v[2745] = if s.b[2745] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) && s.b[2745]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2664), s.ad_value(466)));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) && (!s.b[2745])) {
            s.store_pow_ad(2636, A::sub_from_scalar(1.0, A::mul(s.ad_value(2664), s.ad_value(466))), s.ad_value(467));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) {
            s.store_ad_value(472, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(470), 1.0, s.ad_value(2636)), p.p30, s.ad_value(471), A::sub(s.ad_value(2663), s.ad_value(2664)), p.p30));
            s.store_add(1920, 1920, 472);
        }

        s.b[2746] = (s.v[410] == 0.5);
        s.v[2746] = if s.b[2746] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2743])) && s.b[2746]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::scale(s.ad_value(2628), s.v[407]));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2743])) && (!s.b[2746])) {
            s.store_powf_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2628), s.v[407])), s.v[410]);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2743])) {
            s.store_ad_value(1920, A::add_scaled_inputs3_offset(s.ad_value(2636), ((-s.v[419]) * p.p30), s.ad_value(832), (s.v[422] * p.p30), s.ad_value(2628), ((-s.v[422]) * p.p30), (s.v[419] * p.p30)));
        }

        if (s.b[2665] && (!s.b[2666])) {
            s.store_ad_value(848, A::add_scaled_product(A::add_scaled_products(s.ad_value(646), s.ad_value(1912), 1.0, s.ad_value(647), s.ad_value(1913), 1.0), 1.0, s.ad_value(648), s.ad_value(1914), 1.0));
        }

        s.b[2747] = (s.v[636] > 0.0);
        s.v[2747] = if s.b[2747] { 1.0 } else { 0.0 };

        if ((s.b[2665] && (!s.b[2666])) && s.b[2747]) {
            s.store_mul_sub_ad_rhs(643, 636, A::pow(A::add_scaled_inputs3(s.ad_value(825), 0.5, s.ad_value(827), 0.5, A::sqrt(A::offset(A::mul(A::add(s.ad_value(825), s.ad_value(827)), A::add(s.ad_value(825), s.ad_value(827))), (0.001 * 0.001))), 0.5), s.ad_value(637)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(637)));
            s.store_add(641, 542, 643);
            s.store_div_from_scalar(616, 1.0, 641);
            s.store_div_ad_rhs(619, 619, A::offset(A::div(s.ad_value(643), s.ad_value(542)), 1.0));
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2747])) {
            s.copy_ad(641, 542);
        }

        s.b[2748] = (s.v[638] > 0.0);
        s.v[2748] = if s.b[2748] { 1.0 } else { 0.0 };

        if ((s.b[2665] && (!s.b[2666])) && s.b[2748]) {
            s.store_mul_sub_ad_rhs(645, 638, A::pow(A::add_scaled_inputs3(s.ad_value(825), 0.5, s.ad_value(827), 0.5, A::sqrt(A::offset(A::mul(A::add(s.ad_value(825), s.ad_value(827)), A::add(s.ad_value(825), s.ad_value(827))), (0.001 * 0.001))), 0.5), s.ad_value(639)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(639)));
            s.store_mul_offset_rhs(610, 610, 645, 1.0);
        }

        if (s.b[2665] && (!s.b[2666])) {
            s.store_scalar(2634, 0.0);
            s.store_scalar(2631, 0.0);
        }

        s.b[2749] = (!(((s.v[673] == 0.0) && (s.v[674] == 0.0)) && (s.v[675] == 0.0)));
        s.v[2749] = if s.b[2749] { 1.0 } else { 0.0 };

        if ((s.b[2665] && (!s.b[2666])) && s.b[2749]) {
            s.store_scaled_mul(2621, 684, 684, 4.0);
            s.store_div(2622, 684, 685);
            s.store_ad_value(2623, A::add_scaled_product(s.ad_value(833), 1.0, s.ad_value(684), s.ad_value(2622), 1.0));
            s.store_add(2624, 685, 2623);
            s.store_sub(2625, 685, 2623);
            s.store_sqrt_square_add(2626, 2625, 2621);
            s.store_ad_value(2628, A::div_scaled_product(s.ad_value(833), s.ad_value(685), 2.0, A::add(s.ad_value(2624), s.ad_value(2626)), 1.0));
        }

        s.b[2750] = (s.v[833] < s.v[681]);
        s.v[2750] = if s.b[2750] { 1.0 } else { 0.0 };

        s.b[2751] = (((((-0.5) * (s.v[833] * s.v[371]))) as f64).abs() < 230.25850929940458);
        s.v[2751] = if s.b[2751] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && s.b[2749]) && s.b[2750]) && s.b[2751]) {
            s.store_exp_scaled_input(2629, 833, (s.v[371] * (-0.5)));
        }

        s.b[2752] = (((-0.5) * (s.v[833] * s.v[371])) < 0.0);
        s.v[2752] = if s.b[2752] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && s.b[2749]) && s.b[2750]) && (!s.b[2751])) && s.b[2752]) {
            s.store_div_from_scalar_offset_ad(2629, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::scale(s.ad_value(833), (s.v[371] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(833), (s.v[371] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2665] && (!s.b[2666])) && s.b[2749]) && s.b[2750]) && (!s.b[2751])) && (!s.b[2752])) {
            s.store_scaled_offset_ad(2629, A::mul_offset_rhs(A::scale_offset(s.ad_value(833), (s.v[371] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(833), (s.v[371] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(833), (((s.v[371] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[2665] && (!s.b[2666])) && s.b[2749]) && s.b[2750]) {
            s.store_div_from_scalar(2630, 1.0, 2629);
            s.store_square(2627, 2630);
        }

        if (((s.b[2665] && (!s.b[2666])) && s.b[2749]) && (!s.b[2750])) {
            s.store_mul_offset_ad_lhs(2627, A::sub_scaled_inputs(s.ad_value(833), s.v[371], s.ad_value(681), s.v[371]), 1.0, 682);
            s.store_sqrt(2630, 2627);
            s.store_div_from_scalar(2629, 1.0, 2630);
        }

        if ((s.b[2665] && (!s.b[2666])) && s.b[2749]) {
            s.store_offset(2627, 2627, (-1.0));
        }

        s.b[2753] = (s.v[833] > 0.0);
        s.v[2753] = if s.b[2753] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && s.b[2749]) && s.b[2753]) {
            s.store_scaled_ln_ad(2631, A::add(A::offset(s.ad_value(2629), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2629), 1.0, A::offset(s.ad_value(2629), 3.0)))), (s.v[370] * 2.0));
        }

        if (((s.b[2665] && (!s.b[2666])) && s.b[2749]) && (!s.b[2753])) {
            s.store_sub_ad_lhs(2631, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(2630), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2630), 1.0, A::scale_offset(s.ad_value(2630), 3.0, 1.0))))), (s.v[370] * 2.0)), 833);
        }

        if ((s.b[2665] && (!s.b[2666])) && s.b[2749]) {
            s.store_sub(2632, 683, 2631);
            s.store_ad_value(2633, A::add_scaled_inputs3(s.ad_value(833), 0.5, s.ad_value(2632), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(833), s.ad_value(2632)), A::sub(s.ad_value(833), s.ad_value(2632))), ((4.0 * s.v[370]) * s.v[370]))), (-0.5)));
            s.store_ad_value(2634, A::add_scaled_inputs3(s.ad_value(833), 0.5, s.ad_value(686), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(833), s.ad_value(686)), A::sub(s.ad_value(833), s.ad_value(686))), ((4.0 * s.v[368]) * s.v[368]))), (-0.5)));
            s.store_scaled_sub_ad_rhs(2635, 833, A::sqrt(A::offset(A::mul(s.ad_value(833), s.ad_value(833)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[2754] = (s.v[673] == 0.0);
        s.v[2754] = if s.b[2754] { 1.0 } else { 0.0 };

        if ((s.b[2665] && (!s.b[2666])) && s.b[2754]) {
            s.store_scalar(1915, 0.0);
            s.store_scalar(1921, 0.0);
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) {
            s.store_mul(2637, 563, 2627);
        }

        s.b[2755] = ((s.v[522] == 0.0) && (s.v[525] == 0.0));
        s.v[2755] = if s.b[2755] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && s.b[2755]) {
            s.store_scalar(2638, 0.0);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2755])) {
            s.store_sub(2639, 569, 2633);
            s.store_sub_from_scalar_ad(2640, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2631), s.ad_value(2639)))));
        }

    }

    pub(super) fn stamp_transient_block_42(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2756] = (s.v[511] == 0.5);
        s.v[2756] = if s.b[2756] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2755])) && s.b[2756]) {
            s.store_scalar(2641, 0.0);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2755])) && (!s.b[2756])) {
            s.store_ad_value(2641, A::mul_sub_from_scalar_rhs(A::add(A::div_scaled_product(A::square(s.ad_value(2640)), A::ln(s.ad_value(2640)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2640)), 1.0), s.ad_value(2640)), 1.0, A::scale(s.ad_value(511), 2.0)));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2755])) {
            s.store_add(2642, 2640, 2641);
        }

        s.b[2757] = (s.v[511] == 0.5);
        s.v[2757] = if s.b[2757] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2755])) && s.b[2757]) {
            s.store_sqrt_mul(2636, 2639, 596);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2755])) && (!s.b[2757])) {
            s.store_pow_ad(2636, A::mul(s.ad_value(2639), s.ad_value(596)), s.ad_value(511));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2755])) {
            s.store_mul(2643, 590, 2636);
            s.store_mul_ad_product_lhs(2644, s.ad_value(560), A::offset(s.ad_value(2630), (-1.0)), 2643);
            s.store_mul3_lhs(2638, 522, 2644, 2642);
        }

        s.b[2758] = (s.v[525] == 0.0);
        s.v[2758] = if s.b[2758] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && s.b[2758]) {
            s.store_scalar(2645, 0.0);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) {
            s.store_mul_ad_rhs(2646, 605, A::div_scaled_product(s.ad_value(2643), s.ad_value(575), 1.0, s.ad_value(2639), 1.0));
            s.store_scaled_div(2647, 602, 2646, 0.666666666666667);
            s.store_square(2648, 2647);
            s.store_sqrt_ad(2649, A::div_scaled_product(s.ad_value(2648), s.ad_value(2648), 1.0, A::offset(A::square(s.ad_value(2648)), 1.0), 1.0));
            s.store_sqrt(2650, 2649);
            s.store_mul(2651, 2649, 2650);
        }

        s.b[2759] = (((-s.v[511]) * s.v[578]) == (-1.0));
        s.v[2759] = if s.b[2759] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) && s.b[2759]) {
            s.store_div_from_scalar_offset_ad(2652, 1.0, A::mul(s.ad_value(2646), s.ad_value(2651)), 1.0);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) && (!s.b[2759])) {
            s.store_pow_ad(2652, A::offset(A::mul(s.ad_value(2646), s.ad_value(2651)), 1.0), A::mul_scaled_lhs(s.ad_value(511), -1.0, s.ad_value(578)));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) {
            s.store_ad_value(2653, A::div_scaled_product(s.ad_value(2642), s.ad_value(2652), 1.0, A::add(s.ad_value(2642), s.ad_value(2652)), 1.0));
            s.store_sqrt_scaled_ad(2654, A::div(s.ad_value(2646), s.ad_value(2650)), 0.375);
            s.store_ad_value(2655, A::add_scaled_product(s.ad_value(2649), (-1.0), s.ad_value(2647), s.ad_value(2650), 2.0));
            s.store_ad_value(2656, A::add_scaled_product(A::add_scaled_product(A::mul3(s.ad_value(602), s.ad_value(2647), s.ad_value(2650)), 1.0, s.ad_value(602), s.ad_value(2649), (-1.0)), 1.0, s.ad_value(2646), s.ad_value(2651), 0.5));
            s.store_mul_offset_lhs(2657, 2655, (-1.0), 2654);
            s.store_square(2618, 2657);
        }

        s.b[2760] = (s.v[2657] > 0.0);
        s.v[2760] = if s.b[2760] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) && s.b[2760]) {
            s.store_div_from_scalar_offset_scaled_input(2619, 1.0, 2657, s.v[372], 1.0);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) && (!s.b[2760])) {
            s.store_div_from_scalar_sub_from_scalar_ad(2619, 1.0, 1.0, A::scale(s.ad_value(2657), s.v[372]));
        }

        s.b[2761] = (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458));
        s.v[2761] = if s.b[2761] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) && s.b[2761]) {
            s.store_exp_sub(2636, 2656, 2618);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) && (!s.b[2761])) {
            s.store_div_from_scalar_offset_ad(2636, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) {
            s.store_mul_ad_lhs(2620, A::add_scaled_inputs_product(s.ad_value(2619), 0.29214664, A::square(s.ad_value(2619)), s.v[373], A::square(s.ad_value(2619)), s.ad_value(2619), s.v[374]), 2636);
        }

        s.b[2762] = (s.v[2657] > 0.0);
        s.v[2762] = if s.b[2762] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) && s.b[2762]) {
            s.copy_ad(2658, 2620);
        }

        s.b[2763] = (s.v[2656] > (-230.25850929940458));
        s.v[2763] = if s.b[2763] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) && (!s.b[2762])) && s.b[2763]) {
            s.store_exp(2636, 2656);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) && (!s.b[2762])) && (!s.b[2763])) {
            s.store_div_from_scalar_offset_ad(2636, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2656), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2656), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) && (!s.b[2762])) {
            s.store_sub_scaled_inputs(2658, 2636, 2.0, 2620, 1.0);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) {
            s.store_ad_value(2659, A::div_scaled_product(s.ad_value(602), s.ad_value(2658), (1.772453850905516 * 0.5), s.ad_value(2654), 1.0));
            s.store_mul_ad_rhs(2645, 525, A::mul3(s.ad_value(2644), s.ad_value(2659), s.ad_value(2653)));
        }

        s.b[2764] = (s.v[531] == 0.0);
        s.v[2764] = if s.b[2764] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && s.b[2764]) {
            s.store_scalar(2660, 0.0);
        }

        s.b[2765] = (s.v[511] == 0.5);
        s.v[2765] = if s.b[2765] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2764])) && s.b[2765]) {
            s.store_sqrt_mul_ad(2636, A::sub(s.ad_value(508), s.ad_value(2634)), s.ad_value(596));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2764])) && (!s.b[2765])) {
            s.store_pow_ad(2636, A::mul(A::sub(s.ad_value(508), s.ad_value(2634)), s.ad_value(596)), s.ad_value(511));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2764])) {
            s.store_mul_ad_rhs(2661, 578, A::div_scaled_product(A::sub(s.ad_value(508), s.ad_value(2634)), s.ad_value(593), 1.0, s.ad_value(2636), 1.0));
        }

        s.b[2766] = (((((-s.v[608]) / s.v[2661])) as f64).abs() < 230.25850929940458);
        s.v[2766] = if s.b[2766] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2764])) && s.b[2766]) {
            s.store_exp_ad(2636, A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(2661), 1.0));
        }

        s.b[2767] = (((-s.v[608]) / s.v[2661]) < 0.0);
        s.v[2767] = if s.b[2767] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2764])) && (!s.b[2766])) && s.b[2767]) {
            s.store_div_from_scalar_offset_ad(2636, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(2661), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(2661), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2764])) && (!s.b[2766])) && (!s.b[2767])) {
            let assign60350_ad_e77633: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(2661), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(2661), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(2661), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2636, assign60350_ad_e77633, 1e100);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2764])) {
            s.store_mul_ad_product_lhs(2660, s.ad_value(531), A::mul3(s.ad_value(833), s.ad_value(2661), s.ad_value(2661)), 2636);
        }

        s.b[2768] = (s.v[540] > 1000.0);
        s.v[2768] = if s.b[2768] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && s.b[2768]) {
            s.store_scalar(2662, 1.0);
        }

        s.b[2769] = (s.v[2635] > ((-s.v[444]) * s.v[540]));
        s.v[2769] = if s.b[2769] { 1.0 } else { 0.0 };

        s.b[2770] = (s.v[543] == 4.0);
        s.v[2770] = if s.b[2770] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2768])) && s.b[2769]) && s.b[2770]) {
            s.store_mul_ad_product_lhs(2636, A::mul3(A::mul3(s.ad_value(2635), s.ad_value(614), A::mul(s.ad_value(2635), s.ad_value(614))), s.ad_value(2635), s.ad_value(614)), s.ad_value(2635), 614);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2768])) && s.b[2769]) && (!s.b[2770])) {
            s.store_pow_ad(2636, A::abs(A::mul(s.ad_value(2635), s.ad_value(614))), s.ad_value(543));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2768])) && s.b[2769]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2662, 1.0, 1.0, s.ad_value(2636));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2768])) && (!s.b[2769])) {
            s.store_ad_value(2662, A::add_scaled_product(s.ad_value(611), 1.0, A::add_scaled_inputs(s.ad_value(2635), 1.0, s.ad_value(540), s.v[444]), s.ad_value(617), 1.0));
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) {
            s.store_mul_scale_ad_lhs(1915, A::add(A::add_scaled_inputs3(s.ad_value(2637), 1.0, s.ad_value(2638), 1.0, s.ad_value(2645), 1.0), s.ad_value(2660)), p.p29, 2662);
        }

        s.b[2771] = (s.v[575] == 0.5);
        s.v[2771] = if s.b[2771] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && s.b[2771]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2628), s.ad_value(572)));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2771])) {
            s.store_pow_ad(2636, A::sub_from_scalar(1.0, A::mul(s.ad_value(2628), s.ad_value(572))), s.ad_value(575));
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) {
            s.store_ad_value(1921, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(584), 1.0, s.ad_value(2636)), p.p30, s.ad_value(587), A::sub(s.ad_value(833), s.ad_value(2628)), p.p30));
        }

        s.b[2772] = (s.v[674] == 0.0);
        s.v[2772] = if s.b[2772] { 1.0 } else { 0.0 };

        if ((s.b[2665] && (!s.b[2666])) && s.b[2772]) {
            s.store_scalar(1916, 0.0);
            s.store_scalar(1922, 0.0);
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) {
            s.store_mul(2637, 564, 2627);
        }

        s.b[2773] = ((s.v[523] == 0.0) && (s.v[526] == 0.0));
        s.v[2773] = if s.b[2773] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && s.b[2773]) {
            s.store_scalar(2638, 0.0);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2773])) {
            s.store_sub(2639, 570, 2633);
            s.store_sub_from_scalar_ad(2640, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2631), s.ad_value(2639)))));
        }

        s.b[2774] = (s.v[512] == 0.5);
        s.v[2774] = if s.b[2774] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2773])) && s.b[2774]) {
            s.store_scalar(2641, 0.0);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2773])) && (!s.b[2774])) {
            s.store_ad_value(2641, A::mul_sub_from_scalar_rhs(A::add(A::div_scaled_product(A::square(s.ad_value(2640)), A::ln(s.ad_value(2640)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2640)), 1.0), s.ad_value(2640)), 1.0, A::scale(s.ad_value(512), 2.0)));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2773])) {
            s.store_add(2642, 2640, 2641);
        }

        s.b[2775] = (s.v[512] == 0.5);
        s.v[2775] = if s.b[2775] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2773])) && s.b[2775]) {
            s.store_sqrt_mul(2636, 2639, 597);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2773])) && (!s.b[2775])) {
            s.store_pow_ad(2636, A::mul(s.ad_value(2639), s.ad_value(597)), s.ad_value(512));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2773])) {
            s.store_mul(2643, 591, 2636);
            s.store_mul_ad_product_lhs(2644, s.ad_value(561), A::offset(s.ad_value(2630), (-1.0)), 2643);
            s.store_mul3_lhs(2638, 523, 2644, 2642);
        }

        s.b[2776] = (s.v[526] == 0.0);
        s.v[2776] = if s.b[2776] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && s.b[2776]) {
            s.store_scalar(2645, 0.0);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) {
            s.store_mul_ad_rhs(2646, 606, A::div_scaled_product(s.ad_value(2643), s.ad_value(576), 1.0, s.ad_value(2639), 1.0));
            s.store_scaled_div(2647, 603, 2646, 0.666666666666667);
            s.store_square(2648, 2647);
            s.store_sqrt_ad(2649, A::div_scaled_product(s.ad_value(2648), s.ad_value(2648), 1.0, A::offset(A::square(s.ad_value(2648)), 1.0), 1.0));
            s.store_sqrt(2650, 2649);
            s.store_mul(2651, 2649, 2650);
        }

        s.b[2777] = (((-s.v[512]) * s.v[579]) == (-1.0));
        s.v[2777] = if s.b[2777] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) && s.b[2777]) {
            s.store_div_from_scalar_offset_ad(2652, 1.0, A::mul(s.ad_value(2646), s.ad_value(2651)), 1.0);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) && (!s.b[2777])) {
            s.store_pow_ad(2652, A::offset(A::mul(s.ad_value(2646), s.ad_value(2651)), 1.0), A::mul_scaled_lhs(s.ad_value(512), -1.0, s.ad_value(579)));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) {
            s.store_ad_value(2653, A::div_scaled_product(s.ad_value(2642), s.ad_value(2652), 1.0, A::add(s.ad_value(2642), s.ad_value(2652)), 1.0));
            s.store_sqrt_scaled_ad(2654, A::div(s.ad_value(2646), s.ad_value(2650)), 0.375);
            s.store_ad_value(2655, A::add_scaled_product(s.ad_value(2649), (-1.0), s.ad_value(2647), s.ad_value(2650), 2.0));
            s.store_ad_value(2656, A::add_scaled_product(A::add_scaled_product(A::mul3(s.ad_value(603), s.ad_value(2647), s.ad_value(2650)), 1.0, s.ad_value(603), s.ad_value(2649), (-1.0)), 1.0, s.ad_value(2646), s.ad_value(2651), 0.5));
            s.store_mul_offset_lhs(2657, 2655, (-1.0), 2654);
            s.store_square(2618, 2657);
        }

        s.b[2778] = (s.v[2657] > 0.0);
        s.v[2778] = if s.b[2778] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) && s.b[2778]) {
            s.store_div_from_scalar_offset_scaled_input(2619, 1.0, 2657, s.v[372], 1.0);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) && (!s.b[2778])) {
            s.store_div_from_scalar_sub_from_scalar_ad(2619, 1.0, 1.0, A::scale(s.ad_value(2657), s.v[372]));
        }

        s.b[2779] = (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458));
        s.v[2779] = if s.b[2779] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) && s.b[2779]) {
            s.store_exp_sub(2636, 2656, 2618);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) && (!s.b[2779])) {
            s.store_div_from_scalar_offset_ad(2636, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) {
            s.store_mul_ad_lhs(2620, A::add_scaled_inputs_product(s.ad_value(2619), 0.29214664, A::square(s.ad_value(2619)), s.v[373], A::square(s.ad_value(2619)), s.ad_value(2619), s.v[374]), 2636);
        }

        s.b[2780] = (s.v[2657] > 0.0);
        s.v[2780] = if s.b[2780] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) && s.b[2780]) {
            s.copy_ad(2658, 2620);
        }

        s.b[2781] = (s.v[2656] > (-230.25850929940458));
        s.v[2781] = if s.b[2781] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) && (!s.b[2780])) && s.b[2781]) {
            s.store_exp(2636, 2656);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) && (!s.b[2780])) && (!s.b[2781])) {
            s.store_div_from_scalar_offset_ad(2636, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2656), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2656), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) && (!s.b[2780])) {
            s.store_sub_scaled_inputs(2658, 2636, 2.0, 2620, 1.0);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) {
            s.store_ad_value(2659, A::div_scaled_product(s.ad_value(603), s.ad_value(2658), (1.772453850905516 * 0.5), s.ad_value(2654), 1.0));
            s.store_mul_ad_rhs(2645, 526, A::mul3(s.ad_value(2644), s.ad_value(2659), s.ad_value(2653)));
        }

        s.b[2782] = (s.v[532] == 0.0);
        s.v[2782] = if s.b[2782] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && s.b[2782]) {
            s.store_scalar(2660, 0.0);
        }

        s.b[2783] = (s.v[512] == 0.5);
        s.v[2783] = if s.b[2783] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) && s.b[2783]) {
            s.store_sqrt_mul_ad(2636, A::sub(s.ad_value(509), s.ad_value(2634)), s.ad_value(597));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) && (!s.b[2783])) {
            s.store_pow_ad(2636, A::mul(A::sub(s.ad_value(509), s.ad_value(2634)), s.ad_value(597)), s.ad_value(512));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) {
            s.store_mul_ad_rhs(2661, 579, A::div_scaled_product(A::sub(s.ad_value(509), s.ad_value(2634)), s.ad_value(594), 1.0, s.ad_value(2636), 1.0));
        }

        s.b[2784] = (((((-s.v[609]) / s.v[2661])) as f64).abs() < 230.25850929940458);
        s.v[2784] = if s.b[2784] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) && s.b[2784]) {
            s.store_exp_ad(2636, A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(2661), 1.0));
        }

        s.b[2785] = (((-s.v[609]) / s.v[2661]) < 0.0);
        s.v[2785] = if s.b[2785] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) && (!s.b[2784])) && s.b[2785]) {
            s.store_div_from_scalar_offset_ad(2636, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(2661), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(2661), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) && (!s.b[2784])) && (!s.b[2785])) {
            let assign61100_ad_e78899: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(2661), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(2661), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(2661), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2636, assign61100_ad_e78899, 1e100);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) {
            s.store_mul_ad_product_lhs(2660, s.ad_value(532), A::mul3(s.ad_value(833), s.ad_value(2661), s.ad_value(2661)), 2636);
        }

        s.b[2786] = (s.v[541] > 1000.0);
        s.v[2786] = if s.b[2786] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && s.b[2786]) {
            s.store_scalar(2662, 1.0);
        }

        s.b[2787] = (s.v[2635] > ((-s.v[444]) * s.v[541]));
        s.v[2787] = if s.b[2787] { 1.0 } else { 0.0 };

        s.b[2788] = (s.v[544] == 4.0);
        s.v[2788] = if s.b[2788] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2786])) && s.b[2787]) && s.b[2788]) {
            s.store_mul_ad_product_lhs(2636, A::mul3(A::mul3(s.ad_value(2635), s.ad_value(615), A::mul(s.ad_value(2635), s.ad_value(615))), s.ad_value(2635), s.ad_value(615)), s.ad_value(2635), 615);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2786])) && s.b[2787]) && (!s.b[2788])) {
            s.store_pow_ad(2636, A::abs(A::mul(s.ad_value(2635), s.ad_value(615))), s.ad_value(544));
        }

    }

    pub(super) fn stamp_transient_block_43(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2786])) && s.b[2787]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2662, 1.0, 1.0, s.ad_value(2636));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2786])) && (!s.b[2787])) {
            s.store_ad_value(2662, A::add_scaled_product(s.ad_value(612), 1.0, A::add_scaled_inputs(s.ad_value(2635), 1.0, s.ad_value(541), s.v[444]), s.ad_value(618), 1.0));
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) {
            s.store_mul_scale_ad_lhs(1916, A::add(A::add_scaled_inputs3(s.ad_value(2637), 1.0, s.ad_value(2638), 1.0, s.ad_value(2645), 1.0), s.ad_value(2660)), p.p29, 2662);
        }

        s.b[2789] = (s.v[576] == 0.5);
        s.v[2789] = if s.b[2789] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && s.b[2789]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2628), s.ad_value(573)));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2789])) {
            s.store_pow_ad(2636, A::sub_from_scalar(1.0, A::mul(s.ad_value(2628), s.ad_value(573))), s.ad_value(576));
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) {
            s.store_ad_value(1922, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(585), 1.0, s.ad_value(2636)), p.p30, s.ad_value(588), A::sub(s.ad_value(833), s.ad_value(2628)), p.p30));
        }

        s.b[2790] = (s.v[675] == 0.0);
        s.v[2790] = if s.b[2790] { 1.0 } else { 0.0 };

        if ((s.b[2665] && (!s.b[2666])) && s.b[2790]) {
            s.store_scalar(1917, 0.0);
            s.store_scalar(1923, 0.0);
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) {
            s.store_mul(2637, 565, 2627);
        }

        s.b[2791] = ((s.v[524] == 0.0) && (s.v[527] == 0.0));
        s.v[2791] = if s.b[2791] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2791]) {
            s.store_scalar(2638, 0.0);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2791])) {
            s.store_sub(2639, 571, 2633);
            s.store_sub_from_scalar_ad(2640, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2631), s.ad_value(2639)))));
        }

        s.b[2792] = (s.v[513] == 0.5);
        s.v[2792] = if s.b[2792] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2791])) && s.b[2792]) {
            s.store_scalar(2641, 0.0);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2791])) && (!s.b[2792])) {
            s.store_ad_value(2641, A::mul_sub_from_scalar_rhs(A::add(A::div_scaled_product(A::square(s.ad_value(2640)), A::ln(s.ad_value(2640)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2640)), 1.0), s.ad_value(2640)), 1.0, A::scale(s.ad_value(513), 2.0)));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2791])) {
            s.store_add(2642, 2640, 2641);
        }

        s.b[2793] = (s.v[513] == 0.5);
        s.v[2793] = if s.b[2793] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2791])) && s.b[2793]) {
            s.store_sqrt_mul(2636, 2639, 598);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2791])) && (!s.b[2793])) {
            s.store_pow_ad(2636, A::mul(s.ad_value(2639), s.ad_value(598)), s.ad_value(513));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2791])) {
            s.store_mul(2643, 592, 2636);
            s.store_mul_ad_product_lhs(2644, s.ad_value(562), A::offset(s.ad_value(2630), (-1.0)), 2643);
            s.store_mul3_lhs(2638, 524, 2644, 2642);
        }

        s.b[2794] = (s.v[527] == 0.0);
        s.v[2794] = if s.b[2794] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2794]) {
            s.store_scalar(2645, 0.0);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) {
            s.store_mul_ad_rhs(2646, 607, A::div_scaled_product(s.ad_value(2643), s.ad_value(577), 1.0, s.ad_value(2639), 1.0));
            s.store_scaled_div(2647, 604, 2646, 0.666666666666667);
            s.store_square(2648, 2647);
            s.store_sqrt_ad(2649, A::div_scaled_product(s.ad_value(2648), s.ad_value(2648), 1.0, A::offset(A::square(s.ad_value(2648)), 1.0), 1.0));
            s.store_sqrt(2650, 2649);
            s.store_mul(2651, 2649, 2650);
        }

        s.b[2795] = (((-s.v[513]) * s.v[580]) == (-1.0));
        s.v[2795] = if s.b[2795] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && s.b[2795]) {
            s.store_div_from_scalar_offset_ad(2652, 1.0, A::mul(s.ad_value(2646), s.ad_value(2651)), 1.0);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && (!s.b[2795])) {
            s.store_pow_ad(2652, A::offset(A::mul(s.ad_value(2646), s.ad_value(2651)), 1.0), A::mul_scaled_lhs(s.ad_value(513), -1.0, s.ad_value(580)));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) {
            s.store_ad_value(2653, A::div_scaled_product(s.ad_value(2642), s.ad_value(2652), 1.0, A::add(s.ad_value(2642), s.ad_value(2652)), 1.0));
            s.store_sqrt_scaled_ad(2654, A::div(s.ad_value(2646), s.ad_value(2650)), 0.375);
            s.store_ad_value(2655, A::add_scaled_product(s.ad_value(2649), (-1.0), s.ad_value(2647), s.ad_value(2650), 2.0));
            s.store_ad_value(2656, A::add_scaled_product(A::add_scaled_product(A::mul3(s.ad_value(604), s.ad_value(2647), s.ad_value(2650)), 1.0, s.ad_value(604), s.ad_value(2649), (-1.0)), 1.0, s.ad_value(2646), s.ad_value(2651), 0.5));
            s.store_mul_offset_lhs(2657, 2655, (-1.0), 2654);
            s.store_square(2618, 2657);
        }

        s.b[2796] = (s.v[2657] > 0.0);
        s.v[2796] = if s.b[2796] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && s.b[2796]) {
            s.store_div_from_scalar_offset_scaled_input(2619, 1.0, 2657, s.v[372], 1.0);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && (!s.b[2796])) {
            s.store_div_from_scalar_sub_from_scalar_ad(2619, 1.0, 1.0, A::scale(s.ad_value(2657), s.v[372]));
        }

        s.b[2797] = (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458));
        s.v[2797] = if s.b[2797] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && s.b[2797]) {
            s.store_exp_sub(2636, 2656, 2618);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && (!s.b[2797])) {
            s.store_div_from_scalar_offset_ad(2636, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) {
            s.store_mul_ad_lhs(2620, A::add_scaled_inputs_product(s.ad_value(2619), 0.29214664, A::square(s.ad_value(2619)), s.v[373], A::square(s.ad_value(2619)), s.ad_value(2619), s.v[374]), 2636);
        }

        s.b[2798] = (s.v[2657] > 0.0);
        s.v[2798] = if s.b[2798] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && s.b[2798]) {
            s.copy_ad(2658, 2620);
        }

        s.b[2799] = (s.v[2656] > (-230.25850929940458));
        s.v[2799] = if s.b[2799] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && (!s.b[2798])) && s.b[2799]) {
            s.store_exp(2636, 2656);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && (!s.b[2798])) && (!s.b[2799])) {
            s.store_div_from_scalar_offset_ad(2636, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2656), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2656), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && (!s.b[2798])) {
            s.store_sub_scaled_inputs(2658, 2636, 2.0, 2620, 1.0);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) {
            s.store_ad_value(2659, A::div_scaled_product(s.ad_value(604), s.ad_value(2658), (1.772453850905516 * 0.5), s.ad_value(2654), 1.0));
            s.store_mul_ad_rhs(2645, 527, A::mul3(s.ad_value(2644), s.ad_value(2659), s.ad_value(2653)));
        }

        s.b[2800] = (s.v[533] == 0.0);
        s.v[2800] = if s.b[2800] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2800]) {
            s.store_scalar(2660, 0.0);
        }

        s.b[2801] = (s.v[513] == 0.5);
        s.v[2801] = if s.b[2801] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) && s.b[2801]) {
            s.store_sqrt_mul_ad(2636, A::sub(s.ad_value(510), s.ad_value(2634)), s.ad_value(598));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) && (!s.b[2801])) {
            s.store_pow_ad(2636, A::mul(A::sub(s.ad_value(510), s.ad_value(2634)), s.ad_value(598)), s.ad_value(513));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) {
            s.store_mul_ad_rhs(2661, 580, A::div_scaled_product(A::sub(s.ad_value(510), s.ad_value(2634)), s.ad_value(595), 1.0, s.ad_value(2636), 1.0));
        }

        s.b[2802] = (((((-s.v[610]) / s.v[2661])) as f64).abs() < 230.25850929940458);
        s.v[2802] = if s.b[2802] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) && s.b[2802]) {
            s.store_exp_ad(2636, A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(2661), 1.0));
        }

        s.b[2803] = (((-s.v[610]) / s.v[2661]) < 0.0);
        s.v[2803] = if s.b[2803] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) && (!s.b[2802])) && s.b[2803]) {
            s.store_div_from_scalar_offset_ad(2636, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(2661), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(2661), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) && (!s.b[2802])) && (!s.b[2803])) {
            let assign61850_ad_e80165: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(2661), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(2661), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(2661), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2636, assign61850_ad_e80165, 1e100);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) {
            s.store_mul_ad_product_lhs(2660, s.ad_value(533), A::mul3(s.ad_value(833), s.ad_value(2661), s.ad_value(2661)), 2636);
        }

        s.b[2804] = (s.v[641] > 1000.0);
        s.v[2804] = if s.b[2804] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2804]) {
            s.store_scalar(2662, 1.0);
        }

        s.b[2805] = (s.v[2635] > ((-s.v[444]) * s.v[641]));
        s.v[2805] = if s.b[2805] { 1.0 } else { 0.0 };

        s.b[2806] = (s.v[545] == 4.0);
        s.v[2806] = if s.b[2806] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2804])) && s.b[2805]) && s.b[2806]) {
            s.store_mul_ad_product_lhs(2636, A::mul3(A::mul3(s.ad_value(2635), s.ad_value(616), A::mul(s.ad_value(2635), s.ad_value(616))), s.ad_value(2635), s.ad_value(616)), s.ad_value(2635), 616);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2804])) && s.b[2805]) && (!s.b[2806])) {
            s.store_pow_ad(2636, A::abs(A::mul(s.ad_value(2635), s.ad_value(616))), s.ad_value(545));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2804])) && s.b[2805]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2662, 1.0, 1.0, s.ad_value(2636));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2804])) && (!s.b[2805])) {
            s.store_ad_value(2662, A::add_scaled_product(s.ad_value(613), 1.0, A::add_scaled_inputs(s.ad_value(2635), 1.0, s.ad_value(641), s.v[444]), s.ad_value(619), 1.0));
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) {
            s.store_mul_scale_ad_lhs(1917, A::add(A::add_scaled_inputs3(s.ad_value(2637), 1.0, s.ad_value(2638), 1.0, s.ad_value(2645), 1.0), s.ad_value(2660)), p.p29, 2662);
        }

        s.b[2807] = (s.v[635] == 1.0);
        s.v[2807] = if s.b[2807] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) {
            let assign61970_ad_e80391: A = {
                if (s.v[833] < s.v[550]) {
                    {
                        if (((s.v[833] - s.v[550]) / s.v[551]) < (-37.0)) {
                            s.ad_value(550)
                        } else {
                            A::add_scaled_product(s.ad_value(550), 1.0, A::ln_one_plus_exp(A::div(A::sub(s.ad_value(833), s.ad_value(550)), s.ad_value(551))), s.ad_value(551), 1.0)
                        }
                    }
                } else {
                    {
                        if (((s.v[833] - s.v[550]) / s.v[551]) > 37.0) {
                            s.ad_value(833)
                        } else {
                            A::add_scaled_product(s.ad_value(833), 1.0, A::ln_one_plus_exp(A::div(A::sub(s.ad_value(550), s.ad_value(833)), s.ad_value(551))), s.ad_value(551), 1.0)
                        }
                    }
                }
            };
            s.store_ad_value(2663, assign61970_ad_e80391);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) {
            s.store_scaled_mul(2621, 684, 684, 4.0);
            s.store_div(2622, 684, 685);
            s.store_ad_value(2623, A::add_scaled_product(s.ad_value(2663), 1.0, s.ad_value(684), s.ad_value(2622), 1.0));
            s.store_add(2624, 685, 2623);
            s.store_sub(2625, 685, 2623);
            s.store_sqrt_square_add(2626, 2625, 2621);
            s.store_ad_value(2664, A::div_scaled_product(s.ad_value(2663), s.ad_value(685), 2.0, A::add(s.ad_value(2624), s.ad_value(2626)), 1.0));
        }

        s.b[2808] = (s.v[577] == 0.5);
        s.v[2808] = if s.b[2808] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) && s.b[2808]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2664), s.ad_value(574)));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) && (!s.b[2808])) {
            s.store_pow_ad(2636, A::sub_from_scalar(1.0, A::mul(s.ad_value(2664), s.ad_value(574))), s.ad_value(577));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) {
            s.store_ad_value(1923, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(586), 1.0, s.ad_value(2636)), p.p30, s.ad_value(589), A::sub(s.ad_value(2663), s.ad_value(2664)), p.p30));
            s.store_ad_value(2663, A::add_scaled_inputs3(s.ad_value(833), 1.0, s.ad_value(550), 1.0, s.ad_value(2663), -1.0));
            s.store_scaled_mul(2621, 684, 684, 4.0);
            s.store_div(2622, 684, 685);
            s.store_ad_value(2623, A::add_scaled_product(s.ad_value(2663), 1.0, s.ad_value(684), s.ad_value(2622), 1.0));
            s.store_add(2624, 685, 2623);
            s.store_sub(2625, 685, 2623);
            s.store_sqrt_square_add(2626, 2625, 2621);
            s.store_ad_value(2664, A::div_scaled_product(s.ad_value(2663), s.ad_value(685), 2.0, A::add(s.ad_value(2624), s.ad_value(2626)), 1.0));
        }

        s.b[2809] = (s.v[630] == 0.5);
        s.v[2809] = if s.b[2809] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) && s.b[2809]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2664), s.ad_value(629)));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) && (!s.b[2809])) {
            s.store_pow_ad(2636, A::sub_from_scalar(1.0, A::mul(s.ad_value(2664), s.ad_value(629))), s.ad_value(630));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) {
            s.store_ad_value(472, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(633), 1.0, s.ad_value(2636)), p.p30, s.ad_value(634), A::sub(s.ad_value(2663), s.ad_value(2664)), p.p30));
            s.store_add(1923, 1923, 472);
        }

        s.b[2810] = (s.v[577] == 0.5);
        s.v[2810] = if s.b[2810] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2807])) && s.b[2810]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2628), s.ad_value(574)));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2807])) && (!s.b[2810])) {
            s.store_pow_ad(2636, A::sub_from_scalar(1.0, A::mul(s.ad_value(2628), s.ad_value(574))), s.ad_value(577));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2807])) {
            s.store_ad_value(1923, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(586), 1.0, s.ad_value(2636)), p.p30, s.ad_value(589), A::sub(s.ad_value(833), s.ad_value(2628)), p.p30));
        }

        if (s.b[2665] && (!s.b[2666])) {
            s.store_ad_value(849, A::add_scaled_product(A::add_scaled_products(s.ad_value(673), s.ad_value(1915), 1.0, s.ad_value(674), s.ad_value(1916), 1.0), 1.0, s.ad_value(675), s.ad_value(1917), 1.0));
        }

        s.v[1942] = 0.0;

        s.v[1943] = 0.0;

        s.v[1944] = 0.0;

        s.v[1945] = 0.0;

        s.v[1946] = 0.0;

        s.v[1947] = 0.0;

        s.v[1948] = 0.0;

        s.v[1949] = 0.0;

        s.v[1950] = 0.0;

        s.v[1951] = 0.0;

        s.v[1952] = 0.0;

        s.v[1953] = 0.0;

        s.v[1954] = 0.0;

        s.v[1955] = 0.0;

        s.v[1956] = 0.0;

        s.v[1957] = 0.0;

        s.v[1958] = 0.0;

        s.v[1959] = 0.0;

        s.b[2811] = (s.v[1] != 0.0);
        s.v[2811] = if s.b[2811] { 1.0 } else { 0.0 };

        if s.b[2811] {
            s.store_scalar(1988, 0.0);
            s.store_scalar(1992, 0.0);
            s.store_scalar(1986, 0.0);
            s.store_scalar(1987, 0.0);
            s.store_scalar(1993, 0.0);
            s.store_scalar(1969, 0.0);
            s.store_scalar(1970, 0.0);
            s.store_scalar(1971, 0.0);
            s.store_scalar(1972, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_44(
        s: &mut Scratch,
    ) {
        if s.b[2811] {
            s.store_scalar(1973, 0.0);
            s.store_scalar(1974, 0.0);
            s.store_scalar(1975, 0.0);
            s.store_scalar(1976, 0.0);
            s.store_scalar(1977, 0.0);
            s.store_scalar(1960, 0.0);
            s.store_scalar(1961, 0.0);
            s.store_scalar(1962, 0.0);
            s.store_scalar(1963, 0.0);
            s.store_scalar(1964, 0.0);
            s.store_scalar(1965, 0.0);
            s.store_scalar(1966, 0.0);
            s.store_scalar(1967, 0.0);
            s.store_scalar(1968, 0.0);
        }

        s.b[2812] = (s.v[1890] > 0.0);
        s.v[2812] = if s.b[2812] { 1.0 } else { 0.0 };

        s.b[2813] = (s.v[1] == 1.0);
        s.v[2813] = if s.b[2813] { 1.0 } else { 0.0 };

        if ((s.b[2811] && s.b[2812]) && s.b[2813]) {
            s.store_ad_value(1960, A::add_scaled_product(s.ad_value(1934), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(A::div_scaled_inputs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0), 0.5, s.ad_value(1936))))), s.ad_value(1932), 1.0));
        }

        s.b[2814] = (((s.v[1960]) as f64).abs() <= s.v[1933]);
        s.v[2814] = if s.b[2814] { 1.0 } else { 0.0 };

        if (((s.b[2811] && s.b[2812]) && s.b[2813]) && s.b[2814]) {
            s.store_mul_ad_affine_product_rhs(1996, 1960, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1960), 1.0, A::scale(s.ad_value(1960), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2815] = ((((-s.v[1960])) as f64).abs() < 230.25850929940458);
        s.v[2815] = if s.b[2815] { 1.0 } else { 0.0 };

        if ((((s.b[2811] && s.b[2812]) && s.b[2813]) && (!s.b[2814])) && s.b[2815]) {
            s.store_exp_neg_input(2027, 1960);
        }

        s.b[2816] = ((-s.v[1960]) < 0.0);
        s.v[2816] = if s.b[2816] { 1.0 } else { 0.0 };

        if (((((s.b[2811] && s.b[2812]) && s.b[2813]) && (!s.b[2814])) && (!s.b[2815])) && s.b[2816]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(1960)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(1960)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2811] && s.b[2812]) && s.b[2813]) && (!s.b[2814])) && (!s.b[2815])) && (!s.b[2816])) {
            s.store_scaled_offset_ad(2027, A::mul_offset_lhs(A::neg(s.ad_value(1960)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(1960)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(1960)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((s.b[2811] && s.b[2812]) && s.b[2813]) && (!s.b[2814])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0)));
        }

        s.b[2817] = (s.v[1960] > s.v[1933]);
        s.v[2817] = if s.b[2817] { 1.0 } else { 0.0 };

        if ((((s.b[2811] && s.b[2812]) && s.b[2813]) && (!s.b[2814])) && s.b[2817]) {
            s.store_neg(1996, 1996);
        }

        if ((s.b[2811] && s.b[2812]) && s.b[2813]) {
            s.store_ad_value(1942, A::add_scaled_product(s.ad_value(1996), (-1.0), s.ad_value(1937), A::sub(s.ad_value(1890), s.ad_value(1960)), -1.0));
        }

        s.b[2818] = (s.v[1] == 2.0);
        s.v[2818] = if s.b[2818] { 1.0 } else { 0.0 };

        if (((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) {
            s.store_ad_value(1960, A::add_scaled_product(s.ad_value(1934), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(A::div_scaled_inputs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0), 0.3333333333333333, s.ad_value(1936))))), s.ad_value(1932), 1.0));
        }

        s.b[2819] = (((s.v[1960]) as f64).abs() <= s.v[1933]);
        s.v[2819] = if s.b[2819] { 1.0 } else { 0.0 };

        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && s.b[2819]) {
            s.store_mul_ad_affine_product_rhs(1996, 1960, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1960), 1.0, A::scale(s.ad_value(1960), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2820] = ((((-s.v[1960])) as f64).abs() < 230.25850929940458);
        s.v[2820] = if s.b[2820] { 1.0 } else { 0.0 };

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2819])) && s.b[2820]) {
            s.store_exp_neg_input(2027, 1960);
        }

        s.b[2821] = ((-s.v[1960]) < 0.0);
        s.v[2821] = if s.b[2821] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2819])) && (!s.b[2820])) && s.b[2821]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(1960)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(1960)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2819])) && (!s.b[2820])) && (!s.b[2821])) {
            s.store_scaled_offset_ad(2027, A::mul_offset_lhs(A::neg(s.ad_value(1960)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(1960)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(1960)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2819])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0)));
        }

        s.b[2822] = (s.v[1960] > s.v[1933]);
        s.v[2822] = if s.b[2822] { 1.0 } else { 0.0 };

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2819])) && s.b[2822]) {
            s.store_neg(1996, 1996);
        }

        if (((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) {
            s.store_ad_value(1942, A::add_scaled_product(s.ad_value(1996), (-1.0), s.ad_value(1937), A::sub(s.ad_value(1890), s.ad_value(1960)), -1.0));
            s.store_ad_value(1961, A::add_scaled_product(s.ad_value(1934), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(A::div_scaled_inputs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0), 0.6666666666666666, s.ad_value(1936))))), s.ad_value(1932), 1.0));
        }

        s.b[2823] = (((s.v[1961]) as f64).abs() <= s.v[1933]);
        s.v[2823] = if s.b[2823] { 1.0 } else { 0.0 };

        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && s.b[2823]) {
            s.store_mul_ad_affine_product_rhs(1996, 1961, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1961), 1.0, A::scale(s.ad_value(1961), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2824] = ((((-s.v[1961])) as f64).abs() < 230.25850929940458);
        s.v[2824] = if s.b[2824] { 1.0 } else { 0.0 };

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2823])) && s.b[2824]) {
            s.store_exp_neg_input(2027, 1961);
        }

        s.b[2825] = ((-s.v[1961]) < 0.0);
        s.v[2825] = if s.b[2825] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2823])) && (!s.b[2824])) && s.b[2825]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(1961)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(1961)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2823])) && (!s.b[2824])) && (!s.b[2825])) {
            s.store_scaled_offset_ad(2027, A::mul_offset_lhs(A::neg(s.ad_value(1961)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(1961)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(1961)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2823])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1961)), (-1.0)));
        }

        s.b[2826] = (s.v[1961] > s.v[1933]);
        s.v[2826] = if s.b[2826] { 1.0 } else { 0.0 };

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2823])) && s.b[2826]) {
            s.store_neg(1996, 1996);
        }

        if (((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) {
            s.store_ad_value(1943, A::add_scaled_product(s.ad_value(1996), (-1.0), s.ad_value(1937), A::sub(s.ad_value(1890), s.ad_value(1961)), -1.0));
        }

        s.b[2827] = (s.v[831] < 0.0);
        s.v[2827] = if s.b[2827] { 1.0 } else { 0.0 };

        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && s.b[2827]) {
            s.copy_ad(2027, 1942);
            s.copy_ad(1942, 1943);
            s.copy_ad(1943, 2027);
        }

        s.b[2828] = (s.v[1] == 3.0);
        s.v[2828] = if s.b[2828] { 1.0 } else { 0.0 };

        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) {
            s.store_ad_value(1960, A::add_scaled_product(s.ad_value(1934), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(A::div_scaled_inputs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0), 0.25, s.ad_value(1936))))), s.ad_value(1932), 1.0));
        }

        s.b[2829] = (((s.v[1960]) as f64).abs() <= s.v[1933]);
        s.v[2829] = if s.b[2829] { 1.0 } else { 0.0 };

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && s.b[2829]) {
            s.store_mul_ad_affine_product_rhs(1996, 1960, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1960), 1.0, A::scale(s.ad_value(1960), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2830] = ((((-s.v[1960])) as f64).abs() < 230.25850929940458);
        s.v[2830] = if s.b[2830] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2829])) && s.b[2830]) {
            s.store_exp_neg_input(2027, 1960);
        }

        s.b[2831] = ((-s.v[1960]) < 0.0);
        s.v[2831] = if s.b[2831] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2829])) && (!s.b[2830])) && s.b[2831]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(1960)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(1960)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2829])) && (!s.b[2830])) && (!s.b[2831])) {
            s.store_scaled_offset_ad(2027, A::mul_offset_lhs(A::neg(s.ad_value(1960)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(1960)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(1960)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2829])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0)));
        }

        s.b[2832] = (s.v[1960] > s.v[1933]);
        s.v[2832] = if s.b[2832] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2829])) && s.b[2832]) {
            s.store_neg(1996, 1996);
        }

        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) {
            s.store_ad_value(1942, A::add_scaled_product(s.ad_value(1996), (-1.0), s.ad_value(1937), A::sub(s.ad_value(1890), s.ad_value(1960)), -1.0));
            s.store_ad_value(1961, A::add_scaled_product(s.ad_value(1934), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(A::div_scaled_inputs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0), 0.5, s.ad_value(1936))))), s.ad_value(1932), 1.0));
        }

        s.b[2833] = (((s.v[1961]) as f64).abs() <= s.v[1933]);
        s.v[2833] = if s.b[2833] { 1.0 } else { 0.0 };

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && s.b[2833]) {
            s.store_mul_ad_affine_product_rhs(1996, 1961, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1961), 1.0, A::scale(s.ad_value(1961), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2834] = ((((-s.v[1961])) as f64).abs() < 230.25850929940458);
        s.v[2834] = if s.b[2834] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2833])) && s.b[2834]) {
            s.store_exp_neg_input(2027, 1961);
        }

        s.b[2835] = ((-s.v[1961]) < 0.0);
        s.v[2835] = if s.b[2835] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2833])) && (!s.b[2834])) && s.b[2835]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(1961)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(1961)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2833])) && (!s.b[2834])) && (!s.b[2835])) {
            s.store_scaled_offset_ad(2027, A::mul_offset_lhs(A::neg(s.ad_value(1961)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(1961)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(1961)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2833])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1961)), (-1.0)));
        }

        s.b[2836] = (s.v[1961] > s.v[1933]);
        s.v[2836] = if s.b[2836] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2833])) && s.b[2836]) {
            s.store_neg(1996, 1996);
        }

        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) {
            s.store_ad_value(1943, A::add_scaled_product(s.ad_value(1996), (-1.0), s.ad_value(1937), A::sub(s.ad_value(1890), s.ad_value(1961)), -1.0));
            s.store_ad_value(1962, A::add_scaled_product(s.ad_value(1934), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(A::div_scaled_inputs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0), 0.75, s.ad_value(1936))))), s.ad_value(1932), 1.0));
        }

        s.b[2837] = (((s.v[1962]) as f64).abs() <= s.v[1933]);
        s.v[2837] = if s.b[2837] { 1.0 } else { 0.0 };

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && s.b[2837]) {
            s.store_mul_ad_affine_product_rhs(1996, 1962, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1962), 1.0, A::scale(s.ad_value(1962), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2838] = ((((-s.v[1962])) as f64).abs() < 230.25850929940458);
        s.v[2838] = if s.b[2838] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2837])) && s.b[2838]) {
            s.store_exp_neg_input(2027, 1962);
        }

        s.b[2839] = ((-s.v[1962]) < 0.0);
        s.v[2839] = if s.b[2839] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2837])) && (!s.b[2838])) && s.b[2839]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(1962)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(1962)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2837])) && (!s.b[2838])) && (!s.b[2839])) {
            s.store_scaled_offset_ad(2027, A::mul_offset_lhs(A::neg(s.ad_value(1962)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(1962)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(1962)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2837])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1962)), (-1.0)));
        }

        s.b[2840] = (s.v[1962] > s.v[1933]);
        s.v[2840] = if s.b[2840] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2837])) && s.b[2840]) {
            s.store_neg(1996, 1996);
        }

        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) {
            s.store_ad_value(1944, A::add_scaled_product(s.ad_value(1996), (-1.0), s.ad_value(1937), A::sub(s.ad_value(1890), s.ad_value(1962)), -1.0));
        }

        s.b[2841] = (s.v[831] < 0.0);
        s.v[2841] = if s.b[2841] { 1.0 } else { 0.0 };

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && s.b[2841]) {
            s.copy_ad(2027, 1942);
            s.copy_ad(1942, 1944);
            s.copy_ad(1944, 2027);
        }

        s.b[2842] = (s.v[1] == 5.0);
        s.v[2842] = if s.b[2842] { 1.0 } else { 0.0 };

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) {
            s.store_ad_value(1960, A::add_scaled_product(s.ad_value(1934), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(A::div_scaled_inputs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0), 0.16666666666666666, s.ad_value(1936))))), s.ad_value(1932), 1.0));
        }

        s.b[2843] = (((s.v[1960]) as f64).abs() <= s.v[1933]);
        s.v[2843] = if s.b[2843] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && s.b[2843]) {
            s.store_mul_ad_affine_product_rhs(1996, 1960, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1960), 1.0, A::scale(s.ad_value(1960), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2844] = ((((-s.v[1960])) as f64).abs() < 230.25850929940458);
        s.v[2844] = if s.b[2844] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2843])) && s.b[2844]) {
            s.store_exp_neg_input(2027, 1960);
        }

        s.b[2845] = ((-s.v[1960]) < 0.0);
        s.v[2845] = if s.b[2845] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2843])) && (!s.b[2844])) && s.b[2845]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(1960)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(1960)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2843])) && (!s.b[2844])) && (!s.b[2845])) {
            s.store_scaled_offset_ad(2027, A::mul_offset_lhs(A::neg(s.ad_value(1960)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(1960)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(1960)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2843])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0)));
        }

        s.b[2846] = (s.v[1960] > s.v[1933]);
        s.v[2846] = if s.b[2846] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2843])) && s.b[2846]) {
            s.store_neg(1996, 1996);
        }

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) {
            s.store_ad_value(1942, A::add_scaled_product(s.ad_value(1996), (-1.0), s.ad_value(1937), A::sub(s.ad_value(1890), s.ad_value(1960)), -1.0));
            s.store_ad_value(1961, A::add_scaled_product(s.ad_value(1934), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(A::div_scaled_inputs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0), 0.3333333333333333, s.ad_value(1936))))), s.ad_value(1932), 1.0));
        }

        s.b[2847] = (((s.v[1961]) as f64).abs() <= s.v[1933]);
        s.v[2847] = if s.b[2847] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && s.b[2847]) {
            s.store_mul_ad_affine_product_rhs(1996, 1961, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1961), 1.0, A::scale(s.ad_value(1961), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2848] = ((((-s.v[1961])) as f64).abs() < 230.25850929940458);
        s.v[2848] = if s.b[2848] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2847])) && s.b[2848]) {
            s.store_exp_neg_input(2027, 1961);
        }

        s.b[2849] = ((-s.v[1961]) < 0.0);
        s.v[2849] = if s.b[2849] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2847])) && (!s.b[2848])) && s.b[2849]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(1961)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(1961)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2847])) && (!s.b[2848])) && (!s.b[2849])) {
            s.store_scaled_offset_ad(2027, A::mul_offset_lhs(A::neg(s.ad_value(1961)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(1961)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(1961)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2847])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1961)), (-1.0)));
        }

        s.b[2850] = (s.v[1961] > s.v[1933]);
        s.v[2850] = if s.b[2850] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2847])) && s.b[2850]) {
            s.store_neg(1996, 1996);
        }

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) {
            s.store_ad_value(1943, A::add_scaled_product(s.ad_value(1996), (-1.0), s.ad_value(1937), A::sub(s.ad_value(1890), s.ad_value(1961)), -1.0));
            s.store_ad_value(1962, A::add_scaled_product(s.ad_value(1934), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(A::div_scaled_inputs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0), 0.5, s.ad_value(1936))))), s.ad_value(1932), 1.0));
        }

        s.b[2851] = (((s.v[1962]) as f64).abs() <= s.v[1933]);
        s.v[2851] = if s.b[2851] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && s.b[2851]) {
            s.store_mul_ad_affine_product_rhs(1996, 1962, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1962), 1.0, A::scale(s.ad_value(1962), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2852] = ((((-s.v[1962])) as f64).abs() < 230.25850929940458);
        s.v[2852] = if s.b[2852] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2851])) && s.b[2852]) {
            s.store_exp_neg_input(2027, 1962);
        }

        s.b[2853] = ((-s.v[1962]) < 0.0);
        s.v[2853] = if s.b[2853] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2851])) && (!s.b[2852])) && s.b[2853]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(1962)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(1962)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2851])) && (!s.b[2852])) && (!s.b[2853])) {
            s.store_scaled_offset_ad(2027, A::mul_offset_lhs(A::neg(s.ad_value(1962)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(1962)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(1962)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2851])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1962)), (-1.0)));
        }

        s.b[2854] = (s.v[1962] > s.v[1933]);
        s.v[2854] = if s.b[2854] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2851])) && s.b[2854]) {
            s.store_neg(1996, 1996);
        }

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) {
            s.store_ad_value(1944, A::add_scaled_product(s.ad_value(1996), (-1.0), s.ad_value(1937), A::sub(s.ad_value(1890), s.ad_value(1962)), -1.0));
            s.store_ad_value(1963, A::add_scaled_product(s.ad_value(1934), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(A::div_scaled_inputs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0), 0.6666666666666666, s.ad_value(1936))))), s.ad_value(1932), 1.0));
        }

        s.b[2855] = (((s.v[1963]) as f64).abs() <= s.v[1933]);
        s.v[2855] = if s.b[2855] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && s.b[2855]) {
            s.store_mul_ad_affine_product_rhs(1996, 1963, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1963), 1.0, A::scale(s.ad_value(1963), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2856] = ((((-s.v[1963])) as f64).abs() < 230.25850929940458);
        s.v[2856] = if s.b[2856] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_45(
        s: &mut Scratch,
    ) {
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2855])) && s.b[2856]) {
            s.store_exp_neg_input(2027, 1963);
        }

        s.b[2857] = ((-s.v[1963]) < 0.0);
        s.v[2857] = if s.b[2857] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2855])) && (!s.b[2856])) && s.b[2857]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(1963)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(1963)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2855])) && (!s.b[2856])) && (!s.b[2857])) {
            s.store_scaled_offset_ad(2027, A::mul_offset_lhs(A::neg(s.ad_value(1963)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(1963)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(1963)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2855])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1963)), (-1.0)));
        }

        s.b[2858] = (s.v[1963] > s.v[1933]);
        s.v[2858] = if s.b[2858] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2855])) && s.b[2858]) {
            s.store_neg(1996, 1996);
        }

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) {
            s.store_ad_value(1945, A::add_scaled_product(s.ad_value(1996), (-1.0), s.ad_value(1937), A::sub(s.ad_value(1890), s.ad_value(1963)), -1.0));
            s.store_ad_value(1964, A::add_scaled_product(s.ad_value(1934), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(A::div_scaled_inputs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0), 0.8333333333333333, s.ad_value(1936))))), s.ad_value(1932), 1.0));
        }

        s.b[2859] = (((s.v[1964]) as f64).abs() <= s.v[1933]);
        s.v[2859] = if s.b[2859] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && s.b[2859]) {
            s.store_mul_ad_affine_product_rhs(1996, 1964, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1964), 1.0, A::scale(s.ad_value(1964), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2860] = ((((-s.v[1964])) as f64).abs() < 230.25850929940458);
        s.v[2860] = if s.b[2860] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2859])) && s.b[2860]) {
            s.store_exp_neg_input(2027, 1964);
        }

        s.b[2861] = ((-s.v[1964]) < 0.0);
        s.v[2861] = if s.b[2861] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2859])) && (!s.b[2860])) && s.b[2861]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(1964)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(1964)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2859])) && (!s.b[2860])) && (!s.b[2861])) {
            s.store_scaled_offset_ad(2027, A::mul_offset_lhs(A::neg(s.ad_value(1964)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(1964)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(1964)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2859])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1964)), (-1.0)));
        }

        s.b[2862] = (s.v[1964] > s.v[1933]);
        s.v[2862] = if s.b[2862] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2859])) && s.b[2862]) {
            s.store_neg(1996, 1996);
        }

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) {
            s.store_ad_value(1946, A::add_scaled_product(s.ad_value(1996), (-1.0), s.ad_value(1937), A::sub(s.ad_value(1890), s.ad_value(1964)), -1.0));
        }

        s.b[2863] = (s.v[831] < 0.0);
        s.v[2863] = if s.b[2863] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && s.b[2863]) {
            s.copy_ad(2027, 1942);
            s.copy_ad(1942, 1946);
            s.copy_ad(1946, 2027);
            s.copy_ad(2027, 1943);
            s.copy_ad(1943, 1945);
            s.copy_ad(1945, 2027);
        }

        s.b[2864] = (s.v[1] == 9.0);
        s.v[2864] = if s.b[2864] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_ad_value(1960, A::add_scaled_product(s.ad_value(1934), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(A::div_scaled_inputs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0), 0.1, s.ad_value(1936))))), s.ad_value(1932), 1.0));
        }

        s.b[2865] = (((s.v[1960]) as f64).abs() <= s.v[1933]);
        s.v[2865] = if s.b[2865] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2865]) {
            s.store_mul_ad_affine_product_rhs(1996, 1960, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1960), 1.0, A::scale(s.ad_value(1960), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2866] = ((((-s.v[1960])) as f64).abs() < 230.25850929940458);
        s.v[2866] = if s.b[2866] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2865])) && s.b[2866]) {
            s.store_exp_neg_input(2027, 1960);
        }

        s.b[2867] = ((-s.v[1960]) < 0.0);
        s.v[2867] = if s.b[2867] { 1.0 } else { 0.0 };

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2865])) && (!s.b[2866])) && s.b[2867]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(1960)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(1960)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2865])) && (!s.b[2866])) && (!s.b[2867])) {
            s.store_scaled_offset_ad(2027, A::mul_offset_lhs(A::neg(s.ad_value(1960)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(1960)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(1960)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2865])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0)));
        }

        s.b[2868] = (s.v[1960] > s.v[1933]);
        s.v[2868] = if s.b[2868] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2865])) && s.b[2868]) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_ad_value(1942, A::add_scaled_product(s.ad_value(1996), (-1.0), s.ad_value(1937), A::sub(s.ad_value(1890), s.ad_value(1960)), -1.0));
            s.store_ad_value(1961, A::add_scaled_product(s.ad_value(1934), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(A::div_scaled_inputs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0), 0.2, s.ad_value(1936))))), s.ad_value(1932), 1.0));
        }

        s.b[2869] = (((s.v[1961]) as f64).abs() <= s.v[1933]);
        s.v[2869] = if s.b[2869] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2869]) {
            s.store_mul_ad_affine_product_rhs(1996, 1961, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1961), 1.0, A::scale(s.ad_value(1961), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2870] = ((((-s.v[1961])) as f64).abs() < 230.25850929940458);
        s.v[2870] = if s.b[2870] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2869])) && s.b[2870]) {
            s.store_exp_neg_input(2027, 1961);
        }

        s.b[2871] = ((-s.v[1961]) < 0.0);
        s.v[2871] = if s.b[2871] { 1.0 } else { 0.0 };

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2869])) && (!s.b[2870])) && s.b[2871]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(1961)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(1961)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2869])) && (!s.b[2870])) && (!s.b[2871])) {
            s.store_scaled_offset_ad(2027, A::mul_offset_lhs(A::neg(s.ad_value(1961)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(1961)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(1961)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2869])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1961)), (-1.0)));
        }

        s.b[2872] = (s.v[1961] > s.v[1933]);
        s.v[2872] = if s.b[2872] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2869])) && s.b[2872]) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_ad_value(1943, A::add_scaled_product(s.ad_value(1996), (-1.0), s.ad_value(1937), A::sub(s.ad_value(1890), s.ad_value(1961)), -1.0));
            s.store_ad_value(1962, A::add_scaled_product(s.ad_value(1934), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(A::div_scaled_inputs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0), 0.3, s.ad_value(1936))))), s.ad_value(1932), 1.0));
        }

        s.b[2873] = (((s.v[1962]) as f64).abs() <= s.v[1933]);
        s.v[2873] = if s.b[2873] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2873]) {
            s.store_mul_ad_affine_product_rhs(1996, 1962, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1962), 1.0, A::scale(s.ad_value(1962), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2874] = ((((-s.v[1962])) as f64).abs() < 230.25850929940458);
        s.v[2874] = if s.b[2874] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2873])) && s.b[2874]) {
            s.store_exp_neg_input(2027, 1962);
        }

        s.b[2875] = ((-s.v[1962]) < 0.0);
        s.v[2875] = if s.b[2875] { 1.0 } else { 0.0 };

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2873])) && (!s.b[2874])) && s.b[2875]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(1962)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(1962)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2873])) && (!s.b[2874])) && (!s.b[2875])) {
            s.store_scaled_offset_ad(2027, A::mul_offset_lhs(A::neg(s.ad_value(1962)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(1962)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(1962)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2873])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1962)), (-1.0)));
        }

        s.b[2876] = (s.v[1962] > s.v[1933]);
        s.v[2876] = if s.b[2876] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2873])) && s.b[2876]) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_ad_value(1944, A::add_scaled_product(s.ad_value(1996), (-1.0), s.ad_value(1937), A::sub(s.ad_value(1890), s.ad_value(1962)), -1.0));
            s.store_ad_value(1963, A::add_scaled_product(s.ad_value(1934), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(A::div_scaled_inputs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0), 0.4, s.ad_value(1936))))), s.ad_value(1932), 1.0));
        }

        s.b[2877] = (((s.v[1963]) as f64).abs() <= s.v[1933]);
        s.v[2877] = if s.b[2877] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2877]) {
            s.store_mul_ad_affine_product_rhs(1996, 1963, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1963), 1.0, A::scale(s.ad_value(1963), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2878] = ((((-s.v[1963])) as f64).abs() < 230.25850929940458);
        s.v[2878] = if s.b[2878] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2877])) && s.b[2878]) {
            s.store_exp_neg_input(2027, 1963);
        }

        s.b[2879] = ((-s.v[1963]) < 0.0);
        s.v[2879] = if s.b[2879] { 1.0 } else { 0.0 };

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2877])) && (!s.b[2878])) && s.b[2879]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(1963)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(1963)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2877])) && (!s.b[2878])) && (!s.b[2879])) {
            s.store_scaled_offset_ad(2027, A::mul_offset_lhs(A::neg(s.ad_value(1963)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(1963)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(1963)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2877])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1963)), (-1.0)));
        }

        s.b[2880] = (s.v[1963] > s.v[1933]);
        s.v[2880] = if s.b[2880] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2877])) && s.b[2880]) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_ad_value(1945, A::add_scaled_product(s.ad_value(1996), (-1.0), s.ad_value(1937), A::sub(s.ad_value(1890), s.ad_value(1963)), -1.0));
            s.store_ad_value(1964, A::add_scaled_product(s.ad_value(1934), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(A::div_scaled_inputs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0), 0.5, s.ad_value(1936))))), s.ad_value(1932), 1.0));
        }

        s.b[2881] = (((s.v[1964]) as f64).abs() <= s.v[1933]);
        s.v[2881] = if s.b[2881] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2881]) {
            s.store_mul_ad_affine_product_rhs(1996, 1964, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1964), 1.0, A::scale(s.ad_value(1964), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2882] = ((((-s.v[1964])) as f64).abs() < 230.25850929940458);
        s.v[2882] = if s.b[2882] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2881])) && s.b[2882]) {
            s.store_exp_neg_input(2027, 1964);
        }

        s.b[2883] = ((-s.v[1964]) < 0.0);
        s.v[2883] = if s.b[2883] { 1.0 } else { 0.0 };

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2881])) && (!s.b[2882])) && s.b[2883]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(1964)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(1964)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2881])) && (!s.b[2882])) && (!s.b[2883])) {
            s.store_scaled_offset_ad(2027, A::mul_offset_lhs(A::neg(s.ad_value(1964)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(1964)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(1964)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2881])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1964)), (-1.0)));
        }

        s.b[2884] = (s.v[1964] > s.v[1933]);
        s.v[2884] = if s.b[2884] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2881])) && s.b[2884]) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_ad_value(1946, A::add_scaled_product(s.ad_value(1996), (-1.0), s.ad_value(1937), A::sub(s.ad_value(1890), s.ad_value(1964)), -1.0));
            s.store_ad_value(1965, A::add_scaled_product(s.ad_value(1934), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(A::div_scaled_inputs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0), 0.6, s.ad_value(1936))))), s.ad_value(1932), 1.0));
        }

        s.b[2885] = (((s.v[1965]) as f64).abs() <= s.v[1933]);
        s.v[2885] = if s.b[2885] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2885]) {
            s.store_mul_ad_affine_product_rhs(1996, 1965, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1965), 1.0, A::scale(s.ad_value(1965), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2886] = ((((-s.v[1965])) as f64).abs() < 230.25850929940458);
        s.v[2886] = if s.b[2886] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2885])) && s.b[2886]) {
            s.store_exp_neg_input(2027, 1965);
        }

        s.b[2887] = ((-s.v[1965]) < 0.0);
        s.v[2887] = if s.b[2887] { 1.0 } else { 0.0 };

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2885])) && (!s.b[2886])) && s.b[2887]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(1965)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(1965)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2885])) && (!s.b[2886])) && (!s.b[2887])) {
            s.store_scaled_offset_ad(2027, A::mul_offset_lhs(A::neg(s.ad_value(1965)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(1965)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(1965)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2885])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1965)), (-1.0)));
        }

        s.b[2888] = (s.v[1965] > s.v[1933]);
        s.v[2888] = if s.b[2888] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2885])) && s.b[2888]) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_ad_value(1947, A::add_scaled_product(s.ad_value(1996), (-1.0), s.ad_value(1937), A::sub(s.ad_value(1890), s.ad_value(1965)), -1.0));
            s.store_ad_value(1966, A::add_scaled_product(s.ad_value(1934), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(A::div_scaled_inputs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0), 0.7, s.ad_value(1936))))), s.ad_value(1932), 1.0));
        }

        s.b[2889] = (((s.v[1966]) as f64).abs() <= s.v[1933]);
        s.v[2889] = if s.b[2889] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2889]) {
            s.store_mul_ad_affine_product_rhs(1996, 1966, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1966), 1.0, A::scale(s.ad_value(1966), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2890] = ((((-s.v[1966])) as f64).abs() < 230.25850929940458);
        s.v[2890] = if s.b[2890] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2889])) && s.b[2890]) {
            s.store_exp_neg_input(2027, 1966);
        }

        s.b[2891] = ((-s.v[1966]) < 0.0);
        s.v[2891] = if s.b[2891] { 1.0 } else { 0.0 };

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2889])) && (!s.b[2890])) && s.b[2891]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(1966)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(1966)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2889])) && (!s.b[2890])) && (!s.b[2891])) {
            s.store_scaled_offset_ad(2027, A::mul_offset_lhs(A::neg(s.ad_value(1966)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(1966)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(1966)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2889])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1966)), (-1.0)));
        }

        s.b[2892] = (s.v[1966] > s.v[1933]);
        s.v[2892] = if s.b[2892] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2889])) && s.b[2892]) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_ad_value(1948, A::add_scaled_product(s.ad_value(1996), (-1.0), s.ad_value(1937), A::sub(s.ad_value(1890), s.ad_value(1966)), -1.0));
            s.store_ad_value(1967, A::add_scaled_product(s.ad_value(1934), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(A::div_scaled_inputs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0), 0.8, s.ad_value(1936))))), s.ad_value(1932), 1.0));
        }

        s.b[2893] = (((s.v[1967]) as f64).abs() <= s.v[1933]);
        s.v[2893] = if s.b[2893] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2893]) {
            s.store_mul_ad_affine_product_rhs(1996, 1967, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1967), 1.0, A::scale(s.ad_value(1967), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2894] = ((((-s.v[1967])) as f64).abs() < 230.25850929940458);
        s.v[2894] = if s.b[2894] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2893])) && s.b[2894]) {
            s.store_exp_neg_input(2027, 1967);
        }

        s.b[2895] = ((-s.v[1967]) < 0.0);
        s.v[2895] = if s.b[2895] { 1.0 } else { 0.0 };

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2893])) && (!s.b[2894])) && s.b[2895]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(1967)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(1967)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2893])) && (!s.b[2894])) && (!s.b[2895])) {
            s.store_scaled_offset_ad(2027, A::mul_offset_lhs(A::neg(s.ad_value(1967)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(1967)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(1967)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2893])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1967)), (-1.0)));
        }

        s.b[2896] = (s.v[1967] > s.v[1933]);
        s.v[2896] = if s.b[2896] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2893])) && s.b[2896]) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_ad_value(1949, A::add_scaled_product(s.ad_value(1996), (-1.0), s.ad_value(1937), A::sub(s.ad_value(1890), s.ad_value(1967)), -1.0));
            s.store_ad_value(1968, A::add_scaled_product(s.ad_value(1934), 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(A::div_scaled_inputs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0), 0.9, s.ad_value(1936))))), s.ad_value(1932), 1.0));
        }

        s.b[2897] = (((s.v[1968]) as f64).abs() <= s.v[1933]);
        s.v[2897] = if s.b[2897] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2897]) {
            s.store_mul_ad_affine_product_rhs(1996, 1968, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1968), 1.0, A::scale(s.ad_value(1968), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2898] = ((((-s.v[1968])) as f64).abs() < 230.25850929940458);
        s.v[2898] = if s.b[2898] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2897])) && s.b[2898]) {
            s.store_exp_neg_input(2027, 1968);
        }

        s.b[2899] = ((-s.v[1968]) < 0.0);
        s.v[2899] = if s.b[2899] { 1.0 } else { 0.0 };

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2897])) && (!s.b[2898])) && s.b[2899]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(1968)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(1968)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2897])) && (!s.b[2898])) && (!s.b[2899])) {
            s.store_scaled_offset_ad(2027, A::mul_offset_lhs(A::neg(s.ad_value(1968)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(1968)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(1968)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2897])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1968)), (-1.0)));
        }

        s.b[2900] = (s.v[1968] > s.v[1933]);
        s.v[2900] = if s.b[2900] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2897])) && s.b[2900]) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_ad_value(1950, A::add_scaled_product(s.ad_value(1996), (-1.0), s.ad_value(1937), A::sub(s.ad_value(1890), s.ad_value(1968)), -1.0));
        }

        s.b[2901] = (s.v[831] < 0.0);
        s.v[2901] = if s.b[2901] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2901]) {
            s.copy_ad(2027, 1942);
            s.copy_ad(1942, 1950);
        }

    }

    pub(super) fn stamp_transient_block_46(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2901]) {
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

        s.v[1983] = 0.0;

        s.v[1984] = 0.0;

        s.v[1978] = 0.0;

        s.v[1979] = 0.0;

        s.b[2902] = (s.v[1] != 0.0);
        s.v[2902] = if s.b[2902] { 1.0 } else { 0.0 };

        if s.b[2902] {
            s.store_sub_ad_rhs(1983, 1934, A::mul3_scaled_output(s.ad_value(831), s.ad_value(1893), s.ad_value(1932), 0.5));
            s.store_add_ad_rhs(1984, 1934, A::mul3_scaled_output(s.ad_value(831), s.ad_value(1893), s.ad_value(1932), 0.5));
            s.store_scalar(1978, 0.0);
            s.store_scalar(1979, 0.0);
        }

        s.b[2903] = (s.v[1983] > 0.0);
        s.v[2903] = if s.b[2903] { 1.0 } else { 0.0 };

        s.b[2904] = (((s.v[1983]) as f64).abs() <= s.v[1933]);
        s.v[2904] = if s.b[2904] { 1.0 } else { 0.0 };

        if ((s.b[2902] && s.b[2903]) && s.b[2904]) {
            s.store_mul_ad_affine_product_rhs(1997, 1983, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1983), 1.0, A::scale(s.ad_value(1983), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2905] = ((((-s.v[1983])) as f64).abs() < 230.25850929940458);
        s.v[2905] = if s.b[2905] { 1.0 } else { 0.0 };

        if (((s.b[2902] && s.b[2903]) && (!s.b[2904])) && s.b[2905]) {
            s.store_exp_neg_input(2027, 1983);
        }

        s.b[2906] = ((-s.v[1983]) < 0.0);
        s.v[2906] = if s.b[2906] { 1.0 } else { 0.0 };

        if ((((s.b[2902] && s.b[2903]) && (!s.b[2904])) && (!s.b[2905])) && s.b[2906]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(1983)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(1983)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2902] && s.b[2903]) && (!s.b[2904])) && (!s.b[2905])) && (!s.b[2906])) {
            s.store_scaled_offset_ad(2027, A::mul_offset_lhs(A::neg(s.ad_value(1983)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(1983)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(1983)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2902] && s.b[2903]) && (!s.b[2904])) {
            s.store_mul_sqrt_ad_rhs(1997, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1983)), (-1.0)));
        }

        s.b[2907] = (s.v[1983] > s.v[1933]);
        s.v[2907] = if s.b[2907] { 1.0 } else { 0.0 };

        if (((s.b[2902] && s.b[2903]) && (!s.b[2904])) && s.b[2907]) {
            s.store_neg(1997, 1997);
        }

        if (s.b[2902] && s.b[2903]) {
            s.store_ad_value(1978, A::add_scaled_product(s.ad_value(1997), (-1.0), s.ad_value(1937), A::sub(s.ad_value(1890), s.ad_value(1983)), -1.0));
        }

        s.b[2908] = (s.v[1984] > 0.0);
        s.v[2908] = if s.b[2908] { 1.0 } else { 0.0 };

        s.b[2909] = (((s.v[1984]) as f64).abs() <= s.v[1933]);
        s.v[2909] = if s.b[2909] { 1.0 } else { 0.0 };

        if ((s.b[2902] && s.b[2908]) && s.b[2909]) {
            s.store_mul_ad_affine_product_rhs(1997, 1984, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1984), 1.0, A::scale(s.ad_value(1984), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
        }

        s.b[2910] = ((((-s.v[1984])) as f64).abs() < 230.25850929940458);
        s.v[2910] = if s.b[2910] { 1.0 } else { 0.0 };

        if (((s.b[2902] && s.b[2908]) && (!s.b[2909])) && s.b[2910]) {
            s.store_exp_neg_input(2027, 1984);
        }

        s.b[2911] = ((-s.v[1984]) < 0.0);
        s.v[2911] = if s.b[2911] { 1.0 } else { 0.0 };

        if ((((s.b[2902] && s.b[2908]) && (!s.b[2909])) && (!s.b[2910])) && s.b[2911]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(1984)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(1984)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2902] && s.b[2908]) && (!s.b[2909])) && (!s.b[2910])) && (!s.b[2911])) {
            s.store_scaled_offset_ad(2027, A::mul_offset_lhs(A::neg(s.ad_value(1984)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(1984)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(1984)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2902] && s.b[2908]) && (!s.b[2909])) {
            s.store_mul_sqrt_ad_rhs(1997, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1984)), (-1.0)));
        }

        s.b[2912] = (s.v[1984] > s.v[1933]);
        s.v[2912] = if s.b[2912] { 1.0 } else { 0.0 };

        if (((s.b[2902] && s.b[2908]) && (!s.b[2909])) && s.b[2912]) {
            s.store_neg(1997, 1997);
        }

        if (s.b[2902] && s.b[2908]) {
            s.store_ad_value(1979, A::add_scaled_product(s.ad_value(1997), (-1.0), s.ad_value(1937), A::sub(s.ad_value(1890), s.ad_value(1984)), -1.0));
        }

        s.store_scale(871, 811, s.v[718]);

        s.store_scale(872, 812, s.v[718]);

        s.store_scale(873, 813, s.v[718]);

        s.store_scale(874, 814, s.v[718]);

        s.store_scale(875, 815, s.v[718]);

        s.store_scale(876, 816, s.v[718]);

        s.store_scale(877, 817, s.v[718]);

        s.b[2913] = (s.v[831] > 0.0);
        s.v[2913] = if s.b[2913] { 1.0 } else { 0.0 };

        s.b[2914] = (s.v[300] > 0.0);
        s.v[2914] = if s.b[2914] { 1.0 } else { 0.0 };

        s.b[2915] = (s.v[301] > 0.0);
        s.v[2915] = if s.b[2915] { 1.0 } else { 0.0 };

        s.b[2916] = (s.v[302] > 0.0);
        s.v[2916] = if s.b[2916] { 1.0 } else { 0.0 };

        s.b[2917] = (s.v[303] > 0.0);
        s.v[2917] = if s.b[2917] { 1.0 } else { 0.0 };

        s.b[2918] = (s.v[304] > 0.0);
        s.v[2918] = if s.b[2918] { 1.0 } else { 0.0 };

        s.b[2919] = (s.v[305] > 0.0);
        s.v[2919] = if s.b[2919] { 1.0 } else { 0.0 };

        s.b[2920] = (s.v[306] > 0.0);
        s.v[2920] = if s.b[2920] { 1.0 } else { 0.0 };

        s.store_scaled_voltage(1969, ctx, nodes, Some(12), None, s.v[3]);

        s.store_scaled_voltage(1970, ctx, nodes, Some(13), None, s.v[3]);

        s.store_scaled_voltage(1971, ctx, nodes, Some(14), None, s.v[3]);

        s.store_scaled_voltage(1972, ctx, nodes, Some(15), None, s.v[3]);

        s.store_scaled_voltage(1973, ctx, nodes, Some(16), None, s.v[3]);

        s.store_scaled_voltage(1974, ctx, nodes, Some(17), None, s.v[3]);

        s.store_scaled_voltage(1975, ctx, nodes, Some(18), None, s.v[3]);

        s.store_scaled_voltage(1976, ctx, nodes, Some(19), None, s.v[3]);

        s.store_scaled_voltage(1977, ctx, nodes, Some(20), None, s.v[3]);

        s.v[1995] = 0.0;

        s.b[2921] = (s.v[1] != 0.0);
        s.v[2921] = if s.b[2921] { 1.0 } else { 0.0 };

        if s.b[2921] {
            s.store_ad_value(1995, A::div_scaled_product3(s.ad_value(307), s.ad_value(1888), s.ad_value(716), 1.0, A::mul(s.ad_value(1904), s.ad_value(1906)), 1.0));
            s.store_mul_ad_product_lhs(2018, A::square(s.ad_value(1907)), s.ad_value(1888), 1888);
        }

        s.b[2922] = (s.v[1] == 1.0);
        s.v[2922] = if s.b[2922] { 1.0 } else { 0.0 };

        if (s.b[2921] && s.b[2922]) {
            s.store_sub(1992, 1979, 1978);
            s.store_ad_value(1993, A::add_scaled_inputs3(s.ad_value(1978), 6.0, s.ad_value(1979), 6.0, s.ad_value(1969), (-12.0)));
        }

        s.b[2923] = (s.v[1] == 2.0);
        s.v[2923] = if s.b[2923] { 1.0 } else { 0.0 };

        if ((s.b[2921] && (!s.b[2922])) && s.b[2923]) {
            s.store_ad_value(1992, A::sub_scaled_inputs(A::add_scaled_inputs3(s.ad_value(1978), (-7.0), s.ad_value(1969), (-3.0), s.ad_value(1970), 12.0), 0.2, s.ad_value(1979), (2.0 * 0.2)));
            s.store_scaled_add_ad_lhs(1993, A::add_scaled_inputs3(s.ad_value(1978), (-4.0), s.ad_value(1969), 9.0, s.ad_value(1970), (-6.0)), 1979, ((-18.0) / 5.0));
        }

        s.b[2924] = (s.v[1] == 3.0);
        s.v[2924] = if s.b[2924] { 1.0 } else { 0.0 };

        if (((s.b[2921] && (!s.b[2922])) && (!s.b[2923])) && s.b[2924]) {
            s.store_ad_value(1992, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(1978), (-13.0), s.ad_value(1969), (-6.0), s.ad_value(1970), 24.0), 0.14285714285714285, s.ad_value(1971), ((-6.0) * 0.14285714285714285), s.ad_value(1979), 0.14285714285714285));
            s.store_ad_value(1993, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(1978), 180.0, s.ad_value(1969), (-408.0), s.ad_value(1970), 288.0), 0.14285714285714285, s.ad_value(1971), ((-72.0) * 0.14285714285714285), s.ad_value(1979), (12.0 * 0.14285714285714285)));
        }

        s.b[2925] = (s.v[1] == 5.0);
        s.v[2925] = if s.b[2925] { 1.0 } else { 0.0 };

        if ((((s.b[2921] && (!s.b[2922])) && (!s.b[2923])) && (!s.b[2924])) && s.b[2925]) {
            s.store_ad_value(1992, A::add_scaled_inputs3(A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(1978), (-181.0), s.ad_value(1969), (-84.0), s.ad_value(1972), 24.0), 1.0, s.ad_value(1973), (-6.0), s.ad_value(1971), (-90.0)), 0.015384615384615385, s.ad_value(1979), 0.015384615384615385, s.ad_value(1970), (336.0 * 0.015384615384615385)));
            s.store_ad_value(1993, A::add_scaled_inputs3(A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(1972), 432.0, s.ad_value(1973), (-108.0), s.ad_value(1971), (-1620.0)), 1.0, s.ad_value(1979), 18.0, s.ad_value(1978), 3762.0), 0.015384615384615385, s.ad_value(1969), ((-8532.0) * 0.015384615384615385), s.ad_value(1970), (6048.0 * 0.015384615384615385)));
        }

        s.b[2926] = (s.v[1] == 9.0);
        s.v[2926] = if s.b[2926] { 1.0 } else { 0.0 };

        if (((((s.b[2921] && (!s.b[2922])) && (!s.b[2923])) && (!s.b[2924])) && (!s.b[2925])) && s.b[2926]) {
            let assign66170_ad_e88041: A = A::sub_scaled_inputs(A::add_scaled_inputs3(A::add_scaled_inputs3(A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(1974), 1680.0, s.ad_value(1972), 23400.0, s.ad_value(1979), 5.0), 1.0, s.ad_value(1971), (-87330.0), s.ad_value(1976), 120.0), 1.0, s.ad_value(1975), (-450.0), s.ad_value(1969), (-81480.0)), 1.0, s.ad_value(1970), 325920.0, s.ad_value(1978), (-175565.0)), 2.6434745829918846e-5, s.ad_value(1977), (30.0 * 2.6434745829918846e-5));
            s.store_sub_scaled_ad_lhs(1992, assign66170_ad_e88041, 1973, (30.0 / 181.0));
        }

        if (((((s.b[2921] && (!s.b[2922])) && (!s.b[2923])) && (!s.b[2924])) && (!s.b[2925])) && s.b[2926]) {
            let assign66180_ad_e88106: A = A::add_scaled_inputs(A::add_scaled_inputs3(A::add_scaled_inputs3(A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(1975), (-13500.0), s.ad_value(1972), 702000.0, s.ad_value(1971), (-2619900.0)), 1.0, s.ad_value(1969), (-13793100.0), s.ad_value(1970), 9777600.0), 1.0, s.ad_value(1978), 6081750.0, s.ad_value(1979), 150.0), 1.0, s.ad_value(1976), 3600.0, s.ad_value(1977), (-900.0)), 2.6434745829918846e-5, s.ad_value(1974), (50400.0 * 2.6434745829918846e-5));
            s.store_sub_scaled_ad_lhs(1993, assign66180_ad_e88106, 1973, (900.0 / 181.0));
        }

        if (((((s.b[2921] && (!s.b[2922])) && (!s.b[2923])) && (!s.b[2924])) && (!s.b[2925])) && (!s.b[2926])) {
            s.store_scalar(1992, 0.0);
            s.store_scalar(1993, 0.0);
        }

        if s.b[2921] {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1969), s.ad_value(1937)), 1890);
        }

        s.b[2927] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[2927] = if s.b[2927] { 1.0 } else { 0.0 };

        if (s.b[2921] && s.b[2927]) {
            s.store_div(2016, 2027, 1940);
        }

        s.b[2928] = (s.v[2027] < (-s.v[1941]));
        s.v[2928] = if s.b[2928] { 1.0 } else { 0.0 };

        if ((s.b[2921] && (!s.b[2927])) && s.b[2928]) {
            s.store_neg(1999, 2027);
            s.store_scaled_div(2000, 1999, 1940, 1.25);
            s.store_scaled_sub_ad(2001, A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(2000), (-6.0), A::offset(s.ad_value(2000), (-6.0))), 64.0)), 0.5);
            s.store_ad_value(2002, A::add_scaled_products(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, s.ad_value(1939), A::offset(s.ad_value(2001), 1.0), 1.0));
            s.store_ad_value(2003, A::add_scaled_inputs3(s.ad_value(1999), 2.0, s.ad_value(2001), (-2.0), s.ad_value(1939), -1.0));
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
            s.store_add(824, 2002, 2003);
            s.store_ad_value(823, A::add_scaled_square_product(s.ad_value(824), 1.0, s.ad_value(2004), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0));
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[2929] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[2929] = if s.b[2929] { 1.0 } else { 0.0 };

        if (((s.b[2921] && (!s.b[2927])) && s.b[2928]) && s.b[2929]) {
            s.store_exp(2005, 2015);
        }

        s.b[2930] = (s.v[2015] < 0.0);
        s.v[2930] = if s.b[2930] { 1.0 } else { 0.0 };

        if ((((s.b[2921] && (!s.b[2927])) && s.b[2928]) && (!s.b[2929])) && s.b[2930]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2015), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2015), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2921] && (!s.b[2927])) && s.b[2928]) && (!s.b[2929])) && (!s.b[2930])) {
            s.store_scaled_offset_ad(2005, A::mul_offset_lhs(s.ad_value(2015), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2015), (-230.25850929940458), A::scale_offset(s.ad_value(2015), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2921] && (!s.b[2927])) && s.b[2928]) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul_scaled_output(s.ad_value(1939), s.ad_value(2005), 0.5));
            s.store_ad_value(2007, A::add_scaled_inputs_product(s.ad_value(1999), 2.0, s.ad_value(2015), (-2.0), s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0)), 1.0));
            s.store_ad_value(2008, A::add_scaled_products(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0));
            s.store_ad_value(2009, A::add_scaled_square_product(s.ad_value(2007), 1.0, s.ad_value(2006), s.ad_value(2008), (-4.0)));
            s.store_ad_value(2012, A::div_scaled_inputs(s.ad_value(2008), 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0));
            s.store_neg_ad(2016, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if ((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_ad_value(2011, A::mul_offset_rhs(A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.b[2931] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.v[2931] = if s.b[2931] { 1.0 } else { 0.0 };

        if (((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) && s.b[2931]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[2932] = ((-s.v[2011]) < 0.0);
        s.v[2932] = if s.b[2932] { 1.0 } else { 0.0 };

        if ((((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) && (!s.b[2931])) && s.b[2932]) {
            s.store_div_from_scalar_offset_ad(2009, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(2011)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) && (!s.b[2931])) && (!s.b[2932])) {
            s.store_scaled_offset_ad(2009, A::mul_offset_lhs(A::neg(s.ad_value(2011)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(2011)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(2011)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_ad_value(2013, A::add_scaled_inputs_product(s.ad_value(2027), 1.0, s.ad_value(1939), 0.5, s.ad_value(1938), A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0)));
        }

        s.b[2933] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[2933] = if s.b[2933] { 1.0 } else { 0.0 };

        if (((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) && s.b[2933]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[2934] = ((-s.v[2013]) < 0.0);
        s.v[2934] = if s.b[2934] { 1.0 } else { 0.0 };

        if ((((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) && (!s.b[2933])) && s.b[2934]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(2013)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) && (!s.b[2933])) && (!s.b[2934])) {
            s.store_scaled_offset_ad(2005, A::mul_offset_lhs(A::neg(s.ad_value(2013)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(2013)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(2013)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul_scaled_lhs(s.ad_value(1939), 0.5, s.ad_value(2005)));
            s.store_ad_value(2007, A::add_scaled_inputs3(s.ad_value(2027), 2.0, s.ad_value(2013), (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0));
            s.store_ad_value(2008, A::add_scaled_products(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0)));
            s.store_ad_value(2009, A::add_scaled_square_product(s.ad_value(2007), 1.0, s.ad_value(2006), s.ad_value(2008), (-4.0)));
            s.store_ad_value(2014, A::div_scaled_inputs(s.ad_value(2008), 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0));
            s.store_add(2016, 2013, 2014);
        }

        s.b[2935] = (((s.v[2016]) as f64).abs() <= s.v[1933]);
        s.v[2935] = if s.b[2935] { 1.0 } else { 0.0 };

        if (s.b[2921] && s.b[2935]) {
            s.store_mul_ad_affine_product_rhs(1996, 2016, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
            s.store_ad_value(1991, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1889), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.25), 0.3333333333333333), (-0.70710678)));
            s.store_ad_value(1990, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1889), 1.0, A::scale(s.ad_value(2016), 0.5), (-0.235702)));
        }

        s.b[2936] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);
        s.v[2936] = if s.b[2936] { 1.0 } else { 0.0 };

        if ((s.b[2921] && (!s.b[2935])) && s.b[2936]) {
            s.store_exp_neg_input(2027, 2016);
        }

        s.b[2937] = ((-s.v[2016]) < 0.0);
        s.v[2937] = if s.b[2937] { 1.0 } else { 0.0 };

        if (((s.b[2921] && (!s.b[2935])) && (!s.b[2936])) && s.b[2937]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(2016)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(2016)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[2921] && (!s.b[2935])) && (!s.b[2936])) && (!s.b[2937])) {
            s.store_scaled_offset_ad(2027, A::mul_offset_lhs(A::neg(s.ad_value(2016)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(2016)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(2016)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (s.b[2921] && (!s.b[2935])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));
        }

        s.b[2938] = (s.v[2016] > s.v[1933]);
        s.v[2938] = if s.b[2938] { 1.0 } else { 0.0 };

        if ((s.b[2921] && (!s.b[2935])) && s.b[2938]) {
            s.store_neg(1996, 1996);
        }

        if (s.b[2921] && (!s.b[2935])) {
            s.store_ad_value(1991, A::div_scaled_product3(s.ad_value(1889), s.ad_value(1889), A::sub_from_scalar(1.0, s.ad_value(2027)), 0.5, s.ad_value(1996), 1.0));
            s.store_add_ad_lhs(1990, A::div(A::add_scaled_square_product(s.ad_value(1991), 1.0, s.ad_value(1889), s.ad_value(1889), (-0.5)), s.ad_value(1996)), 1991);
        }

    }

    pub(super) fn stamp_transient_block_47(
        s: &mut Scratch,
    ) {
        if s.b[2921] {
            s.store_sub(1988, 1937, 1991);
            s.store_div_from_scalar(1989, 1.0, 1988);
            s.store_offset_mul(1987, 1969, 1989, (-1.0));
            s.store_mul_sub_from_scalar_ad_lhs(1986, 1.0, A::mul(A::mul3(s.ad_value(1969), s.ad_value(1990), s.ad_value(1989)), s.ad_value(1989)), 1989);
            s.store_ad_value(2017, A::add_scaled_product(A::mul3(s.ad_value(1986), s.ad_value(1992), s.ad_value(1992)), 1.0, s.ad_value(1987), s.ad_value(1993), 1.0));
            s.store_mul_ad_product_lhs(1994, A::square(s.ad_value(1992)), s.ad_value(1989), 1989);
            s.store_mul(1985, 2018, 1994);
        }

        s.b[2939] = (s.v[0] == (-1.0));
        s.v[2939] = if s.b[2939] { 1.0 } else { 0.0 };

        if (s.b[2921] && s.b[2939]) {
            s.store_div_ad_rhs(1985, 1985, A::offset(A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0));
        }

        if s.b[2921] {
            s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);
            s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);
            s.store_ad_value(2028, A::add_scaled_product(s.ad_value(1993), 1.0, s.ad_value(1994), s.ad_value(1990), (-1.0)));
            s.store_mul_sub_ad_rhs(1951, 2019, s.ad_value(2017), A::div_scaled_product(A::mul3(s.ad_value(1985), s.ad_value(1987), s.ad_value(2028)), s.ad_value(2019), 1.0, s.ad_value(2027), 1.0));
        }

        if (!s.b[2921]) {
            s.store_scalar(2018, 0.0);
        }

        s.b[2940] = (s.v[1] >= 2.0);
        s.v[2940] = if s.b[2940] { 1.0 } else { 0.0 };

        s.b[2941] = (s.v[1] == 2.0);
        s.v[2941] = if s.b[2941] { 1.0 } else { 0.0 };

        if (s.b[2940] && s.b[2941]) {
            s.store_ad_value(1992, A::add_scaled_inputs(A::add_scaled_inputs3(s.ad_value(1978), 2.0, s.ad_value(1969), (-12.0), s.ad_value(1970), 3.0), 0.2, s.ad_value(1979), (7.0 * 0.2)));
            s.store_scaled_add_ad_lhs(1993, A::add_scaled_inputs3(s.ad_value(1979), (-4.0), s.ad_value(1970), 9.0, s.ad_value(1969), (-6.0)), 1978, ((-18.0) / 5.0));
        }

        s.b[2942] = (s.v[1] == 3.0);
        s.v[2942] = if s.b[2942] { 1.0 } else { 0.0 };

        if ((s.b[2940] && (!s.b[2941])) && s.b[2942]) {
            s.store_sub_scaled_ad_lhs(1992, A::add_scaled_inputs3(s.ad_value(1978), 0.5, s.ad_value(1969), (-3.0), s.ad_value(1971), 3.0), 1979, 0.5);
            s.store_ad_value(1993, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(1978), (-48.0), s.ad_value(1969), 288.0, s.ad_value(1970), (-480.0)), 0.14285714285714285, s.ad_value(1971), (288.0 * 0.14285714285714285), s.ad_value(1979), ((-48.0) * 0.14285714285714285)));
        }

        s.b[2943] = (s.v[1] == 5.0);
        s.v[2943] = if s.b[2943] { 1.0 } else { 0.0 };

        if (((s.b[2940] && (!s.b[2941])) && (!s.b[2942])) && s.b[2943]) {
            s.store_ad_value(1992, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(1969), (-291.0), s.ad_value(1970), (-6.0), s.ad_value(1972), (-84.0)), 0.015384615384615385, s.ad_value(1973), (21.0 * 0.015384615384615385), A::add_scaled_inputs3(s.ad_value(1971), (630.0 * 0.007692307692307693), s.ad_value(1979), ((-7.0) * 0.007692307692307693), s.ad_value(1978), (97.0 * 0.007692307692307693)), 1.0));
            s.store_ad_value(1993, A::add_scaled_inputs3(A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(1972), (-1728.0), s.ad_value(1973), 432.0, s.ad_value(1971), 6480.0), 1.0, s.ad_value(1979), (-72.0), s.ad_value(1978), (-1008.0)), 0.015384615384615385, s.ad_value(1969), (6048.0 * 0.015384615384615385), s.ad_value(1970), ((-10152.0) * 0.015384615384615385)));
        }

        s.b[2944] = (s.v[1] == 9.0);
        s.v[2944] = if s.b[2944] { 1.0 } else { 0.0 };

        if ((((s.b[2940] && (!s.b[2941])) && (!s.b[2942])) && (!s.b[2943])) && s.b[2944]) {
            let assign67050_ad_e89555: A = A::add_scaled_inputs3(A::add_scaled_inputs3(A::add_scaled_inputs3(A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(1974), (-5880.0), s.ad_value(1972), (-81900.0), s.ad_value(1971), 305655.0), 1.0, s.ad_value(1976), (-420.0), s.ad_value(1977), 105.0), 1.0, s.ad_value(1969), (-282255.0), s.ad_value(1975), 1575.0), 2.6434745829918846e-5, s.ad_value(1970), (-(5850.0 * 2.6434745829918846e-5)), s.ad_value(1973), (105.0 / 181.0)), 1.0, s.ad_value(1978), (94085.0 * 1.3217372914959423e-5), s.ad_value(1979), (-(35.0 * 1.3217372914959423e-5)));
            s.store_ad_value(1992, assign67050_ad_e89555);
        }

        if ((((s.b[2940] && (!s.b[2941])) && (!s.b[2942])) && (!s.b[2943])) && s.b[2944]) {
            let assign67060_ad_e89608: A = A::sub_scaled_inputs(A::add_scaled_inputs3(A::add_scaled_inputs3(A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(1969), 9777600.0, s.ad_value(1975), 54000.0, s.ad_value(1972), (-2808000.0)), 1.0, s.ad_value(1971), 10479600.0, s.ad_value(1970), (-16413000.0)), 1.0, s.ad_value(1978), (-1629600.0), s.ad_value(1979), (-600.0)), 1.0, s.ad_value(1976), (-14400.0), s.ad_value(1977), 3600.0), 1.0, s.ad_value(1974), 201600.0);
            s.store_ad_value(1993, A::add_scaled_inputs(assign67060_ad_e89608, 2.6434745829918846e-5, s.ad_value(1973), (3600.0 * 0.0055248618784530384)));
        }

        if ((((s.b[2940] && (!s.b[2941])) && (!s.b[2942])) && (!s.b[2943])) && (!s.b[2944])) {
            s.store_scalar(1992, 0.0);
            s.store_scalar(1993, 0.0);
        }

        if s.b[2940] {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1970), s.ad_value(1937)), 1890);
        }

        s.b[2945] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[2945] = if s.b[2945] { 1.0 } else { 0.0 };

        if (s.b[2940] && s.b[2945]) {
            s.store_div(2016, 2027, 1940);
        }

        s.b[2946] = (s.v[2027] < (-s.v[1941]));
        s.v[2946] = if s.b[2946] { 1.0 } else { 0.0 };

        if ((s.b[2940] && (!s.b[2945])) && s.b[2946]) {
            s.store_neg(1999, 2027);
            s.store_scaled_div(2000, 1999, 1940, 1.25);
            s.store_scaled_sub_ad(2001, A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(2000), (-6.0), A::offset(s.ad_value(2000), (-6.0))), 64.0)), 0.5);
            s.store_ad_value(2002, A::add_scaled_products(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, s.ad_value(1939), A::offset(s.ad_value(2001), 1.0), 1.0));
            s.store_ad_value(2003, A::add_scaled_inputs3(s.ad_value(1999), 2.0, s.ad_value(2001), (-2.0), s.ad_value(1939), -1.0));
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
            s.store_add(824, 2002, 2003);
            s.store_ad_value(823, A::add_scaled_square_product(s.ad_value(824), 1.0, s.ad_value(2004), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0));
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[2947] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[2947] = if s.b[2947] { 1.0 } else { 0.0 };

        if (((s.b[2940] && (!s.b[2945])) && s.b[2946]) && s.b[2947]) {
            s.store_exp(2005, 2015);
        }

        s.b[2948] = (s.v[2015] < 0.0);
        s.v[2948] = if s.b[2948] { 1.0 } else { 0.0 };

        if ((((s.b[2940] && (!s.b[2945])) && s.b[2946]) && (!s.b[2947])) && s.b[2948]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2015), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2015), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2940] && (!s.b[2945])) && s.b[2946]) && (!s.b[2947])) && (!s.b[2948])) {
            s.store_scaled_offset_ad(2005, A::mul_offset_lhs(s.ad_value(2015), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2015), (-230.25850929940458), A::scale_offset(s.ad_value(2015), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2940] && (!s.b[2945])) && s.b[2946]) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul_scaled_output(s.ad_value(1939), s.ad_value(2005), 0.5));
            s.store_ad_value(2007, A::add_scaled_inputs_product(s.ad_value(1999), 2.0, s.ad_value(2015), (-2.0), s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0)), 1.0));
            s.store_ad_value(2008, A::add_scaled_products(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015)), 1.0, s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005)), 1.0));
            s.store_ad_value(2009, A::add_scaled_square_product(s.ad_value(2007), 1.0, s.ad_value(2006), s.ad_value(2008), (-4.0)));
            s.store_ad_value(2012, A::div_scaled_inputs(s.ad_value(2008), 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0));
            s.store_neg_ad(2016, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if ((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_offset_ad_lhs(2010, A::mul_scaled_lhs(s.ad_value(1940), 1.25, s.ad_value(1998)), (-1.0), 1998);
            s.store_ad_value(2011, A::mul_offset_rhs(A::div(s.ad_value(2027), s.ad_value(1940)), A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.b[2949] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.v[2949] = if s.b[2949] { 1.0 } else { 0.0 };

        if (((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && s.b[2949]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[2950] = ((-s.v[2011]) < 0.0);
        s.v[2950] = if s.b[2950] { 1.0 } else { 0.0 };

        if ((((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && (!s.b[2949])) && s.b[2950]) {
            s.store_div_from_scalar_offset_ad(2009, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(2011)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(2011)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && (!s.b[2949])) && (!s.b[2950])) {
            s.store_scaled_offset_ad(2009, A::mul_offset_lhs(A::neg(s.ad_value(2011)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(2011)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(2011)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_ad_value(2013, A::add_scaled_inputs_product(s.ad_value(2027), 1.0, s.ad_value(1939), 0.5, s.ad_value(1938), A::sqrt(A::add_scaled_inputs3(s.ad_value(2027), 1.0, s.ad_value(1939), 0.25, s.ad_value(2012), -1.0)), (-1.0)));
        }

        s.b[2951] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[2951] = if s.b[2951] { 1.0 } else { 0.0 };

        if (((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && s.b[2951]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[2952] = ((-s.v[2013]) < 0.0);
        s.v[2952] = if s.b[2952] { 1.0 } else { 0.0 };

        if ((((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && (!s.b[2951])) && s.b[2952]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(2013)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(2013)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && (!s.b[2951])) && (!s.b[2952])) {
            s.store_scaled_offset_ad(2005, A::mul_offset_lhs(A::neg(s.ad_value(2013)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(2013)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(2013)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul_scaled_lhs(s.ad_value(1939), 0.5, s.ad_value(2005)));
            s.store_ad_value(2007, A::add_scaled_inputs3(s.ad_value(2027), 2.0, s.ad_value(2013), (-2.0), A::mul_sub_from_scalar_rhs(s.ad_value(1939), 1.0, s.ad_value(2005)), 1.0));
            s.store_ad_value(2008, A::add_scaled_products(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013)), 1.0, s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005)), (-1.0)));
            s.store_ad_value(2009, A::add_scaled_square_product(s.ad_value(2007), 1.0, s.ad_value(2006), s.ad_value(2008), (-4.0)));
            s.store_ad_value(2014, A::div_scaled_inputs(s.ad_value(2008), 2.0, A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))), 1.0));
            s.store_add(2016, 2013, 2014);
        }

        s.b[2953] = (((s.v[2016]) as f64).abs() <= s.v[1933]);
        s.v[2953] = if s.b[2953] { 1.0 } else { 0.0 };

        if (s.b[2940] && s.b[2953]) {
            s.store_mul_ad_affine_product_rhs(1996, 2016, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);
            s.store_ad_value(1991, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1889), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2016), 1.0, A::scale(s.ad_value(2016), 0.25), 0.3333333333333333), (-0.70710678)));
            s.store_ad_value(1990, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1889), 1.0, A::scale(s.ad_value(2016), 0.5), (-0.235702)));
        }

        s.b[2954] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);
        s.v[2954] = if s.b[2954] { 1.0 } else { 0.0 };

        if ((s.b[2940] && (!s.b[2953])) && s.b[2954]) {
            s.store_exp_neg_input(2027, 2016);
        }

        s.b[2955] = ((-s.v[2016]) < 0.0);
        s.v[2955] = if s.b[2955] { 1.0 } else { 0.0 };

        if (((s.b[2940] && (!s.b[2953])) && (!s.b[2954])) && s.b[2955]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(2016)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(2016)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[2940] && (!s.b[2953])) && (!s.b[2954])) && (!s.b[2955])) {
            s.store_scaled_offset_ad(2027, A::mul_offset_lhs(A::neg(s.ad_value(2016)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(2016)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(2016)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (s.b[2940] && (!s.b[2953])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));
        }

        s.b[2956] = (s.v[2016] > s.v[1933]);
        s.v[2956] = if s.b[2956] { 1.0 } else { 0.0 };

        if ((s.b[2940] && (!s.b[2953])) && s.b[2956]) {
            s.store_neg(1996, 1996);
        }

        if (s.b[2940] && (!s.b[2953])) {
            s.store_ad_value(1991, A::div_scaled_product3(s.ad_value(1889), s.ad_value(1889), A::sub_from_scalar(1.0, s.ad_value(2027)), 0.5, s.ad_value(1996), 1.0));
            s.store_add_ad_lhs(1990, A::div(A::add_scaled_square_product(s.ad_value(1991), 1.0, s.ad_value(1889), s.ad_value(1889), (-0.5)), s.ad_value(1996)), 1991);
        }

        if s.b[2940] {
            s.store_sub(1988, 1937, 1991);
            s.store_div_from_scalar(1989, 1.0, 1988);
            s.store_offset_mul(1987, 1970, 1989, (-1.0));
            s.store_mul_sub_from_scalar_ad_lhs(1986, 1.0, A::mul(A::mul3(s.ad_value(1970), s.ad_value(1990), s.ad_value(1989)), s.ad_value(1989)), 1989);
            s.store_ad_value(2017, A::add_scaled_product(A::mul3(s.ad_value(1986), s.ad_value(1992), s.ad_value(1992)), 1.0, s.ad_value(1987), s.ad_value(1993), 1.0));
            s.store_mul_ad_product_lhs(1994, A::square(s.ad_value(1992)), s.ad_value(1989), 1989);
            s.store_mul(1985, 2018, 1994);
        }

        s.b[2957] = (s.v[0] == (-1.0));
        s.v[2957] = if s.b[2957] { 1.0 } else { 0.0 };

        if (s.b[2940] && s.b[2957]) {
            s.store_div_ad_rhs(1985, 1985, A::offset(A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0));
        }

        if s.b[2940] {
            s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);
            s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);
            s.store_ad_value(2028, A::add_scaled_product(s.ad_value(1993), 1.0, s.ad_value(1994), s.ad_value(1990), (-1.0)));
            s.store_mul_sub_ad_rhs(1952, 2019, s.ad_value(2017), A::div_scaled_product(A::mul3(s.ad_value(1985), s.ad_value(1987), s.ad_value(2028)), s.ad_value(2019), 1.0, s.ad_value(2027), 1.0));
        }

        s.b[2958] = (s.v[1] >= 3.0);
        s.v[2958] = if s.b[2958] { 1.0 } else { 0.0 };

        s.b[2959] = (s.v[1] == 3.0);
        s.v[2959] = if s.b[2959] { 1.0 } else { 0.0 };

        if (s.b[2958] && s.b[2959]) {
            s.store_ad_value(1992, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(1979), 13.0, s.ad_value(1971), 6.0, s.ad_value(1970), (-24.0)), 0.14285714285714285, s.ad_value(1969), (6.0 * 0.14285714285714285), s.ad_value(1978), (-0.14285714285714285)));
            s.store_ad_value(1993, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(1979), 180.0, s.ad_value(1971), (-408.0), s.ad_value(1970), 288.0), 0.14285714285714285, s.ad_value(1969), ((-72.0) * 0.14285714285714285), s.ad_value(1978), (12.0 * 0.14285714285714285)));
        }

        s.b[2960] = (s.v[1] == 5.0);
        s.v[2960] = if s.b[2960] { 1.0 } else { 0.0 };

        if ((s.b[2958] && (!s.b[2959])) && s.b[2960]) {
            s.store_scaled_sub_ad_lhs(1992, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(1979), 1.0, s.ad_value(1973), (-6.0), s.ad_value(1972), 24.0), 1.0, s.ad_value(1970), (-24.0), s.ad_value(1969), 6.0), 1978, 0.2);
            s.store_scaled_add_ad(1993, A::sub_scaled_inputs(A::sub(A::add_scaled_inputs(s.ad_value(1972), 1296.0, s.ad_value(1970), 1296.0), A::add_scaled_inputs(s.ad_value(1973), 324.0, s.ad_value(1969), 324.0)), 1.0, s.ad_value(1971), 2052.0), A::add_scaled_inputs(s.ad_value(1979), 54.0, s.ad_value(1978), 54.0), 0.07692307692307693);
        }

        s.b[2961] = (s.v[1] == 9.0);
        s.v[2961] = if s.b[2961] { 1.0 } else { 0.0 };

        if (((s.b[2958] && (!s.b[2959])) && (!s.b[2960])) && s.b[2961]) {
            let assign67890_ad_e90960: A = A::sub_scaled_inputs(A::add_scaled_inputs3(A::add_scaled_inputs3(A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(1974), 21840.0, s.ad_value(1972), 304200.0, s.ad_value(1979), 65.0), 1.0, s.ad_value(1971), (-420.0), s.ad_value(1976), 1560.0), 1.0, s.ad_value(1978), (-12605.0), s.ad_value(1977), (-390.0)), 1.0, s.ad_value(1969), 75630.0, s.ad_value(1975), (-5850.0)), 2.6434745829918846e-5, s.ad_value(1970), (302520.0 * 2.6434745829918846e-5));
            s.store_sub_scaled_ad_lhs(1992, assign67890_ad_e90960, 1973, (390.0 / 181.0));
        }

        if (((s.b[2958] && (!s.b[2959])) && (!s.b[2960])) && s.b[2961]) {
            let assign67900_ad_e91017: A = A::add_scaled_inputs(A::add_scaled_inputs3(A::add_scaled_inputs3(A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(1969), (-2619900.0), s.ad_value(1975), (-202500.0), s.ad_value(1972), 10530000.0), 1.0, s.ad_value(1971), (-16601100.0), s.ad_value(1970), 10479600.0), 1.0, s.ad_value(1978), 436650.0, s.ad_value(1979), 2250.0), 1.0, s.ad_value(1976), 54000.0, s.ad_value(1977), (-13500.0)), 1.0, s.ad_value(1974), 756000.0);
            s.store_ad_value(1993, A::sub_scaled_inputs(assign67900_ad_e91017, 2.6434745829918846e-5, s.ad_value(1973), (13500.0 * 0.0055248618784530384)));
        }

        if (((s.b[2958] && (!s.b[2959])) && (!s.b[2960])) && (!s.b[2961])) {
            s.store_scalar(1992, 0.0);
            s.store_scalar(1993, 0.0);
        }

        if s.b[2958] {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1971), s.ad_value(1937)), 1890);
        }

        s.b[2962] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[2962] = if s.b[2962] { 1.0 } else { 0.0 };

        if (s.b[2958] && s.b[2962]) {
            s.store_div(2016, 2027, 1940);
        }

        s.b[2963] = (s.v[2027] < (-s.v[1941]));
        s.v[2963] = if s.b[2963] { 1.0 } else { 0.0 };

        if ((s.b[2958] && (!s.b[2962])) && s.b[2963]) {
            s.store_neg(1999, 2027);
            s.store_scaled_div(2000, 1999, 1940, 1.25);
            s.store_scaled_sub_ad(2001, A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(2000), (-6.0), A::offset(s.ad_value(2000), (-6.0))), 64.0)), 0.5);
            s.store_ad_value(2002, A::add_scaled_products(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001)), 1.0, s.ad_value(1939), A::offset(s.ad_value(2001), 1.0), 1.0));
            s.store_ad_value(2003, A::add_scaled_inputs3(s.ad_value(1999), 2.0, s.ad_value(2001), (-2.0), s.ad_value(1939), -1.0));
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
            s.store_add(824, 2002, 2003);
            s.store_ad_value(823, A::add_scaled_square_product(s.ad_value(824), 1.0, s.ad_value(2004), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0));
            s.store_add_ad_rhs(2015, 2001, A::div_scaled_product3(s.ad_value(2002), s.ad_value(824), s.ad_value(2004), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004), s.ad_value(2004)), s.ad_value(2003), A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.3333333333333333, s.ad_value(2002), 1.0))), 1.0));
        }

        s.b[2964] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[2964] = if s.b[2964] { 1.0 } else { 0.0 };

        if (((s.b[2958] && (!s.b[2962])) && s.b[2963]) && s.b[2964]) {
            s.store_exp(2005, 2015);
        }

        s.b[2965] = (s.v[2015] < 0.0);
        s.v[2965] = if s.b[2965] { 1.0 } else { 0.0 };

        if ((((s.b[2958] && (!s.b[2962])) && s.b[2963]) && (!s.b[2964])) && s.b[2965]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2015), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2015), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2958] && (!s.b[2962])) && s.b[2963]) && (!s.b[2964])) && (!s.b[2965])) {
            s.store_scaled_offset_ad(2005, A::mul_offset_lhs(s.ad_value(2015), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2015), (-230.25850929940458), A::scale_offset(s.ad_value(2015), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2958] && (!s.b[2962])) && s.b[2963]) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul_scaled_output(s.ad_value(1939), s.ad_value(2005), 0.5));
            s.store_ad_value(2007, A::add_scaled_inputs_product(s.ad_value(1999), 2.0, s.ad_value(2015), (-2.0), s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0)), 1.0));
        }

    }
}

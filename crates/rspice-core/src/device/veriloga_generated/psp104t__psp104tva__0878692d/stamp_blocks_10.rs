#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_10(
        s: &mut ReactiveScratch,
    ) {
        if (((!s.b[2179]) && (!s.b[2180])) && s.b[2182]) {
            s.store_exp_neg_input(2144, 2163);
        }

        if (((!s.b[2179]) && (!s.b[2180])) && (!s.b[2182])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2144, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2163)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((!s.b[2179]) && (!s.b[2180])) {
            s.store_sub_from_scalar(2164, 1.0, 2144);
            s.store_add_scaled_inputs_product_right_ad(2165, 2039, 1.0, 2021, 0.5, 2020, A::sqrt(A::add_scaled_inputs3(s.ad_value(2039), 1.0, s.ad_value(2021), 0.25, s.ad_value(2164), -1.0)), (-1.0));
            s.store_offset(2166, 2045, 3.0);
            s.store_sub_ad(2149, A::add_scaled_inputs3(s.ad_value(2165), 0.5, s.ad_value(2166), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2165), s.ad_value(2166)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2166), 0.5, A::sqrt_square_offset(s.ad_value(2166), 5.0), 0.5));
            s.store_sub(2144, 2039, 2149);
            s.store_exp_neg_input(2145, 2149);
            s.store_div_from_scalar_offset_square(2146, 1.0, 2149, 2.0);
            s.store_mul_square_lhs(2156, 2149, 2146);
            s.store_mul3_affine_lhs(2157, 2149, 2146, 4.0, 0.0, 2146);
            s.store_mul_ad_product_lhs_mixed_ai(2158, A::sub_scaled_inputs(s.ad_value(2146), 8.0, s.ad_value(2156), 12.0), 2146, 2146);
        }

        if ((!s.b[2179]) && (!s.b[2180])) {
            if (1e-40 > ((s.v[2144] * s.v[2144]) - (s.v[2021] * (((s.v[2145] + s.v[2149]) - 1.0) - (s.v[2060] * ((s.v[2149] + 1.0) + s.v[2156])))))) {
                s.store_scalar(2150, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2150, 2144, 1.0, 2021, A::add_scaled_product(A::offset(A::add(s.ad_value(2145), s.ad_value(2149)), (-1.0)), 1.0, s.ad_value(2060), A::add(A::offset(s.ad_value(2149), 1.0), s.ad_value(2156)), (-1.0)), (-1.0));
            }
        }

        if ((!s.b[2179]) && (!s.b[2180])) {
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2167, 1.0, 2021, A::add_scaled_product(s.ad_value(2145), 1.0, s.ad_value(2060), s.ad_value(2158), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(2151, 2144, 2.0, 2021, A::add_scaled_sub_value_product(1.0, s.ad_value(2145), 1.0, s.ad_value(2060), A::offset(s.ad_value(2157), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(2152, 2045, 1.0, 2149, (-1.0), A::ln(A::div(s.ad_value(2150), s.ad_value(2021))), 1.0);
            s.store_add(813, 2150, 2151);
            s.store_add_scaled_square_product_mixed_iia(812, 813, 1.0, 2152, A::add_scaled_square_product(s.ad_value(2151), 0.5, s.ad_value(2150), s.ad_value(2167), (-1.0)), 1.0);
            s.store_add_ad_rhs(2168, 2149, A::div_scaled_product3(s.ad_value(2150), s.ad_value(813), s.ad_value(2152), 1.0, A::add(s.ad_value(812), A::mul3(A::mul3(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2152), s.ad_value(2152)), s.ad_value(2151), A::add_scaled_square_product(s.ad_value(2151), 0.3333333333333333, s.ad_value(2150), s.ad_value(2167), (-1.0)))), 1.0));
        }

        s.b[2183] = (s.v[2168] < 230.25850929940458);
        s.store_scalar(2183, if s.b[2183] { 1.0 } else { 0.0 });

        if (((!s.b[2179]) && (!s.b[2180])) && s.b[2183]) {
            s.store_exp(2154, 2168);
            s.store_div_from_scalar(2155, 1.0, 2154);
            s.store_mul(2154, 2060, 2154);
        }

        s.b[2184] = (s.v[2168] > (s.v[2045] - 230.25850929940458));
        s.store_scalar(2184, if s.b[2184] { 1.0 } else { 0.0 });

        if ((((!s.b[2179]) && (!s.b[2180])) && (!s.b[2183])) && s.b[2184]) {
            s.store_exp_sub(2154, 2168, 2045);
            s.store_div(2155, 2060, 2154);
        }

        if ((((!s.b[2179]) && (!s.b[2180])) && (!s.b[2183])) && (!s.b[2184])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2154, 1e-100, A::sub(s.ad_value(2045), s.ad_value(2168)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2155, 1e-100, 2168, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if ((!s.b[2179]) && (!s.b[2180])) {
            s.store_div_from_scalar_offset_square(2144, 1.0, 2168, 2.0);
            s.store_mul_square_lhs(2156, 2168, 2144);
            s.store_mul3_affine_lhs(2157, 2168, 2144, 4.0, 0.0, 2144);
            s.store_mul_ad_product_lhs_mixed_ai(2158, A::sub_scaled_inputs(s.ad_value(2144), 8.0, s.ad_value(2156), 12.0), 2144, 2144);
            s.store_sub(2144, 2039, 2168);
            s.store_add_scaled_product_right_ad(2159, 2144, 2.0, 2021, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2155)), 1.0, s.ad_value(2154), 1.0, s.ad_value(2060), A::offset(s.ad_value(2157), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2160, 2144, 1.0, 2021, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2155), 1.0, s.ad_value(2168), 1.0, s.ad_value(2154), 1.0, (-1.0)), 1.0, s.ad_value(2060), A::add(A::offset(s.ad_value(2168), 1.0), s.ad_value(2156)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2144, 2.0, 2021, A::add_scaled_inputs_product(s.ad_value(2155), 1.0, s.ad_value(2154), 1.0, s.ad_value(2060), s.ad_value(2158), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(2144, 2159, 1.0, 2160, 2144, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(2059, 2168, 1.0, A::div(s.ad_value(2160), A::add(s.ad_value(2159), A::sqrt(s.ad_value(2144)))), 2.0);
        }

        s.store_scalar(2062, 0.0);

        s.store_scalar(2063, 0.0);

        s.store_scalar(2064, 0.0);

        s.store_scalar(2065, 0.0);

        s.store_scalar(2066, 0.0);

        s.store_scalar(2067, 0.0);

        s.store_scalar(2068, 0.0);

        s.store_scalar(2069, 1.0);

        s.store_scalar(2070, 1.0);

        s.store_sub(2071, 2039, 2059);

        s.store_scalar(2072, 0.0);

        s.store_mul(2073, 2035, 2071);

        s.store_scalar(2074, 1.0);

        s.store_scalar(2075, 1.0);

        s.store_scalar(2079, 1.0);

        s.store_scalar(2080, 1.0);

        s.store_scalar(2082, 1.0);

        s.b[2185] = (s.v[2039] > 0.0);
        s.store_scalar(2185, if s.b[2185] { 1.0 } else { 0.0 });

        if s.b[2185] {
            s.store_div_from_scalar_offset_square(1929, 1.0, 2059, 2.0);
            s.store_mul_square_lhs(2061, 2059, 1929);
            s.store_mul3_affine_lhs(2062, 2059, 1929, 4.0, 0.0, 1929);
            s.store_mul_ad_product_lhs_mixed_ai(2063, A::sub_scaled_inputs(s.ad_value(1929), 8.0, s.ad_value(2061), 12.0), 1929, 1929);
            s.store_scalar(2064, 0.0);
        }

        s.b[2186] = (s.v[2059] < 230.25850929940458);
        s.store_scalar(2186, if s.b[2186] { 1.0 } else { 0.0 });

        if (s.b[2185] && s.b[2186]) {
            s.store_exp(2064, 2059);
            s.store_div_from_scalar(2065, 1.0, 2064);
            s.store_mul(2064, 2060, 2064);
        }

        s.b[2187] = (s.v[2059] > (s.v[2045] - 230.25850929940458));
        s.store_scalar(2187, if s.b[2187] { 1.0 } else { 0.0 });

        if ((s.b[2185] && (!s.b[2186])) && s.b[2187]) {
            s.store_exp_sub(2064, 2059, 2045);
            s.store_div(2065, 2060, 2064);
        }

        if ((s.b[2185] && (!s.b[2186])) && (!s.b[2187])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2064, 1e-100, A::sub(s.ad_value(2045), s.ad_value(2059)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2065, 1e-100, 2059, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if s.b[2185] {
            s.store_add_scaled_product_right_ad(2066, 2064, 1.0, 2060, A::add(A::offset(s.ad_value(2059), 1.0), s.ad_value(2061)), (-1.0));
        }

        s.b[2188] = (s.v[2059] < 1e-5);
        s.store_scalar(2188, if s.b[2188] { 1.0 } else { 0.0 });

        if (s.b[2185] && s.b[2188]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2067, 2059, 1.0, 2059, 1.0, 2059, 0.25, 0.3333333333333333, 0.5);
            s.store_mul3_ad_middle_scaled_output(2066, A::mul3(s.ad_value(2060), s.ad_value(2059), s.ad_value(2059)), 2059, A::scale_offset(s.ad_value(2059), 1.75, 1.0), 0.16666666666666666);
            s.store_sqrt_sub_from_scalar_ad(1929, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2059), 1.0, A::scale(s.ad_value(2059), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2068, 2059, 1929, 0.7071067811865475);
            s.store_offset_div_scaled_product(2069, s.ad_value(2020), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2059), 0.5)), 1.0, A::square(s.ad_value(2059)), 0.16666666666666666), 0.7071067811865475, s.ad_value(1929), 1.0, 1.0);
        }

        if (s.b[2185] && (!s.b[2188])) {
            s.store_add_offset_lhs(2067, 2059, (-1.0), 2065);
            s.store_sqrt(2068, 2067);
            s.store_offset_scaled_ad(2069, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2020), 1.0, s.ad_value(2065)), s.ad_value(2068)), 0.5, 1.0);
        }

        if s.b[2185] {
            s.store_div_scaled_offset_numerator(2070, A::mul_scaled_lhs(s.ad_value(709), 0.2, s.ad_value(2019)), 1.0, 1.0, A::offset(A::mul(s.ad_value(709), s.ad_value(2019)), 1.0), 1.0);
        }

        s.b[2189] = (s.v[2066] > 1e-100);
        s.store_scalar(2189, if s.b[2189] { 1.0 } else { 0.0 });

        if (s.b[2185] && s.b[2189]) {
            s.store_mul_sqrt_ad_rhs(2071, 2020, A::add(s.ad_value(2067), s.ad_value(2066)));
            s.store_div_scaled_product3_mixed_iiia(2072, 2021, 2066, 2035, 1.0, A::add_scaled_product(s.ad_value(2071), 1.0, s.ad_value(2020), s.ad_value(2068), 1.0), 1.0);
            s.store_mul3_lhs(2073, 2068, 2020, 2035);
        }

        s.b[2190] = (s.v[215] < 0.0);
        s.store_scalar(2190, if s.b[2190] { 1.0 } else { 0.0 });

        if ((s.b[2185] && s.b[2189]) && s.b[2190]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2074, 1.0, 1.0, A::mul(s.ad_value(215), s.ad_value(2019)));
        }

        if ((s.b[2185] && s.b[2189]) && (!s.b[2190])) {
            s.store_offset_mul(2074, 215, 2019, 1.0);
        }

        s.b[2191] = (s.v[216] < 0.0);
        s.store_scalar(2191, if s.b[2191] { 1.0 } else { 0.0 });

        if ((s.b[2185] && s.b[2189]) && s.b[2191]) {
            s.store_sub_from_scalar_scaled_mul(2075, 1.0, 216, 2072, 1.0);
        }

        if ((s.b[2185] && s.b[2189]) && (!s.b[2191])) {
            s.store_div_from_scalar_offset_product(2075, 1.0, 216, 2072, 1.0);
        }

        if (s.b[2185] && s.b[2189]) {
            s.store_mul_product3_indices(2076, 2072, 746, 2074, 2075, 1.0);
            s.store_mul_add_scaled_product_rhs(2077, 763, s.ad_value(2073), 1.0, s.ad_value(764), s.ad_value(2072), 1.0);
            s.store_ln_ad(1930, A::div_scaled_value_offset_denominator(s.ad_value(2067), 1.0, A::add(s.ad_value(2067), s.ad_value(2066)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2078, A::pow(A::mul(s.ad_value(2077), s.ad_value(705)), s.ad_value(706)), 1.0, 707, A::exp(A::mul_scaled_lhs(s.ad_value(708), 0.5, s.ad_value(1930))), 1.0);
            s.store_mul_add_ad_lhs(2079, A::offset(s.ad_value(2078), 1.0), s.ad_value(2076), 2070);
        }

        s.b[2192] = (s.v[219] < 0.0);
        s.store_scalar(2192, if s.b[2192] { 1.0 } else { 0.0 });

        if ((s.b[2185] && s.b[2189]) && s.b[2192]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2080, 1.0, 1.0, A::mul(s.ad_value(219), s.ad_value(2019)));
        }

        if ((s.b[2185] && s.b[2189]) && (!s.b[2192])) {
            s.store_offset_mul(2080, 219, 2019, 1.0);
        }

        if (s.b[2185] && s.b[2189]) {
            s.store_mul(1931, 2072, 2080);
            s.store_div_add_scaled_inputs_rhs_indices(2081, 1931, 221, 1.0, 1931, 1.0);
        }

        s.b[2193] = (s.v[220] < 0.0);
        s.store_scalar(2193, if s.b[2193] { 1.0 } else { 0.0 });

        if ((s.b[2185] && s.b[2189]) && s.b[2193]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2082, 1.0, 1.0, A::mul(s.ad_value(220), s.ad_value(2081)));
        }

        if ((s.b[2185] && s.b[2189]) && (!s.b[2193])) {
            s.store_offset_mul(2082, 220, 2081, 1.0);
        }

        s.copy_ad(1806, 2017);

        s.copy_ad(1807, 2019);

        s.copy_ad(1808, 2035);

        s.copy_ad(1809, 2036);

        s.copy_ad(1810, 2020);

        s.copy_ad(1811, 2021);

        s.copy_ad(1812, 2037);

        s.copy_ad(1813, 2039);

        s.copy_ad(1814, 2044);

        s.copy_ad(1815, 2045);

        s.copy_ad(1816, 2056);

        s.copy_ad(1817, 2057);

        s.copy_ad(1818, 2058);

        s.copy_ad(1819, 2165);

        s.copy_ad(1820, 2060);

        s.copy_ad(1821, 2059);

        s.copy_ad(1822, 2062);

        s.copy_ad(1823, 2063);

        s.copy_ad(1824, 2064);

        s.copy_ad(1825, 2065);

        s.copy_ad(1826, 2067);

        s.copy_ad(1827, 2066);

        s.copy_ad(1828, 2068);

        s.copy_ad(1829, 2069);

        s.copy_ad(1830, 2070);

        s.copy_ad(1831, 2071);

        s.copy_ad(1832, 2072);

        s.copy_ad(1833, 2073);

        s.copy_ad(1834, 2074);

        s.copy_ad(1835, 2075);

        s.copy_ad(1836, 2079);

        s.copy_ad(1837, 2080);

        s.copy_ad(1838, 2082);

        s.store_scalar(2084, 0.0);

        s.store_scale(2083, 2035, 4.60517018598809);

        s.copy_ad(2100, 2083);

        s.copy_ad(2101, 815);

        s.store_mul(2102, 815, 2036);

        s.copy_ad(2106, 2059);

        s.store_scalar(2107, 0.0);

        s.store_scalar(2110, 0.0);

        s.copy_ad(2112, 2065);

        s.copy_ad(2113, 2067);

        s.copy_ad(2115, 2066);

        s.copy_ad(2116, 2073);

        s.copy_ad(2117, 2059);

        s.copy_ad(2118, 2065);

        s.copy_ad(2120, 2066);

        s.copy_ad(2121, 2067);

        s.store_sub(2122, 2039, 2059);

        s.store_scalar(2123, 1.0);

        s.store_scalar(2125, 1.0);

        s.store_scalar(2124, 0.0);

        s.copy_ad(2134, 2072);

        s.store_mul(2138, 2122, 2035);

        s.store_scalar(2135, 0.0);

        s.copy_ad(2136, 2073);

        s.store_scalar(2141, 0.0);

        s.store_scalar(2140, 1.0);

        s.copy_ad(2143, 2015);

        s.copy_ad(2142, 2138);

        s.b[2194] = (s.v[2039] > 0.0);
        s.store_scalar(2194, if s.b[2194] { 1.0 } else { 0.0 });

        s.b[2195] = (s.v[2066] > 1e-100);
        s.store_scalar(2195, if s.b[2195] { 1.0 } else { 0.0 });

        if (s.b[2194] && s.b[2195]) {
            s.store_mul(2143, 2015, 2082);
        }

    }

    pub(super) fn stamp_reactive_block_11(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[2194] && s.b[2195]) {
            s.store_div(2084, 2143, 2079);
            s.store_add_scaled_inputs(2085, 2071, 1.0, 2021, 0.5);
            s.store_div_scaled_product_by_product(1929, s.ad_value(2021), s.ad_value(2064), 1.0, s.ad_value(2085), s.ad_value(2085), 1.0);
        }

        s.b[2196] = (s.v[1929] > 0.0001);
        s.store_scalar(2196, if s.b[2196] { 1.0 } else { 0.0 });

        if ((s.b[2194] && s.b[2195]) && s.b[2196]) {
            s.store_sub_from_scalar(1930, 1.0, 1929);
        }

        s.b[2197] = (s.v[1930] < 1e-10);
        s.store_scalar(2197, if s.b[2197] { 1.0 } else { 0.0 });

        if (((s.b[2194] && s.b[2195]) && s.b[2196]) && s.b[2197]) {
            s.store_scalar(1931, 1.0);
        }

        if (((s.b[2194] && s.b[2195]) && s.b[2196]) && (!s.b[2197])) {
            s.store_sub_from_scalar_ad(1931, 1.0, A::sqrt(s.ad_value(1930)));
        }

        if ((s.b[2194] && s.b[2195]) && (!s.b[2196])) {
            s.store_scale(1931, 1929, 0.5);
        }

        if (s.b[2194] && s.b[2195]) {
            s.store_mul(2086, 1931, 2085);
        }

        s.b[2198] = ((s.v[707] > 0.0) && (s.v[708] > 0.0));
        s.store_scalar(2198, if s.b[2198] { 1.0 } else { 0.0 });

        if ((s.b[2194] && s.b[2195]) && s.b[2198]) {
            s.store_scaled_mul(2087, 2035, 2086, 0.475);
            s.store_add_scaled_product_indices(1929, 2072, 1.0, 2069, 2087, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(2088, 1929, 1929, 1e-12, 0.5);
            s.store_add_scaled_value_products(2089, s.ad_value(2072), (-1.0), s.ad_value(2035), s.ad_value(2071), 1.0, A::offset(s.ad_value(2069), (-1.0)), s.ad_value(2087), 1.0);
            s.store_offset_div_scaled_product(2090, s.ad_value(2021), s.ad_value(2035), 0.5, s.ad_value(2089), 1.0, 1.0);
            s.store_add_scaled_product_indices(1929, 2089, 1.0, 764, 2088, 1.0);
            s.store_pow_ad(2091, A::mul3(s.ad_value(763), s.ad_value(1929), s.ad_value(705)), s.ad_value(706));
            s.store_mul_ad_lhs(1930, A::div_scaled_product_offset_rhs(s.ad_value(706), A::mul_sub_from_scalar_rhs(s.ad_value(2090), 1.0, s.ad_value(764)), (-1.0), 1.0, s.ad_value(1929), 1.0), 2091);
            s.store_div(1929, 2088, 2089);
            s.store_mul_pow_ad_rhs(2092, 707, A::offset(s.ad_value(1929), 1.0), A::neg(s.ad_value(708)));
            s.store_mul_div_scaled_product_mixed_iiai(1931, 2092, 708, A::add(A::offset(s.ad_value(2090), (-1.0)), A::div_scalar_offset_denominator(1.0, s.ad_value(1929), 1.0, 1.0)), 1.0, 2089, 1.0);
            s.store_mul_product3_indices(2093, 2088, 746, 2074, 2075, 1.0);
            s.store_offset_ad(1929, A::div_scaled_add_product(s.ad_value(1930), 1.0, A::mul3(s.ad_value(746), s.ad_value(2074), s.ad_value(2075)), s.ad_value(2090), (-1.0), s.ad_value(1931), 1.0), 1.0);
        }

        s.b[2199] = (s.v[1929] < 230.25850929940458);
        s.store_scalar(2199, if s.b[2199] { 1.0 } else { 0.0 });

        if (((s.b[2194] && s.b[2195]) && s.b[2198]) && s.b[2199]) {
            s.store_scaled_ln_one_plus_exp_scaled_input(1930, 1929, 2.0, 0.5);
        }

        if (((s.b[2194] && s.b[2195]) && s.b[2198]) && (!s.b[2199])) {
            s.copy_ad(1930, 1929);
        }

        if ((s.b[2194] && s.b[2195]) && s.b[2198]) {
            s.store_div_scaled_product3_mixed_iiia(2094, 2087, 1931, 1930, -1.0, A::add_scaled_inputs3_offset(s.ad_value(2091), 1.0, s.ad_value(2092), 1.0, s.ad_value(2093), 1.0, 1.0), 1.0);
            s.store_mul_offset_ad_rhs(2095, 2086, A::div_scaled_value_offset_denominator(s.ad_value(2094), 1.0, A::sqrt_square_offset(s.ad_value(2094), 1.0), 1.0, 1.0), 1.0);
        }

        if ((s.b[2194] && s.b[2195]) && (!s.b[2198])) {
            s.copy_ad(2095, 2086);
        }

        if (s.b[2194] && s.b[2195]) {
            s.store_mul3_affine_lhs(2096, 2035, 2084, 0.7071067811865475, 0.0, 2095);
        }

        s.b[2200] = (s.v[0] == (-1.0));
        s.store_scalar(2200, if s.b[2200] { 1.0 } else { 0.0 });

        if ((s.b[2194] && s.b[2195]) && s.b[2200]) {
            s.store_div_ad_rhs(2096, 2096, A::sqrt(A::offset(s.ad_value(2096), 1.0)));
        }

        if (s.b[2194] && s.b[2195]) {
            s.store_div_from_scalar_offset_ad(2097, 2.0, A::sqrt(A::scale_offset(s.ad_value(2096), 4.0, 1.0)), 1.0);
            s.store_mul(1929, 2097, 2096);
            s.store_mul_ad_product_rhs_mixed_ia(2098, 2095, 2097, A::offset(A::div(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1929), 1.0, A::mul(s.ad_value(1929), s.ad_value(2097)), 0.86), A::offset(A::mul3_scaled_output(s.ad_value(1929), s.ad_value(1929), s.ad_value(2097), 4.0), 1.0)), 1.0));
            s.store_scale(2099, 2098, 0.99);
            s.store_div_scaled_product3_mixed_iaii(1929, 2099, A::sub_scaled_inputs(s.ad_value(2099), 1.0, s.ad_value(2085), 2.0), 2037, 1.0, 2066, 1.0);
        }

        if (s.b[2194] && s.b[2195]) {
            s.store_mul_sub_ad_rhs(2100, 2035, s.ad_value(2099), A::ln(A::offset({
                if (s.v[1929] > (-0.99)) {
                    s.ad_value(1929)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0)));
        }

        if (s.b[2194] && (!s.b[2195])) {
            s.copy_ad(2100, 2083);
        }

        if s.b[2194] {
            s.store_offset(1929, 2016, 1.0);
            s.store_div_scaled_product_left_ad(1930, A::sqrt(s.ad_value(1929)), 815, 1.0, 2100, 1.0);
            s.store_add_ad_lhs(1931, A::square(s.ad_value(1930)), 1929);
            s.store_scale(1929, 1930, 2.0);
            s.store_div_scaled_product_add_scaled_denominator(2101, 2100, 1929, 1.0, A::sqrt(A::sub(s.ad_value(1931), s.ad_value(1929))), 1.0, A::sqrt(A::add(s.ad_value(1931), s.ad_value(1929))), 1.0, 1.0);
            s.store_mul(2102, 2101, 2036);
            s.store_add(2103, 2045, 2102);
        }

        s.b[2201] = (s.v[2102] < 460.51701859880916);
        s.store_scalar(2201, if s.b[2201] { 1.0 } else { 0.0 });

        if (s.b[2194] && s.b[2201]) {
            s.store_exp_neg_input(2104, 2102);
        }

        if (s.b[2194] && (!s.b[2201])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2104, 1e-200, 2102, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if s.b[2194] {
            s.store_mul(2105, 2060, 2104);
        }

        s.b[2202] = (((s.v[2039]) as f64).abs() <= s.v[2057]);
        s.store_scalar(2202, if s.b[2202] { 1.0 } else { 0.0 });

        if (s.b[2194] && s.b[2202]) {
            s.store_scaled_square(2145, 2058, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(2106, 2039, 2058, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2039), 1.0, s.ad_value(2105)), s.ad_value(2020), s.ad_value(2145)), 1.0));
        }

        if (s.b[2194] && (!s.b[2202])) {
            s.store_offset(2166, 2103, 3.0);
            s.store_sub_ad(2149, A::add_scaled_inputs3(s.ad_value(2165), 0.5, s.ad_value(2166), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2165), s.ad_value(2166)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2166), 0.5, A::sqrt_square_offset(s.ad_value(2166), 5.0), 0.5));
            s.store_sub(2144, 2039, 2149);
            s.store_exp_neg_input(2145, 2149);
            s.store_div_from_scalar_offset_square(2146, 1.0, 2149, 2.0);
            s.store_mul_square_lhs(2156, 2149, 2146);
            s.store_mul3_affine_lhs(2157, 2149, 2146, 4.0, 0.0, 2146);
            s.store_mul_ad_product_lhs_mixed_ai(2158, A::sub_scaled_inputs(s.ad_value(2146), 8.0, s.ad_value(2156), 12.0), 2146, 2146);
        }

        if (s.b[2194] && (!s.b[2202])) {
            if (1e-40 > ((s.v[2144] * s.v[2144]) - (s.v[2021] * (((s.v[2145] + s.v[2149]) - 1.0) - (s.v[2105] * ((s.v[2149] + 1.0) + s.v[2156])))))) {
                s.store_scalar(2150, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2150, 2144, 1.0, 2021, A::add_scaled_product(A::offset(A::add(s.ad_value(2145), s.ad_value(2149)), (-1.0)), 1.0, s.ad_value(2105), A::add(A::offset(s.ad_value(2149), 1.0), s.ad_value(2156)), (-1.0)), (-1.0));
            }
        }

        if (s.b[2194] && (!s.b[2202])) {
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2167, 1.0, 2021, A::add_scaled_product(s.ad_value(2145), 1.0, s.ad_value(2105), s.ad_value(2158), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(2151, 2144, 2.0, 2021, A::add_scaled_sub_value_product(1.0, s.ad_value(2145), 1.0, s.ad_value(2105), A::offset(s.ad_value(2157), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(2152, 2103, 1.0, 2149, (-1.0), A::ln(A::div(s.ad_value(2150), s.ad_value(2021))), 1.0);
            s.store_add(813, 2150, 2151);
            s.store_add_scaled_square_product_mixed_iia(812, 813, 1.0, 2152, A::add_scaled_square_product(s.ad_value(2151), 0.5, s.ad_value(2150), s.ad_value(2167), (-1.0)), 1.0);
            s.store_add_ad_rhs(2168, 2149, A::div_scaled_product3(s.ad_value(2150), s.ad_value(813), s.ad_value(2152), 1.0, A::add(s.ad_value(812), A::mul3(A::mul3(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2152), s.ad_value(2152)), s.ad_value(2151), A::add_scaled_square_product(s.ad_value(2151), 0.3333333333333333, s.ad_value(2150), s.ad_value(2167), (-1.0)))), 1.0));
        }

        s.b[2203] = (s.v[2168] < 230.25850929940458);
        s.store_scalar(2203, if s.b[2203] { 1.0 } else { 0.0 });

        if ((s.b[2194] && (!s.b[2202])) && s.b[2203]) {
            s.store_exp(2154, 2168);
            s.store_div_from_scalar(2155, 1.0, 2154);
            s.store_mul(2154, 2105, 2154);
        }

        s.b[2204] = (s.v[2168] > (s.v[2103] - 230.25850929940458));
        s.store_scalar(2204, if s.b[2204] { 1.0 } else { 0.0 });

        if (((s.b[2194] && (!s.b[2202])) && (!s.b[2203])) && s.b[2204]) {
            s.store_exp_sub(2154, 2168, 2103);
            s.store_div(2155, 2105, 2154);
        }

        if (((s.b[2194] && (!s.b[2202])) && (!s.b[2203])) && (!s.b[2204])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2154, 1e-100, A::sub(s.ad_value(2103), s.ad_value(2168)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2155, 1e-100, 2168, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if (s.b[2194] && (!s.b[2202])) {
            s.store_div_from_scalar_offset_square(2144, 1.0, 2168, 2.0);
            s.store_mul_square_lhs(2156, 2168, 2144);
            s.store_mul3_affine_lhs(2157, 2168, 2144, 4.0, 0.0, 2144);
            s.store_mul_ad_product_lhs_mixed_ai(2158, A::sub_scaled_inputs(s.ad_value(2144), 8.0, s.ad_value(2156), 12.0), 2144, 2144);
            s.store_sub(2144, 2039, 2168);
            s.store_add_scaled_product_right_ad(2159, 2144, 2.0, 2021, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2155)), 1.0, s.ad_value(2154), 1.0, s.ad_value(2105), A::offset(s.ad_value(2157), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2160, 2144, 1.0, 2021, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2155), 1.0, s.ad_value(2168), 1.0, s.ad_value(2154), 1.0, (-1.0)), 1.0, s.ad_value(2105), A::add(A::offset(s.ad_value(2168), 1.0), s.ad_value(2156)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2144, 2.0, 2021, A::add_scaled_inputs_product(s.ad_value(2155), 1.0, s.ad_value(2154), 1.0, s.ad_value(2105), s.ad_value(2158), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(2144, 2159, 1.0, 2160, 2144, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(2106, 2168, 1.0, A::div(s.ad_value(2160), A::add(s.ad_value(2159), A::sqrt(s.ad_value(2144)))), 2.0);
        }

        if s.b[2194] {
            s.store_sub(2107, 2106, 2059);
        }

        s.b[2205] = (s.v[2107] < 1e-10);
        s.store_scalar(2205, if s.b[2205] { 1.0 } else { 0.0 });

        if (s.b[2194] && s.b[2205]) {
            s.store_add_scaled_inputs_product_right_ad(2108, 2039, 2.0, 2059, (-2.0), 2021, A::add_scaled_offset_product_rhs(A::add_scaled_sub_value_product(1.0, s.ad_value(2065), 1.0, s.ad_value(2064), s.ad_value(2104), 1.0), 1.0, s.ad_value(2105), s.ad_value(2062), 1.0, (-1.0)), 1.0);
            s.store_mul_ad_lhs(2109, A::mul_sub_from_scalar_rhs(s.ad_value(2021), 1.0, s.ad_value(2104)), 2066);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(1929, 2.0, 2021, A::add_scaled_value_products(s.ad_value(2065), 1.0, s.ad_value(2064), s.ad_value(2104), 1.0, s.ad_value(2105), s.ad_value(2063), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(1929, 2108, 1.0, 1929, 2109, (-2.0));
            s.store_scaled_div_ad_rhs(2107, 2109, A::add(s.ad_value(2108), A::sqrt(s.ad_value(1929))), 2.0);
            s.store_add(2106, 2059, 2107);
        }

        if s.b[2194] {
            s.store_mul(2110, 2107, 2035);
            s.store_div_scaled_product_offset_denominator(2111, s.ad_value(2106), s.ad_value(2106), 1.0, A::square(s.ad_value(2106)), 2.0, 1.0);
        }

        s.b[2206] = (s.v[2106] < 230.25850929940458);
        s.store_scalar(2206, if s.b[2206] { 1.0 } else { 0.0 });

        if (s.b[2194] && s.b[2206]) {
            s.store_exp_neg_input(2112, 2106);
        }

        s.b[2207] = (s.v[2106] < 1e-5);
        s.store_scalar(2207, if s.b[2207] { 1.0 } else { 0.0 });

        if ((s.b[2194] && s.b[2206]) && s.b[2207]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2113, 2106, 1.0, 2106, 1.0, 2106, 0.25, 0.3333333333333333, 0.5);
            s.store_sqrt_sub_from_scalar_ad(1929, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2106), 1.0, A::scale(s.ad_value(2106), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2114, 2106, 1929, 0.7071067811865475);
            s.store_mul3_ad_middle(2115, A::mul3_scaled_output(s.ad_value(2105), s.ad_value(2106), s.ad_value(2106), 0.16666666666666666), 2106, A::scale_offset(s.ad_value(2106), 1.75, 1.0));
        }

        if ((s.b[2194] && s.b[2206]) && (!s.b[2207])) {
            s.store_add_offset_lhs(2113, 2106, (-1.0), 2112);
            s.store_sqrt(2114, 2113);
            s.store_mul_add_scaled_inputs3_offset_rhs(2115, 2105, A::div_from_scalar(1.0, s.ad_value(2112)), 1.0, s.ad_value(2106), (-1.0), s.ad_value(2111), -1.0, (-1.0));
        }

        s.b[2208] = (s.v[2106] > (s.v[2103] - 230.25850929940458));
        s.store_scalar(2208, if s.b[2208] { 1.0 } else { 0.0 });

        if ((s.b[2194] && (!s.b[2206])) && s.b[2208]) {
            s.store_exp_sub(1929, 2106, 2103);
            s.store_div(2112, 2105, 1929);
            s.store_add_scaled_product_right_ad(2115, 1929, 1.0, 2105, A::add(A::offset(s.ad_value(2106), 1.0), s.ad_value(2111)), (-1.0));
        }

        if ((s.b[2194] && (!s.b[2206])) && (!s.b[2208])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2112, 1e-100, 2106, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(1929, 1e-100, A::sub(s.ad_value(2103), s.ad_value(2106)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_add_scaled_product_right_ad(2115, 1929, 1.0, 2105, A::add(A::offset(s.ad_value(2106), 1.0), s.ad_value(2111)), (-1.0));
        }

        if (s.b[2194] && (!s.b[2206])) {
            s.store_add_offset_lhs(2113, 2106, (-1.0), 2112);
            s.store_sqrt(2114, 2113);
        }

        if s.b[2194] {
            s.store_mul3_lhs(2116, 2114, 2020, 2035);
            s.store_scaled_add(2117, 2059, 2106, 0.5);
            s.store_scalar(2118, 0.0);
            s.store_mul(1929, 2112, 2065);
        }

        s.b[2209] = (s.v[1929] > 0.0);
        s.store_scalar(2209, if s.b[2209] { 1.0 } else { 0.0 });

        if (s.b[2194] && s.b[2209]) {
            s.store_sqrt(2118, 1929);
        }

        if s.b[2194] {
            s.store_scaled_add(2119, 2066, 2115, 0.5);
            s.store_add_scaled_product_mixed_iaa(2120, 2119, 1.0, A::square(s.ad_value(2107)), A::sub_scaled_inputs(s.ad_value(2118), 1.0, s.ad_value(2037), 2.0), 0.125);
        }

        s.b[2210] = (s.v[2117] < 1e-5);
        s.store_scalar(2210, if s.b[2210] { 1.0 } else { 0.0 });

        if (s.b[2194] && s.b[2210]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2121, 2117, 1.0, 2117, 1.0, 2117, 0.25, 0.3333333333333333, 0.5);
            s.store_mul_sqrt_ad_rhs(2122, 2020, A::add(s.ad_value(2120), s.ad_value(2121)));
        }

        s.b[2211] = (s.v[719] > 0.0);
        s.store_scalar(2211, if s.b[2211] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_12(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[2194] && s.b[2210]) && s.b[2211]) {
            s.store_div_from_scalar_sqrt_ad(2123, 1.0, A::offset(A::mul(s.ad_value(719), s.ad_value(2122)), 1.0));
        }

        if (s.b[2194] && s.b[2210]) {
            s.store_sqrt_sub_from_scalar_ad(1929, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2117), 1.0, A::scale(s.ad_value(2117), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2124, 2117, 1929, 0.7071067811865475);
            s.store_add_ad_rhs(2125, 2123, A::div_scaled_product(s.ad_value(2020), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2117), 0.5)), 1.0, A::square(s.ad_value(2117)), 0.16666666666666666), 0.7071067811865475, s.ad_value(1929), 1.0));
        }

        if (s.b[2194] && (!s.b[2210])) {
            s.store_add_offset_lhs(2121, 2117, (-1.0), 2118);
            s.store_mul_sqrt_ad_rhs(2122, 2020, A::add(s.ad_value(2120), s.ad_value(2121)));
        }

        s.b[2212] = (s.v[719] > 0.0);
        s.store_scalar(2212, if s.b[2212] { 1.0 } else { 0.0 });

        if ((s.b[2194] && (!s.b[2210])) && s.b[2212]) {
            s.store_add_scaled_sub_value_product_indices(2126, 1.0, 2118, 1.0, 2122, 2037, 2.0);
            s.store_div_from_scalar_sqrt_ad(2123, 1.0, A::offset(A::mul(s.ad_value(719), s.ad_value(2122)), 1.0));
            s.store_div_scaled_value_offset_denominator(1929, s.ad_value(2123), 1.0, s.ad_value(2123), 1.0, 1.0);
            s.store_mul_product3_mixed_iaii(2127, 719, A::square(s.ad_value(1929)), 2021, 2120, 1.0);
            s.store_add_scaled_inputs_product_right_ad(2128, 2122, 2.0, 2127, (-2.0), 2021, A::add(A::sub_from_scalar(1.0, s.ad_value(2118)), s.ad_value(2120)), 1.0);
            s.store_mul_sub_scaled_inputs_rhs(2129, 2127, s.ad_value(2127), 1.0, s.ad_value(2122), 2.0);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2130, 1.0, 2021, A::add(s.ad_value(2118), s.ad_value(2120)), 0.5);
            s.store_div_scaled_product_denominator_ad(2131, 2129, 2128, 1.0, A::add_scaled_square_product(s.ad_value(2128), 1.0, s.ad_value(2130), s.ad_value(2129), (-1.0)), 1.0);
            s.store_add(2117, 2117, 2131);
            s.store_exp(2132, 2131);
            s.store_div(2118, 2118, 2132);
            s.store_mul(2120, 2120, 2132);
            s.store_add_offset_lhs(2121, 2117, (-1.0), 2118);
            s.store_mul_sqrt_ad_rhs(2122, 2020, A::add(s.ad_value(2120), s.ad_value(2121)));
            s.store_add_ad(2133, A::sub_from_scalar(1.0, s.ad_value(2118)), A::mul3_scaled_output(s.ad_value(2122), s.ad_value(2123), s.ad_value(2037), 2.0));
            s.store_div_scaled_product3_mixed_iiaa(2107, 2107, 2132, A::add(s.ad_value(2126), s.ad_value(2119)), 1.0, A::add_scaled_product(s.ad_value(2133), 1.0, s.ad_value(2132), s.ad_value(2119), 1.0), 1.0);
            s.store_mul(2110, 2107, 2035);
        }

        if (s.b[2194] && (!s.b[2210])) {
            s.store_sqrt(2124, 2121);
            s.store_add_scaled_inputs_ad_rhs(2125, 2123, 1.0, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2020), 1.0, s.ad_value(2118)), s.ad_value(2124)), 0.5);
        }

        if s.b[2194] {
            s.store_mul_div_scaled_product_mixed_iiia(2134, 2035, 2021, 2120, 1.0, A::add_scaled_product(s.ad_value(2122), 1.0, s.ad_value(2020), s.ad_value(2124), 1.0), 1.0);
            s.store_add_scaled_product_indices(2135, 2134, 1.0, 2035, 2125, 1.0);
            s.store_mul3_lhs(2136, 2124, 2020, 2035);
        }

        s.b[2213] = (s.v[216] < 0.0);
        s.store_scalar(2213, if s.b[2213] { 1.0 } else { 0.0 });

        if (s.b[2194] && s.b[2213]) {
            s.store_sub_from_scalar_scaled_mul(2075, 1.0, 216, 2134, 1.0);
        }

        if (s.b[2194] && (!s.b[2213])) {
            s.store_div_from_scalar_offset_product(2075, 1.0, 216, 2134, 1.0);
        }

        if s.b[2194] {
            s.store_mul_product3_indices(2076, 2134, 746, 2074, 2075, 1.0);
            s.store_add_scaled_product_indices(2137, 2136, 1.0, 764, 2134, 1.0);
            s.store_add_scaled_product_indices(2138, 2136, 1.0, 765, 2134, 1.0);
            s.store_mul(2139, 763, 2137);
            s.store_ln_ad(1930, A::div_scaled_value_offset_denominator(s.ad_value(2121), 1.0, A::add(s.ad_value(2121), s.ad_value(2120)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2078, A::pow(A::mul(s.ad_value(2139), s.ad_value(705)), s.ad_value(706)), 1.0, 707, A::exp(A::mul_scaled_lhs(s.ad_value(708), 0.5, s.ad_value(1930))), 1.0);
            s.store_mul_add_ad_lhs(2140, A::offset(s.ad_value(2078), 1.0), s.ad_value(2076), 2070);
            s.store_ln_ad(2141, A::div_scaled_offset_numerator(A::mul(A::sub(s.ad_value(815), s.ad_value(2110)), s.ad_value(768)), 1.0, 1.0, A::offset(A::mul(A::sub(s.ad_value(2101), s.ad_value(2110)), s.ad_value(768)), 1.0), 1.0));
            s.store_mul(1931, 2134, 2080);
            s.store_div_add_scaled_inputs_rhs_indices(2081, 1931, 221, 1.0, 1931, 1.0);
        }

        s.b[2214] = (s.v[220] < 0.0);
        s.store_scalar(2214, if s.b[2214] { 1.0 } else { 0.0 });

        if (s.b[2194] && s.b[2214]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2082, 1.0, 1.0, A::mul(s.ad_value(220), s.ad_value(2081)));
        }

        if (s.b[2194] && (!s.b[2214])) {
            s.store_offset_mul(2082, 220, 2081, 1.0);
        }

        if s.b[2194] {
            s.store_mul(2143, 2015, 2082);
            s.store_mul(2142, 2122, 2035);
        }

        s.copy_ad(1839, 2083);

        s.copy_ad(1841, 2101);

        s.copy_ad(1842, 2102);

        s.copy_ad(1843, 2107);

        s.copy_ad(1844, 2110);

        s.copy_ad(1846, 2117);

        s.copy_ad(1845, 2116);

        s.copy_ad(1847, 2123);

        s.copy_ad(1848, 2125);

        s.copy_ad(1849, 2134);

        s.copy_ad(1850, 2135);

        s.copy_ad(1851, 2136);

        s.copy_ad(1852, 2138);

        s.copy_ad(1853, 2140);

        s.copy_ad(1855, 2141);

        s.copy_ad(1854, 2143);

        s.copy_ad(1856, 2142);

        s.store_scalar(1857, 1.0);

        s.store_scalar(1858, 1.0);

        s.store_scalar(1860, 1.0);

        s.store_scalar(1861, 1.0);

        s.store_scalar(827, 0.0);

        s.b[2215] = (s.v[1813] > 0.0);
        s.store_scalar(2215, if s.b[2215] { 1.0 } else { 0.0 });

        if s.b[2215] {
            s.store_ln_ad(1939, A::offset(A::mul(s.ad_value(819), s.ad_value(768)), 1.0));
            s.store_div_scaled_product_indices(1929, 1808, 1848, 1.0, 1850, 1.0);
            s.store_add_scaled_product_mixed_aai(1938, A::mul3(A::mul3(s.ad_value(225), s.ad_value(1851), s.ad_value(1929)), s.ad_value(1929), s.ad_value(1939)), 1.0, A::div_scaled_product(A::add(s.ad_value(223), A::div(s.ad_value(224), s.ad_value(1850))), s.ad_value(1849), 1.0, s.ad_value(1850), 1.0), 1855, 1.0);
            s.store_div_from_scalar_add_ad(1857, 1.0, A::offset(s.ad_value(1938), 1.0), A::square(s.ad_value(1938)));
            s.store_mul(1858, 1853, 1857);
            s.store_div(1859, 1854, 1858);
            s.store_mul_ad_product_lhs_mixed_ai(1940, A::square(s.ad_value(1859)), 1844, 1844);
        }

        s.b[2216] = (s.v[0] == (-1.0));
        s.store_scalar(2216, if s.b[2216] { 1.0 } else { 0.0 });

        if (s.b[2215] && s.b[2216]) {
            s.store_div_scaled_value_offset_denominator(1940, s.ad_value(1940), 1.0, A::mul(s.ad_value(1859), s.ad_value(1844)), 1.0, 1.0);
        }

        if s.b[2215] {
            s.store_mul_offset_rhs_scaled_ad_rhs(1941, 1858, A::sqrt(A::scale_offset(s.ad_value(1940), 2.0, 1.0)), 1.0, 0.5);
            s.store_div_from_scalar(1860, 1.0, 1941);
            s.store_mul(1929, 1858, 1860);
            s.store_mul_offset_ad_rhs(1942, 1848, A::mul3_scaled_output(s.ad_value(1940), s.ad_value(1929), s.ad_value(1929), 0.5), 1.0);
            s.store_div_scaled_product_indices(1861, 1929, 1850, 1.0, 1942, 1.0);
            s.store_mul_product3_indices(827, 1860, 1917, 1850, 1844, 1.0);
        }

        s.store_scalar(1944, 0.0);

        s.store_scalar(1945, 0.0);

        s.store_scalar(1862, 0.0);

        s.store_scalar(1863, 0.0);

        s.b[2217] = (((((p.p40 != 0.0) && ((s.v[235] > 0.0) || (s.v[236] > 0.0))) || ((p.p42 != 0.0) && ((s.v[245] > 0.0) || (s.v[246] > 0.0)))) || (s.v[260] > 0.0)) || (s.v[261] > 0.0));
        s.store_scalar(2217, if s.b[2217] { 1.0 } else { 0.0 });

        if s.b[2217] {
            s.store_scaled_add_ad_rhs(1943, 1801, A::sqrt(A::add(A::square(s.ad_value(1801)), s.ad_value(778))), 0.5);
            s.store_add_ad_lhs(1944, A::add_scaled_inputs_product(s.ad_value(1943), -1.0, s.ad_value(773), (-0.5), s.ad_value(771), A::sqrt(A::add_scaled_inputs3(s.ad_value(1943), 1.0, s.ad_value(773), 0.25, s.ad_value(779), 1.0)), 1.0), 780);
            s.store_scaled_add_ad_rhs(1943, 1802, A::sqrt(A::add(A::square(s.ad_value(1802)), s.ad_value(781))), 0.5);
            s.store_add_ad_lhs(1945, A::add_scaled_inputs_product(s.ad_value(1943), -1.0, s.ad_value(774), (-0.5), s.ad_value(772), A::sqrt(A::add_scaled_inputs3(s.ad_value(1943), 1.0, s.ad_value(774), 0.25, s.ad_value(782), 1.0)), 1.0), 783);
            s.store_scaled_add(1862, 1801, 1944, (-s.v[355]));
            s.store_scaled_add(1863, 1802, 1945, (-s.v[355]));
        }

        s.b[2218] = (p.p40 != 0.0);
        s.store_scalar(2218, if s.b[2218] { 1.0 } else { 0.0 });

        s.b[2219] = (s.v[235] > 0.0);
        s.store_scalar(2219, if s.b[2219] { 1.0 } else { 0.0 });

        if (s.b[2218] && s.b[2219]) {
            s.store_mul_sqrt_ad_lhs(1946, A::offset(A::square(s.ad_value(1862)), 1e-6), 784);
        }

        s.b[2220] = (s.v[241] < 0.0);
        s.store_scalar(2220, if s.b[2220] { 1.0 } else { 0.0 });

        if ((s.b[2218] && s.b[2219]) && s.b[2220]) {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1946, 1946, 0.5, 790, 0.5, 1946, 790, 1e-6, (-0.5));
        }

        if (s.b[2218] && s.b[2219]) {
            s.store_mul_offset_ad_rhs(1929, 787, A::mul(s.ad_value(1946), A::add_scaled_product(s.ad_value(240), 1.0, s.ad_value(241), s.ad_value(1946), 1.0)), (-1.5));
            s.store_offset(1948, 1944, 3.0);
            s.store_sub_from_scalar(1949, (-3.0), 233);
            s.store_scale(1950, 823, 30.0);
            s.store_scalar(807, (4.0 - 0.9));
            s.store_add(808, 1948, 1950);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1929, 2.0, 807, A::sub(s.ad_value(808), A::sqrt(A::sub(A::square(s.ad_value(808)), A::mul3(s.ad_value(807), s.ad_value(1948), s.ad_value(1950))))));
            s.store_scalar(807, (4.0 - 0.3));
            s.store_add(808, 1949, 1929);
        }

        s.b[2223] = (s.v[236] > 0.0);
        s.store_scalar(2223, if s.b[2223] { 1.0 } else { 0.0 });

        if (s.b[2218] && s.b[2223]) {
            s.store_mul_sqrt_ad_lhs(1946, A::offset(A::square(s.ad_value(1863)), 1e-6), 784);
        }

        s.b[2224] = (s.v[243] < 0.0);
        s.store_scalar(2224, if s.b[2224] { 1.0 } else { 0.0 });

        if ((s.b[2218] && s.b[2223]) && s.b[2224]) {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1946, 1946, 0.5, 791, 0.5, 1946, 791, 1e-6, (-0.5));
        }

        if (s.b[2218] && s.b[2223]) {
            s.store_mul_offset_ad_rhs(1929, 788, A::mul(s.ad_value(1946), A::add_scaled_product(s.ad_value(242), 1.0, s.ad_value(243), s.ad_value(1946), 1.0)), (-1.5));
            s.store_offset(1948, 1945, 3.0);
            s.store_sub_from_scalar(1949, (-3.0), 233);
            s.store_scale(1950, 826, 30.0);
            s.store_scalar(807, (4.0 - 0.9));
            s.store_add(808, 1948, 1950);
            s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1929, 2.0, 807, A::sub(s.ad_value(808), A::sqrt(A::sub(A::square(s.ad_value(808)), A::mul3(s.ad_value(807), s.ad_value(1948), s.ad_value(1950))))));
            s.store_scalar(807, (4.0 - 0.3));
            s.store_add(808, 1949, 1929);
        }

        s.b[2227] = (s.v[234] > 0.0);
        s.store_scalar(2227, if s.b[2227] { 1.0 } else { 0.0 });

        s.b[2228] = (s.v[1813] <= 0.0);
        s.store_scalar(2228, if s.b[2228] { 1.0 } else { 0.0 });

        if ((s.b[2218] && s.b[2227]) && s.b[2228]) {
            s.store_offset(1929, 766, 1.0);
            s.store_div_scaled_product_left_ad(1930, A::sqrt(s.ad_value(1929)), 815, 1.0, 1839, 1.0);
            s.store_add_ad_lhs(1931, A::square(s.ad_value(1930)), 1929);
            s.store_scale(1929, 1930, 2.0);
            s.store_div_scaled_product3_mixed_iiia(1842, 1839, 1809, 1929, 1.0, A::add(A::sqrt(A::sub(s.ad_value(1931), s.ad_value(1929))), A::sqrt(A::add(s.ad_value(1931), s.ad_value(1929)))), 1.0);
        }

        s.b[2229] = ((s.v[1843] - s.v[1842]) > (-230.25850929940458));
        s.store_scalar(2229, if s.b[2229] { 1.0 } else { 0.0 });

        if ((s.b[2218] && s.b[2227]) && s.b[2229]) {
            s.store_exp_sub(1929, 1843, 1842);
        }

        if ((s.b[2218] && s.b[2227]) && (!s.b[2229])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1929, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1843), s.ad_value(1842)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (s.b[2218] && s.b[2227]) {
            s.store_add_scaled_product_right_ad(1952, 1932, 1.0, 1808, A::sub_scaled_inputs(s.ad_value(1843), 0.5, A::ln_scaled_input(A::offset(s.ad_value(1929), 1.0), 0.5), 1.0), 1.0);
            s.store_mul(1953, 233, 1808);
            s.store_add(1954, 1856, 1953);
            s.store_scaled_sub_ad_rhs(1955, 1954, A::sqrt_square_offset(A::neg(s.ad_value(1954)), 0.01), 0.5);
            s.store_mul_sqrt_ad_lhs(1946, A::offset(A::square(s.ad_value(1856)), 1e-6), 784);
        }

        s.b[2230] = (s.v[239] < 0.0);
        s.store_scalar(2230, if s.b[2230] { 1.0 } else { 0.0 });

        if ((s.b[2218] && s.b[2227]) && s.b[2230]) {
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1946, 1946, 0.5, 789, 0.5, 1946, 789, 1e-6, (-0.5));
        }

        if (s.b[2218] && s.b[2227]) {
            s.store_add_scaled_product_left_ad(1956, 1846, 1.0, A::add_scaled_inputs3(s.ad_value(1955), 1.0, s.ad_value(731), (-1.0), s.ad_value(1952), -1.0), 1809, 1.0);
            s.store_mul_neg_ad_lhs(1956, A::add_scaled_inputs3(s.ad_value(814), 1.0, s.ad_value(1932), 1.0, s.ad_value(1952), -1.0), 1809);
        }

        s.b[2233] = (((s.v[1956]) as f64).abs() < 230.25850929940458);
        s.store_scalar(2233, if s.b[2233] { 1.0 } else { 0.0 });

        if ((s.b[2218] && s.b[2227]) && s.b[2233]) {
            s.store_exp(1929, 1956);
        }

        s.b[2234] = (s.v[1956] < 0.0);
        s.store_scalar(2234, if s.b[2234] { 1.0 } else { 0.0 });

    }

    pub(super) fn stamp_reactive_block_13(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[2218] && s.b[2227]) && (!s.b[2233])) && s.b[2234]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1929, 1e-100, (-230.25850929940458), 1956, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[2218] && s.b[2227]) && (!s.b[2233])) && (!s.b[2234])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(1929, 1956, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (s.b[2218] && s.b[2227]) {
            s.store_mul_offset_ad_rhs(1929, 786, A::mul(s.ad_value(1946), A::add_scaled_product(s.ad_value(238), 1.0, s.ad_value(239), s.ad_value(1946), 1.0)), (-1.5));
        }

        s.b[2237] = ((s.v[1813] <= 0.0) || ((s.v[238] == 0.0) && (s.v[239] == 0.0)));
        s.store_scalar(2237, if s.b[2237] { 1.0 } else { 0.0 });

        if ((s.b[2218] && s.b[2227]) && (!s.b[2237])) {
            s.store_add_scaled_product_indices(1929, 238, 1.0, 239, 1946, 2.0);
            s.store_div_ad_rhs(1960, 244, A::mul(s.ad_value(1929), s.ad_value(786)));
            s.store_scaled_div(1961, 1844, 1960, 0.5);
        }

        s.b[2238] = (s.v[1961] < 0.001);
        s.store_scalar(2238, if s.b[2238] { 1.0 } else { 0.0 });

        s.b[2239] = (((s.v[1961]) as f64).abs() < 230.25850929940458);
        s.store_scalar(2239, if s.b[2239] { 1.0 } else { 0.0 });

        if ((((s.b[2218] && s.b[2227]) && (!s.b[2237])) && (!s.b[2238])) && s.b[2239]) {
            s.store_exp(1969, 1961);
        }

        s.b[2240] = (s.v[1961] < 0.0);
        s.store_scalar(2240, if s.b[2240] { 1.0 } else { 0.0 });

        if (((((s.b[2218] && s.b[2227]) && (!s.b[2237])) && (!s.b[2238])) && (!s.b[2239])) && s.b[2240]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1969, 1e-100, (-230.25850929940458), 1961, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2218] && s.b[2227]) && (!s.b[2237])) && (!s.b[2238])) && (!s.b[2239])) && (!s.b[2240])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(1969, 1961, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[2218] && s.b[2227]) && (!s.b[2237])) && (!s.b[2238])) {
            s.store_div_from_scalar(1970, 1.0, 1969);
            s.store_sub(1929, 1969, 1970);
            s.store_add(1931, 1969, 1970);
        }

        s.b[2241] = (p.p42 != 0.0);
        s.store_scalar(2241, if s.b[2241] { 1.0 } else { 0.0 });

        s.b[2242] = ((s.v[246] > 0.0) && (s.v[1863] < 0.0));
        s.store_scalar(2242, if s.b[2242] { 1.0 } else { 0.0 });

        if (s.b[2241] && s.b[2242]) {
            s.store_sqrt_offset_ad(1973, A::add_scaled_square_product(s.ad_value(1863), 1.0, A::square(s.ad_value(252)), A::square(s.ad_value(825)), 1.0), 1e-6);
            s.store_div_scaled_inputs_indices(1929, 796, -1.0, 1973, 1.0);
        }

        s.b[2243] = (s.v[1929] > (-230.25850929940458));
        s.store_scalar(2243, if s.b[2243] { 1.0 } else { 0.0 });

        if ((s.b[2241] && s.b[2242]) && s.b[2243]) {
            s.store_exp(1931, 1929);
        }

        if ((s.b[2241] && s.b[2242]) && (!s.b[2243])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1931, 1e-100, (-230.25850929940458), 1929, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2244] = ((s.v[245] > 0.0) && (s.v[1862] < 0.0));
        s.store_scalar(2244, if s.b[2244] { 1.0 } else { 0.0 });

        if (s.b[2241] && s.b[2244]) {
            s.store_sqrt_offset_ad(1974, A::add_scaled_square_product(s.ad_value(1862), 1.0, A::square(s.ad_value(251)), A::square(s.ad_value(824)), 1.0), 1e-6);
            s.store_div_scaled_inputs_indices(1929, 795, -1.0, 1974, 1.0);
        }

        s.b[2245] = (s.v[1929] > (-230.25850929940458));
        s.store_scalar(2245, if s.b[2245] { 1.0 } else { 0.0 });

        if ((s.b[2241] && s.b[2244]) && s.b[2245]) {
            s.store_exp(1931, 1929);
        }

        if ((s.b[2241] && s.b[2244]) && (!s.b[2245])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1931, 1e-100, (-230.25850929940458), 1929, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.copy_ad(1978, 1916);

        s.store_scalar(1864, 0.0);

        s.store_scalar(1865, 0.0);

        s.store_scalar(1866, 0.0);

        s.store_scalar(1867, 1e-40);

        s.store_scalar(1868, 1.0);

        s.store_scalar(835, 0.0);

        s.b[2246] = ((p.p46 != 0.0) && (s.v[285] > 0.0));
        s.store_scalar(2246, if s.b[2246] { 1.0 } else { 0.0 });

        if s.b[2246] {
            s.store_add_scaled_inputs4_mixed_iiai(1929, 817, 0.5, 816, 0.5, A::sqrt(A::add(A::square(A::sub(s.ad_value(817), s.ad_value(816))), s.ad_value(753))), (-0.5), 751, 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(1975, 816, 1.0, 1929, (-0.5), A::sqrt(A::add(A::square(s.ad_value(1929)), s.ad_value(752))), (-(-0.5)), 755, 1.0);
            s.store_add_scaled_inputs3_indices(1976, 1975, 1.0, 815, 0.5, 819, (-0.5));
            s.store_mul_ad_product_rhs(1977, 287, A::offset(A::mul(s.ad_value(289), s.ad_value(819)), 1.0), A::offset(A::mul(s.ad_value(288), s.ad_value(1976)), 1.0));
            s.store_mul_offset_rhs(1978, 1924, 1977, 1.0);
            s.store_div_from_scalar(1979, 1.0, 1978);
            s.store_div_scaled_value_offset_denominator(1980, s.ad_value(819), 2.0, A::sqrt_product_offset(s.ad_value(291), s.ad_value(819), 1.0), 1.0, 1.0);
            s.store_mul_ad_product_rhs_mixed_ia(1981, 290, 1980, A::offset(A::mul(s.ad_value(292), s.ad_value(1976)), 1.0));
            s.store_mul_add_scaled_inputs3_offset_rhs(1864, 1979, s.ad_value(818), 1.0, s.ad_value(1981), 1.0, s.ad_value(714), -1.0, 0.0);
            s.store_mul(1982, 1979, 749);
            s.store_scaled_ln_ad(1983, A::add(A::div(s.ad_value(1982), s.ad_value(750)), A::sqrt(s.ad_value(1982))), 2.0);
            s.store_mul(1984, 1979, 1975);
            s.store_add(1989, 1982, 1984);
            s.store_add_scaled_product_right_ad(1990, 1989, 1.0, 750, A::sqrt(s.ad_value(1989)), 1.0);
            s.store_add(1991, 1990, 1983);
            s.store_offset_div_scaled_inputs_mixed_ia(1992, 750, 1.0, A::sqrt(s.ad_value(1989)), 2.0, 1.0);
            s.store_div_from_scalar(1993, 1.0, 1992);
            s.store_sub(1994, 1864, 1991);
        }

        s.b[2247] = (s.v[1994] > (-12.0));
        s.store_scalar(2247, if s.b[2247] { 1.0 } else { 0.0 });

        if (s.b[2246] && s.b[2247]) {
            s.store_offset_add(1995, 1994, 1926, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(1996, 1995, 1995, 10.0, 0.5);
            s.store_add_ad_lhs(1997, A::add_scaled_product(s.ad_value(1994), 1.0, s.ad_value(1992), A::ln(s.ad_value(1996)), (-1.0)), 1926);
            s.store_scaled_add_sqrt_square_offset_rhs(1998, 1997, 1997, 2.0, 0.5);
        }

        s.b[2248] = ((s.v[1994] - s.v[1998]) < 230.25850929940458);
        s.store_scalar(2248, if s.b[2248] { 1.0 } else { 0.0 });

        if ((s.b[2246] && s.b[2247]) && s.b[2248]) {
            s.store_exp_sub(1999, 1994, 1998);
        }

        if ((s.b[2246] && s.b[2247]) && (!s.b[2248])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(1999, A::sub(s.ad_value(1994), s.ad_value(1998)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (s.b[2246] && s.b[2247]) {
            s.store_mul(2000, 1925, 1999);
            s.store_pow_indices(2001, 2000, 1993);
            s.store_add_scaled_square_product_mixed_iai(2002, 1992, 1.0, A::add_scaled_inputs3(s.ad_value(1998), 2.0, s.ad_value(1992), 2.0, s.ad_value(2001), -1.0), 2001, 1.0);
            s.store_mul_offset_ad_rhs(2003, 1992, A::div_scaled_inputs2(A::sqrt(s.ad_value(2002)), 1.0, s.ad_value(1992), (-1.0), s.ad_value(2001), 1.0), (-1.0));
            s.store_sub(1985, 1998, 2003);
        }

        s.b[2249] = ((s.v[1993] * (s.v[1994] + s.v[1926])) > (-230.25850929940458));
        s.store_scalar(2249, if s.b[2249] { 1.0 } else { 0.0 });

        if ((s.b[2246] && (!s.b[2247])) && s.b[2249]) {
            s.store_exp_ad(1985, A::mul(s.ad_value(1993), A::add(s.ad_value(1994), s.ad_value(1926))));
        }

        if ((s.b[2246] && (!s.b[2247])) && (!s.b[2249])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1985, 1e-100, (-230.25850929940458), A::mul(s.ad_value(1993), A::add(s.ad_value(1994), s.ad_value(1926))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if s.b[2246] {
            s.store_mul_add_rhs(1986, 1979, 1841, 1975);
        }

        s.b[2250] = ((s.v[1985] < 0.001) && (s.v[1841] < 1e-6));
        s.store_scalar(2250, if s.b[2250] { 1.0 } else { 0.0 });

        s.b[2251] = (((-s.v[1986]) + s.v[1984]) > (-230.25850929940458));
        s.store_scalar(2251, if s.b[2251] { 1.0 } else { 0.0 });

        if ((s.b[2246] && s.b[2250]) && s.b[2251]) {
            s.store_exp_sub(1929, 1984, 1986);
        }

        if ((s.b[2246] && s.b[2250]) && (!s.b[2251])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1929, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1984), s.ad_value(1986)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (s.b[2246] && s.b[2250]) {
            s.store_mul_offset_rhs(1865, 1985, 1929, (-1.0));
            s.store_add(1987, 1865, 1985);
        }

        if (s.b[2246] && (!s.b[2250])) {
            s.store_add(1989, 1982, 1986);
            s.store_add_scaled_product_right_ad(1990, 1989, 1.0, 750, A::sqrt(s.ad_value(1989)), 1.0);
            s.store_add(1991, 1990, 1983);
            s.store_offset_div_scaled_inputs_mixed_ia(1992, 750, 1.0, A::sqrt(s.ad_value(1989)), 2.0, 1.0);
            s.store_div_from_scalar(1993, 1.0, 1992);
            s.store_sub(1994, 1864, 1991);
        }

        s.b[2252] = (s.v[1994] > (-12.0));
        s.store_scalar(2252, if s.b[2252] { 1.0 } else { 0.0 });

        if ((s.b[2246] && (!s.b[2250])) && s.b[2252]) {
            s.store_offset_add(1995, 1994, 1926, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(1996, 1995, 1995, 10.0, 0.5);
            s.store_add_ad_lhs(1997, A::add_scaled_product(s.ad_value(1994), 1.0, s.ad_value(1992), A::ln(s.ad_value(1996)), (-1.0)), 1926);
            s.store_scaled_add_sqrt_square_offset_rhs(1998, 1997, 1997, 2.0, 0.5);
        }

        s.b[2253] = ((s.v[1994] - s.v[1998]) < 230.25850929940458);
        s.store_scalar(2253, if s.b[2253] { 1.0 } else { 0.0 });

        if (((s.b[2246] && (!s.b[2250])) && s.b[2252]) && s.b[2253]) {
            s.store_exp_sub(1999, 1994, 1998);
        }

        if (((s.b[2246] && (!s.b[2250])) && s.b[2252]) && (!s.b[2253])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(1999, A::sub(s.ad_value(1994), s.ad_value(1998)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[2246] && (!s.b[2250])) && s.b[2252]) {
            s.store_mul(2000, 1925, 1999);
            s.store_pow_indices(2001, 2000, 1993);
            s.store_add_scaled_square_product_mixed_iai(2002, 1992, 1.0, A::add_scaled_inputs3(s.ad_value(1998), 2.0, s.ad_value(1992), 2.0, s.ad_value(2001), -1.0), 2001, 1.0);
            s.store_mul_offset_ad_rhs(2003, 1992, A::div_scaled_inputs2(A::sqrt(s.ad_value(2002)), 1.0, s.ad_value(1992), (-1.0), s.ad_value(2001), 1.0), (-1.0));
            s.store_sub(1987, 1998, 2003);
        }

        s.b[2254] = ((s.v[1993] * (s.v[1994] + s.v[1926])) > (-230.25850929940458));
        s.store_scalar(2254, if s.b[2254] { 1.0 } else { 0.0 });

        if (((s.b[2246] && (!s.b[2250])) && (!s.b[2252])) && s.b[2254]) {
            s.store_exp_ad(1987, A::mul(s.ad_value(1993), A::add(s.ad_value(1994), s.ad_value(1926))));
        }

        if (((s.b[2246] && (!s.b[2250])) && (!s.b[2252])) && (!s.b[2254])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1987, 1e-100, (-230.25850929940458), A::mul(s.ad_value(1993), A::add(s.ad_value(1994), s.ad_value(1926))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (s.b[2246] && (!s.b[2250])) {
            s.store_sub(1865, 1987, 1985);
        }

        if s.b[2246] {
            s.store_scaled_add(1866, 1987, 1985, 0.5);
        }

        if s.b[2246] {
            if ((s.v[1864] - s.v[1866]) > 1e-40) {
                s.store_sub(1867, 1864, 1866);
            } else {
                s.store_scalar(1867, 1e-40);
            }
        }

        if s.b[2246] {
            s.store_sub_from_scalar_ad(1868, 1.0, A::div_scaled_inputs(s.ad_value(750), 0.5, A::sqrt(A::add_scaled_inputs(s.ad_value(1867), 1.0, s.ad_value(1925), 0.25)), 1.0));
            s.store_div_scaled_product3_mixed_aaii(835, A::mul3_scaled_output(s.ad_value(1918), s.ad_value(1978), s.ad_value(1978), -1.0), A::offset(A::mul(s.ad_value(1868), s.ad_value(1866)), 1.0), 1865, 1.0, 1853, 1.0);
        }

        s.store_scalar(1869, 0.0);

        s.store_scalar(836, 0.0);

        s.b[2255] = ((s.v[1813] > 0.0) && (p.p41 != 0.0));
        s.store_scalar(2255, if s.b[2255] { 1.0 } else { 0.0 });

        if s.b[2255] {
            s.store_add_scaled_product_indices(1988, 815, 1.0, 230, 1844, (-1.0));
        }

        s.b[2256] = (s.v[1988] > 0.0);
        s.store_scalar(2256, if s.b[2256] { 1.0 } else { 0.0 });

        if (s.b[2255] && s.b[2256]) {
            s.store_mul_div_scaled_offset_numerator_rhs(1931, 713, A::mul(s.ad_value(231), A::sub(A::sqrt(A::add(s.ad_value(717), s.ad_value(1932))), s.ad_value(725))), 1.0, 1.0, A::offset(s.ad_value(1988), 1e-30), 1.0);
        }

        s.b[2257] = ((((-s.v[1931])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2257, if s.b[2257] { 1.0 } else { 0.0 });

        if ((s.b[2255] && s.b[2256]) && s.b[2257]) {
            s.store_exp_neg_input(1929, 1931);
        }

        s.b[2258] = ((-s.v[1931]) < 0.0);
        s.store_scalar(2258, if s.b[2258] { 1.0 } else { 0.0 });

        if (((s.b[2255] && s.b[2256]) && (!s.b[2257])) && s.b[2258]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1929, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1931)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[2255] && s.b[2256]) && (!s.b[2257])) && (!s.b[2258])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(1929, A::neg(s.ad_value(1931)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (s.b[2255] && s.b[2256]) {
            s.store_mul3_lhs(1869, 227, 1988, 1929);
            s.store_mul_add_rhs(836, 1869, 827, 835);
        }

        s.b[2259] = (s.v[836] > (0.5 * s.v[232]));
        s.store_scalar(2259, if s.b[2259] { 1.0 } else { 0.0 });

        if ((s.b[2255] && s.b[2256]) && s.b[2259]) {
            s.store_offset_div_scaled_inputs_indices(1929, 836, 2.0, 232, 1.0, (-1.0));
            s.store_mul_scaled_offset_ad_rhs(836, 232, 0.5, A::div(s.ad_value(1929), A::sqrt_square_offset(s.ad_value(1929), 1.0)), 1.0);
        }

        s.b[2453] = (((p.p45 == 1.0) || (p.p47 > 0.0)) || (p.p48 > 0.0));
        s.store_scalar(2453, if s.b[2453] { 1.0 } else { 0.0 });

        s.b[2454] = ((p.p45 > 0.0) || (p.p47 > 0.0));
        s.store_scalar(2454, if s.b[2454] { 1.0 } else { 0.0 });

        if (s.b[2453] && s.b[2454]) {
            s.copy_ad(2294, 717);
            s.copy_ad(2295, 727);
            s.copy_ad(2296, 718);
            s.copy_ad(2297, 1804);
            s.copy_ad(2298, 1805);
            s.store_scalar(2302, 0.0);
        }

        s.b[2455] = (p.p47 > 0.0);
        s.store_scalar(2455, if s.b[2455] { 1.0 } else { 0.0 });

        if ((s.b[2453] && s.b[2454]) && s.b[2455]) {
            s.store_add_scaled_inputs4_mixed_iiai(2297, 817, 0.5, 816, 0.5, A::sqrt(A::add(A::square(A::sub(s.ad_value(817), s.ad_value(816))), s.ad_value(738))), (-0.5), 736, 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(1870, 816, 1.0, 2297, (-0.5), A::sqrt(A::add(A::square(s.ad_value(2297)), s.ad_value(737))), (-(-0.5)), 739, 1.0);
            s.copy_ad(2298, 1870);
            s.copy_ad(2294, 734);
            s.copy_ad(2295, 737);
            s.copy_ad(2296, 735);
        }

        if (s.b[2453] && s.b[2454]) {
            s.store_add_scaled_inputs3_indices(2301, 818, 1.0, 2302, (-1.0), 701, -1.0);
        }

    }

    pub(super) fn stamp_reactive_block_14(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[2453] && s.b[2454]) {
            s.store_add_scaled_inputs3_indices(2303, 2298, 1.0, 815, 0.5, 819, (-0.5));
            s.store_scalar(2315, 1.0);
        }

        s.b[2456] = (s.v[188] > 0.0);
        s.store_scalar(2456, if s.b[2456] { 1.0 } else { 0.0 });

        if ((s.b[2453] && s.b[2454]) && s.b[2456]) {
            s.store_mul(2306, 2294, 362);
            s.store_mul(2307, 2303, 362);
            s.store_mul(2308, 2301, 362);
            s.store_offset_div_scaled_inputs_mixed_ia(1930, 2296, 0.5, A::sqrt(s.ad_value(2306)), 1.0, 1.0);
            s.store_add_scaled_product_right_ad(1931, 2306, 1.0, 2296, A::sqrt(s.ad_value(2306)), 1.0);
            s.store_add_scaled_inputs_product_mixed_aiai(2309, A::div_scaled_inputs2(s.ad_value(2308), 1.0, s.ad_value(1931), (-1.0), s.ad_value(1930), 1.0), 1.0, 2306, 0.5, A::offset(s.ad_value(189), 1.0), 2307, (-1.0));
            s.store_offset_scaled(2310, 2306, 0.5, 2.0);
            s.store_add(2311, 2306, 2307);
            s.store_sub_scaled_inputs_ad(1930, A::add_scaled_inputs_product(s.ad_value(2308), 1.0, s.ad_value(2311), (-1.0), s.ad_value(2296), A::sqrt(s.ad_value(2311)), (-1.0)), 1.0, A::ln(A::add(A::div(s.ad_value(2306), s.ad_value(2296)), A::sqrt(s.ad_value(2306)))), 2.0);
            s.store_add_scaled_inputs(2312, 1930, 2.0, 2310, 1.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1930, 2309, 0.5, 2312, 0.5, 2309, 2312, 20.0, 0.5);
            s.store_add_scaled_inputs3_indices(1931, 2308, 2.0, 2307, (-2.0), 2310, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2313, 1930, 0.5, 1931, 0.5, 1930, 1931, 20.0, (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1930, 2313, 0.5, 2310, 0.5, 2313, 2310, 5.0, (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2314, 1930, 0.5, 2310, ((-1.0) * 0.5), A::offset(A::square(A::sub_scaled_inputs(s.ad_value(1930), 1.0, s.ad_value(2310), -1.0)), 20.0), 0.5);
            s.store_mul_offset_ad_rhs(1931, 703, A::div(s.ad_value(2314), s.ad_value(2310)), 1.0);
        }

        s.b[2457] = (s.v[1931] > (-230.25850929940458));
        s.store_scalar(2457, if s.b[2457] { 1.0 } else { 0.0 });

        if (((s.b[2453] && s.b[2454]) && s.b[2456]) && s.b[2457]) {
            s.store_exp(2315, 1931);
        }

        if (((s.b[2453] && s.b[2454]) && s.b[2456]) && (!s.b[2457])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2315, 1e-100, (-230.25850929940458), 1931, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (s.b[2453] && s.b[2454]) {
            s.store_offset_mul(2316, 702, 2315, 1.0);
            s.store_mul(2317, 1916, 2316);
            s.store_mul_ad_product_rhs(2318, 197, A::offset(A::mul(s.ad_value(199), s.ad_value(819)), 1.0), A::offset(A::mul(s.ad_value(198), s.ad_value(2303)), 1.0));
            s.store_mul_offset_rhs(2319, 2317, 2318, 1.0);
            s.store_div_from_scalar(2320, 1.0, 2319);
            s.store_mul_sqrt_ad_rhs(2304, 2296, A::mul(s.ad_value(1916), s.ad_value(2320)));
            s.store_square(2305, 2304);
            s.store_div_from_scalar(2321, 1.0, 2305);
            s.store_mul(2322, 2298, 2320);
            s.store_mul(2323, 2301, 2320);
            s.store_div_scaled_value_offset_denominator(2324, s.ad_value(819), 2.0, A::sqrt_product_offset(s.ad_value(195), s.ad_value(819), 1.0), 1.0, 1.0);
            s.store_mul_ad_product_rhs_mixed_ia(2325, 194, 2324, A::offset(A::mul(s.ad_value(196), s.ad_value(2303)), 1.0));
            s.store_mul(2326, 2294, 2320);
            s.store_sqrt_square_add(1930, 2297, 2295);
            s.store_sqrt_add_ad(1931, A::square(A::sub(s.ad_value(2297), s.ad_value(2325))), s.ad_value(2295));
            s.store_mul_add_scaled_inputs3_offset_rhs(2327, 2320, s.ad_value(2325), 0.5, s.ad_value(1930), 0.5, s.ad_value(1931), ((-1.0) * (0.5)), 0.0);
            s.store_add(2328, 2326, 2322);
            s.store_sub(2329, 2328, 2327);
        }

        s.b[2458] = (p.p45 > 0.0);
        s.store_scalar(2458, if s.b[2458] { 1.0 } else { 0.0 });

        s.b[2459] = (((s.v[2329]) as f64).abs() < 1e-5);
        s.store_scalar(2459, if s.b[2459] { 1.0 } else { 0.0 });

        if (((s.b[2453] && s.b[2454]) && s.b[2458]) && s.b[2459]) {
            s.store_offset_ad(2330, A::mul_sub_from_scalar_rhs(s.ad_value(2304), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2329), 1.0, A::scale(s.ad_value(2329), 0.3125), 0.5)), 1.0);
        }

        s.b[2460] = (s.v[2329] < 460.51701859880916);
        s.store_scalar(2460, if s.b[2460] { 1.0 } else { 0.0 });

        if ((((s.b[2453] && s.b[2454]) && s.b[2458]) && (!s.b[2459])) && s.b[2460]) {
            s.store_exp_neg_input(2344, 2329);
        }

        if ((((s.b[2453] && s.b[2454]) && s.b[2458]) && (!s.b[2459])) && (!s.b[2460])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2344, 1e-200, 2329, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if (((s.b[2453] && s.b[2454]) && s.b[2458]) && (!s.b[2459])) {
            s.store_scalar(1929, (if (s.v[2329] > 0.0) { 1.0 } else { (-1.0) }));
        }

        if (((s.b[2453] && s.b[2454]) && s.b[2458]) && (!s.b[2459])) {
            s.store_offset_ad(2330, A::div_scaled_product3(s.ad_value(1929), s.ad_value(2304), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(2344), 1.0, s.ad_value(2329))), 1.0, A::sqrt(A::mul_sub_from_scalar_rhs(s.ad_value(2329), 1.0, s.ad_value(2344))), 2.0), 1.0);
        }

        if ((s.b[2453] && s.b[2454]) && (!s.b[2458])) {
            s.store_offset_div_scaled_inputs_mixed_ia(2330, 2304, 0.5, A::sqrt(s.ad_value(2329)), 1.0, 1.0);
        }

        if (s.b[2453] && s.b[2454]) {
            s.store_add_scaled_value_products(2331, s.ad_value(2329), 1.0, s.ad_value(2304), A::sqrt(s.ad_value(2329)), 1.0, s.ad_value(2330), A::ln(A::offset(s.ad_value(2330), (-1.0))), (-1.0));
            s.store_div_scaled_inputs2_indices(2332, 2323, 1.0, 2331, (-1.0), 2330, 1.0);
            s.store_mul_scaled_offset_ad_rhs(2338, 2305, 0.5, A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2305)), 1.0)), (-1.0));
            s.store_scalar(2337, 0.0);
            s.store_scalar(2339, 1.0);
        }

        s.b[2461] = (s.v[2332] > (-30.0));
        s.store_scalar(2461, if s.b[2461] { 1.0 } else { 0.0 });

        if ((s.b[2453] && s.b[2454]) && s.b[2461]) {
            s.store_offset_mul(2333, 2330, 2332, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(1929, 2333, 2333, 10.0, 0.5);
            s.store_sub_ad_rhs(2334, 2332, A::ln(s.ad_value(1929)));
            s.store_scaled_add_sqrt_square_offset_rhs(2335, 2334, 2334, 2.0, 0.5);
        }

        s.b[2462] = ((s.v[2332] - s.v[2335]) < 230.25850929940458);
        s.store_scalar(2462, if s.b[2462] { 1.0 } else { 0.0 });

        if (((s.b[2453] && s.b[2454]) && s.b[2461]) && s.b[2462]) {
            s.store_exp_sub(1929, 2332, 2335);
        }

        if (((s.b[2453] && s.b[2454]) && s.b[2461]) && (!s.b[2462])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad(1929, A::sub(s.ad_value(2332), s.ad_value(2335)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if ((s.b[2453] && s.b[2454]) && s.b[2461]) {
            s.store_div(2336, 1929, 2330);
            s.store_sub_ad_lhs(1929, A::scaled_offset(s.ad_value(2335), 1.0, 2.0), 2336);
        }

        s.b[2463] = (s.v[2336] > 1e-6);
        s.store_scalar(2463, if s.b[2463] { 1.0 } else { 0.0 });

        if (((s.b[2453] && s.b[2454]) && s.b[2461]) && s.b[2463]) {
            s.store_mul_offset_ad_rhs(2337, 2330, A::sub(s.ad_value(2335), A::div_scaled_offset_numerator(A::sqrt_product_offset(s.ad_value(2336), s.ad_value(1929), 1.0), 1.0, (-1.0), s.ad_value(2336), 1.0)), 1.0);
        }

        if (((s.b[2453] && s.b[2454]) && s.b[2461]) && (!s.b[2463])) {
            s.store_mul_ad_affine_product_rhs(2337, 2330, s.ad_value(2336), A::offset(A::mul_scaled_lhs(s.ad_value(1929), 0.25, s.ad_value(1929)), 1.0), 0.5, 0.0);
        }

        if ((s.b[2453] && s.b[2454]) && s.b[2461]) {
            s.store_add_scaled_inputs3_offset_mixed_iia(1929, 2323, 0.5, 2337, ((-1.0) * 0.5), A::sqrt_square_offset(A::offset(A::sub(s.ad_value(2323), s.ad_value(2337)), (-2.0)), 1.0), 0.5, (2.0 * 0.5));
            s.store_mul_scaled_offset_ad_rhs(2338, 2305, 0.5, A::sqrt_product_offset(A::div_from_scalar(4.0, s.ad_value(2305)), s.ad_value(1929), 1.0), (-1.0));
            s.store_div_add_scaled_inputs_rhs_indices(2339, 2338, 2338, 1.0, 2337, 1.0);
            s.store_add_scaled_product_indices(2329, 2328, 1.0, 2339, 2327, (-1.0));
        }

        if (s.b[2453] && s.b[2454]) {
            s.store_offset_scaled(2340, 2304, 0.7071067811865475, 1.0);
            s.store_scale(2341, 2340, 1e-5);
            s.store_div_from_scalar(2342, 1.0, 2340);
            s.store_scalar(2449, 0.0);
            s.store_scalar(2343, 0.0);
        }

        s.b[2464] = (s.v[2329] < 460.51701859880916);
        s.store_scalar(2464, if s.b[2464] { 1.0 } else { 0.0 });

        if ((s.b[2453] && s.b[2454]) && s.b[2464]) {
            s.store_exp_neg_input(2344, 2329);
        }

        if ((s.b[2453] && s.b[2454]) && (!s.b[2464])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2344, 1e-200, 2329, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        s.b[2465] = (((s.v[2323]) as f64).abs() <= s.v[2341]);
        s.store_scalar(2465, if s.b[2465] { 1.0 } else { 0.0 });

        if ((s.b[2453] && s.b[2454]) && s.b[2465]) {
            s.store_scaled_square(2429, 2342, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(2343, 2323, 2342, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2323), 1.0, s.ad_value(2344)), s.ad_value(2304), s.ad_value(2429)), 1.0));
        }

        s.b[2466] = (s.v[2323] < (-s.v[2341]));
        s.store_scalar(2466, if s.b[2466] { 1.0 } else { 0.0 });

        if (((s.b[2453] && s.b[2454]) && (!s.b[2465])) && s.b[2466]) {
            s.store_neg(2431, 2323);
            s.store_scaled_mul(2432, 2431, 2342, 1.25);
            s.store_scaled_sub_offset_sqrt_square_offset(2433, 2432, 10.0, (-6.0), 64.0, 0.5);
            s.store_sub(2428, 2431, 2433);
            s.store_add_scaled_square_product_mixed_iia(2434, 2428, 1.0, 2305, A::offset(s.ad_value(2433), 1.0), 1.0);
            s.store_sub_scaled_inputs(2435, 2428, 2.0, 2305, 1.0);
            s.store_sub_ln_mul_lhs(2436, 2434, 2321, 2433);
            s.store_add(813, 2434, 2435);
            s.store_add_scaled_square_product_mixed_iia(812, 813, 1.0, 2436, A::sub_scaled_inputs(A::square(s.ad_value(2435)), 0.5, s.ad_value(2434), 1.0), 1.0);
            s.store_add_ad_rhs(2437, 2433, A::div_scaled_product3(s.ad_value(2434), s.ad_value(813), s.ad_value(2436), 1.0, A::add(s.ad_value(812), A::mul3(A::mul3(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2436), s.ad_value(2436)), s.ad_value(2435), A::sub_scaled_inputs(A::square(s.ad_value(2435)), 0.3333333333333333, s.ad_value(2434), 1.0))), 1.0));
        }

        s.b[2467] = (s.v[2437] < 230.25850929940458);
        s.store_scalar(2467, if s.b[2467] { 1.0 } else { 0.0 });

        if ((((s.b[2453] && s.b[2454]) && (!s.b[2465])) && s.b[2466]) && s.b[2467]) {
            s.store_exp(2438, 2437);
        }

        if ((((s.b[2453] && s.b[2454]) && (!s.b[2465])) && s.b[2466]) && (!s.b[2467])) {
            s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2438, 2437, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        if (((s.b[2453] && s.b[2454]) && (!s.b[2465])) && s.b[2466]) {
            s.store_div_from_scalar(2439, 1.0, 2438);
            s.store_div_from_scalar_offset_square(2428, 1.0, 2437, 2.0);
            s.store_mul_square_lhs(2440, 2437, 2428);
            s.store_mul3_affine_lhs(2441, 2437, 2428, 4.0, 0.0, 2428);
            s.store_mul_ad_product_lhs_mixed_ai(2442, A::sub_scaled_inputs(s.ad_value(2428), 8.0, s.ad_value(2440), 12.0), 2428, 2428);
            s.store_sub(2428, 2431, 2437);
            s.store_mul(2429, 2344, 2439);
            s.store_add_scaled_product_right_ad(2443, 2428, 2.0, 2305, A::add_scaled_inputs3_offset(s.ad_value(2438), 1.0, s.ad_value(2429), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(2344), 1.0, s.ad_value(2441)), 1.0, (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2444, 2428, 1.0, 2305, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2438), 1.0, s.ad_value(2437), (-1.0), s.ad_value(2429), 1.0, (-1.0)), 1.0, s.ad_value(2344), A::sub(A::offset(s.ad_value(2437), (-1.0)), s.ad_value(2440)), 1.0), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2428, 2.0, 2305, A::add_scaled_inputs_product(s.ad_value(2438), 1.0, s.ad_value(2429), 1.0, s.ad_value(2344), s.ad_value(2442), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(2428, 2443, 1.0, 2444, 2428, (-2.0));
            s.store_sub_scaled_inputs_ad_rhs(2343, 2437, -1.0, A::div(s.ad_value(2444), A::add(s.ad_value(2443), A::sqrt(s.ad_value(2428)))), 2.0);
        }

        if (((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) {
            s.store_div_from_scalar_offset_scaled_input(2445, 1.0, 2304, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(2446, A::mul_scaled_lhs(s.ad_value(2340), 1.25, s.ad_value(2445)), (-1.0), 2445);
            s.store_mul_ad_product_rhs_mixed_ia(2447, 2323, 2342, A::offset(A::mul(s.ad_value(2446), s.ad_value(2323)), 1.0));
        }

        s.b[2468] = ((-s.v[2447]) > (-230.25850929940458));
        s.store_scalar(2468, if s.b[2468] { 1.0 } else { 0.0 });

        if ((((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) && s.b[2468]) {
            s.store_exp_neg_input(2428, 2447);
        }

        if ((((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) && (!s.b[2468])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2428, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2447)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) {
            s.store_sub_from_scalar(2448, 1.0, 2428);
            s.store_add_scaled_inputs_product_right_ad(2449, 2323, 1.0, 2305, 0.5, 2304, A::sqrt(A::add_scaled_inputs3(s.ad_value(2323), 1.0, s.ad_value(2305), 0.25, s.ad_value(2448), -1.0)), (-1.0));
            s.store_offset(2450, 2329, 3.0);
            s.store_sub_ad(2433, A::add_scaled_inputs3(s.ad_value(2449), 0.5, s.ad_value(2450), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2449), s.ad_value(2450)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2450), 0.5, A::sqrt_square_offset(s.ad_value(2450), 5.0), 0.5));
            s.store_sub(2428, 2323, 2433);
            s.store_exp_neg_input(2429, 2433);
            s.store_div_from_scalar_offset_square(2430, 1.0, 2433, 2.0);
            s.store_mul_square_lhs(2440, 2433, 2430);
            s.store_mul3_affine_lhs(2441, 2433, 2430, 4.0, 0.0, 2430);
            s.store_mul_ad_product_lhs_mixed_ai(2442, A::sub_scaled_inputs(s.ad_value(2430), 8.0, s.ad_value(2440), 12.0), 2430, 2430);
        }

        if (((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) {
            if (1e-40 > ((s.v[2428] * s.v[2428]) - (s.v[2305] * (((s.v[2429] + s.v[2433]) - 1.0) - (s.v[2344] * ((s.v[2433] + 1.0) + s.v[2440])))))) {
                s.store_scalar(2434, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2434, 2428, 1.0, 2305, A::add_scaled_product(A::offset(A::add(s.ad_value(2429), s.ad_value(2433)), (-1.0)), 1.0, s.ad_value(2344), A::add(A::offset(s.ad_value(2433), 1.0), s.ad_value(2440)), (-1.0)), (-1.0));
            }
        }

        if (((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) {
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2451, 1.0, 2305, A::add_scaled_product(s.ad_value(2429), 1.0, s.ad_value(2344), s.ad_value(2442), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(2435, 2428, 2.0, 2305, A::add_scaled_sub_value_product(1.0, s.ad_value(2429), 1.0, s.ad_value(2344), A::offset(s.ad_value(2441), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(2436, 2329, 1.0, 2433, (-1.0), A::ln(A::div(s.ad_value(2434), s.ad_value(2305))), 1.0);
            s.store_add(813, 2434, 2435);
        }

    }

    pub(super) fn stamp_reactive_block_15(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) {
            s.store_add_scaled_square_product_mixed_iia(812, 813, 1.0, 2436, A::add_scaled_square_product(s.ad_value(2435), 0.5, s.ad_value(2434), s.ad_value(2451), (-1.0)), 1.0);
            s.store_add_ad_rhs(2452, 2433, A::div_scaled_product3(s.ad_value(2434), s.ad_value(813), s.ad_value(2436), 1.0, A::add(s.ad_value(812), A::mul3(A::mul3(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2436), s.ad_value(2436)), s.ad_value(2435), A::add_scaled_square_product(s.ad_value(2435), 0.3333333333333333, s.ad_value(2434), s.ad_value(2451), (-1.0)))), 1.0));
        }

        s.b[2469] = (s.v[2452] < 230.25850929940458);
        s.store_scalar(2469, if s.b[2469] { 1.0 } else { 0.0 });

        if ((((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) && s.b[2469]) {
            s.store_exp(2438, 2452);
            s.store_div_from_scalar(2439, 1.0, 2438);
            s.store_mul(2438, 2344, 2438);
        }

        s.b[2470] = (s.v[2452] > (s.v[2329] - 230.25850929940458));
        s.store_scalar(2470, if s.b[2470] { 1.0 } else { 0.0 });

        if (((((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) && (!s.b[2469])) && s.b[2470]) {
            s.store_exp_sub(2438, 2452, 2329);
            s.store_div(2439, 2344, 2438);
        }

        if (((((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) && (!s.b[2469])) && (!s.b[2470])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2438, 1e-100, A::sub(s.ad_value(2329), s.ad_value(2452)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2439, 1e-100, 2452, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if (((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) {
            s.store_div_from_scalar_offset_square(2428, 1.0, 2452, 2.0);
            s.store_mul_square_lhs(2440, 2452, 2428);
            s.store_mul3_affine_lhs(2441, 2452, 2428, 4.0, 0.0, 2428);
            s.store_mul_ad_product_lhs_mixed_ai(2442, A::sub_scaled_inputs(s.ad_value(2428), 8.0, s.ad_value(2440), 12.0), 2428, 2428);
            s.store_sub(2428, 2323, 2452);
            s.store_add_scaled_product_right_ad(2443, 2428, 2.0, 2305, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2439)), 1.0, s.ad_value(2438), 1.0, s.ad_value(2344), A::offset(s.ad_value(2441), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2444, 2428, 1.0, 2305, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2439), 1.0, s.ad_value(2452), 1.0, s.ad_value(2438), 1.0, (-1.0)), 1.0, s.ad_value(2344), A::add(A::offset(s.ad_value(2452), 1.0), s.ad_value(2440)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2428, 2.0, 2305, A::add_scaled_inputs_product(s.ad_value(2439), 1.0, s.ad_value(2438), 1.0, s.ad_value(2344), s.ad_value(2442), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(2428, 2443, 1.0, 2444, 2428, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(2343, 2452, 1.0, A::div(s.ad_value(2444), A::add(s.ad_value(2443), A::sqrt(s.ad_value(2428)))), 2.0);
        }

        if (s.b[2453] && s.b[2454]) {
            s.store_scalar(2346, 0.0);
            s.store_scalar(2347, 0.0);
            s.store_scalar(2348, 0.0);
            s.store_scalar(2349, 0.0);
            s.store_scalar(2350, 0.0);
            s.store_scalar(2351, 0.0);
            s.store_scalar(2352, 0.0);
            s.store_scalar(2353, 1.0);
            s.store_scalar(2354, 1.0);
            s.store_sub(2355, 2323, 2343);
            s.store_scalar(2356, 0.0);
            s.store_mul(2357, 2319, 2355);
            s.store_scalar(2358, 1.0);
            s.store_scalar(2359, 1.0);
            s.store_scalar(2363, 1.0);
            s.store_scalar(2364, 1.0);
            s.store_scalar(2366, 1.0);
        }

        s.b[2471] = (s.v[2323] > 0.0);
        s.store_scalar(2471, if s.b[2471] { 1.0 } else { 0.0 });

        if ((s.b[2453] && s.b[2454]) && s.b[2471]) {
            s.store_div_from_scalar_offset_square(1929, 1.0, 2343, 2.0);
            s.store_mul_square_lhs(2345, 2343, 1929);
            s.store_mul3_affine_lhs(2346, 2343, 1929, 4.0, 0.0, 1929);
            s.store_mul_ad_product_lhs_mixed_ai(2347, A::sub_scaled_inputs(s.ad_value(1929), 8.0, s.ad_value(2345), 12.0), 1929, 1929);
            s.store_scalar(2348, 0.0);
        }

        s.b[2472] = (s.v[2343] < 230.25850929940458);
        s.store_scalar(2472, if s.b[2472] { 1.0 } else { 0.0 });

        if (((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2472]) {
            s.store_exp(2348, 2343);
            s.store_div_from_scalar(2349, 1.0, 2348);
            s.store_mul(2348, 2344, 2348);
        }

        s.b[2473] = (s.v[2343] > (s.v[2329] - 230.25850929940458));
        s.store_scalar(2473, if s.b[2473] { 1.0 } else { 0.0 });

        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && (!s.b[2472])) && s.b[2473]) {
            s.store_exp_sub(2348, 2343, 2329);
            s.store_div(2349, 2344, 2348);
        }

        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && (!s.b[2472])) && (!s.b[2473])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2348, 1e-100, A::sub(s.ad_value(2329), s.ad_value(2343)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2349, 1e-100, 2343, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if ((s.b[2453] && s.b[2454]) && s.b[2471]) {
            s.store_add_scaled_product_right_ad(2350, 2348, 1.0, 2344, A::add(A::offset(s.ad_value(2343), 1.0), s.ad_value(2345)), (-1.0));
        }

        s.b[2474] = (s.v[2343] < 1e-5);
        s.store_scalar(2474, if s.b[2474] { 1.0 } else { 0.0 });

        if (((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2474]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2351, 2343, 1.0, 2343, 1.0, 2343, 0.25, 0.3333333333333333, 0.5);
            s.store_mul3_ad_middle_scaled_output(2350, A::mul3(s.ad_value(2344), s.ad_value(2343), s.ad_value(2343)), 2343, A::scale_offset(s.ad_value(2343), 1.75, 1.0), 0.16666666666666666);
            s.store_sqrt_sub_from_scalar_ad(1929, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2343), 1.0, A::scale(s.ad_value(2343), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2352, 2343, 1929, 0.7071067811865475);
            s.store_offset_div_scaled_product(2353, s.ad_value(2304), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2343), 0.5)), 1.0, A::square(s.ad_value(2343)), 0.16666666666666666), 0.7071067811865475, s.ad_value(1929), 1.0, 1.0);
        }

        if (((s.b[2453] && s.b[2454]) && s.b[2471]) && (!s.b[2474])) {
            s.store_add_offset_lhs(2351, 2343, (-1.0), 2349);
            s.store_sqrt(2352, 2351);
            s.store_offset_scaled_ad(2353, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2304), 1.0, s.ad_value(2349)), s.ad_value(2352)), 0.5, 1.0);
        }

        if ((s.b[2453] && s.b[2454]) && s.b[2471]) {
            s.store_div_scaled_offset_numerator(2354, A::mul_scaled_lhs(s.ad_value(709), 0.2, s.ad_value(2303)), 1.0, 1.0, A::offset(A::mul(s.ad_value(709), s.ad_value(2303)), 1.0), 1.0);
        }

        s.b[2475] = (s.v[2350] > 1e-100);
        s.store_scalar(2475, if s.b[2475] { 1.0 } else { 0.0 });

        if (((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) {
            s.store_mul_sqrt_ad_rhs(2355, 2304, A::add(s.ad_value(2351), s.ad_value(2350)));
            s.store_div_scaled_product3_mixed_iiia(2356, 2305, 2350, 2319, 1.0, A::add_scaled_product(s.ad_value(2355), 1.0, s.ad_value(2304), s.ad_value(2352), 1.0), 1.0);
            s.store_mul3_lhs(2357, 2352, 2304, 2319);
        }

        s.b[2476] = (s.v[215] < 0.0);
        s.store_scalar(2476, if s.b[2476] { 1.0 } else { 0.0 });

        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) && s.b[2476]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2358, 1.0, 1.0, A::mul(s.ad_value(215), s.ad_value(2303)));
        }

        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) && (!s.b[2476])) {
            s.store_offset_mul(2358, 215, 2303, 1.0);
        }

        s.b[2477] = (s.v[216] < 0.0);
        s.store_scalar(2477, if s.b[2477] { 1.0 } else { 0.0 });

        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) && s.b[2477]) {
            s.store_sub_from_scalar_scaled_mul(2359, 1.0, 216, 2356, 1.0);
        }

        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) && (!s.b[2477])) {
            s.store_div_from_scalar_offset_product(2359, 1.0, 216, 2356, 1.0);
        }

        if (((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) {
            s.store_mul_product3_indices(2360, 2356, 746, 2358, 2359, 1.0);
            s.store_mul_add_scaled_product_rhs(2361, 763, s.ad_value(2357), 1.0, s.ad_value(764), s.ad_value(2356), 1.0);
            s.store_ln_ad(1930, A::div_scaled_value_offset_denominator(s.ad_value(2351), 1.0, A::add(s.ad_value(2351), s.ad_value(2350)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2362, A::pow(A::mul(s.ad_value(2361), s.ad_value(705)), s.ad_value(706)), 1.0, 707, A::exp(A::mul_scaled_lhs(s.ad_value(708), 0.5, s.ad_value(1930))), 1.0);
            s.store_mul_add_ad_lhs(2363, A::offset(s.ad_value(2362), 1.0), s.ad_value(2360), 2354);
        }

        s.b[2478] = (s.v[219] < 0.0);
        s.store_scalar(2478, if s.b[2478] { 1.0 } else { 0.0 });

        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) && s.b[2478]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2364, 1.0, 1.0, A::mul(s.ad_value(219), s.ad_value(2303)));
        }

        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) && (!s.b[2478])) {
            s.store_offset_mul(2364, 219, 2303, 1.0);
        }

        if (((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) {
            s.store_mul(1931, 2356, 2364);
            s.store_div_add_scaled_inputs_rhs_indices(2365, 1931, 221, 1.0, 1931, 1.0);
        }

        s.b[2479] = (s.v[220] < 0.0);
        s.store_scalar(2479, if s.b[2479] { 1.0 } else { 0.0 });

        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) && s.b[2479]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2366, 1.0, 1.0, A::mul(s.ad_value(220), s.ad_value(2365)));
        }

        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) && (!s.b[2479])) {
            s.store_offset_mul(2366, 220, 2365, 1.0);
        }

        if (s.b[2453] && (!s.b[2454])) {
            s.copy_ad(2301, 1806);
            s.copy_ad(2303, 1807);
            s.copy_ad(2319, 1808);
            s.copy_ad(2320, 1809);
            s.copy_ad(2304, 1810);
            s.copy_ad(2305, 1811);
            s.copy_ad(2321, 1812);
            s.copy_ad(2323, 1813);
            s.copy_ad(2328, 1814);
            s.copy_ad(2329, 1815);
            s.copy_ad(2340, 1816);
            s.copy_ad(2341, 1817);
            s.copy_ad(2342, 1818);
            s.copy_ad(2449, 1819);
            s.copy_ad(2344, 1820);
            s.copy_ad(2343, 1821);
            s.copy_ad(2346, 1822);
            s.copy_ad(2347, 1823);
            s.copy_ad(2348, 1824);
            s.copy_ad(2349, 1825);
            s.copy_ad(2351, 1826);
            s.copy_ad(2350, 1827);
            s.copy_ad(2352, 1828);
            s.copy_ad(2353, 1829);
            s.copy_ad(2354, 1830);
            s.copy_ad(2355, 1831);
            s.copy_ad(2356, 1832);
            s.copy_ad(2357, 1833);
            s.copy_ad(2358, 1834);
            s.copy_ad(2359, 1835);
            s.copy_ad(2363, 1836);
            s.copy_ad(2364, 1837);
            s.copy_ad(2366, 1838);
        }

        if s.b[2453] {
            s.copy_ad(2299, 1921);
            s.copy_ad(2300, 766);
        }

        s.b[2480] = (p.p48 != 0.0);
        s.store_scalar(2480, if s.b[2480] { 1.0 } else { 0.0 });

        if (s.b[2453] && s.b[2480]) {
            s.copy_ad(2299, 1922);
            s.copy_ad(2300, 767);
        }

        if s.b[2453] {
            s.store_scalar(2368, 0.0);
            s.store_scale(2367, 2319, 4.60517018598809);
            s.copy_ad(2384, 2367);
            s.copy_ad(2385, 815);
            s.store_mul(2386, 815, 2320);
            s.copy_ad(2390, 2343);
        }

    }

    pub(super) fn stamp_reactive_block_16(
        s: &mut ReactiveScratch,
    ) {
        if s.b[2453] {
            s.store_scalar(2391, 0.0);
            s.store_scalar(2394, 0.0);
            s.copy_ad(2396, 2349);
            s.copy_ad(2397, 2351);
            s.copy_ad(2399, 2350);
            s.copy_ad(2400, 2357);
            s.copy_ad(2401, 2343);
            s.copy_ad(2402, 2349);
            s.copy_ad(2404, 2350);
            s.copy_ad(2405, 2351);
            s.store_sub(2406, 2323, 2343);
            s.store_scalar(2407, 1.0);
            s.store_scalar(2409, 1.0);
            s.store_scalar(2408, 0.0);
            s.copy_ad(2418, 2356);
            s.store_mul(2422, 2406, 2319);
            s.store_scalar(2419, 0.0);
            s.copy_ad(2420, 2357);
            s.store_scalar(2425, 0.0);
            s.store_scalar(2424, 1.0);
            s.copy_ad(2427, 2299);
            s.copy_ad(2426, 2422);
        }

        s.b[2481] = (s.v[2323] > 0.0);
        s.store_scalar(2481, if s.b[2481] { 1.0 } else { 0.0 });

        s.b[2482] = (s.v[2350] > 1e-100);
        s.store_scalar(2482, if s.b[2482] { 1.0 } else { 0.0 });

        if ((s.b[2453] && s.b[2481]) && s.b[2482]) {
            s.store_mul(2427, 2299, 2366);
            s.store_div(2368, 2427, 2363);
            s.store_add_scaled_inputs(2369, 2355, 1.0, 2305, 0.5);
            s.store_div_scaled_product_by_product(1929, s.ad_value(2305), s.ad_value(2348), 1.0, s.ad_value(2369), s.ad_value(2369), 1.0);
        }

        s.b[2483] = (s.v[1929] > 0.0001);
        s.store_scalar(2483, if s.b[2483] { 1.0 } else { 0.0 });

        if (((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2483]) {
            s.store_sub_from_scalar(1930, 1.0, 1929);
        }

        s.b[2484] = (s.v[1930] < 1e-10);
        s.store_scalar(2484, if s.b[2484] { 1.0 } else { 0.0 });

        if ((((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2483]) && s.b[2484]) {
            s.store_scalar(1931, 1.0);
        }

        if ((((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2483]) && (!s.b[2484])) {
            s.store_sub_from_scalar_ad(1931, 1.0, A::sqrt(s.ad_value(1930)));
        }

        if (((s.b[2453] && s.b[2481]) && s.b[2482]) && (!s.b[2483])) {
            s.store_scale(1931, 1929, 0.5);
        }

        if ((s.b[2453] && s.b[2481]) && s.b[2482]) {
            s.store_mul(2370, 1931, 2369);
        }

        s.b[2485] = ((s.v[707] > 0.0) && (s.v[708] > 0.0));
        s.store_scalar(2485, if s.b[2485] { 1.0 } else { 0.0 });

        if (((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2485]) {
            s.store_scaled_mul(2371, 2319, 2370, 0.475);
            s.store_add_scaled_product_indices(1929, 2356, 1.0, 2353, 2371, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(2372, 1929, 1929, 1e-12, 0.5);
            s.store_add_scaled_value_products(2373, s.ad_value(2356), (-1.0), s.ad_value(2319), s.ad_value(2355), 1.0, A::offset(s.ad_value(2353), (-1.0)), s.ad_value(2371), 1.0);
            s.store_offset_div_scaled_product(2374, s.ad_value(2305), s.ad_value(2319), 0.5, s.ad_value(2373), 1.0, 1.0);
            s.store_add_scaled_product_indices(1929, 2373, 1.0, 764, 2372, 1.0);
            s.store_pow_ad(2375, A::mul3(s.ad_value(763), s.ad_value(1929), s.ad_value(705)), s.ad_value(706));
            s.store_mul_ad_lhs(1930, A::div_scaled_product_offset_rhs(s.ad_value(706), A::mul_sub_from_scalar_rhs(s.ad_value(2374), 1.0, s.ad_value(764)), (-1.0), 1.0, s.ad_value(1929), 1.0), 2375);
            s.store_div(1929, 2372, 2373);
            s.store_mul_pow_ad_rhs(2376, 707, A::offset(s.ad_value(1929), 1.0), A::neg(s.ad_value(708)));
            s.store_mul_div_scaled_product_mixed_iiai(1931, 2376, 708, A::add(A::offset(s.ad_value(2374), (-1.0)), A::div_scalar_offset_denominator(1.0, s.ad_value(1929), 1.0, 1.0)), 1.0, 2373, 1.0);
            s.store_mul_product3_indices(2377, 2372, 746, 2358, 2359, 1.0);
            s.store_offset_ad(1929, A::div_scaled_add_product(s.ad_value(1930), 1.0, A::mul3(s.ad_value(746), s.ad_value(2358), s.ad_value(2359)), s.ad_value(2374), (-1.0), s.ad_value(1931), 1.0), 1.0);
        }

        s.b[2486] = (s.v[1929] < 230.25850929940458);
        s.store_scalar(2486, if s.b[2486] { 1.0 } else { 0.0 });

        if ((((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2485]) && s.b[2486]) {
            s.store_scaled_ln_one_plus_exp_scaled_input(1930, 1929, 2.0, 0.5);
        }

        if ((((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2485]) && (!s.b[2486])) {
            s.copy_ad(1930, 1929);
        }

        if (((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2485]) {
            s.store_div_scaled_product3_mixed_iiia(2378, 2371, 1931, 1930, -1.0, A::add_scaled_inputs3_offset(s.ad_value(2375), 1.0, s.ad_value(2376), 1.0, s.ad_value(2377), 1.0, 1.0), 1.0);
            s.store_mul_offset_ad_rhs(2379, 2370, A::div_scaled_value_offset_denominator(s.ad_value(2378), 1.0, A::sqrt_square_offset(s.ad_value(2378), 1.0), 1.0, 1.0), 1.0);
        }

        if (((s.b[2453] && s.b[2481]) && s.b[2482]) && (!s.b[2485])) {
            s.copy_ad(2379, 2370);
        }

        if ((s.b[2453] && s.b[2481]) && s.b[2482]) {
            s.store_mul3_affine_lhs(2380, 2319, 2368, 0.7071067811865475, 0.0, 2379);
        }

        s.b[2487] = (s.v[0] == (-1.0));
        s.store_scalar(2487, if s.b[2487] { 1.0 } else { 0.0 });

        if (((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2487]) {
            s.store_div_ad_rhs(2380, 2380, A::sqrt(A::offset(s.ad_value(2380), 1.0)));
        }

        if ((s.b[2453] && s.b[2481]) && s.b[2482]) {
            s.store_div_from_scalar_offset_ad(2381, 2.0, A::sqrt(A::scale_offset(s.ad_value(2380), 4.0, 1.0)), 1.0);
            s.store_mul(1929, 2381, 2380);
            s.store_mul_ad_product_rhs_mixed_ia(2382, 2379, 2381, A::offset(A::div(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1929), 1.0, A::mul(s.ad_value(1929), s.ad_value(2381)), 0.86), A::offset(A::mul3_scaled_output(s.ad_value(1929), s.ad_value(1929), s.ad_value(2381), 4.0), 1.0)), 1.0));
            s.store_scale(2383, 2382, 0.99);
            s.store_div_scaled_product3_mixed_iaii(1929, 2383, A::sub_scaled_inputs(s.ad_value(2383), 1.0, s.ad_value(2369), 2.0), 2321, 1.0, 2350, 1.0);
        }

        if ((s.b[2453] && s.b[2481]) && s.b[2482]) {
            s.store_mul_sub_ad_rhs(2384, 2319, s.ad_value(2383), A::ln(A::offset({
                if (s.v[1929] > (-0.99)) {
                    s.ad_value(1929)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0)));
        }

        if ((s.b[2453] && s.b[2481]) && (!s.b[2482])) {
            s.copy_ad(2384, 2367);
        }

        if (s.b[2453] && s.b[2481]) {
            s.store_offset(1929, 2300, 1.0);
            s.store_div_scaled_product_left_ad(1930, A::sqrt(s.ad_value(1929)), 815, 1.0, 2384, 1.0);
            s.store_add_ad_lhs(1931, A::square(s.ad_value(1930)), 1929);
            s.store_scale(1929, 1930, 2.0);
            s.store_div_scaled_product_add_scaled_denominator(2385, 2384, 1929, 1.0, A::sqrt(A::sub(s.ad_value(1931), s.ad_value(1929))), 1.0, A::sqrt(A::add(s.ad_value(1931), s.ad_value(1929))), 1.0, 1.0);
            s.store_mul(2386, 2385, 2320);
            s.store_add(2387, 2329, 2386);
        }

        s.b[2488] = (s.v[2386] < 460.51701859880916);
        s.store_scalar(2488, if s.b[2488] { 1.0 } else { 0.0 });

        if ((s.b[2453] && s.b[2481]) && s.b[2488]) {
            s.store_exp_neg_input(2388, 2386);
        }

        if ((s.b[2453] && s.b[2481]) && (!s.b[2488])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2388, 1e-200, 2386, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if (s.b[2453] && s.b[2481]) {
            s.store_mul(2389, 2344, 2388);
        }

        s.b[2489] = (((s.v[2323]) as f64).abs() <= s.v[2341]);
        s.store_scalar(2489, if s.b[2489] { 1.0 } else { 0.0 });

        if ((s.b[2453] && s.b[2481]) && s.b[2489]) {
            s.store_scaled_square(2429, 2342, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs_mixed_ia(2390, 2323, 2342, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2323), 1.0, s.ad_value(2389)), s.ad_value(2304), s.ad_value(2429)), 1.0));
        }

        if ((s.b[2453] && s.b[2481]) && (!s.b[2489])) {
            s.store_offset(2450, 2387, 3.0);
            s.store_sub_ad(2433, A::add_scaled_inputs3(s.ad_value(2449), 0.5, s.ad_value(2450), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2449), s.ad_value(2450)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2450), 0.5, A::sqrt_square_offset(s.ad_value(2450), 5.0), 0.5));
            s.store_sub(2428, 2323, 2433);
            s.store_exp_neg_input(2429, 2433);
            s.store_div_from_scalar_offset_square(2430, 1.0, 2433, 2.0);
            s.store_mul_square_lhs(2440, 2433, 2430);
            s.store_mul3_affine_lhs(2441, 2433, 2430, 4.0, 0.0, 2430);
            s.store_mul_ad_product_lhs_mixed_ai(2442, A::sub_scaled_inputs(s.ad_value(2430), 8.0, s.ad_value(2440), 12.0), 2430, 2430);
        }

        if ((s.b[2453] && s.b[2481]) && (!s.b[2489])) {
            if (1e-40 > ((s.v[2428] * s.v[2428]) - (s.v[2305] * (((s.v[2429] + s.v[2433]) - 1.0) - (s.v[2389] * ((s.v[2433] + 1.0) + s.v[2440])))))) {
                s.store_scalar(2434, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2434, 2428, 1.0, 2305, A::add_scaled_product(A::offset(A::add(s.ad_value(2429), s.ad_value(2433)), (-1.0)), 1.0, s.ad_value(2389), A::add(A::offset(s.ad_value(2433), 1.0), s.ad_value(2440)), (-1.0)), (-1.0));
            }
        }

        if ((s.b[2453] && s.b[2481]) && (!s.b[2489])) {
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2451, 1.0, 2305, A::add_scaled_product(s.ad_value(2429), 1.0, s.ad_value(2389), s.ad_value(2442), (-1.0)), 0.5);
            s.store_add_scaled_product_right_ad(2435, 2428, 2.0, 2305, A::add_scaled_sub_value_product(1.0, s.ad_value(2429), 1.0, s.ad_value(2389), A::offset(s.ad_value(2441), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_inputs3_mixed_iia(2436, 2387, 1.0, 2433, (-1.0), A::ln(A::div(s.ad_value(2434), s.ad_value(2305))), 1.0);
            s.store_add(813, 2434, 2435);
            s.store_add_scaled_square_product_mixed_iia(812, 813, 1.0, 2436, A::add_scaled_square_product(s.ad_value(2435), 0.5, s.ad_value(2434), s.ad_value(2451), (-1.0)), 1.0);
            s.store_add_ad_rhs(2452, 2433, A::div_scaled_product3(s.ad_value(2434), s.ad_value(813), s.ad_value(2436), 1.0, A::add(s.ad_value(812), A::mul3(A::mul3(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2436), s.ad_value(2436)), s.ad_value(2435), A::add_scaled_square_product(s.ad_value(2435), 0.3333333333333333, s.ad_value(2434), s.ad_value(2451), (-1.0)))), 1.0));
        }

        s.b[2490] = (s.v[2452] < 230.25850929940458);
        s.store_scalar(2490, if s.b[2490] { 1.0 } else { 0.0 });

        if (((s.b[2453] && s.b[2481]) && (!s.b[2489])) && s.b[2490]) {
            s.store_exp(2438, 2452);
            s.store_div_from_scalar(2439, 1.0, 2438);
            s.store_mul(2438, 2389, 2438);
        }

        s.b[2491] = (s.v[2452] > (s.v[2387] - 230.25850929940458));
        s.store_scalar(2491, if s.b[2491] { 1.0 } else { 0.0 });

        if ((((s.b[2453] && s.b[2481]) && (!s.b[2489])) && (!s.b[2490])) && s.b[2491]) {
            s.store_exp_sub(2438, 2452, 2387);
            s.store_div(2439, 2389, 2438);
        }

        if ((((s.b[2453] && s.b[2481]) && (!s.b[2489])) && (!s.b[2490])) && (!s.b[2491])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2438, 1e-100, A::sub(s.ad_value(2387), s.ad_value(2452)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2439, 1e-100, 2452, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        if ((s.b[2453] && s.b[2481]) && (!s.b[2489])) {
            s.store_div_from_scalar_offset_square(2428, 1.0, 2452, 2.0);
            s.store_mul_square_lhs(2440, 2452, 2428);
            s.store_mul3_affine_lhs(2441, 2452, 2428, 4.0, 0.0, 2428);
            s.store_mul_ad_product_lhs_mixed_ai(2442, A::sub_scaled_inputs(s.ad_value(2428), 8.0, s.ad_value(2440), 12.0), 2428, 2428);
            s.store_sub(2428, 2323, 2452);
            s.store_add_scaled_product_right_ad(2443, 2428, 2.0, 2305, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2439)), 1.0, s.ad_value(2438), 1.0, s.ad_value(2389), A::offset(s.ad_value(2441), 1.0), (-1.0)), 1.0);
            s.store_add_scaled_square_product_mixed_iia(2444, 2428, 1.0, 2305, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2439), 1.0, s.ad_value(2452), 1.0, s.ad_value(2438), 1.0, (-1.0)), 1.0, s.ad_value(2389), A::add(A::offset(s.ad_value(2452), 1.0), s.ad_value(2440)), (-1.0)), (-1.0));
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2428, 2.0, 2305, A::add_scaled_inputs_product(s.ad_value(2439), 1.0, s.ad_value(2438), 1.0, s.ad_value(2389), s.ad_value(2442), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(2428, 2443, 1.0, 2444, 2428, (-2.0));
            s.store_add_scaled_inputs_ad_rhs(2390, 2452, 1.0, A::div(s.ad_value(2444), A::add(s.ad_value(2443), A::sqrt(s.ad_value(2428)))), 2.0);
        }

        if (s.b[2453] && s.b[2481]) {
            s.store_sub(2391, 2390, 2343);
        }

        s.b[2492] = (s.v[2391] < 1e-10);
        s.store_scalar(2492, if s.b[2492] { 1.0 } else { 0.0 });

        if ((s.b[2453] && s.b[2481]) && s.b[2492]) {
            s.store_add_scaled_inputs_product_right_ad(2392, 2323, 2.0, 2343, (-2.0), 2305, A::add_scaled_offset_product_rhs(A::add_scaled_sub_value_product(1.0, s.ad_value(2349), 1.0, s.ad_value(2348), s.ad_value(2388), 1.0), 1.0, s.ad_value(2389), s.ad_value(2346), 1.0, (-1.0)), 1.0);
            s.store_mul_ad_lhs(2393, A::mul_sub_from_scalar_rhs(s.ad_value(2305), 1.0, s.ad_value(2388)), 2350);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(1929, 2.0, 2305, A::add_scaled_value_products(s.ad_value(2349), 1.0, s.ad_value(2348), s.ad_value(2388), 1.0, s.ad_value(2389), s.ad_value(2347), (-1.0)), 1.0);
            s.store_add_scaled_square_product_indices(1929, 2392, 1.0, 1929, 2393, (-2.0));
            s.store_scaled_div_ad_rhs(2391, 2393, A::add(s.ad_value(2392), A::sqrt(s.ad_value(1929))), 2.0);
            s.store_add(2390, 2343, 2391);
        }

        if (s.b[2453] && s.b[2481]) {
            s.store_mul(2394, 2391, 2319);
            s.store_div_scaled_product_offset_denominator(2395, s.ad_value(2390), s.ad_value(2390), 1.0, A::square(s.ad_value(2390)), 2.0, 1.0);
        }

        s.b[2493] = (s.v[2390] < 230.25850929940458);
        s.store_scalar(2493, if s.b[2493] { 1.0 } else { 0.0 });

        if ((s.b[2453] && s.b[2481]) && s.b[2493]) {
            s.store_exp_neg_input(2396, 2390);
        }

        s.b[2494] = (s.v[2390] < 1e-5);
        s.store_scalar(2494, if s.b[2494] { 1.0 } else { 0.0 });

        if (((s.b[2453] && s.b[2481]) && s.b[2493]) && s.b[2494]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2397, 2390, 1.0, 2390, 1.0, 2390, 0.25, 0.3333333333333333, 0.5);
            s.store_sqrt_sub_from_scalar_ad(1929, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2390), 1.0, A::scale(s.ad_value(2390), 0.25), 0.3333333333333333));
        }

    }

    pub(super) fn stamp_reactive_block_17(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[2453] && s.b[2481]) && s.b[2493]) && s.b[2494]) {
            s.store_scaled_mul(2398, 2390, 1929, 0.7071067811865475);
            s.store_mul3_ad_middle(2399, A::mul3_scaled_output(s.ad_value(2389), s.ad_value(2390), s.ad_value(2390), 0.16666666666666666), 2390, A::scale_offset(s.ad_value(2390), 1.75, 1.0));
        }

        if (((s.b[2453] && s.b[2481]) && s.b[2493]) && (!s.b[2494])) {
            s.store_add_offset_lhs(2397, 2390, (-1.0), 2396);
            s.store_sqrt(2398, 2397);
            s.store_mul_add_scaled_inputs3_offset_rhs(2399, 2389, A::div_from_scalar(1.0, s.ad_value(2396)), 1.0, s.ad_value(2390), (-1.0), s.ad_value(2395), -1.0, (-1.0));
        }

        s.b[2495] = (s.v[2390] > (s.v[2387] - 230.25850929940458));
        s.store_scalar(2495, if s.b[2495] { 1.0 } else { 0.0 });

        if (((s.b[2453] && s.b[2481]) && (!s.b[2493])) && s.b[2495]) {
            s.store_exp_sub(1929, 2390, 2387);
            s.store_div(2396, 2389, 1929);
            s.store_add_scaled_product_right_ad(2399, 1929, 1.0, 2389, A::add(A::offset(s.ad_value(2390), 1.0), s.ad_value(2395)), (-1.0));
        }

        if (((s.b[2453] && s.b[2481]) && (!s.b[2493])) && (!s.b[2495])) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2396, 1e-100, 2390, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(1929, 1e-100, A::sub(s.ad_value(2387), s.ad_value(2390)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
            s.store_add_scaled_product_right_ad(2399, 1929, 1.0, 2389, A::add(A::offset(s.ad_value(2390), 1.0), s.ad_value(2395)), (-1.0));
        }

        if ((s.b[2453] && s.b[2481]) && (!s.b[2493])) {
            s.store_add_offset_lhs(2397, 2390, (-1.0), 2396);
            s.store_sqrt(2398, 2397);
        }

        if (s.b[2453] && s.b[2481]) {
            s.store_mul3_lhs(2400, 2398, 2304, 2319);
            s.store_scaled_add(2401, 2343, 2390, 0.5);
            s.store_scalar(2402, 0.0);
            s.store_mul(1929, 2396, 2349);
        }

        s.b[2496] = (s.v[1929] > 0.0);
        s.store_scalar(2496, if s.b[2496] { 1.0 } else { 0.0 });

        if ((s.b[2453] && s.b[2481]) && s.b[2496]) {
            s.store_sqrt(2402, 1929);
        }

        if (s.b[2453] && s.b[2481]) {
            s.store_scaled_add(2403, 2350, 2399, 0.5);
            s.store_add_scaled_product_mixed_iaa(2404, 2403, 1.0, A::square(s.ad_value(2391)), A::sub_scaled_inputs(s.ad_value(2402), 1.0, s.ad_value(2321), 2.0), 0.125);
        }

        s.b[2497] = (s.v[2401] < 1e-5);
        s.store_scalar(2497, if s.b[2497] { 1.0 } else { 0.0 });

        if ((s.b[2453] && s.b[2481]) && s.b[2497]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2405, 2401, 1.0, 2401, 1.0, 2401, 0.25, 0.3333333333333333, 0.5);
            s.store_mul_sqrt_ad_rhs(2406, 2304, A::add(s.ad_value(2404), s.ad_value(2405)));
        }

        s.b[2498] = (s.v[719] > 0.0);
        s.store_scalar(2498, if s.b[2498] { 1.0 } else { 0.0 });

        if (((s.b[2453] && s.b[2481]) && s.b[2497]) && s.b[2498]) {
            s.store_div_from_scalar_sqrt_ad(2407, 1.0, A::offset(A::mul(s.ad_value(719), s.ad_value(2406)), 1.0));
        }

        if ((s.b[2453] && s.b[2481]) && s.b[2497]) {
            s.store_sqrt_sub_from_scalar_ad(1929, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2401), 1.0, A::scale(s.ad_value(2401), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2408, 2401, 1929, 0.7071067811865475);
            s.store_add_ad_rhs(2409, 2407, A::div_scaled_product(s.ad_value(2304), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2401), 0.5)), 1.0, A::square(s.ad_value(2401)), 0.16666666666666666), 0.7071067811865475, s.ad_value(1929), 1.0));
        }

        if ((s.b[2453] && s.b[2481]) && (!s.b[2497])) {
            s.store_add_offset_lhs(2405, 2401, (-1.0), 2402);
            s.store_mul_sqrt_ad_rhs(2406, 2304, A::add(s.ad_value(2404), s.ad_value(2405)));
        }

        s.b[2499] = (s.v[719] > 0.0);
        s.store_scalar(2499, if s.b[2499] { 1.0 } else { 0.0 });

        if (((s.b[2453] && s.b[2481]) && (!s.b[2497])) && s.b[2499]) {
            s.store_add_scaled_sub_value_product_indices(2410, 1.0, 2402, 1.0, 2406, 2321, 2.0);
            s.store_div_from_scalar_sqrt_ad(2407, 1.0, A::offset(A::mul(s.ad_value(719), s.ad_value(2406)), 1.0));
            s.store_div_scaled_value_offset_denominator(1929, s.ad_value(2407), 1.0, s.ad_value(2407), 1.0, 1.0);
            s.store_mul_product3_mixed_iaii(2411, 719, A::square(s.ad_value(1929)), 2305, 2404, 1.0);
            s.store_add_scaled_inputs_product_right_ad(2412, 2406, 2.0, 2411, (-2.0), 2305, A::add(A::sub_from_scalar(1.0, s.ad_value(2402)), s.ad_value(2404)), 1.0);
            s.store_mul_sub_scaled_inputs_rhs(2413, 2411, s.ad_value(2411), 1.0, s.ad_value(2406), 2.0);
            s.store_sub_from_scalar_scaled_mul_ad_rhs(2414, 1.0, 2305, A::add(s.ad_value(2402), s.ad_value(2404)), 0.5);
            s.store_div_scaled_product_denominator_ad(2415, 2413, 2412, 1.0, A::add_scaled_square_product(s.ad_value(2412), 1.0, s.ad_value(2414), s.ad_value(2413), (-1.0)), 1.0);
            s.store_add(2401, 2401, 2415);
            s.store_exp(2416, 2415);
            s.store_div(2402, 2402, 2416);
            s.store_mul(2404, 2404, 2416);
            s.store_add_offset_lhs(2405, 2401, (-1.0), 2402);
            s.store_mul_sqrt_ad_rhs(2406, 2304, A::add(s.ad_value(2404), s.ad_value(2405)));
            s.store_add_ad(2417, A::sub_from_scalar(1.0, s.ad_value(2402)), A::mul3_scaled_output(s.ad_value(2406), s.ad_value(2407), s.ad_value(2321), 2.0));
            s.store_div_scaled_product3_mixed_iiaa(2391, 2391, 2416, A::add(s.ad_value(2410), s.ad_value(2403)), 1.0, A::add_scaled_product(s.ad_value(2417), 1.0, s.ad_value(2416), s.ad_value(2403), 1.0), 1.0);
            s.store_mul(2394, 2391, 2319);
        }

        if ((s.b[2453] && s.b[2481]) && (!s.b[2497])) {
            s.store_sqrt(2408, 2405);
            s.store_add_scaled_inputs_ad_rhs(2409, 2407, 1.0, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2304), 1.0, s.ad_value(2402)), s.ad_value(2408)), 0.5);
        }

        if (s.b[2453] && s.b[2481]) {
            s.store_mul_div_scaled_product_mixed_iiia(2418, 2319, 2305, 2404, 1.0, A::add_scaled_product(s.ad_value(2406), 1.0, s.ad_value(2304), s.ad_value(2408), 1.0), 1.0);
            s.store_add_scaled_product_indices(2419, 2418, 1.0, 2319, 2409, 1.0);
            s.store_mul3_lhs(2420, 2408, 2304, 2319);
        }

        s.b[2500] = (s.v[216] < 0.0);
        s.store_scalar(2500, if s.b[2500] { 1.0 } else { 0.0 });

        if ((s.b[2453] && s.b[2481]) && s.b[2500]) {
            s.store_sub_from_scalar_scaled_mul(2359, 1.0, 216, 2418, 1.0);
        }

        if ((s.b[2453] && s.b[2481]) && (!s.b[2500])) {
            s.store_div_from_scalar_offset_product(2359, 1.0, 216, 2418, 1.0);
        }

        if (s.b[2453] && s.b[2481]) {
            s.store_mul_product3_indices(2360, 2418, 746, 2358, 2359, 1.0);
            s.store_add_scaled_product_indices(2421, 2420, 1.0, 764, 2418, 1.0);
            s.store_add_scaled_product_indices(2422, 2420, 1.0, 765, 2418, 1.0);
            s.store_mul(2423, 763, 2421);
            s.store_ln_ad(1930, A::div_scaled_value_offset_denominator(s.ad_value(2405), 1.0, A::add(s.ad_value(2405), s.ad_value(2404)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2362, A::pow(A::mul(s.ad_value(2423), s.ad_value(705)), s.ad_value(706)), 1.0, 707, A::exp(A::mul_scaled_lhs(s.ad_value(708), 0.5, s.ad_value(1930))), 1.0);
            s.store_mul_add_ad_lhs(2424, A::offset(s.ad_value(2362), 1.0), s.ad_value(2360), 2354);
            s.store_ln_ad(2425, A::div_scaled_offset_numerator(A::mul(A::sub(s.ad_value(815), s.ad_value(2394)), s.ad_value(768)), 1.0, 1.0, A::offset(A::mul(A::sub(s.ad_value(2385), s.ad_value(2394)), s.ad_value(768)), 1.0), 1.0));
            s.store_mul(1931, 2418, 2364);
            s.store_div_add_scaled_inputs_rhs_indices(2365, 1931, 221, 1.0, 1931, 1.0);
        }

        s.b[2501] = (s.v[220] < 0.0);
        s.store_scalar(2501, if s.b[2501] { 1.0 } else { 0.0 });

        if ((s.b[2453] && s.b[2481]) && s.b[2501]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2366, 1.0, 1.0, A::mul(s.ad_value(220), s.ad_value(2365)));
        }

        if ((s.b[2453] && s.b[2481]) && (!s.b[2501])) {
            s.store_offset_mul(2366, 220, 2365, 1.0);
        }

        if (s.b[2453] && s.b[2481]) {
            s.store_mul(2427, 2299, 2366);
            s.store_mul(2426, 2406, 2319);
        }

        if s.b[2453] {
            s.copy_ad(1871, 2301);
            s.copy_ad(1872, 2319);
            s.copy_ad(1873, 2304);
            s.copy_ad(1874, 2323);
            s.copy_ad(1875, 2328);
            s.copy_ad(1876, 2357);
            s.copy_ad(1877, 2394);
            s.copy_ad(1878, 2400);
            s.copy_ad(1879, 2407);
            s.copy_ad(1880, 2409);
            s.copy_ad(1881, 2418);
            s.copy_ad(1882, 2419);
            s.copy_ad(1883, 2422);
            s.copy_ad(1884, 2424);
            s.copy_ad(1885, 2425);
            s.copy_ad(1886, 2427);
            s.copy_ad(1887, 2426);
        }

        if (!s.b[2453]) {
            s.copy_ad(734, 717);
            s.copy_ad(1871, 1806);
            s.copy_ad(1872, 1808);
            s.copy_ad(1873, 1810);
            s.copy_ad(1874, 1813);
            s.copy_ad(1875, 1814);
            s.copy_ad(1876, 1833);
            s.copy_ad(1877, 1844);
            s.copy_ad(1878, 1845);
            s.copy_ad(1879, 1847);
            s.copy_ad(1880, 1848);
            s.copy_ad(1881, 1849);
            s.copy_ad(1882, 1850);
            s.copy_ad(1883, 1852);
            s.copy_ad(1884, 1853);
            s.copy_ad(1885, 1855);
            s.copy_ad(1886, 1854);
            s.copy_ad(1887, 1856);
        }

        s.copy_ad(1888, 253);

        s.b[2502] = (s.v[762] > 0.0);
        s.store_scalar(2502, if s.b[2502] { 1.0 } else { 0.0 });

        if s.b[2502] {
            s.store_div_scaled_value_offset_denominator(1888, s.ad_value(253), 1.0, A::mul(s.ad_value(762), A::powf(A::add(A::square(s.ad_value(1883)), s.ad_value(722)), ((-1.0) * 0.16666666666666666))), 1.0, 1.0);
        }

        s.store_scalar(1889, 1.0);

        s.store_scalar(1890, 1.0);

        s.store_scalar(1891, 0.0);

        s.store_scalar(1892, 1.0);

        s.store_scalar(1893, 1.0);

        s.copy_ad(2265, 1887);

        s.store_scalar(2268, 0.0);

        s.store_scalar(2267, 0.0);

        s.copy_ad(2269, 2265);

        s.b[2503] = (s.v[1874] > 0.0);
        s.store_scalar(2503, if s.b[2503] { 1.0 } else { 0.0 });

        if s.b[2503] {
            s.store_mul_div_scaled_product_mixed_iaii(2260, 1885, A::add(s.ad_value(258), A::div(s.ad_value(259), s.ad_value(1882))), 1881, 1.0, 1882, 1.0);
        }

        s.b[2504] = (s.v[2260] > 0.0);
        s.store_scalar(2504, if s.b[2504] { 1.0 } else { 0.0 });

        if (s.b[2503] && s.b[2504]) {
            s.store_div_from_scalar_add_ad(1889, 1.0, A::offset(s.ad_value(2260), 1.0), A::square(s.ad_value(2260)));
        }

        if (s.b[2503] && (!s.b[2504])) {
            s.store_sub_from_scalar(1889, 1.0, 2260);
        }

        if s.b[2503] {
            s.store_mul(1890, 1884, 1889);
            s.store_div(1891, 1886, 1890);
            s.store_mul_ad_product_lhs_mixed_ai(2261, A::square(s.ad_value(1891)), 1877, 1877);
        }

        s.b[2505] = (s.v[0] == (-1.0));
        s.store_scalar(2505, if s.b[2505] { 1.0 } else { 0.0 });

        if (s.b[2503] && s.b[2505]) {
            s.store_div_scaled_value_offset_denominator(2261, s.ad_value(2261), 1.0, A::mul(s.ad_value(1891), s.ad_value(1877)), 1.0, 1.0);
        }

        if s.b[2503] {
            s.store_mul_offset_rhs_scaled_ad_rhs(1892, 1890, A::sqrt(A::scale_offset(s.ad_value(2261), 2.0, 1.0)), 1.0, 0.5);
            s.store_div(1929, 1890, 1892);
            s.store_mul_offset_ad_rhs(2262, 1880, A::mul3_scaled_output(s.ad_value(2261), s.ad_value(1929), s.ad_value(1929), 0.5), 1.0);
            s.store_div_scaled_product_indices(1893, 1929, 1882, 1.0, 2262, 1.0);
            s.store_scaled_div(2263, 1877, 1893, 0.5);
        }

    }

    pub(super) fn stamp_reactive_block_18(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[2503] {
            s.store_square(2264, 2263);
            s.store_add_product3_rhs_mixed_iia(2265, 1887, 1879, 1877, A::add(A::offset(A::mul_scaled_output(s.ad_value(2263), s.ad_value(1889), 0.3333333333333333), (-1.0)), s.ad_value(1889)), 0.5);
            s.store_scaled_mul(1929, 1880, 1877, 0.16666666666666666);
        }

        s.b[2506] = (p.p49 == 1.0);
        s.store_scalar(2506, if s.b[2506] { 1.0 } else { 0.0 });

        if (s.b[2503] && s.b[2506]) {
            s.store_scalar(2266, 0.0);
            s.store_mul_ad_affine_product_rhs(2267, 1889, s.ad_value(1889), A::sub(s.ad_value(1881), A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1929), 2.0, s.ad_value(2263), 3.0)), 0.5, 0.0);
        }

        if (s.b[2503] && (!s.b[2506])) {
            s.store_mul_sub_from_scalar_lhs_ad_rhs(2266, 1.0, 1889, A::add_scaled_product(s.ad_value(1881), 1.0, s.ad_value(1880), s.ad_value(1877), (-0.5)));
            s.store_add_scaled_products_mixed_aaia(2267, A::square(s.ad_value(1889)), A::add_scaled_product(s.ad_value(1881), 1.0, s.ad_value(1929), A::sub_scaled_inputs(A::sub_from_scalar(1.0, s.ad_value(2263)), 1.0, s.ad_value(2264), 0.2), (-1.0)), 0.5, 2266, A::offset(s.ad_value(1889), 1.0), 0.5);
        }

        if s.b[2503] {
            s.store_add_scaled_product_right_ad(2268, 2266, 1.0, 1889, A::add_scaled_product(s.ad_value(1881), 1.0, s.ad_value(1929), s.ad_value(2263), 1.0), 1.0);
            s.store_sub(2269, 2265, 2268);
        }

        s.store_mul(840, 2265, 1888);

        s.store_mul_neg_lhs(842, 2267, 1888);

        s.store_mul_neg_lhs(841, 2269, 1888);

        s.store_scalar(2285, 0.0);

        s.store_scalar(2286, 0.0);

        s.store_scalar(2284, 0.0);

        s.b[2507] = ((s.v[266] > 0.0) || (s.v[267] > 0.0));
        s.store_scalar(2507, if s.b[2507] { 1.0 } else { 0.0 });

        if s.b[2507] {
            s.store_scalar(2274, 1.0);
            s.copy_ad(2273, 1871);
        }

        s.b[2508] = (s.v[270] > 1e-10);
        s.store_scalar(2508, if s.b[2508] { 1.0 } else { 0.0 });

        if (s.b[2507] && s.b[2508]) {
            s.store_add_scaled_inputs3_indices(2270, 1871, 1.0, 268, (-1.0), 797, 1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1929, 2270, 0.5, 797, 0.5, A::add(A::square(A::sub(s.ad_value(2270), s.ad_value(797))), s.ad_value(798)), 0.5);
            s.store_mul_add_scaled_inputs3_offset_rhs(1930, 1929, s.ad_value(1929), 2.0, s.ad_value(797), (-1.0), s.ad_value(2270), -1.0, 0.0);
            s.store_div(1931, 797, 1929);
            s.store_mul(2271, 2270, 1931);
            s.store_sqrt_sub_from_scalar_ad(2272, 1.0, A::mul(s.ad_value(2271), s.ad_value(270)));
            s.store_add_scaled_inputs3_mixed_aii(2273, A::div(A::sub_from_scalar(1.0, s.ad_value(2272)), s.ad_value(270)), 1.0, 2270, 1.0, 2271, -1.0);
            s.store_offset_ad(2274, A::div_scaled_product3(A::offset(A::div_from_scalar(0.5, s.ad_value(2272)), (-1.0)), A::add_scaled_product(s.ad_value(1930), 1.0, s.ad_value(2270), A::sub(s.ad_value(797), s.ad_value(1929)), 1.0), s.ad_value(1931), 1.0, s.ad_value(1930), 1.0), 1.0);
        }

        if s.b[2507] {
            s.store_scalar(2276, 1.0);
            s.store_scalar(2277, 0.0);
        }

        s.b[2509] = (s.v[269] > 0.0);
        s.store_scalar(2509, if s.b[2509] { 1.0 } else { 0.0 });

        if (s.b[2507] && s.b[2509]) {
            s.store_add_scaled_product_right_ad(1929, 734, 0.5, 1872, A::scale_offset(s.ad_value(1873), 0.7071067811865475, 1.0), 1.0);
            s.store_div(2275, 1871, 1929);
        }

        s.b[2510] = (((s.v[2275]) as f64).abs() < 230.25850929940458);
        s.store_scalar(2510, if s.b[2510] { 1.0 } else { 0.0 });

        if ((s.b[2507] && s.b[2509]) && s.b[2510]) {
            s.store_div_from_scalar_offset_ad(2276, 1.0, A::exp_scaled_input(s.ad_value(2275), -1.0), 1.0);
        }

        s.b[2511] = (s.v[2275] < 0.0);
        s.store_scalar(2511, if s.b[2511] { 1.0 } else { 0.0 });

        if (((s.b[2507] && s.b[2509]) && (!s.b[2510])) && s.b[2511]) {
            s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2276, 1e-100, 2275, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);
        }

        s.b[2512] = (s.v[2275] < 230.25850929940458);
        s.store_scalar(2512, if s.b[2512] { 1.0 } else { 0.0 });

        if ((s.b[2507] && s.b[2509]) && s.b[2512]) {
            s.store_ln_one_plus_exp(1930, 2275);
        }

        if ((s.b[2507] && s.b[2509]) && (!s.b[2512])) {
            s.copy_ad(1930, 2275);
        }

        if (s.b[2507] && s.b[2509]) {
            s.store_mul(2277, 1929, 1930);
        }

        if s.b[2507] {
            s.store_add_scaled_product_right_sub(2278, 2274, 1.0, 269, 2276, 2274, 1.0);
            s.store_add_scaled_product_right_sub(2279, 2273, 1.0, 269, 2277, 2273, 1.0);
            s.store_add_scaled_inputs3_mixed_aii(2280, A::add_scaled_product(s.ad_value(1871), 1.0, s.ad_value(1872), s.ad_value(1875), (-1.0)), 1.0, 1887, (-1.0), 1877, (-0.5));
            s.store_add_scaled_inputs3_indices(2281, 1871, 1.0, 2280, (-1.0), 1876, -1.0);
            s.store_add_scaled_inputs3_indices(2282, 1877, 1.0, 2280, 1.0, 815, -1.0);
            s.store_add_scaled_inputs3_indices(2283, 1871, 1.0, 2282, (-1.0), 1878, -1.0);
        }

        s.b[2513] = (s.v[820] > 0.0);
        s.store_scalar(2513, if s.b[2513] { 1.0 } else { 0.0 });

        if (s.b[2507] && s.b[2513]) {
            s.store_mul_ad_rhs(2284, 2278, A::add_scaled_products(s.ad_value(267), s.ad_value(2282), 1.0, s.ad_value(266), s.ad_value(2280), 1.0));
            s.store_mul_sub_rhs(2285, 266, 2281, 2279);
            s.store_mul_sub_rhs(2286, 267, 2283, 2279);
        }

        if (s.b[2507] && (!s.b[2513])) {
            s.store_mul_ad_rhs(2284, 2278, A::add_scaled_products(s.ad_value(266), s.ad_value(2282), 1.0, s.ad_value(267), s.ad_value(2280), 1.0));
            s.store_mul_sub_rhs(2285, 267, 2281, 2279);
            s.store_mul_sub_rhs(2286, 266, 2283, 2279);
        }

        if s.b[2507] {
            s.store_add(840, 840, 2284);
            s.store_add(842, 842, 2286);
            s.store_add_scaled_inputs4_indices(841, 841, 1.0, 2284, (-1.0), 2286, -1.0, 2285, -1.0);
        }

        s.store_mul(1894, 260, 1862);

        s.store_mul(1895, 261, 1863);

        s.store_scalar(2289, 0.0);

        s.store_scalar(2287, 0.0);

        s.b[2514] = ((s.v[260] > 0.0) && (s.v[262] > 0.0));
        s.store_scalar(2514, if s.b[2514] { 1.0 } else { 0.0 });

        if s.b[2514] {
            s.store_mul_add_scaled_inputs_rhs(1929, 264, s.ad_value(1803), 0.5, s.ad_value(776), 1.0);
        }

        s.b[2515] = (s.v[1929] < 230.25850929940458);
        s.store_scalar(2515, if s.b[2515] { 1.0 } else { 0.0 });

        s.b[2516] = (s.v[1929] > (-230.25850929940458));
        s.store_scalar(2516, if s.b[2516] { 1.0 } else { 0.0 });

        if ((s.b[2514] && s.b[2515]) && s.b[2516]) {
            s.store_exp(2287, 1929);
        }

        if ((s.b[2514] && s.b[2515]) && (!s.b[2516])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2287, 1e-100, (-230.25850929940458), 1929, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2517] = (s.v[2287] > 1e-10);
        s.store_scalar(2517, if s.b[2517] { 1.0 } else { 0.0 });

        if ((s.b[2514] && s.b[2515]) && s.b[2517]) {
            s.store_ln_offset_input(2288, 2287, 1.0);
            s.store_mul_sub_from_scalar_ad_rhs(1930, 2288, 1.0, A::div(A::ln(A::offset(s.ad_value(2288), 1.0)), A::offset(s.ad_value(2288), 2.0)));
        }

        if ((s.b[2514] && s.b[2515]) && (!s.b[2517])) {
            s.copy_ad(2288, 2287);
            s.store_div_scaled_value_offset_denominator(1930, s.ad_value(2288), 2.0, s.ad_value(2288), 2.0, 1.0);
        }

        if (s.b[2514] && (!s.b[2515])) {
            s.copy_ad(2288, 1929);
            s.store_mul_sub_from_scalar_ad_rhs(1930, 2288, 1.0, A::div(A::ln(A::offset(s.ad_value(2288), 1.0)), A::offset(s.ad_value(2288), 2.0)));
        }

        if s.b[2514] {
            s.store_mul_ad_affine_product_lhs(2289, A::div_scaled_inputs(s.ad_value(262), (-2.0), s.ad_value(264), 1.0), s.ad_value(260), s.v[355], 0.0, 1930);
        }

        s.store_scalar(2292, 0.0);

        s.store_scalar(2290, 0.0);

        s.b[2518] = ((s.v[261] > 0.0) && (s.v[263] > 0.0));
        s.store_scalar(2518, if s.b[2518] { 1.0 } else { 0.0 });

        if s.b[2518] {
            s.store_mul_add_scaled_inputs_rhs(1929, 264, s.ad_value(1803), 0.5, s.ad_value(777), 1.0);
        }

        s.b[2519] = (s.v[1929] < 230.25850929940458);
        s.store_scalar(2519, if s.b[2519] { 1.0 } else { 0.0 });

        s.b[2520] = (s.v[1929] > (-230.25850929940458));
        s.store_scalar(2520, if s.b[2520] { 1.0 } else { 0.0 });

        if ((s.b[2518] && s.b[2519]) && s.b[2520]) {
            s.store_exp(2290, 1929);
        }

        if ((s.b[2518] && s.b[2519]) && (!s.b[2520])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2290, 1e-100, (-230.25850929940458), 1929, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2521] = (s.v[2290] > 1e-10);
        s.store_scalar(2521, if s.b[2521] { 1.0 } else { 0.0 });

        if ((s.b[2518] && s.b[2519]) && s.b[2521]) {
            s.store_ln_offset_input(2291, 2290, 1.0);
            s.store_mul_sub_from_scalar_ad_rhs(1930, 2291, 1.0, A::div(A::ln(A::offset(s.ad_value(2291), 1.0)), A::offset(s.ad_value(2291), 2.0)));
        }

        if ((s.b[2518] && s.b[2519]) && (!s.b[2521])) {
            s.copy_ad(2291, 2290);
            s.store_div_scaled_value_offset_denominator(1930, s.ad_value(2291), 2.0, s.ad_value(2291), 2.0, 1.0);
        }

        if (s.b[2518] && (!s.b[2519])) {
            s.copy_ad(2291, 1929);
            s.store_mul_sub_from_scalar_ad_rhs(1930, 2291, 1.0, A::div(A::ln(A::offset(s.ad_value(2291), 1.0)), A::offset(s.ad_value(2291), 2.0)));
        }

        if s.b[2518] {
            s.store_mul_ad_affine_product_lhs(2292, A::div_scaled_inputs(s.ad_value(263), (-2.0), s.ad_value(264), 1.0), s.ad_value(261), s.v[355], 0.0, 1930);
        }

        s.store_add(2293, 2289, 2292);

        s.store_add_scaled_product_indices(845, 2293, 1.0, 265, 818, 1.0);

        s.store_mul(843, 272, 823);

        s.store_mul(844, 273, 826);

        s.store_scalar(2522, 0.0);

        s.store_scalar(2525, 0.0);

        s.store_scalar(2526, 0.0);

        s.store_scalar(2527, 0.0);

        s.store_scalar(2528, 0.0);

        s.store_scalar(2529, 0.0);

        s.store_scalar(2530, 0.0);

        s.store_scalar(2531, 0.0);

        s.store_scalar(2532, 0.0);

        s.store_scalar(2533, 0.0);

        s.store_scalar(2534, 0.0);

        s.store_scalar(2535, 0.0);

        s.store_scalar(2536, 0.0);

        s.store_scalar(2537, 0.0);

        s.store_scalar(2538, 0.0);

        s.store_scalar(2539, 0.0);

        s.store_scalar(2540, 0.0);

        s.store_scalar(2543, 0.0);

        s.store_scalar(2547, 0.0);

        s.store_scalar(2550, 0.0);

        s.store_scalar(2551, 0.0);

        s.store_scalar(2552, 0.0);

        s.store_scalar(2553, 0.0);

        s.store_scalar(2554, 0.0);

        s.store_scalar(2555, 0.0);

        s.store_scalar(2558, 0.0);

        s.store_scalar(2559, 0.0);

        s.store_scalar(2560, 0.0);

        s.store_scalar(2561, 0.0);

        s.store_scalar(2565, 0.0);

        s.store_scalar(2567, 0.0);

        s.store_scalar(2568, 0.0);

        s.store_scalar(846, 0.0);

        s.store_scalar(1902, 0.0);

        s.store_scalar(1903, 0.0);

        s.store_scalar(1904, 0.0);

        s.store_scalar(847, 0.0);

        s.store_scalar(1905, 0.0);

        s.store_scalar(1906, 0.0);

        s.store_scalar(1907, 0.0);

        s.b[2569] = (p.p43 > 0.0);
        s.store_scalar(2569, if s.b[2569] { 1.0 } else { 0.0 });

        s.b[2570] = (s.v[475] == 1.0);
        s.store_scalar(2570, if s.b[2570] { 1.0 } else { 0.0 });

        if (s.b[2569] && s.b[2570]) {
            s.store_scalar(2573, 0.0);
            s.store_scalar(2574, 0.0);
            s.store_scaled_mul(2525, 658, 658, 4.0);
            s.store_div(2526, 658, 659);
            s.store_add_scaled_product_indices(2527, 821, 1.0, 658, 2526, 1.0);
            s.store_add(2528, 659, 2527);
            s.store_sub(2529, 659, 2527);
            s.store_sqrt_square_add(2530, 2529, 2525);
            s.store_div_scaled_product_add_scaled_denominator_indices(2574, 821, 659, 2.0, 2528, 1.0, 2530, 1.0, 1.0);
        }

        s.b[2575] = (s.v[652] > 0.5);
        s.store_scalar(2575, if s.b[2575] { 1.0 } else { 0.0 });

        s.b[2576] = (s.v[409] == 0.5);
        s.store_scalar(2576, if s.b[2576] { 1.0 } else { 0.0 });

        if (((s.b[2569] && s.b[2570]) && s.b[2575]) && s.b[2576]) {
            s.store_sqrt_sub_from_scalar_ad(2573, 1.0, A::scale(s.ad_value(2574), s.v[406]));
        }

        if (((s.b[2569] && s.b[2570]) && s.b[2575]) && (!s.b[2576])) {
            s.store_powf_ad(2573, A::sub_from_scalar(1.0, A::scale(s.ad_value(2574), s.v[406])), s.v[409]);
        }

        if ((s.b[2569] && s.b[2570]) && s.b[2575]) {
            s.store_add_scaled_inputs3_offset_indices(1902, 2573, (-s.v[418]), 821, s.v[421], 2574, (-s.v[421]), s.v[418]);
        }

        s.b[2577] = (s.v[653] > 0.5);
        s.store_scalar(2577, if s.b[2577] { 1.0 } else { 0.0 });

        s.b[2578] = (s.v[410] == 0.5);
        s.store_scalar(2578, if s.b[2578] { 1.0 } else { 0.0 });

        if (((s.b[2569] && s.b[2570]) && s.b[2577]) && s.b[2578]) {
            s.store_sqrt_sub_from_scalar_ad(2573, 1.0, A::scale(s.ad_value(2574), s.v[407]));
        }

        if (((s.b[2569] && s.b[2570]) && s.b[2577]) && (!s.b[2578])) {
            s.store_powf_ad(2573, A::sub_from_scalar(1.0, A::scale(s.ad_value(2574), s.v[407])), s.v[410]);
        }

        if ((s.b[2569] && s.b[2570]) && s.b[2577]) {
            s.store_add_scaled_inputs3_offset_indices(1903, 2573, (-s.v[419]), 821, s.v[422], 2574, (-s.v[422]), s.v[419]);
        }

        s.b[2579] = (s.v[654] > 0.5);
        s.store_scalar(2579, if s.b[2579] { 1.0 } else { 0.0 });

        s.b[2580] = (s.v[411] == 0.5);
        s.store_scalar(2580, if s.b[2580] { 1.0 } else { 0.0 });

        if (((s.b[2569] && s.b[2570]) && s.b[2579]) && s.b[2580]) {
            s.store_sqrt_sub_from_scalar_ad(2573, 1.0, A::scale(s.ad_value(2574), s.v[408]));
        }

        if (((s.b[2569] && s.b[2570]) && s.b[2579]) && (!s.b[2580])) {
            s.store_powf_ad(2573, A::sub_from_scalar(1.0, A::scale(s.ad_value(2574), s.v[408])), s.v[411]);
        }

        if ((s.b[2569] && s.b[2570]) && s.b[2579]) {
            s.store_add_scaled_inputs3_offset_indices(1904, 2573, (-s.v[420]), 821, s.v[423], 2574, (-s.v[423]), s.v[420]);
        }

        if (s.b[2569] && s.b[2570]) {
            s.store_scalar(2573, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[2569] && s.b[2570]) {
            s.store_scalar(2574, 0.0);
            s.store_scaled_mul(2525, 685, 685, 4.0);
            s.store_div(2526, 685, 686);
            s.store_add_scaled_product_indices(2527, 822, 1.0, 685, 2526, 1.0);
            s.store_add(2528, 686, 2527);
            s.store_sub(2529, 686, 2527);
            s.store_sqrt_square_add(2530, 2529, 2525);
            s.store_div_scaled_product_add_scaled_denominator_indices(2574, 822, 686, 2.0, 2528, 1.0, 2530, 1.0, 1.0);
        }

        s.b[2581] = (s.v[679] > 0.5);
        s.store_scalar(2581, if s.b[2581] { 1.0 } else { 0.0 });

        s.b[2582] = (s.v[576] == 0.5);
        s.store_scalar(2582, if s.b[2582] { 1.0 } else { 0.0 });

        if (((s.b[2569] && s.b[2570]) && s.b[2581]) && s.b[2582]) {
            s.store_sqrt_sub_from_scalar_ad(2573, 1.0, A::mul(s.ad_value(2574), s.ad_value(573)));
        }

        if (((s.b[2569] && s.b[2570]) && s.b[2581]) && (!s.b[2582])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2573, 1.0, 2574, 573, 576);
        }

        if ((s.b[2569] && s.b[2570]) && s.b[2581]) {
            s.store_add_scaled_product_mixed_aia(1905, A::mul_sub_from_scalar_rhs(s.ad_value(585), 1.0, s.ad_value(2573)), 1.0, 588, A::sub(s.ad_value(822), s.ad_value(2574)), 1.0);
        }

        s.b[2583] = (s.v[680] > 0.5);
        s.store_scalar(2583, if s.b[2583] { 1.0 } else { 0.0 });

        s.b[2584] = (s.v[577] == 0.5);
        s.store_scalar(2584, if s.b[2584] { 1.0 } else { 0.0 });

        if (((s.b[2569] && s.b[2570]) && s.b[2583]) && s.b[2584]) {
            s.store_sqrt_sub_from_scalar_ad(2573, 1.0, A::mul(s.ad_value(2574), s.ad_value(574)));
        }

        if (((s.b[2569] && s.b[2570]) && s.b[2583]) && (!s.b[2584])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2573, 1.0, 2574, 574, 577);
        }

        if ((s.b[2569] && s.b[2570]) && s.b[2583]) {
            s.store_add_scaled_product_mixed_aia(1906, A::mul_sub_from_scalar_rhs(s.ad_value(586), 1.0, s.ad_value(2573)), 1.0, 589, A::sub(s.ad_value(822), s.ad_value(2574)), 1.0);
        }

        s.b[2585] = (s.v[681] > 0.5);
        s.store_scalar(2585, if s.b[2585] { 1.0 } else { 0.0 });

        s.b[2586] = (s.v[578] == 0.5);
        s.store_scalar(2586, if s.b[2586] { 1.0 } else { 0.0 });

        if (((s.b[2569] && s.b[2570]) && s.b[2585]) && s.b[2586]) {
            s.store_sqrt_sub_from_scalar_ad(2573, 1.0, A::mul(s.ad_value(2574), s.ad_value(575)));
        }

        if (((s.b[2569] && s.b[2570]) && s.b[2585]) && (!s.b[2586])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2573, 1.0, 2574, 575, 578);
        }

        if ((s.b[2569] && s.b[2570]) && s.b[2585]) {
            s.store_add_scaled_product_mixed_aia(1907, A::mul_sub_from_scalar_rhs(s.ad_value(587), 1.0, s.ad_value(2573)), 1.0, 590, A::sub(s.ad_value(822), s.ad_value(2574)), 1.0);
        }

        s.b[2587] = (p.p889 > 0.0);
        s.store_scalar(2587, if s.b[2587] { 1.0 } else { 0.0 });

        if ((s.b[2569] && (!s.b[2570])) && s.b[2587]) {
            s.store_scaled_offset_ad(643, A::powf(A::add_scaled_inputs3(s.ad_value(814), 0.5, s.ad_value(816), 0.5, A::sqrt_square_offset(A::add(s.ad_value(814), s.ad_value(816)), (0.001 * 0.001)), 0.5), p.p890), (-(((0.5 * 0.001)) as f64).powf(p.p890)), p.p889);
            s.store_offset(641, 643, p.p879);
            s.store_div_from_scalar(451, 1.0, 641);
        }

        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2587])) {
            s.store_scalar(641, p.p879);
        }

        s.b[2588] = (p.p891 > 0.0);
        s.store_scalar(2588, if s.b[2588] { 1.0 } else { 0.0 });

        if ((s.b[2569] && (!s.b[2570])) && s.b[2588]) {
            s.store_scaled_offset_ad(645, A::powf(A::add_scaled_inputs3(s.ad_value(814), 0.5, s.ad_value(816), 0.5, A::sqrt_square_offset(A::add(s.ad_value(814), s.ad_value(816)), (0.001 * 0.001)), 0.5), p.p892), (-(((0.5 * 0.001)) as f64).powf(p.p892)), p.p891);
            s.store_mul_offset_rhs(444, 444, 645, 1.0);
        }

        if (s.b[2569] && (!s.b[2570])) {
            s.store_scalar(2538, 0.0);
            s.store_scalar(2535, 0.0);
        }

        s.b[2589] = (!(((s.v[647] == 0.0) && (s.v[648] == 0.0)) && (s.v[649] == 0.0)));
        s.store_scalar(2589, if s.b[2589] { 1.0 } else { 0.0 });

        if ((s.b[2569] && (!s.b[2570])) && s.b[2589]) {
            s.store_scaled_mul(2525, 658, 658, 4.0);
            s.store_div(2526, 658, 659);
            s.store_add_scaled_product_indices(2527, 821, 1.0, 658, 2526, 1.0);
            s.store_add(2528, 659, 2527);
            s.store_sub(2529, 659, 2527);
            s.store_sqrt_square_add(2530, 2529, 2525);
            s.store_div_scaled_product_add_scaled_denominator_indices(2532, 821, 659, 2.0, 2528, 1.0, 2530, 1.0, 1.0);
        }

        s.b[2590] = (s.v[821] < s.v[655]);
        s.store_scalar(2590, if s.b[2590] { 1.0 } else { 0.0 });

        s.b[2591] = (((((-0.5) * (s.v[821] * s.v[372]))) as f64).abs() < 230.25850929940458);
        s.store_scalar(2591, if s.b[2591] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && s.b[2589]) && s.b[2590]) && s.b[2591]) {
            s.store_exp_scaled_input(2533, 821, (s.v[372] * (-0.5)));
        }

        s.b[2592] = (((-0.5) * (s.v[821] * s.v[372])) < 0.0);
        s.store_scalar(2592, if s.b[2592] { 1.0 } else { 0.0 });

        if (((((s.b[2569] && (!s.b[2570])) && s.b[2589]) && s.b[2590]) && (!s.b[2591])) && s.b[2592]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2533, 1e-100, (-230.25850929940458), A::scale(s.ad_value(821), (s.v[372] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2569] && (!s.b[2570])) && s.b[2589]) && s.b[2590]) && (!s.b[2591])) && (!s.b[2592])) {
            s.store_scaled_offset_ad(2533, A::mul_offset_rhs(A::scale_offset(s.ad_value(821), (s.v[372] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(821), (s.v[372] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(821), (((s.v[372] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[2569] && (!s.b[2570])) && s.b[2589]) && s.b[2590]) {
            s.store_div_from_scalar(2534, 1.0, 2533);
            s.store_square(2531, 2534);
        }

        if (((s.b[2569] && (!s.b[2570])) && s.b[2589]) && (!s.b[2590])) {
            s.store_mul_offset_ad_lhs(2531, A::sub_scaled_inputs(s.ad_value(821), s.v[372], s.ad_value(655), s.v[372]), 1.0, 656);
            s.store_sqrt(2534, 2531);
            s.store_div_from_scalar(2533, 1.0, 2534);
        }

        if ((s.b[2569] && (!s.b[2570])) && s.b[2589]) {
            s.store_offset(2531, 2531, (-1.0));
        }

        s.b[2593] = (s.v[821] > 0.0);
        s.store_scalar(2593, if s.b[2593] { 1.0 } else { 0.0 });

        if (((s.b[2569] && (!s.b[2570])) && s.b[2589]) && s.b[2593]) {
            s.store_scaled_ln_ad(2535, A::add(A::offset(s.ad_value(2533), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2533), 1.0, A::offset(s.ad_value(2533), 3.0)))), (s.v[371] * 2.0));
        }

        if (((s.b[2569] && (!s.b[2570])) && s.b[2589]) && (!s.b[2593])) {
            s.store_sub_ad_lhs(2535, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(2534), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2534), 1.0, A::scale_offset(s.ad_value(2534), 3.0, 1.0))))), (s.v[371] * 2.0)), 821);
        }

        if ((s.b[2569] && (!s.b[2570])) && s.b[2589]) {
            s.store_sub(2536, 657, 2535);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2537, 821, 0.5, 2536, 0.5, 821, 2536, ((4.0 * s.v[371]) * s.v[371]), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2538, 821, 0.5, 660, 0.5, 821, 660, ((4.0 * s.v[369]) * s.v[369]), (-0.5));
            s.store_scaled_sub_sqrt_square_offset_rhs(2539, 821, 821, ((4.0 * 1e-6) * 1e-6), 0.5);
        }

        s.b[2594] = (s.v[647] == 0.0);
        s.store_scalar(2594, if s.b[2594] { 1.0 } else { 0.0 });

        if ((s.b[2569] && (!s.b[2570])) && s.b[2594]) {
            s.store_scalar(1902, 0.0);
        }

        s.b[2595] = ((p.p857 == 0.0) && (p.p862 == 0.0));
        s.store_scalar(2595, if s.b[2595] { 1.0 } else { 0.0 });

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2595])) {
            s.store_sub_from_scalar(2543, s.v[394], 2537);
        }

        s.b[2597] = (p.p848 == 0.5);
        s.store_scalar(2597, if s.b[2597] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2595])) && s.b[2597]) {
            s.store_sqrt_scaled_input(2540, 2543, s.v[430]);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2595])) && (!s.b[2597])) {
            s.store_powf_scaled_input(2540, 2543, s.v[430], p.p848);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2595])) {
            s.store_scale(2547, 2540, s.v[424]);
        }

        s.b[2598] = (p.p862 == 0.0);
        s.store_scalar(2598, if s.b[2598] { 1.0 } else { 0.0 });

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) {
            s.store_div_scaled_inputs_indices(2550, 2547, (s.v[409] * s.v[439]), 2543, 1.0);
            s.store_div_from_scalar(2551, (0.666666666666667 * s.v[436]), 2550);
            s.store_square(2552, 2551);
            s.store_sqrt_div_scaled_square_offset_denominator(2553, 2552, 1.0, 1.0, 1.0);
            s.store_sqrt(2554, 2553);
            s.store_mul(2555, 2553, 2554);
            s.store_sqrt_scaled_input_ad(2558, A::div(s.ad_value(2550), s.ad_value(2554)), 0.375);
            s.store_add_scaled_product_indices(2559, 2553, (-1.0), 2551, 2554, 2.0);
            s.store_add_scaled_value_products(2560, s.ad_value(2553), (-s.v[436]), s.ad_value(2551), s.ad_value(2554), s.v[436], s.ad_value(2550), s.ad_value(2555), 0.5);
            s.store_mul_offset_lhs(2561, 2559, (-1.0), 2558);
            s.store_square(2522, 2561);
        }

        s.b[2601] = (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458));
        s.store_scalar(2601, if s.b[2601] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) && s.b[2601]) {
            s.store_exp_sub(2540, 2560, 2522);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) && (!s.b[2601])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2540, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2602] = (s.v[2561] > 0.0);
        s.store_scalar(2602, if s.b[2602] { 1.0 } else { 0.0 });

        s.b[2603] = (s.v[2560] > (-230.25850929940458));
        s.store_scalar(2603, if s.b[2603] { 1.0 } else { 0.0 });

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) && (!s.b[2602])) && s.b[2603]) {
            s.store_exp(2540, 2560);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) && (!s.b[2602])) && (!s.b[2603])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 2560, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2604] = (p.p868 == 0.0);
        s.store_scalar(2604, if s.b[2604] { 1.0 } else { 0.0 });

        s.b[2605] = (p.p848 == 0.5);
        s.store_scalar(2605, if s.b[2605] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2604])) && s.b[2605]) {
            s.store_sqrt_scaled_input_ad(2540, A::sub_from_scalar(p.p845, s.ad_value(2538)), s.v[430]);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2604])) && (!s.b[2605])) {
            s.store_powf_scale_offset_input(2540, 2538, (-s.v[430]), ((p.p845) * (s.v[430])), p.p848);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2604])) {
            s.store_div_scaled_offset_numerator(2565, s.ad_value(2538), ((-s.v[427]) * s.v[412]), (((p.p845) * (s.v[427])) * s.v[412]), s.ad_value(2540), 1.0);
        }

        s.b[2606] = (((((-s.v[442]) / s.v[2565])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2606, if s.b[2606] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2604])) && s.b[2606]) {
            s.store_ad_value(2540, A::exp_div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(2565), 1.0));
        }

        s.b[2607] = (((-s.v[442]) / s.v[2565]) < 0.0);
        s.store_scalar(2607, if s.b[2607] { 1.0 } else { 0.0 });

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2604])) && (!s.b[2606])) && s.b[2607]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 442, -1.0, 2565, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2604])) && (!s.b[2606])) && (!s.b[2607])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2540, 442, -1.0, 2565, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        s.b[2608] = (p.p877 > 1000.0);
        s.store_scalar(2608, if s.b[2608] { 1.0 } else { 0.0 });

        s.b[2609] = (s.v[2539] > ((-s.v[445]) * p.p877));
        s.store_scalar(2609, if s.b[2609] { 1.0 } else { 0.0 });

        s.b[2610] = (p.p880 == 4.0);
        s.store_scalar(2610, if s.b[2610] { 1.0 } else { 0.0 });

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2608])) && s.b[2609]) && s.b[2610]) {
            s.store_mul_scaled_ad_lhs(2540, A::mul3_scaled_output(s.ad_value(2539), s.ad_value(2539), s.ad_value(2539), ((s.v[449] * s.v[449]) * s.v[449])), 2539, s.v[449]);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2608])) && s.b[2609]) && (!s.b[2610])) {
            s.store_powf_ad(2540, A::abs_scaled_input(s.ad_value(2539), s.v[449]), p.p880);
        }

        s.b[2611] = (s.v[409] == 0.5);
        s.store_scalar(2611, if s.b[2611] { 1.0 } else { 0.0 });

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && s.b[2611]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::scale(s.ad_value(2532), s.v[406]));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2611])) {
            s.store_powf_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2532), s.v[406])), s.v[409]);
        }

        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) {
            s.store_add_scaled_inputs3_offset_indices(1902, 2540, ((-s.v[418]) * p.p30), 821, (s.v[421] * p.p30), 2532, ((-s.v[421]) * p.p30), (s.v[418] * p.p30));
        }

        s.b[2612] = (s.v[648] == 0.0);
        s.store_scalar(2612, if s.b[2612] { 1.0 } else { 0.0 });

        if ((s.b[2569] && (!s.b[2570])) && s.b[2612]) {
            s.store_scalar(1903, 0.0);
        }

        s.b[2613] = ((p.p858 == 0.0) && (p.p863 == 0.0));
        s.store_scalar(2613, if s.b[2613] { 1.0 } else { 0.0 });

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2613])) {
            s.store_sub_from_scalar(2543, s.v[395], 2537);
        }

        s.b[2615] = (p.p849 == 0.5);
        s.store_scalar(2615, if s.b[2615] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2613])) && s.b[2615]) {
            s.store_sqrt_scaled_input(2540, 2543, s.v[431]);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2613])) && (!s.b[2615])) {
            s.store_powf_scaled_input(2540, 2543, s.v[431], p.p849);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2613])) {
            s.store_scale(2547, 2540, s.v[425]);
        }

        s.b[2616] = (p.p863 == 0.0);
        s.store_scalar(2616, if s.b[2616] { 1.0 } else { 0.0 });

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) {
            s.store_div_scaled_inputs_indices(2550, 2547, (s.v[410] * s.v[440]), 2543, 1.0);
            s.store_div_from_scalar(2551, (0.666666666666667 * s.v[437]), 2550);
            s.store_square(2552, 2551);
            s.store_sqrt_div_scaled_square_offset_denominator(2553, 2552, 1.0, 1.0, 1.0);
            s.store_sqrt(2554, 2553);
            s.store_mul(2555, 2553, 2554);
            s.store_sqrt_scaled_input_ad(2558, A::div(s.ad_value(2550), s.ad_value(2554)), 0.375);
            s.store_add_scaled_product_indices(2559, 2553, (-1.0), 2551, 2554, 2.0);
            s.store_add_scaled_value_products(2560, s.ad_value(2553), (-s.v[437]), s.ad_value(2551), s.ad_value(2554), s.v[437], s.ad_value(2550), s.ad_value(2555), 0.5);
            s.store_mul_offset_lhs(2561, 2559, (-1.0), 2558);
            s.store_square(2522, 2561);
        }

        s.b[2619] = (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458));
        s.store_scalar(2619, if s.b[2619] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) && s.b[2619]) {
            s.store_exp_sub(2540, 2560, 2522);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) && (!s.b[2619])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2540, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2620] = (s.v[2561] > 0.0);
        s.store_scalar(2620, if s.b[2620] { 1.0 } else { 0.0 });

        s.b[2621] = (s.v[2560] > (-230.25850929940458));
        s.store_scalar(2621, if s.b[2621] { 1.0 } else { 0.0 });

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) && (!s.b[2620])) && s.b[2621]) {
            s.store_exp(2540, 2560);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) && (!s.b[2620])) && (!s.b[2621])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 2560, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2622] = (p.p869 == 0.0);
        s.store_scalar(2622, if s.b[2622] { 1.0 } else { 0.0 });

        s.b[2623] = (p.p849 == 0.5);
        s.store_scalar(2623, if s.b[2623] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2622])) && s.b[2623]) {
            s.store_sqrt_scaled_input_ad(2540, A::sub_from_scalar(p.p846, s.ad_value(2538)), s.v[431]);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2622])) && (!s.b[2623])) {
            s.store_powf_scale_offset_input(2540, 2538, (-s.v[431]), ((p.p846) * (s.v[431])), p.p849);
        }

    }

    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2622])) {
            s.store_div_scaled_offset_numerator(2565, s.ad_value(2538), ((-s.v[428]) * s.v[413]), (((p.p846) * (s.v[428])) * s.v[413]), s.ad_value(2540), 1.0);
        }

        s.b[2624] = (((((-s.v[443]) / s.v[2565])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2624, if s.b[2624] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2622])) && s.b[2624]) {
            s.store_ad_value(2540, A::exp_div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(2565), 1.0));
        }

        s.b[2625] = (((-s.v[443]) / s.v[2565]) < 0.0);
        s.store_scalar(2625, if s.b[2625] { 1.0 } else { 0.0 });

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2622])) && (!s.b[2624])) && s.b[2625]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 443, -1.0, 2565, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2622])) && (!s.b[2624])) && (!s.b[2625])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2540, 443, -1.0, 2565, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        s.b[2626] = (p.p878 > 1000.0);
        s.store_scalar(2626, if s.b[2626] { 1.0 } else { 0.0 });

        s.b[2627] = (s.v[2539] > ((-s.v[445]) * p.p878));
        s.store_scalar(2627, if s.b[2627] { 1.0 } else { 0.0 });

        s.b[2628] = (p.p881 == 4.0);
        s.store_scalar(2628, if s.b[2628] { 1.0 } else { 0.0 });

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2626])) && s.b[2627]) && s.b[2628]) {
            s.store_mul_scaled_ad_lhs(2540, A::mul3_scaled_output(s.ad_value(2539), s.ad_value(2539), s.ad_value(2539), ((s.v[450] * s.v[450]) * s.v[450])), 2539, s.v[450]);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2626])) && s.b[2627]) && (!s.b[2628])) {
            s.store_powf_ad(2540, A::abs_scaled_input(s.ad_value(2539), s.v[450]), p.p881);
        }

        s.b[2629] = (s.v[410] == 0.5);
        s.store_scalar(2629, if s.b[2629] { 1.0 } else { 0.0 });

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && s.b[2629]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::scale(s.ad_value(2532), s.v[407]));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2629])) {
            s.store_powf_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2532), s.v[407])), s.v[410]);
        }

        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) {
            s.store_add_scaled_inputs3_offset_indices(1903, 2540, ((-s.v[419]) * p.p30), 821, (s.v[422] * p.p30), 2532, ((-s.v[422]) * p.p30), (s.v[419] * p.p30));
        }

        s.b[2630] = (s.v[649] == 0.0);
        s.store_scalar(2630, if s.b[2630] { 1.0 } else { 0.0 });

        if ((s.b[2569] && (!s.b[2570])) && s.b[2630]) {
            s.store_scalar(1904, 0.0);
        }

        s.b[2631] = ((p.p859 == 0.0) && (p.p864 == 0.0));
        s.store_scalar(2631, if s.b[2631] { 1.0 } else { 0.0 });

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2631])) {
            s.store_sub_from_scalar(2543, s.v[396], 2537);
        }

        s.b[2633] = (p.p850 == 0.5);
        s.store_scalar(2633, if s.b[2633] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2631])) && s.b[2633]) {
            s.store_sqrt_scaled_input(2540, 2543, s.v[432]);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2631])) && (!s.b[2633])) {
            s.store_powf_scaled_input(2540, 2543, s.v[432], p.p850);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2631])) {
            s.store_scale(2547, 2540, s.v[426]);
        }

        s.b[2634] = (p.p864 == 0.0);
        s.store_scalar(2634, if s.b[2634] { 1.0 } else { 0.0 });

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) {
            s.store_div_scaled_inputs_indices(2550, 2547, (s.v[411] * s.v[441]), 2543, 1.0);
            s.store_div_from_scalar(2551, (0.666666666666667 * s.v[438]), 2550);
            s.store_square(2552, 2551);
            s.store_sqrt_div_scaled_square_offset_denominator(2553, 2552, 1.0, 1.0, 1.0);
            s.store_sqrt(2554, 2553);
            s.store_mul(2555, 2553, 2554);
            s.store_sqrt_scaled_input_ad(2558, A::div(s.ad_value(2550), s.ad_value(2554)), 0.375);
            s.store_add_scaled_product_indices(2559, 2553, (-1.0), 2551, 2554, 2.0);
            s.store_add_scaled_value_products(2560, s.ad_value(2553), (-s.v[438]), s.ad_value(2551), s.ad_value(2554), s.v[438], s.ad_value(2550), s.ad_value(2555), 0.5);
            s.store_mul_offset_lhs(2561, 2559, (-1.0), 2558);
            s.store_square(2522, 2561);
        }

        s.b[2637] = (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458));
        s.store_scalar(2637, if s.b[2637] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) && s.b[2637]) {
            s.store_exp_sub(2540, 2560, 2522);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) && (!s.b[2637])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2540, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2638] = (s.v[2561] > 0.0);
        s.store_scalar(2638, if s.b[2638] { 1.0 } else { 0.0 });

        s.b[2639] = (s.v[2560] > (-230.25850929940458));
        s.store_scalar(2639, if s.b[2639] { 1.0 } else { 0.0 });

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) && (!s.b[2638])) && s.b[2639]) {
            s.store_exp(2540, 2560);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) && (!s.b[2638])) && (!s.b[2639])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 2560, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2640] = (p.p870 == 0.0);
        s.store_scalar(2640, if s.b[2640] { 1.0 } else { 0.0 });

        s.b[2641] = (p.p850 == 0.5);
        s.store_scalar(2641, if s.b[2641] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2640])) && s.b[2641]) {
            s.store_sqrt_scaled_input_ad(2540, A::sub_from_scalar(p.p847, s.ad_value(2538)), s.v[432]);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2640])) && (!s.b[2641])) {
            s.store_powf_scale_offset_input(2540, 2538, (-s.v[432]), ((p.p847) * (s.v[432])), p.p850);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2640])) {
            s.store_div_scaled_offset_numerator(2565, s.ad_value(2538), ((-s.v[429]) * s.v[414]), (((p.p847) * (s.v[429])) * s.v[414]), s.ad_value(2540), 1.0);
        }

        s.b[2642] = (((((-s.v[444]) / s.v[2565])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2642, if s.b[2642] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2640])) && s.b[2642]) {
            s.store_ad_value(2540, A::exp_div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(2565), 1.0));
        }

        s.b[2643] = (((-s.v[444]) / s.v[2565]) < 0.0);
        s.store_scalar(2643, if s.b[2643] { 1.0 } else { 0.0 });

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2640])) && (!s.b[2642])) && s.b[2643]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 444, -1.0, 2565, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2640])) && (!s.b[2642])) && (!s.b[2643])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2540, 444, -1.0, 2565, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        s.b[2644] = (s.v[641] > 1000.0);
        s.store_scalar(2644, if s.b[2644] { 1.0 } else { 0.0 });

        s.b[2645] = (s.v[2539] > ((-s.v[445]) * s.v[641]));
        s.store_scalar(2645, if s.b[2645] { 1.0 } else { 0.0 });

        s.b[2646] = (p.p882 == 4.0);
        s.store_scalar(2646, if s.b[2646] { 1.0 } else { 0.0 });

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2644])) && s.b[2645]) && s.b[2646]) {
            s.store_mul_ad_product_lhs_mixed_ai(2540, A::mul3(A::square(A::mul(s.ad_value(2539), s.ad_value(451))), s.ad_value(2539), s.ad_value(451)), 2539, 451);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2644])) && s.b[2645]) && (!s.b[2646])) {
            s.store_powf_ad(2540, A::abs(A::mul(s.ad_value(2539), s.ad_value(451))), p.p882);
        }

        s.b[2647] = (s.v[474] == 1.0);
        s.store_scalar(2647, if s.b[2647] { 1.0 } else { 0.0 });

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2647]) {
            if (s.v[821] < p.p887) {
                if (((s.v[821] - p.p887) / p.p888) < (-37.0)) {
                    s.store_scalar(2567, p.p887);
                } else {
                    s.store_offset_scaled_ad(2567, A::ln_one_plus_exp(A::scaled_offset(s.ad_value(821), (-p.p887), 1.0 / (p.p888))), p.p888, p.p887);
                }
            } else {
                if (((s.v[821] - p.p887) / p.p888) > 37.0) {
                    s.copy_ad(2567, 821);
                } else {
                    s.store_add_scaled_inputs_ad_rhs(2567, 821, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(821), (-1.0 / (p.p888)), ((p.p887) * (1.0 / (p.p888))))), p.p888);
                }
            }
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2647]) {
            s.store_scaled_mul(2525, 658, 658, 4.0);
            s.store_div(2526, 658, 659);
            s.store_add_scaled_product_indices(2527, 2567, 1.0, 658, 2526, 1.0);
            s.store_add(2528, 659, 2527);
            s.store_sub(2529, 659, 2527);
            s.store_sqrt_square_add(2530, 2529, 2525);
            s.store_div_scaled_product_add_scaled_denominator_indices(2568, 2567, 659, 2.0, 2528, 1.0, 2530, 1.0, 1.0);
        }

        s.b[2648] = (s.v[411] == 0.5);
        s.store_scalar(2648, if s.b[2648] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2647]) && s.b[2648]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::scale(s.ad_value(2568), s.v[408]));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2647]) && (!s.b[2648])) {
            s.store_powf_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2568), s.v[408])), s.v[411]);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2647]) {
            s.store_add_scaled_inputs3_offset_indices(1904, 2540, ((-s.v[420]) * p.p30), 2567, (s.v[423] * p.p30), 2568, ((-s.v[423]) * p.p30), (s.v[420] * p.p30));
            s.store_sub_offset_lhs(2567, 821, p.p887, 2567);
            s.store_scaled_mul(2525, 658, 658, 4.0);
            s.store_div(2526, 658, 659);
            s.store_add_scaled_product_indices(2527, 2567, 1.0, 658, 2526, 1.0);
            s.store_add(2528, 659, 2527);
            s.store_sub(2529, 659, 2527);
            s.store_sqrt_square_add(2530, 2529, 2525);
            s.store_div_scaled_product_add_scaled_denominator_indices(2568, 2567, 659, 2.0, 2528, 1.0, 2530, 1.0, 1.0);
        }

        s.b[2649] = (s.v[468] == 0.5);
        s.store_scalar(2649, if s.b[2649] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2647]) && s.b[2649]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2568), s.ad_value(467)));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2647]) && (!s.b[2649])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2540, 1.0, 2568, 467, 468);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2647]) {
            s.store_add_scaled_product_mixed_aia(473, A::mul_sub_from_scalar_rhs(s.ad_value(471), 1.0, s.ad_value(2540)), p.p30, 472, A::sub(s.ad_value(2567), s.ad_value(2568)), p.p30);
            s.store_add(1904, 1904, 473);
        }

        s.b[2650] = (s.v[411] == 0.5);
        s.store_scalar(2650, if s.b[2650] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2647])) && s.b[2650]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::scale(s.ad_value(2532), s.v[408]));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2647])) && (!s.b[2650])) {
            s.store_powf_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2532), s.v[408])), s.v[411]);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2647])) {
            s.store_add_scaled_inputs3_offset_indices(1904, 2540, ((-s.v[420]) * p.p30), 821, (s.v[423] * p.p30), 2532, ((-s.v[423]) * p.p30), (s.v[420] * p.p30));
        }

        s.b[2651] = (s.v[637] > 0.0);
        s.store_scalar(2651, if s.b[2651] { 1.0 } else { 0.0 });

        if ((s.b[2569] && (!s.b[2570])) && s.b[2651]) {
            s.store_mul_sub_ad_rhs(644, 637, A::pow(A::add_scaled_inputs3(s.ad_value(814), 0.5, s.ad_value(816), 0.5, A::sqrt_square_offset(A::add(s.ad_value(814), s.ad_value(816)), (0.001 * 0.001)), 0.5), s.ad_value(638)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(638)));
            s.store_add(642, 543, 644);
            s.store_div_from_scalar(617, 1.0, 642);
        }

        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2651])) {
            s.copy_ad(642, 543);
        }

        s.b[2652] = (s.v[639] > 0.0);
        s.store_scalar(2652, if s.b[2652] { 1.0 } else { 0.0 });

        if ((s.b[2569] && (!s.b[2570])) && s.b[2652]) {
            s.store_mul_sub_ad_rhs(646, 639, A::pow(A::add_scaled_inputs3(s.ad_value(814), 0.5, s.ad_value(816), 0.5, A::sqrt_square_offset(A::add(s.ad_value(814), s.ad_value(816)), (0.001 * 0.001)), 0.5), s.ad_value(640)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(640)));
            s.store_mul_offset_rhs(611, 611, 646, 1.0);
        }

        if (s.b[2569] && (!s.b[2570])) {
            s.store_scalar(2538, 0.0);
            s.store_scalar(2535, 0.0);
        }

        s.b[2653] = (!(((s.v[674] == 0.0) && (s.v[675] == 0.0)) && (s.v[676] == 0.0)));
        s.store_scalar(2653, if s.b[2653] { 1.0 } else { 0.0 });

        if ((s.b[2569] && (!s.b[2570])) && s.b[2653]) {
            s.store_scaled_mul(2525, 685, 685, 4.0);
            s.store_div(2526, 685, 686);
            s.store_add_scaled_product_indices(2527, 822, 1.0, 685, 2526, 1.0);
            s.store_add(2528, 686, 2527);
            s.store_sub(2529, 686, 2527);
            s.store_sqrt_square_add(2530, 2529, 2525);
            s.store_div_scaled_product_add_scaled_denominator_indices(2532, 822, 686, 2.0, 2528, 1.0, 2530, 1.0, 1.0);
        }

        s.b[2654] = (s.v[822] < s.v[682]);
        s.store_scalar(2654, if s.b[2654] { 1.0 } else { 0.0 });

        s.b[2655] = (((((-0.5) * (s.v[822] * s.v[372]))) as f64).abs() < 230.25850929940458);
        s.store_scalar(2655, if s.b[2655] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && s.b[2653]) && s.b[2654]) && s.b[2655]) {
            s.store_exp_scaled_input(2533, 822, (s.v[372] * (-0.5)));
        }

        s.b[2656] = (((-0.5) * (s.v[822] * s.v[372])) < 0.0);
        s.store_scalar(2656, if s.b[2656] { 1.0 } else { 0.0 });

        if (((((s.b[2569] && (!s.b[2570])) && s.b[2653]) && s.b[2654]) && (!s.b[2655])) && s.b[2656]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2533, 1e-100, (-230.25850929940458), A::scale(s.ad_value(822), (s.v[372] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2569] && (!s.b[2570])) && s.b[2653]) && s.b[2654]) && (!s.b[2655])) && (!s.b[2656])) {
            s.store_scaled_offset_ad(2533, A::mul_offset_rhs(A::scale_offset(s.ad_value(822), (s.v[372] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(822), (s.v[372] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(822), (((s.v[372] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[2569] && (!s.b[2570])) && s.b[2653]) && s.b[2654]) {
            s.store_div_from_scalar(2534, 1.0, 2533);
            s.store_square(2531, 2534);
        }

        if (((s.b[2569] && (!s.b[2570])) && s.b[2653]) && (!s.b[2654])) {
            s.store_mul_offset_ad_lhs(2531, A::sub_scaled_inputs(s.ad_value(822), s.v[372], s.ad_value(682), s.v[372]), 1.0, 683);
            s.store_sqrt(2534, 2531);
            s.store_div_from_scalar(2533, 1.0, 2534);
        }

        if ((s.b[2569] && (!s.b[2570])) && s.b[2653]) {
            s.store_offset(2531, 2531, (-1.0));
        }

        s.b[2657] = (s.v[822] > 0.0);
        s.store_scalar(2657, if s.b[2657] { 1.0 } else { 0.0 });

        if (((s.b[2569] && (!s.b[2570])) && s.b[2653]) && s.b[2657]) {
            s.store_scaled_ln_ad(2535, A::add(A::offset(s.ad_value(2533), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2533), 1.0, A::offset(s.ad_value(2533), 3.0)))), (s.v[371] * 2.0));
        }

        if (((s.b[2569] && (!s.b[2570])) && s.b[2653]) && (!s.b[2657])) {
            s.store_sub_ad_lhs(2535, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(2534), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2534), 1.0, A::scale_offset(s.ad_value(2534), 3.0, 1.0))))), (s.v[371] * 2.0)), 822);
        }

        if ((s.b[2569] && (!s.b[2570])) && s.b[2653]) {
            s.store_sub(2536, 684, 2535);
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2537, 822, 0.5, 2536, 0.5, 822, 2536, ((4.0 * s.v[371]) * s.v[371]), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2538, 822, 0.5, 687, 0.5, 822, 687, ((4.0 * s.v[369]) * s.v[369]), (-0.5));
            s.store_scaled_sub_sqrt_square_offset_rhs(2539, 822, 822, ((4.0 * 1e-6) * 1e-6), 0.5);
        }

        s.b[2658] = (s.v[674] == 0.0);
        s.store_scalar(2658, if s.b[2658] { 1.0 } else { 0.0 });

        if ((s.b[2569] && (!s.b[2570])) && s.b[2658]) {
            s.store_scalar(1905, 0.0);
        }

        s.b[2659] = ((s.v[523] == 0.0) && (s.v[526] == 0.0));
        s.store_scalar(2659, if s.b[2659] { 1.0 } else { 0.0 });

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2659])) {
            s.store_sub(2543, 570, 2537);
        }

        s.b[2661] = (s.v[512] == 0.5);
        s.store_scalar(2661, if s.b[2661] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2659])) && s.b[2661]) {
            s.store_sqrt_mul(2540, 2543, 597);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2659])) && (!s.b[2661])) {
            s.store_pow_mul_base_indices(2540, 2543, 597, 512);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2659])) {
            s.store_mul(2547, 591, 2540);
        }

    }

    pub(super) fn stamp_reactive_block_21(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2662] = (s.v[526] == 0.0);
        s.store_scalar(2662, if s.b[2662] { 1.0 } else { 0.0 });

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) {
            s.store_mul_div_scaled_product_indices(2550, 606, 2547, 576, 1.0, 2543, 1.0);
            s.store_div_scaled_inputs_indices(2551, 603, 0.666666666666667, 2550, 1.0);
            s.store_square(2552, 2551);
            s.store_sqrt_div_scaled_square_offset_denominator(2553, 2552, 1.0, 1.0, 1.0);
            s.store_sqrt(2554, 2553);
            s.store_mul(2555, 2553, 2554);
            s.store_sqrt_scaled_input_ad(2558, A::div(s.ad_value(2550), s.ad_value(2554)), 0.375);
            s.store_add_scaled_product_indices(2559, 2553, (-1.0), 2551, 2554, 2.0);
            s.store_add_scaled_value_products(2560, A::mul3(s.ad_value(603), s.ad_value(2551), s.ad_value(2554)), 1.0, s.ad_value(603), s.ad_value(2553), (-1.0), s.ad_value(2550), s.ad_value(2555), 0.5);
            s.store_mul_offset_lhs(2561, 2559, (-1.0), 2558);
            s.store_square(2522, 2561);
        }

        s.b[2665] = (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458));
        s.store_scalar(2665, if s.b[2665] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && s.b[2665]) {
            s.store_exp_sub(2540, 2560, 2522);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && (!s.b[2665])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2540, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2666] = (s.v[2561] > 0.0);
        s.store_scalar(2666, if s.b[2666] { 1.0 } else { 0.0 });

        s.b[2667] = (s.v[2560] > (-230.25850929940458));
        s.store_scalar(2667, if s.b[2667] { 1.0 } else { 0.0 });

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && (!s.b[2666])) && s.b[2667]) {
            s.store_exp(2540, 2560);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && (!s.b[2666])) && (!s.b[2667])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 2560, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2668] = (s.v[532] == 0.0);
        s.store_scalar(2668, if s.b[2668] { 1.0 } else { 0.0 });

        s.b[2669] = (s.v[512] == 0.5);
        s.store_scalar(2669, if s.b[2669] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) && s.b[2669]) {
            s.store_sqrt_mul_sub_lhs(2540, 509, 2538, 597);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) && (!s.b[2669])) {
            s.store_pow_mul_base_mixed_ai(2540, A::sub(s.ad_value(509), s.ad_value(2538)), 597, 512);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) {
            s.store_mul_div_scaled_product_mixed_iaii(2565, 579, A::sub(s.ad_value(509), s.ad_value(2538)), 594, 1.0, 2540, 1.0);
        }

        s.b[2670] = (((((-s.v[609]) / s.v[2565])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2670, if s.b[2670] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) && s.b[2670]) {
            s.store_ad_value(2540, A::exp_div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(2565), 1.0));
        }

        s.b[2671] = (((-s.v[609]) / s.v[2565]) < 0.0);
        s.store_scalar(2671, if s.b[2671] { 1.0 } else { 0.0 });

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) && (!s.b[2670])) && s.b[2671]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 609, -1.0, 2565, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) && (!s.b[2670])) && (!s.b[2671])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2540, 609, -1.0, 2565, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        s.b[2672] = (s.v[541] > 1000.0);
        s.store_scalar(2672, if s.b[2672] { 1.0 } else { 0.0 });

        s.b[2673] = (s.v[2539] > ((-s.v[445]) * s.v[541]));
        s.store_scalar(2673, if s.b[2673] { 1.0 } else { 0.0 });

        s.b[2674] = (s.v[544] == 4.0);
        s.store_scalar(2674, if s.b[2674] { 1.0 } else { 0.0 });

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2672])) && s.b[2673]) && s.b[2674]) {
            s.store_mul_ad_product_lhs_mixed_ai(2540, A::mul3(A::square(A::mul(s.ad_value(2539), s.ad_value(615))), s.ad_value(2539), s.ad_value(615)), 2539, 615);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2672])) && s.b[2673]) && (!s.b[2674])) {
            s.store_pow_abs_mul_base_indices(2540, 2539, 615, 544);
        }

        s.b[2675] = (s.v[576] == 0.5);
        s.store_scalar(2675, if s.b[2675] { 1.0 } else { 0.0 });

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && s.b[2675]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2532), s.ad_value(573)));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2675])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2540, 1.0, 2532, 573, 576);
        }

        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) {
            s.store_add_scaled_product_mixed_aia(1905, A::mul_sub_from_scalar_rhs(s.ad_value(585), 1.0, s.ad_value(2540)), p.p30, 588, A::sub(s.ad_value(822), s.ad_value(2532)), p.p30);
        }

        s.b[2676] = (s.v[675] == 0.0);
        s.store_scalar(2676, if s.b[2676] { 1.0 } else { 0.0 });

        if ((s.b[2569] && (!s.b[2570])) && s.b[2676]) {
            s.store_scalar(1906, 0.0);
        }

        s.b[2677] = ((s.v[524] == 0.0) && (s.v[527] == 0.0));
        s.store_scalar(2677, if s.b[2677] { 1.0 } else { 0.0 });

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2677])) {
            s.store_sub(2543, 571, 2537);
        }

        s.b[2679] = (s.v[513] == 0.5);
        s.store_scalar(2679, if s.b[2679] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2677])) && s.b[2679]) {
            s.store_sqrt_mul(2540, 2543, 598);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2677])) && (!s.b[2679])) {
            s.store_pow_mul_base_indices(2540, 2543, 598, 513);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2677])) {
            s.store_mul(2547, 592, 2540);
        }

        s.b[2680] = (s.v[527] == 0.0);
        s.store_scalar(2680, if s.b[2680] { 1.0 } else { 0.0 });

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) {
            s.store_mul_div_scaled_product_indices(2550, 607, 2547, 577, 1.0, 2543, 1.0);
            s.store_div_scaled_inputs_indices(2551, 604, 0.666666666666667, 2550, 1.0);
            s.store_square(2552, 2551);
            s.store_sqrt_div_scaled_square_offset_denominator(2553, 2552, 1.0, 1.0, 1.0);
            s.store_sqrt(2554, 2553);
            s.store_mul(2555, 2553, 2554);
            s.store_sqrt_scaled_input_ad(2558, A::div(s.ad_value(2550), s.ad_value(2554)), 0.375);
            s.store_add_scaled_product_indices(2559, 2553, (-1.0), 2551, 2554, 2.0);
            s.store_add_scaled_value_products(2560, A::mul3(s.ad_value(604), s.ad_value(2551), s.ad_value(2554)), 1.0, s.ad_value(604), s.ad_value(2553), (-1.0), s.ad_value(2550), s.ad_value(2555), 0.5);
            s.store_mul_offset_lhs(2561, 2559, (-1.0), 2558);
            s.store_square(2522, 2561);
        }

        s.b[2683] = (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458));
        s.store_scalar(2683, if s.b[2683] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && s.b[2683]) {
            s.store_exp_sub(2540, 2560, 2522);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && (!s.b[2683])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2540, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2684] = (s.v[2561] > 0.0);
        s.store_scalar(2684, if s.b[2684] { 1.0 } else { 0.0 });

        s.b[2685] = (s.v[2560] > (-230.25850929940458));
        s.store_scalar(2685, if s.b[2685] { 1.0 } else { 0.0 });

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && (!s.b[2684])) && s.b[2685]) {
            s.store_exp(2540, 2560);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && (!s.b[2684])) && (!s.b[2685])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 2560, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2686] = (s.v[533] == 0.0);
        s.store_scalar(2686, if s.b[2686] { 1.0 } else { 0.0 });

        s.b[2687] = (s.v[513] == 0.5);
        s.store_scalar(2687, if s.b[2687] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) && s.b[2687]) {
            s.store_sqrt_mul_sub_lhs(2540, 510, 2538, 598);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) && (!s.b[2687])) {
            s.store_pow_mul_base_mixed_ai(2540, A::sub(s.ad_value(510), s.ad_value(2538)), 598, 513);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) {
            s.store_mul_div_scaled_product_mixed_iaii(2565, 580, A::sub(s.ad_value(510), s.ad_value(2538)), 595, 1.0, 2540, 1.0);
        }

        s.b[2688] = (((((-s.v[610]) / s.v[2565])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2688, if s.b[2688] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) && s.b[2688]) {
            s.store_ad_value(2540, A::exp_div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(2565), 1.0));
        }

        s.b[2689] = (((-s.v[610]) / s.v[2565]) < 0.0);
        s.store_scalar(2689, if s.b[2689] { 1.0 } else { 0.0 });

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) && (!s.b[2688])) && s.b[2689]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 610, -1.0, 2565, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) && (!s.b[2688])) && (!s.b[2689])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2540, 610, -1.0, 2565, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        s.b[2690] = (s.v[542] > 1000.0);
        s.store_scalar(2690, if s.b[2690] { 1.0 } else { 0.0 });

        s.b[2691] = (s.v[2539] > ((-s.v[445]) * s.v[542]));
        s.store_scalar(2691, if s.b[2691] { 1.0 } else { 0.0 });

        s.b[2692] = (s.v[545] == 4.0);
        s.store_scalar(2692, if s.b[2692] { 1.0 } else { 0.0 });

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2690])) && s.b[2691]) && s.b[2692]) {
            s.store_mul_ad_product_lhs_mixed_ai(2540, A::mul3(A::square(A::mul(s.ad_value(2539), s.ad_value(616))), s.ad_value(2539), s.ad_value(616)), 2539, 616);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2690])) && s.b[2691]) && (!s.b[2692])) {
            s.store_pow_abs_mul_base_indices(2540, 2539, 616, 545);
        }

        s.b[2693] = (s.v[577] == 0.5);
        s.store_scalar(2693, if s.b[2693] { 1.0 } else { 0.0 });

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && s.b[2693]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2532), s.ad_value(574)));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2693])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2540, 1.0, 2532, 574, 577);
        }

        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) {
            s.store_add_scaled_product_mixed_aia(1906, A::mul_sub_from_scalar_rhs(s.ad_value(586), 1.0, s.ad_value(2540)), p.p30, 589, A::sub(s.ad_value(822), s.ad_value(2532)), p.p30);
        }

        s.b[2694] = (s.v[676] == 0.0);
        s.store_scalar(2694, if s.b[2694] { 1.0 } else { 0.0 });

        if ((s.b[2569] && (!s.b[2570])) && s.b[2694]) {
            s.store_scalar(1907, 0.0);
        }

        s.b[2695] = ((s.v[525] == 0.0) && (s.v[528] == 0.0));
        s.store_scalar(2695, if s.b[2695] { 1.0 } else { 0.0 });

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2695])) {
            s.store_sub(2543, 572, 2537);
        }

        s.b[2697] = (s.v[514] == 0.5);
        s.store_scalar(2697, if s.b[2697] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2695])) && s.b[2697]) {
            s.store_sqrt_mul(2540, 2543, 599);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2695])) && (!s.b[2697])) {
            s.store_pow_mul_base_indices(2540, 2543, 599, 514);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2695])) {
            s.store_mul(2547, 593, 2540);
        }

        s.b[2698] = (s.v[528] == 0.0);
        s.store_scalar(2698, if s.b[2698] { 1.0 } else { 0.0 });

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) {
            s.store_mul_div_scaled_product_indices(2550, 608, 2547, 578, 1.0, 2543, 1.0);
            s.store_div_scaled_inputs_indices(2551, 605, 0.666666666666667, 2550, 1.0);
            s.store_square(2552, 2551);
            s.store_sqrt_div_scaled_square_offset_denominator(2553, 2552, 1.0, 1.0, 1.0);
            s.store_sqrt(2554, 2553);
            s.store_mul(2555, 2553, 2554);
            s.store_sqrt_scaled_input_ad(2558, A::div(s.ad_value(2550), s.ad_value(2554)), 0.375);
            s.store_add_scaled_product_indices(2559, 2553, (-1.0), 2551, 2554, 2.0);
            s.store_add_scaled_value_products(2560, A::mul3(s.ad_value(605), s.ad_value(2551), s.ad_value(2554)), 1.0, s.ad_value(605), s.ad_value(2553), (-1.0), s.ad_value(2550), s.ad_value(2555), 0.5);
            s.store_mul_offset_lhs(2561, 2559, (-1.0), 2558);
            s.store_square(2522, 2561);
        }

        s.b[2701] = (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458));
        s.store_scalar(2701, if s.b[2701] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && s.b[2701]) {
            s.store_exp_sub(2540, 2560, 2522);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && (!s.b[2701])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2540, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2702] = (s.v[2561] > 0.0);
        s.store_scalar(2702, if s.b[2702] { 1.0 } else { 0.0 });

        s.b[2703] = (s.v[2560] > (-230.25850929940458));
        s.store_scalar(2703, if s.b[2703] { 1.0 } else { 0.0 });

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && (!s.b[2702])) && s.b[2703]) {
            s.store_exp(2540, 2560);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && (!s.b[2702])) && (!s.b[2703])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 2560, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2704] = (s.v[534] == 0.0);
        s.store_scalar(2704, if s.b[2704] { 1.0 } else { 0.0 });

        s.b[2705] = (s.v[514] == 0.5);
        s.store_scalar(2705, if s.b[2705] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) && s.b[2705]) {
            s.store_sqrt_mul_sub_lhs(2540, 511, 2538, 599);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) && (!s.b[2705])) {
            s.store_pow_mul_base_mixed_ai(2540, A::sub(s.ad_value(511), s.ad_value(2538)), 599, 514);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) {
            s.store_mul_div_scaled_product_mixed_iaii(2565, 581, A::sub(s.ad_value(511), s.ad_value(2538)), 596, 1.0, 2540, 1.0);
        }

        s.b[2706] = (((((-s.v[611]) / s.v[2565])) as f64).abs() < 230.25850929940458);
        s.store_scalar(2706, if s.b[2706] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) && s.b[2706]) {
            s.store_ad_value(2540, A::exp_div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(2565), 1.0));
        }

        s.b[2707] = (((-s.v[611]) / s.v[2565]) < 0.0);
        s.store_scalar(2707, if s.b[2707] { 1.0 } else { 0.0 });

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) && (!s.b[2706])) && s.b[2707]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 611, -1.0, 2565, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) && (!s.b[2706])) && (!s.b[2707])) {
            s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2540, 611, -1.0, 2565, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);
        }

        s.b[2708] = (s.v[642] > 1000.0);
        s.store_scalar(2708, if s.b[2708] { 1.0 } else { 0.0 });

        s.b[2709] = (s.v[2539] > ((-s.v[445]) * s.v[642]));
        s.store_scalar(2709, if s.b[2709] { 1.0 } else { 0.0 });

        s.b[2710] = (s.v[546] == 4.0);
        s.store_scalar(2710, if s.b[2710] { 1.0 } else { 0.0 });

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2708])) && s.b[2709]) && s.b[2710]) {
            s.store_mul_ad_product_lhs_mixed_ai(2540, A::mul3(A::square(A::mul(s.ad_value(2539), s.ad_value(617))), s.ad_value(2539), s.ad_value(617)), 2539, 617);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2708])) && s.b[2709]) && (!s.b[2710])) {
            s.store_pow_abs_mul_base_indices(2540, 2539, 617, 546);
        }

        s.b[2711] = (s.v[636] == 1.0);
        s.store_scalar(2711, if s.b[2711] { 1.0 } else { 0.0 });

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) {
            if (s.v[822] < s.v[551]) {
                if (((s.v[822] - s.v[551]) / s.v[552]) < (-37.0)) {
                    s.copy_ad(2567, 551);
                } else {
                    s.store_add_scaled_product_left_ad(2567, 551, 1.0, A::ln_one_plus_exp(A::div_scaled_inputs2(s.ad_value(822), 1.0, s.ad_value(551), (-1.0), s.ad_value(552), 1.0)), 552, 1.0);
                }
            } else {
                if (((s.v[822] - s.v[551]) / s.v[552]) > 37.0) {
                    s.copy_ad(2567, 822);
                } else {
                    s.store_add_scaled_product_left_ad(2567, 822, 1.0, A::ln_one_plus_exp(A::div_scaled_inputs2(s.ad_value(551), 1.0, s.ad_value(822), (-1.0), s.ad_value(552), 1.0)), 552, 1.0);
                }
            }
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) {
            s.store_scaled_mul(2525, 685, 685, 4.0);
            s.store_div(2526, 685, 686);
            s.store_add_scaled_product_indices(2527, 2567, 1.0, 685, 2526, 1.0);
            s.store_add(2528, 686, 2527);
            s.store_sub(2529, 686, 2527);
            s.store_sqrt_square_add(2530, 2529, 2525);
        }

    }

    pub(super) fn stamp_reactive_block_22(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) {
            s.store_div_scaled_product_add_scaled_denominator_indices(2568, 2567, 686, 2.0, 2528, 1.0, 2530, 1.0, 1.0);
        }

        s.b[2712] = (s.v[578] == 0.5);
        s.store_scalar(2712, if s.b[2712] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) && s.b[2712]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2568), s.ad_value(575)));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) && (!s.b[2712])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2540, 1.0, 2568, 575, 578);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) {
            s.store_add_scaled_product_mixed_aia(1907, A::mul_sub_from_scalar_rhs(s.ad_value(587), 1.0, s.ad_value(2540)), p.p30, 590, A::sub(s.ad_value(2567), s.ad_value(2568)), p.p30);
            s.store_add_scaled_inputs3_indices(2567, 822, 1.0, 551, 1.0, 2567, -1.0);
            s.store_scaled_mul(2525, 685, 685, 4.0);
            s.store_div(2526, 685, 686);
            s.store_add_scaled_product_indices(2527, 2567, 1.0, 685, 2526, 1.0);
            s.store_add(2528, 686, 2527);
            s.store_sub(2529, 686, 2527);
            s.store_sqrt_square_add(2530, 2529, 2525);
            s.store_div_scaled_product_add_scaled_denominator_indices(2568, 2567, 686, 2.0, 2528, 1.0, 2530, 1.0, 1.0);
        }

        s.b[2713] = (s.v[631] == 0.5);
        s.store_scalar(2713, if s.b[2713] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) && s.b[2713]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2568), s.ad_value(630)));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) && (!s.b[2713])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2540, 1.0, 2568, 630, 631);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) {
            s.store_add_scaled_product_mixed_aia(473, A::mul_sub_from_scalar_rhs(s.ad_value(634), 1.0, s.ad_value(2540)), p.p30, 635, A::sub(s.ad_value(2567), s.ad_value(2568)), p.p30);
            s.store_add(1907, 1907, 473);
        }

        s.b[2714] = (s.v[578] == 0.5);
        s.store_scalar(2714, if s.b[2714] { 1.0 } else { 0.0 });

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2711])) && s.b[2714]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2532), s.ad_value(575)));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2711])) && (!s.b[2714])) {
            s.store_pow_sub_from_scalar_mul_base_indices(2540, 1.0, 2532, 575, 578);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2711])) {
            s.store_add_scaled_product_mixed_aia(1907, A::mul_sub_from_scalar_rhs(s.ad_value(587), 1.0, s.ad_value(2540)), p.p30, 590, A::sub(s.ad_value(822), s.ad_value(2532)), p.p30);
        }

        s.store_add_scaled_inputs3_indices(839, 840, (-1.0), 841, (-1.0), 842, (-1.0));

        s.store_add(843, 843, 1894);

        s.store_add(844, 844, 1895);

        s.store_add_scaled_products3(846, s.ad_value(647), s.ad_value(1902), 1.0, s.ad_value(648), s.ad_value(1903), 1.0, s.ad_value(649), s.ad_value(1904), 1.0);

        s.store_add_scaled_products3(847, s.ad_value(674), s.ad_value(1905), 1.0, s.ad_value(675), s.ad_value(1906), 1.0, s.ad_value(676), s.ad_value(1907), 1.0);

        s.b[2729] = (s.v[820] < 0.0);
        s.store_scalar(2729, if s.b[2729] { 1.0 } else { 0.0 });

        if s.b[2729] {
            s.copy_ad(2728, 842);
            s.copy_ad(842, 839);
            s.copy_ad(839, 2728);
        }

        s.store_mul(849, 1888, 1879);

        s.b[2762] = ((s.v[1813] > 0.0) && (s.v[1917] > 0.0));
        s.store_scalar(2762, if s.b[2762] { 1.0 } else { 0.0 });

        s.b[2767] = ((((p.p50 == 1.0) && (s.v[1920] > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0));
        s.store_scalar(2767, if s.b[2767] { 1.0 } else { 0.0 });

        if (s.b[2762] && s.b[2767]) {
            s.store_div_scaled_product3_mixed_aiia(849, A::square(s.ad_value(1892)), 1888, 1879, 1.0, A::square(s.ad_value(1890)), 1.0);
        }

        s.b[2771] = (((p.p46 != 0.0) && (s.v[285] > 0.0)) && (s.v[1864] > 0.0));
        s.store_scalar(2771, if s.b[2771] { 1.0 } else { 0.0 });

        if s.b[2771] {
            s.store_div_scaled_inputs_indices(1930, 1867, 4.0, 1925, 1.0);
            s.store_mul(1930, 760, 1916);
            s.store_mul(1930, 1848, 1861);
        }

    }

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        var_chnl_type: f64,
        var_gbulk: f64,
        var_gdrain: f64,
        var_ggate: f64,
        var_gjuns: f64,
        var_gsource: f64,
        var_guard1735: f64,
        var_guard1736: f64,
        var_guard1737: f64,
        var_guard1738: f64,
        var_guard1739: f64,
        var_guard1740: f64,
        var_i_ds: f64,
        var_i_ds_dn4: f64,
        var_i_ds_dn6: f64,
        var_i_ds_dn7: f64,
        var_i_ds_dn8: f64,
        var_i_ds_dn9: f64,
        var_i_dsedge: f64,
        var_i_dsedge_dn4: f64,
        var_i_dsedge_dn6: f64,
        var_i_dsedge_dn7: f64,
        var_i_dsedge_dn8: f64,
        var_i_dsedge_dn9: f64,
        var_i_gb: f64,
        var_i_gb_dn4: f64,
        var_i_gb_dn6: f64,
        var_i_gb_dn7: f64,
        var_i_gb_dn8: f64,
        var_i_gb_dn9: f64,
        var_i_gcd: f64,
        var_i_gcd_dn4: f64,
        var_i_gcd_dn6: f64,
        var_i_gcd_dn7: f64,
        var_i_gcd_dn8: f64,
        var_i_gcd_dn9: f64,
        var_i_gcs: f64,
        var_i_gcs_dn4: f64,
        var_i_gcs_dn6: f64,
        var_i_gcs_dn7: f64,
        var_i_gcs_dn8: f64,
        var_i_gcs_dn9: f64,
        var_i_gidl: f64,
        var_i_gidl_dn4: f64,
        var_i_gidl_dn6: f64,
        var_i_gidl_dn7: f64,
        var_i_gidl_dn8: f64,
        var_i_gidl_dn9: f64,
        var_i_gisl: f64,
        var_i_gisl_dn4: f64,
        var_i_gisl_dn6: f64,
        var_i_gisl_dn7: f64,
        var_i_gisl_dn8: f64,
        var_i_gisl_dn9: f64,
        var_igdov: f64,
        var_igdov_dn4: f64,
        var_igdov_dn6: f64,
        var_igdov_dn7: f64,
        var_igdov_dn8: f64,
        var_igdov_dn9: f64,
        var_igsov: f64,
        var_igsov_dn4: f64,
        var_igsov_dn6: f64,
        var_igsov_dn7: f64,
        var_igsov_dn8: f64,
        var_igsov_dn9: f64,
        var_iimpact: f64,
        var_iimpact_dn4: f64,
        var_iimpact_dn6: f64,
        var_iimpact_dn7: f64,
        var_iimpact_dn8: f64,
        var_iimpact_dn9: f64,
        var_ijun_d: f64,
        var_ijun_d_dn11: f64,
        var_ijun_d_dn12: f64,
        var_ijun_d_dn6: f64,
        var_ijun_d_dn7: f64,
        var_ijun_d_dn8: f64,
        var_ijun_d_dn9: f64,
        var_ijun_s: f64,
        var_ijun_s_dn11: f64,
        var_ijun_s_dn12: f64,
        var_ijun_s_dn6: f64,
        var_ijun_s_dn7: f64,
        var_ijun_s_dn8: f64,
        var_ijun_s_dn9: f64,
        var_mult_inst: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq0_e972, eq0_e972_d_n4, eq0_e972_d_n6, eq0_e972_d_n7, eq0_e972_d_n8, eq0_e972_d_n9,) = {
    if (var_guard1735 != 0.0) {
        let eq0_e966: f64 = (var_chnl_type * var_mult_inst);
        let eq0_e968: f64 = (eq0_e966 * p.p32);
        let eq0_e970: f64 = (eq0_e968 * var_iimpact);
        let eq0_e970_d_n4: f64 = (eq0_e968 * var_iimpact_dn4);
        let eq0_e970_d_n6: f64 = (eq0_e968 * var_iimpact_dn6);
        let eq0_e970_d_n7: f64 = (eq0_e968 * var_iimpact_dn7);
        let eq0_e970_d_n8: f64 = (eq0_e968 * var_iimpact_dn8);
        let eq0_e970_d_n9: f64 = (eq0_e968 * var_iimpact_dn9);
        (eq0_e970, eq0_e970_d_n4, eq0_e970_d_n6, eq0_e970_d_n7, eq0_e970_d_n8, eq0_e970_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e972;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(9),
            multiplicity * (eq0_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq0_e972_d_n4), multiplicity * (eq0_e972_d_n6), multiplicity * (eq0_e972_d_n7), multiplicity * (eq0_e972_d_n8), multiplicity * (eq0_e972_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq1_e984, eq1_e984_d_n4, eq1_e984_d_n6, eq1_e984_d_n7, eq1_e984_d_n8, eq1_e984_d_n9,) = {
    if (var_guard1735 != 0.0) {
        let eq1_e976: f64 = (var_chnl_type * var_mult_inst);
        let eq1_e978: f64 = (eq1_e976 * p.p32);
        let eq1_e981: f64 = (var_i_ds + var_i_dsedge);
        let eq1_e981_d_n4: f64 = (var_i_ds_dn4 + var_i_dsedge_dn4);
        let eq1_e981_d_n6: f64 = (var_i_ds_dn6 + var_i_dsedge_dn6);
        let eq1_e981_d_n7: f64 = (var_i_ds_dn7 + var_i_dsedge_dn7);
        let eq1_e981_d_n8: f64 = (var_i_ds_dn8 + var_i_dsedge_dn8);
        let eq1_e981_d_n9: f64 = (var_i_ds_dn9 + var_i_dsedge_dn9);
        let eq1_e982: f64 = (eq1_e978 * eq1_e981);
        let eq1_e982_d_n4: f64 = (eq1_e978 * eq1_e981_d_n4);
        let eq1_e982_d_n6: f64 = (eq1_e978 * eq1_e981_d_n6);
        let eq1_e982_d_n7: f64 = (eq1_e978 * eq1_e981_d_n7);
        let eq1_e982_d_n8: f64 = (eq1_e978 * eq1_e981_d_n8);
        let eq1_e982_d_n9: f64 = (eq1_e978 * eq1_e981_d_n9);
        (eq1_e982, eq1_e982_d_n4, eq1_e982_d_n6, eq1_e982_d_n7, eq1_e982_d_n8, eq1_e982_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e984;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(7),
            multiplicity * (eq1_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq1_e984_d_n4), multiplicity * (eq1_e984_d_n6), multiplicity * (eq1_e984_d_n7), multiplicity * (eq1_e984_d_n8), multiplicity * (eq1_e984_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq2_e994, eq2_e994_d_n4, eq2_e994_d_n6, eq2_e994_d_n7, eq2_e994_d_n8, eq2_e994_d_n9,) = {
    if (var_guard1735 != 0.0) {
        let eq2_e988: f64 = (var_chnl_type * var_mult_inst);
        let eq2_e990: f64 = (eq2_e988 * p.p32);
        let eq2_e992: f64 = (eq2_e990 * var_i_gcs);
        let eq2_e992_d_n4: f64 = (eq2_e990 * var_i_gcs_dn4);
        let eq2_e992_d_n6: f64 = (eq2_e990 * var_i_gcs_dn6);
        let eq2_e992_d_n7: f64 = (eq2_e990 * var_i_gcs_dn7);
        let eq2_e992_d_n8: f64 = (eq2_e990 * var_i_gcs_dn8);
        let eq2_e992_d_n9: f64 = (eq2_e990 * var_i_gcs_dn9);
        (eq2_e992, eq2_e992_d_n4, eq2_e992_d_n6, eq2_e992_d_n7, eq2_e992_d_n8, eq2_e992_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e994;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq2_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq2_e994_d_n4), multiplicity * (eq2_e994_d_n6), multiplicity * (eq2_e994_d_n7), multiplicity * (eq2_e994_d_n8), multiplicity * (eq2_e994_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq3_e1004, eq3_e1004_d_n4, eq3_e1004_d_n6, eq3_e1004_d_n7, eq3_e1004_d_n8, eq3_e1004_d_n9,) = {
    if (var_guard1735 != 0.0) {
        let eq3_e998: f64 = (var_chnl_type * var_mult_inst);
        let eq3_e1000: f64 = (eq3_e998 * p.p32);
        let eq3_e1002: f64 = (eq3_e1000 * var_i_gcd);
        let eq3_e1002_d_n4: f64 = (eq3_e1000 * var_i_gcd_dn4);
        let eq3_e1002_d_n6: f64 = (eq3_e1000 * var_i_gcd_dn6);
        let eq3_e1002_d_n7: f64 = (eq3_e1000 * var_i_gcd_dn7);
        let eq3_e1002_d_n8: f64 = (eq3_e1000 * var_i_gcd_dn8);
        let eq3_e1002_d_n9: f64 = (eq3_e1000 * var_i_gcd_dn9);
        (eq3_e1002, eq3_e1002_d_n4, eq3_e1002_d_n6, eq3_e1002_d_n7, eq3_e1002_d_n8, eq3_e1002_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e1004;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq3_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq3_e1004_d_n4), multiplicity * (eq3_e1004_d_n6), multiplicity * (eq3_e1004_d_n7), multiplicity * (eq3_e1004_d_n8), multiplicity * (eq3_e1004_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq4_e1015, eq4_e1015_d_n4, eq4_e1015_d_n6, eq4_e1015_d_n7, eq4_e1015_d_n8, eq4_e1015_d_n9,) = {
    if (var_guard1735 == 0.0) {
        let eq4_e1009: f64 = (var_chnl_type * var_mult_inst);
        let eq4_e1011: f64 = (eq4_e1009 * p.p32);
        let eq4_e1013: f64 = (eq4_e1011 * var_iimpact);
        let eq4_e1013_d_n4: f64 = (eq4_e1011 * var_iimpact_dn4);
        let eq4_e1013_d_n6: f64 = (eq4_e1011 * var_iimpact_dn6);
        let eq4_e1013_d_n7: f64 = (eq4_e1011 * var_iimpact_dn7);
        let eq4_e1013_d_n8: f64 = (eq4_e1011 * var_iimpact_dn8);
        let eq4_e1013_d_n9: f64 = (eq4_e1011 * var_iimpact_dn9);
        (eq4_e1013, eq4_e1013_d_n4, eq4_e1013_d_n6, eq4_e1013_d_n7, eq4_e1013_d_n8, eq4_e1013_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e1015;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq4_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq4_e1015_d_n4), multiplicity * (eq4_e1015_d_n6), multiplicity * (eq4_e1015_d_n7), multiplicity * (eq4_e1015_d_n8), multiplicity * (eq4_e1015_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq5_e1028, eq5_e1028_d_n4, eq5_e1028_d_n6, eq5_e1028_d_n7, eq5_e1028_d_n8, eq5_e1028_d_n9,) = {
    if (var_guard1735 == 0.0) {
        let eq5_e1020: f64 = (var_chnl_type * var_mult_inst);
        let eq5_e1022: f64 = (eq5_e1020 * p.p32);
        let eq5_e1025: f64 = (var_i_ds + var_i_dsedge);
        let eq5_e1025_d_n4: f64 = (var_i_ds_dn4 + var_i_dsedge_dn4);
        let eq5_e1025_d_n6: f64 = (var_i_ds_dn6 + var_i_dsedge_dn6);
        let eq5_e1025_d_n7: f64 = (var_i_ds_dn7 + var_i_dsedge_dn7);
        let eq5_e1025_d_n8: f64 = (var_i_ds_dn8 + var_i_dsedge_dn8);
        let eq5_e1025_d_n9: f64 = (var_i_ds_dn9 + var_i_dsedge_dn9);
        let eq5_e1026: f64 = (eq5_e1022 * eq5_e1025);
        let eq5_e1026_d_n4: f64 = (eq5_e1022 * eq5_e1025_d_n4);
        let eq5_e1026_d_n6: f64 = (eq5_e1022 * eq5_e1025_d_n6);
        let eq5_e1026_d_n7: f64 = (eq5_e1022 * eq5_e1025_d_n7);
        let eq5_e1026_d_n8: f64 = (eq5_e1022 * eq5_e1025_d_n8);
        let eq5_e1026_d_n9: f64 = (eq5_e1022 * eq5_e1025_d_n9);
        (eq5_e1026, eq5_e1026_d_n4, eq5_e1026_d_n6, eq5_e1026_d_n7, eq5_e1026_d_n8, eq5_e1026_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e1028;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(8),
            multiplicity * (eq5_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq5_e1028_d_n4), multiplicity * (eq5_e1028_d_n6), multiplicity * (eq5_e1028_d_n7), multiplicity * (eq5_e1028_d_n8), multiplicity * (eq5_e1028_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq6_e1039, eq6_e1039_d_n4, eq6_e1039_d_n6, eq6_e1039_d_n7, eq6_e1039_d_n8, eq6_e1039_d_n9,) = {
    if (var_guard1735 == 0.0) {
        let eq6_e1033: f64 = (var_chnl_type * var_mult_inst);
        let eq6_e1035: f64 = (eq6_e1033 * p.p32);
        let eq6_e1037: f64 = (eq6_e1035 * var_i_gcs);
        let eq6_e1037_d_n4: f64 = (eq6_e1035 * var_i_gcs_dn4);
        let eq6_e1037_d_n6: f64 = (eq6_e1035 * var_i_gcs_dn6);
        let eq6_e1037_d_n7: f64 = (eq6_e1035 * var_i_gcs_dn7);
        let eq6_e1037_d_n8: f64 = (eq6_e1035 * var_i_gcs_dn8);
        let eq6_e1037_d_n9: f64 = (eq6_e1035 * var_i_gcs_dn9);
        (eq6_e1037, eq6_e1037_d_n4, eq6_e1037_d_n6, eq6_e1037_d_n7, eq6_e1037_d_n8, eq6_e1037_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e1039;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq6_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq6_e1039_d_n4), multiplicity * (eq6_e1039_d_n6), multiplicity * (eq6_e1039_d_n7), multiplicity * (eq6_e1039_d_n8), multiplicity * (eq6_e1039_d_n9)],
            [],
            [],
            1.0,
        );
        let (eq7_e1050, eq7_e1050_d_n4, eq7_e1050_d_n6, eq7_e1050_d_n7, eq7_e1050_d_n8, eq7_e1050_d_n9,) = {
    if (var_guard1735 == 0.0) {
        let eq7_e1044: f64 = (var_chnl_type * var_mult_inst);
        let eq7_e1046: f64 = (eq7_e1044 * p.p32);
        let eq7_e1048: f64 = (eq7_e1046 * var_i_gcd);
        let eq7_e1048_d_n4: f64 = (eq7_e1046 * var_i_gcd_dn4);
        let eq7_e1048_d_n6: f64 = (eq7_e1046 * var_i_gcd_dn6);
        let eq7_e1048_d_n7: f64 = (eq7_e1046 * var_i_gcd_dn7);
        let eq7_e1048_d_n8: f64 = (eq7_e1046 * var_i_gcd_dn8);
        let eq7_e1048_d_n9: f64 = (eq7_e1046 * var_i_gcd_dn9);
        (eq7_e1048, eq7_e1048_d_n4, eq7_e1048_d_n6, eq7_e1048_d_n7, eq7_e1048_d_n8, eq7_e1048_d_n9,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e1050;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq7_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq7_e1050_d_n4), multiplicity * (eq7_e1050_d_n6), multiplicity * (eq7_e1050_d_n7), multiplicity * (eq7_e1050_d_n8), multiplicity * (eq7_e1050_d_n9)],
            [],
            [],
            1.0,
        );
        let eq8_e1053: f64 = (var_chnl_type * var_mult_inst);
        let eq8_e1055: f64 = (eq8_e1053 * p.p32);
        let eq8_e1057: f64 = (eq8_e1055 * var_i_gb);
        let eq8_e1057_d_n4: f64 = (eq8_e1055 * var_i_gb_dn4);
        let eq8_e1057_d_n6: f64 = (eq8_e1055 * var_i_gb_dn6);
        let eq8_e1057_d_n7: f64 = (eq8_e1055 * var_i_gb_dn7);
        let eq8_e1057_d_n8: f64 = (eq8_e1055 * var_i_gb_dn8);
        let eq8_e1057_d_n9: f64 = (eq8_e1055 * var_i_gb_dn9);
        let eq8_value: f64 = eq8_e1057;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(9),
            multiplicity * (eq8_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq8_e1057_d_n4), multiplicity * (eq8_e1057_d_n6), multiplicity * (eq8_e1057_d_n7), multiplicity * (eq8_e1057_d_n8), multiplicity * (eq8_e1057_d_n9)],
            [],
            [],
            1.0,
        );
        let eq9_e1060: f64 = (var_chnl_type * var_mult_inst);
        let eq9_e1062: f64 = (eq9_e1060 * p.p32);
        let eq9_e1064: f64 = (eq9_e1062 * var_igsov);
        let eq9_e1064_d_n4: f64 = (eq9_e1062 * var_igsov_dn4);
        let eq9_e1064_d_n6: f64 = (eq9_e1062 * var_igsov_dn6);
        let eq9_e1064_d_n7: f64 = (eq9_e1062 * var_igsov_dn7);
        let eq9_e1064_d_n8: f64 = (eq9_e1062 * var_igsov_dn8);
        let eq9_e1064_d_n9: f64 = (eq9_e1062 * var_igsov_dn9);
        let eq9_value: f64 = eq9_e1064;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq9_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq9_e1064_d_n4), multiplicity * (eq9_e1064_d_n6), multiplicity * (eq9_e1064_d_n7), multiplicity * (eq9_e1064_d_n8), multiplicity * (eq9_e1064_d_n9)],
            [],
            [],
            1.0,
        );
        let eq10_e1067: f64 = (var_chnl_type * var_mult_inst);
        let eq10_e1069: f64 = (eq10_e1067 * p.p32);
        let eq10_e1071: f64 = (eq10_e1069 * var_igdov);
        let eq10_e1071_d_n4: f64 = (eq10_e1069 * var_igdov_dn4);
        let eq10_e1071_d_n6: f64 = (eq10_e1069 * var_igdov_dn6);
        let eq10_e1071_d_n7: f64 = (eq10_e1069 * var_igdov_dn7);
        let eq10_e1071_d_n8: f64 = (eq10_e1069 * var_igdov_dn8);
        let eq10_e1071_d_n9: f64 = (eq10_e1069 * var_igdov_dn9);
        let eq10_value: f64 = eq10_e1071;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq10_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq10_e1071_d_n4), multiplicity * (eq10_e1071_d_n6), multiplicity * (eq10_e1071_d_n7), multiplicity * (eq10_e1071_d_n8), multiplicity * (eq10_e1071_d_n9)],
            [],
            [],
            1.0,
        );
        let eq11_e1074: f64 = (var_chnl_type * var_mult_inst);
        let eq11_e1076: f64 = (eq11_e1074 * p.p32);
        let eq11_e1078: f64 = (eq11_e1076 * var_i_gisl);
        let eq11_e1078_d_n4: f64 = (eq11_e1076 * var_i_gisl_dn4);
        let eq11_e1078_d_n6: f64 = (eq11_e1076 * var_i_gisl_dn6);
        let eq11_e1078_d_n7: f64 = (eq11_e1076 * var_i_gisl_dn7);
        let eq11_e1078_d_n8: f64 = (eq11_e1076 * var_i_gisl_dn8);
        let eq11_e1078_d_n9: f64 = (eq11_e1076 * var_i_gisl_dn9);
        let eq11_value: f64 = eq11_e1078;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(7),
            Some(9),
            multiplicity * (eq11_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq11_e1078_d_n4), multiplicity * (eq11_e1078_d_n6), multiplicity * (eq11_e1078_d_n7), multiplicity * (eq11_e1078_d_n8), multiplicity * (eq11_e1078_d_n9)],
            [],
            [],
            1.0,
        );
        let eq12_e1081: f64 = (var_chnl_type * var_mult_inst);
        let eq12_e1083: f64 = (eq12_e1081 * p.p32);
        let eq12_e1085: f64 = (eq12_e1083 * var_i_gidl);
        let eq12_e1085_d_n4: f64 = (eq12_e1083 * var_i_gidl_dn4);
        let eq12_e1085_d_n6: f64 = (eq12_e1083 * var_i_gidl_dn6);
        let eq12_e1085_d_n7: f64 = (eq12_e1083 * var_i_gidl_dn7);
        let eq12_e1085_d_n8: f64 = (eq12_e1083 * var_i_gidl_dn8);
        let eq12_e1085_d_n9: f64 = (eq12_e1083 * var_i_gidl_dn9);
        let eq12_value: f64 = eq12_e1085;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(9),
            multiplicity * (eq12_value),
            [4, 6, 7, 8, 9],
            [multiplicity * (eq12_e1085_d_n4), multiplicity * (eq12_e1085_d_n6), multiplicity * (eq12_e1085_d_n7), multiplicity * (eq12_e1085_d_n8), multiplicity * (eq12_e1085_d_n9)],
            [],
            [],
            1.0,
        );
        let eq13_e1088: f64 = (var_chnl_type * var_mult_inst);
        let eq13_e1090: f64 = (eq13_e1088 * p.p32);
        let eq13_e1092: f64 = (eq13_e1090 * var_ijun_s);
        let eq13_e1092_d_n6: f64 = (eq13_e1090 * var_ijun_s_dn6);
        let eq13_e1092_d_n7: f64 = (eq13_e1090 * var_ijun_s_dn7);
        let eq13_e1092_d_n8: f64 = (eq13_e1090 * var_ijun_s_dn8);
        let eq13_e1092_d_n9: f64 = (eq13_e1090 * var_ijun_s_dn9);
        let eq13_e1092_d_n11: f64 = (eq13_e1090 * var_ijun_s_dn11);
        let eq13_e1092_d_n12: f64 = (eq13_e1090 * var_ijun_s_dn12);
        let eq13_value: f64 = eq13_e1092;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(7),
            multiplicity * (eq13_value),
            [6, 7, 8, 9, 11, 12],
            [multiplicity * (eq13_e1092_d_n6), multiplicity * (eq13_e1092_d_n7), multiplicity * (eq13_e1092_d_n8), multiplicity * (eq13_e1092_d_n9), multiplicity * (eq13_e1092_d_n11), multiplicity * (eq13_e1092_d_n12)],
            [],
            [],
            1.0,
        );
        let eq14_e1095: f64 = (var_chnl_type * var_mult_inst);
        let eq14_e1097: f64 = (eq14_e1095 * p.p32);
        let eq14_e1099: f64 = (eq14_e1097 * var_ijun_d);
        let eq14_e1099_d_n6: f64 = (eq14_e1097 * var_ijun_d_dn6);
        let eq14_e1099_d_n7: f64 = (eq14_e1097 * var_ijun_d_dn7);
        let eq14_e1099_d_n8: f64 = (eq14_e1097 * var_ijun_d_dn8);
        let eq14_e1099_d_n9: f64 = (eq14_e1097 * var_ijun_d_dn9);
        let eq14_e1099_d_n11: f64 = (eq14_e1097 * var_ijun_d_dn11);
        let eq14_e1099_d_n12: f64 = (eq14_e1097 * var_ijun_d_dn12);
        let eq14_value: f64 = eq14_e1099;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(12),
            Some(8),
            multiplicity * (eq14_value),
            [6, 7, 8, 9, 11, 12],
            [multiplicity * (eq14_e1099_d_n6), multiplicity * (eq14_e1099_d_n7), multiplicity * (eq14_e1099_d_n8), multiplicity * (eq14_e1099_d_n9), multiplicity * (eq14_e1099_d_n11), multiplicity * (eq14_e1099_d_n12)],
            [],
            [],
            1.0,
        );
        let (eq15_e1109, eq15_e1109_d_n1, eq15_e1109_d_n6,) = {
    if (var_guard1736 != 0.0) {
        let eq15_e1103: f64 = (var_mult_inst * p.p32);
        let eq15_e1105: f64 = (eq15_e1103 * var_ggate);
        let eq15_e1107: f64 = (eq15_e1105 * (nv1 - nv6));
        (eq15_e1107, eq15_e1105, (-eq15_e1105),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e1109;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(6),
            multiplicity * (eq15_value),
            1,
            multiplicity * (eq15_e1109_d_n1),
            6,
            multiplicity * (eq15_e1109_d_n6),
        );
        let (eq17_e1124,) = {
    if (var_guard1736 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq17_value: f64 = eq17_e1124;
        stamper.stamp_potential_const_local(
            0,
            eq17_value,
        );
        let (eq18_e1134, eq18_e1134_d_n2, eq18_e1134_d_n7,) = {
    if (var_guard1737 != 0.0) {
        let eq18_e1128: f64 = (var_mult_inst * p.p32);
        let eq18_e1130: f64 = (eq18_e1128 * var_gsource);
        let eq18_e1132: f64 = (eq18_e1130 * (nv2 - nv7));
        (eq18_e1132, eq18_e1130, (-eq18_e1130),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1134;
        stamper.stamp_current_node2_local(
            Some(2),
            Some(7),
            multiplicity * (eq18_value),
            2,
            multiplicity * (eq18_e1134_d_n2),
            7,
            multiplicity * (eq18_e1134_d_n7),
        );
        let (eq20_e1149,) = {
    if (var_guard1737 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e1149;
        stamper.stamp_potential_const_local(
            1,
            eq20_value,
        );
        let (eq21_e1159, eq21_e1159_d_n0, eq21_e1159_d_n8,) = {
    if (var_guard1738 != 0.0) {
        let eq21_e1153: f64 = (var_mult_inst * p.p32);
        let eq21_e1155: f64 = (eq21_e1153 * var_gdrain);
        let eq21_e1157: f64 = (eq21_e1155 * (nv0 - nv8));
        (eq21_e1157, eq21_e1155, (-eq21_e1155),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1159;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(8),
            multiplicity * (eq21_value),
            0,
            multiplicity * (eq21_e1159_d_n0),
            8,
            multiplicity * (eq21_e1159_d_n8),
        );
        let (eq23_e1174,) = {
    if (var_guard1738 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e1174;
        stamper.stamp_potential_const_local(
            2,
            eq23_value,
        );
        let (eq24_e1184, eq24_e1184_d_n9, eq24_e1184_d_n10,) = {
    if (var_guard1739 != 0.0) {
        let eq24_e1178: f64 = (var_mult_inst * p.p32);
        let eq24_e1180: f64 = (eq24_e1178 * var_gbulk);
        let eq24_e1182: f64 = (eq24_e1180 * (nv9 - nv10));
        (eq24_e1182, eq24_e1180, (-eq24_e1180),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1184;
        stamper.stamp_current_node2_local(
            Some(9),
            Some(10),
            multiplicity * (eq24_value),
            9,
            multiplicity * (eq24_e1184_d_n9),
            10,
            multiplicity * (eq24_e1184_d_n10),
        );
        let (eq26_e1199,) = {
    if (var_guard1739 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e1199;
        stamper.stamp_potential_const_local(
            3,
            eq26_value,
        );
        let (eq27_e1209, eq27_e1209_d_n10, eq27_e1209_d_n11,) = {
    if (var_guard1740 != 0.0) {
        let eq27_e1203: f64 = (var_mult_inst * p.p32);
        let eq27_e1205: f64 = (eq27_e1203 * var_gjuns);
        let eq27_e1207: f64 = (eq27_e1205 * (nv11 - nv10));
        (eq27_e1207, (-eq27_e1205), eq27_e1205,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1209;
        stamper.stamp_current_node2_local(
            Some(11),
            Some(10),
            multiplicity * (eq27_value),
            10,
            multiplicity * (eq27_e1209_d_n10),
            11,
            multiplicity * (eq27_e1209_d_n11),
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
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
        var_chnl_type: f64,
        var_cth_i: f64,
        var_gjund: f64,
        var_guard1740: f64,
        var_guard1741: f64,
        var_guard1742: f64,
        var_gwell: f64,
        var_mult_inst: f64,
        var_pdiss_1: f64,
        var_pdiss_1_dn0: f64,
        var_pdiss_1_dn2: f64,
        var_pdiss_1_dn4: f64,
        var_pdiss_1_dn6: f64,
        var_pdiss_1_dn7: f64,
        var_pdiss_1_dn8: f64,
        var_pdiss_1_dn9: f64,
        var_qb: f64,
        var_qb_dn4: f64,
        var_qb_dn6: f64,
        var_qb_dn7: f64,
        var_qb_dn8: f64,
        var_qb_dn9: f64,
        var_qd: f64,
        var_qd_dn4: f64,
        var_qd_dn6: f64,
        var_qd_dn7: f64,
        var_qd_dn8: f64,
        var_qd_dn9: f64,
        var_qfgd: f64,
        var_qfgd_dn6: f64,
        var_qfgd_dn7: f64,
        var_qfgd_dn8: f64,
        var_qfgs: f64,
        var_qfgs_dn6: f64,
        var_qfgs_dn7: f64,
        var_qfgs_dn8: f64,
        var_qg: f64,
        var_qg_dn4: f64,
        var_qg_dn6: f64,
        var_qg_dn7: f64,
        var_qg_dn8: f64,
        var_qg_dn9: f64,
        var_qgb_ov: f64,
        var_qgb_ov_dn4: f64,
        var_qgb_ov_dn6: f64,
        var_qgb_ov_dn7: f64,
        var_qgb_ov_dn8: f64,
        var_qgb_ov_dn9: f64,
        var_qjun_d: f64,
        var_qjun_d_dn11: f64,
        var_qjun_d_dn12: f64,
        var_qjun_d_dn6: f64,
        var_qjun_d_dn7: f64,
        var_qjun_d_dn8: f64,
        var_qjun_d_dn9: f64,
        var_qjun_s: f64,
        var_qjun_s_dn11: f64,
        var_qjun_s_dn12: f64,
        var_qjun_s_dn6: f64,
        var_qjun_s_dn7: f64,
        var_qjun_s_dn8: f64,
        var_qjun_s_dn9: f64,
        var_rth_t: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq29_e1224,) = {
    if (var_guard1740 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e1224;
        stamper.stamp_potential_const_local(
            4,
            eq29_value,
        );
        let (eq30_e1234, eq30_e1234_d_n10, eq30_e1234_d_n12,) = {
    if (var_guard1741 != 0.0) {
        let eq30_e1228: f64 = (var_mult_inst * p.p32);
        let eq30_e1230: f64 = (eq30_e1228 * var_gjund);
        let eq30_e1232: f64 = (eq30_e1230 * (nv12 - nv10));
        (eq30_e1232, (-eq30_e1230), eq30_e1230,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e1234;
        stamper.stamp_current_node2_local(
            Some(12),
            Some(10),
            multiplicity * (eq30_value),
            10,
            multiplicity * (eq30_e1234_d_n10),
            12,
            multiplicity * (eq30_e1234_d_n12),
        );
        let (eq32_e1249,) = {
    if (var_guard1741 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e1249;
        stamper.stamp_potential_const_local(
            5,
            eq32_value,
        );
        let (eq33_e1259, eq33_e1259_d_n3, eq33_e1259_d_n10,) = {
    if (var_guard1742 != 0.0) {
        let eq33_e1253: f64 = (var_mult_inst * p.p32);
        let eq33_e1255: f64 = (eq33_e1253 * var_gwell);
        let eq33_e1257: f64 = (eq33_e1255 * (nv3 - nv10));
        (eq33_e1257, eq33_e1255, (-eq33_e1255),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e1259;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(10),
            multiplicity * (eq33_value),
            3,
            multiplicity * (eq33_e1259_d_n3),
            10,
            multiplicity * (eq33_e1259_d_n10),
        );
        let (eq35_e1274,) = {
    if (var_guard1742 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e1274;
        stamper.stamp_potential_const_local(
            6,
            eq35_value,
        );
        let eq38_e1286: f64 = (-var_mult_inst);
        let eq38_e1288: f64 = (eq38_e1286 * var_pdiss_1);
        let eq38_e1288_d_n0: f64 = (eq38_e1286 * var_pdiss_1_dn0);
        let eq38_e1288_d_n2: f64 = (eq38_e1286 * var_pdiss_1_dn2);
        let eq38_e1288_d_n4: f64 = (eq38_e1286 * var_pdiss_1_dn4);
        let eq38_e1288_d_n6: f64 = (eq38_e1286 * var_pdiss_1_dn6);
        let eq38_e1288_d_n7: f64 = (eq38_e1286 * var_pdiss_1_dn7);
        let eq38_e1288_d_n8: f64 = (eq38_e1286 * var_pdiss_1_dn8);
        let eq38_e1288_d_n9: f64 = (eq38_e1286 * var_pdiss_1_dn9);
        let eq38_value: f64 = eq38_e1288;
        stamper.stamp_current_sparse_local::<7, 0>(
            Some(4),
            None,
            multiplicity * (eq38_value),
            [0, 2, 4, 6, 7, 8, 9],
            [multiplicity * (eq38_e1288_d_n0), multiplicity * (eq38_e1288_d_n2), multiplicity * (eq38_e1288_d_n4), multiplicity * (eq38_e1288_d_n6), multiplicity * (eq38_e1288_d_n7), multiplicity * (eq38_e1288_d_n8), multiplicity * (eq38_e1288_d_n9)],
            [],
            [],
            1.0,
        );
        let eq39_e1291: f64 = (var_mult_inst * var_cth_i);
        let eq39_e1293: f64 = (eq39_e1291 * (nv4 - 0.0));
        let eq39_e1294: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq39_e1293);
        let eq39_value: f64 = eq39_e1294;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq39_value),
            4,
            multiplicity * ((eq39_e1291 * ddt_scale)),
        );
        let eq40_e1297: f64 = (var_mult_inst * (nv4 - 0.0));
        let __rspice_inv_cse_0: f64 = 1.0 / var_rth_t;
        let eq40_e1299: f64 = (eq40_e1297 * __rspice_inv_cse_0);
        let eq40_e1299_d_n4: f64 = (var_mult_inst * __rspice_inv_cse_0);
        let eq40_value: f64 = eq40_e1299;
        stamper.stamp_current_node1_local(
            Some(4),
            None,
            multiplicity * (eq40_value),
            4,
            multiplicity * (eq40_e1299_d_n4),
        );
        let eq41_e1302: f64 = (var_chnl_type * var_mult_inst);
        let eq41_e1304: f64 = (eq41_e1302 * p.p33);
        let eq41_e1306: f64 = (eq41_e1304 * var_qg);
        let eq41_e1306_d_n4: f64 = (eq41_e1304 * var_qg_dn4);
        let eq41_e1306_d_n6: f64 = (eq41_e1304 * var_qg_dn6);
        let eq41_e1306_d_n7: f64 = (eq41_e1304 * var_qg_dn7);
        let eq41_e1306_d_n8: f64 = (eq41_e1304 * var_qg_dn8);
        let eq41_e1306_d_n9: f64 = (eq41_e1304 * var_qg_dn9);
        let eq41_e1307: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq41_e1306);
        let eq41_value: f64 = eq41_e1307;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq41_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((eq41_e1306_d_n4 * ddt_scale)), multiplicity * ((eq41_e1306_d_n6 * ddt_scale)), multiplicity * ((eq41_e1306_d_n7 * ddt_scale)), multiplicity * ((eq41_e1306_d_n8 * ddt_scale)), multiplicity * ((eq41_e1306_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq42_e1310: f64 = (var_chnl_type * var_mult_inst);
        let eq42_e1312: f64 = (eq42_e1310 * p.p33);
        let eq42_e1314: f64 = (eq42_e1312 * var_qb);
        let eq42_e1314_d_n4: f64 = (eq42_e1312 * var_qb_dn4);
        let eq42_e1314_d_n6: f64 = (eq42_e1312 * var_qb_dn6);
        let eq42_e1314_d_n7: f64 = (eq42_e1312 * var_qb_dn7);
        let eq42_e1314_d_n8: f64 = (eq42_e1312 * var_qb_dn8);
        let eq42_e1314_d_n9: f64 = (eq42_e1312 * var_qb_dn9);
        let eq42_e1315: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq42_e1314);
        let eq42_value: f64 = eq42_e1315;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq42_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((eq42_e1314_d_n4 * ddt_scale)), multiplicity * ((eq42_e1314_d_n6 * ddt_scale)), multiplicity * ((eq42_e1314_d_n7 * ddt_scale)), multiplicity * ((eq42_e1314_d_n8 * ddt_scale)), multiplicity * ((eq42_e1314_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq43_e1318: f64 = (var_chnl_type * var_mult_inst);
        let eq43_e1320: f64 = (eq43_e1318 * p.p33);
        let eq43_e1322: f64 = (eq43_e1320 * var_qd);
        let eq43_e1322_d_n4: f64 = (eq43_e1320 * var_qd_dn4);
        let eq43_e1322_d_n6: f64 = (eq43_e1320 * var_qd_dn6);
        let eq43_e1322_d_n7: f64 = (eq43_e1320 * var_qd_dn7);
        let eq43_e1322_d_n8: f64 = (eq43_e1320 * var_qd_dn8);
        let eq43_e1322_d_n9: f64 = (eq43_e1320 * var_qd_dn9);
        let eq43_e1323: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq43_e1322);
        let eq43_value: f64 = eq43_e1323;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(8),
            Some(7),
            multiplicity * (eq43_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((eq43_e1322_d_n4 * ddt_scale)), multiplicity * ((eq43_e1322_d_n6 * ddt_scale)), multiplicity * ((eq43_e1322_d_n7 * ddt_scale)), multiplicity * ((eq43_e1322_d_n8 * ddt_scale)), multiplicity * ((eq43_e1322_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq44_e1326: f64 = (var_chnl_type * var_mult_inst);
        let eq44_e1328: f64 = (eq44_e1326 * p.p33);
        let eq44_e1330: f64 = (eq44_e1328 * var_qfgs);
        let eq44_e1330_d_n6: f64 = (eq44_e1328 * var_qfgs_dn6);
        let eq44_e1330_d_n7: f64 = (eq44_e1328 * var_qfgs_dn7);
        let eq44_e1330_d_n8: f64 = (eq44_e1328 * var_qfgs_dn8);
        let eq44_e1331: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq44_e1330);
        let eq44_value: f64 = eq44_e1331;
        stamper.stamp_current_node3_local(
            Some(6),
            Some(7),
            multiplicity * (eq44_value),
            6,
            multiplicity * ((eq44_e1330_d_n6 * ddt_scale)),
            7,
            multiplicity * ((eq44_e1330_d_n7 * ddt_scale)),
            8,
            multiplicity * ((eq44_e1330_d_n8 * ddt_scale)),
        );
        let eq45_e1334: f64 = (var_chnl_type * var_mult_inst);
        let eq45_e1336: f64 = (eq45_e1334 * p.p33);
        let eq45_e1338: f64 = (eq45_e1336 * var_qfgd);
        let eq45_e1338_d_n6: f64 = (eq45_e1336 * var_qfgd_dn6);
        let eq45_e1338_d_n7: f64 = (eq45_e1336 * var_qfgd_dn7);
        let eq45_e1338_d_n8: f64 = (eq45_e1336 * var_qfgd_dn8);
        let eq45_e1339: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq45_e1338);
        let eq45_value: f64 = eq45_e1339;
        stamper.stamp_current_node3_local(
            Some(6),
            Some(8),
            multiplicity * (eq45_value),
            6,
            multiplicity * ((eq45_e1338_d_n6 * ddt_scale)),
            7,
            multiplicity * ((eq45_e1338_d_n7 * ddt_scale)),
            8,
            multiplicity * ((eq45_e1338_d_n8 * ddt_scale)),
        );
        let eq46_e1342: f64 = (var_chnl_type * var_mult_inst);
        let eq46_e1344: f64 = (eq46_e1342 * p.p33);
        let eq46_e1346: f64 = (eq46_e1344 * var_qgb_ov);
        let eq46_e1346_d_n4: f64 = (eq46_e1344 * var_qgb_ov_dn4);
        let eq46_e1346_d_n6: f64 = (eq46_e1344 * var_qgb_ov_dn6);
        let eq46_e1346_d_n7: f64 = (eq46_e1344 * var_qgb_ov_dn7);
        let eq46_e1346_d_n8: f64 = (eq46_e1344 * var_qgb_ov_dn8);
        let eq46_e1346_d_n9: f64 = (eq46_e1344 * var_qgb_ov_dn9);
        let eq46_e1347: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq46_e1346);
        let eq46_value: f64 = eq46_e1347;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(6),
            Some(9),
            multiplicity * (eq46_value),
            [4, 6, 7, 8, 9],
            [multiplicity * ((eq46_e1346_d_n4 * ddt_scale)), multiplicity * ((eq46_e1346_d_n6 * ddt_scale)), multiplicity * ((eq46_e1346_d_n7 * ddt_scale)), multiplicity * ((eq46_e1346_d_n8 * ddt_scale)), multiplicity * ((eq46_e1346_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq47_e1350: f64 = (var_chnl_type * var_mult_inst);
        let eq47_e1352: f64 = (eq47_e1350 * p.p33);
        let eq47_e1354: f64 = (eq47_e1352 * var_qjun_s);
        let eq47_e1354_d_n6: f64 = (eq47_e1352 * var_qjun_s_dn6);
        let eq47_e1354_d_n7: f64 = (eq47_e1352 * var_qjun_s_dn7);
        let eq47_e1354_d_n8: f64 = (eq47_e1352 * var_qjun_s_dn8);
        let eq47_e1354_d_n9: f64 = (eq47_e1352 * var_qjun_s_dn9);
        let eq47_e1354_d_n11: f64 = (eq47_e1352 * var_qjun_s_dn11);
        let eq47_e1354_d_n12: f64 = (eq47_e1352 * var_qjun_s_dn12);
        let eq47_e1355: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq47_e1354);
        let eq47_value: f64 = eq47_e1355;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(7),
            multiplicity * (eq47_value),
            [6, 7, 8, 9, 11, 12],
            [multiplicity * ((eq47_e1354_d_n6 * ddt_scale)), multiplicity * ((eq47_e1354_d_n7 * ddt_scale)), multiplicity * ((eq47_e1354_d_n8 * ddt_scale)), multiplicity * ((eq47_e1354_d_n9 * ddt_scale)), multiplicity * ((eq47_e1354_d_n11 * ddt_scale)), multiplicity * ((eq47_e1354_d_n12 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq48_e1358: f64 = (var_chnl_type * var_mult_inst);
        let eq48_e1360: f64 = (eq48_e1358 * p.p33);
        let eq48_e1362: f64 = (eq48_e1360 * var_qjun_d);
        let eq48_e1362_d_n6: f64 = (eq48_e1360 * var_qjun_d_dn6);
        let eq48_e1362_d_n7: f64 = (eq48_e1360 * var_qjun_d_dn7);
        let eq48_e1362_d_n8: f64 = (eq48_e1360 * var_qjun_d_dn8);
        let eq48_e1362_d_n9: f64 = (eq48_e1360 * var_qjun_d_dn9);
        let eq48_e1362_d_n11: f64 = (eq48_e1360 * var_qjun_d_dn11);
        let eq48_e1362_d_n12: f64 = (eq48_e1360 * var_qjun_d_dn12);
        let eq48_e1363: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq48_e1362);
        let eq48_value: f64 = eq48_e1363;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(12),
            Some(8),
            multiplicity * (eq48_value),
            [6, 7, 8, 9, 11, 12],
            [multiplicity * ((eq48_e1362_d_n6 * ddt_scale)), multiplicity * ((eq48_e1362_d_n7 * ddt_scale)), multiplicity * ((eq48_e1362_d_n8 * ddt_scale)), multiplicity * ((eq48_e1362_d_n9 * ddt_scale)), multiplicity * ((eq48_e1362_d_n11 * ddt_scale)), multiplicity * ((eq48_e1362_d_n12 * ddt_scale))],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
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
        var_cgeff_dn4: f64,
        var_cgeff_dn6: f64,
        var_cgeff_dn7: f64,
        var_cgeff_dn8: f64,
        var_cgeff_dn9: f64,
        var_mig: f64,
        var_mig_dn4: f64,
        var_mig_dn6: f64,
        var_mig_dn7: f64,
        var_mig_dn8: f64,
        var_mig_dn9: f64,
        var_mult_inst: f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq50_e1371: f64 = ((nv5 - 0.0) / var_mig);
        let eq50_e1371_d_n4: f64 = (-(((nv5 - 0.0) * var_mig_dn4) / (var_mig * var_mig)));
        let eq50_e1371_d_n5: f64 = (1.0 / var_mig);
        let eq50_e1371_d_n6: f64 = (-(((nv5 - 0.0) * var_mig_dn6) / (var_mig * var_mig)));
        let eq50_e1371_d_n7: f64 = (-(((nv5 - 0.0) * var_mig_dn7) / (var_mig * var_mig)));
        let eq50_e1371_d_n8: f64 = (-(((nv5 - 0.0) * var_mig_dn8) / (var_mig * var_mig)));
        let eq50_e1371_d_n9: f64 = (-(((nv5 - 0.0) * var_mig_dn9) / (var_mig * var_mig)));
        let eq50_value: f64 = eq50_e1371;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            None,
            multiplicity * (eq50_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * (eq50_e1371_d_n4), multiplicity * (eq50_e1371_d_n5), multiplicity * (eq50_e1371_d_n6), multiplicity * (eq50_e1371_d_n7), multiplicity * (eq50_e1371_d_n8), multiplicity * (eq50_e1371_d_n9)],
            [],
            [],
            1.0,
        );
        let eq51_e1374: f64 = (var_cgeff * (nv5 - 0.0));
        let eq51_e1374_d_n4: f64 = (var_cgeff_dn4 * (nv5 - 0.0));
        let eq51_e1374_d_n6: f64 = (var_cgeff_dn6 * (nv5 - 0.0));
        let eq51_e1374_d_n7: f64 = (var_cgeff_dn7 * (nv5 - 0.0));
        let eq51_e1374_d_n8: f64 = (var_cgeff_dn8 * (nv5 - 0.0));
        let eq51_e1374_d_n9: f64 = (var_cgeff_dn9 * (nv5 - 0.0));
        let eq51_e1375: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq51_e1374);
        let eq51_value: f64 = eq51_e1375;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(5),
            None,
            multiplicity * (eq51_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * ((eq51_e1374_d_n4 * ddt_scale)), multiplicity * ((var_cgeff * ddt_scale)), multiplicity * ((eq51_e1374_d_n6 * ddt_scale)), multiplicity * ((eq51_e1374_d_n7 * ddt_scale)), multiplicity * ((eq51_e1374_d_n8 * ddt_scale)), multiplicity * ((eq51_e1374_d_n9 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq52_e1378: f64 = (var_mult_inst * p.p32);
        let eq52_e1379: f64 = (eq52_e1378).sqrt();
        let eq52_e1381: f64 = (eq52_e1379 * 0.5);
        let eq52_e1383: f64 = (eq52_e1381 * var_cgeff);
        let eq52_e1383_d_n4: f64 = (eq52_e1381 * var_cgeff_dn4);
        let eq52_e1383_d_n6: f64 = (eq52_e1381 * var_cgeff_dn6);
        let eq52_e1383_d_n7: f64 = (eq52_e1381 * var_cgeff_dn7);
        let eq52_e1383_d_n8: f64 = (eq52_e1381 * var_cgeff_dn8);
        let eq52_e1383_d_n9: f64 = (eq52_e1381 * var_cgeff_dn9);
        let eq52_e1385: f64 = (eq52_e1383 * (nv5 - 0.0));
        let eq52_e1385_d_n4: f64 = (eq52_e1383_d_n4 * (nv5 - 0.0));
        let eq52_e1385_d_n6: f64 = (eq52_e1383_d_n6 * (nv5 - 0.0));
        let eq52_e1385_d_n7: f64 = (eq52_e1383_d_n7 * (nv5 - 0.0));
        let eq52_e1385_d_n8: f64 = (eq52_e1383_d_n8 * (nv5 - 0.0));
        let eq52_e1385_d_n9: f64 = (eq52_e1383_d_n9 * (nv5 - 0.0));
        let eq52_e1386: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq52_e1385);
        let eq52_e1387: f64 = (-eq52_e1386);
        let eq52_e1387_d_n4: f64 = (-(eq52_e1385_d_n4 * ddt_scale));
        let eq52_e1387_d_n5: f64 = (-(eq52_e1383 * ddt_scale));
        let eq52_e1387_d_n6: f64 = (-(eq52_e1385_d_n6 * ddt_scale));
        let eq52_e1387_d_n7: f64 = (-(eq52_e1385_d_n7 * ddt_scale));
        let eq52_e1387_d_n8: f64 = (-(eq52_e1385_d_n8 * ddt_scale));
        let eq52_e1387_d_n9: f64 = (-(eq52_e1385_d_n9 * ddt_scale));
        let eq52_value: f64 = eq52_e1387;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq52_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * (eq52_e1387_d_n4), multiplicity * (eq52_e1387_d_n5), multiplicity * (eq52_e1387_d_n6), multiplicity * (eq52_e1387_d_n7), multiplicity * (eq52_e1387_d_n8), multiplicity * (eq52_e1387_d_n9)],
            [],
            [],
            1.0,
        );
        let eq53_e1390: f64 = (var_mult_inst * p.p32);
        let eq53_e1391: f64 = (eq53_e1390).sqrt();
        let eq53_e1393: f64 = (eq53_e1391 * 0.5);
        let eq53_e1395: f64 = (eq53_e1393 * var_cgeff);
        let eq53_e1395_d_n4: f64 = (eq53_e1393 * var_cgeff_dn4);
        let eq53_e1395_d_n6: f64 = (eq53_e1393 * var_cgeff_dn6);
        let eq53_e1395_d_n7: f64 = (eq53_e1393 * var_cgeff_dn7);
        let eq53_e1395_d_n8: f64 = (eq53_e1393 * var_cgeff_dn8);
        let eq53_e1395_d_n9: f64 = (eq53_e1393 * var_cgeff_dn9);
        let eq53_e1397: f64 = (eq53_e1395 * (nv5 - 0.0));
        let eq53_e1397_d_n4: f64 = (eq53_e1395_d_n4 * (nv5 - 0.0));
        let eq53_e1397_d_n6: f64 = (eq53_e1395_d_n6 * (nv5 - 0.0));
        let eq53_e1397_d_n7: f64 = (eq53_e1395_d_n7 * (nv5 - 0.0));
        let eq53_e1397_d_n8: f64 = (eq53_e1395_d_n8 * (nv5 - 0.0));
        let eq53_e1397_d_n9: f64 = (eq53_e1395_d_n9 * (nv5 - 0.0));
        let eq53_e1398: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq53_e1397);
        let eq53_e1399: f64 = (-eq53_e1398);
        let eq53_e1399_d_n4: f64 = (-(eq53_e1397_d_n4 * ddt_scale));
        let eq53_e1399_d_n5: f64 = (-(eq53_e1395 * ddt_scale));
        let eq53_e1399_d_n6: f64 = (-(eq53_e1397_d_n6 * ddt_scale));
        let eq53_e1399_d_n7: f64 = (-(eq53_e1397_d_n7 * ddt_scale));
        let eq53_e1399_d_n8: f64 = (-(eq53_e1397_d_n8 * ddt_scale));
        let eq53_e1399_d_n9: f64 = (-(eq53_e1397_d_n9 * ddt_scale));
        let eq53_value: f64 = eq53_e1399;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq53_value),
            [4, 5, 6, 7, 8, 9],
            [multiplicity * (eq53_e1399_d_n4), multiplicity * (eq53_e1399_d_n5), multiplicity * (eq53_e1399_d_n6), multiplicity * (eq53_e1399_d_n7), multiplicity * (eq53_e1399_d_n8), multiplicity * (eq53_e1399_d_n9)],
            [],
            [],
            1.0,
        );
    }
}

#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_32(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2225] = (s.v[1929] > 0.0);
        s.v[2225] = if s.b[2225] { 1.0 } else { 0.0 };

        if ((s.b[2218] && s.b[2223]) && s.b[2225]) {
            s.store_offset_mul_offset_rhs_ad_rhs(1947, 1929, A::mul_scaled_output(s.ad_value(1929), A::scale_offset(s.ad_value(1929), 0.3333333333333333, 1.0), 0.5), 1.0, 1.0);
        }

        s.b[2226] = (s.v[1929] > (-230.25850929940458));
        s.v[2226] = if s.b[2226] { 1.0 } else { 0.0 };

        if (((s.b[2218] && s.b[2223]) && (!s.b[2225])) && s.b[2226]) {
            s.store_exp(1947, 1929);
        }

        if (((s.b[2218] && s.b[2223]) && (!s.b[2225])) && (!s.b[2226])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1947, 1e-100, (-230.25850929940458), 1929, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (s.b[2218] && s.b[2223]) {
            s.store_offset(1948, 1945, 3.0);
            s.store_sub_from_scalar(1949, (-3.0), 233);
            s.store_scale(1950, 826, 30.0);
            s.store_scalar(807, (4.0 - 0.9));
            s.store_add(808, 1948, 1950);
            s.store_mul_ad(1929, A::div_from_scalar(2.0, s.ad_value(807)), A::sub(s.ad_value(808), A::sqrt(A::sub(A::square(s.ad_value(808)), A::mul3(s.ad_value(807), s.ad_value(1948), s.ad_value(1950))))));
            s.store_scalar(807, (4.0 - 0.3));
            s.store_add(808, 1949, 1929);
            s.store_mul_ad(1951, A::div_from_scalar(2.0, s.ad_value(807)), A::add(s.ad_value(808), A::sqrt(A::sub(A::square(s.ad_value(808)), A::mul3(s.ad_value(807), s.ad_value(1949), s.ad_value(1929))))));
            s.store_mul3_lhs(829, 236, 1947, 1951);
        }

        s.b[2227] = (s.v[234] > 0.0);
        s.v[2227] = if s.b[2227] { 1.0 } else { 0.0 };

        s.b[2228] = (s.v[1813] <= 0.0);
        s.v[2228] = if s.b[2228] { 1.0 } else { 0.0 };

        if ((s.b[2218] && s.b[2227]) && s.b[2228]) {
            s.store_offset(1929, 766, 1.0);
            s.store_div_scaled_product_left_ad(1930, A::sqrt(s.ad_value(1929)), 815, 1.0, 1839, 1.0);
            s.store_add_ad_lhs(1931, A::square(s.ad_value(1930)), 1929);
            s.store_scale(1929, 1930, 2.0);
            s.store_div_scaled_product3_mixed_iiia(1842, 1839, 1809, 1929, 1.0, A::add(A::sqrt(A::sub(s.ad_value(1931), s.ad_value(1929))), A::sqrt(A::add(s.ad_value(1931), s.ad_value(1929)))), 1.0);
        }

        s.b[2229] = ((s.v[1843] - s.v[1842]) > (-230.25850929940458));
        s.v[2229] = if s.b[2229] { 1.0 } else { 0.0 };

        if ((s.b[2218] && s.b[2227]) && s.b[2229]) {
            s.store_exp_sub(1929, 1843, 1842);
        }

        if ((s.b[2218] && s.b[2227]) && (!s.b[2229])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1929, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1843), s.ad_value(1842)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1843), s.ad_value(1842)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (s.b[2218] && s.b[2227]) {
            s.store_add_scaled_product_right_ad(1952, 1932, 1.0, 1808, A::sub_scaled_inputs(s.ad_value(1843), 0.5, A::ln_scaled_input(A::offset(s.ad_value(1929), 1.0), 0.5), 1.0), 1.0);
            s.store_mul(1953, 233, 1808);
            s.store_add(1954, 1856, 1953);
            s.store_scaled_sub_sqrt_square_offset_rhs(1955, 1954, 1954, 0.01, 0.5);
            s.store_mul_sqrt_ad_lhs(1946, A::offset(A::square(s.ad_value(1856)), 1e-6), 784);
        }

        s.b[2230] = (s.v[239] < 0.0);
        s.v[2230] = if s.b[2230] { 1.0 } else { 0.0 };

        if ((s.b[2218] && s.b[2227]) && s.b[2230]) {
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1946, 1946, 0.5, 789, 0.5, A::offset(A::mul(A::sub(s.ad_value(1946), s.ad_value(789)), A::sub(s.ad_value(1946), s.ad_value(789))), 1e-6), (-0.5));
        }

        if (s.b[2218] && s.b[2227]) {
            s.store_add_scaled_product_left_ad(1956, 1846, 1.0, A::add_scaled_inputs3(s.ad_value(1955), 1.0, s.ad_value(731), (-1.0), s.ad_value(1952), -1.0), 1809, 1.0);
        }

        s.b[2231] = (((s.v[1956]) as f64).abs() < 230.25850929940458);
        s.v[2231] = if s.b[2231] { 1.0 } else { 0.0 };

        if ((s.b[2218] && s.b[2227]) && s.b[2231]) {
            s.store_exp(1957, 1956);
        }

        s.b[2232] = (s.v[1956] < 0.0);
        s.v[2232] = if s.b[2232] { 1.0 } else { 0.0 };

        if (((s.b[2218] && s.b[2227]) && (!s.b[2231])) && s.b[2232]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1957, 1e-100, (-230.25850929940458), 1956, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[2218] && s.b[2227]) && (!s.b[2231])) && (!s.b[2232])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(1957, 1956, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(1956), (-230.25850929940458), A::scale_offset(s.ad_value(1956), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (s.b[2218] && s.b[2227]) {
            s.store_mul_neg_ad_lhs(1956, A::add_scaled_inputs3(s.ad_value(814), 1.0, s.ad_value(1932), 1.0, s.ad_value(1952), -1.0), 1809);
        }

        s.b[2233] = (((s.v[1956]) as f64).abs() < 230.25850929940458);
        s.v[2233] = if s.b[2233] { 1.0 } else { 0.0 };

        if ((s.b[2218] && s.b[2227]) && s.b[2233]) {
            s.store_exp(1929, 1956);
        }

        s.b[2234] = (s.v[1956] < 0.0);
        s.v[2234] = if s.b[2234] { 1.0 } else { 0.0 };

        if (((s.b[2218] && s.b[2227]) && (!s.b[2233])) && s.b[2234]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1929, 1e-100, (-230.25850929940458), 1956, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((s.b[2218] && s.b[2227]) && (!s.b[2233])) && (!s.b[2234])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(1929, 1956, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(1956), (-230.25850929940458), A::scale_offset(s.ad_value(1956), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (s.b[2218] && s.b[2227]) {
            s.store_mul(1958, 1957, 1929);
            s.store_mul_offset_ad_rhs(1929, 786, A::mul(s.ad_value(1946), A::add_scaled_product(s.ad_value(238), 1.0, s.ad_value(239), s.ad_value(1946), 1.0)), (-1.5));
        }

        s.b[2235] = (s.v[1929] > 0.0);
        s.v[2235] = if s.b[2235] { 1.0 } else { 0.0 };

        if ((s.b[2218] && s.b[2227]) && s.b[2235]) {
            s.store_offset_mul_offset_rhs_ad_rhs(1947, 1929, A::mul_scaled_output(s.ad_value(1929), A::scale_offset(s.ad_value(1929), 0.3333333333333333, 1.0), 0.5), 1.0, 1.0);
        }

        s.b[2236] = (s.v[1929] > (-230.25850929940458));
        s.v[2236] = if s.b[2236] { 1.0 } else { 0.0 };

        if (((s.b[2218] && s.b[2227]) && (!s.b[2235])) && s.b[2236]) {
            s.store_exp(1947, 1929);
        }

        if (((s.b[2218] && s.b[2227]) && (!s.b[2235])) && (!s.b[2236])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1947, 1e-100, (-230.25850929940458), 1929, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (s.b[2218] && s.b[2227]) {
            s.store_mul_ad_product_rhs(1959, 234, s.ad_value(1947), A::ln(A::div_scaled_offset_numerator(s.ad_value(1957), 1.0, 1.0, A::offset(s.ad_value(1958), 1.0), 1.0)));
        }

        s.b[2237] = ((s.v[1813] <= 0.0) || ((s.v[238] == 0.0) && (s.v[239] == 0.0)));
        s.v[2237] = if s.b[2237] { 1.0 } else { 0.0 };

        if ((s.b[2218] && s.b[2227]) && s.b[2237]) {
            s.store_scalar(1966, 1.0);
            s.store_scalar(1967, 0.5);
        }

        if ((s.b[2218] && s.b[2227]) && (!s.b[2237])) {
            s.store_add_scaled_product_indices(1929, 238, 1.0, 239, 1946, 2.0);
            s.store_div_ad_rhs(1960, 244, A::mul(s.ad_value(1929), s.ad_value(786)));
            s.store_scaled_div(1961, 1844, 1960, 0.5);
            s.store_div(1962, 1960, 1861);
            s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1963, 1962, 1.0, 1962, 1.0, 0.5);
            s.store_sub_from_scalar_scaled_input(1964, 0.5, 1963, 3.0);
        }

        s.b[2238] = (s.v[1961] < 0.001);
        s.v[2238] = if s.b[2238] { 1.0 } else { 0.0 };

        if (((s.b[2218] && s.b[2227]) && (!s.b[2237])) && s.b[2238]) {
            s.store_square(1965, 1961);
            s.store_offset_mul_ad(1966, s.ad_value(1965), A::add_scaled_product(A::scale_offset(s.ad_value(1962), 0.3333333333333333, 0.16666666666666666), 1.0, s.ad_value(1965), A::scale_offset(s.ad_value(1962), 0.2, 0.05), 0.16666666666666666), 1.0);
            s.store_add_scaled_offset_product_rhs_mixed_iia(1967, 1966, 0.5, 1961, A::mul(s.ad_value(1965), A::add_scaled_offset_product_rhs(A::scaled_offset(s.ad_value(1963), 0.25, 0.4), 1.0, s.ad_value(1965), s.ad_value(1963), 0.125, 0.0285714285714)), 1.0, (-0.16666666666666666));
        }

        if (((s.b[2218] && s.b[2227]) && (!s.b[2237])) && (!s.b[2238])) {
            s.store_div_from_scalar(1968, 1.0, 1961);
        }

        s.b[2239] = (((s.v[1961]) as f64).abs() < 230.25850929940458);
        s.v[2239] = if s.b[2239] { 1.0 } else { 0.0 };

        if ((((s.b[2218] && s.b[2227]) && (!s.b[2237])) && (!s.b[2238])) && s.b[2239]) {
            s.store_exp(1969, 1961);
        }

        s.b[2240] = (s.v[1961] < 0.0);
        s.v[2240] = if s.b[2240] { 1.0 } else { 0.0 };

        if (((((s.b[2218] && s.b[2227]) && (!s.b[2237])) && (!s.b[2238])) && (!s.b[2239])) && s.b[2240]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1969, 1e-100, (-230.25850929940458), 1961, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2218] && s.b[2227]) && (!s.b[2237])) && (!s.b[2238])) && (!s.b[2239])) && (!s.b[2240])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(1969, 1961, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(1961), (-230.25850929940458), A::scale_offset(s.ad_value(1961), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[2218] && s.b[2227]) && (!s.b[2237])) && (!s.b[2238])) {
            s.store_div_from_scalar(1970, 1.0, 1969);
            s.store_sub(1929, 1969, 1970);
            s.store_add(1931, 1969, 1970);
            s.store_add_scaled_products_left_left_ad(1966, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(1962), s.ad_value(1929)), 1968, 0.5, 1962, 1931, 0.5);
            s.store_scaled_sub_ad(1967, A::add_scaled_product(s.ad_value(1966), 1.0, s.ad_value(1929), A::sub(s.ad_value(1963), A::mul3(s.ad_value(1964), s.ad_value(1968), s.ad_value(1968))), (-1.0)), A::mul3(s.ad_value(1964), s.ad_value(1931), s.ad_value(1968)), 0.5);
        }

        if (s.b[2218] && s.b[2227]) {
            s.store_scaled_offset_ad(1971, A::div(s.ad_value(1813), A::sqrt(A::offset(A::square(s.ad_value(1813)), 1e-6))), 1.0, 0.5);
            s.store_mul3_lhs(1972, 1959, 1966, 1971);
            s.store_mul3_lhs(831, 1959, 1967, 1971);
            s.store_sub(830, 1972, 831);
            s.store_mul_ad_product_rhs(832, 1959, s.ad_value(1966), A::sub_from_scalar(1.0, s.ad_value(1971)));
        }

        s.v[834] = 0.0;

        s.v[833] = 0.0;

        s.b[2241] = (p.p42 != 0.0);
        s.v[2241] = if s.b[2241] { 1.0 } else { 0.0 };

        s.b[2242] = ((s.v[246] > 0.0) && (s.v[1863] < 0.0));
        s.v[2242] = if s.b[2242] { 1.0 } else { 0.0 };

        if (s.b[2241] && s.b[2242]) {
            s.store_sqrt_offset_ad(1973, A::add_scaled_square_product(s.ad_value(1863), 1.0, A::square(s.ad_value(252)), A::square(s.ad_value(825)), 1.0), 1e-6);
            s.store_div_scaled_inputs_indices(1929, 796, -1.0, 1973, 1.0);
        }

        s.b[2243] = (s.v[1929] > (-230.25850929940458));
        s.v[2243] = if s.b[2243] { 1.0 } else { 0.0 };

        if ((s.b[2241] && s.b[2242]) && s.b[2243]) {
            s.store_exp(1931, 1929);
        }

        if ((s.b[2241] && s.b[2242]) && (!s.b[2243])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1931, 1e-100, (-230.25850929940458), 1929, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (s.b[2241] && s.b[2242]) {
            s.store_mul_ad_affine_product_lhs(834, s.ad_value(794), A::mul3(s.ad_value(825), s.ad_value(1863), s.ad_value(1973)), -1.0, 0.0, 1931);
        }

        s.b[2244] = ((s.v[245] > 0.0) && (s.v[1862] < 0.0));
        s.v[2244] = if s.b[2244] { 1.0 } else { 0.0 };

        if (s.b[2241] && s.b[2244]) {
            s.store_sqrt_offset_ad(1974, A::add_scaled_square_product(s.ad_value(1862), 1.0, A::square(s.ad_value(251)), A::square(s.ad_value(824)), 1.0), 1e-6);
            s.store_div_scaled_inputs_indices(1929, 795, -1.0, 1974, 1.0);
        }

        s.b[2245] = (s.v[1929] > (-230.25850929940458));
        s.v[2245] = if s.b[2245] { 1.0 } else { 0.0 };

        if ((s.b[2241] && s.b[2244]) && s.b[2245]) {
            s.store_exp(1931, 1929);
        }

        if ((s.b[2241] && s.b[2244]) && (!s.b[2245])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1931, 1e-100, (-230.25850929940458), 1929, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (s.b[2241] && s.b[2244]) {
            s.store_mul_ad_affine_product_lhs(833, s.ad_value(793), A::mul3(s.ad_value(824), s.ad_value(1862), s.ad_value(1974)), -1.0, 0.0, 1931);
        }

        s.copy_ad(1978, 1916);

        s.v[1864] = 0.0;

        s.v[1865] = 0.0;

        s.v[1866] = 0.0;

        s.v[1867] = 1e-40;

        s.v[1868] = 1.0;

        s.v[835] = 0.0;

        s.b[2246] = ((p.p46 != 0.0) && (s.v[285] > 0.0));
        s.v[2246] = if s.b[2246] { 1.0 } else { 0.0 };

        if s.b[2246] {
            s.store_add_scaled_inputs4_mixed_iiai(1929, 817, 0.5, 816, 0.5, A::sqrt(A::add_scaled_product(s.ad_value(753), 1.0, A::sub(s.ad_value(817), s.ad_value(816)), A::sub(s.ad_value(817), s.ad_value(816)), 1.0)), (-0.5), 751, 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(1975, 816, 1.0, 1929, (-0.5), A::sqrt(A::add_scaled_product(s.ad_value(752), 1.0, s.ad_value(1929), s.ad_value(1929), 1.0)), (-(-0.5)), 755, 1.0);
            s.store_add_scaled_inputs3_indices(1976, 1975, 1.0, 815, 0.5, 819, (-0.5));
            s.store_mul_ad_product_rhs(1977, 287, A::offset(A::mul(s.ad_value(289), s.ad_value(819)), 1.0), A::offset(A::mul(s.ad_value(288), s.ad_value(1976)), 1.0));
            s.store_mul_offset_rhs(1978, 1924, 1977, 1.0);
            s.store_div_from_scalar(1979, 1.0, 1978);
            s.store_div_scaled_value_offset_denominator(1980, s.ad_value(819), 2.0, A::sqrt(A::offset(A::mul(s.ad_value(291), s.ad_value(819)), 1.0)), 1.0, 1.0);
            s.store_mul_ad_product_rhs(1981, 290, s.ad_value(1980), A::offset(A::mul(s.ad_value(292), s.ad_value(1976)), 1.0));
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
        s.v[2247] = if s.b[2247] { 1.0 } else { 0.0 };

        if (s.b[2246] && s.b[2247]) {
            s.store_offset_add(1995, 1994, 1926, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(1996, 1995, 1995, 10.0, 0.5);
            s.store_add_ad_lhs(1997, A::add_scaled_product(s.ad_value(1994), 1.0, s.ad_value(1992), A::ln(s.ad_value(1996)), (-1.0)), 1926);
            s.store_scaled_add_sqrt_square_offset_rhs(1998, 1997, 1997, 2.0, 0.5);
        }

        s.b[2248] = ((s.v[1994] - s.v[1998]) < 230.25850929940458);
        s.v[2248] = if s.b[2248] { 1.0 } else { 0.0 };

        if ((s.b[2246] && s.b[2247]) && s.b[2248]) {
            s.store_exp_sub(1999, 1994, 1998);
        }

        if ((s.b[2246] && s.b[2247]) && (!s.b[2248])) {
            s.store_scaled_offset_mul_offset_lhs_ad(1999, A::sub(s.ad_value(1994), s.ad_value(1998)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(1994), s.ad_value(1998)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(1994), s.ad_value(1998)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (s.b[2246] && s.b[2247]) {
            s.store_mul(2000, 1925, 1999);
            s.store_pow_ad(2001, s.ad_value(2000), s.ad_value(1993));
            s.store_add_scaled_square_product_mixed_iai(2002, 1992, 1.0, A::add_scaled_inputs3(s.ad_value(1998), 2.0, s.ad_value(1992), 2.0, s.ad_value(2001), -1.0), 2001, 1.0);
            s.store_mul_offset_ad_rhs(2003, 1992, A::div_scaled_inputs2(A::sqrt(s.ad_value(2002)), 1.0, s.ad_value(1992), (-1.0), s.ad_value(2001), 1.0), (-1.0));
            s.store_sub(1985, 1998, 2003);
        }

        s.b[2249] = ((s.v[1993] * (s.v[1994] + s.v[1926])) > (-230.25850929940458));
        s.v[2249] = if s.b[2249] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_33(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[2246] && (!s.b[2247])) && s.b[2249]) {
            s.store_exp_ad(1985, A::mul(s.ad_value(1993), A::add(s.ad_value(1994), s.ad_value(1926))));
        }

        if ((s.b[2246] && (!s.b[2247])) && (!s.b[2249])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1985, 1e-100, (-230.25850929940458), A::mul(s.ad_value(1993), A::add(s.ad_value(1994), s.ad_value(1926))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::mul(s.ad_value(1993), A::add(s.ad_value(1994), s.ad_value(1926))), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if s.b[2246] {
            s.store_mul_add_rhs(1986, 1979, 1841, 1975);
        }

        s.b[2250] = ((s.v[1985] < 0.001) && (s.v[1841] < 1e-6));
        s.v[2250] = if s.b[2250] { 1.0 } else { 0.0 };

        s.b[2251] = (((-s.v[1986]) + s.v[1984]) > (-230.25850929940458));
        s.v[2251] = if s.b[2251] { 1.0 } else { 0.0 };

        if ((s.b[2246] && s.b[2250]) && s.b[2251]) {
            s.store_exp_sub(1929, 1984, 1986);
        }

        if ((s.b[2246] && s.b[2250]) && (!s.b[2251])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1929, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1984), s.ad_value(1986)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1984), s.ad_value(1986)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
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
        s.v[2252] = if s.b[2252] { 1.0 } else { 0.0 };

        if ((s.b[2246] && (!s.b[2250])) && s.b[2252]) {
            s.store_offset_add(1995, 1994, 1926, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(1996, 1995, 1995, 10.0, 0.5);
            s.store_add_ad_lhs(1997, A::add_scaled_product(s.ad_value(1994), 1.0, s.ad_value(1992), A::ln(s.ad_value(1996)), (-1.0)), 1926);
            s.store_scaled_add_sqrt_square_offset_rhs(1998, 1997, 1997, 2.0, 0.5);
        }

        s.b[2253] = ((s.v[1994] - s.v[1998]) < 230.25850929940458);
        s.v[2253] = if s.b[2253] { 1.0 } else { 0.0 };

        if (((s.b[2246] && (!s.b[2250])) && s.b[2252]) && s.b[2253]) {
            s.store_exp_sub(1999, 1994, 1998);
        }

        if (((s.b[2246] && (!s.b[2250])) && s.b[2252]) && (!s.b[2253])) {
            s.store_scaled_offset_mul_offset_lhs_ad(1999, A::sub(s.ad_value(1994), s.ad_value(1998)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(1994), s.ad_value(1998)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(1994), s.ad_value(1998)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if ((s.b[2246] && (!s.b[2250])) && s.b[2252]) {
            s.store_mul(2000, 1925, 1999);
            s.store_pow_ad(2001, s.ad_value(2000), s.ad_value(1993));
            s.store_add_scaled_square_product_mixed_iai(2002, 1992, 1.0, A::add_scaled_inputs3(s.ad_value(1998), 2.0, s.ad_value(1992), 2.0, s.ad_value(2001), -1.0), 2001, 1.0);
            s.store_mul_offset_ad_rhs(2003, 1992, A::div_scaled_inputs2(A::sqrt(s.ad_value(2002)), 1.0, s.ad_value(1992), (-1.0), s.ad_value(2001), 1.0), (-1.0));
            s.store_sub(1987, 1998, 2003);
        }

        s.b[2254] = ((s.v[1993] * (s.v[1994] + s.v[1926])) > (-230.25850929940458));
        s.v[2254] = if s.b[2254] { 1.0 } else { 0.0 };

        if (((s.b[2246] && (!s.b[2250])) && (!s.b[2252])) && s.b[2254]) {
            s.store_exp_ad(1987, A::mul(s.ad_value(1993), A::add(s.ad_value(1994), s.ad_value(1926))));
        }

        if (((s.b[2246] && (!s.b[2250])) && (!s.b[2252])) && (!s.b[2254])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1987, 1e-100, (-230.25850929940458), A::mul(s.ad_value(1993), A::add(s.ad_value(1994), s.ad_value(1926))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::mul(s.ad_value(1993), A::add(s.ad_value(1994), s.ad_value(1926))), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
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

        s.v[1869] = 0.0;

        s.v[836] = 0.0;

        s.b[2255] = ((s.v[1813] > 0.0) && (p.p41 != 0.0));
        s.v[2255] = if s.b[2255] { 1.0 } else { 0.0 };

        if s.b[2255] {
            s.store_add_scaled_product_indices(1988, 815, 1.0, 230, 1844, (-1.0));
        }

        s.b[2256] = (s.v[1988] > 0.0);
        s.v[2256] = if s.b[2256] { 1.0 } else { 0.0 };

        if (s.b[2255] && s.b[2256]) {
            s.store_mul_div_scaled_offset_numerator_rhs(1931, 713, A::mul(s.ad_value(231), A::sub(A::sqrt(A::add(s.ad_value(717), s.ad_value(1932))), s.ad_value(725))), 1.0, 1.0, A::offset(s.ad_value(1988), 1e-30), 1.0);
        }

        s.b[2257] = ((((-s.v[1931])) as f64).abs() < 230.25850929940458);
        s.v[2257] = if s.b[2257] { 1.0 } else { 0.0 };

        if ((s.b[2255] && s.b[2256]) && s.b[2257]) {
            s.store_exp_neg_input(1929, 1931);
        }

        s.b[2258] = ((-s.v[1931]) < 0.0);
        s.v[2258] = if s.b[2258] { 1.0 } else { 0.0 };

        if (((s.b[2255] && s.b[2256]) && (!s.b[2257])) && s.b[2258]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(1929, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1931)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(1931)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[2255] && s.b[2256]) && (!s.b[2257])) && (!s.b[2258])) {
            s.store_scaled_offset_mul_offset_lhs_ad(1929, A::neg(s.ad_value(1931)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(1931)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(1931)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (s.b[2255] && s.b[2256]) {
            s.store_mul3_lhs(1869, 227, 1988, 1929);
            s.store_mul_add_rhs(836, 1869, 827, 835);
        }

        s.b[2259] = (s.v[836] > (0.5 * s.v[232]));
        s.v[2259] = if s.b[2259] { 1.0 } else { 0.0 };

        if ((s.b[2255] && s.b[2256]) && s.b[2259]) {
            s.store_offset_div_scaled_inputs_indices(1929, 836, 2.0, 232, 1.0, (-1.0));
            s.store_mul_scaled_offset_ad_rhs(836, 232, 0.5, A::div(s.ad_value(1929), A::sqrt(A::offset(A::square(s.ad_value(1929)), 1.0))), 1.0);
        }

        s.b[2453] = (((p.p45 == 1.0) || (p.p47 > 0.0)) || (p.p48 > 0.0));
        s.v[2453] = if s.b[2453] { 1.0 } else { 0.0 };

        s.b[2454] = ((p.p45 > 0.0) || (p.p47 > 0.0));
        s.v[2454] = if s.b[2454] { 1.0 } else { 0.0 };

        if (s.b[2453] && s.b[2454]) {
            s.copy_ad(2294, 717);
            s.copy_ad(2295, 727);
            s.copy_ad(2296, 718);
            s.copy_ad(2297, 1804);
            s.copy_ad(2298, 1805);
            s.store_scalar(2302, 0.0);
        }

        s.b[2455] = (p.p47 > 0.0);
        s.v[2455] = if s.b[2455] { 1.0 } else { 0.0 };

        if ((s.b[2453] && s.b[2454]) && s.b[2455]) {
            s.store_add_scaled_inputs4_mixed_iiai(2297, 817, 0.5, 816, 0.5, A::sqrt(A::add_scaled_product(s.ad_value(738), 1.0, A::sub(s.ad_value(817), s.ad_value(816)), A::sub(s.ad_value(817), s.ad_value(816)), 1.0)), (-0.5), 736, 1.0);
            s.store_add_scaled_inputs4_mixed_iiai(1870, 816, 1.0, 2297, (-0.5), A::sqrt(A::add_scaled_product(s.ad_value(737), 1.0, s.ad_value(2297), s.ad_value(2297), 1.0)), (-(-0.5)), 739, 1.0);
            s.copy_ad(2298, 1870);
            s.copy_ad(2294, 734);
            s.copy_ad(2295, 737);
            s.copy_ad(2296, 735);
        }

        if (s.b[2453] && s.b[2454]) {
            s.store_add_scaled_inputs3_indices(2301, 818, 1.0, 2302, (-1.0), 701, -1.0);
            s.store_add_scaled_inputs3_indices(2303, 2298, 1.0, 815, 0.5, 819, (-0.5));
            s.store_scalar(2315, 1.0);
        }

        s.b[2456] = (s.v[188] > 0.0);
        s.v[2456] = if s.b[2456] { 1.0 } else { 0.0 };

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
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1930, 2309, 0.5, 2312, 0.5, A::offset(A::mul(A::sub(s.ad_value(2309), s.ad_value(2312)), A::sub(s.ad_value(2309), s.ad_value(2312))), 20.0), 0.5);
            s.store_add_scaled_inputs3_indices(1931, 2308, 2.0, 2307, (-2.0), 2310, -1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2313, 1930, 0.5, 1931, 0.5, A::offset(A::mul(A::sub(s.ad_value(1930), s.ad_value(1931)), A::sub(s.ad_value(1930), s.ad_value(1931))), 20.0), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1930, 2313, 0.5, 2310, 0.5, A::offset(A::mul(A::sub(s.ad_value(2313), s.ad_value(2310)), A::sub(s.ad_value(2313), s.ad_value(2310))), 5.0), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2314, 1930, 0.5, 2310, ((-1.0) * 0.5), A::offset(A::mul(A::sub_scaled_inputs(s.ad_value(1930), 1.0, s.ad_value(2310), -1.0), A::sub_scaled_inputs(s.ad_value(1930), 1.0, s.ad_value(2310), -1.0)), 20.0), 0.5);
            s.store_mul_offset_ad_rhs(1931, 703, A::div(s.ad_value(2314), s.ad_value(2310)), 1.0);
        }

        s.b[2457] = (s.v[1931] > (-230.25850929940458));
        s.v[2457] = if s.b[2457] { 1.0 } else { 0.0 };

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
            s.store_div_scaled_value_offset_denominator(2324, s.ad_value(819), 2.0, A::sqrt(A::offset(A::mul(s.ad_value(195), s.ad_value(819)), 1.0)), 1.0, 1.0);
            s.store_mul_ad_product_rhs(2325, 194, s.ad_value(2324), A::offset(A::mul(s.ad_value(196), s.ad_value(2303)), 1.0));
            s.store_mul(2326, 2294, 2320);
            s.store_sqrt_square_add(1930, 2297, 2295);
            s.store_sqrt_ad(1931, A::add_scaled_product(s.ad_value(2295), 1.0, A::sub(s.ad_value(2297), s.ad_value(2325)), A::sub(s.ad_value(2297), s.ad_value(2325)), 1.0));
            s.store_mul_add_scaled_inputs3_offset_rhs(2327, 2320, s.ad_value(2325), 0.5, s.ad_value(1930), 0.5, s.ad_value(1931), ((-1.0) * (0.5)), 0.0);
            s.store_add(2328, 2326, 2322);
            s.store_sub(2329, 2328, 2327);
        }

        s.b[2458] = (p.p45 > 0.0);
        s.v[2458] = if s.b[2458] { 1.0 } else { 0.0 };

        s.b[2459] = (((s.v[2329]) as f64).abs() < 1e-5);
        s.v[2459] = if s.b[2459] { 1.0 } else { 0.0 };

        if (((s.b[2453] && s.b[2454]) && s.b[2458]) && s.b[2459]) {
            s.store_offset_ad(2330, A::mul_sub_from_scalar_rhs(s.ad_value(2304), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2329), 1.0, A::scale(s.ad_value(2329), 0.3125), 0.5)), 1.0);
        }

        s.b[2460] = (s.v[2329] < 460.51701859880916);
        s.v[2460] = if s.b[2460] { 1.0 } else { 0.0 };

        if ((((s.b[2453] && s.b[2454]) && s.b[2458]) && (!s.b[2459])) && s.b[2460]) {
            s.store_exp_neg_input(2344, 2329);
        }

        if ((((s.b[2453] && s.b[2454]) && s.b[2458]) && (!s.b[2459])) && (!s.b[2460])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2344, 1e-200, 2329, (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2329), (-460.51701859880916), A::scale_offset(s.ad_value(2329), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
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
        s.v[2461] = if s.b[2461] { 1.0 } else { 0.0 };

        if ((s.b[2453] && s.b[2454]) && s.b[2461]) {
            s.store_offset_mul(2333, 2330, 2332, (-1.0));
            s.store_scaled_add_sqrt_square_offset_rhs(1929, 2333, 2333, 10.0, 0.5);
            s.store_sub_ad_rhs(2334, 2332, A::ln(s.ad_value(1929)));
            s.store_scaled_add_sqrt_square_offset_rhs(2335, 2334, 2334, 2.0, 0.5);
        }

        s.b[2462] = ((s.v[2332] - s.v[2335]) < 230.25850929940458);
        s.v[2462] = if s.b[2462] { 1.0 } else { 0.0 };

        if (((s.b[2453] && s.b[2454]) && s.b[2461]) && s.b[2462]) {
            s.store_exp_sub(1929, 2332, 2335);
        }

        if (((s.b[2453] && s.b[2454]) && s.b[2461]) && (!s.b[2462])) {
            s.store_scaled_offset_mul_offset_lhs_ad(1929, A::sub(s.ad_value(2332), s.ad_value(2335)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2332), s.ad_value(2335)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2332), s.ad_value(2335)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if ((s.b[2453] && s.b[2454]) && s.b[2461]) {
            s.store_div(2336, 1929, 2330);
            s.store_sub_ad_lhs(1929, A::scaled_offset(s.ad_value(2335), 1.0, 2.0), 2336);
        }

    }

    pub(super) fn stamp_transient_block_34(
        s: &mut Scratch,
    ) {
        s.b[2463] = (s.v[2336] > 1e-6);
        s.v[2463] = if s.b[2463] { 1.0 } else { 0.0 };

        if (((s.b[2453] && s.b[2454]) && s.b[2461]) && s.b[2463]) {
            s.store_mul_offset_ad_rhs(2337, 2330, A::sub(s.ad_value(2335), A::div_scaled_offset_numerator(A::sqrt(A::offset(A::mul(s.ad_value(2336), s.ad_value(1929)), 1.0)), 1.0, (-1.0), s.ad_value(2336), 1.0)), 1.0);
        }

        if (((s.b[2453] && s.b[2454]) && s.b[2461]) && (!s.b[2463])) {
            s.store_mul_ad_affine_product_rhs(2337, 2330, s.ad_value(2336), A::offset(A::mul_scaled_lhs(s.ad_value(1929), 0.25, s.ad_value(1929)), 1.0), 0.5, 0.0);
        }

        if ((s.b[2453] && s.b[2454]) && s.b[2461]) {
            s.store_add_scaled_inputs3_offset_mixed_iia(1929, 2323, 0.5, 2337, ((-1.0) * 0.5), A::sqrt(A::offset(A::mul_offset_lhs(A::sub(s.ad_value(2323), s.ad_value(2337)), (-2.0), A::offset(A::sub(s.ad_value(2323), s.ad_value(2337)), (-2.0))), 1.0)), 0.5, (2.0 * 0.5));
            s.store_mul_scaled_offset_ad_rhs(2338, 2305, 0.5, A::sqrt(A::offset(A::mul(A::div_from_scalar(4.0, s.ad_value(2305)), s.ad_value(1929)), 1.0)), (-1.0));
            s.store_div_add_scaled_inputs_rhs_indices(2339, 2338, 2338, 1.0, 2337, 1.0);
            s.store_add_scaled_product_indices(2329, 2328, 1.0, 2339, 2327, (-1.0));
        }

        if (s.b[2453] && s.b[2454]) {
            s.store_offset_scaled(2340, 2304, 0.7071067811865475, 1.0);
        }

        let (assign49240_e63215,) = {
    if (s.b[2453] && s.b[2454]) {
        let assign49240_e63213: f64 = (1e-5 * s.v[2340]);
        (assign49240_e63213,)
    } else {
        (s.v[2341],)
    }
};
        s.v[2341] = assign49240_e63215;

        if (s.b[2453] && s.b[2454]) {
            s.store_div_from_scalar(2342, 1.0, 2340);
            s.store_scalar(2449, 0.0);
            s.store_scalar(2343, 0.0);
        }

        s.b[2464] = (s.v[2329] < 460.51701859880916);
        s.v[2464] = if s.b[2464] { 1.0 } else { 0.0 };

        if ((s.b[2453] && s.b[2454]) && s.b[2464]) {
            s.store_exp_neg_input(2344, 2329);
        }

        if ((s.b[2453] && s.b[2454]) && (!s.b[2464])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2344, 1e-200, 2329, (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2329), (-460.51701859880916), A::scale_offset(s.ad_value(2329), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        s.b[2465] = (((s.v[2323]) as f64).abs() <= s.v[2341]);
        s.v[2465] = if s.b[2465] { 1.0 } else { 0.0 };

        if ((s.b[2453] && s.b[2454]) && s.b[2465]) {
            s.store_scaled_square(2429, 2342, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(2343, 2323, s.ad_value(2342), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2323), 1.0, s.ad_value(2344)), s.ad_value(2304), s.ad_value(2429)), 1.0));
        }

        s.b[2466] = (s.v[2323] < (-s.v[2341]));
        s.v[2466] = if s.b[2466] { 1.0 } else { 0.0 };

        if (((s.b[2453] && s.b[2454]) && (!s.b[2465])) && s.b[2466]) {
            s.store_neg(2431, 2323);
            s.store_scaled_mul(2432, 2431, 2342, 1.25);
            s.store_scaled_sub_offset_sqrt_square_offset(2433, 2432, 10.0, (-6.0), 64.0, 0.5);
            s.store_sub(2428, 2431, 2433);
            s.store_add_scaled_square_product_mixed_iia(2434, 2428, 1.0, 2305, A::offset(s.ad_value(2433), 1.0), 1.0);
            s.store_sub_scaled_inputs(2435, 2428, 2.0, 2305, 1.0);
            s.store_sub_ad_lhs(2436, A::ln(A::mul(s.ad_value(2434), s.ad_value(2321))), 2433);
            s.store_add(813, 2434, 2435);
            s.store_add_scaled_square_product_mixed_iia(812, 813, 1.0, 2436, A::sub_scaled_inputs(A::square(s.ad_value(2435)), 0.5, s.ad_value(2434), 1.0), 1.0);
            s.store_add_ad_rhs(2437, 2433, A::div_scaled_product3(s.ad_value(2434), s.ad_value(813), s.ad_value(2436), 1.0, A::add(s.ad_value(812), A::mul3(A::mul3(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2436), s.ad_value(2436)), s.ad_value(2435), A::sub_scaled_inputs(A::square(s.ad_value(2435)), 0.3333333333333333, s.ad_value(2434), 1.0))), 1.0));
        }

        s.b[2467] = (s.v[2437] < 230.25850929940458);
        s.v[2467] = if s.b[2467] { 1.0 } else { 0.0 };

        if ((((s.b[2453] && s.b[2454]) && (!s.b[2465])) && s.b[2466]) && s.b[2467]) {
            s.store_exp(2438, 2437);
        }

        if ((((s.b[2453] && s.b[2454]) && (!s.b[2465])) && s.b[2466]) && (!s.b[2467])) {
            s.store_scaled_offset_mul_offset_lhs_ad_rhs(2438, 2437, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2437), (-230.25850929940458), A::scale_offset(s.ad_value(2437), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[2453] && s.b[2454]) && (!s.b[2465])) && s.b[2466]) {
            s.store_div_from_scalar(2439, 1.0, 2438);
            s.store_div_from_scalar_offset_ad(2428, 1.0, A::square(s.ad_value(2437)), 2.0);
            s.store_mul_square_lhs(2440, 2437, 2428);
            s.store_mul3_affine_lhs(2441, 2437, 2428, 4.0, 0.0, 2428);
            s.store_mul_ad_product_lhs(2442, A::sub_scaled_inputs(s.ad_value(2428), 8.0, s.ad_value(2440), 12.0), s.ad_value(2428), 2428);
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
            s.store_mul_ad_product_rhs(2447, 2323, s.ad_value(2342), A::offset(A::mul(s.ad_value(2446), s.ad_value(2323)), 1.0));
        }

        s.b[2468] = ((-s.v[2447]) > (-230.25850929940458));
        s.v[2468] = if s.b[2468] { 1.0 } else { 0.0 };

        if ((((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) && s.b[2468]) {
            s.store_exp_neg_input(2428, 2447);
        }

        if ((((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) && (!s.b[2468])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(2428, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2447)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(2447)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) {
            s.store_sub_from_scalar(2448, 1.0, 2428);
            s.store_add_scaled_inputs_product_right_ad(2449, 2323, 1.0, 2305, 0.5, 2304, A::sqrt(A::add_scaled_inputs3(s.ad_value(2323), 1.0, s.ad_value(2305), 0.25, s.ad_value(2448), -1.0)), (-1.0));
            s.store_offset(2450, 2329, 3.0);
            s.store_sub_ad(2433, A::add_scaled_inputs3(s.ad_value(2449), 0.5, s.ad_value(2450), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2449), s.ad_value(2450)), A::sub(s.ad_value(2449), s.ad_value(2450))), 5.0)), (-0.5)), A::sub_scaled_inputs(s.ad_value(2450), 0.5, A::sqrt(A::offset(A::square(s.ad_value(2450)), 5.0)), 0.5));
            s.store_sub(2428, 2323, 2433);
            s.store_exp_neg_input(2429, 2433);
            s.store_div_from_scalar_offset_ad(2430, 1.0, A::square(s.ad_value(2433)), 2.0);
            s.store_mul_square_lhs(2440, 2433, 2430);
            s.store_mul3_affine_lhs(2441, 2433, 2430, 4.0, 0.0, 2430);
            s.store_mul_ad_product_lhs(2442, A::sub_scaled_inputs(s.ad_value(2430), 8.0, s.ad_value(2440), 12.0), s.ad_value(2430), 2430);
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
            s.store_add_scaled_square_product_mixed_iia(812, 813, 1.0, 2436, A::add_scaled_square_product(s.ad_value(2435), 0.5, s.ad_value(2434), s.ad_value(2451), (-1.0)), 1.0);
            s.store_add_ad_rhs(2452, 2433, A::div_scaled_product3(s.ad_value(2434), s.ad_value(813), s.ad_value(2436), 1.0, A::add(s.ad_value(812), A::mul3(A::mul3(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2436), s.ad_value(2436)), s.ad_value(2435), A::add_scaled_square_product(s.ad_value(2435), 0.3333333333333333, s.ad_value(2434), s.ad_value(2451), (-1.0)))), 1.0));
        }

        s.b[2469] = (s.v[2452] < 230.25850929940458);
        s.v[2469] = if s.b[2469] { 1.0 } else { 0.0 };

        if ((((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) && s.b[2469]) {
            s.store_exp(2438, 2452);
            s.store_div_from_scalar(2439, 1.0, 2438);
            s.store_mul(2438, 2344, 2438);
        }

        s.b[2470] = (s.v[2452] > (s.v[2329] - 230.25850929940458));
        s.v[2470] = if s.b[2470] { 1.0 } else { 0.0 };

        if (((((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) && (!s.b[2469])) && s.b[2470]) {
            s.store_exp_sub(2438, 2452, 2329);
            s.store_div(2439, 2344, 2438);
        }

        if (((((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) && (!s.b[2469])) && (!s.b[2470])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(2438, 1e-100, A::sub(s.ad_value(2329), s.ad_value(2452)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2329), s.ad_value(2452)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2329), s.ad_value(2452)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2439, 1e-100, 2452, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2452), (-230.25850929940458), A::scale_offset(s.ad_value(2452), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if (((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) {
            s.store_div_from_scalar_offset_ad(2428, 1.0, A::square(s.ad_value(2452)), 2.0);
            s.store_mul_square_lhs(2440, 2452, 2428);
            s.store_mul3_affine_lhs(2441, 2452, 2428, 4.0, 0.0, 2428);
            s.store_mul_ad_product_lhs(2442, A::sub_scaled_inputs(s.ad_value(2428), 8.0, s.ad_value(2440), 12.0), s.ad_value(2428), 2428);
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
        s.v[2471] = if s.b[2471] { 1.0 } else { 0.0 };

        if ((s.b[2453] && s.b[2454]) && s.b[2471]) {
            s.store_div_from_scalar_offset_ad(1929, 1.0, A::square(s.ad_value(2343)), 2.0);
            s.store_mul_square_lhs(2345, 2343, 1929);
            s.store_mul3_affine_lhs(2346, 2343, 1929, 4.0, 0.0, 1929);
            s.store_mul_ad_product_lhs(2347, A::sub_scaled_inputs(s.ad_value(1929), 8.0, s.ad_value(2345), 12.0), s.ad_value(1929), 1929);
            s.store_scalar(2348, 0.0);
        }

        s.b[2472] = (s.v[2343] < 230.25850929940458);
        s.v[2472] = if s.b[2472] { 1.0 } else { 0.0 };

        if (((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2472]) {
            s.store_exp(2348, 2343);
            s.store_div_from_scalar(2349, 1.0, 2348);
            s.store_mul(2348, 2344, 2348);
        }

        s.b[2473] = (s.v[2343] > (s.v[2329] - 230.25850929940458));
        s.v[2473] = if s.b[2473] { 1.0 } else { 0.0 };

        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && (!s.b[2472])) && s.b[2473]) {
            s.store_exp_sub(2348, 2343, 2329);
            s.store_div(2349, 2344, 2348);
        }

        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && (!s.b[2472])) && (!s.b[2473])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(2348, 1e-100, A::sub(s.ad_value(2329), s.ad_value(2343)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2329), s.ad_value(2343)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2329), s.ad_value(2343)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2349, 1e-100, 2343, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2343), (-230.25850929940458), A::scale_offset(s.ad_value(2343), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if ((s.b[2453] && s.b[2454]) && s.b[2471]) {
            s.store_add_scaled_product_right_ad(2350, 2348, 1.0, 2344, A::add(A::offset(s.ad_value(2343), 1.0), s.ad_value(2345)), (-1.0));
        }

        s.b[2474] = (s.v[2343] < 1e-5);
        s.v[2474] = if s.b[2474] { 1.0 } else { 0.0 };

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
        }

    }

    pub(super) fn stamp_transient_block_35(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[2453] && s.b[2454]) && s.b[2471]) && (!s.b[2474])) {
            s.store_offset_scaled_ad(2353, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2304), 1.0, s.ad_value(2349)), s.ad_value(2352)), 0.5, 1.0);
        }

        if ((s.b[2453] && s.b[2454]) && s.b[2471]) {
            s.store_div_scaled_offset_numerator(2354, A::mul_scaled_lhs(s.ad_value(709), 0.2, s.ad_value(2303)), 1.0, 1.0, A::offset(A::mul(s.ad_value(709), s.ad_value(2303)), 1.0), 1.0);
        }

        s.b[2475] = (s.v[2350] > 1e-100);
        s.v[2475] = if s.b[2475] { 1.0 } else { 0.0 };

        if (((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) {
            s.store_mul_sqrt_ad_rhs(2355, 2304, A::add(s.ad_value(2351), s.ad_value(2350)));
            s.store_div_scaled_product3_mixed_iiia(2356, 2305, 2350, 2319, 1.0, A::add_scaled_product(s.ad_value(2355), 1.0, s.ad_value(2304), s.ad_value(2352), 1.0), 1.0);
            s.store_mul3_lhs(2357, 2352, 2304, 2319);
        }

        s.b[2476] = (s.v[215] < 0.0);
        s.v[2476] = if s.b[2476] { 1.0 } else { 0.0 };

        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) && s.b[2476]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2358, 1.0, 1.0, A::mul(s.ad_value(215), s.ad_value(2303)));
        }

        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) && (!s.b[2476])) {
            s.store_offset_mul(2358, 215, 2303, 1.0);
        }

        s.b[2477] = (s.v[216] < 0.0);
        s.v[2477] = if s.b[2477] { 1.0 } else { 0.0 };

        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) && s.b[2477]) {
            s.store_sub_from_scalar_scaled_mul(2359, 1.0, 216, 2356, 1.0);
        }

        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) && (!s.b[2477])) {
            s.store_div_from_scalar_offset_ad(2359, 1.0, A::mul(s.ad_value(216), s.ad_value(2356)), 1.0);
        }

        if (((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) {
            s.store_mul_product3_rhs(2360, 2356, s.ad_value(746), s.ad_value(2358), s.ad_value(2359), 1.0);
            s.store_mul_add_scaled_product_rhs(2361, 763, s.ad_value(2357), 1.0, s.ad_value(764), s.ad_value(2356), 1.0);
            s.store_ln_ad(1930, A::div_scaled_value_offset_denominator(s.ad_value(2351), 1.0, A::add(s.ad_value(2351), s.ad_value(2350)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2362, A::pow(A::mul(s.ad_value(2361), s.ad_value(705)), s.ad_value(706)), 1.0, 707, A::exp(A::mul_scaled_lhs(s.ad_value(708), 0.5, s.ad_value(1930))), 1.0);
            s.store_mul_add_ad_lhs(2363, A::offset(s.ad_value(2362), 1.0), s.ad_value(2360), 2354);
        }

        s.b[2478] = (s.v[219] < 0.0);
        s.v[2478] = if s.b[2478] { 1.0 } else { 0.0 };

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
        s.v[2479] = if s.b[2479] { 1.0 } else { 0.0 };

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
        }

        let (assign50880_e65723,) = {
    if (s.b[2453] && (!s.b[2454])) {
        (s.v[1817],)
    } else {
        (s.v[2341],)
    }
};
        s.v[2341] = assign50880_e65723;

        if (s.b[2453] && (!s.b[2454])) {
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
        s.v[2480] = if s.b[2480] { 1.0 } else { 0.0 };

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
        s.v[2481] = if s.b[2481] { 1.0 } else { 0.0 };

        s.b[2482] = (s.v[2350] > 1e-100);
        s.v[2482] = if s.b[2482] { 1.0 } else { 0.0 };

        if ((s.b[2453] && s.b[2481]) && s.b[2482]) {
            s.store_mul(2427, 2299, 2366);
            s.store_div(2368, 2427, 2363);
            s.store_add_scaled_inputs(2369, 2355, 1.0, 2305, 0.5);
            s.store_div_scaled_product_by_product(1929, s.ad_value(2305), s.ad_value(2348), 1.0, s.ad_value(2369), s.ad_value(2369), 1.0);
        }

        s.b[2483] = (s.v[1929] > 0.0001);
        s.v[2483] = if s.b[2483] { 1.0 } else { 0.0 };

        if (((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2483]) {
            s.store_sub_from_scalar(1930, 1.0, 1929);
        }

        s.b[2484] = (s.v[1930] < 1e-10);
        s.v[2484] = if s.b[2484] { 1.0 } else { 0.0 };

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
        s.v[2485] = if s.b[2485] { 1.0 } else { 0.0 };

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
            s.store_mul_div_scaled_product_rhs(1931, 2376, s.ad_value(708), A::add(A::offset(s.ad_value(2374), (-1.0)), A::div_scalar_offset_denominator(1.0, s.ad_value(1929), 1.0, 1.0)), 1.0, s.ad_value(2373), 1.0);
            s.store_mul_product3_rhs(2377, 2372, s.ad_value(746), s.ad_value(2358), s.ad_value(2359), 1.0);
            s.store_offset_ad(1929, A::div_scaled_add_product(s.ad_value(1930), 1.0, A::mul3(s.ad_value(746), s.ad_value(2358), s.ad_value(2359)), s.ad_value(2374), (-1.0), s.ad_value(1931), 1.0), 1.0);
        }

        s.b[2486] = (s.v[1929] < 230.25850929940458);
        s.v[2486] = if s.b[2486] { 1.0 } else { 0.0 };

        if ((((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2485]) && s.b[2486]) {
            s.store_scaled_ln_one_plus_exp_scaled_input(1930, 1929, 2.0, 0.5);
        }

        if ((((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2485]) && (!s.b[2486])) {
            s.copy_ad(1930, 1929);
        }

        if (((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2485]) {
            s.store_div_scaled_product3_mixed_iiia(2378, 2371, 1931, 1930, -1.0, A::add_scaled_inputs3_offset(s.ad_value(2375), 1.0, s.ad_value(2376), 1.0, s.ad_value(2377), 1.0, 1.0), 1.0);
            s.store_mul_offset_ad_rhs(2379, 2370, A::div_scaled_value_offset_denominator(s.ad_value(2378), 1.0, A::sqrt(A::offset(A::square(s.ad_value(2378)), 1.0)), 1.0, 1.0), 1.0);
        }

        if (((s.b[2453] && s.b[2481]) && s.b[2482]) && (!s.b[2485])) {
            s.copy_ad(2379, 2370);
        }

        if ((s.b[2453] && s.b[2481]) && s.b[2482]) {
            s.store_mul3_affine_lhs(2380, 2319, 2368, 0.7071067811865475, 0.0, 2379);
        }

        s.b[2487] = (s.v[0] == (-1.0));
        s.v[2487] = if s.b[2487] { 1.0 } else { 0.0 };

        if (((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2487]) {
            s.store_div_ad_rhs(2380, 2380, A::sqrt(A::offset(s.ad_value(2380), 1.0)));
        }

        if ((s.b[2453] && s.b[2481]) && s.b[2482]) {
            s.store_div_from_scalar_offset_ad(2381, 2.0, A::sqrt(A::scale_offset(s.ad_value(2380), 4.0, 1.0)), 1.0);
            s.store_mul(1929, 2381, 2380);
            s.store_mul_ad_product_rhs(2382, 2379, s.ad_value(2381), A::offset(A::div(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1929), 1.0, A::mul(s.ad_value(1929), s.ad_value(2381)), 0.86), A::offset(A::mul3_scaled_output(s.ad_value(1929), s.ad_value(1929), s.ad_value(2381), 4.0), 1.0)), 1.0));
        }

    }

    pub(super) fn stamp_transient_block_36(
        s: &mut Scratch,
    ) {
        if ((s.b[2453] && s.b[2481]) && s.b[2482]) {
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
        s.v[2488] = if s.b[2488] { 1.0 } else { 0.0 };

        if ((s.b[2453] && s.b[2481]) && s.b[2488]) {
            s.store_exp_neg_input(2388, 2386);
        }

        if ((s.b[2453] && s.b[2481]) && (!s.b[2488])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2388, 1e-200, 2386, (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2386), (-460.51701859880916), A::scale_offset(s.ad_value(2386), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if (s.b[2453] && s.b[2481]) {
            s.store_mul(2389, 2344, 2388);
        }

        s.b[2489] = (((s.v[2323]) as f64).abs() <= s.v[2341]);
        s.v[2489] = if s.b[2489] { 1.0 } else { 0.0 };

        if ((s.b[2453] && s.b[2481]) && s.b[2489]) {
            s.store_scaled_square(2429, 2342, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(2390, 2323, s.ad_value(2342), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2323), 1.0, s.ad_value(2389)), s.ad_value(2304), s.ad_value(2429)), 1.0));
        }

        if ((s.b[2453] && s.b[2481]) && (!s.b[2489])) {
            s.store_offset(2450, 2387, 3.0);
            s.store_sub_ad(2433, A::add_scaled_inputs3(s.ad_value(2449), 0.5, s.ad_value(2450), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2449), s.ad_value(2450)), A::sub(s.ad_value(2449), s.ad_value(2450))), 5.0)), (-0.5)), A::sub_scaled_inputs(s.ad_value(2450), 0.5, A::sqrt(A::offset(A::square(s.ad_value(2450)), 5.0)), 0.5));
            s.store_sub(2428, 2323, 2433);
            s.store_exp_neg_input(2429, 2433);
            s.store_div_from_scalar_offset_ad(2430, 1.0, A::square(s.ad_value(2433)), 2.0);
            s.store_mul_square_lhs(2440, 2433, 2430);
            s.store_mul3_affine_lhs(2441, 2433, 2430, 4.0, 0.0, 2430);
            s.store_mul_ad_product_lhs(2442, A::sub_scaled_inputs(s.ad_value(2430), 8.0, s.ad_value(2440), 12.0), s.ad_value(2430), 2430);
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
        s.v[2490] = if s.b[2490] { 1.0 } else { 0.0 };

        if (((s.b[2453] && s.b[2481]) && (!s.b[2489])) && s.b[2490]) {
            s.store_exp(2438, 2452);
            s.store_div_from_scalar(2439, 1.0, 2438);
            s.store_mul(2438, 2389, 2438);
        }

        s.b[2491] = (s.v[2452] > (s.v[2387] - 230.25850929940458));
        s.v[2491] = if s.b[2491] { 1.0 } else { 0.0 };

        if ((((s.b[2453] && s.b[2481]) && (!s.b[2489])) && (!s.b[2490])) && s.b[2491]) {
            s.store_exp_sub(2438, 2452, 2387);
            s.store_div(2439, 2389, 2438);
        }

        if ((((s.b[2453] && s.b[2481]) && (!s.b[2489])) && (!s.b[2490])) && (!s.b[2491])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(2438, 1e-100, A::sub(s.ad_value(2387), s.ad_value(2452)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2387), s.ad_value(2452)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2387), s.ad_value(2452)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2439, 1e-100, 2452, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2452), (-230.25850929940458), A::scale_offset(s.ad_value(2452), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        if ((s.b[2453] && s.b[2481]) && (!s.b[2489])) {
            s.store_div_from_scalar_offset_ad(2428, 1.0, A::square(s.ad_value(2452)), 2.0);
            s.store_mul_square_lhs(2440, 2452, 2428);
            s.store_mul3_affine_lhs(2441, 2452, 2428, 4.0, 0.0, 2428);
            s.store_mul_ad_product_lhs(2442, A::sub_scaled_inputs(s.ad_value(2428), 8.0, s.ad_value(2440), 12.0), s.ad_value(2428), 2428);
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
        s.v[2492] = if s.b[2492] { 1.0 } else { 0.0 };

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
        s.v[2493] = if s.b[2493] { 1.0 } else { 0.0 };

        if ((s.b[2453] && s.b[2481]) && s.b[2493]) {
            s.store_exp_neg_input(2396, 2390);
        }

        s.b[2494] = (s.v[2390] < 1e-5);
        s.v[2494] = if s.b[2494] { 1.0 } else { 0.0 };

        if (((s.b[2453] && s.b[2481]) && s.b[2493]) && s.b[2494]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2397, 2390, 1.0, 2390, 1.0, 2390, 0.25, 0.3333333333333333, 0.5);
            s.store_sqrt_sub_from_scalar_ad(1929, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2390), 1.0, A::scale(s.ad_value(2390), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2398, 2390, 1929, 0.7071067811865475);
            s.store_mul3_ad_middle(2399, A::mul3_scaled_output(s.ad_value(2389), s.ad_value(2390), s.ad_value(2390), 0.16666666666666666), 2390, A::scale_offset(s.ad_value(2390), 1.75, 1.0));
        }

        if (((s.b[2453] && s.b[2481]) && s.b[2493]) && (!s.b[2494])) {
            s.store_add_offset_lhs(2397, 2390, (-1.0), 2396);
            s.store_sqrt(2398, 2397);
            s.store_mul_add_scaled_inputs3_offset_rhs(2399, 2389, A::div_from_scalar(1.0, s.ad_value(2396)), 1.0, s.ad_value(2390), (-1.0), s.ad_value(2395), -1.0, (-1.0));
        }

        s.b[2495] = (s.v[2390] > (s.v[2387] - 230.25850929940458));
        s.v[2495] = if s.b[2495] { 1.0 } else { 0.0 };

        if (((s.b[2453] && s.b[2481]) && (!s.b[2493])) && s.b[2495]) {
            s.store_exp_sub(1929, 2390, 2387);
            s.store_div(2396, 2389, 1929);
            s.store_add_scaled_product_right_ad(2399, 1929, 1.0, 2389, A::add(A::offset(s.ad_value(2390), 1.0), s.ad_value(2395)), (-1.0));
        }

        if (((s.b[2453] && s.b[2481]) && (!s.b[2493])) && (!s.b[2495])) {
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2396, 1e-100, 2390, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2390), (-230.25850929940458), A::scale_offset(s.ad_value(2390), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
            s.store_div_from_scalar_offset_mul_offset_lhs_ad(1929, 1e-100, A::sub(s.ad_value(2387), s.ad_value(2390)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2387), s.ad_value(2390)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2387), s.ad_value(2390)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
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
        s.v[2496] = if s.b[2496] { 1.0 } else { 0.0 };

        if ((s.b[2453] && s.b[2481]) && s.b[2496]) {
            s.store_sqrt(2402, 1929);
        }

        if (s.b[2453] && s.b[2481]) {
            s.store_scaled_add(2403, 2350, 2399, 0.5);
            s.store_add_scaled_product_mixed_iaa(2404, 2403, 1.0, A::square(s.ad_value(2391)), A::sub_scaled_inputs(s.ad_value(2402), 1.0, s.ad_value(2321), 2.0), 0.125);
        }

        s.b[2497] = (s.v[2401] < 1e-5);
        s.v[2497] = if s.b[2497] { 1.0 } else { 0.0 };

        if ((s.b[2453] && s.b[2481]) && s.b[2497]) {
            s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2405, 2401, 1.0, 2401, 1.0, 2401, 0.25, 0.3333333333333333, 0.5);
            s.store_mul_sqrt_ad_rhs(2406, 2304, A::add(s.ad_value(2404), s.ad_value(2405)));
        }

        s.b[2498] = (s.v[719] > 0.0);
        s.v[2498] = if s.b[2498] { 1.0 } else { 0.0 };

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
        s.v[2499] = if s.b[2499] { 1.0 } else { 0.0 };

        if (((s.b[2453] && s.b[2481]) && (!s.b[2497])) && s.b[2499]) {
            s.store_add_scaled_sub_value_product_indices(2410, 1.0, 2402, 1.0, 2406, 2321, 2.0);
            s.store_div_from_scalar_sqrt_ad(2407, 1.0, A::offset(A::mul(s.ad_value(719), s.ad_value(2406)), 1.0));
            s.store_div_scaled_value_offset_denominator(1929, s.ad_value(2407), 1.0, s.ad_value(2407), 1.0, 1.0);
            s.store_mul_product3_rhs(2411, 719, A::square(s.ad_value(1929)), s.ad_value(2305), s.ad_value(2404), 1.0);
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
            s.store_mul_div_scaled_product_rhs(2418, 2319, s.ad_value(2305), s.ad_value(2404), 1.0, A::add_scaled_product(s.ad_value(2406), 1.0, s.ad_value(2304), s.ad_value(2408), 1.0), 1.0);
            s.store_add_scaled_product_indices(2419, 2418, 1.0, 2319, 2409, 1.0);
            s.store_mul3_lhs(2420, 2408, 2304, 2319);
        }

        s.b[2500] = (s.v[216] < 0.0);
        s.v[2500] = if s.b[2500] { 1.0 } else { 0.0 };

        if ((s.b[2453] && s.b[2481]) && s.b[2500]) {
            s.store_sub_from_scalar_scaled_mul(2359, 1.0, 216, 2418, 1.0);
        }

        if ((s.b[2453] && s.b[2481]) && (!s.b[2500])) {
            s.store_div_from_scalar_offset_ad(2359, 1.0, A::mul(s.ad_value(216), s.ad_value(2418)), 1.0);
        }

        if (s.b[2453] && s.b[2481]) {
            s.store_mul_product3_rhs(2360, 2418, s.ad_value(746), s.ad_value(2358), s.ad_value(2359), 1.0);
            s.store_add_scaled_product_indices(2421, 2420, 1.0, 764, 2418, 1.0);
            s.store_add_scaled_product_indices(2422, 2420, 1.0, 765, 2418, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_37(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[2453] && s.b[2481]) {
            s.store_mul(2423, 763, 2421);
            s.store_ln_ad(1930, A::div_scaled_value_offset_denominator(s.ad_value(2405), 1.0, A::add(s.ad_value(2405), s.ad_value(2404)), 1e-14, 1.0));
            s.store_add_scaled_product_mixed_aia(2362, A::pow(A::mul(s.ad_value(2423), s.ad_value(705)), s.ad_value(706)), 1.0, 707, A::exp(A::mul_scaled_lhs(s.ad_value(708), 0.5, s.ad_value(1930))), 1.0);
            s.store_mul_add_ad_lhs(2424, A::offset(s.ad_value(2362), 1.0), s.ad_value(2360), 2354);
            s.store_ln_ad(2425, A::div_scaled_offset_numerator(A::mul(A::sub(s.ad_value(815), s.ad_value(2394)), s.ad_value(768)), 1.0, 1.0, A::offset(A::mul(A::sub(s.ad_value(2385), s.ad_value(2394)), s.ad_value(768)), 1.0), 1.0));
            s.store_mul(1931, 2418, 2364);
            s.store_div_add_scaled_inputs_rhs_indices(2365, 1931, 221, 1.0, 1931, 1.0);
        }

        s.b[2501] = (s.v[220] < 0.0);
        s.v[2501] = if s.b[2501] { 1.0 } else { 0.0 };

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
        }

        let (assign53260_e68676,) = {
    if s.b[2453] {
        (s.v[2323],)
    } else {
        (s.v[1874],)
    }
};
        s.v[1874] = assign53260_e68676;

        if s.b[2453] {
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
        }

        let (assign53440_e68753,) = {
    if (!s.b[2453]) {
        (s.v[1813],)
    } else {
        (s.v[1874],)
    }
};
        s.v[1874] = assign53440_e68753;

        if (!s.b[2453]) {
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
        s.v[2502] = if s.b[2502] { 1.0 } else { 0.0 };

        if s.b[2502] {
            s.store_div_scaled_value_offset_denominator(1888, s.ad_value(253), 1.0, A::mul(s.ad_value(762), A::powf(A::add(A::square(s.ad_value(1883)), s.ad_value(722)), ((-1.0) * 0.16666666666666666))), 1.0, 1.0);
        }

        s.v[1889] = 1.0;

        s.v[1890] = 1.0;

        s.v[1891] = 0.0;

        s.v[1892] = 1.0;

        s.v[1893] = 1.0;

        s.copy_ad(2265, 1887);

        s.v[2268] = 0.0;

        s.v[2267] = 0.0;

        s.copy_ad(2269, 2265);

        s.b[2503] = (s.v[1874] > 0.0);
        s.v[2503] = if s.b[2503] { 1.0 } else { 0.0 };

        if s.b[2503] {
            s.store_mul_div_scaled_product_rhs(2260, 1885, A::add(s.ad_value(258), A::div(s.ad_value(259), s.ad_value(1882))), s.ad_value(1881), 1.0, s.ad_value(1882), 1.0);
        }

        s.b[2504] = (s.v[2260] > 0.0);
        s.v[2504] = if s.b[2504] { 1.0 } else { 0.0 };

        if (s.b[2503] && s.b[2504]) {
            s.store_div_from_scalar_add_ad(1889, 1.0, A::offset(s.ad_value(2260), 1.0), A::square(s.ad_value(2260)));
        }

        if (s.b[2503] && (!s.b[2504])) {
            s.store_sub_from_scalar(1889, 1.0, 2260);
        }

        if s.b[2503] {
            s.store_mul(1890, 1884, 1889);
            s.store_div(1891, 1886, 1890);
            s.store_mul_ad_product_lhs(2261, A::square(s.ad_value(1891)), s.ad_value(1877), 1877);
        }

        s.b[2505] = (s.v[0] == (-1.0));
        s.v[2505] = if s.b[2505] { 1.0 } else { 0.0 };

        if (s.b[2503] && s.b[2505]) {
            s.store_div_scaled_value_offset_denominator(2261, s.ad_value(2261), 1.0, A::mul(s.ad_value(1891), s.ad_value(1877)), 1.0, 1.0);
        }

        if s.b[2503] {
            s.store_mul_offset_rhs_scaled_ad_rhs(1892, 1890, A::sqrt(A::scale_offset(s.ad_value(2261), 2.0, 1.0)), 1.0, 0.5);
            s.store_div(1929, 1890, 1892);
            s.store_mul_offset_ad_rhs(2262, 1880, A::mul3_scaled_output(s.ad_value(2261), s.ad_value(1929), s.ad_value(1929), 0.5), 1.0);
            s.store_div_scaled_product_indices(1893, 1929, 1882, 1.0, 2262, 1.0);
            s.store_scaled_div(2263, 1877, 1893, 0.5);
            s.store_square(2264, 2263);
            s.store_add_ad_rhs(2265, 1887, A::mul3_scaled_output(s.ad_value(1879), s.ad_value(1877), A::add(A::offset(A::mul_scaled_output(s.ad_value(2263), s.ad_value(1889), 0.3333333333333333), (-1.0)), s.ad_value(1889)), 0.5));
            s.store_scaled_mul(1929, 1880, 1877, 0.16666666666666666);
        }

        s.b[2506] = (p.p49 == 1.0);
        s.v[2506] = if s.b[2506] { 1.0 } else { 0.0 };

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

        s.v[2285] = 0.0;

        s.v[2286] = 0.0;

        s.v[2284] = 0.0;

        s.b[2507] = ((s.v[266] > 0.0) || (s.v[267] > 0.0));
        s.v[2507] = if s.b[2507] { 1.0 } else { 0.0 };

        if s.b[2507] {
            s.store_scalar(2274, 1.0);
            s.copy_ad(2273, 1871);
        }

        s.b[2508] = (s.v[270] > 1e-10);
        s.v[2508] = if s.b[2508] { 1.0 } else { 0.0 };

        if (s.b[2507] && s.b[2508]) {
            s.store_add_scaled_inputs3_indices(2270, 1871, 1.0, 268, (-1.0), 797, 1.0);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1929, 2270, 0.5, 797, 0.5, A::add_scaled_product(s.ad_value(798), 1.0, A::sub(s.ad_value(2270), s.ad_value(797)), A::sub(s.ad_value(2270), s.ad_value(797)), 1.0), 0.5);
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
        s.v[2509] = if s.b[2509] { 1.0 } else { 0.0 };

        if (s.b[2507] && s.b[2509]) {
            s.store_add_scaled_product_right_ad(1929, 734, 0.5, 1872, A::scale_offset(s.ad_value(1873), 0.7071067811865475, 1.0), 1.0);
            s.store_div(2275, 1871, 1929);
        }

        s.b[2510] = (((s.v[2275]) as f64).abs() < 230.25850929940458);
        s.v[2510] = if s.b[2510] { 1.0 } else { 0.0 };

        if ((s.b[2507] && s.b[2509]) && s.b[2510]) {
            s.store_div_from_scalar_offset_ad(2276, 1.0, A::exp_scaled_input(s.ad_value(2275), -1.0), 1.0);
        }

        s.b[2511] = (s.v[2275] < 0.0);
        s.v[2511] = if s.b[2511] { 1.0 } else { 0.0 };

        if (((s.b[2507] && s.b[2509]) && (!s.b[2510])) && s.b[2511]) {
            s.store_div_from_scalar_offset_mul_offset_lhs_mixed_ia(2276, 1e-100, 2275, (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2275), (-230.25850929940458), A::scale_offset(s.ad_value(2275), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0);
        }

        s.b[2512] = (s.v[2275] < 230.25850929940458);
        s.v[2512] = if s.b[2512] { 1.0 } else { 0.0 };

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
            s.store_add_scaled_product_right_ad(2278, 2274, 1.0, 269, A::sub(s.ad_value(2276), s.ad_value(2274)), 1.0);
            s.store_add_scaled_product_right_ad(2279, 2273, 1.0, 269, A::sub(s.ad_value(2277), s.ad_value(2273)), 1.0);
            s.store_add_scaled_inputs3_mixed_aii(2280, A::add_scaled_product(s.ad_value(1871), 1.0, s.ad_value(1872), s.ad_value(1875), (-1.0)), 1.0, 1887, (-1.0), 1877, (-0.5));
            s.store_add_scaled_inputs3_indices(2281, 1871, 1.0, 2280, (-1.0), 1876, -1.0);
            s.store_add_scaled_inputs3_indices(2282, 1877, 1.0, 2280, 1.0, 815, -1.0);
            s.store_add_scaled_inputs3_indices(2283, 1871, 1.0, 2282, (-1.0), 1878, -1.0);
        }

        s.b[2513] = (s.v[820] > 0.0);
        s.v[2513] = if s.b[2513] { 1.0 } else { 0.0 };

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

        s.v[2289] = 0.0;

        s.v[2287] = 0.0;

        s.b[2514] = ((s.v[260] > 0.0) && (s.v[262] > 0.0));
        s.v[2514] = if s.b[2514] { 1.0 } else { 0.0 };

        if s.b[2514] {
            s.store_mul_add_scaled_inputs_rhs(1929, 264, s.ad_value(1803), 0.5, s.ad_value(776), 1.0);
        }

        s.b[2515] = (s.v[1929] < 230.25850929940458);
        s.v[2515] = if s.b[2515] { 1.0 } else { 0.0 };

        s.b[2516] = (s.v[1929] > (-230.25850929940458));
        s.v[2516] = if s.b[2516] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_38(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[2514] && s.b[2515]) && s.b[2516]) {
            s.store_exp(2287, 1929);
        }

        if ((s.b[2514] && s.b[2515]) && (!s.b[2516])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2287, 1e-100, (-230.25850929940458), 1929, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2517] = (s.v[2287] > 1e-10);
        s.v[2517] = if s.b[2517] { 1.0 } else { 0.0 };

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

        s.v[2292] = 0.0;

        s.v[2290] = 0.0;

        s.b[2518] = ((s.v[261] > 0.0) && (s.v[263] > 0.0));
        s.v[2518] = if s.b[2518] { 1.0 } else { 0.0 };

        if s.b[2518] {
            s.store_mul_add_scaled_inputs_rhs(1929, 264, s.ad_value(1803), 0.5, s.ad_value(777), 1.0);
        }

        s.b[2519] = (s.v[1929] < 230.25850929940458);
        s.v[2519] = if s.b[2519] { 1.0 } else { 0.0 };

        s.b[2520] = (s.v[1929] > (-230.25850929940458));
        s.v[2520] = if s.b[2520] { 1.0 } else { 0.0 };

        if ((s.b[2518] && s.b[2519]) && s.b[2520]) {
            s.store_exp(2290, 1929);
        }

        if ((s.b[2518] && s.b[2519]) && (!s.b[2520])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2290, 1e-100, (-230.25850929940458), 1929, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2521] = (s.v[2290] > 1e-10);
        s.v[2521] = if s.b[2521] { 1.0 } else { 0.0 };

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

        s.v[2522] = 0.0;

        s.v[2523] = 0.0;

        s.v[2524] = 0.0;

        s.v[2525] = 0.0;

        s.v[2526] = 0.0;

        s.v[2527] = 0.0;

        s.v[2528] = 0.0;

        s.v[2529] = 0.0;

        s.v[2530] = 0.0;

        s.v[2531] = 0.0;

        s.v[2532] = 0.0;

        s.v[2533] = 0.0;

        s.v[2534] = 0.0;

        s.v[2535] = 0.0;

        s.v[2536] = 0.0;

        s.v[2537] = 0.0;

        s.v[2538] = 0.0;

        s.v[2539] = 0.0;

        s.v[2540] = 0.0;

        s.v[2541] = 0.0;

        s.v[2542] = 0.0;

        s.v[2543] = 0.0;

        s.v[2544] = 0.0;

        s.v[2545] = 0.0;

        s.v[2546] = 0.0;

        s.v[2547] = 0.0;

        s.v[2548] = 0.0;

        s.v[2549] = 0.0;

        s.v[2550] = 0.0;

        s.v[2551] = 0.0;

        s.v[2552] = 0.0;

        s.v[2553] = 0.0;

        s.v[2554] = 0.0;

        s.v[2555] = 0.0;

        s.v[2556] = 0.0;

        s.v[2557] = 0.0;

        s.v[2558] = 0.0;

        s.v[2559] = 0.0;

        s.v[2560] = 0.0;

        s.v[2561] = 0.0;

        s.v[2562] = 0.0;

        s.v[2563] = 0.0;

        s.v[2564] = 0.0;

        s.v[2565] = 0.0;

        s.v[2566] = 0.0;

        s.v[2567] = 0.0;

        s.v[2568] = 0.0;

        s.v[837] = 0.0;

        s.v[1896] = 0.0;

        s.v[1897] = 0.0;

        s.v[1898] = 0.0;

        s.v[838] = 0.0;

        s.v[1899] = 0.0;

        s.v[1900] = 0.0;

        s.v[1901] = 0.0;

        s.v[846] = 0.0;

        s.v[1902] = 0.0;

        s.v[1903] = 0.0;

        s.v[1904] = 0.0;

        s.v[847] = 0.0;

        s.v[1905] = 0.0;

        s.v[1906] = 0.0;

        s.v[1907] = 0.0;

        s.b[2569] = (p.p43 > 0.0);
        s.v[2569] = if s.b[2569] { 1.0 } else { 0.0 };

        s.b[2570] = (s.v[475] == 1.0);
        s.v[2570] = if s.b[2570] { 1.0 } else { 0.0 };

        if (s.b[2569] && s.b[2570]) {
            s.store_scale(497, 821, (s.v[372] * s.v[669]));
        }

        if (s.b[2569] && s.b[2570]) {
            if (s.v[497] < (-230.25850929940458)) {
                s.store_div_from_scalar_offset_ad(498, 1e-100, A::sub_from_scalar((-230.25850929940458), s.ad_value(497)), 1.0);
            } else {
                if (s.v[497] > s.v[661]) {
                    s.store_mul_offset_ad_rhs(498, 662, A::sub(s.ad_value(497), s.ad_value(661)), 1.0);
                } else {
                    s.store_exp(498, 497);
                }
            }
        }

        if (s.b[2569] && s.b[2570]) {
            s.store_mul_offset_rhs(503, 668, 498, (-1.0));
            s.store_scaled_mul(497, 821, 671, s.v[372]);
        }

        if (s.b[2569] && s.b[2570]) {
            if (s.v[497] < (-230.25850929940458)) {
                s.store_div_from_scalar_offset_ad(498, 1e-100, A::sub_from_scalar((-230.25850929940458), s.ad_value(497)), 1.0);
            } else {
                if (s.v[497] > s.v[663]) {
                    s.store_mul_offset_ad_rhs(498, 664, A::sub(s.ad_value(497), s.ad_value(663)), 1.0);
                } else {
                    s.store_exp(498, 497);
                }
            }
        }

        if (s.b[2569] && s.b[2570]) {
            s.store_mul_offset_rhs(504, 670, 498, (-1.0));
            s.store_scalar(505, 0.0);
        }

        s.b[2571] = (s.v[667] > 0.0);
        s.v[2571] = if s.b[2571] { 1.0 } else { 0.0 };

        if ((s.b[2569] && s.b[2570]) && s.b[2571]) {
            s.store_mul_add_scaled_product_rhs(505, 821, s.ad_value(672), 1.0, s.ad_value(821), s.ad_value(673), 1.0);
        }

        if ((s.b[2569] && s.b[2570]) && (!s.b[2571])) {
            s.store_scaled_mul(497, 821, 673, (-s.v[372]));
        }

        if ((s.b[2569] && s.b[2570]) && (!s.b[2571])) {
            if (s.v[497] < (-230.25850929940458)) {
                s.store_div_from_scalar_offset_ad(498, 1e-100, A::sub_from_scalar((-230.25850929940458), s.ad_value(497)), 1.0);
            } else {
                if (s.v[497] > s.v[665]) {
                    s.store_mul_offset_ad_rhs(498, 666, A::sub(s.ad_value(497), s.ad_value(665)), 1.0);
                } else {
                    s.store_exp(498, 497);
                }
            }
        }

        if ((s.b[2569] && s.b[2570]) && (!s.b[2571])) {
            s.store_mul_scaled_offset_rhs(505, 672, -1.0, 498, (-1.0));
        }

        if (s.b[2569] && s.b[2570]) {
            s.store_add_scaled_inputs3_indices(837, 503, 1.0, 504, 1.0, 505, 1.0);
            s.store_scale(497, 822, (s.v[372] * s.v[696]));
        }

        if (s.b[2569] && s.b[2570]) {
            if (s.v[497] < (-230.25850929940458)) {
                s.store_div_from_scalar_offset_ad(498, 1e-100, A::sub_from_scalar((-230.25850929940458), s.ad_value(497)), 1.0);
            } else {
                if (s.v[497] > s.v[688]) {
                    s.store_mul_offset_ad_rhs(498, 689, A::sub(s.ad_value(497), s.ad_value(688)), 1.0);
                } else {
                    s.store_exp(498, 497);
                }
            }
        }

        if (s.b[2569] && s.b[2570]) {
            s.store_mul_offset_rhs(503, 695, 498, (-1.0));
            s.store_scaled_mul(497, 822, 698, s.v[372]);
        }

        if (s.b[2569] && s.b[2570]) {
            if (s.v[497] < (-230.25850929940458)) {
                s.store_div_from_scalar_offset_ad(498, 1e-100, A::sub_from_scalar((-230.25850929940458), s.ad_value(497)), 1.0);
            } else {
                if (s.v[497] > s.v[690]) {
                    s.store_mul_offset_ad_rhs(498, 691, A::sub(s.ad_value(497), s.ad_value(690)), 1.0);
                } else {
                    s.store_exp(498, 497);
                }
            }
        }

        if (s.b[2569] && s.b[2570]) {
            s.store_mul_offset_rhs(504, 697, 498, (-1.0));
            s.store_scalar(505, 0.0);
        }

        s.b[2572] = (s.v[694] > 0.0);
        s.v[2572] = if s.b[2572] { 1.0 } else { 0.0 };

        if ((s.b[2569] && s.b[2570]) && s.b[2572]) {
            s.store_mul_add_scaled_product_rhs(505, 822, s.ad_value(699), 1.0, s.ad_value(822), s.ad_value(700), 1.0);
        }

        if ((s.b[2569] && s.b[2570]) && (!s.b[2572])) {
            s.store_scaled_mul(497, 822, 700, (-s.v[372]));
        }

        if ((s.b[2569] && s.b[2570]) && (!s.b[2572])) {
            if (s.v[497] < (-230.25850929940458)) {
                s.store_div_from_scalar_offset_ad(498, 1e-100, A::sub_from_scalar((-230.25850929940458), s.ad_value(497)), 1.0);
            } else {
                if (s.v[497] > s.v[692]) {
                    s.store_mul_offset_ad_rhs(498, 693, A::sub(s.ad_value(497), s.ad_value(692)), 1.0);
                } else {
                    s.store_exp(498, 497);
                }
            }
        }

        if ((s.b[2569] && s.b[2570]) && (!s.b[2572])) {
            s.store_mul_scaled_offset_rhs(505, 699, -1.0, 498, (-1.0));
        }

        if (s.b[2569] && s.b[2570]) {
            s.store_add_scaled_inputs3_indices(838, 503, 1.0, 504, 1.0, 505, 1.0);
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
        s.v[2575] = if s.b[2575] { 1.0 } else { 0.0 };

        s.b[2576] = (s.v[409] == 0.5);
        s.v[2576] = if s.b[2576] { 1.0 } else { 0.0 };

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
        s.v[2577] = if s.b[2577] { 1.0 } else { 0.0 };

        s.b[2578] = (s.v[410] == 0.5);
        s.v[2578] = if s.b[2578] { 1.0 } else { 0.0 };

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
        s.v[2579] = if s.b[2579] { 1.0 } else { 0.0 };

        s.b[2580] = (s.v[411] == 0.5);
        s.v[2580] = if s.b[2580] { 1.0 } else { 0.0 };

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
            s.store_scalar(2574, 0.0);
            s.store_scaled_mul(2525, 685, 685, 4.0);
        }

    }

    pub(super) fn stamp_transient_block_39(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[2569] && s.b[2570]) {
            s.store_div(2526, 685, 686);
            s.store_add_scaled_product_indices(2527, 822, 1.0, 685, 2526, 1.0);
            s.store_add(2528, 686, 2527);
            s.store_sub(2529, 686, 2527);
            s.store_sqrt_square_add(2530, 2529, 2525);
            s.store_div_scaled_product_add_scaled_denominator_indices(2574, 822, 686, 2.0, 2528, 1.0, 2530, 1.0, 1.0);
        }

        s.b[2581] = (s.v[679] > 0.5);
        s.v[2581] = if s.b[2581] { 1.0 } else { 0.0 };

        s.b[2582] = (s.v[576] == 0.5);
        s.v[2582] = if s.b[2582] { 1.0 } else { 0.0 };

        if (((s.b[2569] && s.b[2570]) && s.b[2581]) && s.b[2582]) {
            s.store_sqrt_sub_from_scalar_ad(2573, 1.0, A::mul(s.ad_value(2574), s.ad_value(573)));
        }

        if (((s.b[2569] && s.b[2570]) && s.b[2581]) && (!s.b[2582])) {
            s.store_pow_ad(2573, A::sub_from_scalar(1.0, A::mul(s.ad_value(2574), s.ad_value(573))), s.ad_value(576));
        }

        if ((s.b[2569] && s.b[2570]) && s.b[2581]) {
            s.store_add_scaled_product_mixed_aia(1905, A::mul_sub_from_scalar_rhs(s.ad_value(585), 1.0, s.ad_value(2573)), 1.0, 588, A::sub(s.ad_value(822), s.ad_value(2574)), 1.0);
        }

        s.b[2583] = (s.v[680] > 0.5);
        s.v[2583] = if s.b[2583] { 1.0 } else { 0.0 };

        s.b[2584] = (s.v[577] == 0.5);
        s.v[2584] = if s.b[2584] { 1.0 } else { 0.0 };

        if (((s.b[2569] && s.b[2570]) && s.b[2583]) && s.b[2584]) {
            s.store_sqrt_sub_from_scalar_ad(2573, 1.0, A::mul(s.ad_value(2574), s.ad_value(574)));
        }

        if (((s.b[2569] && s.b[2570]) && s.b[2583]) && (!s.b[2584])) {
            s.store_pow_ad(2573, A::sub_from_scalar(1.0, A::mul(s.ad_value(2574), s.ad_value(574))), s.ad_value(577));
        }

        if ((s.b[2569] && s.b[2570]) && s.b[2583]) {
            s.store_add_scaled_product_mixed_aia(1906, A::mul_sub_from_scalar_rhs(s.ad_value(586), 1.0, s.ad_value(2573)), 1.0, 589, A::sub(s.ad_value(822), s.ad_value(2574)), 1.0);
        }

        s.b[2585] = (s.v[681] > 0.5);
        s.v[2585] = if s.b[2585] { 1.0 } else { 0.0 };

        s.b[2586] = (s.v[578] == 0.5);
        s.v[2586] = if s.b[2586] { 1.0 } else { 0.0 };

        if (((s.b[2569] && s.b[2570]) && s.b[2585]) && s.b[2586]) {
            s.store_sqrt_sub_from_scalar_ad(2573, 1.0, A::mul(s.ad_value(2574), s.ad_value(575)));
        }

        if (((s.b[2569] && s.b[2570]) && s.b[2585]) && (!s.b[2586])) {
            s.store_pow_ad(2573, A::sub_from_scalar(1.0, A::mul(s.ad_value(2574), s.ad_value(575))), s.ad_value(578));
        }

        if ((s.b[2569] && s.b[2570]) && s.b[2585]) {
            s.store_add_scaled_product_mixed_aia(1907, A::mul_sub_from_scalar_rhs(s.ad_value(587), 1.0, s.ad_value(2573)), 1.0, 590, A::sub(s.ad_value(822), s.ad_value(2574)), 1.0);
        }

        s.b[2587] = (p.p889 > 0.0);
        s.v[2587] = if s.b[2587] { 1.0 } else { 0.0 };

        if ((s.b[2569] && (!s.b[2570])) && s.b[2587]) {
            s.store_scaled_offset_ad(643, A::powf(A::add_scaled_inputs3(s.ad_value(814), 0.5, s.ad_value(816), 0.5, A::sqrt(A::offset(A::mul(A::add(s.ad_value(814), s.ad_value(816)), A::add(s.ad_value(814), s.ad_value(816))), (0.001 * 0.001))), 0.5), p.p890), (-(((0.5 * 0.001)) as f64).powf(p.p890)), p.p889);
            s.store_offset(641, 643, p.p879);
            s.store_div_from_scalar(451, 1.0, 641);
            s.store_div_from_scalar_offset_scaled_input(454, s.v[454], 643, 1.0 / (p.p879), 1.0);
        }

        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2587])) {
            s.store_scalar(641, p.p879);
        }

        s.b[2588] = (p.p891 > 0.0);
        s.v[2588] = if s.b[2588] { 1.0 } else { 0.0 };

        if ((s.b[2569] && (!s.b[2570])) && s.b[2588]) {
            s.store_scaled_offset_ad(645, A::powf(A::add_scaled_inputs3(s.ad_value(814), 0.5, s.ad_value(816), 0.5, A::sqrt(A::offset(A::mul(A::add(s.ad_value(814), s.ad_value(816)), A::add(s.ad_value(814), s.ad_value(816))), (0.001 * 0.001))), 0.5), p.p892), (-(((0.5 * 0.001)) as f64).powf(p.p892)), p.p891);
            s.store_mul_offset_rhs(444, 444, 645, 1.0);
        }

        if (s.b[2569] && (!s.b[2570])) {
            s.store_scalar(2538, 0.0);
            s.store_scalar(2535, 0.0);
        }

        s.b[2589] = (!(((s.v[647] == 0.0) && (s.v[648] == 0.0)) && (s.v[649] == 0.0)));
        s.v[2589] = if s.b[2589] { 1.0 } else { 0.0 };

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
        s.v[2590] = if s.b[2590] { 1.0 } else { 0.0 };

        s.b[2591] = (((((-0.5) * (s.v[821] * s.v[372]))) as f64).abs() < 230.25850929940458);
        s.v[2591] = if s.b[2591] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && s.b[2589]) && s.b[2590]) && s.b[2591]) {
            s.store_exp_scaled_input(2533, 821, (s.v[372] * (-0.5)));
        }

        s.b[2592] = (((-0.5) * (s.v[821] * s.v[372])) < 0.0);
        s.v[2592] = if s.b[2592] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && s.b[2589]) && s.b[2590]) && (!s.b[2591])) && s.b[2592]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(2533, 1e-100, (-230.25850929940458), A::scale(s.ad_value(821), (s.v[372] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(821), (s.v[372] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
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
        s.v[2593] = if s.b[2593] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && s.b[2589]) && s.b[2593]) {
            s.store_scaled_ln_ad(2535, A::add(A::offset(s.ad_value(2533), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2533), 1.0, A::offset(s.ad_value(2533), 3.0)))), (s.v[371] * 2.0));
        }

        if (((s.b[2569] && (!s.b[2570])) && s.b[2589]) && (!s.b[2593])) {
            s.store_sub_ad_lhs(2535, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(2534), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2534), 1.0, A::scale_offset(s.ad_value(2534), 3.0, 1.0))))), (s.v[371] * 2.0)), 821);
        }

        if ((s.b[2569] && (!s.b[2570])) && s.b[2589]) {
            s.store_sub(2536, 657, 2535);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2537, 821, 0.5, 2536, 0.5, A::offset(A::mul(A::sub(s.ad_value(821), s.ad_value(2536)), A::sub(s.ad_value(821), s.ad_value(2536))), ((4.0 * s.v[371]) * s.v[371])), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2538, 821, 0.5, 660, 0.5, A::offset(A::mul(A::sub(s.ad_value(821), s.ad_value(660)), A::sub(s.ad_value(821), s.ad_value(660))), ((4.0 * s.v[369]) * s.v[369])), (-0.5));
            s.store_scaled_sub_sqrt_square_offset_rhs(2539, 821, 821, ((4.0 * 1e-6) * 1e-6), 0.5);
        }

        s.b[2594] = (s.v[647] == 0.0);
        s.v[2594] = if s.b[2594] { 1.0 } else { 0.0 };

        if ((s.b[2569] && (!s.b[2570])) && s.b[2594]) {
            s.store_scalar(1896, 0.0);
            s.store_scalar(1902, 0.0);
        }

        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) {
            s.store_scale(2541, 2531, s.v[388]);
        }

        s.b[2595] = ((p.p857 == 0.0) && (p.p862 == 0.0));
        s.v[2595] = if s.b[2595] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && s.b[2595]) {
            s.store_scalar(2542, 0.0);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2595])) {
            s.store_sub_from_scalar(2543, s.v[394], 2537);
            s.store_sub_from_scalar_ad(2544, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2535), s.ad_value(2543)))));
        }

        s.b[2596] = (p.p848 == 0.5);
        s.v[2596] = if s.b[2596] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2595])) && s.b[2596]) {
            s.store_scalar(2545, 0.0);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2595])) && (!s.b[2596])) {
            s.store_scaled_add_ad_lhs(2545, A::div_scaled_product(A::square(s.ad_value(2544)), A::ln(s.ad_value(2544)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2544)), 1.0), 2544, (1.0 - (2.0 * p.p848)));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2595])) {
            s.store_add(2546, 2544, 2545);
        }

        s.b[2597] = (p.p848 == 0.5);
        s.v[2597] = if s.b[2597] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2595])) && s.b[2597]) {
            s.store_sqrt_scaled_input(2540, 2543, s.v[430]);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2595])) && (!s.b[2597])) {
            s.store_powf_ad(2540, A::scale(s.ad_value(2543), s.v[430]), p.p848);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2595])) {
            s.store_scale(2547, 2540, s.v[424]);
            s.store_mul_offset_lhs_scaled_output(2548, 2534, (-1.0), 2547, s.v[385]);
            s.store_scaled_mul(2542, 2548, 2546, p.p857);
        }

        s.b[2598] = (p.p862 == 0.0);
        s.v[2598] = if s.b[2598] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && s.b[2598]) {
            s.store_scalar(2549, 0.0);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) {
            s.store_div_scaled_inputs_indices(2550, 2547, (s.v[409] * s.v[439]), 2543, 1.0);
            s.store_div_from_scalar(2551, (0.666666666666667 * s.v[436]), 2550);
            s.store_square(2552, 2551);
            s.store_sqrt_ad(2553, A::div_scaled_product_offset_denominator(s.ad_value(2552), s.ad_value(2552), 1.0, A::square(s.ad_value(2552)), 1.0, 1.0));
            s.store_sqrt(2554, 2553);
            s.store_mul(2555, 2553, 2554);
        }

        s.b[2599] = (((-p.p848) * s.v[412]) == (-1.0));
        s.v[2599] = if s.b[2599] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) && s.b[2599]) {
            s.store_div_from_scalar_offset_ad(2556, 1.0, A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) && (!s.b[2599])) {
            s.store_powf_ad(2556, A::offset(A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0), ((-p.p848) * s.v[412]));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(2557, 2546, 2556, 1.0, 2546, 1.0, 2556, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(2558, A::div(s.ad_value(2550), s.ad_value(2554)), 0.375);
            s.store_add_scaled_product_indices(2559, 2553, (-1.0), 2551, 2554, 2.0);
            s.store_add_scaled_value_products(2560, s.ad_value(2553), (-s.v[436]), s.ad_value(2551), s.ad_value(2554), s.v[436], s.ad_value(2550), s.ad_value(2555), 0.5);
            s.store_mul_offset_lhs(2561, 2559, (-1.0), 2558);
            s.store_square(2522, 2561);
        }

        s.b[2600] = (s.v[2561] > 0.0);
        s.v[2600] = if s.b[2600] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) && s.b[2600]) {
            s.store_div_from_scalar_offset_scaled_input(2523, 1.0, 2561, s.v[373], 1.0);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) && (!s.b[2600])) {
            s.store_div_from_scalar_sub_from_scalar_ad(2523, 1.0, 1.0, A::scale(s.ad_value(2561), s.v[373]));
        }

        s.b[2601] = (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458));
        s.v[2601] = if s.b[2601] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) && s.b[2601]) {
            s.store_exp_sub(2540, 2560, 2522);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) && (!s.b[2601])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(2540, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) {
            s.store_mul_ad_lhs(2524, A::add_scaled_inputs_product(s.ad_value(2523), 0.29214664, A::square(s.ad_value(2523)), s.v[374], A::square(s.ad_value(2523)), s.ad_value(2523), s.v[375]), 2540);
        }

        s.b[2602] = (s.v[2561] > 0.0);
        s.v[2602] = if s.b[2602] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) && s.b[2602]) {
            s.copy_ad(2562, 2524);
        }

        s.b[2603] = (s.v[2560] > (-230.25850929940458));
        s.v[2603] = if s.b[2603] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) && (!s.b[2602])) && s.b[2603]) {
            s.store_exp(2540, 2560);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) && (!s.b[2602])) && (!s.b[2603])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 2560, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) && (!s.b[2602])) {
            s.store_sub_scaled_inputs(2562, 2540, 2.0, 2524, 1.0);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) {
            s.store_div_scaled_inputs_indices(2563, 2562, (s.v[436] * (1.772453850905516 * 0.5)), 2558, 1.0);
            s.store_mul3_affine_lhs(2549, 2548, 2563, p.p862, 0.0, 2557);
        }

        s.b[2604] = (p.p868 == 0.0);
        s.v[2604] = if s.b[2604] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && s.b[2604]) {
            s.store_scalar(2564, 0.0);
        }

        s.b[2605] = (p.p848 == 0.5);
        s.v[2605] = if s.b[2605] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2604])) && s.b[2605]) {
            s.store_sqrt_scaled_input_ad(2540, A::sub_from_scalar(p.p845, s.ad_value(2538)), s.v[430]);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2604])) && (!s.b[2605])) {
            s.store_powf_ad(2540, A::scale_offset(s.ad_value(2538), (-s.v[430]), ((p.p845) * (s.v[430]))), p.p848);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2604])) {
            s.store_div_scaled_offset_numerator(2565, s.ad_value(2538), ((-s.v[427]) * s.v[412]), (((p.p845) * (s.v[427])) * s.v[412]), s.ad_value(2540), 1.0);
        }

        s.b[2606] = (((((-s.v[442]) / s.v[2565])) as f64).abs() < 230.25850929940458);
        s.v[2606] = if s.b[2606] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2604])) && s.b[2606]) {
            s.store_exp_ad(2540, A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(2565), 1.0));
        }

        s.b[2607] = (((-s.v[442]) / s.v[2565]) < 0.0);
        s.v[2607] = if s.b[2607] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2604])) && (!s.b[2606])) && s.b[2607]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(2540, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(2565), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(2565), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2604])) && (!s.b[2606])) && (!s.b[2607])) {
            let assign57180_ad_e72546: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(2565), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(2565), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(2565), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2540, assign57180_ad_e72546, 1e100);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2604])) {
            s.store_mul_scaled_ad_lhs(2564, A::mul3(s.ad_value(821), s.ad_value(2565), s.ad_value(2565)), 2540, p.p868);
        }

        s.b[2608] = (p.p877 > 1000.0);
        s.v[2608] = if s.b[2608] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && s.b[2608]) {
            s.store_scalar(2566, 1.0);
        }

        s.b[2609] = (s.v[2539] > ((-s.v[445]) * p.p877));
        s.v[2609] = if s.b[2609] { 1.0 } else { 0.0 };

        s.b[2610] = (p.p880 == 4.0);
        s.v[2610] = if s.b[2610] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2608])) && s.b[2609]) && s.b[2610]) {
            s.store_mul_scaled_ad_lhs(2540, A::mul3_scaled_output(s.ad_value(2539), s.ad_value(2539), s.ad_value(2539), ((s.v[449] * s.v[449]) * s.v[449])), 2539, s.v[449]);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2608])) && s.b[2609]) && (!s.b[2610])) {
            s.store_powf_ad(2540, A::abs_scaled_input(s.ad_value(2539), s.v[449]), p.p880);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2608])) && s.b[2609]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2566, 1.0, 1.0, s.ad_value(2540));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2608])) && (!s.b[2609])) {
            s.store_offset_scaled(2566, 2539, s.v[452], (((((s.v[445] * p.p877)) * (s.v[452]))) + (s.v[446])));
        }

        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) {
            s.store_mul_scale_ad_lhs(1896, A::add_scaled_inputs4(s.ad_value(2541), 1.0, s.ad_value(2542), 1.0, s.ad_value(2549), 1.0, s.ad_value(2564), 1.0), p.p29, 2566);
        }

        s.b[2611] = (s.v[409] == 0.5);
        s.v[2611] = if s.b[2611] { 1.0 } else { 0.0 };

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
        s.v[2612] = if s.b[2612] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_40(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[2569] && (!s.b[2570])) && s.b[2612]) {
            s.store_scalar(1897, 0.0);
            s.store_scalar(1903, 0.0);
        }

        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) {
            s.store_scale(2541, 2531, s.v[389]);
        }

        s.b[2613] = ((p.p858 == 0.0) && (p.p863 == 0.0));
        s.v[2613] = if s.b[2613] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && s.b[2613]) {
            s.store_scalar(2542, 0.0);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2613])) {
            s.store_sub_from_scalar(2543, s.v[395], 2537);
            s.store_sub_from_scalar_ad(2544, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2535), s.ad_value(2543)))));
        }

        s.b[2614] = (p.p849 == 0.5);
        s.v[2614] = if s.b[2614] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2613])) && s.b[2614]) {
            s.store_scalar(2545, 0.0);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2613])) && (!s.b[2614])) {
            s.store_scaled_add_ad_lhs(2545, A::div_scaled_product(A::square(s.ad_value(2544)), A::ln(s.ad_value(2544)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2544)), 1.0), 2544, (1.0 - (2.0 * p.p849)));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2613])) {
            s.store_add(2546, 2544, 2545);
        }

        s.b[2615] = (p.p849 == 0.5);
        s.v[2615] = if s.b[2615] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2613])) && s.b[2615]) {
            s.store_sqrt_scaled_input(2540, 2543, s.v[431]);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2613])) && (!s.b[2615])) {
            s.store_powf_ad(2540, A::scale(s.ad_value(2543), s.v[431]), p.p849);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2613])) {
            s.store_scale(2547, 2540, s.v[425]);
            s.store_mul_offset_lhs_scaled_output(2548, 2534, (-1.0), 2547, s.v[386]);
            s.store_scaled_mul(2542, 2548, 2546, p.p858);
        }

        s.b[2616] = (p.p863 == 0.0);
        s.v[2616] = if s.b[2616] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && s.b[2616]) {
            s.store_scalar(2549, 0.0);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) {
            s.store_div_scaled_inputs_indices(2550, 2547, (s.v[410] * s.v[440]), 2543, 1.0);
            s.store_div_from_scalar(2551, (0.666666666666667 * s.v[437]), 2550);
            s.store_square(2552, 2551);
            s.store_sqrt_ad(2553, A::div_scaled_product_offset_denominator(s.ad_value(2552), s.ad_value(2552), 1.0, A::square(s.ad_value(2552)), 1.0, 1.0));
            s.store_sqrt(2554, 2553);
            s.store_mul(2555, 2553, 2554);
        }

        s.b[2617] = (((-p.p849) * s.v[413]) == (-1.0));
        s.v[2617] = if s.b[2617] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) && s.b[2617]) {
            s.store_div_from_scalar_offset_ad(2556, 1.0, A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) && (!s.b[2617])) {
            s.store_powf_ad(2556, A::offset(A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0), ((-p.p849) * s.v[413]));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(2557, 2546, 2556, 1.0, 2546, 1.0, 2556, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(2558, A::div(s.ad_value(2550), s.ad_value(2554)), 0.375);
            s.store_add_scaled_product_indices(2559, 2553, (-1.0), 2551, 2554, 2.0);
            s.store_add_scaled_value_products(2560, s.ad_value(2553), (-s.v[437]), s.ad_value(2551), s.ad_value(2554), s.v[437], s.ad_value(2550), s.ad_value(2555), 0.5);
            s.store_mul_offset_lhs(2561, 2559, (-1.0), 2558);
            s.store_square(2522, 2561);
        }

        s.b[2618] = (s.v[2561] > 0.0);
        s.v[2618] = if s.b[2618] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) && s.b[2618]) {
            s.store_div_from_scalar_offset_scaled_input(2523, 1.0, 2561, s.v[373], 1.0);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) && (!s.b[2618])) {
            s.store_div_from_scalar_sub_from_scalar_ad(2523, 1.0, 1.0, A::scale(s.ad_value(2561), s.v[373]));
        }

        s.b[2619] = (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458));
        s.v[2619] = if s.b[2619] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) && s.b[2619]) {
            s.store_exp_sub(2540, 2560, 2522);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) && (!s.b[2619])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(2540, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) {
            s.store_mul_ad_lhs(2524, A::add_scaled_inputs_product(s.ad_value(2523), 0.29214664, A::square(s.ad_value(2523)), s.v[374], A::square(s.ad_value(2523)), s.ad_value(2523), s.v[375]), 2540);
        }

        s.b[2620] = (s.v[2561] > 0.0);
        s.v[2620] = if s.b[2620] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) && s.b[2620]) {
            s.copy_ad(2562, 2524);
        }

        s.b[2621] = (s.v[2560] > (-230.25850929940458));
        s.v[2621] = if s.b[2621] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) && (!s.b[2620])) && s.b[2621]) {
            s.store_exp(2540, 2560);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) && (!s.b[2620])) && (!s.b[2621])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 2560, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) && (!s.b[2620])) {
            s.store_sub_scaled_inputs(2562, 2540, 2.0, 2524, 1.0);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) {
            s.store_div_scaled_inputs_indices(2563, 2562, (s.v[437] * (1.772453850905516 * 0.5)), 2558, 1.0);
            s.store_mul3_affine_lhs(2549, 2548, 2563, p.p863, 0.0, 2557);
        }

        s.b[2622] = (p.p869 == 0.0);
        s.v[2622] = if s.b[2622] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && s.b[2622]) {
            s.store_scalar(2564, 0.0);
        }

        s.b[2623] = (p.p849 == 0.5);
        s.v[2623] = if s.b[2623] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2622])) && s.b[2623]) {
            s.store_sqrt_scaled_input_ad(2540, A::sub_from_scalar(p.p846, s.ad_value(2538)), s.v[431]);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2622])) && (!s.b[2623])) {
            s.store_powf_ad(2540, A::scale_offset(s.ad_value(2538), (-s.v[431]), ((p.p846) * (s.v[431]))), p.p849);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2622])) {
            s.store_div_scaled_offset_numerator(2565, s.ad_value(2538), ((-s.v[428]) * s.v[413]), (((p.p846) * (s.v[428])) * s.v[413]), s.ad_value(2540), 1.0);
        }

        s.b[2624] = (((((-s.v[443]) / s.v[2565])) as f64).abs() < 230.25850929940458);
        s.v[2624] = if s.b[2624] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2622])) && s.b[2624]) {
            s.store_exp_ad(2540, A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(2565), 1.0));
        }

        s.b[2625] = (((-s.v[443]) / s.v[2565]) < 0.0);
        s.v[2625] = if s.b[2625] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2622])) && (!s.b[2624])) && s.b[2625]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(2540, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(2565), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(2565), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2622])) && (!s.b[2624])) && (!s.b[2625])) {
            let assign57930_ad_e73812: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(2565), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(2565), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(2565), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2540, assign57930_ad_e73812, 1e100);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2622])) {
            s.store_mul_scaled_ad_lhs(2564, A::mul3(s.ad_value(821), s.ad_value(2565), s.ad_value(2565)), 2540, p.p869);
        }

        s.b[2626] = (p.p878 > 1000.0);
        s.v[2626] = if s.b[2626] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && s.b[2626]) {
            s.store_scalar(2566, 1.0);
        }

        s.b[2627] = (s.v[2539] > ((-s.v[445]) * p.p878));
        s.v[2627] = if s.b[2627] { 1.0 } else { 0.0 };

        s.b[2628] = (p.p881 == 4.0);
        s.v[2628] = if s.b[2628] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2626])) && s.b[2627]) && s.b[2628]) {
            s.store_mul_scaled_ad_lhs(2540, A::mul3_scaled_output(s.ad_value(2539), s.ad_value(2539), s.ad_value(2539), ((s.v[450] * s.v[450]) * s.v[450])), 2539, s.v[450]);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2626])) && s.b[2627]) && (!s.b[2628])) {
            s.store_powf_ad(2540, A::abs_scaled_input(s.ad_value(2539), s.v[450]), p.p881);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2626])) && s.b[2627]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2566, 1.0, 1.0, s.ad_value(2540));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2626])) && (!s.b[2627])) {
            s.store_offset_scaled(2566, 2539, s.v[453], (((((s.v[445] * p.p878)) * (s.v[453]))) + (s.v[447])));
        }

        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) {
            s.store_mul_scale_ad_lhs(1897, A::add_scaled_inputs4(s.ad_value(2541), 1.0, s.ad_value(2542), 1.0, s.ad_value(2549), 1.0, s.ad_value(2564), 1.0), p.p29, 2566);
        }

        s.b[2629] = (s.v[410] == 0.5);
        s.v[2629] = if s.b[2629] { 1.0 } else { 0.0 };

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
        s.v[2630] = if s.b[2630] { 1.0 } else { 0.0 };

        if ((s.b[2569] && (!s.b[2570])) && s.b[2630]) {
            s.store_scalar(1898, 0.0);
            s.store_scalar(1904, 0.0);
        }

        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) {
            s.store_scale(2541, 2531, s.v[390]);
        }

        s.b[2631] = ((p.p859 == 0.0) && (p.p864 == 0.0));
        s.v[2631] = if s.b[2631] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2631]) {
            s.store_scalar(2542, 0.0);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2631])) {
            s.store_sub_from_scalar(2543, s.v[396], 2537);
            s.store_sub_from_scalar_ad(2544, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2535), s.ad_value(2543)))));
        }

        s.b[2632] = (p.p850 == 0.5);
        s.v[2632] = if s.b[2632] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2631])) && s.b[2632]) {
            s.store_scalar(2545, 0.0);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2631])) && (!s.b[2632])) {
            s.store_scaled_add_ad_lhs(2545, A::div_scaled_product(A::square(s.ad_value(2544)), A::ln(s.ad_value(2544)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2544)), 1.0), 2544, (1.0 - (2.0 * p.p850)));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2631])) {
            s.store_add(2546, 2544, 2545);
        }

        s.b[2633] = (p.p850 == 0.5);
        s.v[2633] = if s.b[2633] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2631])) && s.b[2633]) {
            s.store_sqrt_scaled_input(2540, 2543, s.v[432]);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2631])) && (!s.b[2633])) {
            s.store_powf_ad(2540, A::scale(s.ad_value(2543), s.v[432]), p.p850);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2631])) {
            s.store_scale(2547, 2540, s.v[426]);
            s.store_mul_offset_lhs_scaled_output(2548, 2534, (-1.0), 2547, s.v[387]);
            s.store_scaled_mul(2542, 2548, 2546, p.p859);
        }

        s.b[2634] = (p.p864 == 0.0);
        s.v[2634] = if s.b[2634] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2634]) {
            s.store_scalar(2549, 0.0);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) {
            s.store_div_scaled_inputs_indices(2550, 2547, (s.v[411] * s.v[441]), 2543, 1.0);
            s.store_div_from_scalar(2551, (0.666666666666667 * s.v[438]), 2550);
            s.store_square(2552, 2551);
            s.store_sqrt_ad(2553, A::div_scaled_product_offset_denominator(s.ad_value(2552), s.ad_value(2552), 1.0, A::square(s.ad_value(2552)), 1.0, 1.0));
            s.store_sqrt(2554, 2553);
            s.store_mul(2555, 2553, 2554);
        }

        s.b[2635] = (((-p.p850) * s.v[414]) == (-1.0));
        s.v[2635] = if s.b[2635] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) && s.b[2635]) {
            s.store_div_from_scalar_offset_ad(2556, 1.0, A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) && (!s.b[2635])) {
            s.store_powf_ad(2556, A::offset(A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0), ((-p.p850) * s.v[414]));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(2557, 2546, 2556, 1.0, 2546, 1.0, 2556, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(2558, A::div(s.ad_value(2550), s.ad_value(2554)), 0.375);
            s.store_add_scaled_product_indices(2559, 2553, (-1.0), 2551, 2554, 2.0);
            s.store_add_scaled_value_products(2560, s.ad_value(2553), (-s.v[438]), s.ad_value(2551), s.ad_value(2554), s.v[438], s.ad_value(2550), s.ad_value(2555), 0.5);
            s.store_mul_offset_lhs(2561, 2559, (-1.0), 2558);
            s.store_square(2522, 2561);
        }

        s.b[2636] = (s.v[2561] > 0.0);
        s.v[2636] = if s.b[2636] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) && s.b[2636]) {
            s.store_div_from_scalar_offset_scaled_input(2523, 1.0, 2561, s.v[373], 1.0);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) && (!s.b[2636])) {
            s.store_div_from_scalar_sub_from_scalar_ad(2523, 1.0, 1.0, A::scale(s.ad_value(2561), s.v[373]));
        }

        s.b[2637] = (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458));
        s.v[2637] = if s.b[2637] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) && s.b[2637]) {
            s.store_exp_sub(2540, 2560, 2522);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) && (!s.b[2637])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(2540, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) {
            s.store_mul_ad_lhs(2524, A::add_scaled_inputs_product(s.ad_value(2523), 0.29214664, A::square(s.ad_value(2523)), s.v[374], A::square(s.ad_value(2523)), s.ad_value(2523), s.v[375]), 2540);
        }

        s.b[2638] = (s.v[2561] > 0.0);
        s.v[2638] = if s.b[2638] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) && s.b[2638]) {
            s.copy_ad(2562, 2524);
        }

        s.b[2639] = (s.v[2560] > (-230.25850929940458));
        s.v[2639] = if s.b[2639] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) && (!s.b[2638])) && s.b[2639]) {
            s.store_exp(2540, 2560);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) && (!s.b[2638])) && (!s.b[2639])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 2560, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) && (!s.b[2638])) {
            s.store_sub_scaled_inputs(2562, 2540, 2.0, 2524, 1.0);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) {
            s.store_div_scaled_inputs_indices(2563, 2562, (s.v[438] * (1.772453850905516 * 0.5)), 2558, 1.0);
            s.store_mul3_affine_lhs(2549, 2548, 2563, p.p864, 0.0, 2557);
        }

        s.b[2640] = (p.p870 == 0.0);
        s.v[2640] = if s.b[2640] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2640]) {
            s.store_scalar(2564, 0.0);
        }

        s.b[2641] = (p.p850 == 0.5);
        s.v[2641] = if s.b[2641] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2640])) && s.b[2641]) {
            s.store_sqrt_scaled_input_ad(2540, A::sub_from_scalar(p.p847, s.ad_value(2538)), s.v[432]);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2640])) && (!s.b[2641])) {
            s.store_powf_ad(2540, A::scale_offset(s.ad_value(2538), (-s.v[432]), ((p.p847) * (s.v[432]))), p.p850);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2640])) {
            s.store_div_scaled_offset_numerator(2565, s.ad_value(2538), ((-s.v[429]) * s.v[414]), (((p.p847) * (s.v[429])) * s.v[414]), s.ad_value(2540), 1.0);
        }

        s.b[2642] = (((((-s.v[444]) / s.v[2565])) as f64).abs() < 230.25850929940458);
        s.v[2642] = if s.b[2642] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2640])) && s.b[2642]) {
            s.store_exp_ad(2540, A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(2565), 1.0));
        }

        s.b[2643] = (((-s.v[444]) / s.v[2565]) < 0.0);
        s.v[2643] = if s.b[2643] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2640])) && (!s.b[2642])) && s.b[2643]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(2540, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(2565), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(2565), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2640])) && (!s.b[2642])) && (!s.b[2643])) {
            let assign58680_ad_e75078: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(2565), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(2565), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(2565), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2540, assign58680_ad_e75078, 1e100);
        }

    }

    pub(super) fn stamp_transient_block_41(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2640])) {
            s.store_mul_scaled_ad_lhs(2564, A::mul3(s.ad_value(821), s.ad_value(2565), s.ad_value(2565)), 2540, p.p870);
        }

        s.b[2644] = (s.v[641] > 1000.0);
        s.v[2644] = if s.b[2644] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2644]) {
            s.store_scalar(2566, 1.0);
        }

        s.b[2645] = (s.v[2539] > ((-s.v[445]) * s.v[641]));
        s.v[2645] = if s.b[2645] { 1.0 } else { 0.0 };

        s.b[2646] = (p.p882 == 4.0);
        s.v[2646] = if s.b[2646] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2644])) && s.b[2645]) && s.b[2646]) {
            s.store_mul_ad_product_lhs(2540, A::mul3(A::mul3(s.ad_value(2539), s.ad_value(451), A::mul(s.ad_value(2539), s.ad_value(451))), s.ad_value(2539), s.ad_value(451)), s.ad_value(2539), 451);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2644])) && s.b[2645]) && (!s.b[2646])) {
            s.store_powf_ad(2540, A::abs(A::mul(s.ad_value(2539), s.ad_value(451))), p.p882);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2644])) && s.b[2645]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2566, 1.0, 1.0, s.ad_value(2540));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2644])) && (!s.b[2645])) {
            s.store_offset_mul_ad(2566, A::add_scaled_inputs(s.ad_value(2539), 1.0, s.ad_value(641), s.v[445]), s.ad_value(454), s.v[448]);
        }

        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) {
            s.store_mul_scale_ad_lhs(1898, A::add_scaled_inputs4(s.ad_value(2541), 1.0, s.ad_value(2542), 1.0, s.ad_value(2549), 1.0, s.ad_value(2564), 1.0), p.p29, 2566);
        }

        s.b[2647] = (s.v[474] == 1.0);
        s.v[2647] = if s.b[2647] { 1.0 } else { 0.0 };

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
        s.v[2648] = if s.b[2648] { 1.0 } else { 0.0 };

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
        s.v[2649] = if s.b[2649] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2647]) && s.b[2649]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2568), s.ad_value(467)));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2647]) && (!s.b[2649])) {
            s.store_pow_ad(2540, A::sub_from_scalar(1.0, A::mul(s.ad_value(2568), s.ad_value(467))), s.ad_value(468));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2647]) {
            s.store_add_scaled_product_mixed_aia(473, A::mul_sub_from_scalar_rhs(s.ad_value(471), 1.0, s.ad_value(2540)), p.p30, 472, A::sub(s.ad_value(2567), s.ad_value(2568)), p.p30);
            s.store_add(1904, 1904, 473);
        }

        s.b[2650] = (s.v[411] == 0.5);
        s.v[2650] = if s.b[2650] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2647])) && s.b[2650]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::scale(s.ad_value(2532), s.v[408]));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2647])) && (!s.b[2650])) {
            s.store_powf_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2532), s.v[408])), s.v[411]);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2647])) {
            s.store_add_scaled_inputs3_offset_indices(1904, 2540, ((-s.v[420]) * p.p30), 821, (s.v[423] * p.p30), 2532, ((-s.v[423]) * p.p30), (s.v[420] * p.p30));
        }

        if (s.b[2569] && (!s.b[2570])) {
            s.store_add_scaled_products3(837, s.ad_value(647), s.ad_value(1896), 1.0, s.ad_value(648), s.ad_value(1897), 1.0, s.ad_value(649), s.ad_value(1898), 1.0);
        }

        s.b[2651] = (s.v[637] > 0.0);
        s.v[2651] = if s.b[2651] { 1.0 } else { 0.0 };

        if ((s.b[2569] && (!s.b[2570])) && s.b[2651]) {
            s.store_mul_sub_ad_rhs(644, 637, A::pow(A::add_scaled_inputs3(s.ad_value(814), 0.5, s.ad_value(816), 0.5, A::sqrt(A::offset(A::mul(A::add(s.ad_value(814), s.ad_value(816)), A::add(s.ad_value(814), s.ad_value(816))), (0.001 * 0.001))), 0.5), s.ad_value(638)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(638)));
            s.store_add(642, 543, 644);
            s.store_div_from_scalar(617, 1.0, 642);
            s.store_div_scaled_value_offset_denominator(620, s.ad_value(620), 1.0, A::div(s.ad_value(644), s.ad_value(543)), 1.0, 1.0);
        }

        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2651])) {
            s.copy_ad(642, 543);
        }

        s.b[2652] = (s.v[639] > 0.0);
        s.v[2652] = if s.b[2652] { 1.0 } else { 0.0 };

        if ((s.b[2569] && (!s.b[2570])) && s.b[2652]) {
            s.store_mul_sub_ad_rhs(646, 639, A::pow(A::add_scaled_inputs3(s.ad_value(814), 0.5, s.ad_value(816), 0.5, A::sqrt(A::offset(A::mul(A::add(s.ad_value(814), s.ad_value(816)), A::add(s.ad_value(814), s.ad_value(816))), (0.001 * 0.001))), 0.5), s.ad_value(640)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(640)));
            s.store_mul_offset_rhs(611, 611, 646, 1.0);
        }

        if (s.b[2569] && (!s.b[2570])) {
            s.store_scalar(2538, 0.0);
            s.store_scalar(2535, 0.0);
        }

        s.b[2653] = (!(((s.v[674] == 0.0) && (s.v[675] == 0.0)) && (s.v[676] == 0.0)));
        s.v[2653] = if s.b[2653] { 1.0 } else { 0.0 };

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
        s.v[2654] = if s.b[2654] { 1.0 } else { 0.0 };

        s.b[2655] = (((((-0.5) * (s.v[822] * s.v[372]))) as f64).abs() < 230.25850929940458);
        s.v[2655] = if s.b[2655] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && s.b[2653]) && s.b[2654]) && s.b[2655]) {
            s.store_exp_scaled_input(2533, 822, (s.v[372] * (-0.5)));
        }

        s.b[2656] = (((-0.5) * (s.v[822] * s.v[372])) < 0.0);
        s.v[2656] = if s.b[2656] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && s.b[2653]) && s.b[2654]) && (!s.b[2655])) && s.b[2656]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(2533, 1e-100, (-230.25850929940458), A::scale(s.ad_value(822), (s.v[372] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(822), (s.v[372] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
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
        s.v[2657] = if s.b[2657] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && s.b[2653]) && s.b[2657]) {
            s.store_scaled_ln_ad(2535, A::add(A::offset(s.ad_value(2533), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2533), 1.0, A::offset(s.ad_value(2533), 3.0)))), (s.v[371] * 2.0));
        }

        if (((s.b[2569] && (!s.b[2570])) && s.b[2653]) && (!s.b[2657])) {
            s.store_sub_ad_lhs(2535, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(2534), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2534), 1.0, A::scale_offset(s.ad_value(2534), 3.0, 1.0))))), (s.v[371] * 2.0)), 822);
        }

        if ((s.b[2569] && (!s.b[2570])) && s.b[2653]) {
            s.store_sub(2536, 684, 2535);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2537, 822, 0.5, 2536, 0.5, A::offset(A::mul(A::sub(s.ad_value(822), s.ad_value(2536)), A::sub(s.ad_value(822), s.ad_value(2536))), ((4.0 * s.v[371]) * s.v[371])), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2538, 822, 0.5, 687, 0.5, A::offset(A::mul(A::sub(s.ad_value(822), s.ad_value(687)), A::sub(s.ad_value(822), s.ad_value(687))), ((4.0 * s.v[369]) * s.v[369])), (-0.5));
            s.store_scaled_sub_sqrt_square_offset_rhs(2539, 822, 822, ((4.0 * 1e-6) * 1e-6), 0.5);
        }

        s.b[2658] = (s.v[674] == 0.0);
        s.v[2658] = if s.b[2658] { 1.0 } else { 0.0 };

        if ((s.b[2569] && (!s.b[2570])) && s.b[2658]) {
            s.store_scalar(1899, 0.0);
            s.store_scalar(1905, 0.0);
        }

        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) {
            s.store_mul(2541, 564, 2531);
        }

        s.b[2659] = ((s.v[523] == 0.0) && (s.v[526] == 0.0));
        s.v[2659] = if s.b[2659] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && s.b[2659]) {
            s.store_scalar(2542, 0.0);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2659])) {
            s.store_sub(2543, 570, 2537);
            s.store_sub_from_scalar_ad(2544, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2535), s.ad_value(2543)))));
        }

        s.b[2660] = (s.v[512] == 0.5);
        s.v[2660] = if s.b[2660] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2659])) && s.b[2660]) {
            s.store_scalar(2545, 0.0);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2659])) && (!s.b[2660])) {
            s.store_mul_sub_from_scalar_rhs_ad(2545, A::add(A::div_scaled_product(A::square(s.ad_value(2544)), A::ln(s.ad_value(2544)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2544)), 1.0), s.ad_value(2544)), 1.0, A::scale(s.ad_value(512), 2.0));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2659])) {
            s.store_add(2546, 2544, 2545);
        }

        s.b[2661] = (s.v[512] == 0.5);
        s.v[2661] = if s.b[2661] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2659])) && s.b[2661]) {
            s.store_sqrt_mul(2540, 2543, 597);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2659])) && (!s.b[2661])) {
            s.store_pow_ad(2540, A::mul(s.ad_value(2543), s.ad_value(597)), s.ad_value(512));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2659])) {
            s.store_mul(2547, 591, 2540);
            s.store_mul_ad_product_lhs(2548, s.ad_value(561), A::offset(s.ad_value(2534), (-1.0)), 2547);
            s.store_mul3_lhs(2542, 523, 2548, 2546);
        }

        s.b[2662] = (s.v[526] == 0.0);
        s.v[2662] = if s.b[2662] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && s.b[2662]) {
            s.store_scalar(2549, 0.0);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) {
            s.store_mul_div_scaled_product_rhs(2550, 606, s.ad_value(2547), s.ad_value(576), 1.0, s.ad_value(2543), 1.0);
            s.store_div_scaled_inputs_indices(2551, 603, 0.666666666666667, 2550, 1.0);
            s.store_square(2552, 2551);
            s.store_sqrt_ad(2553, A::div_scaled_product_offset_denominator(s.ad_value(2552), s.ad_value(2552), 1.0, A::square(s.ad_value(2552)), 1.0, 1.0));
            s.store_sqrt(2554, 2553);
            s.store_mul(2555, 2553, 2554);
        }

        s.b[2663] = (((-s.v[512]) * s.v[579]) == (-1.0));
        s.v[2663] = if s.b[2663] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && s.b[2663]) {
            s.store_div_from_scalar_offset_ad(2556, 1.0, A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && (!s.b[2663])) {
            s.store_pow_ad(2556, A::offset(A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0), A::mul_scaled_lhs(s.ad_value(512), -1.0, s.ad_value(579)));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(2557, 2546, 2556, 1.0, 2546, 1.0, 2556, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(2558, A::div(s.ad_value(2550), s.ad_value(2554)), 0.375);
            s.store_add_scaled_product_indices(2559, 2553, (-1.0), 2551, 2554, 2.0);
            s.store_add_scaled_value_products(2560, A::mul3(s.ad_value(603), s.ad_value(2551), s.ad_value(2554)), 1.0, s.ad_value(603), s.ad_value(2553), (-1.0), s.ad_value(2550), s.ad_value(2555), 0.5);
            s.store_mul_offset_lhs(2561, 2559, (-1.0), 2558);
            s.store_square(2522, 2561);
        }

        s.b[2664] = (s.v[2561] > 0.0);
        s.v[2664] = if s.b[2664] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && s.b[2664]) {
            s.store_div_from_scalar_offset_scaled_input(2523, 1.0, 2561, s.v[373], 1.0);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && (!s.b[2664])) {
            s.store_div_from_scalar_sub_from_scalar_ad(2523, 1.0, 1.0, A::scale(s.ad_value(2561), s.v[373]));
        }

        s.b[2665] = (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458));
        s.v[2665] = if s.b[2665] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && s.b[2665]) {
            s.store_exp_sub(2540, 2560, 2522);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && (!s.b[2665])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(2540, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) {
            s.store_mul_ad_lhs(2524, A::add_scaled_inputs_product(s.ad_value(2523), 0.29214664, A::square(s.ad_value(2523)), s.v[374], A::square(s.ad_value(2523)), s.ad_value(2523), s.v[375]), 2540);
        }

        s.b[2666] = (s.v[2561] > 0.0);
        s.v[2666] = if s.b[2666] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && s.b[2666]) {
            s.copy_ad(2562, 2524);
        }

        s.b[2667] = (s.v[2560] > (-230.25850929940458));
        s.v[2667] = if s.b[2667] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && (!s.b[2666])) && s.b[2667]) {
            s.store_exp(2540, 2560);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && (!s.b[2666])) && (!s.b[2667])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 2560, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && (!s.b[2666])) {
            s.store_sub_scaled_inputs(2562, 2540, 2.0, 2524, 1.0);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) {
            s.store_div_scaled_product_indices(2563, 603, 2562, (1.772453850905516 * 0.5), 2558, 1.0);
            s.store_mul_product3_rhs(2549, 526, s.ad_value(2548), s.ad_value(2563), s.ad_value(2557), 1.0);
        }

    }

    pub(super) fn stamp_transient_block_42(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2668] = (s.v[532] == 0.0);
        s.v[2668] = if s.b[2668] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && s.b[2668]) {
            s.store_scalar(2564, 0.0);
        }

        s.b[2669] = (s.v[512] == 0.5);
        s.v[2669] = if s.b[2669] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) && s.b[2669]) {
            s.store_sqrt_mul_ad(2540, A::sub(s.ad_value(509), s.ad_value(2538)), s.ad_value(597));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) && (!s.b[2669])) {
            s.store_pow_ad(2540, A::mul(A::sub(s.ad_value(509), s.ad_value(2538)), s.ad_value(597)), s.ad_value(512));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) {
            s.store_mul_div_scaled_product_rhs(2565, 579, A::sub(s.ad_value(509), s.ad_value(2538)), s.ad_value(594), 1.0, s.ad_value(2540), 1.0);
        }

        s.b[2670] = (((((-s.v[609]) / s.v[2565])) as f64).abs() < 230.25850929940458);
        s.v[2670] = if s.b[2670] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) && s.b[2670]) {
            s.store_exp_ad(2540, A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(2565), 1.0));
        }

        s.b[2671] = (((-s.v[609]) / s.v[2565]) < 0.0);
        s.v[2671] = if s.b[2671] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) && (!s.b[2670])) && s.b[2671]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(2540, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(2565), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(2565), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) && (!s.b[2670])) && (!s.b[2671])) {
            let assign60080_ad_e77478: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(2565), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(2565), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(2565), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2540, assign60080_ad_e77478, 1e100);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) {
            s.store_mul_ad_product_lhs(2564, s.ad_value(532), A::mul3(s.ad_value(822), s.ad_value(2565), s.ad_value(2565)), 2540);
        }

        s.b[2672] = (s.v[541] > 1000.0);
        s.v[2672] = if s.b[2672] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && s.b[2672]) {
            s.store_scalar(2566, 1.0);
        }

        s.b[2673] = (s.v[2539] > ((-s.v[445]) * s.v[541]));
        s.v[2673] = if s.b[2673] { 1.0 } else { 0.0 };

        s.b[2674] = (s.v[544] == 4.0);
        s.v[2674] = if s.b[2674] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2672])) && s.b[2673]) && s.b[2674]) {
            s.store_mul_ad_product_lhs(2540, A::mul3(A::mul3(s.ad_value(2539), s.ad_value(615), A::mul(s.ad_value(2539), s.ad_value(615))), s.ad_value(2539), s.ad_value(615)), s.ad_value(2539), 615);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2672])) && s.b[2673]) && (!s.b[2674])) {
            s.store_pow_ad(2540, A::abs(A::mul(s.ad_value(2539), s.ad_value(615))), s.ad_value(544));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2672])) && s.b[2673]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2566, 1.0, 1.0, s.ad_value(2540));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2672])) && (!s.b[2673])) {
            s.store_add_scaled_product_left_ad(2566, 612, 1.0, A::add_scaled_inputs(s.ad_value(2539), 1.0, s.ad_value(541), s.v[445]), 618, 1.0);
        }

        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) {
            s.store_mul_scale_ad_lhs(1899, A::add_scaled_inputs4(s.ad_value(2541), 1.0, s.ad_value(2542), 1.0, s.ad_value(2549), 1.0, s.ad_value(2564), 1.0), p.p29, 2566);
        }

        s.b[2675] = (s.v[576] == 0.5);
        s.v[2675] = if s.b[2675] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && s.b[2675]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2532), s.ad_value(573)));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2675])) {
            s.store_pow_ad(2540, A::sub_from_scalar(1.0, A::mul(s.ad_value(2532), s.ad_value(573))), s.ad_value(576));
        }

        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) {
            s.store_add_scaled_product_mixed_aia(1905, A::mul_sub_from_scalar_rhs(s.ad_value(585), 1.0, s.ad_value(2540)), p.p30, 588, A::sub(s.ad_value(822), s.ad_value(2532)), p.p30);
        }

        s.b[2676] = (s.v[675] == 0.0);
        s.v[2676] = if s.b[2676] { 1.0 } else { 0.0 };

        if ((s.b[2569] && (!s.b[2570])) && s.b[2676]) {
            s.store_scalar(1900, 0.0);
            s.store_scalar(1906, 0.0);
        }

        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) {
            s.store_mul(2541, 565, 2531);
        }

        s.b[2677] = ((s.v[524] == 0.0) && (s.v[527] == 0.0));
        s.v[2677] = if s.b[2677] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && s.b[2677]) {
            s.store_scalar(2542, 0.0);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2677])) {
            s.store_sub(2543, 571, 2537);
            s.store_sub_from_scalar_ad(2544, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2535), s.ad_value(2543)))));
        }

        s.b[2678] = (s.v[513] == 0.5);
        s.v[2678] = if s.b[2678] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2677])) && s.b[2678]) {
            s.store_scalar(2545, 0.0);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2677])) && (!s.b[2678])) {
            s.store_mul_sub_from_scalar_rhs_ad(2545, A::add(A::div_scaled_product(A::square(s.ad_value(2544)), A::ln(s.ad_value(2544)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2544)), 1.0), s.ad_value(2544)), 1.0, A::scale(s.ad_value(513), 2.0));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2677])) {
            s.store_add(2546, 2544, 2545);
        }

        s.b[2679] = (s.v[513] == 0.5);
        s.v[2679] = if s.b[2679] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2677])) && s.b[2679]) {
            s.store_sqrt_mul(2540, 2543, 598);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2677])) && (!s.b[2679])) {
            s.store_pow_ad(2540, A::mul(s.ad_value(2543), s.ad_value(598)), s.ad_value(513));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2677])) {
            s.store_mul(2547, 592, 2540);
            s.store_mul_ad_product_lhs(2548, s.ad_value(562), A::offset(s.ad_value(2534), (-1.0)), 2547);
            s.store_mul3_lhs(2542, 524, 2548, 2546);
        }

        s.b[2680] = (s.v[527] == 0.0);
        s.v[2680] = if s.b[2680] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && s.b[2680]) {
            s.store_scalar(2549, 0.0);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) {
            s.store_mul_div_scaled_product_rhs(2550, 607, s.ad_value(2547), s.ad_value(577), 1.0, s.ad_value(2543), 1.0);
            s.store_div_scaled_inputs_indices(2551, 604, 0.666666666666667, 2550, 1.0);
            s.store_square(2552, 2551);
            s.store_sqrt_ad(2553, A::div_scaled_product_offset_denominator(s.ad_value(2552), s.ad_value(2552), 1.0, A::square(s.ad_value(2552)), 1.0, 1.0));
            s.store_sqrt(2554, 2553);
            s.store_mul(2555, 2553, 2554);
        }

        s.b[2681] = (((-s.v[513]) * s.v[580]) == (-1.0));
        s.v[2681] = if s.b[2681] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && s.b[2681]) {
            s.store_div_from_scalar_offset_ad(2556, 1.0, A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && (!s.b[2681])) {
            s.store_pow_ad(2556, A::offset(A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0), A::mul_scaled_lhs(s.ad_value(513), -1.0, s.ad_value(580)));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(2557, 2546, 2556, 1.0, 2546, 1.0, 2556, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(2558, A::div(s.ad_value(2550), s.ad_value(2554)), 0.375);
            s.store_add_scaled_product_indices(2559, 2553, (-1.0), 2551, 2554, 2.0);
            s.store_add_scaled_value_products(2560, A::mul3(s.ad_value(604), s.ad_value(2551), s.ad_value(2554)), 1.0, s.ad_value(604), s.ad_value(2553), (-1.0), s.ad_value(2550), s.ad_value(2555), 0.5);
            s.store_mul_offset_lhs(2561, 2559, (-1.0), 2558);
            s.store_square(2522, 2561);
        }

        s.b[2682] = (s.v[2561] > 0.0);
        s.v[2682] = if s.b[2682] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && s.b[2682]) {
            s.store_div_from_scalar_offset_scaled_input(2523, 1.0, 2561, s.v[373], 1.0);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && (!s.b[2682])) {
            s.store_div_from_scalar_sub_from_scalar_ad(2523, 1.0, 1.0, A::scale(s.ad_value(2561), s.v[373]));
        }

        s.b[2683] = (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458));
        s.v[2683] = if s.b[2683] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && s.b[2683]) {
            s.store_exp_sub(2540, 2560, 2522);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && (!s.b[2683])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(2540, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) {
            s.store_mul_ad_lhs(2524, A::add_scaled_inputs_product(s.ad_value(2523), 0.29214664, A::square(s.ad_value(2523)), s.v[374], A::square(s.ad_value(2523)), s.ad_value(2523), s.v[375]), 2540);
        }

        s.b[2684] = (s.v[2561] > 0.0);
        s.v[2684] = if s.b[2684] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && s.b[2684]) {
            s.copy_ad(2562, 2524);
        }

        s.b[2685] = (s.v[2560] > (-230.25850929940458));
        s.v[2685] = if s.b[2685] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && (!s.b[2684])) && s.b[2685]) {
            s.store_exp(2540, 2560);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && (!s.b[2684])) && (!s.b[2685])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 2560, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && (!s.b[2684])) {
            s.store_sub_scaled_inputs(2562, 2540, 2.0, 2524, 1.0);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) {
            s.store_div_scaled_product_indices(2563, 604, 2562, (1.772453850905516 * 0.5), 2558, 1.0);
            s.store_mul_product3_rhs(2549, 527, s.ad_value(2548), s.ad_value(2563), s.ad_value(2557), 1.0);
        }

        s.b[2686] = (s.v[533] == 0.0);
        s.v[2686] = if s.b[2686] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && s.b[2686]) {
            s.store_scalar(2564, 0.0);
        }

        s.b[2687] = (s.v[513] == 0.5);
        s.v[2687] = if s.b[2687] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) && s.b[2687]) {
            s.store_sqrt_mul_ad(2540, A::sub(s.ad_value(510), s.ad_value(2538)), s.ad_value(598));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) && (!s.b[2687])) {
            s.store_pow_ad(2540, A::mul(A::sub(s.ad_value(510), s.ad_value(2538)), s.ad_value(598)), s.ad_value(513));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) {
            s.store_mul_div_scaled_product_rhs(2565, 580, A::sub(s.ad_value(510), s.ad_value(2538)), s.ad_value(595), 1.0, s.ad_value(2540), 1.0);
        }

        s.b[2688] = (((((-s.v[610]) / s.v[2565])) as f64).abs() < 230.25850929940458);
        s.v[2688] = if s.b[2688] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) && s.b[2688]) {
            s.store_exp_ad(2540, A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(2565), 1.0));
        }

        s.b[2689] = (((-s.v[610]) / s.v[2565]) < 0.0);
        s.v[2689] = if s.b[2689] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) && (!s.b[2688])) && s.b[2689]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(2540, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(2565), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(2565), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) && (!s.b[2688])) && (!s.b[2689])) {
            let assign60830_ad_e78744: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(2565), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(2565), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(2565), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2540, assign60830_ad_e78744, 1e100);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) {
            s.store_mul_ad_product_lhs(2564, s.ad_value(533), A::mul3(s.ad_value(822), s.ad_value(2565), s.ad_value(2565)), 2540);
        }

        s.b[2690] = (s.v[542] > 1000.0);
        s.v[2690] = if s.b[2690] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && s.b[2690]) {
            s.store_scalar(2566, 1.0);
        }

        s.b[2691] = (s.v[2539] > ((-s.v[445]) * s.v[542]));
        s.v[2691] = if s.b[2691] { 1.0 } else { 0.0 };

        s.b[2692] = (s.v[545] == 4.0);
        s.v[2692] = if s.b[2692] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2690])) && s.b[2691]) && s.b[2692]) {
            s.store_mul_ad_product_lhs(2540, A::mul3(A::mul3(s.ad_value(2539), s.ad_value(616), A::mul(s.ad_value(2539), s.ad_value(616))), s.ad_value(2539), s.ad_value(616)), s.ad_value(2539), 616);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2690])) && s.b[2691]) && (!s.b[2692])) {
            s.store_pow_ad(2540, A::abs(A::mul(s.ad_value(2539), s.ad_value(616))), s.ad_value(545));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2690])) && s.b[2691]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2566, 1.0, 1.0, s.ad_value(2540));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2690])) && (!s.b[2691])) {
            s.store_add_scaled_product_left_ad(2566, 613, 1.0, A::add_scaled_inputs(s.ad_value(2539), 1.0, s.ad_value(542), s.v[445]), 619, 1.0);
        }

        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) {
            s.store_mul_scale_ad_lhs(1900, A::add_scaled_inputs4(s.ad_value(2541), 1.0, s.ad_value(2542), 1.0, s.ad_value(2549), 1.0, s.ad_value(2564), 1.0), p.p29, 2566);
        }

        s.b[2693] = (s.v[577] == 0.5);
        s.v[2693] = if s.b[2693] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && s.b[2693]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2532), s.ad_value(574)));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2693])) {
            s.store_pow_ad(2540, A::sub_from_scalar(1.0, A::mul(s.ad_value(2532), s.ad_value(574))), s.ad_value(577));
        }

        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) {
            s.store_add_scaled_product_mixed_aia(1906, A::mul_sub_from_scalar_rhs(s.ad_value(586), 1.0, s.ad_value(2540)), p.p30, 589, A::sub(s.ad_value(822), s.ad_value(2532)), p.p30);
        }

        s.b[2694] = (s.v[676] == 0.0);
        s.v[2694] = if s.b[2694] { 1.0 } else { 0.0 };

        if ((s.b[2569] && (!s.b[2570])) && s.b[2694]) {
            s.store_scalar(1901, 0.0);
            s.store_scalar(1907, 0.0);
        }

        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) {
            s.store_mul(2541, 566, 2531);
        }

        s.b[2695] = ((s.v[525] == 0.0) && (s.v[528] == 0.0));
        s.v[2695] = if s.b[2695] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2695]) {
            s.store_scalar(2542, 0.0);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2695])) {
            s.store_sub(2543, 572, 2537);
            s.store_sub_from_scalar_ad(2544, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2535), s.ad_value(2543)))));
        }

        s.b[2696] = (s.v[514] == 0.5);
        s.v[2696] = if s.b[2696] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2695])) && s.b[2696]) {
            s.store_scalar(2545, 0.0);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2695])) && (!s.b[2696])) {
            s.store_mul_sub_from_scalar_rhs_ad(2545, A::add(A::div_scaled_product(A::square(s.ad_value(2544)), A::ln(s.ad_value(2544)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2544)), 1.0), s.ad_value(2544)), 1.0, A::scale(s.ad_value(514), 2.0));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2695])) {
            s.store_add(2546, 2544, 2545);
        }

        s.b[2697] = (s.v[514] == 0.5);
        s.v[2697] = if s.b[2697] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2695])) && s.b[2697]) {
            s.store_sqrt_mul(2540, 2543, 599);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2695])) && (!s.b[2697])) {
            s.store_pow_ad(2540, A::mul(s.ad_value(2543), s.ad_value(599)), s.ad_value(514));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2695])) {
            s.store_mul(2547, 593, 2540);
            s.store_mul_ad_product_lhs(2548, s.ad_value(563), A::offset(s.ad_value(2534), (-1.0)), 2547);
            s.store_mul3_lhs(2542, 525, 2548, 2546);
        }

        s.b[2698] = (s.v[528] == 0.0);
        s.v[2698] = if s.b[2698] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2698]) {
            s.store_scalar(2549, 0.0);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) {
            s.store_mul_div_scaled_product_rhs(2550, 608, s.ad_value(2547), s.ad_value(578), 1.0, s.ad_value(2543), 1.0);
            s.store_div_scaled_inputs_indices(2551, 605, 0.666666666666667, 2550, 1.0);
            s.store_square(2552, 2551);
            s.store_sqrt_ad(2553, A::div_scaled_product_offset_denominator(s.ad_value(2552), s.ad_value(2552), 1.0, A::square(s.ad_value(2552)), 1.0, 1.0));
            s.store_sqrt(2554, 2553);
            s.store_mul(2555, 2553, 2554);
        }

        s.b[2699] = (((-s.v[514]) * s.v[581]) == (-1.0));
        s.v[2699] = if s.b[2699] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && s.b[2699]) {
            s.store_div_from_scalar_offset_ad(2556, 1.0, A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && (!s.b[2699])) {
            s.store_pow_ad(2556, A::offset(A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0), A::mul_scaled_lhs(s.ad_value(514), -1.0, s.ad_value(581)));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) {
            s.store_div_scaled_product_add_scaled_denominator_indices(2557, 2546, 2556, 1.0, 2546, 1.0, 2556, 1.0, 1.0);
            s.store_sqrt_scaled_input_ad(2558, A::div(s.ad_value(2550), s.ad_value(2554)), 0.375);
            s.store_add_scaled_product_indices(2559, 2553, (-1.0), 2551, 2554, 2.0);
            s.store_add_scaled_value_products(2560, A::mul3(s.ad_value(605), s.ad_value(2551), s.ad_value(2554)), 1.0, s.ad_value(605), s.ad_value(2553), (-1.0), s.ad_value(2550), s.ad_value(2555), 0.5);
            s.store_mul_offset_lhs(2561, 2559, (-1.0), 2558);
            s.store_square(2522, 2561);
        }

    }

    pub(super) fn stamp_transient_block_43(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[2700] = (s.v[2561] > 0.0);
        s.v[2700] = if s.b[2700] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && s.b[2700]) {
            s.store_div_from_scalar_offset_scaled_input(2523, 1.0, 2561, s.v[373], 1.0);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && (!s.b[2700])) {
            s.store_div_from_scalar_sub_from_scalar_ad(2523, 1.0, 1.0, A::scale(s.ad_value(2561), s.v[373]));
        }

        s.b[2701] = (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458));
        s.v[2701] = if s.b[2701] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && s.b[2701]) {
            s.store_exp_sub(2540, 2560, 2522);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && (!s.b[2701])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(2540, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) {
            s.store_mul_ad_lhs(2524, A::add_scaled_inputs_product(s.ad_value(2523), 0.29214664, A::square(s.ad_value(2523)), s.v[374], A::square(s.ad_value(2523)), s.ad_value(2523), s.v[375]), 2540);
        }

        s.b[2702] = (s.v[2561] > 0.0);
        s.v[2702] = if s.b[2702] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && s.b[2702]) {
            s.copy_ad(2562, 2524);
        }

        s.b[2703] = (s.v[2560] > (-230.25850929940458));
        s.v[2703] = if s.b[2703] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && (!s.b[2702])) && s.b[2703]) {
            s.store_exp(2540, 2560);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && (!s.b[2702])) && (!s.b[2703])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 2560, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && (!s.b[2702])) {
            s.store_sub_scaled_inputs(2562, 2540, 2.0, 2524, 1.0);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) {
            s.store_div_scaled_product_indices(2563, 605, 2562, (1.772453850905516 * 0.5), 2558, 1.0);
            s.store_mul_product3_rhs(2549, 528, s.ad_value(2548), s.ad_value(2563), s.ad_value(2557), 1.0);
        }

        s.b[2704] = (s.v[534] == 0.0);
        s.v[2704] = if s.b[2704] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2704]) {
            s.store_scalar(2564, 0.0);
        }

        s.b[2705] = (s.v[514] == 0.5);
        s.v[2705] = if s.b[2705] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) && s.b[2705]) {
            s.store_sqrt_mul_ad(2540, A::sub(s.ad_value(511), s.ad_value(2538)), s.ad_value(599));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) && (!s.b[2705])) {
            s.store_pow_ad(2540, A::mul(A::sub(s.ad_value(511), s.ad_value(2538)), s.ad_value(599)), s.ad_value(514));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) {
            s.store_mul_div_scaled_product_rhs(2565, 581, A::sub(s.ad_value(511), s.ad_value(2538)), s.ad_value(596), 1.0, s.ad_value(2540), 1.0);
        }

        s.b[2706] = (((((-s.v[611]) / s.v[2565])) as f64).abs() < 230.25850929940458);
        s.v[2706] = if s.b[2706] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) && s.b[2706]) {
            s.store_exp_ad(2540, A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(2565), 1.0));
        }

        s.b[2707] = (((-s.v[611]) / s.v[2565]) < 0.0);
        s.v[2707] = if s.b[2707] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) && (!s.b[2706])) && s.b[2707]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad(2540, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(2565), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(2565), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0), 1.0);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) && (!s.b[2706])) && (!s.b[2707])) {
            let assign61580_ad_e80010: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(2565), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(2565), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(2565), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2540, assign61580_ad_e80010, 1e100);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) {
            s.store_mul_ad_product_lhs(2564, s.ad_value(534), A::mul3(s.ad_value(822), s.ad_value(2565), s.ad_value(2565)), 2540);
        }

        s.b[2708] = (s.v[642] > 1000.0);
        s.v[2708] = if s.b[2708] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2708]) {
            s.store_scalar(2566, 1.0);
        }

        s.b[2709] = (s.v[2539] > ((-s.v[445]) * s.v[642]));
        s.v[2709] = if s.b[2709] { 1.0 } else { 0.0 };

        s.b[2710] = (s.v[546] == 4.0);
        s.v[2710] = if s.b[2710] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2708])) && s.b[2709]) && s.b[2710]) {
            s.store_mul_ad_product_lhs(2540, A::mul3(A::mul3(s.ad_value(2539), s.ad_value(617), A::mul(s.ad_value(2539), s.ad_value(617))), s.ad_value(2539), s.ad_value(617)), s.ad_value(2539), 617);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2708])) && s.b[2709]) && (!s.b[2710])) {
            s.store_pow_ad(2540, A::abs(A::mul(s.ad_value(2539), s.ad_value(617))), s.ad_value(546));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2708])) && s.b[2709]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2566, 1.0, 1.0, s.ad_value(2540));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2708])) && (!s.b[2709])) {
            s.store_add_scaled_product_left_ad(2566, 614, 1.0, A::add_scaled_inputs(s.ad_value(2539), 1.0, s.ad_value(642), s.v[445]), 620, 1.0);
        }

        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) {
            s.store_mul_scale_ad_lhs(1901, A::add_scaled_inputs4(s.ad_value(2541), 1.0, s.ad_value(2542), 1.0, s.ad_value(2549), 1.0, s.ad_value(2564), 1.0), p.p29, 2566);
        }

        s.b[2711] = (s.v[636] == 1.0);
        s.v[2711] = if s.b[2711] { 1.0 } else { 0.0 };

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
            s.store_div_scaled_product_add_scaled_denominator_indices(2568, 2567, 686, 2.0, 2528, 1.0, 2530, 1.0, 1.0);
        }

        s.b[2712] = (s.v[578] == 0.5);
        s.v[2712] = if s.b[2712] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) && s.b[2712]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2568), s.ad_value(575)));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) && (!s.b[2712])) {
            s.store_pow_ad(2540, A::sub_from_scalar(1.0, A::mul(s.ad_value(2568), s.ad_value(575))), s.ad_value(578));
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
        s.v[2713] = if s.b[2713] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) && s.b[2713]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2568), s.ad_value(630)));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) && (!s.b[2713])) {
            s.store_pow_ad(2540, A::sub_from_scalar(1.0, A::mul(s.ad_value(2568), s.ad_value(630))), s.ad_value(631));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) {
            s.store_add_scaled_product_mixed_aia(473, A::mul_sub_from_scalar_rhs(s.ad_value(634), 1.0, s.ad_value(2540)), p.p30, 635, A::sub(s.ad_value(2567), s.ad_value(2568)), p.p30);
            s.store_add(1907, 1907, 473);
        }

        s.b[2714] = (s.v[578] == 0.5);
        s.v[2714] = if s.b[2714] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2711])) && s.b[2714]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2532), s.ad_value(575)));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2711])) && (!s.b[2714])) {
            s.store_pow_ad(2540, A::sub_from_scalar(1.0, A::mul(s.ad_value(2532), s.ad_value(575))), s.ad_value(578));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2711])) {
            s.store_add_scaled_product_mixed_aia(1907, A::mul_sub_from_scalar_rhs(s.ad_value(587), 1.0, s.ad_value(2540)), p.p30, 590, A::sub(s.ad_value(822), s.ad_value(2532)), p.p30);
        }

        if (s.b[2569] && (!s.b[2570])) {
            s.store_add_scaled_products3(838, s.ad_value(674), s.ad_value(1899), 1.0, s.ad_value(675), s.ad_value(1900), 1.0, s.ad_value(676), s.ad_value(1901), 1.0);
        }

        s.b[2715] = (s.v[820] > 0.0);
        s.v[2715] = if s.b[2715] { 1.0 } else { 0.0 };

        s.b[2716] = (s.v[298] > 0.0);
        s.v[2716] = if s.b[2716] { 1.0 } else { 0.0 };

        s.b[2717] = (s.v[299] > 0.0);
        s.v[2717] = if s.b[2717] { 1.0 } else { 0.0 };

        s.b[2718] = (s.v[300] > 0.0);
        s.v[2718] = if s.b[2718] { 1.0 } else { 0.0 };

        s.b[2719] = (s.v[301] > 0.0);
        s.v[2719] = if s.b[2719] { 1.0 } else { 0.0 };

        s.b[2720] = (s.v[302] > 0.0);
        s.v[2720] = if s.b[2720] { 1.0 } else { 0.0 };

        s.b[2721] = (s.v[303] > 0.0);
        s.v[2721] = if s.b[2721] { 1.0 } else { 0.0 };

        s.b[2722] = (s.v[304] > 0.0);
        s.v[2722] = if s.b[2722] { 1.0 } else { 0.0 };

        s.v[1915] = 0.0;

        s.v[2723] = 0.0;

        s.v[2724] = 0.0;

        s.b[2725] = (s.v[299] > 0.0);
        s.v[2725] = if s.b[2725] { 1.0 } else { 0.0 };

        if s.b[2725] {
            s.store_mul_ad_product_rhs(2723, 801, A::voltage(ctx, nodes, Some(2), Some(7)), A::voltage(ctx, nodes, Some(2), Some(7)));
        }

        s.b[2726] = (s.v[300] > 0.0);
        s.v[2726] = if s.b[2726] { 1.0 } else { 0.0 };

        if s.b[2726] {
            s.store_mul_ad_product_rhs(2724, 802, A::voltage(ctx, nodes, Some(0), Some(8)), A::voltage(ctx, nodes, Some(0), Some(8)));
        }

        s.b[2727] = (s.v[172] > 0.001);
        s.v[2727] = if s.b[2727] { 1.0 } else { 0.0 };

        if s.b[2727] {
            s.store_add_scaled_inputs3_mixed_aii(1915, A::add_scaled_products(A::add(s.ad_value(827), s.ad_value(835)), s.ad_value(815), 1.0, s.ad_value(836), A::add(s.ad_value(815), s.ad_value(816)), 1.0), 1.0, 2723, 1.0, 2724, 1.0);
        }

        s.store_neg_ad(839, A::add_scaled_inputs3(s.ad_value(840), 1.0, s.ad_value(841), 1.0, s.ad_value(842), 1.0));

        s.store_add(843, 843, 1894);

        s.store_add(844, 844, 1895);

        s.store_add_scaled_products3(846, s.ad_value(647), s.ad_value(1902), 1.0, s.ad_value(648), s.ad_value(1903), 1.0, s.ad_value(649), s.ad_value(1904), 1.0);

        s.store_add_scaled_products3(847, s.ad_value(674), s.ad_value(1905), 1.0, s.ad_value(675), s.ad_value(1906), 1.0, s.ad_value(676), s.ad_value(1907), 1.0);

        s.b[2729] = (s.v[820] < 0.0);
        s.v[2729] = if s.b[2729] { 1.0 } else { 0.0 };

        if s.b[2729] {
            s.copy_ad(2728, 842);
            s.copy_ad(842, 839);
            s.copy_ad(839, 2728);
        }

        s.v[2746] = 0.0;

        s.v[2741] = 0.0;

        s.v[848] = 1e-40;

        s.v[850] = 0.0;

        s.v[852] = 0.0;

        s.store_mul(849, 1888, 1879);

        s.v[851] = 0.0;

        s.v[2748] = 0.0;

        s.b[2762] = ((s.v[1813] > 0.0) && (s.v[1917] > 0.0));
        s.v[2762] = if s.b[2762] { 1.0 } else { 0.0 };

        s.b[2764] = (p.p32 > 0.0);
        s.v[2764] = if s.b[2764] { 1.0 } else { 0.0 };

        if (s.b[2762] && s.b[2764]) {
            s.store_div(2733, 1850, 1848);
            s.store_div(2734, 1849, 1850);
            s.store_scaled_div(2735, 1844, 2733, (0.5 * 0.16666666666666666));
            s.store_square(2736, 2735);
            s.store_offset_div(2737, 2733, 1861, (-1.0));
        }

        if (s.b[2762] && s.b[2764]) {
            if ((1.0 - (12.0 * (s.v[2737] * s.v[2736]))) > 1e-20) {
                s.store_sub_from_scalar_scaled_mul(2738, 1.0, 2737, 2736, 12.0);
            } else {
                s.store_scalar(2738, 1e-20);
            }
        }

        if (s.b[2762] && s.b[2764]) {
            s.store_div_from_scalar_square_ad(2739, 1.0, s.ad_value(2738));
            s.store_mul3_lhs(2740, 1917, 1850, 1860);
            s.store_add_scaled_inputs3_mixed_iia(2741, 2734, 1.0, 2736, 12.0, A::mul3_scaled_output(A::offset(s.ad_value(2734), 1.0), s.ad_value(2736), s.ad_value(2737), 24.0), -1.0);
        }

        if (s.b[2762] && s.b[2764]) {
            if (s.v[2741] > 1e-40) {
            } else {
                s.store_scalar(2741, 1e-40);
            }
        }

        if (s.b[2762] && s.b[2764]) {
            s.store_mul3_lhs(2741, 2740, 2739, 2741);
        }

        s.b[2765] = (s.v[275] > 0.0);
        s.v[2765] = if s.b[2765] { 1.0 } else { 0.0 };

        if ((s.b[2762] && s.b[2764]) && s.b[2765]) {
            s.store_div(2742, 1854, 1853);
            s.store_mul_ad_product_lhs(2743, A::square(s.ad_value(2742)), s.ad_value(1844), 1844);
        }

        s.b[2766] = (s.v[0] == (-1.0));
        s.v[2766] = if s.b[2766] { 1.0 } else { 0.0 };

        if (((s.b[2762] && s.b[2764]) && s.b[2765]) && s.b[2766]) {
            s.store_div_scaled_value_offset_denominator(2743, s.ad_value(2743), 1.0, A::mul(s.ad_value(2742), s.ad_value(1844)), 1.0, 1.0);
        }

        if ((s.b[2762] && s.b[2764]) && s.b[2765]) {
            s.store_mul_offset_rhs_scaled_ad_rhs(2744, 1853, A::sqrt(A::scale_offset(s.ad_value(2743), 2.0, 1.0)), 1.0, 0.5);
            s.store_div_ad_rhs(2745, 1853, A::mul(s.ad_value(2744), s.ad_value(2738)));
            s.store_mul_ad_product_lhs(2746, A::mul3(s.ad_value(799), s.ad_value(827), s.ad_value(1841)), s.ad_value(2745), 2745);
            s.store_add_ad_rhs(2741, 2741, A::div(s.ad_value(2746), s.ad_value(1919)));
        }

        if (s.b[2762] && s.b[2764]) {
            s.store_sqrt_mul(851, 1920, 2741);
        }

        s.b[2767] = ((((p.p50 == 1.0) && (s.v[1920] > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0));
        s.v[2767] = if s.b[2767] { 1.0 } else { 0.0 };

        if (s.b[2762] && s.b[2767]) {
            s.store_sub_ad(848, A::add_scaled_product(s.ad_value(2734), 0.08333333333333333, s.ad_value(2736), A::sub_scaled_inputs(A::offset(s.ad_value(2734), 0.2), 1.0, s.ad_value(2736), 12.0), (-1.0)), A::mul3_scaled_output(s.ad_value(2736), A::sub_scaled_inputs(A::offset(s.ad_value(2734), 1.0), 1.0, s.ad_value(2736), 12.0), s.ad_value(2737), 1.6));
        }

        if (s.b[2762] && s.b[2767]) {
            if (s.v[848] > 1e-40) {
            } else {
                s.store_scalar(848, 1e-40);
            }
        }

        if (s.b[2762] && s.b[2767]) {
            s.store_mul_div_lhs(848, 2739, 2740, 848);
            s.store_mul_ad_product_rhs(2747, 2739, s.ad_value(2735), A::add_scaled_sub_value_product(1.0, A::scale(s.ad_value(2736), 12.0), 1.0, A::add_scaled_inputs_product(s.ad_value(2734), 1.0, s.ad_value(2736), 19.2, s.ad_value(2734), s.ad_value(2736), (-12.0)), s.ad_value(2737), (-1.0)));
            s.store_div_scaled_product3_mixed_aiia(849, A::square(s.ad_value(1892)), 1888, 1879, 1.0, A::square(s.ad_value(1890)), 1.0);
        }

        s.b[2768] = (s.v[275] > 0.0);
        s.v[2768] = if s.b[2768] { 1.0 } else { 0.0 };

        if ((s.b[2762] && s.b[2767]) && s.b[2768]) {
            s.store_add_ad_rhs(848, 848, A::div_scaled_product(s.ad_value(2746), A::scale_offset(s.ad_value(2736), 12.0, 1.0), 1.0, A::mul3_scaled_output(s.ad_value(2740), s.ad_value(2740), s.ad_value(1919), 12.0), 1.0));
            s.store_sub_ad_rhs(2747, 2747, A::div_scaled_product3_by_product(s.ad_value(2746), s.ad_value(2735), A::offset(s.ad_value(2737), 1.0), 1.0, s.ad_value(2740), s.ad_value(1919), 1.0));
        }

        if (s.b[2762] && s.b[2767]) {
            s.store_sqrt_div(2748, 1920, 848);
        }

        s.b[2769] = (s.v[851] <= 0.0);
        s.v[2769] = if s.b[2769] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_44(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[2762] && s.b[2767]) && s.b[2769]) {
            s.store_scalar(852, 0.0);
        }

        if ((s.b[2762] && s.b[2767]) && (!s.b[2769])) {
            s.store_div_scaled_product_indices(852, 2747, 2748, 1.0, 851, 1.0);
        }

        if (s.b[2762] && s.b[2767]) {
            if (s.v[852] > 0.0) {
                if (s.v[852] < 1.0) {
                } else {
                    s.store_scalar(852, 1.0);
                }
            } else {
                s.store_scalar(852, 0.0);
            }
        }

        if (s.b[2762] && s.b[2767]) {
            s.store_div_scaled_product_indices(850, 852, 851, 1.0, 2748, 1.0);
        }

        s.b[2771] = (((p.p46 != 0.0) && (s.v[285] > 0.0)) && (s.v[1864] > 0.0));
        s.v[2771] = if s.b[2771] { 1.0 } else { 0.0 };

        if s.b[2771] {
            s.store_div_scaled_inputs_indices(1930, 1867, 4.0, 1925, 1.0);
            s.store_mul(1930, 760, 1916);
            s.store_mul(1930, 1848, 1861);
        }

    }

    pub(super) fn stamp_reactive_block_0(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[981] = (p.p37 >= 0.0);
        s.v[981] = if s.b[981] { 1.0 } else { 0.0 };

        if s.b[981] {
            s.store_scalar(0, 1.0);
        }

        if (!s.b[981]) {
            s.store_scalar(0, (-1.0));
        }

        s.v[756] = (8.8541878176e-12 * 11.8);

        s.v[351] = (273.15 + p.p38);

        s.v[475] = 0.0;

        s.b[982] = (p.p944 > 0.5);
        s.v[982] = if s.b[982] { 1.0 } else { 0.0 };

        if s.b[982] {
            s.store_scalar(475, 1.0);
        }

        if (!s.b[982]) {
            s.store_scalar(475, 0.0);
        }

        s.v[365] = (273.15 + p.p840);

        s.v[368] = (1.3806505e-23 / 1.6021918e-19);

        s.v[369] = (s.v[368] * s.v[365]);

        s.v[370] = (1.0 / s.v[369]);

        s.v[376] = ((-((0.000702 * s.v[365]) * s.v[365])) / (1108.0 + s.v[365]));

        s.v[379] = (p.p851 + s.v[376]);

        s.v[380] = (p.p852 + s.v[376]);

        s.v[381] = (p.p853 + s.v[376]);

        s.v[409] = (1.0 - p.p848);

        s.v[410] = (1.0 - p.p849);

        s.v[411] = (1.0 - p.p850);

        s.v[412] = (1.0 / s.v[409]);

        s.v[413] = (1.0 / s.v[410]);

        s.v[414] = (1.0 / s.v[411]);

        s.v[424] = (s.v[756] / p.p842);

        s.v[425] = ((p.p860 * s.v[756]) / p.p843);

        s.v[426] = ((p.p861 * s.v[756]) / p.p844);

        s.v[427] = (1.0 / s.v[424]);

        s.v[428] = (1.0 / s.v[425]);

        s.v[429] = (1.0 / s.v[426]);

        s.v[430] = (1.0 / p.p845);

        s.v[431] = (1.0 / p.p846);

        s.v[432] = (1.0 / p.p847);

        s.v[445] = (1.0 - (1.0 / p.p841));

        s.v[449] = (1.0 / p.p877);

        s.v[450] = (1.0 / p.p878);

        s.v[451] = (1.0 / p.p879);

        s.b[983] = ((((p.p883 != 1.0) || (p.p884 != 1.0)) || (p.p885 != 1.0)) || (p.p886 != 1.0));
        s.v[983] = if s.b[983] { 1.0 } else { 0.0 };

        if s.b[983] {
            s.store_scalar(474, 1.0);
        }

        if (!s.b[983]) {
            s.store_scalar(474, 0.0);
        }

        s.b[984] = (s.v[474] == 1.0);
        s.v[984] = if s.b[984] { 1.0 } else { 0.0 };

        if s.b[984] {
            s.store_scalar(458, (if ((p.p844 * p.p883) > 1e-18) { (p.p844 * p.p883) } else { 1e-18 }));
        }

        if s.b[984] {
            s.store_scalar(459, (if ((p.p847 * p.p884) > 0.05) { (p.p847 * p.p884) } else { 0.05 }));
        }

        if s.b[984] {
            s.store_scalar(460, (if ((if ((p.p850 * p.p885) > 0.05) { (p.p850 * p.p885) } else { 0.05 }) < 0.95) { (if ((p.p850 * p.p885) > 0.05) { (p.p850 * p.p885) } else { 0.05 }) } else { 0.95 }));
        }

        if s.b[984] {
            s.store_scalar(461, (p.p853 * p.p886));
            s.store_offset(463, 461, s.v[376]);
            s.store_sub_from_scalar(468, 1.0, 460);
            s.store_div_from_scalar(469, 1.0, 468);
        }

        s.b[985] = (p.p44 == 0.0);
        s.v[985] = if s.b[985] { 1.0 } else { 0.0 };

        if s.b[985] {
            s.store_scalar(506, p.p842);
            s.store_scalar(507, p.p843);
            s.store_scalar(508, p.p844);
            s.store_scalar(509, p.p845);
            s.store_scalar(510, p.p846);
            s.store_scalar(511, p.p847);
            s.store_scalar(512, p.p848);
            s.store_scalar(513, p.p849);
            s.store_scalar(514, p.p850);
            s.store_scalar(515, p.p851);
            s.store_scalar(516, p.p852);
            s.store_scalar(517, p.p853);
            s.store_scalar(518, p.p854);
            s.store_scalar(519, p.p855);
            s.store_scalar(520, p.p856);
            s.store_scalar(523, p.p857);
            s.store_scalar(524, p.p858);
            s.store_scalar(525, p.p859);
            s.store_scalar(521, p.p860);
            s.store_scalar(522, p.p861);
            s.store_scalar(526, p.p862);
            s.store_scalar(527, p.p863);
            s.store_scalar(528, p.p864);
            s.store_scalar(529, p.p865);
            s.store_scalar(530, p.p866);
            s.store_scalar(531, p.p867);
            s.store_scalar(532, p.p868);
            s.store_scalar(533, p.p869);
            s.store_scalar(534, p.p870);
            s.store_scalar(535, p.p871);
            s.store_scalar(536, p.p872);
            s.store_scalar(537, p.p873);
            s.store_scalar(538, p.p874);
            s.store_scalar(539, p.p875);
            s.store_scalar(540, p.p876);
            s.store_scalar(541, p.p877);
            s.store_scalar(542, p.p878);
            s.store_scalar(543, p.p879);
            s.store_scalar(544, p.p880);
            s.store_scalar(545, p.p881);
            s.store_scalar(546, p.p882);
            s.store_scalar(554, p.p946);
            s.store_scalar(637, p.p889);
            s.store_scalar(638, p.p890);
            s.store_scalar(639, p.p891);
            s.store_scalar(640, p.p892);
            s.store_scalar(547, p.p883);
            s.store_scalar(548, p.p884);
            s.store_scalar(549, p.p885);
            s.store_scalar(550, p.p886);
            s.store_scalar(551, p.p887);
            s.store_scalar(552, p.p888);
        }

        if (!s.b[985]) {
            s.store_scalar(506, p.p893);
            s.store_scalar(507, p.p894);
            s.store_scalar(508, p.p895);
            s.store_scalar(509, p.p896);
            s.store_scalar(510, p.p897);
            s.store_scalar(511, p.p898);
            s.store_scalar(512, p.p899);
            s.store_scalar(513, p.p900);
            s.store_scalar(514, p.p901);
            s.store_scalar(515, p.p902);
            s.store_scalar(516, p.p903);
            s.store_scalar(517, p.p904);
            s.store_scalar(518, p.p905);
            s.store_scalar(519, p.p906);
            s.store_scalar(520, p.p907);
            s.store_scalar(523, p.p908);
            s.store_scalar(524, p.p909);
            s.store_scalar(525, p.p910);
            s.store_scalar(521, p.p911);
            s.store_scalar(522, p.p912);
            s.store_scalar(526, p.p913);
            s.store_scalar(527, p.p914);
            s.store_scalar(528, p.p915);
            s.store_scalar(529, p.p916);
            s.store_scalar(530, p.p917);
            s.store_scalar(531, p.p918);
            s.store_scalar(532, p.p919);
            s.store_scalar(533, p.p920);
            s.store_scalar(534, p.p921);
            s.store_scalar(535, p.p922);
            s.store_scalar(536, p.p923);
            s.store_scalar(537, p.p924);
            s.store_scalar(538, p.p925);
            s.store_scalar(539, p.p926);
            s.store_scalar(540, p.p927);
            s.store_scalar(541, p.p928);
            s.store_scalar(542, p.p929);
            s.store_scalar(543, p.p930);
            s.store_scalar(544, p.p931);
            s.store_scalar(545, p.p932);
            s.store_scalar(546, p.p933);
            s.store_scalar(554, p.p948);
            s.store_scalar(637, p.p940);
            s.store_scalar(638, p.p941);
        }

    }

    pub(super) fn stamp_reactive_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if (!s.b[985]) {
            s.store_scalar(639, p.p942);
            s.store_scalar(640, p.p943);
            s.store_scalar(547, p.p934);
            s.store_scalar(548, p.p935);
            s.store_scalar(549, p.p936);
            s.store_scalar(550, p.p937);
            s.store_scalar(551, p.p938);
            s.store_scalar(552, p.p939);
        }

        s.store_offset(555, 515, s.v[376]);

        s.store_offset(556, 516, s.v[376]);

        s.store_offset(557, 517, s.v[376]);

        s.store_sub_from_scalar(576, 1.0, 512);

        s.store_sub_from_scalar(577, 1.0, 513);

        s.store_sub_from_scalar(578, 1.0, 514);

        s.store_div_from_scalar(579, 1.0, 576);

        s.store_div_from_scalar(580, 1.0, 577);

        s.store_div_from_scalar(581, 1.0, 578);

        s.store_div_from_scalar(591, s.v[756], 506);

        s.store_div_scaled_inputs_indices(592, 521, s.v[756], 507, 1.0);

        s.store_div_scaled_inputs_indices(593, 522, s.v[756], 508, 1.0);

        s.store_div_from_scalar(594, 1.0, 591);

        s.store_div_from_scalar(595, 1.0, 592);

        s.store_div_from_scalar(596, 1.0, 593);

        s.store_div_from_scalar(597, 1.0, 509);

        s.store_div_from_scalar(598, 1.0, 510);

        s.store_div_from_scalar(599, 1.0, 511);

        s.store_div_from_scalar(615, 1.0, 541);

        s.store_div_from_scalar(616, 1.0, 542);

        s.store_div_from_scalar(617, 1.0, 543);

        s.b[986] = ((((s.v[547] != 1.0) || (s.v[548] != 1.0)) || (s.v[549] != 1.0)) || (s.v[550] != 1.0));
        s.v[986] = if s.b[986] { 1.0 } else { 0.0 };

        if s.b[986] {
            s.store_scalar(636, 1.0);
        }

        if (!s.b[986]) {
            s.store_scalar(636, 0.0);
        }

        s.b[987] = (s.v[636] == 1.0);
        s.v[987] = if s.b[987] { 1.0 } else { 0.0 };

        if s.b[987] {
            if ((s.v[508] * s.v[547]) > 1e-18) {
                s.store_mul(621, 508, 547);
            } else {
                s.store_scalar(621, 1e-18);
            }
        }

        if s.b[987] {
            if ((s.v[511] * s.v[548]) > 0.05) {
                s.store_mul(622, 511, 548);
            } else {
                s.store_scalar(622, 0.05);
            }
        }

        if s.b[987] {
            if ((if ((s.v[514] * s.v[549]) > 0.05) { (s.v[514] * s.v[549]) } else { 0.05 }) < 0.95) {
                if ((s.v[514] * s.v[549]) > 0.05) {
                    s.store_mul(623, 514, 549);
                } else {
                    s.store_scalar(623, 0.05);
                }
            } else {
                s.store_scalar(623, 0.95);
            }
        }

        if s.b[987] {
            s.store_mul(624, 517, 550);
            s.store_offset(626, 624, s.v[376]);
            s.store_sub_from_scalar(631, 1.0, 623);
            s.store_div_from_scalar(632, 1.0, 631);
        }

        s.v[352] = ((ctx_temp + p.p55) + p.p35);

        s.v[353] = (s.v[352] / s.v[351]);

        s.v[354] = (s.v[352] - s.v[351]);

        s.v[355] = ((s.v[352] * 1.3806505e-23) / 1.6021918e-19);

        s.v[356] = (1.0 / s.v[355]);

        s.v[366] = (((ctx_temp + p.p55) + p.p35)).max((273.15 + (-250.0)));

        s.v[367] = (s.v[366] / s.v[365]);

        s.v[371] = (s.v[368] * s.v[366]);

        s.v[372] = (1.0 / s.v[371]);

        s.v[377] = ((-((0.000702 * s.v[366]) * s.v[366])) / (1108.0 + s.v[366]));

        s.v[382] = (p.p851 + s.v[377]);

        s.v[383] = (p.p852 + s.v[377]);

        s.v[384] = (p.p853 + s.v[377]);

        s.v[385] = (((s.v[367]) as f64).powf(1.5) * (((0.5 * ((s.v[379] * s.v[370]) - (s.v[382] * s.v[372])))) as f64).exp());

        s.v[386] = (((s.v[367]) as f64).powf(1.5) * (((0.5 * ((s.v[380] * s.v[370]) - (s.v[383] * s.v[372])))) as f64).exp());

        s.v[387] = (((s.v[367]) as f64).powf(1.5) * (((0.5 * ((s.v[381] * s.v[370]) - (s.v[384] * s.v[372])))) as f64).exp());

        s.v[388] = ((p.p854 * s.v[385]) * s.v[385]);

        s.v[389] = ((p.p855 * s.v[386]) * s.v[386]);

        s.v[390] = ((p.p856 * s.v[387]) * s.v[387]);

        s.v[391] = ((p.p845 * s.v[367]) - ((2.0 * s.v[371]) * ((s.v[385]) as f64).ln()));

        s.v[392] = ((p.p846 * s.v[367]) - ((2.0 * s.v[371]) * ((s.v[386]) as f64).ln()));

        s.v[393] = ((p.p847 * s.v[367]) - ((2.0 * s.v[371]) * ((s.v[387]) as f64).ln()));

        s.v[394] = (s.v[391] + (s.v[371] * (((1.0 + ((((0.05 - s.v[391]) * s.v[372])) as f64).exp())) as f64).ln()));

        s.v[395] = (s.v[392] + (s.v[371] * (((1.0 + ((((0.05 - s.v[392]) * s.v[372])) as f64).exp())) as f64).ln()));

        s.v[396] = (s.v[393] + (s.v[371] * (((1.0 + ((((0.05 - s.v[393]) * s.v[372])) as f64).exp())) as f64).ln()));

        s.v[406] = (1.0 / s.v[394]);

        s.v[407] = (1.0 / s.v[395]);

        s.v[408] = (1.0 / s.v[396]);

        s.v[415] = (p.p842 * (((p.p845 * s.v[406])) as f64).powf(p.p848));

        s.v[416] = (p.p843 * (((p.p846 * s.v[407])) as f64).powf(p.p849));

        s.v[417] = (p.p844 * (((p.p847 * s.v[408])) as f64).powf(p.p850));

        s.v[418] = ((s.v[415] * s.v[394]) * s.v[412]);

        s.v[419] = ((s.v[416] * s.v[395]) * s.v[413]);

        s.v[420] = ((s.v[417] * s.v[396]) * s.v[414]);

        s.v[421] = (2.0 * s.v[415]);

        s.v[422] = (2.0 * s.v[416]);

        s.v[423] = (2.0 * s.v[417]);

        s.v[433] = ((0.5 * s.v[382])).max(s.v[371]);

        s.v[434] = ((0.5 * s.v[383])).max(s.v[371]);

        s.v[435] = ((0.5 * s.v[384])).max(s.v[371]);

        s.v[436] = (s.v[433] * s.v[372]);

        s.v[437] = (s.v[434] * s.v[372]);

        s.v[438] = (s.v[435] * s.v[372]);

        s.v[439] = (((((((32.0 * p.p865) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[433] * s.v[433]) * s.v[433]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[440] = (((((((32.0 * p.p866) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[434] * s.v[434]) * s.v[434]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[441] = (((((((32.0 * p.p867) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[435] * s.v[435]) * s.v[435]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[442] = (p.p871 * (1.0 + (p.p874 * (s.v[366] - s.v[365]))));

        s.v[443] = (p.p872 * (1.0 + (p.p875 * (s.v[366] - s.v[365]))));

        s.v[444] = (p.p873 * (1.0 + (p.p876 * (s.v[366] - s.v[365]))));

        if (!(s.v[442] > 0.0)) {
            s.store_scalar(442, 0.0);
        }

        if (!(s.v[443] > 0.0)) {
            s.store_scalar(443, 0.0);
        }

        if (!(s.v[444] > 0.0)) {
            s.store_scalar(444, 0.0);
        }

        s.b[1007] = (s.v[474] == 1.0);
        s.v[1007] = if s.b[1007] { 1.0 } else { 0.0 };

        if s.b[1007] {
            s.store_offset(462, 461, s.v[377]);
            s.store_scale_ad(464, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(463), s.v[370], s.ad_value(462), s.v[372]), 0.5), ((s.v[367]) as f64).powf(1.5));
            s.store_sub_scaled_inputs_ad_rhs(465, 459, s.v[367], A::ln(s.ad_value(464)), (2.0 * s.v[371]));
            s.store_add_scaled_inputs_ad_rhs(466, 465, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(465), (-s.v[372]), ((0.05) * (s.v[372])))), s.v[371]);
            s.store_div_from_scalar(467, 1.0, 466);
            s.store_mul_pow_ad_rhs(470, 458, A::mul(s.ad_value(459), s.ad_value(467)), s.ad_value(460));
            s.store_mul3_lhs(471, 470, 466, 469);
            s.store_scale(472, 470, 2.0);
        }

        s.store_offset(558, 515, s.v[377]);

        s.store_offset(559, 516, s.v[377]);

        s.store_offset(560, 517, s.v[377]);

        s.store_scale_ad(561, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(555), s.v[370], s.ad_value(558), s.v[372]), 0.5), ((s.v[367]) as f64).powf(1.5));

        s.store_scale_ad(562, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(556), s.v[370], s.ad_value(559), s.v[372]), 0.5), ((s.v[367]) as f64).powf(1.5));

        s.store_scale_ad(563, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(557), s.v[370], s.ad_value(560), s.v[372]), 0.5), ((s.v[367]) as f64).powf(1.5));

        s.store_mul3_lhs(564, 518, 561, 561);

        s.store_mul3_lhs(565, 519, 562, 562);

        s.store_mul3_lhs(566, 520, 563, 563);

        s.store_sub_scaled_inputs_ad_rhs(567, 509, s.v[367], A::ln(s.ad_value(561)), (2.0 * s.v[371]));

        s.store_sub_scaled_inputs_ad_rhs(568, 510, s.v[367], A::ln(s.ad_value(562)), (2.0 * s.v[371]));

        s.store_sub_scaled_inputs_ad_rhs(569, 511, s.v[367], A::ln(s.ad_value(563)), (2.0 * s.v[371]));

        s.store_add_scaled_inputs_ad_rhs(570, 567, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(567), (-s.v[372]), ((0.05) * (s.v[372])))), s.v[371]);

        s.store_add_scaled_inputs_ad_rhs(571, 568, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(568), (-s.v[372]), ((0.05) * (s.v[372])))), s.v[371]);

        s.store_add_scaled_inputs_ad_rhs(572, 569, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(569), (-s.v[372]), ((0.05) * (s.v[372])))), s.v[371]);

        s.store_div_from_scalar(573, 1.0, 570);

        s.store_div_from_scalar(574, 1.0, 571);

        s.store_div_from_scalar(575, 1.0, 572);

        s.store_mul_pow_ad_rhs(582, 506, A::mul(s.ad_value(509), s.ad_value(573)), s.ad_value(512));

        s.store_mul_pow_ad_rhs(583, 507, A::mul(s.ad_value(510), s.ad_value(574)), s.ad_value(513));

        s.store_mul_pow_ad_rhs(584, 508, A::mul(s.ad_value(511), s.ad_value(575)), s.ad_value(514));

        s.store_mul3_lhs(585, 582, 570, 579);

        s.store_mul3_lhs(586, 583, 571, 580);

        s.store_mul3_lhs(587, 584, 572, 581);

        s.store_scale(588, 582, 2.0);

        s.store_scale(589, 583, 2.0);

        s.store_scale(590, 584, 2.0);

        s.store_max_with_scalar_ad(600, A::scale(s.ad_value(558), 0.5), s.v[371]);

        s.store_max_with_scalar_ad(601, A::scale(s.ad_value(559), 0.5), s.v[371]);

        s.store_max_with_scalar_ad(602, A::scale(s.ad_value(560), 0.5), s.v[371]);

        s.store_scale(603, 600, s.v[372]);

        s.store_scale(604, 601, s.v[372]);

        s.store_scale(605, 602, s.v[372]);

        s.store_scaled_sqrt_ad(606, A::mul3_scaled_output(s.ad_value(529), A::square(s.ad_value(600)), s.ad_value(600), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_scaled_sqrt_ad(607, A::mul3_scaled_output(s.ad_value(530), A::square(s.ad_value(601)), s.ad_value(601), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_scaled_sqrt_ad(608, A::mul3_scaled_output(s.ad_value(531), A::square(s.ad_value(602)), s.ad_value(602), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_mul_scale_offset_rhs(609, 535, 538, (s.v[366] - s.v[365]), 1.0);

        s.store_mul_scale_offset_rhs(610, 536, 539, (s.v[366] - s.v[365]), 1.0);

        s.store_mul_scale_offset_rhs(611, 537, 540, (s.v[366] - s.v[365]), 1.0);

        if (!(s.v[609] > 0.0)) {
            s.store_scalar(609, 0.0);
        }

        if (!(s.v[610] > 0.0)) {
            s.store_scalar(610, 0.0);
        }

        if (!(s.v[611] > 0.0)) {
            s.store_scalar(611, 0.0);
        }

        s.b[1008] = (s.v[636] == 1.0);
        s.v[1008] = if s.b[1008] { 1.0 } else { 0.0 };

        if s.b[1008] {
            s.store_offset(625, 624, s.v[377]);
            s.store_scale_ad(627, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(626), s.v[370], s.ad_value(625), s.v[372]), 0.5), ((s.v[367]) as f64).powf(1.5));
            s.store_sub_scaled_inputs_ad_rhs(628, 622, s.v[367], A::ln(s.ad_value(627)), (2.0 * s.v[371]));
            s.store_add_scaled_inputs_ad_rhs(629, 628, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(628), (-s.v[372]), ((0.05) * (s.v[372])))), s.v[371]);
            s.store_div_from_scalar(630, 1.0, 629);
            s.store_mul_pow_ad_rhs(633, 621, A::mul(s.ad_value(622), s.ad_value(630)), s.ad_value(623));
            s.store_mul3_lhs(634, 633, 629, 632);
            s.store_scale(635, 633, 2.0);
        }

        s.v[1] = 1.0;

        s.v[2] = 1.0;

        s.v[312] = 0.0;

        s.v[313] = 0.0;

        s.v[3] = p.p0;

        s.v[4] = p.p1;

        s.v[5] = p.p2;

        s.v[6] = p.p3;

        s.v[7] = p.p4;

        s.v[8] = p.p8;

        s.v[647] = p.p19;

        s.v[648] = p.p20;

        s.v[649] = p.p21;

        s.v[674] = p.p22;

        s.v[675] = p.p23;

        s.v[676] = p.p24;

        s.v[650] = p.p25;

        s.v[651] = p.p26;

        s.v[677] = p.p27;

        s.v[678] = p.p28;

        s.v[10] = p.p14;

        s.b[1009] = (p.p39 > 0.0);
        s.v[1009] = if s.b[1009] { 1.0 } else { 0.0 };

        if s.b[1009] {
            s.store_scalar(1, (if (p.p9 > 1.0) { p.p9 } else { 1.0 }));
        }

        if s.b[1009] {
            s.store_floor_ad(1, A::offset(s.ad_value(1), 0.5));
            s.store_div_from_scalar(2, 1.0, 1);
        }

        if ((s.v[4] * s.v[2]) > 1e-9) {
            s.store_scale(4, 2, s.v[4]);
        } else {
            s.store_scalar(4, 1e-9);
        }

        s.v[11] = p.p5;

        s.v[12] = p.p6;

        s.v[13] = p.p7;

        s.v[308] = (1e-6 / s.v[3]);

        s.store_div_from_scalar(309, 1e-6, 4);

        s.store_offset_scaled(310, 309, ((p.p191) * ((p.p189 * (1.0 + (p.p190 * s.v[308]))))), (p.p189 * (1.0 + (p.p190 * s.v[308]))));

        s.store_offset_scaled(311, 309, ((p.p195) * ((p.p193 * (1.0 + (p.p194 * s.v[308]))))), (p.p193 * (1.0 + (p.p194 * s.v[308]))));

        if (((s.v[3] + s.v[310]) - (2.0 * p.p192)) > 1e-9) {
            s.store_offset(312, 310, ((s.v[3]) + ((-(2.0 * p.p192)))));
        } else {
            s.store_scalar(312, 1e-9);
        }

        if (((s.v[4] + s.v[311]) - (2.0 * p.p196)) > 1e-9) {
            s.store_offset_add(313, 4, 311, (-(2.0 * p.p196)));
        } else {
            s.store_scalar(313, 1e-9);
        }

        s.store_div_from_scalar(314, 1e-6, 312);

        s.store_square(315, 314);

        s.store_div_from_scalar(316, 1e-6, 313);

        s.store_div_from_scalar(317, 1.0, 316);

        s.store_mul(318, 314, 316);

        s.store_div_from_scalar(319, 1.0, 318);

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((((s.v[3] + s.v[310]) - (2.0 * p.p192)) + p.p197) > 1e-9) {
            s.store_offset(320, 310, ((((s.v[3]) + ((-(2.0 * p.p192))))) + (p.p197)));
        } else {
            s.store_scalar(320, 1e-9);
        }

        if ((((s.v[4] + s.v[311]) - (2.0 * p.p196)) + p.p198) > 1e-9) {
            s.store_offset_add(321, 4, 311, (((-(2.0 * p.p196))) + (p.p198)));
        } else {
            s.store_scalar(321, 1e-9);
        }

        s.store_scale(322, 321, 1000000.0);

        if (((s.v[3] + s.v[310]) + p.p197) > 1e-9) {
            s.store_offset(323, 310, ((s.v[3]) + (p.p197)));
        } else {
            s.store_scalar(323, 1e-9);
        }

        if (((s.v[4] + s.v[311]) + p.p198) > 1e-9) {
            s.store_offset_add(324, 4, 311, p.p198);
        } else {
            s.store_scalar(324, 1e-9);
        }

        s.store_scale(325, 323, 1000000.0);

        s.store_scale(326, 324, 1000000.0);

        s.v[40] = p.p56;

        s.v[41] = p.p57;

        s.v[42] = p.p58;

        s.v[43] = p.p59;

        s.v[44] = p.p60;

        s.v[45] = p.p61;

        s.v[46] = p.p62;

        s.v[47] = p.p63;

        s.v[48] = p.p64;

        s.v[49] = p.p65;

        s.v[50] = p.p66;

        s.v[55] = p.p67;

        s.v[56] = p.p68;

        s.v[57] = p.p69;

        s.v[58] = p.p70;

        s.v[51] = p.p71;

        s.v[52] = p.p73;

        s.v[53] = p.p72;

        s.v[54] = p.p74;

        s.v[59] = p.p78;

        s.v[60] = p.p80;

        s.v[61] = p.p79;

        s.v[62] = p.p75;

        s.v[63] = p.p77;

        s.v[64] = p.p76;

        s.v[65] = p.p81;

        s.v[66] = p.p82;

        s.v[67] = p.p83;

        s.v[68] = p.p84;

        s.v[69] = p.p85;

        s.v[70] = p.p86;

        s.v[71] = p.p87;

        s.v[72] = p.p88;

        s.v[73] = p.p89;

        s.v[74] = p.p90;

        s.v[75] = p.p91;

        s.v[76] = p.p92;

        s.v[77] = p.p93;

        s.v[78] = p.p94;

        s.v[79] = p.p95;

        s.v[80] = p.p96;

        s.v[81] = p.p97;

        s.v[82] = p.p98;

        s.v[83] = p.p99;

        s.v[84] = p.p100;

        s.v[85] = p.p101;

        s.v[86] = p.p102;

        s.v[87] = p.p103;

        s.v[88] = p.p104;

        s.v[89] = p.p105;

        s.v[90] = p.p106;

        s.v[91] = p.p107;

        s.v[92] = p.p108;

        s.v[93] = p.p109;

        s.v[94] = p.p110;

        s.v[95] = p.p111;

        s.v[96] = p.p112;

        s.v[97] = p.p113;

        s.v[98] = p.p114;

        s.v[99] = p.p115;

        s.v[100] = p.p116;

        s.v[101] = p.p117;

        s.v[102] = p.p118;

        s.v[103] = p.p119;

        s.v[104] = p.p120;

        s.v[105] = p.p119;

        s.b[1010] = param_given[121];
        s.v[1010] = if s.b[1010] { 1.0 } else { 0.0 };

        if s.b[1010] {
            s.store_scalar(105, p.p121);
        }

        s.v[106] = p.p120;

        s.b[1011] = param_given[122];
        s.v[1011] = if s.b[1011] { 1.0 } else { 0.0 };

        if s.b[1011] {
            s.store_scalar(106, p.p122);
        }

        s.copy_ad(107, 105);

        s.b[1012] = param_given[123];
        s.v[1012] = if s.b[1012] { 1.0 } else { 0.0 };

        if s.b[1012] {
            s.store_scalar(107, p.p123);
        }

        s.copy_ad(108, 106);

        s.b[1013] = param_given[124];
        s.v[1013] = if s.b[1013] { 1.0 } else { 0.0 };

        if s.b[1013] {
            s.store_scalar(108, p.p124);
        }

        s.v[109] = p.p125;

        s.v[110] = p.p126;

        s.v[111] = p.p127;

        s.v[112] = p.p128;

        s.v[113] = p.p129;

        s.v[114] = p.p130;

        s.v[115] = p.p131;

        s.v[116] = p.p132;

        s.v[117] = p.p133;

        s.v[118] = p.p134;

        s.v[119] = p.p135;

        s.v[120] = p.p136;

        s.v[121] = p.p98;

        s.b[1014] = param_given[137];
        s.v[1014] = if s.b[1014] { 1.0 } else { 0.0 };

        if s.b[1014] {
            s.store_scalar(121, p.p137);
        }

        s.v[122] = p.p103;

        s.b[1015] = param_given[138];
        s.v[1015] = if s.b[1015] { 1.0 } else { 0.0 };

        if s.b[1015] {
            s.store_scalar(122, p.p138);
        }

        s.v[123] = p.p139;

        s.v[124] = p.p140;

        s.v[125] = p.p141;

        s.v[126] = p.p142;

        s.v[127] = p.p143;

        s.v[128] = p.p144;

        s.v[129] = p.p145;

        s.v[130] = p.p146;

        s.v[131] = p.p147;

        s.v[132] = p.p148;

        s.v[133] = p.p149;

        s.v[134] = p.p150;

        s.v[135] = p.p151;

        s.v[136] = p.p152;

        s.v[137] = p.p153;

        s.v[138] = p.p154;

        s.v[139] = p.p155;

        s.v[145] = p.p161;

        s.v[146] = p.p162;

        s.v[147] = p.p163;

        s.v[148] = p.p164;

        s.v[149] = p.p165;

        s.v[150] = p.p166;

        s.v[151] = p.p167;

        s.v[152] = p.p168;

        s.v[153] = p.p169;

        s.v[154] = p.p170;

        s.v[155] = p.p171;

        s.v[156] = p.p173;

        s.v[157] = p.p172;

        s.v[173] = p.p187;

        s.b[1016] = (p.p39 > 0.0);
        s.v[1016] = if s.b[1016] { 1.0 } else { 0.0 };

        if s.b[1016] {
            s.store_add_scaled_inputs3_offset_mixed_aii(40, A::powf(s.ad_value(314), p.p201), p.p200, 316, p.p202, 318, p.p203, p.p199);
            s.store_add_scaled_inputs3_offset_indices(41, 314, p.p205, 316, p.p206, 318, p.p207, p.p204);
            s.store_scalar(42, p.p208);
            s.store_scalar(43, p.p209);
            s.store_scalar(44, p.p210);
        }

        if s.b[1016] {
            s.store_scale_ad(331, {
                if ((1.0 + ((p.p212 * s.v[316]) * (((1.0 + (s.v[313] / p.p213))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p.p212, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p213), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p211);
        }

        if s.b[1016] {
            s.store_scale_ad(332, {
                if ((1.0 + ((p.p215 * s.v[316]) * (((1.0 + (s.v[313] / p.p216))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p.p215, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p216), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p214);
        }

        if s.b[1016] {
            s.store_scale_ad(333, {
                if ((1.0 + ((p.p218 * s.v[316]) * (((1.0 + (s.v[313] / p.p216))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(316), p.p218, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p216), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p217);
        }

        s.b[1017] = (s.v[312] > (2.0 * s.v[333]));
        s.v[1017] = if s.b[1017] { 1.0 } else { 0.0 };

        if (s.b[1016] && s.b[1017]) {
            s.store_scalar(334, 75000000000.0);
            s.store_sub_ad(335, A::sqrt(A::add_scaled_inputs(s.ad_value(331), 1.0, s.ad_value(332), 0.5)), A::sqrt(s.ad_value(331)));
            s.store_add_scaled_product_mixed_aia(336, A::sqrt(s.ad_value(331)), 1.0, 334, A::ln(A::offset(A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(333), 2.0, s.ad_value(312), 1.0), A::exp(A::div(s.ad_value(335), s.ad_value(334))), (-1.0)), 1.0)), 1.0);
            s.store_square(336, 336);
        }

        s.b[1018] = (s.v[312] >= s.v[333]);
        s.v[1018] = if s.b[1018] { 1.0 } else { 0.0 };

        if ((s.b[1016] && (!s.b[1017])) && s.b[1018]) {
            s.store_add_ad_rhs(336, 331, A::div_scaled_product(s.ad_value(332), s.ad_value(333), 1.0, s.ad_value(312), 1.0));
        }

        if ((s.b[1016] && (!s.b[1017])) && (!s.b[1018])) {
            s.store_add_ad_rhs(336, 331, A::mul_sub_from_scalar_rhs(s.ad_value(332), 2.0, A::div(s.ad_value(312), s.ad_value(333))));
        }

        if s.b[1016] {
            s.store_mul_sub_scaled_inputs_rhs(45, 336, A::sub_from_scalar(1.0, A::scale(s.ad_value(314), p.p219)), 1.0, s.ad_value(315), p.p220);
            s.store_add_scaled_inputs3_offset_mixed_aii(46, A::powf(s.ad_value(314), p.p223), p.p222, 316, p.p224, 318, p.p225, p.p221);
            s.store_scalar(47, p.p226);
            s.store_scalar(48, p.p227);
            s.store_add_scaled_inputs3_offset_mixed_aii(49, A::powf(s.ad_value(314), p.p230), p.p229, 316, p.p231, 318, p.p232, p.p228);
        }

        if s.b[1016] {
            s.store_scale_ad(50, {
                if (1e-6 > (1.0 + (p.p234 * s.v[314]))) {
                    A::constant(1e-6)
                } else {
                    A::scale_offset(s.ad_value(314), p.p234, 1.0)
                }
            }, p.p233);
        }

        if s.b[1016] {
            s.store_scalar(55, p.p235);
            s.store_scalar(56, p.p236);
            s.store_scalar(57, p.p239);
            s.store_scalar(58, p.p240);
            s.store_mul3_ad(51, A::scale_offset(A::powf(s.ad_value(314), p.p243), p.p242, p.p241), A::scale_offset(s.ad_value(316), p.p244, 1.0), A::scale_offset(s.ad_value(318), p.p245, 1.0));
            s.store_scalar(52, p.p247);
            s.store_scalar(53, p.p246);
            s.store_scalar(54, p.p248);
            s.store_scaled_mul_scale_offset_rhs_ad(62, A::powf(s.ad_value(314), p.p250), 316, p.p251, 1.0, p.p249);
            s.store_scalar(63, p.p253);
            s.store_scalar(64, p.p252);
            s.store_scaled_mul_scale_offset_rhs_ad(59, A::powf(s.ad_value(314), p.p255), 316, p.p256, 1.0, p.p254);
            s.store_scalar(60, p.p258);
            s.store_scalar(61, p.p257);
            s.store_offset_scaled(337, 316, ((p.p261) * (p.p260)), p.p260);
        }

        if s.b[1016] {
            s.store_scale_ad(338, {
                if ((1.0 + (p.p263 * s.v[316])) > 0.001) {
                    A::scale_offset(s.ad_value(316), p.p263, 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p262);
        }

        if s.b[1016] {
            s.store_add_ad(339, A::offset(A::mul_sub_from_scalar_rhs(A::div_scaled_product(s.ad_value(337), s.ad_value(338), 1.0, s.ad_value(312), 1.0), 1.0, A::exp(A::div_scaled_inputs(s.ad_value(312), -1.0, s.ad_value(338), 1.0))), 1.0), A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p264 * p.p265), s.ad_value(312)), 1.0, A::exp_scaled_input(s.ad_value(312), (-1.0 / (p.p265)))));
        }

        if s.b[1016] {
            if (s.v[339] > 1e-15) {
            } else {
                s.store_scalar(339, 1e-15);
            }
        }

        if s.b[1016] {
            s.store_add_scaled_product_mixed_aia(340, A::scale_offset(s.ad_value(316), p.p266, 1.0), 1.0, 316, A::ln(A::scale_offset(s.ad_value(313), 1.0 / (p.p268), 1.0)), p.p267);
            s.store_mul_div_scaled_inputs_rhs(65, 340, s.ad_value(313), p.p259, A::mul(s.ad_value(339), s.ad_value(312)), 1.0);
            s.store_add_scaled_inputs3_offset_indices(66, 314, p.p270, 316, p.p271, 318, p.p272, p.p269);
        }

    }
}

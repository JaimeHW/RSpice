#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_32(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2211] = (s.v[1919] > 0.0);
        s.v[2211] = if s.b[2211] { 1.0 } else { 0.0 };

        if ((s.b[2204] && s.b[2209]) && s.b[2211]) {
            s.store_offset_ad(1937, A::mul_offset_rhs(s.ad_value(1919), A::mul_scaled_output(s.ad_value(1919), A::scale_offset(s.ad_value(1919), 0.3333333333333333, 1.0), 0.5), 1.0), 1.0);
        }

        s.b[2212] = (s.v[1919] > (-230.25850929940458));
        s.v[2212] = if s.b[2212] { 1.0 } else { 0.0 };

        if (((s.b[2204] && s.b[2209]) && (!s.b[2211])) && s.b[2212]) {
            s.store_exp(1937, 1919);
        }

        if (((s.b[2204] && s.b[2209]) && (!s.b[2211])) && (!s.b[2212])) {
            s.store_div_from_scalar_offset_ad(1937, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1919), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1919), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (s.b[2204] && s.b[2209]) {
            s.store_offset(1938, 1935, 3.0);
            s.store_sub_from_scalar(1939, (-3.0), 230);
            s.store_scale(1940, 831, 30.0);
            s.store_scalar(812, (4.0 - 0.9));
            s.store_add(813, 1938, 1940);
            s.store_mul_ad(1919, A::div_from_scalar(2.0, s.ad_value(812)), A::sub(s.ad_value(813), A::sqrt(A::sub(A::square(s.ad_value(813)), A::mul3(s.ad_value(812), s.ad_value(1938), s.ad_value(1940))))));
            s.store_scalar(812, (4.0 - 0.3));
            s.store_add(813, 1939, 1919);
            s.store_mul_ad(1941, A::div_from_scalar(2.0, s.ad_value(812)), A::add(s.ad_value(813), A::sqrt(A::sub(A::square(s.ad_value(813)), A::mul3(s.ad_value(812), s.ad_value(1939), s.ad_value(1919))))));
            s.store_mul3_lhs(834, 233, 1937, 1941);
        }

        s.b[2213] = (s.v[231] > 0.0);
        s.v[2213] = if s.b[2213] { 1.0 } else { 0.0 };

        s.b[2214] = (s.v[1817] <= 0.0);
        s.v[2214] = if s.b[2214] { 1.0 } else { 0.0 };

        if ((s.b[2204] && s.b[2213]) && s.b[2214]) {
            s.store_offset(1919, 771, 1.0);
            s.store_ad_value(1920, A::div_scaled_product(A::sqrt(s.ad_value(1919)), s.ad_value(820), 1.0, s.ad_value(1843), 1.0));
            s.store_add_ad_lhs(1921, A::square(s.ad_value(1920)), 1919);
            s.store_scale(1919, 1920, 2.0);
            s.store_ad_value(1846, A::div_scaled_product3(s.ad_value(1843), s.ad_value(1813), s.ad_value(1919), 1.0, A::add(A::sqrt(A::sub(s.ad_value(1921), s.ad_value(1919))), A::sqrt(A::add(s.ad_value(1921), s.ad_value(1919)))), 1.0));
        }

        s.b[2215] = ((s.v[1847] - s.v[1846]) > (-230.25850929940458));
        s.v[2215] = if s.b[2215] { 1.0 } else { 0.0 };

        if ((s.b[2204] && s.b[2213]) && s.b[2215]) {
            s.store_exp_sub(1919, 1847, 1846);
        }

        if ((s.b[2204] && s.b[2213]) && (!s.b[2215])) {
            s.store_div_from_scalar_offset_ad(1919, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1847), s.ad_value(1846)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1847), s.ad_value(1846)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (s.b[2204] && s.b[2213]) {
            s.store_ad_value(1942, A::add_scaled_product(s.ad_value(1922), 1.0, s.ad_value(1812), A::sub_scaled_inputs(s.ad_value(1847), 0.5, A::ln_scaled_input(A::offset(s.ad_value(1919), 1.0), 0.5), 1.0), 1.0));
            s.store_mul(1943, 230, 1812);
            s.store_add(1944, 1860, 1943);
            s.store_scaled_sub_ad_rhs(1945, 1944, A::sqrt(A::offset(A::mul_scaled_output(s.ad_value(1944), s.ad_value(1944), 1.0), 0.01)), 0.5);
            s.store_mul_sqrt_ad_lhs(1936, A::offset(A::square(s.ad_value(1860)), 1e-6), 789);
        }

        s.b[2216] = (s.v[236] < 0.0);
        s.v[2216] = if s.b[2216] { 1.0 } else { 0.0 };

        if ((s.b[2204] && s.b[2213]) && s.b[2216]) {
            s.store_ad_value(1936, A::add_scaled_inputs3(s.ad_value(1936), 0.5, s.ad_value(794), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1936), s.ad_value(794)), A::sub(s.ad_value(1936), s.ad_value(794))), 1e-6)), (-0.5)));
        }

        if (s.b[2204] && s.b[2213]) {
            s.store_ad_value(1946, A::add_scaled_product(s.ad_value(1850), 1.0, A::add_scaled_inputs3(s.ad_value(1945), 1.0, s.ad_value(736), (-1.0), s.ad_value(1942), -1.0), s.ad_value(1813), 1.0));
        }

        s.b[2217] = (((s.v[1946]) as f64).abs() < 230.25850929940458);
        s.v[2217] = if s.b[2217] { 1.0 } else { 0.0 };

        if ((s.b[2204] && s.b[2213]) && s.b[2217]) {
            s.store_exp(1947, 1946);
        }

        s.b[2218] = (s.v[1946] < 0.0);
        s.v[2218] = if s.b[2218] { 1.0 } else { 0.0 };

        if (((s.b[2204] && s.b[2213]) && (!s.b[2217])) && s.b[2218]) {
            s.store_div_from_scalar_offset_ad(1947, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1946), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1946), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[2204] && s.b[2213]) && (!s.b[2217])) && (!s.b[2218])) {
            s.store_scaled_offset_ad(1947, A::mul_offset_lhs(s.ad_value(1946), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(1946), (-230.25850929940458), A::scale_offset(s.ad_value(1946), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (s.b[2204] && s.b[2213]) {
            s.store_mul_neg_ad_lhs(1946, A::add_scaled_inputs3(s.ad_value(819), 1.0, s.ad_value(1922), 1.0, s.ad_value(1942), -1.0), 1813);
        }

        s.b[2219] = (((s.v[1946]) as f64).abs() < 230.25850929940458);
        s.v[2219] = if s.b[2219] { 1.0 } else { 0.0 };

        if ((s.b[2204] && s.b[2213]) && s.b[2219]) {
            s.store_exp(1919, 1946);
        }

        s.b[2220] = (s.v[1946] < 0.0);
        s.v[2220] = if s.b[2220] { 1.0 } else { 0.0 };

        if (((s.b[2204] && s.b[2213]) && (!s.b[2219])) && s.b[2220]) {
            s.store_div_from_scalar_offset_ad(1919, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1946), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1946), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[2204] && s.b[2213]) && (!s.b[2219])) && (!s.b[2220])) {
            s.store_scaled_offset_ad(1919, A::mul_offset_lhs(s.ad_value(1946), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(1946), (-230.25850929940458), A::scale_offset(s.ad_value(1946), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (s.b[2204] && s.b[2213]) {
            s.store_mul(1948, 1947, 1919);
            s.store_mul_offset_ad_rhs(1919, 791, A::mul(s.ad_value(1936), A::add_scaled_product(s.ad_value(235), 1.0, s.ad_value(236), s.ad_value(1936), 1.0)), (-1.5));
        }

        s.b[2221] = (s.v[1919] > 0.0);
        s.v[2221] = if s.b[2221] { 1.0 } else { 0.0 };

        if ((s.b[2204] && s.b[2213]) && s.b[2221]) {
            s.store_offset_ad(1937, A::mul_offset_rhs(s.ad_value(1919), A::mul_scaled_output(s.ad_value(1919), A::scale_offset(s.ad_value(1919), 0.3333333333333333, 1.0), 0.5), 1.0), 1.0);
        }

        s.b[2222] = (s.v[1919] > (-230.25850929940458));
        s.v[2222] = if s.b[2222] { 1.0 } else { 0.0 };

        if (((s.b[2204] && s.b[2213]) && (!s.b[2221])) && s.b[2222]) {
            s.store_exp(1937, 1919);
        }

        if (((s.b[2204] && s.b[2213]) && (!s.b[2221])) && (!s.b[2222])) {
            s.store_div_from_scalar_offset_ad(1937, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1919), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1919), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (s.b[2204] && s.b[2213]) {
            s.store_mul_ad_product_rhs(1949, 231, s.ad_value(1937), A::ln(A::div(A::offset(s.ad_value(1947), 1.0), A::offset(s.ad_value(1948), 1.0))));
        }

        s.b[2223] = ((s.v[1817] <= 0.0) || ((s.v[235] == 0.0) && (s.v[236] == 0.0)));
        s.v[2223] = if s.b[2223] { 1.0 } else { 0.0 };

        if ((s.b[2204] && s.b[2213]) && s.b[2223]) {
            s.store_scalar(1956, 1.0);
            s.store_scalar(1957, 0.5);
        }

        if ((s.b[2204] && s.b[2213]) && (!s.b[2223])) {
            s.store_ad_value(1919, A::add_scaled_product(s.ad_value(235), 1.0, s.ad_value(236), s.ad_value(1936), 2.0));
            s.store_div_ad_rhs(1950, 241, A::mul(s.ad_value(1919), s.ad_value(791)));
            s.store_scaled_div(1951, 1848, 1950, 0.5);
            s.store_div(1952, 1950, 1865);
            s.store_ad_value(1953, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1952), 1.0, s.ad_value(1952), 0.5));
            s.store_sub_from_scalar_ad(1954, 0.5, A::scale(s.ad_value(1953), 3.0));
        }

        s.b[2224] = (s.v[1951] < 0.001);
        s.v[2224] = if s.b[2224] { 1.0 } else { 0.0 };

        if (((s.b[2204] && s.b[2213]) && (!s.b[2223])) && s.b[2224]) {
            s.store_square(1955, 1951);
            s.store_offset_mul_ad(1956, s.ad_value(1955), A::add_scaled_product(A::scale_offset(s.ad_value(1952), 0.3333333333333333, 0.16666666666666666), 1.0, s.ad_value(1955), A::scale_offset(s.ad_value(1952), 0.2, 0.05), 0.16666666666666666), 1.0);
            s.store_ad_value(1957, A::add_scaled_offset_product_rhs(s.ad_value(1956), 0.5, s.ad_value(1951), A::mul(s.ad_value(1955), A::add_scaled_offset_product_rhs(A::scaled_offset(s.ad_value(1953), 0.25, 0.4), 1.0, s.ad_value(1955), s.ad_value(1953), 0.125, 0.0285714285714)), 1.0, (-0.16666666666666666)));
        }

        if (((s.b[2204] && s.b[2213]) && (!s.b[2223])) && (!s.b[2224])) {
            s.store_div_from_scalar(1958, 1.0, 1951);
        }

        s.b[2225] = (((s.v[1951]) as f64).abs() < 230.25850929940458);
        s.v[2225] = if s.b[2225] { 1.0 } else { 0.0 };

        if ((((s.b[2204] && s.b[2213]) && (!s.b[2223])) && (!s.b[2224])) && s.b[2225]) {
            s.store_exp(1959, 1951);
        }

        s.b[2226] = (s.v[1951] < 0.0);
        s.v[2226] = if s.b[2226] { 1.0 } else { 0.0 };

        if (((((s.b[2204] && s.b[2213]) && (!s.b[2223])) && (!s.b[2224])) && (!s.b[2225])) && s.b[2226]) {
            s.store_div_from_scalar_offset_ad(1959, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1951), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1951), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2204] && s.b[2213]) && (!s.b[2223])) && (!s.b[2224])) && (!s.b[2225])) && (!s.b[2226])) {
            s.store_scaled_offset_ad(1959, A::mul_offset_lhs(s.ad_value(1951), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(1951), (-230.25850929940458), A::scale_offset(s.ad_value(1951), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((s.b[2204] && s.b[2213]) && (!s.b[2223])) && (!s.b[2224])) {
            s.store_div_from_scalar(1960, 1.0, 1959);
            s.store_sub(1919, 1959, 1960);
            s.store_add(1921, 1959, 1960);
            s.store_ad_value(1956, A::add_scaled_products(A::mul_sub_from_scalar_lhs(1.0, s.ad_value(1952), s.ad_value(1919)), s.ad_value(1958), 0.5, s.ad_value(1952), s.ad_value(1921), 0.5));
            s.store_scaled_sub_ad(1957, A::add_scaled_product(s.ad_value(1956), 1.0, s.ad_value(1919), A::sub(s.ad_value(1953), A::mul3(s.ad_value(1954), s.ad_value(1958), s.ad_value(1958))), (-1.0)), A::mul3(s.ad_value(1954), s.ad_value(1921), s.ad_value(1958)), 0.5);
        }

        if (s.b[2204] && s.b[2213]) {
            s.store_scaled_offset_ad(1961, A::div(s.ad_value(1817), A::sqrt(A::offset(A::square(s.ad_value(1817)), 1e-6))), 1.0, 0.5);
            s.store_mul3_lhs(1962, 1949, 1956, 1961);
            s.store_mul3_lhs(836, 1949, 1957, 1961);
            s.store_sub(835, 1962, 836);
            s.store_mul_ad_product_rhs(837, 1949, s.ad_value(1956), A::sub_from_scalar(1.0, s.ad_value(1961)));
        }

        s.v[839] = 0.0;

        s.v[838] = 0.0;

        s.b[2227] = (p.p42 != 0.0);
        s.v[2227] = if s.b[2227] { 1.0 } else { 0.0 };

        s.b[2228] = ((s.v[243] > 0.0) && (s.v[1867] < 0.0));
        s.v[2228] = if s.b[2228] { 1.0 } else { 0.0 };

        if (s.b[2227] && s.b[2228]) {
            s.store_sqrt_offset_ad(1963, A::add_scaled_square_product(s.ad_value(1867), 1.0, A::square(s.ad_value(249)), A::square(s.ad_value(830)), 1.0), 1e-6);
            s.store_scaled_div(1919, 801, 1963, -1.0);
        }

        s.b[2229] = (s.v[1919] > (-230.25850929940458));
        s.v[2229] = if s.b[2229] { 1.0 } else { 0.0 };

        if ((s.b[2227] && s.b[2228]) && s.b[2229]) {
            s.store_exp(1921, 1919);
        }

        if ((s.b[2227] && s.b[2228]) && (!s.b[2229])) {
            s.store_div_from_scalar_offset_ad(1921, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1919), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1919), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (s.b[2227] && s.b[2228]) {
            s.store_mul_ad_affine_product_lhs(839, s.ad_value(799), A::mul3(s.ad_value(830), s.ad_value(1867), s.ad_value(1963)), -1.0, 0.0, 1921);
        }

        s.b[2230] = ((s.v[242] > 0.0) && (s.v[1866] < 0.0));
        s.v[2230] = if s.b[2230] { 1.0 } else { 0.0 };

        if (s.b[2227] && s.b[2230]) {
            s.store_sqrt_offset_ad(1964, A::add_scaled_square_product(s.ad_value(1866), 1.0, A::square(s.ad_value(248)), A::square(s.ad_value(829)), 1.0), 1e-6);
            s.store_scaled_div(1919, 800, 1964, -1.0);
        }

        s.b[2231] = (s.v[1919] > (-230.25850929940458));
        s.v[2231] = if s.b[2231] { 1.0 } else { 0.0 };

        if ((s.b[2227] && s.b[2230]) && s.b[2231]) {
            s.store_exp(1921, 1919);
        }

        if ((s.b[2227] && s.b[2230]) && (!s.b[2231])) {
            s.store_div_from_scalar_offset_ad(1921, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1919), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1919), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (s.b[2227] && s.b[2230]) {
            s.store_mul_ad_affine_product_lhs(838, s.ad_value(798), A::mul3(s.ad_value(829), s.ad_value(1866), s.ad_value(1964)), -1.0, 0.0, 1921);
        }

        s.v[1968] = s.v[709];

        s.v[1868] = 0.0;

        s.v[1869] = 0.0;

        s.v[1870] = 0.0;

        s.v[1871] = 1e-40;

        s.v[1872] = 1.0;

        s.v[840] = 0.0;

        s.b[2232] = ((p.p46 != 0.0) && (s.v[282] > 0.0));
        s.v[2232] = if s.b[2232] { 1.0 } else { 0.0 };

        if s.b[2232] {
            s.store_add_ad_lhs(1919, A::add_scaled_inputs3(s.ad_value(822), 0.5, s.ad_value(821), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(758), 1.0, A::sub(s.ad_value(822), s.ad_value(821)), A::sub(s.ad_value(822), s.ad_value(821)), 1.0)), (-0.5)), 756);
            s.store_add_ad_lhs(1965, A::add_scaled_inputs3(s.ad_value(821), 1.0, s.ad_value(1919), (-0.5), A::sqrt(A::add_scaled_product(s.ad_value(757), 1.0, s.ad_value(1919), s.ad_value(1919), 1.0)), (-(-0.5))), 760);
            s.store_ad_value(1966, A::add_scaled_inputs3(s.ad_value(1965), 1.0, s.ad_value(820), 0.5, s.ad_value(824), (-0.5)));
            s.store_mul_ad_product_rhs(1967, 284, A::offset(A::mul(s.ad_value(286), s.ad_value(824)), 1.0), A::offset(A::mul(s.ad_value(285), s.ad_value(1966)), 1.0));
            s.store_mul_offset_rhs(1968, 717, 1967, 1.0);
            s.store_div_from_scalar(1969, 1.0, 1968);
            s.store_ad_value(1970, A::div_scaled_inputs(s.ad_value(824), 2.0, A::offset(A::sqrt(A::offset(A::mul(s.ad_value(288), s.ad_value(824)), 1.0)), 1.0), 1.0));
            s.store_mul_ad_product_rhs(1971, 287, s.ad_value(1970), A::offset(A::mul(s.ad_value(289), s.ad_value(1966)), 1.0));
            s.store_mul_ad_rhs(1868, 1969, A::add_scaled_inputs3(s.ad_value(823), 1.0, s.ad_value(1971), 1.0, s.ad_value(707), -1.0));
            s.store_mul(1972, 1969, 754);
            s.store_scaled_ln_ad(1973, A::add(A::div(s.ad_value(1972), s.ad_value(755)), A::sqrt(s.ad_value(1972))), 2.0);
            s.store_mul(1974, 1969, 1965);
            s.store_add(1979, 1972, 1974);
            s.store_ad_value(1980, A::add_scaled_product(s.ad_value(1979), 1.0, s.ad_value(755), A::sqrt(s.ad_value(1979)), 1.0));
            s.store_add(1981, 1980, 1973);
            s.store_offset_ad(1982, A::div_scaled_inputs(s.ad_value(755), 1.0, A::sqrt(s.ad_value(1979)), 2.0), 1.0);
            s.store_div_from_scalar(1983, 1.0, 1982);
            s.store_sub(1984, 1868, 1981);
        }

        s.b[2233] = (s.v[1984] > (-12.0));
        s.v[2233] = if s.b[2233] { 1.0 } else { 0.0 };

        if (s.b[2232] && s.b[2233]) {
            s.store_offset_add(1985, 1984, 719, (-1.0));
            s.store_scaled_add_ad_rhs(1986, 1985, A::sqrt(A::offset(A::square(s.ad_value(1985)), 10.0)), 0.5);
            s.store_add_ad_lhs(1987, A::add_scaled_product(s.ad_value(1984), 1.0, s.ad_value(1982), A::ln(s.ad_value(1986)), (-1.0)), 719);
            s.store_scaled_add_ad_rhs(1988, 1987, A::sqrt(A::offset(A::square(s.ad_value(1987)), 2.0)), 0.5);
        }

        s.b[2234] = ((s.v[1984] - s.v[1988]) < 230.25850929940458);
        s.v[2234] = if s.b[2234] { 1.0 } else { 0.0 };

        if ((s.b[2232] && s.b[2233]) && s.b[2234]) {
            s.store_exp_sub(1989, 1984, 1988);
        }

        if ((s.b[2232] && s.b[2233]) && (!s.b[2234])) {
            s.store_scaled_offset_ad(1989, A::mul_offset_lhs(A::sub(s.ad_value(1984), s.ad_value(1988)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(1984), s.ad_value(1988)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(1984), s.ad_value(1988)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (s.b[2232] && s.b[2233]) {
            s.store_mul(1990, 718, 1989);
            s.store_pow_ad(1991, s.ad_value(1990), s.ad_value(1983));
            s.store_ad_value(1992, A::add_scaled_square_product(s.ad_value(1982), 1.0, A::add_scaled_inputs3(s.ad_value(1988), 2.0, s.ad_value(1982), 2.0, s.ad_value(1991), -1.0), s.ad_value(1991), 1.0));
            s.store_mul_offset_ad_rhs(1993, 1982, A::div(A::sub(A::sqrt(s.ad_value(1992)), s.ad_value(1982)), s.ad_value(1991)), (-1.0));
            s.store_sub(1975, 1988, 1993);
        }

        s.b[2235] = ((s.v[1983] * (s.v[1984] + s.v[719])) > (-230.25850929940458));
        s.v[2235] = if s.b[2235] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_33(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[2232] && (!s.b[2233])) && s.b[2235]) {
            s.store_exp_ad(1975, A::mul(s.ad_value(1983), A::add(s.ad_value(1984), s.ad_value(719))));
        }

        if ((s.b[2232] && (!s.b[2233])) && (!s.b[2235])) {
            s.store_div_from_scalar_offset_ad(1975, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::mul(s.ad_value(1983), A::add(s.ad_value(1984), s.ad_value(719))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::mul(s.ad_value(1983), A::add(s.ad_value(1984), s.ad_value(719))), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if s.b[2232] {
            s.store_mul_add_rhs(1976, 1969, 1845, 1965);
        }

        s.b[2236] = ((s.v[1975] < 0.001) && (s.v[1845] < 1e-6));
        s.v[2236] = if s.b[2236] { 1.0 } else { 0.0 };

        s.b[2237] = (((-s.v[1976]) + s.v[1974]) > (-230.25850929940458));
        s.v[2237] = if s.b[2237] { 1.0 } else { 0.0 };

        if ((s.b[2232] && s.b[2236]) && s.b[2237]) {
            s.store_exp_sub(1919, 1974, 1976);
        }

        if ((s.b[2232] && s.b[2236]) && (!s.b[2237])) {
            s.store_div_from_scalar_offset_ad(1919, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(1974), s.ad_value(1976)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(1974), s.ad_value(1976)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (s.b[2232] && s.b[2236]) {
            s.store_mul_offset_rhs(1869, 1975, 1919, (-1.0));
            s.store_add(1977, 1869, 1975);
        }

        if (s.b[2232] && (!s.b[2236])) {
            s.store_add(1979, 1972, 1976);
            s.store_ad_value(1980, A::add_scaled_product(s.ad_value(1979), 1.0, s.ad_value(755), A::sqrt(s.ad_value(1979)), 1.0));
            s.store_add(1981, 1980, 1973);
            s.store_offset_ad(1982, A::div_scaled_inputs(s.ad_value(755), 1.0, A::sqrt(s.ad_value(1979)), 2.0), 1.0);
            s.store_div_from_scalar(1983, 1.0, 1982);
            s.store_sub(1984, 1868, 1981);
        }

        s.b[2238] = (s.v[1984] > (-12.0));
        s.v[2238] = if s.b[2238] { 1.0 } else { 0.0 };

        if ((s.b[2232] && (!s.b[2236])) && s.b[2238]) {
            s.store_offset_add(1985, 1984, 719, (-1.0));
            s.store_scaled_add_ad_rhs(1986, 1985, A::sqrt(A::offset(A::square(s.ad_value(1985)), 10.0)), 0.5);
            s.store_add_ad_lhs(1987, A::add_scaled_product(s.ad_value(1984), 1.0, s.ad_value(1982), A::ln(s.ad_value(1986)), (-1.0)), 719);
            s.store_scaled_add_ad_rhs(1988, 1987, A::sqrt(A::offset(A::square(s.ad_value(1987)), 2.0)), 0.5);
        }

        s.b[2239] = ((s.v[1984] - s.v[1988]) < 230.25850929940458);
        s.v[2239] = if s.b[2239] { 1.0 } else { 0.0 };

        if (((s.b[2232] && (!s.b[2236])) && s.b[2238]) && s.b[2239]) {
            s.store_exp_sub(1989, 1984, 1988);
        }

        if (((s.b[2232] && (!s.b[2236])) && s.b[2238]) && (!s.b[2239])) {
            s.store_scaled_offset_ad(1989, A::mul_offset_lhs(A::sub(s.ad_value(1984), s.ad_value(1988)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(1984), s.ad_value(1988)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(1984), s.ad_value(1988)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2232] && (!s.b[2236])) && s.b[2238]) {
            s.store_mul(1990, 718, 1989);
            s.store_pow_ad(1991, s.ad_value(1990), s.ad_value(1983));
            s.store_ad_value(1992, A::add_scaled_square_product(s.ad_value(1982), 1.0, A::add_scaled_inputs3(s.ad_value(1988), 2.0, s.ad_value(1982), 2.0, s.ad_value(1991), -1.0), s.ad_value(1991), 1.0));
            s.store_mul_offset_ad_rhs(1993, 1982, A::div(A::sub(A::sqrt(s.ad_value(1992)), s.ad_value(1982)), s.ad_value(1991)), (-1.0));
            s.store_sub(1977, 1988, 1993);
        }

        s.b[2240] = ((s.v[1983] * (s.v[1984] + s.v[719])) > (-230.25850929940458));
        s.v[2240] = if s.b[2240] { 1.0 } else { 0.0 };

        if (((s.b[2232] && (!s.b[2236])) && (!s.b[2238])) && s.b[2240]) {
            s.store_exp_ad(1977, A::mul(s.ad_value(1983), A::add(s.ad_value(1984), s.ad_value(719))));
        }

        if (((s.b[2232] && (!s.b[2236])) && (!s.b[2238])) && (!s.b[2240])) {
            s.store_div_from_scalar_offset_ad(1977, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::mul(s.ad_value(1983), A::add(s.ad_value(1984), s.ad_value(719))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::mul(s.ad_value(1983), A::add(s.ad_value(1984), s.ad_value(719))), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (s.b[2232] && (!s.b[2236])) {
            s.store_sub(1869, 1977, 1975);
        }

        if s.b[2232] {
            s.store_scaled_add(1870, 1977, 1975, 0.5);
        }

        if s.b[2232] {
            s.store_ad_value(1871, {
                if ((s.v[1868] - s.v[1870]) > 1e-40) {
                    A::sub(s.ad_value(1868), s.ad_value(1870))
                } else {
                    A::constant(1e-40)
                }
            });
        }

        if s.b[2232] {
            s.store_sub_from_scalar_ad(1872, 1.0, A::div_scaled_inputs(s.ad_value(755), 0.5, A::sqrt(A::add_scaled_inputs(s.ad_value(1871), 1.0, s.ad_value(718), 0.25)), 1.0));
            s.store_ad_value(840, A::div_scaled_product3(A::mul3_scaled_output(s.ad_value(711), s.ad_value(1968), s.ad_value(1968), -1.0), A::offset(A::mul(s.ad_value(1872), s.ad_value(1870)), 1.0), s.ad_value(1869), 1.0, s.ad_value(1857), 1.0));
        }

        s.v[1873] = 0.0;

        s.v[841] = 0.0;

        s.b[2241] = ((s.v[1817] > 0.0) && (p.p41 != 0.0));
        s.v[2241] = if s.b[2241] { 1.0 } else { 0.0 };

        if s.b[2241] {
            s.store_ad_value(1978, A::add_scaled_product(s.ad_value(820), 1.0, s.ad_value(227), s.ad_value(1848), (-1.0)));
        }

        s.b[2242] = (s.v[1978] > 0.0);
        s.v[2242] = if s.b[2242] { 1.0 } else { 0.0 };

        if (s.b[2241] && s.b[2242]) {
            s.store_mul_div_ad_rhs(1921, 706, A::offset(A::mul(s.ad_value(228), A::sub(A::sqrt(A::add(s.ad_value(722), s.ad_value(1922))), s.ad_value(730))), 1.0), A::offset(s.ad_value(1978), 1e-30));
        }

        s.b[2243] = ((((-s.v[1921])) as f64).abs() < 230.25850929940458);
        s.v[2243] = if s.b[2243] { 1.0 } else { 0.0 };

        if ((s.b[2241] && s.b[2242]) && s.b[2243]) {
            s.store_exp_neg_input(1919, 1921);
        }

        s.b[2244] = ((-s.v[1921]) < 0.0);
        s.v[2244] = if s.b[2244] { 1.0 } else { 0.0 };

        if (((s.b[2241] && s.b[2242]) && (!s.b[2243])) && s.b[2244]) {
            s.store_div_from_scalar_offset_ad(1919, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(1921)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(1921)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[2241] && s.b[2242]) && (!s.b[2243])) && (!s.b[2244])) {
            s.store_scaled_offset_ad(1919, A::mul_offset_lhs(A::neg(s.ad_value(1921)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::neg(s.ad_value(1921)), (-230.25850929940458), A::scale_offset(A::neg(s.ad_value(1921)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (s.b[2241] && s.b[2242]) {
            s.store_mul3_lhs(1873, 224, 1978, 1919);
            s.store_mul_add_rhs(841, 1873, 832, 840);
        }

        s.b[2245] = (s.v[841] > (0.5 * s.v[229]));
        s.v[2245] = if s.b[2245] { 1.0 } else { 0.0 };

        if ((s.b[2241] && s.b[2242]) && s.b[2245]) {
            s.store_offset_ad(1919, A::div_scaled_inputs(s.ad_value(841), 2.0, s.ad_value(229), 1.0), (-1.0));
            s.store_mul_scaled_ad_rhs(841, 229, 0.5, A::offset(A::div(s.ad_value(1919), A::sqrt(A::offset(A::square(s.ad_value(1919)), 1.0))), 1.0));
        }

        s.b[2439] = (((p.p45 == 1.0) || (p.p47 > 0.0)) || (p.p48 > 0.0));
        s.v[2439] = if s.b[2439] { 1.0 } else { 0.0 };

        s.b[2440] = ((p.p45 > 0.0) || (p.p47 > 0.0));
        s.v[2440] = if s.b[2440] { 1.0 } else { 0.0 };

        if (s.b[2439] && s.b[2440]) {
            s.copy_ad(2280, 722);
            s.copy_ad(2281, 732);
            s.copy_ad(2282, 723);
            s.copy_ad(2283, 1808);
            s.copy_ad(2284, 1809);
            s.store_scalar(2288, 0.0);
        }

        s.b[2441] = (p.p47 > 0.0);
        s.v[2441] = if s.b[2441] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2440]) && s.b[2441]) {
            s.store_add_ad_lhs(2283, A::add_scaled_inputs3(s.ad_value(822), 0.5, s.ad_value(821), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(743), 1.0, A::sub(s.ad_value(822), s.ad_value(821)), A::sub(s.ad_value(822), s.ad_value(821)), 1.0)), (-0.5)), 741);
            s.store_add_ad_lhs(1874, A::add_scaled_inputs3(s.ad_value(821), 1.0, s.ad_value(2283), (-0.5), A::sqrt(A::add_scaled_product(s.ad_value(742), 1.0, s.ad_value(2283), s.ad_value(2283), 1.0)), (-(-0.5))), 744);
            s.copy_ad(2284, 1874);
            s.copy_ad(2280, 739);
            s.copy_ad(2281, 742);
            s.copy_ad(2282, 740);
        }

        if (s.b[2439] && s.b[2440]) {
            s.store_ad_value(2287, A::add_scaled_inputs3(s.ad_value(823), 1.0, s.ad_value(2288), (-1.0), s.ad_value(694), -1.0));
            s.store_ad_value(2289, A::add_scaled_inputs3(s.ad_value(2284), 1.0, s.ad_value(820), 0.5, s.ad_value(824), (-0.5)));
            s.store_scalar(2301, 1.0);
        }

        s.b[2442] = (s.v[185] > 0.0);
        s.v[2442] = if s.b[2442] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2440]) && s.b[2442]) {
            s.store_scale(2292, 2280, s.v[355]);
            s.store_scale(2293, 2289, s.v[355]);
            s.store_scale(2294, 2287, s.v[355]);
            s.store_offset_ad(1920, A::div_scaled_inputs(s.ad_value(2282), 0.5, A::sqrt(s.ad_value(2292)), 1.0), 1.0);
            s.store_ad_value(1921, A::add_scaled_product(s.ad_value(2292), 1.0, s.ad_value(2282), A::sqrt(s.ad_value(2292)), 1.0));
            s.store_ad_value(2295, A::add_scaled_inputs_product(A::div(A::sub(s.ad_value(2294), s.ad_value(1921)), s.ad_value(1920)), 1.0, s.ad_value(2292), 0.5, A::offset(s.ad_value(186), 1.0), s.ad_value(2293), (-1.0)));
            s.store_offset_scaled(2296, 2292, 0.5, 2.0);
            s.store_add(2297, 2292, 2293);
            s.store_ad_value(1920, A::sub_scaled_inputs(A::add_scaled_inputs_product(s.ad_value(2294), 1.0, s.ad_value(2297), (-1.0), s.ad_value(2282), A::sqrt(s.ad_value(2297)), (-1.0)), 1.0, A::ln(A::add(A::div(s.ad_value(2292), s.ad_value(2282)), A::sqrt(s.ad_value(2292)))), 2.0));
            s.store_add_scaled_inputs(2298, 1920, 2.0, 2296, 1.0);
            s.store_ad_value(1920, A::add_scaled_inputs3(s.ad_value(2295), 0.5, s.ad_value(2298), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2295), s.ad_value(2298)), A::sub(s.ad_value(2295), s.ad_value(2298))), 20.0)), 0.5));
            s.store_ad_value(1921, A::add_scaled_inputs3(s.ad_value(2294), 2.0, s.ad_value(2293), (-2.0), s.ad_value(2296), -1.0));
            s.store_ad_value(2299, A::add_scaled_inputs3(s.ad_value(1920), 0.5, s.ad_value(1921), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1920), s.ad_value(1921)), A::sub(s.ad_value(1920), s.ad_value(1921))), 20.0)), (-0.5)));
            s.store_ad_value(1920, A::add_scaled_inputs3(s.ad_value(2299), 0.5, s.ad_value(2296), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2299), s.ad_value(2296)), A::sub(s.ad_value(2299), s.ad_value(2296))), 5.0)), (-0.5)));
            s.store_ad_value(2300, A::add_scaled_inputs3(s.ad_value(1920), 0.5, s.ad_value(2296), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul(A::sub_scaled_inputs(s.ad_value(1920), 1.0, s.ad_value(2296), -1.0), A::sub_scaled_inputs(s.ad_value(1920), 1.0, s.ad_value(2296), -1.0)), 20.0)), 0.5));
            s.store_mul_offset_ad_rhs(1921, 696, A::div(s.ad_value(2300), s.ad_value(2296)), 1.0);
        }

        s.b[2443] = (s.v[1921] > (-230.25850929940458));
        s.v[2443] = if s.b[2443] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2440]) && s.b[2442]) && s.b[2443]) {
            s.store_exp(2301, 1921);
        }

        if (((s.b[2439] && s.b[2440]) && s.b[2442]) && (!s.b[2443])) {
            s.store_div_from_scalar_offset_ad(2301, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1921), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1921), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (s.b[2439] && s.b[2440]) {
            s.store_offset_mul(2302, 695, 2301, 1.0);
            s.store_scale(2303, 2302, s.v[709]);
            s.store_mul_ad_product_rhs(2304, 194, A::offset(A::mul(s.ad_value(196), s.ad_value(824)), 1.0), A::offset(A::mul(s.ad_value(195), s.ad_value(2289)), 1.0));
            s.store_mul_offset_rhs(2305, 2303, 2304, 1.0);
            s.store_div_from_scalar(2306, 1.0, 2305);
            s.store_mul_ad_rhs(2290, 2282, A::sqrt_scaled_input(s.ad_value(2306), s.v[709]));
            s.store_square(2291, 2290);
            s.store_div_from_scalar(2307, 1.0, 2291);
            s.store_mul(2308, 2284, 2306);
            s.store_mul(2309, 2287, 2306);
            s.store_ad_value(2310, A::div_scaled_inputs(s.ad_value(824), 2.0, A::offset(A::sqrt(A::offset(A::mul(s.ad_value(192), s.ad_value(824)), 1.0)), 1.0), 1.0));
            s.store_mul_ad_product_rhs(2311, 191, s.ad_value(2310), A::offset(A::mul(s.ad_value(193), s.ad_value(2289)), 1.0));
            s.store_mul(2312, 2280, 2306);
            s.store_sqrt_square_add(1920, 2283, 2281);
            s.store_sqrt_ad(1921, A::add_scaled_product(s.ad_value(2281), 1.0, A::sub(s.ad_value(2283), s.ad_value(2311)), A::sub(s.ad_value(2283), s.ad_value(2311)), 1.0));
            s.store_mul_scaled_ad_rhs(2313, 2306, 0.5, A::add_scaled_inputs3(s.ad_value(2311), 1.0, s.ad_value(1920), 1.0, s.ad_value(1921), -1.0));
            s.store_add(2314, 2312, 2308);
            s.store_sub(2315, 2314, 2313);
        }

        s.b[2444] = (p.p45 > 0.0);
        s.v[2444] = if s.b[2444] { 1.0 } else { 0.0 };

        s.b[2445] = (((s.v[2315]) as f64).abs() < 1e-5);
        s.v[2445] = if s.b[2445] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2440]) && s.b[2444]) && s.b[2445]) {
            s.store_offset_ad(2316, A::mul_sub_from_scalar_rhs(s.ad_value(2290), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2315), 1.0, A::scale(s.ad_value(2315), 0.3125), 0.5)), 1.0);
        }

        s.b[2446] = (s.v[2315] < 460.51701859880916);
        s.v[2446] = if s.b[2446] { 1.0 } else { 0.0 };

        if ((((s.b[2439] && s.b[2440]) && s.b[2444]) && (!s.b[2445])) && s.b[2446]) {
            s.store_exp_neg_input(2330, 2315);
        }

        if ((((s.b[2439] && s.b[2440]) && s.b[2444]) && (!s.b[2445])) && (!s.b[2446])) {
            s.store_div_from_scalar_offset_ad(2330, 1e-200, A::mul_offset_lhs(s.ad_value(2315), (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2315), (-460.51701859880916), A::scale_offset(s.ad_value(2315), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        if (((s.b[2439] && s.b[2440]) && s.b[2444]) && (!s.b[2445])) {
            s.store_scalar(1919, (if (s.v[2315] > 0.0) { 1.0 } else { (-1.0) }));
        }

        if (((s.b[2439] && s.b[2440]) && s.b[2444]) && (!s.b[2445])) {
            s.store_offset_ad(2316, A::div_scaled_product3(s.ad_value(1919), s.ad_value(2290), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(2330), 1.0, s.ad_value(2315))), 1.0, A::sqrt(A::mul_sub_from_scalar_rhs(s.ad_value(2315), 1.0, s.ad_value(2330))), 2.0), 1.0);
        }

        if ((s.b[2439] && s.b[2440]) && (!s.b[2444])) {
            s.store_offset_ad(2316, A::div_scaled_inputs(s.ad_value(2290), 0.5, A::sqrt(s.ad_value(2315)), 1.0), 1.0);
        }

        if (s.b[2439] && s.b[2440]) {
            s.store_ad_value(2317, A::add_scaled_value_products(s.ad_value(2315), 1.0, s.ad_value(2290), A::sqrt(s.ad_value(2315)), 1.0, s.ad_value(2316), A::ln(A::offset(s.ad_value(2316), (-1.0))), (-1.0)));
            s.store_div_ad_lhs(2318, A::sub(s.ad_value(2309), s.ad_value(2317)), 2316);
            s.store_mul_scaled_ad_rhs(2324, 2291, 0.5, A::offset(A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2291)), 1.0)), (-1.0)));
            s.store_scalar(2323, 0.0);
            s.store_scalar(2325, 1.0);
        }

        s.b[2447] = (s.v[2318] > (-30.0));
        s.v[2447] = if s.b[2447] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2440]) && s.b[2447]) {
            s.store_offset_mul(2319, 2316, 2318, (-1.0));
            s.store_scaled_add_ad_rhs(1919, 2319, A::sqrt(A::offset(A::square(s.ad_value(2319)), 10.0)), 0.5);
            s.store_sub_ad_rhs(2320, 2318, A::ln(s.ad_value(1919)));
            s.store_scaled_add_ad_rhs(2321, 2320, A::sqrt(A::offset(A::square(s.ad_value(2320)), 2.0)), 0.5);
        }

        s.b[2448] = ((s.v[2318] - s.v[2321]) < 230.25850929940458);
        s.v[2448] = if s.b[2448] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2440]) && s.b[2447]) && s.b[2448]) {
            s.store_exp_sub(1919, 2318, 2321);
        }

        if (((s.b[2439] && s.b[2440]) && s.b[2447]) && (!s.b[2448])) {
            s.store_scaled_offset_ad(1919, A::mul_offset_lhs(A::sub(s.ad_value(2318), s.ad_value(2321)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2318), s.ad_value(2321)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2318), s.ad_value(2321)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2439] && s.b[2440]) && s.b[2447]) {
            s.store_div(2322, 1919, 2316);
            s.store_sub_ad_lhs(1919, A::scaled_offset(s.ad_value(2321), 1.0, 2.0), 2322);
        }

    }

    pub(super) fn stamp_transient_block_34(
        s: &mut Scratch,
    ) {
        s.b[2449] = (s.v[2322] > 1e-6);
        s.v[2449] = if s.b[2449] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2440]) && s.b[2447]) && s.b[2449]) {
            s.store_mul_offset_ad_rhs(2323, 2316, A::sub(s.ad_value(2321), A::div(A::offset(A::sqrt(A::offset(A::mul(s.ad_value(2322), s.ad_value(1919)), 1.0)), (-1.0)), s.ad_value(2322))), 1.0);
        }

        if (((s.b[2439] && s.b[2440]) && s.b[2447]) && (!s.b[2449])) {
            s.store_mul_ad_affine_product_rhs(2323, 2316, s.ad_value(2322), A::offset(A::mul_scaled_lhs(s.ad_value(1919), 0.25, s.ad_value(1919)), 1.0), 0.5, 0.0);
        }

        if ((s.b[2439] && s.b[2440]) && s.b[2447]) {
            s.store_ad_value(1919, A::add_scaled_inputs3_offset(s.ad_value(2309), 0.5, s.ad_value(2323), ((-1.0) * 0.5), A::sqrt(A::offset(A::mul_offset_lhs(A::sub(s.ad_value(2309), s.ad_value(2323)), (-2.0), A::offset(A::sub(s.ad_value(2309), s.ad_value(2323)), (-2.0))), 1.0)), 0.5, (2.0 * 0.5)));
            s.store_mul_scaled_ad_rhs(2324, 2291, 0.5, A::offset(A::sqrt(A::offset(A::mul(A::div_from_scalar(4.0, s.ad_value(2291)), s.ad_value(1919)), 1.0)), (-1.0)));
            s.store_div_ad_rhs(2325, 2324, A::add(s.ad_value(2324), s.ad_value(2323)));
            s.store_ad_value(2315, A::add_scaled_product(s.ad_value(2314), 1.0, s.ad_value(2325), s.ad_value(2313), (-1.0)));
        }

        if (s.b[2439] && s.b[2440]) {
            s.store_offset_scaled(2326, 2290, 0.7071067811865475, 1.0);
            s.store_scale(2327, 2326, 1e-5);
            s.store_div_from_scalar(2328, 1.0, 2326);
            s.store_scalar(2435, 0.0);
            s.store_scalar(2329, 0.0);
        }

        s.b[2450] = (s.v[2315] < 460.51701859880916);
        s.v[2450] = if s.b[2450] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2440]) && s.b[2450]) {
            s.store_exp_neg_input(2330, 2315);
        }

        if ((s.b[2439] && s.b[2440]) && (!s.b[2450])) {
            s.store_div_from_scalar_offset_ad(2330, 1e-200, A::mul_offset_lhs(s.ad_value(2315), (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2315), (-460.51701859880916), A::scale_offset(s.ad_value(2315), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        s.b[2451] = (((s.v[2309]) as f64).abs() <= s.v[2327]);
        s.v[2451] = if s.b[2451] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2440]) && s.b[2451]) {
            s.store_scaled_square(2415, 2328, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(2329, 2309, s.ad_value(2328), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2309), 1.0, s.ad_value(2330)), s.ad_value(2290), s.ad_value(2415)), 1.0));
        }

        s.b[2452] = (s.v[2309] < (-s.v[2327]));
        s.v[2452] = if s.b[2452] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && s.b[2452]) {
            s.store_neg(2417, 2309);
            s.store_scaled_mul(2418, 2417, 2328, 1.25);
            s.store_scaled_sub_ad(2419, A::offset(s.ad_value(2418), 10.0), A::sqrt(A::offset(A::mul_offset_lhs(s.ad_value(2418), (-6.0), A::offset(s.ad_value(2418), (-6.0))), 64.0)), 0.5);
            s.store_sub(2414, 2417, 2419);
            s.store_ad_value(2420, A::add_scaled_square_product(s.ad_value(2414), 1.0, s.ad_value(2291), A::offset(s.ad_value(2419), 1.0), 1.0));
            s.store_sub_scaled_inputs(2421, 2414, 2.0, 2291, 1.0);
            s.store_sub_ad_lhs(2422, A::ln(A::mul(s.ad_value(2420), s.ad_value(2307))), 2419);
            s.store_add(818, 2420, 2421);
            s.store_ad_value(817, A::add_scaled_square_product(s.ad_value(818), 1.0, s.ad_value(2422), A::sub_scaled_inputs(A::square(s.ad_value(2421)), 0.5, s.ad_value(2420), 1.0), 1.0));
            s.store_add_ad_rhs(2423, 2419, A::div_scaled_product3(s.ad_value(2420), s.ad_value(818), s.ad_value(2422), 1.0, A::add(s.ad_value(817), A::mul3(A::mul3(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2422), s.ad_value(2422)), s.ad_value(2421), A::sub_scaled_inputs(A::square(s.ad_value(2421)), 0.3333333333333333, s.ad_value(2420), 1.0))), 1.0));
        }

        s.b[2453] = (s.v[2423] < 230.25850929940458);
        s.v[2453] = if s.b[2453] { 1.0 } else { 0.0 };

        if ((((s.b[2439] && s.b[2440]) && (!s.b[2451])) && s.b[2452]) && s.b[2453]) {
            s.store_exp(2424, 2423);
        }

        if ((((s.b[2439] && s.b[2440]) && (!s.b[2451])) && s.b[2452]) && (!s.b[2453])) {
            s.store_scaled_offset_ad(2424, A::mul_offset_lhs(s.ad_value(2423), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2423), (-230.25850929940458), A::scale_offset(s.ad_value(2423), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && s.b[2452]) {
            s.store_div_from_scalar(2425, 1.0, 2424);
            s.store_div_from_scalar_offset_ad(2414, 1.0, A::square(s.ad_value(2423)), 2.0);
            s.store_mul_square_lhs(2426, 2423, 2414);
            s.store_mul3_affine_lhs(2427, 2423, 2414, 4.0, 0.0, 2414);
            s.store_mul_ad_product_lhs(2428, A::sub_scaled_inputs(s.ad_value(2414), 8.0, s.ad_value(2426), 12.0), s.ad_value(2414), 2414);
            s.store_sub(2414, 2417, 2423);
            s.store_mul(2415, 2330, 2425);
            s.store_ad_value(2429, A::add_scaled_product(s.ad_value(2414), 2.0, s.ad_value(2291), A::add_scaled_inputs3_offset(s.ad_value(2424), 1.0, s.ad_value(2415), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(2330), 1.0, s.ad_value(2427)), 1.0, (-1.0)), 1.0));
            s.store_ad_value(2430, A::add_scaled_square_product(s.ad_value(2414), 1.0, s.ad_value(2291), A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2424), 1.0, s.ad_value(2423), (-1.0), s.ad_value(2415), 1.0, (-1.0)), 1.0, s.ad_value(2330), A::sub(A::offset(s.ad_value(2423), (-1.0)), s.ad_value(2426)), 1.0), (-1.0)));
            s.store_sub_from_scalar_ad(2414, 2.0, A::mul(s.ad_value(2291), A::add_scaled_inputs_product(s.ad_value(2424), 1.0, s.ad_value(2415), 1.0, s.ad_value(2330), s.ad_value(2428), (-1.0))));
            s.store_ad_value(2414, A::add_scaled_square_product(s.ad_value(2429), 1.0, s.ad_value(2430), s.ad_value(2414), (-2.0)));
            s.store_ad_value(2329, A::sub_scaled_inputs(s.ad_value(2423), -1.0, A::div(s.ad_value(2430), A::add(s.ad_value(2429), A::sqrt(s.ad_value(2414)))), 2.0));
        }

        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) {
            s.store_div_from_scalar_offset_scaled_input(2431, 1.0, 2290, 0.7324648775608221, 1.25);
            s.store_mul_offset_ad_lhs(2432, A::mul_scaled_lhs(s.ad_value(2326), 1.25, s.ad_value(2431)), (-1.0), 2431);
            s.store_mul_ad_product_rhs(2433, 2309, s.ad_value(2328), A::offset(A::mul(s.ad_value(2432), s.ad_value(2309)), 1.0));
        }

        s.b[2454] = ((-s.v[2433]) > (-230.25850929940458));
        s.v[2454] = if s.b[2454] { 1.0 } else { 0.0 };

        if ((((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) && s.b[2454]) {
            s.store_exp_neg_input(2414, 2433);
        }

        if ((((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) && (!s.b[2454])) {
            s.store_div_from_scalar_offset_ad(2414, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::neg(s.ad_value(2433)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::neg(s.ad_value(2433)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) {
            s.store_sub_from_scalar(2434, 1.0, 2414);
            s.store_ad_value(2435, A::add_scaled_inputs_product(s.ad_value(2309), 1.0, s.ad_value(2291), 0.5, s.ad_value(2290), A::sqrt(A::add_scaled_inputs3(s.ad_value(2309), 1.0, s.ad_value(2291), 0.25, s.ad_value(2434), -1.0)), (-1.0)));
            s.store_offset(2436, 2315, 3.0);
            s.store_ad_value(2419, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(2435), 0.5, s.ad_value(2436), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2435), s.ad_value(2436)), A::sub(s.ad_value(2435), s.ad_value(2436))), 5.0)), (-0.5)), 1.0, s.ad_value(2436), (-0.5), A::sqrt(A::offset(A::square(s.ad_value(2436)), 5.0)), (-(-0.5))));
            s.store_sub(2414, 2309, 2419);
            s.store_exp_neg_input(2415, 2419);
            s.store_div_from_scalar_offset_ad(2416, 1.0, A::square(s.ad_value(2419)), 2.0);
            s.store_mul_square_lhs(2426, 2419, 2416);
            s.store_mul3_affine_lhs(2427, 2419, 2416, 4.0, 0.0, 2416);
            s.store_mul_ad_product_lhs(2428, A::sub_scaled_inputs(s.ad_value(2416), 8.0, s.ad_value(2426), 12.0), s.ad_value(2416), 2416);
        }

        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) {
            let assign49580_ad_e63936: A = {
                if (1e-40 > ((s.v[2414] * s.v[2414]) - (s.v[2291] * (((s.v[2415] + s.v[2419]) - 1.0) - (s.v[2330] * ((s.v[2419] + 1.0) + s.v[2426])))))) {
                    A::constant(1e-40)
                } else {
                    A::add_scaled_square_product(s.ad_value(2414), 1.0, s.ad_value(2291), A::add_scaled_product(A::offset(A::add(s.ad_value(2415), s.ad_value(2419)), (-1.0)), 1.0, s.ad_value(2330), A::add(A::offset(s.ad_value(2419), 1.0), s.ad_value(2426)), (-1.0)), (-1.0))
                }
            };
            s.store_ad_value(2420, assign49580_ad_e63936);
        }

        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) {
            s.store_sub_from_scalar_ad(2437, 1.0, A::mul_scaled_output(s.ad_value(2291), A::add_scaled_product(s.ad_value(2415), 1.0, s.ad_value(2330), s.ad_value(2428), (-1.0)), 0.5));
            s.store_ad_value(2421, A::add_scaled_product(s.ad_value(2414), 2.0, s.ad_value(2291), A::add_scaled_sub_value_product(1.0, s.ad_value(2415), 1.0, s.ad_value(2330), A::offset(s.ad_value(2427), 1.0), (-1.0)), 1.0));
            s.store_ad_value(2422, A::add_scaled_inputs3(s.ad_value(2315), 1.0, s.ad_value(2419), (-1.0), A::ln(A::div(s.ad_value(2420), s.ad_value(2291))), 1.0));
            s.store_add(818, 2420, 2421);
            s.store_ad_value(817, A::add_scaled_square_product(s.ad_value(818), 1.0, s.ad_value(2422), A::add_scaled_square_product(s.ad_value(2421), 0.5, s.ad_value(2420), s.ad_value(2437), (-1.0)), 1.0));
            s.store_add_ad_rhs(2438, 2419, A::div_scaled_product3(s.ad_value(2420), s.ad_value(818), s.ad_value(2422), 1.0, A::add(s.ad_value(817), A::mul3(A::mul3(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2422), s.ad_value(2422)), s.ad_value(2421), A::add_scaled_square_product(s.ad_value(2421), 0.3333333333333333, s.ad_value(2420), s.ad_value(2437), (-1.0)))), 1.0));
        }

        s.b[2455] = (s.v[2438] < 230.25850929940458);
        s.v[2455] = if s.b[2455] { 1.0 } else { 0.0 };

        if ((((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) && s.b[2455]) {
            s.store_exp(2424, 2438);
            s.store_div_from_scalar(2425, 1.0, 2424);
            s.store_mul(2424, 2330, 2424);
        }

        s.b[2456] = (s.v[2438] > (s.v[2315] - 230.25850929940458));
        s.v[2456] = if s.b[2456] { 1.0 } else { 0.0 };

        if (((((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) && (!s.b[2455])) && s.b[2456]) {
            s.store_exp_sub(2424, 2438, 2315);
            s.store_div(2425, 2330, 2424);
        }

        if (((((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) && (!s.b[2455])) && (!s.b[2456])) {
            s.store_div_from_scalar_offset_ad(2424, 1e-100, A::mul_offset_lhs(A::sub(s.ad_value(2315), s.ad_value(2438)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2315), s.ad_value(2438)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2315), s.ad_value(2438)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(2425, 1e-100, A::mul_offset_lhs(s.ad_value(2438), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2438), (-230.25850929940458), A::scale_offset(s.ad_value(2438), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        if (((s.b[2439] && s.b[2440]) && (!s.b[2451])) && (!s.b[2452])) {
            s.store_div_from_scalar_offset_ad(2414, 1.0, A::square(s.ad_value(2438)), 2.0);
            s.store_mul_square_lhs(2426, 2438, 2414);
            s.store_mul3_affine_lhs(2427, 2438, 2414, 4.0, 0.0, 2414);
            s.store_mul_ad_product_lhs(2428, A::sub_scaled_inputs(s.ad_value(2414), 8.0, s.ad_value(2426), 12.0), s.ad_value(2414), 2414);
            s.store_sub(2414, 2309, 2438);
            s.store_ad_value(2429, A::add_scaled_product(s.ad_value(2414), 2.0, s.ad_value(2291), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2425)), 1.0, s.ad_value(2424), 1.0, s.ad_value(2330), A::offset(s.ad_value(2427), 1.0), (-1.0)), 1.0));
            s.store_ad_value(2430, A::add_scaled_square_product(s.ad_value(2414), 1.0, s.ad_value(2291), A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2425), 1.0, s.ad_value(2438), 1.0, s.ad_value(2424), 1.0, (-1.0)), 1.0, s.ad_value(2330), A::add(A::offset(s.ad_value(2438), 1.0), s.ad_value(2426)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_ad(2414, 2.0, A::mul(s.ad_value(2291), A::add_scaled_inputs_product(s.ad_value(2425), 1.0, s.ad_value(2424), 1.0, s.ad_value(2330), s.ad_value(2428), (-1.0))));
            s.store_ad_value(2414, A::add_scaled_square_product(s.ad_value(2429), 1.0, s.ad_value(2430), s.ad_value(2414), (-2.0)));
            s.store_ad_value(2329, A::add_scaled_inputs(s.ad_value(2438), 1.0, A::div(s.ad_value(2430), A::add(s.ad_value(2429), A::sqrt(s.ad_value(2414)))), 2.0));
        }

        if (s.b[2439] && s.b[2440]) {
            s.store_scalar(2332, 0.0);
            s.store_scalar(2333, 0.0);
            s.store_scalar(2334, 0.0);
            s.store_scalar(2335, 0.0);
            s.store_scalar(2336, 0.0);
            s.store_scalar(2337, 0.0);
            s.store_scalar(2338, 0.0);
            s.store_scalar(2339, 1.0);
            s.store_scalar(2340, 1.0);
            s.store_sub(2341, 2309, 2329);
            s.store_scalar(2342, 0.0);
            s.store_mul(2343, 2305, 2341);
            s.store_scalar(2344, 1.0);
            s.store_scalar(2345, 1.0);
            s.store_scalar(2349, 1.0);
            s.store_scalar(2350, 1.0);
            s.store_scalar(2352, 1.0);
        }

        s.b[2457] = (s.v[2309] > 0.0);
        s.v[2457] = if s.b[2457] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2440]) && s.b[2457]) {
            s.store_div_from_scalar_offset_ad(1919, 1.0, A::square(s.ad_value(2329)), 2.0);
            s.store_mul_square_lhs(2331, 2329, 1919);
            s.store_mul3_affine_lhs(2332, 2329, 1919, 4.0, 0.0, 1919);
            s.store_mul_ad_product_lhs(2333, A::sub_scaled_inputs(s.ad_value(1919), 8.0, s.ad_value(2331), 12.0), s.ad_value(1919), 1919);
            s.store_scalar(2334, 0.0);
        }

        s.b[2458] = (s.v[2329] < 230.25850929940458);
        s.v[2458] = if s.b[2458] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2458]) {
            s.store_exp(2334, 2329);
            s.store_div_from_scalar(2335, 1.0, 2334);
            s.store_mul(2334, 2330, 2334);
        }

        s.b[2459] = (s.v[2329] > (s.v[2315] - 230.25850929940458));
        s.v[2459] = if s.b[2459] { 1.0 } else { 0.0 };

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && (!s.b[2458])) && s.b[2459]) {
            s.store_exp_sub(2334, 2329, 2315);
            s.store_div(2335, 2330, 2334);
        }

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && (!s.b[2458])) && (!s.b[2459])) {
            s.store_div_from_scalar_offset_ad(2334, 1e-100, A::mul_offset_lhs(A::sub(s.ad_value(2315), s.ad_value(2329)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2315), s.ad_value(2329)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2315), s.ad_value(2329)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(2335, 1e-100, A::mul_offset_lhs(s.ad_value(2329), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2329), (-230.25850929940458), A::scale_offset(s.ad_value(2329), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        if ((s.b[2439] && s.b[2440]) && s.b[2457]) {
            s.store_ad_value(2336, A::add_scaled_product(s.ad_value(2334), 1.0, s.ad_value(2330), A::add(A::offset(s.ad_value(2329), 1.0), s.ad_value(2331)), (-1.0)));
        }

        s.b[2460] = (s.v[2329] < 1e-5);
        s.v[2460] = if s.b[2460] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2460]) {
            s.store_ad_value(2337, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(2329)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2329), 1.0, A::scale(s.ad_value(2329), 0.25), 0.3333333333333333), 0.5));
            s.store_ad_value(2336, A::mul3_scaled_output(A::mul3(s.ad_value(2330), s.ad_value(2329), s.ad_value(2329)), s.ad_value(2329), A::scale_offset(s.ad_value(2329), 1.75, 1.0), 0.16666666666666666));
            s.store_sqrt_sub_from_scalar_ad(1919, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2329), 1.0, A::scale(s.ad_value(2329), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2338, 2329, 1919, 0.7071067811865475);
            s.store_offset_ad(2339, A::div_scaled_product(s.ad_value(2290), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2329), 0.5)), 1.0, A::square(s.ad_value(2329)), 0.16666666666666666), 0.7071067811865475, s.ad_value(1919), 1.0), 1.0);
        }

        if (((s.b[2439] && s.b[2440]) && s.b[2457]) && (!s.b[2460])) {
            s.store_add_ad_lhs(2337, A::offset(s.ad_value(2329), (-1.0)), 2335);
            s.store_sqrt(2338, 2337);
            s.store_offset_scaled_ad(2339, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2290), 1.0, s.ad_value(2335)), s.ad_value(2338)), 0.5, 1.0);
        }

        if ((s.b[2439] && s.b[2440]) && s.b[2457]) {
            s.store_div_ad(2340, A::offset(A::mul_scaled_lhs(s.ad_value(702), 0.2, s.ad_value(2289)), 1.0), A::offset(A::mul(s.ad_value(702), s.ad_value(2289)), 1.0));
        }

    }

    pub(super) fn stamp_transient_block_35(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2461] = (s.v[2336] > 1e-100);
        s.v[2461] = if s.b[2461] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) {
            s.store_mul_sqrt_ad_rhs(2341, 2290, A::add(s.ad_value(2337), s.ad_value(2336)));
            s.store_ad_value(2342, A::div_scaled_product3(s.ad_value(2291), s.ad_value(2336), s.ad_value(2305), 1.0, A::add_scaled_product(s.ad_value(2341), 1.0, s.ad_value(2290), s.ad_value(2338), 1.0), 1.0));
            s.store_mul3_lhs(2343, 2338, 2290, 2305);
        }

        s.b[2462] = (s.v[212] < 0.0);
        s.v[2462] = if s.b[2462] { 1.0 } else { 0.0 };

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && s.b[2462]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2344, 1.0, 1.0, A::mul(s.ad_value(212), s.ad_value(2289)));
        }

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && (!s.b[2462])) {
            s.store_offset_mul(2344, 212, 2289, 1.0);
        }

        s.b[2463] = (s.v[213] < 0.0);
        s.v[2463] = if s.b[2463] { 1.0 } else { 0.0 };

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && s.b[2463]) {
            s.store_sub_from_scalar_ad(2345, 1.0, A::mul(s.ad_value(213), s.ad_value(2342)));
        }

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && (!s.b[2463])) {
            s.store_div_from_scalar_offset_ad(2345, 1.0, A::mul(s.ad_value(213), s.ad_value(2342)), 1.0);
        }

        if (((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) {
            s.store_mul_ad_lhs(2346, A::mul3(s.ad_value(751), s.ad_value(2344), s.ad_value(2345)), 2342);
            s.store_mul_ad_rhs(2347, 768, A::add_scaled_product(s.ad_value(2343), 1.0, s.ad_value(769), s.ad_value(2342), 1.0));
            s.store_ln_ad(1920, A::div(s.ad_value(2337), A::offset(A::add(s.ad_value(2337), s.ad_value(2336)), 1e-14)));
            s.store_ad_value(2348, A::add_scaled_product(A::pow(A::mul(s.ad_value(2347), s.ad_value(698)), s.ad_value(699)), 1.0, s.ad_value(700), A::exp(A::mul_scaled_lhs(s.ad_value(701), 0.5, s.ad_value(1920))), 1.0));
            s.store_mul_add_ad_lhs(2349, A::offset(s.ad_value(2348), 1.0), s.ad_value(2346), 2340);
        }

        s.b[2464] = (s.v[216] < 0.0);
        s.v[2464] = if s.b[2464] { 1.0 } else { 0.0 };

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && s.b[2464]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2350, 1.0, 1.0, A::mul(s.ad_value(216), s.ad_value(2289)));
        }

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && (!s.b[2464])) {
            s.store_offset_mul(2350, 216, 2289, 1.0);
        }

        if (((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) {
            s.store_mul(1921, 2342, 2350);
            s.store_div_ad_rhs(2351, 1921, A::add(s.ad_value(218), s.ad_value(1921)));
        }

        s.b[2465] = (s.v[217] < 0.0);
        s.v[2465] = if s.b[2465] { 1.0 } else { 0.0 };

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && s.b[2465]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2352, 1.0, 1.0, A::mul(s.ad_value(217), s.ad_value(2351)));
        }

        if ((((s.b[2439] && s.b[2440]) && s.b[2457]) && s.b[2461]) && (!s.b[2465])) {
            s.store_offset_mul(2352, 217, 2351, 1.0);
        }

        if (s.b[2439] && (!s.b[2440])) {
            s.copy_ad(2287, 1810);
            s.copy_ad(2289, 1811);
            s.copy_ad(2305, 1812);
            s.copy_ad(2306, 1813);
            s.copy_ad(2290, 1814);
            s.copy_ad(2291, 1815);
            s.copy_ad(2307, 1816);
            s.copy_ad(2309, 1817);
            s.copy_ad(2314, 1818);
            s.copy_ad(2315, 1819);
            s.copy_ad(2326, 1820);
            s.copy_ad(2327, 1821);
            s.copy_ad(2328, 1822);
            s.copy_ad(2435, 1823);
            s.copy_ad(2330, 1824);
            s.copy_ad(2329, 1825);
            s.copy_ad(2332, 1826);
            s.copy_ad(2333, 1827);
            s.copy_ad(2334, 1828);
            s.copy_ad(2335, 1829);
            s.copy_ad(2337, 1830);
            s.copy_ad(2336, 1831);
            s.copy_ad(2338, 1832);
            s.copy_ad(2339, 1833);
            s.copy_ad(2340, 1834);
            s.copy_ad(2341, 1835);
            s.copy_ad(2342, 1836);
            s.copy_ad(2343, 1837);
            s.copy_ad(2344, 1838);
            s.copy_ad(2345, 1839);
            s.copy_ad(2349, 1840);
            s.copy_ad(2350, 1841);
            s.copy_ad(2352, 1842);
        }

        if s.b[2439] {
            s.copy_ad(2285, 714);
            s.copy_ad(2286, 771);
        }

        s.b[2466] = (p.p48 != 0.0);
        s.v[2466] = if s.b[2466] { 1.0 } else { 0.0 };

        if (s.b[2439] && s.b[2466]) {
            s.copy_ad(2285, 715);
            s.copy_ad(2286, 772);
        }

        if s.b[2439] {
            s.store_scalar(2354, 0.0);
            s.store_scale(2353, 2305, 4.60517018598809);
            s.copy_ad(2370, 2353);
            s.copy_ad(2371, 820);
            s.store_mul(2372, 820, 2306);
            s.copy_ad(2376, 2329);
            s.store_scalar(2377, 0.0);
            s.store_scalar(2380, 0.0);
            s.copy_ad(2382, 2335);
            s.copy_ad(2383, 2337);
            s.copy_ad(2385, 2336);
            s.copy_ad(2386, 2343);
            s.copy_ad(2387, 2329);
            s.copy_ad(2388, 2335);
            s.copy_ad(2390, 2336);
            s.copy_ad(2391, 2337);
            s.store_sub(2392, 2309, 2329);
            s.store_scalar(2393, 1.0);
            s.store_scalar(2395, 1.0);
            s.store_scalar(2394, 0.0);
            s.copy_ad(2404, 2342);
            s.store_mul(2408, 2392, 2305);
            s.store_scalar(2405, 0.0);
            s.copy_ad(2406, 2343);
            s.store_scalar(2411, 0.0);
            s.store_scalar(2410, 1.0);
            s.copy_ad(2413, 2285);
            s.copy_ad(2412, 2408);
        }

        s.b[2467] = (s.v[2309] > 0.0);
        s.v[2467] = if s.b[2467] { 1.0 } else { 0.0 };

        s.b[2468] = (s.v[2336] > 1e-100);
        s.v[2468] = if s.b[2468] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2467]) && s.b[2468]) {
            s.store_mul(2413, 2285, 2352);
            s.store_div(2354, 2413, 2349);
            s.store_add_scaled_inputs(2355, 2341, 1.0, 2291, 0.5);
            s.store_div_ad_lhs(1919, A::div_scaled_product(s.ad_value(2291), s.ad_value(2334), 1.0, s.ad_value(2355), 1.0), 2355);
        }

        s.b[2469] = (s.v[1919] > 0.0001);
        s.v[2469] = if s.b[2469] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2469]) {
            s.store_sub_from_scalar(1920, 1.0, 1919);
        }

        s.b[2470] = (s.v[1920] < 1e-10);
        s.v[2470] = if s.b[2470] { 1.0 } else { 0.0 };

        if ((((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2469]) && s.b[2470]) {
            s.store_scalar(1921, 1.0);
        }

        if ((((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2469]) && (!s.b[2470])) {
            s.store_sub_from_scalar_ad(1921, 1.0, A::sqrt(s.ad_value(1920)));
        }

        if (((s.b[2439] && s.b[2467]) && s.b[2468]) && (!s.b[2469])) {
            s.store_scale(1921, 1919, 0.5);
        }

        if ((s.b[2439] && s.b[2467]) && s.b[2468]) {
            s.store_mul(2356, 1921, 2355);
        }

        s.b[2471] = ((s.v[700] > 0.0) && (s.v[701] > 0.0));
        s.v[2471] = if s.b[2471] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2471]) {
            s.store_scaled_mul(2357, 2305, 2356, 0.475);
            s.store_ad_value(1919, A::add_scaled_product(s.ad_value(2342), 1.0, s.ad_value(2339), s.ad_value(2357), (-1.0)));
            s.store_scaled_add_ad_rhs(2358, 1919, A::sqrt(A::offset(A::square(s.ad_value(1919)), 1e-12)), 0.5);
            s.store_ad_value(2359, A::add_scaled_value_products(s.ad_value(2342), (-1.0), s.ad_value(2305), s.ad_value(2341), 1.0, A::offset(s.ad_value(2339), (-1.0)), s.ad_value(2357), 1.0));
            s.store_offset_ad(2360, A::div_scaled_product(s.ad_value(2291), s.ad_value(2305), 0.5, s.ad_value(2359), 1.0), 1.0);
            s.store_ad_value(1919, A::add_scaled_product(s.ad_value(2359), 1.0, s.ad_value(769), s.ad_value(2358), 1.0));
            s.store_pow_ad(2361, A::mul3(s.ad_value(768), s.ad_value(1919), s.ad_value(698)), s.ad_value(699));
            s.store_mul_ad_lhs(1920, A::div_scaled_product_offset_rhs(s.ad_value(699), A::mul_sub_from_scalar_rhs(s.ad_value(2360), 1.0, s.ad_value(769)), (-1.0), 1.0, s.ad_value(1919), 1.0), 2361);
            s.store_div(1919, 2358, 2359);
            s.store_mul_pow_ad_rhs(2362, 700, A::offset(s.ad_value(1919), 1.0), A::neg(s.ad_value(701)));
            s.store_mul_ad_lhs(1921, A::div_scaled_product(s.ad_value(701), A::add(A::offset(s.ad_value(2360), (-1.0)), A::div_from_scalar(1.0, A::offset(s.ad_value(1919), 1.0))), 1.0, s.ad_value(2359), 1.0), 2362);
            s.store_mul_ad_lhs(2363, A::mul3(s.ad_value(751), s.ad_value(2344), s.ad_value(2345)), 2358);
            s.store_offset_div_ad(1919, A::add_scaled_product(s.ad_value(1920), 1.0, A::mul3(s.ad_value(751), s.ad_value(2344), s.ad_value(2345)), s.ad_value(2360), (-1.0)), s.ad_value(1921), 1.0);
        }

        s.b[2472] = (s.v[1919] < 230.25850929940458);
        s.v[2472] = if s.b[2472] { 1.0 } else { 0.0 };

        if ((((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2471]) && s.b[2472]) {
            s.store_scaled_ln_one_plus_exp_scaled_input(1920, 1919, 2.0, 0.5);
        }

        if ((((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2471]) && (!s.b[2472])) {
            s.copy_ad(1920, 1919);
        }

        if (((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2471]) {
            s.store_ad_value(2364, A::div_scaled_product3(s.ad_value(2357), s.ad_value(1921), s.ad_value(1920), -1.0, A::add_scaled_inputs3_offset(s.ad_value(2361), 1.0, s.ad_value(2362), 1.0, s.ad_value(2363), 1.0, 1.0), 1.0));
            s.store_mul_offset_ad_rhs(2365, 2356, A::div(s.ad_value(2364), A::offset(A::sqrt(A::offset(A::square(s.ad_value(2364)), 1.0)), 1.0)), 1.0);
        }

        if (((s.b[2439] && s.b[2467]) && s.b[2468]) && (!s.b[2471])) {
            s.copy_ad(2365, 2356);
        }

        if ((s.b[2439] && s.b[2467]) && s.b[2468]) {
            s.store_mul3_affine_lhs(2366, 2305, 2354, 0.7071067811865475, 0.0, 2365);
        }

        s.b[2473] = (s.v[0] == (-1.0));
        s.v[2473] = if s.b[2473] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2473]) {
            s.store_div_ad_rhs(2366, 2366, A::sqrt(A::offset(s.ad_value(2366), 1.0)));
        }

        if ((s.b[2439] && s.b[2467]) && s.b[2468]) {
            s.store_div_from_scalar_offset_ad(2367, 2.0, A::sqrt(A::scale_offset(s.ad_value(2366), 4.0, 1.0)), 1.0);
            s.store_mul(1919, 2367, 2366);
            s.store_mul_ad_product_rhs(2368, 2365, s.ad_value(2367), A::offset(A::div(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1919), 1.0, A::mul(s.ad_value(1919), s.ad_value(2367)), 0.86), A::offset(A::mul3_scaled_output(s.ad_value(1919), s.ad_value(1919), s.ad_value(2367), 4.0), 1.0)), 1.0));
            s.store_scale(2369, 2368, 0.99);
            s.store_ad_value(1919, A::div_scaled_product3(s.ad_value(2369), A::sub_scaled_inputs(s.ad_value(2369), 1.0, s.ad_value(2355), 2.0), s.ad_value(2307), 1.0, s.ad_value(2336), 1.0));
        }

    }

    pub(super) fn stamp_transient_block_36(
        s: &mut Scratch,
    ) {
        if ((s.b[2439] && s.b[2467]) && s.b[2468]) {
            s.store_mul_sub_ad_rhs(2370, 2305, s.ad_value(2369), A::ln(A::offset({
                if (s.v[1919] > (-0.99)) {
                    s.ad_value(1919)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0)));
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2468])) {
            s.copy_ad(2370, 2353);
        }

        if (s.b[2439] && s.b[2467]) {
            s.store_offset(1919, 2286, 1.0);
            s.store_ad_value(1920, A::div_scaled_product(A::sqrt(s.ad_value(1919)), s.ad_value(820), 1.0, s.ad_value(2370), 1.0));
            s.store_add_ad_lhs(1921, A::square(s.ad_value(1920)), 1919);
            s.store_scale(1919, 1920, 2.0);
            s.store_ad_value(2371, A::div_scaled_product(s.ad_value(2370), s.ad_value(1919), 1.0, A::add(A::sqrt(A::sub(s.ad_value(1921), s.ad_value(1919))), A::sqrt(A::add(s.ad_value(1921), s.ad_value(1919)))), 1.0));
            s.store_mul(2372, 2371, 2306);
            s.store_add(2373, 2315, 2372);
        }

        s.b[2474] = (s.v[2372] < 460.51701859880916);
        s.v[2474] = if s.b[2474] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2467]) && s.b[2474]) {
            s.store_exp_neg_input(2374, 2372);
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2474])) {
            s.store_div_from_scalar_offset_ad(2374, 1e-200, A::mul_offset_lhs(s.ad_value(2372), (-460.51701859880916), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2372), (-460.51701859880916), A::scale_offset(s.ad_value(2372), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        if (s.b[2439] && s.b[2467]) {
            s.store_mul(2375, 2330, 2374);
        }

        s.b[2475] = (((s.v[2309]) as f64).abs() <= s.v[2327]);
        s.v[2475] = if s.b[2475] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2467]) && s.b[2475]) {
            s.store_scaled_square(2415, 2328, (0.16666666666666666 * 0.7071067811865475));
            s.store_mul_ad_product_rhs(2376, 2309, s.ad_value(2328), A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2309), 1.0, s.ad_value(2375)), s.ad_value(2290), s.ad_value(2415)), 1.0));
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2475])) {
            s.store_offset(2436, 2373, 3.0);
            s.store_ad_value(2419, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(2435), 0.5, s.ad_value(2436), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2435), s.ad_value(2436)), A::sub(s.ad_value(2435), s.ad_value(2436))), 5.0)), (-0.5)), 1.0, s.ad_value(2436), (-0.5), A::sqrt(A::offset(A::square(s.ad_value(2436)), 5.0)), (-(-0.5))));
            s.store_sub(2414, 2309, 2419);
            s.store_exp_neg_input(2415, 2419);
            s.store_div_from_scalar_offset_ad(2416, 1.0, A::square(s.ad_value(2419)), 2.0);
            s.store_mul_square_lhs(2426, 2419, 2416);
            s.store_mul3_affine_lhs(2427, 2419, 2416, 4.0, 0.0, 2416);
            s.store_mul_ad_product_lhs(2428, A::sub_scaled_inputs(s.ad_value(2416), 8.0, s.ad_value(2426), 12.0), s.ad_value(2416), 2416);
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2475])) {
            let assign51910_ad_e66735: A = {
                if (1e-40 > ((s.v[2414] * s.v[2414]) - (s.v[2291] * (((s.v[2415] + s.v[2419]) - 1.0) - (s.v[2375] * ((s.v[2419] + 1.0) + s.v[2426])))))) {
                    A::constant(1e-40)
                } else {
                    A::add_scaled_square_product(s.ad_value(2414), 1.0, s.ad_value(2291), A::add_scaled_product(A::offset(A::add(s.ad_value(2415), s.ad_value(2419)), (-1.0)), 1.0, s.ad_value(2375), A::add(A::offset(s.ad_value(2419), 1.0), s.ad_value(2426)), (-1.0)), (-1.0))
                }
            };
            s.store_ad_value(2420, assign51910_ad_e66735);
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2475])) {
            s.store_sub_from_scalar_ad(2437, 1.0, A::mul_scaled_output(s.ad_value(2291), A::add_scaled_product(s.ad_value(2415), 1.0, s.ad_value(2375), s.ad_value(2428), (-1.0)), 0.5));
            s.store_ad_value(2421, A::add_scaled_product(s.ad_value(2414), 2.0, s.ad_value(2291), A::add_scaled_sub_value_product(1.0, s.ad_value(2415), 1.0, s.ad_value(2375), A::offset(s.ad_value(2427), 1.0), (-1.0)), 1.0));
            s.store_ad_value(2422, A::add_scaled_inputs3(s.ad_value(2373), 1.0, s.ad_value(2419), (-1.0), A::ln(A::div(s.ad_value(2420), s.ad_value(2291))), 1.0));
            s.store_add(818, 2420, 2421);
            s.store_ad_value(817, A::add_scaled_square_product(s.ad_value(818), 1.0, s.ad_value(2422), A::add_scaled_square_product(s.ad_value(2421), 0.5, s.ad_value(2420), s.ad_value(2437), (-1.0)), 1.0));
            s.store_add_ad_rhs(2438, 2419, A::div_scaled_product3(s.ad_value(2420), s.ad_value(818), s.ad_value(2422), 1.0, A::add(s.ad_value(817), A::mul3(A::mul3(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2422), s.ad_value(2422)), s.ad_value(2421), A::add_scaled_square_product(s.ad_value(2421), 0.3333333333333333, s.ad_value(2420), s.ad_value(2437), (-1.0)))), 1.0));
        }

        s.b[2476] = (s.v[2438] < 230.25850929940458);
        s.v[2476] = if s.b[2476] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2467]) && (!s.b[2475])) && s.b[2476]) {
            s.store_exp(2424, 2438);
            s.store_div_from_scalar(2425, 1.0, 2424);
            s.store_mul(2424, 2375, 2424);
        }

        s.b[2477] = (s.v[2438] > (s.v[2373] - 230.25850929940458));
        s.v[2477] = if s.b[2477] { 1.0 } else { 0.0 };

        if ((((s.b[2439] && s.b[2467]) && (!s.b[2475])) && (!s.b[2476])) && s.b[2477]) {
            s.store_exp_sub(2424, 2438, 2373);
            s.store_div(2425, 2375, 2424);
        }

        if ((((s.b[2439] && s.b[2467]) && (!s.b[2475])) && (!s.b[2476])) && (!s.b[2477])) {
            s.store_div_from_scalar_offset_ad(2424, 1e-100, A::mul_offset_lhs(A::sub(s.ad_value(2373), s.ad_value(2438)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2373), s.ad_value(2438)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2373), s.ad_value(2438)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(2425, 1e-100, A::mul_offset_lhs(s.ad_value(2438), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2438), (-230.25850929940458), A::scale_offset(s.ad_value(2438), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2475])) {
            s.store_div_from_scalar_offset_ad(2414, 1.0, A::square(s.ad_value(2438)), 2.0);
            s.store_mul_square_lhs(2426, 2438, 2414);
            s.store_mul3_affine_lhs(2427, 2438, 2414, 4.0, 0.0, 2414);
            s.store_mul_ad_product_lhs(2428, A::sub_scaled_inputs(s.ad_value(2414), 8.0, s.ad_value(2426), 12.0), s.ad_value(2414), 2414);
            s.store_sub(2414, 2309, 2438);
            s.store_ad_value(2429, A::add_scaled_product(s.ad_value(2414), 2.0, s.ad_value(2291), A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2425)), 1.0, s.ad_value(2424), 1.0, s.ad_value(2375), A::offset(s.ad_value(2427), 1.0), (-1.0)), 1.0));
            s.store_ad_value(2430, A::add_scaled_square_product(s.ad_value(2414), 1.0, s.ad_value(2291), A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2425), 1.0, s.ad_value(2438), 1.0, s.ad_value(2424), 1.0, (-1.0)), 1.0, s.ad_value(2375), A::add(A::offset(s.ad_value(2438), 1.0), s.ad_value(2426)), (-1.0)), (-1.0)));
            s.store_sub_from_scalar_ad(2414, 2.0, A::mul(s.ad_value(2291), A::add_scaled_inputs_product(s.ad_value(2425), 1.0, s.ad_value(2424), 1.0, s.ad_value(2375), s.ad_value(2428), (-1.0))));
            s.store_ad_value(2414, A::add_scaled_square_product(s.ad_value(2429), 1.0, s.ad_value(2430), s.ad_value(2414), (-2.0)));
            s.store_ad_value(2376, A::add_scaled_inputs(s.ad_value(2438), 1.0, A::div(s.ad_value(2430), A::add(s.ad_value(2429), A::sqrt(s.ad_value(2414)))), 2.0));
        }

        if (s.b[2439] && s.b[2467]) {
            s.store_sub(2377, 2376, 2329);
        }

        s.b[2478] = (s.v[2377] < 1e-10);
        s.v[2478] = if s.b[2478] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2467]) && s.b[2478]) {
            s.store_ad_value(2378, A::add_scaled_inputs_product(s.ad_value(2309), 2.0, s.ad_value(2329), (-2.0), s.ad_value(2291), A::add_scaled_offset_product_rhs(A::add_scaled_sub_value_product(1.0, s.ad_value(2335), 1.0, s.ad_value(2334), s.ad_value(2374), 1.0), 1.0, s.ad_value(2375), s.ad_value(2332), 1.0, (-1.0)), 1.0));
            s.store_mul_ad_lhs(2379, A::mul_sub_from_scalar_rhs(s.ad_value(2291), 1.0, s.ad_value(2374)), 2336);
            s.store_sub_from_scalar_ad(1919, 2.0, A::mul(s.ad_value(2291), A::add_scaled_value_products(s.ad_value(2335), 1.0, s.ad_value(2334), s.ad_value(2374), 1.0, s.ad_value(2375), s.ad_value(2333), (-1.0))));
            s.store_ad_value(1919, A::add_scaled_square_product(s.ad_value(2378), 1.0, s.ad_value(1919), s.ad_value(2379), (-2.0)));
            s.store_scaled_div_ad_rhs(2377, 2379, A::add(s.ad_value(2378), A::sqrt(s.ad_value(1919))), 2.0);
            s.store_add(2376, 2329, 2377);
        }

        if (s.b[2439] && s.b[2467]) {
            s.store_mul(2380, 2377, 2305);
            s.store_ad_value(2381, A::div_scaled_product_offset_denominator(s.ad_value(2376), s.ad_value(2376), 1.0, A::square(s.ad_value(2376)), 2.0, 1.0));
        }

        s.b[2479] = (s.v[2376] < 230.25850929940458);
        s.v[2479] = if s.b[2479] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2467]) && s.b[2479]) {
            s.store_exp_neg_input(2382, 2376);
        }

        s.b[2480] = (s.v[2376] < 1e-5);
        s.v[2480] = if s.b[2480] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2467]) && s.b[2479]) && s.b[2480]) {
            s.store_ad_value(2383, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(2376)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2376), 1.0, A::scale(s.ad_value(2376), 0.25), 0.3333333333333333), 0.5));
            s.store_sqrt_sub_from_scalar_ad(1919, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2376), 1.0, A::scale(s.ad_value(2376), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2384, 2376, 1919, 0.7071067811865475);
            s.store_ad_value(2385, A::mul3(A::mul3_scaled_output(s.ad_value(2375), s.ad_value(2376), s.ad_value(2376), 0.16666666666666666), s.ad_value(2376), A::scale_offset(s.ad_value(2376), 1.75, 1.0)));
        }

        if (((s.b[2439] && s.b[2467]) && s.b[2479]) && (!s.b[2480])) {
            s.store_add_ad_lhs(2383, A::offset(s.ad_value(2376), (-1.0)), 2382);
            s.store_sqrt(2384, 2383);
            s.store_mul_ad_rhs(2385, 2375, A::add_scaled_inputs3_offset(A::div_from_scalar(1.0, s.ad_value(2382)), 1.0, s.ad_value(2376), (-1.0), s.ad_value(2381), -1.0, (-1.0)));
        }

        s.b[2481] = (s.v[2376] > (s.v[2373] - 230.25850929940458));
        s.v[2481] = if s.b[2481] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2467]) && (!s.b[2479])) && s.b[2481]) {
            s.store_exp_sub(1919, 2376, 2373);
            s.store_div(2382, 2375, 1919);
            s.store_ad_value(2385, A::add_scaled_product(s.ad_value(1919), 1.0, s.ad_value(2375), A::add(A::offset(s.ad_value(2376), 1.0), s.ad_value(2381)), (-1.0)));
        }

        if (((s.b[2439] && s.b[2467]) && (!s.b[2479])) && (!s.b[2481])) {
            s.store_div_from_scalar_offset_ad(2382, 1e-100, A::mul_offset_lhs(s.ad_value(2376), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2376), (-230.25850929940458), A::scale_offset(s.ad_value(2376), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_offset_ad(1919, 1e-100, A::mul_offset_lhs(A::sub(s.ad_value(2373), s.ad_value(2376)), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::sub(s.ad_value(2373), s.ad_value(2376)), (-230.25850929940458), A::scale_offset(A::sub(s.ad_value(2373), s.ad_value(2376)), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_ad_value(2385, A::add_scaled_product(s.ad_value(1919), 1.0, s.ad_value(2375), A::add(A::offset(s.ad_value(2376), 1.0), s.ad_value(2381)), (-1.0)));
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2479])) {
            s.store_add_ad_lhs(2383, A::offset(s.ad_value(2376), (-1.0)), 2382);
            s.store_sqrt(2384, 2383);
        }

        if (s.b[2439] && s.b[2467]) {
            s.store_mul3_lhs(2386, 2384, 2290, 2305);
            s.store_scaled_add(2387, 2329, 2376, 0.5);
            s.store_scalar(2388, 0.0);
            s.store_mul(1919, 2382, 2335);
        }

        s.b[2482] = (s.v[1919] > 0.0);
        s.v[2482] = if s.b[2482] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2467]) && s.b[2482]) {
            s.store_sqrt(2388, 1919);
        }

        if (s.b[2439] && s.b[2467]) {
            s.store_scaled_add(2389, 2336, 2385, 0.5);
            s.store_ad_value(2390, A::add_scaled_product(s.ad_value(2389), 1.0, A::square(s.ad_value(2377)), A::sub_scaled_inputs(s.ad_value(2388), 1.0, s.ad_value(2307), 2.0), 0.125));
        }

        s.b[2483] = (s.v[2387] < 1e-5);
        s.v[2483] = if s.b[2483] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2467]) && s.b[2483]) {
            s.store_ad_value(2391, A::mul_sub_from_scalar_rhs_scaled_output(A::square(s.ad_value(2387)), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2387), 1.0, A::scale(s.ad_value(2387), 0.25), 0.3333333333333333), 0.5));
            s.store_mul_sqrt_ad_rhs(2392, 2290, A::add(s.ad_value(2390), s.ad_value(2391)));
        }

        s.b[2484] = (s.v[724] > 0.0);
        s.v[2484] = if s.b[2484] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2467]) && s.b[2483]) && s.b[2484]) {
            s.store_div_from_scalar_sqrt_ad(2393, 1.0, A::offset(A::mul(s.ad_value(724), s.ad_value(2392)), 1.0));
        }

        if ((s.b[2439] && s.b[2467]) && s.b[2483]) {
            s.store_sqrt_sub_from_scalar_ad(1919, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2387), 1.0, A::scale(s.ad_value(2387), 0.25), 0.3333333333333333));
            s.store_scaled_mul(2394, 2387, 1919, 0.7071067811865475);
            s.store_add_ad_rhs(2395, 2393, A::div_scaled_product(s.ad_value(2290), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2387), 0.5)), 1.0, A::square(s.ad_value(2387)), 0.16666666666666666), 0.7071067811865475, s.ad_value(1919), 1.0));
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2483])) {
            s.store_add_ad_lhs(2391, A::offset(s.ad_value(2387), (-1.0)), 2388);
            s.store_mul_sqrt_ad_rhs(2392, 2290, A::add(s.ad_value(2390), s.ad_value(2391)));
        }

        s.b[2485] = (s.v[724] > 0.0);
        s.v[2485] = if s.b[2485] { 1.0 } else { 0.0 };

        if (((s.b[2439] && s.b[2467]) && (!s.b[2483])) && s.b[2485]) {
            s.store_ad_value(2396, A::add_scaled_sub_value_product(1.0, s.ad_value(2388), 1.0, s.ad_value(2392), s.ad_value(2307), 2.0));
            s.store_div_from_scalar_sqrt_ad(2393, 1.0, A::offset(A::mul(s.ad_value(724), s.ad_value(2392)), 1.0));
            s.store_div_ad_rhs(1919, 2393, A::offset(s.ad_value(2393), 1.0));
            s.store_mul_ad_rhs(2397, 724, A::mul3(A::square(s.ad_value(1919)), s.ad_value(2291), s.ad_value(2390)));
            s.store_ad_value(2398, A::add_scaled_inputs_product(s.ad_value(2392), 2.0, s.ad_value(2397), (-2.0), s.ad_value(2291), A::add(A::sub_from_scalar(1.0, s.ad_value(2388)), s.ad_value(2390)), 1.0));
            s.store_mul_ad_rhs(2399, 2397, A::sub_scaled_inputs(s.ad_value(2397), 1.0, s.ad_value(2392), 2.0));
            s.store_sub_from_scalar_ad(2400, 1.0, A::mul_scaled_output(s.ad_value(2291), A::add(s.ad_value(2388), s.ad_value(2390)), 0.5));
            s.store_ad_value(2401, A::div_scaled_product(s.ad_value(2399), s.ad_value(2398), 1.0, A::add_scaled_square_product(s.ad_value(2398), 1.0, s.ad_value(2400), s.ad_value(2399), (-1.0)), 1.0));
            s.store_add(2387, 2387, 2401);
            s.store_exp(2402, 2401);
            s.store_div(2388, 2388, 2402);
            s.store_mul(2390, 2390, 2402);
            s.store_add_ad_lhs(2391, A::offset(s.ad_value(2387), (-1.0)), 2388);
            s.store_mul_sqrt_ad_rhs(2392, 2290, A::add(s.ad_value(2390), s.ad_value(2391)));
            s.store_add_ad(2403, A::sub_from_scalar(1.0, s.ad_value(2388)), A::mul3_scaled_output(s.ad_value(2392), s.ad_value(2393), s.ad_value(2307), 2.0));
            s.store_ad_value(2377, A::div_scaled_product3(s.ad_value(2377), s.ad_value(2402), A::add(s.ad_value(2396), s.ad_value(2389)), 1.0, A::add_scaled_product(s.ad_value(2403), 1.0, s.ad_value(2402), s.ad_value(2389), 1.0), 1.0));
            s.store_mul(2380, 2377, 2305);
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2483])) {
            s.store_sqrt(2394, 2391);
            s.store_ad_value(2395, A::add_scaled_inputs(s.ad_value(2393), 1.0, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2290), 1.0, s.ad_value(2388)), s.ad_value(2394)), 0.5));
        }

        if (s.b[2439] && s.b[2467]) {
            s.store_mul_ad_rhs(2404, 2305, A::div_scaled_product(s.ad_value(2291), s.ad_value(2390), 1.0, A::add_scaled_product(s.ad_value(2392), 1.0, s.ad_value(2290), s.ad_value(2394), 1.0), 1.0));
            s.store_ad_value(2405, A::add_scaled_product(s.ad_value(2404), 1.0, s.ad_value(2305), s.ad_value(2395), 1.0));
            s.store_mul3_lhs(2406, 2394, 2290, 2305);
        }

        s.b[2486] = (s.v[213] < 0.0);
        s.v[2486] = if s.b[2486] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2467]) && s.b[2486]) {
            s.store_sub_from_scalar_ad(2345, 1.0, A::mul(s.ad_value(213), s.ad_value(2404)));
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2486])) {
            s.store_div_from_scalar_offset_ad(2345, 1.0, A::mul(s.ad_value(213), s.ad_value(2404)), 1.0);
        }

        if (s.b[2439] && s.b[2467]) {
            s.store_mul_ad_lhs(2346, A::mul3(s.ad_value(751), s.ad_value(2344), s.ad_value(2345)), 2404);
            s.store_ad_value(2407, A::add_scaled_product(s.ad_value(2406), 1.0, s.ad_value(769), s.ad_value(2404), 1.0));
            s.store_ad_value(2408, A::add_scaled_product(s.ad_value(2406), 1.0, s.ad_value(770), s.ad_value(2404), 1.0));
            s.store_mul(2409, 768, 2407);
            s.store_ln_ad(1920, A::div(s.ad_value(2391), A::offset(A::add(s.ad_value(2391), s.ad_value(2390)), 1e-14)));
        }

    }

    pub(super) fn stamp_transient_block_37(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[2439] && s.b[2467]) {
            s.store_ad_value(2348, A::add_scaled_product(A::pow(A::mul(s.ad_value(2409), s.ad_value(698)), s.ad_value(699)), 1.0, s.ad_value(700), A::exp(A::mul_scaled_lhs(s.ad_value(701), 0.5, s.ad_value(1920))), 1.0));
            s.store_mul_add_ad_lhs(2410, A::offset(s.ad_value(2348), 1.0), s.ad_value(2346), 2340);
            s.store_ln_ad(2411, A::div(A::offset(A::mul(A::sub(s.ad_value(820), s.ad_value(2380)), s.ad_value(773)), 1.0), A::offset(A::mul(A::sub(s.ad_value(2371), s.ad_value(2380)), s.ad_value(773)), 1.0)));
            s.store_mul(1921, 2404, 2350);
            s.store_div_ad_rhs(2351, 1921, A::add(s.ad_value(218), s.ad_value(1921)));
        }

        s.b[2487] = (s.v[217] < 0.0);
        s.v[2487] = if s.b[2487] { 1.0 } else { 0.0 };

        if ((s.b[2439] && s.b[2467]) && s.b[2487]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2352, 1.0, 1.0, A::mul(s.ad_value(217), s.ad_value(2351)));
        }

        if ((s.b[2439] && s.b[2467]) && (!s.b[2487])) {
            s.store_offset_mul(2352, 217, 2351, 1.0);
        }

        if (s.b[2439] && s.b[2467]) {
            s.store_mul(2413, 2285, 2352);
            s.store_mul(2412, 2392, 2305);
        }

        if s.b[2439] {
            s.copy_ad(1875, 2287);
            s.copy_ad(1876, 2305);
            s.copy_ad(1877, 2290);
            s.copy_ad(1878, 2309);
            s.copy_ad(1879, 2314);
            s.copy_ad(1880, 2343);
            s.copy_ad(1881, 2380);
            s.copy_ad(1882, 2386);
            s.copy_ad(1883, 2393);
            s.copy_ad(1884, 2395);
            s.copy_ad(1885, 2404);
            s.copy_ad(1886, 2405);
            s.copy_ad(1887, 2408);
            s.copy_ad(1888, 2410);
            s.copy_ad(1889, 2411);
            s.copy_ad(1890, 2413);
            s.copy_ad(1891, 2412);
        }

        if (!s.b[2439]) {
            s.copy_ad(739, 722);
            s.copy_ad(1875, 1810);
            s.copy_ad(1876, 1812);
            s.copy_ad(1877, 1814);
            s.copy_ad(1878, 1817);
            s.copy_ad(1879, 1818);
            s.copy_ad(1880, 1837);
            s.copy_ad(1881, 1848);
            s.copy_ad(1882, 1849);
            s.copy_ad(1883, 1851);
            s.copy_ad(1884, 1852);
            s.copy_ad(1885, 1853);
            s.copy_ad(1886, 1854);
            s.copy_ad(1887, 1856);
            s.copy_ad(1888, 1857);
            s.copy_ad(1889, 1859);
            s.copy_ad(1890, 1858);
            s.copy_ad(1891, 1860);
        }

        s.copy_ad(1892, 250);

        s.b[2488] = (s.v[767] > 0.0);
        s.v[2488] = if s.b[2488] { 1.0 } else { 0.0 };

        if s.b[2488] {
            s.store_div_ad_rhs(1892, 250, A::offset(A::mul(s.ad_value(767), A::powf(A::offset(A::square(s.ad_value(1887)), s.v[727]), ((-1.0) * 0.16666666666666666))), 1.0));
        }

        s.v[1893] = 1.0;

        s.v[1894] = 1.0;

        s.v[1895] = 0.0;

        s.v[1896] = 1.0;

        s.v[1897] = 1.0;

        s.copy_ad(2251, 1891);

        s.v[2254] = 0.0;

        s.v[2253] = 0.0;

        s.copy_ad(2255, 2251);

        s.b[2489] = (s.v[1878] > 0.0);
        s.v[2489] = if s.b[2489] { 1.0 } else { 0.0 };

        if s.b[2489] {
            s.store_mul_ad_lhs(2246, A::div_scaled_product(A::add(s.ad_value(255), A::div(s.ad_value(256), s.ad_value(1886))), s.ad_value(1885), 1.0, s.ad_value(1886), 1.0), 1889);
        }

        s.b[2490] = (s.v[2246] > 0.0);
        s.v[2490] = if s.b[2490] { 1.0 } else { 0.0 };

        if (s.b[2489] && s.b[2490]) {
            s.store_div_from_scalar_add_ad(1893, 1.0, A::offset(s.ad_value(2246), 1.0), A::square(s.ad_value(2246)));
        }

        if (s.b[2489] && (!s.b[2490])) {
            s.store_sub_from_scalar(1893, 1.0, 2246);
        }

        if s.b[2489] {
            s.store_mul(1894, 1888, 1893);
            s.store_div(1895, 1890, 1894);
            s.store_mul_ad_product_lhs(2247, A::square(s.ad_value(1895)), s.ad_value(1881), 1881);
        }

        s.b[2491] = (s.v[0] == (-1.0));
        s.v[2491] = if s.b[2491] { 1.0 } else { 0.0 };

        if (s.b[2489] && s.b[2491]) {
            s.store_div_ad_rhs(2247, 2247, A::offset(A::mul(s.ad_value(1895), s.ad_value(1881)), 1.0));
        }

        if s.b[2489] {
            s.store_ad_value(1896, A::mul_offset_rhs_scaled_output(s.ad_value(1894), A::sqrt(A::scale_offset(s.ad_value(2247), 2.0, 1.0)), 1.0, 0.5));
            s.store_div(1919, 1894, 1896);
            s.store_mul_offset_ad_rhs(2248, 1884, A::mul3_scaled_output(s.ad_value(2247), s.ad_value(1919), s.ad_value(1919), 0.5), 1.0);
            s.store_ad_value(1897, A::div_scaled_product(s.ad_value(1919), s.ad_value(1886), 1.0, s.ad_value(2248), 1.0));
            s.store_scaled_div(2249, 1881, 1897, 0.5);
            s.store_square(2250, 2249);
            s.store_add_ad_rhs(2251, 1891, A::mul3_scaled_output(s.ad_value(1883), s.ad_value(1881), A::add(A::offset(A::mul_scaled_output(s.ad_value(2249), s.ad_value(1893), 0.3333333333333333), (-1.0)), s.ad_value(1893)), 0.5));
            s.store_scaled_mul(1919, 1884, 1881, 0.16666666666666666);
        }

        s.b[2492] = (p.p49 == 1.0);
        s.v[2492] = if s.b[2492] { 1.0 } else { 0.0 };

        if (s.b[2489] && s.b[2492]) {
            s.store_scalar(2252, 0.0);
            s.store_mul_ad_affine_product_rhs(2253, 1893, s.ad_value(1893), A::sub(s.ad_value(1885), A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1919), 2.0, s.ad_value(2249), 3.0)), 0.5, 0.0);
        }

        if (s.b[2489] && (!s.b[2492])) {
            s.store_ad_value(2252, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(1893), A::add_scaled_product(s.ad_value(1885), 1.0, s.ad_value(1884), s.ad_value(1881), (-0.5))));
            s.store_ad_value(2253, A::add_scaled_products(A::square(s.ad_value(1893)), A::add_scaled_product(s.ad_value(1885), 1.0, s.ad_value(1919), A::sub_scaled_inputs(A::sub_from_scalar(1.0, s.ad_value(2249)), 1.0, s.ad_value(2250), 0.2), (-1.0)), 0.5, s.ad_value(2252), A::offset(s.ad_value(1893), 1.0), 0.5));
        }

        if s.b[2489] {
            s.store_ad_value(2254, A::add_scaled_product(s.ad_value(2252), 1.0, s.ad_value(1893), A::add_scaled_product(s.ad_value(1885), 1.0, s.ad_value(1919), s.ad_value(2249), 1.0), 1.0));
            s.store_sub(2255, 2251, 2254);
        }

        s.store_mul(845, 2251, 1892);

        s.store_mul_neg_lhs(847, 2253, 1892);

        s.store_mul_neg_lhs(846, 2255, 1892);

        s.v[2271] = 0.0;

        s.v[2272] = 0.0;

        s.v[2270] = 0.0;

        s.b[2493] = ((s.v[263] > 0.0) || (s.v[264] > 0.0));
        s.v[2493] = if s.b[2493] { 1.0 } else { 0.0 };

        if s.b[2493] {
            s.store_scalar(2260, 1.0);
            s.copy_ad(2259, 1875);
        }

        s.b[2494] = (s.v[267] > 1e-10);
        s.v[2494] = if s.b[2494] { 1.0 } else { 0.0 };

        if (s.b[2493] && s.b[2494]) {
            s.store_ad_value(2256, A::add_scaled_inputs3(s.ad_value(1875), 1.0, s.ad_value(265), (-1.0), s.ad_value(802), 1.0));
            s.store_ad_value(1919, A::add_scaled_inputs3(s.ad_value(2256), 0.5, s.ad_value(802), 0.5, A::sqrt(A::add_scaled_product(s.ad_value(803), 1.0, A::sub(s.ad_value(2256), s.ad_value(802)), A::sub(s.ad_value(2256), s.ad_value(802)), 1.0)), 0.5));
            s.store_mul_ad_rhs(1920, 1919, A::add_scaled_inputs3(s.ad_value(1919), 2.0, s.ad_value(802), (-1.0), s.ad_value(2256), -1.0));
            s.store_div(1921, 802, 1919);
            s.store_mul(2257, 2256, 1921);
            s.store_sqrt_sub_from_scalar_ad(2258, 1.0, A::mul(s.ad_value(2257), s.ad_value(267)));
            s.store_ad_value(2259, A::add_scaled_inputs3(A::div(A::sub_from_scalar(1.0, s.ad_value(2258)), s.ad_value(267)), 1.0, s.ad_value(2256), 1.0, s.ad_value(2257), -1.0));
            s.store_offset_ad(2260, A::div_scaled_product3(A::offset(A::div_from_scalar(0.5, s.ad_value(2258)), (-1.0)), A::add_scaled_product(s.ad_value(1920), 1.0, s.ad_value(2256), A::sub(s.ad_value(802), s.ad_value(1919)), 1.0), s.ad_value(1921), 1.0, s.ad_value(1920), 1.0), 1.0);
        }

        if s.b[2493] {
            s.store_scalar(2262, 1.0);
            s.store_scalar(2263, 0.0);
        }

        s.b[2495] = (s.v[266] > 0.0);
        s.v[2495] = if s.b[2495] { 1.0 } else { 0.0 };

        if (s.b[2493] && s.b[2495]) {
            s.store_ad_value(1919, A::add_scaled_product(s.ad_value(739), 0.5, s.ad_value(1876), A::scale_offset(s.ad_value(1877), 0.7071067811865475, 1.0), 1.0));
            s.store_div(2261, 1875, 1919);
        }

        s.b[2496] = (((s.v[2261]) as f64).abs() < 230.25850929940458);
        s.v[2496] = if s.b[2496] { 1.0 } else { 0.0 };

        if ((s.b[2493] && s.b[2495]) && s.b[2496]) {
            s.store_div_from_scalar_offset_ad(2262, 1.0, A::exp_scaled_input(s.ad_value(2261), -1.0), 1.0);
        }

        s.b[2497] = (s.v[2261] < 0.0);
        s.v[2497] = if s.b[2497] { 1.0 } else { 0.0 };

        if (((s.b[2493] && s.b[2495]) && (!s.b[2496])) && s.b[2497]) {
            s.store_div_from_scalar_offset_ad(2262, 1e-100, A::mul_offset_lhs(s.ad_value(2261), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(s.ad_value(2261), (-230.25850929940458), A::scale_offset(s.ad_value(2261), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
        }

        s.b[2498] = (s.v[2261] < 230.25850929940458);
        s.v[2498] = if s.b[2498] { 1.0 } else { 0.0 };

        if ((s.b[2493] && s.b[2495]) && s.b[2498]) {
            s.store_ln_one_plus_exp(1920, 2261);
        }

        if ((s.b[2493] && s.b[2495]) && (!s.b[2498])) {
            s.copy_ad(1920, 2261);
        }

        if (s.b[2493] && s.b[2495]) {
            s.store_mul(2263, 1919, 1920);
        }

        if s.b[2493] {
            s.store_ad_value(2264, A::add_scaled_product(s.ad_value(2260), 1.0, s.ad_value(266), A::sub(s.ad_value(2262), s.ad_value(2260)), 1.0));
            s.store_ad_value(2265, A::add_scaled_product(s.ad_value(2259), 1.0, s.ad_value(266), A::sub(s.ad_value(2263), s.ad_value(2259)), 1.0));
            s.store_ad_value(2266, A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(1875), 1.0, s.ad_value(1876), s.ad_value(1879), (-1.0)), 1.0, s.ad_value(1891), (-1.0), s.ad_value(1881), (-0.5)));
            s.store_ad_value(2267, A::add_scaled_inputs3(s.ad_value(1875), 1.0, s.ad_value(2266), (-1.0), s.ad_value(1880), -1.0));
            s.store_ad_value(2268, A::add_scaled_inputs3(s.ad_value(1881), 1.0, s.ad_value(2266), 1.0, s.ad_value(820), -1.0));
            s.store_ad_value(2269, A::add_scaled_inputs3(s.ad_value(1875), 1.0, s.ad_value(2268), (-1.0), s.ad_value(1882), -1.0));
        }

        s.b[2499] = (s.v[825] > 0.0);
        s.v[2499] = if s.b[2499] { 1.0 } else { 0.0 };

        if (s.b[2493] && s.b[2499]) {
            s.store_mul_ad_rhs(2270, 2264, A::add_scaled_products(s.ad_value(264), s.ad_value(2268), 1.0, s.ad_value(263), s.ad_value(2266), 1.0));
            s.store_mul_sub_rhs(2271, 263, 2267, 2265);
            s.store_mul_sub_rhs(2272, 264, 2269, 2265);
        }

        if (s.b[2493] && (!s.b[2499])) {
            s.store_mul_ad_rhs(2270, 2264, A::add_scaled_products(s.ad_value(263), s.ad_value(2268), 1.0, s.ad_value(264), s.ad_value(2266), 1.0));
            s.store_mul_sub_rhs(2271, 264, 2267, 2265);
            s.store_mul_sub_rhs(2272, 263, 2269, 2265);
        }

        if s.b[2493] {
            s.store_add(845, 845, 2270);
            s.store_add(847, 847, 2272);
            s.store_sub_ad_lhs(846, A::add_scaled_inputs3(s.ad_value(846), 1.0, s.ad_value(2270), (-1.0), s.ad_value(2272), -1.0), 2271);
        }

        s.store_mul(1898, 257, 1866);

        s.store_mul(1899, 258, 1867);

        s.v[2275] = 0.0;

        s.v[2273] = 0.0;

        s.b[2500] = ((s.v[257] > 0.0) && (s.v[259] > 0.0));
        s.v[2500] = if s.b[2500] { 1.0 } else { 0.0 };

        if s.b[2500] {
            s.store_mul_ad_rhs(1919, 261, A::add_scaled_inputs(s.ad_value(1807), 0.5, s.ad_value(781), 1.0));
        }

        s.b[2501] = (s.v[1919] < 230.25850929940458);
        s.v[2501] = if s.b[2501] { 1.0 } else { 0.0 };

        s.b[2502] = (s.v[1919] > (-230.25850929940458));
        s.v[2502] = if s.b[2502] { 1.0 } else { 0.0 };

        if ((s.b[2500] && s.b[2501]) && s.b[2502]) {
            s.store_exp(2273, 1919);
        }

        if ((s.b[2500] && s.b[2501]) && (!s.b[2502])) {
            s.store_div_from_scalar_offset_ad(2273, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1919), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1919), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        s.b[2503] = (s.v[2273] > 1e-10);
        s.v[2503] = if s.b[2503] { 1.0 } else { 0.0 };

        if ((s.b[2500] && s.b[2501]) && s.b[2503]) {
            s.store_ln_offset_input(2274, 2273, 1.0);
            s.store_mul_sub_from_scalar_ad_rhs(1920, 2274, 1.0, A::div(A::ln(A::offset(s.ad_value(2274), 1.0)), A::offset(s.ad_value(2274), 2.0)));
        }

    }

    pub(super) fn stamp_transient_block_38(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[2500] && s.b[2501]) && (!s.b[2503])) {
            s.copy_ad(2274, 2273);
            s.store_ad_value(1920, A::div_scaled_inputs(s.ad_value(2274), 2.0, A::offset(s.ad_value(2274), 2.0), 1.0));
        }

        if (s.b[2500] && (!s.b[2501])) {
            s.copy_ad(2274, 1919);
            s.store_mul_sub_from_scalar_ad_rhs(1920, 2274, 1.0, A::div(A::ln(A::offset(s.ad_value(2274), 1.0)), A::offset(s.ad_value(2274), 2.0)));
        }

        if s.b[2500] {
            s.store_mul_ad_affine_product_lhs(2275, A::div_scaled_inputs(s.ad_value(259), (-2.0), s.ad_value(261), 1.0), s.ad_value(257), s.v[348], 0.0, 1920);
        }

        s.v[2278] = 0.0;

        s.v[2276] = 0.0;

        s.b[2504] = ((s.v[258] > 0.0) && (s.v[260] > 0.0));
        s.v[2504] = if s.b[2504] { 1.0 } else { 0.0 };

        if s.b[2504] {
            s.store_mul_ad_rhs(1919, 261, A::add_scaled_inputs(s.ad_value(1807), 0.5, s.ad_value(782), 1.0));
        }

        s.b[2505] = (s.v[1919] < 230.25850929940458);
        s.v[2505] = if s.b[2505] { 1.0 } else { 0.0 };

        s.b[2506] = (s.v[1919] > (-230.25850929940458));
        s.v[2506] = if s.b[2506] { 1.0 } else { 0.0 };

        if ((s.b[2504] && s.b[2505]) && s.b[2506]) {
            s.store_exp(2276, 1919);
        }

        if ((s.b[2504] && s.b[2505]) && (!s.b[2506])) {
            s.store_div_from_scalar_offset_ad(2276, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(1919), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(1919), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        s.b[2507] = (s.v[2276] > 1e-10);
        s.v[2507] = if s.b[2507] { 1.0 } else { 0.0 };

        if ((s.b[2504] && s.b[2505]) && s.b[2507]) {
            s.store_ln_offset_input(2277, 2276, 1.0);
            s.store_mul_sub_from_scalar_ad_rhs(1920, 2277, 1.0, A::div(A::ln(A::offset(s.ad_value(2277), 1.0)), A::offset(s.ad_value(2277), 2.0)));
        }

        if ((s.b[2504] && s.b[2505]) && (!s.b[2507])) {
            s.copy_ad(2277, 2276);
            s.store_ad_value(1920, A::div_scaled_inputs(s.ad_value(2277), 2.0, A::offset(s.ad_value(2277), 2.0), 1.0));
        }

        if (s.b[2504] && (!s.b[2505])) {
            s.copy_ad(2277, 1919);
            s.store_mul_sub_from_scalar_ad_rhs(1920, 2277, 1.0, A::div(A::ln(A::offset(s.ad_value(2277), 1.0)), A::offset(s.ad_value(2277), 2.0)));
        }

        if s.b[2504] {
            s.store_mul_ad_affine_product_lhs(2278, A::div_scaled_inputs(s.ad_value(260), (-2.0), s.ad_value(261), 1.0), s.ad_value(258), s.v[348], 0.0, 1920);
        }

        s.store_add(2279, 2275, 2278);

        s.store_ad_value(850, A::add_scaled_product(s.ad_value(2279), 1.0, s.ad_value(262), s.ad_value(823), 1.0));

        s.store_mul(848, 269, 828);

        s.store_mul(849, 270, 831);

        s.v[2508] = 0.0;

        s.v[2509] = 0.0;

        s.v[2510] = 0.0;

        s.v[2511] = 0.0;

        s.v[2512] = 0.0;

        s.v[2513] = 0.0;

        s.v[2514] = 0.0;

        s.v[2515] = 0.0;

        s.v[2516] = 0.0;

        s.v[2517] = 0.0;

        s.v[2518] = 0.0;

        s.v[2519] = 0.0;

        s.v[2520] = 0.0;

        s.v[2521] = 0.0;

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

        s.v[842] = 0.0;

        s.v[1900] = 0.0;

        s.v[1901] = 0.0;

        s.v[1902] = 0.0;

        s.v[843] = 0.0;

        s.v[1903] = 0.0;

        s.v[1904] = 0.0;

        s.v[1905] = 0.0;

        s.v[851] = 0.0;

        s.v[1906] = 0.0;

        s.v[1907] = 0.0;

        s.v[1908] = 0.0;

        s.v[852] = 0.0;

        s.v[1909] = 0.0;

        s.v[1910] = 0.0;

        s.v[1911] = 0.0;

        s.b[2555] = (p.p43 > 0.0);
        s.v[2555] = if s.b[2555] { 1.0 } else { 0.0 };

        s.b[2556] = (s.v[468] == 1.0);
        s.v[2556] = if s.b[2556] { 1.0 } else { 0.0 };

        if (s.b[2555] && s.b[2556]) {
            s.store_scale(490, 826, (s.v[365] * s.v[662]));
        }

        if (s.b[2555] && s.b[2556]) {
            s.store_ad_value(491, {
                if (s.v[490] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(490)), 1.0))
                } else {
                    {
                        if (s.v[490] > s.v[654]) {
                            A::mul_offset_rhs(s.ad_value(655), A::sub(s.ad_value(490), s.ad_value(654)), 1.0)
                        } else {
                            A::exp(s.ad_value(490))
                        }
                    }
                }
            });
        }

        if (s.b[2555] && s.b[2556]) {
            s.store_mul_offset_rhs(496, 661, 491, (-1.0));
            s.store_scaled_mul(490, 826, 664, s.v[365]);
        }

        if (s.b[2555] && s.b[2556]) {
            s.store_ad_value(491, {
                if (s.v[490] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(490)), 1.0))
                } else {
                    {
                        if (s.v[490] > s.v[656]) {
                            A::mul_offset_rhs(s.ad_value(657), A::sub(s.ad_value(490), s.ad_value(656)), 1.0)
                        } else {
                            A::exp(s.ad_value(490))
                        }
                    }
                }
            });
        }

        if (s.b[2555] && s.b[2556]) {
            s.store_mul_offset_rhs(497, 663, 491, (-1.0));
            s.store_scalar(498, 0.0);
        }

        s.b[2557] = (s.v[660] > 0.0);
        s.v[2557] = if s.b[2557] { 1.0 } else { 0.0 };

        if ((s.b[2555] && s.b[2556]) && s.b[2557]) {
            s.store_mul_ad_rhs(498, 826, A::add_scaled_product(s.ad_value(665), 1.0, s.ad_value(826), s.ad_value(666), 1.0));
        }

        if ((s.b[2555] && s.b[2556]) && (!s.b[2557])) {
            s.store_scaled_mul(490, 826, 666, (-s.v[365]));
        }

        if ((s.b[2555] && s.b[2556]) && (!s.b[2557])) {
            s.store_ad_value(491, {
                if (s.v[490] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(490)), 1.0))
                } else {
                    {
                        if (s.v[490] > s.v[658]) {
                            A::mul_offset_rhs(s.ad_value(659), A::sub(s.ad_value(490), s.ad_value(658)), 1.0)
                        } else {
                            A::exp(s.ad_value(490))
                        }
                    }
                }
            });
        }

        if ((s.b[2555] && s.b[2556]) && (!s.b[2557])) {
            s.store_mul_scaled_ad_rhs(498, 665, -1.0, A::offset(s.ad_value(491), (-1.0)));
        }

        if (s.b[2555] && s.b[2556]) {
            s.store_ad_value(842, A::add_scaled_inputs3(s.ad_value(496), 1.0, s.ad_value(497), 1.0, s.ad_value(498), 1.0));
            s.store_scale(490, 827, (s.v[365] * s.v[689]));
        }

        if (s.b[2555] && s.b[2556]) {
            s.store_ad_value(491, {
                if (s.v[490] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(490)), 1.0))
                } else {
                    {
                        if (s.v[490] > s.v[681]) {
                            A::mul_offset_rhs(s.ad_value(682), A::sub(s.ad_value(490), s.ad_value(681)), 1.0)
                        } else {
                            A::exp(s.ad_value(490))
                        }
                    }
                }
            });
        }

        if (s.b[2555] && s.b[2556]) {
            s.store_mul_offset_rhs(496, 688, 491, (-1.0));
            s.store_scaled_mul(490, 827, 691, s.v[365]);
        }

        if (s.b[2555] && s.b[2556]) {
            s.store_ad_value(491, {
                if (s.v[490] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(490)), 1.0))
                } else {
                    {
                        if (s.v[490] > s.v[683]) {
                            A::mul_offset_rhs(s.ad_value(684), A::sub(s.ad_value(490), s.ad_value(683)), 1.0)
                        } else {
                            A::exp(s.ad_value(490))
                        }
                    }
                }
            });
        }

        if (s.b[2555] && s.b[2556]) {
            s.store_mul_offset_rhs(497, 690, 491, (-1.0));
            s.store_scalar(498, 0.0);
        }

        s.b[2558] = (s.v[687] > 0.0);
        s.v[2558] = if s.b[2558] { 1.0 } else { 0.0 };

        if ((s.b[2555] && s.b[2556]) && s.b[2558]) {
            s.store_mul_ad_rhs(498, 827, A::add_scaled_product(s.ad_value(692), 1.0, s.ad_value(827), s.ad_value(693), 1.0));
        }

        if ((s.b[2555] && s.b[2556]) && (!s.b[2558])) {
            s.store_scaled_mul(490, 827, 693, (-s.v[365]));
        }

        if ((s.b[2555] && s.b[2556]) && (!s.b[2558])) {
            s.store_ad_value(491, {
                if (s.v[490] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(490)), 1.0))
                } else {
                    {
                        if (s.v[490] > s.v[685]) {
                            A::mul_offset_rhs(s.ad_value(686), A::sub(s.ad_value(490), s.ad_value(685)), 1.0)
                        } else {
                            A::exp(s.ad_value(490))
                        }
                    }
                }
            });
        }

        if ((s.b[2555] && s.b[2556]) && (!s.b[2558])) {
            s.store_mul_scaled_ad_rhs(498, 692, -1.0, A::offset(s.ad_value(491), (-1.0)));
        }

        if (s.b[2555] && s.b[2556]) {
            s.store_ad_value(843, A::add_scaled_inputs3(s.ad_value(496), 1.0, s.ad_value(497), 1.0, s.ad_value(498), 1.0));
            s.store_scalar(2559, 0.0);
            s.store_scalar(2560, 0.0);
            s.store_scaled_mul(2511, 651, 651, 4.0);
            s.store_div(2512, 651, 652);
            s.store_ad_value(2513, A::add_scaled_product(s.ad_value(826), 1.0, s.ad_value(651), s.ad_value(2512), 1.0));
            s.store_add(2514, 652, 2513);
            s.store_sub(2515, 652, 2513);
            s.store_sqrt_square_add(2516, 2515, 2511);
            s.store_ad_value(2560, A::div_scaled_product(s.ad_value(826), s.ad_value(652), 2.0, A::add(s.ad_value(2514), s.ad_value(2516)), 1.0));
        }

        s.b[2561] = (s.v[645] > 0.5);
        s.v[2561] = if s.b[2561] { 1.0 } else { 0.0 };

        s.b[2562] = (s.v[402] == 0.5);
        s.v[2562] = if s.b[2562] { 1.0 } else { 0.0 };

        if (((s.b[2555] && s.b[2556]) && s.b[2561]) && s.b[2562]) {
            s.store_sqrt_sub_from_scalar_ad(2559, 1.0, A::scale(s.ad_value(2560), s.v[399]));
        }

        if (((s.b[2555] && s.b[2556]) && s.b[2561]) && (!s.b[2562])) {
            s.store_powf_ad(2559, A::sub_from_scalar(1.0, A::scale(s.ad_value(2560), s.v[399])), s.v[402]);
        }

        if ((s.b[2555] && s.b[2556]) && s.b[2561]) {
            s.store_ad_value(1906, A::add_scaled_inputs3_offset(s.ad_value(2559), (-s.v[411]), s.ad_value(826), s.v[414], s.ad_value(2560), (-s.v[414]), s.v[411]));
        }

        s.b[2563] = (s.v[646] > 0.5);
        s.v[2563] = if s.b[2563] { 1.0 } else { 0.0 };

        s.b[2564] = (s.v[403] == 0.5);
        s.v[2564] = if s.b[2564] { 1.0 } else { 0.0 };

        if (((s.b[2555] && s.b[2556]) && s.b[2563]) && s.b[2564]) {
            s.store_sqrt_sub_from_scalar_ad(2559, 1.0, A::scale(s.ad_value(2560), s.v[400]));
        }

        if (((s.b[2555] && s.b[2556]) && s.b[2563]) && (!s.b[2564])) {
            s.store_powf_ad(2559, A::sub_from_scalar(1.0, A::scale(s.ad_value(2560), s.v[400])), s.v[403]);
        }

        if ((s.b[2555] && s.b[2556]) && s.b[2563]) {
            s.store_ad_value(1907, A::add_scaled_inputs3_offset(s.ad_value(2559), (-s.v[412]), s.ad_value(826), s.v[415], s.ad_value(2560), (-s.v[415]), s.v[412]));
        }

        s.b[2565] = (s.v[647] > 0.5);
        s.v[2565] = if s.b[2565] { 1.0 } else { 0.0 };

        s.b[2566] = (s.v[404] == 0.5);
        s.v[2566] = if s.b[2566] { 1.0 } else { 0.0 };

        if (((s.b[2555] && s.b[2556]) && s.b[2565]) && s.b[2566]) {
            s.store_sqrt_sub_from_scalar_ad(2559, 1.0, A::scale(s.ad_value(2560), s.v[401]));
        }

        if (((s.b[2555] && s.b[2556]) && s.b[2565]) && (!s.b[2566])) {
            s.store_powf_ad(2559, A::sub_from_scalar(1.0, A::scale(s.ad_value(2560), s.v[401])), s.v[404]);
        }

        if ((s.b[2555] && s.b[2556]) && s.b[2565]) {
            s.store_ad_value(1908, A::add_scaled_inputs3_offset(s.ad_value(2559), (-s.v[413]), s.ad_value(826), s.v[416], s.ad_value(2560), (-s.v[416]), s.v[413]));
        }

        if (s.b[2555] && s.b[2556]) {
            s.store_scalar(2559, 0.0);
            s.store_scalar(2560, 0.0);
            s.store_scaled_mul(2511, 678, 678, 4.0);
            s.store_div(2512, 678, 679);
            s.store_ad_value(2513, A::add_scaled_product(s.ad_value(827), 1.0, s.ad_value(678), s.ad_value(2512), 1.0));
            s.store_add(2514, 679, 2513);
            s.store_sub(2515, 679, 2513);
            s.store_sqrt_square_add(2516, 2515, 2511);
        }

    }

    pub(super) fn stamp_transient_block_39(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[2555] && s.b[2556]) {
            s.store_ad_value(2560, A::div_scaled_product(s.ad_value(827), s.ad_value(679), 2.0, A::add(s.ad_value(2514), s.ad_value(2516)), 1.0));
        }

        s.b[2567] = (s.v[672] > 0.5);
        s.v[2567] = if s.b[2567] { 1.0 } else { 0.0 };

        s.b[2568] = (s.v[569] == 0.5);
        s.v[2568] = if s.b[2568] { 1.0 } else { 0.0 };

        if (((s.b[2555] && s.b[2556]) && s.b[2567]) && s.b[2568]) {
            s.store_sqrt_sub_from_scalar_ad(2559, 1.0, A::mul(s.ad_value(2560), s.ad_value(566)));
        }

        if (((s.b[2555] && s.b[2556]) && s.b[2567]) && (!s.b[2568])) {
            s.store_pow_ad(2559, A::sub_from_scalar(1.0, A::mul(s.ad_value(2560), s.ad_value(566))), s.ad_value(569));
        }

        if ((s.b[2555] && s.b[2556]) && s.b[2567]) {
            s.store_ad_value(1909, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(578), 1.0, s.ad_value(2559)), 1.0, s.ad_value(581), A::sub(s.ad_value(827), s.ad_value(2560)), 1.0));
        }

        s.b[2569] = (s.v[673] > 0.5);
        s.v[2569] = if s.b[2569] { 1.0 } else { 0.0 };

        s.b[2570] = (s.v[570] == 0.5);
        s.v[2570] = if s.b[2570] { 1.0 } else { 0.0 };

        if (((s.b[2555] && s.b[2556]) && s.b[2569]) && s.b[2570]) {
            s.store_sqrt_sub_from_scalar_ad(2559, 1.0, A::mul(s.ad_value(2560), s.ad_value(567)));
        }

        if (((s.b[2555] && s.b[2556]) && s.b[2569]) && (!s.b[2570])) {
            s.store_pow_ad(2559, A::sub_from_scalar(1.0, A::mul(s.ad_value(2560), s.ad_value(567))), s.ad_value(570));
        }

        if ((s.b[2555] && s.b[2556]) && s.b[2569]) {
            s.store_ad_value(1910, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(579), 1.0, s.ad_value(2559)), 1.0, s.ad_value(582), A::sub(s.ad_value(827), s.ad_value(2560)), 1.0));
        }

        s.b[2571] = (s.v[674] > 0.5);
        s.v[2571] = if s.b[2571] { 1.0 } else { 0.0 };

        s.b[2572] = (s.v[571] == 0.5);
        s.v[2572] = if s.b[2572] { 1.0 } else { 0.0 };

        if (((s.b[2555] && s.b[2556]) && s.b[2571]) && s.b[2572]) {
            s.store_sqrt_sub_from_scalar_ad(2559, 1.0, A::mul(s.ad_value(2560), s.ad_value(568)));
        }

        if (((s.b[2555] && s.b[2556]) && s.b[2571]) && (!s.b[2572])) {
            s.store_pow_ad(2559, A::sub_from_scalar(1.0, A::mul(s.ad_value(2560), s.ad_value(568))), s.ad_value(571));
        }

        if ((s.b[2555] && s.b[2556]) && s.b[2571]) {
            s.store_ad_value(1911, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(580), 1.0, s.ad_value(2559)), 1.0, s.ad_value(583), A::sub(s.ad_value(827), s.ad_value(2560)), 1.0));
        }

        s.b[2573] = (p.p865 > 0.0);
        s.v[2573] = if s.b[2573] { 1.0 } else { 0.0 };

        if ((s.b[2555] && (!s.b[2556])) && s.b[2573]) {
            s.store_scaled_offset_ad(636, A::powf(A::add_scaled_inputs3(s.ad_value(819), 0.5, s.ad_value(821), 0.5, A::sqrt(A::offset(A::mul(A::add(s.ad_value(819), s.ad_value(821)), A::add(s.ad_value(819), s.ad_value(821))), (0.001 * 0.001))), 0.5), p.p866), (-(((0.5 * 0.001)) as f64).powf(p.p866)), p.p865);
            s.store_offset(634, 636, p.p855);
            s.store_div_from_scalar(444, 1.0, 634);
            s.store_div_from_scalar_offset_scaled_input(447, s.v[447], 636, 1.0 / (p.p855), 1.0);
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2573])) {
            s.store_scalar(634, p.p855);
        }

        s.b[2574] = (p.p867 > 0.0);
        s.v[2574] = if s.b[2574] { 1.0 } else { 0.0 };

        if ((s.b[2555] && (!s.b[2556])) && s.b[2574]) {
            s.store_scaled_offset_ad(638, A::powf(A::add_scaled_inputs3(s.ad_value(819), 0.5, s.ad_value(821), 0.5, A::sqrt(A::offset(A::mul(A::add(s.ad_value(819), s.ad_value(821)), A::add(s.ad_value(819), s.ad_value(821))), (0.001 * 0.001))), 0.5), p.p868), (-(((0.5 * 0.001)) as f64).powf(p.p868)), p.p867);
            s.store_mul_offset_rhs(437, 437, 638, 1.0);
        }

        if (s.b[2555] && (!s.b[2556])) {
            s.store_scalar(2524, 0.0);
            s.store_scalar(2521, 0.0);
        }

        s.b[2575] = (!(((s.v[640] == 0.0) && (s.v[641] == 0.0)) && (s.v[642] == 0.0)));
        s.v[2575] = if s.b[2575] { 1.0 } else { 0.0 };

        if ((s.b[2555] && (!s.b[2556])) && s.b[2575]) {
            s.store_scaled_mul(2511, 651, 651, 4.0);
            s.store_div(2512, 651, 652);
            s.store_ad_value(2513, A::add_scaled_product(s.ad_value(826), 1.0, s.ad_value(651), s.ad_value(2512), 1.0));
            s.store_add(2514, 652, 2513);
            s.store_sub(2515, 652, 2513);
            s.store_sqrt_square_add(2516, 2515, 2511);
            s.store_ad_value(2518, A::div_scaled_product(s.ad_value(826), s.ad_value(652), 2.0, A::add(s.ad_value(2514), s.ad_value(2516)), 1.0));
        }

        s.b[2576] = (s.v[826] < s.v[648]);
        s.v[2576] = if s.b[2576] { 1.0 } else { 0.0 };

        s.b[2577] = (((((-0.5) * (s.v[826] * s.v[365]))) as f64).abs() < 230.25850929940458);
        s.v[2577] = if s.b[2577] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && s.b[2575]) && s.b[2576]) && s.b[2577]) {
            s.store_exp_scaled_input(2519, 826, (s.v[365] * (-0.5)));
        }

        s.b[2578] = (((-0.5) * (s.v[826] * s.v[365])) < 0.0);
        s.v[2578] = if s.b[2578] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && s.b[2575]) && s.b[2576]) && (!s.b[2577])) && s.b[2578]) {
            s.store_div_from_scalar_offset_ad(2519, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::scale(s.ad_value(826), (s.v[365] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(826), (s.v[365] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2555] && (!s.b[2556])) && s.b[2575]) && s.b[2576]) && (!s.b[2577])) && (!s.b[2578])) {
            s.store_scaled_offset_ad(2519, A::mul_offset_rhs(A::scale_offset(s.ad_value(826), (s.v[365] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(826), (s.v[365] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(826), (((s.v[365] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[2555] && (!s.b[2556])) && s.b[2575]) && s.b[2576]) {
            s.store_div_from_scalar(2520, 1.0, 2519);
            s.store_square(2517, 2520);
        }

        if (((s.b[2555] && (!s.b[2556])) && s.b[2575]) && (!s.b[2576])) {
            s.store_mul_offset_ad_lhs(2517, A::sub_scaled_inputs(s.ad_value(826), s.v[365], s.ad_value(648), s.v[365]), 1.0, 649);
            s.store_sqrt(2520, 2517);
            s.store_div_from_scalar(2519, 1.0, 2520);
        }

        if ((s.b[2555] && (!s.b[2556])) && s.b[2575]) {
            s.store_offset(2517, 2517, (-1.0));
        }

        s.b[2579] = (s.v[826] > 0.0);
        s.v[2579] = if s.b[2579] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && s.b[2575]) && s.b[2579]) {
            s.store_scaled_ln_ad(2521, A::add(A::offset(s.ad_value(2519), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2519), 1.0, A::offset(s.ad_value(2519), 3.0)))), (s.v[364] * 2.0));
        }

        if (((s.b[2555] && (!s.b[2556])) && s.b[2575]) && (!s.b[2579])) {
            s.store_sub_ad_lhs(2521, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(2520), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2520), 1.0, A::scale_offset(s.ad_value(2520), 3.0, 1.0))))), (s.v[364] * 2.0)), 826);
        }

        if ((s.b[2555] && (!s.b[2556])) && s.b[2575]) {
            s.store_sub(2522, 650, 2521);
            s.store_ad_value(2523, A::add_scaled_inputs3(s.ad_value(826), 0.5, s.ad_value(2522), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(826), s.ad_value(2522)), A::sub(s.ad_value(826), s.ad_value(2522))), ((4.0 * s.v[364]) * s.v[364]))), (-0.5)));
            s.store_ad_value(2524, A::add_scaled_inputs3(s.ad_value(826), 0.5, s.ad_value(653), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(826), s.ad_value(653)), A::sub(s.ad_value(826), s.ad_value(653))), ((4.0 * s.v[362]) * s.v[362]))), (-0.5)));
            s.store_scaled_sub_ad_rhs(2525, 826, A::sqrt(A::offset(A::mul(s.ad_value(826), s.ad_value(826)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[2580] = (s.v[640] == 0.0);
        s.v[2580] = if s.b[2580] { 1.0 } else { 0.0 };

        if ((s.b[2555] && (!s.b[2556])) && s.b[2580]) {
            s.store_scalar(1900, 0.0);
            s.store_scalar(1906, 0.0);
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) {
            s.store_scale(2527, 2517, s.v[381]);
        }

        s.b[2581] = ((p.p833 == 0.0) && (p.p838 == 0.0));
        s.v[2581] = if s.b[2581] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && s.b[2581]) {
            s.store_scalar(2528, 0.0);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2581])) {
            s.store_sub_from_scalar(2529, s.v[387], 2523);
            s.store_sub_from_scalar_ad(2530, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2521), s.ad_value(2529)))));
        }

        s.b[2582] = (p.p824 == 0.5);
        s.v[2582] = if s.b[2582] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2581])) && s.b[2582]) {
            s.store_scalar(2531, 0.0);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2581])) && (!s.b[2582])) {
            s.store_scaled_add_ad_lhs(2531, A::div_scaled_product(A::square(s.ad_value(2530)), A::ln(s.ad_value(2530)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2530)), 1.0), 2530, (1.0 - (2.0 * p.p824)));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2581])) {
            s.store_add(2532, 2530, 2531);
        }

        s.b[2583] = (p.p824 == 0.5);
        s.v[2583] = if s.b[2583] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2581])) && s.b[2583]) {
            s.store_sqrt_scaled_input(2526, 2529, s.v[423]);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2581])) && (!s.b[2583])) {
            s.store_powf_ad(2526, A::scale(s.ad_value(2529), s.v[423]), p.p824);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2581])) {
            s.store_scale(2533, 2526, s.v[417]);
            s.store_ad_value(2534, A::mul_offset_lhs_scaled_output(s.ad_value(2520), (-1.0), s.ad_value(2533), s.v[378]));
            s.store_scaled_mul(2528, 2534, 2532, p.p833);
        }

        s.b[2584] = (p.p838 == 0.0);
        s.v[2584] = if s.b[2584] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && s.b[2584]) {
            s.store_scalar(2535, 0.0);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) {
            s.store_scaled_div(2536, 2533, 2529, (s.v[402] * s.v[432]));
            s.store_div_from_scalar(2537, (0.666666666666667 * s.v[429]), 2536);
            s.store_square(2538, 2537);
            s.store_sqrt_ad(2539, A::div_scaled_product_offset_denominator(s.ad_value(2538), s.ad_value(2538), 1.0, A::square(s.ad_value(2538)), 1.0, 1.0));
            s.store_sqrt(2540, 2539);
            s.store_mul(2541, 2539, 2540);
        }

        s.b[2585] = (((-p.p824) * s.v[405]) == (-1.0));
        s.v[2585] = if s.b[2585] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) && s.b[2585]) {
            s.store_div_from_scalar_offset_ad(2542, 1.0, A::mul(s.ad_value(2536), s.ad_value(2541)), 1.0);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) && (!s.b[2585])) {
            s.store_powf_ad(2542, A::offset(A::mul(s.ad_value(2536), s.ad_value(2541)), 1.0), ((-p.p824) * s.v[405]));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) {
            s.store_ad_value(2543, A::div_scaled_product(s.ad_value(2532), s.ad_value(2542), 1.0, A::add(s.ad_value(2532), s.ad_value(2542)), 1.0));
            s.store_sqrt_scaled_ad(2544, A::div(s.ad_value(2536), s.ad_value(2540)), 0.375);
            s.store_ad_value(2545, A::add_scaled_product(s.ad_value(2539), (-1.0), s.ad_value(2537), s.ad_value(2540), 2.0));
            s.store_ad_value(2546, A::add_scaled_value_products(s.ad_value(2539), (-s.v[429]), s.ad_value(2537), s.ad_value(2540), s.v[429], s.ad_value(2536), s.ad_value(2541), 0.5));
            s.store_mul_offset_lhs(2547, 2545, (-1.0), 2544);
            s.store_square(2508, 2547);
        }

        s.b[2586] = (s.v[2547] > 0.0);
        s.v[2586] = if s.b[2586] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) && s.b[2586]) {
            s.store_div_from_scalar_offset_scaled_input(2509, 1.0, 2547, s.v[366], 1.0);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) && (!s.b[2586])) {
            s.store_div_from_scalar_sub_from_scalar_ad(2509, 1.0, 1.0, A::scale(s.ad_value(2547), s.v[366]));
        }

        s.b[2587] = (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458));
        s.v[2587] = if s.b[2587] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) && s.b[2587]) {
            s.store_exp_sub(2526, 2546, 2508);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) && (!s.b[2587])) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) {
            s.store_mul_ad_lhs(2510, A::add_scaled_inputs_product(s.ad_value(2509), 0.29214664, A::square(s.ad_value(2509)), s.v[367], A::square(s.ad_value(2509)), s.ad_value(2509), s.v[368]), 2526);
        }

        s.b[2588] = (s.v[2547] > 0.0);
        s.v[2588] = if s.b[2588] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) && s.b[2588]) {
            s.copy_ad(2548, 2510);
        }

        s.b[2589] = (s.v[2546] > (-230.25850929940458));
        s.v[2589] = if s.b[2589] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) && (!s.b[2588])) && s.b[2589]) {
            s.store_exp(2526, 2546);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) && (!s.b[2588])) && (!s.b[2589])) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2546), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2546), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) && (!s.b[2588])) {
            s.store_sub_scaled_inputs(2548, 2526, 2.0, 2510, 1.0);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) {
            s.store_scaled_div(2549, 2548, 2544, (s.v[429] * (1.772453850905516 * 0.5)));
            s.store_mul3_affine_lhs(2535, 2534, 2549, p.p838, 0.0, 2543);
        }

        s.b[2590] = (p.p844 == 0.0);
        s.v[2590] = if s.b[2590] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && s.b[2590]) {
            s.store_scalar(2550, 0.0);
        }

        s.b[2591] = (p.p824 == 0.5);
        s.v[2591] = if s.b[2591] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2590])) && s.b[2591]) {
            s.store_sqrt_scaled_ad(2526, A::sub_from_scalar(p.p821, s.ad_value(2524)), s.v[423]);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2590])) && (!s.b[2591])) {
            s.store_powf_ad(2526, A::scale_offset(s.ad_value(2524), (-s.v[423]), ((p.p821) * (s.v[423]))), p.p824);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2590])) {
            s.store_scaled_div_ad_lhs(2551, A::scale_offset(s.ad_value(2524), (-s.v[420]), ((p.p821) * (s.v[420]))), 2526, s.v[405]);
        }

        s.b[2592] = (((((-s.v[435]) / s.v[2551])) as f64).abs() < 230.25850929940458);
        s.v[2592] = if s.b[2592] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2590])) && s.b[2592]) {
            s.store_exp_ad(2526, A::div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(2551), 1.0));
        }

        s.b[2593] = (((-s.v[435]) / s.v[2551]) < 0.0);
        s.v[2593] = if s.b[2593] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2590])) && (!s.b[2592])) && s.b[2593]) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(2551), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(2551), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2590])) && (!s.b[2592])) && (!s.b[2593])) {
            let assign57000_ad_e72320: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(2551), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(2551), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(2551), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2526, assign57000_ad_e72320, 1e100);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2590])) {
            s.store_mul_scaled_ad_lhs(2550, A::mul3(s.ad_value(826), s.ad_value(2551), s.ad_value(2551)), 2526, p.p844);
        }

        s.b[2594] = (p.p853 > 1000.0);
        s.v[2594] = if s.b[2594] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && s.b[2594]) {
            s.store_scalar(2552, 1.0);
        }

        s.b[2595] = (s.v[2525] > ((-s.v[438]) * p.p853));
        s.v[2595] = if s.b[2595] { 1.0 } else { 0.0 };

        s.b[2596] = (p.p856 == 4.0);
        s.v[2596] = if s.b[2596] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2594])) && s.b[2595]) && s.b[2596]) {
            s.store_mul_scaled_ad_lhs(2526, A::mul3_scaled_output(s.ad_value(2525), s.ad_value(2525), s.ad_value(2525), ((s.v[442] * s.v[442]) * s.v[442])), 2525, s.v[442]);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2594])) && s.b[2595]) && (!s.b[2596])) {
            s.store_powf_ad(2526, A::abs_scaled_input(s.ad_value(2525), s.v[442]), p.p856);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2594])) && s.b[2595]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2552, 1.0, 1.0, s.ad_value(2526));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2594])) && (!s.b[2595])) {
            s.store_offset_scaled(2552, 2525, s.v[445], (((((s.v[438] * p.p853)) * (s.v[445]))) + (s.v[439])));
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) {
            s.store_mul_scale_ad_lhs(1900, A::add(A::add_scaled_inputs3(s.ad_value(2527), 1.0, s.ad_value(2528), 1.0, s.ad_value(2535), 1.0), s.ad_value(2550)), p.p29, 2552);
        }

        s.b[2597] = (s.v[402] == 0.5);
        s.v[2597] = if s.b[2597] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && s.b[2597]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::scale(s.ad_value(2518), s.v[399]));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2597])) {
            s.store_powf_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2518), s.v[399])), s.v[402]);
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) {
            s.store_ad_value(1906, A::add_scaled_inputs3_offset(s.ad_value(2526), ((-s.v[411]) * p.p30), s.ad_value(826), (s.v[414] * p.p30), s.ad_value(2518), ((-s.v[414]) * p.p30), (s.v[411] * p.p30)));
        }

        s.b[2598] = (s.v[641] == 0.0);
        s.v[2598] = if s.b[2598] { 1.0 } else { 0.0 };

        if ((s.b[2555] && (!s.b[2556])) && s.b[2598]) {
            s.store_scalar(1901, 0.0);
            s.store_scalar(1907, 0.0);
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) {
            s.store_scale(2527, 2517, s.v[382]);
        }

        s.b[2599] = ((p.p834 == 0.0) && (p.p839 == 0.0));
        s.v[2599] = if s.b[2599] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && s.b[2599]) {
            s.store_scalar(2528, 0.0);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2599])) {
            s.store_sub_from_scalar(2529, s.v[388], 2523);
        }

    }

    pub(super) fn stamp_transient_block_40(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2599])) {
            s.store_sub_from_scalar_ad(2530, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2521), s.ad_value(2529)))));
        }

        s.b[2600] = (p.p825 == 0.5);
        s.v[2600] = if s.b[2600] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2599])) && s.b[2600]) {
            s.store_scalar(2531, 0.0);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2599])) && (!s.b[2600])) {
            s.store_scaled_add_ad_lhs(2531, A::div_scaled_product(A::square(s.ad_value(2530)), A::ln(s.ad_value(2530)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2530)), 1.0), 2530, (1.0 - (2.0 * p.p825)));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2599])) {
            s.store_add(2532, 2530, 2531);
        }

        s.b[2601] = (p.p825 == 0.5);
        s.v[2601] = if s.b[2601] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2599])) && s.b[2601]) {
            s.store_sqrt_scaled_input(2526, 2529, s.v[424]);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2599])) && (!s.b[2601])) {
            s.store_powf_ad(2526, A::scale(s.ad_value(2529), s.v[424]), p.p825);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2599])) {
            s.store_scale(2533, 2526, s.v[418]);
            s.store_ad_value(2534, A::mul_offset_lhs_scaled_output(s.ad_value(2520), (-1.0), s.ad_value(2533), s.v[379]));
            s.store_scaled_mul(2528, 2534, 2532, p.p834);
        }

        s.b[2602] = (p.p839 == 0.0);
        s.v[2602] = if s.b[2602] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && s.b[2602]) {
            s.store_scalar(2535, 0.0);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) {
            s.store_scaled_div(2536, 2533, 2529, (s.v[403] * s.v[433]));
            s.store_div_from_scalar(2537, (0.666666666666667 * s.v[430]), 2536);
            s.store_square(2538, 2537);
            s.store_sqrt_ad(2539, A::div_scaled_product_offset_denominator(s.ad_value(2538), s.ad_value(2538), 1.0, A::square(s.ad_value(2538)), 1.0, 1.0));
            s.store_sqrt(2540, 2539);
            s.store_mul(2541, 2539, 2540);
        }

        s.b[2603] = (((-p.p825) * s.v[406]) == (-1.0));
        s.v[2603] = if s.b[2603] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) && s.b[2603]) {
            s.store_div_from_scalar_offset_ad(2542, 1.0, A::mul(s.ad_value(2536), s.ad_value(2541)), 1.0);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) && (!s.b[2603])) {
            s.store_powf_ad(2542, A::offset(A::mul(s.ad_value(2536), s.ad_value(2541)), 1.0), ((-p.p825) * s.v[406]));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) {
            s.store_ad_value(2543, A::div_scaled_product(s.ad_value(2532), s.ad_value(2542), 1.0, A::add(s.ad_value(2532), s.ad_value(2542)), 1.0));
            s.store_sqrt_scaled_ad(2544, A::div(s.ad_value(2536), s.ad_value(2540)), 0.375);
            s.store_ad_value(2545, A::add_scaled_product(s.ad_value(2539), (-1.0), s.ad_value(2537), s.ad_value(2540), 2.0));
            s.store_ad_value(2546, A::add_scaled_value_products(s.ad_value(2539), (-s.v[430]), s.ad_value(2537), s.ad_value(2540), s.v[430], s.ad_value(2536), s.ad_value(2541), 0.5));
            s.store_mul_offset_lhs(2547, 2545, (-1.0), 2544);
            s.store_square(2508, 2547);
        }

        s.b[2604] = (s.v[2547] > 0.0);
        s.v[2604] = if s.b[2604] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) && s.b[2604]) {
            s.store_div_from_scalar_offset_scaled_input(2509, 1.0, 2547, s.v[366], 1.0);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) && (!s.b[2604])) {
            s.store_div_from_scalar_sub_from_scalar_ad(2509, 1.0, 1.0, A::scale(s.ad_value(2547), s.v[366]));
        }

        s.b[2605] = (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458));
        s.v[2605] = if s.b[2605] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) && s.b[2605]) {
            s.store_exp_sub(2526, 2546, 2508);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) && (!s.b[2605])) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) {
            s.store_mul_ad_lhs(2510, A::add_scaled_inputs_product(s.ad_value(2509), 0.29214664, A::square(s.ad_value(2509)), s.v[367], A::square(s.ad_value(2509)), s.ad_value(2509), s.v[368]), 2526);
        }

        s.b[2606] = (s.v[2547] > 0.0);
        s.v[2606] = if s.b[2606] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) && s.b[2606]) {
            s.copy_ad(2548, 2510);
        }

        s.b[2607] = (s.v[2546] > (-230.25850929940458));
        s.v[2607] = if s.b[2607] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) && (!s.b[2606])) && s.b[2607]) {
            s.store_exp(2526, 2546);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) && (!s.b[2606])) && (!s.b[2607])) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2546), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2546), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) && (!s.b[2606])) {
            s.store_sub_scaled_inputs(2548, 2526, 2.0, 2510, 1.0);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) {
            s.store_scaled_div(2549, 2548, 2544, (s.v[430] * (1.772453850905516 * 0.5)));
            s.store_mul3_affine_lhs(2535, 2534, 2549, p.p839, 0.0, 2543);
        }

        s.b[2608] = (p.p845 == 0.0);
        s.v[2608] = if s.b[2608] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && s.b[2608]) {
            s.store_scalar(2550, 0.0);
        }

        s.b[2609] = (p.p825 == 0.5);
        s.v[2609] = if s.b[2609] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) && s.b[2609]) {
            s.store_sqrt_scaled_ad(2526, A::sub_from_scalar(p.p822, s.ad_value(2524)), s.v[424]);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) && (!s.b[2609])) {
            s.store_powf_ad(2526, A::scale_offset(s.ad_value(2524), (-s.v[424]), ((p.p822) * (s.v[424]))), p.p825);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) {
            s.store_scaled_div_ad_lhs(2551, A::scale_offset(s.ad_value(2524), (-s.v[421]), ((p.p822) * (s.v[421]))), 2526, s.v[406]);
        }

        s.b[2610] = (((((-s.v[436]) / s.v[2551])) as f64).abs() < 230.25850929940458);
        s.v[2610] = if s.b[2610] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) && s.b[2610]) {
            s.store_exp_ad(2526, A::div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(2551), 1.0));
        }

        s.b[2611] = (((-s.v[436]) / s.v[2551]) < 0.0);
        s.v[2611] = if s.b[2611] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) && (!s.b[2610])) && s.b[2611]) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(2551), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(2551), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) && (!s.b[2610])) && (!s.b[2611])) {
            let assign57750_ad_e73586: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(2551), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(2551), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(2551), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2526, assign57750_ad_e73586, 1e100);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) {
            s.store_mul_scaled_ad_lhs(2550, A::mul3(s.ad_value(826), s.ad_value(2551), s.ad_value(2551)), 2526, p.p845);
        }

        s.b[2612] = (p.p854 > 1000.0);
        s.v[2612] = if s.b[2612] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && s.b[2612]) {
            s.store_scalar(2552, 1.0);
        }

        s.b[2613] = (s.v[2525] > ((-s.v[438]) * p.p854));
        s.v[2613] = if s.b[2613] { 1.0 } else { 0.0 };

        s.b[2614] = (p.p857 == 4.0);
        s.v[2614] = if s.b[2614] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2612])) && s.b[2613]) && s.b[2614]) {
            s.store_mul_scaled_ad_lhs(2526, A::mul3_scaled_output(s.ad_value(2525), s.ad_value(2525), s.ad_value(2525), ((s.v[443] * s.v[443]) * s.v[443])), 2525, s.v[443]);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2612])) && s.b[2613]) && (!s.b[2614])) {
            s.store_powf_ad(2526, A::abs_scaled_input(s.ad_value(2525), s.v[443]), p.p857);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2612])) && s.b[2613]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2552, 1.0, 1.0, s.ad_value(2526));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2612])) && (!s.b[2613])) {
            s.store_offset_scaled(2552, 2525, s.v[446], (((((s.v[438] * p.p854)) * (s.v[446]))) + (s.v[440])));
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) {
            s.store_mul_scale_ad_lhs(1901, A::add(A::add_scaled_inputs3(s.ad_value(2527), 1.0, s.ad_value(2528), 1.0, s.ad_value(2535), 1.0), s.ad_value(2550)), p.p29, 2552);
        }

        s.b[2615] = (s.v[403] == 0.5);
        s.v[2615] = if s.b[2615] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && s.b[2615]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::scale(s.ad_value(2518), s.v[400]));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2615])) {
            s.store_powf_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2518), s.v[400])), s.v[403]);
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) {
            s.store_ad_value(1907, A::add_scaled_inputs3_offset(s.ad_value(2526), ((-s.v[412]) * p.p30), s.ad_value(826), (s.v[415] * p.p30), s.ad_value(2518), ((-s.v[415]) * p.p30), (s.v[412] * p.p30)));
        }

        s.b[2616] = (s.v[642] == 0.0);
        s.v[2616] = if s.b[2616] { 1.0 } else { 0.0 };

        if ((s.b[2555] && (!s.b[2556])) && s.b[2616]) {
            s.store_scalar(1902, 0.0);
            s.store_scalar(1908, 0.0);
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) {
            s.store_scale(2527, 2517, s.v[383]);
        }

        s.b[2617] = ((p.p835 == 0.0) && (p.p840 == 0.0));
        s.v[2617] = if s.b[2617] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2617]) {
            s.store_scalar(2528, 0.0);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2617])) {
            s.store_sub_from_scalar(2529, s.v[389], 2523);
            s.store_sub_from_scalar_ad(2530, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2521), s.ad_value(2529)))));
        }

        s.b[2618] = (p.p826 == 0.5);
        s.v[2618] = if s.b[2618] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2617])) && s.b[2618]) {
            s.store_scalar(2531, 0.0);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2617])) && (!s.b[2618])) {
            s.store_scaled_add_ad_lhs(2531, A::div_scaled_product(A::square(s.ad_value(2530)), A::ln(s.ad_value(2530)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2530)), 1.0), 2530, (1.0 - (2.0 * p.p826)));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2617])) {
            s.store_add(2532, 2530, 2531);
        }

        s.b[2619] = (p.p826 == 0.5);
        s.v[2619] = if s.b[2619] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2617])) && s.b[2619]) {
            s.store_sqrt_scaled_input(2526, 2529, s.v[425]);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2617])) && (!s.b[2619])) {
            s.store_powf_ad(2526, A::scale(s.ad_value(2529), s.v[425]), p.p826);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2617])) {
            s.store_scale(2533, 2526, s.v[419]);
            s.store_ad_value(2534, A::mul_offset_lhs_scaled_output(s.ad_value(2520), (-1.0), s.ad_value(2533), s.v[380]));
            s.store_scaled_mul(2528, 2534, 2532, p.p835);
        }

        s.b[2620] = (p.p840 == 0.0);
        s.v[2620] = if s.b[2620] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2620]) {
            s.store_scalar(2535, 0.0);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) {
            s.store_scaled_div(2536, 2533, 2529, (s.v[404] * s.v[434]));
            s.store_div_from_scalar(2537, (0.666666666666667 * s.v[431]), 2536);
            s.store_square(2538, 2537);
            s.store_sqrt_ad(2539, A::div_scaled_product_offset_denominator(s.ad_value(2538), s.ad_value(2538), 1.0, A::square(s.ad_value(2538)), 1.0, 1.0));
            s.store_sqrt(2540, 2539);
            s.store_mul(2541, 2539, 2540);
        }

        s.b[2621] = (((-p.p826) * s.v[407]) == (-1.0));
        s.v[2621] = if s.b[2621] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && s.b[2621]) {
            s.store_div_from_scalar_offset_ad(2542, 1.0, A::mul(s.ad_value(2536), s.ad_value(2541)), 1.0);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && (!s.b[2621])) {
            s.store_powf_ad(2542, A::offset(A::mul(s.ad_value(2536), s.ad_value(2541)), 1.0), ((-p.p826) * s.v[407]));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) {
            s.store_ad_value(2543, A::div_scaled_product(s.ad_value(2532), s.ad_value(2542), 1.0, A::add(s.ad_value(2532), s.ad_value(2542)), 1.0));
            s.store_sqrt_scaled_ad(2544, A::div(s.ad_value(2536), s.ad_value(2540)), 0.375);
            s.store_ad_value(2545, A::add_scaled_product(s.ad_value(2539), (-1.0), s.ad_value(2537), s.ad_value(2540), 2.0));
            s.store_ad_value(2546, A::add_scaled_value_products(s.ad_value(2539), (-s.v[431]), s.ad_value(2537), s.ad_value(2540), s.v[431], s.ad_value(2536), s.ad_value(2541), 0.5));
            s.store_mul_offset_lhs(2547, 2545, (-1.0), 2544);
            s.store_square(2508, 2547);
        }

        s.b[2622] = (s.v[2547] > 0.0);
        s.v[2622] = if s.b[2622] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && s.b[2622]) {
            s.store_div_from_scalar_offset_scaled_input(2509, 1.0, 2547, s.v[366], 1.0);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && (!s.b[2622])) {
            s.store_div_from_scalar_sub_from_scalar_ad(2509, 1.0, 1.0, A::scale(s.ad_value(2547), s.v[366]));
        }

        s.b[2623] = (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458));
        s.v[2623] = if s.b[2623] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && s.b[2623]) {
            s.store_exp_sub(2526, 2546, 2508);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && (!s.b[2623])) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) {
            s.store_mul_ad_lhs(2510, A::add_scaled_inputs_product(s.ad_value(2509), 0.29214664, A::square(s.ad_value(2509)), s.v[367], A::square(s.ad_value(2509)), s.ad_value(2509), s.v[368]), 2526);
        }

        s.b[2624] = (s.v[2547] > 0.0);
        s.v[2624] = if s.b[2624] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && s.b[2624]) {
            s.copy_ad(2548, 2510);
        }

        s.b[2625] = (s.v[2546] > (-230.25850929940458));
        s.v[2625] = if s.b[2625] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && (!s.b[2624])) && s.b[2625]) {
            s.store_exp(2526, 2546);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && (!s.b[2624])) && (!s.b[2625])) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2546), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2546), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && (!s.b[2624])) {
            s.store_sub_scaled_inputs(2548, 2526, 2.0, 2510, 1.0);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) {
            s.store_scaled_div(2549, 2548, 2544, (s.v[431] * (1.772453850905516 * 0.5)));
            s.store_mul3_affine_lhs(2535, 2534, 2549, p.p840, 0.0, 2543);
        }

        s.b[2626] = (p.p846 == 0.0);
        s.v[2626] = if s.b[2626] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2626]) {
            s.store_scalar(2550, 0.0);
        }

        s.b[2627] = (p.p826 == 0.5);
        s.v[2627] = if s.b[2627] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) && s.b[2627]) {
            s.store_sqrt_scaled_ad(2526, A::sub_from_scalar(p.p823, s.ad_value(2524)), s.v[425]);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) && (!s.b[2627])) {
            s.store_powf_ad(2526, A::scale_offset(s.ad_value(2524), (-s.v[425]), ((p.p823) * (s.v[425]))), p.p826);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) {
            s.store_scaled_div_ad_lhs(2551, A::scale_offset(s.ad_value(2524), (-s.v[422]), ((p.p823) * (s.v[422]))), 2526, s.v[407]);
        }

        s.b[2628] = (((((-s.v[437]) / s.v[2551])) as f64).abs() < 230.25850929940458);
        s.v[2628] = if s.b[2628] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) && s.b[2628]) {
            s.store_exp_ad(2526, A::div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(2551), 1.0));
        }

        s.b[2629] = (((-s.v[437]) / s.v[2551]) < 0.0);
        s.v[2629] = if s.b[2629] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) && (!s.b[2628])) && s.b[2629]) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(2551), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(2551), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) && (!s.b[2628])) && (!s.b[2629])) {
            let assign58500_ad_e74852: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(2551), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(2551), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(2551), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2526, assign58500_ad_e74852, 1e100);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) {
            s.store_mul_scaled_ad_lhs(2550, A::mul3(s.ad_value(826), s.ad_value(2551), s.ad_value(2551)), 2526, p.p846);
        }

        s.b[2630] = (s.v[634] > 1000.0);
        s.v[2630] = if s.b[2630] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2630]) {
            s.store_scalar(2552, 1.0);
        }

        s.b[2631] = (s.v[2525] > ((-s.v[438]) * s.v[634]));
        s.v[2631] = if s.b[2631] { 1.0 } else { 0.0 };

        s.b[2632] = (p.p858 == 4.0);
        s.v[2632] = if s.b[2632] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2630])) && s.b[2631]) && s.b[2632]) {
            s.store_mul_ad_product_lhs(2526, A::mul3(A::mul3(s.ad_value(2525), s.ad_value(444), A::mul(s.ad_value(2525), s.ad_value(444))), s.ad_value(2525), s.ad_value(444)), s.ad_value(2525), 444);
        }

    }

    pub(super) fn stamp_transient_block_41(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2630])) && s.b[2631]) && (!s.b[2632])) {
            s.store_powf_ad(2526, A::abs(A::mul(s.ad_value(2525), s.ad_value(444))), p.p858);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2630])) && s.b[2631]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2552, 1.0, 1.0, s.ad_value(2526));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2630])) && (!s.b[2631])) {
            s.store_offset_mul_ad(2552, A::add_scaled_inputs(s.ad_value(2525), 1.0, s.ad_value(634), s.v[438]), s.ad_value(447), s.v[441]);
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) {
            s.store_mul_scale_ad_lhs(1902, A::add(A::add_scaled_inputs3(s.ad_value(2527), 1.0, s.ad_value(2528), 1.0, s.ad_value(2535), 1.0), s.ad_value(2550)), p.p29, 2552);
        }

        s.b[2633] = (s.v[467] == 1.0);
        s.v[2633] = if s.b[2633] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) {
            let assign58620_ad_e75078: A = {
                if (s.v[826] < p.p863) {
                    {
                        if (((s.v[826] - p.p863) / p.p864) < (-37.0)) {
                            A::constant(p.p863)
                        } else {
                            A::scale_offset(A::ln_one_plus_exp(A::scaled_offset(s.ad_value(826), (-p.p863), 1.0 / (p.p864))), p.p864, p.p863)
                        }
                    }
                } else {
                    {
                        if (((s.v[826] - p.p863) / p.p864) > 37.0) {
                            s.ad_value(826)
                        } else {
                            A::add_scaled_inputs(s.ad_value(826), 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(826), (-1.0 / (p.p864)), ((p.p863) * (1.0 / (p.p864))))), p.p864)
                        }
                    }
                }
            };
            s.store_ad_value(2553, assign58620_ad_e75078);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) {
            s.store_scaled_mul(2511, 651, 651, 4.0);
            s.store_div(2512, 651, 652);
            s.store_ad_value(2513, A::add_scaled_product(s.ad_value(2553), 1.0, s.ad_value(651), s.ad_value(2512), 1.0));
            s.store_add(2514, 652, 2513);
            s.store_sub(2515, 652, 2513);
            s.store_sqrt_square_add(2516, 2515, 2511);
            s.store_ad_value(2554, A::div_scaled_product(s.ad_value(2553), s.ad_value(652), 2.0, A::add(s.ad_value(2514), s.ad_value(2516)), 1.0));
        }

        s.b[2634] = (s.v[404] == 0.5);
        s.v[2634] = if s.b[2634] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) && s.b[2634]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::scale(s.ad_value(2554), s.v[401]));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) && (!s.b[2634])) {
            s.store_powf_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2554), s.v[401])), s.v[404]);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) {
            s.store_ad_value(1908, A::add_scaled_inputs3_offset(s.ad_value(2526), ((-s.v[413]) * p.p30), s.ad_value(2553), (s.v[416] * p.p30), s.ad_value(2554), ((-s.v[416]) * p.p30), (s.v[413] * p.p30)));
            s.store_sub_ad_lhs(2553, A::offset(s.ad_value(826), p.p863), 2553);
            s.store_scaled_mul(2511, 651, 651, 4.0);
            s.store_div(2512, 651, 652);
            s.store_ad_value(2513, A::add_scaled_product(s.ad_value(2553), 1.0, s.ad_value(651), s.ad_value(2512), 1.0));
            s.store_add(2514, 652, 2513);
            s.store_sub(2515, 652, 2513);
            s.store_sqrt_square_add(2516, 2515, 2511);
            s.store_ad_value(2554, A::div_scaled_product(s.ad_value(2553), s.ad_value(652), 2.0, A::add(s.ad_value(2514), s.ad_value(2516)), 1.0));
        }

        s.b[2635] = (s.v[461] == 0.5);
        s.v[2635] = if s.b[2635] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) && s.b[2635]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::mul(s.ad_value(2554), s.ad_value(460)));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) && (!s.b[2635])) {
            s.store_pow_ad(2526, A::sub_from_scalar(1.0, A::mul(s.ad_value(2554), s.ad_value(460))), s.ad_value(461));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) {
            s.store_ad_value(466, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(464), 1.0, s.ad_value(2526)), p.p30, s.ad_value(465), A::sub(s.ad_value(2553), s.ad_value(2554)), p.p30));
            s.store_add(1908, 1908, 466);
        }

        s.b[2636] = (s.v[404] == 0.5);
        s.v[2636] = if s.b[2636] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2633])) && s.b[2636]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::scale(s.ad_value(2518), s.v[401]));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2633])) && (!s.b[2636])) {
            s.store_powf_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2518), s.v[401])), s.v[404]);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2633])) {
            s.store_ad_value(1908, A::add_scaled_inputs3_offset(s.ad_value(2526), ((-s.v[413]) * p.p30), s.ad_value(826), (s.v[416] * p.p30), s.ad_value(2518), ((-s.v[416]) * p.p30), (s.v[413] * p.p30)));
        }

        if (s.b[2555] && (!s.b[2556])) {
            s.store_ad_value(842, A::add_scaled_products3(s.ad_value(640), s.ad_value(1900), 1.0, s.ad_value(641), s.ad_value(1901), 1.0, s.ad_value(642), s.ad_value(1902), 1.0));
        }

        s.b[2637] = (s.v[630] > 0.0);
        s.v[2637] = if s.b[2637] { 1.0 } else { 0.0 };

        if ((s.b[2555] && (!s.b[2556])) && s.b[2637]) {
            s.store_mul_sub_ad_rhs(637, 630, A::pow(A::add_scaled_inputs3(s.ad_value(819), 0.5, s.ad_value(821), 0.5, A::sqrt(A::offset(A::mul(A::add(s.ad_value(819), s.ad_value(821)), A::add(s.ad_value(819), s.ad_value(821))), (0.001 * 0.001))), 0.5), s.ad_value(631)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(631)));
            s.store_add(635, 536, 637);
            s.store_div_from_scalar(610, 1.0, 635);
            s.store_div_ad_rhs(613, 613, A::offset(A::div(s.ad_value(637), s.ad_value(536)), 1.0));
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2637])) {
            s.copy_ad(635, 536);
        }

        s.b[2638] = (s.v[632] > 0.0);
        s.v[2638] = if s.b[2638] { 1.0 } else { 0.0 };

        if ((s.b[2555] && (!s.b[2556])) && s.b[2638]) {
            s.store_mul_sub_ad_rhs(639, 632, A::pow(A::add_scaled_inputs3(s.ad_value(819), 0.5, s.ad_value(821), 0.5, A::sqrt(A::offset(A::mul(A::add(s.ad_value(819), s.ad_value(821)), A::add(s.ad_value(819), s.ad_value(821))), (0.001 * 0.001))), 0.5), s.ad_value(633)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(633)));
            s.store_mul_offset_rhs(604, 604, 639, 1.0);
        }

        if (s.b[2555] && (!s.b[2556])) {
            s.store_scalar(2524, 0.0);
            s.store_scalar(2521, 0.0);
        }

        s.b[2639] = (!(((s.v[667] == 0.0) && (s.v[668] == 0.0)) && (s.v[669] == 0.0)));
        s.v[2639] = if s.b[2639] { 1.0 } else { 0.0 };

        if ((s.b[2555] && (!s.b[2556])) && s.b[2639]) {
            s.store_scaled_mul(2511, 678, 678, 4.0);
            s.store_div(2512, 678, 679);
            s.store_ad_value(2513, A::add_scaled_product(s.ad_value(827), 1.0, s.ad_value(678), s.ad_value(2512), 1.0));
            s.store_add(2514, 679, 2513);
            s.store_sub(2515, 679, 2513);
            s.store_sqrt_square_add(2516, 2515, 2511);
            s.store_ad_value(2518, A::div_scaled_product(s.ad_value(827), s.ad_value(679), 2.0, A::add(s.ad_value(2514), s.ad_value(2516)), 1.0));
        }

        s.b[2640] = (s.v[827] < s.v[675]);
        s.v[2640] = if s.b[2640] { 1.0 } else { 0.0 };

        s.b[2641] = (((((-0.5) * (s.v[827] * s.v[365]))) as f64).abs() < 230.25850929940458);
        s.v[2641] = if s.b[2641] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && s.b[2639]) && s.b[2640]) && s.b[2641]) {
            s.store_exp_scaled_input(2519, 827, (s.v[365] * (-0.5)));
        }

        s.b[2642] = (((-0.5) * (s.v[827] * s.v[365])) < 0.0);
        s.v[2642] = if s.b[2642] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && s.b[2639]) && s.b[2640]) && (!s.b[2641])) && s.b[2642]) {
            s.store_div_from_scalar_offset_ad(2519, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::scale(s.ad_value(827), (s.v[365] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(827), (s.v[365] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2555] && (!s.b[2556])) && s.b[2639]) && s.b[2640]) && (!s.b[2641])) && (!s.b[2642])) {
            s.store_scaled_offset_ad(2519, A::mul_offset_rhs(A::scale_offset(s.ad_value(827), (s.v[365] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(827), (s.v[365] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(827), (((s.v[365] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[2555] && (!s.b[2556])) && s.b[2639]) && s.b[2640]) {
            s.store_div_from_scalar(2520, 1.0, 2519);
            s.store_square(2517, 2520);
        }

        if (((s.b[2555] && (!s.b[2556])) && s.b[2639]) && (!s.b[2640])) {
            s.store_mul_offset_ad_lhs(2517, A::sub_scaled_inputs(s.ad_value(827), s.v[365], s.ad_value(675), s.v[365]), 1.0, 676);
            s.store_sqrt(2520, 2517);
            s.store_div_from_scalar(2519, 1.0, 2520);
        }

        if ((s.b[2555] && (!s.b[2556])) && s.b[2639]) {
            s.store_offset(2517, 2517, (-1.0));
        }

        s.b[2643] = (s.v[827] > 0.0);
        s.v[2643] = if s.b[2643] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && s.b[2639]) && s.b[2643]) {
            s.store_scaled_ln_ad(2521, A::add(A::offset(s.ad_value(2519), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2519), 1.0, A::offset(s.ad_value(2519), 3.0)))), (s.v[364] * 2.0));
        }

        if (((s.b[2555] && (!s.b[2556])) && s.b[2639]) && (!s.b[2643])) {
            s.store_sub_ad_lhs(2521, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(2520), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2520), 1.0, A::scale_offset(s.ad_value(2520), 3.0, 1.0))))), (s.v[364] * 2.0)), 827);
        }

        if ((s.b[2555] && (!s.b[2556])) && s.b[2639]) {
            s.store_sub(2522, 677, 2521);
            s.store_ad_value(2523, A::add_scaled_inputs3(s.ad_value(827), 0.5, s.ad_value(2522), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(827), s.ad_value(2522)), A::sub(s.ad_value(827), s.ad_value(2522))), ((4.0 * s.v[364]) * s.v[364]))), (-0.5)));
            s.store_ad_value(2524, A::add_scaled_inputs3(s.ad_value(827), 0.5, s.ad_value(680), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(827), s.ad_value(680)), A::sub(s.ad_value(827), s.ad_value(680))), ((4.0 * s.v[362]) * s.v[362]))), (-0.5)));
            s.store_scaled_sub_ad_rhs(2525, 827, A::sqrt(A::offset(A::mul(s.ad_value(827), s.ad_value(827)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[2644] = (s.v[667] == 0.0);
        s.v[2644] = if s.b[2644] { 1.0 } else { 0.0 };

        if ((s.b[2555] && (!s.b[2556])) && s.b[2644]) {
            s.store_scalar(1903, 0.0);
            s.store_scalar(1909, 0.0);
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) {
            s.store_mul(2527, 557, 2517);
        }

        s.b[2645] = ((s.v[516] == 0.0) && (s.v[519] == 0.0));
        s.v[2645] = if s.b[2645] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && s.b[2645]) {
            s.store_scalar(2528, 0.0);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2645])) {
            s.store_sub(2529, 563, 2523);
            s.store_sub_from_scalar_ad(2530, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2521), s.ad_value(2529)))));
        }

        s.b[2646] = (s.v[505] == 0.5);
        s.v[2646] = if s.b[2646] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2645])) && s.b[2646]) {
            s.store_scalar(2531, 0.0);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2645])) && (!s.b[2646])) {
            s.store_ad_value(2531, A::mul_sub_from_scalar_rhs(A::add(A::div_scaled_product(A::square(s.ad_value(2530)), A::ln(s.ad_value(2530)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2530)), 1.0), s.ad_value(2530)), 1.0, A::scale(s.ad_value(505), 2.0)));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2645])) {
            s.store_add(2532, 2530, 2531);
        }

        s.b[2647] = (s.v[505] == 0.5);
        s.v[2647] = if s.b[2647] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2645])) && s.b[2647]) {
            s.store_sqrt_mul(2526, 2529, 590);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2645])) && (!s.b[2647])) {
            s.store_pow_ad(2526, A::mul(s.ad_value(2529), s.ad_value(590)), s.ad_value(505));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2645])) {
            s.store_mul(2533, 584, 2526);
            s.store_mul_ad_product_lhs(2534, s.ad_value(554), A::offset(s.ad_value(2520), (-1.0)), 2533);
            s.store_mul3_lhs(2528, 516, 2534, 2532);
        }

        s.b[2648] = (s.v[519] == 0.0);
        s.v[2648] = if s.b[2648] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && s.b[2648]) {
            s.store_scalar(2535, 0.0);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) {
            s.store_mul_ad_rhs(2536, 599, A::div_scaled_product(s.ad_value(2533), s.ad_value(569), 1.0, s.ad_value(2529), 1.0));
            s.store_scaled_div(2537, 596, 2536, 0.666666666666667);
            s.store_square(2538, 2537);
            s.store_sqrt_ad(2539, A::div_scaled_product_offset_denominator(s.ad_value(2538), s.ad_value(2538), 1.0, A::square(s.ad_value(2538)), 1.0, 1.0));
            s.store_sqrt(2540, 2539);
            s.store_mul(2541, 2539, 2540);
        }

        s.b[2649] = (((-s.v[505]) * s.v[572]) == (-1.0));
        s.v[2649] = if s.b[2649] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) && s.b[2649]) {
            s.store_div_from_scalar_offset_ad(2542, 1.0, A::mul(s.ad_value(2536), s.ad_value(2541)), 1.0);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) && (!s.b[2649])) {
            s.store_pow_ad(2542, A::offset(A::mul(s.ad_value(2536), s.ad_value(2541)), 1.0), A::mul_scaled_lhs(s.ad_value(505), -1.0, s.ad_value(572)));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) {
            s.store_ad_value(2543, A::div_scaled_product(s.ad_value(2532), s.ad_value(2542), 1.0, A::add(s.ad_value(2532), s.ad_value(2542)), 1.0));
            s.store_sqrt_scaled_ad(2544, A::div(s.ad_value(2536), s.ad_value(2540)), 0.375);
            s.store_ad_value(2545, A::add_scaled_product(s.ad_value(2539), (-1.0), s.ad_value(2537), s.ad_value(2540), 2.0));
            s.store_ad_value(2546, A::add_scaled_value_products(A::mul3(s.ad_value(596), s.ad_value(2537), s.ad_value(2540)), 1.0, s.ad_value(596), s.ad_value(2539), (-1.0), s.ad_value(2536), s.ad_value(2541), 0.5));
            s.store_mul_offset_lhs(2547, 2545, (-1.0), 2544);
            s.store_square(2508, 2547);
        }

        s.b[2650] = (s.v[2547] > 0.0);
        s.v[2650] = if s.b[2650] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) && s.b[2650]) {
            s.store_div_from_scalar_offset_scaled_input(2509, 1.0, 2547, s.v[366], 1.0);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) && (!s.b[2650])) {
            s.store_div_from_scalar_sub_from_scalar_ad(2509, 1.0, 1.0, A::scale(s.ad_value(2547), s.v[366]));
        }

        s.b[2651] = (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458));
        s.v[2651] = if s.b[2651] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) && s.b[2651]) {
            s.store_exp_sub(2526, 2546, 2508);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) && (!s.b[2651])) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) {
            s.store_mul_ad_lhs(2510, A::add_scaled_inputs_product(s.ad_value(2509), 0.29214664, A::square(s.ad_value(2509)), s.v[367], A::square(s.ad_value(2509)), s.ad_value(2509), s.v[368]), 2526);
        }

        s.b[2652] = (s.v[2547] > 0.0);
        s.v[2652] = if s.b[2652] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) && s.b[2652]) {
            s.copy_ad(2548, 2510);
        }

        s.b[2653] = (s.v[2546] > (-230.25850929940458));
        s.v[2653] = if s.b[2653] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) && (!s.b[2652])) && s.b[2653]) {
            s.store_exp(2526, 2546);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) && (!s.b[2652])) && (!s.b[2653])) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2546), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2546), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) && (!s.b[2652])) {
            s.store_sub_scaled_inputs(2548, 2526, 2.0, 2510, 1.0);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) {
            s.store_ad_value(2549, A::div_scaled_product(s.ad_value(596), s.ad_value(2548), (1.772453850905516 * 0.5), s.ad_value(2544), 1.0));
            s.store_mul_ad_rhs(2535, 519, A::mul3(s.ad_value(2534), s.ad_value(2549), s.ad_value(2543)));
        }

        s.b[2654] = (s.v[525] == 0.0);
        s.v[2654] = if s.b[2654] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && s.b[2654]) {
            s.store_scalar(2550, 0.0);
        }

        s.b[2655] = (s.v[505] == 0.5);
        s.v[2655] = if s.b[2655] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2654])) && s.b[2655]) {
            s.store_sqrt_mul_ad(2526, A::sub(s.ad_value(502), s.ad_value(2524)), s.ad_value(590));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2654])) && (!s.b[2655])) {
            s.store_pow_ad(2526, A::mul(A::sub(s.ad_value(502), s.ad_value(2524)), s.ad_value(590)), s.ad_value(505));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2654])) {
            s.store_mul_ad_rhs(2551, 572, A::div_scaled_product(A::sub(s.ad_value(502), s.ad_value(2524)), s.ad_value(587), 1.0, s.ad_value(2526), 1.0));
        }

    }

    pub(super) fn stamp_transient_block_42(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2656] = (((((-s.v[602]) / s.v[2551])) as f64).abs() < 230.25850929940458);
        s.v[2656] = if s.b[2656] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2654])) && s.b[2656]) {
            s.store_exp_ad(2526, A::div_scaled_inputs(s.ad_value(602), -1.0, s.ad_value(2551), 1.0));
        }

        s.b[2657] = (((-s.v[602]) / s.v[2551]) < 0.0);
        s.v[2657] = if s.b[2657] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2654])) && (!s.b[2656])) && s.b[2657]) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(602), -1.0, s.ad_value(2551), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(602), -1.0, s.ad_value(2551), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2654])) && (!s.b[2656])) && (!s.b[2657])) {
            let assign59900_ad_e77252: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(602), -1.0, s.ad_value(2551), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(602), -1.0, s.ad_value(2551), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(602), -1.0, s.ad_value(2551), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2526, assign59900_ad_e77252, 1e100);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2654])) {
            s.store_mul_ad_product_lhs(2550, s.ad_value(525), A::mul3(s.ad_value(827), s.ad_value(2551), s.ad_value(2551)), 2526);
        }

        s.b[2658] = (s.v[534] > 1000.0);
        s.v[2658] = if s.b[2658] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && s.b[2658]) {
            s.store_scalar(2552, 1.0);
        }

        s.b[2659] = (s.v[2525] > ((-s.v[438]) * s.v[534]));
        s.v[2659] = if s.b[2659] { 1.0 } else { 0.0 };

        s.b[2660] = (s.v[537] == 4.0);
        s.v[2660] = if s.b[2660] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2658])) && s.b[2659]) && s.b[2660]) {
            s.store_mul_ad_product_lhs(2526, A::mul3(A::mul3(s.ad_value(2525), s.ad_value(608), A::mul(s.ad_value(2525), s.ad_value(608))), s.ad_value(2525), s.ad_value(608)), s.ad_value(2525), 608);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2658])) && s.b[2659]) && (!s.b[2660])) {
            s.store_pow_ad(2526, A::abs(A::mul(s.ad_value(2525), s.ad_value(608))), s.ad_value(537));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2658])) && s.b[2659]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2552, 1.0, 1.0, s.ad_value(2526));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2658])) && (!s.b[2659])) {
            s.store_ad_value(2552, A::add_scaled_product(s.ad_value(605), 1.0, A::add_scaled_inputs(s.ad_value(2525), 1.0, s.ad_value(534), s.v[438]), s.ad_value(611), 1.0));
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) {
            s.store_mul_scale_ad_lhs(1903, A::add(A::add_scaled_inputs3(s.ad_value(2527), 1.0, s.ad_value(2528), 1.0, s.ad_value(2535), 1.0), s.ad_value(2550)), p.p29, 2552);
        }

        s.b[2661] = (s.v[569] == 0.5);
        s.v[2661] = if s.b[2661] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && s.b[2661]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::mul(s.ad_value(2518), s.ad_value(566)));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2661])) {
            s.store_pow_ad(2526, A::sub_from_scalar(1.0, A::mul(s.ad_value(2518), s.ad_value(566))), s.ad_value(569));
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) {
            s.store_ad_value(1909, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(578), 1.0, s.ad_value(2526)), p.p30, s.ad_value(581), A::sub(s.ad_value(827), s.ad_value(2518)), p.p30));
        }

        s.b[2662] = (s.v[668] == 0.0);
        s.v[2662] = if s.b[2662] { 1.0 } else { 0.0 };

        if ((s.b[2555] && (!s.b[2556])) && s.b[2662]) {
            s.store_scalar(1904, 0.0);
            s.store_scalar(1910, 0.0);
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) {
            s.store_mul(2527, 558, 2517);
        }

        s.b[2663] = ((s.v[517] == 0.0) && (s.v[520] == 0.0));
        s.v[2663] = if s.b[2663] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && s.b[2663]) {
            s.store_scalar(2528, 0.0);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2663])) {
            s.store_sub(2529, 564, 2523);
            s.store_sub_from_scalar_ad(2530, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2521), s.ad_value(2529)))));
        }

        s.b[2664] = (s.v[506] == 0.5);
        s.v[2664] = if s.b[2664] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2663])) && s.b[2664]) {
            s.store_scalar(2531, 0.0);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2663])) && (!s.b[2664])) {
            s.store_ad_value(2531, A::mul_sub_from_scalar_rhs(A::add(A::div_scaled_product(A::square(s.ad_value(2530)), A::ln(s.ad_value(2530)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2530)), 1.0), s.ad_value(2530)), 1.0, A::scale(s.ad_value(506), 2.0)));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2663])) {
            s.store_add(2532, 2530, 2531);
        }

        s.b[2665] = (s.v[506] == 0.5);
        s.v[2665] = if s.b[2665] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2663])) && s.b[2665]) {
            s.store_sqrt_mul(2526, 2529, 591);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2663])) && (!s.b[2665])) {
            s.store_pow_ad(2526, A::mul(s.ad_value(2529), s.ad_value(591)), s.ad_value(506));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2663])) {
            s.store_mul(2533, 585, 2526);
            s.store_mul_ad_product_lhs(2534, s.ad_value(555), A::offset(s.ad_value(2520), (-1.0)), 2533);
            s.store_mul3_lhs(2528, 517, 2534, 2532);
        }

        s.b[2666] = (s.v[520] == 0.0);
        s.v[2666] = if s.b[2666] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && s.b[2666]) {
            s.store_scalar(2535, 0.0);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) {
            s.store_mul_ad_rhs(2536, 600, A::div_scaled_product(s.ad_value(2533), s.ad_value(570), 1.0, s.ad_value(2529), 1.0));
            s.store_scaled_div(2537, 597, 2536, 0.666666666666667);
            s.store_square(2538, 2537);
            s.store_sqrt_ad(2539, A::div_scaled_product_offset_denominator(s.ad_value(2538), s.ad_value(2538), 1.0, A::square(s.ad_value(2538)), 1.0, 1.0));
            s.store_sqrt(2540, 2539);
            s.store_mul(2541, 2539, 2540);
        }

        s.b[2667] = (((-s.v[506]) * s.v[573]) == (-1.0));
        s.v[2667] = if s.b[2667] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) && s.b[2667]) {
            s.store_div_from_scalar_offset_ad(2542, 1.0, A::mul(s.ad_value(2536), s.ad_value(2541)), 1.0);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) && (!s.b[2667])) {
            s.store_pow_ad(2542, A::offset(A::mul(s.ad_value(2536), s.ad_value(2541)), 1.0), A::mul_scaled_lhs(s.ad_value(506), -1.0, s.ad_value(573)));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) {
            s.store_ad_value(2543, A::div_scaled_product(s.ad_value(2532), s.ad_value(2542), 1.0, A::add(s.ad_value(2532), s.ad_value(2542)), 1.0));
            s.store_sqrt_scaled_ad(2544, A::div(s.ad_value(2536), s.ad_value(2540)), 0.375);
            s.store_ad_value(2545, A::add_scaled_product(s.ad_value(2539), (-1.0), s.ad_value(2537), s.ad_value(2540), 2.0));
            s.store_ad_value(2546, A::add_scaled_value_products(A::mul3(s.ad_value(597), s.ad_value(2537), s.ad_value(2540)), 1.0, s.ad_value(597), s.ad_value(2539), (-1.0), s.ad_value(2536), s.ad_value(2541), 0.5));
            s.store_mul_offset_lhs(2547, 2545, (-1.0), 2544);
            s.store_square(2508, 2547);
        }

        s.b[2668] = (s.v[2547] > 0.0);
        s.v[2668] = if s.b[2668] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) && s.b[2668]) {
            s.store_div_from_scalar_offset_scaled_input(2509, 1.0, 2547, s.v[366], 1.0);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) && (!s.b[2668])) {
            s.store_div_from_scalar_sub_from_scalar_ad(2509, 1.0, 1.0, A::scale(s.ad_value(2547), s.v[366]));
        }

        s.b[2669] = (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458));
        s.v[2669] = if s.b[2669] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) && s.b[2669]) {
            s.store_exp_sub(2526, 2546, 2508);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) && (!s.b[2669])) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) {
            s.store_mul_ad_lhs(2510, A::add_scaled_inputs_product(s.ad_value(2509), 0.29214664, A::square(s.ad_value(2509)), s.v[367], A::square(s.ad_value(2509)), s.ad_value(2509), s.v[368]), 2526);
        }

        s.b[2670] = (s.v[2547] > 0.0);
        s.v[2670] = if s.b[2670] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) && s.b[2670]) {
            s.copy_ad(2548, 2510);
        }

        s.b[2671] = (s.v[2546] > (-230.25850929940458));
        s.v[2671] = if s.b[2671] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) && (!s.b[2670])) && s.b[2671]) {
            s.store_exp(2526, 2546);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) && (!s.b[2670])) && (!s.b[2671])) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2546), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2546), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) && (!s.b[2670])) {
            s.store_sub_scaled_inputs(2548, 2526, 2.0, 2510, 1.0);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) {
            s.store_ad_value(2549, A::div_scaled_product(s.ad_value(597), s.ad_value(2548), (1.772453850905516 * 0.5), s.ad_value(2544), 1.0));
            s.store_mul_ad_rhs(2535, 520, A::mul3(s.ad_value(2534), s.ad_value(2549), s.ad_value(2543)));
        }

        s.b[2672] = (s.v[526] == 0.0);
        s.v[2672] = if s.b[2672] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && s.b[2672]) {
            s.store_scalar(2550, 0.0);
        }

        s.b[2673] = (s.v[506] == 0.5);
        s.v[2673] = if s.b[2673] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2672])) && s.b[2673]) {
            s.store_sqrt_mul_ad(2526, A::sub(s.ad_value(503), s.ad_value(2524)), s.ad_value(591));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2672])) && (!s.b[2673])) {
            s.store_pow_ad(2526, A::mul(A::sub(s.ad_value(503), s.ad_value(2524)), s.ad_value(591)), s.ad_value(506));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2672])) {
            s.store_mul_ad_rhs(2551, 573, A::div_scaled_product(A::sub(s.ad_value(503), s.ad_value(2524)), s.ad_value(588), 1.0, s.ad_value(2526), 1.0));
        }

        s.b[2674] = (((((-s.v[603]) / s.v[2551])) as f64).abs() < 230.25850929940458);
        s.v[2674] = if s.b[2674] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2672])) && s.b[2674]) {
            s.store_exp_ad(2526, A::div_scaled_inputs(s.ad_value(603), -1.0, s.ad_value(2551), 1.0));
        }

        s.b[2675] = (((-s.v[603]) / s.v[2551]) < 0.0);
        s.v[2675] = if s.b[2675] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2672])) && (!s.b[2674])) && s.b[2675]) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(603), -1.0, s.ad_value(2551), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(603), -1.0, s.ad_value(2551), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2672])) && (!s.b[2674])) && (!s.b[2675])) {
            let assign60650_ad_e78518: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(603), -1.0, s.ad_value(2551), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(603), -1.0, s.ad_value(2551), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(603), -1.0, s.ad_value(2551), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2526, assign60650_ad_e78518, 1e100);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2672])) {
            s.store_mul_ad_product_lhs(2550, s.ad_value(526), A::mul3(s.ad_value(827), s.ad_value(2551), s.ad_value(2551)), 2526);
        }

        s.b[2676] = (s.v[535] > 1000.0);
        s.v[2676] = if s.b[2676] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && s.b[2676]) {
            s.store_scalar(2552, 1.0);
        }

        s.b[2677] = (s.v[2525] > ((-s.v[438]) * s.v[535]));
        s.v[2677] = if s.b[2677] { 1.0 } else { 0.0 };

        s.b[2678] = (s.v[538] == 4.0);
        s.v[2678] = if s.b[2678] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2676])) && s.b[2677]) && s.b[2678]) {
            s.store_mul_ad_product_lhs(2526, A::mul3(A::mul3(s.ad_value(2525), s.ad_value(609), A::mul(s.ad_value(2525), s.ad_value(609))), s.ad_value(2525), s.ad_value(609)), s.ad_value(2525), 609);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2676])) && s.b[2677]) && (!s.b[2678])) {
            s.store_pow_ad(2526, A::abs(A::mul(s.ad_value(2525), s.ad_value(609))), s.ad_value(538));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2676])) && s.b[2677]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2552, 1.0, 1.0, s.ad_value(2526));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2676])) && (!s.b[2677])) {
            s.store_ad_value(2552, A::add_scaled_product(s.ad_value(606), 1.0, A::add_scaled_inputs(s.ad_value(2525), 1.0, s.ad_value(535), s.v[438]), s.ad_value(612), 1.0));
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) {
            s.store_mul_scale_ad_lhs(1904, A::add(A::add_scaled_inputs3(s.ad_value(2527), 1.0, s.ad_value(2528), 1.0, s.ad_value(2535), 1.0), s.ad_value(2550)), p.p29, 2552);
        }

        s.b[2679] = (s.v[570] == 0.5);
        s.v[2679] = if s.b[2679] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && s.b[2679]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::mul(s.ad_value(2518), s.ad_value(567)));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2679])) {
            s.store_pow_ad(2526, A::sub_from_scalar(1.0, A::mul(s.ad_value(2518), s.ad_value(567))), s.ad_value(570));
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) {
            s.store_ad_value(1910, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(579), 1.0, s.ad_value(2526)), p.p30, s.ad_value(582), A::sub(s.ad_value(827), s.ad_value(2518)), p.p30));
        }

        s.b[2680] = (s.v[669] == 0.0);
        s.v[2680] = if s.b[2680] { 1.0 } else { 0.0 };

        if ((s.b[2555] && (!s.b[2556])) && s.b[2680]) {
            s.store_scalar(1905, 0.0);
            s.store_scalar(1911, 0.0);
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) {
            s.store_mul(2527, 559, 2517);
        }

        s.b[2681] = ((s.v[518] == 0.0) && (s.v[521] == 0.0));
        s.v[2681] = if s.b[2681] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2681]) {
            s.store_scalar(2528, 0.0);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2681])) {
            s.store_sub(2529, 565, 2523);
            s.store_sub_from_scalar_ad(2530, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2521), s.ad_value(2529)))));
        }

        s.b[2682] = (s.v[507] == 0.5);
        s.v[2682] = if s.b[2682] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2681])) && s.b[2682]) {
            s.store_scalar(2531, 0.0);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2681])) && (!s.b[2682])) {
            s.store_ad_value(2531, A::mul_sub_from_scalar_rhs(A::add(A::div_scaled_product(A::square(s.ad_value(2530)), A::ln(s.ad_value(2530)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2530)), 1.0), s.ad_value(2530)), 1.0, A::scale(s.ad_value(507), 2.0)));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2681])) {
            s.store_add(2532, 2530, 2531);
        }

        s.b[2683] = (s.v[507] == 0.5);
        s.v[2683] = if s.b[2683] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2681])) && s.b[2683]) {
            s.store_sqrt_mul(2526, 2529, 592);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2681])) && (!s.b[2683])) {
            s.store_pow_ad(2526, A::mul(s.ad_value(2529), s.ad_value(592)), s.ad_value(507));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2681])) {
            s.store_mul(2533, 586, 2526);
            s.store_mul_ad_product_lhs(2534, s.ad_value(556), A::offset(s.ad_value(2520), (-1.0)), 2533);
            s.store_mul3_lhs(2528, 518, 2534, 2532);
        }

        s.b[2684] = (s.v[521] == 0.0);
        s.v[2684] = if s.b[2684] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2684]) {
            s.store_scalar(2535, 0.0);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) {
            s.store_mul_ad_rhs(2536, 601, A::div_scaled_product(s.ad_value(2533), s.ad_value(571), 1.0, s.ad_value(2529), 1.0));
            s.store_scaled_div(2537, 598, 2536, 0.666666666666667);
            s.store_square(2538, 2537);
            s.store_sqrt_ad(2539, A::div_scaled_product_offset_denominator(s.ad_value(2538), s.ad_value(2538), 1.0, A::square(s.ad_value(2538)), 1.0, 1.0));
            s.store_sqrt(2540, 2539);
            s.store_mul(2541, 2539, 2540);
        }

        s.b[2685] = (((-s.v[507]) * s.v[574]) == (-1.0));
        s.v[2685] = if s.b[2685] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) && s.b[2685]) {
            s.store_div_from_scalar_offset_ad(2542, 1.0, A::mul(s.ad_value(2536), s.ad_value(2541)), 1.0);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) && (!s.b[2685])) {
            s.store_pow_ad(2542, A::offset(A::mul(s.ad_value(2536), s.ad_value(2541)), 1.0), A::mul_scaled_lhs(s.ad_value(507), -1.0, s.ad_value(574)));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) {
            s.store_ad_value(2543, A::div_scaled_product(s.ad_value(2532), s.ad_value(2542), 1.0, A::add(s.ad_value(2532), s.ad_value(2542)), 1.0));
            s.store_sqrt_scaled_ad(2544, A::div(s.ad_value(2536), s.ad_value(2540)), 0.375);
            s.store_ad_value(2545, A::add_scaled_product(s.ad_value(2539), (-1.0), s.ad_value(2537), s.ad_value(2540), 2.0));
            s.store_ad_value(2546, A::add_scaled_value_products(A::mul3(s.ad_value(598), s.ad_value(2537), s.ad_value(2540)), 1.0, s.ad_value(598), s.ad_value(2539), (-1.0), s.ad_value(2536), s.ad_value(2541), 0.5));
            s.store_mul_offset_lhs(2547, 2545, (-1.0), 2544);
            s.store_square(2508, 2547);
        }

        s.b[2686] = (s.v[2547] > 0.0);
        s.v[2686] = if s.b[2686] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) && s.b[2686]) {
            s.store_div_from_scalar_offset_scaled_input(2509, 1.0, 2547, s.v[366], 1.0);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) && (!s.b[2686])) {
            s.store_div_from_scalar_sub_from_scalar_ad(2509, 1.0, 1.0, A::scale(s.ad_value(2547), s.v[366]));
        }

        s.b[2687] = (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458));
        s.v[2687] = if s.b[2687] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) && s.b[2687]) {
            s.store_exp_sub(2526, 2546, 2508);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) && (!s.b[2687])) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

    }

    pub(super) fn stamp_transient_block_43(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) {
            s.store_mul_ad_lhs(2510, A::add_scaled_inputs_product(s.ad_value(2509), 0.29214664, A::square(s.ad_value(2509)), s.v[367], A::square(s.ad_value(2509)), s.ad_value(2509), s.v[368]), 2526);
        }

        s.b[2688] = (s.v[2547] > 0.0);
        s.v[2688] = if s.b[2688] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) && s.b[2688]) {
            s.copy_ad(2548, 2510);
        }

        s.b[2689] = (s.v[2546] > (-230.25850929940458));
        s.v[2689] = if s.b[2689] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) && (!s.b[2688])) && s.b[2689]) {
            s.store_exp(2526, 2546);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) && (!s.b[2688])) && (!s.b[2689])) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2546), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2546), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) && (!s.b[2688])) {
            s.store_sub_scaled_inputs(2548, 2526, 2.0, 2510, 1.0);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) {
            s.store_ad_value(2549, A::div_scaled_product(s.ad_value(598), s.ad_value(2548), (1.772453850905516 * 0.5), s.ad_value(2544), 1.0));
            s.store_mul_ad_rhs(2535, 521, A::mul3(s.ad_value(2534), s.ad_value(2549), s.ad_value(2543)));
        }

        s.b[2690] = (s.v[527] == 0.0);
        s.v[2690] = if s.b[2690] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2690]) {
            s.store_scalar(2550, 0.0);
        }

        s.b[2691] = (s.v[507] == 0.5);
        s.v[2691] = if s.b[2691] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2690])) && s.b[2691]) {
            s.store_sqrt_mul_ad(2526, A::sub(s.ad_value(504), s.ad_value(2524)), s.ad_value(592));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2690])) && (!s.b[2691])) {
            s.store_pow_ad(2526, A::mul(A::sub(s.ad_value(504), s.ad_value(2524)), s.ad_value(592)), s.ad_value(507));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2690])) {
            s.store_mul_ad_rhs(2551, 574, A::div_scaled_product(A::sub(s.ad_value(504), s.ad_value(2524)), s.ad_value(589), 1.0, s.ad_value(2526), 1.0));
        }

        s.b[2692] = (((((-s.v[604]) / s.v[2551])) as f64).abs() < 230.25850929940458);
        s.v[2692] = if s.b[2692] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2690])) && s.b[2692]) {
            s.store_exp_ad(2526, A::div_scaled_inputs(s.ad_value(604), -1.0, s.ad_value(2551), 1.0));
        }

        s.b[2693] = (((-s.v[604]) / s.v[2551]) < 0.0);
        s.v[2693] = if s.b[2693] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2690])) && (!s.b[2692])) && s.b[2693]) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(604), -1.0, s.ad_value(2551), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(604), -1.0, s.ad_value(2551), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2690])) && (!s.b[2692])) && (!s.b[2693])) {
            let assign61400_ad_e79784: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(604), -1.0, s.ad_value(2551), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(604), -1.0, s.ad_value(2551), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(604), -1.0, s.ad_value(2551), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2526, assign61400_ad_e79784, 1e100);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2690])) {
            s.store_mul_ad_product_lhs(2550, s.ad_value(527), A::mul3(s.ad_value(827), s.ad_value(2551), s.ad_value(2551)), 2526);
        }

        s.b[2694] = (s.v[635] > 1000.0);
        s.v[2694] = if s.b[2694] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2694]) {
            s.store_scalar(2552, 1.0);
        }

        s.b[2695] = (s.v[2525] > ((-s.v[438]) * s.v[635]));
        s.v[2695] = if s.b[2695] { 1.0 } else { 0.0 };

        s.b[2696] = (s.v[539] == 4.0);
        s.v[2696] = if s.b[2696] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2694])) && s.b[2695]) && s.b[2696]) {
            s.store_mul_ad_product_lhs(2526, A::mul3(A::mul3(s.ad_value(2525), s.ad_value(610), A::mul(s.ad_value(2525), s.ad_value(610))), s.ad_value(2525), s.ad_value(610)), s.ad_value(2525), 610);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2694])) && s.b[2695]) && (!s.b[2696])) {
            s.store_pow_ad(2526, A::abs(A::mul(s.ad_value(2525), s.ad_value(610))), s.ad_value(539));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2694])) && s.b[2695]) {
            s.store_div_from_scalar_sub_from_scalar_ad(2552, 1.0, 1.0, s.ad_value(2526));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2694])) && (!s.b[2695])) {
            s.store_ad_value(2552, A::add_scaled_product(s.ad_value(607), 1.0, A::add_scaled_inputs(s.ad_value(2525), 1.0, s.ad_value(635), s.v[438]), s.ad_value(613), 1.0));
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) {
            s.store_mul_scale_ad_lhs(1905, A::add(A::add_scaled_inputs3(s.ad_value(2527), 1.0, s.ad_value(2528), 1.0, s.ad_value(2535), 1.0), s.ad_value(2550)), p.p29, 2552);
        }

        s.b[2697] = (s.v[629] == 1.0);
        s.v[2697] = if s.b[2697] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) {
            let assign61520_ad_e80010: A = {
                if (s.v[827] < s.v[544]) {
                    {
                        if (((s.v[827] - s.v[544]) / s.v[545]) < (-37.0)) {
                            s.ad_value(544)
                        } else {
                            A::add_scaled_product(s.ad_value(544), 1.0, A::ln_one_plus_exp(A::div(A::sub(s.ad_value(827), s.ad_value(544)), s.ad_value(545))), s.ad_value(545), 1.0)
                        }
                    }
                } else {
                    {
                        if (((s.v[827] - s.v[544]) / s.v[545]) > 37.0) {
                            s.ad_value(827)
                        } else {
                            A::add_scaled_product(s.ad_value(827), 1.0, A::ln_one_plus_exp(A::div(A::sub(s.ad_value(544), s.ad_value(827)), s.ad_value(545))), s.ad_value(545), 1.0)
                        }
                    }
                }
            };
            s.store_ad_value(2553, assign61520_ad_e80010);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) {
            s.store_scaled_mul(2511, 678, 678, 4.0);
            s.store_div(2512, 678, 679);
            s.store_ad_value(2513, A::add_scaled_product(s.ad_value(2553), 1.0, s.ad_value(678), s.ad_value(2512), 1.0));
            s.store_add(2514, 679, 2513);
            s.store_sub(2515, 679, 2513);
            s.store_sqrt_square_add(2516, 2515, 2511);
            s.store_ad_value(2554, A::div_scaled_product(s.ad_value(2553), s.ad_value(679), 2.0, A::add(s.ad_value(2514), s.ad_value(2516)), 1.0));
        }

        s.b[2698] = (s.v[571] == 0.5);
        s.v[2698] = if s.b[2698] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) && s.b[2698]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::mul(s.ad_value(2554), s.ad_value(568)));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) && (!s.b[2698])) {
            s.store_pow_ad(2526, A::sub_from_scalar(1.0, A::mul(s.ad_value(2554), s.ad_value(568))), s.ad_value(571));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) {
            s.store_ad_value(1911, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(580), 1.0, s.ad_value(2526)), p.p30, s.ad_value(583), A::sub(s.ad_value(2553), s.ad_value(2554)), p.p30));
            s.store_ad_value(2553, A::add_scaled_inputs3(s.ad_value(827), 1.0, s.ad_value(544), 1.0, s.ad_value(2553), -1.0));
            s.store_scaled_mul(2511, 678, 678, 4.0);
            s.store_div(2512, 678, 679);
            s.store_ad_value(2513, A::add_scaled_product(s.ad_value(2553), 1.0, s.ad_value(678), s.ad_value(2512), 1.0));
            s.store_add(2514, 679, 2513);
            s.store_sub(2515, 679, 2513);
            s.store_sqrt_square_add(2516, 2515, 2511);
            s.store_ad_value(2554, A::div_scaled_product(s.ad_value(2553), s.ad_value(679), 2.0, A::add(s.ad_value(2514), s.ad_value(2516)), 1.0));
        }

        s.b[2699] = (s.v[624] == 0.5);
        s.v[2699] = if s.b[2699] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) && s.b[2699]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::mul(s.ad_value(2554), s.ad_value(623)));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) && (!s.b[2699])) {
            s.store_pow_ad(2526, A::sub_from_scalar(1.0, A::mul(s.ad_value(2554), s.ad_value(623))), s.ad_value(624));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) {
            s.store_ad_value(466, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(627), 1.0, s.ad_value(2526)), p.p30, s.ad_value(628), A::sub(s.ad_value(2553), s.ad_value(2554)), p.p30));
            s.store_add(1911, 1911, 466);
        }

        s.b[2700] = (s.v[571] == 0.5);
        s.v[2700] = if s.b[2700] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2697])) && s.b[2700]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::mul(s.ad_value(2518), s.ad_value(568)));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2697])) && (!s.b[2700])) {
            s.store_pow_ad(2526, A::sub_from_scalar(1.0, A::mul(s.ad_value(2518), s.ad_value(568))), s.ad_value(571));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2697])) {
            s.store_ad_value(1911, A::add_scaled_product(A::mul_sub_from_scalar_rhs(s.ad_value(580), 1.0, s.ad_value(2526)), p.p30, s.ad_value(583), A::sub(s.ad_value(827), s.ad_value(2518)), p.p30));
        }

        if (s.b[2555] && (!s.b[2556])) {
            s.store_ad_value(843, A::add_scaled_products3(s.ad_value(667), s.ad_value(1903), 1.0, s.ad_value(668), s.ad_value(1904), 1.0, s.ad_value(669), s.ad_value(1905), 1.0));
        }

        s.store_scale(865, 805, s.v[712]);

        s.store_scale(866, 806, s.v[712]);

        s.store_scale(867, 807, s.v[712]);

        s.store_scale(868, 808, s.v[712]);

        s.store_scale(869, 809, s.v[712]);

        s.store_scale(870, 810, s.v[712]);

        s.store_scale(871, 811, s.v[712]);

        s.b[2701] = (s.v[825] > 0.0);
        s.v[2701] = if s.b[2701] { 1.0 } else { 0.0 };

        s.b[2702] = (s.v[295] > 0.0);
        s.v[2702] = if s.b[2702] { 1.0 } else { 0.0 };

        s.b[2703] = (s.v[296] > 0.0);
        s.v[2703] = if s.b[2703] { 1.0 } else { 0.0 };

        s.b[2704] = (s.v[297] > 0.0);
        s.v[2704] = if s.b[2704] { 1.0 } else { 0.0 };

        s.b[2705] = (s.v[298] > 0.0);
        s.v[2705] = if s.b[2705] { 1.0 } else { 0.0 };

        s.b[2706] = (s.v[299] > 0.0);
        s.v[2706] = if s.b[2706] { 1.0 } else { 0.0 };

        s.b[2707] = (s.v[300] > 0.0);
        s.v[2707] = if s.b[2707] { 1.0 } else { 0.0 };

        s.b[2708] = (s.v[301] > 0.0);
        s.v[2708] = if s.b[2708] { 1.0 } else { 0.0 };

        s.store_neg_ad(844, A::add_scaled_inputs3(s.ad_value(845), 1.0, s.ad_value(846), 1.0, s.ad_value(847), 1.0));

        s.store_add(848, 848, 1898);

        s.store_add(849, 849, 1899);

        s.store_ad_value(851, A::add_scaled_products3(s.ad_value(640), s.ad_value(1906), 1.0, s.ad_value(641), s.ad_value(1907), 1.0, s.ad_value(642), s.ad_value(1908), 1.0));

        s.store_ad_value(852, A::add_scaled_products3(s.ad_value(667), s.ad_value(1909), 1.0, s.ad_value(668), s.ad_value(1910), 1.0, s.ad_value(669), s.ad_value(1911), 1.0));

        s.b[2710] = (s.v[825] < 0.0);
        s.v[2710] = if s.b[2710] { 1.0 } else { 0.0 };

        if s.b[2710] {
            s.copy_ad(2709, 847);
            s.copy_ad(847, 844);
            s.copy_ad(844, 2709);
        }

        s.v[858] = 0.0;

        s.v[2727] = 0.0;

        s.v[2722] = 0.0;

        s.v[853] = 1e-40;

        s.v[855] = 0.0;

        s.v[857] = 0.0;

        s.store_mul(854, 1892, 1883);

        s.v[856] = 0.0;

        s.v[2729] = 0.0;

        s.v[863] = 0.0;

        s.v[2742] = 0.0;

        s.v[864] = 0.0;

        s.b[2743] = ((s.v[1817] > 0.0) && (s.v[710] > 0.0));
        s.v[2743] = if s.b[2743] { 1.0 } else { 0.0 };

        s.b[2744] = (p.p34 > 0.0);
        s.v[2744] = if s.b[2744] { 1.0 } else { 0.0 };

        if (s.b[2743] && s.b[2744]) {
            s.store_scaled_mul(2711, 765, 1852, s.v[709]);
            s.store_mul(2712, 765, 1854);
            s.store_mul3_lhs(2713, 765, 1852, 1848);
            s.store_mul_ad(858, A::add_scaled_value_products(s.ad_value(273), 1.0, s.ad_value(274), s.ad_value(2711), (-1.0), s.ad_value(275), A::square(s.ad_value(2711)), 1.0), A::ln(A::div(A::add_scaled_inputs(s.ad_value(2712), 1.0, s.ad_value(2713), 0.5), A::sub_scaled_inputs(s.ad_value(2712), 1.0, s.ad_value(2713), 0.5))));
            s.store_ad_value(858, A::add_scaled_product(s.ad_value(858), 1.0, A::add_scaled_product(s.ad_value(274), 1.0, s.ad_value(275), A::sub_scaled_inputs(s.ad_value(2712), 1.0, s.ad_value(2711), 2.0), 1.0), s.ad_value(2713), 1.0));
            s.store_ad_value(858, A::div_scaled_product(A::mul3(s.ad_value(716), s.ad_value(832), s.ad_value(1864)), s.ad_value(858), 1.0, s.ad_value(2711), 1.0));
        }

        if (s.b[2743] && s.b[2744]) {
            s.store_ad_value(858, {
                if (s.v[858] > 0.0) {
                    s.ad_value(858)
                } else {
                    A::constant(0.0)
                }
            });
        }

        s.b[2745] = (p.p32 > 0.0);
        s.v[2745] = if s.b[2745] { 1.0 } else { 0.0 };

        if (s.b[2743] && s.b[2745]) {
            s.store_div(2714, 1854, 1852);
            s.store_div(2715, 1853, 1854);
            s.store_scaled_div(2716, 1848, 2714, (0.5 * 0.16666666666666666));
            s.store_square(2717, 2716);
            s.store_offset_div(2718, 2714, 1865, (-1.0));
        }

        if (s.b[2743] && s.b[2745]) {
            s.store_ad_value(2719, {
                if ((1.0 - (12.0 * (s.v[2718] * s.v[2717]))) > 1e-20) {
                    A::sub_from_scalar(1.0, A::mul_scaled_output(s.ad_value(2718), s.ad_value(2717), 12.0))
                } else {
                    A::constant(1e-20)
                }
            });
        }

        if (s.b[2743] && s.b[2745]) {
            s.store_div_from_scalar_square_ad(2720, 1.0, s.ad_value(2719));
            s.store_mul3_lhs(2721, 710, 1854, 1864);
            s.store_ad_value(2722, A::add_scaled_inputs3(s.ad_value(2715), 1.0, s.ad_value(2717), 12.0, A::mul3_scaled_output(A::offset(s.ad_value(2715), 1.0), s.ad_value(2717), s.ad_value(2718), 24.0), -1.0));
        }

        if (s.b[2743] && s.b[2745]) {
            s.store_ad_value(2722, {
                if (s.v[2722] > 1e-40) {
                    s.ad_value(2722)
                } else {
                    A::constant(1e-40)
                }
            });
        }

        if (s.b[2743] && s.b[2745]) {
            s.store_mul3_lhs(2722, 2721, 2720, 2722);
        }

        s.b[2746] = (s.v[272] > 0.0);
        s.v[2746] = if s.b[2746] { 1.0 } else { 0.0 };

        if ((s.b[2743] && s.b[2745]) && s.b[2746]) {
            s.store_div(2723, 1858, 1857);
            s.store_mul_ad_product_lhs(2724, A::square(s.ad_value(2723)), s.ad_value(1848), 1848);
        }

        s.b[2747] = (s.v[0] == (-1.0));
        s.v[2747] = if s.b[2747] { 1.0 } else { 0.0 };

        if (((s.b[2743] && s.b[2745]) && s.b[2746]) && s.b[2747]) {
            s.store_div_ad_rhs(2724, 2724, A::offset(A::mul(s.ad_value(2723), s.ad_value(1848)), 1.0));
        }

        if ((s.b[2743] && s.b[2745]) && s.b[2746]) {
            s.store_ad_value(2725, A::mul_offset_rhs_scaled_output(s.ad_value(1857), A::sqrt(A::scale_offset(s.ad_value(2724), 2.0, 1.0)), 1.0, 0.5));
            s.store_div_ad_rhs(2726, 1857, A::mul(s.ad_value(2725), s.ad_value(2719)));
            s.store_mul_ad_product_lhs(2727, A::mul3(s.ad_value(804), s.ad_value(832), s.ad_value(1845)), s.ad_value(2726), 2726);
            s.store_add_scaled_inputs(2722, 2722, 1.0, 2727, 1.0 / (s.v[712]));
        }

        if (s.b[2743] && s.b[2745]) {
            s.store_sqrt_mul(856, 713, 2722);
        }

        s.b[2748] = ((((p.p50 == 1.0) && (s.v[713] > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0));
        s.v[2748] = if s.b[2748] { 1.0 } else { 0.0 };

        if (s.b[2743] && s.b[2748]) {
            s.store_sub_ad(853, A::add_scaled_product(s.ad_value(2715), 0.08333333333333333, s.ad_value(2717), A::sub_scaled_inputs(A::offset(s.ad_value(2715), 0.2), 1.0, s.ad_value(2717), 12.0), (-1.0)), A::mul3_scaled_output(s.ad_value(2717), A::sub_scaled_inputs(A::offset(s.ad_value(2715), 1.0), 1.0, s.ad_value(2717), 12.0), s.ad_value(2718), 1.6));
        }

        if (s.b[2743] && s.b[2748]) {
            s.store_ad_value(853, {
                if (s.v[853] > 1e-40) {
                    s.ad_value(853)
                } else {
                    A::constant(1e-40)
                }
            });
        }

        if (s.b[2743] && s.b[2748]) {
            s.store_mul_div_lhs(853, 2720, 2721, 853);
            s.store_mul_ad_product_rhs(2728, 2720, s.ad_value(2716), A::add_scaled_sub_value_product(1.0, A::scale(s.ad_value(2717), 12.0), 1.0, A::add_scaled_inputs_product(s.ad_value(2715), 1.0, s.ad_value(2717), 19.2, s.ad_value(2715), s.ad_value(2717), (-12.0)), s.ad_value(2718), (-1.0)));
            s.store_ad_value(854, A::div_scaled_product3(A::square(s.ad_value(1896)), s.ad_value(1892), s.ad_value(1883), 1.0, A::square(s.ad_value(1894)), 1.0));
        }

        s.b[2749] = (s.v[272] > 0.0);
        s.v[2749] = if s.b[2749] { 1.0 } else { 0.0 };

        if ((s.b[2743] && s.b[2748]) && s.b[2749]) {
            s.store_add_ad_rhs(853, 853, A::div_scaled_product_by_product(s.ad_value(2727), A::scale_offset(s.ad_value(2717), 12.0, 1.0), 1.0, s.ad_value(2721), s.ad_value(2721), (12.0 * s.v[712])));
        }

    }

    pub(super) fn stamp_transient_block_44(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[2743] && s.b[2748]) && s.b[2749]) {
            s.store_sub_ad_rhs(2728, 2728, A::div_scaled_product3(s.ad_value(2727), s.ad_value(2716), A::offset(s.ad_value(2718), 1.0), 1.0, s.ad_value(2721), s.v[712]));
        }

        if (s.b[2743] && s.b[2748]) {
            s.store_sqrt_div(2729, 713, 853);
        }

        s.b[2750] = (s.v[856] <= 0.0);
        s.v[2750] = if s.b[2750] { 1.0 } else { 0.0 };

        if ((s.b[2743] && s.b[2748]) && s.b[2750]) {
            s.store_scalar(857, 0.0);
        }

        if ((s.b[2743] && s.b[2748]) && (!s.b[2750])) {
            s.store_ad_value(857, A::div_scaled_product(s.ad_value(2728), s.ad_value(2729), 1.0, s.ad_value(856), 1.0));
        }

        if (s.b[2743] && s.b[2748]) {
            s.store_ad_value(857, {
                if (s.v[857] > 0.0) {
                    {
                        if (s.v[857] < 1.0) {
                            s.ad_value(857)
                        } else {
                            A::constant(1.0)
                        }
                    }
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.b[2743] && s.b[2748]) {
            s.store_ad_value(855, A::div_scaled_product(s.ad_value(857), s.ad_value(856), 1.0, s.ad_value(2729), 1.0));
        }

        s.store_scaled_abs(1912, 835, (2.0 * 1.6021918e-19));

        s.store_scaled_abs(1913, 836, (2.0 * 1.6021918e-19));

        s.store_scaled_abs(1914, 833, (2.0 * 1.6021918e-19));

        s.store_scaled_abs(1915, 834, (2.0 * 1.6021918e-19));

        s.store_ad_value(1916, A::mul_offset_lhs_scaled_output(s.ad_value(1873), 1.0, A::abs(s.ad_value(841)), (2.0 * 1.6021918e-19)));

        s.store_scaled_abs(1917, 842, (2.0 * 1.6021918e-19));

        s.store_scaled_abs(1918, 843, (2.0 * 1.6021918e-19));

        s.b[2751] = (s.v[825] > 0.0);
        s.v[2751] = if s.b[2751] { 1.0 } else { 0.0 };

        if s.b[2751] {
            s.store_add(859, 1912, 1914);
            s.store_add(860, 1913, 1915);
            s.copy_ad(861, 1917);
            s.store_add(862, 1918, 1916);
        }

        if (!s.b[2751]) {
            s.store_add(859, 1913, 1914);
            s.store_add(860, 1912, 1915);
            s.store_add(861, 1917, 1916);
            s.copy_ad(862, 1918);
        }

        s.b[2752] = (((p.p46 != 0.0) && (s.v[282] > 0.0)) && (s.v[1868] > 0.0));
        s.v[2752] = if s.b[2752] { 1.0 } else { 0.0 };

        if s.b[2752] {
            s.store_scaled_div(1920, 1871, 718, 4.0);
            s.store_div_ad(2730, A::sqrt(A::offset(s.ad_value(1920), 1.0)), A::offset(A::sqrt(A::offset(s.ad_value(1920), 1.1)), (-1.0)));
            s.store_scale(1920, 765, s.v[709]);
            s.store_mul(2731, 1920, 2730);
            s.store_mul_add_rhs(2732, 1920, 1870, 2730);
            s.store_mul_ad_lhs(2733, A::mul3_scaled_output(s.ad_value(1920), s.ad_value(2730), s.ad_value(1872), -1.0), 1869);
            s.store_mul_ad(863, A::add_scaled_product(s.ad_value(291), 1.0, A::add_scaled_product(s.ad_value(292), 1.0, s.ad_value(293), s.ad_value(2731), (-1.0)), s.ad_value(2731), (-1.0)), A::ln(A::div(A::add_scaled_inputs(s.ad_value(2732), 1.0, s.ad_value(2733), 0.5), A::sub_scaled_inputs(s.ad_value(2732), 1.0, s.ad_value(2733), 0.5))));
            s.store_ad_value(863, A::add_scaled_product(s.ad_value(863), 1.0, A::add_scaled_product(s.ad_value(292), 1.0, s.ad_value(293), A::sub_scaled_inputs(s.ad_value(2732), 1.0, s.ad_value(2731), 2.0), 1.0), s.ad_value(2733), 1.0));
            s.store_ad_value(863, A::div_scaled_product(A::mul3(s.ad_value(720), s.ad_value(840), s.ad_value(1864)), s.ad_value(863), 1.0, s.ad_value(2731), 1.0));
        }

        if s.b[2752] {
            s.store_ad_value(863, {
                if (s.v[863] > 0.0) {
                    s.ad_value(863)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if s.b[2752] {
            s.store_div_ad_lhs(2734, A::add_scaled_inputs(s.ad_value(1870), s.v[709], s.ad_value(2730), s.v[709]), 2730);
            s.store_ad_value(2735, A::div_scaled_product(s.ad_value(1812), s.ad_value(1870), 1.0 / (s.v[709]), A::add(s.ad_value(1870), s.ad_value(2730)), 1.0));
            s.store_ad_value(2736, A::div_scaled_product(s.ad_value(1872), s.ad_value(1869), (((-0.5) * 0.16666666666666666) * s.v[709]), s.ad_value(2734), 1.0));
            s.store_square(2737, 2736);
            s.store_scalar(2738, 0.0);
            s.store_mul(1920, 1852, 1865);
        }

        s.b[2753] = (s.v[1920] > 1e-10);
        s.v[2753] = if s.b[2753] { 1.0 } else { 0.0 };

        if (s.b[2752] && s.b[2753]) {
            s.store_offset_ad(2738, A::div_scaled_product(s.ad_value(2730), s.ad_value(2734), 1.0, s.ad_value(1920), 1.0), (-1.0));
        }

        if s.b[2752] {
            s.store_ad_value(2739, {
                if ((1.0 - (12.0 * (s.v[2738] * s.v[2737]))) > 1e-20) {
                    A::sub_from_scalar(1.0, A::mul_scaled_output(s.ad_value(2738), s.ad_value(2737), 12.0))
                } else {
                    A::constant(1e-20)
                }
            });
        }

        if s.b[2752] {
            s.store_div_from_scalar_square_ad(2740, 1.0, s.ad_value(2739));
            s.store_mul_ad_affine_product_lhs(2741, s.ad_value(711), A::add(s.ad_value(1870), s.ad_value(2730)), s.v[709], 0.0, 1864);
            s.store_ad_value(2742, A::add_scaled_inputs3(s.ad_value(2735), 1.0, s.ad_value(2737), 12.0, A::mul3_scaled_output(A::offset(s.ad_value(2735), 1.0), s.ad_value(2737), s.ad_value(2738), 24.0), -1.0));
        }

        if s.b[2752] {
            s.store_ad_value(2742, {
                if (s.v[2742] > 1e-40) {
                    s.ad_value(2742)
                } else {
                    A::constant(1e-40)
                }
            });
        }

        if s.b[2752] {
            s.store_mul3_lhs(2742, 2741, 2740, 2742);
            s.store_sqrt_mul(864, 721, 2742);
        }

    }

    pub(super) fn stamp_reactive_block_0(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[984] = (p.p37 >= 0.0);
        s.v[984] = if s.b[984] { 1.0 } else { 0.0 };

        if s.b[984] {
            s.store_scalar(0, 1.0);
        }

        if (!s.b[984]) {
            s.store_scalar(0, (-1.0));
        }

        s.v[761] = (8.8541878176e-12 * 11.8);

        s.v[344] = (273.15 + p.p38);

        s.v[468] = 0.0;

        s.b[985] = (p.p920 > 0.5);
        s.v[985] = if s.b[985] { 1.0 } else { 0.0 };

        if s.b[985] {
            s.store_scalar(468, 1.0);
        }

        if (!s.b[985]) {
            s.store_scalar(468, 0.0);
        }

        s.v[358] = (273.15 + p.p816);

        s.v[361] = (1.3806505e-23 / 1.6021918e-19);

        s.v[362] = (s.v[361] * s.v[358]);

        s.v[363] = (1.0 / s.v[362]);

        s.v[369] = ((-((0.000702 * s.v[358]) * s.v[358])) / (1108.0 + s.v[358]));

        s.v[372] = (p.p827 + s.v[369]);

        s.v[373] = (p.p828 + s.v[369]);

        s.v[374] = (p.p829 + s.v[369]);

        s.v[402] = (1.0 - p.p824);

        s.v[403] = (1.0 - p.p825);

        s.v[404] = (1.0 - p.p826);

        s.v[405] = (1.0 / s.v[402]);

        s.v[406] = (1.0 / s.v[403]);

        s.v[407] = (1.0 / s.v[404]);

        s.v[417] = (s.v[761] / p.p818);

        s.v[418] = ((p.p836 * s.v[761]) / p.p819);

        s.v[419] = ((p.p837 * s.v[761]) / p.p820);

        s.v[420] = (1.0 / s.v[417]);

        s.v[421] = (1.0 / s.v[418]);

        s.v[422] = (1.0 / s.v[419]);

        s.v[423] = (1.0 / p.p821);

        s.v[424] = (1.0 / p.p822);

        s.v[425] = (1.0 / p.p823);

        s.v[438] = (1.0 - (1.0 / p.p817));

        s.v[442] = (1.0 / p.p853);

        s.v[443] = (1.0 / p.p854);

        s.v[444] = (1.0 / p.p855);

        s.b[986] = ((((p.p859 != 1.0) || (p.p860 != 1.0)) || (p.p861 != 1.0)) || (p.p862 != 1.0));
        s.v[986] = if s.b[986] { 1.0 } else { 0.0 };

        if s.b[986] {
            s.store_scalar(467, 1.0);
        }

        if (!s.b[986]) {
            s.store_scalar(467, 0.0);
        }

        s.b[987] = (s.v[467] == 1.0);
        s.v[987] = if s.b[987] { 1.0 } else { 0.0 };

        if s.b[987] {
            s.store_scalar(451, (if ((p.p820 * p.p859) > 1e-18) { (p.p820 * p.p859) } else { 1e-18 }));
        }

        if s.b[987] {
            s.store_scalar(452, (if ((p.p823 * p.p860) > 0.05) { (p.p823 * p.p860) } else { 0.05 }));
        }

        if s.b[987] {
            s.store_scalar(453, (if ((if ((p.p826 * p.p861) > 0.05) { (p.p826 * p.p861) } else { 0.05 }) < 0.95) { (if ((p.p826 * p.p861) > 0.05) { (p.p826 * p.p861) } else { 0.05 }) } else { 0.95 }));
        }

        if s.b[987] {
            s.store_scalar(454, (p.p829 * p.p862));
            s.store_offset(456, 454, s.v[369]);
            s.store_sub_from_scalar(461, 1.0, 453);
            s.store_div_from_scalar(462, 1.0, 461);
        }

        s.b[988] = (p.p44 == 0.0);
        s.v[988] = if s.b[988] { 1.0 } else { 0.0 };

        if s.b[988] {
            s.store_scalar(499, p.p818);
            s.store_scalar(500, p.p819);
            s.store_scalar(501, p.p820);
            s.store_scalar(502, p.p821);
            s.store_scalar(503, p.p822);
            s.store_scalar(504, p.p823);
            s.store_scalar(505, p.p824);
            s.store_scalar(506, p.p825);
            s.store_scalar(507, p.p826);
            s.store_scalar(508, p.p827);
            s.store_scalar(509, p.p828);
            s.store_scalar(510, p.p829);
            s.store_scalar(511, p.p830);
            s.store_scalar(512, p.p831);
            s.store_scalar(513, p.p832);
            s.store_scalar(516, p.p833);
            s.store_scalar(517, p.p834);
            s.store_scalar(518, p.p835);
            s.store_scalar(514, p.p836);
            s.store_scalar(515, p.p837);
            s.store_scalar(519, p.p838);
            s.store_scalar(520, p.p839);
            s.store_scalar(521, p.p840);
            s.store_scalar(522, p.p841);
            s.store_scalar(523, p.p842);
            s.store_scalar(524, p.p843);
            s.store_scalar(525, p.p844);
            s.store_scalar(526, p.p845);
            s.store_scalar(527, p.p846);
            s.store_scalar(528, p.p847);
            s.store_scalar(529, p.p848);
            s.store_scalar(530, p.p849);
            s.store_scalar(531, p.p850);
            s.store_scalar(532, p.p851);
            s.store_scalar(533, p.p852);
            s.store_scalar(534, p.p853);
            s.store_scalar(535, p.p854);
            s.store_scalar(536, p.p855);
            s.store_scalar(537, p.p856);
            s.store_scalar(538, p.p857);
            s.store_scalar(539, p.p858);
            s.store_scalar(547, p.p922);
            s.store_scalar(630, p.p865);
            s.store_scalar(631, p.p866);
            s.store_scalar(632, p.p867);
            s.store_scalar(633, p.p868);
            s.store_scalar(540, p.p859);
            s.store_scalar(541, p.p860);
            s.store_scalar(542, p.p861);
            s.store_scalar(543, p.p862);
            s.store_scalar(544, p.p863);
            s.store_scalar(545, p.p864);
        }

        if (!s.b[988]) {
            s.store_scalar(499, p.p869);
            s.store_scalar(500, p.p870);
            s.store_scalar(501, p.p871);
            s.store_scalar(502, p.p872);
            s.store_scalar(503, p.p873);
            s.store_scalar(504, p.p874);
            s.store_scalar(505, p.p875);
            s.store_scalar(506, p.p876);
            s.store_scalar(507, p.p877);
            s.store_scalar(508, p.p878);
            s.store_scalar(509, p.p879);
            s.store_scalar(510, p.p880);
            s.store_scalar(511, p.p881);
            s.store_scalar(512, p.p882);
            s.store_scalar(513, p.p883);
            s.store_scalar(516, p.p884);
            s.store_scalar(517, p.p885);
            s.store_scalar(518, p.p886);
            s.store_scalar(514, p.p887);
            s.store_scalar(515, p.p888);
            s.store_scalar(519, p.p889);
            s.store_scalar(520, p.p890);
            s.store_scalar(521, p.p891);
            s.store_scalar(522, p.p892);
            s.store_scalar(523, p.p893);
            s.store_scalar(524, p.p894);
            s.store_scalar(525, p.p895);
            s.store_scalar(526, p.p896);
            s.store_scalar(527, p.p897);
            s.store_scalar(528, p.p898);
            s.store_scalar(529, p.p899);
            s.store_scalar(530, p.p900);
            s.store_scalar(531, p.p901);
            s.store_scalar(532, p.p902);
            s.store_scalar(533, p.p903);
            s.store_scalar(534, p.p904);
            s.store_scalar(535, p.p905);
            s.store_scalar(536, p.p906);
            s.store_scalar(537, p.p907);
            s.store_scalar(538, p.p908);
            s.store_scalar(539, p.p909);
            s.store_scalar(547, p.p924);
            s.store_scalar(630, p.p916);
            s.store_scalar(631, p.p917);
        }

    }

    pub(super) fn stamp_reactive_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if (!s.b[988]) {
            s.store_scalar(632, p.p918);
            s.store_scalar(633, p.p919);
            s.store_scalar(540, p.p910);
            s.store_scalar(541, p.p911);
            s.store_scalar(542, p.p912);
            s.store_scalar(543, p.p913);
            s.store_scalar(544, p.p914);
            s.store_scalar(545, p.p915);
        }

        s.store_offset(548, 508, s.v[369]);

        s.store_offset(549, 509, s.v[369]);

        s.store_offset(550, 510, s.v[369]);

        s.store_sub_from_scalar(569, 1.0, 505);

        s.store_sub_from_scalar(570, 1.0, 506);

        s.store_sub_from_scalar(571, 1.0, 507);

        s.store_div_from_scalar(572, 1.0, 569);

        s.store_div_from_scalar(573, 1.0, 570);

        s.store_div_from_scalar(574, 1.0, 571);

        s.store_div_from_scalar(584, s.v[761], 499);

        s.store_scaled_div(585, 514, 500, s.v[761]);

        s.store_scaled_div(586, 515, 501, s.v[761]);

        s.store_div_from_scalar(587, 1.0, 584);

        s.store_div_from_scalar(588, 1.0, 585);

        s.store_div_from_scalar(589, 1.0, 586);

        s.store_div_from_scalar(590, 1.0, 502);

        s.store_div_from_scalar(591, 1.0, 503);

        s.store_div_from_scalar(592, 1.0, 504);

        s.store_div_from_scalar(608, 1.0, 534);

        s.store_div_from_scalar(609, 1.0, 535);

        s.store_div_from_scalar(610, 1.0, 536);

        s.b[989] = ((((s.v[540] != 1.0) || (s.v[541] != 1.0)) || (s.v[542] != 1.0)) || (s.v[543] != 1.0));
        s.v[989] = if s.b[989] { 1.0 } else { 0.0 };

        if s.b[989] {
            s.store_scalar(629, 1.0);
        }

        if (!s.b[989]) {
            s.store_scalar(629, 0.0);
        }

        s.b[990] = (s.v[629] == 1.0);
        s.v[990] = if s.b[990] { 1.0 } else { 0.0 };

        if s.b[990] {
            s.store_ad_value(614, {
                if ((s.v[501] * s.v[540]) > 1e-18) {
                    A::mul(s.ad_value(501), s.ad_value(540))
                } else {
                    A::constant(1e-18)
                }
            });
        }

        if s.b[990] {
            s.store_ad_value(615, {
                if ((s.v[504] * s.v[541]) > 0.05) {
                    A::mul(s.ad_value(504), s.ad_value(541))
                } else {
                    A::constant(0.05)
                }
            });
        }

        if s.b[990] {
            s.store_ad_value(616, {
                if ((if ((s.v[507] * s.v[542]) > 0.05) { (s.v[507] * s.v[542]) } else { 0.05 }) < 0.95) {
                    {
                        if ((s.v[507] * s.v[542]) > 0.05) {
                            A::mul(s.ad_value(507), s.ad_value(542))
                        } else {
                            A::constant(0.05)
                        }
                    }
                } else {
                    A::constant(0.95)
                }
            });
        }

        if s.b[990] {
            s.store_mul(617, 510, 543);
            s.store_offset(619, 617, s.v[369]);
            s.store_sub_from_scalar(624, 1.0, 616);
            s.store_div_from_scalar(625, 1.0, 624);
        }

        s.v[345] = ((ctx_temp + p.p55) + p.p35);

        s.v[346] = (s.v[345] / s.v[344]);

        s.v[347] = (s.v[345] - s.v[344]);

        s.v[348] = ((s.v[345] * 1.3806505e-23) / 1.6021918e-19);

        s.v[349] = (1.0 / s.v[348]);

        s.v[350] = s.v[345];

        s.v[351] = (s.v[350] * s.v[350]);

        s.v[352] = (s.v[350] - s.v[344]);

        s.v[353] = (s.v[344] / s.v[350]);

        s.v[354] = ((s.v[353]) as f64).ln();

        s.v[709] = ((s.v[350] * 1.3806505e-23) / 1.6021918e-19);

        s.v[355] = (1.0 / s.v[709]);

        s.v[356] = ((1.179 - (9.025e-5 * s.v[350])) - (3.05e-7 * s.v[351]));

        s.v[357] = ((((1.045 + (0.00045 * s.v[350])) * ((0.523 + (0.0014 * s.v[350])) - (1.48e-6 * s.v[351]))) * s.v[351]) / 90000.0);

        if (!(s.v[357] > 0.001)) {
            s.store_scalar(357, 0.001);
        }

        s.v[359] = (((ctx_temp + p.p55) + p.p35)).max((273.15 + (-250.0)));

        s.v[360] = (s.v[359] / s.v[358]);

        s.v[364] = (s.v[361] * s.v[359]);

        s.v[365] = (1.0 / s.v[364]);

        s.v[370] = ((-((0.000702 * s.v[359]) * s.v[359])) / (1108.0 + s.v[359]));

        s.v[375] = (p.p827 + s.v[370]);

        s.v[376] = (p.p828 + s.v[370]);

        s.v[377] = (p.p829 + s.v[370]);

        s.v[378] = (((s.v[360]) as f64).powf(1.5) * (((0.5 * ((s.v[372] * s.v[363]) - (s.v[375] * s.v[365])))) as f64).exp());

        s.v[379] = (((s.v[360]) as f64).powf(1.5) * (((0.5 * ((s.v[373] * s.v[363]) - (s.v[376] * s.v[365])))) as f64).exp());

        s.v[380] = (((s.v[360]) as f64).powf(1.5) * (((0.5 * ((s.v[374] * s.v[363]) - (s.v[377] * s.v[365])))) as f64).exp());

        s.v[381] = ((p.p830 * s.v[378]) * s.v[378]);

        s.v[382] = ((p.p831 * s.v[379]) * s.v[379]);

        s.v[383] = ((p.p832 * s.v[380]) * s.v[380]);

        s.v[384] = ((p.p821 * s.v[360]) - ((2.0 * s.v[364]) * ((s.v[378]) as f64).ln()));

        s.v[385] = ((p.p822 * s.v[360]) - ((2.0 * s.v[364]) * ((s.v[379]) as f64).ln()));

        s.v[386] = ((p.p823 * s.v[360]) - ((2.0 * s.v[364]) * ((s.v[380]) as f64).ln()));

        s.v[387] = (s.v[384] + (s.v[364] * (((1.0 + ((((0.05 - s.v[384]) * s.v[365])) as f64).exp())) as f64).ln()));

        s.v[388] = (s.v[385] + (s.v[364] * (((1.0 + ((((0.05 - s.v[385]) * s.v[365])) as f64).exp())) as f64).ln()));

        s.v[389] = (s.v[386] + (s.v[364] * (((1.0 + ((((0.05 - s.v[386]) * s.v[365])) as f64).exp())) as f64).ln()));

        s.v[399] = (1.0 / s.v[387]);

        s.v[400] = (1.0 / s.v[388]);

        s.v[401] = (1.0 / s.v[389]);

        s.v[408] = (p.p818 * (((p.p821 * s.v[399])) as f64).powf(p.p824));

        s.v[409] = (p.p819 * (((p.p822 * s.v[400])) as f64).powf(p.p825));

        s.v[410] = (p.p820 * (((p.p823 * s.v[401])) as f64).powf(p.p826));

        s.v[411] = ((s.v[408] * s.v[387]) * s.v[405]);

        s.v[412] = ((s.v[409] * s.v[388]) * s.v[406]);

        s.v[413] = ((s.v[410] * s.v[389]) * s.v[407]);

        s.v[414] = (2.0 * s.v[408]);

        s.v[415] = (2.0 * s.v[409]);

        s.v[416] = (2.0 * s.v[410]);

        s.v[426] = ((0.5 * s.v[375])).max(s.v[364]);

        s.v[427] = ((0.5 * s.v[376])).max(s.v[364]);

        s.v[428] = ((0.5 * s.v[377])).max(s.v[364]);

        s.v[429] = (s.v[426] * s.v[365]);

        s.v[430] = (s.v[427] * s.v[365]);

        s.v[431] = (s.v[428] * s.v[365]);

        s.v[432] = (((((((32.0 * p.p841) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[426] * s.v[426]) * s.v[426]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[433] = (((((((32.0 * p.p842) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[427] * s.v[427]) * s.v[427]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[434] = (((((((32.0 * p.p843) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[428] * s.v[428]) * s.v[428]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[435] = (p.p847 * (1.0 + (p.p850 * (s.v[359] - s.v[358]))));

        s.v[436] = (p.p848 * (1.0 + (p.p851 * (s.v[359] - s.v[358]))));

        s.v[437] = (p.p849 * (1.0 + (p.p852 * (s.v[359] - s.v[358]))));

        if (!(s.v[435] > 0.0)) {
            s.store_scalar(435, 0.0);
        }

        if (!(s.v[436] > 0.0)) {
            s.store_scalar(436, 0.0);
        }

        if (!(s.v[437] > 0.0)) {
            s.store_scalar(437, 0.0);
        }

        s.b[1010] = (s.v[467] == 1.0);
        s.v[1010] = if s.b[1010] { 1.0 } else { 0.0 };

        if s.b[1010] {
            s.store_offset(455, 454, s.v[370]);
            s.store_scale_ad(457, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(456), s.v[363], s.ad_value(455), s.v[365]), 0.5), ((s.v[360]) as f64).powf(1.5));
            s.store_ad_value(458, A::sub_scaled_inputs(s.ad_value(452), s.v[360], A::ln(s.ad_value(457)), (2.0 * s.v[364])));
            s.store_ad_value(459, A::add_scaled_inputs(s.ad_value(458), 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(458), (-s.v[365]), ((0.05) * (s.v[365])))), s.v[364]));
            s.store_div_from_scalar(460, 1.0, 459);
            s.store_mul_pow_ad_rhs(463, 451, A::mul(s.ad_value(452), s.ad_value(460)), s.ad_value(453));
            s.store_mul3_lhs(464, 463, 459, 462);
            s.store_scale(465, 463, 2.0);
        }

        s.store_offset(551, 508, s.v[370]);

        s.store_offset(552, 509, s.v[370]);

        s.store_offset(553, 510, s.v[370]);

        s.store_scale_ad(554, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(548), s.v[363], s.ad_value(551), s.v[365]), 0.5), ((s.v[360]) as f64).powf(1.5));

        s.store_scale_ad(555, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(549), s.v[363], s.ad_value(552), s.v[365]), 0.5), ((s.v[360]) as f64).powf(1.5));

        s.store_scale_ad(556, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(550), s.v[363], s.ad_value(553), s.v[365]), 0.5), ((s.v[360]) as f64).powf(1.5));

        s.store_mul3_lhs(557, 511, 554, 554);

        s.store_mul3_lhs(558, 512, 555, 555);

        s.store_mul3_lhs(559, 513, 556, 556);

        s.store_ad_value(560, A::sub_scaled_inputs(s.ad_value(502), s.v[360], A::ln(s.ad_value(554)), (2.0 * s.v[364])));

        s.store_ad_value(561, A::sub_scaled_inputs(s.ad_value(503), s.v[360], A::ln(s.ad_value(555)), (2.0 * s.v[364])));

        s.store_ad_value(562, A::sub_scaled_inputs(s.ad_value(504), s.v[360], A::ln(s.ad_value(556)), (2.0 * s.v[364])));

        s.store_ad_value(563, A::add_scaled_inputs(s.ad_value(560), 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(560), (-s.v[365]), ((0.05) * (s.v[365])))), s.v[364]));

        s.store_ad_value(564, A::add_scaled_inputs(s.ad_value(561), 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(561), (-s.v[365]), ((0.05) * (s.v[365])))), s.v[364]));

        s.store_ad_value(565, A::add_scaled_inputs(s.ad_value(562), 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(562), (-s.v[365]), ((0.05) * (s.v[365])))), s.v[364]));

        s.store_div_from_scalar(566, 1.0, 563);

        s.store_div_from_scalar(567, 1.0, 564);

        s.store_div_from_scalar(568, 1.0, 565);

        s.store_mul_pow_ad_rhs(575, 499, A::mul(s.ad_value(502), s.ad_value(566)), s.ad_value(505));

        s.store_mul_pow_ad_rhs(576, 500, A::mul(s.ad_value(503), s.ad_value(567)), s.ad_value(506));

        s.store_mul_pow_ad_rhs(577, 501, A::mul(s.ad_value(504), s.ad_value(568)), s.ad_value(507));

        s.store_mul3_lhs(578, 575, 563, 572);

        s.store_mul3_lhs(579, 576, 564, 573);

        s.store_mul3_lhs(580, 577, 565, 574);

        s.store_scale(581, 575, 2.0);

        s.store_scale(582, 576, 2.0);

        s.store_scale(583, 577, 2.0);

        s.store_max_with_scalar_ad(593, A::scale(s.ad_value(551), 0.5), s.v[364]);

        s.store_max_with_scalar_ad(594, A::scale(s.ad_value(552), 0.5), s.v[364]);

        s.store_max_with_scalar_ad(595, A::scale(s.ad_value(553), 0.5), s.v[364]);

        s.store_scale(596, 593, s.v[365]);

        s.store_scale(597, 594, s.v[365]);

        s.store_scale(598, 595, s.v[365]);

        s.store_scaled_sqrt_ad(599, A::mul3_scaled_output(s.ad_value(522), A::square(s.ad_value(593)), s.ad_value(593), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_scaled_sqrt_ad(600, A::mul3_scaled_output(s.ad_value(523), A::square(s.ad_value(594)), s.ad_value(594), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_scaled_sqrt_ad(601, A::mul3_scaled_output(s.ad_value(524), A::square(s.ad_value(595)), s.ad_value(595), ((32.0 * 9.1093826e-31) * 1.6021918e-19)), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_mul_ad_rhs(602, 528, A::scale_offset(s.ad_value(531), (s.v[359] - s.v[358]), 1.0));

        s.store_mul_ad_rhs(603, 529, A::scale_offset(s.ad_value(532), (s.v[359] - s.v[358]), 1.0));

        s.store_mul_ad_rhs(604, 530, A::scale_offset(s.ad_value(533), (s.v[359] - s.v[358]), 1.0));

        if (!(s.v[602] > 0.0)) {
            s.store_scalar(602, 0.0);
        }

        if (!(s.v[603] > 0.0)) {
            s.store_scalar(603, 0.0);
        }

        if (!(s.v[604] > 0.0)) {
            s.store_scalar(604, 0.0);
        }

        s.b[1011] = (s.v[629] == 1.0);
        s.v[1011] = if s.b[1011] { 1.0 } else { 0.0 };

        if s.b[1011] {
            s.store_offset(618, 617, s.v[370]);
            s.store_scale_ad(620, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(619), s.v[363], s.ad_value(618), s.v[365]), 0.5), ((s.v[360]) as f64).powf(1.5));
            s.store_ad_value(621, A::sub_scaled_inputs(s.ad_value(615), s.v[360], A::ln(s.ad_value(620)), (2.0 * s.v[364])));
            s.store_ad_value(622, A::add_scaled_inputs(s.ad_value(621), 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(621), (-s.v[365]), ((0.05) * (s.v[365])))), s.v[364]));
            s.store_div_from_scalar(623, 1.0, 622);
            s.store_mul_pow_ad_rhs(626, 614, A::mul(s.ad_value(615), s.ad_value(623)), s.ad_value(616));
            s.store_mul3_lhs(627, 626, 622, 625);
            s.store_scale(628, 626, 2.0);
        }

        s.v[1] = 1.0;

        s.v[2] = 1.0;

        s.v[306] = 0.0;

        s.v[307] = 0.0;

        s.v[3] = p.p0;

        s.v[4] = p.p1;

        s.v[5] = p.p2;

        s.v[6] = p.p3;

        s.v[7] = p.p4;

        s.v[8] = p.p8;

        s.v[640] = p.p19;

        s.v[641] = p.p20;

        s.v[642] = p.p21;

        s.v[667] = p.p22;

        s.v[668] = p.p23;

        s.v[669] = p.p24;

        s.v[643] = p.p25;

        s.v[644] = p.p26;

        s.v[670] = p.p27;

        s.v[671] = p.p28;

        s.v[10] = p.p14;

        s.b[1012] = (p.p39 > 0.0);
        s.v[1012] = if s.b[1012] { 1.0 } else { 0.0 };

        if s.b[1012] {
            s.store_scalar(1, (if (p.p9 > 1.0) { p.p9 } else { 1.0 }));
        }

        if s.b[1012] {
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

        s.v[302] = (1e-6 / s.v[3]);

        s.store_div_from_scalar(303, 1e-6, 4);

        s.store_offset_scaled(304, 303, ((p.p188) * ((p.p186 * (1.0 + (p.p187 * s.v[302]))))), (p.p186 * (1.0 + (p.p187 * s.v[302]))));

        s.store_offset_scaled(305, 303, ((p.p192) * ((p.p190 * (1.0 + (p.p191 * s.v[302]))))), (p.p190 * (1.0 + (p.p191 * s.v[302]))));

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (((s.v[3] + s.v[304]) - (2.0 * p.p189)) > 1e-9) {
            s.store_offset(306, 304, ((s.v[3]) + ((-(2.0 * p.p189)))));
        } else {
            s.store_scalar(306, 1e-9);
        }

        if (((s.v[4] + s.v[305]) - (2.0 * p.p193)) > 1e-9) {
            s.store_offset_add(307, 4, 305, (-(2.0 * p.p193)));
        } else {
            s.store_scalar(307, 1e-9);
        }

        s.store_div_from_scalar(308, 1e-6, 306);

        s.store_square(309, 308);

        s.store_div_from_scalar(310, 1e-6, 307);

        s.store_div_from_scalar(311, 1.0, 310);

        s.store_mul(312, 308, 310);

        s.store_div_from_scalar(313, 1.0, 312);

        if ((((s.v[3] + s.v[304]) - (2.0 * p.p189)) + p.p194) > 1e-9) {
            s.store_offset(314, 304, ((((s.v[3]) + ((-(2.0 * p.p189))))) + (p.p194)));
        } else {
            s.store_scalar(314, 1e-9);
        }

        if ((((s.v[4] + s.v[305]) - (2.0 * p.p193)) + p.p195) > 1e-9) {
            s.store_offset_add(315, 4, 305, (((-(2.0 * p.p193))) + (p.p195)));
        } else {
            s.store_scalar(315, 1e-9);
        }

        s.store_scale(316, 315, 1000000.0);

        if (((s.v[3] + s.v[304]) + p.p194) > 1e-9) {
            s.store_offset(317, 304, ((s.v[3]) + (p.p194)));
        } else {
            s.store_scalar(317, 1e-9);
        }

        if (((s.v[4] + s.v[305]) + p.p195) > 1e-9) {
            s.store_offset_add(318, 4, 305, p.p195);
        } else {
            s.store_scalar(318, 1e-9);
        }

        s.store_scale(319, 317, 1000000.0);

        s.store_scale(320, 318, 1000000.0);

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

        s.b[1013] = param_given[121];
        s.v[1013] = if s.b[1013] { 1.0 } else { 0.0 };

        if s.b[1013] {
            s.store_scalar(105, p.p121);
        }

        s.v[106] = p.p120;

        s.b[1014] = param_given[122];
        s.v[1014] = if s.b[1014] { 1.0 } else { 0.0 };

        if s.b[1014] {
            s.store_scalar(106, p.p122);
        }

        s.copy_ad(107, 105);

        s.b[1015] = param_given[123];
        s.v[1015] = if s.b[1015] { 1.0 } else { 0.0 };

        if s.b[1015] {
            s.store_scalar(107, p.p123);
        }

        s.copy_ad(108, 106);

        s.b[1016] = param_given[124];
        s.v[1016] = if s.b[1016] { 1.0 } else { 0.0 };

        if s.b[1016] {
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

        s.b[1017] = param_given[137];
        s.v[1017] = if s.b[1017] { 1.0 } else { 0.0 };

        if s.b[1017] {
            s.store_scalar(121, p.p137);
        }

        s.v[122] = p.p103;

        s.b[1018] = param_given[138];
        s.v[1018] = if s.b[1018] { 1.0 } else { 0.0 };

        if s.b[1018] {
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

        s.b[1019] = (p.p39 > 0.0);
        s.v[1019] = if s.b[1019] { 1.0 } else { 0.0 };

        if s.b[1019] {
            s.store_ad_value(40, A::add_scaled_inputs3_offset(A::powf(s.ad_value(308), p.p198), p.p197, s.ad_value(310), p.p199, s.ad_value(312), p.p200, p.p196));
            s.store_ad_value(41, A::add_scaled_inputs3_offset(s.ad_value(308), p.p202, s.ad_value(310), p.p203, s.ad_value(312), p.p204, p.p201));
            s.store_scalar(42, p.p205);
            s.store_scalar(43, p.p206);
            s.store_scalar(44, p.p207);
        }

        if s.b[1019] {
            s.store_scale_ad(325, {
                if ((1.0 + ((p.p209 * s.v[310]) * (((1.0 + (s.v[307] / p.p210))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(310), p.p209, A::ln(A::scale_offset(s.ad_value(307), 1.0 / (p.p210), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p208);
        }

        if s.b[1019] {
            s.store_scale_ad(326, {
                if ((1.0 + ((p.p212 * s.v[310]) * (((1.0 + (s.v[307] / p.p213))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(310), p.p212, A::ln(A::scale_offset(s.ad_value(307), 1.0 / (p.p213), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p211);
        }

        if s.b[1019] {
            s.store_scale_ad(327, {
                if ((1.0 + ((p.p215 * s.v[310]) * (((1.0 + (s.v[307] / p.p213))) as f64).ln())) > 0.001) {
                    A::offset(A::mul_scaled_lhs(s.ad_value(310), p.p215, A::ln(A::scale_offset(s.ad_value(307), 1.0 / (p.p213), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p214);
        }

        s.b[1020] = (s.v[306] > (2.0 * s.v[327]));
        s.v[1020] = if s.b[1020] { 1.0 } else { 0.0 };

        if (s.b[1019] && s.b[1020]) {
            s.store_scalar(328, 75000000000.0);
            s.store_sub_ad(329, A::sqrt(A::add_scaled_inputs(s.ad_value(325), 1.0, s.ad_value(326), 0.5)), A::sqrt(s.ad_value(325)));
            s.store_ad_value(330, A::add_scaled_product(A::sqrt(s.ad_value(325)), 1.0, s.ad_value(328), A::ln(A::offset(A::mul_offset_rhs(A::div_scaled_inputs(s.ad_value(327), 2.0, s.ad_value(306), 1.0), A::exp(A::div(s.ad_value(329), s.ad_value(328))), (-1.0)), 1.0)), 1.0));
            s.store_square(330, 330);
        }

        s.b[1021] = (s.v[306] >= s.v[327]);
        s.v[1021] = if s.b[1021] { 1.0 } else { 0.0 };

        if ((s.b[1019] && (!s.b[1020])) && s.b[1021]) {
            s.store_add_ad_rhs(330, 325, A::div_scaled_product(s.ad_value(326), s.ad_value(327), 1.0, s.ad_value(306), 1.0));
        }

        if ((s.b[1019] && (!s.b[1020])) && (!s.b[1021])) {
            s.store_add_ad_rhs(330, 325, A::mul_sub_from_scalar_rhs(s.ad_value(326), 2.0, A::div(s.ad_value(306), s.ad_value(327))));
        }

        if s.b[1019] {
            s.store_mul_ad_rhs(45, 330, A::sub_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(308), p.p216)), 1.0, s.ad_value(309), p.p217));
            s.store_ad_value(46, A::add_scaled_inputs3_offset(A::powf(s.ad_value(308), p.p220), p.p219, s.ad_value(310), p.p221, s.ad_value(312), p.p222, p.p218));
            s.store_scalar(47, p.p223);
            s.store_scalar(48, p.p224);
            s.store_ad_value(49, A::add_scaled_inputs3_offset(A::powf(s.ad_value(308), p.p227), p.p226, s.ad_value(310), p.p228, s.ad_value(312), p.p229, p.p225));
        }

        if s.b[1019] {
            s.store_scale_ad(50, {
                if (1e-6 > (1.0 + (p.p231 * s.v[308]))) {
                    A::constant(1e-6)
                } else {
                    A::scale_offset(s.ad_value(308), p.p231, 1.0)
                }
            }, p.p230);
        }

        if s.b[1019] {
            s.store_scalar(55, p.p232);
            s.store_scalar(56, p.p233);
            s.store_scalar(57, p.p236);
            s.store_scalar(58, p.p237);
            s.store_ad_value(51, A::mul3(A::scale_offset(A::powf(s.ad_value(308), p.p240), p.p239, p.p238), A::scale_offset(s.ad_value(310), p.p241, 1.0), A::scale_offset(s.ad_value(312), p.p242, 1.0)));
            s.store_scalar(52, p.p244);
            s.store_scalar(53, p.p243);
            s.store_scalar(54, p.p245);
            s.store_scaled_mul_ad(62, A::powf(s.ad_value(308), p.p247), A::scale_offset(s.ad_value(310), p.p248, 1.0), p.p246);
            s.store_scalar(63, p.p250);
            s.store_scalar(64, p.p249);
            s.store_scaled_mul_ad(59, A::powf(s.ad_value(308), p.p252), A::scale_offset(s.ad_value(310), p.p253, 1.0), p.p251);
            s.store_scalar(60, p.p255);
            s.store_scalar(61, p.p254);
            s.store_offset_scaled(331, 310, ((p.p258) * (p.p257)), p.p257);
        }

        if s.b[1019] {
            s.store_scale_ad(332, {
                if ((1.0 + (p.p260 * s.v[310])) > 0.001) {
                    A::scale_offset(s.ad_value(310), p.p260, 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p259);
        }

        if s.b[1019] {
            s.store_add_ad(333, A::offset(A::mul_sub_from_scalar_rhs(A::div_scaled_product(s.ad_value(331), s.ad_value(332), 1.0, s.ad_value(306), 1.0), 1.0, A::exp(A::div_scaled_inputs(s.ad_value(306), -1.0, s.ad_value(332), 1.0))), 1.0), A::mul_sub_from_scalar_rhs(A::div_from_scalar((p.p261 * p.p262), s.ad_value(306)), 1.0, A::exp_scaled_input(s.ad_value(306), (-1.0 / (p.p262)))));
        }

    }
}

#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_30(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_add_scaled_inputs3_offset(781, s.ad_value(1856), 1.0, s.ad_value(1883), -1.0, s.ad_value(1851), 1.0, (-0.01));
            s.store_scaled_sub(782, 1883, 1851, (4.0 * 0.01));
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4(1856, s.ad_value(1883), 1.0, s.ad_value(1851), (-1.0), s.ad_value(781), 0.5, s.ad_value(782), 0.5);
            s.store_div_scaled_product_denominator_ad(1884, 1851, 622, -1.0, A::add(s.ad_value(622), s.ad_value(964)), 1.0);
            s.store_offset_sub(1830, 965, 1831, 1e-15);
            s.store_scalar(79, 0.0);
            s.store_scalar(1846, 0.2);
            s.copy_ad(1849, 1856);
            s.copy_ad(1852, 1847);
            s.copy_ad(1854, 1884);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_31(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign38070_loop_guard: usize = 0;
        while {
            let assign38070_cond_e43943: f64 = if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign38070_cond_e43943 != 0.0
        } {
            assign38070_loop_guard += 1;
            assert!(assign38070_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
                s.store_mul_sub_ad_rhs(1854, 1897, A::add_scaled_product(s.ad_value(1883), 1.0, s.ad_value(1898), s.ad_value(1852), 1.0), s.ad_value(1851));
                s.store_mul(1838, 1897, 1898);
                s.store_sub(335, 1852, 1854);
            }
            s.b[1921] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1921] = if s.b[1921] { 1.0 } else { 0.0 };
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) {
                s.store_sub_from_scalar(781, 0.1, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.1 * 0.1));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1922] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1922] = if s.b[1922] { 1.0 } else { 0.0 };
            s.b[1923] = (2.0 == 1.0);
            s.v[1923] = if s.b[1923] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) && s.b[1923]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1924] = (2.0 == 2.0);
            s.v[1924] = if s.b[1924] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) && (!s.b[1923])) && s.b[1924]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1925] = (2.0 == 4.0);
            s.v[1925] = if s.b[1925] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) && (!s.b[1923])) && (!s.b[1924])) && s.b[1925]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1926] = (2.0 == 8.0);
            s.v[1926] = if s.b[1926] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) && (!s.b[1923])) && (!s.b[1924])) && (!s.b[1925])) && s.b[1926]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign38070_body29_loop_guard: usize = 0;
            while {
                let assign38070_body29_cond_e44306: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38070_body29_cond_e44306 != 0.0
            } {
                assign38070_body29_loop_guard += 1;
                assert!(assign38070_body29_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && (!s.b[1922])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) {
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1921])) {
                s.copy_ad(336, 335);
                s.store_scalar(334, 1.0);
            }
            if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
                s.store_sqrt_mul(1826, 1904, 336);
            }
            s.b[1927] = ((s.v[1826] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.v[1927] = if s.b[1927] { 1.0 } else { 0.0 };
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) {
                s.store_offset_sub(781, 1826, 965, 1e-8);
                s.store_square(722, 781);
                s.store_scalar(723, (1e-8 * 1e-8));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1928] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1928] = if s.b[1928] { 1.0 } else { 0.0 };
            s.b[1929] = (2.0 == 1.0);
            s.v[1929] = if s.b[1929] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) && s.b[1929]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1930] = (2.0 == 2.0);
            s.v[1930] = if s.b[1930] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) && (!s.b[1929])) && s.b[1930]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1931] = (2.0 == 4.0);
            s.v[1931] = if s.b[1931] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) && (!s.b[1929])) && (!s.b[1930])) && s.b[1931]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1932] = (2.0 == 8.0);
            s.v[1932] = if s.b[1932] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) && (!s.b[1929])) && (!s.b[1930])) && (!s.b[1931])) && s.b[1932]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign38070_body65_loop_guard: usize = 0;
            while {
                let assign38070_body65_cond_e44791: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38070_body65_cond_e44791 != 0.0
            } {
                assign38070_body65_loop_guard += 1;
                assert!(assign38070_body65_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && (!s.b[1928])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
                s.store_add_ad_lhs(1826, A::offset(s.ad_value(965), (-1e-8)), 780);
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) {
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1927])) {
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1927])) {
                s.store_scalar(337, 1.0);
            }
            if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
                s.store_mul(1862, 1826, 1901);
                s.store_mul_ad_product_lhs(1840, A::div_from_scalar(1.034943e-10, s.ad_value(1826)), s.ad_value(334), 337);
                s.store_mul_ad_product_lhs(1842, A::div_from_scalar((-1.034943e-10), s.ad_value(1826)), s.ad_value(334), 337);
            }
            s.b[1933] = (p.p49 == 0.0);
            s.v[1933] = if s.b[1933] { 1.0 } else { 0.0 };
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1933]) {
                s.store_add_ad_lhs(1835, A::div_scaled_inputs_product(s.ad_value(1899), 1.0, s.ad_value(1834), 1.0, s.ad_value(965), s.ad_value(1831), (-2.0), s.ad_value(1904), 1.0), 1849);
                s.store_scalar(1836, 1.0);
                s.store_scalar(1837, 0.0);
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1933])) {
                s.store_add_ad_rhs(1835, 1849, A::div_scaled_add_product(s.ad_value(1899), 1.0, s.ad_value(1826), A::sub_scaled_inputs(s.ad_value(1826), 1.0, s.ad_value(965), 2.0), 1.0, s.ad_value(1904), 1.0));
                s.store_scalar(1836, 1.0);
                s.store_ad_value(1837, A::mul_sub_from_scalar_rhs(A::add_scaled_product(s.ad_value(334), 1.0, A::div(s.ad_value(965), s.ad_value(1826)), s.ad_value(334), (-1.0)), 1.0, s.ad_value(1838)));
            }
            s.b[1934] = ((s.v[1835] > (s.v[1847] - s.v[1846])) && (s.v[1846] >= 0.0));
            s.v[1934] = if s.b[1934] { 1.0 } else { 0.0 };
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) {
                s.store_add_scaled_inputs3(781, s.ad_value(1835), 1.0, s.ad_value(1847), (-1.0), s.ad_value(1846), 1.0);
                s.store_square(722, 781);
                s.store_square(723, 1846);
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1935] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[1935] = if s.b[1935] { 1.0 } else { 0.0 };
            s.b[1936] = (4.0 == 1.0);
            s.v[1936] = if s.b[1936] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) && s.b[1936]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1937] = (4.0 == 2.0);
            s.v[1937] = if s.b[1937] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) && (!s.b[1936])) && s.b[1937]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1938] = (4.0 == 4.0);
            s.v[1938] = if s.b[1938] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) && (!s.b[1936])) && (!s.b[1937])) && s.b[1938]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1939] = (4.0 == 8.0);
            s.v[1939] = if s.b[1939] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) && (!s.b[1936])) && (!s.b[1937])) && (!s.b[1938])) && s.b[1939]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign38070_body114_loop_guard: usize = 0;
            while {
                let assign38070_body114_cond_e45464: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38070_body114_cond_e45464 != 0.0
            } {
                assign38070_body114_loop_guard += 1;
                assert!(assign38070_body114_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && (!s.b[1935])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                    }
                });
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1846, 726);
                s.store_div_scaled_product3(334, s.ad_value(1846), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0);
                s.store_add_scaled_inputs3(1835, s.ad_value(1847), 1.0, s.ad_value(1846), (-1.0), s.ad_value(780), 1.0);
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) {
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1934])) {
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1934])) {
                s.store_scalar(334, 1.0);
            }
            if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
                s.store_mul(1836, 1836, 334);
                s.store_mul(1837, 1837, 334);
                s.store_add_scaled_inputs3(335, s.ad_value(1854), 1.0, s.ad_value(1883), (-1.0), s.ad_value(1851), 1.0);
            }
            s.b[1940] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1940] = if s.b[1940] { 1.0 } else { 0.0 };
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) {
                s.store_sub_from_scalar(781, 0.1, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.1 * 0.1));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1941] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1941] = if s.b[1941] { 1.0 } else { 0.0 };
            s.b[1942] = (2.0 == 1.0);
            s.v[1942] = if s.b[1942] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) && s.b[1942]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1943] = (2.0 == 2.0);
            s.v[1943] = if s.b[1943] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) && (!s.b[1942])) && s.b[1943]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1944] = (2.0 == 4.0);
            s.v[1944] = if s.b[1944] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) && (!s.b[1942])) && (!s.b[1943])) && s.b[1944]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1945] = (2.0 == 8.0);
            s.v[1945] = if s.b[1945] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) && (!s.b[1942])) && (!s.b[1943])) && (!s.b[1944])) && s.b[1945]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign38070_body152_loop_guard: usize = 0;
            while {
                let assign38070_body152_cond_e45972: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38070_body152_cond_e45972 != 0.0
            } {
                assign38070_body152_loop_guard += 1;
                assert!(assign38070_body152_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && (!s.b[1941])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(337, 725, 726, 0.1, 770, 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) {
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1940])) {
                s.copy_ad(336, 335);
                s.store_scalar(337, 1.0);
            }
            if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
                s.store_sqrt_mul(1828, 1905, 336);
                s.store_mul_neg_lhs(1863, 1828, 1900);
                s.store_mul_div_from_scalar_lhs(1844, (-1.034943e-10), 1828, 337);
                s.store_mul_sub_rhs(335, 154, 1849, 1852);
                s.store_exp(336, 335);
            }
            s.b[1946] = (s.v[1849] >= s.v[1852]);
            s.v[1946] = if s.b[1946] { 1.0 } else { 0.0 };
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1946]) {
                s.store_mul_scaled_ad_rhs(1858, 209, -1.0, A::sqrt(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15)));
                s.store_mul_ad(1893, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(1858), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
                s.store_neg(1895, 1893);
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1946])) {
                s.store_exp_ad(337, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1849), s.ad_value(1883))));
                s.store_exp_ad(338, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1852), s.ad_value(1883))));
                s.store_mul_sqrt_ad_rhs(1858, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1858, 1.0);
                s.store_mul_add_ad_rhs(1893, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1895, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] != 0.0)) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) {
                s.store_add_scaled_inputs3(1866, A::add_scaled_product(s.ad_value(1858), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1849)), 1.0), 1.0, s.ad_value(1862), 1.0, s.ad_value(1863), 1.0);
                s.store_sub(1867, 1893, 185);
                s.store_add_ad_rhs(1868, 1895, A::add_scaled_value_products(s.ad_value(1840), 1.0, s.ad_value(1842), s.ad_value(1838), 1.0, s.ad_value(1844), s.ad_value(1838), 1.0));
                s.store_sub(1869, 1852, 1835);
                s.store_neg(1870, 1836);
                s.store_sub_from_scalar(1871, 1.0, 1837);
                s.store_add_scaled_products_indices(1872, 1867, 1871, 1.0, 1868, 1870, (-1.0));
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) {
                s.store_ad_value(1873, {
                    if (s.v[1872] > 0.0) {
                        A::div_scalar_offset_denominator(1.0, s.ad_value(1872), 1e-25, 1.0)
                    } else {
                        A::div_scalar_offset_denominator(1.0, s.ad_value(1872), (-1e-25), 1.0)
                    }
                });
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) {
                s.copy_ad(1874, 1871);
                s.store_neg(1875, 1868);
                s.store_neg(1876, 1870);
                s.copy_ad(1877, 1867);
                s.store_mul_scaled_ad_rhs(1878, 1873, -1.0, A::add_scaled_products(s.ad_value(1874), s.ad_value(1866), 1.0, s.ad_value(1875), s.ad_value(1869), 1.0));
                s.store_mul_scaled_ad_rhs(1879, 1873, -1.0, A::add_scaled_products(s.ad_value(1876), s.ad_value(1866), 1.0, s.ad_value(1877), s.ad_value(1869), 1.0));
                s.store_abs(335, 1878);
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) {
                s.store_ad_value(335, {
                    if (s.v[335] < ((s.v[1879]) as f64).abs()) {
                        A::abs(s.ad_value(1879))
                    } else {
                        s.ad_value(335)
                    }
                });
            }
            s.b[1947] = (s.v[335] > 0.1);
            s.v[1947] = if s.b[1947] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) && s.b[1947]) {
                s.store_mul_div_from_scalar_rhs(1878, 1878, 0.1, 335);
                s.store_mul_div_from_scalar_rhs(1879, 1879, 0.1, 335);
            }
            s.b[1948] = (s.v[335] < 1e-12);
            s.v[1948] = if s.b[1948] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) && s.b[1948]) {
                s.store_scalar(79, 1.0);
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) {
                s.store_add(1849, 1849, 1878);
                s.store_add(1852, 1852, 1879);
            }
            if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
                s.store_offset(97, 97, 1.0);
            }
        }

    }

    pub(super) fn stamp_reactive_block_32(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_mul_sub_rhs(335, 154, 1849, 1852);
            s.store_exp(336, 335);
        }

        s.b[1950] = (s.v[1849] >= s.v[1852]);
        s.v[1950] = if s.b[1950] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1950]) {
            s.copy_ad(1888, 1858);
            s.store_scalar(1891, 0.0);
            s.store_scalar(1860, 0.0);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1950])) {
            s.store_scalar(1888, 0.0);
            s.store_mul_sqrt_ad_rhs(1891, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
        }

        s.b[1951] = (s.v[1832] > s.v[965]);
        s.v[1951] = if s.b[1951] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1950])) && s.b[1951]) {
            s.store_scalar(1860, 0.0);
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1950])) && (!s.b[1951])) {
            s.store_mul_sqrt_ad_rhs(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1849), s.ad_value(1883)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1852), s.ad_value(1883)))))), s.ad_value(335)));
            s.store_add_scaled_product_right_ad(1860, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));
        }

        s.b[1952] = (((s.v[1849] - s.v[1847]) < s.v[1907]) && (s.v[1907] >= 0.0));
        s.v[1952] = if s.b[1952] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) {
            s.store_add_scaled_inputs3(781, s.ad_value(1907), 1.0, s.ad_value(1849), -1.0, s.ad_value(1847), 1.0);
            s.store_square(722, 781);
            s.store_square(723, 1907);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1953] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1953] = if s.b[1953] { 1.0 } else { 0.0 };

        s.b[1954] = (4.0 == 1.0);
        s.v[1954] = if s.b[1954] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) && s.b[1954]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1955] = (4.0 == 2.0);
        s.v[1955] = if s.b[1955] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) && (!s.b[1954])) && s.b[1955]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1956] = (4.0 == 4.0);
        s.v[1956] = if s.b[1956] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) && (!s.b[1954])) && (!s.b[1955])) && s.b[1956]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1957] = (4.0 == 8.0);
        s.v[1957] = if s.b[1957] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) && (!s.b[1954])) && (!s.b[1955])) && (!s.b[1956])) && s.b[1957]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign38510_loop_guard: usize = 0;
        while {
            let assign38510_cond_e47297: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign38510_cond_e47297 != 0.0
        } {
            assign38510_loop_guard += 1;
            assert!(assign38510_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && (!s.b[1953])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                }
            });
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1907, 726);
            s.store_div_scaled_product3(334, s.ad_value(1907), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0);
            s.store_sub(336, 1907, 780);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) {
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1952])) {
            s.store_sub(336, 1849, 1847);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_offset_ad(338, A::add_scaled_product(A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0)), 1e-15);
            s.store_mul_scaled_ad_rhs(1885, 209, -1.0, A::sqrt(s.ad_value(338)));
            s.copy_ad(349, 790);
        }

        s.b[1958] = (s.v[790] > 1e-6);
        s.v[1958] = if s.b[1958] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) {
            s.store_scalar(344, 1e-25);
            s.store_offset_mul_ad(338, s.ad_value(1903), A::sub(s.ad_value(85), s.ad_value(344)), 1.0);
            s.store_offset(339, 1903, 1.0);
        }

        s.b[1959] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));
        s.v[1959] = if s.b[1959] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) {
            s.store_sub(781, 339, 338);
            s.store_square(722, 781);
            s.store_square(723, 339);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1960] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1960] = if s.b[1960] { 1.0 } else { 0.0 };

        s.b[1961] = (2.0 == 1.0);
        s.v[1961] = if s.b[1961] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) && s.b[1961]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1962] = (2.0 == 2.0);
        s.v[1962] = if s.b[1962] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) && (!s.b[1961])) && s.b[1962]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1963] = (2.0 == 4.0);
        s.v[1963] = if s.b[1963] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) && (!s.b[1961])) && (!s.b[1962])) && s.b[1963]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1964] = (2.0 == 8.0);
        s.v[1964] = if s.b[1964] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) && (!s.b[1961])) && (!s.b[1962])) && (!s.b[1963])) && s.b[1964]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign38930_loop_guard: usize = 0;
        while {
            let assign38930_cond_e47900: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign38930_cond_e47900 != 0.0
        } {
            assign38930_loop_guard += 1;
            assert!(assign38930_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && (!s.b[1960])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 339, 726);
            s.store_div_scaled_product3(334, s.ad_value(339), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0);
            s.store_sub(338, 339, 780);
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) {
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && (!s.b[1959])) {
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && (!s.b[1959])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) {
            s.store_sqrt(337, 338);
            s.store_add_ad_rhs(344, 85, A::mul_sub_from_scalar_rhs(s.ad_value(1902), 1.0, s.ad_value(337)));
        }

        s.b[1965] = ((s.v[344] < 1.0) && (1.0 >= 0.0));
        s.v[1965] = if s.b[1965] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) {
            s.store_sub_from_scalar(781, 1.0, 344);
            s.store_square(722, 781);
            s.store_scalar(723, 1.0);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[1966] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1966] = if s.b[1966] { 1.0 } else { 0.0 };

        s.b[1967] = (2.0 == 1.0);
        s.v[1967] = if s.b[1967] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) && s.b[1966]) && s.b[1967]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1968] = (2.0 == 2.0);
        s.v[1968] = if s.b[1968] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) && s.b[1966]) && (!s.b[1967])) && s.b[1968]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1969] = (2.0 == 4.0);
        s.v[1969] = if s.b[1969] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) && s.b[1966]) && (!s.b[1967])) && (!s.b[1968])) && s.b[1969]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1970] = (2.0 == 8.0);
        s.v[1970] = if s.b[1970] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) && s.b[1966]) && (!s.b[1967])) && (!s.b[1968])) && (!s.b[1969])) && s.b[1970]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) && s.b[1966]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign39300_loop_guard: usize = 0;
        while {
            let assign39300_cond_e48464: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) && s.b[1966]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign39300_cond_e48464 != 0.0
        } {
            assign39300_loop_guard += 1;
            assert!(assign39300_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) && s.b[1966]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

    }

    pub(super) fn stamp_reactive_block_33(
        s: &mut ReactiveScratch,
    ) {
        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) && (!s.b[1966])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1.0);
            s.store_div_scaled_product_indices(334, 725, 726, 1.0, 770, 1.0);
            s.store_sub_from_scalar(344, 1.0, 780);
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) {
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && (!s.b[1965])) {
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && (!s.b[1965])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) {
            s.store_div(335, 790, 344);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) {
            s.store_ad_value(336, {
                if (s.v[335] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(335), A::offset(s.ad_value(658), (-1.0)))
                }
            });
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) {
            s.store_offset_mul(337, 336, 335, 1.0);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) {
            s.store_ad_value(338, {
                if (s.v[337] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)))
                }
            });
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) {
            s.store_mul(340, 338, 337);
            s.store_div(348, 790, 340);
            s.copy_ad(790, 348);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1958])) {
            s.copy_ad(348, 790);
        }

        s.b[1971] = (s.v[790] < 0.0);
        s.v[1971] = if s.b[1971] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1971]) {
            s.copy_ad(1850, 1849);
            s.copy_ad(1855, 1854);
            s.copy_ad(1853, 1852);
            s.copy_ad(1861, 1860);
            s.copy_ad(1889, 1888);
            s.copy_ad(1886, 1885);
            s.copy_ad(1864, 1862);
            s.copy_ad(1865, 1863);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
            s.copy_ad(1833, 1832);
            s.copy_ad(1848, 790);
            s.store_add_scaled_inputs3_offset(781, s.ad_value(1849), 1.0, s.ad_value(1848), 1.0, s.ad_value(85), -1.0, (-0.01));
            s.store_scaled_add(782, 1849, 1848, (4.0 * 0.01));
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4(1857, s.ad_value(1849), 1.0, s.ad_value(1848), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5));
            s.store_add_scaled_inputs3_offset(781, s.ad_value(1857), 1.0, s.ad_value(1883), -1.0, s.ad_value(1851), 1.0, (-0.01));
            s.store_scaled_sub(782, 1883, 1851, (4.0 * 0.01));
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4(1857, s.ad_value(1883), 1.0, s.ad_value(1851), (-1.0), s.ad_value(781), 0.5, s.ad_value(782), 0.5);
            s.store_mul(212, 209, 186);
            s.store_square(213, 212);
            s.store_offset_ad(332, A::div_scaled_offset_numerator(A::mul(s.ad_value(154), A::sub(s.ad_value(85), s.ad_value(1883))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0), 1.0);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
            s.store_ad_value(332, {
                if (s.v[332] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(332)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
            s.store_add_ad_rhs(92, 85, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 0.5));
            s.store_scalar(79, 0.0);
            s.copy_ad(1850, 1857);
            s.copy_ad(1853, 1848);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_34(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign39790_loop_guard: usize = 0;
        while {
            let assign39790_cond_e49244: f64 = if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign39790_cond_e49244 != 0.0
        } {
            assign39790_loop_guard += 1;
            assert!(assign39790_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
                s.store_mul_sub_ad_rhs(1855, 1897, A::add_scaled_product(s.ad_value(1883), 1.0, s.ad_value(1898), s.ad_value(1853), 1.0), s.ad_value(1851));
                s.store_mul(1839, 1897, 1898);
                s.store_sub(335, 1853, 1855);
            }
            s.b[1972] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1972] = if s.b[1972] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) {
                s.store_sub_from_scalar(781, 0.1, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.1 * 0.1));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1973] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1973] = if s.b[1973] { 1.0 } else { 0.0 };
            s.b[1974] = (2.0 == 1.0);
            s.v[1974] = if s.b[1974] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) && s.b[1973]) && s.b[1974]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1975] = (2.0 == 2.0);
            s.v[1975] = if s.b[1975] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) && s.b[1973]) && (!s.b[1974])) && s.b[1975]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1976] = (2.0 == 4.0);
            s.v[1976] = if s.b[1976] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) && s.b[1973]) && (!s.b[1974])) && (!s.b[1975])) && s.b[1976]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1977] = (2.0 == 8.0);
            s.v[1977] = if s.b[1977] { 1.0 } else { 0.0 };
            if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) && s.b[1973]) && (!s.b[1974])) && (!s.b[1975])) && (!s.b[1976])) && s.b[1977]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) && s.b[1973]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign39790_body29_loop_guard: usize = 0;
            while {
                let assign39790_body29_cond_e49679: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) && s.b[1973]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign39790_body29_cond_e49679 != 0.0
            } {
                assign39790_body29_loop_guard += 1;
                assert!(assign39790_body29_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) && s.b[1973]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) && (!s.b[1973])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) {
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (!s.b[1972])) {
                s.copy_ad(336, 335);
                s.store_scalar(334, 1.0);
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
                s.store_sqrt_mul(1827, 1904, 336);
            }
            s.b[1978] = ((s.v[1827] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.v[1978] = if s.b[1978] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) {
                s.store_offset_sub(781, 1827, 965, 1e-8);
                s.store_square(722, 781);
                s.store_scalar(723, (1e-8 * 1e-8));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1979] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1979] = if s.b[1979] { 1.0 } else { 0.0 };
            s.b[1980] = (2.0 == 1.0);
            s.v[1980] = if s.b[1980] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) && s.b[1979]) && s.b[1980]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1981] = (2.0 == 2.0);
            s.v[1981] = if s.b[1981] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) && s.b[1979]) && (!s.b[1980])) && s.b[1981]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1982] = (2.0 == 4.0);
            s.v[1982] = if s.b[1982] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) && s.b[1979]) && (!s.b[1980])) && (!s.b[1981])) && s.b[1982]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1983] = (2.0 == 8.0);
            s.v[1983] = if s.b[1983] { 1.0 } else { 0.0 };
            if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) && s.b[1979]) && (!s.b[1980])) && (!s.b[1981])) && (!s.b[1982])) && s.b[1983]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) && s.b[1979]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign39790_body65_loop_guard: usize = 0;
            while {
                let assign39790_body65_cond_e50260: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) && s.b[1979]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign39790_body65_cond_e50260 != 0.0
            } {
                assign39790_body65_loop_guard += 1;
                assert!(assign39790_body65_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) && s.b[1979]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) && (!s.b[1979])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
                s.store_add_ad_lhs(1827, A::offset(s.ad_value(965), (-1e-8)), 780);
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) {
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (!s.b[1978])) {
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (!s.b[1978])) {
                s.store_scalar(337, 1.0);
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
                s.store_mul(1864, 1827, 1901);
                s.store_mul_ad_product_lhs(1841, A::div_from_scalar(1.034943e-10, s.ad_value(1827)), s.ad_value(334), 337);
                s.store_mul_ad_product_lhs(1843, A::div_from_scalar((-1.034943e-10), s.ad_value(1827)), s.ad_value(334), 337);
            }
            s.b[1984] = (p.p49 == 0.0);
            s.v[1984] = if s.b[1984] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1984]) {
                s.store_add_ad_lhs(1835, A::div_scaled_inputs_product(s.ad_value(1899), 1.0, s.ad_value(1834), 1.0, s.ad_value(965), s.ad_value(1831), (-2.0), s.ad_value(1904), 1.0), 1850);
                s.store_scalar(1836, 1.0);
                s.store_scalar(1837, 0.0);
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (!s.b[1984])) {
                s.store_add_ad_rhs(1835, 1850, A::div_scaled_add_product(s.ad_value(1899), 1.0, s.ad_value(1827), A::sub_scaled_inputs(s.ad_value(1827), 1.0, s.ad_value(965), 2.0), 1.0, s.ad_value(1904), 1.0));
                s.store_scalar(1836, 1.0);
                s.store_ad_value(1837, A::mul_sub_from_scalar_rhs(A::add_scaled_product(s.ad_value(334), 1.0, A::div(s.ad_value(965), s.ad_value(1827)), s.ad_value(334), (-1.0)), 1.0, s.ad_value(1839)));
            }
            s.b[1985] = ((s.v[1835] > (s.v[1848] - s.v[1846])) && (s.v[1846] >= 0.0));
            s.v[1985] = if s.b[1985] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) {
                s.store_add_scaled_inputs3(781, s.ad_value(1835), 1.0, s.ad_value(1848), (-1.0), s.ad_value(1846), 1.0);
                s.store_square(722, 781);
                s.store_square(723, 1846);
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1986] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[1986] = if s.b[1986] { 1.0 } else { 0.0 };
            s.b[1987] = (4.0 == 1.0);
            s.v[1987] = if s.b[1987] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) && s.b[1986]) && s.b[1987]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1988] = (4.0 == 2.0);
            s.v[1988] = if s.b[1988] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) && s.b[1986]) && (!s.b[1987])) && s.b[1988]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1989] = (4.0 == 4.0);
            s.v[1989] = if s.b[1989] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) && s.b[1986]) && (!s.b[1987])) && (!s.b[1988])) && s.b[1989]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1990] = (4.0 == 8.0);
            s.v[1990] = if s.b[1990] { 1.0 } else { 0.0 };
            if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) && s.b[1986]) && (!s.b[1987])) && (!s.b[1988])) && (!s.b[1989])) && s.b[1990]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) && s.b[1986]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign39790_body114_loop_guard: usize = 0;
            while {
                let assign39790_body114_cond_e51065: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) && s.b[1986]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign39790_body114_cond_e51065 != 0.0
            } {
                assign39790_body114_loop_guard += 1;
                assert!(assign39790_body114_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) && s.b[1986]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) && (!s.b[1986])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                    }
                });
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1846, 726);
                s.store_div_scaled_product3(334, s.ad_value(1846), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0);
                s.store_add_scaled_inputs3(1835, s.ad_value(1848), 1.0, s.ad_value(1846), (-1.0), s.ad_value(780), 1.0);
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) {
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (!s.b[1985])) {
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (!s.b[1985])) {
                s.store_scalar(334, 1.0);
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
                s.store_mul(1836, 1836, 334);
                s.store_mul(1837, 1837, 334);
                s.store_add_scaled_inputs3(335, s.ad_value(1855), 1.0, s.ad_value(1883), (-1.0), s.ad_value(1851), 1.0);
            }
            s.b[1991] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1991] = if s.b[1991] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) {
                s.store_sub_from_scalar(781, 0.1, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.1 * 0.1));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[1992] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1992] = if s.b[1992] { 1.0 } else { 0.0 };
            s.b[1993] = (2.0 == 1.0);
            s.v[1993] = if s.b[1993] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) && s.b[1992]) && s.b[1993]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1994] = (2.0 == 2.0);
            s.v[1994] = if s.b[1994] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) && s.b[1992]) && (!s.b[1993])) && s.b[1994]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1995] = (2.0 == 4.0);
            s.v[1995] = if s.b[1995] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) && s.b[1992]) && (!s.b[1993])) && (!s.b[1994])) && s.b[1995]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1996] = (2.0 == 8.0);
            s.v[1996] = if s.b[1996] { 1.0 } else { 0.0 };
            if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) && s.b[1992]) && (!s.b[1993])) && (!s.b[1994])) && (!s.b[1995])) && s.b[1996]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) && s.b[1992]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign39790_body152_loop_guard: usize = 0;
            while {
                let assign39790_body152_cond_e51675: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) && s.b[1992]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign39790_body152_cond_e51675 != 0.0
            } {
                assign39790_body152_loop_guard += 1;
                assert!(assign39790_body152_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) && s.b[1992]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) && (!s.b[1992])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_div_scaled_product_indices(337, 725, 726, 0.1, 770, 1.0);
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) {
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (!s.b[1991])) {
                s.copy_ad(336, 335);
                s.store_scalar(337, 1.0);
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
                s.store_sqrt_mul(1829, 1905, 336);
                s.store_mul_neg_lhs(1865, 1829, 1900);
                s.store_mul_div_from_scalar_lhs(1845, (-1.034943e-10), 1829, 337);
                s.store_mul_sub_rhs(335, 154, 1850, 1853);
                s.store_exp(336, 335);
            }
            s.b[1997] = (s.v[1850] >= s.v[1853]);
            s.v[1997] = if s.b[1997] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1997]) {
                s.store_mul_scaled_ad_rhs(1859, 209, -1.0, A::sqrt(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15)));
                s.store_mul_ad(1894, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(1859), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
                s.store_neg(1896, 1894);
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (!s.b[1997])) {
                s.store_exp_ad(337, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1850), s.ad_value(1883))));
                s.store_exp_ad(338, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1853), s.ad_value(1883))));
                s.store_mul_sqrt_ad_rhs(1859, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1859, 1.0);
                s.store_mul_add_ad_rhs(1894, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1896, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (s.v[79] != 0.0)) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (s.v[79] == 0.0)) {
                s.store_add_scaled_inputs3(1866, A::add_scaled_product(s.ad_value(1859), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1850)), 1.0), 1.0, s.ad_value(1864), 1.0, s.ad_value(1865), 1.0);
                s.store_sub(1867, 1894, 185);
                s.store_add_ad_rhs(1868, 1896, A::add_scaled_value_products(s.ad_value(1841), 1.0, s.ad_value(1843), s.ad_value(1839), 1.0, s.ad_value(1845), s.ad_value(1839), 1.0));
                s.store_sub(1869, 1853, 1835);
                s.store_neg(1870, 1836);
                s.store_sub_from_scalar(1871, 1.0, 1837);
                s.store_add_scaled_products_indices(1872, 1867, 1871, 1.0, 1868, 1870, (-1.0));
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (s.v[79] == 0.0)) {
                s.store_ad_value(1873, {
                    if (s.v[1872] > 0.0) {
                        A::div_scalar_offset_denominator(1.0, s.ad_value(1872), 1e-25, 1.0)
                    } else {
                        A::div_scalar_offset_denominator(1.0, s.ad_value(1872), (-1e-25), 1.0)
                    }
                });
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (s.v[79] == 0.0)) {
                s.copy_ad(1874, 1871);
                s.store_neg(1875, 1868);
                s.store_neg(1876, 1870);
                s.copy_ad(1877, 1867);
                s.store_mul_scaled_ad_rhs(1878, 1873, -1.0, A::add_scaled_products(s.ad_value(1874), s.ad_value(1866), 1.0, s.ad_value(1875), s.ad_value(1869), 1.0));
                s.store_mul_scaled_ad_rhs(1879, 1873, -1.0, A::add_scaled_products(s.ad_value(1876), s.ad_value(1866), 1.0, s.ad_value(1877), s.ad_value(1869), 1.0));
                s.store_abs(335, 1878);
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (s.v[79] == 0.0)) {
                s.store_ad_value(335, {
                    if (s.v[335] < ((s.v[1879]) as f64).abs()) {
                        A::abs(s.ad_value(1879))
                    } else {
                        s.ad_value(335)
                    }
                });
            }
            s.b[1998] = (s.v[335] > 0.1);
            s.v[1998] = if s.b[1998] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (s.v[79] == 0.0)) && s.b[1998]) {
                s.store_mul_div_from_scalar_rhs(1878, 1878, 0.1, 335);
                s.store_mul_div_from_scalar_rhs(1879, 1879, 0.1, 335);
            }
            s.b[1999] = (s.v[335] < 1e-12);
            s.v[1999] = if s.b[1999] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (s.v[79] == 0.0)) && s.b[1999]) {
                s.store_scalar(79, 1.0);
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (s.v[79] == 0.0)) {
                s.store_add(1850, 1850, 1878);
                s.store_add(1853, 1853, 1879);
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
                s.store_offset(97, 97, 1.0);
            }
        }

    }

    pub(super) fn stamp_reactive_block_35(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
            s.store_mul_sub_rhs(335, 154, 1850, 1853);
            s.store_exp(336, 335);
        }

        s.b[2001] = (s.v[1850] >= s.v[1853]);
        s.v[2001] = if s.b[2001] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2001]) {
            s.copy_ad(1889, 1859);
            s.store_scalar(1892, 0.0);
            s.store_scalar(1861, 0.0);
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (!s.b[2001])) {
            s.store_scalar(1889, 0.0);
            s.store_mul_sqrt_ad_rhs(1892, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
        }

        s.b[2002] = (s.v[1833] > s.v[965]);
        s.v[2002] = if s.b[2002] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (!s.b[2001])) && s.b[2002]) {
            s.store_scalar(1861, 0.0);
        }

        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (!s.b[2001])) && (!s.b[2002])) {
            s.store_mul_sqrt_ad_rhs(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1850), s.ad_value(1883)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1853), s.ad_value(1883)))))), s.ad_value(335)));
            s.store_add_scaled_product_right_ad(1861, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));
        }

        s.b[2003] = (((s.v[1850] - s.v[1848]) < s.v[1907]) && (s.v[1907] >= 0.0));
        s.v[2003] = if s.b[2003] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) {
            s.store_add_scaled_inputs3(781, s.ad_value(1907), 1.0, s.ad_value(1850), -1.0, s.ad_value(1848), 1.0);
            s.store_square(722, 781);
            s.store_square(723, 1907);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2004] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[2004] = if s.b[2004] { 1.0 } else { 0.0 };

        s.b[2005] = (4.0 == 1.0);
        s.v[2005] = if s.b[2005] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) && s.b[2004]) && s.b[2005]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2006] = (4.0 == 2.0);
        s.v[2006] = if s.b[2006] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) && s.b[2004]) && (!s.b[2005])) && s.b[2006]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2007] = (4.0 == 4.0);
        s.v[2007] = if s.b[2007] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) && s.b[2004]) && (!s.b[2005])) && (!s.b[2006])) && s.b[2007]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2008] = (4.0 == 8.0);
        s.v[2008] = if s.b[2008] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) && s.b[2004]) && (!s.b[2005])) && (!s.b[2006])) && (!s.b[2007])) && s.b[2008]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) && s.b[2004]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign40230_loop_guard: usize = 0;
        while {
            let assign40230_cond_e53246: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) && s.b[2004]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign40230_cond_e53246 != 0.0
        } {
            assign40230_loop_guard += 1;
            assert!(assign40230_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) && s.b[2004]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) && (!s.b[2004])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                }
            });
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1907, 726);
            s.store_div_scaled_product3(334, s.ad_value(1907), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0);
            s.store_sub(336, 1907, 780);
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) {
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (!s.b[2003])) {
            s.store_sub(336, 1850, 1848);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
            s.store_offset_ad(338, A::add_scaled_product(A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0)), 1e-15);
            s.store_mul_scaled_ad_rhs(1886, 209, -1.0, A::sqrt(s.ad_value(338)));
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.copy_ad(87, 1849);
            s.copy_ad(91, 1850);
            s.store_sub(94, 1850, 1849);
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 1.0 / ((p.p263 * 0.1))));
            s.store_offset_ad(782, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0)), 1.0);
            s.store_offset_ad(783, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0)), (1.0 / 2.0));
            s.store_div_from_scalar(110, (p.p263 * 0.1), 782);
            s.store_div_scaled_inputs(336, s.ad_value(783), (-2.0), A::square(s.ad_value(782)), 1.0);
        }

        s.b[2009] = ((s.v[110] < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2009] = if s.b[2009] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) {
            s.store_sub_from_scalar(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 110);
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2010] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2010] = if s.b[2010] { 1.0 } else { 0.0 };

        s.b[2011] = (2.0 == 1.0);
        s.v[2011] = if s.b[2011] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) && s.b[2010]) && s.b[2011]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2012] = (2.0 == 2.0);
        s.v[2012] = if s.b[2012] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) && s.b[2010]) && (!s.b[2011])) && s.b[2012]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2013] = (2.0 == 4.0);
        s.v[2013] = if s.b[2013] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) && s.b[2010]) && (!s.b[2011])) && (!s.b[2012])) && s.b[2013]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2014] = (2.0 == 8.0);
        s.v[2014] = if s.b[2014] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) && s.b[2010]) && (!s.b[2011])) && (!s.b[2012])) && (!s.b[2013])) && s.b[2014]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) && s.b[2010]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign40690_loop_guard: usize = 0;
        while {
            let assign40690_cond_e53976: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) && s.b[2010]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign40690_cond_e53976 != 0.0
        } {
            assign40690_loop_guard += 1;
            assert!(assign40690_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) && s.b[2010]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) && (!s.b[2010])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);
            s.store_sub_from_scalar(110, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) {
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2009])) {
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2009])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_add(109, 87, 110);
        }

        s.b[2015] = (((s.v[109] - s.v[1847]) < s.v[1907]) && (s.v[1907] >= 0.0));
        s.v[2015] = if s.b[2015] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) {
            s.store_add_scaled_inputs3(781, s.ad_value(1907), 1.0, s.ad_value(109), -1.0, s.ad_value(1847), 1.0);
            s.store_square(722, 781);
            s.store_square(723, 1907);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2016] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[2016] = if s.b[2016] { 1.0 } else { 0.0 };

        s.b[2017] = (4.0 == 1.0);
        s.v[2017] = if s.b[2017] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) && s.b[2016]) && s.b[2017]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2018] = (4.0 == 2.0);
        s.v[2018] = if s.b[2018] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) && s.b[2016]) && (!s.b[2017])) && s.b[2018]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2019] = (4.0 == 4.0);
        s.v[2019] = if s.b[2019] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) && s.b[2016]) && (!s.b[2017])) && (!s.b[2018])) && s.b[2019]) {
            s.store_scalar(720, 3.0);
        }

    }

    pub(super) fn stamp_reactive_block_36(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2020] = (4.0 == 8.0);
        s.v[2020] = if s.b[2020] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) && s.b[2016]) && (!s.b[2017])) && (!s.b[2018])) && (!s.b[2019])) && s.b[2020]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) && s.b[2016]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign41090_loop_guard: usize = 0;
        while {
            let assign41090_cond_e54524: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) && s.b[2016]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign41090_cond_e54524 != 0.0
        } {
            assign41090_loop_guard += 1;
            assert!(assign41090_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) && s.b[2016]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) && (!s.b[2016])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                }
            });
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1907, 726);
            s.store_div_scaled_product3(334, s.ad_value(1907), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0);
            s.store_sub(336, 1907, 780);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) {
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2015])) {
            s.store_sub(336, 109, 1847);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_offset_ad(338, A::add_scaled_product(A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0)), 1e-15);
            s.store_mul_scaled_ad_rhs(1887, 209, -1.0, A::sqrt(s.ad_value(338)));
        }

        s.b[2026] = (s.v[1832] > s.v[965]);
        s.v[2026] = if s.b[2026] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2026]) {
            s.copy_ad(981, 1830);
        }

        s.b[2027] = ((s.v[87] > (-0.1)) && (0.1 >= 0.0));
        s.v[2027] = if s.b[2027] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) {
            s.store_offset(781, 87, 0.1);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_scalar(719, 0.0);
        }

        let mut assign41330_loop_guard: usize = 0;
        while {
            let assign41330_cond_e54894: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) && (s.v[719] < s.v[1910])) { 1.0 } else { 0.0 };
            assign41330_cond_e54894 != 0.0
        } {
            assign41330_loop_guard += 1;
            assert!(assign41330_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) {
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) {
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2028] = ((((s.v[1910] == 1.0) || (s.v[1910] == 2.0)) || (s.v[1910] == 4.0)) || (s.v[1910] == 8.0));
        s.v[2028] = if s.b[2028] { 1.0 } else { 0.0 };

        s.b[2029] = (s.v[1910] == 1.0);
        s.v[2029] = if s.b[2029] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) && s.b[2028]) && s.b[2029]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2030] = (s.v[1910] == 2.0);
        s.v[2030] = if s.b[2030] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) && s.b[2028]) && (!s.b[2029])) && s.b[2030]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2031] = (s.v[1910] == 4.0);
        s.v[2031] = if s.b[2031] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) && s.b[2028]) && (!s.b[2029])) && (!s.b[2030])) && s.b[2031]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2032] = (s.v[1910] == 8.0);
        s.v[2032] = if s.b[2032] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) && s.b[2028]) && (!s.b[2029])) && (!s.b[2030])) && (!s.b[2031])) && s.b[2032]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) && s.b[2028]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign41460_loop_guard: usize = 0;
        while {
            let assign41460_cond_e55122: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) && s.b[2028]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign41460_cond_e55122 != 0.0
        } {
            assign41460_loop_guard += 1;
            assert!(assign41460_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) && s.b[2028]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) && (!s.b[2028])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(1910), 2.0)))
                }
            });
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_offset(983, 780, (-0.1));
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) {
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (!s.b[2027])) {
            s.copy_ad(983, 87);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) {
            s.store_add_scaled_inputs3_offset(1912, s.ad_value(791), 1.0, s.ad_value(85), (-1.0), s.ad_value(1908), 1.0, (-(s.v[462] - p.p392)));
            s.store_sub(1911, 791, 1912);
        }

        s.b[2033] = ((s.v[1911] > (-s.v[1909])) && (s.v[1909] >= 0.0));
        s.v[2033] = if s.b[2033] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) {
            s.store_add(781, 1911, 1909);
            s.store_square(722, 781);
            s.store_square(723, 1909);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_scalar(719, 0.0);
        }

        let mut assign41680_loop_guard: usize = 0;
        while {
            let assign41680_cond_e55507: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) && (s.v[719] < s.v[1910])) { 1.0 } else { 0.0 };
            assign41680_cond_e55507 != 0.0
        } {
            assign41680_loop_guard += 1;
            assert!(assign41680_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) {
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) {
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2034] = ((((s.v[1910] == 1.0) || (s.v[1910] == 2.0)) || (s.v[1910] == 4.0)) || (s.v[1910] == 8.0));
        s.v[2034] = if s.b[2034] { 1.0 } else { 0.0 };

        s.b[2035] = (s.v[1910] == 1.0);
        s.v[2035] = if s.b[2035] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) && s.b[2034]) && s.b[2035]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2036] = (s.v[1910] == 2.0);
        s.v[2036] = if s.b[2036] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) && s.b[2034]) && (!s.b[2035])) && s.b[2036]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2037] = (s.v[1910] == 4.0);
        s.v[2037] = if s.b[2037] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) && s.b[2034]) && (!s.b[2035])) && (!s.b[2036])) && s.b[2037]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2038] = (s.v[1910] == 8.0);
        s.v[2038] = if s.b[2038] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) && s.b[2034]) && (!s.b[2035])) && (!s.b[2036])) && (!s.b[2037])) && s.b[2038]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) && s.b[2034]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign41810_loop_guard: usize = 0;
        while {
            let assign41810_cond_e55735: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) && s.b[2034]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign41810_cond_e55735 != 0.0
        } {
            assign41810_loop_guard += 1;
            assert!(assign41810_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) && s.b[2034]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) && (!s.b[2034])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(1910), 2.0)))
                }
            });
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1909, 726);
            s.store_div_scaled_product3(334, s.ad_value(1909), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0);
            s.store_add_scaled_inputs(1911, 1909, -1.0, 780, 1.0);
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) {
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (!s.b[2033])) {
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (!s.b[2033])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) {
            s.store_scalar(79, 0.0);
            s.store_scalar(97, 1.0);
        }

        let mut assign41920_loop_guard: usize = 0;
        while {
            let assign41920_cond_e55951: f64 = if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign41920_cond_e55951 != 0.0
        } {
            assign41920_loop_guard += 1;
            assert!(assign41920_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) {
                s.store_mul(335, 154, 983);
                s.store_exp(336, 335);
            }
            s.b[2039] = (s.v[983] >= 0.0);
            s.v[2039] = if s.b[2039] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2039]) {
                s.store_mul_scaled_ad_rhs(2024, 209, -1.0, A::sqrt(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15)));
                s.store_mul_ad(2025, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(2024), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (!s.b[2039])) {
                s.store_exp_ad(337, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(983), s.ad_value(1883))));
                s.store_exp_ad(338, A::mul_scaled_lhs(s.ad_value(154), 1.0, s.ad_value(1883)));
                s.store_mul_sqrt_ad_rhs(2024, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 2024, 1.0);
                s.store_mul_add_ad_rhs(2025, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (s.v[79] != 0.0)) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (s.v[79] == 0.0)) {
                s.store_add_scaled_product_right_ad(1866, 2024, 1.0, 185, A::sub(s.ad_value(1911), s.ad_value(983)), 1.0);
                s.store_sub(1867, 2025, 185);
                s.store_div_scaled_inputs(1878, s.ad_value(1866), -1.0, s.ad_value(1867), 1.0);
            }
            s.b[2040] = (((s.v[1878]) as f64).abs() < (1e-10 * 100.0));
            s.v[2040] = if s.b[2040] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (s.v[79] == 0.0)) && s.b[2040]) {
                s.store_scalar(79, 1.0);
            }
            s.b[2041] = (s.v[1878] > 0.1);
            s.v[2041] = if s.b[2041] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (s.v[79] == 0.0)) && (!s.b[2040])) && s.b[2041]) {
                s.store_scalar(1878, 0.1);
            }
            s.b[2042] = (s.v[1878] < (-0.1));
            s.v[2042] = if s.b[2042] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (s.v[79] == 0.0)) && (!s.b[2040])) && (!s.b[2041])) && s.b[2042]) {
                s.store_scalar(1878, (-0.1));
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (s.v[79] == 0.0)) {
                s.store_add(983, 983, 1878);
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) {
            s.store_neg(983, 983);
            s.store_mul3_affine_lhs(2022, 1901, 1830, (0.5 * 9662367879.197212), 0.0, 1830);
            s.store_scaled_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(2022)), p.p394);
            s.store_scaled_add_ad(335, A::exp(s.ad_value(334)), A::exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(2023, A::ln(s.ad_value(335)), 2022);
            s.store_mul(332, 2023, 983);
            s.store_exp_ad(334, A::mul_scaled_lhs(s.ad_value(2023), -1.0, s.ad_value(2022)));
        }

        s.b[2044] = (((s.v[332]) as f64).abs() > 1e-8);
        s.v[2044] = if s.b[2044] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2044]) {
            s.store_mul_exp_lhs(335, 332, 334);
        }

    }

    pub(super) fn stamp_reactive_block_37(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2044]) {
            s.store_sub(336, 335, 334);
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (!s.b[2044])) {
            s.store_mul_offset_lhs(335, 332, 1.0, 334);
            s.store_mul_ad_product_lhs(336, s.ad_value(332), A::scale_offset(s.ad_value(332), 0.5, 1.0), 334);
        }

        s.b[2045] = (((s.v[336]) as f64).abs() > 1e-8);
        s.v[2045] = if s.b[2045] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2045]) {
            s.store_div_ad_lhs(2021, A::ln(A::offset(s.ad_value(336), 1.0)), 2023);
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (!s.b[2045])) {
            s.store_div(2021, 336, 2023);
        }

        s.b[2046] = ((((2.0 * 1.034943e-10) * (s.v[983] - s.v[2021])) / s.v[1901]) <= 0.0);
        s.v[2046] = if s.b[2046] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2046]) {
            s.store_scalar(981, 0.0);
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (!s.b[2046])) {
            s.store_sqrt_ad(981, A::div_scaled_inputs2(s.ad_value(983), (2.0 * 1.034943e-10), s.ad_value(2021), (-(2.0 * 1.034943e-10)), s.ad_value(1901), 1.0));
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) {
            s.store_ad_value(981, {
                if (s.v[981] > s.v[1830]) {
                    s.ad_value(1830)
                } else {
                    s.ad_value(981)
                }
            });
        }

        s.b[2047] = (s.v[981] < s.v[1830]);
        s.v[2047] = if s.b[2047] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2047]) {
            s.store_sub(990, 1830, 981);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2047])) {
            s.store_scalar(990, 0.0);
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_neg_ad(1890, A::add(s.ad_value(1885), s.ad_value(1886)));
        }

        s.b[2048] = (s.v[94] < 0.0);
        s.v[2048] = if s.b[2048] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2048]) {
            s.store_scalar(94, 0.0);
            s.copy_ad(1850, 1849);
            s.store_scalar(248, 0.0);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2048])) {
            s.store_mul3_affine_lhs(248, 154, 1890, 1.0 / (2.0), 0.0, 94);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2048])) {
            s.store_ad_value(248, {
                if (s.v[248] < 0.0) {
                    A::constant(0.0)
                } else {
                    s.ad_value(248)
                }
            });
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_neg(238, 1887);
            s.copy_ad(170, 162);
            s.store_scalar(336, (s.v[626] / 100.0));
            s.copy_ad(334, 682);
            s.store_offset_sqrt_ad(980, A::offset(A::square(s.ad_value(94)), p.p262), (-((p.p262) as f64).sqrt()));
            s.store_offset_mul(338, 980, 334, 1.0);
            s.store_mul(339, 336, 238);
            s.store_div(337, 339, 338);
            s.copy_ad(251, 337);
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_ad_value(339, {
                if (s.v[251] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(251), (p.p160 - 1.0))
                }
            });
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_mul(342, 339, 251);
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_ad_value(341, {
                if (s.v[251] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(251), A::offset(s.ad_value(624), (-1.0)))
                }
            });
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_mul(340, 341, 251);
            s.store_scalar(343, (1.6021918e-19 * 10000.0));
            s.store_div(252, 238, 343);
            s.store_scalar(336, s.v[474]);
            s.store_add_scaled_ad_lhs(335, A::add_scaled_product(A::div_from_scalar(1.0, A::add_scaled_inputs(s.ad_value(336), 1.0, s.ad_value(252), (s.v[475] * 1e-11))), 1.0, s.ad_value(679), s.ad_value(342), 1.0), 340, 1.0 / (s.v[479]));
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_mul_ad_product_lhs(336, s.ad_value(154), A::offset(s.ad_value(238), 1e-25), 170);
            s.store_div_from_scalar(335, 1.0, 336);
            s.store_mul(333, 248, 335);
            s.store_div_scaled_inputs(336, s.ad_value(257), 0.2, s.ad_value(254), 1.0);
            s.store_sqrt_square_sum(255, 333, 336);
            s.store_div_from_scalar(338, 1.0, 255);
            s.store_mul(256, 254, 255);
            s.store_div(335, 256, 257);
        }

        s.b[2049] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2049] = if s.b[2049] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2049]) {
            s.store_scalar(337, 1.0);
        }

        s.b[2050] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2050] = if s.b[2050] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2049])) && s.b[2050]) {
            s.copy_ad(337, 335);
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2049])) && (!s.b[2050])) {
            s.store_ad_value(337, {
                if (s.v[335] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(335), (p.p178 - 1.0))
                }
            });
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_mul(336, 335, 337);
            s.store_offset(338, 336, 1.0);
        }

        s.b[2051] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2051] = if s.b[2051] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2051]) {
            s.store_div_from_scalar(339, 1.0, 338);
        }

        s.b[2052] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2052] = if s.b[2052] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2051])) && s.b[2052]) {
            s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2051])) && (!s.b[2052])) {
            s.store_ad_value(340, {
                if (s.v[338] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(338), (((-1.0) / p.p178) - 1.0))
                }
            });
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2051])) && (!s.b[2052])) {
            s.store_mul(339, 338, 340);
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_mul(253, 254, 339);
            s.copy_ad(984, 253);
            s.copy_ad(1882, 255);
            s.copy_ad(989, 349);
        }

        s.b[2053] = (s.v[349] > 1e-6);
        s.v[2053] = if s.b[2053] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) {
            s.store_scaled_add(344, 1883, 155, p.p396);
            s.store_offset_mul_ad(338, s.ad_value(1903), A::sub(s.ad_value(85), s.ad_value(344)), 1.0);
            s.store_offset(339, 1903, 1.0);
        }

        s.b[2054] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));
        s.v[2054] = if s.b[2054] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) {
            s.store_sub(781, 339, 338);
            s.store_square(722, 781);
            s.store_square(723, 339);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2055] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2055] = if s.b[2055] { 1.0 } else { 0.0 };

        s.b[2056] = (2.0 == 1.0);
        s.v[2056] = if s.b[2056] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) && s.b[2055]) && s.b[2056]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2057] = (2.0 == 2.0);
        s.v[2057] = if s.b[2057] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) && s.b[2055]) && (!s.b[2056])) && s.b[2057]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2058] = (2.0 == 4.0);
        s.v[2058] = if s.b[2058] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) && s.b[2055]) && (!s.b[2056])) && (!s.b[2057])) && s.b[2058]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2059] = (2.0 == 8.0);
        s.v[2059] = if s.b[2059] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) && s.b[2055]) && (!s.b[2056])) && (!s.b[2057])) && (!s.b[2058])) && s.b[2059]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) && s.b[2055]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign42970_loop_guard: usize = 0;
        while {
            let assign42970_cond_e57736: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) && s.b[2055]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign42970_cond_e57736 != 0.0
        } {
            assign42970_loop_guard += 1;
            assert!(assign42970_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) && s.b[2055]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) && (!s.b[2055])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 339, 726);
            s.store_div_scaled_product3(334, s.ad_value(339), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0);
            s.store_sub(338, 339, 780);
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) {
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && (!s.b[2054])) {
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && (!s.b[2054])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) {
            s.store_sqrt(337, 338);
            s.store_add_ad_rhs(344, 85, A::mul_sub_from_scalar_rhs(s.ad_value(1902), 1.0, s.ad_value(337)));
        }

        s.b[2060] = ((s.v[344] < (s.v[972] + s.v[1906])) && (s.v[1906] >= 0.0));
        s.v[2060] = if s.b[2060] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) {
            s.store_add_scaled_inputs3(781, s.ad_value(972), 1.0, s.ad_value(1906), 1.0, s.ad_value(344), -1.0);
            s.store_square(722, 781);
            s.store_square(723, 1906);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
        }

    }

    pub(super) fn stamp_reactive_block_38(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) {
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2061] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2061] = if s.b[2061] { 1.0 } else { 0.0 };

        s.b[2062] = (2.0 == 1.0);
        s.v[2062] = if s.b[2062] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) && s.b[2061]) && s.b[2062]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2063] = (2.0 == 2.0);
        s.v[2063] = if s.b[2063] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) && s.b[2061]) && (!s.b[2062])) && s.b[2063]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2064] = (2.0 == 4.0);
        s.v[2064] = if s.b[2064] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) && s.b[2061]) && (!s.b[2062])) && (!s.b[2063])) && s.b[2064]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2065] = (2.0 == 8.0);
        s.v[2065] = if s.b[2065] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) && s.b[2061]) && (!s.b[2062])) && (!s.b[2063])) && (!s.b[2064])) && s.b[2065]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) && s.b[2061]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign43340_loop_guard: usize = 0;
        while {
            let assign43340_cond_e58300: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) && s.b[2061]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign43340_cond_e58300 != 0.0
        } {
            assign43340_loop_guard += 1;
            assert!(assign43340_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) && s.b[2061]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) && (!s.b[2061])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1906, 726);
            s.store_div_scaled_product3(334, s.ad_value(1906), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0);
            s.store_add_scaled_inputs3(344, s.ad_value(972), 1.0, s.ad_value(1906), 1.0, s.ad_value(780), -1.0);
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) {
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && (!s.b[2060])) {
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && (!s.b[2060])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) {
            s.store_div(335, 989, 344);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) {
            s.store_ad_value(336, {
                if (s.v[335] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(335), (p.p383 - 1.0))
                }
            });
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) {
            s.store_offset_mul(337, 336, 335, 1.0);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) {
            s.store_ad_value(338, {
                if (s.v[337] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(337), ((1.0 / p.p383) - 1.0))
                }
            });
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) {
            s.store_mul(340, 338, 337);
            s.store_div(989, 989, 340);
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_sub(335, 791, 1883);
        }

        s.b[2066] = ((s.v[335] < 1.0) && (1.0 >= 0.0));
        s.v[2066] = if s.b[2066] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) {
            s.store_sub_from_scalar(781, 1.0, 335);
            s.store_square(722, 781);
            s.store_scalar(723, 1.0);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2067] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2067] = if s.b[2067] { 1.0 } else { 0.0 };

        s.b[2068] = (2.0 == 1.0);
        s.v[2068] = if s.b[2068] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) && s.b[2067]) && s.b[2068]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2069] = (2.0 == 2.0);
        s.v[2069] = if s.b[2069] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) && s.b[2067]) && (!s.b[2068])) && s.b[2069]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2070] = (2.0 == 4.0);
        s.v[2070] = if s.b[2070] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) && s.b[2067]) && (!s.b[2068])) && (!s.b[2069])) && s.b[2070]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2071] = (2.0 == 8.0);
        s.v[2071] = if s.b[2071] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) && s.b[2067]) && (!s.b[2068])) && (!s.b[2069])) && (!s.b[2070])) && s.b[2071]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) && s.b[2067]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign43760_loop_guard: usize = 0;
        while {
            let assign43760_cond_e58900: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) && s.b[2067]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign43760_cond_e58900 != 0.0
        } {
            assign43760_loop_guard += 1;
            assert!(assign43760_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) && s.b[2067]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) && (!s.b[2067])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1.0);
            s.store_div_scaled_product_indices(334, 725, 726, 1.0, 770, 1.0);
            s.store_sub_from_scalar(335, 1.0, 780);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) {
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2066])) {
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2066])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_div(251, 335, 965);
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_ad_value(339, {
                if (s.v[251] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(251), (p.p353 - 1.0))
                }
            });
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_mul(342, 339, 251);
            s.store_offset(336, 966, 1e-25);
            s.store_add_ad(335, A::div_from_scalar(1.0, s.ad_value(336)), A::div(s.ad_value(342), s.ad_value(970)));
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_div(1881, 989, 170);
            s.store_powf(781, 989, 2.0);
            s.store_scalar(782, ((0.1) as f64).powf(2.0));
            s.store_sub_ad(335, A::powf(A::add(s.ad_value(781), s.ad_value(782)), (1.0 / 2.0)), A::powf(s.ad_value(782), (1.0 / 2.0)));
            s.store_div(335, 335, 170);
            s.store_div_scaled_product_indices(335, 254, 335, 1.0, 973, 1.0);
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_ad_value(336, {
                if (s.v[335] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(335), p.p378)
                }
            });
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_offset(337, 336, 1.0);
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_ad_value(338, {
                if (s.v[337] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(337), (1.0 / p.p378))
                }
            });
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_div(985, 254, 338);
            s.store_scaled_mul(991, 990, 964, (-1.6021918e-19));
            s.store_mul3_affine_lhs(987, 991, 985, (-s.v[632]), 0.0, 1881);
            s.store_div_scaled_inputs(115, s.ad_value(155), s.v[632], s.ad_value(170), 1.0);
            s.store_mul3_lhs(986, 115, 248, 984);
            s.store_add(135, 986, 987);
            s.copy_ad(790, 349);
        }

        s.b[2072] = (p.p283 != 0.0);
        s.v[2072] = if s.b[2072] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2072]) {
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 100.0));
            s.store_offset_ad(782, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0)), 1.0);
            s.store_offset_ad(783, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0)), (1.0 / 2.0));
            s.store_div_from_scalar(340, 0.01, 782);
            s.store_div_scaled_inputs(336, s.ad_value(783), (-2.0), A::square(s.ad_value(782)), 1.0);
            s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(1849), s.ad_value(340)));
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));
            s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);
            s.store_scaled_add(336, 335, 782, 0.5);
        }

        s.b[2073] = (s.v[336] < 0.0);
        s.v[2073] = if s.b[2073] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2072]) && s.b[2073]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2072]) {
            s.store_offset(336, 336, 1e-25);
            s.store_scale(334, 154, s.v[672]);
            s.store_mul(337, 185, 334);
            s.store_powf(334, 336, p.p284);
            s.store_mul(343, 337, 334);
            s.store_offset_scaled(338, 1435, p.p285, 1.0);
            s.store_scalar(334, s.v[673]);
            s.store_add_scaled_inputs3(339, s.ad_value(1849), 1.0, s.ad_value(340), 1.0, s.ad_value(1434), -1.0);
            s.store_add_ad_rhs(338, 338, A::mul3(s.ad_value(1435), s.ad_value(334), s.ad_value(339)));
            s.store_mul(340, 343, 338);
            s.copy_ad(343, 340);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2072])) {
            s.store_scalar(343, 0.0);
        }

        s.b[2074] = (p.p287 != 0.0);
        s.v[2074] = if s.b[2074] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2074]) {
            s.store_scale(335, 154, s.v[674]);
            s.store_mul(336, 185, 335);
            s.store_mul(342, 336, 1435);
        }

    }

    pub(super) fn stamp_reactive_block_39(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2074])) {
            s.store_scalar(342, 0.0);
        }

        s.b[2075] = ((s.v[343] + s.v[342]) > 0.0);
        s.v[2075] = if s.b[2075] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2075]) {
            s.store_mul_add_rhs(249, 94, 343, 342);
            s.store_add_ad_rhs(135, 135, A::mul3(s.ad_value(115), s.ad_value(249), s.ad_value(253)));
        }

        s.b[2076] = ((s.v[74] == 2.0) || (s.v[74] == 3.0));
        s.v[2076] = if s.b[2076] { 1.0 } else { 0.0 };

        s.b[2077] = (p.p296 > 0.0);
        s.v[2077] = if s.b[2077] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2076]) && s.b[2077]) {
            s.copy_ad(338, 647);
            s.store_scaled_offset(335, 796, (-p.p300), s.v[533]);
            s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));
            s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2076]) && s.b[2077]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2076]) && s.b[2077]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3(336, s.ad_value(338), 1.0, s.ad_value(781), 0.5, s.ad_value(782), 0.5);
            s.store_scale(337, 338, (p.p296 + 1.0));
            s.store_offset_sub(781, 337, 336, (-(0.01 * 0.01)));
            s.store_scale(782, 337, (4.0 * (0.01 * 0.01)));
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2076]) && s.b[2077]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2076]) && s.b[2077]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3(341, s.ad_value(337), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5));
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2076]) && (!s.b[2077])) {
            s.copy_ad(341, 647);
        }

        s.b[2078] = (s.v[793] >= 0.0);
        s.v[2078] = if s.b[2078] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2076]) && s.b[2078]) {
            s.copy_ad(369, 793);
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2076]) && (!s.b[2078])) {
            s.store_scalar(369, 0.0);
        }

        s.b[2079] = (s.v[369] < (20.0 * 1e-12));
        s.v[2079] = if s.b[2079] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2076]) && s.b[2079]) {
            s.store_scalar(378, (((((20.0 + 1.0)) as f64).powf((p.p297 - 1.0)) * ((20.0 + 1.0) - ((0.5 * p.p297) * 20.0))) * ((1e-12) as f64).powf(p.p297)));
            s.store_scalar(379, ((((0.5 * p.p297) * (((20.0 + 1.0)) as f64).powf((p.p297 - 1.0))) / 20.0) * ((1e-12) as f64).powf((p.p297 - 2.0))));
            s.store_add_ad_rhs(335, 378, A::mul3(s.ad_value(379), s.ad_value(369), s.ad_value(369)));
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2076]) && (!s.b[2079])) {
            s.store_powf_ad(335, A::offset(s.ad_value(369), 1e-12), p.p297);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2076]) {
            s.store_powf_ad(343, A::offset(s.ad_value(369), 1e-12), p.p299);
            s.store_add_scaled_products_indices(368, 341, 335, 1.0 / (s.v[632]), 797, 343, (s.v[531] * 1.0 / (s.v[632])));
            s.store_mul(334, 368, 135);
            s.store_offset(335, 790, 1e-12);
            s.store_div_from_scalar(336, 1.0, 335);
            s.store_offset_mul(337, 334, 336, 1.0);
            s.store_div_from_scalar(338, 1.0, 337);
            s.store_mul(134, 135, 338);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2076])) {
            s.copy_ad(134, 135);
            s.store_scalar(368, 0.0);
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_add_scaled_inputs4(131, s.ad_value(1860), (-0.5), s.ad_value(1861), (-0.5), s.ad_value(1863), (-0.5), s.ad_value(1865), (-0.5));
            s.store_scaled_add_ad_lhs(133, A::add(A::add_scaled_inputs4(s.ad_value(1888), 1.0, s.ad_value(1889), 1.0, s.ad_value(1891), 1.0, s.ad_value(1892), 1.0), s.ad_value(1862)), 1864, (-0.5));
            s.store_scalar(247, 0.5);
            s.store_scaled_add(978, 1888, 1889, (-0.5));
            s.store_neg(238, 1888);
            s.copy_ad(255, 1882);
        }

        s.b[2080] = ((s.v[238] < 1e-25) || (s.v[133] < 1e-25));
        s.v[2080] = if s.b[2080] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2080]) {
            s.store_scalar(78, 1.0);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.copy_ad(2087, 960);
            s.store_scale(2129, 964, 1.6021918e-19);
            s.store_scale(2110, 964, (1.6021918e-19 * 1.034943e-10));
            s.store_div_from_scalar(2132, (2.0 * 1.034943e-10), 2129);
            s.store_div(2126, 964, 622);
            s.store_div_from_scalar_offset_input(2125, 1.0, 2126, 1.0);
            s.store_div_ad_rhs(2130, 2110, A::square(s.ad_value(185)));
            s.store_div_from_scalar(2131, 2.0, 2130);
            s.store_scalar(2139, 2.0);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_scalar(508, (if param_given[227] { s.v[508] } else { (5000000000.0 / (p.p343 * p.p340)) }));
        }

        s.b[2168] = ((s.v[508] < (2.0 + 0.1)) && (0.1 >= 0.0));
        s.v[2168] = if s.b[2168] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) {
            s.store_sub_from_scalar(781, (2.0 + 0.1), 508);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2169] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2169] = if s.b[2169] { 1.0 } else { 0.0 };

        s.b[2170] = (2.0 == 1.0);
        s.v[2170] = if s.b[2170] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) && s.b[2169]) && s.b[2170]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2171] = (2.0 == 2.0);
        s.v[2171] = if s.b[2171] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) && s.b[2169]) && (!s.b[2170])) && s.b[2171]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2172] = (2.0 == 4.0);
        s.v[2172] = if s.b[2172] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) && s.b[2169]) && (!s.b[2170])) && (!s.b[2171])) && s.b[2172]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2173] = (2.0 == 8.0);
        s.v[2173] = if s.b[2173] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) && s.b[2169]) && (!s.b[2170])) && (!s.b[2171])) && (!s.b[2172])) && s.b[2173]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) && s.b[2169]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign45220_loop_guard: usize = 0;
        while {
            let assign45220_cond_e61026: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) && s.b[2169]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign45220_cond_e61026 != 0.0
        } {
            assign45220_loop_guard += 1;
            assert!(assign45220_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) && s.b[2169]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) && (!s.b[2169])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_sub_from_scalar(508, (2.0 + 0.1), 780);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) {
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2168])) {
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2168])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_scalar(2085, 0.0);
            s.store_scalar(2086, 0.0);
            s.store_scalar(2094, 0.0);
            s.store_scalar(2095, 0.0);
            s.store_scalar(2167, 0.0);
            s.store_scalar(2142, 0.0);
            s.copy_ad(2113, 1431);
            s.store_add_scaled_inputs3_offset(85, s.ad_value(791), 1.0, s.ad_value(120), 1.0, s.ad_value(182), (-1.0), (-s.v[160]));
            s.store_offset_sub_from_scalar_ad(781, 0.3, s.ad_value(85), (-0.01));
            s.store_scalar(782, ((4.0 * 0.3) * 0.01));
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_sub_from_scalar_ad(2092, 0.3, A::add_scaled_inputs(s.ad_value(781), 0.5, s.ad_value(782), 0.5));
            s.store_add_scaled_inputs3_offset(781, s.ad_value(2092), 1.0, s.ad_value(2113), -1.0, s.ad_value(2087), 1.0, (-0.01));
            s.store_scaled_sub(782, 2113, 2087, (4.0 * 0.01));
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4(2092, s.ad_value(2113), 1.0, s.ad_value(2087), (-1.0), s.ad_value(781), 0.5, s.ad_value(782), 0.5);
            s.copy_ad(2085, 2092);
            s.store_scalar(2083, 0.0);
            s.copy_ad(2088, 2083);
            s.store_mul_sub_rhs(2090, 2125, 1434, 2087);
            s.store_mul_neg_rhs(2146, 2125, 2087);
        }

        s.b[2174] = (((-s.v[2090]) < 0.001) && (0.001 >= 0.0));
        s.v[2174] = if s.b[2174] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) {
            s.store_sub_from_scalar_ad(781, 0.001, A::neg(s.ad_value(2090)));
            s.store_square(722, 781);
        }

    }

    pub(super) fn stamp_reactive_block_40(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) {
            s.store_scalar(723, (0.001 * 0.001));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2175] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2175] = if s.b[2175] { 1.0 } else { 0.0 };

        s.b[2176] = (2.0 == 1.0);
        s.v[2176] = if s.b[2176] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) && s.b[2175]) && s.b[2176]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2177] = (2.0 == 2.0);
        s.v[2177] = if s.b[2177] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) && s.b[2175]) && (!s.b[2176])) && s.b[2177]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2178] = (2.0 == 4.0);
        s.v[2178] = if s.b[2178] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) && s.b[2175]) && (!s.b[2176])) && (!s.b[2177])) && s.b[2178]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2179] = (2.0 == 8.0);
        s.v[2179] = if s.b[2179] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) && s.b[2175]) && (!s.b[2176])) && (!s.b[2177])) && (!s.b[2178])) && s.b[2179]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) && s.b[2175]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign45820_loop_guard: usize = 0;
        while {
            let assign45820_cond_e61921: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) && s.b[2175]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign45820_cond_e61921 != 0.0
        } {
            assign45820_loop_guard += 1;
            assert!(assign45820_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) && s.b[2175]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) && (!s.b[2175])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.001);
            s.store_div_scaled_product_indices(337, 725, 726, 0.001, 770, 1.0);
            s.store_sub_from_scalar(335, 0.001, 780);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) {
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2174])) {
            s.store_neg(335, 2090);
            s.store_scalar(337, 1.0);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_sqrt_mul(2081, 2132, 335);
        }

        s.b[2180] = (((-s.v[2146]) < 0.001) && (0.001 >= 0.0));
        s.v[2180] = if s.b[2180] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) {
            s.store_sub_from_scalar_ad(781, 0.001, A::neg(s.ad_value(2146)));
            s.store_square(722, 781);
            s.store_scalar(723, (0.001 * 0.001));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2181] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2181] = if s.b[2181] { 1.0 } else { 0.0 };

        s.b[2182] = (2.0 == 1.0);
        s.v[2182] = if s.b[2182] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) && s.b[2181]) && s.b[2182]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2183] = (2.0 == 2.0);
        s.v[2183] = if s.b[2183] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) && s.b[2181]) && (!s.b[2182])) && s.b[2183]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2184] = (2.0 == 4.0);
        s.v[2184] = if s.b[2184] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) && s.b[2181]) && (!s.b[2182])) && (!s.b[2183])) && s.b[2184]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2185] = (2.0 == 8.0);
        s.v[2185] = if s.b[2185] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) && s.b[2181]) && (!s.b[2182])) && (!s.b[2183])) && (!s.b[2184])) && s.b[2185]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) && s.b[2181]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign46180_loop_guard: usize = 0;
        while {
            let assign46180_cond_e62473: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) && s.b[2181]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign46180_cond_e62473 != 0.0
        } {
            assign46180_loop_guard += 1;
            assert!(assign46180_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) && s.b[2181]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) && (!s.b[2181])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.001);
            s.store_div_scaled_product_indices(337, 725, 726, 0.001, 770, 1.0);
            s.store_sub_from_scalar(335, 0.001, 780);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) {
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2180])) {
            s.store_neg(335, 2146);
            s.store_scalar(337, 1.0);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_sqrt_mul(2147, 2132, 335);
        }

        s.b[2186] = (p.p345 != 0.0);
        s.v[2186] = if s.b[2186] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2186]) {
            s.store_mul_sub_from_scalar_ad_rhs(335, 965, 1.0, A::scale(s.ad_value(790), p.p345));
            s.store_scale(336, 965, 0.001);
            s.store_add_scaled_inputs3(781, s.ad_value(335), 1.0, s.ad_value(965), (-0.1), s.ad_value(336), -1.0);
            s.store_scaled_mul(782, 965, 336, (0.1 * 4.0));
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2186]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2186]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3(335, s.ad_value(965), 0.1, s.ad_value(781), 0.5, s.ad_value(782), 0.5);
            s.store_add_scaled_inputs3(781, s.ad_value(965), 2.0, s.ad_value(335), (-1.0), s.ad_value(336), -1.0);
            s.store_scaled_mul(782, 965, 336, (2.0 * 4.0));
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2186]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2186]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3(965, s.ad_value(965), 2.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5));
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.copy_ad(2127, 965);
            s.store_sub(2144, 965, 2081);
            s.store_sub(2145, 965, 2147);
        }

        s.b[2187] = ((s.v[2144] < (p.p344 + (p.p344 * 0.1))) && ((p.p344 * 0.1) >= 0.0));
        s.v[2187] = if s.b[2187] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) {
            s.store_sub_from_scalar(781, (p.p344 + (p.p344 * 0.1)), 2144);
            s.store_square(722, 781);
            s.store_scalar(723, ((p.p344 * 0.1) * (p.p344 * 0.1)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2188] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[2188] = if s.b[2188] { 1.0 } else { 0.0 };

        s.b[2189] = (1.0 == 1.0);
        s.v[2189] = if s.b[2189] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) && s.b[2188]) && s.b[2189]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2190] = (1.0 == 2.0);
        s.v[2190] = if s.b[2190] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) && s.b[2188]) && (!s.b[2189])) && s.b[2190]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2191] = (1.0 == 4.0);
        s.v[2191] = if s.b[2191] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) && s.b[2188]) && (!s.b[2189])) && (!s.b[2190])) && s.b[2191]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2192] = (1.0 == 8.0);
        s.v[2192] = if s.b[2192] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) && s.b[2188]) && (!s.b[2189])) && (!s.b[2190])) && (!s.b[2191])) && s.b[2192]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) && s.b[2188]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign46700_loop_guard: usize = 0;
        while {
            let assign46700_cond_e63307: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) && s.b[2188]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign46700_cond_e63307 != 0.0
        } {
            assign46700_loop_guard += 1;
            assert!(assign46700_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) && s.b[2188]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) && (!s.b[2188])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / 2.0))
                }
            });
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (p.p344 * 0.1));
            s.store_div_scaled_product_indices(334, 725, 726, (p.p344 * 0.1), 770, 1.0);
            s.store_sub_from_scalar(2144, (p.p344 + (p.p344 * 0.1)), 780);
        }

    }

    pub(super) fn stamp_reactive_block_41(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) {
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2187])) {
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2187])) {
            s.store_scalar(334, 1.0);
        }

        s.b[2193] = ((s.v[2145] < (p.p344 * 0.1)) && ((p.p344 * 0.1) >= 0.0));
        s.v[2193] = if s.b[2193] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) {
            s.store_sub_from_scalar(781, (p.p344 * 0.1), 2145);
            s.store_square(722, 781);
            s.store_scalar(723, ((p.p344 * 0.1) * (p.p344 * 0.1)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2194] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[2194] = if s.b[2194] { 1.0 } else { 0.0 };

        s.b[2195] = (1.0 == 1.0);
        s.v[2195] = if s.b[2195] { 1.0 } else { 0.0 };

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) && s.b[2194]) && s.b[2195]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2196] = (1.0 == 2.0);
        s.v[2196] = if s.b[2196] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) && s.b[2194]) && (!s.b[2195])) && s.b[2196]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2197] = (1.0 == 4.0);
        s.v[2197] = if s.b[2197] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) && s.b[2194]) && (!s.b[2195])) && (!s.b[2196])) && s.b[2197]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2198] = (1.0 == 8.0);
        s.v[2198] = if s.b[2198] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) && s.b[2194]) && (!s.b[2195])) && (!s.b[2196])) && (!s.b[2197])) && s.b[2198]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) && s.b[2194]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign47030_loop_guard: usize = 0;
        while {
            let assign47030_cond_e63828: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) && s.b[2194]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign47030_cond_e63828 != 0.0
        } {
            assign47030_loop_guard += 1;
            assert!(assign47030_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) && s.b[2194]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) && (!s.b[2194])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / 2.0))
                }
            });
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (p.p344 * 0.1));
            s.store_div_scaled_product_indices(334, 725, 726, (p.p344 * 0.1), 770, 1.0);
            s.store_sub_from_scalar(2145, (p.p344 * 0.1), 780);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) {
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2193])) {
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2193])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_offset_scaled_div(2148, 2144, 2145, (p.p394 - p.p395), p.p395);
            s.store_scalar(79, 0.0);
            s.store_mul(2136, 2125, 2126);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_42(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign47160_loop_guard: usize = 0;
        while {
            let assign47160_cond_e64069: f64 = if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign47160_cond_e64069 != 0.0
        } {
            assign47160_loop_guard += 1;
            assert!(assign47160_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
                s.store_mul_sub_ad_rhs(2090, 2125, A::add_scaled_product(s.ad_value(2113), 1.0, s.ad_value(2126), s.ad_value(2088), 1.0), s.ad_value(2087));
                s.store_sub(335, 2088, 2090);
            }
            s.b[2199] = ((s.v[335] < 0.001) && (0.001 >= 0.0));
            s.v[2199] = if s.b[2199] { 1.0 } else { 0.0 };
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) {
                s.store_sub_from_scalar(781, 0.001, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.001 * 0.001));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[2200] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[2200] = if s.b[2200] { 1.0 } else { 0.0 };
            s.b[2201] = (2.0 == 1.0);
            s.v[2201] = if s.b[2201] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) && s.b[2200]) && s.b[2201]) {
                s.store_scalar(720, 1.0);
            }
            s.b[2202] = (2.0 == 2.0);
            s.v[2202] = if s.b[2202] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) && s.b[2200]) && (!s.b[2201])) && s.b[2202]) {
                s.store_scalar(720, 2.0);
            }
            s.b[2203] = (2.0 == 4.0);
            s.v[2203] = if s.b[2203] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) && s.b[2200]) && (!s.b[2201])) && (!s.b[2202])) && s.b[2203]) {
                s.store_scalar(720, 3.0);
            }
            s.b[2204] = (2.0 == 8.0);
            s.v[2204] = if s.b[2204] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) && s.b[2200]) && (!s.b[2201])) && (!s.b[2202])) && (!s.b[2203])) && s.b[2204]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) && s.b[2200]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign47160_body28_loop_guard: usize = 0;
            while {
                let assign47160_body28_cond_e64467: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) && s.b[2200]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign47160_body28_cond_e64467 != 0.0
            } {
                assign47160_body28_loop_guard += 1;
                assert!(assign47160_body28_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) && s.b[2200]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) && (!s.b[2200])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.001);
                s.store_div_scaled_product_indices(336, 725, 726, 0.001, 770, 1.0);
                s.store_sub_from_scalar(335, 0.001, 780);
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) {
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2199])) {
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2199])) {
                s.store_scalar(336, 1.0);
            }
            if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
                s.store_sqrt_mul(2081, 2132, 335);
            }
            s.b[2205] = ((s.v[2081] > (s.v[2127] - 1e-12)) && (1e-12 >= 0.0));
            s.v[2205] = if s.b[2205] { 1.0 } else { 0.0 };
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) {
                s.store_offset_sub(781, 2081, 2127, 1e-12);
                s.store_square(722, 781);
                s.store_scalar(723, (1e-12 * 1e-12));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[2206] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[2206] = if s.b[2206] { 1.0 } else { 0.0 };
            s.b[2207] = (2.0 == 1.0);
            s.v[2207] = if s.b[2207] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) && s.b[2206]) && s.b[2207]) {
                s.store_scalar(720, 1.0);
            }
            s.b[2208] = (2.0 == 2.0);
            s.v[2208] = if s.b[2208] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) && s.b[2206]) && (!s.b[2207])) && s.b[2208]) {
                s.store_scalar(720, 2.0);
            }
            s.b[2209] = (2.0 == 4.0);
            s.v[2209] = if s.b[2209] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) && s.b[2206]) && (!s.b[2207])) && (!s.b[2208])) && s.b[2209]) {
                s.store_scalar(720, 3.0);
            }
            s.b[2210] = (2.0 == 8.0);
            s.v[2210] = if s.b[2210] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) && s.b[2206]) && (!s.b[2207])) && (!s.b[2208])) && (!s.b[2209])) && s.b[2210]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) && s.b[2206]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign47160_body64_loop_guard: usize = 0;
            while {
                let assign47160_body64_cond_e65016: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) && s.b[2206]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign47160_body64_cond_e65016 != 0.0
            } {
                assign47160_body64_loop_guard += 1;
                assert!(assign47160_body64_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) && s.b[2206]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) && (!s.b[2206])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-12);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-12, 770, 1.0);
                s.store_add_ad_lhs(2081, A::offset(s.ad_value(2127), (-1e-12)), 780);
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) {
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2205])) {
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2205])) {
                s.store_scalar(337, 1.0);
            }
            if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
                s.store_mul(337, 336, 337);
                s.store_add_ad_rhs(2133, 2085, A::div(A::add_scaled_square_product(s.ad_value(2127), 1.0, s.ad_value(2081), A::sub_scaled_inputs(s.ad_value(2081), 1.0, s.ad_value(2127), 2.0), 1.0), s.ad_value(2132)));
                s.store_scalar(2134, 1.0);
                s.store_ad_value(2135, A::mul_sub_from_scalar_rhs(A::add_scaled_product(s.ad_value(337), 1.0, A::div(s.ad_value(2127), s.ad_value(2081)), s.ad_value(337), (-1.0)), 1.0, s.ad_value(2136)));
            }
            s.b[2211] = ((s.v[2133] > (s.v[2083] - p.p406)) && (p.p406 >= 0.0));
            s.v[2211] = if s.b[2211] { 1.0 } else { 0.0 };
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) {
                s.store_offset_sub(781, 2133, 2083, p.p406);
                s.store_square(722, 781);
                s.store_scalar(723, (p.p406 * p.p406));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[2212] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[2212] = if s.b[2212] { 1.0 } else { 0.0 };
            s.b[2213] = (4.0 == 1.0);
            s.v[2213] = if s.b[2213] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) && s.b[2212]) && s.b[2213]) {
                s.store_scalar(720, 1.0);
            }
            s.b[2214] = (4.0 == 2.0);
            s.v[2214] = if s.b[2214] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) && s.b[2212]) && (!s.b[2213])) && s.b[2214]) {
                s.store_scalar(720, 2.0);
            }
            s.b[2215] = (4.0 == 4.0);
            s.v[2215] = if s.b[2215] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) && s.b[2212]) && (!s.b[2213])) && (!s.b[2214])) && s.b[2215]) {
                s.store_scalar(720, 3.0);
            }
            s.b[2216] = (4.0 == 8.0);
            s.v[2216] = if s.b[2216] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) && s.b[2212]) && (!s.b[2213])) && (!s.b[2214])) && (!s.b[2215])) && s.b[2216]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) && s.b[2212]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign47160_body107_loop_guard: usize = 0;
            while {
                let assign47160_body107_cond_e65681: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) && s.b[2212]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign47160_body107_cond_e65681 != 0.0
            } {
                assign47160_body107_loop_guard += 1;
                assert!(assign47160_body107_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) && s.b[2212]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) && (!s.b[2212])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                    }
                });
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, p.p406);
                s.store_div_scaled_product_indices(334, 725, 726, p.p406, 770, 1.0);
                s.store_add_ad_lhs(2133, A::offset(s.ad_value(2083), (-p.p406)), 780);
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) {
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2211])) {
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2211])) {
                s.store_scalar(334, 1.0);
            }
            if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
                s.store_mul(2134, 2134, 334);
                s.store_mul(2135, 2135, 334);
                s.store_mul_sub_rhs(339, 154, 2085, 2088);
                s.store_exp(340, 339);
                s.store_sub_ad_lhs(344, A::offset(s.ad_value(340), (-1.0)), 339);
            }
            s.b[2217] = (s.v[339] >= 1e-7);
            s.v[2217] = if s.b[2217] { 1.0 } else { 0.0 };
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2217]) {
                s.store_scalar(347, (-1.0));
                s.store_mul_scaled_ad_rhs(2094, 209, -1.0, A::sqrt(s.ad_value(344)));
                s.store_div_scaled_product3(345, s.ad_value(209), s.ad_value(209), s.ad_value(154), 0.5, s.ad_value(2094), 1.0);
                s.store_mul_offset_rhs(2121, 345, 340, (-1.0));
                s.store_mul_sub_from_scalar_rhs(2123, 345, 1.0, 340);
            }
            s.b[2218] = (s.v[339] < (-1e-7));
            s.v[2218] = if s.b[2218] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2217])) && s.b[2218]) {
                s.store_scalar(347, 1.0);
                s.store_exp_ad(342, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub_scaled_inputs(s.ad_value(2085), 1.0, s.ad_value(2113), p.p398)));
                s.store_exp_ad(343, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub_scaled_inputs(s.ad_value(2088), 1.0, s.ad_value(2113), p.p398)));
                s.store_mul_sqrt_ad_rhs(2094, 209, A::add_scaled_product(s.ad_value(344), 1.0, s.ad_value(210), A::add_scaled_inputs3(s.ad_value(342), 1.0, s.ad_value(343), (-1.0), s.ad_value(339), 1.0), 1.0));
                s.store_div_scaled_product3(345, s.ad_value(209), s.ad_value(209), s.ad_value(154), 0.5, s.ad_value(2094), 1.0);
                s.store_mul_add_ad_rhs(2121, 345, A::offset(s.ad_value(340), (-1.0)), A::mul_sub_from_scalar_rhs(s.ad_value(210), 1.0, s.ad_value(342)));
                s.store_mul_ad_rhs(2123, 345, A::add_scaled_sub_value_product(1.0, s.ad_value(340), 1.0, s.ad_value(210), A::offset(s.ad_value(343), (-1.0)), 1.0));
            }
            s.b[2219] = (s.v[339] > 0.0);
            s.v[2219] = if s.b[2219] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2217])) && (!s.b[2218])) && s.b[2219]) {
                s.store_offset_scaled(2159, 339, ((0.3333333333333333) * (0.5)), 0.5);
                s.store_sqrt(2160, 2159);
                s.store_mul_ad_affine_product_lhs(2094, s.ad_value(209), A::sqrt(s.ad_value(2159)), -1.0, 0.0, 339);
                s.store_mul_ad_affine_product_rhs(2121, 209, s.ad_value(154), A::add(s.ad_value(2160), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2160), 1.0)), -1.0, 0.0);
                s.store_neg(2123, 2121);
            }
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2217])) && (!s.b[2218])) && (!s.b[2219])) {
                s.store_offset_scaled(2159, 339, ((0.3333333333333333) * (0.5)), 0.5);
                s.store_sqrt(2160, 2159);
                s.store_mul_ad_affine_product_lhs(2094, s.ad_value(209), A::sqrt(s.ad_value(2159)), -1.0, 0.0, 339);
                s.store_mul_ad_affine_product_rhs(2121, 209, s.ad_value(154), A::add(s.ad_value(2160), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2160), 1.0)), -1.0, 0.0);
                s.store_neg(2123, 2121);
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (s.v[79] != 0.0)) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (s.v[79] == 0.0)) {
                s.store_add_scaled_product_right_ad(2096, 2094, 1.0, 185, A::sub(s.ad_value(85), s.ad_value(2085)), 1.0);
                s.store_sub(2097, 2121, 185);
                s.copy_ad(2098, 2123);
                s.store_sub(2099, 2088, 2133);
                s.store_neg(2100, 2134);
                s.store_sub_from_scalar(2101, 1.0, 2135);
                s.store_add_scaled_products_indices(2102, 2097, 2101, 1.0, 2098, 2100, (-1.0));
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (s.v[79] == 0.0)) {
                s.store_ad_value(2103, {
                    if (s.v[2102] > 0.0) {
                        A::div_scalar_offset_denominator(1.0, s.ad_value(2102), 1e-25, 1.0)
                    } else {
                        A::div_scalar_offset_denominator(1.0, s.ad_value(2102), (-1e-25), 1.0)
                    }
                });
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (s.v[79] == 0.0)) {
                s.copy_ad(2104, 2101);
                s.store_neg(2105, 2098);
                s.store_neg(2106, 2100);
                s.copy_ad(2107, 2097);
                s.store_mul_scaled_ad_rhs(2108, 2103, -1.0, A::add_scaled_products(s.ad_value(2104), s.ad_value(2096), 1.0, s.ad_value(2105), s.ad_value(2099), 1.0));
                s.store_mul_scaled_ad_rhs(2109, 2103, -1.0, A::add_scaled_products(s.ad_value(2106), s.ad_value(2096), 1.0, s.ad_value(2107), s.ad_value(2099), 1.0));
                s.store_abs(335, 2108);
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (s.v[79] == 0.0)) {
                s.store_ad_value(335, {
                    if (s.v[335] < ((s.v[2109]) as f64).abs()) {
                        A::abs(s.ad_value(2109))
                    } else {
                        s.ad_value(335)
                    }
                });
            }
            s.b[2220] = (s.v[335] > 0.1);
            s.v[2220] = if s.b[2220] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (s.v[79] == 0.0)) && s.b[2220]) {
                s.store_mul_div_from_scalar_rhs(2108, 2108, 0.1, 335);
                s.store_mul_div_from_scalar_rhs(2109, 2109, 0.1, 335);
            }
            s.b[2221] = (s.v[335] < 1e-10);
            s.v[2221] = if s.b[2221] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (s.v[79] == 0.0)) && s.b[2221]) {
                s.store_scalar(79, 1.0);
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (s.v[79] == 0.0)) {
                s.store_add(2085, 2085, 2108);
                s.store_add(2088, 2088, 2109);
            }
            if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
                s.store_offset(97, 97, 1.0);
            }
        }

    }

    pub(super) fn stamp_reactive_block_43(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_mul_sub_rhs(339, 154, 2085, 2088);
            s.store_exp(340, 339);
            s.store_offset_sub_ad(344, A::offset(s.ad_value(340), (-1.0)), s.ad_value(339), 1e-15);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_ad_value(2118, {
                if (s.v[339] > 0.0) {
                    A::mul_scaled_lhs(s.ad_value(209), -1.0, A::sqrt(s.ad_value(344)))
                } else {
                    A::mul(s.ad_value(209), A::sqrt(s.ad_value(344)))
                }
            });
        }

        s.b[2223] = (1.0 == 1.0);
        s.v[2223] = if s.b[2223] { 1.0 } else { 0.0 };

        s.b[2224] = (((s.v[2085] - s.v[2083]) < p.p403) && (p.p403 >= 0.0));
        s.v[2224] = if s.b[2224] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) {
            s.store_sub_from_scalar_ad(781, p.p403, A::sub(s.ad_value(2085), s.ad_value(2083)));
            s.store_square(722, 781);
            s.store_scalar(723, (p.p403 * p.p403));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2225] = ((((6.0 == 1.0) || (6.0 == 2.0)) || (6.0 == 4.0)) || (6.0 == 8.0));
        s.v[2225] = if s.b[2225] { 1.0 } else { 0.0 };

        s.b[2226] = (6.0 == 1.0);
        s.v[2226] = if s.b[2226] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) && s.b[2225]) && s.b[2226]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2227] = (6.0 == 2.0);
        s.v[2227] = if s.b[2227] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) && s.b[2225]) && (!s.b[2226])) && s.b[2227]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2228] = (6.0 == 4.0);
        s.v[2228] = if s.b[2228] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) && s.b[2225]) && (!s.b[2226])) && (!s.b[2227])) && s.b[2228]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2229] = (6.0 == 8.0);
        s.v[2229] = if s.b[2229] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) && s.b[2225]) && (!s.b[2226])) && (!s.b[2227])) && (!s.b[2228])) && s.b[2229]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) && s.b[2225]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign47570_loop_guard: usize = 0;
        while {
            let assign47570_cond_e67457: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) && s.b[2225]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign47570_cond_e67457 != 0.0
        } {
            assign47570_loop_guard += 1;
            assert!(assign47570_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) && s.b[2225]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) && (!s.b[2225])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 6.0)))
                }
            });
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, p.p403);
            s.store_div_scaled_product_indices(334, 725, 726, p.p403, 770, 1.0);
            s.store_sub_from_scalar(336, p.p403, 780);
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) {
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && (!s.b[2224])) {
            s.store_sub(336, 2085, 2083);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) {
            s.store_offset_ad(338, A::add_scaled_product(A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0)), 1e-15);
            s.store_mul_scaled_ad_rhs(2114, 209, -1.0, A::sqrt(s.ad_value(338)));
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2223])) {
            s.copy_ad(2114, 2118);
        }

        s.b[2230] = (1.0 == 1.0);
        s.v[2230] = if s.b[2230] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) {
            s.copy_ad(2155, 85);
            s.store_offset_mul(338, 2131, 2155, 1.0);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) {
            s.store_ad_value(337, {
                if (s.v[338] > 0.0) {
                    A::sqrt(s.ad_value(338))
                } else {
                    A::neg(A::sqrt_scaled_input(s.ad_value(338), -1.0))
                }
            });
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) {
            s.store_offset_add_ad(2156, s.ad_value(2155), A::mul_sub_from_scalar_rhs(s.ad_value(2130), 1.0, s.ad_value(337)), p.p397);
            s.copy_ad(2152, 2156);
            s.store_scalar(79, 0.0);
            s.store_scalar(97, 1.0);
        }

        let mut assign47770_loop_guard: usize = 0;
        while {
            let assign47770_cond_e67832: f64 = if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign47770_cond_e67832 != 0.0
        } {
            assign47770_loop_guard += 1;
            assert!(assign47770_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) {
                s.store_mul_neg_lhs(335, 154, 2152);
                s.store_exp(336, 335);
                s.store_sqrt_ad(338, A::div_scaled_inputs(s.ad_value(2110), 2.0, s.ad_value(154), 1.0));
                s.store_offset_sub(344, 336, 335, (-1.0));
                s.store_mul_sqrt_ad_rhs(2153, 338, A::offset(s.ad_value(344), 1e-15));
            }
            s.b[2231] = (s.v[335] > 0.0);
            s.v[2231] = if s.b[2231] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) && s.b[2231]) {
                s.store_neg(2153, 2153);
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) {
                s.store_div_scaled_product3(345, s.ad_value(338), s.ad_value(338), s.ad_value(154), 0.5, s.ad_value(2153), 1.0);
                s.store_mul_sub_from_scalar_rhs(2154, 345, 1.0, 336);
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) && (s.v[79] != 0.0)) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) && (s.v[79] == 0.0)) {
                s.store_ad_value(2096, A::add_scaled_offset_product_rhs(s.ad_value(2153), 1.0, s.ad_value(185), A::sub(s.ad_value(2155), s.ad_value(2152)), p.p397, -1.0));
                s.store_add(2097, 185, 2154);
                s.store_div_scaled_inputs(2108, s.ad_value(2096), -1.0, s.ad_value(2097), 1.0);
            }
            s.b[2232] = (((s.v[2108]) as f64).abs() < 1e-10);
            s.v[2232] = if s.b[2232] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) && (s.v[79] == 0.0)) && s.b[2232]) {
                s.store_scalar(79, 1.0);
            }
            s.b[2233] = (s.v[2108] > 0.1);
            s.v[2233] = if s.b[2233] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) && (s.v[79] == 0.0)) && (!s.b[2232])) && s.b[2233]) {
                s.store_scalar(2108, 0.1);
            }
            s.b[2234] = (s.v[2108] < (-0.1));
            s.v[2234] = if s.b[2234] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) && (s.v[79] == 0.0)) && (!s.b[2232])) && (!s.b[2233])) && s.b[2234]) {
                s.store_scalar(2108, (-0.1));
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) && (s.v[79] == 0.0)) {
                s.store_add(2152, 2152, 2108);
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) {
            s.copy_ad(2157, 2152);
            s.store_sqrt_square_offset(782, 2157, ((4.0 * p.p404) * p.p404));
            s.store_offset_scaled_div(334, 2157, 782, 0.5, 0.5);
            s.store_scaled_add(2158, 2157, 782, 0.5);
        }

        s.b[2235] = (s.v[2158] < 0.0);
        s.v[2235] = if s.b[2235] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) && s.b[2235]) {
            s.store_scalar(2158, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) {
            s.store_offset_mul(338, 2131, 85, 1.0);
            s.store_offset(339, 2131, 1.0);
        }

        s.b[2236] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));
        s.v[2236] = if s.b[2236] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) {
            s.store_sub(781, 339, 338);
            s.store_square(722, 781);
            s.store_square(723, 339);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2237] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2237] = if s.b[2237] { 1.0 } else { 0.0 };

        s.b[2238] = (2.0 == 1.0);
        s.v[2238] = if s.b[2238] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) && s.b[2237]) && s.b[2238]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2239] = (2.0 == 2.0);
        s.v[2239] = if s.b[2239] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) && s.b[2237]) && (!s.b[2238])) && s.b[2239]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2240] = (2.0 == 4.0);
        s.v[2240] = if s.b[2240] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) && s.b[2237]) && (!s.b[2238])) && (!s.b[2239])) && s.b[2240]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2241] = (2.0 == 8.0);
        s.v[2241] = if s.b[2241] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) && s.b[2237]) && (!s.b[2238])) && (!s.b[2239])) && (!s.b[2240])) && s.b[2241]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) && s.b[2237]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign48130_loop_guard: usize = 0;
        while {
            let assign48130_cond_e68727: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) && s.b[2237]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign48130_cond_e68727 != 0.0
        } {
            assign48130_loop_guard += 1;
            assert!(assign48130_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) && s.b[2237]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) && (!s.b[2237])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 339, 726);
            s.store_div_scaled_product3(334, s.ad_value(339), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0);
            s.store_sub(338, 339, 780);
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) {
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && (!s.b[2236])) {
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && (!s.b[2236])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) {
            s.store_sqrt(337, 338);
        }

    }

    pub(super) fn stamp_reactive_block_44(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) {
            s.store_add_ad_rhs(344, 85, A::mul_sub_from_scalar_rhs(s.ad_value(2130), 1.0, s.ad_value(337)));
        }

        s.b[2242] = ((s.v[344] < p.p404) && (p.p404 >= 0.0));
        s.v[2242] = if s.b[2242] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) {
            s.store_sub_from_scalar(781, p.p404, 344);
            s.store_square(722, 781);
            s.store_scalar(723, (p.p404 * p.p404));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2243] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2243] = if s.b[2243] { 1.0 } else { 0.0 };

        s.b[2244] = (2.0 == 1.0);
        s.v[2244] = if s.b[2244] { 1.0 } else { 0.0 };

        if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) && s.b[2243]) && s.b[2244]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2245] = (2.0 == 2.0);
        s.v[2245] = if s.b[2245] { 1.0 } else { 0.0 };

        if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) && s.b[2243]) && (!s.b[2244])) && s.b[2245]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2246] = (2.0 == 4.0);
        s.v[2246] = if s.b[2246] { 1.0 } else { 0.0 };

        if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) && s.b[2243]) && (!s.b[2244])) && (!s.b[2245])) && s.b[2246]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2247] = (2.0 == 8.0);
        s.v[2247] = if s.b[2247] { 1.0 } else { 0.0 };

        if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) && s.b[2243]) && (!s.b[2244])) && (!s.b[2245])) && (!s.b[2246])) && s.b[2247]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) && s.b[2243]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign48500_loop_guard: usize = 0;
        while {
            let assign48500_cond_e69390: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) && s.b[2243]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign48500_cond_e69390 != 0.0
        } {
            assign48500_loop_guard += 1;
            assert!(assign48500_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) && s.b[2243]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) && (!s.b[2243])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, p.p404);
            s.store_div_scaled_product_indices(334, 725, 726, p.p404, 770, 1.0);
            s.store_sub_from_scalar(2158, p.p404, 780);
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) {
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && (!s.b[2242])) {
            s.copy_ad(2158, 344);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.copy_ad(349, 790);
            s.store_div(335, 790, 2158);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_ad_value(336, {
                if (s.v[335] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(335), s.ad_value(658))
                }
            });
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_offset(337, 336, 1.0);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_ad_value(338, {
                if (s.v[337] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(337), A::div_from_scalar(1.0, s.ad_value(658)))
                }
            });
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_div(348, 790, 338);
            s.copy_ad(790, 348);
        }

        s.b[2248] = (s.v[790] < 0.0);
        s.v[2248] = if s.b[2248] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2248]) {
            s.copy_ad(2086, 2085);
            s.copy_ad(2091, 2090);
            s.copy_ad(2089, 2088);
            s.copy_ad(2119, 2118);
            s.copy_ad(2115, 2114);
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
            s.copy_ad(2084, 790);
            s.store_add_scaled_inputs3_offset(781, s.ad_value(2085), 1.0, s.ad_value(2084), 1.0, s.ad_value(85), -1.0, (-0.01));
            s.store_scaled_add(782, 2085, 2084, (4.0 * 0.01));
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4(2093, s.ad_value(2085), 1.0, s.ad_value(2084), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5));
            s.store_add_scaled_inputs3_offset(781, s.ad_value(2093), 1.0, s.ad_value(2113), -1.0, s.ad_value(2087), 1.0, (-0.01));
            s.store_scaled_sub(782, 2113, 2087, (4.0 * 0.01));
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4(2093, s.ad_value(2113), 1.0, s.ad_value(2087), (-1.0), s.ad_value(781), 0.5, s.ad_value(782), 0.5);
            s.copy_ad(2089, 2084);
            s.copy_ad(2086, 2093);
            s.store_scalar(79, 0.0);
            s.store_mul(2137, 2125, 2126);
            s.store_scalar(98, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_45(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign48900_loop_guard: usize = 0;
        while {
            let assign48900_cond_e70099: f64 = if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[98] <= 150.0)) { 1.0 } else { 0.0 };
            assign48900_cond_e70099 != 0.0
        } {
            assign48900_loop_guard += 1;
            assert!(assign48900_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
                s.store_mul_sub_ad_rhs(2091, 2125, A::add_scaled_product(s.ad_value(2113), 1.0, s.ad_value(2126), s.ad_value(2089), 1.0), s.ad_value(2087));
                s.store_sub(335, 2089, 2091);
            }
            s.b[2249] = ((s.v[335] < 0.001) && (0.001 >= 0.0));
            s.v[2249] = if s.b[2249] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) {
                s.store_sub_from_scalar(781, 0.001, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.001 * 0.001));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[2250] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[2250] = if s.b[2250] { 1.0 } else { 0.0 };
            s.b[2251] = (2.0 == 1.0);
            s.v[2251] = if s.b[2251] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) && s.b[2251]) {
                s.store_scalar(720, 1.0);
            }
            s.b[2252] = (2.0 == 2.0);
            s.v[2252] = if s.b[2252] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) && (!s.b[2251])) && s.b[2252]) {
                s.store_scalar(720, 2.0);
            }
            s.b[2253] = (2.0 == 4.0);
            s.v[2253] = if s.b[2253] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) && (!s.b[2251])) && (!s.b[2252])) && s.b[2253]) {
                s.store_scalar(720, 3.0);
            }
            s.b[2254] = (2.0 == 8.0);
            s.v[2254] = if s.b[2254] { 1.0 } else { 0.0 };
            if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) && (!s.b[2251])) && (!s.b[2252])) && (!s.b[2253])) && s.b[2254]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign48900_body28_loop_guard: usize = 0;
            while {
                let assign48900_body28_cond_e70566: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign48900_body28_cond_e70566 != 0.0
            } {
                assign48900_body28_loop_guard += 1;
                assert!(assign48900_body28_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && s.b[2250]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) && (!s.b[2250])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.001);
                s.store_div_scaled_product_indices(336, 725, 726, 0.001, 770, 1.0);
                s.store_sub_from_scalar(335, 0.001, 780);
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2249]) {
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2249])) {
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2249])) {
                s.store_scalar(336, 1.0);
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
                s.store_sqrt_mul(2082, 2132, 335);
            }
            s.b[2255] = ((s.v[2082] > (s.v[2127] - 1e-12)) && (1e-12 >= 0.0));
            s.v[2255] = if s.b[2255] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) {
                s.store_offset_sub(781, 2082, 2127, 1e-12);
                s.store_square(722, 781);
                s.store_scalar(723, (1e-12 * 1e-12));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[2256] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[2256] = if s.b[2256] { 1.0 } else { 0.0 };
            s.b[2257] = (2.0 == 1.0);
            s.v[2257] = if s.b[2257] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) && s.b[2257]) {
                s.store_scalar(720, 1.0);
            }
            s.b[2258] = (2.0 == 2.0);
            s.v[2258] = if s.b[2258] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) && (!s.b[2257])) && s.b[2258]) {
                s.store_scalar(720, 2.0);
            }
            s.b[2259] = (2.0 == 4.0);
            s.v[2259] = if s.b[2259] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) && (!s.b[2257])) && (!s.b[2258])) && s.b[2259]) {
                s.store_scalar(720, 3.0);
            }
            s.b[2260] = (2.0 == 8.0);
            s.v[2260] = if s.b[2260] { 1.0 } else { 0.0 };
            if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) && (!s.b[2257])) && (!s.b[2258])) && (!s.b[2259])) && s.b[2260]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign48900_body64_loop_guard: usize = 0;
            while {
                let assign48900_body64_cond_e71211: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign48900_body64_cond_e71211 != 0.0
            } {
                assign48900_body64_loop_guard += 1;
                assert!(assign48900_body64_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && s.b[2256]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) && (!s.b[2256])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-12);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-12, 770, 1.0);
                s.store_add_ad_lhs(2082, A::offset(s.ad_value(2127), (-1e-12)), 780);
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2255]) {
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2255])) {
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2255])) {
                s.store_scalar(337, 1.0);
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
                s.store_mul(337, 336, 337);
                s.store_add_ad_rhs(2133, 2086, A::div(A::add_scaled_square_product(s.ad_value(2127), 1.0, s.ad_value(2082), A::sub_scaled_inputs(s.ad_value(2082), 1.0, s.ad_value(2127), 2.0), 1.0), s.ad_value(2132)));
                s.store_scalar(2134, 1.0);
                s.store_ad_value(2135, A::mul_sub_from_scalar_rhs(A::add_scaled_product(s.ad_value(337), 1.0, A::div(s.ad_value(2127), s.ad_value(2082)), s.ad_value(337), (-1.0)), 1.0, s.ad_value(2137)));
            }
            s.b[2261] = ((s.v[2133] > (s.v[2084] - p.p406)) && (p.p406 >= 0.0));
            s.v[2261] = if s.b[2261] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) {
                s.store_offset_sub(781, 2133, 2084, p.p406);
                s.store_square(722, 781);
                s.store_scalar(723, (p.p406 * p.p406));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
                s.store_scalar(719, 0.0);
                s.store_scalar(720, 0.0);
                s.store_scalar(770, 0.0);
                s.store_scalar(726, 0.0);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_add(770, 724, 725);
                s.copy_ad(726, 770);
            }
            s.b[2262] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[2262] = if s.b[2262] { 1.0 } else { 0.0 };
            s.b[2263] = (4.0 == 1.0);
            s.v[2263] = if s.b[2263] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) && s.b[2263]) {
                s.store_scalar(720, 1.0);
            }
            s.b[2264] = (4.0 == 2.0);
            s.v[2264] = if s.b[2264] { 1.0 } else { 0.0 };
            if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) && (!s.b[2263])) && s.b[2264]) {
                s.store_scalar(720, 2.0);
            }
            s.b[2265] = (4.0 == 4.0);
            s.v[2265] = if s.b[2265] { 1.0 } else { 0.0 };
            if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) && (!s.b[2263])) && (!s.b[2264])) && s.b[2265]) {
                s.store_scalar(720, 3.0);
            }
            s.b[2266] = (4.0 == 8.0);
            s.v[2266] = if s.b[2266] { 1.0 } else { 0.0 };
            if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) && (!s.b[2263])) && (!s.b[2264])) && (!s.b[2265])) && s.b[2266]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign48900_body107_loop_guard: usize = 0;
            while {
                let assign48900_body107_cond_e71993: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign48900_body107_cond_e71993 != 0.0
            } {
                assign48900_body107_loop_guard += 1;
                assert!(assign48900_body107_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && s.b[2262]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) && (!s.b[2262])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                    }
                });
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, p.p406);
                s.store_div_scaled_product_indices(334, 725, 726, p.p406, 770, 1.0);
                s.store_add_ad_lhs(2133, A::offset(s.ad_value(2084), (-p.p406)), 780);
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2261]) {
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2261])) {
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2261])) {
                s.store_scalar(334, 1.0);
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
                s.store_mul(2134, 2134, 334);
                s.store_mul(2135, 2135, 334);
                s.store_mul_sub_rhs(339, 154, 2086, 2089);
                s.store_exp(340, 339);
                s.store_sub_ad_lhs(344, A::offset(s.ad_value(340), (-1.0)), 339);
            }
            s.b[2267] = (s.v[339] >= 1e-7);
            s.v[2267] = if s.b[2267] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && s.b[2267]) {
                s.store_scalar(347, (-1.0));
                s.store_mul_scaled_ad_rhs(2095, 209, -1.0, A::sqrt(s.ad_value(344)));
                s.store_div_scaled_product3(345, s.ad_value(209), s.ad_value(209), s.ad_value(154), 0.5, s.ad_value(2095), 1.0);
                s.store_mul_offset_rhs(2122, 345, 340, (-1.0));
                s.store_mul_sub_from_scalar_rhs(2124, 345, 1.0, 340);
            }
            s.b[2268] = (s.v[339] < (-1e-7));
            s.v[2268] = if s.b[2268] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2267])) && s.b[2268]) {
                s.store_scalar(347, 1.0);
                s.store_exp_ad(342, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub_scaled_inputs(s.ad_value(2086), 1.0, s.ad_value(2113), p.p398)));
                s.store_exp_ad(343, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub_scaled_inputs(s.ad_value(2089), 1.0, s.ad_value(2113), p.p398)));
                s.store_mul_sqrt_ad_rhs(2095, 209, A::add_scaled_product(s.ad_value(344), 1.0, s.ad_value(210), A::add_scaled_inputs3(s.ad_value(342), 1.0, s.ad_value(343), (-1.0), s.ad_value(339), 1.0), 1.0));
                s.store_div_scaled_product3(345, s.ad_value(209), s.ad_value(209), s.ad_value(154), 0.5, s.ad_value(2095), 1.0);
                s.store_mul_add_ad_rhs(2122, 345, A::offset(s.ad_value(340), (-1.0)), A::mul_sub_from_scalar_rhs(s.ad_value(210), 1.0, s.ad_value(342)));
                s.store_mul_ad_rhs(2124, 345, A::add_scaled_sub_value_product(1.0, s.ad_value(340), 1.0, s.ad_value(210), A::offset(s.ad_value(343), (-1.0)), 1.0));
            }
            s.b[2269] = (s.v[339] > 0.0);
            s.v[2269] = if s.b[2269] { 1.0 } else { 0.0 };
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2267])) && (!s.b[2268])) && s.b[2269]) {
                s.store_offset_scaled(2159, 339, ((0.3333333333333333) * (0.5)), 0.5);
                s.store_sqrt(2160, 2159);
                s.store_mul_ad_affine_product_lhs(2095, s.ad_value(209), A::sqrt(s.ad_value(2159)), -1.0, 0.0, 339);
                s.store_mul_ad_affine_product_rhs(2122, 209, s.ad_value(154), A::add(s.ad_value(2160), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2160), 1.0)), -1.0, 0.0);
                s.store_neg(2124, 2122);
            }
            if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (!s.b[2267])) && (!s.b[2268])) && (!s.b[2269])) {
                s.store_offset_scaled(2159, 339, ((0.3333333333333333) * (0.5)), 0.5);
                s.store_sqrt(2160, 2159);
                s.store_mul_ad_affine_product_lhs(2095, s.ad_value(209), A::sqrt(s.ad_value(2159)), -1.0, 0.0, 339);
                s.store_mul_ad_affine_product_rhs(2122, 209, s.ad_value(154), A::add(s.ad_value(2160), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2160), 1.0)), -1.0, 0.0);
                s.store_neg(2124, 2122);
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] != 0.0)) {
                s.store_scalar(98, (150.0 + 1.0));
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) {
                s.store_add_scaled_product_right_ad(2096, 2095, 1.0, 185, A::sub(s.ad_value(85), s.ad_value(2086)), 1.0);
                s.store_sub(2097, 2122, 185);
                s.copy_ad(2098, 2124);
                s.store_sub(2099, 2089, 2133);
                s.store_neg(2100, 2134);
                s.store_sub_from_scalar(2101, 1.0, 2135);
                s.store_add_scaled_products_indices(2102, 2097, 2101, 1.0, 2098, 2100, (-1.0));
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) {
                s.store_ad_value(2103, {
                    if (s.v[2102] > 0.0) {
                        A::div_scalar_offset_denominator(1.0, s.ad_value(2102), 1e-25, 1.0)
                    } else {
                        A::div_scalar_offset_denominator(1.0, s.ad_value(2102), (-1e-25), 1.0)
                    }
                });
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) {
                s.copy_ad(2104, 2101);
                s.store_neg(2105, 2098);
                s.store_neg(2106, 2100);
                s.copy_ad(2107, 2097);
                s.store_mul_scaled_ad_rhs(2108, 2103, -1.0, A::add_scaled_products(s.ad_value(2104), s.ad_value(2096), 1.0, s.ad_value(2105), s.ad_value(2099), 1.0));
                s.store_mul_scaled_ad_rhs(2109, 2103, -1.0, A::add_scaled_products(s.ad_value(2106), s.ad_value(2096), 1.0, s.ad_value(2107), s.ad_value(2099), 1.0));
                s.store_abs(335, 2108);
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) {
                s.store_ad_value(335, {
                    if (s.v[335] < ((s.v[2109]) as f64).abs()) {
                        A::abs(s.ad_value(2109))
                    } else {
                        s.ad_value(335)
                    }
                });
            }
            s.b[2270] = (s.v[335] > 0.1);
            s.v[2270] = if s.b[2270] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) && s.b[2270]) {
                s.store_mul_div_from_scalar_rhs(2108, 2108, 0.1, 335);
                s.store_mul_div_from_scalar_rhs(2109, 2109, 0.1, 335);
            }
            s.b[2271] = (s.v[335] < 1e-10);
            s.v[2271] = if s.b[2271] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) && s.b[2271]) {
                s.store_scalar(79, 1.0);
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) && (s.v[79] == 0.0)) {
                s.store_add(2086, 2086, 2108);
                s.store_add(2089, 2089, 2109);
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
                s.store_offset(98, 98, 1.0);
            }
        }

    }
}

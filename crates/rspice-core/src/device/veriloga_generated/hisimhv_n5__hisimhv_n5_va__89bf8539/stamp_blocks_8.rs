#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_30(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_offset_ad(781, A::add_scaled_inputs3(s.ad_value(1858), 1.0, s.ad_value(1885), -1.0, s.ad_value(1853), 1.0), (-0.01));
            s.store_scaled_sub(782, 1885, 1853, (4.0 * 0.01));
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_ad(1858, A::sub(s.ad_value(1885), s.ad_value(1853)), A::add_scaled_inputs(s.ad_value(781), 0.5, s.ad_value(782), 0.5));
            s.store_ad_value(1886, A::div_scaled_product(s.ad_value(1853), s.ad_value(622), -1.0, A::add(s.ad_value(622), s.ad_value(964)), 1.0));
            s.store_offset_sub(1832, 965, 1833, 1e-15);
            s.store_scalar(79, 0.0);
            s.store_scalar(1848, 0.2);
            s.copy_ad(1851, 1858);
            s.copy_ad(1854, 1849);
            s.copy_ad(1856, 1886);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_31(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign38090_loop_guard: usize = 0;
        while {
            let assign38090_cond_e43956: f64 = if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign38090_cond_e43956 != 0.0
        } {
            assign38090_loop_guard += 1;
            assert!(assign38090_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
                s.store_mul_sub_ad_rhs(1856, 1899, A::add_scaled_product(s.ad_value(1885), 1.0, s.ad_value(1900), s.ad_value(1854), 1.0), s.ad_value(1853));
                s.store_mul(1840, 1899, 1900);
                s.store_sub(335, 1854, 1856);
            }
            s.b[1923] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1923] = if s.b[1923] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) {
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
            s.b[1924] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1924] = if s.b[1924] { 1.0 } else { 0.0 };
            s.b[1925] = (2.0 == 1.0);
            s.v[1925] = if s.b[1925] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) && s.b[1925]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1926] = (2.0 == 2.0);
            s.v[1926] = if s.b[1926] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) && (!s.b[1925])) && s.b[1926]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1927] = (2.0 == 4.0);
            s.v[1927] = if s.b[1927] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) && (!s.b[1925])) && (!s.b[1926])) && s.b[1927]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1928] = (2.0 == 8.0);
            s.v[1928] = if s.b[1928] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) && (!s.b[1925])) && (!s.b[1926])) && (!s.b[1927])) && s.b[1928]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign38090_body29_loop_guard: usize = 0;
            while {
                let assign38090_body29_cond_e44319: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38090_body29_cond_e44319 != 0.0
            } {
                assign38090_body29_loop_guard += 1;
                assert!(assign38090_body29_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && s.b[1924]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) && (!s.b[1924])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_ad_value(334, A::div_scaled_product(s.ad_value(725), s.ad_value(726), 0.1, s.ad_value(770), 1.0));
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1923]) {
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1923])) {
                s.copy_ad(336, 335);
                s.store_scalar(334, 1.0);
            }
            if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
                s.store_sqrt_mul(1828, 1906, 336);
            }
            s.b[1929] = ((s.v[1828] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.v[1929] = if s.b[1929] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) {
                s.store_offset_sub(781, 1828, 965, 1e-8);
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
            s.b[1930] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1930] = if s.b[1930] { 1.0 } else { 0.0 };
            s.b[1931] = (2.0 == 1.0);
            s.v[1931] = if s.b[1931] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) && s.b[1931]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1932] = (2.0 == 2.0);
            s.v[1932] = if s.b[1932] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) && (!s.b[1931])) && s.b[1932]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1933] = (2.0 == 4.0);
            s.v[1933] = if s.b[1933] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) && (!s.b[1931])) && (!s.b[1932])) && s.b[1933]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1934] = (2.0 == 8.0);
            s.v[1934] = if s.b[1934] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) && (!s.b[1931])) && (!s.b[1932])) && (!s.b[1933])) && s.b[1934]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign38090_body65_loop_guard: usize = 0;
            while {
                let assign38090_body65_cond_e44804: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38090_body65_cond_e44804 != 0.0
            } {
                assign38090_body65_loop_guard += 1;
                assert!(assign38090_body65_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && s.b[1930]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) && (!s.b[1930])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_ad_value(337, A::div_scaled_product(s.ad_value(725), s.ad_value(726), 1e-8, s.ad_value(770), 1.0));
                s.store_add_ad_lhs(1828, A::offset(s.ad_value(965), (-1e-8)), 780);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1929]) {
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1929])) {
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1929])) {
                s.store_scalar(337, 1.0);
            }
            if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
                s.store_mul(1864, 1828, 1903);
                s.store_mul_ad_product_lhs(1842, A::div_from_scalar(1.034943e-10, s.ad_value(1828)), s.ad_value(334), 337);
                s.store_mul_ad_product_lhs(1844, A::div_from_scalar((-1.034943e-10), s.ad_value(1828)), s.ad_value(334), 337);
            }
            s.b[1935] = (p.p49 == 0.0);
            s.v[1935] = if s.b[1935] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1935]) {
                s.store_add_ad_lhs(1837, A::div(A::add_scaled_product(A::add(s.ad_value(1901), s.ad_value(1836)), 1.0, s.ad_value(965), s.ad_value(1833), (-2.0)), s.ad_value(1906)), 1851);
                s.store_scalar(1838, 1.0);
                s.store_scalar(1839, 0.0);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1935])) {
                s.store_add_ad_rhs(1837, 1851, A::div(A::add_scaled_product(s.ad_value(1901), 1.0, s.ad_value(1828), A::sub_scaled_inputs(s.ad_value(1828), 1.0, s.ad_value(965), 2.0), 1.0), s.ad_value(1906)));
                s.store_scalar(1838, 1.0);
                s.store_mul_ad(1839, A::add_scaled_product(s.ad_value(334), 1.0, A::div(s.ad_value(965), s.ad_value(1828)), s.ad_value(334), (-1.0)), A::sub_from_scalar(1.0, s.ad_value(1840)));
            }
            s.b[1936] = ((s.v[1837] > (s.v[1849] - s.v[1848])) && (s.v[1848] >= 0.0));
            s.v[1936] = if s.b[1936] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) {
                s.store_ad_value(781, A::add_scaled_inputs3(s.ad_value(1837), 1.0, s.ad_value(1849), (-1.0), s.ad_value(1848), 1.0));
                s.store_square(722, 781);
                s.store_square(723, 1848);
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
            s.b[1937] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[1937] = if s.b[1937] { 1.0 } else { 0.0 };
            s.b[1938] = (4.0 == 1.0);
            s.v[1938] = if s.b[1938] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) && s.b[1938]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1939] = (4.0 == 2.0);
            s.v[1939] = if s.b[1939] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) && (!s.b[1938])) && s.b[1939]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1940] = (4.0 == 4.0);
            s.v[1940] = if s.b[1940] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) && (!s.b[1938])) && (!s.b[1939])) && s.b[1940]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1941] = (4.0 == 8.0);
            s.v[1941] = if s.b[1941] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) && (!s.b[1938])) && (!s.b[1939])) && (!s.b[1940])) && s.b[1941]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign38090_body114_loop_guard: usize = 0;
            while {
                let assign38090_body114_cond_e45477: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38090_body114_cond_e45477 != 0.0
            } {
                assign38090_body114_loop_guard += 1;
                assert!(assign38090_body114_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && s.b[1937]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) && (!s.b[1937])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                    }
                });
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1848, 726);
                s.store_ad_value(334, A::div_scaled_product3(s.ad_value(1848), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0));
                s.store_ad_value(1837, A::add_scaled_inputs3(s.ad_value(1849), 1.0, s.ad_value(1848), (-1.0), s.ad_value(780), 1.0));
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1936]) {
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1936])) {
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1936])) {
                s.store_scalar(334, 1.0);
            }
            if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
                s.store_mul(1838, 1838, 334);
                s.store_mul(1839, 1839, 334);
                s.store_ad_value(335, A::add_scaled_inputs3(s.ad_value(1856), 1.0, s.ad_value(1885), (-1.0), s.ad_value(1853), 1.0));
            }
            s.b[1942] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1942] = if s.b[1942] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) {
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
            s.b[1943] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1943] = if s.b[1943] { 1.0 } else { 0.0 };
            s.b[1944] = (2.0 == 1.0);
            s.v[1944] = if s.b[1944] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) && s.b[1944]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1945] = (2.0 == 2.0);
            s.v[1945] = if s.b[1945] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) && (!s.b[1944])) && s.b[1945]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1946] = (2.0 == 4.0);
            s.v[1946] = if s.b[1946] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) && (!s.b[1944])) && (!s.b[1945])) && s.b[1946]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1947] = (2.0 == 8.0);
            s.v[1947] = if s.b[1947] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) && (!s.b[1944])) && (!s.b[1945])) && (!s.b[1946])) && s.b[1947]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign38090_body152_loop_guard: usize = 0;
            while {
                let assign38090_body152_cond_e45985: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38090_body152_cond_e45985 != 0.0
            } {
                assign38090_body152_loop_guard += 1;
                assert!(assign38090_body152_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && s.b[1943]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) && (!s.b[1943])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_ad_value(337, A::div_scaled_product(s.ad_value(725), s.ad_value(726), 0.1, s.ad_value(770), 1.0));
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1942]) {
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1942])) {
                s.copy_ad(336, 335);
                s.store_scalar(337, 1.0);
            }
            if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
                s.store_sqrt_mul(1830, 1907, 336);
                s.store_mul_neg_lhs(1865, 1830, 1902);
                s.store_mul_div_from_scalar_lhs(1846, (-1.034943e-10), 1830, 337);
                s.store_mul_sub_rhs(335, 154, 1851, 1854);
                s.store_exp(336, 335);
            }
            s.b[1948] = (s.v[1851] >= s.v[1854]);
            s.v[1948] = if s.b[1948] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1948]) {
                s.store_mul_scaled_ad_rhs(1860, 209, -1.0, A::sqrt(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15)));
                s.store_mul_ad(1895, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(1860), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
                s.store_neg(1897, 1895);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1948])) {
                s.store_exp_ad(337, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1851), s.ad_value(1885))));
                s.store_exp_ad(338, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1854), s.ad_value(1885))));
                s.store_mul_sqrt_ad_rhs(1860, 209, A::offset(A::add_scaled_product(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1.0, s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_ad_value(339, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(1860), 1.0));
                s.store_mul_add_ad_rhs(1895, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1897, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] != 0.0)) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) {
                s.store_ad_value(1868, A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(1860), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1851)), 1.0), 1.0, s.ad_value(1864), 1.0, s.ad_value(1865), 1.0));
                s.store_sub(1869, 1895, 185);
                s.store_add_ad_rhs(1870, 1897, A::add_scaled_product(A::add_scaled_product(s.ad_value(1842), 1.0, s.ad_value(1844), s.ad_value(1840), 1.0), 1.0, s.ad_value(1846), s.ad_value(1840), 1.0));
                s.store_sub(1871, 1854, 1837);
                s.store_neg(1872, 1838);
                s.store_sub_from_scalar(1873, 1.0, 1839);
                s.store_ad_value(1874, A::add_scaled_products(s.ad_value(1869), s.ad_value(1873), 1.0, s.ad_value(1870), s.ad_value(1872), (-1.0)));
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) {
                s.store_ad_value(1875, {
                    if (s.v[1874] > 0.0) {
                        A::div_from_scalar(1.0, A::offset(s.ad_value(1874), 1e-25))
                    } else {
                        A::div_from_scalar(1.0, A::offset(s.ad_value(1874), (-1e-25)))
                    }
                });
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) {
                s.copy_ad(1876, 1873);
                s.store_neg(1877, 1870);
                s.store_neg(1878, 1872);
                s.copy_ad(1879, 1869);
                s.store_mul_scaled_ad_rhs(1880, 1875, -1.0, A::add_scaled_products(s.ad_value(1876), s.ad_value(1868), 1.0, s.ad_value(1877), s.ad_value(1871), 1.0));
                s.store_mul_scaled_ad_rhs(1881, 1875, -1.0, A::add_scaled_products(s.ad_value(1878), s.ad_value(1868), 1.0, s.ad_value(1879), s.ad_value(1871), 1.0));
                s.store_abs(335, 1880);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) {
                s.store_ad_value(335, {
                    if (s.v[335] < ((s.v[1881]) as f64).abs()) {
                        A::abs(s.ad_value(1881))
                    } else {
                        s.ad_value(335)
                    }
                });
            }
            s.b[1949] = (s.v[335] > 0.1);
            s.v[1949] = if s.b[1949] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) && s.b[1949]) {
                s.store_mul_div_from_scalar_rhs(1880, 1880, 0.1, 335);
                s.store_mul_div_from_scalar_rhs(1881, 1881, 0.1, 335);
            }
            s.b[1950] = (s.v[335] < 1e-12);
            s.v[1950] = if s.b[1950] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) && s.b[1950]) {
                s.store_scalar(79, 1.0);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (s.v[79] == 0.0)) {
                s.store_add(1851, 1851, 1880);
                s.store_add(1854, 1854, 1881);
            }
            if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
                s.store_offset(97, 97, 1.0);
            }
        }

    }

    pub(super) fn stamp_reactive_block_32(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_mul_sub_rhs(335, 154, 1851, 1854);
            s.store_exp(336, 335);
        }

        s.b[1952] = (s.v[1851] >= s.v[1854]);
        s.v[1952] = if s.b[1952] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1952]) {
            s.copy_ad(1890, 1860);
            s.store_scalar(1893, 0.0);
            s.store_scalar(1862, 0.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1952])) {
            s.store_scalar(1890, 0.0);
            s.store_mul_sqrt_ad_rhs(1893, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
        }

        s.b[1953] = (s.v[1834] > s.v[965]);
        s.v[1953] = if s.b[1953] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1952])) && s.b[1953]) {
            s.store_scalar(1862, 0.0);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1952])) && (!s.b[1953])) {
            s.store_mul_sqrt_ad_rhs(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1851), s.ad_value(1885)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1854), s.ad_value(1885)))))), s.ad_value(335)));
            s.store_ad_value(1862, A::add_scaled_product(s.ad_value(337), 1.0, s.ad_value(209), A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0)));
        }

        s.b[1954] = (((s.v[1851] - s.v[1849]) < s.v[1909]) && (s.v[1909] >= 0.0));
        s.v[1954] = if s.b[1954] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) {
            s.store_ad_value(781, A::add_scaled_inputs3(s.ad_value(1909), 1.0, s.ad_value(1851), -1.0, s.ad_value(1849), 1.0));
            s.store_square(722, 781);
            s.store_square(723, 1909);
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

        s.b[1955] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1955] = if s.b[1955] { 1.0 } else { 0.0 };

        s.b[1956] = (4.0 == 1.0);
        s.v[1956] = if s.b[1956] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) && s.b[1956]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1957] = (4.0 == 2.0);
        s.v[1957] = if s.b[1957] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) && (!s.b[1956])) && s.b[1957]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1958] = (4.0 == 4.0);
        s.v[1958] = if s.b[1958] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) && (!s.b[1956])) && (!s.b[1957])) && s.b[1958]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1959] = (4.0 == 8.0);
        s.v[1959] = if s.b[1959] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) && (!s.b[1956])) && (!s.b[1957])) && (!s.b[1958])) && s.b[1959]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign38530_loop_guard: usize = 0;
        while {
            let assign38530_cond_e47310: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign38530_cond_e47310 != 0.0
        } {
            assign38530_loop_guard += 1;
            assert!(assign38530_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && s.b[1955]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) && (!s.b[1955])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                }
            });
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1909, 726);
            s.store_ad_value(334, A::div_scaled_product3(s.ad_value(1909), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0));
            s.store_sub(336, 1909, 780);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1954]) {
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1954])) {
            s.store_sub(336, 1851, 1849);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_offset_ad(338, A::add_scaled_product(A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0)), 1e-15);
            s.store_mul_scaled_ad_rhs(1887, 209, -1.0, A::sqrt(s.ad_value(338)));
            s.copy_ad(349, 790);
        }

        s.b[1960] = (s.v[790] > 1e-6);
        s.v[1960] = if s.b[1960] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {
            s.store_scalar(344, 1e-25);
            s.store_offset_mul_ad(338, s.ad_value(1905), A::sub(s.ad_value(85), s.ad_value(344)), 1.0);
            s.store_offset(339, 1905, 1.0);
        }

        s.b[1961] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));
        s.v[1961] = if s.b[1961] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) {
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

        s.b[1962] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1962] = if s.b[1962] { 1.0 } else { 0.0 };

        s.b[1963] = (2.0 == 1.0);
        s.v[1963] = if s.b[1963] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) && s.b[1963]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1964] = (2.0 == 2.0);
        s.v[1964] = if s.b[1964] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) && (!s.b[1963])) && s.b[1964]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1965] = (2.0 == 4.0);
        s.v[1965] = if s.b[1965] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) && (!s.b[1963])) && (!s.b[1964])) && s.b[1965]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1966] = (2.0 == 8.0);
        s.v[1966] = if s.b[1966] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) && (!s.b[1963])) && (!s.b[1964])) && (!s.b[1965])) && s.b[1966]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign38950_loop_guard: usize = 0;
        while {
            let assign38950_cond_e47913: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign38950_cond_e47913 != 0.0
        } {
            assign38950_loop_guard += 1;
            assert!(assign38950_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && s.b[1962]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) && (!s.b[1962])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 339, 726);
            s.store_ad_value(334, A::div_scaled_product3(s.ad_value(339), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0));
            s.store_sub(338, 339, 780);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1961]) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && (!s.b[1961])) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && (!s.b[1961])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {
            s.store_sqrt(337, 338);
            s.store_ad_value(344, A::add_scaled_product(s.ad_value(85), 1.0, s.ad_value(1904), A::sub_from_scalar(1.0, s.ad_value(337)), 1.0));
        }

        s.b[1967] = ((s.v[344] < 1.0) && (1.0 >= 0.0));
        s.v[1967] = if s.b[1967] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) {
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

        s.b[1968] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[1968] = if s.b[1968] { 1.0 } else { 0.0 };

        s.b[1969] = (2.0 == 1.0);
        s.v[1969] = if s.b[1969] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) && s.b[1969]) {
            s.store_scalar(720, 1.0);
        }

        s.b[1970] = (2.0 == 2.0);
        s.v[1970] = if s.b[1970] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) && (!s.b[1969])) && s.b[1970]) {
            s.store_scalar(720, 2.0);
        }

        s.b[1971] = (2.0 == 4.0);
        s.v[1971] = if s.b[1971] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) && (!s.b[1969])) && (!s.b[1970])) && s.b[1971]) {
            s.store_scalar(720, 3.0);
        }

        s.b[1972] = (2.0 == 8.0);
        s.v[1972] = if s.b[1972] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) && (!s.b[1969])) && (!s.b[1970])) && (!s.b[1971])) && s.b[1972]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign39320_loop_guard: usize = 0;
        while {
            let assign39320_cond_e48477: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign39320_cond_e48477 != 0.0
        } {
            assign39320_loop_guard += 1;
            assert!(assign39320_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && s.b[1968]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

    }

    pub(super) fn stamp_reactive_block_33(
        s: &mut ReactiveScratch,
    ) {
        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) && (!s.b[1968])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1.0);
            s.store_ad_value(334, A::div_scaled_product(s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0));
            s.store_sub_from_scalar(344, 1.0, 780);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && s.b[1967]) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && (!s.b[1967])) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) && (!s.b[1967])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {
            s.store_div(335, 790, 344);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {
            s.store_ad_value(336, {
                if (s.v[335] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(335), A::offset(s.ad_value(658), (-1.0)))
                }
            });
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {
            s.store_offset_mul(337, 336, 335, 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {
            s.store_ad_value(338, {
                if (s.v[337] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)))
                }
            });
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1960]) {
            s.store_mul(340, 338, 337);
            s.store_div(348, 790, 340);
            s.copy_ad(790, 348);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1960])) {
            s.copy_ad(348, 790);
        }

        s.b[1973] = (s.v[790] < 0.0);
        s.v[1973] = if s.b[1973] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[1973]) {
            s.copy_ad(1852, 1851);
            s.copy_ad(1857, 1856);
            s.copy_ad(1855, 1854);
            s.copy_ad(1863, 1862);
            s.copy_ad(1891, 1890);
            s.copy_ad(1888, 1887);
            s.copy_ad(1866, 1864);
            s.copy_ad(1867, 1865);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            s.copy_ad(1835, 1834);
            s.copy_ad(1850, 790);
            s.store_offset_ad(781, A::add_scaled_inputs3(s.ad_value(1851), 1.0, s.ad_value(1850), 1.0, s.ad_value(85), -1.0), (-0.01));
            s.store_scaled_add(782, 1851, 1850, (4.0 * 0.01));
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_sub_ad(1859, A::add(s.ad_value(1851), s.ad_value(1850)), A::add_scaled_inputs(s.ad_value(781), 0.5, s.ad_value(782), 0.5));
            s.store_offset_ad(781, A::add_scaled_inputs3(s.ad_value(1859), 1.0, s.ad_value(1885), -1.0, s.ad_value(1853), 1.0), (-0.01));
            s.store_scaled_sub(782, 1885, 1853, (4.0 * 0.01));
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_ad(1859, A::sub(s.ad_value(1885), s.ad_value(1853)), A::add_scaled_inputs(s.ad_value(781), 0.5, s.ad_value(782), 0.5));
            s.store_mul(212, 209, 186);
            s.store_square(213, 212);
            s.store_offset_div_ad(332, A::scaled_offset(A::mul(s.ad_value(154), A::sub(s.ad_value(85), s.ad_value(1885))), (-1.0), 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            s.store_ad_value(332, {
                if (s.v[332] >= (10.0 * 2.220446049250313e-16)) {
                    s.ad_value(332)
                } else {
                    A::constant((10.0 * 2.220446049250313e-16))
                }
            });
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            s.store_add_ad_rhs(92, 85, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 0.5));
            s.store_scalar(79, 0.0);
            s.copy_ad(1852, 1859);
            s.copy_ad(1855, 1850);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_34(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign39810_loop_guard: usize = 0;
        while {
            let assign39810_cond_e49257: f64 = if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign39810_cond_e49257 != 0.0
        } {
            assign39810_loop_guard += 1;
            assert!(assign39810_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
                s.store_mul_sub_ad_rhs(1857, 1899, A::add_scaled_product(s.ad_value(1885), 1.0, s.ad_value(1900), s.ad_value(1855), 1.0), s.ad_value(1853));
                s.store_mul(1841, 1899, 1900);
                s.store_sub(335, 1855, 1857);
            }
            s.b[1974] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1974] = if s.b[1974] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) {
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
            s.b[1975] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1975] = if s.b[1975] { 1.0 } else { 0.0 };
            s.b[1976] = (2.0 == 1.0);
            s.v[1976] = if s.b[1976] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) && s.b[1976]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1977] = (2.0 == 2.0);
            s.v[1977] = if s.b[1977] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) && (!s.b[1976])) && s.b[1977]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1978] = (2.0 == 4.0);
            s.v[1978] = if s.b[1978] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) && (!s.b[1976])) && (!s.b[1977])) && s.b[1978]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1979] = (2.0 == 8.0);
            s.v[1979] = if s.b[1979] { 1.0 } else { 0.0 };
            if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) && (!s.b[1976])) && (!s.b[1977])) && (!s.b[1978])) && s.b[1979]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign39810_body29_loop_guard: usize = 0;
            while {
                let assign39810_body29_cond_e49692: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign39810_body29_cond_e49692 != 0.0
            } {
                assign39810_body29_loop_guard += 1;
                assert!(assign39810_body29_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && s.b[1975]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) && (!s.b[1975])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_ad_value(334, A::div_scaled_product(s.ad_value(725), s.ad_value(726), 0.1, s.ad_value(770), 1.0));
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1974]) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1974])) {
                s.copy_ad(336, 335);
                s.store_scalar(334, 1.0);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
                s.store_sqrt_mul(1829, 1906, 336);
            }
            s.b[1980] = ((s.v[1829] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));
            s.v[1980] = if s.b[1980] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) {
                s.store_offset_sub(781, 1829, 965, 1e-8);
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
            s.b[1981] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1981] = if s.b[1981] { 1.0 } else { 0.0 };
            s.b[1982] = (2.0 == 1.0);
            s.v[1982] = if s.b[1982] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) && s.b[1982]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1983] = (2.0 == 2.0);
            s.v[1983] = if s.b[1983] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) && (!s.b[1982])) && s.b[1983]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1984] = (2.0 == 4.0);
            s.v[1984] = if s.b[1984] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) && (!s.b[1982])) && (!s.b[1983])) && s.b[1984]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1985] = (2.0 == 8.0);
            s.v[1985] = if s.b[1985] { 1.0 } else { 0.0 };
            if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) && (!s.b[1982])) && (!s.b[1983])) && (!s.b[1984])) && s.b[1985]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign39810_body65_loop_guard: usize = 0;
            while {
                let assign39810_body65_cond_e50273: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign39810_body65_cond_e50273 != 0.0
            } {
                assign39810_body65_loop_guard += 1;
                assert!(assign39810_body65_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && s.b[1981]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) && (!s.b[1981])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_ad_value(337, A::div_scaled_product(s.ad_value(725), s.ad_value(726), 1e-8, s.ad_value(770), 1.0));
                s.store_add_ad_lhs(1829, A::offset(s.ad_value(965), (-1e-8)), 780);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1980]) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1980])) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1980])) {
                s.store_scalar(337, 1.0);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
                s.store_mul(1866, 1829, 1903);
                s.store_mul_ad_product_lhs(1843, A::div_from_scalar(1.034943e-10, s.ad_value(1829)), s.ad_value(334), 337);
                s.store_mul_ad_product_lhs(1845, A::div_from_scalar((-1.034943e-10), s.ad_value(1829)), s.ad_value(334), 337);
            }
            s.b[1986] = (p.p49 == 0.0);
            s.v[1986] = if s.b[1986] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1986]) {
                s.store_add_ad_lhs(1837, A::div(A::add_scaled_product(A::add(s.ad_value(1901), s.ad_value(1836)), 1.0, s.ad_value(965), s.ad_value(1833), (-2.0)), s.ad_value(1906)), 1852);
                s.store_scalar(1838, 1.0);
                s.store_scalar(1839, 0.0);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1986])) {
                s.store_add_ad_rhs(1837, 1852, A::div(A::add_scaled_product(s.ad_value(1901), 1.0, s.ad_value(1829), A::sub_scaled_inputs(s.ad_value(1829), 1.0, s.ad_value(965), 2.0), 1.0), s.ad_value(1906)));
                s.store_scalar(1838, 1.0);
                s.store_mul_ad(1839, A::add_scaled_product(s.ad_value(334), 1.0, A::div(s.ad_value(965), s.ad_value(1829)), s.ad_value(334), (-1.0)), A::sub_from_scalar(1.0, s.ad_value(1841)));
            }
            s.b[1987] = ((s.v[1837] > (s.v[1850] - s.v[1848])) && (s.v[1848] >= 0.0));
            s.v[1987] = if s.b[1987] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) {
                s.store_ad_value(781, A::add_scaled_inputs3(s.ad_value(1837), 1.0, s.ad_value(1850), (-1.0), s.ad_value(1848), 1.0));
                s.store_square(722, 781);
                s.store_square(723, 1848);
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
            s.b[1988] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[1988] = if s.b[1988] { 1.0 } else { 0.0 };
            s.b[1989] = (4.0 == 1.0);
            s.v[1989] = if s.b[1989] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) && s.b[1989]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1990] = (4.0 == 2.0);
            s.v[1990] = if s.b[1990] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) && (!s.b[1989])) && s.b[1990]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1991] = (4.0 == 4.0);
            s.v[1991] = if s.b[1991] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) && (!s.b[1989])) && (!s.b[1990])) && s.b[1991]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1992] = (4.0 == 8.0);
            s.v[1992] = if s.b[1992] { 1.0 } else { 0.0 };
            if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) && (!s.b[1989])) && (!s.b[1990])) && (!s.b[1991])) && s.b[1992]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign39810_body114_loop_guard: usize = 0;
            while {
                let assign39810_body114_cond_e51078: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign39810_body114_cond_e51078 != 0.0
            } {
                assign39810_body114_loop_guard += 1;
                assert!(assign39810_body114_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && s.b[1988]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) && (!s.b[1988])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                    }
                });
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1848, 726);
                s.store_ad_value(334, A::div_scaled_product3(s.ad_value(1848), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0));
                s.store_ad_value(1837, A::add_scaled_inputs3(s.ad_value(1850), 1.0, s.ad_value(1848), (-1.0), s.ad_value(780), 1.0));
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1987]) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1987])) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1987])) {
                s.store_scalar(334, 1.0);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
                s.store_mul(1838, 1838, 334);
                s.store_mul(1839, 1839, 334);
                s.store_ad_value(335, A::add_scaled_inputs3(s.ad_value(1857), 1.0, s.ad_value(1885), (-1.0), s.ad_value(1853), 1.0));
            }
            s.b[1993] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1993] = if s.b[1993] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) {
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
            s.b[1994] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[1994] = if s.b[1994] { 1.0 } else { 0.0 };
            s.b[1995] = (2.0 == 1.0);
            s.v[1995] = if s.b[1995] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) && s.b[1995]) {
                s.store_scalar(720, 1.0);
            }
            s.b[1996] = (2.0 == 2.0);
            s.v[1996] = if s.b[1996] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) && (!s.b[1995])) && s.b[1996]) {
                s.store_scalar(720, 2.0);
            }
            s.b[1997] = (2.0 == 4.0);
            s.v[1997] = if s.b[1997] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) && (!s.b[1995])) && (!s.b[1996])) && s.b[1997]) {
                s.store_scalar(720, 3.0);
            }
            s.b[1998] = (2.0 == 8.0);
            s.v[1998] = if s.b[1998] { 1.0 } else { 0.0 };
            if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) && (!s.b[1995])) && (!s.b[1996])) && (!s.b[1997])) && s.b[1998]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign39810_body152_loop_guard: usize = 0;
            while {
                let assign39810_body152_cond_e51688: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign39810_body152_cond_e51688 != 0.0
            } {
                assign39810_body152_loop_guard += 1;
                assert!(assign39810_body152_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && s.b[1994]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) && (!s.b[1994])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.1);
                s.store_ad_value(337, A::div_scaled_product(s.ad_value(725), s.ad_value(726), 0.1, s.ad_value(770), 1.0));
                s.store_sub_from_scalar(336, 0.1, 780);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1993]) {
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1993])) {
                s.copy_ad(336, 335);
                s.store_scalar(337, 1.0);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
                s.store_sqrt_mul(1831, 1907, 336);
                s.store_mul_neg_lhs(1867, 1831, 1902);
                s.store_mul_div_from_scalar_lhs(1847, (-1.034943e-10), 1831, 337);
                s.store_mul_sub_rhs(335, 154, 1852, 1855);
                s.store_exp(336, 335);
            }
            s.b[1999] = (s.v[1852] >= s.v[1855]);
            s.v[1999] = if s.b[1999] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[1999]) {
                s.store_mul_scaled_ad_rhs(1861, 209, -1.0, A::sqrt(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15)));
                s.store_mul_ad(1896, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(1861), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
                s.store_neg(1898, 1896);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[1999])) {
                s.store_exp_ad(337, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1852), s.ad_value(1885))));
                s.store_exp_ad(338, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1855), s.ad_value(1885))));
                s.store_mul_sqrt_ad_rhs(1861, 209, A::offset(A::add_scaled_product(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1.0, s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_ad_value(339, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(1861), 1.0));
                s.store_mul_add_ad_rhs(1896, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1898, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] != 0.0)) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) {
                s.store_ad_value(1868, A::add_scaled_inputs3(A::add_scaled_product(s.ad_value(1861), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1852)), 1.0), 1.0, s.ad_value(1866), 1.0, s.ad_value(1867), 1.0));
                s.store_sub(1869, 1896, 185);
                s.store_add_ad_rhs(1870, 1898, A::add_scaled_product(A::add_scaled_product(s.ad_value(1843), 1.0, s.ad_value(1845), s.ad_value(1841), 1.0), 1.0, s.ad_value(1847), s.ad_value(1841), 1.0));
                s.store_sub(1871, 1855, 1837);
                s.store_neg(1872, 1838);
                s.store_sub_from_scalar(1873, 1.0, 1839);
                s.store_ad_value(1874, A::add_scaled_products(s.ad_value(1869), s.ad_value(1873), 1.0, s.ad_value(1870), s.ad_value(1872), (-1.0)));
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) {
                s.store_ad_value(1875, {
                    if (s.v[1874] > 0.0) {
                        A::div_from_scalar(1.0, A::offset(s.ad_value(1874), 1e-25))
                    } else {
                        A::div_from_scalar(1.0, A::offset(s.ad_value(1874), (-1e-25)))
                    }
                });
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) {
                s.copy_ad(1876, 1873);
                s.store_neg(1877, 1870);
                s.store_neg(1878, 1872);
                s.copy_ad(1879, 1869);
                s.store_mul_scaled_ad_rhs(1880, 1875, -1.0, A::add_scaled_products(s.ad_value(1876), s.ad_value(1868), 1.0, s.ad_value(1877), s.ad_value(1871), 1.0));
                s.store_mul_scaled_ad_rhs(1881, 1875, -1.0, A::add_scaled_products(s.ad_value(1878), s.ad_value(1868), 1.0, s.ad_value(1879), s.ad_value(1871), 1.0));
                s.store_abs(335, 1880);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) {
                s.store_ad_value(335, {
                    if (s.v[335] < ((s.v[1881]) as f64).abs()) {
                        A::abs(s.ad_value(1881))
                    } else {
                        s.ad_value(335)
                    }
                });
            }
            s.b[2000] = (s.v[335] > 0.1);
            s.v[2000] = if s.b[2000] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) && s.b[2000]) {
                s.store_mul_div_from_scalar_rhs(1880, 1880, 0.1, 335);
                s.store_mul_div_from_scalar_rhs(1881, 1881, 0.1, 335);
            }
            s.b[2001] = (s.v[335] < 1e-12);
            s.v[2001] = if s.b[2001] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) && s.b[2001]) {
                s.store_scalar(79, 1.0);
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (s.v[79] == 0.0)) {
                s.store_add(1852, 1852, 1880);
                s.store_add(1855, 1855, 1881);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
                s.store_offset(97, 97, 1.0);
            }
        }

    }

    pub(super) fn stamp_reactive_block_35(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            s.store_mul_sub_rhs(335, 154, 1852, 1855);
            s.store_exp(336, 335);
        }

        s.b[2003] = (s.v[1852] >= s.v[1855]);
        s.v[2003] = if s.b[2003] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2003]) {
            s.copy_ad(1891, 1861);
            s.store_scalar(1894, 0.0);
            s.store_scalar(1863, 0.0);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[2003])) {
            s.store_scalar(1891, 0.0);
            s.store_mul_sqrt_ad_rhs(1894, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
        }

        s.b[2004] = (s.v[1835] > s.v[965]);
        s.v[2004] = if s.b[2004] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[2003])) && s.b[2004]) {
            s.store_scalar(1863, 0.0);
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[2003])) && (!s.b[2004])) {
            s.store_mul_sqrt_ad_rhs(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1852), s.ad_value(1885)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1855), s.ad_value(1885)))))), s.ad_value(335)));
            s.store_ad_value(1863, A::add_scaled_product(s.ad_value(337), 1.0, s.ad_value(209), A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0)));
        }

        s.b[2005] = (((s.v[1852] - s.v[1850]) < s.v[1909]) && (s.v[1909] >= 0.0));
        s.v[2005] = if s.b[2005] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) {
            s.store_ad_value(781, A::add_scaled_inputs3(s.ad_value(1909), 1.0, s.ad_value(1852), -1.0, s.ad_value(1850), 1.0));
            s.store_square(722, 781);
            s.store_square(723, 1909);
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

        s.b[2006] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[2006] = if s.b[2006] { 1.0 } else { 0.0 };

        s.b[2007] = (4.0 == 1.0);
        s.v[2007] = if s.b[2007] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) && s.b[2007]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2008] = (4.0 == 2.0);
        s.v[2008] = if s.b[2008] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) && (!s.b[2007])) && s.b[2008]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2009] = (4.0 == 4.0);
        s.v[2009] = if s.b[2009] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) && (!s.b[2007])) && (!s.b[2008])) && s.b[2009]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2010] = (4.0 == 8.0);
        s.v[2010] = if s.b[2010] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) && (!s.b[2007])) && (!s.b[2008])) && (!s.b[2009])) && s.b[2010]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign40250_loop_guard: usize = 0;
        while {
            let assign40250_cond_e53259: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign40250_cond_e53259 != 0.0
        } {
            assign40250_loop_guard += 1;
            assert!(assign40250_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && s.b[2006]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) && (!s.b[2006])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                }
            });
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1909, 726);
            s.store_ad_value(334, A::div_scaled_product3(s.ad_value(1909), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0));
            s.store_sub(336, 1909, 780);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && s.b[2005]) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) && (!s.b[2005])) {
            s.store_sub(336, 1852, 1850);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[1973])) {
            s.store_offset_ad(338, A::add_scaled_product(A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0)), 1e-15);
            s.store_mul_scaled_ad_rhs(1888, 209, -1.0, A::sqrt(s.ad_value(338)));
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.copy_ad(87, 1851);
            s.copy_ad(91, 1852);
            s.store_sub(94, 1852, 1851);
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 1.0 / ((p.p263 * 0.1))));
            s.store_offset_mul_ad(782, s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0)), 1.0);
            s.store_offset_mul_ad(783, s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0))), (1.0 / 8.0))), (1.0 / 3.0)), (1.0 / 2.0));
            s.store_div_from_scalar(110, (p.p263 * 0.1), 782);
            s.store_ad_value(336, A::div_scaled_inputs(s.ad_value(783), (-2.0), A::square(s.ad_value(782)), 1.0));
        }

        s.b[2011] = ((s.v[110] < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2011] = if s.b[2011] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) {
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

        s.b[2012] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2012] = if s.b[2012] { 1.0 } else { 0.0 };

        s.b[2013] = (2.0 == 1.0);
        s.v[2013] = if s.b[2013] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) && s.b[2013]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2014] = (2.0 == 2.0);
        s.v[2014] = if s.b[2014] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) && (!s.b[2013])) && s.b[2014]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2015] = (2.0 == 4.0);
        s.v[2015] = if s.b[2015] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) && (!s.b[2013])) && (!s.b[2014])) && s.b[2015]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2016] = (2.0 == 8.0);
        s.v[2016] = if s.b[2016] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) && (!s.b[2013])) && (!s.b[2014])) && (!s.b[2015])) && s.b[2016]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign40710_loop_guard: usize = 0;
        while {
            let assign40710_cond_e53989: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign40710_cond_e53989 != 0.0
        } {
            assign40710_loop_guard += 1;
            assert!(assign40710_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && s.b[2012]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) && (!s.b[2012])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));
            s.store_ad_value(334, A::div_scaled_product(s.ad_value(725), s.ad_value(726), (10.0 * 2.220446049250313e-16), s.ad_value(770), 1.0));
            s.store_sub_from_scalar(110, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2011]) {
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2011])) {
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2011])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_add(109, 87, 110);
        }

        s.b[2017] = (((s.v[109] - s.v[1849]) < s.v[1909]) && (s.v[1909] >= 0.0));
        s.v[2017] = if s.b[2017] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) {
            s.store_ad_value(781, A::add_scaled_inputs3(s.ad_value(1909), 1.0, s.ad_value(109), -1.0, s.ad_value(1849), 1.0));
            s.store_square(722, 781);
            s.store_square(723, 1909);
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

        s.b[2018] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[2018] = if s.b[2018] { 1.0 } else { 0.0 };

        s.b[2019] = (4.0 == 1.0);
        s.v[2019] = if s.b[2019] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) && s.b[2019]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2020] = (4.0 == 2.0);
        s.v[2020] = if s.b[2020] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) && (!s.b[2019])) && s.b[2020]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2021] = (4.0 == 4.0);
        s.v[2021] = if s.b[2021] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) && (!s.b[2019])) && (!s.b[2020])) && s.b[2021]) {
            s.store_scalar(720, 3.0);
        }

    }

    pub(super) fn stamp_reactive_block_36(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2022] = (4.0 == 8.0);
        s.v[2022] = if s.b[2022] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) && (!s.b[2019])) && (!s.b[2020])) && (!s.b[2021])) && s.b[2022]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign41110_loop_guard: usize = 0;
        while {
            let assign41110_cond_e54537: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign41110_cond_e54537 != 0.0
        } {
            assign41110_loop_guard += 1;
            assert!(assign41110_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && s.b[2018]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) && (!s.b[2018])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                }
            });
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1909, 726);
            s.store_ad_value(334, A::div_scaled_product3(s.ad_value(1909), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0));
            s.store_sub(336, 1909, 780);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2017]) {
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2017])) {
            s.store_sub(336, 109, 1849);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_offset_ad(338, A::add_scaled_product(A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0)), 1e-15);
            s.store_mul_scaled_ad_rhs(1889, 209, -1.0, A::sqrt(s.ad_value(338)));
        }

        s.b[2028] = (s.v[1834] > s.v[965]);
        s.v[2028] = if s.b[2028] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2028]) {
            s.copy_ad(981, 1832);
        }

        s.b[2029] = ((s.v[87] > (-0.1)) && (0.1 >= 0.0));
        s.v[2029] = if s.b[2029] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) {
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

        let mut assign41350_loop_guard: usize = 0;
        while {
            let assign41350_cond_e54907: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && (s.v[719] < s.v[1912])) { 1.0 } else { 0.0 };
            assign41350_cond_e54907 != 0.0
        } {
            assign41350_loop_guard += 1;
            assert!(assign41350_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) {
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) {
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2030] = ((((s.v[1912] == 1.0) || (s.v[1912] == 2.0)) || (s.v[1912] == 4.0)) || (s.v[1912] == 8.0));
        s.v[2030] = if s.b[2030] { 1.0 } else { 0.0 };

        s.b[2031] = (s.v[1912] == 1.0);
        s.v[2031] = if s.b[2031] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) && s.b[2031]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2032] = (s.v[1912] == 2.0);
        s.v[2032] = if s.b[2032] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) && (!s.b[2031])) && s.b[2032]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2033] = (s.v[1912] == 4.0);
        s.v[2033] = if s.b[2033] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) && (!s.b[2031])) && (!s.b[2032])) && s.b[2033]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2034] = (s.v[1912] == 8.0);
        s.v[2034] = if s.b[2034] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) && (!s.b[2031])) && (!s.b[2032])) && (!s.b[2033])) && s.b[2034]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign41480_loop_guard: usize = 0;
        while {
            let assign41480_cond_e55135: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign41480_cond_e55135 != 0.0
        } {
            assign41480_loop_guard += 1;
            assert!(assign41480_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && s.b[2030]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) && (!s.b[2030])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(1912), 2.0)))
                }
            });
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_ad_value(334, A::div_scaled_product(s.ad_value(725), s.ad_value(726), 0.1, s.ad_value(770), 1.0));
            s.store_offset(983, 780, (-0.1));
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2029]) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2029])) {
            s.copy_ad(983, 87);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {
            s.store_ad_value(1914, A::add_scaled_inputs3(s.ad_value(791), 1.0, s.ad_value(85), (-1.0), A::sub_from_scalar((s.v[462] - p.p392), s.ad_value(1910)), -1.0));
            s.store_sub(1913, 791, 1914);
        }

        s.b[2035] = ((s.v[1913] > (-s.v[1911])) && (s.v[1911] >= 0.0));
        s.v[2035] = if s.b[2035] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {
            s.store_add(781, 1913, 1911);
            s.store_square(722, 781);
            s.store_square(723, 1911);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_scalar(719, 0.0);
        }

        let mut assign41700_loop_guard: usize = 0;
        while {
            let assign41700_cond_e55520: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && (s.v[719] < s.v[1912])) { 1.0 } else { 0.0 };
            assign41700_cond_e55520 != 0.0
        } {
            assign41700_loop_guard += 1;
            assert!(assign41700_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {
                s.store_mul(724, 724, 722);
                s.store_mul(725, 725, 723);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2036] = ((((s.v[1912] == 1.0) || (s.v[1912] == 2.0)) || (s.v[1912] == 4.0)) || (s.v[1912] == 8.0));
        s.v[2036] = if s.b[2036] { 1.0 } else { 0.0 };

        s.b[2037] = (s.v[1912] == 1.0);
        s.v[2037] = if s.b[2037] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) && s.b[2037]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2038] = (s.v[1912] == 2.0);
        s.v[2038] = if s.b[2038] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) && (!s.b[2037])) && s.b[2038]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2039] = (s.v[1912] == 4.0);
        s.v[2039] = if s.b[2039] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) && (!s.b[2037])) && (!s.b[2038])) && s.b[2039]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2040] = (s.v[1912] == 8.0);
        s.v[2040] = if s.b[2040] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) && (!s.b[2037])) && (!s.b[2038])) && (!s.b[2039])) && s.b[2040]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign41830_loop_guard: usize = 0;
        while {
            let assign41830_cond_e55748: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign41830_cond_e55748 != 0.0
        } {
            assign41830_loop_guard += 1;
            assert!(assign41830_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && s.b[2036]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) && (!s.b[2036])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(1912), 2.0)))
                }
            });
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1911, 726);
            s.store_ad_value(334, A::div_scaled_product3(s.ad_value(1911), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0));
            s.store_add_scaled_inputs(1913, 1911, -1.0, 780, 1.0);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2035]) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2035])) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2035])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {
            s.store_scalar(79, 0.0);
            s.store_scalar(97, 1.0);
        }

        let mut assign41940_loop_guard: usize = 0;
        while {
            let assign41940_cond_e55964: f64 = if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign41940_cond_e55964 != 0.0
        } {
            assign41940_loop_guard += 1;
            assert!(assign41940_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {
                s.store_mul(335, 154, 983);
                s.store_exp(336, 335);
            }
            s.b[2041] = (s.v[983] >= 0.0);
            s.v[2041] = if s.b[2041] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2041]) {
                s.store_mul_scaled_ad_rhs(2026, 209, -1.0, A::sqrt(A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15)));
                s.store_mul_ad(2027, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(2026), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2041])) {
                s.store_exp_ad(337, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(983), s.ad_value(1885))));
                s.store_exp_ad(338, A::mul_scaled_lhs(s.ad_value(154), 1.0, s.ad_value(1885)));
                s.store_mul_sqrt_ad_rhs(2026, 209, A::offset(A::add_scaled_product(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1.0, s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_ad_value(339, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(2026), 1.0));
                s.store_mul_add_ad_rhs(2027, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] != 0.0)) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] == 0.0)) {
                s.store_ad_value(1868, A::add_scaled_product(s.ad_value(2026), 1.0, s.ad_value(185), A::sub(s.ad_value(1913), s.ad_value(983)), 1.0));
                s.store_sub(1869, 2027, 185);
                s.store_scaled_div(1880, 1868, 1869, -1.0);
            }
            s.b[2042] = (((s.v[1880]) as f64).abs() < (1e-10 * 100.0));
            s.v[2042] = if s.b[2042] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] == 0.0)) && s.b[2042]) {
                s.store_scalar(79, 1.0);
            }
            s.b[2043] = (s.v[1880] > 0.1);
            s.v[2043] = if s.b[2043] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] == 0.0)) && (!s.b[2042])) && s.b[2043]) {
                s.store_scalar(1880, 0.1);
            }
            s.b[2044] = (s.v[1880] < (-0.1));
            s.v[2044] = if s.b[2044] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] == 0.0)) && (!s.b[2042])) && (!s.b[2043])) && s.b[2044]) {
                s.store_scalar(1880, (-0.1));
            }
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (s.v[79] == 0.0)) {
                s.store_add(983, 983, 1880);
            }
            if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {
            s.store_neg(983, 983);
            s.store_mul3_affine_lhs(2024, 1903, 1832, (0.5 * 9662367879.197212), 0.0, 1832);
            s.store_scaled_sqrt_ad(334, A::mul_scaled_lhs(s.ad_value(154), 2.0, s.ad_value(2024)), p.p394);
            s.store_scaled_add_ad(335, A::exp(s.ad_value(334)), A::exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ad_lhs(2025, A::ln(s.ad_value(335)), 2024);
            s.store_mul(332, 2025, 983);
            s.store_exp_ad(334, A::mul_scaled_lhs(s.ad_value(2025), -1.0, s.ad_value(2024)));
        }

        s.b[2046] = (((s.v[332]) as f64).abs() > 1e-8);
        s.v[2046] = if s.b[2046] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2046]) {
            s.store_mul_exp_lhs(335, 332, 334);
        }

    }

    pub(super) fn stamp_reactive_block_37(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2046]) {
            s.store_sub(336, 335, 334);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2046])) {
            s.store_mul_offset_lhs(335, 332, 1.0, 334);
            s.store_mul_ad_product_lhs(336, s.ad_value(332), A::scale_offset(s.ad_value(332), 0.5, 1.0), 334);
        }

        s.b[2047] = (((s.v[336]) as f64).abs() > 1e-8);
        s.v[2047] = if s.b[2047] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2047]) {
            s.store_div_ad_lhs(2023, A::ln(A::offset(s.ad_value(336), 1.0)), 2025);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2047])) {
            s.store_div(2023, 336, 2025);
        }

        s.b[2048] = ((((2.0 * 1.034943e-10) * (s.v[983] - s.v[2023])) / s.v[1903]) <= 0.0);
        s.v[2048] = if s.b[2048] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && s.b[2048]) {
            s.store_scalar(981, 0.0);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) && (!s.b[2048])) {
            s.store_sqrt_div_ad(981, A::sub_scaled_inputs(s.ad_value(983), (2.0 * 1.034943e-10), s.ad_value(2023), (2.0 * 1.034943e-10)), s.ad_value(1903));
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2028])) {
            s.store_ad_value(981, {
                if (s.v[981] > s.v[1832]) {
                    s.ad_value(1832)
                } else {
                    s.ad_value(981)
                }
            });
        }

        s.b[2049] = (s.v[981] < s.v[1832]);
        s.v[2049] = if s.b[2049] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2049]) {
            s.store_sub(990, 1832, 981);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2049])) {
            s.store_scalar(990, 0.0);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_neg_ad(1892, A::add(s.ad_value(1887), s.ad_value(1888)));
        }

        s.b[2050] = (s.v[94] < 0.0);
        s.v[2050] = if s.b[2050] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2050]) {
            s.store_scalar(94, 0.0);
            s.copy_ad(1852, 1851);
            s.store_scalar(248, 0.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2050])) {
            s.store_mul3_affine_lhs(248, 154, 1892, 1.0 / (2.0), 0.0, 94);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2050])) {
            s.store_ad_value(248, {
                if (s.v[248] < 0.0) {
                    A::constant(0.0)
                } else {
                    s.ad_value(248)
                }
            });
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_neg(238, 1889);
            s.copy_ad(170, 162);
            s.store_scalar(336, (s.v[626] / 100.0));
            s.copy_ad(334, 682);
            s.store_offset_sqrt_ad(980, A::offset(A::square(s.ad_value(94)), p.p262), (-((p.p262) as f64).sqrt()));
            s.store_offset_mul(338, 980, 334, 1.0);
            s.store_mul(339, 336, 238);
            s.store_div(337, 339, 338);
            s.copy_ad(251, 337);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_ad_value(339, {
                if (s.v[251] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(251), (p.p160 - 1.0))
                }
            });
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_mul(342, 339, 251);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_ad_value(341, {
                if (s.v[251] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(251), A::offset(s.ad_value(624), (-1.0)))
                }
            });
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
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
            s.store_scaled_div(336, 257, 254, 0.2);
            s.store_sqrt_square_sum(255, 333, 336);
            s.store_div_from_scalar(338, 1.0, 255);
            s.store_mul(256, 254, 255);
            s.store_div(335, 256, 257);
        }

        s.b[2051] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2051] = if s.b[2051] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2051]) {
            s.store_scalar(337, 1.0);
        }

        s.b[2052] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2052] = if s.b[2052] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2051])) && s.b[2052]) {
            s.copy_ad(337, 335);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2051])) && (!s.b[2052])) {
            s.store_ad_value(337, {
                if (s.v[335] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(335), (p.p178 - 1.0))
                }
            });
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_mul(336, 335, 337);
            s.store_offset(338, 336, 1.0);
        }

        s.b[2053] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2053] = if s.b[2053] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2053]) {
            s.store_div_from_scalar(339, 1.0, 338);
        }

        s.b[2054] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));
        s.v[2054] = if s.b[2054] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2053])) && s.b[2054]) {
            s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2053])) && (!s.b[2054])) {
            s.store_ad_value(340, {
                if (s.v[338] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(338), (((-1.0) / p.p178) - 1.0))
                }
            });
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2053])) && (!s.b[2054])) {
            s.store_mul(339, 338, 340);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_mul(253, 254, 339);
            s.copy_ad(984, 253);
            s.copy_ad(1884, 255);
            s.copy_ad(989, 349);
        }

        s.b[2055] = (s.v[349] > 1e-6);
        s.v[2055] = if s.b[2055] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {
            s.store_scaled_add(344, 1885, 155, p.p396);
            s.store_offset_mul_ad(338, s.ad_value(1905), A::sub(s.ad_value(85), s.ad_value(344)), 1.0);
            s.store_offset(339, 1905, 1.0);
        }

        s.b[2056] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));
        s.v[2056] = if s.b[2056] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) {
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

        s.b[2057] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2057] = if s.b[2057] { 1.0 } else { 0.0 };

        s.b[2058] = (2.0 == 1.0);
        s.v[2058] = if s.b[2058] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) && s.b[2058]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2059] = (2.0 == 2.0);
        s.v[2059] = if s.b[2059] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) && (!s.b[2058])) && s.b[2059]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2060] = (2.0 == 4.0);
        s.v[2060] = if s.b[2060] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) && (!s.b[2058])) && (!s.b[2059])) && s.b[2060]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2061] = (2.0 == 8.0);
        s.v[2061] = if s.b[2061] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) && (!s.b[2058])) && (!s.b[2059])) && (!s.b[2060])) && s.b[2061]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign42990_loop_guard: usize = 0;
        while {
            let assign42990_cond_e57749: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign42990_cond_e57749 != 0.0
        } {
            assign42990_loop_guard += 1;
            assert!(assign42990_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && s.b[2057]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) && (!s.b[2057])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 339, 726);
            s.store_ad_value(334, A::div_scaled_product3(s.ad_value(339), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0));
            s.store_sub(338, 339, 780);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2056]) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && (!s.b[2056])) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && (!s.b[2056])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {
            s.store_sqrt(337, 338);
            s.store_ad_value(344, A::add_scaled_product(s.ad_value(85), 1.0, s.ad_value(1904), A::sub_from_scalar(1.0, s.ad_value(337)), 1.0));
        }

        s.b[2062] = ((s.v[344] < (s.v[972] + s.v[1908])) && (s.v[1908] >= 0.0));
        s.v[2062] = if s.b[2062] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) {
            s.store_ad_value(781, A::add_scaled_inputs3(s.ad_value(972), 1.0, s.ad_value(1908), 1.0, s.ad_value(344), -1.0));
            s.store_square(722, 781);
            s.store_square(723, 1908);
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
        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) {
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2063] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2063] = if s.b[2063] { 1.0 } else { 0.0 };

        s.b[2064] = (2.0 == 1.0);
        s.v[2064] = if s.b[2064] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) && s.b[2064]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2065] = (2.0 == 2.0);
        s.v[2065] = if s.b[2065] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) && (!s.b[2064])) && s.b[2065]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2066] = (2.0 == 4.0);
        s.v[2066] = if s.b[2066] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) && (!s.b[2064])) && (!s.b[2065])) && s.b[2066]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2067] = (2.0 == 8.0);
        s.v[2067] = if s.b[2067] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) && (!s.b[2064])) && (!s.b[2065])) && (!s.b[2066])) && s.b[2067]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign43360_loop_guard: usize = 0;
        while {
            let assign43360_cond_e58313: f64 = if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign43360_cond_e58313 != 0.0
        } {
            assign43360_loop_guard += 1;
            assert!(assign43360_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && s.b[2063]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) && (!s.b[2063])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1908, 726);
            s.store_ad_value(334, A::div_scaled_product3(s.ad_value(1908), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0));
            s.store_ad_value(344, A::add_scaled_inputs3(s.ad_value(972), 1.0, s.ad_value(1908), 1.0, s.ad_value(780), -1.0));
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && s.b[2062]) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && (!s.b[2062])) {
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) && (!s.b[2062])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {
            s.store_div(335, 989, 344);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {
            s.store_ad_value(336, {
                if (s.v[335] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(335), (p.p383 - 1.0))
                }
            });
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {
            s.store_offset_mul(337, 336, 335, 1.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {
            s.store_ad_value(338, {
                if (s.v[337] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(337), ((1.0 / p.p383) - 1.0))
                }
            });
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2055]) {
            s.store_mul(340, 338, 337);
            s.store_div(989, 989, 340);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_sub(335, 791, 1885);
        }

        s.b[2068] = ((s.v[335] < 1.0) && (1.0 >= 0.0));
        s.v[2068] = if s.b[2068] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) {
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

        s.b[2069] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2069] = if s.b[2069] { 1.0 } else { 0.0 };

        s.b[2070] = (2.0 == 1.0);
        s.v[2070] = if s.b[2070] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) && s.b[2070]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2071] = (2.0 == 2.0);
        s.v[2071] = if s.b[2071] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) && (!s.b[2070])) && s.b[2071]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2072] = (2.0 == 4.0);
        s.v[2072] = if s.b[2072] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) && (!s.b[2070])) && (!s.b[2071])) && s.b[2072]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2073] = (2.0 == 8.0);
        s.v[2073] = if s.b[2073] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) && (!s.b[2070])) && (!s.b[2071])) && (!s.b[2072])) && s.b[2073]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign43780_loop_guard: usize = 0;
        while {
            let assign43780_cond_e58913: f64 = if ((((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign43780_cond_e58913 != 0.0
        } {
            assign43780_loop_guard += 1;
            assert!(assign43780_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && s.b[2069]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) && (!s.b[2069])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 1.0);
            s.store_ad_value(334, A::div_scaled_product(s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0));
            s.store_sub_from_scalar(335, 1.0, 780);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2068]) {
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2068])) {
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2068])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_div(251, 335, 965);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_ad_value(339, {
                if (s.v[251] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(251), (p.p353 - 1.0))
                }
            });
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_mul(342, 339, 251);
            s.store_offset(336, 966, 1e-25);
            s.store_add_ad(335, A::div_from_scalar(1.0, s.ad_value(336)), A::div(s.ad_value(342), s.ad_value(970)));
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_div(1883, 989, 170);
            s.store_powf(781, 989, 2.0);
            s.store_scalar(782, ((0.1) as f64).powf(2.0));
            s.store_sub_ad(335, A::powf(A::add(s.ad_value(781), s.ad_value(782)), (1.0 / 2.0)), A::powf(s.ad_value(782), (1.0 / 2.0)));
            s.store_div(335, 335, 170);
            s.store_ad_value(335, A::div_scaled_product(s.ad_value(254), s.ad_value(335), 1.0, s.ad_value(973), 1.0));
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_ad_value(336, {
                if (s.v[335] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(335), p.p378)
                }
            });
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_offset(337, 336, 1.0);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_ad_value(338, {
                if (s.v[337] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(337), (1.0 / p.p378))
                }
            });
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_div(985, 254, 338);
            s.store_scaled_mul(991, 990, 964, (-1.6021918e-19));
            s.store_mul3_affine_lhs(987, 991, 985, (-s.v[632]), 0.0, 1883);
            s.store_scaled_div(115, 155, 170, s.v[632]);
            s.store_mul3_lhs(986, 115, 248, 984);
            s.store_add(135, 986, 987);
            s.copy_ad(790, 349);
        }

        s.b[2074] = (p.p283 != 0.0);
        s.v[2074] = if s.b[2074] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2074]) {
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 100.0));
            s.store_offset_mul_ad(782, s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0))), (1.0 / 24.0))), (1.0 / 6.0))), (1.0 / 2.0)), 1.0);
            s.store_offset_mul_ad(783, s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::offset(A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0))), (1.0 / 8.0))), (1.0 / 3.0)), (1.0 / 2.0));
            s.store_div_from_scalar(340, 0.01, 782);
            s.store_ad_value(336, A::div_scaled_inputs(s.ad_value(783), (-2.0), A::square(s.ad_value(782)), 1.0));
            s.store_sub_from_scalar_ad(335, 1.1, A::add(s.ad_value(1851), s.ad_value(340)));
            s.store_sqrt_square_offset(782, 335, ((4.0 * 0.05) * 0.05));
            s.store_offset_scaled_div(334, 335, 782, 0.5, 0.5);
            s.store_scaled_add(336, 335, 782, 0.5);
        }

        s.b[2075] = (s.v[336] < 0.0);
        s.v[2075] = if s.b[2075] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2074]) && s.b[2075]) {
            s.store_scalar(336, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2074]) {
            s.store_offset(336, 336, 1e-25);
            s.store_scale(334, 154, s.v[672]);
            s.store_mul(337, 185, 334);
            s.store_powf(334, 336, p.p284);
            s.store_mul(343, 337, 334);
            s.store_offset_scaled(338, 1437, p.p285, 1.0);
            s.store_scalar(334, s.v[673]);
            s.store_ad_value(339, A::add_scaled_inputs3(s.ad_value(1851), 1.0, s.ad_value(340), 1.0, s.ad_value(1436), -1.0));
            s.store_add_ad_rhs(338, 338, A::mul3(s.ad_value(1437), s.ad_value(334), s.ad_value(339)));
            s.store_mul(340, 343, 338);
            s.copy_ad(343, 340);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2074])) {
            s.store_scalar(343, 0.0);
        }

        s.b[2076] = (p.p287 != 0.0);
        s.v[2076] = if s.b[2076] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2076]) {
            s.store_scale(335, 154, s.v[674]);
            s.store_mul(336, 185, 335);
            s.store_mul(342, 336, 1437);
        }

    }

    pub(super) fn stamp_reactive_block_39(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2076])) {
            s.store_scalar(342, 0.0);
        }

        s.b[2077] = ((s.v[343] + s.v[342]) > 0.0);
        s.v[2077] = if s.b[2077] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2077]) {
            s.store_mul_add_rhs(249, 94, 343, 342);
            s.store_add_ad_rhs(135, 135, A::mul3(s.ad_value(115), s.ad_value(249), s.ad_value(253)));
        }

        s.b[2078] = ((s.v[74] == 2.0) || (s.v[74] == 3.0));
        s.v[2078] = if s.b[2078] { 1.0 } else { 0.0 };

        s.b[2079] = (p.p296 > 0.0);
        s.v[2079] = if s.b[2079] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && s.b[2079]) {
            s.copy_ad(338, 647);
            s.store_scaled_offset(335, 796, (-p.p300), s.v[533]);
            s.store_offset_sub(781, 335, 338, (-(0.01 * 0.01)));
            s.store_scale(782, 338, (4.0 * (0.01 * 0.01)));
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && s.b[2079]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && s.b[2079]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_ad_value(336, A::add_scaled_inputs3(s.ad_value(338), 1.0, s.ad_value(781), 0.5, s.ad_value(782), 0.5));
            s.store_scale(337, 338, (p.p296 + 1.0));
            s.store_offset_sub(781, 337, 336, (-(0.01 * 0.01)));
            s.store_scale(782, 337, (4.0 * (0.01 * 0.01)));
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && s.b[2079]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && s.b[2079]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_ad_value(341, A::add_scaled_inputs3(s.ad_value(337), 1.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5)));
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && (!s.b[2079])) {
            s.copy_ad(341, 647);
        }

        s.b[2080] = (s.v[793] >= 0.0);
        s.v[2080] = if s.b[2080] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && s.b[2080]) {
            s.copy_ad(369, 793);
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && (!s.b[2080])) {
            s.store_scalar(369, 0.0);
        }

        s.b[2081] = (s.v[369] < (20.0 * 1e-12));
        s.v[2081] = if s.b[2081] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && s.b[2081]) {
            s.store_scalar(378, (((((20.0 + 1.0)) as f64).powf((p.p297 - 1.0)) * ((20.0 + 1.0) - ((0.5 * p.p297) * 20.0))) * ((1e-12) as f64).powf(p.p297)));
            s.store_scalar(379, ((((0.5 * p.p297) * (((20.0 + 1.0)) as f64).powf((p.p297 - 1.0))) / 20.0) * ((1e-12) as f64).powf((p.p297 - 2.0))));
            s.store_add_ad_rhs(335, 378, A::mul3(s.ad_value(379), s.ad_value(369), s.ad_value(369)));
        }

        if (((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) && (!s.b[2081])) {
            s.store_powf_ad(335, A::offset(s.ad_value(369), 1e-12), p.p297);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2078]) {
            s.store_powf_ad(343, A::offset(s.ad_value(369), 1e-12), p.p299);
            s.store_ad_value(368, A::add_scaled_products(s.ad_value(341), s.ad_value(335), 1.0 / (s.v[632]), s.ad_value(797), s.ad_value(343), (s.v[531] * 1.0 / (s.v[632]))));
            s.store_mul(334, 368, 135);
            s.store_offset(335, 790, 1e-12);
            s.store_div_from_scalar(336, 1.0, 335);
            s.store_offset_mul(337, 334, 336, 1.0);
            s.store_div_from_scalar(338, 1.0, 337);
            s.store_mul(134, 135, 338);
        }

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && (!s.b[2078])) {
            s.copy_ad(134, 135);
            s.store_scalar(368, 0.0);
        }

        if (s.b[1441] && (s.b[1443] && (!s.b[1442]))) {
            s.store_scaled_add_ad_lhs(131, A::add_scaled_inputs3(s.ad_value(1862), 1.0, s.ad_value(1863), 1.0, s.ad_value(1865), 1.0), 1867, (-0.5));
            s.store_scaled_add_ad_lhs(133, A::add_scaled_inputs3(A::add_scaled_inputs3(s.ad_value(1890), 1.0, s.ad_value(1891), 1.0, s.ad_value(1893), 1.0), 1.0, s.ad_value(1894), 1.0, s.ad_value(1864), 1.0), 1866, (-0.5));
            s.store_scalar(247, 0.5);
            s.store_scaled_add(978, 1890, 1891, (-0.5));
            s.store_neg(238, 1890);
            s.copy_ad(255, 1884);
        }

        s.b[2082] = ((s.v[238] < 1e-25) || (s.v[133] < 1e-25));
        s.v[2082] = if s.b[2082] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1443] && (!s.b[1442]))) && s.b[2082]) {
            s.store_scalar(78, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.copy_ad(2089, 960);
            s.store_scale(2131, 964, 1.6021918e-19);
            s.store_scale(2112, 964, (1.6021918e-19 * 1.034943e-10));
            s.store_div_from_scalar(2134, (2.0 * 1.034943e-10), 2131);
            s.store_div(2128, 964, 622);
            s.store_div_from_scalar_offset_input(2127, 1.0, 2128, 1.0);
            s.store_div_ad_rhs(2132, 2112, A::square(s.ad_value(185)));
            s.store_div_from_scalar(2133, 2.0, 2132);
            s.store_scalar(2141, 2.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_scalar(508, (if param_given[227] { s.v[508] } else { (5000000000.0 / (p.p343 * p.p340)) }));
        }

        s.b[2170] = ((s.v[508] < (2.0 + 0.1)) && (0.1 >= 0.0));
        s.v[2170] = if s.b[2170] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) {
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

        s.b[2171] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2171] = if s.b[2171] { 1.0 } else { 0.0 };

        s.b[2172] = (2.0 == 1.0);
        s.v[2172] = if s.b[2172] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) && s.b[2172]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2173] = (2.0 == 2.0);
        s.v[2173] = if s.b[2173] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) && (!s.b[2172])) && s.b[2173]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2174] = (2.0 == 4.0);
        s.v[2174] = if s.b[2174] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) && (!s.b[2172])) && (!s.b[2173])) && s.b[2174]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2175] = (2.0 == 8.0);
        s.v[2175] = if s.b[2175] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) && (!s.b[2172])) && (!s.b[2173])) && (!s.b[2174])) && s.b[2175]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign45240_loop_guard: usize = 0;
        while {
            let assign45240_cond_e61039: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign45240_cond_e61039 != 0.0
        } {
            assign45240_loop_guard += 1;
            assert!(assign45240_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && s.b[2171]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) && (!s.b[2171])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_ad_value(334, A::div_scaled_product(s.ad_value(725), s.ad_value(726), 0.1, s.ad_value(770), 1.0));
            s.store_sub_from_scalar(508, (2.0 + 0.1), 780);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2170]) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2170])) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2170])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_scalar(2087, 0.0);
            s.store_scalar(2088, 0.0);
            s.store_scalar(2096, 0.0);
            s.store_scalar(2097, 0.0);
            s.store_scalar(2169, 0.0);
            s.store_scalar(2144, 0.0);
            s.copy_ad(2115, 1433);
            s.store_ad_value(85, A::add_scaled_inputs3(A::offset(s.ad_value(791), (-s.v[160])), 1.0, s.ad_value(120), 1.0, s.ad_value(182), (-1.0)));
            s.store_offset_sub_from_scalar_ad(781, 0.3, s.ad_value(85), (-0.01));
            s.store_scalar(782, ((4.0 * 0.3) * 0.01));
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_sub_from_scalar_ad(2094, 0.3, A::add_scaled_inputs(s.ad_value(781), 0.5, s.ad_value(782), 0.5));
            s.store_offset_ad(781, A::add_scaled_inputs3(s.ad_value(2094), 1.0, s.ad_value(2115), -1.0, s.ad_value(2089), 1.0), (-0.01));
            s.store_scaled_sub(782, 2115, 2089, (4.0 * 0.01));
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_ad(2094, A::sub(s.ad_value(2115), s.ad_value(2089)), A::add_scaled_inputs(s.ad_value(781), 0.5, s.ad_value(782), 0.5));
            s.copy_ad(2087, 2094);
            s.store_scalar(2085, 0.0);
            s.copy_ad(2090, 2085);
            s.store_mul_sub_rhs(2092, 2127, 1436, 2089);
            s.store_mul_neg_rhs(2148, 2127, 2089);
        }

        s.b[2176] = (((-s.v[2092]) < 0.001) && (0.001 >= 0.0));
        s.v[2176] = if s.b[2176] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) {
            s.store_sub_from_scalar_ad(781, 0.001, A::neg(s.ad_value(2092)));
            s.store_square(722, 781);
        }

    }

    pub(super) fn stamp_reactive_block_40(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) {
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

        s.b[2177] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2177] = if s.b[2177] { 1.0 } else { 0.0 };

        s.b[2178] = (2.0 == 1.0);
        s.v[2178] = if s.b[2178] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) && s.b[2178]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2179] = (2.0 == 2.0);
        s.v[2179] = if s.b[2179] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) && (!s.b[2178])) && s.b[2179]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2180] = (2.0 == 4.0);
        s.v[2180] = if s.b[2180] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) && (!s.b[2178])) && (!s.b[2179])) && s.b[2180]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2181] = (2.0 == 8.0);
        s.v[2181] = if s.b[2181] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) && (!s.b[2178])) && (!s.b[2179])) && (!s.b[2180])) && s.b[2181]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign45840_loop_guard: usize = 0;
        while {
            let assign45840_cond_e61934: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign45840_cond_e61934 != 0.0
        } {
            assign45840_loop_guard += 1;
            assert!(assign45840_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && s.b[2177]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) && (!s.b[2177])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.001);
            s.store_ad_value(337, A::div_scaled_product(s.ad_value(725), s.ad_value(726), 0.001, s.ad_value(770), 1.0));
            s.store_sub_from_scalar(335, 0.001, 780);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2176]) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2176])) {
            s.store_neg(335, 2092);
            s.store_scalar(337, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_sqrt_mul(2083, 2134, 335);
        }

        s.b[2182] = (((-s.v[2148]) < 0.001) && (0.001 >= 0.0));
        s.v[2182] = if s.b[2182] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) {
            s.store_sub_from_scalar_ad(781, 0.001, A::neg(s.ad_value(2148)));
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

        s.b[2183] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2183] = if s.b[2183] { 1.0 } else { 0.0 };

        s.b[2184] = (2.0 == 1.0);
        s.v[2184] = if s.b[2184] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) && s.b[2184]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2185] = (2.0 == 2.0);
        s.v[2185] = if s.b[2185] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) && (!s.b[2184])) && s.b[2185]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2186] = (2.0 == 4.0);
        s.v[2186] = if s.b[2186] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) && (!s.b[2184])) && (!s.b[2185])) && s.b[2186]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2187] = (2.0 == 8.0);
        s.v[2187] = if s.b[2187] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) && (!s.b[2184])) && (!s.b[2185])) && (!s.b[2186])) && s.b[2187]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign46200_loop_guard: usize = 0;
        while {
            let assign46200_cond_e62486: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign46200_cond_e62486 != 0.0
        } {
            assign46200_loop_guard += 1;
            assert!(assign46200_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && s.b[2183]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) && (!s.b[2183])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.001);
            s.store_ad_value(337, A::div_scaled_product(s.ad_value(725), s.ad_value(726), 0.001, s.ad_value(770), 1.0));
            s.store_sub_from_scalar(335, 0.001, 780);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2182]) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2182])) {
            s.store_neg(335, 2148);
            s.store_scalar(337, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_sqrt_mul(2149, 2134, 335);
        }

        s.b[2188] = (p.p345 != 0.0);
        s.v[2188] = if s.b[2188] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2188]) {
            s.store_mul_sub_from_scalar_ad_rhs(335, 965, 1.0, A::scale(s.ad_value(790), p.p345));
            s.store_scale(336, 965, 0.001);
            s.store_ad_value(781, A::add_scaled_inputs3(s.ad_value(335), 1.0, s.ad_value(965), (-0.1), s.ad_value(336), -1.0));
            s.store_scaled_mul(782, 965, 336, (0.1 * 4.0));
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2188]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2188]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_ad_value(335, A::add_scaled_inputs3(s.ad_value(965), 0.1, s.ad_value(781), 0.5, s.ad_value(782), 0.5));
            s.store_ad_value(781, A::add_scaled_inputs3(s.ad_value(965), 2.0, s.ad_value(335), (-1.0), s.ad_value(336), -1.0));
            s.store_scaled_mul(782, 965, 336, (2.0 * 4.0));
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2188]) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2188]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_ad_value(965, A::add_scaled_inputs3(s.ad_value(965), 2.0, s.ad_value(781), (-0.5), s.ad_value(782), (-0.5)));
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.copy_ad(2129, 965);
            s.store_sub(2146, 965, 2083);
            s.store_sub(2147, 965, 2149);
        }

        s.b[2189] = ((s.v[2146] < (p.p344 + (p.p344 * 0.1))) && ((p.p344 * 0.1) >= 0.0));
        s.v[2189] = if s.b[2189] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) {
            s.store_sub_from_scalar(781, (p.p344 + (p.p344 * 0.1)), 2146);
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

        s.b[2190] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[2190] = if s.b[2190] { 1.0 } else { 0.0 };

        s.b[2191] = (1.0 == 1.0);
        s.v[2191] = if s.b[2191] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) && s.b[2191]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2192] = (1.0 == 2.0);
        s.v[2192] = if s.b[2192] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) && (!s.b[2191])) && s.b[2192]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2193] = (1.0 == 4.0);
        s.v[2193] = if s.b[2193] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) && (!s.b[2191])) && (!s.b[2192])) && s.b[2193]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2194] = (1.0 == 8.0);
        s.v[2194] = if s.b[2194] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) && (!s.b[2191])) && (!s.b[2192])) && (!s.b[2193])) && s.b[2194]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign46720_loop_guard: usize = 0;
        while {
            let assign46720_cond_e63320: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign46720_cond_e63320 != 0.0
        } {
            assign46720_loop_guard += 1;
            assert!(assign46720_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && s.b[2190]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) && (!s.b[2190])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / 2.0))
                }
            });
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (p.p344 * 0.1));
            s.store_ad_value(334, A::div_scaled_product(s.ad_value(725), s.ad_value(726), (p.p344 * 0.1), s.ad_value(770), 1.0));
            s.store_sub_from_scalar(2146, (p.p344 + (p.p344 * 0.1)), 780);
        }

    }

    pub(super) fn stamp_reactive_block_41(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2189]) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2189])) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2189])) {
            s.store_scalar(334, 1.0);
        }

        s.b[2195] = ((s.v[2147] < (p.p344 * 0.1)) && ((p.p344 * 0.1) >= 0.0));
        s.v[2195] = if s.b[2195] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) {
            s.store_sub_from_scalar(781, (p.p344 * 0.1), 2147);
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

        s.b[2196] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));
        s.v[2196] = if s.b[2196] { 1.0 } else { 0.0 };

        s.b[2197] = (1.0 == 1.0);
        s.v[2197] = if s.b[2197] { 1.0 } else { 0.0 };

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) && s.b[2197]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2198] = (1.0 == 2.0);
        s.v[2198] = if s.b[2198] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) && (!s.b[2197])) && s.b[2198]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2199] = (1.0 == 4.0);
        s.v[2199] = if s.b[2199] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) && (!s.b[2197])) && (!s.b[2198])) && s.b[2199]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2200] = (1.0 == 8.0);
        s.v[2200] = if s.b[2200] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) && (!s.b[2197])) && (!s.b[2198])) && (!s.b[2199])) && s.b[2200]) {
            s.store_scalar(720, 4.0);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign47050_loop_guard: usize = 0;
        while {
            let assign47050_cond_e63841: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign47050_cond_e63841 != 0.0
        } {
            assign47050_loop_guard += 1;
            assert!(assign47050_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && s.b[2196]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) && (!s.b[2196])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / 2.0))
                }
            });
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (p.p344 * 0.1));
            s.store_ad_value(334, A::div_scaled_product(s.ad_value(725), s.ad_value(726), (p.p344 * 0.1), s.ad_value(770), 1.0));
            s.store_sub_from_scalar(2147, (p.p344 * 0.1), 780);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2195]) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2195])) {
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2195])) {
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_offset_scaled_div(2150, 2146, 2147, (p.p394 - p.p395), p.p395);
            s.store_scalar(79, 0.0);
            s.store_mul(2138, 2127, 2128);
            s.store_scalar(97, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_42(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign47180_loop_guard: usize = 0;
        while {
            let assign47180_cond_e64082: f64 = if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign47180_cond_e64082 != 0.0
        } {
            assign47180_loop_guard += 1;
            assert!(assign47180_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
                s.store_mul_sub_ad_rhs(2092, 2127, A::add_scaled_product(s.ad_value(2115), 1.0, s.ad_value(2128), s.ad_value(2090), 1.0), s.ad_value(2089));
                s.store_sub(335, 2090, 2092);
            }
            s.b[2201] = ((s.v[335] < 0.001) && (0.001 >= 0.0));
            s.v[2201] = if s.b[2201] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) {
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
            s.b[2202] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[2202] = if s.b[2202] { 1.0 } else { 0.0 };
            s.b[2203] = (2.0 == 1.0);
            s.v[2203] = if s.b[2203] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) && s.b[2203]) {
                s.store_scalar(720, 1.0);
            }
            s.b[2204] = (2.0 == 2.0);
            s.v[2204] = if s.b[2204] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) && (!s.b[2203])) && s.b[2204]) {
                s.store_scalar(720, 2.0);
            }
            s.b[2205] = (2.0 == 4.0);
            s.v[2205] = if s.b[2205] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) && (!s.b[2203])) && (!s.b[2204])) && s.b[2205]) {
                s.store_scalar(720, 3.0);
            }
            s.b[2206] = (2.0 == 8.0);
            s.v[2206] = if s.b[2206] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) && (!s.b[2203])) && (!s.b[2204])) && (!s.b[2205])) && s.b[2206]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign47180_body28_loop_guard: usize = 0;
            while {
                let assign47180_body28_cond_e64480: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign47180_body28_cond_e64480 != 0.0
            } {
                assign47180_body28_loop_guard += 1;
                assert!(assign47180_body28_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && s.b[2202]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) && (!s.b[2202])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.001);
                s.store_ad_value(336, A::div_scaled_product(s.ad_value(725), s.ad_value(726), 0.001, s.ad_value(770), 1.0));
                s.store_sub_from_scalar(335, 0.001, 780);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2201]) {
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2201])) {
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2201])) {
                s.store_scalar(336, 1.0);
            }
            if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
                s.store_sqrt_mul(2083, 2134, 335);
            }
            s.b[2207] = ((s.v[2083] > (s.v[2129] - 1e-12)) && (1e-12 >= 0.0));
            s.v[2207] = if s.b[2207] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) {
                s.store_offset_sub(781, 2083, 2129, 1e-12);
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
            s.b[2208] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[2208] = if s.b[2208] { 1.0 } else { 0.0 };
            s.b[2209] = (2.0 == 1.0);
            s.v[2209] = if s.b[2209] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) && s.b[2209]) {
                s.store_scalar(720, 1.0);
            }
            s.b[2210] = (2.0 == 2.0);
            s.v[2210] = if s.b[2210] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) && (!s.b[2209])) && s.b[2210]) {
                s.store_scalar(720, 2.0);
            }
            s.b[2211] = (2.0 == 4.0);
            s.v[2211] = if s.b[2211] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) && (!s.b[2209])) && (!s.b[2210])) && s.b[2211]) {
                s.store_scalar(720, 3.0);
            }
            s.b[2212] = (2.0 == 8.0);
            s.v[2212] = if s.b[2212] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) && (!s.b[2209])) && (!s.b[2210])) && (!s.b[2211])) && s.b[2212]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign47180_body64_loop_guard: usize = 0;
            while {
                let assign47180_body64_cond_e65029: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign47180_body64_cond_e65029 != 0.0
            } {
                assign47180_body64_loop_guard += 1;
                assert!(assign47180_body64_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && s.b[2208]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) && (!s.b[2208])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-12);
                s.store_ad_value(337, A::div_scaled_product(s.ad_value(725), s.ad_value(726), 1e-12, s.ad_value(770), 1.0));
                s.store_add_ad_lhs(2083, A::offset(s.ad_value(2129), (-1e-12)), 780);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2207]) {
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2207])) {
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2207])) {
                s.store_scalar(337, 1.0);
            }
            if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
                s.store_mul(337, 336, 337);
                s.store_add_ad_rhs(2135, 2087, A::div(A::add_scaled_square_product(s.ad_value(2129), 1.0, s.ad_value(2083), A::sub_scaled_inputs(s.ad_value(2083), 1.0, s.ad_value(2129), 2.0), 1.0), s.ad_value(2134)));
                s.store_scalar(2136, 1.0);
                s.store_mul_ad(2137, A::add_scaled_product(s.ad_value(337), 1.0, A::div(s.ad_value(2129), s.ad_value(2083)), s.ad_value(337), (-1.0)), A::sub_from_scalar(1.0, s.ad_value(2138)));
            }
            s.b[2213] = ((s.v[2135] > (s.v[2085] - p.p406)) && (p.p406 >= 0.0));
            s.v[2213] = if s.b[2213] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) {
                s.store_offset_sub(781, 2135, 2085, p.p406);
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
            s.b[2214] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[2214] = if s.b[2214] { 1.0 } else { 0.0 };
            s.b[2215] = (4.0 == 1.0);
            s.v[2215] = if s.b[2215] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) && s.b[2215]) {
                s.store_scalar(720, 1.0);
            }
            s.b[2216] = (4.0 == 2.0);
            s.v[2216] = if s.b[2216] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) && (!s.b[2215])) && s.b[2216]) {
                s.store_scalar(720, 2.0);
            }
            s.b[2217] = (4.0 == 4.0);
            s.v[2217] = if s.b[2217] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) && (!s.b[2215])) && (!s.b[2216])) && s.b[2217]) {
                s.store_scalar(720, 3.0);
            }
            s.b[2218] = (4.0 == 8.0);
            s.v[2218] = if s.b[2218] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) && (!s.b[2215])) && (!s.b[2216])) && (!s.b[2217])) && s.b[2218]) {
                s.store_scalar(720, 4.0);
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign47180_body107_loop_guard: usize = 0;
            while {
                let assign47180_body107_cond_e65694: f64 = if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign47180_body107_cond_e65694 != 0.0
            } {
                assign47180_body107_loop_guard += 1;
                assert!(assign47180_body107_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && s.b[2214]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) && (!s.b[2214])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                    }
                });
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, p.p406);
                s.store_ad_value(334, A::div_scaled_product(s.ad_value(725), s.ad_value(726), p.p406, s.ad_value(770), 1.0));
                s.store_add_ad_lhs(2135, A::offset(s.ad_value(2085), (-p.p406)), 780);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2213]) {
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2213])) {
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2213])) {
                s.store_scalar(334, 1.0);
            }
            if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
                s.store_mul(2136, 2136, 334);
                s.store_mul(2137, 2137, 334);
                s.store_mul_sub_rhs(339, 154, 2087, 2090);
                s.store_exp(340, 339);
                s.store_sub_ad_lhs(344, A::offset(s.ad_value(340), (-1.0)), 339);
            }
            s.b[2219] = (s.v[339] >= 1e-7);
            s.v[2219] = if s.b[2219] { 1.0 } else { 0.0 };
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2219]) {
                s.store_scalar(347, (-1.0));
                s.store_mul_scaled_ad_rhs(2096, 209, -1.0, A::sqrt(s.ad_value(344)));
                s.store_ad_value(345, A::div_scaled_product3(s.ad_value(209), s.ad_value(209), s.ad_value(154), 0.5, s.ad_value(2096), 1.0));
                s.store_mul_offset_rhs(2123, 345, 340, (-1.0));
                s.store_mul_sub_from_scalar_rhs(2125, 345, 1.0, 340);
            }
            s.b[2220] = (s.v[339] < (-1e-7));
            s.v[2220] = if s.b[2220] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2219])) && s.b[2220]) {
                s.store_scalar(347, 1.0);
                s.store_exp_ad(342, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub_scaled_inputs(s.ad_value(2087), 1.0, s.ad_value(2115), p.p398)));
                s.store_exp_ad(343, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub_scaled_inputs(s.ad_value(2090), 1.0, s.ad_value(2115), p.p398)));
                s.store_mul_sqrt_ad_rhs(2096, 209, A::add_scaled_product(s.ad_value(344), 1.0, s.ad_value(210), A::add_scaled_inputs3(s.ad_value(342), 1.0, s.ad_value(343), (-1.0), s.ad_value(339), 1.0), 1.0));
                s.store_ad_value(345, A::div_scaled_product3(s.ad_value(209), s.ad_value(209), s.ad_value(154), 0.5, s.ad_value(2096), 1.0));
                s.store_mul_ad_rhs(2123, 345, A::add_scaled_product(A::offset(s.ad_value(340), (-1.0)), 1.0, s.ad_value(210), A::sub_from_scalar(1.0, s.ad_value(342)), 1.0));
                s.store_mul_ad_rhs(2125, 345, A::add_scaled_product(A::sub_from_scalar(1.0, s.ad_value(340)), 1.0, s.ad_value(210), A::offset(s.ad_value(343), (-1.0)), 1.0));
            }
            s.b[2221] = (s.v[339] > 0.0);
            s.v[2221] = if s.b[2221] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2219])) && (!s.b[2220])) && s.b[2221]) {
                s.store_offset_scaled(2161, 339, ((0.3333333333333333) * (0.5)), 0.5);
                s.store_sqrt(2162, 2161);
                s.store_mul_ad_affine_product_lhs(2096, s.ad_value(209), A::sqrt(s.ad_value(2161)), -1.0, 0.0, 339);
                s.store_mul_ad_affine_product_rhs(2123, 209, s.ad_value(154), A::add(s.ad_value(2162), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2162), 1.0)), -1.0, 0.0);
                s.store_neg(2125, 2123);
            }
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2219])) && (!s.b[2220])) && (!s.b[2221])) {
                s.store_offset_scaled(2161, 339, ((0.3333333333333333) * (0.5)), 0.5);
                s.store_sqrt(2162, 2161);
                s.store_mul_ad_affine_product_lhs(2096, s.ad_value(209), A::sqrt(s.ad_value(2161)), -1.0, 0.0, 339);
                s.store_mul_ad_affine_product_rhs(2123, 209, s.ad_value(154), A::add(s.ad_value(2162), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2162), 1.0)), -1.0, 0.0);
                s.store_neg(2125, 2123);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] != 0.0)) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) {
                s.store_ad_value(2098, A::add_scaled_product(s.ad_value(2096), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(2087)), 1.0));
                s.store_sub(2099, 2123, 185);
                s.copy_ad(2100, 2125);
                s.store_sub(2101, 2090, 2135);
                s.store_neg(2102, 2136);
                s.store_sub_from_scalar(2103, 1.0, 2137);
                s.store_ad_value(2104, A::add_scaled_products(s.ad_value(2099), s.ad_value(2103), 1.0, s.ad_value(2100), s.ad_value(2102), (-1.0)));
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) {
                s.store_ad_value(2105, {
                    if (s.v[2104] > 0.0) {
                        A::div_from_scalar(1.0, A::offset(s.ad_value(2104), 1e-25))
                    } else {
                        A::div_from_scalar(1.0, A::offset(s.ad_value(2104), (-1e-25)))
                    }
                });
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) {
                s.copy_ad(2106, 2103);
                s.store_neg(2107, 2100);
                s.store_neg(2108, 2102);
                s.copy_ad(2109, 2099);
                s.store_mul_scaled_ad_rhs(2110, 2105, -1.0, A::add_scaled_products(s.ad_value(2106), s.ad_value(2098), 1.0, s.ad_value(2107), s.ad_value(2101), 1.0));
                s.store_mul_scaled_ad_rhs(2111, 2105, -1.0, A::add_scaled_products(s.ad_value(2108), s.ad_value(2098), 1.0, s.ad_value(2109), s.ad_value(2101), 1.0));
                s.store_abs(335, 2110);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) {
                s.store_ad_value(335, {
                    if (s.v[335] < ((s.v[2111]) as f64).abs()) {
                        A::abs(s.ad_value(2111))
                    } else {
                        s.ad_value(335)
                    }
                });
            }
            s.b[2222] = (s.v[335] > 0.1);
            s.v[2222] = if s.b[2222] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) && s.b[2222]) {
                s.store_mul_div_from_scalar_rhs(2110, 2110, 0.1, 335);
                s.store_mul_div_from_scalar_rhs(2111, 2111, 0.1, 335);
            }
            s.b[2223] = (s.v[335] < 1e-10);
            s.v[2223] = if s.b[2223] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) && s.b[2223]) {
                s.store_scalar(79, 1.0);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (s.v[79] == 0.0)) {
                s.store_add(2087, 2087, 2110);
                s.store_add(2090, 2090, 2111);
            }
            if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
                s.store_offset(97, 97, 1.0);
            }
        }

    }

    pub(super) fn stamp_reactive_block_43(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_mul_sub_rhs(339, 154, 2087, 2090);
            s.store_exp(340, 339);
            s.store_offset_sub_ad(344, A::offset(s.ad_value(340), (-1.0)), s.ad_value(339), 1e-15);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_ad_value(2120, {
                if (s.v[339] > 0.0) {
                    A::mul_scaled_lhs(s.ad_value(209), -1.0, A::sqrt(s.ad_value(344)))
                } else {
                    A::mul(s.ad_value(209), A::sqrt(s.ad_value(344)))
                }
            });
        }

        s.b[2225] = (1.0 == 1.0);
        s.v[2225] = if s.b[2225] { 1.0 } else { 0.0 };

        s.b[2226] = (((s.v[2087] - s.v[2085]) < p.p403) && (p.p403 >= 0.0));
        s.v[2226] = if s.b[2226] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) {
            s.store_sub_from_scalar_ad(781, p.p403, A::sub(s.ad_value(2087), s.ad_value(2085)));
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

        s.b[2227] = ((((6.0 == 1.0) || (6.0 == 2.0)) || (6.0 == 4.0)) || (6.0 == 8.0));
        s.v[2227] = if s.b[2227] { 1.0 } else { 0.0 };

        s.b[2228] = (6.0 == 1.0);
        s.v[2228] = if s.b[2228] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) && s.b[2228]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2229] = (6.0 == 2.0);
        s.v[2229] = if s.b[2229] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) && (!s.b[2228])) && s.b[2229]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2230] = (6.0 == 4.0);
        s.v[2230] = if s.b[2230] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) && (!s.b[2228])) && (!s.b[2229])) && s.b[2230]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2231] = (6.0 == 8.0);
        s.v[2231] = if s.b[2231] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) && (!s.b[2228])) && (!s.b[2229])) && (!s.b[2230])) && s.b[2231]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign47590_loop_guard: usize = 0;
        while {
            let assign47590_cond_e67470: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign47590_cond_e67470 != 0.0
        } {
            assign47590_loop_guard += 1;
            assert!(assign47590_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && s.b[2227]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) && (!s.b[2227])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 6.0)))
                }
            });
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, p.p403);
            s.store_ad_value(334, A::div_scaled_product(s.ad_value(725), s.ad_value(726), p.p403, s.ad_value(770), 1.0));
            s.store_sub_from_scalar(336, p.p403, 780);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && s.b[2226]) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) && (!s.b[2226])) {
            s.store_sub(336, 2087, 2085);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2225]) {
            s.store_offset_ad(338, A::add_scaled_product(A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0)), 1e-15);
            s.store_mul_scaled_ad_rhs(2116, 209, -1.0, A::sqrt(s.ad_value(338)));
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2225])) {
            s.copy_ad(2116, 2120);
        }

        s.b[2232] = (1.0 == 1.0);
        s.v[2232] = if s.b[2232] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
            s.copy_ad(2157, 85);
            s.store_offset_mul(338, 2133, 2157, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
            s.store_ad_value(337, {
                if (s.v[338] > 0.0) {
                    A::sqrt(s.ad_value(338))
                } else {
                    A::neg(A::sqrt_scaled_input(s.ad_value(338), -1.0))
                }
            });
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
            s.store_offset_ad(2158, A::add_scaled_product(s.ad_value(2157), 1.0, s.ad_value(2132), A::sub_from_scalar(1.0, s.ad_value(337)), 1.0), p.p397);
            s.copy_ad(2154, 2158);
            s.store_scalar(79, 0.0);
            s.store_scalar(97, 1.0);
        }

        let mut assign47790_loop_guard: usize = 0;
        while {
            let assign47790_cond_e67845: f64 = if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[97] <= 150.0)) { 1.0 } else { 0.0 };
            assign47790_cond_e67845 != 0.0
        } {
            assign47790_loop_guard += 1;
            assert!(assign47790_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
                s.store_mul_neg_lhs(335, 154, 2154);
                s.store_exp(336, 335);
                s.store_sqrt_ad(338, A::div_scaled_inputs(s.ad_value(2112), 2.0, s.ad_value(154), 1.0));
                s.store_offset_sub(344, 336, 335, (-1.0));
                s.store_mul_sqrt_ad_rhs(2155, 338, A::offset(s.ad_value(344), 1e-15));
            }
            s.b[2233] = (s.v[335] > 0.0);
            s.v[2233] = if s.b[2233] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && s.b[2233]) {
                s.store_neg(2155, 2155);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
                s.store_ad_value(345, A::div_scaled_product3(s.ad_value(338), s.ad_value(338), s.ad_value(154), 0.5, s.ad_value(2155), 1.0));
                s.store_mul_sub_from_scalar_rhs(2156, 345, 1.0, 336);
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[79] != 0.0)) {
                s.store_scalar(97, (150.0 + 1.0));
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[79] == 0.0)) {
                s.store_ad_value(2098, A::add_scaled_product(s.ad_value(2155), 1.0, s.ad_value(185), A::offset(A::sub(s.ad_value(2157), s.ad_value(2154)), p.p397), -1.0));
                s.store_add(2099, 185, 2156);
                s.store_scaled_div(2110, 2098, 2099, -1.0);
            }
            s.b[2234] = (((s.v[2110]) as f64).abs() < 1e-10);
            s.v[2234] = if s.b[2234] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[79] == 0.0)) && s.b[2234]) {
                s.store_scalar(79, 1.0);
            }
            s.b[2235] = (s.v[2110] > 0.1);
            s.v[2235] = if s.b[2235] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[79] == 0.0)) && (!s.b[2234])) && s.b[2235]) {
                s.store_scalar(2110, 0.1);
            }
            s.b[2236] = (s.v[2110] < (-0.1));
            s.v[2236] = if s.b[2236] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[79] == 0.0)) && (!s.b[2234])) && (!s.b[2235])) && s.b[2236]) {
                s.store_scalar(2110, (-0.1));
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && (s.v[79] == 0.0)) {
                s.store_add(2154, 2154, 2110);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
                s.store_offset(97, 97, 1.0);
            }
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) {
            s.copy_ad(2159, 2154);
            s.store_sqrt_square_offset(782, 2159, ((4.0 * p.p404) * p.p404));
            s.store_offset_scaled_div(334, 2159, 782, 0.5, 0.5);
            s.store_scaled_add(2160, 2159, 782, 0.5);
        }

        s.b[2237] = (s.v[2160] < 0.0);
        s.v[2237] = if s.b[2237] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2232]) && s.b[2237]) {
            s.store_scalar(2160, 0.0);
            s.store_scalar(334, 0.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) {
            s.store_offset_mul(338, 2133, 85, 1.0);
            s.store_offset(339, 2133, 1.0);
        }

        s.b[2238] = ((s.v[338] < s.v[339]) && (s.v[339] >= 0.0));
        s.v[2238] = if s.b[2238] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) {
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

        s.b[2239] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2239] = if s.b[2239] { 1.0 } else { 0.0 };

        s.b[2240] = (2.0 == 1.0);
        s.v[2240] = if s.b[2240] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) && s.b[2240]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2241] = (2.0 == 2.0);
        s.v[2241] = if s.b[2241] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) && (!s.b[2240])) && s.b[2241]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2242] = (2.0 == 4.0);
        s.v[2242] = if s.b[2242] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) && (!s.b[2240])) && (!s.b[2241])) && s.b[2242]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2243] = (2.0 == 8.0);
        s.v[2243] = if s.b[2243] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) && (!s.b[2240])) && (!s.b[2241])) && (!s.b[2242])) && s.b[2243]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign48150_loop_guard: usize = 0;
        while {
            let assign48150_cond_e68740: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign48150_cond_e68740 != 0.0
        } {
            assign48150_loop_guard += 1;
            assert!(assign48150_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && s.b[2239]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) && (!s.b[2239])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 339, 726);
            s.store_ad_value(334, A::div_scaled_product3(s.ad_value(339), s.ad_value(725), s.ad_value(726), 1.0, s.ad_value(770), 1.0));
            s.store_sub(338, 339, 780);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2238]) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && (!s.b[2238])) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && (!s.b[2238])) {
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) {
            s.store_sqrt(337, 338);
        }

    }

    pub(super) fn stamp_reactive_block_44(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) {
            s.store_ad_value(344, A::add_scaled_product(s.ad_value(85), 1.0, s.ad_value(2132), A::sub_from_scalar(1.0, s.ad_value(337)), 1.0));
        }

        s.b[2244] = ((s.v[344] < p.p404) && (p.p404 >= 0.0));
        s.v[2244] = if s.b[2244] { 1.0 } else { 0.0 };

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) {
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

        s.b[2245] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2245] = if s.b[2245] { 1.0 } else { 0.0 };

        s.b[2246] = (2.0 == 1.0);
        s.v[2246] = if s.b[2246] { 1.0 } else { 0.0 };

        if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) && s.b[2246]) {
            s.store_scalar(720, 1.0);
        }

        s.b[2247] = (2.0 == 2.0);
        s.v[2247] = if s.b[2247] { 1.0 } else { 0.0 };

        if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) && (!s.b[2246])) && s.b[2247]) {
            s.store_scalar(720, 2.0);
        }

        s.b[2248] = (2.0 == 4.0);
        s.v[2248] = if s.b[2248] { 1.0 } else { 0.0 };

        if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) && (!s.b[2246])) && (!s.b[2247])) && s.b[2248]) {
            s.store_scalar(720, 3.0);
        }

        s.b[2249] = (2.0 == 8.0);
        s.v[2249] = if s.b[2249] { 1.0 } else { 0.0 };

        if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) && (!s.b[2246])) && (!s.b[2247])) && (!s.b[2248])) && s.b[2249]) {
            s.store_scalar(720, 4.0);
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) {
            s.store_scalar(719, 0.0);
        }

        let mut assign48520_loop_guard: usize = 0;
        while {
            let assign48520_cond_e69403: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign48520_cond_e69403 != 0.0
        } {
            assign48520_loop_guard += 1;
            assert!(assign48520_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && s.b[2245]) {
                s.store_sqrt(726, 726);
                s.store_offset(719, 719, 1.0);
            }
        }

        if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) && (!s.b[2245])) {
            s.store_ad_value(726, {
                if (s.v[726] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                }
            });
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, p.p404);
            s.store_ad_value(334, A::div_scaled_product(s.ad_value(725), s.ad_value(726), p.p404, s.ad_value(770), 1.0));
            s.store_sub_from_scalar(2160, p.p404, 780);
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && s.b[2244]) {
        }

        if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2232])) && (!s.b[2244])) {
            s.copy_ad(2160, 344);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.copy_ad(349, 790);
            s.store_div(335, 790, 2160);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_ad_value(336, {
                if (s.v[335] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(335), s.ad_value(658))
                }
            });
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_offset(337, 336, 1.0);
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_ad_value(338, {
                if (s.v[337] == 0.0) {
                    A::constant(0.0)
                } else {
                    A::pow(s.ad_value(337), A::div_from_scalar(1.0, s.ad_value(658)))
                }
            });
        }

        if (s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) {
            s.store_div(348, 790, 338);
            s.copy_ad(790, 348);
        }

        s.b[2250] = (s.v[790] < 0.0);
        s.v[2250] = if s.b[2250] { 1.0 } else { 0.0 };

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && s.b[2250]) {
            s.copy_ad(2088, 2087);
            s.copy_ad(2093, 2092);
            s.copy_ad(2091, 2090);
            s.copy_ad(2121, 2120);
            s.copy_ad(2117, 2116);
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
            s.copy_ad(2086, 790);
            s.store_offset_ad(781, A::add_scaled_inputs3(s.ad_value(2087), 1.0, s.ad_value(2086), 1.0, s.ad_value(85), -1.0), (-0.01));
            s.store_scaled_add(782, 2087, 2086, (4.0 * 0.01));
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_sub_ad(2095, A::add(s.ad_value(2087), s.ad_value(2086)), A::add_scaled_inputs(s.ad_value(781), 0.5, s.ad_value(782), 0.5));
            s.store_offset_ad(781, A::add_scaled_inputs3(s.ad_value(2095), 1.0, s.ad_value(2115), -1.0, s.ad_value(2089), 1.0), (-0.01));
            s.store_scaled_sub(782, 2115, 2089, (4.0 * 0.01));
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
            s.store_ad_value(782, {
                if (s.v[782] > 0.0) {
                    s.ad_value(782)
                } else {
                    A::neg(s.ad_value(782))
                }
            });
        }

        if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_ad(2095, A::sub(s.ad_value(2115), s.ad_value(2089)), A::add_scaled_inputs(s.ad_value(781), 0.5, s.ad_value(782), 0.5));
            s.copy_ad(2091, 2086);
            s.copy_ad(2088, 2095);
            s.store_scalar(79, 0.0);
            s.store_mul(2139, 2127, 2128);
            s.store_scalar(98, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_45(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut assign48920_loop_guard: usize = 0;
        while {
            let assign48920_cond_e70112: f64 = if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (s.v[98] <= 150.0)) { 1.0 } else { 0.0 };
            assign48920_cond_e70112 != 0.0
        } {
            assign48920_loop_guard += 1;
            assert!(assign48920_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
                s.store_mul_sub_ad_rhs(2093, 2127, A::add_scaled_product(s.ad_value(2115), 1.0, s.ad_value(2128), s.ad_value(2091), 1.0), s.ad_value(2089));
                s.store_sub(335, 2091, 2093);
            }
            s.b[2251] = ((s.v[335] < 0.001) && (0.001 >= 0.0));
            s.v[2251] = if s.b[2251] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) {
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
            s.b[2252] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[2252] = if s.b[2252] { 1.0 } else { 0.0 };
            s.b[2253] = (2.0 == 1.0);
            s.v[2253] = if s.b[2253] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) && s.b[2252]) && s.b[2253]) {
                s.store_scalar(720, 1.0);
            }
            s.b[2254] = (2.0 == 2.0);
            s.v[2254] = if s.b[2254] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) && s.b[2252]) && (!s.b[2253])) && s.b[2254]) {
                s.store_scalar(720, 2.0);
            }
            s.b[2255] = (2.0 == 4.0);
            s.v[2255] = if s.b[2255] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) && s.b[2252]) && (!s.b[2253])) && (!s.b[2254])) && s.b[2255]) {
                s.store_scalar(720, 3.0);
            }
            s.b[2256] = (2.0 == 8.0);
            s.v[2256] = if s.b[2256] { 1.0 } else { 0.0 };
            if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) && s.b[2252]) && (!s.b[2253])) && (!s.b[2254])) && (!s.b[2255])) && s.b[2256]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) && s.b[2252]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign48920_body28_loop_guard: usize = 0;
            while {
                let assign48920_body28_cond_e70579: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) && s.b[2252]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign48920_body28_cond_e70579 != 0.0
            } {
                assign48920_body28_loop_guard += 1;
                assert!(assign48920_body28_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) && s.b[2252]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) && (!s.b[2252])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 0.001);
                s.store_ad_value(336, A::div_scaled_product(s.ad_value(725), s.ad_value(726), 0.001, s.ad_value(770), 1.0));
                s.store_sub_from_scalar(335, 0.001, 780);
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2251]) {
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2251])) {
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2251])) {
                s.store_scalar(336, 1.0);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
                s.store_sqrt_mul(2084, 2134, 335);
            }
            s.b[2257] = ((s.v[2084] > (s.v[2129] - 1e-12)) && (1e-12 >= 0.0));
            s.v[2257] = if s.b[2257] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) {
                s.store_offset_sub(781, 2084, 2129, 1e-12);
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
            s.b[2258] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
            s.v[2258] = if s.b[2258] { 1.0 } else { 0.0 };
            s.b[2259] = (2.0 == 1.0);
            s.v[2259] = if s.b[2259] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) && s.b[2258]) && s.b[2259]) {
                s.store_scalar(720, 1.0);
            }
            s.b[2260] = (2.0 == 2.0);
            s.v[2260] = if s.b[2260] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) && s.b[2258]) && (!s.b[2259])) && s.b[2260]) {
                s.store_scalar(720, 2.0);
            }
            s.b[2261] = (2.0 == 4.0);
            s.v[2261] = if s.b[2261] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) && s.b[2258]) && (!s.b[2259])) && (!s.b[2260])) && s.b[2261]) {
                s.store_scalar(720, 3.0);
            }
            s.b[2262] = (2.0 == 8.0);
            s.v[2262] = if s.b[2262] { 1.0 } else { 0.0 };
            if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) && s.b[2258]) && (!s.b[2259])) && (!s.b[2260])) && (!s.b[2261])) && s.b[2262]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) && s.b[2258]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign48920_body64_loop_guard: usize = 0;
            while {
                let assign48920_body64_cond_e71224: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) && s.b[2258]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign48920_body64_cond_e71224 != 0.0
            } {
                assign48920_body64_loop_guard += 1;
                assert!(assign48920_body64_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) && s.b[2258]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) && (!s.b[2258])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 2.0)))
                    }
                });
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-12);
                s.store_ad_value(337, A::div_scaled_product(s.ad_value(725), s.ad_value(726), 1e-12, s.ad_value(770), 1.0));
                s.store_add_ad_lhs(2084, A::offset(s.ad_value(2129), (-1e-12)), 780);
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2257]) {
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2257])) {
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2257])) {
                s.store_scalar(337, 1.0);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
                s.store_mul(337, 336, 337);
                s.store_add_ad_rhs(2135, 2088, A::div(A::add_scaled_square_product(s.ad_value(2129), 1.0, s.ad_value(2084), A::sub_scaled_inputs(s.ad_value(2084), 1.0, s.ad_value(2129), 2.0), 1.0), s.ad_value(2134)));
                s.store_scalar(2136, 1.0);
                s.store_mul_ad(2137, A::add_scaled_product(s.ad_value(337), 1.0, A::div(s.ad_value(2129), s.ad_value(2084)), s.ad_value(337), (-1.0)), A::sub_from_scalar(1.0, s.ad_value(2139)));
            }
            s.b[2263] = ((s.v[2135] > (s.v[2086] - p.p406)) && (p.p406 >= 0.0));
            s.v[2263] = if s.b[2263] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) {
                s.store_offset_sub(781, 2135, 2086, p.p406);
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
            s.b[2264] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
            s.v[2264] = if s.b[2264] { 1.0 } else { 0.0 };
            s.b[2265] = (4.0 == 1.0);
            s.v[2265] = if s.b[2265] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) && s.b[2264]) && s.b[2265]) {
                s.store_scalar(720, 1.0);
            }
            s.b[2266] = (4.0 == 2.0);
            s.v[2266] = if s.b[2266] { 1.0 } else { 0.0 };
            if ((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) && s.b[2264]) && (!s.b[2265])) && s.b[2266]) {
                s.store_scalar(720, 2.0);
            }
            s.b[2267] = (4.0 == 4.0);
            s.v[2267] = if s.b[2267] { 1.0 } else { 0.0 };
            if (((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) && s.b[2264]) && (!s.b[2265])) && (!s.b[2266])) && s.b[2267]) {
                s.store_scalar(720, 3.0);
            }
            s.b[2268] = (4.0 == 8.0);
            s.v[2268] = if s.b[2268] { 1.0 } else { 0.0 };
            if ((((((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) && s.b[2264]) && (!s.b[2265])) && (!s.b[2266])) && (!s.b[2267])) && s.b[2268]) {
                s.store_scalar(720, 4.0);
            }
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) && s.b[2264]) {
                s.store_scalar(719, 0.0);
            }
            let mut assign48920_body107_loop_guard: usize = 0;
            while {
                let assign48920_body107_cond_e72006: f64 = if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) && s.b[2264]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign48920_body107_cond_e72006 != 0.0
            } {
                assign48920_body107_loop_guard += 1;
                assert!(assign48920_body107_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) && s.b[2264]) {
                    s.store_sqrt(726, 726);
                    s.store_offset(719, 719, 1.0);
                }
            }
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) && (!s.b[2264])) {
                s.store_ad_value(726, {
                    if (s.v[726] == 0.0) {
                        A::constant(0.0)
                    } else {
                        A::powf(s.ad_value(726), (1.0 / (2.0 * 4.0)))
                    }
                });
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, p.p406);
                s.store_ad_value(334, A::div_scaled_product(s.ad_value(725), s.ad_value(726), p.p406, s.ad_value(770), 1.0));
                s.store_add_ad_lhs(2135, A::offset(s.ad_value(2086), (-p.p406)), 780);
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2263]) {
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2263])) {
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2263])) {
                s.store_scalar(334, 1.0);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
                s.store_mul(2136, 2136, 334);
                s.store_mul(2137, 2137, 334);
                s.store_mul_sub_rhs(339, 154, 2088, 2091);
                s.store_exp(340, 339);
                s.store_sub_ad_lhs(344, A::offset(s.ad_value(340), (-1.0)), 339);
            }
            s.b[2269] = (s.v[339] >= 1e-7);
            s.v[2269] = if s.b[2269] { 1.0 } else { 0.0 };
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && s.b[2269]) {
                s.store_scalar(347, (-1.0));
                s.store_mul_scaled_ad_rhs(2097, 209, -1.0, A::sqrt(s.ad_value(344)));
                s.store_ad_value(345, A::div_scaled_product3(s.ad_value(209), s.ad_value(209), s.ad_value(154), 0.5, s.ad_value(2097), 1.0));
                s.store_mul_offset_rhs(2124, 345, 340, (-1.0));
                s.store_mul_sub_from_scalar_rhs(2126, 345, 1.0, 340);
            }
            s.b[2270] = (s.v[339] < (-1e-7));
            s.v[2270] = if s.b[2270] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2269])) && s.b[2270]) {
                s.store_scalar(347, 1.0);
                s.store_exp_ad(342, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub_scaled_inputs(s.ad_value(2088), 1.0, s.ad_value(2115), p.p398)));
                s.store_exp_ad(343, A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub_scaled_inputs(s.ad_value(2091), 1.0, s.ad_value(2115), p.p398)));
                s.store_mul_sqrt_ad_rhs(2097, 209, A::add_scaled_product(s.ad_value(344), 1.0, s.ad_value(210), A::add_scaled_inputs3(s.ad_value(342), 1.0, s.ad_value(343), (-1.0), s.ad_value(339), 1.0), 1.0));
                s.store_ad_value(345, A::div_scaled_product3(s.ad_value(209), s.ad_value(209), s.ad_value(154), 0.5, s.ad_value(2097), 1.0));
                s.store_mul_ad_rhs(2124, 345, A::add_scaled_product(A::offset(s.ad_value(340), (-1.0)), 1.0, s.ad_value(210), A::sub_from_scalar(1.0, s.ad_value(342)), 1.0));
                s.store_mul_ad_rhs(2126, 345, A::add_scaled_product(A::sub_from_scalar(1.0, s.ad_value(340)), 1.0, s.ad_value(210), A::offset(s.ad_value(343), (-1.0)), 1.0));
            }
            s.b[2271] = (s.v[339] > 0.0);
            s.v[2271] = if s.b[2271] { 1.0 } else { 0.0 };
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2269])) && (!s.b[2270])) && s.b[2271]) {
                s.store_offset_scaled(2161, 339, ((0.3333333333333333) * (0.5)), 0.5);
                s.store_sqrt(2162, 2161);
                s.store_mul_ad_affine_product_lhs(2097, s.ad_value(209), A::sqrt(s.ad_value(2161)), -1.0, 0.0, 339);
                s.store_mul_ad_affine_product_rhs(2124, 209, s.ad_value(154), A::add(s.ad_value(2162), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2162), 1.0)), -1.0, 0.0);
                s.store_neg(2126, 2124);
            }
            if (((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (!s.b[2269])) && (!s.b[2270])) && (!s.b[2271])) {
                s.store_offset_scaled(2161, 339, ((0.3333333333333333) * (0.5)), 0.5);
                s.store_sqrt(2162, 2161);
                s.store_mul_ad_affine_product_lhs(2097, s.ad_value(209), A::sqrt(s.ad_value(2161)), -1.0, 0.0, 339);
                s.store_mul_ad_affine_product_rhs(2124, 209, s.ad_value(154), A::add(s.ad_value(2162), A::div_scaled_inputs(s.ad_value(339), 0.08333333333333333, s.ad_value(2162), 1.0)), -1.0, 0.0);
                s.store_neg(2126, 2124);
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (s.v[79] != 0.0)) {
                s.store_scalar(98, (150.0 + 1.0));
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (s.v[79] == 0.0)) {
                s.store_ad_value(2098, A::add_scaled_product(s.ad_value(2097), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(2088)), 1.0));
                s.store_sub(2099, 2124, 185);
                s.copy_ad(2100, 2126);
                s.store_sub(2101, 2091, 2135);
                s.store_neg(2102, 2136);
                s.store_sub_from_scalar(2103, 1.0, 2137);
                s.store_ad_value(2104, A::add_scaled_products(s.ad_value(2099), s.ad_value(2103), 1.0, s.ad_value(2100), s.ad_value(2102), (-1.0)));
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (s.v[79] == 0.0)) {
                s.store_ad_value(2105, {
                    if (s.v[2104] > 0.0) {
                        A::div_from_scalar(1.0, A::offset(s.ad_value(2104), 1e-25))
                    } else {
                        A::div_from_scalar(1.0, A::offset(s.ad_value(2104), (-1e-25)))
                    }
                });
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (s.v[79] == 0.0)) {
                s.copy_ad(2106, 2103);
                s.store_neg(2107, 2100);
                s.store_neg(2108, 2102);
                s.copy_ad(2109, 2099);
                s.store_mul_scaled_ad_rhs(2110, 2105, -1.0, A::add_scaled_products(s.ad_value(2106), s.ad_value(2098), 1.0, s.ad_value(2107), s.ad_value(2101), 1.0));
                s.store_mul_scaled_ad_rhs(2111, 2105, -1.0, A::add_scaled_products(s.ad_value(2108), s.ad_value(2098), 1.0, s.ad_value(2109), s.ad_value(2101), 1.0));
                s.store_abs(335, 2110);
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (s.v[79] == 0.0)) {
                s.store_ad_value(335, {
                    if (s.v[335] < ((s.v[2111]) as f64).abs()) {
                        A::abs(s.ad_value(2111))
                    } else {
                        s.ad_value(335)
                    }
                });
            }
            s.b[2272] = (s.v[335] > 0.1);
            s.v[2272] = if s.b[2272] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (s.v[79] == 0.0)) && s.b[2272]) {
                s.store_mul_div_from_scalar_rhs(2110, 2110, 0.1, 335);
                s.store_mul_div_from_scalar_rhs(2111, 2111, 0.1, 335);
            }
            s.b[2273] = (s.v[335] < 1e-10);
            s.v[2273] = if s.b[2273] { 1.0 } else { 0.0 };
            if ((((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (s.v[79] == 0.0)) && s.b[2273]) {
                s.store_scalar(79, 1.0);
            }
            if (((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) && (s.v[79] == 0.0)) {
                s.store_add(2088, 2088, 2110);
                s.store_add(2091, 2091, 2111);
            }
            if ((s.b[1441] && (s.b[1444] && (!(s.b[1442] || s.b[1443])))) && (!s.b[2250])) {
                s.store_offset(98, 98, 1.0);
            }
        }

    }
}

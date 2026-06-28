#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_32(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_scalar(1910, 3.0);
            s.store_scalar(1849, 0.0);
            s.store_scalar(1850, 0.0);
            s.store_scalar(1858, 0.0);
            s.store_scalar(1859, 0.0);
            s.store_scalar(1891, 0.0);
            s.store_scalar(1892, 0.0);
            s.store_scalar(1862, 0.0);
            s.store_scalar(1864, 0.0);
            s.store_scalar(1863, 0.0);
            s.store_scalar(1865, 0.0);
            s.store_scalar(1835, 0.0);
            s.store_scalar(1830, 0.0);
            s.copy_ad(1883, 1431);
            s.store_offset(85, 85, ((10.0 * 2.220446049250313e-16) * 100000000.0));
            s.store_div_scaled_product_add_scaled_denominator_indices(962, 1904, 622, 1.0, 964, 1.0, 622, 1.0, 1.0);
            s.store_sub(335, 1851, 1434);
        }

        s.b[1913] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
        s.v[1913] = if s.b[1913] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) {
            s.store_sub_from_scalar(781, 0.1, 335);
            s.store_square(722, 781);
            s.store_scalar(723, (0.1 * 0.1));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign37330_e42937,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign37330_e42937;

        let (assign37340_e42948,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign37340_e42948;

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) {
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

        s.b[1914] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));
        s.v[1914] = if s.b[1914] { 1.0 } else { 0.0 };

        s.b[1915] = (4.0 == 1.0);
        s.v[1915] = if s.b[1915] { 1.0 } else { 0.0 };

        let (assign37490_e43131,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) && s.b[1915]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign37490_e43131;

        s.b[1916] = (4.0 == 2.0);
        s.v[1916] = if s.b[1916] { 1.0 } else { 0.0 };

        let (assign37510_e43152,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) && (!s.b[1915])) && s.b[1916]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign37510_e43152;

        s.b[1917] = (4.0 == 4.0);
        s.v[1917] = if s.b[1917] { 1.0 } else { 0.0 };

        let (assign37530_e43176,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) && (!s.b[1915])) && (!s.b[1916])) && s.b[1917]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign37530_e43176;

        s.b[1918] = (4.0 == 8.0);
        s.v[1918] = if s.b[1918] { 1.0 } else { 0.0 };

        let (assign37550_e43203,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) && (!s.b[1915])) && (!s.b[1916])) && (!s.b[1917])) && s.b[1918]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign37550_e43203;

        let (assign37560_e43216,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign37560_e43216;

        let mut assign37570_loop_guard: usize = 0;
        while {
            let assign37570_cond_e43230: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign37570_cond_e43230 != 0.0
        } {
            assign37570_loop_guard += 1;
            assert!(assign37570_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) {
                s.store_sqrt(726, 726);
            }
            let (assign37570_body1_e43259,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && s.b[1914]) {
        let assign37570_body1_e43257: f64 = (s.v[719] + 1.0);
        (assign37570_body1_e43257,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign37570_body1_e43259;
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) && (!s.b[1914])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, 0.1);
            s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);
            s.store_sub_from_scalar(336, 0.1, 780);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1913]) {
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1913])) {
            s.copy_ad(336, 335);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_mul(1834, 962, 336);
            s.store_sqrt(1832, 1834);
        }

        s.b[1919] = (p.p345 != 0.0);
        s.v[1919] = if s.b[1919] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1919]) {
            s.store_mul_sub_from_scalar_ad_rhs(335, 965, 1.0, A::scale(s.ad_value(790), p.p345));
            s.store_scale(336, 965, 0.001);
            s.store_add_scaled_inputs3_indices(781, 335, 1.0, 965, (-0.1), 336, -1.0);
            s.store_scaled_mul(782, 965, 336, (0.1 * 4.0));
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1919]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1919]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(335, 965, 0.1, 781, 0.5, 782, 0.5);
            s.store_add_scaled_inputs3_indices(781, 965, 2.0, 335, (-1.0), 336, -1.0);
            s.store_scaled_mul(782, 965, 336, (2.0 * 4.0));
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1919]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1919]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(965, 965, 2.0, 781, (-0.5), 782, (-0.5));
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_scalar(1847, 0.0);
        }

        s.b[1920] = (s.v[1832] > s.v[965]);
        s.v[1920] = if s.b[1920] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1920]) {
            s.copy_ad(1831, 965);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1920])) {
            s.copy_ad(1831, 1832);
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_offset_sub_from_scalar_ad(781, 0.3, s.ad_value(85), (-0.01));
            s.store_scalar(782, ((4.0 * 0.3) * 0.01));
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(1856, 781, (-0.5), 782, (-0.5), 0.3);
            s.store_add_scaled_inputs3_offset_indices(781, 1856, 1.0, 1883, -1.0, 1851, 1.0, (-0.01));
            s.store_scaled_sub(782, 1883, 1851, (4.0 * 0.01));
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4_indices(1856, 1883, 1.0, 1851, (-1.0), 781, 0.5, 782, 0.5);
            s.store_div_scaled_product_add_scaled_denominator_indices(1884, 1851, 622, -1.0, 622, 1.0, 964, 1.0, 1.0);
            s.store_offset_sub(1830, 965, 1831, 1e-15);
        }

        let (assign38010_e43888,) = {
    if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign38010_e43888;

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_scalar(1846, 0.2);
            s.copy_ad(1849, 1856);
            s.copy_ad(1852, 1847);
            s.copy_ad(1854, 1884);
        }

        let (assign38060_e43933,) = {
    if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign38060_e43933;

    }

    pub(super) fn stamp_transient_block_33(
        s: &mut Scratch,
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
            }
            let (assign38070_body9_e44065,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38070_body9_e44065;
            let (assign38070_body10_e44076,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38070_body10_e44076;
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) {
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
            let (assign38070_body21_e44207,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) && s.b[1923]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38070_body21_e44207;
            s.b[1924] = (2.0 == 2.0);
            s.v[1924] = if s.b[1924] { 1.0 } else { 0.0 };
            let (assign38070_body23_e44228,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) && (!s.b[1923])) && s.b[1924]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38070_body23_e44228;
            s.b[1925] = (2.0 == 4.0);
            s.v[1925] = if s.b[1925] { 1.0 } else { 0.0 };
            let (assign38070_body25_e44252,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) && (!s.b[1923])) && (!s.b[1924])) && s.b[1925]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38070_body25_e44252;
            s.b[1926] = (2.0 == 8.0);
            s.v[1926] = if s.b[1926] { 1.0 } else { 0.0 };
            let (assign38070_body27_e44279,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) && (!s.b[1923])) && (!s.b[1924])) && (!s.b[1925])) && s.b[1926]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38070_body27_e44279;
            let (assign38070_body28_e44292,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38070_body28_e44292;
            let mut assign38070_body29_loop_guard: usize = 0;
            while {
                let assign38070_body29_cond_e44306: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38070_body29_cond_e44306 != 0.0
            } {
                assign38070_body29_loop_guard += 1;
                assert!(assign38070_body29_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) {
                    s.store_sqrt(726, 726);
                }
                let (assign38070_body29_body1_e44335,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && s.b[1922]) {
        let assign38070_body29_body1_e44333: f64 = (s.v[719] + 1.0);
        (assign38070_body29_body1_e44333,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign38070_body29_body1_e44335;
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1921]) && (!s.b[1922])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
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
            }
            let (assign38070_body45_e44550,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38070_body45_e44550;
            let (assign38070_body46_e44561,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38070_body46_e44561;
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) {
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
            let (assign38070_body57_e44692,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) && s.b[1929]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38070_body57_e44692;
            s.b[1930] = (2.0 == 2.0);
            s.v[1930] = if s.b[1930] { 1.0 } else { 0.0 };
            let (assign38070_body59_e44713,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) && (!s.b[1929])) && s.b[1930]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38070_body59_e44713;
            s.b[1931] = (2.0 == 4.0);
            s.v[1931] = if s.b[1931] { 1.0 } else { 0.0 };
            let (assign38070_body61_e44737,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) && (!s.b[1929])) && (!s.b[1930])) && s.b[1931]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38070_body61_e44737;
            s.b[1932] = (2.0 == 8.0);
            s.v[1932] = if s.b[1932] { 1.0 } else { 0.0 };
            let (assign38070_body63_e44764,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) && (!s.b[1929])) && (!s.b[1930])) && (!s.b[1931])) && s.b[1932]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38070_body63_e44764;
            let (assign38070_body64_e44777,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38070_body64_e44777;
            let mut assign38070_body65_loop_guard: usize = 0;
            while {
                let assign38070_body65_cond_e44791: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38070_body65_cond_e44791 != 0.0
            } {
                assign38070_body65_loop_guard += 1;
                assert!(assign38070_body65_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) {
                    s.store_sqrt(726, 726);
                }
                let (assign38070_body65_body1_e44820,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && s.b[1928]) {
        let assign38070_body65_body1_e44818: f64 = (s.v[719] + 1.0);
        (assign38070_body65_body1_e44818,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign38070_body65_body1_e44820;
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) && (!s.b[1928])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1927]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
                s.store_add_offset_lhs(1826, 965, (-1e-8), 780);
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
                s.store_mul_sub_from_scalar_rhs_ad_lhs(1837, A::add_scaled_product(s.ad_value(334), 1.0, A::div(s.ad_value(965), s.ad_value(1826)), s.ad_value(334), (-1.0)), 1.0, 1838);
            }
            s.b[1934] = ((s.v[1835] > (s.v[1847] - s.v[1846])) && (s.v[1846] >= 0.0));
            s.v[1934] = if s.b[1934] { 1.0 } else { 0.0 };
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) {
                s.store_add_scaled_inputs3_indices(781, 1835, 1.0, 1847, (-1.0), 1846, 1.0);
                s.store_square(722, 781);
                s.store_square(723, 1846);
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign38070_body90_e45171,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38070_body90_e45171;
            let (assign38070_body91_e45182,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38070_body91_e45182;
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) {
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
            let (assign38070_body106_e45365,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) && s.b[1936]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38070_body106_e45365;
            s.b[1937] = (4.0 == 2.0);
            s.v[1937] = if s.b[1937] { 1.0 } else { 0.0 };
            let (assign38070_body108_e45386,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) && (!s.b[1936])) && s.b[1937]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38070_body108_e45386;
            s.b[1938] = (4.0 == 4.0);
            s.v[1938] = if s.b[1938] { 1.0 } else { 0.0 };
            let (assign38070_body110_e45410,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) && (!s.b[1936])) && (!s.b[1937])) && s.b[1938]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38070_body110_e45410;
            s.b[1939] = (4.0 == 8.0);
            s.v[1939] = if s.b[1939] { 1.0 } else { 0.0 };
            let (assign38070_body112_e45437,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) && (!s.b[1936])) && (!s.b[1937])) && (!s.b[1938])) && s.b[1939]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38070_body112_e45437;
            let (assign38070_body113_e45450,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38070_body113_e45450;
            let mut assign38070_body114_loop_guard: usize = 0;
            while {
                let assign38070_body114_cond_e45464: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38070_body114_cond_e45464 != 0.0
            } {
                assign38070_body114_loop_guard += 1;
                assert!(assign38070_body114_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) {
                    s.store_sqrt(726, 726);
                }
                let (assign38070_body114_body1_e45493,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && s.b[1935]) {
        let assign38070_body114_body1_e45491: f64 = (s.v[719] + 1.0);
        (assign38070_body114_body1_e45491,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign38070_body114_body1_e45493;
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) && (!s.b[1935])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1934]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1846, 726);
                s.store_div_scaled_product3_indices(334, 1846, 725, 726, 1.0, 770, 1.0);
                s.store_add_scaled_inputs3_indices(1835, 1847, 1.0, 1846, (-1.0), 780, 1.0);
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
                s.store_add_scaled_inputs3_indices(335, 1854, 1.0, 1883, (-1.0), 1851, 1.0);
            }
            s.b[1940] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1940] = if s.b[1940] { 1.0 } else { 0.0 };
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) {
                s.store_sub_from_scalar(781, 0.1, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.1 * 0.1));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign38070_body132_e45731,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38070_body132_e45731;
            let (assign38070_body133_e45742,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38070_body133_e45742;
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) {
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
            let (assign38070_body144_e45873,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) && s.b[1942]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38070_body144_e45873;
            s.b[1943] = (2.0 == 2.0);
            s.v[1943] = if s.b[1943] { 1.0 } else { 0.0 };
            let (assign38070_body146_e45894,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) && (!s.b[1942])) && s.b[1943]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38070_body146_e45894;
            s.b[1944] = (2.0 == 4.0);
            s.v[1944] = if s.b[1944] { 1.0 } else { 0.0 };
            let (assign38070_body148_e45918,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) && (!s.b[1942])) && (!s.b[1943])) && s.b[1944]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38070_body148_e45918;
            s.b[1945] = (2.0 == 8.0);
            s.v[1945] = if s.b[1945] { 1.0 } else { 0.0 };
            let (assign38070_body150_e45945,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) && (!s.b[1942])) && (!s.b[1943])) && (!s.b[1944])) && s.b[1945]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign38070_body150_e45945;
            let (assign38070_body151_e45958,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38070_body151_e45958;
            let mut assign38070_body152_loop_guard: usize = 0;
            while {
                let assign38070_body152_cond_e45972: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign38070_body152_cond_e45972 != 0.0
            } {
                assign38070_body152_loop_guard += 1;
                assert!(assign38070_body152_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) {
                    s.store_sqrt(726, 726);
                }
                let (assign38070_body152_body1_e46001,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && s.b[1941]) {
        let assign38070_body152_body1_e45999: f64 = (s.v[719] + 1.0);
        (assign38070_body152_body1_e45999,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign38070_body152_body1_e46001;
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1940]) && (!s.b[1941])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
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
                s.store_mul_scaled_sqrt_ad_rhs(1858, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
                s.store_mul_ad(1893, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(1858), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
                s.store_neg(1895, 1893);
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1946])) {
                s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1849), s.ad_value(1883)));
                s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1852), s.ad_value(1883)));
                s.store_mul_sqrt_ad_rhs(1858, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1858, 1.0);
                s.store_mul_add_ad_rhs(1893, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1895, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            let (assign38070_body176_e46385,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] != 0.0)) {
        let assign38070_body176_e46383: f64 = (150.0 + 1.0);
        (assign38070_body176_e46383,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign38070_body176_e46385;
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) {
                s.store_add_scaled_inputs3_mixed_aii(1866, A::add_scaled_product(s.ad_value(1858), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1849)), 1.0), 1.0, 1862, 1.0, 1863, 1.0);
                s.store_sub(1867, 1893, 185);
                s.store_add_ad_rhs(1868, 1895, A::add_scaled_value_products(s.ad_value(1840), 1.0, s.ad_value(1842), s.ad_value(1838), 1.0, s.ad_value(1844), s.ad_value(1838), 1.0));
                s.store_sub(1869, 1852, 1835);
                s.store_neg(1870, 1836);
                s.store_sub_from_scalar(1871, 1.0, 1837);
                s.store_add_scaled_products_indices(1872, 1867, 1871, 1.0, 1868, 1870, (-1.0));
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) {
                if (s.v[1872] > 0.0) {
                    s.store_div_from_scalar_offset_input(1873, 1.0, 1872, 1e-25);
                } else {
                    s.store_div_from_scalar_offset_input(1873, 1.0, 1872, (-1e-25));
                }
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) {
                s.copy_ad(1874, 1871);
                s.store_neg(1875, 1868);
                s.store_neg(1876, 1870);
                s.copy_ad(1877, 1867);
                s.store_mul_add_scaled_products_indices_rhs(1878, 1873, 1874, 1866, -1.0, 1875, 1869, -1.0);
                s.store_mul_add_scaled_products_indices_rhs(1879, 1873, 1876, 1866, -1.0, 1877, 1869, -1.0);
                s.store_abs(335, 1878);
            }
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) {
                if (s.v[335] < ((s.v[1879]) as f64).abs()) {
                    s.store_abs(335, 1879);
                } else {
                }
            }
            s.b[1947] = (s.v[335] > 0.1);
            s.v[1947] = if s.b[1947] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) && s.b[1947]) {
                s.store_mul_div_from_scalar_rhs(1878, 1878, 0.1, 335);
                s.store_mul_div_from_scalar_rhs(1879, 1879, 0.1, 335);
            }
            s.b[1948] = (s.v[335] < 1e-12);
            s.v[1948] = if s.b[1948] { 1.0 } else { 0.0 };
            let (assign38070_body197_e46708,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) && s.b[1948]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign38070_body197_e46708;
            if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (s.v[79] == 0.0)) {
                s.store_add(1849, 1849, 1878);
                s.store_add(1852, 1852, 1879);
            }
            let (assign38070_body200_e46747,) = {
    if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
        let assign38070_body200_e46745: f64 = (s.v[97] + 1.0);
        (assign38070_body200_e46745,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign38070_body200_e46747;
        }

    }

    pub(super) fn stamp_transient_block_34(
        s: &mut Scratch,
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
            s.store_add_scaled_inputs3_indices(781, 1907, 1.0, 1849, -1.0, 1847, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 1907);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign38270_e47004,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign38270_e47004;

        let (assign38280_e47015,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38280_e47015;

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) {
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

        let (assign38430_e47198,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) && s.b[1954]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38430_e47198;

        s.b[1955] = (4.0 == 2.0);
        s.v[1955] = if s.b[1955] { 1.0 } else { 0.0 };

        let (assign38450_e47219,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) && (!s.b[1954])) && s.b[1955]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38450_e47219;

        s.b[1956] = (4.0 == 4.0);
        s.v[1956] = if s.b[1956] { 1.0 } else { 0.0 };

        let (assign38470_e47243,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) && (!s.b[1954])) && (!s.b[1955])) && s.b[1956]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38470_e47243;

        s.b[1957] = (4.0 == 8.0);
        s.v[1957] = if s.b[1957] { 1.0 } else { 0.0 };

        let (assign38490_e47270,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) && (!s.b[1954])) && (!s.b[1955])) && (!s.b[1956])) && s.b[1957]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38490_e47270;

        let (assign38500_e47283,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign38500_e47283;

        let mut assign38510_loop_guard: usize = 0;
        while {
            let assign38510_cond_e47297: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign38510_cond_e47297 != 0.0
        } {
            assign38510_loop_guard += 1;
            assert!(assign38510_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) {
                s.store_sqrt(726, 726);
            }
            let (assign38510_body1_e47326,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && s.b[1953]) {
        let assign38510_body1_e47324: f64 = (s.v[719] + 1.0);
        (assign38510_body1_e47324,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38510_body1_e47326;
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) && (!s.b[1953])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1907, 726);
            s.store_div_scaled_product3_indices(334, 1907, 725, 726, 1.0, 770, 1.0);
            s.store_sub(336, 1907, 780);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1952]) {
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1952])) {
            s.store_sub(336, 1849, 1847);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), 1e-15);
            s.store_mul_scaled_sqrt_rhs(1885, 209, -1.0, 338);
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
        }

        let (assign38730_e47629,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign38730_e47629;

        let (assign38740_e47642,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38740_e47642;

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) {
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

        let (assign38850_e47791,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) && s.b[1961]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38850_e47791;

        s.b[1962] = (2.0 == 2.0);
        s.v[1962] = if s.b[1962] { 1.0 } else { 0.0 };

        let (assign38870_e47814,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) && (!s.b[1961])) && s.b[1962]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38870_e47814;

        s.b[1963] = (2.0 == 4.0);
        s.v[1963] = if s.b[1963] { 1.0 } else { 0.0 };

        let (assign38890_e47840,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) && (!s.b[1961])) && (!s.b[1962])) && s.b[1963]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38890_e47840;

        s.b[1964] = (2.0 == 8.0);
        s.v[1964] = if s.b[1964] { 1.0 } else { 0.0 };

        let (assign38910_e47869,) = {
    if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) && (!s.b[1961])) && (!s.b[1962])) && (!s.b[1963])) && s.b[1964]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign38910_e47869;

        let (assign38920_e47884,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign38920_e47884;

        let mut assign38930_loop_guard: usize = 0;
        while {
            let assign38930_cond_e47900: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign38930_cond_e47900 != 0.0
        } {
            assign38930_loop_guard += 1;
            assert!(assign38930_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) {
                s.store_sqrt(726, 726);
            }
            let (assign38930_body1_e47933,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && s.b[1960]) {
        let assign38930_body1_e47931: f64 = (s.v[719] + 1.0);
        (assign38930_body1_e47931,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign38930_body1_e47933;
        }

        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) && (!s.b[1960])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1959]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 339, 726);
            s.store_div_scaled_product3_indices(334, 339, 725, 726, 1.0, 770, 1.0);
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
        }

        let (assign39100_e48193,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign39100_e48193;

    }

    pub(super) fn stamp_transient_block_35(
        s: &mut Scratch,
    ) {
        let (assign39110_e48206,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign39110_e48206;

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) {
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

        let (assign39220_e48355,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) && s.b[1966]) && s.b[1967]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign39220_e48355;

        s.b[1968] = (2.0 == 2.0);
        s.v[1968] = if s.b[1968] { 1.0 } else { 0.0 };

        let (assign39240_e48378,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) && s.b[1966]) && (!s.b[1967])) && s.b[1968]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign39240_e48378;

        s.b[1969] = (2.0 == 4.0);
        s.v[1969] = if s.b[1969] { 1.0 } else { 0.0 };

        let (assign39260_e48404,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) && s.b[1966]) && (!s.b[1967])) && (!s.b[1968])) && s.b[1969]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign39260_e48404;

        s.b[1970] = (2.0 == 8.0);
        s.v[1970] = if s.b[1970] { 1.0 } else { 0.0 };

        let (assign39280_e48433,) = {
    if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) && s.b[1966]) && (!s.b[1967])) && (!s.b[1968])) && (!s.b[1969])) && s.b[1970]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign39280_e48433;

        let (assign39290_e48448,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) && s.b[1966]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign39290_e48448;

        let mut assign39300_loop_guard: usize = 0;
        while {
            let assign39300_cond_e48464: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) && s.b[1966]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign39300_cond_e48464 != 0.0
        } {
            assign39300_loop_guard += 1;
            assert!(assign39300_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) && s.b[1966]) {
                s.store_sqrt(726, 726);
            }
            let (assign39300_body1_e48497,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) && s.b[1966]) {
        let assign39300_body1_e48495: f64 = (s.v[719] + 1.0);
        (assign39300_body1_e48495,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign39300_body1_e48497;
        }

        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) && s.b[1965]) && (!s.b[1966])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
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
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_offset_rhs(336, 335, 658, (-1.0));
            }
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) {
            s.store_offset_mul(337, 336, 335, 1.0);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[1958]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
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

        let (assign39560_e48855,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
        (s.v[1832],)
    } else {
        (s.v[1833],)
    }
};
        s.v[1833] = assign39560_e48855;

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
            s.copy_ad(1848, 790);
            s.store_add_scaled_inputs3_offset_indices(781, 1849, 1.0, 1848, 1.0, 85, -1.0, (-0.01));
            s.store_scaled_add(782, 1849, 1848, (4.0 * 0.01));
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4_indices(1857, 1849, 1.0, 1848, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_add_scaled_inputs3_offset_indices(781, 1857, 1.0, 1883, -1.0, 1851, 1.0, (-0.01));
            s.store_scaled_sub(782, 1883, 1851, (4.0 * 0.01));
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4_indices(1857, 1883, 1.0, 1851, (-1.0), 781, 0.5, 782, 0.5);
            s.store_mul(212, 209, 186);
            s.store_square(213, 212);
            s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::sub(s.ad_value(85), s.ad_value(1883))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
            if (s.v[332] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(332, (10.0 * 2.220446049250313e-16));
            }
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
            s.store_add_ad_rhs(92, 85, A::mul3_scaled_output(s.ad_value(213), s.ad_value(154), A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 0.5));
        }

        let (assign39750_e49195,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign39750_e49195;

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
            s.copy_ad(1850, 1857);
            s.copy_ad(1853, 1848);
        }

        let (assign39780_e49231,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign39780_e49231;

    }

    pub(super) fn stamp_transient_block_36(
        s: &mut Scratch,
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
            }
            let (assign39790_body9_e49393,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign39790_body9_e49393;
            let (assign39790_body10_e49407,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39790_body10_e49407;
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) {
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
            let (assign39790_body21_e49565,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) && s.b[1973]) && s.b[1974]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39790_body21_e49565;
            s.b[1975] = (2.0 == 2.0);
            s.v[1975] = if s.b[1975] { 1.0 } else { 0.0 };
            let (assign39790_body23_e49589,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) && s.b[1973]) && (!s.b[1974])) && s.b[1975]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39790_body23_e49589;
            s.b[1976] = (2.0 == 4.0);
            s.v[1976] = if s.b[1976] { 1.0 } else { 0.0 };
            let (assign39790_body25_e49616,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) && s.b[1973]) && (!s.b[1974])) && (!s.b[1975])) && s.b[1976]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39790_body25_e49616;
            s.b[1977] = (2.0 == 8.0);
            s.v[1977] = if s.b[1977] { 1.0 } else { 0.0 };
            let (assign39790_body27_e49646,) = {
    if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) && s.b[1973]) && (!s.b[1974])) && (!s.b[1975])) && (!s.b[1976])) && s.b[1977]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39790_body27_e49646;
            let (assign39790_body28_e49662,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) && s.b[1973]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign39790_body28_e49662;
            let mut assign39790_body29_loop_guard: usize = 0;
            while {
                let assign39790_body29_cond_e49679: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) && s.b[1973]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign39790_body29_cond_e49679 != 0.0
            } {
                assign39790_body29_loop_guard += 1;
                assert!(assign39790_body29_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) && s.b[1973]) {
                    s.store_sqrt(726, 726);
                }
                let (assign39790_body29_body1_e49714,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) && s.b[1973]) {
        let assign39790_body29_body1_e49712: f64 = (s.v[719] + 1.0);
        (assign39790_body29_body1_e49712,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign39790_body29_body1_e49714;
            }
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1972]) && (!s.b[1973])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
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
            }
            let (assign39790_body45_e49974,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign39790_body45_e49974;
            let (assign39790_body46_e49988,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39790_body46_e49988;
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) {
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
            let (assign39790_body57_e50146,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) && s.b[1979]) && s.b[1980]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39790_body57_e50146;
            s.b[1981] = (2.0 == 2.0);
            s.v[1981] = if s.b[1981] { 1.0 } else { 0.0 };
            let (assign39790_body59_e50170,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) && s.b[1979]) && (!s.b[1980])) && s.b[1981]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39790_body59_e50170;
            s.b[1982] = (2.0 == 4.0);
            s.v[1982] = if s.b[1982] { 1.0 } else { 0.0 };
            let (assign39790_body61_e50197,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) && s.b[1979]) && (!s.b[1980])) && (!s.b[1981])) && s.b[1982]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39790_body61_e50197;
            s.b[1983] = (2.0 == 8.0);
            s.v[1983] = if s.b[1983] { 1.0 } else { 0.0 };
            let (assign39790_body63_e50227,) = {
    if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) && s.b[1979]) && (!s.b[1980])) && (!s.b[1981])) && (!s.b[1982])) && s.b[1983]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39790_body63_e50227;
            let (assign39790_body64_e50243,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) && s.b[1979]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign39790_body64_e50243;
            let mut assign39790_body65_loop_guard: usize = 0;
            while {
                let assign39790_body65_cond_e50260: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) && s.b[1979]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign39790_body65_cond_e50260 != 0.0
            } {
                assign39790_body65_loop_guard += 1;
                assert!(assign39790_body65_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) && s.b[1979]) {
                    s.store_sqrt(726, 726);
                }
                let (assign39790_body65_body1_e50295,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) && s.b[1979]) {
        let assign39790_body65_body1_e50293: f64 = (s.v[719] + 1.0);
        (assign39790_body65_body1_e50293,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign39790_body65_body1_e50295;
            }
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) && (!s.b[1979])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1978]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-8);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);
                s.store_add_offset_lhs(1827, 965, (-1e-8), 780);
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
                s.store_mul_sub_from_scalar_rhs_ad_lhs(1837, A::add_scaled_product(s.ad_value(334), 1.0, A::div(s.ad_value(965), s.ad_value(1827)), s.ad_value(334), (-1.0)), 1.0, 1839);
            }
            s.b[1985] = ((s.v[1835] > (s.v[1848] - s.v[1846])) && (s.v[1846] >= 0.0));
            s.v[1985] = if s.b[1985] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) {
                s.store_add_scaled_inputs3_indices(781, 1835, 1.0, 1848, (-1.0), 1846, 1.0);
                s.store_square(722, 781);
                s.store_square(723, 1846);
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign39790_body90_e50715,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign39790_body90_e50715;
            let (assign39790_body91_e50729,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39790_body91_e50729;
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) {
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
            let (assign39790_body106_e50951,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) && s.b[1986]) && s.b[1987]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39790_body106_e50951;
            s.b[1988] = (4.0 == 2.0);
            s.v[1988] = if s.b[1988] { 1.0 } else { 0.0 };
            let (assign39790_body108_e50975,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) && s.b[1986]) && (!s.b[1987])) && s.b[1988]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39790_body108_e50975;
            s.b[1989] = (4.0 == 4.0);
            s.v[1989] = if s.b[1989] { 1.0 } else { 0.0 };
            let (assign39790_body110_e51002,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) && s.b[1986]) && (!s.b[1987])) && (!s.b[1988])) && s.b[1989]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39790_body110_e51002;
            s.b[1990] = (4.0 == 8.0);
            s.v[1990] = if s.b[1990] { 1.0 } else { 0.0 };
            let (assign39790_body112_e51032,) = {
    if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) && s.b[1986]) && (!s.b[1987])) && (!s.b[1988])) && (!s.b[1989])) && s.b[1990]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39790_body112_e51032;
            let (assign39790_body113_e51048,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) && s.b[1986]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign39790_body113_e51048;
            let mut assign39790_body114_loop_guard: usize = 0;
            while {
                let assign39790_body114_cond_e51065: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) && s.b[1986]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign39790_body114_cond_e51065 != 0.0
            } {
                assign39790_body114_loop_guard += 1;
                assert!(assign39790_body114_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) && s.b[1986]) {
                    s.store_sqrt(726, 726);
                }
                let (assign39790_body114_body1_e51100,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) && s.b[1986]) {
        let assign39790_body114_body1_e51098: f64 = (s.v[719] + 1.0);
        (assign39790_body114_body1_e51098,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign39790_body114_body1_e51100;
            }
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) && (!s.b[1986])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1985]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_mul3_lhs(780, 781, 1846, 726);
                s.store_div_scaled_product3_indices(334, 1846, 725, 726, 1.0, 770, 1.0);
                s.store_add_scaled_inputs3_indices(1835, 1848, 1.0, 1846, (-1.0), 780, 1.0);
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
                s.store_add_scaled_inputs3_indices(335, 1855, 1.0, 1883, (-1.0), 1851, 1.0);
            }
            s.b[1991] = ((s.v[335] < 0.1) && (0.1 >= 0.0));
            s.v[1991] = if s.b[1991] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) {
                s.store_sub_from_scalar(781, 0.1, 335);
                s.store_square(722, 781);
                s.store_scalar(723, (0.1 * 0.1));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign39790_body132_e51389,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign39790_body132_e51389;
            let (assign39790_body133_e51403,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39790_body133_e51403;
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) {
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
            let (assign39790_body144_e51561,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) && s.b[1992]) && s.b[1993]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39790_body144_e51561;
            s.b[1994] = (2.0 == 2.0);
            s.v[1994] = if s.b[1994] { 1.0 } else { 0.0 };
            let (assign39790_body146_e51585,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) && s.b[1992]) && (!s.b[1993])) && s.b[1994]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39790_body146_e51585;
            s.b[1995] = (2.0 == 4.0);
            s.v[1995] = if s.b[1995] { 1.0 } else { 0.0 };
            let (assign39790_body148_e51612,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) && s.b[1992]) && (!s.b[1993])) && (!s.b[1994])) && s.b[1995]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39790_body148_e51612;
            s.b[1996] = (2.0 == 8.0);
            s.v[1996] = if s.b[1996] { 1.0 } else { 0.0 };
            let (assign39790_body150_e51642,) = {
    if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) && s.b[1992]) && (!s.b[1993])) && (!s.b[1994])) && (!s.b[1995])) && s.b[1996]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign39790_body150_e51642;
            let (assign39790_body151_e51658,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) && s.b[1992]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign39790_body151_e51658;
            let mut assign39790_body152_loop_guard: usize = 0;
            while {
                let assign39790_body152_cond_e51675: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) && s.b[1992]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign39790_body152_cond_e51675 != 0.0
            } {
                assign39790_body152_loop_guard += 1;
                assert!(assign39790_body152_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) && s.b[1992]) {
                    s.store_sqrt(726, 726);
                }
                let (assign39790_body152_body1_e51710,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) && s.b[1992]) {
        let assign39790_body152_body1_e51708: f64 = (s.v[719] + 1.0);
        (assign39790_body152_body1_e51708,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign39790_body152_body1_e51710;
            }
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[1991]) && (!s.b[1992])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
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
                s.store_mul_scaled_sqrt_ad_rhs(1859, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
                s.store_mul_ad(1894, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(1859), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
                s.store_neg(1896, 1894);
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (!s.b[1997])) {
                s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1850), s.ad_value(1883)));
                s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1853), s.ad_value(1883)));
                s.store_mul_sqrt_ad_rhs(1859, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1859, 1.0);
                s.store_mul_add_ad_rhs(1894, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
                s.store_mul_add_ad_rhs(1896, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));
            }
            let (assign39790_body176_e52163,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (s.v[79] != 0.0)) {
        let assign39790_body176_e52161: f64 = (150.0 + 1.0);
        (assign39790_body176_e52161,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign39790_body176_e52163;
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (s.v[79] == 0.0)) {
                s.store_add_scaled_inputs3_mixed_aii(1866, A::add_scaled_product(s.ad_value(1859), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1850)), 1.0), 1.0, 1864, 1.0, 1865, 1.0);
                s.store_sub(1867, 1894, 185);
                s.store_add_ad_rhs(1868, 1896, A::add_scaled_value_products(s.ad_value(1841), 1.0, s.ad_value(1843), s.ad_value(1839), 1.0, s.ad_value(1845), s.ad_value(1839), 1.0));
                s.store_sub(1869, 1853, 1835);
                s.store_neg(1870, 1836);
                s.store_sub_from_scalar(1871, 1.0, 1837);
                s.store_add_scaled_products_indices(1872, 1867, 1871, 1.0, 1868, 1870, (-1.0));
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (s.v[79] == 0.0)) {
                if (s.v[1872] > 0.0) {
                    s.store_div_from_scalar_offset_input(1873, 1.0, 1872, 1e-25);
                } else {
                    s.store_div_from_scalar_offset_input(1873, 1.0, 1872, (-1e-25));
                }
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (s.v[79] == 0.0)) {
                s.copy_ad(1874, 1871);
                s.store_neg(1875, 1868);
                s.store_neg(1876, 1870);
                s.copy_ad(1877, 1867);
                s.store_mul_add_scaled_products_indices_rhs(1878, 1873, 1874, 1866, -1.0, 1875, 1869, -1.0);
                s.store_mul_add_scaled_products_indices_rhs(1879, 1873, 1876, 1866, -1.0, 1877, 1869, -1.0);
                s.store_abs(335, 1878);
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (s.v[79] == 0.0)) {
                if (s.v[335] < ((s.v[1879]) as f64).abs()) {
                    s.store_abs(335, 1879);
                } else {
                }
            }
            s.b[1998] = (s.v[335] > 0.1);
            s.v[1998] = if s.b[1998] { 1.0 } else { 0.0 };
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (s.v[79] == 0.0)) && s.b[1998]) {
                s.store_mul_div_from_scalar_rhs(1878, 1878, 0.1, 335);
                s.store_mul_div_from_scalar_rhs(1879, 1879, 0.1, 335);
            }
            s.b[1999] = (s.v[335] < 1e-12);
            s.v[1999] = if s.b[1999] { 1.0 } else { 0.0 };
            let (assign39790_body197_e52543,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (s.v[79] == 0.0)) && s.b[1999]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign39790_body197_e52543;
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (s.v[79] == 0.0)) {
                s.store_add(1850, 1850, 1878);
                s.store_add(1853, 1853, 1879);
            }
            let (assign39790_body200_e52591,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
        let assign39790_body200_e52589: f64 = (s.v[97] + 1.0);
        (assign39790_body200_e52589,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign39790_body200_e52591;
        }

    }

    pub(super) fn stamp_transient_block_37(
        s: &mut Scratch,
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
            s.store_add_scaled_inputs3_indices(781, 1907, 1.0, 1850, -1.0, 1848, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 1907);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign39990_e52896,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign39990_e52896;

        let (assign40000_e52910,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40000_e52910;

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) {
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

        let (assign40150_e53132,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) && s.b[2004]) && s.b[2005]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40150_e53132;

        s.b[2006] = (4.0 == 2.0);
        s.v[2006] = if s.b[2006] { 1.0 } else { 0.0 };

        let (assign40170_e53156,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) && s.b[2004]) && (!s.b[2005])) && s.b[2006]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40170_e53156;

        s.b[2007] = (4.0 == 4.0);
        s.v[2007] = if s.b[2007] { 1.0 } else { 0.0 };

        let (assign40190_e53183,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) && s.b[2004]) && (!s.b[2005])) && (!s.b[2006])) && s.b[2007]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40190_e53183;

        s.b[2008] = (4.0 == 8.0);
        s.v[2008] = if s.b[2008] { 1.0 } else { 0.0 };

        let (assign40210_e53213,) = {
    if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) && s.b[2004]) && (!s.b[2005])) && (!s.b[2006])) && (!s.b[2007])) && s.b[2008]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40210_e53213;

        let (assign40220_e53229,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) && s.b[2004]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign40220_e53229;

        let mut assign40230_loop_guard: usize = 0;
        while {
            let assign40230_cond_e53246: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) && s.b[2004]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign40230_cond_e53246 != 0.0
        } {
            assign40230_loop_guard += 1;
            assert!(assign40230_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) && s.b[2004]) {
                s.store_sqrt(726, 726);
            }
            let (assign40230_body1_e53281,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) && s.b[2004]) {
        let assign40230_body1_e53279: f64 = (s.v[719] + 1.0);
        (assign40230_body1_e53279,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign40230_body1_e53281;
        }

        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) && (!s.b[2004])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1907, 726);
            s.store_div_scaled_product3_indices(334, 1907, 725, 726, 1.0, 770, 1.0);
            s.store_sub(336, 1907, 780);
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && s.b[2003]) {
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) && (!s.b[2003])) {
            s.store_sub(336, 1850, 1848);
            s.store_scalar(334, 1.0);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[1971])) {
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), 1e-15);
            s.store_mul_scaled_sqrt_rhs(1886, 209, -1.0, 338);
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.copy_ad(87, 1849);
            s.copy_ad(91, 1850);
            s.store_sub(94, 1850, 1849);
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 1.0 / ((p.p263 * 0.1))));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(110, (p.p263 * 0.1), 782);
            s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);
        }

        s.b[2009] = ((s.v[110] < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));
        s.v[2009] = if s.b[2009] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) {
            s.store_sub_from_scalar(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 110);
            s.store_square(722, 781);
            s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign40490_e53735,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign40490_e53735;

        let (assign40500_e53746,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40500_e53746;

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) {
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

        let (assign40610_e53877,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) && s.b[2010]) && s.b[2011]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40610_e53877;

        s.b[2012] = (2.0 == 2.0);
        s.v[2012] = if s.b[2012] { 1.0 } else { 0.0 };

        let (assign40630_e53898,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) && s.b[2010]) && (!s.b[2011])) && s.b[2012]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40630_e53898;

        s.b[2013] = (2.0 == 4.0);
        s.v[2013] = if s.b[2013] { 1.0 } else { 0.0 };

        let (assign40650_e53922,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) && s.b[2010]) && (!s.b[2011])) && (!s.b[2012])) && s.b[2013]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40650_e53922;

        s.b[2014] = (2.0 == 8.0);
        s.v[2014] = if s.b[2014] { 1.0 } else { 0.0 };

        let (assign40670_e53949,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) && s.b[2010]) && (!s.b[2011])) && (!s.b[2012])) && (!s.b[2013])) && s.b[2014]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40670_e53949;

        let (assign40680_e53962,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) && s.b[2010]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign40680_e53962;

        let mut assign40690_loop_guard: usize = 0;
        while {
            let assign40690_cond_e53976: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) && s.b[2010]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign40690_cond_e53976 != 0.0
        } {
            assign40690_loop_guard += 1;
            assert!(assign40690_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) && s.b[2010]) {
                s.store_sqrt(726, 726);
            }
            let (assign40690_body1_e54005,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) && s.b[2010]) {
        let assign40690_body1_e54003: f64 = (s.v[719] + 1.0);
        (assign40690_body1_e54003,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign40690_body1_e54005;
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2009]) && (!s.b[2010])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
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
            s.store_add_scaled_inputs3_indices(781, 1907, 1.0, 109, -1.0, 1847, 1.0);
            s.store_square(722, 781);
            s.store_square(723, 1907);
            s.store_scalar(724, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_38(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) {
            s.store_scalar(725, 1.0);
        }

        let (assign40850_e54231,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign40850_e54231;

        let (assign40860_e54242,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign40860_e54242;

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) {
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

        let (assign41010_e54425,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) && s.b[2016]) && s.b[2017]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41010_e54425;

        s.b[2018] = (4.0 == 2.0);
        s.v[2018] = if s.b[2018] { 1.0 } else { 0.0 };

        let (assign41030_e54446,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) && s.b[2016]) && (!s.b[2017])) && s.b[2018]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41030_e54446;

        s.b[2019] = (4.0 == 4.0);
        s.v[2019] = if s.b[2019] { 1.0 } else { 0.0 };

        let (assign41050_e54470,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) && s.b[2016]) && (!s.b[2017])) && (!s.b[2018])) && s.b[2019]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41050_e54470;

        s.b[2020] = (4.0 == 8.0);
        s.v[2020] = if s.b[2020] { 1.0 } else { 0.0 };

        let (assign41070_e54497,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) && s.b[2016]) && (!s.b[2017])) && (!s.b[2018])) && (!s.b[2019])) && s.b[2020]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41070_e54497;

        let (assign41080_e54510,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) && s.b[2016]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign41080_e54510;

        let mut assign41090_loop_guard: usize = 0;
        while {
            let assign41090_cond_e54524: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) && s.b[2016]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign41090_cond_e54524 != 0.0
        } {
            assign41090_loop_guard += 1;
            assert!(assign41090_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) && s.b[2016]) {
                s.store_sqrt(726, 726);
            }
            let (assign41090_body1_e54553,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) && s.b[2016]) {
        let assign41090_body1_e54551: f64 = (s.v[719] + 1.0);
        (assign41090_body1_e54551,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign41090_body1_e54553;
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) && (!s.b[2016])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1907, 726);
            s.store_div_scaled_product3_indices(334, 1907, 725, 726, 1.0, 770, 1.0);
            s.store_sub(336, 1907, 780);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2015]) {
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2015])) {
            s.store_sub(336, 109, 1847);
            s.store_scalar(334, 1.0);
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), 1e-15);
            s.store_mul_scaled_sqrt_rhs(1887, 209, -1.0, 338);
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
        }

        let (assign41280_e54823,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign41280_e54823;

        let (assign41290_e54837,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41290_e54837;

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
        }

        let (assign41320_e54879,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign41320_e54879;

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
            }
            let (assign41330_body2_e54942,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) {
        let assign41330_body2_e54940: f64 = (s.v[719] + 1.0);
        (assign41330_body2_e54940,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign41330_body2_e54942;
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) {
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2028] = ((((s.v[1910] == 1.0) || (s.v[1910] == 2.0)) || (s.v[1910] == 4.0)) || (s.v[1910] == 8.0));
        s.v[2028] = if s.b[2028] { 1.0 } else { 0.0 };

        s.b[2029] = (s.v[1910] == 1.0);
        s.v[2029] = if s.b[2029] { 1.0 } else { 0.0 };

        let (assign41380_e55008,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) && s.b[2028]) && s.b[2029]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41380_e55008;

        s.b[2030] = (s.v[1910] == 2.0);
        s.v[2030] = if s.b[2030] { 1.0 } else { 0.0 };

        let (assign41400_e55032,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) && s.b[2028]) && (!s.b[2029])) && s.b[2030]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41400_e55032;

        s.b[2031] = (s.v[1910] == 4.0);
        s.v[2031] = if s.b[2031] { 1.0 } else { 0.0 };

        let (assign41420_e55059,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) && s.b[2028]) && (!s.b[2029])) && (!s.b[2030])) && s.b[2031]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41420_e55059;

        s.b[2032] = (s.v[1910] == 8.0);
        s.v[2032] = if s.b[2032] { 1.0 } else { 0.0 };

        let (assign41440_e55089,) = {
    if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) && s.b[2028]) && (!s.b[2029])) && (!s.b[2030])) && (!s.b[2031])) && s.b[2032]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41440_e55089;

        let (assign41450_e55105,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) && s.b[2028]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign41450_e55105;

        let mut assign41460_loop_guard: usize = 0;
        while {
            let assign41460_cond_e55122: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) && s.b[2028]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign41460_cond_e55122 != 0.0
        } {
            assign41460_loop_guard += 1;
            assert!(assign41460_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) && s.b[2028]) {
                s.store_sqrt(726, 726);
            }
            let (assign41460_body1_e55157,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) && s.b[2028]) {
        let assign41460_body1_e55155: f64 = (s.v[719] + 1.0);
        (assign41460_body1_e55155,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign41460_body1_e55157;
        }

        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2027]) && (!s.b[2028])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_pow_ad(726, s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(1910), 2.0)));
            }
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
            s.store_add_scaled_inputs3_offset_indices(1912, 791, 1.0, 85, (-1.0), 1908, 1.0, (-(s.v[462] - p.p392)));
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
        }

        let (assign41630_e55436,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign41630_e55436;

        let (assign41640_e55450,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41640_e55450;

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
        }

        let (assign41670_e55492,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign41670_e55492;

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
            }
            let (assign41680_body2_e55555,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) {
        let assign41680_body2_e55553: f64 = (s.v[719] + 1.0);
        (assign41680_body2_e55553,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign41680_body2_e55555;
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) {
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2034] = ((((s.v[1910] == 1.0) || (s.v[1910] == 2.0)) || (s.v[1910] == 4.0)) || (s.v[1910] == 8.0));
        s.v[2034] = if s.b[2034] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_39(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2035] = (s.v[1910] == 1.0);
        s.v[2035] = if s.b[2035] { 1.0 } else { 0.0 };

        let (assign41730_e55621,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) && s.b[2034]) && s.b[2035]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41730_e55621;

        s.b[2036] = (s.v[1910] == 2.0);
        s.v[2036] = if s.b[2036] { 1.0 } else { 0.0 };

        let (assign41750_e55645,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) && s.b[2034]) && (!s.b[2035])) && s.b[2036]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41750_e55645;

        s.b[2037] = (s.v[1910] == 4.0);
        s.v[2037] = if s.b[2037] { 1.0 } else { 0.0 };

        let (assign41770_e55672,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) && s.b[2034]) && (!s.b[2035])) && (!s.b[2036])) && s.b[2037]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41770_e55672;

        s.b[2038] = (s.v[1910] == 8.0);
        s.v[2038] = if s.b[2038] { 1.0 } else { 0.0 };

        let (assign41790_e55702,) = {
    if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) && s.b[2034]) && (!s.b[2035])) && (!s.b[2036])) && (!s.b[2037])) && s.b[2038]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign41790_e55702;

        let (assign41800_e55718,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) && s.b[2034]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign41800_e55718;

        let mut assign41810_loop_guard: usize = 0;
        while {
            let assign41810_cond_e55735: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) && s.b[2034]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign41810_cond_e55735 != 0.0
        } {
            assign41810_loop_guard += 1;
            assert!(assign41810_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) && s.b[2034]) {
                s.store_sqrt(726, 726);
            }
            let (assign41810_body1_e55770,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) && s.b[2034]) {
        let assign41810_body1_e55768: f64 = (s.v[719] + 1.0);
        (assign41810_body1_e55768,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign41810_body1_e55770;
        }

        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) && (!s.b[2034])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_pow_ad(726, s.ad_value(726), A::div_from_scalar(1.0, A::scale(s.ad_value(1910), 2.0)));
            }
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1909, 726);
            s.store_div_scaled_product3_indices(334, 1909, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs(1911, 1909, -1.0, 780, 1.0);
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2033]) {
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (!s.b[2033])) {
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (!s.b[2033])) {
            s.store_scalar(334, 1.0);
        }

        let (assign41900_e55926,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign41900_e55926;

        let (assign41910_e55938,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign41910_e55938;

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
                s.store_mul_scaled_sqrt_ad_rhs(2024, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));
                s.store_mul_ad(2025, A::div_scaled_product(s.ad_value(209), s.ad_value(209), 0.5, s.ad_value(2024), 1.0), A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0));
            }
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (!s.b[2039])) {
                s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(983), s.ad_value(1883)));
                s.store_exp_mul_scaled_lhs_indices(338, 154, 1.0, 1883);
                s.store_mul_sqrt_ad_rhs(2024, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));
                s.store_div_scaled_product_indices(339, 209, 209, 0.5, 2024, 1.0);
                s.store_mul_add_ad_rhs(2025, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));
            }
            let (assign41920_body10_e56167,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (s.v[79] != 0.0)) {
        let assign41920_body10_e56165: f64 = (150.0 + 1.0);
        (assign41920_body10_e56165,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign41920_body10_e56167;
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (s.v[79] == 0.0)) {
                s.store_add_scaled_product_right_sub(1866, 2024, 1.0, 185, 1911, 983, 1.0);
                s.store_sub(1867, 2025, 185);
                s.store_div_scaled_inputs_indices(1878, 1866, -1.0, 1867, 1.0);
            }
            s.b[2040] = (((s.v[1878]) as f64).abs() < (1e-10 * 100.0));
            s.v[2040] = if s.b[2040] { 1.0 } else { 0.0 };
            let (assign41920_body15_e56247,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (s.v[79] == 0.0)) && s.b[2040]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign41920_body15_e56247;
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
            let (assign41920_body21_e56329,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) {
        let assign41920_body21_e56327: f64 = (s.v[97] + 1.0);
        (assign41920_body21_e56327,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign41920_body21_e56329;
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) {
            s.store_neg(983, 983);
            s.store_mul3_affine_lhs(2022, 1901, 1830, (0.5 * 9662367879.197212), 0.0, 1830);
            s.store_scaled_sqrt_mul_scaled_lhs(334, 154, 2.0, 2022, p.p394);
            s.store_scaled_add_ad(335, A::exp(s.ad_value(334)), A::exp_scaled_input(s.ad_value(334), -1.0), 0.5);
            s.store_div_ln_lhs(2023, 335, 2022);
            s.store_mul(332, 2023, 983);
            s.store_exp_mul_scaled_lhs_indices(334, 2023, -1.0, 2022);
        }

        s.b[2044] = (((s.v[332]) as f64).abs() > 1e-8);
        s.v[2044] = if s.b[2044] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2044]) {
            s.store_mul_exp_lhs(335, 332, 334);
            s.store_sub(336, 335, 334);
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && (!s.b[2044])) {
            s.store_mul_offset_lhs(335, 332, 1.0, 334);
            s.store_mul_ad_product_lhs(336, s.ad_value(332), A::scale_offset(s.ad_value(332), 0.5, 1.0), 334);
        }

        s.b[2045] = (((s.v[336]) as f64).abs() > 1e-8);
        s.v[2045] = if s.b[2045] { 1.0 } else { 0.0 };

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2026])) && s.b[2045]) {
            s.store_div_ln_offset_lhs(2021, 336, 1.0, 2023);
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
            if (s.v[981] > s.v[1830]) {
                s.copy_ad(981, 1830);
            } else {
            }
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
            s.store_neg_add(1890, 1885, 1886);
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
            if (s.v[248] < 0.0) {
                s.store_scalar(248, 0.0);
            } else {
            }
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
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p160 - 1.0));
            }
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_mul(342, 339, 251);
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            if (s.v[251] == 0.0) {
                s.store_scalar(341, 0.0);
            } else {
                s.store_pow_offset_rhs(341, 251, 624, (-1.0));
            }
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
            s.store_div_scaled_inputs_indices(336, 257, 0.2, 254, 1.0);
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
            if (s.v[335] == 0.0) {
                s.store_scalar(337, 0.0);
            } else {
                s.store_powf(337, 335, (p.p178 - 1.0));
            }
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

    }

    pub(super) fn stamp_transient_block_40(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && (!s.b[2051])) && (!s.b[2052])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_powf(340, 338, (((-1.0) / p.p178) - 1.0));
            }
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
        }

        let (assign42770_e57465,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign42770_e57465;

        let (assign42780_e57478,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign42780_e57478;

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) {
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

        let (assign42890_e57627,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) && s.b[2055]) && s.b[2056]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign42890_e57627;

        s.b[2057] = (2.0 == 2.0);
        s.v[2057] = if s.b[2057] { 1.0 } else { 0.0 };

        let (assign42910_e57650,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) && s.b[2055]) && (!s.b[2056])) && s.b[2057]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign42910_e57650;

        s.b[2058] = (2.0 == 4.0);
        s.v[2058] = if s.b[2058] { 1.0 } else { 0.0 };

        let (assign42930_e57676,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) && s.b[2055]) && (!s.b[2056])) && (!s.b[2057])) && s.b[2058]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign42930_e57676;

        s.b[2059] = (2.0 == 8.0);
        s.v[2059] = if s.b[2059] { 1.0 } else { 0.0 };

        let (assign42950_e57705,) = {
    if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) && s.b[2055]) && (!s.b[2056])) && (!s.b[2057])) && (!s.b[2058])) && s.b[2059]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign42950_e57705;

        let (assign42960_e57720,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) && s.b[2055]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign42960_e57720;

        let mut assign42970_loop_guard: usize = 0;
        while {
            let assign42970_cond_e57736: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) && s.b[2055]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign42970_cond_e57736 != 0.0
        } {
            assign42970_loop_guard += 1;
            assert!(assign42970_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) && s.b[2055]) {
                s.store_sqrt(726, 726);
            }
            let (assign42970_body1_e57769,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) && s.b[2055]) {
        let assign42970_body1_e57767: f64 = (s.v[719] + 1.0);
        (assign42970_body1_e57767,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign42970_body1_e57769;
        }

        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) && (!s.b[2055])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2054]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 339, 726);
            s.store_div_scaled_product3_indices(334, 339, 725, 726, 1.0, 770, 1.0);
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
            s.store_add_scaled_inputs3_indices(781, 972, 1.0, 1906, 1.0, 344, -1.0);
            s.store_square(722, 781);
            s.store_square(723, 1906);
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign43140_e58029,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign43140_e58029;

        let (assign43150_e58042,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43150_e58042;

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) {
            s.store_scalar(770, 0.0);
            s.store_scalar(726, 0.0);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_mul(724, 724, 722);
            s.store_mul(725, 725, 723);
            s.store_add(770, 724, 725);
            s.copy_ad(726, 770);
        }

        s.b[2061] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));
        s.v[2061] = if s.b[2061] { 1.0 } else { 0.0 };

        s.b[2062] = (2.0 == 1.0);
        s.v[2062] = if s.b[2062] { 1.0 } else { 0.0 };

        let (assign43260_e58191,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) && s.b[2061]) && s.b[2062]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43260_e58191;

        s.b[2063] = (2.0 == 2.0);
        s.v[2063] = if s.b[2063] { 1.0 } else { 0.0 };

        let (assign43280_e58214,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) && s.b[2061]) && (!s.b[2062])) && s.b[2063]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43280_e58214;

        s.b[2064] = (2.0 == 4.0);
        s.v[2064] = if s.b[2064] { 1.0 } else { 0.0 };

        let (assign43300_e58240,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) && s.b[2061]) && (!s.b[2062])) && (!s.b[2063])) && s.b[2064]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43300_e58240;

        s.b[2065] = (2.0 == 8.0);
        s.v[2065] = if s.b[2065] { 1.0 } else { 0.0 };

        let (assign43320_e58269,) = {
    if ((((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) && s.b[2061]) && (!s.b[2062])) && (!s.b[2063])) && (!s.b[2064])) && s.b[2065]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43320_e58269;

        let (assign43330_e58284,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) && s.b[2061]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign43330_e58284;

        let mut assign43340_loop_guard: usize = 0;
        while {
            let assign43340_cond_e58300: f64 = if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) && s.b[2061]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign43340_cond_e58300 != 0.0
        } {
            assign43340_loop_guard += 1;
            assert!(assign43340_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) && s.b[2061]) {
                s.store_sqrt(726, 726);
            }
            let (assign43340_body1_e58333,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) && s.b[2061]) {
        let assign43340_body1_e58331: f64 = (s.v[719] + 1.0);
        (assign43340_body1_e58331,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign43340_body1_e58333;
        }

        if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) && (!s.b[2061])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) && s.b[2060]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 1906, 726);
            s.store_div_scaled_product3_indices(334, 1906, 725, 726, 1.0, 770, 1.0);
            s.store_add_scaled_inputs3_indices(344, 972, 1.0, 1906, 1.0, 780, -1.0);
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
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, (p.p383 - 1.0));
            }
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) {
            s.store_offset_mul(337, 336, 335, 1.0);
        }

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2053]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, ((1.0 / p.p383) - 1.0));
            }
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
        }

        let (assign43560_e58659,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign43560_e58659;

    }

    pub(super) fn stamp_transient_block_41(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (assign43570_e58670,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43570_e58670;

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) {
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

        let (assign43680_e58801,) = {
    if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) && s.b[2067]) && s.b[2068]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43680_e58801;

        s.b[2069] = (2.0 == 2.0);
        s.v[2069] = if s.b[2069] { 1.0 } else { 0.0 };

        let (assign43700_e58822,) = {
    if (((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) && s.b[2067]) && (!s.b[2068])) && s.b[2069]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43700_e58822;

        s.b[2070] = (2.0 == 4.0);
        s.v[2070] = if s.b[2070] { 1.0 } else { 0.0 };

        let (assign43720_e58846,) = {
    if ((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) && s.b[2067]) && (!s.b[2068])) && (!s.b[2069])) && s.b[2070]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43720_e58846;

        s.b[2071] = (2.0 == 8.0);
        s.v[2071] = if s.b[2071] { 1.0 } else { 0.0 };

        let (assign43740_e58873,) = {
    if (((((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) && s.b[2067]) && (!s.b[2068])) && (!s.b[2069])) && (!s.b[2070])) && s.b[2071]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign43740_e58873;

        let (assign43750_e58886,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) && s.b[2067]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign43750_e58886;

        let mut assign43760_loop_guard: usize = 0;
        while {
            let assign43760_cond_e58900: f64 = if ((((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) && s.b[2067]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign43760_cond_e58900 != 0.0
        } {
            assign43760_loop_guard += 1;
            assert!(assign43760_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) && s.b[2067]) {
                s.store_sqrt(726, 726);
            }
            let (assign43760_body1_e58929,) = {
    if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) && s.b[2067]) {
        let assign43760_body1_e58927: f64 = (s.v[719] + 1.0);
        (assign43760_body1_e58927,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign43760_body1_e58929;
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2066]) && (!s.b[2067])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
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
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p353 - 1.0));
            }
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_mul(342, 339, 251);
            s.store_offset(336, 966, 1e-25);
            s.store_add_ad(335, A::div_from_scalar(1.0, s.ad_value(336)), A::div(s.ad_value(342), s.ad_value(970)));
            s.store_div_from_scalar(254, 1.0, 335);
            s.store_scale(254, 254, 0.0001);
            s.store_div(1881, 989, 170);
            s.store_square(781, 989);
            s.store_scalar(782, ((0.1) as f64).powf(2.0));
            s.store_sub_ad(335, A::powf(A::add(s.ad_value(781), s.ad_value(782)), (1.0 / 2.0)), A::powf(s.ad_value(782), (1.0 / 2.0)));
            s.store_div(335, 335, 170);
            s.store_div_scaled_product_indices(335, 254, 335, 1.0, 973, 1.0);
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p.p378);
            }
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_offset(337, 336, 1.0);
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, (1.0 / p.p378));
            }
        }

        if (s.b[1439] && (s.b[1441] && (!s.b[1440]))) {
            s.store_div(985, 254, 338);
            s.store_scaled_mul(991, 990, 964, (-1.6021918e-19));
            s.store_mul3_affine_lhs(987, 991, 985, (-s.v[632]), 0.0, 1881);
            s.store_div_scaled_inputs_indices(115, 155, s.v[632], 170, 1.0);
            s.store_mul3_lhs(986, 115, 248, 984);
            s.store_add(135, 986, 987);
            s.copy_ad(790, 349);
        }

        s.b[2072] = (p.p283 != 0.0);
        s.v[2072] = if s.b[2072] { 1.0 } else { 0.0 };

        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2072]) {
            s.store_scaled_sub(335, 790, 94, 0.5);
            s.store_scale(781, 335, (2.0 * 100.0));
            s.store_offset_mul_offset_rhs_ad_rhs(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);
            s.store_offset_mul_offset_rhs_ad_rhs(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));
            s.store_div_from_scalar(340, 0.01, 782);
            s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);
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
            s.store_add_scaled_inputs3_indices(339, 1849, 1.0, 340, 1.0, 1434, -1.0);
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
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2076]) && s.b[2077]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(336, 338, 1.0, 781, 0.5, 782, 0.5);
            s.store_scale(337, 338, (p.p296 + 1.0));
            s.store_offset_sub(781, 337, 336, (-(0.01 * 0.01)));
            s.store_scale(782, 337, (4.0 * (0.01 * 0.01)));
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2076]) && s.b[2077]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2076]) && s.b[2077]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(341, 337, 1.0, 781, (-0.5), 782, (-0.5));
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
            s.store_powf_offset_input(335, 369, 1e-12, p.p297);
        }

    }

    pub(super) fn stamp_transient_block_42(
        s: &mut Scratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2076]) {
            s.store_powf_offset_input(343, 369, 1e-12, p.p299);
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
            s.store_add_scaled_inputs4_indices(131, 1860, (-0.5), 1861, (-0.5), 1863, (-0.5), 1865, (-0.5));
            s.store_scaled_add_ad_lhs(133, A::add(A::add_scaled_inputs4(s.ad_value(1888), 1.0, s.ad_value(1889), 1.0, s.ad_value(1891), 1.0, s.ad_value(1892), 1.0), s.ad_value(1862)), 1864, (-0.5));
            s.store_scalar(247, 0.5);
            s.store_scaled_add(978, 1888, 1889, (-0.5));
            s.store_neg(238, 1888);
            s.copy_ad(255, 1882);
        }

        s.b[2080] = ((s.v[238] < 1e-25) || (s.v[133] < 1e-25));
        s.v[2080] = if s.b[2080] { 1.0 } else { 0.0 };

        let (assign44850_e60520,) = {
    if ((s.b[1439] && (s.b[1441] && (!s.b[1440]))) && s.b[2080]) {
        (1.0,)
    } else {
        (s.v[78],)
    }
};
        s.v[78] = assign44850_e60520;

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.copy_ad(2087, 960);
            s.store_scale(2129, 964, 1.6021918e-19);
            s.store_scale(2110, 964, (1.6021918e-19 * 1.034943e-10));
            s.store_div_from_scalar(2132, (2.0 * 1.034943e-10), 2129);
            s.store_div(2126, 964, 622);
            s.store_div_from_scalar_offset_input(2125, 1.0, 2126, 1.0);
            s.store_div_square_rhs(2130, 2110, 185);
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
        }

        let (assign45020_e60755,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign45020_e60755;

        let (assign45030_e60768,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45030_e60768;

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) {
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

        let (assign45140_e60917,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) && s.b[2169]) && s.b[2170]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45140_e60917;

        s.b[2171] = (2.0 == 2.0);
        s.v[2171] = if s.b[2171] { 1.0 } else { 0.0 };

        let (assign45160_e60940,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) && s.b[2169]) && (!s.b[2170])) && s.b[2171]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45160_e60940;

        s.b[2172] = (2.0 == 4.0);
        s.v[2172] = if s.b[2172] { 1.0 } else { 0.0 };

        let (assign45180_e60966,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) && s.b[2169]) && (!s.b[2170])) && (!s.b[2171])) && s.b[2172]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45180_e60966;

        s.b[2173] = (2.0 == 8.0);
        s.v[2173] = if s.b[2173] { 1.0 } else { 0.0 };

        let (assign45200_e60995,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) && s.b[2169]) && (!s.b[2170])) && (!s.b[2171])) && (!s.b[2172])) && s.b[2173]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45200_e60995;

        let (assign45210_e61010,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) && s.b[2169]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign45210_e61010;

        let mut assign45220_loop_guard: usize = 0;
        while {
            let assign45220_cond_e61026: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) && s.b[2169]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign45220_cond_e61026 != 0.0
        } {
            assign45220_loop_guard += 1;
            assert!(assign45220_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) && s.b[2169]) {
                s.store_sqrt(726, 726);
            }
            let (assign45220_body1_e61059,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) && s.b[2169]) {
        let assign45220_body1_e61057: f64 = (s.v[719] + 1.0);
        (assign45220_body1_e61057,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign45220_body1_e61059;
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2168]) && (!s.b[2169])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
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
            s.store_add_scaled_inputs3_offset_indices(85, 791, 1.0, 120, 1.0, 182, (-1.0), (-s.v[160]));
            s.store_offset_sub_from_scalar_ad(781, 0.3, s.ad_value(85), (-0.01));
            s.store_scalar(782, ((4.0 * 0.3) * 0.01));
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_offset_add_scaled_inputs_indices(2092, 781, (-0.5), 782, (-0.5), 0.3);
            s.store_add_scaled_inputs3_offset_indices(781, 2092, 1.0, 2113, -1.0, 2087, 1.0, (-0.01));
            s.store_scaled_sub(782, 2113, 2087, (4.0 * 0.01));
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4_indices(2092, 2113, 1.0, 2087, (-1.0), 781, 0.5, 782, 0.5);
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
            s.store_scalar(723, (0.001 * 0.001));
            s.store_scalar(724, 1.0);
            s.store_scalar(725, 1.0);
        }

        let (assign45620_e61650,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign45620_e61650;

        let (assign45630_e61663,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45630_e61663;

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) {
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

        let (assign45740_e61812,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) && s.b[2175]) && s.b[2176]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45740_e61812;

        s.b[2177] = (2.0 == 2.0);
        s.v[2177] = if s.b[2177] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_43(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (assign45760_e61835,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) && s.b[2175]) && (!s.b[2176])) && s.b[2177]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45760_e61835;

        s.b[2178] = (2.0 == 4.0);
        s.v[2178] = if s.b[2178] { 1.0 } else { 0.0 };

        let (assign45780_e61861,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) && s.b[2175]) && (!s.b[2176])) && (!s.b[2177])) && s.b[2178]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45780_e61861;

        s.b[2179] = (2.0 == 8.0);
        s.v[2179] = if s.b[2179] { 1.0 } else { 0.0 };

        let (assign45800_e61890,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) && s.b[2175]) && (!s.b[2176])) && (!s.b[2177])) && (!s.b[2178])) && s.b[2179]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45800_e61890;

        let (assign45810_e61905,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) && s.b[2175]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign45810_e61905;

        let mut assign45820_loop_guard: usize = 0;
        while {
            let assign45820_cond_e61921: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) && s.b[2175]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign45820_cond_e61921 != 0.0
        } {
            assign45820_loop_guard += 1;
            assert!(assign45820_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) && s.b[2175]) {
                s.store_sqrt(726, 726);
            }
            let (assign45820_body1_e61954,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) && s.b[2175]) {
        let assign45820_body1_e61952: f64 = (s.v[719] + 1.0);
        (assign45820_body1_e61952,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign45820_body1_e61954;
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2174]) && (!s.b[2175])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
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
        }

        let (assign45980_e62202,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign45980_e62202;

        let (assign45990_e62215,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign45990_e62215;

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) {
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

        let (assign46100_e62364,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) && s.b[2181]) && s.b[2182]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46100_e62364;

        s.b[2183] = (2.0 == 2.0);
        s.v[2183] = if s.b[2183] { 1.0 } else { 0.0 };

        let (assign46120_e62387,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) && s.b[2181]) && (!s.b[2182])) && s.b[2183]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46120_e62387;

        s.b[2184] = (2.0 == 4.0);
        s.v[2184] = if s.b[2184] { 1.0 } else { 0.0 };

        let (assign46140_e62413,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) && s.b[2181]) && (!s.b[2182])) && (!s.b[2183])) && s.b[2184]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46140_e62413;

        s.b[2185] = (2.0 == 8.0);
        s.v[2185] = if s.b[2185] { 1.0 } else { 0.0 };

        let (assign46160_e62442,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) && s.b[2181]) && (!s.b[2182])) && (!s.b[2183])) && (!s.b[2184])) && s.b[2185]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46160_e62442;

        let (assign46170_e62457,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) && s.b[2181]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign46170_e62457;

        let mut assign46180_loop_guard: usize = 0;
        while {
            let assign46180_cond_e62473: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) && s.b[2181]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign46180_cond_e62473 != 0.0
        } {
            assign46180_loop_guard += 1;
            assert!(assign46180_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) && s.b[2181]) {
                s.store_sqrt(726, 726);
            }
            let (assign46180_body1_e62506,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) && s.b[2181]) {
        let assign46180_body1_e62504: f64 = (s.v[719] + 1.0);
        (assign46180_body1_e62504,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign46180_body1_e62506;
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2180]) && (!s.b[2181])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
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
            s.store_add_scaled_inputs3_indices(781, 335, 1.0, 965, (-0.1), 336, -1.0);
            s.store_scaled_mul(782, 965, 336, (0.1 * 4.0));
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2186]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2186]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(335, 965, 0.1, 781, 0.5, 782, 0.5);
            s.store_add_scaled_inputs3_indices(781, 965, 2.0, 335, (-1.0), 336, -1.0);
            s.store_scaled_mul(782, 965, 336, (2.0 * 4.0));
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2186]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2186]) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs3_indices(965, 965, 2.0, 781, (-0.5), 782, (-0.5));
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
        }

        let (assign46520_e63066,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign46520_e63066;

        let (assign46530_e63079,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46530_e63079;

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) {
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

        let (assign46620_e63198,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) && s.b[2188]) && s.b[2189]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46620_e63198;

        s.b[2190] = (1.0 == 2.0);
        s.v[2190] = if s.b[2190] { 1.0 } else { 0.0 };

        let (assign46640_e63221,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) && s.b[2188]) && (!s.b[2189])) && s.b[2190]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46640_e63221;

        s.b[2191] = (1.0 == 4.0);
        s.v[2191] = if s.b[2191] { 1.0 } else { 0.0 };

        let (assign46660_e63247,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) && s.b[2188]) && (!s.b[2189])) && (!s.b[2190])) && s.b[2191]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46660_e63247;

        s.b[2192] = (1.0 == 8.0);
        s.v[2192] = if s.b[2192] { 1.0 } else { 0.0 };

        let (assign46680_e63276,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) && s.b[2188]) && (!s.b[2189])) && (!s.b[2190])) && (!s.b[2191])) && s.b[2192]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46680_e63276;

        let (assign46690_e63291,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) && s.b[2188]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign46690_e63291;

    }

    pub(super) fn stamp_transient_block_44(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let mut assign46700_loop_guard: usize = 0;
        while {
            let assign46700_cond_e63307: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) && s.b[2188]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign46700_cond_e63307 != 0.0
        } {
            assign46700_loop_guard += 1;
            assert!(assign46700_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) && s.b[2188]) {
                s.store_sqrt(726, 726);
            }
            let (assign46700_body1_e63340,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) && s.b[2188]) {
        let assign46700_body1_e63338: f64 = (s.v[719] + 1.0);
        (assign46700_body1_e63338,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign46700_body1_e63340;
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) && (!s.b[2188])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2187]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_scaled_mul(780, 781, 726, (p.p344 * 0.1));
            s.store_div_scaled_product_indices(334, 725, 726, (p.p344 * 0.1), 770, 1.0);
            s.store_sub_from_scalar(2144, (p.p344 + (p.p344 * 0.1)), 780);
        }

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
        }

        let (assign46850_e63587,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign46850_e63587;

        let (assign46860_e63600,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46860_e63600;

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) {
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

        let (assign46950_e63719,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) && s.b[2194]) && s.b[2195]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46950_e63719;

        s.b[2196] = (1.0 == 2.0);
        s.v[2196] = if s.b[2196] { 1.0 } else { 0.0 };

        let (assign46970_e63742,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) && s.b[2194]) && (!s.b[2195])) && s.b[2196]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46970_e63742;

        s.b[2197] = (1.0 == 4.0);
        s.v[2197] = if s.b[2197] { 1.0 } else { 0.0 };

        let (assign46990_e63768,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) && s.b[2194]) && (!s.b[2195])) && (!s.b[2196])) && s.b[2197]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign46990_e63768;

        s.b[2198] = (1.0 == 8.0);
        s.v[2198] = if s.b[2198] { 1.0 } else { 0.0 };

        let (assign47010_e63797,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) && s.b[2194]) && (!s.b[2195])) && (!s.b[2196])) && (!s.b[2197])) && s.b[2198]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign47010_e63797;

        let (assign47020_e63812,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) && s.b[2194]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign47020_e63812;

        let mut assign47030_loop_guard: usize = 0;
        while {
            let assign47030_cond_e63828: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) && s.b[2194]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign47030_cond_e63828 != 0.0
        } {
            assign47030_loop_guard += 1;
            assert!(assign47030_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) && s.b[2194]) {
                s.store_sqrt(726, 726);
            }
            let (assign47030_body1_e63861,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) && s.b[2194]) {
        let assign47030_body1_e63859: f64 = (s.v[719] + 1.0);
        (assign47030_body1_e63859,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign47030_body1_e63861;
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2193]) && (!s.b[2194])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
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
        }

        let (assign47130_e64033,) = {
    if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign47130_e64033;

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_mul(2136, 2125, 2126);
        }

        let (assign47150_e64057,) = {
    if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign47150_e64057;

    }

    pub(super) fn stamp_transient_block_45(
        s: &mut Scratch,
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
            }
            let (assign47160_body8_e64196,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign47160_body8_e64196;
            let (assign47160_body9_e64209,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47160_body9_e64209;
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) {
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
            let (assign47160_body20_e64358,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) && s.b[2200]) && s.b[2201]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47160_body20_e64358;
            s.b[2202] = (2.0 == 2.0);
            s.v[2202] = if s.b[2202] { 1.0 } else { 0.0 };
            let (assign47160_body22_e64381,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) && s.b[2200]) && (!s.b[2201])) && s.b[2202]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47160_body22_e64381;
            s.b[2203] = (2.0 == 4.0);
            s.v[2203] = if s.b[2203] { 1.0 } else { 0.0 };
            let (assign47160_body24_e64407,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) && s.b[2200]) && (!s.b[2201])) && (!s.b[2202])) && s.b[2203]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47160_body24_e64407;
            s.b[2204] = (2.0 == 8.0);
            s.v[2204] = if s.b[2204] { 1.0 } else { 0.0 };
            let (assign47160_body26_e64436,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) && s.b[2200]) && (!s.b[2201])) && (!s.b[2202])) && (!s.b[2203])) && s.b[2204]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47160_body26_e64436;
            let (assign47160_body27_e64451,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) && s.b[2200]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign47160_body27_e64451;
            let mut assign47160_body28_loop_guard: usize = 0;
            while {
                let assign47160_body28_cond_e64467: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) && s.b[2200]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign47160_body28_cond_e64467 != 0.0
            } {
                assign47160_body28_loop_guard += 1;
                assert!(assign47160_body28_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) && s.b[2200]) {
                    s.store_sqrt(726, 726);
                }
                let (assign47160_body28_body1_e64500,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) && s.b[2200]) {
        let assign47160_body28_body1_e64498: f64 = (s.v[719] + 1.0);
        (assign47160_body28_body1_e64498,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign47160_body28_body1_e64500;
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2199]) && (!s.b[2200])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
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
            }
            let (assign47160_body44_e64745,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign47160_body44_e64745;
            let (assign47160_body45_e64758,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47160_body45_e64758;
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) {
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
            let (assign47160_body56_e64907,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) && s.b[2206]) && s.b[2207]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47160_body56_e64907;
            s.b[2208] = (2.0 == 2.0);
            s.v[2208] = if s.b[2208] { 1.0 } else { 0.0 };
            let (assign47160_body58_e64930,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) && s.b[2206]) && (!s.b[2207])) && s.b[2208]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47160_body58_e64930;
            s.b[2209] = (2.0 == 4.0);
            s.v[2209] = if s.b[2209] { 1.0 } else { 0.0 };
            let (assign47160_body60_e64956,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) && s.b[2206]) && (!s.b[2207])) && (!s.b[2208])) && s.b[2209]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47160_body60_e64956;
            s.b[2210] = (2.0 == 8.0);
            s.v[2210] = if s.b[2210] { 1.0 } else { 0.0 };
            let (assign47160_body62_e64985,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) && s.b[2206]) && (!s.b[2207])) && (!s.b[2208])) && (!s.b[2209])) && s.b[2210]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47160_body62_e64985;
            let (assign47160_body63_e65000,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) && s.b[2206]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign47160_body63_e65000;
            let mut assign47160_body64_loop_guard: usize = 0;
            while {
                let assign47160_body64_cond_e65016: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) && s.b[2206]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign47160_body64_cond_e65016 != 0.0
            } {
                assign47160_body64_loop_guard += 1;
                assert!(assign47160_body64_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) && s.b[2206]) {
                    s.store_sqrt(726, 726);
                }
                let (assign47160_body64_body1_e65049,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) && s.b[2206]) {
        let assign47160_body64_body1_e65047: f64 = (s.v[719] + 1.0);
        (assign47160_body64_body1_e65047,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign47160_body64_body1_e65049;
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) && (!s.b[2206])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2205]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, 1e-12);
                s.store_div_scaled_product_indices(337, 725, 726, 1e-12, 770, 1.0);
                s.store_add_offset_lhs(2081, 2127, (-1e-12), 780);
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
                s.store_add_div_rhs_mixed_ai(2133, 2085, A::add_scaled_square_product(s.ad_value(2127), 1.0, s.ad_value(2081), A::sub_scaled_inputs(s.ad_value(2081), 1.0, s.ad_value(2127), 2.0), 1.0), 2132);
                s.store_scalar(2134, 1.0);
                s.store_mul_sub_from_scalar_rhs_ad_lhs(2135, A::add_scaled_product(s.ad_value(337), 1.0, A::div(s.ad_value(2127), s.ad_value(2081)), s.ad_value(337), (-1.0)), 1.0, 2136);
            }
            s.b[2211] = ((s.v[2133] > (s.v[2083] - p.p406)) && (p.p406 >= 0.0));
            s.v[2211] = if s.b[2211] { 1.0 } else { 0.0 };
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) {
                s.store_offset_sub(781, 2133, 2083, p.p406);
                s.store_square(722, 781);
                s.store_scalar(723, (p.p406 * p.p406));
                s.store_scalar(724, 1.0);
                s.store_scalar(725, 1.0);
            }
            let (assign47160_body83_e65350,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign47160_body83_e65350;
            let (assign47160_body84_e65363,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47160_body84_e65363;
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) {
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
            let (assign47160_body99_e65572,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) && s.b[2212]) && s.b[2213]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47160_body99_e65572;
            s.b[2214] = (4.0 == 2.0);
            s.v[2214] = if s.b[2214] { 1.0 } else { 0.0 };
            let (assign47160_body101_e65595,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) && s.b[2212]) && (!s.b[2213])) && s.b[2214]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47160_body101_e65595;
            s.b[2215] = (4.0 == 4.0);
            s.v[2215] = if s.b[2215] { 1.0 } else { 0.0 };
            let (assign47160_body103_e65621,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) && s.b[2212]) && (!s.b[2213])) && (!s.b[2214])) && s.b[2215]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47160_body103_e65621;
            s.b[2216] = (4.0 == 8.0);
            s.v[2216] = if s.b[2216] { 1.0 } else { 0.0 };
            let (assign47160_body105_e65650,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) && s.b[2212]) && (!s.b[2213])) && (!s.b[2214])) && (!s.b[2215])) && s.b[2216]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.v[720] = assign47160_body105_e65650;
            let (assign47160_body106_e65665,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) && s.b[2212]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign47160_body106_e65665;
            let mut assign47160_body107_loop_guard: usize = 0;
            while {
                let assign47160_body107_cond_e65681: f64 = if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) && s.b[2212]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                assign47160_body107_cond_e65681 != 0.0
            } {
                assign47160_body107_loop_guard += 1;
                assert!(assign47160_body107_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) && s.b[2212]) {
                    s.store_sqrt(726, 726);
                }
                let (assign47160_body107_body1_e65714,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) && s.b[2212]) {
        let assign47160_body107_body1_e65712: f64 = (s.v[719] + 1.0);
        (assign47160_body107_body1_e65712,)
    } else {
        (s.v[719],)
    }
};
                s.v[719] = assign47160_body107_body1_e65714;
            }
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) && (!s.b[2212])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2211]) {
                s.store_div_from_scalar(726, 1.0, 726);
                s.store_scaled_mul(780, 781, 726, p.p406);
                s.store_div_scaled_product_indices(334, 725, 726, p.p406, 770, 1.0);
                s.store_add_offset_lhs(2133, 2083, (-p.p406), 780);
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
                s.store_sub_offset_lhs(344, 340, (-1.0), 339);
            }
            s.b[2217] = (s.v[339] >= 1e-7);
            s.v[2217] = if s.b[2217] { 1.0 } else { 0.0 };
            let (assign47160_body122_e65935,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2217]) {
        let assign47160_body122_e65933: f64 = (-1.0);
        (assign47160_body122_e65933,)
    } else {
        (s.v[347],)
    }
};
            s.v[347] = assign47160_body122_e65935;
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2217]) {
                s.store_mul_scaled_sqrt_rhs(2094, 209, -1.0, 344);
                s.store_div_scaled_product3_indices(345, 209, 209, 154, 0.5, 2094, 1.0);
                s.store_mul_offset_rhs(2121, 345, 340, (-1.0));
                s.store_mul_sub_from_scalar_rhs(2123, 345, 1.0, 340);
            }
            s.b[2218] = (s.v[339] < (-1e-7));
            s.v[2218] = if s.b[2218] { 1.0 } else { 0.0 };
            let (assign47160_body128_e66028,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2217])) && s.b[2218]) {
        (1.0,)
    } else {
        (s.v[347],)
    }
};
            s.v[347] = assign47160_body128_e66028;
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2217])) && s.b[2218]) {
                s.store_exp_mul_scaled_lhs_mixed_ia(342, 154, -1.0, A::sub_scaled_inputs(s.ad_value(2085), 1.0, s.ad_value(2113), p.p398));
                s.store_exp_mul_scaled_lhs_mixed_ia(343, 154, -1.0, A::sub_scaled_inputs(s.ad_value(2088), 1.0, s.ad_value(2113), p.p398));
                s.store_mul_sqrt_ad_rhs(2094, 209, A::add_scaled_product(s.ad_value(344), 1.0, s.ad_value(210), A::add_scaled_inputs3(s.ad_value(342), 1.0, s.ad_value(343), (-1.0), s.ad_value(339), 1.0), 1.0));
                s.store_div_scaled_product3_indices(345, 209, 209, 154, 0.5, 2094, 1.0);
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
            let (assign47160_body146_e66444,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (s.v[79] != 0.0)) {
        let assign47160_body146_e66442: f64 = (150.0 + 1.0);
        (assign47160_body146_e66442,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign47160_body146_e66444;
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (s.v[79] == 0.0)) {
                s.store_add_scaled_product_right_sub(2096, 2094, 1.0, 185, 85, 2085, 1.0);
                s.store_sub(2097, 2121, 185);
                s.copy_ad(2098, 2123);
                s.store_sub(2099, 2088, 2133);
                s.store_neg(2100, 2134);
                s.store_sub_from_scalar(2101, 1.0, 2135);
                s.store_add_scaled_products_indices(2102, 2097, 2101, 1.0, 2098, 2100, (-1.0));
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (s.v[79] == 0.0)) {
                if (s.v[2102] > 0.0) {
                    s.store_div_from_scalar_offset_input(2103, 1.0, 2102, 1e-25);
                } else {
                    s.store_div_from_scalar_offset_input(2103, 1.0, 2102, (-1e-25));
                }
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (s.v[79] == 0.0)) {
                s.copy_ad(2104, 2101);
                s.store_neg(2105, 2098);
                s.store_neg(2106, 2100);
                s.copy_ad(2107, 2097);
                s.store_mul_add_scaled_products_indices_rhs(2108, 2103, 2104, 2096, -1.0, 2105, 2099, -1.0);
                s.store_mul_add_scaled_products_indices_rhs(2109, 2103, 2106, 2096, -1.0, 2107, 2099, -1.0);
                s.store_abs(335, 2108);
            }
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (s.v[79] == 0.0)) {
                if (s.v[335] < ((s.v[2109]) as f64).abs()) {
                    s.store_abs(335, 2109);
                } else {
                }
            }
            s.b[2220] = (s.v[335] > 0.1);
            s.v[2220] = if s.b[2220] { 1.0 } else { 0.0 };
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (s.v[79] == 0.0)) && s.b[2220]) {
                s.store_mul_div_from_scalar_rhs(2108, 2108, 0.1, 335);
                s.store_mul_div_from_scalar_rhs(2109, 2109, 0.1, 335);
            }
            s.b[2221] = (s.v[335] < 1e-10);
            s.v[2221] = if s.b[2221] { 1.0 } else { 0.0 };
            let (assign47160_body167_e66791,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (s.v[79] == 0.0)) && s.b[2221]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign47160_body167_e66791;
            if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (s.v[79] == 0.0)) {
                s.store_add(2085, 2085, 2108);
                s.store_add(2088, 2088, 2109);
            }
            let (assign47160_body170_e66836,) = {
    if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
        let assign47160_body170_e66834: f64 = (s.v[97] + 1.0);
        (assign47160_body170_e66834,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign47160_body170_e66836;
        }

    }

    pub(super) fn stamp_transient_block_46(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_mul_sub_rhs(339, 154, 2085, 2088);
            s.store_exp(340, 339);
            s.store_offset_sub_ad(344, A::offset(s.ad_value(340), (-1.0)), s.ad_value(339), 1e-15);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            if (s.v[339] > 0.0) {
                s.store_mul_scaled_sqrt_rhs(2118, 209, -1.0, 344);
            } else {
                s.store_mul_sqrt_rhs(2118, 209, 344);
            }
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
        }

        let (assign47290_e67020,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign47290_e67020;

        let (assign47300_e67035,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign47300_e67035;

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) {
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

        let (assign47490_e67338,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) && s.b[2225]) && s.b[2226]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign47490_e67338;

        s.b[2227] = (6.0 == 2.0);
        s.v[2227] = if s.b[2227] { 1.0 } else { 0.0 };

        let (assign47510_e67363,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) && s.b[2225]) && (!s.b[2226])) && s.b[2227]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign47510_e67363;

        s.b[2228] = (6.0 == 4.0);
        s.v[2228] = if s.b[2228] { 1.0 } else { 0.0 };

        let (assign47530_e67391,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) && s.b[2225]) && (!s.b[2226])) && (!s.b[2227])) && s.b[2228]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign47530_e67391;

        s.b[2229] = (6.0 == 8.0);
        s.v[2229] = if s.b[2229] { 1.0 } else { 0.0 };

        let (assign47550_e67422,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) && s.b[2225]) && (!s.b[2226])) && (!s.b[2227])) && (!s.b[2228])) && s.b[2229]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign47550_e67422;

        let (assign47560_e67439,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) && s.b[2225]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign47560_e67439;

        let mut assign47570_loop_guard: usize = 0;
        while {
            let assign47570_cond_e67457: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) && s.b[2225]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign47570_cond_e67457 != 0.0
        } {
            assign47570_loop_guard += 1;
            assert!(assign47570_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) && s.b[2225]) {
                s.store_sqrt(726, 726);
            }
            let (assign47570_body1_e67494,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) && s.b[2225]) {
        let assign47570_body1_e67492: f64 = (s.v[719] + 1.0);
        (assign47570_body1_e67492,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign47570_body1_e67494;
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2223]) && s.b[2224]) && (!s.b[2225])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 6.0)));
            }
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
            s.store_offset_add_scaled_product(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, s.ad_value(154), s.ad_value(336), (-1.0), 1e-15);
            s.store_mul_scaled_sqrt_rhs(2114, 209, -1.0, 338);
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
            if (s.v[338] > 0.0) {
                s.store_sqrt(337, 338);
            } else {
                s.store_scaled_sqrt_scaled_input(337, 338, -1.0, -1.0);
            }
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) {
            s.store_offset_add_ad(2156, s.ad_value(2155), A::mul_sub_from_scalar_rhs(s.ad_value(2130), 1.0, s.ad_value(337)), p.p397);
            s.copy_ad(2152, 2156);
        }

        let (assign47750_e67805,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign47750_e67805;

        let (assign47760_e67818,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.v[97] = assign47760_e67818;

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
                s.store_div_scaled_product3_indices(345, 338, 338, 154, 0.5, 2153, 1.0);
                s.store_mul_sub_from_scalar_rhs(2154, 345, 1.0, 336);
            }
            let (assign47770_body9_e67990,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) && (s.v[79] != 0.0)) {
        let assign47770_body9_e67988: f64 = (150.0 + 1.0);
        (assign47770_body9_e67988,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign47770_body9_e67990;
            if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) && (s.v[79] == 0.0)) {
                s.store_add_scaled_offset_product_rhs_mixed_iia(2096, 2153, 1.0, 185, A::sub(s.ad_value(2155), s.ad_value(2152)), p.p397, -1.0);
                s.store_add(2097, 185, 2154);
                s.store_div_scaled_inputs_indices(2108, 2096, -1.0, 2097, 1.0);
            }
            s.b[2232] = (((s.v[2108]) as f64).abs() < 1e-10);
            s.v[2232] = if s.b[2232] { 1.0 } else { 0.0 };
            let (assign47770_body14_e68074,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) && (s.v[79] == 0.0)) && s.b[2232]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.v[79] = assign47770_body14_e68074;
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
            let (assign47770_body20_e68160,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && s.b[2230]) {
        let assign47770_body20_e68158: f64 = (s.v[97] + 1.0);
        (assign47770_body20_e68158,)
    } else {
        (s.v[97],)
    }
};
            s.v[97] = assign47770_body20_e68160;
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
        }

        let (assign47930_e68411,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign47930_e68411;

        let (assign47940_e68427,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign47940_e68427;

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) {
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

    }

    pub(super) fn stamp_transient_block_47(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        let (assign48050_e68603,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) && s.b[2237]) && s.b[2238]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign48050_e68603;

        s.b[2239] = (2.0 == 2.0);
        s.v[2239] = if s.b[2239] { 1.0 } else { 0.0 };

        let (assign48070_e68629,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) && s.b[2237]) && (!s.b[2238])) && s.b[2239]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign48070_e68629;

        s.b[2240] = (2.0 == 4.0);
        s.v[2240] = if s.b[2240] { 1.0 } else { 0.0 };

        let (assign48090_e68658,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) && s.b[2237]) && (!s.b[2238])) && (!s.b[2239])) && s.b[2240]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign48090_e68658;

        s.b[2241] = (2.0 == 8.0);
        s.v[2241] = if s.b[2241] { 1.0 } else { 0.0 };

        let (assign48110_e68690,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) && s.b[2237]) && (!s.b[2238])) && (!s.b[2239])) && (!s.b[2240])) && s.b[2241]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign48110_e68690;

        let (assign48120_e68708,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) && s.b[2237]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign48120_e68708;

        let mut assign48130_loop_guard: usize = 0;
        while {
            let assign48130_cond_e68727: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) && s.b[2237]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign48130_cond_e68727 != 0.0
        } {
            assign48130_loop_guard += 1;
            assert!(assign48130_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) && s.b[2237]) {
                s.store_sqrt(726, 726);
            }
            let (assign48130_body1_e68766,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) && s.b[2237]) {
        let assign48130_body1_e68764: f64 = (s.v[719] + 1.0);
        (assign48130_body1_e68764,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign48130_body1_e68766;
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) && (!s.b[2237])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2236]) {
            s.store_div_from_scalar(726, 1.0, 726);
            s.store_mul3_lhs(780, 781, 339, 726);
            s.store_div_scaled_product3_indices(334, 339, 725, 726, 1.0, 770, 1.0);
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
        }

        let (assign48300_e69074,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign48300_e69074;

        let (assign48310_e69090,) = {
    if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign48310_e69090;

        if (((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) {
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

        let (assign48420_e69266,) = {
    if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) && s.b[2243]) && s.b[2244]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign48420_e69266;

        s.b[2245] = (2.0 == 2.0);
        s.v[2245] = if s.b[2245] { 1.0 } else { 0.0 };

        let (assign48440_e69292,) = {
    if ((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) && s.b[2243]) && (!s.b[2244])) && s.b[2245]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign48440_e69292;

        s.b[2246] = (2.0 == 4.0);
        s.v[2246] = if s.b[2246] { 1.0 } else { 0.0 };

        let (assign48460_e69321,) = {
    if (((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) && s.b[2243]) && (!s.b[2244])) && (!s.b[2245])) && s.b[2246]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign48460_e69321;

        s.b[2247] = (2.0 == 8.0);
        s.v[2247] = if s.b[2247] { 1.0 } else { 0.0 };

        let (assign48480_e69353,) = {
    if ((((((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) && s.b[2243]) && (!s.b[2244])) && (!s.b[2245])) && (!s.b[2246])) && s.b[2247]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.v[720] = assign48480_e69353;

        let (assign48490_e69371,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) && s.b[2243]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.v[719] = assign48490_e69371;

        let mut assign48500_loop_guard: usize = 0;
        while {
            let assign48500_cond_e69390: f64 = if (((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) && s.b[2243]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            assign48500_cond_e69390 != 0.0
        } {
            assign48500_loop_guard += 1;
            assert!(assign48500_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) && s.b[2243]) {
                s.store_sqrt(726, 726);
            }
            let (assign48500_body1_e69429,) = {
    if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) && s.b[2243]) {
        let assign48500_body1_e69427: f64 = (s.v[719] + 1.0);
        (assign48500_body1_e69427,)
    } else {
        (s.v[719],)
    }
};
            s.v[719] = assign48500_body1_e69429;
        }

        if ((((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2230])) && s.b[2242]) && (!s.b[2243])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
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
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_ad(336, s.ad_value(335), s.ad_value(658));
            }
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            s.store_offset(337, 336, 1.0);
        }

        if (s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::div_from_scalar(1.0, s.ad_value(658)));
            }
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
            s.store_add_scaled_inputs3_offset_indices(781, 2085, 1.0, 2084, 1.0, 85, -1.0, (-0.01));
            s.store_scaled_add(782, 2085, 2084, (4.0 * 0.01));
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4_indices(2093, 2085, 1.0, 2084, 1.0, 781, (-0.5), 782, (-0.5));
            s.store_add_scaled_inputs3_offset_indices(781, 2093, 1.0, 2113, -1.0, 2087, 1.0, (-0.01));
            s.store_scaled_sub(782, 2113, 2087, (4.0 * 0.01));
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
            s.store_sqrt_square_add(782, 781, 782);
            s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);
            s.store_add_scaled_inputs4_indices(2093, 2113, 1.0, 2087, (-1.0), 781, 0.5, 782, 0.5);
            s.copy_ad(2089, 2084);
            s.copy_ad(2086, 2093);
        }

        let (assign48870_e70054,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.v[79] = assign48870_e70054;

        if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
            s.store_mul(2137, 2125, 2126);
        }

        let (assign48890_e70084,) = {
    if ((s.b[1439] && (s.b[1442] && (!(s.b[1440] || s.b[1441])))) && (!s.b[2248])) {
        (1.0,)
    } else {
        (s.v[98],)
    }
};
        s.v[98] = assign48890_e70084;

    }
}

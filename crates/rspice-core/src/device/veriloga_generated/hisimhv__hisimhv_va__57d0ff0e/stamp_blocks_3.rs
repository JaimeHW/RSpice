#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_48(
        s: &mut Scratch,
    ) {
        let (t0,) = {
    if ((((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) && (!s.b[1695])) && (!s.b[1696])) && (!s.b[1697])) && s.b[1698]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t0);
        let (t1,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t1);let mut t5: usize = 0;
        while {
            let t4: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t4 != 0.0
        } {
            t5 += 1;assert!(t5 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) {s.store_sqrt(726, 726);}
            let (t3,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && s.b[1694]) {
        let t2: f64 = (s.v[719] + 1.0);
        (t2,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t3);
        }
        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) && (!s.b[1694])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.2);s.store_div_scaled_product_indices(334, 725, 726, 0.2, 770, 1.0);s.store_sub_from_scalar(344, (0.3 + 0.2), 780);}
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1693]) {
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && (!s.b[1693])) {
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && (!s.b[1693])) {s.store_scalar(334, 1.0);}
        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {s.store_offset(344, 344, (10.0 * 2.220446049250313e-16));s.store_div(335, 790, 344);}
        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_offset_rhs(336, 335, 658, (-1.0));
            }
        }
        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {s.store_mul(341, 336, 335);s.store_offset(337, 341, 1.0);}
        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
        }
        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {s.store_mul(340, 338, 337);s.store_div(348, 790, 340);}
        s.b[1699] = ((s.v[85] < 0.5) && (0.5 >= 0.0));s.store_scalar(1699, if s.b[1699] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) {s.store_sub_from_scalar(781, 0.5, 85);s.store_square(722, 781);s.store_scalar(723, (0.5 * 0.5));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t6,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t6);
        let (t7,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t7);
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1700] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1700, if s.b[1700] { 1.0 } else { 0.0 });s.b[1701] = (2.0 == 1.0);s.store_scalar(1701, if s.b[1701] { 1.0 } else { 0.0 });
        let (t8,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) && s.b[1701]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t8);s.b[1702] = (2.0 == 2.0);s.store_scalar(1702, if s.b[1702] { 1.0 } else { 0.0 });
        let (t9,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) && (!s.b[1701])) && s.b[1702]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t9);s.b[1703] = (2.0 == 4.0);s.store_scalar(1703, if s.b[1703] { 1.0 } else { 0.0 });
        let (ta,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) && (!s.b[1701])) && (!s.b[1702])) && s.b[1703]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, ta);s.b[1704] = (2.0 == 8.0);s.store_scalar(1704, if s.b[1704] { 1.0 } else { 0.0 });
        let (tb,) = {
    if ((((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) && (!s.b[1701])) && (!s.b[1702])) && (!s.b[1703])) && s.b[1704]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tb);
        let (tc,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tc);let mut t10: usize = 0;
        while {
            let tf: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tf != 0.0
        } {
            t10 += 1;assert!(t10 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) {s.store_sqrt(726, 726);}
            let (te,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && s.b[1700]) {
        let td: f64 = (s.v[719] + 1.0);
        (td,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, te);
        }
        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) && (!s.b[1700])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.5);s.store_div_scaled_product_indices(334, 725, 726, 0.5, 770, 1.0);s.store_sub_from_scalar(1537, 0.5, 780);}
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1699]) {
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && (!s.b[1699])) {s.copy_ad(1537, 85);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_49(
        s: &mut Scratch,
    ) {
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && (!s.b[1699])) {s.store_scalar(334, 1.0);}
        if ((s.b[1443] && s.b[1444]) && s.b[1686]) {s.store_scale(335, 1537, 0.8);}
        s.b[1705] = ((s.v[348] > (s.v[1537] - s.v[335])) && (s.v[335] >= 0.0));s.store_scalar(1705, if s.b[1705] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) {s.store_add_scaled_inputs3_indices(781, 348, 1.0, 1537, (-1.0), 335, 1.0);s.store_square(722, 781);s.store_square(723, 335);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t11,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t11);
        let (t12,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t12);
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1706] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1706, if s.b[1706] { 1.0 } else { 0.0 });s.b[1707] = (2.0 == 1.0);s.store_scalar(1707, if s.b[1707] { 1.0 } else { 0.0 });
        let (t13,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) && s.b[1707]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t13);s.b[1708] = (2.0 == 2.0);s.store_scalar(1708, if s.b[1708] { 1.0 } else { 0.0 });
        let (t14,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) && (!s.b[1707])) && s.b[1708]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t14);s.b[1709] = (2.0 == 4.0);s.store_scalar(1709, if s.b[1709] { 1.0 } else { 0.0 });
        let (t15,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) && (!s.b[1707])) && (!s.b[1708])) && s.b[1709]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t15);s.b[1710] = (2.0 == 8.0);s.store_scalar(1710, if s.b[1710] { 1.0 } else { 0.0 });
        let (t16,) = {
    if ((((((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) && (!s.b[1707])) && (!s.b[1708])) && (!s.b[1709])) && s.b[1710]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t16);
        let (t17,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t17);let mut t1b: usize = 0;
        while {
            let t1a: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t1a != 0.0
        } {
            t1b += 1;assert!(t1b <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) {s.store_sqrt(726, 726);}
            let (t19,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && s.b[1706]) {
        let t18: f64 = (s.v[719] + 1.0);
        (t18,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t19);
        }
        if ((((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) && (!s.b[1706])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 335, 726);s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs3_indices(790, 1537, 1.0, 335, (-1.0), 780, 1.0);}
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && s.b[1705]) {
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1686]) && (!s.b[1705])) {s.copy_ad(790, 348);s.store_scalar(334, 1.0);}
        if ((s.b[1443] && s.b[1444]) && (!s.b[1686])) {s.copy_ad(348, 790);}
        s.b[1711] = (s.v[790] <= 0.0);s.store_scalar(1711, if s.b[1711] { 1.0 } else { 0.0 });
        if ((s.b[1443] && s.b[1444]) && s.b[1711]) {s.copy_ad(1462, 1461);s.copy_ad(1484, 1483);s.copy_ad(1465, 1464);s.copy_ad(1478, 1477);s.copy_ad(1539, 1538);s.copy_ad(1499, 1497);s.copy_ad(1500, 1498);s.copy_ad(1518, 1517);s.copy_ad(1516, 1515);}
        if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {s.store_sqrt_mul_ad(1454, A::div_scaled_product(s.ad_value(1547), s.ad_value(622), 1.0, A::add(s.ad_value(622), s.ad_value(964)), 1.0), A::add_scaled_inputs3(s.ad_value(790), 1.0, s.ad_value(1435), (-1.0), s.ad_value(1463), 1.0));}
        s.b[1712] = (s.v[1454] > s.v[965]);s.store_scalar(1712, if s.b[1712] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {s.copy_ad(1466, 790);s.copy_ad(1448, 965);s.copy_ad(1484, 790);s.copy_ad(1512, 790);s.store_sub_mixed_ia(1465, 1484, A::mul3(s.ad_value(1548), s.ad_value(1448), s.ad_value(1448)));}
        let (t1c,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
        (s.v[1466],)
    } else {
        (s.v[1510],)
    }
};
        s.store_scalar(1510, t1c);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_50(
        s: &mut Scratch,
    ) {
        let (t1d,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
        (s.v[1465],)
    } else {
        (s.v[1473],)
    }
};
        s.store_scalar(1473, t1d);
        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {s.store_mul(1499, 1448, 1546);}
        let (t1e,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, t1e);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_51(
        s: &mut Scratch,
    ) {
        let mut t3d: usize = 0;
        while {
            let t3b: f64 = (150.0 + 1.0);let t3c: f64 = if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (s.v[97] <= t3b)) { 1.0 } else { 0.0 };
            t3c != 0.0
        } {
            t3d += 1;assert!(t3d <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {s.store_sqrt_mul_sub_rhs(1448, 1547, 1484, 1465);}
            s.b[1713] = ((s.v[1448] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1713, if s.b[1713] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) {s.store_offset_sub(781, 1448, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (t33,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t33);
            let (t34,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t34);
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1714] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1714, if s.b[1714] { 1.0 } else { 0.0 });s.b[1715] = (2.0 == 1.0);s.store_scalar(1715, if s.b[1715] { 1.0 } else { 0.0 });
            let (t1f,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) && s.b[1715]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t1f);s.b[1716] = (2.0 == 2.0);s.store_scalar(1716, if s.b[1716] { 1.0 } else { 0.0 });
            let (t20,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) && (!s.b[1715])) && s.b[1716]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t20);s.b[1717] = (2.0 == 4.0);s.store_scalar(1717, if s.b[1717] { 1.0 } else { 0.0 });
            let (t21,) = {
    if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) && (!s.b[1715])) && (!s.b[1716])) && s.b[1717]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t21);s.b[1718] = (2.0 == 8.0);s.store_scalar(1718, if s.b[1718] { 1.0 } else { 0.0 });
            let (t22,) = {
    if (((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) && (!s.b[1715])) && (!s.b[1716])) && (!s.b[1717])) && s.b[1718]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t22);
            let (t23,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t23);let mut t27: usize = 0;
            while {
                let t26: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t26 != 0.0
            } {
                t27 += 1;assert!(t27 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) {s.store_sqrt(726, 726);}
                let (t25,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && s.b[1714]) {
        let t24: f64 = (s.v[719] + 1.0);
        (t24,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t25);
            }
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) && (!s.b[1714])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(334, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1448, 965, (-1e-8), 780);}
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1713]) {
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (!s.b[1713])) {
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (!s.b[1713])) {s.store_scalar(334, 1.0);}
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {s.store_add_scaled_inputs3_indices(335, 1465, 1.0, 1435, (-1.0), 1463, 1.0);}
            s.b[1719] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1719, if s.b[1719] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (t28,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t28);
            let (t29,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t29);
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1720] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1720, if s.b[1720] { 1.0 } else { 0.0 });s.b[1721] = (2.0 == 1.0);s.store_scalar(1721, if s.b[1721] { 1.0 } else { 0.0 });
            let (t2a,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) && s.b[1721]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t2a);s.b[1722] = (2.0 == 2.0);s.store_scalar(1722, if s.b[1722] { 1.0 } else { 0.0 });
            let (t2b,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) && (!s.b[1721])) && s.b[1722]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t2b);s.b[1723] = (2.0 == 4.0);s.store_scalar(1723, if s.b[1723] { 1.0 } else { 0.0 });
            let (t2c,) = {
    if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) && (!s.b[1721])) && (!s.b[1722])) && s.b[1723]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t2c);s.b[1724] = (2.0 == 8.0);s.store_scalar(1724, if s.b[1724] { 1.0 } else { 0.0 });
            let (t2d,) = {
    if (((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) && (!s.b[1721])) && (!s.b[1722])) && (!s.b[1723])) && s.b[1724]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t2d);
            let (t2e,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t2e);let mut t32: usize = 0;
            while {
                let t31: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t31 != 0.0
            } {
                t32 += 1;assert!(t32 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) {s.store_sqrt(726, 726);}
                let (t30,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && s.b[1720]) {
        let t2f: f64 = (s.v[719] + 1.0);
        (t2f,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t30);
            }
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) && (!s.b[1720])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(341, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1719]) {
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (!s.b[1719])) {s.copy_ad(336, 335);s.store_scalar(341, 1.0);}
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {s.store_sqrt_mul(1452, 1550, 336);s.store_mul(1499, 1448, 1546);s.store_mul_div_from_scalar_lhs_ad_indices(1530, (-1.034943e-10), 1448, 334);s.store_mul_scale_offset_indices(1500, 1544, 1452, -1.0, 0.0);s.store_mul_div_from_scalar_lhs_ad_indices(1532, (-1.034943e-10), 1452, 341);s.store_add_mixed_ai(1485, A::add_scaled_product(s.ad_value(1499), 1.0, s.ad_value(185), A::sub(s.ad_value(1466), s.ad_value(1484)), 1.0), 1500);s.copy_ad(1487, 185);s.store_add(1488, 1530, 1532);s.store_add_scaled_product_mixed_iia(1486, 1465, 1.0, 1535, A::sub(A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1484), 1.0), s.ad_value(1463)), (-1.0));s.store_scalar(1489, 0.0);s.store_scalar(1490, 1.0);s.store_add_scaled_products_indices(1491, 1487, 1490, 1.0, 1489, 1488, (-1.0));s.store_div(1492, 1490, 1491);s.store_div_scaled_inputs_indices(1493, 1488, -1.0, 1491, 1.0);s.store_div_scaled_inputs_indices(1494, 1489, -1.0, 1491, 1.0);s.store_div(1495, 1487, 1491);}
            s.b[1725] = (((((s.v[1492] * s.v[1485]) + (s.v[1493] * s.v[1486]))) as f64).abs() > 0.5);s.store_scalar(1725, if s.b[1725] { 1.0 } else { 0.0 });
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1725]) {s.store_offset(1466, 1466, (-(0.5 * (if (((s.v[1492] * s.v[1485]) + (s.v[1493] * s.v[1486])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1725]) {s.store_offset(1465, 1465, (-(0.5 * (if (((s.v[1494] * s.v[1485]) + (s.v[1495] * s.v[1486])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (!s.b[1725])) {s.store_sub_mixed_ia(1466, 1466, A::add_scaled_products(s.ad_value(1492), s.ad_value(1485), 1.0, s.ad_value(1493), s.ad_value(1486), 1.0));s.store_sub_mixed_ia(1465, 1465, A::add_scaled_products(s.ad_value(1494), s.ad_value(1485), 1.0, s.ad_value(1495), s.ad_value(1486), 1.0));}
            s.b[1726] = (((((s.v[1466] - s.v[1510])) as f64).abs() <= 1e-12) && ((((s.v[1465] - s.v[1473])) as f64).abs() <= 1e-12));s.store_scalar(1726, if s.b[1726] { 1.0 } else { 0.0 });
            let (t36,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1726]) {
        let t35: f64 = (150.0 + 1.0);
        (t35,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t36);
            let (t37,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
        (s.v[1466],)
    } else {
        (s.v[1510],)
    }
};
            s.store_scalar(1510, t37);
            let (t38,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
        (s.v[1465],)
    } else {
        (s.v[1473],)
    }
};
            s.store_scalar(1473, t38);
            let (t3a,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
        let t39: f64 = (s.v[97] + 1.0);
        (t39,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t3a);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_52(
        s: &mut Scratch,
    ) {
        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {s.copy_ad(1514, 1465);s.store_mul(1452, 965, 1536);s.store_add_scaled_inputs3_mixed_aii(1465, A::mul3(s.ad_value(1551), s.ad_value(1452), s.ad_value(1452)), 1.0, 1435, 1.0, 1463, -1.0);s.store_add_scaled_product_indices(1484, 1465, 1.0, 1548, 1543, 1.0);s.copy_ad(1462, 1484);s.copy_ad(1467, 1484);}
        let (t3e,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) {
        (s.v[1484],)
    } else {
        (s.v[1509],)
    }
};
        s.store_scalar(1509, t3e);s.b[1727] = (s.v[85] > s.v[1466]);s.store_scalar(1727, if s.b[1727] { 1.0 } else { 0.0 });
        let (t3f,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && s.b[1727]) {
        (1.0,)
    } else {
        (s.v[1479],)
    }
};
        s.store_scalar(1479, t3f);s.b[1728] = (s.v[85] > s.v[1509]);s.store_scalar(1728, if s.b[1728] { 1.0 } else { 0.0 });
        let (t40,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (!s.b[1727])) && s.b[1728]) {
        (3.0,)
    } else {
        (s.v[1479],)
    }
};
        s.store_scalar(1479, t40);
        let (t41,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1712]) && (!s.b[1727])) && (!s.b[1728])) {
        (2.0,)
    } else {
        (s.v[1479],)
    }
};
        s.store_scalar(1479, t41);
        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1712])) {s.copy_ad(1466, 790);}
        let (t42,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1712])) {
        (s.v[1466],)
    } else {
        (s.v[1509],)
    }
};
        s.store_scalar(1509, t42);
        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1712])) {s.copy_ad(1467, 1466);s.copy_ad(1512, 1466);s.copy_ad(1448, 1454);s.store_mul(1452, 1448, 1536);s.store_add_scaled_inputs3_mixed_aii(1465, A::mul3(s.ad_value(1551), s.ad_value(1452), s.ad_value(1452)), 1.0, 1435, 1.0, 1463, -1.0);s.store_add_mixed_ai(1484, A::mul3(s.ad_value(1548), s.ad_value(1448), s.ad_value(1448)), 1465);s.copy_ad(1514, 1465);}
        s.b[1729] = (s.v[85] > s.v[1466]);s.store_scalar(1729, if s.b[1729] { 1.0 } else { 0.0 });
        let (t43,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1712])) && s.b[1729]) {
        (1.0,)
    } else {
        (s.v[1479],)
    }
};
        s.store_scalar(1479, t43);
        let (t44,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1712])) && (!s.b[1729])) {
        (2.0,)
    } else {
        (s.v[1479],)
    }
};
        s.store_scalar(1479, t44);
        if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {s.store_mul_add_scaled_inputs3_offset_rhs_indices(335, 1549, 1467, 1.0, 1435, -1.0, 961, 1.0, 0.0);}
        s.b[1730] = (s.v[335] > 0.0);s.store_scalar(1730, if s.b[1730] { 1.0 } else { 0.0 });
        let (t4a,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1730]) {
        let t45: f64 = (-s.v[961]);let t46: f64 = (t45 + s.v[1435]);let t47: f64 = (s.v[335]).sqrt();let t48: f64 = (t47 / s.v[185]);let t49: f64 = (t46 - t48);
        (t49,)
    } else {
        (s.v[1455],)
    }
};
        s.store_scalar(1455, t4a);
        let (t4d,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1730])) {
        let t4b: f64 = (-s.v[961]);let t4c: f64 = (t4b + s.v[1435]);
        (t4c,)
    } else {
        (s.v[1455],)
    }
};
        s.store_scalar(1455, t4d);s.b[1731] = (s.v[85] > s.v[1466]);s.store_scalar(1731, if s.b[1731] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1731]) {s.copy_ad(1465, 1514);s.copy_ad(1484, 790);s.store_add_div_lhs(1481, A::ln(A::mul3(s.ad_value(1456), s.ad_value(85), s.ad_value(85))), A::add(s.ad_value(154), A::div_from_scalar(2.0, s.ad_value(85))), 790);}
        s.b[1732] = (s.v[1481] < (s.v[1512] + s.v[1553]));s.store_scalar(1732, if s.b[1732] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1731]) && s.b[1732]) {s.store_add(1481, 1512, 1553);}
        s.b[1733] = (s.v[85] > s.v[1509]);s.store_scalar(1733, if s.b[1733] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && s.b[1733]) {s.copy_ad(1481, 1462);}
        s.b[1734] = (s.v[85] > s.v[1455]);s.store_scalar(1734, if s.b[1734] { 1.0 } else { 0.0 });
        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) {s.store_add_scaled_product_indices(1457, 154, 1.0, 1456, 85, (-2.0));s.store_add_scaled_product_mixed_aii(1458, A::mul3(s.ad_value(1456), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1484, (-1.0));}
        let (t4e,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) {
        (s.v[1484],)
    } else {
        (s.v[1471],)
    }
};
        s.store_scalar(1471, t4e);
        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) {s.store_div_scaled_inputs2_mixed_aii(1481, A::sqrt(A::add_scaled_square_product(s.ad_value(1457), 1.0, s.ad_value(1456), s.ad_value(1458), (-4.0))), 0.5, 1457, (-0.5), 1456, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_53(
        s: &mut Scratch,
    ) {
        s.b[1735] = (s.v[1481] > (s.v[1467] - s.v[1553]));s.store_scalar(1735, if s.b[1735] { 1.0 } else { 0.0 });
        if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1735]) {s.store_sub(1481, 1467, 1553);}
        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) {s.store_sqrt_mul_sub_rhs(1450, 1547, 1484, 1481);s.store_sqrt_mul_sub_rhs(1448, 1547, 1484, 1465);}
        s.b[1736] = ((s.v[1450] + s.v[1448]) > s.v[965]);s.store_scalar(1736, if s.b[1736] { 1.0 } else { 0.0 });
        let (t4f,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, t4f);let mut t59: usize = 0;
        while {
            let t57: f64 = (150.0 + 1.0);let t58: f64 = if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && (s.v[97] <= t57)) { 1.0 } else { 0.0 };
            t58 != 0.0
        } {
            t59 += 1;assert!(t59 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) {s.store_add_scaled_inputs3_indices(1468, 1450, 1.0, 1448, 1.0, 965, -1.0);s.store_add_ad(1508, A::div_scalar_by_product(1.034943e-10, s.ad_value(1546), s.ad_value(1450), 1.0), A::div(A::mul_sub_from_scalar_rhs(A::div_from_scalar(1.034943e-10, s.ad_value(1546)), 1.0, A::div_scaled_value_offset_denominator(s.ad_value(1536), 1.0, s.ad_value(1536), 1.0, 1.0)), s.ad_value(1448)));}
            s.b[1737] = ((((s.v[1468] / s.v[1508])) as f64).abs() > 0.5);s.store_scalar(1737, if s.b[1737] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1737]) {s.store_offset(1484, 1484, (-(0.5 * (if ((s.v[1468] / s.v[1508]) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && (!s.b[1737])) {s.store_sub_div_rhs_indices(1484, 1484, 1468, 1508);}
            s.b[1738] = (((s.v[1484] - s.v[1435]) + s.v[1463]) < (10.0 * 2.220446049250313e-16));s.store_scalar(1738, if s.b[1738] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1738]) {s.store_offset_sub(1484, 1435, 1463, (10.0 * 2.220446049250313e-16));}
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) {s.store_add_scaled_product_mixed_aii(1458, A::mul3(s.ad_value(1456), s.ad_value(85), s.ad_value(85)), 1.0, 154, 1484, (-1.0));s.store_add_scaled_square_product_indices(335, 1457, 1.0, 1456, 1458, (-4.0));}
            s.b[1739] = (s.v[335] > 0.0);s.store_scalar(1739, if s.b[1739] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1739]) {s.store_div_scaled_inputs2_sqrt_first(1481, 335, 0.5, 1457, (-0.5), 1456, 1.0);}
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && (!s.b[1739])) {s.store_div_scaled_inputs_indices(1481, 1457, (-0.5), 1456, 1.0);}
            s.b[1740] = (s.v[1481] > s.v[1467]);s.store_scalar(1740, if s.b[1740] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1740]) {s.copy_ad(1481, 1467);}
            s.b[1741] = (s.v[1481] > s.v[1484]);s.store_scalar(1741, if s.b[1741] { 1.0 } else { 0.0 });
            if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1741]) {s.store_sub(1481, 1484, 1553);}
            let (t51,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1741]) {
        let t50: f64 = (150.0 + 1.0);
        (t50,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t51);
            if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) {s.store_sqrt_mul_sub_rhs(1450, 1547, 1484, 1481);s.store_div_scaled_inputs2_mixed_aia(1465, A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1484), 1.0), 1.0, 1463, (-1.0), A::offset(s.ad_value(1536), 1.0), 1.0);s.store_sqrt_mul_sub_rhs(1448, 1547, 1484, 1465);}
            s.b[1742] = ((((s.v[1484] - s.v[1471])) as f64).abs() <= 1e-8);s.store_scalar(1742, if s.b[1742] { 1.0 } else { 0.0 });
            let (t53,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) && s.b[1742]) {
        let t52: f64 = (150.0 + 1.0);
        (t52,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t53);
            let (t54,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) {
        (s.v[1484],)
    } else {
        (s.v[1471],)
    }
};
            s.store_scalar(1471, t54);
            let (t56,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && s.b[1734]) && s.b[1736]) {
        let t55: f64 = (s.v[97] + 1.0);
        (t55,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t56);
        }
        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1731])) && (!s.b[1733])) && (!s.b[1734])) {s.copy_ad(1484, 1483);s.copy_ad(1465, 1464);s.copy_ad(1481, 1461);}
        if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {s.copy_ad(1482, 1484);}
        let (t5a,) = {
    if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
        (0.0,)
    } else {
        (s.v[79],)
    }
};
        s.store_scalar(79, t5a);
        if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {s.copy_ad(1462, 1481);s.copy_ad(1484, 1482);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_54(
        s: &mut Scratch,
    ) {
        let (t5b,) = {
    if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
        (s.v[1462],)
    } else {
        (s.v[1474],)
    }
};
        s.store_scalar(1474, t5b);
        let (t5c,) = {
    if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
        (s.v[1484],)
    } else {
        (s.v[1471],)
    }
};
        s.store_scalar(1471, t5c);
        let (t5d,) = {
    if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
        (1.0,)
    } else {
        (s.v[97],)
    }
};
        s.store_scalar(97, t5d);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_55(
        s: &mut Scratch,
    ) {
        let mut t88: usize = 0;
        while {
            let t86: f64 = (150.0 + 1.0);let t87: f64 = if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (s.v[97] <= t86)) { 1.0 } else { 0.0 };
            t87 != 0.0
        } {
            t88 += 1;assert!(t88 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {s.store_mul_sub_mixed_iai(1465, 1535, A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1484), 1.0), 1463);s.store_mul(1534, 1535, 1536);s.store_sub(335, 1484, 1465);}
            s.b[1743] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1743, if s.b[1743] { 1.0 } else { 0.0 });
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (t85,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t85);
            let (t60,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t60);
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1744] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1744, if s.b[1744] { 1.0 } else { 0.0 });s.b[1745] = (2.0 == 1.0);s.store_scalar(1745, if s.b[1745] { 1.0 } else { 0.0 });
            let (t71,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) && s.b[1745]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t71);s.b[1746] = (2.0 == 2.0);s.store_scalar(1746, if s.b[1746] { 1.0 } else { 0.0 });
            let (t72,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) && (!s.b[1745])) && s.b[1746]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t72);s.b[1747] = (2.0 == 4.0);s.store_scalar(1747, if s.b[1747] { 1.0 } else { 0.0 });
            let (t73,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) && (!s.b[1745])) && (!s.b[1746])) && s.b[1747]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t73);s.b[1748] = (2.0 == 8.0);s.store_scalar(1748, if s.b[1748] { 1.0 } else { 0.0 });
            let (t74,) = {
    if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) && (!s.b[1745])) && (!s.b[1746])) && (!s.b[1747])) && s.b[1748]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t74);
            let (t75,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t75);let mut t79: usize = 0;
            while {
                let t78: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t78 != 0.0
            } {
                t79 += 1;assert!(t79 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) {s.store_sqrt(726, 726);}
                let (t77,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && s.b[1744]) {
        let t76: f64 = (s.v[719] + 1.0);
        (t76,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t77);
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) && (!s.b[1744])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1743]) {
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1743])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
            if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {s.store_sqrt_mul(1448, 1547, 336);}
            s.b[1749] = ((s.v[1448] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1749, if s.b[1749] { 1.0 } else { 0.0 });
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) {s.store_offset_sub(781, 1448, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (t7a,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t7a);
            let (t7b,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t7b);
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1750] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1750, if s.b[1750] { 1.0 } else { 0.0 });s.b[1751] = (2.0 == 1.0);s.store_scalar(1751, if s.b[1751] { 1.0 } else { 0.0 });
            let (t7c,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) && s.b[1751]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t7c);s.b[1752] = (2.0 == 2.0);s.store_scalar(1752, if s.b[1752] { 1.0 } else { 0.0 });
            let (t7d,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) && (!s.b[1751])) && s.b[1752]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t7d);s.b[1753] = (2.0 == 4.0);s.store_scalar(1753, if s.b[1753] { 1.0 } else { 0.0 });
            let (t7e,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) && (!s.b[1751])) && (!s.b[1752])) && s.b[1753]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t7e);s.b[1754] = (2.0 == 8.0);s.store_scalar(1754, if s.b[1754] { 1.0 } else { 0.0 });
            let (t7f,) = {
    if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) && (!s.b[1751])) && (!s.b[1752])) && (!s.b[1753])) && s.b[1754]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t7f);
            let (t80,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t80);let mut t84: usize = 0;
            while {
                let t83: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t83 != 0.0
            } {
                t84 += 1;assert!(t84 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) {s.store_sqrt(726, 726);}
                let (t82,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && s.b[1750]) {
        let t81: f64 = (s.v[719] + 1.0);
        (t81,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t82);
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) && (!s.b[1750])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
                }
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1448, 965, (-1e-8), 780);}
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1749]) {
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1749])) {
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1749])) {s.store_scalar(337, 1.0);}
            if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {s.store_sqrt_mul_ad(1452, s.ad_value(1550), A::add_scaled_inputs3(s.ad_value(1465), 1.0, s.ad_value(1435), (-1.0), s.ad_value(1463), 1.0));s.store_mul(1499, 1448, 1546);s.store_mul_ad_product_lhs_mixed_ai(1528, A::div_from_scalar(1.034943e-10, s.ad_value(1448)), 334, 337);s.store_mul_ad_product_lhs_mixed_ai(1530, A::div_from_scalar((-1.034943e-10), s.ad_value(1448)), 334, 337);s.store_mul_scale_offset_indices(1500, 1544, 1452, -1.0, 0.0);s.store_div_from_scalar(1532, (-1.034943e-10), 1452);s.store_scaled_mul(335, 1502, 1543, 8.0);s.store_div_scaled_inputs_product_mixed_aaaii(1520, A::add_scaled_inputs4(A::mul3_scaled_output(s.ad_value(1465), s.ad_value(1465), s.ad_value(1542), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1465), s.ad_value(1542), s.ad_value(1462), 8.0), (-1.0), A::mul3_scaled_output(s.ad_value(1542), s.ad_value(1462), s.ad_value(1462), 4.0), 1.0, A::mul3_scaled_output(s.ad_value(1465), s.ad_value(1502), s.ad_value(1543), 4.0), 1.0), 1.0, A::mul3_scaled_output(s.ad_value(1462), s.ad_value(1502), s.ad_value(1543), 4.0), 1.0, A::mul3(s.ad_value(1545), s.ad_value(1541), s.ad_value(1543)), 1543, 1.0, 335, 1.0);s.store_div_mixed_ai(1521, A::add_scaled_products3(s.ad_value(1465), s.ad_value(1542), (-8.0), s.ad_value(1542), s.ad_value(1462), (4.0 * 2.0), s.ad_value(1502), s.ad_value(1543), 4.0), 335);s.store_div_mixed_ai(1522, A::add_scaled_products3(s.ad_value(1465), s.ad_value(1542), (4.0 * 2.0), s.ad_value(1542), s.ad_value(1462), (-8.0), s.ad_value(1502), s.ad_value(1543), 4.0), 335);s.store_mul_sub_rhs(335, 154, 1462, 1484);s.store_exp(336, 335);}
            s.b[1755] = (s.v[1462] >= s.v[1484]);s.store_scalar(1755, if s.b[1755] { 1.0 } else { 0.0 });
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1755]) {s.store_mul_scaled_sqrt_ad_rhs(1476, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.store_mul_add_scaled_product_div_scaled_product(1524, 154, (-1.0), 154, 336, 1.0, 209, 209, 0.5, 1476, 1.0);s.store_neg(1526, 1524);}
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1755])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1462), s.ad_value(1435)));s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1484), s.ad_value(1435)));s.store_mul_sqrt_mixed_ia(1476, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));s.store_div_scaled_product_indices(339, 209, 209, 0.5, 1476, 1.0);s.store_mul_add_mixed_iaa(1524, 339, A::add_scaled_product(s.ad_value(154), (-1.0), s.ad_value(154), s.ad_value(336), 1.0), A::mul3_scaled_output(s.ad_value(210), s.ad_value(154), s.ad_value(337), -1.0));s.store_mul_add_mixed_iaa(1526, 339, A::add_scaled_product(s.ad_value(154), 1.0, s.ad_value(154), s.ad_value(336), -1.0), A::mul3(s.ad_value(210), s.ad_value(154), s.ad_value(338)));}
            s.b[1756] = ((s.v[1520] > (s.v[1512] - s.v[1519])) && (s.v[1519] >= 0.0));s.store_scalar(1756, if s.b[1756] { 1.0 } else { 0.0 });
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) {s.store_add_scaled_inputs3_indices(781, 1520, 1.0, 1512, (-1.0), 1519, 1.0);s.store_square(722, 781);s.store_square(723, 1519);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
            let (t5e,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t5e);
            let (t5f,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t5f);
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
            s.b[1757] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1757, if s.b[1757] { 1.0 } else { 0.0 });s.b[1758] = (4.0 == 1.0);s.store_scalar(1758, if s.b[1758] { 1.0 } else { 0.0 });
            let (t61,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) && s.b[1758]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t61);s.b[1759] = (4.0 == 2.0);s.store_scalar(1759, if s.b[1759] { 1.0 } else { 0.0 });
            let (t62,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) && (!s.b[1758])) && s.b[1759]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t62);s.b[1760] = (4.0 == 4.0);s.store_scalar(1760, if s.b[1760] { 1.0 } else { 0.0 });
            let (t63,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) && (!s.b[1758])) && (!s.b[1759])) && s.b[1760]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t63);s.b[1761] = (4.0 == 8.0);s.store_scalar(1761, if s.b[1761] { 1.0 } else { 0.0 });
            let (t64,) = {
    if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) && (!s.b[1758])) && (!s.b[1759])) && (!s.b[1760])) && s.b[1761]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
            s.store_scalar(720, t64);
            let (t65,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t65);let mut t69: usize = 0;
            while {
                let t68: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
                t68 != 0.0
            } {
                t69 += 1;assert!(t69 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
                if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) {s.store_sqrt(726, 726);}
                let (t67,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && s.b[1757]) {
        let t66: f64 = (s.v[719] + 1.0);
        (t66,)
    } else {
        (s.v[719],)
    }
};
                s.store_scalar(719, t67);
            }
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) && (!s.b[1757])) {
                if (s.v[726] == 0.0) {
                    s.store_scalar(726, 0.0);
                } else {
                    s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
                }
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1519, 726);s.store_div_scaled_product3_indices(334, 1519, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs3_indices(335, 1512, 1.0, 1519, (-1.0), 780, 1.0);}
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1756]) {
            }
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1756])) {s.copy_ad(335, 1520);s.store_scalar(334, 1.0);}
            if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {s.store_sub(1485, 1484, 335);s.store_mul_scale_offset_indices(1487, 334, 1521, -1.0, 0.0);s.store_sub_from_scalar_ad(1488, 1.0, A::mul3(s.ad_value(1522), s.ad_value(1534), s.ad_value(334)));s.store_add_scaled_inputs3_mixed_aii(1486, A::add_scaled_product(s.ad_value(1476), 1.0, s.ad_value(185), A::sub(s.ad_value(85), s.ad_value(1462)), 1.0), 1.0, 1499, 1.0, 1500, 1.0);s.store_sub(1489, 1524, 185);s.store_add_scaled_inputs_products_indices(1490, 1526, 1.0, 1528, 1.0, 1530, 1534, 1.0, 1532, 1534, 1.0);s.store_add_scaled_products_indices(1491, 1487, 1490, 1.0, 1489, 1488, (-1.0));s.store_div(1492, 1490, 1491);s.store_div_scaled_inputs_indices(1493, 1488, -1.0, 1491, 1.0);s.store_div_scaled_inputs_indices(1494, 1489, -1.0, 1491, 1.0);s.store_div(1495, 1487, 1491);}
            s.b[1762] = (((((s.v[1494] * s.v[1485]) + (s.v[1495] * s.v[1486]))) as f64).abs() > 0.5);s.store_scalar(1762, if s.b[1762] { 1.0 } else { 0.0 });
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1762]) {s.store_offset(1462, 1462, (-(0.5 * (if (((s.v[1492] * s.v[1485]) + (s.v[1493] * s.v[1486])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1762]) {s.store_offset(1484, 1484, (-(0.5 * (if (((s.v[1494] * s.v[1485]) + (s.v[1495] * s.v[1486])) >= 0.0) { 1.0 } else { (-1.0) }))));}
            if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1762])) {s.store_sub_mixed_ia(1462, 1462, A::add_scaled_products(s.ad_value(1492), s.ad_value(1485), 1.0, s.ad_value(1493), s.ad_value(1486), 1.0));s.store_sub_mixed_ia(1484, 1484, A::add_scaled_products(s.ad_value(1494), s.ad_value(1485), 1.0, s.ad_value(1495), s.ad_value(1486), 1.0));}
            s.b[1763] = (((((s.v[1462] - s.v[1474])) as f64).abs() <= 1e-12) && ((((s.v[1484] - s.v[1471])) as f64).abs() <= 1e-12));s.store_scalar(1763, if s.b[1763] { 1.0 } else { 0.0 });
            let (t6b,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1763]) {
        let t6a: f64 = (150.0 + 1.0);
        (t6a,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t6b);
            let (t6c,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1763]) {
        (1.0,)
    } else {
        (s.v[79],)
    }
};
            s.store_scalar(79, t6c);
            let (t6d,) = {
    if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
        (s.v[1462],)
    } else {
        (s.v[1474],)
    }
};
            s.store_scalar(1474, t6d);
            let (t6e,) = {
    if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
        (s.v[1484],)
    } else {
        (s.v[1471],)
    }
};
            s.store_scalar(1471, t6e);
            let (t70,) = {
    if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {
        let t6f: f64 = (s.v[97] + 1.0);
        (t6f,)
    } else {
        (s.v[97],)
    }
};
            s.store_scalar(97, t70);
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_56(
        s: &mut Scratch,
    ) {
        s.b[1765] = ((s.v[1454] > s.v[965]) && (s.v[1479] != 2.0));s.store_scalar(1765, if s.b[1765] { 1.0 } else { 0.0 });s.b[1766] = ((s.v[1484] > (s.v[1462] - 0.02)) && (0.02 >= 0.0));s.store_scalar(1766, if s.b[1766] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) {s.store_offset_sub(781, 1484, 1462, 0.02);s.store_square(722, 781);s.store_scalar(723, (0.02 * 0.02));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t89,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t89);
        let (t8a,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t8a);
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1767] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1767, if s.b[1767] { 1.0 } else { 0.0 });s.b[1768] = (2.0 == 1.0);s.store_scalar(1768, if s.b[1768] { 1.0 } else { 0.0 });
        let (t8b,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) && s.b[1767]) && s.b[1768]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t8b);s.b[1769] = (2.0 == 2.0);s.store_scalar(1769, if s.b[1769] { 1.0 } else { 0.0 });
        let (t8c,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) && s.b[1767]) && (!s.b[1768])) && s.b[1769]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t8c);s.b[1770] = (2.0 == 4.0);s.store_scalar(1770, if s.b[1770] { 1.0 } else { 0.0 });
        let (t8d,) = {
    if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) && s.b[1767]) && (!s.b[1768])) && (!s.b[1769])) && s.b[1770]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t8d);s.b[1771] = (2.0 == 8.0);s.store_scalar(1771, if s.b[1771] { 1.0 } else { 0.0 });
        let (t8e,) = {
    if (((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) && s.b[1767]) && (!s.b[1768])) && (!s.b[1769])) && (!s.b[1770])) && s.b[1771]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t8e);
        let (t8f,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) && s.b[1767]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t8f);let mut t93: usize = 0;
        while {
            let t92: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) && s.b[1767]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t92 != 0.0
        } {
            t93 += 1;assert!(t93 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) && s.b[1767]) {s.store_sqrt(726, 726);}
            let (t91,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) && s.b[1767]) {
        let t90: f64 = (s.v[719] + 1.0);
        (t90,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t91);
        }
        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) && (!s.b[1767])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.02);s.store_div_scaled_product_indices(335, 725, 726, 0.02, 770, 1.0);s.store_add_offset_lhs(1484, 1462, (-0.02), 780);}
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && s.b[1766]) {
        }
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && (!s.b[1766])) {
        }
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1765]) && (!s.b[1766])) {s.store_scalar(335, 1.0);}
        if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {s.store_mul_sub_mixed_iai(1465, 1535, A::add_scaled_product(s.ad_value(1435), 1.0, s.ad_value(1536), s.ad_value(1484), 1.0), 1463);s.store_mul_sub_rhs(335, 154, 1462, 1484);s.store_exp(336, 335);}
        s.b[1772] = (s.v[1462] >= s.v[1484]);s.store_scalar(1772, if s.b[1772] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) {s.store_mul_scaled_sqrt_ad_rhs(1476, 209, -1.0, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));s.copy_ad(1539, 1476);s.store_scalar(1518, 0.0);s.store_scalar(1478, 0.0);s.store_sqrt_mul_sub_rhs(1448, 1547, 1484, 1465);}
        s.b[1773] = ((s.v[1448] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1773, if s.b[1773] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) {s.store_offset_sub(781, 1448, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t94,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t94);
        let (t95,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t95);
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_57(
        s: &mut Scratch,
    ) {
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) {s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1774] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1774, if s.b[1774] { 1.0 } else { 0.0 });s.b[1775] = (2.0 == 1.0);s.store_scalar(1775, if s.b[1775] { 1.0 } else { 0.0 });
        let (t96,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) && s.b[1774]) && s.b[1775]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t96);s.b[1776] = (2.0 == 2.0);s.store_scalar(1776, if s.b[1776] { 1.0 } else { 0.0 });
        let (t97,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) && s.b[1774]) && (!s.b[1775])) && s.b[1776]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t97);s.b[1777] = (2.0 == 4.0);s.store_scalar(1777, if s.b[1777] { 1.0 } else { 0.0 });
        let (t98,) = {
    if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) && s.b[1774]) && (!s.b[1775])) && (!s.b[1776])) && s.b[1777]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t98);s.b[1778] = (2.0 == 8.0);s.store_scalar(1778, if s.b[1778] { 1.0 } else { 0.0 });
        let (t99,) = {
    if (((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) && s.b[1774]) && (!s.b[1775])) && (!s.b[1776])) && (!s.b[1777])) && s.b[1778]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, t99);
        let (t9a,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) && s.b[1774]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t9a);let mut t9e: usize = 0;
        while {
            let t9d: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) && s.b[1774]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t9d != 0.0
        } {
            t9e += 1;assert!(t9e <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) && s.b[1774]) {s.store_sqrt(726, 726);}
            let (t9c,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) && s.b[1774]) {
        let t9b: f64 = (s.v[719] + 1.0);
        (t9b,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, t9c);
        }
        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) && (!s.b[1774])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1448, 965, (-1e-8), 780);}
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && s.b[1773]) {
        }
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && (!s.b[1773])) {
        }
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) && (!s.b[1773])) {s.store_scalar(337, 1.0);}
        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1772]) {s.store_sqrt_mul_ad(1452, s.ad_value(1550), A::add_scaled_inputs3(s.ad_value(1465), 1.0, s.ad_value(1435), (-1.0), s.ad_value(1463), 1.0));s.store_mul(1499, 1448, 1546);s.store_mul_scale_offset_indices(1500, 1544, 1452, -1.0, 0.0);}
        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) {s.store_exp_mul_scaled_lhs_mixed_ia(337, 154, -1.0, A::sub(s.ad_value(1462), s.ad_value(1435)));s.store_exp_mul_scaled_lhs_mixed_ia(338, 154, -1.0, A::sub(s.ad_value(1484), s.ad_value(1435)));s.store_mul_sqrt_mixed_ia(1476, 209, A::offset(A::add_scaled_inputs_product(A::offset(s.ad_value(336), (-1.0)), 1.0, s.ad_value(335), (-1.0), s.ad_value(210), A::sub(s.ad_value(337), s.ad_value(338)), 1.0), 1e-15));}
        s.b[1779] = ((s.v[1454] > s.v[965]) && (s.v[1479] != 2.0));s.store_scalar(1779, if s.b[1779] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1779]) {s.store_scalar(1478, 0.0);s.store_scalar(1518, 0.0);}
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && (!s.b[1779])) {s.store_mul_sqrt_mixed_ia(337, 209, A::sub(A::mul(s.ad_value(210), A::sub(A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1462), s.ad_value(1435)))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, A::sub(s.ad_value(1484), s.ad_value(1435)))))), s.ad_value(335)));s.store_add_scaled_product_mixed_iia(1478, 337, 1.0, 209, A::sqrt_scaled_input(s.ad_value(335), -1.0), (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_58(
        s: &mut Scratch,
    ) {
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && (!s.b[1779])) {s.store_mul_sqrt_mixed_ia(1518, 209, A::offset(A::sub(A::offset(s.ad_value(336), (-1.0)), s.ad_value(335)), 1e-15));}
        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) {s.store_scalar(1539, 0.0);s.store_sub(335, 1484, 1465);}
        s.b[1780] = ((s.v[335] < 0.1) && (0.1 >= 0.0));s.store_scalar(1780, if s.b[1780] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) {s.store_sub_from_scalar(781, 0.1, 335);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (t9f,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, t9f);
        let (ta0,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, ta0);
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1781] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1781, if s.b[1781] { 1.0 } else { 0.0 });s.b[1782] = (2.0 == 1.0);s.store_scalar(1782, if s.b[1782] { 1.0 } else { 0.0 });
        let (ta1,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) && s.b[1781]) && s.b[1782]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, ta1);s.b[1783] = (2.0 == 2.0);s.store_scalar(1783, if s.b[1783] { 1.0 } else { 0.0 });
        let (ta2,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) && s.b[1781]) && (!s.b[1782])) && s.b[1783]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, ta2);s.b[1784] = (2.0 == 4.0);s.store_scalar(1784, if s.b[1784] { 1.0 } else { 0.0 });
        let (ta3,) = {
    if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) && s.b[1781]) && (!s.b[1782])) && (!s.b[1783])) && s.b[1784]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, ta3);s.b[1785] = (2.0 == 8.0);s.store_scalar(1785, if s.b[1785] { 1.0 } else { 0.0 });
        let (ta4,) = {
    if (((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) && s.b[1781]) && (!s.b[1782])) && (!s.b[1783])) && (!s.b[1784])) && s.b[1785]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, ta4);
        let (ta5,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) && s.b[1781]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, ta5);let mut ta9: usize = 0;
        while {
            let ta8: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) && s.b[1781]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            ta8 != 0.0
        } {
            ta9 += 1;assert!(ta9 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) && s.b[1781]) {s.store_sqrt(726, 726);}
            let (ta7,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) && s.b[1781]) {
        let ta6: f64 = (s.v[719] + 1.0);
        (ta6,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, ta7);
        }
        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) && (!s.b[1781])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_sub_from_scalar(336, 0.1, 780);}
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1780]) {
        }
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && (!s.b[1780])) {s.copy_ad(336, 335);s.store_scalar(334, 1.0);}
        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) {s.store_sqrt_mul(1448, 1547, 336);}
        s.b[1786] = ((s.v[1448] > (s.v[965] - 1e-8)) && (1e-8 >= 0.0));s.store_scalar(1786, if s.b[1786] { 1.0 } else { 0.0 });
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) {s.store_offset_sub(781, 1448, 965, 1e-8);s.store_square(722, 781);s.store_scalar(723, (1e-8 * 1e-8));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (taa,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, taa);
        let (tab,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tab);
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1787] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1787, if s.b[1787] { 1.0 } else { 0.0 });s.b[1788] = (2.0 == 1.0);s.store_scalar(1788, if s.b[1788] { 1.0 } else { 0.0 });
        let (tac,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) && s.b[1787]) && s.b[1788]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tac);s.b[1789] = (2.0 == 2.0);s.store_scalar(1789, if s.b[1789] { 1.0 } else { 0.0 });
        let (tad,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) && s.b[1787]) && (!s.b[1788])) && s.b[1789]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tad);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_59(
        s: &mut Scratch,
    ) {
        s.b[1790] = (2.0 == 4.0);s.store_scalar(1790, if s.b[1790] { 1.0 } else { 0.0 });
        let (tae,) = {
    if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) && s.b[1787]) && (!s.b[1788])) && (!s.b[1789])) && s.b[1790]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tae);s.b[1791] = (2.0 == 8.0);s.store_scalar(1791, if s.b[1791] { 1.0 } else { 0.0 });
        let (taf,) = {
    if (((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) && s.b[1787]) && (!s.b[1788])) && (!s.b[1789])) && (!s.b[1790])) && s.b[1791]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, taf);
        let (tb0,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) && s.b[1787]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tb0);let mut tb4: usize = 0;
        while {
            let tb3: f64 = if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) && s.b[1787]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tb3 != 0.0
        } {
            tb4 += 1;assert!(tb4 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) && s.b[1787]) {s.store_sqrt(726, 726);}
            let (tb2,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) && s.b[1787]) {
        let tb1: f64 = (s.v[719] + 1.0);
        (tb1,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tb2);
        }
        if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) && (!s.b[1787])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 1e-8);s.store_div_scaled_product_indices(337, 725, 726, 1e-8, 770, 1.0);s.store_add_offset_lhs(1448, 965, (-1e-8), 780);}
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && s.b[1786]) {
        }
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && (!s.b[1786])) {
        }
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) && (!s.b[1786])) {s.store_scalar(337, 1.0);}
        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1772])) {s.store_sqrt_mul_ad(1452, s.ad_value(1550), A::add_scaled_inputs3(s.ad_value(1465), 1.0, s.ad_value(1435), (-1.0), s.ad_value(1463), 1.0));s.store_mul(1499, 1448, 1546);s.store_mul_scale_offset_indices(1500, 1544, 1452, -1.0, 0.0);}
        s.b[1792] = (((s.v[1462] - s.v[1512]) < 0.06) && (0.06 >= 0.0));s.store_scalar(1792, if s.b[1792] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) {s.store_sub_from_scalar_ad(781, 0.06, A::sub(s.ad_value(1462), s.ad_value(1512)));s.store_square(722, 781);s.store_scalar(723, (0.06 * 0.06));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tb5,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tb5);
        let (tb6,) = {
    if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tb6);
        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1793] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1793, if s.b[1793] { 1.0 } else { 0.0 });s.b[1794] = (2.0 == 1.0);s.store_scalar(1794, if s.b[1794] { 1.0 } else { 0.0 });
        let (tb7,) = {
    if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) && s.b[1793]) && s.b[1794]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tb7);s.b[1795] = (2.0 == 2.0);s.store_scalar(1795, if s.b[1795] { 1.0 } else { 0.0 });
        let (tb8,) = {
    if ((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) && s.b[1793]) && (!s.b[1794])) && s.b[1795]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tb8);s.b[1796] = (2.0 == 4.0);s.store_scalar(1796, if s.b[1796] { 1.0 } else { 0.0 });
        let (tb9,) = {
    if (((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) && s.b[1793]) && (!s.b[1794])) && (!s.b[1795])) && s.b[1796]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tb9);s.b[1797] = (2.0 == 8.0);s.store_scalar(1797, if s.b[1797] { 1.0 } else { 0.0 });
        let (tba,) = {
    if ((((((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) && s.b[1793]) && (!s.b[1794])) && (!s.b[1795])) && (!s.b[1796])) && s.b[1797]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tba);
        let (tbb,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) && s.b[1793]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tbb);let mut tbf: usize = 0;
        while {
            let tbe: f64 = if (((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) && s.b[1793]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tbe != 0.0
        } {
            tbf += 1;assert!(tbf <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) && s.b[1793]) {s.store_sqrt(726, 726);}
            let (tbd,) = {
    if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) && s.b[1793]) {
        let tbc: f64 = (s.v[719] + 1.0);
        (tbc,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tbd);
        }
        if ((((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) && (!s.b[1793])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.06);s.store_div_scaled_product_indices(334, 725, 726, 0.06, 770, 1.0);s.store_sub_from_scalar(336, 0.06, 780);}
        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && s.b[1792]) {
        }
        if (((s.b[1443] && s.b[1444]) && (!s.b[1711])) && (!s.b[1792])) {s.store_sub(336, 1462, 1512);s.store_scalar(334, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_60(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1443] && s.b[1444]) && (!s.b[1711])) {s.store_offset_add_scaled_product_mixed_aii(338, A::offset(A::exp(A::mul(s.ad_value(154), s.ad_value(336))), (-1.0)), 1.0, 154, 336, (-1.0), (10.0 * 2.220446049250313e-16));s.store_mul_scaled_sqrt_rhs(1516, 209, -1.0, 338);}
        if (s.b[1443] && s.b[1444]) {s.copy_ad(87, 1461);s.copy_ad(91, 1462);s.store_sub(94, 1462, 1461);s.store_neg_add(335, 1475, 1476);}
        s.b[1798] = ((s.v[335] < s.v[1540]) && (s.v[1540] >= 0.0));s.store_scalar(1798, if s.b[1798] { 1.0 } else { 0.0 });
        if ((s.b[1443] && s.b[1444]) && s.b[1798]) {s.store_sub(781, 1540, 335);s.store_square(722, 781);s.store_square(723, 1540);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tc0,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1798]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tc0);
        let (tc1,) = {
    if ((s.b[1443] && s.b[1444]) && s.b[1798]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc1);
        if ((s.b[1443] && s.b[1444]) && s.b[1798]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1799] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1799, if s.b[1799] { 1.0 } else { 0.0 });s.b[1800] = (2.0 == 1.0);s.store_scalar(1800, if s.b[1800] { 1.0 } else { 0.0 });
        let (tc2,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1798]) && s.b[1799]) && s.b[1800]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc2);s.b[1801] = (2.0 == 2.0);s.store_scalar(1801, if s.b[1801] { 1.0 } else { 0.0 });
        let (tc3,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1798]) && s.b[1799]) && (!s.b[1800])) && s.b[1801]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc3);s.b[1802] = (2.0 == 4.0);s.store_scalar(1802, if s.b[1802] { 1.0 } else { 0.0 });
        let (tc4,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1798]) && s.b[1799]) && (!s.b[1800])) && (!s.b[1801])) && s.b[1802]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc4);s.b[1803] = (2.0 == 8.0);s.store_scalar(1803, if s.b[1803] { 1.0 } else { 0.0 });
        let (tc5,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1798]) && s.b[1799]) && (!s.b[1800])) && (!s.b[1801])) && (!s.b[1802])) && s.b[1803]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tc5);
        let (tc6,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1798]) && s.b[1799]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tc6);let mut tca: usize = 0;
        while {
            let tc9: f64 = if ((((s.b[1443] && s.b[1444]) && s.b[1798]) && s.b[1799]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tc9 != 0.0
        } {
            tca += 1;assert!(tca <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.b[1443] && s.b[1444]) && s.b[1798]) && s.b[1799]) {s.store_sqrt(726, 726);}
            let (tc8,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1798]) && s.b[1799]) {
        let tc7: f64 = (s.v[719] + 1.0);
        (tc7,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tc8);
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1798]) && (!s.b[1799])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((s.b[1443] && s.b[1444]) && s.b[1798]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 1540, 726);s.store_div_scaled_product3_indices(334, 1540, 725, 726, 1.0, 770, 1.0);s.store_sub(1556, 1540, 780);}
        if ((s.b[1443] && s.b[1444]) && s.b[1798]) {
        }
        if ((s.b[1443] && s.b[1444]) && (!s.b[1798])) {s.copy_ad(1556, 335);s.store_scalar(334, 1.0);}
        if (s.b[1443] && s.b[1444]) {s.store_mul3_affine_lhs(1503, 154, 1556, 1.0 / (2.0), 0.0, 94);s.store_sub(1504, 1516, 1515);s.store_add(248, 1503, 1504);s.store_neg(133, 1515);s.copy_ad(170, 162);s.store_scalar(336, (s.v[626] / 100.0));s.store_offset_mul_ad(338, A::sub(s.ad_value(1462), s.ad_value(1461)), s.ad_value(682), 1.0);s.store_mul(339, 336, 133);s.store_div(337, 339, 338);s.copy_ad(251, 337);}
        if (s.b[1443] && s.b[1444]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p160 - 1.0));
            }
        }
        if (s.b[1443] && s.b[1444]) {s.store_mul(342, 339, 251);}
        if (s.b[1443] && s.b[1444]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(341, 0.0);
            } else {
                s.store_pow_offset_rhs(341, 251, 624, (-1.0));
            }
        }
        if (s.b[1443] && s.b[1444]) {s.store_mul(340, 341, 251);s.store_scalar(343, (1.6021918e-19 * 10000.0));s.store_div(252, 133, 343);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_61(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[1443] && s.b[1444]) {s.store_add_scaled_inputs_mixed_ai(335, A::add_scaled_product(A::div_from_scalar(1.0, A::scale_offset(s.ad_value(252), (s.v[475] * 1e-11), ((s.v[474]) + (1e-25)))), 1.0, s.ad_value(679), s.ad_value(342), 1.0), 1.0, 340, 1.0 / (s.v[479]));s.store_div_from_scalar(254, 1.0, 335);s.store_scale(254, 254, 0.0001);s.store_mul_ad_product_lhs_mixed_ia(336, 154, A::offset(s.ad_value(133), 1e-25), 170);s.store_div_from_scalar(335, 1.0, 336);s.store_mul(333, 248, 335);s.store_div_scaled_inputs_indices(336, 257, 0.2, 254, 1.0);s.store_sqrt_square_sum(255, 333, 336);s.store_div_from_scalar(338, 1.0, 255);s.store_mul(256, 254, 255);s.store_div(335, 256, 257);s.copy_ad(1558, 255);}
        s.b[1804] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1804, if s.b[1804] { 1.0 } else { 0.0 });
        if ((s.b[1443] && s.b[1444]) && s.b[1804]) {s.store_scalar(337, 1.0);}
        s.b[1805] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1805, if s.b[1805] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && (!s.b[1804])) && s.b[1805]) {s.copy_ad(337, 335);}
        if (((s.b[1443] && s.b[1444]) && (!s.b[1804])) && (!s.b[1805])) {
            if (s.v[335] == 0.0) {
                s.store_scalar(337, 0.0);
            } else {
                s.store_powf(337, 335, (p.p178 - 1.0));
            }
        }
        if (s.b[1443] && s.b[1444]) {s.store_mul(336, 335, 337);s.store_offset(338, 336, 1.0);}
        s.b[1806] = (((1.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (1.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1806, if s.b[1806] { 1.0 } else { 0.0 });
        if ((s.b[1443] && s.b[1444]) && s.b[1806]) {s.store_div_from_scalar(339, 1.0, 338);}
        s.b[1807] = (((2.0 - (10.0 * 2.220446049250313e-16)) <= p.p178) && (p.p178 <= (2.0 + (10.0 * 2.220446049250313e-16))));s.store_scalar(1807, if s.b[1807] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && (!s.b[1806])) && s.b[1807]) {s.store_div_from_scalar_sqrt_ad(339, 1.0, s.ad_value(338));}
        if (((s.b[1443] && s.b[1444]) && (!s.b[1806])) && (!s.b[1807])) {
            if (s.v[338] == 0.0) {
                s.store_scalar(340, 0.0);
            } else {
                s.store_powf(340, 338, (((-1.0) / p.p178) - 1.0));
            }
        }
        if (((s.b[1443] && s.b[1444]) && (!s.b[1806])) && (!s.b[1807])) {s.store_mul(339, 338, 340);}
        if (s.b[1443] && s.b[1444]) {s.store_mul(253, 254, 339);}
        s.b[1808] = (s.v[349] > 1e-6);s.store_scalar(1808, if s.b[1808] { 1.0 } else { 0.0 });
        if ((s.b[1443] && s.b[1444]) && s.b[1808]) {s.store_div_square_rhs(336, 1502, 185);s.store_add_scaled_inputs4_indices(334, 85, 1.0, 974, 1.0, 155, -1.0, 1438, -1.0);s.store_offset_mul_ad(338, A::div_from_scalar(2.0, s.ad_value(336)), s.ad_value(334), 1.0);}
        s.b[1809] = ((s.v[338] < 2.0) && (2.0 >= 0.0));s.store_scalar(1809, if s.b[1809] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) {s.store_sub_from_scalar(781, 2.0, 338);s.store_square(722, 781);s.store_scalar(723, (2.0 * 2.0));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (tcb,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tcb);
        let (tcc,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tcc);
        if (((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1810] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(1810, if s.b[1810] { 1.0 } else { 0.0 });s.b[1811] = (2.0 == 1.0);s.store_scalar(1811, if s.b[1811] { 1.0 } else { 0.0 });
        let (tcd,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) && s.b[1810]) && s.b[1811]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tcd);s.b[1812] = (2.0 == 2.0);s.store_scalar(1812, if s.b[1812] { 1.0 } else { 0.0 });
        let (tce,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) && s.b[1810]) && (!s.b[1811])) && s.b[1812]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tce);s.b[1813] = (2.0 == 4.0);s.store_scalar(1813, if s.b[1813] { 1.0 } else { 0.0 });
        let (tcf,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) && s.b[1810]) && (!s.b[1811])) && (!s.b[1812])) && s.b[1813]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tcf);s.b[1814] = (2.0 == 8.0);s.store_scalar(1814, if s.b[1814] { 1.0 } else { 0.0 });
        let (td0,) = {
    if ((((((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) && s.b[1810]) && (!s.b[1811])) && (!s.b[1812])) && (!s.b[1813])) && s.b[1814]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, td0);
        let (td1,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) && s.b[1810]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, td1);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_62(
        s: &mut Scratch,
    ) {
        let mut td5: usize = 0;
        while {
            let td4: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) && s.b[1810]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            td4 != 0.0
        } {
            td5 += 1;assert!(td5 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) && s.b[1810]) {s.store_sqrt(726, 726);}
            let (td3,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) && s.b[1810]) {
        let td2: f64 = (s.v[719] + 1.0);
        (td2,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, td3);
        }
        if ((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) && (!s.b[1810])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 2.0);s.store_div_scaled_product_indices(334, 725, 726, 2.0, 770, 1.0);s.store_sub_from_scalar(343, 2.0, 780);}
        if (((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1809]) {
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1808]) && (!s.b[1809])) {s.copy_ad(343, 338);s.store_scalar(334, 1.0);}
        if ((s.b[1443] && s.b[1444]) && s.b[1808]) {s.store_offset(343, 343, 1e-25);s.store_sqrt(337, 343);s.store_mul_scale_offset_indices(338, 336, 337, -1.0, 1.0);s.store_add_scaled_inputs3_indices(344, 85, 1.0, 974, 1.0, 338, 1.0);s.store_mul(344, 344, 975);}
        s.b[1815] = ((s.v[344] < (s.v[972] + 4.0)) && (4.0 >= 0.0));s.store_scalar(1815, if s.b[1815] { 1.0 } else { 0.0 });
        if (((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) {s.store_sub_offset_lhs(781, 972, 4.0, 344);s.store_square(722, 781);s.store_scalar(723, (4.0 * 4.0));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);}
        let (td6,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, td6);
        let (td7,) = {
    if (((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) {
        (0.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, td7);
        if (((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) {s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[1816] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(1816, if s.b[1816] { 1.0 } else { 0.0 });s.b[1817] = (4.0 == 1.0);s.store_scalar(1817, if s.b[1817] { 1.0 } else { 0.0 });
        let (td8,) = {
    if (((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) && s.b[1816]) && s.b[1817]) {
        (1.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, td8);s.b[1818] = (4.0 == 2.0);s.store_scalar(1818, if s.b[1818] { 1.0 } else { 0.0 });
        let (td9,) = {
    if ((((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) && s.b[1816]) && (!s.b[1817])) && s.b[1818]) {
        (2.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, td9);s.b[1819] = (4.0 == 4.0);s.store_scalar(1819, if s.b[1819] { 1.0 } else { 0.0 });
        let (tda,) = {
    if (((((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) && s.b[1816]) && (!s.b[1817])) && (!s.b[1818])) && s.b[1819]) {
        (3.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tda);s.b[1820] = (4.0 == 8.0);s.store_scalar(1820, if s.b[1820] { 1.0 } else { 0.0 });
        let (tdb,) = {
    if ((((((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) && s.b[1816]) && (!s.b[1817])) && (!s.b[1818])) && (!s.b[1819])) && s.b[1820]) {
        (4.0,)
    } else {
        (s.v[720],)
    }
};
        s.store_scalar(720, tdb);
        let (tdc,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) && s.b[1816]) {
        (0.0,)
    } else {
        (s.v[719],)
    }
};
        s.store_scalar(719, tdc);let mut te0: usize = 0;
        while {
            let tdf: f64 = if (((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) && s.b[1816]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tdf != 0.0
        } {
            te0 += 1;assert!(te0 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) && s.b[1816]) {s.store_sqrt(726, 726);}
            let (tde,) = {
    if ((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) && s.b[1816]) {
        let tdd: f64 = (s.v[719] + 1.0);
        (tdd,)
    } else {
        (s.v[719],)
    }
};
            s.store_scalar(719, tde);
        }
        if ((((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) && (!s.b[1816])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 4.0);s.store_div_scaled_product_indices(334, 725, 726, 4.0, 770, 1.0);s.store_sub_offset_lhs(344, 972, 4.0, 780);}
        if (((s.b[1443] && s.b[1444]) && s.b[1808]) && s.b[1815]) {
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1808]) && (!s.b[1815])) {
        }
        if (((s.b[1443] && s.b[1444]) && s.b[1808]) && (!s.b[1815])) {s.store_scalar(334, 1.0);}
        if ((s.b[1443] && s.b[1444]) && s.b[1808]) {s.store_div(335, 349, 344);}
        if ((s.b[1443] && s.b[1444]) && s.b[1808]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_pow_offset_rhs(336, 335, 658, (-1.0));
            }
        }
        if ((s.b[1443] && s.b[1444]) && s.b[1808]) {s.store_mul(341, 336, 335);s.store_offset(337, 341, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_63(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[1443] && s.b[1444]) && s.b[1808]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_pow_ad(338, s.ad_value(337), A::offset(A::div_from_scalar(1.0, s.ad_value(658)), (-1.0)));
            }
        }
        if ((s.b[1443] && s.b[1444]) && s.b[1808]) {s.store_mul(340, 338, 337);s.store_div(1557, 349, 340);}
        if ((s.b[1443] && s.b[1444]) && (!s.b[1808])) {s.copy_ad(1557, 349);}
        if (s.b[1443] && s.b[1444]) {s.store_offset_mul_ad(338, A::sub(s.ad_value(1462), s.ad_value(1461)), s.ad_value(682), 1.0);s.store_neg(133, 1496);s.copy_ad(339, 133);s.store_div(337, 339, 338);s.copy_ad(251, 337);}
        if (s.b[1443] && s.b[1444]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p376 - 1.0));
            }
        }
        if (s.b[1443] && s.b[1444]) {s.store_mul(342, 339, 251);s.store_scalar(343, (1.6021918e-19 * 10000.0));s.store_div(252, 133, 343);s.store_add_scaled_product_mixed_aii(335, A::div_scalar_offset_denominator(1.0, A::add_scaled_product(s.ad_value(966), 1.0, s.ad_value(968), s.ad_value(252), 1e-11), 1e-25, 1.0), 1.0, 977, 342, 1.0);s.store_div_from_scalar(254, 1.0, 335);s.store_scale(254, 254, 0.0001);s.store_div(1555, 1557, 170);s.store_div_scaled_product_indices(335, 254, 1555, 1.0, 973, 1.0);}
        if (s.b[1443] && s.b[1444]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p.p378);
            }
        }
        if (s.b[1443] && s.b[1444]) {s.store_offset(337, 336, 1.0);}
        if (s.b[1443] && s.b[1444]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, (1.0 / p.p378));
            }
        }
        if (s.b[1443] && s.b[1444]) {s.store_div(1506, 254, 338);s.store_mul3_affine_lhs(987, 1496, 1506, (-s.v[632]), 0.0, 1555);s.store_offset_mul_ad(338, A::sub(s.ad_value(1462), s.ad_value(1461)), s.ad_value(682), 1.0);s.store_neg(133, 1505);s.copy_ad(339, 133);s.store_div(337, 339, 338);s.copy_ad(251, 337);}
        if (s.b[1443] && s.b[1444]) {
            if (s.v[251] == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf(339, 251, (p.p376 - 1.0));
            }
        }
        if (s.b[1443] && s.b[1444]) {s.store_mul(342, 339, 251);s.store_scalar(343, (1.6021918e-19 * 10000.0));s.store_div(252, 133, 343);s.store_add_scaled_product_mixed_aii(335, A::div_scalar_offset_denominator(1.0, A::add_scaled_product(s.ad_value(967), 1.0, s.ad_value(969), s.ad_value(252), 1e-11), 1e-25, 1.0), 1.0, 977, 342, 1.0);s.store_div_from_scalar(254, 1.0, 335);s.store_scale(254, 254, 0.0001);s.store_div(1555, 1557, 170);s.store_div_scaled_product_indices(335, 254, 1555, 1.0, 973, 1.0);}
        if (s.b[1443] && s.b[1444]) {
            if (s.v[335] == 0.0) {
                s.store_scalar(336, 0.0);
            } else {
                s.store_powf(336, 335, p.p378);
            }
        }
        if (s.b[1443] && s.b[1444]) {s.store_offset(337, 336, 1.0);}
        if (s.b[1443] && s.b[1444]) {
            if (s.v[337] == 0.0) {
                s.store_scalar(338, 0.0);
            } else {
                s.store_powf(338, 337, (1.0 / p.p378));
            }
        }
        if (s.b[1443] && s.b[1444]) {s.store_div(1507, 254, 338);s.store_mul3_affine_lhs(1554, 1505, 1507, (-s.v[632]), 0.0, 1555);s.store_div_scaled_inputs_indices(115, 155, s.v[632], 170, 1.0);s.store_add_scaled_inputs3_mixed_aii(135, A::mul3(s.ad_value(115), s.ad_value(248), s.ad_value(253)), 1.0, 987, 1.0, 1554, 1.0);s.store_mul3_lhs(986, 115, 248, 253);s.copy_ad(984, 253);s.copy_ad(790, 349);}
        s.b[1821] = (p.p283 != 0.0);s.store_scalar(1821, if s.b[1821] { 1.0 } else { 0.0 });
        if ((s.b[1443] && s.b[1444]) && s.b[1821]) {s.store_scaled_sub(335, 790, 94, 0.5);}
    }
}

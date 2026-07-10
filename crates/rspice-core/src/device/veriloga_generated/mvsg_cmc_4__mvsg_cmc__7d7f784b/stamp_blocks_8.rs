#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_128(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff1 == 0.0)) && (l.f1ff2 != 0.0)) {
            let t0: f64 = (-50.0);
            let (td, te,) = {
    if ((!(l.f1513 > 50.0)) && (!(l.f1513 < t0))) {
        let t1: f64 = (l.f1513).exp();
        (t1, (t1 * l.f1514),)
    } else {
        let t2: f64 = (-50.0);
        let (tb, tc,) = {
            if ((!(l.f1513 > 50.0)) && (l.f1513 < t2)) {
                let t3: f64 = (-50.0);let t4: f64 = (t3).exp();
                (t4, 0.0,)
            } else {
                let (t9, ta,) = {
                    if (l.f1513 > 50.0) {
                        let t5: f64 = (50.0_f64).exp();let t6: f64 = (l.f1513 - 50.0);let t7: f64 = (1.0 + t6);let t8: f64 = (t5 * t7);
                        (t8, (t5 * l.f1514),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t9, ta,)
            }
        };
        (tb, tc,)
    }
};
            (l.f1507, l.f1508, ) = (td, te, );
        }
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff1 == 0.0)) && (l.f1ff2 != 0.0)) {let tf: f64 = (l.f154f * l.f152d);let t10: f64 = (l.f1507 - tf);let t11: f64 = (t10 - l.f155c);(l.f1539, l.f153a, ) = (t11, ((l.f1508 - (l.f154f * l.f152e)) - l.f155d), );let t12: f64 = (l.f1554 / l.f1556);let t13: f64 = (t12 * l.f1562);let t14: f64 = (t13 + l.f151f);(l.f150f, l.f1510, l.f1511, l.f1512, ) = (t14, (((-((l.f1554 * l.f1557) / (l.f1556 * l.f1556))) * l.f1562) + l.f1520), (t12 * l.f1563), (t12 * l.f1564), );}
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff1 == 0.0)) && (l.f1ff2 != 0.0)) {
            let t15: f64 = (-50.0);
            let (t26, t27, t28, t29,) = {
    if ((!(l.f150f > 50.0)) && (!(l.f150f < t15))) {
        let t16: f64 = (l.f150f).exp();
        (t16, (t16 * l.f1510), (t16 * l.f1511), (t16 * l.f1512),)
    } else {
        let t17: f64 = (-50.0);
        let (t22, t23, t24, t25,) = {
            if ((!(l.f150f > 50.0)) && (l.f150f < t17)) {
                let t18: f64 = (-50.0);let t19: f64 = (t18).exp();
                (t19, 0.0, 0.0, 0.0,)
            } else {
                let (t1e, t1f, t20, t21,) = {
                    if (l.f150f > 50.0) {
                        let t1a: f64 = (50.0_f64).exp();let t1b: f64 = (l.f150f - 50.0);let t1c: f64 = (1.0 + t1b);let t1d: f64 = (t1a * t1c);
                        (t1d, (t1a * l.f1510), (t1a * l.f1511), (t1a * l.f1512),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t1e, t1f, t20, t21,)
            }
        };
        (t22, t23, t24, t25,)
    }
};
            (l.f1503, l.f1504, l.f1505, l.f1506, ) = (t26, t27, t28, t29, );
        }
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff1 == 0.0)) && (l.f1ff2 != 0.0)) {let t2a: f64 = (l.f154b * l.f153f);let t2b: f64 = (t2a / l.f1539);(l.f1537, l.f1538, ) = (t2b, (((((l.f154c * l.f153f) + (l.f154b * l.f1540)) * l.f1539) - (t2a * l.f153a)) / (l.f1539 * l.f1539)), );let t2c: f64 = (l.f154f * l.f1529);let t2d: f64 = (l.f1503 - t2c);let t2e: f64 = (t2d - l.f155c);let t2f: f64 = (l.f1537 * t2e);(l.f1533, l.f1534, l.f1535, l.f1536, ) = (t2f, ((l.f1538 * t2e) + (l.f1537 * ((l.f1504 - (l.f154f * l.f152a)) - l.f155d))), (l.f1537 * (l.f1505 - (l.f154f * l.f152b))), (l.f1537 * (l.f1506 - (l.f154f * l.f152c))), );}
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff1 == 0.0)) && (l.f1ff2 == 0.0)) {let t30: f64 = (l.f154b * l.f153f);(l.f1533, l.f1534, l.f1535, l.f1536, ) = (t30, ((l.f154c * l.f153f) + (l.f154b * l.f1540)), 0.0, 0.0, );}
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff1 == 0.0)) {let t31: f64 = (l.f14e9 * l.f14e9);let t32: f64 = (t31 * l.f1556);(l.f14e7, l.f14e8, ) = (t32, (t31 * l.f1557), );let t33: f64 = (l.f14e7 / 2.0);let t34: f64 = (l.f1565 - t33);let t35: f64 = (l.f1562 - t34);let t36: f64 = (t35 / l.f14e7);(l.f14fb, l.f14fc, l.f14fd, l.f14fe, ) = (t36, ((((-(-(l.f14e8 / 2.0))) * l.f14e7) - (t35 * l.f14e8)) / (l.f14e7 * l.f14e7)), (l.f1563 / l.f14e7), (l.f1564 / l.f14e7), );}
        let t37: f64 = if l.f14fb > 50.0 { 1.0 } else { 0.0 };l.f1ff3 = t37;
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff1 == 0.0)) && (l.f1ff3 != 0.0)) {(l.f1521, l.f1522, l.f1523, l.f1524, ) = (0.0, 0.0, 0.0, 0.0, );}
        let t38: f64 = (-50.0);let t39: f64 = if l.f14fb < t38 { 1.0 } else { 0.0 };l.f1ff4 = t39;
        if (((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff1 == 0.0)) && (l.f1ff3 == 0.0)) && (l.f1ff4 != 0.0)) {(l.f1521, l.f1522, l.f1523, l.f1524, ) = (1.0, 0.0, 0.0, 0.0, );}
        if (((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff1 == 0.0)) && (l.f1ff3 == 0.0)) && (l.f1ff4 == 0.0)) {let t3a: f64 = (l.f14fb).exp();let t3b: f64 = (1.0 + t3a);let t3c: f64 = (1.0 / t3b);(l.f1521, l.f1522, l.f1523, l.f1524, ) = (t3c, (-((t3a * l.f14fc) / (t3b * t3b))), (-((t3a * l.f14fd) / (t3b * t3b))), (-((t3a * l.f14fe) / (t3b * t3b))), );}
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff1 == 0.0)) {let t3d: f64 = (l.f1521 * l.f153b);let t3e: f64 = (1.0 - l.f1521);let t3f: f64 = (t3e * l.f1533);let t40: f64 = (t3d + t3f);(l.f152f, l.f1530, l.f1531, l.f1532, ) = (t40, (((l.f1522 * l.f153b) + (l.f1521 * l.f153c)) + (((-l.f1522) * l.f1533) + (t3e * l.f1534))), (((l.f1523 * l.f153b) + (l.f1521 * l.f153d)) + (((-l.f1523) * l.f1533) + (t3e * l.f1535))), (((l.f1524 * l.f153b) + (l.f1521 * l.f153e)) + (((-l.f1524) * l.f1533) + (t3e * l.f1536))), );}
        if ((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) {
            let t41: f64 = (-l.f1562);
            let (t50, t51, t52,) = {
    if (p.p52 != 0.0) {
        let t42: f64 = (l.f1562 / l.f1566);let t43: f64 = (0.001 / p.p53);let t44: f64 = (l.f1562 / l.f1566);let t45: f64 = (t43 * t44);let t46: f64 = (t45).tanh();let t47: f64 = (t42 * t46);
        (t47, (((l.f1563 / l.f1566) * t46) + (t42 * ((t43 * (l.f1563 / l.f1566)) / ((t45).cosh() * (t45).cosh())))), (((l.f1564 / l.f1566) * t46) + (t42 * ((t43 * (l.f1564 / l.f1566)) / ((t45).cosh() * (t45).cosh())))),)
    } else {
        let (t4d, t4e, t4f,) = {
            if (p.p52 == 0.0) {
                let __rspice_inv_cse_0: f64 = 1.0 / l.f1566;let t48: f64 = (l.f1562 * __rspice_inv_cse_0);let t49: f64 = (l.f1562 * __rspice_inv_cse_0);let t4a: f64 = (t48 * t49);let t4b: f64 = (t4a + p.p53);let t4c: f64 = (t4b).sqrt();
                (t4c, ((((l.f1563 / l.f1566) * t49) + (t48 * (l.f1563 / l.f1566))) / (2.0 * t4c)), ((((l.f1564 / l.f1566) * t49) + (t48 * (l.f1564 / l.f1566))) / (2.0 * t4c)),)
            } else {
                (0.0, 0.0, 0.0,)
            }
        };
        (t4d, t4e, t4f,)
    }
};
            let t53: f64 = (t50).powf(l.f14ea);let t54: f64 = (1.0 + t53);let t55: f64 = (1.0 / l.f14ea);let t56: f64 = (t54).powf(t55);let t57: f64 = (t41 / t56);(l.f1526, l.f1527, l.f1528, ) = (t57, ((((-l.f1563) * t56) - (t41 * if 0.0 == 0.0 && ((t55) as f64).is_finite() && ((t55) as f64).fract() == 0.0 { if t55 == 0.0 { 0.0 } else { (t55 * ((t54).powf(t55 - 1.0) * if 0.0 == 0.0 && ((l.f14ea) as f64).is_finite() && ((l.f14ea) as f64).fract() == 0.0 { if l.f14ea == 0.0 { 0.0 } else { (l.f14ea * ((t50).powf(l.f14ea - 1.0) * t51)) } } else { (t53 * (l.f14ea * (t51 / t50))) })) } } else { (t56 * (t55 * (if 0.0 == 0.0 && ((l.f14ea) as f64).is_finite() && ((l.f14ea) as f64).fract() == 0.0 { if l.f14ea == 0.0 { 0.0 } else { (l.f14ea * ((t50).powf(l.f14ea - 1.0) * t51)) } } else { (t53 * (l.f14ea * (t51 / t50))) } / t54))) })) / (t56 * t56)), ((((-l.f1564) * t56) - (t41 * if 0.0 == 0.0 && ((t55) as f64).is_finite() && ((t55) as f64).fract() == 0.0 { if t55 == 0.0 { 0.0 } else { (t55 * ((t54).powf(t55 - 1.0) * if 0.0 == 0.0 && ((l.f14ea) as f64).is_finite() && ((l.f14ea) as f64).fract() == 0.0 { if l.f14ea == 0.0 { 0.0 } else { (l.f14ea * ((t50).powf(l.f14ea - 1.0) * t52)) } } else { (t53 * (l.f14ea * (t52 / t50))) })) } } else { (t56 * (t55 * (if 0.0 == 0.0 && ((l.f14ea) as f64).is_finite() && ((l.f14ea) as f64).fract() == 0.0 { if l.f14ea == 0.0 { 0.0 } else { (l.f14ea * ((t50).powf(l.f14ea - 1.0) * t52)) } } else { (t53 * (l.f14ea * (t52 / t50))) } / t54))) })) / (t56 * t56)), );
        }
        if ((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) {let t58: f64 = (-l.f1560);let t59: f64 = (t58 * l.f1568);let t5a: f64 = (t59 * l.f1550);let t5b: f64 = (t5a * l.f154a);let t5c: f64 = (t5b * l.f155e);let t5d: f64 = t5c;(l.f154d, l.f154e, ) = (t5d, (t5b * l.f155f), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_129(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv8 = ctx.node_voltage(nodes[8]);let nv9 = ctx.node_voltage(nodes[9]);
        if ((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) {let t5e: f64 = (l.f1555 / l.f1556);let t5f: f64 = (t5e * l.f1526);(l.f151b, l.f151c, l.f151d, l.f151e, ) = (t5f, ((-((l.f1555 * l.f1557) / (l.f1556 * l.f1556))) * l.f1526), (t5e * l.f1527), (t5e * l.f1528), );}
        if ((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) {
            let t60: f64 = (-50.0);
            let (t71, t72, t73, t74,) = {
    if ((!(l.f151b > 50.0)) && (!(l.f151b < t60))) {
        let t61: f64 = (l.f151b).exp();
        (t61, (t61 * l.f151c), (t61 * l.f151d), (t61 * l.f151e),)
    } else {
        let t62: f64 = (-50.0);
        let (t6d, t6e, t6f, t70,) = {
            if ((!(l.f151b > 50.0)) && (l.f151b < t62)) {
                let t63: f64 = (-50.0);let t64: f64 = (t63).exp();
                (t64, 0.0, 0.0, 0.0,)
            } else {
                let (t69, t6a, t6b, t6c,) = {
                    if (l.f151b > 50.0) {
                        let t65: f64 = (50.0_f64).exp();let t66: f64 = (l.f151b - 50.0);let t67: f64 = (1.0 + t66);let t68: f64 = (t65 * t67);
                        (t68, (t65 * l.f151c), (t65 * l.f151d), (t65 * l.f151e),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t69, t6a, t6b, t6c,)
            }
        };
        (t6d, t6e, t6f, t70,)
    }
};
            (l.f1517, l.f1518, l.f1519, l.f151a, ) = (t71, t72, t73, t74, );
        }
        if ((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) {let t75: f64 = (l.f1517 - 1.0);let t76: f64 = (l.f154d * t75);(l.f1541, l.f1542, l.f1543, l.f1544, ) = (t76, ((l.f154e * t75) + (l.f154d * l.f1518)), (l.f154d * l.f1519), (l.f154d * l.f151a), );let t77: f64 = (l.f152f + l.f1541);(l.f1545, l.f1546, l.f1547, l.f1548, ) = (t77, (l.f1530 + l.f1542), (l.f1531 + l.f1543), (l.f1532 + l.f1544), );(l.f1558, l.f1559, l.f155a, l.f155b, ) = (l.f1545, l.f1546, l.f1547, l.f1548, );(l.f2128, l.f2129, l.f212a, l.f212b, ) = (l.f1558, l.f1559, l.f155a, l.f155b, );}
        let t78: f64 = if p.p282 == 1.0 { 1.0 } else { 0.0 };l.f1ff7 = t78;
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) {(l.f15da, l.f15db, l.f15dc, l.f15dd, ) = (0.0, 0.0, 0.0, 0.0, );(l.f15cd, l.f15ce, ) = (0.0, 0.0, );(l.f15cf, l.f15d0, ) = (0.0, 0.0, );let t79: f64 = (p.p6 * (nv8 - nv9));(l.f15e4, l.f15e5, l.f15e6, ) = (t79, p.p6, (-p.p6), );(l.f15d8, l.f15d9, ) = (l.f215b, l.f215c, );l.f15e7 = p.p260;l.f156b = p.p262;l.f15a7 = 1.0;l.f15d5 = p.p258;l.f15d3 = p.p278;l.f15e3 = p.p277;(l.f15e0, l.f15e1, ) = (l.f22f2, l.f22f3, );l.f15ea = p.p0;l.f15d2 = p.p2;l.f15cb = 0.0;l.f15d1 = 0.0;l.f15e8 = p.p285;l.f156c = p.p286;let t7a: f64 = (p.p255 * p.p284);l.f15cc = t7a;l.f15d7 = p.p283;l.f15d4 = p.p257;l.f15e9 = p.p256;l.f15e2 = p.p6;(l.f15c7, l.f15c8, l.f15c9, l.f15ca, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1569, l.f156a, ) = (0.0, 0.0, );(l.f15de, l.f15df, ) = (0.0, 0.0, );(l.f15a3, l.f15a4, l.f15a5, l.f15a6, ) = (0.0, 0.0, 0.0, 0.0, );(l.f15ab, l.f15ac, l.f15ad, l.f15ae, ) = (0.0, 0.0, 0.0, 0.0, );(l.f15b1, l.f15b2, l.f15b3, l.f15b4, ) = (0.0, 0.0, 0.0, 0.0, );(l.f15a8, l.f15a9, l.f15aa, ) = (0.0, 0.0, 0.0, );(l.f15c3, l.f15c4, l.f15c5, l.f15c6, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1575, l.f1576, l.f1577, l.f1578, ) = (0.0, 0.0, 0.0, 0.0, );(l.f157b, l.f157c, ) = (0.0, 0.0, );(l.f156d, l.f156e, l.f156f, l.f1570, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1573, l.f1574, ) = (0.0, 0.0, );(l.f15a1, l.f15a2, ) = (0.0, 0.0, );(l.f157d, l.f157e, l.f157f, l.f1580, ) = (0.0, 0.0, 0.0, 0.0, );(l.f158d, l.f158e, l.f158f, l.f1590, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1581, l.f1582, l.f1583, l.f1584, ) = (0.0, 0.0, 0.0, 0.0, );(l.f159d, l.f159e, l.f159f, l.f15a0, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1599, l.f159a, l.f159b, l.f159c, ) = (0.0, 0.0, 0.0, 0.0, );l.f15d6 = 0.0;(l.f1579, l.f157a, ) = (0.0, 0.0, );(l.f1571, l.f1572, ) = (0.0, 0.0, );(l.f15af, l.f15b0, ) = (0.0, 0.0, );(l.f1597, l.f1598, ) = (0.0, 0.0, );(l.f158b, l.f158c, ) = (0.0, 0.0, );(l.f15c1, l.f15c2, ) = (0.0, 0.0, );(l.f15bd, l.f15be, l.f15bf, l.f15c0, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1595, l.f1596, ) = (0.0, 0.0, );(l.f1589, l.f158a, ) = (0.0, 0.0, );(l.f15bb, l.f15bc, ) = (0.0, 0.0, );(l.f1591, l.f1592, l.f1593, l.f1594, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1585, l.f1586, l.f1587, l.f1588, ) = (0.0, 0.0, 0.0, 0.0, );(l.f15b9, l.f15ba, ) = (0.0, 0.0, );(l.f15b5, l.f15b6, l.f15b7, l.f15b8, ) = (0.0, 0.0, 0.0, 0.0, );let t7b: f64 = (l.f15d4 / l.f15d8);let t7c: f64 = (-l.f15e9);let t7d: f64 = (t7b * t7c);(l.f15a1, l.f15a2, ) = (t7d, ((-((l.f15d4 * l.f15d9) / (l.f15d8 * l.f15d8))) * t7c), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_130(
        l: &mut StampLocals,
    ) {
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) {
            let t7e: f64 = (-50.0);
            let (t8b, t8c,) = {
    if ((!(l.f15a1 > 50.0)) && (!(l.f15a1 < t7e))) {
        let t7f: f64 = (l.f15a1).exp();
        (t7f, (t7f * l.f15a2),)
    } else {
        let t80: f64 = (-50.0);
        let (t89, t8a,) = {
            if ((!(l.f15a1 > 50.0)) && (l.f15a1 < t80)) {
                let t81: f64 = (-50.0);let t82: f64 = (t81).exp();
                (t82, 0.0,)
            } else {
                let (t87, t88,) = {
                    if (l.f15a1 > 50.0) {
                        let t83: f64 = (50.0_f64).exp();let t84: f64 = (l.f15a1 - 50.0);let t85: f64 = (1.0 + t84);let t86: f64 = (t83 * t85);
                        (t86, (t83 * l.f15a2),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t87, t88,)
            }
        };
        (t89, t8a,)
    }
};
            (l.f15de, l.f15df, ) = (t8b, t8c, );
        }
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) {let t8d: f64 = (-l.f15e4);let t8e: f64 = (t8d - l.f15e3);let t8f: f64 = (l.f15d3 * t8e);let t90: f64 = (t8f + l.f15a1);(l.f1575, l.f1576, l.f1577, l.f1578, ) = (t90, l.f15a2, (l.f15d3 * (-l.f15e5)), (l.f15d3 * (-l.f15e6)), );let t91: f64 = (-l.f15d3);let t92: f64 = (t91 * l.f15e3);let t93: f64 = (t92 + l.f15a1);(l.f157b, l.f157c, ) = (t93, l.f15a2, );}
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) {
            let t94: f64 = (-50.0);
            let (ta5, ta6, ta7, ta8,) = {
    if ((!(l.f1575 > 50.0)) && (!(l.f1575 < t94))) {
        let t95: f64 = (l.f1575).exp();
        (t95, (t95 * l.f1576), (t95 * l.f1577), (t95 * l.f1578),)
    } else {
        let t96: f64 = (-50.0);
        let (ta1, ta2, ta3, ta4,) = {
            if ((!(l.f1575 > 50.0)) && (l.f1575 < t96)) {
                let t97: f64 = (-50.0);let t98: f64 = (t97).exp();
                (t98, 0.0, 0.0, 0.0,)
            } else {
                let (t9d, t9e, t9f, ta0,) = {
                    if (l.f1575 > 50.0) {
                        let t99: f64 = (50.0_f64).exp();let t9a: f64 = (l.f1575 - 50.0);let t9b: f64 = (1.0 + t9a);let t9c: f64 = (t99 * t9b);
                        (t9c, (t99 * l.f1576), (t99 * l.f1577), (t99 * l.f1578),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t9d, t9e, t9f, ta0,)
            }
        };
        (ta1, ta2, ta3, ta4,)
    }
};
            (l.f156d, l.f156e, l.f156f, l.f1570, ) = (ta5, ta6, ta7, ta8, );
        }
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) {
            let ta9: f64 = (-50.0);
            let (tb6, tb7,) = {
    if ((!(l.f157b > 50.0)) && (!(l.f157b < ta9))) {
        let taa: f64 = (l.f157b).exp();
        (taa, (taa * l.f157c),)
    } else {
        let tab: f64 = (-50.0);
        let (tb4, tb5,) = {
            if ((!(l.f157b > 50.0)) && (l.f157b < tab)) {
                let tac: f64 = (-50.0);let tad: f64 = (tac).exp();
                (tad, 0.0,)
            } else {
                let (tb2, tb3,) = {
                    if (l.f157b > 50.0) {
                        let tae: f64 = (50.0_f64).exp();let taf: f64 = (l.f157b - 50.0);let tb0: f64 = (1.0 + taf);let tb1: f64 = (tae * tb0);
                        (tb1, (tae * l.f157c),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (tb2, tb3,)
            }
        };
        (tb4, tb5,)
    }
};
            (l.f1573, l.f1574, ) = (tb6, tb7, );
        }
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) {let tb8: f64 = (l.f156d - l.f1573);(l.f15ab, l.f15ac, l.f15ad, l.f15ae, ) = (tb8, (l.f156e - l.f1574), l.f156f, l.f1570, );let tb9: f64 = (l.f15e2 * l.f15ea);let tba: f64 = (tb9 * l.f15d2);let tbb: f64 = (tba * l.f15cb);let tbc: f64 = (tbb * l.f15e0);(l.f15cd, l.f15ce, ) = (tbc, (tbb * l.f15e1), );let tbd: f64 = (l.f15d5 / l.f15d8);let tbe: f64 = (tbd * l.f15e4);let tbf: f64 = (tbe + l.f15a1);(l.f158d, l.f158e, l.f158f, l.f1590, ) = (tbf, (((-((l.f15d5 * l.f15d9) / (l.f15d8 * l.f15d8))) * l.f15e4) + l.f15a2), (tbd * l.f15e5), (tbd * l.f15e6), );}
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) {
            let tc0: f64 = (-50.0);
            let (td1, td2, td3, td4,) = {
    if ((!(l.f158d > 50.0)) && (!(l.f158d < tc0))) {
        let tc1: f64 = (l.f158d).exp();
        (tc1, (tc1 * l.f158e), (tc1 * l.f158f), (tc1 * l.f1590),)
    } else {
        let tc2: f64 = (-50.0);
        let (tcd, tce, tcf, td0,) = {
            if ((!(l.f158d > 50.0)) && (l.f158d < tc2)) {
                let tc3: f64 = (-50.0);let tc4: f64 = (tc3).exp();
                (tc4, 0.0, 0.0, 0.0,)
            } else {
                let (tc9, tca, tcb, tcc,) = {
                    if (l.f158d > 50.0) {
                        let tc5: f64 = (50.0_f64).exp();let tc6: f64 = (l.f158d - 50.0);let tc7: f64 = (1.0 + tc6);let tc8: f64 = (tc5 * tc7);
                        (tc8, (tc5 * l.f158e), (tc5 * l.f158f), (tc5 * l.f1590),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (tc9, tca, tcb, tcc,)
            }
        };
        (tcd, tce, tcf, td0,)
    }
};
            (l.f1581, l.f1582, l.f1583, l.f1584, ) = (td1, td2, td3, td4, );
        }
        let td5: f64 = if l.f15a7 == 1.0 { 1.0 } else { 0.0 };l.f1ff8 = td5;
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ff8 != 0.0)) {let td6: f64 = (l.f15d1 * l.f15ab);let td7: f64 = (l.f1581 - td6);let td8: f64 = (td7 - l.f15de);let td9: f64 = (l.f15cd * td8);(l.f15b1, l.f15b2, l.f15b3, l.f15b4, ) = (td9, ((l.f15ce * td8) + (l.f15cd * ((l.f1582 - (l.f15d1 * l.f15ac)) - l.f15df))), (l.f15cd * (l.f1583 - (l.f15d1 * l.f15ad))), (l.f15cd * (l.f1584 - (l.f15d1 * l.f15ae))), );}
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ff8 == 0.0)) {let tda: f64 = (-l.f15e7);let tdb: f64 = (tda - l.f15e3);let tdc: f64 = (l.f15d3 * tdb);let tdd: f64 = (tdc + l.f15a1);(l.f1579, l.f157a, ) = (tdd, l.f15a2, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_131(
        l: &mut StampLocals,
    ) {
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ff8 == 0.0)) {
            let tde: f64 = (-50.0);
            let (teb, tec,) = {
    if ((!(l.f1579 > 50.0)) && (!(l.f1579 < tde))) {
        let tdf: f64 = (l.f1579).exp();
        (tdf, (tdf * l.f157a),)
    } else {
        let te0: f64 = (-50.0);
        let (te9, tea,) = {
            if ((!(l.f1579 > 50.0)) && (l.f1579 < te0)) {
                let te1: f64 = (-50.0);let te2: f64 = (te1).exp();
                (te2, 0.0,)
            } else {
                let (te7, te8,) = {
                    if (l.f1579 > 50.0) {
                        let te3: f64 = (50.0_f64).exp();let te4: f64 = (l.f1579 - 50.0);let te5: f64 = (1.0 + te4);let te6: f64 = (te3 * te5);
                        (te6, (te3 * l.f157a),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (te7, te8,)
            }
        };
        (te9, tea,)
    }
};
            (l.f1571, l.f1572, ) = (teb, tec, );
        }
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ff8 == 0.0)) {let ted: f64 = (l.f1571 - l.f1573);(l.f15af, l.f15b0, ) = (ted, (l.f1572 - l.f1574), );let tee: f64 = (l.f15d5 / l.f15d8);let tef: f64 = (tee * l.f15e7);let tf0: f64 = (tef + l.f15a1);(l.f1597, l.f1598, ) = (tf0, (((-((l.f15d5 * l.f15d9) / (l.f15d8 * l.f15d8))) * l.f15e7) + l.f15a2), );}
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ff8 == 0.0)) {
            let tf1: f64 = (-50.0);
            let (tfe, tff,) = {
    if ((!(l.f1597 > 50.0)) && (!(l.f1597 < tf1))) {
        let tf2: f64 = (l.f1597).exp();
        (tf2, (tf2 * l.f1598),)
    } else {
        let tf3: f64 = (-50.0);
        let (tfc, tfd,) = {
            if ((!(l.f1597 > 50.0)) && (l.f1597 < tf3)) {
                let tf4: f64 = (-50.0);let tf5: f64 = (tf4).exp();
                (tf5, 0.0,)
            } else {
                let (tfa, tfb,) = {
                    if (l.f1597 > 50.0) {
                        let tf6: f64 = (50.0_f64).exp();let tf7: f64 = (l.f1597 - 50.0);let tf8: f64 = (1.0 + tf7);let tf9: f64 = (tf6 * tf8);
                        (tf9, (tf6 * l.f1598),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (tfa, tfb,)
            }
        };
        (tfc, tfd,)
    }
};
            (l.f158b, l.f158c, ) = (tfe, tff, );
        }
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ff8 == 0.0)) {let t100: f64 = (l.f15d1 * l.f15af);let t101: f64 = (l.f158b - t100);let t102: f64 = (t101 - l.f15de);(l.f15c1, l.f15c2, ) = (t102, ((l.f158c - (l.f15d1 * l.f15b0)) - l.f15df), );let t103: f64 = (l.f15d1 * l.f15ab);let t104: f64 = (l.f1581 - t103);let t105: f64 = (t104 - l.f15de);let t106: f64 = (l.f15cd * t105);(l.f15bd, l.f15be, l.f15bf, l.f15c0, ) = (t106, ((l.f15ce * t105) + (l.f15cd * ((l.f1582 - (l.f15d1 * l.f15ac)) - l.f15df))), (l.f15cd * (l.f1583 - (l.f15d1 * l.f15ad))), (l.f15cd * (l.f1584 - (l.f15d1 * l.f15ae))), );}
        let t107: f64 = if l.f15a7 > 0.0 { 1.0 } else { 0.0 };l.f1ff9 = t107;
        if (((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ff8 == 0.0)) && (l.f1ff9 != 0.0)) {let t108: f64 = (l.f15a7 * l.f15d5);l.f15d6 = t108;let t109: f64 = (l.f15d6 / l.f15d8);let t10a: f64 = (t109 * l.f15e7);let t10b: f64 = (t10a + l.f15a1);(l.f1595, l.f1596, ) = (t10b, (((-((l.f15d6 * l.f15d9) / (l.f15d8 * l.f15d8))) * l.f15e7) + l.f15a2), );}
        if (((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ff8 == 0.0)) && (l.f1ff9 != 0.0)) {
            let t10c: f64 = (-50.0);
            let (t119, t11a,) = {
    if ((!(l.f1595 > 50.0)) && (!(l.f1595 < t10c))) {
        let t10d: f64 = (l.f1595).exp();
        (t10d, (t10d * l.f1596),)
    } else {
        let t10e: f64 = (-50.0);
        let (t117, t118,) = {
            if ((!(l.f1595 > 50.0)) && (l.f1595 < t10e)) {
                let t10f: f64 = (-50.0);let t110: f64 = (t10f).exp();
                (t110, 0.0,)
            } else {
                let (t115, t116,) = {
                    if (l.f1595 > 50.0) {
                        let t111: f64 = (50.0_f64).exp();let t112: f64 = (l.f1595 - 50.0);let t113: f64 = (1.0 + t112);let t114: f64 = (t111 * t113);
                        (t114, (t111 * l.f1596),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t115, t116,)
            }
        };
        (t117, t118,)
    }
};
            (l.f1589, l.f158a, ) = (t119, t11a, );
        }
        if (((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ff8 == 0.0)) && (l.f1ff9 != 0.0)) {let t11b: f64 = (l.f15d1 * l.f15af);let t11c: f64 = (l.f1589 - t11b);let t11d: f64 = (t11c - l.f15de);(l.f15bb, l.f15bc, ) = (t11d, ((l.f158a - (l.f15d1 * l.f15b0)) - l.f15df), );let t11e: f64 = (l.f15d6 / l.f15d8);let t11f: f64 = (t11e * l.f15e4);let t120: f64 = (t11f + l.f15a1);(l.f1591, l.f1592, l.f1593, l.f1594, ) = (t120, (((-((l.f15d6 * l.f15d9) / (l.f15d8 * l.f15d8))) * l.f15e4) + l.f15a2), (t11e * l.f15e5), (t11e * l.f15e6), );}
        if (((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ff8 == 0.0)) && (l.f1ff9 != 0.0)) {
            let t121: f64 = (-50.0);
            let (t132, t133, t134, t135,) = {
    if ((!(l.f1591 > 50.0)) && (!(l.f1591 < t121))) {
        let t122: f64 = (l.f1591).exp();
        (t122, (t122 * l.f1592), (t122 * l.f1593), (t122 * l.f1594),)
    } else {
        let t123: f64 = (-50.0);
        let (t12e, t12f, t130, t131,) = {
            if ((!(l.f1591 > 50.0)) && (l.f1591 < t123)) {
                let t124: f64 = (-50.0);let t125: f64 = (t124).exp();
                (t125, 0.0, 0.0, 0.0,)
            } else {
                let (t12a, t12b, t12c, t12d,) = {
                    if (l.f1591 > 50.0) {
                        let t126: f64 = (50.0_f64).exp();let t127: f64 = (l.f1591 - 50.0);let t128: f64 = (1.0 + t127);let t129: f64 = (t126 * t128);
                        (t129, (t126 * l.f1592), (t126 * l.f1593), (t126 * l.f1594),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t12a, t12b, t12c, t12d,)
            }
        };
        (t12e, t12f, t130, t131,)
    }
};
            (l.f1585, l.f1586, l.f1587, l.f1588, ) = (t132, t133, t134, t135, );
        }
        if (((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ff8 == 0.0)) && (l.f1ff9 != 0.0)) {let t136: f64 = (l.f15cd * l.f15c1);let t137: f64 = (t136 / l.f15bb);(l.f15b9, l.f15ba, ) = (t137, (((((l.f15ce * l.f15c1) + (l.f15cd * l.f15c2)) * l.f15bb) - (t136 * l.f15bc)) / (l.f15bb * l.f15bb)), );let t138: f64 = (l.f15d1 * l.f15ab);let t139: f64 = (l.f1585 - t138);let t13a: f64 = (t139 - l.f15de);let t13b: f64 = (l.f15b9 * t13a);(l.f15b5, l.f15b6, l.f15b7, l.f15b8, ) = (t13b, ((l.f15ba * t13a) + (l.f15b9 * ((l.f1586 - (l.f15d1 * l.f15ac)) - l.f15df))), (l.f15b9 * (l.f1587 - (l.f15d1 * l.f15ad))), (l.f15b9 * (l.f1588 - (l.f15d1 * l.f15ae))), );}
        if (((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ff8 == 0.0)) && (l.f1ff9 == 0.0)) {let t13c: f64 = (l.f15cd * l.f15c1);(l.f15b5, l.f15b6, l.f15b7, l.f15b8, ) = (t13c, ((l.f15ce * l.f15c1) + (l.f15cd * l.f15c2)), 0.0, 0.0, );}
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ff8 == 0.0)) {let t13d: f64 = (l.f156b * l.f156b);let t13e: f64 = (t13d * l.f15d8);(l.f1569, l.f156a, ) = (t13e, (t13d * l.f15d9), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_132(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);let nv8 = ctx.node_voltage(nodes[8]);
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ff8 == 0.0)) {let t13f: f64 = (l.f1569 / 2.0);let t140: f64 = (l.f15e7 - t13f);let t141: f64 = (l.f15e4 - t140);let t142: f64 = (t141 / l.f1569);(l.f157d, l.f157e, l.f157f, l.f1580, ) = (t142, ((((-(-(l.f156a / 2.0))) * l.f1569) - (t141 * l.f156a)) / (l.f1569 * l.f1569)), (l.f15e5 / l.f1569), (l.f15e6 / l.f1569), );}
        let t143: f64 = if l.f157d > 50.0 { 1.0 } else { 0.0 };l.f1ffa = t143;
        if (((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ff8 == 0.0)) && (l.f1ffa != 0.0)) {(l.f15a3, l.f15a4, l.f15a5, l.f15a6, ) = (0.0, 0.0, 0.0, 0.0, );}
        let t144: f64 = (-50.0);let t145: f64 = if l.f157d < t144 { 1.0 } else { 0.0 };l.f1ffb = t145;
        if ((((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ff8 == 0.0)) && (l.f1ffa == 0.0)) && (l.f1ffb != 0.0)) {(l.f15a3, l.f15a4, l.f15a5, l.f15a6, ) = (1.0, 0.0, 0.0, 0.0, );}
        if ((((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ff8 == 0.0)) && (l.f1ffa == 0.0)) && (l.f1ffb == 0.0)) {let t146: f64 = (l.f157d).exp();let t147: f64 = (1.0 + t146);let t148: f64 = (1.0 / t147);(l.f15a3, l.f15a4, l.f15a5, l.f15a6, ) = (t148, (-((t146 * l.f157e) / (t147 * t147))), (-((t146 * l.f157f) / (t147 * t147))), (-((t146 * l.f1580) / (t147 * t147))), );}
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ff8 == 0.0)) {let t149: f64 = (l.f15a3 * l.f15bd);let t14a: f64 = (1.0 - l.f15a3);let t14b: f64 = (t14a * l.f15b5);let t14c: f64 = (t149 + t14b);(l.f15b1, l.f15b2, l.f15b3, l.f15b4, ) = (t14c, (((l.f15a4 * l.f15bd) + (l.f15a3 * l.f15be)) + (((-l.f15a4) * l.f15b5) + (t14a * l.f15b6))), (((l.f15a5 * l.f15bd) + (l.f15a3 * l.f15bf)) + (((-l.f15a5) * l.f15b5) + (t14a * l.f15b7))), (((l.f15a6 * l.f15bd) + (l.f15a3 * l.f15c0)) + (((-l.f15a6) * l.f15b5) + (t14a * l.f15b8))), );}
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) {
            let t14d: f64 = (-l.f15e4);
            let (t15c, t15d, t15e,) = {
    if (p.p52 != 0.0) {
        let t14e: f64 = (l.f15e4 / l.f15e8);let t14f: f64 = (0.001 / p.p53);let t150: f64 = (l.f15e4 / l.f15e8);let t151: f64 = (t14f * t150);let t152: f64 = (t151).tanh();let t153: f64 = (t14e * t152);
        (t153, (((l.f15e5 / l.f15e8) * t152) + (t14e * ((t14f * (l.f15e5 / l.f15e8)) / ((t151).cosh() * (t151).cosh())))), (((l.f15e6 / l.f15e8) * t152) + (t14e * ((t14f * (l.f15e6 / l.f15e8)) / ((t151).cosh() * (t151).cosh())))),)
    } else {
        let (t159, t15a, t15b,) = {
            if (p.p52 == 0.0) {
                let __rspice_inv_cse_0: f64 = 1.0 / l.f15e8;let t154: f64 = (l.f15e4 * __rspice_inv_cse_0);let t155: f64 = (l.f15e4 * __rspice_inv_cse_0);let t156: f64 = (t154 * t155);let t157: f64 = (t156 + p.p53);let t158: f64 = (t157).sqrt();
                (t158, ((((l.f15e5 / l.f15e8) * t155) + (t154 * (l.f15e5 / l.f15e8))) / (2.0 * t158)), ((((l.f15e6 / l.f15e8) * t155) + (t154 * (l.f15e6 / l.f15e8))) / (2.0 * t158)),)
            } else {
                (0.0, 0.0, 0.0,)
            }
        };
        (t159, t15a, t15b,)
    }
};
            let t15f: f64 = (t15c).powf(l.f156c);let t160: f64 = (1.0 + t15f);let t161: f64 = (1.0 / l.f156c);let t162: f64 = (t160).powf(t161);let t163: f64 = (t14d / t162);(l.f15a8, l.f15a9, l.f15aa, ) = (t163, ((((-l.f15e5) * t162) - (t14d * if 0.0 == 0.0 && ((t161) as f64).is_finite() && ((t161) as f64).fract() == 0.0 { if t161 == 0.0 { 0.0 } else { (t161 * ((t160).powf(t161 - 1.0) * if 0.0 == 0.0 && ((l.f156c) as f64).is_finite() && ((l.f156c) as f64).fract() == 0.0 { if l.f156c == 0.0 { 0.0 } else { (l.f156c * ((t15c).powf(l.f156c - 1.0) * t15d)) } } else { (t15f * (l.f156c * (t15d / t15c))) })) } } else { (t162 * (t161 * (if 0.0 == 0.0 && ((l.f156c) as f64).is_finite() && ((l.f156c) as f64).fract() == 0.0 { if l.f156c == 0.0 { 0.0 } else { (l.f156c * ((t15c).powf(l.f156c - 1.0) * t15d)) } } else { (t15f * (l.f156c * (t15d / t15c))) } / t160))) })) / (t162 * t162)), ((((-l.f15e6) * t162) - (t14d * if 0.0 == 0.0 && ((t161) as f64).is_finite() && ((t161) as f64).fract() == 0.0 { if t161 == 0.0 { 0.0 } else { (t161 * ((t160).powf(t161 - 1.0) * if 0.0 == 0.0 && ((l.f156c) as f64).is_finite() && ((l.f156c) as f64).fract() == 0.0 { if l.f156c == 0.0 { 0.0 } else { (l.f156c * ((t15c).powf(l.f156c - 1.0) * t15e)) } } else { (t15f * (l.f156c * (t15e / t15c))) })) } } else { (t162 * (t161 * (if 0.0 == 0.0 && ((l.f156c) as f64).is_finite() && ((l.f156c) as f64).fract() == 0.0 { if l.f156c == 0.0 { 0.0 } else { (l.f156c * ((t15c).powf(l.f156c - 1.0) * t15e)) } } else { (t15f * (l.f156c * (t15e / t15c))) } / t160))) })) / (t162 * t162)), );
        }
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) {let t164: f64 = (-l.f15e2);let t165: f64 = (t164 * l.f15ea);let t166: f64 = (t165 * l.f15d2);let t167: f64 = (t166 * l.f15cc);let t168: f64 = (t167 * l.f15e0);let t169: f64 = t168;(l.f15cf, l.f15d0, ) = (t169, (t167 * l.f15e1), );let t16a: f64 = (l.f15d7 / l.f15d8);let t16b: f64 = (t16a * l.f15a8);(l.f159d, l.f159e, l.f159f, l.f15a0, ) = (t16b, ((-((l.f15d7 * l.f15d9) / (l.f15d8 * l.f15d8))) * l.f15a8), (t16a * l.f15a9), (t16a * l.f15aa), );}
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) {
            let t16c: f64 = (-50.0);
            let (t17d, t17e, t17f, t180,) = {
    if ((!(l.f159d > 50.0)) && (!(l.f159d < t16c))) {
        let t16d: f64 = (l.f159d).exp();
        (t16d, (t16d * l.f159e), (t16d * l.f159f), (t16d * l.f15a0),)
    } else {
        let t16e: f64 = (-50.0);
        let (t179, t17a, t17b, t17c,) = {
            if ((!(l.f159d > 50.0)) && (l.f159d < t16e)) {
                let t16f: f64 = (-50.0);let t170: f64 = (t16f).exp();
                (t170, 0.0, 0.0, 0.0,)
            } else {
                let (t175, t176, t177, t178,) = {
                    if (l.f159d > 50.0) {
                        let t171: f64 = (50.0_f64).exp();let t172: f64 = (l.f159d - 50.0);let t173: f64 = (1.0 + t172);let t174: f64 = (t171 * t173);
                        (t174, (t171 * l.f159e), (t171 * l.f159f), (t171 * l.f15a0),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t175, t176, t177, t178,)
            }
        };
        (t179, t17a, t17b, t17c,)
    }
};
            (l.f1599, l.f159a, l.f159b, l.f159c, ) = (t17d, t17e, t17f, t180, );
        }
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) {let t181: f64 = (l.f1599 - 1.0);let t182: f64 = (l.f15cf * t181);(l.f15c3, l.f15c4, l.f15c5, l.f15c6, ) = (t182, ((l.f15d0 * t181) + (l.f15cf * l.f159a)), (l.f15cf * l.f159b), (l.f15cf * l.f159c), );let t183: f64 = (l.f15b1 + l.f15c3);(l.f15c7, l.f15c8, l.f15c9, l.f15ca, ) = (t183, (l.f15b2 + l.f15c4), (l.f15b3 + l.f15c5), (l.f15b4 + l.f15c6), );(l.f15da, l.f15db, l.f15dc, l.f15dd, ) = (l.f15c7, l.f15c8, l.f15c9, l.f15ca, );(l.f2138, l.f2139, l.f213a, l.f213b, ) = (l.f15da, l.f15db, l.f15dc, l.f15dd, );(l.f165c, l.f165d, l.f165e, l.f165f, ) = (0.0, 0.0, 0.0, 0.0, );(l.f164f, l.f1650, ) = (0.0, 0.0, );(l.f1651, l.f1652, ) = (0.0, 0.0, );let t184: f64 = (p.p6 * (nv8 - nv5));(l.f1666, l.f1667, l.f1668, ) = (t184, (-p.p6), p.p6, );(l.f165a, l.f165b, ) = (l.f215b, l.f215c, );l.f1669 = p.p265;l.f15ed = p.p267;l.f1629 = 1.0;l.f1657 = p.p263;l.f1655 = p.p281;l.f1665 = p.p280;(l.f1662, l.f1663, ) = (l.f22f2, l.f22f3, );l.f166c = p.p0;l.f1654 = p.p2;l.f164d = 0.0;l.f1653 = 0.0;l.f166a = p.p289;l.f15ee = p.p290;let t185: f64 = (p.p255 * p.p288);l.f164e = t185;l.f1659 = p.p287;l.f1656 = p.p257;l.f166b = p.p256;l.f1664 = p.p6;}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_133(
        l: &mut StampLocals,
    ) {
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) {(l.f1649, l.f164a, l.f164b, l.f164c, ) = (0.0, 0.0, 0.0, 0.0, );(l.f15eb, l.f15ec, ) = (0.0, 0.0, );(l.f1660, l.f1661, ) = (0.0, 0.0, );(l.f1625, l.f1626, l.f1627, l.f1628, ) = (0.0, 0.0, 0.0, 0.0, );(l.f162d, l.f162e, l.f162f, l.f1630, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1633, l.f1634, l.f1635, l.f1636, ) = (0.0, 0.0, 0.0, 0.0, );(l.f162a, l.f162b, l.f162c, ) = (0.0, 0.0, 0.0, );(l.f1645, l.f1646, l.f1647, l.f1648, ) = (0.0, 0.0, 0.0, 0.0, );(l.f15f7, l.f15f8, l.f15f9, l.f15fa, ) = (0.0, 0.0, 0.0, 0.0, );(l.f15fd, l.f15fe, ) = (0.0, 0.0, );(l.f15ef, l.f15f0, l.f15f1, l.f15f2, ) = (0.0, 0.0, 0.0, 0.0, );(l.f15f5, l.f15f6, ) = (0.0, 0.0, );(l.f1623, l.f1624, ) = (0.0, 0.0, );(l.f15ff, l.f1600, l.f1601, l.f1602, ) = (0.0, 0.0, 0.0, 0.0, );(l.f160f, l.f1610, l.f1611, l.f1612, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1603, l.f1604, l.f1605, l.f1606, ) = (0.0, 0.0, 0.0, 0.0, );(l.f161f, l.f1620, l.f1621, l.f1622, ) = (0.0, 0.0, 0.0, 0.0, );(l.f161b, l.f161c, l.f161d, l.f161e, ) = (0.0, 0.0, 0.0, 0.0, );l.f1658 = 0.0;(l.f15fb, l.f15fc, ) = (0.0, 0.0, );(l.f15f3, l.f15f4, ) = (0.0, 0.0, );(l.f1631, l.f1632, ) = (0.0, 0.0, );(l.f1619, l.f161a, ) = (0.0, 0.0, );(l.f160d, l.f160e, ) = (0.0, 0.0, );(l.f1643, l.f1644, ) = (0.0, 0.0, );(l.f163f, l.f1640, l.f1641, l.f1642, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1617, l.f1618, ) = (0.0, 0.0, );(l.f160b, l.f160c, ) = (0.0, 0.0, );(l.f163d, l.f163e, ) = (0.0, 0.0, );(l.f1613, l.f1614, l.f1615, l.f1616, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1607, l.f1608, l.f1609, l.f160a, ) = (0.0, 0.0, 0.0, 0.0, );(l.f163b, l.f163c, ) = (0.0, 0.0, );(l.f1637, l.f1638, l.f1639, l.f163a, ) = (0.0, 0.0, 0.0, 0.0, );let t186: f64 = (l.f1656 / l.f165a);let t187: f64 = (-l.f166b);let t188: f64 = (t186 * t187);(l.f1623, l.f1624, ) = (t188, ((-((l.f1656 * l.f165b) / (l.f165a * l.f165a))) * t187), );}
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) {
            let t189: f64 = (-50.0);
            let (t196, t197,) = {
    if ((!(l.f1623 > 50.0)) && (!(l.f1623 < t189))) {
        let t18a: f64 = (l.f1623).exp();
        (t18a, (t18a * l.f1624),)
    } else {
        let t18b: f64 = (-50.0);
        let (t194, t195,) = {
            if ((!(l.f1623 > 50.0)) && (l.f1623 < t18b)) {
                let t18c: f64 = (-50.0);let t18d: f64 = (t18c).exp();
                (t18d, 0.0,)
            } else {
                let (t192, t193,) = {
                    if (l.f1623 > 50.0) {
                        let t18e: f64 = (50.0_f64).exp();let t18f: f64 = (l.f1623 - 50.0);let t190: f64 = (1.0 + t18f);let t191: f64 = (t18e * t190);
                        (t191, (t18e * l.f1624),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t192, t193,)
            }
        };
        (t194, t195,)
    }
};
            (l.f1660, l.f1661, ) = (t196, t197, );
        }
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) {let t198: f64 = (-l.f1666);let t199: f64 = (t198 - l.f1665);let t19a: f64 = (l.f1655 * t199);let t19b: f64 = (t19a + l.f1623);(l.f15f7, l.f15f8, l.f15f9, l.f15fa, ) = (t19b, l.f1624, (l.f1655 * (-l.f1667)), (l.f1655 * (-l.f1668)), );let t19c: f64 = (-l.f1655);let t19d: f64 = (t19c * l.f1665);let t19e: f64 = (t19d + l.f1623);(l.f15fd, l.f15fe, ) = (t19e, l.f1624, );}
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) {
            let t19f: f64 = (-50.0);
            let (t1b0, t1b1, t1b2, t1b3,) = {
    if ((!(l.f15f7 > 50.0)) && (!(l.f15f7 < t19f))) {
        let t1a0: f64 = (l.f15f7).exp();
        (t1a0, (t1a0 * l.f15f8), (t1a0 * l.f15f9), (t1a0 * l.f15fa),)
    } else {
        let t1a1: f64 = (-50.0);
        let (t1ac, t1ad, t1ae, t1af,) = {
            if ((!(l.f15f7 > 50.0)) && (l.f15f7 < t1a1)) {
                let t1a2: f64 = (-50.0);let t1a3: f64 = (t1a2).exp();
                (t1a3, 0.0, 0.0, 0.0,)
            } else {
                let (t1a8, t1a9, t1aa, t1ab,) = {
                    if (l.f15f7 > 50.0) {
                        let t1a4: f64 = (50.0_f64).exp();let t1a5: f64 = (l.f15f7 - 50.0);let t1a6: f64 = (1.0 + t1a5);let t1a7: f64 = (t1a4 * t1a6);
                        (t1a7, (t1a4 * l.f15f8), (t1a4 * l.f15f9), (t1a4 * l.f15fa),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t1a8, t1a9, t1aa, t1ab,)
            }
        };
        (t1ac, t1ad, t1ae, t1af,)
    }
};
            (l.f15ef, l.f15f0, l.f15f1, l.f15f2, ) = (t1b0, t1b1, t1b2, t1b3, );
        }
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) {
            let t1b4: f64 = (-50.0);
            let (t1c1, t1c2,) = {
    if ((!(l.f15fd > 50.0)) && (!(l.f15fd < t1b4))) {
        let t1b5: f64 = (l.f15fd).exp();
        (t1b5, (t1b5 * l.f15fe),)
    } else {
        let t1b6: f64 = (-50.0);
        let (t1bf, t1c0,) = {
            if ((!(l.f15fd > 50.0)) && (l.f15fd < t1b6)) {
                let t1b7: f64 = (-50.0);let t1b8: f64 = (t1b7).exp();
                (t1b8, 0.0,)
            } else {
                let (t1bd, t1be,) = {
                    if (l.f15fd > 50.0) {
                        let t1b9: f64 = (50.0_f64).exp();let t1ba: f64 = (l.f15fd - 50.0);let t1bb: f64 = (1.0 + t1ba);let t1bc: f64 = (t1b9 * t1bb);
                        (t1bc, (t1b9 * l.f15fe),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t1bd, t1be,)
            }
        };
        (t1bf, t1c0,)
    }
};
            (l.f15f5, l.f15f6, ) = (t1c1, t1c2, );
        }
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) {let t1c3: f64 = (l.f15ef - l.f15f5);(l.f162d, l.f162e, l.f162f, l.f1630, ) = (t1c3, (l.f15f0 - l.f15f6), l.f15f1, l.f15f2, );let t1c4: f64 = (l.f1664 * l.f166c);let t1c5: f64 = (t1c4 * l.f1654);let t1c6: f64 = (t1c5 * l.f164d);let t1c7: f64 = (t1c6 * l.f1662);(l.f164f, l.f1650, ) = (t1c7, (t1c6 * l.f1663), );let t1c8: f64 = (l.f1657 / l.f165a);let t1c9: f64 = (t1c8 * l.f1666);let t1ca: f64 = (t1c9 + l.f1623);(l.f160f, l.f1610, l.f1611, l.f1612, ) = (t1ca, (((-((l.f1657 * l.f165b) / (l.f165a * l.f165a))) * l.f1666) + l.f1624), (t1c8 * l.f1667), (t1c8 * l.f1668), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_134(
        l: &mut StampLocals,
    ) {
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) {
            let t1cb: f64 = (-50.0);
            let (t1dc, t1dd, t1de, t1df,) = {
    if ((!(l.f160f > 50.0)) && (!(l.f160f < t1cb))) {
        let t1cc: f64 = (l.f160f).exp();
        (t1cc, (t1cc * l.f1610), (t1cc * l.f1611), (t1cc * l.f1612),)
    } else {
        let t1cd: f64 = (-50.0);
        let (t1d8, t1d9, t1da, t1db,) = {
            if ((!(l.f160f > 50.0)) && (l.f160f < t1cd)) {
                let t1ce: f64 = (-50.0);let t1cf: f64 = (t1ce).exp();
                (t1cf, 0.0, 0.0, 0.0,)
            } else {
                let (t1d4, t1d5, t1d6, t1d7,) = {
                    if (l.f160f > 50.0) {
                        let t1d0: f64 = (50.0_f64).exp();let t1d1: f64 = (l.f160f - 50.0);let t1d2: f64 = (1.0 + t1d1);let t1d3: f64 = (t1d0 * t1d2);
                        (t1d3, (t1d0 * l.f1610), (t1d0 * l.f1611), (t1d0 * l.f1612),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t1d4, t1d5, t1d6, t1d7,)
            }
        };
        (t1d8, t1d9, t1da, t1db,)
    }
};
            (l.f1603, l.f1604, l.f1605, l.f1606, ) = (t1dc, t1dd, t1de, t1df, );
        }
        let t1e0: f64 = if l.f1629 == 1.0 { 1.0 } else { 0.0 };l.f1ffc = t1e0;
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ffc != 0.0)) {let t1e1: f64 = (l.f1653 * l.f162d);let t1e2: f64 = (l.f1603 - t1e1);let t1e3: f64 = (t1e2 - l.f1660);let t1e4: f64 = (l.f164f * t1e3);(l.f1633, l.f1634, l.f1635, l.f1636, ) = (t1e4, ((l.f1650 * t1e3) + (l.f164f * ((l.f1604 - (l.f1653 * l.f162e)) - l.f1661))), (l.f164f * (l.f1605 - (l.f1653 * l.f162f))), (l.f164f * (l.f1606 - (l.f1653 * l.f1630))), );}
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ffc == 0.0)) {let t1e5: f64 = (-l.f1669);let t1e6: f64 = (t1e5 - l.f1665);let t1e7: f64 = (l.f1655 * t1e6);let t1e8: f64 = (t1e7 + l.f1623);(l.f15fb, l.f15fc, ) = (t1e8, l.f1624, );}
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ffc == 0.0)) {
            let t1e9: f64 = (-50.0);
            let (t1f6, t1f7,) = {
    if ((!(l.f15fb > 50.0)) && (!(l.f15fb < t1e9))) {
        let t1ea: f64 = (l.f15fb).exp();
        (t1ea, (t1ea * l.f15fc),)
    } else {
        let t1eb: f64 = (-50.0);
        let (t1f4, t1f5,) = {
            if ((!(l.f15fb > 50.0)) && (l.f15fb < t1eb)) {
                let t1ec: f64 = (-50.0);let t1ed: f64 = (t1ec).exp();
                (t1ed, 0.0,)
            } else {
                let (t1f2, t1f3,) = {
                    if (l.f15fb > 50.0) {
                        let t1ee: f64 = (50.0_f64).exp();let t1ef: f64 = (l.f15fb - 50.0);let t1f0: f64 = (1.0 + t1ef);let t1f1: f64 = (t1ee * t1f0);
                        (t1f1, (t1ee * l.f15fc),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t1f2, t1f3,)
            }
        };
        (t1f4, t1f5,)
    }
};
            (l.f15f3, l.f15f4, ) = (t1f6, t1f7, );
        }
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ffc == 0.0)) {let t1f8: f64 = (l.f15f3 - l.f15f5);(l.f1631, l.f1632, ) = (t1f8, (l.f15f4 - l.f15f6), );let t1f9: f64 = (l.f1657 / l.f165a);let t1fa: f64 = (t1f9 * l.f1669);let t1fb: f64 = (t1fa + l.f1623);(l.f1619, l.f161a, ) = (t1fb, (((-((l.f1657 * l.f165b) / (l.f165a * l.f165a))) * l.f1669) + l.f1624), );}
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ffc == 0.0)) {
            let t1fc: f64 = (-50.0);
            let (t209, t20a,) = {
    if ((!(l.f1619 > 50.0)) && (!(l.f1619 < t1fc))) {
        let t1fd: f64 = (l.f1619).exp();
        (t1fd, (t1fd * l.f161a),)
    } else {
        let t1fe: f64 = (-50.0);
        let (t207, t208,) = {
            if ((!(l.f1619 > 50.0)) && (l.f1619 < t1fe)) {
                let t1ff: f64 = (-50.0);let t200: f64 = (t1ff).exp();
                (t200, 0.0,)
            } else {
                let (t205, t206,) = {
                    if (l.f1619 > 50.0) {
                        let t201: f64 = (50.0_f64).exp();let t202: f64 = (l.f1619 - 50.0);let t203: f64 = (1.0 + t202);let t204: f64 = (t201 * t203);
                        (t204, (t201 * l.f161a),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t205, t206,)
            }
        };
        (t207, t208,)
    }
};
            (l.f160d, l.f160e, ) = (t209, t20a, );
        }
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ffc == 0.0)) {let t20b: f64 = (l.f1653 * l.f1631);let t20c: f64 = (l.f160d - t20b);let t20d: f64 = (t20c - l.f1660);(l.f1643, l.f1644, ) = (t20d, ((l.f160e - (l.f1653 * l.f1632)) - l.f1661), );let t20e: f64 = (l.f1653 * l.f162d);let t20f: f64 = (l.f1603 - t20e);let t210: f64 = (t20f - l.f1660);let t211: f64 = (l.f164f * t210);(l.f163f, l.f1640, l.f1641, l.f1642, ) = (t211, ((l.f1650 * t210) + (l.f164f * ((l.f1604 - (l.f1653 * l.f162e)) - l.f1661))), (l.f164f * (l.f1605 - (l.f1653 * l.f162f))), (l.f164f * (l.f1606 - (l.f1653 * l.f1630))), );}
        let t212: f64 = if l.f1629 > 0.0 { 1.0 } else { 0.0 };l.f1ffd = t212;
        if (((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ffc == 0.0)) && (l.f1ffd != 0.0)) {let t213: f64 = (l.f1629 * l.f1657);l.f1658 = t213;let t214: f64 = (l.f1658 / l.f165a);let t215: f64 = (t214 * l.f1669);let t216: f64 = (t215 + l.f1623);(l.f1617, l.f1618, ) = (t216, (((-((l.f1658 * l.f165b) / (l.f165a * l.f165a))) * l.f1669) + l.f1624), );}
        if (((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ffc == 0.0)) && (l.f1ffd != 0.0)) {
            let t217: f64 = (-50.0);
            let (t224, t225,) = {
    if ((!(l.f1617 > 50.0)) && (!(l.f1617 < t217))) {
        let t218: f64 = (l.f1617).exp();
        (t218, (t218 * l.f1618),)
    } else {
        let t219: f64 = (-50.0);
        let (t222, t223,) = {
            if ((!(l.f1617 > 50.0)) && (l.f1617 < t219)) {
                let t21a: f64 = (-50.0);let t21b: f64 = (t21a).exp();
                (t21b, 0.0,)
            } else {
                let (t220, t221,) = {
                    if (l.f1617 > 50.0) {
                        let t21c: f64 = (50.0_f64).exp();let t21d: f64 = (l.f1617 - 50.0);let t21e: f64 = (1.0 + t21d);let t21f: f64 = (t21c * t21e);
                        (t21f, (t21c * l.f1618),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t220, t221,)
            }
        };
        (t222, t223,)
    }
};
            (l.f160b, l.f160c, ) = (t224, t225, );
        }
        if (((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ffc == 0.0)) && (l.f1ffd != 0.0)) {let t226: f64 = (l.f1653 * l.f1631);let t227: f64 = (l.f160b - t226);let t228: f64 = (t227 - l.f1660);(l.f163d, l.f163e, ) = (t228, ((l.f160c - (l.f1653 * l.f1632)) - l.f1661), );let t229: f64 = (l.f1658 / l.f165a);let t22a: f64 = (t229 * l.f1666);let t22b: f64 = (t22a + l.f1623);(l.f1613, l.f1614, l.f1615, l.f1616, ) = (t22b, (((-((l.f1658 * l.f165b) / (l.f165a * l.f165a))) * l.f1666) + l.f1624), (t229 * l.f1667), (t229 * l.f1668), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_135(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ffc == 0.0)) && (l.f1ffd != 0.0)) {
            let t22c: f64 = (-50.0);
            let (t23d, t23e, t23f, t240,) = {
    if ((!(l.f1613 > 50.0)) && (!(l.f1613 < t22c))) {
        let t22d: f64 = (l.f1613).exp();
        (t22d, (t22d * l.f1614), (t22d * l.f1615), (t22d * l.f1616),)
    } else {
        let t22e: f64 = (-50.0);
        let (t239, t23a, t23b, t23c,) = {
            if ((!(l.f1613 > 50.0)) && (l.f1613 < t22e)) {
                let t22f: f64 = (-50.0);let t230: f64 = (t22f).exp();
                (t230, 0.0, 0.0, 0.0,)
            } else {
                let (t235, t236, t237, t238,) = {
                    if (l.f1613 > 50.0) {
                        let t231: f64 = (50.0_f64).exp();let t232: f64 = (l.f1613 - 50.0);let t233: f64 = (1.0 + t232);let t234: f64 = (t231 * t233);
                        (t234, (t231 * l.f1614), (t231 * l.f1615), (t231 * l.f1616),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t235, t236, t237, t238,)
            }
        };
        (t239, t23a, t23b, t23c,)
    }
};
            (l.f1607, l.f1608, l.f1609, l.f160a, ) = (t23d, t23e, t23f, t240, );
        }
        if (((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ffc == 0.0)) && (l.f1ffd != 0.0)) {let t241: f64 = (l.f164f * l.f1643);let t242: f64 = (t241 / l.f163d);(l.f163b, l.f163c, ) = (t242, (((((l.f1650 * l.f1643) + (l.f164f * l.f1644)) * l.f163d) - (t241 * l.f163e)) / (l.f163d * l.f163d)), );let t243: f64 = (l.f1653 * l.f162d);let t244: f64 = (l.f1607 - t243);let t245: f64 = (t244 - l.f1660);let t246: f64 = (l.f163b * t245);(l.f1637, l.f1638, l.f1639, l.f163a, ) = (t246, ((l.f163c * t245) + (l.f163b * ((l.f1608 - (l.f1653 * l.f162e)) - l.f1661))), (l.f163b * (l.f1609 - (l.f1653 * l.f162f))), (l.f163b * (l.f160a - (l.f1653 * l.f1630))), );}
        if (((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ffc == 0.0)) && (l.f1ffd == 0.0)) {let t247: f64 = (l.f164f * l.f1643);(l.f1637, l.f1638, l.f1639, l.f163a, ) = (t247, ((l.f1650 * l.f1643) + (l.f164f * l.f1644)), 0.0, 0.0, );}
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ffc == 0.0)) {let t248: f64 = (l.f15ed * l.f15ed);let t249: f64 = (t248 * l.f165a);(l.f15eb, l.f15ec, ) = (t249, (t248 * l.f165b), );let t24a: f64 = (l.f15eb / 2.0);let t24b: f64 = (l.f1669 - t24a);let t24c: f64 = (l.f1666 - t24b);let t24d: f64 = (t24c / l.f15eb);(l.f15ff, l.f1600, l.f1601, l.f1602, ) = (t24d, ((((-(-(l.f15ec / 2.0))) * l.f15eb) - (t24c * l.f15ec)) / (l.f15eb * l.f15eb)), (l.f1667 / l.f15eb), (l.f1668 / l.f15eb), );}
        let t24e: f64 = if l.f15ff > 50.0 { 1.0 } else { 0.0 };l.f1ffe = t24e;
        if (((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ffc == 0.0)) && (l.f1ffe != 0.0)) {(l.f1625, l.f1626, l.f1627, l.f1628, ) = (0.0, 0.0, 0.0, 0.0, );}
        let t24f: f64 = (-50.0);let t250: f64 = if l.f15ff < t24f { 1.0 } else { 0.0 };l.f2001 = t250;
        if ((((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ffc == 0.0)) && (l.f1ffe == 0.0)) && (l.f2001 != 0.0)) {(l.f1625, l.f1626, l.f1627, l.f1628, ) = (1.0, 0.0, 0.0, 0.0, );}
        if ((((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ffc == 0.0)) && (l.f1ffe == 0.0)) && (l.f2001 == 0.0)) {let t251: f64 = (l.f15ff).exp();let t252: f64 = (1.0 + t251);let t253: f64 = (1.0 / t252);(l.f1625, l.f1626, l.f1627, l.f1628, ) = (t253, (-((t251 * l.f1600) / (t252 * t252))), (-((t251 * l.f1601) / (t252 * t252))), (-((t251 * l.f1602) / (t252 * t252))), );}
        if ((((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) && (l.f1ffc == 0.0)) {let t254: f64 = (l.f1625 * l.f163f);let t255: f64 = (1.0 - l.f1625);let t256: f64 = (t255 * l.f1637);let t257: f64 = (t254 + t256);(l.f1633, l.f1634, l.f1635, l.f1636, ) = (t257, (((l.f1626 * l.f163f) + (l.f1625 * l.f1640)) + (((-l.f1626) * l.f1637) + (t255 * l.f1638))), (((l.f1627 * l.f163f) + (l.f1625 * l.f1641)) + (((-l.f1627) * l.f1637) + (t255 * l.f1639))), (((l.f1628 * l.f163f) + (l.f1625 * l.f1642)) + (((-l.f1628) * l.f1637) + (t255 * l.f163a))), );}
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) {
            let t258: f64 = (-l.f1666);
            let (t267, t268, t269,) = {
    if (p.p52 != 0.0) {
        let t259: f64 = (l.f1666 / l.f166a);let t25a: f64 = (0.001 / p.p53);let t25b: f64 = (l.f1666 / l.f166a);let t25c: f64 = (t25a * t25b);let t25d: f64 = (t25c).tanh();let t25e: f64 = (t259 * t25d);
        (t25e, (((l.f1667 / l.f166a) * t25d) + (t259 * ((t25a * (l.f1667 / l.f166a)) / ((t25c).cosh() * (t25c).cosh())))), (((l.f1668 / l.f166a) * t25d) + (t259 * ((t25a * (l.f1668 / l.f166a)) / ((t25c).cosh() * (t25c).cosh())))),)
    } else {
        let (t264, t265, t266,) = {
            if (p.p52 == 0.0) {
                let __rspice_inv_cse_0: f64 = 1.0 / l.f166a;let t25f: f64 = (l.f1666 * __rspice_inv_cse_0);let t260: f64 = (l.f1666 * __rspice_inv_cse_0);let t261: f64 = (t25f * t260);let t262: f64 = (t261 + p.p53);let t263: f64 = (t262).sqrt();
                (t263, ((((l.f1667 / l.f166a) * t260) + (t25f * (l.f1667 / l.f166a))) / (2.0 * t263)), ((((l.f1668 / l.f166a) * t260) + (t25f * (l.f1668 / l.f166a))) / (2.0 * t263)),)
            } else {
                (0.0, 0.0, 0.0,)
            }
        };
        (t264, t265, t266,)
    }
};
            let t26a: f64 = (t267).powf(l.f15ee);let t26b: f64 = (1.0 + t26a);let t26c: f64 = (1.0 / l.f15ee);let t26d: f64 = (t26b).powf(t26c);let t26e: f64 = (t258 / t26d);(l.f162a, l.f162b, l.f162c, ) = (t26e, ((((-l.f1667) * t26d) - (t258 * if 0.0 == 0.0 && ((t26c) as f64).is_finite() && ((t26c) as f64).fract() == 0.0 { if t26c == 0.0 { 0.0 } else { (t26c * ((t26b).powf(t26c - 1.0) * if 0.0 == 0.0 && ((l.f15ee) as f64).is_finite() && ((l.f15ee) as f64).fract() == 0.0 { if l.f15ee == 0.0 { 0.0 } else { (l.f15ee * ((t267).powf(l.f15ee - 1.0) * t268)) } } else { (t26a * (l.f15ee * (t268 / t267))) })) } } else { (t26d * (t26c * (if 0.0 == 0.0 && ((l.f15ee) as f64).is_finite() && ((l.f15ee) as f64).fract() == 0.0 { if l.f15ee == 0.0 { 0.0 } else { (l.f15ee * ((t267).powf(l.f15ee - 1.0) * t268)) } } else { (t26a * (l.f15ee * (t268 / t267))) } / t26b))) })) / (t26d * t26d)), ((((-l.f1668) * t26d) - (t258 * if 0.0 == 0.0 && ((t26c) as f64).is_finite() && ((t26c) as f64).fract() == 0.0 { if t26c == 0.0 { 0.0 } else { (t26c * ((t26b).powf(t26c - 1.0) * if 0.0 == 0.0 && ((l.f15ee) as f64).is_finite() && ((l.f15ee) as f64).fract() == 0.0 { if l.f15ee == 0.0 { 0.0 } else { (l.f15ee * ((t267).powf(l.f15ee - 1.0) * t269)) } } else { (t26a * (l.f15ee * (t269 / t267))) })) } } else { (t26d * (t26c * (if 0.0 == 0.0 && ((l.f15ee) as f64).is_finite() && ((l.f15ee) as f64).fract() == 0.0 { if l.f15ee == 0.0 { 0.0 } else { (l.f15ee * ((t267).powf(l.f15ee - 1.0) * t269)) } } else { (t26a * (l.f15ee * (t269 / t267))) } / t26b))) })) / (t26d * t26d)), );
        }
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) {let t26f: f64 = (-l.f1664);let t270: f64 = (t26f * l.f166c);let t271: f64 = (t270 * l.f1654);let t272: f64 = (t271 * l.f164e);let t273: f64 = (t272 * l.f1662);let t274: f64 = t273;(l.f1651, l.f1652, ) = (t274, (t272 * l.f1663), );let t275: f64 = (l.f1659 / l.f165a);let t276: f64 = (t275 * l.f162a);(l.f161f, l.f1620, l.f1621, l.f1622, ) = (t276, ((-((l.f1659 * l.f165b) / (l.f165a * l.f165a))) * l.f162a), (t275 * l.f162b), (t275 * l.f162c), );}
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) {
            let t277: f64 = (-50.0);
            let (t288, t289, t28a, t28b,) = {
    if ((!(l.f161f > 50.0)) && (!(l.f161f < t277))) {
        let t278: f64 = (l.f161f).exp();
        (t278, (t278 * l.f1620), (t278 * l.f1621), (t278 * l.f1622),)
    } else {
        let t279: f64 = (-50.0);
        let (t284, t285, t286, t287,) = {
            if ((!(l.f161f > 50.0)) && (l.f161f < t279)) {
                let t27a: f64 = (-50.0);let t27b: f64 = (t27a).exp();
                (t27b, 0.0, 0.0, 0.0,)
            } else {
                let (t280, t281, t282, t283,) = {
                    if (l.f161f > 50.0) {
                        let t27c: f64 = (50.0_f64).exp();let t27d: f64 = (l.f161f - 50.0);let t27e: f64 = (1.0 + t27d);let t27f: f64 = (t27c * t27e);
                        (t27f, (t27c * l.f1620), (t27c * l.f1621), (t27c * l.f1622),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t280, t281, t282, t283,)
            }
        };
        (t284, t285, t286, t287,)
    }
};
            (l.f161b, l.f161c, l.f161d, l.f161e, ) = (t288, t289, t28a, t28b, );
        }
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) {let t28c: f64 = (l.f161b - 1.0);let t28d: f64 = (l.f1651 * t28c);(l.f1645, l.f1646, l.f1647, l.f1648, ) = (t28d, ((l.f1652 * t28c) + (l.f1651 * l.f161c)), (l.f1651 * l.f161d), (l.f1651 * l.f161e), );let t28e: f64 = (l.f1633 + l.f1645);(l.f1649, l.f164a, l.f164b, l.f164c, ) = (t28e, (l.f1634 + l.f1646), (l.f1635 + l.f1647), (l.f1636 + l.f1648), );(l.f165c, l.f165d, l.f165e, l.f165f, ) = (l.f1649, l.f164a, l.f164b, l.f164c, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_136(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);let nv8 = ctx.node_voltage(nodes[8]);
        if (((l.f1fd4 != 0.0) && (l.f1fea != 0.0)) && (l.f1ff7 != 0.0)) {(l.f2121, l.f2122, l.f2123, l.f2124, ) = (l.f165c, l.f165d, l.f165e, l.f165f, );}
        (l.f23c7, l.f23c8, l.f23c9, ) = (0.0, 0.0, 0.0, );(l.f20c6, l.f20cb, l.f20cc, l.f20cd, ) = (0.0, 0.0, 0.0, 0.0, );(l.f20c7, l.f20c8, l.f20c9, l.f20ca, ) = (0.0, 0.0, 0.0, 0.0, );(l.f2280, l.f22a1, l.f22a2, ) = (0.0, 0.0, 0.0, );l.f2281 = 0.0;(l.f2283, l.f2284, l.f2285, ) = (0.0, 0.0, 0.0, );(l.f2289, l.f228a, l.f228b, ) = (0.0, 0.0, 0.0, );(l.f228f, l.f2290, l.f2291, ) = (0.0, 0.0, 0.0, );(l.f2295, l.f2296, l.f2297, ) = (0.0, 0.0, 0.0, );(l.f229b, l.f229c, l.f229d, ) = (0.0, 0.0, 0.0, );(l.f23cb, l.f23cc, l.f23cd, ) = (0.0, 0.0, 0.0, );(l.f23cf, l.f23d0, l.f23d1, ) = (0.0, 0.0, 0.0, );(l.f23d3, l.f23d4, l.f23d5, ) = (0.0, 0.0, 0.0, );(l.f23d7, l.f23d8, l.f23d9, ) = (0.0, 0.0, 0.0, );(l.f23db, l.f23dc, l.f23dd, ) = (0.0, 0.0, 0.0, );l.f22ea = 0.0;let t28f: f64 = if p.p291 == 1.0 { 1.0 } else { 0.0 };l.f2002 = t28f;
        if (l.f2002 != 0.0) {let t290: f64 = (p.p6 * (nv8 - nv7));(l.f23c7, l.f23c8, l.f23c9, ) = (t290, (-p.p6), p.p6, );(l.f16de, l.f16df, l.f16e0, l.f16e1, ) = (0.0, 0.0, 0.0, 0.0, );(l.f16d1, l.f16d2, ) = (0.0, 0.0, );(l.f16d3, l.f16d4, ) = (0.0, 0.0, );(l.f16e8, l.f16e9, l.f16ea, ) = (l.f23c7, l.f23c8, l.f23c9, );(l.f16dc, l.f16dd, ) = (l.f215b, l.f215c, );l.f16eb = p.p294;l.f166f = p.p296;l.f16ab = p.p295;l.f16d9 = p.p292;l.f16d7 = 4.0;l.f16e7 = 600.0;(l.f16e4, l.f16e5, ) = (l.f22f2, l.f22f3, );let t291: f64 = (1.0 - p.p311);let t292: f64 = (p.p0 * t291);l.f16ee = t292;l.f16d6 = p.p2;l.f16cf = p.p293;l.f16d5 = 0.0;l.f16ec = p.p299;l.f1670 = p.p300;l.f16d0 = p.p298;l.f16db = p.p297;l.f16d8 = 0.0;l.f16ed = 0.0;l.f16e6 = p.p6;(l.f16cb, l.f16cc, l.f16cd, l.f16ce, ) = (0.0, 0.0, 0.0, 0.0, );(l.f166d, l.f166e, ) = (0.0, 0.0, );(l.f16e2, l.f16e3, ) = (0.0, 0.0, );(l.f16a7, l.f16a8, l.f16a9, l.f16aa, ) = (0.0, 0.0, 0.0, 0.0, );(l.f16af, l.f16b0, l.f16b1, l.f16b2, ) = (0.0, 0.0, 0.0, 0.0, );(l.f16b5, l.f16b6, l.f16b7, l.f16b8, ) = (0.0, 0.0, 0.0, 0.0, );(l.f16ac, l.f16ad, l.f16ae, ) = (0.0, 0.0, 0.0, );(l.f16c7, l.f16c8, l.f16c9, l.f16ca, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1679, l.f167a, l.f167b, l.f167c, ) = (0.0, 0.0, 0.0, 0.0, );(l.f167f, l.f1680, ) = (0.0, 0.0, );(l.f1671, l.f1672, l.f1673, l.f1674, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1677, l.f1678, ) = (0.0, 0.0, );(l.f16a5, l.f16a6, ) = (0.0, 0.0, );(l.f1681, l.f1682, l.f1683, l.f1684, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1691, l.f1692, l.f1693, l.f1694, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1685, l.f1686, l.f1687, l.f1688, ) = (0.0, 0.0, 0.0, 0.0, );(l.f16a1, l.f16a2, l.f16a3, l.f16a4, ) = (0.0, 0.0, 0.0, 0.0, );(l.f169d, l.f169e, l.f169f, l.f16a0, ) = (0.0, 0.0, 0.0, 0.0, );l.f16da = 0.0;(l.f167d, l.f167e, ) = (0.0, 0.0, );(l.f1675, l.f1676, ) = (0.0, 0.0, );(l.f16b3, l.f16b4, ) = (0.0, 0.0, );(l.f169b, l.f169c, ) = (0.0, 0.0, );(l.f168f, l.f1690, ) = (0.0, 0.0, );(l.f16c5, l.f16c6, ) = (0.0, 0.0, );(l.f16c1, l.f16c2, l.f16c3, l.f16c4, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1699, l.f169a, ) = (0.0, 0.0, );(l.f168d, l.f168e, ) = (0.0, 0.0, );(l.f16bf, l.f16c0, ) = (0.0, 0.0, );(l.f1695, l.f1696, l.f1697, l.f1698, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1689, l.f168a, l.f168b, l.f168c, ) = (0.0, 0.0, 0.0, 0.0, );(l.f16bd, l.f16be, ) = (0.0, 0.0, );(l.f16b9, l.f16ba, l.f16bb, l.f16bc, ) = (0.0, 0.0, 0.0, 0.0, );let t293: f64 = (l.f16d8 / l.f16dc);let t294: f64 = (-l.f16ed);let t295: f64 = (t293 * t294);(l.f16a5, l.f16a6, ) = (t295, ((-((l.f16d8 * l.f16dd) / (l.f16dc * l.f16dc))) * t294), );}
        if (l.f2002 != 0.0) {
            let t296: f64 = (-50.0);
            let (t2a3, t2a4,) = {
    if ((!(l.f16a5 > 50.0)) && (!(l.f16a5 < t296))) {
        let t297: f64 = (l.f16a5).exp();
        (t297, (t297 * l.f16a6),)
    } else {
        let t298: f64 = (-50.0);
        let (t2a1, t2a2,) = {
            if ((!(l.f16a5 > 50.0)) && (l.f16a5 < t298)) {
                let t299: f64 = (-50.0);let t29a: f64 = (t299).exp();
                (t29a, 0.0,)
            } else {
                let (t29f, t2a0,) = {
                    if (l.f16a5 > 50.0) {
                        let t29b: f64 = (50.0_f64).exp();let t29c: f64 = (l.f16a5 - 50.0);let t29d: f64 = (1.0 + t29c);let t29e: f64 = (t29b * t29d);
                        (t29e, (t29b * l.f16a6),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t29f, t2a0,)
            }
        };
        (t2a1, t2a2,)
    }
};
            (l.f16e2, l.f16e3, ) = (t2a3, t2a4, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_137(
        l: &mut StampLocals,
    ) {
        if (l.f2002 != 0.0) {let t2a5: f64 = (-l.f16e8);let t2a6: f64 = (t2a5 - l.f16e7);let t2a7: f64 = (l.f16d7 * t2a6);let t2a8: f64 = (t2a7 + l.f16a5);(l.f1679, l.f167a, l.f167b, l.f167c, ) = (t2a8, l.f16a6, (l.f16d7 * (-l.f16e9)), (l.f16d7 * (-l.f16ea)), );let t2a9: f64 = (-l.f16d7);let t2aa: f64 = (t2a9 * l.f16e7);let t2ab: f64 = (t2aa + l.f16a5);(l.f167f, l.f1680, ) = (t2ab, l.f16a6, );}
        if (l.f2002 != 0.0) {
            let t2ac: f64 = (-50.0);
            let (t2bd, t2be, t2bf, t2c0,) = {
    if ((!(l.f1679 > 50.0)) && (!(l.f1679 < t2ac))) {
        let t2ad: f64 = (l.f1679).exp();
        (t2ad, (t2ad * l.f167a), (t2ad * l.f167b), (t2ad * l.f167c),)
    } else {
        let t2ae: f64 = (-50.0);
        let (t2b9, t2ba, t2bb, t2bc,) = {
            if ((!(l.f1679 > 50.0)) && (l.f1679 < t2ae)) {
                let t2af: f64 = (-50.0);let t2b0: f64 = (t2af).exp();
                (t2b0, 0.0, 0.0, 0.0,)
            } else {
                let (t2b5, t2b6, t2b7, t2b8,) = {
                    if (l.f1679 > 50.0) {
                        let t2b1: f64 = (50.0_f64).exp();let t2b2: f64 = (l.f1679 - 50.0);let t2b3: f64 = (1.0 + t2b2);let t2b4: f64 = (t2b1 * t2b3);
                        (t2b4, (t2b1 * l.f167a), (t2b1 * l.f167b), (t2b1 * l.f167c),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t2b5, t2b6, t2b7, t2b8,)
            }
        };
        (t2b9, t2ba, t2bb, t2bc,)
    }
};
            (l.f1671, l.f1672, l.f1673, l.f1674, ) = (t2bd, t2be, t2bf, t2c0, );
        }
        if (l.f2002 != 0.0) {
            let t2c1: f64 = (-50.0);
            let (t2ce, t2cf,) = {
    if ((!(l.f167f > 50.0)) && (!(l.f167f < t2c1))) {
        let t2c2: f64 = (l.f167f).exp();
        (t2c2, (t2c2 * l.f1680),)
    } else {
        let t2c3: f64 = (-50.0);
        let (t2cc, t2cd,) = {
            if ((!(l.f167f > 50.0)) && (l.f167f < t2c3)) {
                let t2c4: f64 = (-50.0);let t2c5: f64 = (t2c4).exp();
                (t2c5, 0.0,)
            } else {
                let (t2ca, t2cb,) = {
                    if (l.f167f > 50.0) {
                        let t2c6: f64 = (50.0_f64).exp();let t2c7: f64 = (l.f167f - 50.0);let t2c8: f64 = (1.0 + t2c7);let t2c9: f64 = (t2c6 * t2c8);
                        (t2c9, (t2c6 * l.f1680),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t2ca, t2cb,)
            }
        };
        (t2cc, t2cd,)
    }
};
            (l.f1677, l.f1678, ) = (t2ce, t2cf, );
        }
        if (l.f2002 != 0.0) {let t2d0: f64 = (l.f1671 - l.f1677);(l.f16af, l.f16b0, l.f16b1, l.f16b2, ) = (t2d0, (l.f1672 - l.f1678), l.f1673, l.f1674, );let t2d1: f64 = (l.f16e6 * l.f16ee);let t2d2: f64 = (t2d1 * l.f16d6);let t2d3: f64 = (t2d2 * l.f16cf);let t2d4: f64 = (t2d3 * l.f16e4);(l.f16d1, l.f16d2, ) = (t2d4, (t2d3 * l.f16e5), );let t2d5: f64 = (l.f16d9 / l.f16dc);let t2d6: f64 = (t2d5 * l.f16e8);let t2d7: f64 = (t2d6 + l.f16a5);(l.f1691, l.f1692, l.f1693, l.f1694, ) = (t2d7, (((-((l.f16d9 * l.f16dd) / (l.f16dc * l.f16dc))) * l.f16e8) + l.f16a6), (t2d5 * l.f16e9), (t2d5 * l.f16ea), );}
        if (l.f2002 != 0.0) {
            let t2d8: f64 = (-50.0);
            let (t2e9, t2ea, t2eb, t2ec,) = {
    if ((!(l.f1691 > 50.0)) && (!(l.f1691 < t2d8))) {
        let t2d9: f64 = (l.f1691).exp();
        (t2d9, (t2d9 * l.f1692), (t2d9 * l.f1693), (t2d9 * l.f1694),)
    } else {
        let t2da: f64 = (-50.0);
        let (t2e5, t2e6, t2e7, t2e8,) = {
            if ((!(l.f1691 > 50.0)) && (l.f1691 < t2da)) {
                let t2db: f64 = (-50.0);let t2dc: f64 = (t2db).exp();
                (t2dc, 0.0, 0.0, 0.0,)
            } else {
                let (t2e1, t2e2, t2e3, t2e4,) = {
                    if (l.f1691 > 50.0) {
                        let t2dd: f64 = (50.0_f64).exp();let t2de: f64 = (l.f1691 - 50.0);let t2df: f64 = (1.0 + t2de);let t2e0: f64 = (t2dd * t2df);
                        (t2e0, (t2dd * l.f1692), (t2dd * l.f1693), (t2dd * l.f1694),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t2e1, t2e2, t2e3, t2e4,)
            }
        };
        (t2e5, t2e6, t2e7, t2e8,)
    }
};
            (l.f1685, l.f1686, l.f1687, l.f1688, ) = (t2e9, t2ea, t2eb, t2ec, );
        }
        let t2ed: f64 = if l.f16ab == 1.0 { 1.0 } else { 0.0 };l.f2004 = t2ed;
        if ((l.f2002 != 0.0) && (l.f2004 != 0.0)) {let t2ee: f64 = (l.f16d5 * l.f16af);let t2ef: f64 = (l.f1685 - t2ee);let t2f0: f64 = (t2ef - l.f16e2);let t2f1: f64 = (l.f16d1 * t2f0);(l.f16b5, l.f16b6, l.f16b7, l.f16b8, ) = (t2f1, ((l.f16d2 * t2f0) + (l.f16d1 * ((l.f1686 - (l.f16d5 * l.f16b0)) - l.f16e3))), (l.f16d1 * (l.f1687 - (l.f16d5 * l.f16b1))), (l.f16d1 * (l.f1688 - (l.f16d5 * l.f16b2))), );}
        if ((l.f2002 != 0.0) && (l.f2004 == 0.0)) {let t2f2: f64 = (-l.f16eb);let t2f3: f64 = (t2f2 - l.f16e7);let t2f4: f64 = (l.f16d7 * t2f3);let t2f5: f64 = (t2f4 + l.f16a5);(l.f167d, l.f167e, ) = (t2f5, l.f16a6, );}
        if ((l.f2002 != 0.0) && (l.f2004 == 0.0)) {
            let t2f6: f64 = (-50.0);
            let (t303, t304,) = {
    if ((!(l.f167d > 50.0)) && (!(l.f167d < t2f6))) {
        let t2f7: f64 = (l.f167d).exp();
        (t2f7, (t2f7 * l.f167e),)
    } else {
        let t2f8: f64 = (-50.0);
        let (t301, t302,) = {
            if ((!(l.f167d > 50.0)) && (l.f167d < t2f8)) {
                let t2f9: f64 = (-50.0);let t2fa: f64 = (t2f9).exp();
                (t2fa, 0.0,)
            } else {
                let (t2ff, t300,) = {
                    if (l.f167d > 50.0) {
                        let t2fb: f64 = (50.0_f64).exp();let t2fc: f64 = (l.f167d - 50.0);let t2fd: f64 = (1.0 + t2fc);let t2fe: f64 = (t2fb * t2fd);
                        (t2fe, (t2fb * l.f167e),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t2ff, t300,)
            }
        };
        (t301, t302,)
    }
};
            (l.f1675, l.f1676, ) = (t303, t304, );
        }
        if ((l.f2002 != 0.0) && (l.f2004 == 0.0)) {let t305: f64 = (l.f1675 - l.f1677);(l.f16b3, l.f16b4, ) = (t305, (l.f1676 - l.f1678), );let t306: f64 = (l.f16d9 / l.f16dc);let t307: f64 = (t306 * l.f16eb);let t308: f64 = (t307 + l.f16a5);(l.f169b, l.f169c, ) = (t308, (((-((l.f16d9 * l.f16dd) / (l.f16dc * l.f16dc))) * l.f16eb) + l.f16a6), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_138(
        l: &mut StampLocals,
    ) {
        if ((l.f2002 != 0.0) && (l.f2004 == 0.0)) {
            let t309: f64 = (-50.0);
            let (t316, t317,) = {
    if ((!(l.f169b > 50.0)) && (!(l.f169b < t309))) {
        let t30a: f64 = (l.f169b).exp();
        (t30a, (t30a * l.f169c),)
    } else {
        let t30b: f64 = (-50.0);
        let (t314, t315,) = {
            if ((!(l.f169b > 50.0)) && (l.f169b < t30b)) {
                let t30c: f64 = (-50.0);let t30d: f64 = (t30c).exp();
                (t30d, 0.0,)
            } else {
                let (t312, t313,) = {
                    if (l.f169b > 50.0) {
                        let t30e: f64 = (50.0_f64).exp();let t30f: f64 = (l.f169b - 50.0);let t310: f64 = (1.0 + t30f);let t311: f64 = (t30e * t310);
                        (t311, (t30e * l.f169c),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t312, t313,)
            }
        };
        (t314, t315,)
    }
};
            (l.f168f, l.f1690, ) = (t316, t317, );
        }
        if ((l.f2002 != 0.0) && (l.f2004 == 0.0)) {let t318: f64 = (l.f16d5 * l.f16b3);let t319: f64 = (l.f168f - t318);let t31a: f64 = (t319 - l.f16e2);(l.f16c5, l.f16c6, ) = (t31a, ((l.f1690 - (l.f16d5 * l.f16b4)) - l.f16e3), );let t31b: f64 = (l.f16d5 * l.f16af);let t31c: f64 = (l.f1685 - t31b);let t31d: f64 = (t31c - l.f16e2);let t31e: f64 = (l.f16d1 * t31d);(l.f16c1, l.f16c2, l.f16c3, l.f16c4, ) = (t31e, ((l.f16d2 * t31d) + (l.f16d1 * ((l.f1686 - (l.f16d5 * l.f16b0)) - l.f16e3))), (l.f16d1 * (l.f1687 - (l.f16d5 * l.f16b1))), (l.f16d1 * (l.f1688 - (l.f16d5 * l.f16b2))), );}
        let t31f: f64 = if l.f16ab > 0.0 { 1.0 } else { 0.0 };l.f2005 = t31f;
        if (((l.f2002 != 0.0) && (l.f2004 == 0.0)) && (l.f2005 != 0.0)) {let t320: f64 = (l.f16ab * l.f16d9);l.f16da = t320;let t321: f64 = (l.f16da / l.f16dc);let t322: f64 = (t321 * l.f16eb);let t323: f64 = (t322 + l.f16a5);(l.f1699, l.f169a, ) = (t323, (((-((l.f16da * l.f16dd) / (l.f16dc * l.f16dc))) * l.f16eb) + l.f16a6), );}
        if (((l.f2002 != 0.0) && (l.f2004 == 0.0)) && (l.f2005 != 0.0)) {
            let t324: f64 = (-50.0);
            let (t331, t332,) = {
    if ((!(l.f1699 > 50.0)) && (!(l.f1699 < t324))) {
        let t325: f64 = (l.f1699).exp();
        (t325, (t325 * l.f169a),)
    } else {
        let t326: f64 = (-50.0);
        let (t32f, t330,) = {
            if ((!(l.f1699 > 50.0)) && (l.f1699 < t326)) {
                let t327: f64 = (-50.0);let t328: f64 = (t327).exp();
                (t328, 0.0,)
            } else {
                let (t32d, t32e,) = {
                    if (l.f1699 > 50.0) {
                        let t329: f64 = (50.0_f64).exp();let t32a: f64 = (l.f1699 - 50.0);let t32b: f64 = (1.0 + t32a);let t32c: f64 = (t329 * t32b);
                        (t32c, (t329 * l.f169a),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t32d, t32e,)
            }
        };
        (t32f, t330,)
    }
};
            (l.f168d, l.f168e, ) = (t331, t332, );
        }
        if (((l.f2002 != 0.0) && (l.f2004 == 0.0)) && (l.f2005 != 0.0)) {let t333: f64 = (l.f16d5 * l.f16b3);let t334: f64 = (l.f168d - t333);let t335: f64 = (t334 - l.f16e2);(l.f16bf, l.f16c0, ) = (t335, ((l.f168e - (l.f16d5 * l.f16b4)) - l.f16e3), );let t336: f64 = (l.f16da / l.f16dc);let t337: f64 = (t336 * l.f16e8);let t338: f64 = (t337 + l.f16a5);(l.f1695, l.f1696, l.f1697, l.f1698, ) = (t338, (((-((l.f16da * l.f16dd) / (l.f16dc * l.f16dc))) * l.f16e8) + l.f16a6), (t336 * l.f16e9), (t336 * l.f16ea), );}
        if (((l.f2002 != 0.0) && (l.f2004 == 0.0)) && (l.f2005 != 0.0)) {
            let t339: f64 = (-50.0);
            let (t34a, t34b, t34c, t34d,) = {
    if ((!(l.f1695 > 50.0)) && (!(l.f1695 < t339))) {
        let t33a: f64 = (l.f1695).exp();
        (t33a, (t33a * l.f1696), (t33a * l.f1697), (t33a * l.f1698),)
    } else {
        let t33b: f64 = (-50.0);
        let (t346, t347, t348, t349,) = {
            if ((!(l.f1695 > 50.0)) && (l.f1695 < t33b)) {
                let t33c: f64 = (-50.0);let t33d: f64 = (t33c).exp();
                (t33d, 0.0, 0.0, 0.0,)
            } else {
                let (t342, t343, t344, t345,) = {
                    if (l.f1695 > 50.0) {
                        let t33e: f64 = (50.0_f64).exp();let t33f: f64 = (l.f1695 - 50.0);let t340: f64 = (1.0 + t33f);let t341: f64 = (t33e * t340);
                        (t341, (t33e * l.f1696), (t33e * l.f1697), (t33e * l.f1698),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t342, t343, t344, t345,)
            }
        };
        (t346, t347, t348, t349,)
    }
};
            (l.f1689, l.f168a, l.f168b, l.f168c, ) = (t34a, t34b, t34c, t34d, );
        }
        if (((l.f2002 != 0.0) && (l.f2004 == 0.0)) && (l.f2005 != 0.0)) {let t34e: f64 = (l.f16d1 * l.f16c5);let t34f: f64 = (t34e / l.f16bf);(l.f16bd, l.f16be, ) = (t34f, (((((l.f16d2 * l.f16c5) + (l.f16d1 * l.f16c6)) * l.f16bf) - (t34e * l.f16c0)) / (l.f16bf * l.f16bf)), );let t350: f64 = (l.f16d5 * l.f16af);let t351: f64 = (l.f1689 - t350);let t352: f64 = (t351 - l.f16e2);let t353: f64 = (l.f16bd * t352);(l.f16b9, l.f16ba, l.f16bb, l.f16bc, ) = (t353, ((l.f16be * t352) + (l.f16bd * ((l.f168a - (l.f16d5 * l.f16b0)) - l.f16e3))), (l.f16bd * (l.f168b - (l.f16d5 * l.f16b1))), (l.f16bd * (l.f168c - (l.f16d5 * l.f16b2))), );}
        if (((l.f2002 != 0.0) && (l.f2004 == 0.0)) && (l.f2005 == 0.0)) {let t354: f64 = (l.f16d1 * l.f16c5);(l.f16b9, l.f16ba, l.f16bb, l.f16bc, ) = (t354, ((l.f16d2 * l.f16c5) + (l.f16d1 * l.f16c6)), 0.0, 0.0, );}
        if ((l.f2002 != 0.0) && (l.f2004 == 0.0)) {let t355: f64 = (l.f166f * l.f166f);let t356: f64 = (t355 * l.f16dc);(l.f166d, l.f166e, ) = (t356, (t355 * l.f16dd), );let t357: f64 = (l.f166d / 2.0);let t358: f64 = (l.f16eb - t357);let t359: f64 = (l.f16e8 - t358);let t35a: f64 = (t359 / l.f166d);(l.f1681, l.f1682, l.f1683, l.f1684, ) = (t35a, ((((-(-(l.f166e / 2.0))) * l.f166d) - (t359 * l.f166e)) / (l.f166d * l.f166d)), (l.f16e9 / l.f166d), (l.f16ea / l.f166d), );}
        let t35b: f64 = if l.f1681 > 50.0 { 1.0 } else { 0.0 };l.f2006 = t35b;
        if (((l.f2002 != 0.0) && (l.f2004 == 0.0)) && (l.f2006 != 0.0)) {(l.f16a7, l.f16a8, l.f16a9, l.f16aa, ) = (0.0, 0.0, 0.0, 0.0, );}
        let t35c: f64 = (-50.0);let t35d: f64 = if l.f1681 < t35c { 1.0 } else { 0.0 };l.f2007 = t35d;
        if ((((l.f2002 != 0.0) && (l.f2004 == 0.0)) && (l.f2006 == 0.0)) && (l.f2007 != 0.0)) {(l.f16a7, l.f16a8, l.f16a9, l.f16aa, ) = (1.0, 0.0, 0.0, 0.0, );}
        if ((((l.f2002 != 0.0) && (l.f2004 == 0.0)) && (l.f2006 == 0.0)) && (l.f2007 == 0.0)) {let t35e: f64 = (l.f1681).exp();let t35f: f64 = (1.0 + t35e);let t360: f64 = (1.0 / t35f);(l.f16a7, l.f16a8, l.f16a9, l.f16aa, ) = (t360, (-((t35e * l.f1682) / (t35f * t35f))), (-((t35e * l.f1683) / (t35f * t35f))), (-((t35e * l.f1684) / (t35f * t35f))), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_139(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.f2002 != 0.0) && (l.f2004 == 0.0)) {let t361: f64 = (l.f16a7 * l.f16c1);let t362: f64 = (1.0 - l.f16a7);let t363: f64 = (t362 * l.f16b9);let t364: f64 = (t361 + t363);(l.f16b5, l.f16b6, l.f16b7, l.f16b8, ) = (t364, (((l.f16a8 * l.f16c1) + (l.f16a7 * l.f16c2)) + (((-l.f16a8) * l.f16b9) + (t362 * l.f16ba))), (((l.f16a9 * l.f16c1) + (l.f16a7 * l.f16c3)) + (((-l.f16a9) * l.f16b9) + (t362 * l.f16bb))), (((l.f16aa * l.f16c1) + (l.f16a7 * l.f16c4)) + (((-l.f16aa) * l.f16b9) + (t362 * l.f16bc))), );}
        if (l.f2002 != 0.0) {
            let t365: f64 = (-l.f16e8);
            let (t374, t375, t376,) = {
    if (p.p52 != 0.0) {
        let t366: f64 = (l.f16e8 / l.f16ec);let t367: f64 = (0.001 / p.p53);let t368: f64 = (l.f16e8 / l.f16ec);let t369: f64 = (t367 * t368);let t36a: f64 = (t369).tanh();let t36b: f64 = (t366 * t36a);
        (t36b, (((l.f16e9 / l.f16ec) * t36a) + (t366 * ((t367 * (l.f16e9 / l.f16ec)) / ((t369).cosh() * (t369).cosh())))), (((l.f16ea / l.f16ec) * t36a) + (t366 * ((t367 * (l.f16ea / l.f16ec)) / ((t369).cosh() * (t369).cosh())))),)
    } else {
        let (t371, t372, t373,) = {
            if (p.p52 == 0.0) {
                let __rspice_inv_cse_0: f64 = 1.0 / l.f16ec;let t36c: f64 = (l.f16e8 * __rspice_inv_cse_0);let t36d: f64 = (l.f16e8 * __rspice_inv_cse_0);let t36e: f64 = (t36c * t36d);let t36f: f64 = (t36e + p.p53);let t370: f64 = (t36f).sqrt();
                (t370, ((((l.f16e9 / l.f16ec) * t36d) + (t36c * (l.f16e9 / l.f16ec))) / (2.0 * t370)), ((((l.f16ea / l.f16ec) * t36d) + (t36c * (l.f16ea / l.f16ec))) / (2.0 * t370)),)
            } else {
                (0.0, 0.0, 0.0,)
            }
        };
        (t371, t372, t373,)
    }
};
            let t377: f64 = (t374).powf(l.f1670);let t378: f64 = (1.0 + t377);let t379: f64 = (1.0 / l.f1670);let t37a: f64 = (t378).powf(t379);let t37b: f64 = (t365 / t37a);(l.f16ac, l.f16ad, l.f16ae, ) = (t37b, ((((-l.f16e9) * t37a) - (t365 * if 0.0 == 0.0 && ((t379) as f64).is_finite() && ((t379) as f64).fract() == 0.0 { if t379 == 0.0 { 0.0 } else { (t379 * ((t378).powf(t379 - 1.0) * if 0.0 == 0.0 && ((l.f1670) as f64).is_finite() && ((l.f1670) as f64).fract() == 0.0 { if l.f1670 == 0.0 { 0.0 } else { (l.f1670 * ((t374).powf(l.f1670 - 1.0) * t375)) } } else { (t377 * (l.f1670 * (t375 / t374))) })) } } else { (t37a * (t379 * (if 0.0 == 0.0 && ((l.f1670) as f64).is_finite() && ((l.f1670) as f64).fract() == 0.0 { if l.f1670 == 0.0 { 0.0 } else { (l.f1670 * ((t374).powf(l.f1670 - 1.0) * t375)) } } else { (t377 * (l.f1670 * (t375 / t374))) } / t378))) })) / (t37a * t37a)), ((((-l.f16ea) * t37a) - (t365 * if 0.0 == 0.0 && ((t379) as f64).is_finite() && ((t379) as f64).fract() == 0.0 { if t379 == 0.0 { 0.0 } else { (t379 * ((t378).powf(t379 - 1.0) * if 0.0 == 0.0 && ((l.f1670) as f64).is_finite() && ((l.f1670) as f64).fract() == 0.0 { if l.f1670 == 0.0 { 0.0 } else { (l.f1670 * ((t374).powf(l.f1670 - 1.0) * t376)) } } else { (t377 * (l.f1670 * (t376 / t374))) })) } } else { (t37a * (t379 * (if 0.0 == 0.0 && ((l.f1670) as f64).is_finite() && ((l.f1670) as f64).fract() == 0.0 { if l.f1670 == 0.0 { 0.0 } else { (l.f1670 * ((t374).powf(l.f1670 - 1.0) * t376)) } } else { (t377 * (l.f1670 * (t376 / t374))) } / t378))) })) / (t37a * t37a)), );
        }
        if (l.f2002 != 0.0) {let t37c: f64 = (-l.f16e6);let t37d: f64 = (t37c * l.f16ee);let t37e: f64 = (t37d * l.f16d6);let t37f: f64 = (t37e * l.f16d0);let t380: f64 = (t37f * l.f16e4);let t381: f64 = t380;(l.f16d3, l.f16d4, ) = (t381, (t37f * l.f16e5), );let t382: f64 = (l.f16db / l.f16dc);let t383: f64 = (t382 * l.f16ac);(l.f16a1, l.f16a2, l.f16a3, l.f16a4, ) = (t383, ((-((l.f16db * l.f16dd) / (l.f16dc * l.f16dc))) * l.f16ac), (t382 * l.f16ad), (t382 * l.f16ae), );}
        if (l.f2002 != 0.0) {
            let t384: f64 = (-50.0);
            let (t395, t396, t397, t398,) = {
    if ((!(l.f16a1 > 50.0)) && (!(l.f16a1 < t384))) {
        let t385: f64 = (l.f16a1).exp();
        (t385, (t385 * l.f16a2), (t385 * l.f16a3), (t385 * l.f16a4),)
    } else {
        let t386: f64 = (-50.0);
        let (t391, t392, t393, t394,) = {
            if ((!(l.f16a1 > 50.0)) && (l.f16a1 < t386)) {
                let t387: f64 = (-50.0);let t388: f64 = (t387).exp();
                (t388, 0.0, 0.0, 0.0,)
            } else {
                let (t38d, t38e, t38f, t390,) = {
                    if (l.f16a1 > 50.0) {
                        let t389: f64 = (50.0_f64).exp();let t38a: f64 = (l.f16a1 - 50.0);let t38b: f64 = (1.0 + t38a);let t38c: f64 = (t389 * t38b);
                        (t38c, (t389 * l.f16a2), (t389 * l.f16a3), (t389 * l.f16a4),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t38d, t38e, t38f, t390,)
            }
        };
        (t391, t392, t393, t394,)
    }
};
            (l.f169d, l.f169e, l.f169f, l.f16a0, ) = (t395, t396, t397, t398, );
        }
        if (l.f2002 != 0.0) {let t399: f64 = (l.f169d - 1.0);let t39a: f64 = (l.f16d3 * t399);(l.f16c7, l.f16c8, l.f16c9, l.f16ca, ) = (t39a, ((l.f16d4 * t399) + (l.f16d3 * l.f169e)), (l.f16d3 * l.f169f), (l.f16d3 * l.f16a0), );let t39b: f64 = (l.f16b5 + l.f16c7);(l.f16cb, l.f16cc, l.f16cd, l.f16ce, ) = (t39b, (l.f16b6 + l.f16c8), (l.f16b7 + l.f16c9), (l.f16b8 + l.f16ca), );(l.f16de, l.f16df, l.f16e0, l.f16e1, ) = (l.f16cb, l.f16cc, l.f16cd, l.f16ce, );(l.f20c6, l.f20cb, l.f20cc, l.f20cd, ) = (l.f16de, l.f16df, l.f16e0, l.f16e1, );}
        let t39c: f64 = if p.p301 == 1.0 { 1.0 } else { 0.0 };l.f2008 = t39c;
        if ((l.f2002 != 0.0) && (l.f2008 != 0.0)) {(l.f1760, l.f1761, l.f1762, l.f1763, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1753, l.f1754, ) = (0.0, 0.0, );(l.f1755, l.f1756, ) = (0.0, 0.0, );(l.f176a, l.f176b, l.f176c, ) = (l.f23c7, l.f23c8, l.f23c9, );(l.f175e, l.f175f, ) = (l.f215b, l.f215c, );l.f176d = 1.0;l.f16f1 = 10.0;l.f172d = 1.0;l.f175b = 0.0;l.f1759 = 4.0;l.f1769 = 600.0;(l.f1766, l.f1767, ) = (l.f22f2, l.f22f3, );let t39d: f64 = (1.0 - p.p311);let t39e: f64 = (p.p0 * t39d);l.f1770 = t39e;l.f1758 = p.p2;l.f1751 = 0.0;l.f1757 = 0.0;l.f176e = p.p304;l.f16f2 = p.p305;l.f1752 = p.p303;l.f175d = p.p302;l.f175a = 0.0;l.f176f = 0.0;l.f1768 = p.p6;(l.f174d, l.f174e, l.f174f, l.f1750, ) = (0.0, 0.0, 0.0, 0.0, );(l.f16ef, l.f16f0, ) = (0.0, 0.0, );(l.f1764, l.f1765, ) = (0.0, 0.0, );(l.f1729, l.f172a, l.f172b, l.f172c, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1731, l.f1732, l.f1733, l.f1734, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1737, l.f1738, l.f1739, l.f173a, ) = (0.0, 0.0, 0.0, 0.0, );(l.f172e, l.f172f, l.f1730, ) = (0.0, 0.0, 0.0, );(l.f1749, l.f174a, l.f174b, l.f174c, ) = (0.0, 0.0, 0.0, 0.0, );(l.f16fb, l.f16fc, l.f16fd, l.f16fe, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1701, l.f1702, ) = (0.0, 0.0, );(l.f16f3, l.f16f4, l.f16f5, l.f16f6, ) = (0.0, 0.0, 0.0, 0.0, );(l.f16f9, l.f16fa, ) = (0.0, 0.0, );(l.f1727, l.f1728, ) = (0.0, 0.0, );(l.f1703, l.f1704, l.f1705, l.f1706, ) = (0.0, 0.0, 0.0, 0.0, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_140(
        l: &mut StampLocals,
    ) {
        if ((l.f2002 != 0.0) && (l.f2008 != 0.0)) {(l.f1713, l.f1714, l.f1715, l.f1716, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1707, l.f1708, l.f1709, l.f170a, ) = (0.0, 0.0, 0.0, 0.0, );(l.f1723, l.f1724, l.f1725, l.f1726, ) = (0.0, 0.0, 0.0, 0.0, );(l.f171f, l.f1720, l.f1721, l.f1722, ) = (0.0, 0.0, 0.0, 0.0, );l.f175c = 0.0;(l.f16ff, l.f1700, ) = (0.0, 0.0, );(l.f16f7, l.f16f8, ) = (0.0, 0.0, );(l.f1735, l.f1736, ) = (0.0, 0.0, );(l.f171d, l.f171e, ) = (0.0, 0.0, );(l.f1711, l.f1712, ) = (0.0, 0.0, );(l.f1747, l.f1748, ) = (0.0, 0.0, );(l.f1743, l.f1744, l.f1745, l.f1746, ) = (0.0, 0.0, 0.0, 0.0, );(l.f171b, l.f171c, ) = (0.0, 0.0, );(l.f170f, l.f1710, ) = (0.0, 0.0, );(l.f1741, l.f1742, ) = (0.0, 0.0, );(l.f1717, l.f1718, l.f1719, l.f171a, ) = (0.0, 0.0, 0.0, 0.0, );(l.f170b, l.f170c, l.f170d, l.f170e, ) = (0.0, 0.0, 0.0, 0.0, );(l.f173f, l.f1740, ) = (0.0, 0.0, );(l.f173b, l.f173c, l.f173d, l.f173e, ) = (0.0, 0.0, 0.0, 0.0, );let t39f: f64 = (l.f175a / l.f175e);let t3a0: f64 = (-l.f176f);let t3a1: f64 = (t39f * t3a0);(l.f1727, l.f1728, ) = (t3a1, ((-((l.f175a * l.f175f) / (l.f175e * l.f175e))) * t3a0), );}
        if ((l.f2002 != 0.0) && (l.f2008 != 0.0)) {
            let t3a2: f64 = (-50.0);
            let (t3af, t3b0,) = {
    if ((!(l.f1727 > 50.0)) && (!(l.f1727 < t3a2))) {
        let t3a3: f64 = (l.f1727).exp();
        (t3a3, (t3a3 * l.f1728),)
    } else {
        let t3a4: f64 = (-50.0);
        let (t3ad, t3ae,) = {
            if ((!(l.f1727 > 50.0)) && (l.f1727 < t3a4)) {
                let t3a5: f64 = (-50.0);let t3a6: f64 = (t3a5).exp();
                (t3a6, 0.0,)
            } else {
                let (t3ab, t3ac,) = {
                    if (l.f1727 > 50.0) {
                        let t3a7: f64 = (50.0_f64).exp();let t3a8: f64 = (l.f1727 - 50.0);let t3a9: f64 = (1.0 + t3a8);let t3aa: f64 = (t3a7 * t3a9);
                        (t3aa, (t3a7 * l.f1728),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t3ab, t3ac,)
            }
        };
        (t3ad, t3ae,)
    }
};
            (l.f1764, l.f1765, ) = (t3af, t3b0, );
        }
        if ((l.f2002 != 0.0) && (l.f2008 != 0.0)) {let t3b1: f64 = (-l.f176a);let t3b2: f64 = (t3b1 - l.f1769);let t3b3: f64 = (l.f1759 * t3b2);let t3b4: f64 = (t3b3 + l.f1727);(l.f16fb, l.f16fc, l.f16fd, l.f16fe, ) = (t3b4, l.f1728, (l.f1759 * (-l.f176b)), (l.f1759 * (-l.f176c)), );let t3b5: f64 = (-l.f1759);let t3b6: f64 = (t3b5 * l.f1769);let t3b7: f64 = (t3b6 + l.f1727);(l.f1701, l.f1702, ) = (t3b7, l.f1728, );}
        if ((l.f2002 != 0.0) && (l.f2008 != 0.0)) {
            let t3b8: f64 = (-50.0);
            let (t3c9, t3ca, t3cb, t3cc,) = {
    if ((!(l.f16fb > 50.0)) && (!(l.f16fb < t3b8))) {
        let t3b9: f64 = (l.f16fb).exp();
        (t3b9, (t3b9 * l.f16fc), (t3b9 * l.f16fd), (t3b9 * l.f16fe),)
    } else {
        let t3ba: f64 = (-50.0);
        let (t3c5, t3c6, t3c7, t3c8,) = {
            if ((!(l.f16fb > 50.0)) && (l.f16fb < t3ba)) {
                let t3bb: f64 = (-50.0);let t3bc: f64 = (t3bb).exp();
                (t3bc, 0.0, 0.0, 0.0,)
            } else {
                let (t3c1, t3c2, t3c3, t3c4,) = {
                    if (l.f16fb > 50.0) {
                        let t3bd: f64 = (50.0_f64).exp();let t3be: f64 = (l.f16fb - 50.0);let t3bf: f64 = (1.0 + t3be);let t3c0: f64 = (t3bd * t3bf);
                        (t3c0, (t3bd * l.f16fc), (t3bd * l.f16fd), (t3bd * l.f16fe),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t3c1, t3c2, t3c3, t3c4,)
            }
        };
        (t3c5, t3c6, t3c7, t3c8,)
    }
};
            (l.f16f3, l.f16f4, l.f16f5, l.f16f6, ) = (t3c9, t3ca, t3cb, t3cc, );
        }
        if ((l.f2002 != 0.0) && (l.f2008 != 0.0)) {
            let t3cd: f64 = (-50.0);
            let (t3da, t3db,) = {
    if ((!(l.f1701 > 50.0)) && (!(l.f1701 < t3cd))) {
        let t3ce: f64 = (l.f1701).exp();
        (t3ce, (t3ce * l.f1702),)
    } else {
        let t3cf: f64 = (-50.0);
        let (t3d8, t3d9,) = {
            if ((!(l.f1701 > 50.0)) && (l.f1701 < t3cf)) {
                let t3d0: f64 = (-50.0);let t3d1: f64 = (t3d0).exp();
                (t3d1, 0.0,)
            } else {
                let (t3d6, t3d7,) = {
                    if (l.f1701 > 50.0) {
                        let t3d2: f64 = (50.0_f64).exp();let t3d3: f64 = (l.f1701 - 50.0);let t3d4: f64 = (1.0 + t3d3);let t3d5: f64 = (t3d2 * t3d4);
                        (t3d5, (t3d2 * l.f1702),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t3d6, t3d7,)
            }
        };
        (t3d8, t3d9,)
    }
};
            (l.f16f9, l.f16fa, ) = (t3da, t3db, );
        }
        if ((l.f2002 != 0.0) && (l.f2008 != 0.0)) {let t3dc: f64 = (l.f16f3 - l.f16f9);(l.f1731, l.f1732, l.f1733, l.f1734, ) = (t3dc, (l.f16f4 - l.f16fa), l.f16f5, l.f16f6, );let t3dd: f64 = (l.f1768 * l.f1770);let t3de: f64 = (t3dd * l.f1758);let t3df: f64 = (t3de * l.f1751);let t3e0: f64 = (t3df * l.f1766);(l.f1753, l.f1754, ) = (t3e0, (t3df * l.f1767), );let t3e1: f64 = (l.f175b / l.f175e);let t3e2: f64 = (t3e1 * l.f176a);let t3e3: f64 = (t3e2 + l.f1727);(l.f1713, l.f1714, l.f1715, l.f1716, ) = (t3e3, (((-((l.f175b * l.f175f) / (l.f175e * l.f175e))) * l.f176a) + l.f1728), (t3e1 * l.f176b), (t3e1 * l.f176c), );}
        if ((l.f2002 != 0.0) && (l.f2008 != 0.0)) {
            let t3e4: f64 = (-50.0);
            let (t3f5, t3f6, t3f7, t3f8,) = {
    if ((!(l.f1713 > 50.0)) && (!(l.f1713 < t3e4))) {
        let t3e5: f64 = (l.f1713).exp();
        (t3e5, (t3e5 * l.f1714), (t3e5 * l.f1715), (t3e5 * l.f1716),)
    } else {
        let t3e6: f64 = (-50.0);
        let (t3f1, t3f2, t3f3, t3f4,) = {
            if ((!(l.f1713 > 50.0)) && (l.f1713 < t3e6)) {
                let t3e7: f64 = (-50.0);let t3e8: f64 = (t3e7).exp();
                (t3e8, 0.0, 0.0, 0.0,)
            } else {
                let (t3ed, t3ee, t3ef, t3f0,) = {
                    if (l.f1713 > 50.0) {
                        let t3e9: f64 = (50.0_f64).exp();let t3ea: f64 = (l.f1713 - 50.0);let t3eb: f64 = (1.0 + t3ea);let t3ec: f64 = (t3e9 * t3eb);
                        (t3ec, (t3e9 * l.f1714), (t3e9 * l.f1715), (t3e9 * l.f1716),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t3ed, t3ee, t3ef, t3f0,)
            }
        };
        (t3f1, t3f2, t3f3, t3f4,)
    }
};
            (l.f1707, l.f1708, l.f1709, l.f170a, ) = (t3f5, t3f6, t3f7, t3f8, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_141(
        l: &mut StampLocals,
    ) {
        let t3f9: f64 = if l.f172d == 1.0 { 1.0 } else { 0.0 };l.f2009 = t3f9;
        if (((l.f2002 != 0.0) && (l.f2008 != 0.0)) && (l.f2009 != 0.0)) {let t3fa: f64 = (l.f1757 * l.f1731);let t3fb: f64 = (l.f1707 - t3fa);let t3fc: f64 = (t3fb - l.f1764);let t3fd: f64 = (l.f1753 * t3fc);(l.f1737, l.f1738, l.f1739, l.f173a, ) = (t3fd, ((l.f1754 * t3fc) + (l.f1753 * ((l.f1708 - (l.f1757 * l.f1732)) - l.f1765))), (l.f1753 * (l.f1709 - (l.f1757 * l.f1733))), (l.f1753 * (l.f170a - (l.f1757 * l.f1734))), );}
        if (((l.f2002 != 0.0) && (l.f2008 != 0.0)) && (l.f2009 == 0.0)) {let t3fe: f64 = (-l.f176d);let t3ff: f64 = (t3fe - l.f1769);let t400: f64 = (l.f1759 * t3ff);let t401: f64 = (t400 + l.f1727);(l.f16ff, l.f1700, ) = (t401, l.f1728, );}
        if (((l.f2002 != 0.0) && (l.f2008 != 0.0)) && (l.f2009 == 0.0)) {
            let t402: f64 = (-50.0);
            let (t40f, t410,) = {
    if ((!(l.f16ff > 50.0)) && (!(l.f16ff < t402))) {
        let t403: f64 = (l.f16ff).exp();
        (t403, (t403 * l.f1700),)
    } else {
        let t404: f64 = (-50.0);
        let (t40d, t40e,) = {
            if ((!(l.f16ff > 50.0)) && (l.f16ff < t404)) {
                let t405: f64 = (-50.0);let t406: f64 = (t405).exp();
                (t406, 0.0,)
            } else {
                let (t40b, t40c,) = {
                    if (l.f16ff > 50.0) {
                        let t407: f64 = (50.0_f64).exp();let t408: f64 = (l.f16ff - 50.0);let t409: f64 = (1.0 + t408);let t40a: f64 = (t407 * t409);
                        (t40a, (t407 * l.f1700),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t40b, t40c,)
            }
        };
        (t40d, t40e,)
    }
};
            (l.f16f7, l.f16f8, ) = (t40f, t410, );
        }
        if (((l.f2002 != 0.0) && (l.f2008 != 0.0)) && (l.f2009 == 0.0)) {let t411: f64 = (l.f16f7 - l.f16f9);(l.f1735, l.f1736, ) = (t411, (l.f16f8 - l.f16fa), );let t412: f64 = (l.f175b / l.f175e);let t413: f64 = (t412 * l.f176d);let t414: f64 = (t413 + l.f1727);(l.f171d, l.f171e, ) = (t414, (((-((l.f175b * l.f175f) / (l.f175e * l.f175e))) * l.f176d) + l.f1728), );}
        if (((l.f2002 != 0.0) && (l.f2008 != 0.0)) && (l.f2009 == 0.0)) {
            let t415: f64 = (-50.0);
            let (t422, t423,) = {
    if ((!(l.f171d > 50.0)) && (!(l.f171d < t415))) {
        let t416: f64 = (l.f171d).exp();
        (t416, (t416 * l.f171e),)
    } else {
        let t417: f64 = (-50.0);
        let (t420, t421,) = {
            if ((!(l.f171d > 50.0)) && (l.f171d < t417)) {
                let t418: f64 = (-50.0);let t419: f64 = (t418).exp();
                (t419, 0.0,)
            } else {
                let (t41e, t41f,) = {
                    if (l.f171d > 50.0) {
                        let t41a: f64 = (50.0_f64).exp();let t41b: f64 = (l.f171d - 50.0);let t41c: f64 = (1.0 + t41b);let t41d: f64 = (t41a * t41c);
                        (t41d, (t41a * l.f171e),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t41e, t41f,)
            }
        };
        (t420, t421,)
    }
};
            (l.f1711, l.f1712, ) = (t422, t423, );
        }
        if (((l.f2002 != 0.0) && (l.f2008 != 0.0)) && (l.f2009 == 0.0)) {let t424: f64 = (l.f1757 * l.f1735);let t425: f64 = (l.f1711 - t424);let t426: f64 = (t425 - l.f1764);(l.f1747, l.f1748, ) = (t426, ((l.f1712 - (l.f1757 * l.f1736)) - l.f1765), );let t427: f64 = (l.f1757 * l.f1731);let t428: f64 = (l.f1707 - t427);let t429: f64 = (t428 - l.f1764);let t42a: f64 = (l.f1753 * t429);(l.f1743, l.f1744, l.f1745, l.f1746, ) = (t42a, ((l.f1754 * t429) + (l.f1753 * ((l.f1708 - (l.f1757 * l.f1732)) - l.f1765))), (l.f1753 * (l.f1709 - (l.f1757 * l.f1733))), (l.f1753 * (l.f170a - (l.f1757 * l.f1734))), );}
        let t42b: f64 = if l.f172d > 0.0 { 1.0 } else { 0.0 };l.f200c = t42b;
        if ((((l.f2002 != 0.0) && (l.f2008 != 0.0)) && (l.f2009 == 0.0)) && (l.f200c != 0.0)) {let t42c: f64 = (l.f172d * l.f175b);l.f175c = t42c;let t42d: f64 = (l.f175c / l.f175e);let t42e: f64 = (t42d * l.f176d);let t42f: f64 = (t42e + l.f1727);(l.f171b, l.f171c, ) = (t42f, (((-((l.f175c * l.f175f) / (l.f175e * l.f175e))) * l.f176d) + l.f1728), );}
        if ((((l.f2002 != 0.0) && (l.f2008 != 0.0)) && (l.f2009 == 0.0)) && (l.f200c != 0.0)) {
            let t430: f64 = (-50.0);
            let (t43d, t43e,) = {
    if ((!(l.f171b > 50.0)) && (!(l.f171b < t430))) {
        let t431: f64 = (l.f171b).exp();
        (t431, (t431 * l.f171c),)
    } else {
        let t432: f64 = (-50.0);
        let (t43b, t43c,) = {
            if ((!(l.f171b > 50.0)) && (l.f171b < t432)) {
                let t433: f64 = (-50.0);let t434: f64 = (t433).exp();
                (t434, 0.0,)
            } else {
                let (t439, t43a,) = {
                    if (l.f171b > 50.0) {
                        let t435: f64 = (50.0_f64).exp();let t436: f64 = (l.f171b - 50.0);let t437: f64 = (1.0 + t436);let t438: f64 = (t435 * t437);
                        (t438, (t435 * l.f171c),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t439, t43a,)
            }
        };
        (t43b, t43c,)
    }
};
            (l.f170f, l.f1710, ) = (t43d, t43e, );
        }
        if ((((l.f2002 != 0.0) && (l.f2008 != 0.0)) && (l.f2009 == 0.0)) && (l.f200c != 0.0)) {let t43f: f64 = (l.f1757 * l.f1735);let t440: f64 = (l.f170f - t43f);let t441: f64 = (t440 - l.f1764);(l.f1741, l.f1742, ) = (t441, ((l.f1710 - (l.f1757 * l.f1736)) - l.f1765), );let t442: f64 = (l.f175c / l.f175e);let t443: f64 = (t442 * l.f176a);let t444: f64 = (t443 + l.f1727);(l.f1717, l.f1718, l.f1719, l.f171a, ) = (t444, (((-((l.f175c * l.f175f) / (l.f175e * l.f175e))) * l.f176a) + l.f1728), (t442 * l.f176b), (t442 * l.f176c), );}
        if ((((l.f2002 != 0.0) && (l.f2008 != 0.0)) && (l.f2009 == 0.0)) && (l.f200c != 0.0)) {
            let t445: f64 = (-50.0);
            let (t456, t457, t458, t459,) = {
    if ((!(l.f1717 > 50.0)) && (!(l.f1717 < t445))) {
        let t446: f64 = (l.f1717).exp();
        (t446, (t446 * l.f1718), (t446 * l.f1719), (t446 * l.f171a),)
    } else {
        let t447: f64 = (-50.0);
        let (t452, t453, t454, t455,) = {
            if ((!(l.f1717 > 50.0)) && (l.f1717 < t447)) {
                let t448: f64 = (-50.0);let t449: f64 = (t448).exp();
                (t449, 0.0, 0.0, 0.0,)
            } else {
                let (t44e, t44f, t450, t451,) = {
                    if (l.f1717 > 50.0) {
                        let t44a: f64 = (50.0_f64).exp();let t44b: f64 = (l.f1717 - 50.0);let t44c: f64 = (1.0 + t44b);let t44d: f64 = (t44a * t44c);
                        (t44d, (t44a * l.f1718), (t44a * l.f1719), (t44a * l.f171a),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t44e, t44f, t450, t451,)
            }
        };
        (t452, t453, t454, t455,)
    }
};
            (l.f170b, l.f170c, l.f170d, l.f170e, ) = (t456, t457, t458, t459, );
        }
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_142(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((((l.f2002 != 0.0) && (l.f2008 != 0.0)) && (l.f2009 == 0.0)) && (l.f200c != 0.0)) {let t45a: f64 = (l.f1753 * l.f1747);let t45b: f64 = (t45a / l.f1741);(l.f173f, l.f1740, ) = (t45b, (((((l.f1754 * l.f1747) + (l.f1753 * l.f1748)) * l.f1741) - (t45a * l.f1742)) / (l.f1741 * l.f1741)), );let t45c: f64 = (l.f1757 * l.f1731);let t45d: f64 = (l.f170b - t45c);let t45e: f64 = (t45d - l.f1764);let t45f: f64 = (l.f173f * t45e);(l.f173b, l.f173c, l.f173d, l.f173e, ) = (t45f, ((l.f1740 * t45e) + (l.f173f * ((l.f170c - (l.f1757 * l.f1732)) - l.f1765))), (l.f173f * (l.f170d - (l.f1757 * l.f1733))), (l.f173f * (l.f170e - (l.f1757 * l.f1734))), );}
        if ((((l.f2002 != 0.0) && (l.f2008 != 0.0)) && (l.f2009 == 0.0)) && (l.f200c == 0.0)) {let t460: f64 = (l.f1753 * l.f1747);(l.f173b, l.f173c, l.f173d, l.f173e, ) = (t460, ((l.f1754 * l.f1747) + (l.f1753 * l.f1748)), 0.0, 0.0, );}
        if (((l.f2002 != 0.0) && (l.f2008 != 0.0)) && (l.f2009 == 0.0)) {let t461: f64 = (l.f16f1 * l.f16f1);let t462: f64 = (t461 * l.f175e);(l.f16ef, l.f16f0, ) = (t462, (t461 * l.f175f), );let t463: f64 = (l.f16ef / 2.0);let t464: f64 = (l.f176d - t463);let t465: f64 = (l.f176a - t464);let t466: f64 = (t465 / l.f16ef);(l.f1703, l.f1704, l.f1705, l.f1706, ) = (t466, ((((-(-(l.f16f0 / 2.0))) * l.f16ef) - (t465 * l.f16f0)) / (l.f16ef * l.f16ef)), (l.f176b / l.f16ef), (l.f176c / l.f16ef), );}
        let t467: f64 = if l.f1703 > 50.0 { 1.0 } else { 0.0 };l.f200d = t467;
        if ((((l.f2002 != 0.0) && (l.f2008 != 0.0)) && (l.f2009 == 0.0)) && (l.f200d != 0.0)) {(l.f1729, l.f172a, l.f172b, l.f172c, ) = (0.0, 0.0, 0.0, 0.0, );}
        let t468: f64 = (-50.0);let t469: f64 = if l.f1703 < t468 { 1.0 } else { 0.0 };l.f200e = t469;
        if (((((l.f2002 != 0.0) && (l.f2008 != 0.0)) && (l.f2009 == 0.0)) && (l.f200d == 0.0)) && (l.f200e != 0.0)) {(l.f1729, l.f172a, l.f172b, l.f172c, ) = (1.0, 0.0, 0.0, 0.0, );}
        if (((((l.f2002 != 0.0) && (l.f2008 != 0.0)) && (l.f2009 == 0.0)) && (l.f200d == 0.0)) && (l.f200e == 0.0)) {let t46a: f64 = (l.f1703).exp();let t46b: f64 = (1.0 + t46a);let t46c: f64 = (1.0 / t46b);(l.f1729, l.f172a, l.f172b, l.f172c, ) = (t46c, (-((t46a * l.f1704) / (t46b * t46b))), (-((t46a * l.f1705) / (t46b * t46b))), (-((t46a * l.f1706) / (t46b * t46b))), );}
        if (((l.f2002 != 0.0) && (l.f2008 != 0.0)) && (l.f2009 == 0.0)) {let t46d: f64 = (l.f1729 * l.f1743);let t46e: f64 = (1.0 - l.f1729);let t46f: f64 = (t46e * l.f173b);let t470: f64 = (t46d + t46f);(l.f1737, l.f1738, l.f1739, l.f173a, ) = (t470, (((l.f172a * l.f1743) + (l.f1729 * l.f1744)) + (((-l.f172a) * l.f173b) + (t46e * l.f173c))), (((l.f172b * l.f1743) + (l.f1729 * l.f1745)) + (((-l.f172b) * l.f173b) + (t46e * l.f173d))), (((l.f172c * l.f1743) + (l.f1729 * l.f1746)) + (((-l.f172c) * l.f173b) + (t46e * l.f173e))), );}
        if ((l.f2002 != 0.0) && (l.f2008 != 0.0)) {
            let t471: f64 = (-l.f176a);
            let (t480, t481, t482,) = {
    if (p.p52 != 0.0) {
        let t472: f64 = (l.f176a / l.f176e);let t473: f64 = (0.001 / p.p53);let t474: f64 = (l.f176a / l.f176e);let t475: f64 = (t473 * t474);let t476: f64 = (t475).tanh();let t477: f64 = (t472 * t476);
        (t477, (((l.f176b / l.f176e) * t476) + (t472 * ((t473 * (l.f176b / l.f176e)) / ((t475).cosh() * (t475).cosh())))), (((l.f176c / l.f176e) * t476) + (t472 * ((t473 * (l.f176c / l.f176e)) / ((t475).cosh() * (t475).cosh())))),)
    } else {
        let (t47d, t47e, t47f,) = {
            if (p.p52 == 0.0) {
                let __rspice_inv_cse_0: f64 = 1.0 / l.f176e;let t478: f64 = (l.f176a * __rspice_inv_cse_0);let t479: f64 = (l.f176a * __rspice_inv_cse_0);let t47a: f64 = (t478 * t479);let t47b: f64 = (t47a + p.p53);let t47c: f64 = (t47b).sqrt();
                (t47c, ((((l.f176b / l.f176e) * t479) + (t478 * (l.f176b / l.f176e))) / (2.0 * t47c)), ((((l.f176c / l.f176e) * t479) + (t478 * (l.f176c / l.f176e))) / (2.0 * t47c)),)
            } else {
                (0.0, 0.0, 0.0,)
            }
        };
        (t47d, t47e, t47f,)
    }
};
            let t483: f64 = (t480).powf(l.f16f2);let t484: f64 = (1.0 + t483);let t485: f64 = (1.0 / l.f16f2);let t486: f64 = (t484).powf(t485);let t487: f64 = (t471 / t486);(l.f172e, l.f172f, l.f1730, ) = (t487, ((((-l.f176b) * t486) - (t471 * if 0.0 == 0.0 && ((t485) as f64).is_finite() && ((t485) as f64).fract() == 0.0 { if t485 == 0.0 { 0.0 } else { (t485 * ((t484).powf(t485 - 1.0) * if 0.0 == 0.0 && ((l.f16f2) as f64).is_finite() && ((l.f16f2) as f64).fract() == 0.0 { if l.f16f2 == 0.0 { 0.0 } else { (l.f16f2 * ((t480).powf(l.f16f2 - 1.0) * t481)) } } else { (t483 * (l.f16f2 * (t481 / t480))) })) } } else { (t486 * (t485 * (if 0.0 == 0.0 && ((l.f16f2) as f64).is_finite() && ((l.f16f2) as f64).fract() == 0.0 { if l.f16f2 == 0.0 { 0.0 } else { (l.f16f2 * ((t480).powf(l.f16f2 - 1.0) * t481)) } } else { (t483 * (l.f16f2 * (t481 / t480))) } / t484))) })) / (t486 * t486)), ((((-l.f176c) * t486) - (t471 * if 0.0 == 0.0 && ((t485) as f64).is_finite() && ((t485) as f64).fract() == 0.0 { if t485 == 0.0 { 0.0 } else { (t485 * ((t484).powf(t485 - 1.0) * if 0.0 == 0.0 && ((l.f16f2) as f64).is_finite() && ((l.f16f2) as f64).fract() == 0.0 { if l.f16f2 == 0.0 { 0.0 } else { (l.f16f2 * ((t480).powf(l.f16f2 - 1.0) * t482)) } } else { (t483 * (l.f16f2 * (t482 / t480))) })) } } else { (t486 * (t485 * (if 0.0 == 0.0 && ((l.f16f2) as f64).is_finite() && ((l.f16f2) as f64).fract() == 0.0 { if l.f16f2 == 0.0 { 0.0 } else { (l.f16f2 * ((t480).powf(l.f16f2 - 1.0) * t482)) } } else { (t483 * (l.f16f2 * (t482 / t480))) } / t484))) })) / (t486 * t486)), );
        }
        if ((l.f2002 != 0.0) && (l.f2008 != 0.0)) {let t488: f64 = (-l.f1768);let t489: f64 = (t488 * l.f1770);let t48a: f64 = (t489 * l.f1758);let t48b: f64 = (t48a * l.f1752);let t48c: f64 = (t48b * l.f1766);let t48d: f64 = t48c;(l.f1755, l.f1756, ) = (t48d, (t48b * l.f1767), );let t48e: f64 = (l.f175d / l.f175e);let t48f: f64 = (t48e * l.f172e);(l.f1723, l.f1724, l.f1725, l.f1726, ) = (t48f, ((-((l.f175d * l.f175f) / (l.f175e * l.f175e))) * l.f172e), (t48e * l.f172f), (t48e * l.f1730), );}
        if ((l.f2002 != 0.0) && (l.f2008 != 0.0)) {
            let t490: f64 = (-50.0);
            let (t4a1, t4a2, t4a3, t4a4,) = {
    if ((!(l.f1723 > 50.0)) && (!(l.f1723 < t490))) {
        let t491: f64 = (l.f1723).exp();
        (t491, (t491 * l.f1724), (t491 * l.f1725), (t491 * l.f1726),)
    } else {
        let t492: f64 = (-50.0);
        let (t49d, t49e, t49f, t4a0,) = {
            if ((!(l.f1723 > 50.0)) && (l.f1723 < t492)) {
                let t493: f64 = (-50.0);let t494: f64 = (t493).exp();
                (t494, 0.0, 0.0, 0.0,)
            } else {
                let (t499, t49a, t49b, t49c,) = {
                    if (l.f1723 > 50.0) {
                        let t495: f64 = (50.0_f64).exp();let t496: f64 = (l.f1723 - 50.0);let t497: f64 = (1.0 + t496);let t498: f64 = (t495 * t497);
                        (t498, (t495 * l.f1724), (t495 * l.f1725), (t495 * l.f1726),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t499, t49a, t49b, t49c,)
            }
        };
        (t49d, t49e, t49f, t4a0,)
    }
};
            (l.f171f, l.f1720, l.f1721, l.f1722, ) = (t4a1, t4a2, t4a3, t4a4, );
        }
        if ((l.f2002 != 0.0) && (l.f2008 != 0.0)) {let t4a5: f64 = (l.f171f - 1.0);let t4a6: f64 = (l.f1755 * t4a5);(l.f1749, l.f174a, l.f174b, l.f174c, ) = (t4a6, ((l.f1756 * t4a5) + (l.f1755 * l.f1720)), (l.f1755 * l.f1721), (l.f1755 * l.f1722), );let t4a7: f64 = (l.f1737 + l.f1749);(l.f174d, l.f174e, l.f174f, l.f1750, ) = (t4a7, (l.f1738 + l.f174a), (l.f1739 + l.f174b), (l.f173a + l.f174c), );(l.f1760, l.f1761, l.f1762, l.f1763, ) = (l.f174d, l.f174e, l.f174f, l.f1750, );(l.f20c7, l.f20c8, l.f20c9, l.f20ca, ) = (l.f1760, l.f1761, l.f1762, l.f1763, );}
        let t4a8: f64 = (p.p308 * p.p306);let t4a9: f64 = if l.f23c7 <= t4a8 { 1.0 } else { 0.0 };l.f200f = t4a9;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_143(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.f2002 != 0.0) && (l.f200f != 0.0)) {let t4aa: f64 = (p.p6 * 2.0);let t4ab: f64 = (t4aa * p.p307);let t4ac: f64 = (t4ab * p.p0);let t4ad: f64 = (1.0 - p.p311);let t4ae: f64 = (t4ac * t4ad);let t4af: f64 = (t4ae * p.p2);let t4b0: f64 = (t4af * p.p306);let t4b1: f64 = (l.f23c7 / p.p306);let t4b2: f64 = (1.0 - t4b1);let t4b3: f64 = (t4b2).sqrt();let t4b4: f64 = (1.0 - t4b3);let t4b5: f64 = (t4b0 * t4b4);(l.f2280, l.f22a1, l.f22a2, ) = (t4b5, (t4b0 * (-((-(l.f23c8 / p.p306)) / (2.0 * t4b3)))), (t4b0 * (-((-(l.f23c9 / p.p306)) / (2.0 * t4b3)))), );}
        if ((l.f2002 != 0.0) && (l.f200f == 0.0)) {let t4b6: f64 = (1.0 - p.p308);let t4b7: f64 = (t4b6).sqrt();let t4b8: f64 = (1.0 - t4b7);l.f2281 = t4b8;}
        let t4b9: f64 = if p.p309 >= 1.0 { 1.0 } else { 0.0 };l.f2011 = t4b9;
        if (((l.f2002 != 0.0) && (l.f200f == 0.0)) && (l.f2011 != 0.0)) {let t4ba: f64 = (2.0 * p.p306);let t4bb: f64 = (1.0 - p.p308);let t4bc: f64 = (t4bb).sqrt();let t4bd: f64 = (t4ba * t4bc);let t4be: f64 = (1.0 / t4bd);l.f2287 = t4be;let t4bf: f64 = (p.p308 * p.p306);let t4c0: f64 = (l.f23c7 - t4bf);(l.f23cb, l.f23cc, l.f23cd, ) = (t4c0, l.f23c8, l.f23c9, );let t4c1: f64 = (l.f2287 * l.f23cb);(l.f2283, l.f2284, l.f2285, ) = (t4c1, (l.f2287 * l.f23cc), (l.f2287 * l.f23cd), );}
        let t4c2: f64 = if p.p309 >= 2.0 { 1.0 } else { 0.0 };l.f2013 = t4c2;
        if ((((l.f2002 != 0.0) && (l.f200f == 0.0)) && (l.f2011 != 0.0)) && (l.f2013 != 0.0)) {let t4c3: f64 = (4.0 * p.p306);let t4c4: f64 = (1.0 - p.p308);let t4c5: f64 = (t4c3 * t4c4);let t4c6: f64 = (l.f2287 / t4c5);l.f228d = t4c6;let t4c7: f64 = (l.f23cb * l.f23cb);(l.f23cf, l.f23d0, l.f23d1, ) = (t4c7, ((l.f23cc * l.f23cb) + (l.f23cb * l.f23cc)), ((l.f23cd * l.f23cb) + (l.f23cb * l.f23cd)), );let t4c8: f64 = (l.f228d * l.f23cf);(l.f2289, l.f228a, l.f228b, ) = (t4c8, (l.f228d * l.f23d0), (l.f228d * l.f23d1), );}
        let t4c9: f64 = if p.p309 >= 3.0 { 1.0 } else { 0.0 };l.f2015 = t4c9;
        if (((((l.f2002 != 0.0) && (l.f200f == 0.0)) && (l.f2011 != 0.0)) && (l.f2013 != 0.0)) && (l.f2015 != 0.0)) {let t4ca: f64 = (2.0 * p.p306);let t4cb: f64 = (1.0 - p.p308);let t4cc: f64 = (t4ca * t4cb);let t4cd: f64 = (l.f228d / t4cc);l.f2293 = t4cd;let t4ce: f64 = (l.f23cf * l.f23cb);(l.f23d3, l.f23d4, l.f23d5, ) = (t4ce, ((l.f23d0 * l.f23cb) + (l.f23cf * l.f23cc)), ((l.f23d1 * l.f23cb) + (l.f23cf * l.f23cd)), );let t4cf: f64 = (l.f2293 * l.f23d3);(l.f228f, l.f2290, l.f2291, ) = (t4cf, (l.f2293 * l.f23d4), (l.f2293 * l.f23d5), );}
        let t4d0: f64 = if p.p309 >= 4.0 { 1.0 } else { 0.0 };l.f2017 = t4d0;
        if ((((((l.f2002 != 0.0) && (l.f200f == 0.0)) && (l.f2011 != 0.0)) && (l.f2013 != 0.0)) && (l.f2015 != 0.0)) && (l.f2017 != 0.0)) {let t4d1: f64 = (5.0 * l.f2293);let t4d2: f64 = (8.0 * p.p306);let t4d3: f64 = (1.0 - p.p308);let t4d4: f64 = (t4d2 * t4d3);let t4d5: f64 = (t4d1 / t4d4);l.f2299 = t4d5;let t4d6: f64 = (l.f23d3 * l.f23cb);(l.f23d7, l.f23d8, l.f23d9, ) = (t4d6, ((l.f23d4 * l.f23cb) + (l.f23d3 * l.f23cc)), ((l.f23d5 * l.f23cb) + (l.f23d3 * l.f23cd)), );let t4d7: f64 = (l.f2299 * l.f23d7);(l.f2295, l.f2296, l.f2297, ) = (t4d7, (l.f2299 * l.f23d8), (l.f2299 * l.f23d9), );}
        let t4d8: f64 = if p.p309 >= 5.0 { 1.0 } else { 0.0 };l.f2019 = t4d8;
        if (((((((l.f2002 != 0.0) && (l.f200f == 0.0)) && (l.f2011 != 0.0)) && (l.f2013 != 0.0)) && (l.f2015 != 0.0)) && (l.f2017 != 0.0)) && (l.f2019 != 0.0)) {let t4d9: f64 = (7.0 * l.f2299);let t4da: f64 = (10.0 * p.p306);let t4db: f64 = (1.0 - p.p308);let t4dc: f64 = (t4da * t4db);let t4dd: f64 = (t4d9 / t4dc);l.f229f = t4dd;let t4de: f64 = (l.f23d7 * l.f23cb);(l.f23db, l.f23dc, l.f23dd, ) = (t4de, ((l.f23d8 * l.f23cb) + (l.f23d7 * l.f23cc)), ((l.f23d9 * l.f23cb) + (l.f23d7 * l.f23cd)), );let t4df: f64 = (l.f229f * l.f23db);(l.f229b, l.f229c, l.f229d, ) = (t4df, (l.f229f * l.f23dc), (l.f229f * l.f23dd), );}
        if (((((((l.f2002 != 0.0) && (l.f200f == 0.0)) && (l.f2011 != 0.0)) && (l.f2013 != 0.0)) && (l.f2015 != 0.0)) && (l.f2017 != 0.0)) && (l.f2019 == 0.0)) {l.f229f = 0.0;}
        if ((((((l.f2002 != 0.0) && (l.f200f == 0.0)) && (l.f2011 != 0.0)) && (l.f2013 != 0.0)) && (l.f2015 != 0.0)) && (l.f2017 == 0.0)) {l.f2299 = 0.0;}
        if (((((l.f2002 != 0.0) && (l.f200f == 0.0)) && (l.f2011 != 0.0)) && (l.f2013 != 0.0)) && (l.f2015 == 0.0)) {l.f2293 = 0.0;}
        if ((((l.f2002 != 0.0) && (l.f200f == 0.0)) && (l.f2011 != 0.0)) && (l.f2013 == 0.0)) {l.f228d = 0.0;}
        if (((l.f2002 != 0.0) && (l.f200f == 0.0)) && (l.f2011 == 0.0)) {l.f2287 = 0.0;}
        if ((l.f2002 != 0.0) && (l.f200f == 0.0)) {let t4e0: f64 = (p.p6 * 2.0);let t4e1: f64 = (t4e0 * p.p307);let t4e2: f64 = (t4e1 * p.p0);let t4e3: f64 = (1.0 - p.p311);let t4e4: f64 = (t4e2 * t4e3);let t4e5: f64 = (t4e4 * p.p2);let t4e6: f64 = (t4e5 * p.p306);let t4e7: f64 = (l.f2281 + l.f2283);let t4e8: f64 = (t4e7 + l.f2289);let t4e9: f64 = (t4e8 + l.f228f);let t4ea: f64 = (t4e9 + l.f2295);let t4eb: f64 = (t4ea + l.f229b);let t4ec: f64 = (t4e6 * t4eb);(l.f2280, l.f22a1, l.f22a2, ) = (t4ec, (t4e6 * ((((l.f2284 + l.f228a) + l.f2290) + l.f2296) + l.f229c)), (t4e6 * ((((l.f2285 + l.f228b) + l.f2291) + l.f2297) + l.f229d)), );}
    }
}

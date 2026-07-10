#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_144(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv2 = ctx.node_voltage(nodes[2]);let nv8 = ctx.node_voltage(nodes[8]);let nv18 = ctx.node_voltage(nodes[18]);let nv19 = ctx.node_voltage(nodes[19]);let t66: f64 = if ((p.p310 != 0.0) && (p.p311 != 0.0)) { 1.0 } else { 0.0 };l.f201b = t66;
        if ((l.f2002 != 0.0) && (l.f201b != 0.0)) {let t67: f64 = (p.p0 * p.p311);let t68: f64 = (t67 * p.p2);let t69: f64 = (p.p310 / t68);l.f22ea = t69;}
        (l.f212c, l.f212d, l.f2130, l.f2131, l.f2132, l.f212e, l.f212f, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f2115, l.f2116, l.f2119, l.f211a, l.f211b, l.f2117, l.f2118, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );let t6a: f64 = ((nv19 - nv18) + (nv19 - nv8));let t6b: f64 = (p.p6 * t6a);(l.f23bd, l.f23be, l.f23c1, l.f23c2, l.f23bf, l.f23c0, ) = (t6b, 0.0, 0.0, (-p.p6), (-p.p6), (p.p6 * (1.0 + 1.0)), );let t6c: f64 = ((nv18 - nv19) + (nv18 - nv8));let t6d: f64 = (p.p6 * t6c);(l.f23b7, l.f23b8, l.f23bb, l.f23bc, l.f23b9, l.f23ba, ) = (t6d, 0.0, 0.0, (-p.p6), (p.p6 * (1.0 + 1.0)), (-p.p6), );let t6e: f64 = if p.p312 == 1.0 { 1.0 } else { 0.0 };l.f201e = t6e;let t6f: f64 = if p.p313 == 0.0 { 1.0 } else { 0.0 };l.f201f = t6f;
        if ((l.f201e != 0.0) && (l.f201f != 0.0)) {let t70: f64 = ((nv2 - nv0) + (nv2 - nv8));let t71: f64 = (p.p6 * t70);(l.f23bd, l.f23be, l.f23c1, l.f23c2, l.f23bf, l.f23c0, ) = (t71, (-p.p6), (p.p6 * (1.0 + 1.0)), (-p.p6), 0.0, 0.0, );let t72: f64 = ((nv0 - nv2) + (nv0 - nv8));let t73: f64 = (p.p6 * t72);(l.f23b7, l.f23b8, l.f23bb, l.f23bc, l.f23b9, l.f23ba, ) = (t73, (p.p6 * (1.0 + 1.0)), (-p.p6), (-p.p6), 0.0, 0.0, );}
        if (l.f201e != 0.0) {(l.f1815, l.f1816, l.f1819, l.f181a, l.f181b, l.f1817, l.f1818, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1808, l.f1809, ) = (0.0, 0.0, );(l.f180a, l.f180b, ) = (0.0, 0.0, );(l.f1822, l.f1823, l.f1826, l.f1827, l.f1824, l.f1825, ) = (l.f23bd, l.f23be, l.f23c1, l.f23c2, l.f23bf, l.f23c0, );(l.f1813, l.f1814, ) = (l.f215b, l.f215c, );l.f1828 = p.p260;l.f1773 = p.p262;l.f17cd = p.p261;l.f1810 = 0.0;l.f180e = p.p317;l.f1821 = p.p316;(l.f181e, l.f181f, ) = (l.f22f2, l.f22f3, );l.f182b = p.p0;l.f180d = p.p2;l.f1806 = p.p314;l.f180c = 1.0;l.f1829 = p.p270;l.f1774 = p.p271;l.f1807 = 0.0;l.f1812 = p.p268;l.f180f = 0.0;l.f182a = p.p256;l.f1820 = p.p6;(l.f17ff, l.f1800, l.f1803, l.f1804, l.f1805, l.f1801, l.f1802, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1771, l.f1772, ) = (0.0, 0.0, );(l.f181c, l.f181d, ) = (0.0, 0.0, );(l.f17c6, l.f17c7, l.f17ca, l.f17cb, l.f17cc, l.f17c8, l.f17c9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f17d4, l.f17d5, l.f17d8, l.f17d9, l.f17da, l.f17d6, l.f17d7, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f17dd, l.f17de, l.f17e1, l.f17e2, l.f17e3, l.f17df, l.f17e0, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f17ce, l.f17cf, l.f17d2, l.f17d3, l.f17d0, l.f17d1, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f17f8, l.f17f9, l.f17fc, l.f17fd, l.f17fe, l.f17fa, l.f17fb, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1780, l.f1781, l.f1784, l.f1785, l.f1786, l.f1782, l.f1783, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1789, l.f178a, ) = (0.0, 0.0, );(l.f1775, l.f1776, l.f1779, l.f177a, l.f177b, l.f1777, l.f1778, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f177e, l.f177f, ) = (0.0, 0.0, );(l.f17c4, l.f17c5, ) = (0.0, 0.0, );(l.f178b, l.f178c, l.f178f, l.f1790, l.f1791, l.f178d, l.f178e, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f17a4, l.f17a5, l.f17a8, l.f17a9, l.f17aa, l.f17a6, l.f17a7, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1792, l.f1793, l.f1796, l.f1797, l.f1798, l.f1794, l.f1795, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f17bd, l.f17be, l.f17c1, l.f17c2, l.f17c3, l.f17bf, l.f17c0, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f17b6, l.f17b7, l.f17ba, l.f17bb, l.f17bc, l.f17b8, l.f17b9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.f1811 = 0.0;(l.f1787, l.f1788, ) = (0.0, 0.0, );(l.f177c, l.f177d, ) = (0.0, 0.0, );(l.f17db, l.f17dc, ) = (0.0, 0.0, );(l.f17b4, l.f17b5, ) = (0.0, 0.0, );(l.f17a2, l.f17a3, ) = (0.0, 0.0, );(l.f17f6, l.f17f7, ) = (0.0, 0.0, );(l.f17ef, l.f17f0, l.f17f3, l.f17f4, l.f17f5, l.f17f1, l.f17f2, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f17b2, l.f17b3, ) = (0.0, 0.0, );(l.f17a0, l.f17a1, ) = (0.0, 0.0, );(l.f17ed, l.f17ee, ) = (0.0, 0.0, );(l.f17ab, l.f17ac, l.f17af, l.f17b0, l.f17b1, l.f17ad, l.f17ae, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1799, l.f179a, l.f179d, l.f179e, l.f179f, l.f179b, l.f179c, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f17eb, l.f17ec, ) = (0.0, 0.0, );(l.f17e4, l.f17e5, l.f17e8, l.f17e9, l.f17ea, l.f17e6, l.f17e7, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );let t7d: f64 = (l.f180f / l.f1813);let t7e: f64 = (-l.f182a);let t7f: f64 = (t7d * t7e);(l.f17c4, l.f17c5, ) = (t7f, ((-((l.f180f * l.f1814) / (l.f1813 * l.f1813))) * t7e), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_145(
        l: &mut StampLocals,
    ) {
        if (l.f201e != 0.0) {
            let t80: f64 = (-50.0);
            let (t8d, t8e,) = {
    if ((!(l.f17c4 > 50.0)) && (!(l.f17c4 < t80))) {
        let t81: f64 = (l.f17c4).exp();
        (t81, (t81 * l.f17c5),)
    } else {
        let t82: f64 = (-50.0);
        let (t8b, t8c,) = {
            if ((!(l.f17c4 > 50.0)) && (l.f17c4 < t82)) {
                let t83: f64 = (-50.0);let t84: f64 = (t83).exp();
                (t84, 0.0,)
            } else {
                let (t89, t8a,) = {
                    if (l.f17c4 > 50.0) {
                        let t85: f64 = (50.0_f64).exp();let t86: f64 = (l.f17c4 - 50.0);let t87: f64 = (1.0 + t86);let t88: f64 = (t85 * t87);
                        (t88, (t85 * l.f17c5),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t89, t8a,)
            }
        };
        (t8b, t8c,)
    }
};
            (l.f181c, l.f181d, ) = (t8d, t8e, );
        }
        if (l.f201e != 0.0) {let t8f: f64 = (-l.f1822);let t90: f64 = (t8f - l.f1821);let t91: f64 = (l.f180e * t90);let t92: f64 = (t91 + l.f17c4);(l.f1780, l.f1781, l.f1784, l.f1785, l.f1786, l.f1782, l.f1783, ) = (t92, (l.f180e * (-l.f1823)), (l.f180e * (-l.f1826)), l.f17c5, (l.f180e * (-l.f1827)), (l.f180e * (-l.f1824)), (l.f180e * (-l.f1825)), );let t93: f64 = (-l.f180e);let t94: f64 = (t93 * l.f1821);let t95: f64 = (t94 + l.f17c4);(l.f1789, l.f178a, ) = (t95, l.f17c5, );}
        if (l.f201e != 0.0) {
            let t96: f64 = (-50.0);
            let (tad, tae, tb1, tb2, tb3, taf, tb0,) = {
    if ((!(l.f1780 > 50.0)) && (!(l.f1780 < t96))) {
        let t97: f64 = (l.f1780).exp();
        (t97, (t97 * l.f1781), (t97 * l.f1784), (t97 * l.f1785), (t97 * l.f1786), (t97 * l.f1782), (t97 * l.f1783),)
    } else {
        let t98: f64 = (-50.0);
        let (ta6, ta7, taa, tab, tac, ta8, ta9,) = {
            if ((!(l.f1780 > 50.0)) && (l.f1780 < t98)) {
                let t99: f64 = (-50.0);let t9a: f64 = (t99).exp();
                (t9a, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let (t9f, ta0, ta3, ta4, ta5, ta1, ta2,) = {
                    if (l.f1780 > 50.0) {
                        let t9b: f64 = (50.0_f64).exp();let t9c: f64 = (l.f1780 - 50.0);let t9d: f64 = (1.0 + t9c);let t9e: f64 = (t9b * t9d);
                        (t9e, (t9b * l.f1781), (t9b * l.f1784), (t9b * l.f1785), (t9b * l.f1786), (t9b * l.f1782), (t9b * l.f1783),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t9f, ta0, ta3, ta4, ta5, ta1, ta2,)
            }
        };
        (ta6, ta7, taa, tab, tac, ta8, ta9,)
    }
};
            (l.f1775, l.f1776, l.f1779, l.f177a, l.f177b, l.f1777, l.f1778, ) = (tad, tae, tb1, tb2, tb3, taf, tb0, );
        }
        if (l.f201e != 0.0) {
            let tb4: f64 = (-50.0);
            let (tc1, tc2,) = {
    if ((!(l.f1789 > 50.0)) && (!(l.f1789 < tb4))) {
        let tb5: f64 = (l.f1789).exp();
        (tb5, (tb5 * l.f178a),)
    } else {
        let tb6: f64 = (-50.0);
        let (tbf, tc0,) = {
            if ((!(l.f1789 > 50.0)) && (l.f1789 < tb6)) {
                let tb7: f64 = (-50.0);let tb8: f64 = (tb7).exp();
                (tb8, 0.0,)
            } else {
                let (tbd, tbe,) = {
                    if (l.f1789 > 50.0) {
                        let tb9: f64 = (50.0_f64).exp();let tba: f64 = (l.f1789 - 50.0);let tbb: f64 = (1.0 + tba);let tbc: f64 = (tb9 * tbb);
                        (tbc, (tb9 * l.f178a),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (tbd, tbe,)
            }
        };
        (tbf, tc0,)
    }
};
            (l.f177e, l.f177f, ) = (tc1, tc2, );
        }
        if (l.f201e != 0.0) {let tc3: f64 = (l.f1775 - l.f177e);(l.f17d4, l.f17d5, l.f17d8, l.f17d9, l.f17da, l.f17d6, l.f17d7, ) = (tc3, l.f1776, l.f1779, (l.f177a - l.f177f), l.f177b, l.f1777, l.f1778, );let tc4: f64 = (l.f1820 * l.f182b);let tc5: f64 = (tc4 * l.f180d);let tc6: f64 = (tc5 * l.f1806);let tc7: f64 = (tc6 * l.f181e);(l.f1808, l.f1809, ) = (tc7, (tc6 * l.f181f), );let tc8: f64 = (l.f1810 / l.f1813);let tc9: f64 = (tc8 * l.f1822);let tca: f64 = (tc9 + l.f17c4);(l.f17a4, l.f17a5, l.f17a8, l.f17a9, l.f17aa, l.f17a6, l.f17a7, ) = (tca, (tc8 * l.f1823), (tc8 * l.f1826), (((-((l.f1810 * l.f1814) / (l.f1813 * l.f1813))) * l.f1822) + l.f17c5), (tc8 * l.f1827), (tc8 * l.f1824), (tc8 * l.f1825), );}
        if (l.f201e != 0.0) {
            let tcb: f64 = (-50.0);
            let (te2, te3, te6, te7, te8, te4, te5,) = {
    if ((!(l.f17a4 > 50.0)) && (!(l.f17a4 < tcb))) {
        let tcc: f64 = (l.f17a4).exp();
        (tcc, (tcc * l.f17a5), (tcc * l.f17a8), (tcc * l.f17a9), (tcc * l.f17aa), (tcc * l.f17a6), (tcc * l.f17a7),)
    } else {
        let tcd: f64 = (-50.0);
        let (tdb, tdc, tdf, te0, te1, tdd, tde,) = {
            if ((!(l.f17a4 > 50.0)) && (l.f17a4 < tcd)) {
                let tce: f64 = (-50.0);let tcf: f64 = (tce).exp();
                (tcf, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let (td4, td5, td8, td9, tda, td6, td7,) = {
                    if (l.f17a4 > 50.0) {
                        let td0: f64 = (50.0_f64).exp();let td1: f64 = (l.f17a4 - 50.0);let td2: f64 = (1.0 + td1);let td3: f64 = (td0 * td2);
                        (td3, (td0 * l.f17a5), (td0 * l.f17a8), (td0 * l.f17a9), (td0 * l.f17aa), (td0 * l.f17a6), (td0 * l.f17a7),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (td4, td5, td8, td9, tda, td6, td7,)
            }
        };
        (tdb, tdc, tdf, te0, te1, tdd, tde,)
    }
};
            (l.f1792, l.f1793, l.f1796, l.f1797, l.f1798, l.f1794, l.f1795, ) = (te2, te3, te6, te7, te8, te4, te5, );
        }
        let te9: f64 = if l.f17cd == 1.0 { 1.0 } else { 0.0 };l.f2020 = te9;
        if ((l.f201e != 0.0) && (l.f2020 != 0.0)) {let tea: f64 = (l.f180c * l.f17d4);let teb: f64 = (l.f1792 - tea);let tec: f64 = (teb - l.f181c);let ted: f64 = (l.f1808 * tec);(l.f17dd, l.f17de, l.f17e1, l.f17e2, l.f17e3, l.f17df, l.f17e0, ) = (ted, (l.f1808 * (l.f1793 - (l.f180c * l.f17d5))), (l.f1808 * (l.f1796 - (l.f180c * l.f17d8))), ((l.f1809 * tec) + (l.f1808 * ((l.f1797 - (l.f180c * l.f17d9)) - l.f181d))), (l.f1808 * (l.f1798 - (l.f180c * l.f17da))), (l.f1808 * (l.f1794 - (l.f180c * l.f17d6))), (l.f1808 * (l.f1795 - (l.f180c * l.f17d7))), );}
        if ((l.f201e != 0.0) && (l.f2020 == 0.0)) {let tee: f64 = (-l.f1828);let tef: f64 = (tee - l.f1821);let tf0: f64 = (l.f180e * tef);let tf1: f64 = (tf0 + l.f17c4);(l.f1787, l.f1788, ) = (tf1, l.f17c5, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_146(
        l: &mut StampLocals,
    ) {
        if ((l.f201e != 0.0) && (l.f2020 == 0.0)) {
            let tf2: f64 = (-50.0);
            let (tff, t100,) = {
    if ((!(l.f1787 > 50.0)) && (!(l.f1787 < tf2))) {
        let tf3: f64 = (l.f1787).exp();
        (tf3, (tf3 * l.f1788),)
    } else {
        let tf4: f64 = (-50.0);
        let (tfd, tfe,) = {
            if ((!(l.f1787 > 50.0)) && (l.f1787 < tf4)) {
                let tf5: f64 = (-50.0);let tf6: f64 = (tf5).exp();
                (tf6, 0.0,)
            } else {
                let (tfb, tfc,) = {
                    if (l.f1787 > 50.0) {
                        let tf7: f64 = (50.0_f64).exp();let tf8: f64 = (l.f1787 - 50.0);let tf9: f64 = (1.0 + tf8);let tfa: f64 = (tf7 * tf9);
                        (tfa, (tf7 * l.f1788),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (tfb, tfc,)
            }
        };
        (tfd, tfe,)
    }
};
            (l.f177c, l.f177d, ) = (tff, t100, );
        }
        if ((l.f201e != 0.0) && (l.f2020 == 0.0)) {let t101: f64 = (l.f177c - l.f177e);(l.f17db, l.f17dc, ) = (t101, (l.f177d - l.f177f), );let t102: f64 = (l.f1810 / l.f1813);let t103: f64 = (t102 * l.f1828);let t104: f64 = (t103 + l.f17c4);(l.f17b4, l.f17b5, ) = (t104, (((-((l.f1810 * l.f1814) / (l.f1813 * l.f1813))) * l.f1828) + l.f17c5), );}
        if ((l.f201e != 0.0) && (l.f2020 == 0.0)) {
            let t105: f64 = (-50.0);
            let (t112, t113,) = {
    if ((!(l.f17b4 > 50.0)) && (!(l.f17b4 < t105))) {
        let t106: f64 = (l.f17b4).exp();
        (t106, (t106 * l.f17b5),)
    } else {
        let t107: f64 = (-50.0);
        let (t110, t111,) = {
            if ((!(l.f17b4 > 50.0)) && (l.f17b4 < t107)) {
                let t108: f64 = (-50.0);let t109: f64 = (t108).exp();
                (t109, 0.0,)
            } else {
                let (t10e, t10f,) = {
                    if (l.f17b4 > 50.0) {
                        let t10a: f64 = (50.0_f64).exp();let t10b: f64 = (l.f17b4 - 50.0);let t10c: f64 = (1.0 + t10b);let t10d: f64 = (t10a * t10c);
                        (t10d, (t10a * l.f17b5),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t10e, t10f,)
            }
        };
        (t110, t111,)
    }
};
            (l.f17a2, l.f17a3, ) = (t112, t113, );
        }
        if ((l.f201e != 0.0) && (l.f2020 == 0.0)) {let t114: f64 = (l.f180c * l.f17db);let t115: f64 = (l.f17a2 - t114);let t116: f64 = (t115 - l.f181c);(l.f17f6, l.f17f7, ) = (t116, ((l.f17a3 - (l.f180c * l.f17dc)) - l.f181d), );let t117: f64 = (l.f180c * l.f17d4);let t118: f64 = (l.f1792 - t117);let t119: f64 = (t118 - l.f181c);let t11a: f64 = (l.f1808 * t119);(l.f17ef, l.f17f0, l.f17f3, l.f17f4, l.f17f5, l.f17f1, l.f17f2, ) = (t11a, (l.f1808 * (l.f1793 - (l.f180c * l.f17d5))), (l.f1808 * (l.f1796 - (l.f180c * l.f17d8))), ((l.f1809 * t119) + (l.f1808 * ((l.f1797 - (l.f180c * l.f17d9)) - l.f181d))), (l.f1808 * (l.f1798 - (l.f180c * l.f17da))), (l.f1808 * (l.f1794 - (l.f180c * l.f17d6))), (l.f1808 * (l.f1795 - (l.f180c * l.f17d7))), );}
        let t11b: f64 = if l.f17cd > 0.0 { 1.0 } else { 0.0 };l.f2021 = t11b;
        if (((l.f201e != 0.0) && (l.f2020 == 0.0)) && (l.f2021 != 0.0)) {let t11c: f64 = (l.f17cd * l.f1810);l.f1811 = t11c;let t11d: f64 = (l.f1811 / l.f1813);let t11e: f64 = (t11d * l.f1828);let t11f: f64 = (t11e + l.f17c4);(l.f17b2, l.f17b3, ) = (t11f, (((-((l.f1811 * l.f1814) / (l.f1813 * l.f1813))) * l.f1828) + l.f17c5), );}
        if (((l.f201e != 0.0) && (l.f2020 == 0.0)) && (l.f2021 != 0.0)) {
            let t120: f64 = (-50.0);
            let (t12d, t12e,) = {
    if ((!(l.f17b2 > 50.0)) && (!(l.f17b2 < t120))) {
        let t121: f64 = (l.f17b2).exp();
        (t121, (t121 * l.f17b3),)
    } else {
        let t122: f64 = (-50.0);
        let (t12b, t12c,) = {
            if ((!(l.f17b2 > 50.0)) && (l.f17b2 < t122)) {
                let t123: f64 = (-50.0);let t124: f64 = (t123).exp();
                (t124, 0.0,)
            } else {
                let (t129, t12a,) = {
                    if (l.f17b2 > 50.0) {
                        let t125: f64 = (50.0_f64).exp();let t126: f64 = (l.f17b2 - 50.0);let t127: f64 = (1.0 + t126);let t128: f64 = (t125 * t127);
                        (t128, (t125 * l.f17b3),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t129, t12a,)
            }
        };
        (t12b, t12c,)
    }
};
            (l.f17a0, l.f17a1, ) = (t12d, t12e, );
        }
        if (((l.f201e != 0.0) && (l.f2020 == 0.0)) && (l.f2021 != 0.0)) {let t12f: f64 = (l.f180c * l.f17db);let t130: f64 = (l.f17a0 - t12f);let t131: f64 = (t130 - l.f181c);(l.f17ed, l.f17ee, ) = (t131, ((l.f17a1 - (l.f180c * l.f17dc)) - l.f181d), );let t132: f64 = (l.f1811 / l.f1813);let t133: f64 = (t132 * l.f1822);let t134: f64 = (t133 + l.f17c4);(l.f17ab, l.f17ac, l.f17af, l.f17b0, l.f17b1, l.f17ad, l.f17ae, ) = (t134, (t132 * l.f1823), (t132 * l.f1826), (((-((l.f1811 * l.f1814) / (l.f1813 * l.f1813))) * l.f1822) + l.f17c5), (t132 * l.f1827), (t132 * l.f1824), (t132 * l.f1825), );}
        if (((l.f201e != 0.0) && (l.f2020 == 0.0)) && (l.f2021 != 0.0)) {
            let t135: f64 = (-50.0);
            let (t14c, t14d, t150, t151, t152, t14e, t14f,) = {
    if ((!(l.f17ab > 50.0)) && (!(l.f17ab < t135))) {
        let t136: f64 = (l.f17ab).exp();
        (t136, (t136 * l.f17ac), (t136 * l.f17af), (t136 * l.f17b0), (t136 * l.f17b1), (t136 * l.f17ad), (t136 * l.f17ae),)
    } else {
        let t137: f64 = (-50.0);
        let (t145, t146, t149, t14a, t14b, t147, t148,) = {
            if ((!(l.f17ab > 50.0)) && (l.f17ab < t137)) {
                let t138: f64 = (-50.0);let t139: f64 = (t138).exp();
                (t139, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let (t13e, t13f, t142, t143, t144, t140, t141,) = {
                    if (l.f17ab > 50.0) {
                        let t13a: f64 = (50.0_f64).exp();let t13b: f64 = (l.f17ab - 50.0);let t13c: f64 = (1.0 + t13b);let t13d: f64 = (t13a * t13c);
                        (t13d, (t13a * l.f17ac), (t13a * l.f17af), (t13a * l.f17b0), (t13a * l.f17b1), (t13a * l.f17ad), (t13a * l.f17ae),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t13e, t13f, t142, t143, t144, t140, t141,)
            }
        };
        (t145, t146, t149, t14a, t14b, t147, t148,)
    }
};
            (l.f1799, l.f179a, l.f179d, l.f179e, l.f179f, l.f179b, l.f179c, ) = (t14c, t14d, t150, t151, t152, t14e, t14f, );
        }
        if (((l.f201e != 0.0) && (l.f2020 == 0.0)) && (l.f2021 != 0.0)) {let t153: f64 = (l.f1808 * l.f17f6);let t154: f64 = (t153 / l.f17ed);(l.f17eb, l.f17ec, ) = (t154, (((((l.f1809 * l.f17f6) + (l.f1808 * l.f17f7)) * l.f17ed) - (t153 * l.f17ee)) / (l.f17ed * l.f17ed)), );let t155: f64 = (l.f180c * l.f17d4);let t156: f64 = (l.f1799 - t155);let t157: f64 = (t156 - l.f181c);let t158: f64 = (l.f17eb * t157);(l.f17e4, l.f17e5, l.f17e8, l.f17e9, l.f17ea, l.f17e6, l.f17e7, ) = (t158, (l.f17eb * (l.f179a - (l.f180c * l.f17d5))), (l.f17eb * (l.f179d - (l.f180c * l.f17d8))), ((l.f17ec * t157) + (l.f17eb * ((l.f179e - (l.f180c * l.f17d9)) - l.f181d))), (l.f17eb * (l.f179f - (l.f180c * l.f17da))), (l.f17eb * (l.f179b - (l.f180c * l.f17d6))), (l.f17eb * (l.f179c - (l.f180c * l.f17d7))), );}
        if (((l.f201e != 0.0) && (l.f2020 == 0.0)) && (l.f2021 == 0.0)) {let t159: f64 = (l.f1808 * l.f17f6);(l.f17e4, l.f17e5, l.f17e8, l.f17e9, l.f17ea, l.f17e6, l.f17e7, ) = (t159, 0.0, 0.0, ((l.f1809 * l.f17f6) + (l.f1808 * l.f17f7)), 0.0, 0.0, 0.0, );}
        if ((l.f201e != 0.0) && (l.f2020 == 0.0)) {let t15a: f64 = (l.f1773 * l.f1773);let t15b: f64 = (t15a * l.f1813);(l.f1771, l.f1772, ) = (t15b, (t15a * l.f1814), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_147(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if ((l.f201e != 0.0) && (l.f2020 == 0.0)) {let t15c: f64 = (l.f1771 / 2.0);let t15d: f64 = (l.f1828 - t15c);let t15e: f64 = (l.f1822 - t15d);let t15f: f64 = (t15e / l.f1771);(l.f178b, l.f178c, l.f178f, l.f1790, l.f1791, l.f178d, l.f178e, ) = (t15f, (l.f1823 / l.f1771), (l.f1826 / l.f1771), ((((-(-(l.f1772 / 2.0))) * l.f1771) - (t15e * l.f1772)) / (l.f1771 * l.f1771)), (l.f1827 / l.f1771), (l.f1824 / l.f1771), (l.f1825 / l.f1771), );}
        let t160: f64 = if l.f178b > 50.0 { 1.0 } else { 0.0 };l.f2022 = t160;
        if (((l.f201e != 0.0) && (l.f2020 == 0.0)) && (l.f2022 != 0.0)) {(l.f17c6, l.f17c7, l.f17ca, l.f17cb, l.f17cc, l.f17c8, l.f17c9, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );}
        let t161: f64 = (-50.0);let t162: f64 = if l.f178b < t161 { 1.0 } else { 0.0 };l.f2023 = t162;
        if ((((l.f201e != 0.0) && (l.f2020 == 0.0)) && (l.f2022 == 0.0)) && (l.f2023 != 0.0)) {(l.f17c6, l.f17c7, l.f17ca, l.f17cb, l.f17cc, l.f17c8, l.f17c9, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );}
        if ((((l.f201e != 0.0) && (l.f2020 == 0.0)) && (l.f2022 == 0.0)) && (l.f2023 == 0.0)) {let t163: f64 = (l.f178b).exp();let t164: f64 = (1.0 + t163);let t165: f64 = (1.0 / t164);(l.f17c6, l.f17c7, l.f17ca, l.f17cb, l.f17cc, l.f17c8, l.f17c9, ) = (t165, (-((t163 * l.f178c) / (t164 * t164))), (-((t163 * l.f178f) / (t164 * t164))), (-((t163 * l.f1790) / (t164 * t164))), (-((t163 * l.f1791) / (t164 * t164))), (-((t163 * l.f178d) / (t164 * t164))), (-((t163 * l.f178e) / (t164 * t164))), );}
        if ((l.f201e != 0.0) && (l.f2020 == 0.0)) {let t166: f64 = (l.f17c6 * l.f17ef);let t167: f64 = (1.0 - l.f17c6);let t168: f64 = (t167 * l.f17e4);let t169: f64 = (t166 + t168);(l.f17dd, l.f17de, l.f17e1, l.f17e2, l.f17e3, l.f17df, l.f17e0, ) = (t169, (((l.f17c7 * l.f17ef) + (l.f17c6 * l.f17f0)) + (((-l.f17c7) * l.f17e4) + (t167 * l.f17e5))), (((l.f17ca * l.f17ef) + (l.f17c6 * l.f17f3)) + (((-l.f17ca) * l.f17e4) + (t167 * l.f17e8))), (((l.f17cb * l.f17ef) + (l.f17c6 * l.f17f4)) + (((-l.f17cb) * l.f17e4) + (t167 * l.f17e9))), (((l.f17cc * l.f17ef) + (l.f17c6 * l.f17f5)) + (((-l.f17cc) * l.f17e4) + (t167 * l.f17ea))), (((l.f17c8 * l.f17ef) + (l.f17c6 * l.f17f1)) + (((-l.f17c8) * l.f17e4) + (t167 * l.f17e6))), (((l.f17c9 * l.f17ef) + (l.f17c6 * l.f17f2)) + (((-l.f17c9) * l.f17e4) + (t167 * l.f17e7))), );}
        if (l.f201e != 0.0) {
            let t16a: f64 = (-l.f1822);
            let (t17c, t17d, t180, t181, t17e, t17f,) = {
    if (p.p52 != 0.0) {
        let t16b: f64 = (l.f1822 / l.f1829);let t16c: f64 = (0.001 / p.p53);let t16d: f64 = (l.f1822 / l.f1829);let t16e: f64 = (t16c * t16d);let t16f: f64 = (t16e).tanh();let t170: f64 = (t16b * t16f);
        (t170, (((l.f1823 / l.f1829) * t16f) + (t16b * ((t16c * (l.f1823 / l.f1829)) / ((t16e).cosh() * (t16e).cosh())))), (((l.f1826 / l.f1829) * t16f) + (t16b * ((t16c * (l.f1826 / l.f1829)) / ((t16e).cosh() * (t16e).cosh())))), (((l.f1827 / l.f1829) * t16f) + (t16b * ((t16c * (l.f1827 / l.f1829)) / ((t16e).cosh() * (t16e).cosh())))), (((l.f1824 / l.f1829) * t16f) + (t16b * ((t16c * (l.f1824 / l.f1829)) / ((t16e).cosh() * (t16e).cosh())))), (((l.f1825 / l.f1829) * t16f) + (t16b * ((t16c * (l.f1825 / l.f1829)) / ((t16e).cosh() * (t16e).cosh())))),)
    } else {
        let (t176, t177, t17a, t17b, t178, t179,) = {
            if (p.p52 == 0.0) {
                let __rspice_inv_cse_0: f64 = 1.0 / l.f1829;let t171: f64 = (l.f1822 * __rspice_inv_cse_0);let t172: f64 = (l.f1822 * __rspice_inv_cse_0);let t173: f64 = (t171 * t172);let t174: f64 = (t173 + p.p53);let t175: f64 = (t174).sqrt();
                (t175, ((((l.f1823 / l.f1829) * t172) + (t171 * (l.f1823 / l.f1829))) / (2.0 * t175)), ((((l.f1826 / l.f1829) * t172) + (t171 * (l.f1826 / l.f1829))) / (2.0 * t175)), ((((l.f1827 / l.f1829) * t172) + (t171 * (l.f1827 / l.f1829))) / (2.0 * t175)), ((((l.f1824 / l.f1829) * t172) + (t171 * (l.f1824 / l.f1829))) / (2.0 * t175)), ((((l.f1825 / l.f1829) * t172) + (t171 * (l.f1825 / l.f1829))) / (2.0 * t175)),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (t176, t177, t17a, t17b, t178, t179,)
    }
};
            let t182: f64 = (t17c).powf(l.f1774);let t183: f64 = (1.0 + t182);let t184: f64 = (1.0 / l.f1774);let t185: f64 = (t183).powf(t184);let t186: f64 = (t16a / t185);
            (l.f17ce, l.f17cf, l.f17d2, l.f17d3, l.f17d0, l.f17d1, ) = (t186, ((((-l.f1823) * t185) - (t16a * if 0.0 == 0.0 && ((t184) as f64).is_finite() && ((t184) as f64).fract() == 0.0 { if t184 == 0.0 { 0.0 } else { (t184 * ((t183).powf(t184 - 1.0) * if 0.0 == 0.0 && ((l.f1774) as f64).is_finite() && ((l.f1774) as f64).fract() == 0.0 { if l.f1774 == 0.0 { 0.0 } else { (l.f1774 * ((t17c).powf(l.f1774 - 1.0) * t17d)) } } else { (t182 * (l.f1774 * (t17d / t17c))) })) } } else { (t185 * (t184 * (if 0.0 == 0.0 && ((l.f1774) as f64).is_finite() && ((l.f1774) as f64).fract() == 0.0 { if l.f1774 == 0.0 { 0.0 } else { (l.f1774 * ((t17c).powf(l.f1774 - 1.0) * t17d)) } } else { (t182 * (l.f1774 * (t17d / t17c))) } / t183))) })) / (t185 * t185)), ((((-l.f1826) * t185) - (t16a * if 0.0 == 0.0 && ((t184) as f64).is_finite() && ((t184) as f64).fract() == 0.0 { if t184 == 0.0 { 0.0 } else { (t184 * ((t183).powf(t184 - 1.0) * if 0.0 == 0.0 && ((l.f1774) as f64).is_finite() && ((l.f1774) as f64).fract() == 0.0 { if l.f1774 == 0.0 { 0.0 } else { (l.f1774 * ((t17c).powf(l.f1774 - 1.0) * t180)) } } else { (t182 * (l.f1774 * (t180 / t17c))) })) } } else { (t185 * (t184 * (if 0.0 == 0.0 && ((l.f1774) as f64).is_finite() && ((l.f1774) as f64).fract() == 0.0 { if l.f1774 == 0.0 { 0.0 } else { (l.f1774 * ((t17c).powf(l.f1774 - 1.0) * t180)) } } else { (t182 * (l.f1774 * (t180 / t17c))) } / t183))) })) / (t185 * t185)), ((((-l.f1827) * t185) - (t16a * if 0.0 == 0.0 && ((t184) as f64).is_finite() && ((t184) as f64).fract() == 0.0 { if t184 == 0.0 { 0.0 } else { (t184 * ((t183).powf(t184 - 1.0) * if 0.0 == 0.0 && ((l.f1774) as f64).is_finite() && ((l.f1774) as f64).fract() == 0.0 { if l.f1774 == 0.0 { 0.0 } else { (l.f1774 * ((t17c).powf(l.f1774 - 1.0) * t181)) } } else { (t182 * (l.f1774 * (t181 / t17c))) })) } } else { (t185 * (t184 * (if 0.0 == 0.0 && ((l.f1774) as f64).is_finite() && ((l.f1774) as f64).fract() == 0.0 { if l.f1774 == 0.0 { 0.0 } else { (l.f1774 * ((t17c).powf(l.f1774 - 1.0) * t181)) } } else { (t182 * (l.f1774 * (t181 / t17c))) } / t183))) })) / (t185 * t185)), ((((-l.f1824) * t185) - (t16a * if 0.0 == 0.0 && ((t184) as f64).is_finite() && ((t184) as f64).fract() == 0.0 { if t184 == 0.0 { 0.0 } else { (t184 * ((t183).powf(t184 - 1.0) * if 0.0 == 0.0 && ((l.f1774) as f64).is_finite() && ((l.f1774) as f64).fract() == 0.0 { if l.f1774 == 0.0 { 0.0 } else { (l.f1774 * ((t17c).powf(l.f1774 - 1.0) * t17e)) } } else { (t182 * (l.f1774 * (t17e / t17c))) })) } } else { (t185 * (t184 * (if 0.0 == 0.0 && ((l.f1774) as f64).is_finite() && ((l.f1774) as f64).fract() == 0.0 { if l.f1774 == 0.0 { 0.0 } else { (l.f1774 * ((t17c).powf(l.f1774 - 1.0) * t17e)) } } else { (t182 * (l.f1774 * (t17e / t17c))) } / t183))) })) / (t185 * t185)), ((((-l.f1825) * t185) - (t16a * if 0.0 == 0.0 && ((t184) as f64).is_finite() && ((t184) as f64).fract() == 0.0 { if t184 == 0.0 { 0.0 } else { (t184 * ((t183).powf(t184 - 1.0) * if 0.0 == 0.0 && ((l.f1774) as f64).is_finite() && ((l.f1774) as f64).fract() == 0.0 { if l.f1774 == 0.0 { 0.0 } else { (l.f1774 * ((t17c).powf(l.f1774 - 1.0) * t17f)) } } else { (t182 * (l.f1774 * (t17f / t17c))) })) } } else { (t185 * (t184 * (if 0.0 == 0.0 && ((l.f1774) as f64).is_finite() && ((l.f1774) as f64).fract() == 0.0 { if l.f1774 == 0.0 { 0.0 } else { (l.f1774 * ((t17c).powf(l.f1774 - 1.0) * t17f)) } } else { (t182 * (l.f1774 * (t17f / t17c))) } / t183))) })) / (t185 * t185)), );
        }
        if (l.f201e != 0.0) {let t187: f64 = (-l.f1820);let t188: f64 = (t187 * l.f182b);let t189: f64 = (t188 * l.f180d);let t18a: f64 = (t189 * l.f1807);let t18b: f64 = (t18a * l.f181e);let t18c: f64 = t18b;(l.f180a, l.f180b, ) = (t18c, (t18a * l.f181f), );let t18d: f64 = (l.f1812 / l.f1813);let t18e: f64 = (t18d * l.f17ce);(l.f17bd, l.f17be, l.f17c1, l.f17c2, l.f17c3, l.f17bf, l.f17c0, ) = (t18e, (t18d * l.f17cf), (t18d * l.f17d2), ((-((l.f1812 * l.f1814) / (l.f1813 * l.f1813))) * l.f17ce), (t18d * l.f17d3), (t18d * l.f17d0), (t18d * l.f17d1), );}
        if (l.f201e != 0.0) {
            let t18f: f64 = (-50.0);
            let (t1a6, t1a7, t1aa, t1ab, t1ac, t1a8, t1a9,) = {
    if ((!(l.f17bd > 50.0)) && (!(l.f17bd < t18f))) {
        let t190: f64 = (l.f17bd).exp();
        (t190, (t190 * l.f17be), (t190 * l.f17c1), (t190 * l.f17c2), (t190 * l.f17c3), (t190 * l.f17bf), (t190 * l.f17c0),)
    } else {
        let t191: f64 = (-50.0);
        let (t19f, t1a0, t1a3, t1a4, t1a5, t1a1, t1a2,) = {
            if ((!(l.f17bd > 50.0)) && (l.f17bd < t191)) {
                let t192: f64 = (-50.0);let t193: f64 = (t192).exp();
                (t193, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let (t198, t199, t19c, t19d, t19e, t19a, t19b,) = {
                    if (l.f17bd > 50.0) {
                        let t194: f64 = (50.0_f64).exp();let t195: f64 = (l.f17bd - 50.0);let t196: f64 = (1.0 + t195);let t197: f64 = (t194 * t196);
                        (t197, (t194 * l.f17be), (t194 * l.f17c1), (t194 * l.f17c2), (t194 * l.f17c3), (t194 * l.f17bf), (t194 * l.f17c0),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t198, t199, t19c, t19d, t19e, t19a, t19b,)
            }
        };
        (t19f, t1a0, t1a3, t1a4, t1a5, t1a1, t1a2,)
    }
};
            (l.f17b6, l.f17b7, l.f17ba, l.f17bb, l.f17bc, l.f17b8, l.f17b9, ) = (t1a6, t1a7, t1aa, t1ab, t1ac, t1a8, t1a9, );
        }
        if (l.f201e != 0.0) {let t1ad: f64 = (l.f17b6 - 1.0);let t1ae: f64 = (l.f180a * t1ad);(l.f17f8, l.f17f9, l.f17fc, l.f17fd, l.f17fe, l.f17fa, l.f17fb, ) = (t1ae, (l.f180a * l.f17b7), (l.f180a * l.f17ba), ((l.f180b * t1ad) + (l.f180a * l.f17bb)), (l.f180a * l.f17bc), (l.f180a * l.f17b8), (l.f180a * l.f17b9), );let t1af: f64 = (l.f17dd + l.f17f8);(l.f17ff, l.f1800, l.f1803, l.f1804, l.f1805, l.f1801, l.f1802, ) = (t1af, (l.f17de + l.f17f9), (l.f17e1 + l.f17fc), (l.f17e2 + l.f17fd), (l.f17e3 + l.f17fe), (l.f17df + l.f17fa), (l.f17e0 + l.f17fb), );(l.f1815, l.f1816, l.f1819, l.f181a, l.f181b, l.f1817, l.f1818, ) = (l.f17ff, l.f1800, l.f1803, l.f1804, l.f1805, l.f1801, l.f1802, );(l.f212c, l.f212d, l.f2130, l.f2131, l.f2132, l.f212e, l.f212f, ) = (l.f1815, l.f1816, l.f1819, l.f181a, l.f181b, l.f1817, l.f1818, );(l.f18d0, l.f18d1, l.f18d4, l.f18d5, l.f18d6, l.f18d2, l.f18d3, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f18c3, l.f18c4, ) = (0.0, 0.0, );(l.f18c5, l.f18c6, ) = (0.0, 0.0, );(l.f18dd, l.f18de, l.f18e1, l.f18e2, l.f18df, l.f18e0, ) = (l.f23b7, l.f23b8, l.f23bb, l.f23bc, l.f23b9, l.f23ba, );(l.f18ce, l.f18cf, ) = (l.f215b, l.f215c, );l.f18e3 = p.p265;l.f182e = p.p267;l.f1888 = p.p266;l.f18cb = 0.0;l.f18c9 = p.p319;l.f18dc = p.p318;(l.f18d9, l.f18da, ) = (l.f22f2, l.f22f3, );l.f18e6 = p.p0;l.f18c8 = p.p2;l.f18c1 = p.p315;l.f18c7 = 1.0;l.f18e4 = p.p274;l.f182f = p.p275;l.f18c2 = 0.0;l.f18cd = p.p272;l.f18ca = 0.0;l.f18e5 = p.p256;l.f18db = p.p6;(l.f18ba, l.f18bb, l.f18be, l.f18bf, l.f18c0, l.f18bc, l.f18bd, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f182c, l.f182d, ) = (0.0, 0.0, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_148(
        l: &mut StampLocals,
    ) {
        if (l.f201e != 0.0) {(l.f18d7, l.f18d8, ) = (0.0, 0.0, );(l.f1881, l.f1882, l.f1885, l.f1886, l.f1887, l.f1883, l.f1884, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f188f, l.f1890, l.f1893, l.f1894, l.f1895, l.f1891, l.f1892, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1898, l.f1899, l.f189c, l.f189d, l.f189e, l.f189a, l.f189b, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1889, l.f188a, l.f188d, l.f188e, l.f188b, l.f188c, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f18b3, l.f18b4, l.f18b7, l.f18b8, l.f18b9, l.f18b5, l.f18b6, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f183b, l.f183c, l.f183f, l.f1840, l.f1841, l.f183d, l.f183e, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1844, l.f1845, ) = (0.0, 0.0, );(l.f1830, l.f1831, l.f1834, l.f1835, l.f1836, l.f1832, l.f1833, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1839, l.f183a, ) = (0.0, 0.0, );(l.f187f, l.f1880, ) = (0.0, 0.0, );(l.f1846, l.f1847, l.f184a, l.f184b, l.f184c, l.f1848, l.f1849, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f185f, l.f1860, l.f1863, l.f1864, l.f1865, l.f1861, l.f1862, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f184d, l.f184e, l.f1851, l.f1852, l.f1853, l.f184f, l.f1850, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1878, l.f1879, l.f187c, l.f187d, l.f187e, l.f187a, l.f187b, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1871, l.f1872, l.f1875, l.f1876, l.f1877, l.f1873, l.f1874, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );l.f18cc = 0.0;(l.f1842, l.f1843, ) = (0.0, 0.0, );(l.f1837, l.f1838, ) = (0.0, 0.0, );(l.f1896, l.f1897, ) = (0.0, 0.0, );(l.f186f, l.f1870, ) = (0.0, 0.0, );(l.f185d, l.f185e, ) = (0.0, 0.0, );(l.f18b1, l.f18b2, ) = (0.0, 0.0, );(l.f18aa, l.f18ab, l.f18ae, l.f18af, l.f18b0, l.f18ac, l.f18ad, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f186d, l.f186e, ) = (0.0, 0.0, );(l.f185b, l.f185c, ) = (0.0, 0.0, );(l.f18a8, l.f18a9, ) = (0.0, 0.0, );(l.f1866, l.f1867, l.f186a, l.f186b, l.f186c, l.f1868, l.f1869, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f1854, l.f1855, l.f1858, l.f1859, l.f185a, l.f1856, l.f1857, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );(l.f18a6, l.f18a7, ) = (0.0, 0.0, );(l.f189f, l.f18a0, l.f18a3, l.f18a4, l.f18a5, l.f18a1, l.f18a2, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );let t1b9: f64 = (l.f18ca / l.f18ce);let t1ba: f64 = (-l.f18e5);let t1bb: f64 = (t1b9 * t1ba);(l.f187f, l.f1880, ) = (t1bb, ((-((l.f18ca * l.f18cf) / (l.f18ce * l.f18ce))) * t1ba), );}
        if (l.f201e != 0.0) {
            let t1bc: f64 = (-50.0);
            let (t1c9, t1ca,) = {
    if ((!(l.f187f > 50.0)) && (!(l.f187f < t1bc))) {
        let t1bd: f64 = (l.f187f).exp();
        (t1bd, (t1bd * l.f1880),)
    } else {
        let t1be: f64 = (-50.0);
        let (t1c7, t1c8,) = {
            if ((!(l.f187f > 50.0)) && (l.f187f < t1be)) {
                let t1bf: f64 = (-50.0);let t1c0: f64 = (t1bf).exp();
                (t1c0, 0.0,)
            } else {
                let (t1c5, t1c6,) = {
                    if (l.f187f > 50.0) {
                        let t1c1: f64 = (50.0_f64).exp();let t1c2: f64 = (l.f187f - 50.0);let t1c3: f64 = (1.0 + t1c2);let t1c4: f64 = (t1c1 * t1c3);
                        (t1c4, (t1c1 * l.f1880),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t1c5, t1c6,)
            }
        };
        (t1c7, t1c8,)
    }
};
            (l.f18d7, l.f18d8, ) = (t1c9, t1ca, );
        }
        if (l.f201e != 0.0) {let t1cb: f64 = (-l.f18dd);let t1cc: f64 = (t1cb - l.f18dc);let t1cd: f64 = (l.f18c9 * t1cc);let t1ce: f64 = (t1cd + l.f187f);(l.f183b, l.f183c, l.f183f, l.f1840, l.f1841, l.f183d, l.f183e, ) = (t1ce, (l.f18c9 * (-l.f18de)), (l.f18c9 * (-l.f18e1)), l.f1880, (l.f18c9 * (-l.f18e2)), (l.f18c9 * (-l.f18df)), (l.f18c9 * (-l.f18e0)), );let t1cf: f64 = (-l.f18c9);let t1d0: f64 = (t1cf * l.f18dc);let t1d1: f64 = (t1d0 + l.f187f);(l.f1844, l.f1845, ) = (t1d1, l.f1880, );}
        if (l.f201e != 0.0) {
            let t1d2: f64 = (-50.0);
            let (t1e9, t1ea, t1ed, t1ee, t1ef, t1eb, t1ec,) = {
    if ((!(l.f183b > 50.0)) && (!(l.f183b < t1d2))) {
        let t1d3: f64 = (l.f183b).exp();
        (t1d3, (t1d3 * l.f183c), (t1d3 * l.f183f), (t1d3 * l.f1840), (t1d3 * l.f1841), (t1d3 * l.f183d), (t1d3 * l.f183e),)
    } else {
        let t1d4: f64 = (-50.0);
        let (t1e2, t1e3, t1e6, t1e7, t1e8, t1e4, t1e5,) = {
            if ((!(l.f183b > 50.0)) && (l.f183b < t1d4)) {
                let t1d5: f64 = (-50.0);let t1d6: f64 = (t1d5).exp();
                (t1d6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let (t1db, t1dc, t1df, t1e0, t1e1, t1dd, t1de,) = {
                    if (l.f183b > 50.0) {
                        let t1d7: f64 = (50.0_f64).exp();let t1d8: f64 = (l.f183b - 50.0);let t1d9: f64 = (1.0 + t1d8);let t1da: f64 = (t1d7 * t1d9);
                        (t1da, (t1d7 * l.f183c), (t1d7 * l.f183f), (t1d7 * l.f1840), (t1d7 * l.f1841), (t1d7 * l.f183d), (t1d7 * l.f183e),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t1db, t1dc, t1df, t1e0, t1e1, t1dd, t1de,)
            }
        };
        (t1e2, t1e3, t1e6, t1e7, t1e8, t1e4, t1e5,)
    }
};
            (l.f1830, l.f1831, l.f1834, l.f1835, l.f1836, l.f1832, l.f1833, ) = (t1e9, t1ea, t1ed, t1ee, t1ef, t1eb, t1ec, );
        }
        if (l.f201e != 0.0) {
            let t1f0: f64 = (-50.0);
            let (t1fd, t1fe,) = {
    if ((!(l.f1844 > 50.0)) && (!(l.f1844 < t1f0))) {
        let t1f1: f64 = (l.f1844).exp();
        (t1f1, (t1f1 * l.f1845),)
    } else {
        let t1f2: f64 = (-50.0);
        let (t1fb, t1fc,) = {
            if ((!(l.f1844 > 50.0)) && (l.f1844 < t1f2)) {
                let t1f3: f64 = (-50.0);let t1f4: f64 = (t1f3).exp();
                (t1f4, 0.0,)
            } else {
                let (t1f9, t1fa,) = {
                    if (l.f1844 > 50.0) {
                        let t1f5: f64 = (50.0_f64).exp();let t1f6: f64 = (l.f1844 - 50.0);let t1f7: f64 = (1.0 + t1f6);let t1f8: f64 = (t1f5 * t1f7);
                        (t1f8, (t1f5 * l.f1845),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t1f9, t1fa,)
            }
        };
        (t1fb, t1fc,)
    }
};
            (l.f1839, l.f183a, ) = (t1fd, t1fe, );
        }
        if (l.f201e != 0.0) {let t1ff: f64 = (l.f1830 - l.f1839);(l.f188f, l.f1890, l.f1893, l.f1894, l.f1895, l.f1891, l.f1892, ) = (t1ff, l.f1831, l.f1834, (l.f1835 - l.f183a), l.f1836, l.f1832, l.f1833, );let t200: f64 = (l.f18db * l.f18e6);let t201: f64 = (t200 * l.f18c8);let t202: f64 = (t201 * l.f18c1);let t203: f64 = (t202 * l.f18d9);(l.f18c3, l.f18c4, ) = (t203, (t202 * l.f18da), );let t204: f64 = (l.f18cb / l.f18ce);let t205: f64 = (t204 * l.f18dd);let t206: f64 = (t205 + l.f187f);(l.f185f, l.f1860, l.f1863, l.f1864, l.f1865, l.f1861, l.f1862, ) = (t206, (t204 * l.f18de), (t204 * l.f18e1), (((-((l.f18cb * l.f18cf) / (l.f18ce * l.f18ce))) * l.f18dd) + l.f1880), (t204 * l.f18e2), (t204 * l.f18df), (t204 * l.f18e0), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_149(
        l: &mut StampLocals,
    ) {
        if (l.f201e != 0.0) {
            let t207: f64 = (-50.0);
            let (t21e, t21f, t222, t223, t224, t220, t221,) = {
    if ((!(l.f185f > 50.0)) && (!(l.f185f < t207))) {
        let t208: f64 = (l.f185f).exp();
        (t208, (t208 * l.f1860), (t208 * l.f1863), (t208 * l.f1864), (t208 * l.f1865), (t208 * l.f1861), (t208 * l.f1862),)
    } else {
        let t209: f64 = (-50.0);
        let (t217, t218, t21b, t21c, t21d, t219, t21a,) = {
            if ((!(l.f185f > 50.0)) && (l.f185f < t209)) {
                let t20a: f64 = (-50.0);let t20b: f64 = (t20a).exp();
                (t20b, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let (t210, t211, t214, t215, t216, t212, t213,) = {
                    if (l.f185f > 50.0) {
                        let t20c: f64 = (50.0_f64).exp();let t20d: f64 = (l.f185f - 50.0);let t20e: f64 = (1.0 + t20d);let t20f: f64 = (t20c * t20e);
                        (t20f, (t20c * l.f1860), (t20c * l.f1863), (t20c * l.f1864), (t20c * l.f1865), (t20c * l.f1861), (t20c * l.f1862),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t210, t211, t214, t215, t216, t212, t213,)
            }
        };
        (t217, t218, t21b, t21c, t21d, t219, t21a,)
    }
};
            (l.f184d, l.f184e, l.f1851, l.f1852, l.f1853, l.f184f, l.f1850, ) = (t21e, t21f, t222, t223, t224, t220, t221, );
        }
        let t225: f64 = if l.f1888 == 1.0 { 1.0 } else { 0.0 };l.f2024 = t225;
        if ((l.f201e != 0.0) && (l.f2024 != 0.0)) {let t226: f64 = (l.f18c7 * l.f188f);let t227: f64 = (l.f184d - t226);let t228: f64 = (t227 - l.f18d7);let t229: f64 = (l.f18c3 * t228);(l.f1898, l.f1899, l.f189c, l.f189d, l.f189e, l.f189a, l.f189b, ) = (t229, (l.f18c3 * (l.f184e - (l.f18c7 * l.f1890))), (l.f18c3 * (l.f1851 - (l.f18c7 * l.f1893))), ((l.f18c4 * t228) + (l.f18c3 * ((l.f1852 - (l.f18c7 * l.f1894)) - l.f18d8))), (l.f18c3 * (l.f1853 - (l.f18c7 * l.f1895))), (l.f18c3 * (l.f184f - (l.f18c7 * l.f1891))), (l.f18c3 * (l.f1850 - (l.f18c7 * l.f1892))), );}
        if ((l.f201e != 0.0) && (l.f2024 == 0.0)) {let t22a: f64 = (-l.f18e3);let t22b: f64 = (t22a - l.f18dc);let t22c: f64 = (l.f18c9 * t22b);let t22d: f64 = (t22c + l.f187f);(l.f1842, l.f1843, ) = (t22d, l.f1880, );}
        if ((l.f201e != 0.0) && (l.f2024 == 0.0)) {
            let t22e: f64 = (-50.0);
            let (t23b, t23c,) = {
    if ((!(l.f1842 > 50.0)) && (!(l.f1842 < t22e))) {
        let t22f: f64 = (l.f1842).exp();
        (t22f, (t22f * l.f1843),)
    } else {
        let t230: f64 = (-50.0);
        let (t239, t23a,) = {
            if ((!(l.f1842 > 50.0)) && (l.f1842 < t230)) {
                let t231: f64 = (-50.0);let t232: f64 = (t231).exp();
                (t232, 0.0,)
            } else {
                let (t237, t238,) = {
                    if (l.f1842 > 50.0) {
                        let t233: f64 = (50.0_f64).exp();let t234: f64 = (l.f1842 - 50.0);let t235: f64 = (1.0 + t234);let t236: f64 = (t233 * t235);
                        (t236, (t233 * l.f1843),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t237, t238,)
            }
        };
        (t239, t23a,)
    }
};
            (l.f1837, l.f1838, ) = (t23b, t23c, );
        }
        if ((l.f201e != 0.0) && (l.f2024 == 0.0)) {let t23d: f64 = (l.f1837 - l.f1839);(l.f1896, l.f1897, ) = (t23d, (l.f1838 - l.f183a), );let t23e: f64 = (l.f18cb / l.f18ce);let t23f: f64 = (t23e * l.f18e3);let t240: f64 = (t23f + l.f187f);(l.f186f, l.f1870, ) = (t240, (((-((l.f18cb * l.f18cf) / (l.f18ce * l.f18ce))) * l.f18e3) + l.f1880), );}
        if ((l.f201e != 0.0) && (l.f2024 == 0.0)) {
            let t241: f64 = (-50.0);
            let (t24e, t24f,) = {
    if ((!(l.f186f > 50.0)) && (!(l.f186f < t241))) {
        let t242: f64 = (l.f186f).exp();
        (t242, (t242 * l.f1870),)
    } else {
        let t243: f64 = (-50.0);
        let (t24c, t24d,) = {
            if ((!(l.f186f > 50.0)) && (l.f186f < t243)) {
                let t244: f64 = (-50.0);let t245: f64 = (t244).exp();
                (t245, 0.0,)
            } else {
                let (t24a, t24b,) = {
                    if (l.f186f > 50.0) {
                        let t246: f64 = (50.0_f64).exp();let t247: f64 = (l.f186f - 50.0);let t248: f64 = (1.0 + t247);let t249: f64 = (t246 * t248);
                        (t249, (t246 * l.f1870),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t24a, t24b,)
            }
        };
        (t24c, t24d,)
    }
};
            (l.f185d, l.f185e, ) = (t24e, t24f, );
        }
        if ((l.f201e != 0.0) && (l.f2024 == 0.0)) {let t250: f64 = (l.f18c7 * l.f1896);let t251: f64 = (l.f185d - t250);let t252: f64 = (t251 - l.f18d7);(l.f18b1, l.f18b2, ) = (t252, ((l.f185e - (l.f18c7 * l.f1897)) - l.f18d8), );let t253: f64 = (l.f18c7 * l.f188f);let t254: f64 = (l.f184d - t253);let t255: f64 = (t254 - l.f18d7);let t256: f64 = (l.f18c3 * t255);(l.f18aa, l.f18ab, l.f18ae, l.f18af, l.f18b0, l.f18ac, l.f18ad, ) = (t256, (l.f18c3 * (l.f184e - (l.f18c7 * l.f1890))), (l.f18c3 * (l.f1851 - (l.f18c7 * l.f1893))), ((l.f18c4 * t255) + (l.f18c3 * ((l.f1852 - (l.f18c7 * l.f1894)) - l.f18d8))), (l.f18c3 * (l.f1853 - (l.f18c7 * l.f1895))), (l.f18c3 * (l.f184f - (l.f18c7 * l.f1891))), (l.f18c3 * (l.f1850 - (l.f18c7 * l.f1892))), );}
        let t257: f64 = if l.f1888 > 0.0 { 1.0 } else { 0.0 };l.f2025 = t257;
        if (((l.f201e != 0.0) && (l.f2024 == 0.0)) && (l.f2025 != 0.0)) {let t258: f64 = (l.f1888 * l.f18cb);l.f18cc = t258;let t259: f64 = (l.f18cc / l.f18ce);let t25a: f64 = (t259 * l.f18e3);let t25b: f64 = (t25a + l.f187f);(l.f186d, l.f186e, ) = (t25b, (((-((l.f18cc * l.f18cf) / (l.f18ce * l.f18ce))) * l.f18e3) + l.f1880), );}
        if (((l.f201e != 0.0) && (l.f2024 == 0.0)) && (l.f2025 != 0.0)) {
            let t25c: f64 = (-50.0);
            let (t269, t26a,) = {
    if ((!(l.f186d > 50.0)) && (!(l.f186d < t25c))) {
        let t25d: f64 = (l.f186d).exp();
        (t25d, (t25d * l.f186e),)
    } else {
        let t25e: f64 = (-50.0);
        let (t267, t268,) = {
            if ((!(l.f186d > 50.0)) && (l.f186d < t25e)) {
                let t25f: f64 = (-50.0);let t260: f64 = (t25f).exp();
                (t260, 0.0,)
            } else {
                let (t265, t266,) = {
                    if (l.f186d > 50.0) {
                        let t261: f64 = (50.0_f64).exp();let t262: f64 = (l.f186d - 50.0);let t263: f64 = (1.0 + t262);let t264: f64 = (t261 * t263);
                        (t264, (t261 * l.f186e),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (t265, t266,)
            }
        };
        (t267, t268,)
    }
};
            (l.f185b, l.f185c, ) = (t269, t26a, );
        }
        if (((l.f201e != 0.0) && (l.f2024 == 0.0)) && (l.f2025 != 0.0)) {let t26b: f64 = (l.f18c7 * l.f1896);let t26c: f64 = (l.f185b - t26b);let t26d: f64 = (t26c - l.f18d7);(l.f18a8, l.f18a9, ) = (t26d, ((l.f185c - (l.f18c7 * l.f1897)) - l.f18d8), );let t26e: f64 = (l.f18cc / l.f18ce);let t26f: f64 = (t26e * l.f18dd);let t270: f64 = (t26f + l.f187f);(l.f1866, l.f1867, l.f186a, l.f186b, l.f186c, l.f1868, l.f1869, ) = (t270, (t26e * l.f18de), (t26e * l.f18e1), (((-((l.f18cc * l.f18cf) / (l.f18ce * l.f18ce))) * l.f18dd) + l.f1880), (t26e * l.f18e2), (t26e * l.f18df), (t26e * l.f18e0), );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_150(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        if (((l.f201e != 0.0) && (l.f2024 == 0.0)) && (l.f2025 != 0.0)) {
            let t271: f64 = (-50.0);
            let (t288, t289, t28c, t28d, t28e, t28a, t28b,) = {
    if ((!(l.f1866 > 50.0)) && (!(l.f1866 < t271))) {
        let t272: f64 = (l.f1866).exp();
        (t272, (t272 * l.f1867), (t272 * l.f186a), (t272 * l.f186b), (t272 * l.f186c), (t272 * l.f1868), (t272 * l.f1869),)
    } else {
        let t273: f64 = (-50.0);
        let (t281, t282, t285, t286, t287, t283, t284,) = {
            if ((!(l.f1866 > 50.0)) && (l.f1866 < t273)) {
                let t274: f64 = (-50.0);let t275: f64 = (t274).exp();
                (t275, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let (t27a, t27b, t27e, t27f, t280, t27c, t27d,) = {
                    if (l.f1866 > 50.0) {
                        let t276: f64 = (50.0_f64).exp();let t277: f64 = (l.f1866 - 50.0);let t278: f64 = (1.0 + t277);let t279: f64 = (t276 * t278);
                        (t279, (t276 * l.f1867), (t276 * l.f186a), (t276 * l.f186b), (t276 * l.f186c), (t276 * l.f1868), (t276 * l.f1869),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t27a, t27b, t27e, t27f, t280, t27c, t27d,)
            }
        };
        (t281, t282, t285, t286, t287, t283, t284,)
    }
};
            (l.f1854, l.f1855, l.f1858, l.f1859, l.f185a, l.f1856, l.f1857, ) = (t288, t289, t28c, t28d, t28e, t28a, t28b, );
        }
        if (((l.f201e != 0.0) && (l.f2024 == 0.0)) && (l.f2025 != 0.0)) {let t28f: f64 = (l.f18c3 * l.f18b1);let t290: f64 = (t28f / l.f18a8);(l.f18a6, l.f18a7, ) = (t290, (((((l.f18c4 * l.f18b1) + (l.f18c3 * l.f18b2)) * l.f18a8) - (t28f * l.f18a9)) / (l.f18a8 * l.f18a8)), );let t291: f64 = (l.f18c7 * l.f188f);let t292: f64 = (l.f1854 - t291);let t293: f64 = (t292 - l.f18d7);let t294: f64 = (l.f18a6 * t293);(l.f189f, l.f18a0, l.f18a3, l.f18a4, l.f18a5, l.f18a1, l.f18a2, ) = (t294, (l.f18a6 * (l.f1855 - (l.f18c7 * l.f1890))), (l.f18a6 * (l.f1858 - (l.f18c7 * l.f1893))), ((l.f18a7 * t293) + (l.f18a6 * ((l.f1859 - (l.f18c7 * l.f1894)) - l.f18d8))), (l.f18a6 * (l.f185a - (l.f18c7 * l.f1895))), (l.f18a6 * (l.f1856 - (l.f18c7 * l.f1891))), (l.f18a6 * (l.f1857 - (l.f18c7 * l.f1892))), );}
        if (((l.f201e != 0.0) && (l.f2024 == 0.0)) && (l.f2025 == 0.0)) {let t295: f64 = (l.f18c3 * l.f18b1);(l.f189f, l.f18a0, l.f18a3, l.f18a4, l.f18a5, l.f18a1, l.f18a2, ) = (t295, 0.0, 0.0, ((l.f18c4 * l.f18b1) + (l.f18c3 * l.f18b2)), 0.0, 0.0, 0.0, );}
        if ((l.f201e != 0.0) && (l.f2024 == 0.0)) {let t296: f64 = (l.f182e * l.f182e);let t297: f64 = (t296 * l.f18ce);(l.f182c, l.f182d, ) = (t297, (t296 * l.f18cf), );let t298: f64 = (l.f182c / 2.0);let t299: f64 = (l.f18e3 - t298);let t29a: f64 = (l.f18dd - t299);let t29b: f64 = (t29a / l.f182c);(l.f1846, l.f1847, l.f184a, l.f184b, l.f184c, l.f1848, l.f1849, ) = (t29b, (l.f18de / l.f182c), (l.f18e1 / l.f182c), ((((-(-(l.f182d / 2.0))) * l.f182c) - (t29a * l.f182d)) / (l.f182c * l.f182c)), (l.f18e2 / l.f182c), (l.f18df / l.f182c), (l.f18e0 / l.f182c), );}
        let t29c: f64 = if l.f1846 > 50.0 { 1.0 } else { 0.0 };l.f2028 = t29c;
        if (((l.f201e != 0.0) && (l.f2024 == 0.0)) && (l.f2028 != 0.0)) {(l.f1881, l.f1882, l.f1885, l.f1886, l.f1887, l.f1883, l.f1884, ) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );}
        let t29d: f64 = (-50.0);let t29e: f64 = if l.f1846 < t29d { 1.0 } else { 0.0 };l.f2029 = t29e;
        if ((((l.f201e != 0.0) && (l.f2024 == 0.0)) && (l.f2028 == 0.0)) && (l.f2029 != 0.0)) {(l.f1881, l.f1882, l.f1885, l.f1886, l.f1887, l.f1883, l.f1884, ) = (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, );}
        if ((((l.f201e != 0.0) && (l.f2024 == 0.0)) && (l.f2028 == 0.0)) && (l.f2029 == 0.0)) {let t29f: f64 = (l.f1846).exp();let t2a0: f64 = (1.0 + t29f);let t2a1: f64 = (1.0 / t2a0);(l.f1881, l.f1882, l.f1885, l.f1886, l.f1887, l.f1883, l.f1884, ) = (t2a1, (-((t29f * l.f1847) / (t2a0 * t2a0))), (-((t29f * l.f184a) / (t2a0 * t2a0))), (-((t29f * l.f184b) / (t2a0 * t2a0))), (-((t29f * l.f184c) / (t2a0 * t2a0))), (-((t29f * l.f1848) / (t2a0 * t2a0))), (-((t29f * l.f1849) / (t2a0 * t2a0))), );}
        if ((l.f201e != 0.0) && (l.f2024 == 0.0)) {let t2a2: f64 = (l.f1881 * l.f18aa);let t2a3: f64 = (1.0 - l.f1881);let t2a4: f64 = (t2a3 * l.f189f);let t2a5: f64 = (t2a2 + t2a4);(l.f1898, l.f1899, l.f189c, l.f189d, l.f189e, l.f189a, l.f189b, ) = (t2a5, (((l.f1882 * l.f18aa) + (l.f1881 * l.f18ab)) + (((-l.f1882) * l.f189f) + (t2a3 * l.f18a0))), (((l.f1885 * l.f18aa) + (l.f1881 * l.f18ae)) + (((-l.f1885) * l.f189f) + (t2a3 * l.f18a3))), (((l.f1886 * l.f18aa) + (l.f1881 * l.f18af)) + (((-l.f1886) * l.f189f) + (t2a3 * l.f18a4))), (((l.f1887 * l.f18aa) + (l.f1881 * l.f18b0)) + (((-l.f1887) * l.f189f) + (t2a3 * l.f18a5))), (((l.f1883 * l.f18aa) + (l.f1881 * l.f18ac)) + (((-l.f1883) * l.f189f) + (t2a3 * l.f18a1))), (((l.f1884 * l.f18aa) + (l.f1881 * l.f18ad)) + (((-l.f1884) * l.f189f) + (t2a3 * l.f18a2))), );}
        if (l.f201e != 0.0) {
            let t2a6: f64 = (-l.f18dd);
            let (t2b8, t2b9, t2bc, t2bd, t2ba, t2bb,) = {
    if (p.p52 != 0.0) {
        let t2a7: f64 = (l.f18dd / l.f18e4);let t2a8: f64 = (0.001 / p.p53);let t2a9: f64 = (l.f18dd / l.f18e4);let t2aa: f64 = (t2a8 * t2a9);let t2ab: f64 = (t2aa).tanh();let t2ac: f64 = (t2a7 * t2ab);
        (t2ac, (((l.f18de / l.f18e4) * t2ab) + (t2a7 * ((t2a8 * (l.f18de / l.f18e4)) / ((t2aa).cosh() * (t2aa).cosh())))), (((l.f18e1 / l.f18e4) * t2ab) + (t2a7 * ((t2a8 * (l.f18e1 / l.f18e4)) / ((t2aa).cosh() * (t2aa).cosh())))), (((l.f18e2 / l.f18e4) * t2ab) + (t2a7 * ((t2a8 * (l.f18e2 / l.f18e4)) / ((t2aa).cosh() * (t2aa).cosh())))), (((l.f18df / l.f18e4) * t2ab) + (t2a7 * ((t2a8 * (l.f18df / l.f18e4)) / ((t2aa).cosh() * (t2aa).cosh())))), (((l.f18e0 / l.f18e4) * t2ab) + (t2a7 * ((t2a8 * (l.f18e0 / l.f18e4)) / ((t2aa).cosh() * (t2aa).cosh())))),)
    } else {
        let (t2b2, t2b3, t2b6, t2b7, t2b4, t2b5,) = {
            if (p.p52 == 0.0) {
                let __rspice_inv_cse_0: f64 = 1.0 / l.f18e4;let t2ad: f64 = (l.f18dd * __rspice_inv_cse_0);let t2ae: f64 = (l.f18dd * __rspice_inv_cse_0);let t2af: f64 = (t2ad * t2ae);let t2b0: f64 = (t2af + p.p53);let t2b1: f64 = (t2b0).sqrt();
                (t2b1, ((((l.f18de / l.f18e4) * t2ae) + (t2ad * (l.f18de / l.f18e4))) / (2.0 * t2b1)), ((((l.f18e1 / l.f18e4) * t2ae) + (t2ad * (l.f18e1 / l.f18e4))) / (2.0 * t2b1)), ((((l.f18e2 / l.f18e4) * t2ae) + (t2ad * (l.f18e2 / l.f18e4))) / (2.0 * t2b1)), ((((l.f18df / l.f18e4) * t2ae) + (t2ad * (l.f18df / l.f18e4))) / (2.0 * t2b1)), ((((l.f18e0 / l.f18e4) * t2ae) + (t2ad * (l.f18e0 / l.f18e4))) / (2.0 * t2b1)),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (t2b2, t2b3, t2b6, t2b7, t2b4, t2b5,)
    }
};
            let t2be: f64 = (t2b8).powf(l.f182f);let t2bf: f64 = (1.0 + t2be);let t2c0: f64 = (1.0 / l.f182f);let t2c1: f64 = (t2bf).powf(t2c0);let t2c2: f64 = (t2a6 / t2c1);
            (l.f1889, l.f188a, l.f188d, l.f188e, l.f188b, l.f188c, ) = (t2c2, ((((-l.f18de) * t2c1) - (t2a6 * if 0.0 == 0.0 && ((t2c0) as f64).is_finite() && ((t2c0) as f64).fract() == 0.0 { if t2c0 == 0.0 { 0.0 } else { (t2c0 * ((t2bf).powf(t2c0 - 1.0) * if 0.0 == 0.0 && ((l.f182f) as f64).is_finite() && ((l.f182f) as f64).fract() == 0.0 { if l.f182f == 0.0 { 0.0 } else { (l.f182f * ((t2b8).powf(l.f182f - 1.0) * t2b9)) } } else { (t2be * (l.f182f * (t2b9 / t2b8))) })) } } else { (t2c1 * (t2c0 * (if 0.0 == 0.0 && ((l.f182f) as f64).is_finite() && ((l.f182f) as f64).fract() == 0.0 { if l.f182f == 0.0 { 0.0 } else { (l.f182f * ((t2b8).powf(l.f182f - 1.0) * t2b9)) } } else { (t2be * (l.f182f * (t2b9 / t2b8))) } / t2bf))) })) / (t2c1 * t2c1)), ((((-l.f18e1) * t2c1) - (t2a6 * if 0.0 == 0.0 && ((t2c0) as f64).is_finite() && ((t2c0) as f64).fract() == 0.0 { if t2c0 == 0.0 { 0.0 } else { (t2c0 * ((t2bf).powf(t2c0 - 1.0) * if 0.0 == 0.0 && ((l.f182f) as f64).is_finite() && ((l.f182f) as f64).fract() == 0.0 { if l.f182f == 0.0 { 0.0 } else { (l.f182f * ((t2b8).powf(l.f182f - 1.0) * t2bc)) } } else { (t2be * (l.f182f * (t2bc / t2b8))) })) } } else { (t2c1 * (t2c0 * (if 0.0 == 0.0 && ((l.f182f) as f64).is_finite() && ((l.f182f) as f64).fract() == 0.0 { if l.f182f == 0.0 { 0.0 } else { (l.f182f * ((t2b8).powf(l.f182f - 1.0) * t2bc)) } } else { (t2be * (l.f182f * (t2bc / t2b8))) } / t2bf))) })) / (t2c1 * t2c1)), ((((-l.f18e2) * t2c1) - (t2a6 * if 0.0 == 0.0 && ((t2c0) as f64).is_finite() && ((t2c0) as f64).fract() == 0.0 { if t2c0 == 0.0 { 0.0 } else { (t2c0 * ((t2bf).powf(t2c0 - 1.0) * if 0.0 == 0.0 && ((l.f182f) as f64).is_finite() && ((l.f182f) as f64).fract() == 0.0 { if l.f182f == 0.0 { 0.0 } else { (l.f182f * ((t2b8).powf(l.f182f - 1.0) * t2bd)) } } else { (t2be * (l.f182f * (t2bd / t2b8))) })) } } else { (t2c1 * (t2c0 * (if 0.0 == 0.0 && ((l.f182f) as f64).is_finite() && ((l.f182f) as f64).fract() == 0.0 { if l.f182f == 0.0 { 0.0 } else { (l.f182f * ((t2b8).powf(l.f182f - 1.0) * t2bd)) } } else { (t2be * (l.f182f * (t2bd / t2b8))) } / t2bf))) })) / (t2c1 * t2c1)), ((((-l.f18df) * t2c1) - (t2a6 * if 0.0 == 0.0 && ((t2c0) as f64).is_finite() && ((t2c0) as f64).fract() == 0.0 { if t2c0 == 0.0 { 0.0 } else { (t2c0 * ((t2bf).powf(t2c0 - 1.0) * if 0.0 == 0.0 && ((l.f182f) as f64).is_finite() && ((l.f182f) as f64).fract() == 0.0 { if l.f182f == 0.0 { 0.0 } else { (l.f182f * ((t2b8).powf(l.f182f - 1.0) * t2ba)) } } else { (t2be * (l.f182f * (t2ba / t2b8))) })) } } else { (t2c1 * (t2c0 * (if 0.0 == 0.0 && ((l.f182f) as f64).is_finite() && ((l.f182f) as f64).fract() == 0.0 { if l.f182f == 0.0 { 0.0 } else { (l.f182f * ((t2b8).powf(l.f182f - 1.0) * t2ba)) } } else { (t2be * (l.f182f * (t2ba / t2b8))) } / t2bf))) })) / (t2c1 * t2c1)), ((((-l.f18e0) * t2c1) - (t2a6 * if 0.0 == 0.0 && ((t2c0) as f64).is_finite() && ((t2c0) as f64).fract() == 0.0 { if t2c0 == 0.0 { 0.0 } else { (t2c0 * ((t2bf).powf(t2c0 - 1.0) * if 0.0 == 0.0 && ((l.f182f) as f64).is_finite() && ((l.f182f) as f64).fract() == 0.0 { if l.f182f == 0.0 { 0.0 } else { (l.f182f * ((t2b8).powf(l.f182f - 1.0) * t2bb)) } } else { (t2be * (l.f182f * (t2bb / t2b8))) })) } } else { (t2c1 * (t2c0 * (if 0.0 == 0.0 && ((l.f182f) as f64).is_finite() && ((l.f182f) as f64).fract() == 0.0 { if l.f182f == 0.0 { 0.0 } else { (l.f182f * ((t2b8).powf(l.f182f - 1.0) * t2bb)) } } else { (t2be * (l.f182f * (t2bb / t2b8))) } / t2bf))) })) / (t2c1 * t2c1)), );
        }
        if (l.f201e != 0.0) {let t2c3: f64 = (-l.f18db);let t2c4: f64 = (t2c3 * l.f18e6);let t2c5: f64 = (t2c4 * l.f18c8);let t2c6: f64 = (t2c5 * l.f18c2);let t2c7: f64 = (t2c6 * l.f18d9);let t2c8: f64 = t2c7;(l.f18c5, l.f18c6, ) = (t2c8, (t2c6 * l.f18da), );let t2c9: f64 = (l.f18cd / l.f18ce);let t2ca: f64 = (t2c9 * l.f1889);(l.f1878, l.f1879, l.f187c, l.f187d, l.f187e, l.f187a, l.f187b, ) = (t2ca, (t2c9 * l.f188a), (t2c9 * l.f188d), ((-((l.f18cd * l.f18cf) / (l.f18ce * l.f18ce))) * l.f1889), (t2c9 * l.f188e), (t2c9 * l.f188b), (t2c9 * l.f188c), );}
        if (l.f201e != 0.0) {
            let t2cb: f64 = (-50.0);
            let (t2e2, t2e3, t2e6, t2e7, t2e8, t2e4, t2e5,) = {
    if ((!(l.f1878 > 50.0)) && (!(l.f1878 < t2cb))) {
        let t2cc: f64 = (l.f1878).exp();
        (t2cc, (t2cc * l.f1879), (t2cc * l.f187c), (t2cc * l.f187d), (t2cc * l.f187e), (t2cc * l.f187a), (t2cc * l.f187b),)
    } else {
        let t2cd: f64 = (-50.0);
        let (t2db, t2dc, t2df, t2e0, t2e1, t2dd, t2de,) = {
            if ((!(l.f1878 > 50.0)) && (l.f1878 < t2cd)) {
                let t2ce: f64 = (-50.0);let t2cf: f64 = (t2ce).exp();
                (t2cf, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let (t2d4, t2d5, t2d8, t2d9, t2da, t2d6, t2d7,) = {
                    if (l.f1878 > 50.0) {
                        let t2d0: f64 = (50.0_f64).exp();let t2d1: f64 = (l.f1878 - 50.0);let t2d2: f64 = (1.0 + t2d1);let t2d3: f64 = (t2d0 * t2d2);
                        (t2d3, (t2d0 * l.f1879), (t2d0 * l.f187c), (t2d0 * l.f187d), (t2d0 * l.f187e), (t2d0 * l.f187a), (t2d0 * l.f187b),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (t2d4, t2d5, t2d8, t2d9, t2da, t2d6, t2d7,)
            }
        };
        (t2db, t2dc, t2df, t2e0, t2e1, t2dd, t2de,)
    }
};
            (l.f1871, l.f1872, l.f1875, l.f1876, l.f1877, l.f1873, l.f1874, ) = (t2e2, t2e3, t2e6, t2e7, t2e8, t2e4, t2e5, );
        }
        if (l.f201e != 0.0) {let t2e9: f64 = (l.f1871 - 1.0);let t2ea: f64 = (l.f18c5 * t2e9);(l.f18b3, l.f18b4, l.f18b7, l.f18b8, l.f18b9, l.f18b5, l.f18b6, ) = (t2ea, (l.f18c5 * l.f1872), (l.f18c5 * l.f1875), ((l.f18c6 * t2e9) + (l.f18c5 * l.f1876)), (l.f18c5 * l.f1877), (l.f18c5 * l.f1873), (l.f18c5 * l.f1874), );let t2eb: f64 = (l.f1898 + l.f18b3);(l.f18ba, l.f18bb, l.f18be, l.f18bf, l.f18c0, l.f18bc, l.f18bd, ) = (t2eb, (l.f1899 + l.f18b4), (l.f189c + l.f18b7), (l.f189d + l.f18b8), (l.f189e + l.f18b9), (l.f189a + l.f18b5), (l.f189b + l.f18b6), );(l.f18d0, l.f18d1, l.f18d4, l.f18d5, l.f18d6, l.f18d2, l.f18d3, ) = (l.f18ba, l.f18bb, l.f18be, l.f18bf, l.f18c0, l.f18bc, l.f18bd, );}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_151(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv2 = ctx.node_voltage(nodes[2]);let nv6 = ctx.node_voltage(nodes[6]);
        if (l.f201e != 0.0) {(l.f2115, l.f2116, l.f2119, l.f211a, l.f211b, l.f2117, l.f2118, ) = (l.f18d0, l.f18d1, l.f18d4, l.f18d5, l.f18d6, l.f18d2, l.f18d3, );}
        let t2f5: f64 = if p.p313 == 0.0 { 1.0 } else { 0.0 };l.f202a = t2f5;let t2f6: f64 = if ((l.f22e4 >= p.p353) && (l.f22e4 > 0.0)) { 1.0 } else { 0.0 };l.f202b = t2f6;let t2f7: f64 = if ((l.f22e5 >= p.p353) && (l.f22e5 > 0.0)) { 1.0 } else { 0.0 };l.f202c = t2f7;let t2f8: f64 = if ((l.f22e8 >= p.p353) && (l.f22e8 > 0.0)) { 1.0 } else { 0.0 };l.f202d = t2f8;let t2f9: f64 = if ((l.f22e9 >= p.p353) && (l.f22e9 > 0.0)) { 1.0 } else { 0.0 };l.f202e = t2f9;let t2fa: f64 = ((nv6 - nv2) - p.p27);let t2fb: f64 = (t2fa / p.p28);let t2fc: f64 = if t2fb > 50.0 { 1.0 } else { 0.0 };l.f202f = t2fc;
        if (l.f202f != 0.0) {let t2fd: f64 = (p.p0 * p.p2);let t2fe: f64 = (l.f72 * (nv6 - nv2));let t2ff: f64 = ((nv6 - nv2) - p.p27);let t300: f64 = (l.f71 * t2ff);let t301: f64 = (t2fe + t300);let t302: f64 = (t2fd * t301);(l.f2276, l.f2277, l.f2278, l.f2279, ) = (t302, (t2fd * ((-l.f72) + (-l.f71))), (t2fd * ((l.f73 * (nv6 - nv2)) + (l.f75 * t2ff))), (t2fd * (l.f72 + l.f71)), );}
        let t303: f64 = ((nv6 - nv2) - p.p27);let t304: f64 = (t303 / p.p28);let t305: f64 = (-50.0);let t306: f64 = if t304 < t305 { 1.0 } else { 0.0 };l.f2031 = t306;
        if ((l.f202f == 0.0) && (l.f2031 != 0.0)) {let t307: f64 = (p.p0 * p.p2);let t308: f64 = (l.f72 * (nv6 - nv2));let t309: f64 = (l.f71 * p.p28);let t30a: f64 = ((nv6 - nv2) - p.p27);let t30b: f64 = (t30a / p.p28);let t30c: f64 = (t30b).exp();let t30d: f64 = (t309 * t30c);let t30e: f64 = (t308 + t30d);let t30f: f64 = (t307 * t30e);(l.f2276, l.f2277, l.f2278, l.f2279, ) = (t30f, (t307 * ((-l.f72) + (t309 * (t30c * (-1.0 / p.p28))))), (t307 * ((l.f73 * (nv6 - nv2)) + ((l.f75 * p.p28) * t30c))), (t307 * (l.f72 + (t309 * (t30c * (1.0 / p.p28))))), );}
        if ((l.f202f == 0.0) && (l.f2031 == 0.0)) {let t310: f64 = (p.p0 * p.p2);let t311: f64 = (l.f72 * (nv6 - nv2));let t312: f64 = (l.f71 * p.p28);let t313: f64 = ((nv6 - nv2) - p.p27);let t314: f64 = (t313 / p.p28);let t315: f64 = (t314).exp();let t316: f64 = (1.0 + t315);let t317: f64 = (t316).ln();let t318: f64 = (t312 * t317);let t319: f64 = (t311 + t318);let t31a: f64 = (t310 * t319);(l.f2276, l.f2277, l.f2278, l.f2279, ) = (t31a, (t310 * ((-l.f72) + (t312 * ((t315 * (-1.0 / p.p28)) / t316)))), (t310 * ((l.f73 * (nv6 - nv2)) + ((l.f75 * p.p28) * t317))), (t310 * (l.f72 + (t312 * ((t315 * (1.0 / p.p28)) / t316)))), );}
        let t31b: f64 = ((nv6 - nv0) - p.p27);let t31c: f64 = (t31b / p.p28);let t31d: f64 = if t31c > 50.0 { 1.0 } else { 0.0 };l.f2033 = t31d;
        if (l.f2033 != 0.0) {let t31e: f64 = (p.p0 * p.p2);let t31f: f64 = (l.f5a * (nv6 - nv0));let t320: f64 = ((nv6 - nv0) - p.p27);let t321: f64 = (l.f59 * t320);let t322: f64 = (t31f + t321);let t323: f64 = (t31e * t322);(l.f2262, l.f2263, l.f2264, l.f2265, ) = (t323, (t31e * ((-l.f5a) + (-l.f59))), (t31e * ((l.f5b * (nv6 - nv0)) + (l.f5d * t320))), (t31e * (l.f5a + l.f59)), );}
        let t324: f64 = ((nv6 - nv0) - p.p27);let t325: f64 = (t324 / p.p28);let t326: f64 = (-50.0);let t327: f64 = if t325 < t326 { 1.0 } else { 0.0 };l.f2038 = t327;
        if ((l.f2033 == 0.0) && (l.f2038 != 0.0)) {let t328: f64 = (p.p0 * p.p2);let t329: f64 = (l.f5a * (nv6 - nv0));let t32a: f64 = (l.f59 * p.p28);let t32b: f64 = ((nv6 - nv0) - p.p27);let t32c: f64 = (t32b / p.p28);let t32d: f64 = (t32c).exp();let t32e: f64 = (t32a * t32d);let t32f: f64 = (t329 + t32e);let t330: f64 = (t328 * t32f);(l.f2262, l.f2263, l.f2264, l.f2265, ) = (t330, (t328 * ((-l.f5a) + (t32a * (t32d * (-1.0 / p.p28))))), (t328 * ((l.f5b * (nv6 - nv0)) + ((l.f5d * p.p28) * t32d))), (t328 * (l.f5a + (t32a * (t32d * (1.0 / p.p28))))), );}
        if ((l.f2033 == 0.0) && (l.f2038 == 0.0)) {let t331: f64 = (p.p0 * p.p2);let t332: f64 = (l.f5a * (nv6 - nv0));let t333: f64 = (l.f59 * p.p28);let t334: f64 = ((nv6 - nv0) - p.p27);let t335: f64 = (t334 / p.p28);let t336: f64 = (t335).exp();let t337: f64 = (1.0 + t336);let t338: f64 = (t337).ln();let t339: f64 = (t333 * t338);let t33a: f64 = (t332 + t339);let t33b: f64 = (t331 * t33a);(l.f2262, l.f2263, l.f2264, l.f2265, ) = (t33b, (t331 * ((-l.f5a) + (t333 * ((t336 * (-1.0 / p.p28)) / t337)))), (t331 * ((l.f5b * (nv6 - nv0)) + ((l.f5d * p.p28) * t338))), (t331 * (l.f5a + (t333 * ((t336 * (1.0 / p.p28)) / t337)))), );}
        let t33c: f64 = ((nv2 - nv0) - p.p27);let t33d: f64 = (t33c / p.p28);let t33e: f64 = if t33d > 50.0 { 1.0 } else { 0.0 };l.f203a = t33e;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_152(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv2 = ctx.node_voltage(nodes[2]);let nv3 = ctx.node_voltage(nodes[3]);
        if (l.f203a != 0.0) {let t33f: f64 = (p.p0 * p.p2);let t340: f64 = (l.f60 * (nv2 - nv0));let t341: f64 = ((nv2 - nv0) - p.p27);let t342: f64 = (l.f5f * t341);let t343: f64 = (t340 + t342);let t344: f64 = (t33f * t343);(l.f2267, l.f2268, l.f2269, l.f226a, ) = (t344, (t33f * ((-l.f60) + (-l.f5f))), (t33f * (l.f60 + l.f5f)), (t33f * ((l.f61 * (nv2 - nv0)) + (l.f63 * t341))), );}
        let t345: f64 = ((nv2 - nv0) - p.p27);let t346: f64 = (t345 / p.p28);let t347: f64 = (-50.0);let t348: f64 = if t346 < t347 { 1.0 } else { 0.0 };l.f203c = t348;
        if ((l.f203a == 0.0) && (l.f203c != 0.0)) {let t349: f64 = (p.p0 * p.p2);let t34a: f64 = (l.f60 * (nv2 - nv0));let t34b: f64 = (l.f5f * p.p28);let t34c: f64 = ((nv2 - nv0) - p.p27);let t34d: f64 = (t34c / p.p28);let t34e: f64 = (t34d).exp();let t34f: f64 = (t34b * t34e);let t350: f64 = (t34a + t34f);let t351: f64 = (t349 * t350);(l.f2267, l.f2268, l.f2269, l.f226a, ) = (t351, (t349 * ((-l.f60) + (t34b * (t34e * (-1.0 / p.p28))))), (t349 * (l.f60 + (t34b * (t34e * (1.0 / p.p28))))), (t349 * ((l.f61 * (nv2 - nv0)) + ((l.f63 * p.p28) * t34e))), );}
        if ((l.f203a == 0.0) && (l.f203c == 0.0)) {let t352: f64 = (p.p0 * p.p2);let t353: f64 = (l.f60 * (nv2 - nv0));let t354: f64 = (l.f5f * p.p28);let t355: f64 = ((nv2 - nv0) - p.p27);let t356: f64 = (t355 / p.p28);let t357: f64 = (t356).exp();let t358: f64 = (1.0 + t357);let t359: f64 = (t358).ln();let t35a: f64 = (t354 * t359);let t35b: f64 = (t353 + t35a);let t35c: f64 = (t352 * t35b);(l.f2267, l.f2268, l.f2269, l.f226a, ) = (t35c, (t352 * ((-l.f60) + (t354 * ((t357 * (-1.0 / p.p28)) / t358)))), (t352 * (l.f60 + (t354 * ((t357 * (1.0 / p.p28)) / t358)))), (t352 * ((l.f61 * (nv2 - nv0)) + ((l.f63 * p.p28) * t359))), );}
        let t35d: f64 = ((nv3 - nv2) - p.p27);let t35e: f64 = (t35d / p.p28);let t35f: f64 = if t35e > 50.0 { 1.0 } else { 0.0 };l.f203e = t35f;
        if (l.f203e != 0.0) {let t360: f64 = (p.p0 * p.p2);let t361: f64 = (l.f78 * (nv3 - nv2));let t362: f64 = ((nv3 - nv2) - p.p27);let t363: f64 = (l.f77 * t362);let t364: f64 = (t361 + t363);let t365: f64 = (t360 * t364);(l.f227b, l.f227c, l.f227d, l.f227e, ) = (t365, (t360 * ((-l.f78) + (-l.f77))), (t360 * (l.f78 + l.f77)), (t360 * ((l.f79 * (nv3 - nv2)) + (l.f7b * t362))), );}
        let t366: f64 = ((nv3 - nv2) - p.p27);let t367: f64 = (t366 / p.p28);let t368: f64 = (-50.0);let t369: f64 = if t367 < t368 { 1.0 } else { 0.0 };l.f2040 = t369;
        if ((l.f203e == 0.0) && (l.f2040 != 0.0)) {let t36a: f64 = (p.p0 * p.p2);let t36b: f64 = (l.f78 * (nv3 - nv2));let t36c: f64 = (l.f77 * p.p28);let t36d: f64 = ((nv3 - nv2) - p.p27);let t36e: f64 = (t36d / p.p28);let t36f: f64 = (t36e).exp();let t370: f64 = (t36c * t36f);let t371: f64 = (t36b + t370);let t372: f64 = (t36a * t371);(l.f227b, l.f227c, l.f227d, l.f227e, ) = (t372, (t36a * ((-l.f78) + (t36c * (t36f * (-1.0 / p.p28))))), (t36a * (l.f78 + (t36c * (t36f * (1.0 / p.p28))))), (t36a * ((l.f79 * (nv3 - nv2)) + ((l.f7b * p.p28) * t36f))), );}
        if ((l.f203e == 0.0) && (l.f2040 == 0.0)) {let t373: f64 = (p.p0 * p.p2);let t374: f64 = (l.f78 * (nv3 - nv2));let t375: f64 = (l.f77 * p.p28);let t376: f64 = ((nv3 - nv2) - p.p27);let t377: f64 = (t376 / p.p28);let t378: f64 = (t377).exp();let t379: f64 = (1.0 + t378);let t37a: f64 = (t379).ln();let t37b: f64 = (t375 * t37a);let t37c: f64 = (t374 + t37b);let t37d: f64 = (t373 * t37c);(l.f227b, l.f227c, l.f227d, l.f227e, ) = (t37d, (t373 * ((-l.f78) + (t375 * ((t378 * (-1.0 / p.p28)) / t379)))), (t373 * (l.f78 + (t375 * ((t378 * (1.0 / p.p28)) / t379)))), (t373 * ((l.f79 * (nv3 - nv2)) + ((l.f7b * p.p28) * t37a))), );}
        let t37e: f64 = ((nv3 - nv0) - p.p27);let t37f: f64 = (t37e / p.p28);let t380: f64 = if t37f > 50.0 { 1.0 } else { 0.0 };l.f2042 = t380;
        if (l.f2042 != 0.0) {let t381: f64 = (p.p0 * p.p2);let t382: f64 = (l.f66 * (nv3 - nv0));let t383: f64 = ((nv3 - nv0) - p.p27);let t384: f64 = (l.f65 * t383);let t385: f64 = (t382 + t384);let t386: f64 = (t381 * t385);(l.f226c, l.f226d, l.f226e, l.f226f, ) = (t386, (t381 * ((-l.f66) + (-l.f65))), (t381 * (l.f66 + l.f65)), (t381 * ((l.f67 * (nv3 - nv0)) + (l.f69 * t383))), );}
        let t387: f64 = ((nv3 - nv0) - p.p27);let t388: f64 = (t387 / p.p28);let t389: f64 = (-50.0);let t38a: f64 = if t388 < t389 { 1.0 } else { 0.0 };l.f2044 = t38a;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_153(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv3 = ctx.node_voltage(nodes[3]);let nv6 = ctx.node_voltage(nodes[6]);
        if ((l.f2042 == 0.0) && (l.f2044 != 0.0)) {let t38b: f64 = (p.p0 * p.p2);let t38c: f64 = (l.f66 * (nv3 - nv0));let t38d: f64 = (l.f65 * p.p28);let t38e: f64 = ((nv3 - nv0) - p.p27);let t38f: f64 = (t38e / p.p28);let t390: f64 = (t38f).exp();let t391: f64 = (t38d * t390);let t392: f64 = (t38c + t391);let t393: f64 = (t38b * t392);(l.f226c, l.f226d, l.f226e, l.f226f, ) = (t393, (t38b * ((-l.f66) + (t38d * (t390 * (-1.0 / p.p28))))), (t38b * (l.f66 + (t38d * (t390 * (1.0 / p.p28))))), (t38b * ((l.f67 * (nv3 - nv0)) + ((l.f69 * p.p28) * t390))), );}
        if ((l.f2042 == 0.0) && (l.f2044 == 0.0)) {let t394: f64 = (p.p0 * p.p2);let t395: f64 = (l.f66 * (nv3 - nv0));let t396: f64 = (l.f65 * p.p28);let t397: f64 = ((nv3 - nv0) - p.p27);let t398: f64 = (t397 / p.p28);let t399: f64 = (t398).exp();let t39a: f64 = (1.0 + t399);let t39b: f64 = (t39a).ln();let t39c: f64 = (t396 * t39b);let t39d: f64 = (t395 + t39c);let t39e: f64 = (t394 * t39d);(l.f226c, l.f226d, l.f226e, l.f226f, ) = (t39e, (t394 * ((-l.f66) + (t396 * ((t399 * (-1.0 / p.p28)) / t39a)))), (t394 * (l.f66 + (t396 * ((t399 * (1.0 / p.p28)) / t39a)))), (t394 * ((l.f67 * (nv3 - nv0)) + ((l.f69 * p.p28) * t39b))), );}
        let t39f: f64 = ((nv6 - nv3) - p.p27);let t3a0: f64 = (t39f / p.p28);let t3a1: f64 = if t3a0 > 50.0 { 1.0 } else { 0.0 };l.f2046 = t3a1;
        if (l.f2046 != 0.0) {let t3a2: f64 = (p.p0 * p.p2);let t3a3: f64 = (l.f6c * (nv6 - nv3));let t3a4: f64 = ((nv6 - nv3) - p.p27);let t3a5: f64 = (l.f6b * t3a4);let t3a6: f64 = (t3a3 + t3a5);let t3a7: f64 = (t3a2 * t3a6);(l.f2271, l.f2272, l.f2273, l.f2274, ) = (t3a7, (t3a2 * ((-l.f6c) + (-l.f6b))), (t3a2 * ((l.f6d * (nv6 - nv3)) + (l.f6f * t3a4))), (t3a2 * (l.f6c + l.f6b)), );}
        let t3a8: f64 = ((nv6 - nv3) - p.p27);let t3a9: f64 = (t3a8 / p.p28);let t3aa: f64 = (-50.0);let t3ab: f64 = if t3a9 < t3aa { 1.0 } else { 0.0 };l.f2048 = t3ab;
        if ((l.f2046 == 0.0) && (l.f2048 != 0.0)) {let t3ac: f64 = (p.p0 * p.p2);let t3ad: f64 = (l.f6c * (nv6 - nv3));let t3ae: f64 = (l.f6b * p.p28);let t3af: f64 = ((nv6 - nv3) - p.p27);let t3b0: f64 = (t3af / p.p28);let t3b1: f64 = (t3b0).exp();let t3b2: f64 = (t3ae * t3b1);let t3b3: f64 = (t3ad + t3b2);let t3b4: f64 = (t3ac * t3b3);(l.f2271, l.f2272, l.f2273, l.f2274, ) = (t3b4, (t3ac * ((-l.f6c) + (t3ae * (t3b1 * (-1.0 / p.p28))))), (t3ac * ((l.f6d * (nv6 - nv3)) + ((l.f6f * p.p28) * t3b1))), (t3ac * (l.f6c + (t3ae * (t3b1 * (1.0 / p.p28))))), );}
        if ((l.f2046 == 0.0) && (l.f2048 == 0.0)) {let t3b5: f64 = (p.p0 * p.p2);let t3b6: f64 = (l.f6c * (nv6 - nv3));let t3b7: f64 = (l.f6b * p.p28);let t3b8: f64 = ((nv6 - nv3) - p.p27);let t3b9: f64 = (t3b8 / p.p28);let t3ba: f64 = (t3b9).exp();let t3bb: f64 = (1.0 + t3ba);let t3bc: f64 = (t3bb).ln();let t3bd: f64 = (t3b7 * t3bc);let t3be: f64 = (t3b6 + t3bd);let t3bf: f64 = (t3b5 * t3be);(l.f2271, l.f2272, l.f2273, l.f2274, ) = (t3bf, (t3b5 * ((-l.f6c) + (t3b7 * ((t3ba * (-1.0 / p.p28)) / t3bb)))), (t3b5 * ((l.f6d * (nv6 - nv3)) + ((l.f6f * p.p28) * t3bc))), (t3b5 * (l.f6c + (t3b7 * ((t3ba * (1.0 / p.p28)) / t3bb)))), );}
        let t3c0: f64 = if p.p347 == 1.0 { 1.0 } else { 0.0 };l.f204a = t3c0;let t3c1: f64 = if ((p.p79 > p.p354) && (p.p29 != 0.0)) { 1.0 } else { 0.0 };l.f204d = t3c1;let t3c2: f64 = if ((p.p101 > p.p354) && (p.p29 != 0.0)) { 1.0 } else { 0.0 };l.f204e = t3c2;let t3c3: f64 = if ((p.p123 > p.p354) && (p.p29 != 0.0)) { 1.0 } else { 0.0 };l.f204f = t3c3;let t3c4: f64 = if ((p.p145 > p.p354) && (p.p29 != 0.0)) { 1.0 } else { 0.0 };l.f2050 = t3c4;let t3c5: f64 = if ((p.p167 > p.p354) && (p.p29 != 0.0)) { 1.0 } else { 0.0 };l.f2051 = t3c5;let t3c6: f64 = if ((p.p189 > p.p354) && (p.p29 != 0.0)) { 1.0 } else { 0.0 };l.f2052 = t3c6;let t3c7: f64 = if ((p.p211 > p.p354) && (p.p29 != 0.0)) { 1.0 } else { 0.0 };l.f2053 = t3c7;let t3c8: f64 = if ((p.p233 > p.p354) && (p.p29 != 0.0)) { 1.0 } else { 0.0 };l.f2054 = t3c8;let t3c9: f64 = if ((l.f22e5 >= p.p353) && (l.f22e5 > 0.0)) { 1.0 } else { 0.0 };l.f2055 = t3c9;let t3ca: f64 = if ((l.f22e4 >= p.p353) && (l.f22e4 > 0.0)) { 1.0 } else { 0.0 };l.f2058 = t3ca;
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_154(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);let nv2 = ctx.node_voltage(nodes[2]);let nv5 = ctx.node_voltage(nodes[5]);let nv9 = ctx.node_voltage(nodes[9]);let nv10 = ctx.node_voltage(nodes[10]);let nv11 = ctx.node_voltage(nodes[11]);let nv12 = ctx.node_voltage(nodes[12]);let nv13 = ctx.node_voltage(nodes[13]);let nv14 = ctx.node_voltage(nodes[14]);let nv15 = ctx.node_voltage(nodes[15]);let nv16 = ctx.node_voltage(nodes[16]);let nv17 = ctx.node_voltage(nodes[17]);let nv18 = ctx.node_voltage(nodes[18]);let nv19 = ctx.node_voltage(nodes[19]);let t3cb: f64 = (l.f20bc * (nv5 - nv9));let t3cc: f64 = (l.f2106 * (nv18 - nv17));let t3cd: f64 = (t3cb + t3cc);let t3ce: f64 = (l.f210f * (nv13 - nv19));let t3cf: f64 = (t3cd + t3ce);let t3d0: f64 = (l.f20ff * (nv12 - nv13));let t3d1: f64 = (t3cf + t3d0);let t3d2: f64 = (l.f20f8 * (nv11 - nv12));let t3d3: f64 = (t3d1 + t3d2);let t3d4: f64 = (l.f20f1 * (nv10 - nv11));let t3d5: f64 = (t3d3 + t3d4);let t3d6: f64 = (l.f20ea * (nv9 - nv10));let t3d7: f64 = (t3d5 + t3d6);let t3d8: f64 = (l.f20ce * (nv14 - nv5));let t3d9: f64 = (t3d7 + t3d8);let t3da: f64 = (l.f20d5 * (nv15 - nv14));let t3db: f64 = (t3d9 + t3da);let t3dc: f64 = (l.f20dc * (nv16 - nv15));let t3dd: f64 = (t3db + t3dc);let t3de: f64 = (l.f20e3 * (nv17 - nv16));let t3df: f64 = (t3dd + t3de);(l.f2143, l.f2144, l.f214f, l.f2155, l.f2156, l.f2157, l.f2158, l.f2159, l.f215a, l.f2145, l.f2146, l.f2147, l.f2148, l.f2149, l.f214a, l.f214b, ) = (t3df, ((l.f2107 * (nv18 - nv17)) + (l.f2110 * (nv13 - nv19))), ((((((((((l.f210a * (nv18 - nv17)) + (l.f2113 * (nv13 - nv19))) + (l.f2102 * (nv12 - nv13))) + (l.f20fb * (nv11 - nv12))) + (l.f20f4 * (nv10 - nv11))) + (l.f20ec * (nv9 - nv10))) + (l.f20d0 * (nv14 - nv5))) + (l.f20d8 * (nv15 - nv14))) + (l.f20df * (nv16 - nv15))) + (l.f20e6 * (nv17 - nv16))), ((((((((l.f2103 * (nv12 - nv13)) + (l.f20fc * (nv11 - nv12))) + (l.f20f5 * (nv10 - nv11))) + (l.f20ed * (nv9 - nv10))) + (l.f20d1 * (nv14 - nv5))) + (l.f20d9 * (nv15 - nv14))) + (l.f20e0 * (nv16 - nv15))) + (l.f20e7 * (nv17 - nv16))), (((((((((((l.f20c1 * (nv5 - nv9)) + (l.f210c * (nv18 - nv17))) + (l.f2114 * (nv13 - nv19))) + (l.f2104 * (nv12 - nv13))) + (l.f20fd * (nv11 - nv12))) + (l.f20f6 * (nv10 - nv11))) + (l.f20ee * (nv9 - nv10))) + (l.f20d2 * (nv14 - nv5))) + (l.f20da * (nv15 - nv14))) + (l.f20e1 * (nv16 - nv15))) + (l.f20e8 * (nv17 - nv16))), (((l.f20c2 * (nv5 - nv9)) + l.f20bc) + ((l.f20d3 * (nv14 - nv5)) + (-l.f20ce))), ((((((((l.f2105 * (nv12 - nv13)) + (l.f20fe * (nv11 - nv12))) + (l.f20f7 * (nv10 - nv11))) + (l.f20ef * (nv9 - nv10))) + (l.f20d4 * (nv14 - nv5))) + (l.f20db * (nv15 - nv14))) + (l.f20e2 * (nv16 - nv15))) + (l.f20e9 * (nv17 - nv16))), (l.f20c3 * (nv5 - nv9)), (((l.f20c4 * (nv5 - nv9)) + (-l.f20bc)) + ((l.f20f0 * (nv9 - nv10)) + l.f20ea)), (((l.f20f2 * (nv10 - nv11)) + l.f20f1) + ((l.f20eb * (nv9 - nv10)) + (-l.f20ea))), (((l.f20f9 * (nv11 - nv12)) + l.f20f8) + ((l.f20f3 * (nv10 - nv11)) + (-l.f20f1))), (((l.f2100 * (nv12 - nv13)) + l.f20ff) + ((l.f20fa * (nv11 - nv12)) + (-l.f20f8))), (((l.f2111 * (nv13 - nv19)) + l.f210f) + ((l.f2101 * (nv12 - nv13)) + (-l.f20ff))), (((l.f20cf * (nv14 - nv5)) + l.f20ce) + ((l.f20d6 * (nv15 - nv14)) + (-l.f20d5))), (((l.f20d7 * (nv15 - nv14)) + l.f20d5) + ((l.f20dd * (nv16 - nv15)) + (-l.f20dc))), (((l.f20de * (nv16 - nv15)) + l.f20dc) + ((l.f20e4 * (nv17 - nv16)) + (-l.f20e3))), );
        (l.f214c, l.f214d, l.f214e, l.f2150, l.f2151, l.f2152, l.f2153, l.f2154, ) = ((((l.f2108 * (nv18 - nv17)) + (-l.f2106)) + ((l.f20e5 * (nv17 - nv16)) + l.f20e3)), ((l.f2109 * (nv18 - nv17)) + l.f2106), ((l.f2112 * (nv13 - nv19)) + (-l.f210f)), (l.f210b * (nv18 - nv17)), (l.f20bd * (nv5 - nv9)), (l.f20be * (nv5 - nv9)), (l.f20bf * (nv5 - nv9)), (l.f20c0 * (nv5 - nv9)), );let t3e0: f64 = if ((l.f22e4 >= p.p353) && (l.f22e4 > 0.0)) { 1.0 } else { 0.0 };l.f2059 = t3e0;
        if (l.f2059 != 0.0) {let t3e1: f64 = ((nv18 - nv0) * (nv18 - nv0));let t3e2: f64 = (t3e1 / l.f22e6);let t3e3: f64 = (l.f2143 + t3e2);(l.f2143, l.f2144, l.f214f, l.f2155, l.f2156, l.f2157, l.f2158, l.f2159, l.f215a, l.f2145, l.f2146, l.f2147, l.f2148, l.f2149, l.f214a, l.f214b, ) = (t3e3, (l.f2144 + (((-(nv18 - nv0)) + (-(nv18 - nv0))) / l.f22e6)), l.f214f, l.f2155, (l.f2156 + (-((t3e1 * l.f22e7) / (l.f22e6 * l.f22e6)))), l.f2157, l.f2158, l.f2159, l.f215a, l.f2145, l.f2146, l.f2147, l.f2148, l.f2149, l.f214a, l.f214b, );(l.f214c, l.f214d, l.f214e, l.f2150, l.f2151, l.f2152, l.f2153, l.f2154, ) = (l.f214c, (l.f214d + (((nv18 - nv0) + (nv18 - nv0)) / l.f22e6)), l.f214e, l.f2150, l.f2151, l.f2152, l.f2153, l.f2154, );}
        let t3e4: f64 = if ((l.f22e5 >= p.p353) && (l.f22e5 > 0.0)) { 1.0 } else { 0.0 };l.f205a = t3e4;
        if (l.f205a != 0.0) {let t3e5: f64 = ((nv19 - nv2) * (nv19 - nv2));let t3e6: f64 = (t3e5 / l.f22eb);let t3e7: f64 = (l.f2143 + t3e6);(l.f2143, l.f2144, l.f214f, l.f2155, l.f2156, l.f2157, l.f2158, l.f2159, l.f215a, l.f2145, l.f2146, l.f2147, l.f2148, l.f2149, l.f214a, l.f214b, ) = (t3e7, l.f2144, (l.f214f + (((-(nv19 - nv2)) + (-(nv19 - nv2))) / l.f22eb)), l.f2155, (l.f2156 + (-((t3e5 * l.f22ec) / (l.f22eb * l.f22eb)))), l.f2157, l.f2158, l.f2159, l.f215a, l.f2145, l.f2146, l.f2147, l.f2148, l.f2149, l.f214a, l.f214b, );(l.f214c, l.f214d, l.f214e, l.f2150, l.f2151, l.f2152, l.f2153, l.f2154, ) = (l.f214c, l.f214d, (l.f214e + (((nv19 - nv2) + (nv19 - nv2)) / l.f22eb)), l.f2150, l.f2151, l.f2152, l.f2153, l.f2154, );}
        let t3e8: f64 = if p.p320 > 0.0 { 1.0 } else { 0.0 };l.f205b = t3e8;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();let nv4 = ctx.node_voltage(nodes[4]);let t6: f64 = (p.p5 + 273.15);l.f22f4 = t6;l.f22f5 = 0.0;let t9: f64 = ctx_temp;l.f22ed = t9;l.f22ee = 0.0;(l.f22f6, l.f22f7, ) = ((nv4 - 0.0), 1.0, );l.f22f8 = 0.0;let t40d: f64 = (l.f22ed + p.p3);let t40e: f64 = (t40d + l.f22f6);(l.f22ef, l.f22f0, ) = (t40e, l.f22f7, );l.f22f1 = 0.0;let t469: f64 = (-270.0);let t46a: f64 = (t469 + 273.15);let t46b: f64 = if l.f22ef < t46a { 1.0 } else { 0.0 };l.f1e5c = t46b;l.f1f31 = 0.0;
        if (l.f1e5c != 0.0) {let t4c6: f64 = (-270.0);let t4c7: f64 = (t4c6 + 273.15);(l.f22ef, l.f22f0, ) = (t4c7, 0.0, );l.f22f1 = 0.0;}
        let t4ca: f64 = (1500.0 + 273.15);let t4cb: f64 = if l.f22ef > t4ca { 1.0 } else { 0.0 };l.f1f32 = t4cb;l.f1fad = 0.0;
        if ((l.f1e5c == 0.0) && (l.f1f32 != 0.0)) {let t4cc: f64 = (1500.0 + 273.15);(l.f22ef, l.f22f0, ) = (t4cc, 0.0, );l.f22f1 = 0.0;}
        let ta: f64 = (1.38062e-23 * l.f22ef);let tb: f64 = (ta / 1.60219e-19);(l.f215b, l.f215c, ) = (tb, ((1.38062e-23 * l.f22f0) / 1.60219e-19), );l.f215d = 0.0;let tc: f64 = (l.f22ef - l.f22f4);let td: f64 = (p.p21 * tc);let te: f64 = (1.0 + td);
        let (t12, t13,) = {
    if (te < 0.01) {
        (0.01, 0.0,)
    } else {
        let tf: f64 = (l.f22ef - l.f22f4);let t10: f64 = (p.p21 * tf);let t11: f64 = (1.0 + t10);
        (t11, (p.p21 * l.f22f0),)
    }
};
        let t14: f64 = (p.p9 * t12);(l.f71, l.f75, ) = (t14, (p.p9 * t13), );l.f76 = 0.0;let t15: f64 = (l.f22ef - l.f22f4);let t16: f64 = (p.p22 * t15);let t17: f64 = (1.0 + t16);
        let (t1b, t1c,) = {
    if (t17 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t18: f64 = (l.f22ef - l.f22f4);let t19: f64 = (p.p22 * t18);let t1a: f64 = (1.0 + t19);
        (t1a, (p.p22 * l.f22f0),)
    }
};
        let t1d: f64 = (p.p10 * t1b);(l.f59, l.f5d, ) = (t1d, (p.p10 * t1c), );l.f5e = 0.0;let t1e: f64 = (l.f22ef - l.f22f4);let t1f: f64 = (p.p23 * t1e);let t20: f64 = (1.0 + t1f);
        let (t24, t25,) = {
    if (t20 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t21: f64 = (l.f22ef - l.f22f4);let t22: f64 = (p.p23 * t21);let t23: f64 = (1.0 + t22);
        (t23, (p.p23 * l.f22f0),)
    }
};
        let t26: f64 = (p.p11 * t24);(l.f5f, l.f63, ) = (t26, (p.p11 * t25), );l.f64 = 0.0;let t27: f64 = (l.f22ef - l.f22f4);let t28: f64 = (p.p24 * t27);let t29: f64 = (1.0 + t28);
        let (t2d, t2e,) = {
    if (t29 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t2a: f64 = (l.f22ef - l.f22f4);let t2b: f64 = (p.p24 * t2a);let t2c: f64 = (1.0 + t2b);
        (t2c, (p.p24 * l.f22f0),)
    }
};
        let t2f: f64 = (p.p13 * t2d);(l.f77, l.f7b, ) = (t2f, (p.p13 * t2e), );l.f7c = 0.0;let t30: f64 = (l.f22ef - l.f22f4);let t31: f64 = (p.p25 * t30);let t32: f64 = (1.0 + t31);
        let (t36, t37,) = {
    if (t32 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t33: f64 = (l.f22ef - l.f22f4);let t34: f64 = (p.p25 * t33);let t35: f64 = (1.0 + t34);
        (t35, (p.p25 * l.f22f0),)
    }
};
        let t38: f64 = (p.p12 * t36);(l.f65, l.f69, ) = (t38, (p.p12 * t37), );l.f6a = 0.0;let t39: f64 = (l.f22ef - l.f22f4);let t3a: f64 = (p.p26 * t39);let t3b: f64 = (1.0 + t3a);
        let (t3f, t40,) = {
    if (t3b < 0.01) {
        (0.01, 0.0,)
    } else {
        let t3c: f64 = (l.f22ef - l.f22f4);let t3d: f64 = (p.p26 * t3c);let t3e: f64 = (1.0 + t3d);
        (t3e, (p.p26 * l.f22f0),)
    }
};
        let t41: f64 = (p.p14 * t3f);(l.f6b, l.f6f, ) = (t41, (p.p14 * t40), );l.f70 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_1(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t42: f64 = (l.f22ef - l.f22f4);let t43: f64 = (p.p21 * t42);let t44: f64 = (1.0 + t43);
        let (t48, t49,) = {
    if (t44 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t45: f64 = (l.f22ef - l.f22f4);let t46: f64 = (p.p21 * t45);let t47: f64 = (1.0 + t46);
        (t47, (p.p21 * l.f22f0),)
    }
};
        let t4a: f64 = (p.p15 * t48);(l.f72, l.f73, ) = (t4a, (p.p15 * t49), );l.f74 = 0.0;let t4b: f64 = (l.f22ef - l.f22f4);let t4c: f64 = (p.p22 * t4b);let t4d: f64 = (1.0 + t4c);
        let (t51, t52,) = {
    if (t4d < 0.01) {
        (0.01, 0.0,)
    } else {
        let t4e: f64 = (l.f22ef - l.f22f4);let t4f: f64 = (p.p22 * t4e);let t50: f64 = (1.0 + t4f);
        (t50, (p.p22 * l.f22f0),)
    }
};
        let t53: f64 = (p.p16 * t51);(l.f5a, l.f5b, ) = (t53, (p.p16 * t52), );l.f5c = 0.0;let t54: f64 = (l.f22ef - l.f22f4);let t55: f64 = (p.p23 * t54);let t56: f64 = (1.0 + t55);
        let (t5a, t5b,) = {
    if (t56 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t57: f64 = (l.f22ef - l.f22f4);let t58: f64 = (p.p23 * t57);let t59: f64 = (1.0 + t58);
        (t59, (p.p23 * l.f22f0),)
    }
};
        let t5c: f64 = (p.p17 * t5a);(l.f60, l.f61, ) = (t5c, (p.p17 * t5b), );l.f62 = 0.0;let t5d: f64 = (l.f22ef - l.f22f4);let t5e: f64 = (p.p24 * t5d);let t5f: f64 = (1.0 + t5e);
        let (t63, t64,) = {
    if (t5f < 0.01) {
        (0.01, 0.0,)
    } else {
        let t60: f64 = (l.f22ef - l.f22f4);let t61: f64 = (p.p24 * t60);let t62: f64 = (1.0 + t61);
        (t62, (p.p24 * l.f22f0),)
    }
};
        let t65: f64 = (p.p19 * t63);(l.f78, l.f79, ) = (t65, (p.p19 * t64), );l.f7a = 0.0;let t74: f64 = (l.f22ef - l.f22f4);let t75: f64 = (p.p25 * t74);let t76: f64 = (1.0 + t75);
        let (t7a, t7b,) = {
    if (t76 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t77: f64 = (l.f22ef - l.f22f4);let t78: f64 = (p.p25 * t77);let t79: f64 = (1.0 + t78);
        (t79, (p.p25 * l.f22f0),)
    }
};
        let t7c: f64 = (p.p18 * t7a);(l.f66, l.f67, ) = (t7c, (p.p18 * t7b), );l.f68 = 0.0;let t1b0: f64 = (l.f22ef - l.f22f4);let t1b1: f64 = (p.p26 * t1b0);let t1b2: f64 = (1.0 + t1b1);
        let (t1b6, t1b7,) = {
    if (t1b2 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t1b3: f64 = (l.f22ef - l.f22f4);let t1b4: f64 = (p.p26 * t1b3);let t1b5: f64 = (1.0 + t1b4);
        (t1b5, (p.p26 * l.f22f0),)
    }
};
        let t1b8: f64 = (p.p20 * t1b6);(l.f6c, l.f6d, ) = (t1b8, (p.p20 * t1b7), );l.f6e = 0.0;let t2ec: f64 = (l.f22ef - l.f22f4);let t2ed: f64 = (p.p8 * t2ec);let t2ee: f64 = (1.0 + t2ed);
        let (t2f2, t2f3,) = {
    if (t2ee < 0.01) {
        (0.01, 0.0,)
    } else {
        let t2ef: f64 = (l.f22ef - l.f22f4);let t2f0: f64 = (p.p8 * t2ef);let t2f1: f64 = (1.0 + t2f0);
        (t2f1, (p.p8 * l.f22f0),)
    }
};
        let t2f4: f64 = (p.p7 * t2f2);(l.f48, l.f49, ) = (t2f4, (p.p7 * t2f3), );l.f4a = 0.0;let t3e9: f64 = (l.f22ef - l.f22f4);let t3ea: f64 = (p.p82 * t3e9);let t3eb: f64 = (1.0 + t3ea);
        let (t3ef, t3f0,) = {
    if (t3eb < 0.01) {
        (0.01, 0.0,)
    } else {
        let t3ec: f64 = (l.f22ef - l.f22f4);let t3ed: f64 = (p.p82 * t3ec);let t3ee: f64 = (1.0 + t3ed);
        (t3ee, (p.p82 * l.f22f0),)
    }
};
        let t3f1: f64 = (p.p81 * t3ef);(l.f3c, l.f3d, ) = (t3f1, (p.p81 * t3f0), );l.f3e = 0.0;let t3f2: f64 = (l.f22ef - l.f22f4);let t3f3: f64 = (p.p104 * t3f2);let t3f4: f64 = (1.0 + t3f3);
        let (t3f8, t3f9,) = {
    if (t3f4 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t3f5: f64 = (l.f22ef - l.f22f4);let t3f6: f64 = (p.p104 * t3f5);let t3f7: f64 = (1.0 + t3f6);
        (t3f7, (p.p104 * l.f22f0),)
    }
};
        let t3fa: f64 = (p.p103 * t3f8);(l.f3f, l.f40, ) = (t3fa, (p.p103 * t3f9), );l.f41 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_2(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t3fb: f64 = (l.f22ef - l.f22f4);let t3fc: f64 = (p.p126 * t3fb);let t3fd: f64 = (1.0 + t3fc);
        let (t401, t402,) = {
    if (t3fd < 0.01) {
        (0.01, 0.0,)
    } else {
        let t3fe: f64 = (l.f22ef - l.f22f4);let t3ff: f64 = (p.p126 * t3fe);let t400: f64 = (1.0 + t3ff);
        (t400, (p.p126 * l.f22f0),)
    }
};
        let t403: f64 = (p.p125 * t401);(l.f42, l.f43, ) = (t403, (p.p125 * t402), );l.f44 = 0.0;let t404: f64 = (l.f22ef - l.f22f4);let t405: f64 = (p.p148 * t404);let t406: f64 = (1.0 + t405);
        let (t40a, t40b,) = {
    if (t406 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t407: f64 = (l.f22ef - l.f22f4);let t408: f64 = (p.p148 * t407);let t409: f64 = (1.0 + t408);
        (t409, (p.p148 * l.f22f0),)
    }
};
        let t40c: f64 = (p.p147 * t40a);(l.f45, l.f46, ) = (t40c, (p.p147 * t40b), );l.f47 = 0.0;let t40f: f64 = (l.f22ef - l.f22f4);let t410: f64 = (p.p87 * t40f);let t411: f64 = (1.0 + t410);
        let (t415, t416,) = {
    if (t411 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t412: f64 = (l.f22ef - l.f22f4);let t413: f64 = (p.p87 * t412);let t414: f64 = (1.0 + t413);
        (t414, (p.p87 * l.f22f0),)
    }
};
        let t417: f64 = (p.p86 * t415);(l.f24, l.f25, ) = (t417, (p.p86 * t416), );l.f26 = 0.0;let t418: f64 = (l.f22ef - l.f22f4);let t419: f64 = (p.p109 * t418);let t41a: f64 = (1.0 + t419);
        let (t41e, t41f,) = {
    if (t41a < 0.01) {
        (0.01, 0.0,)
    } else {
        let t41b: f64 = (l.f22ef - l.f22f4);let t41c: f64 = (p.p109 * t41b);let t41d: f64 = (1.0 + t41c);
        (t41d, (p.p109 * l.f22f0),)
    }
};
        let t420: f64 = (p.p108 * t41e);(l.f27, l.f28, ) = (t420, (p.p108 * t41f), );l.f29 = 0.0;let t421: f64 = (l.f22ef - l.f22f4);let t422: f64 = (p.p131 * t421);let t423: f64 = (1.0 + t422);
        let (t427, t428,) = {
    if (t423 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t424: f64 = (l.f22ef - l.f22f4);let t425: f64 = (p.p131 * t424);let t426: f64 = (1.0 + t425);
        (t426, (p.p131 * l.f22f0),)
    }
};
        let t429: f64 = (p.p130 * t427);(l.f2a, l.f2b, ) = (t429, (p.p130 * t428), );l.f2c = 0.0;let t42a: f64 = (l.f22ef - l.f22f4);let t42b: f64 = (p.p153 * t42a);let t42c: f64 = (1.0 + t42b);
        let (t430, t431,) = {
    if (t42c < 0.01) {
        (0.01, 0.0,)
    } else {
        let t42d: f64 = (l.f22ef - l.f22f4);let t42e: f64 = (p.p153 * t42d);let t42f: f64 = (1.0 + t42e);
        (t42f, (p.p153 * l.f22f0),)
    }
};
        let t432: f64 = (p.p152 * t430);(l.f2d, l.f2e, ) = (t432, (p.p152 * t431), );l.f2f = 0.0;let t433: f64 = (l.f22ef - l.f22f4);let t434: f64 = (p.p89 * t433);let t435: f64 = (1.0 + t434);
        let (t439, t43a,) = {
    if (t435 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t436: f64 = (l.f22ef - l.f22f4);let t437: f64 = (p.p89 * t436);let t438: f64 = (1.0 + t437);
        (t438, (p.p89 * l.f22f0),)
    }
};
        let t43b: f64 = (p.p88 * t439);(l.fc, l.fd, ) = (t43b, (p.p88 * t43a), );l.fe = 0.0;let t43c: f64 = (l.f22ef - l.f22f4);let t43d: f64 = (p.p111 * t43c);let t43e: f64 = (1.0 + t43d);
        let (t442, t443,) = {
    if (t43e < 0.01) {
        (0.01, 0.0,)
    } else {
        let t43f: f64 = (l.f22ef - l.f22f4);let t440: f64 = (p.p111 * t43f);let t441: f64 = (1.0 + t440);
        (t441, (p.p111 * l.f22f0),)
    }
};
        let t444: f64 = (p.p110 * t442);(l.ff, l.f10, ) = (t444, (p.p110 * t443), );l.f11 = 0.0;let t445: f64 = (l.f22ef - l.f22f4);let t446: f64 = (p.p133 * t445);let t447: f64 = (1.0 + t446);
        let (t44b, t44c,) = {
    if (t447 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t448: f64 = (l.f22ef - l.f22f4);let t449: f64 = (p.p133 * t448);let t44a: f64 = (1.0 + t449);
        (t44a, (p.p133 * l.f22f0),)
    }
};
        let t44d: f64 = (p.p132 * t44b);(l.f12, l.f13, ) = (t44d, (p.p132 * t44c), );l.f14 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_3(
        p: &Parameters,
        l: &mut StampLocals,
    ) {
        let t44e: f64 = (l.f22ef - l.f22f4);let t44f: f64 = (p.p155 * t44e);let t450: f64 = (1.0 + t44f);
        let (t454, t455,) = {
    if (t450 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t451: f64 = (l.f22ef - l.f22f4);let t452: f64 = (p.p155 * t451);let t453: f64 = (1.0 + t452);
        (t453, (p.p155 * l.f22f0),)
    }
};
        let t456: f64 = (p.p154 * t454);(l.f15, l.f16, ) = (t456, (p.p154 * t455), );l.f17 = 0.0;let t457: f64 = (l.f22ef - l.f22f4);let t458: f64 = (p.p170 * t457);let t459: f64 = (1.0 + t458);
        let (t45d, t45e,) = {
    if (t459 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t45a: f64 = (l.f22ef - l.f22f4);let t45b: f64 = (p.p170 * t45a);let t45c: f64 = (1.0 + t45b);
        (t45c, (p.p170 * l.f22f0),)
    }
};
        let t45f: f64 = (p.p169 * t45d);(l.f30, l.f31, ) = (t45f, (p.p169 * t45e), );l.f32 = 0.0;let t460: f64 = (l.f22ef - l.f22f4);let t461: f64 = (p.p192 * t460);let t462: f64 = (1.0 + t461);
        let (t466, t467,) = {
    if (t462 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t463: f64 = (l.f22ef - l.f22f4);let t464: f64 = (p.p192 * t463);let t465: f64 = (1.0 + t464);
        (t465, (p.p192 * l.f22f0),)
    }
};
        let t468: f64 = (p.p191 * t466);(l.f33, l.f34, ) = (t468, (p.p191 * t467), );l.f35 = 0.0;let t46c: f64 = (l.f22ef - l.f22f4);let t46d: f64 = (p.p214 * t46c);let t46e: f64 = (1.0 + t46d);
        let (t472, t473,) = {
    if (t46e < 0.01) {
        (0.01, 0.0,)
    } else {
        let t46f: f64 = (l.f22ef - l.f22f4);let t470: f64 = (p.p214 * t46f);let t471: f64 = (1.0 + t470);
        (t471, (p.p214 * l.f22f0),)
    }
};
        let t474: f64 = (p.p213 * t472);(l.f36, l.f37, ) = (t474, (p.p213 * t473), );l.f38 = 0.0;let t475: f64 = (l.f22ef - l.f22f4);let t476: f64 = (p.p236 * t475);let t477: f64 = (1.0 + t476);
        let (t47b, t47c,) = {
    if (t477 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t478: f64 = (l.f22ef - l.f22f4);let t479: f64 = (p.p236 * t478);let t47a: f64 = (1.0 + t479);
        (t47a, (p.p236 * l.f22f0),)
    }
};
        let t47d: f64 = (p.p235 * t47b);(l.f39, l.f3a, ) = (t47d, (p.p235 * t47c), );l.f3b = 0.0;let t47e: f64 = (l.f22ef - l.f22f4);let t47f: f64 = (p.p175 * t47e);let t480: f64 = (1.0 + t47f);
        let (t484, t485,) = {
    if (t480 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t481: f64 = (l.f22ef - l.f22f4);let t482: f64 = (p.p175 * t481);let t483: f64 = (1.0 + t482);
        (t483, (p.p175 * l.f22f0),)
    }
};
        let t486: f64 = (p.p174 * t484);(l.f18, l.f19, ) = (t486, (p.p174 * t485), );l.f1a = 0.0;let t487: f64 = (l.f22ef - l.f22f4);let t488: f64 = (p.p197 * t487);let t489: f64 = (1.0 + t488);
        let (t48d, t48e,) = {
    if (t489 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t48a: f64 = (l.f22ef - l.f22f4);let t48b: f64 = (p.p197 * t48a);let t48c: f64 = (1.0 + t48b);
        (t48c, (p.p197 * l.f22f0),)
    }
};
        let t48f: f64 = (p.p196 * t48d);(l.f1b, l.f1c, ) = (t48f, (p.p196 * t48e), );l.f1d = 0.0;let t490: f64 = (l.f22ef - l.f22f4);let t491: f64 = (p.p219 * t490);let t492: f64 = (1.0 + t491);
        let (t496, t497,) = {
    if (t492 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t493: f64 = (l.f22ef - l.f22f4);let t494: f64 = (p.p219 * t493);let t495: f64 = (1.0 + t494);
        (t495, (p.p219 * l.f22f0),)
    }
};
        let t498: f64 = (p.p218 * t496);(l.f1e, l.f1f, ) = (t498, (p.p218 * t497), );l.f20 = 0.0;let t499: f64 = (l.f22ef - l.f22f4);let t49a: f64 = (p.p241 * t499);let t49b: f64 = (1.0 + t49a);
        let (t49f, t4a0,) = {
    if (t49b < 0.01) {
        (0.01, 0.0,)
    } else {
        let t49c: f64 = (l.f22ef - l.f22f4);let t49d: f64 = (p.p241 * t49c);let t49e: f64 = (1.0 + t49d);
        (t49e, (p.p241 * l.f22f0),)
    }
};
        let t4a1: f64 = (p.p240 * t49f);(l.f21, l.f22, ) = (t4a1, (p.p240 * t4a0), );l.f23 = 0.0;
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_4(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        l: &mut StampLocals,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);let nv7 = ctx.node_voltage(nodes[7]);let nv8 = ctx.node_voltage(nodes[8]);let nv9 = ctx.node_voltage(nodes[9]);let nv10 = ctx.node_voltage(nodes[10]);let nv22 = ctx.node_voltage(nodes[22]);let nv23 = ctx.node_voltage(nodes[23]);let nv25 = ctx.node_voltage(nodes[25]);let nv26 = ctx.node_voltage(nodes[26]);let t4a2: f64 = (l.f22ef - l.f22f4);let t4a3: f64 = (p.p177 * t4a2);let t4a4: f64 = (1.0 + t4a3);
        let (t4a8, t4a9,) = {
    if (t4a4 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t4a5: f64 = (l.f22ef - l.f22f4);let t4a6: f64 = (p.p177 * t4a5);let t4a7: f64 = (1.0 + t4a6);
        (t4a7, (p.p177 * l.f22f0),)
    }
};
        let t4aa: f64 = (p.p176 * t4a8);(l.f0, l.f1, ) = (t4aa, (p.p176 * t4a9), );l.f2 = 0.0;let t4ab: f64 = (l.f22ef - l.f22f4);let t4ac: f64 = (p.p199 * t4ab);let t4ad: f64 = (1.0 + t4ac);
        let (t4b1, t4b2,) = {
    if (t4ad < 0.01) {
        (0.01, 0.0,)
    } else {
        let t4ae: f64 = (l.f22ef - l.f22f4);let t4af: f64 = (p.p199 * t4ae);let t4b0: f64 = (1.0 + t4af);
        (t4b0, (p.p199 * l.f22f0),)
    }
};
        let t4b3: f64 = (p.p198 * t4b1);(l.f3, l.f4, ) = (t4b3, (p.p198 * t4b2), );l.f5 = 0.0;let t4b4: f64 = (l.f22ef - l.f22f4);let t4b5: f64 = (p.p221 * t4b4);let t4b6: f64 = (1.0 + t4b5);
        let (t4ba, t4bb,) = {
    if (t4b6 < 0.01) {
        (0.01, 0.0,)
    } else {
        let t4b7: f64 = (l.f22ef - l.f22f4);let t4b8: f64 = (p.p221 * t4b7);let t4b9: f64 = (1.0 + t4b8);
        (t4b9, (p.p221 * l.f22f0),)
    }
};
        let t4bc: f64 = (p.p220 * t4ba);(l.f6, l.f7, ) = (t4bc, (p.p220 * t4bb), );l.f8 = 0.0;let t4bd: f64 = (l.f22ef - l.f22f4);let t4be: f64 = (p.p243 * t4bd);let t4bf: f64 = (1.0 + t4be);
        let (t4c3, t4c4,) = {
    if (t4bf < 0.01) {
        (0.01, 0.0,)
    } else {
        let t4c0: f64 = (l.f22ef - l.f22f4);let t4c1: f64 = (p.p243 * t4c0);let t4c2: f64 = (1.0 + t4c1);
        (t4c2, (p.p243 * l.f22f0),)
    }
};
        let t4c5: f64 = (p.p242 * t4c3);(l.f9, l.fa, ) = (t4c5, (p.p242 * t4c4), );l.fb = 0.0;let t4c8: f64 = (p.p6 * (nv5 - nv9));(l.f236d, l.f236e, l.f236f, ) = (t4c8, p.p6, (-p.p6), );l.f2370 = 0.0;let t4c9: f64 = (p.p6 * (nv8 - nv9));(l.f23a5, l.f23a6, l.f23a7, ) = (t4c9, p.p6, (-p.p6), );l.f23a8 = 0.0;(l.f2347, l.f2348, ) = (0.0, 0.0, );l.f2349 = 0.0;(l.f2377, l.f2378, ) = (0.0, 0.0, );l.f2379 = 0.0;(l.f234a, l.f234b, ) = (0.0, 0.0, );l.f234c = 0.0;(l.f237a, l.f237b, ) = (0.0, 0.0, );l.f237c = 0.0;(l.f51, l.f52, l.f53, ) = (0.0, 0.0, 0.0, );l.f54 = 0.0;(l.f55, l.f56, l.f57, ) = (0.0, 0.0, 0.0, );l.f58 = 0.0;(l.f4b, l.f4c, l.f4d, l.f4e, l.f4f, ) = (1.0, 0.0, 0.0, 0.0, 0.0, );l.f50 = 0.0;let t4cd: f64 = if p.p328 == 1.0 { 1.0 } else { 0.0 };l.f1db2 = t4cd;l.f1dc7 = 0.0;let t4ce: f64 = if p.p328 == 2.0 { 1.0 } else { 0.0 };l.f1dc8 = t4ce;l.f1ddb = 0.0;
        if ((l.f1db2 == 0.0) && (l.f1dc8 != 0.0)) {(l.f2347, l.f2348, ) = ((nv22 - 0.0), 1.0, );l.f2349 = 0.0;(l.f234a, l.f234b, ) = ((nv23 - 0.0), 1.0, );l.f234c = 0.0;}
        if ((l.f1db2 == 0.0) && (l.f1dc8 != 0.0)) {let t4cf: f64 = (l.f234a - l.f2347);let t4d0: f64 = (t4cf).abs();let t4d1: f64 = (t4d0 / p.p338);(l.f51, l.f52, l.f53, ) = (t4d1, (if t4cf >= 0.0 { (-l.f2348) } else { (-(-l.f2348)) } / p.p338), (if t4cf >= 0.0 { l.f234b } else { (-l.f234b) } / p.p338), );l.f54 = 0.0;}
        if ((l.f1db2 == 0.0) && (l.f1dc8 != 0.0)) {(l.f2377, l.f2378, ) = ((nv25 - 0.0), 1.0, );l.f2379 = 0.0;(l.f237a, l.f237b, ) = ((nv26 - 0.0), 1.0, );l.f237c = 0.0;}
        if ((l.f1db2 == 0.0) && (l.f1dc8 != 0.0)) {let t0: f64 = (l.f237a - l.f2377);let t1: f64 = (t0).abs();let t2: f64 = (t1 / p.p337);(l.f55, l.f56, l.f57, ) = (t2, (if t0 >= 0.0 { (-l.f2378) } else { (-(-l.f2378)) } / p.p337), (if t0 >= 0.0 { l.f237b } else { (-l.f237b) } / p.p337), );l.f58 = 0.0;}
        if ((l.f1db2 == 0.0) && (l.f1dc8 != 0.0)) {let t3: f64 = (1.0 + l.f51);let t4: f64 = (t3 + l.f55);let t5: f64 = (1.0 / t4);(l.f4b, l.f4c, l.f4d, l.f4e, l.f4f, ) = (t5, (-(l.f52 / (t4 * t4))), (-(l.f53 / (t4 * t4))), (-(l.f56 / (t4 * t4))), (-(l.f57 / (t4 * t4))), );l.f50 = 0.0;}
        let t7: f64 = if p.p78 == 1.0 { 1.0 } else { 0.0 };l.f1e06 = t7;l.f1e19 = 0.0;
        if (l.f1e06 != 0.0) {let t8: f64 = (p.p6 * (nv7 - nv10));(l.f2391, l.f2393, l.f2394, l.f2392, ) = (t8, 0.0, p.p6, (-p.p6), );l.f2395 = 0.0;}
    }
}

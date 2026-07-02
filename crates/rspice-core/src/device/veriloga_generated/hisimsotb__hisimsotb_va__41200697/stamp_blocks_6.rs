#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_5(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign2300_e1549, assign2300_e1549_d_n0, assign2300_e1549_d_n2, assign2300_e1549_d_n4, assign2300_e1549_d_n5, assign2300_e1549_d_n6, assign2300_e1549_d_n8, assign2300_e1549_d_n10, assign2300_e1549_d_n11, assign2300_e1549_d_n12,) = {
    if (locals.var_guard15 != 0.0) {
        let assign2300_e1545: f64 = (2.0 * p.p5);
        let assign2300_e1547: f64 = (assign2300_e1545 / locals.var_t1);
        (assign2300_e1547, (-((assign2300_e1545 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((assign2300_e1545 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((assign2300_e1545 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((assign2300_e1545 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((assign2300_e1545 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((assign2300_e1545 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((assign2300_e1545 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((assign2300_e1545 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((assign2300_e1545 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_lod_half, locals.var_lod_half_dn0, locals.var_lod_half_dn2, locals.var_lod_half_dn4, locals.var_lod_half_dn5, locals.var_lod_half_dn6, locals.var_lod_half_dn8, locals.var_lod_half_dn10, locals.var_lod_half_dn11, locals.var_lod_half_dn12,)
    }
};
        locals.var_lod_half = assign2300_e1549;
        locals.var_lod_half_dn0 = assign2300_e1549_d_n0;
        locals.var_lod_half_dn2 = assign2300_e1549_d_n2;
        locals.var_lod_half_dn4 = assign2300_e1549_d_n4;
        locals.var_lod_half_dn5 = assign2300_e1549_d_n5;
        locals.var_lod_half_dn6 = assign2300_e1549_d_n6;
        locals.var_lod_half_dn8 = assign2300_e1549_d_n8;
        locals.var_lod_half_dn10 = assign2300_e1549_d_n10;
        locals.var_lod_half_dn11 = assign2300_e1549_d_n11;
        locals.var_lod_half_dn12 = assign2300_e1549_d_n12;
        locals.var_lod_half_rv = 0.0;

        let (assign2310_e1554, assign2310_e1554_d_n0, assign2310_e1554_d_n2, assign2310_e1554_d_n4, assign2310_e1554_d_n5, assign2310_e1554_d_n6, assign2310_e1554_d_n8, assign2310_e1554_d_n10, assign2310_e1554_d_n11, assign2310_e1554_d_n12,) = {
    if (locals.var_guard15 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lod_half, locals.var_lod_half_dn0, locals.var_lod_half_dn2, locals.var_lod_half_dn4, locals.var_lod_half_dn5, locals.var_lod_half_dn6, locals.var_lod_half_dn8, locals.var_lod_half_dn10, locals.var_lod_half_dn11, locals.var_lod_half_dn12,)
    }
};
        locals.var_lod_half = assign2310_e1554;
        locals.var_lod_half_dn0 = assign2310_e1554_d_n0;
        locals.var_lod_half_dn2 = assign2310_e1554_d_n2;
        locals.var_lod_half_dn4 = assign2310_e1554_d_n4;
        locals.var_lod_half_dn5 = assign2310_e1554_d_n5;
        locals.var_lod_half_dn6 = assign2310_e1554_d_n6;
        locals.var_lod_half_dn8 = assign2310_e1554_d_n8;
        locals.var_lod_half_dn10 = assign2310_e1554_d_n10;
        locals.var_lod_half_dn11 = assign2310_e1554_d_n11;
        locals.var_lod_half_dn12 = assign2310_e1554_d_n12;
        locals.var_lod_half_rv = 0.0;

        let assign2320_e1557: f64 = if locals.var_lod_half > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard16 = assign2320_e1557;
        locals.var_guard16_rv = 0.0;

        let (assign2330_e1565, assign2330_e1565_d_n0, assign2330_e1565_d_n2, assign2330_e1565_d_n4, assign2330_e1565_d_n5, assign2330_e1565_d_n6, assign2330_e1565_d_n8, assign2330_e1565_d_n10, assign2330_e1565_d_n11, assign2330_e1565_d_n12,) = {
    if (locals.var_guard16 != 0.0) {
        let assign2330_e1562: f64 = (1.0 + p.p166);
        let assign2330_e1563: f64 = (1.0 / assign2330_e1562);
        (assign2330_e1563, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign2330_e1565;
        locals.var_t1_dn0 = assign2330_e1565_d_n0;
        locals.var_t1_dn2 = assign2330_e1565_d_n2;
        locals.var_t1_dn4 = assign2330_e1565_d_n4;
        locals.var_t1_dn5 = assign2330_e1565_d_n5;
        locals.var_t1_dn6 = assign2330_e1565_d_n6;
        locals.var_t1_dn8 = assign2330_e1565_d_n8;
        locals.var_t1_dn10 = assign2330_e1565_d_n10;
        locals.var_t1_dn11 = assign2330_e1565_d_n11;
        locals.var_t1_dn12 = assign2330_e1565_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign2340_e1569, assign2340_e1569_d_n0, assign2340_e1569_d_n2, assign2340_e1569_d_n4, assign2340_e1569_d_n5, assign2340_e1569_d_n6, assign2340_e1569_d_n8, assign2340_e1569_d_n10, assign2340_e1569_d_n11, assign2340_e1569_d_n12,) = {
    if (locals.var_guard16 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign2340_e1569;
        locals.var_t2_dn0 = assign2340_e1569_d_n0;
        locals.var_t2_dn2 = assign2340_e1569_d_n2;
        locals.var_t2_dn4 = assign2340_e1569_d_n4;
        locals.var_t2_dn5 = assign2340_e1569_d_n5;
        locals.var_t2_dn6 = assign2340_e1569_d_n6;
        locals.var_t2_dn8 = assign2340_e1569_d_n8;
        locals.var_t2_dn10 = assign2340_e1569_d_n10;
        locals.var_t2_dn11 = assign2340_e1569_d_n11;
        locals.var_t2_dn12 = assign2340_e1569_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign2350_e1573, assign2350_e1573_d_n0, assign2350_e1573_d_n2, assign2350_e1573_d_n4, assign2350_e1573_d_n5, assign2350_e1573_d_n6, assign2350_e1573_d_n8, assign2350_e1573_d_n10, assign2350_e1573_d_n11, assign2350_e1573_d_n12,) = {
    if (locals.var_guard16 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign2350_e1573;
        locals.var_t3_dn0 = assign2350_e1573_d_n0;
        locals.var_t3_dn2 = assign2350_e1573_d_n2;
        locals.var_t3_dn4 = assign2350_e1573_d_n4;
        locals.var_t3_dn5 = assign2350_e1573_d_n5;
        locals.var_t3_dn6 = assign2350_e1573_d_n6;
        locals.var_t3_dn8 = assign2350_e1573_d_n8;
        locals.var_t3_dn10 = assign2350_e1573_d_n10;
        locals.var_t3_dn11 = assign2350_e1573_d_n11;
        locals.var_t3_dn12 = assign2350_e1573_d_n12;
        locals.var_t3_rv = 0.0;

        let (assign2360_e1589, assign2360_e1589_d_n0, assign2360_e1589_d_n2, assign2360_e1589_d_n4, assign2360_e1589_d_n5, assign2360_e1589_d_n6, assign2360_e1589_d_n8, assign2360_e1589_d_n10, assign2360_e1589_d_n11, assign2360_e1589_d_n12,) = {
    if (locals.var_guard16 != 0.0) {
        let assign2360_e1579: f64 = (locals.var_t1 * locals.var_t2);
        let assign2360_e1580: f64 = (1.0 + assign2360_e1579);
        let assign2360_e1581: f64 = (locals.var_nsubpp * assign2360_e1580);
        let assign2360_e1585: f64 = (locals.var_t1 * locals.var_t3);
        let assign2360_e1586: f64 = (1.0 + assign2360_e1585);
        let assign2360_e1587: f64 = (assign2360_e1581 / assign2360_e1586);
        (assign2360_e1587, (((((locals.var_nsubpp_dn0 * assign2360_e1580) + (locals.var_nsubpp * ((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0)))) * assign2360_e1586) - (assign2360_e1581 * ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)))) / (assign2360_e1586 * assign2360_e1586)), (((((locals.var_nsubpp_dn2 * assign2360_e1580) + (locals.var_nsubpp * ((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2)))) * assign2360_e1586) - (assign2360_e1581 * ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)))) / (assign2360_e1586 * assign2360_e1586)), (((((locals.var_nsubpp_dn4 * assign2360_e1580) + (locals.var_nsubpp * ((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)))) * assign2360_e1586) - (assign2360_e1581 * ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)))) / (assign2360_e1586 * assign2360_e1586)), (((((locals.var_nsubpp_dn5 * assign2360_e1580) + (locals.var_nsubpp * ((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)))) * assign2360_e1586) - (assign2360_e1581 * ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)))) / (assign2360_e1586 * assign2360_e1586)), (((((locals.var_nsubpp_dn6 * assign2360_e1580) + (locals.var_nsubpp * ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)))) * assign2360_e1586) - (assign2360_e1581 * ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)))) / (assign2360_e1586 * assign2360_e1586)), (((((locals.var_nsubpp_dn8 * assign2360_e1580) + (locals.var_nsubpp * ((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)))) * assign2360_e1586) - (assign2360_e1581 * ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)))) / (assign2360_e1586 * assign2360_e1586)), (((((locals.var_nsubpp_dn10 * assign2360_e1580) + (locals.var_nsubpp * ((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)))) * assign2360_e1586) - (assign2360_e1581 * ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)))) / (assign2360_e1586 * assign2360_e1586)), (((((locals.var_nsubpp_dn11 * assign2360_e1580) + (locals.var_nsubpp * ((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)))) * assign2360_e1586) - (assign2360_e1581 * ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)))) / (assign2360_e1586 * assign2360_e1586)), (((((locals.var_nsubpp_dn12 * assign2360_e1580) + (locals.var_nsubpp * ((locals.var_t1_dn12 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn12)))) * assign2360_e1586) - (assign2360_e1581 * ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12)))) / (assign2360_e1586 * assign2360_e1586)),)
    } else {
        (locals.var_nsubps, locals.var_nsubps_dn0, locals.var_nsubps_dn2, locals.var_nsubps_dn4, locals.var_nsubps_dn5, locals.var_nsubps_dn6, locals.var_nsubps_dn8, locals.var_nsubps_dn10, locals.var_nsubps_dn11, locals.var_nsubps_dn12,)
    }
};
        locals.var_nsubps = assign2360_e1589;
        locals.var_nsubps_dn0 = assign2360_e1589_d_n0;
        locals.var_nsubps_dn2 = assign2360_e1589_d_n2;
        locals.var_nsubps_dn4 = assign2360_e1589_d_n4;
        locals.var_nsubps_dn5 = assign2360_e1589_d_n5;
        locals.var_nsubps_dn6 = assign2360_e1589_d_n6;
        locals.var_nsubps_dn8 = assign2360_e1589_d_n8;
        locals.var_nsubps_dn10 = assign2360_e1589_d_n10;
        locals.var_nsubps_dn11 = assign2360_e1589_d_n11;
        locals.var_nsubps_dn12 = assign2360_e1589_d_n12;
        locals.var_nsubps_rv = 0.0;

        let (assign2370_e1597, assign2370_e1597_d_n0, assign2370_e1597_d_n2, assign2370_e1597_d_n4, assign2370_e1597_d_n5, assign2370_e1597_d_n6, assign2370_e1597_d_n8, assign2370_e1597_d_n10, assign2370_e1597_d_n11, assign2370_e1597_d_n12,) = {
    if (locals.var_guard16 != 0.0) {
        let assign2370_e1594: f64 = (1.0 + p.p169);
        let assign2370_e1595: f64 = (1.0 / assign2370_e1594);
        (assign2370_e1595, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign2370_e1597;
        locals.var_t1_dn0 = assign2370_e1597_d_n0;
        locals.var_t1_dn2 = assign2370_e1597_d_n2;
        locals.var_t1_dn4 = assign2370_e1597_d_n4;
        locals.var_t1_dn5 = assign2370_e1597_d_n5;
        locals.var_t1_dn6 = assign2370_e1597_d_n6;
        locals.var_t1_dn8 = assign2370_e1597_d_n8;
        locals.var_t1_dn10 = assign2370_e1597_d_n10;
        locals.var_t1_dn11 = assign2370_e1597_d_n11;
        locals.var_t1_dn12 = assign2370_e1597_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign2380_e1605, assign2380_e1605_d_n0, assign2380_e1605_d_n2, assign2380_e1605_d_n4, assign2380_e1605_d_n5, assign2380_e1605_d_n6, assign2380_e1605_d_n8, assign2380_e1605_d_n10, assign2380_e1605_d_n11, assign2380_e1605_d_n12,) = {
    if (locals.var_guard16 != 0.0) {
        let assign2380_e1601: f64 = (p.p168 / locals.var_lod_half);
        let assign2380_e1603: f64 = (assign2380_e1601).powf(p.p170);
        (assign2380_e1603, if 0.0 == 0.0 && ((p.p170) as f64).is_finite() && ((p.p170) as f64).fract() == 0.0 { if p.p170 == 0.0 { 0.0 } else { (p.p170 * ((assign2380_e1601).powf(p.p170 - 1.0) * (-((p.p168 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2380_e1603 * (p.p170 * ((-((p.p168 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))) / assign2380_e1601))) }, if 0.0 == 0.0 && ((p.p170) as f64).is_finite() && ((p.p170) as f64).fract() == 0.0 { if p.p170 == 0.0 { 0.0 } else { (p.p170 * ((assign2380_e1601).powf(p.p170 - 1.0) * (-((p.p168 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2380_e1603 * (p.p170 * ((-((p.p168 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))) / assign2380_e1601))) }, if 0.0 == 0.0 && ((p.p170) as f64).is_finite() && ((p.p170) as f64).fract() == 0.0 { if p.p170 == 0.0 { 0.0 } else { (p.p170 * ((assign2380_e1601).powf(p.p170 - 1.0) * (-((p.p168 * locals.var_lod_half_dn4) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2380_e1603 * (p.p170 * ((-((p.p168 * locals.var_lod_half_dn4) / (locals.var_lod_half * locals.var_lod_half))) / assign2380_e1601))) }, if 0.0 == 0.0 && ((p.p170) as f64).is_finite() && ((p.p170) as f64).fract() == 0.0 { if p.p170 == 0.0 { 0.0 } else { (p.p170 * ((assign2380_e1601).powf(p.p170 - 1.0) * (-((p.p168 * locals.var_lod_half_dn5) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2380_e1603 * (p.p170 * ((-((p.p168 * locals.var_lod_half_dn5) / (locals.var_lod_half * locals.var_lod_half))) / assign2380_e1601))) }, if 0.0 == 0.0 && ((p.p170) as f64).is_finite() && ((p.p170) as f64).fract() == 0.0 { if p.p170 == 0.0 { 0.0 } else { (p.p170 * ((assign2380_e1601).powf(p.p170 - 1.0) * (-((p.p168 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2380_e1603 * (p.p170 * ((-((p.p168 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))) / assign2380_e1601))) }, if 0.0 == 0.0 && ((p.p170) as f64).is_finite() && ((p.p170) as f64).fract() == 0.0 { if p.p170 == 0.0 { 0.0 } else { (p.p170 * ((assign2380_e1601).powf(p.p170 - 1.0) * (-((p.p168 * locals.var_lod_half_dn8) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2380_e1603 * (p.p170 * ((-((p.p168 * locals.var_lod_half_dn8) / (locals.var_lod_half * locals.var_lod_half))) / assign2380_e1601))) }, if 0.0 == 0.0 && ((p.p170) as f64).is_finite() && ((p.p170) as f64).fract() == 0.0 { if p.p170 == 0.0 { 0.0 } else { (p.p170 * ((assign2380_e1601).powf(p.p170 - 1.0) * (-((p.p168 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2380_e1603 * (p.p170 * ((-((p.p168 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))) / assign2380_e1601))) }, if 0.0 == 0.0 && ((p.p170) as f64).is_finite() && ((p.p170) as f64).fract() == 0.0 { if p.p170 == 0.0 { 0.0 } else { (p.p170 * ((assign2380_e1601).powf(p.p170 - 1.0) * (-((p.p168 * locals.var_lod_half_dn11) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2380_e1603 * (p.p170 * ((-((p.p168 * locals.var_lod_half_dn11) / (locals.var_lod_half * locals.var_lod_half))) / assign2380_e1601))) }, if 0.0 == 0.0 && ((p.p170) as f64).is_finite() && ((p.p170) as f64).fract() == 0.0 { if p.p170 == 0.0 { 0.0 } else { (p.p170 * ((assign2380_e1601).powf(p.p170 - 1.0) * (-((p.p168 * locals.var_lod_half_dn12) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign2380_e1603 * (p.p170 * ((-((p.p168 * locals.var_lod_half_dn12) / (locals.var_lod_half * locals.var_lod_half))) / assign2380_e1601))) },)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign2380_e1605;
        locals.var_t2_dn0 = assign2380_e1605_d_n0;
        locals.var_t2_dn2 = assign2380_e1605_d_n2;
        locals.var_t2_dn4 = assign2380_e1605_d_n4;
        locals.var_t2_dn5 = assign2380_e1605_d_n5;
        locals.var_t2_dn6 = assign2380_e1605_d_n6;
        locals.var_t2_dn8 = assign2380_e1605_d_n8;
        locals.var_t2_dn10 = assign2380_e1605_d_n10;
        locals.var_t2_dn11 = assign2380_e1605_d_n11;
        locals.var_t2_dn12 = assign2380_e1605_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign2390_e1613, assign2390_e1613_d_n0, assign2390_e1613_d_n2, assign2390_e1613_d_n4, assign2390_e1613_d_n5, assign2390_e1613_d_n6, assign2390_e1613_d_n8, assign2390_e1613_d_n10, assign2390_e1613_d_n11, assign2390_e1613_d_n12,) = {
    if (locals.var_guard16 != 0.0) {
        let assign2390_e1609: f64 = (p.p168 / locals.var_lod_half_ref);
        let assign2390_e1611: f64 = (assign2390_e1609).powf(p.p170);
        (assign2390_e1611, if 0.0 == 0.0 && ((p.p170) as f64).is_finite() && ((p.p170) as f64).fract() == 0.0 { if p.p170 == 0.0 { 0.0 } else { (p.p170 * ((assign2390_e1609).powf(p.p170 - 1.0) * (-((p.p168 * locals.var_lod_half_ref_dn0) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign2390_e1611 * (p.p170 * ((-((p.p168 * locals.var_lod_half_ref_dn0) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign2390_e1609))) }, if 0.0 == 0.0 && ((p.p170) as f64).is_finite() && ((p.p170) as f64).fract() == 0.0 { if p.p170 == 0.0 { 0.0 } else { (p.p170 * ((assign2390_e1609).powf(p.p170 - 1.0) * (-((p.p168 * locals.var_lod_half_ref_dn2) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign2390_e1611 * (p.p170 * ((-((p.p168 * locals.var_lod_half_ref_dn2) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign2390_e1609))) }, if 0.0 == 0.0 && ((p.p170) as f64).is_finite() && ((p.p170) as f64).fract() == 0.0 { if p.p170 == 0.0 { 0.0 } else { (p.p170 * ((assign2390_e1609).powf(p.p170 - 1.0) * (-((p.p168 * locals.var_lod_half_ref_dn4) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign2390_e1611 * (p.p170 * ((-((p.p168 * locals.var_lod_half_ref_dn4) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign2390_e1609))) }, if 0.0 == 0.0 && ((p.p170) as f64).is_finite() && ((p.p170) as f64).fract() == 0.0 { if p.p170 == 0.0 { 0.0 } else { (p.p170 * ((assign2390_e1609).powf(p.p170 - 1.0) * (-((p.p168 * locals.var_lod_half_ref_dn5) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign2390_e1611 * (p.p170 * ((-((p.p168 * locals.var_lod_half_ref_dn5) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign2390_e1609))) }, if 0.0 == 0.0 && ((p.p170) as f64).is_finite() && ((p.p170) as f64).fract() == 0.0 { if p.p170 == 0.0 { 0.0 } else { (p.p170 * ((assign2390_e1609).powf(p.p170 - 1.0) * (-((p.p168 * locals.var_lod_half_ref_dn6) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign2390_e1611 * (p.p170 * ((-((p.p168 * locals.var_lod_half_ref_dn6) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign2390_e1609))) }, if 0.0 == 0.0 && ((p.p170) as f64).is_finite() && ((p.p170) as f64).fract() == 0.0 { if p.p170 == 0.0 { 0.0 } else { (p.p170 * ((assign2390_e1609).powf(p.p170 - 1.0) * (-((p.p168 * locals.var_lod_half_ref_dn8) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign2390_e1611 * (p.p170 * ((-((p.p168 * locals.var_lod_half_ref_dn8) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign2390_e1609))) }, if 0.0 == 0.0 && ((p.p170) as f64).is_finite() && ((p.p170) as f64).fract() == 0.0 { if p.p170 == 0.0 { 0.0 } else { (p.p170 * ((assign2390_e1609).powf(p.p170 - 1.0) * (-((p.p168 * locals.var_lod_half_ref_dn10) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign2390_e1611 * (p.p170 * ((-((p.p168 * locals.var_lod_half_ref_dn10) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign2390_e1609))) }, if 0.0 == 0.0 && ((p.p170) as f64).is_finite() && ((p.p170) as f64).fract() == 0.0 { if p.p170 == 0.0 { 0.0 } else { (p.p170 * ((assign2390_e1609).powf(p.p170 - 1.0) * (-((p.p168 * locals.var_lod_half_ref_dn11) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign2390_e1611 * (p.p170 * ((-((p.p168 * locals.var_lod_half_ref_dn11) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign2390_e1609))) }, if 0.0 == 0.0 && ((p.p170) as f64).is_finite() && ((p.p170) as f64).fract() == 0.0 { if p.p170 == 0.0 { 0.0 } else { (p.p170 * ((assign2390_e1609).powf(p.p170 - 1.0) * (-((p.p168 * locals.var_lod_half_ref_dn12) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign2390_e1611 * (p.p170 * ((-((p.p168 * locals.var_lod_half_ref_dn12) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign2390_e1609))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign2390_e1613;
        locals.var_t3_dn0 = assign2390_e1613_d_n0;
        locals.var_t3_dn2 = assign2390_e1613_d_n2;
        locals.var_t3_dn4 = assign2390_e1613_d_n4;
        locals.var_t3_dn5 = assign2390_e1613_d_n5;
        locals.var_t3_dn6 = assign2390_e1613_d_n6;
        locals.var_t3_dn8 = assign2390_e1613_d_n8;
        locals.var_t3_dn10 = assign2390_e1613_d_n10;
        locals.var_t3_dn11 = assign2390_e1613_d_n11;
        locals.var_t3_dn12 = assign2390_e1613_d_n12;
        locals.var_t3_rv = 0.0;

        let (assign2400_e1629, assign2400_e1629_d_n0, assign2400_e1629_d_n2, assign2400_e1629_d_n4, assign2400_e1629_d_n5, assign2400_e1629_d_n6, assign2400_e1629_d_n8, assign2400_e1629_d_n10, assign2400_e1629_d_n11, assign2400_e1629_d_n12,) = {
    if (locals.var_guard16 != 0.0) {
        let assign2400_e1619: f64 = (locals.var_t1 * locals.var_t2);
        let assign2400_e1620: f64 = (1.0 + assign2400_e1619);
        let assign2400_e1621: f64 = (locals.var_mks_nsubs * assign2400_e1620);
        let assign2400_e1625: f64 = (locals.var_t1 * locals.var_t3);
        let assign2400_e1626: f64 = (1.0 + assign2400_e1625);
        let assign2400_e1627: f64 = (assign2400_e1621 / assign2400_e1626);
        (assign2400_e1627, (((((locals.var_mks_nsubs_dn0 * assign2400_e1620) + (locals.var_mks_nsubs * ((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0)))) * assign2400_e1626) - (assign2400_e1621 * ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)))) / (assign2400_e1626 * assign2400_e1626)), (((((locals.var_mks_nsubs_dn2 * assign2400_e1620) + (locals.var_mks_nsubs * ((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2)))) * assign2400_e1626) - (assign2400_e1621 * ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)))) / (assign2400_e1626 * assign2400_e1626)), (((((locals.var_mks_nsubs_dn4 * assign2400_e1620) + (locals.var_mks_nsubs * ((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)))) * assign2400_e1626) - (assign2400_e1621 * ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)))) / (assign2400_e1626 * assign2400_e1626)), (((((locals.var_mks_nsubs_dn5 * assign2400_e1620) + (locals.var_mks_nsubs * ((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)))) * assign2400_e1626) - (assign2400_e1621 * ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)))) / (assign2400_e1626 * assign2400_e1626)), (((((locals.var_mks_nsubs_dn6 * assign2400_e1620) + (locals.var_mks_nsubs * ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)))) * assign2400_e1626) - (assign2400_e1621 * ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)))) / (assign2400_e1626 * assign2400_e1626)), (((((locals.var_mks_nsubs_dn8 * assign2400_e1620) + (locals.var_mks_nsubs * ((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)))) * assign2400_e1626) - (assign2400_e1621 * ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)))) / (assign2400_e1626 * assign2400_e1626)), (((((locals.var_mks_nsubs_dn10 * assign2400_e1620) + (locals.var_mks_nsubs * ((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)))) * assign2400_e1626) - (assign2400_e1621 * ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)))) / (assign2400_e1626 * assign2400_e1626)), (((((locals.var_mks_nsubs_dn11 * assign2400_e1620) + (locals.var_mks_nsubs * ((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)))) * assign2400_e1626) - (assign2400_e1621 * ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)))) / (assign2400_e1626 * assign2400_e1626)), (((((locals.var_mks_nsubs_dn12 * assign2400_e1620) + (locals.var_mks_nsubs * ((locals.var_t1_dn12 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn12)))) * assign2400_e1626) - (assign2400_e1621 * ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12)))) / (assign2400_e1626 * assign2400_e1626)),)
    } else {
        (locals.var_mks_nsubs, locals.var_mks_nsubs_dn0, locals.var_mks_nsubs_dn2, locals.var_mks_nsubs_dn4, locals.var_mks_nsubs_dn5, locals.var_mks_nsubs_dn6, locals.var_mks_nsubs_dn8, locals.var_mks_nsubs_dn10, locals.var_mks_nsubs_dn11, locals.var_mks_nsubs_dn12,)
    }
};
        locals.var_mks_nsubs = assign2400_e1629;
        locals.var_mks_nsubs_dn0 = assign2400_e1629_d_n0;
        locals.var_mks_nsubs_dn2 = assign2400_e1629_d_n2;
        locals.var_mks_nsubs_dn4 = assign2400_e1629_d_n4;
        locals.var_mks_nsubs_dn5 = assign2400_e1629_d_n5;
        locals.var_mks_nsubs_dn6 = assign2400_e1629_d_n6;
        locals.var_mks_nsubs_dn8 = assign2400_e1629_d_n8;
        locals.var_mks_nsubs_dn10 = assign2400_e1629_d_n10;
        locals.var_mks_nsubs_dn11 = assign2400_e1629_d_n11;
        locals.var_mks_nsubs_dn12 = assign2400_e1629_d_n12;
        locals.var_mks_nsubs_rv = 0.0;

        let (assign2410_e1634, assign2410_e1634_d_n0, assign2410_e1634_d_n2, assign2410_e1634_d_n4, assign2410_e1634_d_n5, assign2410_e1634_d_n6, assign2410_e1634_d_n8, assign2410_e1634_d_n10, assign2410_e1634_d_n11, assign2410_e1634_d_n12,) = {
    if (locals.var_guard16 == 0.0) {
        (locals.var_nsubpp, locals.var_nsubpp_dn0, locals.var_nsubpp_dn2, locals.var_nsubpp_dn4, locals.var_nsubpp_dn5, locals.var_nsubpp_dn6, locals.var_nsubpp_dn8, locals.var_nsubpp_dn10, locals.var_nsubpp_dn11, locals.var_nsubpp_dn12,)
    } else {
        (locals.var_nsubps, locals.var_nsubps_dn0, locals.var_nsubps_dn2, locals.var_nsubps_dn4, locals.var_nsubps_dn5, locals.var_nsubps_dn6, locals.var_nsubps_dn8, locals.var_nsubps_dn10, locals.var_nsubps_dn11, locals.var_nsubps_dn12,)
    }
};
        locals.var_nsubps = assign2410_e1634;
        locals.var_nsubps_dn0 = assign2410_e1634_d_n0;
        locals.var_nsubps_dn2 = assign2410_e1634_d_n2;
        locals.var_nsubps_dn4 = assign2410_e1634_d_n4;
        locals.var_nsubps_dn5 = assign2410_e1634_d_n5;
        locals.var_nsubps_dn6 = assign2410_e1634_d_n6;
        locals.var_nsubps_dn8 = assign2410_e1634_d_n8;
        locals.var_nsubps_dn10 = assign2410_e1634_d_n10;
        locals.var_nsubps_dn11 = assign2410_e1634_d_n11;
        locals.var_nsubps_dn12 = assign2410_e1634_d_n12;
        locals.var_nsubps_rv = 0.0;

        let assign2420_e1639: f64 = (locals.var_wg).powf(p.p191);
        let assign2420_e1640: f64 = (p.p190 / assign2420_e1639);
        let assign2420_e1641: f64 = (1.0 + assign2420_e1640);
        locals.var_t2 = assign2420_e1641;
        locals.var_t2_dn0 = 0.0;
        locals.var_t2_dn2 = 0.0;
        locals.var_t2_dn4 = 0.0;
        locals.var_t2_dn5 = 0.0;
        locals.var_t2_dn6 = 0.0;
        locals.var_t2_dn8 = 0.0;
        locals.var_t2_dn10 = 0.0;
        locals.var_t2_dn11 = 0.0;
        locals.var_t2_dn12 = 0.0;
        locals.var_t2_rv = 0.0;

        let assign2430_e1644: f64 = (locals.var_mks_nsubsmax / locals.var_mks_nsubs);
        locals.var_t3 = assign2430_e1644;
        locals.var_t3_dn0 = (-((locals.var_mks_nsubsmax * locals.var_mks_nsubs_dn0) / (locals.var_mks_nsubs * locals.var_mks_nsubs)));
        locals.var_t3_dn2 = (-((locals.var_mks_nsubsmax * locals.var_mks_nsubs_dn2) / (locals.var_mks_nsubs * locals.var_mks_nsubs)));
        locals.var_t3_dn4 = (-((locals.var_mks_nsubsmax * locals.var_mks_nsubs_dn4) / (locals.var_mks_nsubs * locals.var_mks_nsubs)));
        locals.var_t3_dn5 = (-((locals.var_mks_nsubsmax * locals.var_mks_nsubs_dn5) / (locals.var_mks_nsubs * locals.var_mks_nsubs)));
        locals.var_t3_dn6 = (-((locals.var_mks_nsubsmax * locals.var_mks_nsubs_dn6) / (locals.var_mks_nsubs * locals.var_mks_nsubs)));
        locals.var_t3_dn8 = (-((locals.var_mks_nsubsmax * locals.var_mks_nsubs_dn8) / (locals.var_mks_nsubs * locals.var_mks_nsubs)));
        locals.var_t3_dn10 = (-((locals.var_mks_nsubsmax * locals.var_mks_nsubs_dn10) / (locals.var_mks_nsubs * locals.var_mks_nsubs)));
        locals.var_t3_dn11 = (-((locals.var_mks_nsubsmax * locals.var_mks_nsubs_dn11) / (locals.var_mks_nsubs * locals.var_mks_nsubs)));
        locals.var_t3_dn12 = (-((locals.var_mks_nsubsmax * locals.var_mks_nsubs_dn12) / (locals.var_mks_nsubs * locals.var_mks_nsubs)));
        locals.var_t3_rv = 0.0;

        let assign2440_e1647: f64 = (locals.var_t3 - locals.var_t2);
        let assign2440_e1649: f64 = (assign2440_e1647 - 0.01);
        locals.var_tmf1 = assign2440_e1649;
        locals.var_tmf1_dn0 = (locals.var_t3_dn0 - locals.var_t2_dn0);
        locals.var_tmf1_dn2 = (locals.var_t3_dn2 - locals.var_t2_dn2);
        locals.var_tmf1_dn4 = (locals.var_t3_dn4 - locals.var_t2_dn4);
        locals.var_tmf1_dn5 = (locals.var_t3_dn5 - locals.var_t2_dn5);
        locals.var_tmf1_dn6 = (locals.var_t3_dn6 - locals.var_t2_dn6);
        locals.var_tmf1_dn8 = (locals.var_t3_dn8 - locals.var_t2_dn8);
        locals.var_tmf1_dn10 = (locals.var_t3_dn10 - locals.var_t2_dn10);
        locals.var_tmf1_dn11 = (locals.var_t3_dn11 - locals.var_t2_dn11);
        locals.var_tmf1_dn12 = (locals.var_t3_dn12 - locals.var_t2_dn12);
        locals.var_tmf1_rv = 0.0;

        let assign2450_e1652: f64 = (4.0 * locals.var_t3);
        let assign2450_e1654: f64 = (assign2450_e1652 * 0.01);
        locals.var_tmf2 = assign2450_e1654;
        locals.var_tmf2_dn0 = ((4.0 * locals.var_t3_dn0) * 0.01);
        locals.var_tmf2_dn2 = ((4.0 * locals.var_t3_dn2) * 0.01);
        locals.var_tmf2_dn4 = ((4.0 * locals.var_t3_dn4) * 0.01);
        locals.var_tmf2_dn5 = ((4.0 * locals.var_t3_dn5) * 0.01);
        locals.var_tmf2_dn6 = ((4.0 * locals.var_t3_dn6) * 0.01);
        locals.var_tmf2_dn8 = ((4.0 * locals.var_t3_dn8) * 0.01);
        locals.var_tmf2_dn10 = ((4.0 * locals.var_t3_dn10) * 0.01);
        locals.var_tmf2_dn11 = ((4.0 * locals.var_t3_dn11) * 0.01);
        locals.var_tmf2_dn12 = ((4.0 * locals.var_t3_dn12) * 0.01);
        locals.var_tmf2_rv = 0.0;

        let (assign2460_e1661, assign2460_e1661_d_n0, assign2460_e1661_d_n2, assign2460_e1661_d_n4, assign2460_e1661_d_n5, assign2460_e1661_d_n6, assign2460_e1661_d_n8, assign2460_e1661_d_n10, assign2460_e1661_d_n11, assign2460_e1661_d_n12,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    } else {
        let assign2460_e1660: f64 = (-locals.var_tmf2);
        (assign2460_e1660, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12),)
    }
};
        locals.var_tmf2 = assign2460_e1661;
        locals.var_tmf2_dn0 = assign2460_e1661_d_n0;
        locals.var_tmf2_dn2 = assign2460_e1661_d_n2;
        locals.var_tmf2_dn4 = assign2460_e1661_d_n4;
        locals.var_tmf2_dn5 = assign2460_e1661_d_n5;
        locals.var_tmf2_dn6 = assign2460_e1661_d_n6;
        locals.var_tmf2_dn8 = assign2460_e1661_d_n8;
        locals.var_tmf2_dn10 = assign2460_e1661_d_n10;
        locals.var_tmf2_dn11 = assign2460_e1661_d_n11;
        locals.var_tmf2_dn12 = assign2460_e1661_d_n12;
        locals.var_tmf2_rv = 0.0;

        let assign2470_e1664: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign2470_e1666: f64 = (assign2470_e1664 + locals.var_tmf2);
        let assign2470_e1667: f64 = (assign2470_e1666).sqrt();
        locals.var_tmf2 = assign2470_e1667;
        locals.var_tmf2_dn0 = ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign2470_e1667));
        locals.var_tmf2_dn2 = ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign2470_e1667));
        locals.var_tmf2_dn4 = ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign2470_e1667));
        locals.var_tmf2_dn5 = ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign2470_e1667));
        locals.var_tmf2_dn6 = ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign2470_e1667));
        locals.var_tmf2_dn8 = ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign2470_e1667));
        locals.var_tmf2_dn10 = ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign2470_e1667));
        locals.var_tmf2_dn11 = ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign2470_e1667));
        locals.var_tmf2_dn12 = ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign2470_e1667));
        locals.var_tmf2_rv = 0.0;

        let assign2480_e1672: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign2480_e1673: f64 = (0.5 * assign2480_e1672);
        let assign2480_e1674: f64 = (locals.var_t3 - assign2480_e1673);
        locals.var_t1 = assign2480_e1674;
        locals.var_t1_dn0 = (locals.var_t3_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)));
        locals.var_t1_dn2 = (locals.var_t3_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)));
        locals.var_t1_dn4 = (locals.var_t3_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)));
        locals.var_t1_dn5 = (locals.var_t3_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)));
        locals.var_t1_dn6 = (locals.var_t3_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)));
        locals.var_t1_dn8 = (locals.var_t3_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)));
        locals.var_t1_dn10 = (locals.var_t3_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)));
        locals.var_t1_dn11 = (locals.var_t3_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)));
        locals.var_t1_dn12 = (locals.var_t3_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12)));
        locals.var_t1_rv = 0.0;

        let assign2490_e1677: f64 = (locals.var_mks_nsubs * locals.var_t1);
        locals.var_uc_nsubs = assign2490_e1677;
        locals.var_uc_nsubs_dn0 = ((locals.var_mks_nsubs_dn0 * locals.var_t1) + (locals.var_mks_nsubs * locals.var_t1_dn0));
        locals.var_uc_nsubs_dn2 = ((locals.var_mks_nsubs_dn2 * locals.var_t1) + (locals.var_mks_nsubs * locals.var_t1_dn2));
        locals.var_uc_nsubs_dn4 = ((locals.var_mks_nsubs_dn4 * locals.var_t1) + (locals.var_mks_nsubs * locals.var_t1_dn4));
        locals.var_uc_nsubs_dn5 = ((locals.var_mks_nsubs_dn5 * locals.var_t1) + (locals.var_mks_nsubs * locals.var_t1_dn5));
        locals.var_uc_nsubs_dn6 = ((locals.var_mks_nsubs_dn6 * locals.var_t1) + (locals.var_mks_nsubs * locals.var_t1_dn6));
        locals.var_uc_nsubs_dn8 = ((locals.var_mks_nsubs_dn8 * locals.var_t1) + (locals.var_mks_nsubs * locals.var_t1_dn8));
        locals.var_uc_nsubs_dn10 = ((locals.var_mks_nsubs_dn10 * locals.var_t1) + (locals.var_mks_nsubs * locals.var_t1_dn10));
        locals.var_uc_nsubs_dn11 = ((locals.var_mks_nsubs_dn11 * locals.var_t1) + (locals.var_mks_nsubs * locals.var_t1_dn11));
        locals.var_uc_nsubs_dn12 = ((locals.var_mks_nsubs_dn12 * locals.var_t1) + (locals.var_mks_nsubs * locals.var_t1_dn12));
        locals.var_uc_nsubs_rv = 0.0;

        let assign2500_e1684: f64 = if ((locals.var_lgate > p.p58) || (p.p58 <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard17 = assign2500_e1684;
        locals.var_guard17_rv = 0.0;

        let (assign2510_e1698, assign2510_e1698_d_n0, assign2510_e1698_d_n2, assign2510_e1698_d_n4, assign2510_e1698_d_n5, assign2510_e1698_d_n6, assign2510_e1698_d_n8, assign2510_e1698_d_n10, assign2510_e1698_d_n11, assign2510_e1698_d_n12,) = {
    if (locals.var_guard17 != 0.0) {
        let assign2510_e1689: f64 = (locals.var_lgate - p.p58);
        let assign2510_e1690: f64 = (locals.var_uc_nsubs * assign2510_e1689);
        let assign2510_e1693: f64 = (locals.var_nsubps * p.p58);
        let assign2510_e1694: f64 = (assign2510_e1690 + assign2510_e1693);
        let assign2510_e1696: f64 = (assign2510_e1694 / locals.var_lgate);
        (assign2510_e1696, (((locals.var_uc_nsubs_dn0 * assign2510_e1689) + (locals.var_nsubps_dn0 * p.p58)) / locals.var_lgate), (((locals.var_uc_nsubs_dn2 * assign2510_e1689) + (locals.var_nsubps_dn2 * p.p58)) / locals.var_lgate), (((locals.var_uc_nsubs_dn4 * assign2510_e1689) + (locals.var_nsubps_dn4 * p.p58)) / locals.var_lgate), (((locals.var_uc_nsubs_dn5 * assign2510_e1689) + (locals.var_nsubps_dn5 * p.p58)) / locals.var_lgate), (((locals.var_uc_nsubs_dn6 * assign2510_e1689) + (locals.var_nsubps_dn6 * p.p58)) / locals.var_lgate), (((locals.var_uc_nsubs_dn8 * assign2510_e1689) + (locals.var_nsubps_dn8 * p.p58)) / locals.var_lgate), (((locals.var_uc_nsubs_dn10 * assign2510_e1689) + (locals.var_nsubps_dn10 * p.p58)) / locals.var_lgate), (((locals.var_uc_nsubs_dn11 * assign2510_e1689) + (locals.var_nsubps_dn11 * p.p58)) / locals.var_lgate), (((locals.var_uc_nsubs_dn12 * assign2510_e1689) + (locals.var_nsubps_dn12 * p.p58)) / locals.var_lgate),)
    } else {
        (locals.var_nsub, locals.var_nsub_dn0, locals.var_nsub_dn2, locals.var_nsub_dn4, locals.var_nsub_dn5, locals.var_nsub_dn6, locals.var_nsub_dn8, locals.var_nsub_dn10, locals.var_nsub_dn11, locals.var_nsub_dn12,)
    }
};
        locals.var_nsub = assign2510_e1698;
        locals.var_nsub_dn0 = assign2510_e1698_d_n0;
        locals.var_nsub_dn2 = assign2510_e1698_d_n2;
        locals.var_nsub_dn4 = assign2510_e1698_d_n4;
        locals.var_nsub_dn5 = assign2510_e1698_d_n5;
        locals.var_nsub_dn6 = assign2510_e1698_d_n6;
        locals.var_nsub_dn8 = assign2510_e1698_d_n8;
        locals.var_nsub_dn10 = assign2510_e1698_d_n10;
        locals.var_nsub_dn11 = assign2510_e1698_d_n11;
        locals.var_nsub_dn12 = assign2510_e1698_d_n12;
        locals.var_nsub_rv = 0.0;

        let (assign2520_e1713, assign2520_e1713_d_n0, assign2520_e1713_d_n2, assign2520_e1713_d_n4, assign2520_e1713_d_n5, assign2520_e1713_d_n6, assign2520_e1713_d_n8, assign2520_e1713_d_n10, assign2520_e1713_d_n11, assign2520_e1713_d_n12,) = {
    if (locals.var_guard17 == 0.0) {
        let assign2520_e1704: f64 = (locals.var_nsubps - locals.var_uc_nsubs);
        let assign2520_e1707: f64 = (p.p58 - locals.var_lgate);
        let assign2520_e1708: f64 = (assign2520_e1704 * assign2520_e1707);
        let assign2520_e1710: f64 = (assign2520_e1708 / p.p58);
        let assign2520_e1711: f64 = (locals.var_nsubps + assign2520_e1710);
        (assign2520_e1711, (locals.var_nsubps_dn0 + (((locals.var_nsubps_dn0 - locals.var_uc_nsubs_dn0) * assign2520_e1707) / p.p58)), (locals.var_nsubps_dn2 + (((locals.var_nsubps_dn2 - locals.var_uc_nsubs_dn2) * assign2520_e1707) / p.p58)), (locals.var_nsubps_dn4 + (((locals.var_nsubps_dn4 - locals.var_uc_nsubs_dn4) * assign2520_e1707) / p.p58)), (locals.var_nsubps_dn5 + (((locals.var_nsubps_dn5 - locals.var_uc_nsubs_dn5) * assign2520_e1707) / p.p58)), (locals.var_nsubps_dn6 + (((locals.var_nsubps_dn6 - locals.var_uc_nsubs_dn6) * assign2520_e1707) / p.p58)), (locals.var_nsubps_dn8 + (((locals.var_nsubps_dn8 - locals.var_uc_nsubs_dn8) * assign2520_e1707) / p.p58)), (locals.var_nsubps_dn10 + (((locals.var_nsubps_dn10 - locals.var_uc_nsubs_dn10) * assign2520_e1707) / p.p58)), (locals.var_nsubps_dn11 + (((locals.var_nsubps_dn11 - locals.var_uc_nsubs_dn11) * assign2520_e1707) / p.p58)), (locals.var_nsubps_dn12 + (((locals.var_nsubps_dn12 - locals.var_uc_nsubs_dn12) * assign2520_e1707) / p.p58)),)
    } else {
        (locals.var_nsub, locals.var_nsub_dn0, locals.var_nsub_dn2, locals.var_nsub_dn4, locals.var_nsub_dn5, locals.var_nsub_dn6, locals.var_nsub_dn8, locals.var_nsub_dn10, locals.var_nsub_dn11, locals.var_nsub_dn12,)
    }
};
        locals.var_nsub = assign2520_e1713;
        locals.var_nsub_dn0 = assign2520_e1713_d_n0;
        locals.var_nsub_dn2 = assign2520_e1713_d_n2;
        locals.var_nsub_dn4 = assign2520_e1713_d_n4;
        locals.var_nsub_dn5 = assign2520_e1713_d_n5;
        locals.var_nsub_dn6 = assign2520_e1713_d_n6;
        locals.var_nsub_dn8 = assign2520_e1713_d_n8;
        locals.var_nsub_dn10 = assign2520_e1713_d_n10;
        locals.var_nsub_dn11 = assign2520_e1713_d_n11;
        locals.var_nsub_dn12 = assign2520_e1713_d_n12;
        locals.var_nsub_rv = 0.0;

        let assign2530_e1716: f64 = (1.6021918e-19 * locals.var_nsub);
        locals.var_q_nsub = assign2530_e1716;
        locals.var_q_nsub_dn0 = (1.6021918e-19 * locals.var_nsub_dn0);
        locals.var_q_nsub_dn2 = (1.6021918e-19 * locals.var_nsub_dn2);
        locals.var_q_nsub_dn4 = (1.6021918e-19 * locals.var_nsub_dn4);
        locals.var_q_nsub_dn5 = (1.6021918e-19 * locals.var_nsub_dn5);
        locals.var_q_nsub_dn6 = (1.6021918e-19 * locals.var_nsub_dn6);
        locals.var_q_nsub_dn8 = (1.6021918e-19 * locals.var_nsub_dn8);
        locals.var_q_nsub_dn10 = (1.6021918e-19 * locals.var_nsub_dn10);
        locals.var_q_nsub_dn11 = (1.6021918e-19 * locals.var_nsub_dn11);
        locals.var_q_nsub_dn12 = (1.6021918e-19 * locals.var_nsub_dn12);
        locals.var_q_nsub_rv = 0.0;

        let assign2540_e1719: f64 = (locals.var_q_nsub * 1.034943e-10);
        locals.var_qnsub_esi = assign2540_e1719;
        locals.var_qnsub_esi_dn0 = (locals.var_q_nsub_dn0 * 1.034943e-10);
        locals.var_qnsub_esi_dn2 = (locals.var_q_nsub_dn2 * 1.034943e-10);
        locals.var_qnsub_esi_dn4 = (locals.var_q_nsub_dn4 * 1.034943e-10);
        locals.var_qnsub_esi_dn5 = (locals.var_q_nsub_dn5 * 1.034943e-10);
        locals.var_qnsub_esi_dn6 = (locals.var_q_nsub_dn6 * 1.034943e-10);
        locals.var_qnsub_esi_dn8 = (locals.var_q_nsub_dn8 * 1.034943e-10);
        locals.var_qnsub_esi_dn10 = (locals.var_q_nsub_dn10 * 1.034943e-10);
        locals.var_qnsub_esi_dn11 = (locals.var_q_nsub_dn11 * 1.034943e-10);
        locals.var_qnsub_esi_dn12 = (locals.var_q_nsub_dn12 * 1.034943e-10);
        locals.var_qnsub_esi_rv = 0.0;

        let assign2550_e1722: f64 = (2.0 * locals.var_qnsub_esi);
        locals.var_qnsub_esi2 = assign2550_e1722;
        locals.var_qnsub_esi2_dn0 = (2.0 * locals.var_qnsub_esi_dn0);
        locals.var_qnsub_esi2_dn2 = (2.0 * locals.var_qnsub_esi_dn2);
        locals.var_qnsub_esi2_dn4 = (2.0 * locals.var_qnsub_esi_dn4);
        locals.var_qnsub_esi2_dn5 = (2.0 * locals.var_qnsub_esi_dn5);
        locals.var_qnsub_esi2_dn6 = (2.0 * locals.var_qnsub_esi_dn6);
        locals.var_qnsub_esi2_dn8 = (2.0 * locals.var_qnsub_esi_dn8);
        locals.var_qnsub_esi2_dn10 = (2.0 * locals.var_qnsub_esi_dn10);
        locals.var_qnsub_esi2_dn11 = (2.0 * locals.var_qnsub_esi_dn11);
        locals.var_qnsub_esi2_dn12 = (2.0 * locals.var_qnsub_esi_dn12);
        locals.var_qnsub_esi2_rv = 0.0;

        let assign2560_e1725: f64 = (1.6021918e-19 * locals.var_n_subbl);
        let assign2560_e1727: f64 = (assign2560_e1725 * 1.034943e-10);
        locals.var_qnbulk_esi = assign2560_e1727;
        locals.var_qnbulk_esi_dn0 = ((1.6021918e-19 * locals.var_n_subbl_dn0) * 1.034943e-10);
        locals.var_qnbulk_esi_dn2 = ((1.6021918e-19 * locals.var_n_subbl_dn2) * 1.034943e-10);
        locals.var_qnbulk_esi_dn4 = ((1.6021918e-19 * locals.var_n_subbl_dn4) * 1.034943e-10);
        locals.var_qnbulk_esi_dn5 = ((1.6021918e-19 * locals.var_n_subbl_dn5) * 1.034943e-10);
        locals.var_qnbulk_esi_dn6 = ((1.6021918e-19 * locals.var_n_subbl_dn6) * 1.034943e-10);
        locals.var_qnbulk_esi_dn8 = ((1.6021918e-19 * locals.var_n_subbl_dn8) * 1.034943e-10);
        locals.var_qnbulk_esi_dn10 = ((1.6021918e-19 * locals.var_n_subbl_dn10) * 1.034943e-10);
        locals.var_qnbulk_esi_dn11 = ((1.6021918e-19 * locals.var_n_subbl_dn11) * 1.034943e-10);
        locals.var_qnbulk_esi_dn12 = ((1.6021918e-19 * locals.var_n_subbl_dn12) * 1.034943e-10);
        locals.var_qnbulk_esi_rv = 0.0;

        let assign2570_e1731: f64 = (-p.p242);
        let assign2570_e1732: f64 = (locals.var_lg).powf(assign2570_e1731);
        let assign2570_e1733: f64 = (p.p239 * assign2570_e1732);
        locals.var_ptl0 = assign2570_e1733;
        locals.var_ptl0_rv = 0.0;

        let assign2580_e1737: f64 = (-p.p244);
        let assign2580_e1738: f64 = (locals.var_lg).powf(assign2580_e1737);
        let assign2580_e1739: f64 = (p.p243 * assign2580_e1738);
        locals.var_pt40 = assign2580_e1739;
        locals.var_pt40_rv = 0.0;

        let assign2590_e1743: f64 = (locals.var_lg + p.p248);
        let assign2590_e1745: f64 = (-p.p247);
        let assign2590_e1746: f64 = (assign2590_e1743).powf(assign2590_e1745);
        let assign2590_e1747: f64 = (p.p246 * assign2590_e1746);
        locals.var_gdl0 = assign2590_e1747;
        locals.var_gdl0_rv = 0.0;

        let assign2600_e1751: f64 = (2.0 * p.p58);
        let assign2600_e1756: f64 = if ((locals.var_lgate <= assign2600_e1751) && (p.p58 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard18 = assign2600_e1756;
        locals.var_guard18_rv = 0.0;

        let (assign2610_e1772, assign2610_e1772_d_n0, assign2610_e1772_d_n2, assign2610_e1772_d_n4, assign2610_e1772_d_n5, assign2610_e1772_d_n6, assign2610_e1772_d_n8, assign2610_e1772_d_n10, assign2610_e1772_d_n11, assign2610_e1772_d_n12,) = {
    if (locals.var_guard18 != 0.0) {
        let assign2610_e1760: f64 = (2.0 * locals.var_nsubps);
        let assign2610_e1763: f64 = (locals.var_nsubps - locals.var_uc_nsubs);
        let assign2610_e1765: f64 = (assign2610_e1763 * locals.var_lgate);
        let assign2610_e1767: f64 = (assign2610_e1765 / p.p58);
        let assign2610_e1768: f64 = (assign2610_e1760 - assign2610_e1767);
        let assign2610_e1770: f64 = (assign2610_e1768 - locals.var_uc_nsubs);
        (assign2610_e1770, (((2.0 * locals.var_nsubps_dn0) - (((locals.var_nsubps_dn0 - locals.var_uc_nsubs_dn0) * locals.var_lgate) / p.p58)) - locals.var_uc_nsubs_dn0), (((2.0 * locals.var_nsubps_dn2) - (((locals.var_nsubps_dn2 - locals.var_uc_nsubs_dn2) * locals.var_lgate) / p.p58)) - locals.var_uc_nsubs_dn2), (((2.0 * locals.var_nsubps_dn4) - (((locals.var_nsubps_dn4 - locals.var_uc_nsubs_dn4) * locals.var_lgate) / p.p58)) - locals.var_uc_nsubs_dn4), (((2.0 * locals.var_nsubps_dn5) - (((locals.var_nsubps_dn5 - locals.var_uc_nsubs_dn5) * locals.var_lgate) / p.p58)) - locals.var_uc_nsubs_dn5), (((2.0 * locals.var_nsubps_dn6) - (((locals.var_nsubps_dn6 - locals.var_uc_nsubs_dn6) * locals.var_lgate) / p.p58)) - locals.var_uc_nsubs_dn6), (((2.0 * locals.var_nsubps_dn8) - (((locals.var_nsubps_dn8 - locals.var_uc_nsubs_dn8) * locals.var_lgate) / p.p58)) - locals.var_uc_nsubs_dn8), (((2.0 * locals.var_nsubps_dn10) - (((locals.var_nsubps_dn10 - locals.var_uc_nsubs_dn10) * locals.var_lgate) / p.p58)) - locals.var_uc_nsubs_dn10), (((2.0 * locals.var_nsubps_dn11) - (((locals.var_nsubps_dn11 - locals.var_uc_nsubs_dn11) * locals.var_lgate) / p.p58)) - locals.var_uc_nsubs_dn11), (((2.0 * locals.var_nsubps_dn12) - (((locals.var_nsubps_dn12 - locals.var_uc_nsubs_dn12) * locals.var_lgate) / p.p58)) - locals.var_uc_nsubs_dn12),)
    } else {
        (locals.var_nsubb0, locals.var_nsubb0_dn0, locals.var_nsubb0_dn2, locals.var_nsubb0_dn4, locals.var_nsubb0_dn5, locals.var_nsubb0_dn6, locals.var_nsubb0_dn8, locals.var_nsubb0_dn10, locals.var_nsubb0_dn11, locals.var_nsubb0_dn12,)
    }
};
        locals.var_nsubb0 = assign2610_e1772;
        locals.var_nsubb0_dn0 = assign2610_e1772_d_n0;
        locals.var_nsubb0_dn2 = assign2610_e1772_d_n2;
        locals.var_nsubb0_dn4 = assign2610_e1772_d_n4;
        locals.var_nsubb0_dn5 = assign2610_e1772_d_n5;
        locals.var_nsubb0_dn6 = assign2610_e1772_d_n6;
        locals.var_nsubb0_dn8 = assign2610_e1772_d_n8;
        locals.var_nsubb0_dn10 = assign2610_e1772_d_n10;
        locals.var_nsubb0_dn11 = assign2610_e1772_d_n11;
        locals.var_nsubb0_dn12 = assign2610_e1772_d_n12;
        locals.var_nsubb0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_6(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (assign2620_e1779, assign2620_e1779_d_n0, assign2620_e1779_d_n2, assign2620_e1779_d_n4, assign2620_e1779_d_n5, assign2620_e1779_d_n6, assign2620_e1779_d_n8, assign2620_e1779_d_n10, assign2620_e1779_d_n11, assign2620_e1779_d_n12,) = {
    if (locals.var_guard18 != 0.0) {
        let assign2620_e1776: f64 = (locals.var_nsubb0 / locals.var_uc_nsubs);
        let assign2620_e1777: f64 = (assign2620_e1776).ln();
        (assign2620_e1777, ((((locals.var_nsubb0_dn0 * locals.var_uc_nsubs) - (locals.var_nsubb0 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / assign2620_e1776), ((((locals.var_nsubb0_dn2 * locals.var_uc_nsubs) - (locals.var_nsubb0 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / assign2620_e1776), ((((locals.var_nsubb0_dn4 * locals.var_uc_nsubs) - (locals.var_nsubb0 * locals.var_uc_nsubs_dn4)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / assign2620_e1776), ((((locals.var_nsubb0_dn5 * locals.var_uc_nsubs) - (locals.var_nsubb0 * locals.var_uc_nsubs_dn5)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / assign2620_e1776), ((((locals.var_nsubb0_dn6 * locals.var_uc_nsubs) - (locals.var_nsubb0 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / assign2620_e1776), ((((locals.var_nsubb0_dn8 * locals.var_uc_nsubs) - (locals.var_nsubb0 * locals.var_uc_nsubs_dn8)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / assign2620_e1776), ((((locals.var_nsubb0_dn10 * locals.var_uc_nsubs) - (locals.var_nsubb0 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / assign2620_e1776), ((((locals.var_nsubb0_dn11 * locals.var_uc_nsubs) - (locals.var_nsubb0 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / assign2620_e1776), ((((locals.var_nsubb0_dn12 * locals.var_uc_nsubs) - (locals.var_nsubb0 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / assign2620_e1776),)
    } else {
        (locals.var_ptovr0, locals.var_ptovr0_dn0, locals.var_ptovr0_dn2, locals.var_ptovr0_dn4, locals.var_ptovr0_dn5, locals.var_ptovr0_dn6, locals.var_ptovr0_dn8, locals.var_ptovr0_dn10, locals.var_ptovr0_dn11, locals.var_ptovr0_dn12,)
    }
};
        locals.var_ptovr0 = assign2620_e1779;
        locals.var_ptovr0_dn0 = assign2620_e1779_d_n0;
        locals.var_ptovr0_dn2 = assign2620_e1779_d_n2;
        locals.var_ptovr0_dn4 = assign2620_e1779_d_n4;
        locals.var_ptovr0_dn5 = assign2620_e1779_d_n5;
        locals.var_ptovr0_dn6 = assign2620_e1779_d_n6;
        locals.var_ptovr0_dn8 = assign2620_e1779_d_n8;
        locals.var_ptovr0_dn10 = assign2620_e1779_d_n10;
        locals.var_ptovr0_dn11 = assign2620_e1779_d_n11;
        locals.var_ptovr0_dn12 = assign2620_e1779_d_n12;
        locals.var_ptovr0_rv = 0.0;

        let (assign2630_e1784, assign2630_e1784_d_n0, assign2630_e1784_d_n2, assign2630_e1784_d_n4, assign2630_e1784_d_n5, assign2630_e1784_d_n6, assign2630_e1784_d_n8, assign2630_e1784_d_n10, assign2630_e1784_d_n11, assign2630_e1784_d_n12,) = {
    if (locals.var_guard18 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ptovr0, locals.var_ptovr0_dn0, locals.var_ptovr0_dn2, locals.var_ptovr0_dn4, locals.var_ptovr0_dn5, locals.var_ptovr0_dn6, locals.var_ptovr0_dn8, locals.var_ptovr0_dn10, locals.var_ptovr0_dn11, locals.var_ptovr0_dn12,)
    }
};
        locals.var_ptovr0 = assign2630_e1784;
        locals.var_ptovr0_dn0 = assign2630_e1784_d_n0;
        locals.var_ptovr0_dn2 = assign2630_e1784_d_n2;
        locals.var_ptovr0_dn4 = assign2630_e1784_d_n4;
        locals.var_ptovr0_dn5 = assign2630_e1784_d_n5;
        locals.var_ptovr0_dn6 = assign2630_e1784_d_n6;
        locals.var_ptovr0_dn8 = assign2630_e1784_d_n8;
        locals.var_ptovr0_dn10 = assign2630_e1784_d_n10;
        locals.var_ptovr0_dn11 = assign2630_e1784_d_n11;
        locals.var_ptovr0_dn12 = assign2630_e1784_d_n12;
        locals.var_ptovr0_rv = 0.0;

        let assign2640_e1787: f64 = (2.0 / 38.68283);
        let assign2640_e1790: f64 = (locals.var_nsub / 1.04e16);
        let assign2640_e1791: f64 = (assign2640_e1790).ln();
        let assign2640_e1792: f64 = (assign2640_e1787 * assign2640_e1791);
        locals.var_pb20 = assign2640_e1792;
        locals.var_pb20_dn0 = (assign2640_e1787 * ((locals.var_nsub_dn0 / 1.04e16) / assign2640_e1790));
        locals.var_pb20_dn2 = (assign2640_e1787 * ((locals.var_nsub_dn2 / 1.04e16) / assign2640_e1790));
        locals.var_pb20_dn4 = (assign2640_e1787 * ((locals.var_nsub_dn4 / 1.04e16) / assign2640_e1790));
        locals.var_pb20_dn5 = (assign2640_e1787 * ((locals.var_nsub_dn5 / 1.04e16) / assign2640_e1790));
        locals.var_pb20_dn6 = (assign2640_e1787 * ((locals.var_nsub_dn6 / 1.04e16) / assign2640_e1790));
        locals.var_pb20_dn8 = (assign2640_e1787 * ((locals.var_nsub_dn8 / 1.04e16) / assign2640_e1790));
        locals.var_pb20_dn10 = (assign2640_e1787 * ((locals.var_nsub_dn10 / 1.04e16) / assign2640_e1790));
        locals.var_pb20_dn11 = (assign2640_e1787 * ((locals.var_nsub_dn11 / 1.04e16) / assign2640_e1790));
        locals.var_pb20_dn12 = (assign2640_e1787 * ((locals.var_nsub_dn12 / 1.04e16) / assign2640_e1790));
        locals.var_pb20_rv = 0.0;

        let assign2650_e1795: f64 = (2.0 / 38.68283);
        let assign2650_e1798: f64 = (locals.var_uc_nsubs / 1.04e16);
        let assign2650_e1799: f64 = (assign2650_e1798).ln();
        let assign2650_e1800: f64 = (assign2650_e1795 * assign2650_e1799);
        locals.var_pb2c = assign2650_e1800;
        locals.var_pb2c_dn0 = (assign2650_e1795 * ((locals.var_uc_nsubs_dn0 / 1.04e16) / assign2650_e1798));
        locals.var_pb2c_dn2 = (assign2650_e1795 * ((locals.var_uc_nsubs_dn2 / 1.04e16) / assign2650_e1798));
        locals.var_pb2c_dn4 = (assign2650_e1795 * ((locals.var_uc_nsubs_dn4 / 1.04e16) / assign2650_e1798));
        locals.var_pb2c_dn5 = (assign2650_e1795 * ((locals.var_uc_nsubs_dn5 / 1.04e16) / assign2650_e1798));
        locals.var_pb2c_dn6 = (assign2650_e1795 * ((locals.var_uc_nsubs_dn6 / 1.04e16) / assign2650_e1798));
        locals.var_pb2c_dn8 = (assign2650_e1795 * ((locals.var_uc_nsubs_dn8 / 1.04e16) / assign2650_e1798));
        locals.var_pb2c_dn10 = (assign2650_e1795 * ((locals.var_uc_nsubs_dn10 / 1.04e16) / assign2650_e1798));
        locals.var_pb2c_dn11 = (assign2650_e1795 * ((locals.var_uc_nsubs_dn11 / 1.04e16) / assign2650_e1798));
        locals.var_pb2c_dn12 = (assign2650_e1795 * ((locals.var_uc_nsubs_dn12 / 1.04e16) / assign2650_e1798));
        locals.var_pb2c_rv = 0.0;

        let assign2660_e1804: f64 = (1.0 / locals.var_lg);
        let assign2660_e1805: f64 = (1.0 + assign2660_e1804);
        let assign2660_e1807: f64 = (assign2660_e1805).powf(p.p77);
        let assign2660_e1809: f64 = (assign2660_e1807 * p.p75);
        locals.var_cnstpgd = assign2660_e1809;
        locals.var_cnstpgd_rv = 0.0;

        let assign2670_e1812: f64 = (p.p116 * locals.var_lg);
        locals.var_t1 = assign2670_e1812;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign2680_e1815: f64 = (locals.var_t1 * p.p115);
        let assign2680_e1818: f64 = (locals.var_t1 + p.p115);
        let assign2680_e1819: f64 = (assign2680_e1815 / assign2680_e1818);
        let assign2680_e1821: f64 = (assign2680_e1819 + p.p117);
        let assign2680_e1823: f64 = (assign2680_e1821 + 1e-50);
        locals.var_ddlte = assign2680_e1823;
        locals.var_ddlte_dn0 = ((((locals.var_t1_dn0 * p.p115) * assign2680_e1818) - (assign2680_e1815 * locals.var_t1_dn0)) / (assign2680_e1818 * assign2680_e1818));
        locals.var_ddlte_dn2 = ((((locals.var_t1_dn2 * p.p115) * assign2680_e1818) - (assign2680_e1815 * locals.var_t1_dn2)) / (assign2680_e1818 * assign2680_e1818));
        locals.var_ddlte_dn4 = ((((locals.var_t1_dn4 * p.p115) * assign2680_e1818) - (assign2680_e1815 * locals.var_t1_dn4)) / (assign2680_e1818 * assign2680_e1818));
        locals.var_ddlte_dn5 = ((((locals.var_t1_dn5 * p.p115) * assign2680_e1818) - (assign2680_e1815 * locals.var_t1_dn5)) / (assign2680_e1818 * assign2680_e1818));
        locals.var_ddlte_dn6 = ((((locals.var_t1_dn6 * p.p115) * assign2680_e1818) - (assign2680_e1815 * locals.var_t1_dn6)) / (assign2680_e1818 * assign2680_e1818));
        locals.var_ddlte_dn8 = ((((locals.var_t1_dn8 * p.p115) * assign2680_e1818) - (assign2680_e1815 * locals.var_t1_dn8)) / (assign2680_e1818 * assign2680_e1818));
        locals.var_ddlte_dn10 = ((((locals.var_t1_dn10 * p.p115) * assign2680_e1818) - (assign2680_e1815 * locals.var_t1_dn10)) / (assign2680_e1818 * assign2680_e1818));
        locals.var_ddlte_dn11 = ((((locals.var_t1_dn11 * p.p115) * assign2680_e1818) - (assign2680_e1815 * locals.var_t1_dn11)) / (assign2680_e1818 * assign2680_e1818));
        locals.var_ddlte_dn12 = ((((locals.var_t1_dn12 * p.p115) * assign2680_e1818) - (assign2680_e1815 * locals.var_t1_dn12)) / (assign2680_e1818 * assign2680_e1818));
        locals.var_ddlte_rv = 0.0;

        let assign2690_e1827: f64 = (locals.var_lg).powf(p.p179);
        let assign2690_e1829: f64 = (assign2690_e1827 * p.p180);
        let assign2690_e1830: f64 = (1.0 + assign2690_e1829);
        locals.var_clmmod = assign2690_e1830;
        locals.var_clmmod_rv = 0.0;

        let assign2700_e1833: f64 = if p.p25 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard19 = assign2700_e1833;
        locals.var_guard19_rv = 0.0;

        let (assign2710_e1843, assign2710_e1843_d_n0, assign2710_e1843_d_n2, assign2710_e1843_d_n4, assign2710_e1843_d_n5, assign2710_e1843_d_n6, assign2710_e1843_d_n8, assign2710_e1843_d_n10, assign2710_e1843_d_n11, assign2710_e1843_d_n12,) = {
    if (locals.var_guard19 != 0.0) {
        let assign2710_e1839: f64 = (3.0 * p.p2);
        let assign2710_e1840: f64 = (locals.var_weff / assign2710_e1839);
        let assign2710_e1841: f64 = (p.p3 + assign2710_e1840);
        (assign2710_e1841, (locals.var_weff_dn0 / assign2710_e1839), (locals.var_weff_dn2 / assign2710_e1839), (locals.var_weff_dn4 / assign2710_e1839), (locals.var_weff_dn5 / assign2710_e1839), (locals.var_weff_dn6 / assign2710_e1839), (locals.var_weff_dn8 / assign2710_e1839), (locals.var_weff_dn10 / assign2710_e1839), (locals.var_weff_dn11 / assign2710_e1839), (locals.var_weff_dn12 / assign2710_e1839),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign2710_e1843;
        locals.var_t1_dn0 = assign2710_e1843_d_n0;
        locals.var_t1_dn2 = assign2710_e1843_d_n2;
        locals.var_t1_dn4 = assign2710_e1843_d_n4;
        locals.var_t1_dn5 = assign2710_e1843_d_n5;
        locals.var_t1_dn6 = assign2710_e1843_d_n6;
        locals.var_t1_dn8 = assign2710_e1843_d_n8;
        locals.var_t1_dn10 = assign2710_e1843_d_n10;
        locals.var_t1_dn11 = assign2710_e1843_d_n11;
        locals.var_t1_dn12 = assign2710_e1843_d_n12;
        locals.var_t1_rv = 0.0;

        let assign2770_e1889: f64 = (locals.var_wg).powf(p.p132);
        let assign2770_e1890: f64 = (p.p131 / assign2770_e1889);
        let assign2770_e1891: f64 = (1.0 + assign2770_e1890);
        locals.var_zvgs = assign2770_e1891;
        locals.var_zvgs_rv = 0.0;

        let assign2780_e1897: f64 = (locals.var_lg).powf(p.p127);
        let assign2780_e1898: f64 = (p.p126 / assign2780_e1897);
        let assign2780_e1899: f64 = (1.0 + assign2780_e1898);
        let assign2780_e1900: f64 = (p.p125 * assign2780_e1899);
        locals.var_xvbs = assign2780_e1900;
        locals.var_xvbs_rv = 0.0;

        let assign2790_e1904: f64 = (locals.var_lg + p.p124);
        let assign2790_e1905: f64 = (locals.var_lg / assign2790_e1904);
        locals.var_xgate = assign2790_e1905;
        locals.var_xgate_rv = 0.0;

        let assign2800_e1911: f64 = (locals.var_lg).powf(p.p121);
        let assign2800_e1912: f64 = (p.p120 / assign2800_e1911);
        let assign2800_e1913: f64 = (1.0 + assign2800_e1912);
        let assign2800_e1914: f64 = (p.p118 * assign2800_e1913);
        locals.var_xsub1 = assign2800_e1914;
        locals.var_xsub1_rv = 0.0;

        let assign2810_e1919: f64 = (p.p122 / locals.var_lg);
        let assign2810_e1920: f64 = (1.0 + assign2810_e1919);
        let assign2810_e1921: f64 = (p.p119 * assign2810_e1920);
        locals.var_xsub2 = assign2810_e1921;
        locals.var_xsub2_rv = 0.0;

        let assign2820_e1924: f64 = (10000.0 * locals.var_weffcv_nf);
        let assign2820_e1926: f64 = (assign2820_e1924 * p.p46);
        let assign2820_e1929: f64 = (locals.var_lg).powf(p.p47);
        let assign2820_e1930: f64 = (assign2820_e1926 / assign2820_e1929);
        locals.var_cqyb0 = assign2820_e1930;
        locals.var_cqyb0_dn0 = (((10000.0 * locals.var_weffcv_nf_dn0) * p.p46) / assign2820_e1929);
        locals.var_cqyb0_dn2 = (((10000.0 * locals.var_weffcv_nf_dn2) * p.p46) / assign2820_e1929);
        locals.var_cqyb0_dn4 = (((10000.0 * locals.var_weffcv_nf_dn4) * p.p46) / assign2820_e1929);
        locals.var_cqyb0_dn5 = (((10000.0 * locals.var_weffcv_nf_dn5) * p.p46) / assign2820_e1929);
        locals.var_cqyb0_dn6 = (((10000.0 * locals.var_weffcv_nf_dn6) * p.p46) / assign2820_e1929);
        locals.var_cqyb0_dn8 = (((10000.0 * locals.var_weffcv_nf_dn8) * p.p46) / assign2820_e1929);
        locals.var_cqyb0_dn10 = (((10000.0 * locals.var_weffcv_nf_dn10) * p.p46) / assign2820_e1929);
        locals.var_cqyb0_dn11 = (((10000.0 * locals.var_weffcv_nf_dn11) * p.p46) / assign2820_e1929);
        locals.var_cqyb0_dn12 = (((10000.0 * locals.var_weffcv_nf_dn12) * p.p46) / assign2820_e1929);
        locals.var_cqyb0_rv = 0.0;

        let assign2830_e1936: f64 = (locals.var_lg).powf(p.p135);
        let assign2830_e1937: f64 = (p.p134 / assign2830_e1936);
        let assign2830_e1938: f64 = (1.0 + assign2830_e1937);
        let assign2830_e1939: f64 = (p.p133 * assign2830_e1938);
        locals.var_vfbsub0 = assign2830_e1939;
        locals.var_vfbsub0_rv = 0.0;

        let assign2840_e1945: f64 = (locals.var_lg).powf(p.p130);
        let assign2840_e1946: f64 = (p.p129 / assign2840_e1945);
        let assign2840_e1947: f64 = (1.0 + assign2840_e1946);
        let assign2840_e1948: f64 = (p.p128 * assign2840_e1947);
        locals.var_uc_svgs = assign2840_e1948;
        locals.var_uc_svgs_rv = 0.0;

        let assign2850_e1951: f64 = (2.0 * 1.034943e-10);
        let assign2850_e1953: f64 = (assign2850_e1951 / 1.6021918e-19);
        locals.var_t1 = assign2850_e1953;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign2860_e1956: f64 = (locals.var_t1 / locals.var_nsub);
        let assign2860_e1957: f64 = (assign2860_e1956).sqrt();
        locals.var_wdpl = assign2860_e1957;
        locals.var_wdpl_dn0 = ((((locals.var_t1_dn0 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn0)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign2860_e1957));
        locals.var_wdpl_dn2 = ((((locals.var_t1_dn2 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn2)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign2860_e1957));
        locals.var_wdpl_dn4 = ((((locals.var_t1_dn4 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn4)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign2860_e1957));
        locals.var_wdpl_dn5 = ((((locals.var_t1_dn5 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn5)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign2860_e1957));
        locals.var_wdpl_dn6 = ((((locals.var_t1_dn6 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn6)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign2860_e1957));
        locals.var_wdpl_dn8 = ((((locals.var_t1_dn8 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn8)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign2860_e1957));
        locals.var_wdpl_dn10 = ((((locals.var_t1_dn10 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn10)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign2860_e1957));
        locals.var_wdpl_dn11 = ((((locals.var_t1_dn11 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn11)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign2860_e1957));
        locals.var_wdpl_dn12 = ((((locals.var_t1_dn12 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn12)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign2860_e1957));
        locals.var_wdpl_rv = 0.0;

        let assign2870_e1960: f64 = (p.p33 * (nv5 - nv12));
        locals.var_vgsi = assign2870_e1960;
        locals.var_vgsi_dn5 = p.p33;
        locals.var_vgsi_dn12 = (-p.p33);
        locals.var_vgsi_rv = 0.0;

        let assign2880_e1963: f64 = (p.p33 * (nv11 - nv12));
        locals.var_vdsi = assign2880_e1963;
        locals.var_vdsi_dn11 = p.p33;
        locals.var_vdsi_dn12 = (-p.p33);
        locals.var_vdsi_rv = 0.0;

        let assign2890_e1966: f64 = (p.p33 * (nv6 - nv12));
        locals.var_vbsi = assign2890_e1966;
        locals.var_vbsi_dn6 = p.p33;
        locals.var_vbsi_dn12 = (-p.p33);
        locals.var_vbsi_rv = 0.0;

        let assign2900_e1969: f64 = (p.p33 * (nv5 - nv2));
        locals.var_vgsei = assign2900_e1969;
        locals.var_vgsei_dn2 = (-p.p33);
        locals.var_vgsei_dn5 = p.p33;
        locals.var_vgsei_rv = 0.0;

        let assign2910_e1972: f64 = (p.p33 * (nv0 - nv2));
        locals.var_vdsei = assign2910_e1972;
        locals.var_vdsei_dn0 = p.p33;
        locals.var_vdsei_dn2 = (-p.p33);
        locals.var_vdsei_rv = 0.0;

        let assign2920_e1975: f64 = (p.p33 * (nv6 - nv2));
        locals.var_vbsei = assign2920_e1975;
        locals.var_vbsei_dn2 = (-p.p33);
        locals.var_vbsei_dn6 = p.p33;
        locals.var_vbsei_rv = 0.0;

        let assign2930_e1980: f64 = if ((p.p28 != 0.0) && (p.p237 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard21 = assign2930_e1980;
        locals.var_guard21_rv = 0.0;

        let (assign2940_e1989, assign2940_e1989_d_n4,) = {
    if (locals.var_guard21 != 0.0) {
        let (assign2940_e1987, assign2940_e1987_d_n4,) = {
            if ((nv4 - 0.0) > 0.0) {
                ((nv4 - 0.0), 1.0,)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign2940_e1987, assign2940_e1987_d_n4,)
    } else {
        (locals.var_deltemp, locals.var_deltemp_dn4,)
    }
};
        locals.var_deltemp = assign2940_e1989;
        locals.var_deltemp_dn4 = assign2940_e1989_d_n4;
        locals.var_deltemp_rv = 0.0;

        let (assign2950_e1994, assign2950_e1994_d_n4,) = {
    if (locals.var_guard21 == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_deltemp, locals.var_deltemp_dn4,)
    }
};
        locals.var_deltemp = assign2950_e1994;
        locals.var_deltemp_dn4 = assign2950_e1994_d_n4;
        locals.var_deltemp_rv = 0.0;

        let (assign2960_e2000, assign2960_e2000_d_n8,) = {
    if (locals.var_flg_nqs != 0.0) {
        let assign2960_e1998: f64 = (1e-9 * (nv8 - 0.0));
        (assign2960_e1998, 1e-9,)
    } else {
        (locals.var_qi_nqs, locals.var_qi_nqs_dn8,)
    }
};
        locals.var_qi_nqs = assign2960_e2000;
        locals.var_qi_nqs_dn8 = assign2960_e2000_d_n8;
        locals.var_qi_nqs_rv = 0.0;

        let (assign2970_e2006, assign2970_e2006_d_n9,) = {
    if (locals.var_flg_nqs != 0.0) {
        let assign2970_e2004: f64 = (1e-9 * (nv9 - 0.0));
        (assign2970_e2004, 1e-9,)
    } else {
        (locals.var_qb_nqs, locals.var_qb_nqs_dn9,)
    }
};
        locals.var_qb_nqs = assign2970_e2006;
        locals.var_qb_nqs_dn9 = assign2970_e2006_d_n9;
        locals.var_qb_nqs_rv = 0.0;

        let (assign2980_e2011, assign2980_e2011_d_n8,) = {
    if (locals.var_flg_nqs == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_qi_nqs, locals.var_qi_nqs_dn8,)
    }
};
        locals.var_qi_nqs = assign2980_e2011;
        locals.var_qi_nqs_dn8 = assign2980_e2011_d_n8;
        locals.var_qi_nqs_rv = 0.0;

        let (assign2990_e2016, assign2990_e2016_d_n9,) = {
    if (locals.var_flg_nqs == 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_qb_nqs, locals.var_qb_nqs_dn9,)
    }
};
        locals.var_qb_nqs = assign2990_e2016;
        locals.var_qb_nqs_dn9 = assign2990_e2016_d_n9;
        locals.var_qb_nqs_rv = 0.0;

        let assign3000_e2019: f64 = if locals.var_vdsi >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard22 = assign3000_e2019;
        locals.var_guard22_rv = 0.0;

        let (assign3010_e2023,) = {
    if (locals.var_guard22 != 0.0) {
        (1.0,)
    } else {
        (locals.var_mode,)
    }
};
        locals.var_mode = assign3010_e2023;
        locals.var_mode_rv = 0.0;

        let (assign3020_e2027,) = {
    if (locals.var_guard22 != 0.0) {
        (1.0,)
    } else {
        (locals.var_modenml,)
    }
};
        locals.var_modenml = assign3020_e2027;
        locals.var_modenml_rv = 0.0;

        let (assign3030_e2031,) = {
    if (locals.var_guard22 != 0.0) {
        (0.0,)
    } else {
        (locals.var_modervs,)
    }
};
        locals.var_modervs = assign3030_e2031;
        locals.var_modervs_rv = 0.0;

        let (assign3040_e2035, assign3040_e2035_d_n5, assign3040_e2035_d_n11, assign3040_e2035_d_n12,) = {
    if (locals.var_guard22 != 0.0) {
        (locals.var_vgsi, locals.var_vgsi_dn5, 0.0, locals.var_vgsi_dn12,)
    } else {
        (locals.var_vgs_mos, locals.var_vgs_mos_dn5, locals.var_vgs_mos_dn11, locals.var_vgs_mos_dn12,)
    }
};
        locals.var_vgs_mos = assign3040_e2035;
        locals.var_vgs_mos_dn5 = assign3040_e2035_d_n5;
        locals.var_vgs_mos_dn11 = assign3040_e2035_d_n11;
        locals.var_vgs_mos_dn12 = assign3040_e2035_d_n12;
        locals.var_vgs_mos_rv = 0.0;

        let (assign3050_e2039, assign3050_e2039_d_n11, assign3050_e2039_d_n12,) = {
    if (locals.var_guard22 != 0.0) {
        (locals.var_vdsi, locals.var_vdsi_dn11, locals.var_vdsi_dn12,)
    } else {
        (locals.var_vds_mos, locals.var_vds_mos_dn11, locals.var_vds_mos_dn12,)
    }
};
        locals.var_vds_mos = assign3050_e2039;
        locals.var_vds_mos_dn11 = assign3050_e2039_d_n11;
        locals.var_vds_mos_dn12 = assign3050_e2039_d_n12;
        locals.var_vds_mos_rv = 0.0;

        let (assign3060_e2043, assign3060_e2043_d_n6, assign3060_e2043_d_n11, assign3060_e2043_d_n12,) = {
    if (locals.var_guard22 != 0.0) {
        (locals.var_vbsi, locals.var_vbsi_dn6, 0.0, locals.var_vbsi_dn12,)
    } else {
        (locals.var_vbs_mos, locals.var_vbs_mos_dn6, locals.var_vbs_mos_dn11, locals.var_vbs_mos_dn12,)
    }
};
        locals.var_vbs_mos = assign3060_e2043;
        locals.var_vbs_mos_dn6 = assign3060_e2043_d_n6;
        locals.var_vbs_mos_dn11 = assign3060_e2043_d_n11;
        locals.var_vbs_mos_dn12 = assign3060_e2043_d_n12;
        locals.var_vbs_mos_rv = 0.0;

        let (assign3070_e2047, assign3070_e2047_d_n0, assign3070_e2047_d_n2, assign3070_e2047_d_n5,) = {
    if (locals.var_guard22 != 0.0) {
        (locals.var_vgsei, 0.0, locals.var_vgsei_dn2, locals.var_vgsei_dn5,)
    } else {
        (locals.var_vgse, locals.var_vgse_dn0, locals.var_vgse_dn2, locals.var_vgse_dn5,)
    }
};
        locals.var_vgse = assign3070_e2047;
        locals.var_vgse_dn0 = assign3070_e2047_d_n0;
        locals.var_vgse_dn2 = assign3070_e2047_d_n2;
        locals.var_vgse_dn5 = assign3070_e2047_d_n5;
        locals.var_vgse_rv = 0.0;

        let (assign3080_e2051, assign3080_e2051_d_n0, assign3080_e2051_d_n2,) = {
    if (locals.var_guard22 != 0.0) {
        (locals.var_vdsei, locals.var_vdsei_dn0, locals.var_vdsei_dn2,)
    } else {
        (locals.var_vdse, locals.var_vdse_dn0, locals.var_vdse_dn2,)
    }
};
        locals.var_vdse = assign3080_e2051;
        locals.var_vdse_dn0 = assign3080_e2051_d_n0;
        locals.var_vdse_dn2 = assign3080_e2051_d_n2;
        locals.var_vdse_rv = 0.0;

        let (assign3090_e2055, assign3090_e2055_d_n0, assign3090_e2055_d_n2, assign3090_e2055_d_n6,) = {
    if (locals.var_guard22 != 0.0) {
        (locals.var_vbsei, 0.0, locals.var_vbsei_dn2, locals.var_vbsei_dn6,)
    } else {
        (locals.var_vbse, locals.var_vbse_dn0, locals.var_vbse_dn2, locals.var_vbse_dn6,)
    }
};
        locals.var_vbse = assign3090_e2055;
        locals.var_vbse_dn0 = assign3090_e2055_d_n0;
        locals.var_vbse_dn2 = assign3090_e2055_d_n2;
        locals.var_vbse_dn6 = assign3090_e2055_d_n6;
        locals.var_vbse_rv = 0.0;

        let (assign3100_e2061,) = {
    if (locals.var_guard22 == 0.0) {
        let assign3100_e2059: f64 = (-1.0);
        (assign3100_e2059,)
    } else {
        (locals.var_mode,)
    }
};
        locals.var_mode = assign3100_e2061;
        locals.var_mode_rv = 0.0;

        let (assign3110_e2066,) = {
    if (locals.var_guard22 == 0.0) {
        (0.0,)
    } else {
        (locals.var_modenml,)
    }
};
        locals.var_modenml = assign3110_e2066;
        locals.var_modenml_rv = 0.0;

        let (assign3120_e2071,) = {
    if (locals.var_guard22 == 0.0) {
        (1.0,)
    } else {
        (locals.var_modervs,)
    }
};
        locals.var_modervs = assign3120_e2071;
        locals.var_modervs_rv = 0.0;

        let (assign3130_e2078, assign3130_e2078_d_n5, assign3130_e2078_d_n11, assign3130_e2078_d_n12,) = {
    if (locals.var_guard22 == 0.0) {
        let assign3130_e2076: f64 = (locals.var_vgsi - locals.var_vdsi);
        (assign3130_e2076, locals.var_vgsi_dn5, (-locals.var_vdsi_dn11), (locals.var_vgsi_dn12 - locals.var_vdsi_dn12),)
    } else {
        (locals.var_vgs_mos, locals.var_vgs_mos_dn5, locals.var_vgs_mos_dn11, locals.var_vgs_mos_dn12,)
    }
};
        locals.var_vgs_mos = assign3130_e2078;
        locals.var_vgs_mos_dn5 = assign3130_e2078_d_n5;
        locals.var_vgs_mos_dn11 = assign3130_e2078_d_n11;
        locals.var_vgs_mos_dn12 = assign3130_e2078_d_n12;
        locals.var_vgs_mos_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_7(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let (assign3140_e2084, assign3140_e2084_d_n11, assign3140_e2084_d_n12,) = {
    if (locals.var_guard22 == 0.0) {
        let assign3140_e2082: f64 = (-locals.var_vdsi);
        (assign3140_e2082, (-locals.var_vdsi_dn11), (-locals.var_vdsi_dn12),)
    } else {
        (locals.var_vds_mos, locals.var_vds_mos_dn11, locals.var_vds_mos_dn12,)
    }
};
        locals.var_vds_mos = assign3140_e2084;
        locals.var_vds_mos_dn11 = assign3140_e2084_d_n11;
        locals.var_vds_mos_dn12 = assign3140_e2084_d_n12;
        locals.var_vds_mos_rv = 0.0;

        let (assign3150_e2091, assign3150_e2091_d_n6, assign3150_e2091_d_n11, assign3150_e2091_d_n12,) = {
    if (locals.var_guard22 == 0.0) {
        let assign3150_e2089: f64 = (locals.var_vbsi - locals.var_vdsi);
        (assign3150_e2089, locals.var_vbsi_dn6, (-locals.var_vdsi_dn11), (locals.var_vbsi_dn12 - locals.var_vdsi_dn12),)
    } else {
        (locals.var_vbs_mos, locals.var_vbs_mos_dn6, locals.var_vbs_mos_dn11, locals.var_vbs_mos_dn12,)
    }
};
        locals.var_vbs_mos = assign3150_e2091;
        locals.var_vbs_mos_dn6 = assign3150_e2091_d_n6;
        locals.var_vbs_mos_dn11 = assign3150_e2091_d_n11;
        locals.var_vbs_mos_dn12 = assign3150_e2091_d_n12;
        locals.var_vbs_mos_rv = 0.0;

        let (assign3160_e2098, assign3160_e2098_d_n0, assign3160_e2098_d_n2, assign3160_e2098_d_n5,) = {
    if (locals.var_guard22 == 0.0) {
        let assign3160_e2096: f64 = (locals.var_vgsei - locals.var_vdsei);
        (assign3160_e2096, (-locals.var_vdsei_dn0), (locals.var_vgsei_dn2 - locals.var_vdsei_dn2), locals.var_vgsei_dn5,)
    } else {
        (locals.var_vgse, locals.var_vgse_dn0, locals.var_vgse_dn2, locals.var_vgse_dn5,)
    }
};
        locals.var_vgse = assign3160_e2098;
        locals.var_vgse_dn0 = assign3160_e2098_d_n0;
        locals.var_vgse_dn2 = assign3160_e2098_d_n2;
        locals.var_vgse_dn5 = assign3160_e2098_d_n5;
        locals.var_vgse_rv = 0.0;

        let (assign3170_e2104, assign3170_e2104_d_n0, assign3170_e2104_d_n2,) = {
    if (locals.var_guard22 == 0.0) {
        let assign3170_e2102: f64 = (-locals.var_vdsei);
        (assign3170_e2102, (-locals.var_vdsei_dn0), (-locals.var_vdsei_dn2),)
    } else {
        (locals.var_vdse, locals.var_vdse_dn0, locals.var_vdse_dn2,)
    }
};
        locals.var_vdse = assign3170_e2104;
        locals.var_vdse_dn0 = assign3170_e2104_d_n0;
        locals.var_vdse_dn2 = assign3170_e2104_d_n2;
        locals.var_vdse_rv = 0.0;

        let (assign3180_e2111, assign3180_e2111_d_n0, assign3180_e2111_d_n2, assign3180_e2111_d_n6,) = {
    if (locals.var_guard22 == 0.0) {
        let assign3180_e2109: f64 = (locals.var_vbsei - locals.var_vdsei);
        (assign3180_e2109, (-locals.var_vdsei_dn0), (locals.var_vbsei_dn2 - locals.var_vdsei_dn2), locals.var_vbsei_dn6,)
    } else {
        (locals.var_vbse, locals.var_vbse_dn0, locals.var_vbse_dn2, locals.var_vbse_dn6,)
    }
};
        locals.var_vbse = assign3180_e2111;
        locals.var_vbse_dn0 = assign3180_e2111_d_n0;
        locals.var_vbse_dn2 = assign3180_e2111_d_n2;
        locals.var_vbse_dn6 = assign3180_e2111_d_n6;
        locals.var_vbse_rv = 0.0;

        let assign3210_e2118: f64 = ctx_temp;
        locals.var_ttemp = assign3210_e2118;
        locals.var_ttemp_dn4 = 0.0;
        locals.var_ttemp_rv = 0.0;

        let (assign3220_e2122, assign3220_e2122_d_n4,) = {
    if (locals.var_temp_given != 0.0) {
        (locals.var_uc_temp, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn4,)
    }
};
        locals.var_ttemp = assign3220_e2122;
        locals.var_ttemp_dn4 = assign3220_e2122_d_n4;
        locals.var_ttemp_rv = 0.0;

        let assign3230_e2125: f64 = (locals.var_ttemp + p.p10);
        let assign3230_e2127: f64 = (assign3230_e2125 + locals.var_deltemp);
        locals.var_ttemp = assign3230_e2127;
        locals.var_ttemp_dn4 = (locals.var_ttemp_dn4 + locals.var_deltemp_dn4);
        locals.var_ttemp_rv = 0.0;

        let assign3240_e2133: f64 = (locals.var_uc_tnom * 1e-7);
        let assign3240_e2134: f64 = (9.025e-5 + assign3240_e2133);
        let assign3240_e2135: f64 = (locals.var_uc_tnom * assign3240_e2134);
        let assign3240_e2136: f64 = (p.p37 - assign3240_e2135);
        locals.var_egtnom = assign3240_e2136;
        locals.var_egtnom_rv = 0.0;

        let assign3250_e2139: f64 = (locals.var_ttemp * locals.var_ttemp);
        let assign3250_e2142: f64 = (locals.var_uc_tnom * locals.var_uc_tnom);
        let assign3250_e2143: f64 = (assign3250_e2139 - assign3250_e2142);
        locals.var_t1 = assign3250_e2143;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = ((locals.var_ttemp_dn4 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn4));
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign3260_e2148: f64 = (locals.var_ttemp - locals.var_uc_tnom);
        let assign3260_e2149: f64 = (p.p35 * assign3260_e2148);
        let assign3260_e2150: f64 = (locals.var_egtnom - assign3260_e2149);
        let assign3260_e2153: f64 = (p.p36 * locals.var_t1);
        let assign3260_e2154: f64 = (assign3260_e2150 - assign3260_e2153);
        locals.var_eg = assign3260_e2154;
        locals.var_eg_dn0 = (-(p.p36 * locals.var_t1_dn0));
        locals.var_eg_dn2 = (-(p.p36 * locals.var_t1_dn2));
        locals.var_eg_dn4 = ((-(p.p35 * locals.var_ttemp_dn4)) - (p.p36 * locals.var_t1_dn4));
        locals.var_eg_dn5 = (-(p.p36 * locals.var_t1_dn5));
        locals.var_eg_dn6 = (-(p.p36 * locals.var_t1_dn6));
        locals.var_eg_dn8 = (-(p.p36 * locals.var_t1_dn8));
        locals.var_eg_dn10 = (-(p.p36 * locals.var_t1_dn10));
        locals.var_eg_dn11 = (-(p.p36 * locals.var_t1_dn11));
        locals.var_eg_dn12 = (-(p.p36 * locals.var_t1_dn12));
        locals.var_eg_rv = 0.0;

        let assign3270_e2158: f64 = (1.3806226e-23 * locals.var_ttemp);
        let assign3270_e2159: f64 = (1.6021918e-19 / assign3270_e2158);
        locals.var_beta = assign3270_e2159;
        locals.var_beta_dn4 = (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn4)) / (assign3270_e2158 * assign3270_e2158)));
        locals.var_beta_rv = 0.0;

        let assign3280_e2162: f64 = (locals.var_beta * locals.var_beta);
        locals.var_beta2 = assign3280_e2162;
        locals.var_beta2_dn4 = ((locals.var_beta_dn4 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn4));
        locals.var_beta2_rv = 0.0;

        let assign3290_e2165: f64 = (1.0 / locals.var_beta);
        locals.var_beta_inv = assign3290_e2165;
        locals.var_beta_inv_dn4 = (-(locals.var_beta_dn4 / (locals.var_beta * locals.var_beta)));
        locals.var_beta_inv_rv = 0.0;

        let assign3300_e2169: f64 = (1.3806226e-23 * locals.var_uc_tnom);
        let assign3300_e2170: f64 = (1.6021918e-19 / assign3300_e2169);
        locals.var_betatnom = assign3300_e2170;
        locals.var_betatnom_rv = 0.0;

        let assign3320_e2183: f64 = (locals.var_wg).powf(p.p96);
        let assign3320_e2184: f64 = (p.p95 / assign3320_e2183);
        let assign3320_e2185: f64 = (1.0 + assign3320_e2184);
        let assign3320_e2186: f64 = (p.p249 * assign3320_e2185);
        let assign3320_e2191: f64 = (locals.var_lg).powf(p.p98);
        let assign3320_e2192: f64 = (p.p97 / assign3320_e2191);
        let assign3320_e2193: f64 = (1.0 + assign3320_e2192);
        let assign3320_e2194: f64 = (assign3320_e2186 * assign3320_e2193);
        let assign3320_e2199: f64 = (locals.var_wlg).powf(p.p100);
        let assign3320_e2200: f64 = (p.p99 / assign3320_e2199);
        let assign3320_e2201: f64 = (1.0 + assign3320_e2200);
        let assign3320_e2202: f64 = (assign3320_e2194 * assign3320_e2201);
        locals.var_cgs_mueph = assign3320_e2202;
        locals.var_cgs_mueph_dn0 = 0.0;
        locals.var_cgs_mueph_dn2 = 0.0;
        locals.var_cgs_mueph_dn4 = 0.0;
        locals.var_cgs_mueph_dn5 = 0.0;
        locals.var_cgs_mueph_dn6 = 0.0;
        locals.var_cgs_mueph_dn8 = 0.0;
        locals.var_cgs_mueph_dn10 = 0.0;
        locals.var_cgs_mueph_dn11 = 0.0;
        locals.var_cgs_mueph_dn12 = 0.0;
        locals.var_cgs_mueph_rv = 0.0;

        let assign3330_e2208: f64 = (locals.var_wg).powf(p.p278);
        let assign3330_e2209: f64 = (p.p277 / assign3330_e2208);
        let assign3330_e2210: f64 = (1.0 + assign3330_e2209);
        let assign3330_e2211: f64 = (p.p276 * assign3330_e2210);
        let assign3330_e2216: f64 = (locals.var_lg).powf(p.p282);
        let assign3330_e2217: f64 = (p.p281 / assign3330_e2216);
        let assign3330_e2218: f64 = (1.0 + assign3330_e2217);
        let assign3330_e2219: f64 = (assign3330_e2211 * assign3330_e2218);
        let assign3330_e2224: f64 = (locals.var_wlg).powf(p.p280);
        let assign3330_e2225: f64 = (p.p279 / assign3330_e2224);
        let assign3330_e2226: f64 = (1.0 + assign3330_e2225);
        let assign3330_e2227: f64 = (assign3330_e2219 * assign3330_e2226);
        locals.var_cgs_muephb = assign3330_e2227;
        locals.var_cgs_muephb_dn0 = 0.0;
        locals.var_cgs_muephb_dn2 = 0.0;
        locals.var_cgs_muephb_dn4 = 0.0;
        locals.var_cgs_muephb_dn5 = 0.0;
        locals.var_cgs_muephb_dn6 = 0.0;
        locals.var_cgs_muephb_dn8 = 0.0;
        locals.var_cgs_muephb_dn10 = 0.0;
        locals.var_cgs_muephb_dn11 = 0.0;
        locals.var_cgs_muephb_dn12 = 0.0;
        locals.var_cgs_muephb_rv = 0.0;

        let assign3340_e2230: f64 = if locals.var_lod_half > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard30 = assign3340_e2230;
        locals.var_guard30_rv = 0.0;

        let (assign3350_e2238, assign3350_e2238_d_n4,) = {
    if (locals.var_guard30 != 0.0) {
        let assign3350_e2235: f64 = (1.0 + p.p163);
        let assign3350_e2236: f64 = (1.0 / assign3350_e2235);
        (assign3350_e2236, 0.0,)
    } else {
        (locals.var_t1__blk27, locals.var_t1__blk27_dn4,)
    }
};
        locals.var_t1__blk27 = assign3350_e2238;
        locals.var_t1__blk27_dn4 = assign3350_e2238_d_n4;
        locals.var_t1__blk27_rv = 0.0;

        let (assign3360_e2246, assign3360_e2246_d_n0, assign3360_e2246_d_n2, assign3360_e2246_d_n4, assign3360_e2246_d_n5, assign3360_e2246_d_n6, assign3360_e2246_d_n8, assign3360_e2246_d_n10, assign3360_e2246_d_n11, assign3360_e2246_d_n12,) = {
    if (locals.var_guard30 != 0.0) {
        let assign3360_e2242: f64 = (p.p162 / locals.var_lod_half);
        let assign3360_e2244: f64 = (assign3360_e2242).powf(p.p164);
        (assign3360_e2244, if 0.0 == 0.0 && ((p.p164) as f64).is_finite() && ((p.p164) as f64).fract() == 0.0 { if p.p164 == 0.0 { 0.0 } else { (p.p164 * ((assign3360_e2242).powf(p.p164 - 1.0) * (-((p.p162 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign3360_e2244 * (p.p164 * ((-((p.p162 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))) / assign3360_e2242))) }, if 0.0 == 0.0 && ((p.p164) as f64).is_finite() && ((p.p164) as f64).fract() == 0.0 { if p.p164 == 0.0 { 0.0 } else { (p.p164 * ((assign3360_e2242).powf(p.p164 - 1.0) * (-((p.p162 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign3360_e2244 * (p.p164 * ((-((p.p162 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))) / assign3360_e2242))) }, if 0.0 == 0.0 && ((p.p164) as f64).is_finite() && ((p.p164) as f64).fract() == 0.0 { if p.p164 == 0.0 { 0.0 } else { (p.p164 * ((assign3360_e2242).powf(p.p164 - 1.0) * (-((p.p162 * locals.var_lod_half_dn4) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign3360_e2244 * (p.p164 * ((-((p.p162 * locals.var_lod_half_dn4) / (locals.var_lod_half * locals.var_lod_half))) / assign3360_e2242))) }, if 0.0 == 0.0 && ((p.p164) as f64).is_finite() && ((p.p164) as f64).fract() == 0.0 { if p.p164 == 0.0 { 0.0 } else { (p.p164 * ((assign3360_e2242).powf(p.p164 - 1.0) * (-((p.p162 * locals.var_lod_half_dn5) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign3360_e2244 * (p.p164 * ((-((p.p162 * locals.var_lod_half_dn5) / (locals.var_lod_half * locals.var_lod_half))) / assign3360_e2242))) }, if 0.0 == 0.0 && ((p.p164) as f64).is_finite() && ((p.p164) as f64).fract() == 0.0 { if p.p164 == 0.0 { 0.0 } else { (p.p164 * ((assign3360_e2242).powf(p.p164 - 1.0) * (-((p.p162 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign3360_e2244 * (p.p164 * ((-((p.p162 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))) / assign3360_e2242))) }, if 0.0 == 0.0 && ((p.p164) as f64).is_finite() && ((p.p164) as f64).fract() == 0.0 { if p.p164 == 0.0 { 0.0 } else { (p.p164 * ((assign3360_e2242).powf(p.p164 - 1.0) * (-((p.p162 * locals.var_lod_half_dn8) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign3360_e2244 * (p.p164 * ((-((p.p162 * locals.var_lod_half_dn8) / (locals.var_lod_half * locals.var_lod_half))) / assign3360_e2242))) }, if 0.0 == 0.0 && ((p.p164) as f64).is_finite() && ((p.p164) as f64).fract() == 0.0 { if p.p164 == 0.0 { 0.0 } else { (p.p164 * ((assign3360_e2242).powf(p.p164 - 1.0) * (-((p.p162 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign3360_e2244 * (p.p164 * ((-((p.p162 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))) / assign3360_e2242))) }, if 0.0 == 0.0 && ((p.p164) as f64).is_finite() && ((p.p164) as f64).fract() == 0.0 { if p.p164 == 0.0 { 0.0 } else { (p.p164 * ((assign3360_e2242).powf(p.p164 - 1.0) * (-((p.p162 * locals.var_lod_half_dn11) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign3360_e2244 * (p.p164 * ((-((p.p162 * locals.var_lod_half_dn11) / (locals.var_lod_half * locals.var_lod_half))) / assign3360_e2242))) }, if 0.0 == 0.0 && ((p.p164) as f64).is_finite() && ((p.p164) as f64).fract() == 0.0 { if p.p164 == 0.0 { 0.0 } else { (p.p164 * ((assign3360_e2242).powf(p.p164 - 1.0) * (-((p.p162 * locals.var_lod_half_dn12) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign3360_e2244 * (p.p164 * ((-((p.p162 * locals.var_lod_half_dn12) / (locals.var_lod_half * locals.var_lod_half))) / assign3360_e2242))) },)
    } else {
        (locals.var_t2__blk28, locals.var_t2__blk28_dn0, locals.var_t2__blk28_dn2, locals.var_t2__blk28_dn4, locals.var_t2__blk28_dn5, locals.var_t2__blk28_dn6, locals.var_t2__blk28_dn8, locals.var_t2__blk28_dn10, locals.var_t2__blk28_dn11, locals.var_t2__blk28_dn12,)
    }
};
        locals.var_t2__blk28 = assign3360_e2246;
        locals.var_t2__blk28_dn0 = assign3360_e2246_d_n0;
        locals.var_t2__blk28_dn2 = assign3360_e2246_d_n2;
        locals.var_t2__blk28_dn4 = assign3360_e2246_d_n4;
        locals.var_t2__blk28_dn5 = assign3360_e2246_d_n5;
        locals.var_t2__blk28_dn6 = assign3360_e2246_d_n6;
        locals.var_t2__blk28_dn8 = assign3360_e2246_d_n8;
        locals.var_t2__blk28_dn10 = assign3360_e2246_d_n10;
        locals.var_t2__blk28_dn11 = assign3360_e2246_d_n11;
        locals.var_t2__blk28_dn12 = assign3360_e2246_d_n12;
        locals.var_t2__blk28_rv = 0.0;

        let (assign3370_e2254, assign3370_e2254_d_n0, assign3370_e2254_d_n2, assign3370_e2254_d_n4, assign3370_e2254_d_n5, assign3370_e2254_d_n6, assign3370_e2254_d_n8, assign3370_e2254_d_n10, assign3370_e2254_d_n11, assign3370_e2254_d_n12,) = {
    if (locals.var_guard30 != 0.0) {
        let assign3370_e2250: f64 = (p.p162 / locals.var_lod_half_ref);
        let assign3370_e2252: f64 = (assign3370_e2250).powf(p.p164);
        (assign3370_e2252, if 0.0 == 0.0 && ((p.p164) as f64).is_finite() && ((p.p164) as f64).fract() == 0.0 { if p.p164 == 0.0 { 0.0 } else { (p.p164 * ((assign3370_e2250).powf(p.p164 - 1.0) * (-((p.p162 * locals.var_lod_half_ref_dn0) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign3370_e2252 * (p.p164 * ((-((p.p162 * locals.var_lod_half_ref_dn0) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign3370_e2250))) }, if 0.0 == 0.0 && ((p.p164) as f64).is_finite() && ((p.p164) as f64).fract() == 0.0 { if p.p164 == 0.0 { 0.0 } else { (p.p164 * ((assign3370_e2250).powf(p.p164 - 1.0) * (-((p.p162 * locals.var_lod_half_ref_dn2) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign3370_e2252 * (p.p164 * ((-((p.p162 * locals.var_lod_half_ref_dn2) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign3370_e2250))) }, if 0.0 == 0.0 && ((p.p164) as f64).is_finite() && ((p.p164) as f64).fract() == 0.0 { if p.p164 == 0.0 { 0.0 } else { (p.p164 * ((assign3370_e2250).powf(p.p164 - 1.0) * (-((p.p162 * locals.var_lod_half_ref_dn4) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign3370_e2252 * (p.p164 * ((-((p.p162 * locals.var_lod_half_ref_dn4) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign3370_e2250))) }, if 0.0 == 0.0 && ((p.p164) as f64).is_finite() && ((p.p164) as f64).fract() == 0.0 { if p.p164 == 0.0 { 0.0 } else { (p.p164 * ((assign3370_e2250).powf(p.p164 - 1.0) * (-((p.p162 * locals.var_lod_half_ref_dn5) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign3370_e2252 * (p.p164 * ((-((p.p162 * locals.var_lod_half_ref_dn5) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign3370_e2250))) }, if 0.0 == 0.0 && ((p.p164) as f64).is_finite() && ((p.p164) as f64).fract() == 0.0 { if p.p164 == 0.0 { 0.0 } else { (p.p164 * ((assign3370_e2250).powf(p.p164 - 1.0) * (-((p.p162 * locals.var_lod_half_ref_dn6) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign3370_e2252 * (p.p164 * ((-((p.p162 * locals.var_lod_half_ref_dn6) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign3370_e2250))) }, if 0.0 == 0.0 && ((p.p164) as f64).is_finite() && ((p.p164) as f64).fract() == 0.0 { if p.p164 == 0.0 { 0.0 } else { (p.p164 * ((assign3370_e2250).powf(p.p164 - 1.0) * (-((p.p162 * locals.var_lod_half_ref_dn8) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign3370_e2252 * (p.p164 * ((-((p.p162 * locals.var_lod_half_ref_dn8) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign3370_e2250))) }, if 0.0 == 0.0 && ((p.p164) as f64).is_finite() && ((p.p164) as f64).fract() == 0.0 { if p.p164 == 0.0 { 0.0 } else { (p.p164 * ((assign3370_e2250).powf(p.p164 - 1.0) * (-((p.p162 * locals.var_lod_half_ref_dn10) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign3370_e2252 * (p.p164 * ((-((p.p162 * locals.var_lod_half_ref_dn10) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign3370_e2250))) }, if 0.0 == 0.0 && ((p.p164) as f64).is_finite() && ((p.p164) as f64).fract() == 0.0 { if p.p164 == 0.0 { 0.0 } else { (p.p164 * ((assign3370_e2250).powf(p.p164 - 1.0) * (-((p.p162 * locals.var_lod_half_ref_dn11) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign3370_e2252 * (p.p164 * ((-((p.p162 * locals.var_lod_half_ref_dn11) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign3370_e2250))) }, if 0.0 == 0.0 && ((p.p164) as f64).is_finite() && ((p.p164) as f64).fract() == 0.0 { if p.p164 == 0.0 { 0.0 } else { (p.p164 * ((assign3370_e2250).powf(p.p164 - 1.0) * (-((p.p162 * locals.var_lod_half_ref_dn12) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign3370_e2252 * (p.p164 * ((-((p.p162 * locals.var_lod_half_ref_dn12) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign3370_e2250))) },)
    } else {
        (locals.var_t3__blk29, locals.var_t3__blk29_dn0, locals.var_t3__blk29_dn2, locals.var_t3__blk29_dn4, locals.var_t3__blk29_dn5, locals.var_t3__blk29_dn6, locals.var_t3__blk29_dn8, locals.var_t3__blk29_dn10, locals.var_t3__blk29_dn11, locals.var_t3__blk29_dn12,)
    }
};
        locals.var_t3__blk29 = assign3370_e2254;
        locals.var_t3__blk29_dn0 = assign3370_e2254_d_n0;
        locals.var_t3__blk29_dn2 = assign3370_e2254_d_n2;
        locals.var_t3__blk29_dn4 = assign3370_e2254_d_n4;
        locals.var_t3__blk29_dn5 = assign3370_e2254_d_n5;
        locals.var_t3__blk29_dn6 = assign3370_e2254_d_n6;
        locals.var_t3__blk29_dn8 = assign3370_e2254_d_n8;
        locals.var_t3__blk29_dn10 = assign3370_e2254_d_n10;
        locals.var_t3__blk29_dn11 = assign3370_e2254_d_n11;
        locals.var_t3__blk29_dn12 = assign3370_e2254_d_n12;
        locals.var_t3__blk29_rv = 0.0;

        let (assign3380_e2270, assign3380_e2270_d_n0, assign3380_e2270_d_n2, assign3380_e2270_d_n4, assign3380_e2270_d_n5, assign3380_e2270_d_n6, assign3380_e2270_d_n8, assign3380_e2270_d_n10, assign3380_e2270_d_n11, assign3380_e2270_d_n12,) = {
    if (locals.var_guard30 != 0.0) {
        let assign3380_e2260: f64 = (locals.var_t1__blk27 * locals.var_t2__blk28);
        let assign3380_e2261: f64 = (1.0 + assign3380_e2260);
        let assign3380_e2262: f64 = (locals.var_cgs_mueph * assign3380_e2261);
        let assign3380_e2266: f64 = (locals.var_t1__blk27 * locals.var_t3__blk29);
        let assign3380_e2267: f64 = (1.0 + assign3380_e2266);
        let assign3380_e2268: f64 = (assign3380_e2262 / assign3380_e2267);
        (assign3380_e2268, (((((locals.var_cgs_mueph_dn0 * assign3380_e2261) + (locals.var_cgs_mueph * (locals.var_t1__blk27 * locals.var_t2__blk28_dn0))) * assign3380_e2267) - (assign3380_e2262 * (locals.var_t1__blk27 * locals.var_t3__blk29_dn0))) / (assign3380_e2267 * assign3380_e2267)), (((((locals.var_cgs_mueph_dn2 * assign3380_e2261) + (locals.var_cgs_mueph * (locals.var_t1__blk27 * locals.var_t2__blk28_dn2))) * assign3380_e2267) - (assign3380_e2262 * (locals.var_t1__blk27 * locals.var_t3__blk29_dn2))) / (assign3380_e2267 * assign3380_e2267)), (((((locals.var_cgs_mueph_dn4 * assign3380_e2261) + (locals.var_cgs_mueph * ((locals.var_t1__blk27_dn4 * locals.var_t2__blk28) + (locals.var_t1__blk27 * locals.var_t2__blk28_dn4)))) * assign3380_e2267) - (assign3380_e2262 * ((locals.var_t1__blk27_dn4 * locals.var_t3__blk29) + (locals.var_t1__blk27 * locals.var_t3__blk29_dn4)))) / (assign3380_e2267 * assign3380_e2267)), (((((locals.var_cgs_mueph_dn5 * assign3380_e2261) + (locals.var_cgs_mueph * (locals.var_t1__blk27 * locals.var_t2__blk28_dn5))) * assign3380_e2267) - (assign3380_e2262 * (locals.var_t1__blk27 * locals.var_t3__blk29_dn5))) / (assign3380_e2267 * assign3380_e2267)), (((((locals.var_cgs_mueph_dn6 * assign3380_e2261) + (locals.var_cgs_mueph * (locals.var_t1__blk27 * locals.var_t2__blk28_dn6))) * assign3380_e2267) - (assign3380_e2262 * (locals.var_t1__blk27 * locals.var_t3__blk29_dn6))) / (assign3380_e2267 * assign3380_e2267)), (((((locals.var_cgs_mueph_dn8 * assign3380_e2261) + (locals.var_cgs_mueph * (locals.var_t1__blk27 * locals.var_t2__blk28_dn8))) * assign3380_e2267) - (assign3380_e2262 * (locals.var_t1__blk27 * locals.var_t3__blk29_dn8))) / (assign3380_e2267 * assign3380_e2267)), (((((locals.var_cgs_mueph_dn10 * assign3380_e2261) + (locals.var_cgs_mueph * (locals.var_t1__blk27 * locals.var_t2__blk28_dn10))) * assign3380_e2267) - (assign3380_e2262 * (locals.var_t1__blk27 * locals.var_t3__blk29_dn10))) / (assign3380_e2267 * assign3380_e2267)), (((((locals.var_cgs_mueph_dn11 * assign3380_e2261) + (locals.var_cgs_mueph * (locals.var_t1__blk27 * locals.var_t2__blk28_dn11))) * assign3380_e2267) - (assign3380_e2262 * (locals.var_t1__blk27 * locals.var_t3__blk29_dn11))) / (assign3380_e2267 * assign3380_e2267)), (((((locals.var_cgs_mueph_dn12 * assign3380_e2261) + (locals.var_cgs_mueph * (locals.var_t1__blk27 * locals.var_t2__blk28_dn12))) * assign3380_e2267) - (assign3380_e2262 * (locals.var_t1__blk27 * locals.var_t3__blk29_dn12))) / (assign3380_e2267 * assign3380_e2267)),)
    } else {
        (locals.var_cgs_mueph, locals.var_cgs_mueph_dn0, locals.var_cgs_mueph_dn2, locals.var_cgs_mueph_dn4, locals.var_cgs_mueph_dn5, locals.var_cgs_mueph_dn6, locals.var_cgs_mueph_dn8, locals.var_cgs_mueph_dn10, locals.var_cgs_mueph_dn11, locals.var_cgs_mueph_dn12,)
    }
};
        locals.var_cgs_mueph = assign3380_e2270;
        locals.var_cgs_mueph_dn0 = assign3380_e2270_d_n0;
        locals.var_cgs_mueph_dn2 = assign3380_e2270_d_n2;
        locals.var_cgs_mueph_dn4 = assign3380_e2270_d_n4;
        locals.var_cgs_mueph_dn5 = assign3380_e2270_d_n5;
        locals.var_cgs_mueph_dn6 = assign3380_e2270_d_n6;
        locals.var_cgs_mueph_dn8 = assign3380_e2270_d_n8;
        locals.var_cgs_mueph_dn10 = assign3380_e2270_d_n10;
        locals.var_cgs_mueph_dn11 = assign3380_e2270_d_n11;
        locals.var_cgs_mueph_dn12 = assign3380_e2270_d_n12;
        locals.var_cgs_mueph_rv = 0.0;

        let (assign3390_e2286, assign3390_e2286_d_n0, assign3390_e2286_d_n2, assign3390_e2286_d_n4, assign3390_e2286_d_n5, assign3390_e2286_d_n6, assign3390_e2286_d_n8, assign3390_e2286_d_n10, assign3390_e2286_d_n11, assign3390_e2286_d_n12,) = {
    if (locals.var_guard30 != 0.0) {
        let assign3390_e2276: f64 = (locals.var_t1__blk27 * locals.var_t2__blk28);
        let assign3390_e2277: f64 = (1.0 + assign3390_e2276);
        let assign3390_e2278: f64 = (locals.var_cgs_muephb * assign3390_e2277);
        let assign3390_e2282: f64 = (locals.var_t1__blk27 * locals.var_t3__blk29);
        let assign3390_e2283: f64 = (1.0 + assign3390_e2282);
        let assign3390_e2284: f64 = (assign3390_e2278 / assign3390_e2283);
        (assign3390_e2284, (((((locals.var_cgs_muephb_dn0 * assign3390_e2277) + (locals.var_cgs_muephb * (locals.var_t1__blk27 * locals.var_t2__blk28_dn0))) * assign3390_e2283) - (assign3390_e2278 * (locals.var_t1__blk27 * locals.var_t3__blk29_dn0))) / (assign3390_e2283 * assign3390_e2283)), (((((locals.var_cgs_muephb_dn2 * assign3390_e2277) + (locals.var_cgs_muephb * (locals.var_t1__blk27 * locals.var_t2__blk28_dn2))) * assign3390_e2283) - (assign3390_e2278 * (locals.var_t1__blk27 * locals.var_t3__blk29_dn2))) / (assign3390_e2283 * assign3390_e2283)), (((((locals.var_cgs_muephb_dn4 * assign3390_e2277) + (locals.var_cgs_muephb * ((locals.var_t1__blk27_dn4 * locals.var_t2__blk28) + (locals.var_t1__blk27 * locals.var_t2__blk28_dn4)))) * assign3390_e2283) - (assign3390_e2278 * ((locals.var_t1__blk27_dn4 * locals.var_t3__blk29) + (locals.var_t1__blk27 * locals.var_t3__blk29_dn4)))) / (assign3390_e2283 * assign3390_e2283)), (((((locals.var_cgs_muephb_dn5 * assign3390_e2277) + (locals.var_cgs_muephb * (locals.var_t1__blk27 * locals.var_t2__blk28_dn5))) * assign3390_e2283) - (assign3390_e2278 * (locals.var_t1__blk27 * locals.var_t3__blk29_dn5))) / (assign3390_e2283 * assign3390_e2283)), (((((locals.var_cgs_muephb_dn6 * assign3390_e2277) + (locals.var_cgs_muephb * (locals.var_t1__blk27 * locals.var_t2__blk28_dn6))) * assign3390_e2283) - (assign3390_e2278 * (locals.var_t1__blk27 * locals.var_t3__blk29_dn6))) / (assign3390_e2283 * assign3390_e2283)), (((((locals.var_cgs_muephb_dn8 * assign3390_e2277) + (locals.var_cgs_muephb * (locals.var_t1__blk27 * locals.var_t2__blk28_dn8))) * assign3390_e2283) - (assign3390_e2278 * (locals.var_t1__blk27 * locals.var_t3__blk29_dn8))) / (assign3390_e2283 * assign3390_e2283)), (((((locals.var_cgs_muephb_dn10 * assign3390_e2277) + (locals.var_cgs_muephb * (locals.var_t1__blk27 * locals.var_t2__blk28_dn10))) * assign3390_e2283) - (assign3390_e2278 * (locals.var_t1__blk27 * locals.var_t3__blk29_dn10))) / (assign3390_e2283 * assign3390_e2283)), (((((locals.var_cgs_muephb_dn11 * assign3390_e2277) + (locals.var_cgs_muephb * (locals.var_t1__blk27 * locals.var_t2__blk28_dn11))) * assign3390_e2283) - (assign3390_e2278 * (locals.var_t1__blk27 * locals.var_t3__blk29_dn11))) / (assign3390_e2283 * assign3390_e2283)), (((((locals.var_cgs_muephb_dn12 * assign3390_e2277) + (locals.var_cgs_muephb * (locals.var_t1__blk27 * locals.var_t2__blk28_dn12))) * assign3390_e2283) - (assign3390_e2278 * (locals.var_t1__blk27 * locals.var_t3__blk29_dn12))) / (assign3390_e2283 * assign3390_e2283)),)
    } else {
        (locals.var_cgs_muephb, locals.var_cgs_muephb_dn0, locals.var_cgs_muephb_dn2, locals.var_cgs_muephb_dn4, locals.var_cgs_muephb_dn5, locals.var_cgs_muephb_dn6, locals.var_cgs_muephb_dn8, locals.var_cgs_muephb_dn10, locals.var_cgs_muephb_dn11, locals.var_cgs_muephb_dn12,)
    }
};
        locals.var_cgs_muephb = assign3390_e2286;
        locals.var_cgs_muephb_dn0 = assign3390_e2286_d_n0;
        locals.var_cgs_muephb_dn2 = assign3390_e2286_d_n2;
        locals.var_cgs_muephb_dn4 = assign3390_e2286_d_n4;
        locals.var_cgs_muephb_dn5 = assign3390_e2286_d_n5;
        locals.var_cgs_muephb_dn6 = assign3390_e2286_d_n6;
        locals.var_cgs_muephb_dn8 = assign3390_e2286_d_n8;
        locals.var_cgs_muephb_dn10 = assign3390_e2286_d_n10;
        locals.var_cgs_muephb_dn11 = assign3390_e2286_d_n11;
        locals.var_cgs_muephb_dn12 = assign3390_e2286_d_n12;
        locals.var_cgs_muephb_rv = 0.0;

        let assign3400_e2291: f64 = (locals.var_lg).powf(p.p113);
        let assign3400_e2292: f64 = (p.p112 / assign3400_e2291);
        let assign3400_e2293: f64 = (1.0 + assign3400_e2292);
        locals.var_t1__blk27 = assign3400_e2293;
        locals.var_t1__blk27_dn4 = 0.0;
        locals.var_t1__blk27_rv = 0.0;

        let assign3410_e2296: f64 = (p.p111 * locals.var_t1__blk27);
        let assign3410_e2300: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        let assign3410_e2302: f64 = (assign3410_e2300 - 1.0);
        let assign3410_e2303: f64 = (p.p253 * assign3410_e2302);
        let assign3410_e2306: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        let assign3410_e2308: f64 = (assign3410_e2306 - 1.0);
        let assign3410_e2309: f64 = (assign3410_e2303 * assign3410_e2308);
        let assign3410_e2310: f64 = (assign3410_e2296 + assign3410_e2309);
        locals.var_mtmp = assign3410_e2310;
        locals.var_mtmp_dn4 = ((p.p111 * locals.var_t1__blk27_dn4) + (((p.p253 * (locals.var_ttemp_dn4 / locals.var_uc_tnom)) * assign3410_e2308) + (assign3410_e2303 * (locals.var_ttemp_dn4 / locals.var_uc_tnom))));
        locals.var_mtmp_rv = 0.0;

        let assign3420_e2313: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        let assign3420_e2315: f64 = (assign3420_e2313).powf(locals.var_mtmp);
        locals.var_t1__blk27 = assign3420_e2315;
        locals.var_t1__blk27_dn4 = if locals.var_mtmp_dn4 == 0.0 && ((locals.var_mtmp) as f64).is_finite() && ((locals.var_mtmp) as f64).fract() == 0.0 { if locals.var_mtmp == 0.0 { 0.0 } else { (locals.var_mtmp * ((assign3420_e2313).powf(locals.var_mtmp - 1.0) * (locals.var_ttemp_dn4 / locals.var_uc_tnom))) } } else { (assign3420_e2315 * ((locals.var_mtmp_dn4 * (assign3420_e2313).ln()) + (locals.var_mtmp * ((locals.var_ttemp_dn4 / locals.var_uc_tnom) / assign3420_e2313)))) };
        locals.var_t1__blk27_rv = 0.0;

        let assign3430_e2318: f64 = (locals.var_t1__blk27 / locals.var_cgs_mueph);
        locals.var_cgs_mphn0 = assign3430_e2318;
        locals.var_cgs_mphn0_dn0 = (-((locals.var_t1__blk27 * locals.var_cgs_mueph_dn0) / (locals.var_cgs_mueph * locals.var_cgs_mueph)));
        locals.var_cgs_mphn0_dn2 = (-((locals.var_t1__blk27 * locals.var_cgs_mueph_dn2) / (locals.var_cgs_mueph * locals.var_cgs_mueph)));
        locals.var_cgs_mphn0_dn4 = (((locals.var_t1__blk27_dn4 * locals.var_cgs_mueph) - (locals.var_t1__blk27 * locals.var_cgs_mueph_dn4)) / (locals.var_cgs_mueph * locals.var_cgs_mueph));
        locals.var_cgs_mphn0_dn5 = (-((locals.var_t1__blk27 * locals.var_cgs_mueph_dn5) / (locals.var_cgs_mueph * locals.var_cgs_mueph)));
        locals.var_cgs_mphn0_dn6 = (-((locals.var_t1__blk27 * locals.var_cgs_mueph_dn6) / (locals.var_cgs_mueph * locals.var_cgs_mueph)));
        locals.var_cgs_mphn0_dn8 = (-((locals.var_t1__blk27 * locals.var_cgs_mueph_dn8) / (locals.var_cgs_mueph * locals.var_cgs_mueph)));
        locals.var_cgs_mphn0_dn10 = (-((locals.var_t1__blk27 * locals.var_cgs_mueph_dn10) / (locals.var_cgs_mueph * locals.var_cgs_mueph)));
        locals.var_cgs_mphn0_dn11 = (-((locals.var_t1__blk27 * locals.var_cgs_mueph_dn11) / (locals.var_cgs_mueph * locals.var_cgs_mueph)));
        locals.var_cgs_mphn0_dn12 = (-((locals.var_t1__blk27 * locals.var_cgs_mueph_dn12) / (locals.var_cgs_mueph * locals.var_cgs_mueph)));
        locals.var_cgs_mphn0_rv = 0.0;

        let assign3440_e2321: f64 = (locals.var_t1__blk27 / locals.var_cgs_muephb);
        locals.var_cgs_mphbn0 = assign3440_e2321;
        locals.var_cgs_mphbn0_dn0 = (-((locals.var_t1__blk27 * locals.var_cgs_muephb_dn0) / (locals.var_cgs_muephb * locals.var_cgs_muephb)));
        locals.var_cgs_mphbn0_dn2 = (-((locals.var_t1__blk27 * locals.var_cgs_muephb_dn2) / (locals.var_cgs_muephb * locals.var_cgs_muephb)));
        locals.var_cgs_mphbn0_dn4 = (((locals.var_t1__blk27_dn4 * locals.var_cgs_muephb) - (locals.var_t1__blk27 * locals.var_cgs_muephb_dn4)) / (locals.var_cgs_muephb * locals.var_cgs_muephb));
        locals.var_cgs_mphbn0_dn5 = (-((locals.var_t1__blk27 * locals.var_cgs_muephb_dn5) / (locals.var_cgs_muephb * locals.var_cgs_muephb)));
        locals.var_cgs_mphbn0_dn6 = (-((locals.var_t1__blk27 * locals.var_cgs_muephb_dn6) / (locals.var_cgs_muephb * locals.var_cgs_muephb)));
        locals.var_cgs_mphbn0_dn8 = (-((locals.var_t1__blk27 * locals.var_cgs_muephb_dn8) / (locals.var_cgs_muephb * locals.var_cgs_muephb)));
        locals.var_cgs_mphbn0_dn10 = (-((locals.var_t1__blk27 * locals.var_cgs_muephb_dn10) / (locals.var_cgs_muephb * locals.var_cgs_muephb)));
        locals.var_cgs_mphbn0_dn11 = (-((locals.var_t1__blk27 * locals.var_cgs_muephb_dn11) / (locals.var_cgs_muephb * locals.var_cgs_muephb)));
        locals.var_cgs_mphbn0_dn12 = (-((locals.var_t1__blk27 * locals.var_cgs_muephb_dn12) / (locals.var_cgs_muephb * locals.var_cgs_muephb)));
        locals.var_cgs_mphbn0_rv = 0.0;

        let assign3450_e2324: f64 = (locals.var_ptovr0 * locals.var_beta_inv);
        locals.var_ptovr = assign3450_e2324;
        locals.var_ptovr_dn0 = (locals.var_ptovr0_dn0 * locals.var_beta_inv);
        locals.var_ptovr_dn2 = (locals.var_ptovr0_dn2 * locals.var_beta_inv);
        locals.var_ptovr_dn4 = ((locals.var_ptovr0_dn4 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn4));
        locals.var_ptovr_dn5 = (locals.var_ptovr0_dn5 * locals.var_beta_inv);
        locals.var_ptovr_dn6 = (locals.var_ptovr0_dn6 * locals.var_beta_inv);
        locals.var_ptovr_dn8 = (locals.var_ptovr0_dn8 * locals.var_beta_inv);
        locals.var_ptovr_dn10 = (locals.var_ptovr0_dn10 * locals.var_beta_inv);
        locals.var_ptovr_dn11 = (locals.var_ptovr0_dn11 * locals.var_beta_inv);
        locals.var_ptovr_dn12 = (locals.var_ptovr0_dn12 * locals.var_beta_inv);
        locals.var_ptovr_rv = 0.0;

        let assign3460_e2329: f64 = (locals.var_lg).powf(p.p182);
        let assign3460_e2330: f64 = (p.p181 / assign3460_e2329);
        let assign3460_e2331: f64 = (1.0 + assign3460_e2330);
        let assign3460_e2336: f64 = (locals.var_lg).powf(p.p186);
        let assign3460_e2337: f64 = (p.p185 / assign3460_e2336);
        let assign3460_e2338: f64 = (1.0 + assign3460_e2337);
        let assign3460_e2339: f64 = (assign3460_e2331 * assign3460_e2338);
        let assign3460_e2344: f64 = (locals.var_wg).powf(p.p188);
        let assign3460_e2345: f64 = (p.p187 / assign3460_e2344);
        let assign3460_e2346: f64 = (1.0 + assign3460_e2345);
        let assign3460_e2347: f64 = (assign3460_e2339 * assign3460_e2346);
        let assign3460_e2352: f64 = (locals.var_wlg).powf(p.p184);
        let assign3460_e2353: f64 = (p.p183 / assign3460_e2352);
        let assign3460_e2354: f64 = (1.0 + assign3460_e2353);
        let assign3460_e2355: f64 = (assign3460_e2347 * assign3460_e2354);
        locals.var_t1 = assign3460_e2355;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign3470_e2358: f64 = (locals.var_t1 * locals.var_t1);
        let assign3470_e2361: f64 = (4.0 * 0.001);
        let assign3470_e2363: f64 = (assign3470_e2361 * 0.001);
        let assign3470_e2364: f64 = (assign3470_e2358 + assign3470_e2363);
        let assign3470_e2365: f64 = (assign3470_e2364).sqrt();
        locals.var_tmf2 = assign3470_e2365;
        locals.var_tmf2_dn0 = (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign3470_e2365));
        locals.var_tmf2_dn2 = (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign3470_e2365));
        locals.var_tmf2_dn4 = (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign3470_e2365));
        locals.var_tmf2_dn5 = (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign3470_e2365));
        locals.var_tmf2_dn6 = (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign3470_e2365));
        locals.var_tmf2_dn8 = (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign3470_e2365));
        locals.var_tmf2_dn10 = (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign3470_e2365));
        locals.var_tmf2_dn11 = (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign3470_e2365));
        locals.var_tmf2_dn12 = (((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)) / (2.0 * assign3470_e2365));
        locals.var_tmf2_rv = 0.0;

        let assign3480_e2370: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign3480_e2371: f64 = (1.0 + assign3480_e2370);
        let assign3480_e2372: f64 = (0.5 * assign3480_e2371);
        locals.var_t2 = assign3480_e2372;
        locals.var_t2_dn0 = (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t2_dn2 = (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t2_dn4 = (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t2_dn5 = (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t2_dn6 = (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t2_dn8 = (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t2_dn10 = (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t2_dn11 = (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t2_dn12 = (0.5 * (((locals.var_t1_dn12 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t2_rv = 0.0;

        let assign3490_e2376: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign3490_e2377: f64 = (0.5 * assign3490_e2376);
        let assign3490_e2380: f64 = (1e-10 * 0.001);
        let assign3490_e2381: f64 = (assign3490_e2377 + assign3490_e2380);
        locals.var_vmax0 = assign3490_e2381;
        locals.var_vmax0_dn0 = (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0));
        locals.var_vmax0_dn2 = (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2));
        locals.var_vmax0_dn4 = (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4));
        locals.var_vmax0_dn5 = (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5));
        locals.var_vmax0_dn6 = (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6));
        locals.var_vmax0_dn8 = (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8));
        locals.var_vmax0_dn10 = (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10));
        locals.var_vmax0_dn11 = (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11));
        locals.var_vmax0_dn12 = (0.5 * (locals.var_t1_dn12 + locals.var_tmf2_dn12));
        locals.var_vmax0_rv = 0.0;

        let assign3500_e2384: f64 = if locals.var_vmax0 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard31 = assign3500_e2384;
        locals.var_guard31_rv = 0.0;

        let (assign3510_e2388, assign3510_e2388_d_n0, assign3510_e2388_d_n2, assign3510_e2388_d_n4, assign3510_e2388_d_n5, assign3510_e2388_d_n6, assign3510_e2388_d_n8, assign3510_e2388_d_n10, assign3510_e2388_d_n11, assign3510_e2388_d_n12,) = {
    if (locals.var_guard31 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vmax0, locals.var_vmax0_dn0, locals.var_vmax0_dn2, locals.var_vmax0_dn4, locals.var_vmax0_dn5, locals.var_vmax0_dn6, locals.var_vmax0_dn8, locals.var_vmax0_dn10, locals.var_vmax0_dn11, locals.var_vmax0_dn12,)
    }
};
        locals.var_vmax0 = assign3510_e2388;
        locals.var_vmax0_dn0 = assign3510_e2388_d_n0;
        locals.var_vmax0_dn2 = assign3510_e2388_d_n2;
        locals.var_vmax0_dn4 = assign3510_e2388_d_n4;
        locals.var_vmax0_dn5 = assign3510_e2388_d_n5;
        locals.var_vmax0_dn6 = assign3510_e2388_d_n6;
        locals.var_vmax0_dn8 = assign3510_e2388_d_n8;
        locals.var_vmax0_dn10 = assign3510_e2388_d_n10;
        locals.var_vmax0_dn11 = assign3510_e2388_d_n11;
        locals.var_vmax0_dn12 = assign3510_e2388_d_n12;
        locals.var_vmax0_rv = 0.0;

        let (assign3520_e2392, assign3520_e2392_d_n0, assign3520_e2392_d_n2, assign3520_e2392_d_n4, assign3520_e2392_d_n5, assign3520_e2392_d_n6, assign3520_e2392_d_n8, assign3520_e2392_d_n10, assign3520_e2392_d_n11, assign3520_e2392_d_n12,) = {
    if (locals.var_guard31 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign3520_e2392;
        locals.var_t2_dn0 = assign3520_e2392_d_n0;
        locals.var_t2_dn2 = assign3520_e2392_d_n2;
        locals.var_t2_dn4 = assign3520_e2392_d_n4;
        locals.var_t2_dn5 = assign3520_e2392_d_n5;
        locals.var_t2_dn6 = assign3520_e2392_d_n6;
        locals.var_t2_dn8 = assign3520_e2392_d_n8;
        locals.var_t2_dn10 = assign3520_e2392_d_n10;
        locals.var_t2_dn11 = assign3520_e2392_d_n11;
        locals.var_t2_dn12 = assign3520_e2392_d_n12;
        locals.var_t2_rv = 0.0;

        let assign3530_e2395: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        locals.var_t1 = assign3530_e2395;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = (locals.var_ttemp_dn4 / locals.var_uc_tnom);
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_8(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign3540_e2400: f64 = (locals.var_lg).powf(p.p103);
        let assign3540_e2401: f64 = (p.p102 / assign3540_e2400);
        let assign3540_e2402: f64 = (1.0 + assign3540_e2401);
        locals.var_t2 = assign3540_e2402;
        locals.var_t2_dn0 = 0.0;
        locals.var_t2_dn2 = 0.0;
        locals.var_t2_dn4 = 0.0;
        locals.var_t2_dn5 = 0.0;
        locals.var_t2_dn6 = 0.0;
        locals.var_t2_dn8 = 0.0;
        locals.var_t2_dn10 = 0.0;
        locals.var_t2_dn11 = 0.0;
        locals.var_t2_dn12 = 0.0;
        locals.var_t2_rv = 0.0;

        let assign3550_e2406: f64 = (locals.var_vmax0 * locals.var_mks_vmax);
        let assign3550_e2409: f64 = (1.8 * 0.01);
        let assign3550_e2412: f64 = (0.4 * locals.var_t1);
        let assign3550_e2414: f64 = (assign3550_e2412 * 0.01);
        let assign3550_e2415: f64 = (assign3550_e2409 + assign3550_e2414);
        let assign3550_e2418: f64 = (0.1 * locals.var_t1);
        let assign3550_e2420: f64 = (assign3550_e2418 * locals.var_t1);
        let assign3550_e2422: f64 = (assign3550_e2420 * 0.01);
        let assign3550_e2423: f64 = (assign3550_e2415 + assign3550_e2422);
        let assign3550_e2426: f64 = (locals.var_mks_vtmp * locals.var_t2);
        let assign3550_e2429: f64 = (1.0 - locals.var_t1);
        let assign3550_e2430: f64 = (assign3550_e2426 * assign3550_e2429);
        let assign3550_e2431: f64 = (assign3550_e2423 - assign3550_e2430);
        let assign3550_e2432: f64 = (assign3550_e2406 / assign3550_e2431);
        let assign3550_e2433: f64 = (0.01 * assign3550_e2432);
        locals.var_vmaxe = assign3550_e2433;
        locals.var_vmaxe_dn0 = (0.01 * ((((locals.var_vmax0_dn0 * locals.var_mks_vmax) * assign3550_e2431) - (assign3550_e2406 * ((((0.4 * locals.var_t1_dn0) * 0.01) + ((((0.1 * locals.var_t1_dn0) * locals.var_t1) + (assign3550_e2418 * locals.var_t1_dn0)) * 0.01)) - (((locals.var_mks_vtmp * locals.var_t2_dn0) * assign3550_e2429) + (assign3550_e2426 * (-locals.var_t1_dn0)))))) / (assign3550_e2431 * assign3550_e2431)));
        locals.var_vmaxe_dn2 = (0.01 * ((((locals.var_vmax0_dn2 * locals.var_mks_vmax) * assign3550_e2431) - (assign3550_e2406 * ((((0.4 * locals.var_t1_dn2) * 0.01) + ((((0.1 * locals.var_t1_dn2) * locals.var_t1) + (assign3550_e2418 * locals.var_t1_dn2)) * 0.01)) - (((locals.var_mks_vtmp * locals.var_t2_dn2) * assign3550_e2429) + (assign3550_e2426 * (-locals.var_t1_dn2)))))) / (assign3550_e2431 * assign3550_e2431)));
        locals.var_vmaxe_dn4 = (0.01 * ((((locals.var_vmax0_dn4 * locals.var_mks_vmax) * assign3550_e2431) - (assign3550_e2406 * ((((0.4 * locals.var_t1_dn4) * 0.01) + ((((0.1 * locals.var_t1_dn4) * locals.var_t1) + (assign3550_e2418 * locals.var_t1_dn4)) * 0.01)) - (((locals.var_mks_vtmp * locals.var_t2_dn4) * assign3550_e2429) + (assign3550_e2426 * (-locals.var_t1_dn4)))))) / (assign3550_e2431 * assign3550_e2431)));
        locals.var_vmaxe_dn5 = (0.01 * ((((locals.var_vmax0_dn5 * locals.var_mks_vmax) * assign3550_e2431) - (assign3550_e2406 * ((((0.4 * locals.var_t1_dn5) * 0.01) + ((((0.1 * locals.var_t1_dn5) * locals.var_t1) + (assign3550_e2418 * locals.var_t1_dn5)) * 0.01)) - (((locals.var_mks_vtmp * locals.var_t2_dn5) * assign3550_e2429) + (assign3550_e2426 * (-locals.var_t1_dn5)))))) / (assign3550_e2431 * assign3550_e2431)));
        locals.var_vmaxe_dn6 = (0.01 * ((((locals.var_vmax0_dn6 * locals.var_mks_vmax) * assign3550_e2431) - (assign3550_e2406 * ((((0.4 * locals.var_t1_dn6) * 0.01) + ((((0.1 * locals.var_t1_dn6) * locals.var_t1) + (assign3550_e2418 * locals.var_t1_dn6)) * 0.01)) - (((locals.var_mks_vtmp * locals.var_t2_dn6) * assign3550_e2429) + (assign3550_e2426 * (-locals.var_t1_dn6)))))) / (assign3550_e2431 * assign3550_e2431)));
        locals.var_vmaxe_dn8 = (0.01 * ((((locals.var_vmax0_dn8 * locals.var_mks_vmax) * assign3550_e2431) - (assign3550_e2406 * ((((0.4 * locals.var_t1_dn8) * 0.01) + ((((0.1 * locals.var_t1_dn8) * locals.var_t1) + (assign3550_e2418 * locals.var_t1_dn8)) * 0.01)) - (((locals.var_mks_vtmp * locals.var_t2_dn8) * assign3550_e2429) + (assign3550_e2426 * (-locals.var_t1_dn8)))))) / (assign3550_e2431 * assign3550_e2431)));
        locals.var_vmaxe_dn10 = (0.01 * ((((locals.var_vmax0_dn10 * locals.var_mks_vmax) * assign3550_e2431) - (assign3550_e2406 * ((((0.4 * locals.var_t1_dn10) * 0.01) + ((((0.1 * locals.var_t1_dn10) * locals.var_t1) + (assign3550_e2418 * locals.var_t1_dn10)) * 0.01)) - (((locals.var_mks_vtmp * locals.var_t2_dn10) * assign3550_e2429) + (assign3550_e2426 * (-locals.var_t1_dn10)))))) / (assign3550_e2431 * assign3550_e2431)));
        locals.var_vmaxe_dn11 = (0.01 * ((((locals.var_vmax0_dn11 * locals.var_mks_vmax) * assign3550_e2431) - (assign3550_e2406 * ((((0.4 * locals.var_t1_dn11) * 0.01) + ((((0.1 * locals.var_t1_dn11) * locals.var_t1) + (assign3550_e2418 * locals.var_t1_dn11)) * 0.01)) - (((locals.var_mks_vtmp * locals.var_t2_dn11) * assign3550_e2429) + (assign3550_e2426 * (-locals.var_t1_dn11)))))) / (assign3550_e2431 * assign3550_e2431)));
        locals.var_vmaxe_dn12 = (0.01 * ((((locals.var_vmax0_dn12 * locals.var_mks_vmax) * assign3550_e2431) - (assign3550_e2406 * ((((0.4 * locals.var_t1_dn12) * 0.01) + ((((0.1 * locals.var_t1_dn12) * locals.var_t1) + (assign3550_e2418 * locals.var_t1_dn12)) * 0.01)) - (((locals.var_mks_vtmp * locals.var_t2_dn12) * assign3550_e2429) + (assign3550_e2426 * (-locals.var_t1_dn12)))))) / (assign3550_e2431 * assign3550_e2431)));
        locals.var_vmaxe_rv = 0.0;

        let assign3560_e2435: f64 = (locals.var_eg).sqrt();
        locals.var_egp12 = assign3560_e2435;
        locals.var_egp12_dn0 = (locals.var_eg_dn0 / (2.0 * assign3560_e2435));
        locals.var_egp12_dn2 = (locals.var_eg_dn2 / (2.0 * assign3560_e2435));
        locals.var_egp12_dn4 = (locals.var_eg_dn4 / (2.0 * assign3560_e2435));
        locals.var_egp12_dn5 = (locals.var_eg_dn5 / (2.0 * assign3560_e2435));
        locals.var_egp12_dn6 = (locals.var_eg_dn6 / (2.0 * assign3560_e2435));
        locals.var_egp12_dn8 = (locals.var_eg_dn8 / (2.0 * assign3560_e2435));
        locals.var_egp12_dn10 = (locals.var_eg_dn10 / (2.0 * assign3560_e2435));
        locals.var_egp12_dn11 = (locals.var_eg_dn11 / (2.0 * assign3560_e2435));
        locals.var_egp12_dn12 = (locals.var_eg_dn12 / (2.0 * assign3560_e2435));
        locals.var_egp12_rv = 0.0;

        let assign3570_e2438: f64 = (locals.var_eg * locals.var_egp12);
        locals.var_egp32 = assign3570_e2438;
        locals.var_egp32_dn0 = ((locals.var_eg_dn0 * locals.var_egp12) + (locals.var_eg * locals.var_egp12_dn0));
        locals.var_egp32_dn2 = ((locals.var_eg_dn2 * locals.var_egp12) + (locals.var_eg * locals.var_egp12_dn2));
        locals.var_egp32_dn4 = ((locals.var_eg_dn4 * locals.var_egp12) + (locals.var_eg * locals.var_egp12_dn4));
        locals.var_egp32_dn5 = ((locals.var_eg_dn5 * locals.var_egp12) + (locals.var_eg * locals.var_egp12_dn5));
        locals.var_egp32_dn6 = ((locals.var_eg_dn6 * locals.var_egp12) + (locals.var_eg * locals.var_egp12_dn6));
        locals.var_egp32_dn8 = ((locals.var_eg_dn8 * locals.var_egp12) + (locals.var_eg * locals.var_egp12_dn8));
        locals.var_egp32_dn10 = ((locals.var_eg_dn10 * locals.var_egp12) + (locals.var_eg * locals.var_egp12_dn10));
        locals.var_egp32_dn11 = ((locals.var_eg_dn11 * locals.var_egp12) + (locals.var_eg * locals.var_egp12_dn11));
        locals.var_egp32_dn12 = ((locals.var_eg_dn12 * locals.var_egp12) + (locals.var_eg * locals.var_egp12_dn12));
        locals.var_egp32_rv = 0.0;

        let assign3580_e2442: f64 = (locals.var_ttemp / locals.var_uc_tnom);
        let assign3580_e2444: f64 = (assign3580_e2442).powf(1.5);
        let assign3580_e2445: f64 = (1.04e16 * assign3580_e2444);
        let assign3580_e2447: f64 = (-locals.var_eg);
        let assign3580_e2449: f64 = (assign3580_e2447 / 2.0);
        let assign3580_e2451: f64 = (assign3580_e2449 * locals.var_beta);
        let assign3580_e2454: f64 = (locals.var_egtnom / 2.0);
        let assign3580_e2456: f64 = (assign3580_e2454 * locals.var_betatnom);
        let assign3580_e2457: f64 = (assign3580_e2451 + assign3580_e2456);
        let assign3580_e2458: f64 = (assign3580_e2457).exp();
        let assign3580_e2459: f64 = (assign3580_e2445 * assign3580_e2458);
        locals.var_nin = assign3580_e2459;
        locals.var_nin_dn0 = (assign3580_e2445 * (assign3580_e2458 * (((-locals.var_eg_dn0) / 2.0) * locals.var_beta)));
        locals.var_nin_dn2 = (assign3580_e2445 * (assign3580_e2458 * (((-locals.var_eg_dn2) / 2.0) * locals.var_beta)));
        locals.var_nin_dn4 = (((1.04e16 * if 0.0 == 0.0 && ((1.5) as f64).is_finite() && ((1.5) as f64).fract() == 0.0 { if 1.5 == 0.0 { 0.0 } else { (1.5 * ((assign3580_e2442).powf(1.5 - 1.0) * (locals.var_ttemp_dn4 / locals.var_uc_tnom))) } } else { (assign3580_e2444 * (1.5 * ((locals.var_ttemp_dn4 / locals.var_uc_tnom) / assign3580_e2442))) }) * assign3580_e2458) + (assign3580_e2445 * (assign3580_e2458 * ((((-locals.var_eg_dn4) / 2.0) * locals.var_beta) + (assign3580_e2449 * locals.var_beta_dn4)))));
        locals.var_nin_dn5 = (assign3580_e2445 * (assign3580_e2458 * (((-locals.var_eg_dn5) / 2.0) * locals.var_beta)));
        locals.var_nin_dn6 = (assign3580_e2445 * (assign3580_e2458 * (((-locals.var_eg_dn6) / 2.0) * locals.var_beta)));
        locals.var_nin_dn8 = (assign3580_e2445 * (assign3580_e2458 * (((-locals.var_eg_dn8) / 2.0) * locals.var_beta)));
        locals.var_nin_dn10 = (assign3580_e2445 * (assign3580_e2458 * (((-locals.var_eg_dn10) / 2.0) * locals.var_beta)));
        locals.var_nin_dn11 = (assign3580_e2445 * (assign3580_e2458 * (((-locals.var_eg_dn11) / 2.0) * locals.var_beta)));
        locals.var_nin_dn12 = (assign3580_e2445 * (assign3580_e2458 * (((-locals.var_eg_dn12) / 2.0) * locals.var_beta)));
        locals.var_nin_rv = 0.0;

        let assign3590_e2462: f64 = (2.0 * 1.6021918e-19);
        let assign3590_e2464: f64 = (assign3590_e2462 * locals.var_uc_nsti);
        let assign3590_e2466: f64 = (assign3590_e2464 * 1.034943e-10);
        let assign3590_e2467: f64 = (assign3590_e2466).sqrt();
        locals.var_costi00 = assign3590_e2467;
        locals.var_costi00_dn0 = (((assign3590_e2462 * locals.var_uc_nsti_dn0) * 1.034943e-10) / (2.0 * assign3590_e2467));
        locals.var_costi00_dn2 = (((assign3590_e2462 * locals.var_uc_nsti_dn2) * 1.034943e-10) / (2.0 * assign3590_e2467));
        locals.var_costi00_dn4 = (((assign3590_e2462 * locals.var_uc_nsti_dn4) * 1.034943e-10) / (2.0 * assign3590_e2467));
        locals.var_costi00_dn5 = (((assign3590_e2462 * locals.var_uc_nsti_dn5) * 1.034943e-10) / (2.0 * assign3590_e2467));
        locals.var_costi00_dn6 = (((assign3590_e2462 * locals.var_uc_nsti_dn6) * 1.034943e-10) / (2.0 * assign3590_e2467));
        locals.var_costi00_dn8 = (((assign3590_e2462 * locals.var_uc_nsti_dn8) * 1.034943e-10) / (2.0 * assign3590_e2467));
        locals.var_costi00_dn10 = (((assign3590_e2462 * locals.var_uc_nsti_dn10) * 1.034943e-10) / (2.0 * assign3590_e2467));
        locals.var_costi00_dn11 = (((assign3590_e2462 * locals.var_uc_nsti_dn11) * 1.034943e-10) / (2.0 * assign3590_e2467));
        locals.var_costi00_dn12 = (((assign3590_e2462 * locals.var_uc_nsti_dn12) * 1.034943e-10) / (2.0 * assign3590_e2467));
        locals.var_costi00_rv = 0.0;

        let assign3600_e2471: f64 = (locals.var_uc_nsti * locals.var_uc_nsti);
        let assign3600_e2472: f64 = (1.0 / assign3600_e2471);
        locals.var_nsti_p2 = assign3600_e2472;
        locals.var_nsti_p2_dn0 = (-(((locals.var_uc_nsti_dn0 * locals.var_uc_nsti) + (locals.var_uc_nsti * locals.var_uc_nsti_dn0)) / (assign3600_e2471 * assign3600_e2471)));
        locals.var_nsti_p2_dn2 = (-(((locals.var_uc_nsti_dn2 * locals.var_uc_nsti) + (locals.var_uc_nsti * locals.var_uc_nsti_dn2)) / (assign3600_e2471 * assign3600_e2471)));
        locals.var_nsti_p2_dn4 = (-(((locals.var_uc_nsti_dn4 * locals.var_uc_nsti) + (locals.var_uc_nsti * locals.var_uc_nsti_dn4)) / (assign3600_e2471 * assign3600_e2471)));
        locals.var_nsti_p2_dn5 = (-(((locals.var_uc_nsti_dn5 * locals.var_uc_nsti) + (locals.var_uc_nsti * locals.var_uc_nsti_dn5)) / (assign3600_e2471 * assign3600_e2471)));
        locals.var_nsti_p2_dn6 = (-(((locals.var_uc_nsti_dn6 * locals.var_uc_nsti) + (locals.var_uc_nsti * locals.var_uc_nsti_dn6)) / (assign3600_e2471 * assign3600_e2471)));
        locals.var_nsti_p2_dn8 = (-(((locals.var_uc_nsti_dn8 * locals.var_uc_nsti) + (locals.var_uc_nsti * locals.var_uc_nsti_dn8)) / (assign3600_e2471 * assign3600_e2471)));
        locals.var_nsti_p2_dn10 = (-(((locals.var_uc_nsti_dn10 * locals.var_uc_nsti) + (locals.var_uc_nsti * locals.var_uc_nsti_dn10)) / (assign3600_e2471 * assign3600_e2471)));
        locals.var_nsti_p2_dn11 = (-(((locals.var_uc_nsti_dn11 * locals.var_uc_nsti) + (locals.var_uc_nsti * locals.var_uc_nsti_dn11)) / (assign3600_e2471 * assign3600_e2471)));
        locals.var_nsti_p2_dn12 = (-(((locals.var_uc_nsti_dn12 * locals.var_uc_nsti) + (locals.var_uc_nsti * locals.var_uc_nsti_dn12)) / (assign3600_e2471 * assign3600_e2471)));
        locals.var_nsti_p2_rv = 0.0;

        let assign3610_e2475: f64 = (locals.var_beta_inv).sqrt();
        let assign3610_e2476: f64 = (locals.var_costi00 * assign3610_e2475);
        locals.var_costi0 = assign3610_e2476;
        locals.var_costi0_dn0 = (locals.var_costi00_dn0 * assign3610_e2475);
        locals.var_costi0_dn2 = (locals.var_costi00_dn2 * assign3610_e2475);
        locals.var_costi0_dn4 = ((locals.var_costi00_dn4 * assign3610_e2475) + (locals.var_costi00 * (locals.var_beta_inv_dn4 / (2.0 * assign3610_e2475))));
        locals.var_costi0_dn5 = (locals.var_costi00_dn5 * assign3610_e2475);
        locals.var_costi0_dn6 = (locals.var_costi00_dn6 * assign3610_e2475);
        locals.var_costi0_dn8 = (locals.var_costi00_dn8 * assign3610_e2475);
        locals.var_costi0_dn10 = (locals.var_costi00_dn10 * assign3610_e2475);
        locals.var_costi0_dn11 = (locals.var_costi00_dn11 * assign3610_e2475);
        locals.var_costi0_dn12 = (locals.var_costi00_dn12 * assign3610_e2475);
        locals.var_costi0_rv = 0.0;

        let assign3620_e2479: f64 = (locals.var_costi0 * locals.var_costi0);
        locals.var_costi0_p2 = assign3620_e2479;
        locals.var_costi0_p2_dn0 = ((locals.var_costi0_dn0 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn0));
        locals.var_costi0_p2_dn2 = ((locals.var_costi0_dn2 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn2));
        locals.var_costi0_p2_dn4 = ((locals.var_costi0_dn4 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn4));
        locals.var_costi0_p2_dn5 = ((locals.var_costi0_dn5 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn5));
        locals.var_costi0_p2_dn6 = ((locals.var_costi0_dn6 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn6));
        locals.var_costi0_p2_dn8 = ((locals.var_costi0_dn8 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn8));
        locals.var_costi0_p2_dn10 = ((locals.var_costi0_dn10 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn10));
        locals.var_costi0_p2_dn11 = ((locals.var_costi0_dn11 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn11));
        locals.var_costi0_p2_dn12 = ((locals.var_costi0_dn12 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn12));
        locals.var_costi0_p2_rv = 0.0;

        let assign3630_e2482: f64 = (locals.var_nin * locals.var_nin);
        let assign3630_e2484: f64 = (assign3630_e2482 * locals.var_nsti_p2);
        locals.var_costi1 = assign3630_e2484;
        locals.var_costi1_dn0 = ((((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_nsti_p2) + (assign3630_e2482 * locals.var_nsti_p2_dn0));
        locals.var_costi1_dn2 = ((((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_nsti_p2) + (assign3630_e2482 * locals.var_nsti_p2_dn2));
        locals.var_costi1_dn4 = ((((locals.var_nin_dn4 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn4)) * locals.var_nsti_p2) + (assign3630_e2482 * locals.var_nsti_p2_dn4));
        locals.var_costi1_dn5 = ((((locals.var_nin_dn5 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn5)) * locals.var_nsti_p2) + (assign3630_e2482 * locals.var_nsti_p2_dn5));
        locals.var_costi1_dn6 = ((((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_nsti_p2) + (assign3630_e2482 * locals.var_nsti_p2_dn6));
        locals.var_costi1_dn8 = ((((locals.var_nin_dn8 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn8)) * locals.var_nsti_p2) + (assign3630_e2482 * locals.var_nsti_p2_dn8));
        locals.var_costi1_dn10 = ((((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_nsti_p2) + (assign3630_e2482 * locals.var_nsti_p2_dn10));
        locals.var_costi1_dn11 = ((((locals.var_nin_dn11 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn11)) * locals.var_nsti_p2) + (assign3630_e2482 * locals.var_nsti_p2_dn11));
        locals.var_costi1_dn12 = ((((locals.var_nin_dn12 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn12)) * locals.var_nsti_p2) + (assign3630_e2482 * locals.var_nsti_p2_dn12));
        locals.var_costi1_rv = 0.0;

        let assign3640_e2488: f64 = (p.p251 + p.p252);
        let assign3640_e2489: f64 = (p.p38 / assign3640_e2488);
        let assign3640_e2491: f64 = (assign3640_e2489 * p.p0);
        locals.var_t1 = assign3640_e2491;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign3650_e2494: f64 = (p.p38 * 0.001);
        let assign3650_e2497: f64 = (10.0 * 2.220446049250313e-16);
        let assign3650_e2499: f64 = (assign3650_e2497 / 100.0);
        let assign3650_e2500: f64 = (assign3650_e2494 + assign3650_e2499);
        let assign3650_e2501: f64 = (assign3650_e2500).abs();
        locals.var_t3 = assign3650_e2501;
        locals.var_t3_dn0 = 0.0;
        locals.var_t3_dn2 = 0.0;
        locals.var_t3_dn4 = 0.0;
        locals.var_t3_dn5 = 0.0;
        locals.var_t3_dn6 = 0.0;
        locals.var_t3_dn8 = 0.0;
        locals.var_t3_dn10 = 0.0;
        locals.var_t3_dn11 = 0.0;
        locals.var_t3_dn12 = 0.0;
        locals.var_t3_rv = 0.0;

        let assign3660_e2504: f64 = if p.p38 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard32 = assign3660_e2504;
        locals.var_guard32_rv = 0.0;

        let (assign3670_e2512, assign3670_e2512_d_n0, assign3670_e2512_d_n2, assign3670_e2512_d_n4, assign3670_e2512_d_n5, assign3670_e2512_d_n6, assign3670_e2512_d_n8, assign3670_e2512_d_n10, assign3670_e2512_d_n11, assign3670_e2512_d_n12,) = {
    if (locals.var_guard32 != 0.0) {
        let assign3670_e2508: f64 = (p.p38 - locals.var_t1);
        let assign3670_e2510: f64 = (assign3670_e2508 - locals.var_t3);
        (assign3670_e2510, ((-locals.var_t1_dn0) - locals.var_t3_dn0), ((-locals.var_t1_dn2) - locals.var_t3_dn2), ((-locals.var_t1_dn4) - locals.var_t3_dn4), ((-locals.var_t1_dn5) - locals.var_t3_dn5), ((-locals.var_t1_dn6) - locals.var_t3_dn6), ((-locals.var_t1_dn8) - locals.var_t3_dn8), ((-locals.var_t1_dn10) - locals.var_t3_dn10), ((-locals.var_t1_dn11) - locals.var_t3_dn11), ((-locals.var_t1_dn12) - locals.var_t3_dn12),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn8, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12,)
    }
};
        locals.var_tmf1 = assign3670_e2512;
        locals.var_tmf1_dn0 = assign3670_e2512_d_n0;
        locals.var_tmf1_dn2 = assign3670_e2512_d_n2;
        locals.var_tmf1_dn4 = assign3670_e2512_d_n4;
        locals.var_tmf1_dn5 = assign3670_e2512_d_n5;
        locals.var_tmf1_dn6 = assign3670_e2512_d_n6;
        locals.var_tmf1_dn8 = assign3670_e2512_d_n8;
        locals.var_tmf1_dn10 = assign3670_e2512_d_n10;
        locals.var_tmf1_dn11 = assign3670_e2512_d_n11;
        locals.var_tmf1_dn12 = assign3670_e2512_d_n12;
        locals.var_tmf1_rv = 0.0;

        let (assign3680_e2520, assign3680_e2520_d_n0, assign3680_e2520_d_n2, assign3680_e2520_d_n4, assign3680_e2520_d_n5, assign3680_e2520_d_n6, assign3680_e2520_d_n8, assign3680_e2520_d_n10, assign3680_e2520_d_n11, assign3680_e2520_d_n12,) = {
    if (locals.var_guard32 != 0.0) {
        let assign3680_e2516: f64 = (4.0 * p.p38);
        let assign3680_e2518: f64 = (assign3680_e2516 * locals.var_t3);
        (assign3680_e2518, (assign3680_e2516 * locals.var_t3_dn0), (assign3680_e2516 * locals.var_t3_dn2), (assign3680_e2516 * locals.var_t3_dn4), (assign3680_e2516 * locals.var_t3_dn5), (assign3680_e2516 * locals.var_t3_dn6), (assign3680_e2516 * locals.var_t3_dn8), (assign3680_e2516 * locals.var_t3_dn10), (assign3680_e2516 * locals.var_t3_dn11), (assign3680_e2516 * locals.var_t3_dn12),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign3680_e2520;
        locals.var_tmf2_dn0 = assign3680_e2520_d_n0;
        locals.var_tmf2_dn2 = assign3680_e2520_d_n2;
        locals.var_tmf2_dn4 = assign3680_e2520_d_n4;
        locals.var_tmf2_dn5 = assign3680_e2520_d_n5;
        locals.var_tmf2_dn6 = assign3680_e2520_d_n6;
        locals.var_tmf2_dn8 = assign3680_e2520_d_n8;
        locals.var_tmf2_dn10 = assign3680_e2520_d_n10;
        locals.var_tmf2_dn11 = assign3680_e2520_d_n11;
        locals.var_tmf2_dn12 = assign3680_e2520_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign3690_e2530, assign3690_e2530_d_n0, assign3690_e2530_d_n2, assign3690_e2530_d_n4, assign3690_e2530_d_n5, assign3690_e2530_d_n6, assign3690_e2530_d_n8, assign3690_e2530_d_n10, assign3690_e2530_d_n11, assign3690_e2530_d_n12,) = {
    if (locals.var_guard32 != 0.0) {
        let (assign3690_e2528, assign3690_e2528_d_n0, assign3690_e2528_d_n2, assign3690_e2528_d_n4, assign3690_e2528_d_n5, assign3690_e2528_d_n6, assign3690_e2528_d_n8, assign3690_e2528_d_n10, assign3690_e2528_d_n11, assign3690_e2528_d_n12,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
            } else {
                let assign3690_e2527: f64 = (-locals.var_tmf2);
                (assign3690_e2527, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12),)
            }
        };
        (assign3690_e2528, assign3690_e2528_d_n0, assign3690_e2528_d_n2, assign3690_e2528_d_n4, assign3690_e2528_d_n5, assign3690_e2528_d_n6, assign3690_e2528_d_n8, assign3690_e2528_d_n10, assign3690_e2528_d_n11, assign3690_e2528_d_n12,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign3690_e2530;
        locals.var_tmf2_dn0 = assign3690_e2530_d_n0;
        locals.var_tmf2_dn2 = assign3690_e2530_d_n2;
        locals.var_tmf2_dn4 = assign3690_e2530_d_n4;
        locals.var_tmf2_dn5 = assign3690_e2530_d_n5;
        locals.var_tmf2_dn6 = assign3690_e2530_d_n6;
        locals.var_tmf2_dn8 = assign3690_e2530_d_n8;
        locals.var_tmf2_dn10 = assign3690_e2530_d_n10;
        locals.var_tmf2_dn11 = assign3690_e2530_d_n11;
        locals.var_tmf2_dn12 = assign3690_e2530_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign3700_e2539, assign3700_e2539_d_n0, assign3700_e2539_d_n2, assign3700_e2539_d_n4, assign3700_e2539_d_n5, assign3700_e2539_d_n6, assign3700_e2539_d_n8, assign3700_e2539_d_n10, assign3700_e2539_d_n11, assign3700_e2539_d_n12,) = {
    if (locals.var_guard32 != 0.0) {
        let assign3700_e2534: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign3700_e2536: f64 = (assign3700_e2534 + locals.var_tmf2);
        let assign3700_e2537: f64 = (assign3700_e2536).sqrt();
        (assign3700_e2537, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign3700_e2537)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign3700_e2537)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign3700_e2537)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign3700_e2537)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign3700_e2537)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign3700_e2537)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign3700_e2537)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign3700_e2537)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign3700_e2537)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign3700_e2539;
        locals.var_tmf2_dn0 = assign3700_e2539_d_n0;
        locals.var_tmf2_dn2 = assign3700_e2539_d_n2;
        locals.var_tmf2_dn4 = assign3700_e2539_d_n4;
        locals.var_tmf2_dn5 = assign3700_e2539_d_n5;
        locals.var_tmf2_dn6 = assign3700_e2539_d_n6;
        locals.var_tmf2_dn8 = assign3700_e2539_d_n8;
        locals.var_tmf2_dn10 = assign3700_e2539_d_n10;
        locals.var_tmf2_dn11 = assign3700_e2539_d_n11;
        locals.var_tmf2_dn12 = assign3700_e2539_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign3710_e2549, assign3710_e2549_d_n0, assign3710_e2549_d_n2, assign3710_e2549_d_n4, assign3710_e2549_d_n5, assign3710_e2549_d_n6, assign3710_e2549_d_n8, assign3710_e2549_d_n10, assign3710_e2549_d_n11, assign3710_e2549_d_n12,) = {
    if (locals.var_guard32 != 0.0) {
        let assign3710_e2545: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign3710_e2546: f64 = (1.0 + assign3710_e2545);
        let assign3710_e2547: f64 = (0.5 * assign3710_e2546);
        (assign3710_e2547, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign3710_e2549;
        locals.var_t1_dn0 = assign3710_e2549_d_n0;
        locals.var_t1_dn2 = assign3710_e2549_d_n2;
        locals.var_t1_dn4 = assign3710_e2549_d_n4;
        locals.var_t1_dn5 = assign3710_e2549_d_n5;
        locals.var_t1_dn6 = assign3710_e2549_d_n6;
        locals.var_t1_dn8 = assign3710_e2549_d_n8;
        locals.var_t1_dn10 = assign3710_e2549_d_n10;
        locals.var_t1_dn11 = assign3710_e2549_d_n11;
        locals.var_t1_dn12 = assign3710_e2549_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign3720_e2559, assign3720_e2559_d_n0, assign3720_e2559_d_n2, assign3720_e2559_d_n4, assign3720_e2559_d_n5, assign3720_e2559_d_n6, assign3720_e2559_d_n8, assign3720_e2559_d_n10, assign3720_e2559_d_n11, assign3720_e2559_d_n12,) = {
    if (locals.var_guard32 != 0.0) {
        let assign3720_e2555: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign3720_e2556: f64 = (0.5 * assign3720_e2555);
        let assign3720_e2557: f64 = (p.p38 - assign3720_e2556);
        (assign3720_e2557, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign3720_e2559;
        locals.var_t2_dn0 = assign3720_e2559_d_n0;
        locals.var_t2_dn2 = assign3720_e2559_d_n2;
        locals.var_t2_dn4 = assign3720_e2559_d_n4;
        locals.var_t2_dn5 = assign3720_e2559_d_n5;
        locals.var_t2_dn6 = assign3720_e2559_d_n6;
        locals.var_t2_dn8 = assign3720_e2559_d_n8;
        locals.var_t2_dn10 = assign3720_e2559_d_n10;
        locals.var_t2_dn11 = assign3720_e2559_d_n11;
        locals.var_t2_dn12 = assign3720_e2559_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign3730_e2568, assign3730_e2568_d_n0, assign3730_e2568_d_n2, assign3730_e2568_d_n4, assign3730_e2568_d_n5, assign3730_e2568_d_n6, assign3730_e2568_d_n8, assign3730_e2568_d_n10, assign3730_e2568_d_n11, assign3730_e2568_d_n12,) = {
    if (locals.var_guard32 == 0.0) {
        let assign3730_e2564: f64 = (locals.var_t1 - p.p38);
        let assign3730_e2566: f64 = (assign3730_e2564 - locals.var_t3);
        (assign3730_e2566, (locals.var_t1_dn0 - locals.var_t3_dn0), (locals.var_t1_dn2 - locals.var_t3_dn2), (locals.var_t1_dn4 - locals.var_t3_dn4), (locals.var_t1_dn5 - locals.var_t3_dn5), (locals.var_t1_dn6 - locals.var_t3_dn6), (locals.var_t1_dn8 - locals.var_t3_dn8), (locals.var_t1_dn10 - locals.var_t3_dn10), (locals.var_t1_dn11 - locals.var_t3_dn11), (locals.var_t1_dn12 - locals.var_t3_dn12),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn8, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12,)
    }
};
        locals.var_tmf1 = assign3730_e2568;
        locals.var_tmf1_dn0 = assign3730_e2568_d_n0;
        locals.var_tmf1_dn2 = assign3730_e2568_d_n2;
        locals.var_tmf1_dn4 = assign3730_e2568_d_n4;
        locals.var_tmf1_dn5 = assign3730_e2568_d_n5;
        locals.var_tmf1_dn6 = assign3730_e2568_d_n6;
        locals.var_tmf1_dn8 = assign3730_e2568_d_n8;
        locals.var_tmf1_dn10 = assign3730_e2568_d_n10;
        locals.var_tmf1_dn11 = assign3730_e2568_d_n11;
        locals.var_tmf1_dn12 = assign3730_e2568_d_n12;
        locals.var_tmf1_rv = 0.0;

        let (assign3740_e2577, assign3740_e2577_d_n0, assign3740_e2577_d_n2, assign3740_e2577_d_n4, assign3740_e2577_d_n5, assign3740_e2577_d_n6, assign3740_e2577_d_n8, assign3740_e2577_d_n10, assign3740_e2577_d_n11, assign3740_e2577_d_n12,) = {
    if (locals.var_guard32 == 0.0) {
        let assign3740_e2573: f64 = (4.0 * p.p38);
        let assign3740_e2575: f64 = (assign3740_e2573 * locals.var_t3);
        (assign3740_e2575, (assign3740_e2573 * locals.var_t3_dn0), (assign3740_e2573 * locals.var_t3_dn2), (assign3740_e2573 * locals.var_t3_dn4), (assign3740_e2573 * locals.var_t3_dn5), (assign3740_e2573 * locals.var_t3_dn6), (assign3740_e2573 * locals.var_t3_dn8), (assign3740_e2573 * locals.var_t3_dn10), (assign3740_e2573 * locals.var_t3_dn11), (assign3740_e2573 * locals.var_t3_dn12),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign3740_e2577;
        locals.var_tmf2_dn0 = assign3740_e2577_d_n0;
        locals.var_tmf2_dn2 = assign3740_e2577_d_n2;
        locals.var_tmf2_dn4 = assign3740_e2577_d_n4;
        locals.var_tmf2_dn5 = assign3740_e2577_d_n5;
        locals.var_tmf2_dn6 = assign3740_e2577_d_n6;
        locals.var_tmf2_dn8 = assign3740_e2577_d_n8;
        locals.var_tmf2_dn10 = assign3740_e2577_d_n10;
        locals.var_tmf2_dn11 = assign3740_e2577_d_n11;
        locals.var_tmf2_dn12 = assign3740_e2577_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign3750_e2588, assign3750_e2588_d_n0, assign3750_e2588_d_n2, assign3750_e2588_d_n4, assign3750_e2588_d_n5, assign3750_e2588_d_n6, assign3750_e2588_d_n8, assign3750_e2588_d_n10, assign3750_e2588_d_n11, assign3750_e2588_d_n12,) = {
    if (locals.var_guard32 == 0.0) {
        let (assign3750_e2586, assign3750_e2586_d_n0, assign3750_e2586_d_n2, assign3750_e2586_d_n4, assign3750_e2586_d_n5, assign3750_e2586_d_n6, assign3750_e2586_d_n8, assign3750_e2586_d_n10, assign3750_e2586_d_n11, assign3750_e2586_d_n12,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
            } else {
                let assign3750_e2585: f64 = (-locals.var_tmf2);
                (assign3750_e2585, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12),)
            }
        };
        (assign3750_e2586, assign3750_e2586_d_n0, assign3750_e2586_d_n2, assign3750_e2586_d_n4, assign3750_e2586_d_n5, assign3750_e2586_d_n6, assign3750_e2586_d_n8, assign3750_e2586_d_n10, assign3750_e2586_d_n11, assign3750_e2586_d_n12,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign3750_e2588;
        locals.var_tmf2_dn0 = assign3750_e2588_d_n0;
        locals.var_tmf2_dn2 = assign3750_e2588_d_n2;
        locals.var_tmf2_dn4 = assign3750_e2588_d_n4;
        locals.var_tmf2_dn5 = assign3750_e2588_d_n5;
        locals.var_tmf2_dn6 = assign3750_e2588_d_n6;
        locals.var_tmf2_dn8 = assign3750_e2588_d_n8;
        locals.var_tmf2_dn10 = assign3750_e2588_d_n10;
        locals.var_tmf2_dn11 = assign3750_e2588_d_n11;
        locals.var_tmf2_dn12 = assign3750_e2588_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign3760_e2598, assign3760_e2598_d_n0, assign3760_e2598_d_n2, assign3760_e2598_d_n4, assign3760_e2598_d_n5, assign3760_e2598_d_n6, assign3760_e2598_d_n8, assign3760_e2598_d_n10, assign3760_e2598_d_n11, assign3760_e2598_d_n12,) = {
    if (locals.var_guard32 == 0.0) {
        let assign3760_e2593: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign3760_e2595: f64 = (assign3760_e2593 + locals.var_tmf2);
        let assign3760_e2596: f64 = (assign3760_e2595).sqrt();
        (assign3760_e2596, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign3760_e2596)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign3760_e2596)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign3760_e2596)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign3760_e2596)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign3760_e2596)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign3760_e2596)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign3760_e2596)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign3760_e2596)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign3760_e2596)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign3760_e2598;
        locals.var_tmf2_dn0 = assign3760_e2598_d_n0;
        locals.var_tmf2_dn2 = assign3760_e2598_d_n2;
        locals.var_tmf2_dn4 = assign3760_e2598_d_n4;
        locals.var_tmf2_dn5 = assign3760_e2598_d_n5;
        locals.var_tmf2_dn6 = assign3760_e2598_d_n6;
        locals.var_tmf2_dn8 = assign3760_e2598_d_n8;
        locals.var_tmf2_dn10 = assign3760_e2598_d_n10;
        locals.var_tmf2_dn11 = assign3760_e2598_d_n11;
        locals.var_tmf2_dn12 = assign3760_e2598_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign3770_e2609, assign3770_e2609_d_n0, assign3770_e2609_d_n2, assign3770_e2609_d_n4, assign3770_e2609_d_n5, assign3770_e2609_d_n6, assign3770_e2609_d_n8, assign3770_e2609_d_n10, assign3770_e2609_d_n11, assign3770_e2609_d_n12,) = {
    if (locals.var_guard32 == 0.0) {
        let assign3770_e2605: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign3770_e2606: f64 = (1.0 + assign3770_e2605);
        let assign3770_e2607: f64 = (0.5 * assign3770_e2606);
        (assign3770_e2607, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign3770_e2609;
        locals.var_t1_dn0 = assign3770_e2609_d_n0;
        locals.var_t1_dn2 = assign3770_e2609_d_n2;
        locals.var_t1_dn4 = assign3770_e2609_d_n4;
        locals.var_t1_dn5 = assign3770_e2609_d_n5;
        locals.var_t1_dn6 = assign3770_e2609_d_n6;
        locals.var_t1_dn8 = assign3770_e2609_d_n8;
        locals.var_t1_dn10 = assign3770_e2609_d_n10;
        locals.var_t1_dn11 = assign3770_e2609_d_n11;
        locals.var_t1_dn12 = assign3770_e2609_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign3780_e2620, assign3780_e2620_d_n0, assign3780_e2620_d_n2, assign3780_e2620_d_n4, assign3780_e2620_d_n5, assign3780_e2620_d_n6, assign3780_e2620_d_n8, assign3780_e2620_d_n10, assign3780_e2620_d_n11, assign3780_e2620_d_n12,) = {
    if (locals.var_guard32 == 0.0) {
        let assign3780_e2616: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign3780_e2617: f64 = (0.5 * assign3780_e2616);
        let assign3780_e2618: f64 = (p.p38 + assign3780_e2617);
        (assign3780_e2618, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign3780_e2620;
        locals.var_t2_dn0 = assign3780_e2620_d_n0;
        locals.var_t2_dn2 = assign3780_e2620_d_n2;
        locals.var_t2_dn4 = assign3780_e2620_d_n4;
        locals.var_t2_dn5 = assign3780_e2620_d_n5;
        locals.var_t2_dn6 = assign3780_e2620_d_n6;
        locals.var_t2_dn8 = assign3780_e2620_d_n8;
        locals.var_t2_dn10 = assign3780_e2620_d_n10;
        locals.var_t2_dn11 = assign3780_e2620_d_n11;
        locals.var_t2_dn12 = assign3780_e2620_d_n12;
        locals.var_t2_rv = 0.0;

        let assign3790_e2624: f64 = (2.0 * locals.var_t2);
        let assign3790_e2625: f64 = (p.p0 - assign3790_e2624);
        locals.var_leff = assign3790_e2625;
        locals.var_leff_dn0 = (-(2.0 * locals.var_t2_dn0));
        locals.var_leff_dn2 = (-(2.0 * locals.var_t2_dn2));
        locals.var_leff_dn4 = (-(2.0 * locals.var_t2_dn4));
        locals.var_leff_dn5 = (-(2.0 * locals.var_t2_dn5));
        locals.var_leff_dn6 = (-(2.0 * locals.var_t2_dn6));
        locals.var_leff_dn8 = (-(2.0 * locals.var_t2_dn8));
        locals.var_leff_dn10 = (-(2.0 * locals.var_t2_dn10));
        locals.var_leff_dn11 = (-(2.0 * locals.var_t2_dn11));
        locals.var_leff_dn12 = (-(2.0 * locals.var_t2_dn12));
        locals.var_leff_rv = 0.0;

        let assign3800_e2627: f64 = (-p.p49);
        let assign3800_e2632: f64 = (locals.var_lg).powf(p.p51);
        let assign3800_e2633: f64 = (p.p50 / assign3800_e2632);
        let assign3800_e2634: f64 = (1.0 + assign3800_e2633);
        let assign3800_e2635: f64 = (assign3800_e2627 * assign3800_e2634);
        locals.var_t1 = assign3800_e2635;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_9(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign3810_e2637: f64 = (-p.p49);
        let assign3810_e2642: f64 = (locals.var_lg).powf(p.p53);
        let assign3810_e2643: f64 = (p.p52 / assign3810_e2642);
        let assign3810_e2644: f64 = (1.0 + assign3810_e2643);
        let assign3810_e2645: f64 = (assign3810_e2637 * assign3810_e2644);
        locals.var_t2 = assign3810_e2645;
        locals.var_t2_dn0 = 0.0;
        locals.var_t2_dn2 = 0.0;
        locals.var_t2_dn4 = 0.0;
        locals.var_t2_dn5 = 0.0;
        locals.var_t2_dn6 = 0.0;
        locals.var_t2_dn8 = 0.0;
        locals.var_t2_dn10 = 0.0;
        locals.var_t2_dn11 = 0.0;
        locals.var_t2_dn12 = 0.0;
        locals.var_t2_rv = 0.0;

        let assign3820_e2649: f64 = (p.p54 * locals.var_lg);
        let assign3820_e2650: f64 = (p.p49 + assign3820_e2649);
        let assign3820_e2651: f64 = (-assign3820_e2650);
        locals.var_t3 = assign3820_e2651;
        locals.var_t3_dn0 = 0.0;
        locals.var_t3_dn2 = 0.0;
        locals.var_t3_dn4 = 0.0;
        locals.var_t3_dn5 = 0.0;
        locals.var_t3_dn6 = 0.0;
        locals.var_t3_dn8 = 0.0;
        locals.var_t3_dn10 = 0.0;
        locals.var_t3_dn11 = 0.0;
        locals.var_t3_dn12 = 0.0;
        locals.var_t3_rv = 0.0;

        let assign3830_e2654: f64 = (locals.var_t1 - locals.var_t2);
        let assign3830_e2656: f64 = (assign3830_e2654 - 1e-12);
        locals.var_tmf1 = assign3830_e2656;
        locals.var_tmf1_dn0 = (locals.var_t1_dn0 - locals.var_t2_dn0);
        locals.var_tmf1_dn2 = (locals.var_t1_dn2 - locals.var_t2_dn2);
        locals.var_tmf1_dn4 = (locals.var_t1_dn4 - locals.var_t2_dn4);
        locals.var_tmf1_dn5 = (locals.var_t1_dn5 - locals.var_t2_dn5);
        locals.var_tmf1_dn6 = (locals.var_t1_dn6 - locals.var_t2_dn6);
        locals.var_tmf1_dn8 = (locals.var_t1_dn8 - locals.var_t2_dn8);
        locals.var_tmf1_dn10 = (locals.var_t1_dn10 - locals.var_t2_dn10);
        locals.var_tmf1_dn11 = (locals.var_t1_dn11 - locals.var_t2_dn11);
        locals.var_tmf1_dn12 = (locals.var_t1_dn12 - locals.var_t2_dn12);
        locals.var_tmf1_rv = 0.0;

        let assign3840_e2659: f64 = (4.0 * locals.var_t2);
        let assign3840_e2661: f64 = (assign3840_e2659 * 1e-12);
        locals.var_tmf2 = assign3840_e2661;
        locals.var_tmf2_dn0 = ((4.0 * locals.var_t2_dn0) * 1e-12);
        locals.var_tmf2_dn2 = ((4.0 * locals.var_t2_dn2) * 1e-12);
        locals.var_tmf2_dn4 = ((4.0 * locals.var_t2_dn4) * 1e-12);
        locals.var_tmf2_dn5 = ((4.0 * locals.var_t2_dn5) * 1e-12);
        locals.var_tmf2_dn6 = ((4.0 * locals.var_t2_dn6) * 1e-12);
        locals.var_tmf2_dn8 = ((4.0 * locals.var_t2_dn8) * 1e-12);
        locals.var_tmf2_dn10 = ((4.0 * locals.var_t2_dn10) * 1e-12);
        locals.var_tmf2_dn11 = ((4.0 * locals.var_t2_dn11) * 1e-12);
        locals.var_tmf2_dn12 = ((4.0 * locals.var_t2_dn12) * 1e-12);
        locals.var_tmf2_rv = 0.0;

        let (assign3850_e2668, assign3850_e2668_d_n0, assign3850_e2668_d_n2, assign3850_e2668_d_n4, assign3850_e2668_d_n5, assign3850_e2668_d_n6, assign3850_e2668_d_n8, assign3850_e2668_d_n10, assign3850_e2668_d_n11, assign3850_e2668_d_n12,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    } else {
        let assign3850_e2667: f64 = (-locals.var_tmf2);
        (assign3850_e2667, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12),)
    }
};
        locals.var_tmf2 = assign3850_e2668;
        locals.var_tmf2_dn0 = assign3850_e2668_d_n0;
        locals.var_tmf2_dn2 = assign3850_e2668_d_n2;
        locals.var_tmf2_dn4 = assign3850_e2668_d_n4;
        locals.var_tmf2_dn5 = assign3850_e2668_d_n5;
        locals.var_tmf2_dn6 = assign3850_e2668_d_n6;
        locals.var_tmf2_dn8 = assign3850_e2668_d_n8;
        locals.var_tmf2_dn10 = assign3850_e2668_d_n10;
        locals.var_tmf2_dn11 = assign3850_e2668_d_n11;
        locals.var_tmf2_dn12 = assign3850_e2668_d_n12;
        locals.var_tmf2_rv = 0.0;

        let assign3860_e2671: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign3860_e2673: f64 = (assign3860_e2671 + locals.var_tmf2);
        let assign3860_e2674: f64 = (assign3860_e2673).sqrt();
        locals.var_tmf2 = assign3860_e2674;
        locals.var_tmf2_dn0 = ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign3860_e2674));
        locals.var_tmf2_dn2 = ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign3860_e2674));
        locals.var_tmf2_dn4 = ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign3860_e2674));
        locals.var_tmf2_dn5 = ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign3860_e2674));
        locals.var_tmf2_dn6 = ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign3860_e2674));
        locals.var_tmf2_dn8 = ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign3860_e2674));
        locals.var_tmf2_dn10 = ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign3860_e2674));
        locals.var_tmf2_dn11 = ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign3860_e2674));
        locals.var_tmf2_dn12 = ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign3860_e2674));
        locals.var_tmf2_rv = 0.0;

        let assign3870_e2679: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign3870_e2680: f64 = (1.0 + assign3870_e2679);
        let assign3870_e2681: f64 = (0.5 * assign3870_e2680);
        locals.var_t1 = assign3870_e2681;
        locals.var_t1_dn0 = (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn2 = (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn4 = (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn5 = (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn6 = (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn8 = (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn10 = (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn11 = (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn12 = (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_rv = 0.0;

        let assign3880_e2686: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign3880_e2687: f64 = (0.5 * assign3880_e2686);
        let assign3880_e2688: f64 = (locals.var_t2 + assign3880_e2687);
        locals.var_vfb = assign3880_e2688;
        locals.var_vfb_dn0 = (locals.var_t2_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)));
        locals.var_vfb_dn2 = (locals.var_t2_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)));
        locals.var_vfb_dn4 = (locals.var_t2_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)));
        locals.var_vfb_dn5 = (locals.var_t2_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)));
        locals.var_vfb_dn6 = (locals.var_t2_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)));
        locals.var_vfb_dn8 = (locals.var_t2_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)));
        locals.var_vfb_dn10 = (locals.var_t2_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)));
        locals.var_vfb_dn11 = (locals.var_t2_dn11 + (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)));
        locals.var_vfb_dn12 = (locals.var_t2_dn12 + (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12)));
        locals.var_vfb_rv = 0.0;

        let assign3890_e2691: f64 = (locals.var_vfb - locals.var_t3);
        let assign3890_e2693: f64 = (assign3890_e2691 - 1e-12);
        locals.var_tmf1 = assign3890_e2693;
        locals.var_tmf1_dn0 = (locals.var_vfb_dn0 - locals.var_t3_dn0);
        locals.var_tmf1_dn2 = (locals.var_vfb_dn2 - locals.var_t3_dn2);
        locals.var_tmf1_dn4 = (locals.var_vfb_dn4 - locals.var_t3_dn4);
        locals.var_tmf1_dn5 = (locals.var_vfb_dn5 - locals.var_t3_dn5);
        locals.var_tmf1_dn6 = (locals.var_vfb_dn6 - locals.var_t3_dn6);
        locals.var_tmf1_dn8 = (locals.var_vfb_dn8 - locals.var_t3_dn8);
        locals.var_tmf1_dn10 = (locals.var_vfb_dn10 - locals.var_t3_dn10);
        locals.var_tmf1_dn11 = (locals.var_vfb_dn11 - locals.var_t3_dn11);
        locals.var_tmf1_dn12 = (locals.var_vfb_dn12 - locals.var_t3_dn12);
        locals.var_tmf1_rv = 0.0;

        let assign3900_e2696: f64 = (4.0 * locals.var_t3);
        let assign3900_e2698: f64 = (assign3900_e2696 * 1e-12);
        locals.var_tmf2 = assign3900_e2698;
        locals.var_tmf2_dn0 = ((4.0 * locals.var_t3_dn0) * 1e-12);
        locals.var_tmf2_dn2 = ((4.0 * locals.var_t3_dn2) * 1e-12);
        locals.var_tmf2_dn4 = ((4.0 * locals.var_t3_dn4) * 1e-12);
        locals.var_tmf2_dn5 = ((4.0 * locals.var_t3_dn5) * 1e-12);
        locals.var_tmf2_dn6 = ((4.0 * locals.var_t3_dn6) * 1e-12);
        locals.var_tmf2_dn8 = ((4.0 * locals.var_t3_dn8) * 1e-12);
        locals.var_tmf2_dn10 = ((4.0 * locals.var_t3_dn10) * 1e-12);
        locals.var_tmf2_dn11 = ((4.0 * locals.var_t3_dn11) * 1e-12);
        locals.var_tmf2_dn12 = ((4.0 * locals.var_t3_dn12) * 1e-12);
        locals.var_tmf2_rv = 0.0;

        let (assign3910_e2705, assign3910_e2705_d_n0, assign3910_e2705_d_n2, assign3910_e2705_d_n4, assign3910_e2705_d_n5, assign3910_e2705_d_n6, assign3910_e2705_d_n8, assign3910_e2705_d_n10, assign3910_e2705_d_n11, assign3910_e2705_d_n12,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    } else {
        let assign3910_e2704: f64 = (-locals.var_tmf2);
        (assign3910_e2704, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12),)
    }
};
        locals.var_tmf2 = assign3910_e2705;
        locals.var_tmf2_dn0 = assign3910_e2705_d_n0;
        locals.var_tmf2_dn2 = assign3910_e2705_d_n2;
        locals.var_tmf2_dn4 = assign3910_e2705_d_n4;
        locals.var_tmf2_dn5 = assign3910_e2705_d_n5;
        locals.var_tmf2_dn6 = assign3910_e2705_d_n6;
        locals.var_tmf2_dn8 = assign3910_e2705_d_n8;
        locals.var_tmf2_dn10 = assign3910_e2705_d_n10;
        locals.var_tmf2_dn11 = assign3910_e2705_d_n11;
        locals.var_tmf2_dn12 = assign3910_e2705_d_n12;
        locals.var_tmf2_rv = 0.0;

        let assign3920_e2708: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign3920_e2710: f64 = (assign3920_e2708 + locals.var_tmf2);
        let assign3920_e2711: f64 = (assign3920_e2710).sqrt();
        locals.var_tmf2 = assign3920_e2711;
        locals.var_tmf2_dn0 = ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign3920_e2711));
        locals.var_tmf2_dn2 = ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign3920_e2711));
        locals.var_tmf2_dn4 = ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign3920_e2711));
        locals.var_tmf2_dn5 = ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign3920_e2711));
        locals.var_tmf2_dn6 = ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign3920_e2711));
        locals.var_tmf2_dn8 = ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign3920_e2711));
        locals.var_tmf2_dn10 = ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign3920_e2711));
        locals.var_tmf2_dn11 = ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign3920_e2711));
        locals.var_tmf2_dn12 = ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign3920_e2711));
        locals.var_tmf2_rv = 0.0;

        let assign3930_e2716: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign3930_e2717: f64 = (1.0 + assign3930_e2716);
        let assign3930_e2718: f64 = (0.5 * assign3930_e2717);
        locals.var_t1 = assign3930_e2718;
        locals.var_t1_dn0 = (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn2 = (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn4 = (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn5 = (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn6 = (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn8 = (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn10 = (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn11 = (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_dn12 = (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t1_rv = 0.0;

        let assign3940_e2723: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign3940_e2724: f64 = (0.5 * assign3940_e2723);
        let assign3940_e2725: f64 = (locals.var_t3 + assign3940_e2724);
        locals.var_vfb = assign3940_e2725;
        locals.var_vfb_dn0 = (locals.var_t3_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)));
        locals.var_vfb_dn2 = (locals.var_t3_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)));
        locals.var_vfb_dn4 = (locals.var_t3_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)));
        locals.var_vfb_dn5 = (locals.var_t3_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)));
        locals.var_vfb_dn6 = (locals.var_t3_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)));
        locals.var_vfb_dn8 = (locals.var_t3_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)));
        locals.var_vfb_dn10 = (locals.var_t3_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)));
        locals.var_vfb_dn11 = (locals.var_t3_dn11 + (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)));
        locals.var_vfb_dn12 = (locals.var_t3_dn12 + (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12)));
        locals.var_vfb_rv = 0.0;

        let assign3950_e2727: f64 = (-locals.var_vfb);
        locals.var_vfb = assign3950_e2727;
        locals.var_vfb_dn0 = (-locals.var_vfb_dn0);
        locals.var_vfb_dn2 = (-locals.var_vfb_dn2);
        locals.var_vfb_dn4 = (-locals.var_vfb_dn4);
        locals.var_vfb_dn5 = (-locals.var_vfb_dn5);
        locals.var_vfb_dn6 = (-locals.var_vfb_dn6);
        locals.var_vfb_dn8 = (-locals.var_vfb_dn8);
        locals.var_vfb_dn10 = (-locals.var_vfb_dn10);
        locals.var_vfb_dn11 = (-locals.var_vfb_dn11);
        locals.var_vfb_dn12 = (-locals.var_vfb_dn12);
        locals.var_vfb_rv = 0.0;

        let assign3960_e2730: f64 = (2.0 * locals.var_beta_inv);
        let assign3960_e2733: f64 = (locals.var_uc_nsubs / locals.var_nin);
        let assign3960_e2734: f64 = (assign3960_e2733).ln();
        let assign3960_e2735: f64 = (assign3960_e2730 * assign3960_e2734);
        locals.var_pb2 = assign3960_e2735;
        locals.var_pb2_dn0 = (assign3960_e2730 * ((((locals.var_uc_nsubs_dn0 * locals.var_nin) - (locals.var_uc_nsubs * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign3960_e2733));
        locals.var_pb2_dn2 = (assign3960_e2730 * ((((locals.var_uc_nsubs_dn2 * locals.var_nin) - (locals.var_uc_nsubs * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign3960_e2733));
        locals.var_pb2_dn4 = (((2.0 * locals.var_beta_inv_dn4) * assign3960_e2734) + (assign3960_e2730 * ((((locals.var_uc_nsubs_dn4 * locals.var_nin) - (locals.var_uc_nsubs * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign3960_e2733)));
        locals.var_pb2_dn5 = (assign3960_e2730 * ((((locals.var_uc_nsubs_dn5 * locals.var_nin) - (locals.var_uc_nsubs * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign3960_e2733));
        locals.var_pb2_dn6 = (assign3960_e2730 * ((((locals.var_uc_nsubs_dn6 * locals.var_nin) - (locals.var_uc_nsubs * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign3960_e2733));
        locals.var_pb2_dn8 = (assign3960_e2730 * ((((locals.var_uc_nsubs_dn8 * locals.var_nin) - (locals.var_uc_nsubs * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign3960_e2733));
        locals.var_pb2_dn10 = (assign3960_e2730 * ((((locals.var_uc_nsubs_dn10 * locals.var_nin) - (locals.var_uc_nsubs * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign3960_e2733));
        locals.var_pb2_dn11 = (assign3960_e2730 * ((((locals.var_uc_nsubs_dn11 * locals.var_nin) - (locals.var_uc_nsubs * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign3960_e2733));
        locals.var_pb2_dn12 = (assign3960_e2730 * ((((locals.var_uc_nsubs_dn12 * locals.var_nin) - (locals.var_uc_nsubs * locals.var_nin_dn12)) / (locals.var_nin * locals.var_nin)) / assign3960_e2733));
        locals.var_pb2_rv = 0.0;

        let assign3970_e2738: f64 = (1.034943e-10 / locals.var_q_nsub);
        let assign3970_e2740: f64 = (assign3970_e2738 * locals.var_beta_inv);
        let assign3970_e2741: f64 = (assign3970_e2740).sqrt();
        locals.var_ldby = assign3970_e2741;
        locals.var_ldby_dn0 = (((-((1.034943e-10 * locals.var_q_nsub_dn0) / (locals.var_q_nsub * locals.var_q_nsub))) * locals.var_beta_inv) / (2.0 * assign3970_e2741));
        locals.var_ldby_dn2 = (((-((1.034943e-10 * locals.var_q_nsub_dn2) / (locals.var_q_nsub * locals.var_q_nsub))) * locals.var_beta_inv) / (2.0 * assign3970_e2741));
        locals.var_ldby_dn4 = ((((-((1.034943e-10 * locals.var_q_nsub_dn4) / (locals.var_q_nsub * locals.var_q_nsub))) * locals.var_beta_inv) + (assign3970_e2738 * locals.var_beta_inv_dn4)) / (2.0 * assign3970_e2741));
        locals.var_ldby_dn5 = (((-((1.034943e-10 * locals.var_q_nsub_dn5) / (locals.var_q_nsub * locals.var_q_nsub))) * locals.var_beta_inv) / (2.0 * assign3970_e2741));
        locals.var_ldby_dn6 = (((-((1.034943e-10 * locals.var_q_nsub_dn6) / (locals.var_q_nsub * locals.var_q_nsub))) * locals.var_beta_inv) / (2.0 * assign3970_e2741));
        locals.var_ldby_dn8 = (((-((1.034943e-10 * locals.var_q_nsub_dn8) / (locals.var_q_nsub * locals.var_q_nsub))) * locals.var_beta_inv) / (2.0 * assign3970_e2741));
        locals.var_ldby_dn10 = (((-((1.034943e-10 * locals.var_q_nsub_dn10) / (locals.var_q_nsub * locals.var_q_nsub))) * locals.var_beta_inv) / (2.0 * assign3970_e2741));
        locals.var_ldby_dn11 = (((-((1.034943e-10 * locals.var_q_nsub_dn11) / (locals.var_q_nsub * locals.var_q_nsub))) * locals.var_beta_inv) / (2.0 * assign3970_e2741));
        locals.var_ldby_dn12 = (((-((1.034943e-10 * locals.var_q_nsub_dn12) / (locals.var_q_nsub * locals.var_q_nsub))) * locals.var_beta_inv) / (2.0 * assign3970_e2741));
        locals.var_ldby_rv = 0.0;

        let assign3980_e2744: f64 = (locals.var_q_nsub * 1.414213562373095);
        let assign3980_e2746: f64 = (assign3980_e2744 * locals.var_ldby);
        locals.var_cnst0soi = assign3980_e2746;
        locals.var_cnst0soi_dn0 = (((locals.var_q_nsub_dn0 * 1.414213562373095) * locals.var_ldby) + (assign3980_e2744 * locals.var_ldby_dn0));
        locals.var_cnst0soi_dn2 = (((locals.var_q_nsub_dn2 * 1.414213562373095) * locals.var_ldby) + (assign3980_e2744 * locals.var_ldby_dn2));
        locals.var_cnst0soi_dn4 = (((locals.var_q_nsub_dn4 * 1.414213562373095) * locals.var_ldby) + (assign3980_e2744 * locals.var_ldby_dn4));
        locals.var_cnst0soi_dn5 = (((locals.var_q_nsub_dn5 * 1.414213562373095) * locals.var_ldby) + (assign3980_e2744 * locals.var_ldby_dn5));
        locals.var_cnst0soi_dn6 = (((locals.var_q_nsub_dn6 * 1.414213562373095) * locals.var_ldby) + (assign3980_e2744 * locals.var_ldby_dn6));
        locals.var_cnst0soi_dn8 = (((locals.var_q_nsub_dn8 * 1.414213562373095) * locals.var_ldby) + (assign3980_e2744 * locals.var_ldby_dn8));
        locals.var_cnst0soi_dn10 = (((locals.var_q_nsub_dn10 * 1.414213562373095) * locals.var_ldby) + (assign3980_e2744 * locals.var_ldby_dn10));
        locals.var_cnst0soi_dn11 = (((locals.var_q_nsub_dn11 * 1.414213562373095) * locals.var_ldby) + (assign3980_e2744 * locals.var_ldby_dn11));
        locals.var_cnst0soi_dn12 = (((locals.var_q_nsub_dn12 * 1.414213562373095) * locals.var_ldby) + (assign3980_e2744 * locals.var_ldby_dn12));
        locals.var_cnst0soi_rv = 0.0;

        locals.var_c0bulk = locals.var_qnbulk_esi;
        locals.var_c0bulk_dn0 = locals.var_qnbulk_esi_dn0;
        locals.var_c0bulk_dn2 = locals.var_qnbulk_esi_dn2;
        locals.var_c0bulk_dn4 = locals.var_qnbulk_esi_dn4;
        locals.var_c0bulk_dn5 = locals.var_qnbulk_esi_dn5;
        locals.var_c0bulk_dn6 = locals.var_qnbulk_esi_dn6;
        locals.var_c0bulk_dn8 = locals.var_qnbulk_esi_dn8;
        locals.var_c0bulk_dn10 = locals.var_qnbulk_esi_dn10;
        locals.var_c0bulk_dn11 = locals.var_qnbulk_esi_dn11;
        locals.var_c0bulk_dn12 = locals.var_qnbulk_esi_dn12;
        locals.var_c0bulk_rv = 0.0;

        let assign4000_e2750: f64 = (2.0 * locals.var_c0bulk);
        let assign4000_e2752: f64 = (assign4000_e2750 * locals.var_beta_inv);
        let assign4000_e2753: f64 = (assign4000_e2752).sqrt();
        locals.var_cnst0bulk = assign4000_e2753;
        locals.var_cnst0bulk_dn0 = (((2.0 * locals.var_c0bulk_dn0) * locals.var_beta_inv) / (2.0 * assign4000_e2753));
        locals.var_cnst0bulk_dn2 = (((2.0 * locals.var_c0bulk_dn2) * locals.var_beta_inv) / (2.0 * assign4000_e2753));
        locals.var_cnst0bulk_dn4 = ((((2.0 * locals.var_c0bulk_dn4) * locals.var_beta_inv) + (assign4000_e2750 * locals.var_beta_inv_dn4)) / (2.0 * assign4000_e2753));
        locals.var_cnst0bulk_dn5 = (((2.0 * locals.var_c0bulk_dn5) * locals.var_beta_inv) / (2.0 * assign4000_e2753));
        locals.var_cnst0bulk_dn6 = (((2.0 * locals.var_c0bulk_dn6) * locals.var_beta_inv) / (2.0 * assign4000_e2753));
        locals.var_cnst0bulk_dn8 = (((2.0 * locals.var_c0bulk_dn8) * locals.var_beta_inv) / (2.0 * assign4000_e2753));
        locals.var_cnst0bulk_dn10 = (((2.0 * locals.var_c0bulk_dn10) * locals.var_beta_inv) / (2.0 * assign4000_e2753));
        locals.var_cnst0bulk_dn11 = (((2.0 * locals.var_c0bulk_dn11) * locals.var_beta_inv) / (2.0 * assign4000_e2753));
        locals.var_cnst0bulk_dn12 = (((2.0 * locals.var_c0bulk_dn12) * locals.var_beta_inv) / (2.0 * assign4000_e2753));
        locals.var_cnst0bulk_rv = 0.0;

        let assign4010_e2756: f64 = (locals.var_nin / locals.var_uc_nsubs);
        locals.var_t1 = assign4010_e2756;
        locals.var_t1_dn0 = (((locals.var_nin_dn0 * locals.var_uc_nsubs) - (locals.var_nin * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs));
        locals.var_t1_dn2 = (((locals.var_nin_dn2 * locals.var_uc_nsubs) - (locals.var_nin * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs));
        locals.var_t1_dn4 = (((locals.var_nin_dn4 * locals.var_uc_nsubs) - (locals.var_nin * locals.var_uc_nsubs_dn4)) / (locals.var_uc_nsubs * locals.var_uc_nsubs));
        locals.var_t1_dn5 = (((locals.var_nin_dn5 * locals.var_uc_nsubs) - (locals.var_nin * locals.var_uc_nsubs_dn5)) / (locals.var_uc_nsubs * locals.var_uc_nsubs));
        locals.var_t1_dn6 = (((locals.var_nin_dn6 * locals.var_uc_nsubs) - (locals.var_nin * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs));
        locals.var_t1_dn8 = (((locals.var_nin_dn8 * locals.var_uc_nsubs) - (locals.var_nin * locals.var_uc_nsubs_dn8)) / (locals.var_uc_nsubs * locals.var_uc_nsubs));
        locals.var_t1_dn10 = (((locals.var_nin_dn10 * locals.var_uc_nsubs) - (locals.var_nin * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs));
        locals.var_t1_dn11 = (((locals.var_nin_dn11 * locals.var_uc_nsubs) - (locals.var_nin * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs));
        locals.var_t1_dn12 = (((locals.var_nin_dn12 * locals.var_uc_nsubs) - (locals.var_nin * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs));
        locals.var_t1_rv = 0.0;

        let assign4020_e2759: f64 = (locals.var_t1 * locals.var_t1);
        locals.var_cnst1soi = assign4020_e2759;
        locals.var_cnst1soi_dn0 = ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0));
        locals.var_cnst1soi_dn2 = ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2));
        locals.var_cnst1soi_dn4 = ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4));
        locals.var_cnst1soi_dn5 = ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5));
        locals.var_cnst1soi_dn6 = ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6));
        locals.var_cnst1soi_dn8 = ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8));
        locals.var_cnst1soi_dn10 = ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10));
        locals.var_cnst1soi_dn11 = ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11));
        locals.var_cnst1soi_dn12 = ((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12));
        locals.var_cnst1soi_rv = 0.0;

        let assign4030_e2762: f64 = (locals.var_nin / locals.var_n_subbl);
        locals.var_t1 = assign4030_e2762;
        locals.var_t1_dn0 = (((locals.var_nin_dn0 * locals.var_n_subbl) - (locals.var_nin * locals.var_n_subbl_dn0)) / (locals.var_n_subbl * locals.var_n_subbl));
        locals.var_t1_dn2 = (((locals.var_nin_dn2 * locals.var_n_subbl) - (locals.var_nin * locals.var_n_subbl_dn2)) / (locals.var_n_subbl * locals.var_n_subbl));
        locals.var_t1_dn4 = (((locals.var_nin_dn4 * locals.var_n_subbl) - (locals.var_nin * locals.var_n_subbl_dn4)) / (locals.var_n_subbl * locals.var_n_subbl));
        locals.var_t1_dn5 = (((locals.var_nin_dn5 * locals.var_n_subbl) - (locals.var_nin * locals.var_n_subbl_dn5)) / (locals.var_n_subbl * locals.var_n_subbl));
        locals.var_t1_dn6 = (((locals.var_nin_dn6 * locals.var_n_subbl) - (locals.var_nin * locals.var_n_subbl_dn6)) / (locals.var_n_subbl * locals.var_n_subbl));
        locals.var_t1_dn8 = (((locals.var_nin_dn8 * locals.var_n_subbl) - (locals.var_nin * locals.var_n_subbl_dn8)) / (locals.var_n_subbl * locals.var_n_subbl));
        locals.var_t1_dn10 = (((locals.var_nin_dn10 * locals.var_n_subbl) - (locals.var_nin * locals.var_n_subbl_dn10)) / (locals.var_n_subbl * locals.var_n_subbl));
        locals.var_t1_dn11 = (((locals.var_nin_dn11 * locals.var_n_subbl) - (locals.var_nin * locals.var_n_subbl_dn11)) / (locals.var_n_subbl * locals.var_n_subbl));
        locals.var_t1_dn12 = (((locals.var_nin_dn12 * locals.var_n_subbl) - (locals.var_nin * locals.var_n_subbl_dn12)) / (locals.var_n_subbl * locals.var_n_subbl));
        locals.var_t1_rv = 0.0;

        let assign4040_e2765: f64 = (locals.var_t1 * locals.var_t1);
        locals.var_cnst1bulk = assign4040_e2765;
        locals.var_cnst1bulk_dn0 = ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0));
        locals.var_cnst1bulk_dn2 = ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2));
        locals.var_cnst1bulk_dn4 = ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4));
        locals.var_cnst1bulk_dn5 = ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5));
        locals.var_cnst1bulk_dn6 = ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6));
        locals.var_cnst1bulk_dn8 = ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8));
        locals.var_cnst1bulk_dn10 = ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10));
        locals.var_cnst1bulk_dn11 = ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11));
        locals.var_cnst1bulk_dn12 = ((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12));
        locals.var_cnst1bulk_rv = 0.0;

        locals.var_tfox0 = p.p226;
        locals.var_tfox0_rv = 0.0;

        let assign4060_e2769: f64 = (3.453133e-11 / locals.var_tfox0);
        locals.var_c_fox0 = assign4060_e2769;
        locals.var_c_fox0_rv = 0.0;

        let assign4070_e2772: f64 = (locals.var_tfox0 / 3.453133e-11);
        locals.var_c_fox0_inv = assign4070_e2772;
        locals.var_c_fox0_inv_rv = 0.0;

        let assign4080_e2775: f64 = (3.453133e-11 / p.p229);
        locals.var_c_box = assign4080_e2775;
        locals.var_c_box_rv = 0.0;

        let assign4090_e2778: f64 = (p.p229 / 3.453133e-11);
        locals.var_c_box_inv = assign4090_e2778;
        locals.var_c_box_inv_rv = 0.0;

        let assign4100_e2780: f64 = (-1.6021918e-19);
        let assign4100_e2782: f64 = (assign4100_e2780 * locals.var_uc_nsubs);
        let assign4100_e2784: f64 = (assign4100_e2782 * p.p227);
        locals.var_q_fd_soi = assign4100_e2784;
        locals.var_q_fd_soi_dn0 = ((assign4100_e2780 * locals.var_uc_nsubs_dn0) * p.p227);
        locals.var_q_fd_soi_dn2 = ((assign4100_e2780 * locals.var_uc_nsubs_dn2) * p.p227);
        locals.var_q_fd_soi_dn4 = ((assign4100_e2780 * locals.var_uc_nsubs_dn4) * p.p227);
        locals.var_q_fd_soi_dn5 = ((assign4100_e2780 * locals.var_uc_nsubs_dn5) * p.p227);
        locals.var_q_fd_soi_dn6 = ((assign4100_e2780 * locals.var_uc_nsubs_dn6) * p.p227);
        locals.var_q_fd_soi_dn8 = ((assign4100_e2780 * locals.var_uc_nsubs_dn8) * p.p227);
        locals.var_q_fd_soi_dn10 = ((assign4100_e2780 * locals.var_uc_nsubs_dn10) * p.p227);
        locals.var_q_fd_soi_dn11 = ((assign4100_e2780 * locals.var_uc_nsubs_dn11) * p.p227);
        locals.var_q_fd_soi_dn12 = ((assign4100_e2780 * locals.var_uc_nsubs_dn12) * p.p227);
        locals.var_q_fd_soi_rv = 0.0;

        let assign4110_e2787: f64 = (1.034943e-10 / p.p227);
        locals.var_c_soi = assign4110_e2787;
        locals.var_c_soi_rv = 0.0;

        let assign4120_e2790: f64 = (1.0 / locals.var_c_soi);
        locals.var_c_soi_inv = assign4120_e2790;
        locals.var_c_soi_inv_rv = 0.0;

        let assign4130_e2793: f64 = (locals.var_c_box_inv + locals.var_c_soi_inv);
        locals.var_c_box_fd_inv = assign4130_e2793;
        locals.var_c_box_fd_inv_rv = 0.0;

        locals.var_vbs_bnd = p.p254;
        locals.var_vbs_bnd_rv = 0.0;

        locals.var_vbs_max = p.p255;
        locals.var_vbs_max_rv = 0.0;

        let assign4230_e2821: f64 = (locals.var_vbs_max * 0.5);
        let assign4230_e2822: f64 = if locals.var_vbs_bnd > assign4230_e2821 { 1.0 } else { 0.0 };
        locals.var_guard37 = assign4230_e2822;
        locals.var_guard37_rv = 0.0;

        let (assign4240_e2828,) = {
    if (locals.var_guard37 != 0.0) {
        let assign4240_e2826: f64 = (0.5 * locals.var_vbs_max);
        (assign4240_e2826,)
    } else {
        (locals.var_vbs_bnd,)
    }
};
        locals.var_vbs_bnd = assign4240_e2828;
        locals.var_vbs_bnd_rv = 0.0;

        let assign4250_e2831: f64 = if locals.var_vbs_mos > locals.var_vbs_bnd { 1.0 } else { 0.0 };
        locals.var_guard38 = assign4250_e2831;
        locals.var_guard38_rv = 0.0;

        let (assign4260_e2837, assign4260_e2837_d_n0, assign4260_e2837_d_n2, assign4260_e2837_d_n4, assign4260_e2837_d_n5, assign4260_e2837_d_n6, assign4260_e2837_d_n8, assign4260_e2837_d_n10, assign4260_e2837_d_n11, assign4260_e2837_d_n12,) = {
    if (locals.var_guard38 != 0.0) {
        let assign4260_e2835: f64 = (locals.var_vbs_mos - locals.var_vbs_bnd);
        (assign4260_e2835, 0.0, 0.0, 0.0, 0.0, locals.var_vbs_mos_dn6, 0.0, 0.0, locals.var_vbs_mos_dn11, locals.var_vbs_mos_dn12,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign4260_e2837;
        locals.var_t2_dn0 = assign4260_e2837_d_n0;
        locals.var_t2_dn2 = assign4260_e2837_d_n2;
        locals.var_t2_dn4 = assign4260_e2837_d_n4;
        locals.var_t2_dn5 = assign4260_e2837_d_n5;
        locals.var_t2_dn6 = assign4260_e2837_d_n6;
        locals.var_t2_dn8 = assign4260_e2837_d_n8;
        locals.var_t2_dn10 = assign4260_e2837_d_n10;
        locals.var_t2_dn11 = assign4260_e2837_d_n11;
        locals.var_t2_dn12 = assign4260_e2837_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign4270_e2843, assign4270_e2843_d_n0, assign4270_e2843_d_n2, assign4270_e2843_d_n4, assign4270_e2843_d_n5, assign4270_e2843_d_n6, assign4270_e2843_d_n8, assign4270_e2843_d_n10, assign4270_e2843_d_n11, assign4270_e2843_d_n12,) = {
    if (locals.var_guard38 != 0.0) {
        let assign4270_e2841: f64 = (locals.var_vbs_max - locals.var_vbs_bnd);
        (assign4270_e2841, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign4270_e2843;
        locals.var_t3_dn0 = assign4270_e2843_d_n0;
        locals.var_t3_dn2 = assign4270_e2843_d_n2;
        locals.var_t3_dn4 = assign4270_e2843_d_n4;
        locals.var_t3_dn5 = assign4270_e2843_d_n5;
        locals.var_t3_dn6 = assign4270_e2843_d_n6;
        locals.var_t3_dn8 = assign4270_e2843_d_n8;
        locals.var_t3_dn10 = assign4270_e2843_d_n10;
        locals.var_t3_dn11 = assign4270_e2843_d_n11;
        locals.var_t3_dn12 = assign4270_e2843_d_n12;
        locals.var_t3_rv = 0.0;

        let (assign4280_e2849, assign4280_e2849_d_n0, assign4280_e2849_d_n2, assign4280_e2849_d_n4, assign4280_e2849_d_n5, assign4280_e2849_d_n6, assign4280_e2849_d_n8, assign4280_e2849_d_n10, assign4280_e2849_d_n11, assign4280_e2849_d_n12,) = {
    if (locals.var_guard38 != 0.0) {
        let assign4280_e2847: f64 = (locals.var_t2 * locals.var_t2);
        (assign4280_e2847, ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)), ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)), ((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)),)
    } else {
        (locals.var_x2, locals.var_x2_dn0, locals.var_x2_dn2, locals.var_x2_dn4, locals.var_x2_dn5, locals.var_x2_dn6, locals.var_x2_dn8, locals.var_x2_dn10, locals.var_x2_dn11, locals.var_x2_dn12,)
    }
};
        locals.var_x2 = assign4280_e2849;
        locals.var_x2_dn0 = assign4280_e2849_d_n0;
        locals.var_x2_dn2 = assign4280_e2849_d_n2;
        locals.var_x2_dn4 = assign4280_e2849_d_n4;
        locals.var_x2_dn5 = assign4280_e2849_d_n5;
        locals.var_x2_dn6 = assign4280_e2849_d_n6;
        locals.var_x2_dn8 = assign4280_e2849_d_n8;
        locals.var_x2_dn10 = assign4280_e2849_d_n10;
        locals.var_x2_dn11 = assign4280_e2849_d_n11;
        locals.var_x2_dn12 = assign4280_e2849_d_n12;
        locals.var_x2_rv = 0.0;

        let (assign4290_e2855, assign4290_e2855_d_n0, assign4290_e2855_d_n2, assign4290_e2855_d_n4, assign4290_e2855_d_n5, assign4290_e2855_d_n6, assign4290_e2855_d_n8, assign4290_e2855_d_n10, assign4290_e2855_d_n11, assign4290_e2855_d_n12,) = {
    if (locals.var_guard38 != 0.0) {
        let assign4290_e2853: f64 = (locals.var_t3 * locals.var_t3);
        (assign4290_e2853, ((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)), ((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)), ((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)), ((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)), ((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)), ((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)), ((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)), ((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11)), ((locals.var_t3_dn12 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn12)),)
    } else {
        (locals.var_xmax2, locals.var_xmax2_dn0, locals.var_xmax2_dn2, locals.var_xmax2_dn4, locals.var_xmax2_dn5, locals.var_xmax2_dn6, locals.var_xmax2_dn8, locals.var_xmax2_dn10, locals.var_xmax2_dn11, locals.var_xmax2_dn12,)
    }
};
        locals.var_xmax2 = assign4290_e2855;
        locals.var_xmax2_dn0 = assign4290_e2855_d_n0;
        locals.var_xmax2_dn2 = assign4290_e2855_d_n2;
        locals.var_xmax2_dn4 = assign4290_e2855_d_n4;
        locals.var_xmax2_dn5 = assign4290_e2855_d_n5;
        locals.var_xmax2_dn6 = assign4290_e2855_d_n6;
        locals.var_xmax2_dn8 = assign4290_e2855_d_n8;
        locals.var_xmax2_dn10 = assign4290_e2855_d_n10;
        locals.var_xmax2_dn11 = assign4290_e2855_d_n11;
        locals.var_xmax2_dn12 = assign4290_e2855_d_n12;
        locals.var_xmax2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_10(
        locals: &mut StampLocals,
    ) {
        let (assign4300_e2859, assign4300_e2859_d_n0, assign4300_e2859_d_n2, assign4300_e2859_d_n4, assign4300_e2859_d_n5, assign4300_e2859_d_n6, assign4300_e2859_d_n8, assign4300_e2859_d_n10, assign4300_e2859_d_n11, assign4300_e2859_d_n12,) = {
    if (locals.var_guard38 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn8, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12,)
    }
};
        locals.var_xp = assign4300_e2859;
        locals.var_xp_dn0 = assign4300_e2859_d_n0;
        locals.var_xp_dn2 = assign4300_e2859_d_n2;
        locals.var_xp_dn4 = assign4300_e2859_d_n4;
        locals.var_xp_dn5 = assign4300_e2859_d_n5;
        locals.var_xp_dn6 = assign4300_e2859_d_n6;
        locals.var_xp_dn8 = assign4300_e2859_d_n8;
        locals.var_xp_dn10 = assign4300_e2859_d_n10;
        locals.var_xp_dn11 = assign4300_e2859_d_n11;
        locals.var_xp_dn12 = assign4300_e2859_d_n12;
        locals.var_xp_rv = 0.0;

        let (assign4310_e2863, assign4310_e2863_d_n0, assign4310_e2863_d_n2, assign4310_e2863_d_n4, assign4310_e2863_d_n5, assign4310_e2863_d_n6, assign4310_e2863_d_n8, assign4310_e2863_d_n10, assign4310_e2863_d_n11, assign4310_e2863_d_n12,) = {
    if (locals.var_guard38 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn8, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12,)
    }
};
        locals.var_xmp = assign4310_e2863;
        locals.var_xmp_dn0 = assign4310_e2863_d_n0;
        locals.var_xmp_dn2 = assign4310_e2863_d_n2;
        locals.var_xmp_dn4 = assign4310_e2863_d_n4;
        locals.var_xmp_dn5 = assign4310_e2863_d_n5;
        locals.var_xmp_dn6 = assign4310_e2863_d_n6;
        locals.var_xmp_dn8 = assign4310_e2863_d_n8;
        locals.var_xmp_dn10 = assign4310_e2863_d_n10;
        locals.var_xmp_dn11 = assign4310_e2863_d_n11;
        locals.var_xmp_dn12 = assign4310_e2863_d_n12;
        locals.var_xmp_rv = 0.0;

        let (assign4320_e2867,) = {
    if (locals.var_guard38 != 0.0) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign4320_e2867;
        locals.var_m0_rv = 0.0;

        let (assign4330_e2871,) = {
    if (locals.var_guard38 != 0.0) {
        (0.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign4330_e2871;
        locals.var_mm_rv = 0.0;

        let (assign4340_e2875, assign4340_e2875_d_n0, assign4340_e2875_d_n2, assign4340_e2875_d_n4, assign4340_e2875_d_n5, assign4340_e2875_d_n6, assign4340_e2875_d_n8, assign4340_e2875_d_n10, assign4340_e2875_d_n11, assign4340_e2875_d_n12,) = {
    if (locals.var_guard38 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn8, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12,)
    }
};
        locals.var_arg = assign4340_e2875;
        locals.var_arg_dn0 = assign4340_e2875_d_n0;
        locals.var_arg_dn2 = assign4340_e2875_d_n2;
        locals.var_arg_dn4 = assign4340_e2875_d_n4;
        locals.var_arg_dn5 = assign4340_e2875_d_n5;
        locals.var_arg_dn6 = assign4340_e2875_d_n6;
        locals.var_arg_dn8 = assign4340_e2875_d_n8;
        locals.var_arg_dn10 = assign4340_e2875_d_n10;
        locals.var_arg_dn11 = assign4340_e2875_d_n11;
        locals.var_arg_dn12 = assign4340_e2875_d_n12;
        locals.var_arg_rv = 0.0;

        let (assign4350_e2879, assign4350_e2879_d_n0, assign4350_e2879_d_n2, assign4350_e2879_d_n4, assign4350_e2879_d_n5, assign4350_e2879_d_n6, assign4350_e2879_d_n8, assign4350_e2879_d_n10, assign4350_e2879_d_n11, assign4350_e2879_d_n12,) = {
    if (locals.var_guard38 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn8, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12,)
    }
};
        locals.var_dnm = assign4350_e2879;
        locals.var_dnm_dn0 = assign4350_e2879_d_n0;
        locals.var_dnm_dn2 = assign4350_e2879_d_n2;
        locals.var_dnm_dn4 = assign4350_e2879_d_n4;
        locals.var_dnm_dn5 = assign4350_e2879_d_n5;
        locals.var_dnm_dn6 = assign4350_e2879_d_n6;
        locals.var_dnm_dn8 = assign4350_e2879_d_n8;
        locals.var_dnm_dn10 = assign4350_e2879_d_n10;
        locals.var_dnm_dn11 = assign4350_e2879_d_n11;
        locals.var_dnm_dn12 = assign4350_e2879_d_n12;
        locals.var_dnm_rv = 0.0;

        let (assign4360_e2885, assign4360_e2885_d_n0, assign4360_e2885_d_n2, assign4360_e2885_d_n4, assign4360_e2885_d_n5, assign4360_e2885_d_n6, assign4360_e2885_d_n8, assign4360_e2885_d_n10, assign4360_e2885_d_n11, assign4360_e2885_d_n12,) = {
    if (locals.var_guard38 != 0.0) {
        let assign4360_e2883: f64 = (locals.var_xp * locals.var_x2);
        (assign4360_e2883, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn8, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12,)
    }
};
        locals.var_xp = assign4360_e2885;
        locals.var_xp_dn0 = assign4360_e2885_d_n0;
        locals.var_xp_dn2 = assign4360_e2885_d_n2;
        locals.var_xp_dn4 = assign4360_e2885_d_n4;
        locals.var_xp_dn5 = assign4360_e2885_d_n5;
        locals.var_xp_dn6 = assign4360_e2885_d_n6;
        locals.var_xp_dn8 = assign4360_e2885_d_n8;
        locals.var_xp_dn10 = assign4360_e2885_d_n10;
        locals.var_xp_dn11 = assign4360_e2885_d_n11;
        locals.var_xp_dn12 = assign4360_e2885_d_n12;
        locals.var_xp_rv = 0.0;

        let (assign4370_e2891, assign4370_e2891_d_n0, assign4370_e2891_d_n2, assign4370_e2891_d_n4, assign4370_e2891_d_n5, assign4370_e2891_d_n6, assign4370_e2891_d_n8, assign4370_e2891_d_n10, assign4370_e2891_d_n11, assign4370_e2891_d_n12,) = {
    if (locals.var_guard38 != 0.0) {
        let assign4370_e2889: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign4370_e2889, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn8, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12,)
    }
};
        locals.var_xmp = assign4370_e2891;
        locals.var_xmp_dn0 = assign4370_e2891_d_n0;
        locals.var_xmp_dn2 = assign4370_e2891_d_n2;
        locals.var_xmp_dn4 = assign4370_e2891_d_n4;
        locals.var_xmp_dn5 = assign4370_e2891_d_n5;
        locals.var_xmp_dn6 = assign4370_e2891_d_n6;
        locals.var_xmp_dn8 = assign4370_e2891_d_n8;
        locals.var_xmp_dn10 = assign4370_e2891_d_n10;
        locals.var_xmp_dn11 = assign4370_e2891_d_n11;
        locals.var_xmp_dn12 = assign4370_e2891_d_n12;
        locals.var_xmp_rv = 0.0;

        let (assign4380_e2897, assign4380_e2897_d_n0, assign4380_e2897_d_n2, assign4380_e2897_d_n4, assign4380_e2897_d_n5, assign4380_e2897_d_n6, assign4380_e2897_d_n8, assign4380_e2897_d_n10, assign4380_e2897_d_n11, assign4380_e2897_d_n12,) = {
    if (locals.var_guard38 != 0.0) {
        let assign4380_e2895: f64 = (locals.var_xp * locals.var_x2);
        (assign4380_e2895, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn8, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12,)
    }
};
        locals.var_xp = assign4380_e2897;
        locals.var_xp_dn0 = assign4380_e2897_d_n0;
        locals.var_xp_dn2 = assign4380_e2897_d_n2;
        locals.var_xp_dn4 = assign4380_e2897_d_n4;
        locals.var_xp_dn5 = assign4380_e2897_d_n5;
        locals.var_xp_dn6 = assign4380_e2897_d_n6;
        locals.var_xp_dn8 = assign4380_e2897_d_n8;
        locals.var_xp_dn10 = assign4380_e2897_d_n10;
        locals.var_xp_dn11 = assign4380_e2897_d_n11;
        locals.var_xp_dn12 = assign4380_e2897_d_n12;
        locals.var_xp_rv = 0.0;

        let (assign4390_e2903, assign4390_e2903_d_n0, assign4390_e2903_d_n2, assign4390_e2903_d_n4, assign4390_e2903_d_n5, assign4390_e2903_d_n6, assign4390_e2903_d_n8, assign4390_e2903_d_n10, assign4390_e2903_d_n11, assign4390_e2903_d_n12,) = {
    if (locals.var_guard38 != 0.0) {
        let assign4390_e2901: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign4390_e2901, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn8, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12,)
    }
};
        locals.var_xmp = assign4390_e2903;
        locals.var_xmp_dn0 = assign4390_e2903_d_n0;
        locals.var_xmp_dn2 = assign4390_e2903_d_n2;
        locals.var_xmp_dn4 = assign4390_e2903_d_n4;
        locals.var_xmp_dn5 = assign4390_e2903_d_n5;
        locals.var_xmp_dn6 = assign4390_e2903_d_n6;
        locals.var_xmp_dn8 = assign4390_e2903_d_n8;
        locals.var_xmp_dn10 = assign4390_e2903_d_n10;
        locals.var_xmp_dn11 = assign4390_e2903_d_n11;
        locals.var_xmp_dn12 = assign4390_e2903_d_n12;
        locals.var_xmp_rv = 0.0;

        let (assign4400_e2909, assign4400_e2909_d_n0, assign4400_e2909_d_n2, assign4400_e2909_d_n4, assign4400_e2909_d_n5, assign4400_e2909_d_n6, assign4400_e2909_d_n8, assign4400_e2909_d_n10, assign4400_e2909_d_n11, assign4400_e2909_d_n12,) = {
    if (locals.var_guard38 != 0.0) {
        let assign4400_e2907: f64 = (locals.var_xp * locals.var_x2);
        (assign4400_e2907, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn8, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12,)
    }
};
        locals.var_xp = assign4400_e2909;
        locals.var_xp_dn0 = assign4400_e2909_d_n0;
        locals.var_xp_dn2 = assign4400_e2909_d_n2;
        locals.var_xp_dn4 = assign4400_e2909_d_n4;
        locals.var_xp_dn5 = assign4400_e2909_d_n5;
        locals.var_xp_dn6 = assign4400_e2909_d_n6;
        locals.var_xp_dn8 = assign4400_e2909_d_n8;
        locals.var_xp_dn10 = assign4400_e2909_d_n10;
        locals.var_xp_dn11 = assign4400_e2909_d_n11;
        locals.var_xp_dn12 = assign4400_e2909_d_n12;
        locals.var_xp_rv = 0.0;

        let (assign4410_e2915, assign4410_e2915_d_n0, assign4410_e2915_d_n2, assign4410_e2915_d_n4, assign4410_e2915_d_n5, assign4410_e2915_d_n6, assign4410_e2915_d_n8, assign4410_e2915_d_n10, assign4410_e2915_d_n11, assign4410_e2915_d_n12,) = {
    if (locals.var_guard38 != 0.0) {
        let assign4410_e2913: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign4410_e2913, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn8, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12,)
    }
};
        locals.var_xmp = assign4410_e2915;
        locals.var_xmp_dn0 = assign4410_e2915_d_n0;
        locals.var_xmp_dn2 = assign4410_e2915_d_n2;
        locals.var_xmp_dn4 = assign4410_e2915_d_n4;
        locals.var_xmp_dn5 = assign4410_e2915_d_n5;
        locals.var_xmp_dn6 = assign4410_e2915_d_n6;
        locals.var_xmp_dn8 = assign4410_e2915_d_n8;
        locals.var_xmp_dn10 = assign4410_e2915_d_n10;
        locals.var_xmp_dn11 = assign4410_e2915_d_n11;
        locals.var_xmp_dn12 = assign4410_e2915_d_n12;
        locals.var_xmp_rv = 0.0;

        let (assign4420_e2921, assign4420_e2921_d_n0, assign4420_e2921_d_n2, assign4420_e2921_d_n4, assign4420_e2921_d_n5, assign4420_e2921_d_n6, assign4420_e2921_d_n8, assign4420_e2921_d_n10, assign4420_e2921_d_n11, assign4420_e2921_d_n12,) = {
    if (locals.var_guard38 != 0.0) {
        let assign4420_e2919: f64 = (locals.var_xp * locals.var_x2);
        (assign4420_e2919, ((locals.var_xp_dn0 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn0)), ((locals.var_xp_dn2 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn2)), ((locals.var_xp_dn4 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn4)), ((locals.var_xp_dn5 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn5)), ((locals.var_xp_dn6 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn6)), ((locals.var_xp_dn8 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn8)), ((locals.var_xp_dn10 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn10)), ((locals.var_xp_dn11 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn11)), ((locals.var_xp_dn12 * locals.var_x2) + (locals.var_xp * locals.var_x2_dn12)),)
    } else {
        (locals.var_xp, locals.var_xp_dn0, locals.var_xp_dn2, locals.var_xp_dn4, locals.var_xp_dn5, locals.var_xp_dn6, locals.var_xp_dn8, locals.var_xp_dn10, locals.var_xp_dn11, locals.var_xp_dn12,)
    }
};
        locals.var_xp = assign4420_e2921;
        locals.var_xp_dn0 = assign4420_e2921_d_n0;
        locals.var_xp_dn2 = assign4420_e2921_d_n2;
        locals.var_xp_dn4 = assign4420_e2921_d_n4;
        locals.var_xp_dn5 = assign4420_e2921_d_n5;
        locals.var_xp_dn6 = assign4420_e2921_d_n6;
        locals.var_xp_dn8 = assign4420_e2921_d_n8;
        locals.var_xp_dn10 = assign4420_e2921_d_n10;
        locals.var_xp_dn11 = assign4420_e2921_d_n11;
        locals.var_xp_dn12 = assign4420_e2921_d_n12;
        locals.var_xp_rv = 0.0;

        let (assign4430_e2927, assign4430_e2927_d_n0, assign4430_e2927_d_n2, assign4430_e2927_d_n4, assign4430_e2927_d_n5, assign4430_e2927_d_n6, assign4430_e2927_d_n8, assign4430_e2927_d_n10, assign4430_e2927_d_n11, assign4430_e2927_d_n12,) = {
    if (locals.var_guard38 != 0.0) {
        let assign4430_e2925: f64 = (locals.var_xmp * locals.var_xmax2);
        (assign4430_e2925, ((locals.var_xmp_dn0 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn0)), ((locals.var_xmp_dn2 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn2)), ((locals.var_xmp_dn4 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn4)), ((locals.var_xmp_dn5 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn5)), ((locals.var_xmp_dn6 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn6)), ((locals.var_xmp_dn8 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn8)), ((locals.var_xmp_dn10 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn10)), ((locals.var_xmp_dn11 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn11)), ((locals.var_xmp_dn12 * locals.var_xmax2) + (locals.var_xmp * locals.var_xmax2_dn12)),)
    } else {
        (locals.var_xmp, locals.var_xmp_dn0, locals.var_xmp_dn2, locals.var_xmp_dn4, locals.var_xmp_dn5, locals.var_xmp_dn6, locals.var_xmp_dn8, locals.var_xmp_dn10, locals.var_xmp_dn11, locals.var_xmp_dn12,)
    }
};
        locals.var_xmp = assign4430_e2927;
        locals.var_xmp_dn0 = assign4430_e2927_d_n0;
        locals.var_xmp_dn2 = assign4430_e2927_d_n2;
        locals.var_xmp_dn4 = assign4430_e2927_d_n4;
        locals.var_xmp_dn5 = assign4430_e2927_d_n5;
        locals.var_xmp_dn6 = assign4430_e2927_d_n6;
        locals.var_xmp_dn8 = assign4430_e2927_d_n8;
        locals.var_xmp_dn10 = assign4430_e2927_d_n10;
        locals.var_xmp_dn11 = assign4430_e2927_d_n11;
        locals.var_xmp_dn12 = assign4430_e2927_d_n12;
        locals.var_xmp_rv = 0.0;

        let (assign4440_e2933, assign4440_e2933_d_n0, assign4440_e2933_d_n2, assign4440_e2933_d_n4, assign4440_e2933_d_n5, assign4440_e2933_d_n6, assign4440_e2933_d_n8, assign4440_e2933_d_n10, assign4440_e2933_d_n11, assign4440_e2933_d_n12,) = {
    if (locals.var_guard38 != 0.0) {
        let assign4440_e2931: f64 = (locals.var_xp + locals.var_xmp);
        (assign4440_e2931, (locals.var_xp_dn0 + locals.var_xmp_dn0), (locals.var_xp_dn2 + locals.var_xmp_dn2), (locals.var_xp_dn4 + locals.var_xmp_dn4), (locals.var_xp_dn5 + locals.var_xmp_dn5), (locals.var_xp_dn6 + locals.var_xmp_dn6), (locals.var_xp_dn8 + locals.var_xmp_dn8), (locals.var_xp_dn10 + locals.var_xmp_dn10), (locals.var_xp_dn11 + locals.var_xmp_dn11), (locals.var_xp_dn12 + locals.var_xmp_dn12),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn8, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12,)
    }
};
        locals.var_arg = assign4440_e2933;
        locals.var_arg_dn0 = assign4440_e2933_d_n0;
        locals.var_arg_dn2 = assign4440_e2933_d_n2;
        locals.var_arg_dn4 = assign4440_e2933_d_n4;
        locals.var_arg_dn5 = assign4440_e2933_d_n5;
        locals.var_arg_dn6 = assign4440_e2933_d_n6;
        locals.var_arg_dn8 = assign4440_e2933_d_n8;
        locals.var_arg_dn10 = assign4440_e2933_d_n10;
        locals.var_arg_dn11 = assign4440_e2933_d_n11;
        locals.var_arg_dn12 = assign4440_e2933_d_n12;
        locals.var_arg_rv = 0.0;

        let (assign4450_e2937, assign4450_e2937_d_n0, assign4450_e2937_d_n2, assign4450_e2937_d_n4, assign4450_e2937_d_n5, assign4450_e2937_d_n6, assign4450_e2937_d_n8, assign4450_e2937_d_n10, assign4450_e2937_d_n11, assign4450_e2937_d_n12,) = {
    if (locals.var_guard38 != 0.0) {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn8, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12,)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn8, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12,)
    }
};
        locals.var_dnm = assign4450_e2937;
        locals.var_dnm_dn0 = assign4450_e2937_d_n0;
        locals.var_dnm_dn2 = assign4450_e2937_d_n2;
        locals.var_dnm_dn4 = assign4450_e2937_d_n4;
        locals.var_dnm_dn5 = assign4450_e2937_d_n5;
        locals.var_dnm_dn6 = assign4450_e2937_d_n6;
        locals.var_dnm_dn8 = assign4450_e2937_d_n8;
        locals.var_dnm_dn10 = assign4450_e2937_d_n10;
        locals.var_dnm_dn11 = assign4450_e2937_d_n11;
        locals.var_dnm_dn12 = assign4450_e2937_d_n12;
        locals.var_dnm_rv = 0.0;

        let assign4460_e2952: f64 = if ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0)) { 1.0 } else { 0.0 };
        locals.var_guard39 = assign4460_e2952;
        locals.var_guard39_rv = 0.0;

        let assign4470_e2955: f64 = if 4.0 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard40 = assign4470_e2955;
        locals.var_guard40_rv = 0.0;

        let (assign4480_e2963,) = {
    if (((locals.var_guard38 != 0.0) && (locals.var_guard39 != 0.0)) && (locals.var_guard40 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign4480_e2963;
        locals.var_mm_rv = 0.0;

        let assign4490_e2966: f64 = if 4.0 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard41 = assign4490_e2966;
        locals.var_guard41_rv = 0.0;

        let (assign4500_e2977,) = {
    if ((((locals.var_guard38 != 0.0) && (locals.var_guard39 != 0.0)) && (locals.var_guard40 == 0.0)) && (locals.var_guard41 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign4500_e2977;
        locals.var_mm_rv = 0.0;

        let assign4510_e2980: f64 = if 4.0 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard42 = assign4510_e2980;
        locals.var_guard42_rv = 0.0;

        let (assign4520_e2994,) = {
    if (((((locals.var_guard38 != 0.0) && (locals.var_guard39 != 0.0)) && (locals.var_guard40 == 0.0)) && (locals.var_guard41 == 0.0)) && (locals.var_guard42 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign4520_e2994;
        locals.var_mm_rv = 0.0;

        let assign4530_e2997: f64 = if 4.0 == 8.0 { 1.0 } else { 0.0 };
        locals.var_guard43 = assign4530_e2997;
        locals.var_guard43_rv = 0.0;

        let (assign4540_e3014,) = {
    if ((((((locals.var_guard38 != 0.0) && (locals.var_guard39 != 0.0)) && (locals.var_guard40 == 0.0)) && (locals.var_guard41 == 0.0)) && (locals.var_guard42 == 0.0)) && (locals.var_guard43 != 0.0)) {
        (4.0,)
    } else {
        (locals.var_mm,)
    }
};
        locals.var_mm = assign4540_e3014;
        locals.var_mm_rv = 0.0;

        let (assign4550_e3020,) = {
    if ((locals.var_guard38 != 0.0) && (locals.var_guard39 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_m0,)
    }
};
        locals.var_m0 = assign4550_e3020;
        locals.var_m0_rv = 0.0;

        let mut assign4560_loop_guard: usize = 0;
        while {
            let assign4560_cond_e3027: f64 = if (((locals.var_guard38 != 0.0) && (locals.var_guard39 != 0.0)) && (locals.var_m0 < locals.var_mm)) { 1.0 } else { 0.0 };
            assign4560_cond_e3027 != 0.0
        } {
            assign4560_loop_guard += 1;
            assert!(assign4560_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign4560_body0_e3034, assign4560_body0_e3034_d_n0, assign4560_body0_e3034_d_n2, assign4560_body0_e3034_d_n4, assign4560_body0_e3034_d_n5, assign4560_body0_e3034_d_n6, assign4560_body0_e3034_d_n8, assign4560_body0_e3034_d_n10, assign4560_body0_e3034_d_n11, assign4560_body0_e3034_d_n12,) = {
    if ((locals.var_guard38 != 0.0) && (locals.var_guard39 != 0.0)) {
        let assign4560_body0_e3032: f64 = (locals.var_dnm).sqrt();
        (assign4560_body0_e3032, (locals.var_dnm_dn0 / (2.0 * assign4560_body0_e3032)), (locals.var_dnm_dn2 / (2.0 * assign4560_body0_e3032)), (locals.var_dnm_dn4 / (2.0 * assign4560_body0_e3032)), (locals.var_dnm_dn5 / (2.0 * assign4560_body0_e3032)), (locals.var_dnm_dn6 / (2.0 * assign4560_body0_e3032)), (locals.var_dnm_dn8 / (2.0 * assign4560_body0_e3032)), (locals.var_dnm_dn10 / (2.0 * assign4560_body0_e3032)), (locals.var_dnm_dn11 / (2.0 * assign4560_body0_e3032)), (locals.var_dnm_dn12 / (2.0 * assign4560_body0_e3032)),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn8, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12,)
    }
};
            locals.var_dnm = assign4560_body0_e3034;
            locals.var_dnm_dn0 = assign4560_body0_e3034_d_n0;
            locals.var_dnm_dn2 = assign4560_body0_e3034_d_n2;
            locals.var_dnm_dn4 = assign4560_body0_e3034_d_n4;
            locals.var_dnm_dn5 = assign4560_body0_e3034_d_n5;
            locals.var_dnm_dn6 = assign4560_body0_e3034_d_n6;
            locals.var_dnm_dn8 = assign4560_body0_e3034_d_n8;
            locals.var_dnm_dn10 = assign4560_body0_e3034_d_n10;
            locals.var_dnm_dn11 = assign4560_body0_e3034_d_n11;
            locals.var_dnm_dn12 = assign4560_body0_e3034_d_n12;
            locals.var_dnm_rv = 0.0;
            let (assign4560_body1_e3042,) = {
    if ((locals.var_guard38 != 0.0) && (locals.var_guard39 != 0.0)) {
        let assign4560_body1_e3040: f64 = (locals.var_m0 + 1.0);
        (assign4560_body1_e3040,)
    } else {
        (locals.var_m0,)
    }
};
            locals.var_m0 = assign4560_body1_e3042;
            locals.var_m0_rv = 0.0;
        }

        let (assign4570_e3055, assign4570_e3055_d_n0, assign4570_e3055_d_n2, assign4570_e3055_d_n4, assign4570_e3055_d_n5, assign4570_e3055_d_n6, assign4570_e3055_d_n8, assign4570_e3055_d_n10, assign4570_e3055_d_n11, assign4570_e3055_d_n12,) = {
    if ((locals.var_guard38 != 0.0) && (locals.var_guard39 == 0.0)) {
        let assign4570_e3051: f64 = (2.0 * 4.0);
        let assign4570_e3052: f64 = (1.0 / assign4570_e3051);
        let assign4570_e3053: f64 = (locals.var_dnm).powf(assign4570_e3052);
        (assign4570_e3053, if 0.0 == 0.0 && ((assign4570_e3052) as f64).is_finite() && ((assign4570_e3052) as f64).fract() == 0.0 { if assign4570_e3052 == 0.0 { 0.0 } else { (assign4570_e3052 * ((locals.var_dnm).powf(assign4570_e3052 - 1.0) * locals.var_dnm_dn0)) } } else { (assign4570_e3053 * (assign4570_e3052 * (locals.var_dnm_dn0 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign4570_e3052) as f64).is_finite() && ((assign4570_e3052) as f64).fract() == 0.0 { if assign4570_e3052 == 0.0 { 0.0 } else { (assign4570_e3052 * ((locals.var_dnm).powf(assign4570_e3052 - 1.0) * locals.var_dnm_dn2)) } } else { (assign4570_e3053 * (assign4570_e3052 * (locals.var_dnm_dn2 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign4570_e3052) as f64).is_finite() && ((assign4570_e3052) as f64).fract() == 0.0 { if assign4570_e3052 == 0.0 { 0.0 } else { (assign4570_e3052 * ((locals.var_dnm).powf(assign4570_e3052 - 1.0) * locals.var_dnm_dn4)) } } else { (assign4570_e3053 * (assign4570_e3052 * (locals.var_dnm_dn4 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign4570_e3052) as f64).is_finite() && ((assign4570_e3052) as f64).fract() == 0.0 { if assign4570_e3052 == 0.0 { 0.0 } else { (assign4570_e3052 * ((locals.var_dnm).powf(assign4570_e3052 - 1.0) * locals.var_dnm_dn5)) } } else { (assign4570_e3053 * (assign4570_e3052 * (locals.var_dnm_dn5 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign4570_e3052) as f64).is_finite() && ((assign4570_e3052) as f64).fract() == 0.0 { if assign4570_e3052 == 0.0 { 0.0 } else { (assign4570_e3052 * ((locals.var_dnm).powf(assign4570_e3052 - 1.0) * locals.var_dnm_dn6)) } } else { (assign4570_e3053 * (assign4570_e3052 * (locals.var_dnm_dn6 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign4570_e3052) as f64).is_finite() && ((assign4570_e3052) as f64).fract() == 0.0 { if assign4570_e3052 == 0.0 { 0.0 } else { (assign4570_e3052 * ((locals.var_dnm).powf(assign4570_e3052 - 1.0) * locals.var_dnm_dn8)) } } else { (assign4570_e3053 * (assign4570_e3052 * (locals.var_dnm_dn8 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign4570_e3052) as f64).is_finite() && ((assign4570_e3052) as f64).fract() == 0.0 { if assign4570_e3052 == 0.0 { 0.0 } else { (assign4570_e3052 * ((locals.var_dnm).powf(assign4570_e3052 - 1.0) * locals.var_dnm_dn10)) } } else { (assign4570_e3053 * (assign4570_e3052 * (locals.var_dnm_dn10 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign4570_e3052) as f64).is_finite() && ((assign4570_e3052) as f64).fract() == 0.0 { if assign4570_e3052 == 0.0 { 0.0 } else { (assign4570_e3052 * ((locals.var_dnm).powf(assign4570_e3052 - 1.0) * locals.var_dnm_dn11)) } } else { (assign4570_e3053 * (assign4570_e3052 * (locals.var_dnm_dn11 / locals.var_dnm))) }, if 0.0 == 0.0 && ((assign4570_e3052) as f64).is_finite() && ((assign4570_e3052) as f64).fract() == 0.0 { if assign4570_e3052 == 0.0 { 0.0 } else { (assign4570_e3052 * ((locals.var_dnm).powf(assign4570_e3052 - 1.0) * locals.var_dnm_dn12)) } } else { (assign4570_e3053 * (assign4570_e3052 * (locals.var_dnm_dn12 / locals.var_dnm))) },)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn8, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12,)
    }
};
        locals.var_dnm = assign4570_e3055;
        locals.var_dnm_dn0 = assign4570_e3055_d_n0;
        locals.var_dnm_dn2 = assign4570_e3055_d_n2;
        locals.var_dnm_dn4 = assign4570_e3055_d_n4;
        locals.var_dnm_dn5 = assign4570_e3055_d_n5;
        locals.var_dnm_dn6 = assign4570_e3055_d_n6;
        locals.var_dnm_dn8 = assign4570_e3055_d_n8;
        locals.var_dnm_dn10 = assign4570_e3055_d_n10;
        locals.var_dnm_dn11 = assign4570_e3055_d_n11;
        locals.var_dnm_dn12 = assign4570_e3055_d_n12;
        locals.var_dnm_rv = 0.0;

        let (assign4580_e3063, assign4580_e3063_d_n0, assign4580_e3063_d_n2, assign4580_e3063_d_n4, assign4580_e3063_d_n5, assign4580_e3063_d_n6, assign4580_e3063_d_n8, assign4580_e3063_d_n10, assign4580_e3063_d_n11, assign4580_e3063_d_n12,) = {
    if (locals.var_guard38 != 0.0) {
        let assign4580_e3060: f64 = (locals.var_dnm + 1e-50);
        let assign4580_e3061: f64 = (1.0 / assign4580_e3060);
        (assign4580_e3061, (-(locals.var_dnm_dn0 / (assign4580_e3060 * assign4580_e3060))), (-(locals.var_dnm_dn2 / (assign4580_e3060 * assign4580_e3060))), (-(locals.var_dnm_dn4 / (assign4580_e3060 * assign4580_e3060))), (-(locals.var_dnm_dn5 / (assign4580_e3060 * assign4580_e3060))), (-(locals.var_dnm_dn6 / (assign4580_e3060 * assign4580_e3060))), (-(locals.var_dnm_dn8 / (assign4580_e3060 * assign4580_e3060))), (-(locals.var_dnm_dn10 / (assign4580_e3060 * assign4580_e3060))), (-(locals.var_dnm_dn11 / (assign4580_e3060 * assign4580_e3060))), (-(locals.var_dnm_dn12 / (assign4580_e3060 * assign4580_e3060))),)
    } else {
        (locals.var_dnm, locals.var_dnm_dn0, locals.var_dnm_dn2, locals.var_dnm_dn4, locals.var_dnm_dn5, locals.var_dnm_dn6, locals.var_dnm_dn8, locals.var_dnm_dn10, locals.var_dnm_dn11, locals.var_dnm_dn12,)
    }
};
        locals.var_dnm = assign4580_e3063;
        locals.var_dnm_dn0 = assign4580_e3063_d_n0;
        locals.var_dnm_dn2 = assign4580_e3063_d_n2;
        locals.var_dnm_dn4 = assign4580_e3063_d_n4;
        locals.var_dnm_dn5 = assign4580_e3063_d_n5;
        locals.var_dnm_dn6 = assign4580_e3063_d_n6;
        locals.var_dnm_dn8 = assign4580_e3063_d_n8;
        locals.var_dnm_dn10 = assign4580_e3063_d_n10;
        locals.var_dnm_dn11 = assign4580_e3063_d_n11;
        locals.var_dnm_dn12 = assign4580_e3063_d_n12;
        locals.var_dnm_rv = 0.0;

        let (assign4590_e3071, assign4590_e3071_d_n0, assign4590_e3071_d_n2, assign4590_e3071_d_n4, assign4590_e3071_d_n5, assign4590_e3071_d_n6, assign4590_e3071_d_n8, assign4590_e3071_d_n10, assign4590_e3071_d_n11, assign4590_e3071_d_n12,) = {
    if (locals.var_guard38 != 0.0) {
        let assign4590_e3067: f64 = (locals.var_t2 * locals.var_t3);
        let assign4590_e3069: f64 = (assign4590_e3067 * locals.var_dnm);
        (assign4590_e3069, ((((locals.var_t2_dn0 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn0)) * locals.var_dnm) + (assign4590_e3067 * locals.var_dnm_dn0)), ((((locals.var_t2_dn2 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn2)) * locals.var_dnm) + (assign4590_e3067 * locals.var_dnm_dn2)), ((((locals.var_t2_dn4 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn4)) * locals.var_dnm) + (assign4590_e3067 * locals.var_dnm_dn4)), ((((locals.var_t2_dn5 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn5)) * locals.var_dnm) + (assign4590_e3067 * locals.var_dnm_dn5)), ((((locals.var_t2_dn6 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn6)) * locals.var_dnm) + (assign4590_e3067 * locals.var_dnm_dn6)), ((((locals.var_t2_dn8 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn8)) * locals.var_dnm) + (assign4590_e3067 * locals.var_dnm_dn8)), ((((locals.var_t2_dn10 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn10)) * locals.var_dnm) + (assign4590_e3067 * locals.var_dnm_dn10)), ((((locals.var_t2_dn11 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn11)) * locals.var_dnm) + (assign4590_e3067 * locals.var_dnm_dn11)), ((((locals.var_t2_dn12 * locals.var_t3) + (locals.var_t2 * locals.var_t3_dn12)) * locals.var_dnm) + (assign4590_e3067 * locals.var_dnm_dn12)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign4590_e3071;
        locals.var_t4_dn0 = assign4590_e3071_d_n0;
        locals.var_t4_dn2 = assign4590_e3071_d_n2;
        locals.var_t4_dn4 = assign4590_e3071_d_n4;
        locals.var_t4_dn5 = assign4590_e3071_d_n5;
        locals.var_t4_dn6 = assign4590_e3071_d_n6;
        locals.var_t4_dn8 = assign4590_e3071_d_n8;
        locals.var_t4_dn10 = assign4590_e3071_d_n10;
        locals.var_t4_dn11 = assign4590_e3071_d_n11;
        locals.var_t4_dn12 = assign4590_e3071_d_n12;
        locals.var_t4_rv = 0.0;

        let (assign4600_e3083, assign4600_e3083_d_n0, assign4600_e3083_d_n2, assign4600_e3083_d_n4, assign4600_e3083_d_n5, assign4600_e3083_d_n6, assign4600_e3083_d_n8, assign4600_e3083_d_n10, assign4600_e3083_d_n11, assign4600_e3083_d_n12,) = {
    if (locals.var_guard38 != 0.0) {
        let assign4600_e3075: f64 = (locals.var_t3 * locals.var_xmp);
        let assign4600_e3077: f64 = (assign4600_e3075 * locals.var_dnm);
        let assign4600_e3080: f64 = (locals.var_arg + 1e-50);
        let assign4600_e3081: f64 = (assign4600_e3077 / assign4600_e3080);
        (assign4600_e3081, (((((((locals.var_t3_dn0 * locals.var_xmp) + (locals.var_t3 * locals.var_xmp_dn0)) * locals.var_dnm) + (assign4600_e3075 * locals.var_dnm_dn0)) * assign4600_e3080) - (assign4600_e3077 * locals.var_arg_dn0)) / (assign4600_e3080 * assign4600_e3080)), (((((((locals.var_t3_dn2 * locals.var_xmp) + (locals.var_t3 * locals.var_xmp_dn2)) * locals.var_dnm) + (assign4600_e3075 * locals.var_dnm_dn2)) * assign4600_e3080) - (assign4600_e3077 * locals.var_arg_dn2)) / (assign4600_e3080 * assign4600_e3080)), (((((((locals.var_t3_dn4 * locals.var_xmp) + (locals.var_t3 * locals.var_xmp_dn4)) * locals.var_dnm) + (assign4600_e3075 * locals.var_dnm_dn4)) * assign4600_e3080) - (assign4600_e3077 * locals.var_arg_dn4)) / (assign4600_e3080 * assign4600_e3080)), (((((((locals.var_t3_dn5 * locals.var_xmp) + (locals.var_t3 * locals.var_xmp_dn5)) * locals.var_dnm) + (assign4600_e3075 * locals.var_dnm_dn5)) * assign4600_e3080) - (assign4600_e3077 * locals.var_arg_dn5)) / (assign4600_e3080 * assign4600_e3080)), (((((((locals.var_t3_dn6 * locals.var_xmp) + (locals.var_t3 * locals.var_xmp_dn6)) * locals.var_dnm) + (assign4600_e3075 * locals.var_dnm_dn6)) * assign4600_e3080) - (assign4600_e3077 * locals.var_arg_dn6)) / (assign4600_e3080 * assign4600_e3080)), (((((((locals.var_t3_dn8 * locals.var_xmp) + (locals.var_t3 * locals.var_xmp_dn8)) * locals.var_dnm) + (assign4600_e3075 * locals.var_dnm_dn8)) * assign4600_e3080) - (assign4600_e3077 * locals.var_arg_dn8)) / (assign4600_e3080 * assign4600_e3080)), (((((((locals.var_t3_dn10 * locals.var_xmp) + (locals.var_t3 * locals.var_xmp_dn10)) * locals.var_dnm) + (assign4600_e3075 * locals.var_dnm_dn10)) * assign4600_e3080) - (assign4600_e3077 * locals.var_arg_dn10)) / (assign4600_e3080 * assign4600_e3080)), (((((((locals.var_t3_dn11 * locals.var_xmp) + (locals.var_t3 * locals.var_xmp_dn11)) * locals.var_dnm) + (assign4600_e3075 * locals.var_dnm_dn11)) * assign4600_e3080) - (assign4600_e3077 * locals.var_arg_dn11)) / (assign4600_e3080 * assign4600_e3080)), (((((((locals.var_t3_dn12 * locals.var_xmp) + (locals.var_t3 * locals.var_xmp_dn12)) * locals.var_dnm) + (assign4600_e3075 * locals.var_dnm_dn12)) * assign4600_e3080) - (assign4600_e3077 * locals.var_arg_dn12)) / (assign4600_e3080 * assign4600_e3080)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn8, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12,)
    }
};
        locals.var_t8 = assign4600_e3083;
        locals.var_t8_dn0 = assign4600_e3083_d_n0;
        locals.var_t8_dn2 = assign4600_e3083_d_n2;
        locals.var_t8_dn4 = assign4600_e3083_d_n4;
        locals.var_t8_dn5 = assign4600_e3083_d_n5;
        locals.var_t8_dn6 = assign4600_e3083_d_n6;
        locals.var_t8_dn8 = assign4600_e3083_d_n8;
        locals.var_t8_dn10 = assign4600_e3083_d_n10;
        locals.var_t8_dn11 = assign4600_e3083_d_n11;
        locals.var_t8_dn12 = assign4600_e3083_d_n12;
        locals.var_t8_rv = 0.0;

        let (assign4610_e3089, assign4610_e3089_d_n0, assign4610_e3089_d_n2, assign4610_e3089_d_n4, assign4610_e3089_d_n5, assign4610_e3089_d_n6, assign4610_e3089_d_n8, assign4610_e3089_d_n10, assign4610_e3089_d_n11, assign4610_e3089_d_n12,) = {
    if (locals.var_guard38 != 0.0) {
        let assign4610_e3087: f64 = (locals.var_vbs_bnd + locals.var_t4);
        (assign4610_e3087, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    } else {
        (locals.var_vbsc, locals.var_vbsc_dn0, locals.var_vbsc_dn2, locals.var_vbsc_dn4, locals.var_vbsc_dn5, locals.var_vbsc_dn6, locals.var_vbsc_dn8, locals.var_vbsc_dn10, locals.var_vbsc_dn11, locals.var_vbsc_dn12,)
    }
};
        locals.var_vbsc = assign4610_e3089;
        locals.var_vbsc_dn0 = assign4610_e3089_d_n0;
        locals.var_vbsc_dn2 = assign4610_e3089_d_n2;
        locals.var_vbsc_dn4 = assign4610_e3089_d_n4;
        locals.var_vbsc_dn5 = assign4610_e3089_d_n5;
        locals.var_vbsc_dn6 = assign4610_e3089_d_n6;
        locals.var_vbsc_dn8 = assign4610_e3089_d_n8;
        locals.var_vbsc_dn10 = assign4610_e3089_d_n10;
        locals.var_vbsc_dn11 = assign4610_e3089_d_n11;
        locals.var_vbsc_dn12 = assign4610_e3089_d_n12;
        locals.var_vbsc_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign4620_e3093, assign4620_e3093_d_n0, assign4620_e3093_d_n2, assign4620_e3093_d_n4, assign4620_e3093_d_n5, assign4620_e3093_d_n6, assign4620_e3093_d_n8, assign4620_e3093_d_n10, assign4620_e3093_d_n11, assign4620_e3093_d_n12,) = {
    if (locals.var_guard38 != 0.0) {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn8, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12,)
    } else {
        (locals.var_vbsc_dvbs, locals.var_vbsc_dvbs_dn0, locals.var_vbsc_dvbs_dn2, locals.var_vbsc_dvbs_dn4, locals.var_vbsc_dvbs_dn5, locals.var_vbsc_dvbs_dn6, locals.var_vbsc_dvbs_dn8, locals.var_vbsc_dvbs_dn10, locals.var_vbsc_dvbs_dn11, locals.var_vbsc_dvbs_dn12,)
    }
};
        locals.var_vbsc_dvbs = assign4620_e3093;
        locals.var_vbsc_dvbs_dn0 = assign4620_e3093_d_n0;
        locals.var_vbsc_dvbs_dn2 = assign4620_e3093_d_n2;
        locals.var_vbsc_dvbs_dn4 = assign4620_e3093_d_n4;
        locals.var_vbsc_dvbs_dn5 = assign4620_e3093_d_n5;
        locals.var_vbsc_dvbs_dn6 = assign4620_e3093_d_n6;
        locals.var_vbsc_dvbs_dn8 = assign4620_e3093_d_n8;
        locals.var_vbsc_dvbs_dn10 = assign4620_e3093_d_n10;
        locals.var_vbsc_dvbs_dn11 = assign4620_e3093_d_n11;
        locals.var_vbsc_dvbs_dn12 = assign4620_e3093_d_n12;
        locals.var_vbsc_dvbs_rv = 0.0;

        let (assign4630_e3098, assign4630_e3098_d_n0, assign4630_e3098_d_n2, assign4630_e3098_d_n4, assign4630_e3098_d_n5, assign4630_e3098_d_n6, assign4630_e3098_d_n8, assign4630_e3098_d_n10, assign4630_e3098_d_n11, assign4630_e3098_d_n12,) = {
    if (locals.var_guard38 == 0.0) {
        (locals.var_vbs_mos, 0.0, 0.0, 0.0, 0.0, locals.var_vbs_mos_dn6, 0.0, 0.0, locals.var_vbs_mos_dn11, locals.var_vbs_mos_dn12,)
    } else {
        (locals.var_vbsc, locals.var_vbsc_dn0, locals.var_vbsc_dn2, locals.var_vbsc_dn4, locals.var_vbsc_dn5, locals.var_vbsc_dn6, locals.var_vbsc_dn8, locals.var_vbsc_dn10, locals.var_vbsc_dn11, locals.var_vbsc_dn12,)
    }
};
        locals.var_vbsc = assign4630_e3098;
        locals.var_vbsc_dn0 = assign4630_e3098_d_n0;
        locals.var_vbsc_dn2 = assign4630_e3098_d_n2;
        locals.var_vbsc_dn4 = assign4630_e3098_d_n4;
        locals.var_vbsc_dn5 = assign4630_e3098_d_n5;
        locals.var_vbsc_dn6 = assign4630_e3098_d_n6;
        locals.var_vbsc_dn8 = assign4630_e3098_d_n8;
        locals.var_vbsc_dn10 = assign4630_e3098_d_n10;
        locals.var_vbsc_dn11 = assign4630_e3098_d_n11;
        locals.var_vbsc_dn12 = assign4630_e3098_d_n12;
        locals.var_vbsc_rv = 0.0;

        let (assign4640_e3103, assign4640_e3103_d_n0, assign4640_e3103_d_n2, assign4640_e3103_d_n4, assign4640_e3103_d_n5, assign4640_e3103_d_n6, assign4640_e3103_d_n8, assign4640_e3103_d_n10, assign4640_e3103_d_n11, assign4640_e3103_d_n12,) = {
    if (locals.var_guard38 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbsc_dvbs, locals.var_vbsc_dvbs_dn0, locals.var_vbsc_dvbs_dn2, locals.var_vbsc_dvbs_dn4, locals.var_vbsc_dvbs_dn5, locals.var_vbsc_dvbs_dn6, locals.var_vbsc_dvbs_dn8, locals.var_vbsc_dvbs_dn10, locals.var_vbsc_dvbs_dn11, locals.var_vbsc_dvbs_dn12,)
    }
};
        locals.var_vbsc_dvbs = assign4640_e3103;
        locals.var_vbsc_dvbs_dn0 = assign4640_e3103_d_n0;
        locals.var_vbsc_dvbs_dn2 = assign4640_e3103_d_n2;
        locals.var_vbsc_dvbs_dn4 = assign4640_e3103_d_n4;
        locals.var_vbsc_dvbs_dn5 = assign4640_e3103_d_n5;
        locals.var_vbsc_dvbs_dn6 = assign4640_e3103_d_n6;
        locals.var_vbsc_dvbs_dn8 = assign4640_e3103_d_n8;
        locals.var_vbsc_dvbs_dn10 = assign4640_e3103_d_n10;
        locals.var_vbsc_dvbs_dn11 = assign4640_e3103_d_n11;
        locals.var_vbsc_dvbs_dn12 = assign4640_e3103_d_n12;
        locals.var_vbsc_dvbs_rv = 0.0;

        locals.var_vdsc = locals.var_vds_mos;
        locals.var_vdsc_dn11 = locals.var_vds_mos_dn11;
        locals.var_vdsc_dn12 = locals.var_vds_mos_dn12;
        locals.var_vdsc_rv = 0.0;

        locals.var_vgsc = locals.var_vgs_mos;
        locals.var_vgsc_dn5 = locals.var_vgs_mos_dn5;
        locals.var_vgsc_dn11 = locals.var_vgs_mos_dn11;
        locals.var_vgsc_dn12 = locals.var_vgs_mos_dn12;
        locals.var_vgsc_rv = 0.0;

        locals.var_flg_pprv = 0.0;
        locals.var_flg_pprv_rv = 0.0;

        locals.var_pss0_ini = 0.0;
        locals.var_pss0_ini_rv = 0.0;

        locals.var_pbs0_ini = 0.0;
        locals.var_pbs0_ini_rv = 0.0;

        locals.var_psb0_ini = 0.0;
        locals.var_psb0_ini_rv = 0.0;

        locals.var_pssl_ini = 0.0;
        locals.var_pssl_ini_rv = 0.0;

        locals.var_pbsl_ini = 0.0;
        locals.var_pbsl_ini_rv = 0.0;

        locals.var_psbl_ini = 0.0;
        locals.var_psbl_ini_rv = 0.0;

        locals.var_vbs = locals.var_vbsc;
        locals.var_vbs_dn0 = locals.var_vbsc_dn0;
        locals.var_vbs_dn2 = locals.var_vbsc_dn2;
        locals.var_vbs_dn4 = locals.var_vbsc_dn4;
        locals.var_vbs_dn5 = locals.var_vbsc_dn5;
        locals.var_vbs_dn6 = locals.var_vbsc_dn6;
        locals.var_vbs_dn8 = locals.var_vbsc_dn8;
        locals.var_vbs_dn10 = locals.var_vbsc_dn10;
        locals.var_vbs_dn11 = locals.var_vbsc_dn11;
        locals.var_vbs_dn12 = locals.var_vbsc_dn12;
        locals.var_vbs_rv = 0.0;

        locals.var_vds = locals.var_vdsc;
        locals.var_vds_dn0 = 0.0;
        locals.var_vds_dn2 = 0.0;
        locals.var_vds_dn4 = 0.0;
        locals.var_vds_dn5 = 0.0;
        locals.var_vds_dn6 = 0.0;
        locals.var_vds_dn8 = 0.0;
        locals.var_vds_dn10 = 0.0;
        locals.var_vds_dn11 = locals.var_vdsc_dn11;
        locals.var_vds_dn12 = locals.var_vdsc_dn12;
        locals.var_vds_rv = 0.0;

        locals.var_vgs = locals.var_vgsc;
        locals.var_vgs_dn5 = locals.var_vgsc_dn5;
        locals.var_vgs_dn11 = locals.var_vgsc_dn11;
        locals.var_vgs_dn12 = locals.var_vgsc_dn12;
        locals.var_vgs_rv = 0.0;

        locals.var_lp_s0 = 0.0;
        locals.var_lp_s0_rv = 0.0;

        locals.var_lp_sl = 0.0;
        locals.var_lp_sl_rv = 0.0;

        let assign4790_e3120: f64 = (locals.var_vbsc_dvbs * locals.var_vds);
        let assign4790_e3122: f64 = (assign4790_e3120 / 2.0);
        locals.var_t1 = assign4790_e3122;
        locals.var_t1_dn0 = (((locals.var_vbsc_dvbs_dn0 * locals.var_vds) + (locals.var_vbsc_dvbs * locals.var_vds_dn0)) / 2.0);
        locals.var_t1_dn2 = (((locals.var_vbsc_dvbs_dn2 * locals.var_vds) + (locals.var_vbsc_dvbs * locals.var_vds_dn2)) / 2.0);
        locals.var_t1_dn4 = (((locals.var_vbsc_dvbs_dn4 * locals.var_vds) + (locals.var_vbsc_dvbs * locals.var_vds_dn4)) / 2.0);
        locals.var_t1_dn5 = (((locals.var_vbsc_dvbs_dn5 * locals.var_vds) + (locals.var_vbsc_dvbs * locals.var_vds_dn5)) / 2.0);
        locals.var_t1_dn6 = (((locals.var_vbsc_dvbs_dn6 * locals.var_vds) + (locals.var_vbsc_dvbs * locals.var_vds_dn6)) / 2.0);
        locals.var_t1_dn8 = (((locals.var_vbsc_dvbs_dn8 * locals.var_vds) + (locals.var_vbsc_dvbs * locals.var_vds_dn8)) / 2.0);
        locals.var_t1_dn10 = (((locals.var_vbsc_dvbs_dn10 * locals.var_vds) + (locals.var_vbsc_dvbs * locals.var_vds_dn10)) / 2.0);
        locals.var_t1_dn11 = (((locals.var_vbsc_dvbs_dn11 * locals.var_vds) + (locals.var_vbsc_dvbs * locals.var_vds_dn11)) / 2.0);
        locals.var_t1_dn12 = (((locals.var_vbsc_dvbs_dn12 * locals.var_vds) + (locals.var_vbsc_dvbs * locals.var_vds_dn12)) / 2.0);
        locals.var_t1_rv = 0.0;

        let assign4800_e3125: f64 = (2.0 * locals.var_t1);
        let assign4800_e3127: f64 = (assign4800_e3125 / p.p216);
        locals.var_tmf1 = assign4800_e3127;
        locals.var_tmf1_dn0 = ((2.0 * locals.var_t1_dn0) / p.p216);
        locals.var_tmf1_dn2 = ((2.0 * locals.var_t1_dn2) / p.p216);
        locals.var_tmf1_dn4 = ((2.0 * locals.var_t1_dn4) / p.p216);
        locals.var_tmf1_dn5 = ((2.0 * locals.var_t1_dn5) / p.p216);
        locals.var_tmf1_dn6 = ((2.0 * locals.var_t1_dn6) / p.p216);
        locals.var_tmf1_dn8 = ((2.0 * locals.var_t1_dn8) / p.p216);
        locals.var_tmf1_dn10 = ((2.0 * locals.var_t1_dn10) / p.p216);
        locals.var_tmf1_dn11 = ((2.0 * locals.var_t1_dn11) / p.p216);
        locals.var_tmf1_dn12 = ((2.0 * locals.var_t1_dn12) / p.p216);
        locals.var_tmf1_rv = 0.0;

        let assign4810_e3132: f64 = (1.0 / 2.0);
        let assign4810_e3136: f64 = (1.0 / 6.0);
        let assign4810_e3140: f64 = (1.0 / 24.0);
        let assign4810_e3144: f64 = (1.0 / 120.0);
        let assign4810_e3148: f64 = (1.0 / 720.0);
        let assign4810_e3152: f64 = (1.0 / 5040.0);
        let assign4810_e3153: f64 = (locals.var_tmf1 * assign4810_e3152);
        let assign4810_e3154: f64 = (assign4810_e3148 + assign4810_e3153);
        let assign4810_e3155: f64 = (locals.var_tmf1 * assign4810_e3154);
        let assign4810_e3156: f64 = (assign4810_e3144 + assign4810_e3155);
        let assign4810_e3157: f64 = (locals.var_tmf1 * assign4810_e3156);
        let assign4810_e3158: f64 = (assign4810_e3140 + assign4810_e3157);
        let assign4810_e3159: f64 = (locals.var_tmf1 * assign4810_e3158);
        let assign4810_e3160: f64 = (assign4810_e3136 + assign4810_e3159);
        let assign4810_e3161: f64 = (locals.var_tmf1 * assign4810_e3160);
        let assign4810_e3162: f64 = (assign4810_e3132 + assign4810_e3161);
        let assign4810_e3163: f64 = (locals.var_tmf1 * assign4810_e3162);
        let assign4810_e3164: f64 = (1.0 + assign4810_e3163);
        locals.var_tmf2 = assign4810_e3164;
        locals.var_tmf2_dn0 = ((locals.var_tmf1_dn0 * assign4810_e3162) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign4810_e3160) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign4810_e3158) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign4810_e3156) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign4810_e3154) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign4810_e3152)))))))))));
        locals.var_tmf2_dn2 = ((locals.var_tmf1_dn2 * assign4810_e3162) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign4810_e3160) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign4810_e3158) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign4810_e3156) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign4810_e3154) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign4810_e3152)))))))))));
        locals.var_tmf2_dn4 = ((locals.var_tmf1_dn4 * assign4810_e3162) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign4810_e3160) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign4810_e3158) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign4810_e3156) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign4810_e3154) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign4810_e3152)))))))))));
        locals.var_tmf2_dn5 = ((locals.var_tmf1_dn5 * assign4810_e3162) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign4810_e3160) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign4810_e3158) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign4810_e3156) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign4810_e3154) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign4810_e3152)))))))))));
        locals.var_tmf2_dn6 = ((locals.var_tmf1_dn6 * assign4810_e3162) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign4810_e3160) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign4810_e3158) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign4810_e3156) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign4810_e3154) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign4810_e3152)))))))))));
        locals.var_tmf2_dn8 = ((locals.var_tmf1_dn8 * assign4810_e3162) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign4810_e3160) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign4810_e3158) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign4810_e3156) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign4810_e3154) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign4810_e3152)))))))))));
        locals.var_tmf2_dn10 = ((locals.var_tmf1_dn10 * assign4810_e3162) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign4810_e3160) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign4810_e3158) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign4810_e3156) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign4810_e3154) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign4810_e3152)))))))))));
        locals.var_tmf2_dn11 = ((locals.var_tmf1_dn11 * assign4810_e3162) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign4810_e3160) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign4810_e3158) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign4810_e3156) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign4810_e3154) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign4810_e3152)))))))))));
        locals.var_tmf2_dn12 = ((locals.var_tmf1_dn12 * assign4810_e3162) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign4810_e3160) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign4810_e3158) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign4810_e3156) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign4810_e3154) + (locals.var_tmf1 * (locals.var_tmf1_dn12 * assign4810_e3152)))))))))));
        locals.var_tmf2_rv = 0.0;

        let assign4820_e3167: f64 = (1.0 / 2.0);
        let assign4820_e3171: f64 = (1.0 / 3.0);
        let assign4820_e3175: f64 = (1.0 / 8.0);
        let assign4820_e3179: f64 = (1.0 / 30.0);
        let assign4820_e3183: f64 = (1.0 / 144.0);
        let assign4820_e3187: f64 = (1.0 / 840.0);
        let assign4820_e3188: f64 = (locals.var_tmf1 * assign4820_e3187);
        let assign4820_e3189: f64 = (assign4820_e3183 + assign4820_e3188);
        let assign4820_e3190: f64 = (locals.var_tmf1 * assign4820_e3189);
        let assign4820_e3191: f64 = (assign4820_e3179 + assign4820_e3190);
        let assign4820_e3192: f64 = (locals.var_tmf1 * assign4820_e3191);
        let assign4820_e3193: f64 = (assign4820_e3175 + assign4820_e3192);
        let assign4820_e3194: f64 = (locals.var_tmf1 * assign4820_e3193);
        let assign4820_e3195: f64 = (assign4820_e3171 + assign4820_e3194);
        let assign4820_e3196: f64 = (locals.var_tmf1 * assign4820_e3195);
        let assign4820_e3197: f64 = (assign4820_e3167 + assign4820_e3196);
        locals.var_tmf3 = assign4820_e3197;
        locals.var_tmf3_dn0 = ((locals.var_tmf1_dn0 * assign4820_e3195) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign4820_e3193) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign4820_e3191) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign4820_e3189) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign4820_e3187)))))))));
        locals.var_tmf3_dn2 = ((locals.var_tmf1_dn2 * assign4820_e3195) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign4820_e3193) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign4820_e3191) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign4820_e3189) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign4820_e3187)))))))));
        locals.var_tmf3_dn4 = ((locals.var_tmf1_dn4 * assign4820_e3195) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign4820_e3193) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign4820_e3191) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign4820_e3189) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign4820_e3187)))))))));
        locals.var_tmf3_dn5 = ((locals.var_tmf1_dn5 * assign4820_e3195) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign4820_e3193) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign4820_e3191) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign4820_e3189) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign4820_e3187)))))))));
        locals.var_tmf3_dn6 = ((locals.var_tmf1_dn6 * assign4820_e3195) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign4820_e3193) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign4820_e3191) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign4820_e3189) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign4820_e3187)))))))));
        locals.var_tmf3_dn8 = ((locals.var_tmf1_dn8 * assign4820_e3195) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign4820_e3193) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign4820_e3191) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign4820_e3189) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign4820_e3187)))))))));
        locals.var_tmf3_dn10 = ((locals.var_tmf1_dn10 * assign4820_e3195) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign4820_e3193) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign4820_e3191) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign4820_e3189) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign4820_e3187)))))))));
        locals.var_tmf3_dn11 = ((locals.var_tmf1_dn11 * assign4820_e3195) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign4820_e3193) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign4820_e3191) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign4820_e3189) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign4820_e3187)))))))));
        locals.var_tmf3_dn12 = ((locals.var_tmf1_dn12 * assign4820_e3195) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign4820_e3193) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign4820_e3191) + (locals.var_tmf1 * ((locals.var_tmf1_dn12 * assign4820_e3189) + (locals.var_tmf1 * (locals.var_tmf1_dn12 * assign4820_e3187)))))))));
        locals.var_tmf3_rv = 0.0;

        let assign4830_e3200: f64 = (p.p216 / locals.var_tmf2);
        locals.var_vzadd = assign4830_e3200;
        locals.var_vzadd_dn0 = (-((p.p216 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn2 = (-((p.p216 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn4 = (-((p.p216 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn5 = (-((p.p216 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn6 = (-((p.p216 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn8 = (-((p.p216 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn10 = (-((p.p216 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn11 = (-((p.p216 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn12 = (-((p.p216 * locals.var_tmf2_dn12) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_rv = 0.0;

        let assign4840_e3202: f64 = (-2.0);
        let assign4840_e3204: f64 = (assign4840_e3202 * locals.var_tmf3);
        let assign4840_e3207: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign4840_e3208: f64 = (assign4840_e3204 / assign4840_e3207);
        locals.var_t2 = assign4840_e3208;
        locals.var_t2_dn0 = ((((assign4840_e3202 * locals.var_tmf3_dn0) * assign4840_e3207) - (assign4840_e3204 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign4840_e3207 * assign4840_e3207));
        locals.var_t2_dn2 = ((((assign4840_e3202 * locals.var_tmf3_dn2) * assign4840_e3207) - (assign4840_e3204 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign4840_e3207 * assign4840_e3207));
        locals.var_t2_dn4 = ((((assign4840_e3202 * locals.var_tmf3_dn4) * assign4840_e3207) - (assign4840_e3204 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign4840_e3207 * assign4840_e3207));
        locals.var_t2_dn5 = ((((assign4840_e3202 * locals.var_tmf3_dn5) * assign4840_e3207) - (assign4840_e3204 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign4840_e3207 * assign4840_e3207));
        locals.var_t2_dn6 = ((((assign4840_e3202 * locals.var_tmf3_dn6) * assign4840_e3207) - (assign4840_e3204 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign4840_e3207 * assign4840_e3207));
        locals.var_t2_dn8 = ((((assign4840_e3202 * locals.var_tmf3_dn8) * assign4840_e3207) - (assign4840_e3204 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign4840_e3207 * assign4840_e3207));
        locals.var_t2_dn10 = ((((assign4840_e3202 * locals.var_tmf3_dn10) * assign4840_e3207) - (assign4840_e3204 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign4840_e3207 * assign4840_e3207));
        locals.var_t2_dn11 = ((((assign4840_e3202 * locals.var_tmf3_dn11) * assign4840_e3207) - (assign4840_e3204 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign4840_e3207 * assign4840_e3207));
        locals.var_t2_dn12 = ((((assign4840_e3202 * locals.var_tmf3_dn12) * assign4840_e3207) - (assign4840_e3204 * ((locals.var_tmf2_dn12 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn12)))) / (assign4840_e3207 * assign4840_e3207));
        locals.var_t2_rv = 0.0;

        let assign4850_e3211: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard50 = assign4850_e3211;
        locals.var_guard50_rv = 0.0;

        let (assign4860_e3215, assign4860_e3215_d_n0, assign4860_e3215_d_n2, assign4860_e3215_d_n4, assign4860_e3215_d_n5, assign4860_e3215_d_n6, assign4860_e3215_d_n8, assign4860_e3215_d_n10, assign4860_e3215_d_n11, assign4860_e3215_d_n12,) = {
    if (locals.var_guard50 != 0.0) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn8, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn12,)
    }
};
        locals.var_vzadd = assign4860_e3215;
        locals.var_vzadd_dn0 = assign4860_e3215_d_n0;
        locals.var_vzadd_dn2 = assign4860_e3215_d_n2;
        locals.var_vzadd_dn4 = assign4860_e3215_d_n4;
        locals.var_vzadd_dn5 = assign4860_e3215_d_n5;
        locals.var_vzadd_dn6 = assign4860_e3215_d_n6;
        locals.var_vzadd_dn8 = assign4860_e3215_d_n8;
        locals.var_vzadd_dn10 = assign4860_e3215_d_n10;
        locals.var_vzadd_dn11 = assign4860_e3215_d_n11;
        locals.var_vzadd_dn12 = assign4860_e3215_d_n12;
        locals.var_vzadd_rv = 0.0;

        let assign4870_e3218: f64 = (locals.var_vbs + locals.var_vzadd);
        locals.var_vbsz = assign4870_e3218;
        locals.var_vbsz_dn0 = (locals.var_vbs_dn0 + locals.var_vzadd_dn0);
        locals.var_vbsz_dn2 = (locals.var_vbs_dn2 + locals.var_vzadd_dn2);
        locals.var_vbsz_dn4 = (locals.var_vbs_dn4 + locals.var_vzadd_dn4);
        locals.var_vbsz_dn5 = (locals.var_vbs_dn5 + locals.var_vzadd_dn5);
        locals.var_vbsz_dn6 = (locals.var_vbs_dn6 + locals.var_vzadd_dn6);
        locals.var_vbsz_dn8 = (locals.var_vbs_dn8 + locals.var_vzadd_dn8);
        locals.var_vbsz_dn10 = (locals.var_vbs_dn10 + locals.var_vzadd_dn10);
        locals.var_vbsz_dn11 = (locals.var_vbs_dn11 + locals.var_vzadd_dn11);
        locals.var_vbsz_dn12 = (locals.var_vbs_dn12 + locals.var_vzadd_dn12);
        locals.var_vbsz_rv = 0.0;

        let assign4880_e3222: f64 = (2.0 * locals.var_vzadd);
        let assign4880_e3223: f64 = (locals.var_vds + assign4880_e3222);
        locals.var_vdsz = assign4880_e3223;
        locals.var_vdsz_dn0 = (locals.var_vds_dn0 + (2.0 * locals.var_vzadd_dn0));
        locals.var_vdsz_dn2 = (locals.var_vds_dn2 + (2.0 * locals.var_vzadd_dn2));
        locals.var_vdsz_dn4 = (locals.var_vds_dn4 + (2.0 * locals.var_vzadd_dn4));
        locals.var_vdsz_dn5 = (locals.var_vds_dn5 + (2.0 * locals.var_vzadd_dn5));
        locals.var_vdsz_dn6 = (locals.var_vds_dn6 + (2.0 * locals.var_vzadd_dn6));
        locals.var_vdsz_dn8 = (locals.var_vds_dn8 + (2.0 * locals.var_vzadd_dn8));
        locals.var_vdsz_dn10 = (locals.var_vds_dn10 + (2.0 * locals.var_vzadd_dn10));
        locals.var_vdsz_dn11 = (locals.var_vds_dn11 + (2.0 * locals.var_vzadd_dn11));
        locals.var_vdsz_dn12 = (locals.var_vds_dn12 + (2.0 * locals.var_vzadd_dn12));
        locals.var_vdsz_rv = 0.0;

        let assign4890_e3226: f64 = (locals.var_vgs + locals.var_vzadd);
        locals.var_vgsz = assign4890_e3226;
        locals.var_vgsz_dn0 = locals.var_vzadd_dn0;
        locals.var_vgsz_dn2 = locals.var_vzadd_dn2;
        locals.var_vgsz_dn4 = locals.var_vzadd_dn4;
        locals.var_vgsz_dn5 = (locals.var_vgs_dn5 + locals.var_vzadd_dn5);
        locals.var_vgsz_dn6 = locals.var_vzadd_dn6;
        locals.var_vgsz_dn8 = locals.var_vzadd_dn8;
        locals.var_vgsz_dn10 = locals.var_vzadd_dn10;
        locals.var_vgsz_dn11 = (locals.var_vgs_dn11 + locals.var_vzadd_dn11);
        locals.var_vgsz_dn12 = (locals.var_vgs_dn12 + locals.var_vzadd_dn12);
        locals.var_vgsz_rv = 0.0;

        let assign4900_e3229: f64 = (2.0 * locals.var_q_nsub);
        let assign4900_e3231: f64 = (assign4900_e3229 * 1.034943e-10);
        let assign4900_e3233: f64 = (assign4900_e3231 * locals.var_c_fox0_inv);
        let assign4900_e3235: f64 = (assign4900_e3233 * locals.var_c_fox0_inv);
        locals.var_t1 = assign4900_e3235;
        locals.var_t1_dn0 = ((((2.0 * locals.var_q_nsub_dn0) * 1.034943e-10) * locals.var_c_fox0_inv) * locals.var_c_fox0_inv);
        locals.var_t1_dn2 = ((((2.0 * locals.var_q_nsub_dn2) * 1.034943e-10) * locals.var_c_fox0_inv) * locals.var_c_fox0_inv);
        locals.var_t1_dn4 = ((((2.0 * locals.var_q_nsub_dn4) * 1.034943e-10) * locals.var_c_fox0_inv) * locals.var_c_fox0_inv);
        locals.var_t1_dn5 = ((((2.0 * locals.var_q_nsub_dn5) * 1.034943e-10) * locals.var_c_fox0_inv) * locals.var_c_fox0_inv);
        locals.var_t1_dn6 = ((((2.0 * locals.var_q_nsub_dn6) * 1.034943e-10) * locals.var_c_fox0_inv) * locals.var_c_fox0_inv);
        locals.var_t1_dn8 = ((((2.0 * locals.var_q_nsub_dn8) * 1.034943e-10) * locals.var_c_fox0_inv) * locals.var_c_fox0_inv);
        locals.var_t1_dn10 = ((((2.0 * locals.var_q_nsub_dn10) * 1.034943e-10) * locals.var_c_fox0_inv) * locals.var_c_fox0_inv);
        locals.var_t1_dn11 = ((((2.0 * locals.var_q_nsub_dn11) * 1.034943e-10) * locals.var_c_fox0_inv) * locals.var_c_fox0_inv);
        locals.var_t1_dn12 = ((((2.0 * locals.var_q_nsub_dn12) * 1.034943e-10) * locals.var_c_fox0_inv) * locals.var_c_fox0_inv);
        locals.var_t1_rv = 0.0;

        let assign4910_e3238: f64 = (locals.var_vgs - locals.var_vfb);
        locals.var_t2 = assign4910_e3238;
        locals.var_t2_dn0 = (-locals.var_vfb_dn0);
        locals.var_t2_dn2 = (-locals.var_vfb_dn2);
        locals.var_t2_dn4 = (-locals.var_vfb_dn4);
        locals.var_t2_dn5 = (locals.var_vgs_dn5 - locals.var_vfb_dn5);
        locals.var_t2_dn6 = (-locals.var_vfb_dn6);
        locals.var_t2_dn8 = (-locals.var_vfb_dn8);
        locals.var_t2_dn10 = (-locals.var_vfb_dn10);
        locals.var_t2_dn11 = (locals.var_vgs_dn11 - locals.var_vfb_dn11);
        locals.var_t2_dn12 = (locals.var_vgs_dn12 - locals.var_vfb_dn12);
        locals.var_t2_rv = 0.0;

        let assign4920_e3242: f64 = (2.0 / locals.var_t1);
        let assign4920_e3245: f64 = (locals.var_t2 - locals.var_beta_inv);
        let assign4920_e3247: f64 = (assign4920_e3245 - locals.var_vbs);
        let assign4920_e3248: f64 = (assign4920_e3242 * assign4920_e3247);
        let assign4920_e3249: f64 = (1.0 + assign4920_e3248);
        locals.var_t3 = assign4920_e3249;
        locals.var_t3_dn0 = (((-((2.0 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))) * assign4920_e3247) + (assign4920_e3242 * (locals.var_t2_dn0 - locals.var_vbs_dn0)));
        locals.var_t3_dn2 = (((-((2.0 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))) * assign4920_e3247) + (assign4920_e3242 * (locals.var_t2_dn2 - locals.var_vbs_dn2)));
        locals.var_t3_dn4 = (((-((2.0 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))) * assign4920_e3247) + (assign4920_e3242 * ((locals.var_t2_dn4 - locals.var_beta_inv_dn4) - locals.var_vbs_dn4)));
        locals.var_t3_dn5 = (((-((2.0 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))) * assign4920_e3247) + (assign4920_e3242 * (locals.var_t2_dn5 - locals.var_vbs_dn5)));
        locals.var_t3_dn6 = (((-((2.0 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))) * assign4920_e3247) + (assign4920_e3242 * (locals.var_t2_dn6 - locals.var_vbs_dn6)));
        locals.var_t3_dn8 = (((-((2.0 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))) * assign4920_e3247) + (assign4920_e3242 * (locals.var_t2_dn8 - locals.var_vbs_dn8)));
        locals.var_t3_dn10 = (((-((2.0 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))) * assign4920_e3247) + (assign4920_e3242 * (locals.var_t2_dn10 - locals.var_vbs_dn10)));
        locals.var_t3_dn11 = (((-((2.0 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))) * assign4920_e3247) + (assign4920_e3242 * (locals.var_t2_dn11 - locals.var_vbs_dn11)));
        locals.var_t3_dn12 = (((-((2.0 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))) * assign4920_e3247) + (assign4920_e3242 * (locals.var_t2_dn12 - locals.var_vbs_dn12)));
        locals.var_t3_rv = 0.0;

        let assign4930_e3252: f64 = (locals.var_t3 * locals.var_t3);
        let assign4930_e3255: f64 = (4.0 * 0.001);
        let assign4930_e3257: f64 = (assign4930_e3255 * 0.001);
        let assign4930_e3258: f64 = (assign4930_e3252 + assign4930_e3257);
        let assign4930_e3259: f64 = (assign4930_e3258).sqrt();
        locals.var_tmf2 = assign4930_e3259;
        locals.var_tmf2_dn0 = (((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)) / (2.0 * assign4930_e3259));
        locals.var_tmf2_dn2 = (((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)) / (2.0 * assign4930_e3259));
        locals.var_tmf2_dn4 = (((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)) / (2.0 * assign4930_e3259));
        locals.var_tmf2_dn5 = (((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)) / (2.0 * assign4930_e3259));
        locals.var_tmf2_dn6 = (((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)) / (2.0 * assign4930_e3259));
        locals.var_tmf2_dn8 = (((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)) / (2.0 * assign4930_e3259));
        locals.var_tmf2_dn10 = (((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)) / (2.0 * assign4930_e3259));
        locals.var_tmf2_dn11 = (((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11)) / (2.0 * assign4930_e3259));
        locals.var_tmf2_dn12 = (((locals.var_t3_dn12 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn12)) / (2.0 * assign4930_e3259));
        locals.var_tmf2_rv = 0.0;

        let assign4940_e3264: f64 = (locals.var_t3 / locals.var_tmf2);
        let assign4940_e3265: f64 = (1.0 + assign4940_e3264);
        let assign4940_e3266: f64 = (0.5 * assign4940_e3265);
        locals.var_t5 = assign4940_e3266;
        locals.var_t5_dn0 = (0.5 * (((locals.var_t3_dn0 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn2 = (0.5 * (((locals.var_t3_dn2 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn4 = (0.5 * (((locals.var_t3_dn4 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn5 = (0.5 * (((locals.var_t3_dn5 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn6 = (0.5 * (((locals.var_t3_dn6 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn8 = (0.5 * (((locals.var_t3_dn8 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn10 = (0.5 * (((locals.var_t3_dn10 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn11 = (0.5 * (((locals.var_t3_dn11 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn12 = (0.5 * (((locals.var_t3_dn12 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_rv = 0.0;

        let assign4950_e3270: f64 = (locals.var_t3 + locals.var_tmf2);
        let assign4950_e3271: f64 = (0.5 * assign4950_e3270);
        let assign4950_e3274: f64 = (1e-10 * 0.001);
        let assign4950_e3275: f64 = (assign4950_e3271 + assign4950_e3274);
        locals.var_t4 = assign4950_e3275;
        locals.var_t4_dn0 = (0.5 * (locals.var_t3_dn0 + locals.var_tmf2_dn0));
        locals.var_t4_dn2 = (0.5 * (locals.var_t3_dn2 + locals.var_tmf2_dn2));
        locals.var_t4_dn4 = (0.5 * (locals.var_t3_dn4 + locals.var_tmf2_dn4));
        locals.var_t4_dn5 = (0.5 * (locals.var_t3_dn5 + locals.var_tmf2_dn5));
        locals.var_t4_dn6 = (0.5 * (locals.var_t3_dn6 + locals.var_tmf2_dn6));
        locals.var_t4_dn8 = (0.5 * (locals.var_t3_dn8 + locals.var_tmf2_dn8));
        locals.var_t4_dn10 = (0.5 * (locals.var_t3_dn10 + locals.var_tmf2_dn10));
        locals.var_t4_dn11 = (0.5 * (locals.var_t3_dn11 + locals.var_tmf2_dn11));
        locals.var_t4_dn12 = (0.5 * (locals.var_t3_dn12 + locals.var_tmf2_dn12));
        locals.var_t4_rv = 0.0;

        let assign4960_e3278: f64 = if locals.var_t4 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard51 = assign4960_e3278;
        locals.var_guard51_rv = 0.0;

        let (assign4970_e3282, assign4970_e3282_d_n0, assign4970_e3282_d_n2, assign4970_e3282_d_n4, assign4970_e3282_d_n5, assign4970_e3282_d_n6, assign4970_e3282_d_n8, assign4970_e3282_d_n10, assign4970_e3282_d_n11, assign4970_e3282_d_n12,) = {
    if (locals.var_guard51 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign4970_e3282;
        locals.var_t4_dn0 = assign4970_e3282_d_n0;
        locals.var_t4_dn2 = assign4970_e3282_d_n2;
        locals.var_t4_dn4 = assign4970_e3282_d_n4;
        locals.var_t4_dn5 = assign4970_e3282_d_n5;
        locals.var_t4_dn6 = assign4970_e3282_d_n6;
        locals.var_t4_dn8 = assign4970_e3282_d_n8;
        locals.var_t4_dn10 = assign4970_e3282_d_n10;
        locals.var_t4_dn11 = assign4970_e3282_d_n11;
        locals.var_t4_dn12 = assign4970_e3282_d_n12;
        locals.var_t4_rv = 0.0;

        let (assign4980_e3286, assign4980_e3286_d_n0, assign4980_e3286_d_n2, assign4980_e3286_d_n4, assign4980_e3286_d_n5, assign4980_e3286_d_n6, assign4980_e3286_d_n8, assign4980_e3286_d_n10, assign4980_e3286_d_n11, assign4980_e3286_d_n12,) = {
    if (locals.var_guard51 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign4980_e3286;
        locals.var_t5_dn0 = assign4980_e3286_d_n0;
        locals.var_t5_dn2 = assign4980_e3286_d_n2;
        locals.var_t5_dn4 = assign4980_e3286_d_n4;
        locals.var_t5_dn5 = assign4980_e3286_d_n5;
        locals.var_t5_dn6 = assign4980_e3286_d_n6;
        locals.var_t5_dn8 = assign4980_e3286_d_n8;
        locals.var_t5_dn10 = assign4980_e3286_d_n10;
        locals.var_t5_dn11 = assign4980_e3286_d_n11;
        locals.var_t5_dn12 = assign4980_e3286_d_n12;
        locals.var_t5_rv = 0.0;

        let assign4990_e3289: f64 = (locals.var_t4 + 1e-50);
        let assign4990_e3290: f64 = (assign4990_e3289).sqrt();
        locals.var_tx = assign4990_e3290;
        locals.var_tx_dn0 = (locals.var_t4_dn0 / (2.0 * assign4990_e3290));
        locals.var_tx_dn2 = (locals.var_t4_dn2 / (2.0 * assign4990_e3290));
        locals.var_tx_dn4 = (locals.var_t4_dn4 / (2.0 * assign4990_e3290));
        locals.var_tx_dn5 = (locals.var_t4_dn5 / (2.0 * assign4990_e3290));
        locals.var_tx_dn6 = (locals.var_t4_dn6 / (2.0 * assign4990_e3290));
        locals.var_tx_dn8 = (locals.var_t4_dn8 / (2.0 * assign4990_e3290));
        locals.var_tx_dn10 = (locals.var_t4_dn10 / (2.0 * assign4990_e3290));
        locals.var_tx_dn11 = (locals.var_t4_dn11 / (2.0 * assign4990_e3290));
        locals.var_tx_dn12 = (locals.var_t4_dn12 / (2.0 * assign4990_e3290));
        locals.var_tx_rv = 0.0;

        let assign5000_e3295: f64 = (1.0 - locals.var_tx);
        let assign5000_e3296: f64 = (locals.var_t1 * assign5000_e3295);
        let assign5000_e3297: f64 = (locals.var_t2 + assign5000_e3296);
        locals.var_pslsat = assign5000_e3297;
        locals.var_pslsat_dn0 = (locals.var_t2_dn0 + ((locals.var_t1_dn0 * assign5000_e3295) + (locals.var_t1 * (-locals.var_tx_dn0))));
        locals.var_pslsat_dn2 = (locals.var_t2_dn2 + ((locals.var_t1_dn2 * assign5000_e3295) + (locals.var_t1 * (-locals.var_tx_dn2))));
        locals.var_pslsat_dn4 = (locals.var_t2_dn4 + ((locals.var_t1_dn4 * assign5000_e3295) + (locals.var_t1 * (-locals.var_tx_dn4))));
        locals.var_pslsat_dn5 = (locals.var_t2_dn5 + ((locals.var_t1_dn5 * assign5000_e3295) + (locals.var_t1 * (-locals.var_tx_dn5))));
        locals.var_pslsat_dn6 = (locals.var_t2_dn6 + ((locals.var_t1_dn6 * assign5000_e3295) + (locals.var_t1 * (-locals.var_tx_dn6))));
        locals.var_pslsat_dn8 = (locals.var_t2_dn8 + ((locals.var_t1_dn8 * assign5000_e3295) + (locals.var_t1 * (-locals.var_tx_dn8))));
        locals.var_pslsat_dn10 = (locals.var_t2_dn10 + ((locals.var_t1_dn10 * assign5000_e3295) + (locals.var_t1 * (-locals.var_tx_dn10))));
        locals.var_pslsat_dn11 = (locals.var_t2_dn11 + ((locals.var_t1_dn11 * assign5000_e3295) + (locals.var_t1 * (-locals.var_tx_dn11))));
        locals.var_pslsat_dn12 = (locals.var_t2_dn12 + ((locals.var_t1_dn12 * assign5000_e3295) + (locals.var_t1 * (-locals.var_tx_dn12))));
        locals.var_pslsat_rv = 0.0;

        let assign5010_e3300: f64 = (locals.var_pslsat - locals.var_pb2);
        locals.var_vdsats = assign5010_e3300;
        locals.var_vdsats_dn0 = (locals.var_pslsat_dn0 - locals.var_pb2_dn0);
        locals.var_vdsats_dn2 = (locals.var_pslsat_dn2 - locals.var_pb2_dn2);
        locals.var_vdsats_dn4 = (locals.var_pslsat_dn4 - locals.var_pb2_dn4);
        locals.var_vdsats_dn5 = (locals.var_pslsat_dn5 - locals.var_pb2_dn5);
        locals.var_vdsats_dn6 = (locals.var_pslsat_dn6 - locals.var_pb2_dn6);
        locals.var_vdsats_dn8 = (locals.var_pslsat_dn8 - locals.var_pb2_dn8);
        locals.var_vdsats_dn10 = (locals.var_pslsat_dn10 - locals.var_pb2_dn10);
        locals.var_vdsats_dn11 = (locals.var_pslsat_dn11 - locals.var_pb2_dn11);
        locals.var_vdsats_dn12 = (locals.var_pslsat_dn12 - locals.var_pb2_dn12);
        locals.var_vdsats_rv = 0.0;

        let assign5020_e3303: f64 = (locals.var_vdsats - 0.1);
        let assign5020_e3305: f64 = (assign5020_e3303 - 0.05);
        locals.var_tmf1 = assign5020_e3305;
        locals.var_tmf1_dn0 = locals.var_vdsats_dn0;
        locals.var_tmf1_dn2 = locals.var_vdsats_dn2;
        locals.var_tmf1_dn4 = locals.var_vdsats_dn4;
        locals.var_tmf1_dn5 = locals.var_vdsats_dn5;
        locals.var_tmf1_dn6 = locals.var_vdsats_dn6;
        locals.var_tmf1_dn8 = locals.var_vdsats_dn8;
        locals.var_tmf1_dn10 = locals.var_vdsats_dn10;
        locals.var_tmf1_dn11 = locals.var_vdsats_dn11;
        locals.var_tmf1_dn12 = locals.var_vdsats_dn12;
        locals.var_tmf1_rv = 0.0;

        let assign5030_e3308: f64 = (4.0 * 0.1);
        let assign5030_e3310: f64 = (assign5030_e3308 * 0.05);
        locals.var_tmf2 = assign5030_e3310;
        locals.var_tmf2_dn0 = 0.0;
        locals.var_tmf2_dn2 = 0.0;
        locals.var_tmf2_dn4 = 0.0;
        locals.var_tmf2_dn5 = 0.0;
        locals.var_tmf2_dn6 = 0.0;
        locals.var_tmf2_dn8 = 0.0;
        locals.var_tmf2_dn10 = 0.0;
        locals.var_tmf2_dn11 = 0.0;
        locals.var_tmf2_dn12 = 0.0;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5040_e3317, assign5040_e3317_d_n0, assign5040_e3317_d_n2, assign5040_e3317_d_n4, assign5040_e3317_d_n5, assign5040_e3317_d_n6, assign5040_e3317_d_n8, assign5040_e3317_d_n10, assign5040_e3317_d_n11, assign5040_e3317_d_n12,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    } else {
        let assign5040_e3316: f64 = (-locals.var_tmf2);
        (assign5040_e3316, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12),)
    }
};
        locals.var_tmf2 = assign5040_e3317;
        locals.var_tmf2_dn0 = assign5040_e3317_d_n0;
        locals.var_tmf2_dn2 = assign5040_e3317_d_n2;
        locals.var_tmf2_dn4 = assign5040_e3317_d_n4;
        locals.var_tmf2_dn5 = assign5040_e3317_d_n5;
        locals.var_tmf2_dn6 = assign5040_e3317_d_n6;
        locals.var_tmf2_dn8 = assign5040_e3317_d_n8;
        locals.var_tmf2_dn10 = assign5040_e3317_d_n10;
        locals.var_tmf2_dn11 = assign5040_e3317_d_n11;
        locals.var_tmf2_dn12 = assign5040_e3317_d_n12;
        locals.var_tmf2_rv = 0.0;

        let assign5050_e3320: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign5050_e3322: f64 = (assign5050_e3320 + locals.var_tmf2);
        let assign5050_e3323: f64 = (assign5050_e3322).sqrt();
        locals.var_tmf2 = assign5050_e3323;
        locals.var_tmf2_dn0 = ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign5050_e3323));
        locals.var_tmf2_dn2 = ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign5050_e3323));
        locals.var_tmf2_dn4 = ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign5050_e3323));
        locals.var_tmf2_dn5 = ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign5050_e3323));
        locals.var_tmf2_dn6 = ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign5050_e3323));
        locals.var_tmf2_dn8 = ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign5050_e3323));
        locals.var_tmf2_dn10 = ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign5050_e3323));
        locals.var_tmf2_dn11 = ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign5050_e3323));
        locals.var_tmf2_dn12 = ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign5050_e3323));
        locals.var_tmf2_rv = 0.0;

        let assign5060_e3328: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign5060_e3329: f64 = (1.0 + assign5060_e3328);
        let assign5060_e3330: f64 = (0.5 * assign5060_e3329);
        locals.var_t6 = assign5060_e3330;
        locals.var_t6_dn0 = (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn2 = (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn4 = (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn5 = (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn6 = (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn8 = (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn10 = (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn11 = (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn12 = (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_rv = 0.0;

        let assign5070_e3335: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign5070_e3336: f64 = (0.5 * assign5070_e3335);
        let assign5070_e3337: f64 = (0.1 + assign5070_e3336);
        locals.var_vdsats = assign5070_e3337;
        locals.var_vdsats_dn0 = (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0));
        locals.var_vdsats_dn2 = (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2));
        locals.var_vdsats_dn4 = (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4));
        locals.var_vdsats_dn5 = (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5));
        locals.var_vdsats_dn6 = (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6));
        locals.var_vdsats_dn8 = (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8));
        locals.var_vdsats_dn10 = (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10));
        locals.var_vdsats_dn11 = (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11));
        locals.var_vdsats_dn12 = (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12));
        locals.var_vdsats_rv = 0.0;

        let assign5080_e3340: f64 = (locals.var_vds / locals.var_vdsats);
        locals.var_t1 = assign5080_e3340;
        locals.var_t1_dn0 = (((locals.var_vds_dn0 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn0)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn2 = (((locals.var_vds_dn2 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn2)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn4 = (((locals.var_vds_dn4 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn4)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn5 = (((locals.var_vds_dn5 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn5)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn6 = (((locals.var_vds_dn6 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn6)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn8 = (((locals.var_vds_dn8 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn8)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn10 = (((locals.var_vds_dn10 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn10)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn11 = (((locals.var_vds_dn11 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn11)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn12 = (((locals.var_vds_dn12 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn12)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_rv = 0.0;

        let assign5090_e3343: f64 = locals.var_t1;
        locals.var_tmf1 = assign5090_e3343;
        locals.var_tmf1_dn0 = locals.var_t1_dn0;
        locals.var_tmf1_dn2 = locals.var_t1_dn2;
        locals.var_tmf1_dn4 = locals.var_t1_dn4;
        locals.var_tmf1_dn5 = locals.var_t1_dn5;
        locals.var_tmf1_dn6 = locals.var_t1_dn6;
        locals.var_tmf1_dn8 = locals.var_t1_dn8;
        locals.var_tmf1_dn10 = locals.var_t1_dn10;
        locals.var_tmf1_dn11 = locals.var_t1_dn11;
        locals.var_tmf1_dn12 = locals.var_t1_dn12;
        locals.var_tmf1_rv = 0.0;

        let assign5100_e3346: f64 = (locals.var_tmf1 * locals.var_tmf1);
        locals.var_tmf2 = assign5100_e3346;
        locals.var_tmf2_dn0 = ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0));
        locals.var_tmf2_dn2 = ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2));
        locals.var_tmf2_dn4 = ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4));
        locals.var_tmf2_dn5 = ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5));
        locals.var_tmf2_dn6 = ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6));
        locals.var_tmf2_dn8 = ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8));
        locals.var_tmf2_dn10 = ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10));
        locals.var_tmf2_dn11 = ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11));
        locals.var_tmf2_dn12 = ((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12));
        locals.var_tmf2_rv = 0.0;

        let assign5110_e3349: f64 = (locals.var_tmf2 * locals.var_tmf1);
        locals.var_tmf3 = assign5110_e3349;
        locals.var_tmf3_dn0 = ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0));
        locals.var_tmf3_dn2 = ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2));
        locals.var_tmf3_dn4 = ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4));
        locals.var_tmf3_dn5 = ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5));
        locals.var_tmf3_dn6 = ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6));
        locals.var_tmf3_dn8 = ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8));
        locals.var_tmf3_dn10 = ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10));
        locals.var_tmf3_dn11 = ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11));
        locals.var_tmf3_dn12 = ((locals.var_tmf2_dn12 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn12));
        locals.var_tmf3_rv = 0.0;

        let assign5120_e3352: f64 = (locals.var_tmf2 * locals.var_tmf2);
        locals.var_tmf4 = assign5120_e3352;
        locals.var_tmf4_dn0 = ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0));
        locals.var_tmf4_dn2 = ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2));
        locals.var_tmf4_dn4 = ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4));
        locals.var_tmf4_dn5 = ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5));
        locals.var_tmf4_dn6 = ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6));
        locals.var_tmf4_dn8 = ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8));
        locals.var_tmf4_dn10 = ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10));
        locals.var_tmf4_dn11 = ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11));
        locals.var_tmf4_dn12 = ((locals.var_tmf2_dn12 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn12));
        locals.var_tmf4_rv = 0.0;

        let assign5130_e3356: f64 = (1.0 + locals.var_tmf1);
        let assign5130_e3358: f64 = (assign5130_e3356 + locals.var_tmf2);
        let assign5130_e3360: f64 = (assign5130_e3358 + locals.var_tmf3);
        let assign5130_e3362: f64 = (assign5130_e3360 + locals.var_tmf4);
        let assign5130_e3363: f64 = (1.0 / assign5130_e3362);
        locals.var_tx = assign5130_e3363;
        locals.var_tx_dn0 = (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign5130_e3362 * assign5130_e3362)));
        locals.var_tx_dn2 = (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign5130_e3362 * assign5130_e3362)));
        locals.var_tx_dn4 = (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign5130_e3362 * assign5130_e3362)));
        locals.var_tx_dn5 = (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign5130_e3362 * assign5130_e3362)));
        locals.var_tx_dn6 = (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign5130_e3362 * assign5130_e3362)));
        locals.var_tx_dn8 = (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign5130_e3362 * assign5130_e3362)));
        locals.var_tx_dn10 = (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign5130_e3362 * assign5130_e3362)));
        locals.var_tx_dn11 = (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign5130_e3362 * assign5130_e3362)));
        locals.var_tx_dn12 = (-((((locals.var_tmf1_dn12 + locals.var_tmf2_dn12) + locals.var_tmf3_dn12) + locals.var_tmf4_dn12) / (assign5130_e3362 * assign5130_e3362)));
        locals.var_tx_rv = 0.0;

        let assign5140_e3367: f64 = (2.0 * locals.var_tmf1);
        let assign5140_e3368: f64 = (1.0 + assign5140_e3367);
        let assign5140_e3371: f64 = (3.0 * locals.var_tmf2);
        let assign5140_e3372: f64 = (assign5140_e3368 + assign5140_e3371);
        let assign5140_e3375: f64 = (4.0 * locals.var_tmf3);
        let assign5140_e3376: f64 = (assign5140_e3372 + assign5140_e3375);
        let assign5140_e3377: f64 = (-assign5140_e3376);
        let assign5140_e3379: f64 = (assign5140_e3377 * locals.var_tx);
        let assign5140_e3381: f64 = (assign5140_e3379 * locals.var_tx);
        locals.var_t0 = assign5140_e3381;
        locals.var_t0_dn0 = (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tx) + (assign5140_e3377 * locals.var_tx_dn0)) * locals.var_tx) + (assign5140_e3379 * locals.var_tx_dn0));
        locals.var_t0_dn2 = (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tx) + (assign5140_e3377 * locals.var_tx_dn2)) * locals.var_tx) + (assign5140_e3379 * locals.var_tx_dn2));
        locals.var_t0_dn4 = (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tx) + (assign5140_e3377 * locals.var_tx_dn4)) * locals.var_tx) + (assign5140_e3379 * locals.var_tx_dn4));
        locals.var_t0_dn5 = (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tx) + (assign5140_e3377 * locals.var_tx_dn5)) * locals.var_tx) + (assign5140_e3379 * locals.var_tx_dn5));
        locals.var_t0_dn6 = (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tx) + (assign5140_e3377 * locals.var_tx_dn6)) * locals.var_tx) + (assign5140_e3379 * locals.var_tx_dn6));
        locals.var_t0_dn8 = (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tx) + (assign5140_e3377 * locals.var_tx_dn8)) * locals.var_tx) + (assign5140_e3379 * locals.var_tx_dn8));
        locals.var_t0_dn10 = (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tx) + (assign5140_e3377 * locals.var_tx_dn10)) * locals.var_tx) + (assign5140_e3379 * locals.var_tx_dn10));
        locals.var_t0_dn11 = (((((-(((2.0 * locals.var_tmf1_dn11) + (3.0 * locals.var_tmf2_dn11)) + (4.0 * locals.var_tmf3_dn11))) * locals.var_tx) + (assign5140_e3377 * locals.var_tx_dn11)) * locals.var_tx) + (assign5140_e3379 * locals.var_tx_dn11));
        locals.var_t0_dn12 = (((((-(((2.0 * locals.var_tmf1_dn12) + (3.0 * locals.var_tmf2_dn12)) + (4.0 * locals.var_tmf3_dn12))) * locals.var_tx) + (assign5140_e3377 * locals.var_tx_dn12)) * locals.var_tx) + (assign5140_e3379 * locals.var_tx_dn12));
        locals.var_t0_rv = 0.0;

        let assign5150_e3385: f64 = (1.0 - locals.var_tx);
        let assign5150_e3386: f64 = assign5150_e3385;
        locals.var_tx = assign5150_e3386;
        locals.var_tx_dn0 = (-locals.var_tx_dn0);
        locals.var_tx_dn2 = (-locals.var_tx_dn2);
        locals.var_tx_dn4 = (-locals.var_tx_dn4);
        locals.var_tx_dn5 = (-locals.var_tx_dn5);
        locals.var_tx_dn6 = (-locals.var_tx_dn6);
        locals.var_tx_dn8 = (-locals.var_tx_dn8);
        locals.var_tx_dn10 = (-locals.var_tx_dn10);
        locals.var_tx_dn11 = (-locals.var_tx_dn11);
        locals.var_tx_dn12 = (-locals.var_tx_dn12);
        locals.var_tx_rv = 0.0;

        let assign5160_e3388: f64 = (-locals.var_t0);
        locals.var_t0 = assign5160_e3388;
        locals.var_t0_dn0 = (-locals.var_t0_dn0);
        locals.var_t0_dn2 = (-locals.var_t0_dn2);
        locals.var_t0_dn4 = (-locals.var_t0_dn4);
        locals.var_t0_dn5 = (-locals.var_t0_dn5);
        locals.var_t0_dn6 = (-locals.var_t0_dn6);
        locals.var_t0_dn8 = (-locals.var_t0_dn8);
        locals.var_t0_dn10 = (-locals.var_t0_dn10);
        locals.var_t0_dn11 = (-locals.var_t0_dn11);
        locals.var_t0_dn12 = (-locals.var_t0_dn12);
        locals.var_t0_rv = 0.0;

        let assign5170_e3391: f64 = (locals.var_tx * locals.var_tx);
        locals.var_fmdvds = assign5170_e3391;
        locals.var_fmdvds_dn0 = ((locals.var_tx_dn0 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn0));
        locals.var_fmdvds_dn2 = ((locals.var_tx_dn2 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn2));
        locals.var_fmdvds_dn4 = ((locals.var_tx_dn4 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn4));
        locals.var_fmdvds_dn5 = ((locals.var_tx_dn5 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn5));
        locals.var_fmdvds_dn6 = ((locals.var_tx_dn6 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn6));
        locals.var_fmdvds_dn8 = ((locals.var_tx_dn8 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn8));
        locals.var_fmdvds_dn10 = ((locals.var_tx_dn10 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn10));
        locals.var_fmdvds_dn11 = ((locals.var_tx_dn11 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn11));
        locals.var_fmdvds_dn12 = ((locals.var_tx_dn12 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn12));
        locals.var_fmdvds_rv = 0.0;

        let assign5180_e3402: f64 = if (((p.p193 == 0.0) && (p.p195 == 0.0)) || (p.p194 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard52 = assign5180_e3402;
        locals.var_guard52_rv = 0.0;

        let (assign5190_e3406,) = {
    if (locals.var_guard52 != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_qme,)
    }
};
        locals.var_flg_qme = assign5190_e3406;
        locals.var_flg_qme_rv = 0.0;

        let (assign5200_e3411,) = {
    if (locals.var_guard52 == 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_qme,)
    }
};
        locals.var_flg_qme = assign5200_e3411;
        locals.var_flg_qme_rv = 0.0;

        let assign5210_e3414: f64 = (locals.var_pb20 + locals.var_vfb);
        let assign5210_e3417: f64 = (2.0 * locals.var_q_nsub);
        let assign5210_e3419: f64 = (assign5210_e3417 * 1.034943e-10);
        let assign5210_e3421: f64 = (assign5210_e3419 * locals.var_pb20);
        let assign5210_e3422: f64 = (assign5210_e3421).sqrt();
        let assign5210_e3424: f64 = (assign5210_e3422 / locals.var_c_fox0);
        let assign5210_e3425: f64 = (assign5210_e3414 + assign5210_e3424);
        locals.var_vthq = assign5210_e3425;
        locals.var_vthq_dn0 = ((locals.var_pb20_dn0 + locals.var_vfb_dn0) + ((((((2.0 * locals.var_q_nsub_dn0) * 1.034943e-10) * locals.var_pb20) + (assign5210_e3419 * locals.var_pb20_dn0)) / (2.0 * assign5210_e3422)) / locals.var_c_fox0));
        locals.var_vthq_dn2 = ((locals.var_pb20_dn2 + locals.var_vfb_dn2) + ((((((2.0 * locals.var_q_nsub_dn2) * 1.034943e-10) * locals.var_pb20) + (assign5210_e3419 * locals.var_pb20_dn2)) / (2.0 * assign5210_e3422)) / locals.var_c_fox0));
        locals.var_vthq_dn4 = ((locals.var_pb20_dn4 + locals.var_vfb_dn4) + ((((((2.0 * locals.var_q_nsub_dn4) * 1.034943e-10) * locals.var_pb20) + (assign5210_e3419 * locals.var_pb20_dn4)) / (2.0 * assign5210_e3422)) / locals.var_c_fox0));
        locals.var_vthq_dn5 = ((locals.var_pb20_dn5 + locals.var_vfb_dn5) + ((((((2.0 * locals.var_q_nsub_dn5) * 1.034943e-10) * locals.var_pb20) + (assign5210_e3419 * locals.var_pb20_dn5)) / (2.0 * assign5210_e3422)) / locals.var_c_fox0));
        locals.var_vthq_dn6 = ((locals.var_pb20_dn6 + locals.var_vfb_dn6) + ((((((2.0 * locals.var_q_nsub_dn6) * 1.034943e-10) * locals.var_pb20) + (assign5210_e3419 * locals.var_pb20_dn6)) / (2.0 * assign5210_e3422)) / locals.var_c_fox0));
        locals.var_vthq_dn8 = ((locals.var_pb20_dn8 + locals.var_vfb_dn8) + ((((((2.0 * locals.var_q_nsub_dn8) * 1.034943e-10) * locals.var_pb20) + (assign5210_e3419 * locals.var_pb20_dn8)) / (2.0 * assign5210_e3422)) / locals.var_c_fox0));
        locals.var_vthq_dn10 = ((locals.var_pb20_dn10 + locals.var_vfb_dn10) + ((((((2.0 * locals.var_q_nsub_dn10) * 1.034943e-10) * locals.var_pb20) + (assign5210_e3419 * locals.var_pb20_dn10)) / (2.0 * assign5210_e3422)) / locals.var_c_fox0));
        locals.var_vthq_dn11 = ((locals.var_pb20_dn11 + locals.var_vfb_dn11) + ((((((2.0 * locals.var_q_nsub_dn11) * 1.034943e-10) * locals.var_pb20) + (assign5210_e3419 * locals.var_pb20_dn11)) / (2.0 * assign5210_e3422)) / locals.var_c_fox0));
        locals.var_vthq_dn12 = ((locals.var_pb20_dn12 + locals.var_vfb_dn12) + ((((((2.0 * locals.var_q_nsub_dn12) * 1.034943e-10) * locals.var_pb20) + (assign5210_e3419 * locals.var_pb20_dn12)) / (2.0 * assign5210_e3422)) / locals.var_c_fox0));
        locals.var_vthq_rv = 0.0;

        let assign5220_e3428: f64 = if locals.var_flg_qme == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard53 = assign5220_e3428;
        locals.var_guard53_rv = 0.0;

        let (assign5230_e3432, assign5230_e3432_d_n0, assign5230_e3432_d_n2, assign5230_e3432_d_n4, assign5230_e3432_d_n5, assign5230_e3432_d_n6, assign5230_e3432_d_n8, assign5230_e3432_d_n10, assign5230_e3432_d_n11, assign5230_e3432_d_n12,) = {
    if (locals.var_guard53 != 0.0) {
        (locals.var_tfox0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tfoxe, locals.var_tfoxe_dn0, locals.var_tfoxe_dn2, locals.var_tfoxe_dn4, locals.var_tfoxe_dn5, locals.var_tfoxe_dn6, locals.var_tfoxe_dn8, locals.var_tfoxe_dn10, locals.var_tfoxe_dn11, locals.var_tfoxe_dn12,)
    }
};
        locals.var_tfoxe = assign5230_e3432;
        locals.var_tfoxe_dn0 = assign5230_e3432_d_n0;
        locals.var_tfoxe_dn2 = assign5230_e3432_d_n2;
        locals.var_tfoxe_dn4 = assign5230_e3432_d_n4;
        locals.var_tfoxe_dn5 = assign5230_e3432_d_n5;
        locals.var_tfoxe_dn6 = assign5230_e3432_d_n6;
        locals.var_tfoxe_dn8 = assign5230_e3432_d_n8;
        locals.var_tfoxe_dn10 = assign5230_e3432_d_n10;
        locals.var_tfoxe_dn11 = assign5230_e3432_d_n11;
        locals.var_tfoxe_dn12 = assign5230_e3432_d_n12;
        locals.var_tfoxe_rv = 0.0;

        let (assign5240_e3436, assign5240_e3436_d_n0, assign5240_e3436_d_n2, assign5240_e3436_d_n4, assign5240_e3436_d_n5, assign5240_e3436_d_n6, assign5240_e3436_d_n8, assign5240_e3436_d_n10, assign5240_e3436_d_n11, assign5240_e3436_d_n12,) = {
    if (locals.var_guard53 != 0.0) {
        (locals.var_c_fox0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_c_fox, locals.var_c_fox_dn0, locals.var_c_fox_dn2, locals.var_c_fox_dn4, locals.var_c_fox_dn5, locals.var_c_fox_dn6, locals.var_c_fox_dn8, locals.var_c_fox_dn10, locals.var_c_fox_dn11, locals.var_c_fox_dn12,)
    }
};
        locals.var_c_fox = assign5240_e3436;
        locals.var_c_fox_dn0 = assign5240_e3436_d_n0;
        locals.var_c_fox_dn2 = assign5240_e3436_d_n2;
        locals.var_c_fox_dn4 = assign5240_e3436_d_n4;
        locals.var_c_fox_dn5 = assign5240_e3436_d_n5;
        locals.var_c_fox_dn6 = assign5240_e3436_d_n6;
        locals.var_c_fox_dn8 = assign5240_e3436_d_n8;
        locals.var_c_fox_dn10 = assign5240_e3436_d_n10;
        locals.var_c_fox_dn11 = assign5240_e3436_d_n11;
        locals.var_c_fox_dn12 = assign5240_e3436_d_n12;
        locals.var_c_fox_rv = 0.0;

        let (assign5250_e3440, assign5250_e3440_d_n0, assign5250_e3440_d_n2, assign5250_e3440_d_n4, assign5250_e3440_d_n5, assign5250_e3440_d_n6, assign5250_e3440_d_n8, assign5250_e3440_d_n10, assign5250_e3440_d_n11, assign5250_e3440_d_n12,) = {
    if (locals.var_guard53 != 0.0) {
        (locals.var_c_fox0_inv, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_c_fox_inv, locals.var_c_fox_inv_dn0, locals.var_c_fox_inv_dn2, locals.var_c_fox_inv_dn4, locals.var_c_fox_inv_dn5, locals.var_c_fox_inv_dn6, locals.var_c_fox_inv_dn8, locals.var_c_fox_inv_dn10, locals.var_c_fox_inv_dn11, locals.var_c_fox_inv_dn12,)
    }
};
        locals.var_c_fox_inv = assign5250_e3440;
        locals.var_c_fox_inv_dn0 = assign5250_e3440_d_n0;
        locals.var_c_fox_inv_dn2 = assign5250_e3440_d_n2;
        locals.var_c_fox_inv_dn4 = assign5250_e3440_d_n4;
        locals.var_c_fox_inv_dn5 = assign5250_e3440_d_n5;
        locals.var_c_fox_inv_dn6 = assign5250_e3440_d_n6;
        locals.var_c_fox_inv_dn8 = assign5250_e3440_d_n8;
        locals.var_c_fox_inv_dn10 = assign5250_e3440_d_n10;
        locals.var_c_fox_inv_dn11 = assign5250_e3440_d_n11;
        locals.var_c_fox_inv_dn12 = assign5250_e3440_d_n12;
        locals.var_c_fox_inv_rv = 0.0;

        let (assign5260_e3448, assign5260_e3448_d_n0, assign5260_e3448_d_n2, assign5260_e3448_d_n4, assign5260_e3448_d_n5, assign5260_e3448_d_n6, assign5260_e3448_d_n8, assign5260_e3448_d_n10, assign5260_e3448_d_n11, assign5260_e3448_d_n12,) = {
    if (locals.var_guard53 != 0.0) {
        let assign5260_e3444: f64 = (locals.var_cnst0soi * locals.var_c_fox0_inv);
        let assign5260_e3446: f64 = (assign5260_e3444 * locals.var_c_fox0_inv);
        (assign5260_e3446, ((locals.var_cnst0soi_dn0 * locals.var_c_fox0_inv) * locals.var_c_fox0_inv), ((locals.var_cnst0soi_dn2 * locals.var_c_fox0_inv) * locals.var_c_fox0_inv), ((locals.var_cnst0soi_dn4 * locals.var_c_fox0_inv) * locals.var_c_fox0_inv), ((locals.var_cnst0soi_dn5 * locals.var_c_fox0_inv) * locals.var_c_fox0_inv), ((locals.var_cnst0soi_dn6 * locals.var_c_fox0_inv) * locals.var_c_fox0_inv), ((locals.var_cnst0soi_dn8 * locals.var_c_fox0_inv) * locals.var_c_fox0_inv), ((locals.var_cnst0soi_dn10 * locals.var_c_fox0_inv) * locals.var_c_fox0_inv), ((locals.var_cnst0soi_dn11 * locals.var_c_fox0_inv) * locals.var_c_fox0_inv), ((locals.var_cnst0soi_dn12 * locals.var_c_fox0_inv) * locals.var_c_fox0_inv),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign5260_e3448;
        locals.var_t0_dn0 = assign5260_e3448_d_n0;
        locals.var_t0_dn2 = assign5260_e3448_d_n2;
        locals.var_t0_dn4 = assign5260_e3448_d_n4;
        locals.var_t0_dn5 = assign5260_e3448_d_n5;
        locals.var_t0_dn6 = assign5260_e3448_d_n6;
        locals.var_t0_dn8 = assign5260_e3448_d_n8;
        locals.var_t0_dn10 = assign5260_e3448_d_n10;
        locals.var_t0_dn11 = assign5260_e3448_d_n11;
        locals.var_t0_dn12 = assign5260_e3448_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign5270_e3454, assign5270_e3454_d_n0, assign5270_e3454_d_n2, assign5270_e3454_d_n4, assign5270_e3454_d_n5, assign5270_e3454_d_n6, assign5270_e3454_d_n8, assign5270_e3454_d_n10, assign5270_e3454_d_n11, assign5270_e3454_d_n12,) = {
    if (locals.var_guard53 != 0.0) {
        let assign5270_e3452: f64 = (locals.var_t0 * locals.var_cnst0soi);
        (assign5270_e3452, ((locals.var_t0_dn0 * locals.var_cnst0soi) + (locals.var_t0 * locals.var_cnst0soi_dn0)), ((locals.var_t0_dn2 * locals.var_cnst0soi) + (locals.var_t0 * locals.var_cnst0soi_dn2)), ((locals.var_t0_dn4 * locals.var_cnst0soi) + (locals.var_t0 * locals.var_cnst0soi_dn4)), ((locals.var_t0_dn5 * locals.var_cnst0soi) + (locals.var_t0 * locals.var_cnst0soi_dn5)), ((locals.var_t0_dn6 * locals.var_cnst0soi) + (locals.var_t0 * locals.var_cnst0soi_dn6)), ((locals.var_t0_dn8 * locals.var_cnst0soi) + (locals.var_t0 * locals.var_cnst0soi_dn8)), ((locals.var_t0_dn10 * locals.var_cnst0soi) + (locals.var_t0 * locals.var_cnst0soi_dn10)), ((locals.var_t0_dn11 * locals.var_cnst0soi) + (locals.var_t0 * locals.var_cnst0soi_dn11)), ((locals.var_t0_dn12 * locals.var_cnst0soi) + (locals.var_t0 * locals.var_cnst0soi_dn12)),)
    } else {
        (locals.var_cnstc_foxi, locals.var_cnstc_foxi_dn0, locals.var_cnstc_foxi_dn2, locals.var_cnstc_foxi_dn4, locals.var_cnstc_foxi_dn5, locals.var_cnstc_foxi_dn6, locals.var_cnstc_foxi_dn8, locals.var_cnstc_foxi_dn10, locals.var_cnstc_foxi_dn11, locals.var_cnstc_foxi_dn12,)
    }
};
        locals.var_cnstc_foxi = assign5270_e3454;
        locals.var_cnstc_foxi_dn0 = assign5270_e3454_d_n0;
        locals.var_cnstc_foxi_dn2 = assign5270_e3454_d_n2;
        locals.var_cnstc_foxi_dn4 = assign5270_e3454_d_n4;
        locals.var_cnstc_foxi_dn5 = assign5270_e3454_d_n5;
        locals.var_cnstc_foxi_dn6 = assign5270_e3454_d_n6;
        locals.var_cnstc_foxi_dn8 = assign5270_e3454_d_n8;
        locals.var_cnstc_foxi_dn10 = assign5270_e3454_d_n10;
        locals.var_cnstc_foxi_dn11 = assign5270_e3454_d_n11;
        locals.var_cnstc_foxi_dn12 = assign5270_e3454_d_n12;
        locals.var_cnstc_foxi_rv = 0.0;

        let (assign5280_e3465, assign5280_e3465_d_n0, assign5280_e3465_d_n2, assign5280_e3465_d_n4, assign5280_e3465_d_n5, assign5280_e3465_d_n6, assign5280_e3465_d_n8, assign5280_e3465_d_n10, assign5280_e3465_d_n11, assign5280_e3465_d_n12,) = {
    if (locals.var_guard53 == 0.0) {
        let assign5280_e3459: f64 = (locals.var_vgs - locals.var_vbs);
        let assign5280_e3461: f64 = (assign5280_e3459 - locals.var_vthq);
        let assign5280_e3463: f64 = (assign5280_e3461 + p.p194);
        (assign5280_e3463, ((-locals.var_vbs_dn0) - locals.var_vthq_dn0), ((-locals.var_vbs_dn2) - locals.var_vthq_dn2), ((-locals.var_vbs_dn4) - locals.var_vthq_dn4), ((locals.var_vgs_dn5 - locals.var_vbs_dn5) - locals.var_vthq_dn5), ((-locals.var_vbs_dn6) - locals.var_vthq_dn6), ((-locals.var_vbs_dn8) - locals.var_vthq_dn8), ((-locals.var_vbs_dn10) - locals.var_vthq_dn10), ((locals.var_vgs_dn11 - locals.var_vbs_dn11) - locals.var_vthq_dn11), ((locals.var_vgs_dn12 - locals.var_vbs_dn12) - locals.var_vthq_dn12),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign5280_e3465;
        locals.var_t5_dn0 = assign5280_e3465_d_n0;
        locals.var_t5_dn2 = assign5280_e3465_d_n2;
        locals.var_t5_dn4 = assign5280_e3465_d_n4;
        locals.var_t5_dn5 = assign5280_e3465_d_n5;
        locals.var_t5_dn6 = assign5280_e3465_d_n6;
        locals.var_t5_dn8 = assign5280_e3465_d_n8;
        locals.var_t5_dn10 = assign5280_e3465_d_n10;
        locals.var_t5_dn11 = assign5280_e3465_d_n11;
        locals.var_t5_dn12 = assign5280_e3465_d_n12;
        locals.var_t5_rv = 0.0;

        let (assign5290_e3479, assign5290_e3479_d_n0, assign5290_e3479_d_n2, assign5290_e3479_d_n4, assign5290_e3479_d_n5, assign5290_e3479_d_n6, assign5290_e3479_d_n8, assign5290_e3479_d_n10, assign5290_e3479_d_n11, assign5290_e3479_d_n12,) = {
    if (locals.var_guard53 == 0.0) {
        let assign5290_e3470: f64 = (locals.var_t5 * locals.var_t5);
        let assign5290_e3473: f64 = (4.0 * 0.0001);
        let assign5290_e3475: f64 = (assign5290_e3473 * 0.0001);
        let assign5290_e3476: f64 = (assign5290_e3470 + assign5290_e3475);
        let assign5290_e3477: f64 = (assign5290_e3476).sqrt();
        (assign5290_e3477, (((locals.var_t5_dn0 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn0)) / (2.0 * assign5290_e3477)), (((locals.var_t5_dn2 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn2)) / (2.0 * assign5290_e3477)), (((locals.var_t5_dn4 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn4)) / (2.0 * assign5290_e3477)), (((locals.var_t5_dn5 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn5)) / (2.0 * assign5290_e3477)), (((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)) / (2.0 * assign5290_e3477)), (((locals.var_t5_dn8 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn8)) / (2.0 * assign5290_e3477)), (((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)) / (2.0 * assign5290_e3477)), (((locals.var_t5_dn11 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn11)) / (2.0 * assign5290_e3477)), (((locals.var_t5_dn12 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn12)) / (2.0 * assign5290_e3477)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign5290_e3479;
        locals.var_tmf2_dn0 = assign5290_e3479_d_n0;
        locals.var_tmf2_dn2 = assign5290_e3479_d_n2;
        locals.var_tmf2_dn4 = assign5290_e3479_d_n4;
        locals.var_tmf2_dn5 = assign5290_e3479_d_n5;
        locals.var_tmf2_dn6 = assign5290_e3479_d_n6;
        locals.var_tmf2_dn8 = assign5290_e3479_d_n8;
        locals.var_tmf2_dn10 = assign5290_e3479_d_n10;
        locals.var_tmf2_dn11 = assign5290_e3479_d_n11;
        locals.var_tmf2_dn12 = assign5290_e3479_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign5300_e3490, assign5300_e3490_d_n0, assign5300_e3490_d_n2, assign5300_e3490_d_n4, assign5300_e3490_d_n5, assign5300_e3490_d_n6, assign5300_e3490_d_n8, assign5300_e3490_d_n10, assign5300_e3490_d_n11, assign5300_e3490_d_n12,) = {
    if (locals.var_guard53 == 0.0) {
        let assign5300_e3486: f64 = (locals.var_t5 / locals.var_tmf2);
        let assign5300_e3487: f64 = (1.0 + assign5300_e3486);
        let assign5300_e3488: f64 = (0.5 * assign5300_e3487);
        (assign5300_e3488, (0.5 * (((locals.var_t5_dn0 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn2 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn4 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn5 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn6 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn8 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn10 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn11 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn12 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign5300_e3490;
        locals.var_t3_dn0 = assign5300_e3490_d_n0;
        locals.var_t3_dn2 = assign5300_e3490_d_n2;
        locals.var_t3_dn4 = assign5300_e3490_d_n4;
        locals.var_t3_dn5 = assign5300_e3490_d_n5;
        locals.var_t3_dn6 = assign5300_e3490_d_n6;
        locals.var_t3_dn8 = assign5300_e3490_d_n8;
        locals.var_t3_dn10 = assign5300_e3490_d_n10;
        locals.var_t3_dn11 = assign5300_e3490_d_n11;
        locals.var_t3_dn12 = assign5300_e3490_d_n12;
        locals.var_t3_rv = 0.0;

        let (assign5310_e3503, assign5310_e3503_d_n0, assign5310_e3503_d_n2, assign5310_e3503_d_n4, assign5310_e3503_d_n5, assign5310_e3503_d_n6, assign5310_e3503_d_n8, assign5310_e3503_d_n10, assign5310_e3503_d_n11, assign5310_e3503_d_n12,) = {
    if (locals.var_guard53 == 0.0) {
        let assign5310_e3496: f64 = (locals.var_t5 + locals.var_tmf2);
        let assign5310_e3497: f64 = (0.5 * assign5310_e3496);
        let assign5310_e3500: f64 = (1e-10 * 0.0001);
        let assign5310_e3501: f64 = (assign5310_e3497 + assign5310_e3500);
        (assign5310_e3501, (0.5 * (locals.var_t5_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t5_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t5_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t5_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t5_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t5_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t5_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t5_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t5_dn12 + locals.var_tmf2_dn12)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign5310_e3503;
        locals.var_t2_dn0 = assign5310_e3503_d_n0;
        locals.var_t2_dn2 = assign5310_e3503_d_n2;
        locals.var_t2_dn4 = assign5310_e3503_d_n4;
        locals.var_t2_dn5 = assign5310_e3503_d_n5;
        locals.var_t2_dn6 = assign5310_e3503_d_n6;
        locals.var_t2_dn8 = assign5310_e3503_d_n8;
        locals.var_t2_dn10 = assign5310_e3503_d_n10;
        locals.var_t2_dn11 = assign5310_e3503_d_n11;
        locals.var_t2_dn12 = assign5310_e3503_d_n12;
        locals.var_t2_rv = 0.0;

        let assign5320_e3506: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard54 = assign5320_e3506;
        locals.var_guard54_rv = 0.0;

        let (assign5330_e3513, assign5330_e3513_d_n0, assign5330_e3513_d_n2, assign5330_e3513_d_n4, assign5330_e3513_d_n5, assign5330_e3513_d_n6, assign5330_e3513_d_n8, assign5330_e3513_d_n10, assign5330_e3513_d_n11, assign5330_e3513_d_n12,) = {
    if ((locals.var_guard53 == 0.0) && (locals.var_guard54 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign5330_e3513;
        locals.var_t2_dn0 = assign5330_e3513_d_n0;
        locals.var_t2_dn2 = assign5330_e3513_d_n2;
        locals.var_t2_dn4 = assign5330_e3513_d_n4;
        locals.var_t2_dn5 = assign5330_e3513_d_n5;
        locals.var_t2_dn6 = assign5330_e3513_d_n6;
        locals.var_t2_dn8 = assign5330_e3513_d_n8;
        locals.var_t2_dn10 = assign5330_e3513_d_n10;
        locals.var_t2_dn11 = assign5330_e3513_d_n11;
        locals.var_t2_dn12 = assign5330_e3513_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign5340_e3520, assign5340_e3520_d_n0, assign5340_e3520_d_n2, assign5340_e3520_d_n4, assign5340_e3520_d_n5, assign5340_e3520_d_n6, assign5340_e3520_d_n8, assign5340_e3520_d_n10, assign5340_e3520_d_n11, assign5340_e3520_d_n12,) = {
    if ((locals.var_guard53 == 0.0) && (locals.var_guard54 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign5340_e3520;
        locals.var_t3_dn0 = assign5340_e3520_d_n0;
        locals.var_t3_dn2 = assign5340_e3520_d_n2;
        locals.var_t3_dn4 = assign5340_e3520_d_n4;
        locals.var_t3_dn5 = assign5340_e3520_d_n5;
        locals.var_t3_dn6 = assign5340_e3520_d_n6;
        locals.var_t3_dn8 = assign5340_e3520_d_n8;
        locals.var_t3_dn10 = assign5340_e3520_d_n10;
        locals.var_t3_dn11 = assign5340_e3520_d_n11;
        locals.var_t3_dn12 = assign5340_e3520_d_n12;
        locals.var_t3_rv = 0.0;

        let (assign5350_e3527, assign5350_e3527_d_n0, assign5350_e3527_d_n2, assign5350_e3527_d_n4, assign5350_e3527_d_n5, assign5350_e3527_d_n6, assign5350_e3527_d_n8, assign5350_e3527_d_n10, assign5350_e3527_d_n11, assign5350_e3527_d_n12,) = {
    if (locals.var_guard53 == 0.0) {
        let assign5350_e3525: f64 = (1.0 / locals.var_t2);
        (assign5350_e3525, (-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn11 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn12 / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign5350_e3527;
        locals.var_t3_dn0 = assign5350_e3527_d_n0;
        locals.var_t3_dn2 = assign5350_e3527_d_n2;
        locals.var_t3_dn4 = assign5350_e3527_d_n4;
        locals.var_t3_dn5 = assign5350_e3527_d_n5;
        locals.var_t3_dn6 = assign5350_e3527_d_n6;
        locals.var_t3_dn8 = assign5350_e3527_d_n8;
        locals.var_t3_dn10 = assign5350_e3527_d_n10;
        locals.var_t3_dn11 = assign5350_e3527_d_n11;
        locals.var_t3_dn12 = assign5350_e3527_d_n12;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5360_e3535, assign5360_e3535_d_n0, assign5360_e3535_d_n2, assign5360_e3535_d_n4, assign5360_e3535_d_n5, assign5360_e3535_d_n6, assign5360_e3535_d_n8, assign5360_e3535_d_n10, assign5360_e3535_d_n11, assign5360_e3535_d_n12,) = {
    if (locals.var_guard53 == 0.0) {
        let assign5360_e3532: f64 = (locals.var_vthq).abs();
        let assign5360_e3533: f64 = (2.0 * assign5360_e3532);
        (assign5360_e3533, (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn0 } else { (-locals.var_vthq_dn0) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn2 } else { (-locals.var_vthq_dn2) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn4 } else { (-locals.var_vthq_dn4) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn5 } else { (-locals.var_vthq_dn5) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn6 } else { (-locals.var_vthq_dn6) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn8 } else { (-locals.var_vthq_dn8) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn10 } else { (-locals.var_vthq_dn10) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn11 } else { (-locals.var_vthq_dn11) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn12 } else { (-locals.var_vthq_dn12) }),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign5360_e3535;
        locals.var_t4_dn0 = assign5360_e3535_d_n0;
        locals.var_t4_dn2 = assign5360_e3535_d_n2;
        locals.var_t4_dn4 = assign5360_e3535_d_n4;
        locals.var_t4_dn5 = assign5360_e3535_d_n5;
        locals.var_t4_dn6 = assign5360_e3535_d_n6;
        locals.var_t4_dn8 = assign5360_e3535_d_n8;
        locals.var_t4_dn10 = assign5360_e3535_d_n10;
        locals.var_t4_dn11 = assign5360_e3535_d_n11;
        locals.var_t4_dn12 = assign5360_e3535_d_n12;
        locals.var_t4_rv = 0.0;

        let (assign5370_e3544, assign5370_e3544_d_n0, assign5370_e3544_d_n2, assign5370_e3544_d_n4, assign5370_e3544_d_n5, assign5370_e3544_d_n6, assign5370_e3544_d_n8, assign5370_e3544_d_n10, assign5370_e3544_d_n11, assign5370_e3544_d_n12,) = {
    if (locals.var_guard53 == 0.0) {
        let assign5370_e3540: f64 = (locals.var_vfb - locals.var_vthq);
        let assign5370_e3542: f64 = (assign5370_e3540 + p.p194);
        (assign5370_e3542, (locals.var_vfb_dn0 - locals.var_vthq_dn0), (locals.var_vfb_dn2 - locals.var_vthq_dn2), (locals.var_vfb_dn4 - locals.var_vthq_dn4), (locals.var_vfb_dn5 - locals.var_vthq_dn5), (locals.var_vfb_dn6 - locals.var_vthq_dn6), (locals.var_vfb_dn8 - locals.var_vthq_dn8), (locals.var_vfb_dn10 - locals.var_vthq_dn10), (locals.var_vfb_dn11 - locals.var_vthq_dn11), (locals.var_vfb_dn12 - locals.var_vthq_dn12),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn8, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12,)
    }
};
        locals.var_t6 = assign5370_e3544;
        locals.var_t6_dn0 = assign5370_e3544_d_n0;
        locals.var_t6_dn2 = assign5370_e3544_d_n2;
        locals.var_t6_dn4 = assign5370_e3544_d_n4;
        locals.var_t6_dn5 = assign5370_e3544_d_n5;
        locals.var_t6_dn6 = assign5370_e3544_d_n6;
        locals.var_t6_dn8 = assign5370_e3544_d_n8;
        locals.var_t6_dn10 = assign5370_e3544_d_n10;
        locals.var_t6_dn11 = assign5370_e3544_d_n11;
        locals.var_t6_dn12 = assign5370_e3544_d_n12;
        locals.var_t6_rv = 0.0;

        let assign5380_e3547: f64 = if locals.var_t6 > locals.var_t4 { 1.0 } else { 0.0 };
        locals.var_guard55 = assign5380_e3547;
        locals.var_guard55_rv = 0.0;

        let (assign5390_e3554, assign5390_e3554_d_n0, assign5390_e3554_d_n2, assign5390_e3554_d_n4, assign5390_e3554_d_n5, assign5390_e3554_d_n6, assign5390_e3554_d_n8, assign5390_e3554_d_n10, assign5390_e3554_d_n11, assign5390_e3554_d_n12,) = {
    if ((locals.var_guard53 == 0.0) && (locals.var_guard55 != 0.0)) {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn8, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign5390_e3554;
        locals.var_t4_dn0 = assign5390_e3554_d_n0;
        locals.var_t4_dn2 = assign5390_e3554_d_n2;
        locals.var_t4_dn4 = assign5390_e3554_d_n4;
        locals.var_t4_dn5 = assign5390_e3554_d_n5;
        locals.var_t4_dn6 = assign5390_e3554_d_n6;
        locals.var_t4_dn8 = assign5390_e3554_d_n8;
        locals.var_t4_dn10 = assign5390_e3554_d_n10;
        locals.var_t4_dn11 = assign5390_e3554_d_n11;
        locals.var_t4_dn12 = assign5390_e3554_d_n12;
        locals.var_t4_rv = 0.0;

        let (assign5400_e3565, assign5400_e3565_d_n0, assign5400_e3565_d_n2, assign5400_e3565_d_n4, assign5400_e3565_d_n5, assign5400_e3565_d_n6, assign5400_e3565_d_n8, assign5400_e3565_d_n10, assign5400_e3565_d_n11, assign5400_e3565_d_n12,) = {
    if (locals.var_guard53 == 0.0) {
        let assign5400_e3559: f64 = (1.0 / locals.var_t4);
        let assign5400_e3561: f64 = (assign5400_e3559 - locals.var_t3);
        let assign5400_e3563: f64 = (assign5400_e3561 - 0.0001);
        (assign5400_e3563, ((-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn0), ((-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn2), ((-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn4), ((-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn5), ((-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn6), ((-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn8), ((-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn10), ((-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn11), ((-(locals.var_t4_dn12 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn12),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn8, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12,)
    }
};
        locals.var_tmf1 = assign5400_e3565;
        locals.var_tmf1_dn0 = assign5400_e3565_d_n0;
        locals.var_tmf1_dn2 = assign5400_e3565_d_n2;
        locals.var_tmf1_dn4 = assign5400_e3565_d_n4;
        locals.var_tmf1_dn5 = assign5400_e3565_d_n5;
        locals.var_tmf1_dn6 = assign5400_e3565_d_n6;
        locals.var_tmf1_dn8 = assign5400_e3565_d_n8;
        locals.var_tmf1_dn10 = assign5400_e3565_d_n10;
        locals.var_tmf1_dn11 = assign5400_e3565_d_n11;
        locals.var_tmf1_dn12 = assign5400_e3565_d_n12;
        locals.var_tmf1_rv = 0.0;

        let (assign5410_e3576, assign5410_e3576_d_n0, assign5410_e3576_d_n2, assign5410_e3576_d_n4, assign5410_e3576_d_n5, assign5410_e3576_d_n6, assign5410_e3576_d_n8, assign5410_e3576_d_n10, assign5410_e3576_d_n11, assign5410_e3576_d_n12,) = {
    if (locals.var_guard53 == 0.0) {
        let assign5410_e3571: f64 = (1.0 / locals.var_t4);
        let assign5410_e3572: f64 = (4.0 * assign5410_e3571);
        let assign5410_e3574: f64 = (assign5410_e3572 * 0.0001);
        (assign5410_e3574, ((4.0 * (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4)))) * 0.0001), ((4.0 * (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4)))) * 0.0001), ((4.0 * (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4)))) * 0.0001), ((4.0 * (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4)))) * 0.0001), ((4.0 * (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4)))) * 0.0001), ((4.0 * (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4)))) * 0.0001), ((4.0 * (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4)))) * 0.0001), ((4.0 * (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4)))) * 0.0001), ((4.0 * (-(locals.var_t4_dn12 / (locals.var_t4 * locals.var_t4)))) * 0.0001),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign5410_e3576;
        locals.var_tmf2_dn0 = assign5410_e3576_d_n0;
        locals.var_tmf2_dn2 = assign5410_e3576_d_n2;
        locals.var_tmf2_dn4 = assign5410_e3576_d_n4;
        locals.var_tmf2_dn5 = assign5410_e3576_d_n5;
        locals.var_tmf2_dn6 = assign5410_e3576_d_n6;
        locals.var_tmf2_dn8 = assign5410_e3576_d_n8;
        locals.var_tmf2_dn10 = assign5410_e3576_d_n10;
        locals.var_tmf2_dn11 = assign5410_e3576_d_n11;
        locals.var_tmf2_dn12 = assign5410_e3576_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign5420_e3587, assign5420_e3587_d_n0, assign5420_e3587_d_n2, assign5420_e3587_d_n4, assign5420_e3587_d_n5, assign5420_e3587_d_n6, assign5420_e3587_d_n8, assign5420_e3587_d_n10, assign5420_e3587_d_n11, assign5420_e3587_d_n12,) = {
    if (locals.var_guard53 == 0.0) {
        let (assign5420_e3585, assign5420_e3585_d_n0, assign5420_e3585_d_n2, assign5420_e3585_d_n4, assign5420_e3585_d_n5, assign5420_e3585_d_n6, assign5420_e3585_d_n8, assign5420_e3585_d_n10, assign5420_e3585_d_n11, assign5420_e3585_d_n12,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
            } else {
                let assign5420_e3584: f64 = (-locals.var_tmf2);
                (assign5420_e3584, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12),)
            }
        };
        (assign5420_e3585, assign5420_e3585_d_n0, assign5420_e3585_d_n2, assign5420_e3585_d_n4, assign5420_e3585_d_n5, assign5420_e3585_d_n6, assign5420_e3585_d_n8, assign5420_e3585_d_n10, assign5420_e3585_d_n11, assign5420_e3585_d_n12,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign5420_e3587;
        locals.var_tmf2_dn0 = assign5420_e3587_d_n0;
        locals.var_tmf2_dn2 = assign5420_e3587_d_n2;
        locals.var_tmf2_dn4 = assign5420_e3587_d_n4;
        locals.var_tmf2_dn5 = assign5420_e3587_d_n5;
        locals.var_tmf2_dn6 = assign5420_e3587_d_n6;
        locals.var_tmf2_dn8 = assign5420_e3587_d_n8;
        locals.var_tmf2_dn10 = assign5420_e3587_d_n10;
        locals.var_tmf2_dn11 = assign5420_e3587_d_n11;
        locals.var_tmf2_dn12 = assign5420_e3587_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign5430_e3597, assign5430_e3597_d_n0, assign5430_e3597_d_n2, assign5430_e3597_d_n4, assign5430_e3597_d_n5, assign5430_e3597_d_n6, assign5430_e3597_d_n8, assign5430_e3597_d_n10, assign5430_e3597_d_n11, assign5430_e3597_d_n12,) = {
    if (locals.var_guard53 == 0.0) {
        let assign5430_e3592: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign5430_e3594: f64 = (assign5430_e3592 + locals.var_tmf2);
        let assign5430_e3595: f64 = (assign5430_e3594).sqrt();
        (assign5430_e3595, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign5430_e3595)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign5430_e3595)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign5430_e3595)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign5430_e3595)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign5430_e3595)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign5430_e3595)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign5430_e3595)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign5430_e3595)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign5430_e3595)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign5430_e3597;
        locals.var_tmf2_dn0 = assign5430_e3597_d_n0;
        locals.var_tmf2_dn2 = assign5430_e3597_d_n2;
        locals.var_tmf2_dn4 = assign5430_e3597_d_n4;
        locals.var_tmf2_dn5 = assign5430_e3597_d_n5;
        locals.var_tmf2_dn6 = assign5430_e3597_d_n6;
        locals.var_tmf2_dn8 = assign5430_e3597_d_n8;
        locals.var_tmf2_dn10 = assign5430_e3597_d_n10;
        locals.var_tmf2_dn11 = assign5430_e3597_d_n11;
        locals.var_tmf2_dn12 = assign5430_e3597_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign5440_e3608, assign5440_e3608_d_n0, assign5440_e3608_d_n2, assign5440_e3608_d_n4, assign5440_e3608_d_n5, assign5440_e3608_d_n6, assign5440_e3608_d_n8, assign5440_e3608_d_n10, assign5440_e3608_d_n11, assign5440_e3608_d_n12,) = {
    if (locals.var_guard53 == 0.0) {
        let assign5440_e3604: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign5440_e3605: f64 = (1.0 + assign5440_e3604);
        let assign5440_e3606: f64 = (0.5 * assign5440_e3605);
        (assign5440_e3606, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn8, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12,)
    }
};
        locals.var_t6 = assign5440_e3608;
        locals.var_t6_dn0 = assign5440_e3608_d_n0;
        locals.var_t6_dn2 = assign5440_e3608_d_n2;
        locals.var_t6_dn4 = assign5440_e3608_d_n4;
        locals.var_t6_dn5 = assign5440_e3608_d_n5;
        locals.var_t6_dn6 = assign5440_e3608_d_n6;
        locals.var_t6_dn8 = assign5440_e3608_d_n8;
        locals.var_t6_dn10 = assign5440_e3608_d_n10;
        locals.var_t6_dn11 = assign5440_e3608_d_n11;
        locals.var_t6_dn12 = assign5440_e3608_d_n12;
        locals.var_t6_rv = 0.0;

        let (assign5450_e3621, assign5450_e3621_d_n0, assign5450_e3621_d_n2, assign5450_e3621_d_n4, assign5450_e3621_d_n5, assign5450_e3621_d_n6, assign5450_e3621_d_n8, assign5450_e3621_d_n10, assign5450_e3621_d_n11, assign5450_e3621_d_n12,) = {
    if (locals.var_guard53 == 0.0) {
        let assign5450_e3613: f64 = (1.0 / locals.var_t4);
        let assign5450_e3617: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign5450_e3618: f64 = (0.5 * assign5450_e3617);
        let assign5450_e3619: f64 = (assign5450_e3613 - assign5450_e3618);
        (assign5450_e3619, ((-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-(locals.var_t4_dn12 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign5450_e3621;
        locals.var_t2_dn0 = assign5450_e3621_d_n0;
        locals.var_t2_dn2 = assign5450_e3621_d_n2;
        locals.var_t2_dn4 = assign5450_e3621_d_n4;
        locals.var_t2_dn5 = assign5450_e3621_d_n5;
        locals.var_t2_dn6 = assign5450_e3621_d_n6;
        locals.var_t2_dn8 = assign5450_e3621_d_n8;
        locals.var_t2_dn10 = assign5450_e3621_d_n10;
        locals.var_t2_dn11 = assign5450_e3621_d_n11;
        locals.var_t2_dn12 = assign5450_e3621_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign5460_e3630, assign5460_e3630_d_n0, assign5460_e3630_d_n2, assign5460_e3630_d_n4, assign5460_e3630_d_n5, assign5460_e3630_d_n6, assign5460_e3630_d_n8, assign5460_e3630_d_n10, assign5460_e3630_d_n11, assign5460_e3630_d_n12,) = {
    if (locals.var_guard53 == 0.0) {
        let assign5460_e3626: f64 = (p.p193 * locals.var_t2);
        let assign5460_e3628: f64 = (assign5460_e3626 + p.p195);
        (assign5460_e3628, (p.p193 * locals.var_t2_dn0), (p.p193 * locals.var_t2_dn2), (p.p193 * locals.var_t2_dn4), (p.p193 * locals.var_t2_dn5), (p.p193 * locals.var_t2_dn6), (p.p193 * locals.var_t2_dn8), (p.p193 * locals.var_t2_dn10), (p.p193 * locals.var_t2_dn11), (p.p193 * locals.var_t2_dn12),)
    } else {
        (locals.var_dtfox, locals.var_dtfox_dn0, locals.var_dtfox_dn2, locals.var_dtfox_dn4, locals.var_dtfox_dn5, locals.var_dtfox_dn6, locals.var_dtfox_dn8, locals.var_dtfox_dn10, locals.var_dtfox_dn11, locals.var_dtfox_dn12,)
    }
};
        locals.var_dtfox = assign5460_e3630;
        locals.var_dtfox_dn0 = assign5460_e3630_d_n0;
        locals.var_dtfox_dn2 = assign5460_e3630_d_n2;
        locals.var_dtfox_dn4 = assign5460_e3630_d_n4;
        locals.var_dtfox_dn5 = assign5460_e3630_d_n5;
        locals.var_dtfox_dn6 = assign5460_e3630_d_n6;
        locals.var_dtfox_dn8 = assign5460_e3630_d_n8;
        locals.var_dtfox_dn10 = assign5460_e3630_d_n10;
        locals.var_dtfox_dn11 = assign5460_e3630_d_n11;
        locals.var_dtfox_dn12 = assign5460_e3630_d_n12;
        locals.var_dtfox_rv = 0.0;

        let assign5470_e3633: f64 = (locals.var_dtfox * 1000000000000.0);
        let assign5470_e3635: f64 = if assign5470_e3633 < locals.var_tfox0 { 1.0 } else { 0.0 };
        locals.var_guard56 = assign5470_e3635;
        locals.var_guard56_rv = 0.0;

        let (assign5480_e3642, assign5480_e3642_d_n0, assign5480_e3642_d_n2, assign5480_e3642_d_n4, assign5480_e3642_d_n5, assign5480_e3642_d_n6, assign5480_e3642_d_n8, assign5480_e3642_d_n10, assign5480_e3642_d_n11, assign5480_e3642_d_n12,) = {
    if ((locals.var_guard53 == 0.0) && (locals.var_guard56 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dtfox, locals.var_dtfox_dn0, locals.var_dtfox_dn2, locals.var_dtfox_dn4, locals.var_dtfox_dn5, locals.var_dtfox_dn6, locals.var_dtfox_dn8, locals.var_dtfox_dn10, locals.var_dtfox_dn11, locals.var_dtfox_dn12,)
    }
};
        locals.var_dtfox = assign5480_e3642;
        locals.var_dtfox_dn0 = assign5480_e3642_d_n0;
        locals.var_dtfox_dn2 = assign5480_e3642_d_n2;
        locals.var_dtfox_dn4 = assign5480_e3642_d_n4;
        locals.var_dtfox_dn5 = assign5480_e3642_d_n5;
        locals.var_dtfox_dn6 = assign5480_e3642_d_n6;
        locals.var_dtfox_dn8 = assign5480_e3642_d_n8;
        locals.var_dtfox_dn10 = assign5480_e3642_d_n10;
        locals.var_dtfox_dn11 = assign5480_e3642_d_n11;
        locals.var_dtfox_dn12 = assign5480_e3642_d_n12;
        locals.var_dtfox_rv = 0.0;

        let (assign5490_e3649,) = {
    if ((locals.var_guard53 == 0.0) && (locals.var_guard56 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_qme,)
    }
};
        locals.var_flg_qme = assign5490_e3649;
        locals.var_flg_qme_rv = 0.0;

        let (assign5500_e3656, assign5500_e3656_d_n0, assign5500_e3656_d_n2, assign5500_e3656_d_n4, assign5500_e3656_d_n5, assign5500_e3656_d_n6, assign5500_e3656_d_n8, assign5500_e3656_d_n10, assign5500_e3656_d_n11, assign5500_e3656_d_n12,) = {
    if (locals.var_guard53 == 0.0) {
        let assign5500_e3654: f64 = (locals.var_tfox0 + locals.var_dtfox);
        (assign5500_e3654, locals.var_dtfox_dn0, locals.var_dtfox_dn2, locals.var_dtfox_dn4, locals.var_dtfox_dn5, locals.var_dtfox_dn6, locals.var_dtfox_dn8, locals.var_dtfox_dn10, locals.var_dtfox_dn11, locals.var_dtfox_dn12,)
    } else {
        (locals.var_tfoxe, locals.var_tfoxe_dn0, locals.var_tfoxe_dn2, locals.var_tfoxe_dn4, locals.var_tfoxe_dn5, locals.var_tfoxe_dn6, locals.var_tfoxe_dn8, locals.var_tfoxe_dn10, locals.var_tfoxe_dn11, locals.var_tfoxe_dn12,)
    }
};
        locals.var_tfoxe = assign5500_e3656;
        locals.var_tfoxe_dn0 = assign5500_e3656_d_n0;
        locals.var_tfoxe_dn2 = assign5500_e3656_d_n2;
        locals.var_tfoxe_dn4 = assign5500_e3656_d_n4;
        locals.var_tfoxe_dn5 = assign5500_e3656_d_n5;
        locals.var_tfoxe_dn6 = assign5500_e3656_d_n6;
        locals.var_tfoxe_dn8 = assign5500_e3656_d_n8;
        locals.var_tfoxe_dn10 = assign5500_e3656_d_n10;
        locals.var_tfoxe_dn11 = assign5500_e3656_d_n11;
        locals.var_tfoxe_dn12 = assign5500_e3656_d_n12;
        locals.var_tfoxe_rv = 0.0;

        let (assign5510_e3663, assign5510_e3663_d_n0, assign5510_e3663_d_n2, assign5510_e3663_d_n4, assign5510_e3663_d_n5, assign5510_e3663_d_n6, assign5510_e3663_d_n8, assign5510_e3663_d_n10, assign5510_e3663_d_n11, assign5510_e3663_d_n12,) = {
    if (locals.var_guard53 == 0.0) {
        let assign5510_e3661: f64 = (3.453133e-11 / locals.var_tfoxe);
        (assign5510_e3661, (-((3.453133e-11 * locals.var_tfoxe_dn0) / (locals.var_tfoxe * locals.var_tfoxe))), (-((3.453133e-11 * locals.var_tfoxe_dn2) / (locals.var_tfoxe * locals.var_tfoxe))), (-((3.453133e-11 * locals.var_tfoxe_dn4) / (locals.var_tfoxe * locals.var_tfoxe))), (-((3.453133e-11 * locals.var_tfoxe_dn5) / (locals.var_tfoxe * locals.var_tfoxe))), (-((3.453133e-11 * locals.var_tfoxe_dn6) / (locals.var_tfoxe * locals.var_tfoxe))), (-((3.453133e-11 * locals.var_tfoxe_dn8) / (locals.var_tfoxe * locals.var_tfoxe))), (-((3.453133e-11 * locals.var_tfoxe_dn10) / (locals.var_tfoxe * locals.var_tfoxe))), (-((3.453133e-11 * locals.var_tfoxe_dn11) / (locals.var_tfoxe * locals.var_tfoxe))), (-((3.453133e-11 * locals.var_tfoxe_dn12) / (locals.var_tfoxe * locals.var_tfoxe))),)
    } else {
        (locals.var_c_fox, locals.var_c_fox_dn0, locals.var_c_fox_dn2, locals.var_c_fox_dn4, locals.var_c_fox_dn5, locals.var_c_fox_dn6, locals.var_c_fox_dn8, locals.var_c_fox_dn10, locals.var_c_fox_dn11, locals.var_c_fox_dn12,)
    }
};
        locals.var_c_fox = assign5510_e3663;
        locals.var_c_fox_dn0 = assign5510_e3663_d_n0;
        locals.var_c_fox_dn2 = assign5510_e3663_d_n2;
        locals.var_c_fox_dn4 = assign5510_e3663_d_n4;
        locals.var_c_fox_dn5 = assign5510_e3663_d_n5;
        locals.var_c_fox_dn6 = assign5510_e3663_d_n6;
        locals.var_c_fox_dn8 = assign5510_e3663_d_n8;
        locals.var_c_fox_dn10 = assign5510_e3663_d_n10;
        locals.var_c_fox_dn11 = assign5510_e3663_d_n11;
        locals.var_c_fox_dn12 = assign5510_e3663_d_n12;
        locals.var_c_fox_rv = 0.0;

        let (assign5520_e3670, assign5520_e3670_d_n0, assign5520_e3670_d_n2, assign5520_e3670_d_n4, assign5520_e3670_d_n5, assign5520_e3670_d_n6, assign5520_e3670_d_n8, assign5520_e3670_d_n10, assign5520_e3670_d_n11, assign5520_e3670_d_n12,) = {
    if (locals.var_guard53 == 0.0) {
        let assign5520_e3668: f64 = (locals.var_tfoxe / 3.453133e-11);
        (assign5520_e3668, (locals.var_tfoxe_dn0 / 3.453133e-11), (locals.var_tfoxe_dn2 / 3.453133e-11), (locals.var_tfoxe_dn4 / 3.453133e-11), (locals.var_tfoxe_dn5 / 3.453133e-11), (locals.var_tfoxe_dn6 / 3.453133e-11), (locals.var_tfoxe_dn8 / 3.453133e-11), (locals.var_tfoxe_dn10 / 3.453133e-11), (locals.var_tfoxe_dn11 / 3.453133e-11), (locals.var_tfoxe_dn12 / 3.453133e-11),)
    } else {
        (locals.var_c_fox_inv, locals.var_c_fox_inv_dn0, locals.var_c_fox_inv_dn2, locals.var_c_fox_inv_dn4, locals.var_c_fox_inv_dn5, locals.var_c_fox_inv_dn6, locals.var_c_fox_inv_dn8, locals.var_c_fox_inv_dn10, locals.var_c_fox_inv_dn11, locals.var_c_fox_inv_dn12,)
    }
};
        locals.var_c_fox_inv = assign5520_e3670;
        locals.var_c_fox_inv_dn0 = assign5520_e3670_d_n0;
        locals.var_c_fox_inv_dn2 = assign5520_e3670_d_n2;
        locals.var_c_fox_inv_dn4 = assign5520_e3670_d_n4;
        locals.var_c_fox_inv_dn5 = assign5520_e3670_d_n5;
        locals.var_c_fox_inv_dn6 = assign5520_e3670_d_n6;
        locals.var_c_fox_inv_dn8 = assign5520_e3670_d_n8;
        locals.var_c_fox_inv_dn10 = assign5520_e3670_d_n10;
        locals.var_c_fox_inv_dn11 = assign5520_e3670_d_n11;
        locals.var_c_fox_inv_dn12 = assign5520_e3670_d_n12;
        locals.var_c_fox_inv_rv = 0.0;

        let (assign5530_e3681, assign5530_e3681_d_n0, assign5530_e3681_d_n2, assign5530_e3681_d_n4, assign5530_e3681_d_n5, assign5530_e3681_d_n6, assign5530_e3681_d_n8, assign5530_e3681_d_n10, assign5530_e3681_d_n11, assign5530_e3681_d_n12,) = {
    if (locals.var_guard53 == 0.0) {
        let assign5530_e3675: f64 = (locals.var_cnst0soi * locals.var_cnst0soi);
        let assign5530_e3677: f64 = (assign5530_e3675 * locals.var_c_fox_inv);
        let assign5530_e3679: f64 = (assign5530_e3677 * locals.var_c_fox_inv);
        (assign5530_e3679, ((((((locals.var_cnst0soi_dn0 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn0)) * locals.var_c_fox_inv) + (assign5530_e3675 * locals.var_c_fox_inv_dn0)) * locals.var_c_fox_inv) + (assign5530_e3677 * locals.var_c_fox_inv_dn0)), ((((((locals.var_cnst0soi_dn2 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn2)) * locals.var_c_fox_inv) + (assign5530_e3675 * locals.var_c_fox_inv_dn2)) * locals.var_c_fox_inv) + (assign5530_e3677 * locals.var_c_fox_inv_dn2)), ((((((locals.var_cnst0soi_dn4 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn4)) * locals.var_c_fox_inv) + (assign5530_e3675 * locals.var_c_fox_inv_dn4)) * locals.var_c_fox_inv) + (assign5530_e3677 * locals.var_c_fox_inv_dn4)), ((((((locals.var_cnst0soi_dn5 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn5)) * locals.var_c_fox_inv) + (assign5530_e3675 * locals.var_c_fox_inv_dn5)) * locals.var_c_fox_inv) + (assign5530_e3677 * locals.var_c_fox_inv_dn5)), ((((((locals.var_cnst0soi_dn6 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn6)) * locals.var_c_fox_inv) + (assign5530_e3675 * locals.var_c_fox_inv_dn6)) * locals.var_c_fox_inv) + (assign5530_e3677 * locals.var_c_fox_inv_dn6)), ((((((locals.var_cnst0soi_dn8 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn8)) * locals.var_c_fox_inv) + (assign5530_e3675 * locals.var_c_fox_inv_dn8)) * locals.var_c_fox_inv) + (assign5530_e3677 * locals.var_c_fox_inv_dn8)), ((((((locals.var_cnst0soi_dn10 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn10)) * locals.var_c_fox_inv) + (assign5530_e3675 * locals.var_c_fox_inv_dn10)) * locals.var_c_fox_inv) + (assign5530_e3677 * locals.var_c_fox_inv_dn10)), ((((((locals.var_cnst0soi_dn11 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn11)) * locals.var_c_fox_inv) + (assign5530_e3675 * locals.var_c_fox_inv_dn11)) * locals.var_c_fox_inv) + (assign5530_e3677 * locals.var_c_fox_inv_dn11)), ((((((locals.var_cnst0soi_dn12 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn12)) * locals.var_c_fox_inv) + (assign5530_e3675 * locals.var_c_fox_inv_dn12)) * locals.var_c_fox_inv) + (assign5530_e3677 * locals.var_c_fox_inv_dn12)),)
    } else {
        (locals.var_cnstc_foxi, locals.var_cnstc_foxi_dn0, locals.var_cnstc_foxi_dn2, locals.var_cnstc_foxi_dn4, locals.var_cnstc_foxi_dn5, locals.var_cnstc_foxi_dn6, locals.var_cnstc_foxi_dn8, locals.var_cnstc_foxi_dn10, locals.var_cnstc_foxi_dn11, locals.var_cnstc_foxi_dn12,)
    }
};
        locals.var_cnstc_foxi = assign5530_e3681;
        locals.var_cnstc_foxi_dn0 = assign5530_e3681_d_n0;
        locals.var_cnstc_foxi_dn2 = assign5530_e3681_d_n2;
        locals.var_cnstc_foxi_dn4 = assign5530_e3681_d_n4;
        locals.var_cnstc_foxi_dn5 = assign5530_e3681_d_n5;
        locals.var_cnstc_foxi_dn6 = assign5530_e3681_d_n6;
        locals.var_cnstc_foxi_dn8 = assign5530_e3681_d_n8;
        locals.var_cnstc_foxi_dn10 = assign5530_e3681_d_n10;
        locals.var_cnstc_foxi_dn11 = assign5530_e3681_d_n11;
        locals.var_cnstc_foxi_dn12 = assign5530_e3681_d_n12;
        locals.var_cnstc_foxi_rv = 0.0;

        let assign5540_e3684: f64 = (0.5 - locals.var_vbsz);
        let assign5540_e3686: f64 = (assign5540_e3684 - 0.001);
        locals.var_tmf1 = assign5540_e3686;
        locals.var_tmf1_dn0 = (-locals.var_vbsz_dn0);
        locals.var_tmf1_dn2 = (-locals.var_vbsz_dn2);
        locals.var_tmf1_dn4 = (-locals.var_vbsz_dn4);
        locals.var_tmf1_dn5 = (-locals.var_vbsz_dn5);
        locals.var_tmf1_dn6 = (-locals.var_vbsz_dn6);
        locals.var_tmf1_dn8 = (-locals.var_vbsz_dn8);
        locals.var_tmf1_dn10 = (-locals.var_vbsz_dn10);
        locals.var_tmf1_dn11 = (-locals.var_vbsz_dn11);
        locals.var_tmf1_dn12 = (-locals.var_vbsz_dn12);
        locals.var_tmf1_rv = 0.0;

        let assign5550_e3689: f64 = (4.0 * 0.5);
        let assign5550_e3691: f64 = (assign5550_e3689 * 0.001);
        locals.var_tmf2 = assign5550_e3691;
        locals.var_tmf2_dn0 = 0.0;
        locals.var_tmf2_dn2 = 0.0;
        locals.var_tmf2_dn4 = 0.0;
        locals.var_tmf2_dn5 = 0.0;
        locals.var_tmf2_dn6 = 0.0;
        locals.var_tmf2_dn8 = 0.0;
        locals.var_tmf2_dn10 = 0.0;
        locals.var_tmf2_dn11 = 0.0;
        locals.var_tmf2_dn12 = 0.0;
        locals.var_tmf2_rv = 0.0;

        let (assign5560_e3698, assign5560_e3698_d_n0, assign5560_e3698_d_n2, assign5560_e3698_d_n4, assign5560_e3698_d_n5, assign5560_e3698_d_n6, assign5560_e3698_d_n8, assign5560_e3698_d_n10, assign5560_e3698_d_n11, assign5560_e3698_d_n12,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    } else {
        let assign5560_e3697: f64 = (-locals.var_tmf2);
        (assign5560_e3697, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12),)
    }
};
        locals.var_tmf2 = assign5560_e3698;
        locals.var_tmf2_dn0 = assign5560_e3698_d_n0;
        locals.var_tmf2_dn2 = assign5560_e3698_d_n2;
        locals.var_tmf2_dn4 = assign5560_e3698_d_n4;
        locals.var_tmf2_dn5 = assign5560_e3698_d_n5;
        locals.var_tmf2_dn6 = assign5560_e3698_d_n6;
        locals.var_tmf2_dn8 = assign5560_e3698_d_n8;
        locals.var_tmf2_dn10 = assign5560_e3698_d_n10;
        locals.var_tmf2_dn11 = assign5560_e3698_d_n11;
        locals.var_tmf2_dn12 = assign5560_e3698_d_n12;
        locals.var_tmf2_rv = 0.0;

        let assign5570_e3701: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign5570_e3703: f64 = (assign5570_e3701 + locals.var_tmf2);
        let assign5570_e3704: f64 = (assign5570_e3703).sqrt();
        locals.var_tmf2 = assign5570_e3704;
        locals.var_tmf2_dn0 = ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign5570_e3704));
        locals.var_tmf2_dn2 = ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign5570_e3704));
        locals.var_tmf2_dn4 = ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign5570_e3704));
        locals.var_tmf2_dn5 = ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign5570_e3704));
        locals.var_tmf2_dn6 = ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign5570_e3704));
        locals.var_tmf2_dn8 = ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign5570_e3704));
        locals.var_tmf2_dn10 = ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign5570_e3704));
        locals.var_tmf2_dn11 = ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign5570_e3704));
        locals.var_tmf2_dn12 = ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign5570_e3704));
        locals.var_tmf2_rv = 0.0;

        let assign5580_e3709: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign5580_e3710: f64 = (1.0 + assign5580_e3709);
        let assign5580_e3711: f64 = (0.5 * assign5580_e3710);
        locals.var_t0 = assign5580_e3711;
        locals.var_t0_dn0 = (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn2 = (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn4 = (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn5 = (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn6 = (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn8 = (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn10 = (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn11 = (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn12 = (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_rv = 0.0;

        let assign5590_e3716: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign5590_e3717: f64 = (0.5 * assign5590_e3716);
        let assign5590_e3718: f64 = (0.5 - assign5590_e3717);
        locals.var_vbsz2 = assign5590_e3718;
        locals.var_vbsz2_dn0 = (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)));
        locals.var_vbsz2_dn2 = (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)));
        locals.var_vbsz2_dn4 = (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)));
        locals.var_vbsz2_dn5 = (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)));
        locals.var_vbsz2_dn6 = (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)));
        locals.var_vbsz2_dn8 = (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)));
        locals.var_vbsz2_dn10 = (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)));
        locals.var_vbsz2_dn11 = (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)));
        locals.var_vbsz2_dn12 = (-(0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12)));
        locals.var_vbsz2_rv = 0.0;

        let assign5600_e3721: f64 = (locals.var_qnsub_esi2 * locals.var_pb20);
        let assign5600_e3722: f64 = (assign5600_e3721).sqrt();
        locals.var_qb0 = assign5600_e3722;
        locals.var_qb0_dn0 = (((locals.var_qnsub_esi2_dn0 * locals.var_pb20) + (locals.var_qnsub_esi2 * locals.var_pb20_dn0)) / (2.0 * assign5600_e3722));
        locals.var_qb0_dn2 = (((locals.var_qnsub_esi2_dn2 * locals.var_pb20) + (locals.var_qnsub_esi2 * locals.var_pb20_dn2)) / (2.0 * assign5600_e3722));
        locals.var_qb0_dn4 = (((locals.var_qnsub_esi2_dn4 * locals.var_pb20) + (locals.var_qnsub_esi2 * locals.var_pb20_dn4)) / (2.0 * assign5600_e3722));
        locals.var_qb0_dn5 = (((locals.var_qnsub_esi2_dn5 * locals.var_pb20) + (locals.var_qnsub_esi2 * locals.var_pb20_dn5)) / (2.0 * assign5600_e3722));
        locals.var_qb0_dn6 = (((locals.var_qnsub_esi2_dn6 * locals.var_pb20) + (locals.var_qnsub_esi2 * locals.var_pb20_dn6)) / (2.0 * assign5600_e3722));
        locals.var_qb0_dn8 = (((locals.var_qnsub_esi2_dn8 * locals.var_pb20) + (locals.var_qnsub_esi2 * locals.var_pb20_dn8)) / (2.0 * assign5600_e3722));
        locals.var_qb0_dn10 = (((locals.var_qnsub_esi2_dn10 * locals.var_pb20) + (locals.var_qnsub_esi2 * locals.var_pb20_dn10)) / (2.0 * assign5600_e3722));
        locals.var_qb0_dn11 = (((locals.var_qnsub_esi2_dn11 * locals.var_pb20) + (locals.var_qnsub_esi2 * locals.var_pb20_dn11)) / (2.0 * assign5600_e3722));
        locals.var_qb0_dn12 = (((locals.var_qnsub_esi2_dn12 * locals.var_pb20) + (locals.var_qnsub_esi2 * locals.var_pb20_dn12)) / (2.0 * assign5600_e3722));
        locals.var_qb0_rv = 0.0;

        let assign5610_e3725: f64 = (locals.var_pb20 + locals.var_vfb);
        let assign5610_e3728: f64 = (locals.var_qb0 * locals.var_c_fox_inv);
        let assign5610_e3729: f64 = (assign5610_e3725 + assign5610_e3728);
        let assign5610_e3731: f64 = (assign5610_e3729 + locals.var_ptovr);
        locals.var_vthp = assign5610_e3731;
        locals.var_vthp_dn0 = (((locals.var_pb20_dn0 + locals.var_vfb_dn0) + ((locals.var_qb0_dn0 * locals.var_c_fox_inv) + (locals.var_qb0 * locals.var_c_fox_inv_dn0))) + locals.var_ptovr_dn0);
        locals.var_vthp_dn2 = (((locals.var_pb20_dn2 + locals.var_vfb_dn2) + ((locals.var_qb0_dn2 * locals.var_c_fox_inv) + (locals.var_qb0 * locals.var_c_fox_inv_dn2))) + locals.var_ptovr_dn2);
        locals.var_vthp_dn4 = (((locals.var_pb20_dn4 + locals.var_vfb_dn4) + ((locals.var_qb0_dn4 * locals.var_c_fox_inv) + (locals.var_qb0 * locals.var_c_fox_inv_dn4))) + locals.var_ptovr_dn4);
        locals.var_vthp_dn5 = (((locals.var_pb20_dn5 + locals.var_vfb_dn5) + ((locals.var_qb0_dn5 * locals.var_c_fox_inv) + (locals.var_qb0 * locals.var_c_fox_inv_dn5))) + locals.var_ptovr_dn5);
        locals.var_vthp_dn6 = (((locals.var_pb20_dn6 + locals.var_vfb_dn6) + ((locals.var_qb0_dn6 * locals.var_c_fox_inv) + (locals.var_qb0 * locals.var_c_fox_inv_dn6))) + locals.var_ptovr_dn6);
        locals.var_vthp_dn8 = (((locals.var_pb20_dn8 + locals.var_vfb_dn8) + ((locals.var_qb0_dn8 * locals.var_c_fox_inv) + (locals.var_qb0 * locals.var_c_fox_inv_dn8))) + locals.var_ptovr_dn8);
        locals.var_vthp_dn10 = (((locals.var_pb20_dn10 + locals.var_vfb_dn10) + ((locals.var_qb0_dn10 * locals.var_c_fox_inv) + (locals.var_qb0 * locals.var_c_fox_inv_dn10))) + locals.var_ptovr_dn10);
        locals.var_vthp_dn11 = (((locals.var_pb20_dn11 + locals.var_vfb_dn11) + ((locals.var_qb0_dn11 * locals.var_c_fox_inv) + (locals.var_qb0 * locals.var_c_fox_inv_dn11))) + locals.var_ptovr_dn11);
        locals.var_vthp_dn12 = (((locals.var_pb20_dn12 + locals.var_vfb_dn12) + ((locals.var_qb0_dn12 * locals.var_c_fox_inv) + (locals.var_qb0 * locals.var_c_fox_inv_dn12))) + locals.var_ptovr_dn12);
        locals.var_vthp_rv = 0.0;

        locals.var_pb20b = locals.var_pb20;
        locals.var_pb20b_dn0 = locals.var_pb20_dn0;
        locals.var_pb20b_dn2 = locals.var_pb20_dn2;
        locals.var_pb20b_dn4 = locals.var_pb20_dn4;
        locals.var_pb20b_dn5 = locals.var_pb20_dn5;
        locals.var_pb20b_dn6 = locals.var_pb20_dn6;
        locals.var_pb20b_dn8 = locals.var_pb20_dn8;
        locals.var_pb20b_dn10 = locals.var_pb20_dn10;
        locals.var_pb20b_dn11 = locals.var_pb20_dn11;
        locals.var_pb20b_dn12 = locals.var_pb20_dn12;
        locals.var_pb20b_rv = 0.0;

        locals.var_t0 = 0.95;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign5640_e3736: f64 = (locals.var_t0 * locals.var_pb20b);
        let assign5640_e3738: f64 = (assign5640_e3736 - locals.var_vbsz2);
        let assign5640_e3740: f64 = (assign5640_e3738 - 0.001);
        locals.var_t1 = assign5640_e3740;
        locals.var_t1_dn0 = (((locals.var_t0_dn0 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn0)) - locals.var_vbsz2_dn0);
        locals.var_t1_dn2 = (((locals.var_t0_dn2 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn2)) - locals.var_vbsz2_dn2);
        locals.var_t1_dn4 = (((locals.var_t0_dn4 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn4)) - locals.var_vbsz2_dn4);
        locals.var_t1_dn5 = (((locals.var_t0_dn5 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn5)) - locals.var_vbsz2_dn5);
        locals.var_t1_dn6 = (((locals.var_t0_dn6 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn6)) - locals.var_vbsz2_dn6);
        locals.var_t1_dn8 = (((locals.var_t0_dn8 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn8)) - locals.var_vbsz2_dn8);
        locals.var_t1_dn10 = (((locals.var_t0_dn10 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn10)) - locals.var_vbsz2_dn10);
        locals.var_t1_dn11 = (((locals.var_t0_dn11 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn11)) - locals.var_vbsz2_dn11);
        locals.var_t1_dn12 = (((locals.var_t0_dn12 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn12)) - locals.var_vbsz2_dn12);
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign5650_e3743: f64 = (locals.var_t1 * locals.var_t1);
        let assign5650_e3746: f64 = (4.0 * locals.var_t0);
        let assign5650_e3748: f64 = (assign5650_e3746 * locals.var_pb20b);
        let assign5650_e3750: f64 = (assign5650_e3748 * 0.001);
        let assign5650_e3751: f64 = (assign5650_e3743 + assign5650_e3750);
        let assign5650_e3752: f64 = (assign5650_e3751).sqrt();
        locals.var_t2 = assign5650_e3752;
        locals.var_t2_dn0 = ((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) + ((((4.0 * locals.var_t0_dn0) * locals.var_pb20b) + (assign5650_e3746 * locals.var_pb20b_dn0)) * 0.001)) / (2.0 * assign5650_e3752));
        locals.var_t2_dn2 = ((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) + ((((4.0 * locals.var_t0_dn2) * locals.var_pb20b) + (assign5650_e3746 * locals.var_pb20b_dn2)) * 0.001)) / (2.0 * assign5650_e3752));
        locals.var_t2_dn4 = ((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + ((((4.0 * locals.var_t0_dn4) * locals.var_pb20b) + (assign5650_e3746 * locals.var_pb20b_dn4)) * 0.001)) / (2.0 * assign5650_e3752));
        locals.var_t2_dn5 = ((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + ((((4.0 * locals.var_t0_dn5) * locals.var_pb20b) + (assign5650_e3746 * locals.var_pb20b_dn5)) * 0.001)) / (2.0 * assign5650_e3752));
        locals.var_t2_dn6 = ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + ((((4.0 * locals.var_t0_dn6) * locals.var_pb20b) + (assign5650_e3746 * locals.var_pb20b_dn6)) * 0.001)) / (2.0 * assign5650_e3752));
        locals.var_t2_dn8 = ((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + ((((4.0 * locals.var_t0_dn8) * locals.var_pb20b) + (assign5650_e3746 * locals.var_pb20b_dn8)) * 0.001)) / (2.0 * assign5650_e3752));
        locals.var_t2_dn10 = ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + ((((4.0 * locals.var_t0_dn10) * locals.var_pb20b) + (assign5650_e3746 * locals.var_pb20b_dn10)) * 0.001)) / (2.0 * assign5650_e3752));
        locals.var_t2_dn11 = ((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + ((((4.0 * locals.var_t0_dn11) * locals.var_pb20b) + (assign5650_e3746 * locals.var_pb20b_dn11)) * 0.001)) / (2.0 * assign5650_e3752));
        locals.var_t2_dn12 = ((((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)) + ((((4.0 * locals.var_t0_dn12) * locals.var_pb20b) + (assign5650_e3746 * locals.var_pb20b_dn12)) * 0.001)) / (2.0 * assign5650_e3752));
        locals.var_t2_rv = 0.0;

        let assign5660_e3756: f64 = (locals.var_t0 * locals.var_pb20b);
        let assign5660_e3760: f64 = (locals.var_t1 + locals.var_t2);
        let assign5660_e3761: f64 = (0.5 * assign5660_e3760);
        let assign5660_e3762: f64 = (assign5660_e3756 - assign5660_e3761);
        let assign5660_e3763: f64 = (locals.var_pb20b - assign5660_e3762);
        locals.var_pbsum = assign5660_e3763;
        locals.var_pbsum_dn0 = (locals.var_pb20b_dn0 - (((locals.var_t0_dn0 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn0)) - (0.5 * (locals.var_t1_dn0 + locals.var_t2_dn0))));
        locals.var_pbsum_dn2 = (locals.var_pb20b_dn2 - (((locals.var_t0_dn2 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn2)) - (0.5 * (locals.var_t1_dn2 + locals.var_t2_dn2))));
        locals.var_pbsum_dn4 = (locals.var_pb20b_dn4 - (((locals.var_t0_dn4 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn4)) - (0.5 * (locals.var_t1_dn4 + locals.var_t2_dn4))));
        locals.var_pbsum_dn5 = (locals.var_pb20b_dn5 - (((locals.var_t0_dn5 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn5)) - (0.5 * (locals.var_t1_dn5 + locals.var_t2_dn5))));
        locals.var_pbsum_dn6 = (locals.var_pb20b_dn6 - (((locals.var_t0_dn6 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn6)) - (0.5 * (locals.var_t1_dn6 + locals.var_t2_dn6))));
        locals.var_pbsum_dn8 = (locals.var_pb20b_dn8 - (((locals.var_t0_dn8 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn8)) - (0.5 * (locals.var_t1_dn8 + locals.var_t2_dn8))));
        locals.var_pbsum_dn10 = (locals.var_pb20b_dn10 - (((locals.var_t0_dn10 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn10)) - (0.5 * (locals.var_t1_dn10 + locals.var_t2_dn10))));
        locals.var_pbsum_dn11 = (locals.var_pb20b_dn11 - (((locals.var_t0_dn11 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn11)) - (0.5 * (locals.var_t1_dn11 + locals.var_t2_dn11))));
        locals.var_pbsum_dn12 = (locals.var_pb20b_dn12 - (((locals.var_t0_dn12 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn12)) - (0.5 * (locals.var_t1_dn12 + locals.var_t2_dn12))));
        locals.var_pbsum_rv = 0.0;

        let assign5670_e3765: f64 = (locals.var_pbsum).sqrt();
        locals.var_sqrt_pbsum = assign5670_e3765;
        locals.var_sqrt_pbsum_dn0 = (locals.var_pbsum_dn0 / (2.0 * assign5670_e3765));
        locals.var_sqrt_pbsum_dn2 = (locals.var_pbsum_dn2 / (2.0 * assign5670_e3765));
        locals.var_sqrt_pbsum_dn4 = (locals.var_pbsum_dn4 / (2.0 * assign5670_e3765));
        locals.var_sqrt_pbsum_dn5 = (locals.var_pbsum_dn5 / (2.0 * assign5670_e3765));
        locals.var_sqrt_pbsum_dn6 = (locals.var_pbsum_dn6 / (2.0 * assign5670_e3765));
        locals.var_sqrt_pbsum_dn8 = (locals.var_pbsum_dn8 / (2.0 * assign5670_e3765));
        locals.var_sqrt_pbsum_dn10 = (locals.var_pbsum_dn10 / (2.0 * assign5670_e3765));
        locals.var_sqrt_pbsum_dn11 = (locals.var_pbsum_dn11 / (2.0 * assign5670_e3765));
        locals.var_sqrt_pbsum_dn12 = (locals.var_pbsum_dn12 / (2.0 * assign5670_e3765));
        locals.var_sqrt_pbsum_rv = 0.0;

        let assign5680_e3768: f64 = if p.p58 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard57 = assign5680_e3768;
        locals.var_guard57_rv = 0.0;

        let (assign5690_e3781, assign5690_e3781_d_n0, assign5690_e3781_d_n2, assign5690_e3781_d_n4, assign5690_e3781_d_n5, assign5690_e3781_d_n6, assign5690_e3781_d_n8, assign5690_e3781_d_n10, assign5690_e3781_d_n11, assign5690_e3781_d_n12,) = {
    if (locals.var_guard57 != 0.0) {
        let assign5690_e3772: f64 = (2.0 * 1.6021918e-19);
        let assign5690_e3774: f64 = (assign5690_e3772 * locals.var_uc_nsubs);
        let assign5690_e3776: f64 = (assign5690_e3774 * 1.034943e-10);
        let assign5690_e3778: f64 = (assign5690_e3776 * locals.var_pb2c);
        let assign5690_e3779: f64 = (assign5690_e3778).sqrt();
        (assign5690_e3779, (((((assign5690_e3772 * locals.var_uc_nsubs_dn0) * 1.034943e-10) * locals.var_pb2c) + (assign5690_e3776 * locals.var_pb2c_dn0)) / (2.0 * assign5690_e3779)), (((((assign5690_e3772 * locals.var_uc_nsubs_dn2) * 1.034943e-10) * locals.var_pb2c) + (assign5690_e3776 * locals.var_pb2c_dn2)) / (2.0 * assign5690_e3779)), (((((assign5690_e3772 * locals.var_uc_nsubs_dn4) * 1.034943e-10) * locals.var_pb2c) + (assign5690_e3776 * locals.var_pb2c_dn4)) / (2.0 * assign5690_e3779)), (((((assign5690_e3772 * locals.var_uc_nsubs_dn5) * 1.034943e-10) * locals.var_pb2c) + (assign5690_e3776 * locals.var_pb2c_dn5)) / (2.0 * assign5690_e3779)), (((((assign5690_e3772 * locals.var_uc_nsubs_dn6) * 1.034943e-10) * locals.var_pb2c) + (assign5690_e3776 * locals.var_pb2c_dn6)) / (2.0 * assign5690_e3779)), (((((assign5690_e3772 * locals.var_uc_nsubs_dn8) * 1.034943e-10) * locals.var_pb2c) + (assign5690_e3776 * locals.var_pb2c_dn8)) / (2.0 * assign5690_e3779)), (((((assign5690_e3772 * locals.var_uc_nsubs_dn10) * 1.034943e-10) * locals.var_pb2c) + (assign5690_e3776 * locals.var_pb2c_dn10)) / (2.0 * assign5690_e3779)), (((((assign5690_e3772 * locals.var_uc_nsubs_dn11) * 1.034943e-10) * locals.var_pb2c) + (assign5690_e3776 * locals.var_pb2c_dn11)) / (2.0 * assign5690_e3779)), (((((assign5690_e3772 * locals.var_uc_nsubs_dn12) * 1.034943e-10) * locals.var_pb2c) + (assign5690_e3776 * locals.var_pb2c_dn12)) / (2.0 * assign5690_e3779)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign5690_e3781;
        locals.var_t0_dn0 = assign5690_e3781_d_n0;
        locals.var_t0_dn2 = assign5690_e3781_d_n2;
        locals.var_t0_dn4 = assign5690_e3781_d_n4;
        locals.var_t0_dn5 = assign5690_e3781_d_n5;
        locals.var_t0_dn6 = assign5690_e3781_d_n6;
        locals.var_t0_dn8 = assign5690_e3781_d_n8;
        locals.var_t0_dn10 = assign5690_e3781_d_n10;
        locals.var_t0_dn11 = assign5690_e3781_d_n11;
        locals.var_t0_dn12 = assign5690_e3781_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign5700_e3791, assign5700_e3791_d_n0, assign5700_e3791_d_n2, assign5700_e3791_d_n4, assign5700_e3791_d_n5, assign5700_e3791_d_n6, assign5700_e3791_d_n8, assign5700_e3791_d_n10, assign5700_e3791_d_n11, assign5700_e3791_d_n12,) = {
    if (locals.var_guard57 != 0.0) {
        let assign5700_e3785: f64 = (locals.var_pb2c + locals.var_vfb);
        let assign5700_e3788: f64 = (locals.var_t0 * locals.var_c_fox_inv);
        let assign5700_e3789: f64 = (assign5700_e3785 + assign5700_e3788);
        (assign5700_e3789, ((locals.var_pb2c_dn0 + locals.var_vfb_dn0) + ((locals.var_t0_dn0 * locals.var_c_fox_inv) + (locals.var_t0 * locals.var_c_fox_inv_dn0))), ((locals.var_pb2c_dn2 + locals.var_vfb_dn2) + ((locals.var_t0_dn2 * locals.var_c_fox_inv) + (locals.var_t0 * locals.var_c_fox_inv_dn2))), ((locals.var_pb2c_dn4 + locals.var_vfb_dn4) + ((locals.var_t0_dn4 * locals.var_c_fox_inv) + (locals.var_t0 * locals.var_c_fox_inv_dn4))), ((locals.var_pb2c_dn5 + locals.var_vfb_dn5) + ((locals.var_t0_dn5 * locals.var_c_fox_inv) + (locals.var_t0 * locals.var_c_fox_inv_dn5))), ((locals.var_pb2c_dn6 + locals.var_vfb_dn6) + ((locals.var_t0_dn6 * locals.var_c_fox_inv) + (locals.var_t0 * locals.var_c_fox_inv_dn6))), ((locals.var_pb2c_dn8 + locals.var_vfb_dn8) + ((locals.var_t0_dn8 * locals.var_c_fox_inv) + (locals.var_t0 * locals.var_c_fox_inv_dn8))), ((locals.var_pb2c_dn10 + locals.var_vfb_dn10) + ((locals.var_t0_dn10 * locals.var_c_fox_inv) + (locals.var_t0 * locals.var_c_fox_inv_dn10))), ((locals.var_pb2c_dn11 + locals.var_vfb_dn11) + ((locals.var_t0_dn11 * locals.var_c_fox_inv) + (locals.var_t0 * locals.var_c_fox_inv_dn11))), ((locals.var_pb2c_dn12 + locals.var_vfb_dn12) + ((locals.var_t0_dn12 * locals.var_c_fox_inv) + (locals.var_t0 * locals.var_c_fox_inv_dn12))),)
    } else {
        (locals.var_vth0, locals.var_vth0_dn0, locals.var_vth0_dn2, locals.var_vth0_dn4, locals.var_vth0_dn5, locals.var_vth0_dn6, locals.var_vth0_dn8, locals.var_vth0_dn10, locals.var_vth0_dn11, locals.var_vth0_dn12,)
    }
};
        locals.var_vth0 = assign5700_e3791;
        locals.var_vth0_dn0 = assign5700_e3791_d_n0;
        locals.var_vth0_dn2 = assign5700_e3791_d_n2;
        locals.var_vth0_dn4 = assign5700_e3791_d_n4;
        locals.var_vth0_dn5 = assign5700_e3791_d_n5;
        locals.var_vth0_dn6 = assign5700_e3791_d_n6;
        locals.var_vth0_dn8 = assign5700_e3791_d_n8;
        locals.var_vth0_dn10 = assign5700_e3791_d_n10;
        locals.var_vth0_dn11 = assign5700_e3791_d_n11;
        locals.var_vth0_dn12 = assign5700_e3791_d_n12;
        locals.var_vth0_rv = 0.0;

        let (assign5710_e3801, assign5710_e3801_d_n0, assign5710_e3801_d_n2, assign5710_e3801_d_n4, assign5710_e3801_d_n5, assign5710_e3801_d_n6, assign5710_e3801_d_n8, assign5710_e3801_d_n10, assign5710_e3801_d_n11, assign5710_e3801_d_n12,) = {
    if (locals.var_guard57 != 0.0) {
        let assign5710_e3795: f64 = (2.0 * p.p227);
        let assign5710_e3798: f64 = (p.p58 * p.p58);
        let assign5710_e3799: f64 = (assign5710_e3795 / assign5710_e3798);
        (assign5710_e3799, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign5710_e3801;
        locals.var_t0_dn0 = assign5710_e3801_d_n0;
        locals.var_t0_dn2 = assign5710_e3801_d_n2;
        locals.var_t0_dn4 = assign5710_e3801_d_n4;
        locals.var_t0_dn5 = assign5710_e3801_d_n5;
        locals.var_t0_dn6 = assign5710_e3801_d_n6;
        locals.var_t0_dn8 = assign5710_e3801_d_n8;
        locals.var_t0_dn10 = assign5710_e3801_d_n10;
        locals.var_t0_dn11 = assign5710_e3801_d_n11;
        locals.var_t0_dn12 = assign5710_e3801_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign5720_e3813, assign5720_e3813_d_n0, assign5720_e3813_d_n2, assign5720_e3813_d_n4, assign5720_e3813_d_n5, assign5720_e3813_d_n6, assign5720_e3813_d_n8, assign5720_e3813_d_n10, assign5720_e3813_d_n11, assign5720_e3813_d_n12,) = {
    if (locals.var_guard57 != 0.0) {
        let assign5720_e3805: f64 = (1.034943e-10 * locals.var_c_fox_inv);
        let assign5720_e3807: f64 = (assign5720_e3805 * locals.var_t0);
        let assign5720_e3810: f64 = (p.p55 - locals.var_pb20b);
        let assign5720_e3811: f64 = (assign5720_e3807 * assign5720_e3810);
        (assign5720_e3811, (((((1.034943e-10 * locals.var_c_fox_inv_dn0) * locals.var_t0) + (assign5720_e3805 * locals.var_t0_dn0)) * assign5720_e3810) + (assign5720_e3807 * (-locals.var_pb20b_dn0))), (((((1.034943e-10 * locals.var_c_fox_inv_dn2) * locals.var_t0) + (assign5720_e3805 * locals.var_t0_dn2)) * assign5720_e3810) + (assign5720_e3807 * (-locals.var_pb20b_dn2))), (((((1.034943e-10 * locals.var_c_fox_inv_dn4) * locals.var_t0) + (assign5720_e3805 * locals.var_t0_dn4)) * assign5720_e3810) + (assign5720_e3807 * (-locals.var_pb20b_dn4))), (((((1.034943e-10 * locals.var_c_fox_inv_dn5) * locals.var_t0) + (assign5720_e3805 * locals.var_t0_dn5)) * assign5720_e3810) + (assign5720_e3807 * (-locals.var_pb20b_dn5))), (((((1.034943e-10 * locals.var_c_fox_inv_dn6) * locals.var_t0) + (assign5720_e3805 * locals.var_t0_dn6)) * assign5720_e3810) + (assign5720_e3807 * (-locals.var_pb20b_dn6))), (((((1.034943e-10 * locals.var_c_fox_inv_dn8) * locals.var_t0) + (assign5720_e3805 * locals.var_t0_dn8)) * assign5720_e3810) + (assign5720_e3807 * (-locals.var_pb20b_dn8))), (((((1.034943e-10 * locals.var_c_fox_inv_dn10) * locals.var_t0) + (assign5720_e3805 * locals.var_t0_dn10)) * assign5720_e3810) + (assign5720_e3807 * (-locals.var_pb20b_dn10))), (((((1.034943e-10 * locals.var_c_fox_inv_dn11) * locals.var_t0) + (assign5720_e3805 * locals.var_t0_dn11)) * assign5720_e3810) + (assign5720_e3807 * (-locals.var_pb20b_dn11))), (((((1.034943e-10 * locals.var_c_fox_inv_dn12) * locals.var_t0) + (assign5720_e3805 * locals.var_t0_dn12)) * assign5720_e3810) + (assign5720_e3807 * (-locals.var_pb20b_dn12))),)
    } else {
        (locals.var_dvth0, locals.var_dvth0_dn0, locals.var_dvth0_dn2, locals.var_dvth0_dn4, locals.var_dvth0_dn5, locals.var_dvth0_dn6, locals.var_dvth0_dn8, locals.var_dvth0_dn10, locals.var_dvth0_dn11, locals.var_dvth0_dn12,)
    }
};
        locals.var_dvth0 = assign5720_e3813;
        locals.var_dvth0_dn0 = assign5720_e3813_d_n0;
        locals.var_dvth0_dn2 = assign5720_e3813_d_n2;
        locals.var_dvth0_dn4 = assign5720_e3813_d_n4;
        locals.var_dvth0_dn5 = assign5720_e3813_d_n5;
        locals.var_dvth0_dn6 = assign5720_e3813_d_n6;
        locals.var_dvth0_dn8 = assign5720_e3813_d_n8;
        locals.var_dvth0_dn10 = assign5720_e3813_d_n10;
        locals.var_dvth0_dn11 = assign5720_e3813_d_n11;
        locals.var_dvth0_dn12 = assign5720_e3813_d_n12;
        locals.var_dvth0_rv = 0.0;

        let (assign5730_e3827, assign5730_e3827_d_n0, assign5730_e3827_d_n2, assign5730_e3827_d_n4, assign5730_e3827_d_n5, assign5730_e3827_d_n6, assign5730_e3827_d_n8, assign5730_e3827_d_n10, assign5730_e3827_d_n11, assign5730_e3827_d_n12,) = {
    if (locals.var_guard57 != 0.0) {
        let assign5730_e3818: f64 = (p.p68 / p.p58);
        let assign5730_e3820: f64 = (assign5730_e3818 * locals.var_pbsum);
        let assign5730_e3821: f64 = (p.p66 + assign5730_e3820);
        let assign5730_e3824: f64 = (p.p67 * locals.var_vdsz);
        let assign5730_e3825: f64 = (assign5730_e3821 + assign5730_e3824);
        (assign5730_e3825, ((assign5730_e3818 * locals.var_pbsum_dn0) + (p.p67 * locals.var_vdsz_dn0)), ((assign5730_e3818 * locals.var_pbsum_dn2) + (p.p67 * locals.var_vdsz_dn2)), ((assign5730_e3818 * locals.var_pbsum_dn4) + (p.p67 * locals.var_vdsz_dn4)), ((assign5730_e3818 * locals.var_pbsum_dn5) + (p.p67 * locals.var_vdsz_dn5)), ((assign5730_e3818 * locals.var_pbsum_dn6) + (p.p67 * locals.var_vdsz_dn6)), ((assign5730_e3818 * locals.var_pbsum_dn8) + (p.p67 * locals.var_vdsz_dn8)), ((assign5730_e3818 * locals.var_pbsum_dn10) + (p.p67 * locals.var_vdsz_dn10)), ((assign5730_e3818 * locals.var_pbsum_dn11) + (p.p67 * locals.var_vdsz_dn11)), ((assign5730_e3818 * locals.var_pbsum_dn12) + (p.p67 * locals.var_vdsz_dn12)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign5730_e3827;
        locals.var_t0_dn0 = assign5730_e3827_d_n0;
        locals.var_t0_dn2 = assign5730_e3827_d_n2;
        locals.var_t0_dn4 = assign5730_e3827_d_n4;
        locals.var_t0_dn5 = assign5730_e3827_d_n5;
        locals.var_t0_dn6 = assign5730_e3827_d_n6;
        locals.var_t0_dn8 = assign5730_e3827_d_n8;
        locals.var_t0_dn10 = assign5730_e3827_d_n10;
        locals.var_t0_dn11 = assign5730_e3827_d_n11;
        locals.var_t0_dn12 = assign5730_e3827_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign5740_e3837, assign5740_e3837_d_n0, assign5740_e3837_d_n2, assign5740_e3837_d_n4, assign5740_e3837_d_n5, assign5740_e3837_d_n6, assign5740_e3837_d_n8, assign5740_e3837_d_n10, assign5740_e3837_d_n11, assign5740_e3837_d_n12,) = {
    if (locals.var_guard57 != 0.0) {
        let assign5740_e3831: f64 = (locals.var_vthp - locals.var_vth0);
        let assign5740_e3833: f64 = (assign5740_e3831 * locals.var_dvth0);
        let assign5740_e3835: f64 = (assign5740_e3833 * locals.var_t0);
        (assign5740_e3835, (((((locals.var_vthp_dn0 - locals.var_vth0_dn0) * locals.var_dvth0) + (assign5740_e3831 * locals.var_dvth0_dn0)) * locals.var_t0) + (assign5740_e3833 * locals.var_t0_dn0)), (((((locals.var_vthp_dn2 - locals.var_vth0_dn2) * locals.var_dvth0) + (assign5740_e3831 * locals.var_dvth0_dn2)) * locals.var_t0) + (assign5740_e3833 * locals.var_t0_dn2)), (((((locals.var_vthp_dn4 - locals.var_vth0_dn4) * locals.var_dvth0) + (assign5740_e3831 * locals.var_dvth0_dn4)) * locals.var_t0) + (assign5740_e3833 * locals.var_t0_dn4)), (((((locals.var_vthp_dn5 - locals.var_vth0_dn5) * locals.var_dvth0) + (assign5740_e3831 * locals.var_dvth0_dn5)) * locals.var_t0) + (assign5740_e3833 * locals.var_t0_dn5)), (((((locals.var_vthp_dn6 - locals.var_vth0_dn6) * locals.var_dvth0) + (assign5740_e3831 * locals.var_dvth0_dn6)) * locals.var_t0) + (assign5740_e3833 * locals.var_t0_dn6)), (((((locals.var_vthp_dn8 - locals.var_vth0_dn8) * locals.var_dvth0) + (assign5740_e3831 * locals.var_dvth0_dn8)) * locals.var_t0) + (assign5740_e3833 * locals.var_t0_dn8)), (((((locals.var_vthp_dn10 - locals.var_vth0_dn10) * locals.var_dvth0) + (assign5740_e3831 * locals.var_dvth0_dn10)) * locals.var_t0) + (assign5740_e3833 * locals.var_t0_dn10)), (((((locals.var_vthp_dn11 - locals.var_vth0_dn11) * locals.var_dvth0) + (assign5740_e3831 * locals.var_dvth0_dn11)) * locals.var_t0) + (assign5740_e3833 * locals.var_t0_dn11)), (((((locals.var_vthp_dn12 - locals.var_vth0_dn12) * locals.var_dvth0) + (assign5740_e3831 * locals.var_dvth0_dn12)) * locals.var_t0) + (assign5740_e3833 * locals.var_t0_dn12)),)
    } else {
        (locals.var_dvthlp, locals.var_dvthlp_dn0, locals.var_dvthlp_dn2, locals.var_dvthlp_dn4, locals.var_dvthlp_dn5, locals.var_dvthlp_dn6, locals.var_dvthlp_dn8, locals.var_dvthlp_dn10, locals.var_dvthlp_dn11, locals.var_dvthlp_dn12,)
    }
};
        locals.var_dvthlp = assign5740_e3837;
        locals.var_dvthlp_dn0 = assign5740_e3837_d_n0;
        locals.var_dvthlp_dn2 = assign5740_e3837_d_n2;
        locals.var_dvthlp_dn4 = assign5740_e3837_d_n4;
        locals.var_dvthlp_dn5 = assign5740_e3837_d_n5;
        locals.var_dvthlp_dn6 = assign5740_e3837_d_n6;
        locals.var_dvthlp_dn8 = assign5740_e3837_d_n8;
        locals.var_dvthlp_dn10 = assign5740_e3837_d_n10;
        locals.var_dvthlp_dn11 = assign5740_e3837_d_n11;
        locals.var_dvthlp_dn12 = assign5740_e3837_d_n12;
        locals.var_dvthlp_rv = 0.0;

        let (assign5750_e3842, assign5750_e3842_d_n0, assign5750_e3842_d_n2, assign5750_e3842_d_n4, assign5750_e3842_d_n5, assign5750_e3842_d_n6, assign5750_e3842_d_n8, assign5750_e3842_d_n10, assign5750_e3842_d_n11, assign5750_e3842_d_n12,) = {
    if (locals.var_guard57 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dvthlp, locals.var_dvthlp_dn0, locals.var_dvthlp_dn2, locals.var_dvthlp_dn4, locals.var_dvthlp_dn5, locals.var_dvthlp_dn6, locals.var_dvthlp_dn8, locals.var_dvthlp_dn10, locals.var_dvthlp_dn11, locals.var_dvthlp_dn12,)
    }
};
        locals.var_dvthlp = assign5750_e3842;
        locals.var_dvthlp_dn0 = assign5750_e3842_d_n0;
        locals.var_dvthlp_dn2 = assign5750_e3842_d_n2;
        locals.var_dvthlp_dn4 = assign5750_e3842_d_n4;
        locals.var_dvthlp_dn5 = assign5750_e3842_d_n5;
        locals.var_dvthlp_dn6 = assign5750_e3842_d_n6;
        locals.var_dvthlp_dn8 = assign5750_e3842_d_n8;
        locals.var_dvthlp_dn10 = assign5750_e3842_d_n10;
        locals.var_dvthlp_dn11 = assign5750_e3842_d_n11;
        locals.var_dvthlp_dn12 = assign5750_e3842_d_n12;
        locals.var_dvthlp_rv = 0.0;

        let assign5760_e3845: f64 = if p.p297 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard58 = assign5760_e3845;
        locals.var_guard58_rv = 0.0;

        let (assign5770_e3859, assign5770_e3859_d_n0, assign5770_e3859_d_n2, assign5770_e3859_d_n4, assign5770_e3859_d_n5, assign5770_e3859_d_n6, assign5770_e3859_d_n8, assign5770_e3859_d_n10, assign5770_e3859_d_n11, assign5770_e3859_d_n12,) = {
    if (locals.var_guard58 != 0.0) {
        let assign5770_e3850: f64 = (locals.var_cnstc_foxi * locals.var_beta);
        let assign5770_e3852: f64 = (assign5770_e3850 * 0.25);
        let assign5770_e3853: f64 = (locals.var_beta_inv - assign5770_e3852);
        let assign5770_e3855: f64 = (assign5770_e3853 + locals.var_vfb);
        let assign5770_e3857: f64 = (assign5770_e3855 + 1e-50);
        (assign5770_e3857, ((-((locals.var_cnstc_foxi_dn0 * locals.var_beta) * 0.25)) + locals.var_vfb_dn0), ((-((locals.var_cnstc_foxi_dn2 * locals.var_beta) * 0.25)) + locals.var_vfb_dn2), ((locals.var_beta_inv_dn4 - (((locals.var_cnstc_foxi_dn4 * locals.var_beta) + (locals.var_cnstc_foxi * locals.var_beta_dn4)) * 0.25)) + locals.var_vfb_dn4), ((-((locals.var_cnstc_foxi_dn5 * locals.var_beta) * 0.25)) + locals.var_vfb_dn5), ((-((locals.var_cnstc_foxi_dn6 * locals.var_beta) * 0.25)) + locals.var_vfb_dn6), ((-((locals.var_cnstc_foxi_dn8 * locals.var_beta) * 0.25)) + locals.var_vfb_dn8), ((-((locals.var_cnstc_foxi_dn10 * locals.var_beta) * 0.25)) + locals.var_vfb_dn10), ((-((locals.var_cnstc_foxi_dn11 * locals.var_beta) * 0.25)) + locals.var_vfb_dn11), ((-((locals.var_cnstc_foxi_dn12 * locals.var_beta) * 0.25)) + locals.var_vfb_dn12),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn8, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12,)
    }
};
        locals.var_t10 = assign5770_e3859;
        locals.var_t10_dn0 = assign5770_e3859_d_n0;
        locals.var_t10_dn2 = assign5770_e3859_d_n2;
        locals.var_t10_dn4 = assign5770_e3859_d_n4;
        locals.var_t10_dn5 = assign5770_e3859_d_n5;
        locals.var_t10_dn6 = assign5770_e3859_d_n6;
        locals.var_t10_dn8 = assign5770_e3859_d_n8;
        locals.var_t10_dn10 = assign5770_e3859_d_n10;
        locals.var_t10_dn11 = assign5770_e3859_d_n11;
        locals.var_t10_dn12 = assign5770_e3859_d_n12;
        locals.var_t10_rv = 0.0;

        let (assign5780_e3867, assign5780_e3867_d_n0, assign5780_e3867_d_n2, assign5780_e3867_d_n4, assign5780_e3867_d_n5, assign5780_e3867_d_n6, assign5780_e3867_d_n8, assign5780_e3867_d_n10, assign5780_e3867_d_n11, assign5780_e3867_d_n12,) = {
    if (locals.var_guard58 != 0.0) {
        let assign5780_e3863: f64 = (locals.var_vgsz - locals.var_t10);
        let assign5780_e3865: f64 = (assign5780_e3863 - 0.005);
        (assign5780_e3865, (locals.var_vgsz_dn0 - locals.var_t10_dn0), (locals.var_vgsz_dn2 - locals.var_t10_dn2), (locals.var_vgsz_dn4 - locals.var_t10_dn4), (locals.var_vgsz_dn5 - locals.var_t10_dn5), (locals.var_vgsz_dn6 - locals.var_t10_dn6), (locals.var_vgsz_dn8 - locals.var_t10_dn8), (locals.var_vgsz_dn10 - locals.var_t10_dn10), (locals.var_vgsz_dn11 - locals.var_t10_dn11), (locals.var_vgsz_dn12 - locals.var_t10_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign5780_e3867;
        locals.var_t1_dn0 = assign5780_e3867_d_n0;
        locals.var_t1_dn2 = assign5780_e3867_d_n2;
        locals.var_t1_dn4 = assign5780_e3867_d_n4;
        locals.var_t1_dn5 = assign5780_e3867_d_n5;
        locals.var_t1_dn6 = assign5780_e3867_d_n6;
        locals.var_t1_dn8 = assign5780_e3867_d_n8;
        locals.var_t1_dn10 = assign5780_e3867_d_n10;
        locals.var_t1_dn11 = assign5780_e3867_d_n11;
        locals.var_t1_dn12 = assign5780_e3867_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign5790_e3877, assign5790_e3877_d_n0, assign5790_e3877_d_n2, assign5790_e3877_d_n4, assign5790_e3877_d_n5, assign5790_e3877_d_n6, assign5790_e3877_d_n8, assign5790_e3877_d_n10, assign5790_e3877_d_n11, assign5790_e3877_d_n12,) = {
    if (locals.var_guard58 != 0.0) {
        let (assign5790_e3875,) = {
            if (locals.var_t10 >= 0.0) {
                (1.0,)
            } else {
                let assign5790_e3874: f64 = (-1.0);
                (assign5790_e3874,)
            }
        };
        (assign5790_e3875, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign5790_e3877;
        locals.var_t0_dn0 = assign5790_e3877_d_n0;
        locals.var_t0_dn2 = assign5790_e3877_d_n2;
        locals.var_t0_dn4 = assign5790_e3877_d_n4;
        locals.var_t0_dn5 = assign5790_e3877_d_n5;
        locals.var_t0_dn6 = assign5790_e3877_d_n6;
        locals.var_t0_dn8 = assign5790_e3877_d_n8;
        locals.var_t0_dn10 = assign5790_e3877_d_n10;
        locals.var_t0_dn11 = assign5790_e3877_d_n11;
        locals.var_t0_dn12 = assign5790_e3877_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign5800_e3892, assign5800_e3892_d_n0, assign5800_e3892_d_n2, assign5800_e3892_d_n4, assign5800_e3892_d_n5, assign5800_e3892_d_n6, assign5800_e3892_d_n8, assign5800_e3892_d_n10, assign5800_e3892_d_n11, assign5800_e3892_d_n12,) = {
    if (locals.var_guard58 != 0.0) {
        let assign5800_e3881: f64 = (locals.var_t1 * locals.var_t1);
        let assign5800_e3884: f64 = (locals.var_t0 * 4.0);
        let assign5800_e3886: f64 = (assign5800_e3884 * locals.var_t10);
        let assign5800_e3888: f64 = (assign5800_e3886 * 0.005);
        let assign5800_e3889: f64 = (assign5800_e3881 + assign5800_e3888);
        let assign5800_e3890: f64 = (assign5800_e3889).sqrt();
        (assign5800_e3890, ((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) + ((((locals.var_t0_dn0 * 4.0) * locals.var_t10) + (assign5800_e3884 * locals.var_t10_dn0)) * 0.005)) / (2.0 * assign5800_e3890)), ((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) + ((((locals.var_t0_dn2 * 4.0) * locals.var_t10) + (assign5800_e3884 * locals.var_t10_dn2)) * 0.005)) / (2.0 * assign5800_e3890)), ((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + ((((locals.var_t0_dn4 * 4.0) * locals.var_t10) + (assign5800_e3884 * locals.var_t10_dn4)) * 0.005)) / (2.0 * assign5800_e3890)), ((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + ((((locals.var_t0_dn5 * 4.0) * locals.var_t10) + (assign5800_e3884 * locals.var_t10_dn5)) * 0.005)) / (2.0 * assign5800_e3890)), ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + ((((locals.var_t0_dn6 * 4.0) * locals.var_t10) + (assign5800_e3884 * locals.var_t10_dn6)) * 0.005)) / (2.0 * assign5800_e3890)), ((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + ((((locals.var_t0_dn8 * 4.0) * locals.var_t10) + (assign5800_e3884 * locals.var_t10_dn8)) * 0.005)) / (2.0 * assign5800_e3890)), ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + ((((locals.var_t0_dn10 * 4.0) * locals.var_t10) + (assign5800_e3884 * locals.var_t10_dn10)) * 0.005)) / (2.0 * assign5800_e3890)), ((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + ((((locals.var_t0_dn11 * 4.0) * locals.var_t10) + (assign5800_e3884 * locals.var_t10_dn11)) * 0.005)) / (2.0 * assign5800_e3890)), ((((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)) + ((((locals.var_t0_dn12 * 4.0) * locals.var_t10) + (assign5800_e3884 * locals.var_t10_dn12)) * 0.005)) / (2.0 * assign5800_e3890)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign5800_e3892;
        locals.var_t2_dn0 = assign5800_e3892_d_n0;
        locals.var_t2_dn2 = assign5800_e3892_d_n2;
        locals.var_t2_dn4 = assign5800_e3892_d_n4;
        locals.var_t2_dn5 = assign5800_e3892_d_n5;
        locals.var_t2_dn6 = assign5800_e3892_d_n6;
        locals.var_t2_dn8 = assign5800_e3892_d_n8;
        locals.var_t2_dn10 = assign5800_e3892_d_n10;
        locals.var_t2_dn11 = assign5800_e3892_d_n11;
        locals.var_t2_dn12 = assign5800_e3892_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign5810_e3904, assign5810_e3904_d_n0, assign5810_e3904_d_n2, assign5810_e3904_d_n4, assign5810_e3904_d_n5, assign5810_e3904_d_n6, assign5810_e3904_d_n8, assign5810_e3904_d_n10, assign5810_e3904_d_n11, assign5810_e3904_d_n12,) = {
    if (locals.var_guard58 != 0.0) {
        let assign5810_e3898: f64 = (locals.var_t1 + locals.var_t2);
        let assign5810_e3899: f64 = (0.5 * assign5810_e3898);
        let assign5810_e3900: f64 = (locals.var_t10 + assign5810_e3899);
        let assign5810_e3902: f64 = (assign5810_e3900 - locals.var_vfb);
        (assign5810_e3902, ((locals.var_t10_dn0 + (0.5 * (locals.var_t1_dn0 + locals.var_t2_dn0))) - locals.var_vfb_dn0), ((locals.var_t10_dn2 + (0.5 * (locals.var_t1_dn2 + locals.var_t2_dn2))) - locals.var_vfb_dn2), ((locals.var_t10_dn4 + (0.5 * (locals.var_t1_dn4 + locals.var_t2_dn4))) - locals.var_vfb_dn4), ((locals.var_t10_dn5 + (0.5 * (locals.var_t1_dn5 + locals.var_t2_dn5))) - locals.var_vfb_dn5), ((locals.var_t10_dn6 + (0.5 * (locals.var_t1_dn6 + locals.var_t2_dn6))) - locals.var_vfb_dn6), ((locals.var_t10_dn8 + (0.5 * (locals.var_t1_dn8 + locals.var_t2_dn8))) - locals.var_vfb_dn8), ((locals.var_t10_dn10 + (0.5 * (locals.var_t1_dn10 + locals.var_t2_dn10))) - locals.var_vfb_dn10), ((locals.var_t10_dn11 + (0.5 * (locals.var_t1_dn11 + locals.var_t2_dn11))) - locals.var_vfb_dn11), ((locals.var_t10_dn12 + (0.5 * (locals.var_t1_dn12 + locals.var_t2_dn12))) - locals.var_vfb_dn12),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign5810_e3904;
        locals.var_t3_dn0 = assign5810_e3904_d_n0;
        locals.var_t3_dn2 = assign5810_e3904_d_n2;
        locals.var_t3_dn4 = assign5810_e3904_d_n4;
        locals.var_t3_dn5 = assign5810_e3904_d_n5;
        locals.var_t3_dn6 = assign5810_e3904_d_n6;
        locals.var_t3_dn8 = assign5810_e3904_d_n8;
        locals.var_t3_dn10 = assign5810_e3904_d_n10;
        locals.var_t3_dn11 = assign5810_e3904_d_n11;
        locals.var_t3_dn12 = assign5810_e3904_d_n12;
        locals.var_t3_rv = 0.0;

        let (assign5820_e3914, assign5820_e3914_d_n0, assign5820_e3914_d_n2, assign5820_e3914_d_n4, assign5820_e3914_d_n5, assign5820_e3914_d_n6, assign5820_e3914_d_n8, assign5820_e3914_d_n10, assign5820_e3914_d_n11, assign5820_e3914_d_n12,) = {
    if (locals.var_guard58 != 0.0) {
        let assign5820_e3908: f64 = (4.0 / locals.var_cnstc_foxi);
        let assign5820_e3910: f64 = (assign5820_e3908 * locals.var_beta_inv);
        let assign5820_e3912: f64 = (assign5820_e3910 * locals.var_beta_inv);
        (assign5820_e3912, (((-((4.0 * locals.var_cnstc_foxi_dn0) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi))) * locals.var_beta_inv) * locals.var_beta_inv), (((-((4.0 * locals.var_cnstc_foxi_dn2) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi))) * locals.var_beta_inv) * locals.var_beta_inv), (((((-((4.0 * locals.var_cnstc_foxi_dn4) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi))) * locals.var_beta_inv) + (assign5820_e3908 * locals.var_beta_inv_dn4)) * locals.var_beta_inv) + (assign5820_e3910 * locals.var_beta_inv_dn4)), (((-((4.0 * locals.var_cnstc_foxi_dn5) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi))) * locals.var_beta_inv) * locals.var_beta_inv), (((-((4.0 * locals.var_cnstc_foxi_dn6) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi))) * locals.var_beta_inv) * locals.var_beta_inv), (((-((4.0 * locals.var_cnstc_foxi_dn8) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi))) * locals.var_beta_inv) * locals.var_beta_inv), (((-((4.0 * locals.var_cnstc_foxi_dn10) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi))) * locals.var_beta_inv) * locals.var_beta_inv), (((-((4.0 * locals.var_cnstc_foxi_dn11) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi))) * locals.var_beta_inv) * locals.var_beta_inv), (((-((4.0 * locals.var_cnstc_foxi_dn12) / (locals.var_cnstc_foxi * locals.var_cnstc_foxi))) * locals.var_beta_inv) * locals.var_beta_inv),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign5820_e3914;
        locals.var_t4_dn0 = assign5820_e3914_d_n0;
        locals.var_t4_dn2 = assign5820_e3914_d_n2;
        locals.var_t4_dn4 = assign5820_e3914_d_n4;
        locals.var_t4_dn5 = assign5820_e3914_d_n5;
        locals.var_t4_dn6 = assign5820_e3914_d_n6;
        locals.var_t4_dn8 = assign5820_e3914_d_n8;
        locals.var_t4_dn10 = assign5820_e3914_d_n10;
        locals.var_t4_dn11 = assign5820_e3914_d_n11;
        locals.var_t4_dn12 = assign5820_e3914_d_n12;
        locals.var_t4_rv = 0.0;

        let (assign5830_e3922, assign5830_e3922_d_n0, assign5830_e3922_d_n2, assign5830_e3922_d_n4, assign5830_e3922_d_n5, assign5830_e3922_d_n6, assign5830_e3922_d_n8, assign5830_e3922_d_n10, assign5830_e3922_d_n11, assign5830_e3922_d_n12,) = {
    if (locals.var_guard58 != 0.0) {
        let assign5830_e3918: f64 = (locals.var_beta * locals.var_t3);
        let assign5830_e3920: f64 = (assign5830_e3918 - 1.0);
        (assign5830_e3920, (locals.var_beta * locals.var_t3_dn0), (locals.var_beta * locals.var_t3_dn2), ((locals.var_beta_dn4 * locals.var_t3) + (locals.var_beta * locals.var_t3_dn4)), (locals.var_beta * locals.var_t3_dn5), (locals.var_beta * locals.var_t3_dn6), (locals.var_beta * locals.var_t3_dn8), (locals.var_beta * locals.var_t3_dn10), (locals.var_beta * locals.var_t3_dn11), (locals.var_beta * locals.var_t3_dn12),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign5830_e3922;
        locals.var_t5_dn0 = assign5830_e3922_d_n0;
        locals.var_t5_dn2 = assign5830_e3922_d_n2;
        locals.var_t5_dn4 = assign5830_e3922_d_n4;
        locals.var_t5_dn5 = assign5830_e3922_d_n5;
        locals.var_t5_dn6 = assign5830_e3922_d_n6;
        locals.var_t5_dn8 = assign5830_e3922_d_n8;
        locals.var_t5_dn10 = assign5830_e3922_d_n10;
        locals.var_t5_dn11 = assign5830_e3922_d_n11;
        locals.var_t5_dn12 = assign5830_e3922_d_n12;
        locals.var_t5_rv = 0.0;

        let (assign5840_e3930, assign5840_e3930_d_n0, assign5840_e3930_d_n2, assign5840_e3930_d_n4, assign5840_e3930_d_n5, assign5840_e3930_d_n6, assign5840_e3930_d_n8, assign5840_e3930_d_n10, assign5840_e3930_d_n11, assign5840_e3930_d_n12,) = {
    if (locals.var_guard58 != 0.0) {
        let assign5840_e3927: f64 = (locals.var_t5 * locals.var_t4);
        let assign5840_e3928: f64 = (1.0 + assign5840_e3927);
        (assign5840_e3928, ((locals.var_t5_dn0 * locals.var_t4) + (locals.var_t5 * locals.var_t4_dn0)), ((locals.var_t5_dn2 * locals.var_t4) + (locals.var_t5 * locals.var_t4_dn2)), ((locals.var_t5_dn4 * locals.var_t4) + (locals.var_t5 * locals.var_t4_dn4)), ((locals.var_t5_dn5 * locals.var_t4) + (locals.var_t5 * locals.var_t4_dn5)), ((locals.var_t5_dn6 * locals.var_t4) + (locals.var_t5 * locals.var_t4_dn6)), ((locals.var_t5_dn8 * locals.var_t4) + (locals.var_t5 * locals.var_t4_dn8)), ((locals.var_t5_dn10 * locals.var_t4) + (locals.var_t5 * locals.var_t4_dn10)), ((locals.var_t5_dn11 * locals.var_t4) + (locals.var_t5 * locals.var_t4_dn11)), ((locals.var_t5_dn12 * locals.var_t4) + (locals.var_t5 * locals.var_t4_dn12)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign5840_e3930;
        locals.var_t1_dn0 = assign5840_e3930_d_n0;
        locals.var_t1_dn2 = assign5840_e3930_d_n2;
        locals.var_t1_dn4 = assign5840_e3930_d_n4;
        locals.var_t1_dn5 = assign5840_e3930_d_n5;
        locals.var_t1_dn6 = assign5840_e3930_d_n6;
        locals.var_t1_dn8 = assign5840_e3930_d_n8;
        locals.var_t1_dn10 = assign5840_e3930_d_n10;
        locals.var_t1_dn11 = assign5840_e3930_d_n11;
        locals.var_t1_dn12 = assign5840_e3930_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign5850_e3943, assign5850_e3943_d_n0, assign5850_e3943_d_n2, assign5850_e3943_d_n4, assign5850_e3943_d_n5, assign5850_e3943_d_n6, assign5850_e3943_d_n8, assign5850_e3943_d_n10, assign5850_e3943_d_n11, assign5850_e3943_d_n12,) = {
    if (locals.var_guard58 != 0.0) {
        let assign5850_e3934: f64 = (locals.var_t1 * locals.var_t1);
        let assign5850_e3937: f64 = (4.0 * 0.001);
        let assign5850_e3939: f64 = (assign5850_e3937 * 0.001);
        let assign5850_e3940: f64 = (assign5850_e3934 + assign5850_e3939);
        let assign5850_e3941: f64 = (assign5850_e3940).sqrt();
        (assign5850_e3941, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign5850_e3941)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign5850_e3941)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign5850_e3941)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign5850_e3941)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign5850_e3941)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign5850_e3941)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign5850_e3941)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign5850_e3941)), (((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)) / (2.0 * assign5850_e3941)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign5850_e3943;
        locals.var_tmf2_dn0 = assign5850_e3943_d_n0;
        locals.var_tmf2_dn2 = assign5850_e3943_d_n2;
        locals.var_tmf2_dn4 = assign5850_e3943_d_n4;
        locals.var_tmf2_dn5 = assign5850_e3943_d_n5;
        locals.var_tmf2_dn6 = assign5850_e3943_d_n6;
        locals.var_tmf2_dn8 = assign5850_e3943_d_n8;
        locals.var_tmf2_dn10 = assign5850_e3943_d_n10;
        locals.var_tmf2_dn11 = assign5850_e3943_d_n11;
        locals.var_tmf2_dn12 = assign5850_e3943_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign5860_e3953, assign5860_e3953_d_n0, assign5860_e3953_d_n2, assign5860_e3953_d_n4, assign5860_e3953_d_n5, assign5860_e3953_d_n6, assign5860_e3953_d_n8, assign5860_e3953_d_n10, assign5860_e3953_d_n11, assign5860_e3953_d_n12,) = {
    if (locals.var_guard58 != 0.0) {
        let assign5860_e3949: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign5860_e3950: f64 = (1.0 + assign5860_e3949);
        let assign5860_e3951: f64 = (0.5 * assign5860_e3950);
        (assign5860_e3951, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn12 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn8, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12,)
    }
};
        locals.var_t7 = assign5860_e3953;
        locals.var_t7_dn0 = assign5860_e3953_d_n0;
        locals.var_t7_dn2 = assign5860_e3953_d_n2;
        locals.var_t7_dn4 = assign5860_e3953_d_n4;
        locals.var_t7_dn5 = assign5860_e3953_d_n5;
        locals.var_t7_dn6 = assign5860_e3953_d_n6;
        locals.var_t7_dn8 = assign5860_e3953_d_n8;
        locals.var_t7_dn10 = assign5860_e3953_d_n10;
        locals.var_t7_dn11 = assign5860_e3953_d_n11;
        locals.var_t7_dn12 = assign5860_e3953_d_n12;
        locals.var_t7_rv = 0.0;

        let (assign5870_e3965, assign5870_e3965_d_n0, assign5870_e3965_d_n2, assign5870_e3965_d_n4, assign5870_e3965_d_n5, assign5870_e3965_d_n6, assign5870_e3965_d_n8, assign5870_e3965_d_n10, assign5870_e3965_d_n11, assign5870_e3965_d_n12,) = {
    if (locals.var_guard58 != 0.0) {
        let assign5870_e3958: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign5870_e3959: f64 = (0.5 * assign5870_e3958);
        let assign5870_e3962: f64 = (1e-10 * 0.001);
        let assign5870_e3963: f64 = (assign5870_e3959 + assign5870_e3962);
        (assign5870_e3963, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn12 + locals.var_tmf2_dn12)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign5870_e3965;
        locals.var_t1_dn0 = assign5870_e3965_d_n0;
        locals.var_t1_dn2 = assign5870_e3965_d_n2;
        locals.var_t1_dn4 = assign5870_e3965_d_n4;
        locals.var_t1_dn5 = assign5870_e3965_d_n5;
        locals.var_t1_dn6 = assign5870_e3965_d_n6;
        locals.var_t1_dn8 = assign5870_e3965_d_n8;
        locals.var_t1_dn10 = assign5870_e3965_d_n10;
        locals.var_t1_dn11 = assign5870_e3965_d_n11;
        locals.var_t1_dn12 = assign5870_e3965_d_n12;
        locals.var_t1_rv = 0.0;

        let assign5880_e3968: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard59 = assign5880_e3968;
        locals.var_guard59_rv = 0.0;

        let (assign5890_e3974, assign5890_e3974_d_n0, assign5890_e3974_d_n2, assign5890_e3974_d_n4, assign5890_e3974_d_n5, assign5890_e3974_d_n6, assign5890_e3974_d_n8, assign5890_e3974_d_n10, assign5890_e3974_d_n11, assign5890_e3974_d_n12,) = {
    if ((locals.var_guard58 != 0.0) && (locals.var_guard59 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign5890_e3974;
        locals.var_t1_dn0 = assign5890_e3974_d_n0;
        locals.var_t1_dn2 = assign5890_e3974_d_n2;
        locals.var_t1_dn4 = assign5890_e3974_d_n4;
        locals.var_t1_dn5 = assign5890_e3974_d_n5;
        locals.var_t1_dn6 = assign5890_e3974_d_n6;
        locals.var_t1_dn8 = assign5890_e3974_d_n8;
        locals.var_t1_dn10 = assign5890_e3974_d_n10;
        locals.var_t1_dn11 = assign5890_e3974_d_n11;
        locals.var_t1_dn12 = assign5890_e3974_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign5900_e3980, assign5900_e3980_d_n0, assign5900_e3980_d_n2, assign5900_e3980_d_n4, assign5900_e3980_d_n5, assign5900_e3980_d_n6, assign5900_e3980_d_n8, assign5900_e3980_d_n10, assign5900_e3980_d_n11, assign5900_e3980_d_n12,) = {
    if ((locals.var_guard58 != 0.0) && (locals.var_guard59 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn8, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12,)
    }
};
        locals.var_t7 = assign5900_e3980;
        locals.var_t7_dn0 = assign5900_e3980_d_n0;
        locals.var_t7_dn2 = assign5900_e3980_d_n2;
        locals.var_t7_dn4 = assign5900_e3980_d_n4;
        locals.var_t7_dn5 = assign5900_e3980_d_n5;
        locals.var_t7_dn6 = assign5900_e3980_d_n6;
        locals.var_t7_dn8 = assign5900_e3980_d_n8;
        locals.var_t7_dn10 = assign5900_e3980_d_n10;
        locals.var_t7_dn11 = assign5900_e3980_d_n11;
        locals.var_t7_dn12 = assign5900_e3980_d_n12;
        locals.var_t7_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign5910_e3989, assign5910_e3989_d_n0, assign5910_e3989_d_n2, assign5910_e3989_d_n4, assign5910_e3989_d_n5, assign5910_e3989_d_n6, assign5910_e3989_d_n8, assign5910_e3989_d_n10, assign5910_e3989_d_n11, assign5910_e3989_d_n12,) = {
    if (locals.var_guard58 != 0.0) {
        let assign5910_e3985: f64 = (10.0 * 2.220446049250313e-16);
        let assign5910_e3986: f64 = (locals.var_t1 + assign5910_e3985);
        let assign5910_e3987: f64 = (assign5910_e3986).sqrt();
        (assign5910_e3987, (locals.var_t1_dn0 / (2.0 * assign5910_e3987)), (locals.var_t1_dn2 / (2.0 * assign5910_e3987)), (locals.var_t1_dn4 / (2.0 * assign5910_e3987)), (locals.var_t1_dn5 / (2.0 * assign5910_e3987)), (locals.var_t1_dn6 / (2.0 * assign5910_e3987)), (locals.var_t1_dn8 / (2.0 * assign5910_e3987)), (locals.var_t1_dn10 / (2.0 * assign5910_e3987)), (locals.var_t1_dn11 / (2.0 * assign5910_e3987)), (locals.var_t1_dn12 / (2.0 * assign5910_e3987)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign5910_e3989;
        locals.var_t2_dn0 = assign5910_e3989_d_n0;
        locals.var_t2_dn2 = assign5910_e3989_d_n2;
        locals.var_t2_dn4 = assign5910_e3989_d_n4;
        locals.var_t2_dn5 = assign5910_e3989_d_n5;
        locals.var_t2_dn6 = assign5910_e3989_d_n6;
        locals.var_t2_dn8 = assign5910_e3989_d_n8;
        locals.var_t2_dn10 = assign5910_e3989_d_n10;
        locals.var_t2_dn11 = assign5910_e3989_d_n11;
        locals.var_t2_dn12 = assign5910_e3989_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign5920_e4003, assign5920_e4003_d_n0, assign5920_e4003_d_n2, assign5920_e4003_d_n4, assign5920_e4003_d_n5, assign5920_e4003_d_n6, assign5920_e4003_d_n8, assign5920_e4003_d_n10, assign5920_e4003_d_n11, assign5920_e4003_d_n12,) = {
    if (locals.var_guard58 != 0.0) {
        let assign5920_e3994: f64 = (locals.var_cnstc_foxi * 0.5);
        let assign5920_e3996: f64 = (assign5920_e3994 * locals.var_beta);
        let assign5920_e3999: f64 = (1.0 - locals.var_t2);
        let assign5920_e4000: f64 = (assign5920_e3996 * assign5920_e3999);
        let assign5920_e4001: f64 = (locals.var_t3 + assign5920_e4000);
        (assign5920_e4001, (locals.var_t3_dn0 + ((((locals.var_cnstc_foxi_dn0 * 0.5) * locals.var_beta) * assign5920_e3999) + (assign5920_e3996 * (-locals.var_t2_dn0)))), (locals.var_t3_dn2 + ((((locals.var_cnstc_foxi_dn2 * 0.5) * locals.var_beta) * assign5920_e3999) + (assign5920_e3996 * (-locals.var_t2_dn2)))), (locals.var_t3_dn4 + (((((locals.var_cnstc_foxi_dn4 * 0.5) * locals.var_beta) + (assign5920_e3994 * locals.var_beta_dn4)) * assign5920_e3999) + (assign5920_e3996 * (-locals.var_t2_dn4)))), (locals.var_t3_dn5 + ((((locals.var_cnstc_foxi_dn5 * 0.5) * locals.var_beta) * assign5920_e3999) + (assign5920_e3996 * (-locals.var_t2_dn5)))), (locals.var_t3_dn6 + ((((locals.var_cnstc_foxi_dn6 * 0.5) * locals.var_beta) * assign5920_e3999) + (assign5920_e3996 * (-locals.var_t2_dn6)))), (locals.var_t3_dn8 + ((((locals.var_cnstc_foxi_dn8 * 0.5) * locals.var_beta) * assign5920_e3999) + (assign5920_e3996 * (-locals.var_t2_dn8)))), (locals.var_t3_dn10 + ((((locals.var_cnstc_foxi_dn10 * 0.5) * locals.var_beta) * assign5920_e3999) + (assign5920_e3996 * (-locals.var_t2_dn10)))), (locals.var_t3_dn11 + ((((locals.var_cnstc_foxi_dn11 * 0.5) * locals.var_beta) * assign5920_e3999) + (assign5920_e3996 * (-locals.var_t2_dn11)))), (locals.var_t3_dn12 + ((((locals.var_cnstc_foxi_dn12 * 0.5) * locals.var_beta) * assign5920_e3999) + (assign5920_e3996 * (-locals.var_t2_dn12)))),)
    } else {
        (locals.var_psi_a, locals.var_psi_a_dn0, locals.var_psi_a_dn2, locals.var_psi_a_dn4, locals.var_psi_a_dn5, locals.var_psi_a_dn6, locals.var_psi_a_dn8, locals.var_psi_a_dn10, locals.var_psi_a_dn11, locals.var_psi_a_dn12,)
    }
};
        locals.var_psi_a = assign5920_e4003;
        locals.var_psi_a_dn0 = assign5920_e4003_d_n0;
        locals.var_psi_a_dn2 = assign5920_e4003_d_n2;
        locals.var_psi_a_dn4 = assign5920_e4003_d_n4;
        locals.var_psi_a_dn5 = assign5920_e4003_d_n5;
        locals.var_psi_a_dn6 = assign5920_e4003_d_n6;
        locals.var_psi_a_dn8 = assign5920_e4003_d_n8;
        locals.var_psi_a_dn10 = assign5920_e4003_d_n10;
        locals.var_psi_a_dn11 = assign5920_e4003_d_n11;
        locals.var_psi_a_dn12 = assign5920_e4003_d_n12;
        locals.var_psi_a_rv = 0.0;

        let (assign5930_e4011, assign5930_e4011_d_n0, assign5930_e4011_d_n2, assign5930_e4011_d_n4, assign5930_e4011_d_n5, assign5930_e4011_d_n6, assign5930_e4011_d_n8, assign5930_e4011_d_n10, assign5930_e4011_d_n11, assign5930_e4011_d_n12,) = {
    if (locals.var_guard58 != 0.0) {
        let assign5930_e4007: f64 = (locals.var_pb20 - locals.var_psi_a);
        let assign5930_e4009: f64 = (assign5930_e4007 - 0.005);
        (assign5930_e4009, (locals.var_pb20_dn0 - locals.var_psi_a_dn0), (locals.var_pb20_dn2 - locals.var_psi_a_dn2), (locals.var_pb20_dn4 - locals.var_psi_a_dn4), (locals.var_pb20_dn5 - locals.var_psi_a_dn5), (locals.var_pb20_dn6 - locals.var_psi_a_dn6), (locals.var_pb20_dn8 - locals.var_psi_a_dn8), (locals.var_pb20_dn10 - locals.var_psi_a_dn10), (locals.var_pb20_dn11 - locals.var_psi_a_dn11), (locals.var_pb20_dn12 - locals.var_psi_a_dn12),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn8, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12,)
    }
};
        locals.var_tmf1 = assign5930_e4011;
        locals.var_tmf1_dn0 = assign5930_e4011_d_n0;
        locals.var_tmf1_dn2 = assign5930_e4011_d_n2;
        locals.var_tmf1_dn4 = assign5930_e4011_d_n4;
        locals.var_tmf1_dn5 = assign5930_e4011_d_n5;
        locals.var_tmf1_dn6 = assign5930_e4011_d_n6;
        locals.var_tmf1_dn8 = assign5930_e4011_d_n8;
        locals.var_tmf1_dn10 = assign5930_e4011_d_n10;
        locals.var_tmf1_dn11 = assign5930_e4011_d_n11;
        locals.var_tmf1_dn12 = assign5930_e4011_d_n12;
        locals.var_tmf1_rv = 0.0;

        let (assign5940_e4019, assign5940_e4019_d_n0, assign5940_e4019_d_n2, assign5940_e4019_d_n4, assign5940_e4019_d_n5, assign5940_e4019_d_n6, assign5940_e4019_d_n8, assign5940_e4019_d_n10, assign5940_e4019_d_n11, assign5940_e4019_d_n12,) = {
    if (locals.var_guard58 != 0.0) {
        let assign5940_e4015: f64 = (4.0 * locals.var_pb20);
        let assign5940_e4017: f64 = (assign5940_e4015 * 0.005);
        (assign5940_e4017, ((4.0 * locals.var_pb20_dn0) * 0.005), ((4.0 * locals.var_pb20_dn2) * 0.005), ((4.0 * locals.var_pb20_dn4) * 0.005), ((4.0 * locals.var_pb20_dn5) * 0.005), ((4.0 * locals.var_pb20_dn6) * 0.005), ((4.0 * locals.var_pb20_dn8) * 0.005), ((4.0 * locals.var_pb20_dn10) * 0.005), ((4.0 * locals.var_pb20_dn11) * 0.005), ((4.0 * locals.var_pb20_dn12) * 0.005),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign5940_e4019;
        locals.var_tmf2_dn0 = assign5940_e4019_d_n0;
        locals.var_tmf2_dn2 = assign5940_e4019_d_n2;
        locals.var_tmf2_dn4 = assign5940_e4019_d_n4;
        locals.var_tmf2_dn5 = assign5940_e4019_d_n5;
        locals.var_tmf2_dn6 = assign5940_e4019_d_n6;
        locals.var_tmf2_dn8 = assign5940_e4019_d_n8;
        locals.var_tmf2_dn10 = assign5940_e4019_d_n10;
        locals.var_tmf2_dn11 = assign5940_e4019_d_n11;
        locals.var_tmf2_dn12 = assign5940_e4019_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign5950_e4029, assign5950_e4029_d_n0, assign5950_e4029_d_n2, assign5950_e4029_d_n4, assign5950_e4029_d_n5, assign5950_e4029_d_n6, assign5950_e4029_d_n8, assign5950_e4029_d_n10, assign5950_e4029_d_n11, assign5950_e4029_d_n12,) = {
    if (locals.var_guard58 != 0.0) {
        let (assign5950_e4027, assign5950_e4027_d_n0, assign5950_e4027_d_n2, assign5950_e4027_d_n4, assign5950_e4027_d_n5, assign5950_e4027_d_n6, assign5950_e4027_d_n8, assign5950_e4027_d_n10, assign5950_e4027_d_n11, assign5950_e4027_d_n12,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
            } else {
                let assign5950_e4026: f64 = (-locals.var_tmf2);
                (assign5950_e4026, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12),)
            }
        };
        (assign5950_e4027, assign5950_e4027_d_n0, assign5950_e4027_d_n2, assign5950_e4027_d_n4, assign5950_e4027_d_n5, assign5950_e4027_d_n6, assign5950_e4027_d_n8, assign5950_e4027_d_n10, assign5950_e4027_d_n11, assign5950_e4027_d_n12,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign5950_e4029;
        locals.var_tmf2_dn0 = assign5950_e4029_d_n0;
        locals.var_tmf2_dn2 = assign5950_e4029_d_n2;
        locals.var_tmf2_dn4 = assign5950_e4029_d_n4;
        locals.var_tmf2_dn5 = assign5950_e4029_d_n5;
        locals.var_tmf2_dn6 = assign5950_e4029_d_n6;
        locals.var_tmf2_dn8 = assign5950_e4029_d_n8;
        locals.var_tmf2_dn10 = assign5950_e4029_d_n10;
        locals.var_tmf2_dn11 = assign5950_e4029_d_n11;
        locals.var_tmf2_dn12 = assign5950_e4029_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign5960_e4038, assign5960_e4038_d_n0, assign5960_e4038_d_n2, assign5960_e4038_d_n4, assign5960_e4038_d_n5, assign5960_e4038_d_n6, assign5960_e4038_d_n8, assign5960_e4038_d_n10, assign5960_e4038_d_n11, assign5960_e4038_d_n12,) = {
    if (locals.var_guard58 != 0.0) {
        let assign5960_e4033: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign5960_e4035: f64 = (assign5960_e4033 + locals.var_tmf2);
        let assign5960_e4036: f64 = (assign5960_e4035).sqrt();
        (assign5960_e4036, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign5960_e4036)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign5960_e4036)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign5960_e4036)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign5960_e4036)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign5960_e4036)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign5960_e4036)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign5960_e4036)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign5960_e4036)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign5960_e4036)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign5960_e4038;
        locals.var_tmf2_dn0 = assign5960_e4038_d_n0;
        locals.var_tmf2_dn2 = assign5960_e4038_d_n2;
        locals.var_tmf2_dn4 = assign5960_e4038_d_n4;
        locals.var_tmf2_dn5 = assign5960_e4038_d_n5;
        locals.var_tmf2_dn6 = assign5960_e4038_d_n6;
        locals.var_tmf2_dn8 = assign5960_e4038_d_n8;
        locals.var_tmf2_dn10 = assign5960_e4038_d_n10;
        locals.var_tmf2_dn11 = assign5960_e4038_d_n11;
        locals.var_tmf2_dn12 = assign5960_e4038_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign5970_e4048, assign5970_e4048_d_n0, assign5970_e4048_d_n2, assign5970_e4048_d_n4, assign5970_e4048_d_n5, assign5970_e4048_d_n6, assign5970_e4048_d_n8, assign5970_e4048_d_n10, assign5970_e4048_d_n11, assign5970_e4048_d_n12,) = {
    if (locals.var_guard58 != 0.0) {
        let assign5970_e4044: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign5970_e4045: f64 = (1.0 + assign5970_e4044);
        let assign5970_e4046: f64 = (0.5 * assign5970_e4045);
        (assign5970_e4046, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign5970_e4048;
        locals.var_t2_dn0 = assign5970_e4048_d_n0;
        locals.var_t2_dn2 = assign5970_e4048_d_n2;
        locals.var_t2_dn4 = assign5970_e4048_d_n4;
        locals.var_t2_dn5 = assign5970_e4048_d_n5;
        locals.var_t2_dn6 = assign5970_e4048_d_n6;
        locals.var_t2_dn8 = assign5970_e4048_d_n8;
        locals.var_t2_dn10 = assign5970_e4048_d_n10;
        locals.var_t2_dn11 = assign5970_e4048_d_n11;
        locals.var_t2_dn12 = assign5970_e4048_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign5980_e4058, assign5980_e4058_d_n0, assign5980_e4058_d_n2, assign5980_e4058_d_n4, assign5980_e4058_d_n5, assign5980_e4058_d_n6, assign5980_e4058_d_n8, assign5980_e4058_d_n10, assign5980_e4058_d_n11, assign5980_e4058_d_n12,) = {
    if (locals.var_guard58 != 0.0) {
        let assign5980_e4054: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign5980_e4055: f64 = (0.5 * assign5980_e4054);
        let assign5980_e4056: f64 = (locals.var_pb20 - assign5980_e4055);
        (assign5980_e4056, (locals.var_pb20_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_pb20_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_pb20_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_pb20_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_pb20_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_pb20_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_pb20_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_pb20_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_pb20_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))),)
    } else {
        (locals.var_pb20a, locals.var_pb20a_dn0, locals.var_pb20a_dn2, locals.var_pb20a_dn4, locals.var_pb20a_dn5, locals.var_pb20a_dn6, locals.var_pb20a_dn8, locals.var_pb20a_dn10, locals.var_pb20a_dn11, locals.var_pb20a_dn12,)
    }
};
        locals.var_pb20a = assign5980_e4058;
        locals.var_pb20a_dn0 = assign5980_e4058_d_n0;
        locals.var_pb20a_dn2 = assign5980_e4058_d_n2;
        locals.var_pb20a_dn4 = assign5980_e4058_d_n4;
        locals.var_pb20a_dn5 = assign5980_e4058_d_n5;
        locals.var_pb20a_dn6 = assign5980_e4058_d_n6;
        locals.var_pb20a_dn8 = assign5980_e4058_d_n8;
        locals.var_pb20a_dn10 = assign5980_e4058_d_n10;
        locals.var_pb20a_dn11 = assign5980_e4058_d_n11;
        locals.var_pb20a_dn12 = assign5980_e4058_d_n12;
        locals.var_pb20a_rv = 0.0;

        let (assign5990_e4068, assign5990_e4068_d_n0, assign5990_e4068_d_n2, assign5990_e4068_d_n4, assign5990_e4068_d_n5, assign5990_e4068_d_n6, assign5990_e4068_d_n8, assign5990_e4068_d_n10, assign5990_e4068_d_n11, assign5990_e4068_d_n12,) = {
    if (locals.var_guard58 != 0.0) {
        let assign5990_e4064: f64 = (locals.var_pb20a - locals.var_pb20);
        let assign5990_e4065: f64 = (p.p297 * assign5990_e4064);
        let assign5990_e4066: f64 = (locals.var_pb20 + assign5990_e4065);
        (assign5990_e4066, (locals.var_pb20_dn0 + (p.p297 * (locals.var_pb20a_dn0 - locals.var_pb20_dn0))), (locals.var_pb20_dn2 + (p.p297 * (locals.var_pb20a_dn2 - locals.var_pb20_dn2))), (locals.var_pb20_dn4 + (p.p297 * (locals.var_pb20a_dn4 - locals.var_pb20_dn4))), (locals.var_pb20_dn5 + (p.p297 * (locals.var_pb20a_dn5 - locals.var_pb20_dn5))), (locals.var_pb20_dn6 + (p.p297 * (locals.var_pb20a_dn6 - locals.var_pb20_dn6))), (locals.var_pb20_dn8 + (p.p297 * (locals.var_pb20a_dn8 - locals.var_pb20_dn8))), (locals.var_pb20_dn10 + (p.p297 * (locals.var_pb20a_dn10 - locals.var_pb20_dn10))), (locals.var_pb20_dn11 + (p.p297 * (locals.var_pb20a_dn11 - locals.var_pb20_dn11))), (locals.var_pb20_dn12 + (p.p297 * (locals.var_pb20a_dn12 - locals.var_pb20_dn12))),)
    } else {
        (locals.var_pb20b, locals.var_pb20b_dn0, locals.var_pb20b_dn2, locals.var_pb20b_dn4, locals.var_pb20b_dn5, locals.var_pb20b_dn6, locals.var_pb20b_dn8, locals.var_pb20b_dn10, locals.var_pb20b_dn11, locals.var_pb20b_dn12,)
    }
};
        locals.var_pb20b = assign5990_e4068;
        locals.var_pb20b_dn0 = assign5990_e4068_d_n0;
        locals.var_pb20b_dn2 = assign5990_e4068_d_n2;
        locals.var_pb20b_dn4 = assign5990_e4068_d_n4;
        locals.var_pb20b_dn5 = assign5990_e4068_d_n5;
        locals.var_pb20b_dn6 = assign5990_e4068_d_n6;
        locals.var_pb20b_dn8 = assign5990_e4068_d_n8;
        locals.var_pb20b_dn10 = assign5990_e4068_d_n10;
        locals.var_pb20b_dn11 = assign5990_e4068_d_n11;
        locals.var_pb20b_dn12 = assign5990_e4068_d_n12;
        locals.var_pb20b_rv = 0.0;

        let assign6000_e4071: f64 = (locals.var_c_fox_inv * 1.034943e-10);
        let assign6000_e4073: f64 = (assign6000_e4071 * p.p227);
        let assign6000_e4075: f64 = (assign6000_e4073 * 2.0);
        locals.var_t1 = assign6000_e4075;
        locals.var_t1_dn0 = (((locals.var_c_fox_inv_dn0 * 1.034943e-10) * p.p227) * 2.0);
        locals.var_t1_dn2 = (((locals.var_c_fox_inv_dn2 * 1.034943e-10) * p.p227) * 2.0);
        locals.var_t1_dn4 = (((locals.var_c_fox_inv_dn4 * 1.034943e-10) * p.p227) * 2.0);
        locals.var_t1_dn5 = (((locals.var_c_fox_inv_dn5 * 1.034943e-10) * p.p227) * 2.0);
        locals.var_t1_dn6 = (((locals.var_c_fox_inv_dn6 * 1.034943e-10) * p.p227) * 2.0);
        locals.var_t1_dn8 = (((locals.var_c_fox_inv_dn8 * 1.034943e-10) * p.p227) * 2.0);
        locals.var_t1_dn10 = (((locals.var_c_fox_inv_dn10 * 1.034943e-10) * p.p227) * 2.0);
        locals.var_t1_dn11 = (((locals.var_c_fox_inv_dn11 * 1.034943e-10) * p.p227) * 2.0);
        locals.var_t1_dn12 = (((locals.var_c_fox_inv_dn12 * 1.034943e-10) * p.p227) * 2.0);
        locals.var_t1_rv = 0.0;

        let assign6010_e4078: f64 = (p.p55 - locals.var_pb20b);
        locals.var_t2 = assign6010_e4078;
        locals.var_t2_dn0 = (-locals.var_pb20b_dn0);
        locals.var_t2_dn2 = (-locals.var_pb20b_dn2);
        locals.var_t2_dn4 = (-locals.var_pb20b_dn4);
        locals.var_t2_dn5 = (-locals.var_pb20b_dn5);
        locals.var_t2_dn6 = (-locals.var_pb20b_dn6);
        locals.var_t2_dn8 = (-locals.var_pb20b_dn8);
        locals.var_t2_dn10 = (-locals.var_pb20b_dn10);
        locals.var_t2_dn11 = (-locals.var_pb20b_dn11);
        locals.var_t2_dn12 = (-locals.var_pb20b_dn12);
        locals.var_t2_rv = 0.0;

        let assign6020_e4081: f64 = (locals.var_lgate - p.p57);
        locals.var_t3 = assign6020_e4081;
        locals.var_t3_dn0 = 0.0;
        locals.var_t3_dn2 = 0.0;
        locals.var_t3_dn4 = 0.0;
        locals.var_t3_dn5 = 0.0;
        locals.var_t3_dn6 = 0.0;
        locals.var_t3_dn8 = 0.0;
        locals.var_t3_dn10 = 0.0;
        locals.var_t3_dn11 = 0.0;
        locals.var_t3_dn12 = 0.0;
        locals.var_t3_rv = 0.0;

        let assign6030_e4084: f64 = (locals.var_t1 * locals.var_t2);
        let assign6030_e4087: f64 = (locals.var_t3 * locals.var_t3);
        let assign6030_e4088: f64 = (assign6030_e4084 / assign6030_e4087);
        locals.var_dvth0 = assign6030_e4088;
        locals.var_dvth0_dn0 = (((((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0)) * assign6030_e4087) - (assign6030_e4084 * ((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)))) / (assign6030_e4087 * assign6030_e4087));
        locals.var_dvth0_dn2 = (((((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2)) * assign6030_e4087) - (assign6030_e4084 * ((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)))) / (assign6030_e4087 * assign6030_e4087));
        locals.var_dvth0_dn4 = (((((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)) * assign6030_e4087) - (assign6030_e4084 * ((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)))) / (assign6030_e4087 * assign6030_e4087));
        locals.var_dvth0_dn5 = (((((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)) * assign6030_e4087) - (assign6030_e4084 * ((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)))) / (assign6030_e4087 * assign6030_e4087));
        locals.var_dvth0_dn6 = (((((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)) * assign6030_e4087) - (assign6030_e4084 * ((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)))) / (assign6030_e4087 * assign6030_e4087));
        locals.var_dvth0_dn8 = (((((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)) * assign6030_e4087) - (assign6030_e4084 * ((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)))) / (assign6030_e4087 * assign6030_e4087));
        locals.var_dvth0_dn10 = (((((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)) * assign6030_e4087) - (assign6030_e4084 * ((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)))) / (assign6030_e4087 * assign6030_e4087));
        locals.var_dvth0_dn11 = (((((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)) * assign6030_e4087) - (assign6030_e4084 * ((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11)))) / (assign6030_e4087 * assign6030_e4087));
        locals.var_dvth0_dn12 = (((((locals.var_t1_dn12 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn12)) * assign6030_e4087) - (assign6030_e4084 * ((locals.var_t3_dn12 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn12)))) / (assign6030_e4087 * assign6030_e4087));
        locals.var_dvth0_rv = 0.0;

        let assign6040_e4091: f64 = (locals.var_vbs * locals.var_vbs);
        let assign6040_e4094: f64 = (4.0 * 0.001);
        let assign6040_e4096: f64 = (assign6040_e4094 * 0.001);
        let assign6040_e4097: f64 = (assign6040_e4091 + assign6040_e4096);
        let assign6040_e4098: f64 = (assign6040_e4097).sqrt();
        locals.var_tmf2 = assign6040_e4098;
        locals.var_tmf2_dn0 = (((locals.var_vbs_dn0 * locals.var_vbs) + (locals.var_vbs * locals.var_vbs_dn0)) / (2.0 * assign6040_e4098));
        locals.var_tmf2_dn2 = (((locals.var_vbs_dn2 * locals.var_vbs) + (locals.var_vbs * locals.var_vbs_dn2)) / (2.0 * assign6040_e4098));
        locals.var_tmf2_dn4 = (((locals.var_vbs_dn4 * locals.var_vbs) + (locals.var_vbs * locals.var_vbs_dn4)) / (2.0 * assign6040_e4098));
        locals.var_tmf2_dn5 = (((locals.var_vbs_dn5 * locals.var_vbs) + (locals.var_vbs * locals.var_vbs_dn5)) / (2.0 * assign6040_e4098));
        locals.var_tmf2_dn6 = (((locals.var_vbs_dn6 * locals.var_vbs) + (locals.var_vbs * locals.var_vbs_dn6)) / (2.0 * assign6040_e4098));
        locals.var_tmf2_dn8 = (((locals.var_vbs_dn8 * locals.var_vbs) + (locals.var_vbs * locals.var_vbs_dn8)) / (2.0 * assign6040_e4098));
        locals.var_tmf2_dn10 = (((locals.var_vbs_dn10 * locals.var_vbs) + (locals.var_vbs * locals.var_vbs_dn10)) / (2.0 * assign6040_e4098));
        locals.var_tmf2_dn11 = (((locals.var_vbs_dn11 * locals.var_vbs) + (locals.var_vbs * locals.var_vbs_dn11)) / (2.0 * assign6040_e4098));
        locals.var_tmf2_dn12 = (((locals.var_vbs_dn12 * locals.var_vbs) + (locals.var_vbs * locals.var_vbs_dn12)) / (2.0 * assign6040_e4098));
        locals.var_tmf2_rv = 0.0;

        let assign6050_e4103: f64 = (locals.var_vbs / locals.var_tmf2);
        let assign6050_e4104: f64 = (1.0 + assign6050_e4103);
        let assign6050_e4105: f64 = (0.5 * assign6050_e4104);
        locals.var_t0 = assign6050_e4105;
        locals.var_t0_dn0 = (0.5 * (((locals.var_vbs_dn0 * locals.var_tmf2) - (locals.var_vbs * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn2 = (0.5 * (((locals.var_vbs_dn2 * locals.var_tmf2) - (locals.var_vbs * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn4 = (0.5 * (((locals.var_vbs_dn4 * locals.var_tmf2) - (locals.var_vbs * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn5 = (0.5 * (((locals.var_vbs_dn5 * locals.var_tmf2) - (locals.var_vbs * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn6 = (0.5 * (((locals.var_vbs_dn6 * locals.var_tmf2) - (locals.var_vbs * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn8 = (0.5 * (((locals.var_vbs_dn8 * locals.var_tmf2) - (locals.var_vbs * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn10 = (0.5 * (((locals.var_vbs_dn10 * locals.var_tmf2) - (locals.var_vbs * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn11 = (0.5 * (((locals.var_vbs_dn11 * locals.var_tmf2) - (locals.var_vbs * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn12 = (0.5 * (((locals.var_vbs_dn12 * locals.var_tmf2) - (locals.var_vbs * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_rv = 0.0;

        let assign6060_e4109: f64 = (locals.var_vbs + locals.var_tmf2);
        let assign6060_e4110: f64 = (0.5 * assign6060_e4109);
        let assign6060_e4113: f64 = (1e-10 * 0.001);
        let assign6060_e4114: f64 = (assign6060_e4110 + assign6060_e4113);
        locals.var_vbs_prime = assign6060_e4114;
        locals.var_vbs_prime_dn0 = (0.5 * (locals.var_vbs_dn0 + locals.var_tmf2_dn0));
        locals.var_vbs_prime_dn2 = (0.5 * (locals.var_vbs_dn2 + locals.var_tmf2_dn2));
        locals.var_vbs_prime_dn4 = (0.5 * (locals.var_vbs_dn4 + locals.var_tmf2_dn4));
        locals.var_vbs_prime_dn5 = (0.5 * (locals.var_vbs_dn5 + locals.var_tmf2_dn5));
        locals.var_vbs_prime_dn6 = (0.5 * (locals.var_vbs_dn6 + locals.var_tmf2_dn6));
        locals.var_vbs_prime_dn8 = (0.5 * (locals.var_vbs_dn8 + locals.var_tmf2_dn8));
        locals.var_vbs_prime_dn10 = (0.5 * (locals.var_vbs_dn10 + locals.var_tmf2_dn10));
        locals.var_vbs_prime_dn11 = (0.5 * (locals.var_vbs_dn11 + locals.var_tmf2_dn11));
        locals.var_vbs_prime_dn12 = (0.5 * (locals.var_vbs_dn12 + locals.var_tmf2_dn12));
        locals.var_vbs_prime_rv = 0.0;

        let assign6070_e4117: f64 = if locals.var_vbs_prime < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard60 = assign6070_e4117;
        locals.var_guard60_rv = 0.0;

        let (assign6080_e4121, assign6080_e4121_d_n0, assign6080_e4121_d_n2, assign6080_e4121_d_n4, assign6080_e4121_d_n5, assign6080_e4121_d_n6, assign6080_e4121_d_n8, assign6080_e4121_d_n10, assign6080_e4121_d_n11, assign6080_e4121_d_n12,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_prime, locals.var_vbs_prime_dn0, locals.var_vbs_prime_dn2, locals.var_vbs_prime_dn4, locals.var_vbs_prime_dn5, locals.var_vbs_prime_dn6, locals.var_vbs_prime_dn8, locals.var_vbs_prime_dn10, locals.var_vbs_prime_dn11, locals.var_vbs_prime_dn12,)
    }
};
        locals.var_vbs_prime = assign6080_e4121;
        locals.var_vbs_prime_dn0 = assign6080_e4121_d_n0;
        locals.var_vbs_prime_dn2 = assign6080_e4121_d_n2;
        locals.var_vbs_prime_dn4 = assign6080_e4121_d_n4;
        locals.var_vbs_prime_dn5 = assign6080_e4121_d_n5;
        locals.var_vbs_prime_dn6 = assign6080_e4121_d_n6;
        locals.var_vbs_prime_dn8 = assign6080_e4121_d_n8;
        locals.var_vbs_prime_dn10 = assign6080_e4121_d_n10;
        locals.var_vbs_prime_dn11 = assign6080_e4121_d_n11;
        locals.var_vbs_prime_dn12 = assign6080_e4121_d_n12;
        locals.var_vbs_prime_rv = 0.0;

        let (assign6090_e4125, assign6090_e4125_d_n0, assign6090_e4125_d_n2, assign6090_e4125_d_n4, assign6090_e4125_d_n5, assign6090_e4125_d_n6, assign6090_e4125_d_n8, assign6090_e4125_d_n10, assign6090_e4125_d_n11, assign6090_e4125_d_n12,) = {
    if (locals.var_guard60 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign6090_e4125;
        locals.var_t0_dn0 = assign6090_e4125_d_n0;
        locals.var_t0_dn2 = assign6090_e4125_d_n2;
        locals.var_t0_dn4 = assign6090_e4125_d_n4;
        locals.var_t0_dn5 = assign6090_e4125_d_n5;
        locals.var_t0_dn6 = assign6090_e4125_d_n6;
        locals.var_t0_dn8 = assign6090_e4125_d_n8;
        locals.var_t0_dn10 = assign6090_e4125_d_n10;
        locals.var_t0_dn11 = assign6090_e4125_d_n11;
        locals.var_t0_dn12 = assign6090_e4125_d_n12;
        locals.var_t0_rv = 0.0;

        let assign6100_e4129: f64 = (p.p71 / locals.var_lgate);
        let assign6100_e4131: f64 = (assign6100_e4129 * locals.var_pbsum);
        let assign6100_e4132: f64 = (p.p69 + assign6100_e4131);
        let assign6100_e4135: f64 = (p.p70 * locals.var_vdsz);
        let assign6100_e4136: f64 = (assign6100_e4132 + assign6100_e4135);
        let assign6100_e4139: f64 = (p.p250 * locals.var_vbs_prime);
        let assign6100_e4140: f64 = (assign6100_e4136 + assign6100_e4139);
        locals.var_t5 = assign6100_e4140;
        locals.var_t5_dn0 = (((assign6100_e4129 * locals.var_pbsum_dn0) + (p.p70 * locals.var_vdsz_dn0)) + (p.p250 * locals.var_vbs_prime_dn0));
        locals.var_t5_dn2 = (((assign6100_e4129 * locals.var_pbsum_dn2) + (p.p70 * locals.var_vdsz_dn2)) + (p.p250 * locals.var_vbs_prime_dn2));
        locals.var_t5_dn4 = (((assign6100_e4129 * locals.var_pbsum_dn4) + (p.p70 * locals.var_vdsz_dn4)) + (p.p250 * locals.var_vbs_prime_dn4));
        locals.var_t5_dn5 = (((assign6100_e4129 * locals.var_pbsum_dn5) + (p.p70 * locals.var_vdsz_dn5)) + (p.p250 * locals.var_vbs_prime_dn5));
        locals.var_t5_dn6 = (((assign6100_e4129 * locals.var_pbsum_dn6) + (p.p70 * locals.var_vdsz_dn6)) + (p.p250 * locals.var_vbs_prime_dn6));
        locals.var_t5_dn8 = (((assign6100_e4129 * locals.var_pbsum_dn8) + (p.p70 * locals.var_vdsz_dn8)) + (p.p250 * locals.var_vbs_prime_dn8));
        locals.var_t5_dn10 = (((assign6100_e4129 * locals.var_pbsum_dn10) + (p.p70 * locals.var_vdsz_dn10)) + (p.p250 * locals.var_vbs_prime_dn10));
        locals.var_t5_dn11 = (((assign6100_e4129 * locals.var_pbsum_dn11) + (p.p70 * locals.var_vdsz_dn11)) + (p.p250 * locals.var_vbs_prime_dn11));
        locals.var_t5_dn12 = (((assign6100_e4129 * locals.var_pbsum_dn12) + (p.p70 * locals.var_vdsz_dn12)) + (p.p250 * locals.var_vbs_prime_dn12));
        locals.var_t5_rv = 0.0;

        let assign6110_e4143: f64 = (locals.var_dvth0 * locals.var_t5);
        locals.var_dvthsc = assign6110_e4143;
        locals.var_dvthsc_dn0 = ((locals.var_dvth0_dn0 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn0));
        locals.var_dvthsc_dn2 = ((locals.var_dvth0_dn2 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn2));
        locals.var_dvthsc_dn4 = ((locals.var_dvth0_dn4 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn4));
        locals.var_dvthsc_dn5 = ((locals.var_dvth0_dn5 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn5));
        locals.var_dvthsc_dn6 = ((locals.var_dvth0_dn6 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn6));
        locals.var_dvthsc_dn8 = ((locals.var_dvth0_dn8 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn8));
        locals.var_dvthsc_dn10 = ((locals.var_dvth0_dn10 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn10));
        locals.var_dvthsc_dn11 = ((locals.var_dvth0_dn11 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn11));
        locals.var_dvthsc_dn12 = ((locals.var_dvth0_dn12 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn12));
        locals.var_dvthsc_rv = 0.0;

        let assign6120_e4146: f64 = if p.p72 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard61 = assign6120_e4146;
        locals.var_guard61_rv = 0.0;

        let (assign6130_e4160, assign6130_e4160_d_n0, assign6130_e4160_d_n2, assign6130_e4160_d_n4, assign6130_e4160_d_n5, assign6130_e4160_d_n6, assign6130_e4160_d_n8, assign6130_e4160_d_n10, assign6130_e4160_d_n11, assign6130_e4160_d_n12,) = {
    if (locals.var_guard61 != 0.0) {
        let assign6130_e4150: f64 = (locals.var_eg + locals.var_pb2);
        let assign6130_e4153: f64 = (2.0 * p.p74);
        let assign6130_e4154: f64 = (assign6130_e4150 - assign6130_e4153);
        let assign6130_e4157: f64 = (p.p73 * locals.var_vdsz);
        let assign6130_e4158: f64 = (assign6130_e4154 + assign6130_e4157);
        (assign6130_e4158, ((locals.var_eg_dn0 + locals.var_pb2_dn0) + (p.p73 * locals.var_vdsz_dn0)), ((locals.var_eg_dn2 + locals.var_pb2_dn2) + (p.p73 * locals.var_vdsz_dn2)), ((locals.var_eg_dn4 + locals.var_pb2_dn4) + (p.p73 * locals.var_vdsz_dn4)), ((locals.var_eg_dn5 + locals.var_pb2_dn5) + (p.p73 * locals.var_vdsz_dn5)), ((locals.var_eg_dn6 + locals.var_pb2_dn6) + (p.p73 * locals.var_vdsz_dn6)), ((locals.var_eg_dn8 + locals.var_pb2_dn8) + (p.p73 * locals.var_vdsz_dn8)), ((locals.var_eg_dn10 + locals.var_pb2_dn10) + (p.p73 * locals.var_vdsz_dn10)), ((locals.var_eg_dn11 + locals.var_pb2_dn11) + (p.p73 * locals.var_vdsz_dn11)), ((locals.var_eg_dn12 + locals.var_pb2_dn12) + (p.p73 * locals.var_vdsz_dn12)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign6130_e4160;
        locals.var_t1_dn0 = assign6130_e4160_d_n0;
        locals.var_t1_dn2 = assign6130_e4160_d_n2;
        locals.var_t1_dn4 = assign6130_e4160_d_n4;
        locals.var_t1_dn5 = assign6130_e4160_d_n5;
        locals.var_t1_dn6 = assign6130_e4160_d_n6;
        locals.var_t1_dn8 = assign6130_e4160_d_n8;
        locals.var_t1_dn10 = assign6130_e4160_d_n10;
        locals.var_t1_dn11 = assign6130_e4160_d_n11;
        locals.var_t1_dn12 = assign6130_e4160_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign6140_e4168, assign6140_e4168_d_n0, assign6140_e4168_d_n2, assign6140_e4168_d_n4, assign6140_e4168_d_n5, assign6140_e4168_d_n6, assign6140_e4168_d_n8, assign6140_e4168_d_n10, assign6140_e4168_d_n11, assign6140_e4168_d_n12,) = {
    if (locals.var_guard61 != 0.0) {
        let assign6140_e4164: f64 = (locals.var_lgate * 0.5);
        let assign6140_e4166: f64 = (assign6140_e4164 + p.p56);
        (assign6140_e4166, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign6140_e4168;
        locals.var_t2_dn0 = assign6140_e4168_d_n0;
        locals.var_t2_dn2 = assign6140_e4168_d_n2;
        locals.var_t2_dn4 = assign6140_e4168_d_n4;
        locals.var_t2_dn5 = assign6140_e4168_d_n5;
        locals.var_t2_dn6 = assign6140_e4168_d_n6;
        locals.var_t2_dn8 = assign6140_e4168_d_n8;
        locals.var_t2_dn10 = assign6140_e4168_d_n10;
        locals.var_t2_dn11 = assign6140_e4168_d_n11;
        locals.var_t2_dn12 = assign6140_e4168_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign6150_e4176, assign6150_e4176_d_n0, assign6150_e4176_d_n2, assign6150_e4176_d_n4, assign6150_e4176_d_n5, assign6150_e4176_d_n6, assign6150_e4176_d_n8, assign6150_e4176_d_n10, assign6150_e4176_d_n11, assign6150_e4176_d_n12,) = {
    if (locals.var_guard61 != 0.0) {
        let assign6150_e4172: f64 = (p.p72 * p.p227);
        let assign6150_e4174: f64 = (assign6150_e4172 / locals.var_t2);
        (assign6150_e4174, (-((assign6150_e4172 * locals.var_t2_dn0) / (locals.var_t2 * locals.var_t2))), (-((assign6150_e4172 * locals.var_t2_dn2) / (locals.var_t2 * locals.var_t2))), (-((assign6150_e4172 * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))), (-((assign6150_e4172 * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))), (-((assign6150_e4172 * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))), (-((assign6150_e4172 * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))), (-((assign6150_e4172 * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))), (-((assign6150_e4172 * locals.var_t2_dn11) / (locals.var_t2 * locals.var_t2))), (-((assign6150_e4172 * locals.var_t2_dn12) / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign6150_e4176;
        locals.var_t3_dn0 = assign6150_e4176_d_n0;
        locals.var_t3_dn2 = assign6150_e4176_d_n2;
        locals.var_t3_dn4 = assign6150_e4176_d_n4;
        locals.var_t3_dn5 = assign6150_e4176_d_n5;
        locals.var_t3_dn6 = assign6150_e4176_d_n6;
        locals.var_t3_dn8 = assign6150_e4176_d_n8;
        locals.var_t3_dn10 = assign6150_e4176_d_n10;
        locals.var_t3_dn11 = assign6150_e4176_d_n11;
        locals.var_t3_dn12 = assign6150_e4176_d_n12;
        locals.var_t3_rv = 0.0;

        let (assign6160_e4182, assign6160_e4182_d_n0, assign6160_e4182_d_n2, assign6160_e4182_d_n4, assign6160_e4182_d_n5, assign6160_e4182_d_n6, assign6160_e4182_d_n8, assign6160_e4182_d_n10, assign6160_e4182_d_n11, assign6160_e4182_d_n12,) = {
    if (locals.var_guard61 != 0.0) {
        let assign6160_e4180: f64 = (locals.var_t1 * locals.var_t3);
        (assign6160_e4180, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn12 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn12)),)
    } else {
        (locals.var_dvthscr, locals.var_dvthscr_dn0, locals.var_dvthscr_dn2, locals.var_dvthscr_dn4, locals.var_dvthscr_dn5, locals.var_dvthscr_dn6, locals.var_dvthscr_dn8, locals.var_dvthscr_dn10, locals.var_dvthscr_dn11, locals.var_dvthscr_dn12,)
    }
};
        locals.var_dvthscr = assign6160_e4182;
        locals.var_dvthscr_dn0 = assign6160_e4182_d_n0;
        locals.var_dvthscr_dn2 = assign6160_e4182_d_n2;
        locals.var_dvthscr_dn4 = assign6160_e4182_d_n4;
        locals.var_dvthscr_dn5 = assign6160_e4182_d_n5;
        locals.var_dvthscr_dn6 = assign6160_e4182_d_n6;
        locals.var_dvthscr_dn8 = assign6160_e4182_d_n8;
        locals.var_dvthscr_dn10 = assign6160_e4182_d_n10;
        locals.var_dvthscr_dn11 = assign6160_e4182_d_n11;
        locals.var_dvthscr_dn12 = assign6160_e4182_d_n12;
        locals.var_dvthscr_rv = 0.0;

        let (assign6170_e4187, assign6170_e4187_d_n0, assign6170_e4187_d_n2, assign6170_e4187_d_n4, assign6170_e4187_d_n5, assign6170_e4187_d_n6, assign6170_e4187_d_n8, assign6170_e4187_d_n10, assign6170_e4187_d_n11, assign6170_e4187_d_n12,) = {
    if (locals.var_guard61 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dvthscr, locals.var_dvthscr_dn0, locals.var_dvthscr_dn2, locals.var_dvthscr_dn4, locals.var_dvthscr_dn5, locals.var_dvthscr_dn6, locals.var_dvthscr_dn8, locals.var_dvthscr_dn10, locals.var_dvthscr_dn11, locals.var_dvthscr_dn12,)
    }
};
        locals.var_dvthscr = assign6170_e4187;
        locals.var_dvthscr_dn0 = assign6170_e4187_d_n0;
        locals.var_dvthscr_dn2 = assign6170_e4187_d_n2;
        locals.var_dvthscr_dn4 = assign6170_e4187_d_n4;
        locals.var_dvthscr_dn5 = assign6170_e4187_d_n5;
        locals.var_dvthscr_dn6 = assign6170_e4187_d_n6;
        locals.var_dvthscr_dn8 = assign6170_e4187_d_n8;
        locals.var_dvthscr_dn10 = assign6170_e4187_d_n10;
        locals.var_dvthscr_dn11 = assign6170_e4187_d_n11;
        locals.var_dvthscr_dn12 = assign6170_e4187_d_n12;
        locals.var_dvthscr_rv = 0.0;

        let assign6180_e4192: f64 = (locals.var_uc_wfc / locals.var_weff);
        let assign6180_e4193: f64 = (locals.var_c_fox + assign6180_e4192);
        let assign6180_e4194: f64 = (1.0 / assign6180_e4193);
        locals.var_t3 = assign6180_e4194;
        locals.var_t3_dn0 = (-((locals.var_c_fox_dn0 + (-((locals.var_uc_wfc * locals.var_weff_dn0) / (locals.var_weff * locals.var_weff)))) / (assign6180_e4193 * assign6180_e4193)));
        locals.var_t3_dn2 = (-((locals.var_c_fox_dn2 + (-((locals.var_uc_wfc * locals.var_weff_dn2) / (locals.var_weff * locals.var_weff)))) / (assign6180_e4193 * assign6180_e4193)));
        locals.var_t3_dn4 = (-((locals.var_c_fox_dn4 + (-((locals.var_uc_wfc * locals.var_weff_dn4) / (locals.var_weff * locals.var_weff)))) / (assign6180_e4193 * assign6180_e4193)));
        locals.var_t3_dn5 = (-((locals.var_c_fox_dn5 + (-((locals.var_uc_wfc * locals.var_weff_dn5) / (locals.var_weff * locals.var_weff)))) / (assign6180_e4193 * assign6180_e4193)));
        locals.var_t3_dn6 = (-((locals.var_c_fox_dn6 + (-((locals.var_uc_wfc * locals.var_weff_dn6) / (locals.var_weff * locals.var_weff)))) / (assign6180_e4193 * assign6180_e4193)));
        locals.var_t3_dn8 = (-((locals.var_c_fox_dn8 + (-((locals.var_uc_wfc * locals.var_weff_dn8) / (locals.var_weff * locals.var_weff)))) / (assign6180_e4193 * assign6180_e4193)));
        locals.var_t3_dn10 = (-((locals.var_c_fox_dn10 + (-((locals.var_uc_wfc * locals.var_weff_dn10) / (locals.var_weff * locals.var_weff)))) / (assign6180_e4193 * assign6180_e4193)));
        locals.var_t3_dn11 = (-((locals.var_c_fox_dn11 + (-((locals.var_uc_wfc * locals.var_weff_dn11) / (locals.var_weff * locals.var_weff)))) / (assign6180_e4193 * assign6180_e4193)));
        locals.var_t3_dn12 = (-((locals.var_c_fox_dn12 + (-((locals.var_uc_wfc * locals.var_weff_dn12) / (locals.var_weff * locals.var_weff)))) / (assign6180_e4193 * assign6180_e4193)));
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_16(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign6190_e4197: f64 = (locals.var_c_fox_inv - locals.var_t3);
        locals.var_t5 = assign6190_e4197;
        locals.var_t5_dn0 = (locals.var_c_fox_inv_dn0 - locals.var_t3_dn0);
        locals.var_t5_dn2 = (locals.var_c_fox_inv_dn2 - locals.var_t3_dn2);
        locals.var_t5_dn4 = (locals.var_c_fox_inv_dn4 - locals.var_t3_dn4);
        locals.var_t5_dn5 = (locals.var_c_fox_inv_dn5 - locals.var_t3_dn5);
        locals.var_t5_dn6 = (locals.var_c_fox_inv_dn6 - locals.var_t3_dn6);
        locals.var_t5_dn8 = (locals.var_c_fox_inv_dn8 - locals.var_t3_dn8);
        locals.var_t5_dn10 = (locals.var_c_fox_inv_dn10 - locals.var_t3_dn10);
        locals.var_t5_dn11 = (locals.var_c_fox_inv_dn11 - locals.var_t3_dn11);
        locals.var_t5_dn12 = (locals.var_c_fox_inv_dn12 - locals.var_t3_dn12);
        locals.var_t5_rv = 0.0;

        let assign6200_e4200: f64 = (locals.var_qb0 * locals.var_t5);
        let assign6200_e4203: f64 = (p.p104 / locals.var_wg);
        let assign6200_e4204: f64 = (assign6200_e4200 + assign6200_e4203);
        locals.var_dvthw = assign6200_e4204;
        locals.var_dvthw_dn0 = ((locals.var_qb0_dn0 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn0));
        locals.var_dvthw_dn2 = ((locals.var_qb0_dn2 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn2));
        locals.var_dvthw_dn4 = ((locals.var_qb0_dn4 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn4));
        locals.var_dvthw_dn5 = ((locals.var_qb0_dn5 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn5));
        locals.var_dvthw_dn6 = ((locals.var_qb0_dn6 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn6));
        locals.var_dvthw_dn8 = ((locals.var_qb0_dn8 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn8));
        locals.var_dvthw_dn10 = ((locals.var_qb0_dn10 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn10));
        locals.var_dvthw_dn11 = ((locals.var_qb0_dn11 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn11));
        locals.var_dvthw_dn12 = ((locals.var_qb0_dn12 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn12));
        locals.var_dvthw_rv = 0.0;

        let assign6210_e4207: f64 = (locals.var_dvthsc + locals.var_dvthlp);
        let assign6210_e4209: f64 = (assign6210_e4207 + locals.var_dvthw);
        let assign6210_e4211: f64 = (assign6210_e4209 + locals.var_dvthscr);
        let assign6210_e4213: f64 = (assign6210_e4211 + locals.var_dvthsm);
        locals.var_dvth = assign6210_e4213;
        locals.var_dvth_dn0 = (((locals.var_dvthsc_dn0 + locals.var_dvthlp_dn0) + locals.var_dvthw_dn0) + locals.var_dvthscr_dn0);
        locals.var_dvth_dn2 = (((locals.var_dvthsc_dn2 + locals.var_dvthlp_dn2) + locals.var_dvthw_dn2) + locals.var_dvthscr_dn2);
        locals.var_dvth_dn4 = (((locals.var_dvthsc_dn4 + locals.var_dvthlp_dn4) + locals.var_dvthw_dn4) + locals.var_dvthscr_dn4);
        locals.var_dvth_dn5 = (((locals.var_dvthsc_dn5 + locals.var_dvthlp_dn5) + locals.var_dvthw_dn5) + locals.var_dvthscr_dn5);
        locals.var_dvth_dn6 = (((locals.var_dvthsc_dn6 + locals.var_dvthlp_dn6) + locals.var_dvthw_dn6) + locals.var_dvthscr_dn6);
        locals.var_dvth_dn8 = (((locals.var_dvthsc_dn8 + locals.var_dvthlp_dn8) + locals.var_dvthw_dn8) + locals.var_dvthscr_dn8);
        locals.var_dvth_dn10 = (((locals.var_dvthsc_dn10 + locals.var_dvthlp_dn10) + locals.var_dvthw_dn10) + locals.var_dvthscr_dn10);
        locals.var_dvth_dn11 = (((locals.var_dvthsc_dn11 + locals.var_dvthlp_dn11) + locals.var_dvthw_dn11) + locals.var_dvthscr_dn11);
        locals.var_dvth_dn12 = (((locals.var_dvthsc_dn12 + locals.var_dvthlp_dn12) + locals.var_dvthw_dn12) + locals.var_dvthscr_dn12);
        locals.var_dvth_rv = 0.0;

        let assign6220_e4216: f64 = (locals.var_vthp - locals.var_dvth);
        locals.var_vth = assign6220_e4216;
        locals.var_vth_dn0 = (locals.var_vthp_dn0 - locals.var_dvth_dn0);
        locals.var_vth_dn2 = (locals.var_vthp_dn2 - locals.var_dvth_dn2);
        locals.var_vth_dn4 = (locals.var_vthp_dn4 - locals.var_dvth_dn4);
        locals.var_vth_dn5 = (locals.var_vthp_dn5 - locals.var_dvth_dn5);
        locals.var_vth_dn6 = (locals.var_vthp_dn6 - locals.var_dvth_dn6);
        locals.var_vth_dn8 = (locals.var_vthp_dn8 - locals.var_dvth_dn8);
        locals.var_vth_dn10 = (locals.var_vthp_dn10 - locals.var_dvth_dn10);
        locals.var_vth_dn11 = (locals.var_vthp_dn11 - locals.var_dvth_dn11);
        locals.var_vth_dn12 = (locals.var_vthp_dn12 - locals.var_dvth_dn12);
        locals.var_vth_rv = 0.0;

        let assign6230_e4219: f64 = if p.p75 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard62 = assign6230_e4219;
        locals.var_guard62_rv = 0.0;

        let (assign6240_e4223,) = {
    if (locals.var_guard62 != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_dppg,)
    }
};
        locals.var_flg_dppg = assign6240_e4223;
        locals.var_flg_dppg_rv = 0.0;

        let (assign6250_e4228,) = {
    if (locals.var_guard62 == 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_dppg,)
    }
};
        locals.var_flg_dppg = assign6250_e4228;
        locals.var_flg_dppg_rv = 0.0;

        let assign6260_e4231: f64 = if locals.var_flg_dppg == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard63 = assign6260_e4231;
        locals.var_guard63_rv = 0.0;

        let (assign6270_e4235, assign6270_e4235_d_n0, assign6270_e4235_d_n2, assign6270_e4235_d_n4, assign6270_e4235_d_n5, assign6270_e4235_d_n6, assign6270_e4235_d_n8, assign6270_e4235_d_n10, assign6270_e4235_d_n11, assign6270_e4235_d_n12,) = {
    if (locals.var_guard63 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn8, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12,)
    }
};
        locals.var_dppg = assign6270_e4235;
        locals.var_dppg_dn0 = assign6270_e4235_d_n0;
        locals.var_dppg_dn2 = assign6270_e4235_d_n2;
        locals.var_dppg_dn4 = assign6270_e4235_d_n4;
        locals.var_dppg_dn5 = assign6270_e4235_d_n5;
        locals.var_dppg_dn6 = assign6270_e4235_d_n6;
        locals.var_dppg_dn8 = assign6270_e4235_d_n8;
        locals.var_dppg_dn10 = assign6270_e4235_d_n10;
        locals.var_dppg_dn11 = assign6270_e4235_d_n11;
        locals.var_dppg_dn12 = assign6270_e4235_d_n12;
        locals.var_dppg_rv = 0.0;

        let (assign6280_e4242, assign6280_e4242_d_n0, assign6280_e4242_d_n2, assign6280_e4242_d_n4, assign6280_e4242_d_n5, assign6280_e4242_d_n6, assign6280_e4242_d_n8, assign6280_e4242_d_n10, assign6280_e4242_d_n11, assign6280_e4242_d_n12,) = {
    if (locals.var_guard63 == 0.0) {
        let assign6280_e4240: f64 = (locals.var_vgsz - p.p76);
        (assign6280_e4240, locals.var_vgsz_dn0, locals.var_vgsz_dn2, locals.var_vgsz_dn4, locals.var_vgsz_dn5, locals.var_vgsz_dn6, locals.var_vgsz_dn8, locals.var_vgsz_dn10, locals.var_vgsz_dn11, locals.var_vgsz_dn12,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign6280_e4242;
        locals.var_t3_dn0 = assign6280_e4242_d_n0;
        locals.var_t3_dn2 = assign6280_e4242_d_n2;
        locals.var_t3_dn4 = assign6280_e4242_d_n4;
        locals.var_t3_dn5 = assign6280_e4242_d_n5;
        locals.var_t3_dn6 = assign6280_e4242_d_n6;
        locals.var_t3_dn8 = assign6280_e4242_d_n8;
        locals.var_t3_dn10 = assign6280_e4242_d_n10;
        locals.var_t3_dn11 = assign6280_e4242_d_n11;
        locals.var_t3_dn12 = assign6280_e4242_d_n12;
        locals.var_t3_rv = 0.0;

        let assign6290_e4245: f64 = (-3.0);
        let assign6290_e4246: f64 = if locals.var_t3 < assign6290_e4245 { 1.0 } else { 0.0 };
        locals.var_guard64 = assign6290_e4246;
        locals.var_guard64_rv = 0.0;

        let (assign6300_e4253, assign6300_e4253_d_n0, assign6300_e4253_d_n2, assign6300_e4253_d_n4, assign6300_e4253_d_n5, assign6300_e4253_d_n6, assign6300_e4253_d_n8, assign6300_e4253_d_n10, assign6300_e4253_d_n11, assign6300_e4253_d_n12,) = {
    if ((locals.var_guard63 == 0.0) && (locals.var_guard64 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn8, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12,)
    }
};
        locals.var_t6 = assign6300_e4253;
        locals.var_t6_dn0 = assign6300_e4253_d_n0;
        locals.var_t6_dn2 = assign6300_e4253_d_n2;
        locals.var_t6_dn4 = assign6300_e4253_d_n4;
        locals.var_t6_dn5 = assign6300_e4253_d_n5;
        locals.var_t6_dn6 = assign6300_e4253_d_n6;
        locals.var_t6_dn8 = assign6300_e4253_d_n8;
        locals.var_t6_dn10 = assign6300_e4253_d_n10;
        locals.var_t6_dn11 = assign6300_e4253_d_n11;
        locals.var_t6_dn12 = assign6300_e4253_d_n12;
        locals.var_t6_rv = 0.0;

        let (assign6310_e4260, assign6310_e4260_d_n0, assign6310_e4260_d_n2, assign6310_e4260_d_n4, assign6310_e4260_d_n5, assign6310_e4260_d_n6, assign6310_e4260_d_n8, assign6310_e4260_d_n10, assign6310_e4260_d_n11, assign6310_e4260_d_n12,) = {
    if ((locals.var_guard63 == 0.0) && (locals.var_guard64 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn8, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12,)
    }
};
        locals.var_dppg = assign6310_e4260;
        locals.var_dppg_dn0 = assign6310_e4260_d_n0;
        locals.var_dppg_dn2 = assign6310_e4260_d_n2;
        locals.var_dppg_dn4 = assign6310_e4260_d_n4;
        locals.var_dppg_dn5 = assign6310_e4260_d_n5;
        locals.var_dppg_dn6 = assign6310_e4260_d_n6;
        locals.var_dppg_dn8 = assign6310_e4260_d_n8;
        locals.var_dppg_dn10 = assign6310_e4260_d_n10;
        locals.var_dppg_dn11 = assign6310_e4260_d_n11;
        locals.var_dppg_dn12 = assign6310_e4260_d_n12;
        locals.var_dppg_rv = 0.0;

        let assign6320_e4263: f64 = if locals.var_t3 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard65 = assign6320_e4263;
        locals.var_guard65_rv = 0.0;

        let (assign6330_e4289, assign6330_e4289_d_n0, assign6330_e4289_d_n2, assign6330_e4289_d_n4, assign6330_e4289_d_n5, assign6330_e4289_d_n6, assign6330_e4289_d_n8, assign6330_e4289_d_n10, assign6330_e4289_d_n11, assign6330_e4289_d_n12,) = {
    if (((locals.var_guard63 == 0.0) && (locals.var_guard64 == 0.0)) && (locals.var_guard65 != 0.0)) {
        let assign6330_e4276: f64 = (1.0 / 3.0);
        let assign6330_e4277: f64 = (2.0 * assign6330_e4276);
        let assign6330_e4280: f64 = (locals.var_t3 * 3.0);
        let assign6330_e4283: f64 = (1.0 / 27.0);
        let assign6330_e4284: f64 = (assign6330_e4280 * assign6330_e4283);
        let assign6330_e4285: f64 = (assign6330_e4277 + assign6330_e4284);
        let assign6330_e4286: f64 = (locals.var_t3 * assign6330_e4285);
        let assign6330_e4287: f64 = (1.0 + assign6330_e4286);
        (assign6330_e4287, ((locals.var_t3_dn0 * assign6330_e4285) + (locals.var_t3 * ((locals.var_t3_dn0 * 3.0) * assign6330_e4283))), ((locals.var_t3_dn2 * assign6330_e4285) + (locals.var_t3 * ((locals.var_t3_dn2 * 3.0) * assign6330_e4283))), ((locals.var_t3_dn4 * assign6330_e4285) + (locals.var_t3 * ((locals.var_t3_dn4 * 3.0) * assign6330_e4283))), ((locals.var_t3_dn5 * assign6330_e4285) + (locals.var_t3 * ((locals.var_t3_dn5 * 3.0) * assign6330_e4283))), ((locals.var_t3_dn6 * assign6330_e4285) + (locals.var_t3 * ((locals.var_t3_dn6 * 3.0) * assign6330_e4283))), ((locals.var_t3_dn8 * assign6330_e4285) + (locals.var_t3 * ((locals.var_t3_dn8 * 3.0) * assign6330_e4283))), ((locals.var_t3_dn10 * assign6330_e4285) + (locals.var_t3 * ((locals.var_t3_dn10 * 3.0) * assign6330_e4283))), ((locals.var_t3_dn11 * assign6330_e4285) + (locals.var_t3 * ((locals.var_t3_dn11 * 3.0) * assign6330_e4283))), ((locals.var_t3_dn12 * assign6330_e4285) + (locals.var_t3 * ((locals.var_t3_dn12 * 3.0) * assign6330_e4283))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn8, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12,)
    }
};
        locals.var_t6 = assign6330_e4289;
        locals.var_t6_dn0 = assign6330_e4289_d_n0;
        locals.var_t6_dn2 = assign6330_e4289_d_n2;
        locals.var_t6_dn4 = assign6330_e4289_d_n4;
        locals.var_t6_dn5 = assign6330_e4289_d_n5;
        locals.var_t6_dn6 = assign6330_e4289_d_n6;
        locals.var_t6_dn8 = assign6330_e4289_d_n8;
        locals.var_t6_dn10 = assign6330_e4289_d_n10;
        locals.var_t6_dn11 = assign6330_e4289_d_n11;
        locals.var_t6_dn12 = assign6330_e4289_d_n12;
        locals.var_t6_rv = 0.0;

        let (assign6340_e4315, assign6340_e4315_d_n0, assign6340_e4315_d_n2, assign6340_e4315_d_n4, assign6340_e4315_d_n5, assign6340_e4315_d_n6, assign6340_e4315_d_n8, assign6340_e4315_d_n10, assign6340_e4315_d_n11, assign6340_e4315_d_n12,) = {
    if (((locals.var_guard63 == 0.0) && (locals.var_guard64 == 0.0)) && (locals.var_guard65 != 0.0)) {
        let assign6340_e4303: f64 = (1.0 / 3.0);
        let assign6340_e4307: f64 = (1.0 / 27.0);
        let assign6340_e4308: f64 = (locals.var_t3 * assign6340_e4307);
        let assign6340_e4309: f64 = (assign6340_e4303 + assign6340_e4308);
        let assign6340_e4310: f64 = (locals.var_t3 * assign6340_e4309);
        let assign6340_e4311: f64 = (1.0 + assign6340_e4310);
        let assign6340_e4312: f64 = (locals.var_t3 * assign6340_e4311);
        let assign6340_e4313: f64 = (1.0 + assign6340_e4312);
        (assign6340_e4313, ((locals.var_t3_dn0 * assign6340_e4311) + (locals.var_t3 * ((locals.var_t3_dn0 * assign6340_e4309) + (locals.var_t3 * (locals.var_t3_dn0 * assign6340_e4307))))), ((locals.var_t3_dn2 * assign6340_e4311) + (locals.var_t3 * ((locals.var_t3_dn2 * assign6340_e4309) + (locals.var_t3 * (locals.var_t3_dn2 * assign6340_e4307))))), ((locals.var_t3_dn4 * assign6340_e4311) + (locals.var_t3 * ((locals.var_t3_dn4 * assign6340_e4309) + (locals.var_t3 * (locals.var_t3_dn4 * assign6340_e4307))))), ((locals.var_t3_dn5 * assign6340_e4311) + (locals.var_t3 * ((locals.var_t3_dn5 * assign6340_e4309) + (locals.var_t3 * (locals.var_t3_dn5 * assign6340_e4307))))), ((locals.var_t3_dn6 * assign6340_e4311) + (locals.var_t3 * ((locals.var_t3_dn6 * assign6340_e4309) + (locals.var_t3 * (locals.var_t3_dn6 * assign6340_e4307))))), ((locals.var_t3_dn8 * assign6340_e4311) + (locals.var_t3 * ((locals.var_t3_dn8 * assign6340_e4309) + (locals.var_t3 * (locals.var_t3_dn8 * assign6340_e4307))))), ((locals.var_t3_dn10 * assign6340_e4311) + (locals.var_t3 * ((locals.var_t3_dn10 * assign6340_e4309) + (locals.var_t3 * (locals.var_t3_dn10 * assign6340_e4307))))), ((locals.var_t3_dn11 * assign6340_e4311) + (locals.var_t3 * ((locals.var_t3_dn11 * assign6340_e4309) + (locals.var_t3 * (locals.var_t3_dn11 * assign6340_e4307))))), ((locals.var_t3_dn12 * assign6340_e4311) + (locals.var_t3 * ((locals.var_t3_dn12 * assign6340_e4309) + (locals.var_t3 * (locals.var_t3_dn12 * assign6340_e4307))))),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn8, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12,)
    }
};
        locals.var_dppg = assign6340_e4315;
        locals.var_dppg_dn0 = assign6340_e4315_d_n0;
        locals.var_dppg_dn2 = assign6340_e4315_d_n2;
        locals.var_dppg_dn4 = assign6340_e4315_d_n4;
        locals.var_dppg_dn5 = assign6340_e4315_d_n5;
        locals.var_dppg_dn6 = assign6340_e4315_d_n6;
        locals.var_dppg_dn8 = assign6340_e4315_d_n8;
        locals.var_dppg_dn10 = assign6340_e4315_d_n10;
        locals.var_dppg_dn11 = assign6340_e4315_d_n11;
        locals.var_dppg_dn12 = assign6340_e4315_d_n12;
        locals.var_dppg_rv = 0.0;

        let (assign6350_e4346, assign6350_e4346_d_n0, assign6350_e4346_d_n2, assign6350_e4346_d_n4, assign6350_e4346_d_n5, assign6350_e4346_d_n6, assign6350_e4346_d_n8, assign6350_e4346_d_n10, assign6350_e4346_d_n11, assign6350_e4346_d_n12,) = {
    if (((locals.var_guard63 == 0.0) && (locals.var_guard64 == 0.0)) && (locals.var_guard65 == 0.0)) {
        let assign6350_e4329: f64 = (1.0 / 3.0);
        let assign6350_e4330: f64 = (2.0 * assign6350_e4329);
        let assign6350_e4334: f64 = (3.0 * 0.0402052934513951);
        let assign6350_e4337: f64 = (locals.var_t3 * 4.0);
        let assign6350_e4339: f64 = (assign6350_e4337 * 0.148148111111111);
        let assign6350_e4340: f64 = (assign6350_e4334 + assign6350_e4339);
        let assign6350_e4341: f64 = (locals.var_t3 * assign6350_e4340);
        let assign6350_e4342: f64 = (assign6350_e4330 + assign6350_e4341);
        let assign6350_e4343: f64 = (locals.var_t3 * assign6350_e4342);
        let assign6350_e4344: f64 = (1.0 + assign6350_e4343);
        (assign6350_e4344, ((locals.var_t3_dn0 * assign6350_e4342) + (locals.var_t3 * ((locals.var_t3_dn0 * assign6350_e4340) + (locals.var_t3 * ((locals.var_t3_dn0 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn2 * assign6350_e4342) + (locals.var_t3 * ((locals.var_t3_dn2 * assign6350_e4340) + (locals.var_t3 * ((locals.var_t3_dn2 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn4 * assign6350_e4342) + (locals.var_t3 * ((locals.var_t3_dn4 * assign6350_e4340) + (locals.var_t3 * ((locals.var_t3_dn4 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn5 * assign6350_e4342) + (locals.var_t3 * ((locals.var_t3_dn5 * assign6350_e4340) + (locals.var_t3 * ((locals.var_t3_dn5 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn6 * assign6350_e4342) + (locals.var_t3 * ((locals.var_t3_dn6 * assign6350_e4340) + (locals.var_t3 * ((locals.var_t3_dn6 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn8 * assign6350_e4342) + (locals.var_t3 * ((locals.var_t3_dn8 * assign6350_e4340) + (locals.var_t3 * ((locals.var_t3_dn8 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn10 * assign6350_e4342) + (locals.var_t3 * ((locals.var_t3_dn10 * assign6350_e4340) + (locals.var_t3 * ((locals.var_t3_dn10 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn11 * assign6350_e4342) + (locals.var_t3 * ((locals.var_t3_dn11 * assign6350_e4340) + (locals.var_t3 * ((locals.var_t3_dn11 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn12 * assign6350_e4342) + (locals.var_t3 * ((locals.var_t3_dn12 * assign6350_e4340) + (locals.var_t3 * ((locals.var_t3_dn12 * 4.0) * 0.148148111111111))))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn8, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12,)
    }
};
        locals.var_t6 = assign6350_e4346;
        locals.var_t6_dn0 = assign6350_e4346_d_n0;
        locals.var_t6_dn2 = assign6350_e4346_d_n2;
        locals.var_t6_dn4 = assign6350_e4346_d_n4;
        locals.var_t6_dn5 = assign6350_e4346_d_n5;
        locals.var_t6_dn6 = assign6350_e4346_d_n6;
        locals.var_t6_dn8 = assign6350_e4346_d_n8;
        locals.var_t6_dn10 = assign6350_e4346_d_n10;
        locals.var_t6_dn11 = assign6350_e4346_d_n11;
        locals.var_t6_dn12 = assign6350_e4346_d_n12;
        locals.var_t6_rv = 0.0;

        let (assign6360_e4375, assign6360_e4375_d_n0, assign6360_e4375_d_n2, assign6360_e4375_d_n4, assign6360_e4375_d_n5, assign6360_e4375_d_n6, assign6360_e4375_d_n8, assign6360_e4375_d_n10, assign6360_e4375_d_n11, assign6360_e4375_d_n12,) = {
    if (((locals.var_guard63 == 0.0) && (locals.var_guard64 == 0.0)) && (locals.var_guard65 == 0.0)) {
        let assign6360_e4361: f64 = (1.0 / 3.0);
        let assign6360_e4366: f64 = (locals.var_t3 * 0.148148111111111);
        let assign6360_e4367: f64 = (0.0402052934513951 + assign6360_e4366);
        let assign6360_e4368: f64 = (locals.var_t3 * assign6360_e4367);
        let assign6360_e4369: f64 = (assign6360_e4361 + assign6360_e4368);
        let assign6360_e4370: f64 = (locals.var_t3 * assign6360_e4369);
        let assign6360_e4371: f64 = (1.0 + assign6360_e4370);
        let assign6360_e4372: f64 = (locals.var_t3 * assign6360_e4371);
        let assign6360_e4373: f64 = (1.0 + assign6360_e4372);
        (assign6360_e4373, ((locals.var_t3_dn0 * assign6360_e4371) + (locals.var_t3 * ((locals.var_t3_dn0 * assign6360_e4369) + (locals.var_t3 * ((locals.var_t3_dn0 * assign6360_e4367) + (locals.var_t3 * (locals.var_t3_dn0 * 0.148148111111111))))))), ((locals.var_t3_dn2 * assign6360_e4371) + (locals.var_t3 * ((locals.var_t3_dn2 * assign6360_e4369) + (locals.var_t3 * ((locals.var_t3_dn2 * assign6360_e4367) + (locals.var_t3 * (locals.var_t3_dn2 * 0.148148111111111))))))), ((locals.var_t3_dn4 * assign6360_e4371) + (locals.var_t3 * ((locals.var_t3_dn4 * assign6360_e4369) + (locals.var_t3 * ((locals.var_t3_dn4 * assign6360_e4367) + (locals.var_t3 * (locals.var_t3_dn4 * 0.148148111111111))))))), ((locals.var_t3_dn5 * assign6360_e4371) + (locals.var_t3 * ((locals.var_t3_dn5 * assign6360_e4369) + (locals.var_t3 * ((locals.var_t3_dn5 * assign6360_e4367) + (locals.var_t3 * (locals.var_t3_dn5 * 0.148148111111111))))))), ((locals.var_t3_dn6 * assign6360_e4371) + (locals.var_t3 * ((locals.var_t3_dn6 * assign6360_e4369) + (locals.var_t3 * ((locals.var_t3_dn6 * assign6360_e4367) + (locals.var_t3 * (locals.var_t3_dn6 * 0.148148111111111))))))), ((locals.var_t3_dn8 * assign6360_e4371) + (locals.var_t3 * ((locals.var_t3_dn8 * assign6360_e4369) + (locals.var_t3 * ((locals.var_t3_dn8 * assign6360_e4367) + (locals.var_t3 * (locals.var_t3_dn8 * 0.148148111111111))))))), ((locals.var_t3_dn10 * assign6360_e4371) + (locals.var_t3 * ((locals.var_t3_dn10 * assign6360_e4369) + (locals.var_t3 * ((locals.var_t3_dn10 * assign6360_e4367) + (locals.var_t3 * (locals.var_t3_dn10 * 0.148148111111111))))))), ((locals.var_t3_dn11 * assign6360_e4371) + (locals.var_t3 * ((locals.var_t3_dn11 * assign6360_e4369) + (locals.var_t3 * ((locals.var_t3_dn11 * assign6360_e4367) + (locals.var_t3 * (locals.var_t3_dn11 * 0.148148111111111))))))), ((locals.var_t3_dn12 * assign6360_e4371) + (locals.var_t3 * ((locals.var_t3_dn12 * assign6360_e4369) + (locals.var_t3 * ((locals.var_t3_dn12 * assign6360_e4367) + (locals.var_t3 * (locals.var_t3_dn12 * 0.148148111111111))))))),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn8, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12,)
    }
};
        locals.var_dppg = assign6360_e4375;
        locals.var_dppg_dn0 = assign6360_e4375_d_n0;
        locals.var_dppg_dn2 = assign6360_e4375_d_n2;
        locals.var_dppg_dn4 = assign6360_e4375_d_n4;
        locals.var_dppg_dn5 = assign6360_e4375_d_n5;
        locals.var_dppg_dn6 = assign6360_e4375_d_n6;
        locals.var_dppg_dn8 = assign6360_e4375_d_n8;
        locals.var_dppg_dn10 = assign6360_e4375_d_n10;
        locals.var_dppg_dn11 = assign6360_e4375_d_n11;
        locals.var_dppg_dn12 = assign6360_e4375_d_n12;
        locals.var_dppg_rv = 0.0;

        let (assign6370_e4393, assign6370_e4393_d_n0, assign6370_e4393_d_n2, assign6370_e4393_d_n4, assign6370_e4393_d_n5, assign6370_e4393_d_n6, assign6370_e4393_d_n8, assign6370_e4393_d_n10, assign6370_e4393_d_n11, assign6370_e4393_d_n12,) = {
    if (locals.var_guard63 == 0.0) {
        let assign6370_e4380: f64 = (locals.var_dppg - 1.0);
        let assign6370_e4383: f64 = (locals.var_dppg - 1.0);
        let assign6370_e4384: f64 = (assign6370_e4380 * assign6370_e4383);
        let assign6370_e4387: f64 = (4.0 * 0.1);
        let assign6370_e4389: f64 = (assign6370_e4387 * 0.1);
        let assign6370_e4390: f64 = (assign6370_e4384 + assign6370_e4389);
        let assign6370_e4391: f64 = (assign6370_e4390).sqrt();
        (assign6370_e4391, (((locals.var_dppg_dn0 * assign6370_e4383) + (assign6370_e4380 * locals.var_dppg_dn0)) / (2.0 * assign6370_e4391)), (((locals.var_dppg_dn2 * assign6370_e4383) + (assign6370_e4380 * locals.var_dppg_dn2)) / (2.0 * assign6370_e4391)), (((locals.var_dppg_dn4 * assign6370_e4383) + (assign6370_e4380 * locals.var_dppg_dn4)) / (2.0 * assign6370_e4391)), (((locals.var_dppg_dn5 * assign6370_e4383) + (assign6370_e4380 * locals.var_dppg_dn5)) / (2.0 * assign6370_e4391)), (((locals.var_dppg_dn6 * assign6370_e4383) + (assign6370_e4380 * locals.var_dppg_dn6)) / (2.0 * assign6370_e4391)), (((locals.var_dppg_dn8 * assign6370_e4383) + (assign6370_e4380 * locals.var_dppg_dn8)) / (2.0 * assign6370_e4391)), (((locals.var_dppg_dn10 * assign6370_e4383) + (assign6370_e4380 * locals.var_dppg_dn10)) / (2.0 * assign6370_e4391)), (((locals.var_dppg_dn11 * assign6370_e4383) + (assign6370_e4380 * locals.var_dppg_dn11)) / (2.0 * assign6370_e4391)), (((locals.var_dppg_dn12 * assign6370_e4383) + (assign6370_e4380 * locals.var_dppg_dn12)) / (2.0 * assign6370_e4391)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign6370_e4393;
        locals.var_tmf2_dn0 = assign6370_e4393_d_n0;
        locals.var_tmf2_dn2 = assign6370_e4393_d_n2;
        locals.var_tmf2_dn4 = assign6370_e4393_d_n4;
        locals.var_tmf2_dn5 = assign6370_e4393_d_n5;
        locals.var_tmf2_dn6 = assign6370_e4393_d_n6;
        locals.var_tmf2_dn8 = assign6370_e4393_d_n8;
        locals.var_tmf2_dn10 = assign6370_e4393_d_n10;
        locals.var_tmf2_dn11 = assign6370_e4393_d_n11;
        locals.var_tmf2_dn12 = assign6370_e4393_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign6380_e4406, assign6380_e4406_d_n0, assign6380_e4406_d_n2, assign6380_e4406_d_n4, assign6380_e4406_d_n5, assign6380_e4406_d_n6, assign6380_e4406_d_n8, assign6380_e4406_d_n10, assign6380_e4406_d_n11, assign6380_e4406_d_n12,) = {
    if (locals.var_guard63 == 0.0) {
        let assign6380_e4400: f64 = (locals.var_dppg - 1.0);
        let assign6380_e4402: f64 = (assign6380_e4400 / locals.var_tmf2);
        let assign6380_e4403: f64 = (1.0 + assign6380_e4402);
        let assign6380_e4404: f64 = (0.5 * assign6380_e4403);
        (assign6380_e4404, (0.5 * (((locals.var_dppg_dn0 * locals.var_tmf2) - (assign6380_e4400 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn2 * locals.var_tmf2) - (assign6380_e4400 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn4 * locals.var_tmf2) - (assign6380_e4400 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn5 * locals.var_tmf2) - (assign6380_e4400 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn6 * locals.var_tmf2) - (assign6380_e4400 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn8 * locals.var_tmf2) - (assign6380_e4400 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn10 * locals.var_tmf2) - (assign6380_e4400 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn11 * locals.var_tmf2) - (assign6380_e4400 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn12 * locals.var_tmf2) - (assign6380_e4400 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn8, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12,)
    }
};
        locals.var_t6 = assign6380_e4406;
        locals.var_t6_dn0 = assign6380_e4406_d_n0;
        locals.var_t6_dn2 = assign6380_e4406_d_n2;
        locals.var_t6_dn4 = assign6380_e4406_d_n4;
        locals.var_t6_dn5 = assign6380_e4406_d_n5;
        locals.var_t6_dn6 = assign6380_e4406_d_n6;
        locals.var_t6_dn8 = assign6380_e4406_d_n8;
        locals.var_t6_dn10 = assign6380_e4406_d_n10;
        locals.var_t6_dn11 = assign6380_e4406_d_n11;
        locals.var_t6_dn12 = assign6380_e4406_d_n12;
        locals.var_t6_rv = 0.0;

        let (assign6390_e4421, assign6390_e4421_d_n0, assign6390_e4421_d_n2, assign6390_e4421_d_n4, assign6390_e4421_d_n5, assign6390_e4421_d_n6, assign6390_e4421_d_n8, assign6390_e4421_d_n10, assign6390_e4421_d_n11, assign6390_e4421_d_n12,) = {
    if (locals.var_guard63 == 0.0) {
        let assign6390_e4412: f64 = (locals.var_dppg - 1.0);
        let assign6390_e4414: f64 = (assign6390_e4412 + locals.var_tmf2);
        let assign6390_e4415: f64 = (0.5 * assign6390_e4414);
        let assign6390_e4418: f64 = (1e-10 * 0.1);
        let assign6390_e4419: f64 = (assign6390_e4415 + assign6390_e4418);
        (assign6390_e4419, (0.5 * (locals.var_dppg_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_dppg_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_dppg_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_dppg_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_dppg_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_dppg_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_dppg_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_dppg_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_dppg_dn12 + locals.var_tmf2_dn12)),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn8, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12,)
    }
};
        locals.var_dppg = assign6390_e4421;
        locals.var_dppg_dn0 = assign6390_e4421_d_n0;
        locals.var_dppg_dn2 = assign6390_e4421_d_n2;
        locals.var_dppg_dn4 = assign6390_e4421_d_n4;
        locals.var_dppg_dn5 = assign6390_e4421_d_n5;
        locals.var_dppg_dn6 = assign6390_e4421_d_n6;
        locals.var_dppg_dn8 = assign6390_e4421_d_n8;
        locals.var_dppg_dn10 = assign6390_e4421_d_n10;
        locals.var_dppg_dn11 = assign6390_e4421_d_n11;
        locals.var_dppg_dn12 = assign6390_e4421_d_n12;
        locals.var_dppg_rv = 0.0;

        let assign6400_e4424: f64 = if locals.var_dppg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard66 = assign6400_e4424;
        locals.var_guard66_rv = 0.0;

        let (assign6410_e4431, assign6410_e4431_d_n0, assign6410_e4431_d_n2, assign6410_e4431_d_n4, assign6410_e4431_d_n5, assign6410_e4431_d_n6, assign6410_e4431_d_n8, assign6410_e4431_d_n10, assign6410_e4431_d_n11, assign6410_e4431_d_n12,) = {
    if ((locals.var_guard63 == 0.0) && (locals.var_guard66 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn8, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12,)
    }
};
        locals.var_dppg = assign6410_e4431;
        locals.var_dppg_dn0 = assign6410_e4431_d_n0;
        locals.var_dppg_dn2 = assign6410_e4431_d_n2;
        locals.var_dppg_dn4 = assign6410_e4431_d_n4;
        locals.var_dppg_dn5 = assign6410_e4431_d_n5;
        locals.var_dppg_dn6 = assign6410_e4431_d_n6;
        locals.var_dppg_dn8 = assign6410_e4431_d_n8;
        locals.var_dppg_dn10 = assign6410_e4431_d_n10;
        locals.var_dppg_dn11 = assign6410_e4431_d_n11;
        locals.var_dppg_dn12 = assign6410_e4431_d_n12;
        locals.var_dppg_rv = 0.0;

        let (assign6420_e4438, assign6420_e4438_d_n0, assign6420_e4438_d_n2, assign6420_e4438_d_n4, assign6420_e4438_d_n5, assign6420_e4438_d_n6, assign6420_e4438_d_n8, assign6420_e4438_d_n10, assign6420_e4438_d_n11, assign6420_e4438_d_n12,) = {
    if ((locals.var_guard63 == 0.0) && (locals.var_guard66 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn8, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12,)
    }
};
        locals.var_t6 = assign6420_e4438;
        locals.var_t6_dn0 = assign6420_e4438_d_n0;
        locals.var_t6_dn2 = assign6420_e4438_d_n2;
        locals.var_t6_dn4 = assign6420_e4438_d_n4;
        locals.var_t6_dn5 = assign6420_e4438_d_n5;
        locals.var_t6_dn6 = assign6420_e4438_d_n6;
        locals.var_t6_dn8 = assign6420_e4438_d_n8;
        locals.var_t6_dn10 = assign6420_e4438_d_n10;
        locals.var_t6_dn11 = assign6420_e4438_d_n11;
        locals.var_t6_dn12 = assign6420_e4438_d_n12;
        locals.var_t6_rv = 0.0;

        let (assign6430_e4445, assign6430_e4445_d_n0, assign6430_e4445_d_n2, assign6430_e4445_d_n4, assign6430_e4445_d_n5, assign6430_e4445_d_n6, assign6430_e4445_d_n8, assign6430_e4445_d_n10, assign6430_e4445_d_n11, assign6430_e4445_d_n12,) = {
    if (locals.var_guard63 == 0.0) {
        let assign6430_e4443: f64 = (locals.var_dppg * locals.var_cnstpgd);
        (assign6430_e4443, (locals.var_dppg_dn0 * locals.var_cnstpgd), (locals.var_dppg_dn2 * locals.var_cnstpgd), (locals.var_dppg_dn4 * locals.var_cnstpgd), (locals.var_dppg_dn5 * locals.var_cnstpgd), (locals.var_dppg_dn6 * locals.var_cnstpgd), (locals.var_dppg_dn8 * locals.var_cnstpgd), (locals.var_dppg_dn10 * locals.var_cnstpgd), (locals.var_dppg_dn11 * locals.var_cnstpgd), (locals.var_dppg_dn12 * locals.var_cnstpgd),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn8, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12,)
    }
};
        locals.var_dppg = assign6430_e4445;
        locals.var_dppg_dn0 = assign6430_e4445_d_n0;
        locals.var_dppg_dn2 = assign6430_e4445_d_n2;
        locals.var_dppg_dn4 = assign6430_e4445_d_n4;
        locals.var_dppg_dn5 = assign6430_e4445_d_n5;
        locals.var_dppg_dn6 = assign6430_e4445_d_n6;
        locals.var_dppg_dn8 = assign6430_e4445_d_n8;
        locals.var_dppg_dn10 = assign6430_e4445_d_n10;
        locals.var_dppg_dn11 = assign6430_e4445_d_n11;
        locals.var_dppg_dn12 = assign6430_e4445_d_n12;
        locals.var_dppg_rv = 0.0;

        let (assign6440_e4454, assign6440_e4454_d_n0, assign6440_e4454_d_n2, assign6440_e4454_d_n4, assign6440_e4454_d_n5, assign6440_e4454_d_n6, assign6440_e4454_d_n8, assign6440_e4454_d_n10, assign6440_e4454_d_n11, assign6440_e4454_d_n12,) = {
    if (locals.var_guard63 == 0.0) {
        let assign6440_e4450: f64 = (1.0 - locals.var_dppg);
        let assign6440_e4452: f64 = (assign6440_e4450 - 0.05);
        (assign6440_e4452, (-locals.var_dppg_dn0), (-locals.var_dppg_dn2), (-locals.var_dppg_dn4), (-locals.var_dppg_dn5), (-locals.var_dppg_dn6), (-locals.var_dppg_dn8), (-locals.var_dppg_dn10), (-locals.var_dppg_dn11), (-locals.var_dppg_dn12),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn8, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12,)
    }
};
        locals.var_tmf1 = assign6440_e4454;
        locals.var_tmf1_dn0 = assign6440_e4454_d_n0;
        locals.var_tmf1_dn2 = assign6440_e4454_d_n2;
        locals.var_tmf1_dn4 = assign6440_e4454_d_n4;
        locals.var_tmf1_dn5 = assign6440_e4454_d_n5;
        locals.var_tmf1_dn6 = assign6440_e4454_d_n6;
        locals.var_tmf1_dn8 = assign6440_e4454_d_n8;
        locals.var_tmf1_dn10 = assign6440_e4454_d_n10;
        locals.var_tmf1_dn11 = assign6440_e4454_d_n11;
        locals.var_tmf1_dn12 = assign6440_e4454_d_n12;
        locals.var_tmf1_rv = 0.0;

        let (assign6450_e4463, assign6450_e4463_d_n0, assign6450_e4463_d_n2, assign6450_e4463_d_n4, assign6450_e4463_d_n5, assign6450_e4463_d_n6, assign6450_e4463_d_n8, assign6450_e4463_d_n10, assign6450_e4463_d_n11, assign6450_e4463_d_n12,) = {
    if (locals.var_guard63 == 0.0) {
        let assign6450_e4459: f64 = 4.0;
        let assign6450_e4461: f64 = (assign6450_e4459 * 0.05);
        (assign6450_e4461, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign6450_e4463;
        locals.var_tmf2_dn0 = assign6450_e4463_d_n0;
        locals.var_tmf2_dn2 = assign6450_e4463_d_n2;
        locals.var_tmf2_dn4 = assign6450_e4463_d_n4;
        locals.var_tmf2_dn5 = assign6450_e4463_d_n5;
        locals.var_tmf2_dn6 = assign6450_e4463_d_n6;
        locals.var_tmf2_dn8 = assign6450_e4463_d_n8;
        locals.var_tmf2_dn10 = assign6450_e4463_d_n10;
        locals.var_tmf2_dn11 = assign6450_e4463_d_n11;
        locals.var_tmf2_dn12 = assign6450_e4463_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign6460_e4474, assign6460_e4474_d_n0, assign6460_e4474_d_n2, assign6460_e4474_d_n4, assign6460_e4474_d_n5, assign6460_e4474_d_n6, assign6460_e4474_d_n8, assign6460_e4474_d_n10, assign6460_e4474_d_n11, assign6460_e4474_d_n12,) = {
    if (locals.var_guard63 == 0.0) {
        let (assign6460_e4472, assign6460_e4472_d_n0, assign6460_e4472_d_n2, assign6460_e4472_d_n4, assign6460_e4472_d_n5, assign6460_e4472_d_n6, assign6460_e4472_d_n8, assign6460_e4472_d_n10, assign6460_e4472_d_n11, assign6460_e4472_d_n12,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
            } else {
                let assign6460_e4471: f64 = (-locals.var_tmf2);
                (assign6460_e4471, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12),)
            }
        };
        (assign6460_e4472, assign6460_e4472_d_n0, assign6460_e4472_d_n2, assign6460_e4472_d_n4, assign6460_e4472_d_n5, assign6460_e4472_d_n6, assign6460_e4472_d_n8, assign6460_e4472_d_n10, assign6460_e4472_d_n11, assign6460_e4472_d_n12,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign6460_e4474;
        locals.var_tmf2_dn0 = assign6460_e4474_d_n0;
        locals.var_tmf2_dn2 = assign6460_e4474_d_n2;
        locals.var_tmf2_dn4 = assign6460_e4474_d_n4;
        locals.var_tmf2_dn5 = assign6460_e4474_d_n5;
        locals.var_tmf2_dn6 = assign6460_e4474_d_n6;
        locals.var_tmf2_dn8 = assign6460_e4474_d_n8;
        locals.var_tmf2_dn10 = assign6460_e4474_d_n10;
        locals.var_tmf2_dn11 = assign6460_e4474_d_n11;
        locals.var_tmf2_dn12 = assign6460_e4474_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign6470_e4484, assign6470_e4484_d_n0, assign6470_e4484_d_n2, assign6470_e4484_d_n4, assign6470_e4484_d_n5, assign6470_e4484_d_n6, assign6470_e4484_d_n8, assign6470_e4484_d_n10, assign6470_e4484_d_n11, assign6470_e4484_d_n12,) = {
    if (locals.var_guard63 == 0.0) {
        let assign6470_e4479: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign6470_e4481: f64 = (assign6470_e4479 + locals.var_tmf2);
        let assign6470_e4482: f64 = (assign6470_e4481).sqrt();
        (assign6470_e4482, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign6470_e4482)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign6470_e4482)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign6470_e4482)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign6470_e4482)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign6470_e4482)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign6470_e4482)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign6470_e4482)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign6470_e4482)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign6470_e4482)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign6470_e4484;
        locals.var_tmf2_dn0 = assign6470_e4484_d_n0;
        locals.var_tmf2_dn2 = assign6470_e4484_d_n2;
        locals.var_tmf2_dn4 = assign6470_e4484_d_n4;
        locals.var_tmf2_dn5 = assign6470_e4484_d_n5;
        locals.var_tmf2_dn6 = assign6470_e4484_d_n6;
        locals.var_tmf2_dn8 = assign6470_e4484_d_n8;
        locals.var_tmf2_dn10 = assign6470_e4484_d_n10;
        locals.var_tmf2_dn11 = assign6470_e4484_d_n11;
        locals.var_tmf2_dn12 = assign6470_e4484_d_n12;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_17(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6480_e4495, assign6480_e4495_d_n0, assign6480_e4495_d_n2, assign6480_e4495_d_n4, assign6480_e4495_d_n5, assign6480_e4495_d_n6, assign6480_e4495_d_n8, assign6480_e4495_d_n10, assign6480_e4495_d_n11, assign6480_e4495_d_n12,) = {
    if (locals.var_guard63 == 0.0) {
        let assign6480_e4491: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign6480_e4492: f64 = (1.0 + assign6480_e4491);
        let assign6480_e4493: f64 = (0.5 * assign6480_e4492);
        (assign6480_e4493, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn8, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign6480_e4495;
        locals.var_t9_dn0 = assign6480_e4495_d_n0;
        locals.var_t9_dn2 = assign6480_e4495_d_n2;
        locals.var_t9_dn4 = assign6480_e4495_d_n4;
        locals.var_t9_dn5 = assign6480_e4495_d_n5;
        locals.var_t9_dn6 = assign6480_e4495_d_n6;
        locals.var_t9_dn8 = assign6480_e4495_d_n8;
        locals.var_t9_dn10 = assign6480_e4495_d_n10;
        locals.var_t9_dn11 = assign6480_e4495_d_n11;
        locals.var_t9_dn12 = assign6480_e4495_d_n12;
        locals.var_t9_rv = 0.0;

        let (assign6490_e4506, assign6490_e4506_d_n0, assign6490_e4506_d_n2, assign6490_e4506_d_n4, assign6490_e4506_d_n5, assign6490_e4506_d_n6, assign6490_e4506_d_n8, assign6490_e4506_d_n10, assign6490_e4506_d_n11, assign6490_e4506_d_n12,) = {
    if (locals.var_guard63 == 0.0) {
        let assign6490_e4502: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign6490_e4503: f64 = (0.5 * assign6490_e4502);
        let assign6490_e4504: f64 = (1.0 - assign6490_e4503);
        (assign6490_e4504, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn8, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn12,)
    }
};
        locals.var_dppg = assign6490_e4506;
        locals.var_dppg_dn0 = assign6490_e4506_d_n0;
        locals.var_dppg_dn2 = assign6490_e4506_d_n2;
        locals.var_dppg_dn4 = assign6490_e4506_d_n4;
        locals.var_dppg_dn5 = assign6490_e4506_d_n5;
        locals.var_dppg_dn6 = assign6490_e4506_d_n6;
        locals.var_dppg_dn8 = assign6490_e4506_d_n8;
        locals.var_dppg_dn10 = assign6490_e4506_d_n10;
        locals.var_dppg_dn11 = assign6490_e4506_d_n11;
        locals.var_dppg_dn12 = assign6490_e4506_d_n12;
        locals.var_dppg_rv = 0.0;

        let assign6500_e4509: f64 = (locals.var_vgs - locals.var_vfb);
        let assign6500_e4511: f64 = (assign6500_e4509 + locals.var_dvth);
        let assign6500_e4513: f64 = (assign6500_e4511 - locals.var_dppg);
        locals.var_vgp = assign6500_e4513;
        locals.var_vgp_dn0 = (((-locals.var_vfb_dn0) + locals.var_dvth_dn0) - locals.var_dppg_dn0);
        locals.var_vgp_dn2 = (((-locals.var_vfb_dn2) + locals.var_dvth_dn2) - locals.var_dppg_dn2);
        locals.var_vgp_dn4 = (((-locals.var_vfb_dn4) + locals.var_dvth_dn4) - locals.var_dppg_dn4);
        locals.var_vgp_dn5 = (((locals.var_vgs_dn5 - locals.var_vfb_dn5) + locals.var_dvth_dn5) - locals.var_dppg_dn5);
        locals.var_vgp_dn6 = (((-locals.var_vfb_dn6) + locals.var_dvth_dn6) - locals.var_dppg_dn6);
        locals.var_vgp_dn8 = (((-locals.var_vfb_dn8) + locals.var_dvth_dn8) - locals.var_dppg_dn8);
        locals.var_vgp_dn10 = (((-locals.var_vfb_dn10) + locals.var_dvth_dn10) - locals.var_dppg_dn10);
        locals.var_vgp_dn11 = (((locals.var_vgs_dn11 - locals.var_vfb_dn11) + locals.var_dvth_dn11) - locals.var_dppg_dn11);
        locals.var_vgp_dn12 = (((locals.var_vgs_dn12 - locals.var_vfb_dn12) + locals.var_dvth_dn12) - locals.var_dppg_dn12);
        locals.var_vgp_rv = 0.0;

        locals.var_vgpz = locals.var_vgp;
        locals.var_vgpz_dn0 = locals.var_vgp_dn0;
        locals.var_vgpz_dn2 = locals.var_vgp_dn2;
        locals.var_vgpz_dn4 = locals.var_vgp_dn4;
        locals.var_vgpz_dn5 = locals.var_vgp_dn5;
        locals.var_vgpz_dn6 = locals.var_vgp_dn6;
        locals.var_vgpz_dn8 = locals.var_vgp_dn8;
        locals.var_vgpz_dn10 = locals.var_vgp_dn10;
        locals.var_vgpz_dn11 = locals.var_vgp_dn11;
        locals.var_vgpz_dn12 = locals.var_vgp_dn12;
        locals.var_vgpz_rv = 0.0;

        let assign6520_e4518: f64 = (locals.var_uc_nsubs / locals.var_n_subbl);
        let assign6520_e4519: f64 = (assign6520_e4518).ln();
        let assign6520_e4520: f64 = (locals.var_beta_inv * assign6520_e4519);
        locals.var_vbi_soi = assign6520_e4520;
        locals.var_vbi_soi_dn0 = (locals.var_beta_inv * ((((locals.var_uc_nsubs_dn0 * locals.var_n_subbl) - (locals.var_uc_nsubs * locals.var_n_subbl_dn0)) / (locals.var_n_subbl * locals.var_n_subbl)) / assign6520_e4518));
        locals.var_vbi_soi_dn2 = (locals.var_beta_inv * ((((locals.var_uc_nsubs_dn2 * locals.var_n_subbl) - (locals.var_uc_nsubs * locals.var_n_subbl_dn2)) / (locals.var_n_subbl * locals.var_n_subbl)) / assign6520_e4518));
        locals.var_vbi_soi_dn4 = ((locals.var_beta_inv_dn4 * assign6520_e4519) + (locals.var_beta_inv * ((((locals.var_uc_nsubs_dn4 * locals.var_n_subbl) - (locals.var_uc_nsubs * locals.var_n_subbl_dn4)) / (locals.var_n_subbl * locals.var_n_subbl)) / assign6520_e4518)));
        locals.var_vbi_soi_dn5 = (locals.var_beta_inv * ((((locals.var_uc_nsubs_dn5 * locals.var_n_subbl) - (locals.var_uc_nsubs * locals.var_n_subbl_dn5)) / (locals.var_n_subbl * locals.var_n_subbl)) / assign6520_e4518));
        locals.var_vbi_soi_dn6 = (locals.var_beta_inv * ((((locals.var_uc_nsubs_dn6 * locals.var_n_subbl) - (locals.var_uc_nsubs * locals.var_n_subbl_dn6)) / (locals.var_n_subbl * locals.var_n_subbl)) / assign6520_e4518));
        locals.var_vbi_soi_dn8 = (locals.var_beta_inv * ((((locals.var_uc_nsubs_dn8 * locals.var_n_subbl) - (locals.var_uc_nsubs * locals.var_n_subbl_dn8)) / (locals.var_n_subbl * locals.var_n_subbl)) / assign6520_e4518));
        locals.var_vbi_soi_dn10 = (locals.var_beta_inv * ((((locals.var_uc_nsubs_dn10 * locals.var_n_subbl) - (locals.var_uc_nsubs * locals.var_n_subbl_dn10)) / (locals.var_n_subbl * locals.var_n_subbl)) / assign6520_e4518));
        locals.var_vbi_soi_dn11 = (locals.var_beta_inv * ((((locals.var_uc_nsubs_dn11 * locals.var_n_subbl) - (locals.var_uc_nsubs * locals.var_n_subbl_dn11)) / (locals.var_n_subbl * locals.var_n_subbl)) / assign6520_e4518));
        locals.var_vbi_soi_dn12 = (locals.var_beta_inv * ((((locals.var_uc_nsubs_dn12 * locals.var_n_subbl) - (locals.var_uc_nsubs * locals.var_n_subbl_dn12)) / (locals.var_n_subbl * locals.var_n_subbl)) / assign6520_e4518));
        locals.var_vbi_soi_rv = 0.0;

        let assign6530_e4523: f64 = (locals.var_vfb - locals.var_dvth);
        let assign6530_e4525: f64 = (assign6530_e4523 + locals.var_dppg);
        locals.var_vgs_fb = assign6530_e4525;
        locals.var_vgs_fb_dn0 = ((locals.var_vfb_dn0 - locals.var_dvth_dn0) + locals.var_dppg_dn0);
        locals.var_vgs_fb_dn2 = ((locals.var_vfb_dn2 - locals.var_dvth_dn2) + locals.var_dppg_dn2);
        locals.var_vgs_fb_dn4 = ((locals.var_vfb_dn4 - locals.var_dvth_dn4) + locals.var_dppg_dn4);
        locals.var_vgs_fb_dn5 = ((locals.var_vfb_dn5 - locals.var_dvth_dn5) + locals.var_dppg_dn5);
        locals.var_vgs_fb_dn6 = ((locals.var_vfb_dn6 - locals.var_dvth_dn6) + locals.var_dppg_dn6);
        locals.var_vgs_fb_dn8 = ((locals.var_vfb_dn8 - locals.var_dvth_dn8) + locals.var_dppg_dn8);
        locals.var_vgs_fb_dn10 = ((locals.var_vfb_dn10 - locals.var_dvth_dn10) + locals.var_dppg_dn10);
        locals.var_vgs_fb_dn11 = ((locals.var_vfb_dn11 - locals.var_dvth_dn11) + locals.var_dppg_dn11);
        locals.var_vgs_fb_dn12 = ((locals.var_vfb_dn12 - locals.var_dvth_dn12) + locals.var_dppg_dn12);
        locals.var_vgs_fb_rv = 0.0;

        let assign6540_e4528: f64 = (locals.var_cnst0soi * locals.var_c_fox_inv);
        locals.var_fac1 = assign6540_e4528;
        locals.var_fac1_dn0 = ((locals.var_cnst0soi_dn0 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn0));
        locals.var_fac1_dn2 = ((locals.var_cnst0soi_dn2 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn2));
        locals.var_fac1_dn4 = ((locals.var_cnst0soi_dn4 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn4));
        locals.var_fac1_dn5 = ((locals.var_cnst0soi_dn5 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn5));
        locals.var_fac1_dn6 = ((locals.var_cnst0soi_dn6 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn6));
        locals.var_fac1_dn8 = ((locals.var_cnst0soi_dn8 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn8));
        locals.var_fac1_dn10 = ((locals.var_cnst0soi_dn10 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn10));
        locals.var_fac1_dn11 = ((locals.var_cnst0soi_dn11 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn11));
        locals.var_fac1_dn12 = ((locals.var_cnst0soi_dn12 * locals.var_c_fox_inv) + (locals.var_cnst0soi * locals.var_c_fox_inv_dn12));
        locals.var_fac1_rv = 0.0;

        let assign6550_e4531: f64 = (locals.var_fac1 * locals.var_fac1);
        locals.var_fac1p2 = assign6550_e4531;
        locals.var_fac1p2_dn0 = ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0));
        locals.var_fac1p2_dn2 = ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2));
        locals.var_fac1p2_dn4 = ((locals.var_fac1_dn4 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn4));
        locals.var_fac1p2_dn5 = ((locals.var_fac1_dn5 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn5));
        locals.var_fac1p2_dn6 = ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6));
        locals.var_fac1p2_dn8 = ((locals.var_fac1_dn8 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn8));
        locals.var_fac1p2_dn10 = ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10));
        locals.var_fac1p2_dn11 = ((locals.var_fac1_dn11 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn11));
        locals.var_fac1p2_dn12 = ((locals.var_fac1_dn12 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn12));
        locals.var_fac1p2_rv = 0.0;

        let (assign6560_e4537, assign6560_e4537_d_n0, assign6560_e4537_d_n2, assign6560_e4537_d_n4, assign6560_e4537_d_n5, assign6560_e4537_d_n6, assign6560_e4537_d_n8, assign6560_e4537_d_n10, assign6560_e4537_d_n11, assign6560_e4537_d_n12,) = {
    if (p.p29 != 0.0) {
        let assign6560_e4535: f64 = (locals.var_vbsz + locals.var_vbi_soi);
        (assign6560_e4535, (locals.var_vbsz_dn0 + locals.var_vbi_soi_dn0), (locals.var_vbsz_dn2 + locals.var_vbi_soi_dn2), (locals.var_vbsz_dn4 + locals.var_vbi_soi_dn4), (locals.var_vbsz_dn5 + locals.var_vbi_soi_dn5), (locals.var_vbsz_dn6 + locals.var_vbi_soi_dn6), (locals.var_vbsz_dn8 + locals.var_vbi_soi_dn8), (locals.var_vbsz_dn10 + locals.var_vbi_soi_dn10), (locals.var_vbsz_dn11 + locals.var_vbi_soi_dn11), (locals.var_vbsz_dn12 + locals.var_vbi_soi_dn12),)
    } else {
        (locals.var_vbsbiz, locals.var_vbsbiz_dn0, locals.var_vbsbiz_dn2, locals.var_vbsbiz_dn4, locals.var_vbsbiz_dn5, locals.var_vbsbiz_dn6, locals.var_vbsbiz_dn8, locals.var_vbsbiz_dn10, locals.var_vbsbiz_dn11, locals.var_vbsbiz_dn12,)
    }
};
        locals.var_vbsbiz = assign6560_e4537;
        locals.var_vbsbiz_dn0 = assign6560_e4537_d_n0;
        locals.var_vbsbiz_dn2 = assign6560_e4537_d_n2;
        locals.var_vbsbiz_dn4 = assign6560_e4537_d_n4;
        locals.var_vbsbiz_dn5 = assign6560_e4537_d_n5;
        locals.var_vbsbiz_dn6 = assign6560_e4537_d_n6;
        locals.var_vbsbiz_dn8 = assign6560_e4537_d_n8;
        locals.var_vbsbiz_dn10 = assign6560_e4537_d_n10;
        locals.var_vbsbiz_dn11 = assign6560_e4537_d_n11;
        locals.var_vbsbiz_dn12 = assign6560_e4537_d_n12;
        locals.var_vbsbiz_rv = 0.0;

        let (assign6570_e4544, assign6570_e4544_d_n0, assign6570_e4544_d_n2, assign6570_e4544_d_n4, assign6570_e4544_d_n5, assign6570_e4544_d_n6, assign6570_e4544_d_n8, assign6570_e4544_d_n10, assign6570_e4544_d_n11, assign6570_e4544_d_n12,) = {
    if (p.p29 == 0.0) {
        let assign6570_e4542: f64 = (locals.var_vbs + locals.var_vbi_soi);
        (assign6570_e4542, (locals.var_vbs_dn0 + locals.var_vbi_soi_dn0), (locals.var_vbs_dn2 + locals.var_vbi_soi_dn2), (locals.var_vbs_dn4 + locals.var_vbi_soi_dn4), (locals.var_vbs_dn5 + locals.var_vbi_soi_dn5), (locals.var_vbs_dn6 + locals.var_vbi_soi_dn6), (locals.var_vbs_dn8 + locals.var_vbi_soi_dn8), (locals.var_vbs_dn10 + locals.var_vbi_soi_dn10), (locals.var_vbs_dn11 + locals.var_vbi_soi_dn11), (locals.var_vbs_dn12 + locals.var_vbi_soi_dn12),)
    } else {
        (locals.var_vbsbiz, locals.var_vbsbiz_dn0, locals.var_vbsbiz_dn2, locals.var_vbsbiz_dn4, locals.var_vbsbiz_dn5, locals.var_vbsbiz_dn6, locals.var_vbsbiz_dn8, locals.var_vbsbiz_dn10, locals.var_vbsbiz_dn11, locals.var_vbsbiz_dn12,)
    }
};
        locals.var_vbsbiz = assign6570_e4544;
        locals.var_vbsbiz_dn0 = assign6570_e4544_d_n0;
        locals.var_vbsbiz_dn2 = assign6570_e4544_d_n2;
        locals.var_vbsbiz_dn4 = assign6570_e4544_d_n4;
        locals.var_vbsbiz_dn5 = assign6570_e4544_d_n5;
        locals.var_vbsbiz_dn6 = assign6570_e4544_d_n6;
        locals.var_vbsbiz_dn8 = assign6570_e4544_d_n8;
        locals.var_vbsbiz_dn10 = assign6570_e4544_d_n10;
        locals.var_vbsbiz_dn11 = assign6570_e4544_d_n11;
        locals.var_vbsbiz_dn12 = assign6570_e4544_d_n12;
        locals.var_vbsbiz_rv = 0.0;

        let assign6580_e4547: f64 = if locals.var_vbsbiz < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard67 = assign6580_e4547;
        locals.var_guard67_rv = 0.0;

        let (assign6590_e4553, assign6590_e4553_d_n0, assign6590_e4553_d_n2, assign6590_e4553_d_n4, assign6590_e4553_d_n5, assign6590_e4553_d_n6, assign6590_e4553_d_n8, assign6590_e4553_d_n10, assign6590_e4553_d_n11, assign6590_e4553_d_n12,) = {
    if (locals.var_guard67 != 0.0) {
        let assign6590_e4551: f64 = (locals.var_n_subbl / locals.var_uc_nsubs);
        (assign6590_e4551, (((locals.var_n_subbl_dn0 * locals.var_uc_nsubs) - (locals.var_n_subbl * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), (((locals.var_n_subbl_dn2 * locals.var_uc_nsubs) - (locals.var_n_subbl * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), (((locals.var_n_subbl_dn4 * locals.var_uc_nsubs) - (locals.var_n_subbl * locals.var_uc_nsubs_dn4)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), (((locals.var_n_subbl_dn5 * locals.var_uc_nsubs) - (locals.var_n_subbl * locals.var_uc_nsubs_dn5)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), (((locals.var_n_subbl_dn6 * locals.var_uc_nsubs) - (locals.var_n_subbl * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), (((locals.var_n_subbl_dn8 * locals.var_uc_nsubs) - (locals.var_n_subbl * locals.var_uc_nsubs_dn8)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), (((locals.var_n_subbl_dn10 * locals.var_uc_nsubs) - (locals.var_n_subbl * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), (((locals.var_n_subbl_dn11 * locals.var_uc_nsubs) - (locals.var_n_subbl * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)), (((locals.var_n_subbl_dn12 * locals.var_uc_nsubs) - (locals.var_n_subbl * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign6590_e4553;
        locals.var_t0_dn0 = assign6590_e4553_d_n0;
        locals.var_t0_dn2 = assign6590_e4553_d_n2;
        locals.var_t0_dn4 = assign6590_e4553_d_n4;
        locals.var_t0_dn5 = assign6590_e4553_d_n5;
        locals.var_t0_dn6 = assign6590_e4553_d_n6;
        locals.var_t0_dn8 = assign6590_e4553_d_n8;
        locals.var_t0_dn10 = assign6590_e4553_d_n10;
        locals.var_t0_dn11 = assign6590_e4553_d_n11;
        locals.var_t0_dn12 = assign6590_e4553_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign6600_e4559, assign6600_e4559_d_n0, assign6600_e4559_d_n2, assign6600_e4559_d_n4, assign6600_e4559_d_n5, assign6600_e4559_d_n6, assign6600_e4559_d_n8, assign6600_e4559_d_n10, assign6600_e4559_d_n11, assign6600_e4559_d_n12,) = {
    if (locals.var_guard67 != 0.0) {
        let assign6600_e4557: f64 = (locals.var_t0 + 1.0);
        (assign6600_e4557, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign6600_e4559;
        locals.var_t1_dn0 = assign6600_e4559_d_n0;
        locals.var_t1_dn2 = assign6600_e4559_d_n2;
        locals.var_t1_dn4 = assign6600_e4559_d_n4;
        locals.var_t1_dn5 = assign6600_e4559_d_n5;
        locals.var_t1_dn6 = assign6600_e4559_d_n6;
        locals.var_t1_dn8 = assign6600_e4559_d_n8;
        locals.var_t1_dn10 = assign6600_e4559_d_n10;
        locals.var_t1_dn11 = assign6600_e4559_d_n11;
        locals.var_t1_dn12 = assign6600_e4559_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign6610_e4571, assign6610_e4571_d_n0, assign6610_e4571_d_n2, assign6610_e4571_d_n4, assign6610_e4571_d_n5, assign6610_e4571_d_n6, assign6610_e4571_d_n8, assign6610_e4571_d_n10, assign6610_e4571_d_n11, assign6610_e4571_d_n12,) = {
    if (locals.var_guard67 != 0.0) {
        let assign6610_e4563: f64 = (locals.var_beta_inv - locals.var_vbsbiz);
        let assign6610_e4567: f64 = (locals.var_beta_inv + locals.var_vbsbiz);
        let assign6610_e4568: f64 = (locals.var_t0 * assign6610_e4567);
        let assign6610_e4569: f64 = (assign6610_e4563 + assign6610_e4568);
        (assign6610_e4569, ((-locals.var_vbsbiz_dn0) + ((locals.var_t0_dn0 * assign6610_e4567) + (locals.var_t0 * locals.var_vbsbiz_dn0))), ((-locals.var_vbsbiz_dn2) + ((locals.var_t0_dn2 * assign6610_e4567) + (locals.var_t0 * locals.var_vbsbiz_dn2))), ((locals.var_beta_inv_dn4 - locals.var_vbsbiz_dn4) + ((locals.var_t0_dn4 * assign6610_e4567) + (locals.var_t0 * (locals.var_beta_inv_dn4 + locals.var_vbsbiz_dn4)))), ((-locals.var_vbsbiz_dn5) + ((locals.var_t0_dn5 * assign6610_e4567) + (locals.var_t0 * locals.var_vbsbiz_dn5))), ((-locals.var_vbsbiz_dn6) + ((locals.var_t0_dn6 * assign6610_e4567) + (locals.var_t0 * locals.var_vbsbiz_dn6))), ((-locals.var_vbsbiz_dn8) + ((locals.var_t0_dn8 * assign6610_e4567) + (locals.var_t0 * locals.var_vbsbiz_dn8))), ((-locals.var_vbsbiz_dn10) + ((locals.var_t0_dn10 * assign6610_e4567) + (locals.var_t0 * locals.var_vbsbiz_dn10))), ((-locals.var_vbsbiz_dn11) + ((locals.var_t0_dn11 * assign6610_e4567) + (locals.var_t0 * locals.var_vbsbiz_dn11))), ((-locals.var_vbsbiz_dn12) + ((locals.var_t0_dn12 * assign6610_e4567) + (locals.var_t0 * locals.var_vbsbiz_dn12))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign6610_e4571;
        locals.var_t2_dn0 = assign6610_e4571_d_n0;
        locals.var_t2_dn2 = assign6610_e4571_d_n2;
        locals.var_t2_dn4 = assign6610_e4571_d_n4;
        locals.var_t2_dn5 = assign6610_e4571_d_n5;
        locals.var_t2_dn6 = assign6610_e4571_d_n6;
        locals.var_t2_dn8 = assign6610_e4571_d_n8;
        locals.var_t2_dn10 = assign6610_e4571_d_n10;
        locals.var_t2_dn11 = assign6610_e4571_d_n11;
        locals.var_t2_dn12 = assign6610_e4571_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign6620_e4581, assign6620_e4581_d_n0, assign6620_e4581_d_n2, assign6620_e4581_d_n4, assign6620_e4581_d_n5, assign6620_e4581_d_n6, assign6620_e4581_d_n8, assign6620_e4581_d_n10, assign6620_e4581_d_n11, assign6620_e4581_d_n12,) = {
    if (locals.var_guard67 != 0.0) {
        let assign6620_e4575: f64 = (locals.var_cnst0bulk * locals.var_cnst0bulk);
        let assign6620_e4577: f64 = (assign6620_e4575 * locals.var_c_box_inv);
        let assign6620_e4579: f64 = (assign6620_e4577 * locals.var_c_box_inv);
        (assign6620_e4579, ((((locals.var_cnst0bulk_dn0 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn0)) * locals.var_c_box_inv) * locals.var_c_box_inv), ((((locals.var_cnst0bulk_dn2 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn2)) * locals.var_c_box_inv) * locals.var_c_box_inv), ((((locals.var_cnst0bulk_dn4 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn4)) * locals.var_c_box_inv) * locals.var_c_box_inv), ((((locals.var_cnst0bulk_dn5 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn5)) * locals.var_c_box_inv) * locals.var_c_box_inv), ((((locals.var_cnst0bulk_dn6 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn6)) * locals.var_c_box_inv) * locals.var_c_box_inv), ((((locals.var_cnst0bulk_dn8 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn8)) * locals.var_c_box_inv) * locals.var_c_box_inv), ((((locals.var_cnst0bulk_dn10 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn10)) * locals.var_c_box_inv) * locals.var_c_box_inv), ((((locals.var_cnst0bulk_dn11 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn11)) * locals.var_c_box_inv) * locals.var_c_box_inv), ((((locals.var_cnst0bulk_dn12 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn12)) * locals.var_c_box_inv) * locals.var_c_box_inv),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign6620_e4581;
        locals.var_t3_dn0 = assign6620_e4581_d_n0;
        locals.var_t3_dn2 = assign6620_e4581_d_n2;
        locals.var_t3_dn4 = assign6620_e4581_d_n4;
        locals.var_t3_dn5 = assign6620_e4581_d_n5;
        locals.var_t3_dn6 = assign6620_e4581_d_n6;
        locals.var_t3_dn8 = assign6620_e4581_d_n8;
        locals.var_t3_dn10 = assign6620_e4581_d_n10;
        locals.var_t3_dn11 = assign6620_e4581_d_n11;
        locals.var_t3_dn12 = assign6620_e4581_d_n12;
        locals.var_t3_rv = 0.0;

        let (assign6630_e4593, assign6630_e4593_d_n0, assign6630_e4593_d_n2, assign6630_e4593_d_n4, assign6630_e4593_d_n5, assign6630_e4593_d_n6, assign6630_e4593_d_n8, assign6630_e4593_d_n10, assign6630_e4593_d_n11, assign6630_e4593_d_n12,) = {
    if (locals.var_guard67 != 0.0) {
        let assign6630_e4585: f64 = (2.0 * locals.var_t2);
        let assign6630_e4587: f64 = (assign6630_e4585 * locals.var_t1);
        let assign6630_e4590: f64 = (locals.var_t3 * locals.var_beta);
        let assign6630_e4591: f64 = (assign6630_e4587 - assign6630_e4590);
        (assign6630_e4591, ((((2.0 * locals.var_t2_dn0) * locals.var_t1) + (assign6630_e4585 * locals.var_t1_dn0)) - (locals.var_t3_dn0 * locals.var_beta)), ((((2.0 * locals.var_t2_dn2) * locals.var_t1) + (assign6630_e4585 * locals.var_t1_dn2)) - (locals.var_t3_dn2 * locals.var_beta)), ((((2.0 * locals.var_t2_dn4) * locals.var_t1) + (assign6630_e4585 * locals.var_t1_dn4)) - ((locals.var_t3_dn4 * locals.var_beta) + (locals.var_t3 * locals.var_beta_dn4))), ((((2.0 * locals.var_t2_dn5) * locals.var_t1) + (assign6630_e4585 * locals.var_t1_dn5)) - (locals.var_t3_dn5 * locals.var_beta)), ((((2.0 * locals.var_t2_dn6) * locals.var_t1) + (assign6630_e4585 * locals.var_t1_dn6)) - (locals.var_t3_dn6 * locals.var_beta)), ((((2.0 * locals.var_t2_dn8) * locals.var_t1) + (assign6630_e4585 * locals.var_t1_dn8)) - (locals.var_t3_dn8 * locals.var_beta)), ((((2.0 * locals.var_t2_dn10) * locals.var_t1) + (assign6630_e4585 * locals.var_t1_dn10)) - (locals.var_t3_dn10 * locals.var_beta)), ((((2.0 * locals.var_t2_dn11) * locals.var_t1) + (assign6630_e4585 * locals.var_t1_dn11)) - (locals.var_t3_dn11 * locals.var_beta)), ((((2.0 * locals.var_t2_dn12) * locals.var_t1) + (assign6630_e4585 * locals.var_t1_dn12)) - (locals.var_t3_dn12 * locals.var_beta)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign6630_e4593;
        locals.var_t4_dn0 = assign6630_e4593_d_n0;
        locals.var_t4_dn2 = assign6630_e4593_d_n2;
        locals.var_t4_dn4 = assign6630_e4593_d_n4;
        locals.var_t4_dn5 = assign6630_e4593_d_n5;
        locals.var_t4_dn6 = assign6630_e4593_d_n6;
        locals.var_t4_dn8 = assign6630_e4593_d_n8;
        locals.var_t4_dn10 = assign6630_e4593_d_n10;
        locals.var_t4_dn11 = assign6630_e4593_d_n11;
        locals.var_t4_dn12 = assign6630_e4593_d_n12;
        locals.var_t4_rv = 0.0;

        let (assign6640_e4607, assign6640_e4607_d_n0, assign6640_e4607_d_n2, assign6640_e4607_d_n4, assign6640_e4607_d_n5, assign6640_e4607_d_n6, assign6640_e4607_d_n8, assign6640_e4607_d_n10, assign6640_e4607_d_n11, assign6640_e4607_d_n12,) = {
    if (locals.var_guard67 != 0.0) {
        let assign6640_e4597: f64 = (locals.var_t2 * locals.var_t2);
        let assign6640_e4600: f64 = (locals.var_t3 * locals.var_beta);
        let assign6640_e4602: f64 = (assign6640_e4600 * locals.var_vbsbiz);
        let assign6640_e4603: f64 = (assign6640_e4597 + assign6640_e4602);
        let assign6640_e4605: f64 = (assign6640_e4603 + locals.var_t3);
        (assign6640_e4605, ((((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) + (((locals.var_t3_dn0 * locals.var_beta) * locals.var_vbsbiz) + (assign6640_e4600 * locals.var_vbsbiz_dn0))) + locals.var_t3_dn0), ((((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) + (((locals.var_t3_dn2 * locals.var_beta) * locals.var_vbsbiz) + (assign6640_e4600 * locals.var_vbsbiz_dn2))) + locals.var_t3_dn2), ((((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) + ((((locals.var_t3_dn4 * locals.var_beta) + (locals.var_t3 * locals.var_beta_dn4)) * locals.var_vbsbiz) + (assign6640_e4600 * locals.var_vbsbiz_dn4))) + locals.var_t3_dn4), ((((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) + (((locals.var_t3_dn5 * locals.var_beta) * locals.var_vbsbiz) + (assign6640_e4600 * locals.var_vbsbiz_dn5))) + locals.var_t3_dn5), ((((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) + (((locals.var_t3_dn6 * locals.var_beta) * locals.var_vbsbiz) + (assign6640_e4600 * locals.var_vbsbiz_dn6))) + locals.var_t3_dn6), ((((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) + (((locals.var_t3_dn8 * locals.var_beta) * locals.var_vbsbiz) + (assign6640_e4600 * locals.var_vbsbiz_dn8))) + locals.var_t3_dn8), ((((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) + (((locals.var_t3_dn10 * locals.var_beta) * locals.var_vbsbiz) + (assign6640_e4600 * locals.var_vbsbiz_dn10))) + locals.var_t3_dn10), ((((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) + (((locals.var_t3_dn11 * locals.var_beta) * locals.var_vbsbiz) + (assign6640_e4600 * locals.var_vbsbiz_dn11))) + locals.var_t3_dn11), ((((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)) + (((locals.var_t3_dn12 * locals.var_beta) * locals.var_vbsbiz) + (assign6640_e4600 * locals.var_vbsbiz_dn12))) + locals.var_t3_dn12),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign6640_e4607;
        locals.var_t5_dn0 = assign6640_e4607_d_n0;
        locals.var_t5_dn2 = assign6640_e4607_d_n2;
        locals.var_t5_dn4 = assign6640_e4607_d_n4;
        locals.var_t5_dn5 = assign6640_e4607_d_n5;
        locals.var_t5_dn6 = assign6640_e4607_d_n6;
        locals.var_t5_dn8 = assign6640_e4607_d_n8;
        locals.var_t5_dn10 = assign6640_e4607_d_n10;
        locals.var_t5_dn11 = assign6640_e4607_d_n11;
        locals.var_t5_dn12 = assign6640_e4607_d_n12;
        locals.var_t5_rv = 0.0;

        let (assign6650_e4636, assign6650_e4636_d_n0, assign6650_e4636_d_n2, assign6650_e4636_d_n4, assign6650_e4636_d_n5, assign6650_e4636_d_n6, assign6650_e4636_d_n8, assign6650_e4636_d_n10, assign6650_e4636_d_n11, assign6650_e4636_d_n12,) = {
    if (locals.var_guard67 != 0.0) {
        let assign6650_e4611: f64 = (locals.var_t4 * locals.var_t4);
        let assign6650_e4614: f64 = (4.0 * locals.var_t1);
        let assign6650_e4616: f64 = (assign6650_e4614 * locals.var_t1);
        let assign6650_e4618: f64 = (assign6650_e4616 * locals.var_t5);
        let assign6650_e4619: f64 = (assign6650_e4611 - assign6650_e4618);
        let (assign6650_e4634, assign6650_e4634_d_n0, assign6650_e4634_d_n2, assign6650_e4634_d_n4, assign6650_e4634_d_n5, assign6650_e4634_d_n6, assign6650_e4634_d_n8, assign6650_e4634_d_n10, assign6650_e4634_d_n11, assign6650_e4634_d_n12,) = {
            if (assign6650_e4619 >= 1e-50) {
                let assign6650_e4624: f64 = (locals.var_t4 * locals.var_t4);
                let assign6650_e4627: f64 = (4.0 * locals.var_t1);
                let assign6650_e4629: f64 = (assign6650_e4627 * locals.var_t1);
                let assign6650_e4631: f64 = (assign6650_e4629 * locals.var_t5);
                let assign6650_e4632: f64 = (assign6650_e4624 - assign6650_e4631);
                (assign6650_e4632, (((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)) - (((((4.0 * locals.var_t1_dn0) * locals.var_t1) + (assign6650_e4627 * locals.var_t1_dn0)) * locals.var_t5) + (assign6650_e4629 * locals.var_t5_dn0))), (((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)) - (((((4.0 * locals.var_t1_dn2) * locals.var_t1) + (assign6650_e4627 * locals.var_t1_dn2)) * locals.var_t5) + (assign6650_e4629 * locals.var_t5_dn2))), (((locals.var_t4_dn4 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn4)) - (((((4.0 * locals.var_t1_dn4) * locals.var_t1) + (assign6650_e4627 * locals.var_t1_dn4)) * locals.var_t5) + (assign6650_e4629 * locals.var_t5_dn4))), (((locals.var_t4_dn5 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn5)) - (((((4.0 * locals.var_t1_dn5) * locals.var_t1) + (assign6650_e4627 * locals.var_t1_dn5)) * locals.var_t5) + (assign6650_e4629 * locals.var_t5_dn5))), (((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)) - (((((4.0 * locals.var_t1_dn6) * locals.var_t1) + (assign6650_e4627 * locals.var_t1_dn6)) * locals.var_t5) + (assign6650_e4629 * locals.var_t5_dn6))), (((locals.var_t4_dn8 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn8)) - (((((4.0 * locals.var_t1_dn8) * locals.var_t1) + (assign6650_e4627 * locals.var_t1_dn8)) * locals.var_t5) + (assign6650_e4629 * locals.var_t5_dn8))), (((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)) - (((((4.0 * locals.var_t1_dn10) * locals.var_t1) + (assign6650_e4627 * locals.var_t1_dn10)) * locals.var_t5) + (assign6650_e4629 * locals.var_t5_dn10))), (((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)) - (((((4.0 * locals.var_t1_dn11) * locals.var_t1) + (assign6650_e4627 * locals.var_t1_dn11)) * locals.var_t5) + (assign6650_e4629 * locals.var_t5_dn11))), (((locals.var_t4_dn12 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn12)) - (((((4.0 * locals.var_t1_dn12) * locals.var_t1) + (assign6650_e4627 * locals.var_t1_dn12)) * locals.var_t5) + (assign6650_e4629 * locals.var_t5_dn12))),)
            } else {
                (1e-50, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign6650_e4634, assign6650_e4634_d_n0, assign6650_e4634_d_n2, assign6650_e4634_d_n4, assign6650_e4634_d_n5, assign6650_e4634_d_n6, assign6650_e4634_d_n8, assign6650_e4634_d_n10, assign6650_e4634_d_n11, assign6650_e4634_d_n12,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn8, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12,)
    }
};
        locals.var_t7 = assign6650_e4636;
        locals.var_t7_dn0 = assign6650_e4636_d_n0;
        locals.var_t7_dn2 = assign6650_e4636_d_n2;
        locals.var_t7_dn4 = assign6650_e4636_d_n4;
        locals.var_t7_dn5 = assign6650_e4636_d_n5;
        locals.var_t7_dn6 = assign6650_e4636_d_n6;
        locals.var_t7_dn8 = assign6650_e4636_d_n8;
        locals.var_t7_dn10 = assign6650_e4636_d_n10;
        locals.var_t7_dn11 = assign6650_e4636_d_n11;
        locals.var_t7_dn12 = assign6650_e4636_d_n12;
        locals.var_t7_rv = 0.0;

        let (assign6660_e4649, assign6660_e4649_d_n0, assign6660_e4649_d_n2, assign6660_e4649_d_n4, assign6660_e4649_d_n5, assign6660_e4649_d_n6, assign6660_e4649_d_n8, assign6660_e4649_d_n10, assign6660_e4649_d_n11, assign6660_e4649_d_n12,) = {
    if (locals.var_guard67 != 0.0) {
        let assign6660_e4640: f64 = (locals.var_t7).sqrt();
        let assign6660_e4641: f64 = (locals.var_t4 + assign6660_e4640);
        let assign6660_e4645: f64 = (locals.var_t1 * locals.var_t1);
        let assign6660_e4646: f64 = (2.0 + assign6660_e4645);
        let assign6660_e4647: f64 = (assign6660_e4641 / assign6660_e4646);
        (assign6660_e4647, ((((locals.var_t4_dn0 + (locals.var_t7_dn0 / (2.0 * assign6660_e4640))) * assign6660_e4646) - (assign6660_e4641 * ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)))) / (assign6660_e4646 * assign6660_e4646)), ((((locals.var_t4_dn2 + (locals.var_t7_dn2 / (2.0 * assign6660_e4640))) * assign6660_e4646) - (assign6660_e4641 * ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)))) / (assign6660_e4646 * assign6660_e4646)), ((((locals.var_t4_dn4 + (locals.var_t7_dn4 / (2.0 * assign6660_e4640))) * assign6660_e4646) - (assign6660_e4641 * ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)))) / (assign6660_e4646 * assign6660_e4646)), ((((locals.var_t4_dn5 + (locals.var_t7_dn5 / (2.0 * assign6660_e4640))) * assign6660_e4646) - (assign6660_e4641 * ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)))) / (assign6660_e4646 * assign6660_e4646)), ((((locals.var_t4_dn6 + (locals.var_t7_dn6 / (2.0 * assign6660_e4640))) * assign6660_e4646) - (assign6660_e4641 * ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)))) / (assign6660_e4646 * assign6660_e4646)), ((((locals.var_t4_dn8 + (locals.var_t7_dn8 / (2.0 * assign6660_e4640))) * assign6660_e4646) - (assign6660_e4641 * ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)))) / (assign6660_e4646 * assign6660_e4646)), ((((locals.var_t4_dn10 + (locals.var_t7_dn10 / (2.0 * assign6660_e4640))) * assign6660_e4646) - (assign6660_e4641 * ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)))) / (assign6660_e4646 * assign6660_e4646)), ((((locals.var_t4_dn11 + (locals.var_t7_dn11 / (2.0 * assign6660_e4640))) * assign6660_e4646) - (assign6660_e4641 * ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)))) / (assign6660_e4646 * assign6660_e4646)), ((((locals.var_t4_dn12 + (locals.var_t7_dn12 / (2.0 * assign6660_e4640))) * assign6660_e4646) - (assign6660_e4641 * ((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)))) / (assign6660_e4646 * assign6660_e4646)),)
    } else {
        (locals.var_phi_s0_bulk_0, locals.var_phi_s0_bulk_0_dn0, locals.var_phi_s0_bulk_0_dn2, locals.var_phi_s0_bulk_0_dn4, locals.var_phi_s0_bulk_0_dn5, locals.var_phi_s0_bulk_0_dn6, locals.var_phi_s0_bulk_0_dn8, locals.var_phi_s0_bulk_0_dn10, locals.var_phi_s0_bulk_0_dn11, locals.var_phi_s0_bulk_0_dn12,)
    }
};
        locals.var_phi_s0_bulk_0 = assign6660_e4649;
        locals.var_phi_s0_bulk_0_dn0 = assign6660_e4649_d_n0;
        locals.var_phi_s0_bulk_0_dn2 = assign6660_e4649_d_n2;
        locals.var_phi_s0_bulk_0_dn4 = assign6660_e4649_d_n4;
        locals.var_phi_s0_bulk_0_dn5 = assign6660_e4649_d_n5;
        locals.var_phi_s0_bulk_0_dn6 = assign6660_e4649_d_n6;
        locals.var_phi_s0_bulk_0_dn8 = assign6660_e4649_d_n8;
        locals.var_phi_s0_bulk_0_dn10 = assign6660_e4649_d_n10;
        locals.var_phi_s0_bulk_0_dn11 = assign6660_e4649_d_n11;
        locals.var_phi_s0_bulk_0_dn12 = assign6660_e4649_d_n12;
        locals.var_phi_s0_bulk_0_rv = 0.0;

        let (assign6670_e4658, assign6670_e4658_d_n0, assign6670_e4658_d_n2, assign6670_e4658_d_n4, assign6670_e4658_d_n5, assign6670_e4658_d_n6, assign6670_e4658_d_n8, assign6670_e4658_d_n10, assign6670_e4658_d_n11, assign6670_e4658_d_n12,) = {
    if (locals.var_guard67 == 0.0) {
        let assign6670_e4654: f64 = (locals.var_cnst0bulk * locals.var_cnst0bulk);
        let assign6670_e4656: f64 = (assign6670_e4654 * locals.var_beta);
        (assign6670_e4656, (((locals.var_cnst0bulk_dn0 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn0)) * locals.var_beta), (((locals.var_cnst0bulk_dn2 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn2)) * locals.var_beta), ((((locals.var_cnst0bulk_dn4 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn4)) * locals.var_beta) + (assign6670_e4654 * locals.var_beta_dn4)), (((locals.var_cnst0bulk_dn5 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn5)) * locals.var_beta), (((locals.var_cnst0bulk_dn6 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn6)) * locals.var_beta), (((locals.var_cnst0bulk_dn8 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn8)) * locals.var_beta), (((locals.var_cnst0bulk_dn10 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn10)) * locals.var_beta), (((locals.var_cnst0bulk_dn11 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn11)) * locals.var_beta), (((locals.var_cnst0bulk_dn12 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn12)) * locals.var_beta),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign6670_e4658;
        locals.var_t1_dn0 = assign6670_e4658_d_n0;
        locals.var_t1_dn2 = assign6670_e4658_d_n2;
        locals.var_t1_dn4 = assign6670_e4658_d_n4;
        locals.var_t1_dn5 = assign6670_e4658_d_n5;
        locals.var_t1_dn6 = assign6670_e4658_d_n6;
        locals.var_t1_dn8 = assign6670_e4658_d_n8;
        locals.var_t1_dn10 = assign6670_e4658_d_n10;
        locals.var_t1_dn11 = assign6670_e4658_d_n11;
        locals.var_t1_dn12 = assign6670_e4658_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign6680_e4667, assign6680_e4667_d_n0, assign6680_e4667_d_n2, assign6680_e4667_d_n4, assign6680_e4667_d_n5, assign6680_e4667_d_n6, assign6680_e4667_d_n8, assign6680_e4667_d_n10, assign6680_e4667_d_n11, assign6680_e4667_d_n12,) = {
    if (locals.var_guard67 == 0.0) {
        let assign6680_e4663: f64 = (locals.var_cnst0soi * locals.var_cnst0soi);
        let assign6680_e4665: f64 = (assign6680_e4663 * locals.var_beta);
        (assign6680_e4665, (((locals.var_cnst0soi_dn0 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn0)) * locals.var_beta), (((locals.var_cnst0soi_dn2 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn2)) * locals.var_beta), ((((locals.var_cnst0soi_dn4 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn4)) * locals.var_beta) + (assign6680_e4663 * locals.var_beta_dn4)), (((locals.var_cnst0soi_dn5 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn5)) * locals.var_beta), (((locals.var_cnst0soi_dn6 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn6)) * locals.var_beta), (((locals.var_cnst0soi_dn8 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn8)) * locals.var_beta), (((locals.var_cnst0soi_dn10 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn10)) * locals.var_beta), (((locals.var_cnst0soi_dn11 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn11)) * locals.var_beta), (((locals.var_cnst0soi_dn12 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn12)) * locals.var_beta),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign6680_e4667;
        locals.var_t2_dn0 = assign6680_e4667_d_n0;
        locals.var_t2_dn2 = assign6680_e4667_d_n2;
        locals.var_t2_dn4 = assign6680_e4667_d_n4;
        locals.var_t2_dn5 = assign6680_e4667_d_n5;
        locals.var_t2_dn6 = assign6680_e4667_d_n6;
        locals.var_t2_dn8 = assign6680_e4667_d_n8;
        locals.var_t2_dn10 = assign6680_e4667_d_n10;
        locals.var_t2_dn11 = assign6680_e4667_d_n11;
        locals.var_t2_dn12 = assign6680_e4667_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign6690_e4677, assign6690_e4677_d_n0, assign6690_e4677_d_n2, assign6690_e4677_d_n4, assign6690_e4677_d_n5, assign6690_e4677_d_n6, assign6690_e4677_d_n8, assign6690_e4677_d_n10, assign6690_e4677_d_n11, assign6690_e4677_d_n12,) = {
    if (locals.var_guard67 == 0.0) {
        let assign6690_e4673: f64 = (2.0 * locals.var_vbsbiz);
        let assign6690_e4674: f64 = (locals.var_beta_inv + assign6690_e4673);
        let assign6690_e4675: f64 = (-assign6690_e4674);
        (assign6690_e4675, (-(2.0 * locals.var_vbsbiz_dn0)), (-(2.0 * locals.var_vbsbiz_dn2)), (-(locals.var_beta_inv_dn4 + (2.0 * locals.var_vbsbiz_dn4))), (-(2.0 * locals.var_vbsbiz_dn5)), (-(2.0 * locals.var_vbsbiz_dn6)), (-(2.0 * locals.var_vbsbiz_dn8)), (-(2.0 * locals.var_vbsbiz_dn10)), (-(2.0 * locals.var_vbsbiz_dn11)), (-(2.0 * locals.var_vbsbiz_dn12)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign6690_e4677;
        locals.var_t3_dn0 = assign6690_e4677_d_n0;
        locals.var_t3_dn2 = assign6690_e4677_d_n2;
        locals.var_t3_dn4 = assign6690_e4677_d_n4;
        locals.var_t3_dn5 = assign6690_e4677_d_n5;
        locals.var_t3_dn6 = assign6690_e4677_d_n6;
        locals.var_t3_dn8 = assign6690_e4677_d_n8;
        locals.var_t3_dn10 = assign6690_e4677_d_n10;
        locals.var_t3_dn11 = assign6690_e4677_d_n11;
        locals.var_t3_dn12 = assign6690_e4677_d_n12;
        locals.var_t3_rv = 0.0;

        let (assign6700_e4686, assign6700_e4686_d_n0, assign6700_e4686_d_n2, assign6700_e4686_d_n4, assign6700_e4686_d_n5, assign6700_e4686_d_n6, assign6700_e4686_d_n8, assign6700_e4686_d_n10, assign6700_e4686_d_n11, assign6700_e4686_d_n12,) = {
    if (locals.var_guard67 == 0.0) {
        let assign6700_e4683: f64 = (locals.var_t2 / locals.var_t1);
        let assign6700_e4684: f64 = (1.0 + assign6700_e4683);
        (assign6700_e4684, (((locals.var_t2_dn0 * locals.var_t1) - (locals.var_t2 * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1)), (((locals.var_t2_dn2 * locals.var_t1) - (locals.var_t2 * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1)), (((locals.var_t2_dn4 * locals.var_t1) - (locals.var_t2 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((locals.var_t2_dn5 * locals.var_t1) - (locals.var_t2 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (((locals.var_t2_dn6 * locals.var_t1) - (locals.var_t2 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), (((locals.var_t2_dn8 * locals.var_t1) - (locals.var_t2 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), (((locals.var_t2_dn10 * locals.var_t1) - (locals.var_t2 * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), (((locals.var_t2_dn11 * locals.var_t1) - (locals.var_t2 * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)), (((locals.var_t2_dn12 * locals.var_t1) - (locals.var_t2 * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign6700_e4686;
        locals.var_t4_dn0 = assign6700_e4686_d_n0;
        locals.var_t4_dn2 = assign6700_e4686_d_n2;
        locals.var_t4_dn4 = assign6700_e4686_d_n4;
        locals.var_t4_dn5 = assign6700_e4686_d_n5;
        locals.var_t4_dn6 = assign6700_e4686_d_n6;
        locals.var_t4_dn8 = assign6700_e4686_d_n8;
        locals.var_t4_dn10 = assign6700_e4686_d_n10;
        locals.var_t4_dn11 = assign6700_e4686_d_n11;
        locals.var_t4_dn12 = assign6700_e4686_d_n12;
        locals.var_t4_rv = 0.0;

        let (assign6710_e4697, assign6710_e4697_d_n0, assign6710_e4697_d_n2, assign6710_e4697_d_n4, assign6710_e4697_d_n5, assign6710_e4697_d_n6, assign6710_e4697_d_n8, assign6710_e4697_d_n10, assign6710_e4697_d_n11, assign6710_e4697_d_n12,) = {
    if (locals.var_guard67 == 0.0) {
        let assign6710_e4691: f64 = (locals.var_cnst0soi * locals.var_cnst0soi);
        let assign6710_e4693: f64 = (assign6710_e4691 * locals.var_c_box_inv);
        let assign6710_e4695: f64 = (assign6710_e4693 * locals.var_c_box_inv);
        (assign6710_e4695, ((((locals.var_cnst0soi_dn0 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn0)) * locals.var_c_box_inv) * locals.var_c_box_inv), ((((locals.var_cnst0soi_dn2 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn2)) * locals.var_c_box_inv) * locals.var_c_box_inv), ((((locals.var_cnst0soi_dn4 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn4)) * locals.var_c_box_inv) * locals.var_c_box_inv), ((((locals.var_cnst0soi_dn5 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn5)) * locals.var_c_box_inv) * locals.var_c_box_inv), ((((locals.var_cnst0soi_dn6 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn6)) * locals.var_c_box_inv) * locals.var_c_box_inv), ((((locals.var_cnst0soi_dn8 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn8)) * locals.var_c_box_inv) * locals.var_c_box_inv), ((((locals.var_cnst0soi_dn10 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn10)) * locals.var_c_box_inv) * locals.var_c_box_inv), ((((locals.var_cnst0soi_dn11 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn11)) * locals.var_c_box_inv) * locals.var_c_box_inv), ((((locals.var_cnst0soi_dn12 * locals.var_cnst0soi) + (locals.var_cnst0soi * locals.var_cnst0soi_dn12)) * locals.var_c_box_inv) * locals.var_c_box_inv),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign6710_e4697;
        locals.var_t5_dn0 = assign6710_e4697_d_n0;
        locals.var_t5_dn2 = assign6710_e4697_d_n2;
        locals.var_t5_dn4 = assign6710_e4697_d_n4;
        locals.var_t5_dn5 = assign6710_e4697_d_n5;
        locals.var_t5_dn6 = assign6710_e4697_d_n6;
        locals.var_t5_dn8 = assign6710_e4697_d_n8;
        locals.var_t5_dn10 = assign6710_e4697_d_n10;
        locals.var_t5_dn11 = assign6710_e4697_d_n11;
        locals.var_t5_dn12 = assign6710_e4697_d_n12;
        locals.var_t5_rv = 0.0;

        let (assign6720_e4710, assign6720_e4710_d_n0, assign6720_e4710_d_n2, assign6720_e4710_d_n4, assign6720_e4710_d_n5, assign6720_e4710_d_n6, assign6720_e4710_d_n8, assign6720_e4710_d_n10, assign6720_e4710_d_n11, assign6720_e4710_d_n12,) = {
    if (locals.var_guard67 == 0.0) {
        let assign6720_e4702: f64 = (locals.var_t5 * locals.var_beta);
        let assign6720_e4705: f64 = (2.0 * locals.var_t3);
        let assign6720_e4707: f64 = (assign6720_e4705 * locals.var_t4);
        let assign6720_e4708: f64 = (assign6720_e4702 - assign6720_e4707);
        (assign6720_e4708, ((locals.var_t5_dn0 * locals.var_beta) - (((2.0 * locals.var_t3_dn0) * locals.var_t4) + (assign6720_e4705 * locals.var_t4_dn0))), ((locals.var_t5_dn2 * locals.var_beta) - (((2.0 * locals.var_t3_dn2) * locals.var_t4) + (assign6720_e4705 * locals.var_t4_dn2))), (((locals.var_t5_dn4 * locals.var_beta) + (locals.var_t5 * locals.var_beta_dn4)) - (((2.0 * locals.var_t3_dn4) * locals.var_t4) + (assign6720_e4705 * locals.var_t4_dn4))), ((locals.var_t5_dn5 * locals.var_beta) - (((2.0 * locals.var_t3_dn5) * locals.var_t4) + (assign6720_e4705 * locals.var_t4_dn5))), ((locals.var_t5_dn6 * locals.var_beta) - (((2.0 * locals.var_t3_dn6) * locals.var_t4) + (assign6720_e4705 * locals.var_t4_dn6))), ((locals.var_t5_dn8 * locals.var_beta) - (((2.0 * locals.var_t3_dn8) * locals.var_t4) + (assign6720_e4705 * locals.var_t4_dn8))), ((locals.var_t5_dn10 * locals.var_beta) - (((2.0 * locals.var_t3_dn10) * locals.var_t4) + (assign6720_e4705 * locals.var_t4_dn10))), ((locals.var_t5_dn11 * locals.var_beta) - (((2.0 * locals.var_t3_dn11) * locals.var_t4) + (assign6720_e4705 * locals.var_t4_dn11))), ((locals.var_t5_dn12 * locals.var_beta) - (((2.0 * locals.var_t3_dn12) * locals.var_t4) + (assign6720_e4705 * locals.var_t4_dn12))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn8, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12,)
    }
};
        locals.var_t6 = assign6720_e4710;
        locals.var_t6_dn0 = assign6720_e4710_d_n0;
        locals.var_t6_dn2 = assign6720_e4710_d_n2;
        locals.var_t6_dn4 = assign6720_e4710_d_n4;
        locals.var_t6_dn5 = assign6720_e4710_d_n5;
        locals.var_t6_dn6 = assign6720_e4710_d_n6;
        locals.var_t6_dn8 = assign6720_e4710_d_n8;
        locals.var_t6_dn10 = assign6720_e4710_d_n10;
        locals.var_t6_dn11 = assign6720_e4710_d_n11;
        locals.var_t6_dn12 = assign6720_e4710_d_n12;
        locals.var_t6_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_18(
        locals: &mut StampLocals,
    ) {
        let (assign6730_e4744, assign6730_e4744_d_n0, assign6730_e4744_d_n2, assign6730_e4744_d_n4, assign6730_e4744_d_n5, assign6730_e4744_d_n6, assign6730_e4744_d_n8, assign6730_e4744_d_n10, assign6730_e4744_d_n11, assign6730_e4744_d_n12,) = {
    if (locals.var_guard67 == 0.0) {
        let assign6730_e4715: f64 = (locals.var_t6 * locals.var_t6);
        let assign6730_e4718: f64 = (4.0 * locals.var_t4);
        let assign6730_e4720: f64 = (assign6730_e4718 * locals.var_t4);
        let assign6730_e4722: f64 = (assign6730_e4720 * locals.var_t3);
        let assign6730_e4724: f64 = (assign6730_e4722 * locals.var_t3);
        let assign6730_e4725: f64 = (assign6730_e4715 - assign6730_e4724);
        let (assign6730_e4742, assign6730_e4742_d_n0, assign6730_e4742_d_n2, assign6730_e4742_d_n4, assign6730_e4742_d_n5, assign6730_e4742_d_n6, assign6730_e4742_d_n8, assign6730_e4742_d_n10, assign6730_e4742_d_n11, assign6730_e4742_d_n12,) = {
            if (assign6730_e4725 >= 1e-50) {
                let assign6730_e4730: f64 = (locals.var_t6 * locals.var_t6);
                let assign6730_e4733: f64 = (4.0 * locals.var_t4);
                let assign6730_e4735: f64 = (assign6730_e4733 * locals.var_t4);
                let assign6730_e4737: f64 = (assign6730_e4735 * locals.var_t3);
                let assign6730_e4739: f64 = (assign6730_e4737 * locals.var_t3);
                let assign6730_e4740: f64 = (assign6730_e4730 - assign6730_e4739);
                (assign6730_e4740, (((locals.var_t6_dn0 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn0)) - (((((((4.0 * locals.var_t4_dn0) * locals.var_t4) + (assign6730_e4733 * locals.var_t4_dn0)) * locals.var_t3) + (assign6730_e4735 * locals.var_t3_dn0)) * locals.var_t3) + (assign6730_e4737 * locals.var_t3_dn0))), (((locals.var_t6_dn2 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn2)) - (((((((4.0 * locals.var_t4_dn2) * locals.var_t4) + (assign6730_e4733 * locals.var_t4_dn2)) * locals.var_t3) + (assign6730_e4735 * locals.var_t3_dn2)) * locals.var_t3) + (assign6730_e4737 * locals.var_t3_dn2))), (((locals.var_t6_dn4 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn4)) - (((((((4.0 * locals.var_t4_dn4) * locals.var_t4) + (assign6730_e4733 * locals.var_t4_dn4)) * locals.var_t3) + (assign6730_e4735 * locals.var_t3_dn4)) * locals.var_t3) + (assign6730_e4737 * locals.var_t3_dn4))), (((locals.var_t6_dn5 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn5)) - (((((((4.0 * locals.var_t4_dn5) * locals.var_t4) + (assign6730_e4733 * locals.var_t4_dn5)) * locals.var_t3) + (assign6730_e4735 * locals.var_t3_dn5)) * locals.var_t3) + (assign6730_e4737 * locals.var_t3_dn5))), (((locals.var_t6_dn6 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn6)) - (((((((4.0 * locals.var_t4_dn6) * locals.var_t4) + (assign6730_e4733 * locals.var_t4_dn6)) * locals.var_t3) + (assign6730_e4735 * locals.var_t3_dn6)) * locals.var_t3) + (assign6730_e4737 * locals.var_t3_dn6))), (((locals.var_t6_dn8 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn8)) - (((((((4.0 * locals.var_t4_dn8) * locals.var_t4) + (assign6730_e4733 * locals.var_t4_dn8)) * locals.var_t3) + (assign6730_e4735 * locals.var_t3_dn8)) * locals.var_t3) + (assign6730_e4737 * locals.var_t3_dn8))), (((locals.var_t6_dn10 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn10)) - (((((((4.0 * locals.var_t4_dn10) * locals.var_t4) + (assign6730_e4733 * locals.var_t4_dn10)) * locals.var_t3) + (assign6730_e4735 * locals.var_t3_dn10)) * locals.var_t3) + (assign6730_e4737 * locals.var_t3_dn10))), (((locals.var_t6_dn11 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn11)) - (((((((4.0 * locals.var_t4_dn11) * locals.var_t4) + (assign6730_e4733 * locals.var_t4_dn11)) * locals.var_t3) + (assign6730_e4735 * locals.var_t3_dn11)) * locals.var_t3) + (assign6730_e4737 * locals.var_t3_dn11))), (((locals.var_t6_dn12 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn12)) - (((((((4.0 * locals.var_t4_dn12) * locals.var_t4) + (assign6730_e4733 * locals.var_t4_dn12)) * locals.var_t3) + (assign6730_e4735 * locals.var_t3_dn12)) * locals.var_t3) + (assign6730_e4737 * locals.var_t3_dn12))),)
            } else {
                (1e-50, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign6730_e4742, assign6730_e4742_d_n0, assign6730_e4742_d_n2, assign6730_e4742_d_n4, assign6730_e4742_d_n5, assign6730_e4742_d_n6, assign6730_e4742_d_n8, assign6730_e4742_d_n10, assign6730_e4742_d_n11, assign6730_e4742_d_n12,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn8, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12,)
    }
};
        locals.var_t7 = assign6730_e4744;
        locals.var_t7_dn0 = assign6730_e4744_d_n0;
        locals.var_t7_dn2 = assign6730_e4744_d_n2;
        locals.var_t7_dn4 = assign6730_e4744_d_n4;
        locals.var_t7_dn5 = assign6730_e4744_d_n5;
        locals.var_t7_dn6 = assign6730_e4744_d_n6;
        locals.var_t7_dn8 = assign6730_e4744_d_n8;
        locals.var_t7_dn10 = assign6730_e4744_d_n10;
        locals.var_t7_dn11 = assign6730_e4744_d_n11;
        locals.var_t7_dn12 = assign6730_e4744_d_n12;
        locals.var_t7_rv = 0.0;

        let (assign6740_e4758, assign6740_e4758_d_n0, assign6740_e4758_d_n2, assign6740_e4758_d_n4, assign6740_e4758_d_n5, assign6740_e4758_d_n6, assign6740_e4758_d_n8, assign6740_e4758_d_n10, assign6740_e4758_d_n11, assign6740_e4758_d_n12,) = {
    if (locals.var_guard67 == 0.0) {
        let assign6740_e4749: f64 = (locals.var_t7).sqrt();
        let assign6740_e4750: f64 = (locals.var_t6 + assign6740_e4749);
        let assign6740_e4753: f64 = (2.0 * locals.var_t4);
        let assign6740_e4755: f64 = (assign6740_e4753 * locals.var_t4);
        let assign6740_e4756: f64 = (assign6740_e4750 / assign6740_e4755);
        (assign6740_e4756, ((((locals.var_t6_dn0 + (locals.var_t7_dn0 / (2.0 * assign6740_e4749))) * assign6740_e4755) - (assign6740_e4750 * (((2.0 * locals.var_t4_dn0) * locals.var_t4) + (assign6740_e4753 * locals.var_t4_dn0)))) / (assign6740_e4755 * assign6740_e4755)), ((((locals.var_t6_dn2 + (locals.var_t7_dn2 / (2.0 * assign6740_e4749))) * assign6740_e4755) - (assign6740_e4750 * (((2.0 * locals.var_t4_dn2) * locals.var_t4) + (assign6740_e4753 * locals.var_t4_dn2)))) / (assign6740_e4755 * assign6740_e4755)), ((((locals.var_t6_dn4 + (locals.var_t7_dn4 / (2.0 * assign6740_e4749))) * assign6740_e4755) - (assign6740_e4750 * (((2.0 * locals.var_t4_dn4) * locals.var_t4) + (assign6740_e4753 * locals.var_t4_dn4)))) / (assign6740_e4755 * assign6740_e4755)), ((((locals.var_t6_dn5 + (locals.var_t7_dn5 / (2.0 * assign6740_e4749))) * assign6740_e4755) - (assign6740_e4750 * (((2.0 * locals.var_t4_dn5) * locals.var_t4) + (assign6740_e4753 * locals.var_t4_dn5)))) / (assign6740_e4755 * assign6740_e4755)), ((((locals.var_t6_dn6 + (locals.var_t7_dn6 / (2.0 * assign6740_e4749))) * assign6740_e4755) - (assign6740_e4750 * (((2.0 * locals.var_t4_dn6) * locals.var_t4) + (assign6740_e4753 * locals.var_t4_dn6)))) / (assign6740_e4755 * assign6740_e4755)), ((((locals.var_t6_dn8 + (locals.var_t7_dn8 / (2.0 * assign6740_e4749))) * assign6740_e4755) - (assign6740_e4750 * (((2.0 * locals.var_t4_dn8) * locals.var_t4) + (assign6740_e4753 * locals.var_t4_dn8)))) / (assign6740_e4755 * assign6740_e4755)), ((((locals.var_t6_dn10 + (locals.var_t7_dn10 / (2.0 * assign6740_e4749))) * assign6740_e4755) - (assign6740_e4750 * (((2.0 * locals.var_t4_dn10) * locals.var_t4) + (assign6740_e4753 * locals.var_t4_dn10)))) / (assign6740_e4755 * assign6740_e4755)), ((((locals.var_t6_dn11 + (locals.var_t7_dn11 / (2.0 * assign6740_e4749))) * assign6740_e4755) - (assign6740_e4750 * (((2.0 * locals.var_t4_dn11) * locals.var_t4) + (assign6740_e4753 * locals.var_t4_dn11)))) / (assign6740_e4755 * assign6740_e4755)), ((((locals.var_t6_dn12 + (locals.var_t7_dn12 / (2.0 * assign6740_e4749))) * assign6740_e4755) - (assign6740_e4750 * (((2.0 * locals.var_t4_dn12) * locals.var_t4) + (assign6740_e4753 * locals.var_t4_dn12)))) / (assign6740_e4755 * assign6740_e4755)),)
    } else {
        (locals.var_phi_s0_bulk_0, locals.var_phi_s0_bulk_0_dn0, locals.var_phi_s0_bulk_0_dn2, locals.var_phi_s0_bulk_0_dn4, locals.var_phi_s0_bulk_0_dn5, locals.var_phi_s0_bulk_0_dn6, locals.var_phi_s0_bulk_0_dn8, locals.var_phi_s0_bulk_0_dn10, locals.var_phi_s0_bulk_0_dn11, locals.var_phi_s0_bulk_0_dn12,)
    }
};
        locals.var_phi_s0_bulk_0 = assign6740_e4758;
        locals.var_phi_s0_bulk_0_dn0 = assign6740_e4758_d_n0;
        locals.var_phi_s0_bulk_0_dn2 = assign6740_e4758_d_n2;
        locals.var_phi_s0_bulk_0_dn4 = assign6740_e4758_d_n4;
        locals.var_phi_s0_bulk_0_dn5 = assign6740_e4758_d_n5;
        locals.var_phi_s0_bulk_0_dn6 = assign6740_e4758_d_n6;
        locals.var_phi_s0_bulk_0_dn8 = assign6740_e4758_d_n8;
        locals.var_phi_s0_bulk_0_dn10 = assign6740_e4758_d_n10;
        locals.var_phi_s0_bulk_0_dn11 = assign6740_e4758_d_n11;
        locals.var_phi_s0_bulk_0_dn12 = assign6740_e4758_d_n12;
        locals.var_phi_s0_bulk_0_rv = 0.0;

        let assign6750_e4761: f64 = (2.0 / locals.var_beta);
        let assign6750_e4764: f64 = (locals.var_n_subbl / locals.var_nin);
        let assign6750_e4765: f64 = (assign6750_e4764).ln();
        let assign6750_e4766: f64 = (assign6750_e4761 * assign6750_e4765);
        locals.var_pb2_bulk = assign6750_e4766;
        locals.var_pb2_bulk_dn0 = (assign6750_e4761 * ((((locals.var_n_subbl_dn0 * locals.var_nin) - (locals.var_n_subbl * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign6750_e4764));
        locals.var_pb2_bulk_dn2 = (assign6750_e4761 * ((((locals.var_n_subbl_dn2 * locals.var_nin) - (locals.var_n_subbl * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign6750_e4764));
        locals.var_pb2_bulk_dn4 = (((-((2.0 * locals.var_beta_dn4) / (locals.var_beta * locals.var_beta))) * assign6750_e4765) + (assign6750_e4761 * ((((locals.var_n_subbl_dn4 * locals.var_nin) - (locals.var_n_subbl * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign6750_e4764)));
        locals.var_pb2_bulk_dn5 = (assign6750_e4761 * ((((locals.var_n_subbl_dn5 * locals.var_nin) - (locals.var_n_subbl * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign6750_e4764));
        locals.var_pb2_bulk_dn6 = (assign6750_e4761 * ((((locals.var_n_subbl_dn6 * locals.var_nin) - (locals.var_n_subbl * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign6750_e4764));
        locals.var_pb2_bulk_dn8 = (assign6750_e4761 * ((((locals.var_n_subbl_dn8 * locals.var_nin) - (locals.var_n_subbl * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign6750_e4764));
        locals.var_pb2_bulk_dn10 = (assign6750_e4761 * ((((locals.var_n_subbl_dn10 * locals.var_nin) - (locals.var_n_subbl * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign6750_e4764));
        locals.var_pb2_bulk_dn11 = (assign6750_e4761 * ((((locals.var_n_subbl_dn11 * locals.var_nin) - (locals.var_n_subbl * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign6750_e4764));
        locals.var_pb2_bulk_dn12 = (assign6750_e4761 * ((((locals.var_n_subbl_dn12 * locals.var_nin) - (locals.var_n_subbl * locals.var_nin_dn12)) / (locals.var_nin * locals.var_nin)) / assign6750_e4764));
        locals.var_pb2_bulk_rv = 0.0;

        let assign6760_e4769: f64 = (locals.var_cnst0bulk * locals.var_cnst0bulk);
        let assign6760_e4771: f64 = (assign6760_e4769 * locals.var_c_box_fd_inv);
        let assign6760_e4773: f64 = (assign6760_e4771 * locals.var_c_box_fd_inv);
        locals.var_t0 = assign6760_e4773;
        locals.var_t0_dn0 = ((((locals.var_cnst0bulk_dn0 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn0)) * locals.var_c_box_fd_inv) * locals.var_c_box_fd_inv);
        locals.var_t0_dn2 = ((((locals.var_cnst0bulk_dn2 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn2)) * locals.var_c_box_fd_inv) * locals.var_c_box_fd_inv);
        locals.var_t0_dn4 = ((((locals.var_cnst0bulk_dn4 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn4)) * locals.var_c_box_fd_inv) * locals.var_c_box_fd_inv);
        locals.var_t0_dn5 = ((((locals.var_cnst0bulk_dn5 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn5)) * locals.var_c_box_fd_inv) * locals.var_c_box_fd_inv);
        locals.var_t0_dn6 = ((((locals.var_cnst0bulk_dn6 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn6)) * locals.var_c_box_fd_inv) * locals.var_c_box_fd_inv);
        locals.var_t0_dn8 = ((((locals.var_cnst0bulk_dn8 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn8)) * locals.var_c_box_fd_inv) * locals.var_c_box_fd_inv);
        locals.var_t0_dn10 = ((((locals.var_cnst0bulk_dn10 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn10)) * locals.var_c_box_fd_inv) * locals.var_c_box_fd_inv);
        locals.var_t0_dn11 = ((((locals.var_cnst0bulk_dn11 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn11)) * locals.var_c_box_fd_inv) * locals.var_c_box_fd_inv);
        locals.var_t0_dn12 = ((((locals.var_cnst0bulk_dn12 * locals.var_cnst0bulk) + (locals.var_cnst0bulk * locals.var_cnst0bulk_dn12)) * locals.var_c_box_fd_inv) * locals.var_c_box_fd_inv);
        locals.var_t0_rv = 0.0;

        let assign6770_e4775: f64 = (-locals.var_vbsbiz);
        locals.var_t1 = assign6770_e4775;
        locals.var_t1_dn0 = (-locals.var_vbsbiz_dn0);
        locals.var_t1_dn2 = (-locals.var_vbsbiz_dn2);
        locals.var_t1_dn4 = (-locals.var_vbsbiz_dn4);
        locals.var_t1_dn5 = (-locals.var_vbsbiz_dn5);
        locals.var_t1_dn6 = (-locals.var_vbsbiz_dn6);
        locals.var_t1_dn8 = (-locals.var_vbsbiz_dn8);
        locals.var_t1_dn10 = (-locals.var_vbsbiz_dn10);
        locals.var_t1_dn11 = (-locals.var_vbsbiz_dn11);
        locals.var_t1_dn12 = (-locals.var_vbsbiz_dn12);
        locals.var_t1_rv = 0.0;

        let assign6780_e4778: f64 = (2.0 * locals.var_t1);
        let assign6780_e4781: f64 = (locals.var_t0 * locals.var_beta);
        let assign6780_e4782: f64 = (assign6780_e4778 + assign6780_e4781);
        let assign6780_e4785: f64 = (2.0 * locals.var_t1);
        let assign6780_e4788: f64 = (locals.var_t0 * locals.var_beta);
        let assign6780_e4789: f64 = (assign6780_e4785 + assign6780_e4788);
        let assign6780_e4790: f64 = (assign6780_e4782 * assign6780_e4789);
        let assign6780_e4794: f64 = (locals.var_t1 * locals.var_t1);
        let assign6780_e4796: f64 = (assign6780_e4794 + locals.var_t0);
        let assign6780_e4797: f64 = (4.0 * assign6780_e4796);
        let assign6780_e4798: f64 = (assign6780_e4790 - assign6780_e4797);
        locals.var_t2 = assign6780_e4798;
        locals.var_t2_dn0 = (((((2.0 * locals.var_t1_dn0) + (locals.var_t0_dn0 * locals.var_beta)) * assign6780_e4789) + (assign6780_e4782 * ((2.0 * locals.var_t1_dn0) + (locals.var_t0_dn0 * locals.var_beta)))) - (4.0 * (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) + locals.var_t0_dn0)));
        locals.var_t2_dn2 = (((((2.0 * locals.var_t1_dn2) + (locals.var_t0_dn2 * locals.var_beta)) * assign6780_e4789) + (assign6780_e4782 * ((2.0 * locals.var_t1_dn2) + (locals.var_t0_dn2 * locals.var_beta)))) - (4.0 * (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) + locals.var_t0_dn2)));
        locals.var_t2_dn4 = (((((2.0 * locals.var_t1_dn4) + ((locals.var_t0_dn4 * locals.var_beta) + (locals.var_t0 * locals.var_beta_dn4))) * assign6780_e4789) + (assign6780_e4782 * ((2.0 * locals.var_t1_dn4) + ((locals.var_t0_dn4 * locals.var_beta) + (locals.var_t0 * locals.var_beta_dn4))))) - (4.0 * (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + locals.var_t0_dn4)));
        locals.var_t2_dn5 = (((((2.0 * locals.var_t1_dn5) + (locals.var_t0_dn5 * locals.var_beta)) * assign6780_e4789) + (assign6780_e4782 * ((2.0 * locals.var_t1_dn5) + (locals.var_t0_dn5 * locals.var_beta)))) - (4.0 * (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + locals.var_t0_dn5)));
        locals.var_t2_dn6 = (((((2.0 * locals.var_t1_dn6) + (locals.var_t0_dn6 * locals.var_beta)) * assign6780_e4789) + (assign6780_e4782 * ((2.0 * locals.var_t1_dn6) + (locals.var_t0_dn6 * locals.var_beta)))) - (4.0 * (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + locals.var_t0_dn6)));
        locals.var_t2_dn8 = (((((2.0 * locals.var_t1_dn8) + (locals.var_t0_dn8 * locals.var_beta)) * assign6780_e4789) + (assign6780_e4782 * ((2.0 * locals.var_t1_dn8) + (locals.var_t0_dn8 * locals.var_beta)))) - (4.0 * (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + locals.var_t0_dn8)));
        locals.var_t2_dn10 = (((((2.0 * locals.var_t1_dn10) + (locals.var_t0_dn10 * locals.var_beta)) * assign6780_e4789) + (assign6780_e4782 * ((2.0 * locals.var_t1_dn10) + (locals.var_t0_dn10 * locals.var_beta)))) - (4.0 * (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + locals.var_t0_dn10)));
        locals.var_t2_dn11 = (((((2.0 * locals.var_t1_dn11) + (locals.var_t0_dn11 * locals.var_beta)) * assign6780_e4789) + (assign6780_e4782 * ((2.0 * locals.var_t1_dn11) + (locals.var_t0_dn11 * locals.var_beta)))) - (4.0 * (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + locals.var_t0_dn11)));
        locals.var_t2_dn12 = (((((2.0 * locals.var_t1_dn12) + (locals.var_t0_dn12 * locals.var_beta)) * assign6780_e4789) + (assign6780_e4782 * ((2.0 * locals.var_t1_dn12) + (locals.var_t0_dn12 * locals.var_beta)))) - (4.0 * (((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)) + locals.var_t0_dn12)));
        locals.var_t2_rv = 0.0;

        let assign6790_e4802: f64 = (10.0 * 2.220446049250313e-16);
        let (assign6790_e4808, assign6790_e4808_d_n0, assign6790_e4808_d_n2, assign6790_e4808_d_n4, assign6790_e4808_d_n5, assign6790_e4808_d_n6, assign6790_e4808_d_n8, assign6790_e4808_d_n10, assign6790_e4808_d_n11, assign6790_e4808_d_n12,) = {
    if (locals.var_t2 >= assign6790_e4802) {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    } else {
        let assign6790_e4807: f64 = (10.0 * 2.220446049250313e-16);
        (assign6790_e4807, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        locals.var_t2 = assign6790_e4808;
        locals.var_t2_dn0 = assign6790_e4808_d_n0;
        locals.var_t2_dn2 = assign6790_e4808_d_n2;
        locals.var_t2_dn4 = assign6790_e4808_d_n4;
        locals.var_t2_dn5 = assign6790_e4808_d_n5;
        locals.var_t2_dn6 = assign6790_e4808_d_n6;
        locals.var_t2_dn8 = assign6790_e4808_d_n8;
        locals.var_t2_dn10 = assign6790_e4808_d_n10;
        locals.var_t2_dn11 = assign6790_e4808_d_n11;
        locals.var_t2_dn12 = assign6790_e4808_d_n12;
        locals.var_t2_rv = 0.0;

        let assign6800_e4810: f64 = (locals.var_t2).sqrt();
        locals.var_t2 = assign6800_e4810;
        locals.var_t2_dn0 = (locals.var_t2_dn0 / (2.0 * assign6800_e4810));
        locals.var_t2_dn2 = (locals.var_t2_dn2 / (2.0 * assign6800_e4810));
        locals.var_t2_dn4 = (locals.var_t2_dn4 / (2.0 * assign6800_e4810));
        locals.var_t2_dn5 = (locals.var_t2_dn5 / (2.0 * assign6800_e4810));
        locals.var_t2_dn6 = (locals.var_t2_dn6 / (2.0 * assign6800_e4810));
        locals.var_t2_dn8 = (locals.var_t2_dn8 / (2.0 * assign6800_e4810));
        locals.var_t2_dn10 = (locals.var_t2_dn10 / (2.0 * assign6800_e4810));
        locals.var_t2_dn11 = (locals.var_t2_dn11 / (2.0 * assign6800_e4810));
        locals.var_t2_dn12 = (locals.var_t2_dn12 / (2.0 * assign6800_e4810));
        locals.var_t2_rv = 0.0;

        let assign6810_e4813: f64 = (2.0 * locals.var_t1);
        let assign6810_e4816: f64 = (locals.var_t0 * locals.var_beta);
        let assign6810_e4817: f64 = (assign6810_e4813 + assign6810_e4816);
        locals.var_t3 = assign6810_e4817;
        locals.var_t3_dn0 = ((2.0 * locals.var_t1_dn0) + (locals.var_t0_dn0 * locals.var_beta));
        locals.var_t3_dn2 = ((2.0 * locals.var_t1_dn2) + (locals.var_t0_dn2 * locals.var_beta));
        locals.var_t3_dn4 = ((2.0 * locals.var_t1_dn4) + ((locals.var_t0_dn4 * locals.var_beta) + (locals.var_t0 * locals.var_beta_dn4)));
        locals.var_t3_dn5 = ((2.0 * locals.var_t1_dn5) + (locals.var_t0_dn5 * locals.var_beta));
        locals.var_t3_dn6 = ((2.0 * locals.var_t1_dn6) + (locals.var_t0_dn6 * locals.var_beta));
        locals.var_t3_dn8 = ((2.0 * locals.var_t1_dn8) + (locals.var_t0_dn8 * locals.var_beta));
        locals.var_t3_dn10 = ((2.0 * locals.var_t1_dn10) + (locals.var_t0_dn10 * locals.var_beta));
        locals.var_t3_dn11 = ((2.0 * locals.var_t1_dn11) + (locals.var_t0_dn11 * locals.var_beta));
        locals.var_t3_dn12 = ((2.0 * locals.var_t1_dn12) + (locals.var_t0_dn12 * locals.var_beta));
        locals.var_t3_rv = 0.0;

        let assign6820_e4820: f64 = (locals.var_t3 - locals.var_t2);
        let assign6820_e4822: f64 = (assign6820_e4820 / 2.0);
        locals.var_psb_inia = assign6820_e4822;
        locals.var_psb_inia_dn0 = ((locals.var_t3_dn0 - locals.var_t2_dn0) / 2.0);
        locals.var_psb_inia_dn2 = ((locals.var_t3_dn2 - locals.var_t2_dn2) / 2.0);
        locals.var_psb_inia_dn4 = ((locals.var_t3_dn4 - locals.var_t2_dn4) / 2.0);
        locals.var_psb_inia_dn5 = ((locals.var_t3_dn5 - locals.var_t2_dn5) / 2.0);
        locals.var_psb_inia_dn6 = ((locals.var_t3_dn6 - locals.var_t2_dn6) / 2.0);
        locals.var_psb_inia_dn8 = ((locals.var_t3_dn8 - locals.var_t2_dn8) / 2.0);
        locals.var_psb_inia_dn10 = ((locals.var_t3_dn10 - locals.var_t2_dn10) / 2.0);
        locals.var_psb_inia_dn11 = ((locals.var_t3_dn11 - locals.var_t2_dn11) / 2.0);
        locals.var_psb_inia_dn12 = ((locals.var_t3_dn12 - locals.var_t2_dn12) / 2.0);
        locals.var_psb_inia_rv = 0.0;

        let assign6830_e4825: f64 = (locals.var_t1 * locals.var_t1);
        let assign6830_e4827: f64 = (assign6830_e4825 / locals.var_t0);
        let assign6830_e4829: f64 = (assign6830_e4827 / locals.var_cnst1bulk);
        let assign6830_e4830: f64 = (assign6830_e4829).ln();
        let assign6830_e4834: f64 = (2.0 / locals.var_t1);
        let assign6830_e4835: f64 = (locals.var_beta + assign6830_e4834);
        let assign6830_e4836: f64 = (assign6830_e4830 / assign6830_e4835);
        locals.var_psb_inib = assign6830_e4836;
        locals.var_psb_inib_dn0 = ((((((((((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) * locals.var_t0) - (assign6830_e4825 * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)) * locals.var_cnst1bulk) - (assign6830_e4827 * locals.var_cnst1bulk_dn0)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign6830_e4829) * assign6830_e4835) - (assign6830_e4830 * (-((2.0 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))))) / (assign6830_e4835 * assign6830_e4835));
        locals.var_psb_inib_dn2 = ((((((((((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) * locals.var_t0) - (assign6830_e4825 * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)) * locals.var_cnst1bulk) - (assign6830_e4827 * locals.var_cnst1bulk_dn2)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign6830_e4829) * assign6830_e4835) - (assign6830_e4830 * (-((2.0 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))))) / (assign6830_e4835 * assign6830_e4835));
        locals.var_psb_inib_dn4 = ((((((((((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) * locals.var_t0) - (assign6830_e4825 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)) * locals.var_cnst1bulk) - (assign6830_e4827 * locals.var_cnst1bulk_dn4)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign6830_e4829) * assign6830_e4835) - (assign6830_e4830 * (locals.var_beta_dn4 + (-((2.0 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1)))))) / (assign6830_e4835 * assign6830_e4835));
        locals.var_psb_inib_dn5 = ((((((((((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) * locals.var_t0) - (assign6830_e4825 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)) * locals.var_cnst1bulk) - (assign6830_e4827 * locals.var_cnst1bulk_dn5)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign6830_e4829) * assign6830_e4835) - (assign6830_e4830 * (-((2.0 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))))) / (assign6830_e4835 * assign6830_e4835));
        locals.var_psb_inib_dn6 = ((((((((((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) * locals.var_t0) - (assign6830_e4825 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)) * locals.var_cnst1bulk) - (assign6830_e4827 * locals.var_cnst1bulk_dn6)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign6830_e4829) * assign6830_e4835) - (assign6830_e4830 * (-((2.0 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))))) / (assign6830_e4835 * assign6830_e4835));
        locals.var_psb_inib_dn8 = ((((((((((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) * locals.var_t0) - (assign6830_e4825 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)) * locals.var_cnst1bulk) - (assign6830_e4827 * locals.var_cnst1bulk_dn8)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign6830_e4829) * assign6830_e4835) - (assign6830_e4830 * (-((2.0 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))))) / (assign6830_e4835 * assign6830_e4835));
        locals.var_psb_inib_dn10 = ((((((((((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) * locals.var_t0) - (assign6830_e4825 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)) * locals.var_cnst1bulk) - (assign6830_e4827 * locals.var_cnst1bulk_dn10)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign6830_e4829) * assign6830_e4835) - (assign6830_e4830 * (-((2.0 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))))) / (assign6830_e4835 * assign6830_e4835));
        locals.var_psb_inib_dn11 = ((((((((((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) * locals.var_t0) - (assign6830_e4825 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)) * locals.var_cnst1bulk) - (assign6830_e4827 * locals.var_cnst1bulk_dn11)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign6830_e4829) * assign6830_e4835) - (assign6830_e4830 * (-((2.0 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))))) / (assign6830_e4835 * assign6830_e4835));
        locals.var_psb_inib_dn12 = ((((((((((((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)) * locals.var_t0) - (assign6830_e4825 * locals.var_t0_dn12)) / (locals.var_t0 * locals.var_t0)) * locals.var_cnst1bulk) - (assign6830_e4827 * locals.var_cnst1bulk_dn12)) / (locals.var_cnst1bulk * locals.var_cnst1bulk)) / assign6830_e4829) * assign6830_e4835) - (assign6830_e4830 * (-((2.0 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))))) / (assign6830_e4835 * assign6830_e4835));
        locals.var_psb_inib_rv = 0.0;

        let assign6840_e4839: f64 = if locals.var_psb_inia < locals.var_pb2_bulk { 1.0 } else { 0.0 };
        locals.var_guard68 = assign6840_e4839;
        locals.var_guard68_rv = 0.0;

        let (assign6850_e4843, assign6850_e4843_d_n0, assign6850_e4843_d_n2, assign6850_e4843_d_n4, assign6850_e4843_d_n5, assign6850_e4843_d_n6, assign6850_e4843_d_n8, assign6850_e4843_d_n10, assign6850_e4843_d_n11, assign6850_e4843_d_n12,) = {
    if (locals.var_guard68 != 0.0) {
        (locals.var_psb_inia, locals.var_psb_inia_dn0, locals.var_psb_inia_dn2, locals.var_psb_inia_dn4, locals.var_psb_inia_dn5, locals.var_psb_inia_dn6, locals.var_psb_inia_dn8, locals.var_psb_inia_dn10, locals.var_psb_inia_dn11, locals.var_psb_inia_dn12,)
    } else {
        (locals.var_phi_s0_bulk_0, locals.var_phi_s0_bulk_0_dn0, locals.var_phi_s0_bulk_0_dn2, locals.var_phi_s0_bulk_0_dn4, locals.var_phi_s0_bulk_0_dn5, locals.var_phi_s0_bulk_0_dn6, locals.var_phi_s0_bulk_0_dn8, locals.var_phi_s0_bulk_0_dn10, locals.var_phi_s0_bulk_0_dn11, locals.var_phi_s0_bulk_0_dn12,)
    }
};
        locals.var_phi_s0_bulk_0 = assign6850_e4843;
        locals.var_phi_s0_bulk_0_dn0 = assign6850_e4843_d_n0;
        locals.var_phi_s0_bulk_0_dn2 = assign6850_e4843_d_n2;
        locals.var_phi_s0_bulk_0_dn4 = assign6850_e4843_d_n4;
        locals.var_phi_s0_bulk_0_dn5 = assign6850_e4843_d_n5;
        locals.var_phi_s0_bulk_0_dn6 = assign6850_e4843_d_n6;
        locals.var_phi_s0_bulk_0_dn8 = assign6850_e4843_d_n8;
        locals.var_phi_s0_bulk_0_dn10 = assign6850_e4843_d_n10;
        locals.var_phi_s0_bulk_0_dn11 = assign6850_e4843_d_n11;
        locals.var_phi_s0_bulk_0_dn12 = assign6850_e4843_d_n12;
        locals.var_phi_s0_bulk_0_rv = 0.0;

        let (assign6860_e4852, assign6860_e4852_d_n0, assign6860_e4852_d_n2, assign6860_e4852_d_n4, assign6860_e4852_d_n5, assign6860_e4852_d_n6, assign6860_e4852_d_n8, assign6860_e4852_d_n10, assign6860_e4852_d_n11, assign6860_e4852_d_n12,) = {
    if (locals.var_guard68 == 0.0) {
        let assign6860_e4848: f64 = (locals.var_psb_inib - locals.var_psb_inia);
        let assign6860_e4850: f64 = (assign6860_e4848 - 0.0008);
        (assign6860_e4850, (locals.var_psb_inib_dn0 - locals.var_psb_inia_dn0), (locals.var_psb_inib_dn2 - locals.var_psb_inia_dn2), (locals.var_psb_inib_dn4 - locals.var_psb_inia_dn4), (locals.var_psb_inib_dn5 - locals.var_psb_inia_dn5), (locals.var_psb_inib_dn6 - locals.var_psb_inia_dn6), (locals.var_psb_inib_dn8 - locals.var_psb_inia_dn8), (locals.var_psb_inib_dn10 - locals.var_psb_inia_dn10), (locals.var_psb_inib_dn11 - locals.var_psb_inia_dn11), (locals.var_psb_inib_dn12 - locals.var_psb_inia_dn12),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn8, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn12,)
    }
};
        locals.var_tmf1 = assign6860_e4852;
        locals.var_tmf1_dn0 = assign6860_e4852_d_n0;
        locals.var_tmf1_dn2 = assign6860_e4852_d_n2;
        locals.var_tmf1_dn4 = assign6860_e4852_d_n4;
        locals.var_tmf1_dn5 = assign6860_e4852_d_n5;
        locals.var_tmf1_dn6 = assign6860_e4852_d_n6;
        locals.var_tmf1_dn8 = assign6860_e4852_d_n8;
        locals.var_tmf1_dn10 = assign6860_e4852_d_n10;
        locals.var_tmf1_dn11 = assign6860_e4852_d_n11;
        locals.var_tmf1_dn12 = assign6860_e4852_d_n12;
        locals.var_tmf1_rv = 0.0;

        let (assign6870_e4861, assign6870_e4861_d_n0, assign6870_e4861_d_n2, assign6870_e4861_d_n4, assign6870_e4861_d_n5, assign6870_e4861_d_n6, assign6870_e4861_d_n8, assign6870_e4861_d_n10, assign6870_e4861_d_n11, assign6870_e4861_d_n12,) = {
    if (locals.var_guard68 == 0.0) {
        let assign6870_e4857: f64 = (4.0 * locals.var_psb_inib);
        let assign6870_e4859: f64 = (assign6870_e4857 * 0.0008);
        (assign6870_e4859, ((4.0 * locals.var_psb_inib_dn0) * 0.0008), ((4.0 * locals.var_psb_inib_dn2) * 0.0008), ((4.0 * locals.var_psb_inib_dn4) * 0.0008), ((4.0 * locals.var_psb_inib_dn5) * 0.0008), ((4.0 * locals.var_psb_inib_dn6) * 0.0008), ((4.0 * locals.var_psb_inib_dn8) * 0.0008), ((4.0 * locals.var_psb_inib_dn10) * 0.0008), ((4.0 * locals.var_psb_inib_dn11) * 0.0008), ((4.0 * locals.var_psb_inib_dn12) * 0.0008),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign6870_e4861;
        locals.var_tmf2_dn0 = assign6870_e4861_d_n0;
        locals.var_tmf2_dn2 = assign6870_e4861_d_n2;
        locals.var_tmf2_dn4 = assign6870_e4861_d_n4;
        locals.var_tmf2_dn5 = assign6870_e4861_d_n5;
        locals.var_tmf2_dn6 = assign6870_e4861_d_n6;
        locals.var_tmf2_dn8 = assign6870_e4861_d_n8;
        locals.var_tmf2_dn10 = assign6870_e4861_d_n10;
        locals.var_tmf2_dn11 = assign6870_e4861_d_n11;
        locals.var_tmf2_dn12 = assign6870_e4861_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign6880_e4872, assign6880_e4872_d_n0, assign6880_e4872_d_n2, assign6880_e4872_d_n4, assign6880_e4872_d_n5, assign6880_e4872_d_n6, assign6880_e4872_d_n8, assign6880_e4872_d_n10, assign6880_e4872_d_n11, assign6880_e4872_d_n12,) = {
    if (locals.var_guard68 == 0.0) {
        let (assign6880_e4870, assign6880_e4870_d_n0, assign6880_e4870_d_n2, assign6880_e4870_d_n4, assign6880_e4870_d_n5, assign6880_e4870_d_n6, assign6880_e4870_d_n8, assign6880_e4870_d_n10, assign6880_e4870_d_n11, assign6880_e4870_d_n12,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
            } else {
                let assign6880_e4869: f64 = (-locals.var_tmf2);
                (assign6880_e4869, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12),)
            }
        };
        (assign6880_e4870, assign6880_e4870_d_n0, assign6880_e4870_d_n2, assign6880_e4870_d_n4, assign6880_e4870_d_n5, assign6880_e4870_d_n6, assign6880_e4870_d_n8, assign6880_e4870_d_n10, assign6880_e4870_d_n11, assign6880_e4870_d_n12,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign6880_e4872;
        locals.var_tmf2_dn0 = assign6880_e4872_d_n0;
        locals.var_tmf2_dn2 = assign6880_e4872_d_n2;
        locals.var_tmf2_dn4 = assign6880_e4872_d_n4;
        locals.var_tmf2_dn5 = assign6880_e4872_d_n5;
        locals.var_tmf2_dn6 = assign6880_e4872_d_n6;
        locals.var_tmf2_dn8 = assign6880_e4872_d_n8;
        locals.var_tmf2_dn10 = assign6880_e4872_d_n10;
        locals.var_tmf2_dn11 = assign6880_e4872_d_n11;
        locals.var_tmf2_dn12 = assign6880_e4872_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign6890_e4882, assign6890_e4882_d_n0, assign6890_e4882_d_n2, assign6890_e4882_d_n4, assign6890_e4882_d_n5, assign6890_e4882_d_n6, assign6890_e4882_d_n8, assign6890_e4882_d_n10, assign6890_e4882_d_n11, assign6890_e4882_d_n12,) = {
    if (locals.var_guard68 == 0.0) {
        let assign6890_e4877: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign6890_e4879: f64 = (assign6890_e4877 + locals.var_tmf2);
        let assign6890_e4880: f64 = (assign6890_e4879).sqrt();
        (assign6890_e4880, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign6890_e4880)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign6890_e4880)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign6890_e4880)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign6890_e4880)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign6890_e4880)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign6890_e4880)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign6890_e4880)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign6890_e4880)), ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign6890_e4880)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    }
};
        locals.var_tmf2 = assign6890_e4882;
        locals.var_tmf2_dn0 = assign6890_e4882_d_n0;
        locals.var_tmf2_dn2 = assign6890_e4882_d_n2;
        locals.var_tmf2_dn4 = assign6890_e4882_d_n4;
        locals.var_tmf2_dn5 = assign6890_e4882_d_n5;
        locals.var_tmf2_dn6 = assign6890_e4882_d_n6;
        locals.var_tmf2_dn8 = assign6890_e4882_d_n8;
        locals.var_tmf2_dn10 = assign6890_e4882_d_n10;
        locals.var_tmf2_dn11 = assign6890_e4882_d_n11;
        locals.var_tmf2_dn12 = assign6890_e4882_d_n12;
        locals.var_tmf2_rv = 0.0;

        let (assign6900_e4893, assign6900_e4893_d_n0, assign6900_e4893_d_n2, assign6900_e4893_d_n4, assign6900_e4893_d_n5, assign6900_e4893_d_n6, assign6900_e4893_d_n8, assign6900_e4893_d_n10, assign6900_e4893_d_n11, assign6900_e4893_d_n12,) = {
    if (locals.var_guard68 == 0.0) {
        let assign6900_e4889: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign6900_e4890: f64 = (1.0 + assign6900_e4889);
        let assign6900_e4891: f64 = (0.5 * assign6900_e4890);
        (assign6900_e4891, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn8, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign6900_e4893;
        locals.var_t1_dn0 = assign6900_e4893_d_n0;
        locals.var_t1_dn2 = assign6900_e4893_d_n2;
        locals.var_t1_dn4 = assign6900_e4893_d_n4;
        locals.var_t1_dn5 = assign6900_e4893_d_n5;
        locals.var_t1_dn6 = assign6900_e4893_d_n6;
        locals.var_t1_dn8 = assign6900_e4893_d_n8;
        locals.var_t1_dn10 = assign6900_e4893_d_n10;
        locals.var_t1_dn11 = assign6900_e4893_d_n11;
        locals.var_t1_dn12 = assign6900_e4893_d_n12;
        locals.var_t1_rv = 0.0;

        let (assign6910_e4904, assign6910_e4904_d_n0, assign6910_e4904_d_n2, assign6910_e4904_d_n4, assign6910_e4904_d_n5, assign6910_e4904_d_n6, assign6910_e4904_d_n8, assign6910_e4904_d_n10, assign6910_e4904_d_n11, assign6910_e4904_d_n12,) = {
    if (locals.var_guard68 == 0.0) {
        let assign6910_e4900: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign6910_e4901: f64 = (0.5 * assign6910_e4900);
        let assign6910_e4902: f64 = (locals.var_psb_inib - assign6910_e4901);
        (assign6910_e4902, (locals.var_psb_inib_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_psb_inib_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_psb_inib_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_psb_inib_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_psb_inib_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_psb_inib_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_psb_inib_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_psb_inib_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_psb_inib_dn12 - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12))),)
    } else {
        (locals.var_phi_s0_bulk_0, locals.var_phi_s0_bulk_0_dn0, locals.var_phi_s0_bulk_0_dn2, locals.var_phi_s0_bulk_0_dn4, locals.var_phi_s0_bulk_0_dn5, locals.var_phi_s0_bulk_0_dn6, locals.var_phi_s0_bulk_0_dn8, locals.var_phi_s0_bulk_0_dn10, locals.var_phi_s0_bulk_0_dn11, locals.var_phi_s0_bulk_0_dn12,)
    }
};
        locals.var_phi_s0_bulk_0 = assign6910_e4904;
        locals.var_phi_s0_bulk_0_dn0 = assign6910_e4904_d_n0;
        locals.var_phi_s0_bulk_0_dn2 = assign6910_e4904_d_n2;
        locals.var_phi_s0_bulk_0_dn4 = assign6910_e4904_d_n4;
        locals.var_phi_s0_bulk_0_dn5 = assign6910_e4904_d_n5;
        locals.var_phi_s0_bulk_0_dn6 = assign6910_e4904_d_n6;
        locals.var_phi_s0_bulk_0_dn8 = assign6910_e4904_d_n8;
        locals.var_phi_s0_bulk_0_dn10 = assign6910_e4904_d_n10;
        locals.var_phi_s0_bulk_0_dn11 = assign6910_e4904_d_n11;
        locals.var_phi_s0_bulk_0_dn12 = assign6910_e4904_d_n12;
        locals.var_phi_s0_bulk_0_rv = 0.0;

        locals.var_lp_s0 = 0.0;
        locals.var_lp_s0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_19(
        locals: &mut StampLocals,
    ) {
        let mut assign6930_loop_guard: usize = 0;
        while {
            let assign6930_cond_e4908: f64 = if locals.var_lp_s0 < locals.var_lp_s0_max { 1.0 } else { 0.0 };
            assign6930_cond_e4908 != 0.0
        } {
            assign6930_loop_guard += 1;
            assert!(assign6930_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            locals.var_t1 = locals.var_cnst0bulk;
            locals.var_t1_dn0 = locals.var_cnst0bulk_dn0;
            locals.var_t1_dn2 = locals.var_cnst0bulk_dn2;
            locals.var_t1_dn4 = locals.var_cnst0bulk_dn4;
            locals.var_t1_dn5 = locals.var_cnst0bulk_dn5;
            locals.var_t1_dn6 = locals.var_cnst0bulk_dn6;
            locals.var_t1_dn8 = locals.var_cnst0bulk_dn8;
            locals.var_t1_dn10 = locals.var_cnst0bulk_dn10;
            locals.var_t1_dn11 = locals.var_cnst0bulk_dn11;
            locals.var_t1_dn12 = locals.var_cnst0bulk_dn12;
            locals.var_t1_rv = 0.0;
            let assign6930_body1_e4912: f64 = (locals.var_beta * locals.var_phi_s0_bulk_0);
            locals.var_t2 = assign6930_body1_e4912;
            locals.var_t2_dn0 = (locals.var_beta * locals.var_phi_s0_bulk_0_dn0);
            locals.var_t2_dn2 = (locals.var_beta * locals.var_phi_s0_bulk_0_dn2);
            locals.var_t2_dn4 = ((locals.var_beta_dn4 * locals.var_phi_s0_bulk_0) + (locals.var_beta * locals.var_phi_s0_bulk_0_dn4));
            locals.var_t2_dn5 = (locals.var_beta * locals.var_phi_s0_bulk_0_dn5);
            locals.var_t2_dn6 = (locals.var_beta * locals.var_phi_s0_bulk_0_dn6);
            locals.var_t2_dn8 = (locals.var_beta * locals.var_phi_s0_bulk_0_dn8);
            locals.var_t2_dn10 = (locals.var_beta * locals.var_phi_s0_bulk_0_dn10);
            locals.var_t2_dn11 = (locals.var_beta * locals.var_phi_s0_bulk_0_dn11);
            locals.var_t2_dn12 = (locals.var_beta * locals.var_phi_s0_bulk_0_dn12);
            locals.var_t2_rv = 0.0;
            let assign6930_body2_e4914: f64 = (-locals.var_t2);
            let assign6930_body2_e4915: f64 = (assign6930_body2_e4914).exp();
            locals.var_t3 = assign6930_body2_e4915;
            locals.var_t3_dn0 = (assign6930_body2_e4915 * (-locals.var_t2_dn0));
            locals.var_t3_dn2 = (assign6930_body2_e4915 * (-locals.var_t2_dn2));
            locals.var_t3_dn4 = (assign6930_body2_e4915 * (-locals.var_t2_dn4));
            locals.var_t3_dn5 = (assign6930_body2_e4915 * (-locals.var_t2_dn5));
            locals.var_t3_dn6 = (assign6930_body2_e4915 * (-locals.var_t2_dn6));
            locals.var_t3_dn8 = (assign6930_body2_e4915 * (-locals.var_t2_dn8));
            locals.var_t3_dn10 = (assign6930_body2_e4915 * (-locals.var_t2_dn10));
            locals.var_t3_dn11 = (assign6930_body2_e4915 * (-locals.var_t2_dn11));
            locals.var_t3_dn12 = (assign6930_body2_e4915 * (-locals.var_t2_dn12));
            locals.var_t3_rv = 0.0;
            let assign6930_body3_e4918: f64 = if locals.var_phi_s0_bulk_0 > 1e-8 { 1.0 } else { 0.0 };
            locals.var_guard69 = assign6930_body3_e4918;
            locals.var_guard69_rv = 0.0;
            let (assign6930_body4_e4925, assign6930_body4_e4925_d_n0, assign6930_body4_e4925_d_n2, assign6930_body4_e4925_d_n4, assign6930_body4_e4925_d_n5, assign6930_body4_e4925_d_n6, assign6930_body4_e4925_d_n8, assign6930_body4_e4925_d_n10, assign6930_body4_e4925_d_n11, assign6930_body4_e4925_d_n12,) = {
    if (locals.var_guard69 != 0.0) {
        let assign6930_body4_e4922: f64 = (locals.var_beta * locals.var_phi_s0_bulk_0);
        let assign6930_body4_e4923: f64 = (assign6930_body4_e4922).exp();
        (assign6930_body4_e4923, (assign6930_body4_e4923 * (locals.var_beta * locals.var_phi_s0_bulk_0_dn0)), (assign6930_body4_e4923 * (locals.var_beta * locals.var_phi_s0_bulk_0_dn2)), (assign6930_body4_e4923 * ((locals.var_beta_dn4 * locals.var_phi_s0_bulk_0) + (locals.var_beta * locals.var_phi_s0_bulk_0_dn4))), (assign6930_body4_e4923 * (locals.var_beta * locals.var_phi_s0_bulk_0_dn5)), (assign6930_body4_e4923 * (locals.var_beta * locals.var_phi_s0_bulk_0_dn6)), (assign6930_body4_e4923 * (locals.var_beta * locals.var_phi_s0_bulk_0_dn8)), (assign6930_body4_e4923 * (locals.var_beta * locals.var_phi_s0_bulk_0_dn10)), (assign6930_body4_e4923 * (locals.var_beta * locals.var_phi_s0_bulk_0_dn11)), (assign6930_body4_e4923 * (locals.var_beta * locals.var_phi_s0_bulk_0_dn12)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
            locals.var_t0 = assign6930_body4_e4925;
            locals.var_t0_dn0 = assign6930_body4_e4925_d_n0;
            locals.var_t0_dn2 = assign6930_body4_e4925_d_n2;
            locals.var_t0_dn4 = assign6930_body4_e4925_d_n4;
            locals.var_t0_dn5 = assign6930_body4_e4925_d_n5;
            locals.var_t0_dn6 = assign6930_body4_e4925_d_n6;
            locals.var_t0_dn8 = assign6930_body4_e4925_d_n8;
            locals.var_t0_dn10 = assign6930_body4_e4925_d_n10;
            locals.var_t0_dn11 = assign6930_body4_e4925_d_n11;
            locals.var_t0_dn12 = assign6930_body4_e4925_d_n12;
            locals.var_t0_rv = 0.0;
            let (assign6930_body5_e4943, assign6930_body5_e4943_d_n0, assign6930_body5_e4943_d_n2, assign6930_body5_e4943_d_n4, assign6930_body5_e4943_d_n5, assign6930_body5_e4943_d_n6, assign6930_body5_e4943_d_n8, assign6930_body5_e4943_d_n10, assign6930_body5_e4943_d_n11, assign6930_body5_e4943_d_n12,) = {
    if (locals.var_guard69 != 0.0) {
        let assign6930_body5_e4928: f64 = (-locals.var_t1);
        let assign6930_body5_e4931: f64 = (locals.var_t3 + locals.var_t2);
        let assign6930_body5_e4933: f64 = (assign6930_body5_e4931 - 1.0);
        let assign6930_body5_e4937: f64 = (locals.var_t0 - 1.0);
        let assign6930_body5_e4938: f64 = (locals.var_cnst1bulk * assign6930_body5_e4937);
        let assign6930_body5_e4939: f64 = (assign6930_body5_e4933 + assign6930_body5_e4938);
        let assign6930_body5_e4940: f64 = (assign6930_body5_e4939).sqrt();
        let assign6930_body5_e4941: f64 = (assign6930_body5_e4928 * assign6930_body5_e4940);
        (assign6930_body5_e4941, (((-locals.var_t1_dn0) * assign6930_body5_e4940) + (assign6930_body5_e4928 * (((locals.var_t3_dn0 + locals.var_t2_dn0) + ((locals.var_cnst1bulk_dn0 * assign6930_body5_e4937) + (locals.var_cnst1bulk * locals.var_t0_dn0))) / (2.0 * assign6930_body5_e4940)))), (((-locals.var_t1_dn2) * assign6930_body5_e4940) + (assign6930_body5_e4928 * (((locals.var_t3_dn2 + locals.var_t2_dn2) + ((locals.var_cnst1bulk_dn2 * assign6930_body5_e4937) + (locals.var_cnst1bulk * locals.var_t0_dn2))) / (2.0 * assign6930_body5_e4940)))), (((-locals.var_t1_dn4) * assign6930_body5_e4940) + (assign6930_body5_e4928 * (((locals.var_t3_dn4 + locals.var_t2_dn4) + ((locals.var_cnst1bulk_dn4 * assign6930_body5_e4937) + (locals.var_cnst1bulk * locals.var_t0_dn4))) / (2.0 * assign6930_body5_e4940)))), (((-locals.var_t1_dn5) * assign6930_body5_e4940) + (assign6930_body5_e4928 * (((locals.var_t3_dn5 + locals.var_t2_dn5) + ((locals.var_cnst1bulk_dn5 * assign6930_body5_e4937) + (locals.var_cnst1bulk * locals.var_t0_dn5))) / (2.0 * assign6930_body5_e4940)))), (((-locals.var_t1_dn6) * assign6930_body5_e4940) + (assign6930_body5_e4928 * (((locals.var_t3_dn6 + locals.var_t2_dn6) + ((locals.var_cnst1bulk_dn6 * assign6930_body5_e4937) + (locals.var_cnst1bulk * locals.var_t0_dn6))) / (2.0 * assign6930_body5_e4940)))), (((-locals.var_t1_dn8) * assign6930_body5_e4940) + (assign6930_body5_e4928 * (((locals.var_t3_dn8 + locals.var_t2_dn8) + ((locals.var_cnst1bulk_dn8 * assign6930_body5_e4937) + (locals.var_cnst1bulk * locals.var_t0_dn8))) / (2.0 * assign6930_body5_e4940)))), (((-locals.var_t1_dn10) * assign6930_body5_e4940) + (assign6930_body5_e4928 * (((locals.var_t3_dn10 + locals.var_t2_dn10) + ((locals.var_cnst1bulk_dn10 * assign6930_body5_e4937) + (locals.var_cnst1bulk * locals.var_t0_dn10))) / (2.0 * assign6930_body5_e4940)))), (((-locals.var_t1_dn11) * assign6930_body5_e4940) + (assign6930_body5_e4928 * (((locals.var_t3_dn11 + locals.var_t2_dn11) + ((locals.var_cnst1bulk_dn11 * assign6930_body5_e4937) + (locals.var_cnst1bulk * locals.var_t0_dn11))) / (2.0 * assign6930_body5_e4940)))), (((-locals.var_t1_dn12) * assign6930_body5_e4940) + (assign6930_body5_e4928 * (((locals.var_t3_dn12 + locals.var_t2_dn12) + ((locals.var_cnst1bulk_dn12 * assign6930_body5_e4937) + (locals.var_cnst1bulk * locals.var_t0_dn12))) / (2.0 * assign6930_body5_e4940)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
            locals.var_t4 = assign6930_body5_e4943;
            locals.var_t4_dn0 = assign6930_body5_e4943_d_n0;
            locals.var_t4_dn2 = assign6930_body5_e4943_d_n2;
            locals.var_t4_dn4 = assign6930_body5_e4943_d_n4;
            locals.var_t4_dn5 = assign6930_body5_e4943_d_n5;
            locals.var_t4_dn6 = assign6930_body5_e4943_d_n6;
            locals.var_t4_dn8 = assign6930_body5_e4943_d_n8;
            locals.var_t4_dn10 = assign6930_body5_e4943_d_n10;
            locals.var_t4_dn11 = assign6930_body5_e4943_d_n11;
            locals.var_t4_dn12 = assign6930_body5_e4943_d_n12;
            locals.var_t4_rv = 0.0;
            let (assign6930_body6_e4958, assign6930_body6_e4958_d_n0, assign6930_body6_e4958_d_n2, assign6930_body6_e4958_d_n4, assign6930_body6_e4958_d_n5, assign6930_body6_e4958_d_n6, assign6930_body6_e4958_d_n8, assign6930_body6_e4958_d_n10, assign6930_body6_e4958_d_n11, assign6930_body6_e4958_d_n12,) = {
    if (locals.var_guard69 != 0.0) {
        let assign6930_body6_e4947: f64 = (locals.var_c0bulk / locals.var_t4);
        let assign6930_body6_e4949: f64 = (-locals.var_t3);
        let assign6930_body6_e4951: f64 = (assign6930_body6_e4949 + 1.0);
        let assign6930_body6_e4954: f64 = (locals.var_cnst1bulk * locals.var_t0);
        let assign6930_body6_e4955: f64 = (assign6930_body6_e4951 + assign6930_body6_e4954);
        let assign6930_body6_e4956: f64 = (assign6930_body6_e4947 * assign6930_body6_e4955);
        (assign6930_body6_e4956, (((((locals.var_c0bulk_dn0 * locals.var_t4) - (locals.var_c0bulk * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)) * assign6930_body6_e4955) + (assign6930_body6_e4947 * ((-locals.var_t3_dn0) + ((locals.var_cnst1bulk_dn0 * locals.var_t0) + (locals.var_cnst1bulk * locals.var_t0_dn0))))), (((((locals.var_c0bulk_dn2 * locals.var_t4) - (locals.var_c0bulk * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)) * assign6930_body6_e4955) + (assign6930_body6_e4947 * ((-locals.var_t3_dn2) + ((locals.var_cnst1bulk_dn2 * locals.var_t0) + (locals.var_cnst1bulk * locals.var_t0_dn2))))), (((((locals.var_c0bulk_dn4 * locals.var_t4) - (locals.var_c0bulk * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) * assign6930_body6_e4955) + (assign6930_body6_e4947 * ((-locals.var_t3_dn4) + ((locals.var_cnst1bulk_dn4 * locals.var_t0) + (locals.var_cnst1bulk * locals.var_t0_dn4))))), (((((locals.var_c0bulk_dn5 * locals.var_t4) - (locals.var_c0bulk * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) * assign6930_body6_e4955) + (assign6930_body6_e4947 * ((-locals.var_t3_dn5) + ((locals.var_cnst1bulk_dn5 * locals.var_t0) + (locals.var_cnst1bulk * locals.var_t0_dn5))))), (((((locals.var_c0bulk_dn6 * locals.var_t4) - (locals.var_c0bulk * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) * assign6930_body6_e4955) + (assign6930_body6_e4947 * ((-locals.var_t3_dn6) + ((locals.var_cnst1bulk_dn6 * locals.var_t0) + (locals.var_cnst1bulk * locals.var_t0_dn6))))), (((((locals.var_c0bulk_dn8 * locals.var_t4) - (locals.var_c0bulk * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) * assign6930_body6_e4955) + (assign6930_body6_e4947 * ((-locals.var_t3_dn8) + ((locals.var_cnst1bulk_dn8 * locals.var_t0) + (locals.var_cnst1bulk * locals.var_t0_dn8))))), (((((locals.var_c0bulk_dn10 * locals.var_t4) - (locals.var_c0bulk * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) * assign6930_body6_e4955) + (assign6930_body6_e4947 * ((-locals.var_t3_dn10) + ((locals.var_cnst1bulk_dn10 * locals.var_t0) + (locals.var_cnst1bulk * locals.var_t0_dn10))))), (((((locals.var_c0bulk_dn11 * locals.var_t4) - (locals.var_c0bulk * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) * assign6930_body6_e4955) + (assign6930_body6_e4947 * ((-locals.var_t3_dn11) + ((locals.var_cnst1bulk_dn11 * locals.var_t0) + (locals.var_cnst1bulk * locals.var_t0_dn11))))), (((((locals.var_c0bulk_dn12 * locals.var_t4) - (locals.var_c0bulk * locals.var_t4_dn12)) / (locals.var_t4 * locals.var_t4)) * assign6930_body6_e4955) + (assign6930_body6_e4947 * ((-locals.var_t3_dn12) + ((locals.var_cnst1bulk_dn12 * locals.var_t0) + (locals.var_cnst1bulk * locals.var_t0_dn12))))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
            locals.var_t5 = assign6930_body6_e4958;
            locals.var_t5_dn0 = assign6930_body6_e4958_d_n0;
            locals.var_t5_dn2 = assign6930_body6_e4958_d_n2;
            locals.var_t5_dn4 = assign6930_body6_e4958_d_n4;
            locals.var_t5_dn5 = assign6930_body6_e4958_d_n5;
            locals.var_t5_dn6 = assign6930_body6_e4958_d_n6;
            locals.var_t5_dn8 = assign6930_body6_e4958_d_n8;
            locals.var_t5_dn10 = assign6930_body6_e4958_d_n10;
            locals.var_t5_dn11 = assign6930_body6_e4958_d_n11;
            locals.var_t5_dn12 = assign6930_body6_e4958_d_n12;
            locals.var_t5_rv = 0.0;
            let assign6930_body7_e4961: f64 = (-1e-8);
            let assign6930_body7_e4962: f64 = if locals.var_phi_s0_bulk_0 < assign6930_body7_e4961 { 1.0 } else { 0.0 };
            locals.var_guard70 = assign6930_body7_e4962;
            locals.var_guard70_rv = 0.0;
            let (assign6930_body8_e4976, assign6930_body8_e4976_d_n0, assign6930_body8_e4976_d_n2, assign6930_body8_e4976_d_n4, assign6930_body8_e4976_d_n5, assign6930_body8_e4976_d_n6, assign6930_body8_e4976_d_n8, assign6930_body8_e4976_d_n10, assign6930_body8_e4976_d_n11, assign6930_body8_e4976_d_n12,) = {
    if ((locals.var_guard69 == 0.0) && (locals.var_guard70 != 0.0)) {
        let assign6930_body8_e4970: f64 = (locals.var_t3 + locals.var_t2);
        let assign6930_body8_e4972: f64 = (assign6930_body8_e4970 - 1.0);
        let assign6930_body8_e4973: f64 = (assign6930_body8_e4972).sqrt();
        let assign6930_body8_e4974: f64 = (locals.var_t1 * assign6930_body8_e4973);
        (assign6930_body8_e4974, ((locals.var_t1_dn0 * assign6930_body8_e4973) + (locals.var_t1 * ((locals.var_t3_dn0 + locals.var_t2_dn0) / (2.0 * assign6930_body8_e4973)))), ((locals.var_t1_dn2 * assign6930_body8_e4973) + (locals.var_t1 * ((locals.var_t3_dn2 + locals.var_t2_dn2) / (2.0 * assign6930_body8_e4973)))), ((locals.var_t1_dn4 * assign6930_body8_e4973) + (locals.var_t1 * ((locals.var_t3_dn4 + locals.var_t2_dn4) / (2.0 * assign6930_body8_e4973)))), ((locals.var_t1_dn5 * assign6930_body8_e4973) + (locals.var_t1 * ((locals.var_t3_dn5 + locals.var_t2_dn5) / (2.0 * assign6930_body8_e4973)))), ((locals.var_t1_dn6 * assign6930_body8_e4973) + (locals.var_t1 * ((locals.var_t3_dn6 + locals.var_t2_dn6) / (2.0 * assign6930_body8_e4973)))), ((locals.var_t1_dn8 * assign6930_body8_e4973) + (locals.var_t1 * ((locals.var_t3_dn8 + locals.var_t2_dn8) / (2.0 * assign6930_body8_e4973)))), ((locals.var_t1_dn10 * assign6930_body8_e4973) + (locals.var_t1 * ((locals.var_t3_dn10 + locals.var_t2_dn10) / (2.0 * assign6930_body8_e4973)))), ((locals.var_t1_dn11 * assign6930_body8_e4973) + (locals.var_t1 * ((locals.var_t3_dn11 + locals.var_t2_dn11) / (2.0 * assign6930_body8_e4973)))), ((locals.var_t1_dn12 * assign6930_body8_e4973) + (locals.var_t1 * ((locals.var_t3_dn12 + locals.var_t2_dn12) / (2.0 * assign6930_body8_e4973)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
            locals.var_t4 = assign6930_body8_e4976;
            locals.var_t4_dn0 = assign6930_body8_e4976_d_n0;
            locals.var_t4_dn2 = assign6930_body8_e4976_d_n2;
            locals.var_t4_dn4 = assign6930_body8_e4976_d_n4;
            locals.var_t4_dn5 = assign6930_body8_e4976_d_n5;
            locals.var_t4_dn6 = assign6930_body8_e4976_d_n6;
            locals.var_t4_dn8 = assign6930_body8_e4976_d_n8;
            locals.var_t4_dn10 = assign6930_body8_e4976_d_n10;
            locals.var_t4_dn11 = assign6930_body8_e4976_d_n11;
            locals.var_t4_dn12 = assign6930_body8_e4976_d_n12;
            locals.var_t4_rv = 0.0;
            let (assign6930_body9_e4990, assign6930_body9_e4990_d_n0, assign6930_body9_e4990_d_n2, assign6930_body9_e4990_d_n4, assign6930_body9_e4990_d_n5, assign6930_body9_e4990_d_n6, assign6930_body9_e4990_d_n8, assign6930_body9_e4990_d_n10, assign6930_body9_e4990_d_n11, assign6930_body9_e4990_d_n12,) = {
    if ((locals.var_guard69 == 0.0) && (locals.var_guard70 != 0.0)) {
        let assign6930_body9_e4983: f64 = (locals.var_c0bulk / locals.var_t4);
        let assign6930_body9_e4985: f64 = (-locals.var_t3);
        let assign6930_body9_e4987: f64 = (assign6930_body9_e4985 + 1.0);
        let assign6930_body9_e4988: f64 = (assign6930_body9_e4983 * assign6930_body9_e4987);
        (assign6930_body9_e4988, (((((locals.var_c0bulk_dn0 * locals.var_t4) - (locals.var_c0bulk * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)) * assign6930_body9_e4987) + (assign6930_body9_e4983 * (-locals.var_t3_dn0))), (((((locals.var_c0bulk_dn2 * locals.var_t4) - (locals.var_c0bulk * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)) * assign6930_body9_e4987) + (assign6930_body9_e4983 * (-locals.var_t3_dn2))), (((((locals.var_c0bulk_dn4 * locals.var_t4) - (locals.var_c0bulk * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) * assign6930_body9_e4987) + (assign6930_body9_e4983 * (-locals.var_t3_dn4))), (((((locals.var_c0bulk_dn5 * locals.var_t4) - (locals.var_c0bulk * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) * assign6930_body9_e4987) + (assign6930_body9_e4983 * (-locals.var_t3_dn5))), (((((locals.var_c0bulk_dn6 * locals.var_t4) - (locals.var_c0bulk * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) * assign6930_body9_e4987) + (assign6930_body9_e4983 * (-locals.var_t3_dn6))), (((((locals.var_c0bulk_dn8 * locals.var_t4) - (locals.var_c0bulk * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) * assign6930_body9_e4987) + (assign6930_body9_e4983 * (-locals.var_t3_dn8))), (((((locals.var_c0bulk_dn10 * locals.var_t4) - (locals.var_c0bulk * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) * assign6930_body9_e4987) + (assign6930_body9_e4983 * (-locals.var_t3_dn10))), (((((locals.var_c0bulk_dn11 * locals.var_t4) - (locals.var_c0bulk * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) * assign6930_body9_e4987) + (assign6930_body9_e4983 * (-locals.var_t3_dn11))), (((((locals.var_c0bulk_dn12 * locals.var_t4) - (locals.var_c0bulk * locals.var_t4_dn12)) / (locals.var_t4 * locals.var_t4)) * assign6930_body9_e4987) + (assign6930_body9_e4983 * (-locals.var_t3_dn12))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
            locals.var_t5 = assign6930_body9_e4990;
            locals.var_t5_dn0 = assign6930_body9_e4990_d_n0;
            locals.var_t5_dn2 = assign6930_body9_e4990_d_n2;
            locals.var_t5_dn4 = assign6930_body9_e4990_d_n4;
            locals.var_t5_dn5 = assign6930_body9_e4990_d_n5;
            locals.var_t5_dn6 = assign6930_body9_e4990_d_n6;
            locals.var_t5_dn8 = assign6930_body9_e4990_d_n8;
            locals.var_t5_dn10 = assign6930_body9_e4990_d_n10;
            locals.var_t5_dn11 = assign6930_body9_e4990_d_n11;
            locals.var_t5_dn12 = assign6930_body9_e4990_d_n12;
            locals.var_t5_rv = 0.0;
            let (assign6930_body10_e5006, assign6930_body10_e5006_d_n0, assign6930_body10_e5006_d_n2, assign6930_body10_e5006_d_n4, assign6930_body10_e5006_d_n5, assign6930_body10_e5006_d_n6, assign6930_body10_e5006_d_n8, assign6930_body10_e5006_d_n10, assign6930_body10_e5006_d_n11, assign6930_body10_e5006_d_n12,) = {
    if ((locals.var_guard69 == 0.0) && (locals.var_guard70 == 0.0)) {
        let assign6930_body10_e4998: f64 = (locals.var_c0bulk / locals.var_beta);
        let assign6930_body10_e4999: f64 = (assign6930_body10_e4998).sqrt();
        let assign6930_body10_e5000: f64 = (-assign6930_body10_e4999);
        let assign6930_body10_e5002: f64 = (assign6930_body10_e5000 * locals.var_beta);
        let assign6930_body10_e5004: f64 = (assign6930_body10_e5002 * locals.var_phi_s0_bulk_0);
        (assign6930_body10_e5004, ((((-((locals.var_c0bulk_dn0 / locals.var_beta) / (2.0 * assign6930_body10_e4999))) * locals.var_beta) * locals.var_phi_s0_bulk_0) + (assign6930_body10_e5002 * locals.var_phi_s0_bulk_0_dn0)), ((((-((locals.var_c0bulk_dn2 / locals.var_beta) / (2.0 * assign6930_body10_e4999))) * locals.var_beta) * locals.var_phi_s0_bulk_0) + (assign6930_body10_e5002 * locals.var_phi_s0_bulk_0_dn2)), (((((-((((locals.var_c0bulk_dn4 * locals.var_beta) - (locals.var_c0bulk * locals.var_beta_dn4)) / (locals.var_beta * locals.var_beta)) / (2.0 * assign6930_body10_e4999))) * locals.var_beta) + (assign6930_body10_e5000 * locals.var_beta_dn4)) * locals.var_phi_s0_bulk_0) + (assign6930_body10_e5002 * locals.var_phi_s0_bulk_0_dn4)), ((((-((locals.var_c0bulk_dn5 / locals.var_beta) / (2.0 * assign6930_body10_e4999))) * locals.var_beta) * locals.var_phi_s0_bulk_0) + (assign6930_body10_e5002 * locals.var_phi_s0_bulk_0_dn5)), ((((-((locals.var_c0bulk_dn6 / locals.var_beta) / (2.0 * assign6930_body10_e4999))) * locals.var_beta) * locals.var_phi_s0_bulk_0) + (assign6930_body10_e5002 * locals.var_phi_s0_bulk_0_dn6)), ((((-((locals.var_c0bulk_dn8 / locals.var_beta) / (2.0 * assign6930_body10_e4999))) * locals.var_beta) * locals.var_phi_s0_bulk_0) + (assign6930_body10_e5002 * locals.var_phi_s0_bulk_0_dn8)), ((((-((locals.var_c0bulk_dn10 / locals.var_beta) / (2.0 * assign6930_body10_e4999))) * locals.var_beta) * locals.var_phi_s0_bulk_0) + (assign6930_body10_e5002 * locals.var_phi_s0_bulk_0_dn10)), ((((-((locals.var_c0bulk_dn11 / locals.var_beta) / (2.0 * assign6930_body10_e4999))) * locals.var_beta) * locals.var_phi_s0_bulk_0) + (assign6930_body10_e5002 * locals.var_phi_s0_bulk_0_dn11)), ((((-((locals.var_c0bulk_dn12 / locals.var_beta) / (2.0 * assign6930_body10_e4999))) * locals.var_beta) * locals.var_phi_s0_bulk_0) + (assign6930_body10_e5002 * locals.var_phi_s0_bulk_0_dn12)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
            locals.var_t4 = assign6930_body10_e5006;
            locals.var_t4_dn0 = assign6930_body10_e5006_d_n0;
            locals.var_t4_dn2 = assign6930_body10_e5006_d_n2;
            locals.var_t4_dn4 = assign6930_body10_e5006_d_n4;
            locals.var_t4_dn5 = assign6930_body10_e5006_d_n5;
            locals.var_t4_dn6 = assign6930_body10_e5006_d_n6;
            locals.var_t4_dn8 = assign6930_body10_e5006_d_n8;
            locals.var_t4_dn10 = assign6930_body10_e5006_d_n10;
            locals.var_t4_dn11 = assign6930_body10_e5006_d_n11;
            locals.var_t4_dn12 = assign6930_body10_e5006_d_n12;
            locals.var_t4_rv = 0.0;
            let (assign6930_body11_e5018, assign6930_body11_e5018_d_n0, assign6930_body11_e5018_d_n2, assign6930_body11_e5018_d_n4, assign6930_body11_e5018_d_n5, assign6930_body11_e5018_d_n6, assign6930_body11_e5018_d_n8, assign6930_body11_e5018_d_n10, assign6930_body11_e5018_d_n11, assign6930_body11_e5018_d_n12,) = {
    if ((locals.var_guard69 == 0.0) && (locals.var_guard70 == 0.0)) {
        let assign6930_body11_e5014: f64 = (locals.var_c0bulk * locals.var_beta);
        let assign6930_body11_e5015: f64 = (assign6930_body11_e5014).sqrt();
        let assign6930_body11_e5016: f64 = (-assign6930_body11_e5015);
        (assign6930_body11_e5016, (-((locals.var_c0bulk_dn0 * locals.var_beta) / (2.0 * assign6930_body11_e5015))), (-((locals.var_c0bulk_dn2 * locals.var_beta) / (2.0 * assign6930_body11_e5015))), (-(((locals.var_c0bulk_dn4 * locals.var_beta) + (locals.var_c0bulk * locals.var_beta_dn4)) / (2.0 * assign6930_body11_e5015))), (-((locals.var_c0bulk_dn5 * locals.var_beta) / (2.0 * assign6930_body11_e5015))), (-((locals.var_c0bulk_dn6 * locals.var_beta) / (2.0 * assign6930_body11_e5015))), (-((locals.var_c0bulk_dn8 * locals.var_beta) / (2.0 * assign6930_body11_e5015))), (-((locals.var_c0bulk_dn10 * locals.var_beta) / (2.0 * assign6930_body11_e5015))), (-((locals.var_c0bulk_dn11 * locals.var_beta) / (2.0 * assign6930_body11_e5015))), (-((locals.var_c0bulk_dn12 * locals.var_beta) / (2.0 * assign6930_body11_e5015))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
            locals.var_t5 = assign6930_body11_e5018;
            locals.var_t5_dn0 = assign6930_body11_e5018_d_n0;
            locals.var_t5_dn2 = assign6930_body11_e5018_d_n2;
            locals.var_t5_dn4 = assign6930_body11_e5018_d_n4;
            locals.var_t5_dn5 = assign6930_body11_e5018_d_n5;
            locals.var_t5_dn6 = assign6930_body11_e5018_d_n6;
            locals.var_t5_dn8 = assign6930_body11_e5018_d_n8;
            locals.var_t5_dn10 = assign6930_body11_e5018_d_n10;
            locals.var_t5_dn11 = assign6930_body11_e5018_d_n11;
            locals.var_t5_dn12 = assign6930_body11_e5018_d_n12;
            locals.var_t5_rv = 0.0;
            let assign6930_body12_e5021: f64 = (locals.var_t4 * locals.var_t4);
            let assign6930_body12_e5024: f64 = (4.0 * 1e-6);
            let assign6930_body12_e5026: f64 = (assign6930_body12_e5024 * 1e-6);
            let assign6930_body12_e5027: f64 = (assign6930_body12_e5021 + assign6930_body12_e5026);
            let assign6930_body12_e5028: f64 = (assign6930_body12_e5027).sqrt();
            locals.var_tmf2 = assign6930_body12_e5028;
            locals.var_tmf2_dn0 = (((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)) / (2.0 * assign6930_body12_e5028));
            locals.var_tmf2_dn2 = (((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)) / (2.0 * assign6930_body12_e5028));
            locals.var_tmf2_dn4 = (((locals.var_t4_dn4 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn4)) / (2.0 * assign6930_body12_e5028));
            locals.var_tmf2_dn5 = (((locals.var_t4_dn5 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn5)) / (2.0 * assign6930_body12_e5028));
            locals.var_tmf2_dn6 = (((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)) / (2.0 * assign6930_body12_e5028));
            locals.var_tmf2_dn8 = (((locals.var_t4_dn8 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn8)) / (2.0 * assign6930_body12_e5028));
            locals.var_tmf2_dn10 = (((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)) / (2.0 * assign6930_body12_e5028));
            locals.var_tmf2_dn11 = (((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)) / (2.0 * assign6930_body12_e5028));
            locals.var_tmf2_dn12 = (((locals.var_t4_dn12 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn12)) / (2.0 * assign6930_body12_e5028));
            locals.var_tmf2_rv = 0.0;
            let assign6930_body13_e5033: f64 = (locals.var_t4 / locals.var_tmf2);
            let assign6930_body13_e5034: f64 = (1.0 + assign6930_body13_e5033);
            let assign6930_body13_e5035: f64 = (0.5 * assign6930_body13_e5034);
            locals.var_t7 = assign6930_body13_e5035;
            locals.var_t7_dn0 = (0.5 * (((locals.var_t4_dn0 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
            locals.var_t7_dn2 = (0.5 * (((locals.var_t4_dn2 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
            locals.var_t7_dn4 = (0.5 * (((locals.var_t4_dn4 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
            locals.var_t7_dn5 = (0.5 * (((locals.var_t4_dn5 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
            locals.var_t7_dn6 = (0.5 * (((locals.var_t4_dn6 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
            locals.var_t7_dn8 = (0.5 * (((locals.var_t4_dn8 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
            locals.var_t7_dn10 = (0.5 * (((locals.var_t4_dn10 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
            locals.var_t7_dn11 = (0.5 * (((locals.var_t4_dn11 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)));
            locals.var_t7_dn12 = (0.5 * (((locals.var_t4_dn12 * locals.var_tmf2) - (locals.var_t4 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)));
            locals.var_t7_rv = 0.0;
            let assign6930_body14_e5039: f64 = (locals.var_t4 + locals.var_tmf2);
            let assign6930_body14_e5040: f64 = (0.5 * assign6930_body14_e5039);
            let assign6930_body14_e5043: f64 = (1e-10 * 1e-6);
            let assign6930_body14_e5044: f64 = (assign6930_body14_e5040 + assign6930_body14_e5043);
            locals.var_t6 = assign6930_body14_e5044;
            locals.var_t6_dn0 = (0.5 * (locals.var_t4_dn0 + locals.var_tmf2_dn0));
            locals.var_t6_dn2 = (0.5 * (locals.var_t4_dn2 + locals.var_tmf2_dn2));
            locals.var_t6_dn4 = (0.5 * (locals.var_t4_dn4 + locals.var_tmf2_dn4));
            locals.var_t6_dn5 = (0.5 * (locals.var_t4_dn5 + locals.var_tmf2_dn5));
            locals.var_t6_dn6 = (0.5 * (locals.var_t4_dn6 + locals.var_tmf2_dn6));
            locals.var_t6_dn8 = (0.5 * (locals.var_t4_dn8 + locals.var_tmf2_dn8));
            locals.var_t6_dn10 = (0.5 * (locals.var_t4_dn10 + locals.var_tmf2_dn10));
            locals.var_t6_dn11 = (0.5 * (locals.var_t4_dn11 + locals.var_tmf2_dn11));
            locals.var_t6_dn12 = (0.5 * (locals.var_t4_dn12 + locals.var_tmf2_dn12));
            locals.var_t6_rv = 0.0;
            let assign6930_body15_e5047: f64 = if locals.var_t6 < 0.0 { 1.0 } else { 0.0 };
            locals.var_guard71 = assign6930_body15_e5047;
            locals.var_guard71_rv = 0.0;
            let (assign6930_body16_e5051, assign6930_body16_e5051_d_n0, assign6930_body16_e5051_d_n2, assign6930_body16_e5051_d_n4, assign6930_body16_e5051_d_n5, assign6930_body16_e5051_d_n6, assign6930_body16_e5051_d_n8, assign6930_body16_e5051_d_n10, assign6930_body16_e5051_d_n11, assign6930_body16_e5051_d_n12,) = {
    if (locals.var_guard71 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn8, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12,)
    }
};
            locals.var_t6 = assign6930_body16_e5051;
            locals.var_t6_dn0 = assign6930_body16_e5051_d_n0;
            locals.var_t6_dn2 = assign6930_body16_e5051_d_n2;
            locals.var_t6_dn4 = assign6930_body16_e5051_d_n4;
            locals.var_t6_dn5 = assign6930_body16_e5051_d_n5;
            locals.var_t6_dn6 = assign6930_body16_e5051_d_n6;
            locals.var_t6_dn8 = assign6930_body16_e5051_d_n8;
            locals.var_t6_dn10 = assign6930_body16_e5051_d_n10;
            locals.var_t6_dn11 = assign6930_body16_e5051_d_n11;
            locals.var_t6_dn12 = assign6930_body16_e5051_d_n12;
            locals.var_t6_rv = 0.0;
            let (assign6930_body17_e5055, assign6930_body17_e5055_d_n0, assign6930_body17_e5055_d_n2, assign6930_body17_e5055_d_n4, assign6930_body17_e5055_d_n5, assign6930_body17_e5055_d_n6, assign6930_body17_e5055_d_n8, assign6930_body17_e5055_d_n10, assign6930_body17_e5055_d_n11, assign6930_body17_e5055_d_n12,) = {
    if (locals.var_guard71 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn8, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12,)
    }
};
            locals.var_t7 = assign6930_body17_e5055;
            locals.var_t7_dn0 = assign6930_body17_e5055_d_n0;
            locals.var_t7_dn2 = assign6930_body17_e5055_d_n2;
            locals.var_t7_dn4 = assign6930_body17_e5055_d_n4;
            locals.var_t7_dn5 = assign6930_body17_e5055_d_n5;
            locals.var_t7_dn6 = assign6930_body17_e5055_d_n6;
            locals.var_t7_dn8 = assign6930_body17_e5055_d_n8;
            locals.var_t7_dn10 = assign6930_body17_e5055_d_n10;
            locals.var_t7_dn11 = assign6930_body17_e5055_d_n11;
            locals.var_t7_dn12 = assign6930_body17_e5055_d_n12;
            locals.var_t7_rv = 0.0;
            let assign6930_body18_e5057: f64 = (-locals.var_q_fd_soi);
            let assign6930_body18_e5059: f64 = (assign6930_body18_e5057 - locals.var_t6);
            let assign6930_body18_e5061: f64 = (assign6930_body18_e5059 - 1e-9);
            locals.var_tmf1 = assign6930_body18_e5061;
            locals.var_tmf1_dn0 = ((-locals.var_q_fd_soi_dn0) - locals.var_t6_dn0);
            locals.var_tmf1_dn2 = ((-locals.var_q_fd_soi_dn2) - locals.var_t6_dn2);
            locals.var_tmf1_dn4 = ((-locals.var_q_fd_soi_dn4) - locals.var_t6_dn4);
            locals.var_tmf1_dn5 = ((-locals.var_q_fd_soi_dn5) - locals.var_t6_dn5);
            locals.var_tmf1_dn6 = ((-locals.var_q_fd_soi_dn6) - locals.var_t6_dn6);
            locals.var_tmf1_dn8 = ((-locals.var_q_fd_soi_dn8) - locals.var_t6_dn8);
            locals.var_tmf1_dn10 = ((-locals.var_q_fd_soi_dn10) - locals.var_t6_dn10);
            locals.var_tmf1_dn11 = ((-locals.var_q_fd_soi_dn11) - locals.var_t6_dn11);
            locals.var_tmf1_dn12 = ((-locals.var_q_fd_soi_dn12) - locals.var_t6_dn12);
            locals.var_tmf1_rv = 0.0;
            let assign6930_body19_e5064: f64 = (-locals.var_q_fd_soi);
            let assign6930_body19_e5065: f64 = (4.0 * assign6930_body19_e5064);
            let assign6930_body19_e5067: f64 = (assign6930_body19_e5065 * 1e-9);
            locals.var_tmf2 = assign6930_body19_e5067;
            locals.var_tmf2_dn0 = ((4.0 * (-locals.var_q_fd_soi_dn0)) * 1e-9);
            locals.var_tmf2_dn2 = ((4.0 * (-locals.var_q_fd_soi_dn2)) * 1e-9);
            locals.var_tmf2_dn4 = ((4.0 * (-locals.var_q_fd_soi_dn4)) * 1e-9);
            locals.var_tmf2_dn5 = ((4.0 * (-locals.var_q_fd_soi_dn5)) * 1e-9);
            locals.var_tmf2_dn6 = ((4.0 * (-locals.var_q_fd_soi_dn6)) * 1e-9);
            locals.var_tmf2_dn8 = ((4.0 * (-locals.var_q_fd_soi_dn8)) * 1e-9);
            locals.var_tmf2_dn10 = ((4.0 * (-locals.var_q_fd_soi_dn10)) * 1e-9);
            locals.var_tmf2_dn11 = ((4.0 * (-locals.var_q_fd_soi_dn11)) * 1e-9);
            locals.var_tmf2_dn12 = ((4.0 * (-locals.var_q_fd_soi_dn12)) * 1e-9);
            locals.var_tmf2_rv = 0.0;
            let (assign6930_body20_e5074, assign6930_body20_e5074_d_n0, assign6930_body20_e5074_d_n2, assign6930_body20_e5074_d_n4, assign6930_body20_e5074_d_n5, assign6930_body20_e5074_d_n6, assign6930_body20_e5074_d_n8, assign6930_body20_e5074_d_n10, assign6930_body20_e5074_d_n11, assign6930_body20_e5074_d_n12,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn8, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn12,)
    } else {
        let assign6930_body20_e5073: f64 = (-locals.var_tmf2);
        (assign6930_body20_e5073, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn12),)
    }
};
            locals.var_tmf2 = assign6930_body20_e5074;
            locals.var_tmf2_dn0 = assign6930_body20_e5074_d_n0;
            locals.var_tmf2_dn2 = assign6930_body20_e5074_d_n2;
            locals.var_tmf2_dn4 = assign6930_body20_e5074_d_n4;
            locals.var_tmf2_dn5 = assign6930_body20_e5074_d_n5;
            locals.var_tmf2_dn6 = assign6930_body20_e5074_d_n6;
            locals.var_tmf2_dn8 = assign6930_body20_e5074_d_n8;
            locals.var_tmf2_dn10 = assign6930_body20_e5074_d_n10;
            locals.var_tmf2_dn11 = assign6930_body20_e5074_d_n11;
            locals.var_tmf2_dn12 = assign6930_body20_e5074_d_n12;
            locals.var_tmf2_rv = 0.0;
            let assign6930_body21_e5077: f64 = (locals.var_tmf1 * locals.var_tmf1);
            let assign6930_body21_e5079: f64 = (assign6930_body21_e5077 + locals.var_tmf2);
            let assign6930_body21_e5080: f64 = (assign6930_body21_e5079).sqrt();
            locals.var_tmf2 = assign6930_body21_e5080;
            locals.var_tmf2_dn0 = ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign6930_body21_e5080));
            locals.var_tmf2_dn2 = ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign6930_body21_e5080));
            locals.var_tmf2_dn4 = ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign6930_body21_e5080));
            locals.var_tmf2_dn5 = ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign6930_body21_e5080));
            locals.var_tmf2_dn6 = ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign6930_body21_e5080));
            locals.var_tmf2_dn8 = ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign6930_body21_e5080));
            locals.var_tmf2_dn10 = ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign6930_body21_e5080));
            locals.var_tmf2_dn11 = ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign6930_body21_e5080));
            locals.var_tmf2_dn12 = ((((locals.var_tmf1_dn12 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn12)) + locals.var_tmf2_dn12) / (2.0 * assign6930_body21_e5080));
            locals.var_tmf2_rv = 0.0;
            let assign6930_body22_e5085: f64 = (locals.var_tmf1 / locals.var_tmf2);
            let assign6930_body22_e5086: f64 = (1.0 + assign6930_body22_e5085);
            let assign6930_body22_e5087: f64 = (0.5 * assign6930_body22_e5086);
            locals.var_t8 = assign6930_body22_e5087;
            locals.var_t8_dn0 = (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
            locals.var_t8_dn2 = (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
            locals.var_t8_dn4 = (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
            locals.var_t8_dn5 = (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
            locals.var_t8_dn6 = (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
            locals.var_t8_dn8 = (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
            locals.var_t8_dn10 = (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
            locals.var_t8_dn11 = (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)));
            locals.var_t8_dn12 = (0.5 * (((locals.var_tmf1_dn12 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn12)) / (locals.var_tmf2 * locals.var_tmf2)));
            locals.var_t8_rv = 0.0;
            let assign6930_body23_e5089: f64 = (-locals.var_q_fd_soi);
            let assign6930_body23_e5093: f64 = (locals.var_tmf1 + locals.var_tmf2);
            let assign6930_body23_e5094: f64 = (0.5 * assign6930_body23_e5093);
            let assign6930_body23_e5095: f64 = (assign6930_body23_e5089 - assign6930_body23_e5094);
            locals.var_t6 = assign6930_body23_e5095;
            locals.var_t6_dn0 = ((-locals.var_q_fd_soi_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)));
            locals.var_t6_dn2 = ((-locals.var_q_fd_soi_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)));
            locals.var_t6_dn4 = ((-locals.var_q_fd_soi_dn4) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)));
            locals.var_t6_dn5 = ((-locals.var_q_fd_soi_dn5) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)));
            locals.var_t6_dn6 = ((-locals.var_q_fd_soi_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)));
            locals.var_t6_dn8 = ((-locals.var_q_fd_soi_dn8) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)));
            locals.var_t6_dn10 = ((-locals.var_q_fd_soi_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)));
            locals.var_t6_dn11 = ((-locals.var_q_fd_soi_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)));
            locals.var_t6_dn12 = ((-locals.var_q_fd_soi_dn12) - (0.5 * (locals.var_tmf1_dn12 + locals.var_tmf2_dn12)));
            locals.var_t6_rv = 0.0;
            let assign6930_body24_e5099: f64 = (locals.var_t5 * locals.var_t8);
            let assign6930_body24_e5100: f64 = (locals.var_t7 * assign6930_body24_e5099);
            locals.var_t7 = assign6930_body24_e5100;
            locals.var_t7_dn0 = ((locals.var_t7_dn0 * assign6930_body24_e5099) + (locals.var_t7 * ((locals.var_t5_dn0 * locals.var_t8) + (locals.var_t5 * locals.var_t8_dn0))));
            locals.var_t7_dn2 = ((locals.var_t7_dn2 * assign6930_body24_e5099) + (locals.var_t7 * ((locals.var_t5_dn2 * locals.var_t8) + (locals.var_t5 * locals.var_t8_dn2))));
            locals.var_t7_dn4 = ((locals.var_t7_dn4 * assign6930_body24_e5099) + (locals.var_t7 * ((locals.var_t5_dn4 * locals.var_t8) + (locals.var_t5 * locals.var_t8_dn4))));
            locals.var_t7_dn5 = ((locals.var_t7_dn5 * assign6930_body24_e5099) + (locals.var_t7 * ((locals.var_t5_dn5 * locals.var_t8) + (locals.var_t5 * locals.var_t8_dn5))));
            locals.var_t7_dn6 = ((locals.var_t7_dn6 * assign6930_body24_e5099) + (locals.var_t7 * ((locals.var_t5_dn6 * locals.var_t8) + (locals.var_t5 * locals.var_t8_dn6))));
            locals.var_t7_dn8 = ((locals.var_t7_dn8 * assign6930_body24_e5099) + (locals.var_t7 * ((locals.var_t5_dn8 * locals.var_t8) + (locals.var_t5 * locals.var_t8_dn8))));
            locals.var_t7_dn10 = ((locals.var_t7_dn10 * assign6930_body24_e5099) + (locals.var_t7 * ((locals.var_t5_dn10 * locals.var_t8) + (locals.var_t5 * locals.var_t8_dn10))));
            locals.var_t7_dn11 = ((locals.var_t7_dn11 * assign6930_body24_e5099) + (locals.var_t7 * ((locals.var_t5_dn11 * locals.var_t8) + (locals.var_t5 * locals.var_t8_dn11))));
            locals.var_t7_dn12 = ((locals.var_t7_dn12 * assign6930_body24_e5099) + (locals.var_t7 * ((locals.var_t5_dn12 * locals.var_t8) + (locals.var_t5 * locals.var_t8_dn12))));
            locals.var_t7_rv = 0.0;
            let assign6930_body25_e5103: f64 = (locals.var_t6 * locals.var_t6);
            let assign6930_body25_e5105: f64 = (assign6930_body25_e5103 / 2.0);
            let assign6930_body25_e5107: f64 = (assign6930_body25_e5105 / 1.034943e-10);
            let assign6930_body25_e5109: f64 = (assign6930_body25_e5107 / 1.6021918e-19);
            let assign6930_body25_e5111: f64 = (assign6930_body25_e5109 / locals.var_uc_nsubs);
            locals.var_phi_b_dep0 = assign6930_body25_e5111;
            locals.var_phi_b_dep0_dn0 = ((((((((locals.var_t6_dn0 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn0)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign6930_body25_e5109 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs));
            locals.var_phi_b_dep0_dn2 = ((((((((locals.var_t6_dn2 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn2)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign6930_body25_e5109 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs));
            locals.var_phi_b_dep0_dn4 = ((((((((locals.var_t6_dn4 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn4)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign6930_body25_e5109 * locals.var_uc_nsubs_dn4)) / (locals.var_uc_nsubs * locals.var_uc_nsubs));
            locals.var_phi_b_dep0_dn5 = ((((((((locals.var_t6_dn5 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn5)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign6930_body25_e5109 * locals.var_uc_nsubs_dn5)) / (locals.var_uc_nsubs * locals.var_uc_nsubs));
            locals.var_phi_b_dep0_dn6 = ((((((((locals.var_t6_dn6 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn6)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign6930_body25_e5109 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs));
            locals.var_phi_b_dep0_dn8 = ((((((((locals.var_t6_dn8 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn8)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign6930_body25_e5109 * locals.var_uc_nsubs_dn8)) / (locals.var_uc_nsubs * locals.var_uc_nsubs));
            locals.var_phi_b_dep0_dn10 = ((((((((locals.var_t6_dn10 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn10)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign6930_body25_e5109 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs));
            locals.var_phi_b_dep0_dn11 = ((((((((locals.var_t6_dn11 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn11)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign6930_body25_e5109 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs));
            locals.var_phi_b_dep0_dn12 = ((((((((locals.var_t6_dn12 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn12)) / 2.0) / 1.034943e-10) / 1.6021918e-19) * locals.var_uc_nsubs) - (assign6930_body25_e5109 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs));
            locals.var_phi_b_dep0_rv = 0.0;
            let assign6930_body26_e5114: f64 = (2.0 * locals.var_phi_b_dep0);
            let assign6930_body26_e5116: f64 = (assign6930_body26_e5114 * locals.var_t7);
            let assign6930_body26_e5118: f64 = (assign6930_body26_e5116 / locals.var_t6);
            locals.var_phi_b_dep0_dpsb = assign6930_body26_e5118;
            locals.var_phi_b_dep0_dpsb_dn0 = ((((((2.0 * locals.var_phi_b_dep0_dn0) * locals.var_t7) + (assign6930_body26_e5114 * locals.var_t7_dn0)) * locals.var_t6) - (assign6930_body26_e5116 * locals.var_t6_dn0)) / (locals.var_t6 * locals.var_t6));
            locals.var_phi_b_dep0_dpsb_dn2 = ((((((2.0 * locals.var_phi_b_dep0_dn2) * locals.var_t7) + (assign6930_body26_e5114 * locals.var_t7_dn2)) * locals.var_t6) - (assign6930_body26_e5116 * locals.var_t6_dn2)) / (locals.var_t6 * locals.var_t6));
            locals.var_phi_b_dep0_dpsb_dn4 = ((((((2.0 * locals.var_phi_b_dep0_dn4) * locals.var_t7) + (assign6930_body26_e5114 * locals.var_t7_dn4)) * locals.var_t6) - (assign6930_body26_e5116 * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6));
            locals.var_phi_b_dep0_dpsb_dn5 = ((((((2.0 * locals.var_phi_b_dep0_dn5) * locals.var_t7) + (assign6930_body26_e5114 * locals.var_t7_dn5)) * locals.var_t6) - (assign6930_body26_e5116 * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6));
            locals.var_phi_b_dep0_dpsb_dn6 = ((((((2.0 * locals.var_phi_b_dep0_dn6) * locals.var_t7) + (assign6930_body26_e5114 * locals.var_t7_dn6)) * locals.var_t6) - (assign6930_body26_e5116 * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6));
            locals.var_phi_b_dep0_dpsb_dn8 = ((((((2.0 * locals.var_phi_b_dep0_dn8) * locals.var_t7) + (assign6930_body26_e5114 * locals.var_t7_dn8)) * locals.var_t6) - (assign6930_body26_e5116 * locals.var_t6_dn8)) / (locals.var_t6 * locals.var_t6));
            locals.var_phi_b_dep0_dpsb_dn10 = ((((((2.0 * locals.var_phi_b_dep0_dn10) * locals.var_t7) + (assign6930_body26_e5114 * locals.var_t7_dn10)) * locals.var_t6) - (assign6930_body26_e5116 * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6));
            locals.var_phi_b_dep0_dpsb_dn11 = ((((((2.0 * locals.var_phi_b_dep0_dn11) * locals.var_t7) + (assign6930_body26_e5114 * locals.var_t7_dn11)) * locals.var_t6) - (assign6930_body26_e5116 * locals.var_t6_dn11)) / (locals.var_t6 * locals.var_t6));
            locals.var_phi_b_dep0_dpsb_dn12 = ((((((2.0 * locals.var_phi_b_dep0_dn12) * locals.var_t7) + (assign6930_body26_e5114 * locals.var_t7_dn12)) * locals.var_t6) - (assign6930_body26_e5116 * locals.var_t6_dn12)) / (locals.var_t6 * locals.var_t6));
            locals.var_phi_b_dep0_dpsb_rv = 0.0;
            let assign6930_body27_e5121: f64 = (-locals.var_phi_s0_bulk_0);
            let assign6930_body27_e5124: f64 = (locals.var_t4 / locals.var_c_box);
            let assign6930_body27_e5125: f64 = (assign6930_body27_e5121 + assign6930_body27_e5124);
            let assign6930_body27_e5127: f64 = (assign6930_body27_e5125 - locals.var_vbsbiz);
            let assign6930_body27_e5129: f64 = (assign6930_body27_e5127 + locals.var_phi_b_dep0);
            let assign6930_body27_e5131: f64 = (-1.0);
            let assign6930_body27_e5134: f64 = (locals.var_t5 / locals.var_c_box);
            let assign6930_body27_e5135: f64 = (assign6930_body27_e5131 + assign6930_body27_e5134);
            let assign6930_body27_e5137: f64 = (assign6930_body27_e5135 + locals.var_phi_b_dep0_dpsb);
            let assign6930_body27_e5138: f64 = (assign6930_body27_e5129 / assign6930_body27_e5137);
            let assign6930_body27_e5139: f64 = (locals.var_phi_s0_bulk_0 - assign6930_body27_e5138);
            locals.var_t6 = assign6930_body27_e5139;
            locals.var_t6_dn0 = (locals.var_phi_s0_bulk_0_dn0 - (((((((-locals.var_phi_s0_bulk_0_dn0) + (locals.var_t4_dn0 / locals.var_c_box)) - locals.var_vbsbiz_dn0) + locals.var_phi_b_dep0_dn0) * assign6930_body27_e5137) - (assign6930_body27_e5129 * ((locals.var_t5_dn0 / locals.var_c_box) + locals.var_phi_b_dep0_dpsb_dn0))) / (assign6930_body27_e5137 * assign6930_body27_e5137)));
            locals.var_t6_dn2 = (locals.var_phi_s0_bulk_0_dn2 - (((((((-locals.var_phi_s0_bulk_0_dn2) + (locals.var_t4_dn2 / locals.var_c_box)) - locals.var_vbsbiz_dn2) + locals.var_phi_b_dep0_dn2) * assign6930_body27_e5137) - (assign6930_body27_e5129 * ((locals.var_t5_dn2 / locals.var_c_box) + locals.var_phi_b_dep0_dpsb_dn2))) / (assign6930_body27_e5137 * assign6930_body27_e5137)));
            locals.var_t6_dn4 = (locals.var_phi_s0_bulk_0_dn4 - (((((((-locals.var_phi_s0_bulk_0_dn4) + (locals.var_t4_dn4 / locals.var_c_box)) - locals.var_vbsbiz_dn4) + locals.var_phi_b_dep0_dn4) * assign6930_body27_e5137) - (assign6930_body27_e5129 * ((locals.var_t5_dn4 / locals.var_c_box) + locals.var_phi_b_dep0_dpsb_dn4))) / (assign6930_body27_e5137 * assign6930_body27_e5137)));
            locals.var_t6_dn5 = (locals.var_phi_s0_bulk_0_dn5 - (((((((-locals.var_phi_s0_bulk_0_dn5) + (locals.var_t4_dn5 / locals.var_c_box)) - locals.var_vbsbiz_dn5) + locals.var_phi_b_dep0_dn5) * assign6930_body27_e5137) - (assign6930_body27_e5129 * ((locals.var_t5_dn5 / locals.var_c_box) + locals.var_phi_b_dep0_dpsb_dn5))) / (assign6930_body27_e5137 * assign6930_body27_e5137)));
            locals.var_t6_dn6 = (locals.var_phi_s0_bulk_0_dn6 - (((((((-locals.var_phi_s0_bulk_0_dn6) + (locals.var_t4_dn6 / locals.var_c_box)) - locals.var_vbsbiz_dn6) + locals.var_phi_b_dep0_dn6) * assign6930_body27_e5137) - (assign6930_body27_e5129 * ((locals.var_t5_dn6 / locals.var_c_box) + locals.var_phi_b_dep0_dpsb_dn6))) / (assign6930_body27_e5137 * assign6930_body27_e5137)));
            locals.var_t6_dn8 = (locals.var_phi_s0_bulk_0_dn8 - (((((((-locals.var_phi_s0_bulk_0_dn8) + (locals.var_t4_dn8 / locals.var_c_box)) - locals.var_vbsbiz_dn8) + locals.var_phi_b_dep0_dn8) * assign6930_body27_e5137) - (assign6930_body27_e5129 * ((locals.var_t5_dn8 / locals.var_c_box) + locals.var_phi_b_dep0_dpsb_dn8))) / (assign6930_body27_e5137 * assign6930_body27_e5137)));
            locals.var_t6_dn10 = (locals.var_phi_s0_bulk_0_dn10 - (((((((-locals.var_phi_s0_bulk_0_dn10) + (locals.var_t4_dn10 / locals.var_c_box)) - locals.var_vbsbiz_dn10) + locals.var_phi_b_dep0_dn10) * assign6930_body27_e5137) - (assign6930_body27_e5129 * ((locals.var_t5_dn10 / locals.var_c_box) + locals.var_phi_b_dep0_dpsb_dn10))) / (assign6930_body27_e5137 * assign6930_body27_e5137)));
            locals.var_t6_dn11 = (locals.var_phi_s0_bulk_0_dn11 - (((((((-locals.var_phi_s0_bulk_0_dn11) + (locals.var_t4_dn11 / locals.var_c_box)) - locals.var_vbsbiz_dn11) + locals.var_phi_b_dep0_dn11) * assign6930_body27_e5137) - (assign6930_body27_e5129 * ((locals.var_t5_dn11 / locals.var_c_box) + locals.var_phi_b_dep0_dpsb_dn11))) / (assign6930_body27_e5137 * assign6930_body27_e5137)));
            locals.var_t6_dn12 = (locals.var_phi_s0_bulk_0_dn12 - (((((((-locals.var_phi_s0_bulk_0_dn12) + (locals.var_t4_dn12 / locals.var_c_box)) - locals.var_vbsbiz_dn12) + locals.var_phi_b_dep0_dn12) * assign6930_body27_e5137) - (assign6930_body27_e5129 * ((locals.var_t5_dn12 / locals.var_c_box) + locals.var_phi_b_dep0_dpsb_dn12))) / (assign6930_body27_e5137 * assign6930_body27_e5137)));
            locals.var_t6_rv = 0.0;
            let assign6930_body28_e5142: f64 = (locals.var_t6 - locals.var_phi_s0_bulk_0);
            let assign6930_body28_e5143: f64 = (assign6930_body28_e5142).abs();
            let assign6930_body28_e5145: f64 = if assign6930_body28_e5143 < 0.001 { 1.0 } else { 0.0 };
            locals.var_guard72 = assign6930_body28_e5145;
            locals.var_guard72_rv = 0.0;
            let (assign6930_body29_e5149,) = {
    if (locals.var_guard72 != 0.0) {
        (locals.var_lp_s0_max,)
    } else {
        (locals.var_lp_s0,)
    }
};
            locals.var_lp_s0 = assign6930_body29_e5149;
            locals.var_lp_s0_rv = 0.0;
            locals.var_phi_s0_bulk_0 = locals.var_t6;
            locals.var_phi_s0_bulk_0_dn0 = locals.var_t6_dn0;
            locals.var_phi_s0_bulk_0_dn2 = locals.var_t6_dn2;
            locals.var_phi_s0_bulk_0_dn4 = locals.var_t6_dn4;
            locals.var_phi_s0_bulk_0_dn5 = locals.var_t6_dn5;
            locals.var_phi_s0_bulk_0_dn6 = locals.var_t6_dn6;
            locals.var_phi_s0_bulk_0_dn8 = locals.var_t6_dn8;
            locals.var_phi_s0_bulk_0_dn10 = locals.var_t6_dn10;
            locals.var_phi_s0_bulk_0_dn11 = locals.var_t6_dn11;
            locals.var_phi_s0_bulk_0_dn12 = locals.var_t6_dn12;
            locals.var_phi_s0_bulk_0_rv = 0.0;
            locals.var_q_s0_bulk_0 = locals.var_t4;
            locals.var_q_s0_bulk_0_dn0 = locals.var_t4_dn0;
            locals.var_q_s0_bulk_0_dn2 = locals.var_t4_dn2;
            locals.var_q_s0_bulk_0_dn4 = locals.var_t4_dn4;
            locals.var_q_s0_bulk_0_dn5 = locals.var_t4_dn5;
            locals.var_q_s0_bulk_0_dn6 = locals.var_t4_dn6;
            locals.var_q_s0_bulk_0_dn8 = locals.var_t4_dn8;
            locals.var_q_s0_bulk_0_dn10 = locals.var_t4_dn10;
            locals.var_q_s0_bulk_0_dn11 = locals.var_t4_dn11;
            locals.var_q_s0_bulk_0_dn12 = locals.var_t4_dn12;
            locals.var_q_s0_bulk_0_rv = 0.0;
            let assign6930_body32_e5154: f64 = (locals.var_lp_s0 + 1.0);
            locals.var_lp_s0 = assign6930_body32_e5154;
            locals.var_lp_s0_rv = 0.0;
        }

        locals.var_phi_b_dep = locals.var_phi_b_dep0;
        locals.var_phi_b_dep_dn0 = locals.var_phi_b_dep0_dn0;
        locals.var_phi_b_dep_dn2 = locals.var_phi_b_dep0_dn2;
        locals.var_phi_b_dep_dn4 = locals.var_phi_b_dep0_dn4;
        locals.var_phi_b_dep_dn5 = locals.var_phi_b_dep0_dn5;
        locals.var_phi_b_dep_dn6 = locals.var_phi_b_dep0_dn6;
        locals.var_phi_b_dep_dn8 = locals.var_phi_b_dep0_dn8;
        locals.var_phi_b_dep_dn10 = locals.var_phi_b_dep0_dn10;
        locals.var_phi_b_dep_dn11 = locals.var_phi_b_dep0_dn11;
        locals.var_phi_b_dep_dn12 = locals.var_phi_b_dep0_dn12;
        locals.var_phi_b_dep_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_20(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign6950_e5158: f64 = (2.0 * 1.034943e-10);
        let assign6950_e5160: f64 = (assign6950_e5158 / 1.6021918e-19);
        let assign6950_e5162: f64 = (assign6950_e5160 * locals.var_phi_b_dep);
        let assign6950_e5164: f64 = (assign6950_e5162 / locals.var_uc_nsubs);
        let assign6950_e5165: f64 = (assign6950_e5164).sqrt();
        locals.var_t1 = assign6950_e5165;
        locals.var_t1_dn0 = (((((assign6950_e5160 * locals.var_phi_b_dep_dn0) * locals.var_uc_nsubs) - (assign6950_e5162 * locals.var_uc_nsubs_dn0)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign6950_e5165));
        locals.var_t1_dn2 = (((((assign6950_e5160 * locals.var_phi_b_dep_dn2) * locals.var_uc_nsubs) - (assign6950_e5162 * locals.var_uc_nsubs_dn2)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign6950_e5165));
        locals.var_t1_dn4 = (((((assign6950_e5160 * locals.var_phi_b_dep_dn4) * locals.var_uc_nsubs) - (assign6950_e5162 * locals.var_uc_nsubs_dn4)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign6950_e5165));
        locals.var_t1_dn5 = (((((assign6950_e5160 * locals.var_phi_b_dep_dn5) * locals.var_uc_nsubs) - (assign6950_e5162 * locals.var_uc_nsubs_dn5)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign6950_e5165));
        locals.var_t1_dn6 = (((((assign6950_e5160 * locals.var_phi_b_dep_dn6) * locals.var_uc_nsubs) - (assign6950_e5162 * locals.var_uc_nsubs_dn6)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign6950_e5165));
        locals.var_t1_dn8 = (((((assign6950_e5160 * locals.var_phi_b_dep_dn8) * locals.var_uc_nsubs) - (assign6950_e5162 * locals.var_uc_nsubs_dn8)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign6950_e5165));
        locals.var_t1_dn10 = (((((assign6950_e5160 * locals.var_phi_b_dep_dn10) * locals.var_uc_nsubs) - (assign6950_e5162 * locals.var_uc_nsubs_dn10)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign6950_e5165));
        locals.var_t1_dn11 = (((((assign6950_e5160 * locals.var_phi_b_dep_dn11) * locals.var_uc_nsubs) - (assign6950_e5162 * locals.var_uc_nsubs_dn11)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign6950_e5165));
        locals.var_t1_dn12 = (((((assign6950_e5160 * locals.var_phi_b_dep_dn12) * locals.var_uc_nsubs) - (assign6950_e5162 * locals.var_uc_nsubs_dn12)) / (locals.var_uc_nsubs * locals.var_uc_nsubs)) / (2.0 * assign6950_e5165));
        locals.var_t1_rv = 0.0;

        let assign6960_e5169: f64 = (0.99 * p.p227);
        let assign6960_e5170: f64 = if locals.var_t1 > assign6960_e5169 { 1.0 } else { 0.0 };
        locals.var_guard73 = assign6960_e5170;
        locals.var_guard73_rv = 0.0;

        let (assign6970_e5176, assign6970_e5176_d_n0, assign6970_e5176_d_n2, assign6970_e5176_d_n4, assign6970_e5176_d_n5, assign6970_e5176_d_n6, assign6970_e5176_d_n8, assign6970_e5176_d_n10, assign6970_e5176_d_n11, assign6970_e5176_d_n12,) = {
    if (locals.var_guard73 != 0.0) {
        let assign6970_e5174: f64 = (1.0 / locals.var_c_fox);
        (assign6970_e5174, (-(locals.var_c_fox_dn0 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn2 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn4 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn5 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn6 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn8 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn10 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn11 / (locals.var_c_fox * locals.var_c_fox))), (-(locals.var_c_fox_dn12 / (locals.var_c_fox * locals.var_c_fox))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn8, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign6970_e5176;
        locals.var_t0_dn0 = assign6970_e5176_d_n0;
        locals.var_t0_dn2 = assign6970_e5176_d_n2;
        locals.var_t0_dn4 = assign6970_e5176_d_n4;
        locals.var_t0_dn5 = assign6970_e5176_d_n5;
        locals.var_t0_dn6 = assign6970_e5176_d_n6;
        locals.var_t0_dn8 = assign6970_e5176_d_n8;
        locals.var_t0_dn10 = assign6970_e5176_d_n10;
        locals.var_t0_dn11 = assign6970_e5176_d_n11;
        locals.var_t0_dn12 = assign6970_e5176_d_n12;
        locals.var_t0_rv = 0.0;

        let (assign6980_e5182, assign6980_e5182_d_n0, assign6980_e5182_d_n2, assign6980_e5182_d_n4, assign6980_e5182_d_n5, assign6980_e5182_d_n6, assign6980_e5182_d_n8, assign6980_e5182_d_n10, assign6980_e5182_d_n11, assign6980_e5182_d_n12,) = {
    if (locals.var_guard73 != 0.0) {
        let assign6980_e5180: f64 = (1.0 / locals.var_c_box);
        (assign6980_e5180, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn8, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign6980_e5182;
        locals.var_t2_dn0 = assign6980_e5182_d_n0;
        locals.var_t2_dn2 = assign6980_e5182_d_n2;
        locals.var_t2_dn4 = assign6980_e5182_d_n4;
        locals.var_t2_dn5 = assign6980_e5182_d_n5;
        locals.var_t2_dn6 = assign6980_e5182_d_n6;
        locals.var_t2_dn8 = assign6980_e5182_d_n8;
        locals.var_t2_dn10 = assign6980_e5182_d_n10;
        locals.var_t2_dn11 = assign6980_e5182_d_n11;
        locals.var_t2_dn12 = assign6980_e5182_d_n12;
        locals.var_t2_rv = 0.0;

        let (assign6990_e5192, assign6990_e5192_d_n0, assign6990_e5192_d_n2, assign6990_e5192_d_n4, assign6990_e5192_d_n5, assign6990_e5192_d_n6, assign6990_e5192_d_n8, assign6990_e5192_d_n10, assign6990_e5192_d_n11, assign6990_e5192_d_n12,) = {
    if (locals.var_guard73 != 0.0) {
        let assign6990_e5187: f64 = (locals.var_t0 + locals.var_c_soi_inv);
        let assign6990_e5189: f64 = (assign6990_e5187 + locals.var_t2);
        let assign6990_e5190: f64 = (1.0 / assign6990_e5189);
        (assign6990_e5190, (-((locals.var_t0_dn0 + locals.var_t2_dn0) / (assign6990_e5189 * assign6990_e5189))), (-((locals.var_t0_dn2 + locals.var_t2_dn2) / (assign6990_e5189 * assign6990_e5189))), (-((locals.var_t0_dn4 + locals.var_t2_dn4) / (assign6990_e5189 * assign6990_e5189))), (-((locals.var_t0_dn5 + locals.var_t2_dn5) / (assign6990_e5189 * assign6990_e5189))), (-((locals.var_t0_dn6 + locals.var_t2_dn6) / (assign6990_e5189 * assign6990_e5189))), (-((locals.var_t0_dn8 + locals.var_t2_dn8) / (assign6990_e5189 * assign6990_e5189))), (-((locals.var_t0_dn10 + locals.var_t2_dn10) / (assign6990_e5189 * assign6990_e5189))), (-((locals.var_t0_dn11 + locals.var_t2_dn11) / (assign6990_e5189 * assign6990_e5189))), (-((locals.var_t0_dn12 + locals.var_t2_dn12) / (assign6990_e5189 * assign6990_e5189))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn8, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign6990_e5192;
        locals.var_t3_dn0 = assign6990_e5192_d_n0;
        locals.var_t3_dn2 = assign6990_e5192_d_n2;
        locals.var_t3_dn4 = assign6990_e5192_d_n4;
        locals.var_t3_dn5 = assign6990_e5192_d_n5;
        locals.var_t3_dn6 = assign6990_e5192_d_n6;
        locals.var_t3_dn8 = assign6990_e5192_d_n8;
        locals.var_t3_dn10 = assign6990_e5192_d_n10;
        locals.var_t3_dn11 = assign6990_e5192_d_n11;
        locals.var_t3_dn12 = assign6990_e5192_d_n12;
        locals.var_t3_rv = 0.0;

        let (assign7000_e5200, assign7000_e5200_d_n0, assign7000_e5200_d_n2, assign7000_e5200_d_n4, assign7000_e5200_d_n5, assign7000_e5200_d_n6, assign7000_e5200_d_n8, assign7000_e5200_d_n10, assign7000_e5200_d_n11, assign7000_e5200_d_n12,) = {
    if (locals.var_guard73 != 0.0) {
        let assign7000_e5197: f64 = (locals.var_t3 * locals.var_t0);
        let assign7000_e5198: f64 = (1.0 - assign7000_e5197);
        (assign7000_e5198, (-((locals.var_t3_dn0 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn0))), (-((locals.var_t3_dn2 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn2))), (-((locals.var_t3_dn4 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn4))), (-((locals.var_t3_dn5 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn5))), (-((locals.var_t3_dn6 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn6))), (-((locals.var_t3_dn8 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn8))), (-((locals.var_t3_dn10 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn10))), (-((locals.var_t3_dn11 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn11))), (-((locals.var_t3_dn12 * locals.var_t0) + (locals.var_t3 * locals.var_t0_dn12))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn8, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign7000_e5200;
        locals.var_t4_dn0 = assign7000_e5200_d_n0;
        locals.var_t4_dn2 = assign7000_e5200_d_n2;
        locals.var_t4_dn4 = assign7000_e5200_d_n4;
        locals.var_t4_dn5 = assign7000_e5200_d_n5;
        locals.var_t4_dn6 = assign7000_e5200_d_n6;
        locals.var_t4_dn8 = assign7000_e5200_d_n8;
        locals.var_t4_dn10 = assign7000_e5200_d_n10;
        locals.var_t4_dn11 = assign7000_e5200_d_n11;
        locals.var_t4_dn12 = assign7000_e5200_d_n12;
        locals.var_t4_rv = 0.0;

        let (assign7010_e5218, assign7010_e5218_d_n0, assign7010_e5218_d_n2, assign7010_e5218_d_n4, assign7010_e5218_d_n5, assign7010_e5218_d_n6, assign7010_e5218_d_n8, assign7010_e5218_d_n10, assign7010_e5218_d_n11, assign7010_e5218_d_n12,) = {
    if (locals.var_guard73 != 0.0) {
        let assign7010_e5205: f64 = (-locals.var_vbsbiz);
        let assign7010_e5209: f64 = (0.5 * locals.var_c_soi_inv);
        let assign7010_e5210: f64 = (locals.var_t2 + assign7010_e5209);
        let assign7010_e5212: f64 = (-locals.var_q_fd_soi);
        let assign7010_e5213: f64 = (assign7010_e5210 * assign7010_e5212);
        let assign7010_e5214: f64 = (assign7010_e5205 + assign7010_e5213);
        let assign7010_e5215: f64 = (locals.var_t3 * assign7010_e5214);
        let assign7010_e5216: f64 = (locals.var_t0 * assign7010_e5215);
        (assign7010_e5216, ((locals.var_t0_dn0 * assign7010_e5215) + (locals.var_t0 * ((locals.var_t3_dn0 * assign7010_e5214) + (locals.var_t3 * ((-locals.var_vbsbiz_dn0) + ((locals.var_t2_dn0 * assign7010_e5212) + (assign7010_e5210 * (-locals.var_q_fd_soi_dn0)))))))), ((locals.var_t0_dn2 * assign7010_e5215) + (locals.var_t0 * ((locals.var_t3_dn2 * assign7010_e5214) + (locals.var_t3 * ((-locals.var_vbsbiz_dn2) + ((locals.var_t2_dn2 * assign7010_e5212) + (assign7010_e5210 * (-locals.var_q_fd_soi_dn2)))))))), ((locals.var_t0_dn4 * assign7010_e5215) + (locals.var_t0 * ((locals.var_t3_dn4 * assign7010_e5214) + (locals.var_t3 * ((-locals.var_vbsbiz_dn4) + ((locals.var_t2_dn4 * assign7010_e5212) + (assign7010_e5210 * (-locals.var_q_fd_soi_dn4)))))))), ((locals.var_t0_dn5 * assign7010_e5215) + (locals.var_t0 * ((locals.var_t3_dn5 * assign7010_e5214) + (locals.var_t3 * ((-locals.var_vbsbiz_dn5) + ((locals.var_t2_dn5 * assign7010_e5212) + (assign7010_e5210 * (-locals.var_q_fd_soi_dn5)))))))), ((locals.var_t0_dn6 * assign7010_e5215) + (locals.var_t0 * ((locals.var_t3_dn6 * assign7010_e5214) + (locals.var_t3 * ((-locals.var_vbsbiz_dn6) + ((locals.var_t2_dn6 * assign7010_e5212) + (assign7010_e5210 * (-locals.var_q_fd_soi_dn6)))))))), ((locals.var_t0_dn8 * assign7010_e5215) + (locals.var_t0 * ((locals.var_t3_dn8 * assign7010_e5214) + (locals.var_t3 * ((-locals.var_vbsbiz_dn8) + ((locals.var_t2_dn8 * assign7010_e5212) + (assign7010_e5210 * (-locals.var_q_fd_soi_dn8)))))))), ((locals.var_t0_dn10 * assign7010_e5215) + (locals.var_t0 * ((locals.var_t3_dn10 * assign7010_e5214) + (locals.var_t3 * ((-locals.var_vbsbiz_dn10) + ((locals.var_t2_dn10 * assign7010_e5212) + (assign7010_e5210 * (-locals.var_q_fd_soi_dn10)))))))), ((locals.var_t0_dn11 * assign7010_e5215) + (locals.var_t0 * ((locals.var_t3_dn11 * assign7010_e5214) + (locals.var_t3 * ((-locals.var_vbsbiz_dn11) + ((locals.var_t2_dn11 * assign7010_e5212) + (assign7010_e5210 * (-locals.var_q_fd_soi_dn11)))))))), ((locals.var_t0_dn12 * assign7010_e5215) + (locals.var_t0 * ((locals.var_t3_dn12 * assign7010_e5214) + (locals.var_t3 * ((-locals.var_vbsbiz_dn12) + ((locals.var_t2_dn12 * assign7010_e5212) + (assign7010_e5210 * (-locals.var_q_fd_soi_dn12)))))))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn8, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign7010_e5218;
        locals.var_t5_dn0 = assign7010_e5218_d_n0;
        locals.var_t5_dn2 = assign7010_e5218_d_n2;
        locals.var_t5_dn4 = assign7010_e5218_d_n4;
        locals.var_t5_dn5 = assign7010_e5218_d_n5;
        locals.var_t5_dn6 = assign7010_e5218_d_n6;
        locals.var_t5_dn8 = assign7010_e5218_d_n8;
        locals.var_t5_dn10 = assign7010_e5218_d_n10;
        locals.var_t5_dn11 = assign7010_e5218_d_n11;
        locals.var_t5_dn12 = assign7010_e5218_d_n12;
        locals.var_t5_rv = 0.0;

        let (assign7020_e5224, assign7020_e5224_d_n0, assign7020_e5224_d_n2, assign7020_e5224_d_n4, assign7020_e5224_d_n5, assign7020_e5224_d_n6, assign7020_e5224_d_n8, assign7020_e5224_d_n10, assign7020_e5224_d_n11, assign7020_e5224_d_n12,) = {
    if (locals.var_guard73 != 0.0) {
        let assign7020_e5222: f64 = (locals.var_t5 / locals.var_t4);
        (assign7020_e5222, (((locals.var_t5_dn0 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn2 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn4 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn5 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn6 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn8 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn10 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn11 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t5_dn12 * locals.var_t4) - (locals.var_t5 * locals.var_t4_dn12)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_shift, locals.var_shift_dn0, locals.var_shift_dn2, locals.var_shift_dn4, locals.var_shift_dn5, locals.var_shift_dn6, locals.var_shift_dn8, locals.var_shift_dn10, locals.var_shift_dn11, locals.var_shift_dn12,)
    }
};
        locals.var_shift = assign7020_e5224;
        locals.var_shift_dn0 = assign7020_e5224_d_n0;
        locals.var_shift_dn2 = assign7020_e5224_d_n2;
        locals.var_shift_dn4 = assign7020_e5224_d_n4;
        locals.var_shift_dn5 = assign7020_e5224_d_n5;
        locals.var_shift_dn6 = assign7020_e5224_d_n6;
        locals.var_shift_dn8 = assign7020_e5224_d_n8;
        locals.var_shift_dn10 = assign7020_e5224_d_n10;
        locals.var_shift_dn11 = assign7020_e5224_d_n11;
        locals.var_shift_dn12 = assign7020_e5224_d_n12;
        locals.var_shift_rv = 0.0;

        let (assign7030_e5230, assign7030_e5230_d_n0, assign7030_e5230_d_n2, assign7030_e5230_d_n4, assign7030_e5230_d_n5, assign7030_e5230_d_n6, assign7030_e5230_d_n8, assign7030_e5230_d_n10, assign7030_e5230_d_n11, assign7030_e5230_d_n12,) = {
    if (locals.var_guard73 != 0.0) {
        let assign7030_e5228: f64 = (locals.var_vgs_fb + locals.var_shift);
        (assign7030_e5228, (locals.var_vgs_fb_dn0 + locals.var_shift_dn0), (locals.var_vgs_fb_dn2 + locals.var_shift_dn2), (locals.var_vgs_fb_dn4 + locals.var_shift_dn4), (locals.var_vgs_fb_dn5 + locals.var_shift_dn5), (locals.var_vgs_fb_dn6 + locals.var_shift_dn6), (locals.var_vgs_fb_dn8 + locals.var_shift_dn8), (locals.var_vgs_fb_dn10 + locals.var_shift_dn10), (locals.var_vgs_fb_dn11 + locals.var_shift_dn11), (locals.var_vgs_fb_dn12 + locals.var_shift_dn12),)
    } else {
        (locals.var_vgs_fb, locals.var_vgs_fb_dn0, locals.var_vgs_fb_dn2, locals.var_vgs_fb_dn4, locals.var_vgs_fb_dn5, locals.var_vgs_fb_dn6, locals.var_vgs_fb_dn8, locals.var_vgs_fb_dn10, locals.var_vgs_fb_dn11, locals.var_vgs_fb_dn12,)
    }
};
        locals.var_vgs_fb = assign7030_e5230;
        locals.var_vgs_fb_dn0 = assign7030_e5230_d_n0;
        locals.var_vgs_fb_dn2 = assign7030_e5230_d_n2;
        locals.var_vgs_fb_dn4 = assign7030_e5230_d_n4;
        locals.var_vgs_fb_dn5 = assign7030_e5230_d_n5;
        locals.var_vgs_fb_dn6 = assign7030_e5230_d_n6;
        locals.var_vgs_fb_dn8 = assign7030_e5230_d_n8;
        locals.var_vgs_fb_dn10 = assign7030_e5230_d_n10;
        locals.var_vgs_fb_dn11 = assign7030_e5230_d_n11;
        locals.var_vgs_fb_dn12 = assign7030_e5230_d_n12;
        locals.var_vgs_fb_rv = 0.0;

        let (assign7040_e5238, assign7040_e5238_d_n0, assign7040_e5238_d_n2, assign7040_e5238_d_n4, assign7040_e5238_d_n5, assign7040_e5238_d_n6, assign7040_e5238_d_n8, assign7040_e5238_d_n10, assign7040_e5238_d_n11, assign7040_e5238_d_n12,) = {
    if (locals.var_guard73 != 0.0) {
        let assign7040_e5235: f64 = (p.p298 * locals.var_shift);
        let assign7040_e5236: f64 = (locals.var_vgp - assign7040_e5235);
        (assign7040_e5236, (locals.var_vgp_dn0 - (p.p298 * locals.var_shift_dn0)), (locals.var_vgp_dn2 - (p.p298 * locals.var_shift_dn2)), (locals.var_vgp_dn4 - (p.p298 * locals.var_shift_dn4)), (locals.var_vgp_dn5 - (p.p298 * locals.var_shift_dn5)), (locals.var_vgp_dn6 - (p.p298 * locals.var_shift_dn6)), (locals.var_vgp_dn8 - (p.p298 * locals.var_shift_dn8)), (locals.var_vgp_dn10 - (p.p298 * locals.var_shift_dn10)), (locals.var_vgp_dn11 - (p.p298 * locals.var_shift_dn11)), (locals.var_vgp_dn12 - (p.p298 * locals.var_shift_dn12)),)
    } else {
        (locals.var_vgp, locals.var_vgp_dn0, locals.var_vgp_dn2, locals.var_vgp_dn4, locals.var_vgp_dn5, locals.var_vgp_dn6, locals.var_vgp_dn8, locals.var_vgp_dn10, locals.var_vgp_dn11, locals.var_vgp_dn12,)
    }
};
        locals.var_vgp = assign7040_e5238;
        locals.var_vgp_dn0 = assign7040_e5238_d_n0;
        locals.var_vgp_dn2 = assign7040_e5238_d_n2;
        locals.var_vgp_dn4 = assign7040_e5238_d_n4;
        locals.var_vgp_dn5 = assign7040_e5238_d_n5;
        locals.var_vgp_dn6 = assign7040_e5238_d_n6;
        locals.var_vgp_dn8 = assign7040_e5238_d_n8;
        locals.var_vgp_dn10 = assign7040_e5238_d_n10;
        locals.var_vgp_dn11 = assign7040_e5238_d_n11;
        locals.var_vgp_dn12 = assign7040_e5238_d_n12;
        locals.var_vgp_rv = 0.0;

        let (assign7050_e5242, assign7050_e5242_d_n0, assign7050_e5242_d_n2, assign7050_e5242_d_n4, assign7050_e5242_d_n5, assign7050_e5242_d_n6, assign7050_e5242_d_n8, assign7050_e5242_d_n10, assign7050_e5242_d_n11, assign7050_e5242_d_n12,) = {
    if (locals.var_guard73 != 0.0) {
        (locals.var_vgp, locals.var_vgp_dn0, locals.var_vgp_dn2, locals.var_vgp_dn4, locals.var_vgp_dn5, locals.var_vgp_dn6, locals.var_vgp_dn8, locals.var_vgp_dn10, locals.var_vgp_dn11, locals.var_vgp_dn12,)
    } else {
        (locals.var_vgpz, locals.var_vgpz_dn0, locals.var_vgpz_dn2, locals.var_vgpz_dn4, locals.var_vgpz_dn5, locals.var_vgpz_dn6, locals.var_vgpz_dn8, locals.var_vgpz_dn10, locals.var_vgpz_dn11, locals.var_vgpz_dn12,)
    }
};
        locals.var_vgpz = assign7050_e5242;
        locals.var_vgpz_dn0 = assign7050_e5242_d_n0;
        locals.var_vgpz_dn2 = assign7050_e5242_d_n2;
        locals.var_vgpz_dn4 = assign7050_e5242_d_n4;
        locals.var_vgpz_dn5 = assign7050_e5242_d_n5;
        locals.var_vgpz_dn6 = assign7050_e5242_d_n6;
        locals.var_vgpz_dn8 = assign7050_e5242_d_n8;
        locals.var_vgpz_dn10 = assign7050_e5242_d_n10;
        locals.var_vgpz_dn11 = assign7050_e5242_d_n11;
        locals.var_vgpz_dn12 = assign7050_e5242_d_n12;
        locals.var_vgpz_rv = 0.0;

        let assign7060_e5245: f64 = if locals.var_flg_pprv >= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard74 = assign7060_e5245;
        locals.var_guard74_rv = 0.0;

        let (assign7070_e5249, assign7070_e5249_d_n0, assign7070_e5249_d_n2, assign7070_e5249_d_n4, assign7070_e5249_d_n5, assign7070_e5249_d_n6, assign7070_e5249_d_n8, assign7070_e5249_d_n10, assign7070_e5249_d_n11, assign7070_e5249_d_n12,) = {
    if (locals.var_guard74 != 0.0) {
        (locals.var_pss0_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_s0_soi, locals.var_phi_s0_soi_dn0, locals.var_phi_s0_soi_dn2, locals.var_phi_s0_soi_dn4, locals.var_phi_s0_soi_dn5, locals.var_phi_s0_soi_dn6, locals.var_phi_s0_soi_dn8, locals.var_phi_s0_soi_dn10, locals.var_phi_s0_soi_dn11, locals.var_phi_s0_soi_dn12,)
    }
};
        locals.var_phi_s0_soi = assign7070_e5249;
        locals.var_phi_s0_soi_dn0 = assign7070_e5249_d_n0;
        locals.var_phi_s0_soi_dn2 = assign7070_e5249_d_n2;
        locals.var_phi_s0_soi_dn4 = assign7070_e5249_d_n4;
        locals.var_phi_s0_soi_dn5 = assign7070_e5249_d_n5;
        locals.var_phi_s0_soi_dn6 = assign7070_e5249_d_n6;
        locals.var_phi_s0_soi_dn8 = assign7070_e5249_d_n8;
        locals.var_phi_s0_soi_dn10 = assign7070_e5249_d_n10;
        locals.var_phi_s0_soi_dn11 = assign7070_e5249_d_n11;
        locals.var_phi_s0_soi_dn12 = assign7070_e5249_d_n12;
        locals.var_phi_s0_soi_rv = 0.0;

        let (assign7080_e5253, assign7080_e5253_d_n0, assign7080_e5253_d_n2, assign7080_e5253_d_n4, assign7080_e5253_d_n5, assign7080_e5253_d_n6, assign7080_e5253_d_n8, assign7080_e5253_d_n10, assign7080_e5253_d_n11, assign7080_e5253_d_n12,) = {
    if (locals.var_guard74 != 0.0) {
        (locals.var_pbs0_ini, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_b0_soi, locals.var_phi_b0_soi_dn0, locals.var_phi_b0_soi_dn2, locals.var_phi_b0_soi_dn4, locals.var_phi_b0_soi_dn5, locals.var_phi_b0_soi_dn6, locals.var_phi_b0_soi_dn8, locals.var_phi_b0_soi_dn10, locals.var_phi_b0_soi_dn11, locals.var_phi_b0_soi_dn12,)
    }
};
        locals.var_phi_b0_soi = assign7080_e5253;
        locals.var_phi_b0_soi_dn0 = assign7080_e5253_d_n0;
        locals.var_phi_b0_soi_dn2 = assign7080_e5253_d_n2;
        locals.var_phi_b0_soi_dn4 = assign7080_e5253_d_n4;
        locals.var_phi_b0_soi_dn5 = assign7080_e5253_d_n5;
        locals.var_phi_b0_soi_dn6 = assign7080_e5253_d_n6;
        locals.var_phi_b0_soi_dn8 = assign7080_e5253_d_n8;
        locals.var_phi_b0_soi_dn10 = assign7080_e5253_d_n10;
        locals.var_phi_b0_soi_dn11 = assign7080_e5253_d_n11;
        locals.var_phi_b0_soi_dn12 = assign7080_e5253_d_n12;
        locals.var_phi_b0_soi_rv = 0.0;

        let (assign7090_e5259, assign7090_e5259_d_n0, assign7090_e5259_d_n2, assign7090_e5259_d_n4, assign7090_e5259_d_n5, assign7090_e5259_d_n6, assign7090_e5259_d_n8, assign7090_e5259_d_n10, assign7090_e5259_d_n11, assign7090_e5259_d_n12,) = {
    if (locals.var_guard74 != 0.0) {
        let assign7090_e5257: f64 = (locals.var_psb0_ini + locals.var_vbsbiz);
        (assign7090_e5257, locals.var_vbsbiz_dn0, locals.var_vbsbiz_dn2, locals.var_vbsbiz_dn4, locals.var_vbsbiz_dn5, locals.var_vbsbiz_dn6, locals.var_vbsbiz_dn8, locals.var_vbsbiz_dn10, locals.var_vbsbiz_dn11, locals.var_vbsbiz_dn12,)
    } else {
        (locals.var_phi_s0_bulk, locals.var_phi_s0_bulk_dn0, locals.var_phi_s0_bulk_dn2, locals.var_phi_s0_bulk_dn4, locals.var_phi_s0_bulk_dn5, locals.var_phi_s0_bulk_dn6, locals.var_phi_s0_bulk_dn8, locals.var_phi_s0_bulk_dn10, locals.var_phi_s0_bulk_dn11, locals.var_phi_s0_bulk_dn12,)
    }
};
        locals.var_phi_s0_bulk = assign7090_e5259;
        locals.var_phi_s0_bulk_dn0 = assign7090_e5259_d_n0;
        locals.var_phi_s0_bulk_dn2 = assign7090_e5259_d_n2;
        locals.var_phi_s0_bulk_dn4 = assign7090_e5259_d_n4;
        locals.var_phi_s0_bulk_dn5 = assign7090_e5259_d_n5;
        locals.var_phi_s0_bulk_dn6 = assign7090_e5259_d_n6;
        locals.var_phi_s0_bulk_dn8 = assign7090_e5259_d_n8;
        locals.var_phi_s0_bulk_dn10 = assign7090_e5259_d_n10;
        locals.var_phi_s0_bulk_dn11 = assign7090_e5259_d_n11;
        locals.var_phi_s0_bulk_dn12 = assign7090_e5259_d_n12;
        locals.var_phi_s0_bulk_rv = 0.0;

        let (assign7100_e5270, assign7100_e5270_d_n0, assign7100_e5270_d_n2, assign7100_e5270_d_n4, assign7100_e5270_d_n5, assign7100_e5270_d_n6, assign7100_e5270_d_n8, assign7100_e5270_d_n10, assign7100_e5270_d_n11, assign7100_e5270_d_n12,) = {
    if (locals.var_guard74 != 0.0) {
        let assign7100_e5262: f64 = (-locals.var_q_fd_soi);
        let assign7100_e5264: f64 = (assign7100_e5262 * locals.var_c_soi_inv);
        let assign7100_e5266: f64 = (assign7100_e5264 / 2.0);
        let assign7100_e5268: f64 = (assign7100_e5266 + locals.var_beta_inv);
        (assign7100_e5268, (((-locals.var_q_fd_soi_dn0) * locals.var_c_soi_inv) / 2.0), (((-locals.var_q_fd_soi_dn2) * locals.var_c_soi_inv) / 2.0), ((((-locals.var_q_fd_soi_dn4) * locals.var_c_soi_inv) / 2.0) + locals.var_beta_inv_dn4), (((-locals.var_q_fd_soi_dn5) * locals.var_c_soi_inv) / 2.0), (((-locals.var_q_fd_soi_dn6) * locals.var_c_soi_inv) / 2.0), (((-locals.var_q_fd_soi_dn8) * locals.var_c_soi_inv) / 2.0), (((-locals.var_q_fd_soi_dn10) * locals.var_c_soi_inv) / 2.0), (((-locals.var_q_fd_soi_dn11) * locals.var_c_soi_inv) / 2.0), (((-locals.var_q_fd_soi_dn12) * locals.var_c_soi_inv) / 2.0),)
    } else {
        (locals.var_fd_start, locals.var_fd_start_dn0, locals.var_fd_start_dn2, locals.var_fd_start_dn4, locals.var_fd_start_dn5, locals.var_fd_start_dn6, locals.var_fd_start_dn8, locals.var_fd_start_dn10, locals.var_fd_start_dn11, locals.var_fd_start_dn12,)
    }
};
        locals.var_fd_start = assign7100_e5270;
        locals.var_fd_start_dn0 = assign7100_e5270_d_n0;
        locals.var_fd_start_dn2 = assign7100_e5270_d_n2;
        locals.var_fd_start_dn4 = assign7100_e5270_d_n4;
        locals.var_fd_start_dn5 = assign7100_e5270_d_n5;
        locals.var_fd_start_dn6 = assign7100_e5270_d_n6;
        locals.var_fd_start_dn8 = assign7100_e5270_d_n8;
        locals.var_fd_start_dn10 = assign7100_e5270_d_n10;
        locals.var_fd_start_dn11 = assign7100_e5270_d_n11;
        locals.var_fd_start_dn12 = assign7100_e5270_d_n12;
        locals.var_fd_start_rv = 0.0;

        let (assign7110_e5278, assign7110_e5278_d_n0, assign7110_e5278_d_n2, assign7110_e5278_d_n4, assign7110_e5278_d_n5, assign7110_e5278_d_n6, assign7110_e5278_d_n8, assign7110_e5278_d_n10, assign7110_e5278_d_n11, assign7110_e5278_d_n12,) = {
    if (locals.var_guard74 != 0.0) {
        let assign7110_e5275: f64 = (locals.var_q_s0_bulk_0 * locals.var_c_soi_inv);
        let assign7110_e5276: f64 = (locals.var_fd_start - assign7110_e5275);
        (assign7110_e5276, (locals.var_fd_start_dn0 - (locals.var_q_s0_bulk_0_dn0 * locals.var_c_soi_inv)), (locals.var_fd_start_dn2 - (locals.var_q_s0_bulk_0_dn2 * locals.var_c_soi_inv)), (locals.var_fd_start_dn4 - (locals.var_q_s0_bulk_0_dn4 * locals.var_c_soi_inv)), (locals.var_fd_start_dn5 - (locals.var_q_s0_bulk_0_dn5 * locals.var_c_soi_inv)), (locals.var_fd_start_dn6 - (locals.var_q_s0_bulk_0_dn6 * locals.var_c_soi_inv)), (locals.var_fd_start_dn8 - (locals.var_q_s0_bulk_0_dn8 * locals.var_c_soi_inv)), (locals.var_fd_start_dn10 - (locals.var_q_s0_bulk_0_dn10 * locals.var_c_soi_inv)), (locals.var_fd_start_dn11 - (locals.var_q_s0_bulk_0_dn11 * locals.var_c_soi_inv)), (locals.var_fd_start_dn12 - (locals.var_q_s0_bulk_0_dn12 * locals.var_c_soi_inv)),)
    } else {
        (locals.var_fd_end, locals.var_fd_end_dn0, locals.var_fd_end_dn2, locals.var_fd_end_dn4, locals.var_fd_end_dn5, locals.var_fd_end_dn6, locals.var_fd_end_dn8, locals.var_fd_end_dn10, locals.var_fd_end_dn11, locals.var_fd_end_dn12,)
    }
};
        locals.var_fd_end = assign7110_e5278;
        locals.var_fd_end_dn0 = assign7110_e5278_d_n0;
        locals.var_fd_end_dn2 = assign7110_e5278_d_n2;
        locals.var_fd_end_dn4 = assign7110_e5278_d_n4;
        locals.var_fd_end_dn5 = assign7110_e5278_d_n5;
        locals.var_fd_end_dn6 = assign7110_e5278_d_n6;
        locals.var_fd_end_dn8 = assign7110_e5278_d_n8;
        locals.var_fd_end_dn10 = assign7110_e5278_d_n10;
        locals.var_fd_end_dn11 = assign7110_e5278_d_n11;
        locals.var_fd_end_dn12 = assign7110_e5278_d_n12;
        locals.var_fd_end_rv = 0.0;

        let assign7120_e5281: f64 = if locals.var_vbsbiz < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard75 = assign7120_e5281;
        locals.var_guard75_rv = 0.0;

        let (assign7130_e5288, assign7130_e5288_d_n0, assign7130_e5288_d_n2, assign7130_e5288_d_n4, assign7130_e5288_d_n5, assign7130_e5288_d_n6, assign7130_e5288_d_n8, assign7130_e5288_d_n10, assign7130_e5288_d_n11, assign7130_e5288_d_n12,) = {
    if ((locals.var_guard74 == 0.0) && (locals.var_guard75 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgs_shift, locals.var_vgs_shift_dn0, locals.var_vgs_shift_dn2, locals.var_vgs_shift_dn4, locals.var_vgs_shift_dn5, locals.var_vgs_shift_dn6, locals.var_vgs_shift_dn8, locals.var_vgs_shift_dn10, locals.var_vgs_shift_dn11, locals.var_vgs_shift_dn12,)
    }
};
        locals.var_vgs_shift = assign7130_e5288;
        locals.var_vgs_shift_dn0 = assign7130_e5288_d_n0;
        locals.var_vgs_shift_dn2 = assign7130_e5288_d_n2;
        locals.var_vgs_shift_dn4 = assign7130_e5288_d_n4;
        locals.var_vgs_shift_dn5 = assign7130_e5288_d_n5;
        locals.var_vgs_shift_dn6 = assign7130_e5288_d_n6;
        locals.var_vgs_shift_dn8 = assign7130_e5288_d_n8;
        locals.var_vgs_shift_dn10 = assign7130_e5288_d_n10;
        locals.var_vgs_shift_dn11 = assign7130_e5288_d_n11;
        locals.var_vgs_shift_dn12 = assign7130_e5288_d_n12;
        locals.var_vgs_shift_rv = 0.0;

        let (assign7140_e5295,) = {
    if ((locals.var_guard74 == 0.0) && (locals.var_guard75 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_lp_s0,)
    }
};
        locals.var_lp_s0 = assign7140_e5295;
        locals.var_lp_s0_rv = 0.0;

    }
}
